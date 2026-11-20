use std::{
    ffi::{c_int, c_void},
    path::{Path, PathBuf},
    ptr::NonNull,
};

use openfx::sys::{
    core::{OfxHost, OfxPlugin, OfxStatus},
    image_effect::kOfxImageEffectPluginApi,
};
use snafu::{ResultExt as _, Snafu};

use crate::{
    definitions::CURRENT_ARCHITECTURE,
    plugins::{Plugin, PluginConstructionError},
};

/// A type that manages a loaded OpenFX plugin bundle. It owns the loaded
/// bundle's dynamic library, and thereby owns everything that library owns.
#[derive(Debug)]
pub struct PluginBundleManager {
    #[allow(dead_code, reason = "It owns the plugins.")]
    bundle: PluginBundle,

    plugins: Vec<Plugin>,
}

#[derive(Debug, Snafu)]
pub enum PluginBundleManagerInitializationError {
    NoPluginAtIndex { index: c_int },
    PluginConstruction { source: PluginConstructionError },
}

pub struct PluginBundleManagerInitializationResult {
    pub manager: Option<PluginBundleManager>,
    pub errors: Vec<PluginBundleManagerInitializationError>,
}

#[derive(Debug, Snafu)]
pub enum PluginBundleConstructionError {
    InvalidBundleName {
        bundle_path: PathBuf,
    },
    NoSuitableArchitecture {
        bundle_path: PathBuf,
        architecture: &'static str,
    },
    NoSuitableBinary {
        bundle_path: PathBuf,
    },
    Libloading {
        source: libloading::Error,
    },
}

impl PluginBundleManager {
    pub fn get_number_of_plugins(&self) -> usize {
        self.plugins.len()
    }

    pub fn plugins(&self) -> &[Plugin] {
        &self.plugins
    }
}

/// A type representing an instance of an OpenFX plugin bundle.
#[derive(Debug)]
pub struct PluginBundle {
    library: PluginLibrary,
}

impl PluginBundle {
    /// ## Safety
    ///
    /// The safety requirements of this function are inherited from those of
    /// invoking [`Self::load_only_with_binary`].
    pub unsafe fn load(bundle_path: &Path) -> Result<Self, PluginBundleConstructionError> {
        let file_name =
            bundle_path
                .file_name()
                .ok_or(PluginBundleConstructionError::InvalidBundleName {
                    bundle_path: bundle_path.to_path_buf(),
                })?;
        let Some(file_name) = file_name.to_str() else {
            return Err(PluginBundleConstructionError::InvalidBundleName {
                bundle_path: bundle_path.to_path_buf(),
            });
        };

        let Some(file_stem_name) = file_name.strip_suffix(".ofx.bundle") else {
            return Err(PluginBundleConstructionError::InvalidBundleName {
                bundle_path: bundle_path.to_path_buf(),
            });
        };

        let architecture_path = bundle_path.join("Contents").join(CURRENT_ARCHITECTURE);
        if !architecture_path.exists() {
            return Err(PluginBundleConstructionError::NoSuitableArchitecture {
                bundle_path: bundle_path.to_path_buf(),
                architecture: CURRENT_ARCHITECTURE,
            });
        }

        let binary_path = architecture_path.join(format!("{}.ofx", file_stem_name));
        if !binary_path.exists() {
            return Err(PluginBundleConstructionError::NoSuitableBinary {
                bundle_path: bundle_path.to_path_buf(),
            });
        }

        unsafe { Self::load_only_with_binary(&binary_path).context(LibloadingSnafu) }
    }

    /// ## Safety
    ///
    /// The safety requirements of this function are inherited from those of
    /// invoking [`PluginLibrary::load`].
    pub unsafe fn load_only_with_binary(binary_path: &Path) -> Result<Self, libloading::Error> {
        let library = unsafe { PluginLibrary::load(binary_path)? };
        Ok(Self { library })
    }

    pub fn library_file_name(&self) -> Option<&Path> {
        self.library.file_name()
    }

    /// ## Safety
    ///
    /// The safety requirements of this function are inherited from those of
    /// invoking [`PluginLibrary::get_number_of_plugins`] and
    /// [`PluginLibrary::get_plugin`].
    pub unsafe fn initialize(self) -> PluginBundleManagerInitializationResult {
        // unsafe { self.bundle.library.set_host(host) };

        let number_of_plugins = unsafe { self.library.get_number_of_plugins() };

        let mut plugins: Vec<Plugin> = Vec::with_capacity(number_of_plugins as usize);
        let mut errors: Vec<PluginBundleManagerInitializationError> = vec![];
        for i in 0..number_of_plugins {
            let plugin_ptr = unsafe { self.library.get_plugin(i) };
            if let Some(plugin_ptr) = NonNull::new(plugin_ptr) {
                match Plugin::try_new(plugin_ptr) {
                    Ok(plugin) => plugins.push(plugin),
                    Err(PluginConstructionError::ApiMismatch { actual_api }) => {
                        self.log_info(&format!("Found an irrelevant plugin for `{actual_api:?}` in the plugin bundle. Skipping it."));
                    }
                    Err(PluginConstructionError::ApiVersionMismatch { actual_version }) => {
                        self.log_warn(&format!("Found a plugin for `{kOfxImageEffectPluginApi:?}` with version `{actual_version}`, which we don't support. Ignoring it."));
                    }
                    Err(err) => {
                        errors.push(PluginBundleManagerInitializationError::PluginConstruction {
                            source: err,
                        })
                    }
                }
            } else {
                errors.push(PluginBundleManagerInitializationError::NoPluginAtIndex { index: i });
            }
        }

        PluginBundleManagerInitializationResult {
            manager: if plugins.is_empty() {
                None
            } else {
                Some(PluginBundleManager {
                    bundle: self,
                    plugins,
                })
            },
            errors,
        }
    }
}

/// A type representing an instance of a dynamically loaded OpenFX plugin
/// library.
#[derive(Debug)]
struct PluginLibrary {
    #[allow(dead_code, reason = "It owns the functions below.")]
    library: libloading::Library,

    file_name: Option<PathBuf>,

    #[expect(non_snake_case)]
    OfxSetHost: Option<unsafe extern "C" fn(*mut c_void) -> c_int>,
    #[expect(non_snake_case)]
    OfxGetNumberOfPlugins: unsafe extern "C" fn() -> c_int,
    #[expect(non_snake_case)]
    OfxGetPlugin: unsafe extern "C" fn(c_int) -> *mut c_void,
}

impl PluginLibrary {
    /// ## Safety
    ///
    /// The safety requirements of this function are inherited from those of
    /// invoking [`libloading::Library::new`] and [`Self::new`].
    unsafe fn load(binary_path: &Path) -> Result<Self, libloading::Error> {
        let library = unsafe { libloading::Library::new(binary_path) }?;
        Self::new(library, binary_path.file_name().map(|s| s.as_ref()))
    }

    /// ## Safety
    ///
    /// The safety requirements of this function are inherited from those of
    /// invoking [`libloading::Library::get`]:
    ///
    /// > Users of this API must specify the correct type of the function or
    /// > variable loaded.
    fn new(
        library: libloading::Library,
        file_name: Option<&Path>,
    ) -> Result<Self, libloading::Error> {
        Ok(unsafe {
            Self {
                file_name: file_name.map(|p| p.to_path_buf()),
                OfxSetHost: library.get(b"OfxSetHost\0").ok().map(|s| *s),
                OfxGetNumberOfPlugins: *library.get(b"OfxGetNumberOfPlugins\0")?,
                OfxGetPlugin: *library.get(b"OfxGetPlugin\0")?,
                library,
            }
        })
    }

    fn file_name(&self) -> Option<&Path> {
        self.file_name.as_deref()
    }

    /// ## Safety
    ///
    /// The execution of the library's `OfxSetHost` function is unsafe.
    ///
    /// Ensure that the host remains alive at least until [Self::get_plugin]
    /// is called. See: https://openfx.readthedocs.io/en/main/Reference/api/file/ofxCore_8h.html#_CPPv410OfxSetHostPK7OfxHost
    #[expect(dead_code, reason = "TODO")]
    unsafe fn set_host(&self, host: &OfxHost) -> Option<openfx::low::InboundResult<()>> {
        self.OfxSetHost
            .map(|f| unsafe { f(host as *const _ as *mut c_void) } as OfxStatus)
            .map(openfx::low::InboundResult::from_sys)
    }

    /// ## Safety
    ///
    /// The execution of the library's `OfxGetNumberOfPlugins` function is
    /// unsafe.
    unsafe fn get_number_of_plugins(&self) -> c_int {
        unsafe { (self.OfxGetNumberOfPlugins)() }
    }

    /// ## Safety
    ///
    /// The execution of the library's `OfxGetPlugin` function is unsafe.
    unsafe fn get_plugin(&self, nth: c_int) -> *mut OfxPlugin {
        unsafe { (self.OfxGetPlugin)(nth) as *mut OfxPlugin }
    }
}

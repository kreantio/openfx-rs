//! ## Terminology
//!
//! In this crate, we use the word “surroundings” to refer to what is commonly
//! called “context” elsewhere. That's because
//! [`openfx::sys::image_effect::kOfxImageEffectPropContext`] is one of the core
//! concepts in OpenFX, and we want to avoid confusion.

pub use openfx::{
    export_plugins as __OPENFX__export_plugins,
    low::OutboundResult as __OPENFX_LOW__OUTBOUND_RESULT,
    low_plugin::{
        Host as __OPENFX_UOW_PLUGINS__HOST, Plugin as __OPENFX_LOW_PLUGIN__Plugin,
        Plugins as __OPENFX_LOW_PLUGINS__Plugins,
        actions::image_effect::ImageEffectAction as __OPENFX_LOW__IMAGE_EFFECT_ACTION,
    },
    plugin_struct as __OPENFX__plugin_struct,
    sys::core::OfxPlugin,
};

pub mod actions;
pub mod host;
pub mod plugin;
pub mod plugin_instance;

mod utils;

#[cfg(feature = "tracing")]
pub static __TRACING_DISPATCH: std::sync::OnceLock<tracing::Dispatch> = std::sync::OnceLock::new();

/// Holds a set of plugins that you want to expose to OpenFX hosts.
pub trait Plugins {
    /// ## Parameters
    ///
    /// - `host`: An optional reference to the host structure that you can query
    ///   for host-specific information to determine which plugins you want to
    ///   expose to the host. [`None`] means that the host does not provide this
    ///   ability. (i.e., It does not call the `OfxSetHost` function exported on
    ///   our side.)
    ///
    /// ## Returns
    ///
    /// A set of plugins that you want to expose to the host.
    fn plugins(
        s: PluginsSurroundings,
        host: Option<&host::Host>,
    ) -> Vec<openfx::sys::core::OfxPlugin>;
}

pub struct PluginsSurroundings;
impl PluginsSurroundings {
    #[cfg(feature = "tracing")]
    pub fn set_tracing_dispatch(
        &self,
        dispatch: tracing::Dispatch,
    ) -> Result<(), tracing::Dispatch> {
        __TRACING_DISPATCH.set(dispatch)
    }
}

/// Exports (`pub extern "C"`) a set of plugins that OpenFX hosts use to
/// identify and load your plugins.
///
/// Each build of your plugin should invoke this macro only once, since the
/// names of the exported functions are fixed.
///
/// The functions exported by this macro are `OfxSetHost`,
/// `OfxGetNumberOfPlugins` and `OfxGetPlugin`. See
/// <https://openfx.readthedocs.io/en/main/Reference/ofxCoreAPI.html#identifying-and-loading-plug-ins>
/// for more information.
///
/// ## Parameters
///
/// - `plugins_type`: A type that implements the [`Plugins`] trait.
#[macro_export]
macro_rules! export_plugins_ex {
    ($plugins_type:ty) => {
        #[expect(non_camel_case_types)]
        struct __OPK__Plugins;
        impl $crate::__OPENFX_LOW_PLUGINS__Plugins for __OPK__Plugins {
            fn plugins(host: Option<$crate::__OPENFX_UOW_PLUGINS__HOST>) -> Vec<$crate::OfxPlugin> {
                let host = host.map(|host| $crate::host::Host::from_low(host));
                let host = host.as_ref();
                <$plugins_type>::plugins($crate::PluginsSurroundings, host)
            }
        }
        $crate::__OPENFX__export_plugins!(__OPK__Plugins);
    };
}

#[macro_export]
macro_rules! plugin_struct_ex {
    ($plugin_type:ty) => {{
        #[expect(non_camel_case_types)]
        struct __OPK__Plugin;
        static MANAGER: ::std::sync::LazyLock<$crate::plugin::PluginManager<$plugin_type>> =
            ::std::sync::LazyLock::new(Default::default);
        impl $crate::__OPENFX_LOW_PLUGIN__Plugin for __OPK__Plugin {
            const PLUGIN_IDENTIFIER: &'static ::std::ffi::CStr =
                <$plugin_type as $crate::plugin::Plugin>::PLUGIN_IDENTIFIER;
            const PLUGIN_VERSION_MAJOR: ::std::os::raw::c_uint =
                <$plugin_type as $crate::plugin::Plugin>::PLUGIN_VERSION_MAJOR;
            const PLUGIN_VERSION_MINOR: ::std::os::raw::c_uint =
                <$plugin_type as $crate::plugin::Plugin>::PLUGIN_VERSION_MINOR;
            fn set_host(host: $crate::__OPENFX_UOW_PLUGINS__HOST) {
                let manager = &*MANAGER;
                manager.__set_host(host);
            }
            fn main_entry(
                action: $crate::__OPENFX_LOW__IMAGE_EFFECT_ACTION,
            ) -> $crate::__OPENFX_LOW__OUTBOUND_RESULT {
                let manager = &*MANAGER;
                manager.__main_entry(action)
            }
        }
        $crate::__OPENFX__plugin_struct!(__OPK__Plugin)
    }};
}

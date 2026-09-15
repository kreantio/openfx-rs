# Crate `openfx`

[<img alt="github" src="https://img.shields.io/badge/github-kreantio/openfx-rs?logo=github" height="20">](https://github.com/kreantio/openfx-rs)
[![Latest version](https://img.shields.io/crates/v/openfx.svg)](https://crates.io/crates/openfx)
[![Documentation](https://docs.rs/openfx/badge.svg)](https://docs.rs/openfx)
[![Crates.io License](https://img.shields.io/crates/l/openfx)](https://github.com/kreantio/openfx-rs/blob/main/LICENSE.md)

> [!CAUTION]
>
> This crate is in the early stages of development:
>
> - Its API is expected to change.
> - At present, only `openfx::{sys, sys_helpers}` is fully implemented.
>   `openfx::{low, low_plugin}` is still under development.
>
> Although `openfx::{low, low_plugin}` already provides some functionality, it
> is not currently recommended for use.

This crate provides bindings for the OpenFX API in 2 abstraction layers:

- layer `sys` (`openfx::sys`): raw low-level bindings generated from the OpenFX
  C headers.
- layer `low` (`openfx::{low, low_plugin}`): unsafe low-level bindings built on
  top of the `sys` bindings, where types of values from the `sys` bindings are
  converted to stronger types generated from the official C++ bindings
  (`$OFX_REPO/openfx-cpp/include`) with runtime overhead.

## for Plugin Development

### Building Plugins

OpenFX plugins require to be packed in a specific way. Therefore, you need to do
some additional work after the binary is built. See:
[OpenFX reference / Packaging OFX Plug-ins]. You can also have a look at
[some of the scripts I use for bundling the example plugins].

[OpenFX reference / Packaging OFX Plug-ins]: https://openfx.readthedocs.io/en/latest/Reference/ofxPackaging.html
[some of the scripts I use for bundling the example plugins]: examples/ofx-guide-example-plugins-layer-sys/scripts/bundle.ts

Please don't forget that the `Cargo.toml` of your plugin crates should contain:

```toml
[lib]
crate-type = ["cdylib"]
```

### Examples

#### writing plugins in layer `sys`

##### The barest way

<details><summary>example code</summary>

```rs
use std::ffi::c_int;

use openfx::sys::{
    generic::core::{OfxHost, OfxPlugin, OfxPropertySetHandle, OfxStatus},
    image_effect_v1::image_effect::kOfxImageEffectPluginApi,
};

// …

const EFFECT_PLUGIN_STRUCTS: [*const OfxPlugin; 1] = [&EFFECT_PLUGIN_STRUCT_FOO];

#[unsafe(no_mangle)]
pub extern "C" fn OfxGetNumberOfPlugins() -> c_int {
    EFFECT_PLUGIN_STRUCTS.len() as c_int
}

#[unsafe(no_mangle)]
pub extern "C" fn OfxGetPlugin(nth: c_int) -> *const OfxPlugin {
    EFFECT_PLUGIN_STRUCTS
        .get(nth as usize)
        .copied()
        .unwrap_or(std::ptr::null())
}

static EFFECT_PLUGIN_STRUCT_FOO: OfxPlugin = OfxPlugin {
    pluginApi: kOfxImageEffectPluginApi.as_ptr(),
    apiVersion: 1,
    pluginIdentifier: c"org.openeffects:BasicExamplePlugin".as_ptr(),
    pluginVersionMajor: 1,
    pluginVersionMinor: 0,
    setHost: Some(set_host),
    mainEntry: Some(main_entry),
};

unsafe extern "C" fn set_host(host: *mut OfxHost) {
    todo!()
}

unsafe extern "C" fn main_entry(
    action: *const c_char,
    handle: *const c_void,
    in_args: OfxPropertySetHandle,
    out_args: OfxPropertySetHandle,
) -> OfxStatus {
    todo!()
}

// …
```

</details>

##### The `sys_helpers` way

See also:
[$REPO_ROOT/examples/ofx-guide-example-plugins-layer-sys/src/lib.rs](../../examples/ofx-guide-example-plugins-layer-sys/src/lib.rs)

<details><summary>example code</summary>

```rs
use openfx::{
    sys::generic::{OfxHost, OfxPlugin, OfxPropertySetHandle, OfxStatus},
    sys_helpers::{
        generic::{Plugins, export_plugins, plugin_struct},
        image_effect_v1::Plugin,
    },
};

// …

struct MyPlugins;
export_plugins!(MyPlugins);

impl Plugins for MyPlugins {
    fn plugins(_host: Option<*const OfxHost>) -> Vec<OfxPlugin> {
        vec![plugin_struct!(BasicExamplePlugin)]
    }
}

struct BasicExamplePlugin;
impl Plugin for BasicExamplePlugin {
    const PLUGIN_IDENTIFIER: &'static CStr = c"org.openeffects:BasicExamplePlugin";
    const PLUGIN_VERSION_MAJOR: c_uint = 1;
    const PLUGIN_VERSION_MINOR: c_uint = 0;
    extern "C" fn set_host(host: *mut OfxHost) {
        todo!()
    }
    extern "C" fn main_entry(
        action: *const c_char,
        handle: *const c_void,
        in_args: OfxPropertySetHandle,
        out_args: OfxPropertySetHandle,
    ) -> OfxStatus {
        todo!()
    }
}

// …
```

</details>

#### WIP: writing plugins in layer `low_plugin`

See also:
[$REPO_ROOT/examples/ofx-guide-example-plugins-layer-low/src/lib.rs](../../examples/ofx-guide-example-plugins-layer-low/src/lib.rs)

<details><summary>example code</summary>

```rs
use openfx::{
    low::Status,
    low_plugin::{Action, Host, Plugin},
    sys_helpers::generic::{PluginStruct, Plugins, export_plugins, plugin_struct},
};

// …

struct MyPlugins;
export_plugins!(MyPlugins);

impl Plugins for MyPlugins {
    fn plugins(_host: Option<*const OfxHost>) -> Vec<PluginStruct> {
        vec![plugin_struct!(BasicExamplePlugin)]
    }
}

struct BasicExamplePlugin;
impl Plugin for BasicExamplePlugin {
    const PLUGIN_IDENTIFIER: &'static CStr = c"org.openeffects:BasicExamplePlugin";
    const PLUGIN_VERSION_MAJOR: c_uint = 1;
    const PLUGIN_VERSION_MINOR: c_uint = 0;
    fn set_host(host: Host) {
        todo!()
    }
    fn main_entry(action: Action) -> Status {
        match action {
            _ => todo!(),
        }
    }
}

// …
```

</details>

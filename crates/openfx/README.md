# Crate `openfx`

[<img alt="github" src="https://img.shields.io/badge/github-kreantio/openfx-rs?logo=github" height="20">](https://github.com/kreantio/openfx-rs)
[![Latest version](https://img.shields.io/crates/v/openfx.svg)](https://crates.io/crates/openfx)
[![Documentation](https://docs.rs/openfx/badge.svg)](https://docs.rs/openfx)
[![Crates.io License](https://img.shields.io/crates/l/openfx)](https://github.com/kreantio/openfx-rs/blob/main/LICENSE)

> [!CAUTION]
>
> This crate is in the early stages of development:
>
> - Its API is expected to change.
> - At present, only `openfx::sys` is fully implemented.
>   `openfx::{low, low_plugin}` is still under development.

This crate provides bindings for the OpenFX API in these abstraction layers:

- layer `sys` (`openfx::sys`): raw low-level bindings generated from the OpenFX
  C headers.
- layer `low` (`openfx::{low, low_plugin}`): unsafe low-level bindings built on
  top of the `sys` bindings, with a little more runtime overhead. Here, values
  from the `sys` bindings are cast into stronger types generated from the
  metadata in the official OpenFX repository.

## for Plugin Development

### Building Plugins

OpenFX plugins require to be packed in a specific way. Therefore, you need to do
some additional work after the binary is built. See:
[OpenFX reference / Packaging OFX Plug-ins]. You can also have a look at
[some of the scripts I use for bundling the example plugins].

[OpenFX reference / Packaging OFX Plug-ins]: https://openfx.readthedocs.io/en/latest/Reference/ofxPackaging.html
[some of the scripts I use for bundling the example plugins]: ../../examples/layer-low/official-guide-example-plugins/scripts/bundle.sh

Please don't forget that the `Cargo.toml` of your plugin crates should contain:

```toml
[lib]
crate-type = ["cdylib"]
```

### Examples

See also: [$REPO_ROOT/examples/README.md](../../examples/README.md)

#### Skeleton

<details open><summary>example code</summary>

```rs
use std::ffi::{CStr, c_uint};

use openfx::{
    export_plugins,
    low::{Status, StatusError},
    low_plugin::{Host, Plugin, actions::image_effect::ImageEffectAction},
    plugin_struct, sys, sys_plugin_helpers,
};

struct MyPlugins;
export_plugins!(MyPlugins);

impl sys_plugin_helpers::Plugins for MyPlugins {
    fn plugins(_host: Option<*const sys::core::OfxHost>) -> Vec<sys::core::OfxPlugin> {
        vec![plugin_struct!(BasicExamplePlugin)]
    }
}

struct BasicExamplePlugin;
impl Plugin for BasicExamplePlugin {
    const PLUGIN_IDENTIFIER: &'static CStr = c"org.foo.bar";
    const PLUGIN_VERSION_MAJOR: c_uint = 1;
    const PLUGIN_VERSION_MINOR: c_uint = 0;
    fn set_host(host: Host) {
        todo!()
    }
    fn main_entry(action: ImageEffectAction) -> std::result::Result<Status, StatusError> {
        match action {
            _ => todo!(),
        }
    }
}
```

</details>

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
>
> Although `openfx::{low, low_plugin}` already provides some functionality, it
> is not currently recommended for use.

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
[some of the scripts I use for bundling the example plugins]: ../../examples/ofx-guide-example-plugins-layer-sys/scripts/bundle.ts

Please don't forget that the `Cargo.toml` of your plugin crates should contain:

```toml
[lib]
crate-type = ["cdylib"]
```

### Examples

#### WIP: writing plugins in layer `low_plugin`

See also:
[$REPO_ROOT/examples/ofx-guide-example-plugins-layer-low/src/lib.rs](../../examples/ofx-guide-example-plugins-layer-low/src/lib.rs)

<details open><summary>example code</summary>

```rs
use openfx::{
    low::Status,
    low_plugin::{Action, Host, Plugin},
    sys_plugin_helpers::{Plugins, export_plugins, plugin_struct},
    sys::generic::core::{OfxHost, OfxPlugin},
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

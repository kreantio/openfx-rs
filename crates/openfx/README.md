# crate `openfx`

This crate provides bindings for the OpenFX API in 4 abstraction layers:

- layer `sys` (`openfx::*::sys`): raw low-level bindings generated from the
  OpenFX C headers.
- layer `low` (`openfx::*::low`): unsafe low-level bindings built on top of the
  `sys` bindings, built around `struct`s and associated functions.
- layer `low_strong` (`openfx::*::low_strong`): unsafe low-level bindings built
  on top of the `low` bindings, where types of values from the `low` bindings
  are converted to stronger types generated from the official C++ bindings
  (`$OFX_REPO/openfx-cpp/include`) with runtime overhead.
- layer `high` (`openfx::*::high`): (generally) safe and rust-idiomatic
  high-level bindings built on top of the `low_strong` bindings. There
  definitely is some runtime overhead.

The first three layers are just building blocks for the last layer. You can
indeed use them directly, but it is recommended to use the `high` layer bindings
for most use cases.

## for Plugin Development

### Examples

#### writing plugins in layer `sys`

- [ ] TODO

```rs
use openfx::generic::sys::{OfxHost, OfxPlugin, OfxPropertySetHandle, OfxStat};

// …

#[unsafe(no_mangle)]
pub extern "C" fn OfxGetNumberOfPlugins() -> c_int {
    1
}

#[unsafe(no_mangle)]
pub extern "C" fn OfxGetPlugin(nth: c_int) -> *const OfxPlugin {
    if nth == 0 {
        return &EFFECT_PLUGIN_STRUCT;
    }

    std::ptr::null()
}

static EFFECT_PLUGIN_STRUCT: OfxPlugin = OfxPlugin {
    pluginApi: kOfxImageEffectPluginApi.as_ptr(),
    apiVersion: 1,
    pluginIdentifier: c"org.openeffects:BasicExamplePlugin".as_ptr(),
    pluginVersionMajor: 1,
    pluginVersionMinor: 0,
    setHost: Some(set_host),
    mainEntry: Some(main_entry),
};

unsafe extern "C" fn set_host(host_struct: *mut OfxHost) {
    todo!()
}

unsafe extern "C" fn main_entry(
    action: *const c_char,
    handle: *const c_void,
    in_args: OfxPropertySetHandle,
    out_args: OfxPropertySetHandle,
) -> OfxStat {
    todo!()
}

// …
```

#### writing plugins in layer `low`

- [ ] TODO

```rs
use openfx::generic::low::{export_plugins, plugin_struct, PluginStruct, Plugins};
use openfx::generic::sys::{OfxHost, OfxPropertySetHandle, OfxStat};
use openfx::image_effect_v1::low::Plugin;

// …

struct MyPlugins;
export_plugins!(MyPlugins);

impl Plugins for MyPlugins {
    fn plugins() -> Vec<PluginStruct> {
        vec![plugin_struct!(BasicExamplePlugin)]
    }
}

struct BasicExamplePlugin;

impl Plugin for BasicExamplePlugin {
    const PLUGIN_IDENTIFIER: &'static CStr = c"org.openeffects:BasicExamplePlugin";
    const PLUGIN_VERSION: (u32, u32) = (1, 0);
    fn set_host(host_struct: *mut OfxHost) {
        todo!()
    }
    fn main_entry(
        action: *const c_char,
        handle: *const c_void,
        in_args: OfxPropertySetHandle,
        out_args: OfxPropertySetHandle,
    ) -> OfxStat {
        todo!()
    }
}

// …
```

#### writing plugins in layer `low_strong`

- [ ] TODO

```rs
use openfx::generic::low::{export_plugins, plugin_struct, PluginStruct, Plugins};
use openfx::generic::low_strong::Stat;
use openfx::image_effect_v1::low_strong::{Action, Host, LowLayerWrapper, Plugin};

// …

struct MyPlugins;
export_plugins!(MyPlugins);

impl Plugins for MyPlugins {
    fn plugins() -> Vec<PluginStruct> {
        vec![plugin_struct!(LowLayerWrapper<BasicExamplePlugin>)]
    }
}

struct BasicExamplePlugin;

impl Plugin for BasicExamplePlugin {
    const PLUGIN_IDENTIFIER: &'static CStr = c"org.openeffects:BasicExamplePlugin";
    const PLUGIN_VERSION: (u32, u32) = (1, 0);
    fn set_host(host_struct: Host) {
        todo!()
    }
    fn main_entry(action: Action) -> Stat {
        match action {
            _ => todo!(),
        }
    }
}

// …
```

#### writing plugins in layer `high`

- [ ] TODO

```rs
use openfx::generic::low::{PluginStruct, Plugins, export_plugins, plugin_struct};
use openfx::image_effect_v1::high::{Context, LowLayerWrapper, Plugin, PluginInstance, actions};

// …

struct MyPlugins;
export_plugins!(MyPlugins);

impl Plugins for MyPlugins {
    fn plugins() -> Vec<PluginStruct> {
        vec![plugin_struct!(LowLayerWrapper<BasicExamplePlugin>)]
    }
}

struct BasicExamplePlugin;

impl Plugin for BasicExamplePlugin {
    const PLUGIN_IDENTIFIER: &'static str = "org.openeffects:BasicExamplePlugin";
    const PLUGIN_VERSION: (u32, u32) = (1, 0);
    type Instance = BasicExamplePluginInstance;
    fn describe(
        &self,
        ctx: &actions::describe::Context,
        in_args: &actions::describe::InArgs,
    ) -> actions::describe::Result {
        todo!()
    }
    fn create_instance(&self, ctx: &Context) -> actions::create_instance::Result<Self::Instance> {
        todo!()
    }
    // …
}

#[derive(Default)]
struct BasicExamplePluginInstance {}

impl PluginInstance for BasicExamplePluginInstance {
    fn get_region_of_definition(
        &self,
        ctx: &Context,
        in_args: &actions::get_region_of_definition::InArgs,
    ) -> actions::get_region_of_definition::Result {
        todo!()
    }
    // …
}

// …
```

## for Host Development

- [ ] TODO

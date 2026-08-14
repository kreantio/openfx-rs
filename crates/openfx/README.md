# crate `openfx`

This crate provides bindings for the OpenFX API in 3 abstraction layers:

- layer `sys` (`openfx::*::sys`): raw low-level bindings generated from the
  OpenFX C headers.
- layer `low_typed` (`openfx::*::low_typed`): unsafe low-level bindings built on
  top of the `sys` bindings, where types of values from the `sys` bindings are
  converted to stronger types generated from the official C++ bindings
  (`$OFX_REPO/openfx-cpp/include`) with runtime overhead.
- layer `high` (`openfx::*::high`): (generally) safe and rust-idiomatic
  high-level bindings built on top of the `low_typed` bindings. There definitely
  is some runtime overhead.

The first two layers are just building blocks for the layer `high`. Although you
can use them directly, it is recommended to use the `high` layer bindings for
most use cases.

## for Plugin Development

### Examples

#### writing plugins in layer `sys`

<details><summary>The barest way</summary>

```rs
use std::ffi::c_int;

use openfx::{
    generic::sys::core::{OfxHost, OfxPlugin, OfxPropertySetHandle, OfxStatus},
    image_effect_v1::sys::image_effect::kOfxImageEffectPluginApi,
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

<details><summary>The <code>sys_helpers</code> way</summary>

See also:
[$REPO_ROOT/examples/ofx-guide-example-plugins-layer-sys/src/lib.rs](../../examples/ofx-guide-example-plugins-layer-sys/src/lib.rs)

```rs
use openfx::{
    generic::{
        sys::{OfxHost, OfxPlugin, OfxPropertySetHandle, OfxStatus},
        sys_helpers::{Plugins, export_plugins, plugin_struct},
    },
    image_effect_v1::sys_helpers::Plugin,
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

#### writing plugins in layer `low_typed`

<details><summary>TODO: example code</summary>

```rs
use openfx::generic::low_typed::Status;
use openfx::generic::sys_helpers::{export_plugins, plugin_struct, PluginStruct, Plugins};
use openfx::image_effect_v1::low_typed::{Action, Host, Plugin};

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

#### writing plugins in layer `high`

<details><summary>TODO: example code</summary>

```rs
use openfx::generic::sys_helpers::{PluginStruct, Plugins, export_plugins, plugin_struct};
use openfx::image_effect_v1::high::{Context, Plugin, PluginInstance, actions};

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
    const PLUGIN_IDENTIFIER: &'static str = "org.openeffects:BasicExamplePlugin";
    const PLUGIN_VERSION_MAJOR: c_uint = 1;
    const PLUGIN_VERSION_MINOR: c_uint = 0;
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

</details>

## for Host Development

- [ ] TODO

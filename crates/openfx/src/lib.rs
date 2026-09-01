#![feature(decl_macro)]

pub mod low;
pub mod low_plugin;
pub mod sys;
pub mod sys_helpers;

/// a workaround for `include_c_bindings`. TODO: remove this.
#[allow(unused_imports)]
pub(crate) mod sys_umbrella {
    pub use crate::sys::generic::core::*;
    pub use crate::sys::generic::property::*;
    pub use crate::sys::image_effect_v1::colour::*;
    pub use crate::sys::image_effect_v1::dialog::*;
    pub use crate::sys::image_effect_v1::draw_suite::*;
    pub use crate::sys::image_effect_v1::gpu_render::*;
    pub use crate::sys::image_effect_v1::image_effect::*;
    pub use crate::sys::image_effect_v1::interact::*;
    pub use crate::sys::image_effect_v1::key_syms::*;
    pub use crate::sys::image_effect_v1::memory::*;
    pub use crate::sys::image_effect_v1::message::*;
    pub use crate::sys::image_effect_v1::multi_thread::*;
    pub use crate::sys::image_effect_v1::old::*;
    pub use crate::sys::image_effect_v1::open_gl_render::*;
    pub use crate::sys::image_effect_v1::param::*;
    pub use crate::sys::image_effect_v1::parametric_param::*;
    pub use crate::sys::image_effect_v1::pixels::*;
    pub use crate::sys::image_effect_v1::progress::*;
    pub use crate::sys::image_effect_v1::time_line::*;
}

mod sys_checks {
    use crate::sys_umbrella::*;
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/generated/c_bindings/_checks.rs",
    ));
}

#[allow(unused_imports)]
pub(crate) mod sys_helpers_properties_umbrella {
    pub use crate::sys_helpers::generic::properties::*;
    pub use crate::sys_helpers::image_effect_v1::properties::*;
}

#![feature(decl_macro)]

pub mod generic;
pub mod image_effect_v1;
pub(crate) mod internal;

/// a workaround for `include_c_bindings`. TODO: remove this.
#[allow(unused_imports)]
pub(crate) mod sys_umbrella {
    pub use crate::generic::sys::core::*;
    pub use crate::generic::sys::property::*;
    pub use crate::image_effect_v1::sys::colour::*;
    pub use crate::image_effect_v1::sys::dialog::*;
    pub use crate::image_effect_v1::sys::draw_suite::*;
    pub use crate::image_effect_v1::sys::gpu_render::*;
    pub use crate::image_effect_v1::sys::image_effect::*;
    pub use crate::image_effect_v1::sys::interact::*;
    pub use crate::image_effect_v1::sys::key_syms::*;
    pub use crate::image_effect_v1::sys::memory::*;
    pub use crate::image_effect_v1::sys::message::*;
    pub use crate::image_effect_v1::sys::multi_thread::*;
    pub use crate::image_effect_v1::sys::old::*;
    pub use crate::image_effect_v1::sys::open_gl_render::*;
    pub use crate::image_effect_v1::sys::param::*;
    pub use crate::image_effect_v1::sys::parametric_param::*;
    pub use crate::image_effect_v1::sys::pixels::*;
    pub use crate::image_effect_v1::sys::progress::*;
    pub use crate::image_effect_v1::sys::time_line::*;
}

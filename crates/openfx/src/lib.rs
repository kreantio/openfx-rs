#[allow(nonstandard_style)]
#[allow(unused_parens)]
#[allow(clippy::tabs_in_doc_comments)]
pub(crate) mod generated;

pub mod low;
pub mod low_plugin;
pub mod sys;
pub mod sys_plugin_helpers;

/// a workaround for `include_c_bindings`. TODO: remove this.
#[allow(unused_imports)]
pub(crate) mod sys_umbrella {
    pub use crate::sys::colour::*;
    pub use crate::sys::core::*;
    pub use crate::sys::dialog::*;
    pub use crate::sys::draw_suite::*;
    pub use crate::sys::gpu_render::*;
    pub use crate::sys::image_effect::*;
    pub use crate::sys::interact::*;
    pub use crate::sys::key_syms::*;
    pub use crate::sys::memory::*;
    pub use crate::sys::message::*;
    pub use crate::sys::multi_thread::*;
    pub use crate::sys::old::*;
    pub use crate::sys::open_gl_render::*;
    pub use crate::sys::param::*;
    pub use crate::sys::parametric_param::*;
    pub use crate::sys::pixels::*;
    pub use crate::sys::progress::*;
    pub use crate::sys::property::*;
    pub use crate::sys::time_line::*;
}

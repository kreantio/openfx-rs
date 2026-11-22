/// ## Lifetime Management
///
/// The user is responsible for ensuring the validity of instances of the types
/// that wrap OpenFX handles defined in this crate. This crate makes no
/// assumptions about the lifetimes of OpenFX handles and provides no RAII. It
/// always hands out the wrapper types themselves and takes them by reference.
///
/// Note that for other types, such as [`std::ffi::CStr`], this crate still
/// follows the usual Rust ownership and borrowing rules.
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

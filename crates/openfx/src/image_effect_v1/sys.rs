//! This module is maintained manually for now.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]
#![allow(clippy::all)]

pub use crate::generic::sys::*;

include_c_bindings!(colour);
include_c_bindings!(dialog);
include_c_bindings!(draw_suite);
include_c_bindings!(gpu_render);
include_c_bindings!(image_effect);
include_c_bindings!(interact);
include_c_bindings!(key_syms);
include_c_bindings!(memory);
include_c_bindings!(message);
include_c_bindings!(multi_thread);
include_c_bindings!(old);
include_c_bindings!(open_gl_render);
include_c_bindings!(param);
include_c_bindings!(parametric_param);
include_c_bindings!(pixels);
include_c_bindings!(progress);
include_c_bindings!(property);
include_c_bindings!(time_line);

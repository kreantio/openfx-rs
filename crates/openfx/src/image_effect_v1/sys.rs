//! The list of C bindings included in this module is currently maintained
//! manually.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]
#![allow(clippy::all)]

use crate::generic::sys::*;

include_mod_c_bindings!(colour);
include_mod_c_bindings!(dialog);
include_mod_c_bindings!(draw_suite);
include_mod_c_bindings!(gpu_render);
include_mod_c_bindings!(image_effect);
include_mod_c_bindings!(interact);
include_mod_c_bindings!(key_syms);
include_mod_c_bindings!(memory);
include_mod_c_bindings!(message);
include_mod_c_bindings!(multi_thread);
include_mod_c_bindings!(old);
include_mod_c_bindings!(open_gl_render);
include_mod_c_bindings!(param);
include_mod_c_bindings!(parametric_param);
include_mod_c_bindings!(pixels);
include_mod_c_bindings!(progress);
include_mod_c_bindings!(time_line);

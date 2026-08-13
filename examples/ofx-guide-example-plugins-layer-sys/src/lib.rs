#![feature(decl_macro)]

use std::ffi::c_int;

use openfx::generic::sys::core::OfxPlugin;

use crate::{
    plugin_1_basics::EFFECT_PLUGIN_STRUCT_BASICS, plugin_2_invert::EFFECT_PLUGIN_STRUCT_INVERT,
    plugin_3_gain::EFFECT_PLUGIN_STRUCT_GAIN, plugin_4_saturation::EFFECT_PLUGIN_STRUCT_SATURATION,
    plugin_5_circle::EFFECT_PLUGIN_STRUCT_CIRCLE,
};

mod definitions;

mod helpers;
mod plugin_1_basics;
mod plugin_2_invert;
mod plugin_3_gain;
mod plugin_4_saturation;
mod plugin_5_circle;

const EFFECT_PLUGIN_STRUCTS: [*const OfxPlugin; 5] = [
    &EFFECT_PLUGIN_STRUCT_BASICS,
    &EFFECT_PLUGIN_STRUCT_INVERT,
    &EFFECT_PLUGIN_STRUCT_GAIN,
    &EFFECT_PLUGIN_STRUCT_SATURATION,
    &EFFECT_PLUGIN_STRUCT_CIRCLE,
];

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

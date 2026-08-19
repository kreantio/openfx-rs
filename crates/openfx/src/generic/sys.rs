//! The list of C bindings included in this module is currently maintained
//! manually.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]
#![allow(clippy::all)]

macro_rules! include_mod_c_bindings {
    ($name:ident) => {
        include_mod_c_bindings!($name, {});
    };
    ($name:ident, {$($additional_items:tt)*}) => {
        pub mod $name {
            include!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/generated/c_bindings/",
                stringify!($name),
                ".rs"
            ));

            mod checks {
                use crate::sys_umbrella::*;
                include!(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/generated/c_bindings/checks/",
                    stringify!($name),
                    ".rs"
                ));
            }

            $($additional_items)*
        }
    }
}

pub(crate) use include_mod_c_bindings;

include_mod_c_bindings!(core, {
    unsafe impl Sync for OfxPlugin {}
});
include_mod_c_bindings!(property);

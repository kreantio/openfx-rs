use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper-core.h");

    println!(
        "cargo:rustc-env=TARGET={}",
        std::env::var("TARGET").unwrap()
    );

    let generated_dir = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/generated"));
    std::fs::create_dir_all(&generated_dir).expect("Unable to create generated directory");

    let generated_c_bindings_dir = generated_dir.join("c_bindings");
    std::fs::create_dir_all(&generated_c_bindings_dir)
        .expect("Unable to create generated/c_bindings directory");

    let vendor_c_include_dir = PathBuf::from(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/vendor/openfx/include"
    ));

    openfx_bindgen::bindings_for_c_headers::generate_bindings_for_c_headers(
        vendor_c_include_dir,
        &generated_c_bindings_dir,
    );
}

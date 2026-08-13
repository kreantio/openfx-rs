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

    // {
    //     // The bindgen::Builder is the main entry point to bindgen, and lets you
    //     // build up options for the resulting bindings.
    //     let bindings = bindgen::Builder::default()
    //         // The input header we would like to generate
    //         // bindings for.
    //         .header("wrapper-core.h")
    //         .allowlist_function("") // blocking all
    //         .allowlist_type("Ofx.+")
    //         .allowlist_var("kOfx.+")
    //         // Tell cargo to invalidate the built crate whenever any of the
    //         // included header files changed.
    //         .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
    //         .generate_cstr(true)
    //         // Finish the builder and generate the bindings.
    //         .generate()
    //         // Unwrap the Result and panic on failure.
    //         .expect("Unable to generate bindings");

    //     // Write the bindings to the $OUT_DIR/bindings.rs file.
    //     bindings
    //         .write_to_file(generated_dir.join("core.rs"))
    //         .expect("Couldn't write bindings!");
    // }
}

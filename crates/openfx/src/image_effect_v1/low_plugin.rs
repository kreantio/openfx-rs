pub mod actions {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/generated/code_from_cpp/low_actions_plugin.rs",
    ));
}

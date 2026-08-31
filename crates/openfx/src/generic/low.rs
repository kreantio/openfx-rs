include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/generated/code_from_c/low_statuses.rs",
));

pub type Result<T> = std::result::Result<T, Status>;

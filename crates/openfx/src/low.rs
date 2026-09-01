include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/generated/code_from_c/low_statuses.rs",
));

pub type Result<T> = std::result::Result<T, Status>;

pub mod enums {
    #![allow(non_camel_case_types)]

    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/generated/code_from_cpp/low_enums.rs",
    ));
}

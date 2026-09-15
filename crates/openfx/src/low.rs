use crate::sys_umbrella::{OfxStatus, kOfxStatOK};

include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/generated/code_from_c/low_statuses.rs",
));

pub type Result<T> = std::result::Result<T, Status>;

impl Status {
    pub fn result_from(status: OfxStatus) -> Result<()> {
        if status == kOfxStatOK {
            Ok(())
        } else {
            Err(Status::from(status))
        }
    }
}

pub mod enums {
    #![allow(non_camel_case_types)]

    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/generated/code_from_cpp/low_enums.rs",
    ));

    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/generated/code_from_c/low_enums_from_c.rs",
    ));
}

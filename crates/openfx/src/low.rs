use crate::sys_umbrella::{OfxStatus, kOfxStatOK};

include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/generated/low/c_statuses.rs",
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
        "/src/generated/low/meta_enums.rs",
    ));

    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/generated/low/c_enums.rs",
    ));
}

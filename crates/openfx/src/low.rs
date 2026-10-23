use crate::sys_umbrella::{OfxStatus, kOfxStatOK};

pub use crate::generated::low::c_statuses::*;

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

    pub use crate::generated::low::c_enums::*;
    pub use crate::generated::low::meta_enums::*;
}

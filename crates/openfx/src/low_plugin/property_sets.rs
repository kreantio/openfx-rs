use std::{
    ffi::{CStr, CString, c_char},
    ptr::NonNull,
};

include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/generated/code_from_metadata/low_plugin_property_sets.rs",
));

pub trait EnumProperty: Sized + Clone + Copy {
    /// ## Safety
    ///
    /// `ptr` must either:
    /// - be `None`, or
    /// - be valid and point to a null-terminated C string.
    unsafe fn try_from_ptr(ptr: Option<NonNull<c_char>>) -> Option<Self>;
    fn as_cstr(&self) -> &CStr;
}

pub trait AsEnumPropertyRawPtr<T: EnumProperty> {
    fn as_raw_ptr(&self) -> *const c_char;
}

/// ## Safety
///
/// If an enum variant is unknown to this crate, a pointer to that variant's
/// value is stored. The underlying data is controlled by the openFX host, and
/// it is the caller's responsibility to ensure that the pointer remains valid
/// while it is in use.
///
/// To keep the unknown variant value around, call [`Self::to_owned`] while this
/// pointer is still valid.
#[derive(Clone, Copy)]
pub enum LosslessEnumProperty<T: EnumProperty> {
    Known(T),
    /// Use [`Self::matches`] instead. Even if matching this variant for
    /// unrecognized values works now, it will break when this crate adds
    /// dedicated variants for those values in a future release.
    UnknownDontUseThisDirectly(Option<NonNull<c_char>>),
}

#[derive(Clone)]
pub enum LosslessEnumPropertyOwned<T: EnumProperty> {
    Known(T),
    /// Use [`Self::matches`] instead. Even if matching this variant for
    /// unrecognized values works now, it will break when this crate adds
    /// dedicated variants for those values in a future release.
    UnknownDontUseThisDirectly(Option<CString>),
}

impl<T: EnumProperty> LosslessEnumProperty<T> {
    /// ## Safety
    ///
    /// `ptr` must either:
    /// - be `None`, or
    /// - be valid and point to a null-terminated C string.
    ///
    /// See the Safety section of [`LosslessEnumProperty`] for more information.
    pub unsafe fn from_ptr(ptr: Option<NonNull<c_char>>) -> Self {
        if let Some(value) = unsafe { T::try_from_ptr(ptr) } {
            Self::Known(value)
        } else {
            Self::UnknownDontUseThisDirectly(ptr)
        }
    }

    /// ## Safety
    ///
    /// See [`Self::from_ptr`] for more information.
    pub unsafe fn from_raw_ptr(ptr: *const c_char) -> Self {
        unsafe { Self::from_ptr(NonNull::new(ptr as *mut c_char)) }
    }

    pub fn known(&self) -> Option<Self> {
        match self {
            Self::Known(value) => Some(Self::Known(*value)),
            _ => None,
        }
    }

    /// ## Safety
    ///
    /// If the enum variant is unknown to this crate, the caller must ensure
    /// that the pointer to the unknown variant remains valid for the duration
    /// of this call.
    ///
    /// See the Safety section of [`LosslessEnumProperty`] for more information.
    pub unsafe fn to_owned(&self) -> LosslessEnumPropertyOwned<T> {
        match self {
            Self::Known(value) => LosslessEnumPropertyOwned::Known(*value),
            Self::UnknownDontUseThisDirectly(ptr) => {
                LosslessEnumPropertyOwned::UnknownDontUseThisDirectly(
                    ptr.map(|p| unsafe { CString::from_raw(p.as_ptr()) }),
                )
            }
        }
    }

    /// ## Safety
    ///
    /// If the comparison involves a variant unknown to this crate, the caller
    /// must ensure that the pointer to the unknown variant remains valid for
    /// the duration of this call.
    ///
    /// See the Safety section of [`Self`] for more information.
    pub unsafe fn matches(&self, val: &std::ffi::CStr) -> bool {
        match self {
            Self::Known(value) => value.as_cstr() == val,
            Self::UnknownDontUseThisDirectly(ptr) => {
                ptr.is_some_and(|p| unsafe { std::ffi::CStr::from_ptr(p.as_ptr()) } == val)
            }
        }
    }
}

impl<T: EnumProperty> AsEnumPropertyRawPtr<T> for LosslessEnumProperty<T> {
    /// ## Safety
    ///
    /// See the Safety section of [`Self`] for more information.
    fn as_raw_ptr(&self) -> *const std::os::raw::c_char {
        match self {
            Self::Known(value) => value.as_cstr().as_ptr(),
            Self::UnknownDontUseThisDirectly(ptr) => ptr.map_or(std::ptr::null(), |p| p.as_ptr()),
        }
    }
}

impl<T: EnumProperty> LosslessEnumPropertyOwned<T> {
    pub fn known(&self) -> Option<Self> {
        match self {
            Self::Known(value) => Some(Self::Known(*value)),
            _ => None,
        }
    }

    pub fn matches(&self, val: &std::ffi::CStr) -> bool {
        match self {
            Self::Known(value) => value.as_cstr() == val,
            Self::UnknownDontUseThisDirectly(cstr) => cstr.as_ref().is_some_and(|cstr| cstr == val),
        }
    }
}

impl<T: EnumProperty> AsEnumPropertyRawPtr<T> for LosslessEnumPropertyOwned<T> {
    fn as_raw_ptr(&self) -> *const std::os::raw::c_char {
        match self {
            Self::Known(value) => value.as_cstr().as_ptr(),
            Self::UnknownDontUseThisDirectly(ptr) => {
                ptr.as_ref().map_or(std::ptr::null(), |p| p.as_ptr())
            }
        }
    }
}

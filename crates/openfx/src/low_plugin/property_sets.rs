use std::{
    ffi::{CStr, CString, c_char},
    ptr::NonNull,
};

include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/generated/code_from_metadata/low_plugin_property_sets.rs",
));

pub trait KnownEnumProperty: Sized + Clone + Copy + PartialEq + Eq {
    /// ## Safety
    ///
    /// `ptr` must be valid and point to a null-terminated C string.
    unsafe fn try_from_ptr(ptr: NonNull<c_char>) -> Option<Self>;
    fn as_cstr(&self) -> &CStr;
}

#[derive(Clone, Copy)]
pub struct EnumPropertyPtr<T: KnownEnumProperty> {
    inner: Option<NonNull<c_char>>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: KnownEnumProperty> EnumPropertyPtr<T> {
    pub fn new(inner: Option<NonNull<c_char>>) -> Self {
        Self {
            inner,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn as_ptr(&self) -> Option<NonNull<c_char>> {
        self.inner
    }
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
pub enum LosslessEnumProperty<T: KnownEnumProperty> {
    Known(T),
    /// Use [`Self::matches`] instead. Even if matching this variant for
    /// unrecognized values works now, it will break when this crate adds
    /// dedicated variants for those values in a future release.
    UnknownDontUseThisDirectly(Option<NonNull<c_char>>),
}

#[derive(Clone, PartialEq, Eq)]
pub enum LosslessEnumPropertyOwned<T: KnownEnumProperty> {
    Known(T),
    /// Use [`Self::matches`] instead. Even if matching this variant for
    /// unrecognized values works now, it will break when this crate adds
    /// dedicated variants for those values in a future release.
    UnknownDontUseThisDirectly(Option<CString>),
}

impl<T: KnownEnumProperty> LosslessEnumProperty<T> {
    /// ## Safety
    ///
    /// `ptr` must either:
    /// - be `None`, or
    /// - be valid and point to a null-terminated C string.
    ///
    /// See the Safety section of [`LosslessEnumProperty`] for more information.
    pub unsafe fn from_ptr(ptr: Option<NonNull<c_char>>) -> Self {
        let Some(ptr) = ptr else {
            return Self::UnknownDontUseThisDirectly(None);
        };

        if let Some(value) = unsafe { T::try_from_ptr(ptr) } {
            Self::Known(value)
        } else {
            Self::UnknownDontUseThisDirectly(Some(ptr))
        }
    }

    /// ## Safety
    ///
    /// See [`Self::from_ptr`] for more information.
    pub unsafe fn from_raw_ptr(ptr: *const c_char) -> Self {
        unsafe { Self::from_ptr(NonNull::new(ptr as *mut c_char)) }
    }

    pub fn known(&self) -> Option<T> {
        match self {
            Self::Known(value) => Some(*value),
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
                    ptr.map(|p| unsafe { CStr::from_ptr(p.as_ptr()).to_owned() }),
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
    /// See the Safety section of [`LosslessEnumProperty`] for more information.
    pub unsafe fn matches(&self, val: &std::ffi::CStr) -> bool {
        match self {
            Self::Known(value) => value.as_cstr() == val,
            Self::UnknownDontUseThisDirectly(ptr) => {
                ptr.is_some_and(|p| unsafe { std::ffi::CStr::from_ptr(p.as_ptr()) } == val)
            }
        }
    }
}

impl<T: KnownEnumProperty> PartialEq<T> for LosslessEnumProperty<T> {
    fn eq(&self, other: &T) -> bool {
        match self {
            Self::Known(value) => value == other,
            Self::UnknownDontUseThisDirectly(_) => false,
        }
    }
}

impl<T: KnownEnumProperty> From<&LosslessEnumProperty<T>> for EnumPropertyPtr<T> {
    /// ## Safety
    ///
    /// See the Safety section of [`LosslessEnumProperty`] for more information.
    fn from(value: &LosslessEnumProperty<T>) -> Self {
        match value {
            LosslessEnumProperty::Known(v) => {
                Self::new(NonNull::new(v.as_cstr().as_ptr() as *mut c_char))
            }
            LosslessEnumProperty::UnknownDontUseThisDirectly(ptr) => Self::new(*ptr),
        }
    }
}

impl<T: KnownEnumProperty> LosslessEnumPropertyOwned<T> {
    /// ## Safety
    ///
    /// `ptr` must either:
    /// - be `None`, or
    /// - be valid and point to a null-terminated C string.
    ///
    /// See the Safety section of [`LosslessEnumProperty`] for more information.
    pub unsafe fn from_ptr(ptr: Option<NonNull<c_char>>) -> Self {
        let Some(ptr) = ptr else {
            return Self::UnknownDontUseThisDirectly(None);
        };

        if let Some(value) = unsafe { T::try_from_ptr(ptr) } {
            Self::Known(value)
        } else {
            Self::UnknownDontUseThisDirectly(Some(unsafe {
                CStr::from_ptr(ptr.as_ptr()).to_owned()
            }))
        }
    }

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

impl<T: KnownEnumProperty> PartialEq<T> for LosslessEnumPropertyOwned<T> {
    fn eq(&self, other: &T) -> bool {
        match self {
            Self::Known(value) => value == other,
            Self::UnknownDontUseThisDirectly(_) => false,
        }
    }
}

impl<T: KnownEnumProperty> From<&LosslessEnumPropertyOwned<T>> for EnumPropertyPtr<T> {
    fn from(value: &LosslessEnumPropertyOwned<T>) -> Self {
        match value {
            LosslessEnumPropertyOwned::Known(v) => {
                Self::new(NonNull::new(v.as_cstr().as_ptr() as *mut c_char))
            }
            LosslessEnumPropertyOwned::UnknownDontUseThisDirectly(cstr) => Self::new(
                cstr.as_ref()
                    .and_then(|cstr| NonNull::new(cstr.as_ptr() as *mut c_char)),
            ),
        }
    }
}

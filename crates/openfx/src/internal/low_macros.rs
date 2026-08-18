pub macro make_enum_from_idents($name:ident, $($var:ident : $var_cstr:literal),*) {
    #[derive(Debug, Clone)]
    pub enum $name {
        $($var,)*
        Other(*const std::os::raw::c_char),
    }
    impl $name {
        /// ## SAFETY
        ///
        /// - The pointer must be valid and point to a null-terminated C string.
        /// - The pointer must live at least as long as the returned [`Self`]
        ///   value.
        pub unsafe fn from_ptr(ptr: *const std::os::raw::c_char) -> Self {
            let cstr = unsafe { std::ffi::CStr::from_ptr(ptr) };
            $(
                if cstr == $var_cstr {
                    return Self::$var;
                }
            )*
            Self::Other(cstr.as_ptr())
        }

        /// ## SAFETY
        ///
        /// The returned pointer is valid as long as the [`std::ffi::CString`]
        /// inside [`Self::Other`] is not dropped.
        pub fn as_ptr(&self) -> *const std::os::raw::c_char {
            if let Self::Other(ptr) = self {
                return *ptr;
            }
            $(
                if matches!(self, Self::$var) {
                    return $var_cstr.as_ptr() as *const std::os::raw::c_char;
                }
            )*
            unreachable!()
        }
    }
}

pub macro make_enum_from_paths($name:ident, $(#[$meta:meta] $var:ident => $var_path:path),*) {
    #[derive(Debug, Clone)]
    pub enum $name {
        $(#[$meta] $var,)*
        Other(*const std::os::raw::c_char),
    }
    impl $name {
        /// ## SAFETY
        ///
        /// - The pointer must be valid and point to a null-terminated C string.
        /// - The pointer must live at least as long as the returned [`Self`]
        ///   value.
        pub unsafe fn from_ptr(ptr: *const std::os::raw::c_char) -> Self {
            let cstr = unsafe { std::ffi::CStr::from_ptr(ptr) };
            match true {
                $(_ if cstr == $var_path => Self::$var,)*
                _ => Self::Other(cstr.as_ptr()),
            }
        }

        /// ## SAFETY
        ///
        /// The returned pointer is valid as long as the [`std::ffi::CString`]
        /// inside [`Self::Other`] is not dropped.
        pub fn as_ptr(&self) -> *const std::os::raw::c_char {
            match self {
                $(Self::$var => $var_path.as_ptr(),)*
                Self::Other(ptr) => *ptr,
            }
        }
    }
}

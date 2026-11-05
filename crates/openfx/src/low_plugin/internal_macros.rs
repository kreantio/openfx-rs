macro_rules! sys_fn {
    ($self:ident, $ident:ident) => {
        $self.sys_ref().$ident.unwrap_unchecked()
    };
}
pub(crate) use sys_fn;

macro_rules! make_param_define_fns {
    ($fn_name:ident, $fn_name_no_output:ident, $prop_set_ty:ty, $type_key:ident) => {
        /// ## Safety
        ///
        /// `self` and `parameter_suite` must be valid."
        pub unsafe fn $fn_name(
            &self,
            parameter_suite: &crate::low_plugin::suites::ParameterSuiteV1,
            name: &std::ffi::CStr,
        ) -> crate::low::InboundResult<$prop_set_ty> {
            unsafe {
                self.param_define(parameter_suite, crate::sys_umbrella::$type_key, name)
                    .map_ok(<$prop_set_ty>::from)
            }
        }
        /// ## Safety
        ///
        /// `self` and `parameter_suite` must be valid."
        pub unsafe fn $fn_name_no_output(
            &self,
            parameter_suite: &crate::low_plugin::suites::ParameterSuiteV1,
            name: &std::ffi::CStr,
        ) -> crate::low::InboundResult<()> {
            unsafe {
                self.param_define_no_output(parameter_suite, crate::sys_umbrella::$type_key, name)
            }
        }
    };
}
pub(crate) use make_param_define_fns;

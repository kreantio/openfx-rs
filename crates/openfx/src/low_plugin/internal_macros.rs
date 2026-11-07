macro_rules! sys_fn {
    ($self:expr, $ident:ident) => {
        $self.sys_ref().$ident.unwrap_unchecked()
    };
}
pub(crate) use sys_fn;

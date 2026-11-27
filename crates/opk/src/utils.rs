use std::ops::Deref;

use openfx::{low::InboundResult, low_plugin::LifetimePlugin};

#[derive(Clone, Copy)]
pub struct GuaranteeSend<T: LifetimePlugin>(pub T);
/// ## Safety
///
/// This is safe only for the lifetime of the plugin from which the wrapped
/// object was obtained. This covers both single-threaded plugins and
/// multi-threaded plugins that use
/// [`openfx::sys::multi_thread::OfxMultiThreadSuiteV1`] or any other mechanism
/// that ensures the spawned threads do not outlive the plugin.
unsafe impl<T: LifetimePlugin> Send for GuaranteeSend<T> {}
impl<T: LifetimePlugin> Deref for GuaranteeSend<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// TODO!!: Add proper helper functions in [`openfx`] for
/// [`openfx::low::InboundResult`].
pub fn ok_or_status<T: Copy>(result: InboundResult<T>) -> Result<T, openfx::low::Status> {
    if let Some(value) = result.ok() {
        Ok(value)
    } else {
        Err(openfx::low::Status::from(result.sys_status()))
    }
}

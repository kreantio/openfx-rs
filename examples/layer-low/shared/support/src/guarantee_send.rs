use std::ops::Deref;

use openfx::low_plugin::{LifetimePlugin, objects::ImageClipInstance};

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

#[derive(Clone, Copy)]
pub struct GuaranteeSendClipInstance(pub ImageClipInstance);
/// ## Safety
///
/// This is safe only for the lifetime of the plugin instance from which the
/// clip instance was retrieved.
unsafe impl Send for GuaranteeSendClipInstance {}

use std::ops::Deref;

use openfx::low_plugin::{LifetimePlugin, objects::ImageClipInstance};

#[derive(Clone, Copy)]
pub struct GuaranteeSend<T: LifetimePlugin>(pub T);
/// ## Safety
///
/// This plugin does not spawn threads, so the host exclusively controls its
/// lifecycle.
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

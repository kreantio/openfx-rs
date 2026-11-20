use crate::bundles::{PluginBundle, PluginBundleManager};

/// ## TODO
///
/// Include bundle information.
impl PluginBundle {
    pub(crate) fn log_warn(&self, message: &str) {
        #[cfg(feature = "tracing")]
        tracing::warn!("{:?}: {}", self.library_file_name(), message);
    }
    pub(crate) fn log_info(&self, message: &str) {
        #[cfg(feature = "tracing")]
        tracing::info!("{:?}: {}", self.library_file_name(), message);
    }
}

/// ## TODO
///
/// Include bundle information.
impl PluginBundleManager {}

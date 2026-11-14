use std::{
    ffi::CString,
    sync::atomic::{self, AtomicU32},
};

#[derive(Default)]
pub struct UsedRenderer(AtomicU32);

impl UsedRenderer {
    const RENDERER_NO_INFO: u16 = 0;
    const RENDERER_CPU_MULTI_THREAD_SUITE: u16 = 1;
    const RENDERER_CPU: u16 = 2;
    const RENDERER_METAL_BUFFER: u16 = 3;

    pub fn store_no_info(&self) {
        self.0
            .store(Self::RENDERER_NO_INFO as u32, atomic::Ordering::Relaxed);
    }
    pub fn store_cpu_multi_thread_suite(&self, n_cpus: u16) {
        self.0.store(
            (Self::RENDERER_CPU_MULTI_THREAD_SUITE as u32) | ((n_cpus as u32) << 16),
            atomic::Ordering::Relaxed,
        );
    }
    pub fn store_cpu(&self) {
        self.0
            .store(Self::RENDERER_CPU as u32, atomic::Ordering::Relaxed);
    }
    pub fn store_metal_buffer(&self) {
        self.0.store(
            Self::RENDERER_METAL_BUFFER as u32,
            atomic::Ordering::Relaxed,
        );
    }

    pub fn assign(&self, another: &UsedRenderer) {
        let value = another.0.load(atomic::Ordering::Relaxed);
        self.0.store(value, atomic::Ordering::Relaxed);
    }

    pub fn describe(&self) -> CString {
        let value = self.0.load(atomic::Ordering::Relaxed);
        let renderer = (value & 0xFFFF) as u16;
        let n_cpus = (value >> 16) as u16;
        match renderer {
            Self::RENDERER_NO_INFO => c"--".to_owned(),
            Self::RENDERER_CPU_MULTI_THREAD_SUITE => {
                CString::new(format!("CPU (with {n_cpus} thread(s))")).unwrap()
            }
            Self::RENDERER_CPU => c"CPU (No `MultiThreadSuiteV1`)".to_owned(),
            Self::RENDERER_METAL_BUFFER => c"Metal (Buffer)".to_owned(),
            _ => c"?".to_owned(),
        }
    }
}

impl PartialEq for UsedRenderer {
    fn eq(&self, other: &Self) -> bool {
        self.0.load(atomic::Ordering::Relaxed) == other.0.load(atomic::Ordering::Relaxed)
    }
}

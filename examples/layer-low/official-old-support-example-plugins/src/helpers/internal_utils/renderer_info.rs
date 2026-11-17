use std::{
    ffi::CString,
    sync::atomic::{self, AtomicU32},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Renderer {
    Cpu,
    OpenclWithBuffers,
    OpenclWithImages,
    MetalWithBuffers,
}

#[derive(Default)]
pub struct UsedRenderer(AtomicU32);

impl UsedRenderer {
    const RENDERER_NO_INFO: u16 = 0;
    const RENDERER_NO_ELIGIBLE: u16 = u16::MAX;

    const RENDERER_CPU_MULTI_THREAD_SUITE: u16 = 1;
    const RENDERER_CPU: u16 = 2;
    const RENDERER_OPENCL_WITH_BUFFERS: u16 = 11;
    const RENDERER_OPENCL_WITH_IMAGES: u16 = 12;
    const RENDERER_METAL_WITH_BUFFERS: u16 = 21;

    pub fn store_no_info(&self) {
        self.0
            .store(Self::RENDERER_NO_INFO as u32, atomic::Ordering::Relaxed);
    }
    pub fn store_no_eligible(&self) {
        self.0
            .store(Self::RENDERER_NO_ELIGIBLE as u32, atomic::Ordering::Relaxed);
    }

    pub fn store_cpu_multi_thread_suite(&self, n_cpus: u16) {
        self.0.store(
            (Self::RENDERER_CPU_MULTI_THREAD_SUITE as u32) | ((n_cpus as u32) << 16),
            atomic::Ordering::Relaxed,
        );
    }
    pub fn store_opencl_buffer(&self) {
        self.0.store(
            Self::RENDERER_OPENCL_WITH_BUFFERS as u32,
            atomic::Ordering::Relaxed,
        );
    }
    pub fn store_opencl_image(&self) {
        self.0.store(
            Self::RENDERER_OPENCL_WITH_IMAGES as u32,
            atomic::Ordering::Relaxed,
        );
    }
    pub fn store_cpu(&self) {
        self.0
            .store(Self::RENDERER_CPU as u32, atomic::Ordering::Relaxed);
    }
    pub fn store_metal_buffer(&self) {
        self.0.store(
            Self::RENDERER_METAL_WITH_BUFFERS as u32,
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
            Self::RENDERER_OPENCL_WITH_BUFFERS => c"OpenCL (Buffers)".to_owned(),
            Self::RENDERER_OPENCL_WITH_IMAGES => c"OpenCL (Images)".to_owned(),
            Self::RENDERER_METAL_WITH_BUFFERS => c"Metal (Buffers)".to_owned(),
            Self::RENDERER_NO_ELIGIBLE => c"No Eligible".to_owned(),
            _ => c"?".to_owned(),
        }
    }
}

impl PartialEq for UsedRenderer {
    fn eq(&self, other: &Self) -> bool {
        self.0.load(atomic::Ordering::Relaxed) == other.0.load(atomic::Ordering::Relaxed)
    }
}

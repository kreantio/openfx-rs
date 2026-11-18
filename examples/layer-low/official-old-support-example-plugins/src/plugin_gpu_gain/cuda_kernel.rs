use std::{
    collections::{HashMap, hash_map},
    ffi::{c_int, c_uint, c_void},
    ptr::NonNull,
    sync::{LazyLock, Mutex},
};

use cudarc::{
    driver::sys::{CUcontext, CUmodule, CUstream},
    nvrtc::{CompileError, Ptx},
};
use openfx::low::StatusError;

const KERNEL_SOURCE: &str = include_str!("cuda_kernel.cu");
static KERNEL_PTX: LazyLock<Result<Ptx, CompileError>> =
    LazyLock::new(|| cudarc::nvrtc::compile_ptx(KERNEL_SOURCE));

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct CacheKey(CUcontext);
unsafe impl Send for CacheKey {}
#[derive(Clone, Copy)]
struct CacheValue(CUmodule);
unsafe impl Send for CacheValue {}

static CACHE_MUTEX: LazyLock<Mutex<HashMap<CacheKey, CacheValue>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

pub unsafe fn run_cuda_kernel(
    stream: Option<NonNull<c_void>>,
    width: c_int,
    height: c_int,
    gain: [f32; 4],
    input: NonNull<c_void>,
    output: NonNull<c_void>,
) -> Result<(), StatusError> {
    let stream = match stream {
        Some(s) => s.as_ptr() as CUstream,
        None => std::ptr::null_mut(),
    };

    let ctx = match cudarc::driver::result::ctx::get_current() {
        Ok(Some(ctx)) => ctx,
        Ok(None) => {
            tracing::error!("No current CUDA context is available.");
            return Err(StatusError::Failed);
        }
        Err(err) => {
            tracing::error!("Failed to get the current CUDA context: {err:?}");
            return Err(StatusError::Failed);
        }
    };

    let module = {
        let mut cache = match CACHE_MUTEX.lock() {
            Ok(map) => map,
            Err(err) => {
                tracing::error!("Failed to lock cache mutex: {err}");
                return Err(StatusError::Failed);
            }
        };

        let key = CacheKey(ctx);
        match cache.entry(key) {
            hash_map::Entry::Occupied(o) => *o.get(),
            hash_map::Entry::Vacant(v) => {
                let module = match &*KERNEL_PTX {
                    Ok(ptx) => {
                        let Some(ptx_bytes) = ptx.as_bytes() else {
                            tracing::error!("Failed to get PTX bytes.");
                            return Err(StatusError::Failed);
                        };
                        match cuda_utils::wrappers::module_load_data(
                            ptx_bytes.as_ptr() as *const c_void
                        ) {
                            Ok(module) => module,
                            Err(err) => {
                                tracing::error!("Failed to load CUDA module: {err:?}");
                                return Err(StatusError::Failed);
                            }
                        }
                    }
                    Err(err) => {
                        tracing::error!("Failed to compile CUDA kernel: {err:?}");
                        return Err(StatusError::Failed);
                    }
                };
                *v.insert(CacheValue(module))
            }
        }
        .0
    };

    let kernel = match cuda_utils::wrappers::module_get_function(module, c"GainAdjustKernel") {
        Ok(f) => f,
        Err(err) => {
            tracing::error!("Failed to get CUDA kernel function: {err:?}");
            return Err(StatusError::Failed);
        }
    };

    let kernel_params: [*mut c_void; 8] = [
        &width as *const _ as *mut c_void,
        &height as *const _ as *mut c_void,
        &gain[0] as *const _ as *mut c_void,
        &gain[1] as *const _ as *mut c_void,
        &gain[2] as *const _ as *mut c_void,
        &gain[3] as *const _ as *mut c_void,
        (&input.as_ptr()) as *const _ as *mut c_void,
        (&output.as_ptr()) as *const _ as *mut c_void,
    ];

    // ```cuda
    // dim3 threads(128, 1, 1);
    // dim3 blocks(((p_Width + threads.x - 1) / threads.x), p_Height, 1);
    // cudaStream_t stream = static_cast<cudaStream_t>(p_Stream);
    // GainAdjustKernel<<<blocks, threads, 0, stream>>>(p_Width, p_Height, p_Gain[0], p_Gain[1], p_Gain[2], p_Gain[3], p_Input, p_Output);
    // ```

    let threads: [c_uint; 3] = [128, 1, 1];
    let blocks: [c_uint; 3] = [(width as c_uint).div_ceil(threads[0]), height as c_uint, 1];

    match cuda_utils::wrappers::launch_kernel(
        kernel,
        blocks,
        threads,
        0,
        stream,
        &kernel_params[..],
        std::ptr::null_mut(),
    ) {
        Ok(_) => {}
        Err(err) => {
            tracing::error!("Failed to launch CUDA kernel: {err:?}");
            return Err(StatusError::Failed);
        }
    }

    Ok(())
}

mod cuda_utils {
    pub mod wrappers {
        use std::ffi::{CStr, c_uint, c_void};

        use cudarc::driver::{
            DriverError,
            sys::{
                CUfunction, CUmodule, CUresult, CUstream, cuLaunchKernel, cuModuleGetFunction,
                cuModuleLoadData,
            },
        };

        pub fn module_load_data(image: *const c_void) -> Result<CUmodule, DriverError> {
            let mut module = std::mem::MaybeUninit::<CUmodule>::uninit();
            let result = unsafe { cuModuleLoadData(module.as_mut_ptr(), image) };
            if result != CUresult::CUDA_SUCCESS {
                return Err(DriverError(result));
            }
            Ok(unsafe { module.assume_init() })
        }

        pub fn module_get_function(hmod: CUmodule, name: &CStr) -> Result<CUfunction, DriverError> {
            let mut function = std::mem::MaybeUninit::<CUfunction>::uninit();
            let result = unsafe { cuModuleGetFunction(function.as_mut_ptr(), hmod, name.as_ptr()) };
            if result != CUresult::CUDA_SUCCESS {
                return Err(DriverError(result));
            }
            Ok(unsafe { function.assume_init() })
        }

        #[allow(clippy::too_many_arguments)]
        pub fn launch_kernel(
            f: CUfunction,
            grid_dim: [c_uint; 3],
            block_dim: [c_uint; 3],
            shared_mem_bytes: c_uint,
            h_stream: CUstream,
            kernel_params: &[*mut c_void],
            extra: *mut *mut c_void,
        ) -> Result<(), DriverError> {
            let result = unsafe {
                cuLaunchKernel(
                    f,
                    grid_dim[0],
                    grid_dim[1],
                    grid_dim[2],
                    block_dim[0],
                    block_dim[1],
                    block_dim[2],
                    shared_mem_bytes,
                    h_stream,
                    (kernel_params).as_ptr() as *mut _,
                    extra,
                )
            };
            if result != CUresult::CUDA_SUCCESS {
                return Err(DriverError(result));
            }
            Ok(())
        }
    }
}

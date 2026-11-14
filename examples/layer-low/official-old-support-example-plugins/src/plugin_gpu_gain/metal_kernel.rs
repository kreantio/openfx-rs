use std::{
    collections::{HashMap, hash_map},
    ffi::c_int,
    os::raw::c_void,
    ptr::NonNull,
    sync::{LazyLock, Mutex},
};

use objc2::{rc::Retained, runtime::ProtocolObject};
use objc2_foundation::ns_string;
use objc2_metal::{
    MTLBuffer, MTLCommandBuffer, MTLCommandEncoder, MTLCommandQueue, MTLCompileOptions,
    MTLComputeCommandEncoder, MTLComputePipelineState, MTLDevice, MTLLibrary, MTLSize,
};
use openfx::low::StatusError;

const KERNEL_SOURCE: &str = include_str!("metal_kernel.metal");

type IdCommandQueue = ProtocolObject<dyn MTLCommandQueue>;
type IdComputePipelineState = ProtocolObject<dyn MTLComputePipelineState>;
type IdBuffer = ProtocolObject<dyn MTLBuffer>;

type PipelineQueueMap = HashMap<Retained<IdCommandQueue>, Retained<IdComputePipelineState>>;

static PIPELINE_QUEUE_MUTEX: LazyLock<Mutex<PipelineQueueMap>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// ## TODO
///
/// The types of `p_Input` and `p_Output` in the original code are wrong (they
/// should not be `float*`). Tell upstream?
pub unsafe fn run_metal_kernel(
    cmd_q: NonNull<c_void>,
    width: c_int,
    height: c_int,
    gain: [f32; 4],
    input: NonNull<c_void>,
    output: NonNull<c_void>,
) -> Result<(), StatusError> {
    const KERNEL_NAME: &str = "GainAdjustKernel";

    let queue: Retained<IdCommandQueue> =
        unsafe { Retained::retain(cmd_q.as_ptr().cast::<IdCommandQueue>()).unwrap_unchecked() };
    let device = queue.device();

    let mut pipeline_queue_map = match PIPELINE_QUEUE_MUTEX.lock() {
        Ok(map) => map,
        Err(err) => {
            tracing::error!("Failed to lock pipeline queue mutex, {err}");
            return Err(StatusError::Failed);
        }
    };

    let pipeline_state = match pipeline_queue_map.entry(queue.clone()) {
        hash_map::Entry::Vacant(v) => {
            let options = MTLCompileOptions::new();
            #[expect(deprecated)]
            options.setFastMathEnabled(true);
            let metal_library = match device
                .newLibraryWithSource_options_error(ns_string!(KERNEL_SOURCE), Some(&options))
            {
                Ok(lib) => lib,
                Err(err) => {
                    tracing::error!("Failed to load metal library, {err}");
                    return Err(StatusError::Failed);
                }
            };
            drop(options);
            let kernel_function = match metal_library.newFunctionWithName(ns_string!(KERNEL_NAME)) {
                Some(func) => func,
                None => {
                    tracing::error!("Failed to retrieve kernel");
                    return Err(StatusError::Failed);
                }
            };
            let pipeline_state =
                match device.newComputePipelineStateWithFunction_error(&kernel_function) {
                    Ok(state) => state,
                    Err(err) => {
                        tracing::error!("Unable to compile, {err}");
                        return Err(StatusError::Failed);
                    }
                };

            v.insert(pipeline_state.clone());
            pipeline_state
        }
        hash_map::Entry::Occupied(entry) => entry.get().clone(),
    };

    let src_device_buf =
        unsafe { Retained::retain(input.as_ptr().cast::<IdBuffer>()).unwrap_unchecked() };
    let dst_device_buf =
        unsafe { Retained::retain(output.as_ptr().cast::<IdBuffer>()).unwrap_unchecked() };

    let Some(command_buffer) = queue.commandBuffer() else {
        tracing::error!("Failed to create command buffer");
        return Err(StatusError::Failed);
    };
    command_buffer.setLabel(Some(ns_string!("GainAdjustKernel")));

    let Some(compute_encoder) = command_buffer.computeCommandEncoder() else {
        tracing::error!("Failed to create compute encoder");
        return Err(StatusError::Failed);
    };
    compute_encoder.setComputePipelineState(&pipeline_state);

    let exe_width = pipeline_state.threadExecutionWidth();
    let thread_group_count = MTLSize {
        width: exe_width,
        height: 1,
        depth: 1,
    };
    let thread_groups = MTLSize {
        width: (width as usize).div_ceil(exe_width),
        height: height as usize,
        depth: 1,
    };

    unsafe {
        macro_rules! ptr {
            (&$x:expr) => {
                NonNull::new(&$x as *const _ as *mut c_void).unwrap_unchecked()
            };
        }

        compute_encoder.setBuffer_offset_atIndex(Some(&src_device_buf), 0, 0);
        compute_encoder.setBuffer_offset_atIndex(Some(&dst_device_buf), 0, 8);
        compute_encoder.setBytes_length_atIndex(ptr!(&width), size_of::<c_int>(), 11);
        compute_encoder.setBytes_length_atIndex(ptr!(&height), size_of::<c_int>(), 12);
        compute_encoder.setBytes_length_atIndex(ptr!(&(gain[0])), size_of::<f32>(), 13);
        compute_encoder.setBytes_length_atIndex(ptr!(&(gain[1])), size_of::<f32>(), 14);
        compute_encoder.setBytes_length_atIndex(ptr!(&(gain[2])), size_of::<f32>(), 15);
        compute_encoder.setBytes_length_atIndex(ptr!(&(gain[3])), size_of::<f32>(), 16);
    }

    compute_encoder.dispatchThreadgroups_threadsPerThreadgroup(thread_groups, thread_group_count);

    compute_encoder.endEncoding();
    command_buffer.commit();

    Ok(())
}

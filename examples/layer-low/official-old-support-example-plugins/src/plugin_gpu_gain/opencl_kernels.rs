use std::{
    collections::{HashMap, hash_map},
    ffi::{CStr, c_int, c_void},
    ptr::NonNull,
    sync::{LazyLock, Mutex},
};

use cl3::{
    command_queue::{
        CL_QUEUE_CONTEXT, CL_QUEUE_DEVICE, enqueue_nd_range_kernel, get_command_queue_info,
    },
    kernel::{
        CL_KERNEL_WORK_GROUP_SIZE, create_kernel, get_kernel_work_group_info, set_kernel_arg,
    },
    program::{build_program, create_program_with_source},
    types::{cl_command_queue, cl_context, cl_device_id, cl_kernel, cl_mem, cl_uint},
};

use openfx::low::StatusError;

const KERNEL_SOURCE_BUFFERS: &str = include_str!("opencl_kernel_buffers.cl");
const KERNEL_SOURCE_IMAGES: &str = include_str!("opencl_kernel_images.cl");

#[derive(Default)]
struct DeviceAndKernelMap(HashMap<cl_command_queue, (cl_device_id, cl_kernel)>);

unsafe impl Send for DeviceAndKernelMap {}

static DEVICE_AND_KERNEL_CACHE_MUTEX: LazyLock<Mutex<DeviceAndKernelMap>> =
    LazyLock::new(|| Mutex::new(DeviceAndKernelMap::default()));

macro_rules! try_set_next_kernel_arg {
    ($kernel:expr, $count:ident, $ty:ty, $val:expr) => {{
        let _: $ty = $val;
        if let Err(err) = set_kernel_arg(
            $kernel,
            $count,
            std::mem::size_of::<$ty>(),
            &$val as *const $ty as *const c_void,
        ) {
            tracing::error!("Unable to set kernel argument [{err}]");
            return Err(StatusError::Failed);
        }
        $count += 1;
    }};
}

pub unsafe fn run_opencl_kernel_buffers(
    cmd_q: NonNull<c_void>,
    width: c_int,
    height: c_int,
    gain: [f32; 4],
    input: NonNull<c_void>,
    output: NonNull<c_void>,
) -> Result<(), StatusError> {
    let cmd_q = cmd_q.as_ptr();

    let (device_id, kernel) = {
        let mut cache = match DEVICE_AND_KERNEL_CACHE_MUTEX.lock() {
            Ok(map) => map,
            Err(err) => {
                tracing::error!("Failed to lock device and kernel mutex, {err}");
                return Err(StatusError::Failed);
            }
        };

        get_or_create_kernel(
            &mut cache,
            c"GainAdjustKernelBuffers",
            KERNEL_SOURCE_BUFFERS,
            cmd_q,
        )?
    };

    let mut count: cl_uint = 0;

    unsafe { try_set_next_kernel_arg!(kernel, count, c_int, width) };
    unsafe { try_set_next_kernel_arg!(kernel, count, c_int, height) };
    unsafe { try_set_next_kernel_arg!(kernel, count, f32, gain[0]) };
    unsafe { try_set_next_kernel_arg!(kernel, count, f32, gain[1]) };
    unsafe { try_set_next_kernel_arg!(kernel, count, f32, gain[2]) };
    unsafe { try_set_next_kernel_arg!(kernel, count, f32, gain[3]) };
    unsafe { try_set_next_kernel_arg!(kernel, count, cl_mem, input.as_ptr()) };
    unsafe { try_set_next_kernel_arg!(kernel, count, cl_mem, output.as_ptr()) };
    assert!(count > 0);

    let local_work_size = [
        match get_kernel_work_group_info(kernel, device_id, CL_KERNEL_WORK_GROUP_SIZE) {
            Ok(size) => size.to_size(),
            Err(err) => {
                tracing::error!(
                    "Failed to call `get_kernel_work_group_info` for `CL_KERNEL_WORK_GROUP_SIZE` [{err}]"
                );
                return Err(StatusError::Failed);
            }
        },
        1,
    ];
    let global_work_size = [
        (width as usize).div_ceil(local_work_size[0]) * local_work_size[0],
        height as usize,
    ];

    if let Err(err) = unsafe {
        enqueue_nd_range_kernel(
            cmd_q,
            kernel,
            2,
            std::ptr::null(),
            &global_work_size as *const _,
            &local_work_size as *const _,
            0,
            std::ptr::null_mut(),
        )
    } {
        tracing::error!("Failed to call `enqueue_nd_range_kernel` [{err}]");
        return Err(StatusError::Failed);
    }

    Ok(())
}

static DEVICE_AND_KERNEL_MUTEX_FOR_IMAGES: LazyLock<Mutex<DeviceAndKernelMap>> =
    LazyLock::new(|| Mutex::new(DeviceAndKernelMap::default()));

pub unsafe fn run_opencl_kernel_images(
    cmd_q: NonNull<c_void>,
    width: c_int,
    height: c_int,
    gain: [f32; 4],
    input: NonNull<c_void>,
    output: NonNull<c_void>,
) -> Result<(), StatusError> {
    let cmd_q = cmd_q.as_ptr();

    let (device_id, kernel) = {
        let mut map = match DEVICE_AND_KERNEL_MUTEX_FOR_IMAGES.lock() {
            Ok(map) => map,
            Err(err) => {
                tracing::error!("Failed to lock device and kernel mutex, {err}");
                return Err(StatusError::Failed);
            }
        };

        get_or_create_kernel(
            &mut map,
            c"GainAdjustKernelImages",
            KERNEL_SOURCE_IMAGES,
            cmd_q,
        )?
    };

    let mut count: cl_uint = 0;

    unsafe { try_set_next_kernel_arg!(kernel, count, c_int, width) };
    unsafe { try_set_next_kernel_arg!(kernel, count, c_int, height) };
    unsafe { try_set_next_kernel_arg!(kernel, count, f32, gain[0]) };
    unsafe { try_set_next_kernel_arg!(kernel, count, f32, gain[1]) };
    unsafe { try_set_next_kernel_arg!(kernel, count, f32, gain[2]) };
    unsafe { try_set_next_kernel_arg!(kernel, count, f32, gain[3]) };
    unsafe { try_set_next_kernel_arg!(kernel, count, cl_mem, input.as_ptr()) };
    unsafe { try_set_next_kernel_arg!(kernel, count, cl_mem, output.as_ptr()) };
    assert!(count > 0);

    let local_work_size = [
        match get_kernel_work_group_info(kernel, device_id, CL_KERNEL_WORK_GROUP_SIZE) {
            Ok(size) => size.to_size(),
            Err(err) => {
                tracing::error!(
                    "Failed to call `get_kernel_work_group_info` for `CL_KERNEL_WORK_GROUP_SIZE` [{err}]"
                );
                return Err(StatusError::Failed);
            }
        },
        1,
    ];
    let global_work_size = [
        (width as usize).div_ceil(local_work_size[0]) * local_work_size[0],
        height as usize,
    ];

    if let Err(err) = unsafe {
        enqueue_nd_range_kernel(
            cmd_q,
            kernel,
            2,
            std::ptr::null(),
            &global_work_size as *const _,
            &local_work_size as *const _,
            0,
            std::ptr::null_mut(),
        )
    } {
        tracing::error!("Failed to call `enqueue_nd_range_kernel` [{err}]");
        return Err(StatusError::Failed);
    }

    Ok(())
}

/// Find the device id and the program kernel corresponding to the command
/// queue.
fn get_or_create_kernel(
    map: &mut DeviceAndKernelMap,
    kernel_name: &CStr,
    source: &str,
    cmd_q: cl_command_queue,
) -> Result<(cl_device_id, cl_kernel), StatusError> {
    match map.0.entry(cmd_q) {
        hash_map::Entry::Vacant(v) => {
            let device_id = match get_command_queue_info(cmd_q, CL_QUEUE_DEVICE) {
                Ok(device_id) => device_id.to_ptr() as cl_device_id,
                Err(err) => {
                    tracing::error!("Unable to get the device [{err}]");
                    return Err(StatusError::Failed);
                }
            };

            let context = match get_command_queue_info(cmd_q, CL_QUEUE_CONTEXT) {
                Ok(context) => context.to_ptr() as cl_context,
                Err(err) => {
                    tracing::error!("Unable to get the context [{err}]");
                    return Err(StatusError::Failed);
                }
            };
            let program = match create_program_with_source(context, &[source]) {
                Ok(program) => program,
                Err(err) => {
                    tracing::error!("Unable to create program [{err}]");
                    return Err(StatusError::Failed);
                }
            };
            if let Err(err) = build_program(program, &[], c"", None, std::ptr::null_mut()) {
                tracing::error!("Unable to build program [{err}]");
                return Err(StatusError::Failed);
            }
            let kernel = match create_kernel(program, kernel_name) {
                Ok(kernel) => kernel,
                Err(err) => {
                    tracing::error!("Unable to create kernel [{err}]");
                    return Err(StatusError::Failed);
                }
            };

            Ok(*v.insert((device_id, kernel)))
        }
        hash_map::Entry::Occupied(entry) => Ok(*entry.get()),
    }
}

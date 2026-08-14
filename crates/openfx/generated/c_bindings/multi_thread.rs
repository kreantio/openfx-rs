use super::core::{
    OfxHost, OfxPlugin, OfxPluginEntryPoint, OfxPointD, OfxPointI, OfxPropertySetHandle,
    OfxPropertySetStruct, OfxRangeD, OfxRangeI, OfxRectD, OfxRectI, OfxStatus, OfxTime,
    kOfxActionBeginInstanceChanged, kOfxActionBeginInstanceEdit,
    kOfxActionCreateInstance, kOfxActionDescribe, kOfxActionDestroyInstance,
    kOfxActionEndInstanceChanged, kOfxActionEndInstanceEdit, kOfxActionInstanceChanged,
    kOfxActionLoad, kOfxActionPurgeCaches, kOfxActionSyncPrivateData, kOfxActionUnload,
    kOfxBitDepthByte, kOfxBitDepthFloat, kOfxBitDepthHalf, kOfxBitDepthNone,
    kOfxBitDepthShort, kOfxChangePluginEdited, kOfxChangeTime, kOfxChangeUserEdited,
    kOfxFlagInfiniteMax, kOfxFlagInfiniteMin, kOfxPluginPropFilePath, kOfxPropAPIVersion,
    kOfxPropChangeReason, kOfxPropEffectInstance, kOfxPropHostOSHandle, kOfxPropIcon,
    kOfxPropInstanceData, kOfxPropIsInteractive, kOfxPropLabel, kOfxPropLongLabel,
    kOfxPropName, kOfxPropPluginDescription, kOfxPropShortLabel, kOfxPropTime,
    kOfxPropType, kOfxPropVersion, kOfxPropVersionLabel,
};
pub const kOfxMultiThreadSuite: &::std::ffi::CStr = c"OfxMultiThreadSuite";
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of OfxHost"][::std::mem::size_of::<OfxHost>() - 16usize];
    ["Alignment of OfxHost"][::std::mem::align_of::<OfxHost>() - 8usize];
    ["Offset of field: OfxHost::host"][::std::mem::offset_of!(OfxHost, host) - 0usize];
    [
        "Offset of field: OfxHost::fetchSuite",
    ][::std::mem::offset_of!(OfxHost, fetchSuite) - 8usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of OfxPlugin"][::std::mem::size_of::<OfxPlugin>() - 48usize];
    ["Alignment of OfxPlugin"][::std::mem::align_of::<OfxPlugin>() - 8usize];
    [
        "Offset of field: OfxPlugin::pluginApi",
    ][::std::mem::offset_of!(OfxPlugin, pluginApi) - 0usize];
    [
        "Offset of field: OfxPlugin::apiVersion",
    ][::std::mem::offset_of!(OfxPlugin, apiVersion) - 8usize];
    [
        "Offset of field: OfxPlugin::pluginIdentifier",
    ][::std::mem::offset_of!(OfxPlugin, pluginIdentifier) - 16usize];
    [
        "Offset of field: OfxPlugin::pluginVersionMajor",
    ][::std::mem::offset_of!(OfxPlugin, pluginVersionMajor) - 24usize];
    [
        "Offset of field: OfxPlugin::pluginVersionMinor",
    ][::std::mem::offset_of!(OfxPlugin, pluginVersionMinor) - 28usize];
    [
        "Offset of field: OfxPlugin::setHost",
    ][::std::mem::offset_of!(OfxPlugin, setHost) - 32usize];
    [
        "Offset of field: OfxPlugin::mainEntry",
    ][::std::mem::offset_of!(OfxPlugin, mainEntry) - 40usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of OfxRangeI"][::std::mem::size_of::<OfxRangeI>() - 8usize];
    ["Alignment of OfxRangeI"][::std::mem::align_of::<OfxRangeI>() - 4usize];
    ["Offset of field: OfxRangeI::min"][::std::mem::offset_of!(OfxRangeI, min) - 0usize];
    ["Offset of field: OfxRangeI::max"][::std::mem::offset_of!(OfxRangeI, max) - 4usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of OfxRangeD"][::std::mem::size_of::<OfxRangeD>() - 16usize];
    ["Alignment of OfxRangeD"][::std::mem::align_of::<OfxRangeD>() - 8usize];
    ["Offset of field: OfxRangeD::min"][::std::mem::offset_of!(OfxRangeD, min) - 0usize];
    ["Offset of field: OfxRangeD::max"][::std::mem::offset_of!(OfxRangeD, max) - 8usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of OfxPointI"][::std::mem::size_of::<OfxPointI>() - 8usize];
    ["Alignment of OfxPointI"][::std::mem::align_of::<OfxPointI>() - 4usize];
    ["Offset of field: OfxPointI::x"][::std::mem::offset_of!(OfxPointI, x) - 0usize];
    ["Offset of field: OfxPointI::y"][::std::mem::offset_of!(OfxPointI, y) - 4usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of OfxPointD"][::std::mem::size_of::<OfxPointD>() - 16usize];
    ["Alignment of OfxPointD"][::std::mem::align_of::<OfxPointD>() - 8usize];
    ["Offset of field: OfxPointD::x"][::std::mem::offset_of!(OfxPointD, x) - 0usize];
    ["Offset of field: OfxPointD::y"][::std::mem::offset_of!(OfxPointD, y) - 8usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of OfxRectI"][::std::mem::size_of::<OfxRectI>() - 16usize];
    ["Alignment of OfxRectI"][::std::mem::align_of::<OfxRectI>() - 4usize];
    ["Offset of field: OfxRectI::x1"][::std::mem::offset_of!(OfxRectI, x1) - 0usize];
    ["Offset of field: OfxRectI::y1"][::std::mem::offset_of!(OfxRectI, y1) - 4usize];
    ["Offset of field: OfxRectI::x2"][::std::mem::offset_of!(OfxRectI, x2) - 8usize];
    ["Offset of field: OfxRectI::y2"][::std::mem::offset_of!(OfxRectI, y2) - 12usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of OfxRectD"][::std::mem::size_of::<OfxRectD>() - 32usize];
    ["Alignment of OfxRectD"][::std::mem::align_of::<OfxRectD>() - 8usize];
    ["Offset of field: OfxRectD::x1"][::std::mem::offset_of!(OfxRectD, x1) - 0usize];
    ["Offset of field: OfxRectD::y1"][::std::mem::offset_of!(OfxRectD, y1) - 8usize];
    ["Offset of field: OfxRectD::x2"][::std::mem::offset_of!(OfxRectD, x2) - 16usize];
    ["Offset of field: OfxRectD::y2"][::std::mem::offset_of!(OfxRectD, y2) - 24usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OfxMutex {
    _unused: [u8; 0],
}
/// @brief Mutex blind data handle
pub type OfxMutexHandle = *mut OfxMutex;
/** @brief The function type to passed to the multi threading routines

\arg \c threadIndex unique index of this thread, will be between 0 and threadMax
\arg \c threadMax to total number of threads executing this function
\arg \c customArg the argument passed into multiThread

A function of this type is passed to OfxMultiThreadSuiteV1::multiThread to be launched in multiple threads.*/
pub type OfxThreadFunctionV1 = ::std::option::Option<
    unsafe extern "C" fn(
        threadIndex: ::std::os::raw::c_uint,
        threadMax: ::std::os::raw::c_uint,
        customArg: *mut ::std::os::raw::c_void,
    ),
>;
/// @brief OFX suite that provides simple SMP style multi-processing
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OfxMultiThreadSuiteV1 {
    /**@brief Function to spawn SMP threads

\arg \c func function to call in each thread.
\arg \c nThreads number of threads to launch
\arg \c customArg parameter to pass to customArg of func in each thread.

This function will spawn nThreads separate threads of computation (typically one per CPU)
to allow something to perform symmetric multi processing. Each thread will call 'func' passing
in the index of the thread and the number of threads actually launched.

multiThread will not return until all the spawned threads have returned. It is up to the host
how it waits for all the threads to return (busy wait, blocking, whatever).

\e nThreads can be more than the value returned by multiThreadNumCPUs, however the threads will
be limited to the number of CPUs returned by multiThreadNumCPUs.

This function cannot be called recursively.

@returns
- ::kOfxStatOK, the function func has executed and returned successfully
- ::kOfxStatFailed, the threading function failed to launch
- ::kOfxStatErrExists, failed in an attempt to call multiThread recursively,
*/
    pub multiThread: ::std::option::Option<
        unsafe extern "C" fn(
            func: OfxThreadFunctionV1,
            nThreads: ::std::os::raw::c_uint,
            customArg: *mut ::std::os::raw::c_void,
        ) -> OfxStatus,
    >,
    /**@brief Function which indicates the number of CPUs available for SMP processing

\arg \c nCPUs pointer to an integer where the result is returned

This value may be less than the actual number of CPUs on a machine, as the host may reserve other CPUs for itself.

@returns
- ::kOfxStatOK, all was OK and the maximum number of threads is in nThreads.
- ::kOfxStatFailed, the function failed to get the number of CPUs*/
    pub multiThreadNumCPUs: ::std::option::Option<
        unsafe extern "C" fn(nCPUs: *mut ::std::os::raw::c_uint) -> OfxStatus,
    >,
    /**@brief Function which indicates the index of the current thread

\arg \c threadIndex  pointer to an integer where the result is returned

This function returns the thread index, which is the same as the \e threadIndex argument passed to the ::OfxThreadFunctionV1.

If there are no threads currently spawned, then this function will set threadIndex to 0

@returns
- ::kOfxStatOK, all was OK and the maximum number of threads is in nThreads.
- ::kOfxStatFailed, the function failed to return an index*/
    pub multiThreadIndex: ::std::option::Option<
        unsafe extern "C" fn(threadIndex: *mut ::std::os::raw::c_uint) -> OfxStatus,
    >,
    /**@brief Function to enquire if the calling thread was spawned by multiThread

@returns
- 0 if the thread is not one spawned by multiThread
- 1 if the thread was spawned by multiThread*/
    pub multiThreadIsSpawnedThread: ::std::option::Option<
        unsafe extern "C" fn() -> ::std::os::raw::c_int,
    >,
    /** @brief Create a mutex

\arg \c mutex where the new handle is returned
\arg \c count initial lock count on the mutex. This can be negative.

Creates a new mutex with lockCount locks on the mutex initially set.

@returns
- kOfxStatOK - mutex is now valid and ready to go*/
    pub mutexCreate: ::std::option::Option<
        unsafe extern "C" fn(
            mutex: *mut OfxMutexHandle,
            lockCount: ::std::os::raw::c_int,
        ) -> OfxStatus,
    >,
    /** @brief Destroy a mutex

Destroys a mutex initially created by mutexCreate.

@returns
- kOfxStatOK - if it destroyed the mutex
- kOfxStatErrBadHandle - if the handle was bad*/
    pub mutexDestroy: ::std::option::Option<
        unsafe extern "C" fn(mutex: OfxMutexHandle) -> OfxStatus,
    >,
    /** @brief Blocking lock on the mutex

This tries to lock a mutex and blocks the thread it is in until the lock succeeds.

A successful lock causes the mutex's lock count to be increased by one and to block any other calls to lock the mutex until it is unlocked.

@returns
- kOfxStatOK - if it got the lock
- kOfxStatErrBadHandle - if the handle was bad*/
    pub mutexLock: ::std::option::Option<
        unsafe extern "C" fn(mutex: OfxMutexHandle) -> OfxStatus,
    >,
    /** @brief Unlock the mutex

This  unlocks a mutex. Unlocking a mutex decreases its lock count by one.

@returns
- kOfxStatOK if it released the lock
- kOfxStatErrBadHandle if the handle was bad*/
    pub mutexUnLock: ::std::option::Option<
        unsafe extern "C" fn(mutex: OfxMutexHandle) -> OfxStatus,
    >,
    /** @brief Non blocking attempt to lock the mutex

This attempts to lock a mutex, if it cannot, it returns and says so, rather than blocking.

A successful lock causes the mutex's lock count to be increased by one, if the lock did not succeed, the call returns immediately and the lock count remains unchanged.

@returns
- kOfxStatOK - if it got the lock
- kOfxStatFailed - if it did not get the lock
- kOfxStatErrBadHandle - if the handle was bad*/
    pub mutexTryLock: ::std::option::Option<
        unsafe extern "C" fn(mutex: OfxMutexHandle) -> OfxStatus,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of OfxMultiThreadSuiteV1",
    ][::std::mem::size_of::<OfxMultiThreadSuiteV1>() - 72usize];
    [
        "Alignment of OfxMultiThreadSuiteV1",
    ][::std::mem::align_of::<OfxMultiThreadSuiteV1>() - 8usize];
    [
        "Offset of field: OfxMultiThreadSuiteV1::multiThread",
    ][::std::mem::offset_of!(OfxMultiThreadSuiteV1, multiThread) - 0usize];
    [
        "Offset of field: OfxMultiThreadSuiteV1::multiThreadNumCPUs",
    ][::std::mem::offset_of!(OfxMultiThreadSuiteV1, multiThreadNumCPUs) - 8usize];
    [
        "Offset of field: OfxMultiThreadSuiteV1::multiThreadIndex",
    ][::std::mem::offset_of!(OfxMultiThreadSuiteV1, multiThreadIndex) - 16usize];
    [
        "Offset of field: OfxMultiThreadSuiteV1::multiThreadIsSpawnedThread",
    ][::std::mem::offset_of!(OfxMultiThreadSuiteV1, multiThreadIsSpawnedThread)
        - 24usize];
    [
        "Offset of field: OfxMultiThreadSuiteV1::mutexCreate",
    ][::std::mem::offset_of!(OfxMultiThreadSuiteV1, mutexCreate) - 32usize];
    [
        "Offset of field: OfxMultiThreadSuiteV1::mutexDestroy",
    ][::std::mem::offset_of!(OfxMultiThreadSuiteV1, mutexDestroy) - 40usize];
    [
        "Offset of field: OfxMultiThreadSuiteV1::mutexLock",
    ][::std::mem::offset_of!(OfxMultiThreadSuiteV1, mutexLock) - 48usize];
    [
        "Offset of field: OfxMultiThreadSuiteV1::mutexUnLock",
    ][::std::mem::offset_of!(OfxMultiThreadSuiteV1, mutexUnLock) - 56usize];
    [
        "Offset of field: OfxMultiThreadSuiteV1::mutexTryLock",
    ][::std::mem::offset_of!(OfxMultiThreadSuiteV1, mutexTryLock) - 64usize];
};

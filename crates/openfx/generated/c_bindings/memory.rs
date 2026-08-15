use super::core::OfxStatus;
pub const kOfxMemorySuite: &::std::ffi::CStr = c"OfxMemorySuite";
/** @brief The OFX suite that implements general purpose memory management.

Use this suite for ordinary memory management functions, where you would normally use malloc/free or new/delete on ordinary objects.

For images, you should use the memory allocation functions in the image effect suite, as many hosts have specific image memory pools.

\note C++ plugin developers will need to redefine new and delete as skins on top of this suite.*/
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OfxMemorySuiteV1 {
    /** @brief Allocate memory.

\arg \c handle	- effect instance to associate with this memory allocation, or NULL.
\arg \c nBytes        number of bytes to allocate
\arg \c allocatedData pointer to the return value. Allocated memory will be aligned for any use.

This function has the host allocate memory using its own memory resources
and returns that to the plugin.

@returns
- ::kOfxStatOK the memory was successfully allocated
- ::kOfxStatErrMemory the request could not be met and no memory was allocated
*/
    pub memoryAlloc: ::std::option::Option<
        unsafe extern "C" fn(
            handle: *mut ::std::os::raw::c_void,
            nBytes: usize,
            allocatedData: *mut *mut ::std::os::raw::c_void,
        ) -> OfxStatus,
    >,
    /** @brief Frees memory.

\arg \c allocatedData pointer to memory previously returned by OfxMemorySuiteV1::memoryAlloc

This function frees any memory that was previously allocated via OfxMemorySuiteV1::memoryAlloc.

@returns
- ::kOfxStatOK the memory was successfully freed
- ::kOfxStatErrBadHandle \e allocatedData was not a valid pointer returned by OfxMemorySuiteV1::memoryAlloc
*/
    pub memoryFree: ::std::option::Option<
        unsafe extern "C" fn(allocatedData: *mut ::std::os::raw::c_void) -> OfxStatus,
    >,
}

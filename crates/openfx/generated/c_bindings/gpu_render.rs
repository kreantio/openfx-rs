// Copyright OpenFX and contributors to the OpenFX project.
// SPDX-License-Identifier: BSD-3-Clause
use super::core::{OfxPropertySetHandle, OfxRectD, OfxStatus, OfxTime};
use super::image_effect::OfxImageClipHandle;
/// ```doxygen
/// @brief The name of the OpenGL render suite, used to fetch from a host
/// via OfxHost::fetchSuite
/// ```
pub const kOfxOpenGLRenderSuite: &::std::ffi::CStr = c"OfxImageEffectOpenGLRenderSuite";
/// ```doxygen
/// @brief Indicates whether a host or plug-in can support OpenGL accelerated
/// rendering
///
///    - Valid Values -
///      - "false"  - in which case the host or plug-in does not support OpenGL
///                   accelerated rendering
///      - "true"   - which means a host or plug-in can support OpenGL accelerated
///                   rendering, in the case of plug-ins this also means that it
///                   is capable of CPU based rendering in the absence of a GPU
///      - "needed" - only for plug-ins, this means that an plug-in has to have
///                   OpenGL support, without which it cannot work.
///
/// V1.4: It is now expected from host reporting v1.4 that the plug-in can during instance change switch from true to false and false to true.
///
///     @propdef
///     type: enum
///     dimension: 1
///     values:
///       - "false"
///       - "true"
///       - needed
/// ```
pub const kOfxImageEffectPropOpenGLRenderSupported: &::std::ffi::CStr = c"OfxImageEffectPropOpenGLRenderSupported";
/// ```doxygen
/// @brief Indicates the bit depths supported by a plug-in during OpenGL renders.
///
///     This is analogous to ::kOfxImageEffectPropSupportedPixelDepths. When a
///     plug-in sets this property, the host will try to provide buffers/textures
///     in one of the supported formats. Additionally, the target buffers where
///     the plug-in renders to will be set to one of the supported formats.
///
///     Unlike ::kOfxImageEffectPropSupportedPixelDepths, this property is
///     optional. Shader-based effects might not really care about any
///     format specifics when using OpenGL textures, so they can leave this unset
///     and allow the host the decide the format.
///
///    - Valid Values -
///        - ::kOfxBitDepthNone (implying a clip is unconnected, not valid for an
///          image)
///        - ::kOfxBitDepthByte
///        - ::kOfxBitDepthShort
///        - ::kOfxBitDepthHalf
///        - ::kOfxBitDepthFloat
///
///     @propdef
///     type: enum
///     dimension: N
///     values:
///       - OfxBitDepthNone
///       - OfxBitDepthByte
///       - OfxBitDepthShort
///       - OfxBitDepthHalf
///       - OfxBitDepthFloat
/// ```
pub const kOfxOpenGLPropPixelDepth: &::std::ffi::CStr = c"OfxOpenGLPropPixelDepth";
/// ```doxygen
/// @brief Indicates that a plug-in SHOULD use OpenGL acceleration in
/// the current action
///
///    When a plug-in and host have established they can both use OpenGL renders
///    then when this property has been set the host expects the plug-in to render
///    its result into the buffer it has setup before calling the render.  The
///    plug-in can then also safely use the 'OfxImageEffectOpenGLRenderSuite'
///
///    - Valid Values
///       - 0 indicates that the plug-in cannot use the OpenGL suite
///       - 1 indicates that the plug-in should render into the texture,
///           and may use the OpenGL suite functions.
///
/// \note Once this property is set, the host and plug-in have agreed to
/// use OpenGL, so the effect SHOULD access all its images through the
/// OpenGL suite.
///
/// v1.4:  kOfxImageEffectPropOpenGLEnabled should probably be checked in Instance Changed prior to try to read image via clipLoadTexture
///
///     @propdef
///     type: bool
///     dimension: 1
/// ```
pub const kOfxImageEffectPropOpenGLEnabled: &::std::ffi::CStr = c"OfxImageEffectPropOpenGLEnabled";
/// ```doxygen
/// @brief Indicates the texture index of an image turned into an OpenGL
/// texture by the host
///
/// This value should be cast to a GLuint and used as the texture index when
/// performing OpenGL texture operations.
///
///    The property set of the following actions should contain this property:
///       - ::kOfxImageEffectActionRender
///       - ::kOfxImageEffectActionBeginSequenceRender
///       - ::kOfxImageEffectActionEndSequenceRender
///
///     @propdef
///     type: int
///     dimension: 1
/// ```
pub const kOfxImageEffectPropOpenGLTextureIndex: &::std::ffi::CStr = c"OfxImageEffectPropOpenGLTextureIndex";
/// ```doxygen
/// @brief Indicates the texture target enumerator of an image turned into
///     an OpenGL texture by the host
///
/// This value should be cast to a GLenum and used as the texture target
/// when performing OpenGL texture operations.
///
///    The property set of the following actions should contain this property:
///       - ::kOfxImageEffectActionRender
///       - ::kOfxImageEffectActionBeginSequenceRender
///       - ::kOfxImageEffectActionEndSequenceRender
///
///     @propdef
///     type: int
///     dimension: 1
/// ```
pub const kOfxImageEffectPropOpenGLTextureTarget: &::std::ffi::CStr = c"OfxImageEffectPropOpenGLTextureTarget";
/// ```doxygen
/// @brief Indicates whether a host or plug-in can (or more importantly cannot)
///     support CPU rendering.
///
///   @version added in version 1.5.1.
///
///    - Valid Values -
///      - "false"  - in which case the host or plug-in does not support CPU
///                   rendering
///      - "true"   - which means a host or plug-in can support CPU rendering
///
///     @propdef
///     type: enum
///     dimension: 1
///     values:
///       - "false"
///       - "true"
///     introduced: "1.5.1"
/// ```
pub const kOfxImageEffectPropCPURenderSupported: &::std::ffi::CStr = c"OfxImageEffectPropCPURenderSupported";
/// ```doxygen
/// @brief Action called when an effect has just been attached to an OpenGL
/// context.
///
/// The purpose of this action is to allow a plug-in to set up any data it may need
/// to do OpenGL rendering in an instance. For example...
///    - allocate a lookup table on a GPU,
///    - create an OpenCL or CUDA context that is bound to the host's OpenGL
///      context so it can share buffers.
///
/// The plug-in will be responsible for deallocating any such shared resource in the
/// \ref ::kOfxActionOpenGLContextDetached action.
///
/// A host cannot call ::kOfxActionOpenGLContextAttached on the same instance
/// without an intervening ::kOfxActionOpenGLContextDetached. A host can have a
/// plug-in swap OpenGL contexts by issuing a attach/detach for the first context
/// then another attach for the next context.
///
/// The arguments to the action are...
///   \arg \c handle handle to the plug-in instance, cast to an
///   \ref OfxImageEffectHandle
///   \arg \c inArgs is redundant and set to NULL
///   \arg \c outArgs is redundant and set to NULL
///
/// A plug-in can return...
///   - ::kOfxStatOK, the action was trapped and all was well
///   - ::kOfxStatReplyDefault, the action was ignored, but all was well anyway
///   - ::kOfxStatErrMemory, in which case this may be called again after a memory
///     purge
///   - ::kOfxStatFailed, something went wrong, but no error code appropriate,
///     the plug-in should to post a message if possible and the host should not
///     attempt to run the plug-in in OpenGL render mode.
/// ```
pub const kOfxActionOpenGLContextAttached: &::std::ffi::CStr = c"OfxActionOpenGLContextAttached";
/// ```doxygen
/// @brief Action called when an effect is about to be detached from an
/// OpenGL context
///
/// The purpose of this action is to allow a plug-in to deallocate any resource
/// allocated in \ref ::kOfxActionOpenGLContextAttached just before the host
/// decouples a plug-in from an OpenGL context.
/// The host must call this with the same OpenGL context active as it
/// called with the corresponding ::kOfxActionOpenGLContextAttached.
///
/// The arguments to the action are...
///   \arg \c handle handle to the plug-in instance, cast to an
///   \ref OfxImageEffectHandle
///   \arg \c inArgs is redundant and set to NULL
///   \arg \c outArgs is redundant and set to NULL
///
/// A plug-in can return...
///   - ::kOfxStatOK, the action was trapped and all was well
///   - ::kOfxStatReplyDefault, the action was ignored, but all was well anyway
///   - ::kOfxStatErrMemory, in which case this may be called again after a memory
///     purge
///   - ::kOfxStatFailed, something went wrong, but no error code appropriate,
///     the plug-in should to post a message if possible and the host should not
///     attempt to run the plug-in in OpenGL render mode.
/// ```
pub const kOfxActionOpenGLContextDetached: &::std::ffi::CStr = c"kOfxActionOpenGLContextDetached";
/// ```doxygen
/// @brief Indicates whether a host or plug-in can support CUDA render
///
///     - Valid Values -
///       - "false"  - the host or plug-in does not support CUDA render
///       - "true"   - the host or plug-in can support CUDA render
///
///     @propdef
///     type: enum
///     dimension: 1
///     values:
///       - "false"
///       - "true"
///       - needed
/// ```
pub const kOfxImageEffectPropCudaRenderSupported: &::std::ffi::CStr = c"OfxImageEffectPropCudaRenderSupported";
/// ```doxygen
/// @brief Indicates that a plug-in SHOULD use CUDA render in
/// the current action
///
///    If a plug-in and host have both set
///    kOfxImageEffectPropCudaRenderSupported="true" then the host MAY set
///    this property to indicate that it is passing images as CUDA memory
///    pointers.
///
///    - Valid Values
///       - 0 indicates that the kOfxImagePropData of each image of each clip
///           is a CPU memory pointer.
///       - 1 indicates that the kOfxImagePropData of each image of each clip
/// 	      is a CUDA memory pointer.
///
///     @propdef
///     type: bool
///     dimension: 1
/// ```
pub const kOfxImageEffectPropCudaEnabled: &::std::ffi::CStr = c"OfxImageEffectPropCudaEnabled";
/// ```doxygen
/// @brief Indicates whether a host or plug-in can support CUDA streams
///
///     - Valid Values -
///       - "false"  - in which case the host or plug-in does not support CUDA streams
///       - "true"   - which means a host or plug-in can support CUDA streams
///
///     @propdef
///     type: enum
///     dimension: 1
///     values:
///       - "false"
///       - "true"
///       - needed
/// ```
pub const kOfxImageEffectPropCudaStreamSupported: &::std::ffi::CStr = c"OfxImageEffectPropCudaStreamSupported";
/// ```doxygen
/// @brief The CUDA stream to be used for rendering
///
/// This property will only be set if the host and plug-in both support CUDA streams.
///
/// If set:
///
/// - this property contains a pointer to the stream of CUDA render (cudaStream_t).
///   In order to use it, reinterpret_cast<cudaStream_t>(pointer) is needed.
///
/// - the plug-in SHOULD ensure that its render action enqueues any
///   asynchronous CUDA operations onto the supplied queue.
///
/// - the plug-in SHOULD NOT wait for final asynchronous operations to
///   complete before returning from the render action, and SHOULD NOT
///   call cudaDeviceSynchronize() at any time.
///
/// If not set:
///
/// - the plug-in SHOULD ensure that any asynchronous operations it
///   enqueues have completed before returning from the render action.
///
///     @propdef
///     type: pointer
///     dimension: 1
/// ```
pub const kOfxImageEffectPropCudaStream: &::std::ffi::CStr = c"OfxImageEffectPropCudaStream";
/// ```doxygen
/// @brief Indicates whether a host or plug-in can support Metal render
///
///     - Valid Values -
///       - "false"  - the host or plug-in does not support Metal render
///       - "true"   - the host or plug-in can support Metal render
///
///     @propdef
///     type: enum
///     dimension: 1
///     values:
///       - "false"
///       - "true"
///       - needed
///     introduced: "1.5"
/// ```
pub const kOfxImageEffectPropMetalRenderSupported: &::std::ffi::CStr = c"OfxImageEffectPropMetalRenderSupported";
/// ```doxygen
/// @brief Indicates that a plug-in SHOULD use Metal render in
/// the current action
///
///    If a plug-in and host have both set
///    kOfxImageEffectPropMetalRenderSupported="true" then the host MAY
///    set this property to indicate that it is passing images as Metal
///    buffers.
///
///    - Valid Values
///       - 0 indicates that the kOfxImagePropData of each image of each clip
///           is a CPU memory pointer.
///       - 1 indicates that the kOfxImagePropData of each image of each clip
/// 	      is a Metal id<MTLBuffer>.
///
///     @propdef
///     type: bool
///     dimension: 1
///     introduced: "1.5"
/// ```
pub const kOfxImageEffectPropMetalEnabled: &::std::ffi::CStr = c"OfxImageEffectPropMetalEnabled";
/// ```doxygen
/// @brief The command queue of Metal render
///
/// This property contains a pointer to the command queue to be used for
/// Metal rendering (id<MTLCommandQueue>). In order to use it,
/// reinterpret_cast<id<MTLCommandQueue>>(pointer) is needed.
///
/// The plug-in SHOULD ensure that its render action enqueues any
/// asynchronous Metal operations onto the supplied queue.
///
/// The plug-in SHOULD NOT wait for final asynchronous operations to
/// complete before returning from the render action.
///
///     @propdef
///     type: pointer
///     dimension: 1
///     introduced: "1.5"
/// ```
pub const kOfxImageEffectPropMetalCommandQueue: &::std::ffi::CStr = c"OfxImageEffectPropMetalCommandQueue";
/// ```doxygen
/// @brief Indicates whether a host or plug-in can support OpenCL Buffers render
///
///     - Valid Values -
///       - "false"  - the host or plug-in does not support OpenCL Buffers render
///       - "true"   - the host or plug-in can support OpenCL Buffers render
///
///     @propdef
///     type: enum
///     dimension: 1
///     values:
///       - "false"
///       - "true"
///       - needed
///     introduced: "1.5"
/// ```
pub const kOfxImageEffectPropOpenCLRenderSupported: &::std::ffi::CStr = c"OfxImageEffectPropOpenCLRenderSupported";
/// ```doxygen
/// @brief Indicates whether a host or plug-in can support OpenCL Images render
///
///     - Valid Values -
///       - "false"  - in which case the host or plug-in does not support OpenCL Images render
///       - "true"   - which means a host or plug-in can support OpenCL Images render
///
///     @propdef
///     type: enum
///     dimension: 1
///     values:
///       - "false"
///       - "true"
///     introduced: "1.5"
/// ```
pub const kOfxImageEffectPropOpenCLSupported: &::std::ffi::CStr = c"OfxImageEffectPropOpenCLSupported";
/// ```doxygen
/// @brief Indicates that a plug-in SHOULD use OpenCL render in
/// the current action
///
///    If a plug-in and host have both set
///    kOfxImageEffectPropOpenCLRenderSupported="true" or have both
///    set kOfxImageEffectPropOpenCLSupported="true" then the host MAY
///    set this property to indicate that it is passing images as OpenCL
///    Buffers or Images.
///
///    When rendering using OpenCL Buffers, the cl_mem of the buffers are retrieved using ::kOfxImagePropData.
///    When rendering using OpenCL Images, the cl_mem of the images are retrieved using ::kOfxImageEffectPropOpenCLImage.
///    If both ::kOfxImageEffectPropOpenCLSupported (Buffers) and ::kOfxImageEffectPropOpenCLRenderSupported (Images) are
///    enabled by the plug-in, it should use ::kOfxImageEffectPropOpenCLImage to determine which is being used by the host.
///
///    - Valid Values
///       - 0 indicates that a plug-in SHOULD use OpenCL render in
///           the render action
///       - 1 indicates that a plug-in SHOULD NOT use OpenCL render in
///           the render action
///
///     @propdef
///     type: bool
///     dimension: 1
///     introduced: "1.5"
/// ```
pub const kOfxImageEffectPropOpenCLEnabled: &::std::ffi::CStr = c"OfxImageEffectPropOpenCLEnabled";
/// ```doxygen
/// @brief Indicates the OpenCL command queue that should be used for rendering
///
/// This property contains a pointer to the command queue to be used for
/// OpenCL rendering (cl_command_queue). In order to use it,
/// reinterpret_cast<cl_command_queue>(pointer) is needed.
///
/// The plug-in SHOULD ensure that its render action enqueues any
/// asynchronous OpenCL operations onto the supplied queue.
///
/// The plug-in SHOULD NOT wait for final asynchronous operations to
/// complete before returning from the render action.
///
///     @propdef
///     type: pointer
///     dimension: 1
///     introduced: "1.5"
/// ```
pub const kOfxImageEffectPropOpenCLCommandQueue: &::std::ffi::CStr = c"OfxImageEffectPropOpenCLCommandQueue";
/// ```doxygen
/// @brief Indicates the image handle of an image supplied as an OpenCL Image by the host
///
/// This value should be cast to a cl_mem and used as the image handle when performing
/// OpenCL Images operations. The property should be used (not ::kOfxImagePropData) when
/// rendering with OpenCL Images (::kOfxImageEffectPropOpenCLSupported), and should be used
/// to determine whether Images or Buffers should be used if a plug-in supports both
/// ::kOfxImageEffectPropOpenCLSupported and ::kOfxImageEffectPropOpenCLRenderSupported.
/// Note: the kOfxImagePropRowBytes property is not required to be set by the host, since
/// OpenCL Images do not have the concept of row bytes.
///
///     @propdef
///     type: pointer
///     dimension: 1
///     introduced: "1.5"
/// ```
pub const kOfxImageEffectPropOpenCLImage: &::std::ffi::CStr = c"OfxImageEffectPropOpenCLImage";
pub const kOfxOpenCLProgramSuite: &::std::ffi::CStr = c"OfxOpenCLProgramSuite";
/// ```doxygen
/// @brief OFX suite that provides image to texture conversion for OpenGL
///     processing
/// ```
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OfxImageEffectOpenGLRenderSuiteV1 {
    /// ```doxygen
    /// @brief loads an image from an OFX clip as a texture into OpenGL
    ///
    ///       \arg \c clip   clip to load the image from
    ///       \arg \c time   effect time to load the image from
    ///       \arg \c format requested texture format (As in
    ///             none,byte,word,half,float, etc..)
    ///             When set to NULL, the host decides the format based on the
    /// 	    plug-in's ::kOfxOpenGLPropPixelDepth setting.
    ///       \arg \c region region of the image to load (optional, set to NULL to
    ///             get a 'default' region)
    /// 	    this is in the \ref CanonicalCoordinates.
    ///       \arg \c textureHandle property set containing information about the
    ///             texture
    ///
    ///   An image is fetched from a clip at the indicated time for the given region
    ///   and loaded into an OpenGL texture. When a specific format is requested, the
    ///   host ensures it gives the requested format.
    ///   When the clip specified is the "Output" clip, the format is ignored and
    ///   the host must bind the resulting texture as the current color buffer
    ///   (render target). This may also be done prior to calling the
    ///   ::kOfxImageEffectActionRender action.
    ///   If the \em region parameter is set to non-NULL, then it will be clipped to
    ///   the clip's Region of Definition for the given time.
    ///   The returned image will be \em at \em least as big as this region.
    ///   If the region parameter is not set or is NULL, then the region fetched will be at
    ///   least the Region of Interest the effect has previously specified, clipped to
    ///   the clip's Region of Definition.
    ///   Information about the texture, including the texture index, is returned in
    ///   the \em textureHandle argument.
    ///   The properties on this handle will be...
    ///     - ::kOfxImageEffectPropOpenGLTextureIndex
    ///     - ::kOfxImageEffectPropOpenGLTextureTarget
    ///     - ::kOfxImageEffectPropPixelDepth
    ///     - ::kOfxImageEffectPropComponents
    ///     - ::kOfxImageEffectPropPreMultiplication
    ///     - ::kOfxImageEffectPropRenderScale
    ///     - ::kOfxImagePropPixelAspectRatio
    ///     - ::kOfxImagePropBounds
    ///     - ::kOfxImagePropRegionOfDefinition
    ///     - ::kOfxImagePropRowBytes
    ///     - ::kOfxImagePropField
    ///     - ::kOfxImagePropUniqueIdentifier
    ///
    ///   With the exception of the OpenGL specifics, these properties are the same
    ///   as the properties in an image handle returned by clipGetImage in the image
    ///   effect suite.
    /// \pre
    ///  - clip was returned by clipGetHandle
    ///  - Format property in the texture handle
    ///
    /// \post
    ///  - texture handle to be disposed of by clipFreeTexture before the action
    /// returns
    ///  - when the clip specified is the "Output" clip, the format is ignored and
    ///    the host must bind the resulting texture as the current color buffer
    ///    (render target).
    ///    This may also be done prior to calling the render action.
    ///
    /// @returns
    ///   - ::kOfxStatOK           - the image was successfully fetched and returned
    ///                              in the handle,
    ///   - ::kOfxStatFailed       - the image could not be fetched because it does
    ///                              not exist in the clip at the indicated
    ///                              time and/or region, the plug-in should continue
    ///                              operation, but assume the image was black and
    /// 			     transparent.
    ///   - ::kOfxStatErrBadHandle - the clip handle was invalid,
    ///   - ::kOfxStatErrMemory    - not enough OpenGL memory was available for the
    ///                              effect to load the texture.
    ///                              The plug-in should abort the GL render and
    /// 			     return ::kOfxStatErrMemory, after which the host can
    /// 			     decide to retry the operation with CPU based processing.
    ///
    /// \note
    ///   - this is the OpenGL equivalent of clipGetImage from OfxImageEffectSuiteV1
    /// ```
    pub clipLoadTexture: ::std::option::Option<
        unsafe extern "C" fn(
            clip: OfxImageClipHandle,
            time: OfxTime,
            format: *const ::std::os::raw::c_char,
            region: *const OfxRectD,
            textureHandle: *mut OfxPropertySetHandle,
        ) -> OfxStatus,
    >,
    /// ```doxygen
    /// @brief Releases the texture handle previously returned by
    /// clipLoadTexture
    ///
    ///   For input clips, this also deletes the texture from OpenGL.
    ///   This should also be called on the output clip; for the Output
    ///   clip, it just releases the handle but does not delete the
    ///   texture (since the host will need to read it).
    ///
    ///   \pre
    ///     - textureHandle was returned by clipGetImage
    ///
    ///   \post
    ///     - all operations on textureHandle will be invalid, and the OpenGL texture
    ///       it referred to has been deleted (for source clips)
    ///
    ///   @returns
    ///     - ::kOfxStatOK - the image was successfully fetched and returned in the
    ///          handle,
    ///     - ::kOfxStatFailed - general failure for some reason,
    ///     - ::kOfxStatErrBadHandle - the image handle was invalid,
    /// ```
    pub clipFreeTexture: ::std::option::Option<
        unsafe extern "C" fn(textureHandle: OfxPropertySetHandle) -> OfxStatus,
    >,
    /// ```doxygen
    /// @brief Request the host to minimize its GPU resource load
    ///
    ///   When a plug-in fails to allocate GPU resources, it can call this function to
    ///   request the host to flush its GPU resources if it holds any.
    ///   After the function the plug-in can try again to allocate resources which then
    ///   might succeed if the host actually has released anything.
    ///
    ///   \pre
    ///   \post
    ///     - No changes to the plug-in GL state should have been made.
    ///
    ///   @returns
    ///     - ::kOfxStatOK           - the host has actually released some
    /// resources,
    ///     - ::kOfxStatReplyDefault - nothing the host could do..
    /// ```
    pub flushResources: ::std::option::Option<unsafe extern "C" fn() -> OfxStatus>,
}
/// ```doxygen
/// @brief OFX suite that allows a plug-in to get OpenCL programs compiled
///
/// This is an optional suite the host can provide for building OpenCL programs for the plug-in,
/// as an alternative to calling clCreateProgramWithSource / clBuildProgram. There are two advantages to
/// doing this: The host can add flags (such as -cl-denorms-are-zero) to the build call, and may also
/// cache program binaries for performance (however, if the source of the program or the OpenCL
/// environment changes, the host must recompile so some mechanism such as hashing must be used).
/// ```
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OfxOpenCLProgramSuiteV1 {
    /// ```doxygen
    /// @brief Compiles the OpenCL program
    /// ```
    /// ## Parameters
    /// ### Parameter `fOptional`
    /// ```doxygen
    /// if non-zero, host may skip compiling on this call
    /// ```
    /// ### Parameter `pResult`
    /// ```doxygen
    /// cast to cl_program*
    /// ```
    pub compileProgram: ::std::option::Option<
        unsafe extern "C" fn(
            pszProgramSource: *const ::std::os::raw::c_char,
            fOptional: ::std::os::raw::c_int,
            pResult: *mut ::std::os::raw::c_void,
        ) -> OfxStatus,
    >,
}
/// ```doxygen
/// @brief GPU render ran out of memory
/// ```
pub const kOfxStatGPUOutOfMemory: OfxStatus = 1001;
/// ```doxygen
/// @brief OpenGL render ran out of memory (same as ``kOfxStatGPUOutOfMemory``)
/// ```
pub const kOfxStatGLOutOfMemory: OfxStatus = 1001;
/// ```doxygen
/// @brief GPU render failed in a non-memory-related way
/// ```
pub const kOfxStatGPURenderFailed: OfxStatus = 1002;
/// ```doxygen
/// @brief OpenGL render failed in a non-memory-related way (same as ``kOfxStatGPURenderFailed``)
/// ```
/// ```doxygen
/// for backward compatibility
/// ```
pub const kOfxStatGLRenderFailed: OfxStatus = 1002;

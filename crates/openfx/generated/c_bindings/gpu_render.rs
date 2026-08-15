use super::core::{OfxPropertySetHandle, OfxRectD, OfxStatus, OfxTime};
use super::image_effect::OfxImageClipHandle;
pub const kOfxOpenGLRenderSuite: &::std::ffi::CStr = c"OfxImageEffectOpenGLRenderSuite";
pub const kOfxImageEffectPropOpenGLRenderSupported: &::std::ffi::CStr = c"OfxImageEffectPropOpenGLRenderSupported";
pub const kOfxOpenGLPropPixelDepth: &::std::ffi::CStr = c"OfxOpenGLPropPixelDepth";
pub const kOfxImageEffectPropOpenGLEnabled: &::std::ffi::CStr = c"OfxImageEffectPropOpenGLEnabled";
pub const kOfxImageEffectPropOpenGLTextureIndex: &::std::ffi::CStr = c"OfxImageEffectPropOpenGLTextureIndex";
pub const kOfxImageEffectPropOpenGLTextureTarget: &::std::ffi::CStr = c"OfxImageEffectPropOpenGLTextureTarget";
pub const kOfxImageEffectPropCPURenderSupported: &::std::ffi::CStr = c"OfxImageEffectPropCPURenderSupported";
pub const kOfxActionOpenGLContextAttached: &::std::ffi::CStr = c"OfxActionOpenGLContextAttached";
pub const kOfxActionOpenGLContextDetached: &::std::ffi::CStr = c"kOfxActionOpenGLContextDetached";
pub const kOfxImageEffectPropCudaRenderSupported: &::std::ffi::CStr = c"OfxImageEffectPropCudaRenderSupported";
pub const kOfxImageEffectPropCudaEnabled: &::std::ffi::CStr = c"OfxImageEffectPropCudaEnabled";
pub const kOfxImageEffectPropCudaStreamSupported: &::std::ffi::CStr = c"OfxImageEffectPropCudaStreamSupported";
pub const kOfxImageEffectPropCudaStream: &::std::ffi::CStr = c"OfxImageEffectPropCudaStream";
pub const kOfxImageEffectPropMetalRenderSupported: &::std::ffi::CStr = c"OfxImageEffectPropMetalRenderSupported";
pub const kOfxImageEffectPropMetalEnabled: &::std::ffi::CStr = c"OfxImageEffectPropMetalEnabled";
pub const kOfxImageEffectPropMetalCommandQueue: &::std::ffi::CStr = c"OfxImageEffectPropMetalCommandQueue";
pub const kOfxImageEffectPropOpenCLRenderSupported: &::std::ffi::CStr = c"OfxImageEffectPropOpenCLRenderSupported";
pub const kOfxImageEffectPropOpenCLSupported: &::std::ffi::CStr = c"OfxImageEffectPropOpenCLSupported";
pub const kOfxImageEffectPropOpenCLEnabled: &::std::ffi::CStr = c"OfxImageEffectPropOpenCLEnabled";
pub const kOfxImageEffectPropOpenCLCommandQueue: &::std::ffi::CStr = c"OfxImageEffectPropOpenCLCommandQueue";
pub const kOfxImageEffectPropOpenCLImage: &::std::ffi::CStr = c"OfxImageEffectPropOpenCLImage";
pub const kOfxOpenCLProgramSuite: &::std::ffi::CStr = c"OfxOpenCLProgramSuite";
/** @brief OFX suite that provides image to texture conversion for OpenGL
processing*/
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OfxImageEffectOpenGLRenderSuiteV1 {
    /** @brief loads an image from an OFX clip as a texture into OpenGL

\arg \c clip   clip to load the image from
\arg \c time   effect time to load the image from
\arg \c format requested texture format (As in
none,byte,word,half,float, etc..)
When set to NULL, the host decides the format based on the
plug-in's ::kOfxOpenGLPropPixelDepth setting.
\arg \c region region of the image to load (optional, set to NULL to
get a 'default' region)
this is in the \ref CanonicalCoordinates.
\arg \c textureHandle property set containing information about the
texture

An image is fetched from a clip at the indicated time for the given region
and loaded into an OpenGL texture. When a specific format is requested, the
host ensures it gives the requested format.
When the clip specified is the "Output" clip, the format is ignored and
the host must bind the resulting texture as the current color buffer
(render target). This may also be done prior to calling the
::kOfxImageEffectActionRender action.
If the \em region parameter is set to non-NULL, then it will be clipped to
the clip's Region of Definition for the given time.
The returned image will be \em at \em least as big as this region.
If the region parameter is not set or is NULL, then the region fetched will be at
least the Region of Interest the effect has previously specified, clipped to
the clip's Region of Definition.
Information about the texture, including the texture index, is returned in
the \em textureHandle argument.
The properties on this handle will be...
- ::kOfxImageEffectPropOpenGLTextureIndex
- ::kOfxImageEffectPropOpenGLTextureTarget
- ::kOfxImageEffectPropPixelDepth
- ::kOfxImageEffectPropComponents
- ::kOfxImageEffectPropPreMultiplication
- ::kOfxImageEffectPropRenderScale
- ::kOfxImagePropPixelAspectRatio
- ::kOfxImagePropBounds
- ::kOfxImagePropRegionOfDefinition
- ::kOfxImagePropRowBytes
- ::kOfxImagePropField
- ::kOfxImagePropUniqueIdentifier

With the exception of the OpenGL specifics, these properties are the same
as the properties in an image handle returned by clipGetImage in the image
effect suite.
\pre
- clip was returned by clipGetHandle
- Format property in the texture handle

\post
- texture handle to be disposed of by clipFreeTexture before the action
returns
- when the clip specified is the "Output" clip, the format is ignored and
the host must bind the resulting texture as the current color buffer
(render target).
This may also be done prior to calling the render action.

@returns
- ::kOfxStatOK           - the image was successfully fetched and returned
in the handle,
- ::kOfxStatFailed       - the image could not be fetched because it does
not exist in the clip at the indicated
time and/or region, the plug-in should continue
operation, but assume the image was black and
transparent.
- ::kOfxStatErrBadHandle - the clip handle was invalid,
- ::kOfxStatErrMemory    - not enough OpenGL memory was available for the
effect to load the texture.
The plug-in should abort the GL render and
return ::kOfxStatErrMemory, after which the host can
decide to retry the operation with CPU based processing.

\note
- this is the OpenGL equivalent of clipGetImage from OfxImageEffectSuiteV1
*/
    pub clipLoadTexture: ::std::option::Option<
        unsafe extern "C" fn(
            clip: OfxImageClipHandle,
            time: OfxTime,
            format: *const ::std::os::raw::c_char,
            region: *const OfxRectD,
            textureHandle: *mut OfxPropertySetHandle,
        ) -> OfxStatus,
    >,
    /** @brief Releases the texture handle previously returned by
clipLoadTexture

For input clips, this also deletes the texture from OpenGL.
This should also be called on the output clip; for the Output
clip, it just releases the handle but does not delete the
texture (since the host will need to read it).

\pre
- textureHandle was returned by clipGetImage

\post
- all operations on textureHandle will be invalid, and the OpenGL texture
it referred to has been deleted (for source clips)

@returns
- ::kOfxStatOK - the image was successfully fetched and returned in the
handle,
- ::kOfxStatFailed - general failure for some reason,
- ::kOfxStatErrBadHandle - the image handle was invalid,*/
    pub clipFreeTexture: ::std::option::Option<
        unsafe extern "C" fn(textureHandle: OfxPropertySetHandle) -> OfxStatus,
    >,
    /** @brief Request the host to minimize its GPU resource load

When a plug-in fails to allocate GPU resources, it can call this function to
request the host to flush its GPU resources if it holds any.
After the function the plug-in can try again to allocate resources which then
might succeed if the host actually has released anything.

\pre
\post
- No changes to the plug-in GL state should have been made.

@returns
- ::kOfxStatOK           - the host has actually released some
resources,
- ::kOfxStatReplyDefault - nothing the host could do..*/
    pub flushResources: ::std::option::Option<unsafe extern "C" fn() -> OfxStatus>,
}
/** @brief OFX suite that allows a plug-in to get OpenCL programs compiled

This is an optional suite the host can provide for building OpenCL programs for the plug-in,
as an alternative to calling clCreateProgramWithSource / clBuildProgram. There are two advantages to
doing this: The host can add flags (such as -cl-denorms-are-zero) to the build call, and may also
cache program binaries for performance (however, if the source of the program or the OpenCL
environment changes, the host must recompile so some mechanism such as hashing must be used).*/
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OfxOpenCLProgramSuiteV1 {
    /// @brief Compiles the OpenCL program
    pub compileProgram: ::std::option::Option<
        unsafe extern "C" fn(
            pszProgramSource: *const ::std::os::raw::c_char,
            fOptional: ::std::os::raw::c_int,
            pResult: *mut ::std::os::raw::c_void,
        ) -> OfxStatus,
    >,
}

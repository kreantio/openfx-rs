// Copyright OpenFX and contributors to the OpenFX project.
// SPDX-License-Identifier: BSD-3-Clause
use super::core::{OfxPropertySetHandle, OfxRangeD, OfxStatus, OfxTime};
/// ```doxygen
/// @brief string value to the ::kOfxPropType property for all parameters
/// ```
pub const kOfxParameterSuite: &::std::ffi::CStr = c"OfxParameterSuite";
/// ```doxygen
/// @brief string value on the ::kOfxPropType property for all parameter definitions (ie: the handle returned in describe)
/// ```
pub const kOfxTypeParameter: &::std::ffi::CStr = c"OfxTypeParameter";
/// ```doxygen
/// @brief string value on the ::kOfxPropType property for all parameter instances
/// ```
pub const kOfxTypeParameterInstance: &::std::ffi::CStr = c"OfxTypeParameterInstance";
/// ```doxygen
/// @brief String to identify a param as a single valued integer
/// ```
pub const kOfxParamTypeInteger: &::std::ffi::CStr = c"OfxParamTypeInteger";
/// ```doxygen
/// @brief String to identify a param as a Single valued floating point parameter
/// ```
pub const kOfxParamTypeDouble: &::std::ffi::CStr = c"OfxParamTypeDouble";
/// ```doxygen
/// @brief String to identify a param as a Single valued boolean parameter
/// ```
pub const kOfxParamTypeBoolean: &::std::ffi::CStr = c"OfxParamTypeBoolean";
/// ```doxygen
/// @brief String to identify a param as a Single valued, 'one-of-many' parameter
/// ```
pub const kOfxParamTypeChoice: &::std::ffi::CStr = c"OfxParamTypeChoice";
/// ```doxygen
/// @brief String to identify a param as a string-valued 'one-of-many' parameter. \since Version 1.5
/// ```
pub const kOfxParamTypeStrChoice: &::std::ffi::CStr = c"OfxParamTypeStrChoice";
/// ```doxygen
/// @brief String to identify a param as a Red, Green, Blue and Alpha colour parameter
/// ```
pub const kOfxParamTypeRGBA: &::std::ffi::CStr = c"OfxParamTypeRGBA";
/// ```doxygen
/// @brief String to identify a param as a Red, Green and Blue colour parameter
/// ```
pub const kOfxParamTypeRGB: &::std::ffi::CStr = c"OfxParamTypeRGB";
/// ```doxygen
/// @brief String to identify a param as a Two dimensional floating point parameter
/// ```
pub const kOfxParamTypeDouble2D: &::std::ffi::CStr = c"OfxParamTypeDouble2D";
/// ```doxygen
/// @brief String to identify a param as a Two dimensional integer point parameter
/// ```
pub const kOfxParamTypeInteger2D: &::std::ffi::CStr = c"OfxParamTypeInteger2D";
/// ```doxygen
/// @brief String to identify a param as a Three dimensional floating point parameter
/// ```
pub const kOfxParamTypeDouble3D: &::std::ffi::CStr = c"OfxParamTypeDouble3D";
/// ```doxygen
/// @brief String to identify a param as a Three dimensional integer parameter
/// ```
pub const kOfxParamTypeInteger3D: &::std::ffi::CStr = c"OfxParamTypeInteger3D";
/// ```doxygen
/// @brief String to identify a param as a String (UTF8) parameter
/// ```
pub const kOfxParamTypeString: &::std::ffi::CStr = c"OfxParamTypeString";
/// ```doxygen
/// @brief String to identify a param as a Plug-in defined parameter
/// ```
pub const kOfxParamTypeCustom: &::std::ffi::CStr = c"OfxParamTypeCustom";
/// ```doxygen
/// @brief String to identify a param as a Plug-in defined opaque data parameter
/// ```
pub const kOfxParamTypeBytes: &::std::ffi::CStr = c"OfxParamTypeBytes";
/// ```doxygen
/// @brief String to identify a param as a Grouping parameter
/// ```
pub const kOfxParamTypeGroup: &::std::ffi::CStr = c"OfxParamTypeGroup";
/// ```doxygen
/// @brief String to identify a param as a page parameter
/// ```
pub const kOfxParamTypePage: &::std::ffi::CStr = c"OfxParamTypePage";
/// ```doxygen
/// @brief String to identify a param as a PushButton parameter
/// ```
pub const kOfxParamTypePushButton: &::std::ffi::CStr = c"OfxParamTypePushButton";
/// ```doxygen
/// @brief Indicates if the host supports animation of custom parameters
///
///     @propdef
///     type: bool
///     dimension: 1
/// ```
pub const kOfxParamHostPropSupportsCustomAnimation: &::std::ffi::CStr = c"OfxParamHostPropSupportsCustomAnimation";
/// ```doxygen
/// @brief Indicates if the host supports animation of string params
///     @propdef
///     type: bool
///     dimension: 1
/// ```
pub const kOfxParamHostPropSupportsStringAnimation: &::std::ffi::CStr = c"OfxParamHostPropSupportsStringAnimation";
/// ```doxygen
/// @brief Indicates if the host supports animation of boolean params
///     @propdef
///     type: bool
///     dimension: 1
/// ```
pub const kOfxParamHostPropSupportsBooleanAnimation: &::std::ffi::CStr = c"OfxParamHostPropSupportsBooleanAnimation";
/// ```doxygen
/// @brief Indicates if the host supports animation of choice params
///     @propdef
///     type: bool
///     dimension: 1
/// ```
pub const kOfxParamHostPropSupportsChoiceAnimation: &::std::ffi::CStr = c"OfxParamHostPropSupportsChoiceAnimation";
/// ```doxygen
/// @brief Indicates if the host supports custom interacts for parameters
///
/// Currently custom interacts for parameters can only be drawn using OpenGL.
/// APIs will be added later to support using the new Draw Suite.
///     @propdef
///     type: bool
///     dimension: 1
/// ```
pub const kOfxParamHostPropSupportsCustomInteract: &::std::ffi::CStr = c"OfxParamHostPropSupportsCustomInteract";
/// ```doxygen
/// @brief Indicates the maximum numbers of parameters available on the host.
///
/// If set to -1 it implies unlimited number of parameters.
///
///     @propdef
///     type: int
///     dimension: 1
/// ```
pub const kOfxParamHostPropMaxParameters: &::std::ffi::CStr = c"OfxParamHostPropMaxParameters";
/// ```doxygen
/// @brief Indicates the maximum number of parameter pages.
///
///     If there is no limit to the number of pages on a host, set this to -1.
///
/// Hosts that do not support paged parameter layout should set this to zero.
///
///     @propdef
///     type: int
///     dimension: 1
/// ```
pub const kOfxParamHostPropMaxPages: &::std::ffi::CStr = c"OfxParamHostPropMaxPages";
/// ```doxygen
/// @brief This indicates the number of parameter rows and columns on a page.
///
/// If the host has supports paged parameter layout, used dimension 0 as the number of columns per page and dimension 1 as the number of rows per page.
///
///     @propdef
///     type: int
///     dimension: 2
/// ```
pub const kOfxParamHostPropPageRowColumnCount: &::std::ffi::CStr = c"OfxParamHostPropPageRowColumnCount";
/// ```doxygen
/// @brief Pseudo parameter name used to skip a row in a page layout.
///
/// Passed as a value to the \ref kOfxParamPropPageChild property.
///
/// See \ref ParametersInterfacesPagedLayouts for more details.
/// ```
pub const kOfxParamPageSkipRow: &::std::ffi::CStr = c"OfxParamPageSkipRow";
/// ```doxygen
/// @brief Pseudo parameter name used to skip a row in a page layout.
///
/// Passed as a value to the \ref kOfxParamPropPageChild property.
///
/// See \ref ParametersInterfacesPagedLayouts for more details.
/// ```
pub const kOfxParamPageSkipColumn: &::std::ffi::CStr = c"OfxParamPageSkipColumn";
/// ```doxygen
/// @brief Overrides the parameter's standard user interface with the given interact.
///
/// If set, the parameter's normal interface is replaced completely by the interact gui.
///
/// Currently custom interacts for parameters can only be drawn using OpenGL.
/// APIs will be added later to support using the new Draw Suite.
///
///     - Valid Values -  must point to a OfxPluginEntryPoint
///     @propdef
///     type: pointer
///     dimension: 1
/// ```
pub const kOfxParamPropInteractV1: &::std::ffi::CStr = c"OfxParamPropInteractV1";
/// ```doxygen
/// @brief The size of a parameter instance's custom interface in screen pixels.
///
/// This is set by a host to indicate the current size of a custom interface if the plug-in has one. If not this is set to (0,0).
///
///   @propdef
///   type: double
///   dimension: 2
/// ```
pub const kOfxParamPropInteractSize: &::std::ffi::CStr = c"OfxParamPropInteractSize";
/// ```doxygen
/// @brief The preferred aspect ratio of a parameter's custom interface.
///
/// If set to anything other than 0.0, the custom interface for this parameter will be of a size with this aspect ratio (x size/y size).
///
///     - Valid Values - greater than or equal to 0.0
///     @propdef
///     type: double
///     dimension: 1
/// ```
pub const kOfxParamPropInteractSizeAspect: &::std::ffi::CStr = c"OfxParamPropInteractSizeAspect";
/// ```doxygen
/// @brief The minimum size of a parameter's custom interface, in screen pixels.
///
/// Any custom interface will not be less than this size.
///
///     - Valid Values - greater than (0, 0)
///     @propdef
///     type: double
///     dimension: 2
/// ```
pub const kOfxParamPropInteractMinimumSize: &::std::ffi::CStr = c"OfxParamPropInteractMinimumSize";
/// ```doxygen
/// @brief The preferred size of a parameter's custom interface.
///
///   A host should attempt to set a parameter's custom interface on a parameter to be this size if possible, otherwise it will be of ::kOfxParamPropInteractSizeAspect aspect but larger than ::kOfxParamPropInteractMinimumSize.
///
///     - Valid Values - greater than (0, 0)
///     @propdef
///     type: int
///     dimension: 2
/// ```
pub const kOfxParamPropInteractPreferedSize: &::std::ffi::CStr = c"OfxParamPropInteractPreferedSize";
/// ```doxygen
/// @brief The type of a parameter.
///
/// This string will be set to the type that the parameter was create with.
///
///    @propdef
///    type: string
///    dimension: 1
/// ```
pub const kOfxParamPropType: &::std::ffi::CStr = c"OfxParamPropType";
/// ```doxygen
/// @brief Flags whether a parameter can animate.
///
/// A plug-in uses this property to indicate if a parameter is able to animate.
///     @propdef
///     type: bool
///     dimension: 1
/// ```
pub const kOfxParamPropAnimates: &::std::ffi::CStr = c"OfxParamPropAnimates";
/// ```doxygen
/// @brief Flags whether changes to a parameter should be put on the undo/redo stack
///     @propdef
///     type: bool
///     dimension: 1
/// ```
pub const kOfxParamPropCanUndo: &::std::ffi::CStr = c"OfxParamPropCanUndo";
/// ```doxygen
/// @brief States whether the plugin needs to resync its private data
///
/// The plugin should set this flag to true whenever any internal state has not
/// been flushed to the set of params.
///
/// The host will examine this property each time it does a copy or save
/// operation on the instance.
/// If it is set to 1, the host will call SyncPrivateData and then set
///    it to zero before doing the copy/save.
/// If it is set to 0, the host will assume that the param data
///    correctly represents the private state, and will not call
///    SyncPrivateData before copying/saving.
/// If this property is not set, the host will always call
///    SyncPrivateData before copying or saving the effect (as if the
///    property were set to 1 -- but the host will not create or
///    modify the property).
///
///     - Valid Values -
///         - 0 - no need to sync
///         - 1 - paramset is not synced
///     @propdef
///     type: bool
///     dimension: 1
/// ```
pub const kOfxPropParamSetNeedsSyncing: &::std::ffi::CStr = c"OfxPropParamSetNeedsSyncing";
/// ```doxygen
/// @brief Flags whether a parameter is currently animating.
///
/// Set by a host on a parameter instance to indicate if the parameter has a non-constant value set on it. This can
/// be as a consequence of animation or of scripting modifying the value, or of a parameter being connected to
/// an expression in the host.
///     @propdef
///     type: bool
///     dimension: 1
/// ```
pub const kOfxParamPropIsAnimating: &::std::ffi::CStr = c"OfxParamPropIsAnimating";
/// ```doxygen
/// @brief Flags whether the plugin will attempt to set the value of a parameter in some callback or analysis pass
///
/// This is used to tell the host whether the plug-in is going to attempt to set the value of the parameter.
///
/// @deprecated - v1.4: deprecated - to be removed in 1.5
///     @propdef
///     type: bool
///     dimension: 1
///     deprecated: "1.4"
/// ```
pub const kOfxParamPropPluginMayWrite: &::std::ffi::CStr = c"OfxParamPropPluginMayWrite";
/// ```doxygen
/// @brief Flags whether the value of a parameter should persist.
///
/// This is used to tell the host whether the value of the parameter is important and should be save in any description of the plug-in.
///     @propdef
///     type: bool
///     dimension: 1
/// ```
pub const kOfxParamPropPersistant: &::std::ffi::CStr = c"OfxParamPropPersistant";
/// ```doxygen
/// @brief Flags whether changing a parameter's value forces an evaluation (ie: render),
///
/// This is used to indicate if the value of a parameter has any affect on an effect's output, eg: the parameter may be purely for GUI purposes, and so changing its value should not trigger a re-render.
///     @propdef
///     type: bool
///     dimension: 1
/// ```
pub const kOfxParamPropEvaluateOnChange: &::std::ffi::CStr = c"OfxParamPropEvaluateOnChange";
/// ```doxygen
/// @brief Flags whether a parameter should be exposed to a user,
///
/// If secret, a parameter is not exposed to a user in any interface, but should otherwise behave as a normal parameter.
///
/// Secret params are typically used to hide important state detail that would otherwise be unintelligible to a user, for example the result of a statical analysis that might need many parameters to store.
///     @propdef
///     type: bool
///     dimension: 1
/// ```
pub const kOfxParamPropSecret: &::std::ffi::CStr = c"OfxParamPropSecret";
/// ```doxygen
/// @brief The value to be used as the id of the parameter in a host scripting language.
///
/// Many hosts have a scripting language that they use to set values of parameters and more. If so, this is the name of a parameter in such scripts.
///
///     - Valid Values - ASCII string unique to all parameters in the plug-in.
///     @propdef
///     type: string
///     dimension: 1
/// ```
pub const kOfxParamPropScriptName: &::std::ffi::CStr = c"OfxParamPropScriptName";
/// ```doxygen
/// @brief Specifies how modifying the value of a param will affect any output of an effect over time.
///
/// Imagine an effect with an animating parameter in a host that caches
/// rendered output. Think of the what happens when you add a new key frame.
///  -If the parameter represents something like an absolute position, the cache will only need to be invalidated for the range of frames that keyframe affects.
/// - If the parameter represents something like a speed which is integrated, the cache will be invalidated from the keyframe until the end of the clip.
/// - There are potentially other situations where the entire cache will need to be invalidated (though I can't think of one off the top of my head).
///
///    - Valid Values - This must be one of
///        - ::kOfxParamInvalidateValueChange
///        - ::kOfxParamInvalidateValueChangeToEnd
///        - ::kOfxParamInvalidateAll
///    @propdef
///    type: enum
///    dimension: 1
///    values:
///      - OfxParamInvalidateValueChange
///      - OfxParamInvalidateValueChangeToEnd
///      - OfxParamInvalidateAll
/// ```
pub const kOfxParamPropCacheInvalidation: &::std::ffi::CStr = c"OfxParamPropCacheInvalidation";
/// ```doxygen
/// @brief Used as a value for the ::kOfxParamPropCacheInvalidation property
/// ```
pub const kOfxParamInvalidateValueChange: &::std::ffi::CStr = c"OfxParamInvalidateValueChange";
/// ```doxygen
/// @brief Used as a value for the ::kOfxParamPropCacheInvalidation property
/// ```
pub const kOfxParamInvalidateValueChangeToEnd: &::std::ffi::CStr = c"OfxParamInvalidateValueChangeToEnd";
/// ```doxygen
/// @brief Used as a value for the ::kOfxParamPropCacheInvalidation property
/// ```
pub const kOfxParamInvalidateAll: &::std::ffi::CStr = c"OfxParamInvalidateAll";
/// ```doxygen
/// @brief A hint to the user as to how the parameter is to be used.
///
///    @propdef
///    type: string
///    dimension: 1
/// ```
pub const kOfxParamPropHint: &::std::ffi::CStr = c"OfxParamPropHint";
/// ```doxygen
/// @brief The default value of a parameter.
///
/// The exact type and dimension is dependent on the type of the parameter. These are....
///   - ::kOfxParamTypeInteger - integer property of one dimension
///   - ::kOfxParamTypeDouble - double property of one dimension
///   - ::kOfxParamTypeBoolean - integer property of one dimension
///   - ::kOfxParamTypeChoice - integer property of one dimension
///   - ::kOfxParamTypeStrChoice - string property of one dimension
///   - ::kOfxParamTypeRGBA - double property of four dimensions
///   - ::kOfxParamTypeRGB - double property of three dimensions
///   - ::kOfxParamTypeDouble2D - double property of two dimensions
///   - ::kOfxParamTypeInteger2D - integer property of two dimensions
///   - ::kOfxParamTypeDouble3D - double property of three dimensions
///   - ::kOfxParamTypeInteger3D - integer property of three dimensions
///   - ::kOfxParamTypeString - string property of one dimension
///   - ::kOfxParamTypeCustom - string property of one dimension
///   - ::kOfxParamTypeBytes - pointer to OfxBytes struct of one dimension, or nullptr
///   - ::kOfxParamTypeGroup - does not have this property
///   - ::kOfxParamTypePage - does not have this property
///   - ::kOfxParamTypePushButton - does not have this property
///
///    @propdef
///    type: [int, double, string, pointer]
///    dimension: N
/// ```
pub const kOfxParamPropDefault: &::std::ffi::CStr = c"OfxParamPropDefault";
/// ```doxygen
/// @brief Describes how the double parameter should be interpreted by a host.
///
///    - Valid Values -This must be one of
///       - ::kOfxParamDoubleTypePlain - parameter has no special interpretation,
///       - ::kOfxParamDoubleTypeAngle - parameter is to be interpreted as an angle,
///       - ::kOfxParamDoubleTypeScale - parameter is to be interpreted as a scale factor,
///       - ::kOfxParamDoubleTypeTime  - parameter represents a time value (1D only),
///       - ::kOfxParamDoubleTypeAbsoluteTime  - parameter represents an absolute time value (1D only),
///
///       - ::kOfxParamDoubleTypeX - size wrt to the project's X dimension (1D only), in canonical coordinates,
///       - ::kOfxParamDoubleTypeXAbsolute - absolute position on the X axis (1D only), in canonical coordinates,
///       - ::kOfxParamDoubleTypeY - size wrt to the project's Y dimension(1D only), in canonical coordinates,
///       - ::kOfxParamDoubleTypeYAbsolute - absolute position on the Y axis (1D only), in canonical coordinates,
///       - ::kOfxParamDoubleTypeXY - size in 2D (2D only), in canonical coordinates,
///       - ::kOfxParamDoubleTypeXYAbsolute - an absolute position on the image plane, in canonical coordinates.
///
/// Double parameters can be interpreted in several different ways, this property tells the host how to do so and thus gives hints
/// as to the interface of the parameter.
///    @propdef
///    type: enum
///    dimension: 1
///    values:
///      - OfxParamDoubleTypePlain
///      - OfxParamDoubleTypeAngle
///      - OfxParamDoubleTypeScale
///      - OfxParamDoubleTypeTime
///      - OfxParamDoubleTypeAbsoluteTime
///      - OfxParamDoubleTypeX
///      - OfxParamDoubleTypeXAbsolute
///      - OfxParamDoubleTypeY
///      - OfxParamDoubleTypeYAbsolute
///      - OfxParamDoubleTypeXY
///      - OfxParamDoubleTypeXYAbsolute
/// ```
pub const kOfxParamPropDoubleType: &::std::ffi::CStr = c"OfxParamPropDoubleType";
/// ```doxygen
/// @brief value for the ::kOfxParamPropDoubleType property, indicating the parameter has no special interpretation and should be interpreted as a raw numeric value.
/// ```
pub const kOfxParamDoubleTypePlain: &::std::ffi::CStr = c"OfxParamDoubleTypePlain";
/// ```doxygen
/// @brief value for the ::kOfxParamPropDoubleType property, indicating the parameter is to be interpreted as a scale factor. See \ref ::kOfxParamPropDoubleType.
/// ```
pub const kOfxParamDoubleTypeScale: &::std::ffi::CStr = c"OfxParamDoubleTypeScale";
/// ```doxygen
/// @brief value for the ::kOfxParamDoubleTypeAngle property, indicating the parameter is to be interpreted as an angle. See \ref ::kOfxParamPropDoubleType.
/// ```
pub const kOfxParamDoubleTypeAngle: &::std::ffi::CStr = c"OfxParamDoubleTypeAngle";
/// ```doxygen
/// @brief value for the ::kOfxParamDoubleTypeAngle property, indicating the parameter is to be interpreted as a time. See \ref ::kOfxParamPropDoubleType.
/// ```
pub const kOfxParamDoubleTypeTime: &::std::ffi::CStr = c"OfxParamDoubleTypeTime";
/// ```doxygen
/// @brief value for the ::kOfxParamDoubleTypeAngle property, indicating the parameter is to be interpreted as an absolute time from the start of the effect. See \ref ::kOfxParamPropDoubleType.
/// ```
pub const kOfxParamDoubleTypeAbsoluteTime: &::std::ffi::CStr = c"OfxParamDoubleTypeAbsoluteTime";
/// ```doxygen
/// @brief value for the ::kOfxParamPropDoubleType property, indicating a size in canonical coords in the X dimension. See \ref ::kOfxParamPropDoubleType.
/// ```
pub const kOfxParamDoubleTypeX: &::std::ffi::CStr = c"OfxParamDoubleTypeX";
/// ```doxygen
/// @brief value for the ::kOfxParamPropDoubleType property, indicating a size in canonical coords in the Y dimension. See \ref ::kOfxParamPropDoubleType.
/// ```
pub const kOfxParamDoubleTypeY: &::std::ffi::CStr = c"OfxParamDoubleTypeY";
/// ```doxygen
/// @brief value for the ::kOfxParamPropDoubleType property, indicating an absolute position in canonical coords in the X dimension. See \ref ::kOfxParamPropDoubleType.
/// ```
pub const kOfxParamDoubleTypeXAbsolute: &::std::ffi::CStr = c"OfxParamDoubleTypeXAbsolute";
/// ```doxygen
/// @brief value for the ::kOfxParamPropDoubleType property, indicating an absolute position in canonical coords in the Y dimension. See \ref ::kOfxParamPropDoubleType.
/// ```
pub const kOfxParamDoubleTypeYAbsolute: &::std::ffi::CStr = c"OfxParamDoubleTypeYAbsolute";
/// ```doxygen
/// @brief value for the ::kOfxParamPropDoubleType property, indicating a 2D size in canonical coords. See \ref ::kOfxParamPropDoubleType.
/// ```
pub const kOfxParamDoubleTypeXY: &::std::ffi::CStr = c"OfxParamDoubleTypeXY";
/// ```doxygen
/// @brief value for the ::kOfxParamPropDoubleType property, indicating a 2D position in canonical coords. See \ref ::kOfxParamPropDoubleType.
/// ```
pub const kOfxParamDoubleTypeXYAbsolute: &::std::ffi::CStr = c"OfxParamDoubleTypeXYAbsolute";
/// ```doxygen
/// @brief Describes in which coordinate system a spatial double parameter's default value is specified.
///
/// This allows a spatial param to specify what its default is, so by saying normalised and "0.5" it would be in the 'middle', by saying canonical and 100 it would be at value 100 independent of the size of the image being applied to.
///
///    - Valid Values - This must be one of
///       - kOfxParamCoordinatesCanonical - the default is in canonical coords
///       - kOfxParamCoordinatesNormalised - the default is in normalised coordinates
///    @propdef
///    type: enum
///    dimension: 1
///    values:
///      - OfxParamCoordinatesCanonical
///      - OfxParamCoordinatesNormalised
/// ```
pub const kOfxParamPropDefaultCoordinateSystem: &::std::ffi::CStr = c"OfxParamPropDefaultCoordinateSystem";
/// ```doxygen
/// @brief Define the canonical coordinate system
/// ```
pub const kOfxParamCoordinatesCanonical: &::std::ffi::CStr = c"OfxParamCoordinatesCanonical";
/// ```doxygen
/// @brief Define the normalised coordinate system
/// ```
pub const kOfxParamCoordinatesNormalised: &::std::ffi::CStr = c"OfxParamCoordinatesNormalised";
/// ```doxygen
/// @brief A flag to indicate if there is a host overlay UI handle for the given parameter.
///
/// If set to 1, then the host is flagging that there is some sort of native user overlay interface handle available for the given parameter.
///     @propdef
///     type: bool
///     dimension: 1
/// ```
pub const kOfxParamPropHasHostOverlayHandle: &::std::ffi::CStr = c"OfxParamPropHasHostOverlayHandle";
/// ```doxygen
/// @brief A flag to indicate that the host should use a native UI overlay handle for the given parameter.
///
/// If set to 1, then a plugin is flaging to the host that the host should use a native UI overlay handle for the given parameter. A plugin can use this to keep a native look and feel for parameter handles. A plugin can use ::kOfxParamPropHasHostOverlayHandle to see if handles are available on the given parameter.
///     @propdef
///     type: bool
///     dimension: 1
///     cname: kOfxParamPropUseHostOverlayHandle
/// ```
pub const kOfxParamPropUseHostOverlayHandle: &::std::ffi::CStr = c"kOfxParamPropUseHostOverlayHandle";
/// ```doxygen
/// @brief value for the ::kOfxParamInterpType property, indicating a constant/hold/step interpolation type. See \ref ::kOfxParamInterpType.
/// ```
pub const kOfxParamInterpTypeConstantStep: &::std::ffi::CStr = c"OfxParamInterpTypeConstantStep";
/// ```doxygen
/// @brief value for the ::kOfxParamInterpType property, indicating a linear interpolation type. See \ref ::kOfxParamInterpType.
/// ```
pub const kOfxParamInterpTypeLinear: &::std::ffi::CStr = c"OfxParamInterpTypeLinear";
/// ```doxygen
/// @brief value for the ::kOfxParamInterpType property, indicating some kind of smooth interpolation type. See \ref ::kOfxParamInterpType.
/// ```
pub const kOfxParamInterpTypeSmooth: &::std::ffi::CStr = c"OfxParamInterpTypeSmooth";
/// ```doxygen
/// @brief Sets the default interpolation type of a Integer or Double parameter.
///
///    - Type - C string X 1
///    - Default - ::kOfxParamInterpTypeLinear
///    - Property Set - 1D integer and double plugin parameter descriptor (read/write) and instance (read only)
///    - Valid Values - This must be one of
///       - ::kOfxParamInterpTypeConstantStep - constant/hold/step interpolation,
///       - ::kOfxParamInterpTypeLinear - linear interpolation,
///       - ::kOfxParamInterpTypeSmooth - some kind of smooth (bezier, hermite, cardinal, etc) interpolation
///
/// This allows a plugin to indicate how a number-type parameter should be interpolated by default. This is useful if a parameter
/// is used to encode a keyed set of interesting frame numbers to be used as markers or retime points without the host animating them.
/// ```
pub const kOfxParamInterpType: &::std::ffi::CStr = c"OfxParamInterpType";
/// ```doxygen
/// @brief Enables the display of a time marker on the host's time line to indicate the value of the absolute time param.
///
/// If a double parameter is has ::kOfxParamPropDoubleType set to ::kOfxParamDoubleTypeAbsoluteTime, then this indicates whether
/// any marker should be made visible on the host's time line.
///     @propdef
///     type: bool
///     dimension: 1
/// ```
pub const kOfxParamPropShowTimeMarker: &::std::ffi::CStr = c"OfxParamPropShowTimeMarker";
/// ```doxygen
/// @brief Sets the parameter pages and order of pages.
///
/// This property sets the preferred order of parameter pages on a host. If this is never set, the preferred order is the order the parameters were declared in.
///
///     - Valid Values - the names of any page param in the plugin
///     @propdef
///     type: string
///     dimension: N
/// ```
pub const kOfxPluginPropParamPageOrder: &::std::ffi::CStr = c"OfxPluginPropParamPageOrder";
/// ```doxygen
/// @brief The names of the parameters included in a page parameter.
///
/// This is a property on parameters of type ::kOfxParamTypePage, and tells the page what parameters it contains. The parameters are added to the page from the top left, filling in columns as we go. The two pseudo param names ::kOfxParamPageSkipRow and ::kOfxParamPageSkipColumn are used to control layout.
///
/// Note parameters can appear in more than one page.
///
///     - Valid Values - the names of any parameter that is not a group or page, as well as ::kOfxParamPageSkipRow and ::kOfxParamPageSkipColumn
///     @propdef
///     type: string
///     dimension: N
/// ```
pub const kOfxParamPropPageChild: &::std::ffi::CStr = c"OfxParamPropPageChild";
/// ```doxygen
/// @brief The name of a parameter's parent group.
///
/// Hosts that have hierarchical layouts of their params use this to recursively group parameter.
///
/// By default parameters are added in order of declaration to the 'root' hierarchy. This property is used to reparent params to a predefined param of type ::kOfxParamTypeGroup.
///
///     - Valid Values - the name of a parameter with type of ::kOfxParamTypeGroup
///     @propdef
///     type: string
///     dimension: 1
/// ```
pub const kOfxParamPropParent: &::std::ffi::CStr = c"OfxParamPropParent";
/// ```doxygen
/// @brief Whether the initial state of a group is open or closed in a hierarchical layout.
///
/// This is a property on parameters of type ::kOfxParamTypeGroup, and tells the group whether it should be open or closed by default.
///     @propdef
///     type: bool
///     dimension: 1
/// ```
pub const kOfxParamPropGroupOpen: &::std::ffi::CStr = c"OfxParamPropGroupOpen";
/// ```doxygen
/// @brief Used to enable a parameter in the user interface.
///
/// When set to 0 a user should not be able to modify the value of the parameter. Note that the plug-in itself can still change the value of a disabled parameter.
///     @propdef
///     type: bool
///     dimension: 1
///     optional: true
/// ```
pub const kOfxParamPropEnabled: &::std::ffi::CStr = c"OfxParamPropEnabled";
/// ```doxygen
/// @brief A private data pointer that the plug-in can store its own data behind.
///
/// This data pointer is unique to each parameter instance, so two instances of the same parameter do not share the same data pointer. Use it to hang any needed private data structures.
///
///     @propdef
///     type: pointer
///     dimension: 1
/// ```
pub const kOfxParamPropDataPtr: &::std::ffi::CStr = c"OfxParamPropDataPtr";
/// ```doxygen
/// @brief Set options of a choice parameter.
///
/// This property contains the set of options that will be presented to a user
/// from a choice parameter. See @ref ParametersChoice for more details.
///
///     @propdef
///     type: string
///     dimension: N
/// ```
pub const kOfxParamPropChoiceOption: &::std::ffi::CStr = c"OfxParamPropChoiceOption";
/// ```doxygen
/// @brief Set values the host should store for a choice parameter.
///
/// (read/write),
/// `OfxParamPropChoiceOption`
///
/// This property specifies the order in which the options are presented.
/// See @ref "Choice Parameters" for more details.
/// This property is optional; if not set, the host will present the options in
/// their natural order.
///
/// This property is useful when changing order of choice param options, or adding
/// new options in the middle, in a new version of the plugin.
///
///   @verbatim
///   Plugin v1:
///   Option = {"OptA", "OptB", "OptC"}
///   Order = {1, 2, 3}
///
///   Plugin v2:
///   // will be shown as OptA / OptB / NewOpt / OptC
///   Option = {"OptA", "OptB", "OptC", NewOpt"}
///   Order = {1, 2, 4, 3}
///   @endverbatim
///
/// Note that this only affects the host UI's display order; the project still
/// stores the index of the selected option as always. Plugins should never
/// reorder existing options if they desire backward compatibility.
///
/// Values may be arbitrary 32-bit integers. Behavior is undefined if the same
/// value occurs twice in the list; plugins should not do that.
///
/// \since Version 1.5
///
///     @propdef
///     type: int
///     dimension: N
/// ```
pub const kOfxParamPropChoiceOrder: &::std::ffi::CStr = c"OfxParamPropChoiceOrder";
/// ```doxygen
/// @brief Set a enumeration string in a StrChoice (string-valued choice) parameter.
///
/// (read/write),
/// This property contains the set of enumeration strings stored by the host in
/// the project corresponding to the options that will be presented to a user
/// from a StrChoice parameter. See @ref ParametersChoice for more details.
///
/// \since Version 1.5
///
///     @propdef
///     type: bool
///     dimension: 1
///     added: "1.5"
/// ```
pub const kOfxParamPropChoiceEnum: &::std::ffi::CStr = c"OfxParamPropChoiceEnum";
/// ```doxygen
/// @brief Indicates if the host supports animation of string choice params.
///
/// \since Version 1.5
///     @propdef
///     type: bool
///     dimension: 1
///     introduced: "1.5"
/// ```
pub const kOfxParamHostPropSupportsStrChoiceAnimation: &::std::ffi::CStr = c"OfxParamHostPropSupportsStrChoiceAnimation";
/// ```doxygen
/// @brief Indicates if the host supports the StrChoice param type.
///
/// \since Version 1.5
///     @propdef
///     type: bool
///     dimension: 1
///     introduced: "1.5"
/// ```
pub const kOfxParamHostPropSupportsStrChoice: &::std::ffi::CStr = c"OfxParamHostPropSupportsStrChoice";
/// ```doxygen
/// @brief The minimum value for a numeric parameter.
///
/// Setting this will also reset ::kOfxParamPropDisplayMin.
///
///     @propdef
///     type: [int, double]
///     dimension: N
/// ```
pub const kOfxParamPropMin: &::std::ffi::CStr = c"OfxParamPropMin";
/// ```doxygen
/// @brief The maximum value for a numeric parameter.
///
/// Setting this will also reset ::kOfxParamPropDisplayMax.
///
///     @propdef
///     type: [int, double]
///     dimension: N
/// ```
pub const kOfxParamPropMax: &::std::ffi::CStr = c"OfxParamPropMax";
/// ```doxygen
/// @brief The minimum value for a numeric parameter on any user interface.
///
/// If a user interface represents a parameter with a slider or similar, this should be the minimum bound on that slider.
///
///     @propdef
///     type: [int, double]
///     dimension: N
/// ```
pub const kOfxParamPropDisplayMin: &::std::ffi::CStr = c"OfxParamPropDisplayMin";
/// ```doxygen
/// @brief The maximum value for a numeric parameter on any user interface.
///
/// If a user interface represents a parameter with a slider or similar, this should be the maximum bound on that slider.
///
///     @propdef
///     type: [int, double]
///     dimension: N
/// ```
pub const kOfxParamPropDisplayMax: &::std::ffi::CStr = c"OfxParamPropDisplayMax";
/// ```doxygen
/// @brief The granularity of a slider used to represent a numeric parameter.
///
/// This value is always in canonical coordinates for double parameters that are normalised.
///
///     - Valid Values - any greater than 0.
///     @propdef
///     type: double
///     dimension: 1
/// ```
pub const kOfxParamPropIncrement: &::std::ffi::CStr = c"OfxParamPropIncrement";
/// ```doxygen
/// @brief How many digits after a decimal point to display for a double param in a GUI.
///
/// This applies to double params of any dimension.
///
///     - Valid Values - any greater than 0.
///     @propdef
///     type: int
///     dimension: 1
/// ```
pub const kOfxParamPropDigits: &::std::ffi::CStr = c"OfxParamPropDigits";
/// ```doxygen
/// @brief Label for individual dimensions on a multidimensional numeric parameter.
///
/// Use this on 2D and 3D double and integer parameters to change the label on an individual dimension in any GUI for that parameter.
///
///     - Valid Values - any
///     @propdef
///     type: string
///     dimension: 1
/// ```
pub const kOfxParamPropDimensionLabel: &::std::ffi::CStr = c"OfxParamPropDimensionLabel";
/// ```doxygen
/// @brief Will a value change on the parameter add automatic keyframes.
///
/// This is set by the host simply to indicate the state of the property.
///     @propdef
///     type: bool
///     dimension: 1
/// ```
pub const kOfxParamPropIsAutoKeying: &::std::ffi::CStr = c"OfxParamPropIsAutoKeying";
/// ```doxygen
/// @brief A pointer to a custom parameter's interpolation function.
///
/// It is an error not to set this property in a custom parameter during a plugin's define call if the custom parameter declares itself to be an animating parameter.
///
///     - Valid Values - must point to a ::OfxCustomParamInterpFuncV1
///     @propdef
///     type: pointer
///     dimension: 1
///     cname: kOfxParamPropCustomInterpCallbackV1
/// ```
pub const kOfxParamPropCustomInterpCallbackV1: &::std::ffi::CStr = c"OfxParamPropCustomCallbackV1";
/// ```doxygen
/// @brief Used to indicate the type of a string parameter.
///
///     - Valid Values - This must be one of the following
///         - ::kOfxParamStringIsSingleLine
///         - ::kOfxParamStringIsMultiLine
///         - ::kOfxParamStringIsFilePath
///         - ::kOfxParamStringIsDirectoryPath
///         - ::kOfxParamStringIsLabel
///         - ::kOfxParamStringIsRichTextFormat
///     @propdef
///     type: enum
///     dimension: 1
///     values:
///       - OfxParamStringIsSingleLine
///       - OfxParamStringIsMultiLine
///       - OfxParamStringIsFilePath
///       - OfxParamStringIsDirectoryPath
///       - OfxParamStringIsLabel
///       - OfxParamStringIsRichTextFormat
/// ```
pub const kOfxParamPropStringMode: &::std::ffi::CStr = c"OfxParamPropStringMode";
/// ```doxygen
/// @brief Indicates string parameters of file or directory type need that file to exist already.
///
/// If set to 0, it implies the user can specify a new file name, not just a pre-existing one.
///     @propdef
///     type: bool
///     dimension: 1
/// ```
pub const kOfxParamPropStringFilePathExists: &::std::ffi::CStr = c"OfxParamPropStringFilePathExists";
/// ```doxygen
/// @brief Used to set a string parameter to be single line,
///     value to be passed to a ::kOfxParamPropStringMode property
/// ```
pub const kOfxParamStringIsSingleLine: &::std::ffi::CStr = c"OfxParamStringIsSingleLine";
/// ```doxygen
/// @brief Used to set a string parameter to be multiple line,
///     value to be passed to a ::kOfxParamPropStringMode property
/// ```
pub const kOfxParamStringIsMultiLine: &::std::ffi::CStr = c"OfxParamStringIsMultiLine";
/// ```doxygen
/// @brief Used to set a string parameter to be a file path,
///     value to be passed to a ::kOfxParamPropStringMode property
/// ```
pub const kOfxParamStringIsFilePath: &::std::ffi::CStr = c"OfxParamStringIsFilePath";
/// ```doxygen
/// @brief Used to set a string parameter to be a directory path,
///     value to be passed to a ::kOfxParamPropStringMode property
/// ```
pub const kOfxParamStringIsDirectoryPath: &::std::ffi::CStr = c"OfxParamStringIsDirectoryPath";
/// ```doxygen
/// @brief Use to set a string parameter to be a simple label,
///     value to be passed to a ::kOfxParamPropStringMode property
/// ```
pub const kOfxParamStringIsLabel: &::std::ffi::CStr = c"OfxParamStringIsLabel";
/// ```doxygen
/// @brief String value on the ::kOfxParamPropStringMode property of a
///     string parameter (added in 1.3)
/// ```
pub const kOfxParamStringIsRichTextFormat: &::std::ffi::CStr = c"OfxParamStringIsRichTextFormat";
/// ```doxygen
/// @brief Used by interpolating custom parameters to get and set interpolated values.
/// This property is on the \e inArgs property and \e outArgs property of a ::OfxCustomParamInterpFuncV1 and in both cases contains the encoded value of a custom parameter. As an \e inArgs property it will have two values, being the two keyframes to interpolate. As an \e outArgs property it will have a single value and the plugin should fill this with the encoded interpolated value of the parameter.
///
///     @propdef
///     type: string
///     dimension: 2
/// ```
pub const kOfxParamPropCustomValue: &::std::ffi::CStr = c"OfxParamPropCustomValue";
/// ```doxygen
/// @brief Used by interpolating custom parameters to indicate the time a key occurs at.
///
/// The two values indicate the absolute times the surrounding keyframes occur at. The keyframes are encoded in a ::kOfxParamPropCustomValue property.
///
///    @propdef
///    type: double
///    dimension: 2
/// ```
pub const kOfxParamPropInterpolationTime: &::std::ffi::CStr = c"OfxParamPropInterpolationTime";
/// ```doxygen
/// @brief Property used by ::OfxCustomParamInterpFuncV1 to indicate the amount of interpolation to perform
///
/// This property indicates how far between the two ::kOfxParamPropCustomValue keys to interpolate.
///
///    - Valid Values - from 0 to 1
///    @propdef
///    type: double
///    dimension: 1
/// ```
pub const kOfxParamPropInterpolationAmount: &::std::ffi::CStr = c"OfxParamPropInterpolationAmount";
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OfxParamStruct {
    _unused: [u8; 0],
}
/// ```doxygen
/// @brief Blind declaration of an OFX param
/// ```
pub type OfxParamHandle = *mut OfxParamStruct;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OfxParamSetStruct {
    _unused: [u8; 0],
}
/// ```doxygen
/// @brief Blind declaration of an OFX parameter set
/// ```
pub type OfxParamSetHandle = *mut OfxParamSetStruct;
/// ```doxygen
/// @brief Provides information for a parameter of type kOfxParamTypeBytes
/// ```
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OfxBytes {
    /// ```doxygen
    /// @brief a pointer to the data buffer
    /// ```
    pub data: *const ::std::os::raw::c_uchar,
    /// ```doxygen
    /// @brief the length of the data buffer, in bytes
    /// ```
    pub length: usize,
}
/// ```doxygen
/// @brief Function prototype for custom parameter interpolation callback functions
///
///   \arg \c instance    the plugin instance that this parameter occurs in
///   \arg \c inArgs      handle holding the following properties...
///     - kOfxPropName - the name of the custom parameter to interpolate
///     - kOfxPropTime - absolute time the interpolation is occurring at
///     - kOfxParamPropCustomValue - string property that gives the value of the two keyframes to interpolate, in this case 2D
///     - kOfxParamPropInterpolationTime - 2D double property that gives the time of the two keyframes we are interpolating
///     - kOfxParamPropInterpolationAmount - 1D double property indicating how much to interpolate between the two keyframes
///
///   \arg \c outArgs     handle holding the following properties to be set
///     - kOfxParamPropCustomValue - the value of the interpolated custom parameter, in this case 1D
///
/// This function allows custom parameters to animate by performing interpolation between keys.
///
/// The plugin needs to parse the two strings encoding keyframes on either side of the time
/// we need a value for. It should then interpolate a new value for it, encode it into a string and set
/// the ::kOfxParamPropCustomValue property with this on the outArgs handle.
///
/// The interp value is a linear interpolation amount, however his may be derived from a cubic (or other) curve.
///
///     @actiondef
///     inArgs:
///       - OfxParamPropCustomValue
///       - OfxParamPropInterpolationTime
///       - OfxParamPropInterpolationAmount
///     outArgs:
///       - OfxParamPropCustomValue
///       - OfxParamPropInterpolationTime
/// ```
pub type OfxCustomParamInterpFuncV1 = ::std::option::Option<
    unsafe extern "C" fn(
        instance: OfxParamSetHandle,
        inArgs: OfxPropertySetHandle,
        outArgs: OfxPropertySetHandle,
    ) -> OfxStatus,
>;
/// ```doxygen
/// @brief The OFX suite used to define and manipulate user visible parameters
/// ```
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OfxParameterSuiteV1 {
    /// ```doxygen
    /// @brief Defines a new parameter of the given type in a describe action
    ///
    ///   \arg \c paramSet    handle to the parameter set descriptor that will hold this parameter
    ///   \arg \c paramType   type of the parameter to create, one of the kOfxParamType* #defines
    ///   \arg \c name        unique name of the parameter
    ///   \arg \c propertySet if not null, a pointer to the parameter descriptor's property set will be placed here.
    ///
    ///   This function defines a parameter in a parameter set and returns a property set which is used to describe that parameter.
    ///
    ///   This function does not actually create a parameter, it only says that one should exist in any subsequent instances. To fetch an
    ///   parameter instance paramGetHandle must be called on an instance.
    ///
    ///   This function can always be called in one of a plug-in's "describe" functions which defines the parameter sets common to all instances of a plugin.
    ///
    /// @returns
    ///   - ::kOfxStatOK             - the parameter was created correctly
    ///   - ::kOfxStatErrBadHandle   - if the plugin handle was invalid
    ///   - ::kOfxStatErrExists      - if a parameter of that name exists already in this plugin
    ///   - ::kOfxStatErrUnknown     - if the type is unknown
    ///   - ::kOfxStatErrUnsupported - if the type is known but unsupported
    /// ```
    pub paramDefine: ::std::option::Option<
        unsafe extern "C" fn(
            paramSet: OfxParamSetHandle,
            paramType: *const ::std::os::raw::c_char,
            name: *const ::std::os::raw::c_char,
            propertySet: *mut OfxPropertySetHandle,
        ) -> OfxStatus,
    >,
    /// ```doxygen
    /// @brief Retrieves the handle for a parameter in a given parameter set
    ///
    ///   \arg \c paramSet    instance of the plug-in to fetch the property handle from
    ///   \arg \c name        parameter to ask about
    ///   \arg \c param       pointer to a param handle, the value is returned here
    ///   \arg \c propertySet if not null, a pointer to the parameter's property set will be placed here.
    ///
    ///   Parameter handles retrieved from an instance are always distinct in each instance. The parameter handle is valid for the life-time of the instance. Parameter handles in instances are distinct from parameter handles in plugins. You cannot call this in a plugin's describe function, as it needs an instance to work on.
    ///
    /// @returns
    ///   - ::kOfxStatOK       - the parameter was found and returned
    ///   - ::kOfxStatErrBadHandle  - if the plugin handle was invalid
    ///   - ::kOfxStatErrUnknown    - if the type is unknown
    /// ```
    pub paramGetHandle: ::std::option::Option<
        unsafe extern "C" fn(
            paramSet: OfxParamSetHandle,
            name: *const ::std::os::raw::c_char,
            param: *mut OfxParamHandle,
            propertySet: *mut OfxPropertySetHandle,
        ) -> OfxStatus,
    >,
    /// ```doxygen
    /// @brief Retrieves the property set handle for the given parameter set
    ///
    ///   \arg \c paramSet    parameter set to get the property set for
    ///   \arg \c propHandle  pointer to a the property set handle, value is returedn her
    ///
    ///   \note The property handle belonging to a parameter set is the same as the property handle belonging to the plugin instance.
    ///
    /// @returns
    ///   - ::kOfxStatOK       - the property set was found and returned
    ///   - ::kOfxStatErrBadHandle  - if the parameter handle was invalid
    ///   - ::kOfxStatErrUnknown    - if the type is unknown
    /// ```
    pub paramSetGetPropertySet: ::std::option::Option<
        unsafe extern "C" fn(
            paramSet: OfxParamSetHandle,
            propHandle: *mut OfxPropertySetHandle,
        ) -> OfxStatus,
    >,
    /// ```doxygen
    /// @brief Retrieves the property set handle for the given parameter
    ///
    ///   \arg \c param       parameter to get the property set for
    ///   \arg \c propHandle  pointer to a the property set handle, value is returedn her
    ///
    ///   The property handle is valid for the lifetime of the parameter, which is the lifetime of the instance that owns the parameter
    ///
    /// @returns
    ///   - ::kOfxStatOK       - the property set was found and returned
    ///   - ::kOfxStatErrBadHandle  - if the parameter handle was invalid
    ///   - ::kOfxStatErrUnknown    - if the type is unknown
    /// ```
    pub paramGetPropertySet: ::std::option::Option<
        unsafe extern "C" fn(
            param: OfxParamHandle,
            propHandle: *mut OfxPropertySetHandle,
        ) -> OfxStatus,
    >,
    /// ```doxygen
    /// @brief Gets the current value of a parameter,
    ///
    ///   \arg \c paramHandle parameter handle to fetch value from
    ///   \arg \c ...         one or more pointers to variables of the relevant type to hold the parameter's value
    ///
    ///   This gets the current value of a parameter. The varargs ... argument needs to be pointer to C variables
    ///   of the relevant type for this parameter. Note that params with multiple values (eg Colour) take
    ///   multiple args here. For example...
    ///
    ///   @verbatim
    ///   OfxParamHandle myDoubleParam, *myColourParam;
    ///   ofxHost->paramGetHandle(instance, "myDoubleParam", &myDoubleParam);
    ///   double myDoubleValue;
    ///   ofxHost->paramGetValue(myDoubleParam, &myDoubleValue);
    ///   ofxHost->paramGetHandle(instance, "myColourParam", &myColourParam);
    ///   double myR, myG, myB;
    ///   ofxHost->paramGetValue(myColourParam, &myR, &myG, &myB);
    ///   @endverbatim
    ///
    ///   \note \c paramGetValue should only be called from within a ::kOfxActionInstanceChanged or interact action and never from the render actions (which should always use paramGetValueAtTime).
    ///
    /// @returns
    ///   - ::kOfxStatOK       - all was OK
    ///   - ::kOfxStatErrBadHandle  - if the parameter handle was invalid
    /// ```
    pub paramGetValue: ::std::option::Option<
        unsafe extern "C" fn(paramHandle: OfxParamHandle, ...) -> OfxStatus,
    >,
    /// ```doxygen
    /// @brief Gets the value of a parameter at a specific time.
    ///
    ///   \arg \c paramHandle parameter handle to fetch value from
    ///   \arg \c time        at what point in time to look up the parameter
    ///   \arg \c ...         one or more pointers to variables of the relevant type to hold the parameter's value
    ///
    ///   This gets the current value of a parameter. The varargs needs to be pointer to C variables
    ///   of the relevant type for this parameter. See OfxParameterSuiteV1::paramGetValue for notes on
    ///   the varags list
    ///
    /// @returns
    ///   - ::kOfxStatOK       - all was OK
    ///   - ::kOfxStatErrBadHandle  - if the parameter handle was invalid
    /// ```
    pub paramGetValueAtTime: ::std::option::Option<
        unsafe extern "C" fn(
            paramHandle: OfxParamHandle,
            time: OfxTime,
            ...
        ) -> OfxStatus,
    >,
    /// ```doxygen
    /// @brief Gets the derivative of a parameter at a specific time.
    ///
    ///   \arg \c paramHandle parameter handle to fetch value from
    ///   \arg \c time        at what point in time to look up the parameter
    ///   \arg \c ...         one or more pointers to variables of the relevant type to hold the parameter's derivative
    ///
    ///   This gets the derivative of the parameter at the indicated time.
    ///
    ///   The varargs needs to be pointer to C variables
    ///   of the relevant type for this parameter. See OfxParameterSuiteV1::paramGetValue for notes on
    ///   the varags list.
    ///
    ///   Only double and colour params can have their derivatives found.
    ///
    /// @returns
    ///   - ::kOfxStatOK       - all was OK
    ///   - ::kOfxStatErrBadHandle  - if the parameter handle was invalid
    /// ```
    pub paramGetDerivative: ::std::option::Option<
        unsafe extern "C" fn(
            paramHandle: OfxParamHandle,
            time: OfxTime,
            ...
        ) -> OfxStatus,
    >,
    /// ```doxygen
    /// @brief Gets the integral of a parameter over a specific time range,
    ///
    ///   \arg \c paramHandle parameter handle to fetch integral from
    ///   \arg \c time1       where to start evaluating the integral
    ///   \arg \c time2       where to stop evaluating the integral
    ///   \arg \c ...         one or more pointers to variables of the relevant type to hold the parameter's integral
    ///
    ///   This gets the integral of the parameter over the specified time range.
    ///
    ///   The varargs needs to be pointer to C variables
    ///   of the relevant type for this parameter. See OfxParameterSuiteV1::paramGetValue for notes on
    ///   the varags list.
    ///
    ///   Only double and colour params can be integrated.
    ///
    /// @returns
    ///   - ::kOfxStatOK       - all was OK
    ///   - ::kOfxStatErrBadHandle  - if the parameter handle was invalid
    /// ```
    pub paramGetIntegral: ::std::option::Option<
        unsafe extern "C" fn(
            paramHandle: OfxParamHandle,
            time1: OfxTime,
            time2: OfxTime,
            ...
        ) -> OfxStatus,
    >,
    /// ```doxygen
    /// @brief Sets the current value of a parameter
    ///
    ///   \arg \c paramHandle parameter handle to set value in
    ///   \arg \c ...         one or more variables of the relevant type to hold the parameter's value
    ///
    ///   This sets the current value of a parameter. The varargs ... argument needs to be values
    ///   of the relevant type for this parameter. Note that params with multiple values (eg Colour) take
    ///   multiple args here. For example...
    ///   @verbatim
    ///   ofxHost->paramSetValue(instance, "myDoubleParam", double(10));
    ///   ofxHost->paramSetValue(instance, "myColourParam", double(pix.r), double(pix.g), double(pix.b));
    ///   @endverbatim
    ///
    ///   \note \c paramSetValue should only be called from within a ::kOfxActionInstanceChanged or interact action.
    ///
    /// @returns
    ///   - ::kOfxStatOK       - all was OK
    ///   - ::kOfxStatErrBadHandle  - if the parameter handle was invalid
    /// ```
    pub paramSetValue: ::std::option::Option<
        unsafe extern "C" fn(paramHandle: OfxParamHandle, ...) -> OfxStatus,
    >,
    /// ```doxygen
    /// @brief Keyframes the value of a parameter at a specific time.
    ///
    ///   \arg \c paramHandle parameter handle to set value in
    ///   \arg \c time        at what point in time to set the keyframe
    ///   \arg \c ...         one or more variables of the relevant type to hold the parameter's value
    ///
    ///   This sets a keyframe in the parameter at the indicated time to have the indicated value.
    ///   The varargs ... argument needs to be values of the relevant type for this parameter. See the note on
    ///   OfxParameterSuiteV1::paramSetValue for more detail
    ///
    ///   \note \c paramSetValueAtTime should only be called from within a ::kOfxActionInstanceChanged or interact action.
    ///
    ///   V1.3: This function can be called the ::kOfxActionInstanceChanged action and during image effect analysis render passes.
    ///   V1.4: This function can be called the ::kOfxActionInstanceChanged action
    /// @returns
    ///   - ::kOfxStatOK       - all was OK
    ///   - ::kOfxStatErrBadHandle  - if the parameter handle was invalid
    /// ```
    /// ## Parameters
    /// ### Parameter `time`
    /// ```doxygen
    /// time in frames
    /// ```
    pub paramSetValueAtTime: ::std::option::Option<
        unsafe extern "C" fn(
            paramHandle: OfxParamHandle,
            time: OfxTime,
            ...
        ) -> OfxStatus,
    >,
    /// ```doxygen
    /// @brief Returns the number of keyframes in the parameter
    ///
    ///   \arg \c paramHandle parameter handle to interrogate
    ///   \arg \c numberOfKeys pointer to integer where the return value is placed
    ///
    ///   V1.3: This function can be called the ::kOfxActionInstanceChanged action and during image effect analysis render passes.
    ///   V1.4: This function can be called the ::kOfxActionInstanceChanged action
    ///
    ///   Returns the number of keyframes in the parameter.
    ///
    /// @returns
    ///   - ::kOfxStatOK       - all was OK
    ///   - ::kOfxStatErrBadHandle  - if the parameter handle was invalid
    /// ```
    pub paramGetNumKeys: ::std::option::Option<
        unsafe extern "C" fn(
            paramHandle: OfxParamHandle,
            numberOfKeys: *mut ::std::os::raw::c_uint,
        ) -> OfxStatus,
    >,
    /// ```doxygen
    /// @brief Returns the time of the nth key
    ///
    ///   \arg \c paramHandle parameter handle to interrogate
    ///   \arg \c nthKey      which key to ask about (0 to paramGetNumKeys -1), ordered by time
    ///   \arg \c time	   pointer to OfxTime where the return value is placed
    ///
    /// @returns
    ///   - ::kOfxStatOK       - all was OK
    ///   - ::kOfxStatErrBadHandle  - if the parameter handle was invalid
    ///   - ::kOfxStatErrBadIndex   - the nthKey does not exist
    /// ```
    pub paramGetKeyTime: ::std::option::Option<
        unsafe extern "C" fn(
            paramHandle: OfxParamHandle,
            nthKey: ::std::os::raw::c_uint,
            time: *mut OfxTime,
        ) -> OfxStatus,
    >,
    /// ```doxygen
    /// @brief Finds the index of a keyframe at/before/after a specified time.
    ///
    ///   \arg \c paramHandle parameter handle to search
    ///   \arg \c time        what time to search from
    ///   \arg \c direction
    ///     - == 0 indicates search for a key at the indicated time (some small delta)
    ///     - > 0 indicates search for the next key after the indicated time
    ///     - < 0 indicates search for the previous key before the indicated time
    ///   \arg \c index	   pointer to an integer which in which the index is returned set to -1 if no key was found
    ///
    /// @returns
    ///   - ::kOfxStatOK            - all was OK
    ///   - ::kOfxStatFailed        - if the search failed to find a key
    ///   - ::kOfxStatErrBadHandle  - if the parameter handle was invalid
    /// ```
    pub paramGetKeyIndex: ::std::option::Option<
        unsafe extern "C" fn(
            paramHandle: OfxParamHandle,
            time: OfxTime,
            direction: ::std::os::raw::c_int,
            index: *mut ::std::os::raw::c_int,
        ) -> OfxStatus,
    >,
    /// ```doxygen
    /// @brief Deletes a keyframe if one exists at the given time.
    ///
    ///   \arg \c paramHandle parameter handle to delete the key from
    ///   \arg \c time        time at which a keyframe is
    ///
    /// @returns
    ///   - ::kOfxStatOK       - all was OK
    ///   - ::kOfxStatErrBadHandle  - if the parameter handle was invalid
    ///   - ::kOfxStatErrBadIndex   - no key at the given time
    /// ```
    pub paramDeleteKey: ::std::option::Option<
        unsafe extern "C" fn(paramHandle: OfxParamHandle, time: OfxTime) -> OfxStatus,
    >,
    /// ```doxygen
    /// @brief Deletes all keyframes from a parameter.
    ///
    ///   \arg \c paramHandle parameter handle to delete the keys from
    ///   \arg \c name        parameter to delete the keyframes from is
    ///
    ///   V1.3: This function can be called the ::kOfxActionInstanceChanged action and during image effect analysis render passes.
    ///   V1.4: This function can be called the ::kOfxActionInstanceChanged action
    ///
    /// @returns
    ///   - ::kOfxStatOK       - all was OK
    ///   - ::kOfxStatErrBadHandle  - if the parameter handle was invalid
    /// ```
    pub paramDeleteAllKeys: ::std::option::Option<
        unsafe extern "C" fn(paramHandle: OfxParamHandle) -> OfxStatus,
    >,
    /// ```doxygen
    /// @brief Copies one parameter to another, including any animation etc...
    ///
    ///   \arg \c paramTo     parameter to set
    ///   \arg \c paramFrom   parameter to copy from
    ///   \arg \c dstOffset   temporal offset to apply to keys when writing to the paramTo
    ///   \arg \c frameRange  if paramFrom has animation, and frameRange is not null, only this range of keys will be copied
    ///
    ///   This copies the value of \e paramFrom to \e paramTo, including any animation it may have. All the previous values in \e paramTo will be lost.
    ///
    ///   To choose all animation in \e paramFrom set \e frameRange to [0, 0]
    ///
    ///    V1.3: This function can be called the ::kOfxActionInstanceChanged action and during image effect analysis render passes.
    ///   V1.4: This function can be called the ::kOfxActionInstanceChanged action
    ///
    ///   \pre
    ///   - Both parameters must be of the same type.
    ///
    ///   \return
    ///   - ::kOfxStatOK       - all was OK
    ///   - ::kOfxStatErrBadHandle  - if the parameter handle was invalid
    /// ```
    pub paramCopy: ::std::option::Option<
        unsafe extern "C" fn(
            paramTo: OfxParamHandle,
            paramFrom: OfxParamHandle,
            dstOffset: OfxTime,
            frameRange: *const OfxRangeD,
        ) -> OfxStatus,
    >,
    /// ```doxygen
    /// @brief Used to group any parameter changes for undo/redo purposes
    ///
    ///   \arg \c paramSet    the parameter set in which this is happening
    ///   \arg \c name        label to attach to any undo/redo string UTF8
    ///
    ///   If a plugin calls paramSetValue/paramSetValueAtTime on one or more parameters, either from custom GUI interaction
    ///   or some analysis of imagery etc.. this is used to indicate the start of a set of a parameter
    ///   changes that should be considered part of a single undo/redo block.
    ///
    ///   \note \c paramEditBegin should only be called from within a ::kOfxActionInstanceChanged or interact action.
    ///
    ///   See also OfxParameterSuiteV1::paramEditEnd
    ///
    ///   \return
    ///   - ::kOfxStatOK       - all was OK
    ///   - ::kOfxStatErrBadHandle  - if the instance handle was invalid
    /// ```
    pub paramEditBegin: ::std::option::Option<
        unsafe extern "C" fn(
            paramSet: OfxParamSetHandle,
            name: *const ::std::os::raw::c_char,
        ) -> OfxStatus,
    >,
    /// ```doxygen
    /// @brief Used to group any parameter changes for undo/redo purposes
    ///
    ///   \arg \c paramSet    parameter set in which this is happening
    ///
    ///   If a plugin calls paramSetValue/paramSetValueAtTime on one or more parameters, either from custom GUI interaction
    ///   or some analysis of imagery etc.. this is used to indicate the end of a set of parameter
    ///   changes that should be considerred part of a single undo/redo block
    ///
    ///   \note \c paramEditEnd should only be called from within a ::kOfxActionInstanceChanged or interact action.
    ///
    ///   See also OfxParameterSuiteV1::paramEditBegin
    ///
    /// @returns
    ///   - ::kOfxStatOK       - all was OK
    ///   - ::kOfxStatErrBadHandle  - if the instance handle was invalid
    /// ```
    pub paramEditEnd: ::std::option::Option<
        unsafe extern "C" fn(paramSet: OfxParamSetHandle) -> OfxStatus,
    >,
}

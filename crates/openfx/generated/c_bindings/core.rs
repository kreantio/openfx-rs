/** @brief

 This action is the first action passed to a plug-in after the
 binary containing the plug-in has been loaded. It is there to allow a
 plug-in to create any global data structures it may need and is also
 when the plug-in should fetch suites from the host.

 The \ref handle, \ref inArgs and \ref outArgs arguments to the \ref mainEntry
 are redundant and should be set to NULL.



 \pre
 - The plugin's \ref OfxPlugin::setHost function has been called

 \post
 This action will not be called again while the binary containing the plug-in remains loaded.

 @returns
 -  \ref kOfxStatOK, the action was trapped and all was well,
 -  \ref kOfxStatReplyDefault, the action was ignored,
 -  \ref kOfxStatFailed, the load action failed, no further actions will be passed to the plug-in.
 Interpret if possible  kOfxStatFailed as plug-in indicating it does not want to load
 Do not create an entry in the host's UI for plug-in then.
 Plug-in also has the option to return 0 for OfxGetNumberOfPlugins or kOfxStatFailed if host supports OfxSetHost in which case kOfxActionLoad will never be called.
 -  \ref kOfxStatErrFatal, fatal error in the plug-in.

    @actiondef
    inArgs:
    outArgs:*/
pub const kOfxActionLoad: &::std::ffi::CStr = c"OfxActionLoad";
/** @brief

 The kOfxActionDescribe is the second action passed to a plug-in. It is
 where a plugin defines how it behaves and the resources it needs to
 function.

 Note that the handle passed in acts as a descriptor for, rather than an
 instance of the plugin. The handle is global and unique. The plug-in is
 at liberty to cache the handle away for future reference until the
 plug-in is unloaded.

 Most importantly, the effect must set what image effect contexts it is
 capable of working in.

 This action *must* be trapped, it is not optional.


 @param handle handle to the plug-in descriptor, cast to an \ref OfxImageEffectHandle
 @param inArgs is redundant and is set to NULL
 @param outArgs is redundant and is set to NULL

 \pre
     - \ref kOfxActionLoad has been called

 \post
     -  \ref kOfxActionDescribe will not be called again, unless it fails and
     returns one of the error codes where the host is allowed to attempt
     the action again
     -  the handle argument, being the global plug-in description handle, is
     a valid handle from the end of a successful describe action until the
     end of the \ref kOfxActionUnload action (ie: the plug-in can cache it away
     without worrying about it changing between actions).
     -  \ref kOfxImageEffectActionDescribeInContext
     will be called once for each context that the host and plug-in
     mutually support.  If a plug-in does not report to support any context supported by host,
	 host should not enable the plug-in.

 @returns
     -  \ref kOfxStatOK, the action was trapped and all was well
     -  \ref kOfxStatErrMissingHostFeature, in which the plugin will be unloaded
     and ignored, plugin may post message
     -  \ref kOfxStatErrMemory, in which case describe may be called again after a
     memory purge
     -  \ref kOfxStatFailed, something wrong, but no error code appropriate,
     plugin to post message
     -  \ref kOfxStatErrFatal


    @actiondef
    inArgs:
    outArgs:*/
pub const kOfxActionDescribe: &::std::ffi::CStr = c"OfxActionDescribe";
/** @brief

 This action is the last action passed to the plug-in before the
 binary containing the plug-in is unloaded. It is there to allow a
 plug-in to destroy any global data structures it may have created.

 The handle, inArgs and outArgs arguments to the main entry
 are redundant and should be set to NULL.

 \pre
     -  the \ref kOfxActionLoad action has been called
     -  all instances of a plugin have been destroyed

 \post
     - No other actions will be called.

 @returns
     -  \ref kOfxStatOK, the action was trapped all was well
     -  \ref kOfxStatReplyDefault, the action was ignored
     -  \ref kOfxStatErrFatal, in which case we the program will be forced to quit


    @actiondef
    inArgs:
    outArgs:*/
pub const kOfxActionUnload: &::std::ffi::CStr = c"OfxActionUnload";
/** @brief

 This action is an action that may be passed to a plug-in
 instance from time to time in low memory situations. Instances receiving
 this action should destroy any data structures they may have and release
 the associated memory, they can later reconstruct this from the effect's
 parameter set and associated information.

 For Image Effects, it is generally a bad idea to call this after each
 render, but rather it should be called after
 \ref kOfxImageEffectActionEndSequenceRender
 Some effects, typically those flagged with the
 \ref kOfxImageEffectInstancePropSequentialRender
 property, may need to cache information from previously rendered frames
 to function correctly, or have data structures that are expensive to
 reconstruct at each frame (eg: a particle system). Ideally, such effect
 should free such structures during the
 \ref kOfxImageEffectActionEndSequenceRender action.

 @param  handle handle to the plug-in instance, cast to an \ref OfxImageEffectHandle
 @param  inArgs is redundant and is set to NULL
 @param  outArgs is redundant and is set to NULL

 \pre
     -  \ref kOfxActionCreateInstance has been called on the instance handle,

 @returns
     -  \ref kOfxStatOK, the action was trapped and all was well
     -  \ref kOfxStatReplyDefault, the action was ignored
     -  \ref kOfxStatErrFatal,
     -  \ref kOfxStatFailed, something went wrong, but no error code appropriate,
     the plugin should to post a message

    @actiondef
    inArgs:
    outArgs:*/
pub const kOfxActionPurgeCaches: &::std::ffi::CStr = c"OfxActionPurgeCaches";
/** @brief

 This action is called when a plugin should synchronise any private data
 structures to its parameter set. This generally occurs when an effect is
 about to be saved or copied, but it could occur in other situations as
 well.

 @param  handle handle to the plug-in instance, cast to an \ref OfxImageEffectHandle
 @param  inArgs is redundant and is set to NULL
 @param  outArgs is redundant and is set to NULL

 \pre
     - \ref kOfxActionCreateInstance has been called on the instance handle,

 \post
     -  Any private state data can be reconstructed from the parameter set,

 @returns
     -  \ref kOfxStatOK, the action was trapped and all was well
     -  \ref kOfxStatReplyDefault, the action was ignored
     -  \ref kOfxStatErrFatal,
     -  \ref kOfxStatFailed, something went wrong, but no error code appropriate,
     the plugin should to post a message

    @actiondef
    inArgs:
    outArgs:*/
pub const kOfxActionSyncPrivateData: &::std::ffi::CStr = c"OfxActionSyncPrivateData";
/** @brief

 Called when an instance of a plugin is created by the host. The plugin can
 create any per-instance data structures it may need. Plugins may also
 enable/disable their params here, based on other param values or clips.

 When a host creates a plugin instance from a saved project with a connected
 clip, the host must either:

  - connect all params and clips to an effect *before* calling createInstance,
    and support the \ref OfxTimeLineSuite so the effect can call
    \ref OfxTimeLineSuiteV1::getTime in createInstance (so the plugin can call
    \ref OfxImageEffectSuiteV1::clipGetRegionOfDefinition),

OR:

  - call `instanceChanged` after `createInstance` and after the host has
    hooked up all clips, with property \ref kOfxPropType = \ref kOfxTypeClip and
    \ref kOfxPropChangeReason = \ref kOfxChangeUserEdited, and ensure that
    \ref OfxImageEffectSuiteV1::clipGetRegionOfDefinition works in that action.

 @param  handle handle to the plug-in instance, cast to an \ref OfxImageEffectHandle
 @param  inArgs is redundant and is set to NULL
 @param  outArgs is redundant and is set to NULL

 \pre
     -  \ref kOfxActionDescribe has been called
     -  the instance is fully constructed, with all objects requested in the
     describe actions (eg, parameters and clips) have been constructed and
     have had their initial values set. This means that if the values are
     being loaded from an old setup, that load should have taken place
     before the create instance action is called.

 \post
     -  the instance pointer will be valid until the
     \ref kOfxActionDestroyInstance
     action is passed to the plug-in with the same instance handle

 @returns
     -  \ref kOfxStatOK, the action was trapped and all was well
     -  \ref kOfxStatReplyDefault, the action was ignored, but all was well anyway
     -  \ref kOfxStatErrFatal
     -  \ref kOfxStatErrMemory, in which case this may be called again after a
     memory purge
     -  \ref kOfxStatFailed, something went wrong, but no error code appropriate,
     the plugin should to post a message if possible and the host should
     destroy the instanace handle and not attempt to proceed further

    @actiondef
    inArgs:
    outArgs:*/
pub const kOfxActionCreateInstance: &::std::ffi::CStr = c"OfxActionCreateInstance";
/** @brief


 This action is the last passed to a plug-in's instance before its
 destruction. It is there to allow a plugin to destroy any per-instance
 data structures it may have created.

 @param  handle
 handle to the plug-in instance, cast to an \ref OfxImageEffectHandle
 @param  inArgs is redundant and is set to NULL
 @param  outArgs is redundant and is set to NULL

 \pre
     -  \ref kOfxActionCreateInstance
     has been called on the handle,
     -  the instance has not had any of its members destroyed yet,

 \post
     -  the instance pointer is no longer valid and any operation on it will
     be undefined

 @returns
     To some extent, what is returned is moot, a bit like throwing an
     exception in a C++ destructor, so the host should continue destruction
     of the instance regardless.

     -  \ref kOfxStatOK, the action was trapped and all was well,
     -  \ref kOfxStatReplyDefault, the action was ignored as the effect had nothing
     to do,
     -  \ref kOfxStatErrFatal,
     -  \ref kOfxStatFailed, something went wrong, but no error code appropriate,
     the plugin should to post a message.


    @actiondef
    inArgs:
    outArgs:*/
pub const kOfxActionDestroyInstance: &::std::ffi::CStr = c"OfxActionDestroyInstance";
/** @brief

 This action signals that something has changed in a plugin's instance,
 either by user action, the host or the plugin itself. All change actions
 are bracketed by a pair of \ref kOfxActionBeginInstanceChanged and
 \ref kOfxActionEndInstanceChanged actions. The ``inArgs`` property set is
 used to determine what was the thing inside the instance that was
 changed.

 Note: when a host loads a plugin instance from a saved project, if the input
 clips are not connected at that time, the host must call `instanceChanged`
 after \ref kOfxActionCreateInstance and after the host has hooked up all clips, with
 property \ref kOfxPropType = \ref kOfxTypeClip and \ref kOfxPropChangeReason =
 \ref kOfxChangePluginEdited, and ensure that \ref OfxImageEffectSuiteV1::clipGetRegionOfDefinition works in
 this action.

 @param  handle handle to the plug-in instance, cast to an \ref OfxImageEffectHandle
 @param  inArgs has the following properties
     - \ref  kOfxPropType The type of the thing that changed which will be one of..

     - \ref kOfxTypeParameter Indicating a parameter's value has changed
     in some way
     -  \ref kOfxTypeClip A clip to an image effect has changed in some
     way (for Image Effect Plugins only)

     -  \ref kOfxPropName the name of the thing that was changed in the instance
     -  \ref kOfxPropChangeReason what triggered the change, which will be one of...

     -  \ref kOfxChangeUserEdited - the user or host changed the instance
     somehow and caused a change to something, this includes
     undo/redos, resets and loading values from files or presets,

     -  \ref kOfxChangePluginEdited - the plugin itself has changed the
     value of the instance in some action
     -  \ref kOfxChangeTime - the time has changed and this has affected the
     value of the object because it varies over time

     -  \ref kOfxPropTime
     - the effect time at which the chang occurred (for Image Effect Plugins only)
     -  \ref kOfxImageEffectPropRenderScale
     - the render scale currently being applied to any image fetched
     from a clip (for Image Effect Plugins only)
     - \ref kOfxImageEffectPropThumbnailRender (optional) if the host considers this render a "thumbnail"

 @param  outArgs is redundant and is set to NULL

 \pre
     -  \ref kOfxActionCreateInstance has been called on the instance handle,
     -  \ref kOfxActionBeginInstanceChanged has been called on the instance
     handle.

 \post
     -  \ref kOfxActionEndInstanceChanged will be called on the instance handle.

 @returns
     -  \ref kOfxStatOK, the action was trapped and all was well
     -  \ref kOfxStatReplyDefault, the action was ignored
     -  \ref kOfxStatErrFatal,
     -  \ref kOfxStatFailed, something went wrong, but no error code appropriate,
     the plugin should to post a message


    @actiondef
    inArgs:
      - OfxPropType
      - OfxPropName
      - OfxPropChangeReason
      - OfxPropTime
      - OfxImageEffectPropRenderScale
      - OfxImageEffectPropThumbnailRender
    outArgs:*/
pub const kOfxActionInstanceChanged: &::std::ffi::CStr = c"OfxActionInstanceChanged";
/** @brief

 The \ref kOfxActionBeginInstanceChanged and \ref kOfxActionEndInstanceChanged actions
 are used to bracket all \ref kOfxActionInstanceChanged actions, whether a
 single change or multiple changes. Some changes to a plugin instance can
 be grouped logically (eg: a 'reset all' button resetting all the
 instance's parameters), the begin/end instance changed actions allow a
 plugin to respond appropriately to a large set of changes. For example,
 a plugin that maintains a complex internal state can delay any changes
 to that state until all parameter changes have completed.

 @param  handle
 handle to the plug-in instance, cast to an \ref OfxImageEffectHandle
 @param  inArgs has the following properties
     -  \ref kOfxPropChangeReason what triggered the change, which will be one of...
     -  \ref kOfxChangeUserEdited - the user or host changed the instance
     somehow and caused a change to something, this includes
     undo/redos, resets and loading values from files or presets,
     -  \ref kOfxChangePluginEdited - the plugin itself has changed the
     value of the instance in some action
     -  \ref kOfxChangeTime - the time has changed and this has affected the
     value of the object because it varies over time

 @param  outArgs is redundant and is set to NULL

 \post
     - For \ref kOfxActionBeginInstanceChanged , \ref kOfxActionCreateInstance has been called on the instance handle.
     - For \ref kOfxActionEndInstanceChanged , \ref kOfxActionBeginInstanceChanged has been called on the instance handle.
     - \ref kOfxActionCreateInstance has been called on the instance handle.

 \post
     - For \ref kOfxActionBeginInstanceChanged, \ref kOfxActionInstanceChanged will be called at least once on the instance handle.
     - \ref kOfxActionEndInstanceChanged will be called on the instance handle.

 @returns
     -  \ref kOfxStatOK, the action was trapped and all was well
     -  \ref kOfxStatReplyDefault, the action was ignored
     -  \ref kOfxStatErrFatal,
     -  \ref kOfxStatFailed, something went wrong, but no error code appropriate,
     the plugin should to post a message

    @actiondef
    inArgs:
      - OfxPropChangeReason
      - OfxImageEffectPropThumbnailRender
    outArgs: []*/
pub const kOfxActionBeginInstanceChanged: &::std::ffi::CStr = c"OfxActionBeginInstanceChanged";
/** @brief Action called after the end of a set of \ref kOfxActionEndInstanceChanged actions, used with ::kOfxActionBeginInstanceChanged to bracket a grouped set of changes,  see \ref kOfxActionBeginInstanceChanged
    @actiondef
    inArgs:
      - OfxPropChangeReason
    outArgs:*/
pub const kOfxActionEndInstanceChanged: &::std::ffi::CStr = c"OfxActionEndInstanceChanged";
/** @brief

 This is called when an instance is *first* actively edited by a user,
 ie: and interface is open and parameter values and input clips can be
 modified. It is there so that effects can create private user interface
 structures when necassary. Note that some hosts can have multiple
 editors open on the same effect instance simulateously.


 @param  handle handle to the plug-in instance, cast to an \ref OfxImageEffectHandle
 @param  inArgs is redundant and is set to NULL
 @param  outArgs is redundant and is set to NULL

 \pre
     -  \ref kOfxActionCreateInstance has been called on the instance handle,

 \post
     -  \ref kOfxActionEndInstanceEdit will be called when the last editor is
     closed on the instance

 @returns
     -  \ref kOfxStatOK, the action was trapped and all was well
     -  \ref kOfxStatReplyDefault, the action was ignored
     -  \ref kOfxStatErrFatal,
     -  \ref kOfxStatFailed, something went wrong, but no error code appropriate,
     the plugin should to post a message

    @actiondef
    inArgs:
    outArgs:*/
pub const kOfxActionBeginInstanceEdit: &::std::ffi::CStr = c"OfxActionBeginInstanceEdit";
/** @brief

 This is called when the *last* user interface on an instance closed. It
 is there so that effects can destroy private user interface structures
 when necassary. Note that some hosts can have multiple editors open on
 the same effect instance simulateously, this will only be called when
 the last of those editors are closed.

 @param  handle handle to the plug-in instance, cast to an \ref OfxImageEffectHandle
 @param  inArgs is redundant and is set to NULL
 @param  outArgs is redundant and is set to NULL

 \pre
     -  \ref kOfxActionBeginInstanceEdit has been called on the instance handle,

 \post
     -  no user interface is open on the instance

 @returns
     -  \ref kOfxStatOK, the action was trapped and all was well
     -  \ref kOfxStatReplyDefault, the action was ignored
     -  \ref kOfxStatErrFatal,
     -  \ref kOfxStatFailed, something went wrong, but no error code appropriate,
     the plugin should to post a message

    @actiondef
    inArgs:
    outArgs:*/
pub const kOfxActionEndInstanceEdit: &::std::ffi::CStr = c"OfxActionEndInstanceEdit";
/** @brief Property on the host descriptor, saying what API version of the API is being implemented

This is a version string that will specify which version of the API is being implemented by a host. It
can have multiple values. For example "1.0", "1.2.4" etc.....

If this is not present, it is safe to assume that the version of the API is "1.0".

    @propdef
    type: int
    dimension: N*/
pub const kOfxPropAPIVersion: &::std::ffi::CStr = c"OfxPropAPIVersion";
/** @brief General property used to get/set the time of something.

    @propdef
    type: double
    dimension: 1*/
pub const kOfxPropTime: &::std::ffi::CStr = c"OfxPropTime";
/** @brief Indicates if a host is actively editing the effect with some GUI.

If false, the effect currently has no interface.  This may be because the effect is loaded in a background render host, or it may be loaded on an interactive host that has not yet opened an editor for the effect.

The output of an effect should only ever depend on the state of its parameters, not on the interactive flag. The interactive flag is more a courtesy flag to let a plugin know that it has an interface. If a plugin wants to have its behaviour depend on the interactive flag, it should make a secret parameter which shadows the state of the flag.

    @propdef
    type: bool
    dimension: 1*/
pub const kOfxPropIsInteractive: &::std::ffi::CStr = c"OfxPropIsInteractive";
/** @brief The file path to the plugin.

This is a string that indicates the file path where the plug-in was found by the host. The path is in the native
path format for the host OS (eg:  UNIX directory separators are forward slashes, Windows ones are backslashes).

The path is to the bundle location, see \ref InstallationLocation.
eg:  '/usr/OFX/Plugins/AcmePlugins/AcmeFantasticPlugin.ofx.bundle'

    @propdef
    type: string
    dimension: 1*/
pub const kOfxPluginPropFilePath: &::std::ffi::CStr = c"OfxPluginPropFilePath";
/** @brief  A private data pointer that the plug-in can store its own data behind.

This data pointer is unique to each plug-in instance, so two instances of the same plug-in do not share the same data pointer. Use it to hang any needed private data structures.

    @propdef
    type: pointer
    dimension: 1*/
pub const kOfxPropInstanceData: &::std::ffi::CStr = c"OfxPropInstanceData";
/** @brief General property, used to identify the kind of an object behind a handle

    - Valid Values - currently this can be...
       - ::kOfxTypeImageEffectHost
       - ::kOfxTypeImageEffect
       - ::kOfxTypeImageEffectInstance
       - ::kOfxTypeParameter
       - ::kOfxTypeParameterInstance
       - ::kOfxTypeClip
       - ::kOfxTypeImage
    @propdef
    type: string
    dimension: 1*/
pub const kOfxPropType: &::std::ffi::CStr = c"OfxPropType";
/** @brief Unique name of an object.

This property is used to label objects uniquely among objects of that type. It is typically set when a plugin creates a new object with a function that takes a name.

    @propdef
    type: string
    dimension: 1*/
pub const kOfxPropName: &::std::ffi::CStr = c"OfxPropName";
/** @brief Identifies a specific version of a host or plugin.

This is a multi dimensional integer property that represents the version of a host (host descriptor), or plugin (plugin descriptor). These represent a version number of the form '1.2.3.4', with each dimension adding another 'dot' on the right.

A version is considered to be more recent than another if its ordered set of values is lexicographically greater than another, reading left to right. (ie: 1.2.4 is smaller than 1.2.6). Also, if the number of dimensions is different, then the values of the missing dimensions are considered to be zero (so 1.2.4 is greater than 1.2).

    - Valid Values - positive integers
    @propdef
    type: int
    dimension: N*/
pub const kOfxPropVersion: &::std::ffi::CStr = c"OfxPropVersion";
/** @brief Unique user readable version string of a plugin or host.

This is purely for user feedback, a plugin or host should use ::kOfxPropVersion if they need
to check for specific versions.

    - Valid Values - ASCII string
    @propdef
    type: string
    dimension: 1*/
pub const kOfxPropVersionLabel: &::std::ffi::CStr = c"OfxPropVersionLabel";
/** @brief Description of the plug-in to a user.

This is a string giving a potentially verbose description of the effect.

    - Valid Values - UTF8 string
    @propdef
    type: string
    dimension: 1*/
pub const kOfxPropPluginDescription: &::std::ffi::CStr = c"OfxPropPluginDescription";
/** @brief User visible name of an object.

The label is what a user sees on any interface in place of the object's name.

Note that resetting this will also reset ::kOfxPropShortLabel and ::kOfxPropLongLabel.

    @propdef
    type: string
    dimension: 1*/
pub const kOfxPropLabel: &::std::ffi::CStr = c"OfxPropLabel";
/** @brief If set this tells the host to use an icon instead of a label for some object in the interface.

The value is a path is defined relative to the Resource folder that points to an SVG or PNG file containing the icon.

The first dimension, if set, will the name of and SVG file, the second a PNG file.

    - Valid Values - ASCII string
    @propdef
    type: string
    dimension: 2
    hostOptional: true*/
pub const kOfxPropIcon: &::std::ffi::CStr = c"OfxPropIcon";
/** @brief Short user visible name of an object.

This is a shorter version of the label, typically 13 character glyphs or less. Hosts should use this if they have limited display space for their object labels.

    @propdef
    type: string
    dimension: 1
    hostOptional: true*/
pub const kOfxPropShortLabel: &::std::ffi::CStr = c"OfxPropShortLabel";
/** @brief Long user visible name of an object.

This is a longer version of the label, typically 32 character glyphs or so. Hosts should use this if they have mucg display space for their object labels.

    @propdef
    type: string
    dimension: 1
    hostOptional: true*/
pub const kOfxPropLongLabel: &::std::ffi::CStr = c"OfxPropLongLabel";
/** @brief Indicates why a plug-in changed.

Argument property for the ::kOfxActionInstanceChanged action.

    - Valid Values - this can be...
       - ::kOfxChangeUserEdited - the user directly edited the instance somehow and caused a change to something, this includes undo/redos and resets
       - ::kOfxChangePluginEdited - the plug-in itself has changed the value of the object in some action
       - ::kOfxChangeTime - the time has changed and this has affected the value of the object because it varies over time
    @propdef
    type: enum
    dimension: 1
    values:
      - OfxChangeUserEdited
      - OfxChangePluginEdited
      - OfxChangeTime*/
pub const kOfxPropChangeReason: &::std::ffi::CStr = c"OfxPropChangeReason";
/** @brief A pointer to an effect instance.

This property is used to link an object to the effect. For example if the plug-in supplies an openGL overlay for an image effect,
the interact instance will have one of these so that the plug-in can connect back to the effect the GUI links to.

    @propdef
    type: pointer
    dimension: 1*/
pub const kOfxPropEffectInstance: &::std::ffi::CStr = c"OfxPropEffectInstance";
/** @brief A pointer to an operating system specific application handle.

Some plug-in vendor want raw OS specific handles back from the host so they can do interesting things with host OS APIs. Typically this is to control windowing properly on Microsoft Windows. This property returns the appropriate 'root' window handle on the current operating system. So on Windows this would be the hWnd of the application main window.

    @propdef
    type: pointer
    dimension: 1*/
pub const kOfxPropHostOSHandle: &::std::ffi::CStr = c"OfxPropHostOSHandle";
/// @brief String used as a value to ::kOfxPropChangeReason to indicate a user has changed something
pub const kOfxChangeUserEdited: &::std::ffi::CStr = c"OfxChangeUserEdited";
/// @brief String used as a value to ::kOfxPropChangeReason to indicate the plug-in itself has changed something
pub const kOfxChangePluginEdited: &::std::ffi::CStr = c"OfxChangePluginEdited";
/// @brief String used as a value to ::kOfxPropChangeReason to a time varying object has changed due to a time change
pub const kOfxChangeTime: &::std::ffi::CStr = c"OfxChangeTime";
/** @brief Used to flag infinite rects. Set minimums to this to indicate infinite

This is effectively INT_MAX.*/
pub const kOfxFlagInfiniteMax: u32 = 2147483647;
/** @brief Used to flag infinite rects. Set minimums to this to indicate infinite.

This is effectively INT_MIN*/
pub const kOfxFlagInfiniteMin: i32 = -2147483648;
/// @brief String used to label unset bitdepths
pub const kOfxBitDepthNone: &::std::ffi::CStr = c"OfxBitDepthNone";
/// @brief String used to label unsigned 8 bit integer samples
pub const kOfxBitDepthByte: &::std::ffi::CStr = c"OfxBitDepthByte";
/// @brief String used to label unsigned 16 bit integer samples
pub const kOfxBitDepthShort: &::std::ffi::CStr = c"OfxBitDepthShort";
/** @brief String used to label half-float (16 bit floating point) samples
 \version Added in Version 1.4. Was in ofxOpenGLRender.h before.

@brief String used to label the OpenGL half float (16 bit floating
point) sample format*/
pub const kOfxBitDepthHalf: &::std::ffi::CStr = c"OfxBitDepthHalf";
/// @brief String used to label signed 32 bit floating point samples
pub const kOfxBitDepthFloat: &::std::ffi::CStr = c"OfxBitDepthFloat";
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OfxPropertySetStruct {
    _unused: [u8; 0],
}
/// @brief Blind data structure to manipulate sets of properties through
pub type OfxPropertySetHandle = *mut OfxPropertySetStruct;
/// @brief OFX status return type
pub type OfxStatus = ::std::os::raw::c_int;
/** @brief Generic host structure passed to OfxPlugin::setHost function

This structure contains what is needed by a plug-in to bootstrap its connection
to the host.*/
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OfxHost {
    /** @brief Global handle to the host. Extract relevant host properties from this.
This pointer will be valid while the binary containing the plug-in is loaded.*/
    pub host: OfxPropertySetHandle,
    /** @brief The function which the plug-in uses to fetch suites from the host.

\arg \c host          the host the suite is being fetched from this \em must be the \e host member of the OfxHost struct containing fetchSuite.
\arg \c suiteName     ASCII string labelling the host supplied API
\arg \c suiteVersion  version of that suite to fetch

Any API fetched will be valid while the binary containing the plug-in is loaded.

Repeated calls to fetchSuite with the same parameters will return the same pointer.

It is recommended that hosts should return the same host and suite pointers to all plugins
in the same shared lib or bundle.

returns
- NULL if the API is unknown (either the api or the version requested),
- pointer to the relevant API if it was found*/
    pub fetchSuite: ::std::option::Option<
        unsafe extern "C" fn(
            host: OfxPropertySetHandle,
            suiteName: *const ::std::os::raw::c_char,
            suiteVersion: ::std::os::raw::c_int,
        ) -> *const ::std::os::raw::c_void,
    >,
}
/** @brief Entry point for plug-ins

\arg \c action   ASCII c string indicating which action to take
\arg \c instance object to which action should be applied, this will need to be cast to the appropriate blind data type depending on the \e action
\arg \c inData   handle that contains action specific properties
\arg \c outData  handle where the plug-in should set various action specific properties

This is how the host generally communicates with a plug-in. Entry points are used to pass messages
to various objects used within OFX. The main use is within the OfxPlugin struct.

The exact set of actions is determined by the plug-in API that is being implemented, however all plug-ins
can perform several actions. For the list of actions consult \ref ActionsAll.*/
pub type OfxPluginEntryPoint = ::std::option::Option<
    unsafe extern "C" fn(
        action: *const ::std::os::raw::c_char,
        handle: *const ::std::os::raw::c_void,
        inArgs: OfxPropertySetHandle,
        outArgs: OfxPropertySetHandle,
    ) -> OfxStatus,
>;
/** @brief The structure that defines a plug-in to a host.

 This structure is the first element in any plug-in structure
 using the OFX plug-in architecture. By examining its members
 a host can determine the API that the plug-in implements,
 the version of that API, its name and version.

 For details see \ref Architecture.
*/
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OfxPlugin {
    /** Defines the type of the plug-in, this will tell the host what the plug-in does. e.g.: an image
effects plug-in would be a "OfxImageEffectPlugin"*/
    pub pluginApi: *const ::std::os::raw::c_char,
    /// Defines the version of the pluginApi that this plug-in implements
    pub apiVersion: ::std::os::raw::c_int,
    /** String that uniquely labels the plug-in among all plug-ins that implement an API.
It need not necessarily be human sensible, however the preference is to use reverse
internet domain name of the developer, followed by a '.' then by a name that represents
the plug-in.. It must be a legal ASCII string and have no whitespace in the
name and no non printing chars.
For example "uk.co.somesoftwarehouse.myPlugin"*/
    pub pluginIdentifier: *const ::std::os::raw::c_char,
    /// Major version of this plug-in, this gets incremented when backwards compatibility is broken.
    pub pluginVersionMajor: ::std::os::raw::c_uint,
    /**  Major version of this plug-in, this gets incremented when software is changed,
but does not break backwards compatibility.*/
    pub pluginVersionMinor: ::std::os::raw::c_uint,
    /** @brief Function the host uses to connect the plug-in to the host's api fetcher

\arg \c fetchApi pointer to host's API fetcher

Mandatory function.

The very first function called in a plug-in. The plug-in \em must \em not call any OFX functions within this, it must only set its local copy of the host pointer.

\pre
- nothing else has been called

\post
- the pointer suite is valid until the plug-in is unloaded

It is recommended that hosts should return the same host and suite pointers to all plugins
in the same shared lib or bundle.*/
    pub setHost: ::std::option::Option<unsafe extern "C" fn(host: *mut OfxHost)>,
    /** @brief Main entry point for plug-ins

Mandatory function.

The exact set of actions is determined by the plug-in API that is being implemented, however all plug-ins
can perform several actions. For the list of actions consult \ref ActionsAll.

Preconditions
- setHost has been called*/
    pub mainEntry: OfxPluginEntryPoint,
}
/// @brief How time is specified within the OFX API
pub type OfxTime = f64;
/// @brief Defines one dimensional integer bounds
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OfxRangeI {
    pub min: ::std::os::raw::c_int,
    pub max: ::std::os::raw::c_int,
}
/// @brief Defines one dimensional double bounds
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OfxRangeD {
    pub min: f64,
    pub max: f64,
}
/// @brief Defines two dimensional integer point
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OfxPointI {
    pub x: ::std::os::raw::c_int,
    pub y: ::std::os::raw::c_int,
}
/// @brief Defines two dimensional double point
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OfxPointD {
    pub x: f64,
    pub y: f64,
}
/** @brief Defines two dimensional integer region

Regions are x1 <= x < x2

Infinite regions are flagged by setting
- x1 = \ref kOfxFlagInfiniteMin
- y1 = \ref kOfxFlagInfiniteMin
- x2 = \ref kOfxFlagInfiniteMax
- y2 = \ref kOfxFlagInfiniteMax
*/
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OfxRectI {
    pub x1: ::std::os::raw::c_int,
    pub y1: ::std::os::raw::c_int,
    pub x2: ::std::os::raw::c_int,
    pub y2: ::std::os::raw::c_int,
}
/** @brief Defines two dimensional double region

Regions are x1 <= x < x2

Infinite regions are flagged by setting
- x1 = \ref kOfxFlagInfiniteMin
- y1 = \ref kOfxFlagInfiniteMin
- x2 = \ref kOfxFlagInfiniteMax
- y2 = \ref kOfxFlagInfiniteMax
*/
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OfxRectD {
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
}

pub const kOfxStatOK: OfxStatus = 0;
pub const kOfxStatFailed: OfxStatus = 1;
pub const kOfxStatErrFatal: OfxStatus = 2;
pub const kOfxStatErrUnknown: OfxStatus = 3;
pub const kOfxStatErrMissingHostFeature: OfxStatus = 4;
pub const kOfxStatErrUnsupported: OfxStatus = 5;
pub const kOfxStatErrExists: OfxStatus = 6;
pub const kOfxStatErrFormat: OfxStatus = 7;
pub const kOfxStatErrMemory: OfxStatus = 8;
pub const kOfxStatErrBadHandle: OfxStatus = 9;
pub const kOfxStatErrBadIndex: OfxStatus = 10;
pub const kOfxStatErrValue: OfxStatus = 11;
pub const kOfxStatReplyYes: OfxStatus = 12;
pub const kOfxStatReplyNo: OfxStatus = 13;
pub const kOfxStatReplyDefault: OfxStatus = 14;
pub const kOfxStatUnlicensed: OfxStatus = 15;
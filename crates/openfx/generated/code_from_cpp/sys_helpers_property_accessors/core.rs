make_property_setter!(
    set_OfxPluginPropFilePath,
    kOfxPluginPropFilePath,
    crate::generic::sys_helpers::properties::set_string,
    1,
    String
);
make_property_getter!(
    get_OfxPluginPropFilePath,
    kOfxPluginPropFilePath,
    crate::generic::sys_helpers::properties::get_string,
    1,
    String
);
make_property_resetter!(reset_OfxPluginPropFilePath, kOfxPluginPropFilePath);
make_property_setter!(set_OfxPropAPIVersion, kOfxPropAPIVersion, crate::generic::sys_helpers::properties::set_ints, ..., Int);
make_property_getter!(get_OfxPropAPIVersion, kOfxPropAPIVersion, crate::generic::sys_helpers::properties::get_ints, ..., Int);
make_property_resetter!(reset_OfxPropAPIVersion, kOfxPropAPIVersion);
make_property_dimension_getter!(get_dimension_OfxPropAPIVersion, kOfxPropAPIVersion);
make_property_setter!(
    set_OfxPropChangeReason,
    kOfxPropChangeReason,
    crate::generic::sys_helpers::properties::set_string,
    1,
    String
);
make_property_getter!(
    get_OfxPropChangeReason,
    kOfxPropChangeReason,
    crate::generic::sys_helpers::properties::get_string,
    1,
    String
);
make_property_resetter!(reset_OfxPropChangeReason, kOfxPropChangeReason);
make_property_setter!(
    set_OfxPropEffectInstance,
    kOfxPropEffectInstance,
    crate::generic::sys_helpers::properties::set_pointer,
    1,
    Pointer
);
make_property_getter!(
    get_OfxPropEffectInstance,
    kOfxPropEffectInstance,
    crate::generic::sys_helpers::properties::get_pointer,
    1,
    Pointer
);
make_property_resetter!(reset_OfxPropEffectInstance, kOfxPropEffectInstance);
make_property_setter!(
    set_OfxPropHostOSHandle,
    kOfxPropHostOSHandle,
    crate::generic::sys_helpers::properties::set_pointer,
    1,
    Pointer
);
make_property_getter!(
    get_OfxPropHostOSHandle,
    kOfxPropHostOSHandle,
    crate::generic::sys_helpers::properties::get_pointer,
    1,
    Pointer
);
make_property_resetter!(reset_OfxPropHostOSHandle, kOfxPropHostOSHandle);
make_property_setter!(
    set_OfxPropIcon,
    kOfxPropIcon,
    crate::generic::sys_helpers::properties::set_strings_2,
    2,
    String
);
make_property_getter!(
    get_OfxPropIcon,
    kOfxPropIcon,
    crate::generic::sys_helpers::properties::get_strings_2,
    2,
    String
);
make_property_resetter!(reset_OfxPropIcon, kOfxPropIcon);
make_property_setter!(
    set_OfxPropInstanceData,
    kOfxPropInstanceData,
    crate::generic::sys_helpers::properties::set_pointer,
    1,
    Pointer
);
make_property_getter!(
    get_OfxPropInstanceData,
    kOfxPropInstanceData,
    crate::generic::sys_helpers::properties::get_pointer,
    1,
    Pointer
);
make_property_resetter!(reset_OfxPropInstanceData, kOfxPropInstanceData);
make_property_setter!(
    set_OfxPropIsInteractive,
    kOfxPropIsInteractive,
    crate::generic::sys_helpers::properties::set_int,
    1,
    Int
);
make_property_getter!(
    get_OfxPropIsInteractive,
    kOfxPropIsInteractive,
    crate::generic::sys_helpers::properties::get_int,
    1,
    Int
);
make_property_resetter!(reset_OfxPropIsInteractive, kOfxPropIsInteractive);
make_property_setter!(
    set_OfxPropLabel,
    kOfxPropLabel,
    crate::generic::sys_helpers::properties::set_string,
    1,
    String
);
make_property_getter!(
    get_OfxPropLabel,
    kOfxPropLabel,
    crate::generic::sys_helpers::properties::get_string,
    1,
    String
);
make_property_resetter!(reset_OfxPropLabel, kOfxPropLabel);
make_property_setter!(
    set_OfxPropLongLabel,
    kOfxPropLongLabel,
    crate::generic::sys_helpers::properties::set_string,
    1,
    String
);
make_property_getter!(
    get_OfxPropLongLabel,
    kOfxPropLongLabel,
    crate::generic::sys_helpers::properties::get_string,
    1,
    String
);
make_property_resetter!(reset_OfxPropLongLabel, kOfxPropLongLabel);
make_property_setter!(
    set_OfxPropName,
    kOfxPropName,
    crate::generic::sys_helpers::properties::set_string,
    1,
    String
);
make_property_getter!(
    get_OfxPropName,
    kOfxPropName,
    crate::generic::sys_helpers::properties::get_string,
    1,
    String
);
make_property_resetter!(reset_OfxPropName, kOfxPropName);
make_property_setter!(
    set_OfxPropPluginDescription,
    kOfxPropPluginDescription,
    crate::generic::sys_helpers::properties::set_string,
    1,
    String
);
make_property_getter!(
    get_OfxPropPluginDescription,
    kOfxPropPluginDescription,
    crate::generic::sys_helpers::properties::get_string,
    1,
    String
);
make_property_resetter!(reset_OfxPropPluginDescription, kOfxPropPluginDescription);
make_property_setter!(
    set_OfxPropShortLabel,
    kOfxPropShortLabel,
    crate::generic::sys_helpers::properties::set_string,
    1,
    String
);
make_property_getter!(
    get_OfxPropShortLabel,
    kOfxPropShortLabel,
    crate::generic::sys_helpers::properties::get_string,
    1,
    String
);
make_property_resetter!(reset_OfxPropShortLabel, kOfxPropShortLabel);
make_property_setter!(
    set_OfxPropTime,
    kOfxPropTime,
    crate::generic::sys_helpers::properties::set_double,
    1,
    Double
);
make_property_getter!(
    get_OfxPropTime,
    kOfxPropTime,
    crate::generic::sys_helpers::properties::get_double,
    1,
    Double
);
make_property_resetter!(reset_OfxPropTime, kOfxPropTime);
make_property_setter!(
    set_OfxPropType,
    kOfxPropType,
    crate::generic::sys_helpers::properties::set_string,
    1,
    String
);
make_property_getter!(
    get_OfxPropType,
    kOfxPropType,
    crate::generic::sys_helpers::properties::get_string,
    1,
    String
);
make_property_resetter!(reset_OfxPropType, kOfxPropType);
make_property_setter!(set_OfxPropVersion, kOfxPropVersion, crate::generic::sys_helpers::properties::set_ints, ..., Int);
make_property_getter!(get_OfxPropVersion, kOfxPropVersion, crate::generic::sys_helpers::properties::get_ints, ..., Int);
make_property_resetter!(reset_OfxPropVersion, kOfxPropVersion);
make_property_dimension_getter!(get_dimension_OfxPropVersion, kOfxPropVersion);
make_property_setter!(
    set_OfxPropVersionLabel,
    kOfxPropVersionLabel,
    crate::generic::sys_helpers::properties::set_string,
    1,
    String
);
make_property_getter!(
    get_OfxPropVersionLabel,
    kOfxPropVersionLabel,
    crate::generic::sys_helpers::properties::get_string,
    1,
    String
);
make_property_resetter!(reset_OfxPropVersionLabel, kOfxPropVersionLabel);

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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of OfxPropertySuiteV1",
    ][::std::mem::size_of::<OfxPropertySuiteV1>() - 144usize];
    [
        "Alignment of OfxPropertySuiteV1",
    ][::std::mem::align_of::<OfxPropertySuiteV1>() - 8usize];
    [
        "Offset of field: OfxPropertySuiteV1::propSetPointer",
    ][::std::mem::offset_of!(OfxPropertySuiteV1, propSetPointer) - 0usize];
    [
        "Offset of field: OfxPropertySuiteV1::propSetString",
    ][::std::mem::offset_of!(OfxPropertySuiteV1, propSetString) - 8usize];
    [
        "Offset of field: OfxPropertySuiteV1::propSetDouble",
    ][::std::mem::offset_of!(OfxPropertySuiteV1, propSetDouble) - 16usize];
    [
        "Offset of field: OfxPropertySuiteV1::propSetInt",
    ][::std::mem::offset_of!(OfxPropertySuiteV1, propSetInt) - 24usize];
    [
        "Offset of field: OfxPropertySuiteV1::propSetPointerN",
    ][::std::mem::offset_of!(OfxPropertySuiteV1, propSetPointerN) - 32usize];
    [
        "Offset of field: OfxPropertySuiteV1::propSetStringN",
    ][::std::mem::offset_of!(OfxPropertySuiteV1, propSetStringN) - 40usize];
    [
        "Offset of field: OfxPropertySuiteV1::propSetDoubleN",
    ][::std::mem::offset_of!(OfxPropertySuiteV1, propSetDoubleN) - 48usize];
    [
        "Offset of field: OfxPropertySuiteV1::propSetIntN",
    ][::std::mem::offset_of!(OfxPropertySuiteV1, propSetIntN) - 56usize];
    [
        "Offset of field: OfxPropertySuiteV1::propGetPointer",
    ][::std::mem::offset_of!(OfxPropertySuiteV1, propGetPointer) - 64usize];
    [
        "Offset of field: OfxPropertySuiteV1::propGetString",
    ][::std::mem::offset_of!(OfxPropertySuiteV1, propGetString) - 72usize];
    [
        "Offset of field: OfxPropertySuiteV1::propGetDouble",
    ][::std::mem::offset_of!(OfxPropertySuiteV1, propGetDouble) - 80usize];
    [
        "Offset of field: OfxPropertySuiteV1::propGetInt",
    ][::std::mem::offset_of!(OfxPropertySuiteV1, propGetInt) - 88usize];
    [
        "Offset of field: OfxPropertySuiteV1::propGetPointerN",
    ][::std::mem::offset_of!(OfxPropertySuiteV1, propGetPointerN) - 96usize];
    [
        "Offset of field: OfxPropertySuiteV1::propGetStringN",
    ][::std::mem::offset_of!(OfxPropertySuiteV1, propGetStringN) - 104usize];
    [
        "Offset of field: OfxPropertySuiteV1::propGetDoubleN",
    ][::std::mem::offset_of!(OfxPropertySuiteV1, propGetDoubleN) - 112usize];
    [
        "Offset of field: OfxPropertySuiteV1::propGetIntN",
    ][::std::mem::offset_of!(OfxPropertySuiteV1, propGetIntN) - 120usize];
    [
        "Offset of field: OfxPropertySuiteV1::propReset",
    ][::std::mem::offset_of!(OfxPropertySuiteV1, propReset) - 128usize];
    [
        "Offset of field: OfxPropertySuiteV1::propGetDimension",
    ][::std::mem::offset_of!(OfxPropertySuiteV1, propGetDimension) - 136usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of OfxBytes"][::std::mem::size_of::<OfxBytes>() - 16usize];
    ["Alignment of OfxBytes"][::std::mem::align_of::<OfxBytes>() - 8usize];
    ["Offset of field: OfxBytes::data"][::std::mem::offset_of!(OfxBytes, data) - 0usize];
    [
        "Offset of field: OfxBytes::length",
    ][::std::mem::offset_of!(OfxBytes, length) - 8usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of OfxParameterSuiteV1",
    ][::std::mem::size_of::<OfxParameterSuiteV1>() - 144usize];
    [
        "Alignment of OfxParameterSuiteV1",
    ][::std::mem::align_of::<OfxParameterSuiteV1>() - 8usize];
    [
        "Offset of field: OfxParameterSuiteV1::paramDefine",
    ][::std::mem::offset_of!(OfxParameterSuiteV1, paramDefine) - 0usize];
    [
        "Offset of field: OfxParameterSuiteV1::paramGetHandle",
    ][::std::mem::offset_of!(OfxParameterSuiteV1, paramGetHandle) - 8usize];
    [
        "Offset of field: OfxParameterSuiteV1::paramSetGetPropertySet",
    ][::std::mem::offset_of!(OfxParameterSuiteV1, paramSetGetPropertySet) - 16usize];
    [
        "Offset of field: OfxParameterSuiteV1::paramGetPropertySet",
    ][::std::mem::offset_of!(OfxParameterSuiteV1, paramGetPropertySet) - 24usize];
    [
        "Offset of field: OfxParameterSuiteV1::paramGetValue",
    ][::std::mem::offset_of!(OfxParameterSuiteV1, paramGetValue) - 32usize];
    [
        "Offset of field: OfxParameterSuiteV1::paramGetValueAtTime",
    ][::std::mem::offset_of!(OfxParameterSuiteV1, paramGetValueAtTime) - 40usize];
    [
        "Offset of field: OfxParameterSuiteV1::paramGetDerivative",
    ][::std::mem::offset_of!(OfxParameterSuiteV1, paramGetDerivative) - 48usize];
    [
        "Offset of field: OfxParameterSuiteV1::paramGetIntegral",
    ][::std::mem::offset_of!(OfxParameterSuiteV1, paramGetIntegral) - 56usize];
    [
        "Offset of field: OfxParameterSuiteV1::paramSetValue",
    ][::std::mem::offset_of!(OfxParameterSuiteV1, paramSetValue) - 64usize];
    [
        "Offset of field: OfxParameterSuiteV1::paramSetValueAtTime",
    ][::std::mem::offset_of!(OfxParameterSuiteV1, paramSetValueAtTime) - 72usize];
    [
        "Offset of field: OfxParameterSuiteV1::paramGetNumKeys",
    ][::std::mem::offset_of!(OfxParameterSuiteV1, paramGetNumKeys) - 80usize];
    [
        "Offset of field: OfxParameterSuiteV1::paramGetKeyTime",
    ][::std::mem::offset_of!(OfxParameterSuiteV1, paramGetKeyTime) - 88usize];
    [
        "Offset of field: OfxParameterSuiteV1::paramGetKeyIndex",
    ][::std::mem::offset_of!(OfxParameterSuiteV1, paramGetKeyIndex) - 96usize];
    [
        "Offset of field: OfxParameterSuiteV1::paramDeleteKey",
    ][::std::mem::offset_of!(OfxParameterSuiteV1, paramDeleteKey) - 104usize];
    [
        "Offset of field: OfxParameterSuiteV1::paramDeleteAllKeys",
    ][::std::mem::offset_of!(OfxParameterSuiteV1, paramDeleteAllKeys) - 112usize];
    [
        "Offset of field: OfxParameterSuiteV1::paramCopy",
    ][::std::mem::offset_of!(OfxParameterSuiteV1, paramCopy) - 120usize];
    [
        "Offset of field: OfxParameterSuiteV1::paramEditBegin",
    ][::std::mem::offset_of!(OfxParameterSuiteV1, paramEditBegin) - 128usize];
    [
        "Offset of field: OfxParameterSuiteV1::paramEditEnd",
    ][::std::mem::offset_of!(OfxParameterSuiteV1, paramEditEnd) - 136usize];
};

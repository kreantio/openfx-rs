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
    ["Size of OfxDialogSuiteV1"][::std::mem::size_of::<OfxDialogSuiteV1>() - 16usize];
    [
        "Alignment of OfxDialogSuiteV1",
    ][::std::mem::align_of::<OfxDialogSuiteV1>() - 8usize];
    [
        "Offset of field: OfxDialogSuiteV1::RequestDialog",
    ][::std::mem::offset_of!(OfxDialogSuiteV1, RequestDialog) - 0usize];
    [
        "Offset of field: OfxDialogSuiteV1::NotifyRedrawPending",
    ][::std::mem::offset_of!(OfxDialogSuiteV1, NotifyRedrawPending) - 8usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of OfxRGBAColourB"][::std::mem::size_of::<OfxRGBAColourB>() - 4usize];
    ["Alignment of OfxRGBAColourB"][::std::mem::align_of::<OfxRGBAColourB>() - 1usize];
    [
        "Offset of field: OfxRGBAColourB::r",
    ][::std::mem::offset_of!(OfxRGBAColourB, r) - 0usize];
    [
        "Offset of field: OfxRGBAColourB::g",
    ][::std::mem::offset_of!(OfxRGBAColourB, g) - 1usize];
    [
        "Offset of field: OfxRGBAColourB::b",
    ][::std::mem::offset_of!(OfxRGBAColourB, b) - 2usize];
    [
        "Offset of field: OfxRGBAColourB::a",
    ][::std::mem::offset_of!(OfxRGBAColourB, a) - 3usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of OfxRGBAColourS"][::std::mem::size_of::<OfxRGBAColourS>() - 8usize];
    ["Alignment of OfxRGBAColourS"][::std::mem::align_of::<OfxRGBAColourS>() - 2usize];
    [
        "Offset of field: OfxRGBAColourS::r",
    ][::std::mem::offset_of!(OfxRGBAColourS, r) - 0usize];
    [
        "Offset of field: OfxRGBAColourS::g",
    ][::std::mem::offset_of!(OfxRGBAColourS, g) - 2usize];
    [
        "Offset of field: OfxRGBAColourS::b",
    ][::std::mem::offset_of!(OfxRGBAColourS, b) - 4usize];
    [
        "Offset of field: OfxRGBAColourS::a",
    ][::std::mem::offset_of!(OfxRGBAColourS, a) - 6usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of OfxRGBAColourF"][::std::mem::size_of::<OfxRGBAColourF>() - 16usize];
    ["Alignment of OfxRGBAColourF"][::std::mem::align_of::<OfxRGBAColourF>() - 4usize];
    [
        "Offset of field: OfxRGBAColourF::r",
    ][::std::mem::offset_of!(OfxRGBAColourF, r) - 0usize];
    [
        "Offset of field: OfxRGBAColourF::g",
    ][::std::mem::offset_of!(OfxRGBAColourF, g) - 4usize];
    [
        "Offset of field: OfxRGBAColourF::b",
    ][::std::mem::offset_of!(OfxRGBAColourF, b) - 8usize];
    [
        "Offset of field: OfxRGBAColourF::a",
    ][::std::mem::offset_of!(OfxRGBAColourF, a) - 12usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of OfxRGBAColourD"][::std::mem::size_of::<OfxRGBAColourD>() - 32usize];
    ["Alignment of OfxRGBAColourD"][::std::mem::align_of::<OfxRGBAColourD>() - 8usize];
    [
        "Offset of field: OfxRGBAColourD::r",
    ][::std::mem::offset_of!(OfxRGBAColourD, r) - 0usize];
    [
        "Offset of field: OfxRGBAColourD::g",
    ][::std::mem::offset_of!(OfxRGBAColourD, g) - 8usize];
    [
        "Offset of field: OfxRGBAColourD::b",
    ][::std::mem::offset_of!(OfxRGBAColourD, b) - 16usize];
    [
        "Offset of field: OfxRGBAColourD::a",
    ][::std::mem::offset_of!(OfxRGBAColourD, a) - 24usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of OfxRGBColourB"][::std::mem::size_of::<OfxRGBColourB>() - 3usize];
    ["Alignment of OfxRGBColourB"][::std::mem::align_of::<OfxRGBColourB>() - 1usize];
    [
        "Offset of field: OfxRGBColourB::r",
    ][::std::mem::offset_of!(OfxRGBColourB, r) - 0usize];
    [
        "Offset of field: OfxRGBColourB::g",
    ][::std::mem::offset_of!(OfxRGBColourB, g) - 1usize];
    [
        "Offset of field: OfxRGBColourB::b",
    ][::std::mem::offset_of!(OfxRGBColourB, b) - 2usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of OfxRGBColourS"][::std::mem::size_of::<OfxRGBColourS>() - 6usize];
    ["Alignment of OfxRGBColourS"][::std::mem::align_of::<OfxRGBColourS>() - 2usize];
    [
        "Offset of field: OfxRGBColourS::r",
    ][::std::mem::offset_of!(OfxRGBColourS, r) - 0usize];
    [
        "Offset of field: OfxRGBColourS::g",
    ][::std::mem::offset_of!(OfxRGBColourS, g) - 2usize];
    [
        "Offset of field: OfxRGBColourS::b",
    ][::std::mem::offset_of!(OfxRGBColourS, b) - 4usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of OfxRGBColourF"][::std::mem::size_of::<OfxRGBColourF>() - 12usize];
    ["Alignment of OfxRGBColourF"][::std::mem::align_of::<OfxRGBColourF>() - 4usize];
    [
        "Offset of field: OfxRGBColourF::r",
    ][::std::mem::offset_of!(OfxRGBColourF, r) - 0usize];
    [
        "Offset of field: OfxRGBColourF::g",
    ][::std::mem::offset_of!(OfxRGBColourF, g) - 4usize];
    [
        "Offset of field: OfxRGBColourF::b",
    ][::std::mem::offset_of!(OfxRGBColourF, b) - 8usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of OfxRGBColourD"][::std::mem::size_of::<OfxRGBColourD>() - 24usize];
    ["Alignment of OfxRGBColourD"][::std::mem::align_of::<OfxRGBColourD>() - 8usize];
    [
        "Offset of field: OfxRGBColourD::r",
    ][::std::mem::offset_of!(OfxRGBColourD, r) - 0usize];
    [
        "Offset of field: OfxRGBColourD::g",
    ][::std::mem::offset_of!(OfxRGBColourD, g) - 8usize];
    [
        "Offset of field: OfxRGBColourD::b",
    ][::std::mem::offset_of!(OfxRGBColourD, b) - 16usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of OfxDrawSuiteV1"][::std::mem::size_of::<OfxDrawSuiteV1>() - 48usize];
    ["Alignment of OfxDrawSuiteV1"][::std::mem::align_of::<OfxDrawSuiteV1>() - 8usize];
    [
        "Offset of field: OfxDrawSuiteV1::getColour",
    ][::std::mem::offset_of!(OfxDrawSuiteV1, getColour) - 0usize];
    [
        "Offset of field: OfxDrawSuiteV1::setColour",
    ][::std::mem::offset_of!(OfxDrawSuiteV1, setColour) - 8usize];
    [
        "Offset of field: OfxDrawSuiteV1::setLineWidth",
    ][::std::mem::offset_of!(OfxDrawSuiteV1, setLineWidth) - 16usize];
    [
        "Offset of field: OfxDrawSuiteV1::setLineStipple",
    ][::std::mem::offset_of!(OfxDrawSuiteV1, setLineStipple) - 24usize];
    [
        "Offset of field: OfxDrawSuiteV1::draw",
    ][::std::mem::offset_of!(OfxDrawSuiteV1, draw) - 32usize];
    [
        "Offset of field: OfxDrawSuiteV1::drawText",
    ][::std::mem::offset_of!(OfxDrawSuiteV1, drawText) - 40usize];
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of OfxInteractSuiteV1",
    ][::std::mem::size_of::<OfxInteractSuiteV1>() - 24usize];
    [
        "Alignment of OfxInteractSuiteV1",
    ][::std::mem::align_of::<OfxInteractSuiteV1>() - 8usize];
    [
        "Offset of field: OfxInteractSuiteV1::interactSwapBuffers",
    ][::std::mem::offset_of!(OfxInteractSuiteV1, interactSwapBuffers) - 0usize];
    [
        "Offset of field: OfxInteractSuiteV1::interactRedraw",
    ][::std::mem::offset_of!(OfxInteractSuiteV1, interactRedraw) - 8usize];
    [
        "Offset of field: OfxInteractSuiteV1::interactGetPropertySet",
    ][::std::mem::offset_of!(OfxInteractSuiteV1, interactGetPropertySet) - 16usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of OfxMessageSuiteV1"][::std::mem::size_of::<OfxMessageSuiteV1>() - 8usize];
    [
        "Alignment of OfxMessageSuiteV1",
    ][::std::mem::align_of::<OfxMessageSuiteV1>() - 8usize];
    [
        "Offset of field: OfxMessageSuiteV1::message",
    ][::std::mem::offset_of!(OfxMessageSuiteV1, message) - 0usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of OfxMessageSuiteV2"][::std::mem::size_of::<OfxMessageSuiteV2>() - 24usize];
    [
        "Alignment of OfxMessageSuiteV2",
    ][::std::mem::align_of::<OfxMessageSuiteV2>() - 8usize];
    [
        "Offset of field: OfxMessageSuiteV2::message",
    ][::std::mem::offset_of!(OfxMessageSuiteV2, message) - 0usize];
    [
        "Offset of field: OfxMessageSuiteV2::setPersistentMessage",
    ][::std::mem::offset_of!(OfxMessageSuiteV2, setPersistentMessage) - 8usize];
    [
        "Offset of field: OfxMessageSuiteV2::clearPersistentMessage",
    ][::std::mem::offset_of!(OfxMessageSuiteV2, clearPersistentMessage) - 16usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of OfxMemorySuiteV1"][::std::mem::size_of::<OfxMemorySuiteV1>() - 16usize];
    [
        "Alignment of OfxMemorySuiteV1",
    ][::std::mem::align_of::<OfxMemorySuiteV1>() - 8usize];
    [
        "Offset of field: OfxMemorySuiteV1::memoryAlloc",
    ][::std::mem::offset_of!(OfxMemorySuiteV1, memoryAlloc) - 0usize];
    [
        "Offset of field: OfxMemorySuiteV1::memoryFree",
    ][::std::mem::offset_of!(OfxMemorySuiteV1, memoryFree) - 8usize];
};
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of OfxImageEffectSuiteV1",
    ][::std::mem::size_of::<OfxImageEffectSuiteV1>() - 104usize];
    [
        "Alignment of OfxImageEffectSuiteV1",
    ][::std::mem::align_of::<OfxImageEffectSuiteV1>() - 8usize];
    [
        "Offset of field: OfxImageEffectSuiteV1::getPropertySet",
    ][::std::mem::offset_of!(OfxImageEffectSuiteV1, getPropertySet) - 0usize];
    [
        "Offset of field: OfxImageEffectSuiteV1::getParamSet",
    ][::std::mem::offset_of!(OfxImageEffectSuiteV1, getParamSet) - 8usize];
    [
        "Offset of field: OfxImageEffectSuiteV1::clipDefine",
    ][::std::mem::offset_of!(OfxImageEffectSuiteV1, clipDefine) - 16usize];
    [
        "Offset of field: OfxImageEffectSuiteV1::clipGetHandle",
    ][::std::mem::offset_of!(OfxImageEffectSuiteV1, clipGetHandle) - 24usize];
    [
        "Offset of field: OfxImageEffectSuiteV1::clipGetPropertySet",
    ][::std::mem::offset_of!(OfxImageEffectSuiteV1, clipGetPropertySet) - 32usize];
    [
        "Offset of field: OfxImageEffectSuiteV1::clipGetImage",
    ][::std::mem::offset_of!(OfxImageEffectSuiteV1, clipGetImage) - 40usize];
    [
        "Offset of field: OfxImageEffectSuiteV1::clipReleaseImage",
    ][::std::mem::offset_of!(OfxImageEffectSuiteV1, clipReleaseImage) - 48usize];
    [
        "Offset of field: OfxImageEffectSuiteV1::clipGetRegionOfDefinition",
    ][::std::mem::offset_of!(OfxImageEffectSuiteV1, clipGetRegionOfDefinition)
        - 56usize];
    [
        "Offset of field: OfxImageEffectSuiteV1::abort",
    ][::std::mem::offset_of!(OfxImageEffectSuiteV1, abort) - 64usize];
    [
        "Offset of field: OfxImageEffectSuiteV1::imageMemoryAlloc",
    ][::std::mem::offset_of!(OfxImageEffectSuiteV1, imageMemoryAlloc) - 72usize];
    [
        "Offset of field: OfxImageEffectSuiteV1::imageMemoryFree",
    ][::std::mem::offset_of!(OfxImageEffectSuiteV1, imageMemoryFree) - 80usize];
    [
        "Offset of field: OfxImageEffectSuiteV1::imageMemoryLock",
    ][::std::mem::offset_of!(OfxImageEffectSuiteV1, imageMemoryLock) - 88usize];
    [
        "Offset of field: OfxImageEffectSuiteV1::imageMemoryUnlock",
    ][::std::mem::offset_of!(OfxImageEffectSuiteV1, imageMemoryUnlock) - 96usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of OfxImageEffectOpenGLRenderSuiteV1",
    ][::std::mem::size_of::<OfxImageEffectOpenGLRenderSuiteV1>() - 24usize];
    [
        "Alignment of OfxImageEffectOpenGLRenderSuiteV1",
    ][::std::mem::align_of::<OfxImageEffectOpenGLRenderSuiteV1>() - 8usize];
    [
        "Offset of field: OfxImageEffectOpenGLRenderSuiteV1::clipLoadTexture",
    ][::std::mem::offset_of!(OfxImageEffectOpenGLRenderSuiteV1, clipLoadTexture)
        - 0usize];
    [
        "Offset of field: OfxImageEffectOpenGLRenderSuiteV1::clipFreeTexture",
    ][::std::mem::offset_of!(OfxImageEffectOpenGLRenderSuiteV1, clipFreeTexture)
        - 8usize];
    [
        "Offset of field: OfxImageEffectOpenGLRenderSuiteV1::flushResources",
    ][::std::mem::offset_of!(OfxImageEffectOpenGLRenderSuiteV1, flushResources)
        - 16usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of OfxOpenCLProgramSuiteV1",
    ][::std::mem::size_of::<OfxOpenCLProgramSuiteV1>() - 8usize];
    [
        "Alignment of OfxOpenCLProgramSuiteV1",
    ][::std::mem::align_of::<OfxOpenCLProgramSuiteV1>() - 8usize];
    [
        "Offset of field: OfxOpenCLProgramSuiteV1::compileProgram",
    ][::std::mem::offset_of!(OfxOpenCLProgramSuiteV1, compileProgram) - 0usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of OfxYUVAColourB"][::std::mem::size_of::<OfxYUVAColourB>() - 4usize];
    ["Alignment of OfxYUVAColourB"][::std::mem::align_of::<OfxYUVAColourB>() - 1usize];
    [
        "Offset of field: OfxYUVAColourB::y",
    ][::std::mem::offset_of!(OfxYUVAColourB, y) - 0usize];
    [
        "Offset of field: OfxYUVAColourB::u",
    ][::std::mem::offset_of!(OfxYUVAColourB, u) - 1usize];
    [
        "Offset of field: OfxYUVAColourB::v",
    ][::std::mem::offset_of!(OfxYUVAColourB, v) - 2usize];
    [
        "Offset of field: OfxYUVAColourB::a",
    ][::std::mem::offset_of!(OfxYUVAColourB, a) - 3usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of OfxYUVAColourS"][::std::mem::size_of::<OfxYUVAColourS>() - 8usize];
    ["Alignment of OfxYUVAColourS"][::std::mem::align_of::<OfxYUVAColourS>() - 2usize];
    [
        "Offset of field: OfxYUVAColourS::y",
    ][::std::mem::offset_of!(OfxYUVAColourS, y) - 0usize];
    [
        "Offset of field: OfxYUVAColourS::u",
    ][::std::mem::offset_of!(OfxYUVAColourS, u) - 2usize];
    [
        "Offset of field: OfxYUVAColourS::v",
    ][::std::mem::offset_of!(OfxYUVAColourS, v) - 4usize];
    [
        "Offset of field: OfxYUVAColourS::a",
    ][::std::mem::offset_of!(OfxYUVAColourS, a) - 6usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of OfxYUVAColourF"][::std::mem::size_of::<OfxYUVAColourF>() - 16usize];
    ["Alignment of OfxYUVAColourF"][::std::mem::align_of::<OfxYUVAColourF>() - 4usize];
    [
        "Offset of field: OfxYUVAColourF::y",
    ][::std::mem::offset_of!(OfxYUVAColourF, y) - 0usize];
    [
        "Offset of field: OfxYUVAColourF::u",
    ][::std::mem::offset_of!(OfxYUVAColourF, u) - 4usize];
    [
        "Offset of field: OfxYUVAColourF::v",
    ][::std::mem::offset_of!(OfxYUVAColourF, v) - 8usize];
    [
        "Offset of field: OfxYUVAColourF::a",
    ][::std::mem::offset_of!(OfxYUVAColourF, a) - 12usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of OfxParametricParameterSuiteV1",
    ][::std::mem::size_of::<OfxParametricParameterSuiteV1>() - 56usize];
    [
        "Alignment of OfxParametricParameterSuiteV1",
    ][::std::mem::align_of::<OfxParametricParameterSuiteV1>() - 8usize];
    [
        "Offset of field: OfxParametricParameterSuiteV1::parametricParamGetValue",
    ][::std::mem::offset_of!(OfxParametricParameterSuiteV1, parametricParamGetValue)
        - 0usize];
    [
        "Offset of field: OfxParametricParameterSuiteV1::parametricParamGetNControlPoints",
    ][::std::mem::offset_of!(
        OfxParametricParameterSuiteV1, parametricParamGetNControlPoints
    ) - 8usize];
    [
        "Offset of field: OfxParametricParameterSuiteV1::parametricParamGetNthControlPoint",
    ][::std::mem::offset_of!(
        OfxParametricParameterSuiteV1, parametricParamGetNthControlPoint
    ) - 16usize];
    [
        "Offset of field: OfxParametricParameterSuiteV1::parametricParamSetNthControlPoint",
    ][::std::mem::offset_of!(
        OfxParametricParameterSuiteV1, parametricParamSetNthControlPoint
    ) - 24usize];
    [
        "Offset of field: OfxParametricParameterSuiteV1::parametricParamAddControlPoint",
    ][::std::mem::offset_of!(
        OfxParametricParameterSuiteV1, parametricParamAddControlPoint
    ) - 32usize];
    [
        "Offset of field: OfxParametricParameterSuiteV1::parametricParamDeleteControlPoint",
    ][::std::mem::offset_of!(
        OfxParametricParameterSuiteV1, parametricParamDeleteControlPoint
    ) - 40usize];
    [
        "Offset of field: OfxParametricParameterSuiteV1::parametricParamDeleteAllControlPoints",
    ][::std::mem::offset_of!(
        OfxParametricParameterSuiteV1, parametricParamDeleteAllControlPoints
    ) - 48usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of OfxProgressSuiteV1",
    ][::std::mem::size_of::<OfxProgressSuiteV1>() - 24usize];
    [
        "Alignment of OfxProgressSuiteV1",
    ][::std::mem::align_of::<OfxProgressSuiteV1>() - 8usize];
    [
        "Offset of field: OfxProgressSuiteV1::progressStart",
    ][::std::mem::offset_of!(OfxProgressSuiteV1, progressStart) - 0usize];
    [
        "Offset of field: OfxProgressSuiteV1::progressUpdate",
    ][::std::mem::offset_of!(OfxProgressSuiteV1, progressUpdate) - 8usize];
    [
        "Offset of field: OfxProgressSuiteV1::progressEnd",
    ][::std::mem::offset_of!(OfxProgressSuiteV1, progressEnd) - 16usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of OfxProgressSuiteV2",
    ][::std::mem::size_of::<OfxProgressSuiteV2>() - 24usize];
    [
        "Alignment of OfxProgressSuiteV2",
    ][::std::mem::align_of::<OfxProgressSuiteV2>() - 8usize];
    [
        "Offset of field: OfxProgressSuiteV2::progressStart",
    ][::std::mem::offset_of!(OfxProgressSuiteV2, progressStart) - 0usize];
    [
        "Offset of field: OfxProgressSuiteV2::progressUpdate",
    ][::std::mem::offset_of!(OfxProgressSuiteV2, progressUpdate) - 8usize];
    [
        "Offset of field: OfxProgressSuiteV2::progressEnd",
    ][::std::mem::offset_of!(OfxProgressSuiteV2, progressEnd) - 16usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of OfxTimeLineSuiteV1",
    ][::std::mem::size_of::<OfxTimeLineSuiteV1>() - 24usize];
    [
        "Alignment of OfxTimeLineSuiteV1",
    ][::std::mem::align_of::<OfxTimeLineSuiteV1>() - 8usize];
    [
        "Offset of field: OfxTimeLineSuiteV1::getTime",
    ][::std::mem::offset_of!(OfxTimeLineSuiteV1, getTime) - 0usize];
    [
        "Offset of field: OfxTimeLineSuiteV1::gotoTime",
    ][::std::mem::offset_of!(OfxTimeLineSuiteV1, gotoTime) - 8usize];
    [
        "Offset of field: OfxTimeLineSuiteV1::getTimeBounds",
    ][::std::mem::offset_of!(OfxTimeLineSuiteV1, getTimeBounds) - 16usize];
};

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

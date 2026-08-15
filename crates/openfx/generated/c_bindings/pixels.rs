/// @brief Defines an 8 bit per component RGBA pixel
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OfxRGBAColourB {
    pub r: ::std::os::raw::c_uchar,
    pub g: ::std::os::raw::c_uchar,
    pub b: ::std::os::raw::c_uchar,
    pub a: ::std::os::raw::c_uchar,
}
/// @brief Defines a 16 bit per component RGBA pixel
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OfxRGBAColourS {
    pub r: ::std::os::raw::c_ushort,
    pub g: ::std::os::raw::c_ushort,
    pub b: ::std::os::raw::c_ushort,
    pub a: ::std::os::raw::c_ushort,
}
/// @brief Defines a floating point component RGBA pixel
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OfxRGBAColourF {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
/// @brief Defines a double precision floating point component RGBA pixel
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OfxRGBAColourD {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}
/// @brief Defines an 8 bit per component RGB pixel
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OfxRGBColourB {
    pub r: ::std::os::raw::c_uchar,
    pub g: ::std::os::raw::c_uchar,
    pub b: ::std::os::raw::c_uchar,
}
/// @brief Defines a 16 bit per component RGB pixel
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OfxRGBColourS {
    pub r: ::std::os::raw::c_ushort,
    pub g: ::std::os::raw::c_ushort,
    pub b: ::std::os::raw::c_ushort,
}
/// @brief Defines a floating point component RGB pixel
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OfxRGBColourF {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}
/// @brief Defines a double precision floating point component RGB pixel
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OfxRGBColourD {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

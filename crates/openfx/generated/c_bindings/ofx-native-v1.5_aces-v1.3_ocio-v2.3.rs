// Copyright OpenFX and contributors to the OpenFX project.
// SPDX-License-Identifier: BSD-3-Clause
pub const kOfxConfigIdentifier: &::std::ffi::CStr = c"ofx-native-v1.5_aces-v1.3_ocio-v2.3";
/// ```doxygen
/// @brief ofx_display_hdr
/// Any display-referred HDR video such as Rec. 2100 HLG or PQ.
/// ```
pub const kOfxColourspaceOfxDisplayHdr: &::std::ffi::CStr = c"ofx_display_hdr";
pub const kOfxColourspaceOfxDisplayHdrLabel: &::std::ffi::CStr = c"OFX generic display HDR";
pub const kOfxColourspaceOfxDisplayHdrEncoding: &::std::ffi::CStr = c"hdr-video";
pub const kOfxColourspaceOfxDisplayHdrIsData: bool = false;
pub const kOfxColourspaceOfxDisplayHdrIsBasic: bool = true;
pub const kOfxColourspaceOfxDisplayHdrIsCore: bool = true;
pub const kOfxColourspaceOfxDisplayHdrIsDisplay: bool = true;
/// ```doxygen
/// @brief ofx_display_sdr
/// Any display-referred SDR video such as Rec. 709.
/// ```
pub const kOfxColourspaceOfxDisplaySdr: &::std::ffi::CStr = c"ofx_display_sdr";
pub const kOfxColourspaceOfxDisplaySdrLabel: &::std::ffi::CStr = c"OFX generic display SDR";
pub const kOfxColourspaceOfxDisplaySdrEncoding: &::std::ffi::CStr = c"sdr-video";
pub const kOfxColourspaceOfxDisplaySdrIsData: bool = false;
pub const kOfxColourspaceOfxDisplaySdrIsBasic: bool = true;
pub const kOfxColourspaceOfxDisplaySdrIsCore: bool = true;
pub const kOfxColourspaceOfxDisplaySdrIsDisplay: bool = true;
/// ```doxygen
/// @brief ofx_raw
/// Image values should not be treated as colour, e.g. motion vectors or masks.
/// ```
pub const kOfxColourspaceOfxRaw: &::std::ffi::CStr = c"ofx_raw";
pub const kOfxColourspaceOfxRawLabel: &::std::ffi::CStr = c"OFX generic raw";
pub const kOfxColourspaceOfxRawEncoding: &::std::ffi::CStr = c"";
pub const kOfxColourspaceOfxRawIsData: bool = true;
pub const kOfxColourspaceOfxRawIsBasic: bool = true;
pub const kOfxColourspaceOfxRawIsCore: bool = true;
pub const kOfxColourspaceOfxRawIsDisplay: bool = false;
/// ```doxygen
/// @brief ofx_scene_linear
/// Any scene-referred linear colourspace.
/// ```
pub const kOfxColourspaceOfxSceneLinear: &::std::ffi::CStr = c"ofx_scene_linear";
pub const kOfxColourspaceOfxSceneLinearLabel: &::std::ffi::CStr = c"OFX generic scene linear";
pub const kOfxColourspaceOfxSceneLinearEncoding: &::std::ffi::CStr = c"scene-linear";
pub const kOfxColourspaceOfxSceneLinearIsData: bool = false;
pub const kOfxColourspaceOfxSceneLinearIsBasic: bool = true;
pub const kOfxColourspaceOfxSceneLinearIsCore: bool = true;
pub const kOfxColourspaceOfxSceneLinearIsDisplay: bool = false;
/// ```doxygen
/// @brief ofx_scene_log
/// Any scene-referred colourspace with a log transfer function.
/// ```
pub const kOfxColourspaceOfxSceneLog: &::std::ffi::CStr = c"ofx_scene_log";
pub const kOfxColourspaceOfxSceneLogLabel: &::std::ffi::CStr = c"OFX generic scene log";
pub const kOfxColourspaceOfxSceneLogEncoding: &::std::ffi::CStr = c"log";
pub const kOfxColourspaceOfxSceneLogIsData: bool = false;
pub const kOfxColourspaceOfxSceneLogIsBasic: bool = true;
pub const kOfxColourspaceOfxSceneLogIsCore: bool = true;
pub const kOfxColourspaceOfxSceneLogIsDisplay: bool = false;
pub const kOfxColourspaceSrgbDisplay: &::std::ffi::CStr = c"srgb_display";
pub const kOfxColourspaceSrgbDisplayLabel: &::std::ffi::CStr = c"sRGB - Display";
pub const kOfxColourspaceSrgbDisplayEncoding: &::std::ffi::CStr = c"sdr-video";
pub const kOfxColourspaceSrgbDisplayIsData: bool = false;
pub const kOfxColourspaceSrgbDisplayIsBasic: bool = false;
pub const kOfxColourspaceSrgbDisplayIsCore: bool = true;
pub const kOfxColourspaceSrgbDisplayIsDisplay: bool = true;
pub const kOfxColourspaceDisplayp3Display: &::std::ffi::CStr = c"displayp3_display";
pub const kOfxColourspaceDisplayp3DisplayLabel: &::std::ffi::CStr = c"Display P3 - Display";
pub const kOfxColourspaceDisplayp3DisplayEncoding: &::std::ffi::CStr = c"sdr-video";
pub const kOfxColourspaceDisplayp3DisplayIsData: bool = false;
pub const kOfxColourspaceDisplayp3DisplayIsBasic: bool = false;
pub const kOfxColourspaceDisplayp3DisplayIsCore: bool = true;
pub const kOfxColourspaceDisplayp3DisplayIsDisplay: bool = true;
pub const kOfxColourspaceRec1886Rec709Display: &::std::ffi::CStr = c"rec1886_rec709_display";
pub const kOfxColourspaceRec1886Rec709DisplayLabel: &::std::ffi::CStr = c"Rec.1886 Rec.709 - Display";
pub const kOfxColourspaceRec1886Rec709DisplayEncoding: &::std::ffi::CStr = c"sdr-video";
pub const kOfxColourspaceRec1886Rec709DisplayIsData: bool = false;
pub const kOfxColourspaceRec1886Rec709DisplayIsBasic: bool = false;
pub const kOfxColourspaceRec1886Rec709DisplayIsCore: bool = true;
pub const kOfxColourspaceRec1886Rec709DisplayIsDisplay: bool = true;
pub const kOfxColourspaceRec1886Rec2020Display: &::std::ffi::CStr = c"rec1886_rec2020_display";
pub const kOfxColourspaceRec1886Rec2020DisplayLabel: &::std::ffi::CStr = c"Rec.1886 Rec.2020 - Display";
pub const kOfxColourspaceRec1886Rec2020DisplayEncoding: &::std::ffi::CStr = c"sdr-video";
pub const kOfxColourspaceRec1886Rec2020DisplayIsData: bool = false;
pub const kOfxColourspaceRec1886Rec2020DisplayIsBasic: bool = false;
pub const kOfxColourspaceRec1886Rec2020DisplayIsCore: bool = true;
pub const kOfxColourspaceRec1886Rec2020DisplayIsDisplay: bool = true;
pub const kOfxColourspaceRec2100HlgDisplay: &::std::ffi::CStr = c"rec2100_hlg_display";
pub const kOfxColourspaceRec2100HlgDisplayLabel: &::std::ffi::CStr = c"Rec.2100-HLG - Display";
pub const kOfxColourspaceRec2100HlgDisplayEncoding: &::std::ffi::CStr = c"hdr-video";
pub const kOfxColourspaceRec2100HlgDisplayIsData: bool = false;
pub const kOfxColourspaceRec2100HlgDisplayIsBasic: bool = false;
pub const kOfxColourspaceRec2100HlgDisplayIsCore: bool = true;
pub const kOfxColourspaceRec2100HlgDisplayIsDisplay: bool = true;
pub const kOfxColourspaceRec2100PqDisplay: &::std::ffi::CStr = c"rec2100_pq_display";
pub const kOfxColourspaceRec2100PqDisplayLabel: &::std::ffi::CStr = c"Rec.2100-PQ - Display";
pub const kOfxColourspaceRec2100PqDisplayEncoding: &::std::ffi::CStr = c"hdr-video";
pub const kOfxColourspaceRec2100PqDisplayIsData: bool = false;
pub const kOfxColourspaceRec2100PqDisplayIsBasic: bool = false;
pub const kOfxColourspaceRec2100PqDisplayIsCore: bool = true;
pub const kOfxColourspaceRec2100PqDisplayIsDisplay: bool = true;
pub const kOfxColourspaceSt2084P3d65Display: &::std::ffi::CStr = c"st2084_p3d65_display";
pub const kOfxColourspaceSt2084P3d65DisplayLabel: &::std::ffi::CStr = c"ST2084-P3-D65 - Display";
pub const kOfxColourspaceSt2084P3d65DisplayEncoding: &::std::ffi::CStr = c"hdr-video";
pub const kOfxColourspaceSt2084P3d65DisplayIsData: bool = false;
pub const kOfxColourspaceSt2084P3d65DisplayIsBasic: bool = false;
pub const kOfxColourspaceSt2084P3d65DisplayIsCore: bool = true;
pub const kOfxColourspaceSt2084P3d65DisplayIsDisplay: bool = true;
pub const kOfxColourspaceP3d65Display: &::std::ffi::CStr = c"p3d65_display";
pub const kOfxColourspaceP3d65DisplayLabel: &::std::ffi::CStr = c"P3-D65 - Display";
pub const kOfxColourspaceP3d65DisplayEncoding: &::std::ffi::CStr = c"sdr-video";
pub const kOfxColourspaceP3d65DisplayIsData: bool = false;
pub const kOfxColourspaceP3d65DisplayIsBasic: bool = false;
pub const kOfxColourspaceP3d65DisplayIsCore: bool = true;
pub const kOfxColourspaceP3d65DisplayIsDisplay: bool = true;
pub const kOfxColourspaceACES20651: &::std::ffi::CStr = c"ACES2065-1";
pub const kOfxColourspaceACES20651Label: &::std::ffi::CStr = c"ACES2065-1";
pub const kOfxColourspaceACES20651Encoding: &::std::ffi::CStr = c"scene-linear";
pub const kOfxColourspaceACES20651IsData: bool = false;
pub const kOfxColourspaceACES20651IsBasic: bool = false;
pub const kOfxColourspaceACES20651IsCore: bool = true;
pub const kOfxColourspaceACES20651IsDisplay: bool = false;
pub const kOfxColourspaceACEScc: &::std::ffi::CStr = c"ACEScc";
pub const kOfxColourspaceACESccLabel: &::std::ffi::CStr = c"ACEScc";
pub const kOfxColourspaceACESccEncoding: &::std::ffi::CStr = c"log";
pub const kOfxColourspaceACESccIsData: bool = false;
pub const kOfxColourspaceACESccIsBasic: bool = false;
pub const kOfxColourspaceACESccIsCore: bool = true;
pub const kOfxColourspaceACESccIsDisplay: bool = false;
pub const kOfxColourspaceACEScct: &::std::ffi::CStr = c"ACEScct";
pub const kOfxColourspaceACEScctLabel: &::std::ffi::CStr = c"ACEScct";
pub const kOfxColourspaceACEScctEncoding: &::std::ffi::CStr = c"log";
pub const kOfxColourspaceACEScctIsData: bool = false;
pub const kOfxColourspaceACEScctIsBasic: bool = false;
pub const kOfxColourspaceACEScctIsCore: bool = true;
pub const kOfxColourspaceACEScctIsDisplay: bool = false;
pub const kOfxColourspaceACEScg: &::std::ffi::CStr = c"ACEScg";
pub const kOfxColourspaceACEScgLabel: &::std::ffi::CStr = c"ACEScg";
pub const kOfxColourspaceACEScgEncoding: &::std::ffi::CStr = c"scene-linear";
pub const kOfxColourspaceACEScgIsData: bool = false;
pub const kOfxColourspaceACEScgIsBasic: bool = false;
pub const kOfxColourspaceACEScgIsCore: bool = true;
pub const kOfxColourspaceACEScgIsDisplay: bool = false;
pub const kOfxColourspaceLinP3d65: &::std::ffi::CStr = c"lin_p3d65";
pub const kOfxColourspaceLinP3d65Label: &::std::ffi::CStr = c"Linear P3-D65";
pub const kOfxColourspaceLinP3d65Encoding: &::std::ffi::CStr = c"scene-linear";
pub const kOfxColourspaceLinP3d65IsData: bool = false;
pub const kOfxColourspaceLinP3d65IsBasic: bool = false;
pub const kOfxColourspaceLinP3d65IsCore: bool = true;
pub const kOfxColourspaceLinP3d65IsDisplay: bool = false;
pub const kOfxColourspaceLinRec2020: &::std::ffi::CStr = c"lin_rec2020";
pub const kOfxColourspaceLinRec2020Label: &::std::ffi::CStr = c"Linear Rec.2020";
pub const kOfxColourspaceLinRec2020Encoding: &::std::ffi::CStr = c"scene-linear";
pub const kOfxColourspaceLinRec2020IsData: bool = false;
pub const kOfxColourspaceLinRec2020IsBasic: bool = false;
pub const kOfxColourspaceLinRec2020IsCore: bool = true;
pub const kOfxColourspaceLinRec2020IsDisplay: bool = false;
pub const kOfxColourspaceLinRec709Srgb: &::std::ffi::CStr = c"lin_rec709_srgb";
pub const kOfxColourspaceLinRec709SrgbLabel: &::std::ffi::CStr = c"Linear Rec.709 (sRGB)";
pub const kOfxColourspaceLinRec709SrgbEncoding: &::std::ffi::CStr = c"scene-linear";
pub const kOfxColourspaceLinRec709SrgbIsData: bool = false;
pub const kOfxColourspaceLinRec709SrgbIsBasic: bool = false;
pub const kOfxColourspaceLinRec709SrgbIsCore: bool = true;
pub const kOfxColourspaceLinRec709SrgbIsDisplay: bool = false;
pub const kOfxColourspaceG18Rec709Tx: &::std::ffi::CStr = c"g18_rec709_tx";
pub const kOfxColourspaceG18Rec709TxLabel: &::std::ffi::CStr = c"Gamma 1.8 Rec.709 - Texture";
pub const kOfxColourspaceG18Rec709TxEncoding: &::std::ffi::CStr = c"sdr-video";
pub const kOfxColourspaceG18Rec709TxIsData: bool = false;
pub const kOfxColourspaceG18Rec709TxIsBasic: bool = false;
pub const kOfxColourspaceG18Rec709TxIsCore: bool = true;
pub const kOfxColourspaceG18Rec709TxIsDisplay: bool = false;
pub const kOfxColourspaceG22Ap1Tx: &::std::ffi::CStr = c"g22_ap1_tx";
pub const kOfxColourspaceG22Ap1TxLabel: &::std::ffi::CStr = c"Gamma 2.2 AP1 - Texture";
pub const kOfxColourspaceG22Ap1TxEncoding: &::std::ffi::CStr = c"sdr-video";
pub const kOfxColourspaceG22Ap1TxIsData: bool = false;
pub const kOfxColourspaceG22Ap1TxIsBasic: bool = false;
pub const kOfxColourspaceG22Ap1TxIsCore: bool = true;
pub const kOfxColourspaceG22Ap1TxIsDisplay: bool = false;
pub const kOfxColourspaceG22Rec709Tx: &::std::ffi::CStr = c"g22_rec709_tx";
pub const kOfxColourspaceG22Rec709TxLabel: &::std::ffi::CStr = c"Gamma 2.2 Rec.709 - Texture";
pub const kOfxColourspaceG22Rec709TxEncoding: &::std::ffi::CStr = c"sdr-video";
pub const kOfxColourspaceG22Rec709TxIsData: bool = false;
pub const kOfxColourspaceG22Rec709TxIsBasic: bool = false;
pub const kOfxColourspaceG22Rec709TxIsCore: bool = true;
pub const kOfxColourspaceG22Rec709TxIsDisplay: bool = false;
pub const kOfxColourspaceG24Rec709Tx: &::std::ffi::CStr = c"g24_rec709_tx";
pub const kOfxColourspaceG24Rec709TxLabel: &::std::ffi::CStr = c"Gamma 2.4 Rec.709 - Texture";
pub const kOfxColourspaceG24Rec709TxEncoding: &::std::ffi::CStr = c"sdr-video";
pub const kOfxColourspaceG24Rec709TxIsData: bool = false;
pub const kOfxColourspaceG24Rec709TxIsBasic: bool = false;
pub const kOfxColourspaceG24Rec709TxIsCore: bool = true;
pub const kOfxColourspaceG24Rec709TxIsDisplay: bool = false;
pub const kOfxColourspaceSrgbEncodedAp1Tx: &::std::ffi::CStr = c"srgb_encoded_ap1_tx";
pub const kOfxColourspaceSrgbEncodedAp1TxLabel: &::std::ffi::CStr = c"sRGB Encoded AP1 - Texture";
pub const kOfxColourspaceSrgbEncodedAp1TxEncoding: &::std::ffi::CStr = c"sdr-video";
pub const kOfxColourspaceSrgbEncodedAp1TxIsData: bool = false;
pub const kOfxColourspaceSrgbEncodedAp1TxIsBasic: bool = false;
pub const kOfxColourspaceSrgbEncodedAp1TxIsCore: bool = true;
pub const kOfxColourspaceSrgbEncodedAp1TxIsDisplay: bool = false;
pub const kOfxColourspaceSrgbEncodedP3d65Tx: &::std::ffi::CStr = c"srgb_encoded_p3d65_tx";
pub const kOfxColourspaceSrgbEncodedP3d65TxLabel: &::std::ffi::CStr = c"sRGB Encoded P3-D65 - Texture";
pub const kOfxColourspaceSrgbEncodedP3d65TxEncoding: &::std::ffi::CStr = c"sdr-video";
pub const kOfxColourspaceSrgbEncodedP3d65TxIsData: bool = false;
pub const kOfxColourspaceSrgbEncodedP3d65TxIsBasic: bool = false;
pub const kOfxColourspaceSrgbEncodedP3d65TxIsCore: bool = true;
pub const kOfxColourspaceSrgbEncodedP3d65TxIsDisplay: bool = false;
pub const kOfxColourspaceSrgbTx: &::std::ffi::CStr = c"srgb_tx";
pub const kOfxColourspaceSrgbTxLabel: &::std::ffi::CStr = c"sRGB - Texture";
pub const kOfxColourspaceSrgbTxEncoding: &::std::ffi::CStr = c"sdr-video";
pub const kOfxColourspaceSrgbTxIsData: bool = false;
pub const kOfxColourspaceSrgbTxIsBasic: bool = false;
pub const kOfxColourspaceSrgbTxIsCore: bool = true;
pub const kOfxColourspaceSrgbTxIsDisplay: bool = false;
pub const kOfxColourspaceRaw: &::std::ffi::CStr = c"Raw";
pub const kOfxColourspaceRawLabel: &::std::ffi::CStr = c"Raw";
pub const kOfxColourspaceRawEncoding: &::std::ffi::CStr = c"";
pub const kOfxColourspaceRawIsData: bool = true;
pub const kOfxColourspaceRawIsBasic: bool = false;
pub const kOfxColourspaceRawIsCore: bool = true;
pub const kOfxColourspaceRawIsDisplay: bool = false;
pub const kOfxColourspaceCIEXYZD65: &::std::ffi::CStr = c"CIE-XYZ-D65";
pub const kOfxColourspaceCIEXYZD65Label: &::std::ffi::CStr = c"CIE-XYZ-D65";
pub const kOfxColourspaceCIEXYZD65Encoding: &::std::ffi::CStr = c"";
pub const kOfxColourspaceCIEXYZD65IsData: bool = false;
pub const kOfxColourspaceCIEXYZD65IsBasic: bool = false;
pub const kOfxColourspaceCIEXYZD65IsCore: bool = false;
pub const kOfxColourspaceCIEXYZD65IsDisplay: bool = true;
pub const kOfxColourspaceP3d60Display: &::std::ffi::CStr = c"p3d60_display";
pub const kOfxColourspaceP3d60DisplayLabel: &::std::ffi::CStr = c"P3-D60 - Display";
pub const kOfxColourspaceP3d60DisplayEncoding: &::std::ffi::CStr = c"sdr-video";
pub const kOfxColourspaceP3d60DisplayIsData: bool = false;
pub const kOfxColourspaceP3d60DisplayIsBasic: bool = false;
pub const kOfxColourspaceP3d60DisplayIsCore: bool = false;
pub const kOfxColourspaceP3d60DisplayIsDisplay: bool = true;
pub const kOfxColourspaceP3DciDisplay: &::std::ffi::CStr = c"p3_dci_display";
pub const kOfxColourspaceP3DciDisplayLabel: &::std::ffi::CStr = c"P3-DCI - Display";
pub const kOfxColourspaceP3DciDisplayEncoding: &::std::ffi::CStr = c"sdr-video";
pub const kOfxColourspaceP3DciDisplayIsData: bool = false;
pub const kOfxColourspaceP3DciDisplayIsBasic: bool = false;
pub const kOfxColourspaceP3DciDisplayIsCore: bool = false;
pub const kOfxColourspaceP3DciDisplayIsDisplay: bool = true;
pub const kOfxColourspaceADX10: &::std::ffi::CStr = c"ADX10";
pub const kOfxColourspaceADX10Label: &::std::ffi::CStr = c"ADX10";
pub const kOfxColourspaceADX10Encoding: &::std::ffi::CStr = c"log";
pub const kOfxColourspaceADX10IsData: bool = false;
pub const kOfxColourspaceADX10IsBasic: bool = false;
pub const kOfxColourspaceADX10IsCore: bool = false;
pub const kOfxColourspaceADX10IsDisplay: bool = false;
pub const kOfxColourspaceADX16: &::std::ffi::CStr = c"ADX16";
pub const kOfxColourspaceADX16Label: &::std::ffi::CStr = c"ADX16";
pub const kOfxColourspaceADX16Encoding: &::std::ffi::CStr = c"log";
pub const kOfxColourspaceADX16IsData: bool = false;
pub const kOfxColourspaceADX16IsBasic: bool = false;
pub const kOfxColourspaceADX16IsCore: bool = false;
pub const kOfxColourspaceADX16IsDisplay: bool = false;
pub const kOfxColourspaceLinArriWideGamut3: &::std::ffi::CStr = c"lin_arri_wide_gamut_3";
pub const kOfxColourspaceLinArriWideGamut3Label: &::std::ffi::CStr = c"Linear ARRI Wide Gamut 3";
pub const kOfxColourspaceLinArriWideGamut3Encoding: &::std::ffi::CStr = c"scene-linear";
pub const kOfxColourspaceLinArriWideGamut3IsData: bool = false;
pub const kOfxColourspaceLinArriWideGamut3IsBasic: bool = false;
pub const kOfxColourspaceLinArriWideGamut3IsCore: bool = false;
pub const kOfxColourspaceLinArriWideGamut3IsDisplay: bool = false;
pub const kOfxColourspaceArriLogc3Ei800: &::std::ffi::CStr = c"arri_logc3_ei800";
pub const kOfxColourspaceArriLogc3Ei800Label: &::std::ffi::CStr = c"ARRI LogC3 (EI800)";
pub const kOfxColourspaceArriLogc3Ei800Encoding: &::std::ffi::CStr = c"log";
pub const kOfxColourspaceArriLogc3Ei800IsData: bool = false;
pub const kOfxColourspaceArriLogc3Ei800IsBasic: bool = false;
pub const kOfxColourspaceArriLogc3Ei800IsCore: bool = false;
pub const kOfxColourspaceArriLogc3Ei800IsDisplay: bool = false;
pub const kOfxColourspaceLinArriWideGamut4: &::std::ffi::CStr = c"lin_arri_wide_gamut_4";
pub const kOfxColourspaceLinArriWideGamut4Label: &::std::ffi::CStr = c"Linear ARRI Wide Gamut 4";
pub const kOfxColourspaceLinArriWideGamut4Encoding: &::std::ffi::CStr = c"scene-linear";
pub const kOfxColourspaceLinArriWideGamut4IsData: bool = false;
pub const kOfxColourspaceLinArriWideGamut4IsBasic: bool = false;
pub const kOfxColourspaceLinArriWideGamut4IsCore: bool = false;
pub const kOfxColourspaceLinArriWideGamut4IsDisplay: bool = false;
pub const kOfxColourspaceArriLogc4: &::std::ffi::CStr = c"arri_logc4";
pub const kOfxColourspaceArriLogc4Label: &::std::ffi::CStr = c"ARRI LogC4";
pub const kOfxColourspaceArriLogc4Encoding: &::std::ffi::CStr = c"log";
pub const kOfxColourspaceArriLogc4IsData: bool = false;
pub const kOfxColourspaceArriLogc4IsBasic: bool = false;
pub const kOfxColourspaceArriLogc4IsCore: bool = false;
pub const kOfxColourspaceArriLogc4IsDisplay: bool = false;
pub const kOfxColourspaceBmdfilmWidegamutGen5: &::std::ffi::CStr = c"bmdfilm_widegamut_gen5";
pub const kOfxColourspaceBmdfilmWidegamutGen5Label: &::std::ffi::CStr = c"BMDFilm WideGamut Gen5";
pub const kOfxColourspaceBmdfilmWidegamutGen5Encoding: &::std::ffi::CStr = c"log";
pub const kOfxColourspaceBmdfilmWidegamutGen5IsData: bool = false;
pub const kOfxColourspaceBmdfilmWidegamutGen5IsBasic: bool = false;
pub const kOfxColourspaceBmdfilmWidegamutGen5IsCore: bool = false;
pub const kOfxColourspaceBmdfilmWidegamutGen5IsDisplay: bool = false;
pub const kOfxColourspaceDavinciIntermediateWidegamut: &::std::ffi::CStr = c"davinci_intermediate_widegamut";
pub const kOfxColourspaceDavinciIntermediateWidegamutLabel: &::std::ffi::CStr = c"DaVinci Intermediate WideGamut";
pub const kOfxColourspaceDavinciIntermediateWidegamutEncoding: &::std::ffi::CStr = c"log";
pub const kOfxColourspaceDavinciIntermediateWidegamutIsData: bool = false;
pub const kOfxColourspaceDavinciIntermediateWidegamutIsBasic: bool = false;
pub const kOfxColourspaceDavinciIntermediateWidegamutIsCore: bool = false;
pub const kOfxColourspaceDavinciIntermediateWidegamutIsDisplay: bool = false;
pub const kOfxColourspaceLinBmdWidegamutGen5: &::std::ffi::CStr = c"lin_bmd_widegamut_gen5";
pub const kOfxColourspaceLinBmdWidegamutGen5Label: &::std::ffi::CStr = c"Linear BMD WideGamut Gen5";
pub const kOfxColourspaceLinBmdWidegamutGen5Encoding: &::std::ffi::CStr = c"scene-linear";
pub const kOfxColourspaceLinBmdWidegamutGen5IsData: bool = false;
pub const kOfxColourspaceLinBmdWidegamutGen5IsBasic: bool = false;
pub const kOfxColourspaceLinBmdWidegamutGen5IsCore: bool = false;
pub const kOfxColourspaceLinBmdWidegamutGen5IsDisplay: bool = false;
pub const kOfxColourspaceLinDavinciWidegamut: &::std::ffi::CStr = c"lin_davinci_widegamut";
pub const kOfxColourspaceLinDavinciWidegamutLabel: &::std::ffi::CStr = c"Linear DaVinci WideGamut";
pub const kOfxColourspaceLinDavinciWidegamutEncoding: &::std::ffi::CStr = c"scene-linear";
pub const kOfxColourspaceLinDavinciWidegamutIsData: bool = false;
pub const kOfxColourspaceLinDavinciWidegamutIsBasic: bool = false;
pub const kOfxColourspaceLinDavinciWidegamutIsCore: bool = false;
pub const kOfxColourspaceLinDavinciWidegamutIsDisplay: bool = false;
pub const kOfxColourspaceCanonlog2CinemagamutD55: &::std::ffi::CStr = c"canonlog2_cinemagamut_d55";
pub const kOfxColourspaceCanonlog2CinemagamutD55Label: &::std::ffi::CStr = c"CanonLog2 CinemaGamut D55";
pub const kOfxColourspaceCanonlog2CinemagamutD55Encoding: &::std::ffi::CStr = c"log";
pub const kOfxColourspaceCanonlog2CinemagamutD55IsData: bool = false;
pub const kOfxColourspaceCanonlog2CinemagamutD55IsBasic: bool = false;
pub const kOfxColourspaceCanonlog2CinemagamutD55IsCore: bool = false;
pub const kOfxColourspaceCanonlog2CinemagamutD55IsDisplay: bool = false;
pub const kOfxColourspaceCanonlog3CinemagamutD55: &::std::ffi::CStr = c"canonlog3_cinemagamut_d55";
pub const kOfxColourspaceCanonlog3CinemagamutD55Label: &::std::ffi::CStr = c"CanonLog3 CinemaGamut D55";
pub const kOfxColourspaceCanonlog3CinemagamutD55Encoding: &::std::ffi::CStr = c"log";
pub const kOfxColourspaceCanonlog3CinemagamutD55IsData: bool = false;
pub const kOfxColourspaceCanonlog3CinemagamutD55IsBasic: bool = false;
pub const kOfxColourspaceCanonlog3CinemagamutD55IsCore: bool = false;
pub const kOfxColourspaceCanonlog3CinemagamutD55IsDisplay: bool = false;
pub const kOfxColourspaceLinCinemagamutD55: &::std::ffi::CStr = c"lin_cinemagamut_d55";
pub const kOfxColourspaceLinCinemagamutD55Label: &::std::ffi::CStr = c"Linear CinemaGamut D55";
pub const kOfxColourspaceLinCinemagamutD55Encoding: &::std::ffi::CStr = c"scene-linear";
pub const kOfxColourspaceLinCinemagamutD55IsData: bool = false;
pub const kOfxColourspaceLinCinemagamutD55IsBasic: bool = false;
pub const kOfxColourspaceLinCinemagamutD55IsCore: bool = false;
pub const kOfxColourspaceLinCinemagamutD55IsDisplay: bool = false;
pub const kOfxColourspaceLinVgamut: &::std::ffi::CStr = c"lin_vgamut";
pub const kOfxColourspaceLinVgamutLabel: &::std::ffi::CStr = c"Linear V-Gamut";
pub const kOfxColourspaceLinVgamutEncoding: &::std::ffi::CStr = c"scene-linear";
pub const kOfxColourspaceLinVgamutIsData: bool = false;
pub const kOfxColourspaceLinVgamutIsBasic: bool = false;
pub const kOfxColourspaceLinVgamutIsCore: bool = false;
pub const kOfxColourspaceLinVgamutIsDisplay: bool = false;
pub const kOfxColourspaceVlogVgamut: &::std::ffi::CStr = c"vlog_vgamut";
pub const kOfxColourspaceVlogVgamutLabel: &::std::ffi::CStr = c"V-Log V-Gamut";
pub const kOfxColourspaceVlogVgamutEncoding: &::std::ffi::CStr = c"log";
pub const kOfxColourspaceVlogVgamutIsData: bool = false;
pub const kOfxColourspaceVlogVgamutIsBasic: bool = false;
pub const kOfxColourspaceVlogVgamutIsCore: bool = false;
pub const kOfxColourspaceVlogVgamutIsDisplay: bool = false;
pub const kOfxColourspaceLinRedwidegamutrgb: &::std::ffi::CStr = c"lin_redwidegamutrgb";
pub const kOfxColourspaceLinRedwidegamutrgbLabel: &::std::ffi::CStr = c"Linear REDWideGamutRGB";
pub const kOfxColourspaceLinRedwidegamutrgbEncoding: &::std::ffi::CStr = c"scene-linear";
pub const kOfxColourspaceLinRedwidegamutrgbIsData: bool = false;
pub const kOfxColourspaceLinRedwidegamutrgbIsBasic: bool = false;
pub const kOfxColourspaceLinRedwidegamutrgbIsCore: bool = false;
pub const kOfxColourspaceLinRedwidegamutrgbIsDisplay: bool = false;
pub const kOfxColourspaceLog3g10Redwidegamutrgb: &::std::ffi::CStr = c"log3g10_redwidegamutrgb";
pub const kOfxColourspaceLog3g10RedwidegamutrgbLabel: &::std::ffi::CStr = c"Log3G10 REDWideGamutRGB";
pub const kOfxColourspaceLog3g10RedwidegamutrgbEncoding: &::std::ffi::CStr = c"log";
pub const kOfxColourspaceLog3g10RedwidegamutrgbIsData: bool = false;
pub const kOfxColourspaceLog3g10RedwidegamutrgbIsBasic: bool = false;
pub const kOfxColourspaceLog3g10RedwidegamutrgbIsCore: bool = false;
pub const kOfxColourspaceLog3g10RedwidegamutrgbIsDisplay: bool = false;
pub const kOfxColourspaceLinSgamut3: &::std::ffi::CStr = c"lin_sgamut3";
pub const kOfxColourspaceLinSgamut3Label: &::std::ffi::CStr = c"Linear S-Gamut3";
pub const kOfxColourspaceLinSgamut3Encoding: &::std::ffi::CStr = c"scene-linear";
pub const kOfxColourspaceLinSgamut3IsData: bool = false;
pub const kOfxColourspaceLinSgamut3IsBasic: bool = false;
pub const kOfxColourspaceLinSgamut3IsCore: bool = false;
pub const kOfxColourspaceLinSgamut3IsDisplay: bool = false;
pub const kOfxColourspaceLinSgamut3cine: &::std::ffi::CStr = c"lin_sgamut3cine";
pub const kOfxColourspaceLinSgamut3cineLabel: &::std::ffi::CStr = c"Linear S-Gamut3.Cine";
pub const kOfxColourspaceLinSgamut3cineEncoding: &::std::ffi::CStr = c"scene-linear";
pub const kOfxColourspaceLinSgamut3cineIsData: bool = false;
pub const kOfxColourspaceLinSgamut3cineIsBasic: bool = false;
pub const kOfxColourspaceLinSgamut3cineIsCore: bool = false;
pub const kOfxColourspaceLinSgamut3cineIsDisplay: bool = false;
pub const kOfxColourspaceLinVeniceSgamut3: &::std::ffi::CStr = c"lin_venice_sgamut3";
pub const kOfxColourspaceLinVeniceSgamut3Label: &::std::ffi::CStr = c"Linear Venice S-Gamut3";
pub const kOfxColourspaceLinVeniceSgamut3Encoding: &::std::ffi::CStr = c"scene-linear";
pub const kOfxColourspaceLinVeniceSgamut3IsData: bool = false;
pub const kOfxColourspaceLinVeniceSgamut3IsBasic: bool = false;
pub const kOfxColourspaceLinVeniceSgamut3IsCore: bool = false;
pub const kOfxColourspaceLinVeniceSgamut3IsDisplay: bool = false;
pub const kOfxColourspaceLinVeniceSgamut3cine: &::std::ffi::CStr = c"lin_venice_sgamut3cine";
pub const kOfxColourspaceLinVeniceSgamut3cineLabel: &::std::ffi::CStr = c"Linear Venice S-Gamut3.Cine";
pub const kOfxColourspaceLinVeniceSgamut3cineEncoding: &::std::ffi::CStr = c"scene-linear";
pub const kOfxColourspaceLinVeniceSgamut3cineIsData: bool = false;
pub const kOfxColourspaceLinVeniceSgamut3cineIsBasic: bool = false;
pub const kOfxColourspaceLinVeniceSgamut3cineIsCore: bool = false;
pub const kOfxColourspaceLinVeniceSgamut3cineIsDisplay: bool = false;
pub const kOfxColourspaceSlog3Sgamut3: &::std::ffi::CStr = c"slog3_sgamut3";
pub const kOfxColourspaceSlog3Sgamut3Label: &::std::ffi::CStr = c"S-Log3 S-Gamut3";
pub const kOfxColourspaceSlog3Sgamut3Encoding: &::std::ffi::CStr = c"log";
pub const kOfxColourspaceSlog3Sgamut3IsData: bool = false;
pub const kOfxColourspaceSlog3Sgamut3IsBasic: bool = false;
pub const kOfxColourspaceSlog3Sgamut3IsCore: bool = false;
pub const kOfxColourspaceSlog3Sgamut3IsDisplay: bool = false;
pub const kOfxColourspaceSlog3Sgamut3cine: &::std::ffi::CStr = c"slog3_sgamut3cine";
pub const kOfxColourspaceSlog3Sgamut3cineLabel: &::std::ffi::CStr = c"S-Log3 S-Gamut3.Cine";
pub const kOfxColourspaceSlog3Sgamut3cineEncoding: &::std::ffi::CStr = c"log";
pub const kOfxColourspaceSlog3Sgamut3cineIsData: bool = false;
pub const kOfxColourspaceSlog3Sgamut3cineIsBasic: bool = false;
pub const kOfxColourspaceSlog3Sgamut3cineIsCore: bool = false;
pub const kOfxColourspaceSlog3Sgamut3cineIsDisplay: bool = false;
pub const kOfxColourspaceSlog3VeniceSgamut3: &::std::ffi::CStr = c"slog3_venice_sgamut3";
pub const kOfxColourspaceSlog3VeniceSgamut3Label: &::std::ffi::CStr = c"S-Log3 Venice S-Gamut3";
pub const kOfxColourspaceSlog3VeniceSgamut3Encoding: &::std::ffi::CStr = c"log";
pub const kOfxColourspaceSlog3VeniceSgamut3IsData: bool = false;
pub const kOfxColourspaceSlog3VeniceSgamut3IsBasic: bool = false;
pub const kOfxColourspaceSlog3VeniceSgamut3IsCore: bool = false;
pub const kOfxColourspaceSlog3VeniceSgamut3IsDisplay: bool = false;
pub const kOfxColourspaceSlog3VeniceSgamut3cine: &::std::ffi::CStr = c"slog3_venice_sgamut3cine";
pub const kOfxColourspaceSlog3VeniceSgamut3cineLabel: &::std::ffi::CStr = c"S-Log3 Venice S-Gamut3.Cine";
pub const kOfxColourspaceSlog3VeniceSgamut3cineEncoding: &::std::ffi::CStr = c"log";
pub const kOfxColourspaceSlog3VeniceSgamut3cineIsData: bool = false;
pub const kOfxColourspaceSlog3VeniceSgamut3cineIsBasic: bool = false;
pub const kOfxColourspaceSlog3VeniceSgamut3cineIsCore: bool = false;
pub const kOfxColourspaceSlog3VeniceSgamut3cineIsDisplay: bool = false;
pub const kOfxColourspaceCameraRec709: &::std::ffi::CStr = c"camera_rec709";
pub const kOfxColourspaceCameraRec709Label: &::std::ffi::CStr = c"Camera Rec.709";
pub const kOfxColourspaceCameraRec709Encoding: &::std::ffi::CStr = c"sdr-video";
pub const kOfxColourspaceCameraRec709IsData: bool = false;
pub const kOfxColourspaceCameraRec709IsBasic: bool = false;
pub const kOfxColourspaceCameraRec709IsCore: bool = false;
pub const kOfxColourspaceCameraRec709IsDisplay: bool = false;
/// ```doxygen
/// @brief aces_interchange
/// Guaranteed to be ACES2065-1.
/// ```
pub const kOfxColourspaceRoleAcesInterchange: &::std::ffi::CStr = c"aces_interchange";
pub const kOfxColourspaceRoleAcesInterchangeIsBasic: bool = false;
pub const kOfxColourspaceRoleAcesInterchangeIsCore: bool = true;
/// ```doxygen
/// @brief cie_xyz_d65_interchange
/// CIE XYZ colorimetry with the neutral axis at D65.
/// ```
pub const kOfxColourspaceRoleCieXyzD65Interchange: &::std::ffi::CStr = c"cie_xyz_d65_interchange";
pub const kOfxColourspaceRoleCieXyzD65InterchangeIsBasic: bool = false;
pub const kOfxColourspaceRoleCieXyzD65InterchangeIsCore: bool = true;
/// ```doxygen
/// @brief color_picking
/// The colourspace to use for colour pickers, typically a display colourspace.
/// ```
pub const kOfxColourspaceRoleColorPicking: &::std::ffi::CStr = c"color_picking";
pub const kOfxColourspaceRoleColorPickingIsBasic: bool = false;
pub const kOfxColourspaceRoleColorPickingIsCore: bool = true;
/// ```doxygen
/// @brief color_timing
/// A colourspace suitable for colour grading, typically a log colourspace.
/// ```
pub const kOfxColourspaceRoleColorTiming: &::std::ffi::CStr = c"color_timing";
pub const kOfxColourspaceRoleColorTimingIsBasic: bool = false;
pub const kOfxColourspaceRoleColorTimingIsCore: bool = true;
/// ```doxygen
/// @brief compositing_log
/// Any scene-referred colourspace with a log transfer function.
/// ```
pub const kOfxColourspaceRoleCompositingLog: &::std::ffi::CStr = c"compositing_log";
pub const kOfxColourspaceRoleCompositingLogIsBasic: bool = false;
pub const kOfxColourspaceRoleCompositingLogIsCore: bool = true;
/// ```doxygen
/// @brief data
/// Image values should not be treated as colour, e.g. motion vectors or masks. Mapped to the raw colourspace.
/// ```
pub const kOfxColourspaceRoleData: &::std::ffi::CStr = c"data";
pub const kOfxColourspaceRoleDataIsBasic: bool = false;
pub const kOfxColourspaceRoleDataIsCore: bool = true;
/// ```doxygen
/// @brief matte_paint
/// A colourspace suitable for matte painting.
/// ```
pub const kOfxColourspaceRoleMattePaint: &::std::ffi::CStr = c"matte_paint";
pub const kOfxColourspaceRoleMattePaintIsBasic: bool = false;
pub const kOfxColourspaceRoleMattePaintIsCore: bool = true;
/// ```doxygen
/// @brief scene_linear
/// Any scene-referred linear colourspace.
/// ```
pub const kOfxColourspaceRoleSceneLinear: &::std::ffi::CStr = c"scene_linear";
pub const kOfxColourspaceRoleSceneLinearIsBasic: bool = false;
pub const kOfxColourspaceRoleSceneLinearIsCore: bool = true;
/// ```doxygen
/// @brief texture_paint
/// A colourspace suitable for texture painting, typically sRGB.
/// ```
pub const kOfxColourspaceRoleTexturePaint: &::std::ffi::CStr = c"texture_paint";
pub const kOfxColourspaceRoleTexturePaintIsBasic: bool = false;
pub const kOfxColourspaceRoleTexturePaintIsCore: bool = true;

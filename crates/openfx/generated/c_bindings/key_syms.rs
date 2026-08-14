use super::core::{
    OfxHost, OfxPlugin, OfxPluginEntryPoint, OfxPointD, OfxPointI, OfxPropertySetHandle,
    OfxPropertySetStruct, OfxRangeD, OfxRangeI, OfxRectD, OfxRectI, OfxStatus, OfxTime,
    kOfxActionBeginInstanceChanged, kOfxActionBeginInstanceEdit,
    kOfxActionCreateInstance, kOfxActionDescribe, kOfxActionDestroyInstance,
    kOfxActionEndInstanceChanged, kOfxActionEndInstanceEdit, kOfxActionInstanceChanged,
    kOfxActionLoad, kOfxActionPurgeCaches, kOfxActionSyncPrivateData, kOfxActionUnload,
    kOfxBitDepthByte, kOfxBitDepthFloat, kOfxBitDepthHalf, kOfxBitDepthNone,
    kOfxBitDepthShort, kOfxChangePluginEdited, kOfxChangeTime, kOfxChangeUserEdited,
    kOfxFlagInfiniteMax, kOfxFlagInfiniteMin, kOfxPluginPropFilePath, kOfxPropAPIVersion,
    kOfxPropChangeReason, kOfxPropEffectInstance, kOfxPropHostOSHandle, kOfxPropIcon,
    kOfxPropInstanceData, kOfxPropIsInteractive, kOfxPropLabel, kOfxPropLongLabel,
    kOfxPropName, kOfxPropPluginDescription, kOfxPropShortLabel, kOfxPropTime,
    kOfxPropType, kOfxPropVersion, kOfxPropVersionLabel,
};
pub const kOfxPropKeySym: &::std::ffi::CStr = c"kOfxPropKeySym";
pub const kOfxPropKeyString: &::std::ffi::CStr = c"kOfxPropKeyString";
pub const kOfxKey_Unknown: u32 = 0;
pub const kOfxKey_BackSpace: u32 = 65288;
pub const kOfxKey_Tab: u32 = 65289;
pub const kOfxKey_Linefeed: u32 = 65290;
pub const kOfxKey_Clear: u32 = 65291;
pub const kOfxKey_Return: u32 = 65293;
pub const kOfxKey_Pause: u32 = 65299;
pub const kOfxKey_Scroll_Lock: u32 = 65300;
pub const kOfxKey_Sys_Req: u32 = 65301;
pub const kOfxKey_Escape: u32 = 65307;
pub const kOfxKey_Delete: u32 = 65535;
pub const kOfxKey_Multi_key: u32 = 65312;
pub const kOfxKey_SingleCandidate: u32 = 65340;
pub const kOfxKey_MultipleCandidate: u32 = 65341;
pub const kOfxKey_PreviousCandidate: u32 = 65342;
pub const kOfxKey_Kanji: u32 = 65313;
pub const kOfxKey_Muhenkan: u32 = 65314;
pub const kOfxKey_Henkan_Mode: u32 = 65315;
pub const kOfxKey_Henkan: u32 = 65315;
pub const kOfxKey_Romaji: u32 = 65316;
pub const kOfxKey_Hiragana: u32 = 65317;
pub const kOfxKey_Katakana: u32 = 65318;
pub const kOfxKey_Hiragana_Katakana: u32 = 65319;
pub const kOfxKey_Zenkaku: u32 = 65320;
pub const kOfxKey_Hankaku: u32 = 65321;
pub const kOfxKey_Zenkaku_Hankaku: u32 = 65322;
pub const kOfxKey_Touroku: u32 = 65323;
pub const kOfxKey_Massyo: u32 = 65324;
pub const kOfxKey_Kana_Lock: u32 = 65325;
pub const kOfxKey_Kana_Shift: u32 = 65326;
pub const kOfxKey_Eisu_Shift: u32 = 65327;
pub const kOfxKey_Eisu_toggle: u32 = 65328;
pub const kOfxKey_Zen_Koho: u32 = 65341;
pub const kOfxKey_Mae_Koho: u32 = 65342;
pub const kOfxKey_Home: u32 = 65360;
pub const kOfxKey_Left: u32 = 65361;
pub const kOfxKey_Up: u32 = 65362;
pub const kOfxKey_Right: u32 = 65363;
pub const kOfxKey_Down: u32 = 65364;
pub const kOfxKey_Prior: u32 = 65365;
pub const kOfxKey_Page_Up: u32 = 65365;
pub const kOfxKey_Next: u32 = 65366;
pub const kOfxKey_Page_Down: u32 = 65366;
pub const kOfxKey_End: u32 = 65367;
pub const kOfxKey_Begin: u32 = 65368;
pub const kOfxKey_Select: u32 = 65376;
pub const kOfxKey_Print: u32 = 65377;
pub const kOfxKey_Execute: u32 = 65378;
pub const kOfxKey_Insert: u32 = 65379;
pub const kOfxKey_Undo: u32 = 65381;
pub const kOfxKey_Redo: u32 = 65382;
pub const kOfxKey_Menu: u32 = 65383;
pub const kOfxKey_Find: u32 = 65384;
pub const kOfxKey_Cancel: u32 = 65385;
pub const kOfxKey_Help: u32 = 65386;
pub const kOfxKey_Break: u32 = 65387;
pub const kOfxKey_Mode_switch: u32 = 65406;
pub const kOfxKey_script_switch: u32 = 65406;
pub const kOfxKey_Num_Lock: u32 = 65407;
pub const kOfxKey_KP_Space: u32 = 65408;
pub const kOfxKey_KP_Tab: u32 = 65417;
pub const kOfxKey_KP_Enter: u32 = 65421;
pub const kOfxKey_KP_F1: u32 = 65425;
pub const kOfxKey_KP_F2: u32 = 65426;
pub const kOfxKey_KP_F3: u32 = 65427;
pub const kOfxKey_KP_F4: u32 = 65428;
pub const kOfxKey_KP_Home: u32 = 65429;
pub const kOfxKey_KP_Left: u32 = 65430;
pub const kOfxKey_KP_Up: u32 = 65431;
pub const kOfxKey_KP_Right: u32 = 65432;
pub const kOfxKey_KP_Down: u32 = 65433;
pub const kOfxKey_KP_Prior: u32 = 65434;
pub const kOfxKey_KP_Page_Up: u32 = 65434;
pub const kOfxKey_KP_Next: u32 = 65435;
pub const kOfxKey_KP_Page_Down: u32 = 65435;
pub const kOfxKey_KP_End: u32 = 65436;
pub const kOfxKey_KP_Begin: u32 = 65437;
pub const kOfxKey_KP_Insert: u32 = 65438;
pub const kOfxKey_KP_Delete: u32 = 65439;
pub const kOfxKey_KP_Equal: u32 = 65469;
pub const kOfxKey_KP_Multiply: u32 = 65450;
pub const kOfxKey_KP_Add: u32 = 65451;
pub const kOfxKey_KP_Separator: u32 = 65452;
pub const kOfxKey_KP_Subtract: u32 = 65453;
pub const kOfxKey_KP_Decimal: u32 = 65454;
pub const kOfxKey_KP_Divide: u32 = 65455;
pub const kOfxKey_KP_0: u32 = 65456;
pub const kOfxKey_KP_1: u32 = 65457;
pub const kOfxKey_KP_2: u32 = 65458;
pub const kOfxKey_KP_3: u32 = 65459;
pub const kOfxKey_KP_4: u32 = 65460;
pub const kOfxKey_KP_5: u32 = 65461;
pub const kOfxKey_KP_6: u32 = 65462;
pub const kOfxKey_KP_7: u32 = 65463;
pub const kOfxKey_KP_8: u32 = 65464;
pub const kOfxKey_KP_9: u32 = 65465;
pub const kOfxKey_F1: u32 = 65470;
pub const kOfxKey_F2: u32 = 65471;
pub const kOfxKey_F3: u32 = 65472;
pub const kOfxKey_F4: u32 = 65473;
pub const kOfxKey_F5: u32 = 65474;
pub const kOfxKey_F6: u32 = 65475;
pub const kOfxKey_F7: u32 = 65476;
pub const kOfxKey_F8: u32 = 65477;
pub const kOfxKey_F9: u32 = 65478;
pub const kOfxKey_F10: u32 = 65479;
pub const kOfxKey_F11: u32 = 65480;
pub const kOfxKey_L1: u32 = 65480;
pub const kOfxKey_F12: u32 = 65481;
pub const kOfxKey_L2: u32 = 65481;
pub const kOfxKey_F13: u32 = 65482;
pub const kOfxKey_L3: u32 = 65482;
pub const kOfxKey_F14: u32 = 65483;
pub const kOfxKey_L4: u32 = 65483;
pub const kOfxKey_F15: u32 = 65484;
pub const kOfxKey_L5: u32 = 65484;
pub const kOfxKey_F16: u32 = 65485;
pub const kOfxKey_L6: u32 = 65485;
pub const kOfxKey_F17: u32 = 65486;
pub const kOfxKey_L7: u32 = 65486;
pub const kOfxKey_F18: u32 = 65487;
pub const kOfxKey_L8: u32 = 65487;
pub const kOfxKey_F19: u32 = 65488;
pub const kOfxKey_L9: u32 = 65488;
pub const kOfxKey_F20: u32 = 65489;
pub const kOfxKey_L10: u32 = 65489;
pub const kOfxKey_F21: u32 = 65490;
pub const kOfxKey_R1: u32 = 65490;
pub const kOfxKey_F22: u32 = 65491;
pub const kOfxKey_R2: u32 = 65491;
pub const kOfxKey_F23: u32 = 65492;
pub const kOfxKey_R3: u32 = 65492;
pub const kOfxKey_F24: u32 = 65493;
pub const kOfxKey_R4: u32 = 65493;
pub const kOfxKey_F25: u32 = 65494;
pub const kOfxKey_R5: u32 = 65494;
pub const kOfxKey_F26: u32 = 65495;
pub const kOfxKey_R6: u32 = 65495;
pub const kOfxKey_F27: u32 = 65496;
pub const kOfxKey_R7: u32 = 65496;
pub const kOfxKey_F28: u32 = 65497;
pub const kOfxKey_R8: u32 = 65497;
pub const kOfxKey_F29: u32 = 65498;
pub const kOfxKey_R9: u32 = 65498;
pub const kOfxKey_F30: u32 = 65499;
pub const kOfxKey_R10: u32 = 65499;
pub const kOfxKey_F31: u32 = 65500;
pub const kOfxKey_R11: u32 = 65500;
pub const kOfxKey_F32: u32 = 65501;
pub const kOfxKey_R12: u32 = 65501;
pub const kOfxKey_F33: u32 = 65502;
pub const kOfxKey_R13: u32 = 65502;
pub const kOfxKey_F34: u32 = 65503;
pub const kOfxKey_R14: u32 = 65503;
pub const kOfxKey_F35: u32 = 65504;
pub const kOfxKey_R15: u32 = 65504;
pub const kOfxKey_Shift_L: u32 = 65505;
pub const kOfxKey_Shift_R: u32 = 65506;
pub const kOfxKey_Control_L: u32 = 65507;
pub const kOfxKey_Control_R: u32 = 65508;
pub const kOfxKey_Caps_Lock: u32 = 65509;
pub const kOfxKey_Shift_Lock: u32 = 65510;
pub const kOfxKey_Meta_L: u32 = 65511;
pub const kOfxKey_Meta_R: u32 = 65512;
pub const kOfxKey_Alt_L: u32 = 65513;
pub const kOfxKey_Alt_R: u32 = 65514;
pub const kOfxKey_Super_L: u32 = 65515;
pub const kOfxKey_Super_R: u32 = 65516;
pub const kOfxKey_Hyper_L: u32 = 65517;
pub const kOfxKey_Hyper_R: u32 = 65518;
pub const kOfxKey_space: u32 = 32;
pub const kOfxKey_exclam: u32 = 33;
pub const kOfxKey_quotedbl: u32 = 34;
pub const kOfxKey_numbersign: u32 = 35;
pub const kOfxKey_dollar: u32 = 36;
pub const kOfxKey_percent: u32 = 37;
pub const kOfxKey_ampersand: u32 = 38;
pub const kOfxKey_apostrophe: u32 = 39;
pub const kOfxKey_quoteright: u32 = 39;
pub const kOfxKey_parenleft: u32 = 40;
pub const kOfxKey_parenright: u32 = 41;
pub const kOfxKey_asterisk: u32 = 42;
pub const kOfxKey_plus: u32 = 43;
pub const kOfxKey_comma: u32 = 44;
pub const kOfxKey_minus: u32 = 45;
pub const kOfxKey_period: u32 = 46;
pub const kOfxKey_slash: u32 = 47;
pub const kOfxKey_0: u32 = 48;
pub const kOfxKey_1: u32 = 49;
pub const kOfxKey_2: u32 = 50;
pub const kOfxKey_3: u32 = 51;
pub const kOfxKey_4: u32 = 52;
pub const kOfxKey_5: u32 = 53;
pub const kOfxKey_6: u32 = 54;
pub const kOfxKey_7: u32 = 55;
pub const kOfxKey_8: u32 = 56;
pub const kOfxKey_9: u32 = 57;
pub const kOfxKey_colon: u32 = 58;
pub const kOfxKey_semicolon: u32 = 59;
pub const kOfxKey_less: u32 = 60;
pub const kOfxKey_equal: u32 = 61;
pub const kOfxKey_greater: u32 = 62;
pub const kOfxKey_question: u32 = 63;
pub const kOfxKey_at: u32 = 64;
pub const kOfxKey_A: u32 = 65;
pub const kOfxKey_B: u32 = 66;
pub const kOfxKey_C: u32 = 67;
pub const kOfxKey_D: u32 = 68;
pub const kOfxKey_E: u32 = 69;
pub const kOfxKey_F: u32 = 70;
pub const kOfxKey_G: u32 = 71;
pub const kOfxKey_H: u32 = 72;
pub const kOfxKey_I: u32 = 73;
pub const kOfxKey_J: u32 = 74;
pub const kOfxKey_K: u32 = 75;
pub const kOfxKey_L: u32 = 76;
pub const kOfxKey_M: u32 = 77;
pub const kOfxKey_N: u32 = 78;
pub const kOfxKey_O: u32 = 79;
pub const kOfxKey_P: u32 = 80;
pub const kOfxKey_Q: u32 = 81;
pub const kOfxKey_R: u32 = 82;
pub const kOfxKey_S: u32 = 83;
pub const kOfxKey_T: u32 = 84;
pub const kOfxKey_U: u32 = 85;
pub const kOfxKey_V: u32 = 86;
pub const kOfxKey_W: u32 = 87;
pub const kOfxKey_X: u32 = 88;
pub const kOfxKey_Y: u32 = 89;
pub const kOfxKey_Z: u32 = 90;
pub const kOfxKey_bracketleft: u32 = 91;
pub const kOfxKey_backslash: u32 = 92;
pub const kOfxKey_bracketright: u32 = 93;
pub const kOfxKey_asciicircum: u32 = 94;
pub const kOfxKey_underscore: u32 = 95;
pub const kOfxKey_grave: u32 = 96;
pub const kOfxKey_quoteleft: u32 = 96;
pub const kOfxKey_a: u32 = 97;
pub const kOfxKey_b: u32 = 98;
pub const kOfxKey_c: u32 = 99;
pub const kOfxKey_d: u32 = 100;
pub const kOfxKey_e: u32 = 101;
pub const kOfxKey_f: u32 = 102;
pub const kOfxKey_g: u32 = 103;
pub const kOfxKey_h: u32 = 104;
pub const kOfxKey_i: u32 = 105;
pub const kOfxKey_j: u32 = 106;
pub const kOfxKey_k: u32 = 107;
pub const kOfxKey_l: u32 = 108;
pub const kOfxKey_m: u32 = 109;
pub const kOfxKey_n: u32 = 110;
pub const kOfxKey_o: u32 = 111;
pub const kOfxKey_p: u32 = 112;
pub const kOfxKey_q: u32 = 113;
pub const kOfxKey_r: u32 = 114;
pub const kOfxKey_s: u32 = 115;
pub const kOfxKey_t: u32 = 116;
pub const kOfxKey_u: u32 = 117;
pub const kOfxKey_v: u32 = 118;
pub const kOfxKey_w: u32 = 119;
pub const kOfxKey_x: u32 = 120;
pub const kOfxKey_y: u32 = 121;
pub const kOfxKey_z: u32 = 122;
pub const kOfxKey_braceleft: u32 = 123;
pub const kOfxKey_bar: u32 = 124;
pub const kOfxKey_braceright: u32 = 125;
pub const kOfxKey_asciitilde: u32 = 126;
pub const kOfxKey_nobreakspace: u32 = 160;
pub const kOfxKey_exclamdown: u32 = 161;
pub const kOfxKey_cent: u32 = 162;
pub const kOfxKey_sterling: u32 = 163;
pub const kOfxKey_currency: u32 = 164;
pub const kOfxKey_yen: u32 = 165;
pub const kOfxKey_brokenbar: u32 = 166;
pub const kOfxKey_section: u32 = 167;
pub const kOfxKey_diaeresis: u32 = 168;
pub const kOfxKey_copyright: u32 = 169;
pub const kOfxKey_ordfeminine: u32 = 170;
pub const kOfxKey_guillemotleft: u32 = 171;
pub const kOfxKey_notsign: u32 = 172;
pub const kOfxKey_hyphen: u32 = 173;
pub const kOfxKey_registered: u32 = 174;
pub const kOfxKey_macron: u32 = 175;
pub const kOfxKey_degree: u32 = 176;
pub const kOfxKey_plusminus: u32 = 177;
pub const kOfxKey_twosuperior: u32 = 178;
pub const kOfxKey_threesuperior: u32 = 179;
pub const kOfxKey_acute: u32 = 180;
pub const kOfxKey_mu: u32 = 181;
pub const kOfxKey_paragraph: u32 = 182;
pub const kOfxKey_periodcentered: u32 = 183;
pub const kOfxKey_cedilla: u32 = 184;
pub const kOfxKey_onesuperior: u32 = 185;
pub const kOfxKey_masculine: u32 = 186;
pub const kOfxKey_guillemotright: u32 = 187;
pub const kOfxKey_onequarter: u32 = 188;
pub const kOfxKey_onehalf: u32 = 189;
pub const kOfxKey_threequarters: u32 = 190;
pub const kOfxKey_questiondown: u32 = 191;
pub const kOfxKey_Agrave: u32 = 192;
pub const kOfxKey_Aacute: u32 = 193;
pub const kOfxKey_Acircumflex: u32 = 194;
pub const kOfxKey_Atilde: u32 = 195;
pub const kOfxKey_Adiaeresis: u32 = 196;
pub const kOfxKey_Aring: u32 = 197;
pub const kOfxKey_AE: u32 = 198;
pub const kOfxKey_Ccedilla: u32 = 199;
pub const kOfxKey_Egrave: u32 = 200;
pub const kOfxKey_Eacute: u32 = 201;
pub const kOfxKey_Ecircumflex: u32 = 202;
pub const kOfxKey_Ediaeresis: u32 = 203;
pub const kOfxKey_Igrave: u32 = 204;
pub const kOfxKey_Iacute: u32 = 205;
pub const kOfxKey_Icircumflex: u32 = 206;
pub const kOfxKey_Idiaeresis: u32 = 207;
pub const kOfxKey_ETH: u32 = 208;
pub const kOfxKey_Eth: u32 = 208;
pub const kOfxKey_Ntilde: u32 = 209;
pub const kOfxKey_Ograve: u32 = 210;
pub const kOfxKey_Oacute: u32 = 211;
pub const kOfxKey_Ocircumflex: u32 = 212;
pub const kOfxKey_Otilde: u32 = 213;
pub const kOfxKey_Odiaeresis: u32 = 214;
pub const kOfxKey_multiply: u32 = 215;
pub const kOfxKey_Ooblique: u32 = 216;
pub const kOfxKey_Ugrave: u32 = 217;
pub const kOfxKey_Uacute: u32 = 218;
pub const kOfxKey_Ucircumflex: u32 = 219;
pub const kOfxKey_Udiaeresis: u32 = 220;
pub const kOfxKey_Yacute: u32 = 221;
pub const kOfxKey_THORN: u32 = 222;
pub const kOfxKey_ssharp: u32 = 223;
pub const kOfxKey_agrave: u32 = 224;
pub const kOfxKey_aacute: u32 = 225;
pub const kOfxKey_acircumflex: u32 = 226;
pub const kOfxKey_atilde: u32 = 227;
pub const kOfxKey_adiaeresis: u32 = 228;
pub const kOfxKey_aring: u32 = 229;
pub const kOfxKey_ae: u32 = 230;
pub const kOfxKey_ccedilla: u32 = 231;
pub const kOfxKey_egrave: u32 = 232;
pub const kOfxKey_eacute: u32 = 233;
pub const kOfxKey_ecircumflex: u32 = 234;
pub const kOfxKey_ediaeresis: u32 = 235;
pub const kOfxKey_igrave: u32 = 236;
pub const kOfxKey_iacute: u32 = 237;
pub const kOfxKey_icircumflex: u32 = 238;
pub const kOfxKey_idiaeresis: u32 = 239;
pub const kOfxKey_eth: u32 = 240;
pub const kOfxKey_ntilde: u32 = 241;
pub const kOfxKey_ograve: u32 = 242;
pub const kOfxKey_oacute: u32 = 243;
pub const kOfxKey_ocircumflex: u32 = 244;
pub const kOfxKey_otilde: u32 = 245;
pub const kOfxKey_odiaeresis: u32 = 246;
pub const kOfxKey_division: u32 = 247;
pub const kOfxKey_oslash: u32 = 248;
pub const kOfxKey_ugrave: u32 = 249;
pub const kOfxKey_uacute: u32 = 250;
pub const kOfxKey_ucircumflex: u32 = 251;
pub const kOfxKey_udiaeresis: u32 = 252;
pub const kOfxKey_yacute: u32 = 253;
pub const kOfxKey_thorn: u32 = 254;
pub const kOfxKey_ydiaeresis: u32 = 255;
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

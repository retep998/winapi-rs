// Copyright © 2018 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use ctypes::c_float;
use shared::guiddef::GUID;
use shared::minwindef::{BOOL, INT, UINT};
use shared::windef::RECT;
use um::gdipluscolormatrix::ColorChannelLUT;
use um::gdiplustypes::Status;
use um::winnt::VOID;
DEFINE_GUID!{BlurEffectGuid,
    0x633c80a4, 0x1843, 0x482b, 0x9e, 0xf2, 0xbe, 0x28, 0x34, 0xc5, 0xfd, 0xd4}
DEFINE_GUID!{SharpenEffectGuid,
    0x63cbf3ee, 0xc526, 0x402c, 0x8f, 0x71, 0x62, 0xc5, 0x40, 0xbf, 0x51, 0x42}
DEFINE_GUID!{ColorMatrixEffectGuid,
    0x718f2615, 0x7933, 0x40e3, 0xa5, 0x11, 0x5f, 0x68, 0xfe, 0x14, 0xdd, 0x74}
DEFINE_GUID!{ColorLUTEffectGuid,
    0xa7ce72a9, 0xf7f, 0x40d7, 0xb3, 0xcc, 0xd0, 0xc0, 0x2d, 0x5c, 0x32, 0x12}
DEFINE_GUID!{BrightnessContrastEffectGuid,
    0xd3a1dbe1, 0x8ec4, 0x4c17, 0x9f, 0x4c, 0xea, 0x97, 0xad, 0x1c, 0x34, 0x3d}
DEFINE_GUID!{HueSaturationLightnessEffectGuid,
    0x8b2dd6c3, 0xeb07, 0x4d87, 0xa5, 0xf0, 0x71, 0x8, 0xe2, 0x6a, 0x9c, 0x5f}
DEFINE_GUID!{LevelsEffectGuid,
    0x99c354ec, 0x2a31, 0x4f3a, 0x8c, 0x34, 0x17, 0xa8, 0x3, 0xb3, 0x3a, 0x25}
DEFINE_GUID!{TintEffectGuid,
    0x1077af00, 0x2848, 0x4441, 0x94, 0x89, 0x44, 0xad, 0x4c, 0x2d, 0x7a, 0x2c}
DEFINE_GUID!{ColorBalanceEffectGuid,
    0x537e597d, 0x251e, 0x48da, 0x96, 0x64, 0x29, 0xca, 0x49, 0x6b, 0x70, 0xf8}
DEFINE_GUID!{RedEyeCorrectionEffectGuid,
    0x74d29d05, 0x69a4, 0x4266, 0x95, 0x49, 0x3c, 0xc5, 0x28, 0x36, 0xb6, 0x32}
DEFINE_GUID!{ColorCurveEffectGuid,
    0xdd6a0022, 0x58e4, 0x4a67, 0x9d, 0x9b, 0xd4, 0x8e, 0xb8, 0x81, 0xa5, 0x3d}
STRUCT!{struct SharpenParams {
    radius: c_float,
    amount: c_float,
}}
STRUCT!{struct BlurParams {
    radius: c_float,
    expandEdge: BOOL,
}}
STRUCT!{struct BrightnessContrastParams {
    brightnessLevel: INT,
    contrastLevel: INT,
}}
STRUCT!{struct RedEyeCorrectionParams {
    numberOfAreas: UINT,
    areas: *mut RECT,
}}
STRUCT!{struct HueSaturationLightnessParams {
    hueLevel: INT,
    saturationLevel: INT,
    lightnessLevel: INT,
}}
STRUCT!{struct TintParams {
    hue: INT,
    amount: INT,
}}
STRUCT!{struct LevelsParams {
    highlight: INT,
    midtone: INT,
    shadow: INT,
}}
STRUCT!{struct ColorBalanceParams {
    cyanRed: INT,
    magentaGreen: INT,
    yellowBlue: INT,
}}
STRUCT!{struct ColorLUTParams {
    lutB: ColorChannelLUT,
    lutG: ColorChannelLUT,
    lutR: ColorChannelLUT,
    lutA: ColorChannelLUT,
}}
ENUM!{enum CurveAdjustments {
    AdjustExposure,
    AdjustDensity,
    AdjustContrast,
    AdjustHighlight,
    AdjustShadow,
    AdjustMidtone,
    AdjustWhiteSaturation,
    AdjustBlackSaturation,
}}
ENUM!{enum CurveChannel {
    CurveChannelAll,
    CurveChannelRed,
    CurveChannelGreen,
    CurveChannelBlue,
}}
STRUCT!{struct ColorCurveParams {
    adjustment: CurveAdjustments,
    channel: CurveChannel,
    adjustValue: INT,
}}
pub enum CGpEffect {}
extern "system" {
    pub fn GdipCreateEffect(
        guid: GUID,
        effect: *mut *mut CGpEffect,
    ) -> Status;
    pub fn GdipDeleteEffect(
        effect: *mut CGpEffect,
    ) -> Status;
    pub fn GdipGetEffectParameterSize(
        effect: *mut CGpEffect,
        size: *mut UINT,
    ) -> Status;
    pub fn GdipSetEffectParameters(
        effect: *mut CGpEffect,
        params: *const VOID,
        size: UINT,
    ) -> Status;
    pub fn GdipGetEffectParameters(
        effect: *mut CGpEffect,
        size: *mut UINT,
        params: *mut VOID,
    ) -> Status;
}

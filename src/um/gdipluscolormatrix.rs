// Copyright © 2018 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::minwindef::BYTE;
use um::gdipluscolor::Color;
use um::gdiplustypes::REAL;
pub type ColorChannelLUT = [BYTE; 256];
ENUM!{enum HistogramFormat {
    HistogramFormatARGB,
    HistogramFormatPARGB,
    HistogramFormatRGB,
    HistogramFormatGray,
    HistogramFormatB,
    HistogramFormatG,
    HistogramFormatR,
    HistogramFormatA,
}}
STRUCT!{struct ColorMatrix {
    m: [[REAL; 5]; 5],
}}
ENUM!{enum ColorMatrixFlags {
    ColorMatrixFlagsDefault = 0,
    ColorMatrixFlagsSkipGrays = 1,
    ColorMatrixFlagsAltGray = 2,
}}
ENUM!{enum ColorAdjustType {
    ColorAdjustTypeDefault,
    ColorAdjustTypeBitmap,
    ColorAdjustTypeBrush,
    ColorAdjustTypePen,
    ColorAdjustTypeText,
    ColorAdjustTypeCount,
    ColorAdjustTypeAny,
}}
STRUCT!{struct ColorMap {
    oldColor: Color,
    newColor: Color,
}}

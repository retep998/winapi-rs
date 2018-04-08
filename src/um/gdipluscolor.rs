// Copyright © 2015-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::minwindef::BYTE;
use shared::windef::COLORREF;
use um::gdipluspixelformats::ARGB;
use um::wingdi::{GetBValue, GetGValue, GetRValue, RGB};
ENUM!{enum ColorMode {
    ColorModeARGB32 = 0,
    ColorModeARGB64 = 1,
}}
ENUM!{enum ColorChannelFlags {
    ColorChannelFlagsC = 0,
    ColorChannelFlagsM,
    ColorChannelFlagsY,
    ColorChannelFlagsK,
    ColorChannelFlagsLast,
}}
#[repr(C)]
pub struct Color {
    Argb: ARGB,
}
impl Color {
    pub fn new() -> Self {
        Self { Argb: Color::Black }
    }
    pub fn from_rgb_parts(r: BYTE, g: BYTE, b: BYTE) -> Self {
        Self { Argb: Self::MakeARGB(255, r, g, b) }
    }
    pub fn from_argb_parts(a: BYTE, r: BYTE, g: BYTE, b: BYTE) -> Self {
        Self { Argb: Self::MakeARGB(a, r, g, b) }
    }
    pub fn from_argb(argb: ARGB) -> Self {
        Self { Argb: argb }
    }
    pub fn GetAlpha(&self) -> BYTE {
        (self.Argb >> Self::AlphaShift) as BYTE
    }
    pub fn GetRed(&self) -> BYTE {
        (self.Argb >> Self::RedShift) as BYTE
    }
    pub fn GetGreen(&self) -> BYTE {
        (self.Argb >> Self::GreenShift) as BYTE
    }
    pub fn GetBlue(&self) -> BYTE {
        (self.Argb >> Self::BlueShift) as BYTE
    }
    pub fn GetA(&self) -> BYTE {
        self.GetAlpha()
    }
    pub fn GetR(&self) -> BYTE {
        self.GetRed()
    }
    pub fn GetG(&self) -> BYTE {
        self.GetGreen()
    }
    pub fn GetB(&self) -> BYTE {
        self.GetBlue()
    }
    pub fn GetValue(&self) -> ARGB {
        self.Argb
    }
    pub fn SetValue(&mut self, argb: ARGB) {
        self.Argb = argb;
    }
    pub fn SetFromCOLORREF(&mut self, rgb: COLORREF) {
        self.Argb = Self::MakeARGB(255, GetRValue(rgb), GetGValue(rgb), GetBValue(rgb));
    }
    pub fn ToCOLORREF(&self) -> COLORREF {
        RGB(self.GetRed(), self.GetGreen(), self.GetBlue())
    }
    pub const AliceBlue: ARGB = 0xFFF0F8FF;
    pub const AntiqueWhite: ARGB = 0xFFFAEBD7;
    pub const Aqua: ARGB = 0xFF00FFFF;
    pub const Aquamarine: ARGB = 0xFF7FFFD4;
    pub const Azure: ARGB = 0xFFF0FFFF;
    pub const Beige: ARGB = 0xFFF5F5DC;
    pub const Bisque: ARGB = 0xFFFFE4C4;
    pub const Black: ARGB = 0xFF000000;
    pub const BlanchedAlmond: ARGB = 0xFFFFEBCD;
    pub const Blue: ARGB = 0xFF0000FF;
    pub const BlueViolet: ARGB = 0xFF8A2BE2;
    pub const Brown: ARGB = 0xFFA52A2A;
    pub const BurlyWood: ARGB = 0xFFDEB887;
    pub const CadetBlue: ARGB = 0xFF5F9EA0;
    pub const Chartreuse: ARGB = 0xFF7FFF00;
    pub const Chocolate: ARGB = 0xFFD2691E;
    pub const Coral: ARGB = 0xFFFF7F50;
    pub const CornflowerBlue: ARGB = 0xFF6495ED;
    pub const Cornsilk: ARGB = 0xFFFFF8DC;
    pub const Crimson: ARGB = 0xFFDC143C;
    pub const Cyan: ARGB = 0xFF00FFFF;
    pub const DarkBlue: ARGB = 0xFF00008B;
    pub const DarkCyan: ARGB = 0xFF008B8B;
    pub const DarkGoldenrod: ARGB = 0xFFB8860B;
    pub const DarkGray: ARGB = 0xFFA9A9A9;
    pub const DarkGreen: ARGB = 0xFF006400;
    pub const DarkKhaki: ARGB = 0xFFBDB76B;
    pub const DarkMagenta: ARGB = 0xFF8B008B;
    pub const DarkOliveGreen: ARGB = 0xFF556B2F;
    pub const DarkOrange: ARGB = 0xFFFF8C00;
    pub const DarkOrchid: ARGB = 0xFF9932CC;
    pub const DarkRed: ARGB = 0xFF8B0000;
    pub const DarkSalmon: ARGB = 0xFFE9967A;
    pub const DarkSeaGreen: ARGB = 0xFF8FBC8B;
    pub const DarkSlateBlue: ARGB = 0xFF483D8B;
    pub const DarkSlateGray: ARGB = 0xFF2F4F4F;
    pub const DarkTurquoise: ARGB = 0xFF00CED1;
    pub const DarkViolet: ARGB = 0xFF9400D3;
    pub const DeepPink: ARGB = 0xFFFF1493;
    pub const DeepSkyBlue: ARGB = 0xFF00BFFF;
    pub const DimGray: ARGB = 0xFF696969;
    pub const DodgerBlue: ARGB = 0xFF1E90FF;
    pub const Firebrick: ARGB = 0xFFB22222;
    pub const FloralWhite: ARGB = 0xFFFFFAF0;
    pub const ForestGreen: ARGB = 0xFF228B22;
    pub const Fuchsia: ARGB = 0xFFFF00FF;
    pub const Gainsboro: ARGB = 0xFFDCDCDC;
    pub const GhostWhite: ARGB = 0xFFF8F8FF;
    pub const Gold: ARGB = 0xFFFFD700;
    pub const Goldenrod: ARGB = 0xFFDAA520;
    pub const Gray: ARGB = 0xFF808080;
    pub const Green: ARGB = 0xFF008000;
    pub const GreenYellow: ARGB = 0xFFADFF2F;
    pub const Honeydew: ARGB = 0xFFF0FFF0;
    pub const HotPink: ARGB = 0xFFFF69B4;
    pub const IndianRed: ARGB = 0xFFCD5C5C;
    pub const Indigo: ARGB = 0xFF4B0082;
    pub const Ivory: ARGB = 0xFFFFFFF0;
    pub const Khaki: ARGB = 0xFFF0E68C;
    pub const Lavender: ARGB = 0xFFE6E6FA;
    pub const LavenderBlush: ARGB = 0xFFFFF0F5;
    pub const LawnGreen: ARGB = 0xFF7CFC00;
    pub const LemonChiffon: ARGB = 0xFFFFFACD;
    pub const LightBlue: ARGB = 0xFFADD8E6;
    pub const LightCoral: ARGB = 0xFFF08080;
    pub const LightCyan: ARGB = 0xFFE0FFFF;
    pub const LightGoldenrodYellow: ARGB = 0xFFFAFAD2;
    pub const LightGray: ARGB = 0xFFD3D3D3;
    pub const LightGreen: ARGB = 0xFF90EE90;
    pub const LightPink: ARGB = 0xFFFFB6C1;
    pub const LightSalmon: ARGB = 0xFFFFA07A;
    pub const LightSeaGreen: ARGB = 0xFF20B2AA;
    pub const LightSkyBlue: ARGB = 0xFF87CEFA;
    pub const LightSlateGray: ARGB = 0xFF778899;
    pub const LightSteelBlue: ARGB = 0xFFB0C4DE;
    pub const LightYellow: ARGB = 0xFFFFFFE0;
    pub const Lime: ARGB = 0xFF00FF00;
    pub const LimeGreen: ARGB = 0xFF32CD32;
    pub const Linen: ARGB = 0xFFFAF0E6;
    pub const Magenta: ARGB = 0xFFFF00FF;
    pub const Maroon: ARGB = 0xFF800000;
    pub const MediumAquamarine: ARGB = 0xFF66CDAA;
    pub const MediumBlue: ARGB = 0xFF0000CD;
    pub const MediumOrchid: ARGB = 0xFFBA55D3;
    pub const MediumPurple: ARGB = 0xFF9370DB;
    pub const MediumSeaGreen: ARGB = 0xFF3CB371;
    pub const MediumSlateBlue: ARGB = 0xFF7B68EE;
    pub const MediumSpringGreen: ARGB = 0xFF00FA9A;
    pub const MediumTurquoise: ARGB = 0xFF48D1CC;
    pub const MediumVioletRed: ARGB = 0xFFC71585;
    pub const MidnightBlue: ARGB = 0xFF191970;
    pub const MintCream: ARGB = 0xFFF5FFFA;
    pub const MistyRose: ARGB = 0xFFFFE4E1;
    pub const Moccasin: ARGB = 0xFFFFE4B5;
    pub const NavajoWhite: ARGB = 0xFFFFDEAD;
    pub const Navy: ARGB = 0xFF000080;
    pub const OldLace: ARGB = 0xFFFDF5E6;
    pub const Olive: ARGB = 0xFF808000;
    pub const OliveDrab: ARGB = 0xFF6B8E23;
    pub const Orange: ARGB = 0xFFFFA500;
    pub const OrangeRed: ARGB = 0xFFFF4500;
    pub const Orchid: ARGB = 0xFFDA70D6;
    pub const PaleGoldenrod: ARGB = 0xFFEEE8AA;
    pub const PaleGreen: ARGB = 0xFF98FB98;
    pub const PaleTurquoise: ARGB = 0xFFAFEEEE;
    pub const PaleVioletRed: ARGB = 0xFFDB7093;
    pub const PapayaWhip: ARGB = 0xFFFFEFD5;
    pub const PeachPuff: ARGB = 0xFFFFDAB9;
    pub const Peru: ARGB = 0xFFCD853F;
    pub const Pink: ARGB = 0xFFFFC0CB;
    pub const Plum: ARGB = 0xFFDDA0DD;
    pub const PowderBlue: ARGB = 0xFFB0E0E6;
    pub const Purple: ARGB = 0xFF800080;
    pub const Red: ARGB = 0xFFFF0000;
    pub const RosyBrown: ARGB = 0xFFBC8F8F;
    pub const RoyalBlue: ARGB = 0xFF4169E1;
    pub const SaddleBrown: ARGB = 0xFF8B4513;
    pub const Salmon: ARGB = 0xFFFA8072;
    pub const SandyBrown: ARGB = 0xFFF4A460;
    pub const SeaGreen: ARGB = 0xFF2E8B57;
    pub const SeaShell: ARGB = 0xFFFFF5EE;
    pub const Sienna: ARGB = 0xFFA0522D;
    pub const Silver: ARGB = 0xFFC0C0C0;
    pub const SkyBlue: ARGB = 0xFF87CEEB;
    pub const SlateBlue: ARGB = 0xFF6A5ACD;
    pub const SlateGray: ARGB = 0xFF708090;
    pub const Snow: ARGB = 0xFFFFFAFA;
    pub const SpringGreen: ARGB = 0xFF00FF7F;
    pub const SteelBlue: ARGB = 0xFF4682B4;
    pub const Tan: ARGB = 0xFFD2B48C;
    pub const Teal: ARGB = 0xFF008080;
    pub const Thistle: ARGB = 0xFFD8BFD8;
    pub const Tomato: ARGB = 0xFFFF6347;
    pub const Transparent: ARGB = 0x00FFFFFF;
    pub const Turquoise: ARGB = 0xFF40E0D0;
    pub const Violet: ARGB = 0xFFEE82EE;
    pub const Wheat: ARGB = 0xFFF5DEB3;
    pub const White: ARGB = 0xFFFFFFFF;
    pub const WhiteSmoke: ARGB = 0xFFF5F5F5;
    pub const Yellow: ARGB = 0xFFFFFF00;
    pub const YellowGreen: ARGB = 0xFF9ACD32;
    pub const AlphaShift: ARGB = 24;
    pub const RedShift: ARGB = 16;
    pub const GreenShift: ARGB = 8;
    pub const BlueShift: ARGB = 0;
    pub const AlphaMask: ARGB = 0xff000000;
    pub const RedMask: ARGB = 0x00ff0000;
    pub const GreenMask: ARGB = 0x0000ff00;
    pub const BlueMask: ARGB = 0x000000ff;
    pub fn MakeARGB(a: BYTE, r: BYTE, g: BYTE, b: BYTE) -> ARGB {
        ((b as ARGB) << Self::BlueShift) |
          ((g as ARGB) << Self::GreenShift) |
          ((r as ARGB) << Self::RedShift) |
          ((a as ARGB) << Self::AlphaShift)
    }
}

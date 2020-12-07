// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::minwindef::DWORD;
pub const IME_CMODE_ALPHANUMERIC: DWORD = 0x0000;
pub const IME_CMODE_NATIVE: DWORD = 0x0001;
pub const IME_CMODE_CHINESE: DWORD = IME_CMODE_NATIVE;
pub const IME_CMODE_HANGUL: DWORD = IME_CMODE_NATIVE;
pub const IME_CMODE_JAPANESE: DWORD = IME_CMODE_NATIVE;
pub const IME_CMODE_KATAKANA: DWORD = 0x0002;
pub const IME_CMODE_LANGUAGE: DWORD = 0x0003;
pub const IME_CMODE_FULLSHAPE: DWORD = 0x0008;
pub const IME_CMODE_ROMAN: DWORD = 0x0010;
pub const IME_CMODE_CHARCODE: DWORD = 0x0020;
pub const IME_CMODE_HANJACONVERT: DWORD = 0x0040;
pub const IME_CMODE_NATIVESYMBOL: DWORD = 0x0080;

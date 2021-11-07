// Copyright © 2018 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Versioning constants for Windows SDK/DDK.
use shared::minwindef::WORD;
//
// _WIN32_WINNT version constants
//
pub const _WIN32_WINNT_NT4 : WORD = 0x0400;
pub const _WIN32_WINNT_WIN2K : WORD = 0x0500;
pub const _WIN32_WINNT_WINXP : WORD = 0x0501;
pub const _WIN32_WINNT_WS03 : WORD = 0x0502;
pub const _WIN32_WINNT_WIN6 : WORD = 0x0600;
pub const _WIN32_WINNT_VISTA : WORD = 0x0600;
pub const _WIN32_WINNT_WS08 : WORD = 0x0600;
pub const _WIN32_WINNT_LONGHORN : WORD = 0x0600;
pub const _WIN32_WINNT_WIN7 : WORD = 0x0601;
pub const _WIN32_WINNT_WIN8 : WORD = 0x0602;
pub const _WIN32_WINNT_WINBLUE : WORD = 0x0603;
pub const _WIN32_WINNT_WINTHRESHOLD : WORD = 0x0A00;
pub const _WIN32_WINNT_WIN10 : WORD = 0x0A00;

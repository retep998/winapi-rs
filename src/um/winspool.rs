// Copyright © 2015-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Winspool header file

use shared::minwindef::{DWORD, UINT};
use um::wingdi::{LPDEVMODEA, LPDEVMODEW};
use um::winnt::{ACCESS_MASK, LPSTR, LPWSTR};

STRUCT!{struct PRINTER_DEFAULTSA {
    pDataType: LPSTR,
    pDevMode: LPDEVMODEA,
    DesiredAccess: ACCESS_MASK,
}}
pub type PPRINTER_DEFAULTSA = *mut PRINTER_DEFAULTSA;
pub type LPPRINTER_DEFAULTSA = *mut PRINTER_DEFAULTSA;
STRUCT!{struct PRINTER_DEFAULTSW {
    pDataType: LPWSTR,
    pDevMode: LPDEVMODEW,
    DesiredAccess: ACCESS_MASK,
}}
pub type PPRINTER_DEFAULTSW = *mut PRINTER_DEFAULTSW;
pub type LPPRINTER_DEFAULTSW = *mut PRINTER_DEFAULTSW;
STRUCT!{struct PRINTER_OPTIONSA {
    cbSize: UINT,
    dwFlags: DWORD,
}}
pub type PPRINTER_OPTIONSA = *mut PRINTER_OPTIONSA;
pub type LPPRINTER_OPTIONSA = *mut PRINTER_OPTIONSA;
STRUCT!{struct PRINTER_OPTIONSW {
    cbSize: UINT,
    dwFlags: DWORD,
}}
pub type PPRINTER_OPTIONSW = *mut PRINTER_OPTIONSW;
pub type LPPRINTER_OPTIONSW = *mut PRINTER_OPTIONSW;

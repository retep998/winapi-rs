// Copyright © 2015-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! This interface definition contains typedefs for Windows Runtime data types.
use ctypes::{c_char};
use um::winnt::{PVOID};

DECLARE_HANDLE!(HSTRING, HSTRING__);
#[cfg(target_arch = "x86_64")]
STRUCT!{struct HSTRING_HEADER {
    Reserved:  [PVOID; 0], // For alignment
    Reserved2:  [c_char; 24],
}}
#[cfg(target_arch = "x86")]
STRUCT!{struct HSTRING_HEADER {
    Reserved:  [PVOID; 0], // For alignment
    Reserved2:  [c_char; 20],
}}
UNION!(HSTRING_HEADER, Reserved2, Reserved1, Reserved1_mut, PVOID);
DECLARE_HANDLE!(HSTRING_BUFFER, HSTRING_BUFFER__);

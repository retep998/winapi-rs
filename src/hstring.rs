// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! This interface definition contains typedefs for Windows Runtime data types.
DECLARE_HANDLE!(HSTRING, HSTRING__);
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HSTRING_HEADER {
    pub Reserved: [::PVOID; 0], // For alignment
    #[cfg(target_arch = "x86_64")]
    pub Reserved2: [::c_char; 24],
    #[cfg(target_arch = "i686")]
    pub Reserved2: [::c_char; 20],
}
UNION!(HSTRING_HEADER, Reserved2, Reserved1, Reserved1_mut, ::PVOID);
DECLARE_HANDLE!(HSTRING_BUFFER, HSTRING_BUFFER__);

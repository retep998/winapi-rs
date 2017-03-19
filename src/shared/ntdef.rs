// Copyright © 2016-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Type definitions for the basic types.
use ctypes::{
    __int64, __uint64, c_char, c_double, c_int, c_long, c_schar, c_short, c_uchar, c_ulong,
    c_ushort, c_void, wchar_t,
};
use shared::basetsd::KAFFINITY;
#[cfg(target_arch = "x86")]
IFDEF!{
pub const MAX_NATURAL_ALIGNMENT: usize = 4;
pub const MEMORY_ALLOCATION_ALIGNMENT: usize = 8;
}
#[cfg(target_arch = "x86_64")]
IFDEF!{
pub const MAX_NATURAL_ALIGNMENT: usize = 8;
pub const MEMORY_ALLOCATION_ALIGNMENT: usize = 16;
}
pub const SYSTEM_CACHE_ALIGNMENT_SIZE: usize = 64;
pub type PVOID = *mut c_void;
pub type PVOID64 = u64; // This is a 64-bit pointer, even when in 32-bit
pub type VOID = c_void;
pub type CHAR = c_char;
pub type SHORT = c_short;
pub type LONG = c_long;
pub type INT = c_int;
pub type WCHAR = wchar_t;
pub type PWCHAR = *mut WCHAR;
pub type LPWCH = *mut WCHAR;
pub type PWCH = *mut WCHAR;
pub type LPCWCH = *const WCHAR;
pub type PCWCH = *const WCHAR;
pub type NWPSTR = *mut WCHAR;
pub type LPWSTR = *mut WCHAR;
pub type LPTSTR = LPSTR;
pub type PWSTR = *mut WCHAR;
pub type PZPWSTR = *mut PWSTR;
pub type PCZPWSTR = *const PWSTR;
pub type LPUWSTR = *mut WCHAR; // Unaligned pointer
pub type PUWSTR = *mut WCHAR; // Unaligned pointer
pub type LPCWSTR = *const WCHAR;
pub type PCWSTR = *const WCHAR;
pub type PZPCWSTR = *mut PCWSTR;
pub type PCZPCWSTR = *const PCWSTR;
pub type LPCUWSTR = *const WCHAR; // Unaligned pointer
pub type PCUWSTR = *const WCHAR; // Unaligned pointer
pub type PZZWSTR = *mut WCHAR;
pub type PCZZWSTR = *const WCHAR;
pub type PUZZWSTR = *mut WCHAR; // Unaligned pointer
pub type PCUZZWSTR = *const WCHAR; // Unaligned pointer
pub type PNZWCH = *mut WCHAR;
pub type PCNZWCH = *const WCHAR;
pub type PUNZWCH = *mut WCHAR; // Unaligned pointer
pub type PCUNZWCH = *const WCHAR; // Unaligned pointer
pub type LPCWCHAR = *const WCHAR;
pub type PCWCHAR = *const WCHAR;
pub type LPCUWCHAR = *const WCHAR; // Unaligned pointer
pub type PCUWCHAR = *const WCHAR; // Unaligned pointer
pub type UCSCHAR = c_ulong;
pub const UCSCHAR_INVALID_CHARACTER: UCSCHAR = 0xffffffff;
pub const MIN_UCSCHAR: UCSCHAR = 0;
pub const MAX_UCSCHAR: UCSCHAR = 0x0010FFFF;
pub type PUCSCHAR = *mut UCSCHAR;
pub type PCUCSCHAR = *const UCSCHAR;
pub type PUCSSTR = *mut UCSCHAR;
pub type PUUCSSTR = *mut UCSCHAR; // Unaligned pointer
pub type PCUCSSTR = *const UCSCHAR;
pub type PCUUCSSTR = *const UCSCHAR; // Unaligned pointer
pub type PUUCSCHAR = *mut UCSCHAR; // Unaligned pointer
pub type PCUUCSCHAR = *const UCSCHAR; // Unaligned pointer
pub type PCHAR = *mut CHAR;
pub type LPCH = *mut CHAR;
pub type PCH = *mut CHAR;
pub type LPCCH = *const CHAR;
pub type PCCH = *const CHAR;
pub type NPSTR = *mut CHAR;
pub type LPSTR = *mut CHAR;
pub type PSTR = *mut CHAR;
pub type PZPSTR = *mut PSTR;
pub type PCZPSTR = *const PSTR;
pub type LPCSTR = *const CHAR;
pub type LPCTSTR = LPCSTR;
pub type PCSTR = *const CHAR;
pub type PZPCSTR = *mut PCSTR;
pub type PCZPCSTR = *const PCSTR;
pub type PZZSTR = *mut CHAR;
pub type PCZZSTR = *const CHAR;
pub type PNZCH = *mut CHAR;
pub type PCNZCH = *const CHAR;
// Skipping TCHAR things
pub type DOUBLE = c_double;
STRUCT!{struct QUAD {
    UseThisFieldToCopy: __int64,
}}
pub type PSHORT = *mut SHORT;
pub type PLONG = *mut LONG;
pub type PQUAD = *mut QUAD;
pub type UCHAR = c_uchar;
pub type USHORT = c_ushort;
pub type ULONG = c_ulong;
pub type UQUAD = QUAD;
pub type PUCHAR = *mut UCHAR;
pub type PUSHORT = *mut USHORT;
pub type PULONG = *mut ULONG;
pub type PUQUAD = *mut UQUAD;
pub type PCUCHAR = *const UCHAR;
pub type PCUSHORT = *const USHORT;
pub type PCULONG = *const ULONG;
pub type PCUQUAD = *const UQUAD;
pub type SCHAR = c_schar;
pub type PSCHAR = *mut SCHAR;
pub type PCSCHAR = *const SCHAR;
pub const ALL_PROCESSOR_GROUPS: USHORT = 0xffff;
STRUCT!{struct PROCESSOR_NUMBER {
    Group: USHORT,
    Number: UCHAR,
    Reserved: UCHAR,
}}
pub type PPROCESSOR_NUMBER = *mut PROCESSOR_NUMBER;
STRUCT!{struct GROUP_AFFINITY {
    Mask: KAFFINITY,
    Group: USHORT,
    Reserved: [USHORT; 3],
}}
pub type PGROUP_AFFINITY = *mut GROUP_AFFINITY;
#[cfg(target_arch = "x86")]
pub const MAXIMUM_PROC_PER_GROUP: UCHAR = 32;
#[cfg(target_arch = "x86_64")]
pub const MAXIMUM_PROC_PER_GROUP: UCHAR = 64;
pub const MAXIMUM_PROCESSORS: UCHAR = MAXIMUM_PROC_PER_GROUP;
pub type HANDLE = *mut c_void;
pub type PHANDLE = *mut HANDLE;
pub type FCHAR = UCHAR;
pub type FSHORT = USHORT;
pub type FLONG = ULONG;
pub type HRESULT = c_long;
pub const OBJ_HANDLE_TAGBITS: usize = 0x00000003;
pub type CCHAR = c_char;
pub type CSHORT = c_short;
pub type CLONG = ULONG;
pub type PCCHAR = *mut CCHAR;
pub type PCSHORT = *mut CSHORT;
pub type PCLONG = *mut CLONG;
pub type LCID = ULONG;
pub type PLCID = PULONG;
pub type LANGID = USHORT;
ENUM!{enum COMPARTMENT_ID {
    UNSPECIFIED_COMPARTMENT_ID = 0,
    DEFAULT_COMPARTMENT_ID,
}}
pub type PCOMPARTMENT_ID = *mut COMPARTMENT_ID;
pub type LOGICAL = ULONG;
pub type PLOGICAL = *mut ULONG;
pub type NTSTATUS = LONG;
pub type PNTSTATUS = *mut NTSTATUS;
pub type PCNTSTATUS = *const NTSTATUS;
// NT_SUCCESS and stuff
pub const APPLICATION_ERROR_MASK: HRESULT = 0x20000000;
pub const ERROR_SEVERITY_SUCCESS: HRESULT = 0x00000000;
pub const ERROR_SEVERITY_INFORMATIONAL: HRESULT = 0x40000000;
pub const ERROR_SEVERITY_WARNING: HRESULT = 0x80000000u32 as i32;
pub const ERROR_SEVERITY_ERROR: HRESULT = 0xC0000000u32 as i32;
// Weird TIME definitions
STRUCT!{struct FLOAT128 {
    LowPart: __int64,
    HighPart: __int64,
}}
pub type PFLOAT128 = *mut FLOAT128;
pub type LONGLONG = __int64;
pub type ULONGLONG = __uint64;
pub const MAXLONGLONG: LONGLONG = 0x7fffffffffffffff;
pub type PLONGLONG = *mut LONGLONG;
pub type PULONGLONG = *mut ULONGLONG;
pub type USN = LONGLONG;
pub type LARGE_INTEGER = LONGLONG;
pub type PLARGE_INTEGER = *mut LARGE_INTEGER;
pub type ULARGE_INTEGER = ULONGLONG;
pub type PULARGE_INTEGER = *mut ULARGE_INTEGER;
// TODO the rest

// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! Basic Windows Type Definitions for minwin partition
pub type ULONG = ::c_ulong;
pub type PULONG = *mut ULONG;
pub type USHORT = ::c_ushort;
pub type PUSHORT = *mut USHORT;
pub type UCHAR = ::c_uchar;
pub type PUCHAR = *mut UCHAR;
pub type PSZ = *mut ::c_char;
pub const MAX_PATH: usize = 260;
pub const FALSE: BOOL = 0;
pub const TRUE: BOOL = 1;
pub type DWORD = ::c_ulong;
pub type BOOL = ::c_int;
pub type BYTE = ::c_uchar;
pub type WORD = ::c_ushort;
pub type FLOAT = ::c_float;
pub type PFLOAT = *mut FLOAT;
pub type PBOOL = *mut BOOL;
pub type LPBOOL = *mut BOOL;
pub type PBYTE = *mut BYTE;
pub type LPBYTE = *mut BYTE;
pub type PINT = *mut ::c_int;
pub type LPINT = *mut ::c_int;
pub type PWORD = *mut WORD;
pub type LPWORD = *mut WORD;
pub type LPLONG = *mut ::c_long;
pub type PDWORD = *mut DWORD;
pub type LPDWORD = *mut DWORD;
pub type LPVOID = *mut ::c_void;
pub type LPCVOID = *const ::c_void;
pub type INT = ::c_int;
pub type UINT = ::c_uint;
pub type PUINT = *mut ::c_uint;
pub type WPARAM = ::UINT_PTR;
pub type LPARAM = ::LONG_PTR;
pub type LRESULT = ::LONG_PTR;
pub type SPHANDLE = *mut ::HANDLE;
pub type LPHANDLE = *mut ::HANDLE;
pub type HGLOBAL = ::HANDLE;
pub type HLOCAL = ::HANDLE;
pub type GLOBALHANDLE = ::HANDLE;
pub type LOCALHANDLE = ::HANDLE;
/// Pointer to probably a function with unknown type signature. Transmute as needed.
pub type FARPROC = *const ::c_void;
/// Pointer to probably a function with unknown type signature. Transmute as needed.
pub type NEARPROC = *const ::c_void;
/// Pointer to probably a function with unknown type signature. Transmute as needed.
pub type PROC = *const ::c_void;
pub type ATOM = WORD;
DECLARE_HANDLE!(HKEY, HKEY__);
pub type PHKEY = *mut HKEY;
DECLARE_HANDLE!(HMETAFILE, HMETAFILE__);
DECLARE_HANDLE!(HINSTANCE, HINSTANCE__);
pub type HMODULE = HINSTANCE;
DECLARE_HANDLE!(HRGN, HRGN__);
DECLARE_HANDLE!(HRSRC, HRSRC__);
DECLARE_HANDLE!(HSPRITE, HSPRITE__);
DECLARE_HANDLE!(HLSURF, HLSURF__);
DECLARE_HANDLE!(HSTR, HSTR__);
DECLARE_HANDLE!(HTASK, HTASK__);
DECLARE_HANDLE!(HWINSTA, HWINSTA__);
DECLARE_HANDLE!(HKL, HKL__);
pub type HFILE = ::c_int;
#[repr(C)] #[derive(Copy)] pub struct FILETIME {
    pub dwLowDateTime: DWORD,
    pub dwHighDateTime: DWORD,
}
pub type PFILETIME = *mut FILETIME;
pub type LPFILETIME = *mut FILETIME;

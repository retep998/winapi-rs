// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! Types and constants for WinAPI bindings.
#![no_std]
#![unstable]
#![feature(core, libc)]
#![allow(bad_style, raw_pointer_derive)]
#![warn(unused_qualifications, unused, unused_typecasts)]

//-------------------------------------------------------------------------------------------------
// External crates
//-------------------------------------------------------------------------------------------------
#[macro_use] extern crate core;
extern crate libc;
#[cfg(test)] extern crate std;
//-------------------------------------------------------------------------------------------------
// Imports
//-------------------------------------------------------------------------------------------------
pub use libc::{
    c_void,
    c_char,
    c_schar,
    c_uchar,
    c_short,
    c_ushort,
    c_int,
    c_uint,
    c_long,
    c_ulong,
    c_longlong,
    c_ulonglong,
    wchar_t,
    c_float,
    c_double,
};
pub use audioclient::*;
pub use fileapi::*;
pub use libloaderapi::*;
pub use minwinbase::*;
pub use minwindef::*;
pub use synchapi::*;
pub use wincon::*;
pub use wincrypt::*;
pub use windowsx::*;
pub use winerror::*;
pub use wingdi::*;
pub use winnt::*;
pub use winuser::*;
//-------------------------------------------------------------------------------------------------
// Macros
//-------------------------------------------------------------------------------------------------
macro_rules! DECLARE_HANDLE {
    ($name:ident, $inner:ident) => {
        #[repr(C)] #[allow(missing_copy_implementations)] struct $inner { unused: () }
        pub type $name = *mut $inner;
    };
}
macro_rules! MAKE_HRESULT {
    ($sev:expr, $fac:expr, $code:expr) => {
        ($sev << 31) | ($fac << 16) | $code
    }
}
macro_rules! MAKE_SCODE {
    ($sev:expr, $fac:expr, $code:expr) => {
        ($sev << 31) | ($fac << 16) | $code
    }
}

//-------------------------------------------------------------------------------------------------
// Modules
//-------------------------------------------------------------------------------------------------
pub mod audioclient;
pub mod fileapi;
pub mod libloaderapi;
pub mod minwinbase;
pub mod minwindef;
pub mod synchapi;
pub mod wincon;
pub mod wincrypt;
pub mod windowsx;
pub mod winerror;
pub mod wingdi;
pub mod winnt;
pub mod winuser;
// #[derive(Copy)] hack
#[cfg(not(test))]
mod std {
    pub mod marker {
        pub use core::marker::Copy;
    }
}
//-------------------------------------------------------------------------------------------------
// Primitive types not defined by libc
//-------------------------------------------------------------------------------------------------
pub type __int8 = i8;
pub type __uint8 = u8;
pub type __int16 = i16;
pub type __uint16 = u16;
pub type __int32 = i32;
pub type __uint32 = u32;
pub type __int64 = i64;
pub type __uint64 = u64;
//-------------------------------------------------------------------------------------------------
// basetsd.h
// Type definitions for the basic sized types.
//-------------------------------------------------------------------------------------------------
#[cfg(target_arch = "x86")]
pub type POINTER_64_INT = c_ulong;
#[cfg(target_arch = "x86_64")]
pub type POINTER_64_INT = __uint64;
pub type INT8 = c_schar;
pub type PINT8 = *mut c_schar;
pub type INT16 = c_short;
pub type PINT16 = *mut c_short;
pub type INT32 = c_int;
pub type PINT32 = *mut c_int;
pub type INT64 = __int64;
pub type PINT64 = *mut __int64;
pub type UINT8 = c_uchar;
pub type PUINT8 = *mut c_uchar;
pub type UINT16 = c_ushort;
pub type PUINT16 = *mut c_ushort;
pub type UINT32 = c_uint;
pub type PUINT32 = *mut c_uint;
pub type UINT64 = __uint64;
pub type PUINT64 = *mut __uint64;
pub type LONG32 = c_int;
pub type PLONG32 = *mut c_int;
pub type ULONG32 = c_uint;
pub type PULONG32 = *mut c_uint;
pub type DWORD32 = c_uint;
pub type PDWORD32 = *mut c_uint;
#[cfg(target_arch = "x86")]
pub type INT_PTR = c_int;
#[cfg(target_arch = "x86_64")]
pub type INT_PTR = __int64;
#[cfg(target_arch = "x86")]
pub type PINT_PTR = *mut c_int;
#[cfg(target_arch = "x86_64")]
pub type PINT_PTR = *mut __int64;
#[cfg(target_arch = "x86")]
pub type UINT_PTR = c_uint;
#[cfg(target_arch = "x86_64")]
pub type UINT_PTR = __uint64;
#[cfg(target_arch = "x86")]
pub type PUINT_PTR = *mut c_uint;
#[cfg(target_arch = "x86_64")]
pub type PUINT_PTR = *mut __uint64;
#[cfg(target_arch = "x86")]
pub type LONG_PTR = c_long;
#[cfg(target_arch = "x86_64")]
pub type LONG_PTR = __int64;
#[cfg(target_arch = "x86")]
pub type PLONG_PTR = *mut c_long;
#[cfg(target_arch = "x86_64")]
pub type PLONG_PTR = *mut __int64;
#[cfg(target_arch = "x86")]
pub type ULONG_PTR = c_ulong;
#[cfg(target_arch = "x86_64")]
pub type ULONG_PTR = __uint64;
#[cfg(target_arch = "x86")]
pub type PULONG_PTR = *mut c_ulong;
#[cfg(target_arch = "x86_64")]
pub type PULONG_PTR = *mut __uint64;
#[cfg(target_arch = "x86_64")]
pub type SHANDLE_PTR = __int64;
#[cfg(target_arch = "x86_64")]
pub type HANDLE_PTR = __uint64;
#[cfg(target_arch = "x86_64")]
pub type UHALF_PTR = c_uint;
#[cfg(target_arch = "x86_64")]
pub type PUHALF_PTR = *mut c_uint;
#[cfg(target_arch = "x86_64")]
pub type HALF_PTR = c_int;
#[cfg(target_arch = "x86_64")]
pub type PHALF_PTR = *mut c_int;
#[cfg(target_arch = "x86")]
pub type SHANDLE_PTR = c_long;
#[cfg(target_arch = "x86")]
pub type HANDLE_PTR = c_ulong;
#[cfg(target_arch = "x86")]
pub type UHALF_PTR = c_ushort;
#[cfg(target_arch = "x86")]
pub type PUHALF_PTR = *mut c_ushort;
#[cfg(target_arch = "x86")]
pub type HALF_PTR = c_short;
#[cfg(target_arch = "x86")]
pub type PHALF_PTR = *mut c_short;
pub type SIZE_T = ULONG_PTR;
pub type PSIZE_T = *mut ULONG_PTR;
pub type SSIZE_T = LONG_PTR;
pub type PSSIZE_T = *mut LONG_PTR;
pub type DWORD_PTR = ULONG_PTR;
pub type PDWORD_PTR = *mut ULONG_PTR;
pub type LONG64 = __int64;
pub type PLONG64 = *mut __int64;
pub type ULONG64 = __uint64;
pub type PULONG64 = *mut __uint64;
pub type DWORD64 = __uint64;
pub type PDWORD64 = *mut __uint64;
pub type KAFFINITY = ULONG_PTR;
pub type PKAFFINITY = *mut KAFFINITY;

//lazy
pub type SCODE = LONG;

//-------------------------------------------------------------------------------------------------
// windef.h
// Basic Windows Type Definitions
//-------------------------------------------------------------------------------------------------
#[repr(C)]
#[derive(Copy)]
pub struct POINTL {
    pub x: LONG,
    pub y: LONG,
}
pub type PPOINTL = *mut POINTL;

//-------------------------------------------------------------------------------------------------
// winnt.h
// This module defines the 32-Bit Windows types and constants that are defined by NT, but exposed
// through the Win32 API.
//-------------------------------------------------------------------------------------------------
pub type PVOID = *mut c_void;
pub type PVOID64 = u64; // This is a 64-bit pointer, even when in 32-bit
pub type WCHAR = wchar_t;
pub type PWCHAR = *mut WCHAR;
pub type LPWCH = *mut WCHAR;
pub type PWCH = *mut WCHAR;
pub type LPCWCH = *const WCHAR;
pub type PCWCH = *const WCHAR;
pub type NWPSTR = *mut WCHAR;
pub type LPWSTR = *mut WCHAR;
pub type PWSTR = *mut WCHAR;
pub type PZPWSTR = *mut PWSTR;
pub type PCZPWSTR = *const PWSTR;
pub type LPUWSTR = *mut WCHAR;
pub type PUWSTR = *mut WCHAR;
pub type LPCWSTR = *const WCHAR;
pub type PCWSTR = *const WCHAR;
pub type PZPCWSTR= *mut PCWSTR;
pub type PCZPCWSTR = *const PCWSTR;
pub type LPCUWSTR = *const WCHAR;
pub type PCUWSTR = *const WCHAR;
pub type PZZWSTR= *mut WCHAR;
pub type PCZZWSTR = *const WCHAR;
pub type PUZZWSTR = *mut WCHAR;
pub type PCUZZWSTR = *const WCHAR;
pub type PNZWCH = *mut WCHAR;
pub type PCNZWCH = *const WCHAR;
pub type PUNZWCH = *mut WCHAR;
pub type PCUNZWCH = *const WCHAR;
pub type LPCWCHAR = *const WCHAR;
pub type PCWCHAR = *const WCHAR;
pub type LPCUWCHAR = *const WCHAR;
pub type PCUWCHAR = *const WCHAR;
pub type UCSCHAR = c_ulong;
pub type PUCSCHAR = *mut UCSCHAR;
pub type PCUCSCHAR = *const UCSCHAR;
pub type PUCSSTR = *mut UCSCHAR;
pub type PUUCSSTR = *mut UCSCHAR;
pub type PCUCSSTR = *const UCSCHAR;
pub type PCUUCSSTR = *const UCSCHAR;
pub type PUUCSCHAR = *mut UCSCHAR;
pub type PCUUCSCHAR = *const UCSCHAR;
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
pub type PCSTR = *const CHAR;
pub type PZPCSTR = *mut PCSTR;
pub type PCZPCSTR = *const PCSTR;
pub type PZZSTR = *mut CHAR;
pub type PCZZSTR = *const CHAR;
pub type PNZCH = *mut CHAR;
pub type PCNZCH = *const CHAR;
// Skipping TCHAR things
pub type PSHORT = *mut SHORT;
pub type PLONG = *mut LONG;
pub type ACCESS_MASK = DWORD;
pub type PACCESS_MASK = *mut ACCESS_MASK;
#[repr(C)]
#[derive(Copy)]
pub struct PROCESSOR_NUMBER {
    pub Group: WORD,
    pub Number: BYTE,
    pub Reserved: BYTE,
}
pub type PPROCESSOR_NUMBER = *mut PROCESSOR_NUMBER;
#[repr(C)]
#[derive(Copy)]
pub struct GROUP_AFFINITY {
    pub Mask: KAFFINITY,
    pub Group: WORD,
    pub Reserved: [WORD; 3],
}
pub type PGROUP_AFFINITY = *mut GROUP_AFFINITY;
pub type HANDLE = *mut c_void;
pub type PHANDLE = *mut HANDLE;
pub type FCHAR = BYTE;
pub type FSHORT = WORD;
pub type FLONG = DWORD;
pub type HRESULT = c_long;
pub type CCHAR = c_char;
pub type LCID = DWORD;
pub type PLCID = PDWORD;
pub type LANGID = WORD;
#[repr(C)]
#[derive(Copy)]
pub enum COMPARTMENT_ID {
    UNSPECIFIED_COMPARTMENT_ID = 0,
    DEFAULT_COMPARTMENT_ID = 1,
}
pub type PCOMPARTMENT_ID = *mut COMPARTMENT_ID;
#[repr(C)]
#[derive(Copy)]
pub struct FLOAT128 {
    pub LowPart: __int64,
    pub HighPart: __int64,
}
pub type PFLOAT128 = *mut FLOAT128;
pub type LONGLONG = __int64;
pub type ULONGLONG = __uint64;
pub type PLONGLONG = *mut LONGLONG;
pub type PULONGLONG = *mut ULONGLONG;
pub type USN = LONGLONG;
pub type LARGE_INTEGER = LONGLONG; // Really this is a union
pub type PLARGE_INTEGER = LARGE_INTEGER;
pub type ULARGE_INTEGER = ULONGLONG;
pub type PULARGE_INTEGER= *mut ULARGE_INTEGER;
pub type RTL_REFERENCE_COUNT = LONG_PTR;
pub type PRTL_REFERENCE_COUNT = *mut LONG_PTR;
#[repr(C)]
#[derive(Copy)]
pub struct LUID {
    pub LowPart: DWORD,
    pub HighPart: LONG,
}
pub type PLUID = *mut LUID;
pub type DWORDLONG = ULONGLONG;
pub type PDWORDLONG = *mut DWORDLONG;
pub type BOOLEAN = BYTE;
pub type PBOOLEAN = *mut BOOLEAN;
#[repr(C)]
#[derive(Copy)]
pub struct LIST_ENTRY {
    pub Flink: *mut LIST_ENTRY,
    pub Blink: *mut LIST_ENTRY,
}
pub type PLIST_ENTRY = *mut LIST_ENTRY;
#[repr(C)]
#[derive(Copy)]
pub struct SINGLE_LIST_ENTRY {
    pub Next: *mut SINGLE_LIST_ENTRY,
}
pub type PSINGLE_LIST_ENTRY = *mut SINGLE_LIST_ENTRY;
#[repr(C)]
#[derive(Copy)]
pub struct LIST_ENTRY32 {
    pub Flink: DWORD,
    pub Blink: DWORD,
}
pub type PLIST_ENTRY32 = *mut LIST_ENTRY32;
#[repr(C)]
#[derive(Copy)]
pub struct LIST_ENTRY64 {
    pub Flink: ULONGLONG,
    pub Blink: ULONGLONG,
}
pub type PLIST_ENTRY64 = *mut LIST_ENTRY64;
#[repr(C)]
#[derive(Copy)]
pub struct OBJECTID {
    pub Lineage: GUID,
    pub Uniquifier: DWORD,
}
pub type EXCEPTION_ROUTINE = unsafe extern "system" fn(
    *mut _EXCEPTION_RECORD,
    PVOID,
    *mut _CONTEXT,
    PVOID,
) -> EXCEPTION_DISPOSITION;
pub type PEXCEPTION_ROUTINE = *mut EXCEPTION_ROUTINE;
pub type KSPIN_LOCK = ULONG_PTR;
pub type PKSPIN_LOCK = *mut KSPIN_LOCK;
#[repr(C)]
#[derive(Copy)]
pub struct M128A { // FIXME align 16
    pub Low: ULONGLONG,
    pub High: LONGLONG,
}
pub type PM128A = *mut M128A;
#[cfg(target_arch = "x86")]
#[repr(C)]
#[derive(Copy)]
pub struct XSAVE_FORMAT { // FIXME align 16
    pub ControlWord: WORD,
    pub StatusWord: WORD,
    pub TagWord: BYTE,
    pub Reserved1: BYTE,
    pub ErrorOpcode: WORD,
    pub ErrorOffset: DWORD,
    pub ErrorSelector: WORD,
    pub Reserved2: WORD,
    pub DataOffset: DWORD,
    pub DataSelector: WORD,
    pub Reserved3: WORD,
    pub MxCsr: DWORD,
    pub MxCsr_Mask: DWORD,
    pub FloatRegisters: [M128A; 8],
    pub XmmRegisters: [M128A; 8],
    pub Reserved4: [BYTE; 224],
}
#[cfg(target_arch = "x86_64")]
#[repr(C)]
#[derive(Copy)]
pub struct XSAVE_FORMAT { // FIXME align 16
    pub ControlWord: WORD,
    pub StatusWord: WORD,
    pub TagWord: BYTE,
    pub Reserved1: BYTE,
    pub ErrorOpcode: WORD,
    pub ErrorOffset: DWORD,
    pub ErrorSelector: WORD,
    pub Reserved2: WORD,
    pub DataOffset: DWORD,
    pub DataSelector: WORD,
    pub Reserved3: WORD,
    pub MxCsr: DWORD,
    pub MxCsr_Mask: DWORD,
    pub FloatRegisters: [M128A; 8],
    pub XmmRegisters: [M128A; 16],
    pub Reserved4: [BYTE; 96],
}
#[repr(C)]
#[derive(Copy)]
pub struct TOKEN_PRIVILEGES {
    PrivilegeCount: DWORD,
    Privileges: [LUID_AND_ATTRIBUTES; 0],
}
pub type PTOKEN_PRIVILEGES = *mut TOKEN_PRIVILEGES;
#[repr(C)]
#[derive(Copy)]
pub struct LUID_AND_ATTRIBUTES {
    Luid: LUID,
    Attributes: DWORD,
}
pub type PLUID_AND_ATTRIBUTES = *mut LUID_AND_ATTRIBUTES;
pub const PROCESS_TERMINATE: DWORD = 0x0001;
pub const PROCESS_CREATE_THREAD: DWORD = 0x0002;
pub const PROCESS_SET_SESSIONID: DWORD = 0x0004;
pub const PROCESS_VM_OPERATION: DWORD = 0x0008;
pub const PROCESS_VM_READ: DWORD = 0x0010;
pub const PROCESS_VM_WRITE: DWORD = 0x0020;
pub const PROCESS_DUP_HANDLE: DWORD = 0x0040;
pub const PROCESS_CREATE_PROCESS: DWORD = 0x0080;
pub const PROCESS_SET_QUOTA: DWORD = 0x0100;
pub const PROCESS_SET_INFORMATION: DWORD = 0x0200;
pub const PROCESS_QUERY_INFORMATION: DWORD = 0x0400;
pub const PROCESS_SUSPEND_RESUME: DWORD = 0x0800;
pub const PROCESS_QUERY_LIMITED_INFORMATION: DWORD = 0x1000;
pub const PROCESS_SET_LIMITED_INFORMATION: DWORD = 0x2000;
pub const PROCESS_ALL_ACCESS: DWORD = STANDARD_RIGHTS_REQUIRED | SYNCHRONIZE | 0xFFFF;

pub const TOKEN_ASSIGN_PRIMARY: DWORD = 0x0001;
pub const TOKEN_DUPLICATE: DWORD = 0x0002;
pub const TOKEN_IMPERSONATE: DWORD = 0x0004;
pub const TOKEN_QUERY: DWORD = 0x0008;
pub const TOKEN_QUERY_SOURCE: DWORD = 0x0010;
pub const TOKEN_ADJUST_PRIVILEGES: DWORD = 0x0020;
pub const TOKEN_ADJUST_GROUPS: DWORD = 0x0040;
pub const TOKEN_ADJUST_DEFAULT: DWORD = 0x0080;
pub const TOKEN_ADJUST_SESSIONID: DWORD = 0x0100;

pub const TOKEN_ALL_ACCESS_P: DWORD = STANDARD_RIGHTS_REQUIRED |
    TOKEN_ASSIGN_PRIMARY |
    TOKEN_DUPLICATE |
    TOKEN_IMPERSONATE |
    TOKEN_QUERY |
    TOKEN_QUERY_SOURCE |
    TOKEN_ADJUST_PRIVILEGES |
    TOKEN_ADJUST_GROUPS |
    TOKEN_ADJUST_DEFAULT;
pub const TOKEN_ALL_ACCESS: DWORD = TOKEN_ALL_ACCESS_P | TOKEN_ADJUST_SESSIONID;
pub const TOKEN_READ: DWORD = STANDARD_RIGHTS_READ | TOKEN_QUERY;
pub const TOKEN_WRITE: DWORD = STANDARD_RIGHTS_WRITE |
    TOKEN_ADJUST_PRIVILEGES |
    TOKEN_ADJUST_GROUPS |
    TOKEN_ADJUST_DEFAULT;
pub const TOKEN_EXECUTE: DWORD = STANDARD_RIGHTS_EXECUTE;

pub const KEY_QUERY_VALUE: REGSAM = 0x0001;
pub const KEY_SET_VALUE: REGSAM = 0x0002;
pub const KEY_CREATE_SUB_KEY: REGSAM = 0x0004;
pub const KEY_ENUMERATE_SUB_KEYS: REGSAM = 0x0008;
pub const KEY_NOTIFY: REGSAM = 0x0010;
pub const KEY_CREATE_LINK: REGSAM = 0x0020;
pub const KEY_WOW64_32KEY: REGSAM = 0x0200;
pub const KEY_WOW64_64KEY: REGSAM = 0x0100;
pub const KEY_WOW64_RES: REGSAM = 0x0300;

pub const KEY_READ: REGSAM = (
        STANDARD_RIGHTS_READ |
        KEY_QUERY_VALUE |
        KEY_ENUMERATE_SUB_KEYS |
        KEY_NOTIFY
    ) & (!SYNCHRONIZE);
pub const KEY_WRITE: REGSAM = (STANDARD_RIGHTS_WRITE | KEY_SET_VALUE | KEY_CREATE_SUB_KEY) & (!SYNCHRONIZE);
pub const KEY_EXECUTE: REGSAM = KEY_READ & (!SYNCHRONIZE);
pub const KEY_ALL_ACCESS: REGSAM = (
        STANDARD_RIGHTS_ALL |
        KEY_QUERY_VALUE |
        KEY_SET_VALUE |
        KEY_CREATE_SUB_KEY |
        KEY_ENUMERATE_SUB_KEYS |
        KEY_NOTIFY |
        KEY_CREATE_LINK
    ) & (!SYNCHRONIZE);

pub const REG_CREATED_NEW_KEY: DWORD = 0x00000001;
pub const REG_OPENED_EXISTING_KEY: DWORD = 0x00000002;

pub const REG_NOTIFY_CHANGE_NAME: DWORD = 0x00000001;
pub const REG_NOTIFY_CHANGE_ATTRIBUTES: DWORD = 0x00000002;
pub const REG_NOTIFY_CHANGE_LAST_SET: DWORD = 0x00000004;
pub const REG_NOTIFY_CHANGE_SECURITY: DWORD = 0x00000008;

pub const REG_LEGAL_CHANGE_FILTER: DWORD = REG_NOTIFY_CHANGE_NAME |
    REG_NOTIFY_CHANGE_ATTRIBUTES |
    REG_NOTIFY_CHANGE_LAST_SET |
    REG_NOTIFY_CHANGE_SECURITY;

pub const REG_NOTIFY_THREAD_AGNOSTIC: DWORD = 0x10000000; //supported only on Windows 8 and later

pub const REG_OPTION_RESERVED: DWORD = 0x00000000;
pub const REG_OPTION_NON_VOLATILE: DWORD = 0x00000000;
pub const REG_OPTION_VOLATILE: DWORD = 0x00000001;
pub const REG_OPTION_CREATE_LINK: DWORD = 0x00000002;
pub const REG_OPTION_BACKUP_RESTORE: DWORD = 0x00000004;
pub const REG_OPTION_OPEN_LINK: DWORD = 0x00000008;

pub const REG_NONE: DWORD = 0;
pub const REG_SZ: DWORD = 1;
pub const REG_EXPAND_SZ: DWORD = 2;
pub const REG_BINARY: DWORD = 3;
pub const REG_DWORD: DWORD = 4;
pub const REG_DWORD_LITTLE_ENDIAN: DWORD = 4;
pub const REG_DWORD_BIG_ENDIAN: DWORD = 5;
pub const REG_LINK: DWORD = 6;
pub const REG_MULTI_SZ: DWORD = 7;
pub const REG_RESOURCE_LIST: DWORD = 8;
pub const REG_FULL_RESOURCE_DESCRIPTOR: DWORD = 9;
pub const REG_RESOURCE_REQUIREMENTS_LIST: DWORD = 10;
pub const REG_QWORD: DWORD = 11;
pub const REG_QWORD_LITTLE_ENDIAN: DWORD = 11;

// guiddef.h
#[repr(C)]
#[derive(Copy)]
pub struct GUID {
    pub Data1: c_ulong,
    pub Data2: c_ushort,
    pub Data3: c_ushort,
    pub Data4: [c_uchar; 8],
}
pub type LPGUID = *mut GUID;
pub type LPCGUID = *const GUID;
pub type IID = GUID;
pub type LPIID = *mut IID;
pub type CLSID = GUID;
pub type LPCLSID = *mut CLSID;
pub type FMTID = GUID;
pub type LPFMTID = *mut FMTID;
pub type REFGUID = *const GUID;
pub type REFIID = *const IID;
pub type REFCLSID = *const IID;
pub type REFFMTID = *const IID;
// excpt.h
#[repr(C)]
#[derive(Copy)]
pub enum EXCEPTION_DISPOSITION {
    ExceptionContinueExecution = 0,
    ExceptionContinueSearch = 1,
    ExceptionNestedException = 2,
    ExceptionCollidedUnwind = 3,
}
#[repr(C)]
#[derive(Copy)]
pub struct _EXCEPTION_RECORD;
#[repr(C)]
#[derive(Copy)]
pub struct _CONTEXT;
#[cfg(target_arch = "x86_64")]
#[repr(C)]
#[derive(Copy)]
pub struct _DISPATCHER_CONTEXT;
// shlobj.h
pub type GPFIDL_FLAGS = c_int;
#[repr(i32)]
#[derive(Copy)]
pub enum KNOWN_FOLDER_FLAG {
    KF_FLAG_DEFAULT = 0x00000000,
    KF_FLAG_NO_APPCONTAINER_REDIRECTION = 0x00010000,
    KF_FLAG_CREATE = 0x00008000,
    KF_FLAG_DONT_VERIFY = 0x00004000,
    KF_FLAG_DONT_UNEXPAND = 0x00002000,
    KF_FLAG_NO_ALIAS = 0x00001000,
    KF_FLAG_INIT = 0x00000800,
    KF_FLAG_DEFAULT_PATH = 0x00000400,
    KF_FLAG_NOT_PARENT_RELATIVE = 0x00000200,
    KF_FLAG_SIMPLE_IDLIST = 0x00000100,
    KF_FLAG_ALIAS_ONLY = 0x80000000u32 as i32,
}
pub const IDO_SHGIOI_SHARE: c_int = 0x0FFFFFFF;
pub const IDO_SHGIOI_LINK: c_int = 0x0FFFFFFE;
// Yes, these values are supposed to overflow. Blame Microsoft.
pub const IDO_SHGIOI_SLOWFILE: c_int = 0xFFFFFFFDu32 as c_int;
pub const IDO_SHGIOI_DEFAULT: c_int = 0xFFFFFFFCu32 as c_int;
pub const GPFIDL_DEFAULT: GPFIDL_FLAGS = 0x0000;
pub const GPFIDL_ALTNAME: GPFIDL_FLAGS = 0x0001;
pub const GPFIDL_UNCPRINTER: GPFIDL_FLAGS = 0x0002;
pub const OFASI_EDIT: DWORD = 0x0001;
pub const OFASI_OPENDESKTOP: DWORD = 0x0002;
// winddef.h
#[repr(C)]
#[derive(Copy)]
pub struct HWND__;
pub type HWND = *mut HWND__;
#[repr(C)]
#[derive(Copy)]
pub struct HHOOK__;
pub type HHOOK = *mut HHOOK__;
pub type HGDIOBJ = *mut c_void;
#[repr(C)]
#[derive(Copy)]
pub struct HACCEL__;
pub type HACCEL = *mut HACCEL__;
#[repr(C)]
#[derive(Copy)]
pub struct HBITMAP__;
pub type HBITMAP = *mut HBITMAP__;
#[repr(C)]
#[derive(Copy)]
pub struct HBRUSH__;
pub type HBRUSH = *mut HBRUSH__;
#[repr(C)]
#[derive(Copy)]
pub struct HCOLORSPACE__;
pub type HCOLORSPACE = *mut HCOLORSPACE__;
#[repr(C)]
#[derive(Copy)]
pub struct HDC__;
pub type HDC = *mut HDC__;
#[repr(C)]
#[derive(Copy)]
pub struct HGLRC__;
pub type HGLRC = *mut HGLRC__;
#[repr(C)]
#[derive(Copy)]
pub struct HDESK__;
pub type HDESK = *mut HDESK__;
#[repr(C)]
#[derive(Copy)]
pub struct HENHMETAFILE__;
pub type HENHMETAFILE = *mut HENHMETAFILE__;
#[repr(C)]
#[derive(Copy)]
pub struct HFONT__;
pub type HFONT = *mut HFONT__;
#[repr(C)]
#[derive(Copy)]
pub struct HICON__;
pub type HICON = *mut HICON__;
#[repr(C)]
#[derive(Copy)]
pub struct HMENU__;
pub type HMENU = *mut HMENU__;
#[repr(C)]
#[derive(Copy)]
pub struct HPALETTE__;
pub type HPALETTE = *mut HPALETTE__;
#[repr(C)]
#[derive(Copy)]
pub struct HPEN__;
pub type HPEN = *mut HPEN__;
#[repr(C)]
#[derive(Copy)]
pub struct HWINEVENTHOOK__;
pub type HWINEVENTHOOK = *mut HWINEVENTHOOK__;
#[repr(C)]
#[derive(Copy)]
pub struct HMONITOR__;
pub type HMONITOR = *mut HMONITOR__;
#[repr(C)]
#[derive(Copy)]
pub struct HUMPD__;
pub type HUMPD = *mut HUMPD__;
pub type HCURSOR = HICON;
pub type COLORREF = DWORD;
pub type LPCOLORREF = *mut DWORD;
#[repr(C)]
#[derive(Copy)]
pub struct RECT {
    pub left: LONG,
    pub top: LONG,
    pub right: LONG,
    pub bottom: LONG,
}
pub type PRECT = *mut RECT;
pub type NPRECT = *mut RECT;
pub type LPRECT = *mut RECT;
pub type LPCRECT = *const RECT;
#[repr(C)]
#[derive(Copy)]
pub struct POINT {
    pub x: LONG,
    pub y: LONG,
}
pub type PPOINT = *mut POINT;
pub type NPPOINT = *mut POINT;
pub type LPPOINT = *mut POINT;

//-------------------------------------------------------------------------------------------------
// winbase.h
// This module defines the 32-Bit Windows Base APIs
//-------------------------------------------------------------------------------------------------
pub const FORMAT_MESSAGE_IGNORE_INSERTS: DWORD = 0x00000200;
pub const FORMAT_MESSAGE_FROM_STRING: DWORD = 0x00000400;
pub const FORMAT_MESSAGE_FROM_HMODULE: DWORD = 0x00000800;
pub const FORMAT_MESSAGE_FROM_SYSTEM: DWORD = 0x00001000;
pub const FORMAT_MESSAGE_ARGUMENT_ARRAY: DWORD = 0x00002000;
pub const FORMAT_MESSAGE_MAX_WIDTH_MASK: DWORD = 0x000000FF;
pub const FORMAT_MESSAGE_ALLOCATE_BUFFER: DWORD = 0x00000100;

// shlobjdl.h
#[repr(C)]
#[derive(Copy)]
pub struct IContextMenu;
#[repr(C)]
#[derive(Copy)]
pub struct IContextMenu2;
#[repr(C)]
#[derive(Copy)]
pub struct IContextMenu3;
#[repr(C)]
#[derive(Copy)]
pub struct IExecuteCommand;
#[repr(C)]
#[derive(Copy)]
pub struct IPersistFolder;
#[repr(C)]
#[derive(Copy)]
pub struct IRunnableTask;
#[repr(C)]
#[derive(Copy)]
pub struct IShellTaskScheduler;
#[repr(C)]
#[derive(Copy)]
pub struct IQueryCodePage;
#[repr(C)]
#[derive(Copy)]
pub struct IPersistFolder2;
#[repr(C)]
#[derive(Copy)]
pub struct IPersistFolder3;
#[repr(C)]
#[derive(Copy)]
pub struct IPersistIDList;
#[repr(C)]
#[derive(Copy)]
pub struct IEnumIDList;
#[repr(C)]
#[derive(Copy)]
pub struct IEnumFullIDList;
#[repr(C)]
#[derive(Copy)]
pub struct IFileSyncMergeHandler;
#[repr(C)]
#[derive(Copy)]
pub struct IObjectWithFolderEnumMode;
#[repr(C)]
#[derive(Copy)]
pub struct IParseAndCreateItem;
#[repr(C)]
#[derive(Copy)]
pub struct IShellFolder;
#[repr(C)]
#[derive(Copy)]
pub struct IEnumExtraSearch;
#[repr(C)]
#[derive(Copy)]
pub struct IShellFolder2;
#[repr(C)]
#[derive(Copy)]
pub struct IFolderViewOptions;
#[repr(C)]
#[derive(Copy)]
pub struct IShellView;
#[repr(C)]
#[derive(Copy)]
pub struct IShellView2;
#[repr(C)]
#[derive(Copy)]
pub struct IShellView3;
#[repr(C)]
#[derive(Copy)]
pub struct IFolderView;
#[repr(C)]
#[derive(Copy)]
pub struct ISearchBoxInfo;
#[repr(C)]
#[derive(Copy)]
pub struct IFolderView2;
#[repr(C)]
#[derive(Copy)]
pub struct IFolderViewSettings;
#[repr(C)]
#[derive(Copy)]
pub struct IPreviewHandlerVisuals;
#[repr(C)]
#[derive(Copy)]
pub struct IVisualProperties;
#[repr(C)]
#[derive(Copy)]
pub struct ICommDlgBrowser;
#[repr(C)]
#[derive(Copy)]
pub struct ICommDlgBrowser2;
#[repr(C)]
#[derive(Copy)]
pub struct ICommDlgBrowser3;
#[repr(C)]
#[derive(Copy)]
pub struct IColumnManager;
#[repr(C)]
#[derive(Copy)]
pub struct IFolderFilterSite;
#[repr(C)]
#[derive(Copy)]
pub struct IFolderFilter;
#[repr(C)]
#[derive(Copy)]
pub struct IInputObjectSite;
#[repr(C)]
#[derive(Copy)]
pub struct IInputObject;
#[repr(C)]
#[derive(Copy)]
pub struct IInputObject2;
#[repr(C)]
#[derive(Copy)]
pub struct IShellIcon;
#[repr(C)]
#[derive(Copy)]
pub struct IShellBrowser;
#[repr(C)]
#[derive(Copy)]
pub struct IProfferService;
#[repr(C)]
#[derive(Copy)]
pub struct IShellItem;
#[repr(C)]
#[derive(Copy)]
pub struct IShellItem2;
#[repr(C)]
#[derive(Copy)]
pub struct IShellItemImageFactory;
#[repr(C)]
#[derive(Copy)]
pub struct IUserAccountChangeCallback;
#[repr(C)]
#[derive(Copy)]
pub struct IEnumShellItems;
#[repr(C)]
#[derive(Copy)]
pub struct ITransferAdviseSink;
#[repr(C)]
#[derive(Copy)]
pub struct ITransferSource;
#[repr(C)]
#[derive(Copy)]
pub struct IEnumResources;
#[repr(C)]
#[derive(Copy)]
pub struct IShellItemResources;
#[repr(C)]
#[derive(Copy)]
pub struct ITransferDestination;
#[repr(C)]
#[derive(Copy)]
pub struct IStreamAsync;
#[repr(C)]
#[derive(Copy)]
pub struct IStreamUnbufferedInfo;
#[repr(C)]
#[derive(Copy)]
pub struct IFileOperationProgressSink;
#[repr(C)]
#[derive(Copy)]
pub struct IShellItemArray;
#[repr(C)]
#[derive(Copy)]
pub struct IInitializeWithItem;
#[repr(C)]
#[derive(Copy)]
pub struct IObjectWithSelection;
#[repr(C)]
#[derive(Copy)]
pub struct IObjectWithBackReferences;
#[repr(C)]
#[derive(Copy)]
pub struct IPropertyUI;
#[repr(C)]
#[derive(Copy)]
pub struct ICategoryProvider;
#[repr(C)]
#[derive(Copy)]
pub struct ICategorizer;
#[repr(C)]
#[derive(Copy)]
pub struct IDropTargetHelper;
#[repr(C)]
#[derive(Copy)]
pub struct IDragSourceHelper;
#[repr(C)]
#[derive(Copy)]
pub struct IDragSourceHelper2;
#[repr(C)]
#[derive(Copy)]
pub struct IShellLinkA;
#[repr(C)]
#[derive(Copy)]
pub struct IShellLinkW;
#[repr(C)]
#[derive(Copy)]
pub struct IShellLinkDataList;
#[repr(C)]
#[derive(Copy)]
pub struct IResolveShellLink;
#[repr(C)]
#[derive(Copy)]
pub struct IActionProgressDialog;
#[repr(C)]
#[derive(Copy)]
pub struct IHWEventHandler;
#[repr(C)]
#[derive(Copy)]
pub struct IHWEventHandler2;
#[repr(C)]
#[derive(Copy)]
pub struct IQueryCancelAutoPlay;
#[repr(C)]
#[derive(Copy)]
pub struct IDynamicHWHandler;
#[repr(C)]
#[derive(Copy)]
pub struct IActionProgress;
#[repr(C)]
#[derive(Copy)]
pub struct IShellExtInit;
#[repr(C)]
#[derive(Copy)]
pub struct IShellPropSheetExt;
#[repr(C)]
#[derive(Copy)]
pub struct IRemoteComputer;
#[repr(C)]
#[derive(Copy)]
pub struct IQueryContinue;
#[repr(C)]
#[derive(Copy)]
pub struct IObjectWithCancelEvent;
#[repr(C)]
#[derive(Copy)]
pub struct IUserNotification;
#[repr(C)]
#[derive(Copy)]
pub struct IUserNotificationCallback;
#[repr(C)]
#[derive(Copy)]
pub struct IUserNotification2;
#[repr(C)]
#[derive(Copy)]
pub struct IItemNameLimits;
#[repr(C)]
#[derive(Copy)]
pub struct ISearchFolderItemFactory;
#[repr(C)]
#[derive(Copy)]
pub struct IExtractImage;
#[repr(C)]
#[derive(Copy)]
pub struct IExtractImage2;
#[repr(C)]
#[derive(Copy)]
pub struct IThumbnailHandlerFactory;
#[repr(C)]
#[derive(Copy)]
pub struct IParentAndItem;
#[repr(C)]
#[derive(Copy)]
pub struct IDockingWindow;
#[repr(C)]
#[derive(Copy)]
pub struct IDeskBand;
#[repr(C)]
#[derive(Copy)]
pub struct IDeskBandInfo;
#[repr(C)]
#[derive(Copy)]
pub struct IDeskBand2;
#[repr(C)]
#[derive(Copy)]
pub struct ITaskbarList;
#[repr(C)]
#[derive(Copy)]
pub struct ITaskbarList2;
#[repr(C)]
#[derive(Copy)]
pub struct ITaskbarList3;
#[repr(C)]
#[derive(Copy)]
pub struct ITaskbarList4;
#[repr(C)]
#[derive(Copy)]
pub struct IStartMenuPinnedList;
#[repr(C)]
#[derive(Copy)]
pub struct ICDBurn;
#[repr(C)]
#[derive(Copy)]
pub struct IWizardSite;
#[repr(C)]
#[derive(Copy)]
pub struct IWizardExtension;
#[repr(C)]
#[derive(Copy)]
pub struct IWebWizardExtension;
#[repr(C)]
#[derive(Copy)]
pub struct IPublishingWizard;
#[repr(C)]
#[derive(Copy)]
pub struct IFolderViewHost;
#[repr(C)]
#[derive(Copy)]
pub struct IExplorerBrowserEvents;
#[repr(C)]
#[derive(Copy)]
pub struct IExplorerBrowser;
#[repr(C)]
#[derive(Copy)]
pub struct IAccessibleObject;
#[repr(C)]
#[derive(Copy)]
pub struct IResultsFolder;
#[repr(C)]
#[derive(Copy)]
pub struct IEnumObjects;
#[repr(C)]
#[derive(Copy)]
pub struct IOperationsProgressDialog;
#[repr(C)]
#[derive(Copy)]
pub struct IIOCancelInformation;
#[repr(C)]
#[derive(Copy)]
pub struct IFileOperation;
#[repr(C)]
#[derive(Copy)]
pub struct IObjectProvider;
#[repr(C)]
#[derive(Copy)]
pub struct INamespaceWalkCB;
#[repr(C)]
#[derive(Copy)]
pub struct INamespaceWalkCB2;
#[repr(C)]
#[derive(Copy)]
pub struct INamespaceWalk;
#[repr(C)]
#[derive(Copy)]
pub struct IAutoCompleteDropDown;
#[repr(C)]
#[derive(Copy)]
pub struct IBandSite;
#[repr(C)]
#[derive(Copy)]
pub struct IModalWindow;
#[repr(C)]
#[derive(Copy)]
pub struct ICDBurnExt;
#[repr(C)]
#[derive(Copy)]
pub struct IContextMenuSite;
#[repr(C)]
#[derive(Copy)]
pub struct IEnumReadyCallback;
#[repr(C)]
#[derive(Copy)]
pub struct IEnumerableView;
#[repr(C)]
#[derive(Copy)]
pub struct IInsertItem;
#[repr(C)]
#[derive(Copy)]
pub struct IMenuBand;
#[repr(C)]
#[derive(Copy)]
pub struct IFolderBandPriv;
#[repr(C)]
#[derive(Copy)]
pub struct IRegTreeItem;
#[repr(C)]
#[derive(Copy)]
pub struct IImageRecompress;
#[repr(C)]
#[derive(Copy)]
pub struct IDeskBar;
#[repr(C)]
#[derive(Copy)]
pub struct IMenuPopup;
#[repr(C)]
#[derive(Copy)]
pub struct IFileIsInUse;
#[repr(C)]
#[derive(Copy)]
pub struct IFileDialogEvents;
#[repr(C)]
#[derive(Copy)]
pub struct IFileDialog;
#[repr(C)]
#[derive(Copy)]
pub struct IFileSaveDialog;
#[repr(C)]
#[derive(Copy)]
pub struct IFileOpenDialog;
#[repr(C)]
#[derive(Copy)]
pub struct IFileDialogCustomize;
#[repr(C)]
#[derive(Copy)]
pub struct IFileDialogControlEvents;
#[repr(C)]
#[derive(Copy)]
pub struct IFileDialog2;
#[repr(C)]
#[derive(Copy)]
pub struct IApplicationAssociationRegistration;
#[repr(C)]
#[derive(Copy)]
pub struct IApplicationAssociationRegistrationUI;
#[repr(C)]
#[derive(Copy)]
pub struct IDelegateFolder;
#[repr(C)]
#[derive(Copy)]
pub struct IBrowserFrameOptions;
#[repr(C)]
#[derive(Copy)]
pub struct INewWindowManager;
#[repr(C)]
#[derive(Copy)]
pub struct IAttachmentExecute;
#[repr(C)]
#[derive(Copy)]
pub struct IShellMenuCallback;
#[repr(C)]
#[derive(Copy)]
pub struct IShellMenu;
#[repr(C)]
#[derive(Copy)]
pub struct IShellRunDll;
#[repr(C)]
#[derive(Copy)]
pub struct IKnownFolder;
#[repr(C)]
#[derive(Copy)]
pub struct IKnownFolderManager;
#[repr(C)]
#[derive(Copy)]
pub struct ISharingConfigurationManager;
#[repr(C)]
#[derive(Copy)]
pub struct IPreviousVersionsInfo;
#[repr(C)]
#[derive(Copy)]
pub struct IRelatedItem;
#[repr(C)]
#[derive(Copy)]
pub struct IIdentityName;
#[repr(C)]
#[derive(Copy)]
pub struct IDelegateItem;
#[repr(C)]
#[derive(Copy)]
pub struct ICurrentItem;
#[repr(C)]
#[derive(Copy)]
pub struct ITransferMediumItem;
#[repr(C)]
#[derive(Copy)]
pub struct IUseToBrowseItem;
#[repr(C)]
#[derive(Copy)]
pub struct IDisplayItem;
#[repr(C)]
#[derive(Copy)]
pub struct IViewStateIdentityItem;
#[repr(C)]
#[derive(Copy)]
pub struct IPreviewItem;
#[repr(C)]
#[derive(Copy)]
pub struct IDestinationStreamFactory;
#[repr(C)]
#[derive(Copy)]
pub struct INewMenuClient;
#[repr(C)]
#[derive(Copy)]
pub struct IInitializeWithBindCtx;
#[repr(C)]
#[derive(Copy)]
pub struct IShellItemFilter;
#[repr(C)]
#[derive(Copy)]
pub struct INameSpaceTreeControl;
#[repr(C)]
#[derive(Copy)]
pub struct INameSpaceTreeControl2;
#[repr(C)]
#[derive(Copy)]
pub struct INameSpaceTreeControlEvents;
#[repr(C)]
#[derive(Copy)]
pub struct INameSpaceTreeControlDropHandler;
#[repr(C)]
#[derive(Copy)]
pub struct INameSpaceTreeAccessible;
#[repr(C)]
#[derive(Copy)]
pub struct INameSpaceTreeControlCustomDraw;
#[repr(C)]
#[derive(Copy)]
pub struct INameSpaceTreeControlFolderCapabilities;
#[repr(C)]
#[derive(Copy)]
pub struct IPreviewHandler;
#[repr(C)]
#[derive(Copy)]
pub struct IPreviewHandlerFrame;
#[repr(C)]
#[derive(Copy)]
pub struct ITrayDeskBand;
#[repr(C)]
#[derive(Copy)]
pub struct IBandHost;
#[repr(C)]
#[derive(Copy)]
pub struct IExplorerPaneVisibility;
#[repr(C)]
#[derive(Copy)]
pub struct IContextMenuCB;
#[repr(C)]
#[derive(Copy)]
pub struct IDefaultExtractIconInit;
#[repr(C)]
#[derive(Copy)]
pub struct IExplorerCommand;
#[repr(C)]
#[derive(Copy)]
pub struct IExplorerCommandState;
#[repr(C)]
#[derive(Copy)]
pub struct IInitializeCommand;
#[repr(C)]
#[derive(Copy)]
pub struct IEnumExplorerCommand;
#[repr(C)]
#[derive(Copy)]
pub struct IExplorerCommandProvider;
#[repr(C)]
#[derive(Copy)]
pub struct IMarkupCallback;
#[repr(C)]
#[derive(Copy)]
pub struct IControlMarkup;
#[repr(C)]
#[derive(Copy)]
pub struct IInitializeNetworkFolder;
#[repr(C)]
#[derive(Copy)]
pub struct IOpenControlPanel;
#[repr(C)]
#[derive(Copy)]
pub struct IComputerInfoChangeNotify;
#[repr(C)]
#[derive(Copy)]
pub struct IFileSystemBindData;
#[repr(C)]
#[derive(Copy)]
pub struct IFileSystemBindData2;
#[repr(C)]
#[derive(Copy)]
pub struct ICustomDestinationList;
#[repr(C)]
#[derive(Copy)]
pub struct IApplicationDestinations;
#[repr(C)]
#[derive(Copy)]
pub struct IApplicationDocumentLists;
#[repr(C)]
#[derive(Copy)]
pub struct IObjectWithAppUserModelID;
#[repr(C)]
#[derive(Copy)]
pub struct IObjectWithProgID;
#[repr(C)]
#[derive(Copy)]
pub struct IUpdateIDList;
#[repr(C)]
#[derive(Copy)]
pub struct IDesktopGadget;
#[repr(C)]
#[derive(Copy)]
pub struct IDesktopWallpaper;
#[repr(C)]
#[derive(Copy)]
pub struct IHomeGroup;
#[repr(C)]
#[derive(Copy)]
pub struct IInitializeWithPropertyStore;
#[repr(C)]
#[derive(Copy)]
pub struct IOpenSearchSource;
#[repr(C)]
#[derive(Copy)]
pub struct IShellLibrary;
#[repr(C)]
#[derive(Copy)]
pub struct IDefaultFolderMenuInitialize;
#[repr(C)]
#[derive(Copy)]
pub struct IApplicationActivationManager;
#[repr(C)]
#[derive(Copy)]
pub struct IAssocHandlerInvoker;
#[repr(C)]
#[derive(Copy)]
pub struct IAssocHandler;
#[repr(C)]
#[derive(Copy)]
pub struct IEnumAssocHandlers;
#[repr(C)]
#[derive(Copy)]
pub struct IDataObjectProvider;
#[repr(C)]
#[derive(Copy)]
pub struct IDataTransferManagerInterop;
#[repr(C)]
#[derive(Copy)]
pub struct IFrameworkInputPaneHandler;
#[repr(C)]
#[derive(Copy)]
pub struct IFrameworkInputPane;
#[repr(C)]
#[derive(Copy)]
pub struct IAccessibilityDockingServiceCallback;
#[repr(C)]
#[derive(Copy)]
pub struct IAccessibilityDockingService;
#[repr(C)]
#[derive(Copy)]
pub struct IAppVisibilityEvents;
#[repr(C)]
#[derive(Copy)]
pub struct IAppVisibility;
#[repr(C)]
#[derive(Copy)]
pub struct IPackageExecutionStateChangeNotification;
#[repr(C)]
#[derive(Copy)]
pub struct IPackageDebugSettings;
#[repr(C)]
#[derive(Copy)]
pub struct ISuspensionDependencyManager;
#[repr(C)]
#[derive(Copy)]
pub struct IExecuteCommandApplicationHostEnvironment;
#[repr(C)]
#[derive(Copy)]
pub struct IExecuteCommandHost;
#[repr(C)]
#[derive(Copy)]
pub struct IApplicationDesignModeSettings;
#[repr(C)]
#[derive(Copy)]
pub struct IApplicationDesignModeSettings2;
#[repr(C)]
#[derive(Copy)]
pub struct ILaunchTargetMonitor;
#[repr(C)]
#[derive(Copy)]
pub struct ILaunchSourceViewSizePreference;
#[repr(C)]
#[derive(Copy)]
pub struct ILaunchTargetViewSizePreference;
#[repr(C)]
#[derive(Copy)]
pub struct ILaunchSourceAppUserModelId;
#[repr(C)]
#[derive(Copy)]
pub struct IInitializeWithWindow;
#[repr(C)]
#[derive(Copy)]
pub struct IHandlerInfo;
#[repr(C)]
#[derive(Copy)]
pub struct IHandlerActivationHost;
#[repr(C)]
#[derive(Copy)]
pub struct IContactManagerInterop;
// shtypes.h
#[repr(C)]
#[derive(Copy)]
pub struct SHITEMID {
    pub cb: USHORT,
    pub abID: [BYTE; 1],
}
pub type LPSHITEMID = *mut SHITEMID;
pub type LPCSHITEMID = *const SHITEMID;
#[repr(C)]
#[derive(Copy)]
pub struct ITEMIDLIST {
    pub mkid: SHITEMID,
}
pub type ITEMIDLIST_RELATIVE = ITEMIDLIST;
pub type ITEMID_CHILD = ITEMIDLIST;
pub type ITEMIDLIST_ABSOLUTE = ITEMIDLIST;
pub type LPITEMIDLIST = *mut ITEMIDLIST;
pub type LPCITEMIDLIST = *const ITEMIDLIST;
pub type PIDLIST_ABSOLUTE = *mut ITEMIDLIST_ABSOLUTE;
pub type PCIDLIST_ABSOLUTE = *const ITEMIDLIST_ABSOLUTE;
pub type PCUIDLIST_ABSOLUTE = *const ITEMIDLIST_ABSOLUTE;
pub type PIDLIST_RELATIVE = *mut ITEMIDLIST_RELATIVE;
pub type PCIDLIST_RELATIVE = *const ITEMIDLIST_RELATIVE;
pub type PUIDLIST_RELATIVE = *mut ITEMIDLIST_RELATIVE;
pub type PCUIDLIST_RELATIVE = *const ITEMIDLIST_RELATIVE;
pub type PITEMID_CHILD = *mut ITEMID_CHILD;
pub type PCITEMID_CHILD = *const ITEMID_CHILD;
pub type PUITEMID_CHILD = *mut ITEMID_CHILD;
pub type PCUITEMID_CHILD = *const ITEMID_CHILD;
pub type PCUITEMID_CHILD_ARRAY = *const PCUITEMID_CHILD;
pub type PCUIDLIST_RELATIVE_ARRAY = *const PCUIDLIST_RELATIVE;
pub type PCIDLIST_ABSOLUTE_ARRAY = *const PCIDLIST_ABSOLUTE;
pub type PCUIDLIST_ABSOLUTE_ARRAY = *const PCUIDLIST_ABSOLUTE;
pub type KNOWNFOLDERID = GUID;
pub type REFKNOWNFOLDERID = *const KNOWNFOLDERID;
// wtypesbase.h
pub type OLECHAR = WCHAR;
pub type LPOLESTR = *mut OLECHAR;

//-------------------------------------------------------------------------------------------------
// wtypes.h
//-------------------------------------------------------------------------------------------------
pub const CLSCTX_INPROC_SERVER: DWORD = 0x1;
pub const CLSCTX_INPROC_HANDLER: DWORD = 0x2;
pub const CLSCTX_LOCAL_SERVER: DWORD = 0x4;
pub const CLSCTX_REMOTE_SERVER: DWORD = 0x10;
pub const CLSCTX_SERVER: DWORD = CLSCTX_INPROC_SERVER | CLSCTX_LOCAL_SERVER |
                                 CLSCTX_REMOTE_SERVER;
pub const CLSCTX_ALL: DWORD = CLSCTX_INPROC_HANDLER | CLSCTX_SERVER;

pub type VARTYPE = c_ushort;

//-------------------------------------------------------------------------------------------------
// mmdeviceapi.h
//-------------------------------------------------------------------------------------------------
#[repr(C)]
#[derive(Copy)]
pub enum EDataFlow {
    eRender,
    eCapture,
    eAll,
    EDataFlow_enum_count,
}

#[repr(C)]
#[derive(Copy)]
pub enum ERole {
    eConsole,
    eMultimedia,
    eCommunications,
    ERole_enum_count,
}

pub const CLSID_MMDeviceEnumerator: CLSID = GUID {
    Data1: 0xBCDE0395,
    Data2: 0xE52F,
    Data3: 0x467C,
    Data4: [0x8E, 0x3D, 0xC4, 0x57, 0x92, 0x91, 0x69, 0x2E],
};

pub const IID_IMMDeviceEnumerator: IID = GUID {
    Data1: 0xA95664D2,
    Data2: 0x9614,
    Data3: 0x4F35,
    Data4: [0xA7, 0x46, 0xDE, 0x8D, 0xB6, 0x36, 0x17, 0xE6],
};

#[repr(C)]
#[derive(Copy)]
pub struct IMMDeviceVtbl {
    pub QueryInterface: unsafe extern "system" fn(
        This: *mut IMMDevice,
        riid: REFIID,
        ppvObject: *mut *mut c_void,
    ) -> HRESULT,
    pub AddRef: unsafe extern "system" fn(
        This: *mut IMMDevice,
    ) -> ULONG,
    pub Release: unsafe extern "system" fn(
        This: *mut IMMDevice,
    ) -> ULONG,
    pub Activate: unsafe extern "system" fn(
        This: *mut IMMDevice,
        iid: REFIID,
        dwClsCtx: DWORD,
        pActivationParams: *mut PROPVARIANT,
        ppInterface: *mut LPVOID,
    ) -> HRESULT,
    pub OpenPropertyStore: unsafe extern "system" fn(
        This: *mut IMMDevice,
        stgmAccess: DWORD,
        ppProperties: *mut *mut IPropertyStore,
    ) -> HRESULT,
    pub GetId: unsafe extern "system" fn(
        This: *mut IMMDevice,
        ppstrId: *mut LPWSTR,
    ) -> HRESULT,
    pub GetState: unsafe extern "system" fn(
        This: *mut IMMDevice,
        pdwState: *mut DWORD,
    ) -> HRESULT,
}
#[repr(C)]
#[derive(Copy)]
pub struct IMMDevice {
    pub lpVtbl: *const IMMDeviceVtbl,
}
#[repr(C)]
#[derive(Copy)]
pub struct IMMDeviceEnumeratorVtbl {
    pub QueryInterface: unsafe extern "system" fn(
        This: *mut IMMDeviceEnumerator,
        riid: REFIID,
        ppvObject: *mut *mut c_void,
    ) -> HRESULT,
    pub AddRef: unsafe extern "system" fn(
        This: *mut IMMDeviceEnumerator,
    ) -> ULONG,
    pub Release: unsafe extern "system" fn(
        This: *mut IMMDeviceEnumerator,
    ) -> ULONG,
    pub EnumAudioEndpoints: unsafe extern "system" fn(
        This: *mut IMMDeviceEnumerator,
        dataFlow: EDataFlow,
        dwStateMask: DWORD,
        ppDevices: *mut *mut IMMDeviceCollection,
    ) -> HRESULT,
    pub GetDefaultAudioEndpoint: unsafe extern "system" fn(
        This: *mut IMMDeviceEnumerator,
        dataFlow: EDataFlow,
        role: ERole,
        ppEndpoint: *mut *mut IMMDevice,
    ) -> HRESULT,
    pub GetDevice: unsafe extern "system" fn(
        This: *mut IMMDeviceEnumerator,
        pwstrId: LPCWSTR,
        ppDevices: *mut *mut IMMDevice,
    ) -> HRESULT,
    pub RegisterEndpointNotificationCallback: unsafe extern "system" fn(
        This: *mut IMMDeviceEnumerator,
        pClient: *mut IMMNotificationClient,
    ) -> HRESULT,
    pub UnregisterEndpointNotificationCallback: unsafe extern "system" fn(
        This: *mut IMMDeviceEnumerator,
        pClient: *mut IMMNotificationClient,
    ) -> HRESULT,
}
#[repr(C)]
#[derive(Copy)]
pub struct IMMDeviceEnumerator {
    pub lpVtbl: *const IMMDeviceEnumeratorVtbl,
}
#[repr(C)]
#[derive(Copy)]
pub struct IMMDeviceCollection;
#[repr(C)]
#[derive(Copy)]
pub struct IMMNotificationClient;

//-------------------------------------------------------------------------------------------------
// audioclient.h
//-------------------------------------------------------------------------------------------------
pub const IID_IAudioClient: IID = GUID {
    Data1: 0x1CB9AD4C,
    Data2: 0xDBFA,
    Data3: 0x4c32,
    Data4: [0xB1, 0x78, 0xC2, 0xF5, 0x68, 0xA7, 0x03, 0xB2],
};

pub const IID_IAudioRenderClient: IID = GUID {
    Data1: 0xF294ACFC,
    Data2: 0x3146,
    Data3: 0x4483,
    Data4: [0xA7, 0xBF, 0xAD, 0xDC, 0xA7, 0xC2, 0x60, 0xE2],
};

#[repr(C)]
#[derive(Copy)]
pub struct IAudioClientVtbl {
    pub QueryInterface: unsafe extern "system" fn(
        This: *mut IAudioClient,
        riid: REFIID,
        ppvObject: *mut *mut c_void,
    ) -> HRESULT,
    pub AddRef: unsafe extern "system" fn(
        This: *mut IAudioClient,
    ) -> ULONG,
    pub Release: unsafe extern "system" fn(
        This: *mut IAudioClient,
    ) -> ULONG,
    pub Initialize: unsafe extern "system" fn(
        This: *mut IAudioClient,
        ShareMode: AUDCLNT_SHAREMODE,
        StreamFlags: DWORD,
        hnsBufferDuration: REFERENCE_TIME,
        hnsPeriodicity: REFERENCE_TIME,
        pFormat: *const WAVEFORMATEX,
        AudioSessionGuid: LPCGUID,
    ) -> HRESULT,
    pub GetBufferSize: unsafe extern "system" fn(
        This: *mut IAudioClient,
        pNumBufferFrames: *mut UINT32,
    ) -> HRESULT,
    pub GetStreamLatency: unsafe extern "system" fn(
        This: *mut IAudioClient,
        phnsLatency: *mut REFERENCE_TIME,
    ) -> HRESULT,
    pub GetCurrentPadding: unsafe extern "system" fn(
        This: *mut IAudioClient,
        pNumPaddingFrames: *mut UINT32,
    ) -> HRESULT,
    pub IsFormatSupported: unsafe extern "system" fn(
        This: *mut IAudioClient,
        ShareMode: AUDCLNT_SHAREMODE,
        pFormat: *const WAVEFORMATEX,
        ppClosestMatch: *mut *mut WAVEFORMATEX,
    ) -> HRESULT,
    pub GetMixFormat: unsafe extern "system" fn(
        This: *mut IAudioClient,
        ppDeviceFormat: *mut *mut WAVEFORMATEX,
    ) -> HRESULT,
    pub GetDevicePeriod: unsafe extern "system" fn(
        This: *mut IAudioClient,
        phnsDefaultDevicePeriod: *mut REFERENCE_TIME,
        phnsMinimumDevicePeriod: *mut REFERENCE_TIME,
    ) -> HRESULT,
    pub Start: unsafe extern "system" fn(
        This: *mut IAudioClient,
    ) -> HRESULT,
    pub Stop: unsafe extern "system" fn(
        This: *mut IAudioClient,
    ) -> HRESULT,
    pub Reset: unsafe extern "system" fn(
        This: *mut IAudioClient,
    ) -> HRESULT,
    pub SetEventHandle: unsafe extern "system" fn(
        This: *mut IAudioClient,
        eventHandle: HANDLE,
    ) -> HRESULT,
    pub GetService: unsafe extern "system" fn(
        This: *mut IAudioClient,
        riid: REFIID,
        ppv: *mut LPVOID,
    ) -> HRESULT,
}
#[repr(C)]
#[derive(Copy)]
pub struct IAudioClient {
    pub lpVtbl: *const IAudioClientVtbl,
}
#[repr(C)]
#[derive(Copy)]
pub struct IAudioRenderClientVtbl {
    pub QueryInterface: unsafe extern "system" fn(
        This: *mut IAudioRenderClient,
        riid: REFIID,
        ppvObject: *mut *mut c_void,
    ) -> HRESULT,
    pub AddRef: unsafe extern "system" fn(
        This: *mut IAudioRenderClient,
    ) -> ULONG,
    pub Release: unsafe extern "system" fn(
        This: *mut IAudioRenderClient,
    ) -> ULONG,
    pub GetBuffer: unsafe extern "system" fn(
        This: *mut IAudioRenderClient,
        NumFramesRequested: UINT32,
        ppData: *mut *mut BYTE,
    ) -> HRESULT,
    pub ReleaseBuffer: unsafe extern "system" fn(
        This: *mut IAudioRenderClient,
        NumFramesWritten: UINT32,
        dwFlags: DWORD,
    ) -> HRESULT,
}
#[repr(C)]
#[derive(Copy)]
pub struct IAudioRenderClient {
    pub lpVtbl: *const IAudioRenderClientVtbl,
}

//-------------------------------------------------------------------------------------------------
// audiosessiontypes.h
//-------------------------------------------------------------------------------------------------
#[repr(C)]
#[derive(Copy)]
pub enum AUDCLNT_SHAREMODE {
    AUDCLNT_SHAREMODE_SHARED,
    AUDCLNT_SHAREMODE_EXCLUSIVE
}

//-------------------------------------------------------------------------------------------------
// strmif.h
//-------------------------------------------------------------------------------------------------
pub type REFERENCE_TIME = LONGLONG;

//-------------------------------------------------------------------------------------------------
// mmreg.h
//-------------------------------------------------------------------------------------------------
#[repr(C)]
#[derive(Copy)]
pub struct WAVEFORMATEX {
    pub wFormatTag: WORD,
    pub nChannels: WORD,
    pub nSamplesPerSec: DWORD,
    pub nAvgBytesPerSec: DWORD,
    pub nBlockAlign: WORD,
    pub wBitsPerSample: WORD,
    pub cbSize: WORD,
}

//-------------------------------------------------------------------------------------------------
// propsys.h
//-------------------------------------------------------------------------------------------------
#[repr(C)]
#[derive(Copy)]
pub struct IPropertyStore;

//-------------------------------------------------------------------------------------------------
// propidl.h
//-------------------------------------------------------------------------------------------------
#[repr(C)]
#[derive(Copy)]
pub struct PROPVARIANT {
    vt: VARTYPE,
    wReserved1: WORD,
    wReserved2: WORD,
    wReserved3: WORD,
    data: [u8;  16],
}

//-------------------------------------------------------------------------------------------------
// objldlbase.h
// this ALWAYS GENERATED file contains the definitions for the interfaces
//-------------------------------------------------------------------------------------------------
#[repr(C)]
#[derive(Copy)]
pub struct IMallocVtbl {
    pub QueryInterface: unsafe extern "system" fn(
        This: *mut IMalloc,
        riid: REFIID,
        ppvObject: *mut *mut c_void,
    ) -> HRESULT,
    pub AddRef: unsafe extern "system" fn(
        This: *mut IMalloc,
    ) -> ULONG,
    pub Release: unsafe extern "system" fn(
        This: *mut IMalloc,
    ) -> ULONG,
    pub Alloc: unsafe extern "system" fn(
        This: *mut IMalloc,
        cb: SIZE_T,
    ) -> *mut c_void,
    pub Realloc: unsafe extern "system" fn(
        This: *mut IMalloc,
        pv: *mut c_void,
        cb: SIZE_T,
    ) -> *mut c_void,
    pub Free: unsafe extern "system" fn(
        This: *mut IMalloc,
        pv: *mut c_void,
    ),
    pub GetSize: unsafe extern "system" fn(
        This: *mut IMalloc,
        pv: *mut c_void,
    ) -> SIZE_T,
    pub DidAlloc: unsafe extern "system" fn(
        This: *mut IMalloc,
        pv: *mut c_void,
    ) -> c_int,
    pub HeapMinimize: unsafe extern "system" fn(
        This: *mut IMalloc,
    ),
}
#[repr(C)]
#[derive(Copy)]
pub struct IMalloc {
    pub lpVtbl: *const IMallocVtbl,
}
pub type LPMALLOC = *mut IMalloc;
#[repr(C)]
#[derive(Copy)]
pub struct STATSTG {
    pwcsName: LPOLESTR,
    type_: DWORD,
    cbSize: ULARGE_INTEGER,
    mtime: FILETIME,
    ctime: FILETIME,
    atime: FILETIME,
    grfMode: DWORD,
    grfLocksSupported: DWORD,
    clsid: CLSID,
    grfStateBits: DWORD,
    reserved: DWORD,
}
#[repr(C)]
#[derive(Copy)]
pub struct IStreamVtbl {
    pub QueryInterface: unsafe extern "system" fn(
        This: *mut IStream,
        riid: REFIID,
        ppvObject: *mut *mut c_void,
    ) -> HRESULT,
    pub AddRef: unsafe extern "system" fn(
        This: *mut IStream,
    ) -> ULONG,
    pub Release: unsafe extern "system" fn(
        This: *mut IStream,
    ) -> ULONG,
    pub Read: unsafe extern "system" fn(
        This: *mut IStream,
        pv: *mut c_void,
        cb: ULONG,
        pcbRead: *mut ULONG,
    ) -> HRESULT,
    pub Write: unsafe extern "system" fn(
        This: *mut IStream,
        pv: *const c_void,
        cb: ULONG,
        pcbWritten: *mut ULONG,
    ) -> HRESULT,
    pub Seek: unsafe extern "system" fn(
        This: *mut IStream,
        dlibMove: LARGE_INTEGER,
        dwOrigin: DWORD,
        plibNewPosition: *mut ULARGE_INTEGER,
    ) -> HRESULT,
    pub SetSize: unsafe extern "system" fn(
        This: *mut IStream,
        libNewSize: ULARGE_INTEGER,
    ) -> HRESULT,
    pub CopyTo: unsafe extern "system" fn(
        This: *mut IStream,
        cb: ULARGE_INTEGER,
        pcbRead: *mut ULARGE_INTEGER,
        pcbWritten: *mut ULARGE_INTEGER,
    ) -> HRESULT,
    pub Commit: unsafe extern "system" fn(
        This: *mut IStream,
        grfCommitFlags: DWORD,
    ) -> HRESULT,
    pub Revert: unsafe extern "system" fn(
        This: *mut IStream,
    ) -> HRESULT,
    pub LockRegion: unsafe extern "system" fn(
        This: *mut IStream,
        libOffset: ULARGE_INTEGER,
        cb: ULARGE_INTEGER,
        dwLockType: DWORD,
    ) -> HRESULT,
    pub UnlockRegion: unsafe extern "system" fn(
        This: *mut IStream,
        libOffset: ULARGE_INTEGER,
        cb: ULARGE_INTEGER,
        dwLockType: DWORD,
    ) -> HRESULT,
    pub Stat: unsafe extern "system" fn(
        This: *mut IStream,
        pstatstg: *mut STATSTG,
        grfStatFlag: DWORD,
    ) -> HRESULT,
    pub Clone: unsafe extern "system" fn(
        This: *mut IStream,
        ppstm: *mut *mut IStream,
    ) -> HRESULT,
}
#[repr(C)]
#[derive(Copy)]
pub struct IStream {
    pub lpVtbl: *const IStreamVtbl,
}
pub type LPSTREAM = *mut IStream;
#[repr(C)]
#[derive(Copy)]
pub enum APTTYPEQUALIFIER {
    APTTYPEQUALIFIER_NONE = 0,
    APTTYPEQUALIFIER_IMPLICIT_MTA = 1,
    APTTYPEQUALIFIER_NA_ON_MTA = 2,
    APTTYPEQUALIFIER_NA_ON_STA = 3,
    APTTYPEQUALIFIER_NA_ON_IMPLICIT_MTA = 4,
    APTTYPEQUALIFIER_NA_ON_MAINSTA = 5,
    APTTYPEQUALIFIER_APPLICATION_STA= 6,
}
#[repr(C)]
#[derive(Copy)]
pub enum APTTYPE {
    APTTYPE_CURRENT = -1,
    APTTYPE_STA = 0,
    APTTYPE_MTA = 1,
    APTTYPE_NA = 2,
    APTTYPE_MAINSTA = 3,
}
//-------------------------------------------------------------------------------------------------
// combaseapi.h
// Base Component Object Model defintions.
//-------------------------------------------------------------------------------------------------
#[repr(C)]
#[derive(Copy)]
pub struct ServerInformation {
    dwServerPid: DWORD,
    dwServerTid: DWORD,
    ui64ServerAddress: UINT64,
}
pub type PServerInformation = *mut ServerInformation;
#[repr(C)]
#[derive(Copy)]
pub struct CO_MTA_USAGE_COOKIE__;
pub type CO_MTA_USAGE_COOKIE = *mut CO_MTA_USAGE_COOKIE__;

//-------------------------------------------------------------------------------------------------
// unknwnbase.h
// this ALWAYS GENERATED file contains the definitions for the interfaces
//-------------------------------------------------------------------------------------------------
#[repr(C)]
#[derive(Copy)]
pub struct IUnknownVtbl {
    pub QueryInterface: unsafe extern "system" fn(
        This: *mut IUnknown,
        riid: REFIID,
        ppvObject: *mut *mut c_void,
    ) -> HRESULT,
    pub AddRef: unsafe extern "system" fn(
        This: *mut IUnknown,
    ) -> ULONG,
    pub Release: unsafe extern "system" fn(
        This: *mut IUnknown,
    ) -> ULONG,
}
#[repr(C)]
#[derive(Copy)]
pub struct IUnknown {
    pub lpVtbl: *const IUnknownVtbl,
}
pub type LPUNKNOWN = *mut IUnknown;

//-------------------------------------------------------------------------------------------------
// playsoundapi.h
// ApiSet Contract for api-ms-win-mm-playsound-l1-1-0
//-------------------------------------------------------------------------------------------------
pub const SND_SYNC: DWORD = 0x0000;
pub const SND_ASYNC: DWORD = 0x0001;
pub const SND_NODEFAULT: DWORD = 0x0002;
pub const SND_MEMORY: DWORD = 0x0004;
pub const SND_LOOP: DWORD = 0x0008;
pub const SND_NOSTOP: DWORD = 0x0010;
pub const SND_NOWAIT: DWORD = 0x00002000;
pub const SND_ALIAS: DWORD = 0x00010000;
pub const SND_ALIAS_ID: DWORD = 0x00110000;
pub const SND_FILENAME: DWORD = 0x00020000;
pub const SND_RESOURCE: DWORD = 0x00040004;
pub const SND_PURGE: DWORD = 0x0040;
pub const SND_APPLICATION: DWORD = 0x0080;
pub const SND_SENTRY: DWORD = 0x00080000;
pub const SND_RING: DWORD = 0x00100000;
pub const SND_SYSTEM: DWORD = 0x00200000;

#[repr(C)]
#[derive(Copy)]
pub struct PROCESS_MEMORY_COUNTERS {
    pub cb: DWORD,
    pub PageFaultCount: DWORD,
    pub PeakWorkingSetSize: SIZE_T,
    pub WorkingSetSize: SIZE_T,
    pub QuotaPeakPagedPoolUsage: SIZE_T,
    pub QuotaPagedPoolUsage: SIZE_T,
    pub QuotaPeakNonPagedPoolUsage: SIZE_T,
    pub QuotaNonPagedPoolUsage: SIZE_T,
    pub PagefileUsage: SIZE_T,
    pub PeakPagefileUsage: SIZE_T,
}
pub type PPROCESS_MEMORY_COUNTERS = *mut PROCESS_MEMORY_COUNTERS;
#[repr(C)]
#[derive(Copy)]
pub struct PROCESS_MEMORY_COUNTERS_EX {
    pub cb: DWORD,
    pub PageFaultCount: DWORD,
    pub PeakWorkingSetSize: SIZE_T,
    pub WorkingSetSize: SIZE_T,
    pub QuotaPeakPagedPoolUsage: SIZE_T,
    pub QuotaPagedPoolUsage: SIZE_T,
    pub QuotaPeakNonPagedPoolUsage: SIZE_T,
    pub QuotaNonPagedPoolUsage: SIZE_T,
    pub PagefileUsage: SIZE_T,
    pub PeakPagefileUsage: SIZE_T,
    pub PrivateUsage: SIZE_T,
}

//-------------------------------------------------------------------------------------------------
// winreg.h
// Registry API procedure declarations, constant definitions and macros
//-------------------------------------------------------------------------------------------------

pub type REGSAM = ACCESS_MASK;

#[repr(C)]
#[derive(Copy)]
pub struct VALENTA {
    pub ve_valuename: LPSTR,
    pub ve_valuelen: DWORD,
    pub ve_valueptr: DWORD_PTR,
    pub ve_type: DWORD,
}
pub type PVALENTA = *mut VALENTA;

#[repr(C)]
#[derive(Copy)]
pub struct VALENTW {
    pub ve_valuename: LPWSTR,
    pub ve_valuelen: DWORD,
    pub ve_valueptr: DWORD_PTR,
    pub ve_type: DWORD,
}
pub type PVALENTW = *mut VALENTW;

pub const HKEY_CLASSES_ROOT: HKEY = 0x80000000 as HKEY;
pub const HKEY_CURRENT_USER: HKEY = 0x80000001 as HKEY;
pub const HKEY_LOCAL_MACHINE: HKEY = 0x80000002 as HKEY;
pub const HKEY_USERS: HKEY = 0x80000003 as HKEY;
pub const HKEY_PERFORMANCE_DATA: HKEY = 0x80000004 as HKEY;
pub const HKEY_PERFORMANCE_TEXT: HKEY = 0x80000050 as HKEY;
pub const HKEY_PERFORMANCE_NLSTEXT: HKEY = 0x80000060 as HKEY;
pub const HKEY_CURRENT_CONFIG: HKEY = 0x80000005 as HKEY;
pub const HKEY_DYN_DATA: HKEY = 0x80000006 as HKEY;
pub const HKEY_CURRENT_USER_LOCAL_SETTINGS: HKEY = 0x80000007 as HKEY;

pub const REG_MUI_STRING_TRUNCATE: DWORD = 0x00000001;

pub const RRF_RT_REG_NONE: DWORD = 0x00000001;
pub const RRF_RT_REG_SZ: DWORD = 0x00000002;
pub const RRF_RT_REG_EXPAND_SZ: DWORD = 0x00000004;
pub const RRF_RT_REG_BINARY: DWORD = 0x00000008;
pub const RRF_RT_REG_DWORD: DWORD = 0x00000010;
pub const RRF_RT_REG_MULTI_SZ: DWORD = 0x00000020;
pub const RRF_RT_REG_QWORD: DWORD = 0x00000040;

pub const RRF_RT_DWORD: DWORD = RRF_RT_REG_BINARY|RRF_RT_REG_DWORD;
pub const RRF_RT_QWORD: DWORD = RRF_RT_REG_BINARY|RRF_RT_REG_QWORD;
pub const RRF_RT_ANY: DWORD = 0x0000ffff;

pub const RRF_NOEXPAND: DWORD = 0x10000000;
pub const RRF_ZEROONFAILURE: DWORD = 0x20000000;

//-------------------------------------------------------------------------------------------------
// winuser.h
// USER procedure declarations, constant definitions and macros
//-------------------------------------------------------------------------------------------------
pub const BN_CLICKED: WORD = 0;
pub const BN_PAINT: WORD = 1;
pub const BN_HILITE: WORD = 2;
pub const BN_UNHILITE: WORD = 3;
pub const BN_DISABLE: WORD = 4;
pub const BN_DOUBLECLICKED: WORD = 5;
pub const BN_PUSHED: WORD = BN_HILITE;
pub const BN_UNPUSHED: WORD = BN_UNHILITE;
pub const BN_DBLCLK: WORD = BN_DOUBLECLICKED;
pub const BN_SETFOCUS: WORD = 6;
pub const BN_KILLFOCUS: WORD = 7;

pub const BS_PUSHBUTTON: DWORD = 0x00000000;
pub const BS_DEFPUSHBUTTON: DWORD = 0x00000001;
pub const BS_CHECKBOX: DWORD = 0x00000002;
pub const BS_AUTOCHECKBOX: DWORD = 0x00000003;
pub const BS_RADIOBUTTON: DWORD = 0x00000004;
pub const BS_3STATE: DWORD = 0x00000005;
pub const BS_AUTO3STATE: DWORD = 0x00000006;
pub const BS_GROUPBOX: DWORD = 0x00000007;
pub const BS_USERBUTTON: DWORD = 0x00000008;
pub const BS_AUTORADIOBUTTON: DWORD = 0x00000009;
pub const BS_PUSHBOX: DWORD = 0x0000000A;
pub const BS_OWNERDRAW: DWORD = 0x0000000B;
pub const BS_TYPEMASK: DWORD = 0x0000000F;
pub const BS_LEFTTEXT: DWORD = 0x00000020;
pub const BS_TEXT: DWORD = 0x00000000;
pub const BS_ICON: DWORD = 0x00000040;
pub const BS_BITMAP: DWORD = 0x00000080;
pub const BS_LEFT: DWORD = 0x00000100;
pub const BS_RIGHT: DWORD = 0x00000200;
pub const BS_CENTER: DWORD = 0x00000300;
pub const BS_TOP: DWORD = 0x00000400;
pub const BS_BOTTOM: DWORD = 0x00000800;
pub const BS_VCENTER: DWORD = 0x00000C00;
pub const BS_PUSHLIKE: DWORD = 0x00001000;
pub const BS_MULTILINE: DWORD = 0x00002000;
pub const BS_NOTIFY: DWORD = 0x00004000;
pub const BS_FLAT: DWORD = 0x00008000;
pub const BS_RIGHTBUTTON: DWORD = BS_LEFTTEXT;

pub const CDS_UPDATEREGISTRY: DWORD = 0x00000001;
pub const CDS_TEST: DWORD = 0x00000002;
pub const CDS_FULLSCREEN: DWORD = 0x00000004;
pub const CDS_GLOBAL: DWORD = 0x00000008;
pub const CDS_SET_PRIMARY: DWORD = 0x00000010;
pub const CDS_VIDEOPARAMETERS: DWORD = 0x00000020;
pub const CDS_ENABLE_UNSAFE_MODES: DWORD = 0x00000100;
pub const CDS_DISABLE_UNSAFE_MODES: DWORD = 0x00000200;
pub const CDS_RESET: DWORD = 0x40000000;
pub const CDS_RESET_EX: DWORD = 0x20000000;
pub const CDS_NORESET: DWORD = 0x10000000;

pub const CS_VREDRAW: DWORD = 0x0001;
pub const CS_HREDRAW: DWORD = 0x0002;
pub const CS_DBLCLKS: DWORD = 0x0008;
pub const CS_OWNDC: DWORD = 0x0020;
pub const CS_CLASSDC: DWORD = 0x0040;
pub const CS_PARENTDC: DWORD = 0x0080;
pub const CS_NOCLOSE: DWORD = 0x0200;
pub const CS_SAVEBITS: DWORD = 0x0800;
pub const CS_BYTEALIGNCLIENT: DWORD = 0x1000;
pub const CS_BYTEALIGNWINDOW: DWORD = 0x2000;
pub const CS_GLOBALCLASS: DWORD = 0x4000;
pub const CS_IME: DWORD = 0x00010000;
pub const CS_DROPSHADOW: DWORD = 0x00020000;

pub const CW_USEDEFAULT: c_int = 0x80000000u32 as c_int;

pub const DISP_CHANGE_SUCCESSFUL: LONG = 0;
pub const DISP_CHANGE_RESTART: LONG = 1;
pub const DISP_CHANGE_FAILED: LONG = -1;
pub const DISP_CHANGE_BADMODE: LONG = -2;
pub const DISP_CHANGE_NOTUPDATED: LONG = -3;
pub const DISP_CHANGE_BADFLAGS: LONG = -4;
pub const DISP_CHANGE_BADPARAM: LONG = -5;
pub const DISP_CHANGE_BADDUALVIEW: LONG = -6;

pub const EDD_GET_DEVICE_INTERFACE_NAME: DWORD = 0x00000001;

pub const ENUM_CURRENT_SETTINGS: DWORD = -1;
pub const ENUM_REGISTRY_SETTINGS: DWORD = -2;

pub const MB_OK: DWORD = 0x00000000;
pub const MB_OKCANCEL: DWORD = 0x00000001;
pub const MB_ABORTRETRYIGNORE: DWORD = 0x00000002;
pub const MB_YESNOCANCEL: DWORD = 0x00000003;
pub const MB_YESNO: DWORD = 0x00000004;
pub const MB_RETRYCANCEL: DWORD = 0x00000005;
pub const MB_CANCELTRYCONTINUE: DWORD = 0x00000006;
pub const MB_ICONHAND: DWORD = 0x00000010;
pub const MB_ICONQUESTION: DWORD = 0x00000020;
pub const MB_ICONEXCLAMATION: DWORD = 0x00000030;
pub const MB_ICONASTERISK: DWORD = 0x00000040;
pub const MB_USERICON: DWORD = 0x00000080;
pub const MB_ICONWARNING: DWORD = MB_ICONEXCLAMATION;
pub const MB_ICONERROR: DWORD = MB_ICONHAND;
pub const MB_ICONINFORMATION: DWORD = MB_ICONASTERISK;
pub const MB_ICONSTOP: DWORD = MB_ICONHAND;
pub const MB_DEFBUTTON1: DWORD = 0x00000000;
pub const MB_DEFBUTTON2: DWORD = 0x00000100;
pub const MB_DEFBUTTON3: DWORD = 0x00000200;
pub const MB_DEFBUTTON4: DWORD = 0x00000300;
pub const MB_APPLMODAL: DWORD = 0x00000000;
pub const MB_SYSTEMMODAL: DWORD = 0x00001000;
pub const MB_TASKMODAL: DWORD = 0x00002000;
pub const MB_HELP: DWORD = 0x00004000;
pub const MB_NOFOCUS: DWORD = 0x00008000;
pub const MB_SETFOREGROUND: DWORD = 0x00010000;
pub const MB_DEFAULT_DESKTOP_ONLY: DWORD = 0x00020000;
pub const MB_TOPMOST: DWORD = 0x00040000;
pub const MB_RIGHT: DWORD = 0x00080000;
pub const MB_RTLREADING: DWORD = 0x00100000;
pub const MB_SERVICE_NOTIFICATION: DWORD = 0x00200000;
pub const MB_SERVICE_NOTIFICATION_NT3X: DWORD = 0x00040000;
pub const MB_TYPEMASK: DWORD = 0x0000000F;
pub const MB_ICONMASK: DWORD = 0x000000F0;
pub const MB_DEFMASK: DWORD = 0x00000F00;
pub const MB_MODEMASK: DWORD = 0x00003000;
pub const MB_MISCMASK: DWORD = 0x0000C000;

pub const SW_HIDE: c_int = 0;
pub const SW_SHOWNORMAL: c_int = 1;
pub const SW_NORMAL: c_int = 1;
pub const SW_SHOWMINIMIZED: c_int = 2;
pub const SW_SHOWMAXIMIZED: c_int = 3;
pub const SW_MAXIMIZE: c_int = 3;
pub const SW_SHOWNOACTIVATE: c_int = 4;
pub const SW_SHOW: c_int = 5;
pub const SW_MINIMIZE: c_int = 6;
pub const SW_SHOWMINNOACTIVE: c_int = 7;
pub const SW_SHOWNA: c_int = 8;
pub const SW_RESTORE: c_int = 9;
pub const SW_SHOWDEFAULT: c_int = 10;
pub const SW_FORCEMINIMIZE: c_int = 11;
pub const SW_MAX: c_int = 11;

pub const SWP_NOSIZE: UINT = 0x0001;
pub const SWP_NOMOVE: UINT = 0x0002;
pub const SWP_NOZORDER: UINT = 0x0004;
pub const SWP_NOREDRAW: UINT = 0x0008;
pub const SWP_NOACTIVATE: UINT = 0x0010;
pub const SWP_FRAMECHANGED: UINT = 0x0020;
pub const SWP_SHOWWINDOW: UINT = 0x0040;
pub const SWP_HIDEWINDOW: UINT = 0x0080;
pub const SWP_NOCOPYBITS: UINT = 0x0100;
pub const SWP_NOOWNERZORDER: UINT = 0x0200;
pub const SWP_NOSENDCHANGING: UINT = 0x0400;
pub const SWP_DRAWFRAME: UINT = SWP_FRAMECHANGED;
pub const SWP_NOREPOSITION: UINT = SWP_NOOWNERZORDER;
pub const SWP_DEFERERASE: UINT = 0x2000;
pub const SWP_ASYNCWINDOWPOS: UINT = 0x4000;

pub const VK_LBUTTON: WPARAM = 0x01;
pub const VK_RBUTTON: WPARAM = 0x02;
pub const VK_CANCEL: WPARAM = 0x03;
pub const VK_MBUTTON: WPARAM = 0x04;
pub const VK_XBUTTON1: WPARAM = 0x05;
pub const VK_XBUTTON2: WPARAM = 0x06;
pub const VK_BACK: WPARAM = 0x08;
pub const VK_TAB: WPARAM = 0x09;
pub const VK_CLEAR: WPARAM = 0x0C;
pub const VK_RETURN: WPARAM = 0x0D;
pub const VK_SHIFT: WPARAM = 0x10;
pub const VK_CONTROL: WPARAM = 0x11;
pub const VK_MENU: WPARAM = 0x12;
pub const VK_PAUSE: WPARAM = 0x13;
pub const VK_CAPITAL: WPARAM = 0x14;
pub const VK_KANA: WPARAM = 0x15;
pub const VK_HANGUEL: WPARAM = 0x15;
pub const VK_HANGUL: WPARAM = 0x15;
pub const VK_JUNJA: WPARAM = 0x17;
pub const VK_FINAL: WPARAM = 0x18;
pub const VK_HANJA: WPARAM = 0x19;
pub const VK_KANJI: WPARAM = 0x19;
pub const VK_ESCAPE: WPARAM = 0x1B;
pub const VK_CONVERT: WPARAM = 0x1C;
pub const VK_NONCONVERT: WPARAM = 0x1D;
pub const VK_ACCEPT: WPARAM = 0x1E;
pub const VK_MODECHANGE: WPARAM = 0x1F;
pub const VK_SPACE: WPARAM = 0x20;
pub const VK_PRIOR: WPARAM = 0x21;
pub const VK_NEXT: WPARAM = 0x22;
pub const VK_END: WPARAM = 0x23;
pub const VK_HOME: WPARAM = 0x24;
pub const VK_LEFT: WPARAM = 0x25;
pub const VK_UP: WPARAM = 0x26;
pub const VK_RIGHT: WPARAM = 0x27;
pub const VK_DOWN: WPARAM = 0x28;
pub const VK_SELECT: WPARAM = 0x29;
pub const VK_PRINT: WPARAM = 0x2A;
pub const VK_EXECUTE: WPARAM = 0x2B;
pub const VK_SNAPSHOT: WPARAM = 0x2C;
pub const VK_INSERT: WPARAM = 0x2D;
pub const VK_DELETE: WPARAM = 0x2E;
pub const VK_HELP: WPARAM = 0x2F;
pub const VK_LWIN: WPARAM = 0x5B;
pub const VK_RWIN: WPARAM = 0x5C;
pub const VK_APPS: WPARAM = 0x5D;
pub const VK_SLEEP: WPARAM = 0x5F;
pub const VK_NUMPAD0: WPARAM = 0x60;
pub const VK_NUMPAD1: WPARAM = 0x61;
pub const VK_NUMPAD2: WPARAM = 0x62;
pub const VK_NUMPAD3: WPARAM = 0x63;
pub const VK_NUMPAD4: WPARAM = 0x64;
pub const VK_NUMPAD5: WPARAM = 0x65;
pub const VK_NUMPAD6: WPARAM = 0x66;
pub const VK_NUMPAD7: WPARAM = 0x67;
pub const VK_NUMPAD8: WPARAM = 0x68;
pub const VK_NUMPAD9: WPARAM = 0x69;
pub const VK_MULTIPLY: WPARAM = 0x6A;
pub const VK_ADD: WPARAM = 0x6B;
pub const VK_SEPARATOR: WPARAM = 0x6C;
pub const VK_SUBTRACT: WPARAM = 0x6D;
pub const VK_DECIMAL: WPARAM = 0x6E;
pub const VK_DIVIDE: WPARAM = 0x6F;
pub const VK_F1: WPARAM = 0x70;
pub const VK_F2: WPARAM = 0x71;
pub const VK_F3: WPARAM = 0x72;
pub const VK_F4: WPARAM = 0x73;
pub const VK_F5: WPARAM = 0x74;
pub const VK_F6: WPARAM = 0x75;
pub const VK_F7: WPARAM = 0x76;
pub const VK_F8: WPARAM = 0x77;
pub const VK_F9: WPARAM = 0x78;
pub const VK_F10: WPARAM = 0x79;
pub const VK_F11: WPARAM = 0x7A;
pub const VK_F12: WPARAM = 0x7B;
pub const VK_F13: WPARAM = 0x7C;
pub const VK_F14: WPARAM = 0x7D;
pub const VK_F15: WPARAM = 0x7E;
pub const VK_F16: WPARAM = 0x7F;
pub const VK_F17: WPARAM = 0x80;
pub const VK_F18: WPARAM = 0x81;
pub const VK_F19: WPARAM = 0x82;
pub const VK_F20: WPARAM = 0x83;
pub const VK_F21: WPARAM = 0x84;
pub const VK_F22: WPARAM = 0x85;
pub const VK_F23: WPARAM = 0x86;
pub const VK_F24: WPARAM = 0x87;
pub const VK_NUMLOCK: WPARAM = 0x90;
pub const VK_SCROLL: WPARAM = 0x91;
pub const VK_OEM_NEC_EQUAL: WPARAM = 0x92;
pub const VK_OEM_FJ_JISHO: WPARAM = 0x92;
pub const VK_OEM_FJ_MASSHOU: WPARAM = 0x93;
pub const VK_OEM_FJ_TOUROKU: WPARAM = 0x94;
pub const VK_OEM_FJ_LOYA: WPARAM = 0x95;
pub const VK_OEM_FJ_ROYA: WPARAM = 0x96;
pub const VK_LSHIFT: WPARAM = 0xA0;
pub const VK_RSHIFT: WPARAM = 0xA1;
pub const VK_LCONTROL: WPARAM = 0xA2;
pub const VK_RCONTROL: WPARAM = 0xA3;
pub const VK_LMENU: WPARAM = 0xA4;
pub const VK_RMENU: WPARAM = 0xA5;
pub const VK_BROWSER_BACK: WPARAM = 0xA6;
pub const VK_BROWSER_FORWARD: WPARAM = 0xA7;
pub const VK_BROWSER_REFRESH: WPARAM = 0xA8;
pub const VK_BROWSER_STOP: WPARAM = 0xA9;
pub const VK_BROWSER_SEARCH: WPARAM = 0xAA;
pub const VK_BROWSER_FAVORITES: WPARAM = 0xAB;
pub const VK_BROWSER_HOME: WPARAM = 0xAC;
pub const VK_VOLUME_MUTE: WPARAM = 0xAD;
pub const VK_VOLUME_DOWN: WPARAM = 0xAE;
pub const VK_VOLUME_UP: WPARAM = 0xAF;
pub const VK_MEDIA_NEXT_TRACK: WPARAM = 0xB0;
pub const VK_MEDIA_PREV_TRACK: WPARAM = 0xB1;
pub const VK_MEDIA_STOP: WPARAM = 0xB2;
pub const VK_MEDIA_PLAY_PAUSE: WPARAM = 0xB3;
pub const VK_LAUNCH_MAIL: WPARAM = 0xB4;
pub const VK_LAUNCH_MEDIA_SELECT: WPARAM = 0xB5;
pub const VK_LAUNCH_APP1: WPARAM = 0xB6;
pub const VK_LAUNCH_APP2: WPARAM = 0xB7;
pub const VK_OEM_1: WPARAM = 0xBA;
pub const VK_OEM_PLUS: WPARAM = 0xBB;
pub const VK_OEM_COMMA: WPARAM = 0xBC;
pub const VK_OEM_MINUS: WPARAM = 0xBD;
pub const VK_OEM_PERIOD: WPARAM = 0xBE;
pub const VK_OEM_2: WPARAM = 0xBF;
pub const VK_OEM_3: WPARAM = 0xC0;
pub const VK_OEM_4: WPARAM = 0xDB;
pub const VK_OEM_5: WPARAM = 0xDC;
pub const VK_OEM_6: WPARAM = 0xDD;
pub const VK_OEM_7: WPARAM = 0xDE;
pub const VK_OEM_8: WPARAM = 0xDF;
pub const VK_OEM_AX: WPARAM = 0xE1;
pub const VK_OEM_102: WPARAM = 0xE2;
pub const VK_ICO_HELP: WPARAM = 0xE3;
pub const VK_ICO_00: WPARAM = 0xE4;
pub const VK_PROCESSKEY: WPARAM = 0xE5;
pub const VK_ICO_CLEAR: WPARAM = 0xE6;
pub const VK_PACKET: WPARAM = 0xE7;
pub const VK_OEM_RESET: WPARAM = 0xE9;
pub const VK_OEM_JUMP: WPARAM = 0xEA;
pub const VK_OEM_PA1: WPARAM = 0xEB;
pub const VK_OEM_PA2: WPARAM = 0xEC;
pub const VK_OEM_PA3: WPARAM = 0xED;
pub const VK_OEM_WSCTRL: WPARAM = 0xEE;
pub const VK_OEM_CUSEL: WPARAM = 0xEF;
pub const VK_OEM_ATTN: WPARAM = 0xF0;
pub const VK_OEM_FINISH: WPARAM = 0xF1;
pub const VK_OEM_COPY: WPARAM = 0xF2;
pub const VK_OEM_AUTO: WPARAM = 0xF3;
pub const VK_OEM_ENLW: WPARAM = 0xF4;
pub const VK_OEM_BACKTAB: WPARAM = 0xF5;
pub const VK_ATTN: WPARAM = 0xF6;
pub const VK_CRSEL: WPARAM = 0xF7;
pub const VK_EXSEL: WPARAM = 0xF8;
pub const VK_EREOF: WPARAM = 0xF9;
pub const VK_PLAY: WPARAM = 0xFA;
pub const VK_ZOOM: WPARAM = 0xFB;
pub const VK_NONAME: WPARAM = 0xFC;
pub const VK_PA1: WPARAM = 0xFD;
pub const VK_OEM_CLEAR: WPARAM = 0xFE;

pub const WM_NULL: UINT = 0x0000;
pub const WM_CREATE: UINT = 0x0001;
pub const WM_DESTROY: UINT = 0x0002;
pub const WM_MOVE: UINT = 0x0003;
pub const WM_SIZE: UINT = 0x0005;
pub const WM_ACTIVATE: UINT = 0x0006;
pub const WM_SETFOCUS: UINT = 0x0007;
pub const WM_KILLFOCUS: UINT = 0x0008;
pub const WM_ENABLE: UINT = 0x000A;
pub const WM_SETREDRAW: UINT = 0x000B;
pub const WM_SETTEXT: UINT = 0x000C;
pub const WM_GETTEXT: UINT = 0x000D;
pub const WM_GETTEXTLENGTH: UINT = 0x000E;
pub const WM_PAINT: UINT = 0x000F;
pub const WM_CLOSE: UINT = 0x0010;
pub const WM_QUERYENDSESSION: UINT = 0x0011;
pub const WM_QUERYOPEN: UINT = 0x0013;
pub const WM_ENDSESSION: UINT = 0x0016;
pub const WM_QUIT: UINT = 0x0012;
pub const WM_ERASEBKGND: UINT = 0x0014;
pub const WM_SYSCOLORCHANGE: UINT = 0x0015;
pub const WM_SHOWWINDOW: UINT = 0x0018;
pub const WM_WININICHANGE: UINT = 0x001A;
pub const WM_SETTINGCHANGE: UINT = WM_WININICHANGE;
pub const WM_DEVMODECHANGE: UINT = 0x001B;
pub const WM_ACTIVATEAPP: UINT = 0x001C;
pub const WM_FONTCHANGE: UINT = 0x001D;
pub const WM_TIMECHANGE: UINT = 0x001E;
pub const WM_CANCELMODE: UINT = 0x001F;
pub const WM_SETCURSOR: UINT = 0x0020;
pub const WM_MOUSEACTIVATE: UINT = 0x0021;
pub const WM_CHILDACTIVATE: UINT = 0x0022;
pub const WM_QUEUESYNC: UINT = 0x0023;
pub const WM_GETMINMAXINFO: UINT = 0x0024;
pub const WM_PAINTICON: UINT = 0x0026;
pub const WM_ICONERASEBKGND: UINT = 0x0027;
pub const WM_NEXTDLGCTL: UINT = 0x0028;
pub const WM_SPOOLERSTATUS: UINT = 0x002A;
pub const WM_DRAWITEM: UINT = 0x002B;
pub const WM_MEASUREITEM: UINT = 0x002C;
pub const WM_DELETEITEM: UINT = 0x002D;
pub const WM_VKEYTOITEM: UINT = 0x002E;
pub const WM_CHARTOITEM: UINT = 0x002F;
pub const WM_SETFONT: UINT = 0x0030;
pub const WM_GETFONT: UINT = 0x0031;
pub const WM_SETHOTKEY: UINT = 0x0032;
pub const WM_GETHOTKEY: UINT = 0x0033;
pub const WM_QUERYDRAGICON: UINT = 0x0037;
pub const WM_COMPAREITEM: UINT = 0x0039;
pub const WM_GETOBJECT: UINT = 0x003D;
pub const WM_COMPACTING: UINT = 0x0041;
pub const WM_COMMNOTIFY: UINT = 0x0044;
pub const WM_WINDOWPOSCHANGING: UINT = 0x0046;
pub const WM_WINDOWPOSCHANGED: UINT = 0x0047;
pub const WM_POWER: UINT = 0x0048;
pub const WM_COPYDATA: UINT = 0x004A;
pub const WM_CANCELJOURNAL: UINT = 0x004B;
pub const WM_NOTIFY: UINT = 0x004E;
pub const WM_INPUTLANGCHANGEREQUEST: UINT = 0x0050;
pub const WM_INPUTLANGCHANGE: UINT = 0x0051;
pub const WM_TCARD: UINT = 0x0052;
pub const WM_HELP: UINT = 0x0053;
pub const WM_USERCHANGED: UINT = 0x0054;
pub const WM_NOTIFYFORMAT: UINT = 0x0055;
pub const WM_CONTEXTMENU: UINT = 0x007B;
pub const WM_STYLECHANGING: UINT = 0x007C;
pub const WM_STYLECHANGED: UINT = 0x007D;
pub const WM_DISPLAYCHANGE: UINT = 0x007E;
pub const WM_GETICON: UINT = 0x007F;
pub const WM_SETICON: UINT = 0x0080;
pub const WM_NCCREATE: UINT = 0x0081;
pub const WM_NCDESTROY: UINT = 0x0082;
pub const WM_NCCALCSIZE: UINT = 0x0083;
pub const WM_NCHITTEST: UINT = 0x0084;
pub const WM_NCPAINT: UINT = 0x0085;
pub const WM_NCACTIVATE: UINT = 0x0086;
pub const WM_GETDLGCODE: UINT = 0x0087;
pub const WM_SYNCPAINT: UINT = 0x0088;
pub const WM_NCMOUSEMOVE: UINT = 0x00A0;
pub const WM_NCLBUTTONDOWN: UINT = 0x00A1;
pub const WM_NCLBUTTONUP: UINT = 0x00A2;
pub const WM_NCLBUTTONDBLCLK: UINT = 0x00A3;
pub const WM_NCRBUTTONDOWN: UINT = 0x00A4;
pub const WM_NCRBUTTONUP: UINT = 0x00A5;
pub const WM_NCRBUTTONDBLCLK: UINT = 0x00A6;
pub const WM_NCMBUTTONDOWN: UINT = 0x00A7;
pub const WM_NCMBUTTONUP: UINT = 0x00A8;
pub const WM_NCMBUTTONDBLCLK: UINT = 0x00A9;
pub const WM_NCXBUTTONDOWN: UINT = 0x00AB;
pub const WM_NCXBUTTONUP: UINT = 0x00AC;
pub const WM_NCXBUTTONDBLCLK: UINT = 0x00AD;
pub const WM_INPUT_DEVICE_CHANGE: UINT = 0x00FE;
pub const WM_INPUT: UINT = 0x00FF;
pub const WM_KEYFIRST: UINT = 0x0100;
pub const WM_KEYDOWN: UINT = 0x0100;
pub const WM_KEYUP: UINT = 0x0101;
pub const WM_CHAR: UINT = 0x0102;
pub const WM_DEADCHAR: UINT = 0x0103;
pub const WM_SYSKEYDOWN: UINT = 0x0104;
pub const WM_SYSKEYUP: UINT = 0x0105;
pub const WM_SYSCHAR: UINT = 0x0106;
pub const WM_SYSDEADCHAR: UINT = 0x0107;
pub const WM_UNICHAR: UINT = 0x0109;
pub const WM_KEYLAST: UINT = 0x0109;
pub const WM_IME_STARTCOMPOSITION: UINT = 0x010D;
pub const WM_IME_ENDCOMPOSITION: UINT = 0x010E;
pub const WM_IME_COMPOSITION: UINT = 0x010F;
pub const WM_IME_KEYLAST: UINT = 0x010F;
pub const WM_INITDIALOG: UINT = 0x0110;
pub const WM_COMMAND: UINT = 0x0111;
pub const WM_SYSCOMMAND: UINT = 0x0112;
pub const WM_TIMER: UINT = 0x0113;
pub const WM_HSCROLL: UINT = 0x0114;
pub const WM_VSCROLL: UINT = 0x0115;
pub const WM_INITMENU: UINT = 0x0116;
pub const WM_INITMENUPOPUP: UINT = 0x0117;
pub const WM_GESTURE: UINT = 0x0119;
pub const WM_GESTURENOTIFY: UINT = 0x011A;
pub const WM_MENUSELECT: UINT = 0x011F;
pub const WM_MENUCHAR: UINT = 0x0120;
pub const WM_ENTERIDLE: UINT = 0x0121;
pub const WM_MENURBUTTONUP: UINT = 0x0122;
pub const WM_MENUDRAG: UINT = 0x0123;
pub const WM_MENUGETOBJECT: UINT = 0x0124;
pub const WM_UNINITMENUPOPUP: UINT = 0x0125;
pub const WM_MENUCOMMAND: UINT = 0x0126;
pub const WM_CHANGEUISTATE: UINT = 0x0127;
pub const WM_UPDATEUISTATE: UINT = 0x0128;
pub const WM_QUERYUISTATE: UINT = 0x0129;
pub const WM_CTLCOLORMSGBOX: UINT = 0x0132;
pub const WM_CTLCOLOREDIT: UINT = 0x0133;
pub const WM_CTLCOLORLISTBOX: UINT = 0x0134;
pub const WM_CTLCOLORBTN: UINT = 0x0135;
pub const WM_CTLCOLORDLG: UINT = 0x0136;
pub const WM_CTLCOLORSCROLLBAR: UINT = 0x0137;
pub const WM_CTLCOLORSTATIC: UINT = 0x0138;
pub const WM_MOUSEFIRST: UINT = 0x0200;
pub const WM_MOUSEMOVE: UINT = 0x0200;
pub const WM_LBUTTONDOWN: UINT = 0x0201;
pub const WM_LBUTTONUP: UINT = 0x0202;
pub const WM_LBUTTONDBLCLK: UINT = 0x0203;
pub const WM_RBUTTONDOWN: UINT = 0x0204;
pub const WM_RBUTTONUP: UINT = 0x0205;
pub const WM_RBUTTONDBLCLK: UINT = 0x0206;
pub const WM_MBUTTONDOWN: UINT = 0x0207;
pub const WM_MBUTTONUP: UINT = 0x0208;
pub const WM_MBUTTONDBLCLK: UINT = 0x0209;
pub const WM_MOUSEWHEEL: UINT = 0x020A;
pub const WM_XBUTTONDOWN: UINT = 0x020B;
pub const WM_XBUTTONUP: UINT = 0x020C;
pub const WM_XBUTTONDBLCLK: UINT = 0x020D;
pub const WM_MOUSEHWHEEL: UINT = 0x020E;
pub const WM_MOUSELAST: UINT = 0x020E;
pub const WM_PARENTNOTIFY: UINT = 0x0210;
pub const WM_ENTERMENULOOP: UINT = 0x0211;
pub const WM_EXITMENULOOP: UINT = 0x0212;
pub const WM_NEXTMENU: UINT = 0x0213;
pub const WM_SIZING: UINT = 0x0214;
pub const WM_CAPTURECHANGED: UINT = 0x0215;
pub const WM_MOVING: UINT = 0x0216;
pub const WM_POWERBROADCAST: UINT = 0x0218;
pub const WM_DEVICECHANGE: UINT = 0x0219;
pub const WM_MDICREATE: UINT = 0x0220;
pub const WM_MDIDESTROY: UINT = 0x0221;
pub const WM_MDIACTIVATE: UINT = 0x0222;
pub const WM_MDIRESTORE: UINT = 0x0223;
pub const WM_MDINEXT: UINT = 0x0224;
pub const WM_MDIMAXIMIZE: UINT = 0x0225;
pub const WM_MDITILE: UINT = 0x0226;
pub const WM_MDICASCADE: UINT = 0x0227;
pub const WM_MDIICONARRANGE: UINT = 0x0228;
pub const WM_MDIGETACTIVE: UINT = 0x0229;
pub const WM_MDISETMENU: UINT = 0x0230;
pub const WM_ENTERSIZEMOVE: UINT = 0x0231;
pub const WM_EXITSIZEMOVE: UINT = 0x0232;
pub const WM_DROPFILES: UINT = 0x0233;
pub const WM_MDIREFRESHMENU: UINT = 0x0234;
pub const WM_POINTERDEVICECHANGE: UINT = 0x238;
pub const WM_POINTERDEVICEINRANGE: UINT = 0x239;
pub const WM_POINTERDEVICEOUTOFRANGE: UINT = 0x23A;
pub const WM_TOUCH: UINT = 0x0240;
pub const WM_NCPOINTERUPDATE: UINT = 0x0241;
pub const WM_NCPOINTERDOWN: UINT = 0x0242;
pub const WM_NCPOINTERUP: UINT = 0x0243;
pub const WM_POINTERUPDATE: UINT = 0x0245;
pub const WM_POINTERDOWN: UINT = 0x0246;
pub const WM_POINTERUP: UINT = 0x0247;
pub const WM_POINTERENTER: UINT = 0x0249;
pub const WM_POINTERLEAVE: UINT = 0x024A;
pub const WM_POINTERACTIVATE: UINT = 0x024B;
pub const WM_POINTERCAPTURECHANGED: UINT = 0x024C;
pub const WM_TOUCHHITTESTING: UINT = 0x024D;
pub const WM_POINTERWHEEL: UINT = 0x024E;
pub const WM_POINTERHWHEEL: UINT = 0x024F;
pub const WM_IME_SETCONTEXT: UINT = 0x0281;
pub const WM_IME_NOTIFY: UINT = 0x0282;
pub const WM_IME_CONTROL: UINT = 0x0283;
pub const WM_IME_COMPOSITIONFULL: UINT = 0x0284;
pub const WM_IME_SELECT: UINT = 0x0285;
pub const WM_IME_CHAR: UINT = 0x0286;
pub const WM_IME_REQUEST: UINT = 0x0288;
pub const WM_IME_KEYDOWN: UINT = 0x0290;
pub const WM_IME_KEYUP: UINT = 0x0291;
pub const WM_MOUSEHOVER: UINT = 0x02A1;
pub const WM_MOUSELEAVE: UINT = 0x02A3;
pub const WM_NCMOUSEHOVER: UINT = 0x02A0;
pub const WM_NCMOUSELEAVE: UINT = 0x02A2;
pub const WM_WTSSESSION_CHANGE: UINT = 0x02B1;
pub const WM_TABLET_FIRST: UINT = 0x02c0;
pub const WM_TABLET_LAST: UINT = 0x02df;
pub const WM_DPICHANGED: UINT = 0x02E0;
pub const WM_CUT: UINT = 0x0300;
pub const WM_COPY: UINT = 0x0301;
pub const WM_PASTE: UINT = 0x0302;
pub const WM_CLEAR: UINT = 0x0303;
pub const WM_UNDO: UINT = 0x0304;
pub const WM_RENDERFORMAT: UINT = 0x0305;
pub const WM_RENDERALLFORMATS: UINT = 0x0306;
pub const WM_DESTROYCLIPBOARD: UINT = 0x0307;
pub const WM_DRAWCLIPBOARD: UINT = 0x0308;
pub const WM_PAINTCLIPBOARD: UINT = 0x0309;
pub const WM_VSCROLLCLIPBOARD: UINT = 0x030A;
pub const WM_SIZECLIPBOARD: UINT = 0x030B;
pub const WM_ASKCBFORMATNAME: UINT = 0x030C;
pub const WM_CHANGECBCHAIN: UINT = 0x030D;
pub const WM_HSCROLLCLIPBOARD: UINT = 0x030E;
pub const WM_QUERYNEWPALETTE: UINT = 0x030F;
pub const WM_PALETTEISCHANGING: UINT = 0x0310;
pub const WM_PALETTECHANGED: UINT = 0x0311;
pub const WM_HOTKEY: UINT = 0x0312;
pub const WM_PRINT: UINT = 0x0317;
pub const WM_PRINTCLIENT: UINT = 0x0318;
pub const WM_APPCOMMAND: UINT = 0x0319;
pub const WM_THEMECHANGED: UINT = 0x031A;
pub const WM_CLIPBOARDUPDATE: UINT = 0x031D;
pub const WM_DWMCOMPOSITIONCHANGED: UINT = 0x031E;
pub const WM_DWMNCRENDERINGCHANGED: UINT = 0x031F;
pub const WM_DWMCOLORIZATIONCOLORCHANGED: UINT = 0x0320;
pub const WM_DWMWINDOWMAXIMIZEDCHANGE: UINT = 0x0321;
pub const WM_DWMSENDICONICTHUMBNAIL: UINT = 0x0323;
pub const WM_DWMSENDICONICLIVEPREVIEWBITMAP: UINT = 0x0326;
pub const WM_GETTITLEBARINFOEX: UINT = 0x033F;
pub const WM_HANDHELDFIRST: UINT = 0x0358;
pub const WM_HANDHELDLAST: UINT = 0x035F;
pub const WM_AFXFIRST: UINT = 0x0360;
pub const WM_AFXLAST: UINT = 0x037F;
pub const WM_PENWINFIRST: UINT = 0x0380;
pub const WM_PENWINLAST: UINT = 0x038F;
pub const WM_APP: UINT = 0x8000;
pub const WM_USER: UINT = 0x0400;

pub const WS_OVERLAPPED: DWORD = 0x00000000;
pub const WS_POPUP: DWORD = 0x80000000;
pub const WS_CHILD: DWORD = 0x40000000;
pub const WS_MINIMIZE: DWORD = 0x20000000;
pub const WS_VISIBLE: DWORD = 0x10000000;
pub const WS_DISABLED: DWORD = 0x08000000;
pub const WS_CLIPSIBLINGS: DWORD = 0x04000000;
pub const WS_CLIPCHILDREN: DWORD = 0x02000000;
pub const WS_MAXIMIZE: DWORD = 0x01000000;
pub const WS_CAPTION: DWORD = 0x00C00000;
pub const WS_BORDER: DWORD = 0x00800000;
pub const WS_DLGFRAME: DWORD = 0x00400000;
pub const WS_VSCROLL: DWORD = 0x00200000;
pub const WS_HSCROLL: DWORD = 0x00100000;
pub const WS_SYSMENU: DWORD = 0x00080000;
pub const WS_THICKFRAME: DWORD = 0x00040000;
pub const WS_GROUP: DWORD = 0x00020000;
pub const WS_TABSTOP: DWORD = 0x00010000;
pub const WS_MINIMIZEBOX: DWORD = 0x00020000;
pub const WS_MAXIMIZEBOX: DWORD = 0x00010000;
pub const WS_TILED: DWORD = WS_OVERLAPPED;
pub const WS_ICONIC: DWORD = WS_MINIMIZE;
pub const WS_SIZEBOX: DWORD = WS_THICKFRAME;
pub const WS_TILEDWINDOW: DWORD = WS_OVERLAPPEDWINDOW;
pub const WS_OVERLAPPEDWINDOW: DWORD = WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_THICKFRAME | WS_MINIMIZEBOX | WS_MAXIMIZEBOX;
pub const WS_POPUPWINDOW: DWORD = WS_POPUP | WS_BORDER | WS_SYSMENU;
pub const WS_CHILDWINDOW: DWORD = WS_CHILD;

pub const WS_EX_DLGMODALFRAME: DWORD = 0x00000001;
pub const WS_EX_NOPARENTNOTIFY: DWORD = 0x00000004;
pub const WS_EX_TOPMOST: DWORD = 0x00000008;
pub const WS_EX_ACCEPTFILES: DWORD = 0x00000010;
pub const WS_EX_TRANSPARENT: DWORD = 0x00000020;
pub const WS_EX_MDICHILD: DWORD = 0x00000040;
pub const WS_EX_TOOLWINDOW: DWORD = 0x00000080;
pub const WS_EX_WINDOWEDGE: DWORD = 0x00000100;
pub const WS_EX_CLIENTEDGE: DWORD = 0x00000200;
pub const WS_EX_CONTEXTHELP: DWORD = 0x00000400;
pub const WS_EX_RIGHT: DWORD = 0x00001000;
pub const WS_EX_LEFT: DWORD = 0x00000000;
pub const WS_EX_RTLREADING: DWORD = 0x00002000;
pub const WS_EX_LTRREADING: DWORD = 0x00000000;
pub const WS_EX_LEFTSCROLLBAR: DWORD = 0x00004000;
pub const WS_EX_RIGHTSCROLLBAR: DWORD = 0x00000000;
pub const WS_EX_CONTROLPARENT: DWORD = 0x00010000;
pub const WS_EX_STATICEDGE: DWORD = 0x00020000;
pub const WS_EX_APPWINDOW: DWORD = 0x00040000;
pub const WS_EX_OVERLAPPEDWINDOW: DWORD = WS_EX_WINDOWEDGE | WS_EX_CLIENTEDGE;
pub const WS_EX_PALETTEWINDOW: DWORD = WS_EX_WINDOWEDGE | WS_EX_TOOLWINDOW | WS_EX_TOPMOST;
pub const WS_EX_LAYERED: DWORD = 0x00080000;
pub const WS_EX_NOINHERITLAYOUT: DWORD = 0x00100000;
pub const WS_EX_NOREDIRECTIONBITMAP: DWORD = 0x00200000;
pub const WS_EX_LAYOUTRTL: DWORD = 0x00400000;
pub const WS_EX_COMPOSITED: DWORD = 0x02000000;
pub const WS_EX_NOACTIVATE: DWORD = 0x08000000;

pub type WNDPROC = unsafe extern "system" fn(HWND, UINT, WPARAM, LPARAM) -> LRESULT;
#[repr(C)]
#[derive(Copy)]
pub struct MSG {
    pub hwnd: HWND,
    pub message: UINT,
    pub wParam: WPARAM,
    pub lParam: LPARAM,
    pub time: DWORD,
    pub pt: POINT,
}
pub type PMSG = *mut MSG;
pub type NPMSG = *mut MSG;
pub type LPMSG = *mut MSG;
#[repr(C)]
#[derive(Copy)]
pub struct PAINTSTRUCT {
    pub hdc: HDC,
    pub fErase: BOOL,
    pub rcPaint: RECT,
    pub fRestore: BOOL,
    pub fIncUpdate: BOOL,
    pub rgbReserved: [BYTE; 32],
}
pub type PPAINTSTRUCT = *mut PAINTSTRUCT;
pub type NPPAINTSTRUCT = *mut PAINTSTRUCT;
pub type LPPAINTSTRUCT = *mut PAINTSTRUCT;
#[repr(C)]
#[derive(Copy)]
pub struct WINDOWPLACEMENT {
    pub length: UINT,
    pub flags: UINT,
    pub showCmd: UINT,
    pub ptMinPosition: POINT,
    pub ptMaxPosition: POINT,
    pub rcNormalPosition: RECT,
}
pub type PWINDOWPLACEMENT = *mut WINDOWPLACEMENT;
pub type LPWINDOWPLACEMENT = *mut WINDOWPLACEMENT;
#[repr(C)]
#[derive(Copy)]
pub struct WNDCLASSEXW {
    pub cbSize: UINT,
    pub style: UINT,
    pub lpfnWndProc: WNDPROC,
    pub cbClsExtra: c_int,
    pub cbWndExtra: c_int,
    pub hInstance: HINSTANCE,
    pub hIcon: HICON,
    pub hCursor: HCURSOR,
    pub hbrBackground: HBRUSH,
    pub lpszMenuName: LPCWSTR,
    pub lpszClassName: LPCWSTR,
    pub hIconSm: HICON,
}
pub type PWNDCLASSEXW = *mut WNDCLASSEXW;
pub type NPWNDCLASSEXW = *mut WNDCLASSEXW;
pub type LPWNDCLASSEXW = *mut WNDCLASSEXW;
#[repr(C)]
#[derive(Copy)]
pub struct WNDCLASSW {
    pub style: UINT,
    pub lpfnWndProc: WNDPROC,
    pub cbClsExtra: c_int,
    pub cbWndExtra: c_int,
    pub hInstance: HINSTANCE,
    pub hIcon: HICON,
    pub hCursor: HCURSOR,
    pub hbrBackground: HBRUSH,
    pub lpszMenuName: LPCWSTR,
    pub lpszClassName: LPCWSTR
}
pub type PWNDCLASSW = *mut WNDCLASSW;
pub type NPWNDCLASSW = *mut WNDCLASSW;
pub type LPWNDCLASSW = *mut WNDCLASSW;

//-------------------------------------------------------------------------------------------------
// wingdi.h
// GDI procedure declarations, constant definitions and macros
//-------------------------------------------------------------------------------------------------
pub const DISPLAY_DEVICE_ATTACHED_TO_DESKTOP: DWORD = 0x00000001;
pub const DISPLAY_DEVICE_MULTI_DRIVER: DWORD = 0x00000002;
pub const DISPLAY_DEVICE_PRIMARY_DEVICE: DWORD = 0x00000004;
pub const DISPLAY_DEVICE_MIRRORING_DRIVER: DWORD = 0x00000008;
pub const DISPLAY_DEVICE_VGA_COMPATIBLE: DWORD = 0x00000010;
pub const DISPLAY_DEVICE_REMOVABLE: DWORD = 0x00000020;
pub const DISPLAY_DEVICE_ACC_DRIVER: DWORD = 0x00000040;
pub const DISPLAY_DEVICE_MODESPRUNED: DWORD = 0x08000000;
pub const DISPLAY_DEVICE_REMOTE: DWORD = 0x04000000;
pub const DISPLAY_DEVICE_DISCONNECT: DWORD = 0x02000000;
pub const DISPLAY_DEVICE_TS_COMPATIBLE: DWORD = 0x00200000;
pub const DISPLAY_DEVICE_UNSAFE_MODES_ON: DWORD = 0x00080000;
pub const DISPLAY_DEVICE_ACTIVE: DWORD = 0x00000001;
pub const DISPLAY_DEVICE_ATTACHED: DWORD = 0x00000002;

pub const DM_ORIENTATION: DWORD = 0x00000001;
pub const DM_PAPERSIZE: DWORD = 0x00000002;
pub const DM_PAPERLENGTH: DWORD = 0x00000004;
pub const DM_PAPERWIDTH: DWORD = 0x00000008;
pub const DM_SCALE: DWORD = 0x00000010;
pub const DM_POSITION: DWORD = 0x00000020;
pub const DM_NUP: DWORD = 0x00000040;
pub const DM_DISPLAYORIENTATION: DWORD = 0x00000080;
pub const DM_COPIES: DWORD = 0x00000100;
pub const DM_DEFAULTSOURCE: DWORD = 0x00000200;
pub const DM_PRINTQUALITY: DWORD = 0x00000400;
pub const DM_COLOR: DWORD = 0x00000800;
pub const DM_DUPLEX: DWORD = 0x00001000;
pub const DM_YRESOLUTION: DWORD = 0x00002000;
pub const DM_TTOPTION: DWORD = 0x00004000;
pub const DM_COLLATE: DWORD = 0x00008000;
pub const DM_FORMNAME: DWORD = 0x00010000;
pub const DM_LOGPIXELS: DWORD = 0x00020000;
pub const DM_BITSPERPEL: DWORD = 0x00040000;
pub const DM_PELSWIDTH: DWORD = 0x00080000;
pub const DM_PELSHEIGHT: DWORD = 0x00100000;
pub const DM_DISPLAYFLAGS: DWORD = 0x00200000;
pub const DM_DISPLAYFREQUENCY: DWORD = 0x00400000;
pub const DM_ICMMETHOD: DWORD = 0x00800000;
pub const DM_ICMINTENT: DWORD = 0x01000000;
pub const DM_MEDIATYPE: DWORD = 0x02000000;
pub const DM_DITHERTYPE: DWORD = 0x04000000;
pub const DM_PANNINGWIDTH: DWORD = 0x08000000;
pub const DM_PANNINGHEIGHT: DWORD = 0x10000000;
pub const DM_DISPLAYFIXEDOUTPUT: DWORD = 0x20000000;

pub const PFD_TYPE_RGBA: BYTE = 0;
pub const PFD_TYPE_COLORINDEX: BYTE = 1;
pub const PFD_MAIN_PLANE: BYTE = 0;
pub const PFD_OVERLAY_PLANE: BYTE = 1;
pub const PFD_UNDERLAY_PLANE: BYTE = -1;
pub const PFD_DOUBLEBUFFER: DWORD = 0x00000001;
pub const PFD_STEREO: DWORD = 0x00000002;
pub const PFD_DRAW_TO_WINDOW: DWORD = 0x00000004;
pub const PFD_DRAW_TO_BITMAP: DWORD = 0x00000008;
pub const PFD_SUPPORT_GDI: DWORD = 0x00000010;
pub const PFD_SUPPORT_OPENGL: DWORD = 0x00000020;
pub const PFD_GENERIC_FORMAT: DWORD = 0x00000040;
pub const PFD_NEED_PALETTE: DWORD = 0x00000080;
pub const PFD_NEED_SYSTEM_PALETTE: DWORD = 0x00000100;
pub const PFD_SWAP_EXCHANGE: DWORD = 0x00000200;
pub const PFD_SWAP_COPY: DWORD = 0x00000400;
pub const PFD_SWAP_LAYER_BUFFERS: DWORD = 0x00000800;
pub const PFD_GENERIC_ACCELERATED: DWORD = 0x00001000;
pub const PFD_SUPPORT_DIRECTDRAW: DWORD = 0x00002000;
pub const PFD_DIRECT3D_ACCELERATED: DWORD = 0x00004000;
pub const PFD_SUPPORT_COMPOSITION: DWORD = 0x00008000;
pub const PFD_DEPTH_DONTCARE: DWORD = 0x20000000;
pub const PFD_DOUBLEBUFFER_DONTCARE: DWORD = 0x40000000;
pub const PFD_STEREO_DONTCARE: DWORD = 0x80000000;

pub const CCHDEVICENAME: usize = 32;
pub const CCHFORMNAME: usize = 32;
#[repr(C)]
#[derive(Copy)]
pub struct DEVMODEW {
    pub dmDeviceName: [WCHAR; CCHDEVICENAME],
    pub dmSpecVersion: WORD,
    pub dmDriverVersion: WORD,
    pub dmSize: WORD,
    pub dmDriverExtra: WORD,
    pub dmFields: DWORD,
    pub union1: [u8; 16],
    pub dmColor: c_short,
    pub dmDuplex: c_short,
    pub dmYResolution: c_short,
    pub dmTTOption: c_short,
    pub dmCollate: c_short,
    pub dmFormName: [WCHAR; CCHFORMNAME],
    pub dmLogPixels: WORD,
    pub dmBitsPerPel: DWORD,
    pub dmPelsWidth: DWORD,
    pub dmPelsHeight: DWORD,
    pub dmDisplayFlags: DWORD,
    pub dmDisplayFrequency: DWORD,
    pub dmICMMethod: DWORD,
    pub dmICMIntent: DWORD,
    pub dmMediaType: DWORD,
    pub dmDitherType: DWORD,
    pub dmReserved1: DWORD,
    pub dmReserved2: DWORD,
    pub dmPanningWidth: DWORD,
    pub dmPanningHeight: DWORD,
}
pub type PDEVMODEW = *mut DEVMODEW;
pub type NPDEVMODEW = *mut DEVMODEW;
pub type LPDEVMODEW = *mut DEVMODEW;
#[repr(C)]
#[derive(Copy)]
pub struct DISPLAY_DEVICEW {
    pub cb: DWORD,
    pub DeviceName: [WCHAR; 32],
    pub DeviceString: [WCHAR; 128],
    pub StateFlags: DWORD,
    pub DeviceID: [WCHAR; 128],
    pub DeviceKey: [WCHAR; 128],
}
pub type PDISPLAY_DEVICEW = *mut DISPLAY_DEVICEW;
pub type LPDISPLAY_DEVICEW = *mut DISPLAY_DEVICEW;
#[repr(C)]
#[derive(Copy)]
pub struct PIXELFORMATDESCRIPTOR {
    pub nSize: WORD,
    pub nVersion: WORD,
    pub dwFlags: DWORD,
    pub iPixelType: BYTE,
    pub cColorBits: BYTE,
    pub cRedBits: BYTE,
    pub cRedShift: BYTE,
    pub cGreenBits: BYTE,
    pub cGreenShift: BYTE,
    pub cBlueBits: BYTE,
    pub cBlueShift: BYTE,
    pub cAlphaBits: BYTE,
    pub cAlphaShift: BYTE,
    pub cAccumBits: BYTE,
    pub cAccumRedBits: BYTE,
    pub cAccumGreenBits: BYTE,
    pub cAccumBlueBits: BYTE,
    pub cAccumAlphaBits: BYTE,
    pub cDepthBits: BYTE,
    pub cStencilBits: BYTE,
    pub cAuxBuffers: BYTE,
    pub iLayerType: BYTE,
    pub bReserved: BYTE,
    pub dwLayerMask: DWORD,
    pub dwVisibleMask: DWORD,
    pub dwDamageMask: DWORD,
}
pub type PPIXELFORMATDESCRIPTOR = *mut PIXELFORMATDESCRIPTOR;
pub type LPPIXELFORMATDESCRIPTOR = *mut PIXELFORMATDESCRIPTOR;

//-------------------------------------------------------------------------------------------------
// Constants
//-------------------------------------------------------------------------------------------------

// shlobj.h
// knownfolders.h
pub const FOLDERID_NetworkFolder: GUID = GUID { Data1: 0xD20BEEC4, Data2: 0x5CA8, Data3: 0x4905, Data4: [0xAE, 0x3B, 0xBF, 0x25, 0x1E, 0xA0, 0x9B, 0x53]};
pub const FOLDERID_ComputerFolder: GUID = GUID { Data1: 0x0AC0837C, Data2: 0xBBF8, Data3: 0x452A, Data4: [0x85, 0x0D, 0x79, 0xD0, 0x8E, 0x66, 0x7C, 0xA7]};
pub const FOLDERID_InternetFolder: GUID = GUID { Data1: 0x4D9F7874, Data2: 0x4E0C, Data3: 0x4904, Data4: [0x96, 0x7B, 0x40, 0xB0, 0xD2, 0x0C, 0x3E, 0x4B]};
pub const FOLDERID_ControlPanelFolder: GUID = GUID { Data1: 0x82A74AEB, Data2: 0xAEB4, Data3: 0x465C, Data4: [0xA0, 0x14, 0xD0, 0x97, 0xEE, 0x34, 0x6D, 0x63]};
pub const FOLDERID_PrintersFolder: GUID = GUID { Data1: 0x76FC4E2D, Data2: 0xD6AD, Data3: 0x4519, Data4: [0xA6, 0x63, 0x37, 0xBD, 0x56, 0x06, 0x81, 0x85]};
pub const FOLDERID_SyncManagerFolder: GUID = GUID { Data1: 0x43668BF8, Data2: 0xC14E, Data3: 0x49B2, Data4: [0x97, 0xC9, 0x74, 0x77, 0x84, 0xD7, 0x84, 0xB7]};
pub const FOLDERID_SyncSetupFolder: GUID = GUID { Data1: 0xf214138, Data2: 0xb1d3, Data3: 0x4a90, Data4: [0xbb, 0xa9, 0x27, 0xcb, 0xc0, 0xc5, 0x38, 0x9a]};
pub const FOLDERID_ConflictFolder: GUID = GUID { Data1: 0x4bfefb45, Data2: 0x347d, Data3: 0x4006, Data4: [0xa5, 0xbe, 0xac, 0x0c, 0xb0, 0x56, 0x71, 0x92]};
pub const FOLDERID_SyncResultsFolder: GUID = GUID { Data1: 0x289a9a43, Data2: 0xbe44, Data3: 0x4057, Data4: [0xa4, 0x1b, 0x58, 0x7a, 0x76, 0xd7, 0xe7, 0xf9]};
pub const FOLDERID_RecycleBinFolder: GUID = GUID { Data1: 0xB7534046, Data2: 0x3ECB, Data3: 0x4C18, Data4: [0xBE, 0x4E, 0x64, 0xCD, 0x4C, 0xB7, 0xD6, 0xAC]};
pub const FOLDERID_ConnectionsFolder: GUID = GUID { Data1: 0x6F0CD92B, Data2: 0x2E97, Data3: 0x45D1, Data4: [0x88, 0xFF, 0xB0, 0xD1, 0x86, 0xB8, 0xDE, 0xDD]};
pub const FOLDERID_Fonts: GUID = GUID { Data1: 0xFD228CB7, Data2: 0xAE11, Data3: 0x4AE3, Data4: [0x86, 0x4C, 0x16, 0xF3, 0x91, 0x0A, 0xB8, 0xFE]};
pub const FOLDERID_Desktop: GUID = GUID { Data1: 0xB4BFCC3A, Data2: 0xDB2C, Data3: 0x424C, Data4: [0xB0, 0x29, 0x7F, 0xE9, 0x9A, 0x87, 0xC6, 0x41]};
pub const FOLDERID_Startup: GUID = GUID { Data1: 0xB97D20BB, Data2: 0xF46A, Data3: 0x4C97, Data4: [0xBA, 0x10, 0x5E, 0x36, 0x08, 0x43, 0x08, 0x54]};
pub const FOLDERID_Programs: GUID = GUID { Data1: 0xA77F5D77, Data2: 0x2E2B, Data3: 0x44C3, Data4: [0xA6, 0xA2, 0xAB, 0xA6, 0x01, 0x05, 0x4A, 0x51]};
pub const FOLDERID_StartMenu: GUID = GUID { Data1: 0x625B53C3, Data2: 0xAB48, Data3: 0x4EC1, Data4: [0xBA, 0x1F, 0xA1, 0xEF, 0x41, 0x46, 0xFC, 0x19]};
pub const FOLDERID_Recent: GUID = GUID { Data1: 0xAE50C081, Data2: 0xEBD2, Data3: 0x438A, Data4: [0x86, 0x55, 0x8A, 0x09, 0x2E, 0x34, 0x98, 0x7A]};
pub const FOLDERID_SendTo: GUID = GUID { Data1: 0x8983036C, Data2: 0x27C0, Data3: 0x404B, Data4: [0x8F, 0x08, 0x10, 0x2D, 0x10, 0xDC, 0xFD, 0x74]};
pub const FOLDERID_Documents: GUID = GUID { Data1: 0xFDD39AD0, Data2: 0x238F, Data3: 0x46AF, Data4: [0xAD, 0xB4, 0x6C, 0x85, 0x48, 0x03, 0x69, 0xC7]};
pub const FOLDERID_Favorites: GUID = GUID { Data1: 0x1777F761, Data2: 0x68AD, Data3: 0x4D8A, Data4: [0x87, 0xBD, 0x30, 0xB7, 0x59, 0xFA, 0x33, 0xDD]};
pub const FOLDERID_NetHood: GUID = GUID { Data1: 0xC5ABBF53, Data2: 0xE17F, Data3: 0x4121, Data4: [0x89, 0x00, 0x86, 0x62, 0x6F, 0xC2, 0xC9, 0x73]};
pub const FOLDERID_PrintHood: GUID = GUID { Data1: 0x9274BD8D, Data2: 0xCFD1, Data3: 0x41C3, Data4: [0xB3, 0x5E, 0xB1, 0x3F, 0x55, 0xA7, 0x58, 0xF4]};
pub const FOLDERID_Templates: GUID = GUID { Data1: 0xA63293E8, Data2: 0x664E, Data3: 0x48DB, Data4: [0xA0, 0x79, 0xDF, 0x75, 0x9E, 0x05, 0x09, 0xF7]};
pub const FOLDERID_CommonStartup: GUID = GUID { Data1: 0x82A5EA35, Data2: 0xD9CD, Data3: 0x47C5, Data4: [0x96, 0x29, 0xE1, 0x5D, 0x2F, 0x71, 0x4E, 0x6E]};
pub const FOLDERID_CommonPrograms: GUID = GUID { Data1: 0x0139D44E, Data2: 0x6AFE, Data3: 0x49F2, Data4: [0x86, 0x90, 0x3D, 0xAF, 0xCA, 0xE6, 0xFF, 0xB8]};
pub const FOLDERID_CommonStartMenu: GUID = GUID { Data1: 0xA4115719, Data2: 0xD62E, Data3: 0x491D, Data4: [0xAA, 0x7C, 0xE7, 0x4B, 0x8B, 0xE3, 0xB0, 0x67]};
pub const FOLDERID_PublicDesktop: GUID = GUID { Data1: 0xC4AA340D, Data2: 0xF20F, Data3: 0x4863, Data4: [0xAF, 0xEF, 0xF8, 0x7E, 0xF2, 0xE6, 0xBA, 0x25]};
pub const FOLDERID_ProgramData: GUID = GUID { Data1: 0x62AB5D82, Data2: 0xFDC1, Data3: 0x4DC3, Data4: [0xA9, 0xDD, 0x07, 0x0D, 0x1D, 0x49, 0x5D, 0x97]};
pub const FOLDERID_CommonTemplates: GUID = GUID { Data1: 0xB94237E7, Data2: 0x57AC, Data3: 0x4347, Data4: [0x91, 0x51, 0xB0, 0x8C, 0x6C, 0x32, 0xD1, 0xF7]};
pub const FOLDERID_PublicDocuments: GUID = GUID { Data1: 0xED4824AF, Data2: 0xDCE4, Data3: 0x45A8, Data4: [0x81, 0xE2, 0xFC, 0x79, 0x65, 0x08, 0x36, 0x34]};
pub const FOLDERID_RoamingAppData: GUID = GUID { Data1: 0x3EB685DB, Data2: 0x65F9, Data3: 0x4CF6, Data4: [0xA0, 0x3A, 0xE3, 0xEF, 0x65, 0x72, 0x9F, 0x3D]};
pub const FOLDERID_LocalAppData: GUID = GUID { Data1: 0xF1B32785, Data2: 0x6FBA, Data3: 0x4FCF, Data4: [0x9D, 0x55, 0x7B, 0x8E, 0x7F, 0x15, 0x70, 0x91]};
pub const FOLDERID_LocalAppDataLow: GUID = GUID { Data1: 0xA520A1A4, Data2: 0x1780, Data3: 0x4FF6, Data4: [0xBD, 0x18, 0x16, 0x73, 0x43, 0xC5, 0xAF, 0x16]};
pub const FOLDERID_InternetCache: GUID = GUID { Data1: 0x352481E8, Data2: 0x33BE, Data3: 0x4251, Data4: [0xBA, 0x85, 0x60, 0x07, 0xCA, 0xED, 0xCF, 0x9D]};
pub const FOLDERID_Cookies: GUID = GUID { Data1: 0x2B0F765D, Data2: 0xC0E9, Data3: 0x4171, Data4: [0x90, 0x8E, 0x08, 0xA6, 0x11, 0xB8, 0x4F, 0xF6]};
pub const FOLDERID_History: GUID = GUID { Data1: 0xD9DC8A3B, Data2: 0xB784, Data3: 0x432E, Data4: [0xA7, 0x81, 0x5A, 0x11, 0x30, 0xA7, 0x59, 0x63]};
pub const FOLDERID_System: GUID = GUID { Data1: 0x1AC14E77, Data2: 0x02E7, Data3: 0x4E5D, Data4: [0xB7, 0x44, 0x2E, 0xB1, 0xAE, 0x51, 0x98, 0xB7]};
pub const FOLDERID_SystemX86: GUID = GUID { Data1: 0xD65231B0, Data2: 0xB2F1, Data3: 0x4857, Data4: [0xA4, 0xCE, 0xA8, 0xE7, 0xC6, 0xEA, 0x7D, 0x27]};
pub const FOLDERID_Windows: GUID = GUID { Data1: 0xF38BF404, Data2: 0x1D43, Data3: 0x42F2, Data4: [0x93, 0x05, 0x67, 0xDE, 0x0B, 0x28, 0xFC, 0x23]};
pub const FOLDERID_Profile: GUID = GUID { Data1: 0x5E6C858F, Data2: 0x0E22, Data3: 0x4760, Data4: [0x9A, 0xFE, 0xEA, 0x33, 0x17, 0xB6, 0x71, 0x73]};
pub const FOLDERID_Pictures: GUID = GUID { Data1: 0x33E28130, Data2: 0x4E1E, Data3: 0x4676, Data4: [0x83, 0x5A, 0x98, 0x39, 0x5C, 0x3B, 0xC3, 0xBB]};
pub const FOLDERID_ProgramFilesX86: GUID = GUID { Data1: 0x7C5A40EF, Data2: 0xA0FB, Data3: 0x4BFC, Data4: [0x87, 0x4A, 0xC0, 0xF2, 0xE0, 0xB9, 0xFA, 0x8E]};
pub const FOLDERID_ProgramFilesCommonX86: GUID = GUID { Data1: 0xDE974D24, Data2: 0xD9C6, Data3: 0x4D3E, Data4: [0xBF, 0x91, 0xF4, 0x45, 0x51, 0x20, 0xB9, 0x17]};
pub const FOLDERID_ProgramFilesX64: GUID = GUID { Data1: 0x6d809377, Data2: 0x6af0, Data3: 0x444b, Data4: [0x89, 0x57, 0xa3, 0x77, 0x3f, 0x02, 0x20, 0x0e ]};
pub const FOLDERID_ProgramFilesCommonX64: GUID = GUID { Data1: 0x6365d5a7, Data2: 0xf0d, Data3: 0x45e5, Data4: [0x87, 0xf6, 0xd, 0xa5, 0x6b, 0x6a, 0x4f, 0x7d ]};
pub const FOLDERID_ProgramFiles: GUID = GUID { Data1: 0x905e63b6, Data2: 0xc1bf, Data3: 0x494e, Data4: [0xb2, 0x9c, 0x65, 0xb7, 0x32, 0xd3, 0xd2, 0x1a]};
pub const FOLDERID_ProgramFilesCommon: GUID = GUID { Data1: 0xF7F1ED05, Data2: 0x9F6D, Data3: 0x47A2, Data4: [0xAA, 0xAE, 0x29, 0xD3, 0x17, 0xC6, 0xF0, 0x66]};
pub const FOLDERID_UserProgramFiles: GUID = GUID { Data1: 0x5cd7aee2, Data2: 0x2219, Data3: 0x4a67, Data4: [0xb8, 0x5d, 0x6c, 0x9c, 0xe1, 0x56, 0x60, 0xcb]};
pub const FOLDERID_UserProgramFilesCommon: GUID = GUID { Data1: 0xbcbd3057, Data2: 0xca5c, Data3: 0x4622, Data4: [0xb4, 0x2d, 0xbc, 0x56, 0xdb, 0x0a, 0xe5, 0x16]};
pub const FOLDERID_AdminTools: GUID = GUID { Data1: 0x724EF170, Data2: 0xA42D, Data3: 0x4FEF, Data4: [0x9F, 0x26, 0xB6, 0x0E, 0x84, 0x6F, 0xBA, 0x4F]};
pub const FOLDERID_CommonAdminTools: GUID = GUID { Data1: 0xD0384E7D, Data2: 0xBAC3, Data3: 0x4797, Data4: [0x8F, 0x14, 0xCB, 0xA2, 0x29, 0xB3, 0x92, 0xB5]};
pub const FOLDERID_Music: GUID = GUID { Data1: 0x4BD8D571, Data2: 0x6D19, Data3: 0x48D3, Data4: [0xBE, 0x97, 0x42, 0x22, 0x20, 0x08, 0x0E, 0x43]};
pub const FOLDERID_Videos: GUID = GUID { Data1: 0x18989B1D, Data2: 0x99B5, Data3: 0x455B, Data4: [0x84, 0x1C, 0xAB, 0x7C, 0x74, 0xE4, 0xDD, 0xFC]};
pub const FOLDERID_Ringtones: GUID = GUID { Data1: 0xC870044B, Data2: 0xF49E, Data3: 0x4126, Data4: [0xA9, 0xC3, 0xB5, 0x2A, 0x1F, 0xF4, 0x11, 0xE8]};
pub const FOLDERID_PublicPictures: GUID = GUID { Data1: 0xB6EBFB86, Data2: 0x6907, Data3: 0x413C, Data4: [0x9A, 0xF7, 0x4F, 0xC2, 0xAB, 0xF0, 0x7C, 0xC5]};
pub const FOLDERID_PublicMusic: GUID = GUID { Data1: 0x3214FAB5, Data2: 0x9757, Data3: 0x4298, Data4: [0xBB, 0x61, 0x92, 0xA9, 0xDE, 0xAA, 0x44, 0xFF]};
pub const FOLDERID_PublicVideos: GUID = GUID { Data1: 0x2400183A, Data2: 0x6185, Data3: 0x49FB, Data4: [0xA2, 0xD8, 0x4A, 0x39, 0x2A, 0x60, 0x2B, 0xA3]};
pub const FOLDERID_PublicRingtones: GUID = GUID { Data1: 0xE555AB60, Data2: 0x153B, Data3: 0x4D17, Data4: [0x9F, 0x04, 0xA5, 0xFE, 0x99, 0xFC, 0x15, 0xEC]};
pub const FOLDERID_ResourceDir: GUID = GUID { Data1: 0x8AD10C31, Data2: 0x2ADB, Data3: 0x4296, Data4: [0xA8, 0xF7, 0xE4, 0x70, 0x12, 0x32, 0xC9, 0x72]};
pub const FOLDERID_LocalizedResourcesDir: GUID = GUID { Data1: 0x2A00375E, Data2: 0x224C, Data3: 0x49DE, Data4: [0xB8, 0xD1, 0x44, 0x0D, 0xF7, 0xEF, 0x3D, 0xDC]};
pub const FOLDERID_CommonOEMLinks: GUID = GUID { Data1: 0xC1BAE2D0, Data2: 0x10DF, Data3: 0x4334, Data4: [0xBE, 0xDD, 0x7A, 0xA2, 0x0B, 0x22, 0x7A, 0x9D]};
pub const FOLDERID_CDBurning: GUID = GUID { Data1: 0x9E52AB10, Data2: 0xF80D, Data3: 0x49DF, Data4: [0xAC, 0xB8, 0x43, 0x30, 0xF5, 0x68, 0x78, 0x55]};
pub const FOLDERID_UserProfiles: GUID = GUID { Data1: 0x0762D272, Data2: 0xC50A, Data3: 0x4BB0, Data4: [0xA3, 0x82, 0x69, 0x7D, 0xCD, 0x72, 0x9B, 0x80]};
pub const FOLDERID_Playlists: GUID = GUID { Data1: 0xDE92C1C7, Data2: 0x837F, Data3: 0x4F69, Data4: [0xA3, 0xBB, 0x86, 0xE6, 0x31, 0x20, 0x4A, 0x23]};
pub const FOLDERID_SamplePlaylists: GUID = GUID { Data1: 0x15CA69B3, Data2: 0x30EE, Data3: 0x49C1, Data4: [0xAC, 0xE1, 0x6B, 0x5E, 0xC3, 0x72, 0xAF, 0xB5]};
pub const FOLDERID_SampleMusic: GUID = GUID { Data1: 0xB250C668, Data2: 0xF57D, Data3: 0x4EE1, Data4: [0xA6, 0x3C, 0x29, 0x0E, 0xE7, 0xD1, 0xAA, 0x1F]};
pub const FOLDERID_SamplePictures: GUID = GUID { Data1: 0xC4900540, Data2: 0x2379, Data3: 0x4C75, Data4: [0x84, 0x4B, 0x64, 0xE6, 0xFA, 0xF8, 0x71, 0x6B]};
pub const FOLDERID_SampleVideos: GUID = GUID { Data1: 0x859EAD94, Data2: 0x2E85, Data3: 0x48AD, Data4: [0xA7, 0x1A, 0x09, 0x69, 0xCB, 0x56, 0xA6, 0xCD]};
pub const FOLDERID_PhotoAlbums: GUID = GUID { Data1: 0x69D2CF90, Data2: 0xFC33, Data3: 0x4FB7, Data4: [0x9A, 0x0C, 0xEB, 0xB0, 0xF0, 0xFC, 0xB4, 0x3C]};
pub const FOLDERID_Public: GUID = GUID { Data1: 0xDFDF76A2, Data2: 0xC82A, Data3: 0x4D63, Data4: [0x90, 0x6A, 0x56, 0x44, 0xAC, 0x45, 0x73, 0x85]};
pub const FOLDERID_ChangeRemovePrograms: GUID = GUID { Data1: 0xdf7266ac, Data2: 0x9274, Data3: 0x4867, Data4: [0x8d, 0x55, 0x3b, 0xd6, 0x61, 0xde, 0x87, 0x2d]};
pub const FOLDERID_AppUpdates: GUID = GUID { Data1: 0xa305ce99, Data2: 0xf527, Data3: 0x492b, Data4: [0x8b, 0x1a, 0x7e, 0x76, 0xfa, 0x98, 0xd6, 0xe4]};
pub const FOLDERID_AddNewPrograms: GUID = GUID { Data1: 0xde61d971, Data2: 0x5ebc, Data3: 0x4f02, Data4: [0xa3, 0xa9, 0x6c, 0x82, 0x89, 0x5e, 0x5c, 0x04]};
pub const FOLDERID_Downloads: GUID = GUID { Data1: 0x374de290, Data2: 0x123f, Data3: 0x4565, Data4: [0x91, 0x64, 0x39, 0xc4, 0x92, 0x5e, 0x46, 0x7b]};
pub const FOLDERID_PublicDownloads: GUID = GUID { Data1: 0x3d644c9b, Data2: 0x1fb8, Data3: 0x4f30, Data4: [0x9b, 0x45, 0xf6, 0x70, 0x23, 0x5f, 0x79, 0xc0]};
pub const FOLDERID_SavedSearches: GUID = GUID { Data1: 0x7d1d3a04, Data2: 0xdebb, Data3: 0x4115, Data4: [0x95, 0xcf, 0x2f, 0x29, 0xda, 0x29, 0x20, 0xda]};
pub const FOLDERID_QuickLaunch: GUID = GUID { Data1: 0x52a4f021, Data2: 0x7b75, Data3: 0x48a9, Data4: [0x9f, 0x6b, 0x4b, 0x87, 0xa2, 0x10, 0xbc, 0x8f]};
pub const FOLDERID_Contacts: GUID = GUID { Data1: 0x56784854, Data2: 0xc6cb, Data3: 0x462b, Data4: [0x81, 0x69, 0x88, 0xe3, 0x50, 0xac, 0xb8, 0x82]};
pub const FOLDERID_SidebarParts: GUID = GUID { Data1: 0xa75d362e, Data2: 0x50fc, Data3: 0x4fb7, Data4: [0xac, 0x2c, 0xa8, 0xbe, 0xaa, 0x31, 0x44, 0x93]};
pub const FOLDERID_SidebarDefaultParts: GUID = GUID { Data1: 0x7b396e54, Data2: 0x9ec5, Data3: 0x4300, Data4: [0xbe, 0xa, 0x24, 0x82, 0xeb, 0xae, 0x1a, 0x26]};
pub const FOLDERID_PublicGameTasks: GUID = GUID { Data1: 0xdebf2536, Data2: 0xe1a8, Data3: 0x4c59, Data4: [0xb6, 0xa2, 0x41, 0x45, 0x86, 0x47, 0x6a, 0xea]};
pub const FOLDERID_GameTasks: GUID = GUID { Data1: 0x54fae61, Data2: 0x4dd8, Data3: 0x4787, Data4: [0x80, 0xb6, 0x9, 0x2, 0x20, 0xc4, 0xb7, 0x0]};
pub const FOLDERID_SavedGames: GUID = GUID { Data1: 0x4c5c32ff, Data2: 0xbb9d, Data3: 0x43b0, Data4: [0xb5, 0xb4, 0x2d, 0x72, 0xe5, 0x4e, 0xaa, 0xa4]};
pub const FOLDERID_Games: GUID = GUID { Data1: 0xcac52c1a, Data2: 0xb53d, Data3: 0x4edc, Data4: [0x92, 0xd7, 0x6b, 0x2e, 0x8a, 0xc1, 0x94, 0x34]};
pub const FOLDERID_SEARCH_MAPI: GUID = GUID { Data1: 0x98ec0e18, Data2: 0x2098, Data3: 0x4d44, Data4: [0x86, 0x44, 0x66, 0x97, 0x93, 0x15, 0xa2, 0x81]};
pub const FOLDERID_SEARCH_CSC: GUID = GUID { Data1: 0xee32e446, Data2: 0x31ca, Data3: 0x4aba, Data4: [0x81, 0x4f, 0xa5, 0xeb, 0xd2, 0xfd, 0x6d, 0x5e]};
pub const FOLDERID_Links: GUID = GUID { Data1: 0xbfb9d5e0, Data2: 0xc6a9, Data3: 0x404c, Data4: [0xb2, 0xb2, 0xae, 0x6d, 0xb6, 0xaf, 0x49, 0x68]};
pub const FOLDERID_UsersFiles: GUID = GUID { Data1: 0xf3ce0f7c, Data2: 0x4901, Data3: 0x4acc, Data4: [0x86, 0x48, 0xd5, 0xd4, 0x4b, 0x04, 0xef, 0x8f]};
pub const FOLDERID_UsersLibraries: GUID = GUID { Data1: 0xa302545d, Data2: 0xdeff, Data3: 0x464b, Data4: [0xab, 0xe8, 0x61, 0xc8, 0x64, 0x8d, 0x93, 0x9b]};
pub const FOLDERID_SearchHome: GUID = GUID { Data1: 0x190337d1, Data2: 0xb8ca, Data3: 0x4121, Data4: [0xa6, 0x39, 0x6d, 0x47, 0x2d, 0x16, 0x97, 0x2a]};
pub const FOLDERID_OriginalImages: GUID = GUID { Data1: 0x2C36C0AA, Data2: 0x5812, Data3: 0x4b87, Data4: [0xbf, 0xd0, 0x4c, 0xd0, 0xdf, 0xb1, 0x9b, 0x39]};
pub const FOLDERID_DocumentsLibrary: GUID = GUID { Data1: 0x7b0db17d, Data2: 0x9cd2, Data3: 0x4a93, Data4: [0x97, 0x33, 0x46, 0xcc, 0x89, 0x02, 0x2e, 0x7c]};
pub const FOLDERID_MusicLibrary: GUID = GUID { Data1: 0x2112ab0a, Data2: 0xc86a, Data3: 0x4ffe, Data4: [0xa3, 0x68, 0xd, 0xe9, 0x6e, 0x47, 0x1, 0x2e]};
pub const FOLDERID_PicturesLibrary: GUID = GUID { Data1: 0xa990ae9f, Data2: 0xa03b, Data3: 0x4e80, Data4: [0x94, 0xbc, 0x99, 0x12, 0xd7, 0x50, 0x41, 0x4]};
pub const FOLDERID_VideosLibrary: GUID = GUID { Data1: 0x491e922f, Data2: 0x5643, Data3: 0x4af4, Data4: [0xa7, 0xeb, 0x4e, 0x7a, 0x13, 0x8d, 0x81, 0x74]};
pub const FOLDERID_RecordedTVLibrary: GUID = GUID { Data1: 0x1a6fdba2, Data2: 0xf42d, Data3: 0x4358, Data4: [0xa7, 0x98, 0xb7, 0x4d, 0x74, 0x59, 0x26, 0xc5]};
pub const FOLDERID_HomeGroup: GUID = GUID { Data1: 0x52528a6b, Data2: 0xb9e3, Data3: 0x4add, Data4: [0xb6, 0xd, 0x58, 0x8c, 0x2d, 0xba, 0x84, 0x2d]};
pub const FOLDERID_HomeGroupCurrentUser: GUID = GUID { Data1: 0x9b74b6a3, Data2: 0xdfd, Data3: 0x4f11, Data4: [0x9e, 0x78, 0x5f, 0x78, 0x0, 0xf2, 0xe7, 0x72]};
pub const FOLDERID_DeviceMetadataStore: GUID = GUID { Data1: 0x5ce4a5e9, Data2: 0xe4eb, Data3: 0x479d, Data4: [0xb8, 0x9f, 0x13, 0x0c, 0x02, 0x88, 0x61, 0x55]};
pub const FOLDERID_Libraries: GUID = GUID { Data1: 0x1b3ea5dc, Data2: 0xb587, Data3: 0x4786, Data4: [0xb4, 0xef, 0xbd, 0x1d, 0xc3, 0x32, 0xae, 0xae]};
pub const FOLDERID_PublicLibraries: GUID = GUID { Data1: 0x48daf80b, Data2: 0xe6cf, Data3: 0x4f4e, Data4: [0xb8, 0x00, 0x0e, 0x69, 0xd8, 0x4e, 0xe3, 0x84]};
pub const FOLDERID_UserPinned: GUID = GUID { Data1: 0x9e3995ab, Data2: 0x1f9c, Data3: 0x4f13, Data4: [0xb8, 0x27, 0x48, 0xb2, 0x4b, 0x6c, 0x71, 0x74]};
pub const FOLDERID_ImplicitAppShortcuts: GUID = GUID { Data1: 0xbcb5256f, Data2: 0x79f6, Data3: 0x4cee, Data4: [0xb7, 0x25, 0xdc, 0x34, 0xe4, 0x2, 0xfd, 0x46]};
pub const FOLDERID_AccountPictures: GUID = GUID { Data1: 0x008ca0b1, Data2: 0x55b4, Data3: 0x4c56, Data4: [0xb8, 0xa8, 0x4d, 0xe4, 0xb2, 0x99, 0xd3, 0xbe]};
pub const FOLDERID_PublicUserTiles: GUID = GUID { Data1: 0x0482af6c, Data2: 0x08f1, Data3: 0x4c34, Data4: [0x8c, 0x90, 0xe1, 0x7e, 0xc9, 0x8b, 0x1e, 0x17]};
pub const FOLDERID_AppsFolder: GUID = GUID { Data1: 0x1e87508d, Data2: 0x89c2, Data3: 0x42f0, Data4: [0x8a, 0x7e, 0x64, 0x5a, 0x0f, 0x50, 0xca, 0x58]};
pub const FOLDERID_ApplicationShortcuts: GUID = GUID { Data1: 0xa3918781, Data2: 0xe5f2, Data3: 0x4890, Data4: [0xb3, 0xd9, 0xa7, 0xe5, 0x43, 0x32, 0x32, 0x8c]};
pub const FOLDERID_RoamingTiles: GUID = GUID { Data1: 0xbcfc5a, Data2: 0xed94, Data3: 0x4e48, Data4: [0x96, 0xa1, 0x3f, 0x62, 0x17, 0xf2, 0x19, 0x90]};
pub const FOLDERID_RoamedTileImages: GUID = GUID { Data1: 0xaaa8d5a5, Data2: 0xf1d6, Data3: 0x4259, Data4: [0xba, 0xa8, 0x78, 0xe7, 0xef, 0x60, 0x83, 0x5e]};
pub const FOLDERID_Screenshots: GUID = GUID { Data1: 0xb7bede81, Data2: 0xdf94, Data3: 0x4682, Data4: [0xa7, 0xd8, 0x57, 0xa5, 0x26, 0x20, 0xb8, 0x6f]};
pub const FOLDERID_CameraRoll: GUID = GUID { Data1: 0xab5fb87b, Data2: 0x7ce2, Data3: 0x4f83, Data4: [0x91, 0x5d, 0x55, 0x8, 0x46, 0xc9, 0x53, 0x7b]};
pub const FOLDERID_SkyDrive: GUID = GUID { Data1: 0xa52bba46, Data2: 0xe9e1, Data3: 0x435f, Data4: [0xb3, 0xd9, 0x28, 0xda, 0xa6, 0x48, 0xc0, 0xf6]};
pub const FOLDERID_SkyDriveDocuments: GUID = GUID { Data1: 0x24d89e24, Data2: 0x2f19, Data3: 0x4534, Data4: [0x9d, 0xde, 0x6a, 0x66, 0x71, 0xfb, 0xb8, 0xfe]};
pub const FOLDERID_SkyDrivePictures: GUID = GUID { Data1: 0x339719b5, Data2: 0x8c47, Data3: 0x4894, Data4: [0x94, 0xc2, 0xd8, 0xf7, 0x7a, 0xdd, 0x44, 0xa6]};
pub const FOLDERID_SkyDriveMusic: GUID = GUID { Data1: 0xc3f2459e, Data2: 0x80d6, Data3: 0x45dc, Data4: [0xbf, 0xef, 0x1f, 0x76, 0x9f, 0x2b, 0xe7, 0x30]};
pub const FOLDERID_SkyDriveCameraRoll: GUID = GUID { Data1: 0x767e6811, Data2: 0x49cb, Data3: 0x4273, Data4: [0x87, 0xc2, 0x20, 0xf3, 0x55, 0xe1, 0x08, 0x5b]};
pub const FOLDERID_SearchHistory: GUID = GUID { Data1: 0x0d4c3db6, Data2: 0x03a3, Data3: 0x462f, Data4: [0xa0, 0xe6, 0x08, 0x92, 0x4c, 0x41, 0xb5, 0xd4]};
pub const FOLDERID_SearchTemplates: GUID = GUID { Data1: 0x7e636bfe, Data2: 0xdfa9, Data3: 0x4d5e, Data4: [0xb4, 0x56, 0xd7, 0xb3, 0x98, 0x51, 0xd8, 0xa9]};
// constants
pub const INVALID_HANDLE_VALUE: HANDLE = -1 as HANDLE;
// error codes
pub const ERROR_INVALID_HANDLE: DWORD = 6;
pub const ERROR_ILLEGAL_CHARACTER: DWORD = 582;

// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! Types and constants for WinAPI bindings.
#![allow(bad_style, raw_pointer_derive)]
#![warn(unused_qualifications, unused)]
#![cfg(windows)]

//-------------------------------------------------------------------------------------------------
// External crates
//-------------------------------------------------------------------------------------------------
extern crate libc;
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
pub use basetsd::*;
pub use commctrl::*;
pub use d3d9::*;
pub use d3d9caps::*;
pub use d3d9types::*;
pub use dwmapi::*;
pub use fileapi::*;
pub use libloaderapi::*;
pub use minwinbase::*;
pub use minwindef::*;
pub use processthreadsapi::*;
pub use synchapi::*;
pub use unknwnbase::*;
pub use vadefs::*;
pub use winbase::*;
pub use wincon::*;
pub use wincred::*;
pub use wincrypt::*;
pub use windowsx::*;
pub use winerror::*;
pub use wingdi::*;
pub use winioctl::*;
pub use winnls::*;
pub use winnt::*;
pub use winsvc::*;
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
macro_rules! MAKEFOURCC {
    ($a:expr, $b:expr, $c:expr, $d:expr) => {
        ($a as i32) | (($b as i32) << 8) | (($c as i32) << 16) | (($d as i32) << 24)
    }
}
macro_rules! DEFINE_GUID {
    (
        $name:ident, $l:expr, $w1:expr, $w2:expr, $b1:expr, $b2:expr, $b3:expr, $b4:expr, $b5:expr,
        $b6:expr, $b7:expr, $b8:expr
    ) => {
        pub const $name: ::GUID = ::GUID {
            Data1: $l,
            Data2: $w1,
            Data3: $w2,
            Data4: [$b1, $b2, $b3, $b4, $b5, $b6, $b7, $b8],
        };
    }
}
macro_rules! CTL_CODE {
    ($DeviceType:expr, $Function:expr, $Method:expr, $Access:expr) => {
        ($DeviceType << 16) | ($Access << 14) | ($Function << 2) | $Method
    }
}
macro_rules! RIDL {
    (interface $interface:ident ($vtbl:ident)
        {$(
            fn $method:ident(&mut self $(,$p:ident : $t:ty)*) -> $rtr:ty
        ),+}
    ) => {
        #[repr(C)]
        pub struct $vtbl {
            $(pub $method: unsafe extern "system" fn(
                This: *mut $interface
                $(,$p: $t)*
            ) -> $rtr),+
        }
        #[repr(C)] #[derive(Debug)]
        pub struct $interface {
            pub lpVtbl: *const $vtbl
        }
        impl $interface {
            $(pub unsafe fn $method(&mut self $(,$p: $t)*) -> $rtr {
                ((*self.lpVtbl).$method)(self $(,$p)*)
            })+
        }
    };
    (interface $interface:ident ($vtbl:ident) : $pinterface:ident ($pvtbl:ident)
        {$(
            fn $method:ident(&mut self $(,$p:ident : $t:ty)*) -> $rtr:ty
        ),+}
    ) => {
        #[repr(C)]
        pub struct $vtbl {
            pub parent: $pvtbl
            $(,pub $method: unsafe extern "system" fn(
                This: *mut $interface
                $(,$p: $t)*
            ) -> $rtr)+
        }
        #[repr(C)] #[derive(Debug)]
        pub struct $interface {
            pub lpVtbl: *const $vtbl
        }
        impl $interface {
            $(pub unsafe fn $method(&mut self $(,$p: $t)*) -> $rtr {
                ((*self.lpVtbl).$method)(self $(,$p)*)
            })+
        }
        impl ::std::ops::Deref for $interface {
            type Target = $pinterface;
            fn deref(&self) -> &$pinterface {
                unsafe { ::std::mem::transmute(self) }
            }
        }
        impl ::std::ops::DerefMut for $interface {
            fn deref_mut(&mut self) -> &mut $pinterface {
                unsafe { ::std::mem::transmute(self) }
            }
        }
    };
}

//-------------------------------------------------------------------------------------------------
// Modules
//-------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------
pub mod audioclient;
pub mod basetsd;
pub mod commctrl;
pub mod d3d9;
pub mod d3d9caps;
pub mod d3d9types;
pub mod dwmapi;
pub mod fileapi;
pub mod libloaderapi;
pub mod minwinbase;
pub mod minwindef;
pub mod processthreadsapi;
pub mod synchapi;
pub mod unknwnbase;
pub mod vadefs;
pub mod winbase;
pub mod wincon;
pub mod wincred;
pub mod wincrypt;
pub mod windowsx;
pub mod winerror;
pub mod wingdi;
pub mod winioctl;
pub mod winnls;
pub mod winnt;
pub mod winsvc;
pub mod winuser;
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

//lazy
pub type SCODE = LONG;

//-------------------------------------------------------------------------------------------------
// windef.h
// Basic Windows Type Definitions
//-------------------------------------------------------------------------------------------------
#[repr(C)] #[derive(Clone, Copy, Debug)]
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
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct PROCESSOR_NUMBER {
    pub Group: WORD,
    pub Number: BYTE,
    pub Reserved: BYTE,
}
pub type PPROCESSOR_NUMBER = *mut PROCESSOR_NUMBER;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct GROUP_AFFINITY {
    pub Mask: KAFFINITY,
    pub Group: WORD,
    pub Reserved: [WORD; 3],
}
pub type HDWP = *mut HANDLE;
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
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub enum COMPARTMENT_ID {
    UNSPECIFIED_COMPARTMENT_ID = 0,
    DEFAULT_COMPARTMENT_ID = 1,
}
pub type PCOMPARTMENT_ID = *mut COMPARTMENT_ID;
#[repr(C)] #[derive(Clone, Copy, Debug)]
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
pub type LARGE_INTEGER = LONGLONG;
pub type PLARGE_INTEGER = *mut LARGE_INTEGER;
pub type ULARGE_INTEGER = ULONGLONG;
pub type PULARGE_INTEGER= *mut ULARGE_INTEGER;
pub type RTL_REFERENCE_COUNT = LONG_PTR;
pub type PRTL_REFERENCE_COUNT = *mut LONG_PTR;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct LUID {
    pub LowPart: DWORD,
    pub HighPart: LONG,
}
pub type PLUID = *mut LUID;
pub type DWORDLONG = ULONGLONG;
pub type PDWORDLONG = *mut DWORDLONG;
pub type BOOLEAN = BYTE;
pub type PBOOLEAN = *mut BOOLEAN;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct LIST_ENTRY {
    pub Flink: *mut LIST_ENTRY,
    pub Blink: *mut LIST_ENTRY,
}
pub type PLIST_ENTRY = *mut LIST_ENTRY;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SINGLE_LIST_ENTRY {
    pub Next: *mut SINGLE_LIST_ENTRY,
}
pub type PSINGLE_LIST_ENTRY = *mut SINGLE_LIST_ENTRY;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct LIST_ENTRY32 {
    pub Flink: DWORD,
    pub Blink: DWORD,
}
pub type PLIST_ENTRY32 = *mut LIST_ENTRY32;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct LIST_ENTRY64 {
    pub Flink: ULONGLONG,
    pub Blink: ULONGLONG,
}
pub type PLIST_ENTRY64 = *mut LIST_ENTRY64;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct OBJECTID {
    pub Lineage: GUID,
    pub Uniquifier: DWORD,
}
pub type PEXCEPTION_ROUTINE = Option<unsafe extern "system" fn(
    ExceptionRecord: *mut EXCEPTION_RECORD, EstablisherFrame: PVOID, ContextRecord: *mut CONTEXT,
    DispatcherContext: PVOID,
) -> EXCEPTION_DISPOSITION>;
pub type KSPIN_LOCK = ULONG_PTR;
pub type PKSPIN_LOCK = *mut KSPIN_LOCK;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct M128A { // FIXME align 16
    pub Low: ULONGLONG,
    pub High: LONGLONG,
}
pub type PM128A = *mut M128A;
#[cfg(target_arch = "x86")] #[repr(C)] #[derive(Copy)]
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
#[cfg(target_arch = "x86_64")] #[repr(C)] #[derive(Copy)]
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
impl Clone for XSAVE_FORMAT { fn clone(&self) -> XSAVE_FORMAT { *self } }
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct TOKEN_PRIVILEGES {
    pub PrivilegeCount: DWORD,
    pub Privileges: [LUID_AND_ATTRIBUTES; 0],
}
pub type PTOKEN_PRIVILEGES = *mut TOKEN_PRIVILEGES;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct LUID_AND_ATTRIBUTES {
    pub Luid: LUID,
    pub Attributes: DWORD,
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
#[repr(C)] #[derive(Clone, Copy, Debug)]
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
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub enum EXCEPTION_DISPOSITION {
    ExceptionContinueExecution = 0,
    ExceptionContinueSearch = 1,
    ExceptionNestedException = 2,
    ExceptionCollidedUnwind = 3,
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct _EXCEPTION_RECORD;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct _CONTEXT;
#[cfg(target_arch = "x86_64")] #[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct _DISPATCHER_CONTEXT;
// shlobj.h
pub type GPFIDL_FLAGS = c_int;
#[repr(i32)] #[derive(Clone, Copy, Debug)]
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
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HWND__;
pub type HWND = *mut HWND__;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HHOOK__;
pub type HHOOK = *mut HHOOK__;
pub type HGDIOBJ = *mut c_void;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HACCEL__;
pub type HACCEL = *mut HACCEL__;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HBITMAP__;
pub type HBITMAP = *mut HBITMAP__;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HBRUSH__;
pub type HBRUSH = *mut HBRUSH__;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HCOLORSPACE__;
pub type HCOLORSPACE = *mut HCOLORSPACE__;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HDC__;
pub type HDC = *mut HDC__;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HGLRC__;
pub type HGLRC = *mut HGLRC__;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HDESK__;
pub type HDESK = *mut HDESK__;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HENHMETAFILE__;
pub type HENHMETAFILE = *mut HENHMETAFILE__;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HFONT__;
pub type HFONT = *mut HFONT__;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HICON__;
pub type HICON = *mut HICON__;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HMENU__;
pub type HMENU = *mut HMENU__;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HPALETTE__;
pub type HPALETTE = *mut HPALETTE__;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HPEN__;
pub type HPEN = *mut HPEN__;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HWINEVENTHOOK__;
pub type HWINEVENTHOOK = *mut HWINEVENTHOOK__;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HMONITOR__;
pub type HMONITOR = *mut HMONITOR__;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HUMPD__;
pub type HUMPD = *mut HUMPD__;
pub type HCURSOR = HICON;
pub type COLORREF = DWORD;
pub type LPCOLORREF = *mut DWORD;
#[repr(C)] #[derive(Clone, Copy, Debug)]
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
#[repr(C)] #[derive(Clone, Copy, Debug)]
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

pub const DEBUG_PROCESS: DWORD = 0x00000001;
pub const DEBUG_ONLY_THIS_PROCESS: DWORD = 0x00000002;
pub const CREATE_SUSPENDED: DWORD = 0x00000004;
pub const DETACHED_PROCESS: DWORD = 0x00000008;

pub const CREATE_NEW_CONSOLE: DWORD = 0x00000010;
pub const NORMAL_PRIORITY_CLASS: DWORD = 0x00000020;
pub const IDLE_PRIORITY_CLASS: DWORD = 0x00000040;
pub const HIGH_PRIORITY_CLASS: DWORD = 0x00000080;

pub const REALTIME_PRIORITY_CLASS: DWORD = 0x00000100;
pub const CREATE_NEW_PROCESS_GROUP: DWORD = 0x00000200;
pub const CREATE_UNICODE_ENVIRONMENT: DWORD = 0x00000400;
pub const CREATE_SEPARATE_WOW_VDM: DWORD = 0x00000800;

pub const CREATE_SHARED_WOW_VDM: DWORD = 0x00001000;
pub const CREATE_FORCEDOS: DWORD = 0x00002000;
pub const BELOW_NORMAL_PRIORITY_CLASS: DWORD = 0x00004000;
pub const ABOVE_NORMAL_PRIORITY_CLASS: DWORD = 0x00008000;

pub const INHERIT_PARENT_AFFINITY: DWORD = 0x00010000;
pub const INHERIT_CALLER_PRIORITY: DWORD = 0x00020000;
pub const CREATE_PROTECTED_PROCESS: DWORD = 0x00040000;
pub const EXTENDED_STARTUPINFO_PRESENT: DWORD = 0x00080000;

pub const PROCESS_MODE_BACKGROUND_BEGIN: DWORD = 0x00100000;
pub const PROCESS_MODE_BACKGROUND_END: DWORD = 0x00200000;

pub const CREATE_BREAKAWAY_FROM_JOB: DWORD = 0x01000000;
pub const CREATE_PRESERVE_CODE_AUTHZ_LEVEL: DWORD = 0x02000000;
pub const CREATE_DEFAULT_ERROR_MODE: DWORD = 0x04000000;
pub const CREATE_NO_WINDOW: DWORD = 0x08000000;

pub const PROFILE_USER: DWORD = 0x10000000;
pub const PROFILE_KERNEL: DWORD = 0x20000000;
pub const PROFILE_SERVER: DWORD = 0x40000000;
pub const CREATE_IGNORE_SYSTEM_DEFAULT: DWORD = 0x80000000;

pub const THREAD_BASE_PRIORITY_LOWRT: DWORD = 15;
pub const THREAD_BASE_PRIORITY_MAX: DWORD = 2;
pub const THREAD_BASE_PRIORITY_MIN: DWORD = (-2 as i32) as DWORD;
pub const THREAD_BASE_PRIORITY_IDLE: DWORD = (-15 as i32) as DWORD;

pub const THREAD_PRIORITY_LOWEST: DWORD = THREAD_BASE_PRIORITY_MIN;
pub const THREAD_PRIORITY_BELOW_NORMAL: DWORD = THREAD_PRIORITY_LOWEST + 1;
pub const THREAD_PRIORITY_NORMAL: DWORD = 0;
pub const THREAD_PRIORITY_HIGHEST: DWORD = THREAD_BASE_PRIORITY_MAX;
pub const THREAD_PRIORITY_ABOVE_NORMAL: DWORD = THREAD_PRIORITY_HIGHEST - 1;
pub const THREAD_PRIORITY_ERROR_RETURN: DWORD = 0x7fffffff;

pub const THREAD_PRIORITY_TIME_CRITICAL: DWORD = THREAD_BASE_PRIORITY_LOWRT;
pub const THREAD_PRIORITY_IDLE: DWORD = THREAD_BASE_PRIORITY_IDLE;

pub const THREAD_MODE_BACKGROUND_BEGIN: DWORD = 0x00010000;
pub const THREAD_MODE_BACKGROUND_END: DWORD = 0x00020000;

// shlobjdl.h
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IContextMenu;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IContextMenu2;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IContextMenu3;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IExecuteCommand;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IPersistFolder;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IRunnableTask;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IShellTaskScheduler;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IQueryCodePage;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IPersistFolder2;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IPersistFolder3;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IPersistIDList;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IEnumIDList;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IEnumFullIDList;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IFileSyncMergeHandler;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IObjectWithFolderEnumMode;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IParseAndCreateItem;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IShellFolder;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IEnumExtraSearch;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IShellFolder2;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IFolderViewOptions;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IShellView;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IShellView2;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IShellView3;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IFolderView;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ISearchBoxInfo;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IFolderView2;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IFolderViewSettings;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IPreviewHandlerVisuals;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IVisualProperties;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ICommDlgBrowser;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ICommDlgBrowser2;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ICommDlgBrowser3;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IColumnManager;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IFolderFilterSite;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IFolderFilter;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IInputObjectSite;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IInputObject;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IInputObject2;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IShellIcon;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IShellBrowser;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IProfferService;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IShellItem;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IShellItem2;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IShellItemImageFactory;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IUserAccountChangeCallback;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IEnumShellItems;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ITransferAdviseSink;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ITransferSource;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IEnumResources;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IShellItemResources;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ITransferDestination;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IStreamAsync;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IStreamUnbufferedInfo;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IFileOperationProgressSink;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IShellItemArray;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IInitializeWithItem;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IObjectWithSelection;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IObjectWithBackReferences;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IPropertyUI;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ICategoryProvider;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ICategorizer;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IDropTargetHelper;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IDragSourceHelper;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IDragSourceHelper2;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IShellLinkA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IShellLinkW;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IShellLinkDataList;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IResolveShellLink;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IActionProgressDialog;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IHWEventHandler;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IHWEventHandler2;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IQueryCancelAutoPlay;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IDynamicHWHandler;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IActionProgress;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IShellExtInit;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IShellPropSheetExt;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IRemoteComputer;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IQueryContinue;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IObjectWithCancelEvent;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IUserNotification;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IUserNotificationCallback;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IUserNotification2;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IItemNameLimits;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ISearchFolderItemFactory;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IExtractImage;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IExtractImage2;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IThumbnailHandlerFactory;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IParentAndItem;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IDockingWindow;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IDeskBand;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IDeskBandInfo;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IDeskBand2;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ITaskbarList;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ITaskbarList2;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ITaskbarList3;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ITaskbarList4;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IStartMenuPinnedList;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ICDBurn;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IWizardSite;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IWizardExtension;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IWebWizardExtension;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IPublishingWizard;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IFolderViewHost;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IExplorerBrowserEvents;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IExplorerBrowser;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IAccessibleObject;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IResultsFolder;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IEnumObjects;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IOperationsProgressDialog;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IIOCancelInformation;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IFileOperation;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IObjectProvider;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct INamespaceWalkCB;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct INamespaceWalkCB2;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct INamespaceWalk;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IAutoCompleteDropDown;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IBandSite;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IModalWindow;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ICDBurnExt;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IContextMenuSite;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IEnumReadyCallback;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IEnumerableView;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IInsertItem;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IMenuBand;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IFolderBandPriv;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IRegTreeItem;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IImageRecompress;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IDeskBar;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IMenuPopup;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IFileIsInUse;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IFileDialogEvents;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IFileDialog;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IFileSaveDialog;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IFileOpenDialog;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IFileDialogCustomize;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IFileDialogControlEvents;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IFileDialog2;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IApplicationAssociationRegistration;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IApplicationAssociationRegistrationUI;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IDelegateFolder;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IBrowserFrameOptions;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct INewWindowManager;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IAttachmentExecute;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IShellMenuCallback;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IShellMenu;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IShellRunDll;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IKnownFolder;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IKnownFolderManager;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ISharingConfigurationManager;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IPreviousVersionsInfo;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IRelatedItem;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IIdentityName;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IDelegateItem;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ICurrentItem;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ITransferMediumItem;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IUseToBrowseItem;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IDisplayItem;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IViewStateIdentityItem;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IPreviewItem;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IDestinationStreamFactory;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct INewMenuClient;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IInitializeWithBindCtx;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IShellItemFilter;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct INameSpaceTreeControl;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct INameSpaceTreeControl2;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct INameSpaceTreeControlEvents;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct INameSpaceTreeControlDropHandler;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct INameSpaceTreeAccessible;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct INameSpaceTreeControlCustomDraw;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct INameSpaceTreeControlFolderCapabilities;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IPreviewHandler;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IPreviewHandlerFrame;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ITrayDeskBand;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IBandHost;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IExplorerPaneVisibility;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IContextMenuCB;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IDefaultExtractIconInit;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IExplorerCommand;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IExplorerCommandState;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IInitializeCommand;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IEnumExplorerCommand;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IExplorerCommandProvider;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IMarkupCallback;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IControlMarkup;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IInitializeNetworkFolder;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IOpenControlPanel;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IComputerInfoChangeNotify;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IFileSystemBindData;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IFileSystemBindData2;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ICustomDestinationList;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IApplicationDestinations;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IApplicationDocumentLists;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IObjectWithAppUserModelID;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IObjectWithProgID;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IUpdateIDList;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IDesktopGadget;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IDesktopWallpaper;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IHomeGroup;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IInitializeWithPropertyStore;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IOpenSearchSource;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IShellLibrary;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IDefaultFolderMenuInitialize;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IApplicationActivationManager;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IAssocHandlerInvoker;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IAssocHandler;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IEnumAssocHandlers;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IDataObjectProvider;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IDataTransferManagerInterop;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IFrameworkInputPaneHandler;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IFrameworkInputPane;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IAccessibilityDockingServiceCallback;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IAccessibilityDockingService;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IAppVisibilityEvents;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IAppVisibility;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IPackageExecutionStateChangeNotification;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IPackageDebugSettings;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ISuspensionDependencyManager;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IExecuteCommandApplicationHostEnvironment;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IExecuteCommandHost;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IApplicationDesignModeSettings;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IApplicationDesignModeSettings2;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ILaunchTargetMonitor;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ILaunchSourceViewSizePreference;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ILaunchTargetViewSizePreference;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ILaunchSourceAppUserModelId;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IInitializeWithWindow;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IHandlerInfo;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IHandlerActivationHost;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IContactManagerInterop;
// shtypes.h
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SHITEMID {
    pub cb: USHORT,
    pub abID: [BYTE; 0],
}
pub type LPSHITEMID = *mut SHITEMID;
pub type LPCSHITEMID = *const SHITEMID;
#[repr(C)] #[derive(Clone, Copy, Debug)]
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
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub enum EDataFlow {
    eRender,
    eCapture,
    eAll,
    EDataFlow_enum_count,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
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
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IMMDevice {
    pub lpVtbl: *const IMMDeviceVtbl,
}
impl IMMDevice {
    /// Part of the IUnknown interface inherited by IMMDeviceEnumerator.
    ///
    /// https://msdn.microsoft.com/en-us/library/windows/desktop/ms682521.aspx
    pub unsafe fn QueryInterface (
        &mut self,
        riid: REFIID,
        ppvObject: *mut *mut c_void,
    ) -> HRESULT {
        ((&*self.lpVtbl).QueryInterface)(self, riid, ppvObject)
    }
    /// Part of the IUnknown interface inherited by IMMDeviceEnumerator.
    ///
    /// https://msdn.microsoft.com/en-us/library/windows/desktop/ms691379.aspx
    pub unsafe fn AddRef(&mut self) -> ULONG {
        ((&*self.lpVtbl).AddRef)(self)
    }
    /// Part of the IUnknown interface inherited by IMMDeviceEnumerator.
    ///
    /// https://msdn.microsoft.com/en-us/library/windows/desktop/ms682317.aspx
    pub unsafe fn Release(&mut self) -> ULONG {
        ((&*self.lpVtbl).Release)(self)
    }
    /// Calls the vtable's Active() member.
    pub unsafe fn Activate(
        &mut self,
        iid: REFIID,
        dwClsCtx: DWORD,
        pActivationParams: *mut PROPVARIANT,
        ppInterface: *mut LPVOID,
    ) -> HRESULT {
        ((&*self.lpVtbl).Activate)(self, iid, dwClsCtx, pActivationParams, ppInterface)
    }
    // pub OpenPropertyStore: unsafe extern "system" fn(
    //     This: *mut IMMDevice,
    //     stgmAccess: DWORD,
    //     ppProperties: *mut *mut IPropertyStore,
    // ) -> HRESULT,
    // pub GetId: unsafe extern "system" fn(
    //     This: *mut IMMDevice,
    //     ppstrId: *mut LPWSTR,
    // ) -> HRESULT,
    // pub GetState: unsafe extern "system" fn(
    //     This: *mut IMMDevice,
    //     pdwState: *mut DWORD,
    // ) -> HRESULT,
}

#[repr(C)]
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
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IMMDeviceEnumerator {
    pub lpVtbl: *const IMMDeviceEnumeratorVtbl,
}

impl IMMDeviceEnumerator {
    /// Part of the IUnknown interface inherited by IMMDeviceEnumerator.
    ///
    /// https://msdn.microsoft.com/en-us/library/windows/desktop/ms682521.aspx
    pub unsafe fn QueryInterface (
        &mut self,
        riid: REFIID,
        ppvObject: *mut *mut c_void,
    ) -> HRESULT {
        ((&*self.lpVtbl).QueryInterface)(self, riid, ppvObject)
    }

    /// Part of the IUnknown interface inherited by IMMDeviceEnumerator.
    ///
    /// https://msdn.microsoft.com/en-us/library/windows/desktop/ms691379.aspx
    pub unsafe fn AddRef(&mut self) -> ULONG {
        ((&*self.lpVtbl).AddRef)(self)
    }

    /// Part of the IUnknown interface inherited by IMMDeviceEnumerator.
    ///
    /// https://msdn.microsoft.com/en-us/library/windows/desktop/ms682317.aspx
    pub unsafe fn Release(&mut self) -> ULONG {
        ((&*self.lpVtbl).Release)(self)
    }

    /// Call the vtable's EnumAudioEndpoints() member.
    ///
    /// https://msdn.microsoft.com/en-us/library/windows/desktop/dd371400.aspx
    pub unsafe fn EnumAudioEndpoints(
        &mut self,
        dataFlow: EDataFlow,
        dwStateMask: DWORD,
        ppDevices: *mut *mut IMMDeviceCollection,
    ) -> HRESULT {
        ((&*self.lpVtbl).EnumAudioEndpoints)(self, dataFlow, dwStateMask, ppDevices)
    }

    /// Call the vtable's GetDefaultAudioEndpoint() member.
    ///
    /// https://msdn.microsoft.com/en-us/library/windows/desktop/dd371401.aspx
    pub unsafe fn GetDefaultAudioEndpoint(
        &mut self,
        dataFlow: EDataFlow,
        role: ERole,
        ppEndpoint: *mut *mut IMMDevice,
    ) -> HRESULT {
        ((&*self.lpVtbl).GetDefaultAudioEndpoint)(self, dataFlow, role, ppEndpoint)
    }

    /// Call the vtable's GetDevice() member.
    ///
    /// https://msdn.microsoft.com/en-us/library/windows/desktop/dd371402.aspx
    pub unsafe fn GetDevice(
        &mut self,
        pwstrId: LPCWSTR,
        ppDevices: *mut *mut IMMDevice,
    ) -> HRESULT {
        ((&*self.lpVtbl).GetDevice)(self, pwstrId, ppDevices)
    }

    /// Call the vtable's RegisterEndpointNotificationCallback() member.
    ///
    /// https://msdn.microsoft.com/en-us/library/windows/desktop/dd371403.aspx
    pub unsafe fn RegisterEndpointNotificationCallback(
        &mut self,
        pClient: *mut IMMNotificationClient,
    ) -> HRESULT {
        ((&*self.lpVtbl).RegisterEndpointNotificationCallback)(self, pClient)
    }

    /// Call the vtable's UnregisterEndpointNotificationCallback() member.
    ///
    /// https://msdn.microsoft.com/en-us/library/windows/desktop/dd371404.aspx
    pub unsafe fn UnregisterEndpointNotificationCallback(
        &mut self,
        pClient: *mut IMMNotificationClient,
    ) -> HRESULT {
        ((&*self.lpVtbl).UnregisterEndpointNotificationCallback)(self, pClient)
    }
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IMMDeviceCollection;
#[repr(C)] #[derive(Clone, Copy, Debug)]
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
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IAudioClient {
    pub lpVtbl: *const IAudioClientVtbl,
}
impl IAudioClient {
    /// Part of the IUnknown interface inherited by IMMDeviceEnumerator.
    ///
    /// https://msdn.microsoft.com/en-us/library/windows/desktop/ms682521.aspx
    pub unsafe fn QueryInterface (
        &mut self,
        riid: REFIID,
        ppvObject: *mut *mut c_void,
    ) -> HRESULT {
        ((&*self.lpVtbl).QueryInterface)(self, riid, ppvObject)
    }
    /// Part of the IUnknown interface inherited by IMMDeviceEnumerator.
    ///
    /// https://msdn.microsoft.com/en-us/library/windows/desktop/ms691379.aspx
    pub unsafe fn AddRef(&mut self) -> ULONG {
        ((&*self.lpVtbl).AddRef)(self)
    }
    /// Part of the IUnknown interface inherited by IMMDeviceEnumerator.
    ///
    /// https://msdn.microsoft.com/en-us/library/windows/desktop/ms682317.aspx
    pub unsafe fn Release(&mut self) -> ULONG {
        ((&*self.lpVtbl).Release)(self)
    }
    pub unsafe fn Initialize(
        &mut self,
        ShareMode: AUDCLNT_SHAREMODE,
        StreamFlags: DWORD,
        hnsBufferDuration: REFERENCE_TIME,
        hnsPeriodicity: REFERENCE_TIME,
        pFormat: *const WAVEFORMATEX,
        AudioSessionGuid: LPCGUID,
    ) -> HRESULT {
        ((&*self.lpVtbl).Initialize)(
            self,
            ShareMode,
            StreamFlags,
            hnsBufferDuration,
            hnsPeriodicity,
            pFormat,
            AudioSessionGuid)
    }
    pub unsafe fn GetBufferSize(
        &mut self,
        pNumBufferFrames: *mut UINT32,
    ) -> HRESULT {
        ((&*self.lpVtbl).GetBufferSize)(self, pNumBufferFrames)
    }
    // pub GetStreamLatency: unsafe extern "system" fn(
    //     This: *mut IAudioClient,
    //     phnsLatency: *mut REFERENCE_TIME,
    // ) -> HRESULT,
    pub unsafe fn GetCurrentPadding(
        &mut self,
        pNumPaddingFrames: *mut UINT32,
    ) -> HRESULT {
        ((&*self.lpVtbl).GetCurrentPadding)(self, pNumPaddingFrames)
    }
    pub unsafe fn IsFormatSupported(
        &mut self,
        ShareMode: AUDCLNT_SHAREMODE,
        pFormat: *const WAVEFORMATEX,
        ppClosestMatch: *mut *mut WAVEFORMATEX,
    ) -> HRESULT {
        ((&*self.lpVtbl).IsFormatSupported)(self, ShareMode, pFormat, ppClosestMatch)
    }
    // pub GetMixFormat: unsafe extern "system" fn(
    //     This: *mut IAudioClient,
    //     ppDeviceFormat: *mut *mut WAVEFORMATEX,
    // ) -> HRESULT,
    // pub GetDevicePeriod: unsafe extern "system" fn(
    //     This: *mut IAudioClient,
    //     phnsDefaultDevicePeriod: *mut REFERENCE_TIME,
    //     phnsMinimumDevicePeriod: *mut REFERENCE_TIME,
    // ) -> HRESULT,
    pub unsafe fn Start(
        &mut self,
    ) -> HRESULT {
        ((&*self.lpVtbl).Start)(self)
    }
    pub unsafe fn Stop(
        &mut self,
    ) -> HRESULT {
        ((&*self.lpVtbl).Stop)(self)
    }
    // pub Reset: unsafe extern "system" fn(
    //     This: *mut IAudioClient,
    // ) -> HRESULT,
    // pub SetEventHandle: unsafe extern "system" fn(
    //     This: *mut IAudioClient,
    //     eventHandle: HANDLE,
    // ) -> HRESULT,
    pub unsafe fn GetService(
        &mut self,
        riid: REFIID,
        ppv: *mut LPVOID,
    ) -> HRESULT {
        ((&*self.lpVtbl).GetService)(self, riid, ppv)
    }
}
#[repr(C)]
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
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IAudioRenderClient {
    pub lpVtbl: *const IAudioRenderClientVtbl,
}
impl IAudioRenderClient {
    /// Part of the IUnknown interface inherited by IMMDeviceEnumerator.
    ///
    /// https://msdn.microsoft.com/en-us/library/windows/desktop/ms682521.aspx
    pub unsafe fn QueryInterface (
        &mut self,
        riid: REFIID,
        ppvObject: *mut *mut c_void,
    ) -> HRESULT {
        ((&*self.lpVtbl).QueryInterface)(self, riid, ppvObject)
    }
    /// Part of the IUnknown interface inherited by IMMDeviceEnumerator.
    ///
    /// https://msdn.microsoft.com/en-us/library/windows/desktop/ms691379.aspx
    pub unsafe fn AddRef(&mut self) -> ULONG {
        ((&*self.lpVtbl).AddRef)(self)
    }
    /// Part of the IUnknown interface inherited by IMMDeviceEnumerator.
    ///
    /// https://msdn.microsoft.com/en-us/library/windows/desktop/ms682317.aspx
    pub unsafe fn Release(&mut self) -> ULONG {
        ((&*self.lpVtbl).Release)(self)
    }
    pub unsafe fn GetBuffer(
        &mut self,
        NumFramesRequested: UINT32,
        ppData: *mut *mut BYTE,
    ) -> HRESULT {
        ((&*self.lpVtbl).GetBuffer)(self, NumFramesRequested, ppData)
    }
    pub unsafe fn ReleaseBuffer(
        &mut self,
        NumFramesWritten: UINT32,
        dwFlags: DWORD,
    ) -> HRESULT {
        ((&*self.lpVtbl).ReleaseBuffer)(self, NumFramesWritten, dwFlags)
    }
}

//-------------------------------------------------------------------------------------------------
// audiosessiontypes.h
//-------------------------------------------------------------------------------------------------
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub enum AUDCLNT_SHAREMODE {
    AUDCLNT_SHAREMODE_SHARED,
    AUDCLNT_SHAREMODE_EXCLUSIVE,
}

//-------------------------------------------------------------------------------------------------
// strmif.h
//-------------------------------------------------------------------------------------------------
pub type REFERENCE_TIME = LONGLONG;

//-------------------------------------------------------------------------------------------------
// mmreg.h
//-------------------------------------------------------------------------------------------------
#[repr(C)] #[derive(Clone, Copy, Debug)]
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
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IPropertyStore;

//-------------------------------------------------------------------------------------------------
// propidl.h
//-------------------------------------------------------------------------------------------------
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct PROPVARIANT {
    pub vt: VARTYPE,
    pub wReserved1: WORD,
    pub wReserved2: WORD,
    pub wReserved3: WORD,
    pub data: [u8; 16],
}

//-------------------------------------------------------------------------------------------------
// objldlbase.h
// this ALWAYS GENERATED file contains the definitions for the interfaces
//-------------------------------------------------------------------------------------------------
#[repr(C)]
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
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IMalloc {
    pub lpVtbl: *const IMallocVtbl,
}
pub type LPMALLOC = *mut IMalloc;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct STATSTG {
    pub pwcsName: LPOLESTR,
    pub type_: DWORD,
    pub cbSize: ULARGE_INTEGER,
    pub mtime: FILETIME,
    pub ctime: FILETIME,
    pub atime: FILETIME,
    pub grfMode: DWORD,
    pub grfLocksSupported: DWORD,
    pub clsid: CLSID,
    pub grfStateBits: DWORD,
    pub reserved: DWORD,
}
#[repr(C)]
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
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IStream {
    pub lpVtbl: *const IStreamVtbl,
}
pub type LPSTREAM = *mut IStream;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub enum APTTYPEQUALIFIER {
    APTTYPEQUALIFIER_NONE = 0,
    APTTYPEQUALIFIER_IMPLICIT_MTA = 1,
    APTTYPEQUALIFIER_NA_ON_MTA = 2,
    APTTYPEQUALIFIER_NA_ON_STA = 3,
    APTTYPEQUALIFIER_NA_ON_IMPLICIT_MTA = 4,
    APTTYPEQUALIFIER_NA_ON_MAINSTA = 5,
    APTTYPEQUALIFIER_APPLICATION_STA= 6,
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
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
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ServerInformation {
    pub dwServerPid: DWORD,
    pub dwServerTid: DWORD,
    pub ui64ServerAddress: UINT64,
}
pub type PServerInformation = *mut ServerInformation;
#[repr(C)]
pub struct CO_MTA_USAGE_COOKIE__;
pub type CO_MTA_USAGE_COOKIE = *mut CO_MTA_USAGE_COOKIE__;

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

#[repr(C)] #[derive(Clone, Copy, Debug)]
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
#[repr(C)] #[derive(Clone, Copy, Debug)]
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

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct VALENTA {
    pub ve_valuename: LPSTR,
    pub ve_valuelen: DWORD,
    pub ve_valueptr: DWORD_PTR,
    pub ve_type: DWORD,
}
pub type PVALENTA = *mut VALENTA;

#[repr(C)] #[derive(Clone, Copy, Debug)]
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

pub const CCHILDREN_SCROLLBAR: usize = 5;

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

pub const ENUM_CURRENT_SETTINGS: DWORD = 0xFFFFFFFF;
pub const ENUM_REGISTRY_SETTINGS: DWORD = 0xFFFFFFFE;

pub const GW_HWNDFIRST: UINT = 0;
pub const GW_HWNDLAST: UINT = 1;
pub const GW_HWNDNEXT: UINT = 2;
pub const GW_HWNDPREV: UINT = 3;
pub const GW_OWNER: UINT = 4;
pub const GW_CHILD: UINT = 5;
pub const GW_ENABLEDPOPUP: UINT = 6;
pub const GW_MAX: UINT = 6;

pub const LSFW_LOCK: UINT = 1;
pub const LSFW_UNLOCK: UINT = 2;

pub const MDITILE_VERTICAL: UINT = 0x0000;
pub const MDITILE_HORIZONTAL: UINT = 0x0001;
pub const MDITILE_SKIPDISABLED: UINT = 0x0002;
pub const MDITILE_ZORDER: UINT = 0x0004;

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

pub const SB_HORZ: c_int = 0;
pub const SB_VERT: c_int = 1;
pub const SB_CTL: c_int = 2;
pub const SB_BOTH: c_int = 3;

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
pub type DESKTOPENUMPROCA = Option<unsafe extern "system" fn(LPSTR, LPARAM) -> BOOL>;
pub type DESKTOPENUMPROCW = Option<unsafe extern "system" fn(LPWSTR, LPARAM) -> BOOL>;
pub type NAMEENUMPROCA = DESKTOPENUMPROCA;
pub type NAMEENUMPROCW = DESKTOPENUMPROCW;
pub type WNDENUMPROC = Option<unsafe extern "system" fn(HWND, LPARAM) -> BOOL>;
pub type WNDPROC = Option<unsafe extern "system" fn(HWND, UINT, WPARAM, LPARAM) -> LRESULT>;
pub type HOOKPROC = Option<unsafe extern "system" fn(
    code: c_int, wParam: WPARAM, lParam: LPARAM,
) -> LRESULT>;
pub type TimerProc = Option<unsafe extern "system" fn(
    hwnd: HWND, uMsg: UINT, idEvent: UINT_PTR, dwTime: DWORD,
)>;

pub type HDEVNOTIFY = PVOID;

#[repr(C)] #[derive(Clone, Copy, Debug)]
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
#[repr(C)] #[derive(Clone, Copy, Debug)]
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
#[repr(C)] #[derive(Clone, Copy, Debug)]
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
#[repr(C)] #[derive(Copy)]
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
impl Clone for WNDCLASSEXW { fn clone(&self) -> WNDCLASSEXW { *self } }
pub type PWNDCLASSEXW = *mut WNDCLASSEXW;
pub type NPWNDCLASSEXW = *mut WNDCLASSEXW;
pub type LPWNDCLASSEXW = *mut WNDCLASSEXW;
#[repr(C)] #[derive(Copy)]
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
impl Clone for WNDCLASSW { fn clone(&self) -> WNDCLASSW { *self } }
pub type PWNDCLASSW = *mut WNDCLASSW;
pub type NPWNDCLASSW = *mut WNDCLASSW;
pub type LPWNDCLASSW = *mut WNDCLASSW;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SCROLLBARINFO {
    pub cbSize: DWORD,
    pub rcScrollBar: RECT,
    pub dxyLineButton: c_int,
    pub xyThumbTop: c_int,
    pub xyThumbBottom: c_int,
    pub reserved: c_int,
    pub rgstate: [DWORD; CCHILDREN_SCROLLBAR + 1]
}
pub type PSCROLLBARINFO = *mut SCROLLBARINFO;
pub type LPSCROLLBARINFO = *mut SCROLLBARINFO;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SCROLLINFO {
    pub cbSize: UINT,
    pub fMask: UINT,
    pub nMin: c_int,
    pub nMax: c_int,
    pub nPage: UINT,
    pub nPos: c_int,
    pub nTrackPos: c_int
}
pub type LPSCROLLINFO = *mut SCROLLINFO;
pub type LPCSCROLLINFO = *const SCROLLINFO;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SIZE {
    pub cx: LONG,
    pub cy: LONG
}
pub type PSIZE = *mut SIZE;

//-------------------------------------------------------------------------------------------------
// mmsystem.h
//-------------------------------------------------------------------------------------------------

pub type MMRESULT = UINT;

pub const TIMERR_NOERROR: MMRESULT = 0;
pub const TIMERR_BASE:    MMRESULT = 96;
pub const TIMERR_NOCANDO: MMRESULT = TIMERR_BASE + 1;
pub const TIMERR_STRUCT:  MMRESULT = TIMERR_BASE + 33;

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
pub const PFD_UNDERLAY_PLANE: BYTE = 0xFF;
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

pub const CCHFORMNAME: usize = 32;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DEVMODEA {
    pub dmDeviceName: [CHAR; CCHDEVICENAME],
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
    pub dmFormName: [CHAR; CCHFORMNAME],
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
pub type PDEVMODEA = *mut DEVMODEA;
pub type NPDEVMODEA = *mut DEVMODEA;
pub type LPDEVMODEA = *mut DEVMODEA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
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
#[repr(C)] #[derive(Copy)]
pub struct DISPLAY_DEVICEW {
    pub cb: DWORD,
    pub DeviceName: [WCHAR; 32],
    pub DeviceString: [WCHAR; 128],
    pub StateFlags: DWORD,
    pub DeviceID: [WCHAR; 128],
    pub DeviceKey: [WCHAR; 128],
}
impl Clone for DISPLAY_DEVICEW { fn clone(&self) -> DISPLAY_DEVICEW { *self } }
pub type PDISPLAY_DEVICEW = *mut DISPLAY_DEVICEW;
pub type LPDISPLAY_DEVICEW = *mut DISPLAY_DEVICEW;
#[repr(C)] #[derive(Copy)]
pub struct DISPLAY_DEVICEA {
    pub cb: DWORD,
    pub DeviceName: [CHAR; 32],
    pub DeviceString: [CHAR; 128],
    pub StateFlags: DWORD,
    pub DeviceID: [CHAR; 128],
    pub DeviceKey: [CHAR; 128],
}
impl Clone for DISPLAY_DEVICEA { fn clone(&self) -> DISPLAY_DEVICEA { *self } }
pub type PDISPLAY_DEVICEA = *mut DISPLAY_DEVICEA;
pub type LPDISPLAY_DEVICEA = *mut DISPLAY_DEVICEA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
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
// constants
#[cfg(target_arch = "x86")]
pub const INVALID_HANDLE_VALUE: HANDLE = 0xFFFFFFFF as HANDLE;
#[cfg(target_arch = "x86_64")]
pub const INVALID_HANDLE_VALUE: HANDLE = 0xFFFFFFFFFFFFFFFF as HANDLE;

//-------------------------------------------------------------------------------------------------
// wingdi.h
// GDI procedure declarations, constant definitions and macros
//-------------------------------------------------------------------------------------------------

pub const COINIT_APARTMENTTHREADED: DWORD = 0x2;
pub const COINIT_MULTITHREADED:     DWORD = 0x0;
pub const COINIT_DISABLE_OLE1DDE:   DWORD = 0x4;
pub const COINIT_SPEED_OVER_MEMORY: DWORD = 0x8;

//-------------------------------------------------------------------------------------------------
// mmreg.h
//-------------------------------------------------------------------------------------------------

pub const WAVE_FORMAT_UNKNOWN: WORD = 0x0000;
pub const WAVE_FORMAT_PCM: WORD = 0x0001;
pub const WAVE_FORMAT_ADPCM: WORD = 0x0002;
pub const WAVE_FORMAT_IEEE_FLOAT: WORD = 0x0003;
pub const WAVE_FORMAT_VSELP: WORD = 0x0004;
pub const WAVE_FORMAT_IBM_CVSD: WORD = 0x0005;
pub const WAVE_FORMAT_ALAW: WORD = 0x0006;
pub const WAVE_FORMAT_MULAW: WORD = 0x0007;
pub const WAVE_FORMAT_DTS: WORD = 0x0008;
pub const WAVE_FORMAT_DRM: WORD = 0x0009;
pub const WAVE_FORMAT_WMAVOICE9: WORD = 0x000A;
pub const WAVE_FORMAT_WMAVOICE10: WORD = 0x000B;
pub const WAVE_FORMAT_OKI_ADPCM: WORD = 0x0010;
pub const WAVE_FORMAT_DVI_ADPCM: WORD = 0x0011;
pub const WAVE_FORMAT_IMA_ADPCM: WORD = WAVE_FORMAT_DVI_ADPCM;
pub const WAVE_FORMAT_MEDIASPACE_ADPCM: WORD = 0x0012;
pub const WAVE_FORMAT_SIERRA_ADPCM: WORD = 0x0013;
pub const WAVE_FORMAT_G723_ADPCM: WORD = 0x0014;
pub const WAVE_FORMAT_DIGISTD: WORD = 0x0015;
pub const WAVE_FORMAT_DIGIFIX: WORD = 0x0016;
pub const WAVE_FORMAT_DIALOGIC_OKI_ADPCM: WORD = 0x0017;
pub const WAVE_FORMAT_MEDIAVISION_ADPCM: WORD = 0x0018;
pub const WAVE_FORMAT_CU_CODEC: WORD = 0x0019;
pub const WAVE_FORMAT_HP_DYN_VOICE: WORD = 0x001A;
pub const WAVE_FORMAT_YAMAHA_ADPCM: WORD = 0x0020;
pub const WAVE_FORMAT_SONARC: WORD = 0x0021;
pub const WAVE_FORMAT_DSPGROUP_TRUESPEECH: WORD = 0x0022;
pub const WAVE_FORMAT_ECHOSC1: WORD = 0x0023;
pub const WAVE_FORMAT_AUDIOFILE_AF36: WORD = 0x0024;
pub const WAVE_FORMAT_APTX: WORD = 0x0025;
pub const WAVE_FORMAT_AUDIOFILE_AF10: WORD = 0x0026;
pub const WAVE_FORMAT_PROSODY_1612: WORD = 0x0027;
pub const WAVE_FORMAT_LRC: WORD = 0x0028;
pub const WAVE_FORMAT_DOLBY_AC2: WORD = 0x0030;
pub const WAVE_FORMAT_GSM610: WORD = 0x0031;
pub const WAVE_FORMAT_MSNAUDIO: WORD = 0x0032;
pub const WAVE_FORMAT_ANTEX_ADPCME: WORD = 0x0033;
pub const WAVE_FORMAT_CONTROL_RES_VQLPC: WORD = 0x0034;
pub const WAVE_FORMAT_DIGIREAL: WORD = 0x0035;
pub const WAVE_FORMAT_DIGIADPCM: WORD = 0x0036;
pub const WAVE_FORMAT_CONTROL_RES_CR10: WORD = 0x0037;
pub const WAVE_FORMAT_NMS_VBXADPCM: WORD = 0x0038;
pub const WAVE_FORMAT_CS_IMAADPCM: WORD = 0x0039;
pub const WAVE_FORMAT_ECHOSC3: WORD = 0x003A;
pub const WAVE_FORMAT_ROCKWELL_ADPCM: WORD = 0x003B;
pub const WAVE_FORMAT_ROCKWELL_DIGITALK: WORD = 0x003C;
pub const WAVE_FORMAT_XEBEC: WORD = 0x003D;
pub const WAVE_FORMAT_G721_ADPCM: WORD = 0x0040;
pub const WAVE_FORMAT_G728_CELP: WORD = 0x0041;
pub const WAVE_FORMAT_MSG723: WORD = 0x0042;
pub const WAVE_FORMAT_INTEL_G723_1: WORD = 0x0043;
pub const WAVE_FORMAT_INTEL_G729: WORD = 0x0044;
pub const WAVE_FORMAT_SHARP_G726: WORD = 0x0045;
pub const WAVE_FORMAT_MPEG: WORD = 0x0050;
pub const WAVE_FORMAT_RT24: WORD = 0x0052;
pub const WAVE_FORMAT_PAC: WORD = 0x0053;
pub const WAVE_FORMAT_MPEGLAYER3: WORD = 0x0055;
pub const WAVE_FORMAT_LUCENT_G723: WORD = 0x0059;
pub const WAVE_FORMAT_CIRRUS: WORD = 0x0060;
pub const WAVE_FORMAT_ESPCM: WORD = 0x0061;
pub const WAVE_FORMAT_VOXWARE: WORD = 0x0062;
pub const WAVE_FORMAT_CANOPUS_ATRAC: WORD = 0x0063;
pub const WAVE_FORMAT_G726_ADPCM: WORD = 0x0064;
pub const WAVE_FORMAT_G722_ADPCM: WORD = 0x0065;
pub const WAVE_FORMAT_DSAT: WORD = 0x0066;
pub const WAVE_FORMAT_DSAT_DISPLAY: WORD = 0x0067;
pub const WAVE_FORMAT_VOXWARE_BYTE_ALIGNED: WORD = 0x0069;
pub const WAVE_FORMAT_VOXWARE_AC8: WORD = 0x0070;
pub const WAVE_FORMAT_VOXWARE_AC10: WORD = 0x0071;
pub const WAVE_FORMAT_VOXWARE_AC16: WORD = 0x0072;
pub const WAVE_FORMAT_VOXWARE_AC20: WORD = 0x0073;
pub const WAVE_FORMAT_VOXWARE_RT24: WORD = 0x0074;
pub const WAVE_FORMAT_VOXWARE_RT29: WORD = 0x0075;
pub const WAVE_FORMAT_VOXWARE_RT29HW: WORD = 0x0076;
pub const WAVE_FORMAT_VOXWARE_VR12: WORD = 0x0077;
pub const WAVE_FORMAT_VOXWARE_VR18: WORD = 0x0078;
pub const WAVE_FORMAT_VOXWARE_TQ40: WORD = 0x0079;
pub const WAVE_FORMAT_VOXWARE_SC3: WORD = 0x007A;
pub const WAVE_FORMAT_VOXWARE_SC3_1: WORD = 0x007B;
pub const WAVE_FORMAT_SOFTSOUND: WORD = 0x0080;
pub const WAVE_FORMAT_VOXWARE_TQ60: WORD = 0x0081;
pub const WAVE_FORMAT_MSRT24: WORD = 0x0082;
pub const WAVE_FORMAT_G729A: WORD = 0x0083;
pub const WAVE_FORMAT_MVI_MVI2: WORD = 0x0084;
pub const WAVE_FORMAT_DF_G726: WORD = 0x0085;
pub const WAVE_FORMAT_DF_GSM610: WORD = 0x0086;
pub const WAVE_FORMAT_ISIAUDIO: WORD = 0x0088;
pub const WAVE_FORMAT_ONLIVE: WORD = 0x0089;
pub const WAVE_FORMAT_MULTITUDE_FT_SX20: WORD = 0x008A;
pub const WAVE_FORMAT_INFOCOM_ITS_G721_ADPCM: WORD = 0x008B;
pub const WAVE_FORMAT_CONVEDIA_G729: WORD = 0x008C;
pub const WAVE_FORMAT_CONGRUENCY: WORD = 0x008D;
pub const WAVE_FORMAT_SBC24: WORD = 0x0091;
pub const WAVE_FORMAT_DOLBY_AC3_SPDIF: WORD = 0x0092;
pub const WAVE_FORMAT_MEDIASONIC_G723: WORD = 0x0093;
pub const WAVE_FORMAT_PROSODY_8KBPS: WORD = 0x0094;
pub const WAVE_FORMAT_ZYXEL_ADPCM: WORD = 0x0097;
pub const WAVE_FORMAT_PHILIPS_LPCBB: WORD = 0x0098;
pub const WAVE_FORMAT_PACKED: WORD = 0x0099;
pub const WAVE_FORMAT_MALDEN_PHONYTALK: WORD = 0x00A0;
pub const WAVE_FORMAT_RACAL_RECORDER_GSM: WORD = 0x00A1;
pub const WAVE_FORMAT_RACAL_RECORDER_G720_A: WORD = 0x00A2;
pub const WAVE_FORMAT_RACAL_RECORDER_G723_1: WORD = 0x00A3;
pub const WAVE_FORMAT_RACAL_RECORDER_TETRA_ACELP: WORD = 0x00A4;
pub const WAVE_FORMAT_NEC_AAC: WORD = 0x00B0;
pub const WAVE_FORMAT_RAW_AAC1: WORD = 0x00FF;
pub const WAVE_FORMAT_RHETOREX_ADPCM: WORD = 0x0100;
pub const WAVE_FORMAT_IRAT: WORD = 0x0101;
pub const WAVE_FORMAT_VIVO_G723: WORD = 0x0111;
pub const WAVE_FORMAT_VIVO_SIREN: WORD = 0x0112;
pub const WAVE_FORMAT_PHILIPS_CELP: WORD = 0x0120;
pub const WAVE_FORMAT_PHILIPS_GRUNDIG: WORD = 0x0121;
pub const WAVE_FORMAT_DIGITAL_G723: WORD = 0x0123;
pub const WAVE_FORMAT_SANYO_LD_ADPCM: WORD = 0x0125;
pub const WAVE_FORMAT_SIPROLAB_ACEPLNET: WORD = 0x0130;
pub const WAVE_FORMAT_SIPROLAB_ACELP4800: WORD = 0x0131;
pub const WAVE_FORMAT_SIPROLAB_ACELP8V3: WORD = 0x0132;
pub const WAVE_FORMAT_SIPROLAB_G729: WORD = 0x0133;
pub const WAVE_FORMAT_SIPROLAB_G729A: WORD = 0x0134;
pub const WAVE_FORMAT_SIPROLAB_KELVIN: WORD = 0x0135;
pub const WAVE_FORMAT_VOICEAGE_AMR: WORD = 0x0136;
pub const WAVE_FORMAT_G726ADPCM: WORD = 0x0140;
pub const WAVE_FORMAT_DICTAPHONE_CELP68: WORD = 0x0141;
pub const WAVE_FORMAT_DICTAPHONE_CELP54: WORD = 0x0142;
pub const WAVE_FORMAT_QUALCOMM_PUREVOICE: WORD = 0x0150;
pub const WAVE_FORMAT_QUALCOMM_HALFRATE: WORD = 0x0151;
pub const WAVE_FORMAT_TUBGSM: WORD = 0x0155;
pub const WAVE_FORMAT_MSAUDIO1: WORD = 0x0160;
pub const WAVE_FORMAT_WMAUDIO2: WORD = 0x0161;
pub const WAVE_FORMAT_WMAUDIO3: WORD = 0x0162;
pub const WAVE_FORMAT_WMAUDIO_LOSSLESS: WORD = 0x0163;
pub const WAVE_FORMAT_WMASPDIF: WORD = 0x0164;
pub const WAVE_FORMAT_UNISYS_NAP_ADPCM: WORD = 0x0170;
pub const WAVE_FORMAT_UNISYS_NAP_ULAW: WORD = 0x0171;
pub const WAVE_FORMAT_UNISYS_NAP_ALAW: WORD = 0x0172;
pub const WAVE_FORMAT_UNISYS_NAP_16K: WORD = 0x0173;
pub const WAVE_FORMAT_SYCOM_ACM_SYC008: WORD = 0x0174;
pub const WAVE_FORMAT_SYCOM_ACM_SYC701_G726L: WORD = 0x0175;
pub const WAVE_FORMAT_SYCOM_ACM_SYC701_CELP54: WORD = 0x0176;
pub const WAVE_FORMAT_SYCOM_ACM_SYC701_CELP68: WORD = 0x0177;
pub const WAVE_FORMAT_KNOWLEDGE_ADVENTURE_ADPCM: WORD = 0x0178;
pub const WAVE_FORMAT_FRAUNHOFER_IIS_MPEG2_AAC: WORD = 0x0180;
pub const WAVE_FORMAT_DTS_DS: WORD = 0x0190;
pub const WAVE_FORMAT_CREATIVE_ADPCM: WORD = 0x0200;
pub const WAVE_FORMAT_CREATIVE_FASTSPEECH8: WORD = 0x0202;
pub const WAVE_FORMAT_CREATIVE_FASTSPEECH10: WORD = 0x0203;
pub const WAVE_FORMAT_UHER_ADPCM: WORD = 0x0210;
pub const WAVE_FORMAT_ULEAD_DV_AUDIO: WORD = 0x0215;
pub const WAVE_FORMAT_ULEAD_DV_AUDIO_1: WORD = 0x0216;
pub const WAVE_FORMAT_QUARTERDECK: WORD = 0x0220;
pub const WAVE_FORMAT_ILINK_VC: WORD = 0x0230;
pub const WAVE_FORMAT_RAW_SPORT: WORD = 0x0240;
pub const WAVE_FORMAT_ESST_AC3: WORD = 0x0241;
pub const WAVE_FORMAT_GENERIC_PASSTHRU: WORD = 0x0249;
pub const WAVE_FORMAT_IPI_HSX: WORD = 0x0250;
pub const WAVE_FORMAT_IPI_RPELP: WORD = 0x0251;
pub const WAVE_FORMAT_CS2: WORD = 0x0260;
pub const WAVE_FORMAT_SONY_SCX: WORD = 0x0270;
pub const WAVE_FORMAT_SONY_SCY: WORD = 0x0271;
pub const WAVE_FORMAT_SONY_ATRAC3: WORD = 0x0272;
pub const WAVE_FORMAT_SONY_SPC: WORD = 0x0273;
pub const WAVE_FORMAT_TELUM_AUDIO: WORD = 0x0280;
pub const WAVE_FORMAT_TELUM_IA_AUDIO: WORD = 0x0281;
pub const WAVE_FORMAT_NORCOM_VOICE_SYSTEMS_ADPCM: WORD = 0x0285;
pub const WAVE_FORMAT_FM_TOWNS_SND: WORD = 0x0300;
pub const WAVE_FORMAT_MICRONAS: WORD = 0x0350;
pub const WAVE_FORMAT_MICRONAS_CELP833: WORD = 0x0351;
pub const WAVE_FORMAT_BTV_DIGITAL: WORD = 0x0400;
pub const WAVE_FORMAT_INTEL_MUSIC_CODER: WORD = 0x0401;
pub const WAVE_FORMAT_INDEO_AUDIO: WORD = 0x0402;
pub const WAVE_FORMAT_QDESIGN_MUSIC: WORD = 0x0450;
pub const WAVE_FORMAT_ON2_VP7_AUDIO: WORD = 0x0500;
pub const WAVE_FORMAT_ON2_VP6_AUDIO: WORD = 0x0501;
pub const WAVE_FORMAT_VME_VMPCM: WORD = 0x0680;
pub const WAVE_FORMAT_TPC: WORD = 0x0681;
pub const WAVE_FORMAT_LIGHTWAVE_LOSSLESS: WORD = 0x08AE;
pub const WAVE_FORMAT_OLIGSM: WORD = 0x1000;
pub const WAVE_FORMAT_OLIADPCM: WORD = 0x1001;
pub const WAVE_FORMAT_OLICELP: WORD = 0x1002;
pub const WAVE_FORMAT_OLISBC: WORD = 0x1003;
pub const WAVE_FORMAT_OLIOPR: WORD = 0x1004;
pub const WAVE_FORMAT_LH_CODEC: WORD = 0x1100;
pub const WAVE_FORMAT_LH_CODEC_CELP: WORD = 0x1101;
pub const WAVE_FORMAT_LH_CODEC_SBC8: WORD = 0x1102;
pub const WAVE_FORMAT_LH_CODEC_SBC12: WORD = 0x1103;
pub const WAVE_FORMAT_LH_CODEC_SBC16: WORD = 0x1104;
pub const WAVE_FORMAT_NORRIS: WORD = 0x1400;
pub const WAVE_FORMAT_ISIAUDIO_2: WORD = 0x1401;
pub const WAVE_FORMAT_SOUNDSPACE_MUSICOMPRESS: WORD = 0x1500;
pub const WAVE_FORMAT_MPEG_ADTS_AAC: WORD = 0x1600;
pub const WAVE_FORMAT_MPEG_RAW_AAC: WORD = 0x1601;
pub const WAVE_FORMAT_MPEG_LOAS: WORD = 0x1602;
pub const WAVE_FORMAT_NOKIA_MPEG_ADTS_AAC: WORD = 0x1608;
pub const WAVE_FORMAT_NOKIA_MPEG_RAW_AAC: WORD = 0x1609;
pub const WAVE_FORMAT_VODAFONE_MPEG_ADTS_AAC: WORD = 0x160A;
pub const WAVE_FORMAT_VODAFONE_MPEG_RAW_AAC: WORD = 0x160B;
pub const WAVE_FORMAT_MPEG_HEAAC: WORD = 0x1610;
pub const WAVE_FORMAT_VOXWARE_RT24_SPEECH: WORD = 0x181C;
pub const WAVE_FORMAT_SONICFOUNDRY_LOSSLESS: WORD = 0x1971;
pub const WAVE_FORMAT_INNINGS_TELECOM_ADPCM: WORD = 0x1979;
pub const WAVE_FORMAT_LUCENT_SX8300P: WORD = 0x1C07;
pub const WAVE_FORMAT_LUCENT_SX5363S: WORD = 0x1C0C;
pub const WAVE_FORMAT_CUSEEME: WORD = 0x1F03;
pub const WAVE_FORMAT_NTCSOFT_ALF2CM_ACM: WORD = 0x1FC4;
pub const WAVE_FORMAT_DVM: WORD = 0x2000;
pub const WAVE_FORMAT_DTS2: WORD = 0x2001;
pub const WAVE_FORMAT_MAKEAVIS: WORD = 0x3313;
pub const WAVE_FORMAT_DIVIO_MPEG4_AAC: WORD = 0x4143;
pub const WAVE_FORMAT_NOKIA_ADAPTIVE_MULTIRATE: WORD = 0x4201;
pub const WAVE_FORMAT_DIVIO_G726: WORD = 0x4243;
pub const WAVE_FORMAT_LEAD_SPEECH: WORD = 0x434C;
pub const WAVE_FORMAT_LEAD_VORBIS: WORD = 0x564C;
pub const WAVE_FORMAT_WAVPACK_AUDIO: WORD = 0x5756;
pub const WAVE_FORMAT_OGG_VORBIS_MODE_1: WORD = 0x674F;
pub const WAVE_FORMAT_OGG_VORBIS_MODE_2: WORD = 0x6750;
pub const WAVE_FORMAT_OGG_VORBIS_MODE_3: WORD = 0x6751;
pub const WAVE_FORMAT_OGG_VORBIS_MODE_1_PLUS: WORD = 0x676F;
pub const WAVE_FORMAT_OGG_VORBIS_MODE_2_PLUS: WORD = 0x6770;
pub const WAVE_FORMAT_OGG_VORBIS_MODE_3_PLUS: WORD = 0x6771;
pub const WAVE_FORMAT_3COM_NBX: WORD = 0x7000;
pub const WAVE_FORMAT_FAAD_AAC: WORD = 0x706D;
pub const WAVE_FORMAT_AMR_NB: WORD = 0x7361;
pub const WAVE_FORMAT_AMR_WB: WORD = 0x7362;
pub const WAVE_FORMAT_AMR_WP: WORD = 0x7363;
pub const WAVE_FORMAT_GSM_AMR_CBR: WORD = 0x7A21;
pub const WAVE_FORMAT_GSM_AMR_VBR_SID: WORD = 0x7A22;
pub const WAVE_FORMAT_COMVERSE_INFOSYS_G723_1: WORD = 0xA100;
pub const WAVE_FORMAT_COMVERSE_INFOSYS_AVQSBC: WORD = 0xA101;
pub const WAVE_FORMAT_COMVERSE_INFOSYS_SBC: WORD = 0xA102;
pub const WAVE_FORMAT_SYMBOL_G729_A: WORD = 0xA103;
pub const WAVE_FORMAT_VOICEAGE_AMR_WB: WORD = 0xA104;
pub const WAVE_FORMAT_INGENIENT_G726: WORD = 0xA105;
pub const WAVE_FORMAT_MPEG4_AAC: WORD = 0xA106;
pub const WAVE_FORMAT_ENCORE_G726: WORD = 0xA107;
pub const WAVE_FORMAT_ZOLL_ASAO: WORD = 0xA108;
pub const WAVE_FORMAT_SPEEX_VOICE: WORD = 0xA109;
pub const WAVE_FORMAT_VIANIX_MASC: WORD = 0xA10A;
pub const WAVE_FORMAT_WM9_SPECTRUM_ANALYZER: WORD = 0xA10B;
pub const WAVE_FORMAT_WMF_SPECTRUM_ANAYZER: WORD = 0xA10C;
pub const WAVE_FORMAT_GSM_610: WORD = 0xA10D;
pub const WAVE_FORMAT_GSM_620: WORD = 0xA10E;
pub const WAVE_FORMAT_GSM_660: WORD = 0xA10F;
pub const WAVE_FORMAT_GSM_690: WORD = 0xA110;
pub const WAVE_FORMAT_GSM_ADAPTIVE_MULTIRATE_WB: WORD = 0xA111;
pub const WAVE_FORMAT_POLYCOM_G722: WORD = 0xA112;
pub const WAVE_FORMAT_POLYCOM_G728: WORD = 0xA113;
pub const WAVE_FORMAT_POLYCOM_G729_A: WORD = 0xA114;
pub const WAVE_FORMAT_POLYCOM_SIREN: WORD = 0xA115;
pub const WAVE_FORMAT_GLOBAL_IP_ILBC: WORD = 0xA116;
pub const WAVE_FORMAT_RADIOTIME_TIME_SHIFT_RADIO: WORD = 0xA117;
pub const WAVE_FORMAT_NICE_ACA: WORD = 0xA118;
pub const WAVE_FORMAT_NICE_ADPCM: WORD = 0xA119;
pub const WAVE_FORMAT_VOCORD_G721: WORD = 0xA11A;
pub const WAVE_FORMAT_VOCORD_G726: WORD = 0xA11B;
pub const WAVE_FORMAT_VOCORD_G722_1: WORD = 0xA11C;
pub const WAVE_FORMAT_VOCORD_G728: WORD = 0xA11D;
pub const WAVE_FORMAT_VOCORD_G729: WORD = 0xA11E;
pub const WAVE_FORMAT_VOCORD_G729_A: WORD = 0xA11F;
pub const WAVE_FORMAT_VOCORD_G723_1: WORD = 0xA120;
pub const WAVE_FORMAT_VOCORD_LBC: WORD = 0xA121;
pub const WAVE_FORMAT_NICE_G728: WORD = 0xA122;
pub const WAVE_FORMAT_FRACE_TELECOM_G729: WORD = 0xA123;
pub const WAVE_FORMAT_CODIAN: WORD = 0xA124;
pub const WAVE_FORMAT_FLAC: WORD = 0xF1AC;
pub const WAVE_FORMAT_EXTENSIBLE: WORD = 0xFFFE;
pub const WAVE_FORMAT_DEVELOPMENT: WORD = 0xFFFF;

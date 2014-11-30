// Copyright © 2014, Peter Atashian

#![feature(globs)]
#![no_std]
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

//-------------------------------------------------------------------------------------------------
// Basic primitives
//-------------------------------------------------------------------------------------------------
extern crate libc;
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
pub type __int64 = i64; // __int64, signed __int64
pub type __uint64 = u64; // unsigned __int64
//-------------------------------------------------------------------------------------------------
// minwindef.h
// Basic Windows Type Definitions for minwin partition
//-------------------------------------------------------------------------------------------------
pub const MAX_PATH: uint = 260;
pub const FALSE: BOOL = 0;
pub const TRUE: BOOL = 1;
pub type ULONG = c_ulong;
pub type PULONG = *mut ULONG;
pub type USHORT = c_ushort;
pub type PUSHORT = *mut USHORT;
pub type UCHAR = c_uchar;
pub type PUCHAR = *mut UCHAR;
pub type PSZ = *mut c_char;
pub type DWORD = c_ulong;
pub type BOOL = c_int;
pub type BYTE = c_uchar;
pub type WORD = c_ushort;
pub type FLOAT = c_float;
pub type PFLOAT = *mut FLOAT;
pub type PBOOL = *mut BOOL;
pub type LPBOOL = *mut BOOL;
pub type PBYTE = *mut BYTE;
pub type LPBYTE = *mut BYTE;
pub type PINT = *mut c_int;
pub type LPINT = *mut c_int;
pub type PWORD = *mut WORD;
pub type LPWORD = *mut WORD;
pub type LPLONG = *mut c_long;
pub type PDWORD = *mut DWORD;
pub type LPDWORD = *mut DWORD;
pub type LPVOID = *mut c_void;
pub type LPCVOID = *const c_void;
pub type INT = c_int;
pub type UINT = c_uint;
pub type PUINT = *mut c_uint;
pub type WPARAM = UINT_PTR;
pub type LPARAM = LONG_PTR;
pub type LRESULT = LONG_PTR;
pub type SPHANDLE = *mut HANDLE;
pub type LPHANDLE = *mut HANDLE;
pub type HGLOBAL = HANDLE;
pub type HLOCAL = HANDLE;
pub type GLOBALHANDLE = HANDLE;
pub type LOCALHANDLE = HANDLE;
#[cfg(target_arch = "x86")]
pub type FARPROC = extern "system" fn() -> INT;
#[cfg(target_arch = "x86_64")]
pub type FARPROC = extern "system" fn() -> INT_PTR;
#[cfg(target_arch = "x86")]
pub type NEARPROC = extern "system" fn() -> INT;
#[cfg(target_arch = "x86_64")]
pub type NEARPROC = extern "system" fn() -> INT_PTR;
#[cfg(target_arch = "x86")]
pub type PROC = extern "system" fn() -> INT;
#[cfg(target_arch = "x86_64")]
pub type PROC = extern "system" fn() -> INT_PTR;
pub type ATOM = WORD;
#[repr(C)]
pub struct HKEY__;
pub type HKEY = *mut HKEY__;
pub type PHKEY = *mut HKEY;
#[repr(C)]
pub struct HMETAFILE__;
pub type HMETAFILE = *mut HMETAFILE__;
#[repr(C)]
pub struct HINSTANCE__;
pub type HINSTANCE = *mut HINSTANCE__;
pub type HMODULE = HINSTANCE;
#[repr(C)]
pub struct HRGN__;
pub type HRGN = *mut HRGN__;
#[repr(C)]
pub struct HRSRC__;
pub type HRSRC = *mut HRSRC__;
#[repr(C)]
pub struct HSPRITE__;
pub type HSPRITE = *mut HSPRITE__;
#[repr(C)]
pub struct HLSURF__;
pub type HLSURF = *mut HLSURF__;
#[repr(C)]
pub struct HSTR__;
pub type HSTR = *mut HSTR__;
#[repr(C)]
pub struct HTASK__;
pub type HTASK = *mut HTASK__;
#[repr(C)]
pub struct HWINSTA__;
pub type HWINSTA = *mut HWINSTA__;
#[repr(C)]
pub struct HKL__;
pub type HKL = *mut HKL__;
pub type HFILE = c_int;
#[repr(C)]
pub struct FILETIME {
    pub dwLowDateTime: DWORD,
    pub dwHighDateTime: DWORD,
}
pub type PFILETIME = *mut FILETIME;
pub type LPFILETIME = *mut FILETIME;
// basetsd.h
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
//-------------------------------------------------------------------------------------------------
// winnt.h
// This module defines the 32-Bit Windows types and constants that are defined by NT, but exposed
// through the Win32 API.
//-------------------------------------------------------------------------------------------------
pub type PVOID = *mut c_void;
pub type PVOID64 = u64; // This is a 64-bit pointer, even when in 32-bit
pub type VOID = c_void;
pub type CHAR = c_char;
pub type SHORT = c_short;
pub type LONG = c_long;
// pub type INT = c_int; // Already defined in minwindef.h
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
#[repr(C)]
pub struct PROCESSOR_NUMBER {
    pub Group: WORD,
    pub Number: BYTE,
    pub Reserved: BYTE,
}
pub type PPROCESSOR_NUMBER = *mut PROCESSOR_NUMBER;
#[repr(C)]
pub struct GROUP_AFFINITY {
    pub Mask: KAFFINITY,
    pub Group: WORD,
    pub Reserved: [WORD, ..3],
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
pub enum COMPARTMENT_ID {
    UNSPECIFIED_COMPARTMENT_ID = 0,
    DEFAULT_COMPARTMENT_ID = 1,
}
pub type PCOMPARTMENT_ID = *mut COMPARTMENT_ID;
#[repr(C)]
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
pub struct LIST_ENTRY {
    pub Flink: *mut LIST_ENTRY,
    pub Blink: *mut LIST_ENTRY,
}
pub type PLIST_ENTRY = *mut LIST_ENTRY;
#[repr(C)]
pub struct SINGLE_LIST_ENTRY {
    pub Next: *mut SINGLE_LIST_ENTRY,
}
pub type PSINGLE_LIST_ENTRY = *mut SINGLE_LIST_ENTRY;
#[repr(C)]
pub struct LIST_ENTRY32 {
    pub Flink: DWORD,
    pub Blink: DWORD,
}
pub type PLIST_ENTRY32 = *mut LIST_ENTRY32;
#[repr(C)]
pub struct LIST_ENTRY64 {
    pub Flink: ULONGLONG,
    pub Blink: ULONGLONG,
}
pub type PLIST_ENTRY64 = *mut LIST_ENTRY64;
#[repr(C)]
pub struct OBJECTID {
    pub Lineage: GUID,
    pub Uniquifier: DWORD,
}
pub type EXCEPTION_ROUTINE = extern "system" fn(
    *mut _EXCEPTION_RECORD,
    PVOID,
    *mut _CONTEXT,
    PVOID,
) -> EXCEPTION_DISPOSITION;
pub type PEXCEPTION_ROUTINE = *mut EXCEPTION_ROUTINE;
pub type KSPIN_LOCK = ULONG_PTR;
pub type PKSPIN_LOCK = *mut KSPIN_LOCK;
#[repr(C)]
pub struct M128A { // FIXME align 16
    pub Low: ULONGLONG,
    pub High: LONGLONG,
}
pub type PM128A = *mut M128A;
#[cfg(target_arch = "x86")]
#[repr(C)]
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
    pub FloatRegisters: [M128A, ..8],
    pub XmmRegisters: [M128A, ..8],
    pub Reserved4: [BYTE, ..224],
}
#[cfg(target_arch = "x86_64")]
#[repr(C)]
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
    pub FloatRegisters: [M128A, ..8],
    pub XmmRegisters: [M128A, ..16],
    pub Reserved4: [BYTE, ..96],
}
#[repr(C)]
pub struct TOKEN_PRIVILEGES {
    PrivilegeCount: DWORD,
    Privileges: [LUID_AND_ATTRIBUTES, ..0],
}
pub type PTOKEN_PRIVILEGES = *mut TOKEN_PRIVILEGES;
#[repr(C)]
pub struct LUID_AND_ATTRIBUTES {
    Luid: LUID,
    Attributes: DWORD,
}
pub type PLUID_AND_ATTRIBUTES = *mut LUID_AND_ATTRIBUTES;
pub const DELETE: DWORD = 0x00010000;
pub const READ_CONTROL: DWORD = 0x00020000;
pub const WRITE_DAC: DWORD = 0x00040000;
pub const WRITE_OWNER: DWORD = 0x00080000;
pub const SYNCHRONIZE: DWORD = 0x00100000;
pub const STANDARD_RIGHTS_REQUIRED: DWORD = 0x000F0000;
pub const STANDARD_RIGHTS_READ: DWORD = READ_CONTROL;
pub const STANDARD_RIGHTS_WRITE: DWORD = READ_CONTROL;
pub const STANDARD_RIGHTS_EXECUTE: DWORD = READ_CONTROL;
pub const STANDARD_RIGHTS_ALL: DWORD = 0x001F0000;
pub const SPECIFIC_RIGHTS_ALL: DWORD = 0x0000FFFF;
pub const ACCESS_SYSTEM_SECURITY: DWORD = 0x01000000;
pub const MAXIMUM_ALLOWED: DWORD = 0x02000000;
pub const GENERIC_READ: DWORD = 0x80000000;
pub const GENERIC_WRITE: DWORD = 0x40000000;
pub const GENERIC_EXECUTE: DWORD = 0x20000000;
pub const GENERIC_ALL: DWORD = 0x10000000;
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
// guiddef.h
#[repr(C)]
pub struct GUID {
    pub Data1: c_ulong,
    pub Data2: c_ushort,
    pub Data3: c_ushort,
    pub Data4: [c_uchar, ..8],
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
pub enum EXCEPTION_DISPOSITION {
    ExceptionContinueExecution = 0,
    ExceptionContinueSearch = 1,
    ExceptionNestedException = 2,
    ExceptionCollidedUnwind = 3,
}
#[repr(C)]
pub struct _EXCEPTION_RECORD;
#[repr(C)]
pub struct _CONTEXT;
#[cfg(target_arch = "x86_64")]
#[repr(C)]
pub struct _DISPATCHER_CONTEXT;
// shlobj.h
pub type GPFIDL_FLAGS = c_int;
#[repr(C)]
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
    KF_FLAG_ALIAS_ONLY = 0x80000000,
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
pub struct HWND__;
pub type HWND = *mut HWND__;
#[repr(C)]
pub struct HHOOK__;
pub type HHOOK = *mut HHOOK__;
pub type HGDIOBJ = *mut c_void;
#[repr(C)]
pub struct HACCEL__;
pub type HACCEL = *mut HACCEL__;
#[repr(C)]
pub struct HBITMAP__;
pub type HBITMAP = *mut HBITMAP__;
#[repr(C)]
pub struct HBRUSH__;
pub type HBRUSH = *mut HBRUSH__;
#[repr(C)]
pub struct HCOLORSPACE__;
pub type HCOLORSPACE = *mut HCOLORSPACE__;
#[repr(C)]
pub struct HDC__;
pub type HDC = *mut HDC__;
#[repr(C)]
pub struct HGLRC__;
pub type HGLRC = *mut HGLRC__;
#[repr(C)]
pub struct HDESK__;
pub type HDESK = *mut HDESK__;
#[repr(C)]
pub struct HENHMETAFILE__;
pub type HENHMETAFILE = *mut HENHMETAFILE__;
#[repr(C)]
pub struct HFONT__;
pub type HFONT = *mut HFONT__;
#[repr(C)]
pub struct HICON__;
pub type HICON = *mut HICON__;
#[repr(C)]
pub struct HMENU__;
pub type HMENU = *mut HMENU__;
#[repr(C)]
pub struct HPALETTE__;
pub type HPALETTE = *mut HPALETTE__;
#[repr(C)]
pub struct HPEN__;
pub type HPEN = *mut HPEN__;
#[repr(C)]
pub struct HWINEVENTHOOK__;
pub type HWINEVENTHOOK = *mut HWINEVENTHOOK__;
#[repr(C)]
pub struct HMONITOR__;
pub type HMONITOR = *mut HMONITOR__;
#[repr(C)]
pub struct HUMPD__;
pub type HUMPD = *mut HUMPD__;
pub type HCURSOR = HICON;
pub type COLORREF = DWORD;
pub type LPCOLORREF = *mut DWORD;
#[repr(C)]
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
pub struct POINT {
    pub x: LONG,
    pub y: LONG,
}
pub type PPOINT = *mut POINT;
pub type NPPOINT = *mut POINT;
pub type LPPOINT = *mut POINT;
// minwinbase.h
#[repr(C)]
pub struct SECURITY_ATTRIBUTES {
    pub nLength: DWORD,
    pub lpSecurityDescriptor: LPVOID,
    pub bInheritHandle: BOOL,
}
pub type PSECURITY_ATTRIBUTES = *mut SECURITY_ATTRIBUTES;
pub type LPSECURITY_ATTRIBUTES = *mut SECURITY_ATTRIBUTES;
#[repr(C)]
pub struct OVERLAPPED {
    pub Internal: ULONG_PTR,
    pub InternalHigh: ULONG_PTR,
    pub Offset: DWORD,
    pub OffsetHigh: DWORD,
    pub hEvent: HANDLE,
}
pub type LPOVERLAPPED = *mut OVERLAPPED;
#[repr(C)]
pub struct OVERLAPPED_ENTRY {
    pub lpCompletionKey: ULONG_PTR,
    pub lpOverlapped: LPOVERLAPPED,
    pub Internal: ULONG_PTR,
    pub dwNumberOfBytesTransferred: DWORD,
}
pub type LPOVERLAPPED_ENTRY = *mut OVERLAPPED_ENTRY;
#[repr(C)]
pub struct SYSTEMTIME {
    pub wYear: DWORD,
    pub wMonth: DWORD,
    pub wDayOfWeek: DWORD,
    pub wDay: DWORD,
    pub wHour: DWORD,
    pub wMinute: DWORD,
    pub wSecond: DWORD,
    pub wMilliseconds: DWORD,
}
pub type PSYSTEMTIME = *mut SYSTEMTIME;
pub type LPSYSTEMTIME = *mut SYSTEMTIME;
// shlobjdl.h
#[repr(C)]
pub struct IContextMenu;
#[repr(C)]
pub struct IContextMenu2;
#[repr(C)]
pub struct IContextMenu3;
#[repr(C)]
pub struct IExecuteCommand;
#[repr(C)]
pub struct IPersistFolder;
#[repr(C)]
pub struct IRunnableTask;
#[repr(C)]
pub struct IShellTaskScheduler;
#[repr(C)]
pub struct IQueryCodePage;
#[repr(C)]
pub struct IPersistFolder2;
#[repr(C)]
pub struct IPersistFolder3;
#[repr(C)]
pub struct IPersistIDList;
#[repr(C)]
pub struct IEnumIDList;
#[repr(C)]
pub struct IEnumFullIDList;
#[repr(C)]
pub struct IFileSyncMergeHandler;
#[repr(C)]
pub struct IObjectWithFolderEnumMode;
#[repr(C)]
pub struct IParseAndCreateItem;
#[repr(C)]
pub struct IShellFolder;
#[repr(C)]
pub struct IEnumExtraSearch;
#[repr(C)]
pub struct IShellFolder2;
#[repr(C)]
pub struct IFolderViewOptions;
#[repr(C)]
pub struct IShellView;
#[repr(C)]
pub struct IShellView2;
#[repr(C)]
pub struct IShellView3;
#[repr(C)]
pub struct IFolderView;
#[repr(C)]
pub struct ISearchBoxInfo;
#[repr(C)]
pub struct IFolderView2;
#[repr(C)]
pub struct IFolderViewSettings;
#[repr(C)]
pub struct IPreviewHandlerVisuals;
#[repr(C)]
pub struct IVisualProperties;
#[repr(C)]
pub struct ICommDlgBrowser;
#[repr(C)]
pub struct ICommDlgBrowser2;
#[repr(C)]
pub struct ICommDlgBrowser3;
#[repr(C)]
pub struct IColumnManager;
#[repr(C)]
pub struct IFolderFilterSite;
#[repr(C)]
pub struct IFolderFilter;
#[repr(C)]
pub struct IInputObjectSite;
#[repr(C)]
pub struct IInputObject;
#[repr(C)]
pub struct IInputObject2;
#[repr(C)]
pub struct IShellIcon;
#[repr(C)]
pub struct IShellBrowser;
#[repr(C)]
pub struct IProfferService;
#[repr(C)]
pub struct IShellItem;
#[repr(C)]
pub struct IShellItem2;
#[repr(C)]
pub struct IShellItemImageFactory;
#[repr(C)]
pub struct IUserAccountChangeCallback;
#[repr(C)]
pub struct IEnumShellItems;
#[repr(C)]
pub struct ITransferAdviseSink;
#[repr(C)]
pub struct ITransferSource;
#[repr(C)]
pub struct IEnumResources;
#[repr(C)]
pub struct IShellItemResources;
#[repr(C)]
pub struct ITransferDestination;
#[repr(C)]
pub struct IStreamAsync;
#[repr(C)]
pub struct IStreamUnbufferedInfo;
#[repr(C)]
pub struct IFileOperationProgressSink;
#[repr(C)]
pub struct IShellItemArray;
#[repr(C)]
pub struct IInitializeWithItem;
#[repr(C)]
pub struct IObjectWithSelection;
#[repr(C)]
pub struct IObjectWithBackReferences;
#[repr(C)]
pub struct IPropertyUI;
#[repr(C)]
pub struct ICategoryProvider;
#[repr(C)]
pub struct ICategorizer;
#[repr(C)]
pub struct IDropTargetHelper;
#[repr(C)]
pub struct IDragSourceHelper;
#[repr(C)]
pub struct IDragSourceHelper2;
#[repr(C)]
pub struct IShellLinkA;
#[repr(C)]
pub struct IShellLinkW;
#[repr(C)]
pub struct IShellLinkDataList;
#[repr(C)]
pub struct IResolveShellLink;
#[repr(C)]
pub struct IActionProgressDialog;
#[repr(C)]
pub struct IHWEventHandler;
#[repr(C)]
pub struct IHWEventHandler2;
#[repr(C)]
pub struct IQueryCancelAutoPlay;
#[repr(C)]
pub struct IDynamicHWHandler;
#[repr(C)]
pub struct IActionProgress;
#[repr(C)]
pub struct IShellExtInit;
#[repr(C)]
pub struct IShellPropSheetExt;
#[repr(C)]
pub struct IRemoteComputer;
#[repr(C)]
pub struct IQueryContinue;
#[repr(C)]
pub struct IObjectWithCancelEvent;
#[repr(C)]
pub struct IUserNotification;
#[repr(C)]
pub struct IUserNotificationCallback;
#[repr(C)]
pub struct IUserNotification2;
#[repr(C)]
pub struct IItemNameLimits;
#[repr(C)]
pub struct ISearchFolderItemFactory;
#[repr(C)]
pub struct IExtractImage;
#[repr(C)]
pub struct IExtractImage2;
#[repr(C)]
pub struct IThumbnailHandlerFactory;
#[repr(C)]
pub struct IParentAndItem;
#[repr(C)]
pub struct IDockingWindow;
#[repr(C)]
pub struct IDeskBand;
#[repr(C)]
pub struct IDeskBandInfo;
#[repr(C)]
pub struct IDeskBand2;
#[repr(C)]
pub struct ITaskbarList;
#[repr(C)]
pub struct ITaskbarList2;
#[repr(C)]
pub struct ITaskbarList3;
#[repr(C)]
pub struct ITaskbarList4;
#[repr(C)]
pub struct IStartMenuPinnedList;
#[repr(C)]
pub struct ICDBurn;
#[repr(C)]
pub struct IWizardSite;
#[repr(C)]
pub struct IWizardExtension;
#[repr(C)]
pub struct IWebWizardExtension;
#[repr(C)]
pub struct IPublishingWizard;
#[repr(C)]
pub struct IFolderViewHost;
#[repr(C)]
pub struct IExplorerBrowserEvents;
#[repr(C)]
pub struct IExplorerBrowser;
#[repr(C)]
pub struct IAccessibleObject;
#[repr(C)]
pub struct IResultsFolder;
#[repr(C)]
pub struct IEnumObjects;
#[repr(C)]
pub struct IOperationsProgressDialog;
#[repr(C)]
pub struct IIOCancelInformation;
#[repr(C)]
pub struct IFileOperation;
#[repr(C)]
pub struct IObjectProvider;
#[repr(C)]
pub struct INamespaceWalkCB;
#[repr(C)]
pub struct INamespaceWalkCB2;
#[repr(C)]
pub struct INamespaceWalk;
#[repr(C)]
pub struct IAutoCompleteDropDown;
#[repr(C)]
pub struct IBandSite;
#[repr(C)]
pub struct IModalWindow;
#[repr(C)]
pub struct ICDBurnExt;
#[repr(C)]
pub struct IContextMenuSite;
#[repr(C)]
pub struct IEnumReadyCallback;
#[repr(C)]
pub struct IEnumerableView;
#[repr(C)]
pub struct IInsertItem;
#[repr(C)]
pub struct IMenuBand;
#[repr(C)]
pub struct IFolderBandPriv;
#[repr(C)]
pub struct IRegTreeItem;
#[repr(C)]
pub struct IImageRecompress;
#[repr(C)]
pub struct IDeskBar;
#[repr(C)]
pub struct IMenuPopup;
#[repr(C)]
pub struct IFileIsInUse;
#[repr(C)]
pub struct IFileDialogEvents;
#[repr(C)]
pub struct IFileDialog;
#[repr(C)]
pub struct IFileSaveDialog;
#[repr(C)]
pub struct IFileOpenDialog;
#[repr(C)]
pub struct IFileDialogCustomize;
#[repr(C)]
pub struct IFileDialogControlEvents;
#[repr(C)]
pub struct IFileDialog2;
#[repr(C)]
pub struct IApplicationAssociationRegistration;
#[repr(C)]
pub struct IApplicationAssociationRegistrationUI;
#[repr(C)]
pub struct IDelegateFolder;
#[repr(C)]
pub struct IBrowserFrameOptions;
#[repr(C)]
pub struct INewWindowManager;
#[repr(C)]
pub struct IAttachmentExecute;
#[repr(C)]
pub struct IShellMenuCallback;
#[repr(C)]
pub struct IShellMenu;
#[repr(C)]
pub struct IShellRunDll;
#[repr(C)]
pub struct IKnownFolder;
#[repr(C)]
pub struct IKnownFolderManager;
#[repr(C)]
pub struct ISharingConfigurationManager;
#[repr(C)]
pub struct IPreviousVersionsInfo;
#[repr(C)]
pub struct IRelatedItem;
#[repr(C)]
pub struct IIdentityName;
#[repr(C)]
pub struct IDelegateItem;
#[repr(C)]
pub struct ICurrentItem;
#[repr(C)]
pub struct ITransferMediumItem;
#[repr(C)]
pub struct IUseToBrowseItem;
#[repr(C)]
pub struct IDisplayItem;
#[repr(C)]
pub struct IViewStateIdentityItem;
#[repr(C)]
pub struct IPreviewItem;
#[repr(C)]
pub struct IDestinationStreamFactory;
#[repr(C)]
pub struct INewMenuClient;
#[repr(C)]
pub struct IInitializeWithBindCtx;
#[repr(C)]
pub struct IShellItemFilter;
#[repr(C)]
pub struct INameSpaceTreeControl;
#[repr(C)]
pub struct INameSpaceTreeControl2;
#[repr(C)]
pub struct INameSpaceTreeControlEvents;
#[repr(C)]
pub struct INameSpaceTreeControlDropHandler;
#[repr(C)]
pub struct INameSpaceTreeAccessible;
#[repr(C)]
pub struct INameSpaceTreeControlCustomDraw;
#[repr(C)]
pub struct INameSpaceTreeControlFolderCapabilities;
#[repr(C)]
pub struct IPreviewHandler;
#[repr(C)]
pub struct IPreviewHandlerFrame;
#[repr(C)]
pub struct ITrayDeskBand;
#[repr(C)]
pub struct IBandHost;
#[repr(C)]
pub struct IExplorerPaneVisibility;
#[repr(C)]
pub struct IContextMenuCB;
#[repr(C)]
pub struct IDefaultExtractIconInit;
#[repr(C)]
pub struct IExplorerCommand;
#[repr(C)]
pub struct IExplorerCommandState;
#[repr(C)]
pub struct IInitializeCommand;
#[repr(C)]
pub struct IEnumExplorerCommand;
#[repr(C)]
pub struct IExplorerCommandProvider;
#[repr(C)]
pub struct IMarkupCallback;
#[repr(C)]
pub struct IControlMarkup;
#[repr(C)]
pub struct IInitializeNetworkFolder;
#[repr(C)]
pub struct IOpenControlPanel;
#[repr(C)]
pub struct IComputerInfoChangeNotify;
#[repr(C)]
pub struct IFileSystemBindData;
#[repr(C)]
pub struct IFileSystemBindData2;
#[repr(C)]
pub struct ICustomDestinationList;
#[repr(C)]
pub struct IApplicationDestinations;
#[repr(C)]
pub struct IApplicationDocumentLists;
#[repr(C)]
pub struct IObjectWithAppUserModelID;
#[repr(C)]
pub struct IObjectWithProgID;
#[repr(C)]
pub struct IUpdateIDList;
#[repr(C)]
pub struct IDesktopGadget;
#[repr(C)]
pub struct IDesktopWallpaper;
#[repr(C)]
pub struct IHomeGroup;
#[repr(C)]
pub struct IInitializeWithPropertyStore;
#[repr(C)]
pub struct IOpenSearchSource;
#[repr(C)]
pub struct IShellLibrary;
#[repr(C)]
pub struct IDefaultFolderMenuInitialize;
#[repr(C)]
pub struct IApplicationActivationManager;
#[repr(C)]
pub struct IAssocHandlerInvoker;
#[repr(C)]
pub struct IAssocHandler;
#[repr(C)]
pub struct IEnumAssocHandlers;
#[repr(C)]
pub struct IDataObjectProvider;
#[repr(C)]
pub struct IDataTransferManagerInterop;
#[repr(C)]
pub struct IFrameworkInputPaneHandler;
#[repr(C)]
pub struct IFrameworkInputPane;
#[repr(C)]
pub struct IAccessibilityDockingServiceCallback;
#[repr(C)]
pub struct IAccessibilityDockingService;
#[repr(C)]
pub struct IAppVisibilityEvents;
#[repr(C)]
pub struct IAppVisibility;
#[repr(C)]
pub struct IPackageExecutionStateChangeNotification;
#[repr(C)]
pub struct IPackageDebugSettings;
#[repr(C)]
pub struct ISuspensionDependencyManager;
#[repr(C)]
pub struct IExecuteCommandApplicationHostEnvironment;
#[repr(C)]
pub struct IExecuteCommandHost;
#[repr(C)]
pub struct IApplicationDesignModeSettings;
#[repr(C)]
pub struct IApplicationDesignModeSettings2;
#[repr(C)]
pub struct ILaunchTargetMonitor;
#[repr(C)]
pub struct ILaunchSourceViewSizePreference;
#[repr(C)]
pub struct ILaunchTargetViewSizePreference;
#[repr(C)]
pub struct ILaunchSourceAppUserModelId;
#[repr(C)]
pub struct IInitializeWithWindow;
#[repr(C)]
pub struct IHandlerInfo;
#[repr(C)]
pub struct IHandlerActivationHost;
#[repr(C)]
pub struct IContactManagerInterop;
// shtypes.h
#[repr(C)]
pub struct SHITEMID {
    pub cb: USHORT,
    pub abID: [BYTE, ..1],
}
pub type LPSHITEMID = *mut SHITEMID;
pub type LPCSHITEMID = *const SHITEMID;
#[repr(C)]
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
// objldlbase.h
// this ALWAYS GENERATED file contains the definitions for the interfaces
//-------------------------------------------------------------------------------------------------
#[repr(C)]
pub struct IMallocVtbl {
    pub QueryInterface: extern "system" fn(
        This: *mut IMalloc,
        riid: REFIID,
        ppvObject: *mut *mut c_void,
    ) -> HRESULT,
    pub AddRef: extern "system" fn(
        This: *mut IMalloc,
    ) -> ULONG,
    pub Release: extern "system" fn(
        This: *mut IMalloc,
    ) -> ULONG,
    pub Alloc: extern "system" fn(
        This: *mut IMalloc,
        cb: SIZE_T,
    ) -> *mut c_void,
    pub Realloc: extern "system" fn(
        This: *mut IMalloc,
        pv: *mut c_void,
        cb: SIZE_T,
    ) -> *mut c_void,
    pub Free: extern "system" fn(
        This: *mut IMalloc,
        pv: *mut c_void,
    ),
    pub GetSize: extern "system" fn(
        This: *mut IMalloc,
        pv: *mut c_void,
    ) -> SIZE_T,
    pub DidAlloc: extern "system" fn(
        This: *mut IMalloc,
        pv: *mut c_void,
    ) -> int,
    pub HeapMinimize: extern "system" fn(
        This: *mut IMalloc,
    ),
}
#[repr(C)]
pub struct IMalloc {
    pub lpVtbl: *const IMallocVtbl,
}
pub type LPMALLOC = *mut IMalloc;
#[repr(C)]
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
pub struct IStreamVtbl {
    pub QueryInterface: extern "system" fn(
        This: *mut IStream,
        riid: REFIID,
        ppvObject: *mut *mut c_void,
    ) -> HRESULT,
    pub AddRef: extern "system" fn(
        This: *mut IStream,
    ) -> ULONG,
    pub Release: extern "system" fn(
        This: *mut IStream,
    ) -> ULONG,
    pub Read: extern "system" fn(
        This: *mut IStream,
        pv: *mut c_void,
        cb: ULONG,
        pcbRead: *mut ULONG,
    ) -> HRESULT,
    pub Write: extern "system" fn(
        This: *mut IStream,
        pv: *const c_void,
        cb: ULONG,
        pcbWritten: *mut ULONG,
    ) -> HRESULT,
    pub Seek: extern "system" fn(
        This: *mut IStream,
        dlibMove: LARGE_INTEGER,
        dwOrigin: DWORD,
        plibNewPosition: ULARGE_INTEGER,
    ) -> HRESULT,
    pub SetSize: extern "system" fn(
        This: *mut IStream,
        libNewSize: ULARGE_INTEGER,
    ) -> HRESULT,
    pub CopyTo: extern "system" fn(
        This: *mut IStream,
        cb: ULARGE_INTEGER,
        pcbRead: *mut ULARGE_INTEGER,
        pcbWritten: *mut ULARGE_INTEGER,
    ) -> HRESULT,
    pub Commit: extern "system" fn(
        This: *mut IStream,
        grfCommitFlags: DWORD,
    ) -> HRESULT,
    pub Revert: extern "system" fn(
        This: *mut IStream,
    ) -> HRESULT,
    pub LockRegion: extern "system" fn(
        This: *mut IStream,
        libOffset: ULARGE_INTEGER,
        cb: ULARGE_INTEGER,
        dwLockType: DWORD,
    ) -> HRESULT,
    pub UnlockRegion: extern "system" fn(
        This: *mut IStream,
        libOffset: ULARGE_INTEGER,
        cb: ULARGE_INTEGER,
        dwLockType: DWORD,
    ) -> HRESULT,
    pub Stat: extern "system" fn(
        This: *mut IStream,
        pstatstg: *mut STATSTG,
        grfStatFlag: DWORD,
    ) -> HRESULT,
    pub Clone: extern "system" fn(
        This: *mut IStream,
        ppstm: *mut *mut IStream,
    ) -> HRESULT,
}
#[repr(C)]
pub struct IStream {
    pub lpVtbl: *const IStreamVtbl,
}
pub type LPSTREAM = *mut IStream;
#[repr(C)]
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
pub struct ServerInformation {
    dwServerPid: DWORD,
    dwServerTid: DWORD,
    ui64ServerAddress: UINT64,
}
pub type PServerInformation = *mut ServerInformation;
#[repr(C)]
pub struct CO_MTA_USAGE_COOKIE__;
pub type CO_MTA_USAGE_COOKIE = *mut CO_MTA_USAGE_COOKIE__;

//-------------------------------------------------------------------------------------------------
// unknwnbase.h
// this ALWAYS GENERATED file contains the definitions for the interfaces
//-------------------------------------------------------------------------------------------------
#[repr(C)]
pub struct IUnknownVtbl {
    pub QueryInterface: extern "system" fn(
        This: *mut IUnknown,
        riid: REFIID,
        ppvObject: *mut *mut c_void,
    ) -> HRESULT,
    pub AddRef: extern "system" fn(
        This: *mut IUnknown,
    ) -> ULONG,
    pub Release: extern "system" fn(
        This: *mut IUnknown,
    ) -> ULONG,
}
#[repr(C)]
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

// old stuff
#[repr(C)]
pub struct CONSOLE_READCONSOLE_CONTROL {
    pub nLength: ULONG,
    pub nInitialChars: ULONG,
    pub dwCtrlWakeupMask: ULONG,
    pub dwControlKeyState: ULONG,
}
pub type PCONSOLE_READCONSOLE_CONTROL = *mut CONSOLE_READCONSOLE_CONTROL;
#[repr(C)]
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
pub static INVALID_HANDLE_VALUE: HANDLE = -1 as HANDLE;
// error codes
pub static ERROR_INVALID_HANDLE: DWORD = 6;
pub static ERROR_ILLEGAL_CHARACTER: DWORD = 582;
// console input flags
pub static ENABLE_PROCESSED_INPUT: DWORD = 0x1;
pub static ENABLE_LINE_INPUT: DWORD = 0x2;
pub static ENABLE_ECHO_INPUT: DWORD = 0x4;
pub static ENABLE_WINDOW_INPUT: DWORD = 0x8;
pub static ENABLE_MOUSE_INPUT: DWORD = 0x10;
pub static ENABLE_INSERT_MODE: DWORD = 0x20;
pub static ENABLE_QUICK_EDIT_MODE: DWORD = 0x40;
pub static ENABLE_EXTENDED_FLAGS: DWORD = 0x80;
pub static ENABLE_AUTO_POSITION: DWORD = 0x100;
//console output flags
pub static ENABLE_PROCESSED_OUTPUT: DWORD = 0x1;
pub static ENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x2;

//-------------------------------------------------------------------------------------------------
// Functions
//-------------------------------------------------------------------------------------------------

#[cfg(feature = "ole32")]
#[link(name = "ole32")]
extern "system" {
    pub fn CoAllowUnmarshalerCLSID(
        clsid: REFCLSID,
    ) -> HRESULT;
    pub fn CoDecodeProxy(
        dwClientPid: DWORD,
        ui64ProxyAddress: UINT64,
        pServerInformation: PServerInformation,
    ) -> HRESULT;
    pub fn CoDecrementMTAUsage(
        Cookie: CO_MTA_USAGE_COOKIE,
    ) -> HRESULT;
    pub fn CoGetApartmentType(
        pAptType: *mut APTTYPE,
        pAptQualifier: *mut APTTYPEQUALIFIER,
    ) -> HRESULT;
    pub fn CoGetCallerTID(
        lpdwTID: LPDWORD,
    ) -> HRESULT;
    pub fn CoGetClassObject(
        rclsid: REFCLSID,
        dwClsContext: DWORD,
        pvReserved: LPVOID,
        riid: REFIID,
        ppv: *mut LPVOID,
    ) -> HRESULT;
    pub fn CoGetContextToken(
        pToken: *mut ULONG_PTR,
    ) -> HRESULT;
    pub fn CoGetCurrentLogicalThreadId(
        pguid: *mut GUID,
    ) -> HRESULT;
    pub fn CoGetCurrentProcess() -> DWORD;
    pub fn CoGetDefaultContext(
        aptType: APTTYPE,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT;
    pub fn CoGetMalloc(
        dwMemContext: DWORD,
        ppMalloc: *mut LPMALLOC,
    ) -> HRESULT;
    pub fn CoGetObjectContext(
        riid: REFIID,
        ppv: *mut LPVOID,
    ) -> HRESULT;
    pub fn CoIncrementMTAUsage(
        pCookie: *mut CO_MTA_USAGE_COOKIE,
    ) -> HRESULT;
    pub fn CoInitializeEx(
        pvReserved: LPVOID,
        dwCoInit: DWORD,
    ) -> HRESULT;
    pub fn CoRegisterClassObject(
        rclsid: REFCLSID,
        pUnk: LPUNKNOWN,
        dwClsContext: DWORD,
        flags: DWORD,
        lpdwRegister: LPDWORD,
    ) -> HRESULT;
    pub fn CoRevokeClassObject(
    ) -> HRESULT;
    pub fn CoUninitialize();
    pub fn CreateStreamOnHGlobal(
        hGlobal: HGLOBAL,
        fDeleteOnRelease: BOOL,
        ppstm: *mut LPSTREAM,
    ) -> HRESULT;
    pub fn GetHGlobalFromStream(
        pstm: LPSTREAM,
        phglobal: *mut HGLOBAL,
    ) -> HRESULT;
}
#[cfg(feature = "shell32")]
#[link(name = "shell32")]
extern "system" {
    pub fn SHCloneSpecialIDList(
        hwnd: HWND,
        csidl: c_int,
        fCreate: BOOL,
    ) -> PIDLIST_ABSOLUTE;
    pub fn SHCreateDirectory(
        hwnd: HWND,
        pszPath: PCWSTR,
    ) -> c_int;
    pub fn SHCreateDirectoryExA(
        hwnd: HWND,
        pszPath: LPCSTR,
        psa: *const SECURITY_ATTRIBUTES,
    ) -> c_int;
    pub fn SHCreateDirectoryExW(
        hwnd: HWND,
        pszPath: LPCWSTR,
        psa: *const SECURITY_ATTRIBUTES,
    ) -> c_int;
    pub fn SHCreateShellItem(
        pidlParent: PCIDLIST_ABSOLUTE,
        psfParent: *mut IShellFolder,
        pidl: PCUITEMID_CHILD,
        ppsi: *mut *mut IShellItem,
    ) -> HRESULT;
    pub fn SHFlushSFCache();
    pub fn SHGetFolderLocation(
        hwnd: HWND,
        csidl: c_int,
        hToken: HANDLE,
        dwFlags: DWORD,
        ppidl: *mut PIDLIST_ABSOLUTE,
    ) -> HRESULT;
    pub fn SHGetFolderPathA(
        hwnd: HWND,
        csidl: c_int,
        hToken: HANDLE,
        dwFlags: DWORD,
        pszPath: LPSTR,
    ) -> HRESULT;
    pub fn SHGetFolderPathAndSubDirA(
        hwnd: HWND,
        csidl: c_int,
        hToken: HANDLE,
        dwFlags: DWORD,
        pszSubDir: LPCSTR,
        pszPath: LPSTR,
    ) -> HRESULT;
    pub fn SHGetFolderPathAndSubDirW(
        hwnd: HWND,
        csidl: c_int,
        hToken: HANDLE,
        dwFlags: DWORD,
        pszSubDir: LPCWSTR,
        pszPath: LPWSTR,
    ) -> HRESULT;
    pub fn SHGetFolderPathW(
        hwnd: HWND,
        csidl: c_int,
        hToken: HANDLE,
        dwFlags: DWORD,
        pszPath: LPWSTR,
    ) -> HRESULT;
    pub fn SHGetIconOverlayIndexA(
        pszIconPath: LPCSTR,
        iIconIndex: c_int
    ) -> c_int;
    pub fn SHGetIconOverlayIndexW(
        pszIconPath: LPCWSTR,
        iIconIndex: c_int
    ) -> c_int;
    pub fn SHGetKnownFolderIDList(
        rfid: REFKNOWNFOLDERID,
        dwFlags: DWORD,
        hToken: HANDLE,
        ppidl: *mut PIDLIST_ABSOLUTE,
    ) -> HRESULT;
    pub fn SHGetKnownFolderItem(
        rfid: REFKNOWNFOLDERID,
        flags: KNOWN_FOLDER_FLAG,
        hToken: HANDLE,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT;
    pub fn SHGetKnownFolderPath(
        rfid: REFKNOWNFOLDERID,
        dwFlags: DWORD,
        hToken: HANDLE,
        pszPath: *mut PWSTR,
    ) -> HRESULT;
    pub fn SHGetPathFromIDListA(
        pidl: PCIDLIST_ABSOLUTE,
        pszPath: LPSTR,
    ) -> BOOL;
    pub fn SHGetPathFromIDListEx(
        pidl: PCIDLIST_ABSOLUTE,
        pszPath: PWSTR,
        cchPath: DWORD,
        uOpts: GPFIDL_FLAGS,
    ) -> BOOL;
    pub fn SHGetPathFromIDListW(
        pidl: PCIDLIST_ABSOLUTE,
        pszPath: LPWSTR,
    ) -> BOOL;
    pub fn SHGetSpecialFolderLocation(
        hwnd: HWND,
        csidl: c_int,
        ppidl: *mut PIDLIST_ABSOLUTE,
    ) -> HRESULT;
    pub fn SHGetSpecialFolderPathA(
        hwnd: HWND,
        pszPath: LPSTR,
        csidl: c_int,
        fCreate: BOOL,
    ) -> BOOL;
    pub fn SHGetSpecialFolderPathW(
        hwnd: HWND,
        pszPath: LPWSTR,
        csidl: c_int,
        fCreate: BOOL,
    ) -> BOOL;
    pub fn SHOpenFolderAndSelectItems(
        pidlFolder: PCIDLIST_ABSOLUTE,
        cidl: UINT,
        apidl: PCUITEMID_CHILD_ARRAY,
        dwFlags: DWORD
    ) -> HRESULT;
    pub fn SHSetFolderPathA(
        csidl: c_int,
        hToken: HANDLE,
        dwFlags: DWORD,
        pszPath: LPCSTR,
    ) -> HRESULT;
    pub fn SHSetFolderPathW(
        csidl: c_int,
        hToken: HANDLE,
        dwFlags: DWORD,
        pszPath: LPCWSTR,
    ) -> HRESULT;
    pub fn SHSetKnownFolderPath(
        rfid: REFKNOWNFOLDERID,
        dwFlags: DWORD,
        hToken: HANDLE,
        pszPath: PCWSTR,
    ) -> HRESULT;
}
#[cfg(feature = "kernel32")]
#[link(name = "kernel32")]
extern "system" {
    pub fn CloseHandle(
        hObject: HANDLE,
    ) -> BOOL;
    pub fn CreateIoCompletionPort(
        FileHandle: HANDLE,
        ExistingCompletionPort: HANDLE,
        CompletionKey: ULONG_PTR,
        NumberOfConcurrentThreads: DWORD,
    ) -> HANDLE;
    pub fn GetConsoleMode(
        hConsoleHandle: HANDLE,
        lpMode: LPDWORD,
    ) -> BOOL;
    pub fn GetLastError() -> DWORD;
    pub fn GetProcessTimes(
        hProcess: HANDLE,
        lpCreationTime: LPFILETIME,
        lpExitTime: LPFILETIME,
        lpKernelTime: LPFILETIME,
        lpUserTime: LPFILETIME,
    ) -> BOOL;
    pub fn K32GetProcessMemoryInfo(
        Process: HANDLE,
        ppsmemCounters: PPROCESS_MEMORY_COUNTERS,
        cb: DWORD,
    ) -> BOOL;
    pub fn OpenProcess(
        dwDesiredAccess: DWORD,
        bInheritHandle: BOOL,
        dwProcessId: DWORD,
    ) -> HANDLE;
    pub fn ReadConsoleW(
        hConsoleInput: HANDLE,
        lpBuffer: LPVOID,
        nNumberOfCharsToRead: DWORD,
        lpNumberOfCharsRead: LPDWORD,
        pInputControl: PCONSOLE_READCONSOLE_CONTROL,
    ) -> BOOL;
    pub fn ReadProcessMemory(
        hProcess: HANDLE,
        lpBaseAddress: LPCVOID,
        lpBuffer: LPVOID,
        nSize: SIZE_T,
        lpNumberOfBytesRead: *mut SIZE_T,
    ) -> BOOL;
    pub fn SetConsoleMode(
        hConsoleHandle: HANDLE,
        lpMode: DWORD,
    ) -> BOOL;
    pub fn WriteConsoleW(
        hConsoleOutput: HANDLE,
        lpBuffer: LPCVOID,
        nNumberOfCharsToWrite: DWORD,
        lpNumberOfCharsWritten: LPDWORD,
        lpReserved: LPVOID,
    ) -> BOOL;
    pub fn WriteProcessMemory(
        hProcess: HANDLE,
        lpBaseAddress: LPVOID,
        lpBuffer: LPCVOID,
        nSize: SIZE_T,
        lpNumberOfBytesWritten: *mut SIZE_T,
    ) -> BOOL;
}
#[cfg(feature = "advapi32")]
#[link(name = "advapi32")]
extern "system" {
    pub fn AdjustTokenPrivileges(
        TokenHandle: HANDLE,
        DisableAllPrivileges: BOOL,
        NewState: PTOKEN_PRIVILEGES,
        BufferLength: DWORD,
        PreviousState: PTOKEN_PRIVILEGES,
        ReturnLength: PDWORD,
    ) -> BOOL;
}
#[cfg(feature = "winmm")]
#[link(name = "winmm")]
extern "system" {
    pub fn PlaySoundA(
        pszSound: LPCSTR,
        hmod: HMODULE,
        fdwSound: DWORD,
    ) -> BOOL;
    pub fn PlaySoundW(
        pszSound: LPCWSTR,
        hmod: HMODULE,
        fdwSound: DWORD,
    ) -> BOOL;
    pub fn sndPlaySoundA(
        pszSound: LPCSTR,
        fuSound: UINT,
    ) -> BOOL;
    pub fn sndPlaySoundW(
        pszSound: LPCWSTR,
        fuSound: UINT,
    ) -> BOOL;
}

// Copyright © 2014, Peter Atashian

#![feature(globs)]
#![no_std]
#![allow(non_camel_case_types, non_snake_case)]

extern crate libc;

pub mod types {
    use libc::{
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
        c_float,
        wchar_t,
    };
    // Primitive types
    pub type __int64 = i64; // __int64, signed __int64
    pub type __uint64 = u64; // unsigned __int64
    // minwindef.h
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
    pub struct HKEY__;
    pub type HKEY = *mut HKEY__;
    pub type PHKEY = *mut HKEY;
    pub struct HMETAFILE__;
    pub type HMETAFILE = *mut HMETAFILE__;
    pub struct HINSTANCE__;
    pub type HINSTANCE = *mut HINSTANCE__;
    pub type HMODULE = HINSTANCE;
    pub struct HRGN__;
    pub type HRGN = *mut HRGN__;
    pub struct HRSRC__;
    pub type HRSRC = *mut HRSRC__;
    pub struct HSPRITE__;
    pub type HSPRITE = *mut HSPRITE__;
    pub struct HLSURF__;
    pub type HLSURF = *mut HLSURF__;
    pub struct HSTR__;
    pub type HSTR = *mut HSTR__;
    pub struct HTASK__;
    pub type HTASK = *mut HTASK__;
    pub struct HWINSTA__;
    pub type HWINSTA = *mut HWINSTA__;
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
    // winnt.h
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
    pub type PLUID= *mut LUID;
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
    // Stopped partway through winnt.h at #include <guiddef.h>

    //structs
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
}
pub mod consts {
    use types::*;
    // minwindef.h
    pub const MAX_PATH: uint = 260;
    // constants
    pub static TRUE: BOOL = 1;
    pub static FALSE: BOOL = 0;
    pub static INVALID_HANDLE_VALUE: HANDLE = -1 as HANDLE;
    pub static PROCESS_QUERY_INFORMATION: DWORD = 0x400;
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
}
pub mod funcs {
    use types::*;
    #[link(name = "kernel32")]
    #[link(name = "psapi")]
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
        pub fn GetProcessMemoryInfo(
            Process: HANDLE,
            ppsmemCounters: PPROCESS_MEMORY_COUNTERS,
            cb: DWORD,
        ) -> BOOL;
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
    }
}

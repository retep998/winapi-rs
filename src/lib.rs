// Copyright © 2014, Peter Atashian

#![feature(globs)]
#![no_std]
#![allow(non_camel_case_types, non_snake_case)]

extern crate libc;

pub mod types {
    pub use libc::c_void;
    // Primitive types
    pub type c_char = i8;
    pub type c_schar = i8;
    pub type c_uchar = u8;
    pub type c_short = i16; // short, signed short, short int, signed short int
    pub type c_ushort = u16; // unsigned short, unsigned short int
    pub type c_int = i32; // int, signed int
    pub type c_uint = u32; // unsigned int
    pub type c_long = i32; // long, signed long, long int, signed long int
    pub type c_ulong = u32; // unsigned long, unsigned long int
    pub type c_longlong = i64;
    pub type c_ulonglong = u64;
    pub type __int64 = i64; // __int64, signed __int64
    pub type __uint64 = u64; // unsigned __int64
    pub type wchar_t = u16; // wchar_t
    pub type c_float = f32; // float
    pub type c_double = f64; // double
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
    // Stopped partway through winnt.h at #include <guiddef.h>
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
}
pub mod consts {
    use types::*;
    // minwindef.h
    pub const MAX_PATH: uint = 260;
    // Shlobj.h
    pub const IDO_SHGIOI_SHARE: c_int = 0x0FFFFFFF;
    pub const IDO_SHGIOI_LINK: c_int = 0x0FFFFFFE;
    // FIXME - These two constants need to be updated when a bug is resolved
    // https://connect.microsoft.com/VisualStudio/feedback/details/1021022/mismatch-in-value-of-constant-between-header-and-documentation
    pub const IDO_SHGIOI_SLOWFILE: c_int = 0x0FFFFFFD;
    pub const IDO_SHGIOI_DEFAULT: c_int = 0x0FFFFFFC;
    pub const GPFIDL_DEFAULT: GPFIDL_FLAGS = 0x0000;
    pub const GPFIDL_ALTNAME: GPFIDL_FLAGS = 0x0001;
    pub const GPFIDL_UNCPRINTER: GPFIDL_FLAGS = 0x0002;
    pub const OFASI_EDIT: DWORD = 0x0001;
    pub const OFASI_OPENDESKTOP: DWORD = 0x0002;
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
            ppidl: PIDLIST_ABSOLUTE,
        ) -> HRESULT;
        pub fn SHGetFolderPathA(
            hwnd: HWND,
            csidl: c_int,
            hToken: HANDLE,
            dwFlags: DWORD,
            pszPath: LPSTR,
        ) -> HRESULT;
        pub fn SHGetFolderPathW(
            hwnd: HWND,
            csidl: c_int,
            hToken: HANDLE,
            dwFlags: DWORD,
            pszPath: LPWSTR,
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
            pszPath: PWSTR,
        ) -> HRESULT;
        pub fn SHGetPathFromIDListA(
            pidl: PCIDLIST_ABSOLUTE,
            pszPath: LPSTR,
        ) -> BOOL;
        pub fn SHGetPathFromIDListW(
            pidl: PCIDLIST_ABSOLUTE,
            pszPath: LPWSTR,
        ) -> BOOL;
        pub fn SHGetPathFromIDListEx(
            pidl: PCIDLIST_ABSOLUTE,
            pszPath: PWSTR,
            cchPath: DWORD,
            uOpts: GPFIDL_FLAGS,
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

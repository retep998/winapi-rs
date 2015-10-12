// Copyright © 2015, skdltmxn
// Licensed under the MIT License <LICENSE.md>
//! Data Protection API Prototypes and Definitions
// This header file provides the definitions and symbols necessary for an
// Application or Smart Card Service Provider to access the Smartcard Subsystem.
pub type LPCBYTE = *const ::BYTE;
pub type SCARDCONTEXT = ::ULONG_PTR;
pub type PSCARDCONTEXT = *mut SCARDCONTEXT;
pub type LPSCARDCONTEXT = *mut SCARDCONTEXT;
pub type SCARDHANDLE = ::ULONG_PTR;
pub type PSCARDHANDLE = *mut SCARDHANDLE;
pub type LPSCARDHANDLE = *mut SCARDHANDLE;
pub const SCARD_AUTOALLOCATE: ::DWORD = -1i32 as ::DWORD;
pub const SCARD_SCOPE_USER: ::DWORD = 0;
pub const SCARD_SCOPE_TERMINAL: ::DWORD = 1;
pub const SCARD_SCOPE_SYSTEM: ::DWORD = 2;
pub const SCARD_PROVIDER_PRIMARY: ::DWORD = 1;
pub const SCARD_PROVIDER_CSP: ::DWORD = 2;
pub const SCARD_PROVIDER_KSP: ::DWORD = 3;
#[repr(C)] #[derive(Copy)]
pub struct SCARD_READERSTATEA {
    pub szReader: ::LPCSTR,
    pub pvUserData: ::LPVOID,
    pub dwCurrentState: ::DWORD,
    pub dwEventState: ::DWORD,
    pub cbAtr: ::DWORD,
    pub rgbAtr: [::BYTE; 36],
}
impl Clone for SCARD_READERSTATEA { fn clone(&self) -> SCARD_READERSTATEA { *self } }
pub type PSCARD_READERSTATEA = *mut SCARD_READERSTATEA;
pub type LPSCARD_READERSTATEA = *mut SCARD_READERSTATEA;
#[repr(C)] #[derive(Copy)]
pub struct SCARD_READERSTATEW {
    pub szReader: ::LPCWSTR,
    pub pvUserData: ::LPVOID,
    pub dwCurrentState: ::DWORD,
    pub dwEventState: ::DWORD,
    pub cbAtr: ::DWORD,
    pub rgbAtr: [::BYTE; 36],
}
impl Clone for SCARD_READERSTATEW { fn clone(&self) -> SCARD_READERSTATEW { *self } }
pub type PSCARD_READERSTATEW = *mut SCARD_READERSTATEW;
pub type LPSCARD_READERSTATEW = *mut SCARD_READERSTATEW;
pub type SCARD_READERSTATE_A = SCARD_READERSTATEA;
pub type SCARD_READERSTATE_W = SCARD_READERSTATEW;
pub type PSCARD_READERSTATE_A = PSCARD_READERSTATEA;
pub type PSCARD_READERSTATE_W = PSCARD_READERSTATEW;
pub type LPSCARD_READERSTATE_A = LPSCARD_READERSTATEA;
pub type LPSCARD_READERSTATE_W = LPSCARD_READERSTATEW;
pub const SCARD_STATE_UNAWARE: ::DWORD = 0x00000000;
pub const SCARD_STATE_IGNORE: ::DWORD = 0x00000001;
pub const SCARD_STATE_CHANGED: ::DWORD = 0x00000002;
pub const SCARD_STATE_UNKNOWN: ::DWORD = 0x00000004;
pub const SCARD_STATE_UNAVAILABLE: ::DWORD = 0x00000008;
pub const SCARD_STATE_EMPTY: ::DWORD = 0x00000010;
pub const SCARD_STATE_PRESENT: ::DWORD = 0x00000020;
pub const SCARD_STATE_ATRMATCH: ::DWORD = 0x00000040;
pub const SCARD_STATE_EXCLUSIVE: ::DWORD = 0x00000080;
pub const SCARD_STATE_INUSE: ::DWORD = 0x00000100;
pub const SCARD_STATE_MUTE: ::DWORD = 0x00000200;
pub const SCARD_STATE_UNPOWERED: ::DWORD = 0x00000400;
#[repr(C)] #[derive(Copy)]
pub struct SCARD_ATRMASK {
    pub cbAtr: ::DWORD,
    pub rgbAtr: [::BYTE; 36],
    pub rgbMask: [::BYTE; 36],
}
impl Clone for SCARD_ATRMASK { fn clone(&self) -> SCARD_ATRMASK { *self } }
pub type PSCARD_ATRMASK = *mut SCARD_ATRMASK;
pub type LPSCARD_ATRMASK = *mut SCARD_ATRMASK;
pub const SCARD_SHARE_EXCLUSIVE: ::DWORD = 1;
pub const SCARD_SHARE_SHARED: ::DWORD = 2;
pub const SCARD_SHARE_DIRECT: ::DWORD = 3;
pub const SCARD_LEAVE_CARD: ::DWORD = 0;
pub const SCARD_RESET_CARD: ::DWORD = 1;
pub const SCARD_UNPOWER_CARD: ::DWORD = 2;
pub const SCARD_EJECT_CARD: ::DWORD = 3;
pub const SC_DLG_MINIMAL_UI: ::DWORD = 0x01;
pub const SC_DLG_NO_UI: ::DWORD = 0x02;
pub const SC_DLG_FORCE_UI: ::DWORD = 0x04;
pub const SCERR_NOCARDNAME: ::DWORD = 0x4000;
pub const SCERR_NOGUIDS: ::DWORD = 0x8000;
pub type LPOCNCONNPROCA = Option<unsafe extern "system" fn(
    SCARDCONTEXT, ::LPSTR, ::LPSTR, ::PVOID,
) -> SCARDHANDLE>;
pub type LPOCNCONNPROCW = Option<unsafe extern "system" fn(
    SCARDCONTEXT, ::LPWSTR, ::LPWSTR, ::PVOID,
) -> SCARDHANDLE>;
pub type LPOCNCHKPROC = Option<unsafe extern "system" fn(
    SCARDCONTEXT, SCARDHANDLE, ::PVOID,
) -> ::BOOL>;
pub type LPOCNDSCPROC = Option<unsafe extern "system" fn(SCARDCONTEXT, SCARDHANDLE, ::PVOID)>;
#[repr(C)] #[derive(Copy)]
pub struct OPENCARD_SEARCH_CRITERIAA {
    pub dwStructSize: ::DWORD,
    pub lpstrGroupNames: ::LPSTR,
    pub nMaxGroupNames: ::DWORD,
    pub rgguidInterfaces: ::LPCGUID,
    pub cguidInterfaces: ::DWORD,
    pub lpstrCardNames: ::LPSTR,
    pub nMaxCardNames: ::DWORD,
    pub lpfnCheck: LPOCNCHKPROC,
    pub lpfnConnect: LPOCNCONNPROCA,
    pub lpfnDisconnect: LPOCNDSCPROC,
    pub pvUserData: ::LPVOID,
    pub dwShareMode: ::DWORD,
    pub dwPreferredProtocols: ::DWORD,
}
impl Clone for OPENCARD_SEARCH_CRITERIAA { fn clone(&self) -> OPENCARD_SEARCH_CRITERIAA { *self } }
pub type POPENCARD_SEARCH_CRITERIAA = *mut OPENCARD_SEARCH_CRITERIAA;
pub type LPOPENCARD_SEARCH_CRITERIAA = *mut OPENCARD_SEARCH_CRITERIAA;
#[repr(C)] #[derive(Copy)]
pub struct OPENCARD_SEARCH_CRITERIAW {
    pub dwStructSize: ::DWORD,
    pub lpstrGroupNames: ::LPWSTR,
    pub nMaxGroupNames: ::DWORD,
    pub rgguidInterfaces: ::LPCGUID,
    pub cguidInterfaces: ::DWORD,
    pub lpstrCardNames: ::LPWSTR,
    pub nMaxCardNames: ::DWORD,
    pub lpfnCheck: LPOCNCHKPROC,
    pub lpfnConnect: LPOCNCONNPROCW,
    pub lpfnDisconnect: LPOCNDSCPROC,
    pub pvUserData: ::LPVOID,
    pub dwShareMode: ::DWORD,
    pub dwPreferredProtocols: ::DWORD,
}
impl Clone for OPENCARD_SEARCH_CRITERIAW { fn clone(&self) -> OPENCARD_SEARCH_CRITERIAW { *self } }
pub type POPENCARD_SEARCH_CRITERIAW = *mut OPENCARD_SEARCH_CRITERIAW;
pub type LPOPENCARD_SEARCH_CRITERIAW = *mut OPENCARD_SEARCH_CRITERIAW;
#[repr(C)] #[derive(Copy)]
pub struct OPENCARDNAME_EXA {
    pub dwStructSize: ::DWORD,
    pub hSCardContext: SCARDCONTEXT,
    pub hwndOwner: ::HWND,
    pub dwFlags: ::DWORD,
    pub lpstrTitle: ::LPCSTR,
    pub lpstrSearchDesc: ::LPCSTR,
    pub hIcon: ::HICON,
    pub pOpenCardSearchCriteria: POPENCARD_SEARCH_CRITERIAA,
    pub lpfnConnect: LPOCNCONNPROCA,
    pub pvUserData: ::LPVOID,
    pub dwShareMode: ::DWORD,
    pub dwPreferredProtocols: ::DWORD,
    pub lpstrRdr: ::LPSTR,
    pub nMaxRdr: ::DWORD,
    pub lpstrCard: ::LPSTR,
    pub nMaxCard: ::DWORD,
    pub dwActiveProtocol: ::DWORD,
    pub hCardHandle: SCARDHANDLE,
}
impl Clone for OPENCARDNAME_EXA { fn clone(&self) -> OPENCARDNAME_EXA { *self } }
pub type POPENCARDNAME_EXA = *mut OPENCARDNAME_EXA;
pub type LPOPENCARDNAME_EXA = *mut OPENCARDNAME_EXA;
#[repr(C)] #[derive(Copy)]
pub struct OPENCARDNAME_EXW {
    pub dwStructSize: ::DWORD,
    pub hSCardContext: SCARDCONTEXT,
    pub hwndOwner: ::HWND,
    pub dwFlags: ::DWORD,
    pub lpstrTitle: ::LPCWSTR,
    pub lpstrSearchDesc: ::LPCWSTR,
    pub hIcon: ::HICON,
    pub pOpenCardSearchCriteria: POPENCARD_SEARCH_CRITERIAW,
    pub lpfnConnect: LPOCNCONNPROCW,
    pub pvUserData: ::LPVOID,
    pub dwShareMode: ::DWORD,
    pub dwPreferredProtocols: ::DWORD,
    pub lpstrRdr: ::LPWSTR,
    pub nMaxRdr: ::DWORD,
    pub lpstrCard: ::LPWSTR,
    pub nMaxCard: ::DWORD,
    pub dwActiveProtocol: ::DWORD,
    pub hCardHandle: SCARDHANDLE,
}
impl Clone for OPENCARDNAME_EXW { fn clone(&self) -> OPENCARDNAME_EXW { *self } }
pub type POPENCARDNAME_EXW = *mut OPENCARDNAME_EXW;
pub type LPOPENCARDNAME_EXW = *mut OPENCARDNAME_EXW;
pub type OPENCARDNAMEA_EX = OPENCARDNAME_EXA;
pub type OPENCARDNAMEW_EX = OPENCARDNAME_EXW;
pub type POPENCARDNAMEA_EX = POPENCARDNAME_EXA;
pub type POPENCARDNAMEW_EX = POPENCARDNAME_EXW;
pub type LPOPENCARDNAMEA_EX = LPOPENCARDNAME_EXA;
pub type LPOPENCARDNAMEW_EX = LPOPENCARDNAME_EXW;
pub const SCARD_READER_SEL_AUTH_PACKAGE: ::DWORD = -629i32 as ::DWORD;
ENUM!{enum READER_SEL_REQUEST_MATCH_TYPE {
    RSR_MATCH_TYPE_READER_AND_CONTAINER = 1,
    RSR_MATCH_TYPE_SERIAL_NUMBER,
    RSR_MATCH_TYPE_ALL_CARDS,
}}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct READER_SEL_REQUEST_ReaderAndContainerParameter {
    pub cbReaderNameOffset: ::DWORD,
    pub cchReaderNameLength: ::DWORD,
    pub cbContainerNameOffset: ::DWORD,
    pub cchContainerNameLength: ::DWORD,
    pub dwDesiredCardModuleVersion: ::DWORD,
    pub dwCspFlags: ::DWORD,
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct READER_SEL_REQUEST_SerialNumberParameter {
    pub cbSerialNumberOffset: ::DWORD,
    pub cbSerialNumberLength: ::DWORD,
    pub dwDesiredCardModuleVersion: ::DWORD,
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct READER_SEL_REQUEST {
    pub dwShareMode: ::DWORD,
    pub dwPreferredProtocols: ::DWORD,
    pub MatchType: READER_SEL_REQUEST_MATCH_TYPE,
    pub ReaderAndContainerParameter: READER_SEL_REQUEST_ReaderAndContainerParameter,
}
UNION!(
    READER_SEL_REQUEST, ReaderAndContainerParameter, SerialNumberParameter,
    SerialNumberParameter_mut, READER_SEL_REQUEST_SerialNumberParameter
);
pub type PREADER_SEL_REQUEST = *mut READER_SEL_REQUEST;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct READER_SEL_RESPONSE {
    pub cbReaderNameOffset: ::DWORD,
    pub cchReaderNameLength: ::DWORD,
    pub cbCardNameOffset: ::DWORD,
    pub cchCardNameLength: ::DWORD,
}
pub type PREADER_SEL_RESPONSE = *mut READER_SEL_RESPONSE;
#[repr(C)] #[derive(Copy)]
pub struct OPENCARDNAMEA {
    pub dwStructSize: ::DWORD,
    pub hwndOwner: ::HWND,
    pub hSCardContext: SCARDCONTEXT,
    pub lpstrGroupNames: ::LPSTR,
    pub nMaxGroupNames: ::DWORD,
    pub lpstrCardNames: ::LPSTR,
    pub nMaxCardNames: ::DWORD,
    pub rgguidInterfaces: ::LPCGUID,
    pub cguidInterfaces: ::DWORD,
    pub lpstrRdr: ::LPSTR,
    pub nMaxRdr: ::DWORD,
    pub lpstrCard: ::LPSTR,
    pub nMaxCard: ::DWORD,
    pub lpstrTitle: ::LPCSTR,
    pub dwFlags: ::DWORD,
    pub pvUserData: ::LPVOID,
    pub dwShareMode: ::DWORD,
    pub dwPreferredProtocols: ::DWORD,
    pub dwActiveProtocol: ::DWORD,
    pub lpfnConnect: LPOCNCONNPROCA,
    pub lpfnCheck: LPOCNCHKPROC,
    pub lpfnDisconnect: LPOCNDSCPROC,
    pub hCardHandle: SCARDHANDLE,
}
impl Clone for OPENCARDNAMEA { fn clone(&self) -> OPENCARDNAMEA { *self } }
pub type POPENCARDNAMEA = *mut OPENCARDNAMEA;
pub type LPOPENCARDNAMEA = *mut OPENCARDNAMEA;
#[repr(C)] #[derive(Copy)]
pub struct OPENCARDNAMEW {
    pub dwStructSize: ::DWORD,
    pub hwndOwner: ::HWND,
    pub hSCardContext: SCARDCONTEXT,
    pub lpstrGroupNames: ::LPWSTR,
    pub nMaxGroupNames: ::DWORD,
    pub lpstrCardNames: ::LPWSTR,
    pub nMaxCardNames: ::DWORD,
    pub rgguidInterfaces: ::LPCGUID,
    pub cguidInterfaces: ::DWORD,
    pub lpstrRdr: ::LPWSTR,
    pub nMaxRdr: ::DWORD,
    pub lpstrCard: ::LPWSTR,
    pub nMaxCard: ::DWORD,
    pub lpstrTitle: ::LPCWSTR,
    pub dwFlags: ::DWORD,
    pub pvUserData: ::LPVOID,
    pub dwShareMode: ::DWORD,
    pub dwPreferredProtocols: ::DWORD,
    pub dwActiveProtocol: ::DWORD,
    pub lpfnConnect: LPOCNCONNPROCW,
    pub lpfnCheck: LPOCNCHKPROC,
    pub lpfnDisconnect: LPOCNDSCPROC,
    pub hCardHandle: SCARDHANDLE,
}
impl Clone for OPENCARDNAMEW { fn clone(&self) -> OPENCARDNAMEW { *self } }
pub type POPENCARDNAMEW = *mut OPENCARDNAMEW;
pub type LPOPENCARDNAMEW = *mut OPENCARDNAMEW;
pub type OPENCARDNAME_A = OPENCARDNAMEA;
pub type OPENCARDNAME_W = OPENCARDNAMEW;
pub type POPENCARDNAME_A = POPENCARDNAMEA;
pub type POPENCARDNAME_W = POPENCARDNAMEW;
pub type LPOPENCARDNAME_A = LPOPENCARDNAMEA;
pub type LPOPENCARDNAME_W = LPOPENCARDNAMEW;
pub const SCARD_AUDIT_CHV_FAILURE: ::DWORD = 0x0;
pub const SCARD_AUDIT_CHV_SUCCESS: ::DWORD = 0x1;

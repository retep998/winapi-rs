// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! DbgHelp include file

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SYMBOL_INFOW {
    pub SizeOfStruct: ::ULONG,
    pub TypeIndex: ::ULONG,
    pub Reserved: [::ULONG64; 2],
    pub Index: ::ULONG,
    pub Size: ::ULONG,
    pub ModBase: ::ULONG64,
    pub Flags: ::ULONG,
    pub Value: ::ULONG64,
    pub Address: ::ULONG64,
    pub Register: ::ULONG,
    pub Scope: ::ULONG,
    pub Tag: ::ULONG,
    pub NameLen: ::ULONG,
    pub MaxNameLen: ::ULONG,
    pub Name: [::WCHAR; 1],
}

pub type PSYMBOL_INFOW = *mut SYMBOL_INFOW;

pub type PREAD_PROCESS_MEMORY_ROUTINE64 =
    Option<unsafe extern "system" fn(::HANDLE, ::DWORD64, ::PVOID,
                                     ::DWORD, ::LPDWORD) -> ::BOOL>;
pub type PFUNCTION_TABLE_ACCESS_ROUTINE64 =
    Option<unsafe extern "system" fn(::HANDLE, ::DWORD64) -> ::PVOID>;
pub type PGET_MODULE_BASE_ROUTINE64 =
    Option<unsafe extern "system" fn(::HANDLE, ::DWORD64) -> ::DWORD64>;
pub type PTRANSLATE_ADDRESS_ROUTINE64 =
    Option<unsafe extern "system" fn(::HANDLE, ::HANDLE,
                                     LPADDRESS64) -> ::DWORD64>;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ADDRESS64 {
    pub Offset: ::DWORD64,
    pub Segment: ::WORD,
    pub Mode: ADDRESS_MODE,
}

pub type LPADDRESS64 = *mut ADDRESS64;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub enum ADDRESS_MODE {
    AddrMode1616,
    AddrMode1632,
    AddrModeReal,
    AddrModeFlat,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct STACKFRAME64 {
    pub AddrPC: ADDRESS64,
    pub AddrReturn: ADDRESS64,
    pub AddrFrame: ADDRESS64,
    pub AddrStack: ADDRESS64,
    pub AddrBStore: ADDRESS64,
    pub FuncTableEntry: ::PVOID,
    pub Params: [::DWORD64; 4],
    pub Far: ::BOOL,
    pub Virtual: ::BOOL,
    pub Reserved: [::DWORD64; 3],
    pub KdHelp: KDHELP64,
}

pub type LPSTACKFRAME64 = *mut STACKFRAME64;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct KDHELP64 {
    pub Thread: ::DWORD64,
    pub ThCallbackStack: ::DWORD,
    pub ThCallbackBStore: ::DWORD,
    pub NextCallback: ::DWORD,
    pub FramePointer: ::DWORD,
    pub KiCallUserMode: ::DWORD64,
    pub KeUserCallbackDispatcher: ::DWORD64,
    pub SystemRangeStart: ::DWORD64,
    pub KiUserExceptionDispatcher: ::DWORD64,
    pub StackBase: ::DWORD64,
    pub StackLimit: ::DWORD64,
    pub Reserved: [::DWORD64; 5],
}

pub type PKDHELP64 = *mut KDHELP64;

pub const MAX_SYM_NAME: usize = 2000;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IMAGEHLP_SYMBOL64 {
    pub SizeOfStruct: ::DWORD,
    pub Address: ::DWORD64,
    pub Size: ::DWORD,
    pub Flags: ::DWORD,
    pub MaxNameLength: ::DWORD,
    pub Name: [::CHAR; 1],
}

pub type PIMAGEHLP_SYMBOL64 = *mut IMAGEHLP_SYMBOL64;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IMAGEHLP_LINEW64 {
    pub SizeOfStruct: ::DWORD,
    pub Key: ::PVOID,
    pub LineNumber: ::DWORD,
    pub FileName: ::PWSTR,
    pub Address: ::DWORD64,
}

pub type PIMAGEHLP_LINEW64 = *mut IMAGEHLP_LINEW64;

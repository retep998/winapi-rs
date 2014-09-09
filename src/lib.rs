// Copyright © 2014 Peter Atashian

extern crate libc;

// basic types
pub type BOOL = libc::c_int;
pub type DWORD = libc::c_ulong;
pub type HANDLE = *mut VOID;
pub type ULONG = libc::c_ulong;
pub type ULONG_PTR = uint;
pub type VOID = libc::c_void;
// mut pointers
pub type PCONSOLE_READCONSOLE_CONTROL = *mut CONSOLE_READCONSOLE_CONTROL;
// long mut pointers
pub type LPDWORD = *mut DWORD;
pub type LPVOID = *mut VOID;
// long const pointers
pub type LPCVOID = *const VOID;

//structs
#[repr(C)] pub struct CONSOLE_READCONSOLE_CONTROL {
    nLength: ULONG,
    nInitialChars: ULONG,
    dwCtrlWakeupMask: ULONG,
    dwControlKeyState: ULONG,
}
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

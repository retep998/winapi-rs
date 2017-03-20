// Copyright © 2016-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! ApiSet Contract for api-ms-win-core-console-l1
use shared::minwindef::{BOOL, DWORD, LPDWORD, LPVOID, UINT};
use um::wincon::{PCONSOLE_READCONSOLE_CONTROL, PHANDLER_ROUTINE, PINPUT_RECORD};
use um::winnt::{HANDLE, VOID};
EXTERN!{stdcall fn AllocConsole() -> BOOL}
EXTERN!{stdcall fn GetConsoleCP() -> UINT}
EXTERN!{stdcall fn GetConsoleMode(
    hConsoleHandle: HANDLE,
    lpMode: LPDWORD
) -> BOOL}
EXTERN!{stdcall fn GetConsoleOutputCP() -> UINT}
EXTERN!{stdcall fn GetNumberOfConsoleInputEvents(
    hConsoleInput: HANDLE,
    lpNumberOfEvents: LPDWORD
) -> BOOL}
EXTERN!{stdcall fn PeekConsoleInputA(
    hConsoleInput: HANDLE,
    lpBuffer: PINPUT_RECORD,
    nLength: DWORD,
    lpNumberOfEventsRead: LPDWORD
) -> BOOL}
EXTERN!{stdcall fn ReadConsoleA(
    hConsoleInput: HANDLE,
    lpBuffer: LPVOID,
    nNumberOfCharsToRead: DWORD,
    lpNumberOfCharsRead: LPDWORD,
    pInputControl: PCONSOLE_READCONSOLE_CONTROL
) -> BOOL}
EXTERN!{stdcall fn ReadConsoleW(
    hConsoleInput: HANDLE,
    lpBuffer: LPVOID,
    nNumberOfCharsToRead: DWORD,
    lpNumberOfCharsRead: LPDWORD,
    pInputControl: PCONSOLE_READCONSOLE_CONTROL
) -> BOOL}
EXTERN!{stdcall fn ReadConsoleInputA(
    hConsoleInput: HANDLE,
    lpBuffer: PINPUT_RECORD,
    nLength: DWORD,
    lpNumberOfEventsRead: LPDWORD
) -> BOOL}
EXTERN!{stdcall fn ReadConsoleInputW(
    hConsoleInput: HANDLE,
    lpBuffer: PINPUT_RECORD,
    nLength: DWORD,
    lpNumberOfEventsRead: LPDWORD
) -> BOOL}
EXTERN!{stdcall fn SetConsoleCtrlHandler(
    HandlerRoutine: PHANDLER_ROUTINE,
    Add: BOOL
) -> BOOL}
EXTERN!{stdcall fn SetConsoleMode(
    hConsoleHandle: HANDLE,
    dwMode: DWORD
) -> BOOL}
EXTERN!{stdcall fn WriteConsoleA(
    hConsoleOutput: HANDLE,
    lpBuffer: *const VOID,
    nNumberOfCharsToWrite: DWORD,
    lpNumberOfCharsWritten: LPDWORD,
    lpReserved: LPVOID
) -> BOOL}
EXTERN!{stdcall fn WriteConsoleW(
    hConsoleOutput: HANDLE,
    lpBuffer: *const VOID,
    nNumberOfCharsToWrite: DWORD,
    lpNumberOfCharsWritten: LPDWORD,
    lpReserved: LPVOID
) -> BOOL}

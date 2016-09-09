// Copyright © 2016 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! This module contains the public data structures, data types, and procedures exported by the NT
//! console subsystem.
// Done as of 10.0.14393.0
#![cfg(feature = "um.wincon")]
use ctypes::c_void;
use shared::minwindef::{ BOOL, DWORD, UINT, ULONG, WORD };
use shared::windef::COLORREF;
use um::wingdi::LF_FACESIZE;
use um::winnt::{ CHAR, SHORT, WCHAR };
#[cfg(feature = "kernel32")]
IFDEF!{
use shared::minwindef::{ LPDWORD, LPVOID, LPWORD };
use shared::windef::HWND;
use um::minwinbase::SECURITY_ATTRIBUTES;
use um::winnt::{ HANDLE, LPCSTR, LPCWSTR, LPSTR, LPWSTR };
}
STRUCT!{struct COORD {
    X: SHORT,
    Y: SHORT,
}}
pub type PCOORD = *mut COORD;
STRUCT!{struct SMALL_RECT {
    Left: SHORT,
    Top: SHORT,
    Right: SHORT,
    Bottom: SHORT,
}}
pub type PSMALL_RECT = *mut SMALL_RECT;
STRUCT!{struct KEY_EVENT_RECORD {
    bKeyDown: BOOL,
    wRepeatCount: WORD,
    wVirtualKeyCode: WORD,
    wVirtualScanCode: WORD,
    UnicodeChar: WCHAR,
    dwControlKeyState: DWORD,
}}
UNION!{KEY_EVENT_RECORD, UnicodeChar, AsciiChar, AsciiChar_mut, CHAR}
pub type PKEY_EVENT_RECORD = *mut KEY_EVENT_RECORD;
pub const RIGHT_ALT_PRESSED: DWORD = 0x0001;
pub const LEFT_ALT_PRESSED: DWORD = 0x0002;
pub const RIGHT_CTRL_PRESSED: DWORD = 0x0004;
pub const LEFT_CTRL_PRESSED: DWORD = 0x0008;
pub const SHIFT_PRESSED: DWORD = 0x0010;
pub const NUMLOCK_ON: DWORD = 0x0020;
pub const SCROLLLOCK_ON: DWORD = 0x0040;
pub const CAPSLOCK_ON: DWORD = 0x0080;
pub const ENHANCED_KEY: DWORD = 0x0100;
pub const NLS_DBCSCHAR: DWORD = 0x00010000;
pub const NLS_ALPHANUMERIC: DWORD = 0x00000000;
pub const NLS_KATAKANA: DWORD = 0x00020000;
pub const NLS_HIRAGANA: DWORD = 0x00040000;
pub const NLS_ROMAN: DWORD = 0x00400000;
pub const NLS_IME_CONVERSION: DWORD = 0x00800000;
pub const NLS_IME_DISABLE: DWORD = 0x20000000;
STRUCT!{struct MOUSE_EVENT_RECORD {
    dwMousePosition: COORD,
    dwButtonState: DWORD,
    dwControlKeyState: DWORD,
    dwEventFlags: DWORD,
}}
pub type PMOUSE_EVENT_RECORD = *mut MOUSE_EVENT_RECORD;
pub const FROM_LEFT_1ST_BUTTON_PRESSED: DWORD = 0x0001;
pub const RIGHTMOST_BUTTON_PRESSED: DWORD = 0x0002;
pub const FROM_LEFT_2ND_BUTTON_PRESSED: DWORD = 0x0004;
pub const FROM_LEFT_3RD_BUTTON_PRESSED: DWORD = 0x0008;
pub const FROM_LEFT_4TH_BUTTON_PRESSED: DWORD = 0x0010;
pub const MOUSE_MOVED: DWORD = 0x0001;
pub const DOUBLE_CLICK: DWORD = 0x0002;
pub const MOUSE_WHEELED: DWORD = 0x0004;
pub const MOUSE_HWHEELED: DWORD = 0x0008;
STRUCT!{struct WINDOW_BUFFER_SIZE_RECORD {
    dwSize: COORD,
}}
pub type PWINDOW_BUFFER_SIZE_RECORD = *mut WINDOW_BUFFER_SIZE_RECORD;
STRUCT!{struct MENU_EVENT_RECORD {
    dwCommandId: UINT,
}}
pub type PMENU_EVENT_RECORD = *mut MENU_EVENT_RECORD;
STRUCT!{struct FOCUS_EVENT_RECORD {
    bSetFocus: BOOL,
}}
pub type PFOCUS_EVENT_RECORD = *mut FOCUS_EVENT_RECORD;
STRUCT!{struct INPUT_RECORD {
    EventType: WORD,
    Event: [u32; 4],
}}
UNION!{INPUT_RECORD, Event, KeyEvent, KeyEvent_mut, KEY_EVENT_RECORD}
UNION!{INPUT_RECORD, Event, MouseEvent, MouseEvent_mut, MOUSE_EVENT_RECORD}
UNION!{INPUT_RECORD, Event, WindowBufferSizeEvent, WindowBufferSizeEvent_mut,
    WINDOW_BUFFER_SIZE_RECORD}
UNION!{INPUT_RECORD, Event, MenuEvent, MenuEvent_mut, MENU_EVENT_RECORD}
UNION!{INPUT_RECORD, Event, FocusEvent, FocusEvent_mut, FOCUS_EVENT_RECORD}
pub type PINPUT_RECORD = *mut INPUT_RECORD;
pub const KEY_EVENT: WORD = 0x0001;
pub const MOUSE_EVENT: WORD = 0x0002;
pub const WINDOW_BUFFER_SIZE_EVENT: WORD = 0x0004;
pub const MENU_EVENT: WORD = 0x0008;
pub const FOCUS_EVENT: WORD = 0x0010;
STRUCT!{struct CHAR_INFO {
    UnicodeChar: WCHAR,
    Attributes: WORD,
}}
UNION!{CHAR_INFO, UnicodeChar, AsciiChar, AsciiChar_mut, CHAR}
pub type PCHAR_INFO = *mut CHAR_INFO;
pub const FOREGROUND_BLUE: DWORD = 0x0001;
pub const FOREGROUND_GREEN: DWORD = 0x0002;
pub const FOREGROUND_RED: DWORD = 0x0004;
pub const FOREGROUND_INTENSITY: DWORD = 0x0008;
pub const BACKGROUND_BLUE: DWORD = 0x0010;
pub const BACKGROUND_GREEN: DWORD = 0x0020;
pub const BACKGROUND_RED: DWORD = 0x0040;
pub const BACKGROUND_INTENSITY: DWORD = 0x0080;
pub const COMMON_LVB_LEADING_BYTE: DWORD = 0x0100;
pub const COMMON_LVB_TRAILING_BYTE: DWORD = 0x0200;
pub const COMMON_LVB_GRID_HORIZONTAL: DWORD = 0x0400;
pub const COMMON_LVB_GRID_LVERTICAL: DWORD = 0x0800;
pub const COMMON_LVB_GRID_RVERTICAL: DWORD = 0x1000;
pub const COMMON_LVB_REVERSE_VIDEO: DWORD = 0x4000;
pub const COMMON_LVB_UNDERSCORE: DWORD = 0x8000;
pub const COMMON_LVB_SBCSDBCS: DWORD = 0x0300;
STRUCT!{struct CONSOLE_SCREEN_BUFFER_INFO {
    dwSize: COORD,
    dwCursorPosition: COORD,
    wAttributes: WORD,
    srWindow: SMALL_RECT,
    dwMaximumWindowSize: COORD,
}}
pub type PCONSOLE_SCREEN_BUFFER_INFO = *mut CONSOLE_SCREEN_BUFFER_INFO;
STRUCT!{struct CONSOLE_SCREEN_BUFFER_INFOEX {
    cbSize: ULONG,
    dwSize: COORD,
    dwCursorPosition: COORD,
    wAttributes: WORD,
    srWindow: SMALL_RECT,
    dwMaximumWindowSize: COORD,
    wPopupAttributes: WORD,
    bFullscreenSupported: BOOL,
    ColorTable: [COLORREF; 16],
}}
pub type PCONSOLE_SCREEN_BUFFER_INFOEX = *mut CONSOLE_SCREEN_BUFFER_INFOEX;
STRUCT!{struct CONSOLE_CURSOR_INFO {
    dwSize: DWORD,
    bVisible: BOOL,
}}
pub type PCONSOLE_CURSOR_INFO = *mut CONSOLE_CURSOR_INFO;
STRUCT!{struct CONSOLE_FONT_INFO {
    nFont: DWORD,
    dwFontSize: COORD,
}}
pub type PCONSOLE_FONT_INFO = *mut CONSOLE_FONT_INFO;
STRUCT!{struct CONSOLE_FONT_INFOEX {
    cbSize: ULONG,
    nFont: DWORD,
    dwFontSize: COORD,
    FontFamily: UINT,
    FontWeight: UINT,
    FaceName: [WCHAR; LF_FACESIZE],
}}
pub type PCONSOLE_FONT_INFOEX = *mut CONSOLE_FONT_INFOEX;
pub const HISTORY_NO_DUP_FLAG: DWORD = 0x1;
STRUCT!{struct CONSOLE_HISTORY_INFO {
    cbSize: UINT,
    HistoryBufferSize: UINT,
    NumberOfHistoryBuffers: UINT,
    dwFlags: DWORD,
}}
pub type PCONSOLE_HISTORY_INFO = *mut CONSOLE_HISTORY_INFO;
STRUCT!{struct CONSOLE_SELECTION_INFO {
    dwFlags: DWORD,
    dwSelectionAnchor: COORD,
    srSelection: SMALL_RECT,
}}
pub type PCONSOLE_SELECTION_INFO = *mut CONSOLE_SELECTION_INFO;
pub const CONSOLE_NO_SELECTION: DWORD = 0x0000;
pub const CONSOLE_SELECTION_IN_PROGRESS: DWORD = 0x0001;
pub const CONSOLE_SELECTION_NOT_EMPTY: DWORD = 0x0002;
pub const CONSOLE_MOUSE_SELECTION: DWORD = 0x0004;
pub const CONSOLE_MOUSE_DOWN: DWORD = 0x0008;
FN!{stdcall PHANDLER_ROUTINE(
    CtrlType: DWORD
) -> BOOL}
pub const CTRL_C_EVENT: DWORD = 0;
pub const CTRL_BREAK_EVENT: DWORD = 1;
pub const CTRL_CLOSE_EVENT: DWORD = 2;
pub const CTRL_LOGOFF_EVENT: DWORD = 5;
pub const CTRL_SHUTDOWN_EVENT: DWORD = 6;
pub const ENABLE_PROCESSED_INPUT: DWORD = 0x0001;
pub const ENABLE_LINE_INPUT: DWORD = 0x0002;
pub const ENABLE_ECHO_INPUT: DWORD = 0x0004;
pub const ENABLE_WINDOW_INPUT: DWORD = 0x0008;
pub const ENABLE_MOUSE_INPUT: DWORD = 0x0010;
pub const ENABLE_INSERT_MODE: DWORD = 0x0020;
pub const ENABLE_QUICK_EDIT_MODE: DWORD = 0x0040;
pub const ENABLE_EXTENDED_FLAGS: DWORD = 0x0080;
pub const ENABLE_AUTO_POSITION: DWORD = 0x0100;
pub const ENABLE_VIRTUAL_TERMINAL_INPUT: DWORD = 0x0200;
pub const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001;
pub const ENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002;
pub const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004;
pub const DISABLE_NEWLINE_AUTO_RETURN: DWORD = 0x0008;
pub const ENABLE_LVB_GRID_WORLDWIDE: DWORD = 0x0010;
EXTERN!{"kernel32" "stdcall" fn PeekConsoleInputW(
    hConsoleInput: HANDLE,
    lpBuffer: PINPUT_RECORD,
    nLength: DWORD,
    lpNumberOfEventsRead: LPDWORD
) -> BOOL}
EXTERN!{"kernel32" "stdcall" fn WriteConsoleInputA(
    hConsoleInput: HANDLE,
    lpBuffer: *const INPUT_RECORD,
    nLength: DWORD,
    lpNumberOfEventsWritten: LPDWORD
) -> BOOL}
EXTERN!{"kernel32" "stdcall" fn WriteConsoleInputW(
    hConsoleInput: HANDLE,
    lpBuffer: *const INPUT_RECORD,
    nLength: DWORD,
    lpNumberOfEventsWritten: LPDWORD
) -> BOOL}
EXTERN!{"kernel32" "stdcall" fn ReadConsoleOutputA(
    hConsoleOutput: HANDLE,
    lpBuffer: PCHAR_INFO,
    dwBufferSize: COORD,
    dwBufferCoord: COORD,
    lpReadRegion: PSMALL_RECT
) -> BOOL}
EXTERN!{"kernel32" "stdcall" fn ReadConsoleOutputW(
    hConsoleOutput: HANDLE,
    lpBuffer: PCHAR_INFO,
    dwBufferSize: COORD,
    dwBufferCoord: COORD,
    lpReadRegion: PSMALL_RECT
) -> BOOL}
EXTERN!{"kernel32" "stdcall" fn WriteConsoleOutputA(
    hConsoleOutput: HANDLE,
    lpBuffer: *const CHAR_INFO,
    dwBufferSize: COORD,
    dwBufferCoord: COORD,
    lpWriteRegion: PSMALL_RECT
) -> BOOL}
EXTERN!{"kernel32" "stdcall" fn WriteConsoleOutputW(
    hConsoleOutput: HANDLE,
    lpBuffer: *const CHAR_INFO,
    dwBufferSize: COORD,
    dwBufferCoord: COORD,
    lpWriteRegion: PSMALL_RECT
) -> BOOL}
EXTERN!{"kernel32" "stdcall" fn ReadConsoleOutputCharacterA(
    hConsoleOutput: HANDLE,
    lpCharacter: LPSTR,
    nLength: DWORD,
    dwReadCoord: COORD,
    lpNumberOfCharsRead: LPDWORD
) -> BOOL}
EXTERN!{"kernel32" "stdcall" fn ReadConsoleOutputCharacterW(
    hConsoleOutput: HANDLE,
    lpCharacter: LPWSTR,
    nLength: DWORD,
    dwReadCoord: COORD,
    lpNumberOfCharsRead: LPDWORD
) -> BOOL}
EXTERN!{"kernel32" "stdcall" fn ReadConsoleOutputAttribute(
    hConsoleOutput: HANDLE,
    lpAttribute: LPWORD,
    nLength: DWORD,
    dwReadCoord: COORD,
    lpNumberOfAttrsRead: LPDWORD
) -> BOOL}
EXTERN!{"kernel32" "stdcall" fn WriteConsoleOutputCharacterA(
    hConsoleOutput: HANDLE,
    lpCharacter: LPCSTR,
    nLength: DWORD,
    dwWriteCoord: COORD,
    lpNumberOfCharsWritten: LPDWORD
) -> BOOL}
EXTERN!{"kernel32" "stdcall" fn WriteConsoleOutputCharacterW(
    hConsoleOutput: HANDLE,
    lpCharacter: LPCWSTR,
    nLength: DWORD,
    dwWriteCoord: COORD,
    lpNumberOfCharsWritten: LPDWORD
) -> BOOL}
EXTERN!{"kernel32" "stdcall" fn WriteConsoleOutputAttribute(
    hConsoleOutput: HANDLE,
    lpAttribute: *const WORD,
    nLength: DWORD,
    dwWriteCoord: COORD,
    lpNumberOfAttrsWritten: LPDWORD
) -> BOOL}
EXTERN!{"kernel32" "stdcall" fn FillConsoleOutputCharacterA(
    hConsoleOutput: HANDLE,
    cCharacter: CHAR,
    nLength: DWORD,
    dwWriteCoord: COORD,
    lpNumberOfCharsWritten: LPDWORD
) -> BOOL}
EXTERN!{"kernel32" "stdcall" fn FillConsoleOutputCharacterW(
    hConsoleOutput: HANDLE,
    cCharacter: WCHAR,
    nLength: DWORD,
    dwWriteCoord: COORD,
    lpNumberOfCharsWritten: LPDWORD
) -> BOOL}
EXTERN!{"kernel32" "stdcall" fn FillConsoleOutputAttribute(
    hConsoleOutput: HANDLE,
    wAttribute: WORD,
    nLength: DWORD,
    dwWriteCoord: COORD,
    lpNumberOfAttrsWritten: LPDWORD
) -> BOOL}
pub const CONSOLE_REAL_OUTPUT_HANDLE: *mut c_void = -2isize as *mut c_void;
pub const CONSOLE_REAL_INPUT_HANDLE: *mut c_void = -3isize as *mut c_void;
EXTERN!{"kernel32" "stdcall" fn GetConsoleScreenBufferInfo(
    hConsoleOutput: HANDLE,
    lpConsoleScreenBufferInfo: PCONSOLE_SCREEN_BUFFER_INFO
) -> BOOL}
EXTERN!{"kernel32" "stdcall" fn GetConsoleScreenBufferInfoEx(
    hConsoleOutput: HANDLE,
    lpConsoleScreenBufferInfoEx: PCONSOLE_SCREEN_BUFFER_INFOEX
) -> BOOL}
EXTERN!{"kernel32" "stdcall" fn SetConsoleScreenBufferInfoEx(
    hConsoleOutput: HANDLE,
    lpConsoleScreenBufferInfoEx: PCONSOLE_SCREEN_BUFFER_INFOEX
) -> BOOL}
EXTERN!{"kernel32" "stdcall" fn GetLargestConsoleWindowSize(
    hConsoleOutput: HANDLE
) -> COORD}
EXTERN!{"kernel32" "stdcall" fn GetConsoleCursorInfo(
    hConsoleOutput: HANDLE,
    lpConsoleCursorInfo: PCONSOLE_CURSOR_INFO
) -> BOOL}
EXTERN!{"kernel32" "stdcall" fn GetCurrentConsoleFont(
    hConsoleOutput: HANDLE,
    bMaximumWindow: BOOL,
    lpConsoleCurrentFont: PCONSOLE_FONT_INFO
) -> BOOL}
EXTERN!{"kernel32" "stdcall" fn GetCurrentConsoleFontEx(
    hConsoleOutput: HANDLE,
    bMaximumWindow: BOOL,
    lpConsoleCurrentFontEx: PCONSOLE_FONT_INFOEX
) -> BOOL}
EXTERN!{"kernel32" "stdcall" fn SetCurrentConsoleFontEx(
    hConsoleOutput: HANDLE,
    bMaximumWindow: BOOL,
    lpConsoleCurrentFontEx: PCONSOLE_FONT_INFOEX
) -> BOOL}
EXTERN!{"kernel32" "stdcall" fn GetConsoleHistoryInfo(
    lpConsoleHistoryInfo: PCONSOLE_HISTORY_INFO
) -> BOOL}
EXTERN!{"kernel32" "stdcall" fn SetConsoleHistoryInfo(
    lpConsoleHistoryInfo: PCONSOLE_HISTORY_INFO
) -> BOOL}
EXTERN!{"kernel32" "stdcall" fn GetConsoleFontSize(
    hConsoleOutput: HANDLE,
    nFont: DWORD
) -> COORD}
EXTERN!{"kernel32" "stdcall" fn GetConsoleSelectionInfo(
    lpConsoleSelectionInfo: PCONSOLE_SELECTION_INFO
) -> BOOL}
EXTERN!{"kernel32" "stdcall" fn GetNumberOfConsoleMouseButtons(
    lpNumberOfMouseButtons: LPDWORD
) -> BOOL}
EXTERN!{"kernel32" "stdcall" fn SetConsoleActiveScreenBuffer(
    hConsoleOutput: HANDLE
) -> BOOL}
EXTERN!{"kernel32" "stdcall" fn FlushConsoleInputBuffer(
    hConsoleInput: HANDLE
) -> BOOL}
EXTERN!{"kernel32" "stdcall" fn SetConsoleScreenBufferSize(
    hConsoleOutput: HANDLE,
    dwSize: COORD
) -> BOOL}
EXTERN!{"kernel32" "stdcall" fn SetConsoleCursorPosition(
    hConsoleOutput: HANDLE,
    dwCursorPosition: COORD
) -> BOOL}
EXTERN!{"kernel32" "stdcall" fn SetConsoleCursorInfo(
    hConsoleOutput: HANDLE,
    lpConsoleCursorInfo: *const CONSOLE_CURSOR_INFO
) -> BOOL}
EXTERN!{"kernel32" "stdcall" fn ScrollConsoleScreenBufferA(
    hConsoleOutput: HANDLE,
    lpScrollRectangle: *const SMALL_RECT,
    lpClipRectangle: *const SMALL_RECT,
    dwDestinationOrigin: COORD,
    lpFill: *const CHAR_INFO
) -> BOOL}
EXTERN!{"kernel32" "stdcall" fn ScrollConsoleScreenBufferW(
    hConsoleOutput: HANDLE,
    lpScrollRectangle: *const SMALL_RECT,
    lpClipRectangle: *const SMALL_RECT,
    dwDestinationOrigin: COORD,
    lpFill: *const CHAR_INFO
) -> BOOL}
EXTERN!{"kernel32" "stdcall" fn SetConsoleWindowInfo(
    hConsoleOutput: HANDLE,
    bAbsolute: BOOL,
    lpConsoleWindow: *const SMALL_RECT
) -> BOOL}
EXTERN!{"kernel32" "stdcall" fn SetConsoleTextAttribute(
    hConsoleOutput: HANDLE,
    wAttributes: WORD
) -> BOOL}
EXTERN!{"kernel32" "stdcall" fn GenerateConsoleCtrlEvent(
    dwCtrlEvent: DWORD,
    dwProcessGroupId: DWORD
) -> BOOL}
EXTERN!{"kernel32" "stdcall" fn FreeConsole() -> BOOL}
EXTERN!{"kernel32" "stdcall" fn AttachConsole(
    dwProcessId: DWORD
) -> BOOL}
pub const ATTACH_PARENT_PROCESS: DWORD = 0xFFFFFFFF;
EXTERN!{"kernel32" "stdcall" fn GetConsoleTitleA(
    lpConsoleTitle: LPSTR,
    nSize: DWORD
) -> DWORD}
EXTERN!{"kernel32" "stdcall" fn GetConsoleTitleW(
    lpConsoleTitle: LPWSTR,
    nSize: DWORD
) -> DWORD}
EXTERN!{"kernel32" "stdcall" fn GetConsoleOriginalTitleA(
    lpConsoleTitle: LPSTR,
    nSize: DWORD
) -> DWORD}
EXTERN!{"kernel32" "stdcall" fn GetConsoleOriginalTitleW(
    lpConsoleTitle: LPWSTR,
    nSize: DWORD
) -> DWORD}
EXTERN!{"kernel32" "stdcall" fn SetConsoleTitleA(
    lpConsoleTitle: LPCSTR
) -> BOOL}
EXTERN!{"kernel32" "stdcall" fn SetConsoleTitleW(
    lpConsoleTitle: LPCWSTR
) -> BOOL}
STRUCT!{struct CONSOLE_READCONSOLE_CONTROL {
    nLength: ULONG,
    nInitialChars: ULONG,
    dwCtrlWakeupMask: ULONG,
    dwControlKeyState: ULONG,
}}
pub type PCONSOLE_READCONSOLE_CONTROL = *mut CONSOLE_READCONSOLE_CONTROL;
pub const CONSOLE_TEXTMODE_BUFFER: DWORD = 1;
EXTERN!{"kernel32" "stdcall" fn CreateConsoleScreenBuffer(
    dwDesiredAccess: DWORD,
    dwShareMode: DWORD,
    lpSecurityAttributes: *const SECURITY_ATTRIBUTES,
    dwFlags: DWORD,
    lpScreenBufferData: LPVOID
) -> HANDLE}
EXTERN!{"kernel32" "stdcall" fn SetConsoleCP(
    wCodePageID: UINT
) -> BOOL}
EXTERN!{"kernel32" "stdcall" fn SetConsoleOutputCP(
    wCodePageID: UINT
) -> BOOL}
pub const CONSOLE_FULLSCREEN: DWORD = 1;
pub const CONSOLE_FULLSCREEN_HARDWARE: DWORD = 2;
EXTERN!{"kernel32" "stdcall" fn GetConsoleDisplayMode(
    lpModeFlags: LPDWORD
) -> BOOL}
pub const CONSOLE_FULLSCREEN_MODE: DWORD = 1;
pub const CONSOLE_WINDOWED_MODE: DWORD = 2;
EXTERN!{"kernel32" "stdcall" fn SetConsoleDisplayMode(
    hConsoleOutput: HANDLE,
    dwFlags: DWORD,
    lpNewScreenBufferDimensions: PCOORD
) -> BOOL}
EXTERN!{"kernel32" "stdcall" fn GetConsoleWindow() -> HWND}
EXTERN!{"kernel32" "stdcall" fn GetConsoleProcessList(
    lpdwProcessList: LPDWORD,
    dwProcessCount: DWORD
) -> DWORD}
EXTERN!{"kernel32" "stdcall" fn AddConsoleAliasA(
    Source: LPSTR,
    Target: LPSTR,
    ExeName: LPSTR
) -> BOOL}
EXTERN!{"kernel32" "stdcall" fn AddConsoleAliasW(
    Source: LPWSTR,
    Target: LPWSTR,
    ExeName: LPWSTR
) -> BOOL}
EXTERN!{"kernel32" "stdcall" fn GetConsoleAliasA(
    Source: LPSTR,
    TargetBuffer: LPSTR,
    TargetBufferLength: DWORD,
    ExeName: LPSTR
) -> DWORD}
EXTERN!{"kernel32" "stdcall" fn GetConsoleAliasW(
    Source: LPWSTR,
    TargetBuffer: LPWSTR,
    TargetBufferLength: DWORD,
    ExeName: LPWSTR
) -> DWORD}
EXTERN!{"kernel32" "stdcall" fn GetConsoleAliasesLengthA(
    ExeName: LPSTR
) -> DWORD}
EXTERN!{"kernel32" "stdcall" fn GetConsoleAliasesLengthW(
    ExeName: LPWSTR
) -> DWORD}
EXTERN!{"kernel32" "stdcall" fn GetConsoleAliasExesLengthA() -> DWORD}
EXTERN!{"kernel32" "stdcall" fn GetConsoleAliasExesLengthW() -> DWORD}
EXTERN!{"kernel32" "stdcall" fn GetConsoleAliasesA(
    AliasBuffer: LPSTR,
    AliasBufferLength: DWORD,
    ExeName: LPSTR
) -> DWORD}
EXTERN!{"kernel32" "stdcall" fn GetConsoleAliasesW(
    AliasBuffer: LPWSTR,
    AliasBufferLength: DWORD,
    ExeName: LPWSTR
) -> DWORD}
EXTERN!{"kernel32" "stdcall" fn GetConsoleAliasExesA(
    ExeNameBuffer: LPSTR,
    ExeNameBufferLength: DWORD
) -> DWORD}
EXTERN!{"kernel32" "stdcall" fn GetConsoleAliasExesW(
    ExeNameBuffer: LPWSTR,
    ExeNameBufferLength: DWORD
) -> DWORD}

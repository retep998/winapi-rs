// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! This module contains the public data structures, data types, and procedures exported by the NT
//! console subsystem.
#[repr(C)] #[derive(Copy)] pub struct COORD {
    pub X: ::SHORT,
    pub Y: ::SHORT,
}
pub type PCOORD = *mut COORD;
#[repr(C)] #[derive(Copy)] pub struct SMALL_RECT {
    pub Left: ::SHORT,
    pub Top: ::SHORT,
    pub Right: ::SHORT,
    pub Bottom: ::SHORT,
}
pub type PSMALL_RECT = *mut SMALL_RECT;
#[repr(C)] #[derive(Copy)] pub struct KEY_EVENT_RECORD {
    pub bKeyDown: ::BOOL,
    pub wRepeatCount: ::WORD,
    pub wVirtualKeyCode: ::WORD,
    pub wVirtualScanCode: ::WORD,
    pub uChar: ::WCHAR, //FIXME - untagged union
    pub dwControlKeyState: ::DWORD,
}
pub type PKEY_EVENT_RECORD = *mut KEY_EVENT_RECORD;
pub const RIGHT_ALT_PRESSED: ::DWORD = 0x0001;
pub const LEFT_ALT_PRESSED: ::DWORD = 0x0002;
pub const RIGHT_CTRL_PRESSED: ::DWORD = 0x0004;
pub const LEFT_CTRL_PRESSED: ::DWORD = 0x0008;
pub const SHIFT_PRESSED: ::DWORD = 0x0010;
pub const NUMLOCK_ON: ::DWORD = 0x0020;
pub const SCROLLLOCK_ON: ::DWORD = 0x0040;
pub const CAPSLOCK_ON: ::DWORD = 0x0080;
pub const ENHANCED_KEY: ::DWORD = 0x0100;
pub const NLS_DBCSCHAR: ::DWORD = 0x00010000;
pub const NLS_ALPHANUMERIC: ::DWORD = 0x00000000;
pub const NLS_KATAKANA: ::DWORD = 0x00020000;
pub const NLS_HIRAGANA: ::DWORD = 0x00040000;
pub const NLS_ROMAN: ::DWORD = 0x00400000;
pub const NLS_IME_CONVERSION: ::DWORD = 0x00800000;
pub const NLS_IME_DISABLE: ::DWORD = 0x20000000;
#[repr(C)] #[derive(Copy)] pub struct MOUSE_EVENT_RECORD {
    pub dwMousePosition: COORD,
    pub dwButtonState: ::DWORD,
    pub dwControlKeyState: ::DWORD,
    pub dwEventFlags: ::DWORD,
}
pub type PMOUSE_EVENT_RECORD = *mut MOUSE_EVENT_RECORD;
pub const FROM_LEFT_1ST_BUTTON_PRESSED: ::DWORD = 0x0001;
pub const RIGHTMOST_BUTTON_PRESSED: ::DWORD = 0x0002;
pub const FROM_LEFT_2ND_BUTTON_PRESSED: ::DWORD = 0x0004;
pub const FROM_LEFT_3RD_BUTTON_PRESSED: ::DWORD = 0x0008;
pub const FROM_LEFT_4TH_BUTTON_PRESSED: ::DWORD = 0x0010;
pub const MOUSE_MOVED: ::DWORD = 0x0001;
pub const DOUBLE_CLICK: ::DWORD = 0x0002;
pub const MOUSE_WHEELED: ::DWORD = 0x0004;
pub const MOUSE_HWHEELED: ::DWORD = 0x0008;
#[repr(C)] #[derive(Copy)] pub struct WINDOW_BUFFER_SIZE_RECORD {
    pub dwSize: COORD,
}
pub type PWINDOW_BUFFER_SIZE_RECORD = *mut WINDOW_BUFFER_SIZE_RECORD;
#[repr(C)] #[derive(Copy)] pub struct MENU_EVENT_RECORD {
    pub dwCommandId: ::UINT,
}
pub type PMENU_EVENT_RECORD = *mut MENU_EVENT_RECORD;
#[repr(C)] #[derive(Copy)] pub struct FOCUS_EVENT_RECORD {
    pub bSetFocus: ::BOOL,
}
pub type PFOCUS_EVENT_RECORD = *mut FOCUS_EVENT_RECORD;
#[repr(C)] #[derive(Copy)] pub struct INPUT_RECORD {
    pub EventType: ::WORD,
    pub Event: MOUSE_EVENT_RECORD, //FIXME - untagged union
}
#[test]
fn test_INPUT_RECORD() {
    use core::mem::{size_of, min_align_of};
    assert!(size_of::<MOUSE_EVENT_RECORD>() >= size_of::<KEY_EVENT_RECORD>());
    assert!(size_of::<MOUSE_EVENT_RECORD>() >= size_of::<WINDOW_BUFFER_SIZE_RECORD>());
    assert!(size_of::<MOUSE_EVENT_RECORD>() >= size_of::<MENU_EVENT_RECORD>());
    assert!(size_of::<MOUSE_EVENT_RECORD>() >= size_of::<FOCUS_EVENT_RECORD>());
    assert!(min_align_of::<MOUSE_EVENT_RECORD>() >= min_align_of::<KEY_EVENT_RECORD>());
    assert!(min_align_of::<MOUSE_EVENT_RECORD>() >= min_align_of::<WINDOW_BUFFER_SIZE_RECORD>());
    assert!(min_align_of::<MOUSE_EVENT_RECORD>() >= min_align_of::<MENU_EVENT_RECORD>());
    assert!(min_align_of::<MOUSE_EVENT_RECORD>() >= min_align_of::<FOCUS_EVENT_RECORD>());
}
pub type PINPUT_RECORD = *mut INPUT_RECORD;
pub const KEY_EVENT: ::DWORD = 0x0001;
pub const MOUSE_EVENT: ::DWORD = 0x0002;
pub const WINDOW_BUFFER_SIZE_EVENT: ::DWORD = 0x0004;
pub const MENU_EVENT: ::DWORD = 0x0008;
pub const FOCUS_EVENT: ::DWORD = 0x0010;
#[repr(C)] #[derive(Copy)] pub struct CHAR_INFO {
    pub Char: ::WCHAR, //FIXME - untagged union
    pub Attributes: ::WORD,
}
#[test]
fn test_CHAR_INFO() {
    use core::mem::{size_of, min_align_of};
    assert!(size_of::<::WCHAR>() >= size_of::<::CHAR>());
    assert!(min_align_of::<::WCHAR>() >= min_align_of::<::CHAR>());
}
pub type PCHAR_INFO = *mut CHAR_INFO;
pub const FOREGROUND_BLUE: ::DWORD = 0x0001;
pub const FOREGROUND_GREEN: ::DWORD = 0x0002;
pub const FOREGROUND_RED: ::DWORD = 0x0004;
pub const FOREGROUND_INTENSITY: ::DWORD = 0x0008;
pub const BACKGROUND_BLUE: ::DWORD = 0x0010;
pub const BACKGROUND_GREEN: ::DWORD = 0x0020;
pub const BACKGROUND_RED: ::DWORD = 0x0040;
pub const BACKGROUND_INTENSITY: ::DWORD = 0x0080;
pub const COMMON_LVB_LEADING_BYTE: ::DWORD = 0x0100;
pub const COMMON_LVB_TRAILING_BYTE: ::DWORD = 0x0200;
pub const COMMON_LVB_GRID_HORIZONTAL: ::DWORD = 0x0400;
pub const COMMON_LVB_GRID_LVERTICAL: ::DWORD = 0x0800;
pub const COMMON_LVB_GRID_RVERTICAL: ::DWORD = 0x1000;
pub const COMMON_LVB_REVERSE_VIDEO: ::DWORD = 0x4000;
pub const COMMON_LVB_UNDERSCORE: ::DWORD = 0x8000;
pub const COMMON_LVB_SBCSDBCS: ::DWORD = 0x0300;
#[repr(C)] #[derive(Copy)] pub struct CONSOLE_SCREEN_BUFFER_INFO {
    pub dwSize: COORD,
    pub dwCursorPosition: COORD,
    pub wAttributes: ::WORD,
    pub srWindow: SMALL_RECT,
    pub dwMaximumWindowSize: COORD,
}
pub type PCONSOLE_SCREEN_BUFFER_INFO = *mut CONSOLE_SCREEN_BUFFER_INFO;
#[repr(C)] #[derive(Copy)] pub struct CONSOLE_SCREEN_BUFFER_INFOEX {
    pub cbSize: ::ULONG,
    pub dwSize: COORD,
    pub dwCursorPosition: COORD,
    pub wAttributes: ::WORD,
    pub srWindow: SMALL_RECT,
    pub dwMaximumWindowSize: COORD,
    pub wPopupAttributes: ::WORD,
    pub bFullscreenSupported: ::BOOL,
    pub ColorTable: [::COLORREF; 16],
}
pub type PCONSOLE_SCREEN_BUFFER_INFOEX = *mut CONSOLE_SCREEN_BUFFER_INFOEX;
#[repr(C)] #[derive(Copy)] pub struct CONSOLE_CURSOR_INFO {
    pub dwSize: ::DWORD,
    pub bVisible: ::BOOL,
}
pub type PCONSOLE_CURSOR_INFO = *mut CONSOLE_CURSOR_INFO;
#[repr(C)] #[derive(Copy)] pub struct CONSOLE_FONT_INFO {
    pub nFont: ::DWORD,
    pub dwFontSize: ::COORD,
}
pub type PCONSOLE_FONT_INFO = *mut CONSOLE_FONT_INFO;
#[repr(C)] #[derive(Copy)] pub struct CONSOLE_FONT_INFOEX {
    pub cbSize: ::ULONG,
    pub nFont: ::DWORD,
    pub dwFontSize: COORD,
    pub FontFamily: ::UINT,
    pub FontWeight: ::UINT,
    pub FaceName: [::WCHAR; ::LF_FACESIZE],
}
pub type PCONSOLE_FONT_INFOEX = *mut CONSOLE_FONT_INFOEX;
pub const HISTORY_NO_DUP_FLAG: ::DWORD = 0x1;
#[repr(C)] #[derive(Copy)] pub struct CONSOLE_HISTORY_INFO {
    pub cbSize: ::UINT,
    pub HistoryBufferSize: ::UINT,
    pub NumberOfHistoryBuffers: ::UINT,
    pub dwFlags: ::DWORD,
}
pub type PCONSOLE_HISTORY_INFO = *mut CONSOLE_HISTORY_INFO;
#[repr(C)] #[derive(Copy)] pub struct CONSOLE_SELECTION_INFO {
    pub dwFlags: ::DWORD,
    pub dwSelectionAnchor: COORD,
    pub srSelection: SMALL_RECT,
}
pub type PCONSOLE_SELECTION_INFO = *mut CONSOLE_SELECTION_INFO;
pub const CONSOLE_NO_SELECTION: ::DWORD = 0x0000;
pub const CONSOLE_SELECTION_IN_PROGRESS: ::DWORD = 0x0001;
pub const CONSOLE_SELECTION_NOT_EMPTY: ::DWORD = 0x0002;
pub const CONSOLE_MOUSE_SELECTION: ::DWORD = 0x0004;
pub const CONSOLE_MOUSE_DOWN: ::DWORD = 0x0008;
pub type PHANDLER_ROUTINE = unsafe extern "system" fn(CtrlType: ::DWORD) -> ::BOOL;
pub const CTRL_C_EVENT: ::DWORD = 0;
pub const CTRL_BREAK_EVENT: ::DWORD = 1;
pub const CTRL_CLOSE_EVENT: ::DWORD = 2;
pub const CTRL_LOGOFF_EVENT: ::DWORD = 5;
pub const CTRL_SHUTDOWN_EVENT: ::DWORD = 6;
pub const ENABLE_PROCESSED_INPUT: ::DWORD = 0x0001;
pub const ENABLE_LINE_INPUT: ::DWORD = 0x0002;
pub const ENABLE_ECHO_INPUT: ::DWORD = 0x0004;
pub const ENABLE_WINDOW_INPUT: ::DWORD = 0x0008;
pub const ENABLE_MOUSE_INPUT: ::DWORD = 0x0010;
pub const ENABLE_INSERT_MODE: ::DWORD = 0x0020;
pub const ENABLE_QUICK_EDIT_MODE: ::DWORD = 0x0040;
pub const ENABLE_EXTENDED_FLAGS: ::DWORD = 0x0080;
pub const ENABLE_AUTO_POSITION: ::DWORD = 0x0100;
pub const ENABLE_PROCESSED_OUTPUT: ::DWORD = 0x0001;
pub const ENABLE_WRAP_AT_EOL_OUTPUT: ::DWORD = 0x0002;
pub const CONSOLE_REAL_OUTPUT_HANDLE: *mut ::c_void = -2 as *mut ::c_void;
pub const CONSOLE_REAL_INPUT_HANDLE: *mut ::c_void = -3 as *mut ::c_void;
pub const ATTACH_PARENT_PROCESS: ::DWORD = -1;
#[repr(C)] #[derive(Copy)] pub struct CONSOLE_READCONSOLE_CONTROL {
    pub nLength: ::ULONG,
    pub nInitialChars: ::ULONG,
    pub dwCtrlWakeupMask: ::ULONG,
    pub dwControlKeyState: ::ULONG,
}
pub type PCONSOLE_READCONSOLE_CONTROL = *mut CONSOLE_READCONSOLE_CONTROL;
pub const CONSOLE_TEXTMODE_BUFFER: ::DWORD = 1;
pub const CONSOLE_FULLSCREEN: ::DWORD = 1;
pub const CONSOLE_FULLSCREEN_HARDWARE: ::DWORD = 2;
pub const CONSOLE_FULLSCREEN_MODE: ::DWORD = 1;
pub const CONSOLE_WINDOWED_MODE: ::DWORD = 2;

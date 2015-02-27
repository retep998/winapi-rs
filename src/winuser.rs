// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! USER procedure declarations, constant definitions and macros
//2193
pub const WHEEL_DELTA: ::DWORD = 120;
//2206
pub const XBUTTON1: ::DWORD = 0x0001;
pub const XBUTTON2: ::DWORD = 0x0002;
//3082
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct CREATESTRUCTA {
    pub lpCreateParams: ::LPVOID,
    pub hInstance: ::HINSTANCE,
    pub hMenu: ::HMENU,
    pub hwndParent: ::HWND,
    pub cy: ::c_int,
    pub cx: ::c_int,
    pub y: ::c_int,
    pub x: ::c_int,
    pub style: ::LONG,
    pub lpszName: ::LPCSTR,
    pub lpszClass: ::LPCSTR,
    pub dwExStyle: ::DWORD,
}
pub type LPCREATESTRUCTA = *mut CREATESTRUCTA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct CREATESTRUCTW {
    pub lpCreateParams: ::LPVOID,
    pub hInstance: ::HINSTANCE,
    pub hMenu: ::HMENU,
    pub hwndParent: ::HWND,
    pub cy: ::c_int,
    pub cx: ::c_int,
    pub y: ::c_int,
    pub x: ::c_int,
    pub style: ::LONG,
    pub lpszName: ::LPCWSTR,
    pub lpszClass: ::LPCWSTR,
    pub dwExStyle: ::DWORD,
}
pub type LPCREATESTRUCTW = *mut CREATESTRUCTW;
//3400
pub const PM_NOREMOVE: ::UINT = 0x0000;
pub const PM_REMOVE: ::UINT = 0x0001;
pub const PM_NOYIELD: ::UINT = 0x0002;
pub const PM_QS_INPUT: ::UINT = QS_INPUT << 16;
pub const PM_QS_POSTMESSAGE: ::UINT = (QS_POSTMESSAGE | QS_HOTKEY | QS_TIMER) << 16;
pub const PM_QS_PAINT: ::UINT = QS_PAINT << 16;
pub const PM_QS_SENDMESSAGE: ::UINT = QS_SENDMESSAGE << 16;
//5741
pub const KEYEVENTF_EXTENDEDKEY: ::DWORD = 0x0001;
pub const KEYEVENTF_KEYUP: ::DWORD = 0x0002;
pub const KEYEVENTF_UNICODE: ::DWORD = 0x0004;
pub const KEYEVENTF_SCANCODE: ::DWORD = 0x0008;
pub const MOUSEEVENTF_MOVE: ::DWORD = 0x0001;
pub const MOUSEEVENTF_LEFTDOWN: ::DWORD = 0x0002;
pub const MOUSEEVENTF_LEFTUP: ::DWORD = 0x0004;
pub const MOUSEEVENTF_RIGHTDOWN: ::DWORD = 0x0008;
pub const MOUSEEVENTF_RIGHTUP: ::DWORD = 0x0010;
pub const MOUSEEVENTF_MIDDLEDOWN: ::DWORD = 0x0020;
pub const MOUSEEVENTF_MIDDLEUP: ::DWORD = 0x0040;
pub const MOUSEEVENTF_XDOWN: ::DWORD = 0x0080;
pub const MOUSEEVENTF_XUP: ::DWORD = 0x0100;
pub const MOUSEEVENTF_WHEEL: ::DWORD = 0x0800;
pub const MOUSEEVENTF_HWHEEL: ::DWORD = 0x01000;
pub const MOUSEEVENTF_MOVE_NOCOALESCE: ::DWORD = 0x2000;
pub const MOUSEEVENTF_VIRTUALDESK: ::DWORD = 0x4000;
pub const MOUSEEVENTF_ABSOLUTE: ::DWORD = 0x8000;
#[repr(C)] #[derive(Clone, Copy, Debug, Default)]
pub struct MOUSEINPUT {
    pub dx: ::LONG,
    pub dy: ::LONG,
    pub mouseData: ::DWORD,
    pub dwFlags: ::DWORD,
    pub time: ::DWORD,
    pub dwExtraInfo: ::ULONG_PTR,
}
pub type PMOUSEINPUT = *mut MOUSEINPUT;
pub type LPMOUSEINPUT = *mut MOUSEINPUT;
#[repr(C)] #[derive(Clone, Copy, Debug, Default)]
pub struct KEYBDINPUT {
    pub wVk: ::WORD,
    pub wScan: ::WORD,
    pub dwFlags: ::DWORD,
    pub time: ::DWORD,
    pub dwExtraInfo: ::ULONG_PTR,
}
pub type PKEYBDINPUT = *mut KEYBDINPUT;
pub type LPKEYBDINPUT = *mut KEYBDINPUT;
#[repr(C)] #[derive(Clone, Copy, Debug, Default)]
pub struct HARDWAREINPUT {
    pub uMsg: ::DWORD,
    pub wParamL: ::WORD,
    pub wParamH: ::WORD,
}
pub type PHARDWAREINPUT = *mut HARDWAREINPUT;
pub type LPHARDWAREINPUT= *mut HARDWAREINPUT;
pub const INPUT_MOUSE: ::DWORD = 0;
pub const INPUT_KEYBOARD: ::DWORD = 1;
pub const INPUT_HARDWARE: ::DWORD = 2;
#[repr(C)] #[derive(Clone, Copy, Debug, Default)]
pub struct INPUT {
    pub type_: ::DWORD,
    pub union_: MOUSEINPUT, // FIXME - Use a proper untagged union here
}
#[test]
fn test_INPUT() {
    use std::mem::{size_of, min_align_of};
    assert!(size_of::<MOUSEINPUT>() >= size_of::<HARDWAREINPUT>());
    assert!(size_of::<MOUSEINPUT>() >= size_of::<KEYBDINPUT>());
    assert!(min_align_of::<MOUSEINPUT>() >= min_align_of::<HARDWAREINPUT>());
    assert!(min_align_of::<MOUSEINPUT>() >= min_align_of::<KEYBDINPUT>());
}
pub type PINPUT = *mut INPUT;
pub type LPINPUT = *mut INPUT;
//Indices for GetWindowLong etc.
pub const GWL_EXSTYLE: ::c_int = -20;
pub const GWL_STYLE: ::c_int = -16;
pub const GWL_WNDPROC: ::c_int = -4;
pub const GWLP_WNDPROC: ::c_int = -4;
pub const GWL_HINSTANCE: ::c_int = -6;
pub const GWLP_HINSTANCE: ::c_int = -6;
pub const GWL_HWNDPARENT: ::c_int = -8;
pub const GWLP_HWNDPARENT: ::c_int = -8;
pub const GWL_ID: ::c_int = -12;
pub const GWLP_ID: ::c_int = -12;
pub const GWL_USERDATA: ::c_int = -21;
pub const GWLP_USERDATA: ::c_int = -21;
//6573
pub const QS_KEY: ::DWORD = 0x0001;
pub const QS_MOUSEMOVE: ::DWORD = 0x0002;
pub const QS_MOUSEBUTTON: ::DWORD = 0x0004;
pub const QS_POSTMESSAGE: ::DWORD = 0x0008;
pub const QS_TIMER: ::DWORD = 0x0010;
pub const QS_PAINT: ::DWORD = 0x0020;
pub const QS_SENDMESSAGE: ::DWORD = 0x0040;
pub const QS_HOTKEY: ::DWORD = 0x0080;
pub const QS_ALLPOSTMESSAGE: ::DWORD = 0x0100;
pub const QS_RAWINPUT: ::DWORD = 0x0400;
pub const QS_TOUCH: ::DWORD = 0x0800;
pub const QS_POINTER: ::DWORD = 0x1000;
pub const QS_MOUSE: ::DWORD = QS_MOUSEMOVE | QS_MOUSEBUTTON;
pub const QS_INPUT: ::DWORD = QS_MOUSE | QS_KEY | QS_RAWINPUT | QS_TOUCH | QS_POINTER;
pub const QS_ALLEVENTS: ::DWORD = QS_INPUT | QS_POSTMESSAGE | QS_TIMER | QS_PAINT | QS_HOTKEY;
pub const QS_ALLINPUT: ::DWORD = QS_INPUT | QS_POSTMESSAGE | QS_TIMER
    | QS_PAINT | QS_HOTKEY | QS_SENDMESSAGE;
//6789
pub const SM_CXSCREEN: ::c_int = 0;
pub const SM_CYSCREEN: ::c_int = 1;
pub const SM_CXVSCROLL: ::c_int = 2;
pub const SM_CYHSCROLL: ::c_int = 3;
pub const SM_CYCAPTION: ::c_int = 4;
pub const SM_CXBORDER: ::c_int = 5;
pub const SM_CYBORDER: ::c_int = 6;
pub const SM_CXDLGFRAME: ::c_int = 7;
pub const SM_CYDLGFRAME: ::c_int = 8;
pub const SM_CYVTHUMB: ::c_int = 9;
pub const SM_CXHTHUMB: ::c_int = 10;
pub const SM_CXICON: ::c_int = 11;
pub const SM_CYICON: ::c_int = 12;
pub const SM_CXCURSOR: ::c_int = 13;
pub const SM_CYCURSOR: ::c_int = 14;
pub const SM_CYMENU: ::c_int = 15;
pub const SM_CXFULLSCREEN: ::c_int = 16;
pub const SM_CYFULLSCREEN: ::c_int = 17;
pub const SM_CYKANJIWINDOW: ::c_int = 18;
pub const SM_MOUSEPRESENT: ::c_int = 19;
pub const SM_CYVSCROLL: ::c_int = 20;
pub const SM_CXHSCROLL: ::c_int = 21;
pub const SM_DEBUG: ::c_int = 22;
pub const SM_SWAPBUTTON: ::c_int = 23;
pub const SM_RESERVED1: ::c_int = 24;
pub const SM_RESERVED2: ::c_int = 25;
pub const SM_RESERVED3: ::c_int = 26;
pub const SM_RESERVED4: ::c_int = 27;
pub const SM_CXMIN: ::c_int = 28;
pub const SM_CYMIN: ::c_int = 29;
pub const SM_CXSIZE: ::c_int = 30;
pub const SM_CYSIZE: ::c_int = 31;
pub const SM_CXFRAME: ::c_int = 32;
pub const SM_CYFRAME: ::c_int = 33;
pub const SM_CXMINTRACK: ::c_int = 34;
pub const SM_CYMINTRACK: ::c_int = 35;
pub const SM_CXDOUBLECLK: ::c_int = 36;
pub const SM_CYDOUBLECLK: ::c_int = 37;
pub const SM_CXICONSPACING: ::c_int = 38;
pub const SM_CYICONSPACING: ::c_int = 39;
pub const SM_MENUDROPALIGNMENT: ::c_int = 40;
pub const SM_PENWINDOWS: ::c_int = 41;
pub const SM_DBCSENABLED: ::c_int = 42;
pub const SM_CMOUSEBUTTONS: ::c_int = 43;
pub const SM_CXFIXEDFRAME: ::c_int = SM_CXDLGFRAME;
pub const SM_CYFIXEDFRAME: ::c_int = SM_CYDLGFRAME;
pub const SM_CXSIZEFRAME: ::c_int = SM_CXFRAME;
pub const SM_CYSIZEFRAME: ::c_int = SM_CYFRAME;
pub const SM_SECURE: ::c_int = 44;
pub const SM_CXEDGE: ::c_int = 45;
pub const SM_CYEDGE: ::c_int = 46;
pub const SM_CXMINSPACING: ::c_int = 47;
pub const SM_CYMINSPACING: ::c_int = 48;
pub const SM_CXSMICON: ::c_int = 49;
pub const SM_CYSMICON: ::c_int = 50;
pub const SM_CYSMCAPTION: ::c_int = 51;
pub const SM_CXSMSIZE: ::c_int = 52;
pub const SM_CYSMSIZE: ::c_int = 53;
pub const SM_CXMENUSIZE: ::c_int = 54;
pub const SM_CYMENUSIZE: ::c_int = 55;
pub const SM_ARRANGE: ::c_int = 56;
pub const SM_CXMINIMIZED: ::c_int = 57;
pub const SM_CYMINIMIZED: ::c_int = 58;
pub const SM_CXMAXTRACK: ::c_int = 59;
pub const SM_CYMAXTRACK: ::c_int = 60;
pub const SM_CXMAXIMIZED: ::c_int = 61;
pub const SM_CYMAXIMIZED: ::c_int = 62;
pub const SM_NETWORK: ::c_int = 63;
pub const SM_CLEANBOOT: ::c_int = 67;
pub const SM_CXDRAG: ::c_int = 68;
pub const SM_CYDRAG: ::c_int = 69;
pub const SM_SHOWSOUNDS: ::c_int = 70;
pub const SM_CXMENUCHECK: ::c_int = 71;
pub const SM_CYMENUCHECK: ::c_int = 72;
pub const SM_SLOWMACHINE: ::c_int = 73;
pub const SM_MIDEASTENABLED: ::c_int = 74;
pub const SM_MOUSEWHEELPRESENT: ::c_int = 75;
pub const SM_XVIRTUALSCREEN: ::c_int = 76;
pub const SM_YVIRTUALSCREEN: ::c_int = 77;
pub const SM_CXVIRTUALSCREEN: ::c_int = 78;
pub const SM_CYVIRTUALSCREEN: ::c_int = 79;
pub const SM_CMONITORS: ::c_int = 80;
pub const SM_SAMEDISPLAYFORMAT: ::c_int = 81;
pub const SM_IMMENABLED: ::c_int = 82;
pub const SM_CXFOCUSBORDER: ::c_int = 83;
pub const SM_CYFOCUSBORDER: ::c_int = 84;
pub const SM_TABLETPC: ::c_int = 86;
pub const SM_MEDIACENTER: ::c_int = 87;
pub const SM_STARTER: ::c_int = 88;
pub const SM_SERVERR2: ::c_int = 89;
pub const SM_MOUSEHORIZONTALWHEELPRESENT: ::c_int = 91;
pub const SM_CXPADDEDBORDER: ::c_int = 92;
pub const SM_DIGITIZER: ::c_int = 94;
pub const SM_MAXIMUMTOUCHES: ::c_int = 95;
pub const SM_CMETRICS: ::c_int = 97;
pub const SM_REMOTESESSION: ::c_int = 0x1000;
pub const SM_SHUTTINGDOWN: ::c_int = 0x2000;
pub const SM_REMOTECONTROL: ::c_int = 0x2001;
pub const SM_CARETBLINKINGENABLED: ::c_int = 0x2002;
pub const SM_CONVERTIBLESLATEMODE: ::c_int = 0x2003;
pub const SM_SYSTEMDOCKED: ::c_int = 0x2004;
//10069
pub const IDC_ARROW: ::LPCWSTR = 32512usize as ::LPCWSTR;
pub const IDC_IBEAM: ::LPCWSTR = 32513usize as ::LPCWSTR;
pub const IDC_WAIT: ::LPCWSTR = 32514usize as ::LPCWSTR;
pub const IDC_CROSS: ::LPCWSTR = 32515usize as ::LPCWSTR;
pub const IDC_UPARROW: ::LPCWSTR = 32516usize as ::LPCWSTR;
pub const IDC_SIZE: ::LPCWSTR = 32640usize as ::LPCWSTR;
pub const IDC_ICON: ::LPCWSTR = 32641usize as ::LPCWSTR;
pub const IDC_SIZENWSE: ::LPCWSTR = 32642usize as ::LPCWSTR;
pub const IDC_SIZENESW: ::LPCWSTR = 32643usize as ::LPCWSTR;
pub const IDC_SIZEWE: ::LPCWSTR = 32644usize as ::LPCWSTR;
pub const IDC_SIZENS: ::LPCWSTR = 32645usize as ::LPCWSTR;
pub const IDC_SIZEALL: ::LPCWSTR = 32646usize as ::LPCWSTR;
pub const IDC_NO: ::LPCWSTR = 32648usize as ::LPCWSTR;
pub const IDC_HAND: ::LPCWSTR = 32649usize as ::LPCWSTR;
pub const IDC_APPSTARTING: ::LPCWSTR = 32650usize as ::LPCWSTR;
pub const IDC_HELP: ::LPCWSTR = 32651usize as ::LPCWSTR;
//12969
pub type MONITORENUMPROC = Option<unsafe extern "system" fn(::HMONITOR, ::HDC, ::LPRECT, ::LPARAM) -> ::BOOL>;
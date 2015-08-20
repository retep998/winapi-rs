// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! USER procedure declarations, constant definitions and macros
//1913
pub const UNICODE_NOCHAR: ::WPARAM = 0xffff;
pub type HDWP = *mut ::HANDLE;
//2193
pub const WHEEL_DELTA: ::DWORD = 120;
//2206
pub const XBUTTON1: ::DWORD = 0x0001;
pub const XBUTTON2: ::DWORD = 0x0002;
//2392
pub const MK_LBUTTON: ::WPARAM = 0x0001;
pub const MK_RBUTTON: ::WPARAM = 0x0002;
pub const MK_SHIFT: ::WPARAM = 0x0004;
pub const MK_CONTROL: ::WPARAM = 0x0008;
pub const MK_MBUTTON: ::WPARAM = 0x0010;
pub const MK_XBUTTON1: ::WPARAM = 0x0020;
pub const MK_XBUTTON2: ::WPARAM = 0x0040;
//2408
pub const TME_HOVER: ::DWORD = 0x0000_0001;
pub const TME_LEAVE: ::DWORD = 0x0000_0002;
pub const TME_NONCLIENT: ::DWORD = 0x0000_0010;
pub const TME_QUERY: ::DWORD = 0x4000_0000;
pub const TME_CANCEL: ::DWORD = 0x8000_0000;
pub const HWND_MESSAGE: ::HWND = -3isize as ::HWND;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct TRACKMOUSEEVENT {
    pub cbSize: ::DWORD,
    pub dwFlags: ::DWORD,
    pub hwndTrack: ::HWND,
    pub dwHoverTime: ::DWORD,
}
pub type LPTRACKMOUSEEVENT = *mut TRACKMOUSEEVENT;
//2575
/// lParam of WM_WINDOWPOSCHANGING, WM_WINDOWPOSCHANGED
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct WINDOWPOS {
    pub hwnd: ::HWND,
    /// hwnd or HWND_BOTTOM, HWND_NOTOPMOST, HWND_TOP, HWND_TOPMOST
    pub hwndInsertAfter: ::HWND,
    pub x: ::c_int,
    pub y: ::c_int,
    pub cx: ::c_int,
    pub cy: ::c_int,
    /// SWP_\*
    pub flags: ::UINT,
}
pub type LPWINDOWPOS = *mut WINDOWPOS;
pub type PWINDOWPOS = *mut WINDOWPOS;
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
//3145
#[repr(C)] #[derive(Clone, Copy, Debug)]
/// The lParam of the WM_NOTIFY message is a pointer to this structure
pub struct NMHDR {
    pub hwndFrom: ::HWND,
    pub idFrom: ::UINT_PTR,
    pub code: ::UINT,  // NM_ code
}

//3400
pub const PM_NOREMOVE: ::UINT = 0x0000;
pub const PM_REMOVE: ::UINT = 0x0001;
pub const PM_NOYIELD: ::UINT = 0x0002;
pub const PM_QS_INPUT: ::UINT = QS_INPUT << 16;
pub const PM_QS_POSTMESSAGE: ::UINT = (QS_POSTMESSAGE | QS_HOTKEY | QS_TIMER) << 16;
pub const PM_QS_PAINT: ::UINT = QS_PAINT << 16;
pub const PM_QS_SENDMESSAGE: ::UINT = QS_SENDMESSAGE << 16;
//
pub const LWA_COLORKEY: ::DWORD = 0x00000001;
pub const LWA_ALPHA: ::DWORD = 0x00000002;
//3469
pub const EWX_LOGOFF: ::UINT = 0x00000000;
pub const EWX_SHUTDOWN: ::UINT = 0x00000001;
pub const EWX_REBOOT: ::UINT = 0x00000002;
pub const EWX_FORCE: ::UINT = 0x00000004;
pub const EWX_POWEROFF: ::UINT = 0x00000008;
pub const EWX_FORCEIFHUNG: ::UINT = 0x00000010;
pub const EWX_QUICKRESOLVE: ::UINT = 0x00000020;
pub const EWX_RESTARTAPPS: ::UINT = 0x00000040;
pub const EWX_HYBRID_SHUTDOWN: ::UINT = 0x00400000;
pub const EWX_BOOTOPTIONS: ::UINT = 0x01000000;
//4054 (Win 7 SDK)
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct FLASHWINFO {
    pub cbSize: ::UINT,
    pub hwnd: ::HWND,
    pub dwFlags: ::DWORD,
    pub uCount: ::UINT,
    pub dwTimeout: ::DWORD,
}
pub type PFLASHWINFO = *mut FLASHWINFO;
pub const FLASHW_STOP: ::DWORD = 0;
pub const FLASHW_CAPTION: ::DWORD = 0x00000001;
pub const FLASHW_TRAY: ::DWORD = 0x00000002;
pub const FLASHW_ALL: ::DWORD = FLASHW_CAPTION | FLASHW_TRAY;
pub const FLASHW_TIMER: ::DWORD = 0x00000004;
pub const FLASHW_TIMERNOFG: ::DWORD = 0x0000000C;
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
#[repr(C)] #[derive(Clone, Copy, Debug)]
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
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct KEYBDINPUT {
    pub wVk: ::WORD,
    pub wScan: ::WORD,
    pub dwFlags: ::DWORD,
    pub time: ::DWORD,
    pub dwExtraInfo: ::ULONG_PTR,
}
pub type PKEYBDINPUT = *mut KEYBDINPUT;
pub type LPKEYBDINPUT = *mut KEYBDINPUT;
#[repr(C)] #[derive(Clone, Copy, Debug)]
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
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct INPUT {
    pub type_: ::DWORD,
    pub union_: MOUSEINPUT, // FIXME - Untagged unions
}
#[test]
fn test_INPUT() {
    use std::mem::{size_of, align_of};
    assert!(size_of::<MOUSEINPUT>() >= size_of::<HARDWAREINPUT>());
    assert!(size_of::<MOUSEINPUT>() >= size_of::<KEYBDINPUT>());
    assert!(align_of::<MOUSEINPUT>() >= align_of::<HARDWAREINPUT>());
    assert!(align_of::<MOUSEINPUT>() >= align_of::<KEYBDINPUT>());
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
//5976
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum POINTER_INPUT_TYPE {
    PT_POINTER = 0x00000001,
    PT_TOUCH = 0x00000002,
    PT_PEN = 0x00000003,
    PT_MOUSE = 0x00000004,
    PT_TOUCHPAD = 0x00000005,
}
//6566
// flags for MsgWaitForMultipleObjectsEx
pub const MWMO_WAITALL: ::DWORD = 0x0001;
pub const MWMO_ALERTABLE: ::DWORD = 0x0002;
pub const MWMO_INPUTAVAILABLE: ::DWORD = 0x0004;
//6573
pub const QS_KEY: ::UINT = 0x0001;
pub const QS_MOUSEMOVE: ::UINT = 0x0002;
pub const QS_MOUSEBUTTON: ::UINT = 0x0004;
pub const QS_POSTMESSAGE: ::UINT = 0x0008;
pub const QS_TIMER: ::UINT = 0x0010;
pub const QS_PAINT: ::UINT = 0x0020;
pub const QS_SENDMESSAGE: ::UINT = 0x0040;
pub const QS_HOTKEY: ::UINT = 0x0080;
pub const QS_ALLPOSTMESSAGE: ::UINT = 0x0100;
pub const QS_RAWINPUT: ::UINT = 0x0400;
pub const QS_TOUCH: ::UINT = 0x0800;
pub const QS_POINTER: ::UINT = 0x1000;
pub const QS_MOUSE: ::UINT = QS_MOUSEMOVE | QS_MOUSEBUTTON;
pub const QS_INPUT: ::UINT = QS_MOUSE | QS_KEY | QS_RAWINPUT | QS_TOUCH | QS_POINTER;
pub const QS_ALLEVENTS: ::UINT = QS_INPUT | QS_POSTMESSAGE | QS_TIMER | QS_PAINT | QS_HOTKEY;
pub const QS_ALLINPUT: ::UINT = QS_INPUT | QS_POSTMESSAGE | QS_TIMER
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
//8855 (Win 7 SDK)
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ICONINFO {
    pub fIcon: ::BOOL,
    pub xHotspot: ::DWORD,
    pub yHotspot: ::DWORD,
    pub hbmMask: ::HBITMAP,
    pub hbmColor: ::HBITMAP,
}
pub type PICONINFO = *mut ICONINFO;
//9066
// Color indexes for use in GetSysColor and SetSysColor
// 0-18 (after incrementing) are also valid in RegisterClass's WNDCLASS
pub const COLOR_SCROLLBAR: ::c_int = 0;
pub const COLOR_BACKGROUND: ::c_int = 1;
pub const COLOR_ACTIVECAPTION: ::c_int = 2;
pub const COLOR_INACTIVECAPTION: ::c_int = 3;
pub const COLOR_MENU: ::c_int = 4;
pub const COLOR_WINDOW: ::c_int = 5;
pub const COLOR_WINDOWFRAME: ::c_int = 6;
pub const COLOR_MENUTEXT: ::c_int = 7;
pub const COLOR_WINDOWTEXT: ::c_int = 8;
pub const COLOR_CAPTIONTEXT: ::c_int = 9;
pub const COLOR_ACTIVEBORDER: ::c_int = 10;
pub const COLOR_INACTIVEBORDER: ::c_int = 11;
pub const COLOR_APPWORKSPACE: ::c_int = 12;
pub const COLOR_HIGHLIGHT: ::c_int = 13;
pub const COLOR_HIGHLIGHTTEXT: ::c_int = 14;
pub const COLOR_BTNFACE: ::c_int = 15;
pub const COLOR_BTNSHADOW: ::c_int = 16;
pub const COLOR_GRAYTEXT: ::c_int = 17;
pub const COLOR_BTNTEXT: ::c_int = 18;
pub const COLOR_INACTIVECAPTIONTEXT: ::c_int = 19;
pub const COLOR_BTNHIGHLIGHT: ::c_int = 20;
// Introduced in Windows 95 (winver 0x0400):
pub const COLOR_3DDKSHADOW: ::c_int = 21;
pub const COLOR_3DLIGHT: ::c_int = 22;
pub const COLOR_INFOTEXT: ::c_int = 23;
pub const COLOR_INFOBK: ::c_int = 24;
pub const COLOR_DESKTOP: ::c_int = COLOR_BACKGROUND;
pub const COLOR_3DFACE: ::c_int = COLOR_BTNFACE;
pub const COLOR_3DSHADOW: ::c_int = COLOR_BTNSHADOW;
pub const COLOR_3DHIGHLIGHT: ::c_int = COLOR_BTNHIGHLIGHT;
pub const COLOR_3DHILIGHT: ::c_int = COLOR_BTNHIGHLIGHT;
pub const COLOR_BTNHILIGHT: ::c_int = COLOR_BTNHIGHLIGHT;
// Introduced in Windows 2000 (winver 0x0500)
pub const COLOR_HOTLIGHT: ::c_int = 26;
pub const COLOR_GRADIENTACTIVECAPTION: ::c_int = 27;
pub const COLOR_GRADIENTINACTIVECAPTION: ::c_int = 28;
// Introduced in Windows XP (winver 0x0501)
pub const COLOR_MENUHILIGHT: ::c_int = 29;
pub const COLOR_MENUBAR: ::c_int = 30;

//10069
pub const IDC_ARROW: ::LPCWSTR = 32512 as ::LPCWSTR;
pub const IDC_IBEAM: ::LPCWSTR = 32513 as ::LPCWSTR;
pub const IDC_WAIT: ::LPCWSTR = 32514 as ::LPCWSTR;
pub const IDC_CROSS: ::LPCWSTR = 32515 as ::LPCWSTR;
pub const IDC_UPARROW: ::LPCWSTR = 32516 as ::LPCWSTR;
pub const IDC_SIZE: ::LPCWSTR = 32640 as ::LPCWSTR;
pub const IDC_ICON: ::LPCWSTR = 32641 as ::LPCWSTR;
pub const IDC_SIZENWSE: ::LPCWSTR = 32642 as ::LPCWSTR;
pub const IDC_SIZENESW: ::LPCWSTR = 32643 as ::LPCWSTR;
pub const IDC_SIZEWE: ::LPCWSTR = 32644 as ::LPCWSTR;
pub const IDC_SIZENS: ::LPCWSTR = 32645 as ::LPCWSTR;
pub const IDC_SIZEALL: ::LPCWSTR = 32646 as ::LPCWSTR;
pub const IDC_NO: ::LPCWSTR = 32648 as ::LPCWSTR;
pub const IDC_HAND: ::LPCWSTR = 32649 as ::LPCWSTR;
pub const IDC_APPSTARTING: ::LPCWSTR = 32650 as ::LPCWSTR;
pub const IDC_HELP: ::LPCWSTR = 32651 as ::LPCWSTR;

//10492
pub const IDI_APPLICATION: ::LPCWSTR = 32512 as ::LPCWSTR;
pub const IDI_HAND: ::LPCWSTR = 32513 as ::LPCWSTR;
pub const IDI_QUESTION: ::LPCWSTR = 32514 as ::LPCWSTR;
pub const IDI_EXCLAMATION: ::LPCWSTR = 32515 as ::LPCWSTR;
pub const IDI_ASTERISK: ::LPCWSTR = 32516 as ::LPCWSTR;
pub const IDI_WINLOGO: ::LPCWSTR = 32517 as ::LPCWSTR;
pub const IDI_SHIELD: ::LPCWSTR = 32518 as ::LPCWSTR;

pub const IDI_WARNING: ::LPCWSTR = IDI_EXCLAMATION;
pub const IDI_ERROR: ::LPCWSTR = IDI_HAND;
pub const IDI_INFORMATION: ::LPCWSTR = IDI_ASTERISK;

pub const SPI_GETBEEP: ::UINT = 0x0001;
pub const SPI_SETBEEP: ::UINT = 0x0002;
pub const SPI_GETMOUSE: ::UINT = 0x0003;
pub const SPI_SETMOUSE: ::UINT = 0x0004;
pub const SPI_GETBORDER: ::UINT = 0x0005;
pub const SPI_SETBORDER: ::UINT = 0x0006;
pub const SPI_GETKEYBOARDSPEED: ::UINT = 0x000A;
pub const SPI_SETKEYBOARDSPEED: ::UINT = 0x000B;
pub const SPI_LANGDRIVER: ::UINT = 0x000C;
pub const SPI_ICONHORIZONTALSPACING: ::UINT = 0x000D;
pub const SPI_GETSCREENSAVETIMEOUT: ::UINT = 0x000E;
pub const SPI_SETSCREENSAVETIMEOUT: ::UINT = 0x000F;
pub const SPI_GETSCREENSAVEACTIVE: ::UINT = 0x0010;
pub const SPI_SETSCREENSAVEACTIVE: ::UINT = 0x0011;
pub const SPI_GETGRIDGRANULARITY: ::UINT = 0x0012;
pub const SPI_SETGRIDGRANULARITY: ::UINT = 0x0013;
pub const SPI_SETDESKWALLPAPER: ::UINT = 0x0014;
pub const SPI_SETDESKPATTERN: ::UINT = 0x0015;
pub const SPI_GETKEYBOARDDELAY: ::UINT = 0x0016;
pub const SPI_SETKEYBOARDDELAY: ::UINT = 0x0017;
pub const SPI_ICONVERTICALSPACING: ::UINT = 0x0018;
pub const SPI_GETICONTITLEWRAP: ::UINT = 0x0019;
pub const SPI_SETICONTITLEWRAP: ::UINT = 0x001A;
pub const SPI_GETMENUDROPALIGNMENT: ::UINT = 0x001B;
pub const SPI_SETMENUDROPALIGNMENT: ::UINT = 0x001C;
pub const SPI_SETDOUBLECLKWIDTH: ::UINT = 0x001D;
pub const SPI_SETDOUBLECLKHEIGHT: ::UINT = 0x001E;
pub const SPI_GETICONTITLELOGFONT: ::UINT = 0x001F;
pub const SPI_SETDOUBLECLICKTIME: ::UINT = 0x0020;
pub const SPI_SETMOUSEBUTTONSWAP: ::UINT = 0x0021;
pub const SPI_SETICONTITLELOGFONT: ::UINT = 0x0022;
pub const SPI_GETFASTTASKSWITCH: ::UINT = 0x0023;
pub const SPI_SETFASTTASKSWITCH: ::UINT = 0x0024;
pub const SPI_SETDRAGFULLWINDOWS: ::UINT = 0x0025;
pub const SPI_GETDRAGFULLWINDOWS: ::UINT = 0x0026;
pub const SPI_GETNONCLIENTMETRICS: ::UINT = 0x0029;
pub const SPI_SETNONCLIENTMETRICS: ::UINT = 0x002A;
pub const SPI_GETMINIMIZEDMETRICS: ::UINT = 0x002B;
pub const SPI_SETMINIMIZEDMETRICS: ::UINT = 0x002C;
pub const SPI_GETICONMETRICS: ::UINT = 0x002D;
pub const SPI_SETICONMETRICS: ::UINT = 0x002E;
pub const SPI_SETWORKAREA: ::UINT = 0x002F;
pub const SPI_GETWORKAREA: ::UINT = 0x0030;
pub const SPI_SETPENWINDOWS: ::UINT = 0x0031;

pub const SPI_GETHIGHCONTRAST: ::UINT = 0x0042;
pub const SPI_SETHIGHCONTRAST: ::UINT = 0x0043;
pub const SPI_GETKEYBOARDPREF: ::UINT = 0x0044;
pub const SPI_SETKEYBOARDPREF: ::UINT = 0x0045;
pub const SPI_GETSCREENREADER: ::UINT = 0x0046;
pub const SPI_SETSCREENREADER: ::UINT = 0x0047;
pub const SPI_GETANIMATION: ::UINT = 0x0048;
pub const SPI_SETANIMATION: ::UINT = 0x0049;
pub const SPI_GETFONTSMOOTHING: ::UINT = 0x004A;
pub const SPI_SETFONTSMOOTHING: ::UINT = 0x004B;
pub const SPI_SETDRAGWIDTH: ::UINT = 0x004C;
pub const SPI_SETDRAGHEIGHT: ::UINT = 0x004D;
pub const SPI_SETHANDHELD: ::UINT = 0x004E;
pub const SPI_GETLOWPOWERTIMEOUT: ::UINT = 0x004F;
pub const SPI_GETPOWEROFFTIMEOUT: ::UINT = 0x0050;
pub const SPI_SETLOWPOWERTIMEOUT: ::UINT = 0x0051;
pub const SPI_SETPOWEROFFTIMEOUT: ::UINT = 0x0052;
pub const SPI_GETLOWPOWERACTIVE: ::UINT = 0x0053;
pub const SPI_GETPOWEROFFACTIVE: ::UINT = 0x0054;
pub const SPI_SETLOWPOWERACTIVE: ::UINT = 0x0055;
pub const SPI_SETPOWEROFFACTIVE: ::UINT = 0x0056;
pub const SPI_SETCURSORS: ::UINT = 0x0057;
pub const SPI_SETICONS: ::UINT = 0x0058;
pub const SPI_GETDEFAULTINPUTLANG: ::UINT = 0x0059;
pub const SPI_SETDEFAULTINPUTLANG: ::UINT = 0x005A;
pub const SPI_SETLANGTOGGLE: ::UINT = 0x005B;
pub const SPI_GETWINDOWSEXTENSION: ::UINT = 0x005C;
pub const SPI_SETMOUSETRAILS: ::UINT = 0x005D;
pub const SPI_GETMOUSETRAILS: ::UINT = 0x005E;
pub const SPI_SETSCREENSAVERRUNNING: ::UINT = 0x0061;
pub const SPI_SCREENSAVERRUNNING: ::UINT = SPI_SETSCREENSAVERRUNNING;
pub const SPI_GETFILTERKEYS: ::UINT = 0x0032;
pub const SPI_SETFILTERKEYS: ::UINT = 0x0033;
pub const SPI_GETTOGGLEKEYS: ::UINT = 0x0034;
pub const SPI_SETTOGGLEKEYS: ::UINT = 0x0035;
pub const SPI_GETMOUSEKEYS: ::UINT = 0x0036;
pub const SPI_SETMOUSEKEYS: ::UINT = 0x0037;
pub const SPI_GETSHOWSOUNDS: ::UINT = 0x0038;
pub const SPI_SETSHOWSOUNDS: ::UINT = 0x0039;
pub const SPI_GETSTICKYKEYS: ::UINT = 0x003A;
pub const SPI_SETSTICKYKEYS: ::UINT = 0x003B;
pub const SPI_GETACCESSTIMEOUT: ::UINT = 0x003C;
pub const SPI_SETACCESSTIMEOUT: ::UINT = 0x003D;
pub const SPI_GETSERIALKEYS: ::UINT = 0x003E;
pub const SPI_SETSERIALKEYS: ::UINT = 0x003F;
pub const SPI_GETSOUNDSENTRY: ::UINT = 0x0040;
pub const SPI_SETSOUNDSENTRY: ::UINT = 0x0041;
pub const SPI_GETSNAPTODEFBUTTON: ::UINT = 0x005F;
pub const SPI_SETSNAPTODEFBUTTON: ::UINT = 0x0060;
pub const SPI_GETMOUSEHOVERWIDTH: ::UINT = 0x0062;
pub const SPI_SETMOUSEHOVERWIDTH: ::UINT = 0x0063;
pub const SPI_GETMOUSEHOVERHEIGHT: ::UINT = 0x0064;
pub const SPI_SETMOUSEHOVERHEIGHT: ::UINT = 0x0065;
pub const SPI_GETMOUSEHOVERTIME: ::UINT = 0x0066;
pub const SPI_SETMOUSEHOVERTIME: ::UINT = 0x0067;
pub const SPI_GETWHEELSCROLLLINES: ::UINT = 0x0068;
pub const SPI_SETWHEELSCROLLLINES: ::UINT = 0x0069;
pub const SPI_GETMENUSHOWDELAY: ::UINT = 0x006A;
pub const SPI_SETMENUSHOWDELAY: ::UINT = 0x006B;

pub const SPI_GETWHEELSCROLLCHARS: ::UINT = 0x006C;
pub const SPI_SETWHEELSCROLLCHARS: ::UINT = 0x006D;

pub const SPI_GETSHOWIMEUI: ::UINT = 0x006E;
pub const SPI_SETSHOWIMEUI: ::UINT = 0x006F;

pub const SPI_GETMOUSESPEED: ::UINT = 0x0070;
pub const SPI_SETMOUSESPEED: ::UINT = 0x0071;
pub const SPI_GETSCREENSAVERRUNNING: ::UINT = 0x0072;
pub const SPI_GETDESKWALLPAPER: ::UINT = 0x0073;

pub const SPI_GETAUDIODESCRIPTION: ::UINT = 0x0074;
pub const SPI_SETAUDIODESCRIPTION: ::UINT = 0x0075;

pub const SPI_GETSCREENSAVESECURE: ::UINT = 0x0076;
pub const SPI_SETSCREENSAVESECURE: ::UINT = 0x0077;

pub const SPI_GETHUNGAPPTIMEOUT: ::UINT = 0x0078;
pub const SPI_SETHUNGAPPTIMEOUT: ::UINT = 0x0079;
pub const SPI_GETWAITTOKILLTIMEOUT: ::UINT = 0x007A;
pub const SPI_SETWAITTOKILLTIMEOUT: ::UINT = 0x007B;
pub const SPI_GETWAITTOKILLSERVICETIMEOUT: ::UINT = 0x007C;
pub const SPI_SETWAITTOKILLSERVICETIMEOUT: ::UINT = 0x007D;
pub const SPI_GETMOUSEDOCKTHRESHOLD: ::UINT = 0x007E;
pub const SPI_SETMOUSEDOCKTHRESHOLD: ::UINT = 0x007F;
pub const SPI_GETPENDOCKTHRESHOLD: ::UINT = 0x0080;
pub const SPI_SETPENDOCKTHRESHOLD: ::UINT = 0x0081;
pub const SPI_GETWINARRANGING: ::UINT = 0x0082;
pub const SPI_SETWINARRANGING: ::UINT = 0x0083;
pub const SPI_GETMOUSEDRAGOUTTHRESHOLD: ::UINT = 0x0084;
pub const SPI_SETMOUSEDRAGOUTTHRESHOLD: ::UINT = 0x0085;
pub const SPI_GETPENDRAGOUTTHRESHOLD: ::UINT = 0x0086;
pub const SPI_SETPENDRAGOUTTHRESHOLD: ::UINT = 0x0087;
pub const SPI_GETMOUSESIDEMOVETHRESHOLD: ::UINT = 0x0088;
pub const SPI_SETMOUSESIDEMOVETHRESHOLD: ::UINT = 0x0089;
pub const SPI_GETPENSIDEMOVETHRESHOLD: ::UINT = 0x008A;
pub const SPI_SETPENSIDEMOVETHRESHOLD: ::UINT = 0x008B;
pub const SPI_GETDRAGFROMMAXIMIZE: ::UINT = 0x008C;
pub const SPI_SETDRAGFROMMAXIMIZE: ::UINT = 0x008D;
pub const SPI_GETSNAPSIZING: ::UINT = 0x008E;
pub const SPI_SETSNAPSIZING: ::UINT = 0x008F;
pub const SPI_GETDOCKMOVING: ::UINT = 0x0090;
pub const SPI_SETDOCKMOVING: ::UINT = 0x0091;

pub const SPI_GETACTIVEWINDOWTRACKING: ::UINT = 0x1000;
pub const SPI_SETACTIVEWINDOWTRACKING: ::UINT = 0x1001;
pub const SPI_GETMENUANIMATION: ::UINT = 0x1002;
pub const SPI_SETMENUANIMATION: ::UINT = 0x1003;
pub const SPI_GETCOMBOBOXANIMATION: ::UINT = 0x1004;
pub const SPI_SETCOMBOBOXANIMATION: ::UINT = 0x1005;
pub const SPI_GETLISTBOXSMOOTHSCROLLING: ::UINT = 0x1006;
pub const SPI_SETLISTBOXSMOOTHSCROLLING: ::UINT = 0x1007;
pub const SPI_GETGRADIENTCAPTIONS: ::UINT = 0x1008;
pub const SPI_SETGRADIENTCAPTIONS: ::UINT = 0x1009;
pub const SPI_GETKEYBOARDCUES: ::UINT = 0x100A;
pub const SPI_SETKEYBOARDCUES: ::UINT = 0x100B;
pub const SPI_GETMENUUNDERLINES: ::UINT = SPI_GETKEYBOARDCUES;
pub const SPI_SETMENUUNDERLINES: ::UINT = SPI_SETKEYBOARDCUES;
pub const SPI_GETACTIVEWNDTRKZORDER: ::UINT = 0x100C;
pub const SPI_SETACTIVEWNDTRKZORDER: ::UINT = 0x100D;
pub const SPI_GETHOTTRACKING: ::UINT = 0x100E;
pub const SPI_SETHOTTRACKING: ::UINT = 0x100F;
pub const SPI_GETMENUFADE: ::UINT = 0x1012;
pub const SPI_SETMENUFADE: ::UINT = 0x1013;
pub const SPI_GETSELECTIONFADE: ::UINT = 0x1014;
pub const SPI_SETSELECTIONFADE: ::UINT = 0x1015;
pub const SPI_GETTOOLTIPANIMATION: ::UINT = 0x1016;
pub const SPI_SETTOOLTIPANIMATION: ::UINT = 0x1017;
pub const SPI_GETTOOLTIPFADE: ::UINT = 0x1018;
pub const SPI_SETTOOLTIPFADE: ::UINT = 0x1019;
pub const SPI_GETCURSORSHADOW: ::UINT = 0x101A;
pub const SPI_SETCURSORSHADOW: ::UINT = 0x101B;
pub const SPI_GETMOUSESONAR: ::UINT = 0x101C;
pub const SPI_SETMOUSESONAR: ::UINT = 0x101D;
pub const SPI_GETMOUSECLICKLOCK: ::UINT = 0x101E;
pub const SPI_SETMOUSECLICKLOCK: ::UINT = 0x101F;
pub const SPI_GETMOUSEVANISH: ::UINT = 0x1020;
pub const SPI_SETMOUSEVANISH: ::UINT = 0x1021;
pub const SPI_GETFLATMENU: ::UINT = 0x1022;
pub const SPI_SETFLATMENU: ::UINT = 0x1023;
pub const SPI_GETDROPSHADOW: ::UINT = 0x1024;
pub const SPI_SETDROPSHADOW: ::UINT = 0x1025;
pub const SPI_GETBLOCKSENDINPUTRESETS: ::UINT = 0x1026;
pub const SPI_SETBLOCKSENDINPUTRESETS: ::UINT = 0x1027;

pub const SPI_GETUIEFFECTS: ::UINT = 0x103E;
pub const SPI_SETUIEFFECTS: ::UINT = 0x103F;

pub const SPI_GETDISABLEOVERLAPPEDCONTENT: ::UINT = 0x1040;
pub const SPI_SETDISABLEOVERLAPPEDCONTENT: ::UINT = 0x1041;
pub const SPI_GETCLIENTAREAANIMATION: ::UINT = 0x1042;
pub const SPI_SETCLIENTAREAANIMATION: ::UINT = 0x1043;
pub const SPI_GETCLEARTYPE: ::UINT = 0x1048;
pub const SPI_SETCLEARTYPE: ::UINT = 0x1049;
pub const SPI_GETSPEECHRECOGNITION: ::UINT = 0x104A;
pub const SPI_SETSPEECHRECOGNITION: ::UINT = 0x104B;

pub const SPI_GETFOREGROUNDLOCKTIMEOUT: ::UINT = 0x2000;
pub const SPI_SETFOREGROUNDLOCKTIMEOUT: ::UINT = 0x2001;
pub const SPI_GETACTIVEWNDTRKTIMEOUT: ::UINT = 0x2002;
pub const SPI_SETACTIVEWNDTRKTIMEOUT: ::UINT = 0x2003;
pub const SPI_GETFOREGROUNDFLASHCOUNT: ::UINT = 0x2004;
pub const SPI_SETFOREGROUNDFLASHCOUNT: ::UINT = 0x2005;
pub const SPI_GETCARETWIDTH: ::UINT = 0x2006;
pub const SPI_SETCARETWIDTH: ::UINT = 0x2007;

pub const SPI_GETMOUSECLICKLOCKTIME: ::UINT = 0x2008;
pub const SPI_SETMOUSECLICKLOCKTIME: ::UINT = 0x2009;
pub const SPI_GETFONTSMOOTHINGTYPE: ::UINT = 0x200A;
pub const SPI_SETFONTSMOOTHINGTYPE: ::UINT = 0x200B;

pub const FE_FONTSMOOTHINGSTANDARD: ::UINT = 0x0001;
pub const FE_FONTSMOOTHINGCLEARTYPE: ::UINT = 0x0002;

pub const SPI_GETFONTSMOOTHINGCONTRAST: ::UINT = 0x200C;
pub const SPI_SETFONTSMOOTHINGCONTRAST: ::UINT = 0x200D;

pub const SPI_GETFOCUSBORDERWIDTH: ::UINT = 0x200E;
pub const SPI_SETFOCUSBORDERWIDTH: ::UINT = 0x200F;
pub const SPI_GETFOCUSBORDERHEIGHT: ::UINT = 0x2010;
pub const SPI_SETFOCUSBORDERHEIGHT: ::UINT = 0x2011;

pub const SPI_GETFONTSMOOTHINGORIENTATION: ::UINT = 0x2012;
pub const SPI_SETFONTSMOOTHINGORIENTATION: ::UINT = 0x2013;

pub const FE_FONTSMOOTHINGORIENTATIONBGR: ::UINT = 0x0000;
pub const FE_FONTSMOOTHINGORIENTATIONRGB: ::UINT = 0x0001;

pub const SPI_GETMINIMUMHITRADIUS: ::UINT = 0x2014;
pub const SPI_SETMINIMUMHITRADIUS: ::UINT = 0x2015;
pub const SPI_GETMESSAGEDURATION: ::UINT = 0x2016;
pub const SPI_SETMESSAGEDURATION: ::UINT = 0x2017;

//12141
#[repr(C)] #[derive(Clone, Copy)]
pub struct NONCLIENTMETRICSA {
    pub cbSize: ::UINT,
    pub iBorderWidth: ::c_int,
    pub iScrollWidth: ::c_int,
    pub iScrollHeight: ::c_int,
    pub iCaptionWidth: ::c_int,
    pub iCaptionHeight: ::c_int,
    pub lfCaptionFont: ::LOGFONTA,
    pub iSmCaptionWidth: ::c_int,
    pub iSmCaptionHeight: ::c_int,
    pub lfSmCaptionFont: ::LOGFONTA,
    pub iMenuWidth: ::c_int,
    pub iMenuHeight: ::c_int,
    pub lfMenuFont: ::LOGFONTA,
    pub lfStatusFont: ::LOGFONTA,
    pub lfMessageFont: ::LOGFONTA,
    pub iPaddedBorderWidth: ::c_int,
}
pub type LPNONCLIENTMETRICSA = *mut NONCLIENTMETRICSA;

#[repr(C)] #[derive(Clone, Copy)]
pub struct NONCLIENTMETRICSW {
    pub cbSize: ::UINT,
    pub iBorderWidth: ::c_int,
    pub iScrollWidth: ::c_int,
    pub iScrollHeight: ::c_int,
    pub iCaptionWidth: ::c_int,
    pub iCaptionHeight: ::c_int,
    pub lfCaptionFont: ::LOGFONTW,
    pub iSmCaptionWidth: ::c_int,
    pub iSmCaptionHeight: ::c_int,
    pub lfSmCaptionFont: ::LOGFONTW,
    pub iMenuWidth: ::c_int,
    pub iMenuHeight: ::c_int,
    pub lfMenuFont: ::LOGFONTW,
    pub lfStatusFont: ::LOGFONTW,
    pub lfMessageFont: ::LOGFONTW,
    pub iPaddedBorderWidth: ::c_int,
}
pub type LPNONCLIENTMETRICSW = *mut NONCLIENTMETRICSW;

//12900
pub const MONITORINFOF_PRIMARY: ::DWORD = 1;
pub const CCHDEVICENAME: usize = 32;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct MONITORINFO {
    pub cbSize: ::DWORD,
    pub rcMonitor: ::RECT,
    pub rcWork: ::RECT,
    pub dwFlags: ::DWORD,
}
pub type LPMONITORINFO = *mut MONITORINFO;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct MONITORINFOEXA {
    pub cbSize: ::DWORD,
    pub rcMonitor: ::RECT,
    pub rcWork: ::RECT,
    pub dwFlags: ::DWORD,
    pub szDevice: [::CHAR; ::CCHDEVICENAME],
}
pub type LPMONITORINFOEXA = *mut MONITORINFOEXA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct MONITORINFOEXW {
    pub cbSize: ::DWORD,
    pub rcMonitor: ::RECT,
    pub rcWork: ::RECT,
    pub dwFlags: ::DWORD,
    pub szDevice: [::WCHAR; ::CCHDEVICENAME],
}
pub type LPMONITORINFOEXW = *mut MONITORINFOEXW;
//12971
pub type MONITORENUMPROC = Option<unsafe extern "system" fn(
    ::HMONITOR, ::HDC, ::LPRECT, ::LPARAM,
) -> ::BOOL>;
//14098
DECLARE_HANDLE!(HRAWINPUT, HRAWINPUT__);
pub fn GET_RAWINPUT_CODE_WPARAM(wParam: ::WPARAM) -> ::WPARAM { wParam & 0xff }
pub const RIM_INPUT: ::WPARAM = 0;
pub const RIM_INPUTSINK: ::WPARAM = 1;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct RAWINPUTHEADER {
    pub dwType: ::DWORD,
    pub dwSize: ::DWORD,
    pub hDevice: ::HANDLE,
    pub wParam: ::WPARAM,
}
pub type PRAWINPUTHEADER = *mut RAWINPUTHEADER;
pub type LPRAWINPUTHEADER = *mut RAWINPUTHEADER;
pub const RIM_TYPEMOUSE: ::DWORD = 0;
pub const RIM_TYPEKEYBOARD: ::DWORD = 1;
pub const RIM_TYPEHID: ::DWORD = 2;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct RAWMOUSE {
    pub usFlags: ::USHORT,
    pub memory_padding: ::USHORT, // 16bit Padding for 32bit align in following union
    pub usButtonFlags: ::USHORT,
    pub usButtonData: ::USHORT,
    pub ulRawButtons: ::ULONG,
    pub lLastX: ::LONG,
    pub lLastY: ::LONG,
    pub ulExtraInformation: ::ULONG,
}
pub type PRAWMOUSE = *mut RAWMOUSE;
pub type LPRAWMOUSE = *mut RAWMOUSE;
pub const RI_MOUSE_LEFT_BUTTON_DOWN: ::USHORT = 0x0001;
pub const RI_MOUSE_LEFT_BUTTON_UP: ::USHORT = 0x0002;
pub const RI_MOUSE_RIGHT_BUTTON_DOWN: ::USHORT = 0x0004;
pub const RI_MOUSE_RIGHT_BUTTON_UP: ::USHORT = 0x0008;
pub const RI_MOUSE_MIDDLE_BUTTON_DOWN: ::USHORT = 0x0010;
pub const RI_MOUSE_MIDDLE_BUTTON_UP: ::USHORT = 0x0020;
pub const RI_MOUSE_BUTTON_1_DOWN: ::USHORT = RI_MOUSE_LEFT_BUTTON_DOWN;
pub const RI_MOUSE_BUTTON_1_UP: ::USHORT = RI_MOUSE_LEFT_BUTTON_UP;
pub const RI_MOUSE_BUTTON_2_DOWN: ::USHORT = RI_MOUSE_RIGHT_BUTTON_DOWN;
pub const RI_MOUSE_BUTTON_2_UP: ::USHORT = RI_MOUSE_RIGHT_BUTTON_UP;
pub const RI_MOUSE_BUTTON_3_DOWN: ::USHORT = RI_MOUSE_MIDDLE_BUTTON_DOWN;
pub const RI_MOUSE_BUTTON_3_UP: ::USHORT = RI_MOUSE_MIDDLE_BUTTON_UP;
pub const RI_MOUSE_BUTTON_4_DOWN: ::USHORT = 0x0040;
pub const RI_MOUSE_BUTTON_4_UP: ::USHORT = 0x0080;
pub const RI_MOUSE_BUTTON_5_DOWN: ::USHORT = 0x0100;
pub const RI_MOUSE_BUTTON_5_UP: ::USHORT = 0x0200;
pub const RI_MOUSE_WHEEL: ::USHORT = 0x0400;
pub const MOUSE_MOVE_RELATIVE: ::USHORT = 0;
pub const MOUSE_MOVE_ABSOLUTE: ::USHORT = 1;
pub const MOUSE_VIRTUAL_DESKTOP: ::USHORT = 0x02;
pub const MOUSE_ATTRIBUTES_CHANGED: ::USHORT = 0x04;
pub const MOUSE_MOVE_NOCOALESCE: ::USHORT = 0x08;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct RAWKEYBOARD {
    pub MakeCode: ::USHORT,
    pub Flags: ::USHORT,
    pub Reserved: ::USHORT,
    pub VKey: ::USHORT,
    pub Message: ::UINT,
    pub ExtraInformation: ::ULONG,
}
pub type PRAWKEYBOARD = *mut RAWKEYBOARD;
pub type LPRAWKEYBOARD = *mut RAWKEYBOARD;
pub const KEYBOARD_OVERRUN_MAKE_CODE: ::DWORD = 0xFF;
pub const RI_KEY_MAKE: ::DWORD = 0;
pub const RI_KEY_BREAK: ::DWORD = 1;
pub const RI_KEY_E0: ::DWORD = 2;
pub const RI_KEY_E1: ::DWORD = 4;
pub const RI_KEY_TERMSRV_SET_LED: ::DWORD = 8;
pub const RI_KEY_TERMSRV_SHADOW: ::DWORD = 0x10;
#[repr(C)] #[derive(Debug)] #[allow(missing_copy_implementations)]
pub struct RAWHID {
    pub dwSizeHid: ::DWORD,
    pub dwCount: ::DWORD,
    pub bRawData: [::BYTE; 0],
}
pub type PRAWHID = *mut RAWHID;
pub type LPRAWHID = *mut RAWHID;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct RAWINPUT {
    pub header: RAWINPUTHEADER,
    pub mouse: RAWMOUSE, // FIXME untagged union
}
#[test]
fn test_RAWINPUT() {
    use std::mem::{size_of, align_of};
    assert!(size_of::<RAWMOUSE>() >= size_of::<RAWMOUSE>());
    assert!(size_of::<RAWMOUSE>() >= size_of::<RAWKEYBOARD>());
    assert!(size_of::<RAWMOUSE>() >= size_of::<RAWHID>());
    assert!(align_of::<RAWMOUSE>() >= align_of::<RAWMOUSE>());
    assert!(align_of::<RAWMOUSE>() >= align_of::<RAWKEYBOARD>());
    assert!(align_of::<RAWMOUSE>() >= align_of::<RAWHID>());
}
pub type PRAWINPUT = *mut RAWINPUT;
pub type LPRAWINPUT = *mut RAWINPUT;
pub const RID_INPUT: ::DWORD = 0x10000003;
pub const RID_HEADER: ::DWORD = 0x10000005;
pub const RIDI_PREPARSEDDATA: ::DWORD = 0x20000005;
pub const RIDI_DEVICENAME: ::DWORD = 0x20000007;
pub const RIDI_DEVICEINFO: ::DWORD = 0x2000000b;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct RID_DEVICE_INFO_MOUSE {
    pub dwId: ::DWORD,
    pub dwNumberOfButtons: ::DWORD,
    pub dwSampleRate: ::DWORD,
    pub fHasHorizontalWheel: ::BOOL,
}
pub type PRID_DEVICE_INFO_MOUSE = *mut RID_DEVICE_INFO_MOUSE;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct RID_DEVICE_INFO_KEYBOARD {
    pub dwType: ::DWORD,
    pub dwSubType: ::DWORD,
    pub dwKeyboardMode: ::DWORD,
    pub dwNumberOfFunctionKeys: ::DWORD,
    pub dwNumberOfIndicators: ::DWORD,
    pub dwNumberOfKeysTotal: ::DWORD,
}
pub type PRID_DEVICE_INFO_KEYBOARD = *mut RID_DEVICE_INFO_KEYBOARD;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct RID_DEVICE_INFO_HID {
    pub dwVendorId: ::DWORD,
    pub dwProductId: ::DWORD,
    pub dwVersionNumber: ::DWORD,
    pub usUsagePage: ::USHORT,
    pub usUsage: ::USHORT,
}
pub type PRID_DEVICE_INFO_HID = *mut RID_DEVICE_INFO_HID;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct RID_DEVICE_INFO {
    pub cbSize: ::DWORD,
    pub dwType: ::DWORD,
    pub keyboard: RID_DEVICE_INFO_KEYBOARD, // FIXME untagged union
}
#[test]
fn test_RID_DEVICE_INFO() {
    use std::mem::{size_of, align_of};
    assert!(size_of::<RID_DEVICE_INFO_KEYBOARD>() >= size_of::<RID_DEVICE_INFO_MOUSE>());
    assert!(size_of::<RID_DEVICE_INFO_KEYBOARD>() >= size_of::<RID_DEVICE_INFO_KEYBOARD>());
    assert!(size_of::<RID_DEVICE_INFO_KEYBOARD>() >= size_of::<RID_DEVICE_INFO_HID>());
    assert!(align_of::<RID_DEVICE_INFO_KEYBOARD>() >= align_of::<RID_DEVICE_INFO_MOUSE>());
    assert!(align_of::<RID_DEVICE_INFO_KEYBOARD>()
        >= align_of::<RID_DEVICE_INFO_KEYBOARD>());
    assert!(align_of::<RID_DEVICE_INFO_KEYBOARD>() >= align_of::<RID_DEVICE_INFO_HID>());
}
pub type PRID_DEVICE_INFO = *mut RID_DEVICE_INFO;
pub type LPRID_DEVICE_INFO = *mut RID_DEVICE_INFO;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct RAWINPUTDEVICE {
    pub usUsagePage: ::USHORT,
    pub usUsage: ::USHORT,
    pub dwFlags: ::DWORD,
    pub hwndTarget: ::HWND,
}
pub type PRAWINPUTDEVICE = *mut RAWINPUTDEVICE;
pub type LPRAWINPUTDEVICE = *mut RAWINPUTDEVICE;
pub type PCRAWINPUTDEVICE = *const RAWINPUTDEVICE;
pub const RIDEV_REMOVE: ::DWORD = 0x00000001;
pub const RIDEV_EXCLUDE: ::DWORD = 0x00000010;
pub const RIDEV_PAGEONLY: ::DWORD = 0x00000020;
pub const RIDEV_NOLEGACY: ::DWORD = 0x00000030;
pub const RIDEV_INPUTSINK: ::DWORD = 0x00000100;
pub const RIDEV_CAPTUREMOUSE: ::DWORD = 0x00000200;
pub const RIDEV_NOHOTKEYS: ::DWORD = 0x00000200;
pub const RIDEV_APPKEYS: ::DWORD = 0x00000400;
pub const RIDEV_EXINPUTSINK: ::DWORD = 0x00001000;
pub const RIDEV_DEVNOTIFY: ::DWORD = 0x00002000;
pub const RIDEV_EXMODEMASK: ::DWORD = 0x000000F0;
pub const GIDC_ARRIVAL: ::DWORD = 1;
pub const GIDC_REMOVAL: ::DWORD = 2;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct RAWINPUTDEVICELIST {
    pub hDevice: ::HANDLE,
    pub dwType: ::DWORD,
}
pub type PRAWINPUTDEVICELIST = *mut RAWINPUTDEVICELIST;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct CHANGEFILTERSTRUCT {
    pub cbSize: ::DWORD,
    pub ExtStatus: ::DWORD,
}
pub type PCHANGEFILTERSTRUCT = *mut CHANGEFILTERSTRUCT;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DLGTEMPLATE {
    pub style: ::DWORD,
    pub dwExtendedStyle: ::DWORD,
    pub cdit: ::WORD,
    pub x: ::c_short,
    pub y: ::c_short,
    pub cx: ::c_short,
    pub cy: ::c_short,
}
pub type LPDLGTEMPLATEA = *mut DLGTEMPLATE;
pub type LPDLGTEMPLATEW = *mut DLGTEMPLATE;
pub type LPCDLGTEMPLATEA = *const DLGTEMPLATE;
pub type LPCDLGTEMPLATEW = *const DLGTEMPLATE;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DRAWTEXTPARAMS {
    pub cbSize: ::UINT,
    pub iTabLength: ::c_int,
    pub iLeftMargin: ::c_int,
    pub iRightMargin: ::c_int,
    pub uiLengthDrawn: ::UINT,
}
pub type LPDRAWTEXTPARAMS = *mut DRAWTEXTPARAMS;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ACCEL {
    pub fVirt: ::BYTE,
    pub key: ::WORD,
    pub cmd: ::WORD,
}
pub type LPACCEL = *mut ACCEL;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct MENUITEMINFOA {
    pub cbSize: ::UINT,
    pub fMask: ::UINT,
    pub fType: ::UINT,
    pub fState: ::UINT,
    pub wID: ::UINT,
    pub hSubMenu: ::HMENU,
    pub hbmpChecked: ::HBITMAP,
    pub hbmpUnchecked: ::HBITMAP,
    pub dwItemData: ::ULONG_PTR,
    pub dwTypeData: ::LPSTR,
    pub cch: ::UINT,
    pub hbmpItem: ::HBITMAP,
}
pub type LPMENUITEMINFOA = *mut MENUITEMINFOA;
pub type LPCMENUITEMINFOA = *const MENUITEMINFOA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct MENUITEMINFOW {
    pub cbSize: ::UINT,
    pub fMask: ::UINT,
    pub fType: ::UINT,
    pub fState: ::UINT,
    pub wID: ::UINT,
    pub hSubMenu: ::HMENU,
    pub hbmpChecked: ::HBITMAP,
    pub hbmpUnchecked: ::HBITMAP,
    pub dwItemData: ::ULONG_PTR,
    pub dwTypeData: ::LPWSTR,
    pub cch: ::UINT,
    pub hbmpItem: ::HBITMAP,
}
pub type LPMENUITEMINFOW = *mut MENUITEMINFOW;
pub type LPCMENUITEMINFOW = *const MENUITEMINFOW;
#[repr(C)] #[derive(Copy)]
pub struct MSGBOXPARAMSA {
    pub cbSize: ::UINT,
    pub hwndOwner: ::HWND,
    pub hInstance: ::HINSTANCE,
    pub lpszText: ::LPCSTR,
    pub lpszCaption: ::LPCSTR,
    pub dwStyle: ::DWORD,
    pub lpszIcon: ::LPCSTR,
    pub dwContextHelpId: ::DWORD_PTR,
    pub lpfnMsgBoxCallback: ::MSGBOXCALLBACK,
    pub dwLanguageId: ::DWORD,
}
impl Clone for MSGBOXPARAMSA { fn clone(&self) -> MSGBOXPARAMSA { *self } }
pub type PMSGBOXPARAMSA = *mut MSGBOXPARAMSA;
pub type LPMSGBOXPARAMSA = *mut MSGBOXPARAMSA;
#[repr(C)] #[derive(Copy)]
pub struct MSGBOXPARAMSW {
    pub cbSize: ::UINT,
    pub hwndOwner: ::HWND,
    pub hInstance: ::HINSTANCE,
    pub lpszText: ::LPCWSTR,
    pub lpszCaption: ::LPCWSTR,
    pub dwStyle: ::DWORD,
    pub lpszIcon: ::LPCWSTR,
    pub dwContextHelpId: ::DWORD_PTR,
    pub lpfnMsgBoxCallback: ::MSGBOXCALLBACK,
    pub dwLanguageId: ::DWORD,
}
impl Clone for MSGBOXPARAMSW { fn clone(&self) -> MSGBOXPARAMSW { *self } }
pub type PMSGBOXPARAMSW = *mut MSGBOXPARAMSW;
pub type LPMSGBOXPARAMSW = *mut MSGBOXPARAMSW;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HELPINFO {
    pub cbSize: ::UINT,
    pub iContextType: ::c_int,
    pub iCtrlId: ::c_int,
    pub hItemHandle: ::HANDLE,
    pub dwContextId: ::DWORD,
    pub MousePos: ::POINT,
}
pub type LPHELPINFO = *mut HELPINFO;

pub fn GET_WHEEL_DELTA_WPARAM(wParam: ::WPARAM) -> ::c_short {
    ::HIWORD(wParam as ::DWORD) as ::c_short
}
pub fn GET_KEYSTATE_WPARAM(wparam: ::WPARAM) -> ::c_int {
    ::LOWORD(wparam as ::DWORD) as ::c_short as ::c_int
}

pub const SIF_RANGE: ::UINT = 0x0001;
pub const SIF_PAGE: ::UINT = 0x0002;
pub const SIF_POS: ::UINT = 0x0004;
pub const SIF_DISABLENOSCROLL: ::UINT = 0x0008;
pub const SIF_TRACKPOS: ::UINT = 0x0010;
pub const SIF_ALL: ::UINT = SIF_RANGE | SIF_PAGE | SIF_POS | SIF_TRACKPOS;

pub const SW_SCROLLCHILDREN: ::UINT = 0x0001;
pub const SW_INVALIDATE: ::UINT = 0x0002;
pub const SW_ERASE: ::UINT = 0x0004;
pub const SW_SMOOTHSCROLL: ::UINT = 0x0010;

pub const SB_LINEUP: ::c_int = 0;
pub const SB_LINELEFT: ::c_int = 0;
pub const SB_LINEDOWN: ::c_int = 1;
pub const SB_LINERIGHT: ::c_int = 1;
pub const SB_PAGEUP: ::c_int = 2;
pub const SB_PAGELEFT: ::c_int = 2;
pub const SB_PAGEDOWN: ::c_int = 3;
pub const SB_PAGERIGHT: ::c_int = 3;
pub const SB_THUMBPOSITION: ::c_int = 4;
pub const SB_THUMBTRACK: ::c_int = 5;
pub const SB_TOP: ::c_int = 6;
pub const SB_LEFT: ::c_int = 6;
pub const SB_BOTTOM: ::c_int = 7;
pub const SB_RIGHT: ::c_int = 7;
pub const SB_ENDSCROLL: ::c_int = 8;

pub const LR_DEFAULTCOLOR: ::UINT = 0x00000000;
pub const LR_MONOCHROME: ::UINT = 0x00000001;
pub const LR_COLOR: ::UINT = 0x00000002;
pub const LR_COPYRETURNORG: ::UINT = 0x00000004;
pub const LR_COPYDELETEORG: ::UINT = 0x00000008;
pub const LR_LOADFROMFILE: ::UINT = 0x00000010;
pub const LR_LOADTRANSPARENT: ::UINT = 0x00000020;
pub const LR_DEFAULTSIZE: ::UINT = 0x00000040;
pub const LR_VGACOLOR: ::UINT = 0x00000080;
pub const LR_LOADMAP3DCOLORS: ::UINT = 0x00001000;
pub const LR_CREATEDIBSECTION: ::UINT = 0x00002000;
pub const LR_COPYFROMRESOURCE: ::UINT = 0x00004000;
pub const LR_SHARED: ::UINT = 0x00008000;

pub const IMAGE_BITMAP: ::UINT = 0;
pub const IMAGE_ICON: ::UINT = 1;
pub const IMAGE_CURSOR: ::UINT = 2;
pub const IMAGE_ENHMETAFILE: ::UINT = 3;

pub const DT_TOP: ::UINT = 0x00000000;
pub const DT_LEFT: ::UINT = 0x00000000;
pub const DT_CENTER: ::UINT = 0x00000001;
pub const DT_RIGHT: ::UINT = 0x00000002;
pub const DT_VCENTER: ::UINT = 0x00000004;
pub const DT_BOTTOM: ::UINT = 0x00000008;
pub const DT_WORDBREAK: ::UINT = 0x00000010;
pub const DT_SINGLELINE: ::UINT = 0x00000020;
pub const DT_EXPANDTABS: ::UINT = 0x00000040;
pub const DT_TABSTOP: ::UINT = 0x00000080;
pub const DT_NOCLIP: ::UINT = 0x00000100;
pub const DT_EXTERNALLEADING: ::UINT = 0x00000200;
pub const DT_CALCRECT: ::UINT = 0x00000400;
pub const DT_NOPREFIX: ::UINT = 0x00000800;
pub const DT_INTERNAL: ::UINT = 0x00001000;

pub const DT_EDITCONTROL: ::UINT = 0x00002000;
pub const DT_PATH_ELLIPSIS: ::UINT = 0x00004000;
pub const DT_END_ELLIPSIS: ::UINT = 0x00008000;
pub const DT_MODIFYSTRING: ::UINT = 0x00010000;
pub const DT_RTLREADING: ::UINT = 0x00020000;
pub const DT_WORD_ELLIPSIS: ::UINT = 0x00040000;
pub const DT_NOFULLWIDTHCHARBREAK: ::UINT = 0x00080000;
pub const DT_HIDEPREFIX: ::UINT = 0x00100000;
pub const DT_PREFIXONLY: ::UINT = 0x00200000;

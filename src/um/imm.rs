// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use ctypes::c_int;
use shared::minwindef::{BOOL, DWORD, HKL, LPARAM, LPDWORD, LPVOID, LRESULT, UINT, WPARAM};
use shared::windef::{HBITMAP, HWND, LPPOINT, POINT, RECT};
use um::ime_cmodes::IME_CMODE_NATIVE;
use um::winnt::{CHAR, LONG, LPCSTR, LPCWSTR, LPSTR, LPWSTR, WCHAR};
use um::winuser::{
    MFS_CHECKED, MFS_DEFAULT, MFS_DISABLED, MFS_ENABLED, MFS_GRAYED, MFS_HILITE, MFS_UNCHECKED,
    MFS_UNHILITE
};
DECLARE_HANDLE!{HIMC, HIMC__}
DECLARE_HANDLE!{HIMCC, HIMCC__}
pub type LPHKL = *mut HKL;
pub type LPUINT = *mut UINT;
STRUCT!{struct COMPOSITIONFORM {
    dwStyle: DWORD,
    ptCurrentPos: POINT,
    rcArea: RECT,
}}
pub type LPCOMPOSITIONFORM = *mut COMPOSITIONFORM;
STRUCT!{struct CANDIDATEFORM {
    dwIndex: DWORD,
    dwStyle: DWORD,
    ptCurrentPos: POINT,
    rcArea: RECT,
}}
pub type LPCANDIDATEFORM = *mut CANDIDATEFORM;
STRUCT!{struct CANDIDATELIST {
    dwSize: DWORD,
    dwStyle: DWORD,
    dwCount: DWORD,
    dwSelection: DWORD,
    dwPageStart: DWORD,
    dwPageSize: DWORD,
    dwOffset: [DWORD; 1],
}}
pub type LPCANDIDATELIST = *mut CANDIDATELIST;
STRUCT!{struct REGISTERWORDA {
    lpReading: LPSTR,
    lpWord: LPSTR,
}}
pub type LPREGISTERWORDA = *mut REGISTERWORDA;
STRUCT!{struct REGISTERWORDW {
    lpReading: LPWSTR,
    lpWord: LPWSTR,
}}
pub type LPREGISTERWORDW = *mut REGISTERWORDW;
STRUCT!{struct RECONVERTSTRING {
    dwSize: DWORD,
    dwVersion: DWORD,
    dwStrLen: DWORD,
    dwStrOffset: DWORD,
    dwCompStrLen: DWORD,
    dwCompStrOffset: DWORD,
    dwTargetStrLen: DWORD,
    dwTargetStrOffset: DWORD,
}}
pub type LPRECONVERTSTRING = *mut RECONVERTSTRING;
pub const STYLE_DESCRIPTION_SIZE: usize = 32;
STRUCT!{struct STYLEBUFA {
    dwStyle: DWORD,
    szDescription: [CHAR; STYLE_DESCRIPTION_SIZE],
}}
pub type LPSTYLEBUFA = *mut STYLEBUFA;
STRUCT!{struct STYLEBUFW {
    dwStyle: DWORD,
    szDescription: [WCHAR; STYLE_DESCRIPTION_SIZE],
}}
pub type LPSTYLEBUFW = *mut STYLEBUFW;
pub const IMEMENUITEM_STRING_SIZE: usize = 80;
STRUCT!{struct IMEMENUITEMINFOA {
    cbSize: UINT,
    fType: UINT,
    fState: UINT,
    wID: UINT,
    hbmpChecked: HBITMAP,
    hbmpUnchecked: HBITMAP,
    dwItemData: DWORD,
    szString: [CHAR; IMEMENUITEM_STRING_SIZE],
    hbmpItem: HBITMAP,
}}
pub type LPIMEMENUITEMINFOA = *mut IMEMENUITEMINFOA;
STRUCT!{struct IMEMENUITEMINFOW {
    cbSize: UINT,
    fType: UINT,
    fState: UINT,
    wID: UINT,
    hbmpChecked: HBITMAP,
    hbmpUnchecked: HBITMAP,
    dwItemData: DWORD,
    szString: [WCHAR; IMEMENUITEM_STRING_SIZE],
    hbmpItem: HBITMAP,
}}
pub type LPIMEMENUITEMINFOW = *mut IMEMENUITEMINFOW;
STRUCT!{struct IMECHARPOSITION {
    dwSize: DWORD,
    dwCharPos: DWORD,
    pt: POINT,
    cLineHeight: UINT,
    rcDocument: RECT,
}}
pub type LPIMECHARPOSITION = *mut IMECHARPOSITION;
FN!{stdcall IMCENUMPROC(
    HIMC,
    LPARAM,
) -> BOOL}
extern "system" {
    pub fn ImmInstallIMEA(
        lpszIMEFileName: LPCSTR,
        lpszLayoutText: LPCSTR,
    ) -> HKL;
    pub fn ImmInstallIMEW(
        lpszIMEFileName: LPCWSTR,
        lpszLayoutText: LPCWSTR,
    ) -> HKL;
    pub fn ImmGetDefaultIMEWnd(
        hWnd: HWND,
    ) -> HWND;
    pub fn ImmGetDescriptionA(
        hKL: HKL,
        lpszDescription: LPSTR,
        uBufLen: UINT,
    ) -> UINT;
    pub fn ImmGetDescriptionW(
        hKL: HKL,
        lpszDescription: LPWSTR,
        uBufLen: UINT,
    ) -> UINT;
    pub fn ImmGetIMEFileNameA(
        hKL: HKL,
        lpszFileName: LPSTR,
        uBufLen: UINT,
    ) -> UINT;
    pub fn ImmGetIMEFileNameW(
        hKL: HKL,
        lpszFileName: LPWSTR,
        uBufLen: UINT,
    ) -> UINT;
    pub fn ImmGetProperty(
        hKL: HKL,
        fdwIndex: DWORD,
    ) -> DWORD;
    pub fn ImmIsIME(
        hKL: HKL,
    ) -> BOOL;
    pub fn ImmSimulateHotKey(
        hWnd: HWND,
        dwHotKeyID: DWORD,
    ) -> BOOL;
    pub fn ImmCreateContext() -> HIMC;
    pub fn ImmDestroyContext(
        hIMC: HIMC,
    ) -> BOOL;
    pub fn ImmGetContext(
        hWnd: HWND,
    ) -> HIMC;
    pub fn ImmReleaseContext(
        hWnd: HWND,
        hIMC: HIMC,
    ) -> BOOL;
    pub fn ImmAssociateContext(
        hWnd: HWND,
        hIMC: HIMC,
    ) -> HIMC;
    pub fn ImmAssociateContextEx(
        hWnd: HWND,
        hIMC: HIMC,
        dwFlags: DWORD,
    ) -> BOOL;
    pub fn ImmGetCompositionStringA(
        hIMC: HIMC,
        dwIndex: DWORD,
        lpBuf: LPVOID,
        dwBufLen: DWORD,
    ) -> LONG;
    pub fn ImmGetCompositionStringW(
        hIMC: HIMC,
        dwIndex: DWORD,
        lpBuf: LPVOID,
        dwBufLen: DWORD,
    ) -> LONG;
    pub fn ImmSetCompositionStringA(
        hIMC: HIMC,
        dwIndex: DWORD,
        lpComp: LPVOID,
        dwCompLen: DWORD,
        lpRead: LPVOID,
        dwReadLen: DWORD,
    ) -> BOOL;
    pub fn ImmSetCompositionStringW(
        hIMC: HIMC,
        dwIndex: DWORD,
        lpComp: LPVOID,
        dwCompLen: DWORD,
        lpRead: LPVOID,
        dwReadLen: DWORD,
    ) -> BOOL;
    pub fn ImmGetCandidateListCountA(
        hIMC: HIMC,
        lpdwListCount: LPDWORD,
    ) -> DWORD;
    pub fn ImmGetCandidateListCountW(
        hIMC: HIMC,
        lpdwListCount: LPDWORD,
    ) -> DWORD;
    pub fn ImmGetCandidateListA(
        hIMC: HIMC,
        deIndex: DWORD,
        lpCandList: LPCANDIDATELIST,
        dwBufLen: DWORD,
    ) -> DWORD;
    pub fn ImmGetCandidateListW(
        hIMC: HIMC,
        deIndex: DWORD,
        lpCandList: LPCANDIDATELIST,
        dwBufLen: DWORD,
    ) -> DWORD;
    pub fn ImmGetGuideLineA(
        hIMC: HIMC,
        dwIndex: DWORD,
        lpBuf: LPSTR,
        dwBufLen: DWORD,
    ) -> DWORD;
    pub fn ImmGetGuideLineW(
        hIMC: HIMC,
        dwIndex: DWORD,
        lpBuf: LPWSTR,
        dwBufLen: DWORD,
    ) -> DWORD;
    pub fn ImmGetConversionStatus(
        hIMC: HIMC,
        lpfdwConversion: LPDWORD,
        lpfdwSentence: LPDWORD,
    ) -> BOOL;
    pub fn ImmSetConversionStatus(
        hIMC: HIMC,
        fdwConversion: DWORD,
        fdwSentence: DWORD,
    ) -> BOOL;
    pub fn ImmGetOpenStatus(
        hIMC: HIMC,
    ) -> BOOL;
    pub fn ImmSetOpenStatus(
        hIMC: HIMC,
        fopen: BOOL,
    ) -> BOOL;
    pub fn ImmConfigureIMEA(
        hKL: HKL,
        hWnd: HWND,
        dwMode: DWORD,
        lpData: LPVOID,
    ) -> BOOL;
    pub fn ImmConfigureIMEW(
        hKL: HKL,
        hWnd: HWND,
        dwMode: DWORD,
        lpData: LPVOID,
    ) -> BOOL;
    pub fn ImmEscapeA(
        hKL: HKL,
        hIMC: HIMC,
        uEscape: UINT,
        lpData: LPVOID,
    ) -> LRESULT;
    pub fn ImmEscapeW(
        hKL: HKL,
        hIMC: HIMC,
        uEscape: UINT,
        lpData: LPVOID,
    ) -> LRESULT;
    pub fn ImmGetConversionListA(
        hKL: HKL,
        hIMC: HIMC,
        lpSrc: LPCSTR,
        lpDst: LPCANDIDATELIST,
        dwBufLen: DWORD,
        uFlag: UINT,
    ) -> DWORD;
    pub fn ImmGetConversionListW(
        hKL: HKL,
        hIMC: HIMC,
        lpSrc: LPCWSTR,
        lpDst: LPCANDIDATELIST,
        dwBufLen: DWORD,
        uFlag: UINT,
    ) -> DWORD;
    pub fn ImmNotifyIME(
        hIMC: HIMC,
        dwAction: DWORD,
        dwIndex: DWORD,
        dwValue: DWORD,
    ) -> BOOL;
    pub fn ImmGetStatusWindowPos(
        hIMC: HIMC,
        lpptPos: LPPOINT,
    ) -> BOOL;
    pub fn ImmSetStatusWindowPos(
        hIMC: HIMC,
        lpptPos: LPPOINT,
    ) -> BOOL;
    pub fn ImmGetCompositionWindow(
        hIMC: HIMC,
        lpCompForm: LPCOMPOSITIONFORM,
    ) -> BOOL;
    pub fn ImmSetCompositionWindow(
        hIMC: HIMC,
        lpCompForm: LPCOMPOSITIONFORM,
    ) -> BOOL;
    pub fn ImmGetCandidateWindow(
        hIMC: HIMC,
        dwIndex: DWORD,
        lpCandidate: LPCANDIDATEFORM,
    ) -> BOOL;
    pub fn ImmSetCandidateWindow(
        hIMC: HIMC,
        lpCandidate: LPCANDIDATEFORM,
    ) -> BOOL;
    pub fn ImmIsUIMessageA(
        hWndIME: HWND,
        msg: UINT,
        wParam: WPARAM,
        lParam: LPARAM,
    ) -> BOOL;
    pub fn ImmIsUIMessageW(
        hWndIME: HWND,
        msg: UINT,
        wParam: WPARAM,
        lParam: LPARAM,
    ) -> BOOL;
    pub fn ImmGetVirtualKey(
        hWnd: HWND,
    ) -> UINT;
}
FN!{stdcall REGISTERWORDENUMPROCA(
    lpszReading: LPCSTR,
    dwStyle: DWORD,
    lpszString: LPCSTR,
    lpData: LPVOID,
) -> c_int}
FN!{stdcall REGISTERWORDENUMPROCW(
    lpszReading: LPCWSTR,
    dwStyle: DWORD,
    lpszString: LPCWSTR,
    lpData: LPVOID,
) -> c_int}
extern "system" {
    pub fn ImmRegisterWordA(
        hKL: HKL,
        lpszReading: LPCSTR,
        dwStyle: DWORD,
        lpszRegister: LPCSTR,
    ) -> BOOL;
    pub fn ImmRegisterWordW(
        hKL: HKL,
        lpszReading: LPCWSTR,
        dwStyle: DWORD,
        lpszRegister: LPCWSTR,
    ) -> BOOL;
    pub fn ImmUnregisterWordA(
        hKL: HKL,
        lpszReading: LPCSTR,
        dwStyle: DWORD,
        lpszRegister: LPCSTR,
    ) -> BOOL;
    pub fn ImmUnregisterWordW(
        hKL: HKL,
        lpszReading: LPCWSTR,
        dwStyle: DWORD,
        lpszRegister: LPCWSTR,
    ) -> BOOL;
    pub fn ImmGetRegisterWordStyleA(
        hKL: HKL,
        nItem: UINT,
        lpStyleBuf: LPSTYLEBUFA,
    ) -> UINT;
    pub fn ImmGetRegisterWordStyleW(
        hKL: HKL,
        nItem: UINT,
        lpStyleBuf: LPSTYLEBUFW,
    ) -> UINT;
    pub fn ImmEnumRegisterWordA(
        hKL: HKL,
        lpfnEnumProc: REGISTERWORDENUMPROCA,
        lpszReading: LPCSTR,
        dwStyle: DWORD,
        lpszRegister: LPCSTR,
        lpData: LPVOID,
    ) -> UINT;
    pub fn ImmEnumRegisterWordW(
        hKL: HKL,
        lpfnEnumProc: REGISTERWORDENUMPROCA,
        lpszReading: LPCWSTR,
        dwStyle: DWORD,
        lpszRegister: LPCWSTR,
        lpData: LPVOID,
    ) -> UINT;
    pub fn ImmDisableIME(
        idThread: DWORD,
    ) -> BOOL;
    pub fn ImmEnumInputContext(
        idThread: DWORD,
        lpfn: IMCENUMPROC,
        lParam: LPARAM
    ) -> BOOL;
    pub fn ImmGetImeMenuItemsA(
        hIMC: HIMC,
        dwFlags: DWORD,
        dwType: DWORD,
        lpImeParentMenu: LPIMEMENUITEMINFOA,
        lpImeMenu: LPIMEMENUITEMINFOA,
        dwSize: DWORD,
    ) -> DWORD;
    pub fn ImmGetImeMenuItemsW(
        hIMC: HIMC,
        dwFlags: DWORD,
        dwType: DWORD,
        lpImeParentMenu: LPIMEMENUITEMINFOW,
        lpImeMenu: LPIMEMENUITEMINFOW,
        dwSize: DWORD,
    ) -> DWORD;
    pub fn ImmDisableTextFrameService(
        idThread: DWORD
    ) -> BOOL;
    pub fn ImmDisableLegacyIME() -> BOOL;
}
pub const IMC_GETCANDIDATEPOS: WPARAM = 0x0007;
pub const IMC_SETCANDIDATEPOS: WPARAM = 0x0008;
pub const IMC_GETCOMPOSITIONFONT: WPARAM = 0x0009;
pub const IMC_SETCOMPOSITIONFONT: WPARAM = 0x000a;
pub const IMC_GETCOMPOSITIONWINDOW: WPARAM = 0x000b;
pub const IMC_SETCOMPOSITIONWINDOW: WPARAM = 0x000c;
pub const IMC_GETSTATUSWINDOWPOS: WPARAM = 0x000f;
pub const IMC_SETSTATUSWINDOWPOS: WPARAM = 0x0010;
pub const IMC_CLOSESTATUSWINDOW: WPARAM = 0x0021;
pub const IMC_OPENSTATUSWINDOW: WPARAM = 0x0022;
pub const NI_OPENCANDIDATE: DWORD = 0x0010;
pub const NI_CLOSECANDIDATE: DWORD = 0x0011;
pub const NI_SELECTCANDIDATESTR: DWORD = 0x0012;
pub const NI_CHANGECANDIDATELIST: DWORD = 0x0013;
pub const NI_FINALIZECONVERSIONRESULT: DWORD = 0x0014;
pub const NI_COMPOSITIONSTR: DWORD = 0x0015;
pub const NI_SETCANDIDATE_PAGESTART: DWORD = 0x0016;
pub const NI_SETCANDIDATE_PAGESIZE: DWORD = 0x0017;
pub const NI_IMEMENUSELECTED: DWORD = 0x0018;
pub const ISC_SHOWUICANDIDATEWINDOW: LPARAM = 0x00000001;
pub const ISC_SHOWUICOMPOSITIONWINDOW: LPARAM = 0x80000000;
pub const ISC_SHOWUIGUIDELINE: LPARAM = 0x40000000;
pub const ISC_SHOWUIALLCANDIDATEWINDOW: LPARAM = 0x0000000f;
pub const ISC_SHOWUIALL: LPARAM = 0xc000000f;
pub const CPS_COMPLETE: DWORD = 0x0001;
pub const CPS_CONVERT: DWORD = 0x0002;
pub const CPS_REVERT: DWORD = 0x0003;
pub const CPS_CANCEL: DWORD = 0x0004;
pub const MOD_ALT: DWORD = 0x0001;
pub const MOD_CONTROL: DWORD = 0x0002;
pub const MOD_SHIFT: DWORD = 0x0004;
pub const MOD_LEFT: DWORD = 0x8000;
pub const MOD_RIGHT: DWORD = 0x4000;
pub const MOD_ON_KEYUP: DWORD = 0x0800;
pub const MOD_IGNORE_ALL_MODIFIER: DWORD = 0x0400;
pub const IME_CHOTKEY_IME_NONIME_TOGGLE: DWORD = 0x10;
pub const IME_CHOTKEY_SHAPE_TOGGLE: DWORD = 0x11;
pub const IME_CHOTKEY_SYMBOL_TOGGLE: DWORD = 0x12;
pub const IME_JHOTKEY_CLOSE_OPEN: DWORD = 0x30;
pub const IME_KHOTKEY_SHAPE_TOGGLE: DWORD = 0x50;
pub const IME_KHOTKEY_HANJACONVERT: DWORD = 0x51;
pub const IME_KHOTKEY_ENGLISH: DWORD = 0x52;
pub const IME_THOTKEY_IME_NONIME_TOGGLE: DWORD = 0x70;
pub const IME_THOTKEY_SHAPE_TOGGLE: DWORD = 0x71;
pub const IME_THOTKEY_SYMBOL_TOGGLE: DWORD = 0x72;
pub const IME_HOTKEY_DSWITCH_FIRST: DWORD = 0x100;
pub const IME_HOTKEY_DSWITCH_LAST: DWORD = 0x11f;
pub const IME_HOTKEY_PRIVATE_FIRST: DWORD = 0x200;
pub const IME_ITHOTKEY_RESEND_RESULTSTR: DWORD = 0x200;
pub const IME_ITHOTKEY_PREVIOUS_COMPOSITION: DWORD = 0x201;
pub const IME_ITHOTKEY_UISTYLE_TOGGLE: DWORD = 0x202;
pub const IME_ITHOTKEY_RECONVERTSTRING: DWORD = 0x203;
pub const IME_HOTKEY_PRIVATE_LAST: DWORD = 0x21f;
pub const GCS_COMPREADSTR: DWORD = 0x0001;
pub const GCS_COMPREADATTR: DWORD = 0x0002;
pub const GCS_COMPREADCLAUSE: DWORD = 0x0004;
pub const GCS_COMPSTR: DWORD = 0x0008;
pub const GCS_COMPATTR: DWORD = 0x0010;
pub const GCS_COMPCLAUSE: DWORD = 0x0020;
pub const GCS_CURSORPOS: DWORD = 0x0080;
pub const GCS_DELTASTART: DWORD = 0x0100;
pub const GCS_RESULTREADSTR: DWORD = 0x0200;
pub const GCS_RESULTREADCLAUSE: DWORD = 0x0400;
pub const GCS_RESULTSTR: DWORD = 0x0800;
pub const GCS_RESULTCLAUSE: DWORD = 0x1000;
pub const CS_INSERTCHAR: LPARAM = 0x2000;
pub const CS_NOMOVECARET: LPARAM = 0x4000;
pub const IMEVER_0310: DWORD = 0x0003000a;
pub const IMEVER_0400: DWORD = 0x00040000;
pub const IME_PROP_AT_CARET: DWORD = 0x00010000;
pub const IME_PROP_SPECIAL_UI: DWORD = 0x00020000;
pub const IME_PROP_CANDLIST_START_FROM_1: DWORD = 0x00040000;
pub const IME_PROP_UNICODE: DWORD = 0x00080000;
pub const IME_PROP_COMPLETE_ON_UNSELECT: DWORD = 0x00100000;
pub const UI_CAP_2700: DWORD = 0x00000001;
pub const UI_CAP_ROT90: DWORD = 0x00000002;
pub const UI_CAP_ROTANY: DWORD = 0x00000004;
pub const SCS_CAP_COMPSTR: DWORD = 0x00000001;
pub const SCS_CAP_MAKEREAD: DWORD = 0x00000002;
pub const SCS_CAP_SETRECONVERTSTRING: DWORD = 0x00000004;
pub const SELECT_CAP_CONVERSION: DWORD = 0x00000001;
pub const SELECT_CAP_SENTENCE: DWORD = 0x00000002;
pub const GGL_LEVEL: DWORD = 0x00000001;
pub const GGL_INDEX: DWORD = 0x00000002;
pub const GGL_STRING: DWORD = 0x00000003;
pub const GGL_PRIVATE: DWORD = 0x00000004;
pub const GL_LEVEL_NOGUIDELINE: DWORD = 0x00000000;
pub const GL_LEVEL_FATAL: DWORD = 0x00000001;
pub const GL_LEVEL_ERROR: DWORD = 0x00000002;
pub const GL_LEVEL_WARNING: DWORD = 0x00000003;
pub const GL_LEVEL_INFORMATION: DWORD = 0x00000004;
pub const GL_ID_UNKNOWN: DWORD = 0x00000000;
pub const GL_ID_NOMODULE: DWORD = 0x00000001;
pub const GL_ID_NODICTIONARY: DWORD = 0x00000010;
pub const GL_ID_CANNOTSAVE: DWORD = 0x00000011;
pub const GL_ID_NOCONVERT: DWORD = 0x00000020;
pub const GL_ID_TYPINGERROR: DWORD = 0x00000021;
pub const GL_ID_TOOMANYSTROKE: DWORD = 0x00000022;
pub const GL_ID_READINGCONFLICT: DWORD = 0x00000023;
pub const GL_ID_INPUTREADING: DWORD = 0x00000024;
pub const GL_ID_INPUTRADICAL: DWORD = 0x00000025;
pub const GL_ID_INPUTCODE: DWORD = 0x00000026;
pub const GL_ID_INPUTSYMBOL: DWORD = 0x00000027;
pub const GL_ID_CHOOSECANDIDATE: DWORD = 0x00000028;
pub const GL_ID_REVERSECONVERSION: DWORD = 0x00000029;
pub const GL_ID_PRIVATE_FIRST: DWORD = 0x00008000;
pub const GL_ID_PRIVATE_LAST: DWORD = 0x0000ffff;
pub const IGP_GETIMEVERSION: DWORD = -4i32 as DWORD;
pub const IGP_PROPERTY: DWORD = 0x00000004;
pub const IGP_CONVERSION: DWORD = 0x00000008;
pub const IGP_SENTENCE: DWORD = 0x0000000c;
pub const IGP_UI: DWORD = 0x00000010;
pub const IGP_SETCOMPSTR: DWORD = 0x00000014;
pub const IGP_SELECT: DWORD = 0x00000018;
pub const SCS_SETSTR: DWORD = GCS_COMPREADSTR | GCS_COMPSTR;
pub const SCS_CHANGEATTR: DWORD = GCS_COMPREADATTR | GCS_COMPATTR;
pub const SCS_CHANGECLAUSE: DWORD = GCS_COMPREADCLAUSE | GCS_COMPCLAUSE;
pub const SCS_SETRECONVERTSTRING: DWORD = 0x00010000;
pub const SCS_QUERYRECONVERTSTRING: DWORD = 0x00020000;
pub const ATTR_INPUT: DWORD = 0x00;
pub const ATTR_TARGET_CONVERTED: DWORD = 0x01;
pub const ATTR_CONVERTED: DWORD = 0x02;
pub const ATTR_TARGET_NOTCONVERTED: DWORD = 0x03;
pub const ATTR_INPUT_ERROR: DWORD = 0x04;
pub const ATTR_FIXEDCONVERTED: DWORD = 0x05;
pub const CFS_DEFAULT: UINT = 0x0000;
pub const CFS_RECT: UINT = 0x0001;
pub const CFS_POINT: UINT = 0x0002;
pub const CFS_FORCE_POSITION: UINT = 0x0020;
pub const CFS_CANDIDATEPOS: UINT = 0x0040;
pub const CFS_EXCLUDE: UINT = 0x0080;
pub const GCL_CONVERSION: UINT = 0x0001;
pub const GCL_REVERSECONVERSION: UINT = 0x0002;
pub const GCL_REVERSE_LENGTH: UINT = 0x0003;
pub const IME_CMODE_HANGEUL: DWORD = IME_CMODE_NATIVE;
pub const IME_CMODE_SOFTKBD: DWORD = 0x0080;
pub const IME_CMODE_NOCONVERSION: DWORD = 0x0100;
pub const IME_CMODE_EUDC: DWORD = 0x0200;
pub const IME_CMODE_SYMBOL: DWORD = 0x0400;
pub const IME_CMODE_FIXED: DWORD = 0x0800;
pub const IME_CMODE_RESERVED: DWORD = 0xf0000000;
pub const IME_SMODE_NONE: DWORD = 0x0000;
pub const IME_SMODE_PLAURALCLAUSE: DWORD = 0x0001;
pub const IME_SMODE_SINGLECONVERT: DWORD = 0x0002;
pub const IME_SMODE_AUTOMATIC: DWORD = 0x0004;
pub const IME_SMODE_PHRASEPREDICT: DWORD = 0x0008;
pub const IME_SMODE_CONVERSATION: DWORD = 0x0010;
pub const IME_SMODE_RESERVED:DWORD = 0x0000f000;
pub const IME_CAND_UNKNOWN:DWORD = 0x0000;
pub const IME_CAND_READ: DWORD = 0x0001;
pub const IME_CAND_CODE: DWORD = 0x0002;
pub const IME_CAND_MEANING: DWORD = 0x0003;
pub const IME_CAND_RADICAL: DWORD = 0x0004;
pub const IME_CAND_STROKE: DWORD = 0x0005;
pub const IMN_CLOSESTATUSWINDOW: WPARAM = 0x0001;
pub const IMN_OPENSTATUSWINDOW: WPARAM = 0x0002;
pub const IMN_CHANGECANDIDATE: WPARAM = 0x0003;
pub const IMN_CLOSECANDIDATE: WPARAM = 0x0004;
pub const IMN_OPENCANDIDATE: WPARAM = 0x0005;
pub const IMN_SETCONVERSIONMODE: WPARAM = 0x0006;
pub const IMN_SETSENTENCEMODE: WPARAM = 0x0007;
pub const IMN_SETOPENSTATUS: WPARAM = 0x0008;
pub const IMN_SETCANDIDATEPOS: WPARAM = 0x0009;
pub const IMN_SETCOMPOSITIONFONT: WPARAM = 0x000a;
pub const IMN_SETCOMPOSITIONWINDOW: WPARAM = 0x000b;
pub const IMN_SETSTATUSWINDOWPOS: WPARAM = 0x000c;
pub const IMN_GUIDELINE: WPARAM = 0x000d;
pub const IMN_PRIVATE: WPARAM = 0x000e;
pub const IMR_COMPOSITIONWINDOW: WPARAM = 0x0001;
pub const IMR_CANDIDATEWINDOW: WPARAM = 0x0002;
pub const IMR_COMPOSITIONFONT: WPARAM = 0x0003;
pub const IMR_RECONVERTSTRING: WPARAM = 0x0004;
pub const IMR_CONFIRMRECONVERTSTRING: WPARAM = 0x0005;
pub const IMR_QUERYCHARPOSITION: WPARAM = 0x0006;
pub const IMR_DOCUMENTFEED: WPARAM = 0x0007;
pub const IMM_ERROR_NODATA: LONG = -1;
pub const IMM_ERROR_GENERAL: LONG = -2;
pub const IME_CONFIG_GENERAL: DWORD = 1;
pub const IME_CONFIG_REGISTERWORD: DWORD = 2;
pub const IME_CONFIG_SELECTDICTIONARY: DWORD = 3;
pub const IME_ESC_QUERY_SUPPORT: UINT = 0x0003;
pub const IME_ESC_RESERVED_FIRST: UINT = 0x0004;
pub const IME_ESC_RESERVED_LAST: UINT = 0x07ff;
pub const IME_ESC_PRIVATE_FIRST: UINT = 0x0800;
pub const IME_ESC_PRIVATE_LAST: UINT = 0x0fff;
pub const IME_ESC_SEQUENCE_TO_INTERNAL: UINT = 0x1001;
pub const IME_ESC_GET_EUDC_DICTIONARY: UINT = 0x1003;
pub const IME_ESC_SET_EUDC_DICTIONARY: UINT = 0x1004;
pub const IME_ESC_MAX_KEY: UINT = 0x1005;
pub const IME_ESC_IME_NAME: UINT = 0x1006;
pub const IME_ESC_SYNC_HOTKEY: UINT = 0x1007;
pub const IME_ESC_HANJA_MODE: UINT = 0x1008;
pub const IME_ESC_AUTOMATA: UINT = 0x1009;
pub const IME_ESC_PRIVATE_HOTKEY: UINT = 0x100a;
pub const IME_ESC_GETHELPFILENAME: UINT = 0x100b;
pub const IME_REGWORD_STYLE_EUDC: DWORD = 0x00000001;
pub const IME_REGWORD_STYLE_USER_FIRST: DWORD = 0x80000000;
pub const IME_REGWORD_STYLE_USER_LAST: DWORD = 0xffffffff;
pub const IACE_CHILDREN: DWORD = 0x0001;
pub const IACE_DEFAULT: DWORD = 0x0010;
pub const IACE_IGNORENOCONTEXT: DWORD = 0x0020;
pub const IGIMIF_RIGHTMENU: DWORD = 0x0001;
pub const IGIMII_CMODE: DWORD = 0x0001;
pub const IGIMII_SMODE: DWORD = 0x0002;
pub const IGIMII_CONFIGURE: DWORD = 0x0004;
pub const IGIMII_TOOLS: DWORD = 0x0008;
pub const IGIMII_HELP: DWORD = 0x0010;
pub const IGIMII_OTHER: DWORD = 0x0020;
pub const IGIMII_INPUTTOOLS: DWORD = 0x0040;
pub const IMFT_RADIOCHECK: UINT = 0x00001;
pub const IMFT_SEPARATOR: UINT = 0x00002;
pub const IMFT_SUBMENU: UINT = 0x00004;
pub const IMFS_GRAYED: UINT = MFS_GRAYED;
pub const IMFS_DISABLED: UINT = MFS_DISABLED;
pub const IMFS_CHECKED: UINT = MFS_CHECKED;
pub const IMFS_HILITE: UINT = MFS_HILITE;
pub const IMFS_ENABLED: UINT = MFS_ENABLED;
pub const IMFS_UNCHECKED: UINT = MFS_UNCHECKED;
pub const IMFS_UNHILITE: UINT = MFS_UNHILITE;
pub const IMFS_DEFAULT: UINT = MFS_DEFAULT;
pub const SOFTKEYBOARD_TYPE_T1: UINT = 0x0001;
pub const SOFTKEYBOARD_TYPE_C1: UINT = 0x0002;

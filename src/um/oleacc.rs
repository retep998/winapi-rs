// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Mappings for the contents of oleacc.h
use ctypes::{c_int, c_long, c_uchar, c_ulong, c_void};
use shared::guiddef::{GUID, REFIID};
use shared::minwindef::{BOOL, BYTE, DWORD, LRESULT, UINT, WPARAM};
use shared::windef::{HMENU, HWND, POINT};
use shared::winerror::HRESULT;
use shared::wtypes::BSTR;
use um::oaidl::{IDispatch, IDispatchVtbl, LPSAFEARRAY, SAFEARRAY, VARIANT};
use um::unknwnbase::{IUnknown, IUnknownVtbl, LPUNKNOWN};
use um::winnt::{LONG, LPCSTR, LPCWSTR, LPSTR, LPWSTR};
FN!{stdcall LPFNLRESULTFROMOBJECT(
    riid: REFIID,
    wParam: WPARAM,
    punk: LPUNKNOWN,
) -> LRESULT}
FN!{stdcall LPFNOBJECTFROMLRESULT(
    lResult: LRESULT,
    riid: REFIID,
    wParam: WPARAM,
    ppvObject: *mut *mut c_void,
) -> HRESULT}
FN!{stdcall LPFNACCESSIBLEOBJECTFROMWINDOW(
    hwnd: HWND,
    dwId: DWORD,
    riid: REFIID,
    ppvObject: *mut *mut c_void,
) -> HRESULT}
FN!{stdcall LPFNACCESSIBLEOBJECTFROMPOINT(
    ptScreen: POINT,
    ppacc: *mut *mut IAccessible,
    pvarChild: *mut VARIANT,
) -> HRESULT}
FN!{stdcall LPFNCREATESTDACCESSIBLEOBJECT(
    hwnd: HWND,
    idObject: LONG,
    riid: REFIID,
    ppvObject: *mut *mut c_void,
) -> HRESULT}
FN!{stdcall LPFNACCESSIBLECHILDREN(
    paccContainer: *mut IAccessible,
    iChildStart: LONG,
    cChildren: LONG,
    rgvarChildren: *mut VARIANT,
    pcObtained: *mut LONG,
) -> HRESULT}
DEFINE_GUID!{LIBID_Accessibility,
    0x1ea4dbf0, 0x3c3b, 0x11cf, 0x81, 0x0c, 0x00, 0xaa, 0x00, 0x38, 0x9b, 0x71}
DEFINE_GUID!{IID_IAccessible,
    0x618736e0, 0x3c3d, 0x11cf, 0x81, 0x0c, 0x00, 0xaa, 0x00, 0x38, 0x9b, 0x71}
DEFINE_GUID!{IID_IAccessibleHandler,
    0x03022430, 0xABC4, 0x11d0, 0xBD, 0xE2, 0x00, 0xAA, 0x00, 0x1A, 0x19, 0x53}
DEFINE_GUID!{IID_IAccIdentity,
    0x7852b78d, 0x1cfd, 0x41c1, 0xa6, 0x15, 0x9c, 0x0c, 0x85, 0x96, 0x0b, 0x5f}
DEFINE_GUID!{IID_IAccPropServer,
    0x76c0dbbb, 0x15e0, 0x4e7b, 0xb6, 0x1b, 0x20, 0xee, 0xea, 0x20, 0x01, 0xe0}
DEFINE_GUID!{IID_IAccPropServices,
    0x6e26e776, 0x04f0, 0x495d, 0x80, 0xe4, 0x33, 0x30, 0x35, 0x2e, 0x31, 0x69}
DEFINE_GUID!{IID_IAccPropMgrInternal,
    0x2bd370a9, 0x3e7f, 0x4edd, 0x8a, 0x85, 0xf8, 0xfe, 0xd1, 0xf8, 0xe5, 0x1f}
DEFINE_GUID!{CLSID_AccPropServices,
    0xb5f8350b, 0x0548, 0x48b1, 0xa6, 0xee, 0x88, 0xbd, 0x00, 0xb4, 0xa5, 0xe7}
DEFINE_GUID!{IIS_IsOleaccProxy,
    0x902697fa, 0x80e4, 0x4560, 0x80, 0x2a, 0xa1, 0x3f, 0x22, 0xa6, 0x47, 0x09}
DEFINE_GUID!{IIS_ControlAccessible,
    0x38c682a6, 0x9731, 0x43f2, 0x9f, 0xae, 0xe9, 0x01, 0xe6, 0x41, 0xb1, 0x01}
extern "C" {
    pub fn LresultFromObject(
        riid: REFIID,
        wParam: WPARAM,
        punk: LPUNKNOWN,
    ) -> LRESULT;
    pub fn ObjectFromLresult(
        lResult: LRESULT,
        riid: REFIID,
        wParam: WPARAM,
        ppvObject: *mut *mut c_void);
    pub fn WindowFromAccessibleObject(
        arg1: *mut IAccessible,
        phwnd: *mut HWND);
    pub fn AccessibleObjectFromWindow(
        hwnd: HWND,
        dwId: DWORD,
        riid: REFIID,
        ppvObject: *mut *mut c_void);
    pub fn AccessibleObjectFromEvent(
        hwnd: HWND,
        dwId: DWORD,
        dwChildId: DWORD,
        ppacc: *mut *mut IAccessible,
        pvarChild: *mut VARIANT);
    pub fn AccessibleObjectFromPoint(
        ptScreen: POINT,
        ppacc: *mut *mut IAccessible,
        pvarChild: *mut VARIANT);
    pub fn AccessibleChildren(
        paccContainer: *mut IAccessible,
        iChildStart: LONG,
        cChildren: LONG,
        rgvarChildren: *mut VARIANT,
        pcObtained: *mut LONG);
    pub fn GetRoleTextA(
        lRole: DWORD,
        lpszRole: LPSTR,
        cchRoleMax: UINT,
    ) -> UINT;
    pub fn GetRoleTextW(
        lRole: DWORD,
        lpszRole: LPWSTR,
        cchRoleMax: UINT,
    ) -> UINT;
    pub fn GetStateTextA(
        lStateBit: DWORD,
        lpszState: LPSTR,
        cchState: UINT,
    ) -> UINT;
    pub fn GetStateTextW(
        lStateBit: DWORD,
        lpszState: LPWSTR,
        cchState: UINT,
    ) -> UINT;
    pub fn GetOleaccVersionInfo(
        pVer: *mut DWORD,
        pBuild: *mut DWORD);
    pub fn CreateStdAccessibleObject(
        hwnd: HWND,
        idObject: LONG,
        riid: REFIID,
        ppvObject: *mut *mut c_void);
    pub fn CreateStdAccessibleProxyA(
        hwnd: HWND,
        pClassName: LPCSTR,
        idObject: LONG,
        riid: REFIID,
        ppvObject: *mut *mut c_void);
    pub fn CreateStdAccessibleProxyW(
        hwnd: HWND,
        pClassName: LPCWSTR,
        idObject: LONG,
        riid: REFIID,
        ppvObject: *mut *mut c_void);
}
pub const ANRUS_ON_SCREEN_KEYBOARD_ACTIVE: DWORD = 0x0000001;
pub const ANRUS_TOUCH_MODIFICATION_ACTIVE: DWORD = 0x0000002;
pub const ANRUS_PRIORITY_AUDIO_ACTIVE: DWORD = 0x0000004;
pub const ANRUS_PRIORITY_AUDIO_ACTIVE_NODUCK: DWORD = 0x0000008;
pub const ANRUS_PRIORITY_AUDIO_DYNAMIC_DUCK: DWORD = 0x0000010;
extern "C" {
    pub fn AccSetRunningUtilityState(
        hwndApp: HWND,
        dwUtilityStateMask: DWORD,
        dwUtilityState: DWORD);
    pub fn AccNotifyTouchInteraction(
        hwndApp: HWND,
        hwndTarget: HWND,
        ptTarget: POINT);
}
pub const MSAA_MENU_SIG: DWORD = 0xAA0DF00D;
STRUCT!{struct MSAAMENUINFO {
    dwMSAASignature: DWORD,
    cchWText: DWORD,
    pszWText: LPWSTR,
}}
pub type LPMSAAMENUINFO = *mut MSAAMENUINFO;
DEFINE_GUID!{PROPID_ACC_NAME,
    0x608d3df8, 0x8128, 0x4aa7, 0xa4, 0x28, 0xf5, 0x5e, 0x49, 0x26, 0x72, 0x91}
DEFINE_GUID!{PROPID_ACC_VALUE,
    0x123fe443, 0x211a, 0x4615, 0x95, 0x27, 0xc4, 0x5a, 0x7e, 0x93, 0x71, 0x7a}
DEFINE_GUID!{PROPID_ACC_DESCRIPTION,
    0x4d48dfe4, 0xbd3f, 0x491f, 0xa6, 0x48, 0x49, 0x2d, 0x6f, 0x20, 0xc5, 0x88}
DEFINE_GUID!{PROPID_ACC_ROLE,
    0xcb905ff2, 0x7bd1, 0x4c05, 0xb3, 0xc8, 0xe6, 0xc2, 0x41, 0x36, 0x4d, 0x70}
DEFINE_GUID!{PROPID_ACC_STATE,
    0xa8d4d5b0, 0x0a21, 0x42d0, 0xa5, 0xc0, 0x51, 0x4e, 0x98, 0x4f, 0x45, 0x7b}
DEFINE_GUID!{PROPID_ACC_HELP,
    0xc831e11f, 0x44db, 0x4a99, 0x97, 0x68, 0xcb, 0x8f, 0x97, 0x8b, 0x72, 0x31}
DEFINE_GUID!{PROPID_ACC_KEYBOARDSHORTCUT,
    0x7d9bceee, 0x7d1e, 0x4979, 0x93, 0x82, 0x51, 0x80, 0xf4, 0x17, 0x2c, 0x34}
DEFINE_GUID!{PROPID_ACC_DEFAULTACTION,
    0x180c072b, 0xc27f, 0x43c7, 0x99, 0x22, 0xf6, 0x35, 0x62, 0xa4, 0x63, 0x2b}
DEFINE_GUID!{PROPID_ACC_HELPTOPIC,
    0x787d1379, 0x8ede, 0x440b, 0x8a, 0xec, 0x11, 0xf7, 0xbf, 0x90, 0x30, 0xb3}
DEFINE_GUID!{PROPID_ACC_FOCUS,
    0x6eb335df, 0x1c29, 0x4127, 0xb1, 0x2c, 0xde, 0xe9, 0xfd, 0x15, 0x7f, 0x2b}
DEFINE_GUID!{PROPID_ACC_SELECTION,
    0xb99d073c, 0xd731, 0x405b, 0x90, 0x61, 0xd9, 0x5e, 0x8f, 0x84, 0x29, 0x84}
DEFINE_GUID!{PROPID_ACC_PARENT,
    0x474c22b6, 0xffc2, 0x467a, 0xb1, 0xb5, 0xe9, 0x58, 0xb4, 0x65, 0x73, 0x30}
DEFINE_GUID!{PROPID_ACC_NAV_UP,
    0x016e1a2b, 0x1a4e, 0x4767, 0x86, 0x12, 0x33, 0x86, 0xf6, 0x69, 0x35, 0xec}
DEFINE_GUID!{PROPID_ACC_NAV_DOWN,
    0x031670ed, 0x3cdf, 0x48d2, 0x96, 0x13, 0x13, 0x8f, 0x2d, 0xd8, 0xa6, 0x68}
DEFINE_GUID!{PROPID_ACC_NAV_LEFT,
    0x228086cb, 0x82f1, 0x4a39, 0x87, 0x05, 0xdc, 0xdc, 0x0f, 0xff, 0x92, 0xf5}
DEFINE_GUID!{PROPID_ACC_NAV_RIGHT,
    0xcd211d9f, 0xe1cb, 0x4fe5, 0xa7, 0x7c, 0x92, 0x0b, 0x88, 0x4d, 0x09, 0x5b}
DEFINE_GUID!{PROPID_ACC_NAV_PREV,
    0x776d3891, 0xc73b, 0x4480, 0xb3, 0xf6, 0x07, 0x6a, 0x16, 0xa1, 0x5a, 0xf6}
DEFINE_GUID!{PROPID_ACC_NAV_NEXT,
    0x1cdc5455, 0x8cd9, 0x4c92, 0xa3, 0x71, 0x39, 0x39, 0xa2, 0xfe, 0x3e, 0xee}
DEFINE_GUID!{PROPID_ACC_NAV_FIRSTCHILD,
    0xcfd02558, 0x557b, 0x4c67, 0x84, 0xf9, 0x2a, 0x09, 0xfc, 0xe4, 0x07, 0x49}
DEFINE_GUID!{PROPID_ACC_NAV_LASTCHILD,
    0x302ecaa5, 0x48d5, 0x4f8d, 0xb6, 0x71, 0x1a, 0x8d, 0x20, 0xa7, 0x78, 0x32}
DEFINE_GUID!{PROPID_ACC_ROLEMAP,
    0xf79acda2, 0x140d, 0x4fe6, 0x89, 0x14, 0x20, 0x84, 0x76, 0x32, 0x82, 0x69}
DEFINE_GUID!{PROPID_ACC_VALUEMAP,
    0xda1c3d79, 0xfc5c, 0x420e, 0xb3, 0x99, 0x9d, 0x15, 0x33, 0x54, 0x9e, 0x75}
DEFINE_GUID!{PROPID_ACC_STATEMAP,
    0x43946c5e, 0x0ac0, 0x4042, 0xb5, 0x25, 0x07, 0xbb, 0xdb, 0xe1, 0x7f, 0xa7}
DEFINE_GUID!{PROPID_ACC_DESCRIPTIONMAP,
    0x1ff1435f, 0x8a14, 0x477b, 0xb2, 0x26, 0xa0, 0xab, 0xe2, 0x79, 0x97, 0x5d}
DEFINE_GUID!{PROPID_ACC_DODEFAULTACTION,
    0x1ba09523, 0x2e3b, 0x49a6, 0xa0, 0x59, 0x59, 0x68, 0x2a, 0x3c, 0x48, 0xfd}
pub type LPACCESSIBLE = *mut IAccessible;
pub const NAVDIR_MIN: LONG = 0;
pub const NAVDIR_UP: LONG = 0x1;
pub const NAVDIR_DOWN: LONG = 0x2;
pub const NAVDIR_LEFT: LONG = 0x3;
pub const NAVDIR_RIGHT: LONG = 0x4;
pub const NAVDIR_NEXT: LONG = 0x5;
pub const NAVDIR_PREVIOUS: LONG = 0x6;
pub const NAVDIR_FIRSTCHILD: LONG = 0x7;
pub const NAVDIR_LASTCHILD: LONG = 0x8;
pub const NAVDIR_MAX: LONG = 0x9;
pub const SELFLAG_NONE: LONG = 0;
pub const SELFLAG_TAKEFOCUS: LONG = 0x1;
pub const SELFLAG_TAKESELECTION: LONG = 0x2;
pub const SELFLAG_EXTENDSELECTION: LONG = 0x4;
pub const SELFLAG_ADDSELECTION: LONG = 0x8;
pub const SELFLAG_REMOVESELECTION: LONG = 0x10;
pub const SELFLAG_VALID: LONG = 0x1f;
pub const STATE_SYSTEM_NORMAL: LONG = 0;
pub const STATE_SYSTEM_UNAVAILABLE: LONG = 0x1;
pub const STATE_SYSTEM_SELECTED: LONG = 0x2;
pub const STATE_SYSTEM_FOCUSED: LONG = 0x4;
pub const STATE_SYSTEM_PRESSED: LONG = 0x8;
pub const STATE_SYSTEM_CHECKED: LONG = 0x10;
pub const STATE_SYSTEM_MIXED: LONG = 0x20;
pub const STATE_SYSTEM_INDETERMINATE: LONG = STATE_SYSTEM_MIXED;
pub const STATE_SYSTEM_READONLY: LONG = 0x40;
pub const STATE_SYSTEM_HOTTRACKED: LONG = 0x80;
pub const STATE_SYSTEM_DEFAULT: LONG = 0x100;
pub const STATE_SYSTEM_EXPANDED: LONG = 0x200;
pub const STATE_SYSTEM_COLLAPSED: LONG = 0x400;
pub const STATE_SYSTEM_BUSY: LONG = 0x800;
pub const STATE_SYSTEM_FLOATING: LONG = 0x1000;
pub const STATE_SYSTEM_MARQUEED: LONG = 0x2000;
pub const STATE_SYSTEM_ANIMATED: LONG = 0x4000;
pub const STATE_SYSTEM_INVISIBLE: LONG = 0x8000;
pub const STATE_SYSTEM_OFFSCREEN: LONG = 0x10000;
pub const STATE_SYSTEM_SIZEABLE: LONG = 0x20000;
pub const STATE_SYSTEM_MOVEABLE: LONG = 0x40000;
pub const STATE_SYSTEM_SELFVOICING: LONG = 0x80000;
pub const STATE_SYSTEM_FOCUSABLE: LONG = 0x100000;
pub const STATE_SYSTEM_SELECTABLE: LONG = 0x200000;
pub const STATE_SYSTEM_LINKED: LONG = 0x400000;
pub const STATE_SYSTEM_TRAVERSED: LONG = 0x800000;
pub const STATE_SYSTEM_MULTISELECTABLE: LONG = 0x1000000;
pub const STATE_SYSTEM_EXTSELECTABLE: LONG = 0x2000000;
pub const STATE_SYSTEM_ALERT_LOW: LONG = 0x4000000;
pub const STATE_SYSTEM_ALERT_MEDIUM: LONG = 0x8000000;
pub const STATE_SYSTEM_ALERT_HIGH: LONG = 0x10000000;
pub const STATE_SYSTEM_PROTECTED: LONG = 0x20000000;
pub const STATE_SYSTEM_VALID: LONG = 0x7fffffff;
pub const STATE_SYSTEM_HASPOPUP: LONG = 0x40000000;
pub const ROLE_SYSTEM_TITLEBAR: LONG = 0x1;
pub const ROLE_SYSTEM_MENUBAR: LONG = 0x2;
pub const ROLE_SYSTEM_SCROLLBAR: LONG = 0x3;
pub const ROLE_SYSTEM_GRIP: LONG = 0x4;
pub const ROLE_SYSTEM_SOUND: LONG = 0x5;
pub const ROLE_SYSTEM_CURSOR: LONG = 0x6;
pub const ROLE_SYSTEM_CARET: LONG = 0x7;
pub const ROLE_SYSTEM_ALERT: LONG = 0x8;
pub const ROLE_SYSTEM_WINDOW: LONG = 0x9;
pub const ROLE_SYSTEM_CLIENT: LONG = 0xa;
pub const ROLE_SYSTEM_MENUPOPUP: LONG = 0xb;
pub const ROLE_SYSTEM_MENUITEM: LONG = 0xc;
pub const ROLE_SYSTEM_TOOLTIP: LONG = 0xd;
pub const ROLE_SYSTEM_APPLICATION: LONG = 0xe;
pub const ROLE_SYSTEM_DOCUMENT: LONG = 0xf;
pub const ROLE_SYSTEM_PANE: LONG = 0x10;
pub const ROLE_SYSTEM_CHART: LONG = 0x11;
pub const ROLE_SYSTEM_DIALOG: LONG = 0x12;
pub const ROLE_SYSTEM_BORDER: LONG = 0x13;
pub const ROLE_SYSTEM_GROUPING: LONG = 0x14;
pub const ROLE_SYSTEM_SEPARATOR: LONG = 0x15;
pub const ROLE_SYSTEM_TOOLBAR: LONG = 0x16;
pub const ROLE_SYSTEM_STATUSBAR: LONG = 0x17;
pub const ROLE_SYSTEM_TABLE: LONG = 0x18;
pub const ROLE_SYSTEM_COLUMNHEADER: LONG = 0x19;
pub const ROLE_SYSTEM_ROWHEADER: LONG = 0x1a;
pub const ROLE_SYSTEM_COLUMN: LONG = 0x1b;
pub const ROLE_SYSTEM_ROW: LONG = 0x1c;
pub const ROLE_SYSTEM_CELL: LONG = 0x1d;
pub const ROLE_SYSTEM_LINK: LONG = 0x1e;
pub const ROLE_SYSTEM_HELPBALLOON: LONG = 0x1f;
pub const ROLE_SYSTEM_CHARACTER: LONG = 0x20;
pub const ROLE_SYSTEM_LIST: LONG = 0x21;
pub const ROLE_SYSTEM_LISTITEM: LONG = 0x22;
pub const ROLE_SYSTEM_OUTLINE: LONG = 0x23;
pub const ROLE_SYSTEM_OUTLINEITEM: LONG = 0x24;
pub const ROLE_SYSTEM_PAGETAB: LONG = 0x25;
pub const ROLE_SYSTEM_PROPERTYPAGE: LONG = 0x26;
pub const ROLE_SYSTEM_INDICATOR: LONG = 0x27;
pub const ROLE_SYSTEM_GRAPHIC: LONG = 0x28;
pub const ROLE_SYSTEM_STATICTEXT: LONG = 0x29;
pub const ROLE_SYSTEM_TEXT: LONG = 0x2a;
pub const ROLE_SYSTEM_PUSHBUTTON: LONG = 0x2b;
pub const ROLE_SYSTEM_CHECKBUTTON: LONG = 0x2c;
pub const ROLE_SYSTEM_RADIOBUTTON: LONG = 0x2d;
pub const ROLE_SYSTEM_COMBOBOX: LONG = 0x2e;
pub const ROLE_SYSTEM_DROPLIST: LONG = 0x2f;
pub const ROLE_SYSTEM_PROGRESSBAR: LONG = 0x30;
pub const ROLE_SYSTEM_DIAL: LONG = 0x31;
pub const ROLE_SYSTEM_HOTKEYFIELD: LONG = 0x32;
pub const ROLE_SYSTEM_SLIDER: LONG = 0x33;
pub const ROLE_SYSTEM_SPINBUTTON: LONG = 0x34;
pub const ROLE_SYSTEM_DIAGRAM: LONG = 0x35;
pub const ROLE_SYSTEM_ANIMATION: LONG = 0x36;
pub const ROLE_SYSTEM_EQUATION: LONG = 0x37;
pub const ROLE_SYSTEM_BUTTONDROPDOWN: LONG = 0x38;
pub const ROLE_SYSTEM_BUTTONMENU: LONG = 0x39;
pub const ROLE_SYSTEM_BUTTONDROPDOWNGRID: LONG = 0x3a;
pub const ROLE_SYSTEM_WHITESPACE: LONG = 0x3b;
pub const ROLE_SYSTEM_PAGETABLIST: LONG = 0x3c;
pub const ROLE_SYSTEM_CLOCK: LONG = 0x3d;
pub const ROLE_SYSTEM_SPLITBUTTON: LONG = 0x3e;
pub const ROLE_SYSTEM_IPADDRESS: LONG = 0x3f;
pub const ROLE_SYSTEM_OUTLINEBUTTON: LONG = 0x40;
RIDL!{#[uuid(0x618736e0, 0x3c3d, 0x11cf, 0x81, 0x0c, 0x00, 0xaa, 0x00, 0x38, 0x9b, 0x71)]
interface IAccessible(IAccessibleVtbl): IDispatch(IDispatchVtbl) {
    fn get_accParent(
        ppdispParent: *mut *mut IDispatch,
    ) -> HRESULT,
    fn get_accChildCount(
        pcountChildren: *mut c_long,
    ) -> HRESULT,
    fn get_accChild(
        varChild: VARIANT,
        ppdispChild: *mut *mut IDispatch,
    ) -> HRESULT,
    fn get_accName(
        varChild: VARIANT,
        pszName: *mut BSTR,
    ) -> HRESULT,
    fn get_accValue(
        varChild: VARIANT,
        pszValue: *mut BSTR,
    ) -> HRESULT,
    fn get_accDescription(
        varChild: VARIANT,
        pszDescription: *mut BSTR,
    ) -> HRESULT,
    fn get_accRole(
        varChild: VARIANT,
        pvarRole: *mut VARIANT,
    ) -> HRESULT,
    fn get_accState(
        varChild: VARIANT,
        pvarState: *mut VARIANT,
    ) -> HRESULT,
    fn get_accHelp(
        varChild: VARIANT,
        pszHelp: *mut BSTR,
    ) -> HRESULT,
    fn get_accHelpTopic(
        pszHelpFile: *mut BSTR,
        varChild: VARIANT,
        pidTopic: *mut c_long,
    ) -> HRESULT,
    fn get_accKeyboardShortcut(
        varChild: VARIANT,
        pszKeyboardShortcut: *mut BSTR,
    ) -> HRESULT,
    fn get_accFocus(
        pvarChild: *mut VARIANT,
    ) -> HRESULT,
    fn get_accSelection(
        pvarChildren: *mut VARIANT,
    ) -> HRESULT,
    fn get_accDefaultAction(
        varChild: VARIANT,
        pszDefaultAction: *mut BSTR,
    ) -> HRESULT,
    fn accSelect(
        flagsSelect: c_long,
        varChild: VARIANT,
    ) -> HRESULT,
    fn accLocation(
        pxLeft: *mut c_long,
        pyTop: *mut c_long,
        pcxWidth: *mut c_long,
        pcyHeight: *mut c_long,
        varChild: VARIANT,
    ) -> HRESULT,
    fn accNavigate(
        navDir: c_long,
        varState: VARIANT,
        pvarEndUpAt: *mut VARIANT,
    ) -> HRESULT,
    fn accHitTest(
        xLeft: c_long,
        yTop: c_long,
        pvarChild: *mut VARIANT,
    ) -> HRESULT,
    fn accDoDefaultAction(
        varChild: VARIANT,
    ) -> HRESULT,
    fn put_accName(
        varChild: VARIANT,
        szName: BSTR,
    ) -> HRESULT,
    fn put_accValue(
        varChild: VARIANT,
        szValue: BSTR,
    ) -> HRESULT,
}}
pub type LPACCESSIBLEHANDLER = *mut IAccessibleHandler;
RIDL!{#[uuid(0x03022430, 0xabc4, 0x11d0, 0xbd, 0xe2, 0x00, 0xaa, 0x00, 0x1a, 0x19, 0x53)]
interface IAccessibleHandler(IAccessibleHandlerVtbl): IUnknown(IUnknownVtbl) {
    fn AccessibleObjectFromID(
        hwnd: c_long,
        lObjectID: c_long,
        pIAccessible: *mut LPACCESSIBLE,
    ) -> HRESULT,
}}
pub type LPACCESSIBLEWINDOWLESSSITE = *mut IAccessibleWindowlessSite;
RIDL!{#[uuid(0xbf3abd9c, 0x76da, 0x4389, 0x9e, 0xb6, 0x14, 0x27, 0xd2, 0x5a, 0xba, 0xb7)]
interface IAccessibleWindowlessSite(IAccessibleWindowlessSiteVtbl): IUnknown(IUnknownVtbl) {
    fn AcquireObjectIdRange(
        rangeSize: c_long,
        pRangeOwner: *mut IAccessibleHandler,
        pRangeBase: *mut c_long,
    ) -> HRESULT,
    fn ReleaseObjectIdRange(
        rangeBase: c_long,
        pRangeOwner: *mut IAccessibleHandler,
    ) -> HRESULT,
    fn QueryObjectIdRanges(
        pRangesOwner: *mut IAccessibleHandler,
        psaRanges: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    fn GetParentAccessible(
        ppParent: *mut *mut IAccessible,
    ) -> HRESULT,
}}
ENUM!{enum AnnoScope {
    ANNO_THIS = 0,
    ANNO_CONTAINER = 1,
}}
pub type MSAAPROPID = GUID;
RIDL!{#[uuid(0x7852b78d, 0x1cfd, 0x41c1, 0xa6, 0x15, 0x9c, 0x0c, 0x85, 0x96, 0x0b, 0x5f)]
interface IAccIdentity(IAccIdentityVtbl): IUnknown(IUnknownVtbl) {
    fn GetIdentityString(
        dwIDChild: DWORD,
        ppIDString: *mut *mut BYTE,
        pdwIDStringLen: *mut BYTE,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x76c0dbbb, 0x15e0, 0x4e7b, 0xb6, 0x1b, 0x20, 0xee, 0xea, 0x20, 0x01, 0xe0)]
interface IAccPropServer(IAccPropServerVtbl): IUnknown(IUnknownVtbl) {
    fn GetPropValue(
        pIDString: *const BYTE,
        dwIDStringLen: DWORD,
        idProp: MSAAPROPID,
        pvarValue: VARIANT,
        pfHasProp: *mut BOOL,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x6e26e776, 0x04f0, 0x495d, 0x80, 0xe4, 0x33, 0x30, 0x35, 0x2e, 0x31, 0x69)]
interface IAccPropServices(IAccPropServicesVtbl): IUnknown(IUnknownVtbl) {
    fn SetPropValue(
        pIDString: *const BYTE,
        dwIDStringLen: DWORD,
        idProp: MSAAPROPID,
        var: VARIANT,
    ) -> HRESULT,
    fn SetPropServer(
        pIDString: *const BYTE,
        dwIDStringLen: DWORD,
        paProps: *const MSAAPROPID,
        cProps: c_int,
        pServer: *mut IAccPropServer,
        annoScope: AnnoScope,
    ) -> HRESULT,
    fn ClearProps(
        pIDString: *const BYTE,
        dwIDStringLen: DWORD,
        paProps: *const MSAAPROPID,
        cProps: c_int,
    ) -> HRESULT,
    fn SetHwndProp(
        hwnd: HWND,
        idObject: DWORD,
        idChild: DWORD,
        idProp: MSAAPROPID,
        var: VARIANT,
    ) -> HRESULT,
    fn SetHwndPropStr(
        hwnd: HWND,
        idObject: DWORD,
        idChild: DWORD,
        idProp: MSAAPROPID,
        str_: LPCWSTR,
    ) -> HRESULT,
    fn SetHwndPropServer(
        hwnd: HWND,
        idObject: DWORD,
        idChild: DWORD,
        paProps: *const MSAAPROPID,
        cProps: c_int,
        pServer: *mut IAccPropServer,
        annoScope: AnnoScope,
    ) -> HRESULT,
    fn ClearHwndProps(
        hwnd: HWND,
        idObject: DWORD,
        idChild: DWORD,
        paProps: *const MSAAPROPID,
        cProps: c_int,
    ) -> HRESULT,
    fn ComposeHwndIdentityString(
        hwnd: HWND,
        idObject: DWORD,
        idChild: DWORD,
        ppIDString: *mut *mut BYTE,
        pdwIDStringLen: *mut DWORD,
    ) -> HRESULT,
    fn DecomposeHwndIdentityString(
        pIDString: *const BYTE,
        dwIDStringLen: DWORD,
        phwnd: *mut HWND,
        pidObject: *mut DWORD,
        pidChild: *mut DWORD,
    ) -> HRESULT,
    fn SetHmenuProp(
        hmenu: HMENU,
        idChild: DWORD,
        idProp: MSAAPROPID,
        var: VARIANT,
    ) -> HRESULT,
    fn SetHmenuPropStr(
        hmenu: HMENU,
        idChild: DWORD,
        idProp: MSAAPROPID,
        str_: LPCWSTR,
    ) -> HRESULT,
    fn SetHmenuPropServer(
        hmenu: HMENU,
        idChild: DWORD,
        paProps: *const MSAAPROPID,
        cProps: c_int,
        pServer: *mut IAccPropServer,
        annoScope: AnnoScope,
    ) -> HRESULT,
    fn ClearHmenuProps(
        hmenu: HMENU,
        idChild: DWORD,
        paProps: *const MSAAPROPID,
        cProps: c_int,
    ) -> HRESULT,
    fn ComposeHmenuIdentityString(
        hmenu: HMENU,
        idChild: DWORD,
        ppIDString: *mut *mut BYTE,
        pdwIDStringLen: *mut DWORD,
    ) -> HRESULT,
    fn DecomposeHmenuIdentityString(
        pIDString: *const BYTE,
        dwIDStringLen: DWORD,
        phmenu: *mut HMENU,
        pidChild: *mut DWORD,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xb5f8350b, 0x0548, 0x48b1, 0xa6, 0xee, 0x88, 0xbd, 0x00, 0xb4, 0xa5, 0xe7)]
class CAccPropServices;}
FN!{stdcall BSTR_UserSize(
    *mut c_ulong,
    c_ulong,
    *mut BSTR,
) -> c_ulong}
FN!{stdcall BSTR_UserMarshal(
    *mut c_ulong,
    *mut c_uchar,
    *mut BSTR,
) -> *mut c_uchar}
FN!{stdcall BSTR_UserUnmarshal(
    *mut c_ulong,
    *mut c_uchar,
    *mut BSTR,
) -> *mut c_uchar}
FN!{stdcall BSTR_UserFree(
    *mut c_ulong,
    *mut BSTR,
) -> c_void}
FN!{stdcall HMENU_UserSize(
    *mut c_ulong,
    c_ulong,
    *mut HMENU,
) -> c_ulong}
FN!{stdcall HMENU_UserMarshal(
    *mut c_ulong,
    *mut c_uchar,
    *mut HMENU,
) -> *mut c_uchar}
FN!{stdcall HMENU_UserUnmarshal(
    *mut c_ulong,
    *mut c_uchar,
    *mut HMENU,
) -> *mut c_uchar}
FN!{stdcall HMENU_UserFree(
    *mut c_ulong,
    *mut HMENU,
) -> c_void}
FN!{stdcall HWND_UserSize(
    *mut c_ulong,
    c_ulong,
    *mut HWND,
) -> c_ulong}
FN!{stdcall HWND_UserMarshal(
    *mut c_ulong,
    *mut c_uchar,
    *mut HWND,
) -> *mut c_uchar}
FN!{stdcall HWND_UserUnmarshal(
    *mut c_ulong,
    *mut c_uchar,
    *mut HWND,
) -> *mut c_uchar}
FN!{stdcall HWND_UserFree(
    *mut c_ulong,
    *mut HWND,
) -> c_void}
FN!{stdcall LPSAFEARRAY_UserSize(
    *mut c_ulong,
    c_ulong,
    *mut LPSAFEARRAY,
) -> c_ulong}
FN!{stdcall LPSAFEARRAY_UserMarshal(
    *mut c_ulong,
    *mut c_uchar,
    *mut LPSAFEARRAY,
) -> *mut c_uchar}
FN!{stdcall LPSAFEARRAY_UserUnmarshal(
    *mut c_ulong,
    *mut c_uchar,
    *mut LPSAFEARRAY,
) -> *mut c_uchar}
FN!{stdcall LPSAFEARRAY_UserFree(
    *mut c_ulong,
    *mut LPSAFEARRAY,
) -> c_void}
FN!{stdcall VARIANT_UserSize(
    *mut c_ulong,
    c_ulong,
    *mut VARIANT,
) -> c_ulong}
FN!{stdcall VARIANT_UserMarshal(
    *mut c_ulong,
    *mut c_uchar,
    *mut VARIANT,
) -> *mut c_uchar}
FN!{stdcall VARIANT_UserUnmarshal(
    *mut c_ulong,
    *mut c_uchar,
    *mut VARIANT,
) -> *mut c_uchar}
FN!{stdcall VARIANT_UserFree(
    *mut c_ulong,
    *mut VARIANT,
) -> c_void}

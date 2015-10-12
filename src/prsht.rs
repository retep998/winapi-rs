// Copyright © 2015, skdltmxn
// Licensed under the MIT License <LICENSE.md>
//! Interface for the Windows Property Sheet Pages
#[repr(C)] #[allow(missing_copy_implementations)]
pub struct PSP {
    unused: ::c_void,
}
pub type HPROPSHEETPAGE = *mut PSP;
pub type LPFNPSPCALLBACKA = Option<unsafe extern "system" fn(
    hwnd: ::HWND, uMsg: ::UINT, ppsp: *mut PROPSHEETPAGEA,
) -> ::UINT>;
pub type LPFNPSPCALLBACKW = Option<unsafe extern "system" fn(
    hwnd: ::HWND, uMsg: ::UINT, ppsp: *mut PROPSHEETPAGEW,
) -> ::UINT>;
pub const PSP_DEFAULT: ::DWORD = 0x00000000;
pub const PSP_DLGINDIRECT: ::DWORD = 0x00000001;
pub const PSP_USEHICON: ::DWORD = 0x00000002;
pub const PSP_USEICONID: ::DWORD = 0x00000004;
pub const PSP_USETITLE: ::DWORD = 0x00000008;
pub const PSP_RTLREADING: ::DWORD = 0x00000010;
pub const PSP_HASHELP: ::DWORD = 0x00000020;
pub const PSP_USEREFPARENT: ::DWORD = 0x00000040;
pub const PSP_USECALLBACK: ::DWORD = 0x00000080;
pub const PSP_PREMATURE: ::DWORD = 0x00000400;
pub const PSP_HIDEHEADER: ::DWORD = 0x00000800;
pub const PSP_USEHEADERTITLE: ::DWORD = 0x00001000;
pub const PSP_USEHEADERSUBTITLE: ::DWORD = 0x00002000;
pub const PSP_USEFUSIONCONTEXT: ::DWORD = 0x00004000;
pub const PSPCB_ADDREF: ::UINT = 0;
pub const PSPCB_RELEASE: ::UINT = 1;
pub const PSPCB_CREATE: ::UINT = 2;
pub type PROPSHEETPAGE_RESOURCE = ::LPCDLGTEMPLATEA;
#[repr(C)] #[derive(Copy)]
pub struct PROPSHEETPAGEA_V4 {
    pub dwSize: ::DWORD,
    pub dwFlags: ::DWORD,
    pub hInstance: ::HINSTANCE,
    pub pszTemplate: ::LPCSTR,
    pub hIcon: ::HICON,
    pub pszTitle: ::LPCSTR,
    pub pfnDlgProc: ::DLGPROC,
    pub lParam: ::LPARAM,
    pub pfnCallback: LPFNPSPCALLBACKA,
    pub pcRefParent: *mut ::UINT,
    pub pszHeaderTitle: ::LPCSTR,
    pub pszHeaderSubTitle: ::LPCSTR,
    pub hActCtx: ::HANDLE,
    pub hbmHeader: ::HBITMAP
}
impl Clone for PROPSHEETPAGEA_V4 { fn clone(&self) -> PROPSHEETPAGEA_V4 { *self } }
UNION!(PROPSHEETPAGEA_V4, pszTemplate, pResource, pResource_mut, PROPSHEETPAGE_RESOURCE);
UNION!(PROPSHEETPAGEA_V4, hIcon, pszIcon, pszIcon_mut, ::LPCSTR);
UNION!(PROPSHEETPAGEA_V4, hbmHeader, pszbmHeader, pszbmHeader_mut, ::LPCSTR);
pub type LPPROPSHEETPAGEA_V4 = *mut PROPSHEETPAGEA_V4;
pub type LPCPROPSHEETPAGEA_V4 = *const PROPSHEETPAGEA_V4;
#[repr(C)] #[derive(Copy)]
pub struct PROPSHEETPAGEW_V4 {
    pub dwSize: ::DWORD,
    pub dwFlags: ::DWORD,
    pub hInstance: ::HINSTANCE,
    pub pszTemplate: ::LPCWSTR,
    pub hIcon: ::HICON,
    pub pszTitle: ::LPCWSTR,
    pub pfnDlgProc: ::DLGPROC,
    pub lParam: ::LPARAM,
    pub pfnCallback: LPFNPSPCALLBACKW,
    pub pcRefParent: *mut ::UINT,
    pub pszHeaderTitle: ::LPCWSTR,
    pub pszHeaderSubTitle: ::LPCWSTR,
    pub hActCtx: ::HANDLE,
    pub hbmHeader: ::HBITMAP
}
impl Clone for PROPSHEETPAGEW_V4 { fn clone(&self) -> PROPSHEETPAGEW_V4 { *self } }
UNION!(PROPSHEETPAGEW_V4, pszTemplate, pResource, pResource_mut, PROPSHEETPAGE_RESOURCE);
UNION!(PROPSHEETPAGEW_V4, hIcon, pszIcon, pszIcon_mut, ::LPCWSTR);
UNION!(PROPSHEETPAGEW_V4, hbmHeader, pszbmHeader, pszbmHeader_mut, ::LPCWSTR);
pub type LPPROPSHEETPAGEW_V4 = *mut PROPSHEETPAGEW_V4;
pub type LPCPROPSHEETPAGEW_V4 = *const PROPSHEETPAGEW_V4;
pub type PROPSHEETPAGEA_LATEST = PROPSHEETPAGEA_V4;
pub type PROPSHEETPAGEW_LATEST = PROPSHEETPAGEW_V4;
pub type LPPROPSHEETPAGEA_LATEST = LPPROPSHEETPAGEA_V4;
pub type LPPROPSHEETPAGEW_LATEST = LPPROPSHEETPAGEW_V4;
pub type LPCPROPSHEETPAGEA_LATEST = LPCPROPSHEETPAGEA_V4;
pub type LPCPROPSHEETPAGEW_LATEST = LPCPROPSHEETPAGEW_V4;
pub type PROPSHEETPAGEA = PROPSHEETPAGEA_V4;
pub type PROPSHEETPAGEW = PROPSHEETPAGEW_V4;
pub type LPPROPSHEETPAGEA = LPPROPSHEETPAGEA_V4;
pub type LPPROPSHEETPAGEW = LPPROPSHEETPAGEW_V4;
pub type LPCPROPSHEETPAGEA = LPCPROPSHEETPAGEA_V4;
pub type LPCPROPSHEETPAGEW = LPCPROPSHEETPAGEW_V4;
pub const PSH_DEFAULT: ::DWORD = 0x00000000;
pub const PSH_PROPTITLE: ::DWORD = 0x00000001;
pub const PSH_USEHICON: ::DWORD = 0x00000002;
pub const PSH_USEICONID: ::DWORD = 0x00000004;
pub const PSH_PROPSHEETPAGE: ::DWORD = 0x00000008;
pub const PSH_WIZARDHASFINISH: ::DWORD = 0x00000010;
pub const PSH_WIZARD: ::DWORD = 0x00000020;
pub const PSH_USEPSTARTPAGE: ::DWORD = 0x00000040;
pub const PSH_NOAPPLYNOW: ::DWORD = 0x00000080;
pub const PSH_USECALLBACK: ::DWORD = 0x00000100;
pub const PSH_HASHELP: ::DWORD = 0x00000200;
pub const PSH_MODELESS: ::DWORD = 0x00000400;
pub const PSH_RTLREADING: ::DWORD = 0x00000800;
pub const PSH_WIZARDCONTEXTHELP: ::DWORD = 0x00001000;
pub const PSH_WIZARD97: ::DWORD = 0x01000000;
pub const PSH_WATERMARK: ::DWORD = 0x00008000;
pub const PSH_USEHBMWATERMARK: ::DWORD = 0x00010000;
pub const PSH_USEHPLWATERMARK: ::DWORD = 0x00020000;
pub const PSH_STRETCHWATERMARK: ::DWORD = 0x00040000;
pub const PSH_HEADER: ::DWORD = 0x00080000;
pub const PSH_USEHBMHEADER: ::DWORD = 0x00100000;
pub const PSH_USEPAGELANG: ::DWORD = 0x00200000;
pub const PSH_WIZARD_LITE: ::DWORD = 0x00400000;
pub const PSH_NOCONTEXTHELP: ::DWORD = 0x02000000;
pub const PSH_AEROWIZARD: ::DWORD = 0x00004000;
pub const PSH_RESIZABLE: ::DWORD = 0x04000000;
pub const PSH_HEADERBITMAP: ::DWORD = 0x08000000;
pub const PSH_NOMARGIN: ::DWORD = 0x10000000;
pub type PFNPROPSHEETCALLBACK = Option<unsafe extern "system" fn(
    ::HWND, ::UINT, ::LPARAM,
) -> ::c_int>;
#[repr(C)] #[derive(Copy)]
pub struct PROPSHEETHEADERA_V2 {
    pub dwSize: ::DWORD,
    pub dwFlags: ::DWORD,
    pub hwndParent: ::HWND,
    pub hInstance: ::HINSTANCE,
    pub hIcon: ::HICON,
    pub pszCaption: ::LPCSTR,
    pub nPages: ::UINT,
    pub pStartPage: ::LPCSTR,
    pub ppsp: LPCPROPSHEETPAGEA,
    pub pfnCallback: PFNPROPSHEETCALLBACK,
    pub hbmWatermark: ::HBITMAP,
    pub hplWatermark: ::HPALETTE,
    pub hbmHeader: ::HBITMAP,
}
impl Clone for PROPSHEETHEADERA_V2 { fn clone(&self) -> PROPSHEETHEADERA_V2 { *self } }
UNION!(PROPSHEETHEADERA_V2, hIcon, pszIcon, pszIcon_mut, ::LPCSTR);
UNION!(PROPSHEETHEADERA_V2, pStartPage, nStartPage, nStartPage_mut, ::UINT);
UNION!(PROPSHEETHEADERA_V2, ppsp, phpage, phpage_mut, *mut HPROPSHEETPAGE);
UNION!(PROPSHEETHEADERA_V2, hbmWatermark, pszbmWatermark, pszbmWatermark_mut, ::LPCSTR);
UNION!(PROPSHEETHEADERA_V2, hbmHeader, pszbmHeader, pszbmHeader_mut, ::LPCSTR);
pub type LPPROPSHEETHEADERA_V2 = *mut PROPSHEETHEADERA_V2;
pub type LPCPROPSHEETHEADERA_V2 = *const PROPSHEETHEADERA_V2;
#[repr(C)] #[derive(Copy)]
pub struct PROPSHEETHEADERW_V2 {
    pub dwSize: ::DWORD,
    pub dwFlags: ::DWORD,
    pub hwndParent: ::HWND,
    pub hInstance: ::HINSTANCE,
    pub hIcon: ::HICON,
    pub pszCaption: ::LPCWSTR,
    pub nPages: ::UINT,
    pub pStartPage: ::LPCWSTR,
    pub ppsp: LPCPROPSHEETPAGEW,
    pub pfnCallback: PFNPROPSHEETCALLBACK,
    pub hbmWatermark: ::HBITMAP,
    pub hplWatermark: ::HPALETTE,
    pub hbmHeader: ::HBITMAP,
}
impl Clone for PROPSHEETHEADERW_V2 { fn clone(&self) -> PROPSHEETHEADERW_V2 { *self } }
UNION!(PROPSHEETHEADERW_V2, hIcon, pszIcon, pszIcon_mut, ::LPCWSTR);
UNION!(PROPSHEETHEADERW_V2, pStartPage, nStartPage, nStartPage_mut, ::UINT);
UNION!(PROPSHEETHEADERW_V2, ppsp, phpage, phpage_mut, *mut HPROPSHEETPAGE);
UNION!(PROPSHEETHEADERW_V2, hbmWatermark, pszbmWatermark, pszbmWatermark_mut, ::LPCWSTR);
UNION!(PROPSHEETHEADERW_V2, hbmHeader, pszbmHeader, pszbmHeader_mut, ::LPCWSTR);
pub type LPPROPSHEETHEADERW_V2 = *mut PROPSHEETHEADERW_V2;
pub type LPCPROPSHEETHEADERW_V2 = *const PROPSHEETHEADERW_V2;
pub type PROPSHEETHEADERA = PROPSHEETHEADERA_V2;
pub type PROPSHEETHEADERW = PROPSHEETHEADERW_V2;
pub type LPPROPSHEETHEADERA = LPPROPSHEETHEADERA_V2;
pub type LPPROPSHEETHEADERW = LPPROPSHEETHEADERW_V2;
pub type LPCPROPSHEETHEADERA = LPCPROPSHEETHEADERA_V2;
pub type LPCPROPSHEETHEADERW = LPCPROPSHEETHEADERW_V2;
pub const PSCB_INITIALIZED: ::UINT = 1;
pub const PSCB_PRECREATE: ::UINT = 2;
pub const PSCB_BUTTONPRESSED: ::UINT = 3;
pub type LPFNADDPROPSHEETPAGE = Option<unsafe extern "system" fn(
    HPROPSHEETPAGE, ::LPARAM,
) -> ::BOOL>;
pub type LPFNADDPROPSHEETPAGES = Option<unsafe extern "system" fn(
    ::LPVOID, LPFNADDPROPSHEETPAGE, ::LPARAM,
) -> ::BOOL>;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct PSHNOTIFY {
    pub hdr: ::NMHDR,
    pub lParam: ::LPARAM,
}
pub type LPPSHNOTIFY = *mut PSHNOTIFY;
pub const PSN_FIRST: ::UINT = (0 - 200);
pub const PSN_LAST: ::UINT = (0 - 299);
pub const PSN_SETACTIVE: ::UINT = PSN_FIRST - 0;
pub const PSN_KILLACTIVE: ::UINT = PSN_FIRST - 1;
pub const PSN_APPLY: ::UINT = PSN_FIRST - 2;
pub const PSN_RESET: ::UINT = PSN_FIRST - 3;
pub const PSN_HELP: ::UINT = PSN_FIRST - 5;
pub const PSN_WIZBACK: ::UINT = PSN_FIRST - 6;
pub const PSN_WIZNEXT: ::UINT = PSN_FIRST - 7;
pub const PSN_WIZFINISH: ::UINT = PSN_FIRST - 8;
pub const PSN_QUERYCANCEL: ::UINT = PSN_FIRST - 9;
pub const PSN_GETOBJECT: ::UINT = PSN_FIRST - 10;
pub const PSN_TRANSLATEACCELERATOR: ::UINT = PSN_FIRST - 12;
pub const PSN_QUERYINITIALFOCUS: ::UINT = PSN_FIRST - 13;
pub const PSNRET_NOERROR: ::LRESULT = 0;
pub const PSNRET_INVALID: ::LRESULT = 1;
pub const PSNRET_INVALID_NOCHANGEPAGE: ::LRESULT = 2;
pub const PSNRET_MESSAGEHANDLED: ::LRESULT = 3;
pub const PSM_SETCURSEL: ::UINT = ::WM_USER + 101;
pub const PSM_REMOVEPAGE: ::UINT = ::WM_USER + 102;
pub const PSM_ADDPAGE: ::UINT = ::WM_USER + 103;
pub const PSM_CHANGED: ::UINT = ::WM_USER + 104;
pub const PSM_RESTARTWINDOWS: ::UINT = ::WM_USER + 105;
pub const PSM_REBOOTSYSTEM: ::UINT = ::WM_USER + 106;
pub const PSM_CANCELTOCLOSE: ::UINT = ::WM_USER + 107;
pub const PSM_QUERYSIBLINGS: ::UINT = ::WM_USER + 108;
pub const PSM_UNCHANGED: ::UINT = ::WM_USER + 109;
pub const PSM_APPLY: ::UINT = ::WM_USER + 110;
pub const PSM_SETTITLEA: ::UINT = ::WM_USER + 111;
pub const PSM_SETTITLEW: ::UINT = ::WM_USER + 120;
pub const PSM_SETWIZBUTTONS: ::UINT = ::WM_USER + 112;
pub const PSWIZB_BACK: ::DWORD = 0x00000001;
pub const PSWIZB_NEXT: ::DWORD = 0x00000002;
pub const PSWIZB_FINISH: ::DWORD = 0x00000004;
pub const PSWIZB_DISABLEDFINISH: ::DWORD = 0x00000008;
pub const PSWIZB_CANCEL: ::DWORD = 0x00000008;
pub const PSWIZBF_ELEVATIONREQUIRED: ::WPARAM = 0x00000001;
pub const PSBTN_BACK: ::c_int = 0;
pub const PSBTN_NEXT: ::c_int = 1;
pub const PSBTN_FINISH: ::c_int = 2;
pub const PSBTN_OK: ::c_int = 3;
pub const PSBTN_APPLYNOW: ::c_int = 4;
pub const PSBTN_CANCEL: ::c_int = 5;
pub const PSBTN_HELP: ::c_int = 6;
pub const PSBTN_MAX: ::c_int = 6;
pub const PSM_PRESSBUTTON: ::UINT = ::WM_USER + 113;
pub const PSM_SETCURSELID: ::UINT = ::WM_USER + 114;
pub const PSM_SETFINISHTEXTA: ::UINT = ::WM_USER + 115;
pub const PSM_SETFINISHTEXTW: ::UINT = ::WM_USER + 121;
pub const PSM_GETTABCONTROL: ::UINT = ::WM_USER + 116;
pub const PSM_ISDIALOGMESSAGE: ::UINT = ::WM_USER + 117;
pub const PSM_GETCURRENTPAGEHWND: ::UINT = ::WM_USER + 118;
pub const PSM_INSERTPAGE: ::UINT = ::WM_USER + 119;
pub const PSM_SETHEADERTITLEA: ::UINT = ::WM_USER + 125;
pub const PSM_SETHEADERTITLEW: ::UINT = ::WM_USER + 126;
pub const PSWIZF_SETCOLOR: ::UINT = (0 - 1) as ::UINT;
pub const PSM_SETHEADERSUBTITLEA: ::UINT = ::WM_USER + 127;
pub const PSM_SETHEADERSUBTITLEW: ::UINT = ::WM_USER + 128;
pub const PSM_HWNDTOINDEX: ::UINT = ::WM_USER + 129;
pub const PSM_INDEXTOHWND: ::UINT = ::WM_USER + 130;
pub const PSM_PAGETOINDEX: ::UINT = ::WM_USER + 131;
pub const PSM_INDEXTOPAGE: ::UINT = ::WM_USER + 132;
pub const PSM_IDTOINDEX: ::UINT = ::WM_USER + 133;
pub const PSM_INDEXTOID: ::UINT = ::WM_USER + 134;
pub const PSM_GETRESULT: ::UINT = ::WM_USER + 135;
pub const PSM_RECALCPAGESIZES: ::UINT = ::WM_USER + 136;
pub const PSM_SETNEXTTEXTW: ::UINT = ::WM_USER + 137;
pub const PSM_SHOWWIZBUTTONS: ::UINT = ::WM_USER + 138;
pub const PSM_ENABLEWIZBUTTONS: ::UINT = ::WM_USER + 139;
pub const PSM_SETBUTTONTEXTW: ::UINT = ::WM_USER + 140;
pub const PSM_SETBUTTONTEXT: ::UINT = PSM_SETBUTTONTEXTW;
pub const ID_PSRESTARTWINDOWS: ::INT_PTR = 0x2;
pub const ID_PSREBOOTSYSTEM: ::INT_PTR = ID_PSRESTARTWINDOWS | 0x1;
pub const WIZ_CXDLG: ::DWORD = 276;
pub const WIZ_CYDLG: ::DWORD = 140;
pub const WIZ_CXBMP: ::DWORD = 80;
pub const WIZ_BODYX: ::DWORD = 92;
pub const WIZ_BODYCX: ::DWORD = 184;
pub const PROP_SM_CXDLG: ::c_short = 212;
pub const PROP_SM_CYDLG: ::c_short = 188;
pub const PROP_MED_CXDLG: ::c_short = 227;
pub const PROP_MED_CYDLG: ::c_short = 215;
pub const PROP_LG_CXDLG: ::c_short = 252;
pub const PROP_LG_CYDLG: ::c_short = 218;

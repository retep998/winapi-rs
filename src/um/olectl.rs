// Copyright © 2015-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use ctypes::{c_float, c_int};
use shared::guiddef::{CLSID, LPCLSID, REFIID};
use shared::minwindef::{BOOL, DWORD, HINSTANCE, HMETAFILE, LPVOID, UINT, ULONG};
use shared::windef::{COLORREF, HBITMAP, HCURSOR, HENHMETAFILE, HICON, HPALETTE, HWND};
use shared::winerror::{FACILITY_ITF, SEVERITY_ERROR, SEVERITY_SUCCESS};
use shared::wtypes::{BSTR, CY, VARIANT_BOOL, VT_BOOL, VT_DISPATCH, VT_I2, VT_I4};
use shared::wtypesbase::{LPCOLESTR, LPOLESTR};
use um::oaidl::{DISPID, LPDISPATCH, VARIANT};
use um::objidlbase::LPSTREAM;
use um::ocidl::OLE_COLOR;
use um::unknwnbase::LPUNKNOWN;
use um::winnt::{HRESULT, LCID, LONG, SHORT};
use um::winuser::{
    WM_CHARTOITEM, WM_COMMAND, WM_COMPAREITEM, WM_CTLCOLORBTN, WM_CTLCOLORDLG, WM_CTLCOLOREDIT,
    WM_CTLCOLORLISTBOX, WM_CTLCOLORMSGBOX, WM_CTLCOLORSCROLLBAR, WM_CTLCOLORSTATIC, WM_DELETEITEM,
    WM_DRAWITEM, WM_HSCROLL, WM_MEASUREITEM, WM_NOTIFY, WM_PARENTNOTIFY, WM_USER, WM_VKEYTOITEM,
    WM_VSCROLL
};
DEFINE_GUID!{IID_IPropertyFrame,
    0xB196B28A,0xBAB4,0x101A,0xB6,0x9C,0x00,0xAA,0x00,0x34,0x1D,0x07}
DEFINE_GUID!{CLSID_CFontPropPage,
    0x0be35200,0x8f91,0x11ce,0x9d,0xe3,0x00,0xaa,0x00,0x4b,0xb8,0x51}
DEFINE_GUID!{CLSID_CColorPropPage,
    0x0be35201,0x8f91,0x11ce,0x9d,0xe3,0x00,0xaa,0x00,0x4b,0xb8,0x51}
DEFINE_GUID!{CLSID_CPicturePropPage,
    0x0be35202,0x8f91,0x11ce,0x9d,0xe3,0x00,0xaa,0x00,0x4b,0xb8,0x51}
DEFINE_GUID!{CLSID_PersistPropset,
    0xfb8f0821,0x0164,0x101b,0x84,0xed,0x08,0x00,0x2b,0x2e,0xc7,0x13}
DEFINE_GUID!{CLSID_ConvertVBX,
    0xfb8f0822,0x0164,0x101b,0x84,0xed,0x08,0x00,0x2b,0x2e,0xc7,0x13}
DEFINE_GUID!{CLSID_StdFont,
    0x0be35203,0x8f91,0x11ce,0x9d,0xe3,0x00,0xaa,0x00,0x4b,0xb8,0x51}
DEFINE_GUID!{CLSID_StdPicture,
    0x0be35204,0x8f91,0x11ce,0x9d,0xe3,0x00,0xaa,0x00,0x4b,0xb8,0x51}
DEFINE_GUID!{GUID_HIMETRIC,
    0x66504300,0xBE0F,0x101A,0x8B,0xBB,0x00,0xAA,0x00,0x30,0x0C,0xAB}
DEFINE_GUID!{GUID_COLOR,
    0x66504301,0xBE0F,0x101A,0x8B,0xBB,0x00,0xAA,0x00,0x30,0x0C,0xAB}
DEFINE_GUID!{GUID_XPOSPIXEL,
    0x66504302,0xBE0F,0x101A,0x8B,0xBB,0x00,0xAA,0x00,0x30,0x0C,0xAB}
DEFINE_GUID!{GUID_YPOSPIXEL,
    0x66504303,0xBE0F,0x101A,0x8B,0xBB,0x00,0xAA,0x00,0x30,0x0C,0xAB}
DEFINE_GUID!{GUID_XSIZEPIXEL,
    0x66504304,0xBE0F,0x101A,0x8B,0xBB,0x00,0xAA,0x00,0x30,0x0C,0xAB}
DEFINE_GUID!{GUID_YSIZEPIXEL,
    0x66504305,0xBE0F,0x101A,0x8B,0xBB,0x00,0xAA,0x00,0x30,0x0C,0xAB}
DEFINE_GUID!{GUID_XPOS,
    0x66504306,0xBE0F,0x101A,0x8B,0xBB,0x00,0xAA,0x00,0x30,0x0C,0xAB}
DEFINE_GUID!{GUID_YPOS,
    0x66504307,0xBE0F,0x101A,0x8B,0xBB,0x00,0xAA,0x00,0x30,0x0C,0xAB}
DEFINE_GUID!{GUID_XSIZE,
    0x66504308,0xBE0F,0x101A,0x8B,0xBB,0x00,0xAA,0x00,0x30,0x0C,0xAB}
DEFINE_GUID!{GUID_YSIZE,
    0x66504309,0xBE0F,0x101A,0x8B,0xBB,0x00,0xAA,0x00,0x30,0x0C,0xAB}
DEFINE_GUID!{GUID_TRISTATE,
    0x6650430A,0xBE0F,0x101A,0x8B,0xBB,0x00,0xAA,0x00,0x30,0x0C,0xAB}
DEFINE_GUID!{GUID_OPTIONVALUEEXCLUSIVE,
    0x6650430B,0xBE0F,0x101A,0x8B,0xBB,0x00,0xAA,0x00,0x30,0x0C,0xAB}
DEFINE_GUID!{GUID_CHECKVALUEEXCLUSIVE,
    0x6650430C,0xBE0F,0x101A,0x8B,0xBB,0x00,0xAA,0x00,0x30,0x0C,0xAB}
DEFINE_GUID!{GUID_FONTNAME,
    0x6650430D,0xBE0F,0x101A,0x8B,0xBB,0x00,0xAA,0x00,0x30,0x0C,0xAB}
DEFINE_GUID!{GUID_FONTSIZE,
    0x6650430E,0xBE0F,0x101A,0x8B,0xBB,0x00,0xAA,0x00,0x30,0x0C,0xAB}
DEFINE_GUID!{GUID_FONTBOLD,
    0x6650430F,0xBE0F,0x101A,0x8B,0xBB,0x00,0xAA,0x00,0x30,0x0C,0xAB}
DEFINE_GUID!{GUID_FONTITALIC,
    0x66504310,0xBE0F,0x101A,0x8B,0xBB,0x00,0xAA,0x00,0x30,0x0C,0xAB}
DEFINE_GUID!{GUID_FONTUNDERSCORE,
    0x66504311,0xBE0F,0x101A,0x8B,0xBB,0x00,0xAA,0x00,0x30,0x0C,0xAB}
DEFINE_GUID!{GUID_FONTSTRIKETHROUGH,
    0x66504312,0xBE0F,0x101A,0x8B,0xBB,0x00,0xAA,0x00,0x30,0x0C,0xAB}
DEFINE_GUID!{GUID_HANDLE,
    0x66504313,0xBE0F,0x101A,0x8B,0xBB,0x00,0xAA,0x00,0x30,0x0C,0xAB}
STRUCT!{struct OCPFIPARAMS {
    cbStructSize: ULONG,
    hWndOwner: HWND,
    x: c_int,
    y: c_int,
    lpszCaption: LPCOLESTR,
    cObjects: ULONG,
    lplpUnk: *mut LPUNKNOWN,
    cPages: ULONG,
    lpPages: *mut CLSID,
    lcid: LCID,
    dispidInitialProperty: DISPID,
}}
pub type LPOCPFIPARAMS = *mut OCPFIPARAMS;
// #define FONTSIZE(n) { n##0000, 0 }
STRUCT!{struct FONTDESC {
    cbSizeofstruct: UINT,
    lpstrName: LPOLESTR,
    cySize: CY,
    sWeight: SHORT,
    sCharset: SHORT,
    fItalic: BOOL,
    fUnderline: BOOL,
    fStrikethrough: BOOL,
}}
pub type LPFONTDESC = *mut FONTDESC;
pub const PICTYPE_UNINITIALIZED: UINT = -1i32 as u32;
pub const PICTYPE_NONE: UINT = 0;
pub const PICTYPE_BITMAP: UINT = 1;
pub const PICTYPE_METAFILE: UINT = 2;
pub const PICTYPE_ICON: UINT = 3;
pub const PICTYPE_ENHMETAFILE: UINT = 4;
STRUCT!{struct PICTDESC_bmp {
    hbitmap: HBITMAP,
    hpal: HPALETTE,
}}
STRUCT!{struct PICTDESC_wmf {
    hmeta: HMETAFILE,
    xExt: c_int,
    yExt: c_int,
}}
STRUCT!{struct PICTDESC_icon {
    hicon: HICON,
}}
STRUCT!{struct PICTDESC_emf {
    hemf: HENHMETAFILE,
}}
UNION!{union PICTDESC_u {
    [u32; 3] [u64; 2],
    bmp bmp_mut: PICTDESC_bmp,
    wmf wmf_mut: PICTDESC_wmf,
    icon icon_mut: PICTDESC_icon,
    emf emf_mut: PICTDESC_emf,
}}
STRUCT!{struct PICTDESC {
    cbSizeofstruct: UINT,
    picType: UINT,
    u: PICTDESC_u,
}}
pub type LPPICTDESC = *mut PICTDESC;
pub type OLE_XPOS_PIXELS = LONG;
pub type OLE_YPOS_PIXELS = LONG;
pub type OLE_XSIZE_PIXELS = LONG;
pub type OLE_YSIZE_PIXELS = LONG;
pub type OLE_XPOS_CONTAINER = c_float;
pub type OLE_YPOS_CONTAINER = c_float;
pub type OLE_XSIZE_CONTAINER = c_float;
pub type OLE_YSIZE_CONTAINER = c_float;
ENUM!{enum OLE_TRISTATE {
    triUnchecked = 0,
    triChecked = 1,
    triGray = 2,
}}
pub type OLE_OPTEXCLUSIVE = VARIANT_BOOL;
pub type OLE_CANCELBOOL = VARIANT_BOOL;
pub type OLE_ENABLEDEFAULTBOOL = VARIANT_BOOL;
pub const FACILITY_CONTROL: HRESULT = 0xa;
pub const CTL_E_ILLEGALFUNCTIONCALL: HRESULT = MAKE_SCODE!(SEVERITY_ERROR, FACILITY_CONTROL, 5);
pub const CTL_E_OVERFLOW: HRESULT = MAKE_SCODE!(SEVERITY_ERROR, FACILITY_CONTROL, 6);
pub const CTL_E_OUTOFMEMORY: HRESULT = MAKE_SCODE!(SEVERITY_ERROR, FACILITY_CONTROL, 7);
pub const CTL_E_DIVISIONBYZERO: HRESULT = MAKE_SCODE!(SEVERITY_ERROR, FACILITY_CONTROL, 11);
pub const CTL_E_OUTOFSTRINGSPACE: HRESULT = MAKE_SCODE!(SEVERITY_ERROR, FACILITY_CONTROL, 14);
pub const CTL_E_OUTOFSTACKSPACE: HRESULT = MAKE_SCODE!(SEVERITY_ERROR, FACILITY_CONTROL, 28);
pub const CTL_E_BADFILENAMEORNUMBER: HRESULT = MAKE_SCODE!(SEVERITY_ERROR, FACILITY_CONTROL, 52);
pub const CTL_E_FILENOTFOUND: HRESULT = MAKE_SCODE!(SEVERITY_ERROR, FACILITY_CONTROL, 53);
pub const CTL_E_BADFILEMODE: HRESULT = MAKE_SCODE!(SEVERITY_ERROR, FACILITY_CONTROL, 54);
pub const CTL_E_FILEALREADYOPEN: HRESULT = MAKE_SCODE!(SEVERITY_ERROR, FACILITY_CONTROL, 55);
pub const CTL_E_DEVICEIOERROR: HRESULT = MAKE_SCODE!(SEVERITY_ERROR, FACILITY_CONTROL, 57);
pub const CTL_E_FILEALREADYEXISTS: HRESULT = MAKE_SCODE!(SEVERITY_ERROR, FACILITY_CONTROL, 58);
pub const CTL_E_BADRECORDLENGTH: HRESULT = MAKE_SCODE!(SEVERITY_ERROR, FACILITY_CONTROL, 59);
pub const CTL_E_DISKFULL: HRESULT = MAKE_SCODE!(SEVERITY_ERROR, FACILITY_CONTROL, 61);
pub const CTL_E_BADRECORDNUMBER: HRESULT = MAKE_SCODE!(SEVERITY_ERROR, FACILITY_CONTROL, 63);
pub const CTL_E_BADFILENAME: HRESULT = MAKE_SCODE!(SEVERITY_ERROR, FACILITY_CONTROL, 64);
pub const CTL_E_TOOMANYFILES: HRESULT = MAKE_SCODE!(SEVERITY_ERROR, FACILITY_CONTROL, 67);
pub const CTL_E_DEVICEUNAVAILABLE: HRESULT = MAKE_SCODE!(SEVERITY_ERROR, FACILITY_CONTROL, 68);
pub const CTL_E_PERMISSIONDENIED: HRESULT = MAKE_SCODE!(SEVERITY_ERROR, FACILITY_CONTROL, 70);
pub const CTL_E_DISKNOTREADY: HRESULT = MAKE_SCODE!(SEVERITY_ERROR, FACILITY_CONTROL, 71);
pub const CTL_E_PATHFILEACCESSERROR: HRESULT = MAKE_SCODE!(SEVERITY_ERROR, FACILITY_CONTROL, 75);
pub const CTL_E_PATHNOTFOUND: HRESULT = MAKE_SCODE!(SEVERITY_ERROR, FACILITY_CONTROL, 76);
pub const CTL_E_INVALIDPATTERNSTRING: HRESULT = MAKE_SCODE!(SEVERITY_ERROR, FACILITY_CONTROL, 93);
pub const CTL_E_INVALIDUSEOFNULL: HRESULT = MAKE_SCODE!(SEVERITY_ERROR, FACILITY_CONTROL, 94);
pub const CTL_E_INVALIDFILEFORMAT: HRESULT = MAKE_SCODE!(SEVERITY_ERROR, FACILITY_CONTROL, 321);
pub const CTL_E_INVALIDPROPERTYVALUE: HRESULT = MAKE_SCODE!(SEVERITY_ERROR, FACILITY_CONTROL, 380);
pub const CTL_E_INVALIDPROPERTYARRAYINDEX: HRESULT =
    MAKE_SCODE!(SEVERITY_ERROR, FACILITY_CONTROL, 381);
pub const CTL_E_SETNOTSUPPORTEDATRUNTIME: HRESULT =
    MAKE_SCODE!(SEVERITY_ERROR, FACILITY_CONTROL, 382);
pub const CTL_E_SETNOTSUPPORTED: HRESULT = MAKE_SCODE!(SEVERITY_ERROR, FACILITY_CONTROL, 383);
pub const CTL_E_NEEDPROPERTYARRAYINDEX: HRESULT =
    MAKE_SCODE!(SEVERITY_ERROR, FACILITY_CONTROL, 385);
pub const CTL_E_SETNOTPERMITTED: HRESULT = MAKE_SCODE!(SEVERITY_ERROR, FACILITY_CONTROL, 387);
pub const CTL_E_GETNOTSUPPORTEDATRUNTIME: HRESULT =
    MAKE_SCODE!(SEVERITY_ERROR, FACILITY_CONTROL, 393);
pub const CTL_E_GETNOTSUPPORTED: HRESULT = MAKE_SCODE!(SEVERITY_ERROR, FACILITY_CONTROL, 394);
pub const CTL_E_PROPERTYNOTFOUND: HRESULT = MAKE_SCODE!(SEVERITY_ERROR, FACILITY_CONTROL, 422);
pub const CTL_E_INVALIDCLIPBOARDFORMAT: HRESULT =
    MAKE_SCODE!(SEVERITY_ERROR, FACILITY_CONTROL, 460);
pub const CTL_E_INVALIDPICTURE: HRESULT = MAKE_SCODE!(SEVERITY_ERROR, FACILITY_CONTROL, 481);
pub const CTL_E_PRINTERERROR: HRESULT = MAKE_SCODE!(SEVERITY_ERROR, FACILITY_CONTROL, 482);
pub const CTL_E_CANTSAVEFILETOTEMP: HRESULT = MAKE_SCODE!(SEVERITY_ERROR, FACILITY_CONTROL, 735);
pub const CTL_E_SEARCHTEXTNOTFOUND: HRESULT = MAKE_SCODE!(SEVERITY_ERROR, FACILITY_CONTROL, 744);
pub const CTL_E_REPLACEMENTSTOOLONG: HRESULT = MAKE_SCODE!(SEVERITY_ERROR, FACILITY_CONTROL, 746);
pub const CTL_E_CUSTOM_FIRST: HRESULT = MAKE_SCODE!(SEVERITY_ERROR, FACILITY_CONTROL, 600);
pub const CONNECT_E_FIRST: HRESULT = MAKE_SCODE!(SEVERITY_ERROR, FACILITY_ITF, 0x0200);
pub const CONNECT_E_LAST: HRESULT = MAKE_SCODE!(SEVERITY_ERROR, FACILITY_ITF, 0x020F);
pub const CONNECT_S_FIRST: HRESULT = MAKE_SCODE!(SEVERITY_SUCCESS, FACILITY_ITF, 0x0200);
pub const CONNECT_S_LAST: HRESULT = MAKE_SCODE!(SEVERITY_SUCCESS, FACILITY_ITF, 0x020F);
pub const CONNECT_E_NOCONNECTION: HRESULT = CONNECT_E_FIRST + 0;
pub const CONNECT_E_ADVISELIMIT: HRESULT = CONNECT_E_FIRST + 1;
pub const CONNECT_E_CANNOTCONNECT: HRESULT = CONNECT_E_FIRST + 2;
pub const CONNECT_E_OVERRIDDEN: HRESULT = CONNECT_E_FIRST + 3;
pub const SELFREG_E_FIRST: HRESULT = MAKE_SCODE!(SEVERITY_ERROR, FACILITY_ITF, 0x0200);
pub const SELFREG_E_LAST: HRESULT = MAKE_SCODE!(SEVERITY_ERROR, FACILITY_ITF, 0x020F);
pub const SELFREG_S_FIRST: HRESULT = MAKE_SCODE!(SEVERITY_SUCCESS, FACILITY_ITF, 0x0200);
pub const SELFREG_S_LAST: HRESULT = MAKE_SCODE!(SEVERITY_SUCCESS, FACILITY_ITF, 0x020F);
pub const SELFREG_E_TYPELIB: HRESULT = SELFREG_E_FIRST + 0;
pub const SELFREG_E_CLASS: HRESULT = SELFREG_E_FIRST + 1;
pub const PERPROP_E_FIRST: HRESULT = MAKE_SCODE!(SEVERITY_ERROR, FACILITY_ITF, 0x0200);
pub const PERPROP_E_LAST: HRESULT = MAKE_SCODE!(SEVERITY_ERROR, FACILITY_ITF, 0x020F);
pub const PERPROP_S_FIRST: HRESULT = MAKE_SCODE!(SEVERITY_SUCCESS, FACILITY_ITF, 0x0200);
pub const PERPROP_S_LAST: HRESULT = MAKE_SCODE!(SEVERITY_SUCCESS, FACILITY_ITF, 0x020F);
pub const PERPROP_E_NOPAGEAVAILABLE: HRESULT = PERPROP_E_FIRST + 0;
pub const OLEMISC_INVISIBLEATRUNTIME: DWORD = 0x00000400;
pub const OLEMISC_ALWAYSRUN: DWORD = 0x00000800;
pub const OLEMISC_ACTSLIKEBUTTON: DWORD = 0x00001000;
pub const OLEMISC_ACTSLIKELABEL: DWORD = 0x00002000;
pub const OLEMISC_NOUIACTIVATE: DWORD = 0x00004000;
pub const OLEMISC_ALIGNABLE: DWORD = 0x00008000;
pub const OLEMISC_SIMPLEFRAME: DWORD = 0x00010000;
pub const OLEMISC_SETCLIENTSITEFIRST: DWORD = 0x00020000;
pub const OLEMISC_IMEMODE: DWORD = 0x00040000;
pub const OLEIVERB_PROPERTIES: LONG = -7;
pub const VT_STREAMED_PROPSET: DWORD = 73;
pub const VT_STORED_PROPSET: DWORD = 74;
pub const VT_BLOB_PROPSET: DWORD = 75;
pub const VT_VERBOSE_ENUM: DWORD = 76;
pub const VT_COLOR: DWORD = VT_I4;
pub const VT_XPOS_PIXELS: DWORD = VT_I4;
pub const VT_YPOS_PIXELS: DWORD = VT_I4;
pub const VT_XSIZE_PIXELS: DWORD = VT_I4;
pub const VT_YSIZE_PIXELS: DWORD = VT_I4;
pub const VT_XPOS_HIMETRIC: DWORD = VT_I4;
pub const VT_YPOS_HIMETRIC: DWORD = VT_I4;
pub const VT_XSIZE_HIMETRIC: DWORD = VT_I4;
pub const VT_YSIZE_HIMETRIC: DWORD = VT_I4;
pub const VT_TRISTATE: DWORD = VT_I2;
pub const VT_OPTEXCLUSIVE: DWORD = VT_BOOL;
pub const VT_FONT: DWORD = VT_DISPATCH;
pub const VT_PICTURE: DWORD = VT_DISPATCH;
pub const VT_HANDLE: DWORD = VT_I4;
pub const OCM__BASE: UINT = WM_USER + 0x1c00;
pub const OCM_COMMAND: UINT = OCM__BASE + WM_COMMAND;
pub const OCM_CTLCOLORBTN: UINT = OCM__BASE + WM_CTLCOLORBTN;
pub const OCM_CTLCOLOREDIT: UINT = OCM__BASE + WM_CTLCOLOREDIT;
pub const OCM_CTLCOLORDLG: UINT = OCM__BASE + WM_CTLCOLORDLG;
pub const OCM_CTLCOLORLISTBOX: UINT = OCM__BASE + WM_CTLCOLORLISTBOX;
pub const OCM_CTLCOLORMSGBOX: UINT = OCM__BASE + WM_CTLCOLORMSGBOX;
pub const OCM_CTLCOLORSCROLLBAR: UINT = OCM__BASE + WM_CTLCOLORSCROLLBAR;
pub const OCM_CTLCOLORSTATIC: UINT = OCM__BASE + WM_CTLCOLORSTATIC;
pub const OCM_DRAWITEM: UINT = OCM__BASE + WM_DRAWITEM;
pub const OCM_MEASUREITEM: UINT = OCM__BASE + WM_MEASUREITEM;
pub const OCM_DELETEITEM: UINT = OCM__BASE + WM_DELETEITEM;
pub const OCM_VKEYTOITEM: UINT = OCM__BASE + WM_VKEYTOITEM;
pub const OCM_CHARTOITEM: UINT = OCM__BASE + WM_CHARTOITEM;
pub const OCM_COMPAREITEM: UINT = OCM__BASE + WM_COMPAREITEM;
pub const OCM_HSCROLL: UINT = OCM__BASE + WM_HSCROLL;
pub const OCM_VSCROLL: UINT = OCM__BASE + WM_VSCROLL;
pub const OCM_PARENTNOTIFY: UINT = OCM__BASE + WM_PARENTNOTIFY;
pub const OCM_NOTIFY: UINT = OCM__BASE + WM_NOTIFY;
extern "system" {
    pub fn OleCreatePropertyFrame(
        hwndOwner: HWND,
        x: UINT,
        y: UINT,
        lpszCaption: LPCOLESTR,
        cObjects: ULONG,
        ppUnk: *mut LPUNKNOWN,
        cPages: ULONG,
        pPageClsID: LPCLSID,
        lcid: LCID,
        dwReserved: DWORD,
        pvReserved: LPVOID,
    ) -> HRESULT;
    pub fn OleCreatePropertyFrameIndirect(
        lpParams: LPOCPFIPARAMS,
    ) -> HRESULT;
    pub fn OleTranslateColor(
        clr: OLE_COLOR,
        hpal: HPALETTE,
        lpcolorref: *mut COLORREF,
    ) -> HRESULT;
    pub fn OleCreateFontIndirect(
        lpFontDesc: LPFONTDESC,
        riid: REFIID,
        lplpvObj: *mut LPVOID,
    ) -> HRESULT;
    pub fn OleCreatePictureIndirect(
        lpPictDesc: LPPICTDESC,
        riid: REFIID,
        fOwn: BOOL,
        lplpvObj: *mut LPVOID,
    ) -> HRESULT;
    pub fn OleLoadPicture(
        lpstream: LPSTREAM,
        lSize: LONG,
        fRunmode: BOOL,
        riid: REFIID,
        lplpvObj: *mut LPVOID,
    ) -> HRESULT;
    pub fn OleLoadPictureEx(
        lpstream: LPSTREAM,
        lSize: LONG,
        fRunmode: BOOL,
        riid: REFIID,
        xSizeDesired: DWORD,
        ySizeDesired: DWORD,
        dwFlags: DWORD,
        lplpvObj: *mut LPVOID,
    ) -> HRESULT;
    pub fn OleLoadPicturePath(
        szURLorPath: LPOLESTR,
        punkCaller: LPUNKNOWN,
        dwReserved: DWORD,
        clrReserved: OLE_COLOR,
        riid: REFIID,
        ppvRet: *mut LPVOID,
    ) -> HRESULT;
    pub fn OleLoadPictureFile(
        varFileName: VARIANT,
        lplpdispPicture: *mut LPDISPATCH,
    ) -> HRESULT;
    pub fn OleLoadPictureFileEx(
        varFileName: VARIANT,
        xSizeDesired: DWORD,
        ySizeDesired: DWORD,
        dwFlags: DWORD,
        lplpdispPicture: *mut LPDISPATCH,
    ) -> HRESULT;
    pub fn OleSavePictureFile(
        lpdispPicture: LPDISPATCH,
        bstrFileName: BSTR,
    ) -> HRESULT;
    pub fn OleIconToCursor(
        hinstExe: HINSTANCE,
        hIcon: HICON,
    ) -> HCURSOR;
}
pub const LP_DEFAULT: DWORD = 0x00;
pub const LP_MONOCHROME: DWORD = 0x01;
pub const LP_VGACOLOR: DWORD = 0x02;
pub const LP_COLOR: DWORD = 0x04;
pub const DISPID_AUTOSIZE: DISPID = -500;
pub const DISPID_BACKCOLOR: DISPID = -501;
pub const DISPID_BACKSTYLE: DISPID = -502;
pub const DISPID_BORDERCOLOR: DISPID = -503;
pub const DISPID_BORDERSTYLE: DISPID = -504;
pub const DISPID_BORDERWIDTH: DISPID = -505;
pub const DISPID_DRAWMODE: DISPID = -507;
pub const DISPID_DRAWSTYLE: DISPID = -508;
pub const DISPID_DRAWWIDTH: DISPID = -509;
pub const DISPID_FILLCOLOR: DISPID = -510;
pub const DISPID_FILLSTYLE: DISPID = -511;
pub const DISPID_FONT: DISPID = -512;
pub const DISPID_FORECOLOR: DISPID = -513;
pub const DISPID_ENABLED: DISPID = -514;
pub const DISPID_HWND: DISPID = -515;
pub const DISPID_TABSTOP: DISPID = -516;
pub const DISPID_TEXT: DISPID = -517;
pub const DISPID_CAPTION: DISPID = -518;
pub const DISPID_BORDERVISIBLE: DISPID = -519;
pub const DISPID_APPEARANCE: DISPID = -520;
pub const DISPID_MOUSEPOINTER: DISPID = -521;
pub const DISPID_MOUSEICON: DISPID = -522;
pub const DISPID_PICTURE: DISPID = -523;
pub const DISPID_VALID: DISPID = -524;
pub const DISPID_READYSTATE: DISPID = -525;
pub const DISPID_LISTINDEX: DISPID = -526;
pub const DISPID_SELECTED: DISPID = -527;
pub const DISPID_LIST: DISPID = -528;
pub const DISPID_COLUMN: DISPID = -529;
pub const DISPID_LISTCOUNT: DISPID = -531;
pub const DISPID_MULTISELECT: DISPID = -532;
pub const DISPID_MAXLENGTH: DISPID = -533;
pub const DISPID_PASSWORDCHAR: DISPID = -534;
pub const DISPID_SCROLLBARS: DISPID = -535;
pub const DISPID_WORDWRAP: DISPID = -536;
pub const DISPID_MULTILINE: DISPID = -537;
pub const DISPID_NUMBEROFROWS: DISPID = -538;
pub const DISPID_NUMBEROFCOLUMNS: DISPID = -539;
pub const DISPID_DISPLAYSTYLE: DISPID = -540;
pub const DISPID_GROUPNAME: DISPID = -541;
pub const DISPID_IMEMODE: DISPID = -542;
pub const DISPID_ACCELERATOR: DISPID = -543;
pub const DISPID_ENTERKEYBEHAVIOR: DISPID = -544;
pub const DISPID_TABKEYBEHAVIOR: DISPID = -545;
pub const DISPID_SELTEXT: DISPID = -546;
pub const DISPID_SELSTART: DISPID = -547;
pub const DISPID_SELLENGTH: DISPID = -548;
pub const DISPID_REFRESH: DISPID = -550;
pub const DISPID_DOCLICK: DISPID = -551;
pub const DISPID_ABOUTBOX: DISPID = -552;
pub const DISPID_ADDITEM: DISPID = -553;
pub const DISPID_CLEAR: DISPID = -554;
pub const DISPID_REMOVEITEM: DISPID = -555;
pub const DISPID_CLICK: DISPID = -600;
pub const DISPID_DBLCLICK: DISPID = -601;
pub const DISPID_KEYDOWN: DISPID = -602;
pub const DISPID_KEYPRESS: DISPID = -603;
pub const DISPID_KEYUP: DISPID = -604;
pub const DISPID_MOUSEDOWN: DISPID = -605;
pub const DISPID_MOUSEMOVE: DISPID = -606;
pub const DISPID_MOUSEUP: DISPID = -607;
pub const DISPID_ERROREVENT: DISPID = -608;
pub const DISPID_READYSTATECHANGE: DISPID = -609;
pub const DISPID_CLICK_VALUE: DISPID = -610;
pub const DISPID_RIGHTTOLEFT: DISPID = -611;
pub const DISPID_TOPTOBOTTOM: DISPID = -612;
pub const DISPID_THIS: DISPID = -613;
pub const DISPID_AMBIENT_BACKCOLOR: DISPID = -701;
pub const DISPID_AMBIENT_DISPLAYNAME: DISPID = -702;
pub const DISPID_AMBIENT_FONT: DISPID = -703;
pub const DISPID_AMBIENT_FORECOLOR: DISPID = -704;
pub const DISPID_AMBIENT_LOCALEID: DISPID = -705;
pub const DISPID_AMBIENT_MESSAGEREFLECT: DISPID = -706;
pub const DISPID_AMBIENT_SCALEUNITS: DISPID = -707;
pub const DISPID_AMBIENT_TEXTALIGN: DISPID = -708;
pub const DISPID_AMBIENT_USERMODE: DISPID = -709;
pub const DISPID_AMBIENT_UIDEAD: DISPID = -710;
pub const DISPID_AMBIENT_SHOWGRABHANDLES: DISPID = -711;
pub const DISPID_AMBIENT_SHOWHATCHING: DISPID = -712;
pub const DISPID_AMBIENT_DISPLAYASDEFAULT: DISPID = -713;
pub const DISPID_AMBIENT_SUPPORTSMNEMONICS: DISPID = -714;
pub const DISPID_AMBIENT_AUTOCLIP: DISPID = -715;
pub const DISPID_AMBIENT_APPEARANCE: DISPID = -716;
pub const DISPID_AMBIENT_CODEPAGE: DISPID = -725;
pub const DISPID_AMBIENT_PALETTE: DISPID = -726;
pub const DISPID_AMBIENT_CHARSET: DISPID = -727;
pub const DISPID_AMBIENT_TRANSFERPRIORITY: DISPID = -728;
pub const DISPID_AMBIENT_RIGHTTOLEFT: DISPID = -732;
pub const DISPID_AMBIENT_TOPTOBOTTOM: DISPID = -733;
pub const DISPID_Name: DISPID = -800;
pub const DISPID_Delete: DISPID = -801;
pub const DISPID_Object: DISPID = -802;
pub const DISPID_Parent: DISPID = -803;
pub const DISPID_FONT_NAME: DISPID = 0;
pub const DISPID_FONT_SIZE: DISPID = 2;
pub const DISPID_FONT_BOLD: DISPID = 3;
pub const DISPID_FONT_ITALIC: DISPID = 4;
pub const DISPID_FONT_UNDER: DISPID = 5;
pub const DISPID_FONT_STRIKE: DISPID = 6;
pub const DISPID_FONT_WEIGHT: DISPID = 7;
pub const DISPID_FONT_CHARSET: DISPID = 8;
pub const DISPID_FONT_CHANGED: DISPID = 9;
pub const DISPID_PICT_HANDLE: DISPID = 0;
pub const DISPID_PICT_HPAL: DISPID = 2;
pub const DISPID_PICT_TYPE: DISPID = 3;
pub const DISPID_PICT_WIDTH: DISPID = 4;
pub const DISPID_PICT_HEIGHT: DISPID = 5;
pub const DISPID_PICT_RENDER: DISPID = 6;

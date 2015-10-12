// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//138
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct INITCOMMONCONTROLSEX {
    pub dwSize: ::DWORD,
    pub dwICC: ::DWORD,
}
pub type LPINITCOMMONCONTROLSEX = *mut INITCOMMONCONTROLSEX;
pub const ICC_LISTVIEW_CLASSES: ::DWORD = 0x1;
pub const ICC_TREEVIEW_CLASSES: ::DWORD = 0x2;
pub const ICC_BAR_CLASSES: ::DWORD = 0x4;
pub const ICC_TAB_CLASSES: ::DWORD = 0x8;
pub const ICC_UPDOWN_CLASS: ::DWORD = 0x10;
pub const ICC_PROGRESS_CLASS: ::DWORD = 0x20;
pub const ICC_HOTKEY_CLASS: ::DWORD = 0x40;
pub const ICC_ANIMATE_CLASS: ::DWORD = 0x80;
pub const ICC_WIN95_CLASSES: ::DWORD = 0xFF;
pub const ICC_DATE_CLASSES: ::DWORD = 0x100;
pub const ICC_USEREX_CLASSES: ::DWORD = 0x200;
pub const ICC_COOL_CLASSES: ::DWORD = 0x400;
pub const ICC_INTERNET_CLASSES: ::DWORD = 0x800;
pub const ICC_PAGESCROLLER_CLASS: ::DWORD = 0x1000;
pub const ICC_NATIVEFNTCTL_CLASS: ::DWORD = 0x2000;
pub const ICC_STANDARD_CLASSES: ::DWORD = 0x4000;
pub const ICC_LINK_CLASS: ::DWORD = 0x8000;
pub const ODT_HEADER: ::UINT = 100;
pub const ODT_TAB: ::UINT = 101;
pub const ODT_LISTVIEW: ::UINT = 102;
pub const LVM_FIRST: ::UINT = 0x1000;
pub const TV_FIRST: ::UINT = 0x1100;
pub const HDM_FIRST: ::UINT = 0x1200;
pub const TCM_FIRST: ::UINT = 0x1300;
pub const PGM_FIRST: ::UINT = 0x1400;
pub const ECM_FIRST: ::UINT = 0x1500;
pub const BCM_FIRST: ::UINT = 0x1600;
pub const CBM_FIRST: ::UINT = 0x1700;
pub const CCM_FIRST: ::UINT = 0x2000;
pub const CCM_LAST: ::UINT = CCM_FIRST + 0x200;
pub const CCM_SETBKCOLOR: ::UINT = CCM_FIRST + 1;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct COLORSCHEME {
    pub dwSize: ::DWORD,
    pub clrBtnHighlight: ::COLORREF,
    pub clrBtnShadow: ::COLORREF,
}
pub type LPCOLORSCHEME = *mut COLORSCHEME;
pub const CCM_SETCOLORSCHEME: ::UINT = CCM_FIRST + 2;
pub const CCM_GETCOLORSCHEME: ::UINT = CCM_FIRST + 3;
pub const CCM_GETDROPTARGET: ::UINT = CCM_FIRST + 4;
pub const CCM_SETUNICODEFORMAT: ::UINT = CCM_FIRST + 5;
pub const CCM_GETUNICODEFORMAT: ::UINT = CCM_FIRST + 6;
pub const CCM_SETVERSION: ::UINT = CCM_FIRST + 7;
pub const CCM_GETVERSION: ::UINT = CCM_FIRST + 8;
pub const CCM_SETNOTIFYWINDOW: ::UINT = CCM_FIRST + 9;
pub const CCM_SETWINDOWTHEME: ::UINT = CCM_FIRST + 0xb;
pub const CCM_DPISCALE: ::UINT = CCM_FIRST + 0xc;
pub const INFOTIPSIZE: ::c_int = 1024;
pub const NM_OUTOFMEMORY: ::UINT = NM_FIRST-1;
pub const NM_CLICK: ::UINT = NM_FIRST-2;
pub const NM_DBLCLK: ::UINT = NM_FIRST-3;
pub const NM_RETURN: ::UINT = NM_FIRST-4;
pub const NM_RCLICK: ::UINT = NM_FIRST-5;
pub const NM_RDBLCLK: ::UINT = NM_FIRST-6;
pub const NM_SETFOCUS: ::UINT = NM_FIRST-7;
pub const NM_KILLFOCUS: ::UINT = NM_FIRST-8;
pub const NM_CUSTOMDRAW: ::UINT = NM_FIRST-12;
pub const NM_HOVER: ::UINT = NM_FIRST-13;
pub const NM_NCHITTEST: ::UINT = NM_FIRST-14;
pub const NM_KEYDOWN: ::UINT = NM_FIRST-15;
pub const NM_RELEASEDCAPTURE: ::UINT = NM_FIRST-16;
pub const NM_SETCURSOR: ::UINT = NM_FIRST-17;
pub const NM_CHAR: ::UINT = NM_FIRST-18;
pub const NM_TOOLTIPSCREATED: ::UINT = NM_FIRST-19;
pub const NM_LDOWN: ::UINT = NM_FIRST-20;
pub const NM_RDOWN: ::UINT = NM_FIRST-21;
pub const NM_THEMECHANGED: ::UINT = NM_FIRST-22;
pub const NM_FONTCHANGED: ::UINT = NM_FIRST-23;
pub const NM_CUSTOMTEXT: ::UINT = NM_FIRST-24;
pub const NM_TVSTATEIMAGECHANGING: ::UINT = NM_FIRST-24;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMTOOLTIPSCREATED {
    pub hdr: ::NMHDR,
    pub hwndToolTips: ::HWND,
}
pub type LPNMTOOLTIPSCREATED = *mut NMTOOLTIPSCREATED;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMMOUSE {
    pub hdr : ::NMHDR,
    pub dwItemSpec: ::DWORD_PTR,
    pub dwItemData: ::DWORD_PTR,
    pub pt: ::POINT,
    pub dwHitInfo: ::LPARAM,
}
pub type LPNMMOUSE = *mut NMMOUSE;
pub type NMCLICK = NMMOUSE;
pub type LPNMCLICK = LPNMMOUSE;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMOBJECTNOTIFY {
    pub hdr: ::NMHDR,
    pub iItem: ::c_int,
    pub piid: *const ::IID,
    pub pObject: *mut ::c_void,
    pub hResult: ::HRESULT,
    pub dwFlags: ::DWORD,
}
pub type LPNMOBJECTNOTIFY = *mut NMOBJECTNOTIFY;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMKEY {
    pub hdr: ::NMHDR,
    pub nVKey: ::UINT,
    pub uFlags: ::UINT,
}
pub type LPNMKEY = *mut NMKEY;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMCHAR {
    pub hdr: ::NMHDR,
    pub ch: ::UINT,
    pub dwItemPrev: ::DWORD,
    pub dwItemNext: ::DWORD,
}
pub type LPNMCHAR = *mut NMCHAR;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMCUSTOMTEXT {
    pub hdr: ::NMHDR,
    pub hDC: ::HDC,
    pub lpString: ::LPCWSTR,
    pub nCount: ::c_int,
    pub lpRect: ::LPRECT,
    pub uFormat: ::UINT,
    pub fLink: ::BOOL,
}
pub type LPNMCUSTOMTEXT = *mut NMCUSTOMTEXT;
pub const NM_FIRST: ::UINT = 0;
pub const NM_LAST: ::UINT = 0 - 99;
pub const LVN_FIRST: ::UINT = 0 - 100;
pub const LVN_LAST: ::UINT = 0 - 199;
pub const HDN_FIRST: ::UINT = 0 - 300;
pub const HDN_LAST: ::UINT = 0 - 399;
pub const TVN_FIRST: ::UINT = 0 - 400;
pub const TVN_LAST: ::UINT = 0 - 499;
pub const TTN_FIRST: ::UINT = 0 - 520;
pub const TTN_LAST: ::UINT = 0 - 549;
pub const TCN_FIRST: ::UINT = 0 - 550;
pub const TCN_LAST: ::UINT = 0 - 580;
// pub const CDN_FIRST: ::UINT = 0 - 601;
// pub const CDN_LAST: ::UINT = 0 - 699;
pub const TBN_FIRST: ::UINT = 0 - 700;
pub const TBN_LAST: ::UINT = 0 - 720;
pub const UDN_FIRST: ::UINT = 0 - 721;
pub const UDN_LAST: ::UINT = 0 - 729;
pub const DTN_FIRST: ::UINT = 0 - 740;
pub const DTN_LAST: ::UINT = 0 - 745;
pub const MCN_FIRST: ::UINT = 0 - 746;
pub const MCN_LAST: ::UINT = 0 - 752;
pub const DTN_FIRST2: ::UINT = 0 - 753;
pub const DTN_LAST2: ::UINT = 0 - 799;
pub const CBEN_FIRST: ::UINT = 0 - 800;
pub const CBEN_LAST: ::UINT = 0 - 830;
pub const RBN_FIRST: ::UINT = 0 - 831;
pub const RBN_LAST: ::UINT = 0 - 859;
pub const IPN_FIRST: ::UINT = 0 - 860;
pub const IPN_LAST: ::UINT = 0 - 879;
pub const SBN_FIRST: ::UINT = 0 - 880;
pub const SBN_LAST: ::UINT = 0 - 899;
pub const PGN_FIRST: ::UINT = 0 - 900;
pub const PGN_LAST: ::UINT = 0 - 950;
pub const WMN_FIRST: ::UINT = 0 - 1000;
pub const WMN_LAST: ::UINT = 0 - 1200;
pub const BCN_FIRST: ::UINT = 0 - 1250;
pub const BCN_LAST: ::UINT = 0 - 1350;
pub const TRBN_FIRST: ::UINT = 0 - 1501;
pub const TRBN_LAST: ::UINT = 0 - 1519;
pub const CDRF_DODEFAULT: ::LRESULT = 0x00000000;
pub const CDRF_NEWFONT: ::LRESULT = 0x00000002;
pub const CDRF_SKIPDEFAULT: ::LRESULT = 0x00000004;
pub const CDRF_DOERASE: ::LRESULT = 0x00000008;
pub const CDRF_SKIPPOSTPAINT: ::LRESULT = 0x00000100;
pub const CDRF_NOTIFYPOSTPAINT: ::LRESULT = 0x00000010;
pub const CDRF_NOTIFYITEMDRAW: ::LRESULT = 0x00000020;
pub const CDRF_NOTIFYSUBITEMDRAW: ::LRESULT = 0x00000020;
pub const CDRF_NOTIFYPOSTERASE: ::LRESULT = 0x00000040;
pub const CDDS_PREPAINT: ::DWORD = 0x00000001;
pub const CDDS_POSTPAINT: ::DWORD = 0x00000002;
pub const CDDS_PREERASE: ::DWORD = 0x00000003;
pub const CDDS_POSTERASE: ::DWORD = 0x00000004;
pub const CDDS_ITEM: ::DWORD = 0x00010000;
pub const CDDS_ITEMPREPAINT: ::DWORD = CDDS_ITEM | CDDS_PREPAINT;
pub const CDDS_ITEMPOSTPAINT: ::DWORD = CDDS_ITEM | CDDS_POSTPAINT;
pub const CDDS_ITEMPREERASE: ::DWORD = CDDS_ITEM | CDDS_PREERASE;
pub const CDDS_ITEMPOSTERASE: ::DWORD = CDDS_ITEM | CDDS_POSTERASE;
pub const CDDS_SUBITEM: ::DWORD = 0x00020000;
pub const CDIS_SELECTED: ::UINT = 0x0001;
pub const CDIS_GRAYED: ::UINT = 0x0002;
pub const CDIS_DISABLED: ::UINT = 0x0004;
pub const CDIS_CHECKED: ::UINT = 0x0008;
pub const CDIS_FOCUS: ::UINT = 0x0010;
pub const CDIS_DEFAULT: ::UINT = 0x0020;
pub const CDIS_HOT: ::UINT = 0x0040;
pub const CDIS_MARKED: ::UINT = 0x0080;
pub const CDIS_INDETERMINATE: ::UINT = 0x0100;
pub const CDIS_SHOWKEYBOARDCUES: ::UINT = 0x0200;
pub const CDIS_NEARHOT: ::UINT = 0x0400;
pub const CDIS_OTHERSIDEHOT: ::UINT = 0x0800;
pub const CDIS_DROPHILITED: ::UINT = 0x1000;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMCUSTOMDRAW {
    pub hdr: ::NMHDR,
    pub dwDrawStage: ::DWORD,
    pub hdc: ::HDC,
    pub rc: ::RECT,
    pub dwItemSpec: ::DWORD_PTR,
    pub uItemState: ::UINT,
    pub lItemlParam: ::LPARAM,
}
pub type LPNMCUSTOMDRAW = *mut NMCUSTOMDRAW;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMTTCUSTOMDRAW {
    pub nmcd: NMCUSTOMDRAW,
    pub uDrawFlags: ::UINT,
}
pub type LPNMTTCUSTOMDRAW = *mut NMTTCUSTOMDRAW;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMCUSTOMSPLITRECTINFO {
    pub hdr: ::NMHDR,
    pub rcClient: ::RECT,
    pub rcButton: ::RECT,
    pub rcSplit: ::RECT,
}
pub type LPNMCUSTOMSPLITRECTINFO = *mut NMCUSTOMSPLITRECTINFO;
pub const NM_GETCUSTOMSPLITRECT: ::UINT = BCN_FIRST + 0x0003;
pub const CLR_NONE: ::DWORD = 0xFFFFFFFF;
pub const CLR_DEFAULT: ::DWORD = 0xFF000000;
#[repr(C)] #[allow(missing_copy_implementations)]
pub struct IMAGELIST {
    unused: ::c_void,
}
pub type HIMAGELIST = *mut IMAGELIST;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IMAGELISTDRAWPARAMS {
    pub cbSize: ::DWORD,
    pub himl: HIMAGELIST,
    pub i: ::c_int,
    pub hdcDst: ::HDC,
    pub x: ::c_int,
    pub y: ::c_int,
    pub cx: ::c_int,
    pub cy: ::c_int,
    pub xBitmap: ::c_int,
    pub yBitmap: ::c_int,
    pub rgbBk: ::COLORREF,
    pub rgbFg: ::COLORREF,
    pub fStyle: ::UINT,
    pub dwRop: ::DWORD,
    pub fState: ::DWORD,
    pub Frame: ::DWORD,
    pub crEffect: ::COLORREF,
}
pub type LPIMAGELISTDRAWPARAMS = *mut IMAGELISTDRAWPARAMS;
pub const ILC_MASK: ::UINT = 0x00000001;
pub const ILC_COLOR: ::UINT = 0x00000000;
pub const ILC_COLORDDB: ::UINT = 0x000000FE;
pub const ILC_COLOR4: ::UINT = 0x00000004;
pub const ILC_COLOR8: ::UINT = 0x00000008;
pub const ILC_COLOR16: ::UINT = 0x00000010;
pub const ILC_COLOR24: ::UINT = 0x00000018;
pub const ILC_COLOR32: ::UINT = 0x00000020;
pub const ILC_PALETTE: ::UINT = 0x00000800;
pub const ILC_MIRROR: ::UINT = 0x00002000;
pub const ILC_PERITEMMIRROR: ::UINT = 0x00008000;
pub const ILC_ORIGINALSIZE: ::UINT = 0x00010000;
pub const ILC_HIGHQUALITYSCALE: ::UINT = 0x00020000;
pub const ILD_NORMAL: ::UINT = 0x00000000;
pub const ILD_TRANSPARENT: ::UINT = 0x00000001;
pub const ILD_MASK: ::UINT = 0x00000010;
pub const ILD_IMAGE: ::UINT = 0x00000020;
pub const ILD_ROP: ::UINT = 0x00000040;
pub const ILD_BLEND25: ::UINT = 0x00000002;
pub const ILD_BLEND50: ::UINT = 0x00000004;
pub const ILD_OVERLAYMASK: ::UINT = 0x00000F00;
#[inline] #[allow(dead_code)]
pub fn INDEXTOOVERLAYMASK(i: ::UINT) -> ::UINT { i << 8 }
pub const ILD_PRESERVEALPHA: ::UINT = 0x00001000;
pub const ILD_SCALE: ::UINT = 0x00002000;
pub const ILD_DPISCALE: ::UINT = 0x00004000;
pub const ILD_ASYNC: ::UINT = 0x00008000;
pub const ILD_SELECTED: ::UINT = ILD_BLEND50;
pub const ILD_FOCUS: ::UINT = ILD_BLEND25;
pub const ILD_BLEND: ::UINT = ILD_BLEND50;
pub const CLR_HILIGHT: ::DWORD = CLR_DEFAULT;
pub const ILS_NORMAL: ::DWORD = 0x00000000;
pub const ILS_GLOW: ::DWORD = 0x00000001;
pub const ILS_SHADOW: ::DWORD = 0x00000002;
pub const ILS_SATURATE: ::DWORD = 0x00000004;
pub const ILS_ALPHA: ::DWORD = 0x00000008;
pub const HBITMAP_CALLBACK: ::HBITMAP = (0-1) as ::HBITMAP;
pub const ILCF_MOVE: ::UINT = 0x00000000;
pub const ILCF_SWAP: ::UINT = 0x00000001;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IMAGEINFO {
    pub hbmImage: ::HBITMAP,
    pub hbmMask: ::HBITMAP,
    pub Unused1: ::c_int,
    pub Unused2: ::c_int,
    pub rcImage: ::RECT,
}
pub type LPIMAGEINFO = *mut IMAGEINFO;
pub const HDS_HORZ: ::DWORD = 0x0000;
pub const HDS_BUTTONS: ::DWORD = 0x0002;
pub const HDS_HOTTRACK: ::DWORD = 0x0004;
pub const HDS_HIDDEN: ::DWORD = 0x0008;
pub const HDS_DRAGDROP: ::DWORD = 0x0040;
pub const HDS_FULLDRAG: ::DWORD = 0x0080;
pub const HDS_FILTERBAR: ::DWORD = 0x0100;
pub const HDS_FLAT: ::DWORD = 0x0200;
pub const HDS_CHECKBOXES: ::DWORD = 0x0400;
pub const HDS_NOSIZING: ::DWORD = 0x0800;
pub const HDS_OVERFLOW: ::DWORD = 0x1000;
pub const HDFT_ISSTRING: ::UINT = 0x0000;
pub const HDFT_ISNUMBER: ::UINT = 0x0001;
pub const HDFT_ISDATE: ::UINT = 0x0002;
pub const HDFT_HASNOVALUE: ::UINT = 0x8000;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HD_TEXTFILTERA {
    pub pszText: ::LPSTR,
    pub cchTextMax: ::INT,
}
pub type LPHD_TEXTFILTERA = *mut HD_TEXTFILTERA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HD_TEXTFILTERW {
    pub pszText: ::LPWSTR,
    pub cchTextMax: ::INT,
}
pub type LPHD_TEXTFILTERW = *mut HD_TEXTFILTERW;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HDITEMA {
    pub mask: ::UINT,
    pub cxy: ::c_int,
    pub pszText: ::LPSTR,
    pub hbm: ::HBITMAP,
    pub cchTextMax: ::c_int,
    pub fmt: ::c_int,
    pub lParam: ::LPARAM,
    pub iImage: ::c_int,
    pub iOrder: ::c_int,
    pub _type: ::UINT,
    pub pvFilter: *mut ::c_void,
    pub state: ::UINT,
}
pub type LPHDITEMA = *mut HDITEMA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HDITEMW {
    pub mask: ::UINT,
    pub cxy: ::c_int,
    pub pszText: ::LPWSTR,
    pub hbm: ::HBITMAP,
    pub cchTextMax: ::c_int,
    pub fmt: ::c_int,
    pub lParam: ::LPARAM,
    pub iImage: ::c_int,
    pub iOrder: ::c_int,
    pub _type: ::UINT,
    pub pvFilter: *mut ::c_void,
    pub state: ::UINT,
}
pub type LPHDITEMW = *mut HDITEMW;
pub const HDI_WIDTH: ::UINT = 0x0001;
pub const HDI_HEIGHT: ::UINT = HDI_WIDTH;
pub const HDI_TEXT: ::UINT = 0x0002;
pub const HDI_FORMAT: ::UINT = 0x0004;
pub const HDI_LPARAM: ::UINT = 0x0008;
pub const HDI_BITMAP: ::UINT = 0x0010;
pub const HDI_IMAGE: ::UINT = 0x0020;
pub const HDI_DI_SETITEM: ::UINT = 0x0040;
pub const HDI_ORDER: ::UINT = 0x0080;
pub const HDI_FILTER: ::UINT = 0x0100;
pub const HDI_STATE: ::UINT = 0x0200;
pub const HDF_LEFT: ::c_int = 0x0000;
pub const HDF_RIGHT: ::c_int = 0x0001;
pub const HDF_CENTER: ::c_int = 0x0002;
pub const HDF_JUSTIFYMASK: ::c_int = 0x0003;
pub const HDF_RTLREADING: ::c_int = 0x0004;
pub const HDF_BITMAP: ::c_int = 0x2000;
pub const HDF_STRING: ::c_int = 0x4000;
pub const HDF_OWNERDRAW: ::c_int = 0x8000;
pub const HDF_IMAGE: ::c_int = 0x0800;
pub const HDF_BITMAP_ON_RIGHT: ::c_int = 0x1000;
pub const HDF_SORTUP: ::c_int = 0x0400;
pub const HDF_SORTDOWN: ::c_int = 0x0200;
pub const HDF_CHECKBOX: ::c_int = 0x0040;
pub const HDF_CHECKED: ::c_int = 0x0080;
pub const HDF_FIXEDWIDTH: ::c_int = 0x0100;
pub const HDF_SPLITBUTTON: ::c_int = 0x1000000;
pub const HDIS_FOCUSED: ::UINT = 0x00000001;
pub const HDM_GETITEMCOUNT: ::UINT = HDM_FIRST + 0;
pub const HDM_INSERTITEMA: ::UINT = HDM_FIRST + 1;
pub const HDM_INSERTITEMW: ::UINT = HDM_FIRST + 10;
pub const HDM_DELETEITEM: ::UINT = HDM_FIRST + 2;
pub const HDM_GETITEMA: ::UINT = HDM_FIRST + 3;
pub const HDM_GETITEMW: ::UINT = HDM_FIRST + 11;
pub const HDM_SETITEMA: ::UINT = HDM_FIRST + 4;
pub const HDM_SETITEMW: ::UINT = HDM_FIRST + 12;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HDLAYOUT {
    pub prc: *mut ::RECT,
    pub pwpos: *mut ::WINDOWPOS,
}
pub type LPHDLAYOUT = *mut HDLAYOUT;
pub const HDM_LAYOUT: ::UINT = HDM_FIRST + 5;
pub const HHT_NOWHERE: ::UINT = 0x0001;
pub const HHT_ONHEADER: ::UINT = 0x0002;
pub const HHT_ONDIVIDER: ::UINT = 0x0004;
pub const HHT_ONDIVOPEN: ::UINT = 0x0008;
pub const HHT_ONFILTER: ::UINT = 0x0010;
pub const HHT_ONFILTERBUTTON: ::UINT = 0x0020;
pub const HHT_ABOVE: ::UINT = 0x0100;
pub const HHT_BELOW: ::UINT = 0x0200;
pub const HHT_TORIGHT: ::UINT = 0x0400;
pub const HHT_TOLEFT: ::UINT = 0x0800;
pub const HHT_ONITEMSTATEICON: ::UINT = 0x1000;
pub const HHT_ONDROPDOWN: ::UINT = 0x2000;
pub const HHT_ONOVERFLOW: ::UINT = 0x4000;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HDHITTESTINFO {
    pub pt: ::POINT,
    pub flags: ::UINT,
    pub iItem: ::c_int,
}
pub type LPHDHITTESTINFO = *mut HDHITTESTINFO;
pub const HDSIL_NORMAL: ::WPARAM = 0;
pub const HDSIL_STATE: ::WPARAM = 1;
pub const HDM_HITTEST: ::UINT = HDM_FIRST + 6;
pub const HDM_GETITEMRECT: ::UINT = HDM_FIRST + 7;
pub const HDM_SETIMAGELIST: ::UINT = HDM_FIRST + 8;
pub const HDM_GETIMAGELIST: ::UINT = HDM_FIRST + 9;
pub const HDM_ORDERTOINDEX: ::UINT = HDM_FIRST + 15;
pub const HDM_CREATEDRAGIMAGE: ::UINT = HDM_FIRST + 16;
pub const HDM_GETORDERARRAY: ::UINT = HDM_FIRST + 17;
pub const HDM_SETORDERARRAY: ::UINT = HDM_FIRST + 18;
pub const HDM_SETHOTDIVIDER: ::UINT = HDM_FIRST + 19;
pub const HDM_SETBITMAPMARGIN: ::UINT = HDM_FIRST + 20;
pub const HDM_GETBITMAPMARGIN: ::UINT = HDM_FIRST + 21;
pub const HDM_SETFILTERCHANGETIMEOUT: ::UINT = HDM_FIRST + 22;
pub const HDM_EDITFILTER: ::UINT = HDM_FIRST + 23;
pub const HDM_CLEARFILTER: ::UINT = HDM_FIRST + 24;
pub const HDM_GETITEMDROPDOWNRECT: ::UINT = HDM_FIRST + 25;
pub const HDM_GETOVERFLOWRECT: ::UINT = HDM_FIRST + 26;
pub const HDM_GETFOCUSEDITEM: ::UINT = HDM_FIRST + 27;
pub const HDM_SETFOCUSEDITEM: ::UINT = HDM_FIRST + 28;
pub const HDN_ITEMCHANGINGA: ::UINT = HDN_FIRST-0;
pub const HDN_ITEMCHANGINGW: ::UINT = HDN_FIRST-20;
pub const HDN_ITEMCHANGEDA: ::UINT = HDN_FIRST-1;
pub const HDN_ITEMCHANGEDW: ::UINT = HDN_FIRST-21;
pub const HDN_ITEMCLICKA: ::UINT = HDN_FIRST-2;
pub const HDN_ITEMCLICKW: ::UINT = HDN_FIRST-22;
pub const HDN_ITEMDBLCLICKA: ::UINT = HDN_FIRST-3;
pub const HDN_ITEMDBLCLICKW: ::UINT = HDN_FIRST-23;
pub const HDN_DIVIDERDBLCLICKA: ::UINT = HDN_FIRST-5;
pub const HDN_DIVIDERDBLCLICKW: ::UINT = HDN_FIRST-25;
pub const HDN_BEGINTRACKA: ::UINT = HDN_FIRST-6;
pub const HDN_BEGINTRACKW: ::UINT = HDN_FIRST-26;
pub const HDN_ENDTRACKA: ::UINT = HDN_FIRST-7;
pub const HDN_ENDTRACKW: ::UINT = HDN_FIRST-27;
pub const HDN_TRACKA: ::UINT = HDN_FIRST-8;
pub const HDN_TRACKW: ::UINT = HDN_FIRST-28;
pub const HDN_GETDISPINFOA: ::UINT = HDN_FIRST-9;
pub const HDN_GETDISPINFOW: ::UINT = HDN_FIRST-29;
pub const HDN_BEGINDRAG: ::UINT = HDN_FIRST-10;
pub const HDN_ENDDRAG: ::UINT = HDN_FIRST-11;
pub const HDN_FILTERCHANGE: ::UINT = HDN_FIRST-12;
pub const HDN_FILTERBTNCLICK: ::UINT = HDN_FIRST-13;
pub const HDN_BEGINFILTEREDIT: ::UINT = HDN_FIRST-14;
pub const HDN_ENDFILTEREDIT: ::UINT = HDN_FIRST-15;
pub const HDN_ITEMSTATEICONCLICK: ::UINT = HDN_FIRST-16;
pub const HDN_ITEMKEYDOWN: ::UINT = HDN_FIRST-17;
pub const HDN_DROPDOWN: ::UINT = HDN_FIRST-18;
pub const HDN_OVERFLOWCLICK: ::UINT = HDN_FIRST-19;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMHEADERA {
    pub hdr: ::NMHDR,
    pub iItem: ::c_int,
    pub iButton: ::c_int,
    pub pitem: *mut HDITEMA,
}
pub type LPNMHEADERA = *mut NMHEADERA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMHEADERW {
    pub hdr: ::NMHDR,
    pub iItem: ::c_int,
    pub iButton: ::c_int,
    pub pitem: *mut HDITEMW,
}
pub type LPNMHEADERW = *mut NMHEADERW;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMHDDISPINFOW {
    pub hdr: ::NMHDR,
    pub iItem: ::c_int,
    pub mask: ::UINT,
    pub pszText: ::LPWSTR,
    pub cchTextMax: ::c_int,
    pub iImage: ::c_int,
    pub lParam: ::LPARAM,
}
pub type LPNMHDDISPINFOW = *mut NMHDDISPINFOW;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMHDDISPINFOA {
    pub hdr: ::NMHDR,
    pub iItem: ::c_int,
    pub mask: ::UINT,
    pub pszText: ::LPSTR,
    pub cchTextMax: ::c_int,
    pub iImage: ::c_int,
    pub lParam: ::LPARAM,
}
pub type LPNMHDDISPINFOA = *mut NMHDDISPINFOA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMHDFILTERBTNCLICK {
    pub hdr: ::NMHDR,
    pub iItem: ::INT,
    pub rc: ::RECT,
}
pub type LPNMHDFILTERBTNCLICK = *mut NMHDFILTERBTNCLICK;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct TBBUTTON {
    pub iBitmap: ::c_int,
    pub idCommand: ::c_int,
    pub fsState: ::BYTE,
    pub fsStyle: ::BYTE,
    #[cfg(target_arch="x86_64")]
    pub bReserved: [::BYTE; 6],
    #[cfg(target_arch="x86")]
    pub bReserved: [::BYTE; 2],
    pub dwData: ::DWORD_PTR,
    pub iString: ::INT_PTR,
}
pub type PTBBUTTON = *mut TBBUTTON;
pub type LPTBBUTTON = *mut TBBUTTON;
pub type LPCTBBUTTON = *const TBBUTTON;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct COLORMAP {
    pub from: ::COLORREF,
    pub to: ::COLORREF,
}
pub type LPCOLORMAP = *mut COLORMAP;
pub const CMB_MASKED: ::UINT = 0x02;
pub const TBSTATE_CHECKED: ::BYTE = 0x01;
pub const TBSTATE_PRESSED: ::BYTE = 0x02;
pub const TBSTATE_ENABLED: ::BYTE = 0x04;
pub const TBSTATE_HIDDEN: ::BYTE = 0x08;
pub const TBSTATE_INDETERMINATE: ::BYTE = 0x10;
pub const TBSTATE_WRAP: ::BYTE = 0x20;
pub const TBSTATE_ELLIPSES: ::BYTE = 0x40;
pub const TBSTATE_MARKED: ::BYTE = 0x80;
pub const TBSTYLE_BUTTON: ::DWORD = 0x0000;
pub const TBSTYLE_SEP: ::DWORD = 0x0001;
pub const TBSTYLE_CHECK: ::DWORD = 0x0002;
pub const TBSTYLE_GROUP: ::DWORD = 0x0004;
pub const TBSTYLE_CHECKGROUP: ::DWORD = TBSTYLE_GROUP | TBSTYLE_CHECK;
pub const TBSTYLE_DROPDOWN: ::DWORD = 0x0008;
pub const TBSTYLE_AUTOSIZE: ::DWORD = 0x0010;
pub const TBSTYLE_NOPREFIX: ::DWORD = 0x0020;
pub const TBSTYLE_TOOLTIPS: ::DWORD = 0x0100;
pub const TBSTYLE_WRAPABLE: ::DWORD = 0x0200;
pub const TBSTYLE_ALTDRAG: ::DWORD = 0x0400;
pub const TBSTYLE_FLAT: ::DWORD = 0x0800;
pub const TBSTYLE_LIST: ::DWORD = 0x1000;
pub const TBSTYLE_CUSTOMERASE: ::DWORD = 0x2000;
pub const TBSTYLE_REGISTERDROP: ::DWORD = 0x4000;
pub const TBSTYLE_TRANSPARENT: ::DWORD = 0x8000;
pub const TBSTYLE_EX_DRAWDDARROWS: ::DWORD = 0x00000001;
pub const BTNS_BUTTON: ::DWORD = TBSTYLE_BUTTON;
pub const BTNS_SEP: ::DWORD = TBSTYLE_SEP;
pub const BTNS_CHECK: ::DWORD = TBSTYLE_CHECK;
pub const BTNS_GROUP: ::DWORD = TBSTYLE_GROUP;
pub const BTNS_CHECKGROUP: ::DWORD = TBSTYLE_CHECKGROUP;
pub const BTNS_DROPDOWN: ::DWORD = TBSTYLE_DROPDOWN;
pub const BTNS_AUTOSIZE: ::DWORD = TBSTYLE_AUTOSIZE;
pub const BTNS_NOPREFIX: ::DWORD = TBSTYLE_NOPREFIX;
pub const BTNS_SHOWTEXT: ::DWORD = 0x0040;
pub const BTNS_WHOLEDROPDOWN: ::DWORD = 0x0080;
pub const TBSTYLE_EX_MIXEDBUTTONS: ::DWORD = 0x00000008;
pub const TBSTYLE_EX_HIDECLIPPEDBUTTONS: ::DWORD = 0x00000010;
pub const TBSTYLE_EX_MULTICOLUMN: ::DWORD = 0x00000002;
pub const TBSTYLE_EX_VERTICAL: ::DWORD = 0x00000004;
pub const TBSTYLE_EX_DOUBLEBUFFER: ::DWORD = 0x00000080;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMTBCUSTOMDRAW {
    pub nmcd: NMCUSTOMDRAW,
    pub hbrMonoDither: ::HBRUSH,
    pub hbrLines: ::HBRUSH,
    pub hpenLines: ::HPEN,
    pub clrText: ::COLORREF,
    pub clrMark: ::COLORREF,
    pub clrTextHighlight: ::COLORREF,
    pub clrBtnFace: ::COLORREF,
    pub clrBtnHighlight: ::COLORREF,
    pub clrHighlightHotTrack: ::COLORREF,
    pub rcText: ::RECT,
    pub nStringBkMode: ::c_int,
    pub nHLStringBkMode: ::c_int,
    pub iListGap: ::c_int,
}
pub type LPNMTBCUSTOMDRAW = *mut NMTBCUSTOMDRAW;
pub const TBCDRF_NOEDGES: ::LRESULT = 0x00010000;
pub const TBCDRF_HILITEHOTTRACK: ::LRESULT = 0x00020000;
pub const TBCDRF_NOOFFSET: ::LRESULT = 0x00040000;
pub const TBCDRF_NOMARK: ::LRESULT = 0x00080000;
pub const TBCDRF_NOETCHEDEFFECT: ::LRESULT = 0x00100000;
pub const TBCDRF_BLENDICON: ::LRESULT = 0x00200000;
pub const TBCDRF_NOBACKGROUND: ::LRESULT = 0x00400000;
pub const TBCDRF_USECDCOLORS: ::LRESULT = 0x00800000;
pub const TB_ENABLEBUTTON: ::UINT = ::WM_USER + 1;
pub const TB_CHECKBUTTON: ::UINT = ::WM_USER + 2;
pub const TB_PRESSBUTTON: ::UINT = ::WM_USER + 3;
pub const TB_HIDEBUTTON: ::UINT = ::WM_USER + 4;
pub const TB_INDETERMINATE: ::UINT = ::WM_USER + 5;
pub const TB_MARKBUTTON: ::UINT = ::WM_USER + 6;
pub const TB_ISBUTTONENABLED: ::UINT = ::WM_USER + 9;
pub const TB_ISBUTTONCHECKED: ::UINT = ::WM_USER + 10;
pub const TB_ISBUTTONPRESSED: ::UINT = ::WM_USER + 11;
pub const TB_ISBUTTONHIDDEN: ::UINT = ::WM_USER + 12;
pub const TB_ISBUTTONINDETERMINATE : ::UINT = ::WM_USER + 13;
pub const TB_ISBUTTONHIGHLIGHTED: ::UINT = ::WM_USER + 14;
pub const TB_SETSTATE: ::UINT = ::WM_USER + 17;
pub const TB_GETSTATE: ::UINT = ::WM_USER + 18;
pub const TB_ADDBITMAP: ::UINT = ::WM_USER + 19;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct TBADDBITMAP {
    pub hInst: ::HINSTANCE,
    pub nID: ::UINT_PTR,
}
pub type LPTBADDBITMAP = *mut TBADDBITMAP;
pub const HINST_COMMCTRL: ::HINSTANCE = (0 - 1) as ::HINSTANCE;
pub const IDB_STD_SMALL_COLOR: ::WPARAM = 0;
pub const IDB_STD_LARGE_COLOR: ::WPARAM = 1;
pub const IDB_VIEW_SMALL_COLOR: ::WPARAM = 4;
pub const IDB_VIEW_LARGE_COLOR: ::WPARAM = 5;
pub const IDB_HIST_SMALL_COLOR: ::WPARAM = 8;
pub const IDB_HIST_LARGE_COLOR: ::WPARAM = 9;
pub const IDB_HIST_NORMAL: ::WPARAM = 12;
pub const IDB_HIST_HOT: ::WPARAM = 13;
pub const IDB_HIST_DISABLED: ::WPARAM = 14;
pub const IDB_HIST_PRESSED: ::WPARAM = 15;
pub const STD_CUT: ::c_int = 0;
pub const STD_COPY: ::c_int = 1;
pub const STD_PASTE: ::c_int = 2;
pub const STD_UNDO: ::c_int = 3;
pub const STD_REDOW: ::c_int = 4;
pub const STD_DELETE: ::c_int = 5;
pub const STD_FILENEW: ::c_int = 6;
pub const STD_FILEOPEN: ::c_int = 7;
pub const STD_FILESAVE: ::c_int = 8;
pub const STD_PRINTPRE: ::c_int = 9;
pub const STD_PROPERTIES: ::c_int = 10;
pub const STD_HELP: ::c_int = 11;
pub const STD_FIND: ::c_int = 12;
pub const STD_REPLACE: ::c_int = 13;
pub const STD_PRINT: ::c_int = 14;
pub const VIEW_LARGEICONS: ::c_int = 0;
pub const VIEW_SMALLICONS: ::c_int = 1;
pub const VIEW_LIST: ::c_int = 2;
pub const VIEW_DETAILS: ::c_int = 3;
pub const VIEW_SORTNAME: ::c_int = 4;
pub const VIEW_SORTSIZE: ::c_int = 5;
pub const VIEW_SORTDATE: ::c_int = 6;
pub const VIEW_SORTTYPE: ::c_int = 7;
pub const VIEW_PARENTFOLDER: ::c_int = 8;
pub const VIEW_NETCONNECT: ::c_int = 9;
pub const VIEW_NETDISCONNECT: ::c_int = 10;
pub const VIEW_NEWFOLDER: ::c_int = 11;
pub const VIEW_VIEWMENU: ::c_int = 12;
pub const HIST_BACK: ::c_int = 0;
pub const HIST_FORWARD: ::c_int = 1;
pub const HIST_FAVORITES: ::c_int = 2;
pub const HIST_ADDTOFAVORITES: ::c_int = 3;
pub const HIST_VIEWTREE: ::c_int = 4;
pub const TB_ADDBUTTONSA: ::UINT = ::WM_USER + 20;
pub const TB_INSERTBUTTONA: ::UINT = ::WM_USER + 21;
pub const TB_DELETEBUTTON: ::UINT = ::WM_USER + 22;
pub const TB_GETBUTTON: ::UINT = ::WM_USER + 23;
pub const TB_BUTTONCOUNT: ::UINT = ::WM_USER + 24;
pub const TB_COMMANDTOINDEX: ::UINT = ::WM_USER + 25;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct TBSAVEPARAMSA {
    pub hkr: ::HKEY,
    pub pszSubKey: ::LPCSTR,
    pub pszValueName: ::LPCSTR,
}
pub type LPTBSAVEPARAMSA = *mut TBSAVEPARAMSA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct TBSAVEPARAMSW {
    pub hkr: ::HKEY,
    pub pszSubKey: ::LPCWSTR,
    pub pszValueName: ::LPCWSTR,
}
pub type LPTBSAVEPARAMSW = *mut TBSAVEPARAMSW;
pub const TB_SAVERESTOREA: ::UINT = ::WM_USER + 26;
pub const TB_SAVERESTOREW: ::UINT = ::WM_USER + 76;
pub const TB_CUSTOMIZE: ::UINT = ::WM_USER + 27;
pub const TB_ADDSTRINGA: ::UINT = ::WM_USER + 28;
pub const TB_ADDSTRINGW: ::UINT = ::WM_USER + 77;
pub const TB_GETITEMRECT: ::UINT = ::WM_USER + 29;
pub const TB_BUTTONSTRUCTSIZE: ::UINT = ::WM_USER + 30;
pub const TB_SETBUTTONSIZE: ::UINT = ::WM_USER + 31;
pub const TB_SETBITMAPSIZE: ::UINT = ::WM_USER + 32;
pub const TB_AUTOSIZE: ::UINT = ::WM_USER + 33;
pub const TB_GETTOOLTIPS: ::UINT = ::WM_USER + 35;
pub const TB_SETTOOLTIPS: ::UINT = ::WM_USER + 36;
pub const TB_SETPARENT: ::UINT = ::WM_USER + 37;
pub const TB_SETROWS: ::UINT = ::WM_USER + 39;
pub const TB_GETROWS: ::UINT = ::WM_USER + 40;
pub const TB_SETCMDID: ::UINT = ::WM_USER + 42;
pub const TB_CHANGEBITMAP: ::UINT = ::WM_USER + 43;
pub const TB_GETBITMAP: ::UINT = ::WM_USER + 44;
pub const TB_GETBUTTONTEXTA: ::UINT = ::WM_USER + 45;
pub const TB_GETBUTTONTEXTW: ::UINT = ::WM_USER + 75;
pub const TB_REPLACEBITMAP: ::UINT = ::WM_USER + 46;
pub const TB_SETINDENT: ::UINT = ::WM_USER + 47;
pub const TB_SETIMAGELIST: ::UINT = ::WM_USER + 48;
pub const TB_GETIMAGELIST: ::UINT = ::WM_USER + 49;
pub const TB_LOADIMAGES: ::UINT = ::WM_USER + 50;
pub const TB_GETRECT: ::UINT = ::WM_USER + 51;
pub const TB_SETHOTIMAGELIST: ::UINT = ::WM_USER + 52;
pub const TB_GETHOTIMAGELIST: ::UINT = ::WM_USER + 53;
pub const TB_SETDISABLEDIMAGELIST: ::UINT = ::WM_USER + 54;
pub const TB_GETDISABLEDIMAGELIST: ::UINT = ::WM_USER + 55;
pub const TB_SETSTYLE: ::UINT = ::WM_USER + 56;
pub const TB_GETSTYLE: ::UINT = ::WM_USER + 57;
pub const TB_GETBUTTONSIZE: ::UINT = ::WM_USER + 58;
pub const TB_SETBUTTONWIDTH: ::UINT = ::WM_USER + 59;
pub const TB_SETMAXTEXTROWS: ::UINT = ::WM_USER + 60;
pub const TB_GETTEXTROWS: ::UINT = ::WM_USER + 61;
pub const TB_GETOBJECT: ::UINT = ::WM_USER + 62;
pub const TB_GETHOTITEM: ::UINT = ::WM_USER + 71;
pub const TB_SETHOTITEM: ::UINT = ::WM_USER + 72;
pub const TB_SETANCHORHIGHLIGHT: ::UINT = ::WM_USER + 73;
pub const TB_GETANCHORHIGHLIGHT: ::UINT = ::WM_USER + 74;
pub const TB_MAPACCELERATORA: ::UINT = ::WM_USER + 78;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct TBINSERTMARK {
    pub iButton: ::c_int,
    pub dwFlags: ::DWORD,
}
pub type LPTBINSERTMARK = *mut TBINSERTMARK;
pub const TBIMHT_AFTER: ::DWORD = 0x00000001;
pub const TBIMHT_BACKGROUND: ::DWORD = 0x00000002;
pub const TB_GETINSERTMARK: ::UINT = ::WM_USER + 79;
pub const TB_SETINSERTMARK: ::UINT = ::WM_USER + 80;
pub const TB_INSERTMARKHITTEST: ::UINT = ::WM_USER + 81;
pub const TB_MOVEBUTTON: ::UINT = ::WM_USER + 82;
pub const TB_GETMAXSIZE: ::UINT = ::WM_USER + 83;
pub const TB_SETEXTENDEDSTYLE: ::UINT = ::WM_USER + 84;
pub const TB_GETEXTENDEDSTYLE: ::UINT = ::WM_USER + 85;
pub const TB_GETPADDING: ::UINT = ::WM_USER + 86;
pub const TB_SETPADDING: ::UINT = ::WM_USER + 87;
pub const TB_SETINSERTMARKCOLOR: ::UINT = ::WM_USER + 88;
pub const TB_GETINSERTMARKCOLOR: ::UINT = ::WM_USER + 89;
pub const TB_SETCOLORSCHEME: ::UINT = CCM_SETCOLORSCHEME;
pub const TB_GETCOLORSCHEME: ::UINT = CCM_GETCOLORSCHEME;
pub const TB_SETUNICODEFORMAT: ::UINT = CCM_SETUNICODEFORMAT;
pub const TB_GETUNICODEFORMAT: ::UINT = CCM_GETUNICODEFORMAT;
pub const TB_MAPACCELERATORW: ::UINT = ::WM_USER + 90;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct TBREPLACEBITMAP {
    pub hInstOld: ::HINSTANCE,
    pub nIDOld: ::UINT_PTR,
    pub hInstNew: ::HINSTANCE,
    pub nIDNew: ::UINT_PTR,
    pub nButtons: ::c_int,
}
pub type LPTBREPLACEBITMAP = *mut TBREPLACEBITMAP;
pub const TBBF_LARGE: ::DWORD = 0x0001;
pub const TB_GETBITMAPFLAGS: ::UINT = ::WM_USER + 41;
pub const TBIF_IMAGE: ::DWORD = 0x00000001;
pub const TBIF_TEXT: ::DWORD = 0x00000002;
pub const TBIF_STATE: ::DWORD = 0x00000004;
pub const TBIF_STYLE: ::DWORD = 0x00000008;
pub const TBIF_LPARAM: ::DWORD = 0x00000010;
pub const TBIF_COMMAND: ::DWORD = 0x00000020;
pub const TBIF_SIZE: ::DWORD = 0x00000040;
pub const TBIF_BYINDEX: ::DWORD = 0x80000000;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct TBBUTTONINFOA {
    pub cbSize: ::UINT,
    pub dwMask: ::DWORD,
    pub idCommand: ::c_int,
    pub iImage: ::c_int,
    pub fsState: ::BYTE,
    pub fsStyle: ::BYTE,
    pub cx: ::WORD,
    pub lParam: ::DWORD_PTR,
    pub pszText: ::LPSTR,
    pub cchText: ::c_int,
}
pub type LPTBBUTTONINFOA = *mut TBBUTTONINFOA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct TBBUTTONINFOW {
    pub cbSize: ::UINT,
    pub dwMask: ::DWORD,
    pub idCommand: ::c_int,
    pub iImage: ::c_int,
    pub fsState: ::BYTE,
    pub fsStyle: ::BYTE,
    pub cx: ::WORD,
    pub lParam: ::DWORD_PTR,
    pub pszText: ::LPWSTR,
    pub cchText: ::c_int,
}
pub type LPTBBUTTONINFOW = *mut TBBUTTONINFOW;
pub const TB_GETBUTTONINFOW: ::UINT = ::WM_USER + 63;
pub const TB_SETBUTTONINFOW: ::UINT = ::WM_USER + 64;
pub const TB_GETBUTTONINFOA: ::UINT = ::WM_USER + 65;
pub const TB_SETBUTTONINFOA: ::UINT = ::WM_USER + 66;
pub const TB_INSERTBUTTONW: ::UINT = ::WM_USER + 67;
pub const TB_ADDBUTTONSW: ::UINT = ::WM_USER + 68;
pub const TB_HITTEST: ::UINT = ::WM_USER + 69;
pub const TB_SETDRAWTEXTFLAGS: ::UINT = ::WM_USER + 70;
pub const TB_GETSTRINGW: ::UINT = ::WM_USER + 91;
pub const TB_GETSTRINGA: ::UINT = ::WM_USER + 92;
pub const TB_SETBOUNDINGSIZE: ::UINT = ::WM_USER + 93;
pub const TB_SETHOTITEM2: ::UINT = ::WM_USER + 94;
pub const TB_HASACCELERATOR: ::UINT = ::WM_USER + 95;
pub const TB_SETLISTGAP: ::UINT = ::WM_USER + 96;
pub const TB_GETIMAGELISTCOUNT: ::UINT = ::WM_USER + 98;
pub const TB_GETIDEALSIZE: ::UINT = ::WM_USER + 99;
pub const TBMF_PAD: ::DWORD = 0x00000001;
pub const TBMF_BARPAD: ::DWORD = 0x00000002;
pub const TBMF_BUTTONSPACING: ::DWORD = 0x00000004;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct TBMETRICS {
    pub cbSize: ::UINT,
    pub dwMask: ::DWORD,
    pub cxPad: ::c_int,
    pub cyPad: ::c_int,
    pub cxBarPad: ::c_int,
    pub cyBarPad: ::c_int,
    pub cxButtonSpacing: ::c_int,
    pub cyButtonSpacing: ::c_int,
}
pub type LPTBMETRICS = *mut TBMETRICS;
pub const TB_GETMETRICS: ::UINT = ::WM_USER + 101;
pub const TB_SETMETRICS: ::UINT = ::WM_USER + 102;
pub const TB_GETITEMDROPDOWNRECT: ::UINT = ::WM_USER + 103;
pub const TB_SETPRESSEDIMAGELIST: ::UINT = ::WM_USER + 104;
pub const TB_GETPRESSEDIMAGELIST: ::UINT = ::WM_USER + 105;
pub const TB_SETWINDOWTHEME: ::UINT = CCM_SETWINDOWTHEME;
pub const TBN_GETBUTTONINFOA: ::UINT = TBN_FIRST - 0;
pub const TBN_BEGINDRAG: ::UINT = TBN_FIRST - 1;
pub const TBN_ENDDRAG: ::UINT = TBN_FIRST - 2;
pub const TBN_BEGINADJUST: ::UINT = TBN_FIRST - 3;
pub const TBN_ENDADJUST: ::UINT = TBN_FIRST - 4;
pub const TBN_RESET: ::UINT = TBN_FIRST - 5;
pub const TBN_QUERYINSERT: ::UINT = TBN_FIRST - 6;
pub const TBN_QUERYDELETE: ::UINT = TBN_FIRST - 7;
pub const TBN_TOOLBARCHANGE: ::UINT = TBN_FIRST - 8;
pub const TBN_CUSTHELP: ::UINT = TBN_FIRST - 9;
pub const TBN_DROPDOWN: ::UINT = TBN_FIRST - 10;
pub const TBN_GETOBJECT: ::UINT = TBN_FIRST - 12;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMTBHOTITEM {
    pub hdr: ::NMHDR,
    pub idOld: ::c_int,
    pub idNew: ::c_int,
    pub dwFlags: ::DWORD,
}
pub type LPNMTBHOTITEM = *mut NMTBHOTITEM;
pub const HICF_OTHER: ::DWORD = 0x00000000;
pub const HICF_MOUSE: ::DWORD = 0x00000001;
pub const HICF_ARROWKEYS: ::DWORD = 0x00000002;
pub const HICF_ACCELERATOR: ::DWORD = 0x00000004;
pub const HICF_DUPACCEL: ::DWORD = 0x00000008;
pub const HICF_ENTERING: ::DWORD = 0x00000010;
pub const HICF_LEAVING: ::DWORD = 0x00000020;
pub const HICF_RESELECT: ::DWORD = 0x00000040;
pub const HICF_LMOUSE: ::DWORD = 0x00000080;
pub const HICF_TOGGLEDROPDOWN: ::DWORD = 0x00000100;
pub const TBN_HOTITEMCHANGE: ::UINT = TBN_FIRST - 13;
pub const TBN_DRAGOUT: ::UINT = TBN_FIRST - 14;
pub const TBN_DELETINGBUTTON: ::UINT = TBN_FIRST - 15;
pub const TBN_GETDISPINFOA: ::UINT = TBN_FIRST - 16;
pub const TBN_GETDISPINFOW: ::UINT = TBN_FIRST - 17;
pub const TBN_GETINFOTIPA: ::UINT = TBN_FIRST - 18;
pub const TBN_GETINFOTIPW: ::UINT = TBN_FIRST - 19;
pub const TBN_GETBUTTONINFOW: ::UINT = TBN_FIRST - 20;
pub const TBN_RESTORE: ::UINT = TBN_FIRST - 21;
pub const TBN_SAVE: ::UINT = TBN_FIRST - 22;
pub const TBN_INITCUSTOMIZE: ::UINT = TBN_FIRST - 23;
pub const TBN_WRAPHOTITEM: ::UINT = TBN_FIRST - 24;
pub const TBN_DUPACCELERATOR: ::UINT = TBN_FIRST - 25;
pub const TBN_WRAPACCELERATOR: ::UINT = TBN_FIRST - 26;
pub const TBN_DRAGOVER: ::UINT = TBN_FIRST - 27;
pub const TBN_MAPACCELERATOR: ::UINT = TBN_FIRST - 28;
pub const TBNRF_HIDEHELP: ::LRESULT = 0x00000001;
pub const TBNRF_ENDCUSTOMIZE: ::LRESULT = 0x00000002;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMTBSAVE {
    pub hdr: ::NMHDR,
    pub pData: *mut ::DWORD,
    pub pCurrent: *mut ::DWORD,
    pub cbData: ::UINT,
    pub iItem: ::c_int,
    pub cButtons: ::c_int,
    pub tbButton: TBBUTTON,
}
pub type LPNMTBSAVE = *mut NMTBSAVE;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMTBRESTORE {
    pub hdr: ::NMHDR,
    pub pData: *mut ::DWORD,
    pub pCurrent: *mut ::DWORD,
    pub cbData: ::UINT,
    pub iItem: ::c_int,
    pub cButtons: ::c_int,
    pub cbBytesPerRecord: ::c_int,
    pub tbButton: TBBUTTON,
}
pub type LPNMTBRESTORE = *mut NMTBRESTORE;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMTBGETINFOTIPA {
    pub hdr: ::NMHDR,
    pub pszText: ::LPSTR,
    pub cchTextMax: ::c_int,
    pub iItem: ::c_int,
    pub lParal: ::LPARAM,
}
pub type LPNMTBGETINFOTIPA = *mut NMTBGETINFOTIPA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMTBGETINFOTIPW {
    pub hdr: ::NMHDR,
    pub pszText: ::LPWSTR,
    pub cchTextMax: ::c_int,
    pub iItem: ::c_int,
    pub lParal: ::LPARAM,
}
pub type LPNMTBGETINFOTIPW = *mut NMTBGETINFOTIPW;
pub const TBNF_IMAGE: ::DWORD = 0x00000001;
pub const TBNF_TEXT: ::DWORD = 0x00000002;
pub const TBNF_DI_SETITEM: ::DWORD = 0x10000000;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMTBDISPINFOA {
    pub hdr: ::NMHDR,
    pub dwMask: ::DWORD,
    pub idCommand: ::c_int,
    pub lParam: ::DWORD_PTR,
    pub iImage: ::c_int,
    pub pszText: ::LPSTR,
    pub cchText: ::c_int,
}
pub type LPNMTBDISPINFOA = *mut NMTBDISPINFOA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMTBDISPINFOW {
    pub hdr: ::NMHDR,
    pub dwMask: ::DWORD,
    pub idCommand: ::c_int,
    pub lParam: ::DWORD_PTR,
    pub iImage: ::c_int,
    pub pszText: ::LPWSTR,
    pub cchText: ::c_int,
}
pub type LPNMTBDISPINFOW = *mut NMTBDISPINFOW;
pub const TBDDRET_DEFAULT: ::LRESULT = 0;
pub const TBDDRET_NODEFAULT: ::LRESULT = 1;
pub const TBDDRET_TREATPRESSED: ::LRESULT = 2;
pub type TBNOTIFYA = NMTOOLBARA;
pub type TBNOTIFYW = NMTOOLBARW;
pub type LPTBNOTIFYA = LPNMTOOLBARA;
pub type LPTBNOTIFYW = LPNMTOOLBARW;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMTOOLBARA {
    pub hdr: ::NMHDR,
    pub iItem: ::c_int,
    pub tbButton: TBBUTTON,
    pub cchText: ::c_int,
    pub pszText: ::LPSTR,
    pub rcButton: ::RECT,
}
pub type LPNMTOOLBARA = *mut NMTOOLBARA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMTOOLBARW {
    pub hdr: ::NMHDR,
    pub iItem: ::c_int,
    pub tbButton: TBBUTTON,
    pub cchText: ::c_int,
    pub pszText: ::LPWSTR,
    pub rcButton: ::RECT,
}
pub type LPNMTOOLBARW = *mut NMTOOLBARW;
pub const RBIM_IMAGELIST: ::UINT = 0x00000001;
pub const RBS_TOOLTIPS: ::DWORD = 0x00000100;
pub const RBS_VARHEIGHT: ::DWORD = 0x00000200;
pub const RBS_BANDBORDERS: ::DWORD = 0x00000400;
pub const RBS_FIXEDORDER: ::DWORD = 0x00000800;
pub const RBS_REGISTERDROP: ::DWORD = 0x00001000;
pub const RBS_AUTOSIZE: ::DWORD = 0x00002000;
pub const RBS_VERTICALGRIPPER: ::DWORD = 0x00004000;
pub const RBS_DBLCLKTOGGLE: ::DWORD = 0x00008000;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct REBARINFO {
    pub cbSize: ::UINT,
    pub fMask: ::UINT,
    pub himl: HIMAGELIST,
}
pub type LPREBARINFO = *mut REBARINFO;
pub const RBBS_BREAK: ::UINT = 0x00000001;
pub const RBBS_FIXEDSIZE: ::UINT = 0x00000002;
pub const RBBS_CHILDEDGE: ::UINT = 0x00000004;
pub const RBBS_HIDDEN: ::UINT = 0x00000008;
pub const RBBS_NOVERT: ::UINT = 0x00000010;
pub const RBBS_FIXEDBMP: ::UINT = 0x00000020;
pub const RBBS_VARIABLEHEIGHT: ::UINT = 0x00000040;
pub const RBBS_GRIPPERALWAYS: ::UINT = 0x00000080;
pub const RBBS_NOGRIPPER: ::UINT = 0x00000100;
pub const RBBS_USECHEVRON: ::UINT = 0x00000200;
pub const RBBS_HIDETITLE: ::UINT = 0x00000400;
pub const RBBS_TOPALIGN: ::UINT = 0x00000800;
pub const RBBIM_STYLE: ::UINT = 0x00000001;
pub const RBBIM_COLORS: ::UINT = 0x00000002;
pub const RBBIM_TEXT: ::UINT = 0x00000004;
pub const RBBIM_IMAGE: ::UINT = 0x00000008;
pub const RBBIM_CHILD: ::UINT = 0x00000010;
pub const RBBIM_CHILDSIZE: ::UINT = 0x00000020;
pub const RBBIM_SIZE: ::UINT = 0x00000040;
pub const RBBIM_BACKGROUND: ::UINT = 0x00000080;
pub const RBBIM_ID: ::UINT = 0x00000100;
pub const RBBIM_IDEALSIZE: ::UINT = 0x00000200;
pub const RBBIM_LPARAM: ::UINT = 0x00000400;
pub const RBBIM_HEADERSIZE: ::UINT = 0x00000800;
pub const RBBIM_CHEVRONLOCATION: ::UINT = 0x00001000;
pub const RBBIM_CHEVRONSTATE: ::UINT = 0x00002000;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct REBARBANDINFOA {
    pub cbSize: ::UINT,
    pub fMask: ::UINT,
    pub fStyle: ::UINT,
    pub clrFore: ::COLORREF,
    pub clrBack: ::COLORREF,
    pub lpText: ::LPSTR,
    pub cch: ::UINT,
    pub iImage: ::c_int,
    pub hwndChild: ::HWND,
    pub cxMinChild: ::UINT,
    pub cyMinChild: ::UINT,
    pub cx: ::UINT,
    pub hbmBack: ::HBITMAP,
    pub wID: ::UINT,
    pub cyChild: ::UINT,
    pub cyMaxChild: ::UINT,
    pub cyIntegral: ::UINT,
    pub cxIdeal: ::UINT,
    pub lParam: ::LPARAM,
    pub cxHeader: ::UINT,
    pub rcChevronLocation: ::RECT,
    pub uChevronState: ::UINT,
}
pub type LPREBARBANDINFOA = *mut REBARBANDINFOA;
pub type LPCREBARBANDINFOA = *const REBARBANDINFOA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct REBARBANDINFOW {
    pub cbSize: ::UINT,
    pub fMask: ::UINT,
    pub fStyle: ::UINT,
    pub clrFore: ::COLORREF,
    pub clrBack: ::COLORREF,
    pub lpText: ::LPWSTR,
    pub cch: ::UINT,
    pub iImage: ::c_int,
    pub hwndChild: ::HWND,
    pub cxMinChild: ::UINT,
    pub cyMinChild: ::UINT,
    pub cx: ::UINT,
    pub hbmBack: ::HBITMAP,
    pub wID: ::UINT,
    pub cyChild: ::UINT,
    pub cyMaxChild: ::UINT,
    pub cyIntegral: ::UINT,
    pub cxIdeal: ::UINT,
    pub lParam: ::LPARAM,
    pub cxHeader: ::UINT,
    pub rcChevronLocation: ::RECT,
    pub uChevronState: ::UINT,
}
pub type LPREBARBANDINFOW = *mut REBARBANDINFOW;
pub type LPCREBARBANDINFOW = *const REBARBANDINFOW;
pub const RB_INSERTBANDA: ::UINT = ::WM_USER + 1;
pub const RB_DELETEBAND: ::UINT = ::WM_USER + 2;
pub const RB_GETBARINFO: ::UINT = ::WM_USER + 3;
pub const RB_SETBARINFO: ::UINT = ::WM_USER + 4;
pub const RB_SETBANDINFOA: ::UINT = ::WM_USER + 6;
pub const RB_SETPARENT: ::UINT = ::WM_USER + 7;
pub const RB_HITTEST: ::UINT = ::WM_USER + 8;
pub const RB_GETRECT: ::UINT = ::WM_USER + 9;
pub const RB_INSERTBANDW: ::UINT = ::WM_USER + 10;
pub const RB_SETBANDINFOW: ::UINT = ::WM_USER + 11;
pub const RB_GETBANDCOUNT: ::UINT = ::WM_USER + 12;
pub const RB_GETROWCOUNT: ::UINT = ::WM_USER + 13;
pub const RB_GETROWHEIGHT: ::UINT = ::WM_USER + 14;
pub const RB_IDTOINDEX: ::UINT = ::WM_USER + 16;
pub const RB_GETTOOLTIPS: ::UINT = ::WM_USER + 17;
pub const RB_SETTOOLTIPS: ::UINT = ::WM_USER + 18;
pub const RB_SETBKCOLOR: ::UINT = ::WM_USER + 19;
pub const RB_GETBKCOLOR: ::UINT = ::WM_USER + 20;
pub const RB_SETTEXTCOLOR: ::UINT = ::WM_USER + 21;
pub const RB_GETTEXTCOLOR: ::UINT = ::WM_USER + 22;
pub const RBSTR_CHANGERECT: ::WPARAM = 0x0001;
pub const RB_SIZETORECT: ::UINT = ::WM_USER + 23;
pub const RB_SETCOLORSCHEME: ::UINT = CCM_SETCOLORSCHEME;
pub const RB_GETCOLORSCHEME: ::UINT = CCM_GETCOLORSCHEME;
pub const RB_BEGINDRAG: ::UINT = ::WM_USER + 24;
pub const RB_ENDDRAG: ::UINT = ::WM_USER + 25;
pub const RB_DRAGMOVE: ::UINT = ::WM_USER + 26;
pub const RB_GETBARHEIGHT: ::UINT = ::WM_USER + 27;
pub const RB_GETBANDINFOW: ::UINT = ::WM_USER + 28;
pub const RB_GETBANDINFOA: ::UINT = ::WM_USER + 29;
pub const RB_MINIMIZEBAND: ::UINT = ::WM_USER + 30;
pub const RB_MAXIMIZEBAND: ::UINT = ::WM_USER + 31;
pub const RB_GETDROPTARGET: ::UINT = CCM_GETDROPTARGET;
pub const RB_GETBANDBORDERS: ::UINT = ::WM_USER + 34;
pub const RB_SHOWBAND: ::UINT = ::WM_USER + 35;
pub const RB_SETPALETTE: ::UINT = ::WM_USER + 37;
pub const RB_GETPALETTE: ::UINT = ::WM_USER + 38;
pub const RB_MOVEBAND: ::UINT = ::WM_USER + 39;
pub const RB_SETUNICODEFORMAT: ::UINT = CCM_SETUNICODEFORMAT;
pub const RB_GETUNICODEFORMAT: ::UINT = CCM_GETUNICODEFORMAT;
pub const RB_GETBANDMARGINS: ::UINT = ::WM_USER + 40;
pub const RB_SETWINDOWTHEME: ::UINT = CCM_SETWINDOWTHEME;
pub const RB_SETEXTENDEDSTYLE: ::UINT = ::WM_USER + 41;
pub const RB_GETEXTENDEDSTYLE: ::UINT = ::WM_USER + 42;
pub const RB_PUSHCHEVRON: ::UINT = ::WM_USER + 43;
pub const RB_SETBANDWIDTH: ::UINT = ::WM_USER + 44;
pub const RBN_HEIGHTCHANGE: ::UINT = RBN_FIRST - 0;
pub const RBN_GETOBJECT: ::UINT = RBN_FIRST - 1;
pub const RBN_LAYOUTCHANGED: ::UINT = RBN_FIRST - 2;
pub const RBN_AUTOSIZE: ::UINT = RBN_FIRST - 3;
pub const RBN_BEGINDRAG: ::UINT = RBN_FIRST - 4;
pub const RBN_ENDDRAG: ::UINT = RBN_FIRST - 5;
pub const RBN_DELETINGBAND: ::UINT = RBN_FIRST - 6;
pub const RBN_DELETEDBAND: ::UINT = RBN_FIRST - 7;
pub const RBN_CHILDSIZE: ::UINT = RBN_FIRST - 8;
pub const RBN_CHEVRONPUSHED: ::UINT = RBN_FIRST - 10;
pub const RBN_SPLITTERDRAG: ::UINT = RBN_FIRST - 11;
pub const RBN_MINMAX: ::UINT = RBN_FIRST - 21;
pub const RBN_AUTOBREAK: ::UINT = RBN_FIRST - 22;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMREBARCHILDSIZE {
    pub hdr: ::NMHDR,
    pub uBand: ::UINT,
    pub wID: ::UINT,
    pub rcChild: ::RECT,
    pub rcBand: ::RECT,
}
pub type LPNMREBARCHILDSIZE = *mut NMREBARCHILDSIZE;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMREBAR {
    pub hdr: ::NMHDR,
    pub dwMask: ::DWORD,
    pub uBand: ::UINT,
    pub fStyle: ::UINT,
    pub wID: ::UINT,
    pub lParam: ::LPARAM,
}
pub type LPNMREBAR = *mut NMREBAR;
pub const RBNM_ID: ::DWORD = 0x00000001;
pub const RBNM_STYLE: ::DWORD = 0x00000002;
pub const RBNM_LPARAM: ::DWORD = 0x00000004;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMRBAUTOSIZE {
    pub hdr: ::NMHDR,
    pub fChanged: ::BOOL,
    pub rcTarget: ::RECT,
    pub rcActual: ::RECT,
}
pub type LPNMRBAUTOSIZE = *mut NMRBAUTOSIZE;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMREBARCHEVRON {
    pub hdr: ::NMHDR,
    pub uBand: ::UINT,
    pub wID: ::UINT,
    pub lParam: ::LPARAM,
    pub rc: ::RECT,
    pub lParamNM: ::LPARAM,
}
pub type LPNMREBARCHEVRON = *mut NMREBARCHEVRON;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMREBARSPLITTER {
    pub hdr: ::NMHDR,
    pub rcSizing: ::RECT,
}
pub type LPNMREBARSPLITTER = *mut NMREBARSPLITTER;
pub const RBAB_AUTOSIZE: ::UINT = 0x0001;
pub const RBAB_ADDBAND: ::UINT = 0x0002;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMREBARAUTOBREAK {
    pub hdr: ::NMHDR,
    pub uBand: ::UINT,
    pub wID: ::UINT,
    pub lParam: ::LPARAM,
    pub uMsg: ::UINT,
    pub fStyleCurrent: ::UINT,
    pub fAutoBreak: ::UINT,
}
pub type LPNMREBARAUTOBREAK = *mut NMREBARAUTOBREAK;
pub const RBHT_NOWHERE: ::UINT = 0x0001;
pub const RBHT_CAPTION: ::UINT = 0x0002;
pub const RBHT_CLIENT: ::UINT = 0x0003;
pub const RBHT_GRABBER: ::UINT = 0x0004;
pub const RBHT_CHEVRON: ::UINT = 0x0008;
pub const RBHT_SPLITTER: ::UINT = 0x0010;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct RBHITTESTINFO {
    pub pt: ::POINT,
    pub flags: ::UINT,
    pub iBand: ::c_int,
}
pub type LPRBHITTESTINFO = *mut RBHITTESTINFO;
pub type LPTOOLINFOA = LPTTTOOLINFOA;
pub type LPTOOLINFOW = LPTTTOOLINFOW;
pub type TOOLINFOA = TTTOOLINFOA;
pub type TOOLINFOW = TTTOOLINFOW;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct TTTOOLINFOA {
    pub cbSize: ::UINT,
    pub uFlags: ::UINT,
    pub hwnd: ::HWND,
    pub uId: ::UINT_PTR,
    pub rect: ::RECT,
    pub hinst: ::HINSTANCE,
    pub lpszText: ::LPSTR,
    pub lParam: ::LPARAM,
    pub lpReserved: *mut ::c_void,
}
pub type PTTTOOLINFOA = *mut TTTOOLINFOA;
pub type LPTTTOOLINFOA = *mut TTTOOLINFOA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct TTTOOLINFOW {
    pub cbSize: ::UINT,
    pub uFlags: ::UINT,
    pub hwnd: ::HWND,
    pub uId: ::UINT_PTR,
    pub rect: ::RECT,
    pub hinst: ::HINSTANCE,
    pub lpszText: ::LPSTR,
    pub lParam: ::LPARAM,
    pub lpReserved: *mut ::c_void,
}
pub type PTTTOOLINFOW = *mut TTTOOLINFOW;
pub type LPTTTOOLINFOW = *mut TTTOOLINFOW;
pub const TTS_ALWAYSTIP: ::DWORD = 0x01;
pub const TTS_NOPREFIX: ::DWORD = 0x02;
pub const TTS_NOANIMATE: ::DWORD = 0x10;
pub const TTS_NOFADE: ::DWORD = 0x20;
pub const TTS_BALLOON: ::DWORD = 0x40;
pub const TTS_CLOSE: ::DWORD = 0x80;
pub const TTS_USEVISUALSTYLE: ::DWORD = 0x100;
pub const TTF_IDISHWND: ::UINT = 0x0001;
pub const TTF_CENTERTIP: ::UINT = 0x0002;
pub const TTF_RTLREADING: ::UINT = 0x0004;
pub const TTF_SUBCLASS: ::UINT = 0x0010;
pub const TTF_TRACK: ::UINT = 0x0020;
pub const TTF_ABSOLUTE: ::UINT = 0x0080;
pub const TTF_TRANSPARENT: ::UINT = 0x0100;
pub const TTF_PARSELINKS: ::UINT = 0x1000;
pub const TTF_DI_SETITEM: ::UINT = 0x8000;
pub const TTDT_AUTOMATIC: ::WPARAM = 0;
pub const TTDT_RESHOW: ::WPARAM = 1;
pub const TTDT_AUTOPOP: ::WPARAM = 2;
pub const TTDT_INITIAL: ::WPARAM = 3;
pub const TTI_NONE: ::WPARAM = 0;
pub const TTI_INFO: ::WPARAM = 1;
pub const TTI_WARNING: ::WPARAM = 2;
pub const TTI_ERROR: ::WPARAM = 3;
pub const TTI_INFO_LARGE: ::WPARAM = 4;
pub const TTI_WARNING_LARGE: ::WPARAM = 5;
pub const TTI_ERROR_LARGE: ::WPARAM = 6;
pub const TTM_ACTIVATE: ::UINT = ::WM_USER + 1;
pub const TTM_SETDELAYTIME: ::UINT = ::WM_USER + 3;
pub const TTM_ADDTOOLA: ::UINT = ::WM_USER + 4;
pub const TTM_ADDTOOLW: ::UINT = ::WM_USER + 50;
pub const TTM_DELTOOLA: ::UINT = ::WM_USER + 5;
pub const TTM_DELTOOLW: ::UINT = ::WM_USER + 51;
pub const TTM_NEWTOOLRECTA: ::UINT = ::WM_USER + 6;
pub const TTM_NEWTOOLRECTW: ::UINT = ::WM_USER + 52;
pub const TTM_RELAYEVENT: ::UINT = ::WM_USER + 7;
pub const TTM_GETTOOLINFOA: ::UINT = ::WM_USER + 8;
pub const TTM_GETTOOLINFOW: ::UINT = ::WM_USER + 53;
pub const TTM_SETTOOLINFOA: ::UINT = ::WM_USER + 9;
pub const TTM_SETTOOLINFOW: ::UINT = ::WM_USER + 54;
pub const TTM_HITTESTA: ::UINT = ::WM_USER + 10;
pub const TTM_HITTESTW: ::UINT = ::WM_USER + 55;
pub const TTM_GETTEXTA: ::UINT = ::WM_USER + 11;
pub const TTM_GETTEXTW: ::UINT = ::WM_USER + 56;
pub const TTM_UPDATETIPTEXTA: ::UINT = ::WM_USER + 12;
pub const TTM_UPDATETIPTEXTW: ::UINT = ::WM_USER + 57;
pub const TTM_GETTOOLCOUNT: ::UINT = ::WM_USER + 13;
pub const TTM_ENUMTOOLSA: ::UINT = ::WM_USER + 14;
pub const TTM_ENUMTOOLSW: ::UINT = ::WM_USER + 58;
pub const TTM_GETCURRENTTOOLA: ::UINT = ::WM_USER + 15;
pub const TTM_GETCURRENTTOOLW: ::UINT = ::WM_USER + 59;
pub const TTM_WINDOWFROMPOINT: ::UINT = ::WM_USER + 16;
pub const TTM_TRACKACTIVATE: ::UINT = ::WM_USER + 17;
pub const TTM_TRACKPOSITION: ::UINT = ::WM_USER + 18;
pub const TTM_SETTIPBKCOLOR: ::UINT = ::WM_USER + 19;
pub const TTM_SETTIPTEXTCOLOR: ::UINT = ::WM_USER + 20;
pub const TTM_GETDELAYTIME: ::UINT = ::WM_USER + 21;
pub const TTM_GETTIPBKCOLOR: ::UINT = ::WM_USER + 22;
pub const TTM_GETTIPTEXTCOLOR: ::UINT = ::WM_USER + 23;
pub const TTM_SETMAXTIPWIDTH: ::UINT = ::WM_USER + 24;
pub const TTM_GETMAXTIPWIDTH: ::UINT = ::WM_USER + 25;
pub const TTM_SETMARGIN: ::UINT = ::WM_USER + 26;
pub const TTM_GETMARGIN: ::UINT = ::WM_USER + 27;
pub const TTM_POP: ::UINT = ::WM_USER + 28;
pub const TTM_UPDATE: ::UINT = ::WM_USER + 29;
pub const TTM_GETBUBBLESIZE: ::UINT = ::WM_USER + 30;
pub const TTM_ADJUSTRECT: ::UINT = ::WM_USER + 31;
pub const TTM_SETTITLEA: ::UINT = ::WM_USER + 32;
pub const TTM_SETTITLEW: ::UINT = ::WM_USER + 33;
pub const TTM_POPUP: ::UINT = ::WM_USER + 34;
pub const TTM_GETTITLE: ::UINT = ::WM_USER + 35;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct TTGETTITLE {
    pub dwSize: ::DWORD,
    pub uTitleBitmap: ::UINT,
    pub cch: ::UINT,
    pub pszTitle: *mut ::WCHAR,
}
pub type LPTTGETTITLE = *mut TTGETTITLE;
pub const TTM_SETWINDOWTHEME: ::UINT = CCM_SETWINDOWTHEME;
pub type LPHITTESTINFOW = LPTTHITTESTINFOW;
pub type LPHITTESTINFOA = LPTTHITTESTINFOA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct TTHITTESTINFOA {
    pub hwnd: ::HWND,
    pub pt: ::POINT,
    pub ti: TTTOOLINFOA,
}
pub type LPTTHITTESTINFOA = *mut TTHITTESTINFOA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct TTHITTESTINFOW {
    pub hwnd: ::HWND,
    pub pt: ::POINT,
    pub ti: TTTOOLINFOW,
}
pub type LPTTHITTESTINFOW = *mut TTHITTESTINFOW;
pub const TTN_GETDISPINFOA: ::UINT = TTN_FIRST - 0;
pub const TTN_GETDISPINFOW: ::UINT = TTN_FIRST - 10;
pub const TTN_SHOW: ::UINT = TTN_FIRST - 1;
pub const TTN_POP: ::UINT = TTN_FIRST - 2;
pub const TTN_LINKCLICK: ::UINT = TTN_FIRST - 3;
pub const TTN_NEEDTEXTA: ::UINT = TTN_GETDISPINFOA;
pub const TTN_NEEDTEXTW: ::UINT = TTN_GETDISPINFOW;
pub type TOOLTIPTEXTW = NMTTDISPINFOW;
pub type TOOLTIPTEXTA = NMTTDISPINFOA;
pub type LPTOOLTIPTEXTA = LPNMTTDISPINFOA;
pub type LPTOOLTIPTEXTW = LPNMTTDISPINFOW;
#[repr(C)] #[derive(Copy)]
pub struct NMTTDISPINFOA {
    pub hdr: ::NMHDR,
    pub lpszText: ::LPSTR,
    pub szText: [::c_char; 80],
    pub hinst: ::HINSTANCE,
    pub uFlags: ::UINT,
    pub lParam: ::LPARAM,
}
impl Clone for NMTTDISPINFOA { fn clone(&self) -> NMTTDISPINFOA { *self } }
pub type LPNMTTDISPINFOA = *mut NMTTDISPINFOA;
#[repr(C)] #[derive(Copy)]
pub struct NMTTDISPINFOW {
    pub hdr: ::NMHDR,
    pub lpszText: ::LPWSTR,
    pub szText: [::WCHAR; 80],
    pub hinst: ::HINSTANCE,
    pub uFlags: ::UINT,
    pub lParam: ::LPARAM,
}
impl Clone for NMTTDISPINFOW { fn clone(&self) -> NMTTDISPINFOW { *self } }
pub type LPNMTTDISPINFOW = *mut NMTTDISPINFOW;
pub const SBARS_SIZEGRIP: ::DWORD = 0x0100;
pub const SBARS_TOOLTIPS: ::DWORD = 0x0800;
pub const SBT_TOOLTIPS: ::DWORD = 0x0800;
pub const SB_SETTEXTA: ::UINT = ::WM_USER + 1;
pub const SB_SETTEXTW: ::UINT = ::WM_USER + 11;
pub const SB_GETTEXTA: ::UINT = ::WM_USER + 2;
pub const SB_GETTEXTW: ::UINT = ::WM_USER + 13;
pub const SB_GETTEXTLENGTHA: ::UINT = ::WM_USER + 3;
pub const SB_GETTEXTLENGTHW: ::UINT = ::WM_USER + 12;
pub const SB_SETPARTS: ::UINT = ::WM_USER + 4;
pub const SB_GETPARTS: ::UINT = ::WM_USER + 6;
pub const SB_GETBORDERS: ::UINT = ::WM_USER + 7;
pub const SB_SETMINHEIGHT: ::UINT = ::WM_USER + 8;
pub const SB_SIMPLE: ::UINT = ::WM_USER + 9;
pub const SB_GETRECT: ::UINT = ::WM_USER + 10;
pub const SB_ISSIMPLE: ::UINT = ::WM_USER + 14;
pub const SB_SETICON: ::UINT = ::WM_USER + 15;
pub const SB_SETTIPTEXTA: ::UINT = ::WM_USER + 16;
pub const SB_SETTIPTEXTW: ::UINT = ::WM_USER + 17;
pub const SB_GETTIPTEXTA: ::UINT = ::WM_USER + 18;
pub const SB_GETTIPTEXTW: ::UINT = ::WM_USER + 19;
pub const SB_GETICON: ::UINT = ::WM_USER + 20;
pub const SB_SETUNICODEFORMAT: ::UINT = CCM_SETUNICODEFORMAT;
pub const SB_GETUNICODEFORMAT: ::UINT = CCM_GETUNICODEFORMAT;
pub const SBT_OWNERDRAW: ::WPARAM = 0x1000;
pub const SBT_NOBORDERS: ::WPARAM = 0x0100;
pub const SBT_POPOUT: ::WPARAM = 0x0200;
pub const SBT_RTLREADING: ::WPARAM = 0x0400;
pub const SBT_NOTABPARSING: ::WPARAM = 0x0800;
pub const SB_SETBKCOLOR: ::UINT = CCM_SETBKCOLOR;
pub const SBN_SIMPLEMODECHANGE: ::UINT = SBN_FIRST - 0;
pub const SB_SIMPLEID: ::WPARAM = 0x00ff;
pub const TBS_AUTOTICKS: ::DWORD = 0x0001;
pub const TBS_VERT: ::DWORD = 0x0002;
pub const TBS_HORZ: ::DWORD = 0x0000;
pub const TBS_TOP: ::DWORD = 0x0004;
pub const TBS_BOTTOM: ::DWORD = 0x0000;
pub const TBS_LEFT: ::DWORD = 0x0004;
pub const TBS_RIGHT: ::DWORD = 0x0000;
pub const TBS_BOTH: ::DWORD = 0x0008;
pub const TBS_NOTICKS: ::DWORD = 0x0010;
pub const TBS_ENABLESELRANGE: ::DWORD = 0x0020;
pub const TBS_FIXEDLENGTH: ::DWORD = 0x0040;
pub const TBS_NOTHUMB: ::DWORD = 0x0080;
pub const TBS_TOOLTIPS: ::DWORD = 0x0100;
pub const TBS_REVERSED: ::DWORD = 0x0200;
pub const TBS_DOWNISLEFT: ::DWORD = 0x0400;
pub const TBS_NOTIFYBEFOREMOVE: ::DWORD = 0x0800;
pub const TBS_TRANSPARENTBKGND: ::DWORD = 0x1000;
pub const TBM_GETPOS: ::UINT = ::WM_USER;
pub const TBM_GETRANGEMIN: ::UINT = ::WM_USER + 1;
pub const TBM_GETRANGEMAX: ::UINT = ::WM_USER + 2;
pub const TBM_GETTIC: ::UINT = ::WM_USER + 3;
pub const TBM_SETTIC: ::UINT = ::WM_USER + 4;
pub const TBM_SETPOS: ::UINT = ::WM_USER + 5;
pub const TBM_SETRANGE: ::UINT = ::WM_USER + 6;
pub const TBM_SETRANGEMIN: ::UINT = ::WM_USER + 7;
pub const TBM_SETRANGEMAX: ::UINT = ::WM_USER + 8;
pub const TBM_CLEARTICS: ::UINT = ::WM_USER + 9;
pub const TBM_SETSEL: ::UINT = ::WM_USER + 10;
pub const TBM_SETSELSTART: ::UINT = ::WM_USER + 11;
pub const TBM_SETSELEND: ::UINT = ::WM_USER + 12;
pub const TBM_GETPTICS: ::UINT = ::WM_USER + 14;
pub const TBM_GETTICPOS: ::UINT = ::WM_USER + 15;
pub const TBM_GETNUMTICS: ::UINT = ::WM_USER + 16;
pub const TBM_GETSELSTART: ::UINT = ::WM_USER + 17;
pub const TBM_GETSELEND: ::UINT = ::WM_USER + 18;
pub const TBM_CLEARSEL: ::UINT = ::WM_USER + 19;
pub const TBM_SETTICFREQ: ::UINT = ::WM_USER + 20;
pub const TBM_SETPAGESIZE: ::UINT = ::WM_USER + 21;
pub const TBM_GETPAGESIZE: ::UINT = ::WM_USER + 22;
pub const TBM_SETLINESIZE: ::UINT = ::WM_USER + 23;
pub const TBM_GETLINESIZE: ::UINT = ::WM_USER + 24;
pub const TBM_GETTHUMBRECT: ::UINT = ::WM_USER + 25;
pub const TBM_GETCHANNELRECT: ::UINT = ::WM_USER + 26;
pub const TBM_SETTHUMBLENGTH: ::UINT = ::WM_USER + 27;
pub const TBM_GETTHUMBLENGTH: ::UINT = ::WM_USER + 28;
pub const TBM_SETTOOLTIPS: ::UINT = ::WM_USER + 29;
pub const TBM_GETTOOLTIPS: ::UINT = ::WM_USER + 30;
pub const TBM_SETTIPSIDE: ::UINT = ::WM_USER + 31;
pub const TBTS_TOP: ::WPARAM = 0;
pub const TBTS_LEFT: ::WPARAM = 1;
pub const TBTS_BOTTOM: ::WPARAM = 2;
pub const TBTS_RIGHT: ::WPARAM = 3;
pub const TBM_SETBUDDY: ::UINT = ::WM_USER + 32;
pub const TBM_GETBUDDY: ::UINT = ::WM_USER + 33;
pub const TBM_SETPOSNOTIFY: ::UINT = ::WM_USER + 34;
pub const TBM_SETUNICODEFORMAT: ::UINT = CCM_SETUNICODEFORMAT;
pub const TBM_GETUNICODEFORMAT: ::UINT = CCM_GETUNICODEFORMAT;
pub const TBCD_TICS: ::DWORD_PTR = 0x0001;
pub const TBCD_THUMB: ::DWORD_PTR = 0x0001;
pub const TBCD_CHANNEL: ::DWORD_PTR = 0x0001;
pub const TB_LINEUP: ::WPARAM = 0;
pub const TB_LINEDOWN: ::WPARAM = 1;
pub const TB_PAGEUP: ::WPARAM = 2;
pub const TB_PAGEDOWN: ::WPARAM = 3;
pub const TB_THUMBPOSITION: ::WPARAM = 4;
pub const TB_THUMBTRACK: ::WPARAM = 5;
pub const TB_TOP: ::WPARAM = 6;
pub const TB_BOTTOM: ::WPARAM = 7;
pub const TB_ENDTRACK: ::WPARAM = 8;
pub const TRBN_THUMBPOSCHANGING: ::UINT = TRBN_FIRST - 1;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMTRBTHUMBPOSCHANGING {
    pub hdr: ::NMHDR,
    pub dwPos: ::DWORD,
    pub nReason: ::c_int,
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DRAGLISTINFO {
    pub uNotification: ::UINT,
    pub hWnd: ::HWND,
    pub ptCursor: ::POINT,
}
pub type LPDRAGLISTINFO = *mut DRAGLISTINFO;
pub const DL_BEGINDRAG: ::UINT = ::WM_USER + 133;
pub const DL_DRAGGING: ::UINT = ::WM_USER + 134;
pub const DL_DROPPED: ::UINT = ::WM_USER + 135;
pub const DL_CANCELDRAG: ::UINT = ::WM_USER + 136;
pub const DL_CURSORSET: ::UINT = 0;
pub const DL_STOPCURSOR: ::UINT = 1;
pub const DL_COPYCURSOR: ::UINT = 2;
pub const DL_MOVECURSOR: ::UINT = 3;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct UDACCEL {
    pub nSec: ::UINT,
    pub nInc: ::UINT,
}
pub type LPUDACCEL = *mut UDACCEL;
pub const UD_MAXVAL: ::c_short = 0x7fff;
pub const UD_MINVAL: ::c_short = 0 - UD_MAXVAL;
pub const UDS_WRAP: ::DWORD = 0x0001;
pub const UDS_SETBUDDYINT: ::DWORD = 0x0002;
pub const UDS_ALIGNRIGHT: ::DWORD = 0x0004;
pub const UDS_ALIGNLEFT: ::DWORD = 0x0008;
pub const UDS_AUTOBUDDY: ::DWORD = 0x0010;
pub const UDS_ARROWKEYS: ::DWORD = 0x0020;
pub const UDS_HORZ: ::DWORD = 0x0040;
pub const UDS_NOTHOUSANDS: ::DWORD = 0x0080;
pub const UDS_HOTTRACK: ::DWORD = 0x0100;
pub const UDM_SETRANGE: ::UINT = ::WM_USER + 101;
pub const UDM_GETRANGE: ::UINT = ::WM_USER + 102;
pub const UDM_SETPOS: ::UINT = ::WM_USER + 103;
pub const UDM_GETPOS: ::UINT = ::WM_USER + 104;
pub const UDM_SETBUDDY: ::UINT = ::WM_USER + 105;
pub const UDM_GETBUDDY: ::UINT = ::WM_USER + 106;
pub const UDM_SETACCEL: ::UINT = ::WM_USER + 107;
pub const UDM_GETACCEL: ::UINT = ::WM_USER + 108;
pub const UDM_SETBASE: ::UINT = ::WM_USER + 109;
pub const UDM_GETBASE: ::UINT = ::WM_USER + 110;
pub const UDM_SETRANGE32: ::UINT = ::WM_USER + 111;
pub const UDM_GETRANGE32: ::UINT = ::WM_USER + 112;
pub const UDM_SETUNICODEFORMAT: ::UINT = CCM_SETUNICODEFORMAT;
pub const UDM_GETUNICODEFORMAT: ::UINT = CCM_GETUNICODEFORMAT;
pub const UDM_SETPOS32: ::UINT = ::WM_USER + 113;
pub const UDM_GETPOS32: ::UINT = ::WM_USER + 114;
pub type NM_UPDOWN = NMUPDOWN;
pub type LPNM_UPDOWN = LPNMUPDOWN;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMUPDOWN {
    pub hdr: ::NMHDR,
    pub iPos: ::c_int,
    pub iDelta: ::c_int,
}
pub type LPNMUPDOWN = *mut NMUPDOWN;
pub const UDN_DELTAPOS: ::UINT = UDN_FIRST - 1;
pub const PBS_SMOOTH: ::DWORD = 0x01;
pub const PBS_VERTICAL: ::DWORD = 0x04;
pub const PBM_SETRANGE: ::UINT = ::WM_USER + 1;
pub const PBM_SETPOS: ::UINT = ::WM_USER + 2;
pub const PBM_DELTAPOS: ::UINT = ::WM_USER + 3;
pub const PBM_SETSTEP: ::UINT = ::WM_USER + 4;
pub const PBM_STEPIT: ::UINT = ::WM_USER + 5;
pub const PBM_SETRANGE32: ::UINT = ::WM_USER + 6;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct PBRANGE {
    pub iLow: ::c_int,
    pub iHigh: ::c_int,
}
pub type LPPBRANGE = *mut PBRANGE;
pub const PBM_GETRANGE: ::UINT = ::WM_USER + 7;
pub const PBM_GETPOS: ::UINT = ::WM_USER + 8;
pub const PBM_SETBARCOLOR: ::UINT = ::WM_USER + 9;
pub const PBM_SETBKCOLOR: ::UINT = CCM_SETBKCOLOR;
pub const PBM_SETMARQUEE: ::UINT = ::WM_USER + 10;
pub const PBS_MARQUEE: ::DWORD = 0x08;
pub const PBS_SMOOTHREVERSE: ::DWORD = 0x10;
pub const PBM_GETSTEP: ::UINT = ::WM_USER + 13;
pub const PBM_GETBKCOLOR: ::UINT = ::WM_USER + 14;
pub const PBM_GETBARCOLOR: ::UINT = ::WM_USER + 15;
pub const PBM_SETSTATE: ::UINT = ::WM_USER + 16;
pub const PBM_GETSTATE: ::UINT = ::WM_USER + 17;
pub const PBST_NORMAL: ::c_int = 0x0001;
pub const PBST_ERROR: ::c_int = 0x0002;
pub const PBST_PAUSED: ::c_int = 0x0003;
pub const HOTKEYF_SHIFT: ::BYTE = 0x01;
pub const HOTKEYF_CONTROL: ::BYTE = 0x02;
pub const HOTKEYF_ALT: ::BYTE = 0x04;
pub const HOTKEYF_EXT: ::BYTE = 0x08;
pub const HKCOMB_NONE: ::WPARAM = 0x0001;
pub const HKCOMB_S: ::WPARAM = 0x0002;
pub const HKCOMB_C: ::WPARAM = 0x0004;
pub const HKCOMB_A: ::WPARAM = 0x0008;
pub const HKCOMB_SC: ::WPARAM = 0x0010;
pub const HKCOMB_SA: ::WPARAM = 0x0020;
pub const HKCOMB_CA: ::WPARAM = 0x0040;
pub const HKCOMB_SCA: ::WPARAM = 0x0080;
pub const HKM_SETHOTKEY: ::UINT = ::WM_USER + 1;
pub const HKM_GETHOTKEY: ::UINT = ::WM_USER + 2;
pub const HKM_SETRULES: ::UINT = ::WM_USER + 3;
pub const CCS_TOP: ::DWORD = 0x00000001;
pub const CCS_NOMOVEY: ::DWORD = 0x00000002;
pub const CCS_BOTTOM: ::DWORD = 0x00000003;
pub const CCS_NORESIZE: ::DWORD = 0x00000004;
pub const CCS_NOPARENTALIGN: ::DWORD = 0x00000008;
pub const CCS_ADJUSTABLE: ::DWORD = 0x00000020;
pub const CCS_NODIVIDER: ::DWORD = 0x00000040;
pub const CCS_VERT: ::DWORD = 0x00000080;
pub const CCS_LEFT: ::DWORD = CCS_VERT | CCS_TOP;
pub const CCS_RIGHT: ::DWORD = CCS_VERT | CCS_BOTTOM;
pub const CCS_NOMOVEX: ::DWORD = CCS_VERT | CCS_NOMOVEY;
pub const MAX_LINKID_TEXT: usize = 48;
pub const L_MAX_URL_LENGTH: usize = 2048 + 32 + 4;
pub const LWS_TRANSPARENT: ::DWORD = 0x0001;
pub const LWS_IGNORERETURN: ::DWORD = 0x0002;
pub const LWS_NOPREFIX: ::DWORD = 0x0004;
pub const LWS_USEVISUALSTYLE: ::DWORD = 0x0008;
pub const LWS_USECUSTOMTEXT: ::DWORD = 0x0010;
pub const LWS_RIGHT: ::DWORD = 0x0020;
pub const LIF_ITEMINDEX: ::UINT = 0x00000001;
pub const LIF_STATE: ::UINT = 0x00000002;
pub const LIF_ITEMID: ::UINT = 0x00000004;
pub const LIF_URL: ::UINT = 0x00000008;
pub const LIS_FOCUSED: ::UINT = 0x00000001;
pub const LIS_ENABLED: ::UINT = 0x00000002;
pub const LIS_VISITED: ::UINT = 0x00000004;
pub const LIS_HOTTRACK: ::UINT = 0x00000008;
pub const LIS_DEFAULTCOLORS: ::UINT = 0x00000010;
#[repr(C)] #[derive(Copy)]
pub struct LITEM {
    pub mask: ::UINT,
    pub iLink: ::c_int,
    pub state: ::UINT,
    pub stateMask: ::UINT,
    pub szID: [::WCHAR; MAX_LINKID_TEXT],
    pub szUrl: [::WCHAR; L_MAX_URL_LENGTH],
}
impl Clone for LITEM { fn clone(&self) -> LITEM { *self } }
pub type PLITEM = *mut LITEM;
#[repr(C)] #[derive(Copy)]
pub struct LHITTESTINFO {
    pub pt: ::POINT,
    pub item: LITEM,
}
impl Clone for LHITTESTINFO { fn clone(&self) -> LHITTESTINFO { *self } }
pub type PLHITTESTINFO = *mut LHITTESTINFO;
#[repr(C)] #[derive(Copy)]
pub struct NMLINK {
    pub hdr: ::NMHDR,
    pub item: LITEM,
}
impl Clone for NMLINK { fn clone(&self) -> NMLINK { *self } }
pub type PNMLINK = *mut NMLINK;
pub const LM_HITTEST: ::UINT = ::WM_USER + 0x300;
pub const LM_GETIDEALHEIGHT: ::UINT = ::WM_USER + 0x301;
pub const LM_SETITEM: ::UINT = ::WM_USER + 0x302;
pub const LM_GETITEM: ::UINT = ::WM_USER + 0x303;
pub const LM_GETIDEALSIZE: ::UINT = LM_GETIDEALHEIGHT;
pub const LVS_ICON: ::DWORD = 0x0000;
pub const LVS_REPORT: ::DWORD = 0x0001;
pub const LVS_SMALLICON: ::DWORD = 0x0002;
pub const LVS_LIST: ::DWORD = 0x0003;
pub const LVS_TYPEMASK: ::DWORD = 0x0003;
pub const LVS_SINGLESEL: ::DWORD = 0x0004;
pub const LVS_SHOWSELALWAYS: ::DWORD = 0x0008;
pub const LVS_SORTASCENDING: ::DWORD = 0x0010;
pub const LVS_SORTDESCENDING: ::DWORD = 0x0020;
pub const LVS_SHAREIMAGELISTS: ::DWORD = 0x0040;
pub const LVS_NOLABELWRAP: ::DWORD = 0x0080;
pub const LVS_AUTOARRANGE: ::DWORD = 0x0100;
pub const LVS_EDITLABELS: ::DWORD = 0x0200;
pub const LVS_OWNERDATA: ::DWORD = 0x1000;
pub const LVS_NOSCROLL: ::DWORD = 0x2000;
pub const LVS_TYPESTYLEMASK: ::DWORD = 0xfc00;
pub const LVS_ALIGNTOP: ::DWORD = 0x0000;
pub const LVS_ALIGNLEFT: ::DWORD = 0x0800;
pub const LVS_ALIGNMASK: ::DWORD = 0x0c00;
pub const LVS_OWNERDRAWFIXED: ::DWORD = 0x0400;
pub const LVS_NOCOLUMNHEADER: ::DWORD = 0x4000;
pub const LVS_NOSORTHEADER: ::DWORD = 0x8000;
pub const LVM_SETUNICODEFORMAT: ::UINT = CCM_SETUNICODEFORMAT;
pub const LVM_GETUNICODEFORMAT: ::UINT = CCM_GETUNICODEFORMAT;
pub const LVM_GETBKCOLOR: ::UINT = LVM_FIRST + 0;
pub const LVM_SETBKCOLOR: ::UINT = LVM_FIRST + 1;
pub const LVM_GETIMAGELIST: ::UINT = LVM_FIRST + 2;
pub const LVM_SETIMAGELIST: ::UINT = LVM_FIRST + 3;
pub const LVM_GETITEMCOUNT: ::UINT = LVM_FIRST + 4;
pub const LVSIL_NORMAL: ::c_int = 0;
pub const LVSIL_SMALL: ::c_int = 1;
pub const LVSIL_STATE: ::c_int = 2;
pub const LVSIL_GROUPHEADER: ::c_int = 3;
pub const LVIF_TEXT: ::UINT = 0x00000001;
pub const LVIF_IMAGE: ::UINT = 0x00000002;
pub const LVIF_PARAM: ::UINT = 0x00000004;
pub const LVIF_STATE: ::UINT = 0x00000008;
pub const LVIF_INDENT: ::UINT = 0x00000010;
pub const LVIF_NORECOMPUTE: ::UINT = 0x00000800;
pub const LVIF_GROUPID: ::UINT = 0x00000100;
pub const LVIF_COLUMNS: ::UINT = 0x00000200;
pub const LVIF_COLFMT: ::UINT = 0x00010000;
pub const LVIS_FOCUSED: ::UINT = 0x0001;
pub const LVIS_SELECTED: ::UINT = 0x0002;
pub const LVIS_CUT: ::UINT = 0x0004;
pub const LVIS_DROPHILITED: ::UINT = 0x0008;
pub const LVIS_GLOW: ::UINT = 0x0010;
pub const LVIS_ACTIVATING: ::UINT = 0x0020;
pub const LVIS_OVERLAYMASK: ::UINT = 0x0F00;
pub const LVIS_STATEIMAGEMASK: ::UINT = 0xF000;
#[inline] #[allow(dead_code)]
pub fn INDEXTOSTATEIMAGEMASK(i: ::UINT) -> ::UINT { i << 12 }
pub const I_INDENTCALLBACK: ::c_int = -1;
pub type LV_ITEMA = LVITEMA;
pub type LV_ITEMW = LVITEMW;
pub const I_GROUPIDCALLBACK: ::c_int = -1;
pub const I_GROUPIDNONE: ::c_int = -2;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct LVITEMA {
    pub mask: ::UINT,
    pub iItem: ::c_int,
    pub iSubItem: ::c_int,
    pub state: ::UINT,
    pub stateMask: ::UINT,
    pub pszText: ::LPSTR,
    pub cchTextMax: ::c_int,
    pub iImage: ::c_int,
    pub lParam: ::LPARAM,
    pub iIndent: ::c_int,
    pub iGroupId: ::c_int,
    pub cColumns: ::UINT,
    pub puColumns: ::PUINT,
    pub piColFmt: *mut ::c_int,
    pub iGroup: ::c_int,
}
pub type LPLVITEMA = *mut LVITEMA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct LVITEMW {
    pub mask: ::UINT,
    pub iItem: ::c_int,
    pub iSubItem: ::c_int,
    pub state: ::UINT,
    pub stateMask: ::UINT,
    pub pszText: ::LPWSTR,
    pub cchTextMax: ::c_int,
    pub iImage: ::c_int,
    pub lParam: ::LPARAM,
    pub iIndent: ::c_int,
    pub iGroupId: ::c_int,
    pub cColumns: ::UINT,
    pub puColumns: ::PUINT,
    pub piColFmt: *mut ::c_int,
    pub iGroup: ::c_int,
}
pub type LPLVITEMW = *mut LVITEMW;
pub const LPSTR_TEXTCALLBACKW: ::LPWSTR = (0 - 1) as ::LPWSTR;
pub const LPSTR_TEXTCALLBACKA: ::LPSTR = (0 - 1) as ::LPSTR;
pub const I_IMAGECALLBACK: ::c_int = -1;
pub const I_IMAGENONE: ::c_int = -2;
pub const I_COLUMNSCALLBACK: ::UINT = 0 - 1;
pub const LVM_GETITEMA: ::UINT = LVM_FIRST + 5;
pub const LVM_GETITEMW: ::UINT = LVM_FIRST + 75;
pub const LVM_SETITEMA: ::UINT = LVM_FIRST + 6;
pub const LVM_SETITEMW: ::UINT = LVM_FIRST + 76;
pub const LVM_INSERTITEMA: ::UINT = LVM_FIRST + 7;
pub const LVM_INSERTITEMW: ::UINT = LVM_FIRST + 77;
pub const LVM_DELETEITEM: ::UINT = LVM_FIRST + 8;
pub const LVM_DELETEALLITEMS: ::UINT = LVM_FIRST + 9;
pub const LVM_GETCALLBACKMASK: ::UINT = LVM_FIRST + 10;
pub const LVM_SETCALLBACKMASK: ::UINT = LVM_FIRST + 11;
pub const LVM_GETNEXTITEM: ::UINT = LVM_FIRST + 12;
pub const LVNI_ALL: ::LPARAM = 0x0000;
pub const LVNI_FOCUSED: ::LPARAM = 0x0001;
pub const LVNI_SELECTED: ::LPARAM = 0x0002;
pub const LVNI_CUT: ::LPARAM = 0x0004;
pub const LVNI_DROPHILITED: ::LPARAM = 0x0008;
pub const LVNI_STATEMASK: ::LPARAM = LVNI_FOCUSED | LVNI_SELECTED | LVNI_CUT | LVNI_DROPHILITED;
pub const LVNI_VISIBLEORDER: ::LPARAM = 0x0010;
pub const LVNI_PREVIOUS: ::LPARAM = 0x0020;
pub const LVNI_VISIBLEONLY: ::LPARAM = 0x0040;
pub const LVNI_SAMEGROUPONLY: ::LPARAM = 0x0080;
pub const LVNI_ABOVE: ::LPARAM = 0x0100;
pub const LVNI_BELOW: ::LPARAM = 0x0200;
pub const LVNI_TOLEFT: ::LPARAM = 0x0400;
pub const LVNI_TORIGHT: ::LPARAM = 0x0800;
pub const LVNI_DIRECTIONMASK: ::LPARAM = LVNI_ABOVE | LVNI_BELOW | LVNI_TOLEFT | LVNI_TORIGHT;
pub const LVFI_PARAM: ::UINT = 0x0001;
pub const LVFI_STRING: ::UINT = 0x0002;
pub const LVFI_SUBSTRING: ::UINT = 0x0004;
pub const LVFI_PARTIAL: ::UINT = 0x0008;
pub const LVFI_WRAP: ::UINT = 0x0020;
pub const LVFI_NEARESTXY: ::UINT = 0x0040;
pub type LV_FINDINFOA = LVFINDINFOA;
pub type LV_FINDINFOW = LVFINDINFOW;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct LVFINDINFOA {
    pub flags: ::UINT,
    pub psz: ::LPCSTR,
    pub lParam: ::LPARAM,
    pub pt: ::POINT,
    pub vkDirection: ::UINT,
}
pub type LPLVFINDINFOA = *mut LVFINDINFOA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct LVFINDINFOW {
    pub flags: ::UINT,
    pub psz: ::LPCWSTR,
    pub lParam: ::LPARAM,
    pub pt: ::POINT,
    pub vkDirection: ::UINT,
}
pub type LPLVFINDINFOW = *mut LVFINDINFOW;
pub const LVM_FINDITEMA: ::UINT = LVM_FIRST + 13;
pub const LVM_FINDITEMW: ::UINT = LVM_FIRST + 83;
pub const LVIR_BOUNDS: ::c_int = 0;
pub const LVIR_ICON: ::c_int = 1;
pub const LVIR_LABEL: ::c_int = 2;
pub const LVIR_SELECTBOUNDS: ::c_int = 3;
pub const LVM_GETITEMRECT: ::UINT = LVM_FIRST + 14;
pub const LVM_SETITEMPOSITION: ::UINT = LVM_FIRST + 15;
pub const LVM_GETITEMPOSITION: ::UINT = LVM_FIRST + 16;
pub const LVM_GETSTRINGWIDTHA: ::UINT = LVM_FIRST + 17;
pub const LVM_GETSTRINGWIDTHW: ::UINT = LVM_FIRST + 87;
pub const LVHT_NOWHERE: ::UINT = 0x00000001;
pub const LVHT_ONITEMICON: ::UINT = 0x00000002;
pub const LVHT_ONITEMLABEL: ::UINT = 0x00000004;
pub const LVHT_ONITEMSTATEICON: ::UINT = 0x00000008;
pub const LVHT_ONITEM: ::UINT = LVHT_ONITEMICON | LVHT_ONITEMLABEL | LVHT_ONITEMSTATEICON;
pub const LVHT_ABOVE: ::UINT = 0x00000008;
pub const LVHT_BELOW: ::UINT = 0x00000010;
pub const LVHT_TORIGHT: ::UINT = 0x00000020;
pub const LVHT_TOLEFT: ::UINT = 0x00000040;
pub const LVHT_EX_GROUP_HEADER: ::UINT = 0x10000000;
pub const LVHT_EX_GROUP_FOOTER: ::UINT = 0x20000000;
pub const LVHT_EX_GROUP_COLLAPSE: ::UINT = 0x40000000;
pub const LVHT_EX_GROUP_BACKGROUND: ::UINT = 0x80000000;
pub const LVHT_EX_GROUP_STATEICON: ::UINT = 0x01000000;
pub const LVHT_EX_GROUP_SUBSETLINK: ::UINT = 0x02000000;
pub const LVHT_EX_GROUP: ::UINT = LVHT_EX_GROUP_BACKGROUND | LVHT_EX_GROUP_COLLAPSE
    | LVHT_EX_GROUP_FOOTER | LVHT_EX_GROUP_HEADER | LVHT_EX_GROUP_STATEICON
    | LVHT_EX_GROUP_SUBSETLINK;
pub const LVHT_EX_ONCONTENTS: ::UINT = 0x04000000;
pub const LVHT_EX_FOOTER: ::UINT = 0x08000000;
pub type LV_HITTESTINFO = LVHITTESTINFO;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct LVHITTESTINFO {
    pub pt: ::POINT,
    pub flags: ::UINT,
    pub iItem: ::c_int,
    pub iSubItem: ::c_int,
    pub iGroup: ::c_int,
}
pub type LPLVHITTESTINFO = *mut LVHITTESTINFO;
pub const LVM_HITTEST: ::UINT = LVM_FIRST + 18;
pub const LVM_ENSUREVISIBLE: ::UINT = LVM_FIRST + 19;
pub const LVM_SCROLL: ::UINT = LVM_FIRST + 20;
pub const LVM_REDRAWITEMS: ::UINT = LVM_FIRST + 21;
pub const LVA_DEFAULT: ::WPARAM = 0x0000;
pub const LVA_ALIGNLEFT: ::WPARAM = 0x0001;
pub const LVA_ALIGNTOP: ::WPARAM = 0x0002;
pub const LVA_SNAPTOGRID: ::WPARAM = 0x0005;
pub const LVM_ARRANGE: ::UINT = LVM_FIRST + 22;
pub const LVM_EDITLABELA: ::UINT = LVM_FIRST + 23;
pub const LVM_EDITLABELW: ::UINT = LVM_FIRST + 118;
pub const LVM_GETEDITCONTROL: ::UINT = LVM_FIRST + 24;
pub type LV_COLUMNA = LVCOLUMNA;
pub type LV_COLUMNW = LVCOLUMNW;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct LVCOLUMNA {
    pub mask: ::UINT,
    pub fmt: ::c_int,
    pub cx: ::c_int,
    pub pszText: ::LPSTR,
    pub cchTextMax: ::c_int,
    pub iSubItem: ::c_int,
    pub iImage: ::c_int,
    pub iOrder: ::c_int,
    pub cxMin: ::c_int,
    pub cxDefault: ::c_int,
    pub cxIdeal: ::c_int,
}
pub type LPLVCOLUMNA = *mut LVCOLUMNA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct LVCOLUMNW {
    pub mask: ::UINT,
    pub fmt: ::c_int,
    pub cx: ::c_int,
    pub pszText: ::LPWSTR,
    pub cchTextMax: ::c_int,
    pub iSubItem: ::c_int,
    pub iImage: ::c_int,
    pub iOrder: ::c_int,
    pub cxMin: ::c_int,
    pub cxDefault: ::c_int,
    pub cxIdeal: ::c_int,
}
pub type LPLVCOLUMNW = *mut LVCOLUMNW;
pub const LVCF_FMT: ::UINT = 0x0001;
pub const LVCF_WIDTH: ::UINT = 0x0002;
pub const LVCF_TEXT: ::UINT = 0x0004;
pub const LVCF_SUBITEM: ::UINT = 0x0008;
pub const LVCF_IMAGE: ::UINT = 0x0010;
pub const LVCF_ORDER: ::UINT = 0x0020;
pub const LVCF_MINWIDTH: ::UINT = 0x0040;
pub const LVCF_DEFAULTWIDTH: ::UINT = 0x0080;
pub const LVCF_IDEALWIDTH: ::UINT = 0x0100;
pub const LVCFMT_LEFT: ::c_int = 0x0000;
pub const LVCFMT_RIGHT: ::c_int = 0x0001;
pub const LVCFMT_CENTER: ::c_int = 0x0002;
pub const LVCFMT_JUSTIFYMASK: ::c_int = 0x0003;
pub const LVCFMT_IMAGE: ::c_int = 0x0800;
pub const LVCFMT_BITMAP_ON_RIGHT: ::c_int = 0x1000;
pub const LVCFMT_COL_HAS_IMAGES: ::c_int = 0x8000;
pub const LVCFMT_FIXED_WIDTH: ::c_int = 0x00100;
pub const LVCFMT_NO_DPI_SCALE: ::c_int = 0x40000;
pub const LVCFMT_FIXED_RATIO: ::c_int = 0x80000;
pub const LVCFMT_LINE_BREAK: ::c_int = 0x100000;
pub const LVCFMT_FILL: ::c_int = 0x200000;
pub const LVCFMT_WRAP: ::c_int = 0x400000;
pub const LVCFMT_NO_TITLE: ::c_int = 0x800000;
pub const LVCFMT_TILE_PLACEMENTMASK: ::c_int = LVCFMT_LINE_BREAK | LVCFMT_FILL;
pub const LVCFMT_SPLITBUTTON: ::c_int = 0x1000000;
pub const LVM_GETCOLUMNA: ::UINT = LVM_FIRST + 25;
pub const LVM_GETCOLUMNW: ::UINT = LVM_FIRST + 95;
pub const LVM_SETCOLUMNA: ::UINT = LVM_FIRST + 26;
pub const LVM_SETCOLUMNW: ::UINT = LVM_FIRST + 96;
pub const LVM_INSERTCOLUMNA: ::UINT = LVM_FIRST + 27;
pub const LVM_INSERTCOLUMNW: ::UINT = LVM_FIRST + 97;
pub const LVM_DELETECOLUMN: ::UINT = LVM_FIRST + 28;
pub const LVM_GETCOLUMNWIDTH: ::UINT = LVM_FIRST + 29;
pub const LVM_SETCOLUMNWIDTH: ::UINT = LVM_FIRST + 30;
pub const LVM_GETHEADER: ::UINT = LVM_FIRST + 31;
pub const LVM_CREATEDRAGIMAGE: ::UINT = LVM_FIRST + 33;
pub const LVM_GETVIEWRECT: ::UINT = LVM_FIRST + 34;
pub const LVM_GETTEXTCOLOR: ::UINT = LVM_FIRST + 35;
pub const LVM_SETTEXTCOLOR: ::UINT = LVM_FIRST + 36;
pub const LVM_GETTEXTBKCOLOR: ::UINT = LVM_FIRST + 37;
pub const LVM_SETTEXTBKCOLOR: ::UINT = LVM_FIRST + 38;
pub const LVM_GETTOPINDEX: ::UINT = LVM_FIRST + 39;
pub const LVM_GETCOUNTPERPAGE: ::UINT = LVM_FIRST + 40;
pub const LVM_GETORIGIN: ::UINT = LVM_FIRST + 41;
pub const LVM_UPDATE: ::UINT = LVM_FIRST + 42;
pub const LVM_SETITEMSTATE: ::UINT = LVM_FIRST + 43;
pub const LVM_GETITEMSTATE: ::UINT = LVM_FIRST + 44;
pub const LVM_GETITEMTEXTA: ::UINT = LVM_FIRST + 45;
pub const LVM_GETITEMTEXTW: ::UINT = LVM_FIRST + 115;
pub const LVM_SETITEMTEXTA: ::UINT = LVM_FIRST + 46;
pub const LVM_SETITEMTEXTW: ::UINT = LVM_FIRST + 116;
pub const LVM_SETITEMCOUNT: ::UINT = LVM_FIRST + 47;
pub const LVM_SORTITEMS: ::UINT = LVM_FIRST + 48;
pub const LVM_SETITEMPOSITION32: ::UINT = LVM_FIRST + 49;
pub const LVM_GETSELECTEDCOUNT: ::UINT = LVM_FIRST + 50;
pub const LVM_GETITEMSPACING: ::UINT = LVM_FIRST + 51;
pub const LVM_GETISEARCHSTRINGA: ::UINT = LVM_FIRST + 52;
pub const LVM_GETISEARCHSTRINGW: ::UINT = LVM_FIRST + 117;
pub const LVM_SETICONSPACING: ::UINT = LVM_FIRST + 53;
pub const LVM_SETEXTENDEDLISTVIEWSTYLE: ::UINT = LVM_FIRST + 54;
pub const LVM_GETEXTENDEDLISTVIEWSTYLE: ::UINT = LVM_FIRST + 55;
pub const LVSICF_NOINVALIDATEALL: ::LPARAM = 0x00000001;
pub const LVSICF_NOSCROLL: ::LPARAM = 0x00000002;
pub const LVS_EX_GRIDLINES: ::DWORD = 0x00000001;
pub const LVS_EX_SUBITEMIMAGES: ::DWORD = 0x00000002;
pub const LVS_EX_CHECKBOXES: ::DWORD = 0x00000004;
pub const LVS_EX_TRACKSELECT: ::DWORD = 0x00000008;
pub const LVS_EX_HEADERDRAGDROP: ::DWORD = 0x00000010;
pub const LVS_EX_FULLROWSELECT: ::DWORD = 0x00000020;
pub const LVS_EX_ONECLICKACTIVATE: ::DWORD = 0x00000040;
pub const LVS_EX_TWOCLICKACTIVATE: ::DWORD = 0x00000080;
pub const LVS_EX_FLATSB: ::DWORD = 0x00000100;
pub const LVS_EX_REGIONAL: ::DWORD = 0x00000200;
pub const LVS_EX_INFOTIP: ::DWORD = 0x00000400;
pub const LVS_EX_UNDERLINEHOT: ::DWORD = 0x00000800;
pub const LVS_EX_UNDERLINECOLD: ::DWORD = 0x00001000;
pub const LVS_EX_MULTIWORKAREAS: ::DWORD = 0x00002000;
pub const LVS_EX_LABELTIP: ::DWORD = 0x00004000;
pub const LVS_EX_BORDERSELECT: ::DWORD = 0x00008000;
pub const LVS_EX_DOUBLEBUFFER: ::DWORD = 0x00010000;
pub const LVS_EX_HIDELABELS: ::DWORD = 0x00020000;
pub const LVS_EX_SINGLEROW: ::DWORD = 0x00040000;
pub const LVS_EX_SNAPTOGRID: ::DWORD = 0x00080000;
pub const LVS_EX_SIMPLESELECT: ::DWORD = 0x00100000;
pub const LVS_EX_JUSTIFYCOLUMNS: ::DWORD = 0x00200000;
pub const LVS_EX_TRANSPARENTBKGND: ::DWORD = 0x00400000;
pub const LVS_EX_TRANSPARENTSHADOWTEXT: ::DWORD = 0x00800000;
pub const LVS_EX_AUTOAUTOARRANGE: ::DWORD = 0x01000000;
pub const LVS_EX_HEADERINALLVIEWS: ::DWORD = 0x02000000;
pub const LVS_EX_AUTOCHECKSELECT: ::DWORD = 0x08000000;
pub const LVS_EX_AUTOSIZECOLUMNS: ::DWORD = 0x10000000;
pub const LVS_EX_COLUMNSNAPPOINTS: ::DWORD = 0x40000000;
pub const LVS_EX_COLUMNOVERFLOW: ::DWORD = 0x80000000;
pub const LVM_GETSUBITEMRECT: ::UINT = LVM_FIRST + 56;
pub const LVM_SUBITEMHITTEST: ::UINT = LVM_FIRST + 57;
pub const LVM_SETCOLUMNORDERARRAY: ::UINT = LVM_FIRST + 58;
pub const LVM_GETCOLUMNORDERARRAY: ::UINT = LVM_FIRST + 59;
pub const LVM_SETHOTITEM: ::UINT = LVM_FIRST + 60;
pub const LVM_GETHOTITEM: ::UINT = LVM_FIRST + 61;
pub const LVM_SETHOTCURSOR: ::UINT = LVM_FIRST + 62;
pub const LVM_GETHOTCURSOR: ::UINT = LVM_FIRST + 63;
pub const LVM_APPROXIMATEVIEWRECT: ::UINT = LVM_FIRST + 64;
pub const LV_MAX_WORKAREAS: ::WPARAM = 16;
pub const LVM_SETWORKAREAS: ::UINT = LVM_FIRST + 65;
pub const LVM_GETWORKAREAS: ::UINT = LVM_FIRST + 70;
pub const LVM_GETNUMBEROFWORKAREAS: ::UINT = LVM_FIRST + 73;
pub const LVM_GETSELECTIONMARK: ::UINT = LVM_FIRST + 66;
pub const LVM_SETSELECTIONMARK: ::UINT = LVM_FIRST + 67;
pub const LVM_SETHOVERTIME: ::UINT = LVM_FIRST + 71;
pub const LVM_GETHOVERTIME: ::UINT = LVM_FIRST + 72;
pub const LVM_SETTOOLTIPS: ::UINT = LVM_FIRST + 74;
pub const LVM_GETTOOLTIPS: ::UINT = LVM_FIRST + 78;
pub const LVM_SORTITEMSEX: ::UINT = LVM_FIRST + 81;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct LVBKIMAGEA {
    pub ulFlags: ::ULONG,
    pub hbm: ::HBITMAP,
    pub pszImage: ::LPSTR,
    pub cchImageMax: ::UINT,
    pub xOffsetPercent: ::c_int,
    pub yOffsetPercent: ::c_int,
}
pub type LPLVBKIMAGEA = *mut LVBKIMAGEA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct LVBKIMAGEW {
    pub ulFlags: ::ULONG,
    pub hbm: ::HBITMAP,
    pub pszImage: ::LPWSTR,
    pub cchImageMax: ::UINT,
    pub xOffsetPercent: ::c_int,
    pub yOffsetPercent: ::c_int,
}
pub type LPLVBKIMAGEW = *mut LVBKIMAGEW;
pub const LVBKIF_SOURCE_NONE: ::ULONG = 0x00000000;
pub const LVBKIF_SOURCE_HBITMAP: ::ULONG = 0x00000001;
pub const LVBKIF_SOURCE_URL: ::ULONG = 0x00000002;
pub const LVBKIF_SOURCE_MASK: ::ULONG = 0x00000003;
pub const LVBKIF_STYLE_NORMAL: ::ULONG = 0x00000000;
pub const LVBKIF_STYLE_TILE: ::ULONG = 0x00000010;
pub const LVBKIF_STYLE_MASK: ::ULONG = 0x00000010;
pub const LVBKIF_FLAG_TILEOFFSET: ::ULONG = 0x00000100;
pub const LVBKIF_TYPE_WATERMARK: ::ULONG = 0x10000000;
pub const LVBKIF_FLAG_ALPHABLEND: ::ULONG = 0x20000000;
pub const LVM_SETBKIMAGEA: ::UINT = LVM_FIRST + 68;
pub const LVM_SETBKIMAGEW: ::UINT = LVM_FIRST + 138;
pub const LVM_GETBKIMAGEA: ::UINT = LVM_FIRST + 69;
pub const LVM_GETBKIMAGEW: ::UINT = LVM_FIRST + 139;
pub const LVM_SETSELECTEDCOLUMN: ::UINT = LVM_FIRST + 140;
pub const LV_VIEW_ICON: ::DWORD = 0x0000;
pub const LV_VIEW_DETAILS: ::DWORD = 0x0001;
pub const LV_VIEW_SMALLICON: ::DWORD = 0x0002;
pub const LV_VIEW_LIST: ::DWORD = 0x0003;
pub const LV_VIEW_TILE: ::DWORD = 0x0004;
pub const LV_VIEW_MAX: ::DWORD = 0x0004;
pub const LVM_SETVIEW: ::UINT = LVM_FIRST + 142;
pub const LVM_GETVIEW: ::UINT = LVM_FIRST + 143;
pub const LVGF_NONE: ::UINT = 0x00000000;
pub const LVGF_HEADER: ::UINT = 0x00000001;
pub const LVGF_FOOTER: ::UINT = 0x00000002;
pub const LVGF_STATE: ::UINT = 0x00000004;
pub const LVGF_ALIGN: ::UINT = 0x00000008;
pub const LVGF_GROUPID: ::UINT = 0x00000010;
pub const LVGF_SUBTITLE: ::UINT = 0x00000100;
pub const LVGF_TASK: ::UINT = 0x00000200;
pub const LVGF_DESCRIPTIONTOP: ::UINT = 0x00000400;
pub const LVGF_DESCRIPTIONBOTTOM: ::UINT = 0x00000800;
pub const LVGF_TITLEIMAGE: ::UINT = 0x00001000;
pub const LVGF_EXTENDEDIMAGE: ::UINT = 0x00002000;
pub const LVGF_ITEMS: ::UINT = 0x00004000;
pub const LVGF_SUBSET: ::UINT = 0x00008000;
pub const LVGF_SUBSETITEMS: ::UINT = 0x00010000;
pub const LVGS_NORMAL: ::UINT = 0x00000000;
pub const LVGS_COLLAPSED: ::UINT = 0x00000001;
pub const LVGS_HIDDEN: ::UINT = 0x00000002;
pub const LVGS_NOHEADER: ::UINT = 0x00000004;
pub const LVGS_COLLAPSIBLE: ::UINT = 0x00000008;
pub const LVGS_FOCUSED: ::UINT = 0x00000010;
pub const LVGS_SELECTED: ::UINT = 0x00000020;
pub const LVGS_SUBSETED: ::UINT = 0x00000040;
pub const LVGS_SUBSETLINKFOCUSED: ::UINT = 0x00000080;
pub const LVGA_HEADER_LEFT: ::UINT = 0x00000001;
pub const LVGA_HEADER_CENTER: ::UINT = 0x00000002;
pub const LVGA_HEADER_RIGHT: ::UINT = 0x00000004;
pub const LVGA_FOOTER_LEFT: ::UINT = 0x00000008;
pub const LVGA_FOOTER_CENTER: ::UINT = 0x00000010;
pub const LVGA_FOOTER_RIGHT: ::UINT = 0x00000020;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct LVGROUP {
    pub cbSize: ::UINT,
    pub mask: ::UINT,
    pub pszHeader: ::LPWSTR,
    pub cchHeader: ::c_int,
    pub pszFooter: ::LPWSTR,
    pub cchFooter: ::c_int,
    pub iGroupId: ::c_int,
    pub stateMask: ::UINT,
    pub state: ::UINT,
    pub uAlign: ::UINT,
    pub pszSubtitle: ::LPWSTR,
    pub cchSubtitle: ::UINT,
    pub pszTask: ::LPWSTR,
    pub cchTask: ::UINT,
    pub pszDescriptionTop: ::LPWSTR,
    pub cchDescriptionTop: ::UINT,
    pub pszDescriptionBottom: ::LPWSTR,
    pub cchDescriptionBottom: ::UINT,
    pub iTitleImage: ::c_int,
    pub iExtendedImage: ::c_int,
    pub iFirstItem: ::c_int,
    pub cItems: ::UINT,
    pub pszSubsetTitle: ::LPWSTR,
    pub cchSubsetTitle: ::UINT,
}
pub type PLVGROUP = *mut LVGROUP;
pub const LVM_INSERTGROUP: ::UINT = LVM_FIRST + 145;
pub const LVM_SETGROUPINFO: ::UINT = LVM_FIRST + 147;
pub const LVM_GETGROUPINFO: ::UINT = LVM_FIRST + 149;
pub const LVM_REMOVEGROUP: ::UINT = LVM_FIRST + 150;
pub const LVM_MOVEGROUP: ::UINT = LVM_FIRST + 151;
pub const LVM_GETGROUPCOUNT: ::UINT = LVM_FIRST + 152;
pub const LVM_GETGROUPINFOBYINDEX: ::UINT = LVM_FIRST + 153;
pub const LVM_MOVEITEMTOGROUP: ::UINT = LVM_FIRST + 154;
pub const LVM_GETGROUPRECT: ::UINT = LVM_FIRST + 98;
pub const LVGGR_GROUP: ::LPARAM = 0;
pub const LVGGR_HEADER: ::LPARAM = 1;
pub const LVGGR_LABEL: ::LPARAM = 2;
pub const LVGGR_SUBSETLINK: ::LPARAM = 3;
pub const LVGMF_NONE: ::UINT = 0x00000000;
pub const LVGMF_BORDERSIZE: ::UINT = 0x00000001;
pub const LVGMF_BORDERCOLOR: ::UINT = 0x00000002;
pub const LVGMF_TEXTCOLOR: ::UINT = 0x00000004;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct LVGROUPMETRICS {
    pub cbSize: ::UINT,
    pub mask: ::UINT,
    pub Left: ::UINT,
    pub Top: ::UINT,
    pub Right: ::UINT,
    pub Bottom: ::UINT,
    pub crLeft: ::COLORREF,
    pub crTop: ::COLORREF,
    pub crRight: ::COLORREF,
    pub crBottom: ::COLORREF,
    pub crHeader: ::COLORREF,
    pub crFooter: ::COLORREF,
}
pub type PLVGROUPMETRICS = *mut LVGROUPMETRICS;
pub const LVM_SETGROUPMETRICS: ::UINT = LVM_FIRST + 155;
pub const LVM_GETGROUPMETRICS: ::UINT = LVM_FIRST + 156;
pub const LVM_ENABLEGROUPVIEW: ::UINT = LVM_FIRST + 157;
pub const LVM_SORTGROUPS: ::UINT = LVM_FIRST + 158;
pub type PFNLVGROUPCOMPARE = Option<unsafe extern "system" fn(
    ::c_int, ::c_int, *mut ::c_void,
) -> ::c_int>;
#[repr(C)] #[derive(Copy)]
pub struct LVINSERTGROUPSORTED {
    pub pfnGroupCompare: PFNLVGROUPCOMPARE,
    pub pvData: *mut ::c_void,
    pub lvGroup: LVGROUP,
}
impl Clone for LVINSERTGROUPSORTED { fn clone(&self) -> LVINSERTGROUPSORTED { *self } }
pub type PLVINSERTGROUPSORTED = *mut LVINSERTGROUPSORTED;
pub const LVM_INSERTGROUPSORTED: ::UINT = LVM_FIRST + 159;
pub const LVM_REMOVEALLGROUPS: ::UINT = LVM_FIRST + 160;
pub const LVM_HASGROUP: ::UINT = LVM_FIRST + 161;
pub const LVM_GETGROUPSTATE: ::UINT = LVM_FIRST + 92;
pub const LVM_GETFOCUSEDGROUP: ::UINT = LVM_FIRST + 93;
pub const LVTVIF_AUTOSIZE: ::DWORD = 0x00000000;
pub const LVTVIF_FIXEDWIDTH: ::DWORD = 0x00000001;
pub const LVTVIF_FIXEDHEIGHT: ::DWORD = 0x00000002;
pub const LVTVIF_FIXEDSIZE: ::DWORD = 0x00000003;
pub const LVTVIF_EXTENDED: ::DWORD = 0x00000004;
pub const LVTVIM_TILESIZE: ::DWORD = 0x00000001;
pub const LVTVIM_COLUMNS: ::DWORD = 0x00000002;
pub const LVTVIM_LABELMARGIN: ::DWORD = 0x00000004;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct LVTILEVIEWINFO {
    pub cbSize: ::UINT,
    pub dwMask: ::DWORD,
    pub dwFlags: ::DWORD,
    pub sizeTile: ::SIZE,
    pub cLines: ::c_int,
    pub rcLabelMargin: ::RECT,
}
pub type PLVTILEVIEWINFO = *mut LVTILEVIEWINFO;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct LVTILEINFO {
    pub cbSize: ::UINT,
    pub iItem: ::c_int,
    pub cColumns: ::UINT,
    pub puColumns: ::PUINT,
    pub piColFmt: *mut ::c_int,
}
pub type PLVTILEINFO = *mut LVTILEINFO;
pub const LVM_SETTILEVIEWINFO: ::UINT = LVM_FIRST + 162;
pub const LVM_GETTILEVIEWINFO: ::UINT = LVM_FIRST + 163;
pub const LVM_SETTILEINFO: ::UINT = LVM_FIRST + 164;
pub const LVM_GETTILEINFO: ::UINT = LVM_FIRST + 165;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct LVINSERTMARK {
    pub cbSize: ::UINT,
    pub dwFlags: ::DWORD,
    pub iItem: ::c_int,
    pub dwReserved: ::DWORD,
}
pub type LPLVINSERTMARK = *mut LVINSERTMARK;
pub const LVIM_AFTER: ::DWORD = 0x00000001;
pub const LVM_SETINSERTMARK: ::UINT = LVM_FIRST + 166;
pub const LVM_GETINSERTMARK: ::UINT = LVM_FIRST + 167;
pub const LVM_INSERTMARKHITTEST: ::UINT = LVM_FIRST + 168;
pub const LVM_GETINSERTMARKRECT: ::UINT = LVM_FIRST + 169;
pub const LVM_SETINSERTMARKCOLOR: ::UINT = LVM_FIRST + 170;
pub const LVM_GETINSERTMARKCOLOR: ::UINT = LVM_FIRST + 171;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct LVSETINFOTIP {
    pub cbSize: ::UINT,
    pub dwFlags: ::DWORD,
    pub pszText: ::LPWSTR,
    pub iItem: ::c_int,
    pub iSubItem: ::c_int,
}
pub type PLVSETINFOTIP = *mut LVSETINFOTIP;
pub const  LVM_SETINFOTIP: ::UINT = LVM_FIRST + 173;
pub const LVM_GETSELECTEDCOLUMN: ::UINT = LVM_FIRST + 174;
pub const LVM_ISGROUPVIEWENABLED: ::UINT = LVM_FIRST + 175;
pub const LVM_GETOUTLINECOLOR: ::UINT = LVM_FIRST + 176;
pub const LVM_SETOUTLINECOLOR: ::UINT = LVM_FIRST + 177;
pub const LVM_CANCELEDITLABEL: ::UINT = LVM_FIRST + 179;
pub const LVM_MAPINDEXTOID: ::UINT = LVM_FIRST + 180;
pub const LVM_MAPIDTOINDEX: ::UINT = LVM_FIRST + 181;
pub const LVM_ISITEMVISIBLE: ::UINT = LVM_FIRST + 182;
pub const LVM_GETEMPTYTEXT: ::UINT = LVM_FIRST + 204;
pub const LVM_GETFOOTERRECT: ::UINT = LVM_FIRST + 205;
pub const LVFF_ITEMCOUNT: ::UINT = 0x00000001;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct LVFOOTERINFO {
    pub mask: ::UINT,
    pub pszText: ::LPWSTR,
    pub cchTextMax: ::c_int,
    pub cItems: ::UINT,
}
pub type LPLVFOOTERINFO = *mut LVFOOTERINFO;
pub const LVM_GETFOOTERINFO: ::UINT = LVM_FIRST + 206;
pub const LVM_GETFOOTERITEMRECT: ::UINT = LVM_FIRST + 207;
pub const LVFIF_TEXT: ::UINT = 0x00000001;
pub const LVFIF_STATE: ::UINT = 0x00000002;
pub const LVFIS_FOCUSED: ::UINT = 0x0001;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct LVFOOTERITEM {
    pub mask: ::UINT,
    pub iItem: ::c_int,
    pub pszText: ::LPWSTR,
    pub cchTextMax: ::c_int,
    pub state: ::UINT,
    pub stateMask: ::UINT,
}
pub type LPLVFOOTERITEM = *mut LVFOOTERITEM;
pub const LVM_GETFOOTERITEM: ::UINT = LVM_FIRST + 208;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct LVITEMINDEX {
    pub iItem: ::c_int,
    pub iGroup: ::c_int,
}
pub type PLVITEMINDEX = *mut LVITEMINDEX;
pub const LVM_GETITEMINDEXRECT: ::UINT = LVM_FIRST + 209;
pub const LVM_SETITEMINDEXSTATE: ::UINT = LVM_FIRST + 210;
pub const LVM_GETNEXTITEMINDEX: ::UINT = LVM_FIRST + 211;
pub type LPNM_LISTVIEW = LPNMLISTVIEW;
pub type NM_LISTVIEW = NMLISTVIEW;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMLISTVIEW {
    pub hdr: ::NMHDR,
    pub iItem: ::c_int,
    pub iSubItem: ::c_int,
    pub uNewState: ::UINT,
    pub uOldState: ::UINT,
    pub uChanged: ::UINT,
    pub ptAction: ::POINT,
    pub lParam: ::LPARAM,
}
pub type LPNMLISTVIEW = *mut NMLISTVIEW;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMITEMACTIVATE {
    pub hdr: ::NMHDR,
    pub iItem: ::c_int,
    pub iSubItem: ::c_int,
    pub uNewState: ::UINT,
    pub uOldState: ::UINT,
    pub uChanged: ::UINT,
    pub ptAction: ::POINT,
    pub lParam: ::LPARAM,
    pub uKeyFlags: ::UINT,
}
pub type LPNMITEMACTIVATE = *mut NMITEMACTIVATE;
pub const LVKF_ALT: ::UINT = 0x0001;
pub const LVKF_CONTROL: ::UINT = 0x0002;
pub const LVKF_SHIFT: ::UINT = 0x0004;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMLVCUSTOMDRAW {
    pub nmcd: NMCUSTOMDRAW,
    pub clrText: ::COLORREF,
    pub clrTextBk: ::COLORREF,
    pub iSubItem: ::c_int,
    pub dwItemType: ::DWORD,
    pub clrFace: ::COLORREF,
    pub iIconEffect: ::c_int,
    pub iIconPhase: ::c_int,
    pub iPartId: ::c_int,
    pub iStateId: ::c_int,
    pub rcText: ::RECT,
    pub uAlign: ::UINT,
}
pub type LPNMLVCUSTOMDRAW = *mut NMLVCUSTOMDRAW;
pub const LVCDI_ITEM: ::DWORD = 0x00000000;
pub const LVCDI_GROUP: ::DWORD = 0x00000001;
pub const LVCDI_ITEMSLIST: ::DWORD = 0x00000002;
pub const LVCDRF_NOSELECT: ::LRESULT = 0x00010000;
pub const LVCDRF_NOGROUPFRAME: ::LRESULT = 0x00020000;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMLVCACHEHINT {
    pub hdr: ::NMHDR,
    pub iFrom: ::c_int,
    pub iTo: ::c_int,
}
pub type LPNMLVCACHEHINT = *mut NMLVCACHEHINT;
pub type LPNM_CACHEHINT = LPNMLVCACHEHINT;
pub type PNM_CACHEHINT = LPNMLVCACHEHINT;
pub type NM_CACHEHINT = NMLVCACHEHINT;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMLVFINDITEMA {
    pub hdr: ::NMHDR,
    pub iStart: ::c_int,
    pub lvfi: LVFINDINFOA,
}
pub type LPNMLVFINDITEMA = *mut NMLVFINDITEMA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMLVFINDITEMW {
    pub hdr: ::NMHDR,
    pub iStart: ::c_int,
    pub lvfi: LVFINDINFOW,
}
pub type LPNMLVFINDITEMW = *mut NMLVFINDITEMW;
pub type PNM_FINDITEMA = LPNMLVFINDITEMA;
pub type LPNM_FINDITEMA = LPNMLVFINDITEMA;
pub type NM_FINDITEMA = NMLVFINDITEMA;
pub type PNM_FINDITEMW = LPNMLVFINDITEMW;
pub type LPNM_FINDITEMW = LPNMLVFINDITEMW;
pub type NM_FINDITEMW = NMLVFINDITEMW;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMLVODSTATECHANGE {
    pub hdr: ::NMHDR,
    pub iFrom: ::c_int,
    pub iTo: ::c_int,
    pub uNewState: ::UINT,
    pub uOldState: ::UINT,
}
pub type LPNMLVODSTATECHANGE = *mut NMLVODSTATECHANGE;
pub type PNM_ODSTATECHANGE = LPNMLVODSTATECHANGE;
pub type LPNM_ODSTATECHANGE = LPNMLVODSTATECHANGE;
pub type NM_ODSTATECHANGE = NMLVODSTATECHANGE;
pub const LVN_ITEMCHANGING: ::UINT = LVN_FIRST - 0;
pub const LVN_ITEMCHANGED: ::UINT = LVN_FIRST - 1;
pub const LVN_INSERTITEM: ::UINT = LVN_FIRST - 2;
pub const LVN_DELETEITEM: ::UINT = LVN_FIRST - 3;
pub const LVN_DELETEALLITEMS: ::UINT = LVN_FIRST - 4;
pub const LVN_BEGINLABELEDITA: ::UINT = LVN_FIRST - 5;
pub const LVN_BEGINLABELEDITW: ::UINT = LVN_FIRST - 75;
pub const LVN_ENDLABELEDITA: ::UINT = LVN_FIRST - 6;
pub const LVN_ENDLABELEDITW: ::UINT = LVN_FIRST - 76;
pub const LVN_COLUMNCLICK: ::UINT = LVN_FIRST - 8;
pub const LVN_BEGINDRAG: ::UINT = LVN_FIRST - 9;
pub const LVN_BEGINRDRAG: ::UINT = LVN_FIRST - 11;
pub const LVN_ODCACHEHINT: ::UINT = LVN_FIRST - 13;
pub const LVN_ODFINDITEMA: ::UINT = LVN_FIRST - 52;
pub const LVN_ODFINDITEMW: ::UINT = LVN_FIRST - 79;
pub const LVN_ITEMACTIVATE: ::UINT = LVN_FIRST - 14;
pub const LVN_ODSTATECHANGED: ::UINT = LVN_FIRST - 15;
pub const LVN_HOTTRACK: ::UINT = LVN_FIRST - 21;
pub const LVN_GETDISPINFOA: ::UINT = LVN_FIRST - 50;
pub const LVN_GETDISPINFOW: ::UINT = LVN_FIRST - 77;
pub const LVN_SETDISPINFOA: ::UINT = LVN_FIRST - 51;
pub const LVN_SETDISPINFOW: ::UINT = LVN_FIRST - 78;
pub const LVIF_DI_SETITEM: ::UINT = 0x1000;
pub type LV_DISPINFOA = NMLVDISPINFOA;
pub type LV_DISPINFOW = NMLVDISPINFOW;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMLVDISPINFOA {
    pub hdr: ::NMHDR,
    pub item: LVITEMA,
}
pub type LPNMLVDISPINFOA = *mut NMLVDISPINFOA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMLVDISPINFOW {
    pub hdr: ::NMHDR,
    pub item: LVITEMW,
}
pub type LPNMLVDISPINFOW = *mut NMLVDISPINFOW;
pub const LVN_KEYDOWN: ::UINT = LVN_FIRST - 55;
pub type LV_KEYDOWN = NMLVKEYDOWN;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMLVKEYDOWN {
    pub hdr: ::NMHDR,
    pub wVKey: ::WORD,
    pub flags: ::UINT,
}
pub type LPNMLVKEYDOWN = *mut NMLVKEYDOWN;
pub const LVN_MARQUEEBEGIN: ::UINT = LVN_FIRST - 56;
#[repr(C)] #[derive(Clone, Copy)]
pub struct NMLVLINK {
    pub hdr: ::NMHDR,
    pub link: LITEM,
    pub iItem: ::c_int,
    pub iSubItem: ::c_int,
}
pub type PNMLVLINK = *mut NMLVLINK;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMLVGETINFOTIPA {
    pub hdr: ::NMHDR,
    pub dwFlags: ::DWORD,
    pub pszText: ::LPSTR,
    pub cchTextMax: ::c_int,
    pub iItem: ::c_int,
    pub iSubItem: ::c_int,
    pub lParam: ::LPARAM,
}
pub type LPNMLVGETINFOTIPA = *mut NMLVGETINFOTIPA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMLVGETINFOTIPW {
    pub hdr: ::NMHDR,
    pub dwFlags: ::DWORD,
    pub pszText: ::LPWSTR,
    pub cchTextMax: ::c_int,
    pub iItem: ::c_int,
    pub iSubItem: ::c_int,
    pub lParam: ::LPARAM,
}
pub type LPNMLVGETINFOTIPW = *mut NMLVGETINFOTIPW;
pub const LVGIT_UNFOLDED: ::DWORD = 0x0001;
pub const LVN_GETINFOTIPA: ::UINT = LVN_FIRST - 57;
pub const LVN_GETINFOTIPW: ::UINT = LVN_FIRST - 58;
pub const LVNSCH_DEFAULT: ::LPARAM = -1;
pub const LVNSCH_ERROR: ::LPARAM = -2;
pub const LVNSCH_IGNORE: ::LPARAM = -3;
pub const LVN_INCREMENTALSEARCHA: ::UINT = LVN_FIRST - 62;
pub const LVN_INCREMENTALSEARCHW: ::UINT = LVN_FIRST - 63;
pub const LVN_COLUMNDROPDOWN: ::UINT = LVN_FIRST - 64;
pub const LVN_COLUMNOVERFLOWCLICK: ::UINT = LVN_FIRST - 66;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMLVSCROLL {
    pub hdr: ::NMHDR,
    pub dx: ::c_int,
    pub dy: ::c_int,
}
pub type LPNMLVSCROLL = *mut NMLVSCROLL;
pub const LVN_BEGINSCROLL: ::UINT = LVN_FIRST - 80;
pub const LVN_ENDSCROLL: ::UINT = LVN_FIRST - 81;
pub const LVN_LINKCLICK: ::UINT = LVN_FIRST - 84;
pub const EMF_CENTERED: ::DWORD = 0x00000001;
#[repr(C)] #[derive(Copy)]
pub struct NMLVEMPTYMARKUP {
    pub hdr: ::NMHDR,
    pub dwFlags: ::DWORD,
    pub szMarkup: [::WCHAR; L_MAX_URL_LENGTH],
}
impl Clone for NMLVEMPTYMARKUP { fn clone(&self) -> NMLVEMPTYMARKUP { *self } }
pub const LVN_GETEMPTYMARKUP: ::UINT = LVN_FIRST - 87;
pub const TVS_HASBUTTONS: ::DWORD = 0x0001;
pub const TVS_HASLINES: ::DWORD = 0x0002;
pub const TVS_LINESATROOT: ::DWORD = 0x0004;
pub const TVS_EDITLABELS: ::DWORD = 0x0008;
pub const TVS_DISABLEDRAGDROP: ::DWORD = 0x0010;
pub const TVS_SHOWSELALWAYS: ::DWORD = 0x0020;
pub const TVS_RTLREADING: ::DWORD = 0x0040;
pub const TVS_NOTOOLTIPS: ::DWORD = 0x0080;
pub const TVS_CHECKBOXES: ::DWORD = 0x0100;
pub const TVS_TRACKSELECT: ::DWORD = 0x0200;
pub const TVS_SINGLEEXPAND: ::DWORD = 0x0400;
pub const TVS_INFOTIP: ::DWORD = 0x0800;
pub const TVS_FULLROWSELECT: ::DWORD = 0x1000;
pub const TVS_NOSCROLL: ::DWORD = 0x2000;
pub const TVS_NONEVENHEIGHT: ::DWORD = 0x4000;
pub const TVS_NOHSCROLL: ::DWORD = 0x8000;
pub const TVS_EX_NOSINGLECOLLAPSE: ::DWORD = 0x0001;
pub const TVS_EX_MULTISELECT: ::DWORD = 0x0002;
pub const TVS_EX_DOUBLEBUFFER: ::DWORD = 0x0004;
pub const TVS_EX_NOINDENTSTATE: ::DWORD = 0x0008;
pub const TVS_EX_RICHTOOLTIP: ::DWORD = 0x0010;
pub const TVS_EX_AUTOHSCROLL: ::DWORD = 0x0020;
pub const TVS_EX_FADEINOUTEXPANDOS: ::DWORD = 0x0040;
pub const TVS_EX_PARTIALCHECKBOXES: ::DWORD = 0x0080;
pub const TVS_EX_EXCLUSIONCHECKBOXES: ::DWORD = 0x0100;
pub const TVS_EX_DIMMEDCHECKBOXES: ::DWORD = 0x0200;
pub const TVS_EX_DRAWIMAGEASYNC: ::DWORD = 0x0400;
#[repr(C)] #[allow(missing_copy_implementations)]
pub struct TREEITEM {
    unused: ::c_void,
}
pub type HTREEITEM = *mut TREEITEM;
pub const TVIF_TEXT: ::UINT = 0x0001;
pub const TVIF_IMAGE: ::UINT = 0x0002;
pub const TVIF_PARAM: ::UINT = 0x0004;
pub const TVIF_STATE: ::UINT = 0x0008;
pub const TVIF_HANDLE: ::UINT = 0x0010;
pub const TVIF_SELECTEDIMAGE: ::UINT = 0x0020;
pub const TVIF_CHILDREN: ::UINT = 0x0040;
pub const TVIF_INTEGRAL: ::UINT = 0x0080;
pub const TVIF_STATEEX: ::UINT = 0x0100;
pub const TVIF_EXPANDEDIMAGE: ::UINT = 0x0200;
pub const TVIS_SELECTED: ::UINT = 0x0002;
pub const TVIS_CUT: ::UINT = 0x0004;
pub const TVIS_DROPHILITED: ::UINT = 0x0008;
pub const TVIS_BOLD: ::UINT = 0x0010;
pub const TVIS_EXPANDED: ::UINT = 0x0020;
pub const TVIS_EXPANDEDONCE: ::UINT = 0x0040;
pub const TVIS_EXPANDPARTIAL: ::UINT = 0x0080;
pub const TVIS_OVERLAYMASK: ::UINT = 0x0F00;
pub const TVIS_STATEIMAGEMASK: ::UINT = 0xF000;
pub const TVIS_USERMASK: ::UINT = 0xF000;
pub const TVIS_EX_FLAT: ::UINT = 0x0001;
pub const TVIS_EX_DISABLED: ::UINT = 0x0002;
pub const TVIS_EX_ALL: ::UINT = 0x0002;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMTVSTATEIMAGECHANGING {
    pub hdr: ::NMHDR,
    pub hti: HTREEITEM,
    pub iOldStateImageIndex: ::c_int,
    pub iNewStateImageIndex: ::c_int,
}
pub type LPNMTVSTATEIMAGECHANGING = *mut NMTVSTATEIMAGECHANGING;
pub const I_CHILDRENCALLBACK: ::c_int = -1;
pub const I_CHILDRENAUTO: ::c_int = -2;
pub type LPTV_ITEMW = LPTVITEMW;
pub type LPTV_ITEMA = LPTVITEMA;
pub type TV_ITEMW = TVITEMW;
pub type TV_ITEMA = TVITEMA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct TVITEMA {
    pub mask: ::UINT,
    pub hItem: HTREEITEM,
    pub state: ::UINT,
    pub stateMask: ::UINT,
    pub pszText: ::LPSTR,
    pub cchTextMax: ::c_int,
    pub iImage: ::c_int,
    pub iSelectedImage: ::c_int,
    pub cChildren: ::c_int,
    pub lParam: ::LPARAM,
}
pub type LPTVITEMA = *mut TVITEMA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct TVITEMW {
    pub mask: ::UINT,
    pub hItem: HTREEITEM,
    pub state: ::UINT,
    pub stateMask: ::UINT,
    pub pszText: ::LPWSTR,
    pub cchTextMax: ::c_int,
    pub iImage: ::c_int,
    pub iSelectedImage: ::c_int,
    pub cChildren: ::c_int,
    pub lParam: ::LPARAM,
}
pub type LPTVITEMW = *mut TVITEMW;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct TVITEMEXA {
    pub mask: ::UINT,
    pub hItem: HTREEITEM,
    pub state: ::UINT,
    pub stateMask: ::UINT,
    pub pszText: ::LPSTR,
    pub cchTextMax: ::c_int,
    pub iImage: ::c_int,
    pub iSelectedImage: ::c_int,
    pub cChildren: ::c_int,
    pub lParam: ::LPARAM,
    pub iIntegral: ::c_int,
    pub uStateEx: ::UINT,
    pub hwnd: ::HWND,
    pub iExpandedImage: ::c_int,
    pub iReserved: ::c_int,
}
pub type LPTVITEMEXA = *mut TVITEMEXA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct TVITEMEXW {
    pub mask: ::UINT,
    pub hItem: HTREEITEM,
    pub state: ::UINT,
    pub stateMask: ::UINT,
    pub pszText: ::LPWSTR,
    pub cchTextMax: ::c_int,
    pub iImage: ::c_int,
    pub iSelectedImage: ::c_int,
    pub cChildren: ::c_int,
    pub lParam: ::LPARAM,
    pub iIntegral: ::c_int,
    pub uStateEx: ::UINT,
    pub hwnd: ::HWND,
    pub iExpandedImage: ::c_int,
    pub iReserved: ::c_int,
}
pub type LPTVITEMEXW = *mut TVITEMEXW;
pub const TVI_ROOT: HTREEITEM = (0 - 0x10000) as HTREEITEM;
pub const TVI_FIRST: HTREEITEM = (0 - 0x0FFFF) as HTREEITEM;
pub const TVI_LAST: HTREEITEM = (0 - 0x0FFFE) as HTREEITEM;
pub const TVI_SORT: HTREEITEM = (0 - 0x0FFFD) as HTREEITEM;
pub type LPTV_INSERTSTRUCTA = LPTVINSERTSTRUCTA;
pub type LPTV_INSERTSTRUCTW = LPTVINSERTSTRUCTW;
pub type TV_INSERTSTRUCTA = TVINSERTSTRUCTA;
pub type TV_INSERTSTRUCTW = TVINSERTSTRUCTW;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct TVINSERTSTRUCTA {
    pub hParent: HTREEITEM,
    pub hInsertAfter: HTREEITEM,
    pub itemex: TVITEMEXA,
}
UNION!(TVINSERTSTRUCTA, itemex, item, item_mut, TV_ITEMA);
pub type LPTVINSERTSTRUCTA = *mut TVINSERTSTRUCTA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct TVINSERTSTRUCTW {
    pub hParent: HTREEITEM,
    pub hInsertAfter: HTREEITEM,
    pub itemex: TVITEMEXW,
}
UNION!(TVINSERTSTRUCTA, itemex, item, item_mut, TV_ITEMW);
pub type LPTVINSERTSTRUCTW = *mut TVINSERTSTRUCTW;
pub const TVM_INSERTITEMA: ::UINT = TV_FIRST + 0;
pub const TVM_INSERTITEMW: ::UINT = TV_FIRST + 50;
pub const TVM_DELETEITEM: ::UINT = TV_FIRST + 1;
pub const TVM_EXPAND: ::UINT = TV_FIRST + 2;
pub const TVM_GETITEMRECT: ::UINT = TV_FIRST + 4;
pub const TVE_COLLAPSE: ::WPARAM = 0x0001;
pub const TVE_EXPAND: ::WPARAM = 0x0002;
pub const TVE_TOGGLE: ::WPARAM = 0x0003;
pub const TVE_EXPANDPARTIAL: ::WPARAM = 0x4000;
pub const TVE_COLLAPSERESET: ::WPARAM = 0x8000;
pub const TVM_GETCOUNT: ::UINT = TV_FIRST + 5;
pub const TVM_GETINDENT: ::UINT = TV_FIRST + 6;
pub const TVM_SETINDENT: ::UINT = TV_FIRST + 7;
pub const TVM_GETIMAGELIST: ::UINT = TV_FIRST + 8;
pub const TVM_SETIMAGELIST: ::UINT = TV_FIRST + 9;
pub const TVM_GETNEXTITEM: ::UINT = TV_FIRST + 10;
pub const TVSIL_NORMAL: ::WPARAM = 0;
pub const TVSIL_STATE: ::WPARAM = 2;
pub const TVGN_ROOT: ::WPARAM = 0x0000;
pub const TVGN_NEXT: ::WPARAM = 0x0001;
pub const TVGN_PREVIOUS: ::WPARAM = 0x0002;
pub const TVGN_PARENT: ::WPARAM = 0x0003;
pub const TVGN_CHILD: ::WPARAM = 0x0004;
pub const TVGN_FIRSTVISIBLE: ::WPARAM = 0x0005;
pub const TVGN_NEXTVISIBLE: ::WPARAM = 0x0006;
pub const TVGN_PREVIOUSVISIBLE: ::WPARAM = 0x0007;
pub const TVGN_DROPHILITE: ::WPARAM = 0x0008;
pub const TVGN_CARET: ::WPARAM = 0x0009;
pub const TVGN_LASTVISIBLE: ::WPARAM = 0x000A;
pub const TVGN_NEXTSELECTED: ::WPARAM = 0x000B;
pub const TVSI_NOSINGLEEXPAND: ::WPARAM = 0x8000;
pub const TVM_SELECTITEM: ::UINT = TV_FIRST + 11;
pub const TVM_GETITEMA: ::UINT = TV_FIRST + 12;
pub const TVM_GETITEMW: ::UINT = TV_FIRST + 62;
pub const TVM_SETITEMA: ::UINT = TV_FIRST + 13;
pub const TVM_SETITEMW: ::UINT = TV_FIRST + 63;
pub const TVM_EDITLABELA: ::UINT = TV_FIRST + 14;
pub const TVM_EDITLABELW: ::UINT = TV_FIRST + 65;
pub const TVM_GETEDITCONTROL: ::UINT = TV_FIRST + 15;
pub const TVM_GETVISIBLECOUNT: ::UINT = TV_FIRST + 16;
pub const TVM_HITTEST: ::UINT = TV_FIRST + 17;
pub type LPTV_HITTESTINFO = LPTVHITTESTINFO;
pub type TV_HITTESTINFO = TVHITTESTINFO;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct TVHITTESTINFO {
    pub pt: ::POINT,
    pub flags: ::UINT,
    pub hItem: HTREEITEM,
}
pub type LPTVHITTESTINFO = *mut TVHITTESTINFO;
pub const TVHT_NOWHERE: ::UINT = 0x0001;
pub const TVHT_ONITEMICON: ::UINT = 0x0002;
pub const TVHT_ONITEMLABEL: ::UINT = 0x0004;
pub const TVHT_ONITEM: ::UINT = TVHT_ONITEMICON | TVHT_ONITEMLABEL | TVHT_ONITEMSTATEICON;
pub const TVHT_ONITEMINDENT: ::UINT = 0x0008;
pub const TVHT_ONITEMBUTTON: ::UINT = 0x0010;
pub const TVHT_ONITEMRIGHT: ::UINT = 0x0020;
pub const TVHT_ONITEMSTATEICON: ::UINT = 0x0040;
pub const TVHT_ABOVE: ::UINT = 0x0100;
pub const TVHT_BELOW: ::UINT = 0x0200;
pub const TVHT_TORIGHT: ::UINT = 0x0400;
pub const TVHT_TOLEFT: ::UINT = 0x0800;
pub const TVM_CREATEDRAGIMAGE: ::UINT = TV_FIRST + 18;
pub const TVM_SORTCHILDREN: ::UINT = TV_FIRST + 19;
pub const TVM_ENSUREVISIBLE: ::UINT = TV_FIRST + 20;
pub const TVM_SORTCHILDRENCB: ::UINT = TV_FIRST + 21;
pub const TVM_ENDEDITLABELNOW: ::UINT = TV_FIRST + 22;
pub const TVM_GETISEARCHSTRINGA: ::UINT = TV_FIRST + 23;
pub const TVM_GETISEARCHSTRINGW: ::UINT = TV_FIRST + 64;
pub const TVM_SETTOOLTIPS: ::UINT = TV_FIRST + 24;
pub const TVM_GETTOOLTIPS: ::UINT = TV_FIRST + 25;
pub const TVM_SETINSERTMARK: ::UINT = TV_FIRST + 26;
pub const TVM_SETUNICODEFORMAT: ::UINT = CCM_SETUNICODEFORMAT;
pub const TVM_GETUNICODEFORMAT: ::UINT = CCM_GETUNICODEFORMAT;
pub const TVM_SETITEMHEIGHT: ::UINT = TV_FIRST + 27;
pub const TVM_GETITEMHEIGHT: ::UINT = TV_FIRST + 28;
pub const TVM_SETBKCOLOR: ::UINT = TV_FIRST + 29;
pub const TVM_SETTEXTCOLOR: ::UINT = TV_FIRST + 30;
pub const TVM_GETBKCOLOR: ::UINT = TV_FIRST + 31;
pub const TVM_GETTEXTCOLOR: ::UINT = TV_FIRST + 32;
pub const TVM_SETSCROLLTIME: ::UINT = TV_FIRST + 33;
pub const TVM_GETSCROLLTIME: ::UINT = TV_FIRST + 34;
pub const TVM_SETINSERTMARKCOLOR: ::UINT = TV_FIRST + 37;
pub const TVM_GETINSERTMARKCOLOR: ::UINT = TV_FIRST + 38;
pub const TVM_SETBORDER: ::UINT = TV_FIRST + 35;
pub const TVSBF_XBORDER: ::WPARAM = 0x00000001;
pub const TVSBF_YBORDER: ::WPARAM = 0x00000002;
pub const TVM_GETITEMSTATE: ::UINT = TV_FIRST + 39;
pub const TVM_SETLINECOLOR: ::UINT = TV_FIRST + 40;
pub const TVM_GETLINECOLOR: ::UINT = TV_FIRST + 41;
pub const TVM_MAPACCIDTOHTREEITEM: ::UINT = TV_FIRST + 42;
pub const TVM_MAPHTREEITEMTOACCID: ::UINT = TV_FIRST + 43;
pub const TVM_SETEXTENDEDSTYLE: ::UINT = TV_FIRST + 44;
pub const TVM_GETEXTENDEDSTYLE: ::UINT = TV_FIRST + 45;
pub const TVM_SETAUTOSCROLLINFO: ::UINT = TV_FIRST + 59;
pub const TVM_SETHOT: ::UINT = TV_FIRST + 58;
pub const TVM_GETSELECTEDCOUNT: ::UINT = TV_FIRST + 70;
pub const TVM_SHOWINFOTIP: ::UINT = TV_FIRST + 71;
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum TVITEMPART {
    TVGIPR_BUTTON = 0x0001,
    DUMMY,
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct TVGETITEMPARTRECTINFO {
    pub hti: HTREEITEM,
    pub prc: *mut ::RECT,
    pub partID: TVITEMPART,
}
pub const TVM_GETITEMPARTRECT: ::UINT = TV_FIRST + 72;
pub type PFNTVCOMPARE = Option<unsafe extern "system" fn(
    lParam1: ::LPARAM, lParam2: ::LPARAM, lParamSort: ::LPARAM,
) -> ::c_int>;
pub type LPTV_SORTCB = LPTVSORTCB;
pub type TV_SORTCB = TVSORTCB;
#[repr(C)] #[derive(Copy)]
pub struct TVSORTCB {
    pub hParent: HTREEITEM,
    pub lpfnCompare: PFNTVCOMPARE,
    pub lParam: ::LPARAM,
}
impl Clone for TVSORTCB { fn clone(&self) -> TVSORTCB { *self } }
pub type LPTVSORTCB = *mut TVSORTCB;
pub type LPNM_TREEVIEWA = LPNMTREEVIEWA;
pub type LPNM_TREEVIEWW = LPNMTREEVIEWW;
pub type NM_TREEVIEWA = NMTREEVIEWA;
pub type NM_TREEVIEWW = NMTREEVIEWW;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMTREEVIEWA {
    pub hdr: ::NMHDR,
    pub action: ::UINT,
    pub itemOld: TVITEMA,
    pub itemNew: TVITEMA,
    pub ptDrag: ::POINT,
}
pub type LPNMTREEVIEWA = *mut NMTREEVIEWA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMTREEVIEWW {
    pub hdr: ::NMHDR,
    pub action: ::UINT,
    pub itemOld: TVITEMW,
    pub itemNew: TVITEMW,
    pub ptDrag: ::POINT,
}
pub type LPNMTREEVIEWW = *mut NMTREEVIEWW;
pub const TVN_SELCHANGINGA: ::UINT = TVN_FIRST - 1;
pub const TVN_SELCHANGINGW: ::UINT = TVN_FIRST - 50;
pub const TVN_SELCHANGEDA: ::UINT = TVN_FIRST - 2;
pub const TVN_SELCHANGEDW: ::UINT = TVN_FIRST - 51;
pub const TVN_GETDISPINFOA: ::UINT = TVN_FIRST - 3;
pub const TVN_GETDISPINFOW: ::UINT = TVN_FIRST - 52;
pub const TVN_SETDISPINFOA: ::UINT = TVN_FIRST - 4;
pub const TVN_SETDISPINFOW: ::UINT = TVN_FIRST - 53;
pub const TVC_UNKNOWN: ::LPARAM = 0x0000;
pub const TVC_BYMOUSE: ::LPARAM = 0x0001;
pub const TVC_BYKEYBOARD: ::LPARAM = 0x0002;
pub const TVIF_DI_SETITEM: ::UINT = 0x1000;
pub type TV_DISPINFOA = NMTVDISPINFOA;
pub type TV_DISPINFOW = NMTVDISPINFOW;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMTVDISPINFOA {
    pub hdr: ::NMHDR,
    pub item: TVITEMA,
}
pub type LPNMTVDISPINFOA = *mut NMTVDISPINFOA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMTVDISPINFOW {
    pub hdr: ::NMHDR,
    pub item: TVITEMW,
}
pub type LPNMTVDISPINFOW = *mut NMTVDISPINFOW;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMTVDISPINFOEXA {
    pub hdr: ::NMHDR,
    pub item: TVITEMEXA,
}
pub type LPNMTVDISPINFOEXA = *mut NMTVDISPINFOEXA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMTVDISPINFOEXW {
    pub hdr: ::NMHDR,
    pub item: TVITEMEXW,
}
pub type LPNMTVDISPINFOEXW = *mut NMTVDISPINFOEXW;
pub type TV_DISPINFOEXA = NMTVDISPINFOEXA;
pub type TV_DISPINFOEXW = NMTVDISPINFOEXW;
pub const TVN_ITEMEXPANDINGA: ::UINT = TVN_FIRST - 5;
pub const TVN_ITEMEXPANDINGW: ::UINT = TVN_FIRST - 54;
pub const TVN_ITEMEXPANDEDA: ::UINT = TVN_FIRST - 6;
pub const TVN_ITEMEXPANDEDW: ::UINT = TVN_FIRST - 55;
pub const TVN_BEGINDRAGA: ::UINT = TVN_FIRST - 7;
pub const TVN_BEGINDRAGW: ::UINT = TVN_FIRST - 56;
pub const TVN_BEGINRDRAGA: ::UINT = TVN_FIRST - 8;
pub const TVN_BEGINRDRAGW: ::UINT = TVN_FIRST - 57;
pub const TVN_DELETEITEMA: ::UINT = TVN_FIRST - 9;
pub const TVN_DELETEITEMW: ::UINT = TVN_FIRST - 58;
pub const TVN_BEGINLABELEDITA: ::UINT = TVN_FIRST - 10;
pub const TVN_BEGINLABELEDITW: ::UINT = TVN_FIRST - 59;
pub const TVN_ENDLABELEDITA: ::UINT = TVN_FIRST - 11;
pub const TVN_ENDLABELEDITW: ::UINT = TVN_FIRST - 60;
pub const TVN_KEYDOWN: ::UINT = TVN_FIRST - 12;
pub const TVN_GETINFOTIPA: ::UINT = TVN_FIRST - 13;
pub const TVN_GETINFOTIPW: ::UINT = TVN_FIRST - 14;
pub const TVN_SINGLEEXPAND: ::UINT = TVN_FIRST - 15;
pub const TVN_ITEMCHANGINGA: ::UINT = TVN_FIRST - 16;
pub const TVN_ITEMCHANGINGW: ::UINT = TVN_FIRST - 17;
pub const TVN_ITEMCHANGEDA: ::UINT = TVN_FIRST - 18;
pub const TVN_ITEMCHANGEDW: ::UINT = TVN_FIRST - 19;
pub const TVN_ASYNCDRAW: ::UINT = TVN_FIRST - 20;
pub const TVNRET_DEFAULT: ::LRESULT = 0;
pub const TVNRET_SKIPOLD: ::LRESULT = 1;
pub const TVNRET_SKIPNEW: ::LRESULT = 2;
pub type TV_KEYDOWN = NMTVKEYDOWN;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMTVKEYDOWN {
    pub hdr: ::NMHDR,
    pub wVKey: ::WORD,
    pub flags: ::UINT,
}
pub type LPNMTVKEYDOWN = *mut NMTVKEYDOWN;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMTVCUSTOMDRAW {
    pub nmcd: NMCUSTOMDRAW,
    pub clrText: ::COLORREF,
    pub clrTextBk: ::COLORREF,
    pub iLevel: ::c_int,
}
pub type LPNMTVCUSTOMDRAW = *mut NMTVCUSTOMDRAW;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMTVGETINFOTIPA {
    pub hdr: ::NMHDR,
    pub pszText: ::LPSTR,
    pub cchTextMax: ::c_int,
    pub hItem: HTREEITEM,
    pub lParam: ::LPARAM,
}
pub type LPNMTVGETINFOTIPA = *mut NMTVGETINFOTIPA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMTVGETINFOTIPW {
    pub hdr: ::NMHDR,
    pub pszText: ::LPWSTR,
    pub cchTextMax: ::c_int,
    pub hItem: HTREEITEM,
    pub lParam: ::LPARAM,
}
pub type LPNMTVGETINFOTIPW = *mut NMTVGETINFOTIPW;
pub const TVCDRF_NOIMAGES: ::LRESULT = 0x00010000;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMTVITEMCHANGE {
    pub hdr: ::NMHDR,
    pub uChanged: ::UINT,
    pub hItem: HTREEITEM,
    pub uStateNew: ::UINT,
    pub uStateOld: ::UINT,
    pub lParam: ::LPARAM,
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMTVASYNCDRAW {
    pub hdr: ::NMHDR,
    pub pimldp: *mut IMAGELISTDRAWPARAMS,
    pub hr: ::HRESULT,
    pub hItem: HTREEITEM,
    pub lParam: ::LPARAM,
    pub dwRetFlags: ::DWORD,
    pub iRetImageIndex: ::c_int,
}
pub const CBEIF_TEXT: ::UINT = 0x00000001;
pub const CBEIF_IMAGE: ::UINT = 0x00000002;
pub const CBEIF_SELECTEDIMAGE: ::UINT = 0x00000004;
pub const CBEIF_OVERLAY: ::UINT = 0x00000008;
pub const CBEIF_INDENT: ::UINT = 0x00000010;
pub const CBEIF_LPARAM: ::UINT = 0x00000020;
pub const CBEIF_DI_SETITEM: ::UINT = 0x10000000;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct COMBOBOXEXITEMA {
    pub mask: ::UINT,
    pub iItem: ::INT_PTR,
    pub pszText: ::LPSTR,
    pub cchTextMax: ::c_int,
    pub iImage: ::c_int,
    pub iSelectedImage: ::c_int,
    pub iOverlay: ::c_int,
    pub iIndent: ::c_int,
    pub lParam: ::LPARAM,
}
pub type PCOMBOBOXEXITEMA = *mut COMBOBOXEXITEMA;
pub type PCCOMBOBOXEXITEMA = *const COMBOBOXEXITEMA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct COMBOBOXEXITEMW {
    pub mask: ::UINT,
    pub iItem: ::INT_PTR,
    pub pszText: ::LPWSTR,
    pub cchTextMax: ::c_int,
    pub iImage: ::c_int,
    pub iSelectedImage: ::c_int,
    pub iOverlay: ::c_int,
    pub iIndent: ::c_int,
    pub lParam: ::LPARAM,
}
pub type PCOMBOBOXEXITEMW = *mut COMBOBOXEXITEMW;
pub type PCCOMBOBOXEXITEMW = *const COMBOBOXEXITEMW;
pub const CBEM_INSERTITEMA: ::UINT = ::WM_USER + 1;
pub const CBEM_SETIMAGELIST: ::UINT = ::WM_USER + 2;
pub const CBEM_GETIMAGELIST: ::UINT = ::WM_USER + 3;
pub const CBEM_GETITEMA: ::UINT = ::WM_USER + 4;
pub const CBEM_SETITEMA: ::UINT = ::WM_USER + 5;
pub const CBEM_DELETEITEM: ::UINT = ::CB_DELETESTRING;
pub const CBEM_GETCOMBOCONTROL: ::UINT = ::WM_USER + 6;
pub const CBEM_GETEDITCONTROL: ::UINT = ::WM_USER + 7;
pub const CBEM_SETEXSTYLE: ::UINT = ::WM_USER + 8;
pub const CBEM_SETEXTENDEDSTYLE: ::UINT = ::WM_USER + 14;
pub const CBEM_GETEXSTYLE: ::UINT = ::WM_USER + 9;
pub const CBEM_GETEXTENDEDSTYLE: ::UINT = ::WM_USER + 9;
pub const CBEM_SETUNICODEFORMAT: ::UINT = CCM_SETUNICODEFORMAT;
pub const CBEM_GETUNICODEFORMAT: ::UINT = CCM_GETUNICODEFORMAT;
pub const CBEM_HASEDITCHANGED: ::UINT = ::WM_USER + 10;
pub const CBEM_INSERTITEMW: ::UINT = ::WM_USER + 11;
pub const CBEM_SETITEMW: ::UINT = ::WM_USER + 12;
pub const CBEM_GETITEMW: ::UINT = ::WM_USER + 13;
pub const CBEM_SETWINDOWTHEME: ::UINT = CCM_SETWINDOWTHEME;
pub const CBES_EX_NOEDITIMAGE: ::DWORD = 0x00000001;
pub const CBES_EX_NOEDITIMAGEINDENT: ::DWORD = 0x00000002;
pub const CBES_EX_PATHWORDBREAKPROC: ::DWORD = 0x00000004;
pub const CBES_EX_NOSIZELIMIT: ::DWORD = 0x00000008;
pub const CBES_EX_CASESENSITIVE: ::DWORD = 0x00000010;
pub const CBES_EX_TEXTENDELLIPSIS: ::DWORD = 0x00000020;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMCOMBOBOXEXA {
    pub hdr: ::NMHDR,
    pub ceItem: COMBOBOXEXITEMA,
}
pub type PNMCOMBOBOXEXA = *mut NMCOMBOBOXEXA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMCOMBOBOXEXW {
    pub hdr: ::NMHDR,
    pub ceItem: COMBOBOXEXITEMW,
}
pub type PNMCOMBOBOXEXW = *mut NMCOMBOBOXEXW;
pub const CBEN_GETDISPINFOA: ::UINT = CBEN_FIRST - 0;
pub const CBEN_INSERTITEM: ::UINT = CBEN_FIRST - 1;
pub const CBEN_DELETEITEM: ::UINT = CBEN_FIRST - 2;
pub const CBEN_BEGINEDIT: ::UINT = CBEN_FIRST - 4;
pub const CBEN_ENDEDITA: ::UINT = CBEN_FIRST - 5;
pub const CBEN_ENDEDITW: ::UINT = CBEN_FIRST - 6;
pub const CBEN_GETDISPINFOW: ::UINT = CBEN_FIRST - 7;
pub const CBEN_DRAGBEGINA: ::UINT = CBEN_FIRST - 8;
pub const CBEN_DRAGBEGINW: ::UINT = CBEN_FIRST - 9;
pub const CBENF_KILLFOCUS: ::c_int = 1;
pub const CBENF_RETURN: ::c_int = 2;
pub const CBENF_ESCAPE: ::c_int = 3;
pub const CBENF_DROPDOWN: ::c_int = 4;
pub const CBEMAXSTRLEN: usize = 260;
#[repr(C)] #[derive(Copy)]
pub struct NMCBEDRAGBEGINW {
    pub hdr: ::NMHDR,
    pub iItemid: ::c_int,
    pub szText: [::WCHAR; CBEMAXSTRLEN],
}
impl Clone for NMCBEDRAGBEGINW { fn clone(&self) -> NMCBEDRAGBEGINW { *self } }
pub type PNMCBEDRAGBEGINW = *mut NMCBEDRAGBEGINW;
pub type LPNMCBEDRAGBEGINW = *mut NMCBEDRAGBEGINW;
#[repr(C)] #[derive(Copy)]
pub struct NMCBEDRAGBEGINA {
    pub hdr: ::NMHDR,
    pub iItemid: ::c_int,
    pub szText: [::c_char; CBEMAXSTRLEN],
}
impl Clone for NMCBEDRAGBEGINA { fn clone(&self) -> NMCBEDRAGBEGINA { *self } }
pub type PNMCBEDRAGBEGINA = *mut NMCBEDRAGBEGINA;
pub type LPNMCBEDRAGBEGINA = *mut NMCBEDRAGBEGINA;
#[repr(C)] #[derive(Copy)]
pub struct NMCBEENDEDITW {
    pub hdr: ::NMHDR,
    pub fChanged: ::BOOL,
    pub iNewSelection: ::c_int,
    pub szText: [::WCHAR; CBEMAXSTRLEN],
    pub iWhy: ::c_int,
}
impl Clone for NMCBEENDEDITW { fn clone(&self) -> NMCBEENDEDITW { *self } }
pub type PNMCBEENDEDITW = *mut NMCBEENDEDITW;
pub type LPNMCBEENDEDITW = *mut NMCBEENDEDITW;
#[repr(C)] #[derive(Copy)]
pub struct NMCBEENDEDITA {
    pub hdr: ::NMHDR,
    pub fChanged: ::BOOL,
    pub iNewSelection: ::c_int,
    pub szText: [::c_char; CBEMAXSTRLEN],
    pub iWhy: ::c_int,
}
impl Clone for NMCBEENDEDITA { fn clone(&self) -> NMCBEENDEDITA { *self } }
pub type PNMCBEENDEDITA = *mut NMCBEENDEDITA;
pub type LPNMCBEENDEDITA = *mut NMCBEENDEDITA;
pub const TCS_SCROLLOPPOSITE: ::DWORD = 0x0001;
pub const TCS_BOTTOM: ::DWORD = 0x0002;
pub const TCS_RIGHT: ::DWORD = 0x0002;
pub const TCS_MULTISELECT: ::DWORD = 0x0004;
pub const TCS_FLATBUTTONS: ::DWORD = 0x0008;
pub const TCS_FORCEICONLEFT: ::DWORD = 0x0010;
pub const TCS_FORCELABELLEFT: ::DWORD = 0x0020;
pub const TCS_HOTTRACK: ::DWORD = 0x0040;
pub const TCS_VERTICAL: ::DWORD = 0x0080;
pub const TCS_TABS: ::DWORD = 0x0000;
pub const TCS_BUTTONS: ::DWORD = 0x0100;
pub const TCS_SINGLELINE: ::DWORD = 0x0000;
pub const TCS_MULTILINE: ::DWORD = 0x0200;
pub const TCS_RIGHTJUSTIFY: ::DWORD = 0x0000;
pub const TCS_FIXEDWIDTH: ::DWORD = 0x0400;
pub const TCS_RAGGEDRIGHT: ::DWORD = 0x0800;
pub const TCS_FOCUSONBUTTONDOWN: ::DWORD = 0x1000;
pub const TCS_OWNERDRAWFIXED: ::DWORD = 0x2000;
pub const TCS_TOOLTIPS: ::DWORD = 0x4000;
pub const TCS_FOCUSNEVER: ::DWORD = 0x8000;
pub const TCS_EX_FLATSEPARATORS: ::DWORD = 0x00000001;
pub const TCS_EX_REGISTERDROP: ::DWORD = 0x00000002;
pub const TCM_GETIMAGELIST: ::UINT = TCM_FIRST + 2;
pub const TCM_SETIMAGELIST: ::UINT = TCM_FIRST + 3;
pub const TCM_GETITEMCOUNT: ::UINT = TCM_FIRST + 4;
pub const TCIF_TEXT: ::UINT = 0x0001;
pub const TCIF_IMAGE: ::UINT = 0x0002;
pub const TCIF_RTLREADING: ::UINT = 0x0004;
pub const TCIF_PARAM: ::UINT = 0x0008;
pub const TCIF_STATE: ::UINT = 0x0010;
pub const TCIS_BUTTONPRESSED: ::DWORD = 0x0001;
pub const TCIS_HIGHLIGHTED: ::DWORD = 0x0002;
pub type TC_ITEMHEADERA = TCITEMHEADERA;
pub type TC_ITEMHEADERW = TCITEMHEADERW;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct TCITEMHEADERA {
    pub mask: ::UINT,
    pub lpReserved1: ::UINT,
    pub lpReserved2: ::UINT,
    pub pszText: ::LPSTR,
    pub cchTextMax: ::c_int,
    pub iImage: ::c_int,
}
pub type LPTCITEMHEADERA = *mut TCITEMHEADERA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct TCITEMHEADERW {
    pub mask: ::UINT,
    pub lpReserved1: ::UINT,
    pub lpReserved2: ::UINT,
    pub pszText: ::LPWSTR,
    pub cchTextMax: ::c_int,
    pub iImage: ::c_int,
}
pub type LPTCITEMHEADERW = *mut TCITEMHEADERW;
pub type TC_ITEMA = TCITEMA;
pub type TC_ITEMW = TCITEMW;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct TCITEMA {
    pub mask: ::UINT,
    pub dwState: ::DWORD,
    pub dwStateMask: ::DWORD,
    pub pszText: ::LPSTR,
    pub cchTextMax: ::c_int,
    pub iImage: ::c_int,
    pub lParam: ::LPARAM,
}
pub type LPTCITEMA = *mut TCITEMA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct TCITEMW {
    pub mask: ::UINT,
    pub dwState: ::DWORD,
    pub dwStateMask: ::DWORD,
    pub pszText: ::LPWSTR,
    pub cchTextMax: ::c_int,
    pub iImage: ::c_int,
    pub lParam: ::LPARAM,
}
pub type LPTCITEMW = *mut TCITEMW;
pub const TCM_GETITEMA: ::UINT = TCM_FIRST + 5;
pub const TCM_GETITEMW: ::UINT = TCM_FIRST + 60;
pub const TCM_SETITEMA: ::UINT = TCM_FIRST + 6;
pub const TCM_SETITEMW: ::UINT = TCM_FIRST + 61;
pub const TCM_INSERTITEMA: ::UINT = TCM_FIRST + 7;
pub const TCM_INSERTITEMW: ::UINT = TCM_FIRST + 62;
pub const TCM_DELETEITEM: ::UINT = TCM_FIRST + 8;
pub const TCM_DELETEALLITEMS: ::UINT = TCM_FIRST + 9;
pub const TCM_GETITEMRECT: ::UINT = TCM_FIRST + 10;
pub const TCM_GETCURSEL: ::UINT = TCM_FIRST + 11;
pub const TCM_SETCURSEL: ::UINT = TCM_FIRST + 12;
pub const TCHT_NOWHERE: ::UINT = 0x0001;
pub const TCHT_ONITEMICON: ::UINT = 0x0002;
pub const TCHT_ONITEMLABEL: ::UINT = 0x0004;
pub const TCHT_ONITEM: ::UINT = TCHT_ONITEMICON | TCHT_ONITEMLABEL;
pub type LPTC_HITTESTINFO = LPTCHITTESTINFO;
pub type TC_HITTESTINFO = TCHITTESTINFO;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct TCHITTESTINFO {
    pub pt: ::POINT,
    pub flags: ::UINT,
}
pub type LPTCHITTESTINFO = *mut TCHITTESTINFO;
pub const TCM_HITTEST: ::UINT = TCM_FIRST + 13;
pub const TCM_SETITEMEXTRA: ::UINT = TCM_FIRST + 14;
pub const TCM_ADJUSTRECT: ::UINT = TCM_FIRST + 40;
pub const TCM_SETITEMSIZE: ::UINT = TCM_FIRST + 41;
pub const TCM_REMOVEIMAGE: ::UINT = TCM_FIRST + 42;
pub const TCM_SETPADDING: ::UINT = TCM_FIRST + 43;
pub const TCM_GETROWCOUNT: ::UINT = TCM_FIRST + 44;
pub const TCM_GETTOOLTIPS: ::UINT = TCM_FIRST + 45;
pub const TCM_SETTOOLTIPS: ::UINT = TCM_FIRST + 46;
pub const TCM_GETCURFOCUS: ::UINT = TCM_FIRST + 47;
pub const TCM_SETCURFOCUS: ::UINT = TCM_FIRST + 48;
pub const TCM_SETMINTABWIDTH: ::UINT = TCM_FIRST + 49;
pub const TCM_DESELECTALL: ::UINT = TCM_FIRST + 50;
pub const TCM_HIGHLIGHTITEM: ::UINT = TCM_FIRST + 51;
pub const TCM_SETEXTENDEDSTYLE: ::UINT = TCM_FIRST + 52;
pub const TCM_GETEXTENDEDSTYLE: ::UINT = TCM_FIRST + 53;
pub const TCM_SETUNICODEFORMAT: ::UINT = CCM_SETUNICODEFORMAT;
pub const TCM_GETUNICODEFORMAT: ::UINT = CCM_GETUNICODEFORMAT;
pub const TCN_KEYDOWN: ::UINT = TCN_FIRST - 0;
pub type TC_KEYDOWN = NMTCKEYDOWN;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMTCKEYDOWN {
    pub hdr: ::NMHDR,
    pub wVKey: ::WORD,
    pub flags: ::UINT,
}
pub const TCN_SELCHANGE: ::UINT = TCN_FIRST - 1;
pub const TCN_SELCHANGING: ::UINT = TCN_FIRST - 2;
pub const TCN_GETOBJECT: ::UINT = TCN_FIRST - 3;
pub const TCN_FOCUSCHANGE: ::UINT = TCN_FIRST - 4;
pub const ACS_CENTER: ::DWORD = 0x0001;
pub const ACS_TRANSPARENT: ::DWORD = 0x0002;
pub const ACS_AUTOPLAY: ::DWORD = 0x0004;
pub const ACS_TIMER: ::DWORD = 0x0008;
pub const ACM_OPENA: ::UINT = ::WM_USER + 100;
pub const ACM_OPENW: ::UINT = ::WM_USER + 103;
pub const ACM_PLAY: ::UINT = ::WM_USER + 101;
pub const ACM_STOP: ::UINT = ::WM_USER + 102;
pub const ACM_ISPLAYING: ::UINT = ::WM_USER + 104;
pub const ACN_START: ::WPARAM = 1;
pub const ACN_STOP: ::WPARAM = 2;
pub type MONTHDAYSTATE = ::DWORD;
pub type LPMONTHDAYSTATE = *mut ::DWORD;
pub const MCM_FIRST: ::UINT = 0x1000;
pub const MCM_GETCURSEL: ::UINT = MCM_FIRST + 1;
pub const MCM_SETCURSEL: ::UINT = MCM_FIRST + 2;
pub const MCM_GETMAXSELCOUNT: ::UINT = MCM_FIRST + 3;
pub const MCM_SETMAXSELCOUNT: ::UINT = MCM_FIRST + 4;
pub const MCM_GETSELRANGE: ::UINT = MCM_FIRST + 5;
pub const MCM_SETSELRANGE: ::UINT = MCM_FIRST + 6;
pub const MCM_GETMONTHRANGE: ::UINT = MCM_FIRST + 7;
pub const MCM_SETDAYSTATE: ::UINT = MCM_FIRST + 8;
pub const MCM_GETMINREQRECT: ::UINT = MCM_FIRST + 9;
pub const MCM_SETCOLOR: ::UINT = MCM_FIRST + 10;
pub const MCM_GETCOLOR: ::UINT = MCM_FIRST + 11;
pub const MCM_SETTODAY: ::UINT = MCM_FIRST + 12;
pub const MCM_GETTODAY: ::UINT = MCM_FIRST + 13;
pub const MCM_HITTEST: ::UINT = MCM_FIRST + 14;
pub const MCSC_BACKGROUND: ::WPARAM = 0;
pub const MCSC_TEXT: ::WPARAM = 1;
pub const MCSC_TITLEBK: ::WPARAM = 2;
pub const MCSC_TITLETEXT: ::WPARAM = 3;
pub const MCSC_MONTHBK: ::WPARAM = 4;
pub const MCSC_TRAILINGTEXT: ::WPARAM = 5;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct MCHITTESTINFO {
    pub cbSize: ::UINT,
    pub pt: ::POINT,
    pub uHit: ::UINT,
    pub st: ::SYSTEMTIME,
    pub rc: ::RECT,
    pub iOffset: ::c_int,
    pub iRow: ::c_int,
    pub iCol: ::c_int,
}
pub type PMCHITTESTINFO = *mut MCHITTESTINFO;
pub const MCHT_TITLE: ::UINT = 0x00010000;
pub const MCHT_CALENDAR: ::UINT = 0x00020000;
pub const MCHT_TODAYLINK: ::UINT = 0x00030000;
pub const MCHT_CALENDARCONTROL: ::UINT = 0x00100000;
pub const MCHT_NEXT: ::UINT = 0x01000000;
pub const MCHT_PREV: ::UINT = 0x02000000;
pub const MCHT_NOWHERE: ::UINT = 0x00000000;
pub const MCHT_TITLEBK: ::UINT = MCHT_TITLE;
pub const MCHT_TITLEMONTH: ::UINT = MCHT_TITLE | 0x0001;
pub const MCHT_TITLEYEAR: ::UINT = MCHT_TITLE | 0x0002;
pub const MCHT_TITLEBTNNEXT: ::UINT = MCHT_TITLE | MCHT_NEXT | 0x0003;
pub const MCHT_TITLEBTNPREV: ::UINT = MCHT_TITLE | MCHT_PREV | 0x0003;
pub const MCHT_CALENDARBK: ::UINT = MCHT_CALENDAR;
pub const MCHT_CALENDARDATE: ::UINT = MCHT_CALENDAR | 0x0001;
pub const MCHT_CALENDARDATENEXT: ::UINT = MCHT_CALENDARDATE | MCHT_NEXT;
pub const MCHT_CALENDARDATEPREV: ::UINT = MCHT_CALENDARDATE | MCHT_PREV;
pub const MCHT_CALENDARDAY: ::UINT = MCHT_CALENDAR | 0x0002;
pub const MCHT_CALENDARWEEKNUM: ::UINT = MCHT_CALENDAR | 0x0003;
pub const MCHT_CALENDARDATEMIN: ::UINT = MCHT_CALENDAR | 0x0004;
pub const MCHT_CALENDARDATEMAX: ::UINT = MCHT_CALENDAR | 0x0005;
pub const MCM_SETFIRSTDAYOFWEEK: ::UINT = MCM_FIRST + 15;
pub const MCM_GETFIRSTDAYOFWEEK: ::UINT = MCM_FIRST + 16;
pub const MCM_GETRANGE: ::UINT = MCM_FIRST + 17;
pub const MCM_SETRANGE: ::UINT = MCM_FIRST + 18;
pub const MCM_GETMONTHDELTA: ::UINT = MCM_FIRST + 19;
pub const MCM_SETMONTHDELTA: ::UINT = MCM_FIRST + 20;
pub const MCM_GETMAXTODAYWIDTH: ::UINT = MCM_FIRST + 21;
pub const MCM_SETUNICODEFORMAT: ::UINT = CCM_SETUNICODEFORMAT;
pub const MCM_GETUNICODEFORMAT: ::UINT = CCM_GETUNICODEFORMAT;
pub const MCM_GETCURRENTVIEW: ::UINT = MCM_FIRST + 22;
pub const MCM_GETCALENDARCOUNT: ::UINT = MCM_FIRST + 23;
pub const MCMV_MONTH: ::DWORD = 0;
pub const MCMV_YEAR: ::DWORD = 1;
pub const MCMV_DECADE: ::DWORD = 2;
pub const MCMV_CENTURY: ::DWORD = 3;
pub const MCMV_MAX: ::DWORD = MCMV_CENTURY;
pub const MCGIP_CALENDARCONTROL: ::DWORD = 0;
pub const MCGIP_NEXT: ::DWORD = 1;
pub const MCGIP_PREV: ::DWORD = 2;
pub const MCGIP_FOOTER: ::DWORD = 3;
pub const MCGIP_CALENDAR: ::DWORD = 4;
pub const MCGIP_CALENDARHEADER: ::DWORD = 5;
pub const MCGIP_CALENDARBODY: ::DWORD = 6;
pub const MCGIP_CALENDARROW: ::DWORD = 7;
pub const MCGIP_CALENDARCELL: ::DWORD = 8;
pub const MCGIF_DATE: ::DWORD = 0x00000001;
pub const MCGIF_RECT: ::DWORD = 0x00000002;
pub const MCGIF_NAME: ::DWORD = 0x00000004;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct MCGRIDINFO {
    pub cbSize: ::UINT,
    pub dwPart: ::DWORD,
    pub dwFlags: ::DWORD,
    pub iCalendar: ::c_int,
    pub iRow: ::c_int,
    pub iCol: ::c_int,
    pub bSelected: ::BOOL,
    pub stStart: ::SYSTEMTIME,
    pub stEnd: ::SYSTEMTIME,
    pub rc: ::RECT,
    pub pszName: ::PWSTR,
    pub cchName: ::size_t,
}
pub type PMCGRIDINFO = *mut MCGRIDINFO;
pub const MCM_GETCALENDARGRIDINFO: ::UINT = MCM_FIRST + 24;
pub const MCM_GETCALID: ::UINT = MCM_FIRST + 27;
pub const MCM_SETCALID: ::UINT = MCM_FIRST + 28;
pub const MCM_SIZERECTTOMIN: ::UINT = MCM_FIRST + 29;
pub const MCM_SETCALENDARBORDER: ::UINT = MCM_FIRST + 30;
pub const MCM_GETCALENDARBORDER: ::UINT = MCM_FIRST + 31;
pub const MCM_SETCURRENTVIEW: ::UINT = MCM_FIRST + 32;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMSELCHANGE {
    pub nmhdr: ::NMHDR,
    pub stSelStart: ::SYSTEMTIME,
    pub stSelEnd: ::SYSTEMTIME,
}
pub type LPNMSELCHANGE = *mut NMSELCHANGE;
pub const MCN_SELCHANGE: ::UINT = MCN_FIRST - 3;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMDAYSTATE {
    pub nmhdr: ::NMHDR,
    pub stStart: ::SYSTEMTIME,
    pub cDayState: ::c_int,
    pub prgDayState: LPMONTHDAYSTATE
}
pub type LPNMDAYSTATE = *mut NMDAYSTATE;
pub const MCN_GETDAYSTATE: ::UINT = MCN_FIRST - 1;
pub type NMSELECT = NMSELCHANGE;
pub type LPNMSELECT = *mut NMSELCHANGE;
pub const MCN_SELECT: ::UINT = MCN_FIRST;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMVIEWCHANGE {
    pub nmhdr: ::NMHDR,
    pub dwOldView: ::DWORD,
    pub dwNewView: ::DWORD,
}
pub type LPNMVIEWCHANGE = *mut NMVIEWCHANGE;
pub const MCN_VIEWCHANGE: ::UINT = MCN_FIRST - 4;
pub const MCS_DAYSTATE: ::DWORD = 0x0001;
pub const MCS_MULTISELECT: ::DWORD = 0x0002;
pub const MCS_WEEKNUMBERS: ::DWORD = 0x0004;
pub const MCS_NOTODAYCIRCLE: ::DWORD = 0x0008;
pub const MCS_NOTODAY: ::DWORD = 0x0010;
pub const MCS_NOTRAILINGDATES: ::DWORD = 0x0040;
pub const MCS_SHORTDAYSOFWEEK: ::DWORD = 0x0080;
pub const MCS_NOSELCHANGEONNAV: ::DWORD = 0x0100;
pub const GMR_VISIBLE: ::DWORD = 0;
pub const GMR_DAYSTATE: ::DWORD = 1;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DATETIMEPICKERINFO {
    pub cbSize: ::UINT,
    pub rcCheck: ::RECT,
    pub stateCheck: ::DWORD,
    pub rcButton: ::RECT,
    pub stateButton: ::DWORD,
    pub hwndEdit: ::HWND,
    pub hwndUD: ::HWND,
    pub hwndDropDown: ::HWND,
}
pub type LPDATETIMEPICKERINFO = *mut DATETIMEPICKERINFO;
pub const DTM_FIRST: ::UINT = 0x1000;
pub const DTM_GETSYSTEMTIME: ::UINT = DTM_FIRST + 1;
pub const DTM_SETSYSTEMTIME: ::UINT = DTM_FIRST + 2;
pub const DTM_GETRANGE: ::UINT = DTM_FIRST + 3;
pub const DTM_SETRANGE: ::UINT = DTM_FIRST + 4;
pub const DTM_SETFORMATA: ::UINT = DTM_FIRST + 5;
pub const DTM_SETFORMATW: ::UINT = DTM_FIRST + 50;
pub const DTM_SETMCCOLOR: ::UINT = DTM_FIRST + 6;
pub const DTM_GETMCCOLOR: ::UINT = DTM_FIRST + 7;
pub const DTM_GETMONTHCAL: ::UINT = DTM_FIRST + 8;
pub const DTM_SETMCFONT: ::UINT = DTM_FIRST + 9;
pub const DTM_GETMCFONT: ::UINT = DTM_FIRST + 10;
pub const DTM_SETMCSTYLE: ::UINT = DTM_FIRST + 11;
pub const DTM_GETMCSTYLE: ::UINT = DTM_FIRST + 12;
pub const DTM_CLOSEMONTHCAL: ::UINT = DTM_FIRST + 13;
pub const DTM_GETDATETIMEPICKERINFO: ::UINT = DTM_FIRST + 14;
pub const DTM_GETIDEALSIZE: ::UINT = DTM_FIRST + 15;
pub const DTS_UPDOWN: ::DWORD = 0x0001;
pub const DTS_SHOWNONE: ::DWORD = 0x0002;
pub const DTS_SHORTDATEFORMAT: ::DWORD = 0x0000;
pub const DTS_LONGDATEFORMAT: ::DWORD = 0x0004;
pub const DTS_SHORTDATECENTURYFORMAT: ::DWORD = 0x000C;
pub const DTS_TIMEFORMAT: ::DWORD = 0x0009;
pub const DTS_APPCANPARSE: ::DWORD = 0x0010;
pub const DTS_RIGHTALIGN: ::DWORD = 0x0020;
pub const DTN_DATETIMECHANGE: ::UINT = DTN_FIRST2 - 6;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMDATETIMECHANGE {
    pub nmhdr: ::NMHDR,
    pub dwFlags: ::DWORD,
    pub st: ::SYSTEMTIME,
}
pub type LPNMDATETIMECHANGE = *mut NMDATETIMECHANGE;
pub const DTN_USERSTRINGA: ::UINT = DTN_FIRST2 - 5;
pub const DTN_USERSTRINGW: ::UINT = DTN_FIRST - 5;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMDATETIMESTRINGA {
    pub nmhdr: ::NMHDR,
    pub pszUserString: ::LPCSTR,
    pub st: ::SYSTEMTIME,
    pub dwFlags: ::DWORD,
}
pub type LPNMDATETIMESTRINGA = *mut NMDATETIMESTRINGA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMDATETIMESTRINGW {
    pub nmhdr: ::NMHDR,
    pub pszUserString: ::LPCWSTR,
    pub st: ::SYSTEMTIME,
    pub dwFlags: ::DWORD,
}
pub type LPNMDATETIMESTRINGW = *mut NMDATETIMESTRINGW;
pub const DTN_WMKEYDOWNA: ::UINT = DTN_FIRST2 - 4;
pub const DTN_WMKEYDOWNW: ::UINT = DTN_FIRST - 4;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMDATETIMEWMKEYDOWNA {
    pub nmhdr: ::NMHDR,
    pub nVirtKey: ::c_int,
    pub pszFormat: ::LPCSTR,
    pub st: ::SYSTEMTIME,
}
pub type LPNMDATETIMEWMKEYDOWNA = *mut NMDATETIMEWMKEYDOWNA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMDATETIMEWMKEYDOWNW {
    pub nmhdr: ::NMHDR,
    pub nVirtKey: ::c_int,
    pub pszFormat: ::LPCWSTR,
    pub st: ::SYSTEMTIME,
}
pub type LPNMDATETIMEWMKEYDOWNW = *mut NMDATETIMEWMKEYDOWNW;
pub const DTN_FORMATA: ::UINT = DTN_FIRST2 - 3;
pub const DTN_FORMATW: ::UINT = DTN_FIRST - 3;
#[repr(C)] #[derive(Copy)]
pub struct NMDATETIMEFORMATA {
    pub nmhdr: ::NMHDR,
    pub pszFormat: ::LPCSTR,
    pub st: ::SYSTEMTIME,
    pub pszDisplay: ::LPCSTR,
    pub szDisplay: [::CHAR; 64],
}
impl Clone for NMDATETIMEFORMATA { fn clone(&self) -> NMDATETIMEFORMATA { *self } }
pub type LPNMDATETIMEFORMATA = *mut NMDATETIMEFORMATA;
#[repr(C)] #[derive(Copy)]
pub struct NMDATETIMEFORMATW {
    pub nmhdr: ::NMHDR,
    pub pszFormat: ::LPCWSTR,
    pub st: ::SYSTEMTIME,
    pub pszDisplay: ::LPCWSTR,
    pub szDisplay: [::WCHAR; 64],
}
impl Clone for NMDATETIMEFORMATW { fn clone(&self) -> NMDATETIMEFORMATW { *self } }
pub type LPNMDATETIMEFORMATW = *mut NMDATETIMEFORMATW;
pub const DTN_FORMATQUERYA: ::UINT = DTN_FIRST2 - 2;
pub const DTN_FORMATQUERYW: ::UINT = DTN_FIRST - 2;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMDATETIMEFORMATQUERYA {
    pub nmhdr: ::NMHDR,
    pub pszFormat: ::LPCSTR,
    pub szMax: ::SIZE,
}
pub type LPNMDATETIMEFORMATQUERYA = *mut NMDATETIMEFORMATQUERYA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMDATETIMEFORMATQUERYW {
    pub nmhdr: ::NMHDR,
    pub pszFormat: ::LPCWSTR,
    pub szMax: ::SIZE,
}
pub type LPNMDATETIMEFORMATQUERYW = *mut NMDATETIMEFORMATQUERYW;
pub const DTN_DROPDOWN: ::UINT = DTN_FIRST2 - 1;
pub const DTN_CLOSEUP: ::UINT = DTN_FIRST2;
pub const GDTR_MIN: ::WPARAM = 0x0001;
pub const GDTR_MAX: ::WPARAM = 0x0002;
pub const GDT_ERROR: ::LRESULT = -1;
pub const GDT_VALID: ::LRESULT = 0;
pub const GDT_NONE: ::LRESULT = 1;
pub const IPM_CLEARADDRESS: ::UINT = ::WM_USER + 100;
pub const IPM_SETADDRESS: ::UINT = ::WM_USER + 101;
pub const IPM_GETADDRESS: ::UINT = ::WM_USER + 102;
pub const IPM_SETRANGE: ::UINT = ::WM_USER + 103;
pub const IPM_SETFOCUS: ::UINT = ::WM_USER + 104;
pub const IPM_ISBLANK: ::UINT = ::WM_USER + 105;
pub const IPN_FIELDCHANGED: ::UINT = IPN_FIRST - 0;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMIPADDRESS {
    pub hdr: ::NMHDR,
    pub iField: ::c_int,
    pub iValue: ::c_int,
}
pub type LPNMIPADDRESS = *mut NMIPADDRESS;
#[inline] #[allow(dead_code)]
pub fn MAKEIPRANGE(low: ::BYTE, high: ::BYTE) -> ::LPARAM {
    (high << 8 + low) as ::LPARAM
}
#[inline] #[allow(dead_code)]
pub fn MAKEIPADDRESS(b1: ::DWORD, b2: ::DWORD, b3: ::DWORD, b4: ::DWORD) -> ::LPARAM {
    ((b1 << 24) + (b2 << 16) + (b3 << 8) + b4) as ::LPARAM
}
pub const PGS_VERT: ::DWORD = 0x00000000;
pub const PGS_HORZ: ::DWORD = 0x00000001;
pub const PGS_AUTOSCROLL: ::DWORD = 0x00000002;
pub const PGS_DRAGNDROP: ::DWORD = 0x00000004;
pub const  PGF_INVISIBLE: ::DWORD = 0;
pub const  PGF_NORMAL: ::DWORD = 1;
pub const  PGF_GRAYED: ::DWORD = 2;
pub const  PGF_DEPRESSED: ::DWORD = 4;
pub const  PGF_HOT: ::DWORD = 8;
pub const PGB_TOPORLEFT: ::c_int = 0;
pub const PGB_BOTTOMORRIGHT: ::c_int = 1;
pub const PGM_SETCHILD: ::UINT = PGM_FIRST + 1;
pub const PGM_RECALCSIZE: ::UINT = PGM_FIRST + 2;
pub const PGM_FORWARDMOUSE: ::UINT = PGM_FIRST + 3;
pub const PGM_SETBKCOLOR: ::UINT = PGM_FIRST + 4;
pub const PGM_GETBKCOLOR: ::UINT = PGM_FIRST + 5;
pub const PGM_SETBORDER: ::UINT = PGM_FIRST + 6;
pub const PGM_GETBORDER: ::UINT = PGM_FIRST + 7;
pub const PGM_SETPOS: ::UINT = PGM_FIRST + 8;
pub const PGM_GETPOS: ::UINT = PGM_FIRST + 9;
pub const PGM_SETBUTTONSIZE: ::UINT = PGM_FIRST + 10;
pub const PGM_GETBUTTONSIZE: ::UINT = PGM_FIRST + 11;
pub const PGM_GETBUTTONSTATE: ::UINT = PGM_FIRST + 12;
pub const PGM_GETDROPTARGET: ::UINT = CCM_GETDROPTARGET;
pub const PGM_SETSCROLLINFO: ::UINT = PGM_FIRST + 13;
pub const PGN_SCROLL: ::UINT = PGN_FIRST - 1;
pub const PGF_SCROLLUP: ::c_int = 1;
pub const PGF_SCROLLDOWN: ::c_int = 2;
pub const PGF_SCROLLLEFT: ::c_int = 4;
pub const PGF_SCROLLRIGHT: ::c_int = 8;
pub const PGK_SHIFT: ::BOOL = 1;
pub const PGK_CONTROL: ::BOOL = 2;
pub const PGK_MENU: ::BOOL = 4;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMPGSCROLL {
    pub hdr: ::NMHDR,
    pub fwKeys: ::BOOL,
    pub rcParent: ::RECT,
    pub iDir: ::c_int,
    pub iXpos: ::c_int,
    pub iYpos: ::c_int,
    pub iScroll: ::c_int,
}
pub type LPNMPGSCROLL = *mut NMPGSCROLL;
pub const PGN_CALCSIZE: ::UINT = PGN_FIRST - 2;
pub const PGF_CALCWIDTH: ::DWORD = 1;
pub const PGF_CALCHEIGHT: ::DWORD = 2;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMPGCALCSIZE {
    pub hdr: ::NMHDR,
    pub dwFlag: ::DWORD,
    pub iWidth: ::c_int,
    pub iHeight: ::c_int,
}
pub type LPNMPGCALCSIZE = *mut NMPGCALCSIZE;
pub const PGN_HOTITEMCHANGE: ::UINT = PGN_FIRST - 3;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMPGHOTITEM {
    pub hdr: ::NMHDR,
    pub idOld: ::c_int,
    pub idNew: ::c_int,
    pub dwFlags: ::DWORD,
}
pub type LPNMPGHOTITEM = *mut NMPGHOTITEM;
pub const NFS_EDIT: ::DWORD = 0x0001;
pub const NFS_STATIC: ::DWORD = 0x0002;
pub const NFS_LISTCOMBO: ::DWORD = 0x0004;
pub const NFS_BUTTON: ::DWORD = 0x0008;
pub const NFS_ALL: ::DWORD = 0x0010;
pub const NFS_USEFONTASSOC: ::DWORD = 0x0020;
pub const BUTTON_IMAGELIST_ALIGN_LEFT: ::UINT = 0;
pub const BUTTON_IMAGELIST_ALIGN_RIGHT: ::UINT = 1;
pub const BUTTON_IMAGELIST_ALIGN_TOP: ::UINT = 2;
pub const BUTTON_IMAGELIST_ALIGN_BOTTOM: ::UINT = 3;
pub const BUTTON_IMAGELIST_ALIGN_CENTER: ::UINT = 4;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct BUTTON_IMAGELIST {
    pub himl: HIMAGELIST,
    pub margin: ::RECT,
    pub uAlign: ::UINT,
}
pub type PBUTTON_IMAGELIST = *mut BUTTON_IMAGELIST;
pub const BCM_GETIDEALSIZE: ::UINT = BCM_FIRST + 0x0001;
pub const BCM_SETIMAGELIST: ::UINT = BCM_FIRST + 0x0002;
pub const BCM_GETIMAGELIST: ::UINT = BCM_FIRST + 0x0003;
pub const BCM_SETTEXTMARGIN: ::UINT = BCM_FIRST + 0x0004;
pub const BCM_GETTEXTMARGIN: ::UINT = BCM_FIRST + 0x0005;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMBCHOTITEM {
    pub hdr: ::NMHDR,
    pub dwFlags: ::DWORD,
}
pub type LPNMBCHOTITEM = *mut NMBCHOTITEM;
pub const BCN_HOTITEMCHANGE: ::UINT = BCN_FIRST + 0x0001;
pub const BS_SPLITBUTTON: ::UINT = 0x0000000C;
pub const BS_DEFSPLITBUTTON: ::UINT = 0x0000000D;
pub const BS_COMMANDLINK: ::UINT = 0x0000000E;
pub const BS_DEFCOMMANDLINK: ::UINT = 0x0000000F;
pub const BCSIF_GLYPH: ::UINT = 0x0001;
pub const BCSIF_IMAGE: ::UINT = 0x0002;
pub const BCSIF_STYLE: ::UINT = 0x0004;
pub const BCSIF_SIZE: ::UINT = 0x0008;
pub const BCSS_NOSPLIT: ::UINT = 0x0001;
pub const BCSS_STRETCH: ::UINT = 0x0002;
pub const BCSS_ALIGNLEFT: ::UINT = 0x0004;
pub const BCSS_IMAGE: ::UINT = 0x0008;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct BUTTON_SPLITINFO {
    pub mask: ::UINT,
    pub himlGlyph: HIMAGELIST,
    pub uSplitStyle: ::UINT,
    pub size: ::SIZE,
}
pub type PBUTTON_SPLITINFO = *mut BUTTON_SPLITINFO;
pub const BCM_SETDROPDOWNSTATE: ::UINT = BCM_FIRST + 0x0006;
pub const BCM_SETSPLITINFO: ::UINT = BCM_FIRST + 0x0007;
pub const BCM_GETSPLITINFO: ::UINT = BCM_FIRST + 0x0008;
pub const BCM_SETNOTE: ::UINT = BCM_FIRST + 0x0009;
pub const BCM_GETNOTE: ::UINT = BCM_FIRST + 0x000A;
pub const BCM_GETNOTELENGTH: ::UINT = BCM_FIRST + 0x000B;
pub const BCM_SETSHIELD: ::UINT = BCM_FIRST + 0x000C;
pub const BCCL_NOGLYPH: HIMAGELIST = (0 - 1) as HIMAGELIST;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMBCDROPDOWN {
    pub hdr: ::NMHDR,
    pub rcButton: ::RECT,
}
pub type LPNMBCDROPDOWN = *mut NMBCDROPDOWN;
pub const BCN_DROPDOWN: ::UINT = BCN_FIRST + 0x0002;
pub const EM_SETCUEBANNER: ::UINT = ECM_FIRST + 1;
pub const EM_GETCUEBANNER: ::UINT = ECM_FIRST + 2;
pub const EM_SHOWBALLOONTIP: ::UINT = ECM_FIRST + 3;
pub const EM_HIDEBALLOONTIP: ::UINT = ECM_FIRST + 4;
pub const EM_SETHILITE: ::UINT = ECM_FIRST + 5;
pub const EM_GETHILITE: ::UINT = ECM_FIRST + 6;
pub const EM_NOSETFOCUS: ::UINT = ECM_FIRST + 7;
pub const EM_TAKEFOCUS: ::UINT = ECM_FIRST + 8;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct EDITBALLOONTIP {
    pub cbStruct: ::DWORD,
    pub pszTitle: ::LPCWSTR,
    pub pszText: ::LPCWSTR,
    pub ttiIcon: ::INT,
}
pub type PEDITBALLOONTIP = *mut EDITBALLOONTIP;
pub const CB_SETMINVISIBLE: ::UINT = CBM_FIRST + 1;
pub const CB_GETMINVISIBLE: ::UINT = CBM_FIRST + 2;
pub const CB_SETCUEBANNER: ::UINT = CBM_FIRST + 3;
pub const CB_GETCUEBANNER: ::UINT = CBM_FIRST + 4;
pub type PFTASKDIALOGCALLBACK = Option<unsafe extern "system" fn(
    hwnd: ::HWND, msg: ::UINT, wParam: ::WPARAM, lParam: ::LPARAM, lpRefData: ::LONG_PTR,
) -> ::HRESULT>;
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum TASKDIALOG_FLAGS {
    TDF_ENABLE_HYPERLINKS = 0x0001,
    TDF_USE_HICON_MAIN = 0x0002,
    TDF_USE_HICON_FOOTER = 0x0004,
    TDF_ALLOW_DIALOG_CANCELLATION = 0x0008,
    TDF_USE_COMMAND_LINKS = 0x0010,
    TDF_USE_COMMAND_LINKS_NO_ICON = 0x0020,
    TDF_EXPAND_FOOTER_AREA = 0x0040,
    TDF_EXPANDED_BY_DEFAULT = 0x0080,
    TDF_VERIFICATION_FLAG_CHECKED = 0x0100,
    TDF_SHOW_PROGRESS_BAR = 0x0200,
    TDF_SHOW_MARQUEE_PROGRESS_BAR = 0x0400,
    TDF_CALLBACK_TIMER = 0x0800,
    TDF_POSITION_RELATIVE_TO_WINDOW = 0x1000,
    TDF_RTL_LAYOUT = 0x2000,
    TDF_NO_DEFAULT_RADIO_BUTTON = 0x4000,
    TDF_CAN_BE_MINIMIZED = 0x8000,
    TDF_NO_SET_FOREGROUND = 0x00010000,
    TDF_SIZE_TO_CONTENT = 0x01000000,
}
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum TASKDIALOG_MESSAGES {
    TDM_NAVIGATE_PAGE = ::WM_USER as i32 + 101,
    TDM_CLICK_BUTTON = ::WM_USER as i32 + 102,
    TDM_SET_MARQUEE_PROGRESS_BAR = ::WM_USER as i32 + 103,
    TDM_SET_PROGRESS_BAR_STATE = ::WM_USER as i32 + 104,
    TDM_SET_PROGRESS_BAR_RANGE = ::WM_USER as i32 + 105,
    TDM_SET_PROGRESS_BAR_POS = ::WM_USER as i32 + 106,
    TDM_SET_PROGRESS_BAR_MARQUEE = ::WM_USER as i32 + 107,
    TDM_SET_ELEMENT_TEXT = ::WM_USER as i32 + 108,
    TDM_CLICK_RADIO_BUTTON = ::WM_USER as i32 + 110,
    TDM_ENABLE_BUTTON = ::WM_USER as i32 + 111,
    TDM_ENABLE_RADIO_BUTTON = ::WM_USER as i32 + 112,
    TDM_CLICK_VERIFICATION = ::WM_USER as i32 + 113,
    TDM_UPDATE_ELEMENT_TEXT = ::WM_USER as i32 + 114,
    TDM_SET_BUTTON_ELEVATION_REQUIRED_STATE = ::WM_USER as i32 + 115,
    TDM_UPDATE_ICON = ::WM_USER as i32 + 116,
}
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum TASKDIALOG_NOTIFICATIONS {
    TDN_CREATED = 0,
    TDN_NAVIGATED = 1,
    TDN_BUTTON_CLICKED = 2,
    TDN_HYPERLINK_CLICKED = 3,
    TDN_TIMER = 4,
    TDN_DESTROYED = 5,
    TDN_RADIO_BUTTON_CLICKED = 6,
    TDN_DIALOG_CONSTRUCTED = 7,
    TDN_VERIFICATION_CLICKED = 8,
    TDN_HELP = 9,
    TDN_EXPANDO_BUTTON_CLICKED = 10,
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct TASKDIALOG_BUTTON {
    pub nButtonID: ::c_int,
    pub pszButtonText: ::PCWSTR,
}
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum TASKDIALOG_ELEMENTS {
    TDE_CONTENT,
    TDE_EXPANDED_INFORMATION,
    TDE_FOOTER,
    TDE_MAIN_INSTRUCTION,
}
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum TASKDIALOG_ICON_ELEMENTS {
    TDIE_ICON_MAIN,
    TDIE_ICON_FOOTER,
}
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum TASKDIALOG_COMMON_BUTTON_FLAGS {
    TDCBF_OK_BUTTON = 0x0001,
    TDCBF_YES_BUTTON = 0x0002,
    TDCBF_NO_BUTTON = 0x0004,
    TDCBF_CANCEL_BUTTON = 0x0008,
    TDCBF_RETRY_BUTTON = 0x0010,
    TDCBF_CLOSE_BUTTON = 0x0020,
}
#[repr(C)] #[derive(Copy)]
pub struct TASKDIALOGCONFIG {
    pub cbSize: ::UINT,
    pub hwndParent: ::HWND,
    pub hInstance: ::HINSTANCE,
    pub dwFlags: TASKDIALOG_FLAGS,
    pub dwCommonButtons: TASKDIALOG_COMMON_BUTTON_FLAGS,
    pub pszWindowTitle: ::PCWSTR,
    pub hMainIcon: ::HICON,
    pub pszMainInstruction: ::PCWSTR,
    pub pszContent: ::PCWSTR,
    pub cButtons: ::UINT,
    pub pButtons: *const TASKDIALOG_BUTTON,
    pub nDefaultButton: ::c_int,
    pub cRadioButtons: ::UINT,
    pub pRadioButtons: *const TASKDIALOG_BUTTON,
    pub nDefaultRadioButton: ::c_int,
    pub pszVerificationText: ::PCWSTR,
    pub pszExpandedInformation: ::PCWSTR,
    pub pszExpandedControlText: ::PCWSTR,
    pub pszCollapsedControlText: ::PCWSTR,
    pub hFooterIcon: ::HICON,
    pub pszFooter: ::PCWSTR,
    pub pfCallback: PFTASKDIALOGCALLBACK,
    pub lpCallbackData: ::LONG_PTR,
    pub cxWidth: ::UINT,
}
impl Clone for TASKDIALOGCONFIG { fn clone(&self) -> TASKDIALOGCONFIG { *self } }
UNION!(TASKDIALOGCONFIG, hMainIcon, pszMainIcon, pszMainIcon_mut, ::PCWSTR);
UNION!(TASKDIALOGCONFIG, hFooterIcon, pszFooterIcon, pszFooterIcon_mut, ::PCWSTR);
pub const DA_LAST: ::c_int = 0x7FFFFFFF;
pub const DA_ERR: ::c_int = -1;
pub type PFNDAENUMCALLBACK = Option<unsafe extern "system" fn(
    p: *mut ::c_void, pData: *mut ::c_void,
) -> ::c_int>;
pub type PFNDAENUMCALLBACKCONST = Option<unsafe extern "system" fn(
    p: *const ::c_void, pData: *mut ::c_void,
) -> ::c_int>;
pub type PFNDACOMPARE = Option<unsafe extern "system" fn(
    p1: *mut ::c_void, p2: *mut ::c_void, lParam: ::LPARAM,
) -> ::c_int>;
pub type PFNDACOMPARECONST = Option<unsafe extern "system" fn(
    p1: *const ::c_void, p2: *const ::c_void, lParam: ::LPARAM,
) -> ::c_int>;
#[repr(C)] #[allow(missing_copy_implementations)]
pub struct DSA {
    unused: ::c_void,
}
pub type HDSA = *mut DSA;
pub const DSA_APPEND: ::c_int = DA_LAST;
pub const DSA_ERR: ::c_int = DA_ERR;
pub type PFNDSAENUMCALLBACK = PFNDAENUMCALLBACK;
pub type PFNDSAENUMCALLBACKCONST = PFNDAENUMCALLBACKCONST;
pub type PFNDSACOMPARE = PFNDACOMPARE;
pub type PFNDSACOMPARECONST = PFNDACOMPARECONST;
#[repr(C)] #[allow(missing_copy_implementations)]
pub struct DPA {
    unused: ::c_void,
}
pub type HDPA = *mut DPA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DPASTREAMINFO {
    pub iPos: ::c_int,
    pub pvItem: *mut ::c_void,
}
pub type PFNDPASTREAM = Option<unsafe extern "system" fn(
    pinfo: *mut DPASTREAMINFO, pstream: *mut ::IStream, pvInstData: *mut ::c_void,
) -> ::HRESULT>;
pub const DPAM_SORTED: ::DWORD = 0x00000001;
pub const DPAM_NORMAL: ::DWORD = 0x00000002;
pub const DPAM_UNION: ::DWORD = 0x00000004;
pub const DPAM_INTERSECT: ::DWORD = 0x00000008;
pub type PFNDPAMERGE = Option<unsafe extern "system" fn(
    uMsg: ::UINT, pvDest: *mut ::c_void, pvSrc: *mut ::c_void, lParam: ::LPARAM,
) -> *mut ::c_void>;
pub type PFNDPAMERGECONST = Option<unsafe extern "system" fn(
    uMsg: ::UINT, pvDest: *const ::c_void, pvSrc: *const ::c_void, lParam: ::LPARAM,
) -> *const ::c_void>;
pub const DPAMM_MERGE: ::UINT = 1;
pub const DPAMM_DELETE: ::UINT = 2;
pub const DPAMM_INSERT: ::UINT = 3;
pub const DPAS_SORTED: ::UINT = 0x0001;
pub const DPAS_INSERTBEFORE: ::UINT = 0x0002;
pub const DPAS_INSERTAFTER: ::UINT = 0x0004;
pub const DPA_APPEND: ::c_int = DA_LAST;
pub const DPA_ERR: ::c_int = DA_ERR;
pub type PFNDPAENUMCALLBACK = PFNDAENUMCALLBACK;
pub type PFNDPAENUMCALLBACKCONST = PFNDAENUMCALLBACKCONST;
pub type PFNDPACOMPARE = PFNDACOMPARE;
pub type PFNDPACOMPARECONST = PFNDACOMPARECONST;
pub const WSB_PROP_CYVSCROLL: ::UINT = 0x00000001;
pub const WSB_PROP_CXHSCROLL: ::UINT = 0x00000002;
pub const WSB_PROP_CYHSCROLL: ::UINT = 0x00000004;
pub const WSB_PROP_CXVSCROLL: ::UINT = 0x00000008;
pub const WSB_PROP_CXHTHUMB: ::UINT = 0x00000010;
pub const WSB_PROP_CYVTHUMB: ::UINT = 0x00000020;
pub const WSB_PROP_VBKGCOLOR: ::UINT = 0x00000040;
pub const WSB_PROP_HBKGCOLOR: ::UINT = 0x00000080;
pub const WSB_PROP_VSTYLE: ::UINT = 0x00000100;
pub const WSB_PROP_HSTYLE: ::UINT = 0x00000200;
pub const WSB_PROP_WINSTYLE: ::UINT = 0x00000400;
pub const WSB_PROP_PALETTE: ::UINT = 0x00000800;
pub const WSB_PROP_MASK: ::UINT = 0x00000FFF;
pub const FSB_FLAT_MODE: ::INT_PTR = 2;
pub const FSB_ENCARTA_MODE: ::INT_PTR = 1;
pub const FSB_REGULAR_MODE: ::INT_PTR = 0;
pub type SUBCLASSPROC = Option<unsafe extern "system" fn(
    hWnd: ::HWND, uMsg: ::UINT, wParam: ::WPARAM, lParam: ::LPARAM, uIdSubclass: ::UINT_PTR,
    dwRefData: ::DWORD_PTR,
) -> ::LRESULT>;

// Copyright © 2015, skdltmxn
// Licensed under the MIT License <LICENSE.md>
//! 32-Bit Common Dialog APIs
pub type LPOFNHOOKPROC = Option<unsafe extern "system" fn(
    ::HWND, ::UINT, ::WPARAM, ::LPARAM,
) -> ::UINT_PTR>;
#[repr(C)] #[derive(Copy)]
pub struct OPENFILENAME_NT4A {
    pub lStructSize: ::DWORD,
    pub hwndOwner: ::HWND,
    pub hInstance: ::HINSTANCE,
    pub lpstrFilter: ::LPCSTR,
    pub lpstrCustomFilter: ::LPSTR,
    pub nMaxCustFilter: ::DWORD,
    pub nFilterIndex: ::DWORD,
    pub lpstrFile: ::LPSTR,
    pub nMaxFile: ::DWORD,
    pub lpstrFileTitle: ::LPSTR,
    pub nMaxFileTitle: ::DWORD,
    pub lpstrInitialDir: ::LPCSTR,
    pub lpstrTitle: ::LPCSTR,
    pub Flags: ::DWORD,
    pub nFileOffset: ::WORD,
    pub nFileExtension: ::WORD,
    pub lpstrDefExt: ::LPCSTR,
    pub lCustData: ::LPARAM,
    pub lpfnHook: LPOFNHOOKPROC,
    pub lpTemplateName: ::LPCSTR,
}
impl Clone for OPENFILENAME_NT4A { fn clone(&self) -> OPENFILENAME_NT4A { *self } }
pub type LPOPENFILENAME_NT4A = *mut OPENFILENAME_NT4A;
#[repr(C)] #[derive(Copy)]
pub struct OPENFILENAME_NT4W {
    pub lStructSize: ::DWORD,
    pub hwndOwner: ::HWND,
    pub hInstance: ::HINSTANCE,
    pub lpstrFilter: ::LPCWSTR,
    pub lpstrCustomFilter: ::LPWSTR,
    pub nMaxCustFilter: ::DWORD,
    pub nFilterIndex: ::DWORD,
    pub lpstrFile: ::LPWSTR,
    pub nMaxFile: ::DWORD,
    pub lpstrFileTitle: ::LPWSTR,
    pub nMaxFileTitle: ::DWORD,
    pub lpstrInitialDir: ::LPCWSTR,
    pub lpstrTitle: ::LPCWSTR,
    pub Flags: ::DWORD,
    pub nFileOffset: ::WORD,
    pub nFileExtension: ::WORD,
    pub lpstrDefExt: ::LPCWSTR,
    pub lCustData: ::LPARAM,
    pub lpfnHook: LPOFNHOOKPROC,
    pub lpTemplateName: ::LPCWSTR,
}
impl Clone for OPENFILENAME_NT4W { fn clone(&self) -> OPENFILENAME_NT4W { *self } }
pub type LPOPENFILENAME_NT4W = *mut OPENFILENAME_NT4W;
#[repr(C)] #[derive(Copy)]
pub struct OPENFILENAMEA {
    pub lStructSize: ::DWORD,
    pub hwndOwner: ::HWND,
    pub hInstance: ::HINSTANCE,
    pub lpstrFilter: ::LPCSTR,
    pub lpstrCustomFilter: ::LPSTR,
    pub nMaxCustFilter: ::DWORD,
    pub nFilterIndex: ::DWORD,
    pub lpstrFile: ::LPSTR,
    pub nMaxFile: ::DWORD,
    pub lpstrFileTitle: ::LPSTR,
    pub nMaxFileTitle: ::DWORD,
    pub lpstrInitialDir: ::LPCSTR,
    pub lpstrTitle: ::LPCSTR,
    pub Flags: ::DWORD,
    pub nFileOffset: ::WORD,
    pub nFileExtension: ::WORD,
    pub lpstrDefExt: ::LPCSTR,
    pub lCustData: ::LPARAM,
    pub lpfnHook: LPOFNHOOKPROC,
    pub lpTemplateName: ::LPCSTR,
    pub pvReserved: *mut ::c_void,
    pub dwReserved: ::DWORD,
    pub FlagsEx: ::DWORD,
}
impl Clone for OPENFILENAMEA { fn clone(&self) -> OPENFILENAMEA { *self } }
pub type LPOPENFILENAMEA = *mut OPENFILENAMEA;
#[repr(C)] #[derive(Copy)]
pub struct OPENFILENAMEW {
    pub lStructSize: ::DWORD,
    pub hwndOwner: ::HWND,
    pub hInstance: ::HINSTANCE,
    pub lpstrFilter: ::LPCWSTR,
    pub lpstrCustomFilter: ::LPWSTR,
    pub nMaxCustFilter: ::DWORD,
    pub nFilterIndex: ::DWORD,
    pub lpstrFile: ::LPWSTR,
    pub nMaxFile: ::DWORD,
    pub lpstrFileTitle: ::LPWSTR,
    pub nMaxFileTitle: ::DWORD,
    pub lpstrInitialDir: ::LPCWSTR,
    pub lpstrTitle: ::LPCWSTR,
    pub Flags: ::DWORD,
    pub nFileOffset: ::WORD,
    pub nFileExtension: ::WORD,
    pub lpstrDefExt: ::LPCWSTR,
    pub lCustData: ::LPARAM,
    pub lpfnHook: LPOFNHOOKPROC,
    pub lpTemplateName: ::LPCWSTR,
    pub pvReserved: *mut ::c_void,
    pub dwReserved: ::DWORD,
    pub FlagsEx: ::DWORD,
}
impl Clone for OPENFILENAMEW { fn clone(&self) -> OPENFILENAMEW { *self } }
pub type LPOPENFILENAMEW = *mut OPENFILENAMEW;
pub const OFN_READONLY: ::DWORD = 0x00000001;
pub const OFN_OVERWRITEPROMPT: ::DWORD = 0x00000002;
pub const OFN_HIDEREADONLY: ::DWORD = 0x00000004;
pub const OFN_NOCHANGEDIR: ::DWORD = 0x00000008;
pub const OFN_SHOWHELP: ::DWORD = 0x00000010;
pub const OFN_ENABLEHOOK: ::DWORD = 0x00000020;
pub const OFN_ENABLETEMPLATE: ::DWORD = 0x00000040;
pub const OFN_ENABLETEMPLATEHANDLE: ::DWORD = 0x00000080;
pub const OFN_NOVALIDATE: ::DWORD = 0x00000100;
pub const OFN_ALLOWMULTISELECT: ::DWORD = 0x00000200;
pub const OFN_EXTENSIONDIFFERENT: ::DWORD = 0x00000400;
pub const OFN_PATHMUSTEXIST: ::DWORD = 0x00000800;
pub const OFN_FILEMUSTEXIST: ::DWORD = 0x00001000;
pub const OFN_CREATEPROMPT: ::DWORD = 0x00002000;
pub const OFN_SHAREAWARE: ::DWORD = 0x00004000;
pub const OFN_NOREADONLYRETURN: ::DWORD = 0x00008000;
pub const OFN_NOTESTFILECREATE: ::DWORD = 0x00010000;
pub const OFN_NONETWORKBUTTON: ::DWORD = 0x00020000;
pub const OFN_NOLONGNAMES: ::DWORD = 0x00040000;
pub const OFN_EXPLORER: ::DWORD = 0x00080000;
pub const OFN_NODEREFERENCELINKS: ::DWORD = 0x00100000;
pub const OFN_LONGNAMES: ::DWORD = 0x00200000;
pub const OFN_ENABLEINCLUDENOTIFY: ::DWORD = 0x00400000;
pub const OFN_ENABLESIZING: ::DWORD = 0x00800000;
pub const OFN_DONTADDTORECENT: ::DWORD = 0x02000000;
pub const OFN_FORCESHOWHIDDEN: ::DWORD = 0x10000000;
pub const OFN_EX_NOPLACESBAR: ::DWORD = 0x00000001;
pub const OFN_SHAREFALLTHROUGH: ::UINT_PTR = 2;
pub const OFN_SHARENOWARN: ::UINT_PTR = 1;
pub const OFN_SHAREWARN: ::UINT_PTR = 0;
pub type LPCCHOOKPROC = Option<unsafe extern "system" fn(
    ::HWND, ::UINT, ::WPARAM, ::LPARAM,
) -> ::UINT_PTR>;
STRUCT!{struct OFNOTIFYA {
    hdr: ::NMHDR,
    lpOFN: LPOPENFILENAMEA,
    pszFile: ::LPSTR,
}}
pub type LPOFNOTIFYA = *mut OFNOTIFYA;
STRUCT!{struct OFNOTIFYW {
    hdr: ::NMHDR,
    lpOFN: LPOPENFILENAMEW,
    pszFile: ::LPWSTR,
}}
pub type LPOFNOTIFYW = *mut OFNOTIFYW;
STRUCT!{struct OFNOTIFYEXA {
    hdr: ::NMHDR,
    lpOFN: LPOPENFILENAMEA,
    psf: ::LPVOID,
    pidl: ::LPVOID,
}}
pub type LPOFNOTIFYEXA = *mut OFNOTIFYEXA;
STRUCT!{struct OFNOTIFYEXW {
    hdr: ::NMHDR,
    lpOFN: LPOPENFILENAMEW,
    psf: ::LPVOID,
    pidl: ::LPVOID,
}}
pub type LPOFNOTIFYEXW = *mut OFNOTIFYEXW;
pub const CDN_FIRST: ::UINT = 0 - 601;
pub const CDN_LAST: ::UINT = 0 - 699;
pub const CDN_INITDONE: ::UINT = CDN_FIRST - 0x0000;
pub const CDN_SELCHANGE: ::UINT = CDN_FIRST - 0x0001;
pub const CDN_FOLDERCHANGE: ::UINT = CDN_FIRST - 0x0002;
pub const CDN_SHAREVIOLATION: ::UINT = CDN_FIRST - 0x0003;
pub const CDN_HELP: ::UINT = CDN_FIRST - 0x0004;
pub const CDN_FILEOK: ::UINT = CDN_FIRST - 0x0005;
pub const CDN_TYPECHANGE: ::UINT = CDN_FIRST - 0x0006;
pub const CDN_INCLUDEITEM: ::UINT = CDN_FIRST - 0x0007;
pub const CDM_FIRST: ::UINT = ::WM_USER + 100;
pub const CDM_LAST: ::UINT = ::WM_USER + 200;
pub const CDM_GETSPEC: ::UINT = CDM_FIRST + 0x0000;
pub const CDM_GETFILEPATH: ::UINT = CDM_FIRST + 0x0001;
pub const CDM_GETFOLDERPATH: ::UINT = CDM_FIRST + 0x0002;
pub const CDM_GETFOLDERIDLIST: ::UINT = CDM_FIRST + 0x0003;
pub const CDM_SETCONTROLTEXT: ::UINT = CDM_FIRST + 0x0004;
pub const CDM_HIDECONTROL: ::UINT = CDM_FIRST + 0x0005;
pub const CDM_SETDEFEXT: ::UINT = CDM_FIRST + 0x0006;
#[repr(C)] #[derive(Copy)]
pub struct CHOOSECOLORA {
    pub lStructSize: ::DWORD,
    pub hwndOwner: ::HWND,
    pub hInstance: ::HWND,
    pub rgbResult: ::COLORREF,
    pub lpCustColors: *mut ::COLORREF,
    pub Flags: ::DWORD,
    pub lCustData: ::LPARAM,
    pub lpfnHook: LPCCHOOKPROC,
    pub lpTemplateName: ::LPCSTR,
}
impl Clone for CHOOSECOLORA { fn clone(&self) -> CHOOSECOLORA { *self } }
pub type LPCHOOSECOLORA = *mut CHOOSECOLORA;
#[repr(C)] #[derive(Copy)]
pub struct CHOOSECOLORW {
    pub lStructSize: ::DWORD,
    pub hwndOwner: ::HWND,
    pub hInstance: ::HWND,
    pub rgbResult: ::COLORREF,
    pub lpCustColors: *mut ::COLORREF,
    pub Flags: ::DWORD,
    pub lCustData: ::LPARAM,
    pub lpfnHook: LPCCHOOKPROC,
    pub lpTemplateName: ::LPCWSTR,
}
impl Clone for CHOOSECOLORW { fn clone(&self) -> CHOOSECOLORW { *self } }
pub type LPCHOOSECOLORW = *mut CHOOSECOLORW;
pub const CC_RGBINIT: ::DWORD = 0x00000001;
pub const CC_FULLOPEN: ::DWORD = 0x00000002;
pub const CC_PREVENTFULLOPEN: ::DWORD = 0x00000004;
pub const CC_SHOWHELP: ::DWORD = 0x00000008;
pub const CC_ENABLEHOOK: ::DWORD = 0x00000010;
pub const CC_ENABLETEMPLATE: ::DWORD = 0x00000020;
pub const CC_ENABLETEMPLATEHANDLE: ::DWORD = 0x00000040;
pub const CC_SOLIDCOLOR: ::DWORD = 0x00000080;
pub const CC_ANYCOLOR: ::DWORD = 0x00000100;
pub type LPFRHOOKPROC = Option<unsafe extern "system" fn(
    ::HWND, ::UINT, ::WPARAM, ::LPARAM,
) -> ::UINT_PTR>;
#[repr(C)] #[derive(Copy)]
pub struct FINDREPLACEA {
    pub lStructSize: ::DWORD,
    pub hwndOwner: ::HWND,
    pub hInstance: ::HINSTANCE,
    pub Flags: ::DWORD,
    pub lpstrFindWhat: ::LPSTR,
    pub lpstrReplaceWith: ::LPSTR,
    pub wFindWhatLen: ::WORD,
    pub wReplaceWithLen: ::WORD,
    pub lCustData: ::LPARAM,
    pub lpfnHook: LPFRHOOKPROC,
    pub lpTemplateName: ::LPCSTR,
}
impl Clone for FINDREPLACEA { fn clone(&self) -> FINDREPLACEA { *self } }
pub type LPFINDREPLACEA = *mut FINDREPLACEA;
#[repr(C)] #[derive(Copy)]
pub struct FINDREPLACEW {
    pub lStructSize: ::DWORD,
    pub hwndOwner: ::HWND,
    pub hInstance: ::HINSTANCE,
    pub Flags: ::DWORD,
    pub lpstrFindWhat: ::LPWSTR,
    pub lpstrReplaceWith: ::LPWSTR,
    pub wFindWhatLen: ::WORD,
    pub wReplaceWithLen: ::WORD,
    pub lCustData: ::LPARAM,
    pub lpfnHook: LPFRHOOKPROC,
    pub lpTemplateName: ::LPCWSTR,
}
impl Clone for FINDREPLACEW { fn clone(&self) -> FINDREPLACEW { *self } }
pub type LPFINDREPLACEW = *mut FINDREPLACEW;
pub const FR_DOWN: ::DWORD = 0x00000001;
pub const FR_WHOLEWORD: ::DWORD = 0x00000002;
pub const FR_MATCHCASE: ::DWORD = 0x00000004;
pub const FR_FINDNEXT: ::DWORD = 0x00000008;
pub const FR_REPLACE: ::DWORD = 0x00000010;
pub const FR_REPLACEALL: ::DWORD = 0x00000020;
pub const FR_DIALOGTERM: ::DWORD = 0x00000040;
pub const FR_SHOWHELP: ::DWORD = 0x00000080;
pub const FR_ENABLEHOOK: ::DWORD = 0x00000100;
pub const FR_ENABLETEMPLATE: ::DWORD = 0x00000200;
pub const FR_NOUPDOWN: ::DWORD = 0x00000400;
pub const FR_NOMATCHCASE: ::DWORD = 0x00000800;
pub const FR_NOWHOLEWORD: ::DWORD = 0x00001000;
pub const FR_ENABLETEMPLATEHANDLE: ::DWORD = 0x00002000;
pub const FR_HIDEUPDOWN: ::DWORD = 0x00004000;
pub const FR_HIDEMATCHCASE: ::DWORD = 0x00008000;
pub const FR_HIDEWHOLEWORD: ::DWORD = 0x00010000;
pub const FR_RAW: ::DWORD = 0x00020000;
pub const FR_MATCHDIAC: ::DWORD = 0x20000000;
pub const FR_MATCHKASHIDA: ::DWORD = 0x40000000;
pub const FR_MATCHALEFHAMZA: ::DWORD = 0x80000000;
pub type LPCFHOOKPROC = Option<unsafe extern "system" fn(
    ::HWND, ::UINT, ::WPARAM, ::LPARAM,
) -> ::UINT_PTR>;
#[repr(C)] #[derive(Copy)]
pub struct CHOOSEFONTA {
    pub lStructSize: ::DWORD,
    pub hwndOwner: ::HWND,
    pub hDC: ::HDC,
    pub lpLogFont: ::LPLOGFONTA,
    pub iPointSize: ::INT,
    pub Flags: ::DWORD,
    pub rgbColors: ::COLORREF,
    pub lCustData: ::LPARAM,
    pub lpfnHook: LPCFHOOKPROC,
    pub lpTemplateName: ::LPCSTR,
    pub hInstance: ::HINSTANCE,
    pub lpszStyle: ::LPSTR,
    pub nFontType: ::WORD,
    pub ___MISSING_ALIGNMENT__: ::WORD,
    pub nSizeMin: ::INT,
    pub nSizeMax: ::INT,
}
impl Clone for CHOOSEFONTA { fn clone(&self) -> CHOOSEFONTA { *self } }
pub type LPCHOOSEFONTA = *mut CHOOSEFONTA;
#[repr(C)] #[derive(Copy)]
pub struct CHOOSEFONTW {
    pub lStructSize: ::DWORD,
    pub hwndOwner: ::HWND,
    pub hDC: ::HDC,
    pub lpLogFont: ::LPLOGFONTW,
    pub iPointSize: ::INT,
    pub Flags: ::DWORD,
    pub rgbColors: ::COLORREF,
    pub lCustData: ::LPARAM,
    pub lpfnHook: LPCFHOOKPROC,
    pub lpTemplateName: ::LPCWSTR,
    pub hInstance: ::HINSTANCE,
    pub lpszStyle: ::LPWSTR,
    pub nFontType: ::WORD,
    pub ___MISSING_ALIGNMENT__: ::WORD,
    pub nSizeMin: ::INT,
    pub nSizeMax: ::INT,
}
impl Clone for CHOOSEFONTW { fn clone(&self) -> CHOOSEFONTW { *self } }
pub type LPCHOOSEFONTW = *mut CHOOSEFONTW;
pub const CF_SCREENFONTS: ::DWORD = 0x00000001;
pub const CF_PRINTERFONTS: ::DWORD = 0x00000002;
pub const CF_BOTH: ::DWORD = CF_SCREENFONTS | CF_PRINTERFONTS;
pub const CF_SHOWHELP: ::DWORD = 0x00000004;
pub const CF_ENABLEHOOK: ::DWORD = 0x00000008;
pub const CF_ENABLETEMPLATE: ::DWORD = 0x00000010;
pub const CF_ENABLETEMPLATEHANDLE: ::DWORD = 0x00000020;
pub const CF_INITTOLOGFONTSTRUCT: ::DWORD = 0x00000040;
pub const CF_USESTYLE: ::DWORD = 0x00000080;
pub const CF_EFFECTS: ::DWORD = 0x00000100;
pub const CF_APPLY: ::DWORD = 0x00000200;
pub const CF_ANSIONLY: ::DWORD = 0x00000400;
pub const CF_SCRIPTSONLY: ::DWORD = CF_ANSIONLY;
pub const CF_NOVECTORFONTS: ::DWORD = 0x00000800;
pub const CF_NOOEMFONTS: ::DWORD = CF_NOVECTORFONTS;
pub const CF_NOSIMULATIONS: ::DWORD = 0x00001000;
pub const CF_LIMITSIZE: ::DWORD = 0x00002000;
pub const CF_FIXEDPITCHONLY: ::DWORD = 0x00004000;
pub const CF_WYSIWYG: ::DWORD = 0x00008000;
pub const CF_FORCEFONTEXIST: ::DWORD = 0x00010000;
pub const CF_SCALABLEONLY: ::DWORD = 0x00020000;
pub const CF_TTONLY: ::DWORD = 0x00040000;
pub const CF_NOFACESEL: ::DWORD = 0x00080000;
pub const CF_NOSTYLESEL: ::DWORD = 0x00100000;
pub const CF_NOSIZESEL: ::DWORD = 0x00200000;
pub const CF_SELECTSCRIPT: ::DWORD = 0x00400000;
pub const CF_NOSCRIPTSEL: ::DWORD = 0x00800000;
pub const CF_NOVERTFONTS: ::DWORD = 0x01000000;
pub const CF_INACTIVEFONTS: ::DWORD = 0x02000000;
pub const SIMULATED_FONTTYPE: ::WORD = 0x8000;
pub const PRINTER_FONTTYPE: ::WORD = 0x4000;
pub const SCREEN_FONTTYPE: ::WORD = 0x2000;
pub const BOLD_FONTTYPE: ::WORD = 0x0100;
pub const ITALIC_FONTTYPE: ::WORD = 0x0200;
pub const REGULAR_FONTTYPE: ::WORD = 0x0400;
pub const PS_OPENTYPE_FONTTYPE: ::DWORD = 0x10000;
pub const TT_OPENTYPE_FONTTYPE: ::DWORD = 0x20000;
pub const TYPE1_FONTTYPE: ::DWORD = 0x40000;
pub const SYMBOL_FONTTYPE: ::DWORD = 0x80000;
pub const WM_CHOOSEFONT_GETLOGFONT: ::UINT = ::WM_USER + 1;
pub const WM_CHOOSEFONT_SETLOGFONT: ::UINT = ::WM_USER + 101;
pub const WM_CHOOSEFONT_SETFLAGS: ::UINT = ::WM_USER + 102;
pub const CD_LBSELNOITEMS: ::WORD = 0 - 1;
pub const CD_LBSELCHANGE: ::WORD = 0;
pub const CD_LBSELSUB: ::WORD = 1;
pub const CD_LBSELADD: ::WORD = 2;
pub type LPPRINTHOOKPROC = Option<unsafe extern "system" fn(
    ::HWND, ::UINT, ::WPARAM, ::LPARAM,
) -> ::UINT_PTR>;
pub type LPSETUPHOOKPROC = Option<unsafe extern "system" fn(
    ::HWND, ::UINT, ::WPARAM, ::LPARAM,
) -> ::UINT_PTR>;
#[repr(C)] #[derive(Copy)]
pub struct PRINTDLGA {
    pub lStructSize: ::DWORD,
    pub hwndOwner: ::HWND,
    pub hDevMode: ::HGLOBAL,
    pub hDevNames: ::HGLOBAL,
    pub hDC: ::HDC,
    pub Flags: ::DWORD,
    pub nFromPage: ::WORD,
    pub nToPage: ::WORD,
    pub nMinPage: ::WORD,
    pub nMaxPage: ::WORD,
    pub nCopies: ::WORD,
    pub hInstance: ::HINSTANCE,
    pub lCustData: ::LPARAM,
    pub lpfnPrintHook: LPPRINTHOOKPROC,
    pub lpfnSetupHook: LPSETUPHOOKPROC,
    pub lpPrintTemplateName: ::LPCSTR,
    pub lpSetupTemplateName: ::LPCSTR,
    pub hPrintTemplate: ::HGLOBAL,
    pub hSetupTemplate: ::HGLOBAL,
}
impl Clone for PRINTDLGA { fn clone(&self) -> PRINTDLGA { *self } }
pub type LPPRINTDLGA = *mut PRINTDLGA;
#[repr(C)] #[derive(Copy)]
pub struct PRINTDLGW {
    pub lStructSize: ::DWORD,
    pub hwndOwner: ::HWND,
    pub hDevMode: ::HGLOBAL,
    pub hDevNames: ::HGLOBAL,
    pub hDC: ::HDC,
    pub Flags: ::DWORD,
    pub nFromPage: ::WORD,
    pub nToPage: ::WORD,
    pub nMinPage: ::WORD,
    pub nMaxPage: ::WORD,
    pub nCopies: ::WORD,
    pub hInstance: ::HINSTANCE,
    pub lCustData: ::LPARAM,
    pub lpfnPrintHook: LPPRINTHOOKPROC,
    pub lpfnSetupHook: LPSETUPHOOKPROC,
    pub lpPrintTemplateName: ::LPCWSTR,
    pub lpSetupTemplateName: ::LPCWSTR,
    pub hPrintTemplate: ::HGLOBAL,
    pub hSetupTemplate: ::HGLOBAL,
}
impl Clone for PRINTDLGW { fn clone(&self) -> PRINTDLGW { *self } }
pub type LPPRINTDLGW = *mut PRINTDLGW;
RIDL!(
interface IPrintDialogCallback(IPrintDialogCallbackVtbl) : IUnknown(IUnknownVtbl) {
    fn InitDone(&mut self) -> ::HRESULT,
    fn SelectionChange(&mut self) -> ::HRESULT,
    fn HandleMessage(
        &mut self, hDlg: ::HWND, uMsg: ::UINT, wParam: ::WPARAM, lParam: ::LPARAM,
        pResult: *mut ::LRESULT
    ) -> ::HRESULT
}
);
RIDL!(
interface IPrintDialogServices(IPrintDialogServicesVtbl) : IUnknown(IUnknownVtbl) {
    fn GetCurrentDevMode(&mut self, pDevMode: ::LPDEVMODEW, pcbSize: *mut ::UINT) -> ::HRESULT,
    fn GetCurrentPrinterName(&mut self, pPrinterName: ::LPWSTR, pcchSize: *mut ::UINT) -> ::HRESULT,
    fn GetCurrentPortName(&mut self, pPortName: ::LPWSTR, pcchSize: *mut ::UINT) -> ::HRESULT
}
);
STRUCT!{struct PRINTPAGERANGE {
    nFromPage: ::DWORD,
    nToPage: ::DWORD,
}}
pub type LPPRINTPAGERANGE = *mut PRINTPAGERANGE;
pub type PCPRINTPAGERANGE = *const PRINTPAGERANGE;
STRUCT!{struct PRINTDLGEXA {
    lStructSize: ::DWORD,
    hwndOwner: ::HWND,
    hDevMode: ::HGLOBAL,
    hDevNames: ::HGLOBAL,
    hDC: ::HDC,
    Flags: ::DWORD,
    Flags2: ::DWORD,
    ExclusionFlags: ::DWORD,
    nPageRanges: ::DWORD,
    nMaxPageRanges: ::DWORD,
    lpPageRanges: LPPRINTPAGERANGE,
    nMinPage: ::DWORD,
    nMaxPage: ::DWORD,
    nCopies: ::DWORD,
    hInstance: ::HINSTANCE,
    lpPrintTemplateName: ::LPCSTR,
    lpCallback: ::LPUNKNOWN,
    nPropertyPages: ::DWORD,
    lphPropertyPages: *mut ::HPROPSHEETPAGE,
    nStartPage: ::DWORD,
    dwResultAction: ::DWORD,
}}
pub type LPPRINTDLGEXA = *mut PRINTDLGEXA;
STRUCT!{struct PRINTDLGEXW {
    lStructSize: ::DWORD,
    hwndOwner: ::HWND,
    hDevMode: ::HGLOBAL,
    hDevNames: ::HGLOBAL,
    hDC: ::HDC,
    Flags: ::DWORD,
    Flags2: ::DWORD,
    ExclusionFlags: ::DWORD,
    nPageRanges: ::DWORD,
    nMaxPageRanges: ::DWORD,
    lpPageRanges: LPPRINTPAGERANGE,
    nMinPage: ::DWORD,
    nMaxPage: ::DWORD,
    nCopies: ::DWORD,
    hInstance: ::HINSTANCE,
    lpPrintTemplateName: ::LPCWSTR,
    lpCallback: ::LPUNKNOWN,
    nPropertyPages: ::DWORD,
    lphPropertyPages: *mut ::HPROPSHEETPAGE,
    nStartPage: ::DWORD,
    dwResultAction: ::DWORD,
}}
pub type LPPRINTDLGEXW = *mut PRINTDLGEXW;
pub const PD_ALLPAGES: ::DWORD = 0x00000000;
pub const PD_SELECTION: ::DWORD = 0x00000001;
pub const PD_PAGENUMS: ::DWORD = 0x00000002;
pub const PD_NOSELECTION: ::DWORD = 0x00000004;
pub const PD_NOPAGENUMS: ::DWORD = 0x00000008;
pub const PD_COLLATE: ::DWORD = 0x00000010;
pub const PD_PRINTTOFILE: ::DWORD = 0x00000020;
pub const PD_PRINTSETUP: ::DWORD = 0x00000040;
pub const PD_NOWARNING: ::DWORD = 0x00000080;
pub const PD_RETURNDC: ::DWORD = 0x00000100;
pub const PD_RETURNIC: ::DWORD = 0x00000200;
pub const PD_RETURNDEFAULT: ::DWORD = 0x00000400;
pub const PD_SHOWHELP: ::DWORD = 0x00000800;
pub const PD_ENABLEPRINTHOOK: ::DWORD = 0x00001000;
pub const PD_ENABLESETUPHOOK: ::DWORD = 0x00002000;
pub const PD_ENABLEPRINTTEMPLATE: ::DWORD = 0x00004000;
pub const PD_ENABLESETUPTEMPLATE: ::DWORD = 0x00008000;
pub const PD_ENABLEPRINTTEMPLATEHANDLE: ::DWORD = 0x00010000;
pub const PD_ENABLESETUPTEMPLATEHANDLE: ::DWORD = 0x00020000;
pub const PD_USEDEVMODECOPIES: ::DWORD = 0x00040000;
pub const PD_USEDEVMODECOPIESANDCOLLATE: ::DWORD = 0x00040000;
pub const PD_DISABLEPRINTTOFILE: ::DWORD = 0x00080000;
pub const PD_HIDEPRINTTOFILE: ::DWORD = 0x00100000;
pub const PD_NONETWORKBUTTON: ::DWORD = 0x00200000;
pub const PD_CURRENTPAGE: ::DWORD = 0x00400000;
pub const PD_NOCURRENTPAGE: ::DWORD = 0x00800000;
pub const PD_EXCLUSIONFLAGS: ::DWORD = 0x01000000;
pub const PD_USELARGETEMPLATE: ::DWORD = 0x10000000;
pub const PD_EXCL_COPIESANDCOLLATE: ::DWORD = ::DM_COPIES | ::DM_COLLATE;
pub const START_PAGE_GENERAL: ::DWORD = 0xffffffff;
pub const PD_RESULT_CANCEL: ::DWORD = 0;
pub const PD_RESULT_PRINT: ::DWORD = 1;
pub const PD_RESULT_APPLY: ::DWORD = 2;
STRUCT!{struct DEVNAMES {
    wDriverOffset: ::WORD,
    wDeviceOffset: ::WORD,
    wOutputOffset: ::WORD,
    wDefault: ::WORD,
}}
pub type LPDEVNAMES = *mut DEVNAMES;
pub type PCDEVNAMES = *const DEVNAMES;
pub const DN_DEFAULTPRN: ::WORD = 0x0001;
pub const WM_PSD_PAGESETUPDLG: ::UINT = ::WM_USER;
pub const WM_PSD_FULLPAGERECT: ::UINT = ::WM_USER + 1;
pub const WM_PSD_MINMARGINRECT: ::UINT = ::WM_USER + 2;
pub const WM_PSD_MARGINRECT: ::UINT = ::WM_USER + 3;
pub const WM_PSD_GREEKTEXTRECT: ::UINT = ::WM_USER + 4;
pub const WM_PSD_ENVSTAMPRECT: ::UINT = ::WM_USER + 5;
pub const WM_PSD_YAFULLPAGERECT: ::UINT = ::WM_USER + 6;
pub type LPPAGEPAINTHOOK = Option<unsafe extern "system" fn(
    ::HWND, ::UINT, ::WPARAM, ::LPARAM,
) -> ::UINT_PTR>;
pub type LPPAGESETUPHOOK = Option<unsafe extern "system" fn(
    ::HWND, ::UINT, ::WPARAM, ::LPARAM,
) -> ::UINT_PTR>;
#[repr(C)] #[derive(Copy)]
pub struct PAGESETUPDLGA {
    pub lStructSize: ::DWORD,
    pub hwndOwner: ::HWND,
    pub hDevMode: ::HGLOBAL,
    pub hDevNames: ::HGLOBAL,
    pub Flags: ::DWORD,
    pub ptPaperSize: ::POINT,
    pub rtMinMargin: ::RECT,
    pub rtMargin: ::RECT,
    pub hInstance: ::HINSTANCE,
    pub lCustData: ::LPARAM,
    pub lpfnPageSetupHook: LPPAGESETUPHOOK,
    pub lpfnPagePaintHook: LPPAGEPAINTHOOK,
    pub lpPageSetupTemplateName: ::LPCSTR,
    pub hPageSetupTemplate: ::HGLOBAL,
}
impl Clone for PAGESETUPDLGA { fn clone(&self) -> PAGESETUPDLGA { *self } }
pub type LPPAGESETUPDLGA = *mut PAGESETUPDLGA;
#[repr(C)] #[derive(Copy)]
pub struct PAGESETUPDLGW {
    pub lStructSize: ::DWORD,
    pub hwndOwner: ::HWND,
    pub hDevMode: ::HGLOBAL,
    pub hDevNames: ::HGLOBAL,
    pub Flags: ::DWORD,
    pub ptPaperSize: ::POINT,
    pub rtMinMargin: ::RECT,
    pub rtMargin: ::RECT,
    pub hInstance: ::HINSTANCE,
    pub lCustData: ::LPARAM,
    pub lpfnPageSetupHook: LPPAGESETUPHOOK,
    pub lpfnPagePaintHook: LPPAGEPAINTHOOK,
    pub lpPageSetupTemplateName: ::LPCWSTR,
    pub hPageSetupTemplate: ::HGLOBAL,
}
impl Clone for PAGESETUPDLGW { fn clone(&self) -> PAGESETUPDLGW { *self } }
pub type LPPAGESETUPDLGW = *mut PAGESETUPDLGW;
pub const PSD_DEFAULTMINMARGINS: ::DWORD = 0x00000000;
pub const PSD_INWININIINTLMEASURE: ::DWORD = 0x00000000;
pub const PSD_MINMARGINS: ::DWORD = 0x00000001;
pub const PSD_MARGINS: ::DWORD = 0x00000002;
pub const PSD_INTHOUSANDTHSOFINCHES: ::DWORD = 0x00000004;
pub const PSD_INHUNDREDTHSOFMILLIMETERS: ::DWORD = 0x00000008;
pub const PSD_DISABLEMARGINS: ::DWORD = 0x00000010;
pub const PSD_DISABLEPRINTER: ::DWORD = 0x00000020;
pub const PSD_NOWARNING: ::DWORD = 0x00000080;
pub const PSD_DISABLEORIENTATION: ::DWORD = 0x00000100;
pub const PSD_RETURNDEFAULT: ::DWORD = 0x00000400;
pub const PSD_DISABLEPAPER: ::DWORD = 0x00000200;
pub const PSD_SHOWHELP: ::DWORD = 0x00000800;
pub const PSD_ENABLEPAGESETUPHOOK: ::DWORD = 0x00002000;
pub const PSD_ENABLEPAGESETUPTEMPLATE: ::DWORD = 0x00008000;
pub const PSD_ENABLEPAGESETUPTEMPLATEHANDLE: ::DWORD = 0x00020000;
pub const PSD_ENABLEPAGEPAINTHOOK: ::DWORD = 0x00040000;
pub const PSD_DISABLEPAGEPAINTING: ::DWORD = 0x00080000;
pub const PSD_NONETWORKBUTTON: ::DWORD = 0x00200000;

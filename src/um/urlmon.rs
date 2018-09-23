// Copyright © 2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! URL Moniker interfaces
use ctypes::{c_uchar, c_ulong, c_void};
use shared::basetsd::{DWORD_PTR, HANDLE_PTR};
use shared::guiddef::{CLSID, IID, REFCLSID, REFGUID, REFIID};
use shared::minwindef::{BOOL, BYTE, DWORD, LPDWORD, LPVOID, UINT, ULONG};
use shared::windef::{HWND};
use shared::wtypes::{BSTR, CLIPFORMAT, QUERYCONTEXT, VARIANT_BOOL, uCLSSPEC};
use shared::wtypesbase::LPOLESTR;
use um::minwinbase::{SECURITY_ATTRIBUTES, SYSTEMTIME};
use um::msxml::IXMLElement;
use um::objidl::{
    FORMATETC, IBindCtx, IEnumFORMATETC, IMoniker, LPBC, LPBINDCTX, LPMONIKER, STGMEDIUM,
};
use um::objidlbase::{IEnumString, LPSTREAM};
use um::servprov::IServiceProvider;
use um::unknwnbase::{IClassFactory, IUnknown, IUnknownVtbl, LPUNKNOWN};
use um::winnt::{
    HANDLE, HRESULT, LARGE_INTEGER, LONG, LPCSTR, LPCWSTR, LPSTR, LPWSTR, PVOID, PWSTR,
    ULARGE_INTEGER, WCHAR,
};
pub const BINDF_DONTUSECACHE: DWORD = BINDF_GETNEWESTVERSION;
pub const BINDF_DONTPUTINCACHE: DWORD = BINDF_NOWRITECACHE;
pub const BINDF_NOCOPYDATA: DWORD = BINDF_PULLDATA;
pub const INVALID_P_ROOT_SECURITY_ID: *mut BYTE = -1isize as *mut BYTE;
pub const PI_DOCFILECLSIDLOOKUP: DWORD = PI_CLSIDLOOKUP;
pub const SZ_URLCONTEXT: &'static str = "URL Context";
pub const SZ_ASYNC_CALLEE: &'static str = "AsyncCallee";
pub const MKSYS_URLMONIKER: DWORD = 6;
pub const URL_MK_LEGACY: DWORD = 0;
pub const URL_MK_UNIFORM: DWORD = 1;
pub const URL_MK_NO_CANONICALIZE: DWORD = 2;
extern "system" {
    pub fn CreateURLMoniker(
        pMkCtx: LPMONIKER,
        szURL: LPCWSTR,
        ppmk: *mut LPMONIKER,
    ) -> HRESULT;
    pub fn CreateURLMonikerEx(
        pMkCtx: LPMONIKER,
        szURL: LPCWSTR,
        ppmk: *mut LPMONIKER,
        dwFlags: DWORD,
    ) -> HRESULT;
    pub fn GetClassURL(
        szURL: LPCWSTR,
        pClsID: *mut CLSID,
    ) -> HRESULT;
    pub fn CreateAsyncBindCtx(
        reserved: DWORD,
        pBSCb: *mut IBindStatusCallback,
        pEFetc: *mut IEnumFORMATETC,
        ppBC: *mut *mut IBindCtx,
    ) -> HRESULT;
    pub fn CreateURLMonikerEx2(
        pMkCtx: LPMONIKER,
        pUri: *mut IUri,
        ppmk: *mut LPMONIKER,
        dwFlags: DWORD,
    ) -> HRESULT;
    pub fn CreateAsyncBindCtxEx(
        pbc: *mut IBindCtx,
        dwOptions: DWORD,
        pBSCb: *mut IBindStatusCallback,
        pEnum: *mut IEnumFORMATETC,
        ppBC: *mut *mut IBindCtx,
        reserved: DWORD,
    ) -> HRESULT;
    pub fn MkParseDisplayNameEx(
        pbc: *mut IBindCtx,
        szDisplayName: LPCWSTR,
        pchEaten: *mut ULONG,
        ppmk: *mut LPMONIKER,
    ) -> HRESULT;
    pub fn RegisterBindStatusCallback(
        pBC: LPBC,
        pBSCb: *mut IBindStatusCallback,
        ppBSCBPrev: *mut *mut IBindStatusCallback,
        dwReserved: DWORD,
    ) -> HRESULT;
    pub fn RevokeBindStatusCallback(
        pBC: LPBC,
        pBSCb: *mut IBindStatusCallback,
    ) -> HRESULT;
    pub fn GetClassFileOrMime(
        pBC: LPBC,
        szFilename: LPCWSTR,
        pBuffer: LPVOID,
        cbSize: DWORD,
        szMime: LPCWSTR,
        dwReserved: DWORD,
        pclsid: *mut CLSID,
    ) -> HRESULT;
    pub fn IsValidURL(
        pBC: LPBC,
        szURL: LPCWSTR,
        dwReserved: DWORD,
    ) -> HRESULT;
    pub fn CoGetClassObjectFromURL(
        rCLASSID: REFCLSID,
        szCODE: LPCWSTR,
        dwFileVersionMS: DWORD,
        dwFileVersionLS: DWORD,
        szTYPE: LPCWSTR,
        pBindCtx: LPBINDCTX,
        dwClsContext: DWORD,
        pvReserved: LPVOID,
        riid: REFIID,
        ppv: *mut LPVOID,
    ) -> HRESULT;
    pub fn IEInstallScope(
        pdwScope: LPDWORD,
    ) -> HRESULT;
    pub fn FaultInIEFeature(
        hWnd: HWND,
        pClassSpec: *mut uCLSSPEC,
        pQuery: *mut QUERYCONTEXT,
        dwFlags: DWORD,
    ) -> HRESULT;
    pub fn GetComponentIDFromCLSSPEC(
        pClassspec: *mut uCLSSPEC,
        ppszComponentID: *mut LPSTR,
    ) -> HRESULT;
}
pub const FIEF_FLAG_FORCE_JITUI: DWORD = 0x1;
pub const FIEF_FLAG_PEEK: DWORD = 0x2;
pub const FIEF_FLAG_SKIP_INSTALLED_VERSION_CHECK: DWORD = 0x4;
pub const FIEF_FLAG_RESERVED_0: DWORD = 0x8;
extern "system" {
    pub fn IsAsyncMoniker(
        pmk: *mut IMoniker,
    ) -> HRESULT;
    pub fn CreateURLBinding(
        lpszUrl: LPCWSTR,
        pbc: *mut IBindCtx,
        ppBdg: *mut *mut IBinding,
    ) -> HRESULT;
    pub fn RegisterMediaTypes(
        ctypes: UINT,
        rgszTypes: *const LPCSTR,
        rgcfTypes: *mut CLIPFORMAT,
    ) -> HRESULT;
    pub fn FindMediaType(
        rgszTypes: LPCSTR,
        rgcfTypes: *mut CLIPFORMAT,
    ) -> HRESULT;
    pub fn CreateFormatEnumerator(
        cfmtetc: UINT,
        rgfmtetc: *mut FORMATETC,
        ppenumfmtetc: *mut *mut IEnumFORMATETC,
    ) -> HRESULT;
    pub fn RegisterFormatEnumerator(
        pBC: LPBC,
        pEFetc: *mut IEnumFORMATETC,
        reserved: DWORD,
    ) -> HRESULT;
    pub fn RevokeFormatEnumerator(
        pBC: LPBC,
        pEFetc: *mut IEnumFORMATETC,
    ) -> HRESULT;
    pub fn RegisterMediaTypeClass(
        pBC: LPBC,
        ctypes: UINT,
        rgszTypes: *const LPCSTR,
        rgclsID: *mut CLSID,
        reserved: DWORD,
    ) -> HRESULT;
    pub fn FindMediaTypeClass(
        pBC: LPBC,
        szType: LPCSTR,
        pclsID: *mut CLSID,
        reserved: DWORD,
    ) -> HRESULT;
    pub fn UrlMkSetSessionOption(
        dwOption: DWORD,
        pBuffer: LPVOID,
        dwBufferLength: DWORD,
        dwReserved: DWORD,
    ) -> HRESULT;
    pub fn UrlMkGetSessionOption(
        dwOption: DWORD,
        pBuffer: LPVOID,
        dwBufferLength: DWORD,
        pdwBufferLengthOut: *mut DWORD,
        dwReserved: DWORD,
    ) -> HRESULT;
    pub fn FindMimeFromData(
        pBC: LPBC,
        pwzUrl: LPCWSTR,
        pBuffer: LPVOID,
        cbSize: DWORD,
        pwzMimeProposed: LPCWSTR,
        dwMimeFlags: DWORD,
        ppwzMimeOut: *mut LPWSTR,
        dwReserved: DWORD,
    ) -> HRESULT;
}
pub const FMFD_DEFAULT: DWORD = 0x00000000;
pub const FMFD_URLASFILENAME: DWORD = 0x00000001;
pub const FMFD_ENABLEMIMESNIFFING: DWORD = 0x00000002;
pub const FMFD_IGNOREMIMETEXTPLAIN: DWORD = 0x00000004;
pub const FMFD_SERVERMIME: DWORD = 0x00000008;
pub const FMFD_RESPECTTEXTPLAIN: DWORD = 0x00000010;
pub const FMFD_RETURNUPDATEDIMGMIMES: DWORD = 0x00000020;
pub const FMFD_RESERVED_1: DWORD = 0x00000040;
pub const UAS_EXACTLEGACY: DWORD = 0x00001000;
extern "system" {
    pub fn ObtainUserAgentString(
        dwOption: DWORD,
        pszUAOut: LPSTR,
        cbSize: DWORD,
    ) -> HRESULT;
    pub fn CompareSecurityIds(
        pbSecurityId1: *mut BYTE,
        dwLen1: DWORD,
        pbSecurityId2: *mut BYTE,
        dwLen2: DWORD,
        dwReserved: DWORD,
    ) -> HRESULT;
    pub fn CompatFlagsFromClsid(
        pclsid: *mut CLSID,
        pdwCompatFlags: LPDWORD,
        pdwMiscStatusFlags: LPDWORD,
    ) -> HRESULT;
}
ENUM!{enum IEObjectType {
    IE_EPM_OBJECT_EVENT,
    IE_EPM_OBJECT_MUTEX,
    IE_EPM_OBJECT_SEMAPHORE,
    IE_EPM_OBJECT_SHARED_MEMORY,
    IE_EPM_OBJECT_WAITABLE_TIMER,
    IE_EPM_OBJECT_FILE,
    IE_EPM_OBJECT_NAMED_PIPE,
    IE_EPM_OBJECT_REGISTRY,
}}
extern "system" {
    pub fn SetAccessForIEAppContainer(
        hObject: HANDLE,
        ieObjectType: IEObjectType,
        dwAccessMask: DWORD,
    ) -> HRESULT;
}
pub const URLMON_OPTION_USERAGENT: DWORD = 0x10000001;
pub const URLMON_OPTION_USERAGENT_REFRESH: DWORD = 0x10000002;
pub const URLMON_OPTION_URL_ENCODING: DWORD = 0x10000004;
pub const URLMON_OPTION_USE_BINDSTRINGCREDS: DWORD = 0x10000008;
pub const URLMON_OPTION_USE_BROWSERAPPSDOCUMENTS: DWORD = 0x10000010;
pub const CF_NULL: PVOID = 0 as PVOID;
pub const CFSTR_MIME_NULL: PVOID = CF_NULL;
pub const CFSTR_MIME_TEXT: &'static str = "text/plain";
pub const CFSTR_MIME_RICHTEXT: &'static str = "text/richtext";
pub const CFSTR_MIME_MANIFEST: &'static str = "text/cache-manifest";
pub const CFSTR_MIME_WEBVTT: &'static str = "text/vtt";
pub const CFSTR_MIME_X_BITMAP: &'static str = "image/x-xbitmap";
pub const CFSTR_MIME_POSTSCRIPT: &'static str = "application/postscript";
pub const CFSTR_MIME_AIFF: &'static str = "audio/aiff";
pub const CFSTR_MIME_BASICAUDIO: &'static str = "audio/basic";
pub const CFSTR_MIME_WAV: &'static str = "audio/wav";
pub const CFSTR_MIME_X_WAV: &'static str = "audio/x-wav";
pub const CFSTR_MIME_GIF: &'static str = "image/gif";
pub const CFSTR_MIME_PJPEG: &'static str = "image/pjpeg";
pub const CFSTR_MIME_JPEG: &'static str = "image/jpeg";
pub const CFSTR_MIME_TIFF: &'static str = "image/tiff";
pub const CFSTR_MIME_JPEG_XR: &'static str = "image/vnd.ms-photo";
pub const CFSTR_MIME_PNG: &'static str = "image/png";
pub const CFSTR_MIME_DDS: &'static str = "image/vnd.ms-dds";
pub const CFSTR_MIME_X_PNG: &'static str = "image/x-png";
pub const CFSTR_MIME_X_ICON: &'static str = "image/x-icon";
pub const CFSTR_MIME_SVG_XML: &'static str = "image/svg+xml";
pub const CFSTR_MIME_BMP: &'static str = "image/bmp";
pub const CFSTR_MIME_X_EMF: &'static str = "image/x-emf";
pub const CFSTR_MIME_X_WMF: &'static str = "image/x-wmf";
pub const CFSTR_MIME_AVI: &'static str = "video/avi";
pub const CFSTR_MIME_MPEG: &'static str = "video/mpeg";
pub const CFSTR_MIME_FRACTALS: &'static str = "application/fractals";
pub const CFSTR_MIME_RAWDATA: &'static str = "application/octet-stream";
pub const CFSTR_MIME_RAWDATASTRM: &'static str = "application/octet-stream";
pub const CFSTR_MIME_PDF: &'static str = "application/pdf";
pub const CFSTR_MIME_HTA: &'static str = "application/hta";
pub const CFSTR_MIME_APP_XML: &'static str = "application/xml";
pub const CFSTR_MIME_XHTML: &'static str = "application/xhtml+xml";
pub const CFSTR_MIME_X_AIFF: &'static str = "audio/x-aiff";
pub const CFSTR_MIME_X_REALAUDIO: &'static str = "audio/x-pn-realaudio";
pub const CFSTR_MIME_XBM: &'static str = "image/xbm";
pub const CFSTR_MIME_QUICKTIME: &'static str = "video/quicktime";
pub const CFSTR_MIME_X_MSVIDEO: &'static str = "video/x-msvideo";
pub const CFSTR_MIME_X_SGI_MOVIE: &'static str = "video/x-sgi-movie";
pub const CFSTR_MIME_X_MIXED_REPLACE: &'static str = "multipart/x-mixed-replace";
pub const CFSTR_MIME_HTML: &'static str = "text/html";
pub const CFSTR_MIME_XML: &'static str = "text/xml";
pub const CFSTR_MIME_TTML: &'static str = "application/ttml+xml";
pub const CFSTR_MIME_TTAF: &'static str = "application/ttaf+xml";
pub const CFSTR_MIME_X_JAVASCRIPT: &'static str = "application/x-javascript";
pub const CFSTR_MIME_TEXT_JSON: &'static str = "text/json";
pub const CFSTR_MIME_APPLICATION_JAVASCRIPT: &'static str = "application/javascript";
pub const MK_S_ASYNCHRONOUS: HRESULT = 0x000401E8;
pub const INET_E_INVALID_URL: HRESULT = 0x800C0002;
pub const INET_E_NO_SESSION: HRESULT = 0x800C0003;
pub const INET_E_CANNOT_CONNECT: HRESULT = 0x800C0004;
pub const INET_E_RESOURCE_NOT_FOUND: HRESULT = 0x800C0005;
pub const INET_E_OBJECT_NOT_FOUND: HRESULT = 0x800C0006;
pub const INET_E_DATA_NOT_AVAILABLE: HRESULT = 0x800C0007;
pub const INET_E_DOWNLOAD_FAILURE: HRESULT = 0x800C0008;
pub const INET_E_AUTHENTICATION_REQUIRED: HRESULT = 0x800C0009;
pub const INET_E_NO_VALID_MEDIA: HRESULT = 0x800C000A;
pub const INET_E_CONNECTION_TIMEOUT: HRESULT = 0x800C000B;
pub const INET_E_INVALID_REQUEST: HRESULT = 0x800C000C;
pub const INET_E_UNKNOWN_PROTOCOL: HRESULT = 0x800C000D;
pub const INET_E_SECURITY_PROBLEM: HRESULT = 0x800C000E;
pub const INET_E_CANNOT_LOAD_DATA: HRESULT = 0x800C000F;
pub const INET_E_CANNOT_INSTANTIATE_OBJECT: HRESULT = 0x800C0010;
pub const INET_E_INVALID_CERTIFICATE: HRESULT = 0x800C0019;
pub const INET_E_REDIRECT_FAILED: HRESULT = 0x800C0014;
pub const INET_E_REDIRECT_TO_DIR: HRESULT = 0x800C0015;
pub const INET_E_CANNOT_LOCK_REQUEST: HRESULT = 0x800C0016;
pub const INET_E_USE_EXTEND_BINDING: HRESULT = 0x800C0017;
pub const INET_E_TERMINATED_BIND: HRESULT = 0x800C0018;
pub const INET_E_RESERVED_1: HRESULT = 0x800C001A;
pub const INET_E_BLOCKED_REDIRECT_XSECURITYID: HRESULT = 0x800C001B;
pub const INET_E_DOMINJECTIONVALIDATION: HRESULT = 0x800C001C;
pub const INET_E_VTAB_SWITCH_FORCE_ENGINE: HRESULT = 0x800C001D;
pub const INET_E_HSTS_CERTIFICATE_ERROR: HRESULT = 0x800C001E;
pub const INET_E_RESERVED_2: HRESULT = 0x800C001F;
pub const INET_E_RESERVED_3: HRESULT = 0x800C0020;
pub const INET_E_RESERVED_4: HRESULT = 0x800C0021;
pub const INET_E_ERROR_FIRST: HRESULT = 0x800C0002;
pub const INET_E_CODE_DOWNLOAD_DECLINED: HRESULT = 0x800C0100;
pub const INET_E_RESULT_DISPATCHED: HRESULT = 0x800C0200;
pub const INET_E_CANNOT_REPLACE_SFP_FILE: HRESULT = 0x800C0300;
pub const INET_E_CODE_INSTALL_SUPPRESSED: HRESULT = 0x800C0400;
pub const INET_E_CODE_INSTALL_BLOCKED_BY_HASH_POLICY: HRESULT = 0x800C0500;
pub const INET_E_DOWNLOAD_BLOCKED_BY_INPRIVATE: HRESULT = 0x800C0501;
pub const INET_E_CODE_INSTALL_BLOCKED_IMMERSIVE: HRESULT = 0x800C0502;
pub const INET_E_FORBIDFRAMING: HRESULT = 0x800C0503;
pub const INET_E_CODE_INSTALL_BLOCKED_ARM: HRESULT = 0x800C0504;
pub const INET_E_BLOCKED_PLUGGABLE_PROTOCOL: HRESULT = 0x800C0505;
pub const INET_E_BLOCKED_ENHANCEDPROTECTEDMODE: HRESULT = 0x800C0506;
pub const INET_E_CODE_INSTALL_BLOCKED_BITNESS: HRESULT = 0x800C0507;
pub const INET_E_DOWNLOAD_BLOCKED_BY_CSP: HRESULT = 0x800C0508;
pub const INET_E_ERROR_LAST: HRESULT = INET_E_DOWNLOAD_BLOCKED_BY_CSP;
pub type LPPERSISTMONIKER = *mut IPersistMoniker;
RIDL!(#[uuid(0x79eac9c9, 0xbaf9, 0x11ce, 0x8c, 0x82, 0x00, 0xaa, 0x00, 0x4b, 0xa9, 0x0b)]
interface IPersistMoniker(IPersistMonikerVtbl): IUnknown(IUnknownVtbl) {
    fn GetClassID(
        pClassID: *mut CLSID,
    ) -> HRESULT,
    fn IsDirty() -> HRESULT,
    fn Load(
        fFullyAvailable: BOOL,
        pimkName: *mut IMoniker,
        pibc: LPBC,
        grfMode: DWORD,
    ) -> HRESULT,
    fn Save(
        pimkName: *mut IMoniker,
        pbc: LPBC,
        fRemember: BOOL,
    ) -> HRESULT,
    fn SaveCompleted(
        pimkName: *mut IMoniker,
        pibc: LPBC,
    ) -> HRESULT,
    fn SaveCoGetCurMonikermpleted(
        ppimkName: *mut *mut IMoniker,
    ) -> HRESULT,
});
pub type LPMONIKERPROP = *mut IMonikerProp;
ENUM!{enum MONIKERPROPERTY {
    MIMETYPEPROP = 0,
    USE_SRC_URL = 0x1,
    CLASSIDPROP = 0x2,
    TRUSTEDDOWNLOADPROP = 0x3,
    POPUPLEVELPROP = 0x4,
}}
RIDL!(#[uuid(0xa5ca5f7f, 0x1847, 0x4d87, 0x9c, 0x5b, 0x91, 0x85, 0x09, 0xf7, 0x51, 0x1d)]
interface IMonikerProp(IMonikerPropVtbl): IUnknown(IUnknownVtbl) {
    fn PutProperty(
        mkp: MONIKERPROPERTY,
        val: LPCWSTR,
    ) -> HRESULT,
});
pub type LPBINDPROTOCOL = *mut IBindProtocol;
RIDL!(#[uuid(0x79eac9cd, 0xbaf9, 0x11ce, 0x8c, 0x82, 0x00, 0xaa, 0x00, 0x4b, 0xa9, 0x0b)]
interface IBindProtocol(IBindProtocolVtbl): IUnknown(IUnknownVtbl) {
    fn CreateBinding(
        szUrl: LPCWSTR,
        pbc: *mut IBindCtx,
        ppb: *mut *mut IBinding,
    ) -> HRESULT,
});
pub type LPBINDING = *mut IBinding;
RIDL!(#[uuid(0x79eac9c0, 0xbaf9, 0x11ce, 0x8c, 0x82, 0x00, 0xaa, 0x00, 0x4b, 0xa9, 0x0b)]
interface IBinding(IBindingVtbl): IUnknown(IUnknownVtbl) {
    fn Abort() -> HRESULT,
    fn Suspend() -> HRESULT,
    fn Resume() -> HRESULT,
    fn SetPriority(
        nPriority: LONG,
    ) -> HRESULT,
    fn GetPriority(
        pnPriority: *mut LONG,
    ) -> HRESULT,
    fn GetBindResult(
        pclsidProtocol: *mut CLSID,
        pdwResult: *mut DWORD,
        pszResult: *mut LPOLESTR,
        pdwReserved: *mut DWORD,
    ) -> HRESULT,
});
pub type LPBINDSTATUSCALLBACK = *mut IBindStatusCallback;
ENUM!{enum BINDVERB {
    BINDVERB_GET = 0,
    BINDVERB_POST = 0x1,
    BINDVERB_PUT = 0x2,
    BINDVERB_CUSTOM = 0x3,
    BINDVERB_RESERVED1 = 0x4,
}}
ENUM!{enum BINDINFOF {
    BINDINFOF_URLENCODESTGMEDDATA = 0x1,
    BINDINFOF_URLENCODEDEXTRAINFO = 0x2,
}}
ENUM!{enum BINDF {
    BINDF_ASYNCHRONOUS = 0x1,
    BINDF_ASYNCSTORAGE = 0x2,
    BINDF_NOPROGRESSIVERENDERING = 0x4,
    BINDF_OFFLINEOPERATION = 0x8,
    BINDF_GETNEWESTVERSION = 0x10,
    BINDF_NOWRITECACHE = 0x20,
    BINDF_NEEDFILE = 0x40,
    BINDF_PULLDATA = 0x80,
    BINDF_IGNORESECURITYPROBLEM = 0x100,
    BINDF_RESYNCHRONIZE = 0x200,
    BINDF_HYPERLINK = 0x400,
    BINDF_NO_UI = 0x800,
    BINDF_SILENTOPERATION = 0x1000,
    BINDF_PRAGMA_NO_CACHE = 0x2000,
    BINDF_GETCLASSOBJECT = 0x4000,
    BINDF_RESERVED_1 = 0x8000,
    BINDF_FREE_THREADED = 0x10000,
    BINDF_DIRECT_READ = 0x20000,
    BINDF_FORMS_SUBMIT = 0x40000,
    BINDF_GETFROMCACHE_IF_NET_FAIL = 0x80000,
    BINDF_FROMURLMON = 0x100000,
    BINDF_FWD_BACK = 0x200000,
    BINDF_PREFERDEFAULTHANDLER = 0x400000,
    BINDF_ENFORCERESTRICTED = 0x800000,
    BINDF_RESERVED_2 = 0x80000000,
    BINDF_RESERVED_3 = 0x1000000,
    BINDF_RESERVED_4 = 0x2000000,
    BINDF_RESERVED_5 = 0x4000000,
    BINDF_RESERVED_6 = 0x8000000,
    BINDF_RESERVED_7 = 0x40000000,
    BINDF_RESERVED_8 = 0x20000000,
}}
ENUM!{enum URL_ENCODING {
    URL_ENCODING_NONE = 0,
    URL_ENCODING_ENABLE_UTF8 = 0x10000000,
    URL_ENCODING_DISABLE_UTF8 = 0x20000000,
}}
STRUCT!{struct BINDINFO {
    cbSize: ULONG,
    szExtraInfo: LPWSTR,
    stgmedData: STGMEDIUM,
    grfBindInfoF: DWORD,
    dwBindVerb: DWORD,
    szCustomVerb: LPWSTR,
    cbstgmedData: DWORD,
    dwOptions: DWORD,
    dwOptionsFlags: DWORD,
    dwCodePage: DWORD,
    securityAttributes: SECURITY_ATTRIBUTES,
    iid: IID,
    pUnk: *mut IUnknown,
    dwReserved: DWORD,
}}
STRUCT!{struct REMSECURITY_ATTRIBUTES {
    nLength: DWORD,
    lpSecurityDescriptor: DWORD,
    bInheritHandle: BOOL,
}}
pub type PREMSECURITY_ATTRIBUTES = *mut REMSECURITY_ATTRIBUTES;
pub type LPREMSECURITY_ATTRIBUTES = *mut REMSECURITY_ATTRIBUTES;
STRUCT!{struct RemBINDINFO {
    cbSize: ULONG,
    szExtraInfo: LPWSTR,
    grfBindInfoF: DWORD,
    dwBindVerb: DWORD,
    szCustomVerb: LPWSTR,
    cbstgmedData: DWORD,
    dwOptions: DWORD,
    dwOptionsFlags: DWORD,
    dwCodePage: DWORD,
    securityAttributes: REMSECURITY_ATTRIBUTES,
    iid: IID,
    pUnk: *mut IUnknown,
    dwReserved: DWORD,
}}
STRUCT!{struct RemFORMATETC {
    cfFormat: DWORD,
    ptd: DWORD,
    dwAspect: DWORD,
    lindex: LONG,
    tymed: DWORD,
}}
pub type LPREMFORMATETC = *mut RemFORMATETC;
ENUM!{enum BINDINFO_OPTIONS {
    BINDINFO_OPTIONS_WININETFLAG = 0x10000,
    BINDINFO_OPTIONS_ENABLE_UTF8 = 0x20000,
    BINDINFO_OPTIONS_DISABLE_UTF8 = 0x40000,
    BINDINFO_OPTIONS_USE_IE_ENCODING = 0x80000,
    BINDINFO_OPTIONS_BINDTOOBJECT = 0x100000,
    BINDINFO_OPTIONS_SECURITYOPTOUT = 0x200000,
    BINDINFO_OPTIONS_IGNOREMIMETEXTPLAIN = 0x400000,
    BINDINFO_OPTIONS_USEBINDSTRINGCREDS = 0x800000,
    BINDINFO_OPTIONS_IGNOREHTTPHTTPSREDIRECTS = 0x1000000,
    BINDINFO_OPTIONS_IGNORE_SSLERRORS_ONCE = 0x2000000,
    BINDINFO_WPC_DOWNLOADBLOCKED = 0x8000000,
    BINDINFO_WPC_LOGGING_ENABLED = 0x10000000,
    BINDINFO_OPTIONS_ALLOWCONNECTDATA = 0x20000000,
    BINDINFO_OPTIONS_DISABLEAUTOREDIRECTS = 0x40000000,
    BINDINFO_OPTIONS_SHDOCVW_NAVIGATE = 0x80000000,
}}
ENUM!{enum BSCF {
    BSCF_FIRSTDATANOTIFICATION = 0x1,
    BSCF_INTERMEDIATEDATANOTIFICATION = 0x2,
    BSCF_LASTDATANOTIFICATION = 0x4,
    BSCF_DATAFULLYAVAILABLE = 0x8,
    BSCF_AVAILABLEDATASIZEUNKNOWN = 0x10,
    BSCF_SKIPDRAINDATAFORFILEURLS = 0x20,
    BSCF_64BITLENGTHDOWNLOAD = 0x40,
}}
ENUM!{enum BINDSTATUS {
    BINDSTATUS_FINDINGRESOURCE = 1,
    BINDSTATUS_CONNECTING = BINDSTATUS_FINDINGRESOURCE + 1,
    BINDSTATUS_REDIRECTING = BINDSTATUS_CONNECTING + 1,
    BINDSTATUS_BEGINDOWNLOADDATA = BINDSTATUS_REDIRECTING + 1,
    BINDSTATUS_DOWNLOADINGDATA = BINDSTATUS_BEGINDOWNLOADDATA + 1,
    BINDSTATUS_ENDDOWNLOADDATA = BINDSTATUS_DOWNLOADINGDATA + 1,
    BINDSTATUS_BEGINDOWNLOADCOMPONENTS = BINDSTATUS_ENDDOWNLOADDATA + 1,
    BINDSTATUS_INSTALLINGCOMPONENTS = BINDSTATUS_BEGINDOWNLOADCOMPONENTS + 1,
    BINDSTATUS_ENDDOWNLOADCOMPONENTS = BINDSTATUS_INSTALLINGCOMPONENTS + 1,
    BINDSTATUS_USINGCACHEDCOPY = BINDSTATUS_ENDDOWNLOADCOMPONENTS + 1,
    BINDSTATUS_SENDINGREQUEST = BINDSTATUS_USINGCACHEDCOPY + 1,
    BINDSTATUS_CLASSIDAVAILABLE = BINDSTATUS_SENDINGREQUEST + 1,
    BINDSTATUS_MIMETYPEAVAILABLE = BINDSTATUS_CLASSIDAVAILABLE + 1,
    BINDSTATUS_CACHEFILENAMEAVAILABLE = BINDSTATUS_MIMETYPEAVAILABLE + 1,
    BINDSTATUS_BEGINSYNCOPERATION = BINDSTATUS_CACHEFILENAMEAVAILABLE + 1,
    BINDSTATUS_ENDSYNCOPERATION = BINDSTATUS_BEGINSYNCOPERATION + 1,
    BINDSTATUS_BEGINUPLOADDATA = BINDSTATUS_ENDSYNCOPERATION + 1,
    BINDSTATUS_UPLOADINGDATA = BINDSTATUS_BEGINUPLOADDATA + 1,
    BINDSTATUS_ENDUPLOADDATA = BINDSTATUS_UPLOADINGDATA + 1,
    BINDSTATUS_PROTOCOLCLASSID = BINDSTATUS_ENDUPLOADDATA + 1,
    BINDSTATUS_ENCODING = BINDSTATUS_PROTOCOLCLASSID + 1,
    BINDSTATUS_VERIFIEDMIMETYPEAVAILABLE = BINDSTATUS_ENCODING + 1,
    BINDSTATUS_CLASSINSTALLLOCATION = BINDSTATUS_VERIFIEDMIMETYPEAVAILABLE + 1,
    BINDSTATUS_DECODING = BINDSTATUS_CLASSINSTALLLOCATION + 1,
    BINDSTATUS_LOADINGMIMEHANDLER = BINDSTATUS_DECODING + 1,
    BINDSTATUS_CONTENTDISPOSITIONATTACH = BINDSTATUS_LOADINGMIMEHANDLER + 1,
    BINDSTATUS_FILTERREPORTMIMETYPE = BINDSTATUS_CONTENTDISPOSITIONATTACH + 1,
    BINDSTATUS_CLSIDCANINSTANTIATE = BINDSTATUS_FILTERREPORTMIMETYPE + 1,
    BINDSTATUS_IUNKNOWNAVAILABLE = BINDSTATUS_CLSIDCANINSTANTIATE + 1,
    BINDSTATUS_DIRECTBIND = BINDSTATUS_IUNKNOWNAVAILABLE + 1,
    BINDSTATUS_RAWMIMETYPE = BINDSTATUS_DIRECTBIND + 1,
    BINDSTATUS_PROXYDETECTING = BINDSTATUS_RAWMIMETYPE + 1,
    BINDSTATUS_ACCEPTRANGES = BINDSTATUS_PROXYDETECTING + 1,
    BINDSTATUS_COOKIE_SENT = BINDSTATUS_ACCEPTRANGES + 1,
    BINDSTATUS_COMPACT_POLICY_RECEIVED = BINDSTATUS_COOKIE_SENT + 1,
    BINDSTATUS_COOKIE_SUPPRESSED = BINDSTATUS_COMPACT_POLICY_RECEIVED + 1,
    BINDSTATUS_COOKIE_STATE_UNKNOWN = BINDSTATUS_COOKIE_SUPPRESSED + 1,
    BINDSTATUS_COOKIE_STATE_ACCEPT = BINDSTATUS_COOKIE_STATE_UNKNOWN + 1,
    BINDSTATUS_COOKIE_STATE_REJECT = BINDSTATUS_COOKIE_STATE_ACCEPT + 1,
    BINDSTATUS_COOKIE_STATE_PROMPT = BINDSTATUS_COOKIE_STATE_REJECT + 1,
    BINDSTATUS_COOKIE_STATE_LEASH = BINDSTATUS_COOKIE_STATE_PROMPT + 1,
    BINDSTATUS_COOKIE_STATE_DOWNGRADE = BINDSTATUS_COOKIE_STATE_LEASH + 1,
    BINDSTATUS_POLICY_HREF = BINDSTATUS_COOKIE_STATE_DOWNGRADE + 1,
    BINDSTATUS_P3P_HEADER = BINDSTATUS_POLICY_HREF + 1,
    BINDSTATUS_SESSION_COOKIE_RECEIVED = BINDSTATUS_P3P_HEADER + 1,
    BINDSTATUS_PERSISTENT_COOKIE_RECEIVED = BINDSTATUS_SESSION_COOKIE_RECEIVED + 1,
    BINDSTATUS_SESSION_COOKIES_ALLOWED = BINDSTATUS_PERSISTENT_COOKIE_RECEIVED + 1,
    BINDSTATUS_CACHECONTROL = BINDSTATUS_SESSION_COOKIES_ALLOWED + 1,
    BINDSTATUS_CONTENTDISPOSITIONFILENAME = BINDSTATUS_CACHECONTROL + 1,
    BINDSTATUS_MIMETEXTPLAINMISMATCH = BINDSTATUS_CONTENTDISPOSITIONFILENAME + 1,
    BINDSTATUS_PUBLISHERAVAILABLE = BINDSTATUS_MIMETEXTPLAINMISMATCH + 1,
    BINDSTATUS_DISPLAYNAMEAVAILABLE = BINDSTATUS_PUBLISHERAVAILABLE + 1,
    BINDSTATUS_SSLUX_NAVBLOCKED = BINDSTATUS_DISPLAYNAMEAVAILABLE + 1,
    BINDSTATUS_SERVER_MIMETYPEAVAILABLE = BINDSTATUS_SSLUX_NAVBLOCKED + 1,
    BINDSTATUS_SNIFFED_CLASSIDAVAILABLE = BINDSTATUS_SERVER_MIMETYPEAVAILABLE + 1,
    BINDSTATUS_64BIT_PROGRESS = BINDSTATUS_SNIFFED_CLASSIDAVAILABLE + 1,
    BINDSTATUS_LAST = BINDSTATUS_64BIT_PROGRESS,
    BINDSTATUS_RESERVED_0 = BINDSTATUS_LAST + 1,
    BINDSTATUS_RESERVED_1 = BINDSTATUS_RESERVED_0 + 1,
    BINDSTATUS_RESERVED_2 = BINDSTATUS_RESERVED_1 + 1,
    BINDSTATUS_RESERVED_3 = BINDSTATUS_RESERVED_2 + 1,
    BINDSTATUS_RESERVED_4 = BINDSTATUS_RESERVED_3 + 1,
    BINDSTATUS_RESERVED_5 = BINDSTATUS_RESERVED_4 + 1,
    BINDSTATUS_RESERVED_6 = BINDSTATUS_RESERVED_5 + 1,
    BINDSTATUS_RESERVED_7 = BINDSTATUS_RESERVED_6 + 1,
    BINDSTATUS_RESERVED_8 = BINDSTATUS_RESERVED_7 + 1,
    BINDSTATUS_RESERVED_9 = BINDSTATUS_RESERVED_8 + 1,
    BINDSTATUS_RESERVED_A = BINDSTATUS_RESERVED_9 + 1,
    BINDSTATUS_RESERVED_B = BINDSTATUS_RESERVED_A + 1,
    BINDSTATUS_RESERVED_C = BINDSTATUS_RESERVED_B + 1,
    BINDSTATUS_RESERVED_D = BINDSTATUS_RESERVED_C + 1,
    BINDSTATUS_RESERVED_E = BINDSTATUS_RESERVED_D + 1,
    BINDSTATUS_RESERVED_F = BINDSTATUS_RESERVED_E + 1,
    BINDSTATUS_RESERVED_10 = BINDSTATUS_RESERVED_F + 1,
    BINDSTATUS_RESERVED_11 = BINDSTATUS_RESERVED_10 + 1,
    BINDSTATUS_RESERVED_12 = BINDSTATUS_RESERVED_11 + 1,
    BINDSTATUS_LAST_PRIVATE = BINDSTATUS_RESERVED_12,
}}
RIDL!(#[uuid(0x79eac9c1, 0xbaf9, 0x11ce, 0x8c, 0x82, 0x00, 0xaa, 0x00, 0x4b, 0xa9, 0x0b)]
interface IBindStatusCallback(IBindStatusCallbackVtbl): IUnknown(IUnknownVtbl) {
    fn OnStartBinding(
        dwReserved: DWORD,
        pib: *mut IBinding,
    ) -> HRESULT,
    fn GetPriority(
        pnPriority: *mut LONG,
    ) -> HRESULT,
    fn OnLowResource(
        reserved: DWORD,
    ) -> HRESULT,
    fn OnProgress(
        ulProgress: ULONG,
        ulProgressMax: ULONG,
        ulStatusCode: ULONG,
        szStatusText: LPCWSTR,
    ) -> HRESULT,
    fn OnStopBinding(
        hresult: HRESULT,
        szError: LPCWSTR,
    ) -> HRESULT,
    fn GetBindInfo(
        grfBINDF: *mut DWORD,
        pbindinfo: *mut BINDINFO,
    ) -> HRESULT,
    fn OnDataAvailable(
        grfBSCF: DWORD,
        dwSize: DWORD,
        pformatetc: *mut FORMATETC,
        pstgmed: *mut STGMEDIUM,
    ) -> HRESULT,
    fn OnObjectAvailable(
        riid: REFIID,
        punk: *mut IUnknown,
    ) -> HRESULT,
});
pub type LPBINDSTATUSCALLBACKEX = *mut IBindStatusCallbackEx;
ENUM!{enum BINDF2 {
    BINDF2_DISABLEBASICOVERHTTP = 0x1,
    BINDF2_DISABLEAUTOCOOKIEHANDLING = 0x2,
    BINDF2_READ_DATA_GREATER_THAN_4GB = 0x4,
    BINDF2_DISABLE_HTTP_REDIRECT_XSECURITYID = 0x8,
    BINDF2_SETDOWNLOADMODE = 0x20,
    BINDF2_DISABLE_HTTP_REDIRECT_CACHING = 0x40,
    BINDF2_KEEP_CALLBACK_MODULE_LOADED = 0x80,
    BINDF2_ALLOW_PROXY_CRED_PROMPT = 0x100,
    BINDF2_RESERVED_17 = 0x200,
    BINDF2_RESERVED_16 = 0x400,
    BINDF2_RESERVED_15 = 0x800,
    BINDF2_RESERVED_14 = 0x1000,
    BINDF2_RESERVED_13 = 0x2000,
    BINDF2_RESERVED_12 = 0x4000,
    BINDF2_RESERVED_11 = 0x8000,
    BINDF2_RESERVED_10 = 0x10000,
    BINDF2_RESERVED_F = 0x20000,
    BINDF2_RESERVED_E = 0x40000,
    BINDF2_RESERVED_D = 0x80000,
    BINDF2_RESERVED_C = 0x100000,
    BINDF2_RESERVED_B = 0x200000,
    BINDF2_RESERVED_A = 0x400000,
    BINDF2_RESERVED_9 = 0x800000,
    BINDF2_RESERVED_8 = 0x1000000,
    BINDF2_RESERVED_7 = 0x2000000,
    BINDF2_RESERVED_6 = 0x4000000,
    BINDF2_RESERVED_5 = 0x8000000,
    BINDF2_RESERVED_4 = 0x10000000,
    BINDF2_RESERVED_3 = 0x20000000,
    BINDF2_RESERVED_2 = 0x40000000,
    BINDF2_RESERVED_1 = 0x80000000,
}}
RIDL!(#[uuid(0xaaa74ef9, 0x8ee7, 0x4659, 0x88, 0xd9, 0xf8, 0xc5, 0x04, 0xda, 0x73, 0xcc)]
interface IBindStatusCallbackEx(IBindStatusCallbackExVtbl):
    IBindStatusCallback(IBindStatusCallbackVtbl) {
    fn GetBindInfoEx(
        grfBINDF: *mut DWORD,
        pbindinfo: *mut BINDINFO,
        grfBINDF2: *mut DWORD,
        pdwReserved: *mut DWORD,
    ) -> HRESULT,
});
pub type LPAUTHENTICATION = *mut IAuthenticate;
RIDL!(#[uuid(0x79eac9d0, 0xbaf9, 0x11ce, 0x8c, 0x82, 0x00, 0xaa, 0x00, 0x4b, 0xa9, 0x0b)]
interface IAuthenticate(IAuthenticateVtbl): IUnknown(IUnknownVtbl) {
    fn Authenticate(
        phwnd: *mut HWND,
        pszUsername: *mut LPWSTR,
        pszPassword: *mut LPWSTR,
    ) -> HRESULT,
});
pub type LPAUTHENTICATIONEX = *mut IAuthenticateEx;
ENUM!{enum AUTHENTICATEF {
    AUTHENTICATEF_PROXY = 0x1,
    AUTHENTICATEF_BASIC = 0x2,
    AUTHENTICATEF_HTTP = 0x4,
}}
STRUCT!{struct AUTHENTICATEINFO {
    dwFlags: DWORD,
    dwReserved: DWORD,
}}
RIDL!(#[uuid(0x2ad1edaf, 0xd83d, 0x48b5, 0x9a, 0xdf, 0x03, 0xdb, 0xe1, 0x9f, 0x53, 0xbd)]
interface IAuthenticateEx(IAuthenticateExVtbl): IAuthenticate(IAuthenticateVtbl) {
    fn AuthenticateEx(
        phwnd: *mut HWND,
        pszUsername: *mut LPWSTR,
        pszPassword: *mut LPWSTR,
        pauthinfo: *mut AUTHENTICATEINFO,
    ) -> HRESULT,
});
pub type LPHTTPNEGOTIATE = *mut IHttpNegotiate;
RIDL!(#[uuid(0x79eac9d2, 0xbaf9, 0x11ce, 0x8c, 0x82, 0x00, 0xaa, 0x00, 0x4b, 0xa9, 0x0b)]
interface IHttpNegotiate(IHttpNegotiateVtbl): IUnknown(IUnknownVtbl) {
    fn BeginningTransaction(
        szURL: LPCWSTR,
        szHeaders: LPCWSTR,
        dwReserved: DWORD,
        pszAdditionalHeaders: *mut LPWSTR,
    ) -> HRESULT,
    fn OnResponse(
        dwResponseCode: DWORD,
        szResponseHeaders: LPCWSTR,
        szRequestHeaders: LPCWSTR,
        pszAdditionalRequestHeaders: *mut LPWSTR,
    ) -> HRESULT,
});
pub type LPHTTPNEGOTIATE2 = *mut IHttpNegotiate2;
RIDL!(#[uuid(0x4f9f9fcb, 0xe0f4, 0x48eb, 0xb7, 0xab, 0xfa, 0x2e, 0xa9, 0x36, 0x5c, 0xb4)]
interface IHttpNegotiate2(IHttpNegotiate2Vtbl): IHttpNegotiate(IHttpNegotiateVtbl) {
    fn GetRootSecurityId(
        pbSecurityId: *mut BYTE,
        pcbSecurityId: *mut DWORD,
        dwReserved: DWORD_PTR,
    ) -> HRESULT,
});
pub type LPHTTPNEGOTIATE3 = *mut IHttpNegotiate3;
RIDL!(#[uuid(0x57b6c80a, 0x34c2, 0x4602, 0xbc, 0x26, 0x66, 0xa0, 0x2f, 0xc5, 0x71, 0x53)]
interface IHttpNegotiate3(IHttpNegotiate3Vtbl): IHttpNegotiate2(IHttpNegotiate2Vtbl) {
    fn GetSerializedClientCertContext(
        ppbCert: *mut *mut BYTE,
        pcbCert: *mut DWORD,
    ) -> HRESULT,
});
pub type LPWININETFILESTREAM = *mut IWinInetFileStream;
RIDL!(#[uuid(0xf134c4b7, 0xb1f8, 0x4e75, 0xb8, 0x86, 0x74, 0xb9, 0x09, 0x43, 0xbe, 0xcb)]
interface IWinInetFileStream(IWinInetFileStreamVtbl): IUnknown(IUnknownVtbl) {
    fn SetHandleForUnlock(
        hWinInetLockHandle: DWORD_PTR,
        dwReserved: DWORD_PTR,
    ) -> HRESULT,
    fn SetDeleteFile(
        dwReserved: DWORD_PTR,
    ) -> HRESULT,
});
pub type LPWINDOWFORBINDINGUI = *mut IWindowForBindingUI;
RIDL!(#[uuid(0x79eac9d5, 0xbafa, 0x11ce, 0x8c, 0x82, 0x00, 0xaa, 0x00, 0x4b, 0xa9, 0x0b)]
interface IWindowForBindingUI(IWindowForBindingUIVtbl): IUnknown(IUnknownVtbl) {
    fn GetWindow(
        rguidReason: REFGUID,
        phwnd: *mut HWND,
    ) -> HRESULT,
});
pub type LPCODEINSTALL = *mut ICodeInstall;
ENUM!{enum CIP_STATUS {
    CIP_DISK_FULL = 0,
    CIP_ACCESS_DENIED = CIP_DISK_FULL + 1,
    CIP_NEWER_VERSION_EXISTS = CIP_ACCESS_DENIED + 1,
    CIP_OLDER_VERSION_EXISTS = CIP_NEWER_VERSION_EXISTS + 1,
    CIP_NAME_CONFLICT = CIP_OLDER_VERSION_EXISTS + 1,
    CIP_TRUST_VERIFICATION_COMPONENT_MISSING = CIP_NAME_CONFLICT + 1,
    CIP_EXE_SELF_REGISTERATION_TIMEOUT = CIP_TRUST_VERIFICATION_COMPONENT_MISSING + 1,
    CIP_UNSAFE_TO_ABORT = CIP_EXE_SELF_REGISTERATION_TIMEOUT + 1,
    CIP_NEED_REBOOT = CIP_UNSAFE_TO_ABORT + 1,
    CIP_NEED_REBOOT_UI_PERMISSION = CIP_NEED_REBOOT + 1,
}}
RIDL!(#[uuid(0x79eac9d1, 0xbaf9, 0x11ce, 0x8c, 0x82, 0x00, 0xaa, 0x00, 0x4b, 0xa9, 0x0b)]
interface ICodeInstall(ICodeInstallVtbl): IWindowForBindingUI(IWindowForBindingUIVtbl) {
    fn OnCodeInstallProblem(
        ulStatusCode: ULONG,
        szDestination: LPCWSTR,
        szSource: LPCWSTR,
        dwReserved: DWORD,
    ) -> HRESULT,
});
ENUM!{enum Uri_PROPERTY {
    Uri_PROPERTY_ABSOLUTE_URI = 0,
    Uri_PROPERTY_STRING_START = Uri_PROPERTY_ABSOLUTE_URI,
    Uri_PROPERTY_AUTHORITY = 1,
    Uri_PROPERTY_DISPLAY_URI = 2,
    Uri_PROPERTY_DOMAIN = 3,
    Uri_PROPERTY_EXTENSION = 4,
    Uri_PROPERTY_FRAGMENT = 5,
    Uri_PROPERTY_HOST = 6,
    Uri_PROPERTY_PASSWORD = 7,
    Uri_PROPERTY_PATH = 8,
    Uri_PROPERTY_PATH_AND_QUERY = 9,
    Uri_PROPERTY_QUERY = 10,
    Uri_PROPERTY_RAW_URI = 11,
    Uri_PROPERTY_SCHEME_NAME = 12,
    Uri_PROPERTY_USER_INFO = 13,
    Uri_PROPERTY_USER_NAME = 14,
    Uri_PROPERTY_STRING_LAST = Uri_PROPERTY_USER_NAME,
    Uri_PROPERTY_HOST_TYPE = 15,
    Uri_PROPERTY_DWORD_START = Uri_PROPERTY_HOST_TYPE,
    Uri_PROPERTY_PORT = 16,
    Uri_PROPERTY_SCHEME = 17,
    Uri_PROPERTY_ZONE = 18,
    Uri_PROPERTY_DWORD_LAST = Uri_PROPERTY_ZONE,
}}
ENUM!{enum Uri_HOST_TYPE {
    Uri_HOST_UNKNOWN = 0,
    Uri_HOST_DNS = Uri_HOST_UNKNOWN + 1,
    Uri_HOST_IPV4 = Uri_HOST_DNS + 1,
    Uri_HOST_IPV6 = Uri_HOST_IPV4 + 1,
    Uri_HOST_IDN = Uri_HOST_IPV6 + 1,
}}
RIDL!(#[uuid(0xa39ee748, 0x6a27, 0x4817, 0xa6, 0xf2, 0x13, 0x91, 0x4b, 0xef, 0x58, 0x90)]
interface IUri(IUriVtbl): IUnknown(IUnknownVtbl) {
    fn GetPropertyBSTR(
        uriProp: Uri_PROPERTY,
        pbstrProperty: *mut BSTR,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn GetPropertyLength(
        uriProp: Uri_PROPERTY,
        pcchProperty: *mut DWORD,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn GetPropertyDWORD(
        uriProp: Uri_PROPERTY,
        pdwProperty: *mut DWORD,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn HasProperty(
        uriProp: Uri_PROPERTY,
        pfHasProperty: *mut BOOL,
    ) -> HRESULT,
    fn GetAbsoluteUri(
        pbstrAbsoluteUri: *mut BSTR,
    ) -> HRESULT,
    fn GetAuthority(
        pbstrAuthority: *mut BSTR,
    ) -> HRESULT,
    fn GetDisplayUri(
        pbstrDisplayString: *mut BSTR,
    ) -> HRESULT,
    fn GetDomain(
        pbstrDomain: *mut BSTR,
    ) -> HRESULT,
    fn GetExtension(
        pbstrExtension: *mut BSTR,
    ) -> HRESULT,
    fn GetFragment(
        pbstrFragment: *mut BSTR,
    ) -> HRESULT,
    fn GetHost(
        pbstrHost: *mut BSTR,
    ) -> HRESULT,
    fn GetPassword(
        pbstrPassword: *mut BSTR,
    ) -> HRESULT,
    fn GetPath(
        pbstrPath: *mut BSTR,
    ) -> HRESULT,
    fn GetPathAndQuery(
        pbstrPathAndQuery: *mut BSTR,
    ) -> HRESULT,
    fn GetQuery(
        pbstrQuery: *mut BSTR,
    ) -> HRESULT,
    fn GetRawUri(
        pbstrRawUri: *mut BSTR,
    ) -> HRESULT,
    fn GetSchemeName(
        pbstrSchemeName: *mut BSTR,
    ) -> HRESULT,
    fn GetUserInfo(
        pbstrUserInfo: *mut BSTR,
    ) -> HRESULT,
    fn GetUserName(
        pbstrUserName: *mut BSTR,
    ) -> HRESULT,
    fn GetHostType(
        pdwHostType: *mut DWORD,
    ) -> HRESULT,
    fn GetPort(
        pdwPort: *mut DWORD,
    ) -> HRESULT,
    fn GetScheme(
        pdwScheme: *mut DWORD,
    ) -> HRESULT,
    fn GetZone(
        pdwZone: *mut DWORD,
    ) -> HRESULT,
    fn GetProperties(
        pdwFlags: LPDWORD,
    ) -> HRESULT,
    fn IsEqual(
        pUri: *mut IUri,
        pfEqual: *mut BOOL,
    ) -> HRESULT,
});
extern "system" {
    pub fn CreateUri(
        pwzURI: LPCWSTR,
        dwFlags: DWORD,
        dwReserved: DWORD_PTR,
        ppURI: *mut *mut IUri,
    ) -> HRESULT;
    pub fn CreateUriWithFragment(
        pwzURI: LPCWSTR,
        pwzFragment: LPCWSTR,
        dwFlags: DWORD,
        dwReserved: DWORD_PTR,
        ppURI: *mut *mut IUri,
    ) -> HRESULT;
    pub fn CreateUriFromMultiByteString(
        pszANSIInputUri: LPCSTR,
        dwEncodingFlags: DWORD,
        dwCodePage: DWORD,
        dwCreateFlags: DWORD,
        dwReserved: DWORD_PTR,
        ppUri: *mut *mut IUri,
    ) -> HRESULT;
}
pub const Uri_HAS_ABSOLUTE_URI: DWORD = 1 << Uri_PROPERTY_ABSOLUTE_URI;
pub const Uri_HAS_AUTHORITY: DWORD = 1 << Uri_PROPERTY_AUTHORITY;
pub const Uri_HAS_DISPLAY_URI: DWORD = 1 << Uri_PROPERTY_DISPLAY_URI;
pub const Uri_HAS_DOMAIN: DWORD = 1 << Uri_PROPERTY_DOMAIN;
pub const Uri_HAS_EXTENSION: DWORD = 1 << Uri_PROPERTY_EXTENSION;
pub const Uri_HAS_FRAGMENT: DWORD = 1 << Uri_PROPERTY_FRAGMENT;
pub const Uri_HAS_HOST: DWORD = 1 << Uri_PROPERTY_HOST;
pub const Uri_HAS_PASSWORD: DWORD = 1 << Uri_PROPERTY_PASSWORD;
pub const Uri_HAS_PATH: DWORD = 1 << Uri_PROPERTY_PATH;
pub const Uri_HAS_QUERY: DWORD = 1 << Uri_PROPERTY_QUERY;
pub const Uri_HAS_RAW_URI: DWORD = 1 << Uri_PROPERTY_RAW_URI;
pub const Uri_HAS_SCHEME_NAME: DWORD = 1 << Uri_PROPERTY_SCHEME_NAME;
pub const Uri_HAS_USER_NAME: DWORD = 1 << Uri_PROPERTY_USER_NAME;
pub const Uri_HAS_PATH_AND_QUERY: DWORD = 1 << Uri_PROPERTY_PATH_AND_QUERY;
pub const Uri_HAS_USER_INFO: DWORD = 1 << Uri_PROPERTY_USER_INFO;
pub const Uri_HAS_HOST_TYPE: DWORD = 1 << Uri_PROPERTY_HOST_TYPE;
pub const Uri_HAS_PORT: DWORD = 1 << Uri_PROPERTY_PORT;
pub const Uri_HAS_SCHEME: DWORD = 1 << Uri_PROPERTY_SCHEME;
pub const Uri_HAS_ZONE: DWORD = 1 << Uri_PROPERTY_ZONE;
pub const Uri_CREATE_ALLOW_RELATIVE: DWORD = 0x00000001;
pub const Uri_CREATE_ALLOW_IMPLICIT_WILDCARD_SCHEME: DWORD = 0x00000002;
pub const Uri_CREATE_ALLOW_IMPLICIT_FILE_SCHEME: DWORD = 0x00000004;
pub const Uri_CREATE_NOFRAG: DWORD = 0x00000008;
pub const Uri_CREATE_NO_CANONICALIZE: DWORD = 0x00000010;
pub const Uri_CREATE_CANONICALIZE: DWORD = 0x00000100;
pub const Uri_CREATE_FILE_USE_DOS_PATH: DWORD = 0x00000020;
pub const Uri_CREATE_DECODE_EXTRA_INFO: DWORD = 0x00000040;
pub const Uri_CREATE_NO_DECODE_EXTRA_INFO: DWORD = 0x00000080;
pub const Uri_CREATE_CRACK_UNKNOWN_SCHEMES: DWORD = 0x00000200;
pub const Uri_CREATE_NO_CRACK_UNKNOWN_SCHEMES: DWORD = 0x00000400;
pub const Uri_CREATE_PRE_PROCESS_HTML_URI: DWORD = 0x00000800;
pub const Uri_CREATE_NO_PRE_PROCESS_HTML_URI: DWORD = 0x00001000;
pub const Uri_CREATE_IE_SETTINGS: DWORD = 0x00002000;
pub const Uri_CREATE_NO_IE_SETTINGS: DWORD = 0x00004000;
pub const Uri_CREATE_NO_ENCODE_FORBIDDEN_CHARACTERS: DWORD = 0x00008000;
pub const Uri_CREATE_NORMALIZE_INTL_CHARACTERS: DWORD = 0x00010000;
pub const Uri_CREATE_CANONICALIZE_ABSOLUTE: DWORD = 0x00020000;
pub const Uri_DISPLAY_NO_FRAGMENT: DWORD = 0x00000001;
pub const Uri_PUNYCODE_IDN_HOST: DWORD = 0x00000002;
pub const Uri_DISPLAY_IDN_HOST: DWORD = 0x00000004;
pub const Uri_DISPLAY_NO_PUNYCODE: DWORD = 0x00000008;
pub const Uri_ENCODING_USER_INFO_AND_PATH_IS_PERCENT_ENCODED_UTF8: DWORD = 0x00000001;
pub const Uri_ENCODING_USER_INFO_AND_PATH_IS_CP: DWORD = 0x00000002;
pub const Uri_ENCODING_HOST_IS_IDN: DWORD = 0x00000004;
pub const Uri_ENCODING_HOST_IS_PERCENT_ENCODED_UTF8: DWORD = 0x00000008;
pub const Uri_ENCODING_HOST_IS_PERCENT_ENCODED_CP: DWORD = 0x00000010;
pub const Uri_ENCODING_QUERY_AND_FRAGMENT_IS_PERCENT_ENCODED_UTF8: DWORD = 0x00000020;
pub const Uri_ENCODING_QUERY_AND_FRAGMENT_IS_CP: DWORD = 0x00000040;
pub const Uri_ENCODING_RFC: DWORD = Uri_ENCODING_USER_INFO_AND_PATH_IS_PERCENT_ENCODED_UTF8 |
    Uri_ENCODING_HOST_IS_PERCENT_ENCODED_UTF8 |
    Uri_ENCODING_QUERY_AND_FRAGMENT_IS_PERCENT_ENCODED_UTF8;
pub const UriBuilder_USE_ORIGINAL_FLAGS: DWORD = 0x00000001;
RIDL!(#[uuid(0xa158a630, 0xed6f, 0x45fb, 0xb9, 0x87, 0xf6, 0x86, 0x76, 0xf5, 0x77, 0x52)]
interface IUriContainer(IUriContainerVtbl): IUnknown(IUnknownVtbl) {
    fn GetIUri(
        ppIUri: *mut *mut IUri,
    ) -> HRESULT,
});
RIDL!(#[uuid(0x4221b2e1, 0x8955, 0x46c0, 0xbd, 0x5b, 0xde, 0x98, 0x97, 0x56, 0x5d, 0xe7)]
interface IUriBuilder(IUriBuilderVtbl): IUnknown(IUnknownVtbl) {
    fn CreateUriSimple(
        dwAllowEncodingPropertyMask: DWORD,
        dwReserved: DWORD_PTR,
        ppIUri: *mut *mut IUri,
    ) -> HRESULT,
    fn CreateUri(
        dwCreateFlags: DWORD,
        dwAllowEncodingPropertyMask: DWORD,
        dwReserved: DWORD_PTR,
        ppIUri: *mut *mut IUri,
    ) -> HRESULT,
    fn CreateUriWithFlags(
        dwCreateFlags: DWORD,
        dwUriBuilderFlags: DWORD,
        dwAllowEncodingPropertyMask: DWORD,
        dwReserved: DWORD_PTR,
        ppIUri: *mut *mut IUri,
    ) -> HRESULT,
    fn GetIUri(
        ppIUri: *mut *mut IUri,
    ) -> HRESULT,
    fn SetIUri(
        pIUri: *mut IUri,
    ) -> HRESULT,
    fn GetFragment(
        pcchFragment: *mut DWORD,
        ppwzFragment: *mut LPCWSTR,
    ) -> HRESULT,
    fn GetHost(
        pcchHost: *mut DWORD,
        ppwzHost: *mut LPCWSTR,
    ) -> HRESULT,
    fn GetPassword(
        pcchPassword: *mut DWORD,
        ppwzPassword: *mut LPCWSTR,
    ) -> HRESULT,
    fn GetPath(
        pcchPath: *mut DWORD,
        ppwzPath: *mut LPCWSTR,
    ) -> HRESULT,
    fn GetPort(
        pfHasPort: *mut BOOL,
        pdwPort: *mut DWORD,
    ) -> HRESULT,
    fn GetQuery(
        pcchQuery: *mut DWORD,
        ppwzQuery: *mut LPCWSTR,
    ) -> HRESULT,
    fn GetSchemeName(
        pcchSchemeName: *mut DWORD,
        ppwzSchemeName: *mut LPCWSTR,
    ) -> HRESULT,
    fn GetUserName(
        pcchUserName: *mut DWORD,
        ppwzUserName: *mut LPCWSTR,
    ) -> HRESULT,
    fn SetFragment(
        pwzNewValue: LPCWSTR,
    ) -> HRESULT,
    fn SetHost(
        pwzNewValue: LPCWSTR,
    ) -> HRESULT,
    fn SetPassword(
        pwzNewValue: LPCWSTR,
    ) -> HRESULT,
    fn SetPath(
        pwzNewValue: LPCWSTR,
    ) -> HRESULT,
    fn SetPort(
        fHasPort: BOOL,
        dwNewValue: DWORD,
    ) -> HRESULT,
    fn SetQuery(
        pwzNewValue: LPCWSTR,
    ) -> HRESULT,
    fn SetSchemeName(
        pwzNewValue: LPCWSTR,
    ) -> HRESULT,
    fn SetUserName(
        pwzNewValue: LPCWSTR,
    ) -> HRESULT,
    fn RemoveProperties(
        dwPropertyMask: DWORD,
    ) -> HRESULT,
    fn HasBeenModified(
        pfModified: *mut BOOL,
    ) -> HRESULT,
});
RIDL!(#[uuid(0xe982ce48, 0x0b96, 0x440c, 0xbc, 0x37, 0x0c, 0x86, 0x9b, 0x27, 0xa2, 0x9e)]
interface IUriBuilderFactory(IUriBuilderFactoryVtbl): IUnknown(IUnknownVtbl) {
    fn CreateIUriBuilder(
        dwFlags: DWORD,
        dwReserved: DWORD_PTR,
        ppIUriBuilder: *mut *mut IUriBuilder,
    ) -> HRESULT,
    fn CreateInitializedIUriBuilder(
        dwFlags: DWORD,
        dwReserved: DWORD_PTR,
        ppIUriBuilder: *mut *mut IUriBuilder,
    ) -> HRESULT,
});
extern "system" {
    pub fn CreateIUriBuilder(
        pIUri: *mut IUri,
        dwFlags: DWORD,
        dwReserved: DWORD_PTR,
        ppIUriBuilder: *mut *mut IUriBuilder,
    ) -> HRESULT;
}
pub type LPWININETINFO = *mut IWinInetInfo;
RIDL!(#[uuid(0x79eac9d6, 0xbafa, 0x11ce, 0x8c, 0x82, 0x00, 0xaa, 0x00, 0x4b, 0xa9, 0x0b)]
interface IWinInetInfo(IWinInetInfoVtbl): IUnknown(IUnknownVtbl) {
    fn QueryOption(
        dwOption: DWORD,
        pBuffer: LPVOID,
        pcbBuf: *mut DWORD,
    ) -> HRESULT,
});
pub const WININETINFO_OPTION_LOCK_HANDLE: DWORD = 65534;
pub type LPHTTPSECURITY = *mut IHttpSecurity;
RIDL!(#[uuid(0x79eac9d7, 0xbafa, 0x11ce, 0x8c, 0x82, 0x00, 0xaa, 0x00, 0x4b, 0xa9, 0x0b)]
interface IHttpSecurity(IHttpSecurityVtbl): IWindowForBindingUI(IWindowForBindingUIVtbl) {
    fn OnSecurityProblem(
        dwProblem: DWORD,
    ) -> HRESULT,
});
pub type LPWININETHTTPINFO = *mut IWinInetHttpInfo;
RIDL!(#[uuid(0x79eac9d8, 0xbafa, 0x11ce, 0x8c, 0x82, 0x00, 0xaa, 0x00, 0x4b, 0xa9, 0x0b)]
interface IWinInetHttpInfo(IWinInetHttpInfoVtbl): IWinInetInfo(IWinInetInfoVtbl) {
    fn QueryInfo(
        dwOption: DWORD,
        pBuffer: LPVOID,
        pcbBuf: *mut DWORD,
        pdwFlags: *mut DWORD,
        pdwReserved: *mut DWORD,
    ) -> HRESULT,
});
RIDL!(#[uuid(0xf286fa56, 0xc1fd, 0x4270, 0x8e, 0x67, 0xb3, 0xeb, 0x79, 0x0a, 0x81, 0xe8)]
interface IWinInetHttpTimeouts(IWinInetHttpTimeoutsVtbl): IUnknown(IUnknownVtbl) {
    fn GetRequestTimeouts(
        pdwConnectTimeout: *mut DWORD,
        pdwSendTimeout: *mut DWORD,
        pdwReceiveTimeout: *mut DWORD,
    ) -> HRESULT,
});
pub type LPWININETCACHEHINTS = *mut IWinInetCacheHints;
RIDL!(#[uuid(0xdd1ec3b3, 0x8391, 0x4fdb, 0xa9, 0xe6, 0x34, 0x7c, 0x3c, 0xaa, 0xa7, 0xdd)]
interface IWinInetCacheHints(IWinInetCacheHintsVtbl): IUnknown(IUnknownVtbl) {
    fn SetCacheExtension(
        pwzExt: LPCWSTR,
        pszCacheFile: LPVOID,
        pcbCacheFile: *mut DWORD,
        pdwWinInetError: *mut DWORD,
        pdwReserved: *mut DWORD,
    ) -> HRESULT,
});
pub type LPWININETCACHEHINTS2 = *mut IWinInetCacheHints2;
RIDL!(#[uuid(0x7857aeac, 0xd31f, 0x49bf, 0x88, 0x4e, 0xdd, 0x46, 0xdf, 0x36, 0x78, 0x0a)]
interface IWinInetCacheHints2(IWinInetCacheHints2Vtbl):
    IWinInetCacheHints(IWinInetCacheHintsVtbl) {
    fn SetCacheExtension2(
        pwzExt: LPCWSTR,
        pwzCacheFile: *mut WCHAR,
        pcchCacheFile: *mut DWORD,
        pdwWinInetError: *mut DWORD,
        pdwReserved: *mut DWORD,
    ) -> HRESULT,
});
pub type LPBINDHOST = *mut IBindHost;
RIDL!(#[uuid(0xfc4801a1, 0x2ba9, 0x11cf, 0xa2, 0x29, 0x00, 0xaa, 0x00, 0x3d, 0x73, 0x52)]
interface IBindHost(IBindHostVtbl): IUnknown(IUnknownVtbl) {
    fn CreateMoniker(
        szName: LPOLESTR,
        pBC: *mut IBindCtx,
        ppmk: *mut *mut IMoniker,
        dwReserved: DWORD,
    ) -> HRESULT,
    fn MonikerBindToStorage(
        pMk: *mut IMoniker,
        pBC: *mut IBindCtx,
        pBSC: *mut IBindStatusCallback,
        riid: REFIID,
        ppvObj: *mut *mut c_void,
    ) -> HRESULT,
    fn MonikerBindToObject(
        pMk: *mut IMoniker,
        pBC: *mut IBindCtx,
        pBSC: *mut IBindStatusCallback,
        riid: REFIID,
        ppvObj: *mut *mut c_void,
    ) -> HRESULT,
});
pub const URLOSTRM_USECACHEDCOPY_ONLY: DWORD = 0x1;
pub const URLOSTRM_USECACHEDCOPY: DWORD = 0x2;
pub const URLOSTRM_GETNEWESTVERSION: DWORD = 0x3;
extern "system" {
    pub fn HlinkSimpleNavigateToString(
        szTarget: LPCWSTR,
        szLocation: LPCWSTR,
        szTargetFrameName: LPCWSTR,
        pUnk: *mut IUnknown,
        pbc: *mut IBindCtx,
        pbsc: *mut IBindStatusCallback,
        grfHLNF: DWORD,
        dwReserved: DWORD,
    ) -> HRESULT;
    pub fn HlinkSimpleNavigateToMoniker(
        pmkTarget: *mut IMoniker,
        szLocation: LPCWSTR,
        szTargetFrameName: LPCWSTR,
        pUnk: *mut IUnknown,
        pbc: *mut IBindCtx,
        pbsc: *mut IBindStatusCallback,
        grfHLNF: DWORD,
        dwReserved: DWORD,
    ) -> HRESULT;
    pub fn URLOpenStreamA(
        pCaller: LPUNKNOWN,
        szURL: LPCSTR,
        dwReserved: DWORD,
        lpfnCB: LPBINDSTATUSCALLBACK,
    ) -> HRESULT;
    pub fn URLOpenStreamW(
        pCaller: LPUNKNOWN,
        szURL: LPCWSTR,
        dwReserved: DWORD,
        lpfnCB: LPBINDSTATUSCALLBACK,
    ) -> HRESULT;
    pub fn URLOpenPullStreamA(
        pCaller: LPUNKNOWN,
        szURL: LPCSTR,
        dwReserved: DWORD,
        lpfnCB: LPBINDSTATUSCALLBACK,
    ) -> HRESULT;
    pub fn URLOpenPullStreamW(
        pCaller: LPUNKNOWN,
        szURL: LPCWSTR,
        dwReserved: DWORD,
        lpfnCB: LPBINDSTATUSCALLBACK,
    ) -> HRESULT;
    pub fn URLDownloadToFileA(
        pCaller: LPUNKNOWN,
        szURL: LPCSTR,
        szFileName: LPCSTR,
        dwReserved: DWORD,
        lpfnCB: LPBINDSTATUSCALLBACK,
    ) -> HRESULT;
    pub fn URLDownloadToFileW(
        pCaller: LPUNKNOWN,
        szURL: LPCWSTR,
        szFileName: LPCWSTR,
        dwReserved: DWORD,
        lpfnCB: LPBINDSTATUSCALLBACK,
    ) -> HRESULT;
    pub fn URLDownloadToCacheFileA(
        lpUnkcaller: LPUNKNOWN,
        szURL: LPCSTR,
        szFileName: LPSTR,
        cchFileName: DWORD,
        dwReserved: DWORD,
        pBSC: IBindStatusCallback,
    ) -> HRESULT;
    pub fn URLDownloadToCacheFileW(
        lpUnkcaller: LPUNKNOWN,
        szURL: LPCWSTR,
        szFileName: LPWSTR,
        cchFileName: DWORD,
        dwReserved: DWORD,
        pBSC: IBindStatusCallback,
    ) -> HRESULT;
    pub fn URLOpenBlockingStreamA(
        pCaller: LPUNKNOWN,
        szURL: LPCSTR,
        ppStream: *mut LPSTREAM,
        dwReserved: DWORD,
        lpfnCB: LPBINDSTATUSCALLBACK,
    ) -> HRESULT;
    pub fn URLOpenBlockingStreamW(
        pCaller: LPUNKNOWN,
        szURL: LPCWSTR,
        ppStream: *mut LPSTREAM,
        dwReserved: DWORD,
        lpfnCB: LPBINDSTATUSCALLBACK,
    ) -> HRESULT;
    pub fn HlinkGoBack(
        pUnk: *mut IUnknown,
    ) -> HRESULT;
    pub fn HlinkGoForward(
        pUnk: *mut IUnknown,
    ) -> HRESULT;
    pub fn HlinkNavigateString(
        pUnk: *mut IUnknown,
        szTarget: LPCWSTR,
    ) -> HRESULT;
    pub fn HlinkNavigateMoniker(
        pUnk: *mut IUnknown,
        pmkTarget: *mut IMoniker,
    ) -> HRESULT;
}
pub type LPIINTERNET = *mut IInternet;
RIDL!(#[uuid(0x79eac9e0, 0xbaf9, 0x11ce, 0x8c, 0x82, 0x00, 0xaa, 0x00, 0x4b, 0xa9, 0x0b)]
interface IInternet(IInternetVtbl): IUnknown(IUnknownVtbl) {
});
pub type LPIINTERNETBINDINFO = *mut IInternetBindInfo;
ENUM!{enum BINDSTRING {
    BINDSTRING_HEADERS = 1,
    BINDSTRING_ACCEPT_MIMES = BINDSTRING_HEADERS + 1,
    BINDSTRING_EXTRA_URL = BINDSTRING_ACCEPT_MIMES + 1,
    BINDSTRING_LANGUAGE = BINDSTRING_EXTRA_URL + 1,
    BINDSTRING_USERNAME = BINDSTRING_LANGUAGE + 1,
    BINDSTRING_PASSWORD = BINDSTRING_USERNAME + 1,
    BINDSTRING_UA_PIXELS = BINDSTRING_PASSWORD + 1,
    BINDSTRING_UA_COLOR = BINDSTRING_UA_PIXELS + 1,
    BINDSTRING_OS = BINDSTRING_UA_COLOR + 1,
    BINDSTRING_USER_AGENT = BINDSTRING_OS + 1,
    BINDSTRING_ACCEPT_ENCODINGS = BINDSTRING_USER_AGENT + 1,
    BINDSTRING_POST_COOKIE = BINDSTRING_ACCEPT_ENCODINGS + 1,
    BINDSTRING_POST_DATA_MIME = BINDSTRING_POST_COOKIE + 1,
    BINDSTRING_URL = BINDSTRING_POST_DATA_MIME + 1,
    BINDSTRING_IID = BINDSTRING_URL + 1,
    BINDSTRING_FLAG_BIND_TO_OBJECT = BINDSTRING_IID + 1,
    BINDSTRING_PTR_BIND_CONTEXT = BINDSTRING_FLAG_BIND_TO_OBJECT + 1,
    BINDSTRING_XDR_ORIGIN = BINDSTRING_PTR_BIND_CONTEXT + 1,
    BINDSTRING_DOWNLOADPATH = BINDSTRING_XDR_ORIGIN + 1,
    BINDSTRING_ROOTDOC_URL = BINDSTRING_DOWNLOADPATH + 1,
    BINDSTRING_INITIAL_FILENAME = BINDSTRING_ROOTDOC_URL + 1,
    BINDSTRING_PROXY_USERNAME = BINDSTRING_INITIAL_FILENAME + 1,
    BINDSTRING_PROXY_PASSWORD = BINDSTRING_PROXY_USERNAME + 1,
    BINDSTRING_ENTERPRISE_ID = BINDSTRING_PROXY_PASSWORD + 1,
    BINDSTRING_DOC_URL = BINDSTRING_ENTERPRISE_ID + 1,
}}
RIDL!(#[uuid(0x79eac9e1, 0xbaf9, 0x11ce, 0x8c, 0x82, 0x00, 0xaa, 0x00, 0x4b, 0xa9, 0x0b)]
interface IInternetBindInfo(IInternetBindInfoVtbl): IUnknown(IUnknownVtbl) {
    fn GetBindInfo(
        grfBINDF: *mut DWORD,
        pbindinfo: *mut BINDINFO,
    ) -> HRESULT,
    fn GetBindString(
        ulStringType: ULONG,
        ppwzStr: *mut LPOLESTR,
        cEl: ULONG,
        pcElFetched: *mut ULONG,
    ) -> HRESULT,
});
pub type LPIINTERNETBINDINFOEX = *mut IInternetBindInfoEx;
RIDL!(#[uuid(0xa3e015b7, 0xa82c, 0x4dcd, 0xa1, 0x50, 0x56, 0x9a, 0xee, 0xed, 0x36, 0xab)]
interface IInternetBindInfoEx(IInternetBindInfoExVtbl): IInternetBindInfo(IInternetBindInfoVtbl) {
    fn GetBindInfoEx(
        grfBINDF: *mut DWORD,
        pbindinfo: *mut BINDINFO,
        grfBINDF2: *mut DWORD,
        pdwReserved: *mut DWORD,
    ) -> HRESULT,
});
pub type LPIINTERNETPROTOCOLROOT = *mut IInternetProtocolRoot;
ENUM!{enum PI_FLAGS {
    PI_PARSE_URL = 0x1,
    PI_FILTER_MODE = 0x2,
    PI_FORCE_ASYNC = 0x4,
    PI_USE_WORKERTHREAD = 0x8,
    PI_MIMEVERIFICATION = 0x10,
    PI_CLSIDLOOKUP = 0x20,
    PI_DATAPROGRESS = 0x40,
    PI_SYNCHRONOUS = 0x80,
    PI_APARTMENTTHREADED = 0x100,
    PI_CLASSINSTALL = 0x200,
    PI_PASSONBINDCTX = 0x2000,
    PI_NOMIMEHANDLER = 0x8000,
    PI_LOADAPPDIRECT = 0x4000,
    PD_FORCE_SWITCH = 0x10000,
    PI_PREFERDEFAULTHANDLER = 0x20000,
}}
STRUCT!{struct PROTOCOLDATA {
    grfFlags: DWORD,
    dwState: DWORD,
    pData: LPVOID,
    cbData: ULONG,
}}
STRUCT!{struct StartParam {
    iid: IID,
    pIBindCtx: *mut IBindCtx,
    pItf: *mut IUnknown,
}}
RIDL!(#[uuid(0x79eac9e3, 0xbaf9, 0x11ce, 0x8c, 0x82, 0x00, 0xaa, 0x00, 0x4b, 0xa9, 0x0b)]
interface IInternetProtocolRoot(IInternetProtocolRootVtbl): IUnknown(IUnknownVtbl) {
    fn Start(
        szUrl: LPCWSTR,
        pOIProtSink: *mut IInternetProtocolSink,
        pOIBindInfo: *mut IInternetBindInfo,
        grfPI: DWORD,
        dwReserved: HANDLE_PTR,
    ) -> HRESULT,
    fn Continue(
        pProtocolData: *mut PROTOCOLDATA,
    ) -> HRESULT,
    fn Abort(
        hrReason: HRESULT,
        dwOptions: DWORD,
    ) -> HRESULT,
    fn Terminate(
        dwOptions: DWORD,
    ) -> HRESULT,
    fn Suspend() -> HRESULT,
    fn Resume() -> HRESULT,
});
pub type LPIINTERNETPROTOCOL = *mut IInternetProtocol;
RIDL!(#[uuid(0x79eac9e4, 0xbaf9, 0x11ce, 0x8c, 0x82, 0x00, 0xaa, 0x00, 0x4b, 0xa9, 0x0b)]
interface IInternetProtocol(IInternetProtocolVtbl):
    IInternetProtocolRoot(IInternetProtocolRootVtbl) {
    fn Read(
        pv: *mut c_void,
        cb: ULONG,
        pcbRead: *mut ULONG,
    ) -> HRESULT,
    fn Seek(
        dlibMove: LARGE_INTEGER,
        dwOrigin: DWORD,
        plibNewPosition: *mut ULARGE_INTEGER,
    ) -> HRESULT,
    fn LockRequest(
        dwOptions: DWORD,
    ) -> HRESULT,
    fn UnlockRequest() -> HRESULT,
});
RIDL!(#[uuid(0xc7a98e66, 0x1010, 0x492c, 0xa1, 0xc8, 0xc8, 0x09, 0xe1, 0xf7, 0x59, 0x05)]
interface IInternetProtocolEx(IInternetProtocolExVtbl): IInternetProtocol(IInternetProtocolVtbl) {
    fn StartEx(
        pUri: *mut IUri,
        pOIProtSink: *mut IInternetProtocolSink,
        pOIBindInfo: *mut IInternetBindInfo,
        grfPI: DWORD,
        dwReserved: HANDLE_PTR,
    ) -> HRESULT,
});
pub type LPIINTERNETPROTOCOLSINK = *mut IInternetProtocolSink;
RIDL!(#[uuid(0x79eac9e5, 0xbaf9, 0x11ce, 0x8c, 0x82, 0x00, 0xaa, 0x00, 0x4b, 0xa9, 0x0b)]
interface IInternetProtocolSink(IInternetProtocolSinkVtbl): IUnknown(IUnknownVtbl) {
    fn Switch(
        pProtocolData: *mut PROTOCOLDATA,
    ) -> HRESULT,
    fn ReportProgress(
        ulStatusCode: ULONG,
        szStatusText: LPCWSTR,
    ) -> HRESULT,
    fn ReportData(
        grfBSCF: DWORD,
        ulProgress: ULONG,
        ulProgressMax: ULONG,
    ) -> HRESULT,
    fn ReportResult(
        hrResult: HRESULT,
        dwError: DWORD,
        szResult: LPCWSTR,
    ) -> HRESULT,
});
pub type LPIINTERNETPROTOCOLSINKStackable = *mut IInternetProtocolSinkStackable;
RIDL!(#[uuid(0x79eac9f0, 0xbaf9, 0x11ce, 0x8c, 0x82, 0x00, 0xaa, 0x00, 0x4b, 0xa9, 0x0b)]
interface IInternetProtocolSinkStackable(IInternetProtocolSinkStackableVtbl):
    IUnknown(IUnknownVtbl) {
    fn SwitchSink(
        pOIProtSink: *mut IInternetProtocolSink,
    ) -> HRESULT,
    fn CommitSwitch() -> HRESULT,
    fn RollbackSwitch() -> HRESULT,
});
pub type LPIINTERNETSESSION = *mut IInternetSession;
ENUM!{enum OIBDG_FLAGS {
    OIBDG_APARTMENTTHREADED = 0x100,
    OIBDG_DATAONLY = 0x1000,
}}
RIDL!(#[uuid(0x79eac9e7, 0xbaf9, 0x11ce, 0x8c, 0x82, 0x00, 0xaa, 0x00, 0x4b, 0xa9, 0x0b)]
interface IInternetSession(IInternetSessionVtbl): IUnknown(IUnknownVtbl) {
    fn RegisterNameSpace(
        pCF: *mut IClassFactory,
        rclsid: REFCLSID,
        pwzProtocol: LPCWSTR,
        cPatterns: ULONG,
        ppwzPatterns: *mut LPCWSTR,
        dwReserved: DWORD,
    ) -> HRESULT,
    fn UnregisterNameSpace(
        pCF: *mut IClassFactory,
        pszProtocol: LPCWSTR,
    ) -> HRESULT,
    fn RegisterMimeFilter(
        pCF: *mut IClassFactory,
        rclsid: REFCLSID,
        pwzType: LPCWSTR,
    ) -> HRESULT,
    fn UnregisterMimeFilter(
        pCF: *mut IClassFactory,
        pwzType: LPCWSTR,
    ) -> HRESULT,
    fn CreateBinding(
        pBC: LPBC,
        szUrl: LPCWSTR,
        pUnkOuter: *mut IUnknown,
        ppUnk: *mut *mut IUnknown,
        ppOInetProt: *mut *mut IInternetProtocol,
        dwOption: DWORD,
    ) -> HRESULT,
    fn SetSessionOption(
        dwOption: DWORD,
        pBuffer: LPVOID,
        dwBufferLength: DWORD,
        dwReserved: DWORD,
    ) -> HRESULT,
    fn GetSessionOption(
        dwOption: DWORD,
        pBuffer: LPVOID,
        pdwBufferLength: *mut DWORD,
        dwReserved: DWORD,
    ) -> HRESULT,
});
pub type LPIINTERNETTHREADSWITCH = *mut IInternetThreadSwitch;
RIDL!(#[uuid(0x79eac9e8, 0xbaf9, 0x11ce, 0x8c, 0x82, 0x00, 0xaa, 0x00, 0x4b, 0xa9, 0x0b)]
interface IInternetThreadSwitch(IInternetThreadSwitchVtbl): IUnknown(IUnknownVtbl) {
    fn Prepare() -> HRESULT,
    fn Continue() -> HRESULT,
});
pub type LPIINTERNETPRIORITY = *mut IInternetPriority;
RIDL!(#[uuid(0x79eac9eb, 0xbaf9, 0x11ce, 0x8c, 0x82, 0x00, 0xaa, 0x00, 0x4b, 0xa9, 0x0b)]
interface IInternetPriority(IInternetPriorityVtbl): IUnknown(IUnknownVtbl) {
    fn SetPriority(
        nPriority: LONG,
    ) -> HRESULT,
    fn GetPriority(
        pnPriority: *mut LONG,
    ) -> HRESULT,
});
pub type LPIINTERNETPROTOCOLINFO = *mut IInternetProtocolInfo;
ENUM!{enum PARSEACTION {
    PARSE_CANONICALIZE = 1,
    PARSE_FRIENDLY = PARSE_CANONICALIZE + 1,
    PARSE_SECURITY_URL = PARSE_FRIENDLY + 1,
    PARSE_ROOTDOCUMENT = PARSE_SECURITY_URL + 1,
    PARSE_DOCUMENT = PARSE_ROOTDOCUMENT + 1,
    PARSE_ANCHOR = PARSE_DOCUMENT + 1,
    PARSE_ENCODE_IS_UNESCAPE = PARSE_ANCHOR + 1,
    PARSE_DECODE_IS_ESCAPE = PARSE_ENCODE_IS_UNESCAPE + 1,
    PARSE_PATH_FROM_URL = PARSE_DECODE_IS_ESCAPE + 1,
    PARSE_URL_FROM_PATH = PARSE_PATH_FROM_URL + 1,
    PARSE_MIME = PARSE_URL_FROM_PATH + 1,
    PARSE_SERVER = PARSE_MIME + 1,
    PARSE_SCHEMA = PARSE_SERVER + 1,
    PARSE_SITE = PARSE_SCHEMA + 1,
    PARSE_DOMAIN = PARSE_SITE + 1,
    PARSE_LOCATION = PARSE_DOMAIN + 1,
    PARSE_SECURITY_DOMAIN = PARSE_LOCATION + 1,
    PARSE_ESCAPE = PARSE_SECURITY_DOMAIN + 1,
    PARSE_UNESCAPE = PARSE_ESCAPE + 1,
}}
ENUM!{enum PSUACTION {
    PSU_DEFAULT = 1,
    PSU_SECURITY_URL_ONLY = PSU_DEFAULT + 1,
}}
ENUM!{enum QUERYOPTION {
    QUERY_EXPIRATION_DATE = 1,
    QUERY_TIME_OF_LAST_CHANGE = QUERY_EXPIRATION_DATE + 1,
    QUERY_CONTENT_ENCODING = QUERY_TIME_OF_LAST_CHANGE + 1,
    QUERY_CONTENT_TYPE = QUERY_CONTENT_ENCODING + 1,
    QUERY_REFRESH = QUERY_CONTENT_TYPE + 1,
    QUERY_RECOMBINE = QUERY_REFRESH + 1,
    QUERY_CAN_NAVIGATE = QUERY_RECOMBINE + 1,
    QUERY_USES_NETWORK = QUERY_CAN_NAVIGATE + 1,
    QUERY_IS_CACHED = QUERY_USES_NETWORK + 1,
    QUERY_IS_INSTALLEDENTRY = QUERY_IS_CACHED + 1,
    QUERY_IS_CACHED_OR_MAPPED = QUERY_IS_INSTALLEDENTRY + 1,
    QUERY_USES_CACHE = QUERY_IS_CACHED_OR_MAPPED + 1,
    QUERY_IS_SECURE = QUERY_USES_CACHE + 1,
    QUERY_IS_SAFE = QUERY_IS_SECURE + 1,
    QUERY_USES_HISTORYFOLDER = QUERY_IS_SAFE + 1,
    QUERY_IS_CACHED_AND_USABLE_OFFLINE = QUERY_USES_HISTORYFOLDER + 1,
}}
RIDL!(#[uuid(0x79eac9ec, 0xbaf9, 0x11ce, 0x8c, 0x82, 0x00, 0xaa, 0x00, 0x4b, 0xa9, 0x0b)]
interface IInternetProtocolInfo(IInternetProtocolInfoVtbl): IUnknown(IUnknownVtbl) {
    fn ParseUrl(
        pwzUrl: LPCWSTR,
        ParseAction: PARSEACTION,
        dwParseFlags: DWORD,
        pwzResult: LPWSTR,
        cchResult: DWORD,
        pcchResult: *mut DWORD,
        dwReserved: DWORD,
    ) -> HRESULT,
    fn CombineUrl(
        pwzBaseUrl: LPCWSTR,
        pwzRelativeUrl: LPCWSTR,
        dwCombineFlags: DWORD,
        pwzResult: LPWSTR,
        cchResult: DWORD,
        pcchResult: *mut DWORD,
        dwReserved: DWORD,
    ) -> HRESULT,
    fn CompareUrl(
        pwzUrl1: LPCWSTR,
        pwzUrl2: LPCWSTR,
        dwCompareFlags: DWORD,
    ) -> HRESULT,
    fn QueryInfo(
        pwzUrl: LPCWSTR,
        OueryOption: QUERYOPTION,
        dwQueryFlags: DWORD,
        pBuffer: LPVOID,
        cbBuffer: DWORD,
        pcbBuf: *mut DWORD,
        dwReserved: DWORD,
    ) -> HRESULT,
});
pub type IOInet = IInternet;
pub type IOInetBindInfo = IInternetBindInfo;
pub type IOInetBindInfoEx = IInternetBindInfoEx;
pub type IOInetProtocolRoot = IInternetProtocolRoot;
pub type IOInetProtocol = IInternetProtocol;
pub type IOInetProtocolEx = IInternetProtocolEx;
pub type IOInetProtocolSink = IInternetProtocolSink;
pub type IOInetProtocolInfo = IInternetProtocolInfo;
pub type IOInetSession = IInternetSession;
pub type IOInetPriority = IInternetPriority;
pub type IOInetThreadSwitch = IInternetThreadSwitch;
pub type IOInetProtocolSinkStackable = IInternetProtocolSinkStackable;
pub type LPOINET = LPIINTERNET;
pub type LPOINETPROTOCOLINFO = LPIINTERNETPROTOCOLINFO;
pub type LPOINETBINDINFO = LPIINTERNETBINDINFO;
pub type LPOINETPROTOCOLROOT = LPIINTERNETPROTOCOLROOT;
pub type LPOINETPROTOCOL = LPIINTERNETPROTOCOL;
pub type LPOINETPROTOCOLSINK = LPIINTERNETPROTOCOLSINK;
pub type LPOINETSESSION = LPIINTERNETSESSION;
pub type LPOINETTHREADSWITCH = LPIINTERNETTHREADSWITCH;
pub type LPOINETPRIORITY = LPIINTERNETPRIORITY;
extern "system" {
    pub fn CoInternetParseUrl(
        pwzUrl: LPCWSTR,
        ParseAction: PARSEACTION,
        dwFlags: DWORD,
        pszResult: LPWSTR,
        cchResult: DWORD,
        pcchResult: *mut DWORD,
        dwReserved: DWORD,
    ) -> HRESULT;
    pub fn CoInternetParseIUri(
        pIUri: *mut IUri,
        ParseAction: PARSEACTION,
        dwFlags: DWORD,
        pszResult: LPWSTR,
        cchResult: DWORD,
        pcchResult: *mut DWORD,
        dwReserved: DWORD,
    ) -> HRESULT;
    pub fn CoInternetCombineUrl(
        pwzBaseUrl: LPCWSTR,
        pwzRelativeUrl: LPCWSTR,
        dwCombineFlags: DWORD,
        pszResult: LPWSTR,
        cchResult: DWORD,
        pcchResult: *mut DWORD,
        dwReserved: DWORD,
    ) -> HRESULT;
    pub fn CoInternetCombineUrlEx(
        pBaseUri: *mut IUri,
        pwzRelativeUrl: LPCWSTR,
        dwCombineFlags: DWORD,
        ppCombinedUri: *mut *mut IUri,
        dwReserved: DWORD_PTR,
    ) -> HRESULT;
    pub fn CoInternetCombineIUri(
        pBaseUri: *mut IUri,
        pRelativeUri: *mut IUri,
        dwCombineFlags: DWORD,
        ppCombinedUri: *mut *mut IUri,
        dwReserved: DWORD_PTR,
    ) -> HRESULT;
    pub fn CoInternetCompareUrl(
        pwzUrl1: LPCWSTR,
        pwzUrl2: LPCWSTR,
        dwFlags: DWORD,
    ) -> HRESULT;
    pub fn CoInternetGetProtocolFlags(
        pwzUrl1: LPCWSTR,
        pdwFlags: *mut DWORD,
        dwReserved: DWORD,
    ) -> HRESULT;
    pub fn CoInternetQueryInfo(
        pwzUrl: LPCWSTR,
        QueryOptions: QUERYOPTION,
        dwQueryFlags: DWORD,
        pvBuffer: LPVOID,
        cbBuffer: DWORD,
        pcbBuffer: DWORD,
        dwReserved: DWORD,
    ) -> HRESULT;
    pub fn CoInternetGetSession(
        dwSessionMode: DWORD,
        ppIInternetSession: *mut *mut IInternetSession,
        dwReserved: DWORD,
    ) -> HRESULT;
    pub fn CoInternetGetSecurityUrl(
        pwszUrl: LPCWSTR,
        ppwszSecUrl: *mut LPWSTR,
        psuAction: PSUACTION,
        dwReserved: DWORD,
    ) -> HRESULT;
    pub fn AsyncInstallDistributionUnit(
        szDistUnit: LPCWSTR,
        szTYPE: LPCWSTR,
        szExt: LPCWSTR,
        dwFileVersionMS: DWORD,
        dwFileVersionLS: DWORD,
        szURL: LPCWSTR,
        pbc: *mut IBindCtx,
        pvReserved: LPVOID,
        flags: DWORD,
    ) -> HRESULT;
    pub fn CoInternetGetSecurityUrlEx(
        pUri: *mut IUri,
        ppSecUri: *mut *mut IUri,
        psuAction: PSUACTION,
        dwReserved: DWORD_PTR,
    ) -> HRESULT;
}
ENUM!{enum INTERNETFEATURELIST {
    FEATURE_OBJECT_CACHING = 0,
    FEATURE_ZONE_ELEVATION = FEATURE_OBJECT_CACHING + 1,
    FEATURE_MIME_HANDLING = FEATURE_ZONE_ELEVATION + 1,
    FEATURE_MIME_SNIFFING = FEATURE_MIME_HANDLING + 1,
    FEATURE_WINDOW_RESTRICTIONS = FEATURE_MIME_SNIFFING + 1,
    FEATURE_WEBOC_POPUPMANAGEMENT = FEATURE_WINDOW_RESTRICTIONS + 1,
    FEATURE_BEHAVIORS = FEATURE_WEBOC_POPUPMANAGEMENT + 1,
    FEATURE_DISABLE_MK_PROTOCOL = FEATURE_BEHAVIORS + 1,
    FEATURE_LOCALMACHINE_LOCKDOWN = FEATURE_DISABLE_MK_PROTOCOL + 1,
    FEATURE_SECURITYBAND = FEATURE_LOCALMACHINE_LOCKDOWN + 1,
    FEATURE_RESTRICT_ACTIVEXINSTALL = FEATURE_SECURITYBAND + 1,
    FEATURE_VALIDATE_NAVIGATE_URL = FEATURE_RESTRICT_ACTIVEXINSTALL + 1,
    FEATURE_RESTRICT_FILEDOWNLOAD = FEATURE_VALIDATE_NAVIGATE_URL + 1,
    FEATURE_ADDON_MANAGEMENT = FEATURE_RESTRICT_FILEDOWNLOAD + 1,
    FEATURE_PROTOCOL_LOCKDOWN = FEATURE_ADDON_MANAGEMENT + 1,
    FEATURE_HTTP_USERNAME_PASSWORD_DISABLE = FEATURE_PROTOCOL_LOCKDOWN + 1,
    FEATURE_SAFE_BINDTOOBJECT = FEATURE_HTTP_USERNAME_PASSWORD_DISABLE + 1,
    FEATURE_UNC_SAVEDFILECHECK = FEATURE_SAFE_BINDTOOBJECT + 1,
    FEATURE_GET_URL_DOM_FILEPATH_UNENCODED = FEATURE_UNC_SAVEDFILECHECK + 1,
    FEATURE_TABBED_BROWSING = FEATURE_GET_URL_DOM_FILEPATH_UNENCODED + 1,
    FEATURE_SSLUX = FEATURE_TABBED_BROWSING + 1,
    FEATURE_DISABLE_NAVIGATION_SOUNDS = FEATURE_SSLUX + 1,
    FEATURE_DISABLE_LEGACY_COMPRESSION = FEATURE_DISABLE_NAVIGATION_SOUNDS + 1,
    FEATURE_FORCE_ADDR_AND_STATUS = FEATURE_DISABLE_LEGACY_COMPRESSION + 1,
    FEATURE_XMLHTTP = FEATURE_FORCE_ADDR_AND_STATUS + 1,
    FEATURE_DISABLE_TELNET_PROTOCOL = FEATURE_XMLHTTP + 1,
    FEATURE_FEEDS = FEATURE_DISABLE_TELNET_PROTOCOL + 1,
    FEATURE_BLOCK_INPUT_PROMPTS = FEATURE_FEEDS + 1,
    FEATURE_ENTRY_COUNT = FEATURE_BLOCK_INPUT_PROMPTS + 1,
}}
pub const SET_FEATURE_ON_THREAD: DWORD = 0x00000001;
pub const SET_FEATURE_ON_PROCESS: DWORD = 0x00000002;
pub const SET_FEATURE_IN_REGISTRY: DWORD = 0x00000004;
pub const SET_FEATURE_ON_THREAD_LOCALMACHINE: DWORD = 0x00000008;
pub const SET_FEATURE_ON_THREAD_INTRANET: DWORD = 0x00000010;
pub const SET_FEATURE_ON_THREAD_TRUSTED: DWORD = 0x00000020;
pub const SET_FEATURE_ON_THREAD_INTERNET: DWORD = 0x00000040;
pub const SET_FEATURE_ON_THREAD_RESTRICTED: DWORD = 0x00000080;
pub const GET_FEATURE_FROM_THREAD: DWORD = 0x00000001;
pub const GET_FEATURE_FROM_PROCESS: DWORD = 0x00000002;
pub const GET_FEATURE_FROM_REGISTRY: DWORD = 0x00000004;
pub const GET_FEATURE_FROM_THREAD_LOCALMACHINE: DWORD = 0x00000008;
pub const GET_FEATURE_FROM_THREAD_INTRANET: DWORD = 0x00000010;
pub const GET_FEATURE_FROM_THREAD_TRUSTED: DWORD = 0x00000020;
pub const GET_FEATURE_FROM_THREAD_INTERNET: DWORD = 0x00000040;
pub const GET_FEATURE_FROM_THREAD_RESTRICTED: DWORD = 0x00000080;
extern "system" {
    pub fn CoInternetSetFeatureEnabled(
        FeatureEntry: INTERNETFEATURELIST,
        dwFlags: DWORD,
        fEnable: BOOL,
    ) -> HRESULT;
    pub fn CoInternetIsFeatureEnabled(
        FeatureEntry: INTERNETFEATURELIST,
        dwFlags: DWORD,
    ) -> HRESULT;
    pub fn CoInternetIsFeatureEnabledForUrl(
        FeatureEntry: INTERNETFEATURELIST,
        dwFlags: DWORD,
        szURL: LPCWSTR,
        pSecMgr: *mut IInternetSecurityManager,
    ) -> HRESULT;
    pub fn CoInternetIsFeatureEnabledForIUri(
        FeatureEntry: INTERNETFEATURELIST,
        dwFlags: DWORD,
        pIUri: *mut IUri,
        pSecMgr: *mut IInternetSecurityManagerEx2,
    ) -> HRESULT;
    pub fn CoInternetIsFeatureZoneElevationEnabled(
        szFromURL: LPCWSTR,
        szToURL: LPCWSTR,
        pSecMgr: *mut IInternetSecurityManager,
        dwFlags: DWORD,
    ) -> HRESULT;
    pub fn CopyStgMedium(
        pcstgmedSrc: *const STGMEDIUM,
        pstgmedDest: *mut STGMEDIUM,
    ) -> HRESULT;
    pub fn CopyBindInfo(
        pcbiSrc: *const BINDINFO,
        pbiDest: *mut BINDINFO,
    ) -> HRESULT;
    pub fn ReleaseBindInfo(
        pbindinfo: *mut BINDINFO,
    ) -> HRESULT;
}
pub const INET_E_USE_DEFAULT_PROTOCOLHANDLER: HRESULT = 0x800C0011;
pub const INET_E_USE_DEFAULT_SETTING: HRESULT = 0x800C0012;
pub const INET_E_DEFAULT_ACTION: HRESULT = INET_E_USE_DEFAULT_PROTOCOLHANDLER;
pub const INET_E_QUERYOPTION_UNKNOWN: HRESULT = 0x800C0013;
pub const INET_E_REDIRECTING: HRESULT = 0x800C0014;
pub const PROTOCOLFLAG_NO_PICS_CHECK: DWORD = 0x00000001;
extern "system" {
    pub fn IEGetUserPrivateNamespaceName() -> PWSTR;
    pub fn CoInternetCreateSecurityManager(
        pSP: *mut IServiceProvider,
        ppSM: *mut *mut IInternetSecurityManager,
        dwReserved: DWORD,
    ) -> HRESULT;
    pub fn CoInternetCreateZoneManager(
        pSP: *mut IServiceProvider,
        ppZM: *mut *mut IInternetZoneManager,
        dwReserved: DWORD,
    ) -> HRESULT;
}
RIDL!(#[uuid(0x79eac9ed, 0xbaf9, 0x11ce, 0x8c, 0x82, 0x00, 0xaa, 0x00, 0x4b, 0xa9, 0x0b)]
interface IInternetSecurityMgrSite(IInternetSecurityMgrSiteVtbl): IUnknown(IUnknownVtbl) {
    fn GetWindow(
        phwnd: *mut HWND,
    ) -> HRESULT,
    fn EnableModeless(
        fEnable: BOOL,
    ) -> HRESULT,
});
pub const MUTZ_NOSAVEDFILECHECK: DWORD = 0x00000001;
pub const MUTZ_ISFILE: DWORD = 0x00000002;
pub const MUTZ_ACCEPT_WILDCARD_SCHEME: DWORD = 0x00000080;
pub const MUTZ_ENFORCERESTRICTED: DWORD = 0x00000100;
pub const MUTZ_RESERVED: DWORD = 0x00000200;
pub const MUTZ_REQUIRESAVEDFILECHECK: DWORD = 0x00000400;
pub const MUTZ_DONT_UNESCAPE: DWORD = 0x00000800;
pub const MUTZ_DONT_USE_CACHE: DWORD = 0x00001000;
pub const MUTZ_FORCE_INTRANET_FLAGS: DWORD = 0x00002000;
pub const MUTZ_IGNORE_ZONE_MAPPINGS: DWORD = 0x00004000;
pub const MAX_SIZE_SECURITY_ID: usize = 512;
ENUM!{enum PUAF {
    PUAF_DEFAULT = 0,
    PUAF_NOUI = 0x1,
    PUAF_ISFILE = 0x2,
    PUAF_WARN_IF_DENIED = 0x4,
    PUAF_FORCEUI_FOREGROUND = 0x8,
    PUAF_CHECK_TIFS = 0x10,
    PUAF_DONTCHECKBOXINDIALOG = 0x20,
    PUAF_TRUSTED = 0x40,
    PUAF_ACCEPT_WILDCARD_SCHEME = 0x80,
    PUAF_ENFORCERESTRICTED = 0x100,
    PUAF_NOSAVEDFILECHECK = 0x200,
    PUAF_REQUIRESAVEDFILECHECK = 0x400,
    PUAF_DONT_USE_CACHE = 0x1000,
    PUAF_RESERVED1 = 0x2000,
    PUAF_RESERVED2 = 0x4000,
    PUAF_LMZ_UNLOCKED = 0x10000,
    PUAF_LMZ_LOCKED = 0x20000,
    PUAF_DEFAULTZONEPOL = 0x40000,
    PUAF_NPL_USE_LOCKED_IF_RESTRICTED = 0x80000,
    PUAF_NOUIIFLOCKED = 0x100000,
    PUAF_DRAGPROTOCOLCHECK = 0x200000,
}}
ENUM!{enum PUAFOUT {
    PUAFOUT_DEFAULT = 0,
    PUAFOUT_ISLOCKZONEPOLICY = 0x1,
}}
ENUM!{enum SZM_FLAGS {
    SZM_CREATE = 0,
    SZM_DELETE = 0x1,
}}
RIDL!(#[uuid(0x79eac9ee, 0xbaf9, 0x11ce, 0x8c, 0x82, 0x00, 0xaa, 0x00, 0x4b, 0xa9, 0x0b)]
interface IInternetSecurityManager(IInternetSecurityManagerVtbl): IUnknown(IUnknownVtbl) {
    fn SetSecuritySite(
        pSite: *mut IInternetSecurityMgrSite,
    ) -> HRESULT,
    fn GetSecuritySite(
        ppSite: *mut *mut IInternetSecurityMgrSite,
    ) -> HRESULT,
    fn MapUrlToZone(
        pwszUrl: LPCWSTR,
        pdwZone: *mut DWORD,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn GetSecurityId(
        pwszUrl: LPCWSTR,
        pbSecurityId: *mut BYTE,
        pcbSecurityId: *mut DWORD,
        dwReserved: DWORD_PTR,
    ) -> HRESULT,
    fn ProcessUrlAction(
        pwszUrl: LPCWSTR,
        dwAction: DWORD,
        pPolicy: *mut BYTE,
        cbPolicy: DWORD,
        pContext: *mut BYTE,
        cbContext: DWORD,
        dwFlags: DWORD,
        dwReserved: DWORD,
    ) -> HRESULT,
    fn QueryCustomPolicy(
        pwszUrl: LPCWSTR,
        guidKey: REFGUID,
        ppPolicy: *mut *mut BYTE,
        pcbPolicy: *mut DWORD,
        pContext: *mut BYTE,
        cbContext: DWORD,
        dwReserved: DWORD,
    ) -> HRESULT,
    fn SetZoneMapping(
        dwZone: DWORD,
        lpszPattern: LPCWSTR,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn GetZoneMappings(
        dwZone: DWORD,
        ppenumString: *mut *mut IEnumString,
        dwFlags: DWORD,
    ) -> HRESULT,
});
RIDL!(#[uuid(0xf164edf1, 0xcc7c, 0x4f0d, 0x9a, 0x94, 0x34, 0x22, 0x26, 0x25, 0xc3, 0x93)]
interface IInternetSecurityManagerEx(IInternetSecurityManagerExVtbl):
    IInternetSecurityManager(IInternetSecurityManagerVtbl) {
    fn ProcessUrlActionEx(
        pwszUrl: LPCWSTR,
        dwAction: DWORD,
        pPolicy: *mut BYTE,
        cbPolicy: DWORD,
        pContext: *mut BYTE,
        cbContext: DWORD,
        dwFlags: DWORD,
        dwReserved: DWORD,
        pdwOutFlags: *mut DWORD,
    ) -> HRESULT,
});
RIDL!(#[uuid(0xf1e50292, 0xa795, 0x4117, 0x8e, 0x09, 0x2b, 0x56, 0x0a, 0x72, 0xac, 0x60)]
interface IInternetSecurityManagerEx2(IInternetSecurityManagerEx2Vtbl):
    IInternetSecurityManagerEx(IInternetSecurityManagerExVtbl) {
    fn MapUrlToZoneEx2(
        pUri: *mut IUri,
        pdwZone: *mut DWORD,
        dwFlags: DWORD,
        ppwszMappedUrl: *mut LPWSTR,
        pdwOutFlags: *mut DWORD,
    ) -> HRESULT,
    fn ProcessUrlActionEx2(
        pUri: *mut IUri,
        dwAction: DWORD,
        pPolicy: *mut BYTE,
        cbPolicy: DWORD,
        pContext: *mut BYTE,
        cbContext: DWORD,
        dwFlags: DWORD,
        dwReserved: DWORD_PTR,
        pdwOutFlags: *mut DWORD,
    ) -> HRESULT,
    fn GetSecurityIdEx2(
        pUri: *mut IUri,
        pbSecurityId: *mut BYTE,
        pcbSecurityId: *mut DWORD,
        dwReserved: DWORD_PTR,
    ) -> HRESULT,
    fn QueryCustomPolicyEx2(
        pUri: *mut IUri,
        guidKey: REFGUID,
        ppPolicy: *mut *mut BYTE,
        pcbPolicy: *mut DWORD,
        pContext: *mut BYTE,
        cbContext: DWORD,
        dwReserved: DWORD_PTR,
    ) -> HRESULT,
});
RIDL!(#[uuid(0xcd45f185, 0x1b21, 0x48e2, 0x96, 0x7b, 0xea, 0xd7, 0x43, 0xa8, 0x91, 0x4e)]
interface IZoneIdentifier(IZoneIdentifierVtbl): IUnknown(IUnknownVtbl) {
    fn GetId(
        pdwZone: *mut DWORD,
    ) -> HRESULT,
    fn SetId(
        dwZone: DWORD,
    ) -> HRESULT,
    fn Remove() -> HRESULT,
});
RIDL!(#[uuid(0xeb5e760c, 0x09ef, 0x45c0, 0xb5, 0x10, 0x70, 0x83, 0x0c, 0xe3, 0x1e, 0x6a)]
interface IZoneIdentifier2(IZoneIdentifier2Vtbl): IZoneIdentifier(IZoneIdentifierVtbl) {
    fn GetLastWriterPackageFamilyName(
        packageFamilyName: *mut LPWSTR,
    ) -> HRESULT,
    fn SetLastWriterPackageFamilyName(
        packageFamilyName: LPCWSTR,
    ) -> HRESULT,
    fn RemoveLastWriterPackageFamilyName() -> HRESULT,
    fn GetAppZoneId(
        zone: *mut DWORD,
    ) -> HRESULT,
    fn SetAppZoneId(
        zone: DWORD,
    ) -> HRESULT,
    fn RemoveAppZoneId() -> HRESULT,
});
RIDL!(#[uuid(0x3af280b6, 0xcb3f, 0x11d0, 0x89, 0x1e, 0x00, 0xc0, 0x4f, 0xb6, 0xbf, 0xc4)]
interface IInternetHostSecurityManager(IInternetHostSecurityManagerVtbl): IUnknown(IUnknownVtbl) {
    fn GetSecurityId(
        pbSecurityId: *mut BYTE,
        pcbSecurityId: *mut DWORD,
        dwReserved: DWORD_PTR,
    ) -> HRESULT,
    fn ProcessUrlAction(
        dwAction: DWORD,
        pPolicy: *mut BYTE,
        cbPolicy: DWORD,
        pContext: *mut BYTE,
        cbContext: DWORD,
        dwFlags: DWORD,
        dwReserved: DWORD,
    ) -> HRESULT,
    fn QueryCustomPolicy(
        guidKey: REFGUID,
        ppPolicy: *mut *mut BYTE,
        pcbPolicy: *mut DWORD,
        pContext: *mut BYTE,
        cbContext: DWORD,
        dwReserved: DWORD,
    ) -> HRESULT,
});
pub const URLACTION_MIN: DWORD = 0x00001000;
pub const URLACTION_DOWNLOAD_MIN: DWORD = 0x00001000;
pub const URLACTION_DOWNLOAD_SIGNED_ACTIVEX: DWORD = 0x00001001;
pub const URLACTION_DOWNLOAD_UNSIGNED_ACTIVEX: DWORD = 0x00001004;
pub const URLACTION_DOWNLOAD_CURR_MAX: DWORD = 0x00001004;
pub const URLACTION_DOWNLOAD_MAX: DWORD = 0x000011FF;
pub const URLACTION_ACTIVEX_MIN: DWORD = 0x00001200;
pub const URLACTION_ACTIVEX_RUN: DWORD = 0x00001200;
pub const URLPOLICY_ACTIVEX_CHECK_LIST: DWORD = 0x00010000;
pub const URLACTION_ACTIVEX_OVERRIDE_OBJECT_SAFETY: DWORD = 0x00001201;
pub const URLACTION_ACTIVEX_OVERRIDE_DATA_SAFETY: DWORD = 0x00001202;
pub const URLACTION_ACTIVEX_OVERRIDE_SCRIPT_SAFETY: DWORD = 0x00001203;
pub const URLACTION_SCRIPT_OVERRIDE_SAFETY: DWORD = 0x00001401;
pub const URLACTION_ACTIVEX_CONFIRM_NOOBJECTSAFETY: DWORD = 0x00001204;
pub const URLACTION_ACTIVEX_TREATASUNTRUSTED: DWORD = 0x00001205;
pub const URLACTION_ACTIVEX_NO_WEBOC_SCRIPT: DWORD = 0x00001206;
pub const URLACTION_ACTIVEX_OVERRIDE_REPURPOSEDETECTION: DWORD = 0x00001207;
pub const URLACTION_ACTIVEX_OVERRIDE_OPTIN: DWORD = 0x00001208;
pub const URLACTION_ACTIVEX_SCRIPTLET_RUN: DWORD = 0x00001209;
pub const URLACTION_ACTIVEX_DYNSRC_VIDEO_AND_ANIMATION: DWORD = 0x0000120A;
pub const URLACTION_ACTIVEX_OVERRIDE_DOMAINLIST: DWORD = 0x0000120B;
pub const URLACTION_ACTIVEX_ALLOW_TDC: DWORD = 0x0000120C;
pub const URLACTION_ACTIVEX_CURR_MAX: DWORD = 0x0000120C;
pub const URLACTION_ACTIVEX_MAX: DWORD = 0x000013ff;
pub const URLACTION_SCRIPT_MIN: DWORD = 0x00001400;
pub const URLACTION_SCRIPT_RUN: DWORD = 0x00001400;
pub const URLACTION_SCRIPT_JAVA_USE: DWORD = 0x00001402;
pub const URLACTION_SCRIPT_SAFE_ACTIVEX: DWORD = 0x00001405;
pub const URLACTION_CROSS_DOMAIN_DATA: DWORD = 0x00001406;
pub const URLACTION_SCRIPT_PASTE: DWORD = 0x00001407;
pub const URLACTION_ALLOW_XDOMAIN_SUBFRAME_RESIZE: DWORD = 0x00001408;
pub const URLACTION_SCRIPT_XSSFILTER: DWORD = 0x00001409;
pub const URLACTION_SCRIPT_NAVIGATE: DWORD = 0x0000140A;
pub const URLACTION_PLUGGABLE_PROTOCOL_XHR: DWORD = 0x0000140B;
pub const URLACTION_ALLOW_VBSCRIPT_IE: DWORD = 0x0000140C;
pub const URLACTION_SCRIPT_CURR_MAX: DWORD = 0x0000140C;
pub const URLACTION_SCRIPT_MAX: DWORD = 0x000015ff;
pub const URLACTION_HTML_MIN: DWORD = 0x00001600;
pub const URLACTION_HTML_SUBMIT_FORMS: DWORD = 0x00001601;
pub const URLACTION_HTML_SUBMIT_FORMS_FROM: DWORD = 0x00001602;
pub const URLACTION_HTML_SUBMIT_FORMS_TO: DWORD = 0x00001603;
pub const URLACTION_HTML_FONT_DOWNLOAD: DWORD = 0x00001604;
pub const URLACTION_HTML_JAVA_RUN: DWORD = 0x00001605;
pub const URLACTION_HTML_USERDATA_SAVE: DWORD = 0x00001606;
pub const URLACTION_HTML_SUBFRAME_NAVIGATE: DWORD = 0x00001607;
pub const URLACTION_HTML_META_REFRESH: DWORD = 0x00001608;
pub const URLACTION_HTML_MIXED_CONTENT: DWORD = 0x00001609;
pub const URLACTION_HTML_INCLUDE_FILE_PATH: DWORD = 0x0000160A;
pub const URLACTION_HTML_ALLOW_INJECTED_DYNAMIC_HTML: DWORD = 0x0000160B;
pub const URLACTION_HTML_REQUIRE_UTF8_DOCUMENT_CODEPAGE: DWORD = 0x0000160C;
pub const URLACTION_HTML_ALLOW_CROSS_DOMAIN_CANVAS: DWORD = 0x0000160D;
pub const URLACTION_HTML_ALLOW_WINDOW_CLOSE: DWORD = 0x0000160E;
pub const URLACTION_HTML_ALLOW_CROSS_DOMAIN_WEBWORKER: DWORD = 0x0000160F;
pub const URLACTION_HTML_ALLOW_CROSS_DOMAIN_TEXTTRACK: DWORD = 0x00001610;
pub const URLACTION_HTML_ALLOW_INDEXEDDB: DWORD = 0x00001611;
pub const URLACTION_HTML_MAX: DWORD = 0x000017ff;
pub const URLACTION_SHELL_MIN: DWORD = 0x00001800;
pub const URLACTION_SHELL_INSTALL_DTITEMS: DWORD = 0x00001800;
pub const URLACTION_SHELL_MOVE_OR_COPY: DWORD = 0x00001802;
pub const URLACTION_SHELL_FILE_DOWNLOAD: DWORD = 0x00001803;
pub const URLACTION_SHELL_VERB: DWORD = 0x00001804;
pub const URLACTION_SHELL_WEBVIEW_VERB: DWORD = 0x00001805;
pub const URLACTION_SHELL_SHELLEXECUTE: DWORD = 0x00001806;
pub const URLACTION_SHELL_EXECUTE_HIGHRISK: DWORD = 0x00001806;
pub const URLACTION_SHELL_EXECUTE_MODRISK: DWORD = 0x00001807;
pub const URLACTION_SHELL_EXECUTE_LOWRISK: DWORD = 0x00001808;
pub const URLACTION_SHELL_POPUPMGR: DWORD = 0x00001809;
pub const URLACTION_SHELL_RTF_OBJECTS_LOAD: DWORD = 0x0000180A;
pub const URLACTION_SHELL_ENHANCED_DRAGDROP_SECURITY: DWORD = 0x0000180B;
pub const URLACTION_SHELL_EXTENSIONSECURITY: DWORD = 0x0000180C;
pub const URLACTION_SHELL_SECURE_DRAGSOURCE: DWORD = 0x0000180D;
pub const URLACTION_SHELL_REMOTEQUERY: DWORD = 0x0000180E;
pub const URLACTION_SHELL_PREVIEW: DWORD = 0x0000180F;
pub const URLACTION_SHELL_SHARE: DWORD = 0x00001810;
pub const URLACTION_SHELL_ALLOW_CROSS_SITE_SHARE: DWORD = 0x00001811;
pub const URLACTION_SHELL_TOCTOU_RISK: DWORD = 0x00001812;
pub const URLACTION_SHELL_CURR_MAX: DWORD = 0x00001812;
pub const URLACTION_SHELL_MAX: DWORD = 0x000019ff;
pub const URLACTION_NETWORK_MIN: DWORD = 0x00001A00;
pub const URLACTION_CREDENTIALS_USE: DWORD = 0x00001A00;
pub const URLPOLICY_CREDENTIALS_SILENT_LOGON_OK: DWORD = 0x00000000;
pub const URLPOLICY_CREDENTIALS_MUST_PROMPT_USER: DWORD = 0x00010000;
pub const URLPOLICY_CREDENTIALS_CONDITIONAL_PROMPT: DWORD = 0x00020000;
pub const URLPOLICY_CREDENTIALS_ANONYMOUS_ONLY: DWORD = 0x00030000;
pub const URLACTION_AUTHENTICATE_CLIENT: DWORD = 0x00001A01;
pub const URLPOLICY_AUTHENTICATE_CLEARTEXT_OK: DWORD = 0x00000000;
pub const URLPOLICY_AUTHENTICATE_CHALLENGE_RESPONSE: DWORD = 0x00010000;
pub const URLPOLICY_AUTHENTICATE_MUTUAL_ONLY: DWORD = 0x00030000;
pub const URLACTION_COOKIES: DWORD = 0x00001A02;
pub const URLACTION_COOKIES_SESSION: DWORD = 0x00001A03;
pub const URLACTION_CLIENT_CERT_PROMPT: DWORD = 0x00001A04;
pub const URLACTION_COOKIES_THIRD_PARTY: DWORD = 0x00001A05;
pub const URLACTION_COOKIES_SESSION_THIRD_PARTY: DWORD = 0x00001A06;
pub const URLACTION_COOKIES_ENABLED: DWORD = 0x00001A10;
pub const URLACTION_NETWORK_CURR_MAX: DWORD = 0x00001A10;
pub const URLACTION_NETWORK_MAX: DWORD = 0x00001Bff;
pub const URLACTION_JAVA_MIN: DWORD = 0x00001C00;
pub const URLACTION_JAVA_PERMISSIONS: DWORD = 0x00001C00;
pub const URLPOLICY_JAVA_PROHIBIT: DWORD = 0x00000000;
pub const URLPOLICY_JAVA_HIGH: DWORD = 0x00010000;
pub const URLPOLICY_JAVA_MEDIUM: DWORD = 0x00020000;
pub const URLPOLICY_JAVA_LOW: DWORD = 0x00030000;
pub const URLPOLICY_JAVA_CUSTOM: DWORD = 0x00800000;
pub const URLACTION_JAVA_CURR_MAX: DWORD = 0x00001C00;
pub const URLACTION_JAVA_MAX: DWORD = 0x00001Cff;
pub const URLACTION_INFODELIVERY_MIN: DWORD = 0x00001D00;
pub const URLACTION_INFODELIVERY_NO_ADDING_CHANNELS: DWORD = 0x00001D00;
pub const URLACTION_INFODELIVERY_NO_EDITING_CHANNELS: DWORD = 0x00001D01;
pub const URLACTION_INFODELIVERY_NO_REMOVING_CHANNELS: DWORD = 0x00001D02;
pub const URLACTION_INFODELIVERY_NO_ADDING_SUBSCRIPTIONS: DWORD = 0x00001D03;
pub const URLACTION_INFODELIVERY_NO_EDITING_SUBSCRIPTIONS: DWORD = 0x00001D04;
pub const URLACTION_INFODELIVERY_NO_REMOVING_SUBSCRIPTIONS: DWORD = 0x00001D05;
pub const URLACTION_INFODELIVERY_NO_CHANNEL_LOGGING: DWORD = 0x00001D06;
pub const URLACTION_INFODELIVERY_CURR_MAX: DWORD = 0x00001D06;
pub const URLACTION_INFODELIVERY_MAX: DWORD = 0x00001Dff;
pub const URLACTION_CHANNEL_SOFTDIST_MIN: DWORD = 0x00001E00;
pub const URLACTION_CHANNEL_SOFTDIST_PERMISSIONS: DWORD = 0x00001E05;
pub const URLPOLICY_CHANNEL_SOFTDIST_PROHIBIT: DWORD = 0x00010000;
pub const URLPOLICY_CHANNEL_SOFTDIST_PRECACHE: DWORD = 0x00020000;
pub const URLPOLICY_CHANNEL_SOFTDIST_AUTOINSTALL: DWORD = 0x00030000;
pub const URLACTION_CHANNEL_SOFTDIST_MAX: DWORD = 0x00001Eff;
pub const URLACTION_DOTNET_USERCONTROLS: DWORD = 0x00002005;
pub const URLACTION_BEHAVIOR_MIN: DWORD = 0x00002000;
pub const URLACTION_BEHAVIOR_RUN: DWORD = 0x00002000;
pub const URLPOLICY_BEHAVIOR_CHECK_LIST: DWORD = 0x00010000;
pub const URLACTION_FEATURE_MIN: DWORD = 0x00002100;
pub const URLACTION_FEATURE_MIME_SNIFFING: DWORD = 0x00002100;
pub const URLACTION_FEATURE_ZONE_ELEVATION: DWORD = 0x00002101;
pub const URLACTION_FEATURE_WINDOW_RESTRICTIONS: DWORD = 0x00002102;
pub const URLACTION_FEATURE_SCRIPT_STATUS_BAR: DWORD = 0x00002103;
pub const URLACTION_FEATURE_FORCE_ADDR_AND_STATUS: DWORD = 0x00002104;
pub const URLACTION_FEATURE_BLOCK_INPUT_PROMPTS: DWORD = 0x00002105;
pub const URLACTION_FEATURE_DATA_BINDING: DWORD = 0x00002106;
pub const URLACTION_FEATURE_CROSSDOMAIN_FOCUS_CHANGE: DWORD = 0x00002107;
pub const URLACTION_AUTOMATIC_DOWNLOAD_UI_MIN: DWORD = 0x00002200;
pub const URLACTION_AUTOMATIC_DOWNLOAD_UI: DWORD = 0x00002200;
pub const URLACTION_AUTOMATIC_ACTIVEX_UI: DWORD = 0x00002201;
pub const URLACTION_ALLOW_RESTRICTEDPROTOCOLS: DWORD = 0x00002300;
pub const URLACTION_ALLOW_APEVALUATION: DWORD = 0x00002301;
pub const URLACTION_ALLOW_XHR_EVALUATION: DWORD = 0x00002302;
pub const URLACTION_WINDOWS_BROWSER_APPLICATIONS: DWORD = 0x00002400;
pub const URLACTION_XPS_DOCUMENTS: DWORD = 0x00002401;
pub const URLACTION_LOOSE_XAML: DWORD = 0x00002402;
pub const URLACTION_LOWRIGHTS: DWORD = 0x00002500;
pub const URLACTION_WINFX_SETUP: DWORD = 0x00002600;
pub const URLACTION_INPRIVATE_BLOCKING: DWORD = 0x00002700;
pub const URLACTION_ALLOW_AUDIO_VIDEO: DWORD = 0x00002701;
pub const URLACTION_ALLOW_ACTIVEX_FILTERING: DWORD = 0x00002702;
pub const URLACTION_ALLOW_STRUCTURED_STORAGE_SNIFFING: DWORD = 0x00002703;
pub const URLACTION_ALLOW_AUDIO_VIDEO_PLUGINS: DWORD = 0x00002704;
pub const URLACTION_ALLOW_ZONE_ELEVATION_VIA_OPT_OUT: DWORD = 0x00002705;
pub const URLACTION_ALLOW_ZONE_ELEVATION_OPT_OUT_ADDITION: DWORD = 0x00002706;
pub const URLACTION_ALLOW_CROSSDOMAIN_DROP_WITHIN_WINDOW: DWORD = 0x00002708;
pub const URLACTION_ALLOW_CROSSDOMAIN_DROP_ACROSS_WINDOWS: DWORD = 0x00002709;
pub const URLACTION_ALLOW_CROSSDOMAIN_APPCACHE_MANIFEST: DWORD = 0x0000270A;
pub const URLACTION_ALLOW_RENDER_LEGACY_DXTFILTERS: DWORD = 0x0000270B;
pub const URLACTION_ALLOW_ANTIMALWARE_SCANNING_OF_ACTIVEX: DWORD = 0x0000270C;
pub const URLACTION_ALLOW_CSS_EXPRESSIONS: DWORD = 0x0000270D;
pub const URLPOLICY_ALLOW: BYTE = 0x00;
pub const URLPOLICY_QUERY: BYTE = 0x01;
pub const URLPOLICY_DISALLOW: BYTE = 0x03;
pub const URLPOLICY_NOTIFY_ON_ALLOW: BYTE = 0x10;
pub const URLPOLICY_NOTIFY_ON_DISALLOW: BYTE = 0x20;
pub const URLPOLICY_LOG_ON_ALLOW: BYTE = 0x40;
pub const URLPOLICY_LOG_ON_DISALLOW: BYTE = 0x80;
pub const URLPOLICY_MASK_PERMISSIONS: BYTE = 0x0f;
#[inline]
pub fn GetUrlPolicyPermissions(dw: DWORD) -> BYTE {
    dw as BYTE & URLPOLICY_MASK_PERMISSIONS
}
pub const URLPOLICY_DONTCHECKDLGBOX: DWORD = 0x100;
ENUM!{enum URLZONE {
    URLZONE_INVALID = -1i32 as u32,
    URLZONE_PREDEFINED_MIN = 0,
    URLZONE_LOCAL_MACHINE = 0,
    URLZONE_INTRANET = URLZONE_LOCAL_MACHINE + 1,
    URLZONE_TRUSTED = URLZONE_INTRANET + 1,
    URLZONE_INTERNET = URLZONE_TRUSTED + 1,
    URLZONE_UNTRUSTED = URLZONE_INTERNET + 1,
    URLZONE_PREDEFINED_MAX = 999,
    URLZONE_USER_MIN = 1000,
    URLZONE_USER_MAX = 10000,
}}
pub const URLZONE_ESC_FLAG: DWORD = 0x100;
ENUM!{enum URLTEMPLATE {
    URLTEMPLATE_CUSTOM = 0,
    URLTEMPLATE_PREDEFINED_MIN = 0x10000,
    URLTEMPLATE_LOW = 0x10000,
    URLTEMPLATE_MEDLOW = 0x10500,
    URLTEMPLATE_MEDIUM = 0x11000,
    URLTEMPLATE_MEDHIGH = 0x11500,
    URLTEMPLATE_HIGH = 0x12000,
    URLTEMPLATE_PREDEFINED_MAX = 0x20000,
}}
ENUM!{enum __MIDL_IInternetZoneManager_0001 {
    MAX_ZONE_PATH = 260,
    MAX_ZONE_DESCRIPTION = 200,
}}
ENUM!{enum ZAFLAGS {
    ZAFLAGS_CUSTOM_EDIT = 0x1,
    ZAFLAGS_ADD_SITES = 0x2,
    ZAFLAGS_REQUIRE_VERIFICATION = 0x4,
    ZAFLAGS_INCLUDE_PROXY_OVERRIDE = 0x8,
    ZAFLAGS_INCLUDE_INTRANET_SITES = 0x10,
    ZAFLAGS_NO_UI = 0x20,
    ZAFLAGS_SUPPORTS_VERIFICATION = 0x40,
    ZAFLAGS_UNC_AS_INTRANET = 0x80,
    ZAFLAGS_DETECT_INTRANET = 0x100,
    ZAFLAGS_USE_LOCKED_ZONES = 0x10000,
    ZAFLAGS_VERIFY_TEMPLATE_SETTINGS = 0x20000,
    ZAFLAGS_NO_CACHE = 0x40000,
}}
STRUCT!{struct ZONEATTRIBUTES {
    cbSize: ULONG,
    szDisplayName: [WCHAR; 260],
    szDescription: [WCHAR; 200],
    szIconPath: [WCHAR; 260],
    dwTemplateMinLevel: DWORD,
    dwTemplateRecommended: DWORD,
    dwTemplateCurrentLevel: DWORD,
    dwFlags: DWORD,
}}
pub type LPZONEATTRIBUTES = *mut ZONEATTRIBUTES;
ENUM!{enum URLZONEREG {
    URLZONEREG_DEFAULT = 0,
    URLZONEREG_HKLM = URLZONEREG_DEFAULT + 1,
    URLZONEREG_HKCU = URLZONEREG_HKLM + 1,
}}
RIDL!(#[uuid(0x79eac9ef, 0xbaf9, 0x11ce, 0x8c, 0x82, 0x00, 0xaa, 0x00, 0x4b, 0xa9, 0x0b)]
interface IInternetZoneManager(IInternetZoneManagerVtbl): IUnknown(IUnknownVtbl) {
    fn GetZoneAttributes(
        dwZone: DWORD,
        pZoneAttributes: *mut ZONEATTRIBUTES,
    ) -> HRESULT,
    fn SetZoneAttributes(
        dwZone: DWORD,
        pZoneAttributes: *mut ZONEATTRIBUTES,
    ) -> HRESULT,
    fn GetZoneCustomPolicy(
        dwZone: DWORD,
        guidKey: REFGUID,
        ppPolicy: *mut *mut BYTE,
        pcbPolicy: *mut DWORD,
        urlZoneReg: URLZONEREG,
    ) -> HRESULT,
    fn SetZoneCustomPolicy(
        dwZone: DWORD,
        guidKey: REFGUID,
        pPolicy: *mut BYTE,
        cbPolicy: DWORD,
        urlZoneReg: URLZONEREG,
    ) -> HRESULT,
    fn GetZoneActionPolicy(
        dwZone: DWORD,
        dwAction: DWORD,
        pPolicy: *mut BYTE,
        cbPolicy: DWORD,
        urlZoneReg: URLZONEREG,
    ) -> HRESULT,
    fn SetZoneActionPolicy(
        dwZone: DWORD,
        dwAction: DWORD,
        pPolicy: *mut BYTE,
        cbPolicy: DWORD,
        urlZoneReg: URLZONEREG,
    ) -> HRESULT,
    fn PromptAction(
        dwAction: DWORD,
        hwndParent: HWND,
        pwszUrl: LPCWSTR,
        pwszText: LPCWSTR,
        dwPromptFlags: DWORD,
    ) -> HRESULT,
    fn LogAction(
        dwAction: DWORD,
        pwszUrl: LPCWSTR,
        pwszText: LPCWSTR,
        dwLogFlags: DWORD,
    ) -> HRESULT,
    fn CreateZoneEnumerator(
        pdwEnum: *mut DWORD,
        pdwCount: *mut DWORD,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn GetZoneAt(
        dwEnum: DWORD,
        dwIndex: DWORD,
        pdwZone: *mut DWORD,
    ) -> HRESULT,
    fn DestroyZoneEnumerator(
        dwEnum: DWORD,
    ) -> HRESULT,
    fn CopyTemplatePoliciesToZone(
        dwTemplate: DWORD,
        dwZone: DWORD,
        dwReserved: DWORD,
    ) -> HRESULT,
});
RIDL!(#[uuid(0xa4c23339, 0x8e06, 0x431e, 0x9b, 0xf4, 0x7e, 0x71, 0x1c, 0x08, 0x56, 0x48)]
interface IInternetZoneManagerEx(IInternetZoneManagerExVtbl):
    IInternetZoneManager(IInternetZoneManagerVtbl) {
    fn GetZoneActionPolicyEx(
        dwZone: DWORD,
        dwAction: DWORD,
        pPolicy: *mut BYTE,
        cbPolicy: DWORD,
        urlZoneReg: URLZONEREG,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn SetZoneActionPolicyEx(
        dwZone: DWORD,
        dwAction: DWORD,
        pPolicy: *mut BYTE,
        cbPolicy: DWORD,
        urlZoneReg: URLZONEREG,
        dwFlags: DWORD,
    ) -> HRESULT,
});
pub const SECURITY_IE_STATE_GREEN: DWORD = 0x00000000;
pub const SECURITY_IE_STATE_RED: DWORD = 0x00000001;
RIDL!(#[uuid(0xedc17559, 0xdd5d, 0x4846, 0x8e, 0xef, 0x8b, 0xec, 0xba, 0x5a, 0x4a, 0xbf)]
interface IInternetZoneManagerEx2(IInternetZoneManagerEx2Vtbl):
    IInternetZoneManagerEx(IInternetZoneManagerExVtbl) {
    fn GetZoneAttributesEx(
        dwZone: DWORD,
        pZoneAttributes: *mut ZONEATTRIBUTES,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn GetZoneSecurityState(
        dwZoneIndex: DWORD,
        fRespectPolicy: BOOL,
        pdwState: LPDWORD,
        pfPolicyEncountered: *mut BOOL,
    ) -> HRESULT,
    fn GetIESecurityState(
        fRespectPolicy: BOOL,
        pdwState: LPDWORD,
        pfPolicyEncountered: *mut BOOL,
        fNoCache: BOOL,
    ) -> HRESULT,
    fn FixUnsecureSettings() -> HRESULT,
});
pub const SOFTDIST_FLAG_USAGE_EMAIL: DWORD = 0x00000001;
pub const SOFTDIST_FLAG_USAGE_PRECACHE: DWORD = 0x00000002;
pub const SOFTDIST_FLAG_USAGE_AUTOINSTALL: DWORD = 0x00000004;
pub const SOFTDIST_FLAG_DELETE_SUBSCRIPTION: DWORD = 0x00000008;
pub const SOFTDIST_ADSTATE_NONE: DWORD = 0x00000000;
pub const SOFTDIST_ADSTATE_AVAILABLE: DWORD = 0x00000001;
pub const SOFTDIST_ADSTATE_DOWNLOADED: DWORD = 0x00000002;
pub const SOFTDIST_ADSTATE_INSTALLED: DWORD = 0x00000003;
STRUCT!{struct CODEBASEHOLD {
    cbSize: ULONG,
    szDistUnit: LPWSTR,
    szCodeBase: LPWSTR,
    dwVersionMS: DWORD,
    dwVersionLS: DWORD,
    dwStyle: DWORD,
}}
pub type LPCODEBASEHOLD = *mut CODEBASEHOLD;
STRUCT!{struct SOFTDISTINFO {
    cbSize: ULONG,
    dwFlags: DWORD,
    dwAdState: DWORD,
    szTitle: LPWSTR,
    szAbstract: LPWSTR,
    szHREF: LPWSTR,
    dwInstalledVersionMS: DWORD,
    dwInstalledVersionLS: DWORD,
    dwUpdateVersionMS: DWORD,
    dwUpdateVersionLS: DWORD,
    dwAdvertisedVersionMS: DWORD,
    dwAdvertisedVersionLS: DWORD,
    dwReserved: DWORD,
}}
pub type LPSOFTDISTINFO = *mut SOFTDISTINFO;
RIDL!(#[uuid(0xb15b8dc1, 0xc7e1, 0x11d0, 0x86, 0x80, 0x00, 0xaa, 0x00, 0xbd, 0xcb, 0x71)]
interface ISoftDistExt(ISoftDistExtVtbl): IUnknown(IUnknownVtbl) {
    fn ProcessSoftDist(
        szCDFURL: LPCWSTR,
        pSoftDistElement: *mut IXMLElement,
        lpsdi: LPSOFTDISTINFO,
    ) -> HRESULT,
    fn GetFirstCodeBase(
        szCodeBase: *mut LPWSTR,
        dwMaxSize: LPDWORD,
    ) -> HRESULT,
    fn GetNextCodeBase(
        szCodeBase: *mut LPWSTR,
        dwMaxSize: LPDWORD,
    ) -> HRESULT,
    fn AsyncInstallDistributionUnit(
        pbc: *mut IBindCtx,
        pvReserved: LPVOID,
        flags: DWORD,
        lpcbh: LPCODEBASEHOLD,
    ) -> HRESULT,
});
extern "system" {
    pub fn GetSoftwareUpdateInfo(
        szDistUnit: LPCWSTR,
        psdi: LPSOFTDISTINFO,
    ) -> HRESULT;
    pub fn SetSoftwareUpdateAdvertisementState(
        szDistUnit: LPCWSTR,
        dwAdState: DWORD,
        dwAdvertisedVersionMS: DWORD,
        dwAdvertisedVersionLS: DWORD,
    ) -> HRESULT;
}
pub type LPCATALOGFILEINFO = *mut ICatalogFileInfo;
RIDL!(#[uuid(0x711c7600, 0x6b48, 0x11d1, 0xb4, 0x03, 0x00, 0xaa, 0x00, 0xb9, 0x2a, 0xf1)]
interface ICatalogFileInfo(ICatalogFileInfoVtbl): IUnknown(IUnknownVtbl) {
    fn GetCatalogFile(
        ppszCatalogFile: *mut LPSTR,
    ) -> HRESULT,
    fn GetJavaTrust(
        ppJavaTrust: *mut *mut c_void,
    ) -> HRESULT,
});
pub type LPDATAFILTER = *mut IDataFilter;
RIDL!(#[uuid(0x69d14c80, 0xc18e, 0x11d0, 0xa9, 0xce, 0x00, 0x60, 0x97, 0x94, 0x23, 0x11)]
interface IDataFilter(IDataFilterVtbl): IUnknown(IUnknownVtbl) {
    fn DoEncode(
        dwFlags: DWORD,
        lInBufferSize: LONG,
        pbInBuffer: *mut BYTE,
        lOutBufferSize: LONG,
        pbOutBuffer: *mut BYTE,
        lInBytesAvailable: LONG,
        plInBytesRead: *mut LONG,
        plOutBytesWritten: *mut LONG,
        dwReserved: DWORD,
    ) -> HRESULT,
    fn DoDecode(
        dwFlags: DWORD,
        lInBufferSize: LONG,
        pbInBuffer: *mut BYTE,
        lOutBufferSize: LONG,
        pbOutBuffer: *mut BYTE,
        lInBytesAvailable: LONG,
        plInBytesRead: *mut LONG,
        plOutBytesWritten: *mut LONG,
        dwReserved: DWORD,
    ) -> HRESULT,
    fn SetEncodingLevel(
        dwEncLevel: DWORD,
    ) -> HRESULT,
});
STRUCT!{struct PROTOCOLFILTERDATA {
    cbSize: DWORD,
    pProtocolSink: *mut IInternetProtocolSink,
    pProtocol: *mut IInternetProtocol,
    pUnk: *mut IUnknown,
    dwFilterFlags: DWORD,
}}
pub type LPENCODINGFILTERFACTORY = *mut IEncodingFilterFactory;
STRUCT!{struct DATAINFO {
    ulTotalSize: ULONG,
    ulavrPacketSize: ULONG,
    ulConnectSpeed: ULONG,
    ulProcessorSpeed: ULONG,
}}
RIDL!(#[uuid(0x70bdde00, 0xc18e, 0x11d0, 0xa9, 0xce, 0x00, 0x60, 0x97, 0x94, 0x23, 0x11)]
interface IEncodingFilterFactory(IEncodingFilterFactoryVtbl): IUnknown(IUnknownVtbl) {
    fn FindBestFilter(
        pwzCodeIn: LPCWSTR,
        pwzCodeOut: LPCWSTR,
        info: DATAINFO,
        ppDF: *mut *mut IDataFilter,
    ) -> HRESULT,
    fn GetDefaultFilter(
        pwzCodeIn: LPCWSTR,
        pwzCodeOut: LPCWSTR,
        ppDF: *mut *mut IDataFilter,
    ) -> HRESULT,
});
extern "system" {
    pub fn IsLoggingEnabledA(
        pszUrl: LPCSTR,
    ) -> BOOL;
    pub fn IsLoggingEnabledW(
        pszUrl: LPCWSTR,
    ) -> BOOL;
}
STRUCT!{struct HIT_LOGGING_INFO {
    dwStructSize: DWORD,
    lpszLoggedUrlName: LPSTR,
    StartTime: SYSTEMTIME,
    EndTime: SYSTEMTIME,
    lpszExtendedInfo: LPSTR,
}}
pub type LPHIT_LOGGING_INFO = *mut HIT_LOGGING_INFO;
pub const CONFIRMSAFETYACTION_LOADOBJECT: DWORD = 0x00000001;
STRUCT!{struct CONFIRMSAFETY {
    clsid: CLSID,
    pUnk: *mut IUnknown,
    dwFlags: DWORD,
}}
pub type LPIWRAPPEDPROTOCOL = *mut IWrappedProtocol;
RIDL!(#[uuid(0x53c84785, 0x8425, 0x4dc5, 0x97, 0x1b, 0xe5, 0x8d, 0x9c, 0x19, 0xf9, 0xb6)]
interface IWrappedProtocol(IWrappedProtocolVtbl): IUnknown(IUnknownVtbl) {
    fn GetWrapperCode(
        pnCode: *mut LONG,
        dwReserved: DWORD_PTR,
    ) -> HRESULT,
});
pub type LPGETBINDHANDLE = *mut IGetBindHandle;
ENUM!{enum BINDHANDLETYPES {
    BINDHANDLETYPES_APPCACHE = 0,
    BINDHANDLETYPES_DEPENDENCY = 0x1,
    BINDHANDLETYPES_COUNT = BINDHANDLETYPES_DEPENDENCY + 1,
}}
RIDL!(#[uuid(0xaf0ff408, 0x129d, 0x4b20, 0x91, 0xf0, 0x02, 0xbd, 0x23, 0xd8, 0x83, 0x52)]
interface IGetBindHandle(IGetBindHandleVtbl): IUnknown(IUnknownVtbl) {
    fn GetBindHandle(
        enumRequestedHandle: BINDHANDLETYPES,
        pRetHandle: *mut HANDLE,
    ) -> HRESULT,
});
STRUCT!{struct PROTOCOL_ARGUMENT {
    szMethod: LPCWSTR,
    szTargetUrl: LPCWSTR,
}}
pub type LPPROTOCOL_ARGUMENT = *mut PROTOCOL_ARGUMENT;
pub type LPBINDCALLBACKREDIRECT = *mut IBindCallbackRedirect;
RIDL!(#[uuid(0x11c81bc2, 0x121e, 0x4ed5, 0xb9, 0xc4, 0xb4, 0x30, 0xbd, 0x54, 0xf2, 0xc0)]
interface IBindCallbackRedirect(IBindCallbackRedirectVtbl): IUnknown(IUnknownVtbl) {
    fn Redirect(
        lpcUrl: LPCWSTR,
        vbCancel: *mut VARIANT_BOOL,
    ) -> HRESULT,
});
RIDL!(#[uuid(0xa9eda967, 0xf50e, 0x4a33, 0xb3, 0x58, 0x20, 0x6f, 0x6e, 0xf3, 0x08, 0x6d)]
interface IBindHttpSecurity(IBindHttpSecurityVtbl): IUnknown(IUnknownVtbl) {
    fn GetIgnoreCertMask(
        pdwIgnoreCertMask: *mut DWORD,
    ) -> HRESULT,
});
extern "system" {
    pub fn BSTR_UserSize(
        pFlags: *mut c_ulong,
        Offset: c_ulong,
        pBstr: *mut BSTR,
    ) -> c_ulong;
    pub fn BSTR_UserMarshal(
        pFlags: *mut c_ulong,
        pBuffer: *mut c_uchar,
        pBstr: *mut BSTR,
    ) -> *mut c_uchar;
    pub fn BSTR_UserUnmarshal(
        pFlags: *mut c_ulong,
        pBuffer: *mut c_uchar,
        pBstr: *mut BSTR,
    ) -> *mut c_uchar;
    pub fn BSTR_UserFree(
        pFlags: *mut c_ulong,
        pBstr: *mut BSTR,
    );
    pub fn HWND_UserSize(
        pFlags: *mut c_ulong,
        Offset: c_ulong,
        pHwnd: *mut HWND,
    ) -> c_ulong;
    pub fn HWND_UserMarshal(
        pFlags: *mut c_ulong,
        pBuffer: *mut c_uchar,
        pHwnd: *mut HWND,
    ) -> *mut c_uchar;
    pub fn HWND_UserUnmarshal(
        pFlags: *mut c_ulong,
        pBuffer: *mut c_uchar,
        pHwnd: *mut HWND,
    ) -> *mut c_uchar;
    pub fn HWND_UserFree(
        pFlags: *mut c_ulong,
        pHwnd: *mut HWND,
    );
}
#[cfg(target_arch = "x86_64")]
extern "system" {
    pub fn BSTR_UserSize64(
        pFlags: *mut c_ulong,
        Offset: c_ulong,
        pBstr: *mut BSTR,
    ) -> c_ulong;
    pub fn BSTR_UserMarshal64(
        pFlags: *mut c_ulong,
        pBuffer: *mut c_uchar,
        pBstr: *mut BSTR,
    ) -> *mut c_uchar;
    pub fn BSTR_UserUnmarshal64(
        pFlags: *mut c_ulong,
        pBuffer: *mut c_uchar,
        pBstr: *mut BSTR,
    ) -> *mut c_uchar;
    pub fn BSTR_UserFree64(
        pFlags: *mut c_ulong,
        pBstr: *mut BSTR,
    );
    pub fn HWND_UserSize64(
        pFlags: *mut c_ulong,
        Offset: c_ulong,
        pHwnd: *mut HWND,
    ) -> c_ulong;
    pub fn HWND_UserMarshal64(
        pFlags: *mut c_ulong,
        pBuffer: *mut c_uchar,
        pHwnd: *mut HWND,
    ) -> *mut c_uchar;
    pub fn HWND_UserUnmarshal64(
        pFlags: *mut c_ulong,
        pBuffer: *mut c_uchar,
        pHwnd: *mut HWND,
    ) -> *mut c_uchar;
    pub fn HWND_UserFree64(
        pFlags: *mut c_ulong,
        pHwnd: *mut HWND,
    );
}

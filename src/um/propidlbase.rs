// Copyright © 2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::guiddef::{CLSID, FMTID, GUID, REFCLSID, REFFMTID};
use shared::minwindef::{
    BYTE, DWORD, FILETIME, FLOAT, HIBYTE, HIWORD, INT, LOBYTE, LOWORD, UCHAR, UINT, ULONG, USHORT,
    WORD
};
use shared::wtypes::{BSTR, BSTRBLOB, CLIPDATA, CY, DATE, DECIMAL, PROPID, VARIANT_BOOL, VARTYPE};
use shared::wtypesbase::{BLOB, DOUBLE, LPOLESTR, SCODE};
use um::oaidl::{IDispatch, LPSAFEARRAY};
use um::objidl::IStorage;
use um::objidlbase::IStream;
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::{CHAR, HRESULT, LARGE_INTEGER, LONG, LPSTR, LPWSTR, SHORT, ULARGE_INTEGER};
STRUCT!{struct VERSIONEDSTREAM {
    guidVersion: GUID,
    pStream: *mut IStream,
}}
pub type LPVERSIONEDSTREAM = *mut VERSIONEDSTREAM;
pub const PROPSETFLAG_DEFAULT: DWORD = 0;
pub const PROPSETFLAG_NONSIMPLE: DWORD = 1;
pub const PROPSETFLAG_ANSI: DWORD = 2;
pub const PROPSETFLAG_UNBUFFERED: DWORD = 4;
pub const PROPSETFLAG_CASE_SENSITIVE: DWORD = 8;
pub const PROPSET_BEHAVIOR_CASE_SENSITIVE: DWORD = 1;
pub type PROPVARIANT = tagPROPVARIANT;
STRUCT!{struct CAC {
    cElems: ULONG,
    pElems: *mut CHAR,
}}
STRUCT!{struct CAUB {
    cElems: ULONG,
    pElems: *mut UCHAR,
}}
STRUCT!{struct CAI {
    cElems: ULONG,
    pElems: *mut SHORT,
}}
STRUCT!{struct CAUI {
    cElems: ULONG,
    pElems: *mut USHORT,
}}
STRUCT!{struct CAL {
    cElems: ULONG,
    pElems: *mut LONG,
}}
STRUCT!{struct CAUL {
    cElems: ULONG,
    pElems: *mut ULONG,
}}
STRUCT!{struct CAFLT {
    cElems: ULONG,
    pElems: *mut FLOAT,
}}
STRUCT!{struct CADBL {
    cElems: ULONG,
    pElems: *mut DOUBLE,
}}
STRUCT!{struct CACY {
    cElems: ULONG,
    pElems: *mut CY,
}}
STRUCT!{struct CADATE {
    cElems: ULONG,
    pElems: *mut DATE,
}}
STRUCT!{struct CABSTR {
    cElems: ULONG,
    pElems: *mut BSTR,
}}
STRUCT!{struct CABSTRBLOB {
    cElems: ULONG,
    pElems: *mut BSTRBLOB,
}}
STRUCT!{struct CABOOL {
    cElems: ULONG,
    pElems: *mut VARIANT_BOOL,
}}
STRUCT!{struct CASCODE {
    cElems: ULONG,
    pElems: *mut SCODE,
}}
STRUCT!{struct CAPROPVARIANT {
    cElems: ULONG,
    pElems: *mut PROPVARIANT,
}}
STRUCT!{struct CAH {
    cElems: ULONG,
    pElems: *mut LARGE_INTEGER,
}}
STRUCT!{struct CAUH {
    cElems: ULONG,
    pElems: *mut ULARGE_INTEGER,
}}
STRUCT!{struct CALPSTR {
    cElems: ULONG,
    pElems: *mut LPSTR,
}}
STRUCT!{struct CALPWSTR {
    cElems: ULONG,
    pElems: *mut LPWSTR,
}}
STRUCT!{struct CAFILETIME {
    cElems: ULONG,
    pElems: *mut FILETIME,
}}
STRUCT!{struct CACLIPDATA {
    cElems: ULONG,
    pElems: *mut CLIPDATA,
}}
STRUCT!{struct CACLSID {
    cElems: ULONG,
    pElems: *mut CLSID,
}}
pub type PROPVAR_PAD1 = WORD;
pub type PROPVAR_PAD2 = WORD;
pub type PROPVAR_PAD3 = WORD;
UNION!{union tag_inner_PROPVARIANT_u {
    [usize; 2],
    cVal cVal_mut: CHAR,
    bVal bVal_mut: UCHAR,
    iVal iVal_mut: SHORT,
    uiVal uiVal_mut: USHORT,
    lVal lVal_mut: LONG,
    ulVal ulVal_mut: ULONG,
    intVal intVal_mut: INT,
    uintVal uintVal_mut: UINT,
    hVal hVal_mut: LARGE_INTEGER,
    uhVal uhVal_mut: ULARGE_INTEGER,
    fltVal fltVal_mut: FLOAT,
    dblVal dblVal_mut: DOUBLE,
    boolVal boolVal_mut: VARIANT_BOOL,
    scode scode_mut: SCODE,
    cyVal cyVal_mut: CY,
    date date_mut: DATE,
    filetime filetime_mut: FILETIME,
    puuid puuid_mut: *mut CLSID,
    pclipdata pclipdata_mut: *mut CLIPDATA,
    bstrVal bstrVal_mut: BSTR,
    bstrblobVal bstrblobVal_mut: BSTRBLOB,
    blob blob_mut: BLOB,
    pszVal pszVal_mut: LPSTR,
    pwszVal pwszVal_mut: LPWSTR,
    punkVal punkVal_mut: *mut IUnknown,
    pdispVal pdispVal_mut: *mut IDispatch,
    pStream pStream_mut: *mut IStream,
    pStorage pStorage_mut: *mut IStorage,
    pVersionedStream pVersionedStream_mut: LPVERSIONEDSTREAM,
    parray parray_mut: LPSAFEARRAY,
    cac cac_mut: CAC,
    caub caub_mut: CAUB,
    cai cai_mut: CAI,
    caui caui_mut: CAUI,
    cal cal_mut: CAL,
    caul caul_mut: CAUL,
    cah cah_mut: CAH,
    cauh cauh_mut: CAUH,
    caflt caflt_mut: CAFLT,
    cadbl cadbl_mut: CADBL,
    cabool cabool_mut: CABOOL,
    cascode cascode_mut: CASCODE,
    cacy cacy_mut: CACY,
    cadate cadate_mut: CADATE,
    cafiletime cafiletime_mut: CAFILETIME,
    cauuid cauuid_mut: CACLSID,
    caclipdata caclipdata_mut: CACLIPDATA,
    cabstr cabstr_mut: CABSTR,
    cabstrblob cabstrblob_mut: CABSTRBLOB,
    calpstr calpstr_mut: CALPSTR,
    calpwstr calpwstr_mut: CALPWSTR,
    capropvar capropvar_mut: CAPROPVARIANT,
    pcVal pcVal_mut: *mut CHAR,
    pbVal pbVal_mut: *mut UCHAR,
    piVal piVal_mut: *mut SHORT,
    puiVal puiVal_mut: *mut USHORT,
    plVal plVal_mut: *mut LONG,
    pulVal pulVal_mut: *mut ULONG,
    pintVal pintVal_mut: *mut INT,
    puintVal puintVal_mut: *mut UINT,
    pfltVal pfltVal_mut: *mut FLOAT,
    pdblVal pdblVal_mut: *mut DOUBLE,
    pboolVal pboolVal_mut: *mut VARIANT_BOOL,
    pdecVal pdecVal_mut: *mut DECIMAL,
    pscode pscode_mut: *mut SCODE,
    pcyVal pcyVal_mut: *mut CY,
    pdate pdate_mut: *mut DATE,
    pbstrVal pbstrVal_mut: *mut BSTR,
    ppunkVal ppunkVal_mut: *mut *mut IUnknown,
    ppdispVal ppdispVal_mut: *mut *mut IDispatch,
    pparray pparray_mut: *mut LPSAFEARRAY,
    pvarVal pvarVal_mut: *mut PROPVARIANT,
}}
STRUCT!{struct tag_inner_PROPVARIANT {
    vt: VARTYPE,
    wReserved1: PROPVAR_PAD1,
    wReserved2: PROPVAR_PAD2,
    wReserved3: PROPVAR_PAD3,
    u: tag_inner_PROPVARIANT_u,
}}
UNION!{union tagPROPVARIANT_u {
    [u32; 4] [u64; 3],
    s s_mut: tag_inner_PROPVARIANT,
    decVal decVal_mut: DECIMAL,
}}
STRUCT!{struct tagPROPVARIANT {
    u: tagPROPVARIANT_u,
}}
pub type LPPROPVARIANT = *mut tagPROPVARIANT;
pub type REFPROPVARIANT = *const PROPVARIANT;
pub const PID_DICTIONARY: ULONG = 0;
pub const PID_CODEPAGE: ULONG = 0x1;
pub const PID_FIRST_USABLE: ULONG = 0x2;
pub const PID_FIRST_NAME_DEFAULT: ULONG = 0xfff;
pub const PID_LOCALE: ULONG = 0x80000000;
pub const PID_MODIFY_TIME: ULONG = 0x80000001;
pub const PID_SECURITY: ULONG = 0x80000002;
pub const PID_BEHAVIOR: ULONG = 0x80000003;
pub const PID_ILLEGAL: ULONG = 0xffffffff;
pub const PID_MIN_READONLY: ULONG = 0x80000000;
pub const PID_MAX_READONLY: ULONG = 0xbfffffff;
pub const PRSPEC_INVALID: ULONG = 0xffffffff;
pub const PRSPEC_LPWSTR: ULONG = 0;
pub const PRSPEC_PROPID: ULONG = 1;
UNION!{union tagPROPSPEC_u {
    [usize; 1],
    propid propid_mut: PROPID,
    lpwstr lpwstr_mut: LPOLESTR,
}}
STRUCT!{struct PROPSPEC {
    ulKind: ULONG,
    u: tagPROPSPEC_u,
}}
STRUCT!{struct STATPROPSTG {
    lpwstrName: LPOLESTR,
    propid: PROPID,
    vt: VARTYPE,
}}
#[inline]
pub fn PROPSETHDR_OSVER_KIND(dwOSVer: DWORD) -> WORD {
    HIWORD(dwOSVer)
}
#[inline]
pub fn PROPSETHDR_OSVER_MAJOR(dwOSVer: DWORD) -> BYTE {
    LOBYTE(LOWORD(dwOSVer))
}
#[inline]
pub fn PROPSETHDR_OSVER_MINOR(dwOSVer: DWORD) -> BYTE {
    HIBYTE(LOWORD(dwOSVer))
}
pub const PROPSETHDR_OSVERSION_UNKNOWN: DWORD = 0xFFFFFFFF;
STRUCT!{struct STATPROPSETSTG {
    fmtid: FMTID,
    clsid: CLSID,
    grfFlags: DWORD,
    mtime: FILETIME,
    ctime: FILETIME,
    atime: FILETIME,
    dwOSVersion: DWORD,
}}
// extern RPC_IF_HANDLE __MIDL_itf_propidl_0000_0000_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_propidl_0000_0000_v0_0_s_ifspec;
DEFINE_GUID!{IID_IPropertyStorage,
    0x00000138, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x00000138, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IPropertyStorage(IPropertyStorageVtbl): IUnknown(IUnknownVtbl) {
    fn ReadMultiple(
        cpspec: ULONG,
        rgpspec: *const PROPSPEC,
        rgpropvar: *mut PROPVARIANT,
    ) -> HRESULT,
    fn WriteMultiple(
        cpspec: ULONG,
        rgpspec: *const PROPSPEC,
        rgpropvar: *const PROPVARIANT,
        propidNameFirst: PROPID,
    ) -> HRESULT,
    fn DeleteMultiple(
        cpspec: ULONG,
        rgpspec: *const PROPSPEC,
    ) -> HRESULT,
    fn ReadPropertyNames(
        cpropid: ULONG,
        rgpropid: *const PROPID,
        rglpwstrName: *mut LPOLESTR,
    ) -> HRESULT,
    fn WritePropertyNames(
        cpropid: ULONG,
        rgpropid: *const PROPID,
        rglpwstrName: *const LPOLESTR,
    ) -> HRESULT,
    fn DeletePropertyNames(
        cpropid: ULONG,
        rgpropid: *const PROPID,
    ) -> HRESULT,
    fn Commit(
        grfCommitFlags: DWORD,
    ) -> HRESULT,
    fn Revert() -> HRESULT,
    fn Enum(
        ppenum: *mut *mut IEnumSTATPROPSTG,
    ) -> HRESULT,
    fn SetTimes(
        pctime: *const FILETIME,
        patime: *const FILETIME,
        pmtime: *const FILETIME,
    ) -> HRESULT,
    fn SetClass(
        clsid: REFCLSID,
    ) -> HRESULT,
    fn Stat(
        pstatpsstg: *mut STATPROPSETSTG,
    ) -> HRESULT,
}}
pub type LPPROPERTYSETSTORAGE = *mut IPropertySetStorage;
DEFINE_GUID!{IID_IPropertySetStorage,
    0x0000013a, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x0000013a, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IPropertySetStorage(IPropertySetStorageVtbl): IUnknown(IUnknownVtbl) {
    fn Create(
        rfmtid: REFFMTID,
        pclsid: *const CLSID,
        grfFlags: DWORD,
        grfMode: DWORD,
        ppprstg: *mut *mut IPropertyStorage,
    ) -> HRESULT,
    fn Open(
        rfmtid: REFFMTID,
        grfMode: DWORD,
        ppprstg: *mut *mut IPropertyStorage,
    ) -> HRESULT,
    fn Delete(
        rfmtid: REFFMTID,
    ) -> HRESULT,
    fn Enum(
        ppenum: *mut *mut IEnumSTATPROPSETSTG,
    ) -> HRESULT,
}}
pub type LPENUMSTATPROPSTG = *mut IEnumSTATPROPSTG;
DEFINE_GUID!{IID_IEnumSTATPROPSTG,
    0x00000139, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x00000139, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IEnumSTATPROPSTG(IEnumSTATPROPSTGVtbl): IUnknown(IUnknownVtbl) {
    fn Next(
        celt: ULONG,
        rgelt: *mut STATPROPSTG,
        pceltFetched: *mut ULONG,
    ) -> HRESULT,
    fn Skip(
        celt: ULONG,
    ) -> HRESULT,
    fn Reset() -> HRESULT,
    fn Clone(
        ppenum: *mut *mut IEnumSTATPROPSTG,
    ) -> HRESULT,
}}
// HRESULT STDMETHODCALLTYPE IEnumSTATPROPSTG_RemoteNext_Proxy(
//     IEnumSTATPROPSTG * This,
//     LONG celt,
//     STATPROPSTG *rgelt,
//     ULONG *pceltFetched);
// void __RPC_STUB IEnumSTATPROPSTG_RemoteNext_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
pub type LPENUMSTATPROPSETSTG = *mut IEnumSTATPROPSETSTG;
DEFINE_GUID!{IID_IEnumSTATPROPSETSTG,
    0x0000013b, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x0000013b, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IEnumSTATPROPSETSTG(IEnumSTATPROPSETSTGVtbl): IUnknown(IUnknownVtbl) {
    fn Next(
        celt: ULONG,
        rgelt: *mut STATPROPSETSTG,
        pceltFetched: *mut ULONG,
    ) -> HRESULT,
    fn Skip(
        celt: LONG,
    ) -> HRESULT,
    fn Reset() -> HRESULT,
    fn Clone(
        ppenum: *mut *mut IEnumSTATPROPSETSTG,
    ) -> HRESULT,
}}
// HRESULT STDMETHODCALLTYPE IEnumSTATPROPSETSTG_RemoteNext_Proxy(
//     IEnumSTATPROPSETSTG * This,
//     LONG celt,
//     STATPROPSETSTG *rgelt,
//     ULONG *pceltFetched);
// void __RPC_STUB IEnumSTATPROPSETSTG_RemoteNext_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
pub type LPPROPERTYSTORAGE = *mut IPropertyStorage;
// extern RPC_IF_HANDLE __MIDL_itf_propidl_0000_0004_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_propidl_0000_0004_v0_0_s_ifspec;
// unsigned long __RPC_USER BSTR_UserSize( __RPC__in unsigned long *, unsigned long, __RPC__in BSTR * );
// unsigned char * __RPC_USER BSTR_UserMarshal( __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * );
// unsigned char * __RPC_USER BSTR_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * );
// void __RPC_USER BSTR_UserFree( __RPC__in unsigned long *, __RPC__in BSTR * );
// unsigned long __RPC_USER LPSAFEARRAY_UserSize( __RPC__in unsigned long *, unsigned long, __RPC__in LPSAFEARRAY * );
// unsigned char * __RPC_USER LPSAFEARRAY_UserMarshal( __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in LPSAFEARRAY * );
// unsigned char * __RPC_USER LPSAFEARRAY_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out LPSAFEARRAY * );
// void __RPC_USER LPSAFEARRAY_UserFree( __RPC__in unsigned long *, __RPC__in LPSAFEARRAY * );
// unsigned long __RPC_USER BSTR_UserSize64( __RPC__in unsigned long *, unsigned long, __RPC__in BSTR * );
// unsigned char * __RPC_USER BSTR_UserMarshal64( __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * );
// unsigned char * __RPC_USER BSTR_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * );
// void __RPC_USER BSTR_UserFree64( __RPC__in unsigned long *, __RPC__in BSTR * );
// unsigned long __RPC_USER LPSAFEARRAY_UserSize64( __RPC__in unsigned long *, unsigned long, __RPC__in LPSAFEARRAY * );
// unsigned char * __RPC_USER LPSAFEARRAY_UserMarshal64( __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in LPSAFEARRAY * );
// unsigned char * __RPC_USER LPSAFEARRAY_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out LPSAFEARRAY * );
// void __RPC_USER LPSAFEARRAY_UserFree64( __RPC__in unsigned long *, __RPC__in LPSAFEARRAY * );
// HRESULT STDMETHODCALLTYPE IEnumSTATPROPSTG_Next_Proxy(
//     IEnumSTATPROPSTG * This,
//     LONG celt,
//     STATPROPSTG *rgelt,
//     ULONG *pceltFetched);
// HRESULT STDMETHODCALLTYPE IEnumSTATPROPSTG_Next_Stub(
//     IEnumSTATPROPSTG * This,
//     LONG celt,
//     STATPROPSTG *rgelt,
//     ULONG *pceltFetched);
// HRESULT STDMETHODCALLTYPE IEnumSTATPROPSETSTG_Next_Proxy(
//     IEnumSTATPROPSETSTG * This,
//     LONG celt,
//     STATPROPSETSTG *rgelt,
//     ULONG *pceltFetched);
// HRESULT STDMETHODCALLTYPE IEnumSTATPROPSETSTG_Next_Stub(
//     IEnumSTATPROPSETSTG * This,
//     LONG celt,
//     STATPROPSETSTG *rgelt,
//     ULONG *pceltFetched);

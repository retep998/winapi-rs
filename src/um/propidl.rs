// Copyright © 2016-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::guiddef::CLSID;
use shared::minwindef::{FILETIME, FLOAT, INT, UINT, WORD};
use shared::ntdef::{
    CHAR, LARGE_INTEGER, LONG, LPSTR, LPWSTR, SHORT, UCHAR, ULARGE_INTEGER, ULONG, USHORT
};
use shared::wtypes::{BSTR, CY, DATE, DECIMAL, VARIANT_BOOL, VARTYPE};
use shared::wtypesbase::{BLOB, DOUBLE, SCODE};
use um::oaidl::{IDispatch, LPSAFEARRAY};
use um::objidlbase::IStream;
use um::unknwnbase::IUnknown;
UNION!{union PROPVARIANT_u {
    [u64; 1] [u64; 2],
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
    // bool bool_mut: _VARIANT_BOOL,
    scode scode_mut: SCODE,
    cyVal cyVal_mut: CY,
    date date_mut: DATE,
    filetime filetime_mut: FILETIME,
    puuid puuid_mut: *mut CLSID,
    // pclipdata pclipdata_mut: *mut CLIPDATA,
    bstrVal bstrVal_mut: BSTR,
    // bstrblobVal bstrblobVal_mut: BSTRBLOB,
    blob blob_mut: BLOB,
    pszVal pszVal_mut: LPSTR,
    pwszVal pwszVal_mut: LPWSTR,
    punkVal punkVal_mut: *mut IUnknown,
    pdispVal pdisp_mut: *mut IDispatch,
    pStream pStream_mut: *mut IStream,
    // pStorage pStorage_mut: *mut IStorage,
    // pVersionedStream pVersionedStream_mut: LPVERSIONEDSTREAM,
    parray parray_mut: LPSAFEARRAY,
    // cac cac_mut: CAC,
    // caub caub_mut: CAUB,
    // cai cai_mut: CAI,
    // caui caui_mut: CAUI,
    // cal cal_mut: CAL,
    // caul caul_mut: CAUL,
    // cah cah_mut: CAH,
    // cauh cauh_mut: CAUH,
    // caflt caflt_mut: CAFLT,
    // cadbl cadbl_mut: CADBL,
    // cabool cabool_mut: CABOOL,
    // cascode cascode_mut: CASCODE,
    // cacy cacy_mut: CACY,
    // cadate cadate_mut: CADATE,
    // cafiletime cafiletime_mut: CAFILETIME,
    // cauuid cauuid_mut: CACLSID,
    // caclipdata caclipdata_mut: CACLIPDATA,
    // cabstr cabstr_mut: CABSTR,
    // cabstrblob cabstrblob_mut: CABSTRBLOB,
    // calpstr calpstr_mut: CALPSTR,
    // calpwstr calpwstr_mut: CALPWSTR,
    // capropvar capropvar_mut: CAPROPVARIANT,
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
    ppunkVal ppunkVal_mut: *mut *mut IUnknown,
    ppdispVal ppdispVal_mut: *mut *mut IDispatch,
    ppStream ppStream_mut: *mut *mut IStream,
    // ppStorage ppStorage_mut: *mut *mut IStorage,
}}
STRUCT!{struct PROPVARIANT {
    vt: VARTYPE,
    wReserved1: WORD,
    wReserved2: WORD,
    wReserved3: WORD,
    data: PROPVARIANT_u,
}}
pub type REFPROPVARIANT = *const PROPVARIANT;

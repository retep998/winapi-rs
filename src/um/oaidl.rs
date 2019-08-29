// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Mappings for the contents of OAIdl.h
use shared::basetsd::ULONG_PTR;
use shared::guiddef::{GUID, IID, REFGUID, REFIID};
use shared::minwindef::{BOOL, BYTE, DWORD, FLOAT, INT, UINT, ULONG, USHORT, WORD};
use shared::rpcndr::byte;
use shared::wtypes::{
    BSTR, CY, DATE, DECIMAL, VARIANT_BOOL, VARTYPE, VT_BSTR, VT_DISPATCH, VT_ERROR, VT_I1, VT_I2,
    VT_I4, VT_I8, VT_RECORD, VT_RESERVED, VT_UNKNOWN, VT_VARIANT, wireBSTR
};
use shared::wtypesbase::{
    BYTE_SIZEDARR, DOUBLE, DWORD_SIZEDARR, HYPER_SIZEDARR, LPCOLESTR, LPOLESTR, SCODE,
    WORD_SIZEDARR
};
use um::objidlbase::IEnumUnknown;
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::{CHAR, HRESULT, LCID, LONG, LONGLONG, PVOID, SHORT, ULONGLONG};
// extern RPC_IF_HANDLE __MIDL_itf_oaidl_0000_0000_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_oaidl_0000_0000_v0_0_s_ifspec;
pub type CURRENCY = CY;
STRUCT!{struct SAFEARRAYBOUND {
    cElements: ULONG,
    lLbound: LONG,
}}
pub type LPSAFEARRAYBOUND = *mut SAFEARRAYBOUND;
pub type wireBRECORD = *mut _wireBRECORD;
pub type wireVARIANT = *mut _wireVARIANT;
STRUCT!{struct SAFEARR_BSTR {
    Size: ULONG,
    aBstr: *mut wireBSTR,
}}
STRUCT!{struct SAFEARR_UNKNOWN {
    Size: ULONG,
    apUnknown: *mut *mut IUnknown,
}}
STRUCT!{struct SAFEARR_DISPATCH {
    Size: ULONG,
    apDispatch: *mut *mut IDispatch,
}}
STRUCT!{struct SAFEARR_VARIANT {
    Size: ULONG,
    aVariant: *mut wireVARIANT,
}}
STRUCT!{struct SAFEARR_BRECORD {
    Size: ULONG,
    aRecord: *mut wireBRECORD,
}}
STRUCT!{struct SAFEARR_HAVEIID {
    Size: ULONG,
    apUnknown: *mut *mut IUnknown,
    iid: IID,
}}
ENUM!{enum SF_TYPE {
    SF_ERROR = VT_ERROR,
    SF_I1 = VT_I1,
    SF_I2 = VT_I2,
    SF_I4 = VT_I4,
    SF_I8 = VT_I8,
    SF_BSTR = VT_BSTR,
    SF_UNKNOWN = VT_UNKNOWN,
    SF_DISPATCH = VT_DISPATCH,
    SF_VARIANT = VT_VARIANT,
    SF_RECORD = VT_RECORD,
    SF_HAVEIID = VT_UNKNOWN | VT_RESERVED,
}}
UNION!{union SAFEARRAYUNION_u {
    [u32; 6] [u64; 4],
    BstrStr BstrStr_mut: SAFEARR_BSTR,
    UnknownStr UnknownStr_mut: SAFEARR_UNKNOWN,
    DispatchStr DispatchStr_mut: SAFEARR_DISPATCH,
    VariantStr VariantStr_mut: SAFEARR_VARIANT,
    RecordStr RecordStr_mut: SAFEARR_BRECORD,
    HaveIidStr HaveIidStr_mut: SAFEARR_HAVEIID,
    ByteStr ByteStr_mut: BYTE_SIZEDARR,
    WordStr WordStr_mut: WORD_SIZEDARR,
    LongStr LongStr_mut: DWORD_SIZEDARR,
    HyperStr HyperStr_mut: HYPER_SIZEDARR,
}}
STRUCT!{struct SAFEARRAYUNION {
    sfType: ULONG,
    u: SAFEARRAYUNION_u,
}}
STRUCT!{struct _wireSAFEARRAY {
    cDims: USHORT,
    fFeatures: USHORT,
    cbElements: ULONG,
    cLocks: ULONG,
    uArrayStructs: SAFEARRAYUNION,
    rgsaBound: [SAFEARRAYBOUND; 1],
}}
pub type wireSAFEARRAY = *mut _wireSAFEARRAY;
pub type wirePSAFEARRAY = *mut wireSAFEARRAY;
STRUCT!{struct SAFEARRAY {
    cDims: USHORT,
    fFeatures: USHORT,
    cbElements: ULONG,
    cLocks: ULONG,
    pvData: PVOID,
    rgsabound: [SAFEARRAYBOUND; 1],
}}
pub type LPSAFEARRAY = *mut SAFEARRAY;
pub const FADF_AUTO: DWORD = 0x1;
pub const FADF_STATIC: DWORD = 0x2;
pub const FADF_EMBEDDED: DWORD = 0x4;
pub const FADF_FIXEDSIZE: DWORD = 0x10;
pub const FADF_RECORD: DWORD = 0x20;
pub const FADF_HAVEIID: DWORD = 0x40;
pub const FADF_HAVEVARTYPE: DWORD = 0x80;
pub const FADF_BSTR: DWORD = 0x100;
pub const FADF_UNKNOWN: DWORD = 0x200;
pub const FADF_DISPATCH: DWORD = 0x400;
pub const FADF_VARIANT: DWORD = 0x800;
pub const FADF_RESERVED: DWORD = 0xf008;
STRUCT!{struct __tagBRECORD {
    pvRecord: PVOID,
    pRecInfo: *mut IRecordInfo,
}}
UNION!{union VARIANT_n3 {
    [u64; 1] [u64; 2],
    llVal llVal_mut: LONGLONG,
    lVal lVal_mut: LONG,
    bVal bVal_mut: BYTE,
    iVal iVal_mut: SHORT,
    fltVal fltVal_mut: FLOAT,
    dblVal dblVal_mut: DOUBLE,
    boolVal boolVal_mut: VARIANT_BOOL,
    scode scode_mut: SCODE,
    cyVal cyVal_mut: CY,
    date date_mut: DATE,
    bstrVal bstrVal_mut: BSTR,
    punkVal punkVal_mut: *mut IUnknown,
    pdispVal pdispVal_mut: *mut IDispatch,
    parray parray_mut: *mut SAFEARRAY,
    pbVal pbVal_mut: *mut BYTE,
    piVal piVal_mut: *mut SHORT,
    plVal plVal_mut: *mut LONG,
    pllVal pllVal_mut: *mut LONGLONG,
    pfltVal pfltVal_mut: *mut FLOAT,
    pdblVal pdblVal_mut: *mut DOUBLE,
    pboolVal pboolVal_mut: *mut VARIANT_BOOL,
    pscode pscode_mut: *mut SCODE,
    pcyVal pcyVal_mut: *mut CY,
    pdate pdate_mut: *mut DATE,
    pbstrVal pbstrVal_mut: *mut BSTR,
    ppunkVal ppunkVal_mut: *mut *mut IUnknown,
    ppdispVal ppdispVal_mut: *mut *mut IDispatch,
    pparray pparray_mut: *mut *mut SAFEARRAY,
    pvarVal pvarVal_mut: *mut VARIANT,
    byref byref_mut: PVOID,
    cVal cVal_mut: CHAR,
    uiVal uiVal_mut: USHORT,
    ulVal ulVal_mut: ULONG,
    ullVal ullVal_mut: ULONGLONG,
    intVal intVal_mut: INT,
    uintVal uintVal_mut: UINT,
    pdecVal pdecVal_mut: *mut DECIMAL,
    pcVal pcVal_mut: *mut CHAR,
    puiVal puiVal_mut: *mut USHORT,
    pulVal pulVal_mut: *mut ULONG,
    pullVal pullVal_mut: *mut ULONGLONG,
    pintVal pintVal_mut: *mut INT,
    puintVal puintVal_mut: *mut UINT,
    n4 n4_mut: __tagBRECORD,
}}
STRUCT!{struct __tagVARIANT {
    vt: VARTYPE,
    wReserved1: WORD,
    wReserved2: WORD,
    wReserved3: WORD,
    n3: VARIANT_n3,
}}
UNION!{union VARIANT_n1 {
    [u64; 2] [u64; 3],
    n2 n2_mut: __tagVARIANT,
    decVal decVal_mut: DECIMAL,
}}
STRUCT!{struct VARIANT {
    n1: VARIANT_n1,
}}
pub type LPVARIANT = *mut VARIANT;
pub type VARIANTARG = VARIANT;
pub type LPVARIANTARG = *mut VARIANT;
pub type REFVARIANT = *const VARIANT;
STRUCT!{struct _wireBRECORD {
    fFlags: ULONG,
    clSize: ULONG,
    pRecInfo: *mut IRecordInfo,
    pRecord: *mut byte,
}}
UNION!{union _wireVARIANT_u {
    [u64; 2],
    llVal llVal_mut: LONGLONG,
    lVal lVal_mut: LONG,
    bVal bVal_mut: BYTE,
    iVal iVal_mut: SHORT,
    fltVal fltVal_mut: FLOAT,
    dblVal dblVal_mut: DOUBLE,
    boolVal boolVal_mut: VARIANT_BOOL,
    scode scode_mut: SCODE,
    cyVal cyVal_mut: CY,
    date date_mut: DATE,
    bstrVal bstrVal_mut: wireBSTR,
    punkVal punkVal_mut: *mut IUnknown,
    pdispVal pdispVal_mut: *mut IDispatch,
    parray parray_mut: wirePSAFEARRAY,
    brecVal brecVal_mut: wireBRECORD,
    pbVal pbVal_mut: *mut BYTE,
    piVal piVal_mut: *mut SHORT,
    plVal plVal_mut: *mut LONG,
    pllVal pllVal_mut: *mut LONGLONG,
    pfltVal pfltVal_mut: *mut FLOAT,
    pdblVal pdblVal_mut: *mut DOUBLE,
    pboolVal pboolVal_mut: *mut VARIANT_BOOL,
    pscode pscode_mut: *mut SCODE,
    pcyVal pcyVal_mut: *mut CY,
    pdate pdate_mut: *mut DATE,
    pbstrVal pbstrVal_mut: *mut wireBSTR,
    ppunkVal ppunkVal_mut: *mut *mut IUnknown,
    ppdispVal ppdispVal_mut: *mut *mut IDispatch,
    pparray pparray_mut: *mut wirePSAFEARRAY,
    pvarVal pvarVal_mut: *mut wireVARIANT,
    cVal cVal_mut: CHAR,
    uiVal uiVal_mut: USHORT,
    ulVal ulVal_mut: ULONG,
    ullVal ullVal_mut: ULONGLONG,
    intVal intVal_mut: INT,
    uintVal uintVal_mut: UINT,
    decVal decVal_mut: DECIMAL,
    pdecVal pdecVal_mut: *mut DECIMAL,
    pcVal pcVal_mut: *mut CHAR,
    puiVal puiVal_mut: *mut USHORT,
    pulVal pulVal_mut: *mut ULONG,
    pullVal pullVal_mut: *mut ULONGLONG,
    pintVal pintVal_mut: *mut INT,
    puintVal puintVal_mut: *mut UINT,
}}
STRUCT!{struct _wireVARIANT {
    clSize: DWORD,
    rpcReserved: DWORD,
    vt: USHORT,
    wReserved1: USHORT,
    wReserved2: USHORT,
    wReserved3: USHORT,
    u: _wireVARIANT_u,
}}
pub type DISPID = LONG;
pub type MEMBERID = DISPID;
pub type HREFTYPE = DWORD;
ENUM!{enum TYPEKIND {
    TKIND_ENUM = 0,
    TKIND_RECORD,
    TKIND_MODULE,
    TKIND_INTERFACE,
    TKIND_DISPATCH,
    TKIND_COCLASS,
    TKIND_ALIAS,
    TKIND_UNION,
    TKIND_MAX,
}}
UNION!{union TYPEDESC_u {
    [usize; 1],
    lptdesc lptdesc_mut: *mut TYPEDESC,
    lpadesc lpadesc_mut: *mut ARRAYDESC,
    hreftype hreftype_mut: HREFTYPE,
}}
STRUCT!{struct TYPEDESC {
    u: TYPEDESC_u,
    vt: VARTYPE,
}}
STRUCT!{struct ARRAYDESC {
    tdescElem: TYPEDESC,
    cDims: USHORT,
    rgbounds: [SAFEARRAYBOUND; 1],
}}
STRUCT!{struct PARAMDESCEX {
    cBytes: ULONG,
    varDefaultValue: VARIANTARG,
}}
pub type LPPARAMDESCEX = *mut PARAMDESCEX;
STRUCT!{struct PARAMDESC {
    pparamdescex: LPPARAMDESCEX,
    wParamFlags: USHORT,
}}
pub type LPPARAMDESC = *mut PARAMDESC;
pub const PARAMFLAG_NONE: DWORD = 0;
pub const PARAMFLAG_FIN: DWORD = 0x1;
pub const PARAMFLAG_FOUT: DWORD = 0x2;
pub const PARAMFLAG_FLCID: DWORD = 0x4;
pub const PARAMFLAG_FRETVAL: DWORD = 0x8;
pub const PARAMFLAG_FOPT: DWORD = 0x10;
pub const PARAMFLAG_FHASDEFAULT: DWORD = 0x20;
pub const PARAMFLAG_FHASCUSTDATA: DWORD = 0x40;
STRUCT!{struct IDLDESC {
    dwReserved: ULONG_PTR,
    wIDLFlags: USHORT,
}}
pub type LPIDLDESC = *mut IDLDESC;
pub const IDLFLAG_NONE: DWORD = PARAMFLAG_NONE;
pub const IDLFLAG_FIN: DWORD = PARAMFLAG_FIN;
pub const IDLFLAG_FOUT: DWORD = PARAMFLAG_FOUT;
pub const IDLFLAG_FLCID: DWORD = PARAMFLAG_FLCID;
pub const IDLFLAG_FRETVAL: DWORD = PARAMFLAG_FRETVAL;
UNION!{union ELEMDESC_u {
    [usize; 2],
    idldesc idldesc_mut: IDLDESC,
    paramdesc paramdesc_mut: PARAMDESC,
}}
STRUCT!{struct ELEMDESC {
    tdesc: TYPEDESC,
    u: ELEMDESC_u,
}}
pub type LPELEMDESC = *mut ELEMDESC;
STRUCT!{struct TYPEATTR {
    guid: GUID,
    lcid: LCID,
    dwReserved: DWORD,
    memidConstructor: MEMBERID,
    memidDestructor: MEMBERID,
    lpstrSchema: LPOLESTR,
    cbSizeInstance: ULONG,
    typekind: TYPEKIND,
    cFuncs: WORD,
    cVars: WORD,
    cImplTypes: WORD,
    cbSizeVft: WORD,
    cbAlignment: WORD,
    wTypeFlags: WORD,
    wMajorVerNum: WORD,
    wMinorVerNum: WORD,
    tdescAlias: TYPEDESC,
    idldescType: IDLDESC,
}}
pub type LPTYPEATTR = *mut TYPEATTR;
STRUCT!{struct DISPPARAMS {
    rgvarg: *mut VARIANTARG,
    rgdispidNamedArgs: *mut DISPID,
    cArgs: UINT,
    cNamedArgs: UINT,
}}
STRUCT!{struct EXCEPINFO {
    wCode: WORD,
    wReserved: WORD,
    bstrSource: BSTR,
    bstrDescription: BSTR,
    bstrHelpFile: BSTR,
    dwHelpContext: DWORD,
    pvReserved: PVOID,
    pfnDeferredFillIn: Option<unsafe extern "system" fn(
        einfo: *mut EXCEPINFO,
    ) -> HRESULT>,
    scode: SCODE,
}}
ENUM!{enum CALLCONV {
    CC_FASTCALL = 0,
    CC_CDECL = 1,
    CC_MSCPASCAL,
    CC_PASCAL,
    CC_MACPASCAL,
    CC_STDCALL,
    CC_FPFASTCALL,
    CC_SYSCALL,
    CC_MPWCDECL,
    CC_MPWPASCAL,
    CC_MAX,
}}
ENUM!{enum FUNCKIND {
    FUNC_VIRTUAL = 0,
    FUNC_PUREVIRTUAL,
    FUNC_NONVIRTUAL,
    FUNC_STATIC,
    FUNC_DISPATCH,
}}
ENUM!{enum INVOKEKIND {
    INVOKE_FUNC = 1,
    INVOKE_PROPERTYGET = 2,
    INVOKE_PROPERTYPUT = 4,
    INVOKE_PROPERTYPUTREF = 8,
}}
STRUCT!{struct FUNCDESC {
    memid: MEMBERID,
    lprgscode: *mut SCODE,
    lprgelemdescParam: *mut ELEMDESC,
    funckind: FUNCKIND,
    invkind: INVOKEKIND,
    callconv: CALLCONV,
    cParams: SHORT,
    cParamsOpt: SHORT,
    oVft: SHORT,
    cScodes: SHORT,
    elemdescFunc: ELEMDESC,
    wFuncFlags: WORD,
}}
pub type LPFUNCDESC = *mut FUNCDESC;
ENUM!{enum VARKIND {
    VAR_PERINSTANCE = 0,
    VAR_STATIC,
    VAR_CONST,
    VAR_DISPATCH,
}}
pub const IMPLTYPEFLAG_FDEFAULT: DWORD = 0x1;
pub const IMPLTYPEFLAG_FSOURCE: DWORD = 0x2;
pub const IMPLTYPEFLAG_FRESTRICTED: DWORD = 0x4;
pub const IMPLTYPEFLAG_FDEFAULTVTABLE: DWORD = 0x8;
UNION!{union VARDESC_u {
    [usize; 1],
    oInst oInst_mut: ULONG,
    lpvarValue lpvarValue_mut: *mut VARIANT,
}}
STRUCT!{struct VARDESC {
    memid: MEMBERID,
    lpstrSchema: LPOLESTR,
    u: VARDESC_u,
    elemdescVar: ELEMDESC,
    wVarFlags: WORD,
    varkind: VARKIND,
}}
pub type LPVARDESC = *mut VARDESC;
ENUM!{enum TYPEFLAGS {
    TYPEFLAG_FAPPOBJECT = 0x1,
    TYPEFLAG_FCANCREATE = 0x2,
    TYPEFLAG_FLICENSED = 0x4,
    TYPEFLAG_FPREDECLID = 0x8,
    TYPEFLAG_FHIDDEN = 0x10,
    TYPEFLAG_FCONTROL = 0x20,
    TYPEFLAG_FDUAL = 0x40,
    TYPEFLAG_FNONEXTENSIBLE = 0x80,
    TYPEFLAG_FOLEAUTOMATION = 0x100,
    TYPEFLAG_FRESTRICTED = 0x200,
    TYPEFLAG_FAGGREGATABLE = 0x400,
    TYPEFLAG_FREPLACEABLE = 0x800,
    TYPEFLAG_FDISPATCHABLE = 0x1000,
    TYPEFLAG_FREVERSEBIND = 0x2000,
    TYPEFLAG_FPROXY = 0x4000,
}}
ENUM!{enum FUNCFLAGS {
    FUNCFLAG_FRESTRICTED = 0x1,
    FUNCFLAG_FSOURCE = 0x2,
    FUNCFLAG_FBINDABLE = 0x4,
    FUNCFLAG_FREQUESTEDIT = 0x8,
    FUNCFLAG_FDISPLAYBIND = 0x10,
    FUNCFLAG_FDEFAULTBIND = 0x20,
    FUNCFLAG_FHIDDEN = 0x40,
    FUNCFLAG_FUSESGETLASTERROR = 0x80,
    FUNCFLAG_FDEFAULTCOLLELEM = 0x100,
    FUNCFLAG_FUIDEFAULT = 0x200,
    FUNCFLAG_FNONBROWSABLE = 0x400,
    FUNCFLAG_FREPLACEABLE = 0x800,
    FUNCFLAG_FIMMEDIATEBIND = 0x1000,
}}
ENUM!{enum VARFLAGS {
    VARFLAG_FREADONLY = 0x1,
    VARFLAG_FSOURCE = 0x2,
    VARFLAG_FBINDABLE = 0x4,
    VARFLAG_FREQUESTEDIT = 0x8,
    VARFLAG_FDISPLAYBIND = 0x10,
    VARFLAG_FDEFAULTBIND = 0x20,
    VARFLAG_FHIDDEN = 0x40,
    VARFLAG_FRESTRICTED = 0x80,
    VARFLAG_FDEFAULTCOLLELEM = 0x100,
    VARFLAG_FUIDEFAULT = 0x200,
    VARFLAG_FNONBROWSABLE = 0x400,
    VARFLAG_FREPLACEABLE = 0x800,
    VARFLAG_FIMMEDIATEBIND = 0x1000,
}}
STRUCT!{struct CLEANLOCALSTORAGE {
    pInterface: *mut IUnknown,
    pStorage: PVOID,
    flags: DWORD,
}}
STRUCT!{struct CUSTDATAITEM {
    guid: GUID,
    varValue: VARIANTARG,
}}
pub type LPCUSTDATAITEM = *mut CUSTDATAITEM;
STRUCT!{struct CUSTDATA {
    cCustData: DWORD,
    prgCustData: LPCUSTDATAITEM,
}}
pub type LPCUSTDATA = *mut CUSTDATA;
// extern RPC_IF_HANDLE IOleAutomationTypes_v1_0_c_ifspec;
// extern RPC_IF_HANDLE IOleAutomationTypes_v1_0_s_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_oaidl_0000_0001_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_oaidl_0000_0001_v0_0_s_ifspec;
pub type LPCREATETYPEINFO = *mut ICreateTypeInfo;
DEFINE_GUID!{IID_ICreateTypeInfo,
    0x00020405, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x00020405, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface ICreateTypeInfo(ICreateTypeInfoVtbl): IUnknown(IUnknownVtbl) {
    fn SetGuid(
        guid: REFGUID,
    ) -> HRESULT,
    fn SetTypeFlags(
        uTypeFlags: UINT,
    ) -> HRESULT,
    fn SetDocString(
        pStrDoc: LPOLESTR,
    ) -> HRESULT,
    fn SetHelpContext(
        dwHelpContext: DWORD,
    ) -> HRESULT,
    fn SetVersion(
        wMajorVerNum: WORD,
        wMinorVerNum: WORD,
    ) -> HRESULT,
    fn AddRefTypeInfo(
        pTInfo: *mut ITypeInfo,
    ) -> HRESULT,
    fn AddFuncDesc(
        index: UINT,
        pFuncDesc: *mut FUNCDESC,
    ) -> HRESULT,
    fn SetImplTypeFlags(
        index: UINT,
        implTypeFlags: INT,
    ) -> HRESULT,
    fn SetAlignment(
        cbAlignment: WORD,
    ) -> HRESULT,
    fn SetSchema(
        pStrSchema: LPOLESTR,
    ) -> HRESULT,
    fn AddVarDesc(
        index: UINT,
        pVarDesc: *mut VARDESC,
    ) -> HRESULT,
    fn SetFuncAndParamNames(
        index: UINT,
        rgszNames: *mut LPOLESTR,
        cNames: UINT,
    ) -> HRESULT,
    fn SetVarName(
        index: UINT,
        szName: LPOLESTR,
    ) -> HRESULT,
    fn SetTypeDescAlias(
        pTDescAlias: *mut TYPEDESC,
    ) -> HRESULT,
    fn DefineFuncAsDllEntry(
        index: UINT,
        szDllName: LPOLESTR,
        szProcName: LPOLESTR,
    ) -> HRESULT,
    fn SetFuncDocString(
        index: UINT,
        szDocString: LPOLESTR,
    ) -> HRESULT,
    fn SetVarDocString(
        index: UINT,
        szDocString: LPOLESTR,
    ) -> HRESULT,
    fn SetFuncHelpContext(
        index: UINT,
        dwHelpContext: DWORD,
    ) -> HRESULT,
    fn SetVarHelpContext(
        index: UINT,
        dwHelpContext: DWORD,
    ) -> HRESULT,
    fn SetMops(
        index: UINT,
        bstrMops: BSTR,
    ) -> HRESULT,
    fn SetTypeIdldesc(
        pIdlDesc: *mut IDLDESC,
    ) -> HRESULT,
    fn LayOut() -> HRESULT,
}}
pub type LPCREATETYPEINFO2 = *mut ICreateTypeInfo2;
DEFINE_GUID!{IID_ICreateTypeInfo2,
    0x0002040e, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x0002040e, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface ICreateTypeInfo2(ICreateTypeInfo2Vtbl): ICreateTypeInfo(ICreateTypeInfoVtbl) {
    fn DeleteFuncDesc(
        index: UINT,
    ) -> HRESULT,
    fn DeleteFuncDescByMemId(
        memid: MEMBERID,
        invKind: INVOKEKIND,
    ) -> HRESULT,
    fn DeleteVarDesc(
        index: UINT,
    ) -> HRESULT,
    fn DeleteVarDescByMemId(
        memid: MEMBERID,
    ) -> HRESULT,
    fn DeleteImplType(
        index: UINT,
    ) -> HRESULT,
    fn SetCustData(
        guid: REFGUID,
        pVarVal: *mut VARIANT,
    ) -> HRESULT,
    fn SetFuncCustData(
        index: UINT,
        guid: REFGUID,
        pVarVal: *mut VARIANT,
    ) -> HRESULT,
    fn SetParamCustData(
        indexFunc: UINT,
        indexParam: UINT,
        guid: REFGUID,
        pVarVal: *mut VARIANT,
    ) -> HRESULT,
    fn SetVarCustData(
        index: UINT,
        guid: REFGUID,
        pVarVal: *mut VARIANT,
    ) -> HRESULT,
    fn SetImplTypeCustData(
        index: UINT,
        guid: REFGUID,
        pVarVal: *mut VARIANT,
    ) -> HRESULT,
    fn SetHelpStringContext(
        dwHelpStringContext: ULONG,
    ) -> HRESULT,
    fn SetFuncHelpStringContext(
        index: UINT,
        dwHelpStringContext: ULONG,
    ) -> HRESULT,
    fn SetVarHelpStringContext(
        index: UINT,
        dwHelpStringContext: ULONG,
    ) -> HRESULT,
    fn Invalidate() -> HRESULT,
    fn SetName(
        szName: LPOLESTR,
    ) -> HRESULT,
}}
pub type LPCREATETYPELIB = *mut ICreateTypeLib;
DEFINE_GUID!{IID_ICreateTypeLib,
    0x00020406, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x00020406, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface ICreateTypeLib(ICreateTypeLibVtbl): IUnknown(IUnknownVtbl) {
    fn CreateTypeInfo(
        szName: LPOLESTR,
        tkind: TYPEKIND,
        ppCTInfo: *mut *mut ICreateTypeInfo,
    ) -> HRESULT,
    fn SetName(
        szName: LPOLESTR,
    ) -> HRESULT,
    fn SetVersion(
        wMajorVerNum: WORD,
        wMinorVerNum: WORD,
    ) -> HRESULT,
    fn SetGuid(
        guid: REFGUID,
    ) -> HRESULT,
    fn SetDocString(
        szDoc: LPOLESTR,
    ) -> HRESULT,
    fn SetHelpFileName(
        szHelpFileName: LPOLESTR,
    ) -> HRESULT,
    fn SetHelpContext(
        dwHelpContext: DWORD,
    ) -> HRESULT,
    fn SetLcid(
        lcid: LCID,
    ) -> HRESULT,
    fn SetLibFlags(
        uLibFlags: UINT,
    ) -> HRESULT,
    fn SaveAllChanges() -> HRESULT,
}}
pub type LPCREATETYPELIB2 = *mut ICreateTypeLib2;
DEFINE_GUID!{IID_ICreateTypeLib2,
    0x0002040f, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x0002040f, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface ICreateTypeLib2(ICreateTypeLib2Vtbl): ICreateTypeLib(ICreateTypeLibVtbl) {
    fn DeleteTypeInfo(
        szName: LPOLESTR,
    ) -> HRESULT,
    fn SetCustData(
        guid: REFGUID,
        pVarVal: *mut VARIANT,
    ) -> HRESULT,
    fn SetHelpStringContext(
        dwHelpStringContext: ULONG,
    ) -> HRESULT,
    fn SetHelpStringDll(
        szFileName: LPOLESTR,
    ) -> HRESULT,
}}
// extern RPC_IF_HANDLE __MIDL_itf_oaidl_0000_0005_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_oaidl_0000_0005_v0_0_s_ifspec;
pub type LPDISPATCH = *mut IDispatch;
pub const DISPID_UNKNOWN: DISPID = -1;
pub const DISPID_VALUE: DISPID = 0;
pub const DISPID_PROPERTYPUT: DISPID = -3;
pub const DISPID_NEWENUM: DISPID = -4;
pub const DISPID_EVALUATE: DISPID = -5;
pub const DISPID_CONSTRUCTOR: DISPID = -6;
pub const DISPID_DESTRUCTOR: DISPID = -7;
pub const DISPID_COLLECT: DISPID = -8;
DEFINE_GUID!{IID_IDispatch,
    0x00020400, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x00020400, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IDispatch(IDispatchVtbl): IUnknown(IUnknownVtbl) {
    fn GetTypeInfoCount(
        pctinfo: *mut UINT,
    ) -> HRESULT,
    fn GetTypeInfo(
        iTInfo: UINT,
        lcid: LCID,
        ppTInfo: *mut *mut ITypeInfo,
    ) -> HRESULT,
    fn GetIDsOfNames(
        riid: REFIID,
        rgszNames: *mut LPOLESTR,
        cNames: UINT,
        lcid: LCID,
        rgDispId: *mut DISPID,
    ) -> HRESULT,
    fn Invoke(
        dispIdMember: DISPID,
        riid: REFIID,
        lcid: LCID,
        wFlags: WORD,
        pDispParams: *mut DISPPARAMS,
        pVarResult: *mut VARIANT,
        pExcepInfo: *mut EXCEPINFO,
        puArgErr: *mut UINT,
    ) -> HRESULT,
}}
// HRESULT STDMETHODCALLTYPE IDispatch_RemoteInvoke_Proxy(
//     IDispatch * This,
//     DISPID dispIdMember,
//     REFIID riid,
//     LCID lcid,
//     DWORD dwFlags,
//     DISPPARAMS *pDispParams,
//     VARIANT *pVarResult,
//     EXCEPINFO *pExcepInfo,
//     UINT *pArgErr,
//     UINT cVarRef,
//     UINT *rgVarRefIdx,
//     VARIANTARG *rgVarRef);
// void __RPC_STUB IDispatch_RemoteInvoke_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
pub type LPENUMVARIANT = *mut IEnumVARIANT;
DEFINE_GUID!{IID_IEnumVARIANT,
    0x00020404, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x00020404, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IEnumVARIANT(IEnumVARIANTVtbl): IUnknown(IUnknownVtbl) {
    fn Next(
        celt: ULONG,
        rgVar: *mut VARIANT,
        pCeltFetched: *mut ULONG,
    ) -> HRESULT,
    fn Skip(
        celt: ULONG,
    ) -> HRESULT,
    fn Reset() -> HRESULT,
    fn Clone(
        ppEnum: *mut *mut IEnumVARIANT,
    ) -> HRESULT,
}}
// HRESULT STDMETHODCALLTYPE IEnumVARIANT_RemoteNext_Proxy(
//     IEnumVARIANT * This,
//     ULONG celt,
//     VARIANT *rgVar,
//     ULONG *pCeltFetched);
// void __RPC_STUB IEnumVARIANT_RemoteNext_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
pub type LPTYPECOMP = *mut ITypeComp;
ENUM!{enum DESCKIND {
    DESCKIND_NONE = 0,
    DESCKIND_FUNCDESC,
    DESCKIND_VARDESC,
    DESCKIND_TYPECOMP,
    DESCKIND_IMPLICITAPPOBJ,
    DESCKIND_MAX,
}}
UNION!{union BINDPTR{
    [usize; 1],
    lpfuncdesc lpfuncdesc_mut: *mut FUNCDESC,
    lpvardesc lpvardesc_mut: *mut VARDESC,
    lptcomp lptcomp_mut: *mut ITypeComp,
}}
pub type LPBINDPTR = *mut BINDPTR;
DEFINE_GUID!{IID_ITypeComp,
    0x00020403, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x00020403, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface ITypeComp(ITypeCompVtbl): IUnknown(IUnknownVtbl) {
    fn Bind(
        szName: LPOLESTR,
        lHashVal: ULONG,
        wFlags: WORD,
        ppTInfo: *mut *mut ITypeInfo,
        pDescKind: *mut DESCKIND,
        pBindPtr: *mut BINDPTR,
    ) -> HRESULT,
    fn BindType(
        szName: LPOLESTR,
        lHashVal: ULONG,
        ppTInfo: *mut *mut ITypeInfo,
        ppTComp: *mut *mut ITypeComp,
    ) -> HRESULT,
}}
// HRESULT STDMETHODCALLTYPE ITypeComp_RemoteBind_Proxy(
//     ITypeComp * This,
//     LPOLESTR szName,
//     ULONG lHashVal,
//     WORD wFlags,
//     ITypeInfo **ppTInfo,
//     DESCKIND *pDescKind,
//     LPFUNCDESC *ppFuncDesc,
//     LPVARDESC *ppVarDesc,
//     ITypeComp **ppTypeComp,
//     CLEANLOCALSTORAGE *pDummy);
// void __RPC_STUB ITypeComp_RemoteBind_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
// HRESULT STDMETHODCALLTYPE ITypeComp_RemoteBindType_Proxy(
//     ITypeComp * This,
//     LPOLESTR szName,
//     ULONG lHashVal,
//     ITypeInfo **ppTInfo);
// void __RPC_STUB ITypeComp_RemoteBindType_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
// extern RPC_IF_HANDLE __MIDL_itf_oaidl_0000_0008_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_oaidl_0000_0008_v0_0_s_ifspec;
pub type LPTYPEINFO = *mut ITypeInfo;
DEFINE_GUID!{IID_ITypeInfo,
    0x00020401, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x00020401, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface ITypeInfo(ITypeInfoVtbl): IUnknown(IUnknownVtbl) {
    fn GetTypeAttr(
        ppTypeAttr: *mut *mut TYPEATTR,
    ) -> HRESULT,
    fn GetTypeComp(
        ppTComp: *mut *mut ITypeComp,
    ) -> HRESULT,
    fn GetFuncDesc(
        index: UINT,
        ppFuncDesc: *mut *mut FUNCDESC,
    ) -> HRESULT,
    fn GetVarDesc(
        index: UINT,
        ppVarDesc: *mut *mut VARDESC,
    ) -> HRESULT,
    fn GetNames(
        memid: MEMBERID,
        rgBstrNames: *mut BSTR,
        cMaxNames: UINT,
        pcNames: *mut UINT,
    ) -> HRESULT,
    fn GetRefTypeOfImplType(
        index: UINT,
        pRefType: *mut HREFTYPE,
    ) -> HRESULT,
    fn GetImplTypeFlags(
        index: UINT,
        pImplTypeFlags: *mut INT,
    ) -> HRESULT,
    fn GetIDsOfNames(
        rgszNames: *mut LPOLESTR,
        cNames: UINT,
        pMemId: *mut MEMBERID,
    ) -> HRESULT,
    fn Invoke(
        pvInstance: PVOID,
        memid: MEMBERID,
        wFlags: WORD,
        pDispParams: *mut DISPPARAMS,
        pVarResult: *mut VARIANT,
        pExcepInfo: *mut EXCEPINFO,
        puArgErr: *mut UINT,
    ) -> HRESULT,
    fn GetDocumentation(
        memid: MEMBERID,
        pBstrName: *mut BSTR,
        pBstrDocString: *mut BSTR,
        pdwHelpContext: *mut DWORD,
        pBstrHelpFile: *mut BSTR,
    ) -> HRESULT,
    fn GetDllEntry(
        memid: MEMBERID,
        invKind: INVOKEKIND,
        pBstrDllName: *mut BSTR,
        pBstrName: *mut BSTR,
        pwOrdinal: *mut WORD,
    ) -> HRESULT,
    fn GetRefTypeInfo(
        hRefType: HREFTYPE,
        ppTInfo: *mut *mut ITypeInfo,
    ) -> HRESULT,
    fn AddressOfMember(
        memid: MEMBERID,
        invKind: INVOKEKIND,
        ppv: *mut PVOID,
    ) -> HRESULT,
    fn CreateInstance(
        pUnkOuter: *mut IUnknown,
        riid: REFIID,
        ppvObj: *mut PVOID,
    ) -> HRESULT,
    fn GetMops(
        memid: MEMBERID,
        pBstrMops: *mut BSTR,
    ) -> HRESULT,
    fn GetContainingTypeLib(
        ppTLib: *mut *mut ITypeLib,
        pIndex: *mut UINT,
    ) -> HRESULT,
    fn ReleaseTypeAttr(
        pTypeAttr: *mut TYPEATTR,
    ) -> (),
    fn ReleaseFuncDesc(
        pFuncDesc: *mut FUNCDESC,
    ) -> (),
    fn ReleaseVarDesc(
        pVarDesc: *mut VARDESC,
    ) -> (),
}}
// HRESULT STDMETHODCALLTYPE ITypeInfo_RemoteGetTypeAttr_Proxy(
//     ITypeInfo * This,
//     LPTYPEATTR *ppTypeAttr,
//     CLEANLOCALSTORAGE *pDummy);
// void __RPC_STUB ITypeInfo_RemoteGetTypeAttr_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
// HRESULT STDMETHODCALLTYPE ITypeInfo_RemoteGetFuncDesc_Proxy(
//     ITypeInfo * This,
//     UINT index,
//     LPFUNCDESC *ppFuncDesc,
//     CLEANLOCALSTORAGE *pDummy);
// void __RPC_STUB ITypeInfo_RemoteGetFuncDesc_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
// HRESULT STDMETHODCALLTYPE ITypeInfo_RemoteGetVarDesc_Proxy(
//     ITypeInfo * This,
//     UINT index,
//     LPVARDESC *ppVarDesc,
//     CLEANLOCALSTORAGE *pDummy);
// void __RPC_STUB ITypeInfo_RemoteGetVarDesc_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
// HRESULT STDMETHODCALLTYPE ITypeInfo_RemoteGetNames_Proxy(
//     ITypeInfo * This,
//     MEMBERID memid,
//     BSTR *rgBstrNames,
//     UINT cMaxNames,
//     UINT *pcNames);
// void __RPC_STUB ITypeInfo_RemoteGetNames_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
// HRESULT STDMETHODCALLTYPE ITypeInfo_LocalGetIDsOfNames_Proxy(
//     ITypeInfo * This);
// void __RPC_STUB ITypeInfo_LocalGetIDsOfNames_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
// HRESULT STDMETHODCALLTYPE ITypeInfo_LocalInvoke_Proxy(
//     ITypeInfo * This);
// void __RPC_STUB ITypeInfo_LocalInvoke_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
// HRESULT STDMETHODCALLTYPE ITypeInfo_RemoteGetDocumentation_Proxy(
//     ITypeInfo * This,
//     MEMBERID memid,
//     DWORD refPtrFlags,
//     BSTR *pBstrName,
//     BSTR *pBstrDocString,
//     DWORD *pdwHelpContext,
//     BSTR *pBstrHelpFile);
// void __RPC_STUB ITypeInfo_RemoteGetDocumentation_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
// HRESULT STDMETHODCALLTYPE ITypeInfo_RemoteGetDllEntry_Proxy(
//     ITypeInfo * This,
//     MEMBERID memid,
//     INVOKEKIND invKind,
//     DWORD refPtrFlags,
//     BSTR *pBstrDllName,
//     BSTR *pBstrName,
//     WORD *pwOrdinal);
// void __RPC_STUB ITypeInfo_RemoteGetDllEntry_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
// HRESULT STDMETHODCALLTYPE ITypeInfo_LocalAddressOfMember_Proxy(
//     ITypeInfo * This);
// void __RPC_STUB ITypeInfo_LocalAddressOfMember_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
// HRESULT STDMETHODCALLTYPE ITypeInfo_RemoteCreateInstance_Proxy(
//     ITypeInfo * This,
//     REFIID riid,
//     IUnknown **ppvObj);
// void __RPC_STUB ITypeInfo_RemoteCreateInstance_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
// HRESULT STDMETHODCALLTYPE ITypeInfo_RemoteGetContainingTypeLib_Proxy(
//     ITypeInfo * This,
//     ITypeLib **ppTLib,
//     UINT *pIndex);
// void __RPC_STUB ITypeInfo_RemoteGetContainingTypeLib_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
// HRESULT STDMETHODCALLTYPE ITypeInfo_LocalReleaseTypeAttr_Proxy(
//     ITypeInfo * This);
// void __RPC_STUB ITypeInfo_LocalReleaseTypeAttr_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
// HRESULT STDMETHODCALLTYPE ITypeInfo_LocalReleaseFuncDesc_Proxy(
//     ITypeInfo * This);
// void __RPC_STUB ITypeInfo_LocalReleaseFuncDesc_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
// HRESULT STDMETHODCALLTYPE ITypeInfo_LocalReleaseVarDesc_Proxy(
//     ITypeInfo * This);
// void __RPC_STUB ITypeInfo_LocalReleaseVarDesc_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
pub type LPTYPEINFO2 = *mut ITypeInfo2;
DEFINE_GUID!{IID_ITypeInfo2,
    0x00020412, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x00020412, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface ITypeInfo2(ITypeInfo2Vtbl): ITypeInfo(ITypeInfoVtbl) {
    fn GetTypeKind(
        pTypeKind: *mut TYPEKIND,
    ) -> HRESULT,
    fn GetTypeFlags(
        pTypeFlags: *mut ULONG,
    ) -> HRESULT,
    fn GetFuncIndexOfMemId(
        memid: MEMBERID,
        invKind: INVOKEKIND,
        pFuncIndex: *mut UINT,
    ) -> HRESULT,
    fn GetVarIndexOfMemId(
        memid: MEMBERID,
        pVarIndex: *mut UINT,
    ) -> HRESULT,
    fn GetCustData(
        guid: REFGUID,
        pVarVal: *mut VARIANT,
    ) -> HRESULT,
    fn GetFuncCustData(
        index: UINT,
        guid: REFGUID,
        pVarVal: *mut VARIANT,
    ) -> HRESULT,
    fn GetParamCustData(
        indexFunc: UINT,
        indexParam: UINT,
        guid: REFGUID,
        pVarVal: *mut VARIANT,
    ) -> HRESULT,
    fn GetVarCustData(
        index: UINT,
        guid: REFGUID,
        pVarVal: *mut VARIANT,
    ) -> HRESULT,
    fn GetImplTypeCustData(
        index: UINT,
        guid: REFGUID,
        pVarVal: *mut VARIANT,
    ) -> HRESULT,
    fn GetDocumentation2(
        memid: MEMBERID,
        lcid: LCID,
        pbstrHelpString: *mut BSTR,
        pdwHelpStringContext: *mut DWORD,
        pbstrHelpStringDll: *mut BSTR,
    ) -> HRESULT,
    fn GetAllCustData(
        pCustData: *mut CUSTDATA,
    ) -> HRESULT,
    fn GetAllFuncCustData(
        index: UINT,
        pCustData: *mut CUSTDATA,
    ) -> HRESULT,
    fn GetAllParamCustData(
        indexFunc: UINT,
        indexParam: UINT,
        pCustData: *mut CUSTDATA,
    ) -> HRESULT,
    fn GetAllVarCustData(
        index: UINT,
        pCustData: *mut CUSTDATA,
    ) -> HRESULT,
    fn GetAllImplTypeCustData(
        index: UINT,
        pCustData: *mut CUSTDATA,
    ) -> HRESULT,
}}
// HRESULT STDMETHODCALLTYPE ITypeInfo2_RemoteGetDocumentation2_Proxy(
//     ITypeInfo2 * This,
//     MEMBERID memid,
//     LCID lcid,
//     DWORD refPtrFlags,
//     BSTR *pbstrHelpString,
//     DWORD *pdwHelpStringContext,
//     BSTR *pbstrHelpStringDll);
// void __RPC_STUB ITypeInfo2_RemoteGetDocumentation2_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
// extern RPC_IF_HANDLE __MIDL_itf_oaidl_0000_0010_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_oaidl_0000_0010_v0_0_s_ifspec;
ENUM!{enum SYSKIND {
    SYS_WIN16 = 0,
    SYS_WIN32,
    SYS_MAC,
    SYS_WIN64,
}}
ENUM!{enum LIBFLAGS {
    LIBFLAG_FRESTRICTED = 0x01,
    LIBFLAG_FCONTROL = 0x02,
    LIBFLAG_FHIDDEN = 0x04,
    LIBFLAG_FHASDISKIMAGE = 0x08,
}}
pub type LPTYPELIB = *mut ITypeLib;
STRUCT!{struct TLIBATTR {
    guid: GUID,
    lcid: LCID,
    syskind: SYSKIND,
    wMajorVerNum: WORD,
    wMinorVerNum: WORD,
    wLibFlags: WORD,
}}
pub type LPTLIBATTR = *mut TLIBATTR;
DEFINE_GUID!{IID_ITypeLib,
    0x00020402, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x00020402, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface ITypeLib(ITypeLibVtbl): IUnknown(IUnknownVtbl) {
    fn GetTypeInfoCount() -> UINT,
    fn GetTypeInfo(
        index: UINT,
        ppTInfo: *mut *mut ITypeInfo,
    ) -> HRESULT,
    fn GetTypeInfoType(
        index: UINT,
        pTKind: *mut TYPEKIND,
    ) -> HRESULT,
    fn GetTypeInfoOfGuid(
        guid: REFGUID,
        ppTInfo: *mut *mut ITypeInfo,
    ) -> HRESULT,
    fn GetLibAttr(
        ppTLibAttr: *mut *mut TLIBATTR,
    ) -> HRESULT,
    fn GetTypeComp(
        ppTComp: *mut *mut ITypeComp,
    ) -> HRESULT,
    fn GetDocumentation(
        index: INT,
        pbstrName: *mut BSTR,
        pBstrDocString: *mut BSTR,
        pdwHelpContext: *mut DWORD,
        pBstrHelpFile: *mut BSTR,
    ) -> HRESULT,
    fn IsName(
        szNameBuf: LPOLESTR,
        lHashVal: ULONG,
        pfName: *mut BOOL,
    ) -> HRESULT,
    fn FindName(
        szNameBuf: LPOLESTR,
        lHashVal: ULONG,
        ppTInfo: *mut *mut ITypeInfo,
        rgMemId: *mut MEMBERID,
        pcFound: *mut USHORT,
    ) -> HRESULT,
    fn ReleaseTLibAttr(
        pTLibAttr: *const TLIBATTR,
    ) -> HRESULT,
}}
// HRESULT STDMETHODCALLTYPE ITypeLib_RemoteGetTypeInfoCount_Proxy(
//     ITypeLib * This,
//     UINT *pcTInfo);
// void __RPC_STUB ITypeLib_RemoteGetTypeInfoCount_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
// HRESULT STDMETHODCALLTYPE ITypeLib_RemoteGetLibAttr_Proxy(
//     ITypeLib * This,
//     LPTLIBATTR *ppTLibAttr,
//     CLEANLOCALSTORAGE *pDummy);
// void __RPC_STUB ITypeLib_RemoteGetLibAttr_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
// HRESULT STDMETHODCALLTYPE ITypeLib_RemoteGetDocumentation_Proxy(
//     ITypeLib * This,
//     INT index,
//     DWORD refPtrFlags,
//     BSTR *pBstrName,
//     BSTR *pBstrDocString,
//     DWORD *pdwHelpContext,
//     BSTR *pBstrHelpFile);
// void __RPC_STUB ITypeLib_RemoteGetDocumentation_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
// HRESULT STDMETHODCALLTYPE ITypeLib_RemoteIsName_Proxy(
//     ITypeLib * This,
//     LPOLESTR szNameBuf,
//     ULONG lHashVal,
//     BOOL *pfName,
//     BSTR *pBstrLibName);
// void __RPC_STUB ITypeLib_RemoteIsName_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
// HRESULT STDMETHODCALLTYPE ITypeLib_RemoteFindName_Proxy(
//     ITypeLib * This,
//     LPOLESTR szNameBuf,
//     ULONG lHashVal,
//     ITypeInfo **ppTInfo,
//     MEMBERID *rgMemId,
//     USHORT *pcFound,
//     BSTR *pBstrLibName);
// void __RPC_STUB ITypeLib_RemoteFindName_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
// HRESULT STDMETHODCALLTYPE ITypeLib_LocalReleaseTLibAttr_Proxy(
//     ITypeLib * This);
// void __RPC_STUB ITypeLib_LocalReleaseTLibAttr_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
// extern RPC_IF_HANDLE __MIDL_itf_oaidl_0000_0011_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_oaidl_0000_0011_v0_0_s_ifspec;
pub type LPTYPELIB2 = *mut ITypeLib2;
DEFINE_GUID!{IID_ITypeLib2,
    0x00020411, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x00020411, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface ITypeLib2(ITypeLib2Vtbl): ITypeLib(ITypeLibVtbl) {
    fn GetCustData(
        guid: REFGUID,
        pVarVal: *mut VARIANT,
    ) -> HRESULT,
    fn GetLibStatistics(
        pcUniqueNames: *mut ULONG,
        pcchUniqueNames: *mut ULONG,
    ) -> HRESULT,
    fn GetDocumentation2(
        index: INT,
        lcid: LCID,
        pbstrHelpString: *mut BSTR,
        pdwHelpStringContext: *mut DWORD,
        pbstrHelpStringDll: *mut BSTR,
    ) -> HRESULT,
    fn GetAllCustData(
        pCustData: *mut CUSTDATA,
    ) -> HRESULT,
}}
// HRESULT STDMETHODCALLTYPE ITypeLib2_RemoteGetLibStatistics_Proxy(
//     ITypeLib2 * This,
//     ULONG *pcUniqueNames,
//     ULONG *pcchUniqueNames);
// void __RPC_STUB ITypeLib2_RemoteGetLibStatistics_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
// HRESULT STDMETHODCALLTYPE ITypeLib2_RemoteGetDocumentation2_Proxy(
//     ITypeLib2 * This,
//     INT index,
//     LCID lcid,
//     DWORD refPtrFlags,
//     BSTR *pbstrHelpString,
//     DWORD *pdwHelpStringContext,
//     BSTR *pbstrHelpStringDll);
// void __RPC_STUB ITypeLib2_RemoteGetDocumentation2_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
pub type LPTYPECHANGEEVENTS = *mut ITypeChangeEvents;
ENUM!{enum CHANGEKIND {
    CHANGEKIND_ADDMEMBER,
    CHANGEKIND_DELETEMEMBER,
    CHANGEKIND_SETNAMES,
    CHANGEKIND_SETDOCUMENTATION,
    CHANGEKIND_GENERAL,
    CHANGEKIND_INVALIDATE,
    CHANGEKIND_CHANGEFAILED,
    CHANGEKIND_MAX,
}}
DEFINE_GUID!{IID_ITypeChangeEvents,
    0x00020410, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x00020410, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface ITypeChangeEvents(ITypeChangeEventsVtbl): IUnknown(IUnknownVtbl) {
    fn RequestTypeChange(
        changeKind: CHANGEKIND,
        pTInfoBefore: *mut ITypeInfo,
        pStrName: LPOLESTR,
        pfCancel: *mut INT,
    ) -> HRESULT,
    fn AfterTypeChange(
        changeKind: CHANGEKIND,
        pTInfoAfter: *mut ITypeInfo,
        pStrName: LPOLESTR,
    ) -> HRESULT,
}}
pub type LPERRORINFO = *mut IErrorInfo;
DEFINE_GUID!{IID_IErrorInfo,
    0x1cf2b120, 0x547d, 0x101b, 0x8e, 0x65, 0x08, 0x00, 0x2b, 0x2b, 0xd1, 0x19}
RIDL!{#[uuid(0x1cf2b120, 0x547d, 0x101b, 0x8e, 0x65, 0x08, 0x00, 0x2b, 0x2b, 0xd1, 0x19)]
interface IErrorInfo(IErrorInfoVtbl): IUnknown(IUnknownVtbl) {
    fn GetGUID(
        pGUID: *mut GUID,
    ) -> HRESULT,
    fn GetSource(
        pBstrSource: *mut BSTR,
    ) -> HRESULT,
    fn GetDescription(
        pBstrDescription: *mut BSTR,
    ) -> HRESULT,
    fn GetHelpFile(
        pBstrHelpFile: *mut BSTR,
    ) -> HRESULT,
    fn GetHelpContext(
        pdwHelpContext: *mut DWORD,
    ) -> HRESULT,
}}
pub type LPCREATEERRORINFO = *mut ICreateErrorInfo;
DEFINE_GUID!{IID_ICreateErrorInfo,
    0x22f03340, 0x547d, 0x101b, 0x8e, 0x65, 0x08, 0x00, 0x2b, 0x2b, 0xd1, 0x19}
RIDL!{#[uuid(0x22f03340, 0x547d, 0x101b, 0x8e, 0x65, 0x08, 0x00, 0x2b, 0x2b, 0xd1, 0x19)]
interface ICreateErrorInfo(ICreateErrorInfoVtbl): IUnknown(IUnknownVtbl) {
    fn SetGUID(
        rguid: REFGUID,
    ) -> HRESULT,
    fn SetSource(
        szSource: LPOLESTR,
    ) -> HRESULT,
    fn SetDescription(
        szDescription: LPOLESTR,
    ) -> HRESULT,
    fn SetHelpFile(
        szHelpFile: LPOLESTR,
    ) -> HRESULT,
    fn SetHelpContext(
        dwHelpContext: DWORD,
    ) -> HRESULT,
}}
pub type LPSUPPORTERRORINFO = *mut ISupportErrorInfo;
DEFINE_GUID!{IID_ISupportErrorInfo,
    0xdf0b3d60, 0x548f, 0x101b, 0x8e, 0x65, 0x08, 0x00, 0x2b, 0x2b, 0xd1, 0x19}
RIDL!{#[uuid(0xdf0b3d60, 0x548f, 0x101b, 0x8e, 0x65, 0x08, 0x00, 0x2b, 0x2b, 0xd1, 0x19)]
interface ISupportErrorInfo(ISupportErrorInfoVtbl): IUnknown(IUnknownVtbl) {
    fn InterfaceSupportsErrorInfo(
        riid: REFIID,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_ITypeFactory,
    0x0000002e, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x0000002e, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface ITypeFactory(ITypeFactoryVtbl): IUnknown(IUnknownVtbl) {
    fn CreateFromTypeInfo(
        pTypeInfo: *mut ITypeInfo,
        riid: REFIID,
        ppv: *mut *mut IUnknown,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_ITypeMarshal,
    0x0000002d, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x0000002d, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface ITypeMarshal(ITypeMarshalVtbl): IUnknown(IUnknownVtbl) {
    fn Size(
        pvType: PVOID,
        dwDestContext: DWORD,
        pvDestContext: PVOID,
        pSize: *mut ULONG,
    ) -> HRESULT,
    fn Marshal(
        pvType: PVOID,
        dwDestContext: DWORD,
        pvDestContext: PVOID,
        cbBufferLength: ULONG,
        pBuffer: *mut BYTE,
        pcbWritten: *mut ULONG,
    ) -> HRESULT,
    fn Unmarshal(
        pvType: PVOID,
        dwFlags: DWORD,
        cbBufferLength: ULONG,
        pBuffer: *mut BYTE,
        pcbRead: *mut ULONG,
    ) -> HRESULT,
    fn Free(
        pvType: PVOID,
    ) -> HRESULT,
}}
pub type LPRECORDINFO = *mut IRecordInfo;
DEFINE_GUID!{IID_IRecordInfo,
    0x0000002f, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x0000002f, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IRecordInfo(IRecordInfoVtbl): IUnknown(IUnknownVtbl) {
    fn RecordInit(
        pvNew: PVOID,
    ) -> HRESULT,
    fn RecordClear(
        pvExisting: PVOID,
    ) -> HRESULT,
    fn RecordCopy(
        pvExisting: PVOID,
        pvNew: PVOID,
    ) -> HRESULT,
    fn GetGuid(
        pguid: *mut GUID,
    ) -> HRESULT,
    fn GetName(
        pbstrName: *mut BSTR,
    ) -> HRESULT,
    fn GetSize(
        pcbSize: *mut ULONG,
    ) -> HRESULT,
    fn GetTypeInfo(
        ppTypeInfo: *mut *mut ITypeInfo,
    ) -> HRESULT,
    fn GetField(
        pvData: PVOID,
        szFieldName: LPCOLESTR,
        pvarField: *mut VARIANT,
    ) -> HRESULT,
    fn GetFieldNoCopy(
        pvData: PVOID,
        szFieldName: LPCOLESTR,
        pvarField: *mut VARIANT,
        ppvDataCArray: *mut PVOID,
    ) -> HRESULT,
    fn PutField(
        wFlags: ULONG,
        pvData: PVOID,
        szFieldName: LPCOLESTR,
        pvarField: *mut VARIANT,
    ) -> HRESULT,
    fn PutFieldNoCopy(
        wFlags: ULONG,
        pvData: PVOID,
        szFieldName: LPCOLESTR,
        pvarField: *mut VARIANT,
    ) -> HRESULT,
    fn GetFieldNames(
        pcNames: *mut ULONG,
        rgBstrNames: *mut BSTR,
    ) -> HRESULT,
    fn IsMatchingType(
        pRecordInfo: *mut IRecordInfo,
    ) -> BOOL,
    fn RecordCreate() -> PVOID,
    fn RecordCreateCopy(
        pvSource: PVOID,
        ppvDest: *mut PVOID,
    ) -> HRESULT,
    fn RecordDestroy(
        pvRecord: PVOID,
    ) -> HRESULT,
}}
pub type LPERRORLOG = *mut IErrorLog;
DEFINE_GUID!{IID_IErrorLog,
    0x3127ca40, 0x446e, 0x11ce, 0x81, 0x35, 0x00, 0xaa, 0x00, 0x4b, 0xb8, 0x51}
RIDL!{#[uuid(0x3127ca40, 0x446e, 0x11ce, 0x81, 0x35, 0x00, 0xaa, 0x00, 0x4b, 0xb8, 0x51)]
interface IErrorLog(IErrorLogVtbl): IUnknown(IUnknownVtbl) {
    fn AddError(
        pszPropName: LPCOLESTR,
        pExcepInfo: *mut EXCEPINFO,
    ) -> HRESULT,
}}
pub type LPPROPERTYBAG = *mut IPropertyBag;
DEFINE_GUID!{IID_IPropertyBag,
    0x55272a00, 0x42cb, 0x11ce, 0x81, 0x35, 0x00, 0xaa, 0x00, 0x4b, 0xb8, 0x51}
RIDL!{#[uuid(0x55272a00, 0x42cb, 0x11ce, 0x81, 0x35, 0x00, 0xaa, 0x00, 0x4b, 0xb8, 0x51)]
interface IPropertyBag(IPropertyBagVtbl): IUnknown(IUnknownVtbl) {
    fn Read(
        pszPropName: LPCOLESTR,
        pVar: *mut VARIANT,
        pErrorLog: *mut IErrorLog,
    ) -> HRESULT,
    fn Write(
        pszPropName: LPCOLESTR,
        pVar: *mut VARIANT,
    ) -> HRESULT,
}}
// HRESULT STDMETHODCALLTYPE IPropertyBag_RemoteRead_Proxy(
//     IPropertyBag * This,
//     LPCOLESTR pszPropName,
//     VARIANT *pVar,
//     IErrorLog *pErrorLog,
//     DWORD varType,
//     IUnknown *pUnkObj);
// void __RPC_STUB IPropertyBag_RemoteRead_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
DEFINE_GUID!{IID_ITypeLibRegistrationReader,
    0xed6a8a2a, 0xb160, 0x4e77, 0x8f, 0x73, 0xaa, 0x74, 0x35, 0xcd, 0x5c, 0x27}
RIDL!{#[uuid(0xed6a8a2a, 0xb160, 0x4e77, 0x8f, 0x73, 0xaa, 0x74, 0x35, 0xcd, 0x5c, 0x27)]
interface ITypeLibRegistrationReader(ITypeLibRegistrationReaderVtbl): IUnknown(IUnknownVtbl) {
    fn EnumTypeLibRegistrations(
        ppEnumUnknown: *mut *mut IEnumUnknown,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_ITypeLibRegistration,
    0x76a3e735, 0x02df, 0x4a12, 0x98, 0xeb, 0x04, 0x3a, 0xd3, 0x60, 0x0a, 0xf3}
RIDL!{#[uuid(0x76a3e735, 0x02df, 0x4a12, 0x98, 0xeb, 0x04, 0x3a, 0xd3, 0x60, 0x0a, 0xf3)]
interface ITypeLibRegistration(ITypeLibRegistrationVtbl): IUnknown(IUnknownVtbl) {
    fn GetGuid(
        pGuid: *mut GUID,
    ) -> HRESULT,
    fn GetVersion(
        pVersion: *mut BSTR,
    ) -> HRESULT,
    fn GetLcid(
        pLcid: *mut LCID,
    ) -> HRESULT,
    fn GetWin32Path(
        pWin32Path: *mut BSTR,
    ) -> HRESULT,
    fn GetWin64Path(
        pWin64Path: *mut BSTR,
    ) -> HRESULT,
    fn GetDisplayName(
        pDisplayName: *mut BSTR,
    ) -> HRESULT,
    fn GetFlags(
        pFlags: *mut DWORD,
    ) -> HRESULT,
    fn GetHelpDir(
        pHelpDir: *mut BSTR,
    ) -> HRESULT,
}}
// EXTERN_C const CLSID CLSID_TypeLibRegistrationReader;
// extern RPC_IF_HANDLE __MIDL_itf_oaidl_0000_0023_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_oaidl_0000_0023_v0_0_s_ifspec;
// unsigned long __RPC_USER BSTR_UserSize( __RPC__in unsigned long *, unsigned long , __RPC__in BSTR * );
// unsigned char * __RPC_USER BSTR_UserMarshal( __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * );
// unsigned char * __RPC_USER BSTR_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * );
// void __RPC_USER BSTR_UserFree( __RPC__in unsigned long *, __RPC__in BSTR * );
// unsigned long __RPC_USER CLEANLOCALSTORAGE_UserSize( __RPC__in unsigned long *, unsigned long , __RPC__in CLEANLOCALSTORAGE * );
// unsigned char * __RPC_USER CLEANLOCALSTORAGE_UserMarshal( __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in CLEANLOCALSTORAGE * );
// unsigned char * __RPC_USER CLEANLOCALSTORAGE_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out CLEANLOCALSTORAGE * );
// void __RPC_USER CLEANLOCALSTORAGE_UserFree( __RPC__in unsigned long *, __RPC__in CLEANLOCALSTORAGE * );
// unsigned long __RPC_USER VARIANT_UserSize( __RPC__in unsigned long *, unsigned long , __RPC__in VARIANT * );
// unsigned char * __RPC_USER VARIANT_UserMarshal( __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * );
// unsigned char * __RPC_USER VARIANT_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * );
// void __RPC_USER VARIANT_UserFree( __RPC__in unsigned long *, __RPC__in VARIANT * );
// unsigned long __RPC_USER BSTR_UserSize64( __RPC__in unsigned long *, unsigned long , __RPC__in BSTR * );
// unsigned char * __RPC_USER BSTR_UserMarshal64( __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * );
// unsigned char * __RPC_USER BSTR_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * );
// void __RPC_USER BSTR_UserFree64( __RPC__in unsigned long *, __RPC__in BSTR * );
// unsigned long __RPC_USER CLEANLOCALSTORAGE_UserSize64( __RPC__in unsigned long *, unsigned long , __RPC__in CLEANLOCALSTORAGE * );
// unsigned char * __RPC_USER CLEANLOCALSTORAGE_UserMarshal64( __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in CLEANLOCALSTORAGE * );
// unsigned char * __RPC_USER CLEANLOCALSTORAGE_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out CLEANLOCALSTORAGE * );
// void __RPC_USER CLEANLOCALSTORAGE_UserFree64( __RPC__in unsigned long *, __RPC__in CLEANLOCALSTORAGE * );
// unsigned long __RPC_USER VARIANT_UserSize64( __RPC__in unsigned long *, unsigned long , __RPC__in VARIANT * );
// unsigned char * __RPC_USER VARIANT_UserMarshal64( __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * );
// unsigned char * __RPC_USER VARIANT_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * );
// void __RPC_USER VARIANT_UserFree64( __RPC__in unsigned long *, __RPC__in VARIANT * );
// HRESULT STDMETHODCALLTYPE IDispatch_Invoke_Proxy(
//     IDispatch * This,
//     DISPID dispIdMember,
//     REFIID riid,
//     LCID lcid,
//     WORD wFlags,
//     DISPPARAMS *pDispParams,
//     VARIANT *pVarResult,
//     EXCEPINFO *pExcepInfo,
//     UINT *puArgErr);
// HRESULT STDMETHODCALLTYPE IDispatch_Invoke_Stub(
//     IDispatch * This,
//     DISPID dispIdMember,
//     REFIID riid,
//     LCID lcid,
//     DWORD dwFlags,
//     DISPPARAMS *pDispParams,
//     VARIANT *pVarResult,
//     EXCEPINFO *pExcepInfo,
//     UINT *pArgErr,
//     UINT cVarRef,
//     UINT *rgVarRefIdx,
//     VARIANTARG *rgVarRef);
// HRESULT STDMETHODCALLTYPE IEnumVARIANT_Next_Proxy(
//     IEnumVARIANT * This,
//     ULONG celt,
//     VARIANT *rgVar,
//     ULONG *pCeltFetched);
// HRESULT STDMETHODCALLTYPE IEnumVARIANT_Next_Stub(
//     IEnumVARIANT * This,
//     ULONG celt,
//     VARIANT *rgVar,
//     ULONG *pCeltFetched);
// HRESULT STDMETHODCALLTYPE ITypeComp_Bind_Proxy(
//     ITypeComp * This,
//     LPOLESTR szName,
//     ULONG lHashVal,
//     WORD wFlags,
//     ITypeInfo **ppTInfo,
//     DESCKIND *pDescKind,
//     BINDPTR *pBindPtr);
// HRESULT STDMETHODCALLTYPE ITypeComp_Bind_Stub(
//     ITypeComp * This,
//     LPOLESTR szName,
//     ULONG lHashVal,
//     WORD wFlags,
//     ITypeInfo **ppTInfo,
//     DESCKIND *pDescKind,
//     LPFUNCDESC *ppFuncDesc,
//     LPVARDESC *ppVarDesc,
//     ITypeComp **ppTypeComp,
//     CLEANLOCALSTORAGE *pDummy);
// HRESULT STDMETHODCALLTYPE ITypeComp_BindType_Proxy(
//     ITypeComp * This,
//     LPOLESTR szName,
//     ULONG lHashVal,
//     ITypeInfo **ppTInfo,
//     ITypeComp **ppTComp);
// HRESULT STDMETHODCALLTYPE ITypeComp_BindType_Stub(
//     ITypeComp * This,
//     LPOLESTR szName,
//     ULONG lHashVal,
//     ITypeInfo **ppTInfo);
// HRESULT STDMETHODCALLTYPE ITypeInfo_GetTypeAttr_Proxy(
//     ITypeInfo * This,
//     TYPEATTR **ppTypeAttr);
// HRESULT STDMETHODCALLTYPE ITypeInfo_GetTypeAttr_Stub(
//     ITypeInfo * This,
//     LPTYPEATTR *ppTypeAttr,
//     CLEANLOCALSTORAGE *pDummy);
// HRESULT STDMETHODCALLTYPE ITypeInfo_GetFuncDesc_Proxy(
//     ITypeInfo * This,
//     UINT index,
//     FUNCDESC **ppFuncDesc);
// HRESULT STDMETHODCALLTYPE ITypeInfo_GetFuncDesc_Stub(
//     ITypeInfo * This,
//     UINT index,
//     LPFUNCDESC *ppFuncDesc,
//     CLEANLOCALSTORAGE *pDummy);
// HRESULT STDMETHODCALLTYPE ITypeInfo_GetVarDesc_Proxy(
//     ITypeInfo * This,
//     UINT index,
//     VARDESC **ppVarDesc);
// HRESULT STDMETHODCALLTYPE ITypeInfo_GetVarDesc_Stub(
//     ITypeInfo * This,
//     UINT index,
//     LPVARDESC *ppVarDesc,
//     CLEANLOCALSTORAGE *pDummy);
// HRESULT STDMETHODCALLTYPE ITypeInfo_GetNames_Proxy(
//     ITypeInfo * This,
//     MEMBERID memid,
//     BSTR *rgBstrNames,
//     UINT cMaxNames,
//     UINT *pcNames);
// HRESULT STDMETHODCALLTYPE ITypeInfo_GetNames_Stub(
//     ITypeInfo * This,
//     MEMBERID memid,
//     BSTR *rgBstrNames,
//     UINT cMaxNames,
//     UINT *pcNames);
// HRESULT STDMETHODCALLTYPE ITypeInfo_GetIDsOfNames_Proxy(
//     ITypeInfo * This,
//     LPOLESTR *rgszNames,
//     UINT cNames,
//     MEMBERID *pMemId);
// HRESULT STDMETHODCALLTYPE ITypeInfo_GetIDsOfNames_Stub(
//     ITypeInfo * This);
// HRESULT STDMETHODCALLTYPE ITypeInfo_Invoke_Proxy(
//     ITypeInfo * This,
//     PVOID pvInstance,
//     MEMBERID memid,
//     WORD wFlags,
//     DISPPARAMS *pDispParams,
//     VARIANT *pVarResult,
//     EXCEPINFO *pExcepInfo,
//     UINT *puArgErr);
// HRESULT STDMETHODCALLTYPE ITypeInfo_Invoke_Stub(
//     ITypeInfo * This);
// HRESULT STDMETHODCALLTYPE ITypeInfo_GetDocumentation_Proxy(
//     ITypeInfo * This,
//     MEMBERID memid,
//     BSTR *pBstrName,
//     BSTR *pBstrDocString,
//     DWORD *pdwHelpContext,
//     BSTR *pBstrHelpFile);
// HRESULT STDMETHODCALLTYPE ITypeInfo_GetDocumentation_Stub(
//     ITypeInfo * This,
//     MEMBERID memid,
//     DWORD refPtrFlags,
//     BSTR *pBstrName,
//     BSTR *pBstrDocString,
//     DWORD *pdwHelpContext,
//     BSTR *pBstrHelpFile);
// HRESULT STDMETHODCALLTYPE ITypeInfo_GetDllEntry_Proxy(
//     ITypeInfo * This,
//     MEMBERID memid,
//     INVOKEKIND invKind,
//     BSTR *pBstrDllName,
//     BSTR *pBstrName,
//     WORD *pwOrdinal);
// HRESULT STDMETHODCALLTYPE ITypeInfo_GetDllEntry_Stub(
//     ITypeInfo * This,
//     MEMBERID memid,
//     INVOKEKIND invKind,
//     DWORD refPtrFlags,
//     BSTR *pBstrDllName,
//     BSTR *pBstrName,
//     WORD *pwOrdinal);
// HRESULT STDMETHODCALLTYPE ITypeInfo_AddressOfMember_Proxy(
//     ITypeInfo * This,
//     MEMBERID memid,
//     INVOKEKIND invKind,
//     PVOID *ppv);
// HRESULT STDMETHODCALLTYPE ITypeInfo_AddressOfMember_Stub(
//     ITypeInfo * This);
// HRESULT STDMETHODCALLTYPE ITypeInfo_CreateInstance_Proxy(
//     ITypeInfo * This,
//     IUnknown *pUnkOuter,
//     REFIID riid,
//     PVOID *ppvObj);
// HRESULT STDMETHODCALLTYPE ITypeInfo_CreateInstance_Stub(
//     ITypeInfo * This,
//     REFIID riid,
//     IUnknown **ppvObj);
// HRESULT STDMETHODCALLTYPE ITypeInfo_GetContainingTypeLib_Proxy(
//     ITypeInfo * This,
//     ITypeLib **ppTLib,
//     UINT *pIndex);
// HRESULT STDMETHODCALLTYPE ITypeInfo_GetContainingTypeLib_Stub(
//     ITypeInfo * This,
//     ITypeLib **ppTLib,
//     UINT *pIndex);
// void STDMETHODCALLTYPE ITypeInfo_ReleaseTypeAttr_Proxy(
//     ITypeInfo * This,
//     TYPEATTR *pTypeAttr);
// HRESULT STDMETHODCALLTYPE ITypeInfo_ReleaseTypeAttr_Stub(
//     ITypeInfo * This);
// void STDMETHODCALLTYPE ITypeInfo_ReleaseFuncDesc_Proxy(
//     ITypeInfo * This,
//     FUNCDESC *pFuncDesc);
// HRESULT STDMETHODCALLTYPE ITypeInfo_ReleaseFuncDesc_Stub(
//     ITypeInfo * This);
// void STDMETHODCALLTYPE ITypeInfo_ReleaseVarDesc_Proxy(
//     ITypeInfo * This,
//     VARDESC *pVarDesc);
// HRESULT STDMETHODCALLTYPE ITypeInfo_ReleaseVarDesc_Stub(
//     ITypeInfo * This);
// HRESULT STDMETHODCALLTYPE ITypeInfo2_GetDocumentation2_Proxy(
//     ITypeInfo2 * This,
//     MEMBERID memid,
//     LCID lcid,
//     BSTR *pbstrHelpString,
//     DWORD *pdwHelpStringContext,
//     BSTR *pbstrHelpStringDll);
// HRESULT STDMETHODCALLTYPE ITypeInfo2_GetDocumentation2_Stub(
//     ITypeInfo2 * This,
//     MEMBERID memid,
//     LCID lcid,
//     DWORD refPtrFlags,
//     BSTR *pbstrHelpString,
//     DWORD *pdwHelpStringContext,
//     BSTR *pbstrHelpStringDll);
// UINT STDMETHODCALLTYPE ITypeLib_GetTypeInfoCount_Proxy(
//     ITypeLib * This);
// HRESULT STDMETHODCALLTYPE ITypeLib_GetTypeInfoCount_Stub(
//     ITypeLib * This,
//     UINT *pcTInfo);
// HRESULT STDMETHODCALLTYPE ITypeLib_GetLibAttr_Proxy(
//     ITypeLib * This,
//     TLIBATTR **ppTLibAttr);
// HRESULT STDMETHODCALLTYPE ITypeLib_GetLibAttr_Stub(
//     ITypeLib * This,
//     LPTLIBATTR *ppTLibAttr,
//     CLEANLOCALSTORAGE *pDummy);
// HRESULT STDMETHODCALLTYPE ITypeLib_GetDocumentation_Proxy(
//     ITypeLib * This,
//     INT index,
//     BSTR *pBstrName,
//     BSTR *pBstrDocString,
//     DWORD *pdwHelpContext,
//     BSTR *pBstrHelpFile);
// HRESULT STDMETHODCALLTYPE ITypeLib_GetDocumentation_Stub(
//     ITypeLib * This,
//     INT index,
//     DWORD refPtrFlags,
//     BSTR *pBstrName,
//     BSTR *pBstrDocString,
//     DWORD *pdwHelpContext,
//     BSTR *pBstrHelpFile);
// HRESULT STDMETHODCALLTYPE ITypeLib_IsName_Proxy(
//     ITypeLib * This,
//     LPOLESTR szNameBuf,
//     ULONG lHashVal,
//     BOOL *pfName);
// HRESULT STDMETHODCALLTYPE ITypeLib_IsName_Stub(
//     ITypeLib * This,
//     LPOLESTR szNameBuf,
//     ULONG lHashVal,
//     BOOL *pfName,
//     BSTR *pBstrLibName);
// HRESULT STDMETHODCALLTYPE ITypeLib_FindName_Proxy(
//     ITypeLib * This,
//     LPOLESTR szNameBuf,
//     ULONG lHashVal,
//     ITypeInfo **ppTInfo,
//     MEMBERID *rgMemId,
//     USHORT *pcFound);
// HRESULT STDMETHODCALLTYPE ITypeLib_FindName_Stub(
//     ITypeLib * This,
//     LPOLESTR szNameBuf,
//     ULONG lHashVal,
//     ITypeInfo **ppTInfo,
//     MEMBERID *rgMemId,
//     USHORT *pcFound,
//     BSTR *pBstrLibName);
// void STDMETHODCALLTYPE ITypeLib_ReleaseTLibAttr_Proxy(
//     ITypeLib * This,
//     TLIBATTR *pTLibAttr);
// HRESULT STDMETHODCALLTYPE ITypeLib_ReleaseTLibAttr_Stub(
//     ITypeLib * This);
// HRESULT STDMETHODCALLTYPE ITypeLib2_GetLibStatistics_Proxy(
//     ITypeLib2 * This,
//     ULONG *pcUniqueNames,
//     ULONG *pcchUniqueNames);
// HRESULT STDMETHODCALLTYPE ITypeLib2_GetLibStatistics_Stub(
//     ITypeLib2 * This,
//     ULONG *pcUniqueNames,
//     ULONG *pcchUniqueNames);
// HRESULT STDMETHODCALLTYPE ITypeLib2_GetDocumentation2_Proxy(
//     ITypeLib2 * This,
//     INT index,
//     LCID lcid,
//     BSTR *pbstrHelpString,
//     DWORD *pdwHelpStringContext,
//     BSTR *pbstrHelpStringDll);
// HRESULT STDMETHODCALLTYPE ITypeLib2_GetDocumentation2_Stub(
//     ITypeLib2 * This,
//     INT index,
//     LCID lcid,
//     DWORD refPtrFlags,
//     BSTR *pbstrHelpString,
//     DWORD *pdwHelpStringContext,
//     BSTR *pbstrHelpStringDll);
// HRESULT STDMETHODCALLTYPE IPropertyBag_Read_Proxy(
//     IPropertyBag * This,
//     LPCOLESTR pszPropName,
//     VARIANT *pVar,
//     IErrorLog *pErrorLog);
// HRESULT STDMETHODCALLTYPE IPropertyBag_Read_Stub(
//     IPropertyBag * This,
//     LPCOLESTR pszPropName,
//     VARIANT *pVar,
//     IErrorLog *pErrorLog,
//     DWORD varType,
//     IUnknown *pUnkObj);

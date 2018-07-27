// Copyright © 2015-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use ctypes::{c_int, c_void};
use shared::guiddef::{CLSID, GUID, REFCLSID, REFIID};
use shared::minwindef::{BOOL, DWORD, INT, UINT};
use shared::windef::{POINTL, POINTS, RECTL};
use shared::wtypes::{BSTR, PROPERTYKEY, VARTYPE};
use um::oaidl::{IPropertyBag, VARIANT};
use um::objidlbase::IStream;
use um::propidlbase::{IPropertySetStorage, PROPVARIANT, REFPROPVARIANT};
use um::propkeydef::REFPROPERTYKEY;
use um::shtypes::SHCOLSTATEF;
use um::structuredquerycondition::CONDITION_OPERATION;
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::{HRESULT, LONG, LPCWSTR, LPWSTR, PCWSTR, PWSTR, SHORT, ULONGLONG};
// extern RPC_IF_HANDLE __MIDL_itf_propsys_0000_0000_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_propsys_0000_0000_v0_0_s_ifspec;
DEFINE_GUID!{IID_IInitializeWithFile,
    0xb7d14566, 0x0509, 0x4cce, 0xa7, 0x1f, 0x0a, 0x55, 0x42, 0x33, 0xbd, 0x9b}
RIDL!{#[uuid(0xb7d14566, 0x0509, 0x4cce, 0xa7, 0x1f, 0x0a, 0x55, 0x42, 0x33, 0xbd, 0x9b)]
interface IInitializeWithFile(IInitializeWithFileVtbl): IUnknown(IUnknownVtbl) {
    fn Initialize(
        pszFilePath: LPCWSTR,
        grfMode: DWORD,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IInitializeWithStream,
    0xb824b49d, 0x22ac, 0x4161, 0xac, 0x8a, 0x99, 0x16, 0xe8, 0xfa, 0x3f, 0x7f}
RIDL!{#[uuid(0xb824b49d, 0x22ac, 0x4161, 0xac, 0x8a, 0x99, 0x16, 0xe8, 0xfa, 0x3f, 0x7f)]
interface IInitializeWithStream(IInitializeWithStreamVtbl): IUnknown(IUnknownVtbl) {
    fn Initialize(
        pstream: *mut IStream,
        grfMode: DWORD,
    ) -> HRESULT,
}}
// HRESULT STDMETHODCALLTYPE IInitializeWithStream_RemoteInitialize_Proxy(
//     IInitializeWithStream * This,
//     IStream *pstream,
//     DWORD grfMode);
// c_void __RPC_STUB IInitializeWithStream_RemoteInitialize_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
// extern RPC_IF_HANDLE __MIDL_itf_propsys_0000_0002_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_propsys_0000_0002_v0_0_s_ifspec;
DEFINE_GUID!{IID_IPropertyStore,
    0x886d8eeb, 0x8cf2, 0x4446, 0x8d, 0x02, 0xcd, 0xba, 0x1d, 0xbd, 0xcf, 0x99}
RIDL!{#[uuid(0x886d8eeb, 0x8cf2, 0x4446, 0x8d, 0x02, 0xcd, 0xba, 0x1d, 0xbd, 0xcf, 0x99)]
interface IPropertyStore(IPropertyStoreVtbl): IUnknown(IUnknownVtbl) {
    fn GetCount(
        cProps: *mut DWORD,
    ) -> HRESULT,
    fn GetAt(
        iProp: DWORD,
        pkey: *mut PROPERTYKEY,
    ) -> HRESULT,
    fn GetValue(
        key: REFPROPERTYKEY,
        pv: *mut PROPVARIANT,
    ) -> HRESULT,
    fn SetValue(
        key: REFPROPERTYKEY,
        propvar: REFPROPVARIANT,
    ) -> HRESULT,
    fn Commit() -> HRESULT,
}}
pub type LPPROPERTYSTORE = *mut IPropertyStore;
extern "system" {
    pub fn PropVariantToWinRTPropertyValue(
        propvar: REFPROPVARIANT,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT;
    pub fn WinRTPropertyValueToPropVariant(
        punkPropertyValue: *mut IUnknown,
        ppropvar: *mut PROPVARIANT,
    ) -> HRESULT;
}
// extern RPC_IF_HANDLE __MIDL_itf_propsys_0000_0003_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_propsys_0000_0003_v0_0_s_ifspec;
DEFINE_GUID!{IID_INamedPropertyStore,
    0x71604b0f, 0x97b0, 0x4764, 0x85, 0x77, 0x2f, 0x13, 0xe9, 0x8a, 0x14, 0x22}
RIDL!{#[uuid(0x71604b0f, 0x97b0, 0x4764, 0x85, 0x77, 0x2f, 0x13, 0xe9, 0x8a, 0x14, 0x22)]
interface INamedPropertyStore(INamedPropertyStoreVtbl): IUnknown(IUnknownVtbl) {
    fn GetNamedValue(
        pszName: LPCWSTR,
        ppropvar: *mut PROPVARIANT,
    ) -> HRESULT,
    fn SetNamedValue(
        pszName: LPCWSTR,
        propvar: REFPROPVARIANT,
    ) -> HRESULT,
    fn GetNameCount(
        pdwCount: *mut DWORD,
    ) -> HRESULT,
    fn GetNameAt(
        iProp: DWORD,
        pbstrName: *mut BSTR,
    ) -> HRESULT,
}}
ENUM!{enum GETPROPERTYSTOREFLAGS {
    GPS_DEFAULT = 0,
    GPS_HANDLERPROPERTIESONLY = 0x1,
    GPS_READWRITE = 0x2,
    GPS_TEMPORARY = 0x4,
    GPS_FASTPROPERTIESONLY = 0x8,
    GPS_OPENSLOWITEM = 0x10,
    GPS_DELAYCREATION = 0x20,
    GPS_BESTEFFORT = 0x40,
    GPS_NO_OPLOCK = 0x80,
    GPS_PREFERQUERYPROPERTIES = 0x100,
    GPS_EXTRINSICPROPERTIES = 0x200,
    GPS_EXTRINSICPROPERTIESONLY = 0x400,
    GPS_VOLATILEPROPERTIES = 0x800,
    GPS_VOLATILEPROPERTIESONLY = 0x1000,
    GPS_MASK_VALID = 0x1fff,
}}
// extern RPC_IF_HANDLE __MIDL_itf_propsys_0000_0004_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_propsys_0000_0004_v0_0_s_ifspec;
DEFINE_GUID!{IID_IObjectWithPropertyKey,
    0xfc0ca0a7, 0xc316, 0x4fd2, 0x90, 0x31, 0x3e, 0x62, 0x8e, 0x6d, 0x4f, 0x23}
RIDL!{#[uuid(0xfc0ca0a7, 0xc316, 0x4fd2, 0x90, 0x31, 0x3e, 0x62, 0x8e, 0x6d, 0x4f, 0x23)]
interface IObjectWithPropertyKey(IObjectWithPropertyKeyVtbl): IUnknown(IUnknownVtbl) {
    fn SetPropertyKey(
        key: REFPROPERTYKEY,
    ) -> HRESULT,
    fn GetPropertyKey(
        pkey: *mut PROPERTYKEY,
    ) -> HRESULT,
}}
ENUM!{enum PKA_FLAGS {
    PKA_SET = 0,
    PKA_APPEND = PKA_SET + 1,
    PKA_DELETE = PKA_APPEND + 1,
}}
// extern RPC_IF_HANDLE __MIDL_itf_propsys_0000_0005_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_propsys_0000_0005_v0_0_s_ifspec;
DEFINE_GUID!{IID_IPropertyChange,
    0xf917bc8a, 0x1bba, 0x4478, 0xa2, 0x45, 0x1b, 0xde, 0x03, 0xeb, 0x94, 0x31}
RIDL!{#[uuid(0xf917bc8a, 0x1bba, 0x4478, 0xa2, 0x45, 0x1b, 0xde, 0x03, 0xeb, 0x94, 0x31)]
interface IPropertyChange(IPropertyChangeVtbl): IObjectWithPropertyKey(IObjectWithPropertyKeyVtbl) {
    fn ApplyToPropVariant(
        propvarIn: REFPROPVARIANT,
        ppropvarOut: *mut PROPVARIANT,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IPropertyChangeArray,
    0x380f5cad, 0x1b5e, 0x42f2, 0x80, 0x5d, 0x63, 0x7f, 0xd3, 0x92, 0xd3, 0x1e}
RIDL!{#[uuid(0x380f5cad, 0x1b5e, 0x42f2, 0x80, 0x5d, 0x63, 0x7f, 0xd3, 0x92, 0xd3, 0x1e)]
interface IPropertyChangeArray(IPropertyChangeArrayVtbl): IUnknown(IUnknownVtbl) {
    fn GetCount(
        pcOperations: *mut UINT,
    ) -> HRESULT,
    fn GetAt(
        iIndex: UINT,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
    fn InsertAt(
        iIndex: UINT,
        ppropChange: *mut IPropertyChange,
    ) -> HRESULT,
    fn Append(
        ppropChange: *mut IPropertyChange,
    ) -> HRESULT,
    fn AppendOrReplace(
        ppropChange: *mut IPropertyChange,
    ) -> HRESULT,
    fn RemoveAt(
        iIndex: UINT,
    ) -> HRESULT,
    fn IsKeyInArray(
        key: REFPROPERTYKEY,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IPropertyStoreCapabilities,
    0xc8e2d566, 0x186e, 0x4d49, 0xbf, 0x41, 0x69, 0x09, 0xea, 0xd5, 0x6a, 0xcc}
RIDL!{#[uuid(0xc8e2d566, 0x186e, 0x4d49, 0xbf, 0x41, 0x69, 0x09, 0xea, 0xd5, 0x6a, 0xcc)]
interface IPropertyStoreCapabilities(IPropertyStoreCapabilitiesVtbl): IUnknown(IUnknownVtbl) {
    fn IsPropertyWritable(
        key: REFPROPERTYKEY,
    ) -> HRESULT,
}}
ENUM!{enum PSC_STATE {
    PSC_NORMAL = 0,
    PSC_NOTINSOURCE = 1,
    PSC_DIRTY = 2,
    PSC_READONLY = 3,
}}
DEFINE_GUID!{IID_IPropertyStoreCache,
    0x3017056d, 0x9a91, 0x4e90, 0x93, 0x7d, 0x74, 0x6c, 0x72, 0xab, 0xbf, 0x4f}
RIDL!{#[uuid(0x3017056d, 0x9a91, 0x4e90, 0x93, 0x7d, 0x74, 0x6c, 0x72, 0xab, 0xbf, 0x4f)]
interface IPropertyStoreCache(IPropertyStoreCacheVtbl): IPropertyStore(IPropertyStoreVtbl) {
    fn GetState(
        key: REFPROPERTYKEY,
        pstate: *mut PSC_STATE,
    ) -> HRESULT,
    fn GetValueAndState(
        key: REFPROPERTYKEY,
        ppropvar: *mut PROPVARIANT,
        pstate: *mut PSC_STATE,
    ) -> HRESULT,
    fn SetState(
        key: REFPROPERTYKEY,
        state: PSC_STATE,
    ) -> HRESULT,
    fn SetValueAndState(
        key: REFPROPERTYKEY,
        ppropvar: *const PROPVARIANT,
        state: PSC_STATE,
    ) -> HRESULT,
}}
ENUM!{enum PROPENUMTYPE {
    PET_DISCRETEVALUE = 0,
    PET_RANGEDVALUE = 1,
    PET_DEFAULTVALUE = 2,
    PET_ENDRANGE = 3,
}}
DEFINE_GUID!{IID_IPropertyEnumType,
    0x11e1fbf9, 0x2d56, 0x4a6b, 0x8d, 0xb3, 0x7c, 0xd1, 0x93, 0xa4, 0x71, 0xf2}
RIDL!{#[uuid(0x11e1fbf9, 0x2d56, 0x4a6b, 0x8d, 0xb3, 0x7c, 0xd1, 0x93, 0xa4, 0x71, 0xf2)]
interface IPropertyEnumType(IPropertyEnumTypeVtbl): IUnknown(IUnknownVtbl) {
    fn GetEnumType(
        penumtype: *mut PROPENUMTYPE,
    ) -> HRESULT,
    fn GetValue(
        ppropvar: *mut PROPVARIANT,
    ) -> HRESULT,
    fn GetRangeMinValue(
        ppropvarMin: *mut PROPVARIANT,
    ) -> HRESULT,
    fn GetRangeSetValue(
        ppropvarSet: *mut PROPVARIANT,
    ) -> HRESULT,
    fn GetDisplayText(
        ppszDisplay: *mut LPWSTR,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IPropertyEnumType2,
    0x9b6e051c, 0x5ddd, 0x4321, 0x90, 0x70, 0xfe, 0x2a, 0xcb, 0x55, 0xe7, 0x94}
RIDL!{#[uuid(0x9b6e051c, 0x5ddd, 0x4321, 0x90, 0x70, 0xfe, 0x2a, 0xcb, 0x55, 0xe7, 0x94)]
interface IPropertyEnumType2(IPropertyEnumType2Vtbl): IPropertyEnumType(IPropertyEnumTypeVtbl) {
    fn GetImageReference(
        ppszImageRes: *mut LPWSTR,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IPropertyEnumTypeList,
    0xa99400f4, 0x3d84, 0x4557, 0x94, 0xba, 0x12, 0x42, 0xfb, 0x2c, 0xc9, 0xa6}
RIDL!{#[uuid(0xa99400f4, 0x3d84, 0x4557, 0x94, 0xba, 0x12, 0x42, 0xfb, 0x2c, 0xc9, 0xa6)]
interface IPropertyEnumTypeList(IPropertyEnumTypeListVtbl): IUnknown(IUnknownVtbl) {
    fn GetCount(
        pctypes: *mut UINT,
    ) -> HRESULT,
    fn GetAt(
        itype: UINT,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
    fn GetConditionAt(
        nIndex: UINT,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
    fn FindMatchingIndex(
        propvarCmp: REFPROPVARIANT,
        pnIndex: *mut UINT,
    ) -> HRESULT,
}}
ENUM!{enum PROPDESC_TYPE_FLAGS {
    PDTF_DEFAULT = 0,
    PDTF_MULTIPLEVALUES = 0x1,
    PDTF_ISINNATE = 0x2,
    PDTF_ISGROUP = 0x4,
    PDTF_CANGROUPBY = 0x8,
    PDTF_CANSTACKBY = 0x10,
    PDTF_ISTREEPROPERTY = 0x20,
    PDTF_INCLUDEINFULLTEXTQUERY = 0x40,
    PDTF_ISVIEWABLE = 0x80,
    PDTF_ISQUERYABLE = 0x100,
    PDTF_CANBEPURGED = 0x200,
    PDTF_SEARCHRAWVALUE = 0x400,
    PDTF_DONTCOERCEEMPTYSTRINGS = 0x800,
    PDTF_ALWAYSINSUPPLEMENTALSTORE = 0x1000,
    PDTF_ISSYSTEMPROPERTY = 0x80000000,
    PDTF_MASK_ALL = 0x80001fff,
}}
ENUM!{enum PROPDESC_VIEW_FLAGS {
    PDVF_DEFAULT = 0,
    PDVF_CENTERALIGN = 0x1,
    PDVF_RIGHTALIGN = 0x2,
    PDVF_BEGINNEWGROUP = 0x4,
    PDVF_FILLAREA = 0x8,
    PDVF_SORTDESCENDING = 0x10,
    PDVF_SHOWONLYIFPRESENT = 0x20,
    PDVF_SHOWBYDEFAULT = 0x40,
    PDVF_SHOWINPRIMARYLIST = 0x80,
    PDVF_SHOWINSECONDARYLIST = 0x100,
    PDVF_HIDELABEL = 0x200,
    PDVF_HIDDEN = 0x800,
    PDVF_CANWRAP = 0x1000,
    PDVF_MASK_ALL = 0x1bff,
}}
ENUM!{enum PROPDESC_DISPLAYTYPE {
    PDDT_STRING = 0,
    PDDT_NUMBER = 1,
    PDDT_BOOLEAN = 2,
    PDDT_DATETIME = 3,
    PDDT_ENUMERATED = 4,
}}
ENUM!{enum PROPDESC_GROUPING_RANGE {
    PDGR_DISCRETE = 0,
    PDGR_ALPHANUMERIC = 1,
    PDGR_SIZE = 2,
    PDGR_DYNAMIC = 3,
    PDGR_DATE = 4,
    PDGR_PERCENT = 5,
    PDGR_ENUMERATED = 6,
}}
ENUM!{enum PROPDESC_FORMAT_FLAGS {
    PDFF_DEFAULT = 0,
    PDFF_PREFIXNAME = 0x1,
    PDFF_FILENAME = 0x2,
    PDFF_ALWAYSKB = 0x4,
    PDFF_RESERVED_RIGHTTOLEFT = 0x8,
    PDFF_SHORTTIME = 0x10,
    PDFF_LONGTIME = 0x20,
    PDFF_HIDETIME = 0x40,
    PDFF_SHORTDATE = 0x80,
    PDFF_LONGDATE = 0x100,
    PDFF_HIDEDATE = 0x200,
    PDFF_RELATIVEDATE = 0x400,
    PDFF_USEEDITINVITATION = 0x800,
    PDFF_READONLY = 0x1000,
    PDFF_NOAUTOREADINGORDER = 0x2000,
}}
ENUM!{enum PROPDESC_SORTDESCRIPTION {
    PDSD_GENERAL = 0,
    PDSD_A_Z = 1,
    PDSD_LOWEST_HIGHEST = 2,
    PDSD_SMALLEST_BIGGEST = 3,
    PDSD_OLDEST_NEWEST = 4,
}}
ENUM!{enum PROPDESC_RELATIVEDESCRIPTION_TYPE {
    PDRDT_GENERAL = 0,
    PDRDT_DATE = 1,
    PDRDT_SIZE = 2,
    PDRDT_COUNT = 3,
    PDRDT_REVISION = 4,
    PDRDT_LENGTH = 5,
    PDRDT_DURATION = 6,
    PDRDT_SPEED = 7,
    PDRDT_RATE = 8,
    PDRDT_RATING = 9,
    PDRDT_PRIORITY = 10,
}}
ENUM!{enum PROPDESC_AGGREGATION_TYPE {
    PDAT_DEFAULT = 0,
    PDAT_FIRST = 1,
    PDAT_SUM = 2,
    PDAT_AVERAGE = 3,
    PDAT_DATERANGE = 4,
    PDAT_UNION = 5,
    PDAT_MAX = 6,
    PDAT_MIN = 7,
}}
ENUM!{enum PROPDESC_CONDITION_TYPE {
    PDCOT_NONE = 0,
    PDCOT_STRING = 1,
    PDCOT_SIZE = 2,
    PDCOT_DATETIME = 3,
    PDCOT_BOOLEAN = 4,
    PDCOT_NUMBER = 5,
}}
DEFINE_GUID!{IID_IPropertyDescription,
    0x6f79d558, 0x3e96, 0x4549, 0xa1, 0xd1, 0x7d, 0x75, 0xd2, 0x28, 0x88, 0x14}
RIDL!{#[uuid(0x6f79d558, 0x3e96, 0x4549, 0xa1, 0xd1, 0x7d, 0x75, 0xd2, 0x28, 0x88, 0x14)]
interface IPropertyDescription(IPropertyDescriptionVtbl): IUnknown(IUnknownVtbl) {
    fn GetPropertyKey(
        pkey: *mut PROPERTYKEY,
    ) -> HRESULT,
    fn GetCanonicalName(
        ppszName: *mut LPWSTR,
    ) -> HRESULT,
    fn GetPropertyType(
        pvartype: *mut VARTYPE,
    ) -> HRESULT,
    fn GetDisplayName(
        ppszName: *mut LPWSTR,
    ) -> HRESULT,
    fn GetEditInvitation(
        ppszInvite: *mut LPWSTR,
    ) -> HRESULT,
    fn GetTypeFlags(
        mask: PROPDESC_TYPE_FLAGS,
        ppdtFlags: *mut PROPDESC_TYPE_FLAGS,
    ) -> HRESULT,
    fn GetViewFlags(
        ppdvFlags: *mut PROPDESC_VIEW_FLAGS,
    ) -> HRESULT,
    fn GetDefaultColumnWidth(
        pcxChars: *mut UINT,
    ) -> HRESULT,
    fn GetDisplayType(
        pdisplaytype: *mut PROPDESC_DISPLAYTYPE,
    ) -> HRESULT,
    fn GetColumnState(
        pcsFlags: *mut SHCOLSTATEF,
    ) -> HRESULT,
    fn GetGroupingRange(
        pgr: *mut PROPDESC_GROUPING_RANGE,
    ) -> HRESULT,
    fn GetRelativeDescriptionType(
        prdt: *mut PROPDESC_RELATIVEDESCRIPTION_TYPE,
    ) -> HRESULT,
    fn GetRelativeDescription(
        propvar1: REFPROPVARIANT,
        propvar2: REFPROPVARIANT,
        ppszDesc1: *mut LPWSTR,
        ppszDesc2: *mut LPWSTR,
    ) -> HRESULT,
    fn GetSortDescription(
        psd: *mut PROPDESC_SORTDESCRIPTION,
    ) -> HRESULT,
    fn GetSortDescriptionLabel(
        fDescending: BOOL,
        ppszDescription: *mut LPWSTR,
    ) -> HRESULT,
    fn GetAggregationType(
        paggtype: *mut PROPDESC_AGGREGATION_TYPE,
    ) -> HRESULT,
    fn GetConditionType(
        pcontype: *mut PROPDESC_CONDITION_TYPE,
        popDefault: *mut CONDITION_OPERATION,
    ) -> HRESULT,
    fn GetEnumTypeList(
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
    fn CoerceToCanonicalValue(
        ppropvar: *mut PROPVARIANT,
    ) -> HRESULT,
    fn FormatForDisplay(
        propvar: REFPROPVARIANT,
        pdfFlags: PROPDESC_FORMAT_FLAGS,
        ppszDisplay: *mut LPWSTR,
    ) -> HRESULT,
    fn IsValueCanonical(
        propvar: REFPROPVARIANT,
    ) -> HRESULT,
}}
// HRESULT STDMETHODCALLTYPE IPropertyDescription_RemoteCoerceToCanonicalValue_Proxy(
//     IPropertyDescription * This,
//     REFPROPVARIANT propvar,
//     PROPVARIANT *ppropvar);
// c_void __RPC_STUB IPropertyDescription_RemoteCoerceToCanonicalValue_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
DEFINE_GUID!{IID_IPropertyDescription2,
    0x57d2eded, 0x5062, 0x400e, 0xb1, 0x07, 0x5d, 0xae, 0x79, 0xfe, 0x57, 0xa6}
RIDL!{#[uuid(0x57d2eded, 0x5062, 0x400e, 0xb1, 0x07, 0x5d, 0xae, 0x79, 0xfe, 0x57, 0xa6)]
interface IPropertyDescription2(IPropertyDescription2Vtbl):
    IPropertyDescription(IPropertyDescriptionVtbl)
{
    fn GetImageReferenceForValue(
        propvar: REFPROPVARIANT,
        ppszImageRes: *mut LPWSTR,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IPropertyDescriptionAliasInfo,
    0xf67104fc, 0x2af9, 0x46fd, 0xb3, 0x2d, 0x24, 0x3c, 0x14, 0x04, 0xf3, 0xd1}
RIDL!{#[uuid(0xf67104fc, 0x2af9, 0x46fd, 0xb3, 0x2d, 0x24, 0x3c, 0x14, 0x04, 0xf3, 0xd1)]
interface IPropertyDescriptionAliasInfo(IPropertyDescriptionAliasInfoVtbl):
    IPropertyDescription(IPropertyDescriptionVtbl)
{
    fn GetSortByAlias(
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
    fn GetAdditionalSortByAliases(
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
}}
ENUM!{enum PROPDESC_SEARCHINFO_FLAGS {
    PDSIF_DEFAULT = 0,
    PDSIF_ININVERTEDINDEX = 0x1,
    PDSIF_ISCOLUMN = 0x2,
    PDSIF_ISCOLUMNSPARSE = 0x4,
    PDSIF_ALWAYSINCLUDE = 0x8,
    PDSIF_USEFORTYPEAHEAD = 0x10,
}}
ENUM!{enum PROPDESC_COLUMNINDEX_TYPE {
    PDCIT_NONE = 0,
    PDCIT_ONDISK = 1,
    PDCIT_INMEMORY = 2,
    PDCIT_ONDEMAND = 3,
    PDCIT_ONDISKALL = 4,
    PDCIT_ONDISKVECTOR = 5,
}}
DEFINE_GUID!{IID_IPropertyDescriptionSearchInfo,
    0x078f91bd, 0x29a2, 0x440f, 0x92, 0x4e, 0x46, 0xa2, 0x91, 0x52, 0x45, 0x20}
RIDL!{#[uuid(0x078f91bd, 0x29a2, 0x440f, 0x92, 0x4e, 0x46, 0xa2, 0x91, 0x52, 0x45, 0x20)]
interface IPropertyDescriptionSearchInfo(IPropertyDescriptionSearchInfoVtbl):
    IPropertyDescription(IPropertyDescriptionVtbl)
{
    fn GetSearchInfoFlags(
        ppdsiFlags: *mut PROPDESC_SEARCHINFO_FLAGS,
    ) -> HRESULT,
    fn GetColumnIndexType(
        ppdciType: *mut PROPDESC_COLUMNINDEX_TYPE,
    ) -> HRESULT,
    fn GetProjectionString(
        ppszProjection: *mut LPWSTR,
    ) -> HRESULT,
    fn GetMaxSize(
        pcbMaxSize: *mut UINT,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IPropertyDescriptionRelatedPropertyInfo,
    0x507393f4, 0x2a3d, 0x4a60, 0xb5, 0x9e, 0xd9, 0xc7, 0x57, 0x16, 0xc2, 0xdd}
RIDL!{#[uuid(0x507393f4, 0x2a3d, 0x4a60, 0xb5, 0x9e, 0xd9, 0xc7, 0x57, 0x16, 0xc2, 0xdd)]
interface IPropertyDescriptionRelatedPropertyInfo(IPropertyDescriptionRelatedPropertyInfoVtbl):
    IPropertyDescription(IPropertyDescriptionVtbl)
{
    fn GetRelatedProperty(
        pszRelationshipName: LPCWSTR,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
}}
ENUM!{enum PROPDESC_ENUMFILTER {
    PDEF_ALL = 0,
    PDEF_SYSTEM = 1,
    PDEF_NONSYSTEM = 2,
    PDEF_VIEWABLE = 3,
    PDEF_QUERYABLE = 4,
    PDEF_INFULLTEXTQUERY = 5,
    PDEF_COLUMN = 6,
}}
// extern RPC_IF_HANDLE __MIDL_itf_propsys_0000_0017_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_propsys_0000_0017_v0_0_s_ifspec;
DEFINE_GUID!{IID_IPropertySystem,
    0xca724e8a, 0xc3e6, 0x442b, 0x88, 0xa4, 0x6f, 0xb0, 0xdb, 0x80, 0x35, 0xa3}
RIDL!{#[uuid(0xca724e8a, 0xc3e6, 0x442b, 0x88, 0xa4, 0x6f, 0xb0, 0xdb, 0x80, 0x35, 0xa3)]
interface IPropertySystem(IPropertySystemVtbl): IUnknown(IUnknownVtbl) {
    fn GetPropertyDescription(
        propkey: REFPROPERTYKEY,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
    fn GetPropertyDescriptionByName(
        pszCanonicalName: LPCWSTR,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
    fn GetPropertyDescriptionListFromString(
        pszPropList: LPCWSTR,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
    fn EnumeratePropertyDescriptions(
        filterOn: PROPDESC_ENUMFILTER,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
    fn FormatForDisplay(
        key: REFPROPERTYKEY,
        propvar: REFPROPVARIANT,
        pdff: PROPDESC_FORMAT_FLAGS,
        pszText: LPWSTR,
        cchText: DWORD,
    ) -> HRESULT,
    fn FormatForDisplayAlloc(
        key: REFPROPERTYKEY,
        propvar: REFPROPVARIANT,
        pdff: PROPDESC_FORMAT_FLAGS,
        ppszDisplay: *mut LPWSTR,
    ) -> HRESULT,
    fn RegisterPropertySchema(
        pszPath: LPCWSTR,
    ) -> HRESULT,
    fn UnregisterPropertySchema(
        pszPath: LPCWSTR,
    ) -> HRESULT,
    fn RefreshPropertySchema() -> HRESULT,
}}
DEFINE_GUID!{IID_IPropertyDescriptionList,
    0x1f9fc1d0, 0xc39b, 0x4b26, 0x81, 0x7f, 0x01, 0x19, 0x67, 0xd3, 0x44, 0x0e}
RIDL!{#[uuid(0x1f9fc1d0, 0xc39b, 0x4b26, 0x81, 0x7f, 0x01, 0x19, 0x67, 0xd3, 0x44, 0x0e)]
interface IPropertyDescriptionList(IPropertyDescriptionListVtbl): IUnknown(IUnknownVtbl) {
    fn GetCount(
        pcElem: *mut UINT,
    ) -> HRESULT,
    fn GetAt(
        iElem: UINT,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IPropertyStoreFactory,
    0xbc110b6d, 0x57e8, 0x4148, 0xa9, 0xc6, 0x91, 0x01, 0x5a, 0xb2, 0xf3, 0xa5}
RIDL!{#[uuid(0xbc110b6d, 0x57e8, 0x4148, 0xa9, 0xc6, 0x91, 0x01, 0x5a, 0xb2, 0xf3, 0xa5)]
interface IPropertyStoreFactory(IPropertyStoreFactoryVtbl): IUnknown(IUnknownVtbl) {
    fn GetPropertyStore(
        flags: GETPROPERTYSTOREFLAGS,
        pUnkFactory: *mut IUnknown,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
    fn GetPropertyStoreForKeys(
        rgKeys: *const PROPERTYKEY,
        cKeys: UINT,
        flags: GETPROPERTYSTOREFLAGS,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IDelayedPropertyStoreFactory,
    0x40d4577f, 0xe237, 0x4bdb, 0xbd, 0x69, 0x58, 0xf0, 0x89, 0x43, 0x1b, 0x6a}
RIDL!{#[uuid(0x40d4577f, 0xe237, 0x4bdb, 0xbd, 0x69, 0x58, 0xf0, 0x89, 0x43, 0x1b, 0x6a)]
interface IDelayedPropertyStoreFactory(IDelayedPropertyStoreFactoryVtbl):
    IPropertyStoreFactory(IPropertyStoreFactoryVtbl)
{
    fn GetDelayedPropertyStore(
        flags: GETPROPERTYSTOREFLAGS,
        dwStoreId: DWORD,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
}}
ENUM!{enum _PERSIST_SPROPSTORE_FLAGS {
    FPSPS_DEFAULT = 0,
    FPSPS_READONLY = 0x1,
    FPSPS_TREAT_NEW_VALUES_AS_DIRTY = 0x2,
}}
pub type PERSIST_SPROPSTORE_FLAGS = c_int;
#[repr(C)]
pub struct SERIALIZEDPROPSTORAGE(u8);
pub type PUSERIALIZEDPROPSTORAGE = *mut SERIALIZEDPROPSTORAGE;
pub type PCUSERIALIZEDPROPSTORAGE = *const SERIALIZEDPROPSTORAGE;
// extern RPC_IF_HANDLE __MIDL_itf_propsys_0000_0021_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_propsys_0000_0021_v0_0_s_ifspec;
DEFINE_GUID!{IID_IPersistSerializedPropStorage,
    0xe318ad57, 0x0aa0, 0x450f, 0xac, 0xa5, 0x6f, 0xab, 0x71, 0x03, 0xd9, 0x17}
RIDL!{#[uuid(0xe318ad57, 0x0aa0, 0x450f, 0xac, 0xa5, 0x6f, 0xab, 0x71, 0x03, 0xd9, 0x17)]
interface IPersistSerializedPropStorage(IPersistSerializedPropStorageVtbl):
    IUnknown(IUnknownVtbl)
{
    fn SetFlags(
        flags: PERSIST_SPROPSTORE_FLAGS,
    ) -> HRESULT,
    fn SetPropertyStorage(
        psps: PCUSERIALIZEDPROPSTORAGE,
        cb: DWORD,
    ) -> HRESULT,
    fn GetPropertyStorage(
        ppsps: *mut *mut SERIALIZEDPROPSTORAGE,
        pcb: *mut DWORD,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IPersistSerializedPropStorage2,
    0x77effa68, 0x4f98, 0x4366, 0xba, 0x72, 0x57, 0x3b, 0x3d, 0x88, 0x05, 0x71}
RIDL!{#[uuid(0x77effa68, 0x4f98, 0x4366, 0xba, 0x72, 0x57, 0x3b, 0x3d, 0x88, 0x05, 0x71)]
interface IPersistSerializedPropStorage2(IPersistSerializedPropStorage2Vtbl):
    IPersistSerializedPropStorage(IPersistSerializedPropStorageVtbl)
{
    fn GetPropertyStorageSize(
        pcb: *mut DWORD,
    ) -> HRESULT,
    fn GetPropertyStorageBuffer(
        psps: *mut SERIALIZEDPROPSTORAGE,
        cb: DWORD,
        pcbWritten: *mut DWORD,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IPropertySystemChangeNotify,
    0xfa955fd9, 0x38be, 0x4879, 0xa6, 0xce, 0x82, 0x4c, 0xf5, 0x2d, 0x60, 0x9f}
RIDL!{#[uuid(0xfa955fd9, 0x38be, 0x4879, 0xa6, 0xce, 0x82, 0x4c, 0xf5, 0x2d, 0x60, 0x9f)]
interface IPropertySystemChangeNotify(IPropertySystemChangeNotifyVtbl): IUnknown(IUnknownVtbl) {
    fn SchemaRefreshed() -> HRESULT,
}}
DEFINE_GUID!{IID_ICreateObject,
    0x75121952, 0xe0d0, 0x43e5, 0x93, 0x80, 0x1d, 0x80, 0x48, 0x3a, 0xcf, 0x72}
RIDL!{#[uuid(0x75121952, 0xe0d0, 0x43e5, 0x93, 0x80, 0x1d, 0x80, 0x48, 0x3a, 0xcf, 0x72)]
interface ICreateObject(ICreateObjectVtbl): IUnknown(IUnknownVtbl) {
    fn CreateObject(
        clsid: REFCLSID,
        pUnkOuter: *mut IUnknown,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
}}
extern "system" {
    pub fn PSFormatForDisplay(
        propkey: REFPROPERTYKEY,
        propvar: REFPROPVARIANT,
        pdfFlags: PROPDESC_FORMAT_FLAGS,
        pwszText: LPWSTR,
        cchText: DWORD,
    ) -> HRESULT;
    pub fn PSFormatForDisplayAlloc(
        key: REFPROPERTYKEY,
        propvar: REFPROPVARIANT,
        pdff: PROPDESC_FORMAT_FLAGS,
        ppszDisplay: *mut PWSTR,
    ) -> HRESULT;
    pub fn PSFormatPropertyValue(
        pps: *mut IPropertyStore,
        ppd: *mut IPropertyDescription,
        pdff: PROPDESC_FORMAT_FLAGS,
        ppszDisplay: *mut LPWSTR,
    ) -> HRESULT;
    pub fn PSGetImageReferenceForValue(
        propkey: REFPROPERTYKEY,
        propvar: REFPROPVARIANT,
        ppszImageRes: *mut PWSTR,
    ) -> HRESULT;
}
pub const PKEY_PIDSTR_MAX: UINT = 10;
pub const GUIDSTRING_MAX: UINT = 1 + 8 + 1 + 4 + 1 + 4 + 1 + 4 + 1 + 12 + 1 + 1;
pub const PKEYSTR_MAX: UINT = GUIDSTRING_MAX + 1 + PKEY_PIDSTR_MAX;
extern "system" {
    pub fn PSStringFromPropertyKey(
        pkey: REFPROPERTYKEY,
        psz: LPWSTR,
        cch: UINT,
    ) -> HRESULT;
    pub fn PSPropertyKeyFromString(
        pszString: LPCWSTR,
        pkey: *mut PROPERTYKEY,
    ) -> HRESULT;
    pub fn PSCreateMemoryPropertyStore(
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT;
    pub fn PSCreateDelayedMultiplexPropertyStore(
        flags: GETPROPERTYSTOREFLAGS,
        pdpsf: *mut IDelayedPropertyStoreFactory,
        rgStoreIds: *const DWORD,
        cStores: DWORD,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT;
    pub fn PSCreateMultiplexPropertyStore(
        prgpunkStores: *mut *mut IUnknown,
        cStores: DWORD,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT;
    pub fn PSCreatePropertyChangeArray(
        rgpropkey: *const PROPERTYKEY,
        rgflags: *const PKA_FLAGS,
        rgpropvar: *const PROPVARIANT,
        cChanges: UINT,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT;
    pub fn PSCreateSimplePropertyChange(
        flags: PKA_FLAGS,
        key: REFPROPERTYKEY,
        propvar: REFPROPVARIANT,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT;
    pub fn PSGetPropertyDescription(
        propkey: REFPROPERTYKEY,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT;
    pub fn PSGetPropertyDescriptionByName(
        pszCanonicalName: LPCWSTR,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT;
    pub fn PSLookupPropertyHandlerCLSID(
        pszFilePath: PCWSTR,
        pclsid: *mut CLSID,
    ) -> HRESULT;
    pub fn PSGetItemPropertyHandler(
        punkItem: *mut IUnknown,
        fReadWrite: BOOL,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT;
    pub fn PSGetItemPropertyHandlerWithCreateObject(
        punkItem: *mut IUnknown,
        fReadWrite: BOOL,
        punkCreateObject: *mut IUnknown,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT;
    pub fn PSGetPropertyValue(
        pps: *mut IPropertyStore,
        ppd: *mut IPropertyDescription,
        ppropvar: *mut PROPVARIANT,
    ) -> HRESULT;
    pub fn PSSetPropertyValue(
        pps: *mut IPropertyStore,
        ppd: *mut IPropertyDescription,
        propvar: REFPROPVARIANT,
    ) -> HRESULT;
    pub fn PSRegisterPropertySchema(
        pszPath: PCWSTR,
    ) -> HRESULT;
    pub fn PSUnregisterPropertySchema(
        pszPath: PCWSTR,
    ) -> HRESULT;
    pub fn PSRefreshPropertySchema() -> HRESULT;
    pub fn PSEnumeratePropertyDescriptions(
        filterOn: PROPDESC_ENUMFILTER,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT;
    pub fn PSGetPropertyKeyFromName(
        pszName: PCWSTR,
        ppropkey: *mut PROPERTYKEY,
    ) -> HRESULT;
    pub fn PSGetNameFromPropertyKey(
        propkey: REFPROPERTYKEY,
        ppszCanonicalName: *mut PWSTR,
    ) -> HRESULT;
    pub fn PSCoerceToCanonicalValue(
        key: REFPROPERTYKEY,
        ppropvar: *mut PROPVARIANT,
    ) -> HRESULT;
    pub fn PSGetPropertyDescriptionListFromString(
        pszPropList: LPCWSTR,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT;
    pub fn PSCreatePropertyStoreFromPropertySetStorage(
        ppss: *mut IPropertySetStorage,
        grfMode: DWORD,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT;
    pub fn PSCreatePropertyStoreFromObject(
        punk: *mut IUnknown,
        grfMode: DWORD,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT;
    pub fn PSCreateAdapterFromPropertyStore(
        pps: *mut IPropertyStore,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT;
    pub fn PSGetPropertySystem(
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT;
    pub fn PSGetPropertyFromPropertyStorage(
        psps: PCUSERIALIZEDPROPSTORAGE,
        cb: DWORD,
        rpkey: REFPROPERTYKEY,
        ppropvar: *mut PROPVARIANT,
    ) -> HRESULT;
    pub fn PSGetNamedPropertyFromPropertyStorage(
        psps: PCUSERIALIZEDPROPSTORAGE,
        cb: DWORD,
        pszName: LPCWSTR,
        ppropvar: *mut PROPVARIANT,
    ) -> HRESULT;
    pub fn PSPropertyBag_ReadType(
        propBag: *mut IPropertyBag,
        propName: LPCWSTR,
        var: *mut VARIANT,
        type_: VARTYPE,
    ) -> HRESULT;
    pub fn PSPropertyBag_ReadStr(
        propBag: *mut IPropertyBag,
        propName: LPCWSTR,
        value: LPWSTR,
        characterCount: c_int,
    ) -> HRESULT;
    pub fn PSPropertyBag_ReadStrAlloc(
        propBag: *mut IPropertyBag,
        propName: LPCWSTR,
        value: *mut PWSTR,
    ) -> HRESULT;
    pub fn PSPropertyBag_ReadBSTR(
        propBag: *mut IPropertyBag,
        propName: LPCWSTR,
        value: *mut BSTR,
    ) -> HRESULT;
    pub fn PSPropertyBag_WriteStr(
        propBag: *mut IPropertyBag,
        propName: LPCWSTR,
        value: LPCWSTR,
    ) -> HRESULT;
    pub fn PSPropertyBag_WriteBSTR(
        propBag: *mut IPropertyBag,
        propName: LPCWSTR,
        value: BSTR,
    ) -> HRESULT;
    pub fn PSPropertyBag_ReadInt(
        propBag: *mut IPropertyBag,
        propName: LPCWSTR,
        value: *mut INT,
    ) -> HRESULT;
    pub fn PSPropertyBag_WriteInt(
        propBag: *mut IPropertyBag,
        propName: LPCWSTR,
        value: INT,
    ) -> HRESULT;
    pub fn PSPropertyBag_ReadSHORT(
        propBag: *mut IPropertyBag,
        propName: LPCWSTR,
        value: *mut SHORT,
    ) -> HRESULT;
    pub fn PSPropertyBag_WriteSHORT(
        propBag: *mut IPropertyBag,
        propName: LPCWSTR,
        value: SHORT,
    ) -> HRESULT;
    pub fn PSPropertyBag_ReadLONG(
        propBag: *mut IPropertyBag,
        propName: LPCWSTR,
        value: *mut LONG,
    ) -> HRESULT;
    pub fn PSPropertyBag_WriteLONG(
        propBag: *mut IPropertyBag,
        propName: LPCWSTR,
        value: LONG,
    ) -> HRESULT;
    pub fn PSPropertyBag_ReadDWORD(
        propBag: *mut IPropertyBag,
        propName: LPCWSTR,
        value: *mut DWORD,
    ) -> HRESULT;
    pub fn PSPropertyBag_WriteDWORD(
        propBag: *mut IPropertyBag,
        propName: LPCWSTR,
        value: DWORD,
    ) -> HRESULT;
    pub fn PSPropertyBag_ReadBOOL(
        propBag: *mut IPropertyBag,
        propName: LPCWSTR,
        value: *mut BOOL,
    ) -> HRESULT;
    pub fn PSPropertyBag_WriteBOOL(
        propBag: *mut IPropertyBag,
        propName: LPCWSTR,
        value: BOOL,
    ) -> HRESULT;
    pub fn PSPropertyBag_ReadPOINTL(
        propBag: *mut IPropertyBag,
        propName: LPCWSTR,
        value: *mut POINTL,
    ) -> HRESULT;
    pub fn PSPropertyBag_WritePOINTL(
        propBag: *mut IPropertyBag,
        propName: LPCWSTR,
        value: *const POINTL,
    ) -> HRESULT;
    pub fn PSPropertyBag_ReadPOINTS(
        propBag: *mut IPropertyBag,
        propName: LPCWSTR,
        value: *mut POINTS,
    ) -> HRESULT;
    pub fn PSPropertyBag_WritePOINTS(
        propBag: *mut IPropertyBag,
        propName: LPCWSTR,
        value: *const POINTS,
    ) -> HRESULT;
    pub fn PSPropertyBag_ReadRECTL(
        propBag: *mut IPropertyBag,
        propName: LPCWSTR,
        value: *mut RECTL,
    ) -> HRESULT;
    pub fn PSPropertyBag_WriteRECTL(
        propBag: *mut IPropertyBag,
        propName: LPCWSTR,
        value: *const RECTL,
    ) -> HRESULT;
    pub fn PSPropertyBag_ReadStream(
        propBag: *mut IPropertyBag,
        propName: LPCWSTR,
        value: *mut *mut IStream,
    ) -> HRESULT;
    pub fn PSPropertyBag_WriteStream(
        propBag: *mut IPropertyBag,
        propName: LPCWSTR,
        value: *mut IStream,
    ) -> HRESULT;
    pub fn PSPropertyBag_Delete(
        propBag: *mut IPropertyBag,
        propName: LPCWSTR,
    ) -> HRESULT;
    pub fn PSPropertyBag_ReadULONGLONG(
        propBag: *mut IPropertyBag,
        propName: LPCWSTR,
        value: *mut ULONGLONG,
    ) -> HRESULT;
    pub fn PSPropertyBag_WriteULONGLONG(
        propBag: *mut IPropertyBag,
        propName: LPCWSTR,
        value: ULONGLONG,
    ) -> HRESULT;
    pub fn PSPropertyBag_ReadUnknown(
        propBag: *mut IPropertyBag,
        propName: LPCWSTR,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT;
    pub fn PSPropertyBag_WriteUnknown(
        propBag: *mut IPropertyBag,
        propName: LPCWSTR,
        punk: *mut IUnknown,
    ) -> HRESULT;
    pub fn PSPropertyBag_ReadGUID(
        propBag: *mut IPropertyBag,
        propName: LPCWSTR,
        value: *mut GUID,
    ) -> HRESULT;
    pub fn PSPropertyBag_WriteGUID(
        propBag: *mut IPropertyBag,
        propName: LPCWSTR,
        value: *const GUID,
    ) -> HRESULT;
    pub fn PSPropertyBag_ReadPropertyKey(
        propBag: *mut IPropertyBag,
        propName: LPCWSTR,
        value: *mut PROPERTYKEY,
    ) -> HRESULT;
    pub fn PSPropertyBag_WritePropertyKey(
        propBag: *mut IPropertyBag,
        propName: LPCWSTR,
        value: REFPROPERTYKEY,
    ) -> HRESULT;
}
// extern RPC_IF_HANDLE __MIDL_itf_propsys_0000_0025_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_propsys_0000_0025_v0_0_s_ifspec;
// EXTERN_C const IID LIBID_PropSysObjects;
DEFINE_GUID!{CLSID_InMemoryPropertyStore,
    0x9a02e012, 0x6303, 0x4e1e, 0xb9, 0xa1, 0x63, 0x0f, 0x80, 0x25, 0x92, 0xc5}
// class DECLSPEC_UUID("9a02e012-6303-4e1e-b9a1-630f802592c5")
// InMemoryPropertyStore;
DEFINE_GUID!{CLSID_InMemoryPropertyStoreMarshalByValue,
    0xD4CA0E2D, 0x6DA7, 0x4b75, 0xA9, 0x7C, 0x5F, 0x30, 0x6F, 0x0E, 0xAE, 0xDC}
// class DECLSPEC_UUID("D4CA0E2D-6DA7-4b75-A97C-5F306F0EAEDC")
// InMemoryPropertyStoreMarshalByValue;
DEFINE_GUID!{CLSID_PropertySystem,
    0xb8967f85, 0x58ae, 0x4f46, 0x9f, 0xb2, 0x5d, 0x79, 0x04, 0x79, 0x8f, 0x4b}
// class DECLSPEC_UUID("b8967f85-58ae-4f46-9fb2-5d7904798f4b")
// PropertySystem;
// extern RPC_IF_HANDLE __MIDL_itf_propsys_0000_0026_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_propsys_0000_0026_v0_0_s_ifspec;
// unsigned long __RPC_USER BSTR_UserSize( __RPC__in unsigned long *, unsigned long, __RPC__in BSTR * );
// unsigned char * __RPC_USER BSTR_UserMarshal( __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * );
// unsigned char * __RPC_USER BSTR_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * );
// c_void __RPC_USER BSTR_UserFree( __RPC__in unsigned long *, __RPC__in BSTR * );
// unsigned long __RPC_USER LPSAFEARRAY_UserSize( __RPC__in unsigned long *, unsigned long, __RPC__in LPSAFEARRAY * );
// unsigned char * __RPC_USER LPSAFEARRAY_UserMarshal( __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in LPSAFEARRAY * );
// unsigned char * __RPC_USER LPSAFEARRAY_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out LPSAFEARRAY * );
// c_void __RPC_USER LPSAFEARRAY_UserFree( __RPC__in unsigned long *, __RPC__in LPSAFEARRAY * );
// unsigned long __RPC_USER BSTR_UserSize64( __RPC__in unsigned long *, unsigned long, __RPC__in BSTR * );
// unsigned char * __RPC_USER BSTR_UserMarshal64( __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * );
// unsigned char * __RPC_USER BSTR_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * );
// c_void __RPC_USER BSTR_UserFree64( __RPC__in unsigned long *, __RPC__in BSTR * );
// unsigned long __RPC_USER LPSAFEARRAY_UserSize64( __RPC__in unsigned long *, unsigned long, __RPC__in LPSAFEARRAY * );
// unsigned char * __RPC_USER LPSAFEARRAY_UserMarshal64( __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in LPSAFEARRAY * );
// unsigned char * __RPC_USER LPSAFEARRAY_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out LPSAFEARRAY * );
// c_void __RPC_USER LPSAFEARRAY_UserFree64( __RPC__in unsigned long *, __RPC__in LPSAFEARRAY * );
// HRESULT STDMETHODCALLTYPE IInitializeWithStream_Initialize_Proxy(
//     IInitializeWithStream * This,
//     IStream *pstream,
//     DWORD grfMode);
// HRESULT STDMETHODCALLTYPE IInitializeWithStream_Initialize_Stub(
//     IInitializeWithStream * This,
//     IStream *pstream,
//     DWORD grfMode);
// HRESULT STDMETHODCALLTYPE IPropertyDescription_CoerceToCanonicalValue_Proxy(
//     IPropertyDescription * This,
//     PROPVARIANT *ppropvar);
// HRESULT STDMETHODCALLTYPE IPropertyDescription_CoerceToCanonicalValue_Stub(
//     IPropertyDescription * This,
//     REFPROPVARIANT propvar,
//     PROPVARIANT *ppropvar);

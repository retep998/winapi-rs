// Copyright © 2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::guiddef::{GUID, REFCLSID, REFGUID};
use shared::minwindef::{ULONG};
use shared::wtypesbase::OLECHAR;
use um::cguid::GUID_NULL;
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::{HRESULT, LCID, LPWSTR};
pub type CATID = GUID;
pub type REFCATID = REFGUID;
// #define IID_IEnumCLSID IID_IEnumGUID
pub type IEnumCLSID = IEnumGUID;
pub type LPENUMCLSID = LPENUMGUID;
pub const CATID_NULL: CATID = GUID_NULL;
pub use shared::guiddef::IsEqualGUID as IsEqualCATID;
// #define IID_IEnumCATID IID_IEnumGUID
pub type IEnumCATID = IEnumGUID;
// EXTERN_C const CATID CATID_Insertable;
// EXTERN_C const CATID CATID_Control;
// EXTERN_C const CATID CATID_Programmable;
// EXTERN_C const CATID CATID_IsShortcut;
// EXTERN_C const CATID CATID_NeverShowExt;
// EXTERN_C const CATID CATID_DocObject;
// EXTERN_C const CATID CATID_Printable;
// EXTERN_C const CATID CATID_RequiresDataPathHost;
// EXTERN_C const CATID CATID_PersistsToMoniker;
// EXTERN_C const CATID CATID_PersistsToStorage;
// EXTERN_C const CATID CATID_PersistsToStreamInit;
// EXTERN_C const CATID CATID_PersistsToStream;
// EXTERN_C const CATID CATID_PersistsToMemory;
// EXTERN_C const CATID CATID_PersistsToFile;
// EXTERN_C const CATID CATID_PersistsToPropertyBag;
// EXTERN_C const CATID CATID_InternetAware;
// EXTERN_C const CATID CATID_DesignTimeUIActivatableControl;
// extern RPC_IF_HANDLE __MIDL_itf_comcat_0000_0000_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_comcat_0000_0000_v0_0_s_ifspec;
pub type LPENUMGUID = *mut IEnumGUID;
DEFINE_GUID!{IID_IEnumGUID,
    0x0002e000, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x0002e000, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IEnumGUID(IEnumGUIDVtbl): IUnknown(IUnknownVtbl) {
    fn Next(
        celt: ULONG,
        rgelt: *mut GUID,
        pceltFetched: *mut ULONG,
    ) -> HRESULT,
    fn Skip(
        celt: ULONG,
    ) -> HRESULT,
    fn Reset() -> HRESULT,
    fn Clone(
        ppenum: *mut *mut IEnumGUID,
    ) -> HRESULT,
}}
// HRESULT STDMETHODCALLTYPE IEnumGUID_RemoteNext_Proxy(
//     IEnumGUID * This,
//     ULONG celt,
//     GUID *rgelt,
//     ULONG *pceltFetched);
// void __RPC_STUB IEnumGUID_RemoteNext_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
// extern RPC_IF_HANDLE __MIDL_itf_comcat_0000_0001_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_comcat_0000_0001_v0_0_s_ifspec;
pub type LPENUMCATEGORYINFO = *mut IEnumCATEGORYINFO;
STRUCT!{struct CATEGORYINFO {
    catid: CATID,
    lcid: LCID,
    szDescription: [OLECHAR; 128],
}}
pub type LPCATEGORYINFO = *mut CATEGORYINFO;
DEFINE_GUID!{IID_IEnumCATEGORYINFO,
    0x0002e011, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x0002e011, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IEnumCATEGORYINFO(IEnumCATEGORYINFOVtbl): IUnknown(IUnknownVtbl) {
    fn Next(
        celt: ULONG,
        rgelt: *mut CATEGORYINFO,
        pceltFetched: *mut ULONG,
    ) -> HRESULT,
    fn Skip(
        celt: ULONG,
    ) -> HRESULT,
    fn Reset() -> HRESULT,
    fn Clone(
        ppenum: *mut *mut IEnumCATEGORYINFO,
    ) -> HRESULT,
}}
// extern RPC_IF_HANDLE __MIDL_itf_comcat_0000_0002_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_comcat_0000_0002_v0_0_s_ifspec;
pub type LPCATREGISTER = *mut ICatRegister;
DEFINE_GUID!{IID_ICatRegister,
    0x0002e012, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x0002e012, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface ICatRegister(ICatRegisterVtbl): IUnknown(IUnknownVtbl) {
    fn RegisterCategories(
        cCategories: ULONG,
        rgCategoryInfo: *mut CATEGORYINFO,
    ) -> HRESULT,
    fn UnRegisterCategories(
        cCategories: ULONG,
        rgcatid: *mut CATID,
    ) -> HRESULT,
    fn RegisterClassImplCategories(
        rclsid: REFCLSID,
        cCategories: ULONG,
        rgcatid: *mut CATID,
    ) -> HRESULT,
    fn UnRegisterClassImplCategories(
        rclsid: REFCLSID,
        cCategories: ULONG,
        rgcatid: *mut CATID,
    ) -> HRESULT,
    fn RegisterClassReqCategories(
        rclsid: REFCLSID,
        cCategories: ULONG,
        rgcatid: *mut CATID,
    ) -> HRESULT,
    fn UnRegisterClassReqCategories(
        rclsid: REFCLSID,
        cCategories: ULONG,
        rgcatid: *mut CATID,
    ) -> HRESULT,
}}
// extern RPC_IF_HANDLE __MIDL_itf_comcat_0000_0003_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_comcat_0000_0003_v0_0_s_ifspec;
pub type LPCATINFORMATION = *mut ICatInformation;
DEFINE_GUID!{IID_ICatInformation,
    0x0002e013, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x0002e013, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface ICatInformation(ICatInformationVtbl): IUnknown(IUnknownVtbl) {
    fn EnumCategories(
        lcid: LCID,
        ppenumCategoryInfo: *mut *mut IEnumCATEGORYINFO,
    ) -> HRESULT,
    fn GetCategoryDesc(
        rcatid: REFCATID,
        lcid: LCID,
        pszDesc: *mut LPWSTR,
    ) -> HRESULT,
    fn EnumClassesOfCategories(
        cImplemented: ULONG,
        rgcatidImpl: *const CATID,
        cRequired: ULONG,
        rgcatidReq: *const CATID,
        ppenumClsid: *mut *mut IEnumGUID,
    ) -> HRESULT,
    fn IsClassOfCategories(
        rclsid: REFCLSID,
        cImplemented: ULONG,
        rgcatidImpl: *const CATID,
        cRequired: ULONG,
        rgcatidReq: *const CATID,
    ) -> HRESULT,
    fn EnumImplCategoriesOfClass(
        rclsid: REFCLSID,
        ppenumCatid: *mut *mut IEnumGUID,
    ) -> HRESULT,
    fn EnumReqCategoriesOfClass(
        rclsid: REFCLSID,
        ppenumCatid: *mut *mut IEnumGUID,
    ) -> HRESULT,
}}
// HRESULT STDMETHODCALLTYPE ICatInformation_RemoteEnumClassesOfCategories_Proxy(
//     ICatInformation * This,
//     ULONG cImplemented,
//     const CATID rgcatidImpl[ ],
//     ULONG cRequired,
//     const CATID rgcatidReq[ ],
//     IEnumGUID **ppenumClsid);
// void __RPC_STUB ICatInformation_RemoteEnumClassesOfCategories_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
// HRESULT STDMETHODCALLTYPE ICatInformation_RemoteIsClassOfCategories_Proxy(
//     ICatInformation * This,
//     REFCLSID rclsid,
//     ULONG cImplemented,
//     const CATID rgcatidImpl[ ],
//     ULONG cRequired,
//     const CATID rgcatidReq[ ]);
// void __RPC_STUB ICatInformation_RemoteIsClassOfCategories_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
// extern RPC_IF_HANDLE __MIDL_itf_comcat_0000_0004_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_comcat_0000_0004_v0_0_s_ifspec;
// HRESULT STDMETHODCALLTYPE IEnumGUID_Next_Proxy(
//     IEnumGUID * This,
//     ULONG celt,
//     GUID *rgelt,
//     ULONG *pceltFetched);
// HRESULT STDMETHODCALLTYPE IEnumGUID_Next_Stub(
//     IEnumGUID * This,
//     ULONG celt,
//     GUID *rgelt,
//     ULONG *pceltFetched);
// HRESULT STDMETHODCALLTYPE ICatInformation_EnumClassesOfCategories_Proxy(
//     ICatInformation * This,
//     ULONG cImplemented,
//     const CATID rgcatidImpl[ ],
//     ULONG cRequired,
//     const CATID rgcatidReq[ ],
//     IEnumGUID **ppenumClsid);
// HRESULT STDMETHODCALLTYPE ICatInformation_EnumClassesOfCategories_Stub(
//     ICatInformation * This,
//     ULONG cImplemented,
//     const CATID rgcatidImpl[ ],
//     ULONG cRequired,
//     const CATID rgcatidReq[ ],
//     IEnumGUID **ppenumClsid);
// HRESULT STDMETHODCALLTYPE ICatInformation_IsClassOfCategories_Proxy(
//     ICatInformation * This,
//     REFCLSID rclsid,
//     ULONG cImplemented,
//     const CATID rgcatidImpl[ ],
//     ULONG cRequired,
//     const CATID rgcatidReq[ ]);
// HRESULT STDMETHODCALLTYPE ICatInformation_IsClassOfCategories_Stub(
//     ICatInformation * This,
//     REFCLSID rclsid,
//     ULONG cImplemented,
//     const CATID rgcatidImpl[ ],
//     ULONG cRequired,
//     const CATID rgcatidReq[ ]);

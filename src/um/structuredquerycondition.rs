// Copyright © 2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use ctypes::c_void;
use shared::guiddef::REFIID;
use shared::minwindef::ULONG;
use shared::wtypes::PROPERTYKEY;
use um::objidl::{IPersistStream, IPersistStreamVtbl};
use um::propidlbase::PROPVARIANT;
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::{HRESULT, LPWSTR};
ENUM!{enum CONDITION_TYPE {
    CT_AND_CONDITION = 0,
    CT_OR_CONDITION = CT_AND_CONDITION + 1,
    CT_NOT_CONDITION = CT_OR_CONDITION + 1,
    CT_LEAF_CONDITION = CT_NOT_CONDITION + 1,
}}
ENUM!{enum CONDITION_OPERATION {
    COP_IMPLICIT = 0,
    COP_EQUAL = COP_IMPLICIT + 1,
    COP_NOTEQUAL = COP_EQUAL + 1,
    COP_LESSTHAN = COP_NOTEQUAL + 1,
    COP_GREATERTHAN = COP_LESSTHAN + 1,
    COP_LESSTHANOREQUAL = COP_GREATERTHAN + 1,
    COP_GREATERTHANOREQUAL = COP_LESSTHANOREQUAL + 1,
    COP_VALUE_STARTSWITH = COP_GREATERTHANOREQUAL + 1,
    COP_VALUE_ENDSWITH = COP_VALUE_STARTSWITH + 1,
    COP_VALUE_CONTAINS = COP_VALUE_ENDSWITH + 1,
    COP_VALUE_NOTCONTAINS = COP_VALUE_CONTAINS + 1,
    COP_DOSWILDCARDS = COP_VALUE_NOTCONTAINS + 1,
    COP_WORD_EQUAL = COP_DOSWILDCARDS + 1,
    COP_WORD_STARTSWITH = COP_WORD_EQUAL + 1,
    COP_APPLICATION_SPECIFIC = COP_WORD_STARTSWITH + 1,
}}
// extern RPC_IF_HANDLE __MIDL_itf_structuredquerycondition_0000_0000_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_structuredquerycondition_0000_0000_v0_0_s_ifspec;
DEFINE_GUID!{IID_IRichChunk,
    0x4fdef69c, 0xdbc9, 0x454e, 0x99, 0x10, 0xb3, 0x4f, 0x3c, 0x64, 0xb5, 0x10}
RIDL!{#[uuid(0x4fdef69c, 0xdbc9, 0x454e, 0x99, 0x10, 0xb3, 0x4f, 0x3c, 0x64, 0xb5, 0x10)]
interface IRichChunk(IRichChunkVtbl): IUnknown(IUnknownVtbl) {
    fn GetData(
        pFirstPos: *mut ULONG,
        pLength: *mut ULONG,
        ppsz: *mut LPWSTR,
        pValue: *mut PROPVARIANT,
    ) -> HRESULT,
}}
// HRESULT STDMETHODCALLTYPE IRichChunk_RemoteGetData_Proxy(
//     IRichChunk * This,
//     ULONG *pFirstPos,
//     ULONG *pLength,
//     LPWSTR *ppsz,
//     PROPVARIANT *pValue);
// void __RPC_STUB IRichChunk_RemoteGetData_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
DEFINE_GUID!{IID_ICondition,
    0x0fc988d4, 0xc935, 0x4b97, 0xa9, 0x73, 0x46, 0x28, 0x2e, 0xa1, 0x75, 0xc8}
RIDL!{#[uuid(0x0fc988d4, 0xc935, 0x4b97, 0xa9, 0x73, 0x46, 0x28, 0x2e, 0xa1, 0x75, 0xc8)]
interface ICondition(IConditionVtbl): IPersistStream(IPersistStreamVtbl) {
    fn GetConditionType(
        pNodeType: *mut CONDITION_TYPE,
    ) -> HRESULT,
    fn GetSubConditions(
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
    fn GetComparisonInfo(
        ppszPropertyName: *mut LPWSTR,
        pcop: *mut CONDITION_OPERATION,
        ppropvar: *mut PROPVARIANT,
    ) -> HRESULT,
    fn GetValueType(
        ppszValueTypeName: *mut LPWSTR,
    ) -> HRESULT,
    fn GetValueNormalization(
        ppszNormalization: *mut LPWSTR,
    ) -> HRESULT,
    fn GetInputTerms(
        ppPropertyTerm: *mut *mut IRichChunk,
        ppOperationTerm: *mut *mut IRichChunk,
        ppValueTerm: *mut *mut IRichChunk,
    ) -> HRESULT,
    fn Clone(
        ppc: *mut *mut ICondition,
    ) -> HRESULT,
}}
// HRESULT STDMETHODCALLTYPE ICondition_RemoteGetComparisonInfo_Proxy(
//     ICondition * This,
//     LPWSTR *ppszPropertyName,
//     CONDITION_OPERATION *pcop,
//     PROPVARIANT *ppropvar);
// void __RPC_STUB ICondition_RemoteGetComparisonInfo_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
// HRESULT STDMETHODCALLTYPE ICondition_RemoteGetInputTerms_Proxy(
//     ICondition * This,
//     IRichChunk **ppPropertyTerm,
//     IRichChunk **ppOperationTerm,
//     IRichChunk **ppValueTerm);
// void __RPC_STUB ICondition_RemoteGetInputTerms_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
DEFINE_GUID!{IID_ICondition2,
    0x0db8851d, 0x2e5b, 0x47eb, 0x92, 0x08, 0xd2, 0x8c, 0x32, 0x5a, 0x01, 0xd7}
RIDL!{#[uuid(0x0db8851d, 0x2e5b, 0x47eb, 0x92, 0x08, 0xd2, 0x8c, 0x32, 0x5a, 0x01, 0xd7)]
interface ICondition2(ICondition2Vtbl): ICondition(IConditionVtbl) {
    fn GetLocale(
        ppszLocaleName: *mut LPWSTR,
    ) -> HRESULT,
    fn GetLeafConditionInfo(
        ppropkey: *mut PROPERTYKEY,
        pcop: *mut CONDITION_OPERATION,
        ppropvar: *mut PROPVARIANT,
    ) -> HRESULT,
}}
// HRESULT STDMETHODCALLTYPE ICondition2_RemoteGetLeafConditionInfo_Proxy(
//     ICondition2 * This,
//     PROPERTYKEY *ppropkey,
//     CONDITION_OPERATION *pcop,
//     PROPVARIANT *ppropvar);
// void __RPC_STUB ICondition2_RemoteGetLeafConditionInfo_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
// extern RPC_IF_HANDLE __MIDL_itf_structuredquerycondition_0000_0003_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_structuredquerycondition_0000_0003_v0_0_s_ifspec;
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
// HRESULT STDMETHODCALLTYPE IRichChunk_GetData_Proxy(
//     IRichChunk * This,
//     ULONG *pFirstPos,
//     ULONG *pLength,
//     LPWSTR *ppsz,
//     PROPVARIANT *pValue);
// HRESULT STDMETHODCALLTYPE IRichChunk_GetData_Stub(
//     IRichChunk * This,
//     ULONG *pFirstPos,
//     ULONG *pLength,
//     LPWSTR *ppsz,
//     PROPVARIANT *pValue);
// HRESULT STDMETHODCALLTYPE ICondition_GetComparisonInfo_Proxy(
//     ICondition * This,
//     LPWSTR *ppszPropertyName,
//     CONDITION_OPERATION *pcop,
//     PROPVARIANT *ppropvar);
// HRESULT STDMETHODCALLTYPE ICondition_GetComparisonInfo_Stub(
//     ICondition * This,
//     LPWSTR *ppszPropertyName,
//     CONDITION_OPERATION *pcop,
//     PROPVARIANT *ppropvar);
// HRESULT STDMETHODCALLTYPE ICondition_GetInputTerms_Proxy(
//     ICondition * This,
//     IRichChunk **ppPropertyTerm,
//     IRichChunk **ppOperationTerm,
//     IRichChunk **ppValueTerm);
// HRESULT STDMETHODCALLTYPE ICondition_GetInputTerms_Stub(
//     ICondition * This,
//     IRichChunk **ppPropertyTerm,
//     IRichChunk **ppOperationTerm,
//     IRichChunk **ppValueTerm);
// HRESULT STDMETHODCALLTYPE ICondition2_GetLeafConditionInfo_Proxy(
//     ICondition2 * This,
//     PROPERTYKEY *ppropkey,
//     CONDITION_OPERATION *pcop,
//     PROPVARIANT *ppropvar);
// HRESULT STDMETHODCALLTYPE ICondition2_GetLeafConditionInfo_Stub(
//     ICondition2 * This,
//     PROPERTYKEY *ppropkey,
//     CONDITION_OPERATION *pcop,
//     PROPVARIANT *ppropvar);

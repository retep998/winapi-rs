// Copyright © 2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use ctypes::c_void;
use shared::guiddef::REFIID;
use shared::minwindef::UINT;
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::HRESULT;
// extern RPC_IF_HANDLE __MIDL_itf_objectarray_0000_0000_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_objectarray_0000_0000_v0_0_s_ifspec;
DEFINE_GUID!{IID_IObjectArray,
    0x92ca9dcd, 0x5622, 0x4bba, 0xa8, 0x05, 0x5e, 0x9f, 0x54, 0x1b, 0xd8, 0xc9}
RIDL!{#[uuid(0x92ca9dcd, 0x5622, 0x4bba, 0xa8, 0x05, 0x5e, 0x9f, 0x54, 0x1b, 0xd8, 0xc9)]
interface IObjectArray(IObjectArrayVtbl): IUnknown(IUnknownVtbl) {
    fn GetCount(
        pcObjects: *mut UINT,
    ) -> HRESULT,
    fn GetAt(
        uiIndex: UINT,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IObjectCollection,
    0x5632b1a4, 0xe38a, 0x400a, 0x92, 0x8a, 0xd4, 0xcd, 0x63, 0x23, 0x02, 0x95}
RIDL!{#[uuid(0x5632b1a4, 0xe38a, 0x400a, 0x92, 0x8a, 0xd4, 0xcd, 0x63, 0x23, 0x02, 0x95)]
interface IObjectCollection(IObjectCollectionVtbl): IObjectArray(IObjectArrayVtbl) {
    fn AddObject(
        punk: *mut IUnknown,
    ) -> HRESULT,
    fn AddFromArray(
        poaSource: *mut IObjectArray,
    ) -> HRESULT,
    fn RemoveObjectAt(
        uiIndex: UINT,
    ) -> HRESULT,
    fn Clear() -> HRESULT,
}}
// extern RPC_IF_HANDLE __MIDL_itf_objectarray_0000_0002_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_objectarray_0000_0002_v0_0_s_ifspec;

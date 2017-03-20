// Copyright © 2015-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms
//! this ALWAYS GENERATED file contains the definitions for the interfaces
use shared::guiddef::REFIID;
use shared::minwindef::{DWORD, LPVOID, UINT};
use um::propidl::PROPVARIANT;
use um::propsys::IPropertyStore;
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::{HRESULT, LPCWSTR, LPWSTR};
pub const DEVICE_STATE_ACTIVE: DWORD = 0x00000001;
pub const DEVICE_STATE_DISABLED: DWORD = 0x00000002;
pub const DEVICE_STATE_NOTPRESENT: DWORD = 0x00000004;
pub const DEVICE_STATE_UNPLUGGED: DWORD = 0x00000008;
pub const DEVICE_STATEMASK_ALL: DWORD = 0x0000000F;
ENUM!{enum EDataFlow {
    eRender,
    eCapture,
    eAll,
    EDataFlow_enum_count,
}}
ENUM!{enum ERole {
    eConsole,
    eMultimedia,
    eCommunications,
    ERole_enum_count,
}}
DEFINE_GUID!(CLSID_MMDeviceEnumerator, 0xBCDE0395, 0xE52F, 0x467C,
    0x8E, 0x3D, 0xC4, 0x57, 0x92, 0x91, 0x69, 0x2E);
DEFINE_GUID!(IID_IMMDeviceEnumerator, 0xA95664D2, 0x9614, 0x4F35,
    0xA7, 0x46, 0xDE, 0x8D, 0xB6, 0x36, 0x17, 0xE6);
RIDL!(#[uuid(0xd666063f, 0x1587, 0x4e43, 0x81, 0xf1, 0xb9, 0x48, 0xe8, 0x07, 0x36, 0x3f)]
interface IMMDevice(IMMDeviceVtbl): IUnknown(IUnknownVtbl) {
    fn Activate(
        iid: REFIID, dwClsCtx: DWORD, pActivationParams: *mut PROPVARIANT,
        ppInterface: *mut LPVOID
    ) -> HRESULT,
    fn OpenPropertyStore(
        stgmAccess: DWORD, ppProperties: *mut *mut IPropertyStore
    ) -> HRESULT,
    fn GetId(ppstrId: *mut LPWSTR) -> HRESULT,
    fn GetState(pdwState: *mut DWORD) -> HRESULT
}
);
RIDL!(#[uuid(0xa95664d2, 0x9614, 0x4f35, 0xa7, 0x46, 0xde, 0x8d, 0xb6, 0x36, 0x17, 0xe6)]
interface IMMDeviceEnumerator(IMMDeviceEnumeratorVtbl): IUnknown(IUnknownVtbl) {
    fn EnumAudioEndpoints(
        dataFlow: EDataFlow, dwStateMask: DWORD,
        ppDevices: *mut *mut IMMDeviceCollection
    ) -> HRESULT,
    fn GetDefaultAudioEndpoint(
        dataFlow: EDataFlow, role: ERole, ppEndpoint: *mut *mut IMMDevice
    ) -> HRESULT,
    fn GetDevice(pwstrId: LPCWSTR, ppDevices: *mut *mut IMMDevice) -> HRESULT,
    fn RegisterEndpointNotificationCallback(
        pClient: *mut IMMNotificationClient
    ) -> HRESULT,
    fn UnregisterEndpointNotificationCallback(
        pClient: *mut IMMNotificationClient
    ) -> HRESULT
}
);
RIDL!(#[uuid(0x0bd7a1be, 0x7a1a, 0x44db, 0x83, 0x97, 0xcc, 0x53, 0x92, 0x38, 0x7b, 0x5e)]
interface IMMDeviceCollection(IMMDeviceCollectionVtbl): IUnknown(IUnknownVtbl) {
    fn GetCount(pcDevices: *const UINT) -> HRESULT,
    fn Item(nDevice: UINT, ppDevice: *mut *mut IMMDevice) -> HRESULT
}
);
pub enum IMMNotificationClient {} // FIXME

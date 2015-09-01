// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! this ALWAYS GENERATED file contains the definitions for the interfaces
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum EDataFlow {
    eRender,
    eCapture,
    eAll,
    EDataFlow_enum_count,
}
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum ERole {
    eConsole,
    eMultimedia,
    eCommunications,
    ERole_enum_count,
}
DEFINE_GUID!(CLSID_MMDeviceEnumerator, 0xBCDE0395, 0xE52F, 0x467C,
    0x8E, 0x3D, 0xC4, 0x57, 0x92, 0x91, 0x69, 0x2E);
DEFINE_GUID!(IID_IMMDeviceEnumerator, 0xA95664D2, 0x9614, 0x4F35,
    0xA7, 0x46, 0xDE, 0x8D, 0xB6, 0x36, 0x17, 0xE6);
RIDL!(
interface IMMDevice(IMMDeviceVtbl): IUnknown(IUnknownVtbl) {
    fn Activate(
        &mut self, iid: ::REFIID, dwClsCtx: ::DWORD, pActivationParams: *mut ::PROPVARIANT,
        ppInterface: *mut ::LPVOID
    ) -> ::HRESULT,
    fn OpenPropertyStore(
        &mut self, stgmAccess: ::DWORD, ppProperties: *mut *mut ::IPropertyStore
    ) -> ::HRESULT,
    fn GetId(&mut self, ppstrId: *mut ::LPWSTR) -> ::HRESULT,
    fn GetState(&mut self, pdwState: *mut ::DWORD) -> ::HRESULT
}
);
RIDL!(
interface IMMDeviceEnumerator(IMMDeviceEnumeratorVtbl): IUnknown(IUnknownVtbl) {
    fn EnumAudioEndpoints(
        &mut self, dataFlow: EDataFlow, dwStateMask: ::DWORD,
        ppDevices: *mut *mut IMMDeviceCollection
    ) -> ::HRESULT,
    fn GetDefaultAudioEndpoint(
        &mut self, dataFlow: EDataFlow, role: ERole, ppEndpoint: *mut *mut IMMDevice
    ) -> ::HRESULT,
    fn GetDevice(&mut self, pwstrId: ::LPCWSTR, ppDevices: *mut *mut IMMDevice) -> ::HRESULT,
    fn RegisterEndpointNotificationCallback(
        &mut self, pClient: *mut IMMNotificationClient
    ) -> ::HRESULT,
    fn UnregisterEndpointNotificationCallback(
        &mut self, pClient: *mut IMMNotificationClient
    ) -> ::HRESULT
}
);
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IMMDeviceCollection;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IMMNotificationClient;

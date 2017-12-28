// Copyright © 2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::guiddef::REFIID;
use shared::minwindef::DWORD;
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::HRESULT;
pub const INTERFACESAFE_FOR_UNTRUSTED_CALLER: DWORD = 0x00000001;
pub const INTERFACESAFE_FOR_UNTRUSTED_DATA: DWORD = 0x00000002;
pub const INTERFACE_USES_DISPEX: DWORD = 0x00000004;
pub const INTERFACE_USES_SECURITY_MANAGER: DWORD = 0x00000008;
DEFINE_GUID!{IID_IObjectSafety,
    0xcb5bdc81, 0x93c1, 0x11cf, 0x8f, 0x20, 0x0, 0x80, 0x5f, 0x2c, 0xd0, 0x64}
// EXTERN_C GUID CATID_SafeForScripting;
// EXTERN_C GUID CATID_SafeForInitializing;
// extern RPC_IF_HANDLE __MIDL_itf_objsafe_0000_0000_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_objsafe_0000_0000_v0_0_s_ifspec;
RIDL!{#[uuid(0xcb5bdc81, 0x93c1, 0x11cf, 0x8f, 0x20, 0x00, 0x80, 0x5f, 0x2c, 0xd0, 0x64)]
interface IObjectSafety(IObjectSafetyVtbl): IUnknown(IUnknownVtbl) {
    fn GetInterfaceSafetyOptions(
        riid: REFIID,
        pdwSupportedOptions: *mut DWORD,
        pdwEnabledOptions: *mut DWORD,
    ) -> HRESULT,
    fn SetInterfaceSafetyOptions(
        riid: REFIID,
        dwOptionSetMask: DWORD,
        dwEnabledOptions: DWORD,
    ) -> HRESULT,
}}
pub type LPOBJECTSAFETY = *mut IObjectSafety;
// extern RPC_IF_HANDLE __MIDL_itf_objsafe_0000_0001_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_objsafe_0000_0001_v0_0_s_ifspec;

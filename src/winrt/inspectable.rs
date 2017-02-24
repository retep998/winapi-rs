// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
use shared::guiddef::{IID};
use shared::minwindef::{ULONG};
use um::unknwnbase::{ IUnknown, IUnknownVtbl };
use um::winnt::{HRESULT};
use winrt::hstring::{HSTRING};

pub type LPINSPECTABLE = *mut IInspectable;
ENUM!{enum TrustLevel {
    BaseTrust = 0,
    PartialTrust,
    FullTrust,
}}
RIDL!{#[uuid(0xaf86e2e0, 0xb12d, 0x4c6a, 0x9c, 0x5a, 0xd7, 0xaa, 0x65, 0x10, 0x1e, 0x90)]
interface IInspectable(IInspectableVtbl): IUnknown(IUnknownVtbl) {
    fn GetIids(iidCount: *mut  ULONG, iids: *mut *mut  IID) ->  HRESULT,
    fn GetRuntimeClassName(className: *mut  HSTRING) ->  HRESULT,
    fn GetTrustLevel(trustLevel: *mut TrustLevel) ->  HRESULT
}}

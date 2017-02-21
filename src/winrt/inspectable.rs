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
RIDL!{#[uuid(0xAF86E2E0, 0xB12D, 0x4c6a, 0x9C, 0x5A, 0xD7, 0xAA, 0x65, 0x10, 0x1E, 0x90)]
interface IInspectable(IInspectableVtbl): IUnknown(IUnknownVtbl) {
    fn GetIids(iidCount: *mut  ULONG, iids: *mut *mut  IID) ->  HRESULT,
    fn GetRuntimeClassName(className: *mut  HSTRING) ->  HRESULT,
    fn GetTrustLevel(trustLevel: *mut TrustLevel) ->  HRESULT
}}

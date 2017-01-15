// Copyright © 2016 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use ctypes::c_void;
use shared::guiddef::REFIID;
use shared::minwindef::{ BOOL, ULONG };
use um::winnt::HRESULT;
RIDL!{interface IUnknown(IUnknownVtbl) {
    fn QueryInterface(&self, riid: REFIID, ppvObject: *mut *mut c_void) -> HRESULT,
    fn AddRef(&self) -> ULONG,
    fn Release(&self) -> ULONG
}}
pub type LPUNKNOWN = *mut IUnknown;
RIDL!{interface AsyncIUnknown(AsyncIUnknownVtbl): IUnknown(IUnknownVtbl) {
    fn Begin_QueryInterface(&self, riid: REFIID) -> HRESULT,
    fn Finish_QueryInterface(&self, ppvObject: *mut *mut c_void) -> HRESULT,
    fn Begin_AddRef(&self) -> HRESULT,
    fn Finish_AddRef(&self) -> ULONG,
    fn Begin_Release(&self) -> HRESULT,
    fn Finish_Release(&self) -> ULONG
}}
RIDL!{interface IClassFactory(IClassFactoryVtbl): IUnknown(IUnknownVtbl) {
    fn CreateInstance(
        &self, pUnkOuter: *mut IUnknown, riid: REFIID, ppvObject: *mut *mut c_void
    ) -> HRESULT,
    fn LockServer(&self, fLock: BOOL) -> HRESULT
}}

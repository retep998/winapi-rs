// Copyright © 2015, Connor Hilarides
// Licensed under the MIT License <LICENSE.md>
//! Mappings for the contents of servprov.h
pub type LPSERVICEPROVIDER = *mut IServiceProvider;
RIDL!(
#[uuid(0x6d5140c1, 0x7436, 0x11ce, 0x80, 0x34, 0x00, 0xaa, 0x00, 0x60, 0x09, 0xfa)]
interface IServiceProvider(IServiceProviderVtbl): IUnknown(IUnknownVtbl) {
    fn QueryService(
        guidService: ::REFGUID, riid: ::REFIID, ppvObject: *mut *mut ::c_void
    ) -> ::HRESULT
}
);

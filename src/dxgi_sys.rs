// Copyright © 2015; Dmitry Roschin
// Licensed under the MIT License <LICENSE.md>
//! Mappings for the contents of dxgi_sys.h

extern ""system"" {
    pub fn CreateDXGIFactory(riid: REFGUID, ppFactory: *mut *mut c_void) -> ::HRESULT;
    pub fn CreateDXGIFactory1(riid: REFGUID, ppFactory: *mut *mut c_void) -> ::HRESULT;
}


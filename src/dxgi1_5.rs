// Copyright © 2015; Dmitry Roschin
// Licensed under the MIT License <LICENSE.md>
//! Mappings for the contents of dxgi1_5.h

ENUM!{ enum DXGI_HDR_METADATA_TYPE {
    DXGI_HDR_METADATA_TYPE_NONE = 0,
    DXGI_HDR_METADATA_TYPE_HDR10 = 1,
}}

ENUM!{ enum DXGI_FEATURE {
    DXGI_FEATURE_ALLOW_TEARING = 0,
}}

ENUM!{ enum DXGI_OFFER_RESOURCE_FLAGS {
    DXGI_OFFER_RESOURCE_FLAG_ALLOW_DECOMMIT = 0x1,
}}

ENUM!{ enum DXGI_RECLAIM_RESOURCE_RESULTS {
    DXGI_RECLAIM_RESOURCE_RESULT_OK = 0,
    DXGI_RECLAIM_RESOURCE_RESULT_DISCARDED = 1,
    DXGI_RECLAIM_RESOURCE_RESULT_NOT_COMMITTED = 2,
}}

STRUCT!{struct DXGI_HDR_METADATA_HDR10 {
    RedPrimary: [::UINT16; 2],
    GreenPrimary: [::UINT16; 2],
    BluePrimary: [::UINT16; 2],
    WhitePoint: [::UINT16; 2],
    MaxMasteringLuminance: ::UINT,
    MinMasteringLuminance: ::UINT,
    MaxContentLightLevel: ::UINT16,
    MaxFrameAverageLightLevel: ::UINT16,
}}

RIDL!(
interface IDXGIDevice4(IDXGIDevice4Vtbl): IDXGIDevice3(IDXGIDevice3Vtbl) {
    fn OfferResources1(
        &mut self, NumResources: ::UINT,
        ppResources: *const *mut ::IDXGIResource,
        Priority: ::DXGI_OFFER_RESOURCE_PRIORITY, Flags: ::UINT
    ) -> ::HRESULT,
    fn ReclaimResources1(
        &mut self, NumResources: ::UINT,
        ppResources: *const *mut ::IDXGIResource,
        pResults: *mut ::DXGI_RECLAIM_RESOURCE_RESULTS
    ) -> ::HRESULT
});

RIDL!(
interface IDXGIFactory5(IDXGIFactory5Vtbl): IDXGIFactory4(IDXGIFactory4Vtbl) {
    fn CheckFeatureSupport(
        &mut self, Feature: ::DXGI_FEATURE, pFeatureSupportData: *mut ::c_void,
        FeatureSupportDataSize: ::UINT
    ) -> ::HRESULT
});

RIDL!(
interface IDXGIOutput5(IDXGIOutput5Vtbl): IDXGIOutput4(IDXGIOutput4Vtbl) {
    fn DuplicateOutput1(
        &mut self, pDevice: *mut ::IUnknown, Flags: ::UINT,
        SupportedFormatsCount: ::UINT, pSupportedFormats: *const ::DXGI_FORMAT,
        ppOutputDuplication: *mut *mut ::IDXGIOutputDuplication
    ) -> ::HRESULT
});

RIDL!(
interface IDXGISwapChain4(IDXGISwapChain4Vtbl): IDXGISwapChain3(IDXGISwapChain3Vtbl) {
    fn SetHDRMetaData(
        &mut self, Type: ::DXGI_HDR_METADATA_TYPE, Size: ::UINT,
        pMetaData: *mut ::c_void
    ) -> ::HRESULT
});

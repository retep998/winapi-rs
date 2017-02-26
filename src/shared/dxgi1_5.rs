// Copyright © 2017; ekicyou
// Licensed under the MIT License <LICENSE.md>
//! Mappings for the contents of dxgi1_5.h

use ctypes::c_void;
use shared::basetsd::UINT16;
use shared::dxgi::IDXGIResource;
use shared::dxgi1_2::{DXGI_OFFER_RESOURCE_PRIORITY, IDXGIOutputDuplication};
use shared::dxgi1_3::{IDXGIDevice3, IDXGIDevice3Vtbl};
use shared::dxgi1_4::{IDXGIOutput4, IDXGIOutput4Vtbl, IDXGISwapChain3, IDXGISwapChain3Vtbl,
                      IDXGIFactory4, IDXGIFactory4Vtbl};
use shared::dxgiformat::DXGI_FORMAT;
use shared::minwindef::UINT;
use um::unknwnbase::IUnknown;
use um::winnt::HRESULT;

RIDL!(#[uuid(0x80A07424, 0xAB52, 0x42EB, 0x83, 0x3C, 0x0C, 0x42, 0xFD, 0x28, 0x2D, 0x98)]
interface IDXGIOutput5(IDXGIOutput5Vtbl): IDXGIOutput4(IDXGIOutput4Vtbl) {
    fn DuplicateOutput1(pDevice: *mut IUnknown,
                        Flags: UINT,
                        SupportedFormatsCount: UINT,
                        pSupportedFormats: *const DXGI_FORMAT,
                        ppOutputDuplication: *mut *mut IDXGIOutputDuplication
    )-> HRESULT
});

ENUM!{ enum DXGI_HDR_METADATA_TYPE {
    DXGI_HDR_METADATA_TYPE_NONE = 0,
    DXGI_HDR_METADATA_TYPE_HDR10 = 1,
}}

STRUCT!{struct DXGI_HDR_METADATA_HDR10 {
    RedPrimary: [UINT16; 2],
    GreenPrimary: [UINT16; 2],
    BluePrimary: [UINT16; 2],
    WhitePoint: [UINT16; 2],
    MaxMasteringLuminance: UINT,
    MinMasteringLuminance: UINT,
    MaxContentLightLevel: UINT16,
    MaxFrameAverageLightLevel: UINT16,
}}

RIDL!(#[uuid(0x3D585D5A, 0xBD4A, 0x489E, 0xB1, 0xF4, 0x3D, 0xBC, 0xB6, 0x45, 0x2F, 0xFB)]
interface IDXGISwapChain4(IDXGISwapChain4Vtbl): IDXGISwapChain3(IDXGISwapChain3Vtbl) {
    fn SetHDRMetaData(
        Type: DXGI_HDR_METADATA_TYPE,
        Size: UINT,
        pMetaData: *mut c_void
    )-> HRESULT
});

ENUM!{ enum DXGI_OFFER_RESOURCE_FLAGS {
    DXGI_OFFER_RESOURCE_FLAG_ALLOW_DECOMMIT = 0x1,
}}

ENUM!{ enum DXGI_RECLAIM_RESOURCE_RESULTS {
    DXGI_RECLAIM_RESOURCE_RESULT_OK = 0,
    DXGI_RECLAIM_RESOURCE_RESULT_DISCARDED = 1,
    DXGI_RECLAIM_RESOURCE_RESULT_NOT_COMMITTED = 2,
}}

RIDL!(#[uuid(0x95B4F95F, 0xD8DA, 0x4CA4, 0x9E, 0xE6, 0x3B, 0x76, 0xD5, 0x96, 0x8A, 0x10)]
interface IDXGIDevice4(IDXGIDevice4Vtbl): IDXGIDevice3(IDXGIDevice3Vtbl) {
    fn OfferResources1(
        NumResources: UINT,
        ppResources: *mut *mut IDXGIResource,
        Priority: DXGI_OFFER_RESOURCE_PRIORITY,
        Flags:UINT    
    ) -> HRESULT,
    fn ReclaimResources1(
        NumResources: UINT,
        ppResources: *mut *mut IDXGIResource,
        pResults: *mut DXGI_RECLAIM_RESOURCE_RESULTS
    ) -> HRESULT
});

ENUM!{ enum DXGI_FEATURE {
    DXGI_FEATURE_PRESENT_ALLOW_TEARING = 0,
}}

RIDL!(#[uuid(0x7632e1f5, 0xee65, 0x4dca, 0x87, 0xfd, 0x84, 0xcd, 0x75, 0xf8, 0x83, 0x8d)]
interface IDXGIFactory5(IDXGIFactory5Vtbl): IDXGIFactory4(IDXGIFactory4Vtbl) {
    fn CheckFeatureSupport(
        Feature: DXGI_FEATURE,
        pFeatureSupportData: *mut c_void,
        FeatureSupportDataSize: UINT
    ) -> HRESULT
});

DEFINE_GUID!{IID_IDXGIOutput5,0x80A07424,0xAB52,0x42EB,0x83,0x3C,0x0C,0x42,0xFD,0x28,0x2D,0x98}
DEFINE_GUID!{IID_IDXGISwapChain4,0x3D585D5A,0xBD4A,0x489E,0xB1,0xF4,0x3D,0xBC,0xB6,0x45,0x2F,0xFB}
DEFINE_GUID!{IID_IDXGIDevice4,0x95B4F95F,0xD8DA,0x4CA4,0x9E,0xE6,0x3B,0x76,0xD5,0x96,0x8A,0x10}
DEFINE_GUID!{IID_IDXGIFactory5,0x7632e1f5,0xee65,0x4dca,0x87,0xfd,0x84,0xcd,0x75,0xf8,0x83,0x8d}
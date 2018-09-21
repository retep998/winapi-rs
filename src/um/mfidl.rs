// Copyright © 2018 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Mappings for the contents of mfidl.h
use shared::basetsd::UINT32;
use shared::guiddef::{GUID, REFGUID, REFIID};
use shared::minwindef::{BOOL, DWORD, LPVOID};
use um::mfobjects::{
    IMFActivate, IMFAttributes, IMFAttributesVtbl, IMFMediaEventGenerator,
    IMFMediaEventGeneratorVtbl, IMFMediaType,
};
use um::propidl::PROPVARIANT;
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::{HRESULT, LONGLONG};
ENUM!{enum MFMEDIASOURCE_CHARACTERISTICS {
    MFMEDIASOURCE_IS_LIVE = 0x1,
    MFMEDIASOURCE_CAN_SEEK = 0x2,
    MFMEDIASOURCE_CAN_PAUSE = 0x4,
    MFMEDIASOURCE_HAS_SLOW_SEEK = 0x8,
    MFMEDIASOURCE_HAS_MULTIPLE_PRESENTATIONS = 0x10,
    MFMEDIASOURCE_CAN_SKIPFORWARD = 0x20,
    MFMEDIASOURCE_CAN_SKIPBACKWARD = 0x40,
    MFMEDIASOURCE_DOES_NOT_USE_NETWORK = 0x80,
}}
RIDL!{#[uuid(0x279a808d, 0xaec7, 0x40c8, 0x9c, 0x6b, 0xa6, 0xb4, 0x92, 0xc7, 0x8a, 0x66)]
interface IMFMediaSource(IMFMediaSourceVtbl): IMFMediaEventGenerator(IMFMediaEventGeneratorVtbl) {
    fn GetCharacteristics( 
        pdwCharacteristics: *mut DWORD,
    ) -> HRESULT,
    fn CreatePresentationDescriptor( 
        ppPresentationDescriptor: *mut *mut IMFPresentationDescriptor,
    ) -> HRESULT,
    fn Start( 
        pPresentationDescriptor: *mut IMFPresentationDescriptor,
        pguidTimeFormat: *const GUID,
        pvarStartPosition: *const PROPVARIANT,
    ) -> HRESULT,
    fn Stop() -> HRESULT,
    fn Pause() -> HRESULT,
    fn Shutdown() -> HRESULT,
}}
RIDL!{#[uuid(0xfa993888, 0x4383, 0x415a, 0xa9, 0x30, 0xdd, 0x47, 0x2a, 0x8c, 0xf6, 0xf7)]
interface IMFGetService(IMFGetServiceVtbl): IUnknown(IUnknownVtbl) {
    fn GetService(
        guidService: REFGUID,
        riid: REFIID,
        ppvObject: *mut LPVOID,
    ) -> HRESULT,
}}
extern "system" {
    pub fn MFGetService(
        punkObject: *mut IUnknown,
        guidService: REFGUID,
        riid: REFIID,
        ppvObject: *mut LPVOID,
    ) -> HRESULT;
}
pub type MFTIME = LONGLONG;
RIDL!{#[uuid(0x03cb2711, 0x24d7, 0x4db6, 0xa1, 0x7f, 0xf3, 0xa7, 0xa4, 0x79, 0xa5, 0x36)]
interface IMFPresentationDescriptor(IMFPresentationDescriptorVtbl):
    IMFAttributes(IMFAttributesVtbl) {
    fn GetStreamDescriptorCount( 
        pdwDescriptorCount: *mut DWORD,
    ) -> HRESULT,
    fn GetStreamDescriptorByIndex( 
        dwIndex: DWORD,
        pfSelected: *mut BOOL,
        ppDescriptor: *mut *mut IMFStreamDescriptor,
    ) -> HRESULT,
    fn SelectStream( 
        dwDescriptorIndex: DWORD,
    ) -> HRESULT,
    fn DeselectStream( 
        dwDescriptorIndex: DWORD,
    ) -> HRESULT,
    fn Clone( 
        ppPresentationDescriptor: *mut *mut IMFPresentationDescriptor,
    ) -> HRESULT,
}}
DEFINE_GUID!{MF_SD_LANGUAGE,
    0x00af2180, 0xbdc2, 0x423c, 0xab, 0xca, 0xf5, 0x03, 0x59, 0x3b, 0xc1, 0x21}
DEFINE_GUID!{MF_SD_PROTECTED,
    0x00af2181, 0xbdc2, 0x423c, 0xab, 0xca, 0xf5, 0x03, 0x59, 0x3b, 0xc1, 0x21}
DEFINE_GUID!{MF_SD_STREAM_NAME,
    0x4f1b099d, 0xd314, 0x41e5, 0xa7, 0x81, 0x7f, 0xef, 0xaa, 0x4c, 0x50, 0x1f}
DEFINE_GUID!{MF_SD_MUTUALLY_EXCLUSIVE,
    0x023ef79c, 0x388d, 0x487f, 0xac, 0x17, 0x69, 0x6c, 0xd6, 0xe3, 0xc6, 0xf5}
RIDL!{#[uuid(0x56c03d9c, 0x9dbb, 0x45f5, 0xab, 0x4b, 0xd8, 0x0f, 0x47, 0xc0, 0x59, 0x38)]
interface IMFStreamDescriptor(IMFStreamDescriptorVtbl): IMFAttributes(IMFAttributesVtbl) {
    fn GetStreamIdentifier( 
        pdwStreamIdentifier: *mut DWORD,
    ) -> HRESULT,
    fn GetMediaTypeHandler( 
        ppMediaTypeHandler: *mut *mut IMFMediaTypeHandler,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xe93dcf6c, 0x4b07, 0x4e1e, 0x81, 0x23, 0xaa, 0x16, 0xed, 0x6e, 0xad, 0xf5)]
interface IMFMediaTypeHandler(IMFMediaTypeHandlerVtbl): IUnknown(IUnknownVtbl) {
    fn IsMediaTypeSupported( 
        pMediaType: *mut IMFMediaType,
        ppMediaType: *mut *mut IMFMediaType,
    ) -> HRESULT,
    fn GetMediaTypeCount( 
        pdwTypeCount: *mut DWORD,
    ) -> HRESULT,
    fn GetMediaTypeByIndex( 
        dwIndex: DWORD,
        ppType: *mut *mut IMFMediaType,
    ) -> HRESULT,
    fn SetCurrentMediaType( 
        pMediaType: *mut IMFMediaType,
    ) -> HRESULT,
    fn GetCurrentMediaType( 
        ppMediaType: *mut *mut IMFMediaType,
    ) -> HRESULT,
    fn GetMajorType( 
        pguidMajorType: *mut GUID,
    ) -> HRESULT,
}}
extern "system" {
    pub fn MFEnumDeviceSources(
        pAttributes: *mut IMFAttributes,
        pppSourceActivate: *mut *mut *mut IMFActivate,
        pcSourceActivate: *mut UINT32,
    ) -> HRESULT;
    pub fn MFCreateDeviceSource(
        pAttributes: *mut IMFAttributes,
        ppSource: *mut *mut IMFMediaSource,
    ) -> HRESULT;
    pub fn MFCreateDeviceSourceActivate( 
        pAttributes: *mut IMFAttributes,
        ppActivate: *mut *mut IMFActivate,
    ) -> HRESULT;
}
DEFINE_GUID!{MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE,
    0xc60ac5fe, 0x252a, 0x478f, 0xa0, 0xef, 0xbc, 0x8f, 0xa5, 0xf7, 0xca, 0xd3}
DEFINE_GUID!{MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE_VIDCAP_HW_SOURCE,
    0xde7046ba, 0x54d6, 0x4487, 0xa2, 0xa4, 0xec, 0x7c, 0x0d, 0x1b, 0xd1, 0x63}
DEFINE_GUID!{MF_DEVSOURCE_ATTRIBUTE_FRIENDLY_NAME,
    0x60d0e559, 0x52f8, 0x4fa2, 0xbb, 0xce, 0xac, 0xdb, 0x34, 0xa8, 0xec, 0x01}
DEFINE_GUID!{MF_DEVSOURCE_ATTRIBUTE_MEDIA_TYPE,
    0x56a819ca, 0x0c78, 0x4de4, 0xa0, 0xa7, 0x3d, 0xda, 0xba, 0x0f, 0x24, 0xd4}
DEFINE_GUID!{MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE_VIDCAP_CATEGORY,
    0x77f0ae69, 0xc3bd, 0x4509, 0x94, 0x1d, 0x46, 0x7e, 0x4d, 0x24, 0x89, 0x9e}
DEFINE_GUID!{MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE_VIDCAP_SYMBOLIC_LINK,
    0x58f0aad8, 0x22bf, 0x4f8a, 0xbb, 0x3d, 0xd2, 0xc4, 0x97, 0x8c, 0x6e, 0x2f}
DEFINE_GUID!{MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE_AUDCAP_SYMBOLIC_LINK,
    0x98d24b5e, 0x5930, 0x4614, 0xb5, 0xa1, 0xf6, 0x00, 0xf9, 0x35, 0x5a, 0x78}
DEFINE_GUID!{MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE_VIDCAP_MAX_BUFFERS,
    0x7dd9b730, 0x4f2d, 0x41d5, 0x8f, 0x95, 0x0c, 0xc9, 0xa9, 0x12, 0xba, 0x26}
DEFINE_GUID!{MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE_AUDCAP_ENDPOINT_ID,
    0x30da9258, 0xfeb9, 0x47a7, 0xa4, 0x53, 0x76, 0x3a, 0x7a, 0x8e, 0x1c, 0x5f}
DEFINE_GUID!{MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE_AUDCAP_ROLE,
    0xbc9d118e, 0x8c67, 0x4a18, 0x85, 0xd4, 0x12, 0xd3, 0x00, 0x40, 0x05, 0x52}
DEFINE_GUID!{MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE_VIDCAP_PROVIDER_DEVICE_ID,
    0x36689d42, 0xa06c, 0x40ae, 0x84, 0xcf, 0xf5, 0xa0, 0x34, 0x06, 0x7c, 0xc4}
DEFINE_GUID!{MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE_AUDCAP_GUID,
    0x14dd9a1c, 0x7cff, 0x41be, 0xb1, 0xb9, 0xba, 0x1a, 0xc6, 0xec, 0xb5, 0x71}
DEFINE_GUID!{MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE_VIDCAP_GUID,
    0x8ac3587a, 0x4ae7, 0x42d8, 0x99, 0xe0, 0x0a, 0x60, 0x13, 0xee, 0xf9, 0x0f}
DEFINE_GUID!{MF_DEVICESTREAM_IMAGE_STREAM,
    0xa7ffb865, 0xe7b2, 0x43b0, 0x9f, 0x6f, 0x9a, 0xf2, 0xa0, 0xe5, 0x0f, 0xc0}
DEFINE_GUID!{MF_DEVICESTREAM_INDEPENDENT_IMAGE_STREAM,
    0x03eeec7e, 0xd605, 0x4576, 0x8b, 0x29, 0x65, 0x80, 0xb4, 0x90, 0xd7, 0xd3}
DEFINE_GUID!{MF_DEVICESTREAM_STREAM_ID,
    0x11bd5120, 0xd124, 0x446b, 0x88, 0xe6, 0x17, 0x06, 0x02, 0x57, 0xff, 0xf9}
DEFINE_GUID!{MF_DEVICESTREAM_STREAM_CATEGORY,
    0x2939e7b8, 0xa62e, 0x4579, 0xb6, 0x74, 0xd4, 0x07, 0x3d, 0xfa, 0xbb, 0xba}
DEFINE_GUID!{MF_DEVICESTREAM_FRAMESERVER_SHARED,
    0x1CB378E9, 0xB279, 0x41D4, 0xAF, 0x97, 0x34, 0xA2, 0x43, 0xE6, 0x83, 0x20}
DEFINE_GUID!{MF_DEVICESTREAM_TRANSFORM_STREAM_ID,
    0xe63937b7, 0xdaaf, 0x4d49, 0x81, 0x5f, 0xd8, 0x26, 0xf8, 0xad, 0x31, 0xe7}
DEFINE_GUID!{MF_DEVICESTREAM_EXTENSION_PLUGIN_CLSID,
    0x048e6558, 0x60c4, 0x4173, 0xbd, 0x5b, 0x6a, 0x3c, 0xa2, 0x89, 0x6a, 0xee}
DEFINE_GUID!{MF_DEVICEMFT_EXTENSION_PLUGIN_CLSID,
    0x0844dbae, 0x34fa, 0x48a0, 0xa7, 0x83, 0x8e, 0x69, 0x6f, 0xb1, 0xc9, 0xa8}
DEFINE_GUID!{MF_DEVICESTREAM_EXTENSION_PLUGIN_CONNECTION_POINT,
    0x37f9375c, 0xe664, 0x4ea4, 0xaa, 0xe4, 0xcb, 0x6d, 0x1d, 0xac, 0xa1, 0xf4}
DEFINE_GUID!{MF_DEVICESTREAM_TAKEPHOTO_TRIGGER,
    0x1d180e34, 0x538c, 0x4fbb, 0xa7, 0x5a, 0x85, 0x9a, 0xf7, 0xd2, 0x61, 0xa6}
DEFINE_GUID!{MF_DEVICESTREAM_MAX_FRAME_BUFFERS,
    0x1684cebe, 0x3175, 0x4985, 0x88, 0x2c, 0x0e, 0xfd, 0x3e, 0x8a, 0xc1, 0x1e}
DEFINE_GUID!{MF_DEVICEMFT_CONNECTED_FILTER_KSCONTROL,
    0x6a2c4fa6, 0xd179, 0x41cd, 0x95, 0x23, 0x82, 0x23, 0x71, 0xea, 0x40, 0xe5}
DEFINE_GUID!{MF_DEVICEMFT_CONNECTED_PIN_KSCONTROL,
    0xe63310f7, 0xb244, 0x4ef8, 0x9a, 0x7d, 0x24, 0xc7, 0x4e, 0x32, 0xeb, 0xd0}
DEFINE_GUID!{MF_DEVICE_THERMAL_STATE_CHANGED,
    0x70ccd0af, 0xfc9f, 0x4deb, 0xa8, 0x75, 0x9f, 0xec, 0xd1, 0x6c, 0x5b, 0xd4}
DEFINE_GUID!{MFSampleExtension_DeviceTimestamp,
    0x8f3e35e7, 0x2dcd, 0x4887, 0x86, 0x22, 0x2a, 0x58, 0xba, 0xa6, 0x52, 0xb0}
DEFINE_GUID!{MFSampleExtension_Spatial_CameraViewTransform,
    0x4e251fa4, 0x830f, 0x4770, 0x85, 0x9a, 0x4b, 0x8d, 0x99, 0xaa, 0x80, 0x9b}
DEFINE_GUID!{MFSampleExtension_Spatial_CameraCoordinateSystem,
    0x9d13c82f, 0x2199, 0x4e67, 0x91, 0xcd, 0xd1, 0xa4, 0x18, 0x1f, 0x25, 0x34}
DEFINE_GUID!{MFSampleExtension_Spatial_CameraProjectionTransform,
    0x47f9fcb5, 0x2a02, 0x4f26, 0xa4, 0x77, 0x79, 0x2f, 0xdf, 0x95, 0x88, 0x6a}

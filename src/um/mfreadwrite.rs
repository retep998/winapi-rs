// Copyright © 2018 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Mappings for the contents of mfreadwrite.h
use shared::guiddef::{REFGUID, REFIID};
use shared::minwindef::{BOOL, DWORD, LPVOID};
use shared::ntdef::{HRESULT, LONGLONG};
use um::mfidl::IMFMediaSource;
use um::mfobjects::{IMFAttributes, IMFMediaEvent, IMFMediaType, IMFSample};
use um::propidl::{PROPVARIANT, REFPROPVARIANT};
use um::unknwnbase::{IUnknown, IUnknownVtbl};
extern "system" {
    pub fn MFCreateSourceReaderFromMediaSource(
        pMediaSource: *mut IMFMediaSource,
        pAttributes: *mut IMFAttributes,
        ppSourceReader: *mut *mut IMFSourceReader,
    ) -> HRESULT;
}
DEFINE_GUID!{MF_SOURCE_READER_ASYNC_CALLBACK,
    0x1e3dbeac, 0xbb43, 0x4c35, 0xb5, 0x07, 0xcd, 0x64, 0x44, 0x64, 0xc9, 0x65}
DEFINE_GUID!{MF_SOURCE_READER_D3D_MANAGER,
    0xec822da2, 0xe1e9, 0x4b29, 0xa0, 0xd8, 0x56, 0x3c, 0x71, 0x9f, 0x52, 0x69}
DEFINE_GUID!{MF_SOURCE_READER_DISABLE_DXVA,
    0xaa456cfd, 0x3943, 0x4a1e, 0xa7, 0x7d, 0x18, 0x38, 0xc0, 0xea, 0x2e, 0x35}
DEFINE_GUID!{MF_SOURCE_READER_MEDIASOURCE_CONFIG,
    0x9085abeb, 0x0354, 0x48f9, 0xab, 0xb5, 0x20, 0x0d, 0xf8, 0x38, 0xc6, 0x8e}
DEFINE_GUID!{MF_SOURCE_READER_MEDIASOURCE_CHARACTERISTICS,
    0x6d23f5c8, 0xc5d7, 0x4a9b, 0x99, 0x71, 0x5d, 0x11, 0xf8, 0xbc, 0xa8, 0x80}
DEFINE_GUID!{MF_SOURCE_READER_ENABLE_VIDEO_PROCESSING,
    0xfb394f3d, 0xccf1, 0x42ee, 0xbb, 0xb3, 0xf9, 0xb8, 0x45, 0xd5, 0x68, 0x1d}
DEFINE_GUID!{MF_SOURCE_READER_ENABLE_ADVANCED_VIDEO_PROCESSING,
    0x0f81da2c, 0xb537, 0x4672, 0xa8, 0xb2, 0xa6, 0x81, 0xb1, 0x73, 0x07, 0xa3}
DEFINE_GUID!{MF_SOURCE_READER_DISABLE_CAMERA_PLUGINS,
    0x9d3365dd, 0x058f, 0x4cfb, 0x9f, 0x97, 0xb3, 0x14, 0xcc, 0x99, 0xc8, 0xad}
DEFINE_GUID!{MF_SOURCE_READER_DISCONNECT_MEDIASOURCE_ON_SHUTDOWN,
    0x56b67165, 0x219e, 0x456d, 0xa2, 0x2e, 0x2d, 0x30, 0x04, 0xc7, 0xfe, 0x56}
DEFINE_GUID!{MF_SOURCE_READER_ENABLE_TRANSCODE_ONLY_TRANSFORMS,
    0xdfd4f008, 0xb5fd, 0x4e78, 0xae, 0x44, 0x62, 0xa1, 0xe6, 0x7b, 0xbe, 0x27}
DEFINE_GUID!{MF_SOURCE_READER_D3D11_BIND_FLAGS,
    0x33f3197b, 0xf73a, 0x4e14, 0x8d, 0x85, 0x0e, 0x4c, 0x43, 0x68, 0x78, 0x8d}
ENUM!{enum MF_SOURCE_READER_FLAG {
    MF_SOURCE_READERF_ERROR = 0x1,
    MF_SOURCE_READERF_ENDOFSTREAM = 0x2,
    MF_SOURCE_READERF_NEWSTREAM = 0x4,
    MF_SOURCE_READERF_NATIVEMEDIATYPECHANGED = 0x10,
    MF_SOURCE_READERF_CURRENTMEDIATYPECHANGED = 0x20,
    MF_SOURCE_READERF_STREAMTICK = 0x100,
    MF_SOURCE_READERF_ALLEFFECTSREMOVED = 0x200,
}}
ENUM!{enum MF_SOURCE_READER_CONTROL_FLAG {
    MF_SOURCE_READER_CONTROLF_DRAIN = 0x1,
}}
ENUM!{enum __MIDL___MIDL_itf_mfreadwrite_0000_0001_0001 {
    MF_SOURCE_READER_INVALID_STREAM_INDEX = 0xffffffff,
    MF_SOURCE_READER_ALL_STREAMS = 0xfffffffe,
    MF_SOURCE_READER_ANY_STREAM = 0xfffffffe,
    MF_SOURCE_READER_FIRST_AUDIO_STREAM = 0xfffffffd,
    MF_SOURCE_READER_FIRST_VIDEO_STREAM = 0xfffffffc,
    MF_SOURCE_READER_MEDIASOURCE = 0xffffffff,
}}
ENUM!{enum __MIDL___MIDL_itf_mfreadwrite_0000_0001_0002 {
    MF_SOURCE_READER_CURRENT_TYPE_INDEX = 0xffffffff,
}}
RIDL!(#[uuid(0x70ae66f2, 0xc809, 0x4e4f, 0x89, 0x15, 0xbd, 0xcb, 0x40, 0x6b, 0x79, 0x93)]
interface IMFSourceReader(IMFSourceReaderVtbl): IUnknown(IUnknownVtbl) {
    fn GetStreamSelection( 
        dwStreamIndex: DWORD,
        pfSelected: *mut BOOL,
    ) -> HRESULT,
    fn SetStreamSelection( 
        dwStreamIndex: DWORD,
        fSelected: BOOL,
    ) -> HRESULT,
    fn GetNativeMediaType( 
        dwStreamIndex: DWORD,
        dwMediaTypeIndex: DWORD,
        ppMediaType: *mut *mut IMFMediaType,
    ) -> HRESULT,
    fn GetCurrentMediaType( 
        dwStreamIndex: DWORD,
        ppMediaType: *mut *mut IMFMediaType,
    ) -> HRESULT,
    fn SetCurrentMediaType( 
        dwStreamIndex: DWORD,
        pdwReserved: *mut DWORD,
        pMediaType: *mut IMFMediaType,
    ) -> HRESULT,
    fn SetCurrentPosition( 
        guidTimeFormat: REFGUID,
        varPosition: REFPROPVARIANT,
    ) -> HRESULT,
    fn ReadSample( 
        dwStreamIndex: DWORD,
        dwControlFlags: DWORD,
        pdwActualStreamIndex: *mut DWORD,
        pdwStreamFlags: *mut DWORD,
        pllTimestamp: *mut LONGLONG,
        ppSample: *mut *mut IMFSample,
    ) -> HRESULT,
    fn Flush( 
        dwStreamIndex: DWORD,
    ) -> HRESULT,
    fn GetServiceForStream( 
        dwStreamIndex: DWORD,
        guidService: REFGUID,
        riid: REFIID,
        ppvObject: *mut LPVOID,
    ) -> HRESULT,
    fn GetPresentationAttribute( 
        dwStreamIndex: DWORD,
        guidAttribute: REFGUID,
        pvarAttribute: *mut PROPVARIANT,
    ) -> HRESULT,
});
RIDL!(#[uuid(0xdeec8d99, 0xfa1d, 0x4d82, 0x84, 0xc2, 0x2c, 0x89, 0x69, 0x94, 0x48, 0x67)]
interface IMFSourceReaderCallback(IMFSourceReaderCallbackVtbl): IUnknown(IUnknownVtbl) {
    fn OnReadSample( 
        hrStatus: HRESULT,
        dwStreamIndex: DWORD,
        dwStreamFlags: DWORD,
        llTimestamp: LONGLONG,
        pSample: *mut IMFSample,
    ) -> HRESULT,
    fn OnFlush( 
        dwStreamIndex: DWORD,
    ) -> HRESULT,
    fn OnEvent( 
        dwStreamIndex: DWORD,
        pEvent: *mut IMFMediaEvent,
    ) -> HRESULT,
});

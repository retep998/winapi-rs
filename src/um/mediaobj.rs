// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::guiddef::{CLSID};
use shared::minwindef::{BYTE, DWORD, ULONG};
use um::strmif::{AM_MEDIA_TYPE, REFERENCE_TIME};
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::{HRESULT, LONG, LPWSTR};
pub type DMO_MEDIA_TYPE = AM_MEDIA_TYPE;
ENUM!{enum DMO_INPUT_DATA_BUFFER_FLAGS {
    DMO_INPUT_DATA_BUFFERF_SYNCPOINT = 1,
    DMO_INPUT_DATA_BUFFERF_TIME = 2,
    DMO_INPUT_DATA_BUFFERF_TIMELENGTH = 4,
    DMO_INPUT_DATA_BUFFERF_DISCONTINUITY = 8,
}}
ENUM!{enum DMO_OUTPUT_DATA_BUFFER_FLAGS {
    DMO_OUTPUT_DATA_BUFFERF_SYNCPOINT = 1,
    DMO_OUTPUT_DATA_BUFFERF_TIME = 2,
    DMO_OUTPUT_DATA_BUFFERF_TIMELENGTH = 4,
    DMO_OUTPUT_DATA_BUFFERF_DISCONTINUITY = 8,
    DMO_OUTPUT_DATA_BUFFERF_INCOMPLETE = 16777216,
}}
ENUM!{enum DMO_INPUT_STATUS_FLAGS {
    DMO_INPUT_STATUSF_ACCEPT_DATA = 1,
}}
ENUM!{enum DMO_INPUT_STREAM_INFO_FLAGS {
    DMO_INPUT_STREAMF_WHOLE_SAMPLES = 1,
    DMO_INPUT_STREAMF_SINGLE_SAMPLE_PER_BUFFER = 2,
    DMO_INPUT_STREAMF_FIXED_SAMPLE_SIZE = 4,
    DMO_INPUT_STREAMF_HOLDS_BUFFERS = 8,
}}
ENUM!{enum DMO_OUTPUT_STREAM_INFO_FLAGS {
    DMO_OUTPUT_STREAMF_WHOLE_SAMPLES = 1,
    DMO_OUTPUT_STREAMF_SINGLE_SAMPLE_PER_BUFFER = 2,
    DMO_OUTPUT_STREAMF_FIXED_SAMPLE_SIZE = 4,
    DMO_OUTPUT_STREAMF_DISCARDABLE = 8,
    DMO_OUTPUT_STREAMF_OPTIONAL = 16,
}}
ENUM!{enum DMO_SET_TYPE_FLAGS {
    DMO_SET_TYPEF_TEST_ONLY = 1,
    DMO_SET_TYPEF_CLEAR = 2,
}}
ENUM!{enum DMO_PROCESS_OUTPUT_FLAGS {
    DMO_PROCESS_OUTPUT_DISCARD_WHEN_NO_BUFFER = 1,
}}
RIDL!{#[uuid(0x59eff8b9, 0x938c, 0x4a26, 0x82, 0xf2, 0x95, 0xcb, 0x84, 0xcd, 0xc8, 0x37)]
interface IMediaBuffer(IMediaBufferVtbl): IUnknown(IUnknownVtbl) {
    fn SetLength(
        cbLength: DWORD,
    ) -> HRESULT,
    fn GetMaxLength(
        pcbMaxLength: *mut DWORD,
    ) -> HRESULT,
    fn GetBufferAndLength(
        ppBuffer: *mut *mut BYTE,
        pcbLength: *mut DWORD,
    ) -> HRESULT,
}}
STRUCT!{struct DMO_OUTPUT_DATA_BUFFER {
    pBuffer: *mut IMediaBuffer,
    dwStatus: DWORD,
    rtTimestamp: REFERENCE_TIME,
    rtTimelength: REFERENCE_TIME,
}}
pub type PDMO_OUTPUT_DATA_BUFFER = *mut DMO_OUTPUT_DATA_BUFFER;
RIDL!{#[uuid(0xd8ad0f58, 0x5494, 0x4102, 0x97, 0xc5, 0xec, 0x79, 0x8e, 0x59, 0xbc, 0xf4)]
interface IMediaObject(IMediaObjectVtbl): IUnknown(IUnknownVtbl) {
    fn GetStreamCount(
        pcInputStreams: *mut DWORD,
        pcOutputStreams: *mut DWORD,
    ) -> HRESULT,
    fn GetInputStreamInfo(
        dwInputStreamIndex: DWORD,
        pdwFlags: *mut DWORD,
    ) -> HRESULT,
    fn GetOutputStreamInfo(
        dwOutputStreamIndex: DWORD,
        pdwFlags: *mut DWORD,
    ) -> HRESULT,
    fn GetInputType(
        dwInputStreamIndex: DWORD,
        dwTypeIndex: DWORD,
        pmt: *mut DMO_MEDIA_TYPE,
    ) -> HRESULT,
    fn GetOutputType(
        dwOutputStreamIndex: DWORD,
        dwTypeIndex: DWORD,
        pmt: *mut DMO_MEDIA_TYPE,
    ) -> HRESULT,
    fn SetInputType(
        dwInputStreamIndex: DWORD,
        pmt: *const DMO_MEDIA_TYPE,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn SetOutputType(
        dwOutputStreamIndex: DWORD,
        pmt: *const DMO_MEDIA_TYPE,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn GetInputCurrentType(
        dwInputStreamIndex: DWORD,
        pmt: *mut DMO_MEDIA_TYPE,
    ) -> HRESULT,
    fn GetOutputCurrentType(
        dwOutputStreamIndex: DWORD,
        pmt: *mut DMO_MEDIA_TYPE,
    ) -> HRESULT,
    fn GetInputSizeInfo(
        dwInputStreamIndex: DWORD,
        pcbSize: *mut DWORD,
        pcbMaxLookahead: *mut DWORD,
        pcbAlignment: *mut DWORD,
    ) -> HRESULT,
    fn GetOutputSizeInfo(
        dwOutputStreamIndex: DWORD,
        pcbSize: *mut DWORD,
        pcbAlignment: *mut DWORD,
    ) -> HRESULT,
    fn GetInputMaxLatency(
        dwInputStreamIndex: DWORD,
        prtMaxLatency: *mut REFERENCE_TIME,
    ) -> HRESULT,
    fn SetInputMaxLatency(
        dwInputStreamIndex: DWORD,
        rtMaxLatency: REFERENCE_TIME,
    ) -> HRESULT,
    fn Flush(
    ) -> HRESULT,
    fn Discontinuity(
        dwInputStreamIndex: DWORD,
    ) -> HRESULT,
    fn AllocateStreamingResources(
    ) -> HRESULT,
    fn FreeStreamingResources(
    ) -> HRESULT,
    fn GetInputStatus(
        dwInputStreamIndex: DWORD,
        dwFlags: *mut DWORD,
    ) -> HRESULT,
    fn ProcessInput(
        dwInputStreamIndex: DWORD,
        pBuffer: *mut IMediaBuffer,
        dwFlags: DWORD,
        rtTimestamp: REFERENCE_TIME,
        rtTimelength: REFERENCE_TIME,
    ) -> HRESULT,
    fn ProcessOutput(
        dwFlags: DWORD,
        cOutputBufferCount: DWORD,
        pOutputBuffers: *mut DMO_OUTPUT_DATA_BUFFER,
        pdwStatus: *mut DWORD,
    ) -> HRESULT,
    fn Lock(
        bLock: LONG,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x2c3cd98a, 0x2bfa, 0x4a53, 0x9c, 0x27, 0x52, 0x49, 0xba, 0x64, 0xba, 0x0f)]
interface IEnumDMO(IEnumDMOVtbl): IUnknown(IUnknownVtbl) {
    fn Next(
        cItemsToFetch: DWORD,
        pCLSID: *mut CLSID,
        Names: *mut LPWSTR,
        pcItemsFetched: *mut DWORD,
    ) -> HRESULT,
    fn Skip(
        cItemsToSkip: DWORD,
    ) -> HRESULT,
    fn Reset(
    ) -> HRESULT,
    fn Clone(
        ppEnum: *mut *mut IEnumDMO,
    ) -> HRESULT,
}}
ENUM!{enum DMO_INPLACE_PROCESS_FLAGS {
    DMO_INPLACE_NORMAL = 0,
    DMO_INPLACE_ZERO = 1,
}}
RIDL!{#[uuid(0x651b9ad0, 0x0fc7, 0x4aa9, 0x95, 0x38, 0xd8, 0x99, 0x31, 0x01, 0x07, 0x41)]
interface IMediaObjectInPlace(IMediaObjectInPlaceVtbl): IUnknown(IUnknownVtbl) {
    fn Process(
        ulSize: ULONG,
        pData: *mut BYTE,
        refTimeStart: REFERENCE_TIME,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn Clone(
        ppMediaObject: *mut *mut IMediaObjectInPlace,
    ) -> HRESULT,
    fn GetLatency(
        pLatencyTime: *mut REFERENCE_TIME,
    ) -> HRESULT,
}}
ENUM!{enum DMO_QUALITY_STATUS_FLAGS {
    DMO_QUALITY_STATUS_ENABLED = 1,
}}
RIDL!{#[uuid(0x65abea96, 0xcf36, 0x453f, 0xaf, 0x8a, 0x70, 0x5e, 0x98, 0xf1, 0x62, 0x60)]
interface IDMOQualityControl(IDMOQualityControlVtbl): IUnknown(IUnknownVtbl) {
    fn SetNow(
        rtNow: REFERENCE_TIME,
    ) -> HRESULT,
    fn SetStatus(
        dwFlags: DWORD,
    ) -> HRESULT,
    fn GetStatus(
        pdwFlags: *mut DWORD,
    ) -> HRESULT,
}}
ENUM!{enum DMO_VIDEO_OUTPUT_STREAM_FLAGS {
    DMO_VOSF_NEEDS_PREVIOUS_SAMPLE = 1,
}}
RIDL!{#[uuid(0xbe8f4f4e, 0x5b16, 0x4d29, 0xb3, 0x50, 0x7f, 0x6b, 0x5d, 0x92, 0x98, 0xac)]
interface IDMOVideoOutputOptimizations(IDMOVideoOutputOptimizationsVtbl): IUnknown(IUnknownVtbl) {
    fn QueryOperationModePreferences(
        ulOutputStreamIndex: ULONG,
        pdwRequestedCapabilities: *mut DWORD,
    ) -> HRESULT,
    fn SetOperationMode(
        ulOutputStreamIndex: ULONG,
        dwEnabledFeatures: DWORD,
    ) -> HRESULT,
    fn GetCurrentOperationMode(
        ulOutputStreamIndex: ULONG,
        pdwEnabledFeatures: *mut DWORD,
    ) -> HRESULT,
    fn GetCurrentSampleRequirements(
        ulOutputStreamIndex: ULONG,
        pdwRequestedFeatures: *mut DWORD,
    ) -> HRESULT,
}}

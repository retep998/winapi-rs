// Copyright © 2018 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Mappings for the contents of mfobjects.h
use ctypes::{c_double, c_void};
use shared::basetsd::{UINT32, UINT64, UINT8};
use shared::guiddef::{GUID, REFGUID, REFIID};
use shared::minwindef::{BOOL, BYTE, DWORD, LPVOID, UINT};
use shared::wtypes::{VT_CLSID, VT_LPWSTR, VT_R8, VT_UI1, VT_UI4, VT_UI8, VT_UNKNOWN, VT_VECTOR};
use um::propidl::{PROPVARIANT, REFPROPVARIANT};
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::{HRESULT, LONG, LONGLONG, LPCWSTR, LPWSTR};
ENUM!{enum MF_ATTRIBUTE_TYPE {
    MF_ATTRIBUTE_UINT32 = VT_UI4,
    MF_ATTRIBUTE_UINT64 = VT_UI8,
    MF_ATTRIBUTE_DOUBLE = VT_R8,
    MF_ATTRIBUTE_GUID = VT_CLSID,
    MF_ATTRIBUTE_STRING = VT_LPWSTR,
    MF_ATTRIBUTE_BLOB = VT_VECTOR | VT_UI1,
    MF_ATTRIBUTE_IUNKNOWN = VT_UNKNOWN,
}}
ENUM!{enum MF_ATTRIBUTES_MATCH_TYPE {
    MF_ATTRIBUTES_MATCH_OUR_ITEMS = 0,
    MF_ATTRIBUTES_MATCH_THEIR_ITEMS = 1,
    MF_ATTRIBUTES_MATCH_ALL_ITEMS = 2,
    MF_ATTRIBUTES_MATCH_INTERSECTION = 3,
    MF_ATTRIBUTES_MATCH_SMALLER = 4,
}}
RIDL!(#[uuid(0x2cd2d921, 0xc447, 0x44a7, 0xa1, 0x3c, 0x4a, 0xda, 0xbf, 0xc2, 0x47, 0xe3)]
interface IMFAttributes(IMFAttributesVtbl): IUnknown(IUnknownVtbl) {
    fn GetItem(
        guidkey: REFGUID,
        pValue: *mut PROPVARIANT,
    ) -> HRESULT,
    fn GetItemType(
        guidkey: REFGUID,
        pType: *mut MF_ATTRIBUTE_TYPE,
    ) -> HRESULT,
    fn CompareItem(
        guidkey: REFGUID,
        Value: REFPROPVARIANT,
        pbResult: *mut BOOL,
    ) -> HRESULT,
    fn Compare(
        pTheirs: *mut IMFAttributes,
        MatchType: MF_ATTRIBUTES_MATCH_TYPE,
        pbResult: *mut BOOL,
    ) -> HRESULT,
    fn GetUINT32(
        guidkey: REFGUID,
        punValue: *mut UINT32,
    ) -> HRESULT,
    fn GetUINT64(
        guidkey: REFGUID,
        punValue: *mut UINT64,
    ) -> HRESULT,
    fn GetDouble(
        guidkey: REFGUID,
        pfValue: *mut c_double,
    ) -> HRESULT,
    fn GetGUID(
        guidkey: REFGUID,
        pguidValue: *mut GUID,
    ) -> HRESULT,
    fn GetStringLength(
        guidkey: REFGUID,
        pcchLength: *mut UINT32,
    ) -> HRESULT,
    fn GetString(
        guidkey: REFGUID,
        pwszValue: LPWSTR,
        cchBufSize: UINT32,
        pcchLength: *mut UINT32,
    ) -> HRESULT,
    fn GetAllocatedString(
        guidkey: REFGUID,
        ppwszValue: *mut LPWSTR,
        pcchLength: *mut UINT32,
    ) -> HRESULT,
    fn GetBlobSize(
        guidkey: REFGUID,
        pcbBlobSize: *mut UINT32,
    ) -> HRESULT,
    fn GetBlob(
        guidkey: REFGUID,
        pBuf: *mut UINT8,
        cbBufSize: UINT32,
        pcbBlobSize: *mut UINT32,
    ) -> HRESULT,
    fn GetAllocatedBlob(
        guidkey: REFGUID,
        ppBuf: *mut *mut UINT8,
        pcbSize: *mut UINT32,
    ) -> HRESULT,
    fn GetUnknown(
        guidkey: REFGUID,
        riid: REFIID,
        ppv: *mut LPVOID,
    ) -> HRESULT,
    fn SetItem(
        guidkey: REFGUID,
        Value: REFPROPVARIANT,
    ) -> HRESULT,
    fn DeleteItem(
        guidkey: REFGUID,
    ) -> HRESULT,
    fn DeleteAllItems(
    ) -> HRESULT,
    fn SetUINT32(
        guidkey: REFGUID,
        unValue: UINT32,
    ) -> HRESULT,
    fn SetUINT64(
        guidkey: REFGUID,
        unValue: UINT64,
    ) -> HRESULT,
    fn SetDouble(
        guidkey: REFGUID,
        fValue: c_double,
    ) -> HRESULT,
    fn SetGUID(
        guidkey: REFGUID,
        guidValue: REFGUID,
    ) -> HRESULT,
    fn SetString(
        guidkey: REFGUID,
        wszValue: LPCWSTR,
    ) -> HRESULT,
    fn SetBlob(
        guidkey: REFGUID,
        pBuf: *const UINT8,
        cbBufSize: UINT32,
    ) -> HRESULT,
    fn SetUnknown(
        guidkey: REFGUID,
        pUnknown: *mut IUnknown,
    ) -> HRESULT,
    fn LockStore() -> HRESULT,
    fn UnlockStore(
    ) -> HRESULT,
    fn GetCount(
        pcItems: *mut UINT32,
    ) -> HRESULT,
    fn GetItemByIndex(
        unIndex: UINT32,
        pguidKey: *mut GUID,
        pValue: *mut PROPVARIANT,
    ) -> HRESULT,
    fn CopyAllItems(
        pDest: *mut IMFAttributes,
    ) -> HRESULT,
});
RIDL!(#[uuid(0x045fa593, 0x8799, 0x42b8, 0xbc, 0x8d, 0x89, 0x68, 0xc6, 0x45, 0x35, 0x07)]
interface IMFMediaBuffer(IMFMediaBufferVtbl): IUnknown(IUnknownVtbl) {
    fn Lock(
        ppbBuffer: *mut *mut BYTE,
        pcbMaxLength: *mut DWORD,
        pcbCurrentLength: *mut DWORD,
    ) -> HRESULT,
    fn Unlock() -> HRESULT,
    fn GetCurrentLength(
        pcbCurrentLength: *mut DWORD,
    ) -> HRESULT,
    fn SetCurrentLength(
        cbCurrentLength: DWORD,
    ) -> HRESULT,
    fn GetMaxLength(
        pcbMaxLength: *mut DWORD,
    ) -> HRESULT,
});
RIDL!(#[uuid(0xc40a00f2, 0xb93a, 0x4d80, 0xae, 0x8c, 0x5a, 0x1c, 0x63, 0x4f, 0x58, 0xe4)]
interface IMFSample(IMFSampleVtbl): IMFAttributes(IMFAttributesVtbl) {
    fn GetSampleFlags(
        pdwSampleFlags: *mut DWORD,
    ) -> HRESULT,
    fn SetSampleFlags(
        dwSampleFlags: DWORD,
    ) -> HRESULT,
    fn GetSampleTime(
        phnsSampleTime: *mut LONGLONG,
    ) -> HRESULT,
    fn SetSampleTime(
        hnsSampleTime: LONGLONG,
    ) -> HRESULT,
    fn GetSampleDuration(
        phnsSampleDuration: *mut LONGLONG,
    ) -> HRESULT,
    fn SetSampleDuration(
        hnsSampleDuration: LONGLONG,
    ) -> HRESULT,
    fn GetBufferCount(
        pdwBufferCount: *mut DWORD,
    ) -> HRESULT,
    fn GetBufferByIndex(
        dwIndex: DWORD,
        ppBuffer: *mut *mut IMFMediaBuffer,
    ) -> HRESULT,
    fn ConvertToContiguousBuffer(
        ppBuffer: *mut *mut IMFMediaBuffer,
    ) -> HRESULT,
    fn AddBuffer(
        pBuffer: *mut IMFMediaBuffer,
    ) -> HRESULT,
    fn RemoveBufferByIndex(
        dwIndex: DWORD,
    ) -> HRESULT,
    fn RemoveAllBuffers(
    ) -> HRESULT,
    fn GetTotalLength(
        pcbTotalLength: *mut DWORD,
    ) -> HRESULT,
    fn CopyToBuffer(
        pBuffer: *mut IMFMediaBuffer,
    ) -> HRESULT,
});
RIDL!(#[uuid(0x7dc9d5f9, 0x9ed9, 0x44ec, 0x9b, 0xbf, 0x06, 0x00, 0xbb, 0x58, 0x9f, 0xbb)]
interface IMF2DBuffer(IMF2DBufferVtbl): IUnknown(IUnknownVtbl) {
    fn Lock2D(
        ppbScanline0: *mut *mut BYTE,
        plPitch: *mut LONG,
    ) -> HRESULT,
    fn Unlock2D() -> HRESULT,
    fn GetScanline0AndPitch(
        pbScanline0: *mut *mut BYTE,
        plPitch: *mut LONG,
    ) -> HRESULT,
    fn IsContiguousFormat(
        pfIsContiguous: *mut BOOL,
    ) -> HRESULT,
    fn GetContiguousLength(
        pcbLength: *mut DWORD,
    ) -> HRESULT,
    fn ContiguousCopyTo(
        pbDestBuffer: *mut BYTE,
        cbDestBuffer: DWORD,
    ) -> HRESULT,
    fn ContiguousCopyFrom(
        pbSrcBuffer: *const BYTE,
        cbSrcBuffer: DWORD,
    ) -> HRESULT,
});
ENUM!{enum MF2DBuffer_LockFlags {
    MF2DBuffer_LockFlags_LockTypeMask = 0x1|0x2|0x3,
    MF2DBuffer_LockFlags_Read = 0x1,
    MF2DBuffer_LockFlags_Write = 0x2,
    MF2DBuffer_LockFlags_ReadWrite = 0x3,
    MF2DBuffer_LockFlags_ForceDWORD = 0x7fffffff,
}}
RIDL!(#[uuid(0x33ae5ea6, 0x4316, 0x436f, 0x8d, 0xdd, 0xd7, 0x3d, 0x22, 0xf8, 0x29, 0xec)]
interface IMF2DBuffer2(IMF2DBuffer2Vtbl): IMF2DBuffer(IMF2DBufferVtbl) {
    fn Lock2DSize(
        lockFlags: MF2DBuffer_LockFlags,
        ppbScanline0: *mut *mut BYTE,
        plPitch: *mut LONG,
        ppbBufferStart: *mut *mut BYTE,
        pcbBufferLength: *mut DWORD,
    ) -> HRESULT,
    fn Copy2DTo(
        pDestBuffer: *mut IMF2DBuffer2,
    ) -> HRESULT,
});
RIDL!(#[uuid(0xe7174cfa, 0x1c9e, 0x48b1, 0x88, 0x66, 0x62, 0x62, 0x26, 0xbf, 0xc2, 0x58)]
interface IMFDXGIBuffer(IMFDXGIBufferVtbl): IUnknown(IUnknownVtbl) {
    fn GetResource(
        riid: REFIID,
        ppvObject: *mut LPVOID,
    ) -> HRESULT,
    fn GetSubresourceIndex(
        puSubresource: *mut UINT,
    ) -> HRESULT,
    fn GetUnknown(
        guid: REFIID,
        riid: REFIID,
        ppvObject: *mut LPVOID,
    ) -> HRESULT,
    fn SetUnknown(
        guid: REFIID,
        pUnkData: *mut IUnknown,
    ) -> HRESULT,
});
RIDL!(#[uuid(0x44ae0fa8, 0xea31, 0x4109, 0x8d, 0x2e, 0x4c, 0xae, 0x49, 0x97, 0xc5, 0x55)]
interface IMFMediaType(IMFMediaTypeVtbl): IMFAttributes(IMFAttributesVtbl) {
    fn GetMajorType(
        pguidMajorType: *mut GUID,
    ) -> HRESULT,
    fn IsCompressedFormat(
        pfCompressed: *mut BOOL,
    ) -> HRESULT,
    fn IsEqual(
        pIMediaType: *mut IMFMediaType,
        pdwFlags: *mut DWORD,
    ) -> HRESULT,
    fn GetRepresentation(
        guidRepresentation: GUID,
        ppvRepresentation: *mut LPVOID,
    ) -> HRESULT,
    fn FreeRepresentation(
        guidRepresentation: GUID,
        pvRepresentation: LPVOID,
    ) -> HRESULT,
});
RIDL!(#[uuid(0xac6b7889, 0x0740, 0x4d51, 0x86, 0x19, 0x90, 0x59, 0x94, 0xa5, 0x5c, 0xc6)]
interface IMFAsyncResult(IMFAsyncResultVtbl): IUnknown(IUnknownVtbl) {
    fn GetState(
        ppunkState: *mut *mut IUnknown,
    ) -> HRESULT,
    fn GetStatus() -> HRESULT,
    fn SetStatus(
        hrStatus: HRESULT,
    ) -> HRESULT,
    fn GetObject(
        ppObject: *mut *mut IUnknown,
    ) -> HRESULT,
    fn GetStateNoAddRef() -> *mut IUnknown,
});
RIDL!(#[uuid(0xa27003cf, 0x2354, 0x4f2a, 0x8d, 0x6a, 0xab, 0x7c, 0xff, 0x15, 0x43, 0x7e)]
interface IMFAsyncCallback(IMFAsyncCallbackVtbl): IUnknown(IUnknownVtbl) {
    fn GetParameters(
        pdwFlags: *mut DWORD,
        pdwQueue: *mut DWORD,
    ) -> HRESULT,
    fn Invoke(
        pAsyncResult: *mut IMFAsyncResult,
    ) -> HRESULT,
});
ENUM!{enum __MIDL___MIDL_itf_mfobjects_0000_0012_0001 {
    MEUnknown = 0,
    MEError = 1,
    MEExtendedType = 2,
    MENonFatalError = 3,
    MEGenericV1Anchor = MENonFatalError,
    MESessionUnknown = 100,
    MESessionTopologySet = 101,
    MESessionTopologiesCleared = 102,
    MESessionStarted = 103,
    MESessionPaused = 104,
    MESessionStopped = 105,
    MESessionClosed = 106,
    MESessionEnded = 107,
    MESessionRateChanged = 108,
    MESessionScrubSampleComplete = 109,
    MESessionCapabilitiesChanged = 110,
    MESessionTopologyStatus = 111,
    MESessionNotifyPresentationTime = 112,
    MENewPresentation = 113,
    MELicenseAcquisitionStart = 114,
    MELicenseAcquisitionCompleted = 115,
    MEIndividualizationStart = 116,
    MEIndividualizationCompleted = 117,
    MEEnablerProgress = 118,
    MEEnablerCompleted = 119,
    MEPolicyError = 120,
    MEPolicyReport = 121,
    MEBufferingStarted = 122,
    MEBufferingStopped = 123,
    MEConnectStart = 124,
    MEConnectEnd = 125,
    MEReconnectStart = 126,
    MEReconnectEnd = 127,
    MERendererEvent = 128,
    MESessionStreamSinkFormatChanged = 129,
    MESessionV1Anchor = MESessionStreamSinkFormatChanged,
    MESourceUnknown = 200,
    MESourceStarted = 201,
    MEStreamStarted = 202,
    MESourceSeeked = 203,
    MEStreamSeeked = 204,
    MENewStream = 205,
    MEUpdatedStream = 206,
    MESourceStopped = 207,
    MEStreamStopped = 208,
    MESourcePaused = 209,
    MEStreamPaused = 210,
    MEEndOfPresentation = 211,
    MEEndOfStream = 212,
    MEMediaSample = 213,
    MEStreamTick = 214,
    MEStreamThinMode = 215,
    MEStreamFormatChanged = 216,
    MESourceRateChanged = 217,
    MEEndOfPresentationSegment = 218,
    MESourceCharacteristicsChanged = 219,
    MESourceRateChangeRequested = 220,
    MESourceMetadataChanged = 221,
    MESequencerSourceTopologyUpdated = 222,
    MESourceV1Anchor = MESequencerSourceTopologyUpdated,
    MESinkUnknown = 300,
    MEStreamSinkStarted = 301,
    MEStreamSinkStopped = 302,
    MEStreamSinkPaused = 303,
    MEStreamSinkRateChanged = 304,
    MEStreamSinkRequestSample = 305,
    MEStreamSinkMarker = 306,
    MEStreamSinkPrerolled = 307,
    MEStreamSinkScrubSampleComplete = 308,
    MEStreamSinkFormatChanged = 309,
    MEStreamSinkDeviceChanged = 310,
    MEQualityNotify = 311,
    MESinkInvalidated = 312,
    MEAudioSessionNameChanged = 313,
    MEAudioSessionVolumeChanged = 314,
    MEAudioSessionDeviceRemoved = 315,
    MEAudioSessionServerShutdown = 316,
    MEAudioSessionGroupingParamChanged = 317,
    MEAudioSessionIconChanged = 318,
    MEAudioSessionFormatChanged = 319,
    MEAudioSessionDisconnected = 320,
    MEAudioSessionExclusiveModeOverride = 321,
    MESinkV1Anchor = MEAudioSessionExclusiveModeOverride,
    MECaptureAudioSessionVolumeChanged = 322,
    MECaptureAudioSessionDeviceRemoved = 323,
    MECaptureAudioSessionFormatChanged = 324,
    MECaptureAudioSessionDisconnected = 325,
    MECaptureAudioSessionExclusiveModeOverride = 326,
    MECaptureAudioSessionServerShutdown = 327,
    MESinkV2Anchor = MECaptureAudioSessionServerShutdown,
    METrustUnknown = 400,
    MEPolicyChanged = 401,
    MEContentProtectionMessage = 402,
    MEPolicySet = 403,
    METrustV1Anchor = MEPolicySet,
    MEWMDRMLicenseBackupCompleted = 500,
    MEWMDRMLicenseBackupProgress = 501,
    MEWMDRMLicenseRestoreCompleted = 502,
    MEWMDRMLicenseRestoreProgress = 503,
    MEWMDRMLicenseAcquisitionCompleted = 506,
    MEWMDRMIndividualizationCompleted = 508,
    MEWMDRMIndividualizationProgress = 513,
    MEWMDRMProximityCompleted = 514,
    MEWMDRMLicenseStoreCleaned = 515,
    MEWMDRMRevocationDownloadCompleted = 516,
    MEWMDRMV1Anchor = MEWMDRMRevocationDownloadCompleted,
    METransformUnknown = 600,
    METransformNeedInput = METransformUnknown + 1,
    METransformHaveOutput = METransformNeedInput + 1,
    METransformDrainComplete = METransformHaveOutput + 1,
    METransformMarker = METransformDrainComplete + 1,
    METransformInputStreamStateChanged = METransformMarker + 1,
    MEByteStreamCharacteristicsChanged = 700,
    MEVideoCaptureDeviceRemoved = 800,
    MEVideoCaptureDevicePreempted = 801,
    MEStreamSinkFormatInvalidated = 802,
    MEEncodingParameters = 803,
    MEContentProtectionMetadata = 900,
    MEDeviceThermalStateChanged = 950,
    MEReservedMax = 10000,
}}
pub type MediaEventType = DWORD;
RIDL!(#[uuid(0xdf598932, 0xf10c, 0x4e39, 0xbb, 0xa2, 0xc3, 0x08, 0xf1, 0x01, 0xda, 0xa3)]
interface IMFMediaEvent(IMFMediaEventVtbl): IMFAttributes(IMFAttributesVtbl) {
    fn GetType(
        met: *mut MediaEventType,
    ) -> HRESULT,
    fn GetExtendedType(
        pguidExtendedType: *mut GUID,
    ) -> HRESULT,
    fn GetStatus(
        phrStatus: *mut HRESULT,
    ) -> HRESULT,
    fn GetValue(
        pvValue: *mut PROPVARIANT,
    ) -> HRESULT,
});
RIDL!(#[uuid(0x2cd0bd52, 0xbcd5, 0x4b89, 0xb6, 0x2c, 0xea, 0xdc, 0x0c, 0x03, 0x1e, 0x7d)]
interface IMFMediaEventGenerator(IMFMediaEventGeneratorVtbl): IUnknown(IUnknownVtbl) {
    fn GetEvent(
        dwFlags: DWORD,
        ppEvent: *mut *mut IMFMediaEvent,
    ) -> HRESULT,
    fn BeginGetEvent(
        pCallback: *mut IMFAsyncCallback,
        punkState: *mut IUnknown,
    ) -> HRESULT,
    fn EndGetEvent(
        pResult: *mut IMFAsyncResult,
        ppEvent: *mut *mut IMFMediaEvent,
    ) -> HRESULT,
    fn QueueEvent(
        met: MediaEventType,
        guidExtendedType: REFGUID,
        hrStatus: HRESULT,
        prValue: *const PROPVARIANT,
    ) -> HRESULT,
});
RIDL!(#[uuid(0x7FEE9E9A, 0x4A89, 0x47a6, 0x89, 0x9C, 0xB6, 0xA5, 0x3A, 0x70, 0xFB, 0x67)]
interface IMFActivate(IMFActivateVtbl): IMFAttributes(IMFAttributesVtbl) {
    fn ActivateObject(
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
    fn ShutdownObject() -> HRESULT,
    fn DetachObject() -> HRESULT,
});

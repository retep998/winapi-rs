// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms
use ctypes::c_void;
use shared::basetsd::ULONG64;
use shared::guiddef::{GUID, REFCLSID, REFIID};
use shared::minwindef::{BOOL, BYTE, DWORD, FILETIME, ULONG};
use shared::windef::{HWND, RECT};
use shared::wtypes::{BSTR, PROPID};
use shared::wtypesbase::LPOLESTR;
use um::objidl::STGMEDIUM;
use um::objidlbase::IStream;
use um::propidl::{IEnumSTATPROPSTG, PROPSPEC, PROPVARIANT, STATPROPSETSTG};
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::{HRESULT, LONG};
STRUCT!{struct WIA_DITHER_PATTERN_DATA {
    lSize: LONG,
    bstrPatternName: BSTR,
    lPatternWidth: LONG,
    lPatternLength: LONG,
    cbPattern: LONG,
    pbPattern: *mut BYTE,
}}
pub type PWIA_DITHER_PATTERN_DATA = *mut WIA_DITHER_PATTERN_DATA;
STRUCT!{struct WIA_PROPID_TO_NAME {
    propid: PROPID,
    pszName: LPOLESTR,
}}
pub type PWIA_PROPID_TO_NAME = *mut WIA_PROPID_TO_NAME;
STRUCT!{struct WIA_FORMAT_INFO {
    guidFormatID: GUID,
    lTymed: LONG,
}}
pub type PWIA_FORMAT_INFO = *mut WIA_FORMAT_INFO;
DEFINE_GUID!{IID_IWiaDevMgr,
    0x5eb2502a, 0x8cf1, 0x11d1, 0xbf, 0x92, 0x00, 0x60, 0x08, 0x1e, 0xd8, 0x11}
RIDL!{#[uuid(0x5eb2502a, 0x8cf1, 0x11d1, 0xbf, 0x92, 0x00, 0x60, 0x08, 0x1e, 0xd8, 0x11)]
interface IWiaDevMgr(IWiaDevMgrVtbl): IUnknown(IUnknownVtbl) {
    fn EnumDeviceInfo(
        lFlag: LONG,
        ppIEnum: *mut *mut IEnumWIA_DEV_INFO,
    ) -> HRESULT,
    fn CreateDevice(
        bstrDeviceID: BSTR,
        ppWiaItemRoot: *mut *mut IWiaItem,
    ) -> HRESULT,
    fn SelectDeviceDlg(
        hwndParent: HWND,
        lDeviceType: LONG,
        lFlags: LONG,
        pbstrDeviceID: *mut BSTR,
        ppItemRoot: *mut *mut IWiaItem,
    ) -> HRESULT,
    fn SelectDeviceDlgID(
        hwndParent: HWND,
        lDeviceType: LONG,
        lFlags: LONG,
        pbstrDeviceID: *mut BSTR,
    ) -> HRESULT,
    fn GetImageDlg(
        hwndParent: HWND,
        lDeviceType: LONG,
        lFlags: LONG,
        lIntent: LONG,
        pItemRoot: *mut IWiaItem,
        bstrFilename: BSTR,
        pguidFormat: *mut GUID,
    ) -> HRESULT,
    fn RegisterEventCallbackProgram(
        lFlags: LONG,
        bstrDeviceID: BSTR,
        pEventGUID: *const GUID,
        bstrCommandline: BSTR,
        bstrName: BSTR,
        bstrDescription: BSTR,
        bstrIcon: BSTR,
    ) -> HRESULT,
    fn RegisterEventCallbackInterface(
        lFlags: LONG,
        bstrDeviceID: BSTR,
        pEventGUID: *const GUID,
        pIWiaEventCallback: *mut IWiaEventCallback,
        pEventObject: *mut *mut IUnknown,
    ) -> HRESULT,
    fn RegisterEventCallbackCLSID(
        lFlags: LONG,
        bstrDeviceID: BSTR,
        pEventGUID: *const GUID,
        pClsID: *const GUID,
        bstrName: BSTR,
        bstrDescription: BSTR,
        bstrIcon: BSTR,
    ) -> HRESULT,
    fn AddDeviceDlg(
        hwndParent: HWND,
        lFlags: LONG,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IEnumWIA_DEV_INFO,
    0x5e38b83c, 0x8cf1, 0x11d1, 0xbf, 0x92, 0x00, 0x60, 0x08, 0x1e, 0xd8, 0x11}
RIDL!{#[uuid(0x5e38b83c, 0x8cf1, 0x11d1, 0xbf, 0x92, 0x00, 0x60, 0x08, 0x1e, 0xd8, 0x11)]
interface IEnumWIA_DEV_INFO(IEnumWIA_DEV_INFOVtbl): IUnknown(IUnknownVtbl) {
    fn Next(
        celt: ULONG,
        rgelt: *mut *mut IWiaPropertyStorage,
        pceltFetched: *mut ULONG,
    ) -> HRESULT,
    fn Skip(
        celt: ULONG,
    ) -> HRESULT,
    fn Reset() -> HRESULT,
    fn Clone(
        ppIEnum: *mut *mut IEnumWIA_DEV_INFO,
    ) -> HRESULT,
    fn GetCount(
        celt: *mut ULONG,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IWiaEventCallback,
    0xae6287b0, 0x0084, 0x11d2, 0x97, 0x3b, 0x00, 0xa0, 0xc9, 0x06, 0x8f, 0x2e}
RIDL!{#[uuid(0xae6287b0, 0x0084, 0x11d2, 0x97, 0x3b, 0x00, 0xa0, 0xc9, 0x06, 0x8f, 0x2e)]
interface IWiaEventCallback(IWiaEventCallbackVtbl): IUnknown(IUnknownVtbl) {
    fn ImageEventCallback(
        pEventGUID: *const GUID,
        bstrEventDescription: BSTR,
        bstrDeviceID: BSTR,
        bstrDeviceDescription: BSTR,
        dwDeviceType: DWORD,
        bstrFullItemName: BSTR,
        pulEventType: *mut ULONG,
        ulReserved: ULONG,
    ) -> HRESULT,
}}
STRUCT!{struct WIA_DATA_CALLBACK_HEADER {
    lSize: LONG,
    guidFormatID: GUID,
    lBufferSize: LONG,
    lPageCount: LONG,
}}
pub type PWIA_DATA_CALLBACK_HEADER = *mut WIA_DATA_CALLBACK_HEADER;
DEFINE_GUID!{IID_IWiaDataCallback,
    0xa558a866, 0xa5b0, 0x11d2, 0xa0, 0x8f, 0x00, 0xc0, 0x4f, 0x72, 0xdc, 0x3c}
RIDL!{#[uuid(0xa558a866, 0xa5b0, 0x11d2, 0xa0, 0x8f, 0x00, 0xc0, 0x4f, 0x72, 0xdc, 0x3c)]
interface IWiaDataCallback(IWiaDataCallbackVtbl): IUnknown(IUnknownVtbl) {
    fn BandedDataCallback(
        lMessage: LONG,
        lStatus: LONG,
        lPercentComplete: LONG,
        lOffset: LONG,
        lLength: LONG,
        lReserved: LONG,
        lResLength: LONG,
        pbBuffer: *mut BYTE,
    ) -> HRESULT,
}}
STRUCT!{struct WIA_DATA_TRANSFER_INFO {
    ulSize: ULONG,
    ulSection: ULONG,
    ulBufferSize: ULONG,
    bDoubleBuffer: BOOL,
    ulReserved1: ULONG,
    ulReserved2: ULONG,
    ulReserved3: ULONG,
}}
pub type PWIA_DATA_TRANSFER_INFO = *mut WIA_DATA_TRANSFER_INFO;
STRUCT!{struct WIA_EXTENDED_TRANSFER_INFO {
    ulSize: ULONG,
    ulMinBufferSize: ULONG,
    ulOptimalBufferSize: ULONG,
    ulMaxBufferSize: ULONG,
    ulNumBuffers: ULONG,
}}
pub type PWIA_EXTENDED_TRANSFER_INFO = *mut WIA_EXTENDED_TRANSFER_INFO;
DEFINE_GUID!{IID_IWiaDataTransfer,
    0xa6cef998, 0xa5b0, 0x11d2, 0xa0, 0x8f, 0x00, 0xc0, 0x4f, 0x72, 0xdc, 0x3c}
RIDL!{#[uuid(0xa6cef998, 0xa5b0, 0x11d2, 0xa0, 0x8f, 0x00, 0xc0, 0x4f, 0x72, 0xdc, 0x3c)]
interface IWiaDataTransfer(IIWiaDataTransferkVtbl): IUnknown(IUnknownVtbl) {
    fn idtGetData(
        pMedium: *mut STGMEDIUM,
        pIWiaDataCallback: *mut IWiaDataCallback,
    ) -> HRESULT,
    fn idtGetBandedData(
        pWiaDataTransInfo: PWIA_DATA_TRANSFER_INFO,
        pIWiaDataCallback: *mut IWiaDataCallback,
    ) -> HRESULT,
    fn idtQueryGetData(
        pfe: *mut WIA_FORMAT_INFO,
    ) -> HRESULT,
    fn idtEnumWIA_FORMAT_INFO(
        ppEnum: *mut *mut IEnumWIA_FORMAT_INFO,
    ) -> HRESULT,
    fn idtGetExtendedTransferInfo(
        pExtendedTransferInfo: PWIA_EXTENDED_TRANSFER_INFO,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IWiaItem,
    0x4db1ad10, 0x3391, 0x11d2, 0x9a, 0x33, 0x00, 0xc0, 0x4f, 0xa3, 0x61, 0x45}
RIDL!{#[uuid(0x4db1ad10, 0x3391, 0x11d2, 0x9a, 0x33, 0x00, 0xc0, 0x4f, 0xa3, 0x61, 0x45)]
interface IWiaItem(IWiaItemVtbl): IUnknown(IUnknownVtbl) {
    fn GetItemType(
        pItemType: *mut LONG,
    ) -> HRESULT,
    fn AnalyzeItem(
        lFlags: LONG,
    ) -> HRESULT,
    fn EnumChildItems(
        ppIEnumWiaItem: *mut *mut IEnumWiaItem,
    ) -> HRESULT,
    fn DeleteItem(
        lFlags: LONG,
    ) -> HRESULT,
    fn CreateChildItem(
        lFlags: LONG,
        bstrItemName: BSTR,
        bstrFullItemName: BSTR,
        ppIWiaItem: *mut *mut IWiaItem,
    ) -> HRESULT,
    fn EnumRegisterEventInfo(
        lFlags: LONG,
        pEventGUID: *const GUID,
        ppIEnum: *mut *mut IEnumWIA_DEV_CAPS,
    ) -> HRESULT,
    fn FindItemByName(
        lFlags: LONG,
        bstrFullItemName: BSTR,
        ppIWiaItem: *mut *mut IWiaItem,
    ) -> HRESULT,
    fn DeviceDlg(
        hwndParent: HWND,
        lFlags: LONG,
        lIntent: LONG,
        plItemCount: *mut LONG,
        ppIWiaItem: *mut *mut *mut IWiaItem,
    ) -> HRESULT,
    fn DeviceCommand(
        lFlags: LONG,
        pCmdGUID: *const GUID,
        ppIWiaItem: *mut *mut IWiaItem,
    ) -> HRESULT,
    fn GetRootItem(
        ppIWiaItem: *mut *mut IWiaItem,
    ) -> HRESULT,
    fn EnumDeviceCapabilities(
        lFlags: LONG,
        ppIEnumWIA_DEV_CAPS: *mut *mut IEnumWIA_DEV_CAPS,
    ) -> HRESULT,
    fn DumpItemData(
        bstrData: *mut BSTR,
    ) -> HRESULT,
    fn DumpDrvItemData(
        bstrData: *mut BSTR,
    ) -> HRESULT,
    fn DumpTreeItemData(
        bstrData: *mut BSTR,
    ) -> HRESULT,
    fn Diagnostic(
        ulSize: ULONG,
        pBuffer: *mut BYTE,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IWiaPropertyStorage,
    0x98B5E8A0, 0x29CC, 0x491a, 0xAA, 0xC0, 0xE6, 0xDB, 0x4F, 0xDC, 0xCE, 0xB6}
RIDL!{#[uuid(0x98b5e8a0, 0x29cc, 0x491a, 0xaa, 0xc0, 0xe6, 0xdb, 0x4f, 0xdc, 0xce, 0xb6)]
interface IWiaPropertyStorage(IWiaPropertyStorageVtbl): IUnknown(IUnknownVtbl) {
    fn ReadMultiple(
        cpspec: ULONG,
        rgpspec: *const PROPSPEC,
        rgpropvar: *mut PROPVARIANT,
    ) -> HRESULT,
    fn WriteMultiple(
        cpspec: ULONG,
        rgpspec: *const PROPSPEC,
        rgpropvar: *const PROPVARIANT,
        propidNameFirst: PROPID,
    ) -> HRESULT,
    fn DeleteMultiple(
        cpspec: ULONG,
        rgpspec: *const PROPSPEC,
    ) -> HRESULT,
    fn ReadPropertyNames(
        cpropid: ULONG,
        rgpropid: *const PROPID,
        rglpwstrName: *mut LPOLESTR,
    ) -> HRESULT,
    fn WritePropertyNames(
        cpropid: ULONG,
        rgpropid: *const PROPID,
        rglpwstrName: *const LPOLESTR,
    ) -> HRESULT,
    fn DeletePropertyNames(
        cpropid: ULONG,
        rgpropid: *const PROPID,
    ) -> HRESULT,
    fn Commit(
        grfCommitFlags: DWORD,
    ) -> HRESULT,
    fn Revert() -> HRESULT,
    fn Enum(
        ppenum: *mut *mut IEnumSTATPROPSTG,
    ) -> HRESULT,
    fn SetTimes(
        pctime: *const FILETIME,
        patime: *const FILETIME,
        pmtime: *const FILETIME,
    ) -> HRESULT,
    fn SetClass(
        clsid: REFCLSID,
    ) -> HRESULT,
    fn Stat(
        pstatpsstg: *mut STATPROPSETSTG,
    ) -> HRESULT,
    fn GetPropertyAttributes(
        cpspec: ULONG,
        rgpspec: *mut PROPSPEC,
        rgflags: *mut ULONG,
        rgpropvar: *mut PROPVARIANT,
    ) -> HRESULT,
    fn GetCount(
        pulNumProps: *mut ULONG,
    ) -> HRESULT,
    fn GetPropertyStream(
        pCompatibilityId: *mut GUID,
        ppIStream: *mut *mut IStream,
    ) -> HRESULT,
    fn SetPropertyStream(
        pCompatibilityId: *const GUID,
        pIStream: *mut IStream,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IEnumWiaItem,
    0x5e8383fc, 0x3391, 0x11d2, 0x9a, 0x33, 0x00, 0xc0, 0x4f, 0xa3, 0x61, 0x45}
RIDL!{#[uuid(0x5e8383fc, 0x3391, 0x11d2, 0x9a, 0x33, 0x00, 0xc0, 0x4f, 0xa3, 0x61, 0x45)]
interface IEnumWiaItem(IEnumWiaItemVtbl): IUnknown(IUnknownVtbl) {
    fn Next(
        celt: ULONG,
        ppIWiaItem: *mut *mut IWiaItem,
        pceltFetched: *mut ULONG,
    ) -> HRESULT,
    fn Skip(
        celt: ULONG,
    ) -> HRESULT,
    fn Reset() -> HRESULT,
    fn Clone(
        ppIEnum: *mut *mut IEnumWiaItem,
    ) -> HRESULT,
    fn GetCount(
        celt: *mut ULONG,
    ) -> HRESULT,
}}
STRUCT!{struct WIA_DEV_CAP {
    guid: GUID,
    ulFlags: ULONG,
    bstrName: BSTR,
    bstrDescription: BSTR,
    bstrIcon: BSTR,
    bstrCommandline: BSTR,
}}
pub type PWIA_DEV_CAP = *mut WIA_DEV_CAP;
pub type WIA_EVENT_HANDLER = WIA_DEV_CAP;
pub type PWIA_EVENT_HANDLER = *mut WIA_EVENT_HANDLER;
DEFINE_GUID!{IID_IEnumWIA_DEV_CAPS,
    0x1fcc4287, 0xaca6, 0x11d2, 0xa0, 0x93, 0x00, 0xc0, 0x4f, 0x72, 0xdc, 0x3c}
RIDL!{#[uuid(0x1fcc4287, 0xaca6, 0x11d2, 0xa0, 0x93, 0x00, 0xc0, 0x4f, 0x72, 0xdc, 0x3c)]
interface IEnumWIA_DEV_CAPS(IEnumWIA_DEV_CAPSVtbl): IUnknown(IUnknownVtbl) {
    fn Next(
        celt: ULONG,
        rgelt: *mut WIA_DEV_CAP,
        pceltFetched: *mut ULONG,
    ) -> HRESULT,
    fn Skip(
        celt: ULONG,
    ) -> HRESULT,
    fn Reset() -> HRESULT,
    fn Clone(
        ppIEnum: *mut *mut IEnumWIA_DEV_CAPS,
    ) -> HRESULT,
    fn GetCount(
        celt: *mut ULONG,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IEnumWIA_FORMAT_INFO,
    0x81BEFC5B, 0x656D, 0x44f1, 0xB2, 0x4C, 0xD4, 0x1D, 0x51, 0xB4, 0xDC, 0x81}
RIDL!{#[uuid(0x81befc5b, 0x656d, 0x44f1, 0xb2, 0x4c, 0xd4, 0x1d, 0x51, 0xb4, 0xdc, 0x81)]
interface IEnumWIA_FORMAT_INFO(IEnumWIA_FORMAT_INFOVtbl): IUnknown(IUnknownVtbl) {
    fn Next(
        celt: ULONG,
        rgelt: *mut WIA_FORMAT_INFO,
        pceltFetched: *mut ULONG,
    ) -> HRESULT,
    fn Skip(
        celt: ULONG,
    ) -> HRESULT,
    fn Reset() -> HRESULT,
    fn Clone(
        ppIEnum: *mut *mut IEnumWIA_FORMAT_INFO,
    ) -> HRESULT,
    fn GetCount(
        celt: *mut ULONG,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IWiaLog,
    0xA00C10B6, 0x82A1, 0x452f, 0x8B, 0x6C, 0x86, 0x06, 0x2A, 0xAD, 0x68, 0x90}
RIDL!{#[uuid(0xa00c10b6, 0x82a1, 0x452f, 0x8b, 0x6c, 0x86, 0x06, 0x2a, 0xad, 0x68, 0x90)]
interface IWiaLog(IWiaLogVtbl): IUnknown(IUnknownVtbl) {
    fn InitializeLog(
        hInstance: LONG,
    ) -> HRESULT,
    fn hResult(
        hResult: HRESULT,
    ) -> HRESULT,
    fn Log(
        lFlags: LONG,
        lResID: LONG,
        lDetail: LONG,
        bstrText: BSTR,
    ) -> HRESULT, 
}}
DEFINE_GUID!{IID_IWiaLogEx,
    0xAF1F22AC, 0x7A40, 0x4787, 0xB4, 0x21, 0xAE, 0xb4, 0x7A, 0x1F, 0xBD, 0x0B}
RIDL!{#[uuid(0xaf1f22ac, 0x7a40, 0x4787, 0xb4, 0x21, 0xae, 0xb4, 0x7a, 0x1f, 0xbd, 0x0b)]
interface IWiaLogEx(IWiaLogExVtbl): IUnknown(IUnknownVtbl) {
    fn InitializeLogEx(
        hInstance: *mut BYTE,
    ) -> HRESULT,
    fn hResult(
        hResult: HRESULT,
    ) -> HRESULT,
    fn Log(
        lFlags: LONG,
        lResID: LONG,
        lDetail: LONG,
        bstrText: BSTR,
    ) -> HRESULT, 
    fn hResultEx(
        lMethodId: LONG,
        hResult: HRESULT,
    ) -> HRESULT,
    fn LogEx(
        lMethodId: LONG,
        lFlags: LONG,
        lResID: LONG,
        lDetail: LONG,
        bstrText: BSTR,
    ) -> HRESULT, 
}}
DEFINE_GUID!{IID_IWiaNotifyDevMgr,
    0x70681EA0, 0xE7BF, 0x4291, 0x9F, 0xB1, 0x4E, 0x88, 0x13, 0xA3, 0xF7, 0x8E}
RIDL!{#[uuid(0x70681ea0, 0xe7bf, 0x4291, 0x9f, 0xb1, 0x4e, 0x88, 0x13, 0xa3, 0xf7, 0x8e)]
interface IWiaNotifyDevMgr(IWiaNotifyDevMgrVtbl): IUnknown(IUnknownVtbl) {
    fn NewDeviceArrival() -> HRESULT,
}}
DEFINE_GUID!{IID_IWiaItemExtras,
    0x6291ef2c, 0x36ef, 0x4532, 0x87, 0x6a, 0x8e, 0x13, 0x25, 0x93, 0x77, 0x8d}
RIDL!{#[uuid(0x6291ef2c, 0x36ef, 0x4532, 0x87, 0x6a, 0x8e, 0x13, 0x25, 0x93, 0x77, 0x8d)]
interface IWiaItemExtras(IWiaItemExtrasVtbl): IUnknown(IUnknownVtbl) {
    fn GetExtendedErrorInfo(
        bstrErrorText: *mut BSTR,
    ) -> HRESULT,
    fn Escape(
        dwEscapeCode: DWORD,
        lpInData: *const BYTE,
        cbInDataSize: DWORD,
        pOutData: *mut BYTE,
        dwOutDataSize: DWORD,
        pdwActualDataSize: *mut DWORD,
    ) -> HRESULT,
    fn CancelPendingIO() -> HRESULT,
}}
DEFINE_GUID!{IID_IWiaAppErrorHandler,
    0x6C16186C, 0xD0A6, 0x400c, 0x80, 0xF4, 0xD2, 0x69, 0x86, 0xA0, 0xE7, 0x34}
RIDL!{#[uuid(0x6c16186c, 0xd0a6, 0x400c, 0x80, 0xf4, 0xd2, 0x69, 0x86, 0xa0, 0xe7, 0x34e)]
interface IWiaAppErrorHandler(IWiaAppErrorHandlerVtbl): IUnknown(IUnknownVtbl) {
    fn GetWindow(
        phwnd: *mut HWND,
    ) -> HRESULT,
    fn ReportStatus(
        lFlags: LONG,
        pWiaItem2: *mut IWiaItem2,
        hrStatus: HRESULT,
        lPercentComplete: LONG,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IWiaErrorHandler,
    0x0e4a51b1, 0xbc1f, 0x443d, 0xa8, 0x35, 0x72, 0xe8, 0x90, 0x75, 0x9e, 0xf3}
RIDL!{#[uuid(0x0e4a51b1, 0xbc1f, 0x443d, 0xa8, 0x35, 0x72, 0xe8, 0x90, 0x75, 0x9e, 0xf3)]
interface IWiaErrorHandler(IWiaErrorHandlerVtbl): IUnknown(IUnknownVtbl) {
    fn ReportStatus(
        lFlags: LONG,
        hwndParent: HWND,
        pWiaItem2: *mut IWiaItem2,
        hrStatus: HRESULT,
        lPercentComplete: LONG,
    ) -> HRESULT,
    fn GetStatusDescription(
        lFlags: LONG,
        pWiaItem2: *mut IWiaItem2,
        hrStatus: HRESULT,
        pbstrDescription: *mut BSTR,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IWiaTransfer,
    0xc39d6942, 0x2f4e, 0x4d04, 0x92, 0xfe, 0x4e, 0xf4, 0xd3, 0xa1, 0xde, 0x5a}
RIDL!{#[uuid(0xc39d6942, 0x2f4e, 0x4d04, 0x92, 0xfe, 0x4e, 0xf4, 0xd3, 0xa1, 0xde, 0x5a)]
interface IWiaTransfer(IWiaTransferVtbl): IUnknown(IUnknownVtbl) {
    fn Download(
        lFlags: LONG,
        pIWiaTransferCallback: *mut IWiaTransferCallback,
    ) -> HRESULT,
    fn Upload(
        lFlags: LONG,
        pSource: *mut IStream,
        pIWiaTransferCallback: *mut IWiaTransferCallback,
    ) -> HRESULT,
    fn Cancel() -> HRESULT,
    fn EnumWIA_FORMAT_INFO(
        ppEnum: *mut *mut IEnumWIA_FORMAT_INFO,
    ) -> HRESULT,
}}
STRUCT!{struct WiaTransferParams {
    lMessage: LONG,
    lPercentComplete: LONG,
    ulTransferredBytes: ULONG64,
    hrErrorStatus: HRESULT,
}}
DEFINE_GUID!{IID_IWiaTransferCallback,
    0x27d4eaaf, 0x28a6, 0x4ca5, 0x9a, 0xab, 0xe6, 0x78, 0x16, 0x8b, 0x95, 0x27}
RIDL!{#[uuid(0x27d4eaaf, 0x28a6, 0x4ca5, 0x9a, 0xab, 0xe6, 0x78, 0x16, 0x8b, 0x95, 0x27)]
interface IWiaTransferCallback(IWiaTransferCallbackVtbl): IUnknown(IUnknownVtbl) {
    fn TransferCallback(
        lFlags: LONG,
        pWiaTransferParams: *mut WiaTransferParams,
    ) -> HRESULT,
    fn GetNextStream(
        lFlags: LONG,
        bstrItemName: BSTR,
        bstrFullItemName: BSTR,
        ppDestination: *mut *mut IStream,
    ) -> HRESULT, 
}}
DEFINE_GUID!{IID_IWiaSegmentationFilter,
    0xEC46A697, 0xAC04, 0x4447, 0x8F, 0x65, 0xFF, 0x63, 0xD5, 0x15, 0x4B, 0x21}
RIDL!{#[uuid(0xec46a697, 0xac04, 0x4447, 0x8f, 0x65, 0xff, 0x63, 0xd5, 0x15, 0x4b, 0x21)]
interface IWiaSegmentationFilter(IWiaSegmentationFilterVtbl): IUnknown(IUnknownVtbl) {
    fn DetectRegions(
        lFlags: LONG,
        pInputStream: *mut IStream,
        pWiaItem2: *mut IWiaItem2,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IWiaImageFilter,
    0xA8A79FFA, 0x450B, 0x41f1, 0x8F, 0x87, 0x84, 0x9C, 0xCD, 0x94, 0xEB, 0xF6}
RIDL!{#[uuid(0xa8a79ffa, 0x450b, 0x41f1, 0x8f, 0x87, 0x84, 0x9c, 0xcd, 0x94, 0xeb, 0xf)]
interface IWiaImageFilter(IWiaImageFilterVtbl): IUnknown(IUnknownVtbl) {
    fn InitializeFilter(
        pWiaItem2: *mut IWiaItem2,
        pWiaTransferCallback: *mut IWiaTransferCallback,
    ) -> HRESULT,
    fn SetNewCallback(
        pWiaTransferCallback: *mut IWiaTransferCallback,
    ) -> HRESULT,
    fn FilterPreviewImage(
        lFlags: LONG,
        pWiaChildItem2: *mut IWiaItem2,
        InputImageExtents: RECT,
        pInputStream: *mut IStream,
    ) -> HRESULT,
    fn ApplyProperties(
        pWiaPropertyStorage: *mut IWiaPropertyStorage,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IWiaPreview,
    0x95C2B4FD, 0x33F2, 0x4d86, 0xAD, 0x40, 0x94, 0x31, 0xF0, 0xDF, 0x08, 0xF7}
RIDL!{#[uuid(0x95c2b4fd, 0x33f2, 0x4d86, 0xad, 0x40, 0x94, 0x31, 0xf0, 0xdf, 0x08, 0xf7)]
interface IWiaPreview(IWiaPreviewVtbl): IUnknown(IUnknownVtbl) {
    fn GetNewPreview(
        lFlags: LONG,
        pWiaItem2: *mut IWiaItem2,
        pWiaTransferCallback: *mut IWiaTransferCallback,
    ) -> HRESULT,
    fn UpdatePreview(
        lFlags: LONG,
        pChildWiaItem2: *mut IWiaItem2,
        pWiaTransferCallback: *mut IWiaTransferCallback,
    ) -> HRESULT,
    fn DetectRegions(
        lFlags: LONG,
    ) -> HRESULT,
    fn Clear() -> HRESULT,
}}
DEFINE_GUID!{IID_IEnumWiaItem2,
    0x59970AF4, 0xCD0D, 0x44d9, 0xAB, 0x24, 0x52, 0x29, 0x56, 0x30, 0xE5, 0x82}
RIDL!{#[uuid(0x59970af4, 0xcd0d, 0x44d9, 0xab, 0x24, 0x52, 0x29, 0x56, 0x30, 0xe5, 0x82)]
interface IEnumWiaItem2(IEnumWiaItem2Vtbl): IUnknown(IUnknownVtbl) {
    fn Next(
        celt: ULONG,
        ppIWiaItem2: *mut *mut IWiaItem2,
        pceltFetched: *mut ULONG,
    ) -> HRESULT,
    fn Skip(
        celt: ULONG,
    ) -> HRESULT,
    fn Reset() -> HRESULT,
    fn Clone(
        ppIEnum: *mut *mut IEnumWiaItem2,
    ) -> HRESULT,
    fn GetCount(
        celt: *mut ULONG,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IWiaItem2,
    0x6CBA0075, 0x1287, 0x407d, 0x9B, 0x77, 0xCF, 0x0E, 0x03, 0x04, 0x35, 0xCC}
RIDL!{#[uuid(0x6cba0075, 0x1287, 0x407d, 0x9b, 0x77, 0xcf, 0x0e, 0x03, 0x04, 0x35, 0xcc)]
interface IWiaItem2(IWiaItem2Vtbl): IUnknown(IUnknownVtbl) {
    fn CreateChildItem(
        lItemFlags: LONG,
        lCreationFlags: LONG,
        bstrItemName: BSTR,
        ppIWiaItem2: *mut *mut IWiaItem2,
    ) -> HRESULT,
    fn DeleteItem(
        lFlags: LONG,
    ) -> HRESULT,
    fn EnumChildItems(
        pCategoryGUID: *const GUID,
        ppIEnumWiaItem2: *mut *mut IEnumWiaItem2,
    ) -> HRESULT,
    fn FindItemByName(
        lFlags: LONG,
        bstrFullItemName: BSTR,
        ppIWiaItem2: *mut *mut IWiaItem2,
    ) -> HRESULT,
    fn GetItemCategory(
        pItemCategoryGUID: *const GUID,
    ) -> HRESULT,
    fn GetItemType(
        pItemType: *mut LONG,
    ) -> HRESULT,
    fn DeviceDlg(
        lFlags: LONG,
        hwndParent: HWND,
        bstrFolderName: BSTR,
        bstrFilename: BSTR,
        plNumFiles: *mut LONG,
        ppbstrFilePaths: *mut *mut BSTR,
        ppItem: *mut *mut IWiaItem2,
    ) -> HRESULT,
    fn DeviceCommand(
        lFlags: LONG,
        pCmdGUID: *const GUID,
    ) -> HRESULT,
    fn EnumDeviceCapabilities(
        lFlags: LONG,
        ppIEnumWIA_DEV_CAPS: *mut *mut IEnumWIA_DEV_CAPS,
    ) -> HRESULT,
    fn CheckExtension(
        lFlags: LONG,
        bstrName: BSTR,
        riidExtensionInterface: REFIID,
        pbExtensionExists: *mut BOOL,
    ) -> HRESULT,
    fn GetExtension(
        lFlags: LONG,
        bstrName: BSTR,
        riidExtensionInterface: REFIID,
        ppOut: *mut *mut c_void,
    ) -> HRESULT,
    fn GetParentItem(
        ppIWiaItem2: *mut *mut IWiaItem2,
    ) -> HRESULT,
    fn GetRootItem(
        ppIWiaItem2: *mut *mut IWiaItem2,
    ) -> HRESULT,
    fn GetPreviewComponent(
        lFlags: LONG,
        ppWiaPreview: *mut *mut IWiaPreview,
    ) -> HRESULT,
    fn EnumRegisterEventInfo(
        lFlags: LONG,
        pEventGUID: *const GUID,
        ppIEnum: *mut *mut IEnumWIA_DEV_CAPS,
    ) -> HRESULT,
    fn Diagnostic(
        ulSize: ULONG,
        pBuffer: *mut BYTE,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IWiaDevMgr2,
    0x79C07CF1, 0xCBDD, 0x41ee, 0x8E, 0xC3, 0xF0, 0x00, 0x80, 0xCA, 0xDA, 0x7A}
RIDL!{#[uuid(0x79c07cf1, 0xcbdd, 0x41ee, 0x8e, 0xc3, 0xf0, 0x00, 0x80, 0xca, 0xda, 0x7a)]
interface IWiaDevMgr2(IWiaDevMgr2Vtbl): IUnknown(IUnknownVtbl) {
    fn EnumDeviceInfo(
        lFlags: LONG,
        ppIEnum: *mut *mut IEnumWIA_DEV_INFO,
    ) -> HRESULT,
    fn CreateDevice(
        lFlags: LONG,
        bstrDeviceID: BSTR,
        ppWiaItem2Root: *mut *mut IWiaItem2,
    ) -> HRESULT,
    fn SelectDeviceDlg(
        hwndParent: HWND,
        lDeviceType: LONG,
        lFlags: LONG,
        pbstrDeviceID: *mut BSTR,
        ppItemRoot: *mut *mut IWiaItem2,
    ) -> HRESULT,
    fn SelectDeviceDlgID(
        hwndParent: HWND,
        lDeviceType: LONG,
        lFlags: LONG,
        pbstrDeviceID: *mut BSTR,
    ) -> HRESULT,
    fn RegisterEventCallbackInterface(
        lFlags: LONG,
        bstrDeviceID: BSTR,
        pEventGUID: *const GUID,
        pIWiaEventCallback: *mut IWiaEventCallback,
        pEventObject: *mut *mut IUnknown,
    ) -> HRESULT,
    fn RegisterEventCallbackProgram(
        lFlags: LONG,
        bstrDeviceID: BSTR,
        pEventGUID: *const GUID,
        bstrFullAppName: BSTR,
        bstrCommandline: BSTR,
        bstrName: BSTR,
        bstrDescription: BSTR,
        bstrIcon: BSTR,
    ) -> HRESULT,
    fn RegisterEventCallbackCLSID(
        lFlags: LONG,
        bstrDeviceID: BSTR,
        pEventGUID: *const GUID,
        pClsID: *const GUID,
        bstrName: BSTR,
        bstrDescription: BSTR,
        bstrIcon: BSTR,
    ) -> HRESULT,
    fn GetImageDlg(
        lFlags: LONG,
        bstrDeviceID: BSTR,
        hwndParent: HWND,
        bstrFolderName: BSTR,
        bstrFilename: BSTR,
        plNumFiles: *mut LONG,
        ppbstrFilePaths: *mut *mut BSTR,
        ppItem: *mut *mut IWiaItem2,
    ) -> HRESULT,
}}
DEFINE_GUID!{CLSID_WiaDevMgr,
    0xa1f4e726, 0x8cf1, 0x11d1, 0xbf, 0x92, 0x00, 0x60, 0x08, 0x1e, 0xd8, 0x11}
RIDL!{#[uuid(0xa1f4e726, 0x8cf1, 0x11d1, 0xbf, 0x92, 0x00, 0x60, 0x08, 0x1e, 0xd8, 0x11)]
        class WiaDevMgr;}
DEFINE_GUID!{CLSID_WiaDevMgr2,
    0xB6C292BC, 0x7C88, 0x41ee, 0x8B, 0x54, 0x8E, 0xC9, 0x26, 0x17, 0xE5, 0x99}
RIDL!{#[uuid(0xB6C292BC, 0x7C88, 0x41ee, 0x8B, 0x54, 0x8E, 0xC9, 0x26, 0x17, 0xE5, 0x99)]
        class WiaDevMgr2;}
DEFINE_GUID!{CLSID_WiaLog,
    0xA1E75357, 0x881A, 0x419e, 0x83, 0xE2, 0xBB, 0x16, 0xDB, 0x19, 0x7C, 0x68}
RIDL!{#[uuid(0xA1E75357, 0x881A, 0x419e, 0x83, 0xE2, 0xBB, 0x16, 0xDB, 0x19, 0x7C, 0x68)]
        class WiaLog;}

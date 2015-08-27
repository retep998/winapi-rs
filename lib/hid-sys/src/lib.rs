// Copyright © 2015, Alex Daniel Jones
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to hid.
#![cfg(windows)]
extern crate winapi;
use winapi::*;

pub type PHIDP_PREPARSED_DATA = *mut c_void;
pub type USAGE = USHORT;
pub type PUSAGE = *mut USAGE;
pub const HIDP_STATUS_SUCCESS: NTSTATUS = 0x00110000;
pub const HIDP_STATUS_NULL: NTSTATUS = 0x08110001;
pub const HIDP_STATUS_INVALID_PREPARSED_DATA: NTSTATUS = 0x0C110001;
pub const HIDP_STATUS_INVALID_REPORT_TYPE: NTSTATUS = 0x0C110002;
pub const HIDP_STATUS_INVALID_REPORT_LENGTH: NTSTATUS = 0x0C110003;
pub const HIDP_STATUS_USAGE_NOT_FOUND: NTSTATUS = 0x0C110004;
pub const HIDP_STATUS_VALUE_OUT_OF_RANGE: NTSTATUS = 0x0C110005;
pub const HIDP_STATUS_BAD_LOG_PHY_VALUES: NTSTATUS = 0x0C110006;
pub const HIDP_STATUS_BUFFER_TOO_SMALL: NTSTATUS = 0x0C110007;
pub const HIDP_STATUS_INTERNAL_ERROR: NTSTATUS = 0x0C110008;
pub const HIDP_STATUS_I8042_TRANS_UNKNOWN: NTSTATUS = 0x0C110009;
pub const HIDP_STATUS_INCOMPATIBLE_REPORT_ID: NTSTATUS = 0x0C11000A;
pub const HIDP_STATUS_NOT_VALUE_ARRAY: NTSTATUS = 0x0C11000B;
pub const HIDP_STATUS_IS_VALUE_ARRAY: NTSTATUS = 0x0C11000C;
pub const HIDP_STATUS_DATA_INDEX_NOT_FOUND: NTSTATUS = 0x0C11000D;
pub const HIDP_STATUS_DATA_INDEX_OUT_OF_RANGE: NTSTATUS = 0x0C11000E;
pub const HIDP_STATUS_BUTTON_NOT_PRESSED: NTSTATUS = 0x0C11000F;
pub const HIDP_STATUS_REPORT_DOES_NOT_EXIST: NTSTATUS = 0x0C110010;
pub const HIDP_STATUS_NOT_IMPLEMENTED: NTSTATUS = 0x0C110020;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HID_COLLECTION_INFORMATION {
    pub DescriptorSize: ULONG,
    pub Polled: BOOLEAN,
    pub Reserved1: [UCHAR; 1],
    pub VendorID: USHORT,
    pub ProductID: USHORT,
    pub VersionNumber: USHORT,
}
pub type PHID_COLLECTION_INFORMATION = *mut HID_COLLECTION_INFORMATION;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HIDP_UNKNOWN_TOKEN {
    pub Token: UCHAR,
    pub Reserved: [UCHAR;3],
    pub BitField: ULONG,
}
pub type PHIDP_UNKNOWN_TOKEN = *mut HIDP_UNKNOWN_TOKEN;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HIDP_EXTENDED_ATTRIBUTES {
    pub NumGlobalUnknowns: UCHAR,
    pub Reserved: [UCHAR;3],
    pub GlobalUnknowns: PHIDP_UNKNOWN_TOKEN,
    pub Data: [ULONG;1],
}
pub type PHIDP_EXTENDED_ATTRIBUTES = *mut HIDP_EXTENDED_ATTRIBUTES;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HIDP_LINK_COLLECTION_NODE {
    pub LinkUsage: USAGE,
    pub LinkUsagePage: USAGE,
    pub Parent: USHORT,
    pub NumberOfChildren: USHORT,
    pub NextSibling: USHORT,
    pub FirstChild: USHORT,
    pub CollectionType: ULONG,
    pub IsAlias: ULONG,
    pub Reserved: ULONG,
    pub UserContext: PVOID,
}
pub type PHIDP_LINK_COLLECTION_NODE = *mut HIDP_LINK_COLLECTION_NODE;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HIDP_CAPS
{
    pub Usage: USAGE,
    pub UsagePage: USAGE,
    pub InputReportByteLength: USHORT,
    pub OutputReportByteLength: USHORT,
    pub FeatureReportByteLength: USHORT,
    pub Reserved: [USHORT; 17],

    pub NumberLinkCollectionNodes: USHORT,

    pub NumberInputButtonCaps: USHORT,
    pub NumberInputValueCaps: USHORT,
    pub NumberInputDataIndices: USHORT,

    pub NumberOutputButtonCaps: USHORT,
    pub NumberOutputValueCaps: USHORT,
    pub NumberOutputDataIndices: USHORT,

    pub NumberFeatureButtonCaps: USHORT,
    pub NumberFeatureValueCaps: USHORT,
    pub NumberFeatureDataIndices: USHORT,
}

pub type PHIDP_CAPS = *mut HIDP_CAPS;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HIDP_BUTTON_CAPS
{
    pub UsagePage: USAGE,
    pub ReportID: UCHAR,
    pub IsAlias: BOOLEAN,

    pub BitField: USHORT,
    pub LinkCollection: USHORT,

    pub LinkUsage: USAGE,
    pub LinkUsagePage: USAGE,

    pub IsRange: BOOLEAN,
    pub IsStringRange: BOOLEAN,
    pub IsDesignatorRange: BOOLEAN,
    pub IsAbsolute: BOOLEAN,

    pub Reserved: [ULONG; 10],
    pub Range: RANGE_STRUCT,
}

pub type PHIDP_BUTTON_CAPS = *mut HIDP_BUTTON_CAPS;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct RANGE_STRUCT{
    pub UsageMin: USAGE,
    pub UsageMax: USAGE,
    pub StringMin: USHORT,
    pub StringMax: USHORT,
    pub DesignatorMin: USHORT,
    pub DesignatorMax: USHORT,
    pub DataIndexMin: USHORT,
    pub DataIndexMax: USHORT,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HIDP_VALUE_CAPS
{
    pub UsagePage: USAGE,
    pub ReportID: UCHAR,
    pub IsAlias: BOOLEAN,

    pub BitField: USHORT,
    pub LinkCollection: USHORT,

    pub LinkUsage: USAGE,
    pub LinkUsagePage: USAGE,

    pub IsRange: BOOLEAN,
    pub IsStringRange: BOOLEAN,
    pub IsDesignatorRange: BOOLEAN,
    pub IsAbsolute: BOOLEAN,

    pub HasNull: BOOLEAN,
    pub Reserved: UCHAR,
    pub BitSize: USHORT,

    pub ReportCount: USHORT,
    pub Reserved2: [USHORT; 5],

    pub UnitsExp: ULONG,
    pub Units: ULONG,

    pub LogicalMin: LONG,
    pub LogicalMax: LONG,
    pub PhysicalMin: LONG,
    pub PhysicalMax: LONG,

    pub Range: RANGE_STRUCT,
}
pub type PHIDP_VALUE_CAPS = *mut HIDP_VALUE_CAPS;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub enum HIDP_REPORT_TYPE {
  HidP_Input,
  HidP_Output,
  HidP_Feature,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HIDP_DATA {
  pub DataIndex: USHORT,
  pub Reserved: USHORT,
  pub RawValue: ULONG,
}
pub type PHIDP_DATA = *mut HIDP_DATA;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HIDD_ATTRIBUTES {
  pub Size: ULONG,
  pub VendorID: USHORT,
  pub ProductID: USHORT,
  pub VersionNumber: USHORT,
}
pub type PHIDD_ATTRIBUTES = *mut HIDD_ATTRIBUTES;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct USAGE_AND_PAGE {
    pub Usage: USAGE,
    pub UsagePage: USAGE,
}
pub type PUSAGE_AND_PAGE = *mut USAGE_AND_PAGE;

extern "system" {
    pub fn HidP_GetCaps(
        PreparsedData: PHIDP_PREPARSED_DATA,
        Capabilities: PHIDP_CAPS ) -> NTSTATUS;

    pub fn HidP_GetButtonCaps(
        ReportType: HIDP_REPORT_TYPE,
        ButtonCaps: PHIDP_BUTTON_CAPS,
        ButtonCapsLength: PUSHORT,
        PreparsedData: PHIDP_PREPARSED_DATA) -> NTSTATUS;

    pub fn HidP_GetValueCaps(
        ReportType: HIDP_REPORT_TYPE,
        ValueCaps: PHIDP_VALUE_CAPS,
        ValueCapsLength: PUSHORT,
        PreparsedData: PHIDP_PREPARSED_DATA) -> NTSTATUS;

    pub fn HidP_GetUsages(
        ReportType: HIDP_REPORT_TYPE,
        UsagePage: USAGE,
        LinkCollection: USHORT,
        UsageList: PUSAGE,
        UsageLength: PULONG,
        PreparsedData: PHIDP_PREPARSED_DATA,
        Report: PCHAR,
        ReportLength: ULONG) -> NTSTATUS;

    pub fn HidP_GetUsageValue(
        ReportType: HIDP_REPORT_TYPE,
        UsagePage: USAGE,
        LinkCollection: USHORT,
        Usage: USAGE,
        UsageValue: PULONG,
        PreparsedData: PHIDP_PREPARSED_DATA,
        Report: PCHAR,
        ReportLength: ULONG) -> NTSTATUS;

    pub fn HidP_GetData(
        ReportType: HIDP_REPORT_TYPE,
        DataList: PHIDP_DATA,
        DataLength: PULONG,
        PreparsedData: PHIDP_PREPARSED_DATA,
        Report: PCHAR,
        ReportLength: ULONG) -> NTSTATUS;

    pub fn HidD_GetProductString(
        HidDeviceObject: HANDLE,
        Buffer: PVOID,
        BufferLength: ULONG) -> BOOLEAN;

    pub fn HidD_GetSerialNumberString(
        HidDeviceObject: HANDLE,
        Buffer: PVOID,
        BufferLength: ULONG) -> BOOLEAN;

    pub fn HidD_GetManufacturerString(
        HidDeviceObject: HANDLE,
        Buffer: PVOID,
        BufferLength: ULONG) -> BOOLEAN;

    pub fn HidD_GetAttributes(
        HidDeviceObject: HANDLE,
        Attributes: PHIDD_ATTRIBUTES) -> BOOLEAN;

    pub fn HidD_FlushQueue(
        HidDeviceObject: HANDLE ) -> BOOLEAN;

    pub fn HidD_FreePreparsedData(
        PreparsedData: PHIDP_PREPARSED_DATA) -> BOOLEAN;

    pub fn HidD_GetFeature(
        HidDeviceObject: HANDLE,
        ReportBuffer: PVOID,
        ReportBufferLength: ULONG) -> BOOLEAN;

    pub fn HidD_GetHidGuid(
        HidGuid: LPGUID);

    pub fn HidD_GetIndexedString(
        HidDeviceObject: HANDLE,
        StringIndex: ULONG,
        Buffer: PVOID,
        BufferLength: ULONG) -> BOOLEAN;

    pub fn HidD_GetInputReport(
        HidDeviceObject: HANDLE,
        ReportBuffer: PVOID,
        ReportBufferLength: ULONG) -> BOOLEAN;

    pub fn HidD_GetNumInputBuffers(
        HidDeviceObject: HANDLE,
        NumberBuffers: PULONG) -> BOOLEAN;

    pub fn HidD_GetPhysicalDescriptor(
        HidDeviceObject: HANDLE,
        Buffer: PVOID,
        BufferLength: ULONG) -> BOOLEAN;

    pub fn HidD_GetPreparsedData(
        HidDeviceObject: HANDLE,
        PreparsedData: *mut PHIDP_PREPARSED_DATA) -> BOOLEAN;

    pub fn HidD_SetFeature(
        HidDeviceObject: HANDLE,
        ReportBuffer: PVOID,
        ReportBufferLength: ULONG) -> BOOLEAN;

    pub fn HidD_SetNumInputBuffers(
        HidDeviceObject: HANDLE,
        NumberBuffers: ULONG) -> BOOLEAN;

    pub fn HidD_SetOutputReport(
        HidDeviceObject: HANDLE,
        ReportBuffer: PVOID,
        ReportBufferLength: ULONG) -> BOOLEAN;

    pub fn HidP_GetExtendedAttributes(
        ReportType: HIDP_REPORT_TYPE,
        DataIndex: USHORT,
        PreparsedData: PHIDP_PREPARSED_DATA,
        Attributes: PHIDP_EXTENDED_ATTRIBUTES,
        LengthAttributes: PULONG) -> NTSTATUS;

    pub fn HidP_GetLinkCollectionNodes(
        LinkCollectionNodes: PHIDP_LINK_COLLECTION_NODE,
        LinkCollectionNodesLength: PULONG,
        PreparsedData: PHIDP_PREPARSED_DATA) -> NTSTATUS;

    pub fn HidP_GetScaledUsageValue(
        ReportType: HIDP_REPORT_TYPE,
        UsagePage: USAGE,
        LinkCollection: USHORT,
        Usage: USAGE,
        UsageValue: PLONG,
        PreparsedData: PHIDP_PREPARSED_DATA,
        Report: PCHAR,
        ReportLength: ULONG) -> NTSTATUS;

    pub fn HidP_GetSpecificButtonCaps(
        ReportType: HIDP_REPORT_TYPE,
        UsagePage: USAGE,
        LinkCollection: USHORT,
        Usage: USAGE,
        ButtonCaps: PHIDP_BUTTON_CAPS,
        ButtonCapsLength: PUSHORT,
        PreparsedData: PHIDP_PREPARSED_DATA) -> NTSTATUS;

    pub fn HidP_GetSpecificValueCaps(
        ReportType: HIDP_REPORT_TYPE,
        UsagePage: USAGE,
        LinkCollection: USHORT,
        Usage: USAGE,
        ValueCaps: PHIDP_VALUE_CAPS,
        ValueCapsLength: PUSHORT,
        PreparsedData: PHIDP_PREPARSED_DATA) -> NTSTATUS;

    pub fn HidP_GetUsageValueArray(
        ReportType: HIDP_REPORT_TYPE,
        UsagePage: USAGE,
        LinkCollection: USHORT,
        Usage: USAGE,
        UsageValue: PCHAR,
        UsageValueByteLength: USHORT,
        PreparsedData: PHIDP_PREPARSED_DATA,
        Report: PCHAR,
        ReportLength: ULONG) -> NTSTATUS;

    pub fn HidP_GetUsagesEx(
        ReportType: HIDP_REPORT_TYPE,
        LinkCollection: USHORT,
        ButtonList: PUSAGE_AND_PAGE,
        UsageLength: *mut ULONG,
        PreparsedData: PHIDP_PREPARSED_DATA,
        Report: PCHAR,
        ReportLength: ULONG) -> NTSTATUS;

    pub fn HidP_InitializeReportForID(
        ReportType: HIDP_REPORT_TYPE,
        ReportID: UCHAR,
        PreparsedData: PHIDP_PREPARSED_DATA,
        Report: PCHAR,
        ReportLength: ULONG) -> NTSTATUS;

    pub fn HidP_MaxDataListLength(
        ReportType: HIDP_REPORT_TYPE,
        PreparsedData: PHIDP_PREPARSED_DATA) -> ULONG;

    pub fn HidP_MaxUsageListLength(
        ReportType: HIDP_REPORT_TYPE,
        UsagePage: USAGE,
        PreparsedData: PHIDP_PREPARSED_DATA) -> ULONG;

    pub fn HidP_SetData(
        ReportType: HIDP_REPORT_TYPE,
        DataList: PHIDP_DATA,
        DataLength: PULONG,
        PreparsedData: PHIDP_PREPARSED_DATA,
        Report: PCHAR,
        ReportLength: ULONG) -> NTSTATUS;

    pub fn HidP_SetScaledUsageValue(
        ReportType: HIDP_REPORT_TYPE,
        UsagePage: USAGE,
        LinkCollection: USHORT,
        Usage: USAGE,
        UsageValue: LONG,
        PreparsedData: PHIDP_PREPARSED_DATA,
        Report: PCHAR,
        ReportLength: ULONG) -> NTSTATUS;

    pub fn HidP_SetUsageValue(
        ReportType: HIDP_REPORT_TYPE,
        UsagePage: USAGE,
        LinkCollection: USHORT,
        Usage: USAGE,
        UsageValue: ULONG,
        PreparsedData: PHIDP_PREPARSED_DATA,
        Report: PCHAR,
        ReportLength: ULONG) -> NTSTATUS;

    pub fn HidP_SetUsageValueArray(
        ReportType: HIDP_REPORT_TYPE,
        UsagePage: USAGE,
        LinkCollection: USHORT,
        Usage: USAGE,
        UsageValue: PCHAR,
        UsageValueByteLength: USHORT,
        PreparsedData: PHIDP_PREPARSED_DATA,
        Report: PCHAR,
        ReportLength: ULONG) -> NTSTATUS;

    pub fn HidP_SetUsages(
        ReportType: HIDP_REPORT_TYPE,
        UsagePage: USAGE,
        LinkCollection: USHORT,
        UsageList: PUSAGE,
        UsageLength: PULONG,
        PreparsedData: PHIDP_PREPARSED_DATA,
        Report: PCHAR,
        ReportLength: ULONG) -> NTSTATUS;

    pub fn HidP_UnsetUsages(
        ReportType: HIDP_REPORT_TYPE,
        UsagePage: USAGE,
        LinkCollection: USHORT,
        UsageList: PUSAGE,
        UsageLength: PULONG,
        PreparsedData: PHIDP_PREPARSED_DATA,
        Report: PCHAR,
        ReportLength: ULONG) -> NTSTATUS;

    pub fn HidP_UsageListDifference(
        PreviousUsageList: PUSAGE,
        CurrentUsageList: PUSAGE,
        BreakUsageList: PUSAGE,
        MakeUsageList: PUSAGE,
        UsageListLength: ULONG) -> NTSTATUS;

    // pub fn HidD_GetConfiguration();
    // pub fn HidD_SetConfiguration();
    // pub fn HidD_GetMsGenreDescriptor();
    // pub fn HidP_TranslateUsagesToI8042ScanCodes();
}

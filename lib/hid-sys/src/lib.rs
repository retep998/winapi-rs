// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to hid.
#![cfg(windows)]
extern crate winapi;
use winapi::*;

extern "system" {
    pub fn HidD_FlushQueue(HidDeviceObject: HANDLE) -> BOOLEAN;
    pub fn HidD_FreePreparsedData(PreparsedData: PHIDP_PREPARSED_DATA) -> BOOLEAN;
    pub fn HidD_GetAttributes(
        HidDeviceObject: HANDLE, Attributes: PHIDD_ATTRIBUTES,
    ) -> BOOLEAN;
    pub fn HidD_GetFeature(
        HidDeviceObject: HANDLE, ReportBuffer: PVOID, ReportBufferLength: ULONG,
    ) -> BOOLEAN;
    pub fn HidD_GetHidGuid(HidGuid: LPGUID);
    pub fn HidD_GetIndexedString(
        HidDeviceObject: HANDLE, StringIndex: ULONG, Buffer: PVOID, BufferLength: ULONG,
    ) -> BOOLEAN;
    pub fn HidD_GetInputReport(
        HidDeviceObject: HANDLE, ReportBuffer: PVOID, ReportBufferLength: ULONG,
    ) -> BOOLEAN;
    pub fn HidD_GetManufacturerString(
        HidDeviceObject: HANDLE, Buffer: PVOID, BufferLength: ULONG,
    ) -> BOOLEAN;
    pub fn HidD_GetNumInputBuffers(
        HidDeviceObject: HANDLE, NumberBuffers: PULONG,
    ) -> BOOLEAN;
    pub fn HidD_GetPhysicalDescriptor(
        HidDeviceObject: HANDLE, Buffer: PVOID, BufferLength: ULONG,
    ) -> BOOLEAN;
    pub fn HidD_GetPreparsedData(
        HidDeviceObject: HANDLE, PreparsedData: *mut PHIDP_PREPARSED_DATA,
    ) -> BOOLEAN;
    pub fn HidD_GetProductString(
        HidDeviceObject: HANDLE, Buffer: PVOID, BufferLength: ULONG,
    ) -> BOOLEAN;
    pub fn HidD_GetSerialNumberString(
        HidDeviceObject: HANDLE, Buffer: PVOID, BufferLength: ULONG,
    ) -> BOOLEAN;
    pub fn HidD_SetFeature(
        HidDeviceObject: HANDLE, ReportBuffer: PVOID, ReportBufferLength: ULONG,
    ) -> BOOLEAN;
    pub fn HidD_SetNumInputBuffers(
        HidDeviceObject: HANDLE, NumberBuffers: ULONG,
    ) -> BOOLEAN;
    pub fn HidD_SetOutputReport(
        HidDeviceObject: HANDLE, ReportBuffer: PVOID, ReportBufferLength: ULONG,
    ) -> BOOLEAN;
    pub fn HidP_GetButtonCaps(
        ReportType: HIDP_REPORT_TYPE, ButtonCaps: PHIDP_BUTTON_CAPS,
        ButtonCapsLength: PUSHORT, PreparsedData: PHIDP_PREPARSED_DATA,
    ) -> NTSTATUS;
    pub fn HidP_GetCaps(
        PreparsedData: PHIDP_PREPARSED_DATA, Capabilities: PHIDP_CAPS,
    ) -> NTSTATUS;
    pub fn HidP_GetData(
        ReportType: HIDP_REPORT_TYPE, DataList: PHIDP_DATA, DataLength: PULONG,
        PreparsedData: PHIDP_PREPARSED_DATA, Report: PCHAR,ReportLength: ULONG,
    ) -> NTSTATUS;
    pub fn HidP_GetExtendedAttributes(
        ReportType: HIDP_REPORT_TYPE, DataIndex: USHORT,
        PreparsedData: PHIDP_PREPARSED_DATA, Attributes: PHIDP_EXTENDED_ATTRIBUTES,
        LengthAttributes: PULONG,
    ) -> NTSTATUS;
    pub fn HidP_GetLinkCollectionNodes(
        LinkCollectionNodes: PHIDP_LINK_COLLECTION_NODE,
        LinkCollectionNodesLength: PULONG, PreparsedData: PHIDP_PREPARSED_DATA,
    ) -> NTSTATUS;
    pub fn HidP_GetScaledUsageValue(
        ReportType: HIDP_REPORT_TYPE, UsagePage: USAGE, LinkCollection: USHORT,
        Usage: USAGE, UsageValue: PLONG, PreparsedData: PHIDP_PREPARSED_DATA,
        Report: PCHAR, ReportLength: ULONG,
    ) -> NTSTATUS;
    pub fn HidP_GetSpecificButtonCaps(
        ReportType: HIDP_REPORT_TYPE, UsagePage: USAGE, LinkCollection: USHORT,
        Usage: USAGE, ButtonCaps: PHIDP_BUTTON_CAPS, ButtonCapsLength: PUSHORT,
        PreparsedData: PHIDP_PREPARSED_DATA,
    ) -> NTSTATUS;
    pub fn HidP_GetSpecificValueCaps(
        ReportType: HIDP_REPORT_TYPE, UsagePage: USAGE, LinkCollection: USHORT,
        Usage: USAGE, ValueCaps: PHIDP_VALUE_CAPS, ValueCapsLength: PUSHORT,
        PreparsedData: PHIDP_PREPARSED_DATA,
    ) -> NTSTATUS;
    pub fn HidP_GetUsages(
        ReportType: HIDP_REPORT_TYPE, UsagePage: USAGE, LinkCollection: USHORT,
        UsageList: PUSAGE, UsageLength: PULONG, PreparsedData: PHIDP_PREPARSED_DATA,
        Report: PCHAR, ReportLength: ULONG,
    ) -> NTSTATUS;
    pub fn HidP_GetUsagesEx(
        ReportType: HIDP_REPORT_TYPE, LinkCollection: USHORT, ButtonList: PUSAGE_AND_PAGE,
        UsageLength: *mut ULONG, PreparsedData: PHIDP_PREPARSED_DATA, Report: PCHAR,
        ReportLength: ULONG,
    ) -> NTSTATUS;
    pub fn HidP_GetUsageValue(
        ReportType: HIDP_REPORT_TYPE, UsagePage: USAGE, LinkCollection: USHORT,
        Usage: USAGE, UsageValue: PULONG, PreparsedData: PHIDP_PREPARSED_DATA,
        Report: PCHAR, ReportLength: ULONG,
    ) -> NTSTATUS;
    pub fn HidP_GetUsageValueArray(
        ReportType: HIDP_REPORT_TYPE, UsagePage: USAGE, LinkCollection: USHORT,
        Usage: USAGE, UsageValue: PCHAR, UsageValueByteLength: USHORT,
        PreparsedData: PHIDP_PREPARSED_DATA, Report: PCHAR, ReportLength: ULONG,
    ) -> NTSTATUS;
    pub fn HidP_GetValueCaps(
        ReportType: HIDP_REPORT_TYPE, ValueCaps: PHIDP_VALUE_CAPS,
        ValueCapsLength: PUSHORT, PreparsedData: PHIDP_PREPARSED_DATA,
    ) -> NTSTATUS;
    pub fn HidP_InitializeReportForID(
        ReportType: HIDP_REPORT_TYPE, ReportID: UCHAR,
        PreparsedData: PHIDP_PREPARSED_DATA, Report: PCHAR, ReportLength: ULONG,
    ) -> NTSTATUS;
    pub fn HidP_MaxDataListLength(
        ReportType: HIDP_REPORT_TYPE, PreparsedData: PHIDP_PREPARSED_DATA,
    ) -> ULONG;
    pub fn HidP_MaxUsageListLength(
        ReportType: HIDP_REPORT_TYPE, UsagePage: USAGE,
        PreparsedData: PHIDP_PREPARSED_DATA,
    ) -> ULONG;
    pub fn HidP_SetData(
        ReportType: HIDP_REPORT_TYPE, DataList: PHIDP_DATA, DataLength: PULONG,
        PreparsedData: PHIDP_PREPARSED_DATA, Report: PCHAR, ReportLength: ULONG,
    ) -> NTSTATUS;
    pub fn HidP_SetScaledUsageValue(
        ReportType: HIDP_REPORT_TYPE, UsagePage: USAGE, LinkCollection: USHORT,
        Usage: USAGE, UsageValue: LONG, PreparsedData: PHIDP_PREPARSED_DATA,
        Report: PCHAR, ReportLength: ULONG,
    ) -> NTSTATUS;
    pub fn HidP_SetUsageValue(
        ReportType: HIDP_REPORT_TYPE, UsagePage: USAGE, LinkCollection: USHORT,
        Usage: USAGE, UsageValue: ULONG, PreparsedData: PHIDP_PREPARSED_DATA,
        Report: PCHAR, ReportLength: ULONG,
    ) -> NTSTATUS;
    pub fn HidP_SetUsageValueArray(
        ReportType: HIDP_REPORT_TYPE, UsagePage: USAGE, LinkCollection: USHORT,
        Usage: USAGE, UsageValue: PCHAR, UsageValueByteLength: USHORT,
        PreparsedData: PHIDP_PREPARSED_DATA, Report: PCHAR, ReportLength: ULONG,
    ) -> NTSTATUS;
    pub fn HidP_SetUsages(
        ReportType: HIDP_REPORT_TYPE, UsagePage: USAGE, LinkCollection: USHORT,
        UsageList: PUSAGE, UsageLength: PULONG, PreparsedData: PHIDP_PREPARSED_DATA,
        Report: PCHAR, ReportLength: ULONG,
    ) -> NTSTATUS;
    pub fn HidP_UnsetUsages(
        ReportType: HIDP_REPORT_TYPE, UsagePage: USAGE, LinkCollection: USHORT,
        UsageList: PUSAGE, UsageLength: PULONG, PreparsedData: PHIDP_PREPARSED_DATA,
        Report: PCHAR, ReportLength: ULONG,
    ) -> NTSTATUS;
    pub fn HidP_UsageListDifference(
        PreviousUsageList: PUSAGE, CurrentUsageList: PUSAGE, BreakUsageList: PUSAGE,
        MakeUsageList: PUSAGE, UsageListLength: ULONG,
    ) -> NTSTATUS;
    // pub fn HidD_GetConfiguration();
    // pub fn HidD_SetConfiguration();
    // pub fn HidD_GetMsGenreDescriptor();
    // pub fn HidP_TranslateUsagesToI8042ScanCodes();
}

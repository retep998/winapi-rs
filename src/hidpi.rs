// Copyright © 2015, Alex Daniel Jones
// Licensed under the MIT License <LICENSE.md>
// Taken from hidpi.h
ENUM!{enum HIDP_REPORT_TYPE {
  HidP_Input,
  HidP_Output,
  HidP_Feature,
}}
STRUCT!{struct USAGE_AND_PAGE {
    Usage: ::USAGE,
    UsagePage: ::USAGE,
}}
pub type PUSAGE_AND_PAGE = *mut USAGE_AND_PAGE;
STRUCT!{struct HIDP_BUTTON_CAPS {
    UsagePage: ::USAGE,
    ReportID: ::UCHAR,
    IsAlias: ::BOOLEAN,
    BitField: ::USHORT,
    LinkCollection: ::USHORT,
    LinkUsage: ::USAGE,
    LinkUsagePage: ::USAGE,
    IsRange: ::BOOLEAN,
    IsStringRange: ::BOOLEAN,
    IsDesignatorRange: ::BOOLEAN,
    IsAbsolute: ::BOOLEAN,
    Reserved: [::ULONG; 10],
    S_un: [u8; 16],
}}
UNION!(HIDP_BUTTON_CAPS, S_un, Range, Range_mut, HIDP_RANGE_STRUCT);
UNION!(HIDP_BUTTON_CAPS, S_un, NotRange, NotRange_mut, HIDP_NOTRANGE_STRUCT);
pub type PHIDP_BUTTON_CAPS = *mut HIDP_BUTTON_CAPS;
STRUCT!{struct HIDP_RANGE_STRUCT {
    UsageMin: ::USAGE,
    UsageMax: ::USAGE,
    StringMin: ::USHORT,
    StringMax: ::USHORT,
    DesignatorMin: ::USHORT,
    DesignatorMax: ::USHORT,
    DataIndexMin: ::USHORT,
    DataIndexMax: ::USHORT,
}}
STRUCT!{struct HIDP_NOTRANGE_STRUCT {
    Usage: ::USAGE,
    Reserved1: ::USAGE,
    StringIndex: ::USHORT,
    Reserved2: ::USHORT,
    DesignatorIndex: ::USHORT,
    Reserved3: ::USHORT,
    DataIndex: ::USHORT,
    Reserved4: ::USHORT,
}}
STRUCT!{struct HIDP_VALUE_CAPS {
    UsagePage: ::USAGE,
    ReportID: ::UCHAR,
    IsAlias: ::BOOLEAN,
    BitField: ::USHORT,
    LinkCollection: ::USHORT,
    LinkUsage: ::USAGE,
    LinkUsagePage: ::USAGE,
    IsRange: ::BOOLEAN,
    IsStringRange: ::BOOLEAN,
    IsDesignatorRange: ::BOOLEAN,
    IsAbsolute: ::BOOLEAN,
    HasNull: ::BOOLEAN,
    Reserved: ::UCHAR,
    BitSize: ::USHORT,
    ReportCount: ::USHORT,
    Reserved2: [::USHORT; 5],
    UnitsExp: ::ULONG,
    Units: ::ULONG,
    LogicalMin: ::LONG,
    LogicalMax: ::LONG,
    PhysicalMin: ::LONG,
    PhysicalMax: ::LONG,
    S_un: [u8; 16],
}}
UNION!(HIDP_VALUE_CAPS, S_un, Range, Range_mut, HIDP_RANGE_STRUCT);
UNION!(HIDP_VALUE_CAPS, S_un, NotRange, NotRange_mut, HIDP_NOTRANGE_STRUCT);
pub type PHIDP_VALUE_CAPS = *mut HIDP_VALUE_CAPS;
STRUCT!{struct HIDP_LINK_COLLECTION_NODE {
    LinkUsage: ::USAGE,
    LinkUsagePage: ::USAGE,
    Parent: ::USHORT,
    NumberOfChildren: ::USHORT,
    NextSibling: ::USHORT,
    FirstChild: ::USHORT,
    bit_fields: ::ULONG,
    UserContext: ::PVOID,
}}
BITFIELD!(HIDP_LINK_COLLECTION_NODE bit_fields: ::ULONG [
    CollectionType set_CollectionType[0..8],
    IsAlias set_IsAlias[8..9],
    Reserved set_Reserved[9..32],
]);
pub type PHIDP_LINK_COLLECTION_NODE = *mut HIDP_LINK_COLLECTION_NODE;
STRUCT!{struct HIDP_CAPS {
    Usage: ::USAGE,
    UsagePage: ::USAGE,
    InputReportByteLength: ::USHORT,
    OutputReportByteLength: ::USHORT,
    FeatureReportByteLength: ::USHORT,
    Reserved: [::USHORT; 17],
    NumberLinkCollectionNodes: ::USHORT,
    NumberInputButtonCaps: ::USHORT,
    NumberInputValueCaps: ::USHORT,
    NumberInputDataIndices: ::USHORT,
    NumberOutputButtonCaps: ::USHORT,
    NumberOutputValueCaps: ::USHORT,
    NumberOutputDataIndices: ::USHORT,
    NumberFeatureButtonCaps: ::USHORT,
    NumberFeatureValueCaps: ::USHORT,
    NumberFeatureDataIndices: ::USHORT,
}}
pub type PHIDP_CAPS = *mut HIDP_CAPS;
STRUCT!{struct HIDP_DATA {
    DataIndex: ::USHORT,
    Reserved: ::USHORT,
    S_un: [u8; 4],
}}
UNION!(HIDP_DATA, S_un, RawValue, RawValue_mut, ::ULONG);
UNION!(HIDP_DATA, S_un, On, On_mut, ::BOOLEAN);
pub type PHIDP_DATA = *mut HIDP_DATA;
STRUCT!{struct HIDP_UNKNOWN_TOKEN {
    Token: ::UCHAR,
    Reserved: [::UCHAR; 3],
    BitField: ::ULONG,
}}
pub type PHIDP_UNKNOWN_TOKEN = *mut HIDP_UNKNOWN_TOKEN;
STRUCT!{struct HIDP_EXTENDED_ATTRIBUTES {
    NumGlobalUnknowns: ::UCHAR,
    Reserved: [::UCHAR; 3],
    GlobalUnknowns: ::PHIDP_UNKNOWN_TOKEN,
    Data: [::ULONG; 1],
}}
pub type PHIDP_EXTENDED_ATTRIBUTES = *mut HIDP_EXTENDED_ATTRIBUTES;
pub const HIDP_STATUS_SUCCESS: ::NTSTATUS = 0x00110000;
pub const HIDP_STATUS_NULL: ::NTSTATUS = 0x08110001;
pub const HIDP_STATUS_INVALID_PREPARSED_DATA: ::NTSTATUS = 0x0C110001;
pub const HIDP_STATUS_INVALID_REPORT_TYPE: ::NTSTATUS = 0x0C110002;
pub const HIDP_STATUS_INVALID_REPORT_LENGTH: ::NTSTATUS = 0x0C110003;
pub const HIDP_STATUS_USAGE_NOT_FOUND: ::NTSTATUS = 0x0C110004;
pub const HIDP_STATUS_VALUE_OUT_OF_RANGE: ::NTSTATUS = 0x0C110005;
pub const HIDP_STATUS_BAD_LOG_PHY_VALUES: ::NTSTATUS = 0x0C110006;
pub const HIDP_STATUS_BUFFER_TOO_SMALL: ::NTSTATUS = 0x0C110007;
pub const HIDP_STATUS_INTERNAL_ERROR: ::NTSTATUS = 0x0C110008;
pub const HIDP_STATUS_I8042_TRANS_UNKNOWN: ::NTSTATUS = 0x0C110009;
pub const HIDP_STATUS_INCOMPATIBLE_REPORT_ID: ::NTSTATUS = 0x0C11000A;
pub const HIDP_STATUS_NOT_VALUE_ARRAY: ::NTSTATUS = 0x0C11000B;
pub const HIDP_STATUS_IS_VALUE_ARRAY: ::NTSTATUS = 0x0C11000C;
pub const HIDP_STATUS_DATA_INDEX_NOT_FOUND: ::NTSTATUS = 0x0C11000D;
pub const HIDP_STATUS_DATA_INDEX_OUT_OF_RANGE: ::NTSTATUS = 0x0C11000E;
pub const HIDP_STATUS_BUTTON_NOT_PRESSED: ::NTSTATUS = 0x0C11000F;
pub const HIDP_STATUS_REPORT_DOES_NOT_EXIST: ::NTSTATUS = 0x0C110010;
pub const HIDP_STATUS_NOT_IMPLEMENTED: ::NTSTATUS = 0x0C110020;

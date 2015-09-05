// Copyright © 2015, Alex Daniel Jones
// Licensed under the MIT License <LICENSE.md>
// Taken from hidpi.h

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub enum HIDP_REPORT_TYPE {
  HidP_Input,
  HidP_Output,
  HidP_Feature,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct USAGE_AND_PAGE {
    pub Usage: ::USAGE,
    pub UsagePage: ::USAGE,
}
pub type PUSAGE_AND_PAGE = *mut USAGE_AND_PAGE;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HIDP_BUTTON_CAPS {
    pub UsagePage: ::USAGE,
    pub ReportID: ::UCHAR,
    pub IsAlias: ::BOOLEAN,

    pub BitField: ::USHORT,
    pub LinkCollection: ::USHORT,

    pub LinkUsage: ::USAGE,
    pub LinkUsagePage: ::USAGE,

    pub IsRange: ::BOOLEAN,
    pub IsStringRange: ::BOOLEAN,
    pub IsDesignatorRange: ::BOOLEAN,
    pub IsAbsolute: ::BOOLEAN,

    pub Reserved: [::ULONG; 10],
    pub S_un: [u8; 16],
}
UNION!(HIDP_BUTTON_CAPS, S_un, Range, Range_mut, HIDP_RANGE_STRUCT);
UNION!(HIDP_BUTTON_CAPS, S_un, NotRange, NotRange_mut, HIDP_NOTRANGE_STRUCT);
pub type PHIDP_BUTTON_CAPS = *mut HIDP_BUTTON_CAPS;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HIDP_RANGE_STRUCT {
    pub UsageMin: ::USAGE,
    pub UsageMax: ::USAGE,
    pub StringMin: ::USHORT,
    pub StringMax: ::USHORT,
    pub DesignatorMin: ::USHORT,
    pub DesignatorMax: ::USHORT,
    pub DataIndexMin: ::USHORT,
    pub DataIndexMax: ::USHORT,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HIDP_NOTRANGE_STRUCT {
    pub Usage: ::USAGE,
    pub Reserved1: ::USAGE,
    pub StringIndex: ::USHORT,
    pub Reserved2: ::USHORT,
    pub DesignatorIndex: ::USHORT,
    pub Reserved3: ::USHORT,
    pub DataIndex: ::USHORT,
    pub Reserved4: ::USHORT,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HIDP_VALUE_CAPS {
    pub UsagePage: ::USAGE,
    pub ReportID: ::UCHAR,
    pub IsAlias: ::BOOLEAN,

    pub BitField: ::USHORT,
    pub LinkCollection: ::USHORT,

    pub LinkUsage: ::USAGE,
    pub LinkUsagePage: ::USAGE,

    pub IsRange: ::BOOLEAN,
    pub IsStringRange: ::BOOLEAN,
    pub IsDesignatorRange: ::BOOLEAN,
    pub IsAbsolute: ::BOOLEAN,

    pub HasNull: ::BOOLEAN,
    pub Reserved: ::UCHAR,
    pub BitSize: ::USHORT,

    pub ReportCount: ::USHORT,
    pub Reserved2: [::USHORT; 5],

    pub UnitsExp: ::ULONG,
    pub Units: ::ULONG,

    pub LogicalMin: ::LONG,
    pub LogicalMax: ::LONG,
    pub PhysicalMin: ::LONG,
    pub PhysicalMax: ::LONG,

    pub S_un: [u8; 16],
}
UNION!(HIDP_VALUE_CAPS, S_un, Range, Range_mut, HIDP_RANGE_STRUCT);
UNION!(HIDP_VALUE_CAPS, S_un, NotRange, NotRange_mut, HIDP_NOTRANGE_STRUCT);
pub type PHIDP_VALUE_CAPS = *mut HIDP_VALUE_CAPS;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HIDP_LINK_COLLECTION_NODE {
    pub LinkUsage: ::USAGE,
    pub LinkUsagePage: ::USAGE,
    pub Parent: ::USHORT,
    pub NumberOfChildren: ::USHORT,
    pub NextSibling: ::USHORT,
    pub FirstChild: ::USHORT,
    pub bit_fields: ::ULONG,
    pub UserContext: ::PVOID,
}
BITFIELD!(HIDP_LINK_COLLECTION_NODE bit_fields: ::ULONG [
    CollectionType set_CollectionType[0..8],
    IsAlias set_IsAlias[8..9],
    Reserved set_Reserved[9..32],
]);
pub type PHIDP_LINK_COLLECTION_NODE = *mut HIDP_LINK_COLLECTION_NODE;


#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HIDP_CAPS {
    pub Usage: ::USAGE,
    pub UsagePage: ::USAGE,
    pub InputReportByteLength: ::USHORT,
    pub OutputReportByteLength: ::USHORT,
    pub FeatureReportByteLength: ::USHORT,
    pub Reserved: [::USHORT; 17],

    pub NumberLinkCollectionNodes: ::USHORT,

    pub NumberInputButtonCaps: ::USHORT,
    pub NumberInputValueCaps: ::USHORT,
    pub NumberInputDataIndices: ::USHORT,

    pub NumberOutputButtonCaps: ::USHORT,
    pub NumberOutputValueCaps: ::USHORT,
    pub NumberOutputDataIndices: ::USHORT,

    pub NumberFeatureButtonCaps: ::USHORT,
    pub NumberFeatureValueCaps: ::USHORT,
    pub NumberFeatureDataIndices: ::USHORT,
}
pub type PHIDP_CAPS = *mut HIDP_CAPS;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HIDP_DATA {
  pub DataIndex: ::USHORT,
  pub Reserved: ::USHORT,
  pub S_un: [u8; 4],
}
UNION!(HIDP_DATA, S_un, RawValue, RawValue_mut, ::ULONG);
UNION!(HIDP_DATA, S_un, On, On_mut, ::BOOLEAN);
pub type PHIDP_DATA = *mut HIDP_DATA;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HIDP_UNKNOWN_TOKEN {
    pub Token: ::UCHAR,
    pub Reserved: [::UCHAR; 3],
    pub BitField: ::ULONG,
}
pub type PHIDP_UNKNOWN_TOKEN = *mut HIDP_UNKNOWN_TOKEN;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HIDP_EXTENDED_ATTRIBUTES {
    pub NumGlobalUnknowns: ::UCHAR,
    pub Reserved: [::UCHAR; 3],
    pub GlobalUnknowns: ::PHIDP_UNKNOWN_TOKEN,
    pub Data: [::ULONG; 1],
}
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

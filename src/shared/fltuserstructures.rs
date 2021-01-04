// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Structures, types and definitions for filters that are common to both user and kernel mode environments.
use shared::minwindef::{ULONG, USHORT};
use shared::ntdef::NTSTATUS;
use um::winnt::{HANDLE, ULONGLONG, WCHAR};
pub type HFILTER = HANDLE;
pub type HFILTER_INSTANCE = HANDLE;
pub type HFILTER_VOLUME = HANDLE;
ENUM!{enum FLT_FILESYSTEM_TYPE {
    FLT_FSTYPE_UNKNOWN,
    FLT_FSTYPE_RAW,
    FLT_FSTYPE_NTFS,
    FLT_FSTYPE_FAT,
    FLT_FSTYPE_CDFS,
    FLT_FSTYPE_UDFS,
    FLT_FSTYPE_LANMAN,
    FLT_FSTYPE_WEBDAV,
    FLT_FSTYPE_RDPDR,
    FLT_FSTYPE_NFS,
    FLT_FSTYPE_MS_NETWARE,
    FLT_FSTYPE_NETWARE,
    FLT_FSTYPE_BSUDF,
    FLT_FSTYPE_MUP,
    FLT_FSTYPE_RSFX,
    FLT_FSTYPE_ROXIO_UDF1,
    FLT_FSTYPE_ROXIO_UDF2,
    FLT_FSTYPE_ROXIO_UDF3,
    FLT_FSTYPE_TACIT,
    FLT_FSTYPE_FS_REC,
    FLT_FSTYPE_INCD,
    FLT_FSTYPE_INCD_FAT,
    FLT_FSTYPE_EXFAT,
    FLT_FSTYPE_PSFS,
    FLT_FSTYPE_GPFS,
    FLT_FSTYPE_NPFS,
    FLT_FSTYPE_MSFS,
    FLT_FSTYPE_CSVFS,
    FLT_FSTYPE_REFS,
    FLT_FSTYPE_OPENAFS,
    FLT_FSTYPE_CIMFS,
}}
pub type PFLT_FILESYSTEM_TYPE = *mut FLT_FILESYSTEM_TYPE;
ENUM!{enum FILTER_INFORMATION_CLASS {
    FilterFullInformation,
    FilterAggregateBasicInformation,
    FilterAggregateStandardInformation,
}}
pub type PFILTER_INFORMATION_CLASS = *mut FILTER_INFORMATION_CLASS;
STRUCT!{struct FILTER_FULL_INFORMATION {
    NextEntryOffset: ULONG,
    FrameID: ULONG,
    NumberOfInstances: ULONG,
    FilterNameLength: USHORT,
    FilterNameBuffer: [WCHAR; 1],
}}
pub type PFILTER_FULL_INFORMATION = *mut FILTER_FULL_INFORMATION;
STRUCT!{struct FILTER_AGGREGATE_BASIC_INFORMATION_Type_MiniFilter {
    FrameID: ULONG,
    NumberOfInstances: ULONG,
    FilterNameLength: USHORT,
    FilterNameBufferOffset: USHORT,
    FilterAltitudeLength: USHORT,
    FilterAltitudeBufferOffset: USHORT,
}}
STRUCT!{struct FILTER_AGGREGATE_BASIC_INFORMATION_Type_LegacyFilter {
    FilterNameLength: USHORT,
    FilterNameBufferOffset: USHORT,
}}
UNION!{union FILTER_AGGREGATE_BASIC_INFORMATION_Type {
    [u32; 4],
    MiniFilter MiniFilter_mut: FILTER_AGGREGATE_BASIC_INFORMATION_Type_MiniFilter,
    LegacyFilter LegacyFilter_mut: FILTER_AGGREGATE_BASIC_INFORMATION_Type_LegacyFilter,
}}
STRUCT!{struct FILTER_AGGREGATE_BASIC_INFORMATION {
    NextEntryOffset: ULONG,
    Flags: ULONG,
    Type: FILTER_AGGREGATE_BASIC_INFORMATION_Type,
}}
pub type PFILTER_AGGREGATE_BASIC_INFORMATION = *mut FILTER_AGGREGATE_BASIC_INFORMATION;
pub const FLTFL_AGGREGATE_INFO_IS_MINIFILTER: ULONG = 0x00000001;
pub const FLTFL_AGGREGATE_INFO_IS_LEGACYFILTER: ULONG = 0x00000002;
STRUCT!{struct FILTER_AGGREGATE_STANDARD_INFORMATION_Type_MiniFilter {
    Flags: ULONG,
    FrameID: ULONG,
    NumberOfInstances: ULONG,
    FilterNameLength: USHORT,
    FilterNameBufferOffset: USHORT,
    FilterAltitudeLength: USHORT,
    FilterAltitudeBufferOffset: USHORT,
}}
STRUCT!{struct FILTER_AGGREGATE_STANDARD_INFORMATION_Type_LegacyFilter {
    Flags: ULONG,
    FilterNameLength: USHORT,
    FilterNameBufferOffset: USHORT,
    FilterAltitudeLength: USHORT,
    FilterAltitudeBufferOffset: USHORT,
}}
UNION!{union FILTER_AGGREGATE_STANDARD_INFORMATION_Type {
    [u32; 5],
    MiniFilter MiniFilter_mut: FILTER_AGGREGATE_STANDARD_INFORMATION_Type_MiniFilter,
    LegacyFilter LegacyFilter_mut: FILTER_AGGREGATE_STANDARD_INFORMATION_Type_LegacyFilter,
}}
STRUCT!{struct FILTER_AGGREGATE_STANDARD_INFORMATION {
    NextEntryOffset: ULONG,
    Flags: ULONG,
    Type: FILTER_AGGREGATE_STANDARD_INFORMATION_Type,
}}
pub type PFILTER_AGGREGATE_STANDARD_INFORMATION = *mut FILTER_AGGREGATE_STANDARD_INFORMATION;
pub const FLTFL_ASI_IS_MINIFILTER: ULONG = 0x00000001;
pub const FLTFL_ASI_IS_LEGACYFILTER: ULONG = 0x00000002;
ENUM!{enum FILTER_VOLUME_INFORMATION_CLASS {
    FilterVolumeBasicInformation,
    FilterVolumeStandardInformation,
}}
pub type PFILTER_VOLUME_INFORMATION_CLASS = *mut FILTER_VOLUME_INFORMATION_CLASS;
STRUCT!{struct FILTER_VOLUME_BASIC_INFORMATION {
    FilterVolumeNameLength: USHORT,
    FilterVolumeName: [WCHAR; 1],
}}
pub type PFILTER_VOLUME_BASIC_INFORMATION = *mut FILTER_VOLUME_BASIC_INFORMATION;
STRUCT!{struct FILTER_VOLUME_STANDARD_INFORMATION {
    NextEntryOffset: ULONG,
    Flags: ULONG,
    FrameID: ULONG,
    FileSystemType: FLT_FILESYSTEM_TYPE,
    FilterVolumeNameLength: USHORT,
    FilterVolumeName: [WCHAR; 1],
}}
pub type PFILTER_VOLUME_STANDARD_INFORMATION = *mut FILTER_VOLUME_STANDARD_INFORMATION;
pub const FLTFL_VSI_DETACHED_VOLUME: ULONG = 0x00000001;
ENUM!{enum INSTANCE_INFORMATION_CLASS {
    InstanceBasicInformation,
    InstancePartialInformation,
    InstanceFullInformation,
    InstanceAggregateStandardInformation,
}}
pub type PINSTANCE_INFORMATION_CLASS = *mut INSTANCE_INFORMATION_CLASS;
STRUCT!{struct INSTANCE_BASIC_INFORMATION {
    NextEntryOffset: ULONG,
    InstanceNameLength: USHORT,
    InstanceNameBufferOffset: USHORT,
}}
pub type PINSTANCE_BASIC_INFORMATION = *mut INSTANCE_BASIC_INFORMATION;
STRUCT!{struct INSTANCE_PARTIAL_INFORMATION {
    NextEntryOffset: ULONG,
    InstanceNameLength: USHORT,
    InstanceNameBufferOffset: USHORT,
    AltitudeLength: USHORT,
    AltitudeBufferOffset: USHORT,
}}
pub type PINSTANCE_PARTIAL_INFORMATION = *mut INSTANCE_PARTIAL_INFORMATION;
STRUCT!{struct INSTANCE_FULL_INFORMATION {
    NextEntryOffset: ULONG,
    InstanceNameLength: USHORT,
    InstanceNameBufferOffset: USHORT,
    AltitudeLength: USHORT,
    AltitudeBufferOffset: USHORT,
    VolumeNameLength: USHORT,
    VolumeNameBufferOffset: USHORT,
    FilterNameLength: USHORT,
    FilterNameBufferOffset: USHORT,
}}
pub type PINSTANCE_FULL_INFORMATION = *mut INSTANCE_FULL_INFORMATION;
STRUCT!{struct INSTANCE_AGGREGATE_STANDARD_INFORMATION_Type_MiniFilter {
    Flags: ULONG,
    FrameID: ULONG,
    VolumeFileSystemType: FLT_FILESYSTEM_TYPE,
    InstanceNameLength: USHORT,
    InstanceNameBufferOffset: USHORT,
    AltitudeLength: USHORT,
    AltitudeBufferOffset: USHORT,
    VolumeNameLength: USHORT,
    VolumeNameBufferOffset: USHORT,
    FilterNameLength: USHORT,
    FilterNameBufferOffset: USHORT,
    SupportedFeatures: ULONG,
}}
pub const FLTFL_IASIM_DETACHED_VOLUME: ULONG = 0x00000001;
STRUCT!{struct INSTANCE_AGGREGATE_STANDARD_INFORMATION_Type_LegacyFilter {
    Flags: ULONG,
    AltitudeLength: USHORT,
    AltitudeBufferOffset: USHORT,
    VolumeNameLength: USHORT,
    VolumeNameBufferOffset: USHORT,
    FilterNameLength: USHORT,
    FilterNameBufferOffset: USHORT,
    SupportedFeatures: ULONG,
}}
pub const FLTFL_IASIL_DETACHED_VOLUME: ULONG = 0x00000001;
UNION!{union INSTANCE_AGGREGATE_STANDARD_INFORMATION_Type {
    [u32; 8],
    MiniFilter MiniFilter_mut: INSTANCE_AGGREGATE_STANDARD_INFORMATION_Type_MiniFilter,
    LegacyFilter LegacyFilter_mut: INSTANCE_AGGREGATE_STANDARD_INFORMATION_Type_LegacyFilter,
}}
STRUCT!{struct INSTANCE_AGGREGATE_STANDARD_INFORMATION {
    NextEntryOffset: ULONG,
    Flags: ULONG,
    Type: INSTANCE_AGGREGATE_STANDARD_INFORMATION_Type,
}}
pub type PINSTANCE_AGGREGATE_STANDARD_INFORMATION = *mut INSTANCE_AGGREGATE_STANDARD_INFORMATION;
pub const FLTFL_IASI_IS_MINIFILTER: ULONG = 0x00000001;
pub const FLTFL_IASI_IS_LEGACYFILTER: ULONG = 0x00000002;
STRUCT!{struct FILTER_MESSAGE_HEADER {
    ReplyLength: ULONG,
    MessageId: ULONGLONG,
}}
pub type PFILTER_MESSAGE_HEADER = *mut FILTER_MESSAGE_HEADER;
STRUCT!{struct FILTER_REPLY_HEADER {
    Status: NTSTATUS,
    MessageId: ULONGLONG,
}}
pub type PFILTER_REPLY_HEADER = *mut FILTER_REPLY_HEADER;

// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::minwindef::{ULONG, USHORT};
use shared::ntdef::NTSTATUS;
use um::winnt::{HANDLE, ULONGLONG, WCHAR};
pub type HFILTER = HANDLE;
pub type HFILTER_INSTANCE = HANDLE;
pub type HFILTER_VOLUME = HANDLE;

ENUM!{enum FLT_FILESYSTEM_TYPE {
    FLT_FSTYPE_UNKNOWN,         //an UNKNOWN file system type
    FLT_FSTYPE_RAW,             //Microsoft's RAW file system       (\FileSystem\RAW)
    FLT_FSTYPE_NTFS,            //Microsoft's NTFS file system      (\FileSystem\Ntfs)
    FLT_FSTYPE_FAT,             //Microsoft's FAT file system       (\FileSystem\Fastfat)
    FLT_FSTYPE_CDFS,            //Microsoft's CDFS file system      (\FileSystem\Cdfs)
    FLT_FSTYPE_UDFS,            //Microsoft's UDFS file system      (\FileSystem\Udfs)
    FLT_FSTYPE_LANMAN,          //Microsoft's LanMan Redirector     (\FileSystem\MRxSmb)
    FLT_FSTYPE_WEBDAV,          //Microsoft's WebDav redirector     (\FileSystem\MRxDav)
    FLT_FSTYPE_RDPDR,           //Microsoft's Terminal Server redirector    (\Driver\rdpdr)
    FLT_FSTYPE_NFS,             //Microsoft's NFS file system       (\FileSystem\NfsRdr)
    FLT_FSTYPE_MS_NETWARE,      //Microsoft's NetWare redirector    (\FileSystem\nwrdr)
    FLT_FSTYPE_NETWARE,         //Novell's NetWare redirector
    FLT_FSTYPE_BSUDF,           //The BsUDF CD-ROM driver           (\FileSystem\BsUDF)
    FLT_FSTYPE_MUP,             //Microsoft's Mup redirector        (\FileSystem\Mup)
    FLT_FSTYPE_RSFX,            //Microsoft's WinFS redirector      (\FileSystem\RsFxDrv)
    FLT_FSTYPE_ROXIO_UDF1,      //Roxio's UDF writeable file system (\FileSystem\cdudf_xp)
    FLT_FSTYPE_ROXIO_UDF2,      //Roxio's UDF readable file system  (\FileSystem\UdfReadr_xp)
    FLT_FSTYPE_ROXIO_UDF3,      //Roxio's DVD file system           (\FileSystem\DVDVRRdr_xp)
    FLT_FSTYPE_TACIT,           //Tacit FileSystem                  (\Device\TCFSPSE)
    FLT_FSTYPE_FS_REC,          //Microsoft's File system recognizer (\FileSystem\Fs_rec)
    FLT_FSTYPE_INCD,            //Nero's InCD file system           (\FileSystem\InCDfs)
    FLT_FSTYPE_INCD_FAT,        //Nero's InCD FAT file system       (\FileSystem\InCDFat)
    FLT_FSTYPE_EXFAT,           //Microsoft's EXFat FILE SYSTEM     (\FileSystem\exfat)
    FLT_FSTYPE_PSFS,            //PolyServ's file system            (\FileSystem\psfs)
    FLT_FSTYPE_GPFS,            //IBM General Parallel File System  (\FileSystem\gpfs)
    FLT_FSTYPE_NPFS,            //Microsoft's Named Pipe file system(\FileSystem\npfs)
    FLT_FSTYPE_MSFS,            //Microsoft's Mailslot file system  (\FileSystem\msfs)
    FLT_FSTYPE_CSVFS,           //Microsoft's Cluster Shared Volume file system  (\FileSystem\csvfs)
    FLT_FSTYPE_REFS,            //Microsoft's ReFS file system      (\FileSystem\Refs or \FileSystem\Refsv1)
    FLT_FSTYPE_OPENAFS,         //OpenAFS file system               (\Device\AFSRedirector)
    FLT_FSTYPE_CIMFS,            //Composite Image file system       (\FileSystem\cimfs)
}}

ENUM!{enum FILTER_INFORMATION_CLASS {
    FilterFullInformation,
    FilterAggregateBasicInformation,
    FilterAggregateStandardInformation,
}}

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


pub const FLTFL_AGGREGATE_INFO_IS_MINIFILTER: ULONG = 0x00000001;
pub const FLTFL_AGGREGATE_INFO_IS_LEGACYFILTER: ULONG = 0x00000002;

STRUCT!{struct FILTER_AGGREGATE_BASIC_INFORMATION {
    NextEntryOffset: ULONG,
    Flags: ULONG,
    Type: FILTER_AGGREGATE_BASIC_INFORMATION_Type,
}}

pub type PFILTER_AGGREGATE_BASIC_INFORMATION = *mut FILTER_AGGREGATE_BASIC_INFORMATION;


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

pub const FLTFL_ASI_IS_MINIFILTER: ULONG = 0x00000001;
pub const FLTFL_ASI_IS_LEGACYFILTER: ULONG = 0x00000002;

STRUCT!{struct FILTER_AGGREGATE_STANDARD_INFORMATION {
    NextEntryOffset: ULONG,
    Flags: ULONG,
    Type: FILTER_AGGREGATE_STANDARD_INFORMATION_Type,
}}

pub type PFILTER_AGGREGATE_STANDARD_INFORMATION = *mut FILTER_AGGREGATE_STANDARD_INFORMATION;

ENUM!{enum FILTER_VOLUME_INFORMATION_CLASS {
    FilterVolumeBasicInformation,
    FilterVolumeStandardInformation,
}}

STRUCT!{struct FILTER_VOLUME_BASIC_INFORMATION {
    FilterVolumeNameLength: USHORT,
    FilterVolumeName: [WCHAR; 1],
 }}

 pub type PFILTER_VOLUME_BASIC_INFORMATION = *mut FILTER_VOLUME_BASIC_INFORMATION;

 pub const FLTFL_VSI_DETACHED_VOLUME: ULONG = 0x00000001;

 STRUCT!{struct FILTER_VOLUME_STANDARD_INFORMATION {
    NextEntryOffset: ULONG,
    Flags: ULONG,
    FrameID: ULONG,
    FileSystemType: FLT_FILESYSTEM_TYPE,
    FilterVolumeNameLength: USHORT,
    FilterVolumeName: [WCHAR; 1],
 }}

 pub type PFILTER_VOLUME_STANDARD_INFORMATION = *mut FILTER_VOLUME_STANDARD_INFORMATION;

 ENUM!{enum INSTANCE_INFORMATION_CLASS {
    InstanceBasicInformation,
    InstancePartialInformation,
    InstanceFullInformation,
    InstanceAggregateStandardInformation,
}}

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

pub const FLTFL_IASIM_DETACHED_VOLUME: ULONG = 0x00000001;

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

pub const FLTFL_IASIL_DETACHED_VOLUME: ULONG = 0x00000001;

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

UNION!{union INSTANCE_AGGREGATE_STANDARD_INFORMATION_Type {
    [u32; 8],
    MiniFilter MiniFilter_mut: INSTANCE_AGGREGATE_STANDARD_INFORMATION_Type_MiniFilter,
    LegacyFilter LegacyFilter_mut: INSTANCE_AGGREGATE_STANDARD_INFORMATION_Type_LegacyFilter,
}}

pub const FLTFL_IASI_IS_MINIFILTER: ULONG = 0x00000001;
pub const FLTFL_IASI_IS_LEGACYFILTER: ULONG = 0x00000002;

STRUCT!{struct INSTANCE_AGGREGATE_STANDARD_INFORMATION {
    NextEntryOffset: ULONG,
    Flags: ULONG,
    Type: INSTANCE_AGGREGATE_STANDARD_INFORMATION_Type,
}}

pub type PINSTANCE_AGGREGATE_STANDARD_INFORMATION = *mut INSTANCE_AGGREGATE_STANDARD_INFORMATION;

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
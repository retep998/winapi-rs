// Copyright © 2015-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::basetsd::ULONG64;
use shared::devpropdef::DEVPROPKEY;
use shared::guiddef::GUID;
use shared::minwindef::{UCHAR, ULONG, USHORT};
use shared::ntdef::{CHAR, LARGE_INTEGER, LONGLONG, SHORT, ULONGLONG, WCHAR};
use um::winioctl::{
    DEVICE_TYPE, FILE_ANY_ACCESS, FILE_DEVICE_MASS_STORAGE, FILE_READ_ACCESS,
    FILE_WRITE_ACCESS, METHOD_BUFFERED,
};
use um::winnt::{ANYSIZE_ARRAY, BOOLEAN};
DEFINE_GUID!(GUID_DEVINTERFACE_DISK,
    0x53f56307, 0xb6bf, 0x11d0, 0x94, 0xf2, 0x00, 0xa0, 0xc9, 0x1e, 0xfb, 0x8b);
DEFINE_GUID!(GUID_DEVINTERFACE_CDROM,
    0x53f56308, 0xb6bf, 0x11d0, 0x94, 0xf2, 0x00, 0xa0, 0xc9, 0x1e, 0xfb, 0x8b);
DEFINE_GUID!(GUID_DEVINTERFACE_PARTITION,
    0x53f5630a, 0xb6bf, 0x11d0, 0x94, 0xf2, 0x00, 0xa0, 0xc9, 0x1e, 0xfb, 0x8b);
DEFINE_GUID!(GUID_DEVINTERFACE_TAPE,
    0x53f5630b, 0xb6bf, 0x11d0, 0x94, 0xf2, 0x00, 0xa0, 0xc9, 0x1e, 0xfb, 0x8b);
DEFINE_GUID!(GUID_DEVINTERFACE_WRITEONCEDISK,
    0x53f5630c, 0xb6bf, 0x11d0, 0x94, 0xf2, 0x00, 0xa0, 0xc9, 0x1e, 0xfb, 0x8b);
DEFINE_GUID!(GUID_DEVINTERFACE_VOLUME,
    0x53f5630d, 0xb6bf, 0x11d0, 0x94, 0xf2, 0x00, 0xa0, 0xc9, 0x1e, 0xfb, 0x8b);
DEFINE_GUID!(GUID_DEVINTERFACE_MEDIUMCHANGER,
    0x53f56310, 0xb6bf, 0x11d0, 0x94, 0xf2, 0x00, 0xa0, 0xc9, 0x1e, 0xfb, 0x8b);
DEFINE_GUID!(GUID_DEVINTERFACE_FLOPPY,
    0x53f56311, 0xb6bf, 0x11d0, 0x94, 0xf2, 0x00, 0xa0, 0xc9, 0x1e, 0xfb, 0x8b);
DEFINE_GUID!(GUID_DEVINTERFACE_CDCHANGER,
    0x53f56312, 0xb6bf, 0x11d0, 0x94, 0xf2, 0x00, 0xa0, 0xc9, 0x1e, 0xfb, 0x8b);
DEFINE_GUID!(GUID_DEVINTERFACE_STORAGEPORT,
    0x2accfe60, 0xc130, 0x11d2, 0xb0, 0x82, 0x00, 0xa0, 0xc9, 0x1e, 0xfb, 0x8b);
DEFINE_GUID!(GUID_DEVINTERFACE_VMLUN,
    0x6f416619, 0x9f29, 0x42a5, 0xb2, 0x0b, 0x37, 0xe2, 0x19, 0xca, 0x02, 0xb0);
DEFINE_GUID!(GUID_DEVINTERFACE_SES,
    0x1790c9ec, 0x47d5, 0x4df3, 0xb5, 0xaf, 0x9a, 0xdf, 0x3c, 0xf2, 0x3e, 0x48);
DEFINE_GUID!(WDI_STORAGE_PREDICT_FAILURE_DPS_GUID,
    0xe9f2d03a, 0x747c, 0x41c2, 0xbb, 0x9a, 0x02, 0xc6, 0x2b, 0x6d, 0x5f, 0xcb);
DEFINE_GUID!(GUID_DEVINTERFACE_HIDDEN_VOLUME,
    0x7f108a28, 0x9833, 0x4b3b, 0xb7, 0x80, 0x2c, 0x6b, 0x5f, 0xa5, 0xc0, 0x62);
DEFINE_GUID!(GUID_DEVINTERFACE_UNIFIED_ACCESS_RPMB,
    0x27447c21, 0xbcc3, 0x4d07, 0xa0, 0x5b, 0xa3, 0x39, 0x5b, 0xb4, 0xee, 0xe7);
pub const DiskClassGuid: GUID = GUID_DEVINTERFACE_DISK;
pub const CdRomClassGuid: GUID = GUID_DEVINTERFACE_CDROM;
pub const PartitionClassGuid: GUID = GUID_DEVINTERFACE_PARTITION;
pub const TapeClassGuid: GUID = GUID_DEVINTERFACE_TAPE;
pub const WriteOnceDiskClassGuid: GUID = GUID_DEVINTERFACE_WRITEONCEDISK;
pub const VolumeClassGuid: GUID = GUID_DEVINTERFACE_VOLUME;
pub const MediumChangerClassGuid: GUID = GUID_DEVINTERFACE_MEDIUMCHANGER;
pub const FloppyClassGuid: GUID = GUID_DEVINTERFACE_FLOPPY;
pub const CdChangerClassGuid: GUID = GUID_DEVINTERFACE_CDCHANGER;
pub const StoragePortClassGuid: GUID = GUID_DEVINTERFACE_STORAGEPORT;
pub const HiddenVolumeClassGuid: GUID = GUID_DEVINTERFACE_HIDDEN_VOLUME;
DEFINE_GUID!(GUID_DEVICEDUMP_STORAGE_DEVICE,
    0xd8e2592f, 0x1aab, 0x4d56, 0xa7, 0x46, 0x1f, 0x75, 0x85, 0xdf, 0x40, 0xf4);
DEFINE_GUID!(GUID_DEVICEDUMP_DRIVER_STORAGE_PORT,
    0xda82441d, 0x7142, 0x4bc1, 0xb8, 0x44, 0x08, 0x07, 0xc5, 0xa4, 0xb6, 0x7f);
DEFINE_DEVPROPKEY!(DEVPKEY_Storage_Portable,
    0x4d1ebee8, 0x803, 0x4774, 0x98, 0x42, 0xb7, 0x7d, 0xb5, 0x2, 0x65, 0xe9, 2);
DEFINE_DEVPROPKEY!(DEVPKEY_Storage_Removable_Media,
    0x4d1ebee8, 0x803, 0x4774, 0x98, 0x42, 0xb7, 0x7d, 0xb5, 0x2, 0x65, 0xe9, 3);
DEFINE_DEVPROPKEY!(DEVPKEY_Storage_System_Critical,
    0x4d1ebee8, 0x803, 0x4774, 0x98, 0x42, 0xb7, 0x7d, 0xb5, 0x2, 0x65, 0xe9, 4);
pub const IOCTL_STORAGE_BASE: ULONG = FILE_DEVICE_MASS_STORAGE;
pub const IOCTL_STORAGE_CHECK_VERIFY: ULONG =
    CTL_CODE!(IOCTL_STORAGE_BASE, 0x0200, METHOD_BUFFERED, FILE_READ_ACCESS);
pub const IOCTL_STORAGE_CHECK_VERIFY2: ULONG =
    CTL_CODE!(IOCTL_STORAGE_BASE, 0x0200, METHOD_BUFFERED, FILE_ANY_ACCESS);
pub const IOCTL_STORAGE_MEDIA_REMOVAL: ULONG =
    CTL_CODE!(IOCTL_STORAGE_BASE, 0x0201, METHOD_BUFFERED, FILE_READ_ACCESS);
pub const IOCTL_STORAGE_EJECT_MEDIA: ULONG =
    CTL_CODE!(IOCTL_STORAGE_BASE, 0x0202, METHOD_BUFFERED, FILE_READ_ACCESS);
pub const IOCTL_STORAGE_LOAD_MEDIA: ULONG =
    CTL_CODE!(IOCTL_STORAGE_BASE, 0x0203, METHOD_BUFFERED, FILE_READ_ACCESS);
pub const IOCTL_STORAGE_LOAD_MEDIA2: ULONG =
    CTL_CODE!(IOCTL_STORAGE_BASE, 0x0203, METHOD_BUFFERED, FILE_ANY_ACCESS);
pub const IOCTL_STORAGE_RESERVE: ULONG =
    CTL_CODE!(IOCTL_STORAGE_BASE, 0x0204, METHOD_BUFFERED, FILE_READ_ACCESS);
pub const IOCTL_STORAGE_RELEASE: ULONG =
    CTL_CODE!(IOCTL_STORAGE_BASE, 0x0205, METHOD_BUFFERED, FILE_READ_ACCESS);
pub const IOCTL_STORAGE_FIND_NEW_DEVICES: ULONG =
    CTL_CODE!(IOCTL_STORAGE_BASE, 0x0206, METHOD_BUFFERED, FILE_READ_ACCESS);
pub const IOCTL_STORAGE_EJECTION_CONTROL: ULONG =
    CTL_CODE!(IOCTL_STORAGE_BASE, 0x0250, METHOD_BUFFERED, FILE_ANY_ACCESS);
pub const IOCTL_STORAGE_MCN_CONTROL: ULONG =
    CTL_CODE!(IOCTL_STORAGE_BASE, 0x0251, METHOD_BUFFERED, FILE_ANY_ACCESS);
pub const IOCTL_STORAGE_GET_MEDIA_TYPES: ULONG =
    CTL_CODE!(IOCTL_STORAGE_BASE, 0x0300, METHOD_BUFFERED, FILE_ANY_ACCESS);
pub const IOCTL_STORAGE_GET_MEDIA_TYPES_EX: ULONG =
    CTL_CODE!(IOCTL_STORAGE_BASE, 0x0301, METHOD_BUFFERED, FILE_ANY_ACCESS);
pub const IOCTL_STORAGE_GET_MEDIA_SERIAL_NUMBER: ULONG =
    CTL_CODE!(IOCTL_STORAGE_BASE, 0x0304, METHOD_BUFFERED, FILE_ANY_ACCESS);
pub const IOCTL_STORAGE_GET_HOTPLUG_INFO: ULONG =
    CTL_CODE!(IOCTL_STORAGE_BASE, 0x0305, METHOD_BUFFERED, FILE_ANY_ACCESS);
pub const IOCTL_STORAGE_SET_HOTPLUG_INFO: ULONG =
    CTL_CODE!(IOCTL_STORAGE_BASE, 0x0306, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS);
pub const IOCTL_STORAGE_RESET_BUS: ULONG =
    CTL_CODE!(IOCTL_STORAGE_BASE, 0x0400, METHOD_BUFFERED, FILE_READ_ACCESS);
pub const IOCTL_STORAGE_RESET_DEVICE: ULONG =
    CTL_CODE!(IOCTL_STORAGE_BASE, 0x0401, METHOD_BUFFERED, FILE_READ_ACCESS);
pub const IOCTL_STORAGE_BREAK_RESERVATION: ULONG =
    CTL_CODE!(IOCTL_STORAGE_BASE, 0x0405, METHOD_BUFFERED, FILE_READ_ACCESS);
pub const IOCTL_STORAGE_PERSISTENT_RESERVE_IN: ULONG =
    CTL_CODE!(IOCTL_STORAGE_BASE, 0x0406, METHOD_BUFFERED, FILE_READ_ACCESS);
pub const IOCTL_STORAGE_PERSISTENT_RESERVE_OUT: ULONG =
    CTL_CODE!(IOCTL_STORAGE_BASE, 0x0407, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS);
pub const IOCTL_STORAGE_GET_DEVICE_NUMBER: ULONG =
    CTL_CODE!(IOCTL_STORAGE_BASE, 0x0420, METHOD_BUFFERED, FILE_ANY_ACCESS);
pub const IOCTL_STORAGE_GET_DEVICE_NUMBER_EX: ULONG =
    CTL_CODE!(IOCTL_STORAGE_BASE, 0x0421, METHOD_BUFFERED, FILE_ANY_ACCESS);
pub const IOCTL_STORAGE_PREDICT_FAILURE: ULONG =
    CTL_CODE!(IOCTL_STORAGE_BASE, 0x0440, METHOD_BUFFERED, FILE_ANY_ACCESS);
pub const IOCTL_STORAGE_FAILURE_PREDICTION_CONFIG: ULONG =
    CTL_CODE!(IOCTL_STORAGE_BASE, 0x0441, METHOD_BUFFERED, FILE_ANY_ACCESS);
pub const IOCTL_STORAGE_GET_COUNTERS: ULONG =
    CTL_CODE!(IOCTL_STORAGE_BASE, 0x442, METHOD_BUFFERED, FILE_ANY_ACCESS);
pub const IOCTL_STORAGE_READ_CAPACITY: ULONG =
    CTL_CODE!(IOCTL_STORAGE_BASE, 0x0450, METHOD_BUFFERED, FILE_READ_ACCESS);
pub const IOCTL_STORAGE_GET_DEVICE_TELEMETRY: ULONG =
    CTL_CODE!(IOCTL_STORAGE_BASE, 0x0470, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS);
pub const IOCTL_STORAGE_DEVICE_TELEMETRY_NOTIFY: ULONG =
    CTL_CODE!(IOCTL_STORAGE_BASE, 0x0471, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS);
pub const IOCTL_STORAGE_DEVICE_TELEMETRY_QUERY_CAPS: ULONG =
    CTL_CODE!(IOCTL_STORAGE_BASE, 0x0472, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS);
pub const IOCTL_STORAGE_GET_DEVICE_TELEMETRY_RAW: ULONG =
    CTL_CODE!(IOCTL_STORAGE_BASE, 0x0473, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS);
pub const IOCTL_STORAGE_SET_TEMPERATURE_THRESHOLD: ULONG =
    CTL_CODE!(IOCTL_STORAGE_BASE, 0x0480, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS);
pub const IOCTL_STORAGE_PROTOCOL_COMMAND: ULONG =
    CTL_CODE!(IOCTL_STORAGE_BASE, 0x04F0, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS);
pub const IOCTL_STORAGE_QUERY_PROPERTY: ULONG =
    CTL_CODE!(IOCTL_STORAGE_BASE, 0x0500, METHOD_BUFFERED, FILE_ANY_ACCESS);
pub const IOCTL_STORAGE_MANAGE_DATA_SET_ATTRIBUTES: ULONG =
    CTL_CODE!(IOCTL_STORAGE_BASE, 0x0501, METHOD_BUFFERED, FILE_WRITE_ACCESS);
pub const IOCTL_STORAGE_GET_LB_PROVISIONING_MAP_RESOURCES: ULONG =
    CTL_CODE!(IOCTL_STORAGE_BASE, 0x0502, METHOD_BUFFERED, FILE_READ_ACCESS);
pub const IOCTL_STORAGE_REINITIALIZE_MEDIA: ULONG =
    CTL_CODE!(IOCTL_STORAGE_BASE, 0x0590, METHOD_BUFFERED, FILE_WRITE_ACCESS);
pub const IOCTL_STORAGE_GET_BC_PROPERTIES: ULONG =
    CTL_CODE!(IOCTL_STORAGE_BASE, 0x0600, METHOD_BUFFERED, FILE_READ_ACCESS);
pub const IOCTL_STORAGE_ALLOCATE_BC_STREAM: ULONG =
    CTL_CODE!(IOCTL_STORAGE_BASE, 0x0601, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS);
pub const IOCTL_STORAGE_FREE_BC_STREAM: ULONG =
    CTL_CODE!(IOCTL_STORAGE_BASE, 0x0602, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS);
pub const IOCTL_STORAGE_CHECK_PRIORITY_HINT_SUPPORT: ULONG =
    CTL_CODE!(IOCTL_STORAGE_BASE, 0x0620, METHOD_BUFFERED, FILE_ANY_ACCESS);
pub const IOCTL_STORAGE_START_DATA_INTEGRITY_CHECK: ULONG =
    CTL_CODE!(IOCTL_STORAGE_BASE, 0x0621, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS);
pub const IOCTL_STORAGE_STOP_DATA_INTEGRITY_CHECK: ULONG =
    CTL_CODE!(IOCTL_STORAGE_BASE, 0x0622, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS);
pub const OBSOLETE_IOCTL_STORAGE_RESET_BUS: ULONG =
    CTL_CODE!(IOCTL_STORAGE_BASE, 0x0400, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS);
pub const OBSOLETE_IOCTL_STORAGE_RESET_DEVICE: ULONG =
    CTL_CODE!(IOCTL_STORAGE_BASE, 0x0401, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS);
pub const IOCTL_STORAGE_FIRMWARE_GET_INFO: ULONG =
    CTL_CODE!(IOCTL_STORAGE_BASE, 0x0700, METHOD_BUFFERED, FILE_ANY_ACCESS);
pub const IOCTL_STORAGE_FIRMWARE_DOWNLOAD: ULONG =
    CTL_CODE!(IOCTL_STORAGE_BASE, 0x0701, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS);
pub const IOCTL_STORAGE_FIRMWARE_ACTIVATE: ULONG =
    CTL_CODE!(IOCTL_STORAGE_BASE, 0x0702, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS);
pub const IOCTL_STORAGE_ENABLE_IDLE_POWER: ULONG =
    CTL_CODE!(IOCTL_STORAGE_BASE, 0x0720, METHOD_BUFFERED, FILE_ANY_ACCESS);
pub const IOCTL_STORAGE_GET_IDLE_POWERUP_REASON: ULONG =
    CTL_CODE!(IOCTL_STORAGE_BASE, 0x0721, METHOD_BUFFERED, FILE_ANY_ACCESS);
pub const IOCTL_STORAGE_POWER_ACTIVE: ULONG =
    CTL_CODE!(IOCTL_STORAGE_BASE, 0x0722, METHOD_BUFFERED, FILE_ANY_ACCESS);
pub const IOCTL_STORAGE_POWER_IDLE: ULONG =
    CTL_CODE!(IOCTL_STORAGE_BASE, 0x0723, METHOD_BUFFERED, FILE_ANY_ACCESS);
pub const IOCTL_STORAGE_EVENT_NOTIFICATION: ULONG =
    CTL_CODE!(IOCTL_STORAGE_BASE, 0x0724, METHOD_BUFFERED, FILE_ANY_ACCESS);
pub const IOCTL_STORAGE_DEVICE_POWER_CAP: ULONG =
    CTL_CODE!(IOCTL_STORAGE_BASE, 0x0725, METHOD_BUFFERED, FILE_ANY_ACCESS);
pub const IOCTL_STORAGE_RPMB_COMMAND: ULONG =
    CTL_CODE!(IOCTL_STORAGE_BASE, 0x0726, METHOD_BUFFERED, FILE_ANY_ACCESS);
pub const IOCTL_STORAGE_ATTRIBUTE_MANAGEMENT: ULONG =
    CTL_CODE!(IOCTL_STORAGE_BASE, 0x0727, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS);
STRUCT!{struct STORAGE_HOTPLUG_INFO {
    Size: ULONG,
    MediaRemovable: BOOLEAN,
    MediaHotplug: BOOLEAN,
    DeviceHotplug: BOOLEAN,
    WriteCacheEnableOverride: BOOLEAN,
}}
pub type PSTORAGE_HOTPLUG_INFO = *mut STORAGE_HOTPLUG_INFO;
STRUCT!{struct STORAGE_DEVICE_NUMBER {
    DeviceType: DEVICE_TYPE,
    DeviceNumber: ULONG,
    PartitionNumber: ULONG,
}}
pub type PSTORAGE_DEVICE_NUMBER = *mut STORAGE_DEVICE_NUMBER;
STRUCT!{struct STORAGE_DEVICE_NUMBERS {
    NumberOfDevices: ULONG,
    Devices: [STORAGE_DEVICE_NUMBER; ANYSIZE_ARRAY],
}}
pub type PSTORAGE_DEVICE_NUMBERS = *mut STORAGE_DEVICE_NUMBERS;
pub const STORAGE_DEVICE_FLAGS_RANDOM_DEVICEGUID_REASON_CONFLICT: ULONG = 0x1;
pub const STORAGE_DEVICE_FLAGS_RANDOM_DEVICEGUID_REASON_NOHWID: ULONG = 0x2;
pub const STORAGE_DEVICE_FLAGS_PAGE_83_DEVICEGUID: ULONG = 0x4;
STRUCT!{struct STORAGE_DEVICE_NUMBER_EX {
    Version: ULONG,
    Size: ULONG,
    Flags: ULONG,
    DeviceType: DEVICE_TYPE,
    DeviceNumber: ULONG,
    DeviceGuid: GUID,
    PartitionNumber: ULONG,
}}
pub type PSTORAGE_DEVICE_NUMBER_EX = *mut STORAGE_DEVICE_NUMBER_EX;
STRUCT!{struct STORAGE_BUS_RESET_REQUEST {
    PathId: UCHAR,
}}
pub type PSTORAGE_BUS_RESET_REQUEST = *mut STORAGE_BUS_RESET_REQUEST;
STRUCT!{struct STORAGE_BREAK_RESERVATION_REQUEST {
    Length: ULONG,
    _unused: UCHAR,
    PathId: UCHAR,
    TargetId: UCHAR,
    Lun: UCHAR,
}}
pub type PSTORAGE_BREAK_RESERVATION_REQUEST = *mut STORAGE_BREAK_RESERVATION_REQUEST;
STRUCT!{struct PREVENT_MEDIA_REMOVAL {
    PreventMediaRemoval: BOOLEAN,
}}
pub type PPREVENT_MEDIA_REMOVAL = *mut PREVENT_MEDIA_REMOVAL;
STRUCT!{struct CLASS_MEDIA_CHANGE_CONTEXT {
    MediaChangeCount: ULONG,
    NewState: ULONG,
}}
pub type PCLASS_MEDIA_CHANGE_CONTEXT = *mut CLASS_MEDIA_CHANGE_CONTEXT;
STRUCT!{struct TAPE_STATISTICS {
    Version: ULONG,
    Flags: ULONG,
    RecoveredWrites: LARGE_INTEGER,
    UnrecoveredWrites: LARGE_INTEGER,
    RecoveredReads: LARGE_INTEGER,
    UnrecoveredReads: LARGE_INTEGER,
    CompressionRatioReads: UCHAR,
    CompressionRatioWrites: UCHAR,
}}
pub type PTAPE_STATISTICS = *mut TAPE_STATISTICS;
pub const RECOVERED_WRITES_VALID: ULONG = 0x00000001;
pub const UNRECOVERED_WRITES_VALID: ULONG = 0x00000002;
pub const RECOVERED_READS_VALID: ULONG = 0x00000004;
pub const UNRECOVERED_READS_VALID: ULONG = 0x00000008;
pub const WRITE_COMPRESSION_INFO_VALID: ULONG = 0x00000010;
pub const READ_COMPRESSION_INFO_VALID: ULONG = 0x00000020;
STRUCT!{struct TAPE_GET_STATISTICS {
    Operation: ULONG,
}}
pub type PTAPE_GET_STATISTICS = *mut TAPE_GET_STATISTICS;
pub const TAPE_RETURN_STATISTICS: ULONG = 0;
pub const TAPE_RETURN_ENV_INFO: ULONG = 1;
pub const TAPE_RESET_STATISTICS: ULONG = 2;
ENUM!{enum STORAGE_MEDIA_TYPE {
    DDS_4mm = 0x20,
    MiniQic,
    Travan,
    QIC,
    MP_8mm,
    AME_8mm,
    AIT1_8mm,
    DLT,
    NCTP,
    IBM_3480,
    IBM_3490E,
    IBM_Magstar_3590,
    IBM_Magstar_MP,
    STK_DATA_D3,
    SONY_DTF,
    DV_6mm,
    DMI,
    SONY_D2,
    CLEANER_CARTRIDGE,
    CD_ROM,
    CD_R,
    CD_RW,
    DVD_ROM,
    DVD_R,
    DVD_RW,
    MO_3_RW,
    MO_5_WO,
    MO_5_RW,
    MO_5_LIMDOW,
    PC_5_WO,
    PC_5_RW,
    PD_5_RW,
    ABL_5_WO,
    PINNACLE_APEX_5_RW,
    SONY_12_WO,
    PHILIPS_12_WO,
    HITACHI_12_WO,
    CYGNET_12_WO,
    KODAK_14_WO,
    MO_NFR_525,
    NIKON_12_RW,
    IOMEGA_ZIP,
    IOMEGA_JAZ,
    SYQUEST_EZ135,
    SYQUEST_EZFLYER,
    SYQUEST_SYJET,
    AVATAR_F2,
    MP2_8mm,
    DST_S,
    DST_M,
    DST_L,
    VXATape_1,
    VXATape_2,
    STK_9840,
    LTO_Ultrium,
    LTO_Accelis,
    DVD_RAM,
    AIT_8mm,
    ADR_1,
    ADR_2,
    STK_9940,
    SAIT,
    VXATape,
}}
pub type PSTORAGE_MEDIA_TYPE = *mut STORAGE_MEDIA_TYPE;
pub const MEDIA_ERASEABLE: ULONG = 0x00000001;
pub const MEDIA_WRITE_ONCE: ULONG = 0x00000002;
pub const MEDIA_READ_ONLY: ULONG = 0x00000004;
pub const MEDIA_READ_WRITE: ULONG = 0x00000008;
pub const MEDIA_WRITE_PROTECTED: ULONG = 0x00000100;
pub const MEDIA_CURRENTLY_MOUNTED: ULONG = 0x80000000;
ENUM!{enum STORAGE_BUS_TYPE {
    BusTypeUnknown = 0x00,
    BusTypeScsi,
    BusTypeAtapi,
    BusTypeAta,
    BusType1394,
    BusTypeSsa,
    BusTypeFibre,
    BusTypeUsb,
    BusTypeRAID,
    BusTypeiScsi,
    BusTypeSas,
    BusTypeSata,
    BusTypeSd,
    BusTypeMmc,
    BusTypeVirtual,
    BusTypeFileBackedVirtual,
    BusTypeSpaces,
    BusTypeNvme,
    BusTypeSCM,
    BusTypeUfs,
    BusTypeMax,
    BusTypeMaxReserved = 0x7F,
}}
pub type PSTORAGE_BUS_TYPE = *mut STORAGE_BUS_TYPE;
#[inline]
pub fn SupportsDeviceSharing(BusType: STORAGE_BUS_TYPE) -> bool {
    (BusType == BusTypeScsi)  ||
    (BusType == BusTypeFibre) ||
    (BusType == BusTypeiScsi) ||
    (BusType == BusTypeSas)   ||
    (BusType == BusTypeSpaces)
}
STRUCT!{struct DEVICE_MEDIA_INFO {
    DeviceSpecific: DEVICE_MEDIA_INFO_DeviceSpecific,
}}
pub type PDEVICE_MEDIA_INFO = *mut DEVICE_MEDIA_INFO;
UNION2!{union DEVICE_MEDIA_INFO_DeviceSpecific {
    [u32; 8],
    DiskInfo DiskInfo_mut: DEVICE_MEDIA_INFO_DeviceSpecific_DiskInfo,
    RemovableDiskInfo RemovableDiskInfo_mut: DEVICE_MEDIA_INFO_DeviceSpecific_RemovableDiskInfo,
    TapeInfo TapeInfo_mut: DEVICE_MEDIA_INFO_DeviceSpecific_TapeInfo,
}}
STRUCT!{struct DEVICE_MEDIA_INFO_DeviceSpecific_DiskInfo {
    Cylinders: LARGE_INTEGER,
    MediaType: STORAGE_MEDIA_TYPE,
    TracksPerCylinder: ULONG,
    SectorsPerTrack: ULONG,
    BytesPerSector: ULONG,
    NumberMediaSides: ULONG,
    MediaCharacteristics: ULONG,
}}
STRUCT!{struct DEVICE_MEDIA_INFO_DeviceSpecific_RemovableDiskInfo {
    Cylinders: LARGE_INTEGER,
    MediaType: STORAGE_MEDIA_TYPE,
    TracksPerCylinder: ULONG,
    SectorsPerTrack: ULONG,
    BytesPerSector: ULONG,
    NumberMediaSides: ULONG,
    MediaCharacteristics: ULONG,
}}
STRUCT!{struct DEVICE_MEDIA_INFO_DeviceSpecific_TapeInfo {
    MediaType: STORAGE_MEDIA_TYPE,
    MediaCharacteristics: ULONG,
    CurrentBlockSize: ULONG,
    BusType: STORAGE_BUS_TYPE,
    BusSpecificData: DEVICE_MEDIA_INFO_DeviceSpecific_TapeInfo_BusSpecificData,
}}
UNION2!{union DEVICE_MEDIA_INFO_DeviceSpecific_TapeInfo_BusSpecificData {
    [u8; 2],
    ScsiInformation ScsiInformation_mut:
        DEVICE_MEDIA_INFO_DeviceSpecific_TapeInfo_BusSpecificData_ScsiInformation,
}}
STRUCT!{struct DEVICE_MEDIA_INFO_DeviceSpecific_TapeInfo_BusSpecificData_ScsiInformation {
    MediumType: UCHAR,
    DensityCode: UCHAR,
}}
STRUCT!{struct GET_MEDIA_TYPES {
    DeviceType: ULONG,
    MediaInfoCount: ULONG,
    MediaInfo: [DEVICE_MEDIA_INFO; 1],
}}
pub type PGET_MEDIA_TYPES = *mut GET_MEDIA_TYPES;
STRUCT!{struct STORAGE_PREDICT_FAILURE {
    PredictFailure: ULONG,
    VendorSpecific: [UCHAR; 512],
}}
pub type PSTORAGE_PREDICT_FAILURE = *mut STORAGE_PREDICT_FAILURE;
STRUCT!{struct STORAGE_FAILURE_PREDICTION_CONFIG {
    Version: ULONG,
    Size: ULONG,
    Set: BOOLEAN,
    Enabled: BOOLEAN,
    Reserved: USHORT,
}}
pub type PSTORAGE_FAILURE_PREDICTION_CONFIG = *mut STORAGE_FAILURE_PREDICTION_CONFIG;
pub const STORAGE_FAILURE_PREDICTION_CONFIG_V1: ULONG = 1;
ENUM!{enum STORAGE_QUERY_TYPE {
    PropertyStandardQuery = 0,
    PropertyExistsQuery,
    PropertyMaskQuery,
    PropertyQueryMaxDefined,
}}
pub type PSTORAGE_QUERY_TYPE = *mut STORAGE_QUERY_TYPE;
ENUM!{enum STORAGE_PROPERTY_ID {
    StorageDeviceProperty = 0,
    StorageAdapterProperty,
    StorageDeviceIdProperty,
    StorageDeviceUniqueIdProperty,
    StorageDeviceWriteCacheProperty,
    StorageMiniportProperty,
    StorageAccessAlignmentProperty,
    StorageDeviceSeekPenaltyProperty,
    StorageDeviceTrimProperty,
    StorageDeviceWriteAggregationProperty,
    StorageDeviceDeviceTelemetryProperty,
    StorageDeviceLBProvisioningProperty,
    StorageDevicePowerProperty,
    StorageDeviceCopyOffloadProperty,
    StorageDeviceResiliencyProperty,
    StorageDeviceMediumProductType,
    StorageAdapterRpmbProperty,
    StorageAdapterCryptoProperty,
    StorageDeviceTieringProperty,
    StorageDeviceFaultDomainProperty,
    StorageDeviceClusportProperty,
    StorageDeviceIoCapabilityProperty = 48,
    StorageAdapterProtocolSpecificProperty,
    StorageDeviceProtocolSpecificProperty,
    StorageAdapterTemperatureProperty,
    StorageDeviceTemperatureProperty,
    StorageAdapterPhysicalTopologyProperty,
    StorageDevicePhysicalTopologyProperty,
    StorageDeviceAttributesProperty,
    StorageDeviceManagementStatus,
    StorageAdapterSerialNumberProperty,
    StorageDeviceLocationProperty,
    StorageDeviceNumaProperty,
}}
pub type PSTORAGE_PROPERTY_ID = *mut STORAGE_PROPERTY_ID;
STRUCT!{struct STORAGE_PROPERTY_QUERY {
    PropertyId: STORAGE_PROPERTY_ID,
    QueryType: STORAGE_QUERY_TYPE,
    AdditionalParameters: [UCHAR; 1],
}}
pub type PSTORAGE_PROPERTY_QUERY = *mut STORAGE_PROPERTY_QUERY;
STRUCT!{struct STORAGE_DESCRIPTOR_HEADER {
    Version: ULONG,
    Size: ULONG,
}}
pub type PSTORAGE_DESCRIPTOR_HEADER = *mut STORAGE_DESCRIPTOR_HEADER;
STRUCT!{struct STORAGE_DEVICE_DESCRIPTOR {
    Version: ULONG,
    Size: ULONG,
    DeviceType: UCHAR,
    DeviceTypeModifier: UCHAR,
    RemovableMedia: BOOLEAN,
    CommandQueueing: BOOLEAN,
    VendorIdOffset: ULONG,
    ProductIdOffset: ULONG,
    ProductRevisionOffset: ULONG,
    SerialNumberOffset: ULONG,
    BusType: STORAGE_BUS_TYPE,
    RawPropertiesLength: ULONG,
    RawDeviceProperties: [UCHAR; 1],
}}
pub type PSTORAGE_DEVICE_DESCRIPTOR = *mut STORAGE_DEVICE_DESCRIPTOR;
STRUCT!{struct STORAGE_ADAPTER_DESCRIPTOR {
    Version: ULONG,
    Size: ULONG,
    MaximumTransferLength: ULONG,
    MaximumPhysicalPages: ULONG,
    AlignmentMask: ULONG,
    AdapterUsesPio: BOOLEAN,
    AdapterScansDown: BOOLEAN,
    CommandQueueing: BOOLEAN,
    AcceleratedTransfer: BOOLEAN,
    BusType: UCHAR,
    BusMajorVersion: USHORT,
    BusMinorVersion: USHORT,
    SrbType: UCHAR,
    AddressType: UCHAR,
}}
pub type PSTORAGE_ADAPTER_DESCRIPTOR = *mut STORAGE_ADAPTER_DESCRIPTOR;
// TODO: Revisit this when offset_of! arrives.
pub const NO_SRBTYPE_ADAPTER_DESCRIPTOR_SIZE: ULONG = 30;
pub const SRB_TYPE_SCSI_REQUEST_BLOCK: UCHAR = 0;
pub const SRB_TYPE_STORAGE_REQUEST_BLOCK: UCHAR = 1;
pub const STORAGE_ADDRESS_TYPE_BTL8: UCHAR = 0;
STRUCT!{struct STORAGE_ACCESS_ALIGNMENT_DESCRIPTOR {
    Version: ULONG,
    Size: ULONG,
    BytesPerCacheLine: ULONG,
    BytesOffsetForCacheAlignment: ULONG,
    BytesPerLogicalSector: ULONG,
    BytesPerPhysicalSector: ULONG,
    BytesOffsetForSectorAlignment: ULONG,
}}
pub type PSTORAGE_ACCESS_ALIGNMENT_DESCRIPTOR = *mut STORAGE_ACCESS_ALIGNMENT_DESCRIPTOR;
STRUCT!{struct STORAGE_MEDIUM_PRODUCT_TYPE_DESCRIPTOR {
    Version: ULONG,
    Size: ULONG,
    MediumProductType: ULONG,
}}
pub type PSTORAGE_MEDIUM_PRODUCT_TYPE_DESCRIPTOR = *mut STORAGE_MEDIUM_PRODUCT_TYPE_DESCRIPTOR;
ENUM!{enum STORAGE_PORT_CODE_SET {
    StoragePortCodeSetReserved = 0,
    StoragePortCodeSetStorport = 1,
    StoragePortCodeSetSCSIport = 2,
    StoragePortCodeSetSpaceport = 3,
    StoragePortCodeSetATAport = 4,
    StoragePortCodeSetUSBport = 5,
    StoragePortCodeSetSBP2port = 6,
    StoragePortCodeSetSDport = 7,
}}
pub type PSTORAGE_PORT_CODE_SET = *mut STORAGE_PORT_CODE_SET;
// TODO: Revisit this when offset_of! arrives.
pub const STORAGE_MINIPORT_DESCRIPTOR_V1_SIZE: ULONG = 16;
STRUCT!{struct STORAGE_MINIPORT_DESCRIPTOR {
    Version: ULONG,
    Size: ULONG,
    Portdriver: STORAGE_PORT_CODE_SET,
    LUNResetSupported: BOOLEAN,
    TargetResetSupported: BOOLEAN,
    IoTimeoutValue: USHORT,
    ExtraIoInfoSupported: BOOLEAN,
    Reserved0: [UCHAR; 3],
    Reserved1: ULONG,
}}
pub type PSTORAGE_MINIPORT_DESCRIPTOR = *mut STORAGE_MINIPORT_DESCRIPTOR;
ENUM!{enum STORAGE_IDENTIFIER_CODE_SET {
    StorageIdCodeSetReserved = 0,
    StorageIdCodeSetBinary = 1,
    StorageIdCodeSetAscii = 2,
    StorageIdCodeSetUtf8 = 3,
}}
pub type PSTORAGE_IDENTIFIER_CODE_SET = *mut STORAGE_IDENTIFIER_CODE_SET;
ENUM!{enum STORAGE_IDENTIFIER_TYPE {
    StorageIdTypeVendorSpecific = 0,
    StorageIdTypeVendorId = 1,
    StorageIdTypeEUI64 = 2,
    StorageIdTypeFCPHName = 3,
    StorageIdTypePortRelative = 4,
    StorageIdTypeTargetPortGroup = 5,
    StorageIdTypeLogicalUnitGroup = 6,
    StorageIdTypeMD5LogicalUnitIdentifier = 7,
    StorageIdTypeScsiNameString = 8,
}}
pub type PSTORAGE_IDENTIFIER_TYPE = *mut STORAGE_IDENTIFIER_TYPE;
pub const StorageIdTypeNAA: STORAGE_IDENTIFIER_TYPE = StorageIdTypeFCPHName;
ENUM!{enum STORAGE_ID_NAA_FORMAT {
    StorageIdNAAFormatIEEEExtended = 2,
    StorageIdNAAFormatIEEERegistered = 3,
    StorageIdNAAFormatIEEEERegisteredExtended = 5,
}}
pub type PSTORAGE_ID_NAA_FORMAT = *mut STORAGE_ID_NAA_FORMAT;
ENUM!{enum STORAGE_ASSOCIATION_TYPE {
    StorageIdAssocDevice = 0,
    StorageIdAssocPort = 1,
    StorageIdAssocTarget = 2,
}}
pub type PSTORAGE_ASSOCIATION_TYPE = *mut STORAGE_ASSOCIATION_TYPE;
STRUCT!{struct STORAGE_IDENTIFIER {
    CodeSet: STORAGE_IDENTIFIER_CODE_SET,
    Type: STORAGE_IDENTIFIER_TYPE,
    IdentifierSize: USHORT,
    NextOffset: USHORT,
    Association: STORAGE_ASSOCIATION_TYPE,
    Identifier: [UCHAR; 1],
}}
pub type PSTORAGE_IDENTIFIER = *mut STORAGE_IDENTIFIER;
STRUCT!{struct STORAGE_DEVICE_ID_DESCRIPTOR {
    Version: ULONG,
    Size: ULONG,
    NumberOfIdentifiers: ULONG,
    Identifiers: [UCHAR; 1],
}}
pub type PSTORAGE_DEVICE_ID_DESCRIPTOR = *mut STORAGE_DEVICE_ID_DESCRIPTOR;
STRUCT!{struct DEVICE_SEEK_PENALTY_DESCRIPTOR {
    Version: ULONG,
    Size: ULONG,
    IncursSeekPenalty: BOOLEAN,
}}
pub type PDEVICE_SEEK_PENALTY_DESCRIPTOR = *mut DEVICE_SEEK_PENALTY_DESCRIPTOR;
STRUCT!{struct DEVICE_WRITE_AGGREGATION_DESCRIPTOR {
    Version: ULONG,
    Size: ULONG,
    BenefitsFromWriteAggregation: BOOLEAN,
}}
pub type PDEVICE_WRITE_AGGREGATION_DESCRIPTOR = *mut DEVICE_WRITE_AGGREGATION_DESCRIPTOR;
STRUCT!{struct DEVICE_TRIM_DESCRIPTOR {
    Version: ULONG,
    Size: ULONG,
    TrimEnabled: BOOLEAN,
}}
pub type PDEVICE_TRIM_DESCRIPTOR = *mut DEVICE_TRIM_DESCRIPTOR;
STRUCT!{struct DEVICE_LB_PROVISIONING_DESCRIPTOR {
    Version: ULONG,
    Size: ULONG,
    Bitfield: UCHAR,
    Reserved1: [UCHAR; 7],
    OptimalUnmapGranularity: ULONGLONG,
    UnmapGranularityAlignment: ULONGLONG,
    MaxUnmapLbaCount: ULONG,
    MaxUnmapBlockDescriptorCount: ULONG,
}}
pub type PDEVICE_LB_PROVISIONING_DESCRIPTOR = *mut DEVICE_LB_PROVISIONING_DESCRIPTOR;
BITFIELD!(DEVICE_LB_PROVISIONING_DESCRIPTOR Bitfield: UCHAR [
    ThinProvisioningEnabled set_ThinProvisioningEnabled[0..1],
    ThinProvisioningReadZeros set_ThinProvisioningReadZeros[1..2],
    AnchorSupported set_AnchorSupported[2..5],
    UnmapGranularityAlignmentValid set_UnmapGranularityAlignmentValid[5..6],
    Reserved0 set_Reserved0[6..8],
]);
// TODO: Revisit this when offset_of! arrives.
pub const DEVICE_LB_PROVISIONING_DESCRIPTOR_V1_SIZE: ULONG = 32;
STRUCT!{struct STORAGE_LB_PROVISIONING_MAP_RESOURCES {
    Size: ULONG,
    Version: ULONG,
    Bitfield0: UCHAR,
    Reserved1: [UCHAR; 3],
    Bitfield1: UCHAR,
    Reserved3: [UCHAR; 3],
    AvailableMappingResources: ULONGLONG,
    UsedMappingResources: ULONGLONG,
}}
pub type PSTORAGE_LB_PROVISIONING_MAP_RESOURCES = *mut STORAGE_LB_PROVISIONING_MAP_RESOURCES;
BITFIELD!(STORAGE_LB_PROVISIONING_MAP_RESOURCES Bitfield0: UCHAR [
    AvailableMappingResourcesValid set_AvailableMappingResourcesValid[0..1],
    UsedMappingResourcesValid set_UsedMappingResourcesValid[1..2],
    Reserved0 set_Reserved0[2..8],
]);
BITFIELD!(STORAGE_LB_PROVISIONING_MAP_RESOURCES Bitfield1: UCHAR [
    AvailableMappingResourcesScope set_AvailableMappingResourcesScope[0..2],
    UsedMappingResourcesScope set_UsedMappingResourcesScope[2..4],
    Reserved2 set_Reserved2[4..8],
]);
STRUCT!{struct DEVICE_POWER_DESCRIPTOR {
    Version: ULONG,
    Size: ULONG,
    DeviceAttentionSupported: BOOLEAN,
    AsynchronousNotificationSupported: BOOLEAN,
    IdlePowerManagementEnabled: BOOLEAN,
    D3ColdEnabled: BOOLEAN,
    D3ColdSupported: BOOLEAN,
    NoVerifyDuringIdlePower: BOOLEAN,
    Reserved: [UCHAR; 2],
    IdleTimeoutInMS: ULONG,
}}
pub type PDEVICE_POWER_DESCRIPTOR = *mut DEVICE_POWER_DESCRIPTOR;
STRUCT!{struct DEVICE_COPY_OFFLOAD_DESCRIPTOR {
    Version: ULONG,
    Size: ULONG,
    MaximumTokenLifetime: ULONG,
    DefaultTokenLifetime: ULONG,
    MaximumTransferSize: ULONGLONG,
    OptimalTransferCount: ULONGLONG,
    MaximumDataDescriptors: ULONG,
    MaximumTransferLengthPerDescriptor: ULONG,
    OptimalTransferLengthPerDescriptor: ULONG,
    OptimalTransferLengthGranularity: USHORT,
    Reserved: [UCHAR; 2],
}}
pub type PDEVICE_COPY_OFFLOAD_DESCRIPTOR = *mut DEVICE_COPY_OFFLOAD_DESCRIPTOR;
STRUCT!{struct STORAGE_DEVICE_RESILIENCY_DESCRIPTOR {
    Version: ULONG,
    Size: ULONG,
    NameOffset: ULONG,
    NumberOfLogicalCopies: ULONG,
    NumberOfPhysicalCopies: ULONG,
    PhysicalDiskRedundancy: ULONG,
    NumberOfColumns: ULONG,
    Interleave: ULONG,
}}
pub type PSTORAGE_DEVICE_RESILIENCY_DESCRIPTOR = *mut STORAGE_DEVICE_RESILIENCY_DESCRIPTOR;
ENUM!{enum STORAGE_RPMB_FRAME_TYPE {
    StorageRpmbFrameTypeUnknown = 0,
    StorageRpmbFrameTypeStandard,
    StorageRpmbFrameTypeMax,
}}
pub type PSTORAGE_RPMB_FRAME_TYPE = *mut STORAGE_RPMB_FRAME_TYPE;
pub const STORAGE_RPMB_DESCRIPTOR_VERSION_1: ULONG = 1;
pub const STORAGE_RPMB_MINIMUM_RELIABLE_WRITE_SIZE: ULONG = 512;
STRUCT!{struct STORAGE_RPMB_DESCRIPTOR {
    Version: ULONG,
    Sie: ULONG,
    SizeInBytes: ULONG,
    MaxReliableWriteSizeInBytes: ULONG,
    FrameFormat: STORAGE_RPMB_FRAME_TYPE,
}}
pub type PSTORAGE_RPMB_DESCRIPTOR = *mut STORAGE_RPMB_DESCRIPTOR;
ENUM!{enum STORAGE_CRYPTO_ALGORITHM_ID {
    StorageCryptoAlgorithmUnknown = 0,
    StorageCryptoAlgorithmXTSAES = 1,
    StorageCryptoAlgorithmBitlockerAESCBC,
    StorageCryptoAlgorithmAESECB,
    StorageCryptoAlgorithmESSIVAESCBC,
    StorageCryptoAlgorithmMax,
}}
pub type PSTORAGE_CRYPTO_ALGORITHM_ID = *mut STORAGE_CRYPTO_ALGORITHM_ID;
ENUM!{enum STORAGE_CRYPTO_KEY_SIZE {
    StorageCryptoKeySizeUnknown = 0,
    StorageCryptoKeySize128Bits = 1,
    StorageCryptoKeySize192Bits,
    StorageCryptoKeySize256Bits,
    StorageCryptoKeySize512Bits,
}}
pub type PSTORAGE_CRYPTO_KEY_SIZE = *mut STORAGE_CRYPTO_KEY_SIZE;
pub const STORAGE_CRYPTO_CAPABILITY_VERSION_1: ULONG = 1;
STRUCT!{struct STORAGE_CRYPTO_CAPABILITY {
    Version: ULONG,
    Size: ULONG,
    CryptoCapabilityIndex: ULONG,
    AlgorithmId: STORAGE_CRYPTO_ALGORITHM_ID,
    KeySize: STORAGE_CRYPTO_KEY_SIZE,
    DataUnitSizeBitmask: ULONG,
}}
pub type PSTORAGE_CRYPTO_CAPABILITY = *mut STORAGE_CRYPTO_CAPABILITY;
pub const STORAGE_CRYPTO_DESCRIPTOR_VERSION_1: ULONG = 1;
STRUCT!{struct STORAGE_CRYPTO_DESCRIPTOR {
    Version: ULONG,
    Size: ULONG,
    NumKeysSupported: ULONG,
    NumCryptoCapabilities: ULONG,
    CryptoCapabilities: [STORAGE_CRYPTO_CAPABILITY; ANYSIZE_ARRAY],
}}
pub type PSTORAGE_CRYPTO_DESCRIPTOR = *mut STORAGE_CRYPTO_DESCRIPTOR;
pub const STORAGE_TIER_NAME_LENGTH: usize = 256;
pub const STORAGE_TIER_DESCRIPTION_LENGTH: usize = 512;
pub const STORAGE_TIER_FLAG_NO_SEEK_PENALTY: ULONGLONG = 0x00020000;
pub const STORAGE_TIER_FLAG_WRITE_BACK_CACHE: ULONGLONG = 0x00200000;
pub const STORAGE_TIER_FLAG_READ_CACHE: ULONGLONG = 0x00400000;
pub const STORAGE_TIER_FLAG_PARITY: ULONGLONG = 0x00800000;
ENUM!{enum STORAGE_TIER_MEDIA_TYPE {
    StorageTierMediaTypeUnspecified = 0,
    StorageTierMediaTypeDisk = 1,
    StorageTierMediaTypeSsd = 2,
    StorageTierMediaTypeScm = 4,
    StorageTierMediaTypeMax,
}}
pub type PSTORAGE_TIER_MEDIA_TYPE = *mut STORAGE_TIER_MEDIA_TYPE;
ENUM!{enum STORAGE_TIER_CLASS {
    StorageTierClassUnspecified = 0,
    StorageTierClassCapacity,
    StorageTierClassPerformance,
    StorageTierClassMax,
}}
pub type PSTORAGE_TIER_CLASS = *mut STORAGE_TIER_CLASS;
STRUCT!{struct STORAGE_TIER {
    Id: GUID,
    Name: [WCHAR; STORAGE_TIER_NAME_LENGTH],
    Description: [WCHAR; STORAGE_TIER_NAME_LENGTH],
    Flags: ULONGLONG,
    ProvisionedCapacity: ULONGLONG,
    MediaType: STORAGE_TIER_MEDIA_TYPE,
    Class: STORAGE_TIER_CLASS,
}}
pub type PSTORAGE_TIER = *mut STORAGE_TIER;
STRUCT!{struct STORAGE_DEVICE_TIERING_DESCRIPTOR {
    Version: ULONG,
    Size: ULONG,
    Flags: ULONG,
    TotalNumberOfTiers: ULONG,
    NumberOfTiersReturned: ULONG,
    Tiers: [STORAGE_TIER; ANYSIZE_ARRAY],
}}
pub type PSTORAGE_DEVICE_TIERING_DESCRIPTOR = *mut STORAGE_DEVICE_TIERING_DESCRIPTOR;
STRUCT!{struct STORAGE_DEVICE_FAULT_DOMAIN_DESCRIPTOR {
    Version: ULONG,
    Size: ULONG,
    NumberOfFaultDomains: ULONG,
    FaultDomainIds: [GUID; ANYSIZE_ARRAY],
}}
pub type PSTORAGE_DEVICE_FAULT_DOMAIN_DESCRIPTOR = *mut STORAGE_DEVICE_FAULT_DOMAIN_DESCRIPTOR;
ENUM!{enum STORAGE_PROTOCOL_TYPE {
    ProtocolTypeUnknown = 0x00,
    ProtocolTypeScsi,
    ProtocolTypeAta,
    ProtocolTypeNvme,
    ProtocolTypeSd,
    ProtocolTypeProprietary = 0x7E,
    ProtocolTypeMaxReserved = 0x7F,
}}
pub type PSTORAGE_PROTOCOL_TYPE = *mut STORAGE_PROTOCOL_TYPE;
ENUM!{enum STORAGE_PROTOCOL_NVME_DATA_TYPE {
    NVMeDataTypeUnknown = 0,
    NVMeDataTypeIdentify,
    NVMeDataTypeLogPage,
    NVMeDataTypeFeature,
}}
pub type PSTORAGE_PROTOCOL_NVME_DATA_TYPE = *mut STORAGE_PROTOCOL_NVME_DATA_TYPE;
ENUM!{enum STORAGE_PROTOCOL_ATA_DATA_TYPE {
    AtaDataTypeUnknown = 0,
    AtaDataTypeIdentify,
    AtaDataTypeLogPage,
}}
pub type PSTORAGE_PROTOCOL_ATA_DATA_TYPE = *mut STORAGE_PROTOCOL_ATA_DATA_TYPE;
STRUCT!{struct STORAGE_PROTOCOL_SPECIFIC_DATA {
    ProtocolType: STORAGE_PROTOCOL_TYPE,
    DataType: ULONG,
    ProtocolDataRequestValue: ULONG,
    ProtocolDataRequestSubValue: ULONG,
    ProtocolDataOffset: ULONG,
    ProtocolDataLength: ULONG,
    FixedProtocolReturnData: ULONG,
    Reserved: [ULONG; 3],
}}
pub type PSTORAGE_PROTOCOL_SPECIFIC_DATA = *mut STORAGE_PROTOCOL_SPECIFIC_DATA;
STRUCT!{struct STORAGE_PROTOCOL_DATA_DESCRIPTOR {
    Version: ULONG,
    Size: ULONG,
    ProtocolSpecificData: STORAGE_PROTOCOL_SPECIFIC_DATA,
}}
pub type PSTORAGE_PROTOCOL_DATA_DESCRIPTOR = *mut STORAGE_PROTOCOL_DATA_DESCRIPTOR;
pub const STORAGE_TEMPERATURE_VALUE_NOT_REPORTED: SHORT = 0x8000u16 as SHORT;
STRUCT!{struct STORAGE_TEMPERATURE_INFO {
    Index: USHORT,
    Temperature: SHORT,
    OverThreshold: SHORT,
    UnderThreshold: SHORT,
    OverThresholdChangable: BOOLEAN,
    UnderThresholdChangable: BOOLEAN,
    EventGenerated: BOOLEAN,
    Reserved0: UCHAR,
    Reserved1: ULONG,
}}
pub type PSTORAGE_TEMPERATURE_INFO = *mut STORAGE_TEMPERATURE_INFO;
STRUCT!{struct STORAGE_TEMPERATURE_DATA_DESCRIPTOR {
    Version: ULONG,
    Size: ULONG,
    CriticalTemperature: SHORT,
    WarningTemperature: SHORT,
    InfoCount: USHORT,
    Reserved0: [UCHAR; 2],
    Reserved1: [ULONG; 2],
    TemperatureInfo: [STORAGE_TEMPERATURE_INFO; ANYSIZE_ARRAY],
}}
pub type PSTORAGE_TEMPERATURE_DATA_DESCRIPTOR = *mut STORAGE_TEMPERATURE_DATA_DESCRIPTOR;
pub const STORAGE_TEMPERATURE_THRESHOLD_FLAG_ADAPTER_REQUEST: USHORT = 0x0001;
STRUCT!{struct STORAGE_TEMPERATURE_THRESHOLD {
    Version: ULONG,
    Size: ULONG,
    Flags: USHORT,
    Index: USHORT,
    Threshold: SHORT,
    OverThreshold: BOOLEAN,
    Reserved: UCHAR,
}}
pub type PSTORAGE_TEMPERATURE_THESHOLD = *mut STORAGE_TEMPERATURE_THRESHOLD;
pub const STORAGE_COMPONENT_ROLE_CACHE: ULONG = 0x00000001;
pub const STORAGE_COMPONENT_ROLE_TIERING: ULONG = 0x00000002;
pub const STORAGE_COMPONENT_ROLE_DATA: ULONG = 0x00000004;
ENUM!{enum STORAGE_DEVICE_FORM_FACTOR {
    FormFactorUnknown = 0,
    FormFactor3_5,
    FormFactor2_5,
    FormFactor1_8,
    FormFactor1_8Less,
    FormFactorEmbedded,
    FormFactorMemoryCard,
    FormFactormSata,
    FormFactorM_2,
    FormFactorPCIeBoard,
    FormFactorDimm,
}}
pub type PSTORAGE_DEVICE_FORM_FACTOR = *mut STORAGE_DEVICE_FORM_FACTOR;
ENUM!{enum STORAGE_COMPONENT_HEALTH_STATUS {
    HealthStatusUnknown = 0,
    HealthStatusNormal,
    HealthStatusThrottled,
    HealthStatusWarning,
    HealthStatusDisabled,
    HealthStatusFailed,
}}
pub type PSTORAGE_COMPONENT_HEALTH_STATUS = *mut STORAGE_COMPONENT_HEALTH_STATUS;
UNION2!{union STORAGE_SPEC_VERSION {
    [u32; 1],
    s s_mut: STORAGE_SPEC_VERSION_s,
    AsUlong AsUlong_mut: ULONG,
}}
pub type PSTORAGE_SPEC_VERSION = *mut STORAGE_SPEC_VERSION;
STRUCT!{struct STORAGE_SPEC_VERSION_s {
    MinorVersion: STORAGE_SPEC_VERSION_s_MinorVersion,
    MajorVersion: USHORT,
}}
UNION2!{union STORAGE_SPEC_VERSION_s_MinorVersion {
    [u16; 1],
    s s_mut: STORAGE_SPEC_VERSION_s_MinorVersion_s,
    AsUshort AsUshort_mut: USHORT,
}}
STRUCT!{struct STORAGE_SPEC_VERSION_s_MinorVersion_s {
    SubMinor: UCHAR,
    Minor: UCHAR,
}}
STRUCT!{struct STORAGE_PHYSICAL_DEVICE_DATA {
    DeviceId: ULONG,
    Role: ULONG,
    HealthStatus: STORAGE_COMPONENT_HEALTH_STATUS,
    CommandProtocol: STORAGE_PROTOCOL_TYPE,
    SpecVersion: STORAGE_SPEC_VERSION,
    FormFactor: STORAGE_DEVICE_FORM_FACTOR,
    Vendor: [UCHAR; 8],
    Model: [UCHAR; 40],
    FirmwareRevision: [UCHAR; 16],
    Capacity: ULONGLONG,
    PhysicalLocation: [UCHAR; 32],
    Reserved: [ULONG; 2],
}}
pub type PSTORAGE_PHYSICAL_DEVICE_DATA = *mut STORAGE_PHYSICAL_DEVICE_DATA;
STRUCT!{struct STORAGE_PHYSICAL_ADAPTER_DATA {
    AdapterId: ULONG,
    HealthStatus: STORAGE_COMPONENT_HEALTH_STATUS,
    CommandProtocol: STORAGE_PROTOCOL_TYPE,
    SpecVersion: STORAGE_SPEC_VERSION,
    Vendor: [UCHAR; 8],
    Model: [UCHAR; 40],
    FirmwareRevision: [UCHAR; 16],
    PhysicalLocation: [UCHAR; 32],
    ExpanderConnected: BOOLEAN,
    Reserved0: [UCHAR; 3],
    Reserved1: [ULONG; 3],
}}
pub type PSTORAGE_PHYSICAL_ADAPTER_DATA = *mut STORAGE_PHYSICAL_ADAPTER_DATA;
STRUCT!{struct STORAGE_PHYSICAL_NODE_DATA {
    NodeId: ULONG,
    AdapterCount: ULONG,
    AdapterDataLength: ULONG,
    AdapterDataOffset: ULONG,
    DeviceCount: ULONG,
    DeviceDataLength: ULONG,
    DeviceDataOffset: ULONG,
    Reserved: [ULONG; 3],
}}
pub type PSTORAGE_PHYSICAL_NODE_DATA = *mut STORAGE_PHYSICAL_NODE_DATA;
STRUCT!{struct STORAGE_PHYSICAL_TOPOLOGY_DESCRIPTOR {
    Version: ULONG,
    Size: ULONG,
    NodeCount: ULONG,
    Reserved: ULONG,
    Node: [STORAGE_PHYSICAL_NODE_DATA; ANYSIZE_ARRAY],
}}
pub type PSTORAGE_PHYSICAL_TOPOLOGY_DESCRIPTOR = *mut STORAGE_PHYSICAL_TOPOLOGY_DESCRIPTOR;
STRUCT!{struct STORAGE_DEVICE_IO_CAPABILITY_DESCRIPTOR {
    Version: ULONG,
    Size: ULONG,
    LunMaxIoCount: ULONG,
    AdapterMaxIoCount: ULONG,
}}
pub type PSTORAGE_DEVICE_IO_CAPABILITY_DESCRIPTOR = *mut STORAGE_DEVICE_IO_CAPABILITY_DESCRIPTOR;
STRUCT!{struct STORAGE_DEVICE_ATTRIBUTES_DESCRIPTOR {
    Version: ULONG,
    Size: ULONG,
    Attributes: ULONG64,
}}
pub type PSTORAGE_DEVICE_ATTRIBUTES_DESCRIPTOR = *mut STORAGE_DEVICE_ATTRIBUTES_DESCRIPTOR;
pub const STORAGE_ATTRIBUTE_BYTE_ADDRESSABLE_IO: ULONG64 = 0x01;
pub const STORAGE_ATTRIBUTE_BLOCK_IO: ULONG64 = 0x02;
pub const STORAGE_ATTRIBUTE_DYNAMIC_PERSISTENCE: ULONG64 = 0x04;
pub const STORAGE_ATTRIBUTE_VOLATILE: ULONG64 = 0x08;
pub const STORAGE_ATTRIBUTE_ASYNC_EVENT_NOTIFICATION: ULONG64 = 0x10;
pub const STORAGE_ATTRIBUTE_PERF_SIZE_INDEPENDENT: ULONG64 = 0x20;
ENUM!{enum STORAGE_DISK_HEALTH_STATUS {
    DiskHealthUnknown = 0,
    DiskHealthUnhealthy,
    DiskHealthWarning,
    DiskHealthHealthy,
    DiskHealthMax,
}}
pub type PSTORAGE_DISK_HEALTH_STATUS = *mut STORAGE_DISK_HEALTH_STATUS;
ENUM!{enum STORAGE_DISK_OPERATIONAL_STATUS {
    DiskOpStatusNone = 0,
    DiskOpStatusUnknown,
    DiskOpStatusOk,
    DiskOpStatusPredictingFailure,
    DiskOpStatusInService,
    DiskOpStatusHardwareError,
    DiskOpStatusNotUsable,
    DiskOpStatusTransientError,
    DiskOpStatusMissing,
}}
pub type PSTORAGE_DISK_OPERATIONAL_STATUS = *mut STORAGE_DISK_OPERATIONAL_STATUS;
ENUM!{enum STORAGE_OPERATIONAL_STATUS_REASON {
    DiskOpReasonUnknown = 0,
    DiskOpReasonScsiSenseCode,
    DiskOpReasonMedia,
    DiskOpReasonIo,
    DiskOpReasonThresholdExceeded,
    DiskOpReasonLostData,
    DiskOpReasonEnergySource,
    DiskOpReasonConfiguration,
    DiskOpReasonDeviceController,
    DiskOpReasonMediaController,
    DiskOpReasonComponent,
    DiskOpReasonNVDIMM_N,
    DiskOpReasonBackgroundOperation,
    DiskOpReasonInvalidFirmware,
    DiskOpReasonHealthCheck,
    DiskOpReasonLostDataPersistence,
    DiskOpReasonDisabledByPlatform,
    DiskOpReasonMax,
}}
pub type PSTORAGE_OPERATIONAL_STATUS_REASON = *mut STORAGE_OPERATIONAL_STATUS_REASON;
STRUCT!{struct STORAGE_OPERATIONAL_REASON {
    Version: ULONG,
    Size: ULONG,
    Reason: STORAGE_OPERATIONAL_STATUS_REASON,
    RawBytes: STORAGE_OPERATIONAL_REASON_RawBytes,
}}
pub type PSTORAGE_OPERATIONAL_REASON = *mut STORAGE_OPERATIONAL_REASON;
UNION2!{union STORAGE_OPERATIONAL_REASON_RawBytes {
    [u32; 1],
    ScsiSenseKey ScsiSenseKey_mut: STORAGE_OPERATIONAL_REASON_RawBytes_ScsiSenseKey,
    NVDIMM_N NVDIMM_N_mut: STORAGE_OPERATIONAL_REASON_RawBytes_NVDIMM_N,
    AsUlong AsUlong_mut: ULONG,
}}
STRUCT!{struct STORAGE_OPERATIONAL_REASON_RawBytes_ScsiSenseKey {
    SenseKey: UCHAR,
    ASC: UCHAR,
    ASCQ: UCHAR,
    Reserved: UCHAR,
}}
STRUCT!{struct STORAGE_OPERATIONAL_REASON_RawBytes_NVDIMM_N {
    CriticalHealth: UCHAR,
    ModuleHealth: [UCHAR; 2],
    ErrorThresholdStatus: UCHAR,
}}
pub const STORAGE_DEVICE_MAX_OPERATIONAL_STATUS: usize = 16;
STRUCT!{struct STORAGE_DEVICE_MANAGEMENT_STATUS {
    Version: ULONG,
    Size: ULONG,
    Health: STORAGE_DISK_HEALTH_STATUS,
    NumberOfOperationalStatus: ULONG,
    NumberOfAdditionalReasons: ULONG,
    OperationalStatus: [STORAGE_DISK_OPERATIONAL_STATUS; STORAGE_DEVICE_MAX_OPERATIONAL_STATUS],
    AdditionalReasons: [STORAGE_OPERATIONAL_REASON; ANYSIZE_ARRAY],
}}
pub type PSTORAGE_DEVICE_MANAGEMENT_STATUS = *mut STORAGE_DEVICE_MANAGEMENT_STATUS;
pub const STORAGE_ADAPTER_SERIAL_NUMBER_V1_MAX_LENGTH: usize = 128;
STRUCT!{struct STORAGE_ADAPTER_SERIAL_NUMBER {
    Version: ULONG,
    Size: ULONG,
    SerialNumber: [WCHAR; STORAGE_ADAPTER_SERIAL_NUMBER_V1_MAX_LENGTH],
}}
pub type PSTORAGE_ADAPTER_SERIAL_NUMBER = *mut STORAGE_ADAPTER_SERIAL_NUMBER;
// TODO: Revisit these when const fn size_of arrives.
pub const STORAGE_ADAPTER_SERIAL_NUMBER_V1_VERSION: ULONG = 264;
pub const STORAGE_ADAPTER_SERIAL_NUMBER_V1_SIZE: ULONG = 264;
STRUCT!{struct DEVICE_LOCATION {
    Socket: ULONG,
    Slot: ULONG,
    Adapter: ULONG,
    Port: ULONG,
    u: DEVICE_LOCATION_u,
}}
pub type PDEVICE_LOCATION = *mut DEVICE_LOCATION;
UNION2!{union DEVICE_LOCATION_u {
    [u32; 2],
    s s_mut: DEVICE_LOCATION_u_s,
    s2 s2_mut: DEVICE_LOCATION_u_s2,
}}
STRUCT!{struct DEVICE_LOCATION_u_s {
    Channel: ULONG,
    Device: ULONG,
}}
STRUCT!{struct DEVICE_LOCATION_u_s2 {
    Target: ULONG,
    Lun: ULONG,
}}
STRUCT!{struct STORAGE_DEVICE_LOCATION_DESCRIPTOR {
    Version: ULONG,
    Size: ULONG,
    Location: DEVICE_LOCATION,
    StringOffset: ULONG,
}}
pub type PSTORAGE_DEVICE_LOCATION_DESCRIPTOR = *mut STORAGE_DEVICE_LOCATION_DESCRIPTOR;
STRUCT!{struct STORAGE_DEVICE_NUMA_PROPERTY {
    Version: ULONG,
    Size: ULONG,
    NumaNode: ULONG,
}}
pub type PSTORAGE_DEVICE_NUMA_PROPERTY = *mut STORAGE_DEVICE_NUMA_PROPERTY;
// TODO: Needs shared::ntdef::MAXULONG
//pub const STORAGE_DEVICE_NUMA_NODE_UNKNOWN: ULONG = MAXULONG;
pub const DeviceDsmActionFlag_NonDestructive: ULONG = 0x80000000;
#[inline]
pub fn IsDsmActionNonDestructive(_Action: DEVICE_DATA_MANAGEMENT_SET_ACTION) -> BOOLEAN {
    ((_Action & DeviceDsmActionFlag_NonDestructive) != 0) as BOOLEAN
}
pub type DEVICE_DATA_MANAGEMENT_SET_ACTION = ULONG;
pub const DeviceDsmAction_None: DEVICE_DATA_MANAGEMENT_SET_ACTION = 0;
pub const DeviceDsmAction_Trim: DEVICE_DATA_MANAGEMENT_SET_ACTION = 1;
pub const DeviceDsmAction_Notification: DEVICE_DATA_MANAGEMENT_SET_ACTION = 2  | DeviceDsmActionFlag_NonDestructive;
pub const DeviceDsmAction_OffloadRead: DEVICE_DATA_MANAGEMENT_SET_ACTION = 3 | DeviceDsmActionFlag_NonDestructive;
pub const DeviceDsmAction_OffloadWrite: DEVICE_DATA_MANAGEMENT_SET_ACTION = 4;
pub const DeviceDsmAction_Allocation: DEVICE_DATA_MANAGEMENT_SET_ACTION = 5 | DeviceDsmActionFlag_NonDestructive;
pub const DeviceDsmAction_Repair: DEVICE_DATA_MANAGEMENT_SET_ACTION = 6 | DeviceDsmActionFlag_NonDestructive;
pub const DeviceDsmAction_Scrub: DEVICE_DATA_MANAGEMENT_SET_ACTION = 7 | DeviceDsmActionFlag_NonDestructive;
pub const DeviceDsmAction_DrtQuery: DEVICE_DATA_MANAGEMENT_SET_ACTION = 8 | DeviceDsmActionFlag_NonDestructive;
pub const DeviceDsmAction_DrtClear: DEVICE_DATA_MANAGEMENT_SET_ACTION = 9 | DeviceDsmActionFlag_NonDestructive;
pub const DeviceDsmAction_DrtDisable: DEVICE_DATA_MANAGEMENT_SET_ACTION = 10 | DeviceDsmActionFlag_NonDestructive;
pub const DeviceDsmAction_TieringQuery: DEVICE_DATA_MANAGEMENT_SET_ACTION = 11 | DeviceDsmActionFlag_NonDestructive;
pub const DeviceDsmAction_Map: DEVICE_DATA_MANAGEMENT_SET_ACTION = 12 | DeviceDsmActionFlag_NonDestructive;
pub const DeviceDsmAction_RegenerateParity: DEVICE_DATA_MANAGEMENT_SET_ACTION = 13 | DeviceDsmActionFlag_NonDestructive;
pub const DeviceDsmAction_NvCache_Change_Priority: DEVICE_DATA_MANAGEMENT_SET_ACTION = 14 | DeviceDsmActionFlag_NonDestructive;
pub const DeviceDsmAction_NvCache_Evict: DEVICE_DATA_MANAGEMENT_SET_ACTION = 15 | DeviceDsmActionFlag_NonDestructive;
pub const DeviceDsmAction_TopologyIdQuery: DEVICE_DATA_MANAGEMENT_SET_ACTION = 16 | DeviceDsmActionFlag_NonDestructive;
pub const DeviceDsmAction_GetPhysicalAddresses: DEVICE_DATA_MANAGEMENT_SET_ACTION = 17 | DeviceDsmActionFlag_NonDestructive;
pub const DeviceDsmAction_ScopeRegen: DEVICE_DATA_MANAGEMENT_SET_ACTION = 18 | DeviceDsmActionFlag_NonDestructive;
pub const DEVICE_DSM_FLAG_ENTIRE_DATA_SET_RANGE: ULONG = 0x00000001;
pub const DEVICE_DSM_FLAG_TRIM_NOT_FS_ALLOCATED: ULONG = 0x80000000;
pub const DEVICE_DSM_FLAG_ALLOCATION_CONSOLIDATEABLE_ONLY: ULONG = 0x40000000;
pub const DEVICE_DSM_FLAG_SCRUB_SKIP_IN_SYNC: ULONG = 0x10000000;
pub const DEVICE_DSM_FLAG_SCRUB_OUTPUT_PARITY_EXTENT: ULONG = 0x20000000;
pub const DEVICE_DSM_FLAG_REPAIR_OUTPUT_PARITY_EXTENT: ULONG = 0x20000000;
pub const DEVICE_DSM_FLAG_REPAIR_INPUT_TOPOLOGY_ID_PRESENT: ULONG = 0x40000000;
pub const DEVICE_DSM_FLAG_PHYSICAL_ADDRESSES_OMIT_TOTAL_RANGES: ULONG = 0x10000000;
STRUCT!{struct DEVICE_DATA_SET_RANGE {
    StartingOffset: LONGLONG,
    LengthInBytes: ULONGLONG,
}}
pub type PDEVICE_DATA_SET_RANGE = *mut DEVICE_DATA_SET_RANGE;
STRUCT!{struct DEVICE_MANAGE_DATA_SET_ATTRIBUTES {
    Size: ULONG,
    Action: DEVICE_DATA_MANAGEMENT_SET_ACTION,
    Flags: ULONG,
    ParameterBlockOffset: ULONG,
    ParameterBlockLength: ULONG,
    DataSetRangesOffset: ULONG,
    DataSetRangesLength: ULONG,
}}
pub type PDEVICE_MANAGE_DATA_SET_ATTRIBUTES = *mut DEVICE_MANAGE_DATA_SET_ATTRIBUTES;
STRUCT!{struct DEVICE_DSM_NOTIFICATION_PARAMETERS {
    Size: ULONG,
    Flags: ULONG,
    NumFileTypeIDs: ULONG,
    FileTypeID: [GUID; 1],
}}
pub type PDEVICE_DSM_NOTIFICATION_PARAMETERS = *mut DEVICE_DSM_NOTIFICATION_PARAMETERS;
pub const DEVICE_DSM_NOTIFY_FLAG_BEGIN: ULONG = 0x00000001;
pub const DEVICE_DSM_NOTIFY_FLAG_END: ULONG = 0x00000002;
STRUCT!{struct DEVICE_DSM_NVCACHE_CHANGE_PRIORITY_PARAMETERS {
    Size: ULONG,
    TargetPriority: UCHAR,
    Reserved: [UCHAR; 3],
}}
pub type PDEVICE_DSM_NVCACHE_CHANGE_PRIORITY_PARAMETERS =
    *mut DEVICE_DSM_NVCACHE_CHANGE_PRIORITY_PARAMETERS;
pub const STORAGE_OFFLOAD_MAX_TOKEN_LENGTH: usize = 512;
pub const STORAGE_OFFLOAD_TOKEN_ID_LENGTH: usize = 0x1F8;
pub const STORAGE_OFFLOAD_TOKEN_TYPE_ZERO_DATA: ULONG = 0xFFFF0001;
STRUCT!{struct STORAGE_OFFLOAD_TOKEN {
    TokenType: [UCHAR; 4],
    Reserved: [UCHAR; 2],
    TokenIdLength: [UCHAR; 2],
    u: STORAGE_OFFLOAD_TOKEN_u,
}}
pub type PSTORAGE_OFFLOAD_TOKEN = *mut STORAGE_OFFLOAD_TOKEN;
UNION2!{union STORAGE_OFFLOAD_TOKEN_u {
    [u8; STORAGE_OFFLOAD_TOKEN_ID_LENGTH],
    StorageOffloadZeroDataToken StorageOffloadZeroDataToken_mut:
        STORAGE_OFFLOAD_TOKEN_u_StorageOffloadZeroDataToken,
    Token Token_mut: [UCHAR; STORAGE_OFFLOAD_TOKEN_ID_LENGTH],
}}
STRUCT!{struct STORAGE_OFFLOAD_TOKEN_u_StorageOffloadZeroDataToken {
    Reserved2: [UCHAR; STORAGE_OFFLOAD_TOKEN_ID_LENGTH],
}}
#[inline]
pub unsafe fn MAKE_ZERO_TOKEN(T: PSTORAGE_OFFLOAD_TOKEN) {
    (*T).TokenType[0] = 0xFF;
    (*T).TokenType[1] = 0xFF;
    (*T).TokenType[2] = 0x00;
    (*T).TokenType[3] = 0x01;
    (*T).TokenIdLength[0] = 0x01;
    (*T).TokenIdLength[1] = 0xF8;
}
#[inline]
pub unsafe fn IS_ZERO_TOKEN(T: PSTORAGE_OFFLOAD_TOKEN) -> bool {
    (*T).TokenType[0] == 0xFF &&
        (*T).TokenType[1] == 0xFF &&
        (*T).TokenType[2] == 0x00 &&
        (*T).TokenType[3] == 0x01 &&
        (*T).TokenIdLength[0] == 0x01 &&
        (*T).TokenIdLength[1] == 0xF8
}
STRUCT!{struct DEVICE_DSM_OFFLOAD_READ_PARAMETERS {
    Flags: ULONG,
    TimeToLive: ULONG,
    Reserved: [ULONG; 2],
}}
pub type PDEVICE_DSM_OFFLOAD_READ_PARAMETERS = *mut DEVICE_DSM_OFFLOAD_READ_PARAMETERS;
STRUCT!{struct DEVICE_DSM_OFFLOAD_WRITE_PARAMETERS {
    Flags: ULONG,
    Reserved: ULONG,
    TokenOffset: ULONGLONG,
    Token: STORAGE_OFFLOAD_TOKEN,
}}
pub type PDEVICE_DSM_OFFLOAD_WRITE_PARAMETERS = *mut DEVICE_DSM_OFFLOAD_WRITE_PARAMETERS;
// TODO
/* #[inline]
pub unsafe fn GET_REPAIR_TOPOLOGY_ID(R: PDEVICE_DATA_SET_REPAIR_PARAMETERS) -> PUCHAR {
    unimplemented!()
} */
STRUCT!{struct DEVICE_DATA_SET_REPAIR_PARAMETERS {
    NumberOfRepairCopies: ULONG,
    SourceCopy: ULONG,
    RepairCopies: [ULONG; ANYSIZE_ARRAY],
}}
pub type PDEVICE_DATA_SET_REPAIR_PARAMETERS = *mut DEVICE_DATA_SET_REPAIR_PARAMETERS;
STRUCT!{struct DEVICE_DATA_SET_LBP_STATE_PARAMETERS {
    Version: ULONG,
    Size: ULONG,
    Flags: ULONG,
    OutputVersion: ULONG,
}}
pub type PDEVICE_DATA_SET_LBP_STATE_PARAMETERS = *mut DEVICE_DATA_SET_LBP_STATE_PARAMETERS;
pub const DEVICE_DATA_SET_LBP_STATE_PARAMETERS_VERSION_V1: ULONG = 1;
STRUCT!{struct DEVICE_MANAGE_DATA_SET_ATTRIBUTES_OUTPUT {
    Size: ULONG,
    Action: DEVICE_DATA_MANAGEMENT_SET_ACTION,
    Flags: ULONG,
    OperationStatus: ULONG,
    ExtendedError: ULONG,
    TargetDetailedError: ULONG,
    ReservedStatus: ULONG,
    OutputBlockOffset: ULONG,
    OutputBlockLength: ULONG,
}}
pub type PDEVICE_MANAGE_DATA_SET_ATTRIBUTES_OUTPUT =
    *mut DEVICE_MANAGE_DATA_SET_ATTRIBUTES_OUTPUT;
STRUCT!{struct DEVICE_DATA_SET_LB_PROVISIONING_STATE {
    Size: ULONG,
    Version: ULONG,
    SlabSizeInBytes: ULONGLONG,
    SlabOffsetDeltaInBytes: ULONG,
    SlabAllocationBitMapBitCount: ULONG,
    SlabAllocationBitMapLength: ULONG,
    SlabAllocationBitMap: [ULONG; ANYSIZE_ARRAY],
}}
pub type PDEVICE_DATA_SET_LB_PROVISIONING_STATE = *mut DEVICE_DATA_SET_LB_PROVISIONING_STATE;
// TODO: Revisit this when const fn size_of arrives.
pub const DEVICE_DATA_SET_LB_PROVISIONING_STATE_VERSION_V1: ULONG = 32;
STRUCT!{struct DEVICE_DATA_SET_LB_PROVISIONING_STATE_V2 {
    Size: ULONG,
    Version: ULONG,
    SlabSizeInBytes: ULONGLONG,
    SlabOffsetDeltaInBytes: ULONGLONG,
    SlabAllocationBitMapBitCount: ULONG,
    SlabAllocationBitMapLength: ULONG,
    SlabAllocationBitMap: [ULONG; ANYSIZE_ARRAY],
}}
pub type PDEVICE_DATA_SET_LB_PROVISIONING_STATE_V2 =
    *mut DEVICE_DATA_SET_LB_PROVISIONING_STATE_V2;
// TODO: Revisit this when const fn size_of arrives.
pub const DEVICE_DATA_SET_LB_PROVISIONING_STATE_VERSION_V2: ULONG = 40;
STRUCT!{struct STORAGE_OFFLOAD_READ_OUTPUT {
    OffloadReadFlags: ULONG,
    Reserved: ULONG,
    LengthProtected: ULONGLONG,
    TokenLength: ULONG,
}}
pub type PSTORAGE_OFFLOAD_READ_OUTPUT = *mut STORAGE_OFFLOAD_READ_OUTPUT;
pub const STORAGE_OFFLOAD_READ_RANGE_TRUNCATED: ULONG = 0x0001;
STRUCT!{struct STORAGE_OFFLOAD_WRITE_OUTPUT {
    OffloadWriteFlags: ULONG,
    Reserved: ULONG,
    LengthCopied: ULONGLONG,
}}
pub type PSTORAGE_OFFLOAD_WRITE_OUTPUT = *mut STORAGE_OFFLOAD_WRITE_OUTPUT;
pub const STORAGE_OFFLOAD_WRITE_RANGE_TRUNCATED: ULONG = 0x0001;
pub const STORAGE_OFFLOAD_TOKEN_INVALID: ULONG = 0x0002;
STRUCT!{struct DEVICE_DATA_SET_SCRUB_OUTPUT {
    BytesProcessed: ULONGLONG,
    BytesRepaired: ULONGLONG,
    BytesFailed: ULONGLONG,
}}
pub type PDEVICE_DATA_SET_SCRUB_OUTPUT = *mut DEVICE_DATA_SET_SCRUB_OUTPUT;
STRUCT!{struct DEVICE_DATA_SET_SCRUB_EX_OUTPUT {
    BytesProcessed: ULONGLONG,
    BytesRepaired: ULONGLONG,
    BytesFailed: ULONGLONG,
    ParityExtent: DEVICE_DATA_SET_RANGE,
}}
pub type PDEVICE_DATA_SET_SCRUB_EX_OUTPUT = *mut DEVICE_DATA_SET_SCRUB_EX_OUTPUT;
STRUCT!{struct DEVICE_DATA_SET_REPAIR_OUTPUT {
    ParityExtent: DEVICE_DATA_SET_RANGE,
}}
pub type PDEVICE_DATA_SET_REPAIR_OUTPUT = *mut DEVICE_DATA_SET_REPAIR_OUTPUT;
STRUCT!{struct DEVICE_DSM_TIERING_QUERY_INPUT {
    Version: ULONG,
    Size: ULONG,
    Flags: ULONG,
    NumberOfTierIds: ULONG,
    TierIds: [GUID; ANYSIZE_ARRAY],
}}
pub type PDEVICE_DSM_TIERING_QUERY_INPUT = *mut DEVICE_DSM_TIERING_QUERY_INPUT;
STRUCT!{struct STORAGE_TIER_REGION {
    TierId: GUID,
    Offset: ULONGLONG,
    Length: ULONGLONG,
}}
pub type PSTORAGE_TIER_REGION = *mut STORAGE_TIER_REGION;
STRUCT!{struct DEVICE_DSM_TIERING_QUERY_OUTPUT {
    Version: ULONG,
    Size: ULONG,
    Flags: ULONG,
    Reserved: ULONG,
    Alignment: ULONGLONG,
    TotalNumberOfRegions: ULONG,
    NumberOfRegionsReturned: ULONG,
    Regions: [STORAGE_TIER_REGION; ANYSIZE_ARRAY],
}}
pub type PDEVICE_DSM_TIERING_QUERY_OUTPUT = *mut DEVICE_DSM_TIERING_QUERY_OUTPUT;
STRUCT!{struct DEVICE_DATA_SET_TOPOLOGY_ID_QUERY_OUTPUT {
    TopologyRangeBytes: ULONGLONG,
    TopologyId: [UCHAR; 16],
}}
pub type PDEVICE_DATA_SET_TOPOLOGY_ID_QUERY_OUTPUT =
    *mut DEVICE_DATA_SET_TOPOLOGY_ID_QUERY_OUTPUT;
STRUCT!{struct DEVICE_STORAGE_ADDRESS_RANGE {
    StartAddress: LONGLONG,
    LengthInBytes: ULONGLONG,
}}
pub type PDEVICE_STORAGE_ADDRESS_RANGE = *mut DEVICE_STORAGE_ADDRESS_RANGE;
pub const DEVICE_DSM_PHYSICAL_ADDRESSES_OUTPUT_VERSION_V1: ULONG = 1;
STRUCT!{struct DEVICE_DSM_PHYSICAL_ADDRESSES_OUTPUT {
    Version: ULONG,
    Flags: ULONG,
    TotalNumberOfRanges: ULONG,
    NumberOfRangesReturned: ULONG,
    Ranges: [DEVICE_STORAGE_ADDRESS_RANGE; ANYSIZE_ARRAY],
}}
pub type PDEVICE_DSM_PHYSICAL_ADDRESSES_OUTPUT = *mut DEVICE_DSM_PHYSICAL_ADDRESSES_OUTPUT;
STRUCT!{struct STORAGE_GET_BC_PROPERTIES_OUTPUT {
    MaximumRequestsPerPeriod: ULONG,
    MinimumPeriod: ULONG,
    MaximumRequestSize: ULONGLONG,
    EstimatedTimePerRequest: ULONG,
    NumOutStandingRequests: ULONG,
    RequestSize: ULONGLONG,
}}
pub type PSTORAGE_GET_BC_PROPERTIES_OUTPUT = *mut STORAGE_GET_BC_PROPERTIES_OUTPUT;
pub const IOCTL_STORAGE_BC_VERSION: ULONG = 1;
STRUCT!{struct STORAGE_ALLOCATE_BC_STREAM_INPUT {
    Version: ULONG,
    RequestsPerPeriod: ULONG,
    Period: ULONG,
    RetryFailures: BOOLEAN,
    Discardable: BOOLEAN,
    Reserved1: [BOOLEAN; 2],
    AccessType: ULONG,
    AccessMode: ULONG,
}}
pub type PSTORAGE_ALLOCATE_BC_STREAM_INPUT = *mut STORAGE_ALLOCATE_BC_STREAM_INPUT;
STRUCT!{struct STORAGE_ALLOCATE_BC_STREAM_OUTPUT {
    RequestSize: ULONGLONG,
    NumOutStandingRequests: ULONG,
}}
pub type PSTORAGE_ALLOCATE_BC_STREAM_OUTPUT = *mut STORAGE_ALLOCATE_BC_STREAM_OUTPUT;
pub const STORAGE_PRIORITY_HINT_SUPPORTED: ULONG = 0x0001;
STRUCT!{struct STORAGE_PRIORITY_HINT_SUPPORT {
    SupportFlags: ULONG,
}}
pub type PSTORAGE_PRIORITY_HINT_SUPPORT = *mut STORAGE_PRIORITY_HINT_SUPPORT;
STRUCT!{struct STORAGE_MEDIA_SERIAL_NUMBER_DATA {
    Reserved: USHORT,
    SerialNumberLength: USHORT,
    SerialNumber: [UCHAR; 0],
}}
pub type PSTORAGE_MEDIA_SERIAL_NUMBER_DATA = *mut STORAGE_MEDIA_SERIAL_NUMBER_DATA;
STRUCT!{struct STORAGE_READ_CAPACITY {
    Version: ULONG,
    Size: ULONG,
    BlockLength: ULONG,
    NumberOfBlocks: LARGE_INTEGER,
    DiskLength: LARGE_INTEGER,
}}
pub type PSTORAGE_READ_CAPACITY = *mut STORAGE_READ_CAPACITY;
ENUM!{enum WRITE_CACHE_TYPE {
    WriteCacheTypeUnknown,
    WriteCacheTypeNone,
    WriteCacheTypeWriteBack,
    WriteCacheTypeWriteThrough,
}}
ENUM!{enum WRITE_CACHE_ENABLE {
    WriteCacheEnableUnknown,
    WriteCacheDisabled,
    WriteCacheEnabled,
}}
ENUM!{enum WRITE_CACHE_CHANGE {
    WriteCacheChangeUnknown,
    WriteCacheNotChangeable,
    WriteCacheChangeable,
}}
ENUM!{enum WRITE_THROUGH {
    WriteThroughUnknown,
    WriteThroughNotSupported,
    WriteThroughSupported,
}}
STRUCT!{struct STORAGE_WRITE_CACHE_PROPERTY {
    Version: ULONG,
    Size: ULONG,
    WriteCacheType: WRITE_CACHE_TYPE,
    WriteCacheEnabled: WRITE_CACHE_ENABLE,
    WriteCacheChangeable: WRITE_CACHE_CHANGE,
    WriteThroughSupported: WRITE_THROUGH,
    FlushCacheSupported: BOOLEAN,
    UserDefinedPowerProtection: BOOLEAN,
    NVCacheEnabled: BOOLEAN,
}}
pub type PSTORAGE_WRITE_CACHE_PROPERTY = *mut STORAGE_WRITE_CACHE_PROPERTY;
STRUCT!{struct PERSISTENT_RESERVE_COMMAND {
    Version: ULONG,
    Size: ULONG,
    u: PERSISTENT_RESERVE_COMMAND_u,
}}
pub type PPERSISTENT_RESERVE_COMMAND = *mut PERSISTENT_RESERVE_COMMAND;
UNION2!{union PERSISTENT_RESERVE_COMMAND_u {
    [u8; 4],
    PR_IN PR_IN_mut: PERSISTENT_RESERVE_COMMAND_u_PR_IN,
    PR_OUT PR_OUT_mut: PERSISTENT_RESERVE_COMMAND_u_PR_OUT,
}}
STRUCT!{struct PERSISTENT_RESERVE_COMMAND_u_PR_IN {
    Bitfield: UCHAR,
    AllocationLength: USHORT,
}}
BITFIELD!(PERSISTENT_RESERVE_COMMAND_u_PR_IN Bitfield: UCHAR [
    ServiceAction set_ServiceAction[0..5],
    Reserved1 set_Reserved1[5..8],
]);
STRUCT!{struct PERSISTENT_RESERVE_COMMAND_u_PR_OUT {
    Bitfield1: UCHAR,
    Bitfield2: UCHAR,
    ParameterList: [UCHAR; 0],
}}
BITFIELD!(PERSISTENT_RESERVE_COMMAND_u_PR_OUT Bitfield1: UCHAR [
    ServiceAction set_ServiceAction[0..5],
    Reserved1 set_Reserved1[5..8],
]);
BITFIELD!(PERSISTENT_RESERVE_COMMAND_u_PR_OUT Bitfield2: UCHAR [
    Type set_Type[0..4],
    Scope set_Scope[4..8],
]);
pub const DEVICEDUMP_STRUCTURE_VERSION_V1: ULONG = 1;
pub const DEVICEDUMP_MAX_IDSTRING: usize = 32;
pub const MAX_FW_BUCKET_ID_LENGTH: usize = 132;
pub const STORAGE_CRASH_TELEMETRY_REGKEY: &'static str =
    "\\Registry\\Machine\\System\\CurrentControlSet\\Control\\CrashControl\\StorageTelemetry";
pub const STORAGE_DEVICE_TELEMETRY_REGKEY: &'static str =
    "\\Registry\\Machine\\System\\CurrentControlSet\\Control\\Storage\\StorageTelemetry";
ENUM!{enum DEVICEDUMP_COLLECTION_TYPE {
    TCCollectionBugCheck = 1,
    TCCollectionApplicationRequested,
    TCCollectionDeviceRequested,
}}
pub type PDEVICEDUMP_COLLECTION_TYPE = *mut DEVICEDUMP_COLLECTION_TYPE;
pub const DDUMP_FLAG_DATA_READ_FROM_DEVICE: ULONG = 0x0001;
pub const FW_ISSUEID_NO_ISSUE: ULONG = 0x00000000;
pub const FW_ISSUEID_UNKNOWN: ULONG = 0xFFFFFFFF;
STRUCT!{#[repr(packed)] struct DEVICEDUMP_SUBSECTION_POINTER {
    dwSize: ULONG,
    dwFlags: ULONG,
    dwOffset: ULONG,
}}
pub type PDEVICEDUMP_SUBSECTION_POINTER = *mut DEVICEDUMP_SUBSECTION_POINTER;
STRUCT!{#[repr(packed)] struct DEVICEDUMP_STRUCTURE_VERSION {
    dwSignature: ULONG,
    dwVersion: ULONG,
    dwSize: ULONG,
}}
pub type PDEVICEDUMP_STRUCTURE_VERSION = *mut DEVICEDUMP_STRUCTURE_VERSION;
STRUCT!{#[repr(packed)] struct DEVICEDUMP_SECTION_HEADER {
    guidDeviceDataId: GUID,
    sOrganizationID: [UCHAR; 16],
    dwFirmwareRevision: ULONG,
    sModelNumber: [UCHAR; DEVICEDUMP_MAX_IDSTRING],
    sDeviceManufacturingID: [UCHAR; DEVICEDUMP_MAX_IDSTRING],
    dwFlags: ULONG,
    bRestrictedPrivateDataVersion: ULONG,
    dwFirmwareIssueId: ULONG,
    szIssueDescriptionString: [UCHAR; MAX_FW_BUCKET_ID_LENGTH],
}}
pub type PDEVICEDUMP_SECTION_HEADER = *mut DEVICEDUMP_SECTION_HEADER;
pub const TC_PUBLIC_DEVICEDUMP_CONTENT_SMART: ULONG = 0x01;
pub const TC_PUBLIC_DEVICEDUMP_CONTENT_GPLOG: ULONG = 0x02;
pub const TC_PUBLIC_DEVICEDUMP_CONTENT_GPLOG_MAX: usize = 16;
pub const TC_DEVICEDUMP_SUBSECTION_DESC_LENGTH: usize = 16;
pub const TC_PUBLIC_DATA_TYPE_ATAGP: &'static str = "ATAGPLogPages";
pub const TC_PUBLIC_DATA_TYPE_ATASMART: &'static str = "ATASMARTPages";
STRUCT!{#[repr(packed)] struct GP_LOG_PAGE_DESCRIPTOR {
    LogAddress: USHORT,
    LogSectors: USHORT,
}}
pub type PGP_LOG_PAGE_DESCRIPTOR = *mut GP_LOG_PAGE_DESCRIPTOR;
STRUCT!{#[repr(packed)] struct DEVICEDUMP_PUBLIC_SUBSECTION {
    dwFlags: ULONG,
    GPLogTable: [GP_LOG_PAGE_DESCRIPTOR; TC_PUBLIC_DEVICEDUMP_CONTENT_GPLOG_MAX],
    szDescription: [CHAR; TC_DEVICEDUMP_SUBSECTION_DESC_LENGTH],
    bData: [UCHAR; ANYSIZE_ARRAY],
}}
pub type PDEVICEDUMP_PUBLIC_SUBSECTION = *mut DEVICEDUMP_PUBLIC_SUBSECTION;
STRUCT!{#[repr(packed)] struct DEVICEDUMP_RESTRICTED_SUBSECTION {
    bData: [UCHAR; ANYSIZE_ARRAY],
}}
pub type PDEVICEDUMP_RESTRICTED_SUBSECTION = *mut DEVICEDUMP_RESTRICTED_SUBSECTION;
STRUCT!{#[repr(packed)] struct DEVICEDUMP_PRIVATE_SUBSECTION {
    dwFlags: ULONG,
    GPLogId: GP_LOG_PAGE_DESCRIPTOR,
    bData: [UCHAR; ANYSIZE_ARRAY],
}}
pub type PDEVICEDUMP_PRIVATE_SUBSECTION = *mut DEVICEDUMP_PRIVATE_SUBSECTION;
STRUCT!{#[repr(packed)] struct DEVICEDUMP_STORAGEDEVICE_DATA {
    Descriptor: DEVICEDUMP_STRUCTURE_VERSION,
    SectionHeader: DEVICEDUMP_SECTION_HEADER,
    dwBufferSize: ULONG,
    dwReasonForCollection: ULONG,
    PublicData: DEVICEDUMP_SUBSECTION_POINTER,
    RestrictedData: DEVICEDUMP_SUBSECTION_POINTER,
    PrivateData: DEVICEDUMP_SUBSECTION_POINTER,
}}
pub type PDEVICEDUMP_STORAGEDEVICE_DATA = *mut DEVICEDUMP_STORAGEDEVICE_DATA;
pub const CDB_SIZE: usize = 16;
pub const TELEMETRY_COMMAND_SIZE: usize = 16;
STRUCT!{#[repr(packed)] struct DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD {
    Cdb: [UCHAR; CDB_SIZE],
    Command: [UCHAR; TELEMETRY_COMMAND_SIZE],
    StartTime: ULONGLONG,
    EndTime: ULONGLONG,
    OperationStatus: ULONG,
    OperationError: ULONG,
    StackSpecific: DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD_StackSpecific,
}}
pub type PDEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD =
    *mut DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD;
impl DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD {
    pub fn TCRecordStorportSrbFunction(&self) -> &UCHAR {
        &self.Command[0]
    }
    pub fn TCRecordStorportSrbFunction_mut(&mut self) -> &mut UCHAR {
        &mut self.Command[0]
    }
}
UNION2!{union DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD_StackSpecific {
    [u32; 1],
    ExternalStack ExternalStack_mut:
        DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD_StackSpecific_ExternalStack,
    AtaPort AtaPort_mut: DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD_StackSpecific_AtaPort,
    StorPort StorPort_mut: DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD_StackSpecific_StorPort,
}}
STRUCT!{#[repr(packed)]
struct DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD_StackSpecific_ExternalStack {
    dwReserved: ULONG,
}}
STRUCT!{#[repr(packed)] struct DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD_StackSpecific_AtaPort {
    dwAtaPortSpecific: ULONG,
}}
STRUCT!{#[repr(packed)]
struct DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD_StackSpecific_StorPort {
    SrbTag: ULONG,
}}
STRUCT!{#[repr(packed)] struct DEVICEDUMP_STORAGESTACK_PUBLIC_DUMP {
    Descriptor: DEVICEDUMP_STRUCTURE_VERSION,
    dwReasonForCollection: ULONG,
    cDriverName: [UCHAR; 16],
    uiNumRecords: ULONG,
    RecordArray: [DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD; ANYSIZE_ARRAY],
}}
pub type PDEVICEDUMP_STORAGESTACK_PUBLIC_DUMP = *mut DEVICEDUMP_STORAGESTACK_PUBLIC_DUMP;
pub const DEVICEDUMP_CAP_PRIVATE_SECTION: ULONG = 0x00000001;
pub const DEVICEDUMP_CAP_RESTRICTED_SECTION: ULONG = 0x00000002;
STRUCT!{struct STORAGE_IDLE_POWER {
    Version: ULONG,
    Size: ULONG,
    Bitfield: ULONG,
    D3IdleTimeout: ULONG,
}}
pub type PSTORAGE_IDLE_POWER = *mut STORAGE_IDLE_POWER;
BITFIELD!(STORAGE_IDLE_POWER Bitfield: ULONG [
    WakeCapableHint set_WakeCapableHint[0..1],
    D3ColdSupported set_D3ColdSupported[1..2],
    Reserved set_Reserved[2..32],
]);
ENUM!{enum STORAGE_POWERUP_REASON_TYPE {
  StoragePowerupUnknown = 0,
  StoragePowerupIO,
  StoragePowerupDeviceAttention,
}}
pub type PSTORAGE_POWERUP_REASON_TYPE = *mut STORAGE_POWERUP_REASON_TYPE;
STRUCT!{struct STORAGE_IDLE_POWERUP_REASON {
    Version: ULONG,
    Size: ULONG,
    PowerupReason: STORAGE_POWERUP_REASON_TYPE,
}}
pub type PSTORAGE_IDLE_POWERUP_REASON = *mut STORAGE_IDLE_POWERUP_REASON;
pub const STORAGE_IDLE_POWERUP_REASON_VERSION_V1: ULONG = 1;
ENUM!{enum STORAGE_DEVICE_POWER_CAP_UNITS {
    StorageDevicePowerCapUnitsPercent,
    StorageDevicePowerCapUnitsMilliwatts,
}}
pub type PSTORAGE_DEVICE_POWER_CAP_UNITS = *mut STORAGE_DEVICE_POWER_CAP_UNITS;
STRUCT!{struct STORAGE_DEVICE_POWER_CAP {
    Version: ULONG,
    Size: ULONG,
    Units: STORAGE_DEVICE_POWER_CAP_UNITS,
    MaxPower: ULONGLONG,
}}
pub type PSTORAGE_DEVICE_POWER_CAP = *mut STORAGE_DEVICE_POWER_CAP;
pub const STORAGE_DEVICE_POWER_CAP_VERSION_V1: ULONG = 1;
STRUCT!{#[repr(packed)] struct STORAGE_RPMB_DATA_FRAME {
    Stuff: [UCHAR; 196],
    KeyOrMAC: [UCHAR; 32],
    Data: [UCHAR; 256],
    Nonce: [UCHAR; 16],
    WriteCounter: [UCHAR; 4],
    Address: [UCHAR; 2],
    BlockCount: [UCHAR; 2],
    OperationResult: [UCHAR; 2],
    RequestOrResponseType: [UCHAR; 2],
}}
pub type PSTORAGE_RPMB_DATA_FRAME = *mut STORAGE_RPMB_DATA_FRAME;
ENUM!{enum STORAGE_RPMB_COMMAND_TYPE {
    StorRpmbProgramAuthKey = 0x00000001,
    StorRpmbQueryWriteCounter = 0x00000002,
    StorRpmbAuthenticatedWrite = 0x00000003,
    StorRpmbAuthenticatedRead = 0x00000004,
    StorRpmbReadResultRequest = 0x00000005,
    StorRpmbAuthenticatedDeviceConfigWrite = 0x00000006,
    StorRpmbAuthenticatedDeviceConfigRead  = 0x00000007,
}}
pub type PSTORAGE_RPMB_COMMAND_TYPE = *mut STORAGE_RPMB_COMMAND_TYPE;
STRUCT!{struct STORAGE_EVENT_NOTIFICATION {
    Version: ULONG,
    Size: ULONG,
    Events: ULONGLONG,
}}
pub type PSTORAGE_EVENT_NOTIFICATION = *mut STORAGE_EVENT_NOTIFICATION;
pub const STORAGE_EVENT_NOTIFICATION_VERSION_V1: ULONG = 1;
pub const STORAGE_EVENT_MEDIA_STATUS: ULONGLONG = 0x0000000000000001;
pub const STORAGE_EVENT_DEVICE_STATUS: ULONGLONG = 0x0000000000000002;
pub const STORAGE_EVENT_DEVICE_OPERATION: ULONGLONG = 0x0000000000000004;
pub const STORAGE_EVENT_ALL: ULONGLONG =
    (STORAGE_EVENT_MEDIA_STATUS | STORAGE_EVENT_DEVICE_STATUS | STORAGE_EVENT_DEVICE_OPERATION);
pub const READ_COPY_NUMBER_KEY: ULONG = 0x52434e00;
#[inline]
pub fn IsKeyReadCopyNumber(_k: ULONG) -> bool {
    (_k & 0xFFFFFF00) == READ_COPY_NUMBER_KEY
}
#[inline]
pub fn ReadCopyNumberToKey(_c: UCHAR) -> ULONG {
    READ_COPY_NUMBER_KEY | (_c as ULONG)
}
#[inline]
pub fn ReadCopyNumberFromKey(_k: ULONG) -> UCHAR {
    (_k & 0x000000FF) as UCHAR
}
ENUM!{enum STORAGE_COUNTER_TYPE {
    StorageCounterTypeUnknown = 0,
    StorageCounterTypeTemperatureCelsius,
    StorageCounterTypeTemperatureCelsiusMax,
    StorageCounterTypeReadErrorsTotal,
    StorageCounterTypeReadErrorsCorrected,
    StorageCounterTypeReadErrorsUncorrected,
    StorageCounterTypeWriteErrorsTotal,
    StorageCounterTypeWriteErrorsCorrected,
    StorageCounterTypeWriteErrorsUncorrected,
    StorageCounterTypeManufactureDate,
    StorageCounterTypeStartStopCycleCount,
    StorageCounterTypeStartStopCycleCountMax,
    StorageCounterTypeLoadUnloadCycleCount,
    StorageCounterTypeLoadUnloadCycleCountMax,
    StorageCounterTypeWearPercentage,
    StorageCounterTypeWearPercentageWarning,
    StorageCounterTypeWearPercentageMax,
    StorageCounterTypePowerOnHours,
    StorageCounterTypeReadLatency100NSMax,
    StorageCounterTypeWriteLatency100NSMax,
    StorageCounterTypeFlushLatency100NSMax,
    StorageCounterTypeMax,
}}
pub type PSTORAGE_COUNTER_TYPE = *mut STORAGE_COUNTER_TYPE;
STRUCT!{struct STORAGE_COUNTER {
    Type: STORAGE_COUNTER_TYPE,
    Value: STORAGE_COUNTER_Value,
}}
pub type PSTORAGE_COUNTER = *mut STORAGE_COUNTER;
UNION2!{union STORAGE_COUNTER_Value {
    [u64; 1],
    ManufactureDate ManufactureDate_mut: STORAGE_COUNTER_Value_ManufactureDate,
    AsUlonglon AsUlonglong_mut: ULONGLONG,
}}
STRUCT!{struct STORAGE_COUNTER_Value_ManufactureDate {
    Week: ULONG,
    Year: ULONG,
}}
STRUCT!{struct STORAGE_COUNTERS {
    Version: ULONG,
    Size: ULONG,
    NumberOfCounters: ULONG,
    Counters: [STORAGE_COUNTER; ANYSIZE_ARRAY],
}}
pub type PSTORAGE_COUNTERS = *mut STORAGE_COUNTERS;
// TODO: Revisit this when const fn size_of arrives.
pub const STORAGE_COUNTERS_VERSION_V1: ULONG = 32;
pub const STORAGE_HW_FIRMWARE_REQUEST_FLAG_CONTROLLER: ULONG = 0x00000001;
pub const STORAGE_HW_FIRMWARE_REQUEST_FLAG_LAST_SEGMENT: ULONG = 0x00000002;
pub const STORAGE_HW_FIRMWARE_REQUEST_FLAG_SWITCH_TO_EXISTING_FIRMWARE: ULONG = 0x80000000;
STRUCT!{struct STORAGE_HW_FIRMWARE_INFO_QUERY {
    Version: ULONG,
    Size: ULONG,
    Flags: ULONG,
    Reserved: ULONG,
}}
pub type PSTORAGE_HW_FIRMWARE_INFO_QUERY = *mut STORAGE_HW_FIRMWARE_INFO_QUERY;
pub const STORAGE_HW_FIRMWARE_INVALID_SLOT: UCHAR = 0xFF;
pub const STORAGE_HW_FIRMWARE_REVISION_LENGTH: usize = 16;
STRUCT!{struct STORAGE_HW_FIRMWARE_SLOT_INFO {
    Version: ULONG,
    Size: ULONG,
    SlotNumber: UCHAR,
    Bitfield: UCHAR,
    Reserved1: [UCHAR; 6],
    Revision: [UCHAR; STORAGE_HW_FIRMWARE_REVISION_LENGTH],
}}
pub type PSTORAGE_HW_FIRMWARE_SLOT_INFO = *mut STORAGE_HW_FIRMWARE_SLOT_INFO;
BITFIELD!(STORAGE_HW_FIRMWARE_SLOT_INFO Bitfield: UCHAR [
    ReadOnly set_ReadOnly[0..1],
    Reserved0 set_Reserved0[1..8],
]);
STRUCT!{struct STORAGE_HW_FIRMWARE_INFO {
    Version: ULONG,
    Size: ULONG,
    Bitfield: UCHAR,
    SlotCount: UCHAR,
    ActiveSlot: UCHAR,
    PendingActiveSlot: UCHAR,
    FirmwareShared: BOOLEAN,
    Reserved: [UCHAR; 3],
    ImagePayloadAlignment: ULONG,
    ImagePayloadMaxSize: ULONG,
    Slot: [STORAGE_HW_FIRMWARE_SLOT_INFO; ANYSIZE_ARRAY],
}}
pub type PSTORAGE_HW_FIRMWARE_INFO = *mut STORAGE_HW_FIRMWARE_INFO;
BITFIELD!(STORAGE_HW_FIRMWARE_INFO Bitfield: UCHAR [
    SupportUpgrade set_SupportUpgrade[0..1],
    Reserved0 set_Reserved0[1..8],
]);
STRUCT!{struct STORAGE_HW_FIRMWARE_DOWNLOAD {
    Version: ULONG,
    Size: ULONG,
    Flags: ULONG,
    Slot: UCHAR,
    Reserved: [UCHAR; 3],
    Offset: ULONGLONG,
    BufferSize: ULONGLONG,
    ImageBuffer: [UCHAR; ANYSIZE_ARRAY],
}}
pub type PSTORAGE_HW_FIRMWARE_DOWNLOAD = *mut STORAGE_HW_FIRMWARE_DOWNLOAD;
STRUCT!{struct STORAGE_HW_FIRMWARE_ACTIVATE {
    Version: ULONG,
    Size: ULONG,
    Flags: ULONG,
    Slot: UCHAR,
    Reserved0: [UCHAR; 3],
}}
pub type PSTORAGE_HW_FIRMWARE_ACTIVATE = *mut STORAGE_HW_FIRMWARE_ACTIVATE;
pub const STORAGE_PROTOCOL_STRUCTURE_VERSION: ULONG = 0x1;
STRUCT!{struct STORAGE_PROTOCOL_COMMAND {
    Version: ULONG,
    Length: ULONG,
    ProtocolType: STORAGE_PROTOCOL_TYPE,
    Flags: ULONG,
    ReturnStatus: ULONG,
    ErrorCode: ULONG,
    CommandLength: ULONG,
    ErrorInfoLength: ULONG,
    DataToDeviceTransferLength: ULONG,
    DataFromDeviceTransferLength: ULONG,
    TimeOutValue: ULONG,
    ErrorInfoOffset: ULONG,
    DataToDeviceBufferOffset: ULONG,
    DataFromDeviceBufferOffset: ULONG,
    CommandSpecific: ULONG,
    Reserved0: ULONG,
    FixedProtocolReturnData: ULONG,
    Reserved1: [ULONG; 3],
    Command: [UCHAR; ANYSIZE_ARRAY],
}}
pub type PSTORAGE_PROTOCOL_COMMAND = *mut STORAGE_PROTOCOL_COMMAND;
pub const STORAGE_PROTOCOL_COMMAND_FLAG_ADAPTER_REQUEST: ULONG = 0x80000000;
pub const STORAGE_PROTOCOL_STATUS_PENDING: ULONG = 0x0;
pub const STORAGE_PROTOCOL_STATUS_SUCCESS: ULONG = 0x1;
pub const STORAGE_PROTOCOL_STATUS_ERROR: ULONG = 0x2;
pub const STORAGE_PROTOCOL_STATUS_INVALID_REQUEST: ULONG = 0x3;
pub const STORAGE_PROTOCOL_STATUS_NO_DEVICE: ULONG = 0x4;
pub const STORAGE_PROTOCOL_STATUS_BUSY: ULONG = 0x5;
pub const STORAGE_PROTOCOL_STATUS_DATA_OVERRUN: ULONG = 0x6;
pub const STORAGE_PROTOCOL_STATUS_INSUFFICIENT_RESOURCES: ULONG = 0x7;
pub const STORAGE_PROTOCOL_STATUS_NOT_SUPPORTED: ULONG = 0xFF;
pub const STORAGE_PROTOCOL_COMMAND_LENGTH_NVME: ULONG = 0x40;
pub const STORAGE_PROTOCOL_SPECIFIC_NVME_ADMIN_COMMAND: ULONG = 0x01;
pub const STORAGE_PROTOCOL_SPECIFIC_NVME_NVM_COMMAND: ULONG = 0x02;
ENUM!{enum STORAGE_ATTRIBUTE_MGMT_ACTION {
    StorAttributeMgmt_ClearAttribute = 0,
    StorAttributeMgmt_SetAttribute = 1,
    StorAttributeMgmt_ResetAttribute = 2,
}}
pub type PSTORAGE_ATTRIBUTE_MGMT_ACTION = *mut STORAGE_ATTRIBUTE_MGMT_ACTION;
pub const STORATTRIBUTE_NONE: ULONG = 0;
pub const STORATTRIBUTE_MANAGEMENT_STATE: ULONG = 1;
STRUCT!{struct STORAGE_ATTRIBUTE_MGMT {
    Version: ULONG,
    Size: ULONG,
    Action: STORAGE_ATTRIBUTE_MGMT_ACTION,
    Attribute: ULONG,
}}
pub type PSTORAGE_ATTRIBUTE_MGMT = *mut STORAGE_ATTRIBUTE_MGMT;

use shared::minwindef::{UCHAR, ULONG};
use um::winnt::{BOOLEAN};

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum _STORAGE_QUERY_TYPE {
    PropertyStandardQuery = 0,
    PropertyExistsQuery = 1,
    PropertyMaskQuery = 2,
    PropertyQueryMaxDefined = 3,
}
pub use self::_STORAGE_QUERY_TYPE as STORAGE_QUERY_TYPE;
pub type PSTORAGE_QUERY_TYPE = *mut _STORAGE_QUERY_TYPE;
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum _STORAGE_PROPERTY_ID {
    StorageDeviceProperty = 0,
    StorageAdapterProperty = 1,
    StorageDeviceIdProperty = 2,
    StorageDeviceUniqueIdProperty = 3,
    StorageDeviceWriteCacheProperty = 4,
    StorageMiniportProperty = 5,
    StorageAccessAlignmentProperty = 6,
    StorageDeviceSeekPenaltyProperty = 7,
    StorageDeviceTrimProperty = 8,
    StorageDeviceWriteAggregationProperty = 9,
    StorageDeviceDeviceTelemetryProperty = 10,
    StorageDeviceLBProvisioningProperty = 11,
    StorageDevicePowerProperty = 12,
    StorageDeviceCopyOffloadProperty = 13,
    StorageDeviceResiliencyProperty = 14,
    StorageDeviceMediumProductType = 15,
    StorageDeviceTieringProperty = 16,
}
pub use self::_STORAGE_PROPERTY_ID as STORAGE_PROPERTY_ID;
pub type PSTORAGE_PROPERTY_ID = *mut _STORAGE_PROPERTY_ID;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _STORAGE_PROPERTY_QUERY {
    pub PropertyId: STORAGE_PROPERTY_ID,
    pub QueryType: STORAGE_QUERY_TYPE,
    pub AdditionalParameters: [UCHAR; 1usize],
}
pub type STORAGE_PROPERTY_QUERY = _STORAGE_PROPERTY_QUERY;
pub type PSTORAGE_PROPERTY_QUERY = *mut _STORAGE_PROPERTY_QUERY;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _STORAGE_DESCRIPTOR_HEADER {
    pub Version: ULONG,
    pub Size: ULONG,
}
pub type STORAGE_DESCRIPTOR_HEADER = _STORAGE_DESCRIPTOR_HEADER;
pub type PSTORAGE_DESCRIPTOR_HEADER = *mut _STORAGE_DESCRIPTOR_HEADER;
pub type STORAGE_BUS_TYPE = _STORAGE_BUS_TYPE;
pub type STORAGE_DEVICE_DESCRIPTOR = _STORAGE_DEVICE_DESCRIPTOR;
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum _STORAGE_BUS_TYPE {
    BusTypeUnknown = 0,
    BusTypeScsi = 1,
    BusTypeAtapi = 2,
    BusTypeAta = 3,
    BusType1394 = 4,
    BusTypeSsa = 5,
    BusTypeFibre = 6,
    BusTypeUsb = 7,
    BusTypeRAID = 8,
    BusTypeiScsi = 9,
    BusTypeSas = 10,
    BusTypeSata = 11,
    BusTypeSd = 12,
    BusTypeMmc = 13,
    BusTypeVirtual = 14,
    BusTypeFileBackedVirtual = 15,
    BusTypeSpaces = 16,
    BusTypeNvme = 17,
    BusTypeMax = 18,
    BusTypeMaxReserved = 127,
}

impl Default for _STORAGE_BUS_TYPE {
    fn default() -> Self {
        _STORAGE_BUS_TYPE::BusTypeUnknown
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct _STORAGE_DEVICE_DESCRIPTOR {
    pub Version: ULONG,
    pub Size: ULONG,
    pub DeviceType: UCHAR,
    pub DeviceTypeModifier: UCHAR,
    pub RemovableMedia: BOOLEAN,
    pub CommandQueueing: BOOLEAN,
    pub VendorIdOffset: ULONG,
    pub ProductIdOffset: ULONG,
    pub ProductRevisionOffset: ULONG,
    pub SerialNumberOffset: ULONG,
    pub BusType: STORAGE_BUS_TYPE,
    pub RawPropertiesLength: ULONG,
    pub RawDeviceProperties: [UCHAR; 1usize],
}

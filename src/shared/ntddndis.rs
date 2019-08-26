// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::ifdef::IF_MAX_STRING_SIZE;
pub const NDIS_IF_MAX_STRING_SIZE: usize = IF_MAX_STRING_SIZE;
ENUM!{enum NDIS_MEDIUM {
    NdisMedium802_3 = 0,
    NdisMedium802_5 = 1,
    NdisMediumFddi = 2,
    NdisMediumWan = 3,
    NdisMediumLocalTalk = 4,
    NdisMediumDix = 5, // defined for convenience, not a real medium
    NdisMediumArcnetRaw = 6,
    NdisMediumArcnet878_2 = 7,
    NdisMediumAtm = 8,
    NdisMediumWirelessWan = 9,
    NdisMediumIrda = 10,
    NdisMediumBpc = 11,
    NdisMediumCoWan = 12,
    NdisMedium1394 = 13,
    NdisMediumInfiniBand = 14,
// #if ((NTDDI_VERSION >= NTDDI_VISTA) || NDIS_SUPPORT_NDIS6)
    NdisMediumTunnel = 15,
    NdisMediumNative802_11 = 16,
    NdisMediumLoopback = 17,
// #endif // (NTDDI_VERSION >= NTDDI_VISTA)
// #if (NTDDI_VERSION >= NTDDI_WIN7)
    NdisMediumWiMAX = 18,
    NdisMediumIP = 19,
// #endif
    NdisMediumMax = 20, // Not a real medium, defined as an upper-bound
}}
pub type PNDIS_MEDIUM = *mut NDIS_MEDIUM;
ENUM!{enum NDIS_PHYSICAL_MEDIUM {
    NdisPhysicalMediumUnspecified = 0,
    NdisPhysicalMediumWirelessLan = 1,
    NdisPhysicalMediumCableModem = 2,
    NdisPhysicalMediumPhoneLine = 3,
    NdisPhysicalMediumPowerLine = 4,
    NdisPhysicalMediumDSL = 5, // includes ADSL and UADSL (G.Lite)
    NdisPhysicalMediumFibreChannel = 6,
    NdisPhysicalMedium1394 = 7,
    NdisPhysicalMediumWirelessWan = 8,
    NdisPhysicalMediumNative802_11 = 9,
    NdisPhysicalMediumBluetooth = 10,
    NdisPhysicalMediumInfiniband = 11,
    NdisPhysicalMediumWiMax = 12,
    NdisPhysicalMediumUWB = 13,
    NdisPhysicalMedium802_3 = 14,
    NdisPhysicalMedium802_5 = 15,
    NdisPhysicalMediumIrda = 16,
    NdisPhysicalMediumWiredWAN = 17,
    NdisPhysicalMediumWiredCoWan = 18,
    NdisPhysicalMediumOther = 19,
    NdisPhysicalMediumMax = 20, // Not a real physical type, defined as an upper-bound
}}
pub type PNDIS_PHYSICAL_MEDIUM = *mut NDIS_PHYSICAL_MEDIUM;

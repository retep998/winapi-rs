// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! This file contains custom property definitions for a PCI root bus and a PCI device.
use shared::devpropdef::DEVPROPKEY;
use shared::minwindef::ULONG;
macro_rules! DEFINE_PCI_ROOT_BUS_DEVPKEY {
    ($_DevPkeyName:ident, $_Pid:expr) => {
        DEFINE_DEVPROPKEY!{
            $_DevPkeyName, 0xd817fc28, 0x793e, 0x4b9e, 0x99, 0x70, 0x46, 0x9d, 0x8b, 0xe6, 0x30,
            0x73, $_Pid
        }
    };
}
pub const DevProp_PciRootBus_SecondaryInterface_PciConventional: ULONG = 0;
pub const DevProp_PciRootBus_SecondaryInterface_PciXMode1: ULONG = 1;
pub const DevProp_PciRootBus_SecondaryInterface_PciXMode2: ULONG = 2;
pub const DevProp_PciRootBus_SecondaryInterface_PciExpress: ULONG = 3;
DEFINE_PCI_ROOT_BUS_DEVPKEY!{DEVPKEY_PciRootBus_SecondaryInterface, 1}
pub const DevProp_PciRootBus_CurrentSpeedAndMode_Pci_Conventional_33Mhz: ULONG = 0;
pub const DevProp_PciRootBus_CurrentSpeedAndMode_Pci_Conventional_66Mhz: ULONG = 1;
pub const DevProp_PciRootBus_CurrentSpeedAndMode_Pci_X_Mode1_66Mhz: ULONG = 2;
pub const DevProp_PciRootBus_CurrentSpeedAndMode_Pci_X_Mode1_100Mhz: ULONG = 3;
pub const DevProp_PciRootBus_CurrentSpeedAndMode_Pci_X_Mode1_133Mhz: ULONG = 4;
pub const DevProp_PciRootBus_CurrentSpeedAndMode_Pci_X_Mode1_ECC_66Mhz: ULONG = 5;
pub const DevProp_PciRootBus_CurrentSpeedAndMode_Pci_X_Mode1_ECC_100Mhz: ULONG = 6;
pub const DevProp_PciRootBus_CurrentSpeedAndMode_Pci_X_Mode1_ECC_133Mhz: ULONG = 7;
pub const DevProp_PciRootBus_CurrentSpeedAndMode_Pci_X_266_Mode2_66Mhz: ULONG = 8;
pub const DevProp_PciRootBus_CurrentSpeedAndMode_Pci_X_266_Mode2_100Mhz: ULONG = 9;
pub const DevProp_PciRootBus_CurrentSpeedAndMode_Pci_X_266_Mode2_133Mhz: ULONG = 10;
pub const DevProp_PciRootBus_CurrentSpeedAndMode_Pci_X_533_Mode2_66Mhz: ULONG = 11;
pub const DevProp_PciRootBus_CurrentSpeedAndMode_Pci_X_533_Mode2_100Mhz: ULONG = 12;
pub const DevProp_PciRootBus_CurrentSpeedAndMode_Pci_X_533_Mode2_133Mhz: ULONG = 13;
DEFINE_PCI_ROOT_BUS_DEVPKEY!{DEVPKEY_PciRootBus_CurrentSpeedAndMode, 2}
pub const DevProp_PciRootBus_SupportedSpeedsAndModes_Pci_Conventional_33Mhz: ULONG = 1;
pub const DevProp_PciRootBus_SupportedSpeedsAndModes_Pci_Conventional_66Mhz: ULONG = 2;
pub const DevProp_PciRootBus_SupportedSpeedsAndModes_Pci_X_66Mhz: ULONG = 4;
pub const DevProp_PciRootBus_SupportedSpeedsAndModes_Pci_X_133Mhz: ULONG = 8;
pub const DevProp_PciRootBus_SupportedSpeedsAndModes_Pci_X_266Mhz: ULONG = 16;
pub const DevProp_PciRootBus_SupportedSpeedsAndModes_Pci_X_533Mhz: ULONG = 32;
DEFINE_PCI_ROOT_BUS_DEVPKEY!{DEVPKEY_PciRootBus_SupportedSpeedsAndModes, 3}
DEFINE_PCI_ROOT_BUS_DEVPKEY!{DEVPKEY_PciRootBus_DeviceIDMessagingCapable, 4}
pub const DevProp_PciRootBus_BusWidth_32Bits: ULONG = 0;
pub const DevProp_PciRootBus_BusWidth_64Bits: ULONG = 1;
DEFINE_PCI_ROOT_BUS_DEVPKEY!{DEVPKEY_PciRootBus_SecondaryBusWidth, 5}
DEFINE_PCI_ROOT_BUS_DEVPKEY!{DEVPKEY_PciRootBus_ExtendedConfigAvailable, 6}
DEFINE_PCI_ROOT_BUS_DEVPKEY!{DEVPKEY_PciRootBus_ExtendedPCIConfigOpRegionSupport, 7}
DEFINE_PCI_ROOT_BUS_DEVPKEY!{DEVPKEY_PciRootBus_ASPMSupport, 8}
DEFINE_PCI_ROOT_BUS_DEVPKEY!{DEVPKEY_PciRootBus_ClockPowerManagementSupport, 9}
DEFINE_PCI_ROOT_BUS_DEVPKEY!{DEVPKEY_PciRootBus_PCISegmentGroupsSupport, 10}
DEFINE_PCI_ROOT_BUS_DEVPKEY!{DEVPKEY_PciRootBus_MSISupport, 11}
DEFINE_PCI_ROOT_BUS_DEVPKEY!{DEVPKEY_PciRootBus_PCIExpressNativeHotPlugControl, 12}
DEFINE_PCI_ROOT_BUS_DEVPKEY!{DEVPKEY_PciRootBus_SHPCNativeHotPlugControl, 13}
DEFINE_PCI_ROOT_BUS_DEVPKEY!{DEVPKEY_PciRootBus_PCIExpressNativePMEControl, 14}
DEFINE_PCI_ROOT_BUS_DEVPKEY!{DEVPKEY_PciRootBus_PCIExpressAERControl, 15}
DEFINE_PCI_ROOT_BUS_DEVPKEY!{DEVPKEY_PciRootBus_PCIExpressCapabilityControl, 16}
DEFINE_PCI_ROOT_BUS_DEVPKEY!{DEVPKEY_PciRootBus_NativePciExpressControl, 17}
DEFINE_PCI_ROOT_BUS_DEVPKEY!{DEVPKEY_PciRootBus_SystemMsiSupport, 18}
macro_rules! DEFINE_PCI_DEVICE_DEVPKEY {
    ($_DevPkeyName:ident, $_Pid:expr) => {
        DEFINE_DEVPROPKEY!{
            $_DevPkeyName, 0x3ab22e31, 0x8264, 0x4b4e, 0x9a, 0xf5, 0xa8, 0xd2, 0xd8, 0xe3, 0x3e,
            0x62, $_Pid
        }
    };
}
pub const DevProp_PciDevice_DeviceType_PciConventional: ULONG = 0;
pub const DevProp_PciDevice_DeviceType_PciX: ULONG = 1;
pub const DevProp_PciDevice_DeviceType_PciExpressEndpoint: ULONG = 2;
pub const DevProp_PciDevice_DeviceType_PciExpressLegacyEndpoint: ULONG = 3;
pub const DevProp_PciDevice_DeviceType_PciExpressRootComplexIntegratedEndpoint: ULONG = 4;
pub const DevProp_PciDevice_DeviceType_PciExpressTreatedAsPci: ULONG = 5;
pub const DevProp_PciDevice_BridgeType_PciConventional: ULONG = 6;
pub const DevProp_PciDevice_BridgeType_PciX: ULONG = 7;
pub const DevProp_PciDevice_BridgeType_PciExpressRootPort: ULONG = 8;
pub const DevProp_PciDevice_BridgeType_PciExpressUpstreamSwitchPort: ULONG = 9;
pub const DevProp_PciDevice_BridgeType_PciExpressDownstreamSwitchPort: ULONG = 10;
pub const DevProp_PciDevice_BridgeType_PciExpressToPciXBridge: ULONG = 11;
pub const DevProp_PciDevice_BridgeType_PciXToExpressBridge: ULONG = 12;
pub const DevProp_PciDevice_BridgeType_PciExpressTreatedAsPci: ULONG = 13;
pub const DevProp_PciDevice_BridgeType_PciExpressEventCollector: ULONG = 14;
DEFINE_PCI_DEVICE_DEVPKEY!{DEVPKEY_PciDevice_DeviceType, 1}
pub const DevProp_PciDevice_CurrentSpeedAndMode_Pci_Conventional_33MHz: ULONG = 0;
pub const DevProp_PciDevice_CurrentSpeedAndMode_Pci_Conventional_66MHz: ULONG = 1;
pub const DevProp_PciDevice_CurrentSpeedAndMode_PciX_Mode_Conventional_Pci: ULONG = 0x0;
pub const DevProp_PciDevice_CurrentSpeedAndMode_PciX_Mode1_66Mhz: ULONG = 0x1;
pub const DevProp_PciDevice_CurrentSpeedAndMode_PciX_Mode1_100Mhz: ULONG = 0x2;
pub const DevProp_PciDevice_CurrentSpeedAndMode_PciX_Mode1_133MHZ: ULONG = 0x3;
pub const DevProp_PciDevice_CurrentSpeedAndMode_PciX_Mode1_ECC_66Mhz: ULONG = 0x5;
pub const DevProp_PciDevice_CurrentSpeedAndMode_PciX_Mode1_ECC_100Mhz: ULONG = 0x6;
pub const DevProp_PciDevice_CurrentSpeedAndMode_PciX_Mode1_ECC_133Mhz: ULONG = 0x7;
pub const DevProp_PciDevice_CurrentSpeedAndMode_PciX_Mode2_266_66MHz: ULONG = 0x9;
pub const DevProp_PciDevice_CurrentSpeedAndMode_PciX_Mode2_266_100MHz: ULONG = 0xa;
pub const DevProp_PciDevice_CurrentSpeedAndMode_PciX_Mode2_266_133MHz: ULONG = 0xb;
pub const DevProp_PciDevice_CurrentSpeedAndMode_PciX_Mode2_533_66MHz: ULONG = 0xd;
pub const DevProp_PciDevice_CurrentSpeedAndMode_PciX_Mode2_533_100MHz: ULONG = 0xe;
pub const DevProp_PciDevice_CurrentSpeedAndMode_PciX_Mode2_533_133MHz: ULONG = 0xf;
DEFINE_PCI_DEVICE_DEVPKEY!{DEVPKEY_PciDevice_CurrentSpeedAndMode, 2}
DEFINE_PCI_DEVICE_DEVPKEY!{DEVPKEY_PciDevice_BaseClass, 3}
DEFINE_PCI_DEVICE_DEVPKEY!{DEVPKEY_PciDevice_SubClass, 4}
DEFINE_PCI_DEVICE_DEVPKEY!{DEVPKEY_PciDevice_ProgIf, 5}
pub const DevProp_PciExpressDevice_PayloadOrRequestSize_128Bytes: ULONG = 0;
pub const DevProp_PciExpressDevice_PayloadOrRequestSize_256Bytes: ULONG = 1;
pub const DevProp_PciExpressDevice_PayloadOrRequestSize_512Bytes: ULONG = 2;
pub const DevProp_PciExpressDevice_PayloadOrRequestSize_1024Bytes: ULONG = 3;
pub const DevProp_PciExpressDevice_PayloadOrRequestSize_2048Bytes: ULONG = 4;
pub const DevProp_PciExpressDevice_PayloadOrRequestSize_4096Bytes: ULONG = 5;
DEFINE_PCI_DEVICE_DEVPKEY!{DEVPKEY_PciDevice_CurrentPayloadSize, 6}
DEFINE_PCI_DEVICE_DEVPKEY!{DEVPKEY_PciDevice_MaxPayloadSize, 7}
DEFINE_PCI_DEVICE_DEVPKEY!{DEVPKEY_PciDevice_MaxReadRequestSize, 8}
pub const DevProp_PciExpressDevice_LinkSpeed_TwoAndHalf_Gbps: ULONG = 1;
pub const DevProp_PciExpressDevice_LinkSpeed_Five_Gbps: ULONG = 2;
DEFINE_PCI_DEVICE_DEVPKEY!{DEVPKEY_PciDevice_CurrentLinkSpeed, 9}
pub const DevProp_PciExpressDevice_LinkWidth_By_1: ULONG = 1;
pub const DevProp_PciExpressDevice_LinkWidth_By_2: ULONG = 2;
pub const DevProp_PciExpressDevice_LinkWidth_By_4: ULONG = 4;
pub const DevProp_PciExpressDevice_LinkWidth_By_8: ULONG = 8;
pub const DevProp_PciExpressDevice_LinkWidth_By_12: ULONG = 12;
pub const DevProp_PciExpressDevice_LinkWidth_By_16: ULONG = 16;
pub const DevProp_PciExpressDevice_LinkWidth_By_32: ULONG = 32;
DEFINE_PCI_DEVICE_DEVPKEY!{DEVPKEY_PciDevice_CurrentLinkWidth, 10}
DEFINE_PCI_DEVICE_DEVPKEY!{DEVPKEY_PciDevice_MaxLinkSpeed, 11}
DEFINE_PCI_DEVICE_DEVPKEY!{DEVPKEY_PciDevice_MaxLinkWidth, 12}
pub const DevProp_PciExpressDevice_Spec_Version_10: ULONG = 1;
pub const DevProp_PciExpressDevice_Spec_Version_11: ULONG = 2;
DEFINE_PCI_DEVICE_DEVPKEY!{DEVPKEY_PciDevice_ExpressSpecVersion, 13}
pub const DevProp_PciDevice_InterruptType_LineBased: ULONG = 1;
pub const DevProp_PciDevice_InterruptType_Msi: ULONG = 2;
pub const DevProp_PciDevice_InterruptType_MsiX: ULONG = 4;
DEFINE_PCI_DEVICE_DEVPKEY!{DEVPKEY_PciDevice_InterruptSupport, 14}
DEFINE_PCI_DEVICE_DEVPKEY!{DEVPKEY_PciDevice_InterruptMessageMaximum, 15}
#[inline]
pub fn DevProp_PciDevice_IoBarCount(_PropertyData: u32) -> u32 {
    _PropertyData & 0xff
}
#[inline]
pub fn DevProp_PciDevice_NonPrefetchable_MemoryBarCount(_PropertyData: u32) -> u32 {
    (_PropertyData >> 8) & 0xff
}
#[inline]
pub fn DevProp_PciDevice_32BitPrefetchable_MemoryBarCount(_PropertyData: u32) -> u32 {
    (_PropertyData >> 16) & 0xff
}
#[inline]
pub fn DevProp_PciDevice_64BitPrefetchable_MemoryBarCount(_PropertyData: u32) -> u32 {
    (_PropertyData >> 24) & 0xff
}
DEFINE_PCI_DEVICE_DEVPKEY!{DEVPKEY_PciDevice_BarTypes, 16}
DEFINE_PCI_DEVICE_DEVPKEY!{DEVPKEY_PciDevice_AERCapabilityPresent, 17}
DEFINE_PCI_DEVICE_DEVPKEY!{DEVPKEY_PciDevice_FirmwareErrorHandling, 18}
DEFINE_PCI_DEVICE_DEVPKEY!{DEVPKEY_PciDevice_Uncorrectable_Error_Mask, 19}
DEFINE_PCI_DEVICE_DEVPKEY!{DEVPKEY_PciDevice_Uncorrectable_Error_Severity, 20}
DEFINE_PCI_DEVICE_DEVPKEY!{DEVPKEY_PciDevice_Correctable_Error_Mask, 21}
DEFINE_PCI_DEVICE_DEVPKEY!{DEVPKEY_PciDevice_ECRC_Errors, 22}
DEFINE_PCI_DEVICE_DEVPKEY!{DEVPKEY_PciDevice_Error_Reporting, 23}
DEFINE_PCI_DEVICE_DEVPKEY!{DEVPKEY_PciDevice_RootError_Reporting, 24}
DEFINE_PCI_DEVICE_DEVPKEY!{DEVPKEY_PciDevice_S0WakeupSupported, 25}
pub const DevProp_PciDevice_SriovSupport_Ok: ULONG = 0x0;
pub const DevProp_PciDevice_SriovSupport_MissingAcs: ULONG = 0x1;
pub const DevProp_PciDevice_SriovSupport_MissingPfDriver: ULONG = 0x2;
pub const DevProp_PciDevice_SriovSupport_NoBusResource: ULONG = 0x3;
pub const DevProp_PciDevice_SriovSupport_DidntGetVfBarSpace: ULONG = 0x4;
DEFINE_PCI_DEVICE_DEVPKEY!{DEVPKEY_PciDevice_SriovSupport, 26}
DEFINE_PCI_DEVICE_DEVPKEY!{DEVPKEY_PciDevice_Label_Id, 27}
DEFINE_PCI_DEVICE_DEVPKEY!{DEVPKEY_PciDevice_Label_String, 28}
pub const DevProp_PciDevice_AcsSupport_Present: ULONG = 0x0;
pub const DevProp_PciDevice_AcsSupport_NotNeeded: ULONG = 0x1;
pub const DevProp_PciDevice_AcsSupport_Missing: ULONG = 0x2;
DEFINE_PCI_DEVICE_DEVPKEY!{DEVPKEY_PciDevice_AcsSupport, 29}
DEFINE_PCI_DEVICE_DEVPKEY!{DEVPKEY_PciDevice_AriSupport, 30}
pub const DevProp_PciDevice_AcsCompatibleUpHierarchy_NotSupported: ULONG = 0x0;
pub const DevProp_PciDevice_AcsCompatibleUpHierarchy_SingleFunctionSupported: ULONG = 0x1;
pub const DevProp_PciDevice_AcsCompatibleUpHierarchy_NoP2PSupported: ULONG = 0x2;
pub const DevProp_PciDevice_AcsCompatibleUpHierarchy_Supported: ULONG = 0x3;
DEFINE_PCI_DEVICE_DEVPKEY!{DEVPKEY_PciDevice_AcsCompatibleUpHierarchy, 31}
DEFINE_PCI_DEVICE_DEVPKEY!{DEVPKEY_PciDevice_AcsCapabilityRegister, 32}
DEFINE_PCI_DEVICE_DEVPKEY!{DEVPKEY_PciDevice_AtsSupport, 33}
DEFINE_PCI_DEVICE_DEVPKEY!{DEVPKEY_PciDevice_RequiresReservedMemoryRegion, 34}
DEFINE_PCI_DEVICE_DEVPKEY!{DEVPKEY_PciDevice_AtomicsSupported, 35}
DEFINE_PCI_DEVICE_DEVPKEY!{DEVPKEY_PciDevice_SupportedLinkSubState, 36}

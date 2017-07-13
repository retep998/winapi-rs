// Copyright © 2016-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! USB Definitions.
use shared::minwindef::{UCHAR, ULONG, USHORT};
use shared::usbspec::{
    PUSB_CONFIGURATION_DESCRIPTOR, USB_DEVICE_DESCRIPTOR, USB_ENDPOINT_DIRECTION_MASK,
};
use um::winnt::{LONG, PVOID, WCHAR};
pub type PRIP = PVOID;
pub type PMDL = PVOID;
pub const USBDI_VERSION: ULONG = 0x00000600;
pub const USB_PORTATTR_NO_CONNECTOR: ULONG = 0x00000001;
pub const USB_PORTATTR_SHARED_USB2: ULONG = 0x00000002;
pub const USB_PORTATTR_MINI_CONNECTOR: ULONG = 0x00000004;
pub const USB_PORTATTR_OEM_CONNECTOR: ULONG = 0x00000008;
pub const USB_PORTATTR_OWNED_BY_CC: ULONG = 0x01000000;
pub const USB_PORTATTR_NO_OVERCURRENT_UI: ULONG = 0x02000000;
ENUM!{enum USB_CONTROLLER_FLAVOR {
    USB_HcGeneric = 0,
    OHCI_Generic = 100,
    OHCI_Hydra,
    OHCI_NEC,
    UHCI_Generic = 200,
    UHCI_Piix4 = 201,
    UHCI_Piix3 = 202,
    UHCI_Ich2 = 203,
    UHCI_Reserved204 = 204,
    UHCI_Ich1 = 205,
    UHCI_Ich3m = 206,
    UHCI_Ich4 = 207,
    UHCI_Ich5 = 208,
    UHCI_Ich6 = 209,
    UHCI_Intel = 249,
    UHCI_VIA = 250,
    UHCI_VIA_x01 = 251,
    UHCI_VIA_x02 = 252,
    UHCI_VIA_x03 = 253,
    UHCI_VIA_x04 = 254,
    UHCI_VIA_x0E_FIFO = 264,
    EHCI_Generic = 1000,
    EHCI_NEC = 2000,
    EHCI_Lucent = 3000,
    EHCI_NVIDIA_Tegra2 = 4000,
    EHCI_NVIDIA_Tegra3 = 4001,
    EHCI_Intel_Medfield = 5001,
}}
pub const USB_DEFAULT_DEVICE_ADDRESS: UCHAR = 0;
pub const USB_DEFAULT_ENDPOINT_ADDRESS: UCHAR = 0;
pub const USB_DEFAULT_MAX_PACKET: USHORT = 64;
pub const URB_FUNCTION_SELECT_CONFIGURATION: USHORT = 0x0000;
pub const URB_FUNCTION_SELECT_INTERFACE: USHORT = 0x0001;
pub const URB_FUNCTION_ABORT_PIPE: USHORT = 0x0002;
pub const URB_FUNCTION_TAKE_FRAME_LENGTH_CONTROL: USHORT = 0x0003;
pub const URB_FUNCTION_RELEASE_FRAME_LENGTH_CONTROL: USHORT = 0x0004;
pub const URB_FUNCTION_GET_FRAME_LENGTH: USHORT = 0x0005;
pub const URB_FUNCTION_SET_FRAME_LENGTH: USHORT = 0x0006;
pub const URB_FUNCTION_GET_CURRENT_FRAME_NUMBER: USHORT = 0x0007;
pub const URB_FUNCTION_CONTROL_TRANSFER: USHORT = 0x0008;
pub const URB_FUNCTION_BULK_OR_INTERRUPT_TRANSFER: USHORT = 0x0009;
pub const URB_FUNCTION_ISOCH_TRANSFER: USHORT = 0x000A;
pub const URB_FUNCTION_GET_DESCRIPTOR_FROM_DEVICE: USHORT = 0x000B;
pub const URB_FUNCTION_SET_DESCRIPTOR_TO_DEVICE: USHORT = 0x000C;
pub const URB_FUNCTION_SET_FEATURE_TO_DEVICE: USHORT = 0x000D;
pub const URB_FUNCTION_SET_FEATURE_TO_INTERFACE: USHORT = 0x000E;
pub const URB_FUNCTION_SET_FEATURE_TO_ENDPOINT: USHORT = 0x000F;
pub const URB_FUNCTION_CLEAR_FEATURE_TO_DEVICE: USHORT = 0x0010;
pub const URB_FUNCTION_CLEAR_FEATURE_TO_INTERFACE: USHORT = 0x0011;
pub const URB_FUNCTION_CLEAR_FEATURE_TO_ENDPOINT: USHORT = 0x0012;
pub const URB_FUNCTION_GET_STATUS_FROM_DEVICE: USHORT = 0x0013;
pub const URB_FUNCTION_GET_STATUS_FROM_INTERFACE: USHORT = 0x0014;
pub const URB_FUNCTION_GET_STATUS_FROM_ENDPOINT: USHORT = 0x0015;
pub const URB_FUNCTION_RESERVED_0X0016: USHORT = 0x0016;
pub const URB_FUNCTION_VENDOR_DEVICE: USHORT = 0x0017;
pub const URB_FUNCTION_VENDOR_INTERFACE: USHORT = 0x0018;
pub const URB_FUNCTION_VENDOR_ENDPOINT: USHORT = 0x0019;
pub const URB_FUNCTION_CLASS_DEVICE: USHORT = 0x001A;
pub const URB_FUNCTION_CLASS_INTERFACE: USHORT = 0x001B;
pub const URB_FUNCTION_CLASS_ENDPOINT: USHORT = 0x001C;
pub const URB_FUNCTION_RESERVE_0X001D: USHORT = 0x001D;
pub const URB_FUNCTION_SYNC_RESET_PIPE_AND_CLEAR_STALL: USHORT = 0x001E;
pub const URB_FUNCTION_CLASS_OTHER: USHORT = 0x001F;
pub const URB_FUNCTION_VENDOR_OTHER: USHORT = 0x0020;
pub const URB_FUNCTION_GET_STATUS_FROM_OTHER: USHORT = 0x0021;
pub const URB_FUNCTION_CLEAR_FEATURE_TO_OTHER: USHORT = 0x0022;
pub const URB_FUNCTION_SET_FEATURE_TO_OTHER: USHORT = 0x0023;
pub const URB_FUNCTION_GET_DESCRIPTOR_FROM_ENDPOINT: USHORT = 0x0024;
pub const URB_FUNCTION_SET_DESCRIPTOR_TO_ENDPOINT: USHORT = 0x0025;
pub const URB_FUNCTION_GET_CONFIGURATION: USHORT = 0x0026;
pub const URB_FUNCTION_GET_INTERFACE: USHORT = 0x0027;
pub const URB_FUNCTION_GET_DESCRIPTOR_FROM_INTERFACE: USHORT = 0x0028;
pub const URB_FUNCTION_SET_DESCRIPTOR_TO_INTERFACE: USHORT = 0x0029;
pub const URB_FUNCTION_GET_MS_FEATURE_DESCRIPTOR: USHORT = 0x002A;
pub const URB_FUNCTION_SYNC_RESET_PIPE: USHORT = 0x0030;
pub const URB_FUNCTION_SYNC_CLEAR_STALL: USHORT = 0x0031;
pub const URB_FUNCTION_CONTROL_TRANSFER_EX: USHORT = 0x0032;
pub const URB_FUNCTION_RESERVE_0X0033: USHORT = 0x0033;
pub const URB_FUNCTION_RESERVE_0X0034: USHORT = 0x0034;
pub const URB_FUNCTION_OPEN_STATIC_STREAMS: USHORT = 0x0035;
pub const URB_FUNCTION_CLOSE_STATIC_STREAMS: USHORT = 0x0036;
pub const URB_FUNCTION_BULK_OR_INTERRUPT_TRANSFER_USING_CHAINED_MDL: USHORT = 0x0037;
pub const URB_FUNCTION_ISOCH_TRANSFER_USING_CHAINED_MDL: USHORT = 0x0038;
pub const URB_FUNCTION_RESERVE_0X002B: USHORT = 0x002B;
pub const URB_FUNCTION_RESERVE_0X002C: USHORT = 0x002C;
pub const URB_FUNCTION_RESERVE_0X002D: USHORT = 0x002D;
pub const URB_FUNCTION_RESERVE_0X002E: USHORT = 0x002E;
pub const URB_FUNCTION_RESERVE_0X002F: USHORT = 0x002F;
pub const URB_FUNCTION_RESET_PIPE: USHORT = URB_FUNCTION_SYNC_RESET_PIPE_AND_CLEAR_STALL;
pub const USBD_SHORT_TRANSFER_OK: ULONG = 0x00000002;
pub const USBD_START_ISO_TRANSFER_ASAP: ULONG = 0x00000004;
pub const USBD_DEFAULT_PIPE_TRANSFER: ULONG = 0x00000008;
pub const USBD_TRANSFER_DIRECTION_OUT: ULONG = 0;
pub const USBD_TRANSFER_DIRECTION_IN: ULONG = 1;
pub const USBD_TRANSFER_DIRECTION: ULONG = USBD_TRANSFER_DIRECTION_IN;
#[inline]
pub fn USBD_TRANSFER_DIRECTION_FLAG(flags: ULONG) -> ULONG {
    flags & USBD_TRANSFER_DIRECTION
}
pub const VALID_TRANSFER_FLAGS_MASK: ULONG = USBD_SHORT_TRANSFER_OK | USBD_TRANSFER_DIRECTION
    | USBD_START_ISO_TRANSFER_ASAP | USBD_DEFAULT_PIPE_TRANSFER;
pub const USBD_ISO_START_FRAME_RANGE: ULONG = 1024;
pub type USBD_STATUS = LONG;
#[inline]
pub fn USBD_SUCCESS(Status: USBD_STATUS) -> bool {
    Status >= 0
}
#[inline]
pub fn USBD_PENDING(Status: ULONG) -> bool {
    (Status >> 30) == 1
}
pub const USBD_STATUS_SUCCESS: USBD_STATUS = 0x00000000;
pub const USBD_STATUS_PORT_OPERATION_PENDING: USBD_STATUS = 0x00000001;
pub const USBD_STATUS_PENDING: USBD_STATUS = 0x40000000;
pub const USBD_STATUS_CRC: USBD_STATUS = 0xC0000001u32 as USBD_STATUS;
pub const USBD_STATUS_BTSTUFF: USBD_STATUS = 0xC0000002u32 as USBD_STATUS;
pub const USBD_STATUS_DATA_TOGGLE_MISMATCH: USBD_STATUS = 0xC0000003u32 as USBD_STATUS;
pub const USBD_STATUS_STALL_PID: USBD_STATUS = 0xC0000004u32 as USBD_STATUS;
pub const USBD_STATUS_DEV_NOT_RESPONDING: USBD_STATUS = 0xC0000005u32 as USBD_STATUS;
pub const USBD_STATUS_PID_CHECK_FAILURE: USBD_STATUS = 0xC0000006u32 as USBD_STATUS;
pub const USBD_STATUS_UNEXPECTED_PID: USBD_STATUS = 0xC0000007u32 as USBD_STATUS;
pub const USBD_STATUS_DATA_OVERRUN: USBD_STATUS = 0xC0000008u32 as USBD_STATUS;
pub const USBD_STATUS_DATA_UNDERRUN: USBD_STATUS = 0xC0000009u32 as USBD_STATUS;
pub const USBD_STATUS_RESERVED1: USBD_STATUS = 0xC000000Au32 as USBD_STATUS;
pub const USBD_STATUS_RESERVED2: USBD_STATUS = 0xC000000Bu32 as USBD_STATUS;
pub const USBD_STATUS_BUFFER_OVERRUN: USBD_STATUS = 0xC000000Cu32 as USBD_STATUS;
pub const USBD_STATUS_BUFFER_UNDERRUN: USBD_STATUS = 0xC000000Du32 as USBD_STATUS;
pub const USBD_STATUS_NOT_ACCESSED: USBD_STATUS = 0xC000000Fu32 as USBD_STATUS;
pub const USBD_STATUS_FIFO: USBD_STATUS = 0xC0000010u32 as USBD_STATUS;
pub const USBD_STATUS_XACT_ERROR: USBD_STATUS = 0xC0000011u32 as USBD_STATUS;
pub const USBD_STATUS_BABBLE_DETECTED: USBD_STATUS = 0xC0000012u32 as USBD_STATUS;
pub const USBD_STATUS_DATA_BUFFER_ERROR: USBD_STATUS = 0xC0000013u32 as USBD_STATUS;
pub const USBD_STATUS_NO_PING_RESPONSE: USBD_STATUS = 0xC0000014u32 as USBD_STATUS;
pub const USBD_STATUS_INVALID_STREAM_TYPE: USBD_STATUS = 0xC0000015u32 as USBD_STATUS;
pub const USBD_STATUS_INVALID_STREAM_ID: USBD_STATUS = 0xC0000016u32 as USBD_STATUS;
pub const USBD_STATUS_ENDPOINT_HALTED: USBD_STATUS = 0xC0000030u32 as USBD_STATUS;
pub const USBD_STATUS_INVALID_URB_FUNCTION: USBD_STATUS = 0x80000200u32 as USBD_STATUS;
pub const USBD_STATUS_INVALID_PARAMETER: USBD_STATUS = 0x80000300u32 as USBD_STATUS;
pub const USBD_STATUS_ERROR_BUSY: USBD_STATUS = 0x80000400u32 as USBD_STATUS;
pub const USBD_STATUS_INVALID_PIPE_HANDLE: USBD_STATUS = 0x80000600u32 as USBD_STATUS;
pub const USBD_STATUS_NO_BANDWIDTH: USBD_STATUS = 0x80000700u32 as USBD_STATUS;
pub const USBD_STATUS_INTERNAL_HC_ERROR: USBD_STATUS = 0x80000800u32 as USBD_STATUS;
pub const USBD_STATUS_ERROR_SHORT_TRANSFER: USBD_STATUS = 0x80000900u32 as USBD_STATUS;
pub const USBD_STATUS_BAD_START_FRAME: USBD_STATUS = 0xC0000A00u32 as USBD_STATUS;
pub const USBD_STATUS_ISOCH_REQUEST_FAILED: USBD_STATUS = 0xC0000B00u32 as USBD_STATUS;
pub const USBD_STATUS_FRAME_CONTROL_OWNED: USBD_STATUS = 0xC0000C00u32 as USBD_STATUS;
pub const USBD_STATUS_FRAME_CONTROL_NOT_OWNED: USBD_STATUS = 0xC0000D00u32 as USBD_STATUS;
pub const USBD_STATUS_NOT_SUPPORTED: USBD_STATUS = 0xC0000E00u32 as USBD_STATUS;
pub const USBD_STATUS_INAVLID_CONFIGURATION_DESCRIPTOR: USBD_STATUS = 0xC0000F00u32 as USBD_STATUS;
pub const USBD_STATUS_INVALID_CONFIGURATION_DESCRIPTOR: USBD_STATUS = 0xC0000F00u32 as USBD_STATUS;
pub const USBD_STATUS_INSUFFICIENT_RESOURCES: USBD_STATUS = 0xC0001000u32 as USBD_STATUS;
pub const USBD_STATUS_SET_CONFIG_FAILED: USBD_STATUS = 0xC0002000u32 as USBD_STATUS;
pub const USBD_STATUS_BUFFER_TOO_SMALL: USBD_STATUS = 0xC0003000u32 as USBD_STATUS;
pub const USBD_STATUS_INTERFACE_NOT_FOUND: USBD_STATUS = 0xC0004000u32 as USBD_STATUS;
pub const USBD_STATUS_INAVLID_PIPE_FLAGS: USBD_STATUS = 0xC0005000u32 as USBD_STATUS;
pub const USBD_STATUS_TIMEOUT: USBD_STATUS = 0xC0006000u32 as USBD_STATUS;
pub const USBD_STATUS_DEVICE_GONE: USBD_STATUS = 0xC0007000u32 as USBD_STATUS;
pub const USBD_STATUS_STATUS_NOT_MAPPED: USBD_STATUS = 0xC0008000u32 as USBD_STATUS;
pub const USBD_STATUS_HUB_INTERNAL_ERROR: USBD_STATUS = 0xC0009000u32 as USBD_STATUS;
pub const USBD_STATUS_CANCELED: USBD_STATUS = 0xC0010000u32 as USBD_STATUS;
pub const USBD_STATUS_ISO_NOT_ACCESSED_BY_HW: USBD_STATUS = 0xC0020000u32 as USBD_STATUS;
pub const USBD_STATUS_ISO_TD_ERROR: USBD_STATUS = 0xC0030000u32 as USBD_STATUS;
pub const USBD_STATUS_ISO_NA_LATE_USBPORT: USBD_STATUS = 0xC0040000u32 as USBD_STATUS;
pub const USBD_STATUS_ISO_NOT_ACCESSED_LATE: USBD_STATUS = 0xC0050000u32 as USBD_STATUS;
pub const USBD_STATUS_BAD_DESCRIPTOR: USBD_STATUS = 0xC0100000u32 as USBD_STATUS;
pub const USBD_STATUS_BAD_DESCRIPTOR_BLEN: USBD_STATUS = 0xC0100001u32 as USBD_STATUS;
pub const USBD_STATUS_BAD_DESCRIPTOR_TYPE: USBD_STATUS = 0xC0100002u32 as USBD_STATUS;
pub const USBD_STATUS_BAD_INTERFACE_DESCRIPTOR: USBD_STATUS = 0xC0100003u32 as USBD_STATUS;
pub const USBD_STATUS_BAD_ENDPOINT_DESCRIPTOR: USBD_STATUS = 0xC0100004u32 as USBD_STATUS;
pub const USBD_STATUS_BAD_INTERFACE_ASSOC_DESCRIPTOR: USBD_STATUS = 0xC0100005u32 as USBD_STATUS;
pub const USBD_STATUS_BAD_CONFIG_DESC_LENGTH: USBD_STATUS = 0xC0100006u32 as USBD_STATUS;
pub const USBD_STATUS_BAD_NUMBER_OF_INTERFACES: USBD_STATUS = 0xC0100007u32 as USBD_STATUS;
pub const USBD_STATUS_BAD_NUMBER_OF_ENDPOINTS: USBD_STATUS = 0xC0100008u32 as USBD_STATUS;
pub const USBD_STATUS_BAD_ENDPOINT_ADDRESS: USBD_STATUS = 0xC0100009u32 as USBD_STATUS;
pub type USBD_PIPE_HANDLE = PVOID;
pub type USBD_CONFIGURATION_HANDLE = PVOID;
pub type USBD_INTERFACE_HANDLE = PVOID;
pub const USBD_DEFAULT_MAXIMUM_TRANSFER_SIZE: ULONG = 0xFFFFFFFF;
STRUCT!{struct USBD_VERSION_INFORMATION {
    USBDI_Version: ULONG,
    Supported_USB_Version: ULONG,
}}
pub type PUSBD_VERSION_INFORMATION = *mut USBD_VERSION_INFORMATION;
ENUM!{enum USBD_PIPE_TYPE {
    UsbdPipeTypeControl,
    UsbdPipeTypeIsochronous,
    UsbdPipeTypeBulk,
    UsbdPipeTypeInterrupt,
}}
#[inline]
pub fn USBD_PIPE_DIRECTION_IN(pipeInformation: &USBD_PIPE_INFORMATION) -> UCHAR {
    (*pipeInformation).EndpointAddress & USB_ENDPOINT_DIRECTION_MASK
}
STRUCT!{struct USBD_DEVICE_INFORMATION {
    OffsetNext: ULONG,
    UsbdDeviceHandle: PVOID,
    DeviceDescriptor: USB_DEVICE_DESCRIPTOR,
}}
pub type PUSBD_DEVICE_INFORMATION = *mut USBD_DEVICE_INFORMATION;
STRUCT!{struct USBD_PIPE_INFORMATION {
    MaximumPacketSize: USHORT,
    EndpointAddress: UCHAR,
    Interval: UCHAR,
    PipeType: USBD_PIPE_TYPE,
    PipeHandle: USBD_PIPE_HANDLE,
    MaximumTransferSize: ULONG,
    PipeFlags: ULONG,
}}
pub type PUSBD_PIPE_INFORMATION = *mut USBD_PIPE_INFORMATION;
pub const USBD_PF_CHANGE_MAX_PACKET: ULONG = 0x00000001;
pub const USBD_PF_SHORT_PACKET_OPT: ULONG = 0x00000002;
pub const USBD_PF_ENABLE_RT_THREAD_ACCESS: ULONG = 0x00000004;
pub const USBD_PF_MAP_ADD_TRANSFERS: ULONG = 0x00000008;
pub const USBD_PF_VALID_MASK: ULONG = USBD_PF_CHANGE_MAX_PACKET | USBD_PF_SHORT_PACKET_OPT
    | USBD_PF_ENABLE_RT_THREAD_ACCESS | USBD_PF_MAP_ADD_TRANSFERS;
STRUCT!{struct USBD_INTERFACE_INFORMATION {
    Length: USHORT,
    InterfaceNumber: UCHAR,
    AlternateSetting: UCHAR,
    Class: UCHAR,
    SubClass: UCHAR,
    Protocol: UCHAR,
    Reserved: UCHAR,
    InterfaceHandle: USBD_INTERFACE_HANDLE,
    NumberOfPipes: ULONG,
    Pipes: [USBD_PIPE_INFORMATION; 1],
}}
pub type PUSBD_INTERFACE_INFORMATION = *mut USBD_INTERFACE_INFORMATION;
STRUCT!{struct URB_HCD_AREA {
    Reserved8: [PVOID; 8],
}}
STRUCT!{struct URB_HEADER {
    Length: USHORT,
    Function: USHORT,
    Status: USBD_STATUS,
    UsbdDeviceHandle: PVOID,
    UsbdFlags: ULONG,
}}
STRUCT!{struct URB_SELECT_INTERFACE {
    Hdr: URB_HEADER,
    ConfigurationHandle: USBD_CONFIGURATION_HANDLE,
    Interface: USBD_INTERFACE_INFORMATION,
}}
STRUCT!{struct URB_SELECT_CONFIGURATION {
    Hdr: URB_HEADER,
    ConfigurationDescriptor: PUSB_CONFIGURATION_DESCRIPTOR,
    ConfigurationHandle: USBD_CONFIGURATION_HANDLE,
    Interface: USBD_INTERFACE_INFORMATION,
}}
STRUCT!{struct URB_PIPE_REQUEST {
    Hdr: URB_HEADER,
    PipeHandle: USBD_PIPE_HANDLE,
    Reserved: ULONG,
}}
STRUCT!{struct URB_FRAME_LENGTH_CONTROL {
    Hdr: URB_HEADER,
}}
STRUCT!{struct URB_GET_FRAME_LENGTH {
    Hdr: URB_HEADER,
    FrameLength: ULONG,
    FrameNumber: ULONG,
}}
STRUCT!{struct URB_SET_FRAME_LENGTH {
    Hdr: URB_HEADER,
    FrameLengthDelta: LONG,
}}
STRUCT!{struct URB_GET_CURRENT_FRAME_NUMBER {
    Hdr: URB_HEADER,
    FrameNumber: ULONG,
}}
STRUCT!{struct URB_CONTROL_DESCRIPTOR_REQUEST {
    Hdr: URB_HEADER,
    Reserved: PVOID,
    Reserved0: ULONG,
    TransferBufferLength: ULONG,
    TransferBuffer: PVOID,
    TransferBufferMDL: PMDL,
    UrbLink: *mut URB,
    hca: URB_HCD_AREA,
    Reserved1: USHORT,
    Index: UCHAR,
    DescriptorType: UCHAR,
    LanguageId: USHORT,
    Reserved2: USHORT,
}}
STRUCT!{struct URB_CONTROL_GET_STATUS_REQUEST {
    Hdr: URB_HEADER,
    Reserved: PVOID,
    Reserved0: ULONG,
    TransferBufferLength: ULONG,
    TransferBuffer: PVOID,
    TransferBufferMDL: PMDL,
    UrbLink: *mut URB,
    hca: URB_HCD_AREA,
    Reserved1: [UCHAR; 4],
    Index: USHORT,
    Reserved2: USHORT,
}}
STRUCT!{struct URB_CONTROL_FEATURE_REQUEST {
    Hdr: URB_HEADER,
    Reserved: PVOID,
    Reserved2: ULONG,
    Reserved3: ULONG,
    Reserved4: PVOID,
    Reserved5: PMDL,
    UrbLink: *mut URB,
    hca: URB_HCD_AREA,
    Reserved0: USHORT,
    FeatureSelector: USHORT,
    Index: USHORT,
    Reserved1: USHORT,
}}
STRUCT!{struct URB_CONTROL_VENDOR_OR_CLASS_REQUEST {
    Hdr: URB_HEADER,
    Reserved: PVOID,
    TransferFlags: ULONG,
    TransferBufferLength: ULONG,
    TransferBuffer: PVOID,
    TransferBufferMDL: PMDL,
    UrbLink: *mut URB,
    hca: URB_HCD_AREA,
    RequestTypeReservedBits: UCHAR,
    Request: UCHAR,
    Value: USHORT,
    Index: USHORT,
    Reserved1: USHORT,
}}
STRUCT!{struct URB_CONTROL_GET_INTERFACE_REQUEST {
    Hdr: URB_HEADER,
    Reserved: PVOID,
    Reserved0: ULONG,
    TransferBufferLength: ULONG,
    TransferBuffer: PVOID,
    TransferBufferMDL: PMDL,
    UrbLink: *mut URB,
    hca: URB_HCD_AREA,
    Reserved1: [UCHAR; 4],
    Interface: USHORT,
    Reserved2: USHORT,
}}
STRUCT!{struct URB_CONTROL_GET_CONFIGURATION_REQUEST {
    Hdr: URB_HEADER,
    Reserved: PVOID,
    Reserved0: ULONG,
    TransferBufferLength: ULONG,
    TransferBuffer: PVOID,
    TransferBufferMDL: PMDL,
    UrbLink: *mut URB,
    hca: URB_HCD_AREA,
    Reserved1: [UCHAR; 8],
}}
pub const OS_STRING_DESCRIPTOR_INDEX: UCHAR = 0xEE;
pub const MS_GENRE_DESCRIPTOR_INDEX: USHORT = 0x0001;
pub const MS_POWER_DESCRIPTOR_INDEX: USHORT = 0x0002;
pub const MS_OS_STRING_SIGNATURE: &'static str = "MSFT100";
pub const MS_OS_FLAGS_CONTAINERID: UCHAR = 0x02;
UNION2!{union OS_STRING_u {
    [u8; 1],
    bPad bPad_mut: UCHAR,
    bFlags bFlags_mut: UCHAR,
}}
STRUCT!{struct OS_STRING {
    bLength: UCHAR,
    bDescriptorType: UCHAR,
    MicrosoftString: [WCHAR; 7],
    bVendorCode: UCHAR,
    u: OS_STRING_u,
}}
pub type POS_STRING = *mut OS_STRING;
STRUCT!{struct URB_OS_FEATURE_DESCRIPTOR_REQUEST {
    Hdr: URB_HEADER,
    Reserved: PVOID,
    Reserved0: ULONG,
    TransferBufferLength: ULONG,
    TransferBuffer: PVOID,
    TransferBufferMDL: PMDL,
    UrbLink: *mut URB,
    hca: URB_HCD_AREA,
    BitField: UCHAR,
    Reserved2: UCHAR,
    InterfaceNumber: UCHAR,
    MS_PageIndex: UCHAR,
    MS_FeatureDescriptorIndex: USHORT,
    Reserved3: USHORT,
}}
BITFIELD!{URB_OS_FEATURE_DESCRIPTOR_REQUEST BitField: UCHAR [
    Recipient set_Recipient[0..5],
    Reserved1 set_Reserved1[5..8],
]}
STRUCT!{struct URB_CONTROL_TRANSFER {
    Hdr: URB_HEADER,
    PipeHandle: USBD_PIPE_HANDLE,
    TransferFlags: ULONG,
    TransferBufferLength: ULONG,
    TransferBuffer: PVOID,
    TransferBufferMDL: PMDL,
    UrbLink: *mut URB,
    hca: URB_HCD_AREA,
    SetupPacket: [UCHAR; 8],
}}
#[cfg(target_arch = "x86_64")]
STRUCT!{struct URB_CONTROL_TRANSFER_EX {
    Hdr: URB_HEADER,
    PipeHandle: USBD_PIPE_HANDLE,
    TransferFlags: ULONG,
    TransferBufferLength: ULONG,
    TransferBuffer: PVOID,
    TransferBufferMDL: PMDL,
    Timeout: ULONG,
    Pad: ULONG,
    hca: URB_HCD_AREA,
    SetupPacket: [UCHAR; 8],
}}
#[cfg(target_arch = "x86")]
STRUCT!{struct URB_CONTROL_TRANSFER_EX {
    Hdr: URB_HEADER,
    PipeHandle: USBD_PIPE_HANDLE,
    TransferFlags: ULONG,
    TransferBufferLength: ULONG,
    TransferBuffer: PVOID,
    TransferBufferMDL: PMDL,
    Timeout: ULONG,
    hca: URB_HCD_AREA,
    SetupPacket: [UCHAR; 8],
}}
STRUCT!{struct URB_BULK_OR_INTERRUPT_TRANSFER {
    Hdr: URB_HEADER,
    PipeHandle: USBD_PIPE_HANDLE,
    TransferFlags: ULONG,
    TransferBufferLength: ULONG,
    TransferBuffer: PVOID,
    TransferBufferMDL: PMDL,
    UrbLink: *mut URB,
    hca: URB_HCD_AREA,
}}
STRUCT!{struct USBD_ISO_PACKET_DESCRIPTOR {
    Offset: ULONG,
    Length: ULONG,
    Status: USBD_STATUS,
}}
pub type PUSBD_ISO_PACKET_DESCRIPTOR = *mut USBD_ISO_PACKET_DESCRIPTOR;
STRUCT!{struct URB_ISOCH_TRANSFER {
    Hdr: URB_HEADER,
    PipeHandle: USBD_PIPE_HANDLE,
    TransferFlags: ULONG,
    TransferBufferLength: ULONG,
    TransferBuffer: PVOID,
    TransferBufferMDL: PMDL,
    UrbLink: *mut URB,
    hca: URB_HCD_AREA,
    StartFrame: ULONG,
    NumberOfPackets: ULONG,
    ErrorCount: ULONG,
    IsoPacket: [USBD_ISO_PACKET_DESCRIPTOR; 1],
}}
pub const URB_OPEN_STATIC_STREAMS_VERSION_100: USHORT = 0x100;
STRUCT!{struct USBD_STREAM_INFORMATION {
    PipeHandle: USBD_PIPE_HANDLE,
    StreamID: ULONG,
    MaximumTransferSize: ULONG,
    PipeFlags: ULONG,
}}
pub type PUSBD_STREAM_INFORMATION = *mut USBD_STREAM_INFORMATION;
STRUCT!{struct URB_OPEN_STATIC_STREAMS {
    Hdr: URB_HEADER,
    PipeHandle: USBD_PIPE_HANDLE,
    NumberOfStreams: ULONG,
    StreamInfoVersion: USHORT,
    StreamInfoSize: USHORT,
    Streams: PUSBD_STREAM_INFORMATION,
}}
UNION2!{union URB_u {
    [u8; 96] [u8; 152],
    UrbHeader UrbHeader_mut: URB_HEADER,
    UrbSelectInterface UrbSelectInterface_mut: URB_SELECT_INTERFACE,
    UrbSelectConfiguration UrbSelectConfiguration_mut: URB_SELECT_CONFIGURATION,
    UrbPipeRequest UrbPipeRequest_mut: URB_PIPE_REQUEST,
    UrbFrameLengthControl UrbFrameLengthControl_mut: URB_FRAME_LENGTH_CONTROL,
    UrbGetFrameLength UrbGetFrameLength_mut: URB_GET_FRAME_LENGTH,
    UrbSetFrameLength UrbSetFrameLength_mut: URB_SET_FRAME_LENGTH,
    UrbGetCurrentFrameNumber UrbGetCurrentFrameNumber_mut: URB_GET_CURRENT_FRAME_NUMBER,
    UrbControlTransfer UrbControlTransfer_mut: URB_CONTROL_TRANSFER,
    UrbControlTransferEx UrbControlTransferEx_mut: URB_CONTROL_TRANSFER_EX,
    UrbBulkOrInterruptTransfer UrbBulkOrInterruptTransfer_mut: URB_BULK_OR_INTERRUPT_TRANSFER,
    UrbIsochronousTransfer UrbIsochronousTransfer_mut: URB_ISOCH_TRANSFER,
    UrbControlDescriptorRequest UrbControlDescriptorRequest_mut: URB_CONTROL_DESCRIPTOR_REQUEST,
    UrbControlGetStatusRequest UrbControlGetStatusRequest_mut: URB_CONTROL_GET_STATUS_REQUEST,
    UrbControlFeatureRequest UrbControlFeatureRequest_mut: URB_CONTROL_FEATURE_REQUEST,
    UrbControlVendorClassRequest UrbControlVendorClassRequest_mut
        : URB_CONTROL_VENDOR_OR_CLASS_REQUEST,
    UrbControlGetInterfaceRequest UrbControlGetInterfaceRequest_mut
        : URB_CONTROL_GET_INTERFACE_REQUEST,
    UrbControlGetConfigurationRequest UrbControlGetConfigurationRequest_mut
        : URB_CONTROL_GET_CONFIGURATION_REQUEST,
    UrbOSFeatureDescriptorRequest UrbOSFeatureDescriptorRequest_mut
        : URB_OS_FEATURE_DESCRIPTOR_REQUEST,
    UrbOpenStaticStreams UrbOpenStaticStreams_mut: URB_OPEN_STATIC_STREAMS,
}}
STRUCT!{struct URB {
    u: URB_u,
}}
pub type PURB = *mut URB;

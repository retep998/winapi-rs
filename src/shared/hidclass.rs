// Copyright © 2015-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::guiddef::{GUID};
use shared::minwindef::{ULONG, USHORT, UCHAR, PUCHAR, DWORD};
use um::winioctl::{
    METHOD_IN_DIRECT, METHOD_OUT_DIRECT, METHOD_NEITHER, METHOD_BUFFERED, FILE_ANY_ACCESS,
    FILE_DEVICE_KEYBOARD
};
use um::winnt::{BOOLEAN};

DEFINE_GUID!{GUID_DEVINTERFACE_HID,
    0x4D1E55B2, 0xF16F, 0x11CF, 0x88, 0xCB, 0x00, 0x11, 0x11, 0x00, 0x00, 0x30}
pub const GUID_CLASS_INPUT: GUID = GUID_DEVINTERFACE_HID;
DEFINE_GUID!{GUID_HID_INTERFACE_NOTIFY,
    0x2c4e2e88, 0x25e6, 0x4c33, 0x88, 0x2f, 0x3d, 0x82, 0xe6, 0x07, 0x36, 0x81}
DEFINE_GUID!{GUID_HID_INTERFACE_HIDPARSE,
    0xf5c315a5, 0x69ac, 0x4bc2, 0x92, 0x79, 0xd0, 0xb6, 0x45, 0x76, 0xf4, 0x4b}
// FIXME devpropkey stuff
pub const HID_REVISION: DWORD = 0x00000001;
pub const IOCTL_HID_GET_DRIVER_CONFIG: DWORD = HID_BUFFER_CTL_CODE!(100);
pub const IOCTL_HID_SET_DRIVER_CONFIG: DWORD = HID_BUFFER_CTL_CODE!(101);
pub const IOCTL_HID_GET_POLL_FREQUENCY_MSEC: DWORD = HID_BUFFER_CTL_CODE!(102);
pub const IOCTL_HID_SET_POLL_FREQUENCY_MSEC: DWORD = HID_BUFFER_CTL_CODE!(103);
pub const IOCTL_GET_NUM_DEVICE_INPUT_BUFFERS: DWORD = HID_BUFFER_CTL_CODE!(104);
pub const IOCTL_SET_NUM_DEVICE_INPUT_BUFFERS: DWORD = HID_BUFFER_CTL_CODE!(105);
pub const IOCTL_HID_GET_COLLECTION_INFORMATION: DWORD = HID_BUFFER_CTL_CODE!(106);
pub const IOCTL_HID_ENABLE_WAKE_ON_SX: DWORD = HID_BUFFER_CTL_CODE!(107);
pub const IOCTL_HID_SET_S0_IDLE_TIMEOUT: DWORD = HID_BUFFER_CTL_CODE!(108);
pub const IOCTL_HID_GET_COLLECTION_DESCRIPTOR: DWORD = HID_CTL_CODE!(100);
pub const IOCTL_HID_FLUSH_QUEUE: DWORD = HID_CTL_CODE!(101);
pub const IOCTL_HID_SET_FEATURE: DWORD = HID_IN_CTL_CODE!(100);
pub const IOCTL_HID_SET_OUTPUT_REPORT: DWORD = HID_IN_CTL_CODE!(101);
pub const IOCTL_HID_GET_FEATURE: DWORD = HID_OUT_CTL_CODE!(100);
pub const IOCTL_GET_PHYSICAL_DESCRIPTOR: DWORD = HID_OUT_CTL_CODE!(102);
pub const IOCTL_HID_GET_HARDWARE_ID: DWORD = HID_OUT_CTL_CODE!(103);
pub const IOCTL_HID_GET_INPUT_REPORT: DWORD = HID_OUT_CTL_CODE!(104);
pub const IOCTL_HID_GET_OUTPUT_REPORT: DWORD = HID_OUT_CTL_CODE!(105);
pub const IOCTL_HID_GET_MANUFACTURER_STRING: DWORD = HID_OUT_CTL_CODE!(110);
pub const IOCTL_HID_GET_PRODUCT_STRING: DWORD = HID_OUT_CTL_CODE!(111);
pub const IOCTL_HID_GET_SERIALNUMBER_STRING: DWORD = HID_OUT_CTL_CODE!(112);
pub const IOCTL_HID_GET_INDEXED_STRING: DWORD = HID_OUT_CTL_CODE!(120);
pub const IOCTL_HID_GET_MS_GENRE_DESCRIPTOR: DWORD = HID_OUT_CTL_CODE!(121);
pub const IOCTL_HID_ENABLE_SECURE_READ: DWORD = HID_CTL_CODE!(130);
pub const IOCTL_HID_DISABLE_SECURE_READ: DWORD = HID_CTL_CODE!(131);
pub const IOCTL_HID_DEVICERESET_NOTIFICATION: DWORD = HID_CTL_CODE!(140);
STRUCT!{struct HID_XFER_PACKET {
    reportBuffer: PUCHAR,
    reportBufferLen: ULONG,
    reportId: UCHAR,
}}
pub type PHID_XFER_PACKET = *mut HID_XFER_PACKET;
//FIXME Stuff for NT_INCLUDED
STRUCT!{struct HID_COLLECTION_INFORMATION {
    DescriptorSize: ULONG,
    Polled: BOOLEAN,
    Reserved1: [UCHAR; 1],
    VendorID: USHORT,
    ProductID: USHORT,
    VersionNumber: USHORT,
}}
pub type PHID_COLLECTION_INFORMATION = *mut HID_COLLECTION_INFORMATION;
STRUCT!{struct HID_DRIVER_CONFIG {
    Size: ULONG,
    RingBufferSize: ULONG,
}}
pub type PHID_DRIVER_CONFIG = *mut HID_DRIVER_CONFIG;

// Copyright © 2016-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! USB Spec Definitions.
ENUM!{enum USB_DEVICE_SPEED {
    UsbLowSpeed = 0,
    UsbFullSpeed,
    UsbHighSpeed,
    UsbSuperSpeed,
}}
ENUM!{enum USB_DEVICE_TYPE {
    Usb11Device = 0,
    Usb20Device,
}}
STRUCT!{struct BM_REQUEST_TYPE {
    _BM: ::UCHAR,
    B: ::UCHAR,
}}
BITFIELD!{BM_REQUEST_TYPE _BM: ::UINT8 [
    Recipient set_Recipient[0..2],
    Reserved set_Reserved[2..5],
    Type set_Type[5..7],
    Dir set_Dir[7..8],
]}
pub type PBM_REQUEST_TYPE = *mut BM_REQUEST_TYPE;

STRUCT!{#[repr(packed)] struct USB_CONFIGURATION_DESCRIPTOR {
    bLength: ::UCHAR,
    bDescriptorType: ::UCHAR,
    wTotalLength: ::USHORT,
    bNumInterfaces: ::UCHAR,
    bConfigurationValue: ::UCHAR,
    iConfiguration: ::UCHAR,
    bmAttributes: ::UCHAR,
    MaxPower: ::UCHAR,
}}
pub type PUSB_CONFIGURATION_DESCRIPTOR = *mut USB_CONFIGURATION_DESCRIPTOR;

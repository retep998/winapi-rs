// Copyright © 2016-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! FFI bindings to winusb.
pub type WINUSB_INTERFACE_HANDLE = ::PVOID;
pub type PWINUSB_INTERFACE_HANDLE = *mut ::PVOID;
pub type WINUSB_ISOCH_BUFFER_HANDLE = ::PVOID;
pub type PWINUSB_ISOCH_BUFFER_HANDLE = *mut ::PVOID;
STRUCT!{#[repr(packed)] struct WINUSB_SETUP_PACKET {
    RequestType: ::UCHAR,
    Request: ::UCHAR,
    Value: ::USHORT,
    Index: ::USHORT,
    Length: ::USHORT,
}}
pub type PWINUSB_SETUP_PACKET = *mut WINUSB_SETUP_PACKET;

STRUCT!{struct USB_INTERFACE_DESCRIPTOR {
    bLength: ::UCHAR,
    bDescriptorType: ::UCHAR,
    bInterfaceNumber: ::UCHAR,
    bAlternateSetting: ::UCHAR,
    bNumEndpoints: ::UCHAR,
    bInterfaceClass: ::UCHAR,
    bInterfaceSubClass: ::UCHAR,
    bInterfaceProtocol: ::UCHAR,
    iInterface: ::UCHAR,
}}
pub type PUSB_INTERFACE_DESCRIPTOR = *mut USB_INTERFACE_DESCRIPTOR;

// Copyright © 2016-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! USBIO Definitions.
STRUCT!{struct WINUSB_PIPE_INFORMATION {
    PipeType: ::USBD_PIPE_TYPE,
    PipeId: ::UCHAR,
    MaximumPacketSize: ::USHORT,
    Interval: ::UCHAR,
}}
pub type PWINUSB_PIPE_INFORMATION = *mut WINUSB_PIPE_INFORMATION;
STRUCT!{struct WINUSB_PIPE_INFORMATION_EX {
    PipeType: ::USBD_PIPE_TYPE,
    PipeId: ::UCHAR,
    MaximumPacketSize: ::USHORT,
    Interval: ::UCHAR,
    MaximumBytesPerInterval: ::ULONG,
}}
pub type PWINUSB_PIPE_INFORMATION_EX = *mut WINUSB_PIPE_INFORMATION_EX;

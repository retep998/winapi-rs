// Copyright © 2016-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! USB Definitions.
use shared::minwindef::{ULONG};
use um::winnt::{LONG};

ENUM!{enum USBD_PIPE_TYPE {
    UsbdPipeTypeControl,
    UsbdPipeTypeIsochronous,
    UsbdPipeTypeBulk,
    UsbdPipeTypeInterrupt,
}}

pub type USBD_STATUS = LONG;

STRUCT!{struct USBD_ISO_PACKET_DESCRIPTOR {
    Offset: ULONG,
    Length: ULONG,
    Status: USBD_STATUS,
}}
pub type PUSBD_ISO_PACKET_DESCRIPTOR = *mut USBD_ISO_PACKET_DESCRIPTOR;

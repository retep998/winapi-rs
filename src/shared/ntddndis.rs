// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use ctypes::c_int;
use shared::minwindef::{UCHAR, USHORT};
STRUCT!{struct NDIS_OBJECT_HEADER {
    Type: UCHAR,
    Revision: UCHAR,
    Size: USHORT,
}}
pub type PNDIS_OBJECT_HEADER = *mut NDIS_OBJECT_HEADER;
pub type NDIS_STATUS = c_int;
pub type PNDIS_STATUS = *mut c_int;
pub const NDIS_PACKET_TYPE_DIRECTED: u32 = 0x00000001;
pub const NDIS_PACKET_TYPE_MULTICAST: u32 = 0x00000002;
pub const NDIS_PACKET_TYPE_ALL_MULTICAST: u32 = 0x00000004;
pub const NDIS_PACKET_TYPE_BROADCAST: u32 = 0x00000008;
pub const NDIS_PACKET_TYPE_PROMISCUOUS: u32 = 0x00000020;

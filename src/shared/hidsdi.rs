// Copyright © 2015-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::minwindef::{USHORT, ULONG};
use um::winnt::{PVOID};

STRUCT!{struct HIDD_CONFIGURATION {
    cookie: PVOID,
    size: ULONG,
    RingBufferSize: ULONG,
}}
pub type PHIDD_CONFIGURATION = *mut HIDD_CONFIGURATION;
STRUCT!{struct HIDD_ATTRIBUTES {
    Size: ULONG,
    VendorID: USHORT,
    ProductID: USHORT,
    VersionNumber: USHORT,
}}
pub type PHIDD_ATTRIBUTES = *mut HIDD_ATTRIBUTES;

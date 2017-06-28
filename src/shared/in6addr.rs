// Copyright © 2015-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms
use shared::minwindef::{UCHAR, USHORT};
UNION2!{union IN6_ADDR_u {
    [u64; 2],
    Byte Byte_mut: [UCHAR; 16],
    Word Word_mut: [USHORT; 8],
}}
STRUCT!{struct IN6_ADDR {
    u: IN6_ADDR_u,
}}
pub type PIN6_ADDR = *mut IN6_ADDR;
pub type LPIN6_ADDR = *mut IN6_ADDR;

// Copyright © 2015-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! MM procedure declarations, constant definitions and macros
use ctypes::c_int;
use shared::minwindef::{MAKELONG, WORD};
use um::winnt::LONG;
pub const NEWTRANSPARENT: c_int = 3;
pub const QUERYROPSUPPORT: c_int = 40;
pub const SELECTDIB: c_int = 41;
#[inline]
pub fn DIBINDEX(n: WORD) -> LONG {
    MAKELONG(n, 0x10ff)
}

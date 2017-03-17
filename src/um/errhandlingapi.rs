// Copyright © 2015-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! ApiSet Contract for api-ms-win-core-errorhandling-l1
use um::winnt::{EXCEPTION_POINTERS, LONG};

pub type PTOP_LEVEL_EXCEPTION_FILTER = Option<unsafe extern "system" fn(
    ExceptionInfo: *mut EXCEPTION_POINTERS,
) -> LONG>;
pub type LPTOP_LEVEL_EXCEPTION_FILTER = PTOP_LEVEL_EXCEPTION_FILTER;

// Copyright © 2016 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! handleapi include file
// Done as of 10.0.14393.0
use um::winnt::HANDLE;

pub const INVALID_HANDLE_VALUE: HANDLE = -1isize as HANDLE;

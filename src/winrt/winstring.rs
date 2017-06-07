// Copyright © 2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.

use shared::basetsd::{UINT32, UINT_PTR};
use shared::minwindef::BYTE;
use um::winnt::{HRESULT, VOID};

FN!{stdcall PINSPECT_HSTRING_CALLBACK(
    *const VOID,
    UINT_PTR,
    UINT32,
    *mut BYTE,
) -> HRESULT}

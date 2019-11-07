// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use um::shtypes::{PCUITEMID_CHILD, STRRET};
use um::winnt::{HRESULT, LPSTR, LPWSTR};
extern "system" {
    pub fn StrRetToStrA(
        pstr: *mut STRRET,
        pidl: PCUITEMID_CHILD,
        ppsz: *mut LPSTR,
    ) -> HRESULT;
    pub fn StrRetToStrW(
        pstr: *mut STRRET,
        pidl: PCUITEMID_CHILD,
        ppsz: *mut LPWSTR,
    ) -> HRESULT;
}

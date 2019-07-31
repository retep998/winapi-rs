// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::minwindef::{DWORD};
use um::mediaobj::{DMO_MEDIA_TYPE};
use um::winnt::{HRESULT};
extern "system" {
    pub fn MoInitMediaType(
        pmt: *mut DMO_MEDIA_TYPE,
        cbFormat: DWORD,
    ) -> HRESULT;
    pub fn MoFreeMediaType(
        pmt: *mut DMO_MEDIA_TYPE,
    ) -> HRESULT;
    pub fn MoCopyMediaType(
        pmtDest: *mut DMO_MEDIA_TYPE,
        pmtSrc: *const DMO_MEDIA_TYPE,
    ) -> HRESULT;
    pub fn MoCreateMediaType(
        ppmt: *mut *mut DMO_MEDIA_TYPE,
        cbFormat: DWORD,
    ) -> HRESULT;
    pub fn MoDeleteMediaType(
        pmt: *mut DMO_MEDIA_TYPE,
    ) -> HRESULT;
    pub fn MoDuplicateMediaType(
        ppmtDest: *mut *mut DMO_MEDIA_TYPE,
        pmtSrc: *const DMO_MEDIA_TYPE,
    ) -> HRESULT;
}

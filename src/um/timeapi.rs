// Copyright © 2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::minwindef::{DWORD, UINT};
use um::mmsyscom::{LPMMTIME, MMRESULT, TIMERR_BASE};
pub const TIMERR_NOERROR: UINT = 0;
pub const TIMERR_NOCANDO: UINT = TIMERR_BASE + 1;
pub const TIMERR_STRUCT: UINT = TIMERR_BASE + 33;
STRUCT!{#[repr(packed)] struct TIMECAPS {
    wPeriodMin: UINT,
    wPeriodMax: UINT,
}}
pub type PTIMECAPS = *mut TIMECAPS;
pub type NPTIMECAPS = *mut TIMECAPS;
pub type LPTIMECAPS = *mut TIMECAPS;
extern "system" {
    pub fn timeGetSystemTime(
        pmmt: LPMMTIME,
        cbmmt: UINT,
    ) -> MMRESULT;
    pub fn timeGetTime() -> DWORD;
    pub fn timeGetDevCaps(
        ptc: LPTIMECAPS,
        cbtc: UINT,
    ) -> MMRESULT;
    pub fn timeBeginPeriod(
        uPeriod: UINT,
    ) -> MMRESULT;
    pub fn timeEndPeriod(
        uPeriod: UINT,
    ) -> MMRESULT;
}

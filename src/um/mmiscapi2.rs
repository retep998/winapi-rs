// Copyright © 2015-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! ApiSet Contract for api-ms-win-mm-misc-l2-1-0
use shared::basetsd::DWORD_PTR;
use shared::minwindef::UINT;
use um::mmsyscom::MMRESULT;
FN!{stdcall LPTIMECALLBACK(
    uTimerID: UINT,
    uMsg: UINT,
    dwUser: DWORD_PTR,
    dw1: DWORD_PTR,
    dw2: DWORD_PTR,
) -> ()}
pub const TIME_ONESHOT: UINT = 0x0000;
pub const TIME_PERIODIC: UINT = 0x0001;
pub const TIME_CALLBACK_FUNCTION: UINT = 0x0000;
pub const TIME_CALLBACK_EVENT_SET: UINT = 0x0010;
pub const TIME_CALLBACK_EVENT_PULSE: UINT = 0x0020;
pub const TIME_KILL_SYNCHRONOUS: UINT = 0x0100;
extern "system" {
    pub fn timeSetEvent(
        uDelay: UINT,
        uResolution: UINT,
        fptc: LPTIMECALLBACK,
        dwUser: DWORD_PTR,
        fuEvent: UINT,
    ) -> MMRESULT;
    pub fn timeKillEvent(
        uTimerID: UINT,
    ) -> MMRESULT;
}

// Copyright © 2015-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! ApiSet Contract for api-ms-win-core-timezone-l1

use shared::minwindef::DWORD;
use um::minwinbase::SYSTEMTIME;
use um::winnt::{BOOLEAN, LONG, WCHAR};

pub const TIME_ZONE_ID_INVALID: DWORD = 0xFFFFFFFF;
STRUCT!{struct TIME_ZONE_INFORMATION {
    Bias: LONG,
    StandardName: [WCHAR; 32],
    StandardDate: SYSTEMTIME,
    StandardBias: LONG,
    DaylightName: [WCHAR; 32],
    DaylightDate: SYSTEMTIME,
    DaylightBias: LONG,
}}
pub type PTIME_ZONE_INFORMATION = *mut TIME_ZONE_INFORMATION;
pub type LPTIME_ZONE_INFORMATION = *mut TIME_ZONE_INFORMATION;
STRUCT!{struct DYNAMIC_TIME_ZONE_INFORMATION {
    Bias: LONG,
    StandardName: [WCHAR; 32],
    StandardDate: SYSTEMTIME,
    StandardBias: LONG,
    DaylightName: [WCHAR; 32],
    DaylightDate: SYSTEMTIME,
    DaylightBias: LONG,
    TimeZoneKeyName: [WCHAR; 128],
    DynamicDaylightTimeDisabled: BOOLEAN,
}}
pub type PDYNAMIC_TIME_ZONE_INFORMATION = *mut DYNAMIC_TIME_ZONE_INFORMATION;

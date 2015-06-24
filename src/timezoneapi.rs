// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! ApiSet Contract for api-ms-win-core-timezone-l1
pub const TIME_ZONE_ID_INVALID: ::DWORD = 0xFFFFFFFF;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct TIME_ZONE_INFORMATION {
    pub Bias: ::LONG,
    pub StandardName: [::WCHAR; 32],
    pub StandardDate: ::SYSTEMTIME,
    pub StandardBias: ::LONG,
    pub DaylightName: [::WCHAR; 32],
    pub DaylightDate: ::SYSTEMTIME,
    pub DaylightBias: ::LONG,
}
pub type PTIME_ZONE_INFORMATION = *mut TIME_ZONE_INFORMATION;
pub type LPTIME_ZONE_INFORMATION = *mut TIME_ZONE_INFORMATION;
#[repr(C)] #[derive(Copy)]
pub struct DYNAMIC_TIME_ZONE_INFORMATION {
    pub Bias: ::LONG,
    pub StandardName: [::WCHAR; 32],
    pub StandardDate: ::SYSTEMTIME,
    pub StandardBias: ::LONG,
    pub DaylightName: [::WCHAR; 32],
    pub DaylightDate: ::SYSTEMTIME,
    pub DaylightBias: ::LONG,
    pub TimeZoneKeyName: [::WCHAR; 128],
    pub DynamicDaylightTimeDisabled: ::BOOLEAN,
}
impl Clone for DYNAMIC_TIME_ZONE_INFORMATION {
    fn clone(&self) -> DYNAMIC_TIME_ZONE_INFORMATION { *self }
}
pub type PDYNAMIC_TIME_ZONE_INFORMATION = *mut DYNAMIC_TIME_ZONE_INFORMATION;

// Copyright © 2015-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! ApiSet Contract for api-ms-win-core-synch-l1
pub type SRWLOCK = ::RTL_SRWLOCK;
pub type PSRWLOCK = *mut ::RTL_SRWLOCK;
pub type SYNCHRONIZATION_BARRIER = ::RTL_BARRIER;
pub type PSYNCHRONIZATION_BARRIER = ::PRTL_BARRIER;
pub type LPSYNCHRONIZATION_BARRIER = ::PRTL_BARRIER;
pub type PINIT_ONCE_FN = Option<unsafe extern "system" fn(
    InitOnce: ::PINIT_ONCE,
    Parameter: ::PVOID,
    Context: *mut ::PVOID,
) -> ::BOOL>;
pub type PTIMERAPCROUTINE = Option<unsafe extern "system" fn(
    lpArgToCompletionRoutine: ::LPVOID,
    dwTimerLowValue: ::DWORD,
    dwTimerHighValue: ::DWORD,
)>;

// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
pub type MMRESULT = ::UINT;

pub const TIMERR_NOERROR: ::MMRESULT = 0;
pub const TIMERR_BASE: ::MMRESULT = 96;
pub const TIMERR_NOCANDO: ::MMRESULT = TIMERR_BASE + 1;
pub const TIMERR_STRUCT: ::MMRESULT = TIMERR_BASE + 33;

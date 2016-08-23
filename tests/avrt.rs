// Copyright © 2016, Baptiste AUBRY
// Licensed under the MIT License <LICENSE.md>
#![cfg(windows)]
extern crate avrt;
extern crate winapi;

use avrt::*;

#[inline(never)] fn bb<T>(_: T) {}
#[test] #[cfg(target_env = "msvc")]
fn functions() {
    bb(AvSetMmThreadCharacteristicsA);   
    bb(AvSetMmThreadCharacteristicsW);
    bb(AvSetMmMaxThreadCharacteristicsA);
    bb(AvSetMmMaxThreadCharacteristicsW);
    bb(AvRevertMmThreadCharacteristics);
    bb(AvRtCreateThreadOrderingGroup);
    bb(AvRtCreateThreadOrderingGroupExA);
    bb(AvRtCreateThreadOrderingGroupExW);
    bb(AvRtJoinThreadOrderingGroup);
    bb(AvRtWaitOnThreadOrderingGroup);
    bb(AvRtLeaveThreadOrderingGroup);
    bb(AvRtDeleteThreadOrderingGroup);
    bb(AvQuerySystemResponsiveness);

    assert_eq!(winapi::THREAD_ORDER_GROUP_INFINITE_TIMEOUT, -1);
}

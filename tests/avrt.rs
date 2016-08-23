// Copyright © 2016, Baptiste AUBRY
// Licensed under the MIT License <LICENSE.md>
#![cfg(windows)]
extern crate avrt;
use avrt::*;
#[inline(never)] fn bb<T>(_: T) {}
#[test]
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
}

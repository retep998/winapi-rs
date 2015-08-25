// Copyright © 2015, skdltmxn
// Licensed under the MIT License <LICENSE.md>
#![cfg(windows)]
extern crate psapi;
use psapi::*;
#[inline(never)] fn bb<T>(_: T) {}
#[test]
fn functions() {
    bb(EmptyWorkingSet);
    bb(EnumDeviceDrivers);
    bb(EnumPageFilesA);
    bb(EnumPageFilesW);
    bb(EnumProcessModules);
    bb(EnumProcessModulesEx);
    bb(EnumProcesses);
    bb(GetDeviceDriverBaseNameA);
    bb(GetDeviceDriverBaseNameW);
    bb(GetDeviceDriverFileNameA);
    bb(GetDeviceDriverFileNameW);
    bb(GetMappedFileNameA);
    bb(GetMappedFileNameW);
    bb(GetModuleBaseNameA);
    bb(GetModuleBaseNameW);
    bb(GetModuleFileNameExA);
    bb(GetModuleFileNameExW);
    bb(GetModuleInformation);
    bb(GetPerformanceInfo);
    bb(GetProcessImageFileNameA);
    bb(GetProcessImageFileNameW);
    bb(GetProcessMemoryInfo);
    bb(GetWsChanges);
    bb(GetWsChangesEx);
    bb(InitializeProcessForWsWatch);
    bb(QueryWorkingSet);
    bb(QueryWorkingSetEx);
}

// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to psapi.
#![cfg(windows)]
extern crate winapi;
use winapi::*;
extern "system" {
    // pub fn EmptyWorkingSet();
    // pub fn EnumDeviceDrivers();
    // pub fn EnumPageFilesA();
    // pub fn EnumPageFilesW();
    // pub fn EnumProcessModules();
    // pub fn EnumProcessModulesEx();
    // pub fn EnumProcesses();
    // pub fn GetDeviceDriverBaseNameA();
    // pub fn GetDeviceDriverBaseNameW();
    // pub fn GetDeviceDriverFileNameA();
    // pub fn GetDeviceDriverFileNameW();
    // pub fn GetMappedFileNameA();
    // pub fn GetMappedFileNameW();
    // pub fn GetModuleBaseNameA();
    // pub fn GetModuleBaseNameW();
    // pub fn GetModuleFileNameExA();
    // pub fn GetModuleFileNameExW();
    // pub fn GetModuleInformation();
    // pub fn GetPerformanceInfo();
    pub fn GetProcessImageFileNameA(
        hProcess: HANDLE, lpImageFileName: LPSTR, nSize: DWORD,
    ) -> DWORD;
    pub fn GetProcessImageFileNameW(
        hProcess: HANDLE, lpImageFileName: LPWSTR, nSize: DWORD,
    ) -> DWORD;
    // pub fn GetProcessMemoryInfo();
    // pub fn GetWsChanges();
    // pub fn GetWsChangesEx();
    // pub fn InitializeProcessForWsWatch();
    // pub fn QueryWorkingSet();
    // pub fn QueryWorkingSetEx();
}

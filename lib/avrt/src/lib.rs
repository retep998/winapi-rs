// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to avrt.
#![cfg(windows)]
extern crate winapi;

extern "system" {
    pub fn AvSetMmThreadCharacteristicsA(
        TaskName: winapi::LPCSTR, TaskIndex: winapi::LPDWORD
    ) -> winapi::HANDLE;
    
    pub fn AvSetMmThreadCharacteristicsW(
        TaskName: winapi::LPCSTR, TaskIndex: winapi::LPDWORD
    ) -> winapi::HANDLE;

    pub fn AvSetMmMaxThreadCharacteristicsA (
        FirstTask: winapi::LPCSTR, SecondTask: winapi::LPCSTR, TaskIndex: winapi::LPDWORD
    ) -> winapi::HANDLE;

    pub fn AvSetMmMaxThreadCharacteristicsW (
        FirstTask: winapi::LPCWSTR, SecondTask: winapi::LPCWSTR, TaskIndex: winapi::LPDWORD
    ) -> winapi::HANDLE;

    pub fn AvRevertMmThreadCharacteristics(avrt_handle: winapi::HANDLE) -> winapi::BOOL;
    
    pub fn AvSetMmThreadPriority (
        AvrtHandle: winapi::HANDLE, Priority: winapi::AVRT_PRIORITY) -> winapi::BOOL;

    pub fn AvRtCreateThreadOrderingGroup (
        Context: winapi::PHANDLE, Period: winapi::PLARGE_INTEGER,
        ThreadOrderingGuid: * mut winapi::GUID, Timeout: winapi::PLARGE_INTEGER
    ) -> winapi::BOOL;
    
    pub fn AvRtCreateThreadOrderingGroupExA (
        Context: winapi::PHANDLE, Period: winapi::PLARGE_INTEGER,
        ThreadOrderingGuid: * mut winapi::GUID, Timeout: winapi::PLARGE_INTEGER,
        TaskName: winapi::LPCSTR
    ) -> winapi::BOOL;

    pub fn AvRtCreateThreadOrderingGroupExW (
        Context: winapi::PHANDLE, Period: winapi::PLARGE_INTEGER,
        ThreadOrderingGuid: * mut winapi::GUID, Timeout: winapi::PLARGE_INTEGER,
        TaskName: winapi::LPCWSTR
    ) -> winapi::BOOL;

    pub fn AvRtJoinThreadOrderingGroup (
        Context: winapi::PHANDLE, ThreadOrderingGuid: * mut winapi::GUID,
        Before: winapi::BOOL
    ) -> winapi::BOOL;
    
    pub fn AvRtWaitOnThreadOrderingGroup (Context: winapi::HANDLE) -> winapi::BOOL;

    pub fn AvRtLeaveThreadOrderingGroup (Context: winapi::HANDLE) -> winapi::BOOL;

    pub fn AvRtDeleteThreadOrderingGroup (Context: winapi::HANDLE) -> winapi::BOOL;

    pub fn AvQuerySystemResponsiveness (
        AvrtHandle: winapi::HANDLE, SystemResponsivenessValue: winapi::PULONG
    ) -> winapi::BOOL;
}

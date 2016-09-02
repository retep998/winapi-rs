// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to shcore.
#![cfg(windows)]
extern crate winapi;
use winapi::*;
extern "system" {
    pub fn GetDpiForMonitor(hmonitor: HMONITOR, dpiType: MONITOR_DPI_TYPE, dpiX: *mut UINT, dpiY: *mut UINT) -> HRESULT;
    pub fn GetDpiForShellUiComponent(component: SHELL_UI_COMPONENT) -> UINT;
    pub fn GetProcessDPIAwareness(hProcess: HANDLE, value: *mut PROCESS_DPI_AWARENESS) -> HRESULT;
    pub fn SetProcessDPIAwareness(value: PROCESS_DPI_AWARENESS) -> HRESULT;
    pub fn LogicalToPhysicalPointForPerMonitorDPI(hwnd: HWND, lpPoint: LPPOINT) -> BOOL;
    pub fn PhysicalToLogicalPointForPerMonitorDPI(hwnd: HWND, lpPoint: LPPOINT) -> BOOL;
}

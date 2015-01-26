// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to user32.
#![no_std]
#![experimental]
extern crate winapi;
use winapi::*;
extern "system" {
    pub fn AdjustWindowRectEx(
        lpRect: LPRECT, dwStyle: DWORD, bMenu: BOOL, dwExStyle: DWORD,
    ) -> BOOL;
    pub fn BeginPaint(hwnd: HWND, lpPaint: LPPAINTSTRUCT) -> HDC;
    pub fn ChangeDisplaySettingsW(lpDevMode: *mut DEVMODEW, dwFlags: DWORD) -> LONG;
    pub fn ChangeDisplaySettingsExW(
        lpszDeviceName: LPCWSTR, lpDevMode: *mut DEVMODEW, hwnd: HWND, dwFlags: DWORD,
        lParam: LPVOID,
    ) -> LONG;
    pub fn CreateWindowExW(
        dwExStyle: DWORD, lpClassName: LPCWSTR, lpWindowName: LPCWSTR, dwStyle: DWORD, x: c_int,
        y: c_int, nWidth: c_int, nHeight: c_int, hWndParent: HWND, hMenu: HMENU,
        hInstance: HINSTANCE, lpParam: LPVOID,
    ) -> HWND;
    pub fn DefWindowProcW(hWnd: HWND, Msg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT;
    pub fn DestroyWindow(hWnd: HWND) -> BOOL;
    pub fn DispatchMessageW(lpmsg: *const MSG) -> LRESULT;
    pub fn EndPaint(hWnd: HWND, lpPaint: *const PAINTSTRUCT) -> BOOL;
    pub fn EnumDisplayDevicesW(
        lpDevice: LPCWSTR, iDevNum: DWORD, lpDisplayDevice: PDISPLAY_DEVICEW, dwFlags: DWORD,
    ) -> BOOL;
    pub fn EnumDisplaySettingsExW(
        lpszDeviceName: LPCWSTR, iModeNum: DWORD, lpDevMode: *mut DEVMODEW, dwFlags: DWORD,
    ) -> BOOL;
    pub fn FillRect(hDC: HDC, lprc: *const RECT, hbr: HBRUSH) -> c_int;
    pub fn FindWindowA (lpClassName: LPCSTR, lpWindowName: LPCSTR) -> HWND;
    pub fn GetClientRect(hWnd: HWND, lpRect: LPRECT) -> BOOL;
    pub fn GetDC(hWnd: HWND) -> HDC;
    pub fn GetMessageW(lpMsg: LPMSG, hWnd: HWND, wMsgFilterMin: UINT, wMsgFilterMax: UINT) -> BOOL;
    pub fn GetSystemMetrics(nIndex: c_int) -> c_int;
    pub fn GetWindowPlacement(hWnd: HWND, lpwndpl: *mut WINDOWPLACEMENT) -> BOOL;
    pub fn GetWindowRect(hWnd: HWND, lpRect: LPRECT) -> BOOL;
    pub fn InvalidateRect(hWnd: HWND, lpRect: *const RECT, bErase: BOOL) -> BOOL;
    pub fn LoadImageA(
        hInst: HINSTANCE, name: LPCSTR, type_: UINT, cx: c_int, cy: c_int, fuLoad: UINT,
    ) -> HANDLE;
    pub fn LoadImageW(
        hInst: HINSTANCE, name: LPCWSTR, type_: UINT, cx: c_int, cy: c_int, fuLoad: UINT,
    ) -> HANDLE;
    pub fn MessageBoxA(hWnd: HWND, lpText: LPCSTR, lpCaption: LPCSTR, uType: UINT) -> c_int;
    pub fn MessageBoxW(hWnd: HWND, lpText: LPCWSTR, lpCaption: LPCWSTR, uType: UINT) -> c_int;
    pub fn MessageBoxExA(
        hWnd: HWND, lpText: LPCSTR, lpCaption: LPCSTR, uType: UINT, wLanguageId: WORD,
    ) -> c_int;
    pub fn MessageBoxExW(
        hWnd: HWND, lpText: LPCWSTR, lpCaption: LPCWSTR, uType: UINT, wLanguageId: WORD,
    ) -> c_int;
    pub fn PeekMessageW(
        lpMsg: LPMSG, hWnd: HWND, wMsgFilterMin: UINT, wMsgFilterMax: UINT, wRemoveMsg: UINT,
    ) -> BOOL;
    pub fn PostMessageW(hWnd: HWND, Msg: UINT, wParam: WPARAM, lParam: LPARAM) -> BOOL;
    pub fn PostQuitMessage(nExitCode: c_int);
    pub fn RegisterClassExW(lpWndClass: *const WNDCLASSEXW) -> ATOM;
    pub fn SendInput(cInputs: UINT, pInputs: LPINPUT, cbSize: c_int) -> UINT;
    pub fn SendMessageA(hWnd: HWND, Msg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT;
    pub fn SendMessageW(hWnd: HWND, Msg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT;
    pub fn SetFocus(hWnd: HWND) -> HWND;
    pub fn SetForegroundWindow(hWnd: HWND) -> BOOL;
    pub fn SetWindowPos(
        hWnd: HWND, hWndInsertAfter: HWND, X: c_int, Y: c_int, cx: c_int, cy: c_int, uFlags: UINT,
    ) -> BOOL;
    pub fn SetWindowTextW(hWnd: HWND, lpString: LPCWSTR) -> BOOL;
    pub fn ShowWindow(hWnd: HWND, nCmdShow: c_int) -> BOOL;
    pub fn ShowWindowAsync(hWnd: HWND, nCmdShow: c_int) -> BOOL;
    pub fn TranslateMessage(lpmsg: *const MSG) -> BOOL;
    pub fn UpdateWindow(hWnd: HWND) -> BOOL;
    pub fn WaitMessage() -> BOOL;
}

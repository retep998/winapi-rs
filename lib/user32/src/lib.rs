// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to user32.
#![cfg(windows)]
extern crate winapi;
use winapi::*;
extern "system" {
    pub fn ArrangeIconicWindows(hWnd: HWND) -> UINT;
    pub fn BlockInput(fBlockIt: BOOL) -> BOOL;
    // pub fn BroadcastSystemMessage();
    pub fn CallNextHookEx(hhk: HHOOK, nCode: c_int, wParam: WPARAM, lParam: LPARAM) -> LRESULT;
    pub fn CancelShutdown() -> BOOL;
    // pub fn CascadeChildWindows();
    pub fn CascadeWindows(
        hwndParent: HWND, wHow: UINT, lpRect: *const RECT, cKids: UINT, lpKids: *const HWND,
    ) -> WORD;
    pub fn ChangeDisplaySettingsA(lpDevMode: *mut DEVMODEA, dwFlags: DWORD) -> LONG;
    pub fn ChangeDisplaySettingsExA(
        lpszDeviceName: LPCSTR, lpDevMode: *mut DEVMODEA, hwnd: HWND, dwFlags: DWORD,
        lParam: LPVOID,
    ) -> LONG;
    pub fn ChangeDisplaySettingsExW(
        lpszDeviceName: LPCWSTR, lpDevMode: *mut DEVMODEW, hwnd: HWND, dwFlags: DWORD,
        lParam: LPVOID,
    ) -> LONG;
    pub fn ChangeDisplaySettingsW(lpDevMode: *mut DEVMODEW, dwFlags: DWORD) -> LONG;
    pub fn ChangeWindowMessageFilter(message: UINT, dwFlag: DWORD) -> BOOL;
    pub fn ChangeWindowMessageFilterEx(
        hwnd: HWND, message: UINT, action: DWORD, pChangeFilterStruct: PCHANGEFILTERSTRUCT,
    ) -> BOOL;
    pub fn CheckMenuRadioItem(
        hMenu: HMENU, first: UINT, last: UINT, check: UINT, flags: UINT,
    ) -> BOOL;
    // pub fn CloseGestureInfoHandle();
    pub fn CopyIcon(hIcon: HICON) -> HICON;
    pub fn CopyImage(h: HANDLE, type_: UINT, cx: c_int, cy: c_int, flags: UINT) -> HANDLE;
    pub fn CreateCursor(
        hInst: HINSTANCE, xHotSpot: c_int, yHotSpot: c_int, nWidth: c_int, nHeight: c_int,
        pvAndPlane: *const VOID, pvXORPlane: *const VOID,
    ) -> HCURSOR;
    pub fn CreateIcon(
        hInstance: HINSTANCE, nWidth: c_int, nHeight: c_int, cPlanes: BYTE, cBitsPixel: BYTE,
        lpbANDbits: *const BYTE, lpbXORbits: *const BYTE,
    ) -> HICON;
    pub fn CreateIconFromResource(
        presbits: PBYTE, dwResSize: DWORD, fIcon: BOOL, dwVer: DWORD,
    ) -> HICON;
    pub fn CreateIconFromResourceEx(
        presbits: PBYTE, dwResSize: DWORD, fIcon: BOOL, dwVer: DWORD, cxDesired: c_int,
        cyDesired: c_int, Flags: UINT,
    ) -> HICON;
    pub fn CreateIconIndirect(piconinfo: PICONINFO) -> HICON;
    pub fn CreateMDIWindowA(
        lpClassName: LPCSTR, lpWindowName: LPCSTR, dwStyle: DWORD, X: c_int, Y: c_int,
        nWidth: c_int, nHeight: c_int, hWndParent: HWND, hInstance: HINSTANCE, lParam: LPARAM,
    ) -> HWND;
    pub fn CreateMDIWindowW(
        lpClassName: LPCWSTR, lpWindowName: LPCWSTR, dwStyle: DWORD, X: c_int, Y: c_int,
        nWidth: c_int, nHeight: c_int, hWndParent: HWND, hInstance: HINSTANCE, lParam: LPARAM,
    ) -> HWND;
    // pub fn DdeAbandonTransaction();
    // pub fn DdeAccessData();
    // pub fn DdeAddData();
    // pub fn DdeClientTransaction();
    // pub fn DdeCmpStringHandles();
    // pub fn DdeConnect();
    // pub fn DdeConnectList();
    // pub fn DdeCreateDataHandle();
    // pub fn DdeCreateStringHandleA();
    // pub fn DdeCreateStringHandleW();
    // pub fn DdeDisconnect();
    // pub fn DdeDisconnectList();
    // pub fn DdeEnableCallback();
    // pub fn DdeFreeDataHandle();
    // pub fn DdeFreeStringHandle();
    // pub fn DdeGetData();
    // pub fn DdeGetLastError();
    // pub fn DdeImpersonateClient();
    // pub fn DdeInitializeA();
    // pub fn DdeInitializeW();
    // pub fn DdeKeepStringHandle();
    // pub fn DdeNameService();
    // pub fn DdePostAdvise();
    // pub fn DdeQueryConvInfo();
    // pub fn DdeQueryNextServer();
    // pub fn DdeQueryStringA();
    // pub fn DdeQueryStringW();
    // pub fn DdeReconnect();
    // pub fn DdeSetQualityOfService();
    // pub fn DdeSetUserHandle();
    // pub fn DdeUnaccessData();
    // pub fn DdeUninitialize();
    pub fn DefFrameProcA(
        hwnd: HWND, hwndMDIClient: HWND, uMsg: UINT, wParam: WPARAM, lParam: LPARAM,
    ) -> LRESULT;
    pub fn DefFrameProcW(
        hwnd: HWND, hwndMDIClient: HWND, uMsg: UINT, wParam: WPARAM, lParam: LPARAM,
    ) -> LRESULT;
    pub fn DefMDIChildProcA(
        hwnd: HWND, uMsg: UINT, wParam: WPARAM, lParam: LPARAM,
    ) -> LRESULT;
    pub fn DefMDIChildProcW(
        hwnd: HWND, uMsg: UINT, wParam: WPARAM, lParam: LPARAM,
    ) -> LRESULT;
    pub fn DefRawInputProc(paRawInput: *mut PRAWINPUT, nInput: INT, cbSizeHeader: UINT) -> LRESULT;
    pub fn DeregisterShellHookWindow(hwnd: HWND) -> BOOL;
    pub fn DestroyCursor(hCursor: HCURSOR) -> BOOL;
    pub fn DestroyIcon(hIcon: HICON) -> BOOL;
    // pub fn DisplayConfigGetDeviceInfo();
    // pub fn DisplayConfigSetDeviceInfo();
    pub fn DlgDirListA(
        hDlg: HWND, lpPathSpec: LPSTR, nIDListBox: c_int, nIDStaticPath: c_int, uFileType: UINT
    ) -> c_int;
    pub fn DlgDirListComboBoxA(
        hDlg: HWND, lpPathSpec: LPSTR, nIDComboBox: c_int, nIDStaticPath: c_int, uFiletype: UINT
    ) -> c_int;
    pub fn DlgDirListComboBoxW(
        hDlg: HWND, lpPathSpec: LPWSTR, nIDComboBox: c_int, nIDStaticPath: c_int, uFiletype: UINT
    ) -> c_int;
    pub fn DlgDirListW(
        hDlg: HWND, lpPathSpec: LPWSTR, nIDListBox: c_int, nIDStaticPath: c_int, uFileType: UINT
    ) -> c_int;
    pub fn DlgDirSelectComboBoxExA(
        hwndDlg: HWND, lpString: LPSTR, cchOut: c_int, idComboBox: c_int
    ) -> BOOL;
    pub fn DlgDirSelectComboBoxExW(
        hwndDlg: HWND, lpString: LPWSTR, cchOut: c_int, idComboBox: c_int
    ) -> BOOL;
    pub fn DlgDirSelectExA(
        hwndDlg: HWND, lpString: LPSTR, chCount: c_int, idListBox: c_int
    ) -> BOOL;
    pub fn DlgDirSelectExW(
        hwndDlg: HWND, lpString: LPWSTR, chCount: c_int, idListBox: c_int
    ) -> BOOL;
    // pub fn DrawFrame();
    pub fn DrawIconEx(
        hdc: HDC, xLeft: c_int, yTop: c_int, hIcon: HICON, cxWidth: c_int, cyWidth: c_int,
        istepIfAniCur: UINT, hbrFlickerFreeDraw: HBRUSH, diFlags: UINT,
    ) -> BOOL;
    // pub fn EditWndProc();
    // pub fn EnableSessionForMMCSS();
    pub fn EndTask(hWnd: HWND, fShutDown: BOOL, fForce: BOOL) -> BOOL;
    pub fn EnumChildWindows(
        hwndParent: HWND, lpEnumFunc: WNDENUMPROC, lpParam: LPARAM,
    ) -> BOOL;
    pub fn EnumDisplayDevicesA(
        lpDevice: LPCSTR, iDevNum: DWORD, lpDisplayDevice: PDISPLAY_DEVICEA, dwFlags: DWORD,
    ) -> BOOL;
    pub fn EnumDisplayDevicesW(
        lpDevice: LPCWSTR, iDevNum: DWORD, lpDisplayDevice: PDISPLAY_DEVICEW, dwFlags: DWORD,
    ) -> BOOL;
    pub fn EnumDisplayMonitors(
        hdc: HDC, lprcClip: LPCRECT, lpfnEnum: MONITORENUMPROC, dwData: LPARAM,
    ) -> BOOL;
    pub fn EnumDisplaySettingsA(
        lpszDeviceName: LPCSTR, iModeNum: DWORD, lpDevMode: *mut DEVMODEA,
    ) -> BOOL;
    pub fn EnumDisplaySettingsExA(
        lpszDeviceName: LPCSTR, iModeNum: DWORD, lpDevMode: *mut DEVMODEA, dwFlags: DWORD,
    ) -> BOOL;
    pub fn EnumDisplaySettingsExW(
        lpszDeviceName: LPCWSTR, iModeNum: DWORD, lpDevMode: *mut DEVMODEW, dwFlags: DWORD,
    ) -> BOOL;
    pub fn EnumDisplaySettingsW(
        lpszDeviceName: LPCWSTR, iModeNum: DWORD, lpDevMode: *mut DEVMODEW,
    ) -> BOOL;
    pub fn EnumThreadWindows(dwThreadId: DWORD, lpfn: WNDENUMPROC, lParam: LPARAM) -> BOOL;
    pub fn EnumWindows(lpEnumFunc: WNDENUMPROC, lParam: LPARAM) -> BOOL;
    pub fn FindWindowA (lpClassName: LPCSTR, lpWindowName: LPCSTR) -> HWND;
    pub fn FindWindowExA(
        hWndParent: HWND, hWndChildAfter: HWND, lpszClass: LPCSTR, lpszWindow: LPCSTR,
    ) -> HWND;
    pub fn FindWindowExW(
        hWndParent: HWND, hWndChildAfter: HWND, lpszClass: LPCWSTR, lpszWindow: LPCWSTR,
    ) -> HWND;
    pub fn FindWindowW(lpClassName: LPCWSTR, lpWindowName: LPCWSTR) -> HWND;
    // pub fn FreeDDElParam();
    // pub fn GetAltTabInfo();
    // pub fn GetAltTabInfoA();
    // pub fn GetAltTabInfoW();
    pub fn GetAncestor(hWnd: HWND, gaFlags: UINT) -> HWND;
    // pub fn GetAutoRotationState();
    // pub fn GetCIMSSM();
    pub fn GetClassNameA(hWnd: HWND, lpClassName: LPCSTR, nMaxCount: c_int) -> c_int;
    pub fn GetClassNameW(hWnd: HWND, lpClassName: LPCWSTR, nMaxCount: c_int) -> c_int;
    // pub fn GetComboBoxInfo();
    // pub fn GetCurrentInputMessageSource();
    // pub fn GetCursorInfo();
    pub fn GetDesktopWindow() -> HWND;
    // pub fn GetDisplayAutoRotationPreferences();
    // pub fn GetDisplayConfigBufferSizes();
    // pub fn GetGUIThreadInfo();
    // pub fn GetGestureConfig();
    // pub fn GetGestureExtraArgs();
    // pub fn GetGestureInfo();
    // pub fn GetGuiResources();
    pub fn GetIconInfo(hIcon: HICON, piconinfo: PICONINFO) -> BOOL;
    // pub fn GetIconInfoExA();
    // pub fn GetIconInfoExW();
    // pub fn GetInputDesktop();
    // pub fn GetInputLocaleInfo();
    pub fn GetLastActivePopup(hWnd: HWND) -> HWND;
    // pub fn GetListBoxInfo();
    // pub fn GetMenuBarInfo();
    pub fn GetMonitorInfoA(hMonitor: HMONITOR, lpmi: LPMONITORINFO) -> BOOL;
    pub fn GetMonitorInfoW(hMonitor: HMONITOR, lpmi: LPMONITORINFO) -> BOOL;
    pub fn GetParent(hWnd: HWND) -> HWND;
    // pub fn GetPointerDevice();
    // pub fn GetPointerDeviceCursors();
    // pub fn GetPointerDeviceProperties();
    // pub fn GetPointerDeviceRects();
    // pub fn GetPointerDevices();
    // pub fn GetProcessDefaultLayout();
    pub fn GetRawInputBuffer(pData: PRAWINPUT, pcbSize: PUINT, cbSizeHeader: UINT) -> UINT;
    pub fn GetRawInputData(
        hRawInput: HRAWINPUT, uiCommand: UINT, pData: LPVOID, pcbSize: PUINT, cbSizeHeader: UINT,
    ) -> UINT;
    pub fn GetRawInputDeviceInfoA(
        hDevice: HANDLE, uiCommand: UINT, pData: LPVOID, pcbSize: PUINT,
    ) -> UINT;
    pub fn GetRawInputDeviceInfoW(
        hDevice: HANDLE, uiCommand: UINT, pData: LPVOID, pcbSize: PUINT,
    ) -> UINT;
    pub fn GetRawInputDeviceList(
        pRawInputDeviceList: PRAWINPUTDEVICELIST, puiNumDevices: PUINT, cbSize: UINT,
    ) -> UINT;
    // pub fn GetRawPointerDeviceData();
    pub fn GetRegisteredRawInputDevices(
        pRawInputDevices: PRAWINPUTDEVICE, puiNumDevices: PUINT, cbSize: UINT,
    ) -> UINT;
    // pub fn GetScrollBarInfo();
    pub fn GetScrollInfo(hwnd: HWND, nBar: c_int, lpsi: *mut SCROLLINFO) -> BOOL;
    pub fn GetShellWindow() -> HWND;
    // pub fn GetTitleBarInfo();
    pub fn GetTopWindow(hWnd: HWND) -> HWND;
    pub fn GetWindow(hWnd: HWND, uCmd: UINT) -> HWND;
    // pub fn GetWindowInfo();
    // pub fn GetWindowModuleFileName();
    pub fn GetWindowModuleFileNameA(
        hWnd: HWND, lpszFileName: LPCSTR, cchFileNameMax: UINT,
    ) -> UINT;
    pub fn GetWindowModuleFileNameW(
        hWnd: HWND, lpszFileName: LPWSTR, cchFileNameMax: UINT,
    ) -> UINT;
    pub fn GetWindowThreadProcessId(hWnd: HWND, lpdwProcessId: LPDWORD) -> DWORD;
    // pub fn IMPGetIMEA();
    // pub fn IMPGetIMEW();
    // pub fn IMPQueryIMEA();
    // pub fn IMPQueryIMEW();
    // pub fn IMPSetIMEA();
    // pub fn IMPSetIMEW();
    // pub fn ImpersonateDdeClientWindow();
    pub fn InternalGetWindowText(hWnd: HWND, pString: LPWSTR, cchMaxCount: c_int) -> c_int;
    pub fn IsDialogMessageA(hDlg: HWND, lpMsg: LPMSG) -> BOOL;
    pub fn IsDialogMessageW(hDlg: HWND, lpMsg: LPMSG) -> BOOL;
    pub fn IsGUIThread(bConvert: BOOL) -> BOOL;
    pub fn IsImmersiveProcess(hProcess: HANDLE) -> BOOL;
    // pub fn IsInDesktopWindowBand();
    pub fn IsProcessDPIAware() -> BOOL;
    pub fn IsWinEventHookInstalled(event: DWORD) -> BOOL;
    pub fn LoadBitmapA(hInstance: HINSTANCE, lpBitmapName: LPCSTR) -> HBITMAP;
    pub fn LoadBitmapW(hInstance: HINSTANCE, lpBitmapName: LPCWSTR) -> HBITMAP;
    pub fn LoadCursorA(hInstance: HINSTANCE, lpCursorName: LPCSTR) -> HCURSOR;
    pub fn LoadCursorFromFileA(lpFileName: LPCSTR) -> HCURSOR;
    pub fn LoadCursorFromFileW(lpFileName: LPCWSTR) -> HCURSOR;
    pub fn LoadCursorW(hInstance: HINSTANCE, lpCursorName: LPCWSTR) -> HCURSOR;
    pub fn LoadImageA(
        hInst: HINSTANCE, name: LPCSTR, type_: UINT, cx: c_int, cy: c_int, fuLoad: UINT,
    ) -> HANDLE;
    pub fn LoadImageW(
        hInst: HINSTANCE, name: LPCWSTR, type_: UINT, cx: c_int, cy: c_int, fuLoad: UINT,
    ) -> HANDLE;
    pub fn LockWorkStation() -> BOOL;
    pub fn LookupIconIdFromDirectory(presbits: PBYTE, fIcon: BOOL) -> c_int;
    pub fn LookupIconIdFromDirectoryEx(
        presbits: PBYTE, fIcon: BOOL, cxDesired: c_int, cyDesired: c_int, Flags: UINT,
    ) -> c_int;
    pub fn MapDialogRect(hDlg: HWND, lpRect: LPRECT) -> BOOL;
    // pub fn MessageBoxTimeoutA();
    // pub fn MessageBoxTimeoutW();
    pub fn MonitorFromPoint(pt: POINT, dwFlags: DWORD) -> HMONITOR;
    pub fn MonitorFromRect(lprc: LPCRECT, dwFlags: DWORD) -> HMONITOR;
    pub fn MonitorFromWindow(hwnd: HWND, dwFlags: DWORD) -> HMONITOR;
    pub fn NotifyWinEvent(event: DWORD, hwnd: HWND, idObject: LONG, idChild: LONG);
    pub fn PackDDElParam(msg: UINT, uiLo: UINT_PTR, uiHi: UINT_PTR) -> LPARAM;
    // pub fn PrivateExtractIconsA();
    // pub fn PrivateExtractIconsW();
    // pub fn QueryDisplayConfig();
    pub fn RealChildWindowFromPoint(
        hwndParent: HWND, ptParentClientCoords: POINT,
    ) -> HWND;
    pub fn RealGetWindowClass(
        hwnd: HWND, ptszClassName: LPSTR, cchClassNameMax: UINT,
    ) -> UINT;
    pub fn RealGetWindowClassA(
        hwnd: HWND, ptszClassName: LPSTR, cchClassNameMax: UINT,
    ) -> UINT;
    pub fn RealGetWindowClassW(
        hwnd: HWND, ptszClassName: LPWSTR, cchClassNameMax: UINT,
    ) -> UINT;
    // pub fn RegisterPointerDeviceNotifications();
    pub fn RegisterRawInputDevices(
        pRawInputDevices: PCRAWINPUTDEVICE, uiNumDevices: UINT, cbSize: UINT,
    ) -> BOOL;
    // pub fn RegisterShellHookWindow();
    // pub fn ReuseDDElParam();
    // pub fn SendIMEMessageExA();
    // pub fn SendIMEMessageExW();
    // pub fn SetDebugErrorLevel();
    // pub fn SetDeskWallpaper();
    // pub fn SetDisplayAutoRotationPreferences();
    // pub fn SetDisplayConfig();
    // pub fn SetGestureConfig();
    pub fn SetLastErrorEx(dwErrCode: DWORD, dwType: DWORD);
    // pub fn SetMenuDefaultItem();
    pub fn SetParent(hWndChild: HWND, hWndNewParent: HWND) -> HWND;
    // pub fn SetProcessDPIAware();
    // pub fn SetProcessDefaultLayout();
    // pub fn SetProcessRestrictionExemption();
    pub fn SetScrollInfo(hwnd: HWND, nBar: c_int, lpsi: *const SCROLLINFO, redraw: BOOL) -> c_int;
    // pub fn SetShellWindow();
    pub fn SetSystemCursor(hcur: HCURSOR, id: DWORD) -> BOOL;
    pub fn SetWinEventHook(
        eventMin: DWORD, eventMax: DWORD, hmodWinEventProc: HMODULE, pfnWinEventProc: WINEVENTPROC,
        idProcess: DWORD, idThread: DWORD, dwFlags: DWORD,
    ) -> HWINEVENTHOOK;
    // pub fn SetWindowsHookA();
    pub fn SetWindowsHookExA(
        idHook: c_int, lpfn: HOOKPROC, hmod: HINSTANCE, dwThreadId: DWORD,
    ) -> HHOOK;
    pub fn SetWindowsHookExW(
        idHook: c_int, lpfn: HOOKPROC, hmod: HINSTANCE, dwThreadId: DWORD,
    ) -> HHOOK;
    // pub fn SetWindowsHookW();
    // pub fn ShowSystemCursor();
    // pub fn ShutdownBlockReasonCreate();
    // pub fn ShutdownBlockReasonDestroy();
    // pub fn ShutdownBlockReasonQuery();
    // pub fn SoundSentry();
    pub fn SystemParametersInfoA(
        uiAction: UINT, uiParam: UINT, pvParam: PVOID, fWinIni: UINT
    ) -> BOOL;
    pub fn SystemParametersInfoW(
        uiAction: UINT, uiParam: UINT, pvParam: PVOID, fWinIni: UINT
    ) -> BOOL;
    // pub fn TileChildWindows();
    // pub fn TileWindows();
    pub fn TrackPopupMenuEx(
        hMenu: HMENU, uFlags: UINT, x: INT, y: INT, hwnd: HWND, lptpm: LPTPMPARAMS,
    ) -> BOOL;
    // pub fn TranslateAccelerator();
    // pub fn TranslateMDISysAccel();
    pub fn UnhookWinEvent(hWinEventHook: HWINEVENTHOOK) -> BOOL;
    // pub fn UnhookWindowsHook();
    pub fn UnhookWindowsHookEx(hhk: HHOOK) -> BOOL;
    pub fn UnpackDDElParam(msg: UINT, lParam: LPARAM, puiLo: PUINT_PTR, puiHi: PUINT_PTR) -> BOOL;
    pub fn UserHandleGrantAccess(hUserHandle: HANDLE, hJob: HANDLE, bGrant: BOOL) -> BOOL;
    // pub fn WINNLSEnableIME();
    // pub fn WINNLSGetEnableStatus();
    // pub fn WINNLSGetIMEHotkey();
    pub fn WinHelpA(hWndMain: HWND, lpszHelp: LPCSTR, uCommand: UINT, dwData: ULONG_PTR) -> BOOL;
    pub fn WinHelpW(hWndMain: HWND, lpszHelp: LPCWSTR, uCommand: UINT, dwData: ULONG_PTR) -> BOOL;
    // pub fn wsprintfA();
    // pub fn wsprintfW();
}

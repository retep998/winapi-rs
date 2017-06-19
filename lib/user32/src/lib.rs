// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to user32.
#![cfg(windows)]
extern crate winapi;
use winapi::*;
extern "system" {
    pub fn AdjustWindowRect(lpRect: LPRECT, dwStyle: DWORD, bMenu: BOOL) -> BOOL;
    pub fn AdjustWindowRectEx(
        lpRect: LPRECT, dwStyle: DWORD, bMenu: BOOL, dwExStyle: DWORD,
    ) -> BOOL;
    pub fn ArrangeIconicWindows(hWnd: HWND) -> UINT;
    pub fn BeginPaint(hwnd: HWND, lpPaint: LPPAINTSTRUCT) -> HDC;
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
    pub fn ChildWindowFromPoint(hWndParent: HWND, point: POINT) -> HWND;
    pub fn ChildWindowFromPointEx(hwnd: HWND, pt: POINT, flags: UINT) -> HWND;
    pub fn ClientToScreen(hWnd: HWND, lpPoint: LPPOINT) -> BOOL;
    pub fn ClipCursor(lpRect: *const RECT) -> BOOL;
    // pub fn CloseGestureInfoHandle();
    pub fn CopyIcon(hIcon: HICON) -> HICON;
    pub fn CopyImage(h: HANDLE, type_: UINT, cx: c_int, cy: c_int, flags: UINT) -> HANDLE;
    pub fn CopyRect(lprcDst: LPRECT, lprcSrc: *const RECT) -> BOOL;
    pub fn CreateCaret(hWnd: HWND, hBitmap: HBITMAP, nWidth: c_int, nHeight: c_int) -> BOOL;
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
    pub fn DestroyCaret() -> BOOL;
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
    pub fn DrawFocusRect(hDC: HDC, lprc: *const RECT) -> BOOL;
    // pub fn DrawFrame();
    pub fn DrawIconEx(
        hdc: HDC, xLeft: c_int, yTop: c_int, hIcon: HICON, cxWidth: c_int, cyWidth: c_int,
        istepIfAniCur: UINT, hbrFlickerFreeDraw: HBRUSH, diFlags: UINT,
    ) -> BOOL;
    // pub fn EditWndProc();
    pub fn EnableScrollBar(hWnd: HWND, wSBflags: UINT, wArrows: UINT) -> BOOL;
    // pub fn EnableSessionForMMCSS();
    pub fn EndPaint(hWnd: HWND, lpPaint: *const PAINTSTRUCT) -> BOOL;
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
    pub fn EnumPropsA(hWnd: HWND, lpEnumFunc: PROPENUMPROCA) -> c_int;
    pub fn EnumPropsExA(hWnd: HWND, lpEnumFunc: PROPENUMPROCA, lParam: LPARAM) -> c_int;
    pub fn EnumPropsExW(hWnd: HWND, lpEnumFunc: PROPENUMPROCW, lParam: LPARAM) -> c_int;
    pub fn EnumPropsW(hWnd: HWND, lpEnumFunc: PROPENUMPROCW) -> c_int;
    pub fn EnumThreadWindows(dwThreadId: DWORD, lpfn: WNDENUMPROC, lParam: LPARAM) -> BOOL;
    pub fn EnumWindows(lpEnumFunc: WNDENUMPROC, lParam: LPARAM) -> BOOL;
    // pub fn EqualRect();
    pub fn ExcludeUpdateRgn(hDC: HDC, hWnd: HWND) -> c_int;
    pub fn FillRect(hDC: HDC, lprc: *const RECT, hbr: HBRUSH) -> c_int;
    pub fn FindWindowA (lpClassName: LPCSTR, lpWindowName: LPCSTR) -> HWND;
    pub fn FindWindowExA(
        hWndParent: HWND, hWndChildAfter: HWND, lpszClass: LPCSTR, lpszWindow: LPCSTR,
    ) -> HWND;
    pub fn FindWindowExW(
        hWndParent: HWND, hWndChildAfter: HWND, lpszClass: LPCWSTR, lpszWindow: LPCWSTR,
    ) -> HWND;
    pub fn FindWindowW(lpClassName: LPCWSTR, lpWindowName: LPCWSTR) -> HWND;
    pub fn FrameRect(hDC: HDC, lprc: *const RECT, hbr: HBRUSH) -> c_int;
    // pub fn FreeDDElParam();
    // pub fn GetAltTabInfo();
    // pub fn GetAltTabInfoA();
    // pub fn GetAltTabInfoW();
    pub fn GetAncestor(hWnd: HWND, gaFlags: UINT) -> HWND;
    // pub fn GetAutoRotationState();
    // pub fn GetCIMSSM();
    pub fn GetCaretBlinkTime() -> UINT;
    pub fn GetCaretPos(lpPoint: LPPOINT) -> BOOL;
    pub fn GetClassLongA(hWnd: HWND, nIndex: c_int) -> DWORD;
    #[cfg(target_arch = "x86_64")]
    pub fn GetClassLongPtrA(hWnd: HWND, nIndex: c_int) -> ULONG_PTR;
    #[cfg(target_arch = "x86_64")]
    pub fn GetClassLongPtrW(hWnd: HWND, nIndex: c_int) -> ULONG_PTR;
    pub fn GetClassLongW(hWnd: HWND, nIndex: c_int) -> DWORD;
    pub fn GetClassNameA(hWnd: HWND, lpClassName: LPCSTR, nMaxCount: c_int) -> c_int;
    pub fn GetClassNameW(hWnd: HWND, lpClassName: LPCWSTR, nMaxCount: c_int) -> c_int;
    pub fn GetClassWord(hWnd: HWND, nIndex: c_int) -> WORD;
    pub fn GetClientRect(hWnd: HWND, lpRect: LPRECT) -> BOOL;
    pub fn GetClipCursor(lpRect: LPRECT) -> BOOL;
    // pub fn GetComboBoxInfo();
    // pub fn GetCurrentInputMessageSource();
    pub fn GetCursor() -> HCURSOR;
    // pub fn GetCursorInfo();
    pub fn GetCursorPos(lpPoint: LPPOINT) -> BOOL;
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
    // pub fn GetMenuContextHelpId();
    pub fn GetMonitorInfoA(hMonitor: HMONITOR, lpmi: LPMONITORINFO) -> BOOL;
    pub fn GetMonitorInfoW(hMonitor: HMONITOR, lpmi: LPMONITORINFO) -> BOOL;
    pub fn GetParent(hWnd: HWND) -> HWND;
    pub fn GetPhysicalCursorPos(lpPoint: LPPOINT) -> BOOL;
    // pub fn GetPointerDevice();
    // pub fn GetPointerDeviceCursors();
    // pub fn GetPointerDeviceProperties();
    // pub fn GetPointerDeviceRects();
    // pub fn GetPointerDevices();
    // pub fn GetProcessDefaultLayout();
    pub fn GetPropA(hwnd: HWND, lpString: LPCSTR) -> HANDLE;
    pub fn GetPropW(hwnd: HWND, lpString: LPCWSTR) -> HANDLE;
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
    pub fn GetScrollPos(hWnd: HWND, nBar: c_int) -> c_int;
    pub fn GetScrollRange(hWnd: HWND, nBar: c_int, lpMinPos: LPINT, lpMaxPos: LPINT) -> BOOL;
    pub fn GetShellWindow() -> HWND;
    pub fn GetSysColor(nIndex: c_int) -> DWORD;
    pub fn GetSysColorBrush(nIndex: c_int) -> HBRUSH;
    // pub fn GetTitleBarInfo();
    pub fn GetTopWindow(hWnd: HWND) -> HWND;
    pub fn GetUpdateRect(hWnd: HWND, lpRect: LPRECT, bErase: BOOL) -> BOOL;
    pub fn GetUpdateRgn(hWnd: HWND, hRgn: HRGN, bErase: BOOL) -> c_int;
    pub fn GetWindow(hWnd: HWND, uCmd: UINT) -> HWND;
    pub fn GetWindowContextHelpId(h: HWND) -> DWORD;
    pub fn GetWindowDC(hWnd: HWND) -> HDC;
    // pub fn GetWindowInfo();
    pub fn GetWindowLongA(hWnd: HWND, nIndex: c_int) -> LONG;
    #[cfg(target_arch = "x86_64")]
    pub fn GetWindowLongPtrA(hWnd: HWND, nIndex: c_int) -> LONG_PTR;
    #[cfg(target_arch = "x86_64")]
    pub fn GetWindowLongPtrW(hWnd: HWND, nIndex: c_int) -> LONG_PTR;
    pub fn GetWindowLongW(hWnd: HWND, nIndex: c_int) -> LONG;
    // pub fn GetWindowModuleFileName();
    pub fn GetWindowModuleFileNameA(
        hWnd: HWND, lpszFileName: LPCSTR, cchFileNameMax: UINT,
    ) -> UINT;
    pub fn GetWindowModuleFileNameW(
        hWnd: HWND, lpszFileName: LPWSTR, cchFileNameMax: UINT,
    ) -> UINT;
    pub fn GetWindowRect(hWnd: HWND, lpRect: LPRECT) -> BOOL;
    pub fn GetWindowRgn(hWnd: HWND, hRgn: HRGN) -> c_int;
    pub fn GetWindowRgnBox(hWnd: HWND, lprc: LPRECT) -> c_int;
    pub fn GetWindowTextA(hWnd: HWND, lpString: LPSTR, nMaxCount: c_int) -> c_int;
    pub fn GetWindowTextLengthA(hWnd: HWND) -> c_int;
    pub fn GetWindowTextLengthW(hWnd: HWND) -> c_int;
    pub fn GetWindowTextW(hWnd: HWND, lpString: LPWSTR, nMaxCount: c_int) -> c_int;
    pub fn GetWindowThreadProcessId(hWnd: HWND, lpdwProcessId: LPDWORD) -> DWORD;
    pub fn GetWindowWord(hWnd: HWND,nIndex: c_int) -> WORD;
    pub fn HideCaret(hWnd: HWND) -> BOOL;
    // pub fn IMPGetIMEA();
    // pub fn IMPGetIMEW();
    // pub fn IMPQueryIMEA();
    // pub fn IMPQueryIMEW();
    // pub fn IMPSetIMEA();
    // pub fn IMPSetIMEW();
    // pub fn ImpersonateDdeClientWindow();
    pub fn InflateRect(lprc: LPRECT, dx: c_int, dy: c_int) -> BOOL;
    pub fn InternalGetWindowText(hWnd: HWND, pString: LPWSTR, cchMaxCount: c_int) -> c_int;
    pub fn IntersectRect(
        lprcDst: LPRECT, lprcSrc1: *const RECT, lprcSrc2: *const RECT,
    ) -> BOOL;
    pub fn InvalidateRect(hWnd: HWND, lpRect: *const RECT, bErase: BOOL) -> BOOL;
    pub fn InvalidateRgn(hWnd: HWND, hRgn: HRGN, bErase: BOOL) -> BOOL;
    pub fn InvertRect(hDC: HDC, lprc: *const RECT) -> BOOL;
    pub fn IsDialogMessageA(hDlg: HWND, lpMsg: LPMSG) -> BOOL;
    pub fn IsDialogMessageW(hDlg: HWND, lpMsg: LPMSG) -> BOOL;
    pub fn IsGUIThread(bConvert: BOOL) -> BOOL;
    pub fn IsImmersiveProcess(hProcess: HANDLE) -> BOOL;
    // pub fn IsInDesktopWindowBand();
    pub fn IsProcessDPIAware() -> BOOL;
    pub fn IsRectEmpty(lprc: *const RECT) -> BOOL;
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
    pub fn LockWindowUpdate(hWndLock: HWND) -> BOOL;
    pub fn LockWorkStation() -> BOOL;
    // pub fn LogicalToPhysicalPoint();
    // pub fn LogicalToPhysicalPointForPerMonitorDPI();
    pub fn LookupIconIdFromDirectory(presbits: PBYTE, fIcon: BOOL) -> c_int;
    pub fn LookupIconIdFromDirectoryEx(
        presbits: PBYTE, fIcon: BOOL, cxDesired: c_int, cyDesired: c_int, Flags: UINT,
    ) -> c_int;
    pub fn MapDialogRect(hDlg: HWND, lpRect: LPRECT) -> BOOL;
    pub fn MapWindowPoints(hWndFrom: HWND, hWndTo: HWND, lpPoints: LPPOINT, cPoints: UINT) -> c_int;
    pub fn MessageBeep(uType: UINT) -> BOOL;
    pub fn MessageBoxExA(
        hWnd: HWND, lpText: LPCSTR, lpCaption: LPCSTR, uType: UINT, wLanguageId: WORD,
    ) -> c_int;
    pub fn MessageBoxExW(
        hWnd: HWND, lpText: LPCWSTR, lpCaption: LPCWSTR, uType: UINT, wLanguageId: WORD,
    ) -> c_int;
    pub fn MessageBoxIndirectA(lpmbp: *const MSGBOXPARAMSA) -> c_int;
    pub fn MessageBoxIndirectW(lpmbp: *const MSGBOXPARAMSW) -> c_int;
    // pub fn MessageBoxTimeoutA();
    // pub fn MessageBoxTimeoutW();
    pub fn MonitorFromPoint(pt: POINT, dwFlags: DWORD) -> HMONITOR;
    pub fn MonitorFromRect(lprc: LPCRECT, dwFlags: DWORD) -> HMONITOR;
    pub fn MonitorFromWindow(hwnd: HWND, dwFlags: DWORD) -> HMONITOR;
    pub fn NotifyWinEvent(event: DWORD, hwnd: HWND, idObject: LONG, idChild: LONG);
    pub fn OffsetRect(lprc: LPRECT, dx: c_int, dy: c_int) -> BOOL;
    pub fn PackDDElParam(msg: UINT, uiLo: UINT_PTR, uiHi: UINT_PTR) -> LPARAM;
    // pub fn PhysicalToLogicalPoint();
    // pub fn PhysicalToLogicalPointForPerMonitorDPI();
    // pub fn PrivateExtractIconsA();
    // pub fn PrivateExtractIconsW();
    pub fn PtInRect(lprc: *const RECT, pt: POINT) -> BOOL;
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
    pub fn RedrawWindow(
        hwnd: HWND, lprcUpdate: *const RECT, hrgnUpdate: HRGN, flags: UINT,
    ) -> BOOL;
    // pub fn RegisterPointerDeviceNotifications();
    pub fn RegisterRawInputDevices(
        pRawInputDevices: PCRAWINPUTDEVICE, uiNumDevices: UINT, cbSize: UINT,
    ) -> BOOL;
    // pub fn RegisterShellHookWindow();
    pub fn ReleaseDC(hWnd: HWND, hDC: HDC) -> c_int;
    pub fn RemovePropA(hWnd: HWND, lpStr: LPCSTR) -> HANDLE;
    pub fn RemovePropW(hWnd: HWND, lpStr: LPCWSTR) -> HANDLE;
    // pub fn ReuseDDElParam();
    pub fn ScreenToClient(hWnd: HWND, lpPoint: LPPOINT) -> BOOL;
    pub fn ScrollDC(
        hDC: HDC, dx: c_int, dy: c_int, lprcScroll: *const RECT, lprcClip: *const RECT,
        hrgnUpdate: HRGN, lprcUpdate: LPRECT,
    ) -> BOOL;
    pub fn ScrollWindow(
        hWnd: HWND, xAmount: c_int, yAmount: c_int, lpRect: *const RECT, lpClipRect: *const RECT,
    ) -> BOOL;
    pub fn ScrollWindowEx(
        hWnd: HWND, dx: c_int, dy: c_int, prcScroll: *const RECT, prcClip: *const RECT,
        hrgnUpdate: HRGN, prcUpdate: LPRECT, flags: UINT,
    ) -> c_int;
    // pub fn SendIMEMessageExA();
    // pub fn SendIMEMessageExW();
    pub fn SetCaretBlinkTime(uMSeconds: UINT) -> BOOL;
    pub fn SetCaretPos(x: c_int, y: c_int) -> BOOL;
    pub fn SetClassLongA(hWnd: HWND, nIndex: c_int, dwNewLong: LONG) -> DWORD;
    #[cfg(target_arch = "x86_64")]
    pub fn SetClassLongPtrA(hWnd: HWND, nIndex: c_int, dwNewLong: LONG_PTR) -> ULONG_PTR;
    #[cfg(target_arch = "x86_64")]
    pub fn SetClassLongPtrW(hWnd: HWND, nIndex: c_int, dwNewLong: LONG_PTR) -> ULONG_PTR;
    pub fn SetClassLongW(hWnd: HWND, nIndex: c_int, dwNewLong: LONG) -> DWORD;
    pub fn SetClassWord(hWnd: HWND, nIndex: c_int, wNewWord: WORD) -> WORD;
    pub fn SetCursor(hCursor: HCURSOR) -> HCURSOR;
    pub fn SetCursorPos(x: c_int, y: c_int) -> BOOL;
    // pub fn SetDebugErrorLevel();
    // pub fn SetDeskWallpaper();
    // pub fn SetDisplayAutoRotationPreferences();
    // pub fn SetDisplayConfig();
    // pub fn SetGestureConfig();
    pub fn SetLastErrorEx(dwErrCode: DWORD, dwType: DWORD);
    // pub fn SetMenuContextHelpId();
    // pub fn SetMenuDefaultItem();
    pub fn SetParent(hWndChild: HWND, hWndNewParent: HWND) -> HWND;
    pub fn SetPhysicalCursorPos(x: c_int, y: c_int) -> BOOL;
    // pub fn SetProcessDPIAware();
    // pub fn SetProcessDefaultLayout();
    // pub fn SetProcessRestrictionExemption();
    pub fn SetPropA(hWnd: HWND, lpString: LPCSTR, hData: HANDLE) -> BOOL;
    pub fn SetPropW(hWnd: HWND, lpString: LPCWSTR, hData: HANDLE) -> BOOL;
    pub fn SetRect(lprc: LPRECT, xLeft: c_int, yTop: c_int, xRight: c_int, yBottom: c_int) -> BOOL;
    pub fn SetRectEmpty(lprc: LPRECT) -> BOOL;
    pub fn SetScrollInfo(hwnd: HWND, nBar: c_int, lpsi: *const SCROLLINFO, redraw: BOOL) -> c_int;
    pub fn SetScrollPos(hWnd: HWND, nBar: c_int, nPos: c_int, bRedraw: BOOL) -> c_int;
    pub fn SetScrollRange(
        hWnd: HWND, nBar: c_int, nMinPos: c_int, nMaxPos: c_int, bRedraw: BOOL,
    ) -> BOOL;
    // pub fn SetShellWindow();
    pub fn SetSysColors(
        cElements: c_int, lpaElements: *const INT, lpaRgbValues: *const COLORREF,
    ) -> BOOL;
    pub fn SetSystemCursor(hcur: HCURSOR, id: DWORD) -> BOOL;
    pub fn SetWinEventHook(
        eventMin: DWORD, eventMax: DWORD, hmodWinEventProc: HMODULE, pfnWinEventProc: WINEVENTPROC,
        idProcess: DWORD, idThread: DWORD, dwFlags: DWORD,
    ) -> HWINEVENTHOOK;
    pub fn SetWindowContextHelpId(h: HWND, d: DWORD) -> BOOL;
    pub fn SetWindowLongA(hWnd: HWND, nIndex: c_int, dwNewLong: LONG) -> LONG;
    #[cfg(target_arch = "x86_64")]
    pub fn SetWindowLongPtrA(hWnd: HWND, nIndex: c_int, dwNewLong: LONG_PTR) -> LONG_PTR;
    #[cfg(target_arch = "x86_64")]
    pub fn SetWindowLongPtrW(hWnd: HWND, nIndex: c_int, dwNewLong: LONG_PTR) -> LONG_PTR;
    pub fn SetWindowLongW(hWnd: HWND, nIndex: c_int, dwNewLong: LONG) -> LONG;
    pub fn SetWindowRgn(hWnd: HWND, hRgn: HRGN, bRedraw: BOOL) -> c_int;
    pub fn SetWindowTextA(hWnd: HWND, lpString: LPCSTR) -> BOOL;
    pub fn SetWindowTextW(hWnd: HWND, lpString: LPCWSTR) -> BOOL;
    pub fn SetWindowWord(hwnd: HWND, nIndex: c_int, wNewWord: WORD) -> WORD;
    // pub fn SetWindowsHookA();
    pub fn SetWindowsHookExA(
        idHook: c_int, lpfn: HOOKPROC, hmod: HINSTANCE, dwThreadId: DWORD,
    ) -> HHOOK;
    pub fn SetWindowsHookExW(
        idHook: c_int, lpfn: HOOKPROC, hmod: HINSTANCE, dwThreadId: DWORD,
    ) -> HHOOK;
    // pub fn SetWindowsHookW();
    pub fn ShowCaret(hWnd: HWND) -> BOOL;
    pub fn ShowCursor(bShow: BOOL) -> c_int;
    pub fn ShowScrollBar(hWnd: HWND, wBar: c_int, bShow: BOOL) -> BOOL;
    // pub fn ShowSystemCursor();
    // pub fn ShutdownBlockReasonCreate();
    // pub fn ShutdownBlockReasonDestroy();
    // pub fn ShutdownBlockReasonQuery();
    // pub fn SoundSentry();
    // pub fn SubtractRect();
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
    pub fn UnionRect(lprcDst: LPRECT, lprcSrc1: *const RECT, lprcSrc2: *const RECT) -> BOOL;
    pub fn UnpackDDElParam(msg: UINT, lParam: LPARAM, puiLo: PUINT_PTR, puiHi: PUINT_PTR) -> BOOL;
    pub fn UserHandleGrantAccess(hUserHandle: HANDLE, hJob: HANDLE, bGrant: BOOL) -> BOOL;
    pub fn ValidateRect(hWnd: HWND, lpRect: *const RECT) -> BOOL;
    pub fn ValidateRgn(hWnd: HWND, hRgn: HRGN) -> BOOL;
    // pub fn WINNLSEnableIME();
    // pub fn WINNLSGetEnableStatus();
    // pub fn WINNLSGetIMEHotkey();
    pub fn WinHelpA(hWndMain: HWND, lpszHelp: LPCSTR, uCommand: UINT, dwData: ULONG_PTR) -> BOOL;
    pub fn WinHelpW(hWndMain: HWND, lpszHelp: LPCWSTR, uCommand: UINT, dwData: ULONG_PTR) -> BOOL;
    // pub fn WindowFromPhysicalPoint();
    pub fn WindowFromPoint(Point: POINT) -> HWND;
    // pub fn wsprintfA();
    // pub fn wsprintfW();
}

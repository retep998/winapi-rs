// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to user32.
#![cfg(windows)]
extern crate winapi;
use winapi::*;
extern "system" {
    pub fn ActivateKeyboardLayout(hkl: HKL, flags: UINT) -> HKL;
    pub fn AddClipboardFormatListener(hWnd: HWND) -> BOOL;
    pub fn AdjustWindowRect(lpRect: LPRECT, dwStyle: DWORD, bMenu: BOOL) -> BOOL;
    pub fn AdjustWindowRectEx(
        lpRect: LPRECT, dwStyle: DWORD, bMenu: BOOL, dwExStyle: DWORD,
    ) -> BOOL;
    pub fn AllowSetForegroundWindow(dwProcessId: DWORD) -> BOOL;
    pub fn AnimateWindow(hWnd: HWND, dwTime: DWORD, dwFlags: DWORD) -> BOOL;
    pub fn AnyPopup() -> BOOL;
    pub fn AppendMenuA(
        hMenu: HMENU, uFlags: UINT, uIDNewItem: UINT_PTR, lpNewItem: LPCSTR,
    ) -> BOOL;
    pub fn AppendMenuW(
        hMenu: HMENU, uFlags: UINT, uIDNewItem: UINT_PTR, lpNewItem: LPCWSTR,
    ) -> BOOL;
    pub fn ArrangeIconicWindows(hWnd: HWND) -> UINT;
    pub fn AttachThreadInput(idAttach: DWORD, idAttachTo: DWORD, fAttach: BOOL) -> BOOL;
    // pub fn BeginDeferWindowPos();
    pub fn BeginPaint(hwnd: HWND, lpPaint: LPPAINTSTRUCT) -> HDC;
    pub fn BlockInput(fBlockIt: BOOL) -> BOOL;
    pub fn BringWindowToTop(hWnd: HWND) -> BOOL;
    // pub fn BroadcastSystemMessage();
    pub fn BroadcastSystemMessageA(
        flags: DWORD, lpInfo: LPDWORD, Msg: UINT, wParam: WPARAM, lParam: LPARAM,
    ) -> LONG;
    // pub fn BroadcastSystemMessageExA();
    // pub fn BroadcastSystemMessageExW();
    pub fn BroadcastSystemMessageW(
        flags: DWORD, lpInfo: LPDWORD, Msg: UINT, wParam: WPARAM, lParam: LPARAM,
    ) -> LONG;
    pub fn CalculatePopupWindowPosition(
        anchorPoint: *const POINT, windowSize: *const SIZE, flags: UINT, excludeRect: *mut RECT,
        popupWindowPosition: *mut RECT,
    ) -> BOOL;
    // pub fn CallMsgFilter();
    pub fn CallMsgFilterA(lpMsg: LPMSG, nCode: c_int) -> BOOL;
    pub fn CallMsgFilterW(lpMsg: LPMSG, nCode: c_int) -> BOOL;
    // pub fn CallNextHookEx();
    // pub fn CallWindowProcA();
    // pub fn CallWindowProcW();
    pub fn CancelShutdown() -> BOOL;
    // pub fn CascadeChildWindows();
    pub fn CascadeWindows(
        hwndParent: HWND, wHow: UINT, lpRect: *const RECT, cKids: UINT, lpKids: *const HWND,
    ) -> WORD;
    pub fn ChangeClipboardChain(hwndRemove: HWND, hwndNewNext: HWND) -> BOOL;
    // pub fn ChangeDisplaySettingsA();
    // pub fn ChangeDisplaySettingsExA();
    pub fn ChangeDisplaySettingsExW(
        lpszDeviceName: LPCWSTR, lpDevMode: *mut DEVMODEW, hwnd: HWND, dwFlags: DWORD,
        lParam: LPVOID,
    ) -> LONG;
    pub fn ChangeDisplaySettingsW(lpDevMode: *mut DEVMODEW, dwFlags: DWORD) -> LONG;
    pub fn ChangeMenuA(
        hMenu: HMENU, cmd: UINT, lpszNewItem: LPCSTR, cmdInsert: UINT, flags: UINT,
    ) -> BOOL;
    pub fn ChangeMenuW(
        hMenu: HMENU, cmd: UINT, lpszNewItem: LPCWSTR, cmdInsert: UINT, flags: UINT,
    ) -> BOOL;
    // pub fn ChangeWindowMessageFilter();
    // pub fn ChangeWindowMessageFilterEx();
    pub fn CharLowerA(lpsz: LPSTR) -> LPSTR;
    // pub fn CharLowerBuffA();
    // pub fn CharLowerBuffW();
    pub fn CharLowerW(lpsz: LPWSTR) -> LPWSTR;
    pub fn CharNextA(lpsz: LPCSTR) -> LPSTR;
    pub fn CharNextExA(codePage: WORD, lpCurrentChar: LPSTR, dwFlags: DWORD) -> LPSTR;
    pub fn CharNextW(lpsz: LPCWSTR) -> LPWSTR;
    pub fn CharPrevA(lpszStart: LPCSTR, lpszCurrent: LPCSTR) -> LPSTR;
    pub fn CharPrevExA(
        codePage: WORD, lpStart: LPCSTR, lpCurrentChar: LPCSTR, dwFlags: DWORD,
    ) -> LPSTR;
    pub fn CharPrevW(lpszStart: LPCWSTR, lpszCurrent: LPCWSTR) -> LPWSTR;
    // pub fn CharToOemA();
    // pub fn CharToOemBuffA();
    // pub fn CharToOemBuffW();
    // pub fn CharToOemW();
    pub fn CharUpperA(lpsz: LPSTR) -> LPSTR;
    // pub fn CharUpperBuffA();
    // pub fn CharUpperBuffW();
    pub fn CharUpperW(lpsz: LPWSTR) -> LPWSTR;
    // pub fn CheckDlgButton();
    pub fn CheckMenuItem(hMenu: HMENU, uIDCheckItem: UINT, uCheck: UINT) -> DWORD;
    pub fn CheckMenuRadioItem(
        hMenu: HMENU, first: UINT, last: UINT, check: UINT, flags: UINT,
    ) -> BOOL;
    pub fn CheckRadioButton(
        hDlg: HWND, nIDFirstButton: c_int, nIDLasatButton: c_int, nIDCheckButton: c_int,
    ) -> BOOL;
    pub fn ChildWindowFromPoint(hWndParent: HWND, point: POINT) -> HWND;
    pub fn ChildWindowFromPointEx(hwnd: HWND, pt: POINT, flags: UINT) -> HWND;
    pub fn ClientToScreen(hWnd: HWND, lpPoint: LPPOINT) -> BOOL;
    pub fn ClipCursor(lpRect: *const RECT) -> BOOL;
    pub fn CloseClipboard() -> BOOL;
    pub fn CloseDesktop(hDesktop: HDESK) -> BOOL;
    // pub fn CloseGestureInfoHandle();
    // pub fn CloseTouchInputHandle();
    pub fn CloseWindow(hWnd: HWND) -> BOOL;
    pub fn CloseWindowStation(hWinSta: HWINSTA) -> BOOL;
    // pub fn CopyAcceleratorTableA();
    // pub fn CopyAcceleratorTableW();
    pub fn CopyIcon(hIcon: HICON) -> HICON;
    pub fn CopyImage(h: HANDLE, type_: UINT, cx: c_int, cy: c_int, flags: UINT) -> HANDLE;
    // pub fn CopyRect();
    pub fn CountClipboardFormats() -> c_int;
    // pub fn CreateAcceleratorTableA();
    // pub fn CreateAcceleratorTableW();
    pub fn CreateCaret(hWnd: HWND, hBitmap: HBITMAP, nWidth: c_int, nHeight: c_int) -> BOOL;
    pub fn CreateCursor(
        hInst: HINSTANCE, xHotSpot: c_int, yHotSpot: c_int, nWidth: c_int, nHeight: c_int,
        pvAndPlane: *const VOID, pvXORPlane: *const VOID,
    ) -> HCURSOR;
    // pub fn CreateDesktopA();
    // pub fn CreateDesktopExA();
    // pub fn CreateDesktopExW();
    // pub fn CreateDesktopW();
    // pub fn CreateDialogIndirectParamA();
    // pub fn CreateDialogIndirectParamW();
    // pub fn CreateDialogParamA();
    // pub fn CreateDialogParamW();
    pub fn CreateIcon(
        hInstance: HINSTANCE, nWidth: c_int, nHeight: c_int, cPlanes: BYTE, cBitsPixel: BYTE,
        lpbANDbits: *const BYTE, lpbXORbits: *const BYTE,
    ) -> HICON;
    // pub fn CreateIconFromResource();
    // pub fn CreateIconFromResourceEx();
    // pub fn CreateIconIndirect(piconinfo: PICONINFO) -> HICON;
    // pub fn CreateMDIWindowA();
    // pub fn CreateMDIWindowW();
    pub fn CreateMenu() -> HMENU;
    pub fn CreatePopupMenu() ->HMENU;
    // pub fn CreateWindowExA();
    pub fn CreateWindowExW(
        dwExStyle: DWORD, lpClassName: LPCWSTR, lpWindowName: LPCWSTR, dwStyle: DWORD, x: c_int,
        y: c_int, nWidth: c_int, nHeight: c_int, hWndParent: HWND, hMenu: HMENU,
        hInstance: HINSTANCE, lpParam: LPVOID,
    ) -> HWND;
    // pub fn CreateWindowStationA();
    // pub fn CreateWindowStationW();
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
    pub fn DefDlgProcA(
        hDlg: HWND, msg: UINT, wParam: WPARAM, lParam: LPARAM,
    ) -> LRESULT;
    pub fn DefDlgProcW(
        hDlg: HWND, msg: UINT, wParam: WPARAM, lParam: LPARAM,
    ) -> LRESULT;
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
    // pub fn DefWindowProcA();
    pub fn DefWindowProcW(hWnd: HWND, Msg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT;
    pub fn DeferWindowPos(
        hWinPosInfo: HDWP, hWnd: HWND, hWndInserAfter: HWND, x: c_int, y: c_int, cx: c_int,
        cy: c_int, uFlags: UINT,
    ) -> HDWP;
    pub fn DeleteMenu(hMenu: HMENU, uPosition: UINT, uFlags: UINT) -> BOOL;
    pub fn DeregisterShellHookWindow(hwnd: HWND) -> BOOL;
    pub fn DestroyAcceleratorTable(hAccel: HACCEL) -> BOOL;
    pub fn DestroyCaret() -> BOOL;
    pub fn DestroyCursor(hCursor: HCURSOR) -> BOOL;
    pub fn DestroyIcon(hIcon: HICON) -> BOOL;
    pub fn DestroyMenu(hMenu: HMENU) -> BOOL;
    pub fn DestroyWindow(hWnd: HWND) -> BOOL;
    // pub fn DialogBoxIndirectParamA();
    // pub fn DialogBoxIndirectParamW();
    // pub fn DialogBoxParamA();
    // pub fn DialogBoxParamW();
    // pub fn DisableProcessWindowsGhosting();
    // pub fn DispatchMessageA();
    pub fn DispatchMessageW(lpmsg: *const MSG) -> LRESULT;
    // pub fn DisplayConfigGetDeviceInfo();
    // pub fn DisplayConfigSetDeviceInfo();
    // pub fn DlgDirListA();
    // pub fn DlgDirListComboBoxA();
    // pub fn DlgDirListComboBoxW();
    // pub fn DlgDirListW();
    // pub fn DlgDirSelectComboBoxExA();
    // pub fn DlgDirSelectComboBoxExW();
    // pub fn DlgDirSelectExA();
    // pub fn DlgDirSelectExW();
    pub fn DragDetect(hwnd: HWND, pt: POINT) -> BOOL;
    pub fn DragObject(
        hwndParent: HWND, hwndFrom: HWND, fmt: UINT, data: ULONG_PTR, hcur: HCURSOR,
    ) -> DWORD;
    pub fn DrawAnimatedRects(
        hwnd: HWND, idAni: c_int, lprcFrom: *const RECT, lprcTo: *const RECT,
    ) -> BOOL;
    pub fn DrawCaption(hwnd: HWND, hdc: HDC, lprect: *const RECT, flags: UINT) -> BOOL;
    pub fn DrawEdge(hdc: HDC, qrc: LPRECT, edge: UINT, grfFlags: UINT) -> BOOL;
    pub fn DrawFocusRect(hDC: HDC, lprc: *const RECT) -> BOOL;
    // pub fn DrawFrame();
    pub fn DrawFrameControl(hdc: HDC, lprc: LPRECT, uType: UINT, uState: UINT) -> BOOL;
    pub fn DrawIcon(hDC: HDC, x: c_int, y: c_int, hIcon: HICON) -> BOOL;
    pub fn DrawIconEx(
        hdc: HDC, xLeft: c_int, yTop: c_int, hIcon: HICON, cxWidth: c_int, cyWidth: c_int,
        istepIfAniCur: UINT, hbrFlickerFreeDraw: HBRUSH, diFlags: UINT,
    ) -> BOOL;
    pub fn DrawMenuBar(hwnd: HWND) -> BOOL;
    // pub fn DrawStateA();
    // pub fn DrawStateW();
    // pub fn DrawTextA();
    // pub fn DrawTextExA();
    // pub fn DrawTextExW();
    // pub fn DrawTextW();
    // pub fn EditWndProc();
    pub fn EmptyClipboard() -> BOOL;
    // pub fn EnableMenuItem();
    pub fn EnableMouseInPointer(fEnable: BOOL) -> BOOL;
    pub fn EnableScrollBar(hWnd: HWND, wSBflags: UINT, wArrows: UINT) -> BOOL;
    // pub fn EnableSessionForMMCSS();
    pub fn EnableWindow(hWnd: HWND, bEnable: BOOL) -> BOOL;
    // pub fn EndDeferWindowPos();
    // pub fn EndDialog();
    pub fn EndMenu(hMenu: HMENU, uFlags: UINT, uIDNewItem: UINT_PTR, lpNewItem: LPCSTR) -> BOOL;
    pub fn EndPaint(hWnd: HWND, lpPaint: *const PAINTSTRUCT) -> BOOL;
    pub fn EndTask(hWnd: HWND, fShutDown: BOOL, fForce: BOOL) -> BOOL;
    pub fn EnumChildWindows(
        hwndParent: HWND, lpEnumFunc: WNDENUMPROC, lpParam: LPARAM,
    ) -> BOOL;
    pub fn EnumClipboardFormats(format: UINT) -> UINT;
    pub fn EnumDesktopWindows(hDesktop: HDESK, lpfn: WNDENUMPROC, lParam: LPARAM) -> BOOL;
    pub fn EnumDesktopsA(
        hwinsta: HWINSTA, lpEnumFunc: DESKTOPENUMPROCA, lParam: LPARAM,
    ) -> BOOL;
    pub fn EnumDesktopsW(
        hwinsta: HWINSTA, lpEnumFunc: DESKTOPENUMPROCW, lParam: LPARAM,
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
    // pub fn EnumDisplaySettingsExA();
    pub fn EnumDisplaySettingsExW(
        lpszDeviceName: LPCWSTR, iModeNum: DWORD, lpDevMode: *mut DEVMODEW, dwFlags: DWORD,
    ) -> BOOL;
    pub fn EnumDisplaySettingsW(
        lpszDeviceName: LPCWSTR, iModeNum: DWORD, lpDevMode: *mut DEVMODEW,
    ) -> BOOL;
    // pub fn EnumPropsA();
    // pub fn EnumPropsExA();
    // pub fn EnumPropsExW();
    // pub fn EnumPropsW();
    pub fn EnumThreadWindows(dwThreadId: DWORD, lpfn: WNDENUMPROC, lParam: LPARAM) -> BOOL;
    // pub fn EnumWindowStationsA();
    // pub fn EnumWindowStationsW();
    pub fn EnumWindows(lpEnumFunc: WNDENUMPROC, lParam: LPARAM) -> BOOL;
    // pub fn EqualRect();
    // pub fn EvaluateProximityToPolygon();
    // pub fn EvaluateProximityToRect();
    // pub fn ExcludeUpdateRgn();
    // pub fn ExitWindowsEx();
    pub fn FillRect(hDC: HDC, lprc: *const RECT, hbr: HBRUSH) -> c_int;
    pub fn FindWindowA (lpClassName: LPCSTR, lpWindowName: LPCSTR) -> HWND;
    pub fn FindWindowExA(
        hWndParent: HWND, hWndChildAfter: HWND, lpszClass: LPCSTR, lpszWindow: LPCSTR,
    ) -> HWND;
    pub fn FindWindowExW(
        hWndParent: HWND, hWndChildAfter: HWND, lpszClass: LPCWSTR, lpszWindow: LPCWSTR,
    ) -> HWND;
    pub fn FindWindowW(lpClassName: LPCWSTR, lpWindowName: LPCWSTR) -> HWND;
    pub fn FlashWindow(hwnd: HWND, bInvert: BOOL) -> BOOL;
    // pub fn FlashWindowEx();
    pub fn FrameRect(hDC: HDC, lprc: *const RECT, hbr: HBRUSH) -> c_int;
    // pub fn FreeDDElParam();
    pub fn GetActiveWindow() -> HWND;
    // pub fn GetAltTabInfo();
    // pub fn GetAltTabInfoA();
    // pub fn GetAltTabInfoW();
    pub fn GetAncestor(hWnd: HWND, gaFlags: UINT) -> HWND;
    pub fn GetAsyncKeyState(vKey: c_int) -> SHORT;
    // pub fn GetAutoRotationState();
    // pub fn GetCIMSSM();
    pub fn GetCapture() -> HWND;
    pub fn GetCaretBlinkTime() -> UINT;
    pub fn GetCaretPos(lpPoint: LPPOINT) -> BOOL;
    // pub fn GetClassInfoA();
    // pub fn GetClassInfoExA();
    pub fn GetClassInfoExW(
        hinst: HINSTANCE, lpszClass: LPCWSTR, lpwcx: LPWNDCLASSEXW,
    ) -> BOOL;
    pub fn GetClassInfoW(
        hInstance: HINSTANCE, lpClassName: LPCWSTR, lpWndClass: LPWNDCLASSW,
    ) -> BOOL;
    pub fn GetClassLongA(hWnd: HWND, nIndex: c_int) -> DWORD;
    #[cfg(target_arch = "x86_64")]
    pub fn GetClassLongPtrA(hWnd: HWND, nIndex: c_int) -> ULONG_PTR;
    #[cfg(target_arch = "x86_64")]
    pub fn GetClassLongPtrW(hWnd: HWND, nIndex: c_int) -> ULONG_PTR;
    pub fn GetClassLongW(hWnd: HWND, nIndex: c_int) -> DWORD;
    // pub fn GetClassNameA();
    // pub fn GetClassNameW();
    pub fn GetClassWord(hWnd: HWND, nIndex: c_int) -> WORD;
    pub fn GetClientRect(hWnd: HWND, lpRect: LPRECT) -> BOOL;
    pub fn GetClipCursor(lpRect: LPRECT) -> BOOL;
    pub fn GetClipboardData(uFormat: UINT) -> HANDLE;
    pub fn GetClipboardFormatNameA(
        format: UINT, lpszFormatName: LPSTR, cchMaxCount: c_int,
    ) -> c_int;
    pub fn GetClipboardFormatNameW(
        format: UINT, lpszFormatName: LPWSTR, cchMaxCount: c_int,
    ) -> c_int;
    pub fn GetClipboardOwner() -> HWND;
    pub fn GetClipboardSequenceNumber() -> DWORD;
    pub fn GetClipboardViewer() -> HWND;
    // pub fn GetComboBoxInfo();
    // pub fn GetCurrentInputMessageSource();
    pub fn GetCursor() -> HCURSOR;
    // pub fn GetCursorInfo();
    pub fn GetCursorPos(lpPoint: LPPOINT) -> BOOL;
    pub fn GetDC(hWnd: HWND) -> HDC;
    // pub fn GetDCEx();
    pub fn GetDesktopWindow() -> HWND;
    pub fn GetDialogBaseUnits() -> LONG;
    // pub fn GetDisplayAutoRotationPreferences();
    // pub fn GetDisplayConfigBufferSizes();
    pub fn GetDlgCtrlID(hwnd: HWND) -> c_int;
    pub fn GetDlgItem(hDlg: HWND, nIDDlgItem: c_int) -> HWND;
    pub fn GetDlgItemInt(
        hDlg: HWND, nIDDlgItem: c_int, lpTranslated: *mut BOOL, bSigned: BOOL,
    ) -> UINT;
    pub fn GetDlgItemTextA(
        hDlg: HWND, nIDDlgItem: c_int, lpString: LPSTR, nMaxCount: c_int,
    ) -> UINT;
    pub fn GetDlgItemTextW(
        hDlg: HWND, nIDDlgItem: c_int, lpString: LPWSTR, nMaxCount: c_int,
    ) -> UINT;
    // pub fn GetDoubleClickTime();
    pub fn GetFocus() -> HWND;
    pub fn GetForegroundWindow() -> HWND;
    // pub fn GetGUIThreadInfo();
    // pub fn GetGestureConfig();
    // pub fn GetGestureExtraArgs();
    // pub fn GetGestureInfo();
    // pub fn GetGuiResources();
    // pub fn GetIconInfo();
    // pub fn GetIconInfoExA();
    // pub fn GetIconInfoExW();
    // pub fn GetInputDesktop();
    // pub fn GetInputLocaleInfo();
    // pub fn GetInputState();
    pub fn GetKBCodePage() -> UINT;
    pub fn GetKeyNameTextA(lparam: LONG, lpString: LPSTR, cchSize: c_int) -> c_int;
    pub fn GetKeyNameTextW(lParam: LONG, lpString: LPWSTR, cchSize: c_int) -> c_int;
    pub fn GetKeyState(nVirtKey: c_int) -> SHORT;
    pub fn GetKeyboardLayout(idThread: DWORD) -> HKL;
    pub fn GetKeyboardLayoutList(nBuff: c_int, lpList: *mut HKL) -> c_int;
    pub fn GetKeyboardLayoutNameA(pwszKLID: LPSTR) -> BOOL;
    pub fn GetKeyboardLayoutNameW(pwszKLID: LPWSTR) -> BOOL;
    pub fn GetKeyboardState(lpKeyState: PBYTE) -> BOOL;
    pub fn GetKeyboardType(nTypeFlag: c_int) -> c_int;
    // pub fn GetLastActivePopup();
    // pub fn GetLastInputInfo();
    // pub fn GetLayeredWindowAttributes();
    // pub fn GetListBoxInfo();
    pub fn GetMenu(hWnd: HWND) -> HMENU;
    // pub fn GetMenuBarInfo();
    // pub fn GetMenuCheckMarkDimensions();
    // pub fn GetMenuContextHelpId();
    // pub fn GetMenuDefaultItem();
    // pub fn GetMenuInfo();
    // pub fn GetMenuItemCount();
    // pub fn GetMenuItemID();
    // pub fn GetMenuItemInfoA();
    // pub fn GetMenuItemInfoW();
    // pub fn GetMenuItemRect();
    // pub fn GetMenuState();
    // pub fn GetMenuStringA();
    // pub fn GetMenuStringW();
    // pub fn GetMessageA();
    // pub fn GetMessageExtraInfo();
    // pub fn GetMessagePos();
    pub fn GetMessageTime() -> LONG;
    pub fn GetMessageW(lpMsg: LPMSG, hWnd: HWND, wMsgFilterMin: UINT, wMsgFilterMax: UINT) -> BOOL;
    pub fn GetMonitorInfoA(hMonitor: HMONITOR, lpmi: LPMONITORINFO) -> BOOL;
    pub fn GetMonitorInfoW(hMonitor: HMONITOR, lpmi: LPMONITORINFO) -> BOOL;
    // pub fn GetMouseMovePointsEx();
    // pub fn GetNextDlgGroupItem();
    // pub fn GetNextDlgTabItem();
    pub fn GetOpenClipboardWindow() -> HWND;
    pub fn GetParent(hWnd: HWND) -> HWND;
    pub fn GetPhysicalCursorPos(lpPoint: LPPOINT) -> BOOL;
    // pub fn GetPointerCursorId();
    // pub fn GetPointerDevice();
    // pub fn GetPointerDeviceCursors();
    // pub fn GetPointerDeviceProperties();
    // pub fn GetPointerDeviceRects();
    // pub fn GetPointerDevices();
    // pub fn GetPointerFrameInfo();
    // pub fn GetPointerFrameInfoHistory();
    // pub fn GetPointerFramePenInfo();
    // pub fn GetPointerFramePenInfoHistory();
    // pub fn GetPointerFrameTouchInfo();
    // pub fn GetPointerFrameTouchInfoHistory();
    // pub fn GetPointerInfo();
    // pub fn GetPointerInfoHistory();
    // pub fn GetPointerInputTransform();
    // pub fn GetPointerPenInfo();
    // pub fn GetPointerPenInfoHistory();
    // pub fn GetPointerTouchInfo();
    // pub fn GetPointerTouchInfoHistory();
    // pub fn GetPointerType();
    // pub fn GetPriorityClipboardFormat();
    // pub fn GetProcessDefaultLayout();
    // pub fn GetProcessWindowStation();
    pub fn GetPropA(hwnd: HWND, lpString: LPCSTR) -> HANDLE;
    pub fn GetPropW(hwnd: HWND, lpString: LPCWSTR) -> HANDLE;
    pub fn GetQueueStatus(flags: UINT) -> DWORD;
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
    // pub fn GetScrollInfo();
    pub fn GetScrollPos(hWnd: HWND, nBar: c_int) -> c_int;
    pub fn GetScrollRange(hWnd: HWND, nBar: c_int, lpMinPos: LPINT, lpMaxPos: LPINT) -> BOOL;
    pub fn GetShellWindow() -> HWND;
    // pub fn GetSubMenu();
    pub fn GetSysColor(nIndex: c_int) -> DWORD;
    pub fn GetSysColorBrush(nIndex: c_int) -> HBRUSH;
    // pub fn GetSystemMenu();
    pub fn GetSystemMetrics(nIndex: c_int) -> c_int;
    // pub fn GetTabbedTextExtentA();
    // pub fn GetTabbedTextExtentW();
    pub fn GetThreadDesktop(dwThreadId: DWORD) -> HDESK;
    // pub fn GetTitleBarInfo();
    // pub fn GetTopWindow();
    // pub fn GetTouchInputInfo();
    // pub fn GetUnpredictedMessagePos();
    // pub fn GetUpdateRect();
    // pub fn GetUpdateRgn();
    pub fn GetUpdatedClipboardFormats(
        lpuiFormats: PUINT, cFormats: UINT, pcFormatsOUT: PUINT,
    ) -> BOOL;
    // pub fn GetUserObjectInformationA();
    // pub fn GetUserObjectInformationW();
    // pub fn GetUserObjectSecurity();
    pub fn GetWindow(hWnd: HWND, uCmd: UINT) -> HWND;
    // pub fn GetWindowContextHelpId();
    // pub fn GetWindowDC();
    // pub fn GetWindowDisplayAffinity();
    // pub fn GetWindowFeedbackSetting();
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
    pub fn GetWindowPlacement(hWnd: HWND, lpwndpl: *mut WINDOWPLACEMENT) -> BOOL;
    pub fn GetWindowRect(hWnd: HWND, lpRect: LPRECT) -> BOOL;
    // pub fn GetWindowRgn();
    // pub fn GetWindowRgnBox();
    pub fn GetWindowTextA(hWnd: HWND, lpString: LPSTR, nMaxCount: c_int) -> c_int;
    pub fn GetWindowTextLengthA(hWnd: HWND) -> c_int;
    pub fn GetWindowTextLengthW(hWnd: HWND) -> c_int;
    pub fn GetWindowTextW(hWnd: HWND, lpString: LPWSTR, nMaxCount: c_int) -> c_int;
    pub fn GetWindowThreadProcessId(hWnd: HWND, lpdwProcessId: LPDWORD) -> DWORD;
    // pub fn GetWindowWord();
    // pub fn GrayStringA();
    // pub fn GrayStringW();
    pub fn HideCaret(hWnd: HWND) -> BOOL;
    // pub fn HiliteMenuItem();
    // pub fn IMPGetIMEA();
    // pub fn IMPGetIMEW();
    // pub fn IMPQueryIMEA();
    // pub fn IMPQueryIMEW();
    // pub fn IMPSetIMEA();
    // pub fn IMPSetIMEW();
    // pub fn ImpersonateDdeClientWindow();
    // pub fn InSendMessage();
    // pub fn InSendMessageEx();
    pub fn InflateRect(lprc: LPRECT, dx: c_int, dy: c_int) -> BOOL;
    // pub fn InitializeTouchInjection();
    // pub fn InjectTouchInput();
    pub fn InsertMenuA(
        hMenu: HMENU, uPosition: UINT, uFlags: UINT, uIDNewItem: UINT_PTR, lpNewItem: LPCSTR,
    ) -> BOOL;
    // pub fn InsertMenuItemA();
    // pub fn InsertMenuItemW();
    pub fn InsertMenuW(
        hMenu: HMENU, uPosition: UINT, uFlags: UINT, uIDNewItem: UINT_PTR, lpNewItem: LPCWSTR,
    ) -> BOOL;
    // pub fn InternalGetWindowText();
    pub fn IntersectRect(
        lprcDst: LPRECT, lprcSrc1: *const RECT, lprcSrc2: *const RECT,
    ) -> BOOL;
    pub fn InvalidateRect(hWnd: HWND, lpRect: *const RECT, bErase: BOOL) -> BOOL;
    // pub fn InvalidateRgn();
    pub fn InvertRect(hDC: HDC, lprc: *const RECT) -> BOOL;
    pub fn IsCharAlphaA(ch: CHAR) -> BOOL;
    pub fn IsCharAlphaNumericA(ch: CHAR) -> BOOL;
    pub fn IsCharAlphaNumericW(ch: WCHAR) -> BOOL;
    pub fn IsCharAlphaW(ch: WCHAR) -> BOOL;
    pub fn IsCharLowerA(ch: CHAR) -> BOOL;
    pub fn IsCharLowerW(ch: WCHAR) -> BOOL;
    pub fn IsCharUpperA(ch: CHAR) -> BOOL;
    pub fn IsCharUpperW(ch: WCHAR) -> BOOL;
    // pub fn IsChild();
    pub fn IsClipboardFormatAvailable(format: UINT) -> BOOL;
    pub fn IsDialogMessage(hDlg: HWND, lpMsg: LPMSG) -> BOOL;
    pub fn IsDialogMessageA(hDlg: HWND, lpMsg: LPMSG) -> BOOL;
    pub fn IsDialogMessageW(hDlg: HWND, lpMsg: LPMSG) -> BOOL;
    pub fn IsDlgButtonChecked(hDlg: HWND, nIDButton: c_int) -> UINT;
    pub fn IsGUIThread(bConvert: BOOL) -> BOOL;
    pub fn IsHungAppWindow(hwnd: HWND) -> BOOL;
    pub fn IsIconic(hWnd: HWND) -> BOOL;
    pub fn IsImmersiveProcess(hProcess: HANDLE) -> BOOL;
    // pub fn IsInDesktopWindowBand();
    pub fn IsMenu(hMenu: HMENU) -> BOOL;
    pub fn IsMouseInPointerEnabled() -> BOOL;
    pub fn IsProcessDPIAware() -> BOOL;
    pub fn IsRectEmpty(lprc: *const RECT) -> BOOL;
    pub fn IsTouchWindow(hwnd: HWND, pulFlags: PULONG) -> BOOL;
    // pub fn IsWinEventHookInstalled();
    pub fn IsWindow(hWnd: HWND) -> BOOL;
    pub fn IsWindowEnabled(hWnd: HWND) -> BOOL;
    // pub fn IsWindowUnicode();
    pub fn IsWindowVisible(hWnd: HWND) -> BOOL;
    pub fn IsWow64Message() -> BOOL;
    pub fn IsZoomed(hwnd: HWND) -> BOOL;
    pub fn KillTimer(hwnd: HWND, uIDEvent: UINT_PTR) -> BOOL;
    pub fn LoadAcceleratorsA(hInstance: HINSTANCE, lpTableName: LPCSTR) -> HACCEL;
    pub fn LoadAcceleratorsW(hInstance: HINSTANCE, lpTableName: LPCWSTR) -> HACCEL;
    pub fn LoadBitmapA(hInstance: HINSTANCE, lpBitmapName: LPCSTR) -> HBITMAP;
    pub fn LoadBitmapW(hInstance: HINSTANCE, lpBitmapName: LPCWSTR) -> HBITMAP;
    pub fn LoadCursorA(hInstance: HINSTANCE, lpCursorName: LPCSTR) -> HCURSOR;
    pub fn LoadCursorFromFileA(lpFileName: LPCSTR) -> HCURSOR;
    pub fn LoadCursorFromFileW(lpFileName: LPCWSTR) -> HCURSOR;
    pub fn LoadCursorW(hInstance: HINSTANCE, lpCursorName: LPCWSTR) -> HCURSOR;
    pub fn LoadIconA(hInstance: HINSTANCE, lpIconName: LPCSTR) -> HICON;
    pub fn LoadIconW(hInstance: HINSTANCE, lpIconName: LPCWSTR) -> HICON;
    pub fn LoadImageA(
        hInst: HINSTANCE, name: LPCSTR, type_: UINT, cx: c_int, cy: c_int, fuLoad: UINT,
    ) -> HANDLE;
    pub fn LoadImageW(
        hInst: HINSTANCE, name: LPCWSTR, type_: UINT, cx: c_int, cy: c_int, fuLoad: UINT,
    ) -> HANDLE;
    // pub fn LoadKeyboardLayoutA();
    // pub fn LoadKeyboardLayoutW();
    pub fn LoadMenuA(hInstance: HINSTANCE, lpMenuName: LPCSTR) -> HMENU;
    // pub fn LoadMenuIndirectA();
    // pub fn LoadMenuIndirectW();
    pub fn LoadMenuW(hInstance: HINSTANCE, lpMenuName: LPCWSTR) -> HMENU;
    // pub fn LoadStringA();
    // pub fn LoadStringW();
    // pub fn LockSetForegroundWindow();
    // pub fn LockWindowUpdate();
    // pub fn LockWorkStation();
    // pub fn LogicalToPhysicalPoint();
    // pub fn LogicalToPhysicalPointForPerMonitorDPI();
    // pub fn LookupIconIdFromDirectory();
    // pub fn LookupIconIdFromDirectoryEx();
    // pub fn MapDialogRect();
    // pub fn MapVirtualKeyA();
    // pub fn MapVirtualKeyExA();
    // pub fn MapVirtualKeyExW();
    // pub fn MapVirtualKeyW();
    // pub fn MapWindowPoints();
    // pub fn MenuItemFromPoint();
    // pub fn MessageBeep();
    pub fn MessageBoxA(hWnd: HWND, lpText: LPCSTR, lpCaption: LPCSTR, uType: UINT) -> c_int;
    pub fn MessageBoxExA(
        hWnd: HWND, lpText: LPCSTR, lpCaption: LPCSTR, uType: UINT, wLanguageId: WORD,
    ) -> c_int;
    pub fn MessageBoxExW(
        hWnd: HWND, lpText: LPCWSTR, lpCaption: LPCWSTR, uType: UINT, wLanguageId: WORD,
    ) -> c_int;
    // pub fn MessageBoxIndirectA();
    // pub fn MessageBoxIndirectW();
    // pub fn MessageBoxTimeoutA();
    // pub fn MessageBoxTimeoutW();
    pub fn MessageBoxW(hWnd: HWND, lpText: LPCWSTR, lpCaption: LPCWSTR, uType: UINT) -> c_int;
    // pub fn ModifyMenuA();
    // pub fn ModifyMenuW();
    // pub fn MonitorFromPoint();
    // pub fn MonitorFromRect();
    // pub fn MonitorFromWindow();
    // pub fn MoveWindow();
    // Use UINT instead of DWORD for dwWaitMask to be consistent with GetQueueStatus
    pub fn MsgWaitForMultipleObjects(
        nCount: DWORD, pHandles: *const HANDLE, fWaitAll: BOOL, dwMilliseconds: DWORD,
        dwWakeMask: UINT,
    ) -> DWORD;
    pub fn MsgWaitForMultipleObjectsEx(
        nCount: DWORD, pHandles: *const HANDLE, dwMilliseconds: DWORD, dwWakeMask: UINT,
        dwFlags: DWORD,
    ) -> DWORD;
    // pub fn NotifyWinEvent();
    // pub fn OemKeyScan();
    // pub fn OemToCharA();
    // pub fn OemToCharBuffA();
    // pub fn OemToCharBuffW();
    // pub fn OemToCharW();
    // pub fn OffsetRect();
    pub fn OpenClipboard(hWnd: HWND) -> BOOL;
    // pub fn OpenDesktopA();
    // pub fn OpenDesktopW();
    // pub fn OpenIcon();
    // pub fn OpenInputDesktop();
    // pub fn OpenWindowStationA();
    // pub fn OpenWindowStationW();
    // pub fn PackDDElParam();
    // pub fn PackTouchHitTestingProximityEvaluation();
    // pub fn PaintDesktop();
    // pub fn PeekMessageA();
    pub fn PeekMessageW(
        lpMsg: LPMSG, hWnd: HWND, wMsgFilterMin: UINT, wMsgFilterMax: UINT, wRemoveMsg: UINT,
    ) -> BOOL;
    // pub fn PhysicalToLogicalPoint();
    // pub fn PhysicalToLogicalPointForPerMonitorDPI();
    // pub fn PostMessageA();
    pub fn PostMessageW(hWnd: HWND, Msg: UINT, wParam: WPARAM, lParam: LPARAM) -> BOOL;
    pub fn PostQuitMessage(nExitCode: c_int);
    pub fn PostThreadMessageA(
        idThread: DWORD, msg: UINT, wParam: WPARAM, lParam: LPARAM,
    ) -> BOOL;
    pub fn PostThreadMessageW(
        idThread: DWORD, msg: UINT, wParam: WPARAM, lParam: LPARAM,
    ) -> BOOL;
    pub fn PrintWindow(hwnd: HWND, hdcBlt: HDC, nFlags: UINT) -> BOOL;
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
    // pub fn RegisterClassA();
    // pub fn RegisterClassExA();
    pub fn RegisterClassExW(lpWndClass: *const WNDCLASSEXW) -> ATOM;
    pub fn RegisterClassW(lpWndClass: *const WNDCLASSW) -> ATOM;
    pub fn RegisterClipboardFormatA(lpszFormat: LPCSTR) -> UINT;
    pub fn RegisterClipboardFormatW(lpszFormat: LPCWSTR) -> UINT;
    pub fn RegisterDeviceNotificationA(
        hRecipient: HANDLE, notificationFilter: LPVOID, flags: DWORD,
    ) -> HDEVNOTIFY;
    pub fn RegisterDeviceNotificationW(
        hRecipient: HANDLE, notificationFilter: LPVOID, flags: DWORD,
    ) -> HDEVNOTIFY;
    pub fn RegisterHotKey(hwnd: HWND, id: c_int, fsModifiers: UINT, vk: UINT) -> BOOL;
    // pub fn RegisterPointerDeviceNotifications();
    // pub fn RegisterPointerInputTarget();
    // pub fn RegisterPowerSettingNotification();
    pub fn RegisterRawInputDevices(
        pRawInputDevices: PCRAWINPUTDEVICE, uiNumDevices: UINT, cbSize: UINT,
    ) -> BOOL;
    // pub fn RegisterShellHookWindow();
    // pub fn RegisterSuspendResumeNotification();
    // pub fn RegisterTouchHitTestingWindow();
    // pub fn RegisterTouchWindow();
    // pub fn RegisterWindowMessageA();
    // pub fn RegisterWindowMessageW();
    pub fn ReleaseCapture() -> BOOL;
    pub fn ReleaseDC(hWnd: HWND, hDC: HDC) -> c_int;
    // pub fn RemoveClipboardFormatListener();
    // pub fn RemoveMenu();
    pub fn RemovePropA(hWnd: HWND, lpStr: LPCSTR) -> HANDLE;
    pub fn RemovePropW(hWnd: HWND, lpStr: LPCWSTR) -> HANDLE;
    // pub fn ReplyMessage();
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
    // pub fn SendDlgItemMessageA();
    // pub fn SendDlgItemMessageW();
    // pub fn SendIMEMessageExA();
    // pub fn SendIMEMessageExW();
    pub fn SendInput(cInputs: UINT, pInputs: LPINPUT, cbSize: c_int) -> UINT;
    pub fn SendMessageA(hWnd: HWND, Msg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT;
    // pub fn SendMessageCallbackA();
    // pub fn SendMessageCallbackW();
    pub fn SendMessageTimeoutA(
        hWnd: HWND, Msg: UINT, wParam: WPARAM, lParam: LPARAM, fuFlags: UINT, uTimeout: UINT,
        lpdwResult: PDWORD_PTR,
    ) -> LRESULT;
    pub fn SendMessageTimeoutW(
        hWnd: HWND, Msg: UINT, wParam: WPARAM, lParam: LPARAM, fuFlags: UINT, uTimeout: UINT,
        lpdwResult: PDWORD_PTR,
    ) -> LRESULT;
    pub fn SendMessageW(hWnd: HWND, Msg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT;
    pub fn SendNotifyMessageA(hWnd: HWND, msg: UINT, wParam: WPARAM, lParam: LPARAM) -> BOOL;
    pub fn SendNotifyMessageW(hWnd: HWND, msg: UINT, wParam: WPARAM, lParam: LPARAM) -> BOOL;
    pub fn SetActiveWindow(hWnd: HWND) -> HWND;
    pub fn SetCapture(hWnd: HWND) -> HWND;
    pub fn SetCaretBlinkTime(uMSeconds: UINT) -> BOOL;
    pub fn SetCaretPos(x: c_int, y: c_int) -> BOOL;
    pub fn SetClassLongA(hWnd: HWND, nIndex: c_int, dwNewLong: LONG) -> DWORD;
    #[cfg(target_arch = "x86_64")]
    pub fn SetClassLongPtrA(hWnd: HWND, nIndex: c_int, dwNewLong: LONG_PTR) -> ULONG_PTR;
    #[cfg(target_arch = "x86_64")]
    pub fn SetClassLongPtrW(hWnd: HWND, nIndex: c_int, dwNewLong: LONG_PTR) -> ULONG_PTR;
    pub fn SetClassLongW(hWnd: HWND, nIndex: c_int, dwNewLong: LONG) -> DWORD;
    pub fn SetClassWord(hWnd: HWND, nIndex: c_int, wNewWord: WORD) -> WORD;
    pub fn SetClipboardData(uFormat: UINT, hMem: HANDLE) -> HANDLE;
    pub fn SetClipboardViewer(hWndNewViewer: HWND) -> HWND;
    // pub fn SetCoalescableTimer();
    pub fn SetCursor(hCursor: HCURSOR) -> HCURSOR;
    pub fn SetCursorPos(x: c_int, y: c_int) -> BOOL;
    // pub fn SetDebugErrorLevel();
    // pub fn SetDeskWallpaper();
    // pub fn SetDisplayAutoRotationPreferences();
    // pub fn SetDisplayConfig();
    // pub fn SetDlgItemInt();
    // pub fn SetDlgItemTextA();
    // pub fn SetDlgItemTextW();
    // pub fn SetDoubleClickTime();
    pub fn SetFocus(hWnd: HWND) -> HWND;
    pub fn SetForegroundWindow(hWnd: HWND) -> BOOL;
    // pub fn SetGestureConfig();
    // pub fn SetKeyboardState();
    // pub fn SetLastErrorEx();
    // pub fn SetLayeredWindowAttributes();
    // pub fn SetMenu();
    // pub fn SetMenuContextHelpId();
    // pub fn SetMenuDefaultItem();
    // pub fn SetMenuInfo();
    // pub fn SetMenuItemBitmaps();
    // pub fn SetMenuItemInfoA();
    // pub fn SetMenuItemInfoW();
    // pub fn SetMessageExtraInfo();
    // pub fn SetMessageQueue();
    // pub fn SetParent();
    pub fn SetPhysicalCursorPos(x: c_int, y: c_int) -> BOOL;
    // pub fn SetProcessDPIAware();
    // pub fn SetProcessDefaultLayout();
    // pub fn SetProcessRestrictionExemption();
    // pub fn SetProcessWindowStation();
    pub fn SetPropA(hWnd: HWND, lpString: LPCSTR, hData: HANDLE) -> BOOL;
    pub fn SetPropW(hWnd: HWND, lpString: LPCWSTR, hData: HANDLE) -> BOOL;
    // pub fn SetRect();
    // pub fn SetRectEmpty();
    // pub fn SetScrollInfo();
    pub fn SetScrollPos(hWnd: HWND, nBar: c_int, nPos: c_int, bRedraw: BOOL) -> c_int;
    pub fn SetScrollRange(
        hWnd: HWND, nBar: c_int, nMinPos: c_int, nMaxPos: c_int, bRedraw: BOOL,
    ) -> BOOL;
    // pub fn SetShellWindow();
    // pub fn SetSysColors();
    pub fn SetSystemCursor(hcur: HCURSOR, id: DWORD) -> BOOL;
    pub fn SetThreadDesktop(hDesktop: HDESK) -> BOOL;
    // pub fn SetTimer();
    // pub fn SetUserObjectInformationA();
    // pub fn SetUserObjectInformationW();
    // pub fn SetUserObjectSecurity();
    // pub fn SetWinEventHook();
    // pub fn SetWindowContextHelpId();
    // pub fn SetWindowDisplayAffinity();
    // pub fn SetWindowFeedbackSetting();
    pub fn SetWindowLongA(hWnd: HWND, nIndex: c_int, dwNewLong: LONG) -> LONG;
    #[cfg(target_arch = "x86_64")]
    pub fn SetWindowLongPtrA(hWnd: HWND, nIndex: c_int, dwNewLong: LONG_PTR) -> LONG_PTR;
    #[cfg(target_arch = "x86_64")]
    pub fn SetWindowLongPtrW(hWnd: HWND, nIndex: c_int, dwNewLong: LONG_PTR) -> LONG_PTR;
    pub fn SetWindowLongW(hWnd: HWND, nIndex: c_int, dwNewLong: LONG) -> LONG;
    // pub fn SetWindowPlacement();
    pub fn SetWindowPos(
        hWnd: HWND, hWndInsertAfter: HWND, X: c_int, Y: c_int, cx: c_int, cy: c_int, uFlags: UINT,
    ) -> BOOL;
    // pub fn SetWindowRgn();
    // pub fn SetWindowTextA();
    pub fn SetWindowTextW(hWnd: HWND, lpString: LPCWSTR) -> BOOL;
    pub fn SetWindowWord(hwnd: HWND, nIndex: c_int, wNewWord: WORD) -> WORD;
    // pub fn SetWindowsHookA();
    // pub fn SetWindowsHookExA();
    // pub fn SetWindowsHookExW();
    // pub fn SetWindowsHookW();
    pub fn ShowCaret(hWnd: HWND) -> BOOL;
    pub fn ShowCursor(bShow: BOOL) -> c_int;
    // pub fn ShowOwnedPopups();
    // pub fn ShowScrollBar();
    // pub fn ShowSystemCursor();
    pub fn ShowWindow(hWnd: HWND, nCmdShow: c_int) -> BOOL;
    pub fn ShowWindowAsync(hWnd: HWND, nCmdShow: c_int) -> BOOL;
    // pub fn ShutdownBlockReasonCreate();
    // pub fn ShutdownBlockReasonDestroy();
    // pub fn ShutdownBlockReasonQuery();
    // pub fn SkipPointerFrameMessages();
    // pub fn SoundSentry();
    // pub fn SubtractRect();
    pub fn SwapMouseButton(fSwap: BOOL) -> BOOL;
    pub fn SwitchDesktop(hDesktop: HDESK) -> BOOL;
    // pub fn SwitchToThisWindow();
    // pub fn SystemParametersInfoA();
    // pub fn SystemParametersInfoW();
    // pub fn TabbedTextOutA();
    // pub fn TabbedTextOutW();
    // pub fn TileChildWindows();
    // pub fn TileWindows();
    // pub fn ToAscii();
    // pub fn ToAsciiEx();
    // pub fn ToUnicode();
    // pub fn ToUnicodeEx();
    pub fn TrackMouseEvent(lpEventTrack: LPTRACKMOUSEEVENT) -> BOOL;
    // pub fn TrackPopupMenu();
    // pub fn TrackPopupMenuEx();
    // pub fn TranslateAccelerator();
    // pub fn TranslateAcceleratorA();
    // pub fn TranslateAcceleratorW();
    // pub fn TranslateMDISysAccel();
    pub fn TranslateMessage(lpmsg: *const MSG) -> BOOL;
    // pub fn UnhookWinEvent();
    // pub fn UnhookWindowsHook();
    // pub fn UnhookWindowsHookEx();
    // pub fn UnionRect();
    // pub fn UnloadKeyboardLayout();
    // pub fn UnpackDDElParam();
    pub fn UnregisterClassA(lpClassName: LPCSTR, hInstance: HINSTANCE) -> BOOL;
    pub fn UnregisterClassW(lpClassName: LPCWSTR, hInstance: HINSTANCE) -> BOOL;
    // pub fn UnregisterDeviceNotification();
    // pub fn UnregisterHotKey();
    // pub fn UnregisterPointerInputTarget();
    // pub fn UnregisterPowerSettingNotification();
    // pub fn UnregisterSuspendResumeNotification();
    // pub fn UnregisterTouchWindow();
    // pub fn UpdateLayeredWindow();
    // pub fn UpdateLayeredWindowIndirect();
    pub fn UpdateWindow(hWnd: HWND) -> BOOL;
    // pub fn UserHandleGrantAccess();
    // pub fn ValidateRect();
    // pub fn ValidateRgn();
    // pub fn VkKeyScanA();
    // pub fn VkKeyScanExA();
    // pub fn VkKeyScanExW();
    // pub fn VkKeyScanW();
    // pub fn WINNLSEnableIME();
    // pub fn WINNLSGetEnableStatus();
    // pub fn WINNLSGetIMEHotkey();
    // pub fn WaitForInputIdle();
    pub fn WaitMessage() -> BOOL;
    // pub fn WinHelpA();
    // pub fn WinHelpW();
    // pub fn WindowFromDC();
    // pub fn WindowFromPhysicalPoint();
    // pub fn WindowFromPoint();
    // pub fn keybd_event();
    // pub fn mouse_event();
    // pub fn wsprintfA();
    // pub fn wsprintfW();
    // pub fn wvsprintfA();
    // pub fn wvsprintfW();
}

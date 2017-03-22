// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to comctl32.
#![cfg(windows)]

pub mod isolation_aware;

extern crate winapi;
#[cfg(test)]
#[cfg_attr(test, macro_use)]
extern crate kernel32;
use winapi::*;
extern "system" {
    pub fn AddMRUStringW(hMRU: HANDLE, szString: LPCWSTR) -> c_int;
    //pub fn CreateMRUListW(lpmi: LPMRUINFOW) -> HANDLE;
    pub fn CreateMappedBitmap(
        hInstance: HINSTANCE, idBitmap: INT_PTR, wFlags: UINT, lpColorMap: LPCOLORMAP,
        iNumMaps: c_int,
    ) -> HBITMAP;
    pub fn CreatePropertySheetPage(constPropSheetPagePointer: LPCPROPSHEETPAGEA) -> HPROPSHEETPAGE;
    pub fn CreatePropertySheetPageA(constPropSheetPagePointer: LPCPROPSHEETPAGEA) -> HPROPSHEETPAGE;
    pub fn CreatePropertySheetPageW(constPropSheetPagePointer: LPCPROPSHEETPAGEW) -> HPROPSHEETPAGE;
    pub fn CreateStatusWindow(style: LONG, lpszText: LPCSTR, hwndParent: HWND, wID: UINT) -> HWND;
    pub fn CreateStatusWindowA(style: LONG, lpszText: LPCSTR, hwndParent: HWND, wID: UINT) -> HWND;
    pub fn CreateStatusWindowW(style: LONG, lpszText: LPCWSTR, hwndParent: HWND, wID: UINT) -> HWND;
    //pub fn CreateToolbar();
    pub fn CreateToolbarEx(
        hwnd: HWND, ws: DWORD, wID: UINT, nBitmaps: c_int, hBMInst: HINSTANCE, wBMID: UINT_PTR,
        lpButtons: LPCTBBUTTON, iNumButtons: c_int, dxButton: c_int, dyButton: c_int,
        dxBitmap: c_int, dyBitmap: c_int, uStructSize: UINT,
    ) -> HWND;
    pub fn CreateUpDownControl(
        dwStyle: DWORD, x: c_int, y: c_int, cx: c_int, cy: c_int, hParent: HWND, nID: c_int,
        hInst: HINSTANCE, nBuddy: HWND, nUpper: c_int, nLower: c_int, nPos: c_int,
    ) -> HWND;
    pub fn DPA_Clone(hdpa: HDPA, hdpaNew: HDPA) -> HDPA;
    pub fn DPA_Create(cItemGrow: c_int) -> HDPA;
    pub fn DPA_CreateEx(cpGrow: c_int, hheap: HANDLE) -> HDPA;
    pub fn DPA_DeleteAllPtrs(hdpa: HDPA) -> BOOL;
    pub fn DPA_DeletePtr(hdpa: HDPA, i: c_int) -> PVOID;
    pub fn DPA_Destroy(hdpa: HDPA) -> BOOL;
    pub fn DPA_DestroyCallback(hdpa: HDPA, pfnCB: PFNDAENUMCALLBACK, pData: *mut c_void);
    pub fn DPA_EnumCallback(hdpa: HDPA, pfnCB: PFNDAENUMCALLBACK, pData: *mut c_void);
    pub fn DPA_GetPtr(hdpa: HDPA, i: INT_PTR) -> PVOID;
    pub fn DPA_GetPtrIndex(hdpa: HDPA, p: *const c_void) -> c_int;
    pub fn DPA_GetSize(hdpa: HDPA) -> ULONGLONG;
    pub fn DPA_Grow(hdpa: HDPA, cp: c_int) -> BOOL;
    pub fn DPA_InsertPtr(hdpa: HDPA, i: c_int, p: *mut c_void) -> c_int;
    pub fn DPA_LoadStream(
        phdpa: *mut HDPA, pfn: PFNDPASTREAM, pstream: *mut IStream, pvInstData: *mut c_void,
    ) -> HRESULT;
    pub fn DPA_Merge(
        hdpaDest: HDPA, hdpaSrc: HDPA, dwFlags: DWORD, pfnCompare: PFNDACOMPARE,
        pfnMerge: PFNDPAMERGE, lParam: LPARAM,
    ) -> BOOL;
    pub fn DPA_SaveStream(
        hdpa: HDPA, pfn: PFNDPASTREAM, pstream: *mut IStream, pvInstData: *mut c_void,
    ) -> HRESULT;
    pub fn DPA_Search(
        hdpa: HDPA, pFind: *mut c_void, iStart: c_int, pfnCompare: PFNDACOMPARE, lParam: LPARAM,
        options: UINT,
    ) -> c_int;
    pub fn DPA_SetPtr(hdpa: HDPA, i: c_int, p: *mut c_void) -> BOOL;
    pub fn DPA_Sort(hdpa: HDPA, pfnCompare: PFNDACOMPARE, lParam: LPARAM) -> BOOL;
    pub fn DSA_Clone(hdsa: HDSA) -> HDSA;
    pub fn DSA_Create(cbItem: c_int, cItemGrow: c_int) -> HDSA;
    pub fn DSA_DeleteAllItems(hdsa: HDSA) -> BOOL;
    pub fn DSA_DeleteItem(hdsa: HDSA, i: c_int) -> BOOL;
    pub fn DSA_Destroy(hdsa: HDSA) -> BOOL;
    pub fn DSA_DestroyCallback(hdsa: HDSA, pfnCB: PFNDAENUMCALLBACK, pData: *mut c_void);
    pub fn DSA_EnumCallback(hdsa: HDSA, pfnCB: PFNDAENUMCALLBACK, pData: *mut c_void);
    pub fn DSA_GetItem(hdsa: HDSA, i: c_int, pitem: *mut c_void) -> BOOL;
    pub fn DSA_GetItemPtr(hdsa: HDSA, i: c_int) -> PVOID;
    pub fn DSA_GetSize(hdsa: HDSA) -> ULONGLONG;
    pub fn DSA_InsertItem(hdsa: HDSA, i: c_int, pitem: *const c_void) -> c_int;
    pub fn DSA_SetItem(hdsa: HDSA, i: c_int, pitem: *const c_void) -> BOOL;
    pub fn DSA_Sort(pdsa: HDSA, pfnCompare: PFNDACOMPARE, lParam: LPARAM) -> BOOL;
    pub fn DefSubclassProc(hWnd: HWND, uMsg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT;
    pub fn DestroyPropertySheetPage(hPSPage: HPROPSHEETPAGE) -> BOOL;
    pub fn DrawInsert(handParent: HWND, hLB: HWND, nItem: c_int);
    pub fn DrawShadowText(
        hdc: HDC, pszText: LPCWSTR, cch: UINT, prc: *mut RECT, dwFlags: DWORD, crText: COLORREF,
        crShadow: COLORREF, ixOffset: c_int, iyOffset: c_int,
    ) -> c_int;
    pub fn DrawStatusText(hDC: HDC, lprc: LPCRECT, pszText: LPCSTR, uFlags: UINT);
    pub fn DrawStatusTextA(hDC: HDC, lprc: LPCRECT, pszText: LPCSTR, uFlags: UINT);
    pub fn DrawStatusTextW(hDC: HDC, lprc: LPCRECT, pszText: LPCWSTR, uFlags: UINT);
    pub fn EnumMRUListW(hMRU: HANDLE, nItem: c_int, lpData: *mut c_void, uLen: UINT) -> c_int;
    pub fn FlatSB_EnableScrollBar(hWnd: HWND, wSBflags: c_int, wArrows: UINT) -> BOOL;
    pub fn FlatSB_GetScrollInfo(hwnd: HWND, code: c_int, lpsi: LPSCROLLINFO) -> BOOL;
    pub fn FlatSB_GetScrollPos(hWnd: HWND, code: c_int) -> c_int;
    pub fn FlatSB_GetScrollProp(hWnd: HWND, propIndex: c_int, pValue: LPINT) -> BOOL;
    pub fn FlatSB_GetScrollRange(hWnd: HWND, code: c_int, lpMinPos: LPINT, lpMaxPos: LPINT) -> BOOL;
    pub fn FlatSB_SetScrollInfo(hWnd: HWND, code: c_int, psi: LPSCROLLINFO, fRedraw: BOOL) -> c_int;
    pub fn FlatSB_SetScrollPos(hWnd: HWND, code: c_int, pos: c_int, fRedraw: BOOL) -> c_int;
    pub fn FlatSB_SetScrollProp(hWnd: HWND, index: UINT, newValue: INT_PTR, fRedraw: BOOL) -> BOOL;
    pub fn FlatSB_SetScrollRange(
        hWnd: HWND, code: c_int, min: c_int, max: c_int, fRedraw: BOOL
    ) -> c_int;
    pub fn FlatSB_ShowScrollBar(hWnd: HWND, code: c_int, fShow: BOOL) -> BOOL;
    pub fn FreeMRUList(hMRU: HANDLE) -> c_int;
    pub fn GetEffectiveClientRect(hWnd: HWND, lprc: LPRECT, lpInfo: *const INT);
    pub fn GetMUILanguage() -> LANGID;
    pub fn GetWindowSubclass(
        hWnd: HWND, pfnSubclass: SUBCLASSPROC, uIdSubclass: UINT_PTR, pdwRefData: *mut DWORD_PTR,
    ) -> BOOL;
    pub fn HIMAGELIST_QueryInterface(
        himl: HIMAGELIST, riid: REFIID, ppv: *mut *mut c_void,
    ) -> HRESULT;
    pub fn ImageList_Add(himl: HIMAGELIST, hbmImage: HBITMAP, hbmMask: HBITMAP) -> c_int;
    pub fn ImageList_AddIcon(himl: HIMAGELIST, hicon: HICON) -> c_int;
    pub fn ImageList_AddMasked(himl: HIMAGELIST, hbmImage: HBITMAP, crMask: COLORREF) -> c_int;
    pub fn ImageList_BeginDrag(
        himlTrack: HIMAGELIST, iTrack: c_int, dxHotspot: c_int, dyHotspot: c_int,
    ) -> BOOL;
    pub fn ImageList_CoCreateInstance(
        rclsid: REFCLSID, punkOuter: *const IUnknown, riid: REFIID, ppv: *mut *mut c_void,
    ) -> HRESULT;
    pub fn ImageList_Copy(
        himlDst: HIMAGELIST, iDst: c_int, himlSrc: HIMAGELIST, iSrc: c_int, uFlags: UINT,
    ) -> BOOL;
    pub fn ImageList_Create(
        cx: c_int, cy: c_int, flags: UINT, cInitial: c_int, cGrow: c_int,
    ) -> HIMAGELIST;
    pub fn ImageList_Destroy(himl: HIMAGELIST) -> BOOL;
    pub fn ImageList_DragEnter(hwndLock: HWND, x: c_int, y: c_int) -> BOOL;
    pub fn ImageList_DragLeave(hwndLock: HWND) -> BOOL;
    pub fn ImageList_DragMove(x: c_int, y: c_int) -> BOOL;
    pub fn ImageList_DragShowNolock(fShow: BOOL) -> BOOL;
    pub fn ImageList_Draw(
        himl: HIMAGELIST, i: c_int, hdcDst: HDC, x: c_int, y: c_int, fStyle: UINT,
    ) -> BOOL;
    pub fn ImageList_DrawEx(
        himl: HIMAGELIST, i: c_int, hdcDst: HDC, x: c_int, y: c_int, dx: c_int, dy: c_int,
        rgbBk: COLORREF, rgbFg: COLORREF, fStyle: UINT,
    ) -> BOOL;
    pub fn ImageList_DrawIndirect(pimldp: *mut IMAGELISTDRAWPARAMS) -> BOOL;
    pub fn ImageList_Duplicate(himl: HIMAGELIST) -> HIMAGELIST;
    pub fn ImageList_EndDrag();
    pub fn ImageList_GetBkColor(himl: HIMAGELIST) -> COLORREF;
    pub fn ImageList_GetDragImage(ppt: *mut POINT, pptHotspot: *mut POINT) -> HIMAGELIST;
    pub fn ImageList_GetIcon(himl: HIMAGELIST, i: c_int, flags: UINT) -> HICON;
    pub fn ImageList_GetIconSize(himl: HIMAGELIST, cx: *mut c_int, cy: *mut c_int) -> BOOL;
    pub fn ImageList_GetImageCount(himl: HIMAGELIST) -> c_int;
    pub fn ImageList_GetImageInfo(himl: HIMAGELIST, i: c_int, pImageInfo: *mut IMAGEINFO) -> BOOL;
    // pub fn ImageList_GetImageRect();
    pub fn ImageList_LoadImage(
        hi: HINSTANCE, lpbmp: LPCSTR, cx: c_int, cGrow: c_int, crMask: COLORREF, uType: UINT,
        uFlags: UINT,
    ) -> HIMAGELIST;
    pub fn ImageList_LoadImageA(
        hi: HINSTANCE, lpbmp: LPCSTR, cx: c_int, cGrow: c_int, crMask: COLORREF, uType: UINT,
        uFlags: UINT,
    ) -> HIMAGELIST;
    pub fn ImageList_LoadImageW(
        hi: HINSTANCE, lpbmp: LPCWSTR, cx: c_int, cGrow: c_int, crMask: COLORREF, uType: UINT,
        uFlags: UINT,
    ) -> HIMAGELIST;
    pub fn ImageList_Merge(
        himl1: HIMAGELIST, i1: c_int, himl2: HIMAGELIST, i2: c_int, dx: c_int, dy: c_int,
    ) -> HIMAGELIST;
    pub fn ImageList_Read(pstm: *mut IStream) -> HIMAGELIST;
    pub fn ImageList_ReadEx(
        dwFlags: DWORD, pstm: *mut IStream, riid: REFIID, ppv: *mut PVOID,
    ) -> HRESULT;
    pub fn ImageList_Remove(himl: HIMAGELIST, i: c_int) -> BOOL;
    pub fn ImageList_Replace(
        himl: HIMAGELIST, i: c_int, hbmImage: HBITMAP, hbmMask: HBITMAP,
    ) -> BOOL;
    pub fn ImageList_ReplaceIcon(himl: HIMAGELIST, i: c_int, hicon: HICON) -> c_int;
    pub fn ImageList_SetBkColor(himl: HIMAGELIST, clrBk: COLORREF) -> COLORREF;
    pub fn ImageList_SetDragCursorImage(
        himlDrag: HIMAGELIST, iDrag: c_int, dxHotspot: c_int, dyHotspot: c_int,
    ) -> BOOL;
    // pub fn ImageList_SetFilter();
    pub fn ImageList_SetIconSize(himl: HIMAGELIST, cx: c_int, cy: c_int) -> BOOL;
    pub fn ImageList_SetImageCount(himl: HIMAGELIST, uNewCount: UINT) -> BOOL;
    pub fn ImageList_SetOverlayImage(himl: HIMAGELIST, iImage: c_int, iOverlay: c_int) -> BOOL;
    pub fn ImageList_Write(himl: HIMAGELIST, pstm: *mut IStream) -> BOOL;
    pub fn ImageList_WriteEx(himl: HIMAGELIST, dwFlags: DWORD, pstm: *mut IStream) -> HRESULT;
    pub fn InitCommonControls();
    pub fn InitCommonControlsEx(lpInitCtrls: *const INITCOMMONCONTROLSEX) -> BOOL;
    pub fn InitMUILanguage(uiLang: LANGID);
    pub fn InitializeFlatSB(hWnd: HWND) -> BOOL;
    pub fn LBItemFromPt(hLB: HWND, pt: POINT, bAutoScroll: BOOL) -> c_int;
    pub fn LoadIconMetric(
        hinst: HINSTANCE, pszName: PCWSTR, lims: c_int, phico: *mut HICON,
    ) -> HRESULT;
    pub fn LoadIconWithScaleDown(
        hinst: HINSTANCE, pszName: PCWSTR, cx: c_int, cy: c_int, phico: *mut HICON,
    ) -> HRESULT;
    pub fn MakeDragList(hLB: HWND) -> BOOL;
    pub fn MenuHelp(
        uMsg: UINT, wParam: WPARAM, lParam: LPARAM, hMainMenu: HMENU, hInst: HINSTANCE,
        hwndStatus: HWND, lpwIDs: *mut UINT,
    );
    pub fn PropertySheet(lppsph: LPCPROPSHEETHEADERA) -> INT_PTR;
    pub fn PropertySheetA(lppsph: LPCPROPSHEETHEADERA) -> INT_PTR;
    pub fn PropertySheetW(lppsph: LPCPROPSHEETHEADERW) -> INT_PTR;
    pub fn RemoveWindowSubclass(
        hWnd: HWND, pfnSubclass: SUBCLASSPROC, uIdSubclass: UINT_PTR,
    ) -> BOOL;
    pub fn SetWindowSubclass(
        hWnd: HWND, pfnSubclass: SUBCLASSPROC, uIdSubclass: UINT_PTR, dwRefData: DWORD_PTR,
    ) -> BOOL;
    pub fn ShowHideMenuCtl(hWnd: HWND, uFlags: UINT_PTR, lpInfo: LPINT) -> BOOL;
    pub fn Str_SetPtrW(ppsz: *mut LPWSTR, psz: LPCWSTR) -> BOOL;
    pub fn TaskDialog(
        hwndOwner: HWND, hInstance: HINSTANCE, pszWindowTitle: PCWSTR, pszMainInstruction: PCWSTR,
        pszContent: PCWSTR, dwCommonButtons: TASKDIALOG_COMMON_BUTTON_FLAGS, pszIcon: PCWSTR,
        pnButton: *mut c_int,
    ) -> HRESULT;
    pub fn TaskDialogIndirect(
        pTaskConfig: *const TASKDIALOGCONFIG, pnButton: *mut c_int, pnRadioButton: *mut c_int,
        pfVerificationFlagChecked: *mut BOOL,
    ) -> HRESULT;
    pub fn UninitializeFlatSB(hWnd: HWND) -> HRESULT;
    pub fn _TrackMouseEvent(lpEventTrack: LPTRACKMOUSEEVENT) -> BOOL;
}

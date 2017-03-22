#[macro_export]
#[doc(hidden)]
macro_rules! __winapi_comctl_isolation_aware {
    (ia_kernel32 = $($p:ident)::+;) => ();
    (
        ia_kernel32 = $($p:ident)::+;

        pub fn $isolation_aware_fn_name:ident($($param:ident: $param_ty:ty),*) $(-> $ret:ty)*
            where normal_ident = $fn_name:ident,
                  default_ret = $default_ret:expr;

        $($rest:tt)*
    ) => {
        #[inline]
        #[allow(non_snake_case)]
        pub unsafe extern "system" fn $isolation_aware_fn_name($($param: $param_ty),*) $(-> $ret)* {
            extern "system" fn default_pfn($(_: $param_ty),*) $(-> $ret)* {
                panic!("Attempted to call un-initialized function")
            }

            type FnType = extern "system" fn($($param_ty),*) $(-> $ret)*;
            static mut PFN: FnType = default_pfn;

            if let Some(ulp_cookie) = $($p::)+isolation_aware_prepare() {
                if PFN as *const usize == &default_pfn as *const _ as *const usize {
                    let pfn = load_comctl_fn(concat!(stringify!($fn_name), "\0").as_ptr() as *const c_char);

                    if pfn != ptr::null() {
                        PFN = mem::transmute(pfn);
                    } else {
                        $($p::)+isolation_aware_finish(ulp_cookie, true);
                        return $default_ret;
                    }
                }

                let ret = PFN($($param),*);
                $($p::)+isolation_aware_finish(ulp_cookie, false);
                ret
            } else {
                $default_ret
            }
        }

        __winapi_comctl_isolation_aware!{
            ia_kernel32 = $($p)::+;

            $($rest)*
        }
    }
}

#[macro_export]
macro_rules! isolation_aware_comctl32 {
    (mod_ia_kernel32 = $($p:ident)::+) => {mod __ia_comctl32_inner {
        extern crate winapi as __ia_kernel32_inner_winapi;
        use self::__ia_kernel32_inner_winapi::*;
        use super::*;
        use $($p)::+ as ia_kernel32;
        extern crate kernel32 as __kernel32;
        use std::{ptr, mem};

        __winapi_comctl_isolation_aware!{
            ia_kernel32 = $($p)::+;

            pub fn IsolationAwareCreateMappedBitmap(
                hInstance: HINSTANCE, idBitmap: INT_PTR, wFlags: UINT, lpColorMap: LPCOLORMAP,
                iNumMaps: c_int
            ) -> HBITMAP
                where normal_ident = CreateMappedBitmap,
                      default_ret = ptr::null_mut();
            pub fn IsolationAwareCreateStatusWindowA(
                style: LONG, lpszText: LPCSTR, hwndParent: HWND, wID: UINT
            ) -> HWND
                where normal_ident = CreateStatusWindowA,
                      default_ret = ptr::null_mut();
            pub fn IsolationAwareCreateStatusWindowW(
                style: LONG, lpszText: LPCWSTR, hwndParent: HWND, wID: UINT
            ) -> HWND
                where normal_ident = CreateStatusWindowW,
                      default_ret = ptr::null_mut();
            pub fn IsolationAwareCreateToolbarEx(
                hwnd: HWND, ws: DWORD, wID: UINT, nBitmaps: c_int, hBMInst: HINSTANCE,
                wBMID: UINT_PTR, lpButtons: LPCTBBUTTON, iNumButtons: c_int, dxButton: c_int,
                dyButton: c_int, dxBitmap: c_int, dyBitmap: c_int, uStructSize: UINT
            ) -> HWND
                where normal_ident = CreateToolbarEx,
                      default_ret = ptr::null_mut();
            pub fn IsolationAwareCreateUpDownControl(
                dwStyle: DWORD, x: c_int, y: c_int, cx: c_int, cy: c_int, hParent: HWND, nID: c_int,
                hInst: HINSTANCE, nBuddy: HWND, nUpper: c_int, nLower: c_int, nPos: c_int
            ) -> HWND
                where normal_ident = CreateUpDownControl,
                      default_ret = ptr::null_mut();
            pub fn IsolationAwareDPA_Clone(hdpa: HDPA, hdpaNew: HDPA) -> HDPA
                where normal_ident = DPA_Clone,
                      default_ret = ptr::null_mut();
            pub fn IsolationAwareDPA_Create(cItemGrow: c_int) -> HDPA
                where normal_ident = DPA_Create,
                      default_ret = ptr::null_mut();
            pub fn IsolationAwareDPA_CreateEx(cpGrow: c_int, hheap: HANDLE) -> HDPA
                where normal_ident = DPA_CreateEx,
                      default_ret = ptr::null_mut();
            pub fn IsolationAwareDPA_DeleteAllPtrs(hdpa: HDPA) -> BOOL
                where normal_ident = DPA_DeleteAllPtrs,
                      default_ret = FALSE;
            pub fn IsolationAwareDPA_DeletePtr(hdpa: HDPA, i: c_int) -> PVOID
                where normal_ident = DPA_DeletePtr,
                      default_ret = ptr::null_mut();
            pub fn IsolationAwareDPA_Destroy(hdpa: HDPA) -> BOOL
                where normal_ident = DPA_Destroy,
                      default_ret = FALSE;
            pub fn IsolationAwareDPA_DestroyCallback(
                hdpa: HDPA, pfnCB: PFNDAENUMCALLBACK, pData: *mut c_void
            )
                where normal_ident = DPA_DestroyCallback,
                      default_ret = ();
            pub fn IsolationAwareDPA_EnumCallback(
                hdpa: HDPA, pfnCB: PFNDAENUMCALLBACK, pData: *mut c_void
            )
                where normal_ident = DPA_EnumCallback,
                      default_ret = ();
            pub fn IsolationAwareDPA_GetPtr(hdpa: HDPA, i: INT_PTR) -> PVOID
                where normal_ident = DPA_GetPtr,
                      default_ret = ptr::null_mut();
            pub fn IsolationAwareDPA_GetPtrIndex(hdpa: HDPA, p: *const c_void) -> c_int
                where normal_ident = DPA_GetPtrIndex,
                      default_ret = -1;
            pub fn IsolationAwareDPA_GetSize(hdpa: HDPA) -> ULONGLONG
                where normal_ident = DPA_GetSize,
                      default_ret = 0;
            pub fn IsolationAwareDPA_Grow(hdpa: HDPA, cp: c_int) -> BOOL
                where normal_ident = DPA_Grow,
                      default_ret = FALSE;
            pub fn IsolationAwareDPA_InsertPtr(hdpa: HDPA, i: c_int, p: *mut c_void) -> c_int
                where normal_ident = DPA_InsertPtr,
                      default_ret = -1;
            pub fn IsolationAwareDPA_LoadStream(
                phdpa: *mut HDPA, pfn: PFNDPASTREAM, pstream: *mut IStream, pvInstData: *mut c_void
            ) -> HRESULT
                where normal_ident = DPA_LoadStream,
                      default_ret = S_OK;
            pub fn IsolationAwareDPA_Merge(
                hdpaDest: HDPA, hdpaSrc: HDPA, dwFlags: DWORD, pfnCompare: PFNDACOMPARE,
                pfnMerge: PFNDPAMERGE, lParam: LPARAM
            ) -> BOOL
                where normal_ident = DPA_Merge,
                      default_ret = FALSE;
            pub fn IsolationAwareDPA_SaveStream(
                hdpa: HDPA, pfn: PFNDPASTREAM, pstream: *mut IStream, pvInstData: *mut c_void
            ) -> HRESULT
                where normal_ident = DPA_SaveStream,
                      default_ret = S_OK;
            pub fn IsolationAwareDPA_Search(
                hdpa: HDPA, pFind: *mut c_void, iStart: c_int, pfnCompare: PFNDACOMPARE,
                lParam: LPARAM, options: UINT
            ) -> c_int
                where normal_ident = DPA_Search,
                      default_ret = -1;
            pub fn IsolationAwareDPA_SetPtr(hdpa: HDPA, i: c_int, p: *mut c_void) -> BOOL
                where normal_ident = DPA_SetPtr,
                      default_ret = FALSE;
            pub fn IsolationAwareDPA_Sort(hdpa: HDPA, pfnCompare: PFNDACOMPARE, lParam: LPARAM) -> BOOL
                where normal_ident = DPA_Sort,
                      default_ret = FALSE;
            pub fn IsolationAwareDSA_Clone(hdsa: HDSA) -> HDSA
                where normal_ident = DSA_Clone,
                      default_ret = ptr::null_mut();
            pub fn IsolationAwareDSA_Create(cbItem: c_int, cItemGrow: c_int) -> HDSA
                where normal_ident = DSA_Create,
                      default_ret = ptr::null_mut();
            pub fn IsolationAwareDSA_DeleteAllItems(hdsa: HDSA) -> BOOL
                where normal_ident = DSA_DeleteAllItems,
                      default_ret = FALSE;
            pub fn IsolationAwareDSA_DeleteItem(hdsa: HDSA, i: c_int) -> BOOL
                where normal_ident = DSA_DeleteItem,
                      default_ret = FALSE;
            pub fn IsolationAwareDSA_Destroy(hdsa: HDSA) -> BOOL
                where normal_ident = DSA_Destroy,
                      default_ret = FALSE;
            pub fn IsolationAwareDSA_DestroyCallback(
                hdsa: HDSA, pfnCB: PFNDAENUMCALLBACK, pData: *mut c_void
            )
                where normal_ident = DSA_DestroyCallback,
                      default_ret = ();
            pub fn IsolationAwareDSA_EnumCallback(
                hdsa: HDSA, pfnCB: PFNDAENUMCALLBACK, pData: *mut c_void
            )
                where normal_ident = DSA_EnumCallback,
                      default_ret = ();
            pub fn IsolationAwareDSA_GetItem(hdsa: HDSA, i: c_int, pitem: *mut c_void) -> BOOL
                where normal_ident = DSA_GetItem,
                      default_ret = FALSE;
            pub fn IsolationAwareDSA_GetItemPtr(hdsa: HDSA, i: c_int) -> PVOID
                where normal_ident = DSA_GetItemPtr,
                      default_ret = ptr::null_mut();
            pub fn IsolationAwareDSA_GetSize(hdsa: HDSA) -> ULONGLONG
                where normal_ident = DSA_GetSize,
                      default_ret = 0;
            pub fn IsolationAwareDSA_InsertItem(hdsa: HDSA, i: c_int, pitem: *const c_void) -> c_int
                where normal_ident = DSA_InsertItem,
                      default_ret = -1;
            pub fn IsolationAwareDSA_SetItem(hdsa: HDSA, i: c_int, pitem: *const c_void) -> BOOL
                where normal_ident = DSA_SetItem,
                      default_ret = FALSE;
            pub fn IsolationAwareDSA_Sort(
                pdsa: HDSA, pfnCompare: PFNDACOMPARE, lParam: LPARAM
            ) -> BOOL
                where normal_ident = DSA_Sort,
                      default_ret = FALSE;
            pub fn IsolationAwareDefSubclassProc(
                hWnd: HWND, uMsg: UINT, wParam: WPARAM, lParam: LPARAM
            ) -> LRESULT
                where normal_ident = DefSubclassProc,
                      default_ret = 0;
            pub fn IsolationAwareDrawInsert(handParent: HWND, hLB: HWND, nItem: c_int)
                where normal_ident = DrawInsert,
                      default_ret = ();
            pub fn IsolationAwareDrawShadowText(
                hdc: HDC, pszText: LPCWSTR, cch: UINT, prc: *mut RECT, dwFlags: DWORD,
                crText: COLORREF, crShadow: COLORREF, ixOffset: c_int, iyOffset: c_int
            ) -> c_int
                where normal_ident = DrawShadowText,
                      default_ret = -1;
            pub fn IsolationAwareDrawStatusTextA(
                hDC: HDC, lprc: LPCRECT, pszText: LPCSTR, uFlags: UINT
            )
                where normal_ident = DrawStatusTextA,
                      default_ret = ();
            pub fn IsolationAwareDrawStatusTextW(
                hDC: HDC, lprc: LPCRECT, pszText: LPCWSTR, uFlags: UINT
            )
                where normal_ident = DrawStatusTextW,
                      default_ret = ();
            pub fn IsolationAwareFlatSB_EnableScrollBar(
                hWnd: HWND, wSBflags: c_int, wArrows: UINT
            ) -> BOOL
                where normal_ident = FlatSB_EnableScrollBar,
                      default_ret = FALSE;
            pub fn IsolationAwareFlatSB_GetScrollInfo(
                hwnd: HWND, code: c_int, lpsi: LPSCROLLINFO
            ) -> BOOL
                where normal_ident = FlatSB_GetScrollInfo,
                      default_ret = FALSE;
            pub fn IsolationAwareFlatSB_GetScrollPos(hWnd: HWND, code: c_int) -> c_int
                where normal_ident = FlatSB_GetScrollPos,
                      default_ret = 0;
            pub fn IsolationAwareFlatSB_GetScrollProp(
                hWnd: HWND, propIndex: c_int, pValue: LPINT
            ) -> BOOL
                where normal_ident = FlatSB_GetScrollProp,
                      default_ret = FALSE;
            pub fn IsolationAwareFlatSB_GetScrollRange(
                hWnd: HWND, code: c_int, lpMinPos: LPINT, lpMaxPos: LPINT
            ) -> BOOL
                where normal_ident = FlatSB_GetScrollRange,
                      default_ret = FALSE;
            pub fn IsolationAwareFlatSB_SetScrollInfo(
                hWnd: HWND, code: c_int, psi: LPSCROLLINFO, fRedraw: BOOL
            ) -> c_int
                where normal_ident = FlatSB_SetScrollInfo,
                      default_ret = 0;
            pub fn IsolationAwareFlatSB_SetScrollPos(
                hWnd: HWND, code: c_int, pos: c_int, fRedraw: BOOL
            ) -> c_int
                where normal_ident = FlatSB_SetScrollPos,
                      default_ret = 0;
            pub fn IsolationAwareFlatSB_SetScrollProp(
                hWnd: HWND, index: UINT, newValue: INT_PTR, fRedraw: BOOL
            ) -> BOOL
                where normal_ident = FlatSB_SetScrollProp,
                      default_ret = FALSE;
            pub fn IsolationAwareFlatSB_SetScrollRange(
                hWnd: HWND, code: c_int, min: c_int, max: c_int, fRedraw: BOOL
            ) -> c_int
                where normal_ident = FlatSB_SetScrollRange,
                      default_ret = 0;
            pub fn IsolationAwareFlatSB_ShowScrollBar(hWnd: HWND, code: c_int, fShow: BOOL) -> BOOL
                where normal_ident = FlatSB_ShowScrollBar,
                      default_ret = FALSE;
            pub fn IsolationAwareGetEffectiveClientRect(
                hWnd: HWND, lprc: LPRECT, lpInfo: *const INT
            )
                where normal_ident = GetEffectiveClientRect,
                      default_ret = ();
            pub fn IsolationAwareGetMUILanguage() -> LANGID
                where normal_ident = GetMUILanguage,
                      default_ret = LANG_NEUTRAL << 10 | SUBLANG_NEUTRAL;
            pub fn IsolationAwareGetWindowSubclass(
                hWnd: HWND, pfnSubclass: SUBCLASSPROC, uIdSubclass: UINT_PTR,
                pdwRefData: *mut DWORD_PTR
            ) -> BOOL
                where normal_ident = GetWindowSubclass,
                      default_ret = FALSE;
            pub fn IsolationAwareHIMAGELIST_QueryInterface(
                himl: HIMAGELIST, riid: REFIID, ppv: *mut *mut c_void
            ) -> HRESULT
                where normal_ident = HIMAGELIST_QueryInterface,
                      default_ret = S_OK;
        }
        // Split into two invocations to avoid macro recursion errors
        __winapi_comctl_isolation_aware!{
            ia_kernel32 = $($p)::+;

            pub fn IsolationAwareImageList_Add(
                himl: HIMAGELIST, hbmImage: HBITMAP, hbmMask: HBITMAP
            ) -> c_int
                where normal_ident = ImageList_Add,
                      default_ret = -1;
            pub fn IsolationAwareImageList_AddMasked(
                himl: HIMAGELIST, hbmImage: HBITMAP, crMask: COLORREF
            ) -> c_int
                where normal_ident = ImageList_AddMasked,
                      default_ret = -1;
            pub fn IsolationAwareImageList_BeginDrag(
                himlTrack: HIMAGELIST, iTrack: c_int, dxHotspot: c_int, dyHotspot: c_int
            ) -> BOOL
                where normal_ident = ImageList_BeginDrag,
                      default_ret = FALSE;
            pub fn IsolationAwareImageList_Copy(
                himlDst: HIMAGELIST, iDst: c_int, himlSrc: HIMAGELIST, iSrc: c_int, uFlags: UINT
            ) -> BOOL
                where normal_ident = ImageList_Copy,
                      default_ret = FALSE;
            pub fn IsolationAwareImageList_Create(
                cx: c_int, cy: c_int, flags: UINT, cInitial: c_int, cGrow: c_int
            ) -> HIMAGELIST
                where normal_ident = ImageList_Create,
                      default_ret = ptr::null_mut();
            pub fn IsolationAwareImageList_Destroy(himl: HIMAGELIST) -> BOOL
                where normal_ident = ImageList_Destroy,
                      default_ret = FALSE;
            pub fn IsolationAwareImageList_DragEnter(hwndLock: HWND, x: c_int, y: c_int) -> BOOL
                where normal_ident = ImageList_DragEnter,
                      default_ret = FALSE;
            pub fn IsolationAwareImageList_DragLeave(hwndLock: HWND) -> BOOL
                where normal_ident = ImageList_DragLeave,
                      default_ret = FALSE;
            pub fn IsolationAwareImageList_DragMove(x: c_int, y: c_int) -> BOOL
                where normal_ident = ImageList_DragMove,
                      default_ret = FALSE;
            pub fn IsolationAwareImageList_DragShowNolock(fShow: BOOL) -> BOOL
                where normal_ident = ImageList_DragShowNolock,
                      default_ret = FALSE;
            pub fn IsolationAwareImageList_Draw(
                himl: HIMAGELIST, i: c_int, hdcDst: HDC, x: c_int, y: c_int, fStyle: UINT
            ) -> BOOL
                where normal_ident = ImageList_Draw,
                      default_ret = FALSE;
            pub fn IsolationAwareImageList_DrawEx(
                himl: HIMAGELIST, i: c_int, hdcDst: HDC, x: c_int, y: c_int, dx: c_int, dy: c_int,
                rgbBk: COLORREF, rgbFg: COLORREF, fStyle: UINT
            ) -> BOOL
                where normal_ident = ImageList_DrawEx,
                      default_ret = FALSE;
            pub fn IsolationAwareImageList_DrawIndirect(pimldp: *mut IMAGELISTDRAWPARAMS) -> BOOL
                where normal_ident = ImageList_DrawIndirect,
                      default_ret = FALSE;
            pub fn IsolationAwareImageList_Duplicate(himl: HIMAGELIST) -> HIMAGELIST
                where normal_ident = ImageList_Duplicate,
                      default_ret = ptr::null_mut();
            pub fn IsolationAwareImageList_EndDrag()
                where normal_ident = ImageList_EndDrag,
                      default_ret = ();
            pub fn IsolationAwareImageList_GetBkColor(himl: HIMAGELIST) -> COLORREF
                where normal_ident = ImageList_GetBkColor,
                      default_ret = 0;
            pub fn IsolationAwareImageList_GetDragImage(
                ppt: *mut POINT, pptHotspot: *mut POINT
            ) -> HIMAGELIST
                where normal_ident = ImageList_GetDragImage,
                      default_ret = ptr::null_mut();
            pub fn IsolationAwareImageList_GetIcon(himl: HIMAGELIST, i: c_int, flags: UINT) -> HICON
                where normal_ident = ImageList_GetIcon,
                      default_ret = ptr::null_mut();
            pub fn IsolationAwareImageList_GetIconSize(
                himl: HIMAGELIST, cx: *mut c_int, cy: *mut c_int
            ) -> BOOL
                where normal_ident = ImageList_GetIconSize,
                      default_ret = FALSE;
            pub fn IsolationAwareImageList_GetImageCount(himl: HIMAGELIST) -> c_int
                where normal_ident = ImageList_GetImageCount,
                      default_ret = 0;
            pub fn IsolationAwareImageList_GetImageInfo(
                himl: HIMAGELIST, i: c_int, pImageInfo: *mut IMAGEINFO
            ) -> BOOL
                where normal_ident = ImageList_GetImageInfo,
                      default_ret = FALSE;
            pub fn IsolationAwareImageList_LoadImageA(
                hi: HINSTANCE, lpbmp: LPCSTR, cx: c_int, cGrow: c_int, crMask: COLORREF,
                uType: UINT, uFlags: UINT
            ) -> HIMAGELIST
                where normal_ident = ImageList_LoadImageA,
                      default_ret = ptr::null_mut();
            pub fn IsolationAwareImageList_LoadImageW(
                hi: HINSTANCE, lpbmp: LPCWSTR, cx: c_int, cGrow: c_int, crMask: COLORREF,
                uType: UINT, uFlags: UINT
            ) -> HIMAGELIST
                where normal_ident = ImageList_LoadImageW,
                      default_ret = ptr::null_mut();
            pub fn IsolationAwareImageList_Merge(
                himl1: HIMAGELIST, i1: c_int, himl2: HIMAGELIST, i2: c_int, dx: c_int, dy: c_int
            ) -> HIMAGELIST
                where normal_ident = ImageList_Merge,
                      default_ret = ptr::null_mut();
            pub fn IsolationAwareImageList_Read(pstm: *mut IStream) -> HIMAGELIST
                where normal_ident = ImageList_Read,
                      default_ret = ptr::null_mut();
            pub fn IsolationAwareImageList_ReadEx(
                dwFlags: DWORD, pstm: *mut IStream, riid: REFIID, ppv: *mut PVOID
            ) -> HRESULT
                where normal_ident = ImageList_ReadEx,
                      default_ret = S_OK;
            pub fn IsolationAwareImageList_Remove(himl: HIMAGELIST, i: c_int) -> BOOL
                where normal_ident = ImageList_Remove,
                      default_ret = FALSE;
            pub fn IsolationAwareImageList_Replace(
                himl: HIMAGELIST, i: c_int, hbmImage: HBITMAP, hbmMask: HBITMAP
            ) -> BOOL
                where normal_ident = ImageList_Replace,
                      default_ret = FALSE;
            pub fn IsolationAwareImageList_ReplaceIcon(
                himl: HIMAGELIST, i: c_int, hicon: HICON
            ) -> c_int
                where normal_ident = ImageList_ReplaceIcon,
                      default_ret = -1;
            pub fn IsolationAwareImageList_SetBkColor(himl: HIMAGELIST, clrBk: COLORREF) -> COLORREF
                where normal_ident = ImageList_SetBkColor,
                      default_ret = 0;
            pub fn IsolationAwareImageList_SetDragCursorImage(
                himlDrag: HIMAGELIST, iDrag: c_int, dxHotspot: c_int, dyHotspot: c_int
            ) -> BOOL
                where normal_ident = ImageList_SetDragCursorImage,
                      default_ret = FALSE;
            pub fn IsolationAwareImageList_SetIconSize(
                himl: HIMAGELIST, cx: c_int, cy: c_int
            ) -> BOOL
                where normal_ident = ImageList_SetIconSize,
                      default_ret = FALSE;
            pub fn IsolationAwareImageList_SetImageCount(himl: HIMAGELIST, uNewCount: UINT) -> BOOL
                where normal_ident = ImageList_SetImageCount,
                      default_ret = FALSE;
            pub fn IsolationAwareImageList_SetOverlayImage(
                himl: HIMAGELIST, iImage: c_int, iOverlay: c_int
            ) -> BOOL
                where normal_ident = ImageList_SetOverlayImage,
                      default_ret = FALSE;
            pub fn IsolationAwareImageList_Write(himl: HIMAGELIST, pstm: *mut IStream) -> BOOL
                where normal_ident = ImageList_Write,
                      default_ret = FALSE;
            pub fn IsolationAwareImageList_WriteEx(
                himl: HIMAGELIST, dwFlags: DWORD, pstm: *mut IStream
            ) -> HRESULT
                where normal_ident = ImageList_WriteEx,
                      default_ret = S_OK;
            pub fn IsolationAwareInitCommonControls()
                where normal_ident = InitCommonControls,
                      default_ret = ();
            pub fn IsolationAwareInitCommonControlsEx(
                lpInitCtrls: *const INITCOMMONCONTROLSEX
            ) -> BOOL
                where normal_ident = InitCommonControlsEx,
                      default_ret = FALSE;
            pub fn IsolationAwareInitMUILanguage(uiLang: LANGID)
                where normal_ident = InitMUILanguage,
                      default_ret = ();
            pub fn IsolationAwareInitializeFlatSB(hWnd: HWND) -> BOOL
                where normal_ident = InitializeFlatSB,
                      default_ret = FALSE;
            pub fn IsolationAwareLBItemFromPt(hLB: HWND, pt: POINT, bAutoScroll: BOOL) -> c_int
                where normal_ident = LBItemFromPt,
                      default_ret = -1;
            pub fn IsolationAwareLoadIconMetric(
                hinst: HINSTANCE, pszName: PCWSTR, lims: c_int, phico: *mut HICON
            ) -> HRESULT
                where normal_ident = LoadIconMetric,
                      default_ret = S_OK;
            pub fn IsolationAwareLoadIconWithScaleDown(
                hinst: HINSTANCE, pszName: PCWSTR, cx: c_int, cy: c_int, phico: *mut HICON
            ) -> HRESULT
                where normal_ident = LoadIconWithScaleDown,
                      default_ret = S_OK;
            pub fn IsolationAwareMakeDragList(hLB: HWND) -> BOOL
                where normal_ident = MakeDragList,
                      default_ret = FALSE;
            pub fn IsolationAwareMenuHelp(
                uMsg: UINT, wParam: WPARAM, lParam: LPARAM, hMainMenu: HMENU, hInst: HINSTANCE,
                hwndStatus: HWND, lpwIDs: *mut UINT
            )
                where normal_ident = MenuHelp,
                      default_ret = ();
            pub fn IsolationAwareRemoveWindowSubclass(
                hWnd: HWND, pfnSubclass: SUBCLASSPROC, uIdSubclass: UINT_PTR
            ) -> BOOL
                where normal_ident = RemoveWindowSubclass,
                      default_ret = FALSE;
            pub fn IsolationAwareSetWindowSubclass(
                hWnd: HWND, pfnSubclass: SUBCLASSPROC, uIdSubclass: UINT_PTR, dwRefData: DWORD_PTR
            ) -> BOOL
                where normal_ident = SetWindowSubclass,
                      default_ret = FALSE;
            pub fn IsolationAwareShowHideMenuCtl(
                hWnd: HWND, uFlags: UINT_PTR, lpInfo: LPINT
            ) -> BOOL
                where normal_ident = ShowHideMenuCtl,
                      default_ret = FALSE;
            pub fn IsolationAwareStr_SetPtrW(ppsz: *mut LPWSTR, psz: LPCWSTR) -> BOOL
                where normal_ident = Str_SetPtrW,
                      default_ret = FALSE;
            pub fn IsolationAwareTaskDialog(
                hwndOwner: HWND, hInstance: HINSTANCE, pszWindowTitle: PCWSTR,
                pszMainInstruction: PCWSTR, pszContent: PCWSTR,
                dwCommonButtons: TASKDIALOG_COMMON_BUTTON_FLAGS, pszIcon: PCWSTR,
                pnButton: *mut c_int
            ) -> HRESULT
                where normal_ident = TaskDialog,
                      default_ret = S_OK;
            pub fn IsolationAwareTaskDialogIndirect(
                pTaskConfig: *const TASKDIALOGCONFIG, pnButton: *mut c_int,
                pnRadioButton: *mut c_int, pfVerificationFlagChecked: *mut BOOL
            ) -> HRESULT
                where normal_ident = TaskDialogIndirect,
                      default_ret = S_OK;
            pub fn IsolationAwareUninitializeFlatSB(hWnd: HWND) -> HRESULT
                where normal_ident = UninitializeFlatSB,
                      default_ret = S_OK;
            pub fn IsolationAware_TrackMouseEvent(lpEventTrack: LPTRACKMOUSEEVENT) -> BOOL
                where normal_ident = _TrackMouseEvent,
                      default_ret = FALSE;
        }

        unsafe fn load_comctl_fn(proc_name: LPCSTR) -> FARPROC {
            static mut COMCTL32_LIB: HMODULE = 0 as HMODULE;
            let mut comctl_fn: FARPROC = ptr::null();

            if false == ia_kernel32::isolation_aware_init_failed() {
                if let Some(ulp_cookie) = ia_kernel32::isolation_aware_prepare() {
                    // "Comctl32.dll" in UCS-2
                    const COMCTL32_DLL: &'static [WCHAR] = &[
                        0x0043, 0x006F, 0x006D, 0x0063, 0x0074, 0x006C, 0x0033, 0x0032, 0x002E,
                        0x0064, 0x006C, 0x006C, 0x0000
                    ];

                    if COMCTL32_LIB == ptr::null_mut() {
                        COMCTL32_LIB = __kernel32::LoadLibraryW(COMCTL32_DLL.as_ptr());
                        if COMCTL32_LIB == ptr::null_mut() {
                            ia_kernel32::isolation_aware_finish(ulp_cookie, true);
                            return ptr::null();
                        }
                    }

                    comctl_fn = __kernel32::GetProcAddress(COMCTL32_LIB, proc_name);

                    ia_kernel32::isolation_aware_finish(ulp_cookie, comctl_fn == ptr::null());
                }
            }

            comctl_fn
        }

        pub use self::{
            IsolationAwareCreateMappedBitmap as CreateMappedBitmap,
            IsolationAwareCreateStatusWindowA as CreateStatusWindowA,
            IsolationAwareCreateStatusWindowW as CreateStatusWindowW,
            IsolationAwareCreateToolbarEx as CreateToolbarEx,
            IsolationAwareCreateUpDownControl as CreateUpDownControl,
            IsolationAwareDPA_Clone as DPA_Clone,
            IsolationAwareDPA_Create as DPA_Create,
            IsolationAwareDPA_CreateEx as DPA_CreateEx,
            IsolationAwareDPA_DeleteAllPtrs as DPA_DeleteAllPtrs,
            IsolationAwareDPA_DeletePtr as DPA_DeletePtr,
            IsolationAwareDPA_Destroy as DPA_Destroy,
            IsolationAwareDPA_DestroyCallback as DPA_DestroyCallback,
            IsolationAwareDPA_EnumCallback as DPA_EnumCallback,
            IsolationAwareDPA_GetPtr as DPA_GetPtr,
            IsolationAwareDPA_GetPtrIndex as DPA_GetPtrIndex,
            IsolationAwareDPA_GetSize as DPA_GetSize,
            IsolationAwareDPA_Grow as DPA_Grow,
            IsolationAwareDPA_InsertPtr as DPA_InsertPtr,
            IsolationAwareDPA_LoadStream as DPA_LoadStream,
            IsolationAwareDPA_Merge as DPA_Merge,
            IsolationAwareDPA_SaveStream as DPA_SaveStream,
            IsolationAwareDPA_Search as DPA_Search,
            IsolationAwareDPA_SetPtr as DPA_SetPtr,
            IsolationAwareDPA_Sort as DPA_Sort,
            IsolationAwareDSA_Clone as DSA_Clone,
            IsolationAwareDSA_Create as DSA_Create,
            IsolationAwareDSA_DeleteAllItems as DSA_DeleteAllItems,
            IsolationAwareDSA_DeleteItem as DSA_DeleteItem,
            IsolationAwareDSA_Destroy as DSA_Destroy,
            IsolationAwareDSA_DestroyCallback as DSA_DestroyCallback,
            IsolationAwareDSA_EnumCallback as DSA_EnumCallback,
            IsolationAwareDSA_GetItem as DSA_GetItem,
            IsolationAwareDSA_GetItemPtr as DSA_GetItemPtr,
            IsolationAwareDSA_GetSize as DSA_GetSize,
            IsolationAwareDSA_InsertItem as DSA_InsertItem,
            IsolationAwareDSA_SetItem as DSA_SetItem,
            IsolationAwareDSA_Sort as DSA_Sort,
            IsolationAwareDefSubclassProc as DefSubclassProc,
            IsolationAwareDrawInsert as DrawInsert,
            IsolationAwareDrawShadowText as DrawShadowText,
            IsolationAwareDrawStatusTextA as DrawStatusTextA,
            IsolationAwareDrawStatusTextW as DrawStatusTextW,
            IsolationAwareFlatSB_EnableScrollBar as FlatSB_EnableScrollBar,
            IsolationAwareFlatSB_GetScrollInfo as FlatSB_GetScrollInfo,
            IsolationAwareFlatSB_GetScrollPos as FlatSB_GetScrollPos,
            IsolationAwareFlatSB_GetScrollProp as FlatSB_GetScrollProp,
            IsolationAwareFlatSB_GetScrollRange as FlatSB_GetScrollRange,
            IsolationAwareFlatSB_SetScrollInfo as FlatSB_SetScrollInfo,
            IsolationAwareFlatSB_SetScrollPos as FlatSB_SetScrollPos,
            IsolationAwareFlatSB_SetScrollProp as FlatSB_SetScrollProp,
            IsolationAwareFlatSB_SetScrollRange as FlatSB_SetScrollRange,
            IsolationAwareFlatSB_ShowScrollBar as FlatSB_ShowScrollBar,
            IsolationAwareGetEffectiveClientRect as GetEffectiveClientRect,
            IsolationAwareGetMUILanguage as GetMUILanguage,
            IsolationAwareGetWindowSubclass as GetWindowSubclass,
            IsolationAwareHIMAGELIST_QueryInterface as HIMAGELIST_QueryInterface,
            IsolationAwareImageList_Add as ImageList_Add,
            IsolationAwareImageList_AddMasked as ImageList_AddMasked,
            IsolationAwareImageList_BeginDrag as ImageList_BeginDrag,
            IsolationAwareImageList_Copy as ImageList_Copy,
            IsolationAwareImageList_Create as ImageList_Create,
            IsolationAwareImageList_Destroy as ImageList_Destroy,
            IsolationAwareImageList_DragEnter as ImageList_DragEnter,
            IsolationAwareImageList_DragLeave as ImageList_DragLeave,
            IsolationAwareImageList_DragMove as ImageList_DragMove,
            IsolationAwareImageList_DragShowNolock as ImageList_DragShowNolock,
            IsolationAwareImageList_Draw as ImageList_Draw,
            IsolationAwareImageList_DrawEx as ImageList_DrawEx,
            IsolationAwareImageList_DrawIndirect as ImageList_DrawIndirect,
            IsolationAwareImageList_Duplicate as ImageList_Duplicate,
            IsolationAwareImageList_EndDrag as ImageList_EndDrag,
            IsolationAwareImageList_GetBkColor as ImageList_GetBkColor,
            IsolationAwareImageList_GetDragImage as ImageList_GetDragImage,
            IsolationAwareImageList_GetIcon as ImageList_GetIcon,
            IsolationAwareImageList_GetIconSize as ImageList_GetIconSize,
            IsolationAwareImageList_GetImageCount as ImageList_GetImageCount,
            IsolationAwareImageList_GetImageInfo as ImageList_GetImageInfo,
            IsolationAwareImageList_LoadImageA as ImageList_LoadImageA,
            IsolationAwareImageList_LoadImageW as ImageList_LoadImageW,
            IsolationAwareImageList_Merge as ImageList_Merge,
            IsolationAwareImageList_Read as ImageList_Read,
            IsolationAwareImageList_ReadEx as ImageList_ReadEx,
            IsolationAwareImageList_Remove as ImageList_Remove,
            IsolationAwareImageList_Replace as ImageList_Replace,
            IsolationAwareImageList_ReplaceIcon as ImageList_ReplaceIcon,
            IsolationAwareImageList_SetBkColor as ImageList_SetBkColor,
            IsolationAwareImageList_SetDragCursorImage as ImageList_SetDragCursorImage,
            IsolationAwareImageList_SetIconSize as ImageList_SetIconSize,
            IsolationAwareImageList_SetImageCount as ImageList_SetImageCount,
            IsolationAwareImageList_SetOverlayImage as ImageList_SetOverlayImage,
            IsolationAwareImageList_Write as ImageList_Write,
            IsolationAwareImageList_WriteEx as ImageList_WriteEx,
            IsolationAwareInitCommonControls as InitCommonControls,
            IsolationAwareInitCommonControlsEx as InitCommonControlsEx,
            IsolationAwareInitMUILanguage as InitMUILanguage,
            IsolationAwareInitializeFlatSB as InitializeFlatSB,
            IsolationAwareLBItemFromPt as LBItemFromPt,
            IsolationAwareLoadIconMetric as LoadIconMetric,
            IsolationAwareLoadIconWithScaleDown as LoadIconWithScaleDown,
            IsolationAwareMakeDragList as MakeDragList,
            IsolationAwareMenuHelp as MenuHelp,
            IsolationAwareRemoveWindowSubclass as RemoveWindowSubclass,
            IsolationAwareSetWindowSubclass as SetWindowSubclass,
            IsolationAwareShowHideMenuCtl as ShowHideMenuCtl,
            IsolationAwareStr_SetPtrW as Str_SetPtrW,
            IsolationAwareTaskDialog as TaskDialog,
            IsolationAwareTaskDialogIndirect as TaskDialogIndirect,
            IsolationAwareUninitializeFlatSB as UninitializeFlatSB,
            IsolationAware_TrackMouseEvent as _TrackMouseEvent,
        };

        pub use $crate::{
            AddMRUStringW,
            // CreateMRUListW,
            CreatePropertySheetPage,
            CreatePropertySheetPageA,
            CreatePropertySheetPageW,
            CreateStatusWindow,
            // CreateToolbar,
            DestroyPropertySheetPage,
            DrawStatusText,
            EnumMRUListW,
            FreeMRUList,
            ImageList_CoCreateInstance,
            // ImageList_GetImageRect,
            ImageList_LoadImage,
            // ImageList_SetFilter,
            PropertySheet,
            PropertySheetA,
            PropertySheetW
        };
    }
    pub use self::__ia_comctl32_inner::*;
    }
}

#[cfg(test)]
pub mod ia_kernel32 {
    isolation_aware_kernel32!();
}

#[cfg(test)]
isolation_aware_comctl32!(mod_ia_kernel32 = self::ia_kernel32);

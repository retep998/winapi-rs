macro_rules! isolation_aware_user32 {
    (mod_ia_kernel32 = $($p:ident)::+) => {mod __ia_user32_inner {
        extern crate winapi as __ia_user32_inner_winapi;
        use self::__ia_user32_inner_winapi::*;
        use super::*;
        use std::ptr;

        __winapi_basic_isolation_aware!{
            crate_root = $crate;
            ia_kernel32 = $($p)::+;
            pub fn IsolationAwareCreateDialogIndirectParamA(
                hInstance: HINSTANCE, lpTemplate: LPCDLGTEMPLATEA, hWndParent: HWND,
                lpDialogFunc: DLGPROC, dwInitParam: LPARAM
            ) -> HWND
                where normal_ident = CreateDialogIndirectParamA,
                      default_ret = ptr::null_mut();
            pub fn IsolationAwareCreateDialogIndirectParamW(
                hInstance: HINSTANCE, lpTemplate: LPCDLGTEMPLATEW, hWndParent: HWND,
                lpDialogFunc: DLGPROC, dwInitParam: LPARAM
            ) -> HWND
                where normal_ident = CreateDialogIndirectParamW,
                      default_ret = ptr::null_mut();
            pub fn IsolationAwareCreateDialogParamA(
                hInstance: HINSTANCE, lpTemplateName: LPCSTR, hWndParent: HWND,
                lpDialogFunc: DLGPROC, dwInitParam: LPARAM
            ) -> HWND
                where normal_ident = CreateDialogParamA,
                      default_ret = ptr::null_mut();
            pub fn IsolationAwareCreateDialogParamW(
                hInstance: HINSTANCE, lpTemplateName: LPCWSTR, hWndParent: HWND,
                lpDialogFunc: DLGPROC, dwInitParam: LPARAM
            ) -> HWND
                where normal_ident = CreateDialogParamW,
                      default_ret = ptr::null_mut();
            pub fn IsolationAwareCreateWindowExA(
                dwExStyle: DWORD, lpClassName: LPCSTR, lpWindowName: LPCSTR, dwStyle: DWORD,
                x: c_int, y: c_int, nWidth: c_int, nHeight: c_int, hWndParent: HWND, hMenu: HMENU,
                hInstance: HINSTANCE, lpParam: LPVOID
            ) -> HWND
                where normal_ident = CreateWindowExA,
                      default_ret = ptr::null_mut();
            pub fn IsolationAwareCreateWindowExW(
                dwExStyle: DWORD, lpClassName: LPCWSTR, lpWindowName: LPCWSTR, dwStyle: DWORD,
                x: c_int, y: c_int, nWidth: c_int, nHeight: c_int, hWndParent: HWND, hMenu: HMENU,
                hInstance: HINSTANCE, lpParam: LPVOID
            ) -> HWND
                where normal_ident = CreateWindowExW,
                      default_ret = ptr::null_mut();
            pub fn IsolationAwareDialogBoxIndirectParamA(
                hInstance: HINSTANCE, hDialogTemplate: LPCDLGTEMPLATEA, hWndParent: HWND,
                lpDialogFunc: DLGPROC, dwInitParam: LPARAM
            ) -> INT_PTR
                where normal_ident = DialogBoxIndirectParamA,
                      default_ret = -1;
            pub fn IsolationAwareDialogBoxIndirectParamW(
                hInstance: HINSTANCE, hDialogTemplate: LPCDLGTEMPLATEW, hWndParent: HWND,
                lpDialogFunc: DLGPROC, dwInitParam: LPARAM
            ) -> INT_PTR
                where normal_ident = DialogBoxIndirectParamW,
                      default_ret = -1;
            // pub fn GetClassInfoA();
            // pub fn GetClassInfoExA();
            pub fn IsolationAwareGetClassInfoExW(
                hinst: HINSTANCE, lpszClass: LPCWSTR, lpwcx: LPWNDCLASSEXW
            ) -> BOOL
                where normal_ident = GetClassInfoExW,
                      default_ret = FALSE;
            pub fn IsolationAwareGetClassInfoW(
                hInstance: HINSTANCE, lpClassName: LPCWSTR, lpWndClass: LPWNDCLASSW
            ) -> BOOL
                where normal_ident = GetClassInfoW,
                      default_ret = FALSE;
            pub fn IsolationAwareMessageBoxA(
                hWnd: HWND, lpText: LPCSTR, lpCaption: LPCSTR, uType: UINT
            ) -> c_int
                where normal_ident = MessageBoxA,
                      default_ret = 0;
            pub fn IsolationAwareMessageBoxExA(
                hWnd: HWND, lpText: LPCSTR, lpCaption: LPCSTR, uType: UINT, wLanguageId: WORD
            ) -> c_int
                where normal_ident = MessageBoxExA,
                      default_ret = 0;
            pub fn IsolationAwareMessageBoxExW(
                hWnd: HWND, lpText: LPCWSTR, lpCaption: LPCWSTR, uType: UINT, wLanguageId: WORD
            ) -> c_int
                where normal_ident = MessageBoxExW,
                      default_ret = 0;
            pub fn IsolationAwareMessageBoxIndirectA(lpmbp: *const MSGBOXPARAMSA) -> c_int
                where normal_ident = MessageBoxIndirectA,
                      default_ret = 0;
            pub fn IsolationAwareMessageBoxIndirectW(lpmbp: *const MSGBOXPARAMSW) -> c_int
                where normal_ident = MessageBoxIndirectW,
                      default_ret = 0;
            pub fn IsolationAwareMessageBoxW(
                hWnd: HWND, lpText: LPCWSTR, lpCaption: LPCWSTR, uType: UINT
            ) -> c_int
                where normal_ident = MessageBoxW,
                      default_ret = 0;
            // pub fn RegisterClassA();
            // pub fn RegisterClassExA();
            pub fn IsolationAwareRegisterClassExW(lpWndClass: *const WNDCLASSEXW) -> ATOM
                where normal_ident = RegisterClassExW,
                      default_ret = 0;
            pub fn IsolationAwareRegisterClassW(lpWndClass: *const WNDCLASSW) -> ATOM
                where normal_ident = RegisterClassW,
                      default_ret = 0;
            pub fn IsolationAwareUnregisterClassA(lpClassName: LPCSTR, hInstance: HINSTANCE) -> BOOL
                where normal_ident = UnregisterClassA,
                      default_ret = FALSE;
            pub fn IsolationAwareUnregisterClassW(
                lpClassName: LPCWSTR, hInstance: HINSTANCE
            ) -> BOOL
                where normal_ident = UnregisterClassW,
                      default_ret = FALSE;
        }
    }
    pub use self::__ia_user32_inner::*;
    }
}

#[cfg(test)]
pub mod ia_kernel32 {
    isolation_aware_kernel32!();
}

#[cfg(test)]
isolation_aware_user32!(mod_ia_kernel32 = self::ia_kernel32);

/// Create isolation-aware comdlg32 bindings.
///
/// # Usage
///
/// ```ignore
/// mod isolation_aware {
///     pub mod kernel32 {
///         isolation_aware_kernel32!();
///     }
///
///     pub mod comdlg32 {
///         isolation_aware_comdlg32!(isolation_aware::kernel32);
///     }
/// }
/// ```
///
/// This is used similarly to the `isolation_aware_kernel32!()` macro, but takes a path to the
/// `kernel32` module from the crate root.
#[macro_export]
macro_rules! isolation_aware_comdlg32 {
    ($($p:ident)::+) => {mod __ia_comdlg32_inner {
        #![allow(dead_code)]
        extern crate winapi as __ia_kernel32_inner_winapi;
        use self::__ia_kernel32_inner_winapi::*;
        use ::$($p)::+ as ia_kernel32;
        extern crate kernel32 as __kernel32;
        use std::{ptr, mem};

        __winapi_comstar_isolation_aware!{
            ia_kernel32 = $($p)::+;

            pub fn IsolationAwareChooseColorA(lpcc: LPCHOOSECOLORA) -> BOOL
                where normal_ident = ChooseColorA,
                      default_ret = FALSE;
            pub fn IsolationAwareChooseColorW(lpcc: LPCHOOSECOLORW) -> BOOL
                where normal_ident = ChooseColorW,
                      default_ret = FALSE;
            pub fn IsolationAwareChooseFontA(lpcf: LPCHOOSEFONTA) -> BOOL
                where normal_ident = ChooseFontA,
                      default_ret = FALSE;
            pub fn IsolationAwareChooseFontW(lpcf: LPCHOOSEFONTW) -> BOOL
                where normal_ident = ChooseFontW,
                      default_ret = FALSE;
            pub fn IsolationAwareCommDlgExtendedError() -> DWORD
                where normal_ident = CommDlgExtendedError,
                      default_ret = 0;
            pub fn IsolationAwareFindTextA(lpfr: LPFINDREPLACEA) -> HWND
                where normal_ident = FindTextA,
                      default_ret = ptr::null_mut();
            pub fn IsolationAwareFindTextW(lpfr: LPFINDREPLACEW) -> HWND
                where normal_ident = FindTextW,
                      default_ret = ptr::null_mut();
            pub fn IsolationAwareGetFileTitleA(lpszFile: LPCSTR, Buf: LPSTR, cchSize: WORD) -> c_short
                where normal_ident = GetFileTitleA,
                      default_ret = -1;
            pub fn IsolationAwareGetFileTitleW(lpszFile: LPCWSTR, Buf: LPWSTR, cchSize: WORD) -> c_short
                where normal_ident = GetFileTitleW,
                      default_ret = -1;
            pub fn IsolationAwareGetOpenFileNameA(lpofn: LPOPENFILENAMEA) -> BOOL
                where normal_ident = GetOpenFileNameA,
                      default_ret = FALSE;
            pub fn IsolationAwareGetOpenFileNameW(lpofn: LPOPENFILENAMEW) -> BOOL
                where normal_ident = GetOpenFileNameW,
                      default_ret = FALSE;
            pub fn IsolationAwareGetSaveFileNameA(lpofn: LPOPENFILENAMEA) -> BOOL
                where normal_ident = GetSaveFileNameA,
                      default_ret = FALSE;
            pub fn IsolationAwareGetSaveFileNameW(lpofn: LPOPENFILENAMEW) -> BOOL
                where normal_ident = GetSaveFileNameW,
                      default_ret = FALSE;
            pub fn IsolationAwarePageSetupDlgA(lppsd: LPPAGESETUPDLGA) -> BOOL
                where normal_ident = PageSetupDlgA,
                      default_ret = FALSE;
            pub fn IsolationAwarePageSetupDlgW(lppsd: LPPAGESETUPDLGW) -> BOOL
                where normal_ident = PageSetupDlgW,
                      default_ret = FALSE;
            pub fn IsolationAwarePrintDlgA(pPD: LPPRINTDLGA) -> BOOL
                where normal_ident = PrintDlgA,
                      default_ret = FALSE;
            pub fn IsolationAwarePrintDlgExA(pPD: LPPRINTDLGEXA) -> HRESULT
                where normal_ident = PrintDlgExA,
                      default_ret = S_OK;
            pub fn IsolationAwarePrintDlgExW(pPD: LPPRINTDLGEXW) -> HRESULT
                where normal_ident = PrintDlgExW,
                      default_ret = S_OK;
            pub fn IsolationAwarePrintDlgW(pPD: LPPRINTDLGW) -> BOOL
                where normal_ident = PrintDlgW,
                      default_ret = FALSE;
            pub fn IsolationAwareReplaceTextA(lpfr: LPFINDREPLACEA) -> HWND
                where normal_ident = ReplaceTextA,
                      default_ret = ptr::null_mut();
            pub fn IsolationAwareReplaceTextW(lpfr: LPFINDREPLACEW) -> HWND
                where normal_ident = ReplaceTextW,
                      default_ret = ptr::null_mut();
        }

        unsafe fn load_comdlg_fn(proc_name: LPCSTR) -> FARPROC {
            static mut COMDLG32_LIB: HMODULE = 0 as HMODULE;
            let mut comdlg_fn: FARPROC = ptr::null();

            if false == ia_kernel32::isolation_aware_init_failed() {
                if let Some(ulp_cookie) = ia_kernel32::isolation_aware_prepare() {
                    // "Comdlg32.dll" in UCS-2
                    const COMDLG32_DLL: &'static [WCHAR] = &[
                        0x0043, 0x006F, 0x006D, 0x0064, 0x006C, 0x0067, 0x0033, 0x0032, 0x002E,
                        0x0064, 0x006C, 0x006C, 0x0000
                    ];

                    if COMDLG32_LIB == ptr::null_mut() {
                        COMDLG32_LIB = __kernel32::LoadLibraryW(COMDLG32_DLL.as_ptr());
                        if COMDLG32_LIB == ptr::null_mut() {
                            ia_kernel32::isolation_aware_finish(ulp_cookie, true);
                            return ptr::null();
                        }
                    }

                    comdlg_fn = __kernel32::GetProcAddress(COMDLG32_LIB, proc_name);

                    ia_kernel32::isolation_aware_finish(ulp_cookie, comdlg_fn == ptr::null());
                }
            }

            comdlg_fn
        }
    }
    pub use self::__ia_comdlg32_inner::*;
    }
}

#[cfg(test)]
pub mod ia_kernel32 {
    isolation_aware_kernel32!();
}

#[cfg(test)]
isolation_aware_comdlg32!(isolation_aware::ia_kernel32);
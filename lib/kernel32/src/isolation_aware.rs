#[macro_export]
#[doc(hidden)]
macro_rules! __winbase_wrapper_isolation_aware {
    () => ();
    (
        pub fn $isolation_aware_fn_name:ident($($param:ident: $param_ty:ty),*) $(-> $ret:ty)*
            where normal_ident = $fn_name:ident,
                  default_ret = $default_ret:expr;

        $($rest:tt)*
    ) => {
        #[inline]
        #[allow(non_snake_case)]
        pub unsafe extern "system" fn $isolation_aware_fn_name($($param: $param_ty),*) $(-> $ret)* {
            #[cfg(target_pointer_width = "64")]
            {
                $crate::$fn_name($($param),*)
            }
            #[cfg(not(target_pointer_width = "64"))]
            {
                use std::{ptr, mem};

                static mut FN_PTR: FARPROC = 0 as FARPROC;
                if FN_PTR == ptr::null() {
                    FN_PTR = load_winbase_fn(concat!(stringify!($fn_name), "\0").as_ptr() as LPCSTR);
                    if FN_PTR == ptr::null() {
                        return $default_ret;
                    }
                }

                type FnType = unsafe extern "system" fn($($param_ty),*) $(-> $ret)*;
                mem::transmute::<_, FnType>(FN_PTR)($($param),*)

            }
        }

        __winbase_wrapper_isolation_aware!{$($rest)*}
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __winapi_basic_isolation_aware {
    (crate_root = $crate_root:ident; ia_kernel32 = $($p:ident)::+;) => ();
    (
        crate_root = $crate_root:ident;
        ia_kernel32 = $($p:ident)::+;

        pub fn $isolation_aware_fn_name:ident($($param:ident: $param_ty:ty),*) $(-> $ret:ty)*
            where normal_ident = $fn_name:ident,
                  default_ret = $default_ret:expr;

        $($rest:tt)*
    ) => {
        #[inline]
        #[allow(non_snake_case)]
        pub unsafe extern "system" fn $isolation_aware_fn_name($($param: $param_ty),*) $(-> $ret)* {
            if let Some(ulp_cookie) = $($p::)+isolation_aware_prepare() {
                let result = $crate_root::$fn_name($($param),*);
                $($p::)+isolation_aware_finish(ulp_cookie, result == $default_ret);
                result
            } else {
                $default_ret
            }
        }

        __winapi_basic_isolation_aware!{
            crate_root = $crate_root;
            ia_kernel32 = $($p)::+;

            $($rest)*
        }
    };
}

#[macro_export]
macro_rules! isolation_aware_kernel32 {
    () => {mod __ia_kernel32_inner {
        extern crate winapi as __ia_kernel32_inner_winapi;
        use self::__ia_kernel32_inner_winapi::*;
        use std::{mem, ptr};

        __winbase_wrapper_isolation_aware!{
            pub fn IsolationAwareCreateActCtxW(pcActCtx: PCACTCTXW) -> HANDLE
                where normal_ident = CreateActCtxW,
                      default_ret = INVALID_HANDLE_VALUE;

            pub fn IsolationAwareReleaseActCtx(hActCtx: HANDLE)
                where normal_ident = ReleaseActCtx,
                      default_ret = ();

            pub fn IsolationAwareActivateActCtx(hActCtx: HANDLE, lpCookie: *mut ULONG_PTR) -> BOOL
                where normal_ident = ActivateActCtx,
                      default_ret = FALSE;

            pub fn IsolationAwareDeactivateActCtx(dwFlags: DWORD, ulCookie: ULONG_PTR) -> BOOL
                where normal_ident = DeactivateActCtx,
                      default_ret = FALSE;

            pub fn IsolationAwareFindActCtxSectionStringW(
                dwFlags: DWORD, lpExtensionGuid: *const GUID, ulSectionId: ULONG,
                lpStringToFind: LPCWSTR, ReturnedData: PACTCTX_SECTION_KEYED_DATA
            ) -> BOOL
                where normal_ident = FindActCtxSectionStringW,
                      default_ret = FALSE;

            pub fn IsolationAwareQueryActCtxW(
                dwFlags: DWORD, hActCtx: HANDLE, pvSubInstance: PVOID, ulInfoClass: ULONG,
                pvBuffer: PVOID, cbBuffer: SIZE_T, pcbWrittenOrRequired: *mut SIZE_T
            ) -> BOOL
                where normal_ident = QueryActCtxW,
                      default_ret = FALSE;
        }

        #[cfg(not(target_pointer_width = "64"))]
        unsafe fn load_winbase_fn(proc_name: LPCSTR) -> FARPROC {
            use std::ptr;

            // "kernel32.dll" in UCS-2
            const KERNEL32_DLL: &'static [WCHAR] = &[
                0x004B, 0x0065, 0x0072, 0x006E, 0x0065, 0x006C, 0x0033, 0x0032, 0x002E, 0x0064,
                0x006C, 0x006C, 0x0000
            ];

            static mut WINBASE_MODULE: HMODULE = 0 as HMODULE;

            if WINBASE_MODULE == 0 as HMODULE {
                WINBASE_MODULE = $crate::GetModuleHandleW(KERNEL32_DLL.as_ptr());
                if WINBASE_MODULE == 0 as HMODULE {
                    return ptr::null();
                }
            }

            $crate::GetProcAddress(WINBASE_MODULE, proc_name)
        }

        __winapi_basic_isolation_aware!{
            crate_root = $crate;
            ia_kernel32 = self;
            pub fn IsolationAwareLoadLibraryExA(
                lpLibFileName: LPCSTR, hFile: HANDLE, dwFlags: DWORD
            ) -> HMODULE
                where normal_ident = LoadLibraryExA,
                      default_ret = ptr::null_mut();
            pub fn IsolationAwareLoadLibraryExW(
                lpLibFileName: LPCWSTR, hFile: HANDLE, dwFlags: DWORD
            ) -> HMODULE
                where normal_ident = LoadLibraryExW,
                      default_ret = ptr::null_mut();
            pub fn IsolationAwareLoadLibraryA(lpLibFileName: LPCSTR) -> HMODULE
                where normal_ident = LoadLibraryA,
                      default_ret = ptr::null_mut();
            pub fn IsolationAwareLoadLibraryW(lpLibFileName: LPCWSTR) -> HMODULE
                where normal_ident = LoadLibraryW,
                      default_ret = ptr::null_mut();
        }


        #[repr(C)]
        struct ACTIVATION_CONTEXT_BASIC_INFORMATION {
            act_ctx: HANDLE,
            dw_flags: DWORD
        }

        static mut ISOLATION_AWARE_HANDLE: HANDLE = INVALID_HANDLE_VALUE;
        static mut ISOLATION_AWARE_INIT_FAILED: bool = false; // IsolationAwarePrivateT_SqbjaYRiRY
        static mut ISOLATION_AWARE_CTX_CREATED: bool = false;

        #[inline]
        pub unsafe fn isolation_aware_init_failed() -> bool {
            ISOLATION_AWARE_INIT_FAILED
        }

        #[inline]
        unsafe fn isolation_aware_init_inner() -> bool {
            // If the initialization routine has already completed we don't need to do anything else
            if ISOLATION_AWARE_HANDLE != INVALID_HANDLE_VALUE {
                return true;
            }

            const QUERY_ACTCTX_FLAG_ACTCTX_IS_ADDRESS: DWORD = 0x00000010;
            const QUERY_ACTCTX_FLAG_NO_ADDREF: DWORD = 0x80000000;

            let mut context_basic_info = mem::zeroed::<ACTIVATION_CONTEXT_BASIC_INFORMATION>();

            let query_act_ctx_result = IsolationAwareQueryActCtxW(
                QUERY_ACTCTX_FLAG_ACTCTX_IS_ADDRESS | QUERY_ACTCTX_FLAG_NO_ADDREF,
                &mut ISOLATION_AWARE_HANDLE as *mut *mut c_void as *mut c_void,
                ptr::null_mut(),
                1,
                &mut context_basic_info as *mut ACTIVATION_CONTEXT_BASIC_INFORMATION as *mut c_void,
                mem::size_of::<ACTIVATION_CONTEXT_BASIC_INFORMATION>() as SIZE_T,
                ptr::null_mut()
            );
            if query_act_ctx_result == FALSE {
                return false;
            }

            if context_basic_info.act_ctx == ptr::null_mut() {
                const GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT: DWORD = 0x00000002;
                const GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS: DWORD = 0x00000004;

                let mut self_hmod = ptr::null_mut();

                // Do we need to call this function indirectly on 32-bit? The offical headers do,
                // but some testing should be done to see if it's actually warranted.
                let get_module_handle_result = $crate::GetModuleHandleExW(
                    GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT |
                    GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS,
                    &ISOLATION_AWARE_HANDLE as *const _ as LPCWSTR,
                    &mut self_hmod
                );

                if FALSE == get_module_handle_result {
                    return false;
                }

                let mut self_module_path: [WCHAR; MAX_PATH + 1] = [0; MAX_PATH + 1];
                let get_file_name_result = $crate::GetModuleFileNameW(
                    self_hmod,
                    self_module_path.as_mut_ptr(),
                    self_module_path.len() as DWORD
                );
                if get_file_name_result == 0 ||
                   get_file_name_result >= self_module_path.len() as DWORD
                {
                    $crate::SetLastError(ERROR_BUFFER_OVERFLOW);
                    return false;
                }

                const ACTCTX_FLAG_HMODULE_VALID: DWORD = 0x00000080;
                const ACTCTX_FLAG_RESOURCE_NAME_VALID: DWORD = 0x00000008;

                let mut act_ctx = ACTCTXW {
                    cbSize: mem::size_of::<ACTCTXW>() as ULONG,
                    dwFlags: ACTCTX_FLAG_RESOURCE_NAME_VALID | ACTCTX_FLAG_HMODULE_VALID,
                    lpSource: self_module_path.as_mut_ptr(),
                    lpResourceName: 3 as ULONG_PTR as LPCWSTR,
                    hModule: self_hmod,
                    ..mem::zeroed()
                };

                context_basic_info.act_ctx = IsolationAwareCreateActCtxW(&mut act_ctx);
                if context_basic_info.act_ctx == INVALID_HANDLE_VALUE {
                    let last_error = $crate::GetLastError();
                    if last_error != ERROR_RESOURCE_DATA_NOT_FOUND &&
                       last_error != ERROR_RESOURCE_TYPE_NOT_FOUND &&
                       last_error != ERROR_RESOURCE_LANG_NOT_FOUND &&
                       last_error != ERROR_RESOURCE_NAME_NOT_FOUND &&
                       last_error != ERROR_FILE_NOT_FOUND &&
                       last_error != ERROR_PATH_NOT_FOUND
                    {
                        return false;
                    }

                    context_basic_info.act_ctx = ptr::null_mut();
                }
                ISOLATION_AWARE_CTX_CREATED = true;
            }

            ISOLATION_AWARE_HANDLE = context_basic_info.act_ctx;

            const ACTIVATION_CONTEXT_SECTION_DLL_REDIRECTION: ULONG  = 2;

            let mut ulp_cookie: ULONG_PTR = 0;
            if TRUE == IsolationAwareActivateActCtx(context_basic_info.act_ctx, &mut ulp_cookie) {
                let mut ctx_keyed_data = ACTCTX_SECTION_KEYED_DATA {
                    cbSize: mem::size_of::<ACTCTX_SECTION_KEYED_DATA>() as ULONG,
                    ..mem::zeroed()
                };

                // "Comctl32.dll" in UCS-2
                const COMCTL32_DLL: &'static [WCHAR] = &[
                    0x0043, 0x006F, 0x006D, 0x0063, 0x0074, 0x006C, 0x0033, 0x0032, 0x002E, 0x0064,
                    0x006C, 0x006C, 0x0000
                ];

                if TRUE == IsolationAwareFindActCtxSectionStringW(
                    0,
                    ptr::null(),
                    ACTIVATION_CONTEXT_SECTION_DLL_REDIRECTION,
                    COMCTL32_DLL.as_ptr(),
                    &mut ctx_keyed_data
                ) {
                    $crate::LoadLibraryW(COMCTL32_DLL.as_ptr());
                }

                IsolationAwareDeactivateActCtx(0, ulp_cookie);
            }

            true
        }

        #[doc(hidden)]
        pub unsafe fn isolation_aware_prepare() -> Option<ULONG_PTR> {
            let mut ulp_cookie = 0;
            // If cleanup is ever implemented, don't call `isolation_aware_init_inner` or
            // `IsolationAwareActivateActCtx` after cleanup.
            let succeeded =
                ISOLATION_AWARE_INIT_FAILED || (
                    isolation_aware_init_inner() &&
                    IsolationAwareActivateActCtx(ISOLATION_AWARE_HANDLE, &mut ulp_cookie) == TRUE
                );

            if succeeded {
                Some(ulp_cookie)
            } else {
                let last_error = $crate::GetLastError();
                if last_error == ERROR_PROC_NOT_FOUND ||
                   last_error == ERROR_MOD_NOT_FOUND ||
                   last_error == ERROR_CALL_NOT_IMPLEMENTED ||
                   last_error == ERROR_INVALID_FUNCTION ||
                   last_error == ERROR_NOT_SUPPORTED
                {
                    ISOLATION_AWARE_INIT_FAILED = true;
                    Some(ulp_cookie)
                } else {
                    assert_eq!(0, ulp_cookie);
                    None
                }
            }
        }

        #[doc(hidden)]
        pub unsafe fn isolation_aware_finish(ulp_cookie: ULONG_PTR, preserve_last_err: bool) {
            if ISOLATION_AWARE_INIT_FAILED == false {
                let last_error = if preserve_last_err {$crate::GetLastError()} else {NO_ERROR};
                IsolationAwareDeactivateActCtx(0, ulp_cookie);
                if preserve_last_err {
                    $crate::SetLastError(last_error);
                }
            }
        }
    }
    pub use self::__ia_kernel32_inner::*;
    }
}

#[cfg(test)]
isolation_aware_kernel32!();

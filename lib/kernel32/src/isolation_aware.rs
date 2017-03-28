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
    (crate_root = $crate_root:ident; ia_kernel32 = $p:path;) => ();
    (
        crate_root = $crate_root:ident;
        ia_kernel32 = $p:path;

        pub fn $isolation_aware_fn_name:ident($($param:ident: $param_ty:ty),*) $(-> $ret:ty)*
            where normal_ident = $fn_name:ident,
                  default_ret = $default_ret:expr;

        $($rest:tt)*
    ) => {
        #[inline]
        #[allow(non_snake_case)]
        pub unsafe extern "system" fn $isolation_aware_fn_name($($param: $param_ty),*) $(-> $ret)* {
            use $p::{isolation_aware_prepare, isolation_aware_finish};

            if let Some(ulp_cookie) = isolation_aware_prepare() {
                let result = $crate_root::$fn_name($($param),*);
                isolation_aware_finish(ulp_cookie, result == $default_ret);
                result
            } else {
                $default_ret
            }
        }

        __winapi_basic_isolation_aware!{
            crate_root = $crate_root;
            ia_kernel32 = $p;

            $($rest)*
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __winapi_comstar_isolation_aware {
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

            if let Some(ulp_cookie) = ::$($p::)+isolation_aware_prepare() {
                if PFN as *const () == default_pfn as FnType as *const () {
                    let pfn = load_comctl_fn(
                        concat!(stringify!($fn_name), "\0").as_ptr() as *const c_char
                    );

                    if pfn != ptr::null() {
                        PFN = mem::transmute(pfn);
                    } else {
                        ::$($p::)+isolation_aware_finish(ulp_cookie, true);
                        return $default_ret;
                    }
                }

                let ret = PFN($($param),*);
                ::$($p::)+isolation_aware_finish(ulp_cookie, false);
                ret
            } else {
                $default_ret
            }
        }

        __winapi_comstar_isolation_aware!{
            ia_kernel32 = $($p)::+;

            $($rest)*
        }
    }
}

/// Create [isolation-aware][] kernel32 bindings.
///
/// # Usage
///
/// ```ignore
/// mod kernel32 {
///     isolation_aware_kernel32!();
/// }
/// ```
///
/// Generally, this macro should be invoked in its own module, and it shouldn't be invoked more than
/// once in a crate. Nothing particularly bad will happen if it's invoked multiple times, but it
/// would add a bit of unnecessary resource waste.
///
/// # Forms
/// * `isolation_aware_kernel32!()`: Generate isolation-aware functions using the `RT_MANIFEST`
///   manifest specified in the invoking crate's resource file.
/// * `isolation_aware_kernel32!($create_activation_ctx)`: Generate isolation-aware functions using
///   the [activation context][] handle returned by the `$create_activation_ctx` expression.
///
/// [activation context]: https://msdn.microsoft.com/en-us/library/windows/desktop/aa374166.aspx
/// [isolation-aware]: https://msdn.microsoft.com/en-us/library/windows/desktop/aa375197.aspx
#[macro_export]
macro_rules! isolation_aware_kernel32 {
    () => {isolation_aware_kernel32!{{
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
            lpResourceName: 124 as ULONG_PTR as LPCWSTR,
            hModule: self_hmod,
            ..mem::zeroed()
        };

        IsolationAwareCreateActCtxW(&mut act_ctx)
    }}};
    ($create_activation_ctx:expr) => {mod __ia_kernel32_inner {
        #![allow(dead_code)]
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
                context_basic_info.act_ctx = $create_activation_ctx;

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

        pub use self::{
            IsolationAwareCreateActCtxW as CreateActCtxW,
            IsolationAwareReleaseActCtx as ReleaseActCtx,
            IsolationAwareActivateActCtx as ActivateActCtx,
            IsolationAwareDeactivateActCtx as DeactivateActCtx,
            IsolationAwareFindActCtxSectionStringW as FindActCtxSectionStringW,
            IsolationAwareQueryActCtxW as QueryActCtxW,
            IsolationAwareLoadLibraryExA as LoadLibraryExA,
            IsolationAwareLoadLibraryExW as LoadLibraryExW,
            IsolationAwareLoadLibraryA as LoadLibraryA,
            IsolationAwareLoadLibraryW as LoadLibraryW
        };

        #[cfg(target_arch = "x86")]
        pub use $crate::{
            InterlockedCompareExchange,
            InterlockedCompareExchange64,
            InterlockedDecrement,
            InterlockedExchange,
            InterlockedExchangeAdd,
            InterlockedIncrement
        };

        pub use $crate::{
            AcquireSRWLockExclusive,
            AcquireSRWLockShared,
            AddAtomA,
            AddAtomW,
            AddConsoleAliasA,
            AddConsoleAliasW,
            AddDllDirectory,
            AddIntegrityLabelToBoundaryDescriptor,
            // AddLocalAlternateComputerNameA,
            // AddLocalAlternateComputerNameW,
            AddRefActCtx,
            AddResourceAttributeAce,
            AddSIDToBoundaryDescriptor,
            AddScopedPolicyIDAce,
            AddSecureMemoryCacheCallback,
            AddVectoredContinueHandler,
            AddVectoredExceptionHandler,
            AllocConsole,
            AllocateUserPhysicalPages,
            AllocateUserPhysicalPagesNuma,
            // AppXGetOSMaxVersionTested,
            ApplicationRecoveryFinished,
            ApplicationRecoveryInProgress,
            AreFileApisANSI,
            AssignProcessToJobObject,
            AttachConsole,
            BackupRead,
            BackupSeek,
            BackupWrite,
            // BaseSetLastNTError,
            Beep,
            BeginUpdateResourceA,
            BeginUpdateResourceW,
            BindIoCompletionCallback,
            BuildCommDCBA,
            BuildCommDCBAndTimeoutsA,
            BuildCommDCBAndTimeoutsW,
            BuildCommDCBW,
            CallNamedPipeA,
            CallNamedPipeW,
            CallbackMayRunLong,
            CalloutOnFiberStack,
            CancelDeviceWakeupRequest,
            CancelIo,
            CancelIoEx,
            CancelSynchronousIo,
            CancelThreadpoolIo,
            CancelTimerQueueTimer,
            CancelWaitableTimer,
            CeipIsOptedIn,
            ChangeTimerQueueTimer,
            // CheckElevation,
            // CheckElevationEnabled,
            CheckNameLegalDOS8Dot3A,
            CheckNameLegalDOS8Dot3W,
            CheckRemoteDebuggerPresent,
            CheckTokenCapability,
            CheckTokenMembershipEx,
            ClearCommBreak,
            ClearCommError,
            CloseHandle,
            // ClosePackageInfo,
            ClosePrivateNamespace,
            // CloseState,
            CloseThreadpool,
            CloseThreadpoolCleanupGroup,
            CloseThreadpoolCleanupGroupMembers,
            CloseThreadpoolIo,
            CloseThreadpoolTimer,
            CloseThreadpoolWait,
            CloseThreadpoolWork,
            CommConfigDialogA,
            CommConfigDialogW,
            CompareFileTime,
            CompareStringA,
            CompareStringEx,
            CompareStringOrdinal,
            CompareStringW,
            ConnectNamedPipe,
            ContinueDebugEvent,
            ConvertDefaultLocale,
            ConvertFiberToThread,
            ConvertThreadToFiber,
            ConvertThreadToFiberEx,
            CopyContext,
            CopyFile2,
            CopyFileA,
            CopyFileExA,
            CopyFileExW,
            CopyFileTransactedA,
            CopyFileTransactedW,
            CopyFileW,
            CreateActCtxA,
            CreateBoundaryDescriptorA,
            CreateBoundaryDescriptorW,
            CreateConsoleScreenBuffer,
            CreateDirectoryA,
            CreateDirectoryExA,
            CreateDirectoryExW,
            CreateDirectoryTransactedA,
            CreateDirectoryTransactedW,
            CreateDirectoryW,
            CreateEventA,
            CreateEventW,
            CreateEventExA,
            CreateEventExW,
            CreateFiber,
            CreateFiberEx,
            CreateFile2,
            CreateFileA,
            CreateFileMappingA,
            CreateFileMappingFromApp,
            CreateFileMappingNumaA,
            CreateFileMappingNumaW,
            CreateFileMappingW,
            CreateFileTransactedA,
            CreateFileTransactedW,
            CreateFileW,
            CreateHardLinkA,
            CreateHardLinkTransactedA,
            CreateHardLinkTransactedW,
            CreateHardLinkW,
            CreateIoCompletionPort,
            CreateJobObjectA,
            CreateJobObjectW,
            CreateJobSet,
            CreateMailslotA,
            CreateMailslotW,
            CreateMemoryResourceNotification,
            CreateMutexA,
            CreateMutexExA,
            CreateMutexExW,
            CreateMutexW,
            CreateNamedPipeA,
            CreateNamedPipeW,
            CreatePipe,
            CreatePrivateNamespaceA,
            CreatePrivateNamespaceW,
            CreateProcessA,
            CreateProcessW,
            CreateRemoteThread,
            CreateRemoteThreadEx,
            CreateSemaphoreA,
            CreateSemaphoreExA,
            CreateSemaphoreExW,
            CreateSemaphoreW,
            CreateSymbolicLinkA,
            CreateSymbolicLinkTransactedA,
            CreateSymbolicLinkTransactedW,
            CreateSymbolicLinkW,
            CreateTapePartition,
            CreateThread,
            CreateThreadpool,
            CreateThreadpoolCleanupGroup,
            CreateThreadpoolIo,
            CreateThreadpoolTimer,
            CreateThreadpoolWait,
            CreateThreadpoolWork,
            CreateTimerQueue,
            CreateTimerQueueTimer,
            CreateToolhelp32Snapshot,
            CreateUmsCompletionList,
            CreateUmsThreadContext,
            CreateWaitableTimerA,
            CreateWaitableTimerExA,
            CreateWaitableTimerExW,
            CreateWaitableTimerW,
            // CtrlRoutine,
            DebugActiveProcess,
            DebugActiveProcessStop,
            DebugBreak,
            DebugBreakProcess,
            DebugSetProcessKillOnExit,
            DecodePointer,
            DecodeSystemPointer,
            DefineDosDeviceA,
            DefineDosDeviceW,
            DelayLoadFailureHook,
            DeleteAtom,
            DeleteBoundaryDescriptor,
            DeleteCriticalSection,
            DeleteFiber,
            DeleteFileA,
            DeleteFileTransactedA,
            DeleteFileTransactedW,
            DeleteFileW,
            DeleteProcThreadAttributeList,
            DeleteSynchronizationBarrier,
            DeleteTimerQueue,
            DeleteTimerQueueEx,
            DeleteTimerQueueTimer,
            DeleteUmsCompletionList,
            DeleteUmsThreadContext,
            DeleteVolumeMountPointA,
            DeleteVolumeMountPointW,
            DequeueUmsCompletionListItems,
            DeviceIoControl,
            DisableThreadLibraryCalls,
            DisableThreadProfiling,
            DisassociateCurrentThreadFromCallback,
            DisconnectNamedPipe,
            DnsHostnameToComputerNameA,
            DnsHostnameToComputerNameExW,
            DnsHostnameToComputerNameW,
            DosDateTimeToFileTime,
            // DosPathToSessionPathW,
            DuplicateHandle,
            EnableThreadProfiling,
            EncodePointer,
            EncodeSystemPointer,
            EndUpdateResourceA,
            EndUpdateResourceW,
            EnterCriticalSection,
            EnterSynchronizationBarrier,
            EnterUmsSchedulingMode,
            EnumCalendarInfoA,
            EnumCalendarInfoExA,
            EnumCalendarInfoExEx,
            EnumCalendarInfoExW,
            EnumCalendarInfoW,
            EnumDateFormatsA,
            EnumDateFormatsExA,
            EnumDateFormatsExEx,
            EnumDateFormatsExW,
            EnumDateFormatsW,
            EnumLanguageGroupLocalesA,
            EnumLanguageGroupLocalesW,
            EnumResourceLanguagesA,
            EnumResourceLanguagesExA,
            EnumResourceLanguagesExW,
            EnumResourceLanguagesW,
            EnumResourceNamesA,
            EnumResourceNamesExA,
            EnumResourceNamesExW,
            EnumResourceNamesW,
            EnumResourceTypesA,
            EnumResourceTypesExA,
            EnumResourceTypesExW,
            EnumResourceTypesW,
            EnumSystemCodePagesA,
            EnumSystemCodePagesW,
            EnumSystemFirmwareTables,
            EnumSystemGeoID,
            EnumSystemLanguageGroupsA,
            EnumSystemLanguageGroupsW,
            EnumSystemLocalesA,
            EnumSystemLocalesEx,
            EnumSystemLocalesW,
            EnumTimeFormatsA,
            EnumTimeFormatsEx,
            EnumTimeFormatsW,
            EnumUILanguagesA,
            EnumUILanguagesW,
            // EnumerateLocalComputerNamesA,
            // EnumerateLocalComputerNamesW,
            EraseTape,
            EscapeCommFunction,
            ExecuteUmsThread,
            ExitProcess,
            ExitThread,
            ExpandEnvironmentStringsA,
            ExpandEnvironmentStringsW,
            FatalAppExitA,
            FatalAppExitW,
            FatalExit,
            FileTimeToDosDateTime,
            FileTimeToLocalFileTime,
            FileTimeToSystemTime,
            FillConsoleOutputAttribute,
            FillConsoleOutputCharacterA,
            FillConsoleOutputCharacterW,
            FindActCtxSectionGuid,
            FindActCtxSectionStringA,
            FindAtomA,
            FindAtomW,
            FindClose,
            FindCloseChangeNotification,
            FindFirstChangeNotificationA,
            FindFirstChangeNotificationW,
            FindFirstFileA,
            FindFirstFileExA,
            FindFirstFileExW,
            FindFirstFileNameTransactedW,
            FindFirstFileNameW,
            FindFirstFileTransactedA,
            FindFirstFileTransactedW,
            FindFirstFileW,
            FindFirstStreamTransactedW,
            FindFirstStreamW,
            FindFirstVolumeA,
            FindFirstVolumeMountPointA,
            FindFirstVolumeMountPointW,
            FindFirstVolumeW,
            FindNLSString,
            FindNLSStringEx,
            FindNextChangeNotification,
            FindNextFileA,
            FindNextFileNameW,
            FindNextFileW,
            FindNextStreamW,
            FindNextVolumeA,
            FindNextVolumeMountPointA,
            FindNextVolumeMountPointW,
            FindNextVolumeW,
            // FindPackagesByPackageFamily,
            FindResourceA,
            FindResourceExA,
            FindResourceExW,
            FindResourceW,
            FindStringOrdinal,
            FindVolumeClose,
            FindVolumeMountPointClose,
            FlsAlloc,
            FlsFree,
            FlsGetValue,
            FlsSetValue,
            FlushConsoleInputBuffer,
            FlushFileBuffers,
            FlushInstructionCache,
            FlushProcessWriteBuffers,
            FlushViewOfFile,
            FoldStringA,
            FoldStringW,
            // FormatApplicationUserModelId,
            FormatMessageA,
            FormatMessageW,
            FreeConsole,
            FreeEnvironmentStringsA,
            FreeEnvironmentStringsW,
            FreeLibrary,
            FreeLibraryAndExitThread,
            FreeLibraryWhenCallbackReturns,
            FreeResource,
            FreeUserPhysicalPages,
            GenerateConsoleCtrlEvent,
            GetACP,
            GetActiveProcessorCount,
            GetActiveProcessorGroupCount,
            GetAppContainerAce,
            GetAppContainerNamedObjectPath,
            GetApplicationRecoveryCallback,
            GetApplicationRestartSettings,
            // GetApplicationUserModelId,
            GetAtomNameA,
            GetAtomNameW,
            GetBinaryTypeA,
            GetBinaryTypeW,
            GetCPInfo,
            GetCPInfoExA,
            GetCPInfoExW,
            GetCachedSigningLevel,
            GetCalendarInfoA,
            GetCalendarInfoEx,
            GetCalendarInfoW,
            GetCommConfig,
            GetCommMask,
            GetCommModemStatus,
            GetCommProperties,
            GetCommState,
            GetCommTimeouts,
            GetCommandLineA,
            GetCommandLineW,
            GetCompressedFileSizeA,
            GetCompressedFileSizeTransactedA,
            GetCompressedFileSizeTransactedW,
            GetCompressedFileSizeW,
            GetComputerNameA,
            GetComputerNameExA,
            GetComputerNameExW,
            GetComputerNameW,
            GetConsoleAliasA,
            GetConsoleAliasExesA,
            GetConsoleAliasExesLengthA,
            GetConsoleAliasExesLengthW,
            GetConsoleAliasExesW,
            GetConsoleAliasW,
            GetConsoleAliasesA,
            GetConsoleAliasesLengthA,
            GetConsoleAliasesLengthW,
            GetConsoleAliasesW,
            GetConsoleCP,
            GetConsoleCursorInfo,
            GetConsoleDisplayMode,
            GetConsoleFontSize,
            GetConsoleHistoryInfo,
            GetConsoleMode,
            GetConsoleOriginalTitleA,
            GetConsoleOriginalTitleW,
            GetConsoleOutputCP,
            GetConsoleProcessList,
            GetConsoleScreenBufferInfo,
            GetConsoleScreenBufferInfoEx,
            GetConsoleSelectionInfo,
            GetConsoleTitleA,
            GetConsoleTitleW,
            GetConsoleWindow,
            GetCurrencyFormatA,
            GetCurrencyFormatEx,
            GetCurrencyFormatW,
            GetCurrentActCtx,
            // GetCurrentApplicationUserModelId,
            GetCurrentConsoleFont,
            GetCurrentConsoleFontEx,
            GetCurrentDirectoryA,
            GetCurrentDirectoryW,
            // GetCurrentPackageFamilyName,
            // GetCurrentPackageFullName,
            // GetCurrentPackageId,
            // GetCurrentPackageInfo,
            // GetCurrentPackagePath,
            GetCurrentProcess,
            GetCurrentProcessId,
            GetCurrentProcessorNumber,
            GetCurrentProcessorNumberEx,
            GetCurrentThread,
            GetCurrentThreadId,
            GetCurrentThreadStackLimits,
            GetCurrentUmsThread,
            GetDateFormatA,
            GetDateFormatEx,
            GetDateFormatW,
            GetDefaultCommConfigA,
            GetDefaultCommConfigW,
            GetDevicePowerState,
            GetDiskFreeSpaceA,
            GetDiskFreeSpaceExA,
            GetDiskFreeSpaceExW,
            GetDiskFreeSpaceW,
            GetDllDirectoryA,
            GetDllDirectoryW,
            GetDriveTypeA,
            GetDriveTypeW,
            GetDurationFormat,
            GetDurationFormatEx,
            GetDynamicTimeZoneInformation,
            GetEnabledXStateFeatures,
            GetEnvironmentStrings,
            GetEnvironmentStringsW,
            GetEnvironmentVariableA,
            GetEnvironmentVariableW,
            // GetEraNameCountedString,
            GetErrorMode,
            GetExitCodeProcess,
            GetExitCodeThread,
            GetFileAttributesA,
            GetFileAttributesExA,
            GetFileAttributesExW,
            GetFileAttributesTransactedA,
            GetFileAttributesTransactedW,
            GetFileAttributesW,
            GetFileBandwidthReservation,
            GetFileInformationByHandle,
            GetFileInformationByHandleEx,
            GetFileMUIInfo,
            GetFileMUIPath,
            GetFileSize,
            GetFileSizeEx,
            GetFileTime,
            GetFileType,
            GetFinalPathNameByHandleA,
            GetFinalPathNameByHandleW,
            GetFirmwareEnvironmentVariableA,
            GetFirmwareEnvironmentVariableExA,
            GetFirmwareEnvironmentVariableExW,
            GetFirmwareEnvironmentVariableW,
            GetFirmwareType,
            GetFullPathNameA,
            GetFullPathNameTransactedA,
            GetFullPathNameTransactedW,
            GetFullPathNameW,
            GetGeoInfoA,
            GetGeoInfoW,
            GetHandleInformation,
            GetLargePageMinimum,
            GetLargestConsoleWindowSize,
            GetLastError,
            GetLocalTime,
            GetLocaleInfoA,
            GetLocaleInfoEx,
            GetLocaleInfoW,
            GetLogicalDriveStringsA,
            GetLogicalDriveStringsW,
            GetLogicalDrives,
            GetLogicalProcessorInformation,
            GetLogicalProcessorInformationEx,
            GetLongPathNameA,
            GetLongPathNameTransactedA,
            GetLongPathNameTransactedW,
            GetLongPathNameW,
            GetMailslotInfo,
            GetMaximumProcessorCount,
            GetMaximumProcessorGroupCount,
            GetMemoryErrorHandlingCapabilities,
            GetModuleFileNameA,
            GetModuleFileNameW,
            GetModuleHandleA,
            GetModuleHandleExA,
            GetModuleHandleExW,
            GetModuleHandleW,
            GetNLSVersion,
            GetNLSVersionEx,
            // GetNamedPipeAttribute,
            GetNamedPipeClientComputerNameA,
            GetNamedPipeClientComputerNameW,
            GetNamedPipeClientProcessId,
            GetNamedPipeClientSessionId,
            GetNamedPipeHandleStateA,
            GetNamedPipeHandleStateW,
            GetNamedPipeInfo,
            GetNamedPipeServerProcessId,
            GetNamedPipeServerSessionId,
            GetNativeSystemInfo,
            GetNextUmsListItem,
            GetNumaAvailableMemoryNode,
            GetNumaAvailableMemoryNodeEx,
            GetNumaHighestNodeNumber,
            GetNumaNodeNumberFromHandle,
            GetNumaNodeProcessorMask,
            GetNumaNodeProcessorMaskEx,
            GetNumaProcessorNode,
            GetNumaProcessorNodeEx,
            GetNumaProximityNode,
            GetNumaProximityNodeEx,
            GetNumberFormatA,
            GetNumberFormatEx,
            GetNumberFormatW,
            GetNumberOfConsoleInputEvents,
            GetNumberOfConsoleMouseButtons,
            GetOEMCP,
            GetOverlappedResult,
            GetOverlappedResultEx,
            // GetPackageApplicationIds,
            // GetPackageFamilyName,
            // GetPackageFullName,
            // GetPackageId,
            // GetPackageInfo,
            // GetPackagePath,
            // GetPackagePathByFullName,
            // GetPackagesByPackageFamily,
            GetPhysicallyInstalledSystemMemory,
            GetPriorityClass,
            GetPrivateProfileIntA,
            GetPrivateProfileIntW,
            GetPrivateProfileSectionA,
            GetPrivateProfileSectionNamesA,
            GetPrivateProfileSectionNamesW,
            GetPrivateProfileSectionW,
            GetPrivateProfileStringA,
            GetPrivateProfileStringW,
            GetPrivateProfileStructA,
            GetPrivateProfileStructW,
            GetProcAddress,
            GetProcessAffinityMask,
            GetProcessDEPPolicy,
            GetProcessGroupAffinity,
            GetProcessHandleCount,
            GetProcessHeap,
            GetProcessHeaps,
            GetProcessId,
            GetProcessIdOfThread,
            GetProcessInformation,
            GetProcessIoCounters,
            GetProcessMitigationPolicy,
            GetProcessPreferredUILanguages,
            GetProcessPriorityBoost,
            GetProcessShutdownParameters,
            GetProcessTimes,
            GetProcessVersion,
            GetProcessWorkingSetSize,
            GetProcessWorkingSetSizeEx,
            GetProcessorSystemCycleTime,
            GetProductInfo,
            GetProfileIntA,
            GetProfileIntW,
            GetProfileSectionA,
            GetProfileSectionW,
            GetProfileStringA,
            GetProfileStringW,
            GetQueuedCompletionStatus,
            GetQueuedCompletionStatusEx,
            GetShortPathNameA,
            GetShortPathNameW,
            // GetStagedPackagePathByFullName,
            GetStartupInfoA,
            GetStartupInfoW,
            // GetStateFolder,
            GetStdHandle,
            GetStringScripts,
            GetStringTypeA,
            GetStringTypeExA,
            GetStringTypeExW,
            GetStringTypeW,
            // GetSystemAppDataKey,
            GetSystemDEPPolicy,
            GetSystemDefaultLCID,
            GetSystemDefaultLangID,
            GetSystemDefaultLocaleName,
            GetSystemDefaultUILanguage,
            GetSystemDirectoryA,
            GetSystemDirectoryW,
            GetSystemFileCacheSize,
            GetSystemFirmwareTable,
            GetSystemInfo,
            GetSystemPowerStatus,
            GetSystemPreferredUILanguages,
            GetSystemRegistryQuota,
            GetSystemTime,
            GetSystemTimeAdjustment,
            GetSystemTimeAsFileTime,
            GetSystemTimePreciseAsFileTime,
            GetSystemTimes,
            GetSystemWindowsDirectoryA,
            GetSystemWindowsDirectoryW,
            GetSystemWow64DirectoryA,
            GetSystemWow64DirectoryW,
            GetTapeParameters,
            GetTapePosition,
            GetTapeStatus,
            GetTempFileNameA,
            GetTempFileNameW,
            GetTempPathA,
            GetTempPathW,
            GetThreadContext,
            GetThreadErrorMode,
            GetThreadGroupAffinity,
            GetThreadIOPendingFlag,
            GetThreadId,
            GetThreadIdealProcessorEx,
            GetThreadInformation,
            GetThreadLocale,
            GetThreadPreferredUILanguages,
            GetThreadPriority,
            GetThreadPriorityBoost,
            GetThreadSelectorEntry,
            GetThreadTimes,
            GetThreadUILanguage,
            GetTickCount,
            GetTickCount64,
            GetTimeFormatA,
            GetTimeFormatEx,
            GetTimeFormatW,
            GetTimeZoneInformation,
            GetTimeZoneInformationForYear,
            GetUILanguageInfo,
            GetUmsCompletionListEvent,
            GetUmsSystemThreadInformation,
            GetUserDefaultLCID,
            GetUserDefaultLangID,
            GetUserDefaultLocaleName,
            GetUserDefaultUILanguage,
            GetUserGeoID,
            GetUserPreferredUILanguages,
            GetVersion,
            GetVersionExA,
            GetVersionExW,
            GetVolumeInformationA,
            GetVolumeInformationByHandleW,
            GetVolumeInformationW,
            GetVolumeNameForVolumeMountPointA,
            GetVolumeNameForVolumeMountPointW,
            GetVolumePathNameA,
            GetVolumePathNameW,
            GetVolumePathNamesForVolumeNameA,
            GetVolumePathNamesForVolumeNameW,
            GetWindowsDirectoryA,
            GetWindowsDirectoryW,
            GetWriteWatch,
            GetXStateFeaturesMask,
            GlobalAddAtomA,
            GlobalAddAtomExA,
            GlobalAddAtomExW,
            GlobalAddAtomW,
            GlobalAlloc,
            GlobalCompact,
            GlobalDeleteAtom,
            GlobalFindAtomA,
            GlobalFindAtomW,
            GlobalFix,
            GlobalFlags,
            GlobalFree,
            GlobalGetAtomNameA,
            GlobalGetAtomNameW,
            GlobalHandle,
            GlobalLock,
            GlobalMemoryStatus,
            GlobalMemoryStatusEx,
            GlobalReAlloc,
            GlobalSize,
            GlobalUnWire,
            GlobalUnfix,
            GlobalUnlock,
            GlobalWire,
            Heap32First,
            Heap32ListFirst,
            Heap32ListNext,
            Heap32Next,
            HeapAlloc,
            HeapCompact,
            HeapCreate,
            HeapDestroy,
            HeapFree,
            HeapLock,
            HeapQueryInformation,
            HeapReAlloc,
            HeapSetInformation,
            HeapSize,
            HeapSummary,
            HeapUnlock,
            HeapValidate,
            HeapWalk,
            InitAtomTable,
            InitOnceBeginInitialize,
            InitOnceComplete,
            InitOnceExecuteOnce,
            InitOnceInitialize,
            InitializeConditionVariable,
            InitializeContext,
            InitializeCriticalSection,
            InitializeCriticalSectionAndSpinCount,
            InitializeCriticalSectionEx,
            InitializeProcThreadAttributeList,
            InitializeSListHead,
            InitializeSRWLock,
            InitializeSynchronizationBarrier,
            InstallELAMCertificateInfo,
            InterlockedFlushSList,
            InterlockedPopEntrySList,
            InterlockedPushEntrySList,
            InterlockedPushListSListEx,
            IsBadCodePtr,
            IsBadHugeReadPtr,
            IsBadHugeWritePtr,
            IsBadReadPtr,
            IsBadStringPtrA,
            IsBadStringPtrW,
            IsBadWritePtr,
            IsDBCSLeadByte,
            IsDBCSLeadByteEx,
            IsDebuggerPresent,
            IsNLSDefinedString,
            IsNativeVhdBoot,
            IsNormalizedString,
            IsProcessCritical,
            IsProcessInJob,
            IsProcessorFeaturePresent,
            IsSystemResumeAutomatic,
            IsThreadAFiber,
            IsThreadpoolTimerSet,
            IsValidCodePage,
            IsValidLanguageGroup,
            IsValidLocale,
            IsValidLocaleName,
            IsValidNLSVersion,
            IsWow64Process,
            K32EmptyWorkingSet,
            K32EnumDeviceDrivers,
            K32EnumPageFilesA,
            K32EnumPageFilesW,
            K32EnumProcessModules,
            K32EnumProcessModulesEx,
            K32EnumProcesses,
            K32GetDeviceDriverBaseNameA,
            K32GetDeviceDriverBaseNameW,
            K32GetDeviceDriverFileNameA,
            K32GetDeviceDriverFileNameW,
            K32GetMappedFileNameA,
            K32GetMappedFileNameW,
            K32GetModuleBaseNameA,
            K32GetModuleBaseNameW,
            K32GetModuleFileNameExA,
            K32GetModuleFileNameExW,
            K32GetModuleInformation,
            K32GetPerformanceInfo,
            K32GetProcessImageFileNameA,
            K32GetProcessImageFileNameW,
            K32GetProcessMemoryInfo,
            K32GetWsChanges,
            K32GetWsChangesEx,
            K32InitializeProcessForWsWatch,
            K32QueryWorkingSet,
            K32QueryWorkingSetEx,
            LCIDToLocaleName,
            LCMapStringA,
            LCMapStringEx,
            LCMapStringW,
            LeaveCriticalSection,
            LeaveCriticalSectionWhenCallbackReturns,
            // LoadAppInitDlls,
            LoadModule,
            LoadPackagedLibrary,
            LoadResource,
            // LoadStringBaseExW,
            // LoadStringBaseW,
            LocalAlloc,
            LocalCompact,
            LocalFileTimeToFileTime,
            LocalFlags,
            LocalFree,
            LocalHandle,
            LocalLock,
            LocalReAlloc,
            LocalShrink,
            LocalSize,
            LocalUnlock,
            LocaleNameToLCID,
            LocateXStateFeature,
            LockFile,
            LockFileEx,
            LockResource,
            MapUserPhysicalPages,
            MapUserPhysicalPagesScatter,
            MapViewOfFile,
            MapViewOfFileEx,
            MapViewOfFileExNuma,
            MapViewOfFileFromApp,
            Module32First,
            Module32FirstW,
            Module32Next,
            Module32NextW,
            MoveFileA,
            MoveFileExA,
            MoveFileExW,
            MoveFileTransactedA,
            MoveFileTransactedW,
            MoveFileW,
            MoveFileWithProgressA,
            MoveFileWithProgressW,
            MulDiv,
            MultiByteToWideChar,
            NeedCurrentDirectoryForExePathA,
            NeedCurrentDirectoryForExePathW,
            NormalizeString,
            // NotifyMountMgr,
            NotifyUILanguageChange,
            // OOBEComplete,
            OpenEventA,
            OpenEventW,
            OpenFile,
            OpenFileById,
            OpenFileMappingA,
            OpenFileMappingW,
            OpenJobObjectA,
            OpenJobObjectW,
            OpenMutexA,
            OpenMutexW,
            // OpenPackageInfoByFullName,
            OpenPrivateNamespaceA,
            OpenPrivateNamespaceW,
            OpenProcess,
            OpenSemaphoreA,
            OpenSemaphoreW,
            // OpenState,
            // OpenStateExplicit,
            OpenThread,
            OpenWaitableTimerA,
            OpenWaitableTimerW,
            OutputDebugStringA,
            OutputDebugStringW,
            // PackageFamilyNameFromFullName,
            // PackageFamilyNameFromId,
            // PackageFullNameFromId,
            // PackageIdFromFullName,
            // PackageNameAndPublisherIdFromFamilyName,
            // ParseApplicationUserModelId,
            PeekConsoleInputA,
            PeekConsoleInputW,
            PeekNamedPipe,
            PostQueuedCompletionStatus,
            PowerClearRequest,
            PowerCreateRequest,
            PowerSetRequest,
            PrefetchVirtualMemory,
            PrepareTape,
            Process32First,
            Process32FirstW,
            Process32Next,
            Process32NextW,
            ProcessIdToSessionId,
            PssCaptureSnapshot,
            PssDuplicateSnapshot,
            PssFreeSnapshot,
            PssQuerySnapshot,
            PssWalkMarkerCreate,
            PssWalkMarkerFree,
            PssWalkMarkerGetPosition,
            // PssWalkMarkerRewind,
            // PssWalkMarkerSeek,
            PssWalkMarkerSeekToBeginning,
            PssWalkMarkerSetPosition,
            // PssWalkMarkerTell,
            PssWalkSnapshot,
            PulseEvent,
            PurgeComm,
            QueryActCtxSettingsW,
            QueryDepthSList,
            QueryDosDeviceA,
            QueryDosDeviceW,
            QueryFullProcessImageNameA,
            QueryFullProcessImageNameW,
            QueryIdleProcessorCycleTime,
            QueryIdleProcessorCycleTimeEx,
            QueryInformationJobObject,
            QueryMemoryResourceNotification,
            QueryPerformanceCounter,
            QueryPerformanceFrequency,
            QueryProcessAffinityUpdateMode,
            QueryProcessCycleTime,
            QueryProtectedPolicy,
            QueryThreadCycleTime,
            QueryThreadProfiling,
            QueryThreadpoolStackInformation,
            QueryUmsThreadInformation,
            QueryUnbiasedInterruptTime,
            QueueUserAPC,
            QueueUserWorkItem,
            RaiseException,
            RaiseFailFastException,
            ReOpenFile,
            ReadConsoleA,
            ReadConsoleInputA,
            ReadConsoleInputW,
            ReadConsoleOutputA,
            ReadConsoleOutputAttribute,
            ReadConsoleOutputCharacterA,
            ReadConsoleOutputCharacterW,
            ReadConsoleOutputW,
            ReadConsoleW,
            ReadDirectoryChangesW,
            ReadFile,
            ReadFileEx,
            ReadFileScatter,
            ReadProcessMemory,
            ReadThreadProfilingData,
            RegisterApplicationRecoveryCallback,
            RegisterApplicationRestart,
            RegisterBadMemoryNotification,
            // RegisterWaitForInputIdle,
            RegisterWaitForSingleObject,
            RegisterWaitForSingleObjectEx,
            // RegisterWaitUntilOOBECompleted,
            ReleaseMutex,
            ReleaseMutexWhenCallbackReturns,
            ReleaseSRWLockExclusive,
            ReleaseSRWLockShared,
            ReleaseSemaphore,
            ReleaseSemaphoreWhenCallbackReturns,
            RemoveDirectoryA,
            RemoveDirectoryTransactedA,
            RemoveDirectoryTransactedW,
            RemoveDirectoryW,
            RemoveDllDirectory,
            // RemoveLocalAlternateComputerNameA,
            // RemoveLocalAlternateComputerNameW,
            RemoveSecureMemoryCacheCallback,
            RemoveVectoredContinueHandler,
            RemoveVectoredExceptionHandler,
            ReplaceFileA,
            ReplaceFileW,
            ReplacePartitionUnit,
            RequestDeviceWakeup,
            RequestWakeupLatency,
            ResetEvent,
            ResetWriteWatch,
            // ResolveDelayLoadedAPI,
            // ResolveDelayLoadsFromDll,
            ResolveLocaleName,
            RestoreLastError,
            ResumeThread,
            RtlAddFunctionTable,
            RtlCaptureContext,
            RtlCaptureStackBackTrace,
            RtlCompareMemory,
            RtlCopyMemory,
            RtlDeleteFunctionTable,
            // RtlFillMemory,
            RtlInstallFunctionTableCallback,
            RtlLookupFunctionEntry,
            // RtlMoveMemory,
            RtlPcToFileHeader,
            // RtlRaiseException,
            RtlRestoreContext,
            RtlUnwind,
            RtlUnwindEx,
            RtlVirtualUnwind,
            // RtlZeroMemory,
            ScrollConsoleScreenBufferA,
            ScrollConsoleScreenBufferW,
            SearchPathA,
            SearchPathW,
            SetCachedSigningLevel,
            SetCalendarInfoA,
            SetCalendarInfoW,
            SetCommBreak,
            SetCommConfig,
            SetCommMask,
            SetCommState,
            SetCommTimeouts,
            SetComputerNameA,
            SetComputerNameEx2W,
            SetComputerNameExA,
            SetComputerNameExW,
            SetComputerNameW,
            SetConsoleActiveScreenBuffer,
            SetConsoleCP,
            SetConsoleCtrlHandler,
            // SetConsoleCursor,
            SetConsoleCursorInfo,
            SetConsoleCursorPosition,
            SetConsoleDisplayMode,
            SetConsoleHistoryInfo,
            SetConsoleMode,
            SetConsoleOutputCP,
            SetConsoleScreenBufferInfoEx,
            SetConsoleScreenBufferSize,
            SetConsoleTextAttribute,
            SetConsoleTitleA,
            SetConsoleTitleW,
            SetConsoleWindowInfo,
            SetCriticalSectionSpinCount,
            SetCurrentConsoleFontEx,
            SetCurrentDirectoryA,
            SetCurrentDirectoryW,
            SetDefaultCommConfigA,
            SetDefaultCommConfigW,
            SetDefaultDllDirectories,
            SetDllDirectoryA,
            SetDllDirectoryW,
            SetDynamicTimeZoneInformation,
            SetEndOfFile,
            SetEnvironmentStringsA,
            SetEnvironmentStringsW,
            SetEnvironmentVariableA,
            SetEnvironmentVariableW,
            SetErrorMode,
            SetEvent,
            SetEventWhenCallbackReturns,
            SetFileApisToANSI,
            SetFileApisToOEM,
            SetFileAttributesA,
            SetFileAttributesTransactedA,
            SetFileAttributesTransactedW,
            SetFileAttributesW,
            SetFileBandwidthReservation,
            SetFileCompletionNotificationModes,
            SetFileInformationByHandle,
            SetFileIoOverlappedRange,
            SetFilePointer,
            SetFilePointerEx,
            SetFileShortNameA,
            SetFileShortNameW,
            SetFileTime,
            SetFileValidData,
            SetFirmwareEnvironmentVariableA,
            SetFirmwareEnvironmentVariableExA,
            SetFirmwareEnvironmentVariableExW,
            SetFirmwareEnvironmentVariableW,
            SetHandleCount,
            SetHandleInformation,
            SetInformationJobObject,
            SetLastError,
            // SetLocalPrimaryComputerNameA,
            // SetLocalPrimaryComputerNameW,
            SetLocalTime,
            SetLocaleInfoA,
            SetLocaleInfoW,
            SetMailslotInfo,
            SetMessageWaitingIndicator,
            SetNamedPipeAttribute,
            SetNamedPipeHandleState,
            SetPriorityClass,
            SetProcessAffinityMask,
            SetProcessAffinityUpdateMode,
            SetProcessDEPPolicy,
            SetProcessInformation,
            SetProcessMitigationPolicy,
            SetProcessPreferredUILanguages,
            SetProcessPriorityBoost,
            SetProcessShutdownParameters,
            SetProcessWorkingSetSize,
            SetProcessWorkingSetSizeEx,
            SetProtectedPolicy,
            SetSearchPathMode,
            SetStdHandle,
            SetStdHandleEx,
            SetSystemFileCacheSize,
            SetSystemPowerState,
            SetSystemTime,
            SetSystemTimeAdjustment,
            SetTapeParameters,
            SetTapePosition,
            SetThreadAffinityMask,
            SetThreadContext,
            SetThreadErrorMode,
            SetThreadExecutionState,
            SetThreadGroupAffinity,
            SetThreadIdealProcessor,
            SetThreadIdealProcessorEx,
            SetThreadInformation,
            SetThreadLocale,
            SetThreadPreferredUILanguages,
            SetThreadPriority,
            SetThreadPriorityBoost,
            SetThreadStackGuarantee,
            SetThreadUILanguage,
            SetThreadpoolStackInformation,
            SetThreadpoolThreadMaximum,
            SetThreadpoolThreadMinimum,
            SetThreadpoolTimer,
            SetThreadpoolTimerEx,
            SetThreadpoolWait,
            SetThreadpoolWaitEx,
            SetTimeZoneInformation,
            SetTimerQueueTimer,
            SetUmsThreadInformation,
            SetUnhandledExceptionFilter,
            SetUserGeoID,
            SetVolumeLabelA,
            SetVolumeLabelW,
            SetVolumeMountPointA,
            SetVolumeMountPointW,
            SetWaitableTimer,
            SetWaitableTimerEx,
            SetXStateFeaturesMask,
            SetupComm,
            SignalObjectAndWait,
            SizeofResource,
            Sleep,
            SleepConditionVariableCS,
            SleepConditionVariableSRW,
            SleepEx,
            StartThreadpoolIo,
            SubmitThreadpoolWork,
            SuspendThread,
            SwitchToFiber,
            SwitchToThread,
            SystemTimeToFileTime,
            SystemTimeToTzSpecificLocalTime,
            SystemTimeToTzSpecificLocalTimeEx,
            TerminateJobObject,
            TerminateProcess,
            TerminateThread,
            Thread32First,
            Thread32Next,
            TlsAlloc,
            TlsFree,
            TlsGetValue,
            TlsSetValue,
            Toolhelp32ReadProcessMemory,
            TransactNamedPipe,
            TransmitCommChar,
            TryAcquireSRWLockExclusive,
            TryAcquireSRWLockShared,
            TryEnterCriticalSection,
            TrySubmitThreadpoolCallback,
            TzSpecificLocalTimeToSystemTime,
            TzSpecificLocalTimeToSystemTimeEx,
            UmsThreadYield,
            UnhandledExceptionFilter,
            UnlockFile,
            UnlockFileEx,
            UnmapViewOfFile,
            UnregisterApplicationRecoveryCallback,
            UnregisterApplicationRestart,
            UnregisterBadMemoryNotification,
            UnregisterWait,
            UnregisterWaitEx,
            // UnregisterWaitUntilOOBECompleted,
            UpdateProcThreadAttribute,
            UpdateResourceA,
            UpdateResourceW,
            VerLanguageNameA,
            VerLanguageNameW,
            VerSetConditionMask,
            VerifyScripts,
            VerifyVersionInfoA,
            VerifyVersionInfoW,
            VirtualAlloc,
            VirtualAllocEx,
            VirtualAllocExNuma,
            VirtualFree,
            VirtualFreeEx,
            VirtualLock,
            VirtualProtect,
            VirtualProtectEx,
            VirtualQuery,
            VirtualQueryEx,
            VirtualUnlock,
            WTSGetActiveConsoleSessionId,
            WaitCommEvent,
            WaitForDebugEvent,
            WaitForMultipleObjects,
            WaitForMultipleObjectsEx,
            WaitForSingleObject,
            WaitForSingleObjectEx,
            WaitForThreadpoolIoCallbacks,
            WaitForThreadpoolTimerCallbacks,
            WaitForThreadpoolWaitCallbacks,
            WaitForThreadpoolWorkCallbacks,
            WaitNamedPipeA,
            WaitNamedPipeW,
            WakeAllConditionVariable,
            WakeConditionVariable,
            WerGetFlags,
            WerRegisterFile,
            WerRegisterMemoryBlock,
            WerRegisterRuntimeExceptionModule,
            WerSetFlags,
            WerUnregisterFile,
            WerUnregisterMemoryBlock,
            WerUnregisterRuntimeExceptionModule,
            // WerpInitiateRemoteRecovery,
            WideCharToMultiByte,
            WinExec,
            Wow64DisableWow64FsRedirection,
            Wow64EnableWow64FsRedirection,
            Wow64GetThreadContext,
            Wow64GetThreadSelectorEntry,
            Wow64RevertWow64FsRedirection,
            Wow64SetThreadContext,
            Wow64SuspendThread,
            WriteConsoleA,
            WriteConsoleInputA,
            WriteConsoleInputW,
            WriteConsoleOutputA,
            WriteConsoleOutputAttribute,
            WriteConsoleOutputCharacterA,
            WriteConsoleOutputCharacterW,
            WriteConsoleOutputW,
            WriteConsoleW,
            WriteFile,
            WriteFileEx,
            WriteFileGather,
            WritePrivateProfileSectionA,
            WritePrivateProfileSectionW,
            WritePrivateProfileStringA,
            WritePrivateProfileStringW,
            WritePrivateProfileStructA,
            WritePrivateProfileStructW,
            WriteProcessMemory,
            WriteProfileSectionA,
            WriteProfileSectionW,
            WriteProfileStringA,
            WriteProfileStringW,
            WriteTapemark,
            ZombifyActCtx,
            _hread,
            _hwrite,
            _lclose,
            _lcreat,
            _llseek,
            _lopen,
            _lread,
            _lwrite,
            lstrcat,
            lstrcatA,
            lstrcatW,
            lstrcmp,
            lstrcmpA,
            lstrcmpW,
            lstrcmpi,
            lstrcmpiA,
            lstrcmpiW,
            lstrcpy,
            lstrcpyA,
            lstrcpyW,
            lstrcpyn,
            lstrcpynA,
            lstrcpynW,
            lstrlen,
            lstrlenA,
            lstrlenW,
            uaw_lstrcmpW,
            uaw_lstrcmpiW,
            uaw_lstrlenW,
            uaw_wcschr,
            uaw_wcscpy,
            uaw_wcsicmp,
            uaw_wcslen,
            uaw_wcsrchr,
        };
    }
    pub use self::__ia_kernel32_inner::*;
    }
}

#[cfg(test)]
isolation_aware_kernel32!();

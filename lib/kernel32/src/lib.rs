// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to kernel32.
#![cfg(windows)]
extern crate winapi;
use winapi::*;
extern "system" {
    pub fn CallbackMayRunLong(pci: PTP_CALLBACK_INSTANCE) -> BOOL;
    pub fn CalloutOnFiberStack(
        lpFiber: PVOID, lpStartAddress: PFIBER_CALLOUT_ROUTINE, lpParameter: PVOID,
    ) -> PVOID;
    pub fn CancelThreadpoolIo(pio: PTP_IO);
    pub fn ChangeTimerQueueTimer(
        TimerQueue: HANDLE, Timer: HANDLE, DueTime: ULONG, Period: ULONG,
    ) -> BOOL;
    pub fn CloseThreadpool(ptpp: PTP_POOL);
    pub fn CloseThreadpoolCleanupGroup(ptpcg: PTP_CLEANUP_GROUP);
    pub fn CloseThreadpoolCleanupGroupMembers(
        ptpcg: PTP_CLEANUP_GROUP, fCancelPendingCallbacks: BOOL, pvCleanupContext: PVOID,
    );
    pub fn CloseThreadpoolIo(pio: PTP_IO);
    pub fn CloseThreadpoolTimer(pti: PTP_TIMER);
    pub fn CloseThreadpoolWait(pwa: PTP_WAIT);
    pub fn CloseThreadpoolWork(pwk: PTP_WORK);
    pub fn CreateThreadpool(reserved: PVOID) -> PTP_POOL;
    pub fn CreateThreadpoolCleanupGroup() -> PTP_CLEANUP_GROUP;
    pub fn CreateThreadpoolIo(
        fl: HANDLE, pfnio: PTP_WIN32_IO_CALLBACK, pv: PVOID, pcbe: PTP_CALLBACK_ENVIRON,
    ) -> PTP_IO;
    pub fn CreateThreadpoolTimer(
        pfnti: PTP_TIMER_CALLBACK, pv: PVOID, pcbe: PTP_CALLBACK_ENVIRON,
    ) -> PTP_TIMER;
    pub fn CreateThreadpoolWait(
        pfnwa: PTP_WAIT_CALLBACK, pv: PVOID, pcbe: PTP_CALLBACK_ENVIRON,
    ) -> PTP_WAIT;
    pub fn CreateThreadpoolWork(
        pfnwk: PTP_WORK_CALLBACK, pv: PVOID, pcbe: PTP_CALLBACK_ENVIRON,
    ) -> PTP_WORK;
    pub fn CreateTimerQueue() -> HANDLE;
    pub fn CreateTimerQueueTimer(
        phNewTimer: PHANDLE, TimerQueue: HANDLE, Callback: WAITORTIMERCALLBACK, Parameter: PVOID,
        DueTime: DWORD, Period: DWORD, Flags: ULONG,
    ) -> BOOL;
    pub fn DelayLoadFailureHook(pszDllName: LPCSTR, pszProcName: LPCSTR) -> FARPROC;
    pub fn DeleteTimerQueueEx(TimerQueue: HANDLE, CompletionEvent: HANDLE) -> BOOL;
    pub fn DeleteTimerQueueTimer(
        TimerQueue: HANDLE, Timer: HANDLE, CompletionEvent: HANDLE,
    ) -> BOOL;
    pub fn FindClose(hFindFile: HANDLE) -> BOOL;
    pub fn FindNLSString(
        Locale: LCID, dwFindNLSStringFlags: DWORD, lpStringSource: LPCWSTR, cchSource: c_int,
        lpStringValue: LPCWSTR, cchValue: c_int, pcchFound: LPINT,
    ) -> c_int;
    pub fn FindNLSStringEx(
        lpLocaleName: LPCWSTR, dwFindNLSStringFlags: DWORD, lpStringSource: LPCWSTR,
        cchSource: c_int, lpStringValue: LPCWSTR, cchValue: c_int, pcchFound: LPINT,
        lpVersionInformation: LPNLSVERSIONINFO, lpReserved: LPVOID, sortHandle: LPARAM,
    ) -> c_int;
    pub fn FlsAlloc(lpCallback: PFLS_CALLBACK_FUNCTION) -> DWORD;
    pub fn FlsFree(dwFlsIndex: DWORD) -> BOOL;
    pub fn FlsGetValue(dwFlsIndex: DWORD) -> PVOID;
    pub fn FlsSetValue(dwFlsIndex: DWORD, lpFlsData: PVOID) -> BOOL;
    pub fn FreeLibraryWhenCallbackReturns(pci: PTP_CALLBACK_INSTANCE, module: HMODULE);
    pub fn GetProcessGroupAffinity(
        hProcess: HANDLE, GroupCount: PUSHORT, GroupArray: PUSHORT,
    ) -> BOOL;
    pub fn GetProcessorSystemCycleTime(
        Group: USHORT, Buffer: PSYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION, ReturnedLength: PDWORD,
    ) -> BOOL;
    pub fn GetProductInfo(
        dwOSMajorVersion: DWORD, dwOSMinorVersion: DWORD, dwSpMajorVersion: DWORD,
        dwSpMinorVersion: DWORD, pdwReturnedProductType: PDWORD,
    ) -> BOOL;
    pub fn GetSystemDirectoryA(lpBuffer: LPSTR, uSize: UINT) -> UINT;
    pub fn GetSystemDirectoryW(lpBuffer: LPWSTR, uSize: UINT) -> UINT;
    pub fn GetSystemFirmwareTable(
        FirmwareTableProviderSignature: DWORD, FirmwareTableID: DWORD, pFirmwareTableBuffer: PVOID,
        BufferSize: DWORD,
    ) -> UINT;
    pub fn GetSystemInfo(lpSystemInfo: LPSYSTEM_INFO);
    pub fn GetThreadGroupAffinity(hThread: HANDLE, GroupAffinity: PGROUP_AFFINITY) -> BOOL;
    pub fn GetThreadSelectorEntry(
        hThread: HANDLE, dwSelector: DWORD, lpSelectorEntry: LPLDT_ENTRY,
    ) -> BOOL;
    pub fn GetTimeFormatA(
        Locale: LCID, dwFlags: DWORD, lpTime: *const SYSTEMTIME, lpFormat: LPCSTR,
        lpTimeStr: LPSTR, cchTime: c_int,
    ) -> c_int;
    pub fn GetTimeFormatEx(
        lpLocaleName: LPCWSTR, dwFlags: DWORD, lpTime: *const SYSTEMTIME, lpFormat: LPCWSTR,
        lpTimeStr: LPWSTR, cchTime: c_int,
    ) -> c_int;
    pub fn GetTimeFormatW(
        Locale: LCID, dwFlags: DWORD, lpTime: *const SYSTEMTIME, lpFormat: LPCWSTR,
        lpTimeStr: LPWSTR, cchTime: c_int,
    ) -> c_int;
    pub fn GetVolumePathNamesForVolumeNameW(
        lpszVolumeName: LPCWSTR, lpszVolumePathNames: LPWCH, cchBufferLength: DWORD,
        lpcchReturnLength: PDWORD,
    ) -> BOOL;
    #[cfg(target_arch = "x86")]
    pub fn InterlockedCompareExchange64(
        Destination: *mut LONG64, ExChange: LONG64, Comperand: LONG64,
    ) -> LONG64;
    pub fn IsProcessInJob(ProcessHandle: HANDLE, JobHandle: HANDLE, Result: PBOOL) -> BOOL;
    pub fn IsThreadAFiber() -> BOOL;
    pub fn IsThreadpoolTimerSet(pti: PTP_TIMER) -> BOOL;
    pub fn K32EmptyWorkingSet(hProcess: HANDLE) -> BOOL;
    pub fn K32EnumDeviceDrivers(lpImageBase: *mut LPVOID, cb: DWORD, lpcbNeeded: LPDWORD) -> BOOL;
    pub fn K32EnumPageFilesA(
        pCallBackRoutine: PENUM_PAGE_FILE_CALLBACKA, pContext: LPVOID,
    ) -> BOOL;
    pub fn K32EnumPageFilesW(
        pCallBackRoutine: PENUM_PAGE_FILE_CALLBACKW, pContext: LPVOID,
    ) -> BOOL;
    pub fn K32EnumProcessModules(
        hProcess: HANDLE, lphModule: *mut HMODULE, cb: DWORD, lpcbNeeded: LPDWORD,
    ) -> BOOL;
    pub fn K32EnumProcessModulesEx(
        hProcess: HANDLE, lphModule: *mut HMODULE, cb: DWORD, lpcbNeeded: LPDWORD,
        dwFilterFlag: DWORD,
    ) -> BOOL;
    pub fn K32EnumProcesses(
        lpidProcess: *mut DWORD, cb: DWORD, lpcbNeeded: LPDWORD,
    ) -> BOOL;
    pub fn K32GetDeviceDriverBaseNameA(ImageBase: LPVOID, lpFilename: LPSTR, nSize: DWORD) -> DWORD;
    pub fn K32GetDeviceDriverBaseNameW(
        ImageBase: LPVOID, lpFilename: LPWSTR, nSize: DWORD,
    ) -> DWORD;
    pub fn K32GetDeviceDriverFileNameA(ImageBase: LPVOID, lpFilename: LPSTR, nSize: DWORD) -> DWORD;
    pub fn K32GetDeviceDriverFileNameW(
        ImageBase: LPVOID, lpFilename: LPWSTR, nSize: DWORD,
    ) -> DWORD;
    pub fn K32GetMappedFileNameA(
        hProcess: HANDLE, lpv: LPVOID, lpFilename: LPSTR, nSize: DWORD,
    ) -> DWORD;
    pub fn K32GetMappedFileNameW(
        hProcess: HANDLE, lpv: LPVOID, lpFilename: LPWSTR, nSize: DWORD,
    ) -> DWORD;
    pub fn K32GetModuleBaseNameA(
        hProcess: HANDLE, hModule: HMODULE, lpBaseName: LPSTR, nSize: DWORD,
    ) -> DWORD;
    pub fn K32GetModuleBaseNameW(
        hProcess: HANDLE, hModule: HMODULE, lpBaseName: LPWSTR, nSize: DWORD,
    ) -> DWORD;
    pub fn K32GetModuleFileNameExA(
        hProcess: HANDLE, hModule: HMODULE, lpFilename: LPSTR, nSize: DWORD,
    ) -> DWORD;
    pub fn K32GetModuleFileNameExW(
        hProcess: HANDLE, hModule: HMODULE, lpFilename: LPWSTR, nSize: DWORD,
    ) -> DWORD;
    pub fn K32GetModuleInformation(
        hProcess: HANDLE, hModule: HMODULE, lpmodinfo: LPMODULEINFO, cb: DWORD,
    ) -> BOOL;
    pub fn K32GetPerformanceInfo(
        pPerformanceInformation: PPERFORMANCE_INFORMATION, cb: DWORD,
    ) -> BOOL;
    pub fn K32GetProcessImageFileNameA(
        hProcess: HANDLE, lpImageFileName: LPSTR, nSize: DWORD,
    ) -> DWORD;
    pub fn K32GetProcessImageFileNameW(
        hProcess: HANDLE, lpImageFileName: LPWSTR, nSize: DWORD,
    ) -> DWORD;
    pub fn K32GetProcessMemoryInfo(
        Process: HANDLE, ppsmemCounters: PPROCESS_MEMORY_COUNTERS, cb: DWORD,
    ) -> BOOL;
    pub fn K32GetWsChanges(
        hProcess: HANDLE, lpWatchInfo: PPSAPI_WS_WATCH_INFORMATION, cb: DWORD,
    ) -> BOOL;
    pub fn K32GetWsChangesEx(
        hProcess: HANDLE, lpWatchInfoEx: PPSAPI_WS_WATCH_INFORMATION_EX, cb: PDWORD,
    ) -> BOOL;
    pub fn K32InitializeProcessForWsWatch(hProcess: HANDLE) -> BOOL;
    pub fn K32QueryWorkingSet(hProcess: HANDLE, pv: PVOID, cb: DWORD) -> BOOL;
    pub fn K32QueryWorkingSetEx(hProcess: HANDLE, pv: PVOID, cb: DWORD) -> BOOL;
    pub fn LeaveCriticalSectionWhenCallbackReturns(
        pci: PTP_CALLBACK_INSTANCE, pcs: PCRITICAL_SECTION,
    );
    pub fn QueryIdleProcessorCycleTime(
        BufferLength: PULONG, ProcessorIdleCycleTime: PULONG64,
    ) -> BOOL;
    pub fn QueryIdleProcessorCycleTimeEx(
        Group: USHORT, BufferLength: PULONG, ProcessorIdleCycleTime: PULONG64,
    ) -> BOOL;
    pub fn QueryPerformanceCounter(lpPerformanceCount: *mut LARGE_INTEGER) -> BOOL;
    pub fn QueryPerformanceFrequency(lpFrequency: *mut LARGE_INTEGER) -> BOOL;
    pub fn QueryProcessCycleTime(ProcessHandle: HANDLE, CycleTime: PULONG64) -> BOOL;
    pub fn QueryThreadCycleTime(ThreadHandle: HANDLE, CycleTime: PULONG64) -> BOOL;
    pub fn QueryUnbiasedInterruptTime(UnbiasedTime: PULONGLONG) -> BOOL;
    pub fn QueueUserWorkItem(
        Function: LPTHREAD_START_ROUTINE, Context: PVOID, Flags: ULONG,
    ) -> BOOL;
    pub fn RegisterApplicationRestart(pwzCommandline: PCWSTR, dwFlags: DWORD) -> HRESULT;
    pub fn RegisterWaitForSingleObjectEx(
        hObject: HANDLE, Callback: WAITORTIMERCALLBACK, Context: PVOID, dwMilliseconds: ULONG,
        dwFlags: ULONG,
    ) -> HANDLE;
    pub fn ReleaseMutexWhenCallbackReturns(pci: PTP_CALLBACK_INSTANCE, mutex: HANDLE);
    pub fn ReleaseSemaphoreWhenCallbackReturns(
        pci: PTP_CALLBACK_INSTANCE, sem: HANDLE, crel: DWORD,
    );
    pub fn RtlCopyMemory(Destination: PVOID, Source: *const VOID, Length: SIZE_T);
    pub fn SetNamedPipeAttribute(
        Pipe: HANDLE, AttributeType: PIPE_ATTRIBUTE_TYPE, AttributeName: PSTR,
        AttributeValue: PVOID, AttributeValueLength: SIZE_T,
    ) -> BOOL;
    pub fn SetSystemTime(lpSystemTime: *const SYSTEMTIME) -> BOOL;
    pub fn SetSystemTimeAdjustment(dwTimeAdjustment: DWORD, bTimeAdjustmentDisabled: BOOL) -> BOOL;
    pub fn SetThreadGroupAffinity(
        hThread: HANDLE, GroupAffinity: *const GROUP_AFFINITY,
        PreviousGroupAffinity: PGROUP_AFFINITY,
    ) -> BOOL;
    pub fn SetThreadpoolStackInformation(
        ptpp: PTP_POOL, ptpsi: PTP_POOL_STACK_INFORMATION,
    ) -> BOOL;
    pub fn SetThreadpoolThreadMaximum(ptpp: PTP_POOL, cthrdMost: DWORD);
    pub fn SetThreadpoolThreadMinimum(ptpp: PTP_POOL, cthrdMic: DWORD) -> BOOL;
    pub fn SetThreadpoolTimer(
        pti: PTP_TIMER, pftDueTime: PFILETIME, msPeriod: DWORD, msWindowLength: DWORD,
    );
    pub fn SetThreadpoolTimerEx(
        pti: PTP_TIMER, pftDueTime: PFILETIME, msPeriod: DWORD, msWindowLength: DWORD,
    ) -> BOOL;
    pub fn SetThreadpoolWait(pwa: PTP_WAIT, h: HANDLE, pftTimeout: PFILETIME);
    pub fn SetThreadpoolWaitEx(
        pwa: PTP_WAIT, h: HANDLE, pftTimeout: PFILETIME, Reserved: PVOID,
    ) -> BOOL;
    pub fn StartThreadpoolIo(pio: PTP_IO);
    pub fn SubmitThreadpoolWork(pwk: PTP_WORK);
    pub fn TrySubmitThreadpoolCallback(
        pfns: PTP_SIMPLE_CALLBACK, pv: PVOID, pcbe: PTP_CALLBACK_ENVIRON,
    ) -> BOOL;
    pub fn UnregisterWaitEx(WaitHandle: HANDLE, CompletionEvent: HANDLE) -> BOOL;
    pub fn WaitForMultipleObjectsEx(
        nCount: DWORD, lpHandles: *const HANDLE, bWaitAll: BOOL, dwMilliseconds: DWORD,
        bAlertable: BOOL,
    ) -> DWORD;
    pub fn WaitForThreadpoolIoCallbacks(pio: PTP_IO, fCancelPendingCallbacks: BOOL);
    pub fn WaitForThreadpoolTimerCallbacks(pti: PTP_TIMER, fCancelPendingCallbacks: BOOL);
    pub fn WaitForThreadpoolWaitCallbacks(pwa: PTP_WAIT, fCancelPendingCallbacks: BOOL);
    pub fn WaitForThreadpoolWorkCallbacks(pwk: PTP_WORK, fCancelPendingCallbacks: BOOL);
    pub fn lstrcat(lpString1: LPSTR, lpString2: LPCSTR) -> LPSTR;
    pub fn lstrcmp(lpString1: LPCSTR, lpString2: LPCSTR) -> c_int;
    pub fn lstrcmpi(lpString1: LPCSTR, lpString2: LPCSTR) -> c_int;
    pub fn lstrcpy(lpString1: LPSTR, lpString2: LPCSTR) -> LPSTR;
    pub fn lstrcpyn(lpString1: LPSTR, lpString2: LPCSTR, iMaxLength: c_int) -> LPSTR;
    pub fn lstrlen(lpString: LPCSTR) -> c_int;
}

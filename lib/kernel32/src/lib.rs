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
    pub fn CheckRemoteDebuggerPresent(hProcess: HANDLE, pbDebuggerPresent: PBOOL) -> BOOL;
    pub fn CloseThreadpool(ptpp: PTP_POOL);
    pub fn CloseThreadpoolCleanupGroup(ptpcg: PTP_CLEANUP_GROUP);
    pub fn CloseThreadpoolCleanupGroupMembers(
        ptpcg: PTP_CLEANUP_GROUP, fCancelPendingCallbacks: BOOL, pvCleanupContext: PVOID,
    );
    pub fn CloseThreadpoolIo(pio: PTP_IO);
    pub fn CloseThreadpoolTimer(pti: PTP_TIMER);
    pub fn CloseThreadpoolWait(pwa: PTP_WAIT);
    pub fn CloseThreadpoolWork(pwk: PTP_WORK);
    pub fn CompareStringOrdinal(
        lpString1: LPCWCH, cchCount1: c_int, lpString2: LPCWCH, cchCount2: c_int, bIgnoreCase: BOOL,
    ) -> c_int;
    pub fn CompareStringW(
        Locale: LCID, dwCmpFlags: DWORD, lpString1: PCNZWCH, cchCount1: c_int, lpString2: PCNZWCH,
        cchCount2: c_int,
    ) -> c_int;
    pub fn ContinueDebugEvent(
        dwProcessId: DWORD, dwThreadId: DWORD, dwContinueStatus: DWORD,
    ) -> BOOL;
    pub fn ConvertFiberToThread() -> BOOL;
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
    pub fn CreateToolhelp32Snapshot(dwFlags: DWORD, th32ProcessID: DWORD) -> HANDLE;
    pub fn DebugActiveProcess(dwProcessId: DWORD) -> BOOL;
    pub fn DebugActiveProcessStop(dwProcessId: DWORD) -> BOOL;
    pub fn DebugBreak();
    pub fn DelayLoadFailureHook(pszDllName: LPCSTR, pszProcName: LPCSTR) -> FARPROC;
    pub fn DeleteTimerQueueEx(TimerQueue: HANDLE, CompletionEvent: HANDLE) -> BOOL;
    pub fn DeleteTimerQueueTimer(
        TimerQueue: HANDLE, Timer: HANDLE, CompletionEvent: HANDLE,
    ) -> BOOL;
    pub fn ExpandEnvironmentStringsA(lpSrc: LPCSTR, lpDst: LPSTR, nSize: DWORD) -> DWORD;
    pub fn ExpandEnvironmentStringsW(lpSrc: LPCWSTR, lpDst: LPWSTR, nSize: DWORD) -> DWORD;
    pub fn FileTimeToSystemTime(
        lpFileTime: *const FILETIME, lpSystemTime: LPSYSTEMTIME,
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
    pub fn FoldStringW(
        dwMapFlags: DWORD, lpSrcStr: LPCWCH, cchSrc: c_int, lpDestStr: LPWSTR, cchDest: c_int,
    ) -> c_int;
    pub fn FreeEnvironmentStringsA(penv: LPCH) -> BOOL;
    pub fn FreeEnvironmentStringsW(penv: LPWCH) -> BOOL;
    pub fn GetAppContainerNamedObjectPath(
        Token: HANDLE, AppContainerSid: PSID, ObjectPathLength: ULONG, ObjectPath: LPWSTR,
        ReturnLength: PULONG,
    ) -> BOOL;
    pub fn GetCPInfo(CodePage: UINT, lpCPInfo: LPCPINFO) -> BOOL;
    pub fn GetCPInfoExA(CodePage: UINT, dwFlags: DWORD, lpCPInfoEx: LPCPINFOEXA) -> BOOL;
    pub fn GetCPInfoExW(CodePage: UINT, dwFlags: DWORD, lpCPInfoEx: LPCPINFOEXW) -> BOOL;
    pub fn GetCalendarInfoA(
        Locale: LCID, Calendar: CALID, CalType: CALTYPE, lpCalData: LPSTR, cchData: c_int,
        lpValue: LPDWORD,
    ) -> c_int;
    pub fn GetCalendarInfoEx(
        lpLocaleName: LPCWSTR, Calendar: CALID, lpReserved: LPCWSTR, CalType: CALTYPE,
        lpCalData: LPWSTR, cchData: c_int, lpValue: LPDWORD,
    ) -> c_int;
    pub fn GetCalendarInfoW(
        Locale: LCID, Calendar: CALID, CalType: CALTYPE, lpCalData: LPWSTR, cchData: c_int,
        lpValue: LPDWORD,
    ) -> c_int;
    pub fn GetCommandLineA() -> LPSTR;
    pub fn GetCommandLineW() -> LPWSTR;
    pub fn GetProcessGroupAffinity(
        hProcess: HANDLE, GroupCount: PUSHORT, GroupArray: PUSHORT,
    ) -> BOOL;
    pub fn GetProcessHeap() -> HANDLE;
    pub fn GetProcessHeaps(NumberOfHeaps: DWORD, ProcessHeaps: PHANDLE) -> DWORD;
    pub fn GetProcessorSystemCycleTime(
        Group: USHORT, Buffer: PSYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION, ReturnedLength: PDWORD,
    ) -> BOOL;
    pub fn GetProductInfo(
        dwOSMajorVersion: DWORD, dwOSMinorVersion: DWORD, dwSpMajorVersion: DWORD,
        dwSpMinorVersion: DWORD, pdwReturnedProductType: PDWORD,
    ) -> BOOL;
    pub fn GetStdHandle(nStdHandle: DWORD) -> HANDLE;
    pub fn GetStringTypeExW(
        Locale: LCID, dwInfoType: DWORD, lpSrcStr: LPCWCH, cchSrc: c_int, lpCharType: LPWORD,
    ) -> BOOL;
    pub fn GetStringTypeW(
        dwInfoType: DWORD, lpSrcStr: LPCWCH, cchSrc: c_int, lpCharType: LPWORD,
    ) -> BOOL;
    pub fn GetSystemDefaultLCID() -> LCID;
    pub fn GetSystemDefaultLangID() -> LANGID;
    pub fn GetSystemDefaultLocaleName(lpLocaleName: LPWSTR, cchLocaleName: c_int) -> c_int;
    pub fn GetSystemDefaultUILanguage() -> LANGID;
    pub fn GetSystemDirectoryA(lpBuffer: LPSTR, uSize: UINT) -> UINT;
    pub fn GetSystemDirectoryW(lpBuffer: LPWSTR, uSize: UINT) -> UINT;
    pub fn GetSystemFirmwareTable(
        FirmwareTableProviderSignature: DWORD, FirmwareTableID: DWORD, pFirmwareTableBuffer: PVOID,
        BufferSize: DWORD,
    ) -> UINT;
    pub fn GetSystemInfo(lpSystemInfo: LPSYSTEM_INFO);
    pub fn GetSystemWow64DirectoryA(lpBuffer: LPSTR, uSize: UINT) -> UINT;
    pub fn GetSystemWow64DirectoryW(lpBuffer: LPWSTR, uSize: UINT) -> UINT;
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
    pub fn GetTimeZoneInformation(lpTimeZoneInformation: LPTIME_ZONE_INFORMATION) -> DWORD;
    pub fn GetTimeZoneInformationForYear(
        wYear: USHORT, pdtzi: PDYNAMIC_TIME_ZONE_INFORMATION, ptzi: LPTIME_ZONE_INFORMATION,
    ) -> BOOL;
    pub fn GetVolumePathNamesForVolumeNameW(
        lpszVolumeName: LPCWSTR, lpszVolumePathNames: LPWCH, cchBufferLength: DWORD,
        lpcchReturnLength: PDWORD,
    ) -> BOOL;

    pub fn GetWriteWatch(
        dwFlags: DWORD, lpBaseAddress: PVOID, dwRegionSize: SIZE_T, lpAddresses: *mut PVOID,
        lpdwCount: *mut ULONG_PTR, lpdwGranularity: LPDWORD,
    ) -> UINT;

    pub fn Heap32First(lphe: LPHEAPENTRY32, th32ProcessID: DWORD, th32HeapID: ULONG_PTR) -> BOOL;
    pub fn Heap32ListFirst(hSnapshot: HANDLE, lphl: LPHEAPLIST32) -> BOOL;
    pub fn Heap32ListNext(hSnapshot: HANDLE, lphl: LPHEAPLIST32) -> BOOL;
    pub fn Heap32Next(lphe: LPHEAPENTRY32) -> BOOL;
    pub fn HeapAlloc(hHeap: HANDLE, dwFlags: DWORD, dwBytes: SIZE_T) -> LPVOID;
    pub fn HeapCompact(hHeap: HANDLE, dwFlags: DWORD) -> SIZE_T;
    pub fn HeapCreate(flOptions: DWORD, dwInitialSize: SIZE_T, dwMaximumSize: SIZE_T) -> HANDLE;
    pub fn HeapDestroy(hHeap: HANDLE) -> BOOL;
    pub fn HeapFree(hHeap: HANDLE, dwFlags: DWORD, lpMem: LPVOID) -> BOOL;
    pub fn HeapLock(hHeap: HANDLE) -> BOOL;
    pub fn HeapQueryInformation(
        HeapHandle: HANDLE, HeapInformationClass: HEAP_INFORMATION_CLASS, HeapInformation: PVOID,
        HeapInformationLength: SIZE_T, ReturnLength: PSIZE_T,
    ) -> BOOL;
    pub fn HeapReAlloc(hHeap: HANDLE, dwFlags: DWORD, lpMem: LPVOID, dwBytes: SIZE_T) -> LPVOID;
    pub fn HeapSetInformation(
        HeapHandle: HANDLE, HeapInformationClass: HEAP_INFORMATION_CLASS, HeapInformation: PVOID,
        HeapInformationLength: SIZE_T,
    ) -> BOOL;
    pub fn HeapSize(hHeap: HANDLE, dwFlags: DWORD, lpMem: LPCVOID) -> SIZE_T;
    pub fn HeapSummary(hHeap: HANDLE, dwFlags: DWORD, lpSummary: LPHEAP_SUMMARY) -> BOOL;
    pub fn HeapUnlock(hHeap: HANDLE) -> BOOL;
    pub fn HeapValidate(hHeap: HANDLE, dwFlags: DWORD, lpMem: LPCVOID) -> BOOL;
    pub fn HeapWalk(hHeap: HANDLE, lpEntry: LPPROCESS_HEAP_ENTRY) -> BOOL;
    pub fn InitializeSListHead(ListHead: PSLIST_HEADER);

    #[cfg(target_arch = "x86")]
    pub fn InterlockedCompareExchange64(
        Destination: *mut LONG64, ExChange: LONG64, Comperand: LONG64,
    ) -> LONG64;
    pub fn InterlockedFlushSList(ListHead: PSLIST_HEADER) -> PSLIST_ENTRY;
    pub fn InterlockedPopEntrySList(ListHead: PSLIST_HEADER) -> PSLIST_ENTRY;
    pub fn InterlockedPushEntrySList(
        ListHead: PSLIST_HEADER, ListEntry: PSLIST_ENTRY,
    ) -> PSLIST_ENTRY;
    pub fn InterlockedPushListSListEx(
        ListHead: PSLIST_HEADER, List: PSLIST_ENTRY, ListEnd: PSLIST_ENTRY, Count: ULONG,
    ) -> PSLIST_ENTRY;
    pub fn IsDebuggerPresent() -> BOOL;
    pub fn IsProcessInJob(ProcessHandle: HANDLE, JobHandle: HANDLE, Result: PBOOL) -> BOOL;
    pub fn IsThreadAFiber() -> BOOL;
    pub fn IsWow64Process(hProcess: HANDLE, Wow64Process: PBOOL) -> BOOL;
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
    pub fn LocalFlags(hMem: HLOCAL) -> UINT;
    pub fn Module32First(hSnapshot: HANDLE, lpme: LPMODULEENTRY32) -> BOOL;
    pub fn Module32FirstW(hSnapshot: HANDLE, lpme: LPMODULEENTRY32W) -> BOOL;
    pub fn Module32Next(hSnapshot: HANDLE, lpme: LPMODULEENTRY32) -> BOOL;
    pub fn Module32NextW(hSnapshot: HANDLE, lpme: LPMODULEENTRY32W) -> BOOL;
    pub fn MultiByteToWideChar(
        CodePage: UINT, dwFlags: DWORD, lpMultiByteStr: LPCSTR, cbMultiByte: c_int,
        lpWideCharStr: LPWSTR, cchWideChar: c_int,
    ) -> c_int;
    pub fn NeedCurrentDirectoryForExePathA(ExeName: LPCSTR) -> BOOL;
    pub fn NeedCurrentDirectoryForExePathW(ExeName: LPCWSTR) -> BOOL;
    pub fn NormalizeString(
        NormForm: NORM_FORM, lpSrcString: LPCWSTR, cwSrcLength: c_int, lpDstString: LPWSTR,
        cwDstLength: c_int,
    ) -> c_int;
    pub fn NotifyUILanguageChange(
        dwFlags: DWORD, pcwstrNewLanguage: PCWSTR, pcwstrPreviousLanguage: PCWSTR,
        dwReserved: DWORD, pdwStatusRtrn: PDWORD,
    ) -> BOOL;
    pub fn OutputDebugStringA(lpOutputString: LPCSTR);
    pub fn OutputDebugStringW(lpOutputString: LPCWSTR);
    pub fn Process32First(hSnapshot: HANDLE, lppe: LPPROCESSENTRY32) -> BOOL;
    pub fn Process32FirstW(hSnapshot: HANDLE, lppe: LPPROCESSENTRY32W) -> BOOL;
    pub fn Process32Next(hSnapshot: HANDLE, lppe: LPPROCESSENTRY32) -> BOOL;
    pub fn Process32NextW(hSnapshot: HANDLE, lppe: LPPROCESSENTRY32W) -> BOOL;
    pub fn QueryDepthSList(ListHead: PSLIST_HEADER) -> USHORT;
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
    pub fn ResetWriteWatch(lpBaseAddress: LPVOID, dwRegionSize: SIZE_T) -> UINT;
    pub fn ResolveLocaleName(
        lpNameToResolve: LPCWSTR, lpLocaleName: LPWSTR, cchLocaleName: c_int,
    ) -> c_int;
    #[cfg(target_arch = "arm")]
    pub fn RtlAddFunctionTable(
        FunctionTable: PRUNTIME_FUNCTION, EntryCount: DWORD, BaseAddress: DWORD,
    ) -> BOOLEAN;
    pub fn RtlCopyMemory(Destination: PVOID, Source: *const VOID, Length: SIZE_T);

    #[cfg(target_arch = "arm")]
    pub fn RtlInstallFunctionTableCallback(
        TableIdentifier: DWORD, BaseAddress: DWORD, Length: DWORD,
        Callback: PGET_RUNTIME_FUNCTION_CALLBACK, Context: PVOID, OutOfProcessCallbackDll: PCWSTR,
    ) -> BOOLEAN;

    #[cfg(target_arch = "arm")]
    pub fn RtlLookupFunctionEntry(
        ControlPc: ULONG_PTR, ImageBase: PDWORD, HistoryTable: PUNWIND_HISTORY_TABLE,
    ) -> PRUNTIME_FUNCTION;


    #[cfg(any(target_arch = "arm", target_arch = "x86_64"))]

    #[cfg(target_arch = "arm")]
    pub fn RtlVirtualUnwind(
        HandlerType: DWORD, ImageBase: DWORD, ControlPc: DWORD, FunctionEntry: PRUNTIME_FUNCTION,
        ContextRecord: PCONTEXT, HandlerData: *mut PVOID, EstablisherFrame: PDWORD,
        ContextPointers: PKNONVOLATILE_CONTEXT_POINTERS,
    ) -> PEXCEPTION_ROUTINE;
    #[cfg(target_arch = "x86_64")]

    pub fn SearchPathA(
        lpPath: LPCSTR, lpFileName: LPCSTR, lpExtension: LPCSTR, nBufferLength: DWORD,
        lpBuffer: LPSTR, lpFilePart: *mut LPSTR,
    ) -> DWORD;
    pub fn SearchPathW(
        lpPath: LPCWSTR, lpFileName: LPCWSTR, lpExtension: LPCWSTR, nBufferLength: DWORD,
        lpBuffer: LPWSTR, lpFilePart: *mut LPWSTR,
    ) -> DWORD;
    pub fn SetCurrentDirectoryA(lpPathName: LPCSTR) -> BOOL;
    pub fn SetCurrentDirectoryW(lpPathName: LPCWSTR) -> BOOL;
    pub fn SetDynamicTimeZoneInformation(
        lpTimeZoneInformation: *const DYNAMIC_TIME_ZONE_INFORMATION,
    ) -> BOOL;
    pub fn SetEnvironmentStringsW(NewEnvironment: LPWCH) -> BOOL;
    pub fn SetEnvironmentVariableA(lpName: LPCSTR, lpValue: LPCSTR) -> BOOL;
    pub fn SetEnvironmentVariableW(lpName: LPCWSTR, lpValue: LPCWSTR) -> BOOL;
    pub fn SetNamedPipeAttribute(
        Pipe: HANDLE, AttributeType: PIPE_ATTRIBUTE_TYPE, AttributeName: PSTR,
        AttributeValue: PVOID, AttributeValueLength: SIZE_T,
    ) -> BOOL;
    pub fn SetProcessPreferredUILanguages(
        dwFlags: DWORD, pwszLanguagesBuffer: PCZZWSTR, pulNumLanguages: PULONG,
    ) -> BOOL;
    pub fn SetStdHandle(nStdHandle: DWORD, hHandle: HANDLE) -> BOOL;
    pub fn SetStdHandleEx(nStdHandle: DWORD, hHandle: HANDLE, phPrevValue: PHANDLE) -> BOOL;
    pub fn SetSystemFileCacheSize(
        MinimumFileCacheSize: SIZE_T, MaximumFileCacheSize: SIZE_T, Flags: DWORD,
    ) -> BOOL;
    pub fn SetSystemTime(lpSystemTime: *const SYSTEMTIME) -> BOOL;
    pub fn SetSystemTimeAdjustment(dwTimeAdjustment: DWORD, bTimeAdjustmentDisabled: BOOL) -> BOOL;
    pub fn SetThreadGroupAffinity(
        hThread: HANDLE, GroupAffinity: *const GROUP_AFFINITY,
        PreviousGroupAffinity: PGROUP_AFFINITY,
    ) -> BOOL;
    pub fn SetTimeZoneInformation(lpTimeZoneInformation: *const TIME_ZONE_INFORMATION) -> BOOL;
    pub fn SystemTimeToFileTime(lpSystemTime: *const SYSTEMTIME, lpFileTime: LPFILETIME) -> BOOL;
    pub fn SystemTimeToTzSpecificLocalTime(
        lpTimeZoneInformation: *const TIME_ZONE_INFORMATION, lpUniversalTime: *const SYSTEMTIME,
        lpLocalTime: LPSYSTEMTIME,
    ) -> BOOL;
    pub fn SystemTimeToTzSpecificLocalTimeEx(
        lpTimeZoneInformation: *const DYNAMIC_TIME_ZONE_INFORMATION,
        lpUniversalTime: *const SYSTEMTIME, lpLocalTime: LPSYSTEMTIME,
    ) -> BOOL;
    pub fn Thread32First(hSnapshot: HANDLE, lpte: LPTHREADENTRY32) -> BOOL;
    pub fn Thread32Next(hSnapshot: HANDLE, lpte: LPTHREADENTRY32) -> BOOL;
    pub fn Toolhelp32ReadProcessMemory(th32ProcessID: DWORD, lpBaseAddress: LPCVOID,
        lpBuffer: LPVOID, cbRead: SIZE_T, lpNumberOfBytesRead: *mut SIZE_T
    ) -> BOOL;
    pub fn TrySubmitThreadpoolCallback(
        pfns: PTP_SIMPLE_CALLBACK, pv: PVOID, pcbe: PTP_CALLBACK_ENVIRON,
    ) -> BOOL;
    pub fn TzSpecificLocalTimeToSystemTime(
        lpTimeZoneInformation: *const TIME_ZONE_INFORMATION, lpLocalTime: *const SYSTEMTIME,
        lpUniversalTime: LPSYSTEMTIME,
    ) -> BOOL;
    pub fn TzSpecificLocalTimeToSystemTimeEx(
        lpTimeZoneInformation: *const DYNAMIC_TIME_ZONE_INFORMATION,
        lpLocalTime: *const SYSTEMTIME, lpUniversalTime: LPSYSTEMTIME,
    ) -> BOOL;
    pub fn UnregisterWaitEx(WaitHandle: HANDLE, CompletionEvent: HANDLE) -> BOOL;
    pub fn VerLanguageNameA(wLang: DWORD, szLang: LPSTR, cchLang: DWORD) -> DWORD;
    pub fn VerLanguageNameW(wLang: DWORD, szLang: LPWSTR, cchLang: DWORD) -> DWORD;
    pub fn WaitForDebugEvent(lpDebugEvent: LPDEBUG_EVENT, dwMilliseconds: DWORD) -> BOOL;
    pub fn WaitForMultipleObjectsEx(
        nCount: DWORD, lpHandles: *const HANDLE, bWaitAll: BOOL, dwMilliseconds: DWORD,
        bAlertable: BOOL,
    ) -> DWORD;
    pub fn WaitForThreadpoolIoCallbacks(pio: PTP_IO, fCancelPendingCallbacks: BOOL);
    pub fn WaitForThreadpoolTimerCallbacks(pti: PTP_TIMER, fCancelPendingCallbacks: BOOL);
    pub fn WaitForThreadpoolWaitCallbacks(pwa: PTP_WAIT, fCancelPendingCallbacks: BOOL);
    pub fn WaitForThreadpoolWorkCallbacks(pwk: PTP_WORK, fCancelPendingCallbacks: BOOL);
    pub fn WideCharToMultiByte(
        CodePage: UINT, dwFlags: DWORD, lpWideCharStr: LPCWSTR, cchWideChar: c_int,
        lpMultiByteStr: LPSTR, cbMultiByte: c_int, lpDefaultChar: LPCSTR, lpUsedDefaultChar: LPBOOL,
    ) -> c_int;
    pub fn Wow64DisableWow64FsRedirection(OldValue: *mut PVOID) -> BOOL;
    pub fn Wow64RevertWow64FsRedirection(OlValue: PVOID) -> BOOL;
    pub fn lstrcat(lpString1: LPSTR, lpString2: LPCSTR) -> LPSTR;
    pub fn lstrcmp(lpString1: LPCSTR, lpString2: LPCSTR) -> c_int;
    pub fn lstrcmpi(lpString1: LPCSTR, lpString2: LPCSTR) -> c_int;
    pub fn lstrcpy(lpString1: LPSTR, lpString2: LPCSTR) -> LPSTR;
    pub fn lstrcpyn(lpString1: LPSTR, lpString2: LPCSTR, iMaxLength: c_int) -> LPSTR;
    pub fn lstrlen(lpString: LPCSTR) -> c_int;
    #[cfg(any(target_arch = "arm", target_arch = "x86_64"))]
    pub fn uaw_lstrcmpW(String1: PCUWSTR, String2: PCUWSTR) -> c_int;
    #[cfg(any(target_arch = "arm", target_arch = "x86_64"))]
    pub fn uaw_lstrcmpiW(String1: PCUWSTR, String2: PCUWSTR) -> c_int;
    #[cfg(any(target_arch = "arm", target_arch = "x86_64"))]
    pub fn uaw_lstrlenW(String: LPCUWSTR) -> c_int;
    #[cfg(any(target_arch = "arm", target_arch = "x86_64"))]
    pub fn uaw_wcschr(String: PCUWSTR, Character: WCHAR) -> PUWSTR;
    #[cfg(any(target_arch = "arm", target_arch = "x86_64"))]
    pub fn uaw_wcscpy(Destination: PUWSTR, Source: PCUWSTR) -> PUWSTR;
    #[cfg(any(target_arch = "arm", target_arch = "x86_64"))]
    pub fn uaw_wcsicmp(String1: PCUWSTR, String2: PCUWSTR) -> c_int;
    #[cfg(any(target_arch = "arm", target_arch = "x86_64"))]
    pub fn uaw_wcslen(String: PCUWSTR) -> size_t;
    #[cfg(any(target_arch = "arm", target_arch = "x86_64"))]
    pub fn uaw_wcsrchr(String: PCUWSTR, Character: WCHAR) -> PUWSTR;
}

// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to kernel32.
#![cfg(windows)]
extern crate winapi;
use winapi::*;
extern "system" {
    // pub fn AddLocalAlternateComputerNameA();
    // pub fn AddLocalAlternateComputerNameW();
    // pub fn AppXGetOSMaxVersionTested();
    // pub fn BaseSetLastNTError();
    pub fn Beep(dwFreq: DWORD, dwDuration: DWORD) -> BOOL;
    pub fn CallNamedPipeW(
        lpNamedPipeName: LPCWSTR, lpInBuffer: LPVOID, nInBufferSize: DWORD, lpOutBuffer: LPVOID,
        nOutBufferSize: DWORD, lpBytesRead: LPDWORD, nTimeOut: DWORD,
    ) -> BOOL;
    pub fn CallbackMayRunLong(pci: PTP_CALLBACK_INSTANCE) -> BOOL;
    pub fn CalloutOnFiberStack(
        lpFiber: PVOID, lpStartAddress: PFIBER_CALLOUT_ROUTINE, lpParameter: PVOID,
    ) -> PVOID;
    pub fn CancelIo(hFile: HANDLE) -> BOOL;
    pub fn CancelIoEx(hFile: HANDLE, lpOverlapped: LPOVERLAPPED) -> BOOL;
    pub fn CancelSynchronousIo(hThread: HANDLE) -> BOOL;
    pub fn CancelThreadpoolIo(pio: PTP_IO);
    pub fn CeipIsOptedIn() -> BOOL;
    pub fn ChangeTimerQueueTimer(
        TimerQueue: HANDLE, Timer: HANDLE, DueTime: ULONG, Period: ULONG,
    ) -> BOOL;
    // pub fn CheckElevation();
    // pub fn CheckElevationEnabled();
    pub fn CheckRemoteDebuggerPresent(hProcess: HANDLE, pbDebuggerPresent: PBOOL) -> BOOL;
    // pub fn ClosePackageInfo();
    // pub fn CloseState();
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
    pub fn ConnectNamedPipe(hNamedPipe: HANDLE, lpOverlapped: LPOVERLAPPED) -> BOOL;
    pub fn ContinueDebugEvent(
        dwProcessId: DWORD, dwThreadId: DWORD, dwContinueStatus: DWORD,
    ) -> BOOL;
    pub fn ConvertFiberToThread() -> BOOL;
    pub fn CreateIoCompletionPort(
        FileHandle: HANDLE, ExistingCompletionPort: HANDLE, CompletionKey: ULONG_PTR,
        NumberOfConcurrentThreads: DWORD,
    ) -> HANDLE;
    pub fn CreateNamedPipeW(
        lpName: LPCWSTR, dwOpenMode: DWORD, dwPipeMode: DWORD, nMaxInstances: DWORD,
        nOutBufferSize: DWORD, nInBufferSize: DWORD, nDefaultTimeOut: DWORD,
        lpSecurityAttributes: LPSECURITY_ATTRIBUTES,
    ) -> HANDLE;
    pub fn CreatePipe(
        hReadPipe: PHANDLE, hWritePipe: PHANDLE, lpPipeAttributes: LPSECURITY_ATTRIBUTES,
        nSize: DWORD,
    ) -> BOOL;
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
    // pub fn CtrlRoutine();
    pub fn DebugActiveProcess(dwProcessId: DWORD) -> BOOL;
    pub fn DebugActiveProcessStop(dwProcessId: DWORD) -> BOOL;
    pub fn DebugBreak();
    pub fn DecodePointer(Ptr: PVOID) -> PVOID;
    pub fn DecodeSystemPointer(Ptr: PVOID) -> PVOID;
    pub fn DelayLoadFailureHook(pszDllName: LPCSTR, pszProcName: LPCSTR) -> FARPROC;
    pub fn DeleteTimerQueueEx(TimerQueue: HANDLE, CompletionEvent: HANDLE) -> BOOL;
    pub fn DeleteTimerQueueTimer(
        TimerQueue: HANDLE, Timer: HANDLE, CompletionEvent: HANDLE,
    ) -> BOOL;
    pub fn DeviceIoControl(
        hDevice: HANDLE, dwIoControlCode: DWORD, lpInBuffer: LPVOID, nInBufferSize: DWORD,
        lpOutBuffer: LPVOID, nOutBufferSize: DWORD, lpBytesReturned: LPDWORD,
        lpOverlapped: LPOVERLAPPED,
    ) -> BOOL;
    pub fn DisassociateCurrentThreadFromCallback(pci: PTP_CALLBACK_INSTANCE);
    pub fn DisconnectNamedPipe(hNamedPipe: HANDLE) -> BOOL;
    pub fn DnsHostnameToComputerNameExW(
        Hostname: LPCWSTR, ComputerName: LPWSTR, nSize: LPDWORD,
    ) -> BOOL;
    // pub fn DosPathToSessionPathW();
    pub fn EncodePointer(Ptr: PVOID) -> PVOID;
    pub fn EncodeSystemPointer(Ptr: PVOID) -> PVOID;
    pub fn EnumSystemFirmwareTables(
        FirmwareTableProviderSignature: DWORD, pFirmwareTableEnumBuffer: PVOID, BufferSize: DWORD,
    ) -> UINT;
    // pub fn EnumerateLocalComputerNamesA();
    // pub fn EnumerateLocalComputerNamesW();
    pub fn ExpandEnvironmentStringsA(lpSrc: LPCSTR, lpDst: LPSTR, nSize: DWORD) -> DWORD;
    pub fn ExpandEnvironmentStringsW(lpSrc: LPCWSTR, lpDst: LPWSTR, nSize: DWORD) -> DWORD;
    pub fn FileTimeToSystemTime(
        lpFileTime: *const FILETIME, lpSystemTime: LPSYSTEMTIME,
    ) -> BOOL;
    pub fn FindClose(hFindFile: HANDLE) -> BOOL;
    // pub fn FindPackagesByPackageFamily();
    pub fn FlsAlloc(lpCallback: PFLS_CALLBACK_FUNCTION) -> DWORD;
    pub fn FlsFree(dwFlsIndex: DWORD) -> BOOL;
    pub fn FlsGetValue(dwFlsIndex: DWORD) -> PVOID;
    pub fn FlsSetValue(dwFlsIndex: DWORD, lpFlsData: PVOID) -> BOOL;
    pub fn FoldStringW(
        dwMapFlags: DWORD, lpSrcStr: LPCWCH, cchSrc: c_int, lpDestStr: LPWSTR, cchDest: c_int,
    ) -> c_int;
    // pub fn FormatApplicationUserModelId();
    pub fn FreeEnvironmentStringsA(penv: LPCH) -> BOOL;
    pub fn FreeEnvironmentStringsW(penv: LPWCH) -> BOOL;
    pub fn FreeLibraryWhenCallbackReturns(pci: PTP_CALLBACK_INSTANCE, module: HMODULE);
    pub fn GetAppContainerNamedObjectPath(
        Token: HANDLE, AppContainerSid: PSID, ObjectPathLength: ULONG, ObjectPath: LPWSTR,
        ReturnLength: PULONG,
    ) -> BOOL;
    // pub fn GetApplicationUserModelId();
    pub fn GetCommandLineA() -> LPSTR;
    pub fn GetCommandLineW() -> LPWSTR;
    pub fn GetComputerNameExA(
        NameType: COMPUTER_NAME_FORMAT, lpBuffer: LPSTR, nSize: LPDWORD,
    ) -> BOOL;
    pub fn GetComputerNameExW(
        NameType: COMPUTER_NAME_FORMAT, lpBuffer: LPWSTR, nSize: LPDWORD,
    ) -> BOOL;
    // pub fn GetCurrentApplicationUserModelId();
    pub fn GetCurrentDirectoryA(nBufferLength: DWORD, lpBuffer: LPSTR) -> DWORD;
    pub fn GetCurrentDirectoryW(nBufferLength: DWORD, lpBuffer: LPWSTR) -> DWORD;
    // pub fn GetCurrentPackageFamilyName();
    // pub fn GetCurrentPackageFullName();
    // pub fn GetCurrentPackageId();
    // pub fn GetCurrentPackageInfo();
    // pub fn GetCurrentPackagePath();
    pub fn GetDateFormatA(
        Locale: LCID, dwFlags: DWORD, lpDate: *const SYSTEMTIME, lpFormat: LPCSTR, lpDateStr: LPSTR,
        cchDate: c_int,
    ) -> c_int;
    pub fn GetDateFormatEx(
        lpLocaleName: LPCWSTR, dwFlags: DWORD, lpDate: *const SYSTEMTIME, lpFormat: LPCWSTR,
        lpDateStr: LPWSTR, cchDate: c_int, lpCalendar: LPCWSTR,
    ) -> c_int;
    pub fn GetDateFormatW(
        Locale: LCID, dwFlags: DWORD, lpDate: *const SYSTEMTIME, lpFormat: LPCWSTR,
        lpDateStr: LPWSTR, cchDate: c_int,
    ) -> c_int;
    pub fn GetDynamicTimeZoneInformation(
        pTimeZoneInformation: PDYNAMIC_TIME_ZONE_INFORMATION,
    ) -> DWORD;
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    pub fn GetEnvironmentStrings() -> LPCH;
    pub fn GetEnvironmentStringsW() -> LPWCH;
    pub fn GetEnvironmentVariableA(lpName: LPCSTR, lpBuffer: LPSTR, nSize: DWORD) -> DWORD;
    pub fn GetEnvironmentVariableW(lpName: LPCWSTR, lpBuffer: LPWSTR, nSize: DWORD) -> DWORD;
    // pub fn GetEraNameCountedString();
    pub fn GetLocalTime(lpSystemTime: LPSYSTEMTIME);
    pub fn GetLogicalProcessorInformation(
        Buffer: PSYSTEM_LOGICAL_PROCESSOR_INFORMATION, ReturnedLength: PDWORD,
    ) -> BOOL;
    pub fn GetLogicalProcessorInformationEx(
        RelationshipType: LOGICAL_PROCESSOR_RELATIONSHIP,
        Buffer: PSYSTEM_LOGICAL_PROCESSOR_INFORMATION,
        ReturnedLength: PDWORD,
    ) -> BOOL;
    // pub fn GetNamedPipeAttribute();
    pub fn GetNamedPipeClientComputerNameW(
        Pipe: HANDLE, ClientComputerName: LPWSTR, ClientComputerNameLength: ULONG,
    ) -> BOOL;
    pub fn GetNamedPipeHandleStateW(
        hNamedPipe: HANDLE, lpState: LPDWORD, lpCurInstances: LPDWORD,
        lpMaxCollectionCount: LPDWORD, lpCollectDataTimeout: LPDWORD, lpUserName: LPWSTR,
        nMaxUserNameSize: DWORD,
    ) -> BOOL;
    pub fn GetNamedPipeInfo(
        hNamedPipe: HANDLE, lpFlags: LPDWORD, lpOutBufferSize: LPDWORD, lpInBufferSize: LPDWORD,
        lpMaxInstances: LPDWORD,
    ) -> BOOL;
    pub fn GetNativeSystemInfo(lpSystemInfo: LPSYSTEM_INFO);
    pub fn GetNumaAvailableMemoryNodeEx(Node: USHORT, AvailableBytes: PULONGLONG) -> BOOL;
    pub fn GetNumaHighestNodeNumber(HighestNodeNumber: PULONG) -> BOOL;
    pub fn GetNumaProximityNodeEx(ProximityId: ULONG, NodeNumber: PUSHORT) -> BOOL;
    pub fn GetOverlappedResult(
        hFile: HANDLE, lpOverlapped: LPOVERLAPPED, lpNumberOfBytesTransferred: LPDWORD, bWait: BOOL,
    ) -> BOOL;
    pub fn GetOverlappedResultEx(
        hFile: HANDLE, lpOverlapped: LPOVERLAPPED, lpNumberOfBytesTransferred: LPDWORD,
        dwMilliseconds: DWORD, bAlertable: BOOL,
    ) -> BOOL;
    // pub fn GetPackageApplicationIds();
    // pub fn GetPackageFamilyName();
    // pub fn GetPackageFullName();
    // pub fn GetPackageId();
    // pub fn GetPackageInfo();
    // pub fn GetPackagePath();
    // pub fn GetPackagePathByFullName();
    // pub fn GetPackagesByPackageFamily();
    pub fn GetPhysicallyInstalledSystemMemory(TotalMemoryInKilobytes: PULONGLONG) -> BOOL;
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
    pub fn GetQueuedCompletionStatus(
        CompletionPort: HANDLE, lpNumberOfBytesTransferred: LPDWORD, lpCompletionKey: PULONG_PTR,
        lpOverlapped: *mut LPOVERLAPPED, dwMilliseconds: DWORD,
    ) -> BOOL;
    pub fn GetQueuedCompletionStatusEx(
        CompletionPort: HANDLE, lpCompletionPortEntries: LPOVERLAPPED_ENTRY, ulCount: ULONG,
        ulNumEntriesRemoved: PULONG, dwMilliseconds: DWORD, fAlertable: BOOL,
    ) -> BOOL;
    // pub fn GetStagedPackagePathByFullName();
    // pub fn GetStateFolder();
    pub fn GetStdHandle(nStdHandle: DWORD) -> HANDLE;
    pub fn GetStringTypeExW(
        Locale: LCID, dwInfoType: DWORD, lpSrcStr: LPCWCH, cchSrc: c_int, lpCharType: LPWORD,
    ) -> BOOL;
    // pub fn GetSystemAppDataKey();
    pub fn GetSystemDirectoryA(lpBuffer: LPSTR, uSize: UINT) -> UINT;
    pub fn GetSystemDirectoryW(lpBuffer: LPWSTR, uSize: UINT) -> UINT;
    pub fn GetSystemFirmwareTable(
        FirmwareTableProviderSignature: DWORD, FirmwareTableID: DWORD, pFirmwareTableBuffer: PVOID,
        BufferSize: DWORD,
    ) -> UINT;
    pub fn GetSystemInfo(lpSystemInfo: LPSYSTEM_INFO);
    pub fn GetSystemTime(lpSystemTime: LPSYSTEMTIME);
    pub fn GetSystemTimeAdjustment(
        lpTimeAdjustment: PDWORD, lpTimeIncrement: PDWORD, lpTimeAdjustmentDisabled: PBOOL,
    ) -> BOOL;
    pub fn GetSystemTimeAsFileTime(lpSystemTimeAsFileTime: LPFILETIME);
    pub fn GetSystemTimePreciseAsFileTime(lpSystemTimeAsFileTime: LPFILETIME);
    pub fn GetSystemWindowsDirectoryA(lpBuffer: LPSTR, uSize: UINT) -> UINT;
    pub fn GetSystemWindowsDirectoryW(lpBuffer: LPWSTR, uSize: UINT) -> UINT;
    pub fn GetSystemWow64DirectoryA(lpBuffer: LPSTR, uSize: UINT) -> UINT;
    pub fn GetSystemWow64DirectoryW(lpBuffer: LPWSTR, uSize: UINT) -> UINT;
    pub fn GetThreadGroupAffinity(hThread: HANDLE, GroupAffinity: PGROUP_AFFINITY) -> BOOL;
    pub fn GetThreadSelectorEntry(
        hThread: HANDLE, dwSelector: DWORD, lpSelectorEntry: LPLDT_ENTRY,
    ) -> BOOL;
    pub fn GetTickCount() -> DWORD;
    pub fn GetTickCount64() -> ULONGLONG;
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
    pub fn GetVersion() -> DWORD;
    pub fn GetVersionExA(lpVersionInformation: LPOSVERSIONINFOA) -> BOOL;
    pub fn GetVersionExW(lpVersionInformation: LPOSVERSIONINFOW) -> BOOL;
    pub fn GetVolumePathNamesForVolumeNameW(
        lpszVolumeName: LPCWSTR, lpszVolumePathNames: LPWCH, cchBufferLength: DWORD,
        lpcchReturnLength: PDWORD,
    ) -> BOOL;
    pub fn GetWindowsDirectoryA(lpBuffer: LPSTR, uSize: UINT) -> UINT;
    pub fn GetWindowsDirectoryW(lpBuffer: LPWSTR, uSize: UINT) -> UINT;
    pub fn GetWriteWatch(
        dwFlags: DWORD, lpBaseAddress: PVOID, dwRegionSize: SIZE_T, lpAddresses: *mut PVOID,
        lpdwCount: *mut ULONG_PTR, lpdwGranularity: LPDWORD,
    ) -> UINT;
    pub fn GlobalMemoryStatusEx(lpBuffer: LPMEMORYSTATUSEX) -> BOOL;
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
    pub fn InstallELAMCertificateInfo(ELAMFile: HANDLE) -> BOOL;
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
    pub fn IsThreadpoolTimerSet(pti: PTP_TIMER) -> BOOL;
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
    // pub fn LoadAppInitDlls();
    // pub fn LoadStringBaseExW();
    // pub fn LoadStringBaseW();
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
    // pub fn NotifyMountMgr();
    // pub fn OOBEComplete();
    // pub fn OpenPackageInfoByFullName();
    // pub fn OpenState();
    // pub fn OpenStateExplicit();
    pub fn OutputDebugStringA(lpOutputString: LPCSTR);
    pub fn OutputDebugStringW(lpOutputString: LPCWSTR);
    // pub fn PackageFamilyNameFromFullName();
    // pub fn PackageFamilyNameFromId();
    // pub fn PackageFullNameFromId();
    // pub fn PackageIdFromFullName();
    // pub fn PackageNameAndPublisherIdFromFamilyName();
    // pub fn ParseApplicationUserModelId();
    pub fn PeekNamedPipe(
        hNamedPipe: HANDLE, lpBuffer: LPVOID, nBufferSize: DWORD, lpBytesRead: LPDWORD,
        lpTotalBytesAvail: LPDWORD, lpBytesLeftThisMessage: LPDWORD,
    ) -> BOOL;
    pub fn PostQueuedCompletionStatus(
        CompletionPort: HANDLE, dwNumberOfBytesTransferred: DWORD, dwCompletionKey: ULONG_PTR,
        lpOverlapped: LPOVERLAPPED,
    ) -> BOOL;
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
    pub fn QueryThreadpoolStackInformation(
        ptpp: PTP_POOL, ptpsi: PTP_POOL_STACK_INFORMATION,
    ) -> BOOL;
    pub fn QueryUnbiasedInterruptTime(UnbiasedTime: PULONGLONG) -> BOOL;
    pub fn QueueUserWorkItem(
        Function: LPTHREAD_START_ROUTINE, Context: PVOID, Flags: ULONG,
    ) -> BOOL;
    pub fn RegisterApplicationRestart(pwzCommandline: PCWSTR, dwFlags: DWORD) -> HRESULT;
    // pub fn RegisterWaitForInputIdle();
    pub fn RegisterWaitForSingleObjectEx(
        hObject: HANDLE, Callback: WAITORTIMERCALLBACK, Context: PVOID, dwMilliseconds: ULONG,
        dwFlags: ULONG,
    ) -> HANDLE;
    // pub fn RegisterWaitUntilOOBECompleted();
    pub fn ReleaseMutexWhenCallbackReturns(pci: PTP_CALLBACK_INSTANCE, mutex: HANDLE);
    pub fn ReleaseSemaphoreWhenCallbackReturns(
        pci: PTP_CALLBACK_INSTANCE, sem: HANDLE, crel: DWORD,
    );
    // pub fn RemoveLocalAlternateComputerNameA();
    // pub fn RemoveLocalAlternateComputerNameW();
    pub fn ResetWriteWatch(lpBaseAddress: LPVOID, dwRegionSize: SIZE_T) -> UINT;
    // pub fn ResolveDelayLoadedAPI();
    // pub fn ResolveDelayLoadsFromDll();
    #[cfg(target_arch = "arm")]
    pub fn RtlAddFunctionTable(
        FunctionTable: PRUNTIME_FUNCTION, EntryCount: DWORD, BaseAddress: DWORD,
    ) -> BOOLEAN;

    // #[cfg(any(target_arch = "arm", target_arch = "x86_64"))]

    // #[cfg(any(target_arch = "arm", target_arch = "x86_64"))]
    pub fn RtlCopyMemory(Destination: PVOID, Source: *const VOID, Length: SIZE_T);

    // pub fn RtlFillMemory();
    #[cfg(target_arch = "arm")]
    pub fn RtlInstallFunctionTableCallback(
        TableIdentifier: DWORD, BaseAddress: DWORD, Length: DWORD,
        Callback: PGET_RUNTIME_FUNCTION_CALLBACK, Context: PVOID, OutOfProcessCallbackDll: PCWSTR,
    ) -> BOOLEAN;

    #[cfg(target_arch = "arm")]
    pub fn RtlLookupFunctionEntry(
        ControlPc: ULONG_PTR, ImageBase: PDWORD, HistoryTable: PUNWIND_HISTORY_TABLE,
    ) -> PRUNTIME_FUNCTION;

    // pub fn RtlMoveMemory();
    // #[cfg(any(target_arch = "arm", target_arch = "x86_64"))]

    // #[cfg(any(target_arch = "arm", target_arch = "x86_64"))]
    // pub fn RtlRaiseException();


    #[cfg(any(target_arch = "arm", target_arch = "x86_64"))]

    #[cfg(target_arch = "arm")]
    pub fn RtlVirtualUnwind(
        HandlerType: DWORD, ImageBase: DWORD, ControlPc: DWORD, FunctionEntry: PRUNTIME_FUNCTION,
        ContextRecord: PCONTEXT, HandlerData: *mut PVOID, EstablisherFrame: PDWORD,
        ContextPointers: PKNONVOLATILE_CONTEXT_POINTERS,
    ) -> PEXCEPTION_ROUTINE;
    #[cfg(target_arch = "x86_64")]

    // pub fn RtlZeroMemory();
    pub fn SearchPathA(
        lpPath: LPCSTR, lpFileName: LPCSTR, lpExtension: LPCSTR, nBufferLength: DWORD,
        lpBuffer: LPSTR, lpFilePart: *mut LPSTR,
    ) -> DWORD;
    pub fn SearchPathW(
        lpPath: LPCWSTR, lpFileName: LPCWSTR, lpExtension: LPCWSTR, nBufferLength: DWORD,
        lpBuffer: LPWSTR, lpFilePart: *mut LPWSTR,
    ) -> DWORD;
    pub fn SetComputerNameA(lpComputerName: LPCSTR) -> BOOL;
    pub fn SetComputerNameEx2W(
        NameType: COMPUTER_NAME_FORMAT, Flags: DWORD, lpBuffer: LPCWSTR,
    ) -> BOOL;
    pub fn SetComputerNameExA(NameType: COMPUTER_NAME_FORMAT, lpBuffer: LPCSTR) -> BOOL;
    pub fn SetComputerNameExW(NameType: COMPUTER_NAME_FORMAT, lpBuffer: LPCWSTR) -> BOOL;
    pub fn SetComputerNameW(lpComputerName: LPCWSTR) -> BOOL;
    // pub fn SetConsoleCursor();
    pub fn SetCurrentDirectoryA(lpPathName: LPCSTR) -> BOOL;
    pub fn SetCurrentDirectoryW(lpPathName: LPCWSTR) -> BOOL;
    pub fn SetDynamicTimeZoneInformation(
        lpTimeZoneInformation: *const DYNAMIC_TIME_ZONE_INFORMATION,
    ) -> BOOL;
    pub fn SetEnvironmentStringsW(NewEnvironment: LPWCH) -> BOOL;
    pub fn SetEnvironmentVariableA(lpName: LPCSTR, lpValue: LPCSTR) -> BOOL;
    pub fn SetEnvironmentVariableW(lpName: LPCWSTR, lpValue: LPCWSTR) -> BOOL;
    pub fn SetEventWhenCallbackReturns(pci: PTP_CALLBACK_INSTANCE, evt: HANDLE);
    // pub fn SetLocalPrimaryComputerNameA();
    // pub fn SetLocalPrimaryComputerNameW();
    pub fn SetLocalTime(lpSystemTime: *const SYSTEMTIME) -> BOOL;
    pub fn SetNamedPipeAttribute(
        Pipe: HANDLE, AttributeType: PIPE_ATTRIBUTE_TYPE, AttributeName: PSTR,
        AttributeValue: PVOID, AttributeValueLength: SIZE_T,
    ) -> BOOL;
    pub fn SetNamedPipeHandleState(
        hNamedPipe: HANDLE, lpMode: LPDWORD, lpMaxCollectionCount: LPDWORD,
        lpCollectDataTimeout: LPDWORD,
    ) -> BOOL;
    pub fn SetStdHandle(nStdHandle: DWORD, hHandle: HANDLE) -> BOOL;
    pub fn SetStdHandleEx(nStdHandle: DWORD, hHandle: HANDLE, phPrevValue: PHANDLE) -> BOOL;
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
    pub fn SetTimeZoneInformation(lpTimeZoneInformation: *const TIME_ZONE_INFORMATION) -> BOOL;
    pub fn StartThreadpoolIo(pio: PTP_IO);
    pub fn SubmitThreadpoolWork(pwk: PTP_WORK);
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
    pub fn TransactNamedPipe(
        hNamedPipe: HANDLE, lpInBuffer: LPVOID, nInBufferSize: DWORD, lpOutBuffer: LPVOID,
        nOutBufferSize: DWORD, lpBytesRead: LPDWORD, lpOverlapped: LPOVERLAPPED,
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
    // pub fn UnregisterWaitUntilOOBECompleted();
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
    pub fn WaitNamedPipeW(lpNamedPipeName: LPCWSTR, nTimeOut: DWORD) -> BOOL;
    // pub fn WerpInitiateRemoteRecovery();
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

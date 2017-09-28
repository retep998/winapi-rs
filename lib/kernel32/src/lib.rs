// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to kernel32.
#![cfg(windows)]
extern crate winapi;
use winapi::*;
extern "system" {
    // pub fn AddLocalAlternateComputerNameA();
    // pub fn AddLocalAlternateComputerNameW();
    pub fn AllocateUserPhysicalPages(
        hProcess: HANDLE, NumberOfPages: PULONG_PTR, PageArray: PULONG_PTR,
    ) -> BOOL;
    pub fn AllocateUserPhysicalPagesNuma(
        hProcess: HANDLE, NumberOfPages: PULONG_PTR, PageArray: PULONG_PTR, nndPreferred: DWORD,
    ) -> BOOL;
    // pub fn AppXGetOSMaxVersionTested();
    pub fn AssignProcessToJobObject(hJob: HANDLE, hProcess: HANDLE) -> BOOL;
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
    pub fn CompareStringA(
        Locale: LCID, dwCmpFlags: DWORD, lpString1: PCNZCH, cchCount1: c_int, lpString2: PCNZCH,
        cchCount2: c_int,
    ) -> c_int;
    pub fn CompareStringEx(
        lpLocaleName: LPCWSTR, dwCmpFlags: DWORD, lpString1: LPCWCH, cchCount1: c_int,
        lpString2: LPCWCH, cchCount2: c_int, lpVersionInformation: LPNLSVERSIONINFO,
        lpReserved: LPVOID, lParam: LPARAM,
    ) -> c_int;
    pub fn CompareStringOrdinal(
        lpString1: LPCWCH, cchCount1: c_int, lpString2: LPCWCH, cchCount2: c_int, bIgnoreCase: BOOL,
    ) -> c_int;
    pub fn CompareStringW(
        Locale: LCID, dwCmpFlags: DWORD, lpString1: PCNZWCH, cchCount1: c_int, lpString2: PCNZWCH,
        cchCount2: c_int,
    ) -> c_int;
    pub fn ConnectNamedPipe(hNamedPipe: HANDLE, lpOverlapped: LPOVERLAPPED) -> BOOL;
    pub fn ContinueDebugEvent(
        dwProcessId: DWORD, dwThreadId: DWORD, dwContinueStatus: DWORD,
    ) -> BOOL;
    pub fn ConvertDefaultLocale(Locale: LCID) -> LCID;
    pub fn ConvertFiberToThread() -> BOOL;
    pub fn CreateFileMappingFromApp(
        hFile: HANDLE, SecurityAttributes: PSECURITY_ATTRIBUTES, PageProtection: ULONG,
        MaximumSize: ULONG64, Name: PCWSTR,
    ) -> HANDLE;
    pub fn CreateFileMappingNumaW(
        hFile: HANDLE, lpFileMappingAttributes: LPSECURITY_ATTRIBUTES, flProtect: DWORD,
        dwMaximumSizeHigh: DWORD, dwMaximumSizeLow: DWORD, lpName: LPCWSTR, nndPreferred: DWORD,
    ) -> HANDLE;
    pub fn CreateFileMappingW(
        hFile: HANDLE, lpAttributes: LPSECURITY_ATTRIBUTES, flProtect: DWORD,
        dwMaximumSizeHigh: DWORD, dwMaximumSizeLow: DWORD, lpName: LPCWSTR,
    ) -> HANDLE;
    pub fn CreateIoCompletionPort(
        FileHandle: HANDLE, ExistingCompletionPort: HANDLE, CompletionKey: ULONG_PTR,
        NumberOfConcurrentThreads: DWORD,
    ) -> HANDLE;
    pub fn CreateJobObjectW(lpJobAttributes: LPSECURITY_ATTRIBUTES, lpName: LPCWSTR) -> HANDLE;
    pub fn CreateMemoryResourceNotification(
        NotificationType: MEMORY_RESOURCE_NOTIFICATION_TYPE,
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
    pub fn EnumCalendarInfoA(
        lpCalInfoEnumProc: CALINFO_ENUMPROCA, Locale: LCID, Calendar: CALID, CalType: CALTYPE,
    ) -> BOOL;
    pub fn EnumCalendarInfoExA(
        lpCalInfoEnumProcEx: CALINFO_ENUMPROCEXA, Locale: LCID, Calendar: CALID, CalType: CALTYPE,
    ) -> BOOL;
    pub fn EnumCalendarInfoExEx(
        pCalInfoEnumProcExEx: CALINFO_ENUMPROCEXEX, lpLocaleName: LPCWSTR, Calendar: CALID,
        lpReserved: LPCWSTR, CalType: CALTYPE, lParam: LPARAM,
    ) -> BOOL;
    pub fn EnumCalendarInfoExW(
        lpCalInfoEnumProcEx: CALINFO_ENUMPROCEXW, Locale: LCID, Calendar: CALID, CalType: CALTYPE,
    ) -> BOOL;
    pub fn EnumCalendarInfoW(
        lpCalInfoEnumProc: CALINFO_ENUMPROCW, Locale: LCID, Calendar: CALID, CalType: CALTYPE,
    ) -> BOOL;
    pub fn EnumDateFormatsA(
        lpDateFmtEnumProc: DATEFMT_ENUMPROCA, Locale: LCID, dwFlags: DWORD,
    ) -> BOOL;
    pub fn EnumDateFormatsExA(
        lpDateFmtEnumProcEx: DATEFMT_ENUMPROCEXA, Locale: LCID, dwFlags: DWORD,
    ) -> BOOL;
    pub fn EnumDateFormatsExEx(
        lpDateFmtEnumProcExEx: DATEFMT_ENUMPROCEXEX, lpLocaleName: LPCWSTR, dwFlags: DWORD,
        lParam: LPARAM,
    ) -> BOOL;
    pub fn EnumDateFormatsExW(
        lpDateFmtEnumProcEx: DATEFMT_ENUMPROCEXW, Locale: LCID, dwFlags: DWORD,
    ) -> BOOL;
    pub fn EnumDateFormatsW(
        lpDateFmtEnumProc: DATEFMT_ENUMPROCW, Locale: LCID, dwFlags: DWORD,
    ) -> BOOL;
    pub fn EnumLanguageGroupLocalesA(
        lpLangGroupLocaleEnumProc: LANGGROUPLOCALE_ENUMPROCA, LanguageGroup: LGRPID, dwFlags: DWORD,
        lParam: LONG_PTR,
    ) -> BOOL;
    pub fn EnumLanguageGroupLocalesW(
        lpLangGroupLocaleEnumProc: LANGGROUPLOCALE_ENUMPROCW, LanguageGroup: LGRPID, dwFlags: DWORD,
        lParam: LONG_PTR,
    ) -> BOOL;
    pub fn EnumSystemCodePagesA(lpCodePageEnumProc: CODEPAGE_ENUMPROCA, dwFlags: DWORD) -> BOOL;
    pub fn EnumSystemCodePagesW(lpCodePageEnumProc: CODEPAGE_ENUMPROCW, dwFlags: DWORD) -> BOOL;
    pub fn EnumSystemFirmwareTables(
        FirmwareTableProviderSignature: DWORD, pFirmwareTableEnumBuffer: PVOID, BufferSize: DWORD,
    ) -> UINT;
    pub fn EnumSystemGeoID(
        GeoClass: GEOCLASS, ParentGeoId: GEOID, lpGeoEnumProc: GEO_ENUMPROC,
    ) -> BOOL;
    pub fn EnumSystemLanguageGroupsA(
        lpLanguageGroupEnumProc: LANGUAGEGROUP_ENUMPROCA, dwFlags: DWORD, lParam: LONG_PTR,
    ) -> BOOL;
    pub fn EnumSystemLanguageGroupsW(
        lpLanguageGroupEnumProc: LANGUAGEGROUP_ENUMPROCW, dwFlags: DWORD, lParam: LONG_PTR,
    ) -> BOOL;
    pub fn EnumSystemLocalesA(lpLocaleEnumProc: LOCALE_ENUMPROCA, dwFlags: DWORD) -> BOOL;
    pub fn EnumSystemLocalesEx(
        lpLocaleEnumProcEx: LOCALE_ENUMPROCEX, dwFlags: DWORD, lParam: LPARAM, lpReserved: LPVOID,
    ) -> BOOL;
    pub fn EnumSystemLocalesW(lpLocaleEnumProc: LOCALE_ENUMPROCW, dwFlags: DWORD) -> BOOL;
    pub fn EnumTimeFormatsA(
        lpTimeFmtEnumProc: TIMEFMT_ENUMPROCA, Locale: LCID, dwFlags: DWORD,
    ) -> BOOL;
    pub fn EnumTimeFormatsEx(
        lpTimeFmtEnumProcEx: TIMEFMT_ENUMPROCEX, lpLocaleName: LPCWSTR, dwFlags: DWORD,
        lParam: LPARAM,
    ) -> BOOL;
    pub fn EnumTimeFormatsW(
        lpTimeFmtEnumProc: TIMEFMT_ENUMPROCW, Locale: LCID, dwFlags: DWORD,
    ) -> BOOL;
    pub fn EnumUILanguagesA(
        lpUILanguageEnumProc: UILANGUAGE_ENUMPROCA, dwFlags: DWORD, lParam: LONG_PTR,
    ) -> BOOL;
    pub fn EnumUILanguagesW(
        lpUILanguageEnumProc: UILANGUAGE_ENUMPROCW, dwFlags: DWORD, lParam: LONG_PTR,
    ) -> BOOL;
    // pub fn EnumerateLocalComputerNamesA();
    // pub fn EnumerateLocalComputerNamesW();
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
    // pub fn FindPackagesByPackageFamily();
    pub fn FlsAlloc(lpCallback: PFLS_CALLBACK_FUNCTION) -> DWORD;
    pub fn FlsFree(dwFlsIndex: DWORD) -> BOOL;
    pub fn FlsGetValue(dwFlsIndex: DWORD) -> PVOID;
    pub fn FlsSetValue(dwFlsIndex: DWORD, lpFlsData: PVOID) -> BOOL;
    pub fn FlushViewOfFile(lpBaseAddress: LPCVOID, dwNumberOfBytesToFlush: SIZE_T) -> BOOL;
    pub fn FoldStringA(
        dwMapFlags: DWORD, lpSrcStr: LPCSTR, cchSrc: c_int, lpDestStr: LPSTR, cchDest: c_int,
    ) -> c_int;
    pub fn FoldStringW(
        dwMapFlags: DWORD, lpSrcStr: LPCWCH, cchSrc: c_int, lpDestStr: LPWSTR, cchDest: c_int,
    ) -> c_int;
    // pub fn FormatApplicationUserModelId();
    pub fn FreeEnvironmentStringsA(penv: LPCH) -> BOOL;
    pub fn FreeEnvironmentStringsW(penv: LPWCH) -> BOOL;
    pub fn FreeLibraryWhenCallbackReturns(pci: PTP_CALLBACK_INSTANCE, module: HMODULE);
    pub fn FreeUserPhysicalPages(
        hProcess: HANDLE, NumberOfPages: PULONG_PTR, PageArray: PULONG_PTR,
    ) -> BOOL;
    pub fn GetACP() -> UINT;
    pub fn GetAppContainerNamedObjectPath(
        Token: HANDLE, AppContainerSid: PSID, ObjectPathLength: ULONG, ObjectPath: LPWSTR,
        ReturnLength: PULONG,
    ) -> BOOL;
    // pub fn GetApplicationUserModelId();
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
    pub fn GetComputerNameExA(
        NameType: COMPUTER_NAME_FORMAT, lpBuffer: LPSTR, nSize: LPDWORD,
    ) -> BOOL;
    pub fn GetComputerNameExW(
        NameType: COMPUTER_NAME_FORMAT, lpBuffer: LPWSTR, nSize: LPDWORD,
    ) -> BOOL;
    pub fn GetCurrencyFormatA(
        Locale: LCID, dwFlags: DWORD, lpValue: LPCSTR, lpFormat: *const CURRENCYFMTA,
        lpCurrencyStr: LPSTR, cchCurrency: c_int,
    ) -> c_int;
    pub fn GetCurrencyFormatEx(
        lpLocaleName: LPCWSTR, dwFlags: DWORD, lpValue: LPCWSTR, lpFormat: *const CURRENCYFMTW,
        lpCurrencyStr: LPWSTR, cchCurrency: c_int,
    ) -> c_int;
    pub fn GetCurrencyFormatW(
        Locale: LCID, dwFlags: DWORD, lpValue: LPCWSTR, lpFormat: *const CURRENCYFMTW,
        lpCurrencyStr: LPWSTR, cchCurrency: c_int,
    ) -> c_int;
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
    pub fn GetDurationFormat(
        Locale: LCID, dwFlags: DWORD, lpDuration: *const SYSTEMTIME, ullDuration: ULONGLONG,
        lpFormat: LPCWSTR, lpDurationStr: LPWSTR, cchDuration: c_int,
    ) -> c_int;
    pub fn GetDurationFormatEx(
        lpLocaleName: LPCWSTR, dwFlags: DWORD, lpDuration: *const SYSTEMTIME,
        ullDuration: ULONGLONG, lpFormat: LPCWSTR, lpDurationStr: LPWSTR, cchDuration: c_int,
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
    pub fn GetFileMUIInfo(
        dwFlags: DWORD, pcwszFilePath: PCWSTR, pFileMUIInfo: PFILEMUIINFO,
        pcbFileMUIInfo: *mut DWORD,
    ) -> BOOL;
    pub fn GetFileMUIPath(
        dwFlags: DWORD, pcwszFilePath: PCWSTR, pwszLanguage: PWSTR, pcchLanguage: PULONG,
        pwszFileMUIPath: PWSTR, pcchFileMUIPath: PULONG, pululEnumerator: PULONGLONG,
    ) -> BOOL;
    pub fn GetGeoInfoA(
        Location: GEOID, GeoType: GEOTYPE, lpGeoData: LPSTR, cchData: c_int, LangId: LANGID,
    ) -> c_int;
    pub fn GetGeoInfoW(
        Location: GEOID, GeoType: GEOTYPE, lpGeoData: LPWSTR, cchData: c_int, LangId: LANGID,
    ) -> c_int;
    pub fn GetLargePageMinimum() -> SIZE_T;
    pub fn GetLocalTime(lpSystemTime: LPSYSTEMTIME);
    pub fn GetLocaleInfoA(
        Locale: LCID, LCType: LCTYPE, lpLCData: LPSTR, cchData: c_int,
    ) -> c_int;
    pub fn GetLocaleInfoEx(
        lpLocaleName: LPCWSTR, LCType: LCTYPE, lpLCData: LPWSTR, cchData: c_int,
    ) -> c_int;
    pub fn GetLocaleInfoW(
        Locale: LCID, LCType: LCTYPE, lpLCData: LPWSTR, cchData: c_int,
    ) -> c_int;
    pub fn GetLogicalProcessorInformation(
        Buffer: PSYSTEM_LOGICAL_PROCESSOR_INFORMATION, ReturnedLength: PDWORD,
    ) -> BOOL;
    pub fn GetLogicalProcessorInformationEx(
        RelationshipType: LOGICAL_PROCESSOR_RELATIONSHIP,
        Buffer: PSYSTEM_LOGICAL_PROCESSOR_INFORMATION,
        ReturnedLength: PDWORD,
    ) -> BOOL;
    pub fn GetMemoryErrorHandlingCapabilities(Capabilities: PULONG) -> BOOL;
    pub fn GetNLSVersion(
        Function: NLS_FUNCTION, Locale: LCID, lpVersionInformation: LPNLSVERSIONINFO,
    ) -> BOOL;
    pub fn GetNLSVersionEx(
        function: NLS_FUNCTION, lpLocaleName: LPCWSTR, lpVersionInformation: LPNLSVERSIONINFOEX,
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
    pub fn GetNumberFormatA(
        Locale: LCID, dwFlags: DWORD, lpValue: LPCSTR, lpFormat: *const NUMBERFMTA,
        lpNumberStr: LPSTR, cchNumber: c_int,
    ) -> c_int;
    pub fn GetNumberFormatEx(
        lpLocaleName: LPCWSTR, dwFlags: DWORD, lpValue: LPCWSTR, lpFormat: *const NUMBERFMTW,
        lpNumberStr: LPWSTR, cchNumber: c_int,
    ) -> c_int;
    pub fn GetNumberFormatW(
        Locale: LCID, dwFlags: DWORD, lpValue: LPCWSTR, lpFormat: *const NUMBERFMTW,
        lpNumberStr: LPWSTR, cchNumber: c_int,
    ) -> c_int;
    pub fn GetOEMCP() -> UINT;
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
    pub fn GetProcessPreferredUILanguages(
        dwFlags: DWORD, pulNumLanguages: PULONG, pwszLanguagesBuffer: PZZWSTR,
        pcchLanguagesBuffer: PULONG,
    ) -> BOOL;
    pub fn GetProcessWorkingSetSizeEx(
        hProcess: HANDLE, lpMinimumWorkingSetSize: PSIZE_T, lpMaximumWorkingSetSize: PSIZE_T,
        Flags: PDWORD,
    ) -> BOOL;
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
    pub fn GetStringScripts(
        dwFlags: DWORD, lpString: LPCWSTR, cchString: c_int, lpScripts: LPWSTR, cchScripts: c_int,
    ) -> c_int;
    pub fn GetStringTypeA(
        Locale: LCID, dwInfoType: DWORD, lpSrcStr: LPCSTR, cchSrc: c_int, lpCharType: LPWORD,
    ) -> BOOL;
    pub fn GetStringTypeExA(
        Locale: LCID, dwInfoType: DWORD, lpSrcStr: LPCSTR, cchSrc: c_int, lpCharType: LPWORD,
    ) -> BOOL;
    pub fn GetStringTypeExW(
        Locale: LCID, dwInfoType: DWORD, lpSrcStr: LPCWCH, cchSrc: c_int, lpCharType: LPWORD,
    ) -> BOOL;
    pub fn GetStringTypeW(
        dwInfoType: DWORD, lpSrcStr: LPCWCH, cchSrc: c_int, lpCharType: LPWORD,
    ) -> BOOL;
    // pub fn GetSystemAppDataKey();
    pub fn GetSystemDefaultLCID() -> LCID;
    pub fn GetSystemDefaultLangID() -> LANGID;
    pub fn GetSystemDefaultLocaleName(lpLocaleName: LPWSTR, cchLocaleName: c_int) -> c_int;
    pub fn GetSystemDefaultUILanguage() -> LANGID;
    pub fn GetSystemDirectoryA(lpBuffer: LPSTR, uSize: UINT) -> UINT;
    pub fn GetSystemDirectoryW(lpBuffer: LPWSTR, uSize: UINT) -> UINT;
    pub fn GetSystemFileCacheSize(
        lpMinimumFileCacheSize: PSIZE_T, lpMaximumFileCacheSize: PSIZE_T, lpFlags: PDWORD,
    ) -> BOOL;
    pub fn GetSystemFirmwareTable(
        FirmwareTableProviderSignature: DWORD, FirmwareTableID: DWORD, pFirmwareTableBuffer: PVOID,
        BufferSize: DWORD,
    ) -> UINT;
    pub fn GetSystemInfo(lpSystemInfo: LPSYSTEM_INFO);
    pub fn GetSystemPreferredUILanguages(
        dwFlags: DWORD, pulNumLanguages: PULONG, pwszLanguagesBuffer: PZZWSTR,
        pcchLanguagesBuffer: PULONG,
    ) -> BOOL;
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
    pub fn GetThreadLocale() -> LCID;
    pub fn GetThreadPreferredUILanguages(
        dwFlags: DWORD, pulNumLanguages: PULONG, pwszLanguagesBuffer: PZZWSTR,
        pcchLanguagesBuffer: PULONG,
    ) -> BOOL;
    pub fn GetThreadSelectorEntry(
        hThread: HANDLE, dwSelector: DWORD, lpSelectorEntry: LPLDT_ENTRY,
    ) -> BOOL;
    pub fn GetThreadUILanguage() -> LANGID;
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
    pub fn GetUILanguageInfo(
        dwFlags: DWORD, pwmszLanguage: PCZZWSTR, pwszFallbackLanguages: PZZWSTR,
        pcchFallbackLanguages: PDWORD, pAttributes: PDWORD,
    ) -> BOOL;
    pub fn GetUserDefaultLCID() -> LCID;
    pub fn GetUserDefaultLangID() -> LANGID;
    pub fn GetUserDefaultLocaleName(lpLocaleName: LPWSTR, cchLocaleName: c_int) -> c_int;
    pub fn GetUserDefaultUILanguage() -> LANGID;
    pub fn GetUserGeoID(GeoClass: GEOCLASS) -> GEOID;
    pub fn GetUserPreferredUILanguages(
        dwFlags: DWORD, pulNumLanguages: PULONG, pwszLanguagesBuffer: PZZWSTR,
        pcchLanguagesBuffer: PULONG,
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
    pub fn IsDBCSLeadByte(TestChar: BYTE) -> BOOL;
    pub fn IsDBCSLeadByteEx(CodePage: UINT, TestChar: BYTE) -> BOOL;
    pub fn IsDebuggerPresent() -> BOOL;
    pub fn IsNLSDefinedString(
        Function: NLS_FUNCTION, dwFlags: DWORD, lpVersionInformation: LPNLSVERSIONINFO,
        lpString: LPCWSTR, cchStr: INT,
    ) -> BOOL;
    pub fn IsNormalizedString(NormForm: NORM_FORM, lpString: LPCWSTR, cwLength: c_int) -> BOOL;
    pub fn IsProcessInJob(ProcessHandle: HANDLE, JobHandle: HANDLE, Result: PBOOL) -> BOOL;
    pub fn IsThreadAFiber() -> BOOL;
    pub fn IsThreadpoolTimerSet(pti: PTP_TIMER) -> BOOL;
    pub fn IsValidCodePage(CodePage: UINT) -> BOOL;
    pub fn IsValidLanguageGroup(LanguageGroup: LGRPID, dwFlags: DWORD) -> BOOL;
    pub fn IsValidLocale(Locale: LCID, dwFlags: DWORD) -> BOOL;
    pub fn IsValidLocaleName(lpLocaleName: LPCWSTR) -> BOOL;
    pub fn IsValidNLSVersion(
        function: NLS_FUNCTION, lpLocaleName: LPCWSTR, lpVersionInformation: LPNLSVERSIONINFOEX,
    ) -> BOOL;
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
    pub fn LCIDToLocaleName(Locale: LCID, lpName: LPWSTR, cchName: c_int, dwFlags: DWORD) -> c_int;
    pub fn LCMapStringA(
        Locale: LCID, dwMapFlags: DWORD, lpSrcStr: LPCSTR, cchSrc: c_int, lpDestStr: LPSTR,
        cchDest: c_int,
    ) -> c_int;
    pub fn LCMapStringEx(
        lpLocaleName: LPCWSTR, dwMapFlags: DWORD, lpSrcStr: LPCWSTR, cchSrc: c_int,
        lpDestStr: LPWSTR, cchDest: c_int, lpVersionInformation: LPNLSVERSIONINFO,
        lpReserved: LPVOID, sortHandle: LPARAM,
    ) -> c_int;
    pub fn LCMapStringW(
        Locale: LCID, dwMapFlags: DWORD, lpSrcStr: LPCWSTR, cchSrc: c_int, lpDestStr: LPWSTR,
        cchDest: c_int,
    ) -> c_int;
    pub fn LeaveCriticalSectionWhenCallbackReturns(
        pci: PTP_CALLBACK_INSTANCE, pcs: PCRITICAL_SECTION,
    );
    // pub fn LoadAppInitDlls();
    // pub fn LoadStringBaseExW();
    // pub fn LoadStringBaseW();
    pub fn LocalFlags(hMem: HLOCAL) -> UINT;
    pub fn LocaleNameToLCID(lpName: LPCWSTR, dwFlags: DWORD) -> LCID;
    pub fn MapUserPhysicalPages(
        VirtualAddress: PVOID, NumberOfPages: ULONG_PTR, PageArray: PULONG_PTR,
    ) -> BOOL;
    pub fn MapViewOfFile(
        hFileMappingObject: HANDLE, dwDesiredAccess: DWORD, dwFileOffsetHigh: DWORD,
        dwFileOffsetLow: DWORD, dwNumberOfBytesToMap: SIZE_T,
    ) -> LPVOID;
    pub fn MapViewOfFileEx(
        hFileMappingObject: HANDLE, dwDesiredAccess: DWORD, dwFileOffsetHigh: DWORD,
        dwFileOffsetLow: DWORD, dwNumberOfBytesToMap: SIZE_T, lpBaseAddress: LPVOID,
    ) -> LPVOID;
    pub fn MapViewOfFileFromApp(
        hFileMappingObject: HANDLE, DesiredAccess: ULONG, FileOffset: ULONG64,
        NumberOfBytesToMap: SIZE_T,
    ) -> PVOID;
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
    // pub fn NotifyMountMgr();
    pub fn NotifyUILanguageChange(
        dwFlags: DWORD, pcwstrNewLanguage: PCWSTR, pcwstrPreviousLanguage: PCWSTR,
        dwReserved: DWORD, pdwStatusRtrn: PDWORD,
    ) -> BOOL;
    // pub fn OOBEComplete();
    pub fn OpenFileMappingW(
        dwDesiredAccess: DWORD, bInheritHandle: BOOL, lpName: LPCWSTR,
    ) -> HANDLE;
    pub fn OpenJobObjectW(dwDesiredAccess: DWORD, bInheritHandle: BOOL, lpName: LPCWSTR) -> HANDLE;
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
    pub fn PrefetchVirtualMemory(
        hProcess: HANDLE, NumberOfEntries: ULONG_PTR, VirtualAddresses: PWIN32_MEMORY_RANGE_ENTRY,
        Flags: ULONG,
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
    pub fn QueryInformationJobObject(
        hJob: HANDLE, JobObjectInformationClass: JOBOBJECTINFOCLASS,
        lpJobObjectInformation: LPVOID, cbJobObjectInformationLength: DWORD,
        lpReturnLength: LPDWORD,
    ) -> BOOL;
    pub fn QueryMemoryResourceNotification(
        ResourceNotificationHandle: HANDLE, ResourceState: PBOOL,
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
    pub fn ReadProcessMemory(
        hProcess: HANDLE, lpBaseAddress: LPCVOID, lpBuffer: LPVOID, nSize: SIZE_T,
        lpNumberOfBytesRead: *mut SIZE_T,
    ) -> BOOL;
    pub fn RegisterApplicationRestart(pwzCommandline: PCWSTR, dwFlags: DWORD) -> HRESULT;
    pub fn RegisterBadMemoryNotification(Callback: PBAD_MEMORY_CALLBACK_ROUTINE) -> PVOID;
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
    pub fn ResolveLocaleName(
        lpNameToResolve: LPCWSTR, lpLocaleName: LPWSTR, cchLocaleName: c_int,
    ) -> c_int;
    #[cfg(target_arch = "arm")]
    pub fn RtlAddFunctionTable(
        FunctionTable: PRUNTIME_FUNCTION, EntryCount: DWORD, BaseAddress: DWORD,
    ) -> BOOLEAN;
    #[cfg(target_arch = "x86_64")]
    pub fn RtlAddFunctionTable(
        FunctionTable: PRUNTIME_FUNCTION, EntryCount: DWORD, BaseAddress: DWORD64,
    ) -> BOOLEAN;
    pub fn RtlCaptureContext(ContextRecord: PCONTEXT);
    pub fn RtlCaptureStackBackTrace(
        FramesToSkip: DWORD, FramesToCapture: DWORD, BackTrace: *mut PVOID, BackTraceHash: PDWORD,
    ) -> WORD;
    // #[cfg(any(target_arch = "arm", target_arch = "x86_64"))]
    pub fn RtlCompareMemory(Source1: *const VOID, Source2: *const VOID, Length: SIZE_T) -> SIZE_T;
    // #[cfg(any(target_arch = "arm", target_arch = "x86_64"))]
    pub fn RtlCopyMemory(Destination: PVOID, Source: *const VOID, Length: SIZE_T);
    #[cfg(any(target_arch = "arm", target_arch = "x86_64"))]
    pub fn RtlDeleteFunctionTable(FunctionTable: PRUNTIME_FUNCTION) -> BOOLEAN;
    // pub fn RtlFillMemory();
    #[cfg(target_arch = "arm")]
    pub fn RtlInstallFunctionTableCallback(
        TableIdentifier: DWORD, BaseAddress: DWORD, Length: DWORD,
        Callback: PGET_RUNTIME_FUNCTION_CALLBACK, Context: PVOID, OutOfProcessCallbackDll: PCWSTR,
    ) -> BOOLEAN;
    #[cfg(target_arch = "x86_64")]
    pub fn RtlInstallFunctionTableCallback(
        TableIdentifier: DWORD64, BaseAddress: DWORD64, Length: DWORD,
        Callback: PGET_RUNTIME_FUNCTION_CALLBACK, Context: PVOID, OutOfProcessCallbackDll: PCWSTR,
    ) -> BOOLEAN;
    #[cfg(target_arch = "arm")]
    pub fn RtlLookupFunctionEntry(
        ControlPc: ULONG_PTR, ImageBase: PDWORD, HistoryTable: PUNWIND_HISTORY_TABLE,
    ) -> PRUNTIME_FUNCTION;
    #[cfg(target_arch = "x86_64")]
    pub fn RtlLookupFunctionEntry(
        ControlPc: DWORD64, ImageBase: PDWORD64, HistoryTable: PUNWIND_HISTORY_TABLE,
    ) -> PRUNTIME_FUNCTION;
    // pub fn RtlMoveMemory();
    // #[cfg(any(target_arch = "arm", target_arch = "x86_64"))]
    pub fn RtlPcToFileHeader(PcValue: PVOID, BaseOfImage: *mut PVOID) -> PVOID;
    // #[cfg(any(target_arch = "arm", target_arch = "x86_64"))]
    // pub fn RtlRaiseException();
    #[cfg(any(target_arch = "arm", target_arch = "x86_64"))]
    pub fn RtlRestoreContext(ContextRecord: PCONTEXT, ExceptionRecord: *mut EXCEPTION_RECORD);
    pub fn RtlUnwind(
        TargetFrame: PVOID, TargetIp: PVOID, ExceptionRecord: PEXCEPTION_RECORD, ReturnValue: PVOID,
    );
    #[cfg(any(target_arch = "arm", target_arch = "x86_64"))]
    pub fn RtlUnwindEx(
        TargetFrame: PVOID, TargetIp: PVOID, ExceptionRecord: PEXCEPTION_RECORD, ReturnValue: PVOID,
        ContextRecord: PCONTEXT, HistoryTable: PUNWIND_HISTORY_TABLE,
    );
    #[cfg(target_arch = "arm")]
    pub fn RtlVirtualUnwind(
        HandlerType: DWORD, ImageBase: DWORD, ControlPc: DWORD, FunctionEntry: PRUNTIME_FUNCTION,
        ContextRecord: PCONTEXT, HandlerData: *mut PVOID, EstablisherFrame: PDWORD,
        ContextPointers: PKNONVOLATILE_CONTEXT_POINTERS,
    ) -> PEXCEPTION_ROUTINE;
    #[cfg(target_arch = "x86_64")]
    pub fn RtlVirtualUnwind(
        HandlerType: DWORD, ImageBase: DWORD64, ControlPc: DWORD64,
        FunctionEntry: PRUNTIME_FUNCTION, ContextRecord: PCONTEXT, HandlerData: *mut PVOID,
        EstablisherFrame: PDWORD64, ContextPointers: PKNONVOLATILE_CONTEXT_POINTERS,
    ) -> PEXCEPTION_ROUTINE;
    // pub fn RtlZeroMemory();
    pub fn SearchPathA(
        lpPath: LPCSTR, lpFileName: LPCSTR, lpExtension: LPCSTR, nBufferLength: DWORD,
        lpBuffer: LPSTR, lpFilePart: *mut LPSTR,
    ) -> DWORD;
    pub fn SearchPathW(
        lpPath: LPCWSTR, lpFileName: LPCWSTR, lpExtension: LPCWSTR, nBufferLength: DWORD,
        lpBuffer: LPWSTR, lpFilePart: *mut LPWSTR,
    ) -> DWORD;
    pub fn SetCalendarInfoA(
        Locale: LCID, Calendar: CALID, CalType: CALTYPE, lpCalData: LPCSTR,
    ) -> BOOL;
    pub fn SetCalendarInfoW(
        Locale: LCID, Calendar: CALID, CalType: CALTYPE, lpCalData: LPCWSTR,
    ) -> BOOL;
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
    pub fn SetInformationJobObject(
        hJob: HANDLE, JobObjectInformationClass: JOBOBJECTINFOCLASS,
        lpJobObjectInformation: LPVOID, cbJobObjectInformationLength: DWORD,
    ) -> BOOL;
    // pub fn SetLocalPrimaryComputerNameA();
    // pub fn SetLocalPrimaryComputerNameW();
    pub fn SetLocalTime(lpSystemTime: *const SYSTEMTIME) -> BOOL;
    pub fn SetLocaleInfoA(Locale: LCID, LCType: LCTYPE, lpLCData: LPCSTR) -> BOOL;
    pub fn SetLocaleInfoW(Locale: LCID, LCType: LCTYPE, lpLCData: LPCWSTR) -> BOOL;
    pub fn SetNamedPipeAttribute(
        Pipe: HANDLE, AttributeType: PIPE_ATTRIBUTE_TYPE, AttributeName: PSTR,
        AttributeValue: PVOID, AttributeValueLength: SIZE_T,
    ) -> BOOL;
    pub fn SetNamedPipeHandleState(
        hNamedPipe: HANDLE, lpMode: LPDWORD, lpMaxCollectionCount: LPDWORD,
        lpCollectDataTimeout: LPDWORD,
    ) -> BOOL;
    pub fn SetProcessPreferredUILanguages(
        dwFlags: DWORD, pwszLanguagesBuffer: PCZZWSTR, pulNumLanguages: PULONG,
    ) -> BOOL;
    pub fn SetProcessWorkingSetSizeEx(
        hProcess: HANDLE, dwMinimumWorkingSetSize: SIZE_T, dwMaximumWorkingSetSize: SIZE_T,
        Flags: DWORD,
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
    pub fn SetThreadLocale(Locale: LCID) -> BOOL;
    pub fn SetThreadPreferredUILanguages(
        dwFlags: DWORD, pwszLanguagesBuffer: PCZZWSTR, pulNumLanguages: PULONG,
    ) -> BOOL;
    pub fn SetThreadUILanguage(LangId: LANGID) -> LANGID;
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
    pub fn SetUserGeoID(GeoId: GEOID) -> BOOL;
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
    pub fn TerminateJobObject(hJob: HANDLE, uExitCode: UINT) -> BOOL;
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
    pub fn UnmapViewOfFile(lpBaseAddress: LPCVOID) -> BOOL;
    pub fn UnregisterBadMemoryNotification(RegistrationHandle: PVOID) -> BOOL;
    pub fn UnregisterWaitEx(WaitHandle: HANDLE, CompletionEvent: HANDLE) -> BOOL;
    // pub fn UnregisterWaitUntilOOBECompleted();
    pub fn VerLanguageNameA(wLang: DWORD, szLang: LPSTR, cchLang: DWORD) -> DWORD;
    pub fn VerLanguageNameW(wLang: DWORD, szLang: LPWSTR, cchLang: DWORD) -> DWORD;
    pub fn VerSetConditionMask(
        ConditionMask: ULONGLONG, TypeMask: DWORD, Condition: BYTE,
    ) -> ULONGLONG;
    pub fn VerifyScripts(
        dwFlags: DWORD, lpLocaleScripts: LPCWSTR, cchLocaleScripts: c_int, lpTestScripts: LPCWSTR,
        cchTestScripts: c_int,
    ) -> BOOL;
    pub fn VirtualAlloc(
        lpAddress: LPVOID, dwSize: SIZE_T, flAllocationType: DWORD, flProtect: DWORD,
    ) -> LPVOID;
    pub fn VirtualAllocEx(
        hProcess: HANDLE, lpAddress: LPVOID, dwSize: SIZE_T, flAllocationType: DWORD,
        flProtect: DWORD,
    ) -> LPVOID;
    pub fn VirtualAllocExNuma(
        hProcess: HANDLE, lpAddress: LPVOID, dwSize: SIZE_T, flAllocationType: DWORD,
        flProtect: DWORD, nndPreferred: DWORD,
    ) -> LPVOID;
    pub fn VirtualFree(lpAddress: LPVOID, dwSize: SIZE_T, dwFreeType: DWORD) -> BOOL;
    pub fn VirtualFreeEx(
        hProcess: HANDLE, lpAddress: LPVOID, dwSize: SIZE_T, dwFreeType: DWORD,
    ) -> BOOL;
    pub fn VirtualLock(lpAddress: LPVOID, dwSize: SIZE_T) -> BOOL;
    pub fn VirtualProtect(
        lpAddress: LPVOID, dwSize: SIZE_T, flNewProtect: DWORD, lpflOldProtect: PDWORD,
    ) -> BOOL;
    pub fn VirtualProtectEx(
        hProcess: HANDLE, lpAddress: LPVOID, dwSize: SIZE_T, flNewProtect: DWORD,
        lpflOldProtect: PDWORD,
    ) -> BOOL;
    pub fn VirtualQuery(
        lpAddress: LPCVOID, lpBuffer: PMEMORY_BASIC_INFORMATION, dwLength: SIZE_T,
    ) -> SIZE_T;
    pub fn VirtualQueryEx(
        hProcess: HANDLE, lpAddress: LPCVOID, lpBuffer: PMEMORY_BASIC_INFORMATION, dwLength: SIZE_T,
    ) -> SIZE_T;
    pub fn VirtualUnlock(lpAddress: LPVOID, dwSize: SIZE_T) -> BOOL;
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
    pub fn WriteProcessMemory(
        hProcess: HANDLE, lpBaseAddress: LPVOID, lpBuffer: LPCVOID, nSize: SIZE_T,
        lpNumberOfBytesWritten: *mut SIZE_T,
    ) -> BOOL;
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

// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to kernel32.
#![cfg(windows)]
extern crate winapi;
use winapi::*;
extern "system" {
    pub fn AcquireSRWLockExclusive(SRWLock: PSRWLOCK);
    pub fn AcquireSRWLockShared(SRWLock: PSRWLOCK);
    pub fn ActivateActCtx(hActCtx: HANDLE, lpCookie: *mut ULONG_PTR) -> BOOL;
    pub fn AddAtomA(lpString: LPCSTR) -> ATOM;
    pub fn AddAtomW(lpString: LPCWSTR) -> ATOM;
    pub fn AddConsoleAliasA(Source: LPSTR, Target: LPSTR, ExeName: LPSTR) -> BOOL;
    pub fn AddConsoleAliasW(Source: LPWSTR, Target: LPWSTR, ExeName: LPWSTR) -> BOOL;
    pub fn AddDllDirectory(NewDirectory: PCWSTR) -> DLL_DIRECTORY_COOKIE;
    pub fn AddIntegrityLabelToBoundaryDescriptor(
        BoundaryDescriptor: *mut HANDLE, IntegrityLabel: PSID,
    ) -> BOOL;
    // pub fn AddLocalAlternateComputerNameA();
    // pub fn AddLocalAlternateComputerNameW();
    pub fn AddRefActCtx(hActCtx: HANDLE);
    pub fn AddResourceAttributeAce(
        pAcl: PACL, dwAceRevision: DWORD, AceFlags: DWORD, AccessMask: DWORD, pSid: PSID,
        pAttributeInfo: PCLAIM_SECURITY_ATTRIBUTES_INFORMATION, pReturnLength: PDWORD,
    ) -> BOOL;
    pub fn AddSIDToBoundaryDescriptor(BoundaryDescriptor: *mut HANDLE, RequiredSid: PSID) -> BOOL;
    pub fn AddScopedPolicyIDAce(
        pAcl: PACL, dwAceRevision: DWORD, AceFlags: DWORD, AccessMask: DWORD, pSid: PSID,
    ) -> BOOL;
    pub fn AddSecureMemoryCacheCallback(pfnCallBack: PSECURE_MEMORY_CACHE_CALLBACK) -> BOOL;
    pub fn AddVectoredContinueHandler(First: ULONG, Handler: PVECTORED_EXCEPTION_HANDLER) -> PVOID;
    pub fn AddVectoredExceptionHandler(
        First: ULONG, Handler: PVECTORED_EXCEPTION_HANDLER,
    ) -> PVOID;
    pub fn AllocConsole() -> BOOL;
    // pub fn AllocateUserPhysicalPages();
    // pub fn AllocateUserPhysicalPagesNuma();
    // pub fn AppXGetOSMaxVersionTested();
    // pub fn ApplicationRecoveryFinished();
    // pub fn ApplicationRecoveryInProgress();
    // pub fn AreFileApisANSI();
    pub fn AssignProcessToJobObject(hJob: HANDLE, hProcess: HANDLE) -> BOOL;
    pub fn AttachConsole(dwProcessId: DWORD) -> BOOL;
    // pub fn BackupRead();
    // pub fn BackupSeek();
    // pub fn BackupWrite();
    // pub fn BaseSetLastNTError();
    // pub fn Beep();
    // pub fn BeginUpdateResourceA();
    // pub fn BeginUpdateResourceW();
    // pub fn BindIoCompletionCallback();
    // pub fn BuildCommDCBA();
    // pub fn BuildCommDCBAndTimeoutsA();
    // pub fn BuildCommDCBAndTimeoutsW();
    // pub fn BuildCommDCBW();
    // pub fn CallNamedPipeA();
    // pub fn CallNamedPipeW();
    // pub fn CallbackMayRunLong();
    // pub fn CalloutOnFiberStack();
    // pub fn CancelDeviceWakeupRequest();
    pub fn CancelIo(hFile: HANDLE) -> BOOL;
    pub fn CancelIoEx(hFile: HANDLE, lpOverlapped: LPOVERLAPPED) -> BOOL;
    // pub fn CancelSynchronousIo();
    // pub fn CancelThreadpoolIo();
    // pub fn CancelTimerQueueTimer();
    // pub fn CancelWaitableTimer();
    // pub fn CeipIsOptedIn();
    // pub fn ChangeTimerQueueTimer();
    // pub fn CheckElevation();
    // pub fn CheckElevationEnabled();
    // pub fn CheckNameLegalDOS8Dot3A();
    // pub fn CheckNameLegalDOS8Dot3W();
    // pub fn CheckRemoteDebuggerPresent();
    // pub fn CheckTokenCapability();
    // pub fn CheckTokenMembershipEx();
    // pub fn ClearCommBreak();
    // pub fn ClearCommError();
    pub fn CloseHandle(hObject: HANDLE) -> BOOL;
    // pub fn ClosePackageInfo();
    // pub fn ClosePrivateNamespace();
    // pub fn CloseState();
    // pub fn CloseThreadpool();
    // pub fn CloseThreadpoolCleanupGroup();
    // pub fn CloseThreadpoolCleanupGroupMembers();
    // pub fn CloseThreadpoolIo();
    // pub fn CloseThreadpoolTimer();
    // pub fn CloseThreadpoolWait();
    // pub fn CloseThreadpoolWork();
    // pub fn CommConfigDialogA();
    // pub fn CommConfigDialogW();
    pub fn CompareFileTime(lpFileTime1: *const FILETIME, lpFileTime2: *const FILETIME) -> LONG;
    // pub fn CompareStringA();
    // pub fn CompareStringEx();
    // pub fn CompareStringOrdinal();
    // pub fn CompareStringW();
    pub fn ConnectNamedPipe(hNamedPipe: HANDLE, lpOverlapped: LPOVERLAPPED) -> BOOL;
    // pub fn ContinueDebugEvent();
    // pub fn ConvertDefaultLocale();
    // pub fn ConvertFiberToThread();
    // pub fn ConvertThreadToFiber();
    // pub fn ConvertThreadToFiberEx();
    // pub fn CopyContext();
    // pub fn CopyFile2();
    // pub fn CopyFileA();
    // pub fn CopyFileExA();
    // pub fn CopyFileExW();
    // pub fn CopyFileTransactedA();
    // pub fn CopyFileTransactedW();
    // pub fn CopyFileW();
    // pub fn CreateActCtxA();
    // pub fn CreateActCtxW();
    // pub fn CreateBoundaryDescriptorA();
    // pub fn CreateBoundaryDescriptorW();
    pub fn CreateConsoleScreenBuffer(
        dwDesiredAccess: DWORD, dwShareMode: DWORD,
        lpSecurityAttributes: *const SECURITY_ATTRIBUTES, dwFlags: DWORD,
        lpScreenBufferData: LPVOID,
    ) -> HANDLE;
    pub fn CreateDirectoryA(
        lpPathName: LPCSTR, lpSecurityAttributes: LPSECURITY_ATTRIBUTES,
    ) -> BOOL;
    pub fn CreateDirectoryExA(
        lpTemplateDirectory: LPCSTR, lpNewDirectory: LPCSTR,
        lpSecurityAttributes: LPSECURITY_ATTRIBUTES,
    ) -> BOOL;
    pub fn CreateDirectoryExW(
        lpTemplateDirectory: LPCWSTR, lpNewDirectory: LPCWSTR,
        lpSecurityAttributes: LPSECURITY_ATTRIBUTES,
    ) -> BOOL;
    // pub fn CreateDirectoryTransactedA();
    // pub fn CreateDirectoryTransactedW();
    pub fn CreateDirectoryW(
        lpPathName: LPCWSTR, lpSecurityAttributes: LPSECURITY_ATTRIBUTES,
    ) -> BOOL;
    pub fn CreateEventA(
        lpEventAttributes: LPSECURITY_ATTRIBUTES, bManualReset: BOOL, bInitialState: BOOL,
        lpName: LPCSTR,
    ) -> HANDLE;
    pub fn CreateEventW(
        lpEventAttributes: LPSECURITY_ATTRIBUTES, bManualReset: BOOL, bInitialState: BOOL,
        lpName: LPCWSTR,
    ) -> HANDLE;
    pub fn CreateEventExA(
        lpEventAttributes: LPSECURITY_ATTRIBUTES, lpName: LPCSTR, dwFlags: DWORD,
        dwDesiredAccess: DWORD,
    ) -> HANDLE;
    pub fn CreateEventExW(
        lpEventAttributes: LPSECURITY_ATTRIBUTES, lpName: LPWSTR, dwFlags: DWORD,
        dwDesiredAccess: DWORD,
    ) -> HANDLE;
    // pub fn CreateFiber();
    // pub fn CreateFiberEx();
    pub fn CreateFile2(
        lpFileName: LPCWSTR, dwDesiredAccess: DWORD, dwShareMode: DWORD,
        dwCreationDisposition: DWORD, pCreateExParams: LPCREATEFILE2_EXTENDED_PARAMETERS,
    ) -> HANDLE;
    pub fn CreateFileA(
        lpFileName: LPCSTR, dwDesiredAccess: DWORD, dwShareMode: DWORD,
        lpSecurityAttributes: LPSECURITY_ATTRIBUTES, dwCreationDisposition: DWORD,
        dwFlagsAndAttributes: DWORD, hTemplateFile: HANDLE,
    ) -> HANDLE;
    pub fn CreateFileMappingA(
        hFile: HANDLE, lpAttributes: LPSECURITY_ATTRIBUTES, flProtect: DWORD,
        dwMaximumSizeHigh: DWORD, dwMaximumSizeLow: DWORD, lpName: LPCSTR,
    ) -> HANDLE;
    // pub fn CreateFileMappingFromApp();
    // pub fn CreateFileMappingNumaA();
    // pub fn CreateFileMappingNumaW();
    pub fn CreateFileMappingW(
        hFile: HANDLE, lpAttributes: LPSECURITY_ATTRIBUTES, flProtect: DWORD,
        dwMaximumSizeHigh: DWORD, dwMaximumSizeLow: DWORD, lpName: LPCWSTR,
    ) -> HANDLE;
    // pub fn CreateFileTransactedA();
    // pub fn CreateFileTransactedW();
    pub fn CreateFileW(
        lpFileName: LPCWSTR, dwDesiredAccess: DWORD, dwShareMode: DWORD,
        lpSecurityAttributes: LPSECURITY_ATTRIBUTES, dwCreationDisposition: DWORD,
        dwFlagsAndAttributes: DWORD, hTemplateFile: HANDLE,
    ) -> HANDLE;
    // pub fn CreateHardLinkA();
    // pub fn CreateHardLinkTransactedA();
    // pub fn CreateHardLinkTransactedW();
    // pub fn CreateHardLinkW();
    pub fn CreateIoCompletionPort(
        FileHandle: HANDLE, ExistingCompletionPort: HANDLE, CompletionKey: ULONG_PTR,
        NumberOfConcurrentThreads: DWORD,
    ) -> HANDLE;
    // pub fn CreateJobObjectA();
    pub fn CreateJobObjectW(lpJobAttributes: LPSECURITY_ATTRIBUTES, lpName: LPCWSTR) -> HANDLE;
    // pub fn CreateJobSet();
    // pub fn CreateMailslotA();
    // pub fn CreateMailslotW();
    // pub fn CreateMemoryResourceNotification();
    // pub fn CreateMutexA();
    // pub fn CreateMutexExA();
    // pub fn CreateMutexExW();
    // pub fn CreateMutexW();
    pub fn CreateNamedPipeA(
        lpName: LPCSTR, dwOpenMode: DWORD, dwPipeMode: DWORD, nMaxInstances: DWORD,
        nOutBufferSize: DWORD, nInBufferSize: DWORD, nDefaultTimeOut: DWORD,
        lpSecurityAttributes: LPSECURITY_ATTRIBUTES,
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
    // pub fn CreatePrivateNamespaceA();
    // pub fn CreatePrivateNamespaceW();
    // pub fn CreateProcessA();
    // pub fn CreateProcessW();
    // pub fn CreateRemoteThread();
    // pub fn CreateRemoteThreadEx();
    pub fn CreateSemaphoreA(
        lpSemaphoreAttributes: LPSECURITY_ATTRIBUTES, lInitialCount: LONG, lMaximumCount: LONG,
        lpName: LPCSTR,
    ) -> HANDLE;
    pub fn CreateSemaphoreExA(
        lpSemaphoreAttributes: LPSECURITY_ATTRIBUTES, lInitialCount: LONG, lMaximumCount: LONG,
        lpName: LPCSTR, dwFlags: DWORD, dwDesiredAccess: DWORD,
    ) -> HANDLE;
    pub fn CreateSemaphoreExW(
        lpSemaphoreAttributes: LPSECURITY_ATTRIBUTES, lInitialCount: LONG, lMaximumCount: LONG,
        lpName: LPCWSTR, dwFlags: DWORD, dwDesiredAccess: DWORD,
    ) -> HANDLE;
    pub fn CreateSemaphoreW(
        lpSemaphoreAttributes: LPSECURITY_ATTRIBUTES, lInitialCount: LONG, lMaximumCount: LONG,
        lpName: LPCWSTR,
    ) -> HANDLE;
    // pub fn CreateSymbolicLinkA();
    // pub fn CreateSymbolicLinkTransactedA();
    // pub fn CreateSymbolicLinkTransactedW();
    // pub fn CreateSymbolicLinkW();
    // pub fn CreateTapePartition();
    // pub fn CreateThread();
    // pub fn CreateThreadpool();
    // pub fn CreateThreadpoolCleanupGroup();
    // pub fn CreateThreadpoolIo();
    // pub fn CreateThreadpoolTimer();
    // pub fn CreateThreadpoolWait();
    // pub fn CreateThreadpoolWork();
    // pub fn CreateTimerQueue();
    // pub fn CreateTimerQueueTimer();
    // pub fn CreateToolhelp32Snapshot();
    // #[cfg(target_arch = "x86_64")]
    // pub fn CreateUmsCompletionList();
    // #[cfg(target_arch = "x86_64")]
    // pub fn CreateUmsThreadContext();
    // pub fn CreateWaitableTimerA();
    // pub fn CreateWaitableTimerExA();
    // pub fn CreateWaitableTimerExW();
    // pub fn CreateWaitableTimerW();
    // pub fn CtrlRoutine();
    // pub fn DeactivateActCtx();
    // pub fn DebugActiveProcess();
    // pub fn DebugActiveProcessStop();
    // pub fn DebugBreak();
    // pub fn DebugBreakProcess();
    // pub fn DebugSetProcessKillOnExit();
    // pub fn DecodePointer();
    // pub fn DecodeSystemPointer();
    // pub fn DefineDosDeviceA();
    pub fn DefineDosDeviceW(dwFlags: DWORD, lpDeviceName: LPCWSTR, lpTargetPath: LPCWSTR) -> BOOL;
    // pub fn DelayLoadFailureHook();
    // pub fn DeleteAtom();
    // pub fn DeleteBoundaryDescriptor();
    pub fn DeleteCriticalSection(lpCriticalSection: LPCRITICAL_SECTION);
    // pub fn DeleteFiber();
    pub fn DeleteFileA(lpFileName: LPCSTR) -> BOOL;
    // pub fn DeleteFileTransactedA();
    // pub fn DeleteFileTransactedW();
    pub fn DeleteFileW(lpFileName: LPCWSTR) -> BOOL;
    // pub fn DeleteProcThreadAttributeList();
    // pub fn DeleteSynchronizationBarrier();
    // pub fn DeleteTimerQueue();
    // pub fn DeleteTimerQueueEx();
    // pub fn DeleteTimerQueueTimer();
    // #[cfg(target_arch = "x86_64")]
    // pub fn DeleteUmsCompletionList();
    // #[cfg(target_arch = "x86_64")]
    // pub fn DeleteUmsThreadContext();
    // pub fn DeleteVolumeMountPointA();
    pub fn DeleteVolumeMountPointW(lpszVolumeMountPoint: LPCWSTR) -> BOOL;
    // #[cfg(target_arch = "x86_64")]
    // pub fn DequeueUmsCompletionListItems();
    pub fn DeviceIoControl(
        hDevice: HANDLE, dwIoControlCode: DWORD, lpInBuffer: LPVOID, nInBufferSize: DWORD,
        lpOutBuffer: LPVOID, nOutBufferSize: DWORD, lpBytesReturned: LPDWORD,
        lpOverlapped: LPOVERLAPPED,
    ) -> BOOL;
    // pub fn DisableThreadLibraryCalls();
    // pub fn DisableThreadProfiling();
    // pub fn DisassociateCurrentThreadFromCallback();
    // pub fn DisconnectNamedPipe();
    // pub fn DnsHostnameToComputerNameA();
    // pub fn DnsHostnameToComputerNameExW();
    // pub fn DnsHostnameToComputerNameW();
    pub fn DosDateTimeToFileTime(wFatDate: WORD, wFatTime: WORD, lpFileTime: LPFILETIME) -> BOOL;
    // pub fn DosPathToSessionPathW();
    // pub fn DuplicateHandle();
    // pub fn EnableThreadProfiling();
    // pub fn EncodePointer();
    // pub fn EncodeSystemPointer();
    // pub fn EndUpdateResourceA();
    // pub fn EndUpdateResourceW();
    pub fn EnterCriticalSection(lpCriticalSection: LPCRITICAL_SECTION);
    // pub fn EnterSynchronizationBarrier();
    // #[cfg(target_arch = "x86_64")]
    // pub fn EnterUmsSchedulingMode();
    // pub fn EnumCalendarInfoA();
    // pub fn EnumCalendarInfoExA();
    // pub fn EnumCalendarInfoExEx();
    // pub fn EnumCalendarInfoExW();
    // pub fn EnumCalendarInfoW();
    // pub fn EnumDateFormatsA();
    // pub fn EnumDateFormatsExA();
    // pub fn EnumDateFormatsExEx();
    // pub fn EnumDateFormatsExW();
    // pub fn EnumDateFormatsW();
    // pub fn EnumLanguageGroupLocalesA();
    // pub fn EnumLanguageGroupLocalesW();
    // pub fn EnumResourceLanguagesA();
    // pub fn EnumResourceLanguagesExA();
    // pub fn EnumResourceLanguagesExW();
    // pub fn EnumResourceLanguagesW();
    // pub fn EnumResourceNamesA();
    // pub fn EnumResourceNamesExA();
    // pub fn EnumResourceNamesExW();
    // pub fn EnumResourceNamesW();
    // pub fn EnumResourceTypesA();
    // pub fn EnumResourceTypesExA();
    // pub fn EnumResourceTypesExW();
    // pub fn EnumResourceTypesW();
    // pub fn EnumSystemCodePagesA();
    // pub fn EnumSystemCodePagesW();
    // pub fn EnumSystemFirmwareTables();
    // pub fn EnumSystemGeoID();
    // pub fn EnumSystemLanguageGroupsA();
    // pub fn EnumSystemLanguageGroupsW();
    // pub fn EnumSystemLocalesA();
    // pub fn EnumSystemLocalesEx();
    // pub fn EnumSystemLocalesW();
    // pub fn EnumTimeFormatsA();
    // pub fn EnumTimeFormatsEx();
    // pub fn EnumTimeFormatsW();
    // pub fn EnumUILanguagesA();
    // pub fn EnumUILanguagesW();
    // pub fn EnumerateLocalComputerNamesA();
    // pub fn EnumerateLocalComputerNamesW();
    // pub fn EraseTape();
    // pub fn EscapeCommFunction();
    // #[cfg(target_arch = "x86_64")]
    // pub fn ExecuteUmsThread();
    // pub fn ExitProcess();
    // pub fn ExitThread();
    // pub fn ExpandEnvironmentStringsA();
    // pub fn ExpandEnvironmentStringsW();
    // pub fn FatalAppExitA();
    // pub fn FatalAppExitW();
    // pub fn FatalExit();
    pub fn FileTimeToDosDateTime(
        lpFileTime: *const FILETIME, lpFatDate: LPWORD, lpFatTime: LPWORD,
    ) -> BOOL;
    pub fn FileTimeToLocalFileTime(
        lpFileTime: *const FILETIME, lpLocalFileTime: LPFILETIME,
    ) -> BOOL;
    pub fn FileTimeToSystemTime(
        lpFileTime: *const FILETIME, lpSystemTime: LPSYSTEMTIME,
    ) -> BOOL;
    pub fn FillConsoleOutputAttribute(
        hConsoleOutput: HANDLE, wAttribute: WORD, nLength: DWORD, dwWriteCoord: COORD,
        lpNumberOfAttrsWritten: LPDWORD,
    ) -> BOOL;
    pub fn FillConsoleOutputCharacterA(
        hConsoleOutput: HANDLE, cCharacter: CHAR, nLength: DWORD, dwWriteCoord: COORD,
        lpNumberOfCharsWritten: LPDWORD,
    ) -> BOOL;
    pub fn FillConsoleOutputCharacterW(
        hConsoleOutput: HANDLE, cCharacter: WCHAR, nLength: DWORD, dwWriteCoord: COORD,
        lpNumberOfCharsWritten: LPDWORD,
    ) -> BOOL;
    // pub fn FindActCtxSectionGuid();
    // pub fn FindActCtxSectionStringA();
    // pub fn FindActCtxSectionStringW();
    // pub fn FindAtomA();
    // pub fn FindAtomW();
    pub fn FindClose(hFindFile: HANDLE) -> BOOL;
    pub fn FindCloseChangeNotification(hChangeHandle: HANDLE) -> BOOL;
    pub fn FindFirstChangeNotificationA(
        lpPathName: LPCSTR, bWatchSubtree: BOOL, dwNotifyFilter: DWORD,
    ) -> HANDLE;
    pub fn FindFirstChangeNotificationW(
        lpPathName: LPCWSTR, bWatchSubtree: BOOL, dwNotifyFilter: DWORD,
    ) -> HANDLE;
    pub fn FindFirstFileA(lpFileName: LPCSTR, lpFindFileData: LPWIN32_FIND_DATAA) -> HANDLE;
    pub fn FindFirstFileExA(
        lpFileName: LPCSTR, fInfoLevelId: FINDEX_INFO_LEVELS, lpFindFileData: LPVOID,
        fSearchOp: FINDEX_SEARCH_OPS, lpSearchFilter: LPVOID, dwAdditionalFlags: DWORD,
    ) -> HANDLE;
    pub fn FindFirstFileExW(
        lpFileName: LPCWSTR, fInfoLevelId: FINDEX_INFO_LEVELS, lpFindFileData: LPVOID,
        fSearchOp: FINDEX_SEARCH_OPS, lpSearchFilter: LPVOID, dwAdditionalFlags: DWORD,
    ) -> HANDLE;
    // pub fn FindFirstFileNameTransactedW();
    // pub fn FindFirstFileNameW();
    // pub fn FindFirstFileTransactedA();
    // pub fn FindFirstFileTransactedW();
    pub fn FindFirstFileW(lpFileName: LPCWSTR, lpFindFileData: LPWIN32_FIND_DATAW) -> HANDLE;
    // pub fn FindFirstStreamTransactedW();
    // pub fn FindFirstStreamW();
    // pub fn FindFirstVolumeA();
    // pub fn FindFirstVolumeMountPointA();
    // pub fn FindFirstVolumeMountPointW();
    pub fn FindFirstVolumeW(lpszVolumeName: LPWSTR, cchBufferLength: DWORD) -> HANDLE;
    // pub fn FindNLSString();
    // pub fn FindNLSStringEx();
    pub fn FindNextChangeNotification(hChangeHandle: HANDLE) -> BOOL;
    pub fn FindNextFileA(hFindFile: HANDLE, lpFindFileData: LPWIN32_FIND_DATAA) -> BOOL;
    // pub fn FindNextFileNameW();
    pub fn FindNextFileW(hFindFile: HANDLE, lpFindFileData: LPWIN32_FIND_DATAW) -> BOOL;
    // pub fn FindNextStreamW();
    // pub fn FindNextVolumeA();
    // pub fn FindNextVolumeMountPointA();
    // pub fn FindNextVolumeMountPointW();
    pub fn FindNextVolumeW(
        hFindVolume: HANDLE, lpszVolumeName: LPWSTR, cchBufferLength: DWORD,
    ) -> BOOL;
    // pub fn FindPackagesByPackageFamily();
    // pub fn FindResourceA();
    // pub fn FindResourceExA();
    // pub fn FindResourceExW();
    // pub fn FindResourceW();
    // pub fn FindStringOrdinal();
    pub fn FindVolumeClose(hFindVolume: HANDLE) -> BOOL;
    // pub fn FindVolumeMountPointClose();
    // pub fn FlsAlloc();
    // pub fn FlsFree();
    // pub fn FlsGetValue();
    // pub fn FlsSetValue();
    pub fn FlushConsoleInputBuffer(hConsoleInput: HANDLE) -> BOOL;
    pub fn FlushFileBuffers(hFile: HANDLE) -> BOOL;
    // pub fn FlushInstructionCache();
    // pub fn FlushProcessWriteBuffers();
     pub fn FlushViewOfFile(lpBaseAddress: LPCVOID, dwNumberOfBytesToFlush: SIZE_T) -> BOOL;
    // pub fn FoldStringA();
    // pub fn FoldStringW();
    // pub fn FormatApplicationUserModelId();
    pub fn FormatMessageA(
        dwFlags: DWORD, lpSource: LPCVOID, dwMessageId: DWORD, dwLanguageId: DWORD,
        lpBuffer: LPSTR, nSize: DWORD, Arguments: *mut va_list,
    ) -> DWORD;
    pub fn FormatMessageW(
        dwFlags: DWORD, lpSource: LPCVOID, dwMessageId: DWORD, dwLanguageId: DWORD,
        lpBuffer: LPWSTR, nSize: DWORD, Arguments: *mut va_list,
    ) -> DWORD;
    pub fn FreeConsole() -> BOOL;
    // pub fn FreeEnvironmentStringsA();
    // pub fn FreeEnvironmentStringsW();
    pub fn FreeLibrary(hLibModule: HMODULE) -> BOOL;
    // pub fn FreeLibraryAndExitThread();
    // pub fn FreeLibraryWhenCallbackReturns();
    // pub fn FreeResource();
    // pub fn FreeUserPhysicalPages();
    pub fn GenerateConsoleCtrlEvent(dwCtrlEvent: DWORD, dwProcessGroupId: DWORD) -> BOOL;
    // pub fn GetACP();
    // pub fn GetActiveProcessorCount();
    // pub fn GetActiveProcessorGroupCount();
    // pub fn GetAppContainerAce();
    // pub fn GetAppContainerNamedObjectPath();
    // pub fn GetApplicationRecoveryCallback();
    // pub fn GetApplicationRestartSettings();
    // pub fn GetApplicationUserModelId();
    // pub fn GetAtomNameA();
    // pub fn GetAtomNameW();
    // pub fn GetBinaryType();
    // pub fn GetBinaryTypeA();
    // pub fn GetBinaryTypeW();
    // pub fn GetCPInfo();
    // pub fn GetCPInfoExA();
    // pub fn GetCPInfoExW();
    // pub fn GetCachedSigningLevel();
    // pub fn GetCalendarInfoA();
    // pub fn GetCalendarInfoEx();
    // pub fn GetCalendarInfoW();
    // pub fn GetCommConfig();
    // pub fn GetCommMask();
    // pub fn GetCommModemStatus();
    // pub fn GetCommProperties();
    // pub fn GetCommState();
    // pub fn GetCommTimeouts();
    pub fn GetCommandLineA() -> LPSTR;
    pub fn GetCommandLineW() -> LPWSTR;
    pub fn GetCompressedFileSizeA(lpFileName: LPCSTR, lpFileSizeHigh: LPDWORD) -> DWORD;
    // pub fn GetCompressedFileSizeTransactedA();
    // pub fn GetCompressedFileSizeTransactedW();
    pub fn GetCompressedFileSizeW(lpFileName: LPCWSTR, lpFileSizeHigh: LPDWORD) -> DWORD;
    // pub fn GetComputerNameA();
    // pub fn GetComputerNameExA();
    // pub fn GetComputerNameExW();
    // pub fn GetComputerNameW();
    pub fn GetConsoleAliasA(
        Source: LPSTR, TargetBuffer: LPSTR, TargetBufferLength: DWORD, ExeName: LPSTR,
    ) -> DWORD;
    pub fn GetConsoleAliasExesA(ExeNameBuffer: LPSTR, ExeNameBufferLength: DWORD) -> DWORD;
    pub fn GetConsoleAliasExesLengthA() -> DWORD;
    pub fn GetConsoleAliasExesLengthW() -> DWORD;
    pub fn GetConsoleAliasExesW(ExeNameBuffer: LPWSTR, ExeNameBufferLength: DWORD) -> DWORD;
    pub fn GetConsoleAliasW(
        Source: LPWSTR, TargetBuffer: LPWSTR, TargetBufferLength: DWORD, ExeName: LPWSTR,
    ) -> DWORD;
    pub fn GetConsoleAliasesA(
        AliasBuffer: LPSTR, AliasBufferLength: DWORD, ExeName: LPSTR,
    ) -> DWORD;
    pub fn GetConsoleAliasesLengthA(ExeName: LPSTR) -> DWORD;
    pub fn GetConsoleAliasesLengthW(ExeName: LPWSTR) -> DWORD;
    pub fn GetConsoleAliasesW(
        AliasBuffer: LPWSTR, AliasBufferLength: DWORD, ExeName: LPWSTR,
    ) -> DWORD;
    pub fn GetConsoleCP() -> UINT;
    pub fn GetConsoleCursorInfo(
        hConsoleOutput: HANDLE, lpConsoleCursorInfo: PCONSOLE_CURSOR_INFO,
    ) -> BOOL;
    pub fn GetConsoleDisplayMode(lpModeFlags: LPDWORD) -> BOOL;
    pub fn GetConsoleFontSize(hConsoleOutput: HANDLE, nFont: DWORD) -> COORD;
    pub fn GetConsoleHistoryInfo(lpConsoleHistoryInfo: PCONSOLE_HISTORY_INFO) -> BOOL;
    pub fn GetConsoleMode(hConsoleHandle: HANDLE, lpMode: LPDWORD) -> BOOL;
    pub fn GetConsoleOriginalTitleA(lpConsoleTitle: LPSTR, nSize: DWORD) -> DWORD;
    pub fn GetConsoleOriginalTitleW(lpConsoleTitle: LPWSTR, nSize: DWORD) -> DWORD;
    pub fn GetConsoleOutputCP() -> UINT;
    pub fn GetConsoleProcessList(lpdwProcessList: LPDWORD, dwProcessCount: DWORD) -> DWORD;
    pub fn GetConsoleScreenBufferInfo(
        hConsoleOutput: HANDLE, lpConsoleScreenBufferInfo: PCONSOLE_SCREEN_BUFFER_INFO,
    ) -> BOOL;
    pub fn GetConsoleScreenBufferInfoEx(
        hConsoleOutput: HANDLE, lpConsoleScreenBufferInfoEx: PCONSOLE_SCREEN_BUFFER_INFOEX,
    ) -> BOOL;
    pub fn GetConsoleSelectionInfo(lpConsoleSelectionInfo: PCONSOLE_SELECTION_INFO) -> BOOL;
    pub fn GetConsoleTitleA(lpConsoleTitle: LPSTR, nSize: DWORD) -> DWORD;
    pub fn GetConsoleTitleW(lpConsoleTitle: LPWSTR, nSize: DWORD) -> DWORD;
    pub fn GetConsoleWindow() -> HWND;
    // pub fn GetCurrencyFormatA();
    // pub fn GetCurrencyFormatEx();
    // pub fn GetCurrencyFormatW();
    // pub fn GetCurrentActCtx();
    // pub fn GetCurrentApplicationUserModelId();
    pub fn GetCurrentConsoleFont(
        hConsoleOutput: HANDLE, bMaximumWindow: BOOL, lpConsoleCurrentFont: PCONSOLE_FONT_INFO,
    ) -> BOOL;
    pub fn GetCurrentConsoleFontEx(
        hConsoleOutput: HANDLE, bMaximumWindow: BOOL, lpConsoleCurrentFontEx: PCONSOLE_FONT_INFOEX,
    ) -> BOOL;
    pub fn GetCurrentDirectoryA(nBufferLength: DWORD, lpBuffer: LPSTR) -> DWORD;
    pub fn GetCurrentDirectoryW(nBufferLength: DWORD, lpBuffer: LPWSTR) -> DWORD;
    // pub fn GetCurrentPackageFamilyName();
    // pub fn GetCurrentPackageFullName();
    // pub fn GetCurrentPackageId();
    // pub fn GetCurrentPackageInfo();
    // pub fn GetCurrentPackagePath();
    pub fn GetCurrentProcess() -> HANDLE;
    pub fn GetCurrentProcessId() -> DWORD;
    // pub fn GetCurrentProcessorNumber();
    // pub fn GetCurrentProcessorNumberEx();
    pub fn GetCurrentThread() -> HANDLE;
    pub fn GetCurrentThreadId() -> DWORD;
    // pub fn GetCurrentThreadStackLimits();
    // #[cfg(target_arch = "x86_64")]
    // pub fn GetCurrentUmsThread();
    // pub fn GetDateFormatA();
    // pub fn GetDateFormatEx();
    // pub fn GetDateFormatW();
    // pub fn GetDefaultCommConfigA();
    // pub fn GetDefaultCommConfigW();
    // pub fn GetDevicePowerState();
    pub fn GetDiskFreeSpaceA(
        lpRootPathName: LPCSTR, lpSectorsPerCluster: LPDWORD, lpBytesPerSector: LPDWORD,
        lpNumberOfFreeClusters: LPDWORD, lpTotalNumberOfClusters: LPDWORD,
    ) -> BOOL;
    pub fn GetDiskFreeSpaceExA(
        lpDirectoryName: LPCSTR, lpFreeBytesAvailableToCaller: PULARGE_INTEGER,
        lpTotalNumberOfBytes: PULARGE_INTEGER, lpTotalNumberOfFreeBytes: PULARGE_INTEGER,
    ) -> BOOL;
    pub fn GetDiskFreeSpaceExW(
        lpDirectoryName: LPCWSTR, lpFreeBytesAvailableToCaller: PULARGE_INTEGER,
        lpTotalNumberOfBytes: PULARGE_INTEGER, lpTotalNumberOfFreeBytes: PULARGE_INTEGER,
    ) -> BOOL;
    pub fn GetDiskFreeSpaceW(
        lpRootPathName: LPCWSTR, lpSectorsPerCluster: LPDWORD, lpBytesPerSector: LPDWORD,
        lpNumberOfFreeClusters: LPDWORD, lpTotalNumberOfClusters: LPDWORD,
    ) -> BOOL;
    // pub fn GetDllDirectoryA();
    // pub fn GetDllDirectoryW();
    pub fn GetDriveTypeA(lpRootPathName: LPCSTR) -> UINT;
    pub fn GetDriveTypeW(lpRootPathName: LPCWSTR) -> UINT;
    // pub fn GetDurationFormat();
    // pub fn GetDurationFormatEx();
    pub fn GetDynamicTimeZoneInformation(
        pTimeZoneInformation: PDYNAMIC_TIME_ZONE_INFORMATION,
    ) -> DWORD;
    // #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    // pub fn GetEnabledXStateFeatures();
    // pub fn GetEnvironmentStrings();
    // pub fn GetEnvironmentStringsA();
    // pub fn GetEnvironmentStringsW();
    // pub fn GetEnvironmentVariableA();
    // pub fn GetEnvironmentVariableW();
    // pub fn GetEraNameCountedString();
    // pub fn GetErrorMode();
    // pub fn GetExitCodeProcess();
    // pub fn GetExitCodeThread();
    pub fn GetFileAttributesA(lpFileName: LPCSTR) -> DWORD;
    pub fn GetFileAttributesExA(
        lpFileName: LPCSTR, fInfoLevelId: GET_FILEEX_INFO_LEVELS, lpFileInformation: LPVOID,
    ) -> BOOL;
    pub fn GetFileAttributesExW(
        lpFileName: LPCWSTR, fInfoLevelId: GET_FILEEX_INFO_LEVELS, lpFileInformation: LPVOID,
    ) -> BOOL;
    // pub fn GetFileAttributesTransactedA();
    // pub fn GetFileAttributesTransactedW();
    pub fn GetFileAttributesW(lpFileName: LPCWSTR) -> DWORD;
    // pub fn GetFileBandwidthReservation();
    pub fn GetFileInformationByHandle(hFile: HANDLE, lpFileInformation: LPBY_HANDLE_FILE_INFORMATION) -> BOOL;
    // pub fn GetFileInformationByHandleEx();
    // pub fn GetFileMUIInfo();
    // pub fn GetFileMUIPath();
    pub fn GetFileSize(hFile: HANDLE, lpFileSizeHigh: LPDWORD) -> DWORD;
    pub fn GetFileSizeEx(hFile: HANDLE, lpFileSize: PLARGE_INTEGER) -> BOOL;
    pub fn GetFileTime(
        hFile: HANDLE, lpCreationTime: LPFILETIME, lpLastAccessTime: LPFILETIME,
        lpLastWriteTime: LPFILETIME,
    ) -> BOOL;
    pub fn GetFileType(hFile: HANDLE) -> DWORD;
    pub fn GetFinalPathNameByHandleA(
        hFile: HANDLE, lpszFilePath: LPSTR, cchFilePath: DWORD, dwFlags: DWORD,
    ) -> DWORD;
    pub fn GetFinalPathNameByHandleW(
        hFile: HANDLE, lpszFilePath: LPWSTR, cchFilePath: DWORD, dwFlags: DWORD,
    ) -> DWORD;
    // pub fn GetFirmwareEnvironmentVariableA();
    // pub fn GetFirmwareEnvironmentVariableExA();
    // pub fn GetFirmwareEnvironmentVariableExW();
    // pub fn GetFirmwareEnvironmentVariableW();
    // pub fn GetFirmwareType();
    pub fn GetFullPathNameA(
        lpFileName: LPCSTR, nBufferLength: DWORD, lpBuffer: LPSTR, lpFilePart: *mut LPSTR,
    ) -> DWORD;
    // pub fn GetFullPathNameTransactedA();
    // pub fn GetFullPathNameTransactedW();
    pub fn GetFullPathNameW(
        lpFileName: LPCWSTR, nBufferLength: DWORD, lpBuffer: LPWSTR, lpFilePart: *mut LPWSTR,
    ) -> DWORD;
    // pub fn GetGeoInfoA();
    // pub fn GetGeoInfoW();
    // pub fn GetHandleInformation();
    // pub fn GetLargePageMinimum();
    pub fn GetLargestConsoleWindowSize(hConsoleOutput: HANDLE) -> COORD;
    pub fn GetLastError() -> DWORD;
    pub fn GetLocalTime(lpSystemTime: LPSYSTEMTIME);
    // pub fn GetLocaleInfoA();
    // pub fn GetLocaleInfoEx();
    // pub fn GetLocaleInfoW();
    // pub fn GetLogicalDriveStringsA();
    pub fn GetLogicalDriveStringsW(nBufferLength: DWORD, lpBuffer: LPWSTR) -> DWORD;
    pub fn GetLogicalDrives() -> DWORD;
    // pub fn GetLogicalProcessorInformation();
    // pub fn GetLogicalProcessorInformationEx();
    pub fn GetLongPathNameA(lpszShortPath: LPCSTR, lpszLongPath: LPSTR, cchBuffer: DWORD) -> DWORD;
    // pub fn GetLongPathNameTransactedA();
    // pub fn GetLongPathNameTransactedW();
    pub fn GetLongPathNameW(
        lpszShortPath: LPCWSTR, lpszLongPath: LPWSTR, cchBuffer: DWORD,
    ) -> DWORD;
    // pub fn GetMailslotInfo();
    // pub fn GetMaximumProcessorCount();
    // pub fn GetMaximumProcessorGroupCount();
    // pub fn GetMemoryErrorHandlingCapabilities();
    // pub fn GetModuleFileNameA();
    // pub fn GetModuleFileNameW();
    pub fn GetModuleHandleA(lpModuleName: LPCSTR) -> HMODULE;
    pub fn GetModuleHandleExA(
        dwFlags: DWORD, lpModuleName: LPCSTR, phModule: *mut HMODULE,
    ) -> BOOL;
    pub fn GetModuleHandleExW(
        dwFlags: DWORD, lpModuleName: LPCWSTR, phModule: *mut HMODULE,
    ) -> BOOL;
    pub fn GetModuleHandleW(lpModuleName: LPCWSTR) -> HMODULE;
    // pub fn GetNLSVersion();
    // pub fn GetNLSVersionEx();
    // pub fn GetNamedPipeAttribute();
    // pub fn GetNamedPipeClientComputerNameA();
    // pub fn GetNamedPipeClientComputerNameW();
    // pub fn GetNamedPipeClientProcessId();
    // pub fn GetNamedPipeClientSessionId();
    // pub fn GetNamedPipeHandleStateA();
    // pub fn GetNamedPipeHandleStateW();
    // pub fn GetNamedPipeInfo();
    // pub fn GetNamedPipeServerProcessId();
    // pub fn GetNamedPipeServerSessionId();
    // pub fn GetNativeSystemInfo();
    // #[cfg(target_arch = "x86_64")]
    // pub fn GetNextUmsListItem();
    // pub fn GetNumaAvailableMemoryNode();
    // pub fn GetNumaAvailableMemoryNodeEx();
    // pub fn GetNumaHighestNodeNumber();
    // pub fn GetNumaNodeNumberFromHandle();
    // pub fn GetNumaNodeProcessorMask();
    // pub fn GetNumaNodeProcessorMaskEx();
    // pub fn GetNumaProcessorNode();
    // pub fn GetNumaProcessorNodeEx();
    // pub fn GetNumaProximityNode();
    // pub fn GetNumaProximityNodeEx();
    // pub fn GetNumberFormatA();
    // pub fn GetNumberFormatEx();
    // pub fn GetNumberFormatW();
    pub fn GetNumberOfConsoleInputEvents(hConsoleInput: HANDLE, lpNumberOfEvents: LPDWORD) -> BOOL;
    pub fn GetNumberOfConsoleMouseButtons(lpNumberOfMouseButtons: LPDWORD) -> BOOL;
    // pub fn GetOEMCP();
    // pub fn GetOverlappedResult();
    // pub fn GetOverlappedResultEx();
    // pub fn GetPackageApplicationIds();
    // pub fn GetPackageFamilyName();
    // pub fn GetPackageFullName();
    // pub fn GetPackageId();
    // pub fn GetPackageInfo();
    // pub fn GetPackagePath();
    // pub fn GetPackagePathByFullName();
    // pub fn GetPackagesByPackageFamily();
    // pub fn GetPhysicallyInstalledSystemMemory();
    // pub fn GetPriorityClass();
    // pub fn GetPrivateProfileIntA();
    // pub fn GetPrivateProfileIntW();
    // pub fn GetPrivateProfileSectionA();
    // pub fn GetPrivateProfileSectionNamesA();
    // pub fn GetPrivateProfileSectionNamesW();
    // pub fn GetPrivateProfileSectionW();
    // pub fn GetPrivateProfileStringA();
    // pub fn GetPrivateProfileStringW();
    // pub fn GetPrivateProfileStructA();
    // pub fn GetPrivateProfileStructW();
    pub fn GetProcAddress(hModule: HMODULE, lpProcName: LPCSTR) -> FARPROC;
    // pub fn GetProcessAffinityMask();
    // pub fn GetProcessDEPPolicy();
    // pub fn GetProcessGroupAffinity();
    // pub fn GetProcessHandleCount();
    pub fn GetProcessHeap() -> HANDLE;
    pub fn GetProcessHeaps(NumberOfHeaps: DWORD, ProcessHeaps: PHANDLE) -> DWORD;
    // pub fn GetProcessId();
    // pub fn GetProcessIdOfThread();
    // pub fn GetProcessInformation();
    // pub fn GetProcessIoCounters();
    // pub fn GetProcessMitigationPolicy();
    // pub fn GetProcessPreferredUILanguages();
    // pub fn GetProcessPriorityBoost();
    // pub fn GetProcessShutdownParameters();
    pub fn GetProcessTimes(
        hProcess: HANDLE, lpCreationTime: LPFILETIME, lpExitTime: LPFILETIME,
        lpKernelTime: LPFILETIME, lpUserTime: LPFILETIME,
    ) -> BOOL;
    // pub fn GetProcessVersion();
    // pub fn GetProcessWorkingSetSize();
    // pub fn GetProcessWorkingSetSizeEx();
    // pub fn GetProcessorSystemCycleTime();
    // pub fn GetProductInfo();
    // pub fn GetProfileIntA();
    // pub fn GetProfileIntW();
    // pub fn GetProfileSectionA();
    // pub fn GetProfileSectionW();
    // pub fn GetProfileStringA();
    // pub fn GetProfileStringW();
    pub fn GetQueuedCompletionStatus(
        CompletionPort: HANDLE, lpNumberOfBytesTransferred: LPDWORD, lpCompletionKey: PULONG_PTR,
        lpOverlapped: *mut LPOVERLAPPED, dwMilliseconds: DWORD,
    ) -> BOOL;
    pub fn GetQueuedCompletionStatusEx(
        CompletionPort: HANDLE, lpCompletionPortEntries: LPOVERLAPPED_ENTRY, ulCount: ULONG,
        ulNumEntriesRemoved: PULONG, dwMilliseconds: DWORD, fAlertable: BOOL,
    ) -> BOOL;
    // pub fn GetShortPathNameA();
    pub fn GetShortPathNameW(
        lpszLongPath: LPCWSTR, lpszShortPath: LPWSTR, cchBuffer: DWORD,
    ) -> DWORD;
    // pub fn GetStagedPackagePathByFullName();
    pub fn GetStartupInfoA(lpStartupInfo: LPSTARTUPINFOA);
    pub fn GetStartupInfoW(lpStartupInfo: LPSTARTUPINFOW);
    // pub fn GetStateFolder();
    pub fn GetStdHandle(nStdHandle: DWORD) -> HANDLE;
    // pub fn GetStringScripts();
    // pub fn GetStringTypeA();
    // pub fn GetStringTypeExA();
    // pub fn GetStringTypeExW();
    // pub fn GetStringTypeW();
    // pub fn GetSystemAppDataKey();
    // pub fn GetSystemDEPPolicy();
    // pub fn GetSystemDefaultLCID();
    // pub fn GetSystemDefaultLangID();
    // pub fn GetSystemDefaultLocaleName();
    // pub fn GetSystemDefaultUILanguage();
    // pub fn GetSystemDirectoryA();
    // pub fn GetSystemDirectoryW();
    // pub fn GetSystemFileCacheSize();
    // pub fn GetSystemFirmwareTable();
    // pub fn GetSystemInfo();
    // pub fn GetSystemPowerStatus();
    // pub fn GetSystemPreferredUILanguages();
    pub fn GetSystemRegistryQuota(pdwQuotaAllowed: PDWORD, pdwQuotaUsed: PDWORD) -> BOOL;
    pub fn GetSystemTime(lpSystemTime: LPSYSTEMTIME);
    pub fn GetSystemTimeAdjustment(
        lpTimeAdjustment: PDWORD, lpTimeIncrement: PDWORD, lpTimeAdjustmentDisabled: PBOOL,
    ) -> BOOL;
    pub fn GetSystemTimeAsFileTime(lpSystemTimeAsFileTime: LPFILETIME);
    pub fn GetSystemTimePreciseAsFileTime(lpSystemTimeAsFileTime: LPFILETIME);
    pub fn GetSystemTimes(
        lpIdleTime: PFILETIME, lpKernelTime: PFILETIME, lpUserTime: PFILETIME,
    ) -> BOOL;
    // pub fn GetSystemWindowsDirectoryA();
    // pub fn GetSystemWindowsDirectoryW();
    // pub fn GetSystemWow64DirectoryA();
    // pub fn GetSystemWow64DirectoryW();
    // pub fn GetTapeParameters();
    // pub fn GetTapePosition();
    // pub fn GetTapeStatus();
    // pub fn GetTempFileNameA();
    pub fn GetTempFileNameW(
        lpPathName: LPCWSTR, lpPrefixString: LPCWSTR, uUnique: UINT, lpTempFileName: LPWSTR,
    ) -> UINT;
    // pub fn GetTempPathA();
    pub fn GetTempPathW(nBufferLength: DWORD, lpBuffer: LPWSTR) -> DWORD;
    // pub fn GetThreadContext();
    // pub fn GetThreadErrorMode();
    // pub fn GetThreadGroupAffinity();
    // pub fn GetThreadIOPendingFlag();
    // pub fn GetThreadId();
    // pub fn GetThreadIdealProcessorEx();
    // pub fn GetThreadInformation();
    // pub fn GetThreadLocale();
    // pub fn GetThreadPreferredUILanguages();
    // pub fn GetThreadPriority();
    // pub fn GetThreadPriorityBoost();
    // pub fn GetThreadSelectorEntry();
    pub fn GetThreadTimes(
        hThread: HANDLE, lpCreationTime: LPFILETIME, lpExitTime: LPFILETIME,
        lpKernelTime: LPFILETIME, lpUserTime: LPFILETIME,
    ) -> BOOL;
    // pub fn GetThreadUILanguage();
    // pub fn GetTickCount();
    // pub fn GetTickCount64();
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
    // pub fn GetUILanguageInfo();
    // #[cfg(target_arch = "x86_64")]
    // pub fn GetUmsCompletionListEvent();
    // #[cfg(target_arch = "x86_64")]
    // pub fn GetUmsSystemThreadInformation();
    // pub fn GetUserDefaultLCID();
    // pub fn GetUserDefaultLangID();
    // pub fn GetUserDefaultLocaleName();
    // pub fn GetUserDefaultUILanguage();
    // pub fn GetUserGeoID();
    // pub fn GetUserPreferredUILanguages();
    // pub fn GetVersion();
    // pub fn GetVersionExA();
    // pub fn GetVersionExW();
    // pub fn GetVolumeInformationA();
    pub fn GetVolumeInformationByHandleW(
        hFile: HANDLE, lpVolumeNameBuffer: LPWSTR, nVolumeNameSize: DWORD,
        lpVolumeSerialNumber: LPDWORD, lpMaximumComponentLength: LPDWORD,
        lpFileSystemFlags: LPDWORD, lpFileSystemNameBuffer: LPWSTR, nFileSystemNameSize: DWORD,
    ) -> BOOL;
    pub fn GetVolumeInformationW(
        lpRootPathName: LPCWSTR, lpVolumeNameBuffer: LPWSTR, nVolumeNameSize: DWORD,
        lpVolumeSerialNumber: LPDWORD, lpMaximumComponentLength: LPDWORD,
        lpFileSystemFlags: LPDWORD, lpFileSystemNameBuffer: LPWSTR, nFileSystemNameSize: DWORD,
    ) -> BOOL;
    // pub fn GetVolumeNameForVolumeMountPointA();
    pub fn GetVolumeNameForVolumeMountPointW(
        lpszVolumeMountPoint: LPCWSTR, lpszVolumeName: LPWSTR, cchBufferLength: DWORD,
    ) -> BOOL;
    // pub fn GetVolumePathNameA();
    pub fn GetVolumePathNameW(
        lpszFileName: LPCWSTR, lpszVolumePathName: LPWSTR, cchBufferLength: DWORD,
    ) -> BOOL;
    // pub fn GetVolumePathNamesForVolumeNameA();
    pub fn GetVolumePathNamesForVolumeNameW(
        lpszVolumeName: LPCWSTR, lpszVolumePathNames: LPWCH, cchBufferLength: DWORD,
        lpcchReturnLength: PDWORD,
    ) -> BOOL;
    // pub fn GetWindowsDirectoryA();
    // pub fn GetWindowsDirectoryW();
    // pub fn GetWriteWatch();
    // #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    // pub fn GetXStateFeaturesMask();
    // pub fn GlobalAddAtomA();
    // pub fn GlobalAddAtomExA();
    // pub fn GlobalAddAtomExW();
    // pub fn GlobalAddAtomW();
    pub fn GlobalAlloc(uFlags: UINT, dwBytes: SIZE_T) -> HGLOBAL;
    // pub fn GlobalCompact();
    // pub fn GlobalDeleteAtom();
    // pub fn GlobalFindAtomA();
    // pub fn GlobalFindAtomW();
    // pub fn GlobalFix();
    // pub fn GlobalFlags();
    // pub fn GlobalFree();
    // pub fn GlobalGetAtomNameA();
    // pub fn GlobalGetAtomNameW();
    // pub fn GlobalHandle();
    pub fn GlobalLock(hMem: HGLOBAL) -> LPVOID;
    // pub fn GlobalMemoryStatus();
    // pub fn GlobalMemoryStatusEx();
    // pub fn GlobalReAlloc();
    // pub fn GlobalSize();
    // pub fn GlobalUnWire();
    // pub fn GlobalUnfix();
    pub fn GlobalUnlock(hMem: HGLOBAL) -> BOOL;
    // pub fn GlobalWire();
    // pub fn Heap32First();
    // pub fn Heap32ListFirst();
    // pub fn Heap32ListNext();
    // pub fn Heap32Next();
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
    // pub fn InitAtomTable();
    // pub fn InitOnceBeginInitialize();
    // pub fn InitOnceComplete();
    // pub fn InitOnceExecuteOnce();
    // pub fn InitOnceInitialize();
    // pub fn InitializeConditionVariable();
    // pub fn InitializeContext();
    pub fn InitializeCriticalSection(lpCriticalSection: LPCRITICAL_SECTION);
    pub fn InitializeCriticalSectionAndSpinCount(
        lpCriticalSection: LPCRITICAL_SECTION, dwSpinCount: DWORD,
    ) -> BOOL;
    pub fn InitializeCriticalSectionEx(
        lpCriticalSection: LPCRITICAL_SECTION, dwSpinCount: DWORD, Flags: DWORD,
    ) -> BOOL;
    // pub fn InitializeProcThreadAttributeList();
    // pub fn InitializeSListHead();
    pub fn InitializeSRWLock(SRWLock: PSRWLOCK);
    // pub fn InitializeSynchronizationBarrier();
    // pub fn InstallELAMCertificateInfo();
    // #[cfg(target_arch = "x86")]
    // pub fn InterlockedCompareExchange();
    // #[cfg(target_arch = "x86")]
    // pub fn InterlockedCompareExchange64();
    // #[cfg(target_arch = "x86")]
    // pub fn InterlockedDecrement();
    // #[cfg(target_arch = "x86")]
    // pub fn InterlockedExchange();
    // #[cfg(target_arch = "x86")]
    // pub fn InterlockedExchangeAdd();
    // pub fn InterlockedFlushSList();
    // #[cfg(target_arch = "x86")]
    // pub fn InterlockedIncrement();
    // pub fn InterlockedPopEntrySList();
    // pub fn InterlockedPushEntrySList();
    // pub fn InterlockedPushListSList();
    // pub fn InterlockedPushListSListEx();
    // pub fn IsBadCodePtr();
    // pub fn IsBadHugeReadPtr();
    // pub fn IsBadHugeWritePtr();
    // pub fn IsBadReadPtr();
    // pub fn IsBadStringPtrA();
    // pub fn IsBadStringPtrW();
    // pub fn IsBadWritePtr();
    // pub fn IsDBCSLeadByte();
    // pub fn IsDBCSLeadByteEx();
    // pub fn IsDebuggerPresent();
    // pub fn IsNLSDefinedString();
    // pub fn IsNativeVhdBoot();
    // pub fn IsNormalizedString();
    // pub fn IsProcessCritical();
    // pub fn IsProcessInJob();
    // pub fn IsProcessorFeaturePresent();
    // pub fn IsSystemResumeAutomatic();
    // pub fn IsThreadAFiber();
    // pub fn IsThreadpoolTimerSet();
    // pub fn IsValidCodePage();
    // pub fn IsValidLanguageGroup();
    // pub fn IsValidLocale();
    // pub fn IsValidLocaleName();
    // pub fn IsValidNLSVersion();
    // pub fn IsWow64Process();
    // pub fn K32EmptyWorkingSet();
    // pub fn K32EnumDeviceDrivers();
    // pub fn K32EnumPageFilesA();
    // pub fn K32EnumPageFilesW();
    // pub fn K32EnumProcessModules();
    // pub fn K32EnumProcessModulesEx();
    pub fn K32EnumProcesses(
        lpidProcess: *mut DWORD, cb: DWORD, lpcbNeeded: LPDWORD,
    ) -> BOOL;
    // pub fn K32GetDeviceDriverBaseNameA();
    // pub fn K32GetDeviceDriverBaseNameW();
    // pub fn K32GetDeviceDriverFileNameA();
    // pub fn K32GetDeviceDriverFileNameW();
    // pub fn K32GetMappedFileNameA();
    // pub fn K32GetMappedFileNameW();
    // pub fn K32GetModuleBaseNameA();
    // pub fn K32GetModuleBaseNameW();
    // pub fn K32GetModuleFileNameExA();
    pub fn K32GetModuleFileNameExW(
        hProcess: HANDLE, hModule: HMODULE, lpFilename: LPWSTR, nSize: DWORD,
    ) -> DWORD;
    // pub fn K32GetModuleInformation();
    // pub fn K32GetPerformanceInfo();
    pub fn K32GetProcessImageFileNameA(
        hProcess: HANDLE, lpImageFileName: LPSTR, nSize: DWORD,
    ) -> DWORD;
    pub fn K32GetProcessImageFileNameW(
        hProcess: HANDLE, lpImageFileName: LPWSTR, nSize: DWORD,
    ) -> DWORD;
    pub fn K32GetProcessMemoryInfo(
        Process: HANDLE, ppsmemCounters: PPROCESS_MEMORY_COUNTERS, cb: DWORD,
    ) -> BOOL;
    // pub fn K32GetWsChanges();
    // pub fn K32GetWsChangesEx();
    // pub fn K32InitializeProcessForWsWatch();
    // pub fn K32QueryWorkingSet();
    // pub fn K32QueryWorkingSetEx();
    // pub fn LCIDToLocaleName();
    // pub fn LCMapStringA();
    // pub fn LCMapStringEx();
    // pub fn LCMapStringW();
    pub fn LeaveCriticalSection(lpCriticalSection: LPCRITICAL_SECTION);
    // pub fn LeaveCriticalSectionWhenCallbackReturns();
    // pub fn LoadAppInitDlls();
    pub fn LoadLibraryA(lpFileName: LPCSTR) -> HMODULE;
    // pub fn LoadLibraryExA();
    // pub fn LoadLibraryExW();
    pub fn LoadLibraryW(lpFileName: LPCWSTR) -> HMODULE;
    // pub fn LoadModule();
    // pub fn LoadPackagedLibrary();
    // pub fn LoadResource();
    // pub fn LoadStringBaseExW();
    // pub fn LoadStringBaseW();
    // pub fn LocalAlloc();
    // pub fn LocalCompact();
    pub fn LocalFileTimeToFileTime(
        lpLocalFileTime: *const FILETIME, lpFileTime: LPFILETIME,
    ) -> BOOL;
    // pub fn LocalFlags();
    pub fn LocalFree(hMem: HLOCAL) -> HLOCAL;
    // pub fn LocalHandle();
    // pub fn LocalLock();
    // pub fn LocalReAlloc();
    // pub fn LocalShrink();
    // pub fn LocalSize();
    // pub fn LocalUnlock();
    // pub fn LocaleNameToLCID();
    // #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    // pub fn LocateXStateFeature();
    pub fn LockFile(
        hFile: HANDLE, dwFileOffsetLow: DWORD, dwFileOffsetHigh: DWORD,
        nNumberOfBytesToLockLow: DWORD, nNumberOfBytesToLockHigh: DWORD,
    ) -> BOOL;
    pub fn LockFileEx(
        hFile: HANDLE, dwFlags: DWORD, dwReserved: DWORD, nNumberOfBytesToLockLow: DWORD,
        nNumberOfBytesToLockHigh: DWORD, lpOverlapped: LPOVERLAPPED,
    ) -> BOOL;
    // pub fn LockResource();
    // pub fn MapUserPhysicalPages();
    // pub fn MapUserPhysicalPagesScatter();
    pub fn MapViewOfFile(
        hFileMappingObject: HANDLE, dwDesiredAccess: DWORD, dwFileOffsetHigh: DWORD,
        dwFileOffsetLow: DWORD, dwNumberOfBytesToMap: SIZE_T,
    ) -> LPVOID;
    pub fn MapViewOfFileEx(
        hFileMappingObject: HANDLE, dwDesiredAccess: DWORD, dwFileOffsetHigh: DWORD,
        dwFileOffsetLow: DWORD, dwNumberOfBytesToMap: SIZE_T, lpBaseAddress: LPVOID,
    ) -> LPVOID;
    // pub fn MapViewOfFileExNuma();
    // pub fn MapViewOfFileFromApp();
    // pub fn Module32First();
    // pub fn Module32FirstW();
    // pub fn Module32Next();
    // pub fn Module32NextW();
    // pub fn MoveFileA();
    pub fn MoveFileExA(lpExistingFileName: LPCSTR, lpNewFileName: LPCSTR, dwFlags: DWORD) -> BOOL;
    pub fn MoveFileExW(
        lpExistingFileName: LPCWSTR, lpNewFileName: LPCWSTR, dwFlags: DWORD,
    ) -> BOOL;
    // pub fn MoveFileTransactedA();
    // pub fn MoveFileTransactedW();
    // pub fn MoveFileW();
    // pub fn MoveFileWithProgressA();
    // pub fn MoveFileWithProgressW();
    // pub fn MulDiv();
    pub fn MultiByteToWideChar(
        CodePage: UINT, dwFlags: DWORD, lpMultiByteStr: LPCCH, cbMultiByte: c_int,
        lpWideCharStr: LPWSTR, cchWideChar: c_int,
    ) -> c_int;
    // pub fn NeedCurrentDirectoryForExePathA();
    // pub fn NeedCurrentDirectoryForExePathW();
    // pub fn NormalizeString();
    // pub fn NotifyMountMgr();
    // pub fn NotifyUILanguageChange();
    // pub fn OOBEComplete();
    // pub fn OpenEventA();
    // pub fn OpenEventW();
    // pub fn OpenFile();
    // pub fn OpenFileById();
    pub fn OpenFileMappingA(
        dwDesiredAccess: DWORD, bInheritHandle: BOOL, lpName: LPCSTR,
    ) -> HANDLE;
    pub fn OpenFileMappingW(
        dwDesiredAccess: DWORD, bInheritHandle: BOOL, lpName: LPCWSTR,
    ) -> HANDLE;
    // pub fn OpenJobObjectA();
    // pub fn OpenJobObjectW();
    // pub fn OpenMutexA();
    // pub fn OpenMutexW();
    // pub fn OpenPackageInfoByFullName();
    // pub fn OpenPrivateNamespaceA();
    // pub fn OpenPrivateNamespaceW();
    pub fn OpenProcess(dwDesiredAccess: DWORD, bInheritHandle: BOOL, dwProcessId: DWORD) -> HANDLE;
    pub fn OpenSemaphoreA(dwDesiredAccess: DWORD, bInheritHandle: BOOL, lpName: LPCSTR) -> HANDLE;
    pub fn OpenSemaphoreW(dwDesiredAccess: DWORD, bInheritHandle: BOOL, lpName: LPCWSTR) -> HANDLE;
    // pub fn OpenState();
    // pub fn OpenStateExplicit();
    // pub fn OpenThread();
    // pub fn OpenWaitableTimerA();
    // pub fn OpenWaitableTimerW();
    // pub fn OutputDebugStringA();
    // pub fn OutputDebugStringW();
    // pub fn PackageFamilyNameFromFullName();
    // pub fn PackageFamilyNameFromId();
    // pub fn PackageFullNameFromId();
    // pub fn PackageIdFromFullName();
    // pub fn PackageNameAndPublisherIdFromFamilyName();
    // pub fn ParseApplicationUserModelId();
    pub fn PeekConsoleInputA(
        hConsoleInput: HANDLE, lpBuffer: PINPUT_RECORD, nLength: DWORD,
        lpNumberOfEventsRead: LPDWORD,
    ) -> BOOL;
    pub fn PeekConsoleInputW(
        hConsoleInput: HANDLE, lpBuffer: PINPUT_RECORD, nLength: DWORD,
        lpNumberOfEventsRead: LPDWORD,
    ) -> BOOL;
    // pub fn PeekNamedPipe();
    pub fn PostQueuedCompletionStatus(
        CompletionPort: HANDLE, dwNumberOfBytesTransferred: DWORD, dwCompletionKey: ULONG_PTR,
        lpOverlapped: LPOVERLAPPED,
    ) -> BOOL;
    // pub fn PowerClearRequest();
    // pub fn PowerCreateRequest();
    // pub fn PowerSetRequest();
    // pub fn PrefetchVirtualMemory();
    // pub fn PrepareTape();
    // pub fn Process32First();
    // pub fn Process32FirstW();
    // pub fn Process32Next();
    // pub fn Process32NextW();
    // pub fn ProcessIdToSessionId();
    // pub fn PssCaptureSnapshot();
    // pub fn PssDuplicateSnapshot();
    // pub fn PssFreeSnapshot();
    // pub fn PssQuerySnapshot();
    // pub fn PssWalkMarkerCreate();
    // pub fn PssWalkMarkerFree();
    // pub fn PssWalkMarkerGetPosition();
    // pub fn PssWalkMarkerRewind();
    // pub fn PssWalkMarkerSeek();
    // pub fn PssWalkMarkerSeekToBeginning();
    // pub fn PssWalkMarkerSetPosition();
    // pub fn PssWalkMarkerTell();
    // pub fn PssWalkSnapshot();
    // pub fn PulseEvent();
    // pub fn PurgeComm();
    // pub fn QueryActCtxSettingsW();
    // pub fn QueryActCtxW();
    // pub fn QueryDepthSList();
    // pub fn QueryDosDeviceA();
    pub fn QueryDosDeviceW(lpDeviceName: LPCWSTR, lpTargetPath: LPWSTR, ucchMax: DWORD) -> DWORD;
    // pub fn QueryFullProcessImageNameA();
    // pub fn QueryFullProcessImageNameW();
    // pub fn QueryIdleProcessorCycleTime();
    // pub fn QueryIdleProcessorCycleTimeEx();
    pub fn QueryInformationJobObject(
        hJob: HANDLE, JobObjectInformationClass: JOBOBJECTINFOCLASS,
        lpJobObjectInformation: LPVOID, cbJobObjectInformationLength: DWORD,
        lpReturnLength: LPDWORD,
    ) -> BOOL;
    // pub fn QueryMemoryResourceNotification();
    pub fn QueryPerformanceCounter(lpPerformanceCount: *mut LARGE_INTEGER) -> BOOL;
    pub fn QueryPerformanceFrequency(lpFrequency: *mut LARGE_INTEGER) -> BOOL;
    // pub fn QueryProcessAffinityUpdateMode();
    // pub fn QueryProcessCycleTime();
    // pub fn QueryProtectedPolicy();
    // pub fn QueryThreadCycleTime();
    // pub fn QueryThreadProfiling();
    // pub fn QueryThreadpoolStackInformation();
    // #[cfg(target_arch = "x86_64")]
    // pub fn QueryUmsThreadInformation();
    // pub fn QueryUnbiasedInterruptTime();
    pub fn QueueUserAPC(pfnAPC: PAPCFUNC, hThread: HANDLE, dwData: ULONG_PTR) -> DWORD;
    // pub fn QueueUserWorkItem();
    // pub fn RaiseException();
    // pub fn RaiseFailFastException();
    pub fn ReOpenFile(
        hOriginalFile: HANDLE, dwDesiredAccess: DWORD, dwShareMode: DWORD, dwFlags: DWORD,
    ) -> HANDLE;
    pub fn ReadConsoleA(
        hConsoleInput: HANDLE, lpBuffer: LPVOID, nNumberOfCharsToRead: DWORD,
        lpNumberOfCharsRead: LPDWORD, pInputControl: PCONSOLE_READCONSOLE_CONTROL,
    ) -> BOOL;
    pub fn ReadConsoleInputA(
        hConsoleInput: HANDLE, lpBuffer: PINPUT_RECORD, nLength: DWORD,
        lpNumberOfEventsRead: LPDWORD,
    ) -> BOOL;
    pub fn ReadConsoleInputW(
        hConsoleInput: HANDLE, lpBuffer: PINPUT_RECORD, nLength: DWORD,
        lpNumberOfEventsRead: LPDWORD,
    ) -> BOOL;
    pub fn ReadConsoleOutputA(
        hConsoleOutput: HANDLE, lpBuffer: PCHAR_INFO, dwBufferSize: COORD, dwBufferCoord: COORD,
        lpReadRegion: PSMALL_RECT,
    ) -> BOOL;
    pub fn ReadConsoleOutputAttribute(
        hConsoleOutput: HANDLE, lpAttribute: LPWORD, nLength: DWORD, dwReadCoord: COORD,
        lpNumberOfAttrsRead: LPDWORD,
    ) -> BOOL;
    pub fn ReadConsoleOutputCharacterA(
        hConsoleOutput: HANDLE, lpCharacter: LPSTR, nLength: DWORD, dwReadCoord: COORD,
        lpNumberOfCharsRead: LPDWORD,
    ) -> BOOL;
    pub fn ReadConsoleOutputCharacterW(
        hConsoleOutput: HANDLE, lpCharacter: LPWSTR, nLength: DWORD, dwReadCoord: COORD,
        lpNumberOfCharsRead: LPDWORD,
    ) -> BOOL;
    pub fn ReadConsoleOutputW(
        hConsoleOutput: HANDLE, lpBuffer: PCHAR_INFO, dwBufferSize: COORD, dwBufferCoord: COORD,
        lpReadRegion: PSMALL_RECT,
    ) -> BOOL;
    pub fn ReadConsoleW(
        hConsoleInput: HANDLE, lpBuffer: LPVOID, nNumberOfCharsToRead: DWORD,
        lpNumberOfCharsRead: LPDWORD, pInputControl: PCONSOLE_READCONSOLE_CONTROL,
    ) -> BOOL;
    pub fn ReadDirectoryChangesW(
        hDirectory: HANDLE, lpBuffer: LPVOID, nBufferLength: DWORD, bWatchSubtree: BOOL,
        dwNotifyFilter: DWORD, lpBytesReturned: LPDWORD, lpOverlapped: LPOVERLAPPED,
        lpCompletionRoutine: LPOVERLAPPED_COMPLETION_ROUTINE,
    ) -> BOOL;
    pub fn ReadFile(
        hFile: HANDLE, lpBuffer: LPVOID, nNumberOfBytesToRead: DWORD, lpNumberOfBytesRead: LPDWORD,
        lpOverlapped: LPOVERLAPPED,
    ) -> BOOL;
    pub fn ReadFileEx(
        hFile: HANDLE, lpBuffer: LPVOID, nNumberOfBytesToRead: DWORD, lpOverlapped: LPOVERLAPPED,
        lpCompletionRoutine: LPOVERLAPPED_COMPLETION_ROUTINE,
    ) -> BOOL;
    pub fn ReadFileScatter(
        hFile: HANDLE, aSegmentArray: *mut FILE_SEGMENT_ELEMENT, nNumberOfBytesToRead: DWORD,
        lpReserved: LPDWORD, lpOverlapped: LPOVERLAPPED,
    ) -> BOOL;
    pub fn ReadProcessMemory(
        hProcess: HANDLE, lpBaseAddress: LPCVOID, lpBuffer: LPVOID, nSize: SIZE_T,
        lpNumberOfBytesRead: *mut SIZE_T,
    ) -> BOOL;
    // pub fn ReadThreadProfilingData();
    // pub fn RegisterApplicationRecoveryCallback();
    // pub fn RegisterApplicationRestart();
    // pub fn RegisterBadMemoryNotification();
    // pub fn RegisterWaitForInputIdle();
    pub fn RegisterWaitForSingleObject(
        phNewWaitObject: PHANDLE, hObject: HANDLE, Callback: WAITORTIMERCALLBACK, Context: PVOID,
        dwMilliseconds: ULONG, dwFlags: ULONG,
    ) -> BOOL;
    // pub fn RegisterWaitForSingleObjectEx();
    // pub fn RegisterWaitUntilOOBECompleted();
    // pub fn ReleaseActCtx();
    // pub fn ReleaseMutex();
    // pub fn ReleaseMutexWhenCallbackReturns();
    pub fn ReleaseSRWLockExclusive(SRWLock: PSRWLOCK);
    pub fn ReleaseSRWLockShared(SRWLock: PSRWLOCK);
    pub fn ReleaseSemaphore(
        hSemaphore: HANDLE, lReleaseCount: LONG, lpPreviousCount: LPLONG,
    ) -> BOOL;
    // pub fn ReleaseSemaphoreWhenCallbackReturns();
    pub fn RemoveDirectoryA(lpPathName: LPCSTR) -> BOOL;
    // pub fn RemoveDirectoryTransactedA();
    // pub fn RemoveDirectoryTransactedW();
    pub fn RemoveDirectoryW(lpPathName: LPCWSTR) -> BOOL;
    // pub fn RemoveDllDirectory();
    // pub fn RemoveLocalAlternateComputerNameA();
    // pub fn RemoveLocalAlternateComputerNameW();
    // pub fn RemoveSecureMemoryCacheCallback();
    // pub fn RemoveVectoredContinueHandler();
    // pub fn RemoveVectoredExceptionHandler();
    // pub fn ReplaceFile();
    // pub fn ReplaceFileA();
    // pub fn ReplaceFileW();
    // pub fn ReplacePartitionUnit();
    // pub fn RequestDeviceWakeup();
    // pub fn RequestWakeupLatency();
    // pub fn ResetEvent();
    // pub fn ResetWriteWatch();
    // pub fn ResolveDelayLoadedAPI();
    // pub fn ResolveDelayLoadsFromDll();
    // pub fn ResolveLocaleName();
    // pub fn RestoreLastError();
    // pub fn ResumeThread();
    // #[cfg(any(target_arch = "arm", target_arch = "x86_64"))]
    // pub fn RtlAddFunctionTable();
    pub fn RtlCaptureContext(ContextRecord: PCONTEXT);
    // pub fn RtlCaptureStackBackTrace();
    // #[cfg(any(target_arch = "arm", target_arch = "x86_64"))]
    // pub fn RtlCompareMemory();
    // #[cfg(any(target_arch = "arm", target_arch = "x86_64"))]
    // pub fn RtlCopyMemory();
    // #[cfg(any(target_arch = "arm", target_arch = "x86_64"))]
    // pub fn RtlDeleteFunctionTable();
    // pub fn RtlFillMemory();
    // #[cfg(any(target_arch = "arm", target_arch = "x86_64"))]
    // pub fn RtlInstallFunctionTableCallback();
    // #[cfg(any(target_arch = "arm", target_arch = "x86_64"))]
    // pub fn RtlLookupFunctionEntry();
    // pub fn RtlMoveMemory();
    // #[cfg(any(target_arch = "arm", target_arch = "x86_64"))]
    // pub fn RtlPcToFileHeader();
    // #[cfg(any(target_arch = "arm", target_arch = "x86_64"))]
    // pub fn RtlRaiseException();
    // #[cfg(any(target_arch = "arm", target_arch = "x86_64"))]
    // pub fn RtlRestoreContext();
    // pub fn RtlUnwind();
    // #[cfg(any(target_arch = "arm", target_arch = "x86_64"))]
    // pub fn RtlUnwindEx();
    // #[cfg(any(target_arch = "arm", target_arch = "x86_64"))]
    // pub fn RtlVirtualUnwind();
    // pub fn RtlZeroMemory();
    pub fn ScrollConsoleScreenBufferA(
        hConsoleOutput: HANDLE, lpScrollRectangle: *const SMALL_RECT,
        lpClipRectangle: *const SMALL_RECT, dwDestinationOrigin: COORD, lpFill: *const CHAR_INFO,
    ) -> BOOL;
    pub fn ScrollConsoleScreenBufferW(
        hConsoleOutput: HANDLE, lpScrollRectangle: *const SMALL_RECT,
        lpClipRectangle: *const SMALL_RECT, dwDestinationOrigin: COORD, lpFill: *const CHAR_INFO,
    ) -> BOOL;
    // pub fn SearchPathA();
    // pub fn SearchPathW();
    // pub fn SetCachedSigningLevel();
    // pub fn SetCalendarInfoA();
    // pub fn SetCalendarInfoW();
    // pub fn SetCommBreak();
    // pub fn SetCommConfig();
    // pub fn SetCommMask();
    // pub fn SetCommState();
    // pub fn SetCommTimeouts();
    // pub fn SetComputerNameA();
    // pub fn SetComputerNameEx2W();
    // pub fn SetComputerNameExA();
    // pub fn SetComputerNameExW();
    // pub fn SetComputerNameW();
    pub fn SetConsoleActiveScreenBuffer(hConsoleOutput: HANDLE) -> BOOL;
    pub fn SetConsoleCP(wCodePageID: UINT) -> BOOL;
    pub fn SetConsoleCtrlHandler(HandlerRoutine: PHANDLER_ROUTINE, Add: BOOL) -> BOOL;
    // pub fn SetConsoleCursor();
    pub fn SetConsoleCursorInfo(
        hConsoleOutput: HANDLE, lpConsoleCursorInfo: *const CONSOLE_CURSOR_INFO,
    ) -> BOOL;
    pub fn SetConsoleCursorPosition(hConsoleOutput: HANDLE, dwCursorPosition: COORD) -> BOOL;
    pub fn SetConsoleDisplayMode(
        hConsoleOutput: HANDLE, dwFlags: DWORD, lpNewScreenBufferDimensions: PCOORD,
    ) -> BOOL;
    pub fn SetConsoleHistoryInfo(lpConsoleHistoryInfo: PCONSOLE_HISTORY_INFO) -> BOOL;
    pub fn SetConsoleMode(hConsoleHandle: HANDLE, dwMode: DWORD) -> BOOL;
    pub fn SetConsoleOutputCP(wCodePageID: UINT) -> BOOL;
    pub fn SetConsoleScreenBufferInfoEx(
        hConsoleOutput: HANDLE, lpConsoleScreenBufferInfoEx: PCONSOLE_SCREEN_BUFFER_INFOEX,
    ) -> BOOL;
    pub fn SetConsoleScreenBufferSize(hConsoleOutput: HANDLE, dwSize: COORD) -> BOOL;
    pub fn SetConsoleTextAttribute(hConsoleOutput: HANDLE, wAttributes: WORD) -> BOOL;
    pub fn SetConsoleTitleA(lpConsoleTitle: LPCSTR) -> BOOL;
    pub fn SetConsoleTitleW(lpConsoleTitle: LPCWSTR) -> BOOL;
    pub fn SetConsoleWindowInfo(
        hConsoleOutput: HANDLE, bAbsolute: BOOL, lpConsoleWindow: *const SMALL_RECT,
    ) -> BOOL;
    pub fn SetCriticalSectionSpinCount(
        lpCriticalSection: LPCRITICAL_SECTION, dwSpinCount: DWORD,
    ) -> DWORD;
    // pub fn SetCurrentConsoleFontEx();
    pub fn SetCurrentDirectoryA(lpPathName: LPCSTR) -> BOOL;
    pub fn SetCurrentDirectoryW(lpPathName: LPCWSTR) -> BOOL;
    // pub fn SetDefaultCommConfigA();
    // pub fn SetDefaultCommConfigW();
    // pub fn SetDefaultDllDirectories();
    // pub fn SetDllDirectoryA();
    // pub fn SetDllDirectoryW();
    pub fn SetDynamicTimeZoneInformation(
        lpTimeZoneInformation: *const DYNAMIC_TIME_ZONE_INFORMATION,
    ) -> BOOL;
    pub fn SetEndOfFile(hFile: HANDLE) -> BOOL;
    // pub fn SetEnvironmentStringsA();
    // pub fn SetEnvironmentStringsW();
    // pub fn SetEnvironmentVariableA();
    // pub fn SetEnvironmentVariableW();
    // pub fn SetErrorMode();
    // pub fn SetEvent();
    // pub fn SetEventWhenCallbackReturns();
    // pub fn SetFileApisToANSI();
    // pub fn SetFileApisToOEM();
    pub fn SetFileAttributesA(lpFileName: LPCSTR, dwFileAttributes: DWORD) -> BOOL;
    // pub fn SetFileAttributesTransactedA();
    // pub fn SetFileAttributesTransactedW();
    pub fn SetFileAttributesW(lpFileName: LPCWSTR, dwFileAttributes: DWORD) -> BOOL;
    // pub fn SetFileBandwidthReservation();
    // pub fn SetFileCompletionNotificationModes();
    pub fn SetFileInformationByHandle(
        hFile: HANDLE, FileInformationClass: FILE_INFO_BY_HANDLE_CLASS, lpFileInformation: LPVOID,
        dwBufferSize: DWORD,
    ) -> BOOL;
    pub fn SetFileIoOverlappedRange(
        FileHandle: HANDLE, OverlappedRangeStart: PUCHAR, Length: ULONG,
    ) -> BOOL;
    pub fn SetFilePointer(
        hFile: HANDLE, lDistanceToMove: LONG, lpDistanceToMoveHigh: PLONG, dwMoveMethod: DWORD,
    ) -> DWORD;
    pub fn SetFilePointerEx(
        hFile: HANDLE, liDistanceToMove: LARGE_INTEGER, lpNewFilePointer: PLARGE_INTEGER,
        dwMoveMethod: DWORD,
    ) -> BOOL;
    // pub fn SetFileShortNameA();
    // pub fn SetFileShortNameW();
    pub fn SetFileTime(
        hFile: HANDLE, lpCreationTime: *const FILETIME, lpLastAccessTime: *const FILETIME,
        lpLastWriteTime: *const FILETIME,
    ) -> BOOL;
    pub fn SetFileValidData(hFile: HANDLE, ValidDataLength: LONGLONG) -> BOOL;
    // pub fn SetFirmwareEnvironmentVariableA();
    // pub fn SetFirmwareEnvironmentVariableExA();
    // pub fn SetFirmwareEnvironmentVariableExW();
    // pub fn SetFirmwareEnvironmentVariableW();
    // pub fn SetHandleCount();
    // pub fn SetHandleInformation();
    pub fn SetInformationJobObject(
        hJob: HANDLE, JobObjectInformationClass: JOBOBJECTINFOCLASS,
        lpJobObjectInformation: LPVOID, cbJobObjectInformationLength: DWORD,
    ) -> BOOL;
    pub fn SetLastError(dwErrCode: DWORD);
    // pub fn SetLocalPrimaryComputerNameA();
    // pub fn SetLocalPrimaryComputerNameW();
    pub fn SetLocalTime(lpSystemTime: *const SYSTEMTIME) -> BOOL;
    // pub fn SetLocaleInfoA();
    // pub fn SetLocaleInfoW();
    // pub fn SetMailslotInfo();
    // pub fn SetMessageWaitingIndicator();
    // pub fn SetNamedPipeAttribute();
    // pub fn SetNamedPipeHandleState();
    pub fn SetPriorityClass(hProcess: HANDLE, dwPriorityClass: DWORD);
    // pub fn SetProcessAffinityMask();
    // pub fn SetProcessAffinityUpdateMode();
    // pub fn SetProcessDEPPolicy();
    // pub fn SetProcessInformation();
    // pub fn SetProcessMitigationPolicy();
    // pub fn SetProcessPreferredUILanguages();
    // pub fn SetProcessPriorityBoost();
    // pub fn SetProcessShutdownParameters();
    // pub fn SetProcessWorkingSetSize();
    // pub fn SetProcessWorkingSetSizeEx();
    // pub fn SetProtectedPolicy();
    // pub fn SetSearchPathMode();
    // pub fn SetStdHandle();
    // pub fn SetStdHandleEx();
    // pub fn SetSystemFileCacheSize();
    // pub fn SetSystemPowerState();
    pub fn SetSystemTime(lpSystemTime: *const SYSTEMTIME) -> BOOL;
    pub fn SetSystemTimeAdjustment(dwTimeAdjustment: DWORD, bTimeAdjustmentDisabled: BOOL) -> BOOL;
    // pub fn SetTapeParameters();
    // pub fn SetTapePosition();
    // pub fn SetThreadAffinityMask();
    // pub fn SetThreadContext();
    // pub fn SetThreadErrorMode();
    // pub fn SetThreadExecutionState();
    // pub fn SetThreadGroupAffinity();
    // pub fn SetThreadIdealProcessor();
    // pub fn SetThreadIdealProcessorEx();
    // pub fn SetThreadInformation();
    // pub fn SetThreadLocale();
    // pub fn SetThreadPreferredUILanguages();
    // pub fn SetThreadPriority();
    // pub fn SetThreadPriorityBoost();
    // pub fn SetThreadStackGuarantee();
    // pub fn SetThreadUILanguage();
    // pub fn SetThreadpoolStackInformation();
    // pub fn SetThreadpoolThreadMaximum();
    // pub fn SetThreadpoolThreadMinimum();
    // pub fn SetThreadpoolTimer();
    // pub fn SetThreadpoolTimerEx();
    // pub fn SetThreadpoolWait();
    // pub fn SetThreadpoolWaitEx();
    pub fn SetTimeZoneInformation(lpTimeZoneInformation: *const TIME_ZONE_INFORMATION) -> BOOL;
    // pub fn SetTimerQueueTimer();
    // #[cfg(target_arch = "x86_64")]
    // pub fn SetUmsThreadInformation();
    // pub fn SetUnhandledExceptionFilter();
    // pub fn SetUserGeoID();
    // pub fn SetVolumeLabelA();
    // pub fn SetVolumeLabelW();
    // pub fn SetVolumeMountPointA();
    // pub fn SetVolumeMountPointW();
    // pub fn SetWaitableTimer();
    // pub fn SetWaitableTimerEx();
    // #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    // pub fn SetXStateFeaturesMask();
    // pub fn SetupComm();
    // pub fn SignalObjectAndWait();
    // pub fn SizeofResource();
    pub fn Sleep(dwMilliseconds: DWORD);
    // pub fn SleepConditionVariableCS();
    // pub fn SleepConditionVariableSRW();
    pub fn SleepEx(dwMilliseconds: DWORD, bAlertable: BOOL) -> DWORD;
    // pub fn StartThreadpoolIo();
    // pub fn SubmitThreadpoolWork();
    // pub fn SuspendThread();
    // pub fn SwitchToFiber();
    // pub fn SwitchToThread();
    pub fn SystemTimeToFileTime(
        lpSystemTime: *const SYSTEMTIME, lpFileTime: LPFILETIME,
    ) -> BOOL;
    pub fn SystemTimeToTzSpecificLocalTime(
        lpTimeZoneInformation: *const TIME_ZONE_INFORMATION, lpUniversalTime: *const SYSTEMTIME,
        lpLocalTime: LPSYSTEMTIME,
    ) -> BOOL;
    pub fn SystemTimeToTzSpecificLocalTimeEx(
        lpTimeZoneInformation: *const DYNAMIC_TIME_ZONE_INFORMATION,
        lpUniversalTime: *const SYSTEMTIME, lpLocalTime: LPSYSTEMTIME,
    ) -> BOOL;
    pub fn TerminateJobObject(hJob: HANDLE, uExitCode: UINT) -> BOOL;
    pub fn TerminateProcess(hProcess: HANDLE, uExitCode: UINT) -> BOOL;
    // pub fn TerminateThread();
    // pub fn Thread32First();
    // pub fn Thread32Next();
    // pub fn TlsAlloc();
    // pub fn TlsFree();
    // pub fn TlsGetValue();
    // pub fn TlsSetValue();
    // pub fn Toolhelp32ReadProcessMemory();
    // pub fn TransactNamedPipe();
    // pub fn TransmitCommChar();
    pub fn TryAcquireSRWLockExclusive(SRWLock: PSRWLOCK) -> BOOLEAN;
    pub fn TryAcquireSRWLockShared(SRWLock: PSRWLOCK) -> BOOLEAN;
    pub fn TryEnterCriticalSection(lpCriticalSection: LPCRITICAL_SECTION) -> BOOL;
    // pub fn TrySubmitThreadpoolCallback();
    pub fn TzSpecificLocalTimeToSystemTime(
        lpTimeZoneInformation: *const TIME_ZONE_INFORMATION, lpLocalTime: *const SYSTEMTIME,
        lpUniversalTime: LPSYSTEMTIME,
    ) -> BOOL;
    pub fn TzSpecificLocalTimeToSystemTimeEx(
        lpTimeZoneInformation: *const DYNAMIC_TIME_ZONE_INFORMATION,
        lpLocalTime: *const SYSTEMTIME, lpUniversalTime: LPSYSTEMTIME,
    ) -> BOOL;
    // #[cfg(target_arch = "x86_64")]
    // pub fn UmsThreadYield();
    // pub fn UnhandledExceptionFilter();
    pub fn UnlockFile(
        hFile: HANDLE, dwFileOffsetLow: DWORD, dwFileOffsetHigh: DWORD,
        nNumberOfBytesToUnlockLow: DWORD, nNumberOfBytesToUnlockHigh: DWORD,
    ) -> BOOL;
    pub fn UnlockFileEx(
        hFile: HANDLE, dwReserved: DWORD, nNumberOfBytesToUnlockLow: DWORD,
        nNumberOfBytesToUnlockHigh: DWORD, lpOverlapped: LPOVERLAPPED,
    ) -> BOOL;
    pub fn UnmapViewOfFile(lpBaseAddress: LPCVOID) -> BOOL;
    // pub fn UnregisterApplicationRecoveryCallback();
    // pub fn UnregisterApplicationRestart();
    // pub fn UnregisterBadMemoryNotification();
    pub fn UnregisterWait(WaitHandle: HANDLE) -> BOOL;
    pub fn UnregisterWaitEx(WaitHandle: HANDLE, CompletionEvent: HANDLE) -> BOOL;
    // pub fn UnregisterWaitUntilOOBECompleted();
    // pub fn UpdateProcThreadAttribute();
    // pub fn UpdateResourceA();
    // pub fn UpdateResourceW();
    // pub fn VerLanguageNameA();
    // pub fn VerLanguageNameW();
    // pub fn VerSetConditionMask();
    // pub fn VerifyScripts();
    // pub fn VerifyVersionInfoA();
    // pub fn VerifyVersionInfoW();
    pub fn VirtualAlloc(
        lpAddress: LPVOID, dwSize: SIZE_T, flAllocationType: DWORD, flProtect: DWORD
    ) -> LPVOID;
    // pub fn VirtualAllocEx();
    // pub fn VirtualAllocExNuma();
    pub fn VirtualFree(lpAddress: LPVOID, dwSize: SIZE_T, dwFreeType: DWORD) -> BOOL;
    // pub fn VirtualFreeEx();
    // pub fn VirtualLock();
    // pub fn VirtualProtect();
    // pub fn VirtualProtectEx();
    // pub fn VirtualQuery();
    // pub fn VirtualQueryEx();
    // pub fn VirtualUnlock();
    // pub fn WTSGetActiveConsoleSessionId();
    // pub fn WaitCommEvent();
    // pub fn WaitForDebugEvent();
    // pub fn WaitForMultipleObjects();
    // pub fn WaitForMultipleObjectsEx();
    // pub fn WaitForSingleObject();
    // pub fn WaitForSingleObjectEx();
    // pub fn WaitForThreadpoolIoCallbacks();
    // pub fn WaitForThreadpoolTimerCallbacks();
    // pub fn WaitForThreadpoolWaitCallbacks();
    // pub fn WaitForThreadpoolWorkCallbacks();
    // pub fn WaitNamedPipeA();
    // pub fn WaitNamedPipeW();
    // pub fn WakeAllConditionVariable();
    // pub fn WakeConditionVariable();
    // pub fn WerGetFlags();
    // pub fn WerRegisterFile();
    // pub fn WerRegisterMemoryBlock();
    // pub fn WerRegisterRuntimeExceptionModule();
    // pub fn WerSetFlags();
    // pub fn WerUnregisterFile();
    // pub fn WerUnregisterMemoryBlock();
    // pub fn WerUnregisterRuntimeExceptionModule();
    // pub fn WerpInitiateRemoteRecovery();
    pub fn WideCharToMultiByte(
      CodePage: UINT, dwFlags: DWORD, lpWideCharStr: LPCWCH, cchWideChar: c_int,
      lpMultiByteStr: LPSTR, cbMultiByte: c_int, lpDefaultChar: LPCCH, lpUsedDefaultChar: LPBOOL,
    ) -> c_int;
    pub fn WinExec(lpCmdLine: LPCSTR, uCmdShow: UINT) -> UINT;
    // pub fn Wow64DisableWow64FsRedirection();
    // pub fn Wow64EnableWow64FsRedirection();
    // pub fn Wow64GetThreadContext();
    // pub fn Wow64GetThreadSelectorEntry();
    // pub fn Wow64RevertWow64FsRedirection();
    // pub fn Wow64SetThreadContext();
    // pub fn Wow64SuspendThread();
    pub fn WriteConsoleA(
        hConsoleOutput: HANDLE, lpBuffer: *const VOID, nNumberOfCharsToWrite: DWORD,
        lpNumberOfCharsWritten: LPDWORD, lpReserved: LPVOID,
    ) -> BOOL;
    pub fn WriteConsoleInputA(
        hConsoleInput: HANDLE, lpBuffer: *const INPUT_RECORD, nLength: DWORD,
        lpNumberOfEventsWritten: LPDWORD,
    ) -> BOOL;
    pub fn WriteConsoleInputW(
        hConsoleInput: HANDLE, lpBuffer: *const INPUT_RECORD, nLength: DWORD,
        lpNumberOfEventsWritten: LPDWORD,
    ) -> BOOL;
    pub fn WriteConsoleOutputA(
        hConsoleOutput: HANDLE, lpBuffer: *const CHAR_INFO, dwBufferSize: COORD,
        dwBufferCoord: COORD, lpWriteRegion: PSMALL_RECT,
    ) -> BOOL;
    pub fn WriteConsoleOutputAttribute(
        hConsoleOutput: HANDLE, lpAttribute: *const WORD, nLength: DWORD, dwWriteCoord: COORD,
        lpNumberOfAttrsWritten: LPDWORD,
    ) -> BOOL;
    pub fn WriteConsoleOutputCharacterA(
        hConsoleOutput: HANDLE, lpCharacter: LPCSTR, nLength: DWORD, dwWriteCoord: COORD,
        lpNumberOfCharsWritten: LPDWORD,
    ) -> BOOL;
    pub fn WriteConsoleOutputCharacterW(
        hConsoleOutput: HANDLE, lpCharacter: LPCWSTR, nLength: DWORD, dwWriteCoord: COORD,
        lpNumberOfCharsWritten: LPDWORD,
    ) -> BOOL;
    pub fn WriteConsoleOutputW(
        hConsoleOutput: HANDLE, lpBuffer: *const CHAR_INFO, dwBufferSize: COORD,
        dwBufferCoord: COORD, lpWriteRegion: PSMALL_RECT,
    ) -> BOOL;
    pub fn WriteConsoleW(
        hConsoleOutput: HANDLE, lpBuffer: *const VOID, nNumberOfCharsToWrite: DWORD,
        lpNumberOfCharsWritten: LPDWORD, lpReserved: LPVOID,
    ) -> BOOL;
    pub fn WriteFile(
        hFile: HANDLE, lpBuffer: LPCVOID, nNumberOfBytesToWrite: DWORD,
        lpNumberOfBytesWritten: LPDWORD, lpOverlapped: LPOVERLAPPED,
    ) -> BOOL;
    pub fn WriteFileEx(
        hFile: HANDLE, lpBuffer: LPCVOID, nNumberOfBytesToWrite: DWORD, lpOverlapped: LPOVERLAPPED,
        lpCompletionRoutine: LPOVERLAPPED_COMPLETION_ROUTINE,
    ) -> BOOL;
    pub fn WriteFileGather(
        hFile: HANDLE, aSegmentArray: *mut FILE_SEGMENT_ELEMENT, nNumberOfBytesToWrite: DWORD,
        lpReserved: LPDWORD, lpOverlapped: LPOVERLAPPED,
    ) -> BOOL;
    // pub fn WritePrivateProfileSectionA();
    // pub fn WritePrivateProfileSectionW();
    // pub fn WritePrivateProfileStringA();
    // pub fn WritePrivateProfileStringW();
    // pub fn WritePrivateProfileStructA();
    // pub fn WritePrivateProfileStructW();
    pub fn WriteProcessMemory(
        hProcess: HANDLE, lpBaseAddress: LPVOID, lpBuffer: LPCVOID, nSize: SIZE_T,
        lpNumberOfBytesWritten: *mut SIZE_T,
    ) -> BOOL;
    // pub fn WriteProfileSectionA();
    // pub fn WriteProfileSectionW();
    // pub fn WriteProfileStringA();
    // pub fn WriteProfileStringW();
    // pub fn WriteTapemark();
    // pub fn ZombifyActCtx();
    // pub fn _hread();
    // pub fn _hwrite();
    // pub fn _lclose();
    // pub fn _lcreat();
    // pub fn _llseek();
    // pub fn _lopen();
    // pub fn _lread();
    // pub fn _lwrite();
    // pub fn lstrcat();
    // pub fn lstrcatA();
    // pub fn lstrcatW();
    // pub fn lstrcmp();
    // pub fn lstrcmpA();
    // pub fn lstrcmpW();
    // pub fn lstrcmpi();
    // pub fn lstrcmpiA();
    // pub fn lstrcmpiW();
    // pub fn lstrcpy();
    // pub fn lstrcpyA();
    // pub fn lstrcpyW();
    // pub fn lstrcpyn();
    // pub fn lstrcpynA();
    // pub fn lstrcpynW();
    // pub fn lstrlen();
    pub fn lstrlenA(lpString: LPCSTR) -> i32;
    pub fn lstrlenW(lpString: LPCWSTR) -> i32;
    // #[cfg(any(target_arch = "arm", target_arch = "x86_64"))]
    // pub fn uaw_lstrcmpW();
    // #[cfg(any(target_arch = "arm", target_arch = "x86_64"))]
    // pub fn uaw_lstrcmpiW();
    // #[cfg(any(target_arch = "arm", target_arch = "x86_64"))]
    // pub fn uaw_lstrlenW();
    // #[cfg(any(target_arch = "arm", target_arch = "x86_64"))]
    // pub fn uaw_wcschr();
    // #[cfg(any(target_arch = "arm", target_arch = "x86_64"))]
    // pub fn uaw_wcscpy();
    // #[cfg(any(target_arch = "arm", target_arch = "x86_64"))]
    // pub fn uaw_wcsicmp();
    // #[cfg(any(target_arch = "arm", target_arch = "x86_64"))]
    // pub fn uaw_wcslen();
    // #[cfg(any(target_arch = "arm", target_arch = "x86_64"))]
    // pub fn uaw_wcsrchr();
}

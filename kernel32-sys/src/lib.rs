// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to kernel32.
#![no_std]
#![experimental]
#[cfg(test)] extern crate std;
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
    // pub fn AddVectoredExceptionHandler();
    pub fn AllocConsole() -> BOOL;
    // pub fn AllocateUserPhysicalPages();
    // pub fn AllocateUserPhysicalPagesNuma();
    // pub fn AppXGetOSMaxVersionTested();
    // pub fn ApplicationRecoveryFinished();
    // pub fn ApplicationRecoveryInProgress();
    // pub fn AreFileApisANSI();
    // pub fn AssignProcessToJobObject();
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
    // pub fn CancelIo();
    // pub fn CancelIoEx();
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
    // pub fn CompareFileTime();
    // pub fn CompareStringA();
    // pub fn CompareStringEx();
    // pub fn CompareStringOrdinal();
    // pub fn CompareStringW();
    // pub fn ConnectNamedPipe();
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
    // pub fn CreateEventA();
    // pub fn CreateEventExA();
    // pub fn CreateEventExW();
    // pub fn CreateEventW();
    // pub fn CreateFiber();
    // pub fn CreateFiberEx();
    // pub fn CreateFile2();
    // pub fn CreateFileA();
    // pub fn CreateFileMappingA();
    // pub fn CreateFileMappingFromApp();
    // pub fn CreateFileMappingNumaA();
    // pub fn CreateFileMappingNumaW();
    // pub fn CreateFileMappingW();
    // pub fn CreateFileTransactedA();
    // pub fn CreateFileTransactedW();
    // pub fn CreateFileW();
    // pub fn CreateHardLinkA();
    // pub fn CreateHardLinkTransactedA();
    // pub fn CreateHardLinkTransactedW();
    // pub fn CreateHardLinkW();
    pub fn CreateIoCompletionPort(
        FileHandle: HANDLE, ExistingCompletionPort: HANDLE, CompletionKey: ULONG_PTR,
        NumberOfConcurrentThreads: DWORD,
    ) -> HANDLE;
    // pub fn CreateJobObjectA();
    // pub fn CreateJobObjectW();
    // pub fn CreateJobSet();
    // pub fn CreateMailslotA();
    // pub fn CreateMailslotW();
    // pub fn CreateMemoryResourceNotification();
    // pub fn CreateMutexA();
    // pub fn CreateMutexExA();
    // pub fn CreateMutexExW();
    // pub fn CreateMutexW();
    // pub fn CreateNamedPipeA();
    // pub fn CreateNamedPipeW();
    // pub fn CreatePipe();
    // pub fn CreatePrivateNamespaceA();
    // pub fn CreatePrivateNamespaceW();
    // pub fn CreateProcessA();
    // pub fn CreateProcessW();
    // pub fn CreateRemoteThread();
    // pub fn CreateRemoteThreadEx();
    // pub fn CreateSemaphoreA();
    // pub fn CreateSemaphoreExA();
    // pub fn CreateSemaphoreExW();
    // pub fn CreateSemaphoreW();
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
    // pub fn DefineDosDeviceW();
    // pub fn DelayLoadFailureHook();
    // pub fn DeleteAtom();
    // pub fn DeleteBoundaryDescriptor();
    // pub fn DeleteCriticalSection();
    // pub fn DeleteFiber();
    // pub fn DeleteFileA();
    // pub fn DeleteFileTransactedA();
    // pub fn DeleteFileTransactedW();
    // pub fn DeleteFileW();
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
    // pub fn DeleteVolumeMountPointW();
    // #[cfg(target_arch = "x86_64")]
    // pub fn DequeueUmsCompletionListItems();
    // pub fn DeviceIoControl();
    // pub fn DisableThreadLibraryCalls();
    // pub fn DisableThreadProfiling();
    // pub fn DisassociateCurrentThreadFromCallback();
    // pub fn DisconnectNamedPipe();
    // pub fn DnsHostnameToComputerNameA();
    // pub fn DnsHostnameToComputerNameExW();
    // pub fn DnsHostnameToComputerNameW();
    // pub fn DosDateTimeToFileTime();
    // pub fn DosPathToSessionPathW();
    // pub fn DuplicateHandle();
    // pub fn EnableThreadProfiling();
    // pub fn EncodePointer();
    // pub fn EncodeSystemPointer();
    // pub fn EndUpdateResourceA();
    // pub fn EndUpdateResourceW();
    // pub fn EnterCriticalSection();
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
    // pub fn FileTimeToDosDateTime();
    // pub fn FileTimeToLocalFileTime();
    // pub fn FileTimeToSystemTime();
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
    // pub fn FindClose();
    pub fn FindCloseChangeNotification(hChangeHandle: HANDLE) -> BOOL;
    pub fn FindFirstChangeNotificationA(
        lpPathName: LPCSTR, bWatchSubtree: BOOL, dwNotifyFilter: DWORD,
    ) -> HANDLE;
    pub fn FindFirstChangeNotificationW(
        lpPathName: LPCWSTR, bWatchSubtree: BOOL, dwNotifyFilter: DWORD,
    ) -> HANDLE;
    // pub fn FindFirstFileA();
    // pub fn FindFirstFileExA();
    // pub fn FindFirstFileExW();
    // pub fn FindFirstFileNameTransactedW();
    // pub fn FindFirstFileNameW();
    // pub fn FindFirstFileTransactedA();
    // pub fn FindFirstFileTransactedW();
    // pub fn FindFirstFileW();
    // pub fn FindFirstStreamTransactedW();
    // pub fn FindFirstStreamW();
    // pub fn FindFirstVolumeA();
    // pub fn FindFirstVolumeMountPointA();
    // pub fn FindFirstVolumeMountPointW();
    // pub fn FindFirstVolumeW();
    // pub fn FindNLSString();
    // pub fn FindNLSStringEx();
    pub fn FindNextChangeNotification(hChangeHandle: HANDLE) -> BOOL;
    // pub fn FindNextFileA();
    // pub fn FindNextFileNameW();
    // pub fn FindNextFileW();
    // pub fn FindNextStreamW();
    // pub fn FindNextVolumeA();
    // pub fn FindNextVolumeMountPointA();
    // pub fn FindNextVolumeMountPointW();
    // pub fn FindNextVolumeW();
    // pub fn FindPackagesByPackageFamily();
    // pub fn FindResourceA();
    // pub fn FindResourceExA();
    // pub fn FindResourceExW();
    // pub fn FindResourceW();
    // pub fn FindStringOrdinal();
    // pub fn FindVolumeClose();
    // pub fn FindVolumeMountPointClose();
    // pub fn FlsAlloc();
    // pub fn FlsFree();
    // pub fn FlsGetValue();
    // pub fn FlsSetValue();
    pub fn FlushConsoleInputBuffer(hConsoleInput: HANDLE) -> BOOL;
    // pub fn FlushFileBuffers();
    // pub fn FlushInstructionCache();
    // pub fn FlushProcessWriteBuffers();
    // pub fn FlushViewOfFile();
    // pub fn FoldStringA();
    // pub fn FoldStringW();
    // pub fn FormatApplicationUserModelId();
    // pub fn FormatMessageA();
    // pub fn FormatMessageW();
    pub fn FreeConsole() -> BOOL;
    // pub fn FreeEnvironmentStringsA();
    // pub fn FreeEnvironmentStringsW();
    // pub fn FreeLibrary();
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
    // pub fn GetCommandLineA();
    // pub fn GetCommandLineW();
    // pub fn GetCompressedFileSizeA();
    // pub fn GetCompressedFileSizeTransactedA();
    // pub fn GetCompressedFileSizeTransactedW();
    // pub fn GetCompressedFileSizeW();
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
    // pub fn GetCurrentProcessId();
    // pub fn GetCurrentProcessorNumber();
    // pub fn GetCurrentProcessorNumberEx();
    // pub fn GetCurrentThread();
    // pub fn GetCurrentThreadId();
    // pub fn GetCurrentThreadStackLimits();
    // #[cfg(target_arch = "x86_64")]
    // pub fn GetCurrentUmsThread();
    // pub fn GetDateFormatA();
    // pub fn GetDateFormatEx();
    // pub fn GetDateFormatW();
    // pub fn GetDefaultCommConfigA();
    // pub fn GetDefaultCommConfigW();
    // pub fn GetDevicePowerState();
    // pub fn GetDiskFreeSpaceA();
    // pub fn GetDiskFreeSpaceExA();
    // pub fn GetDiskFreeSpaceExW();
    // pub fn GetDiskFreeSpaceW();
    // pub fn GetDllDirectoryA();
    // pub fn GetDllDirectoryW();
    // pub fn GetDriveTypeA();
    // pub fn GetDriveTypeW();
    // pub fn GetDurationFormat();
    // pub fn GetDurationFormatEx();
    // pub fn GetDynamicTimeZoneInformation();
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
    // pub fn GetFileAttributesA();
    // pub fn GetFileAttributesExA();
    // pub fn GetFileAttributesExW();
    // pub fn GetFileAttributesTransactedA();
    // pub fn GetFileAttributesTransactedW();
    // pub fn GetFileAttributesW();
    // pub fn GetFileBandwidthReservation();
    // pub fn GetFileInformationByHandle();
    // pub fn GetFileInformationByHandleEx();
    // pub fn GetFileMUIInfo();
    // pub fn GetFileMUIPath();
    // pub fn GetFileSize();
    // pub fn GetFileSizeEx();
    // pub fn GetFileTime();
    // pub fn GetFileType();
    // pub fn GetFinalPathNameByHandleA();
    // pub fn GetFinalPathNameByHandleW();
    // pub fn GetFirmwareEnvironmentVariableA();
    // pub fn GetFirmwareEnvironmentVariableExA();
    // pub fn GetFirmwareEnvironmentVariableExW();
    // pub fn GetFirmwareEnvironmentVariableW();
    // pub fn GetFirmwareType();
    // pub fn GetFullPathNameA();
    // pub fn GetFullPathNameTransactedA();
    // pub fn GetFullPathNameTransactedW();
    // pub fn GetFullPathNameW();
    // pub fn GetGeoInfoA();
    // pub fn GetGeoInfoW();
    // pub fn GetHandleInformation();
    // pub fn GetLargePageMinimum();
    pub fn GetLargestConsoleWindowSize(hConsoleOutput: HANDLE) -> COORD;
    pub fn GetLastError() -> DWORD;
    // pub fn GetLocalTime();
    // pub fn GetLocaleInfoA();
    // pub fn GetLocaleInfoEx();
    // pub fn GetLocaleInfoW();
    // pub fn GetLogicalDriveStringsA();
    // pub fn GetLogicalDriveStringsW();
    // pub fn GetLogicalDrives();
    // pub fn GetLogicalProcessorInformation();
    // pub fn GetLogicalProcessorInformationEx();
    // pub fn GetLongPathNameA();
    // pub fn GetLongPathNameTransactedA();
    // pub fn GetLongPathNameTransactedW();
    // pub fn GetLongPathNameW();
    // pub fn GetMailslotInfo();
    // pub fn GetMaximumProcessorCount();
    // pub fn GetMaximumProcessorGroupCount();
    // pub fn GetMemoryErrorHandlingCapabilities();
    // pub fn GetModuleFileNameA();
    // pub fn GetModuleFileNameW();
    pub fn GetModuleHandleA(lpModuleName: LPCSTR) -> HMODULE;
    // pub fn GetModuleHandleExA();
    // pub fn GetModuleHandleExW();
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
    // pub fn GetProcessHeap();
    // pub fn GetProcessHeaps();
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
    // pub fn GetShortPathNameW();
    // pub fn GetStagedPackagePathByFullName();
    // pub fn GetStartupInfoA();
    // pub fn GetStartupInfoW();
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
    // pub fn GetSystemTime();
    // pub fn GetSystemTimeAdjustment();
    // pub fn GetSystemTimeAsFileTime();
    // pub fn GetSystemTimePreciseAsFileTime();
    // pub fn GetSystemTimes();
    // pub fn GetSystemWindowsDirectoryA();
    // pub fn GetSystemWindowsDirectoryW();
    // pub fn GetSystemWow64DirectoryA();
    // pub fn GetSystemWow64DirectoryW();
    // pub fn GetTapeParameters();
    // pub fn GetTapePosition();
    // pub fn GetTapeStatus();
    // pub fn GetTempFileNameA();
    // pub fn GetTempFileNameW();
    // pub fn GetTempPathA();
    // pub fn GetTempPathW();
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
    // pub fn GetThreadTimes();
    // pub fn GetThreadUILanguage();
    // pub fn GetTickCount();
    // pub fn GetTickCount64();
    // pub fn GetTimeFormatA();
    // pub fn GetTimeFormatEx();
    // pub fn GetTimeFormatW();
    // pub fn GetTimeZoneInformation();
    // pub fn GetTimeZoneInformationForYear();
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
    // pub fn GetVolumeInformationByHandleW();
    // pub fn GetVolumeInformationW();
    // pub fn GetVolumeNameForVolumeMountPointA();
    // pub fn GetVolumeNameForVolumeMountPointW();
    // pub fn GetVolumePathNameA();
    // pub fn GetVolumePathNameW();
    // pub fn GetVolumePathNamesForVolumeNameA();
    // pub fn GetVolumePathNamesForVolumeNameW();
    // pub fn GetWindowsDirectoryA();
    // pub fn GetWindowsDirectoryW();
    // pub fn GetWriteWatch();
    // #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    // pub fn GetXStateFeaturesMask();
    // pub fn GlobalAddAtomA();
    // pub fn GlobalAddAtomExA();
    // pub fn GlobalAddAtomExW();
    // pub fn GlobalAddAtomW();
    // pub fn GlobalAlloc();
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
    // pub fn GlobalLock();
    // pub fn GlobalMemoryStatus();
    // pub fn GlobalMemoryStatusEx();
    // pub fn GlobalReAlloc();
    // pub fn GlobalSize();
    // pub fn GlobalUnWire();
    // pub fn GlobalUnfix();
    // pub fn GlobalUnlock();
    // pub fn GlobalWire();
    // pub fn Heap32First();
    // pub fn Heap32ListFirst();
    // pub fn Heap32ListNext();
    // pub fn Heap32Next();
    // pub fn HeapAlloc();
    // pub fn HeapCompact();
    // pub fn HeapCreate();
    // pub fn HeapDestroy();
    // pub fn HeapFree();
    // pub fn HeapLock();
    // pub fn HeapQueryInformation();
    // pub fn HeapReAlloc();
    // pub fn HeapSetInformation();
    // pub fn HeapSize();
    // pub fn HeapSummary();
    // pub fn HeapUnlock();
    // pub fn HeapValidate();
    // pub fn HeapWalk();
    // pub fn InitAtomTable();
    // pub fn InitOnceBeginInitialize();
    // pub fn InitOnceComplete();
    // pub fn InitOnceExecuteOnce();
    // pub fn InitOnceInitialize();
    // pub fn InitializeConditionVariable();
    // pub fn InitializeContext();
    // pub fn InitializeCriticalSection();
    // pub fn InitializeCriticalSectionAndSpinCount();
    // pub fn InitializeCriticalSectionEx();
    // pub fn InitializeProcThreadAttributeList();
    // pub fn InitializeSListHead();
    // pub fn InitializeSRWLock();
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
    // pub fn K32EnumProcesses();
    // pub fn K32GetDeviceDriverBaseNameA();
    // pub fn K32GetDeviceDriverBaseNameW();
    // pub fn K32GetDeviceDriverFileNameA();
    // pub fn K32GetDeviceDriverFileNameW();
    // pub fn K32GetMappedFileNameA();
    // pub fn K32GetMappedFileNameW();
    // pub fn K32GetModuleBaseNameA();
    // pub fn K32GetModuleBaseNameW();
    // pub fn K32GetModuleFileNameExA();
    // pub fn K32GetModuleFileNameExW();
    // pub fn K32GetModuleInformation();
    // pub fn K32GetPerformanceInfo();
    // pub fn K32GetProcessImageFileNameA();
    // pub fn K32GetProcessImageFileNameW();
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
    // pub fn LeaveCriticalSection();
    // pub fn LeaveCriticalSectionWhenCallbackReturns();
    // pub fn LoadAppInitDlls();
    // pub fn LoadLibraryA();
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
    // pub fn LocalFileTimeToFileTime();
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
    // pub fn LockFile();
    // pub fn LockFileEx();
    // pub fn LockResource();
    // pub fn MapUserPhysicalPages();
    // pub fn MapUserPhysicalPagesScatter();
    // pub fn MapViewOfFile();
    // pub fn MapViewOfFileEx();
    // pub fn MapViewOfFileExNuma();
    // pub fn MapViewOfFileFromApp();
    // pub fn Module32First();
    // pub fn Module32FirstW();
    // pub fn Module32Next();
    // pub fn Module32NextW();
    // pub fn MoveFileA();
    // pub fn MoveFileExA();
    // pub fn MoveFileExW();
    // pub fn MoveFileTransactedA();
    // pub fn MoveFileTransactedW();
    // pub fn MoveFileW();
    // pub fn MoveFileWithProgressA();
    // pub fn MoveFileWithProgressW();
    // pub fn MulDiv();
    // pub fn MultiByteToWideChar();
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
    // pub fn OpenFileMappingA();
    // pub fn OpenFileMappingW();
    // pub fn OpenJobObjectA();
    // pub fn OpenJobObjectW();
    // pub fn OpenMutexA();
    // pub fn OpenMutexW();
    // pub fn OpenPackageInfoByFullName();
    // pub fn OpenPrivateNamespaceA();
    // pub fn OpenPrivateNamespaceW();
    pub fn OpenProcess(dwDesiredAccess: DWORD, bInheritHandle: BOOL, dwProcessId: DWORD) -> HANDLE;
    // pub fn OpenSemaphoreA();
    // pub fn OpenSemaphoreW();
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
    // pub fn QueryDosDeviceW();
    // pub fn QueryFullProcessImageNameA();
    // pub fn QueryFullProcessImageNameW();
    // pub fn QueryIdleProcessorCycleTime();
    // pub fn QueryIdleProcessorCycleTimeEx();
    // pub fn QueryInformationJobObject();
    // pub fn QueryMemoryResourceNotification();
    // pub fn QueryPerformanceCounter();
    // pub fn QueryPerformanceFrequency();
    // pub fn QueryProcessAffinityUpdateMode();
    // pub fn QueryProcessCycleTime();
    // pub fn QueryProtectedPolicy();
    // pub fn QueryThreadCycleTime();
    // pub fn QueryThreadProfiling();
    // pub fn QueryThreadpoolStackInformation();
    // #[cfg(target_arch = "x86_64")]
    // pub fn QueryUmsThreadInformation();
    // pub fn QueryUnbiasedInterruptTime();
    // pub fn QueueUserAPC();
    // pub fn QueueUserWorkItem();
    // pub fn RaiseException();
    // pub fn RaiseFailFastException();
    // pub fn ReOpenFile();
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
    );
    // pub fn ReadFile();
    // pub fn ReadFileEx();
    // pub fn ReadFileScatter();
    pub fn ReadProcessMemory(
        hProcess: HANDLE, lpBaseAddress: LPCVOID, lpBuffer: LPVOID, nSize: SIZE_T,
        lpNumberOfBytesRead: *mut SIZE_T,
    ) -> BOOL;
    // pub fn ReadThreadProfilingData();
    // pub fn RegisterApplicationRecoveryCallback();
    // pub fn RegisterApplicationRestart();
    // pub fn RegisterBadMemoryNotification();
    // pub fn RegisterWaitForInputIdle();
    // pub fn RegisterWaitForSingleObject();
    // pub fn RegisterWaitForSingleObjectEx();
    // pub fn RegisterWaitUntilOOBECompleted();
    // pub fn ReleaseActCtx();
    // pub fn ReleaseMutex();
    // pub fn ReleaseMutexWhenCallbackReturns();
    // pub fn ReleaseSRWLockExclusive();
    // pub fn ReleaseSRWLockShared();
    // pub fn ReleaseSemaphore();
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
    // pub fn RtlCaptureContext();
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
    // pub fn SetCriticalSectionSpinCount();
    // pub fn SetCurrentConsoleFontEx();
    pub fn SetCurrentDirectoryA(lpPathName: LPCSTR) -> BOOL;
    pub fn SetCurrentDirectoryW(lpPathName: LPCWSTR) -> BOOL;
    // pub fn SetDefaultCommConfigA();
    // pub fn SetDefaultCommConfigW();
    // pub fn SetDefaultDllDirectories();
    // pub fn SetDllDirectoryA();
    // pub fn SetDllDirectoryW();
    // pub fn SetDynamicTimeZoneInformation();
    // pub fn SetEndOfFile();
    // pub fn SetEnvironmentStringsA();
    // pub fn SetEnvironmentStringsW();
    // pub fn SetEnvironmentVariableA();
    // pub fn SetEnvironmentVariableW();
    // pub fn SetErrorMode();
    // pub fn SetEvent();
    // pub fn SetEventWhenCallbackReturns();
    // pub fn SetFileApisToANSI();
    // pub fn SetFileApisToOEM();
    // pub fn SetFileAttributesA();
    // pub fn SetFileAttributesTransactedA();
    // pub fn SetFileAttributesTransactedW();
    // pub fn SetFileAttributesW();
    // pub fn SetFileBandwidthReservation();
    // pub fn SetFileCompletionNotificationModes();
    // pub fn SetFileInformationByHandle();
    // pub fn SetFileIoOverlappedRange();
    // pub fn SetFilePointer();
    // pub fn SetFilePointerEx();
    // pub fn SetFileShortNameA();
    // pub fn SetFileShortNameW();
    // pub fn SetFileTime();
    // pub fn SetFileValidData();
    // pub fn SetFirmwareEnvironmentVariableA();
    // pub fn SetFirmwareEnvironmentVariableExA();
    // pub fn SetFirmwareEnvironmentVariableExW();
    // pub fn SetFirmwareEnvironmentVariableW();
    // pub fn SetHandleCount();
    // pub fn SetHandleInformation();
    // pub fn SetInformationJobObject();
    // pub fn SetLastError();
    // pub fn SetLocalPrimaryComputerNameA();
    // pub fn SetLocalPrimaryComputerNameW();
    // pub fn SetLocalTime();
    // pub fn SetLocaleInfoA();
    // pub fn SetLocaleInfoW();
    // pub fn SetMailslotInfo();
    // pub fn SetMessageWaitingIndicator();
    // pub fn SetNamedPipeAttribute();
    // pub fn SetNamedPipeHandleState();
    // pub fn SetPriorityClass();
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
    // pub fn SetSystemTime();
    // pub fn SetSystemTimeAdjustment();
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
    // pub fn SetTimeZoneInformation();
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
    // pub fn Sleep();
    // pub fn SleepConditionVariableCS();
    // pub fn SleepConditionVariableSRW();
    // pub fn SleepEx();
    // pub fn StartThreadpoolIo();
    // pub fn SubmitThreadpoolWork();
    // pub fn SuspendThread();
    // pub fn SwitchToFiber();
    // pub fn SwitchToThread();
    // pub fn SystemTimeToFileTime();
    // pub fn SystemTimeToTzSpecificLocalTime();
    // pub fn SystemTimeToTzSpecificLocalTimeEx();
    // pub fn TerminateJobObject();
    // pub fn TerminateProcess();
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
    // pub fn TryAcquireSRWLockExclusive();
    // pub fn TryAcquireSRWLockShared();
    // pub fn TryEnterCriticalSection();
    // pub fn TrySubmitThreadpoolCallback();
    // pub fn TzSpecificLocalTimeToSystemTime();
    // pub fn TzSpecificLocalTimeToSystemTimeEx();
    // #[cfg(target_arch = "x86_64")]
    // pub fn UmsThreadYield();
    // pub fn UnhandledExceptionFilter();
    // pub fn UnlockFile();
    // pub fn UnlockFileEx();
    // pub fn UnmapViewOfFile();
    // pub fn UnmapViewOfFileEx();
    // pub fn UnregisterApplicationRecoveryCallback();
    // pub fn UnregisterApplicationRestart();
    // pub fn UnregisterBadMemoryNotification();
    // pub fn UnregisterWait();
    // pub fn UnregisterWaitEx();
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
    // pub fn VirtualAlloc();
    // pub fn VirtualAllocEx();
    // pub fn VirtualAllocExNuma();
    // pub fn VirtualFree();
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
    // pub fn WideCharToMultiByte();
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
    // pub fn WriteFile();
    // pub fn WriteFileEx();
    // pub fn WriteFileGather();
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
    // pub fn lstrlenA();
    // pub fn lstrlenW();
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

// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! This module defines the 32-Bit Windows Base APIs
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SECURITY_ATTRIBUTES {
    pub nLength: ::DWORD,
    pub lpSecurityDescriptor: ::LPVOID,
    pub bInheritHandle: ::BOOL,
}
pub type PSECURITY_ATTRIBUTES = *mut SECURITY_ATTRIBUTES;
pub type LPSECURITY_ATTRIBUTES = *mut SECURITY_ATTRIBUTES;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct OVERLAPPED {
    pub Internal: ::ULONG_PTR,
    pub InternalHigh: ::ULONG_PTR,
    pub Offset: ::DWORD,
    pub OffsetHigh: ::DWORD,
    pub hEvent: ::HANDLE,
}
UNION!(OVERLAPPED, Offset, Pointer, Pointer_mut, ::PVOID);
pub type LPOVERLAPPED = *mut OVERLAPPED;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct OVERLAPPED_ENTRY {
    pub lpCompletionKey: ::ULONG_PTR,
    pub lpOverlapped: LPOVERLAPPED,
    pub Internal: ::ULONG_PTR,
    pub dwNumberOfBytesTransferred: ::DWORD,
}
pub type LPOVERLAPPED_ENTRY = *mut OVERLAPPED_ENTRY;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SYSTEMTIME {
    pub wYear: ::WORD,
    pub wMonth: ::WORD,
    pub wDayOfWeek: ::WORD,
    pub wDay: ::WORD,
    pub wHour: ::WORD,
    pub wMinute: ::WORD,
    pub wSecond: ::WORD,
    pub wMilliseconds: ::WORD,
}
pub type PSYSTEMTIME = *mut SYSTEMTIME;
pub type LPSYSTEMTIME = *mut SYSTEMTIME;
#[repr(C)] #[derive(Copy)]
pub struct WIN32_FIND_DATAA {
    pub dwFileAttributes: ::DWORD,
    pub ftCreationTime: ::FILETIME,
    pub ftLastAccessTime: ::FILETIME,
    pub ftLastWriteTime: ::FILETIME,
    pub nFileSizeHigh: ::DWORD,
    pub nFileSizeLow: ::DWORD,
    pub dwReserved0: ::DWORD,
    pub dwReserved1: ::DWORD,
    pub cFileName: [::CHAR; ::MAX_PATH],
    pub cAlternateFileName: [::CHAR; 14],
}
impl Clone for WIN32_FIND_DATAA { fn clone(&self) -> WIN32_FIND_DATAA { *self } }
pub type PWIN32_FIND_DATAA = *mut WIN32_FIND_DATAA;
pub type LPWIN32_FIND_DATAA = *mut WIN32_FIND_DATAA;
#[repr(C)] #[derive(Copy)]
pub struct WIN32_FIND_DATAW {
    pub dwFileAttributes: ::DWORD,
    pub ftCreationTime: ::FILETIME,
    pub ftLastAccessTime: ::FILETIME,
    pub ftLastWriteTime: ::FILETIME,
    pub nFileSizeHigh: ::DWORD,
    pub nFileSizeLow: ::DWORD,
    pub dwReserved0: ::DWORD,
    pub dwReserved1: ::DWORD,
    pub cFileName: [::WCHAR; ::MAX_PATH],
    pub cAlternateFileName: [::WCHAR; 14],
}
impl Clone for WIN32_FIND_DATAW { fn clone(&self) -> WIN32_FIND_DATAW { *self } }
pub type PWIN32_FIND_DATAW = *mut WIN32_FIND_DATAW;
pub type LPWIN32_FIND_DATAW = *mut WIN32_FIND_DATAW;
#[repr(i32)] #[derive(Clone, Copy, Debug)] #[allow(unused_qualifications)]
pub enum FINDEX_INFO_LEVELS {
    FindExInfoStandard,
    FindExInfoBasic,
    FindExInfoMaxInfoLevel,
}
pub use self::FINDEX_INFO_LEVELS::*;
pub const FIND_FIRST_EX_CASE_SENSITIVE: ::DWORD = 0x00000001;
pub const FIND_FIRST_EX_LARGE_FETCH: ::DWORD = 0x00000002;
#[repr(i32)] #[derive(Clone, Copy, Debug)] #[allow(unused_qualifications)]
pub enum FINDEX_SEARCH_OPS {
    FindExSearchNameMatch,
    FindExSearchLimitToDirectories,
    FindExSearchLimitToDevices,
    FindExSearchMaxSearchOp,
}
pub use self::FINDEX_SEARCH_OPS::*;
#[repr(i32)] #[derive(Clone, Copy, Debug)] #[allow(unused_qualifications)]
pub enum GET_FILEEX_INFO_LEVELS {
    GetFileExInfoStandard,
    GetFileExMaxInfoLevel,
}
pub use self::GET_FILEEX_INFO_LEVELS::*;
#[repr(i32)] #[derive(Clone, Copy, Debug)] #[allow(unused_qualifications)]
pub enum FILE_INFO_BY_HANDLE_CLASS {
    FileBasicInfo,
    FileStandardInfo,
    FileNameInfo,
    FileRenameInfo,
    FileDispositionInfo,
    FileAllocationInfo,
    FileEndOfFileInfo,
    FileStreamInfo,
    FileCompressionInfo,
    FileAttributeTagInfo,
    FileIdBothDirectoryInfo,
    FileIdBothDirectoryRestartInfo,
    FileIoPriorityHintInfo,
    FileRemoteProtocolInfo,
    FileFullDirectoryInfo,
    FileFullDirectoryRestartInfo,
    FileStorageInfo,
    FileAlignmentInfo,
    FileIdInfo,
    FileIdExtdDirectoryInfo,
    FileIdExtdDirectoryRestartInfo,
    MaximumFileInfoByHandleClass,
}
pub use self::FILE_INFO_BY_HANDLE_CLASS::*;
pub type PFILE_INFO_BY_HANDLE_CLASS = *mut FILE_INFO_BY_HANDLE_CLASS;
pub type CRITICAL_SECTION = ::RTL_CRITICAL_SECTION;
pub type PCRITICAL_SECTION = ::PRTL_CRITICAL_SECTION;
pub type LPCRITICAL_SECTION = ::PRTL_CRITICAL_SECTION;
pub type CRITICAL_SECTION_DEBUG = ::RTL_CRITICAL_SECTION_DEBUG;
pub type PCRITICAL_SECTION_DEBUG = ::PRTL_CRITICAL_SECTION_DEBUG;
pub type LPCRITICAL_SECTION_DEBUG = ::PRTL_CRITICAL_SECTION_DEBUG;
pub type LPOVERLAPPED_COMPLETION_ROUTINE = Option<unsafe extern "system" fn(
    dwErrorCode: ::DWORD, dwNumberOfBytesTransfered: ::DWORD, lpOverlapped: LPOVERLAPPED,
)>;
pub const LOCKFILE_FAIL_IMMEDIATELY: ::DWORD = 0x00000001;
pub const LOCKFILE_EXCLUSIVE_LOCK: ::DWORD = 0x00000002;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct PROCESS_HEAP_ENTRY_Block {
    pub hMem: ::HANDLE,
    pub dwReserved: [::DWORD; 3],
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct PROCESS_HEAP_ENTRY_Region {
    pub dwCommittedSize: ::DWORD,
    pub dwUnCommittedSize: ::DWORD,
    pub lpFirstBlock: ::LPVOID,
    pub lpLastBlock: ::LPVOID,
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct PROCESS_HEAP_ENTRY {
    pub lpData: ::PVOID,
    pub cbData: ::DWORD,
    pub cbOverhead: ::BYTE,
    pub iRegionIndex: ::BYTE,
    pub wFlags: ::WORD,
    pub Region: PROCESS_HEAP_ENTRY_Region,
}
UNION!(PROCESS_HEAP_ENTRY, Region, Block, Block_mut, PROCESS_HEAP_ENTRY_Block);
pub type LPPROCESS_HEAP_ENTRY = *mut PROCESS_HEAP_ENTRY;
pub type PPROCESS_HEAP_ENTRY = *mut PROCESS_HEAP_ENTRY;
pub const PROCESS_HEAP_REGION: ::WORD = 0x0001;
pub const PROCESS_HEAP_UNCOMMITTED_RANGE: ::WORD = 0x0002;
pub const PROCESS_HEAP_ENTRY_BUSY: ::WORD = 0x0004;
pub const PROCESS_HEAP_SEG_ALLOC: ::WORD = 0x0008;
pub const PROCESS_HEAP_ENTRY_MOVEABLE: ::WORD = 0x0010;
pub const PROCESS_HEAP_ENTRY_DDESHARE: ::WORD = 0x0020;
pub type PTHREAD_START_ROUTINE = Option<unsafe extern "system" fn(
    lpThreadParameter: ::LPVOID,
) -> ::DWORD>;
pub type LPTHREAD_START_ROUTINE = PTHREAD_START_ROUTINE;
pub type LPCONTEXT = ::PCONTEXT;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct REASON_CONTEXT_Detailed {
    pub LocalizedReasonModule: ::HMODULE,
    pub LocalizedReasonId: ::ULONG,
    pub ReasonStringCount: ::ULONG,
    pub ReasonStrings: *mut ::LPWSTR,
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct REASON_CONTEXT {
    pub Version: ::ULONG,
    pub Flags: ::DWORD,
    pub Reason: REASON_CONTEXT_Detailed,
}
UNION!(REASON_CONTEXT, Reason, SimpleReasonString, SimpleReasonString_mut, ::LPWSTR);
pub type PREASON_CONTEXT = *mut REASON_CONTEXT;
pub const EXCEPTION_DEBUG_EVENT: ::DWORD = 1;
pub const CREATE_THREAD_DEBUG_EVENT: ::DWORD = 2;
pub const CREATE_PROCESS_DEBUG_EVENT: ::DWORD = 3;
pub const EXIT_THREAD_DEBUG_EVENT: ::DWORD = 4;
pub const EXIT_PROCESS_DEBUG_EVENT: ::DWORD = 5;
pub const LOAD_DLL_DEBUG_EVENT: ::DWORD = 6;
pub const UNLOAD_DLL_DEBUG_EVENT: ::DWORD = 7;
pub const OUTPUT_DEBUG_STRING_EVENT: ::DWORD = 8;
pub const RIP_EVENT: ::DWORD = 9;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct EXCEPTION_DEBUG_INFO {
    pub ExceptionRecord: ::EXCEPTION_RECORD,
    pub dwFirstChance: ::DWORD,
}
pub type LPEXCEPTION_DEBUG_INFO = *mut EXCEPTION_DEBUG_INFO;
#[repr(C)] #[derive(Copy)]
pub struct CREATE_THREAD_DEBUG_INFO {
    pub hThread: ::HANDLE,
    pub lpThreadLocalBase: ::LPVOID,
    pub lpStartAddress: LPTHREAD_START_ROUTINE,
}
impl Clone for CREATE_THREAD_DEBUG_INFO { fn clone(&self) -> CREATE_THREAD_DEBUG_INFO { *self } }
pub type LPCREATE_THREAD_DEBUG_INFO = *mut CREATE_THREAD_DEBUG_INFO;
#[repr(C)] #[derive(Copy)]
pub struct CREATE_PROCESS_DEBUG_INFO {
    pub hFile: ::HANDLE,
    pub hProcess: ::HANDLE,
    pub hThread: ::HANDLE,
    pub lpBaseOfImage: ::LPVOID,
    pub dwDebugInfoFileOffset: ::DWORD,
    pub nDebugInfoSize: ::DWORD,
    pub lpThreadLocalBase: ::LPVOID,
    pub lpStartAddress: LPTHREAD_START_ROUTINE,
    pub lpImageName: ::LPVOID,
    pub fUnicode: ::WORD,
}
impl Clone for CREATE_PROCESS_DEBUG_INFO { fn clone(&self) -> CREATE_PROCESS_DEBUG_INFO { *self } }
pub type LPCREATE_PROCESS_DEBUG_INFO = *mut CREATE_PROCESS_DEBUG_INFO;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct EXIT_THREAD_DEBUG_INFO {
    pub dwExitCode: ::DWORD,
}
pub type LPEXIT_THREAD_DEBUG_INFO = *mut EXIT_THREAD_DEBUG_INFO;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct EXIT_PROCESS_DEBUG_INFO {
    pub dwExitCode: ::DWORD,
}
pub type LPEXIT_PROCESS_DEBUG_INFO = *mut EXIT_PROCESS_DEBUG_INFO;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct LOAD_DLL_DEBUG_INFO {
    pub hFile: ::HANDLE,
    pub lpBaseOfDll: ::LPVOID,
    pub dwDebugInfoFileOffset: ::DWORD,
    pub nDebugInfoSize: ::DWORD,
    pub lpImageName: ::LPVOID,
    pub fUnicode: ::WORD,
}
pub type LPLOAD_DLL_DEBUG_INFO = *mut LOAD_DLL_DEBUG_INFO;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct UNLOAD_DLL_DEBUG_INFO {
    pub lpBaseOfDll: ::LPVOID,
}
pub type LPUNLOAD_DLL_DEBUG_INFO = *mut UNLOAD_DLL_DEBUG_INFO;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct OUTPUT_DEBUG_STRING_INFO {
    pub lpDebugStringData: ::LPSTR,
    pub fUnicode: ::WORD,
    pub nDebugStringLength: ::WORD,
}
pub type LPOUTPUT_DEBUG_STRING_INFO = *mut OUTPUT_DEBUG_STRING_INFO;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct RIP_INFO {
    pub dwError: ::DWORD,
    pub dwType: ::DWORD,
}
pub type LPRIP_INFO = *mut RIP_INFO;
#[repr(C)] #[derive(Copy)]
pub struct DEBUG_EVENT {
    pub dwDebugEventCode: ::DWORD,
    pub dwProcessId: ::DWORD,
    pub dwThreadId: ::DWORD,
    #[cfg(target_arch="x86")]
    pub u: [u8; 84],
    #[cfg(target_arch="x86_64")]
    pub u: [u8; 160],
}
impl Clone for DEBUG_EVENT { fn clone(&self) -> DEBUG_EVENT { *self } }
UNION!(DEBUG_EVENT, u, Exception, Exception_mut, EXCEPTION_DEBUG_INFO);
UNION!(DEBUG_EVENT, u, CreateThread, CreateThread_mut, CREATE_THREAD_DEBUG_INFO);
UNION!(DEBUG_EVENT, u, CreateProcessInfo, CreateProcessInfo_mut, CREATE_PROCESS_DEBUG_INFO);
UNION!(DEBUG_EVENT, u, ExitThread, ExitThread_mut, EXIT_THREAD_DEBUG_INFO);
UNION!(DEBUG_EVENT, u, ExitProcess, ExitProcess_mut, EXIT_PROCESS_DEBUG_INFO);
UNION!(DEBUG_EVENT, u, LoadDll, LoadDll_mut, LOAD_DLL_DEBUG_INFO);
UNION!(DEBUG_EVENT, u, UnloadDll, UnloadDll_mut, UNLOAD_DLL_DEBUG_INFO);
UNION!(DEBUG_EVENT, u, DebugString, DebugString_mut, OUTPUT_DEBUG_STRING_INFO);
UNION!(DEBUG_EVENT, u, RipInfo, RipInfo_mut, RIP_INFO);
pub type LPDEBUG_EVENT = *mut DEBUG_EVENT;

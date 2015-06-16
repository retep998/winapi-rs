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

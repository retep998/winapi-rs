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
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum FINDEX_INFO_LEVELS {
    FindExInfoStandard = 0,
    FindExInfoBasic = 1,
    FindExInfoMaxInfoLevel = 2,
}
pub const FIND_FIRST_EX_CASE_SENSITIVE: ::DWORD = 0x00000001;
pub const FIND_FIRST_EX_LARGE_FETCH: ::DWORD = 0x00000002;
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum FINDEX_SEARCH_OPS {
    FindExSearchNameMatch = 0,
    FindExSearchLimitToDirectories = 1,
    FindExSearchLimitToDevices = 2,
    FindExSearchMaxSearchOp = 3,
}
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum GET_FILEEX_INFO_LEVELS {
    GetFileExInfoStandard = 0,
    GetFileExMaxInfoLevel = 1,
}
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum FILE_INFO_BY_HANDLE_CLASS {
    FileBasicInfo = 0,
    FileStandardInfo = 1,
    FileNameInfo = 2,
    FileRenameInfo = 3,
    FileDispositionInfo = 4,
    FileAllocationInfo = 5,
    FileEndOfFileInfo = 6,
    FileStreamInfo = 7,
    FileCompressionInfo = 8,
    FileAttributeTagInfo = 9,
    FileIdBothDirectoryInfo = 10,
    FileIdBothDirectoryRestartInfo = 11,
    FileIoPriorityHintInfo = 12,
    FileRemoteProtocolInfo = 13,
    FileFullDirectoryInfo = 14,
    FileFullDirectoryRestartInfo = 15,
    FileStorageInfo = 16,
    FileAlignmentInfo = 17,
    FileIdInfo = 18,
    FileIdExtdDirectoryInfo = 19,
    FileIdExtdDirectoryRestartInfo = 20,
    MaximumFileInfoByHandleClass = 21,
}
pub type PFILE_INFO_BY_HANDLE_CLASS = *mut FILE_INFO_BY_HANDLE_CLASS;
//206
pub type LPOVERLAPPED_COMPLETION_ROUTINE = Option<unsafe extern "system" fn(
    dwErrorCode: ::DWORD, dwNumberOfBytesTransfered: ::DWORD, lpOverlapped: LPOVERLAPPED,
)>;
pub const LOCKFILE_FAIL_IMMEDIATELY: ::DWORD = 0x00000001;
pub const LOCKFILE_EXCLUSIVE_LOCK: ::DWORD = 0x00000002;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct PROCESS_HEAP_ENTRY {
    pub lpData: ::PVOID,
    pub cbData: ::DWORD,
    pub cbOverhead: ::BYTE,
    pub iRegionIndex: ::BYTE,
    pub wFlags: ::WORD,
    pub dwCommittedSize: ::DWORD,
    pub dwUnCommittedSize: ::DWORD,
    pub lpFirstBlock: ::LPVOID,
    pub lpLastBlock: ::LPVOID,
}
pub type LPPROCESS_HEAP_ENTRY = *mut PROCESS_HEAP_ENTRY;
pub type PPROCESS_HEAP_ENTRY = *mut PROCESS_HEAP_ENTRY;
pub const PROCESS_HEAP_REGION: ::WORD = 0x0001;
pub const PROCESS_HEAP_UNCOMMITTED_RANGE: ::WORD = 0x0002;
pub const PROCESS_HEAP_ENTRY_BUSY: ::WORD = 0x0004;
pub const PROCESS_HEAP_SEG_ALLOC: ::WORD = 0x0008;
pub const PROCESS_HEAP_ENTRY_MOVEABLE: ::WORD = 0x0010;
pub const PROCESS_HEAP_ENTRY_DDESHARE: ::WORD = 0x0020;

// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! ApiSet Contract for api-ms-win-core-file-l1
pub const CREATE_NEW: ::DWORD = 1;
pub const CREATE_ALWAYS: ::DWORD = 2;
pub const OPEN_EXISTING: ::DWORD = 3;
pub const OPEN_ALWAYS: ::DWORD = 4;
pub const TRUNCATE_EXISTING: ::DWORD = 5;
pub const INVALID_FILE_SIZE: ::DWORD = 0xFFFFFFFF;
pub const INVALID_SET_FILE_POINTER: ::DWORD = 0xFFFFFFFF;
pub const INVALID_FILE_ATTRIBUTES: ::DWORD = 0xFFFFFFFF;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct WIN32_FILE_ATTRIBUTE_DATA {
    pub dwFileAttributes: ::DWORD,
    pub ftCreationTime: ::FILETIME,
    pub ftLastAccessTime: ::FILETIME,
    pub ftLastWriteTime: ::FILETIME,
    pub nFileSizeHigh: ::DWORD,
    pub nFileSizeLow: ::DWORD,
}
pub type LPWIN32_FILE_ATTRIBUTE_DATA = *mut WIN32_FILE_ATTRIBUTE_DATA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct BY_HANDLE_FILE_INFORMATION {
    pub dwFileAttributes: ::DWORD,
    pub ftCreationTime: ::FILETIME,
    pub ftLastAccessTime: ::FILETIME,
    pub ftLastWriteTime: ::FILETIME,
    pub dwVolumeSerialNumber: ::DWORD,
    pub nFileSizeHigh: ::DWORD,
    pub nFileSizeLow: ::DWORD,
    pub nNumberOfLinks: ::DWORD,
    pub nFileIndexHigh: ::DWORD,
    pub nFileIndexLow: ::DWORD,
}
pub type PBY_HANDLE_FILE_INFORMATION = *mut BY_HANDLE_FILE_INFORMATION;
pub type LPBY_HANDLE_FILE_INFORMATION = *mut BY_HANDLE_FILE_INFORMATION;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct CREATEFILE2_EXTENDED_PARAMETERS {
    pub dwSize: ::DWORD,
    pub dwFileAttributes: ::DWORD,
    pub dwFileFlags: ::DWORD,
    pub dwSecurityQosFlags: ::DWORD,
    pub lpSecurityAttributes: ::LPSECURITY_ATTRIBUTES,
    pub hTemplateFile: ::HANDLE,
}
pub type PCREATEFILE2_EXTENDED_PARAMETERS = *mut CREATEFILE2_EXTENDED_PARAMETERS;
pub type LPCREATEFILE2_EXTENDED_PARAMETERS = *mut CREATEFILE2_EXTENDED_PARAMETERS;
ENUM!{enum PRIORITY_HINT {
    IoPriorityHintVeryLow = 0,
    IoPriorityHintLow = 1,
    IoPriorityHintNormal = 2,
    MaximumIoPriorityHintType = 3,
}}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct FILE_BASIC_INFO {
    pub CreationTime: ::LARGE_INTEGER,
    pub LastAccessTime: ::LARGE_INTEGER,
    pub LastWriteTime: ::LARGE_INTEGER,
    pub ChangeTime: ::LARGE_INTEGER,
    pub FileAttributes: ::DWORD,
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct FILE_STANDARD_INFO {
    pub AllocationSize: ::LARGE_INTEGER,
    pub EndOfFile: ::LARGE_INTEGER,
    pub NumberOfLinks: ::DWORD,
    pub DeletePending: ::BOOLEAN,
    pub Directory: ::BOOLEAN,
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct FILE_NAME_INFO {
    pub FileNameLength: ::DWORD,
    pub FileName: [::WCHAR; 0],
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct FILE_RENAME_INFO {
    pub ReplaceIfExists: ::BOOL,
    pub RootDirectory: ::HANDLE,
    pub FileNameLength: ::DWORD,
    pub FileName: [::WCHAR; 0],
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct FILE_DISPOSITION_INFO {
    pub DeleteFile: ::BOOL,
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct FILE_ALLOCATION_INFO {
    pub AllocationSize: ::LARGE_INTEGER,
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct FILE_END_OF_FILE_INFO {
    pub EndOfFile: ::LARGE_INTEGER,
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct FILE_STREAM_INFO {
    pub NextEntryOffset: ::DWORD,
    pub StreamNameLength: ::DWORD,
    pub StreamSize: ::DWORD,
    pub StreamAllocationSize: ::DWORD,
    pub StreamName: [::WCHAR; 0],
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct FILE_COMPRESSION_INFO {
    pub CompressedFileSize: ::LARGE_INTEGER,
    pub CompressionFormat: ::WORD,
    pub CompressionUnitShift: ::UCHAR,
    pub ChunkShift: ::UCHAR,
    pub ClusterShift: ::UCHAR,
    pub Reserved: [::UCHAR; 3],
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct FILE_ATTRIBUTE_TAG_INFO {
    pub NextEntryOffset: ::DWORD,
    pub ReparseTag: ::DWORD,
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct FILE_ID_BOTH_DIR_INFO {
    pub NextEntryOffset: ::DWORD,
    pub FileIndex: ::DWORD,
    pub CreationTime: ::LARGE_INTEGER,
    pub LastAccessTime: ::LARGE_INTEGER,
    pub LastWriteTime: ::LARGE_INTEGER,
    pub ChangeTime: ::LARGE_INTEGER,
    pub EndOfFile: ::LARGE_INTEGER,
    pub AllocationSize: ::LARGE_INTEGER,
    pub FileAttributes: ::DWORD,
    pub FileNameLength: ::DWORD,
    pub EaSize: ::DWORD,
    pub ShortNameLength: ::CCHAR,
    pub ShortName: [::WCHAR; 12],
    pub FileId: ::LARGE_INTEGER,
    pub FileName: [::WCHAR; 0],
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct FILE_IO_PRIORITY_HINT_INFO {
    pub PriorityHint: ::PRIORITY_HINT,
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct FILE_FULL_DIR_INFO {
    pub NextEntryOffset: ::ULONG,
    pub FileIndex: ::ULONG,
    pub CreationTime: ::LARGE_INTEGER,
    pub LastAccessTime: ::LARGE_INTEGER,
    pub LastWriteTime: ::LARGE_INTEGER,
    pub ChangeTime: ::LARGE_INTEGER,
    pub EndOfFile: ::LARGE_INTEGER,
    pub AllocationSize: ::LARGE_INTEGER,
    pub FileAttributes: ::ULONG,
    pub FileNameLength: ::ULONG,
    pub EaSize: ::ULONG,
    pub FileName: [::WCHAR; 0],
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct FILE_STORAGE_INFO {
    pub LogicalBytesPerSector: ::ULONG,
    pub PhysicalBytesPerSectorForAtomicity: ::ULONG,
    pub PhysicalBytesPerSectorForPerformance: ::ULONG,
    pub FileSystemEffectivePhysicalBytesPerSectorForAtomicity: ::ULONG,
    pub Flags: ::ULONG,
    pub ByteOffsetForSectorAlignment: ::ULONG,
    pub ByteOffsetForPartitionAlignment: ::ULONG,
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct FILE_ALIGNMENT_INFO {
    pub AlignmentRequirement: ::ULONG,
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct FILE_ID_INFO {
    pub VolumeSerialNumber: ::ULONGLONG,
    pub FileId: ::FILE_ID_128,
}

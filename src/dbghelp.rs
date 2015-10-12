// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! DbgHelp include file
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct LOADED_IMAGE {
    pub ModuleName: ::PSTR,
    pub hFile: ::HANDLE,
    pub MappedAddress: ::PUCHAR,
    #[cfg(target_arch = "x86_64")]
    pub FileHeader: ::PIMAGE_NT_HEADERS64,
    #[cfg(not(target_arch = "x86_64"))]
    pub FileHeader: ::PIMAGE_NT_HEADERS32,
    pub LastRvaSection: ::PIMAGE_SECTION_HEADER,
    pub NumberOfSections: ::ULONG,
    pub Sections: ::PIMAGE_SECTION_HEADER,
    pub Characteristics: ::ULONG,
    pub fSystemImage: ::BOOLEAN,
    pub fDOSImage: ::BOOLEAN,
    pub fReadOnly: ::BOOLEAN,
    pub Version: ::UCHAR,
    pub Links: ::LIST_ENTRY,
    pub SizeOfImage: ::ULONG,
}
pub const MAX_SYM_NAME: usize = 2000;
pub const ERROR_IMAGE_NOT_STRIPPED: ::DWORD = 0x8800;
pub const ERROR_NO_DBG_POINTER: ::DWORD = 0x8801;
pub const ERROR_NO_PDB_POINTER: ::DWORD = 0x8802;
pub type PFIND_DEBUG_FILE_CALLBACK = Option<unsafe extern "system" fn(
    FileHandle: ::HANDLE, FileName: ::PCSTR, CallerData: ::PVOID,
) -> ::BOOL>;
pub type PFIND_DEBUG_FILE_CALLBACKW = Option<unsafe extern "system" fn(
    FileHandle: ::HANDLE, FileName: ::PCWSTR, CallerData: ::PVOID,
) -> ::BOOL>;
pub type PFINDFILEINPATHCALLBACK = Option<unsafe extern "system" fn(
    filename: ::PCSTR, context: ::PVOID,
) -> ::BOOL>;
pub type PFINDFILEINPATHCALLBACKW = Option<unsafe extern "system" fn(
    filename: ::PCWSTR, context: ::PVOID,
) -> ::BOOL>;
pub type PFIND_EXE_FILE_CALLBACK = Option<unsafe extern "system" fn(
    FileHandle: ::HANDLE, FileName: ::PCSTR, CallerData: ::PVOID,
) -> ::BOOL>;
pub type PFIND_EXE_FILE_CALLBACKW = Option<unsafe extern "system" fn(
    FileHandle: ::HANDLE, FileName: ::PCWSTR, CallerData: ::PVOID,
) -> ::BOOL>;
#[repr(C)] #[derive(Clone, Copy, Debug)] #[cfg(target_arch = "x86")]
pub struct IMAGE_DEBUG_INFORMATION {
    pub List: ::LIST_ENTRY,
    pub ReservedSize: ::DWORD,
    pub ReservedMappedBase: ::PVOID,
    pub ReservedMachine: ::USHORT,
    pub ReservedCharacteristics: ::USHORT,
    pub ReservedCheckSum: ::DWORD,
    pub ImageBase: ::DWORD,
    pub SizeOfImage: ::DWORD,
    pub ReservedNumberOfSections: ::DWORD,
    pub ReservedSections: ::PIMAGE_SECTION_HEADER,
    pub ReservedExportedNamesSize: ::DWORD,
    pub ReservedExportedNames: ::PSTR,
    pub ReservedNumberOfFunctionTableEntries: ::DWORD,
    pub ReservedFunctionTableEntries: ::PIMAGE_FUNCTION_ENTRY,
    pub ReservedLowestFunctionStartingAddress: ::DWORD,
    pub ReservedHighestFunctionEndingAddress: ::DWORD,
    pub ReservedNumberOfFpoTableEntries: ::DWORD,
    pub ReservedFpoTableEntries: ::PFPO_DATA,
    pub SizeOfCoffSymbols: ::DWORD,
    pub CoffSymbols: ::PIMAGE_COFF_SYMBOLS_HEADER,
    pub ReservedSizeOfCodeViewSymbols: ::DWORD,
    pub ReservedCodeViewSymbols: ::PVOID,
    pub ImageFilePath: ::PSTR,
    pub ImageFileName: ::PSTR,
    pub ReservedDebugFilePath: ::PSTR,
    pub ReservedTimeDateStamp: ::DWORD,
    pub ReservedRomImage: ::BOOL,
    pub ReservedDebugDirectory: ::PIMAGE_DEBUG_DIRECTORY,
    pub ReservedNumberOfDebugDirectories: ::DWORD,
    pub ReservedOriginalFunctionTableBaseAddress: ::DWORD,
    pub Reserved: [::DWORD; 2],
}
#[cfg(target_arch = "x86")]
pub type PIMAGE_DEBUG_INFORMATION = *mut IMAGE_DEBUG_INFORMATION;
pub type PENUMDIRTREE_CALLBACK = Option<unsafe extern "system" fn(
    FilePath: ::PCSTR, CallerData: ::PVOID,
) -> ::BOOL>;
pub type PENUMDIRTREE_CALLBACKW = Option<unsafe extern "system" fn(
    FilePath: ::PCWSTR, CallerData: ::PVOID,
) -> ::BOOL>;
pub const UNDNAME_COMPLETE: ::DWORD = 0x0000;
pub const UNDNAME_NO_LEADING_UNDERSCORES: ::DWORD = 0x0001;
pub const UNDNAME_NO_MS_KEYWORDS: ::DWORD = 0x0002;
pub const UNDNAME_NO_FUNCTION_RETURNS: ::DWORD = 0x0004;
pub const UNDNAME_NO_ALLOCATION_MODEL: ::DWORD = 0x0008;
pub const UNDNAME_NO_ALLOCATION_LANGUAGE: ::DWORD = 0x0010;
pub const UNDNAME_NO_MS_THISTYPE: ::DWORD = 0x0020;
pub const UNDNAME_NO_CV_THISTYPE: ::DWORD = 0x0040;
pub const UNDNAME_NO_THISTYPE: ::DWORD = 0x0060;
pub const UNDNAME_NO_ACCESS_SPECIFIERS: ::DWORD = 0x0080;
pub const UNDNAME_NO_THROW_SIGNATURES: ::DWORD = 0x0100;
pub const UNDNAME_NO_MEMBER_TYPE: ::DWORD = 0x0200;
pub const UNDNAME_NO_RETURN_UDT_MODEL: ::DWORD = 0x0400;
pub const UNDNAME_32_BIT_DECODE: ::DWORD = 0x0800;
pub const UNDNAME_NAME_ONLY: ::DWORD = 0x1000;
pub const UNDNAME_NO_ARGUMENTS: ::DWORD = 0x2000;
pub const UNDNAME_NO_SPECIAL_SYMS: ::DWORD = 0x4000;
pub const DBHHEADER_DEBUGDIRS: ::DWORD = 0x1;
pub const DBHHEADER_CVMISC: ::DWORD = 0x2;
pub const DBHHEADER_PDBGUID: ::DWORD = 0x3;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct MODLOAD_DATA {
    pub ssize: ::DWORD,
    pub ssig: ::DWORD,
    pub data: ::PVOID,
    pub size: ::DWORD,
    pub flags: ::DWORD,
}
pub type PMODLOAD_DATA = *mut MODLOAD_DATA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct MODLOAD_CVMISC {
    pub oCV: ::DWORD,
    pub cCV: ::size_t,
    pub oMisc: ::DWORD,
    pub cMisc: ::size_t,
    pub dtImage: ::DWORD,
    pub cImage: ::DWORD,
}
pub type PMODLOAD_CVMISC = *mut MODLOAD_CVMISC;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct MODLOAD_PDBGUID_PDBAGE {
    pub PdbGuid: ::GUID,
    pub PdbAge: ::DWORD,
}
pub type PMODLOAD_PDBGUID_PDBAGE = *mut MODLOAD_PDBGUID_PDBAGE;
#[repr(i32)] #[derive(Clone, Copy, Debug)] #[allow(unused_qualifications)]
pub enum ADDRESS_MODE {
    AddrMode1616,
    AddrMode1632,
    AddrModeReal,
    AddrModeFlat,
}
pub use self::ADDRESS_MODE::*;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ADDRESS64 {
    pub Offset: ::DWORD64,
    pub Segment: ::WORD,
    pub Mode: ::ADDRESS_MODE,
}
pub type LPADDRESS64 = *mut ADDRESS64;
#[cfg(target_arch = "x86_64")]
pub type ADDRESS = ADDRESS64;
#[cfg(target_arch = "x86_64")]
pub type LPADDRESS = LPADDRESS64;
#[repr(C)] #[derive(Clone, Copy, Debug)] #[cfg(target_arch = "x86")]
pub struct ADDRESS {
    pub Offset: ::DWORD,
    pub Segment: ::WORD,
    pub Mode: ::ADDRESS_MODE,
}
#[cfg(target_arch = "x86")]
pub type LPADDRESS = *mut ADDRESS;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct KDHELP64 {
    pub Thread: ::DWORD64,
    pub ThCallbackStack: ::DWORD,
    pub ThCallbackBStore: ::DWORD,
    pub NextCallback: ::DWORD,
    pub FramePointer: ::DWORD,
    pub KiCallUserMode: ::DWORD64,
    pub KeUserCallbackDispatcher: ::DWORD64,
    pub SystemRangeStart: ::DWORD64,
    pub KiUserExceptionDispatcher: ::DWORD64,
    pub StackBase: ::DWORD64,
    pub StackLimit: ::DWORD64,
    pub BuildVersion: ::DWORD,
    pub Reserved0: ::DWORD,
    pub Reserved1: [::DWORD64; 4],
}
pub type PKDHELP64 = *mut KDHELP64;
#[cfg(target_arch = "x86_64")]
pub type KDHELP = KDHELP64;
#[cfg(target_arch = "x86_64")]
pub type PKDHELP = PKDHELP64;
#[repr(C)] #[derive(Clone, Copy, Debug)] #[cfg(target_arch = "x86")]
pub struct KDHELP {
    pub Thread: ::DWORD,
    pub ThCallbackStack: ::DWORD,
    pub NextCallback: ::DWORD,
    pub FramePointer: ::DWORD,
    pub KiCallUserMode: ::DWORD,
    pub KeUserCallbackDispatcher: ::DWORD,
    pub SystemRangeStart: ::DWORD,
    pub ThCallbackBStore: ::DWORD,
    pub KiUserExceptionDispatcher: ::DWORD,
    pub StackBase: ::DWORD,
    pub StackLimit: ::DWORD,
    pub Reserved: [::DWORD; 5],
}
#[cfg(target_arch = "x86")]
pub type PKDHELP = *mut KDHELP;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct STACKFRAME64 {
    pub AddrPC: ::ADDRESS64,
    pub AddrReturn: ::ADDRESS64,
    pub AddrFrame: ::ADDRESS64,
    pub AddrStack: ::ADDRESS64,
    pub AddrBStore: ::ADDRESS64,
    pub FuncTableEntry: ::PVOID,
    pub Params: [::DWORD64; 4],
    pub Far: ::BOOL,
    pub Virtual: ::BOOL,
    pub Reserved: [::DWORD64; 3],
    pub KdHelp: ::KDHELP64,
}
pub type LPSTACKFRAME64 = *mut STACKFRAME64;
pub const INLINE_FRAME_CONTEXT_INIT: ::DWORD = 0;
pub const INLINE_FRAME_CONTEXT_IGNORE: ::DWORD = 0xFFFFFFFF;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct STACKFRAME_EX {
    pub AddrPC: ::ADDRESS64,
    pub AddrReturn: ::ADDRESS64,
    pub AddrFrame: ::ADDRESS64,
    pub AddrStack: ::ADDRESS64,
    pub AddrBStore: ::ADDRESS64,
    pub FuncTableEntry: ::PVOID,
    pub Params: [::DWORD64; 4],
    pub Far: ::BOOL,
    pub Virtual: ::BOOL,
    pub Reserved: [::DWORD64; 3],
    pub KdHelp: ::KDHELP64,
    pub StackFrameSize: ::DWORD,
    pub InlineFrameContext: ::DWORD,
}
pub type LPSTACKFRAME_EX = *mut STACKFRAME_EX;
#[cfg(target_arch = "x86_64")]
pub type STACKFRAME = STACKFRAME64;
#[cfg(target_arch = "x86_64")]
pub type LPSTACKFRAME = LPSTACKFRAME64;
#[repr(C)] #[derive(Clone, Copy, Debug)] #[cfg(target_arch = "x86")]
pub struct STACKFRAME {
    pub AddrPC: ::ADDRESS,
    pub AddrReturn: ::ADDRESS,
    pub AddrFrame: ::ADDRESS,
    pub AddrStack: ::ADDRESS,
    pub FuncTableEntry: ::PVOID,
    pub Params: [::DWORD; 4],
    pub Far: ::BOOL,
    pub Virtual: ::BOOL,
    pub Reserved: [::DWORD; 3],
    pub KdHelp: ::KDHELP,
    pub AddrBStore: ::ADDRESS,
}
#[cfg(target_arch = "x86")]
pub type LPSTACKFRAME = *mut STACKFRAME;
pub type PREAD_PROCESS_MEMORY_ROUTINE64 = Option<unsafe extern "system" fn(
    hProcess: ::HANDLE, qwBaseAddress: ::DWORD64, lpBuffer: ::PVOID, nSize: ::DWORD,
    lpNumberOfBytesRead: ::LPDWORD,
) -> ::BOOL>;
pub type PFUNCTION_TABLE_ACCESS_ROUTINE64 = Option<unsafe extern "system" fn(
    ahProcess: ::HANDLE, AddrBase: ::DWORD64,
) -> ::PVOID>;
pub type PGET_MODULE_BASE_ROUTINE64 = Option<unsafe extern "system" fn(
    hProcess: ::HANDLE, Address: ::DWORD64,
) -> ::DWORD64>;
pub type PTRANSLATE_ADDRESS_ROUTINE64 = Option<unsafe extern "system" fn(
    hProcess: ::HANDLE, hThread: ::HANDLE, lpaddr: LPADDRESS64,
) -> ::DWORD64>;
pub const SYM_STKWALK_DEFAULT: ::DWORD = 0x00000000;
pub const SYM_STKWALK_FORCE_FRAMEPTR: ::DWORD = 0x00000001;
#[cfg(target_arch = "x86_64")]
pub type PREAD_PROCESS_MEMORY_ROUTINE = PREAD_PROCESS_MEMORY_ROUTINE64;
#[cfg(target_arch = "x86_64")]
pub type PFUNCTION_TABLE_ACCESS_ROUTINE = PFUNCTION_TABLE_ACCESS_ROUTINE64;
#[cfg(target_arch = "x86_64")]
pub type PGET_MODULE_BASE_ROUTINE = PGET_MODULE_BASE_ROUTINE64;
#[cfg(target_arch = "x86_64")]
pub type PTRANSLATE_ADDRESS_ROUTINE = PTRANSLATE_ADDRESS_ROUTINE64;
#[cfg(target_arch = "x86")]
pub type PREAD_PROCESS_MEMORY_ROUTINE = Option<unsafe extern "system" fn(
    hProcess: ::HANDLE, qwBaseAddress: ::DWORD, lpBuffer: ::PVOID, nSize: ::DWORD,
    lpNumberOfBytesRead: ::PDWORD,
) -> ::BOOL>;
#[cfg(target_arch = "x86")]
pub type PFUNCTION_TABLE_ACCESS_ROUTINE = Option<unsafe extern "system" fn(
    ahProcess: ::HANDLE, AddrBase: ::DWORD,
) -> ::PVOID>;
#[cfg(target_arch = "x86")]
pub type PGET_MODULE_BASE_ROUTINE = Option<unsafe extern "system" fn(
    hProcess: ::HANDLE, Address: ::DWORD,
) -> ::DWORD>;
#[cfg(target_arch = "x86")]
pub type PTRANSLATE_ADDRESS_ROUTINE = Option<unsafe extern "system" fn(
    hProcess: ::HANDLE, hThread: ::HANDLE, lpaddr: LPADDRESS,
) -> ::DWORD>;
pub const API_VERSION_NUMBER: ::USHORT = 12;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct API_VERSION {
    pub MajorVersion: ::USHORT,
    pub MinorVersion: ::USHORT,
    pub Revision: ::USHORT,
    pub Reserved: ::USHORT,
}
pub type LPAPI_VERSION = *mut API_VERSION;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SYMBOL_INFOW {
    pub SizeOfStruct: ::ULONG,
    pub TypeIndex: ::ULONG,
    pub Reserved: [::ULONG64; 2],
    pub Index: ::ULONG,
    pub Size: ::ULONG,
    pub ModBase: ::ULONG64,
    pub Flags: ::ULONG,
    pub Value: ::ULONG64,
    pub Address: ::ULONG64,
    pub Register: ::ULONG,
    pub Scope: ::ULONG,
    pub Tag: ::ULONG,
    pub NameLen: ::ULONG,
    pub MaxNameLen: ::ULONG,
    pub Name: [::WCHAR; 1],
}
pub type PSYMBOL_INFOW = *mut SYMBOL_INFOW;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IMAGEHLP_SYMBOL64 {
    pub SizeOfStruct: ::DWORD,
    pub Address: ::DWORD64,
    pub Size: ::DWORD,
    pub Flags: ::DWORD,
    pub MaxNameLength: ::DWORD,
    pub Name: [::CHAR; 1],
}
pub type PIMAGEHLP_SYMBOL64 = *mut IMAGEHLP_SYMBOL64;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IMAGEHLP_LINEW64 {
    pub SizeOfStruct: ::DWORD,
    pub Key: ::PVOID,
    pub LineNumber: ::DWORD,
    pub FileName: ::PWSTR,
    pub Address: ::DWORD64,
}
pub type PIMAGEHLP_LINEW64 = *mut IMAGEHLP_LINEW64;

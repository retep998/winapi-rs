// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::basetsd::{ULONG32, ULONG64};
use shared::minwindef::{BOOL, DWORD, ULONG};
use shared::ntdef::PWCHAR;
use um::verrsrc::VS_FIXEDFILEINFO;
use um::winnt::{CONTEXT, HANDLE, HRESULT, PEXCEPTION_POINTERS, PVOID};
#[cfg(target_pointer_width = "64")]
use um::winnt::M128A;
STRUCT!{#[repr(packed(4))] struct MINIDUMP_EXCEPTION_INFORMATION {
    ThreadId: DWORD,
    ExceptionPointers: PEXCEPTION_POINTERS,
    ClientPointers: BOOL,
}}
pub type PMINIDUMP_EXCEPTION_INFORMATION = *mut MINIDUMP_EXCEPTION_INFORMATION;
STRUCT!{#[repr(packed(4))] struct MINIDUMP_MEMORY_INFO {
    BaseAddress: ULONG64,
    AllocationBase: ULONG64,
    AllocationProtect: ULONG32,
    __alignment1: ULONG32,
    RegionSize: ULONG64,
    State: ULONG32,
    Protect: ULONG32,
    Type: ULONG32,
    __alignment2: ULONG32,
}}
pub type PMINIDUMP_MEMORY_INFO = *mut MINIDUMP_MEMORY_INFO;
STRUCT!{#[repr(packed(4))] struct MINIDUMP_USER_STREAM {
    Type: ULONG32,
    BufferSize: ULONG,
    Buffer: PVOID,
}}
pub type PMINIDUMP_USER_STREAM = *mut MINIDUMP_USER_STREAM;
STRUCT!{#[repr(packed(4))] struct MINIDUMP_USER_STREAM_INFORMATION {
    UserStreamCount: ULONG,
    UserStreamArray: PMINIDUMP_USER_STREAM,
}}
pub type PMINIDUMP_USER_STREAM_INFORMATION = *mut MINIDUMP_USER_STREAM_INFORMATION;
#[cfg(target_pointer_width = "64")]
STRUCT!{#[repr(align(16))] struct MINIDUMP_THREAD_CALLBACK {
    ThreadId: ULONG,
    ThreadHandle: HANDLE,
    Pad: ULONG,
    Context: CONTEXT,
    SizeOfContext: ULONG,
    StackBase: ULONG64,
    StackEnd: ULONG64,
}}
#[cfg(target_pointer_width = "32")]
STRUCT!{#[repr(packed(4))] struct MINIDUMP_THREAD_CALLBACK {
    ThreadId: ULONG,
    ThreadHandle: HANDLE,
    Context: CONTEXT,
    SizeOfContext: ULONG,
    StackBase: ULONG64,
    StackEnd: ULONG64,
}}
pub type PMINIDUMP_THREAD_CALLBACK = *mut MINIDUMP_THREAD_CALLBACK;
#[cfg(target_pointer_width = "64")]
STRUCT!{#[repr(align(16))] struct MINIDUMP_THREAD_EX_CALLBACK {
    ThreadId: ULONG,
    ThreadHandle: HANDLE,
    Pad: ULONG,
    Context: CONTEXT,
    SizeOfContext: ULONG,
    StackBase: ULONG64,
    StackEnd: ULONG64,
    BackingStoreBase: ULONG64,
    BackingStoreEnd: ULONG64,
}}
#[cfg(target_pointer_width = "32")]
STRUCT!{#[repr(packed(4))] struct MINIDUMP_THREAD_EX_CALLBACK {
    ThreadId: ULONG,
    ThreadHandle: HANDLE,
    Context: CONTEXT,
    SizeOfContext: ULONG,
    StackBase: ULONG64,
    StackEnd: ULONG64,
    BackingStoreBase: ULONG64,
    BackingStoreEnd: ULONG64,
}}
pub type PMINIDUMP_THREAD_EX_CALLBACK = *mut MINIDUMP_THREAD_EX_CALLBACK;
STRUCT!{#[repr(packed(4))] struct MINIDUMP_INCLUDE_THREAD_CALLBACK {
    ThreadId: ULONG,
}}
pub type PMINIDUMP_INCLUDE_THREAD_CALLBACK = *mut MINIDUMP_INCLUDE_THREAD_CALLBACK;
STRUCT!{#[repr(packed(4))] struct MINIDUMP_MODULE_CALLBACK {
    FullPath: PWCHAR,
    BaseOfImage: ULONG64,
    SizeOfImage: ULONG,
    CheckSum: ULONG,
    TimeDateStamp: ULONG,
    VersionInfo: VS_FIXEDFILEINFO,
    CvRecord: PVOID, 
    SizeOfCvRecord: ULONG,
    MiscRecord: PVOID,
    SizeOfMiscRecord: ULONG,
}}
pub type PMINIDUMP_MODULE_CALLBACK = *mut MINIDUMP_MODULE_CALLBACK;
STRUCT!{#[repr(packed(4))] struct MINIDUMP_INCLUDE_MODULE_CALLBACK {
    BaseOfImage: ULONG64,
}}
pub type PMINIDUMP_INCLUDE_MODULE_CALLBACK = *mut MINIDUMP_INCLUDE_MODULE_CALLBACK;
STRUCT!{#[repr(packed(4))] struct MINIDUMP_IO_CALLBACK {
    Handle: HANDLE,
    Offset: ULONG64,
    Buffer: PVOID,
    BufferBytes: ULONG,
}}
pub type PMINIDUMP_IO_CALLBACK = *mut MINIDUMP_IO_CALLBACK;
STRUCT!{#[repr(packed(4))] struct MINIDUMP_READ_MEMORY_FAILURE_CALLBACK {
    Offset: ULONG64,
    Bytes: ULONG,
    FailureStatus: HRESULT,
}}
pub type PMINIDUMP_READ_MEMORY_FAILURE_CALLBACK = *mut MINIDUMP_READ_MEMORY_FAILURE_CALLBACK;
STRUCT!{#[repr(packed(4))] struct MINIDUMP_VM_QUERY_CALLBACK {
    Offset: ULONG64,
}}
pub type PMINIDUMP_VM_QUERY_CALLBACK = *mut MINIDUMP_VM_QUERY_CALLBACK;
STRUCT!{#[repr(packed(4))] struct MINIDUMP_VM_PRE_READ_CALLBACK {
    Offset: ULONG64,
    Buffer: PVOID,
    Size: ULONG,
}}
pub type PMINIDUMP_VM_PRE_READ_CALLBACK = *mut MINIDUMP_VM_PRE_READ_CALLBACK;
STRUCT!{#[repr(packed(4))] struct MINIDUMP_VM_POST_READ_CALLBACK {
    Offset: ULONG64,
    Buffer: PVOID,
    Size: ULONG,
    Completed: ULONG,
    Status: HRESULT,
}}
pub type PMINIDUMP_VM_POST_READ_CALLBACK = *mut MINIDUMP_VM_POST_READ_CALLBACK;
UNION!{union MINIDUMP_CALLBACK_INPUT_u {
    [u32; 190] [M128A; 81],
    Status Status_mut: HRESULT,
    Thread Thread_mut: MINIDUMP_THREAD_CALLBACK,
    ThreadEx ThreadEx_mut: MINIDUMP_THREAD_EX_CALLBACK,
    Module Module_mut: MINIDUMP_MODULE_CALLBACK,
    IncludeThread IncludeThread_mut: MINIDUMP_INCLUDE_THREAD_CALLBACK,
    IncludeModule IncludeModule_mut: MINIDUMP_INCLUDE_MODULE_CALLBACK,
    Io Io_mut: MINIDUMP_IO_CALLBACK,
    ReadMemoryFailure ReadMemoryFailure_mut: MINIDUMP_READ_MEMORY_FAILURE_CALLBACK,
    SecondaryFlags SecondaryFlags_mut: ULONG,
    VmQuery VmQuery_mut: MINIDUMP_VM_QUERY_CALLBACK,
    VmPreRead VmPreRead_mut: MINIDUMP_VM_PRE_READ_CALLBACK,
    VmPostRead VmPostRead_mut: MINIDUMP_VM_POST_READ_CALLBACK,
}}
STRUCT!{#[repr(packed(4))] struct MINIDUMP_CALLBACK_INPUT {
    ProcessId: ULONG,
    ProcessHandle: HANDLE,
    CallbackType: ULONG,
    u: MINIDUMP_CALLBACK_INPUT_u,
}}
pub type PMINIDUMP_CALLBACK_INPUT = *mut MINIDUMP_CALLBACK_INPUT;
STRUCT!{#[repr(packed(4))] struct MINIDUMP_CALLBACK_OUTPUT_u_s1 {
    MemoryBase: ULONG64,
    MemorySize: ULONG,
}}
STRUCT!{#[repr(packed(4))] struct MINIDUMP_CALLBACK_OUTPUT_u_s2 {
    CheckCancel: BOOL,
    Cancel: BOOL,
}}
STRUCT!{#[repr(packed(4))] struct MINIDUMP_CALLBACK_OUTPUT_u_s3 {
    VmRegion: MINIDUMP_MEMORY_INFO,
    Continue: BOOL,
}}
STRUCT!{#[repr(packed(4))] struct MINIDUMP_CALLBACK_OUTPUT_u_s4 {
    VmQueryStatus: HRESULT,
    VmQueryResult: MINIDUMP_MEMORY_INFO,
}}
STRUCT!{#[repr(packed(4))] struct MINIDUMP_CALLBACK_OUTPUT_u_s5 {
    VmReadStatus: HRESULT,
    VmReadBytesCompleted: ULONG,
}}
UNION!{union MINIDUMP_CALLBACK_OUTPUT_u {
    [u32; 13],
    ModuleWriteFlags ModuleWriteFlags_mut: ULONG,
    ThreadWriteFlags ThreadWriteFlags_mut: ULONG,
    SecondaryFlags SecondaryFlags_mut: ULONG,
    s1 s1_mut: MINIDUMP_CALLBACK_OUTPUT_u_s1,
    s2 s2_mut: MINIDUMP_CALLBACK_OUTPUT_u_s2,
    Handle Handle_mut: HANDLE,
    s3 s3_mut: MINIDUMP_CALLBACK_OUTPUT_u_s3,
    s4 s4_mut: MINIDUMP_CALLBACK_OUTPUT_u_s4,
    s5 s5_mut: MINIDUMP_CALLBACK_OUTPUT_u_s5,
    Status Status_mut: HRESULT,
}}
STRUCT!{struct MINIDUMP_CALLBACK_OUTPUT {
    u: MINIDUMP_CALLBACK_OUTPUT_u,
}}
pub type PMINIDUMP_CALLBACK_OUTPUT = *mut MINIDUMP_CALLBACK_OUTPUT;
ENUM!{enum MINIDUMP_TYPE {
    MiniDumpNormal = 0x00000000,
    MiniDumpWithDataSegs = 0x00000001,
    MiniDumpWithFullMemory = 0x00000002,
    MiniDumpWithHandleData = 0x00000004,
    MiniDumpFilterMemory = 0x00000008,
    MiniDumpScanMemory = 0x00000010,
    MiniDumpWithUnloadedModules = 0x00000020,
    MiniDumpWithIndirectlyReferencedMemory = 0x00000040,
    MiniDumpFilterModulePaths = 0x00000080,
    MiniDumpWithProcessThreadData = 0x00000100,
    MiniDumpWithPrivateReadWriteMemory = 0x00000200,
    MiniDumpWithoutOptionalData = 0x00000400,
    MiniDumpWithFullMemoryInfo = 0x00000800,
    MiniDumpWithThreadInfo = 0x00001000,
    MiniDumpWithCodeSegs = 0x00002000,
    MiniDumpWithoutAuxiliaryState = 0x00004000,
    MiniDumpWithFullAuxiliaryState = 0x00008000,
    MiniDumpWithPrivateWriteCopyMemory = 0x00010000,
    MiniDumpIgnoreInaccessibleMemory = 0x00020000,
    MiniDumpWithTokenInformation = 0x00040000,
    MiniDumpWithModuleHeaders = 0x00080000,
    MiniDumpFilterTriage = 0x00100000,
    MiniDumpWithAvxXStateContext = 0x00200000,
    MiniDumpWithIptTrace = 0x00400000,
    MiniDumpScanInaccessiblePartialPages = 0x00800000,
    MiniDumpValidTypeFlags = 0x00ffffff,
}}
FN!{stdcall MINIDUMP_CALLBACK_ROUTINE(
    CallbackParam: PVOID,
    CallbackInput: PMINIDUMP_CALLBACK_INPUT,
    CallbackOutput: PMINIDUMP_CALLBACK_OUTPUT,
) -> BOOL}
STRUCT!{#[repr(packed(4))] struct MINIDUMP_CALLBACK_INFORMATION {
    CallbackRoutine: MINIDUMP_CALLBACK_ROUTINE,
    CallbackParam: PVOID,
}}
pub type PMINIDUMP_CALLBACK_INFORMATION = *mut MINIDUMP_CALLBACK_INFORMATION;
extern "system" {
    pub fn MiniDumpWriteDump(
        hProcess: HANDLE,
        ProcessId: DWORD,
        hFile: HANDLE,
        DumpType: MINIDUMP_TYPE,
        ExceptionParam: PMINIDUMP_EXCEPTION_INFORMATION,
        UserStreamParam: PMINIDUMP_USER_STREAM_INFORMATION,
        CallbackParam: PMINIDUMP_CALLBACK_INFORMATION,
    ) -> BOOL;
}

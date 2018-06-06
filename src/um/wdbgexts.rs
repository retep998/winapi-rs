// Copyright © 2015-2018 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Function prototypes for Windows Debugger Extensions
use ctypes::c_int;
use shared::basetsd::{PULONG64, ULONG64, ULONG_PTR};
use shared::guiddef::IID;
use shared::minwindef::{LPCVOID, PUCHAR, PULONG, UCHAR, ULONG, USHORT};
use um::winnt::{
    HANDLE, HRESULT, LIST_ENTRY32, LIST_ENTRY64, LONGLONG, PCHAR, PCONTEXT, PCSTR, PCWSTR, PSTR,
    PVOID, PWSTR, ULONGLONG, VOID,
};
FN!{cdecl PWINDBG_OUTPUT_ROUTINE(lpExpression: PCSTR, ...) -> VOID}
FN!{stdcall PWINDBG_GET_EXPRESSION(lpExpression: PCSTR,) -> ULONG_PTR}
FN!{stdcall PWINDBG_GET_EXPRESSION32(lpExpression: PCSTR,) -> ULONG}
FN!{stdcall PWINDBG_GET_EXPRESSION64(lpExpression: PCSTR,) -> ULONG64}
FN!{stdcall PWINDBG_GET_SYMBOL(
    offset: PVOID,
    pchBuffer: PCHAR,
    pDisplacement: *const ULONG_PTR,
) -> VOID}
FN!{stdcall PWINDBG_GET_SYMBOL32(
    offset: ULONG,
    pchBuffer: PCHAR,
    pDisplacement: PULONG,
) -> VOID}
FN!{stdcall PWINDBG_GET_SYMBOL64(
    offset: ULONG64,
    pchBuffer: PCHAR,
    pDisplacement: PULONG64,
) -> VOID}
FN!{stdcall PWINDBG_DISASM(
    lpOffset: *const ULONG_PTR,
    lpBuffer: PCSTR,
    fShowEffectiveAddress: ULONG,
) -> ULONG}
FN!{stdcall PWINDBG_DISASM32(
    lpOffset: *const ULONG,
    lpBuffer: PCSTR,
    fShowEffectiveAddress: ULONG,
) -> ULONG}
FN!{stdcall PWINDBG_DISASM64(
    lpOffset: *const ULONG64,
    lpBuffer: PCSTR,
    fShowEffectiveAddress: ULONG,
) -> ULONG}
FN!{stdcall PWINDBG_CHECK_CONTROL_C() -> ULONG}
FN!{stdcall PWINDBG_READ_PROCESS_MEMORY_ROUTINE(
    offset: ULONG_PTR,
    lpBuffer: PVOID,
    cb: ULONG,
    lpcbBytesRead: PULONG,
) -> ULONG}
FN!{stdcall PWINDBG_READ_PROCESS_MEMORY_ROUTINE32(
    offset: ULONG,
    lpBuffer: PVOID,
    cb: ULONG,
    lpcbBytesRead: PULONG,
) -> ULONG}
FN!{stdcall PWINDBG_READ_PROCESS_MEMORY_ROUTINE64(
    offset: ULONG64,
    lpBuffer: PVOID,
    cb: ULONG,
    lpcbBytesRead: PULONG,
) -> ULONG}
FN!{stdcall PWINDBG_WRITE_PROCESS_MEMORY_ROUTINE(
    offset: ULONG_PTR,
    lpBuffer: LPCVOID,
    cb: ULONG,
    lpcbBytesWritten: PULONG,
) -> ULONG}
FN!{stdcall PWINDBG_WRITE_PROCESS_MEMORY_ROUTINE32(
    offset: ULONG,
    lpBuffer: LPCVOID,
    cb: ULONG,
    lpcbBytesWritten: PULONG,
) -> ULONG}
FN!{stdcall PWINDBG_WRITE_PROCESS_MEMORY_ROUTINE64(
    offset: ULONG64,
    lpBuffer: LPCVOID,
    cb: ULONG,
    lpcbBytesWritten: PULONG,
) -> ULONG}
FN!{stdcall PWINDBG_GET_THREAD_CONTEXT_ROUTINE(
    Processor: ULONG,
    lpContext: PCONTEXT,
    cbSizeOfContext: ULONG,
) -> ULONG}
FN!{stdcall PWINDBG_SET_THREAD_CONTEXT_ROUTINE(
    Processor: ULONG,
    lpContext: PCONTEXT,
    cbSizeOfContext: ULONG,
) -> ULONG }
FN!{stdcall PWINDBG_IOCTL_ROUTINE(
    IoctlType: USHORT,
    lpvData: PVOID,
    cbSize: ULONG,
) -> ULONG}
FN!{stdcall PWINDBG_OLDKD_READ_PHYSICAL_MEMORY(
    address: ULONGLONG,
    buffer: PVOID,
    count: ULONG,
    bytesread: PULONG,
) -> ULONG}
FN!{stdcall PWINDBG_OLDKD_WRITE_PHYSICAL_MEMORY(
    address: ULONGLONG,
    buffer: PVOID,
    length: ULONG,
    byteswritten: PULONG,
) -> ULONG}
STRUCT!{struct EXTSTACKTRACE {
    FramePointer: ULONG,
    ProgramCounter: ULONG,
    ReturnAddress: ULONG,
    Args: [ULONG; 4],
}}
pub type PEXTSTACKTRACE = *mut EXTSTACKTRACE;
STRUCT!{struct EXTSTACKTRACE32 {
    FramePointer: ULONG,
    ProgramCounter: ULONG,
    ReturnAddress: ULONG,
    Args: [ULONG; 4],
}}
pub type PEXTSTACKTRACE32 = *mut EXTSTACKTRACE32;
STRUCT!{struct EXTSTACKTRACE64 {
    FramePointer: ULONG64,
    ProgramCounter: ULONG64,
    ReturnAddress: ULONG64,
    Args: [ULONG64; 4],
}}
pub type PEXTSTACKTRACE64 = *mut EXTSTACKTRACE64;
FN!{stdcall PWINDBG_STACKTRACE_ROUTINE(
    FramePointer: ULONG,
    StackPointer: ULONG,
    ProgramCounter: ULONG,
    StackFrames: PEXTSTACKTRACE,
    Frames: ULONG,
) -> ULONG}
FN!{stdcall PWINDBG_STACKTRACE_ROUTINE32(
    FramePointer: ULONG,
    StackPointer: ULONG,
    ProgramCounter: ULONG,
    StackFrames: PEXTSTACKTRACE32,
    Frames: ULONG,
) -> ULONG}
FN!{stdcall PWINDBG_STACKTRACE_ROUTINE64(
    FramePointer: ULONG64,
    StackPointer: ULONG64,
    ProgramCounter: ULONG64,
    StackFrames: PEXTSTACKTRACE64,
    Frames: ULONG,
) -> ULONG}
STRUCT!{struct WINDBG_EXTENSION_APIS {
    nSize: ULONG,
    lpOutputRoutine: PWINDBG_OUTPUT_ROUTINE,
    lpGetExpressionRoutine: PWINDBG_GET_EXPRESSION,
    lpGetSymbolRoutine: PWINDBG_GET_SYMBOL,
    lpDisasmRoutine: PWINDBG_DISASM,
    lpCheckControlCRoutine: PWINDBG_CHECK_CONTROL_C,
    lpReadProcessMemoryRoutine: PWINDBG_READ_PROCESS_MEMORY_ROUTINE,
    lpWriteProcessMemoryRoutine: PWINDBG_WRITE_PROCESS_MEMORY_ROUTINE,
    lpGetThreadContextRoutine: PWINDBG_GET_THREAD_CONTEXT_ROUTINE,
    lpSetThreadContextRoutine: PWINDBG_SET_THREAD_CONTEXT_ROUTINE,
    lpIoctlRoutine: PWINDBG_IOCTL_ROUTINE,
    lpStackTraceRoutine: PWINDBG_STACKTRACE_ROUTINE,
}}
pub type PWINDBG_EXTENSION_APIS = *mut WINDBG_EXTENSION_APIS;
STRUCT!{struct WINDBG_EXTENSION_APIS32 {
    nSize: ULONG,
    lpOutputRoutine: PWINDBG_OUTPUT_ROUTINE,
    lpGetExpressionRoutine: PWINDBG_GET_EXPRESSION32,
    lpGetSymbolRoutine: PWINDBG_GET_SYMBOL32,
    lpDisasmRoutine: PWINDBG_DISASM32,
    lpCheckControlCRoutine: PWINDBG_CHECK_CONTROL_C,
    lpReadProcessMemoryRoutine: PWINDBG_READ_PROCESS_MEMORY_ROUTINE32,
    lpWriteProcessMemoryRoutine: PWINDBG_WRITE_PROCESS_MEMORY_ROUTINE32,
    lpGetThreadContextRoutine: PWINDBG_GET_THREAD_CONTEXT_ROUTINE,
    lpSetThreadContextRoutine: PWINDBG_SET_THREAD_CONTEXT_ROUTINE,
    lpIoctlRoutine: PWINDBG_IOCTL_ROUTINE,
    lpStackTraceRoutine: PWINDBG_STACKTRACE_ROUTINE32,
}}
pub type PWINDBG_EXTENSION_APIS32 = WINDBG_EXTENSION_APIS32;
STRUCT!{struct WINDBG_EXTENSION_APIS64 {
    nSize: ULONG,
    lpOutputRoutine: PWINDBG_OUTPUT_ROUTINE,
    lpGetExpressionRoutine: PWINDBG_GET_EXPRESSION64,
    lpGetSymbolRoutine: PWINDBG_GET_SYMBOL64,
    lpDisasmRoutine: PWINDBG_DISASM64,
    lpCheckControlCRoutine: PWINDBG_CHECK_CONTROL_C,
    lpReadProcessMemoryRoutine: PWINDBG_READ_PROCESS_MEMORY_ROUTINE64,
    lpWriteProcessMemoryRoutine: PWINDBG_WRITE_PROCESS_MEMORY_ROUTINE64,
    lpGetThreadContextRoutine: PWINDBG_GET_THREAD_CONTEXT_ROUTINE,
    lpSetThreadContextRoutine: PWINDBG_SET_THREAD_CONTEXT_ROUTINE,
    lpIoctlRoutine: PWINDBG_IOCTL_ROUTINE,
    lpStackTraceRoutine: PWINDBG_STACKTRACE_ROUTINE64,
}}
pub type PWINDBG_EXTENSION_APIS64 = *mut WINDBG_EXTENSION_APIS64;
STRUCT!{struct WINDBG_OLD_EXTENSION_APIS {
    nSize: ULONG,
    lpOutputRoutine: PWINDBG_OUTPUT_ROUTINE,
    lpGetExpressionRoutine: PWINDBG_GET_EXPRESSION,
    lpGetSymbolRoutine: PWINDBG_GET_SYMBOL,
    lpDisasmRoutine: PWINDBG_DISASM,
    lpCheckControlCRoutine: PWINDBG_CHECK_CONTROL_C,
}}
pub type PWINDBG_OLD_EXTENSION_APIS = *mut WINDBG_OLD_EXTENSION_APIS;
STRUCT!{struct WINDBG_OLDKD_EXTENSION_APIS {
    nSize: ULONG,
    lpOutputRoutine: PWINDBG_OUTPUT_ROUTINE,
    lpGetExpressionRoutine: PWINDBG_GET_EXPRESSION32,
    lpGetSymbolRoutine: PWINDBG_GET_SYMBOL32,
    lpDisasmRoutine: PWINDBG_DISASM32,
    lpCheckControlCRoutine: PWINDBG_CHECK_CONTROL_C,
    lpReadVirtualMemRoutine: PWINDBG_READ_PROCESS_MEMORY_ROUTINE32,
    lpWriteVirtualMemRoutine: PWINDBG_WRITE_PROCESS_MEMORY_ROUTINE32,
    lpReadPhysicalMemRoutine: PWINDBG_OLDKD_READ_PHYSICAL_MEMORY,
    lpWritePhysicalMemRoutine: PWINDBG_OLDKD_WRITE_PHYSICAL_MEMORY,
}}
pub type PWINDBG_OLDKD_EXTENSION_APIS = *mut WINDBG_OLDKD_EXTENSION_APIS;
FN!{stdcall PWINDBG_OLD_EXTENSION_ROUTINE(
    dwCurrentPc: ULONG,
    lpExtensionApis: PWINDBG_EXTENSION_APIS,
    lpArgumentString: PCSTR,
) -> VOID}
FN!{stdcall PWINDBG_EXTENSION_ROUTINE(
    hCurrentProcess: HANDLE,
    hCurrentThread: HANDLE,
    dwCurrentPc: ULONG,
    dwProcessor: ULONG,
    lpArgumentString: PCSTR,
) -> VOID}
FN!{stdcall PWINDBG_EXTENSION_ROUTINE32(
    hCurrentProcess: HANDLE,
    hCurrentThread: HANDLE,
    dwCurrentPc: ULONG,
    dwProcessor: ULONG,
    lpArgumentString: PCSTR,
) -> VOID}
FN!{stdcall PWINDBG_EXTENSION_ROUTINE64(
    hCurrentProcess: HANDLE,
    hCurrentThread: HANDLE,
    dwCurrentPc: ULONG64,
    dwProcessor: ULONG,
    lpArgumentString: PCSTR,
) -> VOID}
FN!{stdcall PWINDBG_OLDKD_EXTENSION_ROUTINE(
    dwCurrentPc: ULONG,
    lpExtensionApis: PWINDBG_OLDKD_EXTENSION_APIS,
    lpArgumentString: PCSTR,
) -> VOID}
FN!{stdcall PWINDBG_EXTENSION_DLL_INIT(
    lpExtensionApis: PWINDBG_EXTENSION_APIS,
    MajorVersion: USHORT,
    MinorVersion: USHORT,
) -> VOID}
FN!{stdcall PWINDBG_EXTENSION_DLL_INIT32(
    lpExtensionApis: PWINDBG_EXTENSION_APIS32,
    MajorVersion: USHORT,
    MinorVersion: USHORT,
) -> VOID}
FN!{stdcall PWINDBG_EXTENSION_DLL_INIT64(
    lpExtensionApis: PWINDBG_EXTENSION_APIS64,
    MajorVersion: USHORT,
    MinorVersion: USHORT,
) -> VOID}
FN!{stdcall PWINDBG_CHECK_VERSION() -> ULONG}
pub const EXT_API_VERSION_NUMBER: USHORT = 5;
pub const EXT_API_VERSION_NUMBER32: USHORT = 5;
pub const EXT_API_VERSION_NUMBER64: USHORT = 6;
STRUCT!{struct EXT_API_VERSION {
    MajorVersion: USHORT,
    MinorVersion: USHORT,
    Revision: USHORT,
    Reserved: USHORT,
}}
pub type LPEXT_API_VERSION = *mut EXT_API_VERSION;
FN!{stdcall PWINDBG_EXTENSION_API_VERSION() -> LPEXT_API_VERSION}
pub const IG_KD_CONTEXT: USHORT = 1;
pub const IG_READ_CONTROL_SPACE: USHORT = 2;
pub const IG_WRITE_CONTROL_SPACE: USHORT = 3;
pub const IG_READ_IO_SPACE: USHORT = 4;
pub const IG_WRITE_IO_SPACE: USHORT = 5;
pub const IG_READ_PHYSICAL: USHORT = 6;
pub const IG_WRITE_PHYSICAL: USHORT = 7;
pub const IG_READ_IO_SPACE_EX: USHORT = 8;
pub const IG_WRITE_IO_SPACE_EX: USHORT = 9;
pub const IG_KSTACK_HELP: USHORT = 10;
pub const IG_SET_THREAD: USHORT = 11;
pub const IG_READ_MSR: USHORT = 12;
pub const IG_WRITE_MSR: USHORT = 13;
pub const IG_GET_DEBUGGER_DATA: USHORT = 14;
pub const IG_GET_KERNEL_VERSION: USHORT = 15;
pub const IG_RELOAD_SYMBOLS: USHORT = 16;
pub const IG_GET_SET_SYMPATH: USHORT = 17;
pub const IG_GET_EXCEPTION_RECORD: USHORT = 18;
pub const IG_IS_PTR64: USHORT = 19;
pub const IG_GET_BUS_DATA: USHORT = 20;
pub const IG_SET_BUS_DATA: USHORT = 21;
pub const IG_DUMP_SYMBOL_INFO: USHORT = 22;
pub const IG_LOWMEM_CHECK: USHORT = 23;
pub const IG_SEARCH_MEMORY: USHORT = 24;
pub const IG_GET_CURRENT_THREAD: USHORT = 25;
pub const IG_GET_CURRENT_PROCESS: USHORT = 26;
pub const IG_GET_TYPE_SIZE: USHORT = 27;
pub const IG_GET_CURRENT_PROCESS_HANDLE: USHORT = 28;
pub const IG_GET_INPUT_LINE: USHORT = 29;
pub const IG_GET_EXPRESSION_EX: USHORT = 30;
pub const IG_TRANSLATE_VIRTUAL_TO_PHYSICAL: USHORT = 31;
pub const IG_GET_CACHE_SIZE: USHORT = 32;
pub const IG_READ_PHYSICAL_WITH_FLAGS: USHORT = 33;
pub const IG_WRITE_PHYSICAL_WITH_FLAGS: USHORT = 34;
pub const IG_POINTER_SEARCH_PHYSICAL: USHORT = 35;
pub const IG_OBSOLETE_PLACEHOLDER_36: USHORT = 36;
pub const IG_GET_THREAD_OS_INFO: USHORT = 37;
pub const IG_GET_CLR_DATA_INTERFACE: USHORT = 38;
pub const IG_MATCH_PATTERN_A: USHORT = 39;
pub const IG_FIND_FILE: USHORT = 40;
pub const IG_TYPED_DATA_OBSOLETE: USHORT = 41;
pub const IG_QUERY_TARGET_INTERFACE: USHORT = 42;
pub const IG_TYPED_DATA: USHORT = 43;
pub const IG_DISASSEMBLE_BUFFER: USHORT = 44;
pub const IG_GET_ANY_MODULE_IN_RANGE: USHORT = 45;
pub const IG_VIRTUAL_TO_PHYSICAL: USHORT = 46;
pub const IG_PHYSICAL_TO_VIRTUAL: USHORT = 47;
pub const IG_GET_CONTEXT_EX: USHORT = 48;
pub const IG_GET_TEB_ADDRESS: USHORT = 128;
pub const IG_GET_PEB_ADDRESS: USHORT = 129;
STRUCT!{struct PROCESSORINFO {
    processor: USHORT,
    processors: USHORT,
}}
pub type PPROCESSORINFO = *mut PROCESSORINFO;
STRUCT!{struct READCONTROLSPACE {
    Processor: USHORT,
    Address: ULONG,
    BufLen: ULONG,
    Buf: [UCHAR; 1],
}}
pub type PREADCONTROLSPACE = *mut READCONTROLSPACE;
STRUCT!{struct READCONTROLSPACE32 {
    Processor: USHORT,
    Address: ULONG,
    BufLen: ULONG,
    Buf: [UCHAR; 1],
}}
pub type PREADCONTROLSPACE32 = *mut READCONTROLSPACE32;
STRUCT!{struct READCONTROLSPACE64 {
    Processor: USHORT,
    Address: ULONG64,
    BufLen: ULONG,
    Buf: [UCHAR; 1],
}}
pub type PREADCONTROLSPACE64 = *mut READCONTROLSPACE64;
STRUCT!{struct IOSPACE {
    Address: ULONG,
    Length: ULONG,
    Data: ULONG,
}}
pub type PIOSPACE = *mut IOSPACE;
STRUCT!{struct IOSPACE32 {
    Address: ULONG,
    Length: ULONG,
    Data: ULONG,
}}
pub type PIOSPACE32 = *mut IOSPACE32;
STRUCT!{struct IOSPACE64 {
    Address: ULONG64,
    Length: ULONG,
    Data: ULONG,
}}
pub type PIOSPACE64 = *mut IOSPACE64;
STRUCT!{struct IOSPACE_EX {
    Address: ULONG,
    Length: ULONG,
    Data: ULONG,
    InterfaceType: ULONG,
    BusNumber: ULONG,
    AddressSpace: ULONG,
}}
pub type PIOSPACE_EX = *mut IOSPACE_EX;
STRUCT!{struct IOSPACE_EX32 {
    Address: ULONG,
    Length: ULONG,
    Data: ULONG,
    InterfaceType: ULONG,
    BusNumber: ULONG,
    AddressSpace: ULONG,
}}
pub type PIOSPACE_EX32 = *mut IOSPACE_EX32;
STRUCT!{struct IOSPACE_EX64 {
    Address: ULONG64,
    Length: ULONG,
    Data: ULONG,
    InterfaceType: ULONG,
    BusNumber: ULONG,
    AddressSpace: ULONG,
}}
pub type PIOSPACE_EX64 = *mut IOSPACE_EX64;
STRUCT!{struct BUSDATA {
    BusDataType: ULONG,
    BusNumber: ULONG,
    SlotNumber: ULONG,
    Buffer: PVOID,
    Offset: ULONG,
    Length: ULONG,
}}
pub type PBUSDATA = *mut BUSDATA;
STRUCT!{struct SEARCHMEMORY {
    SearchAddress: ULONG64,
    SearchLength: ULONG64,
    FoundAddress: ULONG64,
    PatternLength: ULONG,
    Pattern: PVOID,
}}
pub type PSEARCHMEMORY = *mut SEARCHMEMORY;
STRUCT!{struct PHYSICAL {
    Address: ULONGLONG,
    BufLen: ULONG,
    Buf: [UCHAR; 1],
}}
pub type PPHYSICAL = *mut PHYSICAL;
pub const PHYS_FLAG_DEFAULT: ULONG = 0;
pub const PHYS_FLAG_CACHED: ULONG = 1;
pub const PHYS_FLAG_UNCACHED: ULONG = 2;
pub const PHYS_FLAG_WRITE_COMBINED: ULONG = 3;
STRUCT!{struct PHYSICAL_WITH_FLAGS {
    Address: ULONGLONG,
    BufLen: ULONG,
    Flags: ULONG,
    Buf: [UCHAR; 1],
}}
pub type PPHYSICAL_WITH_FLAGS = *mut PHYSICAL_WITH_FLAGS;
STRUCT!{struct READ_WRITE_MSR {
    Msr: ULONG,
    Value: LONGLONG,
}}
pub type PREAD_WRITE_MSR = *mut READ_WRITE_MSR;
STRUCT!{struct GET_SET_SYMPATH {
    Args: PCSTR,
    Result: PSTR,
    Length: c_int,
}}
pub type PGET_SET_SYMPATH = *mut GET_SET_SYMPATH;
STRUCT!{struct GET_TEB_ADDRESS {
    Address: ULONGLONG,
}}
pub type PGET_TEB_ADDRESS = *mut GET_TEB_ADDRESS;
STRUCT!{struct GET_PEB_ADDRESS {
    CurrentThread: ULONG64,
    Address: ULONGLONG,
}}
pub type PGET_PEB_ADDRESS = *mut GET_PEB_ADDRESS;
STRUCT!{struct GET_CURRENT_THREAD_ADDRESS {
    Processor: ULONG,
    Address: ULONG64,
}}
pub type PGET_CURRENT_THREAD_ADDRESS = *mut GET_CURRENT_THREAD_ADDRESS;
STRUCT!{struct GET_CURRENT_PROCESS_ADDRESS {
    Processor: ULONG,
    CurrentThread: ULONG64,
    Address: ULONG64,
}}
pub type PGET_CURRENT_PROCESS_ADDRESS = *mut GET_CURRENT_PROCESS_ADDRESS;
STRUCT!{struct GET_INPUT_LINE {
    Prompt: PCSTR,
    Buffer: PSTR,
    BufferSize: ULONG,
    InputSize: ULONG,
}}
pub type PGET_INPUT_LINE = *mut GET_INPUT_LINE;
STRUCT!{struct GET_EXPRESSION_EX {
    Expression: PCSTR,
    Remainder: PCSTR,
    Value: ULONG64,
}}
pub type PGET_EXPRESSION_EX = *mut GET_EXPRESSION_EX;
STRUCT!{struct TRANSLATE_VIRTUAL_TO_PHYSICAL {
    Virtual: ULONG64,
    Physical: ULONG64,
}}
pub type PTRANSLATE_VIRTUAL_TO_PHYSICAL = *mut TRANSLATE_VIRTUAL_TO_PHYSICAL;
STRUCT!{struct VIRTUAL_TO_PHYSICAL {
    Status: ULONG,
    Size: ULONG,
    PdeAddress: ULONG64,
    Virtual: ULONG64,
    Physical: ULONG64,
}}
pub type PVIRTUAL_TO_PHYSICAL = *mut VIRTUAL_TO_PHYSICAL;
STRUCT!{struct PHYSICAL_TO_VIRTUAL {
    Status: ULONG,
    Size: ULONG,
    PdeAddress: ULONG64,
}}
pub type PPHYSICAL_TO_VIRTUAL = *mut PHYSICAL_TO_VIRTUAL;
STRUCT!{struct GET_CONTEXT_EX {
    Status: ULONG,
    ContextSize: ULONG,
    pContext: PVOID,
}}
pub type PGET_CONTEXT_EX = *mut GET_CONTEXT_EX;
pub const PTR_SEARCH_PHYS_ALL_HITS: ULONG = 0x00000001;
pub const PTR_SEARCH_PHYS_PTE: ULONG = 0x00000002;
pub const PTR_SEARCH_PHYS_RANGE_CHECK_ONLY: ULONG = 0x00000004;
pub const PTR_SEARCH_PHYS_SIZE_SHIFT: ULONG = 3;
pub const PTR_SEARCH_PHYS_SIZE_MASK: ULONG = (0xf << PTR_SEARCH_PHYS_SIZE_SHIFT);
pub const PTR_SEARCH_NO_SYMBOL_CHECK: ULONG = 0x80000000;
STRUCT!{struct POINTER_SEARCH_PHYSICAL {
    Offset: ULONG64,
    Length: ULONG64,
    PointerMin: ULONG64,
    PointerMax: ULONG64,
    Flags: ULONG,
    MatchOffsets: PULONG64,
    MatchOffsetsSize: ULONG,
    MatchOffsetsCount: ULONG,
}}
pub type PPOINTER_SEARCH_PHYSICAL = *mut POINTER_SEARCH_PHYSICAL;
STRUCT!{struct WDBGEXTS_THREAD_OS_INFO {
    ThreadId: ULONG,
    ExitStatus: ULONG,
    PriorityClass: ULONG,
    Priority: ULONG,
    CreateTime: ULONG64,
    ExitTime: ULONG64,
    KernelTime: ULONG64,
    UserTime: ULONG64,
    StartOffset: ULONG64,
    Affinity: ULONG64,
}}
pub type PWDBGEXTS_THREAD_OS_INFO = *mut WDBGEXTS_THREAD_OS_INFO;
STRUCT!{struct WDBGEXTS_CLR_DATA_INTERFACE {
    Iid: *const IID,
    Iface: PVOID,
}}
pub type PWDBGEXTS_CLR_DATA_INTERFACE = *mut WDBGEXTS_CLR_DATA_INTERFACE;
STRUCT!{struct EXT_MATCH_PATTERN_A {
    Str: PCSTR,
    Pattern: PCSTR,
    CaseSensitive: ULONG,
}}
pub type PEXT_MATCH_PATTERN_A = *mut EXT_MATCH_PATTERN_A;
pub const EXT_FIND_FILE_ALLOW_GIVEN_PATH: ULONG = 0x00000001;
STRUCT!{struct EXT_FIND_FILE {
    FileName: PCWSTR,
    IndexedSize: ULONG64,
    ImageTimeDateStamp: ULONG,
    ImageCheckSum: ULONG,
    ExtraInfo: PVOID,
    ExtraInfoSize: ULONG,
    Flags: ULONG,
    FileMapping: PVOID,
    FileMappingSize: ULONG64,
    FileHandle: HANDLE,
    FoundFileName: PWSTR,
    FoundFileNameChars: ULONG,
}}
pub type PEXT_FIND_FILE = *mut EXT_FIND_FILE;
pub const DEBUG_TYPED_DATA_IS_IN_MEMORY: ULONG = 0x00000001;
pub const DEBUG_TYPED_DATA_PHYSICAL_DEFAULT: ULONG = 0x00000002;
pub const DEBUG_TYPED_DATA_PHYSICAL_CACHED: ULONG = 0x00000004;
pub const DEBUG_TYPED_DATA_PHYSICAL_UNCACHED: ULONG = 0x00000006;
pub const DEBUG_TYPED_DATA_PHYSICAL_WRITE_COMBINED: ULONG = 0x00000008;
pub const DEBUG_TYPED_DATA_PHYSICAL_MEMORY: ULONG = 0x0000000e;
STRUCT!{struct DEBUG_TYPED_DATA {
    ModBase: ULONG64,
    Offset: ULONG64,
    EngineHandle: ULONG64,
    Data: ULONG64,
    Size: ULONG,
    Flags: ULONG,
    TypeId: ULONG,
    BaseTypeId: ULONG,
    Tag: ULONG,
    Register: ULONG,
    Internal: [ULONG64; 9],
}}
pub type PDEBUG_TYPED_DATA = *mut DEBUG_TYPED_DATA;
ENUM!{enum EXT_TDOP {
    EXT_TDOP_COPY,
    EXT_TDOP_RELEASE,
    EXT_TDOP_SET_FROM_EXPR,
    EXT_TDOP_SET_FROM_U64_EXPR,
    EXT_TDOP_GET_FIELD,
    EXT_TDOP_EVALUATE,
    EXT_TDOP_GET_TYPE_NAME,
    EXT_TDOP_OUTPUT_TYPE_NAME,
    EXT_TDOP_OUTPUT_SIMPLE_VALUE,
    EXT_TDOP_OUTPUT_FULL_VALUE,
    EXT_TDOP_HAS_FIELD,
    EXT_TDOP_GET_FIELD_OFFSET,
    EXT_TDOP_GET_ARRAY_ELEMENT,
    EXT_TDOP_GET_DEREFERENCE,
    EXT_TDOP_GET_TYPE_SIZE,
    EXT_TDOP_OUTPUT_TYPE_DEFINITION,
    EXT_TDOP_GET_POINTER_TO,
    EXT_TDOP_SET_FROM_TYPE_ID_AND_U64,
    EXT_TDOP_SET_PTR_FROM_TYPE_ID_AND_U64,
    EXT_TDOP_COUNT,
}}
pub const EXT_TDF_PHYSICAL_DEFAULT: ULONG = 0x00000002;
pub const EXT_TDF_PHYSICAL_CACHED: ULONG = 0x00000004;
pub const EXT_TDF_PHYSICAL_UNCACHED: ULONG = 0x00000006;
pub const EXT_TDF_PHYSICAL_WRITE_COMBINED: ULONG = 0x00000008;
pub const EXT_TDF_PHYSICAL_MEMORY: ULONG = 0x0000000e;
STRUCT!{struct EXT_TYPED_DATA {
    Operation: EXT_TDOP,
    Flags: ULONG,
    InData: DEBUG_TYPED_DATA,
    OutData: DEBUG_TYPED_DATA,
    InStrIndex: ULONG,
    In32: ULONG,
    Out32: ULONG,
    In64: ULONG64,
    Out64: ULONG64,
    StrBufferIndex: ULONG,
    StrBufferChars: ULONG,
    StrCharsNeeded: ULONG,
    DataBufferIndex: ULONG,
    DataBufferBytes: ULONG,
    DataBytesNeeded: ULONG,
    Status: HRESULT,
    Reserved: [ULONG; 8],
}}
pub type PEXT_TYPED_DATA = *mut EXT_TYPED_DATA;
STRUCT!{struct WDBGEXTS_QUERY_INTERFACE {
    Iid: *const IID,
    Iface: PVOID,
}}
pub type PWDBGEXTS_QUERY_INTERFACE = *mut WDBGEXTS_QUERY_INTERFACE;
pub const WDBGEXTS_ADDRESS_DEFAULT: ULONG = 0x00000000;
pub const WDBGEXTS_ADDRESS_SEG16: ULONG = 0x00000001;
pub const WDBGEXTS_ADDRESS_SEG32: ULONG = 0x00000002;
pub const WDBGEXTS_ADDRESS_RESERVED0: ULONG = 0x80000000;
STRUCT!{struct WDBGEXTS_DISASSEMBLE_BUFFER {
    InOffset: ULONG64,
    OutOffset: ULONG64,
    AddrFlags: ULONG,
    FormatFlags: ULONG,
    DataBufferBytes: ULONG,
    DisasmBufferChars: ULONG,
    DataBuffer: PVOID,
    DisasmBuffer: PWSTR,
    Reserved0: [ULONG64; 3],
}}
pub type PWDBGEXTS_DISASSEMBLE_BUFFER = *mut WDBGEXTS_DISASSEMBLE_BUFFER;
STRUCT!{struct WDBGEXTS_MODULE_IN_RANGE {
    Start: ULONG64,
    End: ULONG64,
    FoundModBase: ULONG64,
    FoundModSize: ULONG,
}}
pub type PWDBGEXTS_MODULE_IN_RANGE = *mut WDBGEXTS_MODULE_IN_RANGE;
pub const DBGKD_VERS_FLAG_MP: ULONG = 0x0001;
pub const DBGKD_VERS_FLAG_DATA: ULONG = 0x0002;
pub const DBGKD_VERS_FLAG_PTR64: ULONG = 0x0004;
pub const DBGKD_VERS_FLAG_NOMM: ULONG = 0x0008;
pub const DBGKD_VERS_FLAG_HSS: ULONG = 0x0010;
pub const DBGKD_VERS_FLAG_PARTITIONS: ULONG = 0x0020;
pub const KDBG_TAG: ULONG = 0x4742444b; // 'GBDK'
ENUM!{enum DBGKD_MAJOR_TYPES{
    DBGKD_MAJOR_NT,
    DBGKD_MAJOR_XBOX,
    DBGKD_MAJOR_BIG,
    DBGKD_MAJOR_EXDI,
    DBGKD_MAJOR_NTBD,
    DBGKD_MAJOR_EFI,
    DBGKD_MAJOR_TNT,
    DBGKD_MAJOR_SINGULARITY,
    DBGKD_MAJOR_HYPERVISOR,
    DBGKD_MAJOR_MIDORI,
    DBGKD_MAJOR_CE,
    DBGKD_MAJOR_COUNT,
}}
STRUCT!{struct DBGKD_GET_VERSION32 {
    MajorVersion: USHORT,
    MinorVersion: USHORT,
    ProtocolVersion: USHORT,
    Flags: USHORT,
    KernBase: ULONG,
    PsLoadedModuleList: ULONG,
    MachineType: USHORT,
    ThCallbackStack: USHORT,
    NextCallback: USHORT,
    FramePointer: USHORT,
    KiCallUserMode: ULONG,
    KeUserCallbackDispatcher: ULONG,
    BreakpointWithStatus: ULONG,
    DebuggerDataList: ULONG,
}}
pub type PDBGKD_GET_VERSION32 = *mut DBGKD_GET_VERSION32;
STRUCT!{struct DBGKD_DEBUG_DATA_HEADER32 {
    List: LIST_ENTRY32,
    OwnerTag: ULONG,
    Size: ULONG,
}}
pub type PDBGKD_DEBUG_DATA_HEADER32 = *mut DBGKD_DEBUG_DATA_HEADER32;
STRUCT!{struct KDDEBUGGER_DATA32 {
    Header: DBGKD_DEBUG_DATA_HEADER32,
    KernBase: ULONG,
    BreakpointWithStatus: ULONG,
    SavedContext: ULONG,
    ThCallbackStack: USHORT,
    NextCallback: USHORT,
    FramePointer: USHORT,
    BitFields: USHORT,
    KiCallUserMode: ULONG,
    KeUserCallbackDispatcher: ULONG,
    PsLoadedModuleList: ULONG,
    PsActiveProcessHead: ULONG,
    PspCidTable: ULONG,
    ExpSystemResourcesList: ULONG,
    ExpPagedPoolDescriptor: ULONG,
    ExpNumberOfPagedPools: ULONG,
    KeTimeIncrement: ULONG,
    KeBugCheckCallbackListHead: ULONG,
    KiBugcheckData: ULONG,
    IopErrorLogListHead: ULONG,
    ObpRootDirectoryObject: ULONG,
    ObpTypeObjectType: ULONG,
    MmSystemCacheStart: ULONG,
    MmSystemCacheEnd: ULONG,
    MmSystemCacheWs: ULONG,
    MmPfnDatabase: ULONG,
    MmSystemPtesStart: ULONG,
    MmSystemPtesEnd: ULONG,
    MmSubsectionBase: ULONG,
    MmNumberOfPagingFiles: ULONG,
    MmLowestPhysicalPage: ULONG,
    MmHighestPhysicalPage: ULONG,
    MmNumberOfPhysicalPages: ULONG,
    MmMaximumNonPagedPoolInBytes: ULONG,
    MmNonPagedSystemStart: ULONG,
    MmNonPagedPoolStart: ULONG,
    MmNonPagedPoolEnd: ULONG,
    MmPagedPoolStart: ULONG,
    MmPagedPoolEnd: ULONG,
    MmPagedPoolInformation: ULONG,
    MmPageSize: ULONG,
    MmSizeOfPagedPoolInBytes: ULONG,
    MmTotalCommitLimit: ULONG,
    MmTotalCommittedPages: ULONG,
    MmSharedCommit: ULONG,
    MmDriverCommit: ULONG,
    MmProcessCommit: ULONG,
    MmPagedPoolCommit: ULONG,
    MmExtendedCommit: ULONG,
    MmZeroedPageListHead: ULONG,
    MmFreePageListHead: ULONG,
    MmStandbyPageListHead: ULONG,
    MmModifiedPageListHead: ULONG,
    MmModifiedNoWritePageListHead: ULONG,
    MmAvailablePages: ULONG,
    MmResidentAvailablePages: ULONG,
    PoolTrackTable: ULONG,
    NonPagedPoolDescriptor: ULONG,
    MmHighestUserAddress: ULONG,
    MmSystemRangeStart: ULONG,
    MmUserProbeAddress: ULONG,
    KdPrintCircularBuffer: ULONG,
    KdPrintCircularBufferEnd: ULONG,
    KdPrintWritePointer: ULONG,
    KdPrintRolloverCount: ULONG,
    MmLoadedUserImageList: ULONG,
}}
pub type PKDDEBUGGER_DATA32 = *mut KDDEBUGGER_DATA32;
BITFIELD!(KDDEBUGGER_DATA32 BitFields: USHORT [
    PaeEnabled set_PaeEnabled[0..1],
]);
pub const KD_SECONDARY_VERSION_DEFAULT: ULONG = 0;
pub const KD_SECONDARY_VERSION_AMD64_OBSOLETE_CONTEXT_1: ULONG = 0;
pub const KD_SECONDARY_VERSION_AMD64_OBSOLETE_CONTEXT_2: ULONG = 1;
pub const KD_SECONDARY_VERSION_AMD64_CONTEXT: ULONG = 2;
STRUCT!{struct DBGKD_GET_VERSION64 {
    MajorVersion: USHORT,
    MinorVersion: USHORT,
    ProtocolVersion: UCHAR,
    KdSecondaryVersion: UCHAR,
    Flags: USHORT,
    MachineType: USHORT,
    MaxPacketType: UCHAR,
    MaxStateChange: UCHAR,
    MaxManipulate: UCHAR,
    Simulation: UCHAR,
    Unused: [USHORT; 1],
    KernBase: ULONG64,
    PsLoadedModuleList: ULONG64,
    DebuggerDataList: ULONG64,
}}
pub type PDBGKD_GET_VERSION64 = *mut DBGKD_GET_VERSION64;
STRUCT!{struct DBGKD_DEBUG_DATA_HEADER64 {
    List: LIST_ENTRY64,
    OwnerTag: ULONG,
    Size: ULONG,
}}
pub type PDBGKD_DEBUG_DATA_HEADER64 = *mut DBGKD_DEBUG_DATA_HEADER64;
STRUCT!{struct KDDEBUGGER_DATA64 {
    Header: DBGKD_DEBUG_DATA_HEADER64,
    KernBase: ULONG64,
    BreakpointWithStatus: ULONG64,
    SavedContext: ULONG64,
    ThCallbackStack: USHORT,
    NextCallback: USHORT,
    FramePointer: USHORT,
    BitFields: USHORT,
    KiCallUserMode: ULONG64,
    KeUserCallbackDispatcher: ULONG64,
    PsLoadedModuleList: ULONG64,
    PsActiveProcessHead: ULONG64,
    PspCidTable: ULONG64,
    ExpSystemResourcesList: ULONG64,
    ExpPagedPoolDescriptor: ULONG64,
    ExpNumberOfPagedPools: ULONG64,
    KeTimeIncrement: ULONG64,
    KeBugCheckCallbackListHead: ULONG64,
    KiBugcheckData: ULONG64,
    IopErrorLogListHead: ULONG64,
    ObpRootDirectoryObject: ULONG64,
    ObpTypeObjectType: ULONG64,
    MmSystemCacheStart: ULONG64,
    MmSystemCacheEnd: ULONG64,
    MmSystemCacheWs: ULONG64,
    MmPfnDatabase: ULONG64,
    MmSystemPtesStart: ULONG64,
    MmSystemPtesEnd: ULONG64,
    MmSubsectionBase: ULONG64,
    MmNumberOfPagingFiles: ULONG64,
    MmLowestPhysicalPage: ULONG64,
    MmHighestPhysicalPage: ULONG64,
    MmNumberOfPhysicalPages: ULONG64,
    MmMaximumNonPagedPoolInBytes: ULONG64,
    MmNonPagedSystemStart: ULONG64,
    MmNonPagedPoolStart: ULONG64,
    MmNonPagedPoolEnd: ULONG64,
    MmPagedPoolStart: ULONG64,
    MmPagedPoolEnd: ULONG64,
    MmPagedPoolInformation: ULONG64,
    MmPageSize: ULONG64,
    MmSizeOfPagedPoolInBytes: ULONG64,
    MmTotalCommitLimit: ULONG64,
    MmTotalCommittedPages: ULONG64,
    MmSharedCommit: ULONG64,
    MmDriverCommit: ULONG64,
    MmProcessCommit: ULONG64,
    MmPagedPoolCommit: ULONG64,
    MmExtendedCommit: ULONG64,
    MmZeroedPageListHead: ULONG64,
    MmFreePageListHead: ULONG64,
    MmStandbyPageListHead: ULONG64,
    MmModifiedPageListHead: ULONG64,
    MmModifiedNoWritePageListHead: ULONG64,
    MmAvailablePages: ULONG64,
    MmResidentAvailablePages: ULONG64,
    PoolTrackTable: ULONG64,
    NonPagedPoolDescriptor: ULONG64,
    MmHighestUserAddress: ULONG64,
    MmSystemRangeStart: ULONG64,
    MmUserProbeAddress: ULONG64,
    KdPrintCircularBuffer: ULONG64,
    KdPrintCircularBufferEnd: ULONG64,
    KdPrintWritePointer: ULONG64,
    KdPrintRolloverCount: ULONG64,
    MmLoadedUserImageList: ULONG64,
    NtBuildLab: ULONG64,
    KiNormalSystemCall: ULONG64,
    KiProcessorBlock: ULONG64,
    MmUnloadedDrivers: ULONG64,
    MmLastUnloadedDriver: ULONG64,
    MmTriageActionTaken: ULONG64,
    MmSpecialPoolTag: ULONG64,
    KernelVerifier: ULONG64,
    MmVerifierData: ULONG64,
    MmAllocatedNonPagedPool: ULONG64,
    MmPeakCommitment: ULONG64,
    MmTotalCommitLimitMaximum: ULONG64,
    CmNtCSDVersion: ULONG64,
    MmPhysicalMemoryBlock: ULONG64,
    MmSessionBase: ULONG64,
    MmSessionSize: ULONG64,
    MmSystemParentTablePage: ULONG64,
    MmVirtualTranslationBase: ULONG64,
    OffsetKThreadNextProcessor: USHORT,
    OffsetKThreadTeb: USHORT,
    OffsetKThreadKernelStack: USHORT,
    OffsetKThreadInitialStack: USHORT,
    OffsetKThreadApcProcess: USHORT,
    OffsetKThreadState: USHORT,
    OffsetKThreadBStore: USHORT,
    OffsetKThreadBStoreLimit: USHORT,
    SizeEProcess: USHORT,
    OffsetEprocessPeb: USHORT,
    OffsetEprocessParentCID: USHORT,
    OffsetEprocessDirectoryTableBase: USHORT,
    SizePrcb: USHORT,
    OffsetPrcbDpcRoutine: USHORT,
    OffsetPrcbCurrentThread: USHORT,
    OffsetPrcbMhz: USHORT,
    OffsetPrcbCpuType: USHORT,
    OffsetPrcbVendorString: USHORT,
    OffsetPrcbProcStateContext: USHORT,
    OffsetPrcbNumber: USHORT,
    SizeEThread: USHORT,
    KdPrintCircularBufferPtr: ULONG64,
    KdPrintBufferSize: ULONG64,
    KeLoaderBlock: ULONG64,
    SizePcr: USHORT,
    OffsetPcrSelfPcr: USHORT,
    OffsetPcrCurrentPrcb: USHORT,
    OffsetPcrContainedPrcb: USHORT,
    OffsetPcrInitialBStore: USHORT,
    OffsetPcrBStoreLimit: USHORT,
    OffsetPcrInitialStack: USHORT,
    OffsetPcrStackLimit: USHORT,
    OffsetPrcbPcrPage: USHORT,
    OffsetPrcbProcStateSpecialReg: USHORT,
    GdtR0Code: USHORT,
    GdtR0Data: USHORT,
    GdtR0Pcr: USHORT,
    GdtR3Code: USHORT,
    GdtR3Data: USHORT,
    GdtR3Teb: USHORT,
    GdtLdt: USHORT,
    GdtTss: USHORT,
    Gdt64R3CmCode: USHORT,
    Gdt64R3CmTeb: USHORT,
    IopNumTriageDumpDataBlocks: ULONG64,
    IopTriageDumpDataBlocks: ULONG64,
    VfCrashDataBlock: ULONG64,
    MmBadPagesDetected: ULONG64,
    MmZeroedPageSingleBitErrorsDetected: ULONG64,
    EtwpDebuggerData: ULONG64,
    OffsetPrcbContext: USHORT,
    OffsetPrcbMaxBreakpoints: USHORT,
    OffsetPrcbMaxWatchpoints: USHORT,
    OffsetKThreadStackLimit: ULONG,
    OffsetKThreadStackBase: ULONG,
    OffsetKThreadQueueListEntry: ULONG,
    OffsetEThreadIrpList: ULONG,
    OffsetPrcbIdleThread: USHORT,
    OffsetPrcbNormalDpcState: USHORT,
    OffsetPrcbDpcStack: USHORT,
    OffsetPrcbIsrStack: USHORT,
    SizeKDPC_STACK_FRAME: USHORT,
    OffsetKPriQueueThreadListHead: USHORT,
    OffsetKThreadWaitReason: USHORT,
    Padding: USHORT,
    PteBase: ULONG64,
}}
pub type PKDDEBUGGER_DATA64 = *mut KDDEBUGGER_DATA64;
BITFIELD!(KDDEBUGGER_DATA64 BitFields: USHORT [
    PaeEnabled set_PaeEnabled[0..1],
]);
pub const DBG_DUMP_NO_INDENT: ULONG = 0x00000001;
pub const DBG_DUMP_NO_OFFSET: ULONG = 0x00000002;
pub const DBG_DUMP_VERBOSE: ULONG = 0x00000004;
pub const DBG_DUMP_CALL_FOR_EACH: ULONG = 0x00000008;
pub const DBG_DUMP_LIST: ULONG = 0x00000020;
pub const DBG_DUMP_NO_PRINT: ULONG = 0x00000040;
pub const DBG_DUMP_GET_SIZE_ONLY: ULONG = 0x00000080;
#[inline]
pub fn DBG_DUMP_RECUR_LEVEL(l: ULONG) -> ULONG {
    ((l & 0xf) << 8)
}
pub const DBG_DUMP_COMPACT_OUT: ULONG = 0x00002000;
pub const DBG_DUMP_ARRAY: ULONG = 0x00008000;
pub const DBG_DUMP_ADDRESS_OF_FIELD: ULONG = 0x00010000;
pub const DBG_DUMP_ADDRESS_AT_END: ULONG = 0x00020000;
pub const DBG_DUMP_COPY_TYPE_DATA: ULONG = 0x00040000;
pub const DBG_DUMP_READ_PHYSICAL: ULONG = 0x00080000;
pub const DBG_DUMP_FUNCTION_FORMAT: ULONG = 0x00100000;
pub const DBG_DUMP_BLOCK_RECURSE: ULONG = 0x00200000;
pub const DBG_DUMP_MATCH_SIZE: ULONG = 0x00400000;
pub const DBG_RETURN_TYPE: ULONG = 0;
pub const DBG_RETURN_SUBTYPES: ULONG = 0;
pub const DBG_RETURN_TYPE_VALUES: ULONG = 0;
pub const DBG_DUMP_FIELD_CALL_BEFORE_PRINT: ULONG = 0x00000001;
pub const DBG_DUMP_FIELD_NO_CALLBACK_REQ: ULONG = 0x00000002;
pub const DBG_DUMP_FIELD_RECUR_ON_THIS: ULONG = 0x00000004;
pub const DBG_DUMP_FIELD_FULL_NAME: ULONG = 0x00000008;
pub const DBG_DUMP_FIELD_ARRAY: ULONG = 0x00000010;
pub const DBG_DUMP_FIELD_COPY_FIELD_DATA: ULONG = 0x00000020;
pub const DBG_DUMP_FIELD_RETURN_ADDRESS: ULONG = 0x00001000;
pub const DBG_DUMP_FIELD_SIZE_IN_BITS: ULONG = 0x00002000;
pub const DBG_DUMP_FIELD_NO_PRINT: ULONG = 0x00004000;
pub const DBG_DUMP_FIELD_DEFAULT_STRING: ULONG = 0x00010000;
pub const DBG_DUMP_FIELD_WCHAR_STRING: ULONG = 0x00020000;
pub const DBG_DUMP_FIELD_MULTI_STRING: ULONG = 0x00040000;
pub const DBG_DUMP_FIELD_GUID_STRING: ULONG = 0x00080000;
pub const MEMORY_READ_ERROR: ULONG = 0x01;
pub const SYMBOL_TYPE_INDEX_NOT_FOUND: ULONG = 0x02;
pub const SYMBOL_TYPE_INFO_NOT_FOUND: ULONG = 0x03;
pub const FIELDS_DID_NOT_MATCH: ULONG = 0x04;
pub const NULL_SYM_DUMP_PARAM: ULONG = 0x05;
pub const NULL_FIELD_NAME: ULONG = 0x06;
pub const INCORRECT_VERSION_INFO: ULONG = 0x07;
pub const EXIT_ON_CONTROLC: ULONG = 0x08;
pub const CANNOT_ALLOCATE_MEMORY: ULONG = 0x09;
pub const INSUFFICIENT_SPACE_TO_COPY: ULONG = 0x0a;
pub const ADDRESS_TYPE_INDEX_NOT_FOUND: ULONG = 0x0b;
pub const UNAVAILABLE_ERROR: ULONG = 0x0c;
FN!{stdcall PSYM_DUMP_FIELD_CALLBACK(
    pField: *mut FIELD_INFO,
    UserContext: PVOID,
) -> ULONG}
UNION!{union FIELD_INFO_u {
    [usize; 1],
    fieldCallBack fieldCallBack_mut: PVOID,
    pBuffer pBuffer_mut: PVOID,
}}
STRUCT!{struct FIELD_INFO_BitField {
    Position: USHORT,
    Size: USHORT,
}}
STRUCT!{struct FIELD_INFO {
   fName: PUCHAR,
   printName: PUCHAR,
   size: ULONG,
   fOptions: ULONG,
   address: ULONG64,
   u1: FIELD_INFO_u,
   TypeId: ULONG,
   FieldOffset: ULONG,
   BufferSize: ULONG,
   BitField: FIELD_INFO_BitField,
   BitFields: ULONG,
}}
pub type PFIELD_INFO = *mut FIELD_INFO;
BITFIELD!(FIELD_INFO BitFields: ULONG [
   fPointer set_fPointer[0..2],
   fArray set_fArray[2..3],
   fStruct set_fStruct[3..4],
   fConstant set_fConstant[4..5],
   fStatic set_fStatic[5..6],
   Reserved set_Reserved[6..32],
]);
UNION!{union SYM_DUMP_PARAM_u {
    [usize; 1],
    Context Context_mut: PVOID,
    pBuffer pBuffer_mut: PVOID,
}}
STRUCT!{struct SYM_DUMP_PARAM {
   size: ULONG,
   sName: PUCHAR,
   Options: ULONG,
   addr: ULONG64,
   listLink: PFIELD_INFO,
   u: SYM_DUMP_PARAM_u,
   CallbackRoutine: PSYM_DUMP_FIELD_CALLBACK,
   nFields: ULONG,
   Fields: PFIELD_INFO,
   ModBase: ULONG64,
   TypeId: ULONG,
   TypeSize: ULONG,
   BufferSize: ULONG,
   BitFields: ULONG,
}}
pub type PSYM_DUMP_PARAM = *mut SYM_DUMP_PARAM;
BITFIELD!(SYM_DUMP_PARAM BitFields: ULONG[
   fPointer set_fPointer[0..2],
   fArray set_fArray[2..3],
   fStruct set_fStruct[3..4],
   fConstant set_fConstant[4..5],
   Reserved set_Reserved[5..32],
]);


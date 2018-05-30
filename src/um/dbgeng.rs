// Copyright © 2015-2018 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Function prototypes for Windows Debugger Extensions
use ctypes::{c_double, c_float};
use shared::basetsd::{LONG64, PULONG64, ULONG64};
use shared::guiddef::{LPGUID, REFIID};
use shared::minwindef::{BOOL, BYTE, DWORD, FARPROC, PBOOL, PULONG, UCHAR, UINT, USHORT, WORD};
use shared::ntdef::{HRESULT, LONG, PCSTR, PCWSTR, PSTR, PVOID, PWSTR, ULONG};
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::wdbgexts::{PWINDBG_EXTENSION_APIS32, PWINDBG_EXTENSION_APIS64};
use um::winbase::{CREATE_UNICODE_ENVIRONMENT, STACK_SIZE_PARAM_IS_A_RESERVATION};
use um::winnt::{
    EXCEPTION_RECORD64, PEXCEPTION_RECORD64, PIMAGE_NT_HEADERS64, PMEMORY_BASIC_INFORMATION64, CHAR,
};
use vc::vadefs::va_list;
pub const ERROR_DBG_CANCELLED: DWORD = 0xC00004C7;
pub const ERROR_DBG_TIMEOUT: DWORD = 0xC00005B4;
DEFINE_GUID!{IID_IDebugAdvanced, 0xf2df5f53, 0x071f, 0x47bd,
0x9d, 0xe6, 0x57, 0x34, 0xc3, 0xfe, 0xd6, 0x89}
DEFINE_GUID!{IID_IDebugAdvanced2, 0x716d14c9, 0x119b, 0x4ba5,
0xaf, 0x1f, 0x08, 0x90, 0xe6, 0x72, 0x41, 0x6a}
DEFINE_GUID!{IID_IDebugAdvanced3, 0xcba4abb4, 0x84c4, 0x444d,
0x87, 0xca, 0xa0, 0x4e, 0x13, 0x28, 0x67, 0x39}
DEFINE_GUID!{IID_IDebugAdvanced4, 0xd1069067, 0x2a65, 0x4bf0,
0xae, 0x97, 0x76, 0x18, 0x4b, 0x67, 0x85, 0x6b}
DEFINE_GUID!{IID_IDebugBreakpoint, 0x5bd9d474, 0x5975, 0x423a,
0xb8, 0x8b, 0x65, 0xa8, 0xe7, 0x11, 0x0e, 0x65}
DEFINE_GUID!{IID_IDebugBreakpoint2, 0x1b278d20, 0x79f2, 0x426e,
0xa3, 0xf9, 0xc1, 0xdd, 0xf3, 0x75, 0xd4, 0x8e}
DEFINE_GUID!{IID_IDebugBreakpoint3, 0x38f5c249, 0xb448, 0x43bb,
0x98, 0x35, 0x57, 0x9d, 0x4e, 0xc0, 0x22, 0x49}
DEFINE_GUID!{IID_IDebugClient, 0x27fe5639, 0x8407, 0x4f47,
0x83, 0x64, 0xee, 0x11, 0x8f, 0xb0, 0x8a, 0xc8}
DEFINE_GUID!{IID_IDebugClient2, 0xedbed635, 0x372e, 0x4dab,
0xbb, 0xfe, 0xed, 0x0d, 0x2f, 0x63, 0xbe, 0x81}
DEFINE_GUID!{IID_IDebugClient3, 0xdd492d7f, 0x71b8, 0x4ad6,
0xa8, 0xdc, 0x1c, 0x88, 0x74, 0x79, 0xff, 0x91}
DEFINE_GUID!{IID_IDebugClient4, 0xca83c3de, 0x5089, 0x4cf8,
0x93, 0xc8, 0xd8, 0x92, 0x38, 0x7f, 0x2a, 0x5e}
DEFINE_GUID!{IID_IDebugClient5, 0xe3acb9d7, 0x7ec2, 0x4f0c,
0xa0, 0xda, 0xe8, 0x1e, 0x0c, 0xbb, 0xe6, 0x28}
DEFINE_GUID!{IID_IDebugClient6, 0xfd28b4c5, 0xc498, 0x4686,
0xa2, 0x8e, 0x62, 0xca, 0xd2, 0x15, 0x4e, 0xb3}
DEFINE_GUID!{IID_IDebugClient7, 0x13586be3, 0x542e, 0x481e,
0xb1, 0xf2, 0x84, 0x97, 0xba, 0x74, 0xf9, 0xa9}
DEFINE_GUID!{IID_IDebugPlmClient, 0xa02b66c4, 0xaea3, 0x4234,
0xa9, 0xf7, 0xfe, 0x4c, 0x38, 0x3d, 0x4e, 0x29}
DEFINE_GUID!{IID_IDebugPlmClient2, 0x597c980d, 0xe7bd, 0x4309,
0x96, 0x2c, 0x9d, 0x9b, 0x69, 0xa7, 0x37, 0x2c}
DEFINE_GUID!{IID_IDebugPlmClient3, 0xcdf48669, 0x901f, 0x4791,
0xb8, 0x68, 0x7d, 0x2c, 0xb3, 0xa2, 0xd7, 0xfc}
DEFINE_GUID!{IID_IDebugOutputStream, 0x7782d8f2, 0x2b85, 0x4059,
0xab, 0x88, 0x28, 0xce, 0xdd, 0xca, 0x1c, 0x80}
DEFINE_GUID!{IID_IDebugControl, 0x5182e668, 0x105e, 0x416e,
0xad, 0x92, 0x24, 0xef, 0x80, 0x04, 0x24, 0xba}
DEFINE_GUID!{IID_IDebugControl2, 0xd4366723, 0x44df, 0x4bed,
0x8c, 0x7e, 0x4c, 0x05, 0x42, 0x4f, 0x45, 0x88}
DEFINE_GUID!{IID_IDebugControl3, 0x7df74a86, 0xb03f, 0x407f,
0x90, 0xab, 0xa2, 0x0d, 0xad, 0xce, 0xad, 0x08}
DEFINE_GUID!{IID_IDebugControl4, 0x94e60ce9, 0x9b41, 0x4b19,
0x9f, 0xc0, 0x6d, 0x9e, 0xb3, 0x52, 0x72, 0xb3}
DEFINE_GUID!{IID_IDebugControl5, 0xb2ffe162, 0x2412, 0x429f,
0x8d, 0x1d, 0x5b, 0xf6, 0xdd, 0x82, 0x46, 0x96}
DEFINE_GUID!{IID_IDebugControl6, 0xbc0d583f, 0x126d, 0x43a1,
0x9c, 0xc4, 0xa8, 0x60, 0xab, 0x1d, 0x53, 0x7b}
DEFINE_GUID!{IID_IDebugControl7, 0xb86fb3b1, 0x80d4, 0x475b,
0xae, 0xa3, 0xcf, 0x06, 0x53, 0x9c, 0xf6, 0x3a}
DEFINE_GUID!{IID_IDebugDataSpaces, 0x88f7dfab, 0x3ea7, 0x4c3a,
0xae, 0xfb, 0xc4, 0xe8, 0x10, 0x61, 0x73, 0xaa}
DEFINE_GUID!{IID_IDebugDataSpaces2, 0x7a5e852f, 0x96e9, 0x468f,
0xac, 0x1b, 0x0b, 0x3a, 0xdd, 0xc4, 0xa0, 0x49}
DEFINE_GUID!{IID_IDebugDataSpaces3, 0x23f79d6c, 0x8aaf, 0x4f7c,
0xa6, 0x07, 0x99, 0x95, 0xf5, 0x40, 0x7e, 0x63}
DEFINE_GUID!{IID_IDebugDataSpaces4, 0xd98ada1f, 0x29e9, 0x4ef5,
0xa6, 0xc0, 0xe5, 0x33, 0x49, 0x88, 0x32, 0x12}
DEFINE_GUID!{IID_IDebugEventCallbacks, 0x337be28b, 0x5036, 0x4d72,
0xb6, 0xbf, 0xc4, 0x5f, 0xbb, 0x9f, 0x2e, 0xaa}
DEFINE_GUID!{IID_IDebugEventCallbacksWide, 0x0690e046, 0x9c23, 0x45ac,
0xa0, 0x4f, 0x98, 0x7a, 0xc2, 0x9a, 0xd0, 0xd3}
DEFINE_GUID!{IID_IDebugEventContextCallbacks, 0x61a4905b, 0x23f9, 0x4247,
0xb3, 0xc5, 0x53, 0xd0, 0x87, 0x52, 0x9a, 0xb7}
DEFINE_GUID!{IID_IDebugInputCallbacks, 0x9f50e42c, 0xf136, 0x499e,
0x9a, 0x97, 0x73, 0x03, 0x6c, 0x94, 0xed, 0x2d}
DEFINE_GUID!{IID_IDebugOutputCallbacks, 0x4bf58045, 0xd654, 0x4c40,
0xb0, 0xaf, 0x68, 0x30, 0x90, 0xf3, 0x56, 0xdc}
DEFINE_GUID!{IID_IDebugOutputCallbacksWide, 0x4c7fd663, 0xc394, 0x4e26,
0x8e, 0xf1, 0x34, 0xad, 0x5e, 0xd3, 0x76, 0x4c}
DEFINE_GUID!{IID_IDebugOutputCallbacks2, 0x67721fe9, 0x56d2, 0x4a44,
0xa3, 0x25, 0x2b, 0x65, 0x51, 0x3c, 0xe6, 0xeb}
DEFINE_GUID!{IID_IDebugRegisters, 0xce289126, 0x9e84, 0x45a7,
0x93, 0x7e, 0x67, 0xbb, 0x18, 0x69, 0x14, 0x93}
DEFINE_GUID!{IID_IDebugRegisters2, 0x1656afa9, 0x19c6, 0x4e3a,
0x97, 0xe7, 0x5d, 0xc9, 0x16, 0x0c, 0xf9, 0xc4}
DEFINE_GUID!{IID_IDebugSymbolGroup, 0xf2528316, 0x0f1a, 0x4431,
0xae, 0xed, 0x11, 0xd0, 0x96, 0xe1, 0xe2, 0xab}
DEFINE_GUID!{IID_IDebugSymbolGroup2, 0x6a7ccc5f, 0xfb5e, 0x4dcc,
0xb4, 0x1c, 0x6c, 0x20, 0x30, 0x7b, 0xcc, 0xc7}
DEFINE_GUID!{IID_IDebugSymbols, 0x8c31e98c, 0x983a, 0x48a5,
0x90, 0x16, 0x6f, 0xe5, 0xd6, 0x67, 0xa9, 0x50}
DEFINE_GUID!{IID_IDebugSymbols2, 0x3a707211, 0xafdd, 0x4495,
0xad, 0x4f, 0x56, 0xfe, 0xcd, 0xf8, 0x16, 0x3f}
DEFINE_GUID!{IID_IDebugSymbols3, 0xf02fbecc, 0x50ac, 0x4f36,
0x9a, 0xd9, 0xc9, 0x75, 0xe8, 0xf3, 0x2f, 0xf8}
DEFINE_GUID!{IID_IDebugSymbols4, 0xe391bbd8, 0x9d8c, 0x4418,
0x84, 0x0b, 0xc0, 0x06, 0x59, 0x2a, 0x17, 0x52}
DEFINE_GUID!{IID_IDebugSymbols5, 0xc65fa83e, 0x1e69, 0x475e,
0x8e, 0x0e, 0xb5, 0xd7, 0x9e, 0x9c, 0xc1, 0x7e}
DEFINE_GUID!{IID_IDebugSystemObjects, 0x6b86fe2c, 0x2c4f, 0x4f0c,
0x9d, 0xa2, 0x17, 0x43, 0x11, 0xac, 0xc3, 0x27}
DEFINE_GUID!{IID_IDebugSystemObjects2, 0x0ae9f5ff, 0x1852, 0x4679,
0xb0, 0x55, 0x49, 0x4b, 0xee, 0x64, 0x07, 0xee}
DEFINE_GUID!{IID_IDebugSystemObjects3, 0xe9676e2f, 0xe286, 0x4ea3,
0xb0, 0xf9, 0xdf, 0xe5, 0xd9, 0xfc, 0x33, 0x0e}
DEFINE_GUID!{IID_IDebugSystemObjects4, 0x489468e6, 0x7d0f, 0x4af5,
0x87, 0xab, 0x25, 0x20, 0x74, 0x54, 0xd5, 0x53}
pub type PDEBUG_ADVANCED = *mut IDebugAdvanced;
pub type PDEBUG_ADVANCED2 = *mut IDebugAdvanced2;
pub type PDEBUG_ADVANCED3 = *mut IDebugAdvanced3;
pub type PDEBUG_ADVANCED4 = *mut IDebugAdvanced4;
pub type PDEBUG_BREAKPOINT = *mut IDebugBreakpoint;
pub type PDEBUG_BREAKPOINT2 = *mut IDebugBreakpoint2;
pub type PDEBUG_BREAKPOINT3 = *mut IDebugBreakpoint3;
pub type PDEBUG_CLIENT = *mut IDebugClient;
pub type PDEBUG_CLIENT2 = *mut IDebugClient2;
pub type PDEBUG_CLIENT3 = *mut IDebugClient3;
pub type PDEBUG_CLIENT4 = *mut IDebugClient4;
pub type PDEBUG_CLIENT5 = *mut IDebugClient5;
pub type PDEBUG_CLIENT6 = *mut IDebugClient6;
pub type PDEBUG_CLIENT7 = *mut IDebugClient7;
pub type PDEBUG_PLMCLIENT = *mut IDebugPlmClient;
pub type PDEBUG_PLMCLIENT2 = *mut IDebugPlmClient2;
pub type PDEBUG_PLMCLIENT3 = *mut IDebugPlmClient3;
pub type PDEBUG_OUTPUT_STREAM = *mut IDebugOutputStream;
pub type PDEBUG_CONTROL = *mut IDebugControl;
pub type PDEBUG_CONTROL2 = *mut IDebugControl2;
pub type PDEBUG_CONTROL3 = *mut IDebugControl3;
pub type PDEBUG_CONTROL4 = *mut IDebugControl4;
pub type PDEBUG_CONTROL5 = *mut IDebugControl5;
pub type PDEBUG_CONTROL6 = *mut IDebugControl6;
pub type PDEBUG_CONTROL7 = *mut IDebugControl7;
pub type PDEBUG_DATASPACES = *mut IDebugDataSpaces;
pub type PDEBUG_DATASPACES2 = *mut IDebugDataSpaces2;
pub type PDEBUG_DATASPACES3 = *mut IDebugDataSpaces3;
pub type PDEBUG_DATASPACES4 = *mut IDebugDataSpaces4;
pub type PDEBUG_EVENT_CALLBACKS = *mut IDebugEventCallbacks;
pub type PDEBUG_EVENT_CALLBACKS_WIDE = *mut IDebugEventCallbacksWide;
pub type PDEBUG_EVENT_CONTEXT_CALLBACKS = *mut IDebugEventContextCallbacks;
pub type PDEBUG_INPUT_CALLBACKS = *mut IDebugInputCallbacks;
pub type PDEBUG_OUTPUT_CALLBACKS = *mut IDebugOutputCallbacks;
pub type PDEBUG_OUTPUT_CALLBACKS_WIDE = *mut IDebugOutputCallbacksWide;
pub type PDEBUG_OUTPUT_CALLBACKS2 = *mut IDebugOutputCallbacks2;
pub type PDEBUG_REGISTERS = *mut IDebugRegisters;
pub type PDEBUG_REGISTERS2 = *mut IDebugRegisters2;
pub type PDEBUG_SYMBOL_GROUP = *mut IDebugSymbolGroup;
pub type PDEBUG_SYMBOL_GROUP2 = *mut IDebugSymbolGroup2;
pub type PDEBUG_SYMBOLS = *mut IDebugSymbols;
pub type PDEBUG_SYMBOLS2 = *mut IDebugSymbols2;
pub type PDEBUG_SYMBOLS3 = *mut IDebugSymbols3;
pub type PDEBUG_SYMBOLS4 = *mut IDebugSymbols4;
pub type PDEBUG_SYMBOLS5 = *mut IDebugSymbols5;
pub type PDEBUG_SYSTEM_OBJECTS = *mut IDebugSystemObjects;
pub type PDEBUG_SYSTEM_OBJECTS2 = *mut IDebugSystemObjects2;
pub type PDEBUG_SYSTEM_OBJECTS3 = *mut IDebugSystemObjects3;
pub type PDEBUG_SYSTEM_OBJECTS4 = *mut IDebugSystemObjects4;
extern "system" {
    pub fn DebugConnect(
        RemoteOptions: PCSTR,
        InterfaceId: REFIID,
        Interface: *mut PVOID,
    ) -> HRESULT;
    pub fn DebugConnectWide(
        RemoteOptions: PCWSTR,
        InterfaceId: REFIID,
        Interface: *mut PVOID,
    ) -> HRESULT;
    pub fn DebugCreate(InterfaceId: REFIID, Interface: *mut PVOID) -> HRESULT;
    pub fn DebugCreateEx(
        InterfaceId: REFIID,
        DbgEngOptions: DWORD,
        Interface: *mut PVOID,
    ) -> HRESULT;
}
STRUCT!{struct DEBUG_OFFSET_REGION {
    Base: ULONG64,
    Size: ULONG64,
}}
pub type PDEBUG_OFFSET_REGION = *mut DEBUG_OFFSET_REGION;
RIDL!(#[uuid(0xf2df5f53, 0x071f, 0x47bd, 0x9d, 0xe6, 0x57, 0x34, 0xc3, 0xfe, 0xd6, 0x89)]
interface IDebugAdvanced(IDebugAdvancedVtbl): IUnknown(IUnknownVtbl) {
    fn GetThreadContext(
        Context: PVOID,
        ContextSize: ULONG,
    ) -> HRESULT,
    fn SetThreadContext(
        Context: PVOID,
        ContextSize: ULONG,
    ) -> HRESULT,
});
STRUCT!{struct DEBUG_READ_USER_MINIDUMP_STREAM {
    StreamType: ULONG,
    Flags: ULONG,
    Offset: ULONG64,
    Buffer: PVOID,
    BufferSize: ULONG,
    BufferUsed: ULONG,
}}
pub type PDEBUG_READ_USER_MINIDUMP_STREAM = *mut DEBUG_READ_USER_MINIDUMP_STREAM;
pub const DEBUG_GET_TEXT_COMPLETIONS_NO_DOT_COMMANDS: ULONG = 0x00000001;
pub const DEBUG_GET_TEXT_COMPLETIONS_NO_EXTENSION_COMMANDS: ULONG = 0x00000002;
pub const DEBUG_GET_TEXT_COMPLETIONS_NO_SYMBOLS: ULONG = 0x00000004;
STRUCT!{struct DEBUG_GET_TEXT_COMPLETIONS_IN {
    Flags: ULONG,
    MatchCountLimit: ULONG,
    Reserved: [ULONG64; 3],
}}
pub type PDEBUG_GET_TEXT_COMPLETIONS_IN = *mut DEBUG_GET_TEXT_COMPLETIONS_IN;
pub const DEBUG_GET_TEXT_COMPLETIONS_IS_DOT_COMMAND: ULONG = 0x00000001;
pub const DEBUG_GET_TEXT_COMPLETIONS_IS_EXTENSION_COMMAND: ULONG = 0x00000002;
pub const DEBUG_GET_TEXT_COMPLETIONS_IS_SYMBOL: ULONG = 0x00000004;
STRUCT!{struct DEBUG_GET_TEXT_COMPLETIONS_OUT {
    Flags: ULONG,
    ReplaceIndex: ULONG,
    MatchCount: ULONG,
    Reserved1: ULONG,
    Reserved2: [ULONG64; 2],
}}
pub type PDEBUG_GET_TEXT_COMPLETIONS_OUT = *mut DEBUG_GET_TEXT_COMPLETIONS_OUT;
STRUCT!{struct DEBUG_CACHED_SYMBOL_INFO {
    ModBase: ULONG64,
    Arg1: ULONG64,
    Arg2: ULONG64,
    Id: ULONG,
    Arg3: ULONG,
}}
pub type PDEBUG_CACHED_SYMBOL_INFO = *mut DEBUG_CACHED_SYMBOL_INFO;
pub const DEBUG_REQUEST_SOURCE_PATH_HAS_SOURCE_SERVER: ULONG = 0;
pub const DEBUG_REQUEST_TARGET_EXCEPTION_CONTEXT: ULONG = 1;
pub const DEBUG_REQUEST_TARGET_EXCEPTION_THREAD: ULONG = 2;
pub const DEBUG_REQUEST_TARGET_EXCEPTION_RECORD: ULONG = 3;
pub const DEBUG_REQUEST_GET_ADDITIONAL_CREATE_OPTIONS: ULONG = 4;
pub const DEBUG_REQUEST_SET_ADDITIONAL_CREATE_OPTIONS: ULONG = 5;
pub const DEBUG_REQUEST_GET_WIN32_MAJOR_MINOR_VERSIONS: ULONG = 6;
pub const DEBUG_REQUEST_READ_USER_MINIDUMP_STREAM: ULONG = 7;
pub const DEBUG_REQUEST_TARGET_CAN_DETACH: ULONG = 8;
pub const DEBUG_REQUEST_SET_LOCAL_IMPLICIT_COMMAND_LINE: ULONG = 9;
pub const DEBUG_REQUEST_GET_CAPTURED_EVENT_CODE_OFFSET: ULONG = 10;
pub const DEBUG_REQUEST_READ_CAPTURED_EVENT_CODE_STREAM: ULONG = 11;
pub const DEBUG_REQUEST_EXT_TYPED_DATA_ANSI: ULONG = 12;
pub const DEBUG_REQUEST_GET_EXTENSION_SEARCH_PATH_WIDE: ULONG = 13;
pub const DEBUG_REQUEST_GET_TEXT_COMPLETIONS_WIDE: ULONG = 14;
pub const DEBUG_REQUEST_GET_CACHED_SYMBOL_INFO: ULONG = 15;
pub const DEBUG_REQUEST_ADD_CACHED_SYMBOL_INFO: ULONG = 16;
pub const DEBUG_REQUEST_REMOVE_CACHED_SYMBOL_INFO: ULONG = 17;
pub const DEBUG_REQUEST_GET_TEXT_COMPLETIONS_ANSI: ULONG = 18;
pub const DEBUG_REQUEST_CURRENT_OUTPUT_CALLBACKS_ARE_DML_AWARE: ULONG = 19;
pub const DEBUG_REQUEST_GET_OFFSET_UNWIND_INFORMATION: ULONG = 20;
pub const DEBUG_REQUEST_GET_DUMP_HEADER: ULONG = 21;
pub const DEBUG_REQUEST_SET_DUMP_HEADER: ULONG = 22;
pub const DEBUG_REQUEST_MIDORI: ULONG = 23;
pub const DEBUG_REQUEST_PROCESS_DESCRIPTORS: ULONG = 24;
pub const DEBUG_REQUEST_MISC_INFORMATION: ULONG = 25;
pub const DEBUG_REQUEST_OPEN_PROCESS_TOKEN: ULONG = 26;
pub const DEBUG_REQUEST_OPEN_THREAD_TOKEN: ULONG = 27;
pub const DEBUG_REQUEST_DUPLICATE_TOKEN: ULONG = 28;
pub const DEBUG_REQUEST_QUERY_INFO_TOKEN: ULONG = 29;
pub const DEBUG_REQUEST_CLOSE_TOKEN: ULONG = 30;
pub const DEBUG_REQUEST_WOW_PROCESS: ULONG = 31;
pub const DEBUG_REQUEST_WOW_MODULE: ULONG = 32;
pub const DEBUG_LIVE_USER_NON_INVASIVE: ULONG = 33;
pub const DEBUG_REQUEST_RESUME_THREAD: ULONG = 34;
pub const DEBUG_REQUEST_INLINE_QUERY: ULONG = 35;
pub const DEBUG_REQUEST_TL_INSTRUMENTATION_AWARE: ULONG = 36;
pub const DEBUG_REQUEST_GET_INSTRUMENTATION_VERSION: ULONG = 37;
pub const DEBUG_SRCFILE_SYMBOL_TOKEN: ULONG = 0;
pub const DEBUG_SRCFILE_SYMBOL_TOKEN_SOURCE_COMMAND_WIDE: ULONG = 1;
pub const DEBUG_SRCFILE_SYMBOL_CHECKSUMINFO: ULONG = 2;
pub const DEBUG_SYMINFO_BREAKPOINT_SOURCE_LINE: ULONG = 0;
pub const DEBUG_SYMINFO_IMAGEHLP_MODULEW64: ULONG = 1;
pub const DEBUG_SYMINFO_GET_SYMBOL_NAME_BY_OFFSET_AND_TAG_WIDE: ULONG = 2;
pub const DEBUG_SYMINFO_GET_MODULE_SYMBOL_NAMES_AND_OFFSETS: ULONG = 3;
pub const DEBUG_SYSOBJINFO_THREAD_BASIC_INFORMATION: ULONG = 0;
pub const DEBUG_SYSOBJINFO_THREAD_NAME_WIDE: ULONG = 1;
pub const DEBUG_SYSOBJINFO_CURRENT_PROCESS_COOKIE: ULONG = 2;
pub const DEBUG_TBINFO_EXIT_STATUS: ULONG = 0x00000001;
pub const DEBUG_TBINFO_PRIORITY_CLASS: ULONG = 0x00000002;
pub const DEBUG_TBINFO_PRIORITY: ULONG = 0x00000004;
pub const DEBUG_TBINFO_TIMES: ULONG = 0x00000008;
pub const DEBUG_TBINFO_START_OFFSET: ULONG = 0x00000010;
pub const DEBUG_TBINFO_AFFINITY: ULONG = 0x00000020;
pub const DEBUG_TBINFO_ALL: ULONG = 0x0000003f;
STRUCT!{struct DEBUG_THREAD_BASIC_INFORMATION {
    Valid: ULONG,
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
pub type PDEBUG_THREAD_BASIC_INFORMATION = *mut DEBUG_THREAD_BASIC_INFORMATION;
RIDL!(#[uuid(0x716d14c9, 0x119b, 0x4ba5, 0xaf, 0x1f, 0x08, 0x90, 0xe6, 0x72, 0x41, 0x6a)]
interface IDebugAdvanced2(IDebugAdvanced2Vtbl): IDebugAdvanced(IDebugAdvancedVtbl) {
    fn Request(
        Request: ULONG,
        InBuffer: PVOID,
        InBufferSize: ULONG,
        OutBuffer: PVOID,
        OutBufferSize: ULONG,
        OutSize: PULONG,
    ) -> HRESULT,
    fn GetSourceFileInformation(
        Which: ULONG,
        SourceFile: PSTR,
        Arg64: ULONG64,
        Arg32: ULONG,
        Buffer: PVOID,
        BufferSize: ULONG,
        InfoSize: PULONG,
    ) -> HRESULT,
    fn FindSourceFileAndToken(
        StartElement: ULONG,
        ModAddr: ULONG64,
        File: PCSTR,
        Flags: ULONG,
        FileToken: PVOID,
        FileTokenSize: ULONG,
        FoundElement: PULONG,
        Buffer: PSTR,
        BufferSize: ULONG,
        FoundSize: PULONG,
    ) -> HRESULT,
    fn GetSymbolInformation(
        Which: ULONG,
        Arg64: ULONG64,
        Arg32: ULONG,
        Buffer: PVOID,
        BufferSize: ULONG,
        InfoSize: PULONG,
        StringBuffer: PSTR,
        StringBufferSize: ULONG,
        StringSize: PULONG,
    ) -> HRESULT,
    fn GetSystemObjectInformation(
        Which: ULONG,
        Arg64: ULONG64,
        Arg32: ULONG,
        Buffer: PVOID,
        BufferSize: ULONG,
        InfoSize: PULONG,
    ) -> HRESULT,
});
RIDL!(#[uuid(0xcba4abb4, 0x84c4, 0x444d, 0x87, 0xca, 0xa0, 0x4e, 0x13, 0x28, 0x67, 0x39)]
interface IDebugAdvanced3(IDebugAdvanced3Vtbl): IDebugAdvanced2(IDebugAdvanced2Vtbl) {
    fn GetSourceFileInformationWide(
        Which: ULONG,
        SourceFile: PWSTR,
        Arg64: ULONG64,
        Arg32: ULONG,
        Buffer: PVOID,
        BufferSize: ULONG,
        InfoSize: PULONG,
    ) -> HRESULT,
    fn FindSourceFileAndTokenWide(
        StartElement: ULONG,
        ModAddr: ULONG64,
        File: PCWSTR,
        Flags: ULONG,
        FileToken: PVOID,
        FileTokenSize: ULONG,
        FoundElement: PULONG,
        Buffer: PWSTR,
        BufferSize: ULONG,
        FoundSize: PULONG,
    ) -> HRESULT,
    fn GetSymbolInformationWide(
        Which: ULONG,
        Arg64: ULONG64,
        Arg32: ULONG,
        Buffer: PVOID,
        BufferSize: ULONG,
        InfoSize: PULONG,
        StringBuffer: PWSTR,
        StringBufferSize: ULONG,
        StringSize: PULONG,
    ) -> HRESULT,
});
STRUCT!{struct SYMBOL_INFO_EX {
    SizeOfStruct: ULONG,
    TypeOfInfo: ULONG,
    Offset: ULONG64,
    Line: ULONG,
    Displacement: ULONG,
    Reserved: [ULONG; 4],
}}
pub type PSYMBOL_INFO_EX = *mut SYMBOL_INFO_EX;
RIDL!(#[uuid(0xd1069067, 0x2a65, 0x4bf0, 0xae, 0x97, 0x76, 0x18, 0x4b, 0x67, 0x85, 0x6b)]
interface IDebugAdvanced4(IDebugAdvanced4Vtbl): IDebugAdvanced3(IDebugAdvanced3Vtbl) {
    fn GetSymbolInformationWideEx(
        Which: ULONG,
        Arg64: ULONG64,
        Arg32: ULONG,
        Buffer: PVOID,
        BufferSize: ULONG,
        InfoSize: PULONG,
        StringBuffer: PWSTR,
        StringBufferSize: ULONG,
        StringSize: PULONG,
        pInfoEx: PSYMBOL_INFO_EX,
    ) -> HRESULT,
});
pub const DEBUG_BREAKPOINT_CODE: ULONG = 0;
pub const DEBUG_BREAKPOINT_DATA: ULONG = 1;
pub const DEBUG_BREAKPOINT_TIME: ULONG = 2;
pub const DEBUG_BREAKPOINT_INLINE: ULONG = 3;
pub const DEBUG_BREAKPOINT_GO_ONLY: ULONG = 0x00000001;
pub const DEBUG_BREAKPOINT_DEFERRED: ULONG = 0x00000002;
pub const DEBUG_BREAKPOINT_ENABLED: ULONG = 0x00000004;
pub const DEBUG_BREAKPOINT_ADDER_ONLY: ULONG = 0x00000008;
pub const DEBUG_BREAKPOINT_ONE_SHOT: ULONG = 0x00000010;
pub const DEBUG_BREAK_READ: ULONG = 0x00000001;
pub const DEBUG_BREAK_WRITE: ULONG = 0x00000002;
pub const DEBUG_BREAK_EXECUTE: ULONG = 0x00000004;
pub const DEBUG_BREAK_IO: ULONG = 0x00000008;
STRUCT!{struct DEBUG_BREAKPOINT_PARAMETERS {
    Offset: ULONG64,
    Id: ULONG,
    BreakType: ULONG,
    ProcType: ULONG,
    Flags: ULONG,
    DataSize: ULONG,
    DataAccessType: ULONG,
    PassCount: ULONG,
    CurrentPassCount: ULONG,
    MatchThread: ULONG,
    CommandSize: ULONG,
    OffsetExpressionSize: ULONG,
}}
pub type PDEBUG_BREAKPOINT_PARAMETERS = *mut DEBUG_BREAKPOINT_PARAMETERS;
RIDL!(#[uuid(0x5bd9d474, 0x5975, 0x423a, 0xb8, 0x8b, 0x65, 0xa8, 0xe7, 0x11, 0x0e, 0x65)]
interface IDebugBreakpoint(IDebugBreakpointVtbl): IUnknown(IUnknownVtbl) {
    fn GetId(Id: PULONG,) -> HRESULT,
    fn GetType(
        BreakType: PULONG,
        ProcType: PULONG,
    ) -> HRESULT,
    fn GetAdder(Adder: *mut PDEBUG_CLIENT,) -> HRESULT,
    fn GetFlags(Flags: PULONG,) -> HRESULT,
    fn AddFlags(Flags: ULONG,) -> HRESULT,
    fn RemoveFlags(Flags: ULONG,) -> HRESULT,
    fn SetFlags(Flags: ULONG,) -> HRESULT,
    fn GetOffset(Offset: PULONG64,) -> HRESULT,
    fn SetOffset(Offset: ULONG64,) -> HRESULT,
    fn GetDataParameters(
        Size: PULONG,
        AccessType: PULONG,
    ) -> HRESULT,
    fn SetDataParameters(
        Size: ULONG,
        AccessType: ULONG,
    ) -> HRESULT,
    fn GetPassCount(Count: PULONG,) -> HRESULT,
    fn SetPassCount(Count: ULONG,) -> HRESULT,
    fn GetCurrentPassCount(Count: PULONG,) -> HRESULT,
    fn GetMatchThreadId(Id: PULONG,) -> HRESULT,
    fn SetMatchThreadId(Thread: ULONG,) -> HRESULT,
    fn GetCommand(
        Buffer: PSTR,
        BufferSize: ULONG,
        CommandSize: PULONG,
    ) -> HRESULT,
    fn SetCommand(Command: PCSTR,) -> HRESULT,
    fn GetOffsetExpression(
        Buffer: PSTR,
        BufferSize: ULONG,
        ExpressionSize: PULONG,
    ) -> HRESULT,
    fn SetOffsetExpression(Expression: PCSTR,) -> HRESULT,
    fn GetParameters(Params: PDEBUG_BREAKPOINT_PARAMETERS,) -> HRESULT,
});
RIDL!(#[uuid(0x1b278d20, 0x79f2, 0x426e, 0xa3, 0xf9, 0xc1, 0xdd, 0xf3, 0x75, 0xd4, 0x8e)]
interface IDebugBreakpoint2(IDebugBreakpoint2Vtbl): IDebugBreakpoint(IDebugBreakpointVtbl) {
    fn GetCommandWide(
        Buffer: PWSTR,
        BufferSize: ULONG,
        CommandSize: PULONG,
    ) -> HRESULT,
    fn SetCommandWide(Command: PCWSTR,) -> HRESULT,
    fn GetOffsetExpressionWide(
        Buffer: PWSTR,
        BufferSize: ULONG,
        ExpressionSize: PULONG,
    ) -> HRESULT,
    fn SetOffsetExpressionWide(Expression: PCWSTR,) -> HRESULT,
});
RIDL!(#[uuid(0x38f5c249, 0xb448, 0x43bb, 0x98, 0x35, 0x57, 0x9d, 0x4e, 0xc0, 0x22, 0x49)]
interface IDebugBreakpoint3(IDebugBreakpoint3Vtbl): IDebugBreakpoint2(IDebugBreakpoint2Vtbl) {
    fn GetGuid(Guid: LPGUID,) -> HRESULT,
});
pub const DEBUG_ATTACH_KERNEL_CONNECTION: ULONG = 0x00000000;
pub const DEBUG_ATTACH_LOCAL_KERNEL: ULONG = 0x00000001;
pub const DEBUG_ATTACH_EXDI_DRIVER: ULONG = 0x00000002;
pub const DEBUG_ATTACH_INSTALL_DRIVER: ULONG = 0x00000004;
pub const DEBUG_GET_PROC_DEFAULT: ULONG = 0x00000000;
pub const DEBUG_GET_PROC_FULL_MATCH: ULONG = 0x00000001;
pub const DEBUG_GET_PROC_ONLY_MATCH: ULONG = 0x00000002;
pub const DEBUG_GET_PROC_SERVICE_NAME: ULONG = 0x00000004;
pub const DEBUG_PROC_DESC_DEFAULT: ULONG = 0x00000000;
pub const DEBUG_PROC_DESC_NO_PATHS: ULONG = 0x00000001;
pub const DEBUG_PROC_DESC_NO_SERVICES: ULONG = 0x00000002;
pub const DEBUG_PROC_DESC_NO_MTS_PACKAGES: ULONG = 0x00000004;
pub const DEBUG_PROC_DESC_NO_COMMAND_LINE: ULONG = 0x00000008;
pub const DEBUG_PROC_DESC_NO_SESSION_ID: ULONG = 0x00000010;
pub const DEBUG_PROC_DESC_NO_USER_NAME: ULONG = 0x00000020;
pub const DEBUG_ATTACH_DEFAULT: ULONG = 0x00000000;
pub const DEBUG_ATTACH_NONINVASIVE: ULONG = 0x00000001;
pub const DEBUG_ATTACH_EXISTING: ULONG = 0x00000002;
pub const DEBUG_ATTACH_NONINVASIVE_NO_SUSPEND: ULONG = 0x00000004;
pub const DEBUG_ATTACH_INVASIVE_NO_INITIAL_BREAK: ULONG = 0x00000008;
pub const DEBUG_ATTACH_INVASIVE_RESUME_PROCESS: ULONG = 0x00000010;
pub const DEBUG_ATTACH_NONINVASIVE_ALLOW_PARTIAL: ULONG = 0x00000020;
pub const DEBUG_CREATE_PROCESS_NO_DEBUG_HEAP: ULONG = CREATE_UNICODE_ENVIRONMENT;
pub const DEBUG_CREATE_PROCESS_THROUGH_RTL: ULONG = STACK_SIZE_PARAM_IS_A_RESERVATION;
pub const DEBUG_ECREATE_PROCESS_DEFAULT: ULONG = 0x00000000;
pub const DEBUG_ECREATE_PROCESS_INHERIT_HANDLES: ULONG = 0x00000001;
pub const DEBUG_ECREATE_PROCESS_USE_VERIFIER_FLAGS: ULONG = 0x00000002;
pub const DEBUG_ECREATE_PROCESS_USE_IMPLICIT_COMMAND_LINE: ULONG = 0x00000004;
STRUCT!{struct DEBUG_CREATE_PROCESS_OPTIONS {
    CreateFlags: ULONG,
    EngCreateFlags: ULONG,
    VerifierFlags: ULONG,
    Reserved: ULONG,
}}
pub type PDEBUG_CREATE_PROCESS_OPTIONS = *mut DEBUG_CREATE_PROCESS_OPTIONS;
pub const DEBUG_PROCESS_DETACH_ON_EXIT: ULONG = 0x00000001;
pub const DEBUG_PROCESS_ONLY_THIS_PROCESS: ULONG = 0x00000002;
pub const DEBUG_CONNECT_SESSION_DEFAULT: ULONG = 0x00000000;
pub const DEBUG_CONNECT_SESSION_NO_VERSION: ULONG = 0x00000001;
pub const DEBUG_CONNECT_SESSION_NO_ANNOUNCE: ULONG = 0x00000002;
pub const DEBUG_SERVERS_DEBUGGER: ULONG = 0x00000001;
pub const DEBUG_SERVERS_PROCESS: ULONG = 0x00000002;
pub const DEBUG_SERVERS_ALL: ULONG = 0x00000003;
pub const DEBUG_END_PASSIVE: ULONG = 0x00000000;
pub const DEBUG_END_ACTIVE_TERMINATE: ULONG = 0x00000001;
pub const DEBUG_END_ACTIVE_DETACH: ULONG = 0x00000002;
pub const DEBUG_END_REENTRANT: ULONG = 0x00000003;
pub const DEBUG_END_DISCONNECT: ULONG = 0x00000004;
pub const DEBUG_OUTPUT_NORMAL: ULONG = 0x00000001;
pub const DEBUG_OUTPUT_ERROR: ULONG = 0x00000002;
pub const DEBUG_OUTPUT_WARNING: ULONG = 0x00000004;
pub const DEBUG_OUTPUT_VERBOSE: ULONG = 0x00000008;
pub const DEBUG_OUTPUT_PROMPT: ULONG = 0x00000010;
pub const DEBUG_OUTPUT_PROMPT_REGISTERS: ULONG = 0x00000020;
pub const DEBUG_OUTPUT_EXTENSION_WARNING: ULONG = 0x00000040;
pub const DEBUG_OUTPUT_DEBUGGEE: ULONG = 0x00000080;
pub const DEBUG_OUTPUT_DEBUGGEE_PROMPT: ULONG = 0x00000100;
pub const DEBUG_OUTPUT_SYMBOLS: ULONG = 0x00000200;
pub const DEBUG_OUTPUT_STATUS: ULONG = 0x00000400;
pub const DEBUG_IOUTPUT_KD_PROTOCOL: ULONG = 0x80000000;
pub const DEBUG_IOUTPUT_REMOTING: ULONG = 0x40000000;
pub const DEBUG_IOUTPUT_BREAKPOINT: ULONG = 0x20000000;
pub const DEBUG_IOUTPUT_EVENT: ULONG = 0x10000000;
pub const DEBUG_IOUTPUT_ADDR_TRANSLATE: ULONG = 0x08000000;
pub const DEBUG_OUTPUT_IDENTITY_DEFAULT: ULONG = 0x00000000;
pub const DEBUG_CLIENT_UNKNOWN: ULONG = 0x0;
pub const DEBUG_CLIENT_VSINT: ULONG = 0x1;
pub const DEBUG_CLIENT_NTSD: ULONG = 0x2;
pub const DEBUG_CLIENT_NTKD: ULONG = 0x3;
pub const DEBUG_CLIENT_CDB: ULONG = 0x4;
pub const DEBUG_CLIENT_KD: ULONG = 0x5;
pub const DEBUG_CLIENT_WINDBG: ULONG = 0x6;
pub const DEBUG_CLIENT_WINIDE: ULONG = 0x7;
STRUCT!{struct DEBUG_CLIENT_CONTEXT {
    cbSize: UINT,
    eClient: UINT,
}}
pub type PDEBUG_CLIENT_CONTEXT = *mut DEBUG_CLIENT_CONTEXT;
RIDL!(#[uuid(0x27fe5639, 0x8407, 0x4f47, 0x83, 0x64, 0xee, 0x11, 0x8f, 0xb0, 0x8a, 0xc8)]
interface IDebugClient(IDebugClientVtbl): IUnknown(IUnknownVtbl) {
    fn AttachKernel(
        Flags: ULONG,
        ConnectOptions: PCSTR,
    ) -> HRESULT,
    fn GetKernelConnectionOptions(
        Buffer: PSTR,
        BufferSize: ULONG,
        OptionsSize: PULONG,
    ) -> HRESULT,
    fn SetKernelConnectionOptions(Options: PCSTR,) -> HRESULT,
    fn StartProcessServer(
        Flags: ULONG,
        Options: PCSTR,
        Reserved: PVOID,
    ) -> HRESULT,
    fn ConnectProcessServer(
        RemoteOptions: PCSTR,
        Server: PULONG64,
    ) -> HRESULT,
    fn DisconnectProcessServer(Server: ULONG64,) -> HRESULT,
    fn GetRunningProcessSystemIds(
        Server: ULONG64,
        Ids: PULONG,
        Count: ULONG,
        ActualCount: PULONG,
    ) -> HRESULT,
    fn GetRunningProcessSystemIdByExecutableName(
        Server: ULONG64,
        ExeName: PCSTR,
        Flags: ULONG,
        Id: PULONG,
    ) -> HRESULT,
    fn GetRunningProcessDescription(
        Server: ULONG64,
        SystemId: ULONG,
        Flags: ULONG,
        ExeName: PSTR,
        ExeNameSize: ULONG,
        ActualExeNameSize: PULONG,
        Description: PSTR,
        DescriptionSize: ULONG,
        ActualDescriptionSize: PULONG,
    ) -> HRESULT,
    fn AttachProcess(
        Server: ULONG64,
        ProcessId: ULONG,
        AttachFlags: ULONG,
    ) -> HRESULT,
    fn CreateProcess(
        Server: ULONG64,
        CommandLine: PSTR,
        CreateFlags: ULONG,
    ) -> HRESULT,
    fn CreateProcessAndAttach(
        Server: ULONG64,
        CommandLine: PSTR,
        CreateFlags: ULONG,
        ProcessId: ULONG,
        AttachFlags: ULONG,
    ) -> HRESULT,
    fn GetProcessOptions(Options: PULONG,) -> HRESULT,
    fn AddProcessOptions(Options: ULONG,) -> HRESULT,
    fn RemoveProcessOptions(Options: ULONG,) -> HRESULT,
    fn SetProcessOptions(Options: ULONG,) -> HRESULT,
    fn OpenDumpFile(DumpFile: PCSTR,) -> HRESULT,
    fn WriteDumpFile(
        DumpFile: PCSTR,
        Qualifier: ULONG,
    ) -> HRESULT,
    fn ConnectSession(
        Flags: ULONG,
        HistoryLimit: ULONG,
    ) -> HRESULT,
    fn StartServer(Options: PCSTR,) -> HRESULT,
    fn OutputServers(
        OutputControl: ULONG,
        Machine: PCSTR,
        Flags: ULONG,
    ) -> HRESULT,
    fn TerminateProcesses() -> HRESULT,
    fn DetachProcesses() -> HRESULT,
    fn EndSession(Flags: ULONG,) -> HRESULT,
    fn GetExitCode(Code: PULONG,) -> HRESULT,
    fn DispatchCallbacks(Timeout: ULONG,) -> HRESULT,
    fn ExitDispatch(Client: PDEBUG_CLIENT,) -> HRESULT,
    fn CreateClient(Client: *mut PDEBUG_CLIENT,) -> HRESULT,
    fn GetInputCallbacks(Callbacks: *mut PDEBUG_INPUT_CALLBACKS,) -> HRESULT,
    fn SetInputCallbacks(Callbacks: PDEBUG_INPUT_CALLBACKS,) -> HRESULT,
    fn GetOutputCallbacks(Callbacks: *mut PDEBUG_OUTPUT_CALLBACKS,) -> HRESULT,
    fn SetOutputCallbacks(Callbacks: PDEBUG_OUTPUT_CALLBACKS,) -> HRESULT,
    fn GetOutputMask(Mask: PULONG,) -> HRESULT,
    fn SetOutputMask(Mask: ULONG,) -> HRESULT,
    fn GetOtherOutputMask(
        Client: PDEBUG_CLIENT,
        Mask: PULONG,
    ) -> HRESULT,
    fn SetOtherOutputMask(
        Client: PDEBUG_CLIENT,
        Mask: ULONG,
    ) -> HRESULT,
    fn GetOutputWidth(Columns: PULONG,) -> HRESULT,
    fn SetOutputWidth(Columns: ULONG,) -> HRESULT,
    fn GetOutputLinePrefix(
        Buffer: PSTR,
        BufferSize: ULONG,
        PrefixSize: PULONG,
    ) -> HRESULT,
    fn SetOutputLinePrefix(Prefix: PCSTR,) -> HRESULT,
    fn GetIdentity(
        Buffer: PSTR,
        BufferSize: ULONG,
        IdentitySize: PULONG,
    ) -> HRESULT,
    fn OutputIdentity(
        OutputControl: ULONG,
        Flags: ULONG,
        Format: PCSTR,
    ) -> HRESULT,
    fn GetEventCallbacks(Callbacks: *mut PDEBUG_EVENT_CALLBACKS,) -> HRESULT,
    fn SetEventCallbacks(Callbacks: PDEBUG_EVENT_CALLBACKS,) -> HRESULT,
    fn FlushCallbacks() -> HRESULT,
});
pub const DEBUG_FORMAT_DEFAULT: ULONG = 0x00000000;
pub const DEBUG_FORMAT_CAB_SECONDARY_ALL_IMAGES: ULONG = 0x10000000;
pub const DEBUG_FORMAT_WRITE_CAB: ULONG = 0x20000000;
pub const DEBUG_FORMAT_CAB_SECONDARY_FILES: ULONG = 0x40000000;
pub const DEBUG_FORMAT_NO_OVERWRITE: ULONG = 0x80000000;
pub const DEBUG_FORMAT_USER_SMALL_FULL_MEMORY: ULONG = 0x00000001;
pub const DEBUG_FORMAT_USER_SMALL_HANDLE_DATA: ULONG = 0x00000002;
pub const DEBUG_FORMAT_USER_SMALL_UNLOADED_MODULES: ULONG = 0x00000004;
pub const DEBUG_FORMAT_USER_SMALL_INDIRECT_MEMORY: ULONG = 0x00000008;
pub const DEBUG_FORMAT_USER_SMALL_DATA_SEGMENTS: ULONG = 0x00000010;
pub const DEBUG_FORMAT_USER_SMALL_FILTER_MEMORY: ULONG = 0x00000020;
pub const DEBUG_FORMAT_USER_SMALL_FILTER_PATHS: ULONG = 0x00000040;
pub const DEBUG_FORMAT_USER_SMALL_PROCESS_THREAD_DATA: ULONG = 0x00000080;
pub const DEBUG_FORMAT_USER_SMALL_PRIVATE_READ_WRITE_MEMORY: ULONG = 0x00000100;
pub const DEBUG_FORMAT_USER_SMALL_NO_OPTIONAL_DATA: ULONG = 0x00000200;
pub const DEBUG_FORMAT_USER_SMALL_FULL_MEMORY_INFO: ULONG = 0x00000400;
pub const DEBUG_FORMAT_USER_SMALL_THREAD_INFO: ULONG = 0x00000800;
pub const DEBUG_FORMAT_USER_SMALL_CODE_SEGMENTS: ULONG = 0x00001000;
pub const DEBUG_FORMAT_USER_SMALL_NO_AUXILIARY_STATE: ULONG = 0x00002000;
pub const DEBUG_FORMAT_USER_SMALL_FULL_AUXILIARY_STATE: ULONG = 0x00004000;
pub const DEBUG_FORMAT_USER_SMALL_MODULE_HEADERS: ULONG = 0x00008000;
pub const DEBUG_FORMAT_USER_SMALL_FILTER_TRIAGE: ULONG = 0x00010000;
pub const DEBUG_FORMAT_USER_SMALL_ADD_AVX_XSTATE_CONTEXT: ULONG = 0x00020000;
pub const DEBUG_FORMAT_USER_SMALL_IGNORE_INACCESSIBLE_MEM: ULONG = 0x08000000;
pub const DEBUG_DUMP_FILE_BASE: ULONG = 0xffffffff;
pub const DEBUG_DUMP_FILE_PAGE_FILE_DUMP: ULONG = 0x00000000;
RIDL!(#[uuid(0xedbed635, 0x372e, 0x4dab, 0xbb, 0xfe, 0xed, 0x0d, 0x2f, 0x63, 0xbe, 0x81)]
interface IDebugClient2(IDebugClient2Vtbl): IDebugClient(IDebugClientVtbl) {
    fn WriteDumpFile2(
        DumpFile: PCSTR,
        Qualifier: ULONG,
        FormatFlags: ULONG,
        Comment: PCSTR,
    ) -> HRESULT,
    fn AddDumpInformationFile(
        InfoFile: PCSTR,
        Type: ULONG,
    ) -> HRESULT,
    fn EndProcessServer(Server: ULONG64,) -> HRESULT,
    fn WaitForProcessServerEnd(Timeout: ULONG,) -> HRESULT,
    fn IsKernelDebuggerEnabled() -> HRESULT,
    fn TerminateCurrentProcess() -> HRESULT,
    fn DetachCurrentProcess() -> HRESULT,
    fn AbandonCurrentProcess() -> HRESULT,
});
RIDL!(#[uuid(0xdd492d7f, 0x71b8, 0x4ad6, 0xa8, 0xdc, 0x1c, 0x88, 0x74, 0x79, 0xff, 0x91)]
interface IDebugClient3(IDebugClient3Vtbl): IDebugClient2(IDebugClient2Vtbl) {
    fn GetRunningProcessSystemIdByExecutableNameWide(
        Server: ULONG64,
        ExeName: PCWSTR,
        Flags: ULONG,
        Id: PULONG,
    ) -> HRESULT,
    fn GetRunningProcessDescriptionWide(
        Server: ULONG64,
        SystemId: ULONG,
        Flags: ULONG,
        ExeName: PWSTR,
        ExeNameSize: ULONG,
        ActualExeNameSize: PULONG,
        Description: PWSTR,
        DescriptionSize: ULONG,
        ActualDescriptionSize: PULONG,
    ) -> HRESULT,
    fn CreateProcessWide(
        Server: ULONG64,
        CommandLine: PWSTR,
        CreateFlags: ULONG,
    ) -> HRESULT,
    fn CreateProcessAndAttachWide(
        Server: ULONG64,
        CommandLine: PWSTR,
        CreateFlags: ULONG,
        ProcessId: ULONG,
        AttachFlags: ULONG,
    ) -> HRESULT,
});
pub const DEBUG_DUMP_FILE_LOAD_FAILED_INDEX: ULONG = 0xffffffff;
pub const DEBUG_DUMP_FILE_ORIGINAL_CAB_INDEX: ULONG = 0xfffffffe;
RIDL!(#[uuid(0xca83c3de, 0x5089, 0x4cf8, 0x93, 0xc8, 0xd8, 0x92, 0x38, 0x7f, 0x2a, 0x5e)]
interface IDebugClient4(IDebugClient4Vtbl): IDebugClient3(IDebugClient3Vtbl) {
    fn OpenDumpFileWide(
        FileName: PCWSTR,
        FileHandle: ULONG64,
    ) -> HRESULT,
    fn WriteDumpFileWide(
        FileName: PCWSTR,
        FileHandle: ULONG64,
        Qualifier: ULONG,
        FormatFlags: ULONG,
        Comment: PCWSTR,
    ) -> HRESULT,
    fn AddDumpInformationFileWide(
        FileName: PCWSTR,
        FileHandle: ULONG64,
        Type: ULONG,
    ) -> HRESULT,
    fn GetNumberDumpFiles(Number: PULONG,) -> HRESULT,
    fn GetDumpFile(
        Index: ULONG,
        Buffer: PSTR,
        BufferSize: ULONG,
        NameSize: PULONG,
        Handle: PULONG64,
        Type: PULONG,
    ) -> HRESULT,
    fn GetDumpFileWide(
        Index: ULONG,
        Buffer: PWSTR,
        BufferSize: ULONG,
        NameSize: PULONG,
        Handle: PULONG64,
        Type: PULONG,
    ) -> HRESULT,
});
RIDL!(#[uuid(0xe3acb9d7, 0x7ec2, 0x4f0c, 0xa0, 0xda, 0xe8, 0x1e, 0x0c, 0xbb, 0xe6, 0x28)]
interface IDebugClient5(IDebugClient5Vtbl): IDebugClient4(IDebugClient4Vtbl) {
    fn AttachKernelWide(
        Flags: ULONG,
        ConnectOptions: PCWSTR,
    ) -> HRESULT,
    fn GetKernelConnectionOptionsWide(
        Buffer: PWSTR,
        BufferSize: ULONG,
        OptionsSize: PULONG,
    ) -> HRESULT,
    fn SetKernelConnectionOptionsWide(Options: PCWSTR,) -> HRESULT,
    fn StartProcessServerWide(
        Flags: ULONG,
        Options: PCWSTR,
        Reserved: PVOID,
    ) -> HRESULT,
    fn ConnectProcessServerWide(
        RemoteOptions: PCWSTR,
        Server: PULONG64,
    ) -> HRESULT,
    fn StartServerWide(Options: PCWSTR,) -> HRESULT,
    fn OutputServersWide(
        OutputControl: ULONG,
        Machine: PCWSTR,
        Flags: ULONG,
    ) -> HRESULT,
    fn GetOutputCallbacksWide(Callbacks: *mut PDEBUG_OUTPUT_CALLBACKS_WIDE,) -> HRESULT,
    fn SetOutputCallbacksWide(Callbacks: PDEBUG_OUTPUT_CALLBACKS_WIDE,) -> HRESULT,
    fn GetOutputLinePrefixWide(
        Buffer: PWSTR,
        BufferSize: ULONG,
        PrefixSize: PULONG,
    ) -> HRESULT,
    fn SetOutputLinePrefixWide(Prefix: PCWSTR,) -> HRESULT,
    fn GetIdentityWide(
        Buffer: PWSTR,
        BufferSize: ULONG,
        IdentitySize: PULONG,
    ) -> HRESULT,
    fn OutputIdentityWide(
        OutputControl: ULONG,
        Flags: ULONG,
        Format: PCWSTR,
    ) -> HRESULT,
    fn GetEventCallbacksWide(Callbacks: *mut PDEBUG_EVENT_CALLBACKS_WIDE,) -> HRESULT,
    fn SetEventCallbacksWide(Callbacks: PDEBUG_EVENT_CALLBACKS_WIDE,) -> HRESULT,
    fn CreateProcess2(
        Server: ULONG64,
        CommandLine: PSTR,
        OptionsBuffer: PVOID,
        OptionsBufferSize: ULONG,
        InitialDirectory: PCSTR,
        Environment: PCSTR,
    ) -> HRESULT,
    fn CreateProcess2Wide(
        Server: ULONG64,
        CommandLine: PWSTR,
        OptionsBuffer: PVOID,
        OptionsBufferSize: ULONG,
        InitialDirectory: PCWSTR,
        Environment: PCWSTR,
    ) -> HRESULT,
    fn CreateProcessAndAttach2(
        Server: ULONG64,
        CommandLine: PSTR,
        OptionsBuffer: PVOID,
        OptionsBufferSize: ULONG,
        InitialDirectory: PCSTR,
        Environment: PCSTR,
        ProcessId: ULONG,
        AttachFlags: ULONG,
    ) -> HRESULT,
    fn CreateProcessAndAttach2Wide(
        Server: ULONG64,
        CommandLine: PWSTR,
        OptionsBuffer: PVOID,
        OptionsBufferSize: ULONG,
        InitialDirectory: PCWSTR,
        Environment: PCWSTR,
        ProcessId: ULONG,
        AttachFlags: ULONG,
    ) -> HRESULT,
    fn PushOutputLinePrefix(
        NewPrefix: PCSTR,
        Handle: PULONG64,
    ) -> HRESULT,
    fn PushOutputLinePrefixWide(
        NewPrefix: PCWSTR,
        Handle: PULONG64,
    ) -> HRESULT,
    fn PopOutputLinePrefix(Handle: ULONG64,) -> HRESULT,
    fn GetNumberInputCallbacks(Count: PULONG,) -> HRESULT,
    fn GetNumberOutputCallbacks(Count: PULONG,) -> HRESULT,
    fn GetNumberEventCallbacks(
        EventFlags: ULONG,
        Count: PULONG,
    ) -> HRESULT,
    fn GetQuitLockString(
        Buffer: PSTR,
        BufferSize: ULONG,
        StringSize: PULONG,
    ) -> HRESULT,
    fn SetQuitLockString(String: PCSTR,) -> HRESULT,
    fn GetQuitLockStringWide(
        Buffer: PWSTR,
        BufferSize: ULONG,
        StringSize: PULONG,
    ) -> HRESULT,
    fn SetQuitLockStringWide(String: PCWSTR,) -> HRESULT,
});
RIDL!(#[uuid(0xfd28b4c5, 0xc498, 0x4686, 0xa2, 0x8e, 0x62, 0xca, 0xd2, 0x15, 0x4e, 0xb3)]
interface IDebugClient6(IDebugClient6Vtbl): IDebugClient5(IDebugClient5Vtbl) {
    fn SetEventContextCallbacks(Callbacks: PDEBUG_EVENT_CONTEXT_CALLBACKS,) -> HRESULT,
});
RIDL!(#[uuid(0x13586be3, 0x542e, 0x481e, 0xb1, 0xf2, 0x84, 0x97, 0xba, 0x74, 0xf9, 0xa9)]
interface IDebugClient7(IDebugClient7Vtbl): IDebugClient6(IDebugClient6Vtbl) {
    fn SetClientContext(
        Context: PVOID,
        ContextSize: ULONG,
    ) -> HRESULT,
});
RIDL!(#[uuid(0xa02b66c4, 0xaea3, 0x4234, 0xa9, 0xf7, 0xfe, 0x4c, 0x38, 0x3d, 0x4e, 0x29)]
interface IDebugPlmClient(IDebugPlmClientVtbl): IUnknown(IUnknownVtbl) {
    fn LaunchPlmPackageForDebugWide(
        Server: ULONG64,
        Timeout: ULONG,
        PackageFullName: PCWSTR,
        AppName: PCWSTR,
        Arguments: PCWSTR,
        ProcessId: PULONG,
        ThreadId: PULONG,
    ) -> HRESULT,
});
RIDL!(#[uuid(0x597c980d, 0xe7bd, 0x4309, 0x96, 0x2c, 0x9d, 0x9b, 0x69, 0xa7, 0x37, 0x2c)]
interface IDebugPlmClient2(IDebugPlmClient2Vtbl): IDebugPlmClient(IDebugPlmClientVtbl) {
    fn LaunchPlmBgTaskForDebugWide(
        Server: ULONG64,
        Timeout: ULONG,
        PackageFullName: PCWSTR,
        BackgroundTaskId: PCWSTR,
        ProcessId: PULONG,
        ThreadId: PULONG,
    ) -> HRESULT,
});
RIDL!(#[uuid(0xcdf48669, 0x901f, 0x4791, 0xb8, 0x68, 0x7d, 0x2c, 0xb3, 0xa2, 0xd7, 0xfc)]
interface IDebugPlmClient3(IDebugPlmClient3Vtbl): IDebugPlmClient2(IDebugPlmClient2Vtbl) {
    fn QueryPlmPackageWide(
        Server: ULONG64,
        PackageFullName: PCWSTR,
        Stream: PDEBUG_OUTPUT_STREAM,
    ) -> HRESULT,
    fn QueryPlmPackageList(
        Server: ULONG64,
        Stream: PDEBUG_OUTPUT_STREAM,
    ) -> HRESULT,
    fn EnablePlmPackageDebugWide(
        Server: ULONG64,
        PackageFullName: PCWSTR,
    ) -> HRESULT,
    fn DisablePlmPackageDebugWide(
        Server: ULONG64,
        PackageFullName: PCWSTR,
    ) -> HRESULT,
    fn SuspendPlmPackageWide(
        Server: ULONG64,
        PackageFullName: PCWSTR,
    ) -> HRESULT,
    fn ResumePlmPackageWide(
        Server: ULONG64,
        PackageFullName: PCWSTR,
    ) -> HRESULT,
    fn TerminatePlmPackageWide(
        Server: ULONG64,
        PackageFullName: PCWSTR,
    ) -> HRESULT,
    fn LaunchAndDebugPlmAppWide(
        Server: ULONG64,
        PackageFullName: PCWSTR,
        AppName: PCWSTR,
        Arguments: PCWSTR,
    ) -> HRESULT,
    fn ActivateAndDebugPlmBgTaskWide(
        Server: ULONG64,
        PackageFullName: PCWSTR,
        BackgroundTaskId: PCWSTR,
    ) -> HRESULT,
});
RIDL!(#[uuid(0x7782d8f2, 0x2b85, 0x4059, 0xab, 0x88, 0x28, 0xce, 0xdd, 0xca, 0x1c, 0x80)]
interface IDebugOutputStream(IDebugOutputStreamVtbl): IUnknown(IUnknownVtbl) {
    fn Write(psz: PCWSTR,) -> HRESULT,
});
pub const DEBUG_STATUS_NO_CHANGE: ULONG = 0;
pub const DEBUG_STATUS_GO: ULONG = 1;
pub const DEBUG_STATUS_GO_HANDLED: ULONG = 2;
pub const DEBUG_STATUS_GO_NOT_HANDLED: ULONG = 3;
pub const DEBUG_STATUS_STEP_OVER: ULONG = 4;
pub const DEBUG_STATUS_STEP_INTO: ULONG = 5;
pub const DEBUG_STATUS_BREAK: ULONG = 6;
pub const DEBUG_STATUS_NO_DEBUGGEE: ULONG = 7;
pub const DEBUG_STATUS_STEP_BRANCH: ULONG = 8;
pub const DEBUG_STATUS_IGNORE_EVENT: ULONG = 9;
pub const DEBUG_STATUS_RESTART_REQUESTED: ULONG = 10;
pub const DEBUG_STATUS_REVERSE_GO: ULONG = 11;
pub const DEBUG_STATUS_REVERSE_STEP_BRANCH: ULONG = 12;
pub const DEBUG_STATUS_REVERSE_STEP_OVER: ULONG = 13;
pub const DEBUG_STATUS_REVERSE_STEP_INTO: ULONG = 14;
pub const DEBUG_STATUS_OUT_OF_SYNC: ULONG = 15;
pub const DEBUG_STATUS_WAIT_INPUT: ULONG = 16;
pub const DEBUG_STATUS_TIMEOUT: ULONG = 17;
pub const DEBUG_STATUS_MASK: ULONG = 0x1f;
pub const DEBUG_STATUS_INSIDE_WAIT: ULONG = 0x100000000;
pub const DEBUG_STATUS_WAIT_TIMEOUT: ULONG = 0x200000000;
pub const DEBUG_OUTCTL_THIS_CLIENT: ULONG = 0x00000000;
pub const DEBUG_OUTCTL_ALL_CLIENTS: ULONG = 0x00000001;
pub const DEBUG_OUTCTL_ALL_OTHER_CLIENTS: ULONG = 0x00000002;
pub const DEBUG_OUTCTL_IGNORE: ULONG = 0x00000003;
pub const DEBUG_OUTCTL_LOG_ONLY: ULONG = 0x00000004;
pub const DEBUG_OUTCTL_SEND_MASK: ULONG = 0x00000007;
pub const DEBUG_OUTCTL_NOT_LOGGED: ULONG = 0x00000008;
pub const DEBUG_OUTCTL_OVERRIDE_MASK: ULONG = 0x00000010;
pub const DEBUG_OUTCTL_DML: ULONG = 0x00000020;
pub const DEBUG_OUTCTL_AMBIENT_DML: ULONG = 0xfffffffe;
pub const DEBUG_OUTCTL_AMBIENT_TEXT: ULONG = 0xffffffff;
pub const DEBUG_OUTCTL_AMBIENT: ULONG = DEBUG_OUTCTL_AMBIENT_TEXT;
pub const DEBUG_INTERRUPT_ACTIVE: ULONG = 0;
pub const DEBUG_INTERRUPT_PASSIVE: ULONG = 1;
pub const DEBUG_INTERRUPT_EXIT: ULONG = 2;
pub const DEBUG_CURRENT_DEFAULT: ULONG = 0x0000000f;
pub const DEBUG_CURRENT_SYMBOL: ULONG = 0x00000001;
pub const DEBUG_CURRENT_DISASM: ULONG = 0x00000002;
pub const DEBUG_CURRENT_REGISTERS: ULONG = 0x00000004;
pub const DEBUG_CURRENT_SOURCE_LINE: ULONG = 0x00000008;
pub const DEBUG_DISASM_EFFECTIVE_ADDRESS: ULONG = 0x00000001;
pub const DEBUG_DISASM_MATCHING_SYMBOLS: ULONG = 0x00000002;
pub const DEBUG_DISASM_SOURCE_LINE_NUMBER: ULONG = 0x00000004;
pub const DEBUG_DISASM_SOURCE_FILE_NAME: ULONG = 0x00000008;
pub const DEBUG_LEVEL_SOURCE: ULONG = 0;
pub const DEBUG_LEVEL_ASSEMBLY: ULONG = 1;
pub const DEBUG_ENGOPT_IGNORE_DBGHELP_VERSION: ULONG = 0x00000001;
pub const DEBUG_ENGOPT_IGNORE_EXTENSION_VERSIONS: ULONG = 0x00000002;
pub const DEBUG_ENGOPT_ALLOW_NETWORK_PATHS: ULONG = 0x00000004;
pub const DEBUG_ENGOPT_DISALLOW_NETWORK_PATHS: ULONG = 0x00000008;
pub const DEBUG_ENGOPT_NETWORK_PATHS: ULONG = (0x00000004 | 0x00000008);
pub const DEBUG_ENGOPT_IGNORE_LOADER_EXCEPTIONS: ULONG = 0x00000010;
pub const DEBUG_ENGOPT_INITIAL_BREAK: ULONG = 0x00000020;
pub const DEBUG_ENGOPT_INITIAL_MODULE_BREAK: ULONG = 0x00000040;
pub const DEBUG_ENGOPT_FINAL_BREAK: ULONG = 0x00000080;
pub const DEBUG_ENGOPT_NO_EXECUTE_REPEAT: ULONG = 0x00000100;
pub const DEBUG_ENGOPT_FAIL_INCOMPLETE_INFORMATION: ULONG = 0x00000200;
pub const DEBUG_ENGOPT_ALLOW_READ_ONLY_BREAKPOINTS: ULONG = 0x00000400;
pub const DEBUG_ENGOPT_SYNCHRONIZE_BREAKPOINTS: ULONG = 0x00000800;
pub const DEBUG_ENGOPT_DISALLOW_SHELL_COMMANDS: ULONG = 0x00001000;
pub const DEBUG_ENGOPT_KD_QUIET_MODE: ULONG = 0x00002000;
pub const DEBUG_ENGOPT_DISABLE_MANAGED_SUPPORT: ULONG = 0x00004000;
pub const DEBUG_ENGOPT_DISABLE_MODULE_SYMBOL_LOAD: ULONG = 0x00008000;
pub const DEBUG_ENGOPT_DISABLE_EXECUTION_COMMANDS: ULONG = 0x00010000;
pub const DEBUG_ENGOPT_DISALLOW_IMAGE_FILE_MAPPING: ULONG = 0x00020000;
pub const DEBUG_ENGOPT_PREFER_DML: ULONG = 0x00040000;
pub const DEBUG_ENGOPT_DISABLESQM: ULONG = 0x00080000;
pub const DEBUG_ENGOPT_DISABLE_STEPLINES_OPTIONS: ULONG = 0x00200000;
pub const DEBUG_ENGOPT_ALL: ULONG = 0x002FFFFF;
pub const DEBUG_ANY_ID: ULONG = 0xffffffff;
STRUCT!{struct DEBUG_STACK_FRAME {
    InstructionOffset: ULONG64,
    ReturnOffset: ULONG64,
    FrameOffset: ULONG64,
    StackOffset: ULONG64,
    FuncTableEntry: ULONG64,
    Params: [ULONG64; 4],
    Reserved: [ULONG64; 6],
    Virtual: BOOL,
    FrameNumber: ULONG,
}}
pub type PDEBUG_STACK_FRAME = *mut DEBUG_STACK_FRAME;
pub const DBG_FRAME_DEFAULT: ULONG = 0;
pub const DBG_FRAME_IGNORE_INLINE: ULONG = 0xFFFFFFFF;
STRUCT!{struct DEBUG_STACK_FRAME_EX {
    InstructionOffset: ULONG64,
    ReturnOffset: ULONG64,
    FrameOffset: ULONG64,
    StackOffset: ULONG64,
    FuncTableEntry: ULONG64,
    Params: [ULONG64; 4],
    Reserved: [ULONG64; 6],
    Virtual: BOOL,
    FrameNumber: ULONG,
    InlineFrameContext: ULONG,
    Reserved1: ULONG,
}}
pub type PDEBUG_STACK_FRAME_EX = *mut DEBUG_STACK_FRAME_EX;
pub const STACK_FRAME_TYPE_INIT: ULONG = 0x00;
pub const STACK_FRAME_TYPE_STACK: ULONG = 0x01;
pub const STACK_FRAME_TYPE_INLINE: ULONG = 0x02;
pub const STACK_FRAME_TYPE_RA: ULONG = 0x80;
pub const STACK_FRAME_TYPE_IGNORE: ULONG = 0xFF;
STRUCT!{struct INLINE_FRAME_CONTEXT_s {
    FrameId: BYTE,
    FrameType: BYTE,
    FrameSignature: WORD,
}}
UNION!{union INLINE_FRAME_CONTEXT {
    [u32; 1],
    ContextValue ContextValue_mut: DWORD,
    s s_mut: INLINE_FRAME_CONTEXT_s,
}}
STRUCT!{struct STACK_SRC_INFO {
    ImagePath: PCWSTR,
    ModuleName: PCWSTR,
    Function: PCWSTR,
    Displacement: ULONG,
    Row: ULONG,
    Column: ULONG,
}}
pub type PSTACK_SRC_INFO = *mut STACK_SRC_INFO;
STRUCT!{struct STACK_SYM_FRAME_INFO {
    StackFrameEx: DEBUG_STACK_FRAME_EX,
    SrcInfo: STACK_SRC_INFO,
}}
pub type PSTACK_SYM_FRAME_INFO = *mut STACK_SYM_FRAME_INFO;
pub const DEBUG_STACK_ARGUMENTS: ULONG = 0x00000001;
pub const DEBUG_STACK_FUNCTION_INFO: ULONG = 0x00000002;
pub const DEBUG_STACK_SOURCE_LINE: ULONG = 0x00000004;
pub const DEBUG_STACK_FRAME_ADDRESSES: ULONG = 0x00000008;
pub const DEBUG_STACK_COLUMN_NAMES: ULONG = 0x00000010;
pub const DEBUG_STACK_NONVOLATILE_REGISTERS: ULONG = 0x00000020;
pub const DEBUG_STACK_FRAME_NUMBERS: ULONG = 0x00000040;
pub const DEBUG_STACK_PARAMETERS: ULONG = 0x00000080;
pub const DEBUG_STACK_FRAME_ADDRESSES_RA_ONLY: ULONG = 0x00000100;
pub const DEBUG_STACK_FRAME_MEMORY_USAGE: ULONG = 0x00000200;
pub const DEBUG_STACK_PARAMETERS_NEWLINE: ULONG = 0x00000400;
pub const DEBUG_STACK_DML: ULONG = 0x00000800;
pub const DEBUG_STACK_FRAME_OFFSETS: ULONG = 0x00001000;
pub const DEBUG_STACK_PROVIDER: ULONG = 0x00002000;
pub const DEBUG_CLASS_UNINITIALIZED: ULONG = 0;
pub const DEBUG_CLASS_KERNEL: ULONG = 1;
pub const DEBUG_CLASS_USER_WINDOWS: ULONG = 2;
pub const DEBUG_CLASS_IMAGE_FILE: ULONG = 3;
pub const DEBUG_DUMP_SMALL: ULONG = 1024;
pub const DEBUG_DUMP_DEFAULT: ULONG = 1025;
pub const DEBUG_DUMP_FULL: ULONG = 1026;
pub const DEBUG_DUMP_IMAGE_FILE: ULONG = 1027;
pub const DEBUG_DUMP_TRACE_LOG: ULONG = 1028;
pub const DEBUG_DUMP_WINDOWS_CE: ULONG = 1029;
pub const DEBUG_KERNEL_CONNECTION: ULONG = 0;
pub const DEBUG_KERNEL_LOCAL: ULONG = 1;
pub const DEBUG_KERNEL_EXDI_DRIVER: ULONG = 2;
pub const DEBUG_KERNEL_IDNA: ULONG = 3;
pub const DEBUG_KERNEL_INSTALL_DRIVER: ULONG = 4;
pub const DEBUG_KERNEL_SMALL_DUMP: ULONG = DEBUG_DUMP_SMALL;
pub const DEBUG_KERNEL_DUMP: ULONG = DEBUG_DUMP_DEFAULT;
pub const DEBUG_KERNEL_FULL_DUMP: ULONG = DEBUG_DUMP_FULL;
pub const DEBUG_KERNEL_TRACE_LOG: ULONG = DEBUG_DUMP_TRACE_LOG;
pub const DEBUG_USER_WINDOWS_PROCESS: ULONG = 0;
pub const DEBUG_USER_WINDOWS_PROCESS_SERVER: ULONG = 1;
pub const DEBUG_USER_WINDOWS_IDNA: ULONG = 2;
pub const DEBUG_USER_WINDOWS_SMALL_DUMP: ULONG = DEBUG_DUMP_SMALL;
pub const DEBUG_USER_WINDOWS_DUMP: ULONG = DEBUG_DUMP_DEFAULT;
pub const DEBUG_USER_WINDOWS_DUMP_WINDOWS_CE: ULONG = DEBUG_DUMP_WINDOWS_CE;
pub const DEBUG_EXTENSION_AT_ENGINE: ULONG = 0x00000000;
pub const DEBUG_EXECUTE_DEFAULT: ULONG = 0x00000000;
pub const DEBUG_EXECUTE_ECHO: ULONG = 0x00000001;
pub const DEBUG_EXECUTE_NOT_LOGGED: ULONG = 0x00000002;
pub const DEBUG_EXECUTE_NO_REPEAT: ULONG = 0x00000004;
pub const DEBUG_EXECUTE_USER_TYPED: ULONG = 0x00000008;
pub const DEBUG_EXECUTE_USER_CLICKED: ULONG = 0x00000010;
pub const DEBUG_EXECUTE_EXTENSION: ULONG = 0x00000020;
pub const DEBUG_EXECUTE_INTERNAL: ULONG = 0x00000040;
pub const DEBUG_EXECUTE_SCRIPT: ULONG = 0x00000080;
pub const DEBUG_EXECUTE_TOOLBAR: ULONG = 0x00000100;
pub const DEBUG_EXECUTE_MENU: ULONG = 0x00000200;
pub const DEBUG_EXECUTE_HOTKEY: ULONG = 0x00000400;
pub const DEBUG_EXECUTE_EVENT: ULONG = 0x00000800;
pub const DEBUG_FILTER_CREATE_THREAD: ULONG = 0x00000000;
pub const DEBUG_FILTER_EXIT_THREAD: ULONG = 0x00000001;
pub const DEBUG_FILTER_CREATE_PROCESS: ULONG = 0x00000002;
pub const DEBUG_FILTER_EXIT_PROCESS: ULONG = 0x00000003;
pub const DEBUG_FILTER_LOAD_MODULE: ULONG = 0x00000004;
pub const DEBUG_FILTER_UNLOAD_MODULE: ULONG = 0x00000005;
pub const DEBUG_FILTER_SYSTEM_ERROR: ULONG = 0x00000006;
pub const DEBUG_FILTER_INITIAL_BREAKPOINT: ULONG = 0x00000007;
pub const DEBUG_FILTER_INITIAL_MODULE_LOAD: ULONG = 0x00000008;
pub const DEBUG_FILTER_DEBUGGEE_OUTPUT: ULONG = 0x00000009;
pub const DEBUG_FILTER_BREAK: ULONG = 0x00000000;
pub const DEBUG_FILTER_SECOND_CHANCE_BREAK: ULONG = 0x00000001;
pub const DEBUG_FILTER_OUTPUT: ULONG = 0x00000002;
pub const DEBUG_FILTER_IGNORE: ULONG = 0x00000003;
pub const DEBUG_FILTER_REMOVE: ULONG = 0x00000004;
pub const DEBUG_FILTER_GO_HANDLED: ULONG = 0x00000000;
pub const DEBUG_FILTER_GO_NOT_HANDLED: ULONG = 0x00000001;
STRUCT!{struct DEBUG_SPECIFIC_FILTER_PARAMETERS {
    ExecutionOption: ULONG,
    ContinueOption: ULONG,
    TextSize: ULONG,
    CommandSize: ULONG,
    ArgumentSize: ULONG,
}}
pub type PDEBUG_SPECIFIC_FILTER_PARAMETERS = *mut DEBUG_SPECIFIC_FILTER_PARAMETERS;
STRUCT!{struct DEBUG_EXCEPTION_FILTER_PARAMETERS {
    ExecutionOption: ULONG,
    ContinueOption: ULONG,
    TextSize: ULONG,
    CommandSize: ULONG,
    SecondCommandSize: ULONG,
    ExceptionCode: ULONG,
}}
pub type PDEBUG_EXCEPTION_FILTER_PARAMETERS = *mut DEBUG_EXCEPTION_FILTER_PARAMETERS;
pub const DEBUG_WAIT_DEFAULT: ULONG = 0x00000000;
STRUCT!{struct DEBUG_LAST_EVENT_INFO_BREAKPOINT {
    Id: ULONG,
}}
pub type PDEBUG_LAST_EVENT_INFO_BREAKPOINT = *mut DEBUG_LAST_EVENT_INFO_BREAKPOINT;
STRUCT!{struct DEBUG_LAST_EVENT_INFO_EXCEPTION {
    ExceptionRecord: EXCEPTION_RECORD64,
    FirstChance: ULONG,
}}
pub type PDEBUG_LAST_EVENT_INFO_EXCEPTION = *mut DEBUG_LAST_EVENT_INFO_EXCEPTION;
STRUCT!{struct DEBUG_LAST_EVENT_INFO_EXIT_THREAD {
    ExitCode: ULONG,
}}
pub type PDEBUG_LAST_EVENT_INFO_EXIT_THREAD = *mut DEBUG_LAST_EVENT_INFO_EXIT_THREAD;
STRUCT!{struct DEBUG_LAST_EVENT_INFO_EXIT_PROCESS {
    ExitCode: ULONG,
}}
pub type PDEBUG_LAST_EVENT_INFO_EXIT_PROCESS = *mut DEBUG_LAST_EVENT_INFO_EXIT_PROCESS;
STRUCT!{struct DEBUG_LAST_EVENT_INFO_LOAD_MODULE {
    Base: ULONG64,
}}
pub type PDEBUG_LAST_EVENT_INFO_LOAD_MODULE = *mut DEBUG_LAST_EVENT_INFO_LOAD_MODULE;
STRUCT!{struct DEBUG_LAST_EVENT_INFO_UNLOAD_MODULE {
    Base: ULONG64,
}}
pub type PDEBUG_LAST_EVENT_INFO_UNLOAD_MODULE = *mut DEBUG_LAST_EVENT_INFO_UNLOAD_MODULE;
STRUCT!{struct DEBUG_LAST_EVENT_INFO_SYSTEM_ERROR {
    Error: ULONG,
    Level: ULONG,
}}
pub type PDEBUG_LAST_EVENT_INFO_SYSTEM_ERROR = *mut DEBUG_LAST_EVENT_INFO_SYSTEM_ERROR;
pub const DEBUG_VALUE_INVALID: ULONG = 0;
pub const DEBUG_VALUE_INT8: ULONG = 1;
pub const DEBUG_VALUE_INT16: ULONG = 2;
pub const DEBUG_VALUE_INT32: ULONG = 3;
pub const DEBUG_VALUE_INT64: ULONG = 4;
pub const DEBUG_VALUE_FLOAT32: ULONG = 5;
pub const DEBUG_VALUE_FLOAT64: ULONG = 6;
pub const DEBUG_VALUE_FLOAT80: ULONG = 7;
pub const DEBUG_VALUE_FLOAT82: ULONG = 8;
pub const DEBUG_VALUE_FLOAT128: ULONG = 9;
pub const DEBUG_VALUE_VECTOR64: ULONG = 10;
pub const DEBUG_VALUE_VECTOR128: ULONG = 11;
pub const DEBUG_VALUE_TYPES: ULONG = 12;
STRUCT!{struct DEBUG_VALUE_u_s1 {
    I64: ULONG64,
    Nat: BOOL,
}}
STRUCT!{struct DEBUG_VALUE_u_s2 {
    LowPart: ULONG,
    HighPart: ULONG,
}}
STRUCT!{struct DEBUG_VALUE_u_s3 {
    LowPart: ULONG64,
    HighPart: LONG64,
}}
UNION!{union DEBUG_VALUE_u {
    [u64; 4],
    I8 I8_mut: UCHAR,
    I16 I16_mut: USHORT,
    I32 I32_mut: ULONG,
    s1 s1_mut: DEBUG_VALUE_u_s1,
    F32 F32_mut: c_float,
    F64 F64_mut: c_double,
    F80Bytes F80Bytes_mut: [UCHAR; 10],
    F82Bytes F82Bytes_mut: [UCHAR; 11],
    F128Bytes F128Bytes_mut: [UCHAR; 16],
    VI8 VI8_mut: [UCHAR; 16],
    VI16 VI16_mut: [USHORT; 8],
    VI32 VI32_mut: [ULONG; 4],
    VI64 VI64_mut: [ULONG64; 2],
    VF32 VF32_mut: [c_float; 4],
    VF64 VF64_mut: [c_double; 2],
    I64Parts32 I64Parts32_mut: DEBUG_VALUE_u_s2,
    F128Parts64 F128Parts64_mut: DEBUG_VALUE_u_s3,
    RawBytes RawBytes_mut: [UCHAR; 24],
}}
STRUCT!{struct DEBUG_VALUE {
    u: DEBUG_VALUE_u,
    TailOfRawBytes: ULONG,
    Type: ULONG,
}}
pub type PDEBUG_VALUE = *mut DEBUG_VALUE;
RIDL!(#[uuid(0x5182e668, 0x105e, 0x416e, 0xad, 0x92, 0x24, 0xef, 0x80, 0x04, 0x24, 0xba)]
interface IDebugControl(IDebugControlVtbl): IUnknown(IUnknownVtbl) {
    fn GetInterrupt() -> HRESULT,
    fn SetInterrupt(Flags: ULONG,) -> HRESULT,
    fn GetInterruptTimeout(Seconds: PULONG,) -> HRESULT,
    fn SetInterruptTimeout(Seconds: ULONG,) -> HRESULT,
    fn GetLogFile(
        Buffer: PSTR,
        BufferSize: ULONG,
        FileSize: PULONG,
        Append: PBOOL,
    ) -> HRESULT,
    fn OpenLogFile(
        File: PCSTR,
        Append: BOOL,
    ) -> HRESULT,
    fn CloseLogFile() -> HRESULT,
    fn GetLogMask(Mask: PULONG,) -> HRESULT,
    fn SetLogMask(Mask: ULONG,) -> HRESULT,
    fn Input(
        Buffer: PSTR,
        BufferSize: ULONG,
        InputSize: PULONG,
    ) -> HRESULT,
    fn ReturnInput(Buffer: PCSTR,) -> HRESULT,
    fn Output_fixme_varargs(
        Mask: ULONG,
        Format: PCSTR,
        //FIXME ...
    ) -> HRESULT,
    fn OutputVaList(
        Mask: ULONG,
        Format: PCSTR,
        Args: va_list,
    ) -> HRESULT,
    fn ControlledOutput_fixme_varargs(
        OutputControl: ULONG,
        Mask: ULONG,
        Format: PCSTR,
        //FIXME ...
    ) -> HRESULT,
    fn ControlledOutputVaList(
        OutputControl: ULONG,
        Mask: ULONG,
        Format: PCSTR,
        Args: va_list,
    ) -> HRESULT,
    fn OutputPrompt_fixme_varargs(
        OutputControl: ULONG,
        Format: PCSTR,
        //FIXME ...
    ) -> HRESULT,
    fn OutputPromptVaList(
        OutputControl: ULONG,
        Format: PCSTR,
        Args: va_list,
    ) -> HRESULT,
    fn GetPromptText(
        Buffer: PSTR,
        BufferSize: ULONG,
        TextSize: PULONG,
    ) -> HRESULT,
    fn OutputCurrentState(
        OutputControl: ULONG,
        Flags: ULONG,
    ) -> HRESULT,
    fn OutputVersionInformation(OutputControl: ULONG,) -> HRESULT,
    fn GetNotifyEventHandle(Handle: PULONG64,) -> HRESULT,
    fn SetNotifyEventHandle(Handle: ULONG64,) -> HRESULT,
    fn Assemble(
        Offset: ULONG64,
        Instr: PCSTR,
        EndOffset: PULONG64,
    ) -> HRESULT,
    fn Disassemble(
        Offset: ULONG64,
        Flags: ULONG,
        Buffer: PSTR,
        BufferSize: ULONG,
        DisassemblySize: PULONG,
        EndOffset: PULONG64,
    ) -> HRESULT,
    fn GetDisassembleEffectiveOffset(Offset: PULONG64,) -> HRESULT,
    fn OutputDisassembly(
        OutputControl: ULONG,
        Offset: ULONG64,
        Flags: ULONG,
        EndOffset: PULONG64,
    ) -> HRESULT,
    fn OutputDisassemblyLines(
        OutputControl: ULONG,
        PreviousLines: ULONG,
        TotalLines: ULONG,
        Offset: ULONG64,
        Flags: ULONG,
        OffsetLine: PULONG,
        StartOffset: PULONG64,
        EndOffset: PULONG64,
        LineOffsets: PULONG64,
    ) -> HRESULT,
    fn GetNearInstruction(
        Offset: ULONG64,
        Delta: LONG,
        NearOffset: PULONG64,
    ) -> HRESULT,
    fn GetStackTrace(
        FrameOffset: ULONG64,
        StackOffset: ULONG64,
        InstructionOffset: ULONG64,
        Frames: PDEBUG_STACK_FRAME,
        FramesSize: ULONG,
        FramesFilled: PULONG,
    ) -> HRESULT,
    fn GetReturnOffset(Offset: PULONG64,) -> HRESULT,
    fn OutputStackTrace(
        OutputControl: ULONG,
        Frames: PDEBUG_STACK_FRAME,
        FramesSize: ULONG,
        Flags: ULONG,
    ) -> HRESULT,
    fn GetDebuggeeType(
        Class: PULONG,
        Qualifier: PULONG,
    ) -> HRESULT,
    fn GetActualProcessorType(Type: PULONG,) -> HRESULT,
    fn GetExecutingProcessorType(Type: PULONG,) -> HRESULT,
    fn GetNumberPossibleExecutingProcessorTypes(Number: PULONG,) -> HRESULT,
    fn GetPossibleExecutingProcessorTypes(
        Start: ULONG,
        Count: ULONG,
        Types: PULONG,
    ) -> HRESULT,
    fn GetNumberProcessors(Number: PULONG,) -> HRESULT,
    fn GetSystemVersion(
        PlatformId: PULONG,
        Major: PULONG,
        Minor: PULONG,
        ServicePackString: PSTR,
        ServicePackStringSize: ULONG,
        ServicePackStringUsed: PULONG,
        ServicePackNumber: PULONG,
        BuildString: PSTR,
        BuildStringSize: ULONG,
        BuildStringUsed: PULONG,
    ) -> HRESULT,
    fn GetPageSize(Size: PULONG,) -> HRESULT,
    fn IsPointer64Bit() -> HRESULT,
    fn ReadBugCheckData(
        Code: PULONG,
        Arg1: PULONG64,
        Arg2: PULONG64,
        Arg3: PULONG64,
        Arg4: PULONG64,
    ) -> HRESULT,
    fn GetNumberSupportedProcessorTypes(Number: PULONG,) -> HRESULT,
    fn GetSupportedProcessorTypes(
        Start: ULONG,
        Count: ULONG,
        Types: PULONG,
    ) -> HRESULT,
    fn GetProcessorTypeNames(
        Type: ULONG,
        FullNameBuffer: PSTR,
        FullNameBufferSize: ULONG,
        FullNameSize: PULONG,
        AbbrevNameBuffer: PSTR,
        AbbrevNameBufferSize: ULONG,
        AbbrevNameSize: PULONG,
    ) -> HRESULT,
    fn GetEffectiveProcessorType(Type: PULONG,) -> HRESULT,
    fn SetEffectiveProcessorType(Type: ULONG,) -> HRESULT,
    fn GetExecutionStatus(Status: PULONG,) -> HRESULT,
    fn SetExecutionStatus(Status: ULONG,) -> HRESULT,
    fn GetCodeLevel(Level: PULONG,) -> HRESULT,
    fn SetCodeLevel(Level: ULONG,) -> HRESULT,
    fn GetEngineOptions(Options: PULONG,) -> HRESULT,
    fn AddEngineOptions(Options: ULONG,) -> HRESULT,
    fn RemoveEngineOptions(Options: ULONG,) -> HRESULT,
    fn SetEngineOptions(Options: ULONG,) -> HRESULT,
    fn GetSystemErrorControl(
        OutputLevel: PULONG,
        BreakLevel: PULONG,
    ) -> HRESULT,
    fn SetSystemErrorControl(
        OutputLevel: ULONG,
        BreakLevel: ULONG,
    ) -> HRESULT,
    fn GetTextMacro(
        Slot: ULONG,
        Buffer: PSTR,
        BufferSize: ULONG,
        MacroSize: PULONG,
    ) -> HRESULT,
    fn SetTextMacro(
        Slot: ULONG,
        Macro: PCSTR,
    ) -> HRESULT,
    fn GetRadix(Radix: PULONG,) -> HRESULT,
    fn SetRadix(Radix: ULONG,) -> HRESULT,
    fn Evaluate(
        Expression: PCSTR,
        DesiredType: ULONG,
        Value: PDEBUG_VALUE,
        RemainderIndex: PULONG,
    ) -> HRESULT,
    fn CoerceValue(
        In: PDEBUG_VALUE,
        OutType: ULONG,
        Out: PDEBUG_VALUE,
    ) -> HRESULT,
    fn CoerceValues(
        Count: ULONG,
        In: PDEBUG_VALUE,
        OutTypes: PULONG,
        Out: PDEBUG_VALUE,
    ) -> HRESULT,
    fn Execute(
        OutputControl: ULONG,
        Command: PCSTR,
        Flags: ULONG,
    ) -> HRESULT,
    fn ExecuteCommandFile(
        OutputControl: ULONG,
        CommandFile: PCSTR,
        Flags: ULONG,
    ) -> HRESULT,
    fn GetNumberBreakpoints(Number: PULONG,) -> HRESULT,
    fn GetBreakpointByIndex(
        Index: ULONG,
        Bp: *mut PDEBUG_BREAKPOINT,
    ) -> HRESULT,
    fn GetBreakpointById(
        Id: ULONG,
        Bp: *mut PDEBUG_BREAKPOINT,
    ) -> HRESULT,
    fn GetBreakpointParameters(
        Count: ULONG,
        Ids: PULONG,
        Start: ULONG,
        Params: PDEBUG_BREAKPOINT_PARAMETERS,
    ) -> HRESULT,
    fn AddBreakpoint(
        Type: ULONG,
        DesiredId: ULONG,
        Bp: *mut PDEBUG_BREAKPOINT,
    ) -> HRESULT,
    fn RemoveBreakpoint(
        Bp: PDEBUG_BREAKPOINT,
    ) -> HRESULT,
    fn AddExtension(
        Path: PCSTR,
        Flags: ULONG,
        Handle: PULONG64,
    ) -> HRESULT,
    fn RemoveExtension(Handle: ULONG64,) -> HRESULT,
    fn GetExtensionByPath(
        Path: PCSTR,
        Handle: PULONG64,
    ) -> HRESULT,
    fn CallExtension(
        Handle: ULONG64,
        Function: PCSTR,
        Arguments: PCSTR,
    ) -> HRESULT,
    fn GetExtensionFunction(
        Handle: ULONG64,
        FuncName: PCSTR,
        Function: *mut FARPROC,
    ) -> HRESULT,
    fn GetWindbgExtensionApis32(
        Api: PWINDBG_EXTENSION_APIS32,
    ) -> HRESULT,
    fn GetWindbgExtensionApis64(
        Api: PWINDBG_EXTENSION_APIS64,
    ) -> HRESULT,
    fn GetNumberEventFilters(
        SpecificEvents: PULONG,
        SpecificExceptions: PULONG,
        ArbitraryExceptions: PULONG,
    ) -> HRESULT,
    fn GetEventFilterText(
        Index: ULONG,
        Buffer: PSTR,
        BufferSize: ULONG,
        TextSize: PULONG,
    ) -> HRESULT,
    fn GetEventFilterCommand(
        Index: ULONG,
        Buffer: PSTR,
        BufferSize: ULONG,
        CommandSize: PULONG,
    ) -> HRESULT,
    fn SetEventFilterCommand(
        Index: ULONG,
        Command: PCSTR,
    ) -> HRESULT,
    fn GetSpecificFilterParameters(
        Start: ULONG,
        Count: ULONG,
        Params: PDEBUG_SPECIFIC_FILTER_PARAMETERS,
    ) -> HRESULT,
    fn SetSpecificFilterParameters(
        Start: ULONG,
        Count: ULONG,
        Params: PDEBUG_SPECIFIC_FILTER_PARAMETERS,
    ) -> HRESULT,
    fn GetSpecificFilterArgument(
        Index: ULONG,
        Buffer: PSTR,
        BufferSize: ULONG,
        ArgumentSize: PULONG,
    ) -> HRESULT,
    fn SetSpecificFilterArgument(
        Index: ULONG,
        Argument: PCSTR,
    ) -> HRESULT,
    fn GetExceptionFilterParameters(
        Count: ULONG,
        Codes: PULONG,
        Start: ULONG,
        Params: PDEBUG_EXCEPTION_FILTER_PARAMETERS,
    ) -> HRESULT,
    fn SetExceptionFilterParameters(
        Count: ULONG,
        Params: PDEBUG_EXCEPTION_FILTER_PARAMETERS,
    ) -> HRESULT,
    fn GetExceptionFilterSecondCommand(
        Index: ULONG,
        Buffer: PSTR,
        BufferSize: ULONG,
        CommandSize: PULONG,
    ) -> HRESULT,
    fn SetExceptionFilterSecondCommand(
        Index: ULONG,
        Command: PCSTR,
    ) -> HRESULT,
    fn WaitForEvent(
        Flags: ULONG,
        Timeout: ULONG,
    ) -> HRESULT,
    fn GetLastEventInformation(
        Type: PULONG,
        ProcessId: PULONG,
        ThreadId: PULONG,
        ExtraInformation: PVOID,
        ExtraInformationSize: ULONG,
        ExtraInformationUsed: PULONG,
        Description: PSTR,
        DescriptionSize: ULONG,
        DescriptionUsed: PULONG,
    ) -> HRESULT,
});
pub const DEBUG_OUT_TEXT_REPL_DEFAULT: ULONG = 0x00000000;
RIDL!(#[uuid(0xd4366723, 0x44df, 0x4bed, 0x8c, 0x7e, 0x4c, 0x05, 0x42, 0x4f, 0x45, 0x88)]
interface IDebugControl2(IDebugControl2Vtbl): IDebugControl(IDebugControlVtbl) {
    fn GetCurrentTimeDate(TimeDate: PULONG,) -> HRESULT,
    fn GetCurrentSystemUpTime(UpTime: PULONG,) -> HRESULT,
    fn GetDumpFormatFlags(FormatFlags: PULONG,) -> HRESULT,
    fn GetNumberTextReplacements(NumRepl: PULONG,) -> HRESULT,
    fn GetTextReplacement(
        SrcText: PCSTR,
        Index: ULONG,
        SrcBuffer: PSTR,
        SrcBufferSize: ULONG,
        SrcSize: PULONG,
        DstBuffer: PSTR,
        DstBufferSize: ULONG,
        DstSize: PULONG,
    ) -> HRESULT,
    fn SetTextReplacement(
        SrcText: PCSTR,
        DstText: PCSTR,
    ) -> HRESULT,
    fn RemoveTextReplacements() -> HRESULT,
    fn OutputTextReplacements(
        OutputControl: ULONG,
        Flags: ULONG,
    ) -> HRESULT,
});
pub const DEBUG_ASMOPT_DEFAULT: ULONG = 0x00000000;
pub const DEBUG_ASMOPT_VERBOSE: ULONG = 0x00000001;
pub const DEBUG_ASMOPT_NO_CODE_BYTES: ULONG = 0x00000002;
pub const DEBUG_ASMOPT_IGNORE_OUTPUT_WIDTH: ULONG = 0x00000004;
pub const DEBUG_ASMOPT_SOURCE_LINE_NUMBER: ULONG = 0x00000008;
pub const DEBUG_EXPR_MASM: ULONG = 0x00000000;
pub const DEBUG_EXPR_CPLUSPLUS: ULONG = 0x00000001;
pub const DEBUG_EINDEX_NAME: ULONG = 0x00000000;
pub const DEBUG_EINDEX_FROM_START: ULONG = 0x00000000;
pub const DEBUG_EINDEX_FROM_END: ULONG = 0x00000001;
pub const DEBUG_EINDEX_FROM_CURRENT: ULONG = 0x00000002;
RIDL!(#[uuid(0x7df74a86, 0xb03f, 0x407f, 0x90, 0xab, 0xa2, 0x0d, 0xad, 0xce, 0xad, 0x08)]
interface IDebugControl3(IDebugControl3Vtbl): IDebugControl2(IDebugControl2Vtbl) {
    fn GetAssemblyOptions(Options: PULONG,) -> HRESULT,
    fn AddAssemblyOptions(Options: ULONG,) -> HRESULT,
    fn RemoveAssemblyOptions(Options: ULONG,) -> HRESULT,
    fn SetAssemblyOptions(Options: ULONG,) -> HRESULT,
    fn GetExpressionSyntax(Flags: PULONG,) -> HRESULT,
    fn SetExpressionSyntax(Flags: ULONG,) -> HRESULT,
    fn SetExpressionSyntaxByName(AbbrevName: PCSTR,) -> HRESULT,
    fn GetNumberExpressionSyntaxes(Number: PULONG,) -> HRESULT,
    fn GetExpressionSyntaxNames(
        Index: ULONG,
        FullNameBuffer: PSTR,
        FullNameBufferSize: ULONG,
        FullNameSize: PULONG,
        AbbrevNameBuffer: PSTR,
        AbbrevNameBufferSize: ULONG,
        AbbrevNameSize: PULONG,
    ) -> HRESULT,
    fn GetNumberEvents(Events: PULONG,) -> HRESULT,
    fn GetEventIndexDescription(
        Index: ULONG,
        Which: ULONG,
        Buffer: PSTR,
        BufferSize: ULONG,
        DescSize: PULONG,
    ) -> HRESULT,
    fn GetCurrentEventIndex(Index: PULONG,) -> HRESULT,
    fn SetNextEventIndex(
        Relation: ULONG,
        Value: ULONG,
        NextIndex: PULONG,
    ) -> HRESULT,
});
pub const DEBUG_LOG_DEFAULT: ULONG = 0x00000000;
pub const DEBUG_LOG_APPEND: ULONG = 0x00000001;
pub const DEBUG_LOG_UNICODE: ULONG = 0x00000002;
pub const DEBUG_LOG_DML: ULONG = 0x00000004;
pub const DEBUG_SYSVERSTR_SERVICE_PACK: ULONG = 0x00000000;
pub const DEBUG_SYSVERSTR_BUILD: ULONG = 0x00000001;
pub const DEBUG_MANAGED_DISABLED: ULONG = 0x00000000;
pub const DEBUG_MANAGED_ALLOWED: ULONG = 0x00000001;
pub const DEBUG_MANAGED_DLL_LOADED: ULONG = 0x00000002;
pub const DEBUG_MANSTR_NONE: ULONG = 0x00000000;
pub const DEBUG_MANSTR_LOADED_SUPPORT_DLL: ULONG = 0x00000001;
pub const DEBUG_MANSTR_LOAD_STATUS: ULONG = 0x00000002;
pub const DEBUG_MANRESET_DEFAULT: ULONG = 0x00000000;
pub const DEBUG_MANRESET_LOAD_DLL: ULONG = 0x00000001;
RIDL!(#[uuid(0x94e60ce9, 0x9b41, 0x4b19, 0x9f, 0xc0, 0x6d, 0x9e, 0xb3, 0x52, 0x72, 0xb3)]
interface IDebugControl4(IDebugControl4Vtbl): IDebugControl3(IDebugControl3Vtbl) {
    fn GetLogFileWide(
        Buffer: PWSTR,
        BufferSize: ULONG,
        FileSize: PULONG,
        Append: PBOOL,
    ) -> HRESULT,
    fn OpenLogFileWide(
        File: PCWSTR,
        Append: BOOL,
    ) -> HRESULT,
    fn InputWide(
        Buffer: PWSTR,
        BufferSize: ULONG,
        InputSize: PULONG,
    ) -> HRESULT,
    fn ReturnInputWide(Buffer: PCWSTR,) -> HRESULT,
    fn OutputWide_fixme_varargs(
        Mask: ULONG,
        Format: PCWSTR,
        //FIXME ...
    ) -> HRESULT,
    fn OutputVaListWide(
        Mask: ULONG,
        Format: PCWSTR,
        Args: va_list,
    ) -> HRESULT,
    fn ControlledOutputWide_fixme_varargs(
        OutputControl: ULONG,
        Mask: ULONG,
        Format: PCWSTR,
        //FIXME ...
    ) -> HRESULT,
    fn ControlledOutputVaListWide(
        OutputControl: ULONG,
        Mask: ULONG,
        Format: PCWSTR,
        Args: va_list,
    ) -> HRESULT,
    fn OutputPromptWide_fixme_varargs(
        OutputControl: ULONG,
        Format: PCWSTR,
        //FIXME ...
    ) -> HRESULT,
    fn OutputPromptVaListWide(
        OutputControl: ULONG,
        Format: PCWSTR,
        Args: va_list,
    ) -> HRESULT,
    fn GetPromptTextWide(
        Buffer: PWSTR,
        BufferSize: ULONG,
        TextSize: PULONG,
    ) -> HRESULT,
    fn AssembleWide(
        Offset: ULONG64,
        Instr: PCWSTR,
        EndOffset: PULONG64,
    ) -> HRESULT,
    fn DisassembleWide(
        Offset: ULONG64,
        Flags: ULONG,
        Buffer: PWSTR,
        BufferSize: ULONG,
        DisassemblySize: PULONG,
        EndOffset: PULONG64,
    ) -> HRESULT,
    fn GetProcessorTypeNamesWide(
        Type: ULONG,
        FullNameBuffer: PWSTR,
        FullNameBufferSize: ULONG,
        FullNameSize: PULONG,
        AbbrevNameBuffer: PWSTR,
        AbbrevNameBufferSize: ULONG,
        AbbrevNameSize: PULONG,
    ) -> HRESULT,
    fn GetTextMacroWide(
        Slot: ULONG,
        Buffer: PWSTR,
        BufferSize: ULONG,
        MacroSize: PULONG,
    ) -> HRESULT,
    fn SetTextMacroWide(
        Slot: ULONG,
        Macro: PCWSTR,
    ) -> HRESULT,
    fn EvaluateWide(
        Expression: PCWSTR,
        DesiredType: ULONG,
        Value: PDEBUG_VALUE,
        RemainderIndex: PULONG,
    ) -> HRESULT,
    fn ExecuteWide(
        OutputControl: ULONG,
        Command: PCWSTR,
        Flags: ULONG,
    ) -> HRESULT,
    fn ExecuteCommandFileWide(
        OutputControl: ULONG,
        CommandFile: PCWSTR,
        Flags: ULONG,
    ) -> HRESULT,
    fn GetBreakpointByIndex2(
        Index: ULONG,
        Bp: *mut PDEBUG_BREAKPOINT2,
    ) -> HRESULT,
    fn GetBreakpointById2(
        Id: ULONG,
        Bp: *mut PDEBUG_BREAKPOINT2,
    ) -> HRESULT,
    fn AddBreakpoint2(
        Type: ULONG,
        DesiredId: ULONG,
        Bp: *mut PDEBUG_BREAKPOINT2,
    ) -> HRESULT,
    fn RemoveBreakpoint2(
        Bp: *mut PDEBUG_BREAKPOINT2 ,
    ) -> HRESULT,
    fn AddExtensionWide(
        Path: PCWSTR,
        Flags: ULONG,
        Handle: PULONG64,
    ) -> HRESULT,
    fn GetExtensionByPathWide(
        Path: PCWSTR,
        Handle: PULONG64,
    ) -> HRESULT,
    fn CallExtensionWide(
        Handle: ULONG64,
        Function: PCWSTR,
        Arguments: PCWSTR,
    ) -> HRESULT,
    fn GetExtensionFunctionWide(
        Handle: ULONG64,
        FuncName: PCWSTR,
        Function: *mut FARPROC,
    ) -> HRESULT,
    fn GetEventFilterTextWide(
        Index: ULONG,
        Buffer: PWSTR,
        BufferSize: ULONG,
        TextSize: PULONG,
    ) -> HRESULT,
    fn GetEventFilterCommandWide(
        Index: ULONG,
        Buffer: PWSTR,
        BufferSize: ULONG,
        CommandSize: PULONG,
    ) -> HRESULT,
    fn SetEventFilterCommandWide(
        Index: ULONG,
        Command: PCWSTR,
    ) -> HRESULT,
    fn GetSpecificFilterArgumentWide(
        Index: ULONG,
        Buffer: PWSTR,
        BufferSize: ULONG,
        ArgumentSize: PULONG,
    ) -> HRESULT,
    fn SetSpecificFilterArgumentWide(
        Index: ULONG,
        Argument: PCWSTR,
    ) -> HRESULT,
    fn GetExceptionFilterSecondCommandWide(
        Index: ULONG,
        Buffer: PWSTR,
        BufferSize: ULONG,
        CommandSize: PULONG,
    ) -> HRESULT,
    fn SetExceptionFilterSecondCommandWide(
        Index: ULONG,
        Command: PCWSTR,
    ) -> HRESULT,
    fn GetLastEventInformationWide(
        Type: PULONG,
        ProcessId: PULONG,
        ThreadId: PULONG,
        ExtraInformation: PVOID,
        ExtraInformationSize: ULONG,
        ExtraInformationUsed: PULONG,
        Description: PWSTR,
        DescriptionSize: ULONG,
        DescriptionUsed: PULONG,
    ) -> HRESULT,
    fn GetTextReplacementWide(
        SrcText: PCWSTR,
        Index: ULONG,
        SrcBuffer: PWSTR,
        SrcBufferSize: ULONG,
        SrcSize: PULONG,
        DstBuffer: PWSTR,
        DstBufferSize: ULONG,
        DstSize: PULONG,
    ) -> HRESULT,
    fn SetTextReplacementWide(
        SrcText: PCWSTR,
        DstText: PCWSTR,
    ) -> HRESULT,
    fn SetExpressionSyntaxByNameWide(AbbrevName: PCWSTR,) -> HRESULT,
    fn GetExpressionSyntaxNamesWide(
        Index: ULONG,
        FullNameBuffer: PWSTR,
        FullNameBufferSize: ULONG,
        FullNameSize: PULONG,
        AbbrevNameBuffer: PWSTR,
        AbbrevNameBufferSize: ULONG,
        AbbrevNameSize: PULONG,
    ) -> HRESULT,
    fn GetEventIndexDescriptionWide(
        Index: ULONG,
        Which: ULONG,
        Buffer: PWSTR,
        BufferSize: ULONG,
        DescSize: PULONG,
    ) -> HRESULT,
    fn GetLogFile2(
        Buffer: PSTR,
        BufferSize: ULONG,
        FileSize: PULONG,
        Flags: PULONG,
    ) -> HRESULT,
    fn OpenLogFile2(
        File: PCSTR,
        Flags: ULONG,
    ) -> HRESULT,
    fn GetLogFile2Wide(
        Buffer: PWSTR,
        BufferSize: ULONG,
        FileSize: PULONG,
        Flags: PULONG,
    ) -> HRESULT,
    fn OpenLogFile2Wide(
        File: PCWSTR,
        Flags: ULONG,
    ) -> HRESULT,
    fn GetSystemVersionValues(
        PlatformId: PULONG,
        Win32Major: PULONG,
        Win32Minor: PULONG,
        KdMajor: PULONG,
        KdMinor: PULONG,
    ) -> HRESULT,
    fn GetSystemVersionString(
        Which: ULONG,
        Buffer: PSTR,
        BufferSize: ULONG,
        StringSize: PULONG,
    ) -> HRESULT,
    fn GetSystemVersionStringWide(
        Which: ULONG,
        Buffer: PWSTR,
        BufferSize: ULONG,
        StringSize: PULONG,
    ) -> HRESULT,
    fn GetContextStackTrace(
        StartContext: PVOID,
        StartContextSize: ULONG,
        Frames: PDEBUG_STACK_FRAME,
        FramesSize: ULONG,
        FrameContexts: PVOID,
        FrameContextsSize: ULONG,
        FrameContextsEntrySize: ULONG,
        FramesFilled: PULONG,
    ) -> HRESULT,
    fn OutputContextStackTrace(
        OutputControl: ULONG,
        Frames: PDEBUG_STACK_FRAME,
        FramesSize: ULONG,
        FrameContexts: PVOID,
        FrameContextsSize: ULONG,
        FrameContextsEntrySize: ULONG,
        Flags: ULONG,
    ) -> HRESULT,
    fn GetStoredEventInformation(
        Type: PULONG,
        ProcessId: PULONG,
        ThreadId: PULONG,
        Context: PVOID,
        ContextSize: ULONG,
        ContextUsed: PULONG,
        ExtraInformation: PVOID,
        ExtraInformationSize: ULONG,
        ExtraInformationUsed: PULONG,
    ) -> HRESULT,
    fn GetManagedStatus(
        Flags: PULONG,
        WhichString: ULONG,
        String: PSTR,
        StringSize: ULONG,
        StringNeeded: PULONG,
    ) -> HRESULT,
    fn GetManagedStatusWide(
        Flags: PULONG,
        WhichString: ULONG,
        String: PWSTR,
        StringSize: ULONG,
        StringNeeded: PULONG,
    ) -> HRESULT,
    fn ResetManagedStatus(Flags: ULONG,) -> HRESULT,
});
RIDL!(#[uuid(0xb2ffe162, 0x2412, 0x429f, 0x8d, 0x1d, 0x5b, 0xf6, 0xdd, 0x82, 0x46, 0x96)]
interface IDebugControl5(IDebugControl5Vtbl): IDebugControl4(IDebugControl4Vtbl) {
    fn GetStackTraceEx(
        FrameOffset: ULONG64,
        StackOffset: ULONG64,
        InstructionOffset: ULONG64,
        Frames: PDEBUG_STACK_FRAME_EX,
        FramesSize: ULONG,
        FramesFilled: PULONG,
    ) -> HRESULT,
    fn OutputStackTraceEx(
        OutputControl: ULONG,
        Frames: PDEBUG_STACK_FRAME_EX,
        FramesSize: ULONG,
        Flags: ULONG,
    ) -> HRESULT,
    fn GetContextStackTraceEx(
        StartContext: PVOID,
        StartContextSize: ULONG,
        Frames: PDEBUG_STACK_FRAME_EX,
        FramesSize: ULONG,
        FrameContexts: PVOID,
        FrameContextsSize: ULONG,
        FrameContextsEntrySize: ULONG,
        FramesFilled: PULONG,
    ) -> HRESULT,
    fn OutputContextStackTraceEx(
        OutputControl: ULONG,
        Frames: PDEBUG_STACK_FRAME_EX,
        FramesSize: ULONG,
        FrameContexts: PVOID,
        FrameContextsSize: ULONG,
        FrameContextsEntrySize: ULONG,
        Flags: ULONG,
    ) -> HRESULT,
    fn GetBreakpointByGuid(
        Guid: LPGUID,
        Bp: *mut PDEBUG_BREAKPOINT3,
    ) -> HRESULT,
});
RIDL!(#[uuid(0xbc0d583f, 0x126d, 0x43a1, 0x9c, 0xc4, 0xa8, 0x60, 0xab, 0x1d, 0x53, 0x7b)]
interface IDebugControl6(IDebugControl6Vtbl): IDebugControl5(IDebugControl5Vtbl) {
    fn GetExecutionStatusEx(Status: PULONG,) -> HRESULT,
    fn GetSynchronizationStatus(
        SendsAttempted: PULONG,
        SecondsSinceLastResponse: PULONG,
    ) -> HRESULT,
});
pub const DEBUG_EXEC_FLAGS_NONBLOCK: ULONG = 0x00000001;
RIDL!(#[uuid(0xb86fb3b1, 0x80d4, 0x475b, 0xae, 0xa3, 0xcf, 0x06, 0x53, 0x9c, 0xf6, 0x3a)]
interface IDebugControl7(IDebugControl7Vtbl): IDebugControl6(IDebugControl6Vtbl) {
    fn GetDebuggeeType2(
        Flags: ULONG,
        Class: PULONG,
        Qualifier: PULONG,
    ) -> HRESULT,
});
pub const DEBUG_DATA_SPACE_VIRTUAL: ULONG = 0;
pub const DEBUG_DATA_SPACE_PHYSICAL: ULONG = 1;
pub const DEBUG_DATA_SPACE_CONTROL: ULONG = 2;
pub const DEBUG_DATA_SPACE_IO: ULONG = 3;
pub const DEBUG_DATA_SPACE_MSR: ULONG = 4;
pub const DEBUG_DATA_SPACE_BUS_DATA: ULONG = 5;
pub const DEBUG_DATA_SPACE_DEBUGGER_DATA: ULONG = 6;
pub const DEBUG_DATA_SPACE_COUNT: ULONG = 7;
pub const DEBUG_DATA_KernBase: ULONG = 24;
pub const DEBUG_DATA_BreakpointWithStatusAddr: ULONG = 32;
pub const DEBUG_DATA_SavedContextAddr: ULONG = 40;
pub const DEBUG_DATA_KiCallUserModeAddr: ULONG = 56;
pub const DEBUG_DATA_KeUserCallbackDispatcherAddr: ULONG = 64;
pub const DEBUG_DATA_PsLoadedModuleListAddr: ULONG = 72;
pub const DEBUG_DATA_PsActiveProcessHeadAddr: ULONG = 80;
pub const DEBUG_DATA_PspCidTableAddr: ULONG = 88;
pub const DEBUG_DATA_ExpSystemResourcesListAddr: ULONG = 96;
pub const DEBUG_DATA_ExpPagedPoolDescriptorAddr: ULONG = 104;
pub const DEBUG_DATA_ExpNumberOfPagedPoolsAddr: ULONG = 112;
pub const DEBUG_DATA_KeTimeIncrementAddr: ULONG = 120;
pub const DEBUG_DATA_KeBugCheckCallbackListHeadAddr: ULONG = 128;
pub const DEBUG_DATA_KiBugcheckDataAddr: ULONG = 136;
pub const DEBUG_DATA_IopErrorLogListHeadAddr: ULONG = 144;
pub const DEBUG_DATA_ObpRootDirectoryObjectAddr: ULONG = 152;
pub const DEBUG_DATA_ObpTypeObjectTypeAddr: ULONG = 160;
pub const DEBUG_DATA_MmSystemCacheStartAddr: ULONG = 168;
pub const DEBUG_DATA_MmSystemCacheEndAddr: ULONG = 176;
pub const DEBUG_DATA_MmSystemCacheWsAddr: ULONG = 184;
pub const DEBUG_DATA_MmPfnDatabaseAddr: ULONG = 192;
pub const DEBUG_DATA_MmSystemPtesStartAddr: ULONG = 200;
pub const DEBUG_DATA_MmSystemPtesEndAddr: ULONG = 208;
pub const DEBUG_DATA_MmSubsectionBaseAddr: ULONG = 216;
pub const DEBUG_DATA_MmNumberOfPagingFilesAddr: ULONG = 224;
pub const DEBUG_DATA_MmLowestPhysicalPageAddr: ULONG = 232;
pub const DEBUG_DATA_MmHighestPhysicalPageAddr: ULONG = 240;
pub const DEBUG_DATA_MmNumberOfPhysicalPagesAddr: ULONG = 248;
pub const DEBUG_DATA_MmMaximumNonPagedPoolInBytesAddr: ULONG = 256;
pub const DEBUG_DATA_MmNonPagedSystemStartAddr: ULONG = 264;
pub const DEBUG_DATA_MmNonPagedPoolStartAddr: ULONG = 272;
pub const DEBUG_DATA_MmNonPagedPoolEndAddr: ULONG = 280;
pub const DEBUG_DATA_MmPagedPoolStartAddr: ULONG = 288;
pub const DEBUG_DATA_MmPagedPoolEndAddr: ULONG = 296;
pub const DEBUG_DATA_MmPagedPoolInformationAddr: ULONG = 304;
pub const DEBUG_DATA_MmPageSize: ULONG = 312;
pub const DEBUG_DATA_MmSizeOfPagedPoolInBytesAddr: ULONG = 320;
pub const DEBUG_DATA_MmTotalCommitLimitAddr: ULONG = 328;
pub const DEBUG_DATA_MmTotalCommittedPagesAddr: ULONG = 336;
pub const DEBUG_DATA_MmSharedCommitAddr: ULONG = 344;
pub const DEBUG_DATA_MmDriverCommitAddr: ULONG = 352;
pub const DEBUG_DATA_MmProcessCommitAddr: ULONG = 360;
pub const DEBUG_DATA_MmPagedPoolCommitAddr: ULONG = 368;
pub const DEBUG_DATA_MmExtendedCommitAddr: ULONG = 376;
pub const DEBUG_DATA_MmZeroedPageListHeadAddr: ULONG = 384;
pub const DEBUG_DATA_MmFreePageListHeadAddr: ULONG = 392;
pub const DEBUG_DATA_MmStandbyPageListHeadAddr: ULONG = 400;
pub const DEBUG_DATA_MmModifiedPageListHeadAddr: ULONG = 408;
pub const DEBUG_DATA_MmModifiedNoWritePageListHeadAddr: ULONG = 416;
pub const DEBUG_DATA_MmAvailablePagesAddr: ULONG = 424;
pub const DEBUG_DATA_MmResidentAvailablePagesAddr: ULONG = 432;
pub const DEBUG_DATA_PoolTrackTableAddr: ULONG = 440;
pub const DEBUG_DATA_NonPagedPoolDescriptorAddr: ULONG = 448;
pub const DEBUG_DATA_MmHighestUserAddressAddr: ULONG = 456;
pub const DEBUG_DATA_MmSystemRangeStartAddr: ULONG = 464;
pub const DEBUG_DATA_MmUserProbeAddressAddr: ULONG = 472;
pub const DEBUG_DATA_KdPrintCircularBufferAddr: ULONG = 480;
pub const DEBUG_DATA_KdPrintCircularBufferEndAddr: ULONG = 488;
pub const DEBUG_DATA_KdPrintWritePointerAddr: ULONG = 496;
pub const DEBUG_DATA_KdPrintRolloverCountAddr: ULONG = 504;
pub const DEBUG_DATA_MmLoadedUserImageListAddr: ULONG = 512;
pub const DEBUG_DATA_NtBuildLabAddr: ULONG = 520;
pub const DEBUG_DATA_KiNormalSystemCall: ULONG = 528;
pub const DEBUG_DATA_KiProcessorBlockAddr: ULONG = 536;
pub const DEBUG_DATA_MmUnloadedDriversAddr: ULONG = 544;
pub const DEBUG_DATA_MmLastUnloadedDriverAddr: ULONG = 552;
pub const DEBUG_DATA_MmTriageActionTakenAddr: ULONG = 560;
pub const DEBUG_DATA_MmSpecialPoolTagAddr: ULONG = 568;
pub const DEBUG_DATA_KernelVerifierAddr: ULONG = 576;
pub const DEBUG_DATA_MmVerifierDataAddr: ULONG = 584;
pub const DEBUG_DATA_MmAllocatedNonPagedPoolAddr: ULONG = 592;
pub const DEBUG_DATA_MmPeakCommitmentAddr: ULONG = 600;
pub const DEBUG_DATA_MmTotalCommitLimitMaximumAddr: ULONG = 608;
pub const DEBUG_DATA_CmNtCSDVersionAddr: ULONG = 616;
pub const DEBUG_DATA_MmPhysicalMemoryBlockAddr: ULONG = 624;
pub const DEBUG_DATA_MmSessionBase: ULONG = 632;
pub const DEBUG_DATA_MmSessionSize: ULONG = 640;
pub const DEBUG_DATA_MmSystemParentTablePage: ULONG = 648;
pub const DEBUG_DATA_MmVirtualTranslationBase: ULONG = 656;
pub const DEBUG_DATA_OffsetKThreadNextProcessor: ULONG = 664;
pub const DEBUG_DATA_OffsetKThreadTeb: ULONG = 666;
pub const DEBUG_DATA_OffsetKThreadKernelStack: ULONG = 668;
pub const DEBUG_DATA_OffsetKThreadInitialStack: ULONG = 670;
pub const DEBUG_DATA_OffsetKThreadApcProcess: ULONG = 672;
pub const DEBUG_DATA_OffsetKThreadState: ULONG = 674;
pub const DEBUG_DATA_OffsetKThreadBStore: ULONG = 676;
pub const DEBUG_DATA_OffsetKThreadBStoreLimit: ULONG = 678;
pub const DEBUG_DATA_SizeEProcess: ULONG = 680;
pub const DEBUG_DATA_OffsetEprocessPeb: ULONG = 682;
pub const DEBUG_DATA_OffsetEprocessParentCID: ULONG = 684;
pub const DEBUG_DATA_OffsetEprocessDirectoryTableBase: ULONG = 686;
pub const DEBUG_DATA_SizePrcb: ULONG = 688;
pub const DEBUG_DATA_OffsetPrcbDpcRoutine: ULONG = 690;
pub const DEBUG_DATA_OffsetPrcbCurrentThread: ULONG = 692;
pub const DEBUG_DATA_OffsetPrcbMhz: ULONG = 694;
pub const DEBUG_DATA_OffsetPrcbCpuType: ULONG = 696;
pub const DEBUG_DATA_OffsetPrcbVendorString: ULONG = 698;
pub const DEBUG_DATA_OffsetPrcbProcessorState: ULONG = 700;
pub const DEBUG_DATA_OffsetPrcbNumber: ULONG = 702;
pub const DEBUG_DATA_SizeEThread: ULONG = 704;
pub const DEBUG_DATA_KdPrintCircularBufferPtrAddr: ULONG = 712;
pub const DEBUG_DATA_KdPrintBufferSizeAddr: ULONG = 720;
pub const DEBUG_DATA_MmBadPagesDetected: ULONG = 800;
pub const DEBUG_DATA_EtwpDebuggerData: ULONG = 816;
pub const DEBUG_DATA_PteBase: ULONG = 864;
pub const DEBUG_DATA_PaeEnabled: ULONG = 100000;
pub const DEBUG_DATA_SharedUserData: ULONG = 100008;
pub const DEBUG_DATA_ProductType: ULONG = 100016;
pub const DEBUG_DATA_SuiteMask: ULONG = 100024;
pub const DEBUG_DATA_DumpWriterStatus: ULONG = 100032;
pub const DEBUG_DATA_DumpFormatVersion: ULONG = 100040;
pub const DEBUG_DATA_DumpWriterVersion: ULONG = 100048;
pub const DEBUG_DATA_DumpPowerState: ULONG = 100056;
pub const DEBUG_DATA_DumpMmStorage: ULONG = 100064;
pub const DEBUG_DATA_DumpAttributes: ULONG = 100072;
STRUCT!{struct DEBUG_PROCESSOR_IDENTIFICATION_ALPHA {
    Type: ULONG,
    Revision: ULONG,
}}
pub type PDEBUG_PROCESSOR_IDENTIFICATION_ALPHA = *mut DEBUG_PROCESSOR_IDENTIFICATION_ALPHA;
STRUCT!{struct DEBUG_PROCESSOR_IDENTIFICATION_AMD64 {
    Family: ULONG,
    Model: ULONG,
    Stepping: ULONG,
    VendorString: [CHAR; 16],
}}
pub type PDEBUG_PROCESSOR_IDENTIFICATION_AMD64 = *mut DEBUG_PROCESSOR_IDENTIFICATION_AMD64;
STRUCT!{struct DEBUG_PROCESSOR_IDENTIFICATION_IA64 {
    Model: ULONG,
    Revision: ULONG,
    Family: ULONG,
    ArchRev: ULONG,
    VendorString: [CHAR; 16],
}}
pub type PDEBUG_PROCESSOR_IDENTIFICATION_IA64 = *mut DEBUG_PROCESSOR_IDENTIFICATION_IA64;
STRUCT!{struct DEBUG_PROCESSOR_IDENTIFICATION_X86 {
    Family: ULONG,
    Model: ULONG,
    Stepping: ULONG,
    VendorString: [CHAR; 16],
}}
pub type PDEBUG_PROCESSOR_IDENTIFICATION_X86 = *mut DEBUG_PROCESSOR_IDENTIFICATION_X86;
STRUCT!{struct DEBUG_PROCESSOR_IDENTIFICATION_ARM {
    Model: ULONG,
    Revision: ULONG,
    VendorString: [CHAR; 16],
}}
pub type PDEBUG_PROCESSOR_IDENTIFICATION_ARM = *mut DEBUG_PROCESSOR_IDENTIFICATION_ARM;
STRUCT!{struct DEBUG_PROCESSOR_IDENTIFICATION_ARM64 {
    Model: ULONG,
    Revision: ULONG,
    VendorString: [CHAR; 16],
}}
pub type PDEBUG_PROCESSOR_IDENTIFICATION_ARM64 = *mut DEBUG_PROCESSOR_IDENTIFICATION_ARM64;
UNION!{union DEBUG_PROCESSOR_IDENTIFICATION_ALL {
    [u32; 8],
    Alpha Alpha_mut: DEBUG_PROCESSOR_IDENTIFICATION_ALPHA,
    Amd64 Amd64_mut: DEBUG_PROCESSOR_IDENTIFICATION_AMD64,
    Ia64 Ia64_mut: DEBUG_PROCESSOR_IDENTIFICATION_IA64,
    X86 X86_mut: DEBUG_PROCESSOR_IDENTIFICATION_X86,
    Arm Arm_mut: DEBUG_PROCESSOR_IDENTIFICATION_ARM,
    Arm64 Arm64_mut: DEBUG_PROCESSOR_IDENTIFICATION_ARM64,
}}
pub type PDEBUG_PROCESSOR_IDENTIFICATION_ALL = *mut DEBUG_PROCESSOR_IDENTIFICATION_ALL;
pub const DEBUG_DATA_KPCR_OFFSET: ULONG = 0;
pub const DEBUG_DATA_KPRCB_OFFSET: ULONG = 1;
pub const DEBUG_DATA_KTHREAD_OFFSET: ULONG = 2;
pub const DEBUG_DATA_BASE_TRANSLATION_VIRTUAL_OFFSET: ULONG = 3;
pub const DEBUG_DATA_PROCESSOR_IDENTIFICATION: ULONG = 4;
pub const DEBUG_DATA_PROCESSOR_SPEED: ULONG = 5;
RIDL!(#[uuid(0x88f7dfab, 0x3ea7, 0x4c3a, 0xae, 0xfb, 0xc4, 0xe8, 0x10, 0x61, 0x73, 0xaa)]
interface IDebugDataSpaces(IDebugDataSpacesVtbl): IUnknown(IUnknownVtbl) {
    fn ReadVirtual(
        Offset: ULONG64,
        Buffer: PVOID,
        BufferSize: ULONG,
        BytesRead: PULONG,
    ) -> HRESULT,
    fn WriteVirtual(
        Offset: ULONG64,
        Buffer: PVOID,
        BufferSize: ULONG,
        BytesWritten: PULONG,
    ) -> HRESULT,
    fn SearchVirtual(
        Offset: ULONG64,
        Length: ULONG64,
        Pattern: PVOID,
        PatternSize: ULONG,
        PatternGranularity: ULONG,
        MatchOffset: PULONG64,
    ) -> HRESULT,
    fn ReadVirtualUncached(
        Offset: ULONG64,
        Buffer: PVOID,
        BufferSize: ULONG,
        BytesRead: PULONG,
    ) -> HRESULT,
    fn WriteVirtualUncached(
        Offset: ULONG64,
        Buffer: PVOID,
        BufferSize: ULONG,
        BytesWritten: PULONG,
    ) -> HRESULT,
    fn ReadPointersVirtual(
        Count: ULONG,
        Offset: ULONG64,
        Ptrs: PULONG64,
    ) -> HRESULT,
    fn WritePointersVirtual(
        Count: ULONG,
        Offset: ULONG64,
        Ptrs: PULONG64,
    ) -> HRESULT,
    fn ReadPhysical(
        Offset: ULONG64,
        Buffer: PVOID,
        BufferSize: ULONG,
        BytesRead: PULONG,
    ) -> HRESULT,
    fn WritePhysical(
        Offset: ULONG64,
        Buffer: PVOID,
        BufferSize: ULONG,
        BytesWritten: PULONG,
    ) -> HRESULT,
    fn ReadControl(
        Processor: ULONG,
        Offset: ULONG64,
        Buffer: PVOID,
        BufferSize: ULONG,
        BytesRead: PULONG,
    ) -> HRESULT,
    fn WriteControl(
        Processor: ULONG,
        Offset: ULONG64,
        Buffer: PVOID,
        BufferSize: ULONG,
        BytesWritten: PULONG,
    ) -> HRESULT,
    fn ReadIo(
        InterfaceType: ULONG,
        BusNumber: ULONG,
        AddressSpace: ULONG,
        Offset: ULONG64,
        Buffer: PVOID,
        BufferSize: ULONG,
        BytesRead: PULONG,
    ) -> HRESULT,
    fn WriteIo(
        InterfaceType: ULONG,
        BusNumber: ULONG,
        AddressSpace: ULONG,
        Offset: ULONG64,
        Buffer: PVOID,
        BufferSize: ULONG,
        BytesWritten: PULONG,
    ) -> HRESULT,
    fn ReadMsr(
        Msr: ULONG,
        Value: PULONG64,
    ) -> HRESULT,
    fn WriteMsr(
        Msr: ULONG,
        Value: ULONG64,
    ) -> HRESULT,
    fn ReadBusData(
        BusDataType: ULONG,
        BusNumber: ULONG,
        SlotNumber: ULONG,
        Offset: ULONG,
        Buffer: PVOID,
        BufferSize: ULONG,
        BytesRead: PULONG,
    ) -> HRESULT,
    fn WriteBusData(
        BusDataType: ULONG,
        BusNumber: ULONG,
        SlotNumber: ULONG,
        Offset: ULONG,
        Buffer: PVOID,
        BufferSize: ULONG,
        BytesWritten: PULONG,
    ) -> HRESULT,
    fn CheckLowMemory() -> HRESULT,
    fn ReadDebuggerData(
        Index: ULONG,
        Buffer: PVOID,
        BufferSize: ULONG,
        DataSize: PULONG,
    ) -> HRESULT,
    fn ReadProcessorSystemData(
        Processor: ULONG,
        Index: ULONG,
        Buffer: PVOID,
        BufferSize: ULONG,
        DataSize: PULONG,
    ) -> HRESULT,
});
pub const DEBUG_HANDLE_DATA_TYPE_BASIC: ULONG = 0;
pub const DEBUG_HANDLE_DATA_TYPE_TYPE_NAME: ULONG = 1;
pub const DEBUG_HANDLE_DATA_TYPE_OBJECT_NAME: ULONG = 2;
pub const DEBUG_HANDLE_DATA_TYPE_HANDLE_COUNT: ULONG = 3;
pub const DEBUG_HANDLE_DATA_TYPE_TYPE_NAME_WIDE: ULONG = 4;
pub const DEBUG_HANDLE_DATA_TYPE_OBJECT_NAME_WIDE: ULONG = 5;
pub const DEBUG_HANDLE_DATA_TYPE_MINI_THREAD_1: ULONG = 6;
pub const DEBUG_HANDLE_DATA_TYPE_MINI_MUTANT_1: ULONG = 7;
pub const DEBUG_HANDLE_DATA_TYPE_MINI_MUTANT_2: ULONG = 8;
pub const DEBUG_HANDLE_DATA_TYPE_PER_HANDLE_OPERATIONS: ULONG = 9;
pub const DEBUG_HANDLE_DATA_TYPE_ALL_HANDLE_OPERATIONS: ULONG = 10;
pub const DEBUG_HANDLE_DATA_TYPE_MINI_PROCESS_1: ULONG = 11;
pub const DEBUG_HANDLE_DATA_TYPE_MINI_PROCESS_2: ULONG = 12;
pub const DEBUG_HANDLE_DATA_TYPE_MINI_EVENT_1: ULONG = 13;
pub const DEBUG_HANDLE_DATA_TYPE_MINI_SECTION_1: ULONG = 14;
pub const DEBUG_HANDLE_DATA_TYPE_MINI_SEMAPHORE_1: ULONG = 15;
STRUCT!{struct DEBUG_HANDLE_DATA_BASIC {
    TypeNameSize: ULONG,
    ObjectNameSize: ULONG,
    Attributes: ULONG,
    GrantedAccess: ULONG,
    HandleCount: ULONG,
    PointerCount: ULONG,
}}
pub type PDEBUG_HANDLE_DATA_BASIC = *mut DEBUG_HANDLE_DATA_BASIC;
RIDL!(#[uuid(0x7a5e852f, 0x96e9, 0x468f, 0xac, 0x1b, 0x0b, 0x3a, 0xdd, 0xc4, 0xa0, 0x49)]
interface IDebugDataSpaces2(IDebugDataSpaces2Vtbl): IDebugDataSpaces(IDebugDataSpacesVtbl) {
    fn VirtualToPhysical(
        Virtual: ULONG64,
        Physical: PULONG64,
    ) -> HRESULT,
    fn GetVirtualTranslationPhysicalOffsets(
        Virtual: ULONG64,
        Offsets: PULONG64,
        OffsetsSize: ULONG,
        Levels: PULONG,
    ) -> HRESULT,
    fn ReadHandleData(
        Handle: ULONG64,
        DataType: ULONG,
        Buffer: PVOID,
        BufferSize: ULONG,
        DataSize: PULONG,
    ) -> HRESULT,
    fn FillVirtual(
        Start: ULONG64,
        Size: ULONG,
        Pattern: PVOID,
        PatternSize: ULONG,
        Filled: PULONG,
    ) -> HRESULT,
    fn FillPhysical(
        Start: ULONG64,
        Size: ULONG,
        Pattern: PVOID,
        PatternSize: ULONG,
        Filled: PULONG,
    ) -> HRESULT,
    fn QueryVirtual(
        Offset: ULONG64,
        Info: PMEMORY_BASIC_INFORMATION64,
    ) -> HRESULT,
});
RIDL!(#[uuid(0x23f79d6c, 0x8aaf, 0x4f7c, 0xa6, 0x07, 0x99, 0x95, 0xf5, 0x40, 0x7e, 0x63)]
interface IDebugDataSpaces3(IDebugDataSpaces3Vtbl): IDebugDataSpaces2(IDebugDataSpaces2Vtbl) {
    fn ReadImageNtHeaders(
        ImageBase: ULONG64,
        Headers: PIMAGE_NT_HEADERS64,
    ) -> HRESULT,
    fn ReadTagged(
        Tag: LPGUID,
        Offset: ULONG,
        Buffer: PVOID,
        BufferSize: ULONG,
        TotalSize: PULONG,
    ) -> HRESULT,
    fn StartEnumTagged(Handle: PULONG64,) -> HRESULT,
    fn GetNextTagged(
        Handle: ULONG64,
        Tag: LPGUID,
        Size: PULONG,
    ) -> HRESULT,
    fn EndEnumTagged(Handle: ULONG64,) -> HRESULT,
});
pub const DEBUG_OFFSINFO_VIRTUAL_SOURCE: ULONG = 0x00000001;
pub const DEBUG_VSOURCE_INVALID: ULONG = 0x00000000;
pub const DEBUG_VSOURCE_DEBUGGEE: ULONG = 0x00000001;
pub const DEBUG_VSOURCE_MAPPED_IMAGE: ULONG = 0x00000002;
pub const DEBUG_VSOURCE_DUMP_WITHOUT_MEMINFO: ULONG = 0x00000003;
pub const DEBUG_VSEARCH_DEFAULT: ULONG = 0x00000000;
pub const DEBUG_VSEARCH_WRITABLE_ONLY: ULONG = 0x00000001;
pub const DEBUG_PHYSICAL_DEFAULT: ULONG = 0x00000000;
pub const DEBUG_PHYSICAL_CACHED: ULONG = 0x00000001;
pub const DEBUG_PHYSICAL_UNCACHED: ULONG = 0x00000002;
pub const DEBUG_PHYSICAL_WRITE_COMBINED: ULONG = 0x00000003;
RIDL!(#[uuid(0xd98ada1f, 0x29e9, 0x4ef5, 0xa6, 0xc0, 0xe5, 0x33, 0x49, 0x88, 0x32, 0x12)]
interface IDebugDataSpaces4(IDebugDataSpaces4Vtbl): IDebugDataSpaces3(IDebugDataSpaces3Vtbl) {
    fn GetOffsetInformation(
        Space: ULONG,
        Which: ULONG,
        Offset: ULONG64,
        Buffer: PVOID,
        BufferSize: ULONG,
        InfoSize: PULONG,
    ) -> HRESULT,
    fn GetNextDifferentlyValidOffsetVirtual(
        Offset: ULONG64,
        NextOffset: PULONG64,
    ) -> HRESULT,
    fn GetValidRegionVirtual(
        Base: ULONG64,
        Size: ULONG,
        ValidBase: PULONG64,
        ValidSize: PULONG,
    ) -> HRESULT,
    fn SearchVirtual2(
        Offset: ULONG64,
        Length: ULONG64,
        Flags: ULONG,
        Pattern: PVOID,
        PatternSize: ULONG,
        PatternGranularity: ULONG,
        MatchOffset: PULONG64,
    ) -> HRESULT,
    fn ReadMultiByteStringVirtual(
        Offset: ULONG64,
        MaxBytes: ULONG,
        Buffer: PSTR,
        BufferSize: ULONG,
        StringBytes: PULONG,
    ) -> HRESULT,
    fn ReadMultiByteStringVirtualWide(
        Offset: ULONG64,
        MaxBytes: ULONG,
        CodePage: ULONG,
        Buffer: PWSTR,
        BufferSize: ULONG,
        StringBytes: PULONG,
    ) -> HRESULT,
    fn ReadUnicodeStringVirtual(
        Offset: ULONG64,
        MaxBytes: ULONG,
        CodePage: ULONG,
        Buffer: PSTR,
        BufferSize: ULONG,
        StringBytes: PULONG,
    ) -> HRESULT,
    fn ReadUnicodeStringVirtualWide(
        Offset: ULONG64,
        MaxBytes: ULONG,
        Buffer: PWSTR,
        BufferSize: ULONG,
        StringBytes: PULONG,
    ) -> HRESULT,
    fn ReadPhysical2(
        Offset: ULONG64,
        Flags: ULONG,
        Buffer: PVOID,
        BufferSize: ULONG,
        BytesRead: PULONG,
    ) -> HRESULT,
    fn WritePhysical2(
        Offset: ULONG64,
        Flags: ULONG,
        Buffer: PVOID,
        BufferSize: ULONG,
        BytesWritten: PULONG,
    ) -> HRESULT,
});
pub const DEBUG_EVENT_BREAKPOINT: ULONG = 0x00000001;
pub const DEBUG_EVENT_EXCEPTION: ULONG = 0x00000002;
pub const DEBUG_EVENT_CREATE_THREAD: ULONG = 0x00000004;
pub const DEBUG_EVENT_EXIT_THREAD: ULONG = 0x00000008;
pub const DEBUG_EVENT_CREATE_PROCESS: ULONG = 0x00000010;
pub const DEBUG_EVENT_EXIT_PROCESS: ULONG = 0x00000020;
pub const DEBUG_EVENT_LOAD_MODULE: ULONG = 0x00000040;
pub const DEBUG_EVENT_UNLOAD_MODULE: ULONG = 0x00000080;
pub const DEBUG_EVENT_SYSTEM_ERROR: ULONG = 0x00000100;
pub const DEBUG_EVENT_SESSION_STATUS: ULONG = 0x00000200;
pub const DEBUG_EVENT_CHANGE_DEBUGGEE_STATE: ULONG = 0x00000400;
pub const DEBUG_EVENT_CHANGE_ENGINE_STATE: ULONG = 0x00000800;
pub const DEBUG_EVENT_CHANGE_SYMBOL_STATE: ULONG = 0x00001000;
pub const DEBUG_SESSION_ACTIVE: ULONG = 0x00000000;
pub const DEBUG_SESSION_END_SESSION_ACTIVE_TERMINATE: ULONG = 0x00000001;
pub const DEBUG_SESSION_END_SESSION_ACTIVE_DETACH: ULONG = 0x00000002;
pub const DEBUG_SESSION_END_SESSION_PASSIVE: ULONG = 0x00000003;
pub const DEBUG_SESSION_END: ULONG = 0x00000004;
pub const DEBUG_SESSION_REBOOT: ULONG = 0x00000005;
pub const DEBUG_SESSION_HIBERNATE: ULONG = 0x00000006;
pub const DEBUG_SESSION_FAILURE: ULONG = 0x00000007;
pub const DEBUG_CDS_ALL: ULONG = 0xffffffff;
pub const DEBUG_CDS_REGISTERS: ULONG = 0x00000001;
pub const DEBUG_CDS_DATA: ULONG = 0x00000002;
pub const DEBUG_CDS_REFRESH: ULONG = 0x00000004;
pub const DEBUG_CDS_REFRESH_EVALUATE: ULONG = 1;
pub const DEBUG_CDS_REFRESH_EXECUTE: ULONG = 2;
pub const DEBUG_CDS_REFRESH_EXECUTECOMMANDFILE: ULONG = 3;
pub const DEBUG_CDS_REFRESH_ADDBREAKPOINT: ULONG = 4;
pub const DEBUG_CDS_REFRESH_REMOVEBREAKPOINT: ULONG = 5;
pub const DEBUG_CDS_REFRESH_WRITEVIRTUAL: ULONG = 6;
pub const DEBUG_CDS_REFRESH_WRITEVIRTUALUNCACHED: ULONG = 7;
pub const DEBUG_CDS_REFRESH_WRITEPHYSICAL: ULONG = 8;
pub const DEBUG_CDS_REFRESH_WRITEPHYSICAL2: ULONG = 9;
pub const DEBUG_CDS_REFRESH_SETVALUE: ULONG = 10;
pub const DEBUG_CDS_REFRESH_SETVALUE2: ULONG = 11;
pub const DEBUG_CDS_REFRESH_SETSCOPE: ULONG = 12;
pub const DEBUG_CDS_REFRESH_SETSCOPEFRAMEBYINDEX: ULONG = 13;
pub const DEBUG_CDS_REFRESH_SETSCOPEFROMJITDEBUGINFO: ULONG = 14;
pub const DEBUG_CDS_REFRESH_SETSCOPEFROMSTOREDEVENT: ULONG = 15;
pub const DEBUG_CDS_REFRESH_INLINESTEP: ULONG = 16;
pub const DEBUG_CDS_REFRESH_INLINESTEP_PSEUDO: ULONG = 17;
pub const DEBUG_CES_ALL: ULONG = 0xffffffff;
pub const DEBUG_CES_CURRENT_THREAD: ULONG = 0x00000001;
pub const DEBUG_CES_EFFECTIVE_PROCESSOR: ULONG = 0x00000002;
pub const DEBUG_CES_BREAKPOINTS: ULONG = 0x00000004;
pub const DEBUG_CES_CODE_LEVEL: ULONG = 0x00000008;
pub const DEBUG_CES_EXECUTION_STATUS: ULONG = 0x00000010;
pub const DEBUG_CES_ENGINE_OPTIONS: ULONG = 0x00000020;
pub const DEBUG_CES_LOG_FILE: ULONG = 0x00000040;
pub const DEBUG_CES_RADIX: ULONG = 0x00000080;
pub const DEBUG_CES_EVENT_FILTERS: ULONG = 0x00000100;
pub const DEBUG_CES_PROCESS_OPTIONS: ULONG = 0x00000200;
pub const DEBUG_CES_EXTENSIONS: ULONG = 0x00000400;
pub const DEBUG_CES_SYSTEMS: ULONG = 0x00000800;
pub const DEBUG_CES_ASSEMBLY_OPTIONS: ULONG = 0x00001000;
pub const DEBUG_CES_EXPRESSION_SYNTAX: ULONG = 0x00002000;
pub const DEBUG_CES_TEXT_REPLACEMENTS: ULONG = 0x00004000;
pub const DEBUG_CSS_ALL: ULONG = 0xffffffff;
pub const DEBUG_CSS_LOADS: ULONG = 0x00000001;
pub const DEBUG_CSS_UNLOADS: ULONG = 0x00000002;
pub const DEBUG_CSS_SCOPE: ULONG = 0x00000004;
pub const DEBUG_CSS_PATHS: ULONG = 0x00000008;
pub const DEBUG_CSS_SYMBOL_OPTIONS: ULONG = 0x00000010;
pub const DEBUG_CSS_TYPE_OPTIONS: ULONG = 0x00000020;
pub const DEBUG_CSS_COLLAPSE_CHILDREN: ULONG = 0x00000040;
RIDL!(#[uuid(0x337be28b, 0x5036, 0x4d72, 0xb6, 0xbf, 0xc4, 0x5f, 0xbb, 0x9f, 0x2e, 0xaa)]
interface IDebugEventCallbacks(IDebugEventCallbacksVtbl): IUnknown(IUnknownVtbl) {
    fn GetInterestMask(Mask: PULONG,) -> HRESULT,
    fn Breakpoint(Bp: PDEBUG_BREAKPOINT,) -> HRESULT,
    fn Exception(
        Exception: PEXCEPTION_RECORD64,
        FirstChance: ULONG,
    ) -> HRESULT,
    fn CreateThread(
        Handle: ULONG64,
        DataOffset: ULONG64,
        StartOffset: ULONG64,
    ) -> HRESULT,
    fn ExitThread(ExitCode: ULONG,) -> HRESULT,
    fn CreateProcess(
        ImageFileHandle: ULONG64,
        Handle: ULONG64,
        BaseOffset: ULONG64,
        ModuleSize: ULONG,
        ModuleName: PCSTR,
        ImageName: PCSTR,
        CheckSum: ULONG,
        TimeDateStamp: ULONG,
        InitialThreadHandle: ULONG64,
        ThreadDataOffset: ULONG64,
        StartOffset: ULONG64,
    ) -> HRESULT,
    fn ExitProcess(ExitCode: ULONG,) -> HRESULT,
    fn LoadModule(
        ImageFileHandle: ULONG64,
        BaseOffset: ULONG64,
        ModuleSize: ULONG,
        ModuleName: PCSTR,
        ImageName: PCSTR,
        CheckSum: ULONG,
        TimeDateStamp: ULONG,
    ) -> HRESULT,
    fn UnloadModule(
        ImageBaseName: PCSTR,
        BaseOffset: ULONG64,
    ) -> HRESULT,
    fn SystemError(
        Error: ULONG,
        Level: ULONG,
    ) -> HRESULT,
    fn SessionStatus(Status: ULONG,) -> HRESULT,
    fn ChangeDebuggeeState(
        Flags: ULONG,
        Argument: ULONG64,
    ) -> HRESULT,
    fn ChangeEngineState(
        Flags: ULONG,
        Argument: ULONG64,
    ) -> HRESULT,
    fn ChangeSymbolState(
        Flags: ULONG,
        Argument: ULONG64,
    ) -> HRESULT,
});
RIDL!(#[uuid(0x0690e046, 0x9c23, 0x45ac, 0xa0, 0x4f, 0x98, 0x7a, 0xc2, 0x9a, 0xd0, 0xd3)]
interface IDebugEventCallbacksWide(IDebugEventCallbacksWideVtbl): IUnknown(IUnknownVtbl) {
    fn GetInterestMask(Mask: PULONG,) -> HRESULT,
    fn Breakpoint(Bp: PDEBUG_BREAKPOINT2,) -> HRESULT,
    fn Exception(
        Exception: PEXCEPTION_RECORD64,
        FirstChance: ULONG,
    ) -> HRESULT,
    fn CreateThread(
        Handle: ULONG64,
        DataOffset: ULONG64,
        StartOffset: ULONG64,
    ) -> HRESULT,
    fn ExitThread(ExitCode: ULONG,) -> HRESULT,
    fn CreateProcess(
        ImageFileHandle: ULONG64,
        Handle: ULONG64,
        BaseOffset: ULONG64,
        ModuleSize: ULONG,
        ModuleName: PCWSTR,
        ImageName: PCWSTR,
        CheckSum: ULONG,
        TimeDateStamp: ULONG,
        InitialThreadHandle: ULONG64,
        ThreadDataOffset: ULONG64,
        StartOffset: ULONG64,
    ) -> HRESULT,
    fn ExitProcess(ExitCode: ULONG,) -> HRESULT,
    fn LoadModule(
        ImageFileHandle: ULONG64,
        BaseOffset: ULONG64,
        ModuleSize: ULONG,
        ModuleName: PCWSTR,
        ImageName: PCWSTR,
        CheckSum: ULONG,
        TimeDateStamp: ULONG,
    ) -> HRESULT,
    fn UnloadModule(
        ImageBaseName: PCWSTR,
        BaseOffset: ULONG64,
    ) -> HRESULT,
    fn SystemError(
        Error: ULONG,
        Level: ULONG,
    ) -> HRESULT,
    fn SessionStatus(Status: ULONG,) -> HRESULT,
    fn ChangeDebuggeeState(
        Flags: ULONG,
        Argument: ULONG64,
    ) -> HRESULT,
    fn ChangeEngineState(
        Flags: ULONG,
        Argument: ULONG64,
    ) -> HRESULT,
    fn ChangeSymbolState(
        Flags: ULONG,
        Argument: ULONG64,
    ) -> HRESULT,
});
STRUCT!{struct DEBUG_EVENT_CONTEXT {
    Size: ULONG,
    ProcessEngineId: ULONG,
    ThreadEngineId: ULONG,
    FrameEngineId: ULONG,
}}
pub type PDEBUG_EVENT_CONTEXT = *mut DEBUG_EVENT_CONTEXT;
RIDL!(#[uuid(0x61a4905b, 0x23f9, 0x4247, 0xb3, 0xc5, 0x53, 0xd0, 0x87, 0x52, 0x9a, 0xb7)]
interface IDebugEventContextCallbacks(IDebugEventContextCallbacksVtbl): IUnknown(IUnknownVtbl) {
    fn GetInterestMask(Mask: PULONG,) -> HRESULT,
    fn Breakpoint(
        Bp: PDEBUG_BREAKPOINT2,
        Context: PVOID,
        ContextSize: ULONG,
    ) -> HRESULT,
    fn Exception(
        Exception: PEXCEPTION_RECORD64,
        FirstChance: ULONG,
        Context: PVOID,
        ContextSize: ULONG,
    ) -> HRESULT,
    fn CreateThread(
        Handle: ULONG64,
        DataOffset: ULONG64,
        StartOffset: ULONG64,
        Context: PVOID,
        ContextSize: ULONG,
    ) -> HRESULT,
    fn ExitThread(
        ExitCode: ULONG,
        Context: PVOID,
        ContextSize: ULONG,
    ) -> HRESULT,
    fn CreateProcess(
        ImageFileHandle: ULONG64,
        Handle: ULONG64,
        BaseOffset: ULONG64,
        ModuleSize: ULONG,
        ModuleName: PCWSTR,
        ImageName: PCWSTR,
        CheckSum: ULONG,
        TimeDateStamp: ULONG,
        InitialThreadHandle: ULONG64,
        ThreadDataOffset: ULONG64,
        StartOffset: ULONG64,
        Context: PVOID,
        ContextSize: ULONG,
    ) -> HRESULT,
    fn ExitProcess(
        ExitCode: ULONG,
        Context: PVOID,
        ContextSize: ULONG,
    ) -> HRESULT,
    fn LoadModule(
        ImageFileHandle: ULONG64,
        BaseOffset: ULONG64,
        ModuleSize: ULONG,
        ModuleName: PCWSTR,
        ImageName: PCWSTR,
        CheckSum: ULONG,
        TimeDateStamp: ULONG,
        Context: PVOID,
        ContextSize: ULONG,
    ) -> HRESULT,
    fn UnloadModule(
        ImageBaseName: PCWSTR,
        BaseOffset: ULONG64,
        Context: PVOID,
        ContextSize: ULONG,
    ) -> HRESULT,
    fn SystemError(
        Error: ULONG,
        Level: ULONG,
        Context: PVOID,
        ContextSize: ULONG,
    ) -> HRESULT,
    fn SessionStatus(Status: ULONG,) -> HRESULT,
    fn ChangeDebuggeeState(
        Flags: ULONG,
        Argument: ULONG64,
        Context: PVOID,
        ContextSize: ULONG,
    ) -> HRESULT,
    fn ChangeEngineState(
        Flags: ULONG,
        Argument: ULONG64,
        Context: PVOID,
        ContextSize: ULONG,
    ) -> HRESULT,
    fn ChangeSymbolState(
        Flags: ULONG,
        Argument: ULONG64,
    ) -> HRESULT,
});
RIDL!(#[uuid(0x9f50e42c, 0xf136, 0x499e, 0x9a, 0x97, 0x73, 0x03, 0x6c, 0x94, 0xed, 0x2d)]
interface IDebugInputCallbacks(IDebugInputCallbacksVtbl): IUnknown(IUnknownVtbl) {
    fn StartInput(BufferSize: ULONG,) -> HRESULT,
    fn EndInput() -> HRESULT,
});
RIDL!(#[uuid(0x4bf58045, 0xd654, 0x4c40, 0xb0, 0xaf, 0x68, 0x30, 0x90, 0xf3, 0x56, 0xdc)]
interface IDebugOutputCallbacks(IDebugOutputCallbacksVtbl): IUnknown(IUnknownVtbl) {
    fn Output(
        Mask: ULONG,
        Text: PCSTR,
    ) -> HRESULT,
});
RIDL!(#[uuid(0x4c7fd663, 0xc394, 0x4e26, 0x8e, 0xf1, 0x34, 0xad, 0x5e, 0xd3, 0x76, 0x4c)]
interface IDebugOutputCallbacksWide(IDebugOutputCallbacksWideVtbl): IUnknown(IUnknownVtbl) {
    fn Output(
        Mask: ULONG,
        Text: PCWSTR,
    ) -> HRESULT,
});
pub const DEBUG_OUTCBI_EXPLICIT_FLUSH: ULONG = 0x00000001;
pub const DEBUG_OUTCBI_TEXT: ULONG = 0x00000002;
pub const DEBUG_OUTCBI_DML: ULONG = 0x00000004;
pub const DEBUG_OUTCBI_ANY_FORMAT: ULONG = 0x00000006;
pub const DEBUG_OUTCB_TEXT: ULONG = 0;
pub const DEBUG_OUTCB_DML: ULONG = 1;
pub const DEBUG_OUTCB_EXPLICIT_FLUSH: ULONG = 2;
pub const DEBUG_OUTCBF_COMBINED_EXPLICIT_FLUSH: ULONG = 0x00000001;
pub const DEBUG_OUTCBF_DML_HAS_TAGS: ULONG = 0x00000002;
pub const DEBUG_OUTCBF_DML_HAS_SPECIAL_CHARACTERS: ULONG = 0x00000004;
RIDL!(#[uuid(0x67721fe9, 0x56d2, 0x4a44, 0xa3, 0x25, 0x2b, 0x65, 0x51, 0x3c, 0xe6, 0xeb)]
interface IDebugOutputCallbacks2(IDebugOutputCallbacks2Vtbl): IDebugOutputCallbacks(IDebugOutputCallbacksVtbl) {
    fn GetInterestMask(Mask: PULONG,) -> HRESULT,
    fn Output2(
        Which: ULONG,
        Flags: ULONG,
        Arg: ULONG64,
        Text: PCWSTR,
    ) -> HRESULT,
});
pub const DEBUG_REGISTERS_DEFAULT: ULONG = 0x00000000;
pub const DEBUG_REGISTERS_INT32: ULONG = 0x00000001;
pub const DEBUG_REGISTERS_INT64: ULONG = 0x00000002;
pub const DEBUG_REGISTERS_FLOAT: ULONG = 0x00000004;
pub const DEBUG_REGISTERS_ALL: ULONG = 0x00000007;
pub const DEBUG_REGISTER_SUB_REGISTER: ULONG = 0x00000001;
STRUCT!{struct DEBUG_REGISTER_DESCRIPTION {
    Type: ULONG,
    Flags: ULONG,
    SubregMaster: ULONG,
    SubregLength: ULONG,
    SubregMask: ULONG64,
    SubregShift: ULONG,
    Reserved0: ULONG,
}}
pub type PDEBUG_REGISTER_DESCRIPTION = *mut DEBUG_REGISTER_DESCRIPTION;
RIDL!(#[uuid(0xce289126, 0x9e84, 0x45a7, 0x93, 0x7e, 0x67, 0xbb, 0x18, 0x69, 0x14, 0x93)]
interface IDebugRegisters(IDebugRegistersVtbl): IUnknown(IUnknownVtbl) {
    fn GetNumberRegisters(Number: PULONG,) -> HRESULT,
    fn GetDescription(
        Register: ULONG,
        NameBuffer: PSTR,
        NameBufferSize: ULONG,
        NameSize: PULONG,
        Desc: PDEBUG_REGISTER_DESCRIPTION,
    ) -> HRESULT,
    fn GetIndexByName(
        Name: PCSTR,
        Index: PULONG,
    ) -> HRESULT,
    fn GetValue(
        Register: ULONG,
        Value: PDEBUG_VALUE,
    ) -> HRESULT,
    fn SetValue(
        Register: ULONG,
        Value: PDEBUG_VALUE,
    ) -> HRESULT,
    fn GetValues(
        Count: ULONG,
        Indices: PULONG,
        Start: ULONG,
        Values: PDEBUG_VALUE,
    ) -> HRESULT,
    fn SetValues(
        Count: ULONG,
        Indices: PULONG,
        Start: ULONG,
        Values: PDEBUG_VALUE,
    ) -> HRESULT,
    fn OutputRegisters(
        OutputControl: ULONG,
        Flags: ULONG,
    ) -> HRESULT,
    fn GetInstructionOffset(Offset: PULONG64,) -> HRESULT,
    fn GetStackOffset(Offset: PULONG64,) -> HRESULT,
    fn GetFrameOffset(Offset: PULONG64,) -> HRESULT,
});
pub const DEBUG_REGSRC_DEBUGGEE: ULONG = 0x00000000;
pub const DEBUG_REGSRC_EXPLICIT: ULONG = 0x00000001;
pub const DEBUG_REGSRC_FRAME: ULONG = 0x00000002;
RIDL!(#[uuid(0x1656afa9, 0x19c6, 0x4e3a, 0x97, 0xe7, 0x5d, 0xc9, 0x16, 0x0c, 0xf9, 0xc4)]
interface IDebugRegisters2(IDebugRegisters2Vtbl): IDebugRegisters(IDebugRegistersVtbl) {
    fn GetDescriptionWide(
        Register: ULONG,
        NameBuffer: PWSTR,
        NameBufferSize: ULONG,
        NameSize: PULONG,
        Desc: PDEBUG_REGISTER_DESCRIPTION,
    ) -> HRESULT,
    fn GetIndexByNameWide(
        Name: PCWSTR,
        Index: PULONG,
    ) -> HRESULT,
    fn GetNumberPseudoRegisters(Number: PULONG,) -> HRESULT,
    fn GetPseudoDescription(
        Register: ULONG,
        NameBuffer: PSTR,
        NameBufferSize: ULONG,
        NameSize: PULONG,
        TypeModule: PULONG64,
        TypeId: PULONG,
    ) -> HRESULT,
    fn GetPseudoDescriptionWide(
        Register: ULONG,
        NameBuffer: PWSTR,
        NameBufferSize: ULONG,
        NameSize: PULONG,
        TypeModule: PULONG64,
        TypeId: PULONG,
    ) -> HRESULT,
    fn GetPseudoIndexByName(
        Name: PCSTR,
        Index: PULONG,
    ) -> HRESULT,
    fn GetPseudoIndexByNameWide(
        Name: PCWSTR,
        Index: PULONG,
    ) -> HRESULT,
    fn GetPseudoValues(
        Source: ULONG,
        Count: ULONG,
        Indices: PULONG,
        Start: ULONG,
        Values: PDEBUG_VALUE,
    ) -> HRESULT,
    fn SetPseudoValues(
        Source: ULONG,
        Count: ULONG,
        Indices: PULONG,
        Start: ULONG,
        Values: PDEBUG_VALUE,
    ) -> HRESULT,
    fn GetValues2(
        Source: ULONG,
        Count: ULONG,
        Indices: PULONG,
        Start: ULONG,
        Values: PDEBUG_VALUE,
    ) -> HRESULT,
    fn SetValues2(
        Source: ULONG,
        Count: ULONG,
        Indices: PULONG,
        Start: ULONG,
        Values: PDEBUG_VALUE,
    ) -> HRESULT,
    fn OutputRegisters2(
        OutputControl: ULONG,
        Source: ULONG,
        Flags: ULONG,
    ) -> HRESULT,
    fn GetInstructionOffset2(
        Source: ULONG,
        Offset: PULONG64,
    ) -> HRESULT,
    fn GetStackOffset2(
        Source: ULONG,
        Offset: PULONG64,
    ) -> HRESULT,
    fn GetFrameOffset2(
        Source: ULONG,
        Offset: PULONG64,
    ) -> HRESULT,
});
pub const DEBUG_OUTPUT_SYMBOLS_DEFAULT: ULONG = 0x00000000;
pub const DEBUG_OUTPUT_SYMBOLS_NO_NAMES: ULONG = 0x00000001;
pub const DEBUG_OUTPUT_SYMBOLS_NO_OFFSETS: ULONG = 0x00000002;
pub const DEBUG_OUTPUT_SYMBOLS_NO_VALUES: ULONG = 0x00000004;
pub const DEBUG_OUTPUT_SYMBOLS_NO_TYPES: ULONG = 0x00000010;
pub const DEBUG_OUTPUT_NAME_END: &'static str = "**NAME**";
pub const DEBUG_OUTPUT_OFFSET_END: &'static str = "**OFF**";
pub const DEBUG_OUTPUT_VALUE_END: &'static str = "**VALUE**";
pub const DEBUG_OUTPUT_TYPE_END: &'static str = "**TYPE**";
pub const DEBUG_SYMBOL_EXPANSION_LEVEL_MASK: ULONG = 0x0000000f;
pub const DEBUG_SYMBOL_EXPANDED: ULONG = 0x00000010;
pub const DEBUG_SYMBOL_READ_ONLY: ULONG = 0x00000020;
pub const DEBUG_SYMBOL_IS_ARRAY: ULONG = 0x00000040;
pub const DEBUG_SYMBOL_IS_FLOAT: ULONG = 0x00000080;
pub const DEBUG_SYMBOL_IS_ARGUMENT: ULONG = 0x00000100;
pub const DEBUG_SYMBOL_IS_LOCAL: ULONG = 0x00000200;
STRUCT!{struct DEBUG_SYMBOL_PARAMETERS {
    Module: ULONG64,
    TypeId: ULONG,
    ParentSymbol: ULONG,
    SubElements: ULONG,
    Flags: ULONG,
    Reserved: ULONG64,
}}
pub type PDEBUG_SYMBOL_PARAMETERS = *mut DEBUG_SYMBOL_PARAMETERS;
RIDL!(#[uuid(0xf2528316, 0x0f1a, 0x4431, 0xae, 0xed, 0x11, 0xd0, 0x96, 0xe1, 0xe2, 0xab)]
interface IDebugSymbolGroup(IDebugSymbolGroupVtbl): IUnknown(IUnknownVtbl) {
    fn GetNumberSymbols(Number: PULONG,) -> HRESULT,
    fn AddSymbol(
        Name: PCSTR,
        Index: PULONG,
    ) -> HRESULT,
    fn RemoveSymbolByName(Name: PCSTR,) -> HRESULT,
    fn RemoveSymbolByIndex(Index: ULONG,) -> HRESULT,
    fn GetSymbolName(
        Index: ULONG,
        Buffer: PSTR,
        BufferSize: ULONG,
        NameSize: PULONG,
    ) -> HRESULT,
    fn GetSymbolParameters(
        Start: ULONG,
        Count: ULONG,
        Params: PDEBUG_SYMBOL_PARAMETERS,
    ) -> HRESULT,
    fn ExpandSymbol(
        Index: ULONG,
        Expand: BOOL,
    ) -> HRESULT,
    fn OutputSymbols(
        OutputControl: ULONG,
        Flags: ULONG,
        Start: ULONG,
        Count: ULONG,
    ) -> HRESULT,
    fn WriteSymbol(
        Index: ULONG,
        Value: PCSTR,
    ) -> HRESULT,
    fn OutputAsType(
        Index: ULONG,
        Type: PCSTR,
    ) -> HRESULT,
});
pub const DEBUG_SYMENT_IS_CODE: ULONG = 0x00000001;
pub const DEBUG_SYMENT_IS_DATA: ULONG = 0x00000002;
pub const DEBUG_SYMENT_IS_PARAMETER: ULONG = 0x00000004;
pub const DEBUG_SYMENT_IS_LOCAL: ULONG = 0x00000008;
pub const DEBUG_SYMENT_IS_MANAGED: ULONG = 0x00000010;
pub const DEBUG_SYMENT_IS_SYNTHETIC: ULONG = 0x00000020;
STRUCT!{struct DEBUG_SYMBOL_ENTRY {
    ModuleBase: ULONG64,
    Offset: ULONG64,
    Id: ULONG64,
    Arg64: ULONG64,
    Size: ULONG,
    Flags: ULONG,
    TypeId: ULONG,
    NameSize: ULONG,
    Token: ULONG,
    Tag: ULONG,
    Arg32: ULONG,
    Reserved: ULONG,
}}
pub type PDEBUG_SYMBOL_ENTRY = *mut DEBUG_SYMBOL_ENTRY;
RIDL!(#[uuid(0x6a7ccc5f, 0xfb5e, 0x4dcc, 0xb4, 0x1c, 0x6c, 0x20, 0x30, 0x7b, 0xcc, 0xc7)]
interface IDebugSymbolGroup2(IDebugSymbolGroup2Vtbl): IDebugSymbolGroup(IDebugSymbolGroupVtbl) {
    fn AddSymbolWide(
        Name: PCWSTR,
        Index: PULONG,
    ) -> HRESULT,
    fn RemoveSymbolByNameWide(Name: PCWSTR,) -> HRESULT,
    fn GetSymbolNameWide(
        Index: ULONG,
        Buffer: PWSTR,
        BufferSize: ULONG,
        NameSize: PULONG,
    ) -> HRESULT,
    fn WriteSymbolWide(
        Index: ULONG,
        Value: PCWSTR,
    ) -> HRESULT,
    fn OutputAsTypeWide(
        Index: ULONG,
        Type: PCWSTR,
    ) -> HRESULT,
    fn GetSymbolTypeName(
        Index: ULONG,
        Buffer: PSTR,
        BufferSize: ULONG,
        NameSize: PULONG,
    ) -> HRESULT,
    fn GetSymbolTypeNameWide(
        Index: ULONG,
        Buffer: PWSTR,
        BufferSize: ULONG,
        NameSize: PULONG,
    ) -> HRESULT,
    fn GetSymbolSize(
        Index: ULONG,
        Size: PULONG,
    ) -> HRESULT,
    fn GetSymbolOffset(
        Index: ULONG,
        Offset: PULONG64,
    ) -> HRESULT,
    fn GetSymbolRegister(
        Index: ULONG,
        Register: PULONG,
    ) -> HRESULT,
    fn GetSymbolValueText(
        Index: ULONG,
        Buffer: PSTR,
        BufferSize: ULONG,
        NameSize: PULONG,
    ) -> HRESULT,
    fn GetSymbolValueTextWide(
        Index: ULONG,
        Buffer: PWSTR,
        BufferSize: ULONG,
        NameSize: PULONG,
    ) -> HRESULT,
    fn GetSymbolEntryInformation(
        Index: ULONG,
        Entry: PDEBUG_SYMBOL_ENTRY,
    ) -> HRESULT,
});
pub const DEBUG_MODULE_LOADED: ULONG = 0x00000000;
pub const DEBUG_MODULE_UNLOADED: ULONG = 0x00000001;
pub const DEBUG_MODULE_USER_MODE: ULONG = 0x00000002;
pub const DEBUG_MODULE_EXE_MODULE: ULONG = 0x00000004;
pub const DEBUG_MODULE_EXPLICIT: ULONG = 0x00000008;
pub const DEBUG_MODULE_SECONDARY: ULONG = 0x00000010;
pub const DEBUG_MODULE_SYNTHETIC: ULONG = 0x00000020;
pub const DEBUG_MODULE_SYM_BAD_CHECKSUM: ULONG = 0x00010000;
pub const DEBUG_SYMTYPE_NONE: ULONG = 0;
pub const DEBUG_SYMTYPE_COFF: ULONG = 1;
pub const DEBUG_SYMTYPE_CODEVIEW: ULONG = 2;
pub const DEBUG_SYMTYPE_PDB: ULONG = 3;
pub const DEBUG_SYMTYPE_EXPORT: ULONG = 4;
pub const DEBUG_SYMTYPE_DEFERRED: ULONG = 5;
pub const DEBUG_SYMTYPE_SYM: ULONG = 6;
pub const DEBUG_SYMTYPE_DIA: ULONG = 7;
STRUCT!{struct DEBUG_MODULE_PARAMETERS {
    Base: ULONG64,
    Size: ULONG,
    TimeDateStamp: ULONG,
    Checksum: ULONG,
    Flags: ULONG,
    SymbolType: ULONG,
    ImageNameSize: ULONG,
    ModuleNameSize: ULONG,
    LoadedImageNameSize: ULONG,
    SymbolFileNameSize: ULONG,
    MappedImageNameSize: ULONG,
    Reserved: [ULONG64; 2],
}}
pub type PDEBUG_MODULE_PARAMETERS = *mut DEBUG_MODULE_PARAMETERS;
pub const DEBUG_SCOPE_GROUP_ARGUMENTS: ULONG = 0x00000001;
pub const DEBUG_SCOPE_GROUP_LOCALS: ULONG = 0x00000002;
pub const DEBUG_SCOPE_GROUP_ALL: ULONG = 0x00000003;
pub const DEBUG_SCOPE_GROUP_BY_DATAMODEL: ULONG = 0x00000004;
pub const DEBUG_SCOPE_GROUP_VALID_FLAGS: ULONG =
    (DEBUG_SCOPE_GROUP_ALL | DEBUG_SCOPE_GROUP_BY_DATAMODEL);
pub const DEBUG_OUTTYPE_DEFAULT: ULONG = 0x00000000;
pub const DEBUG_OUTTYPE_NO_INDENT: ULONG = 0x00000001;
pub const DEBUG_OUTTYPE_NO_OFFSET: ULONG = 0x00000002;
pub const DEBUG_OUTTYPE_VERBOSE: ULONG = 0x00000004;
pub const DEBUG_OUTTYPE_COMPACT_OUTPUT: ULONG = 0x00000008;
#[inline]
pub fn DEBUG_OUTTYPE_RECURSION_LEVEL(Max: ULONG) -> ULONG {
    (((Max) & 0xf) << 4)
}
pub const DEBUG_OUTTYPE_ADDRESS_OF_FIELD: ULONG = 0x00010000;
pub const DEBUG_OUTTYPE_ADDRESS_AT_END: ULONG = 0x00020000;
pub const DEBUG_OUTTYPE_BLOCK_RECURSE: ULONG = 0x00200000;
pub const DEBUG_FIND_SOURCE_DEFAULT: ULONG = 0x00000000;
pub const DEBUG_FIND_SOURCE_FULL_PATH: ULONG = 0x00000001;
pub const DEBUG_FIND_SOURCE_BEST_MATCH: ULONG = 0x00000002;
pub const DEBUG_FIND_SOURCE_NO_SRCSRV: ULONG = 0x00000004;
pub const DEBUG_FIND_SOURCE_TOKEN_LOOKUP: ULONG = 0x00000008;
pub const DEBUG_FIND_SOURCE_WITH_CHECKSUM: ULONG = 0x00000010;
pub const DEBUG_INVALID_OFFSET: ULONG64 = -1i64 as ULONG64;
pub const MODULE_ORDERS_MASK: ULONG = 0xF0000000;
pub const MODULE_ORDERS_LOADTIME: ULONG = 0x10000000;
pub const MODULE_ORDERS_MODULENAME: ULONG = 0x20000000;
RIDL!(#[uuid(0xf2528316, 0x0f1a, 0x4431, 0xae, 0xed, 0x11, 0xd0, 0x96, 0xe1, 0xe2, 0xab)]
interface IDebugSymbols(IDebugSymbolsVtbl): IUnknown(IUnknownVtbl) {
    fn GetSymbolOptions(Options: PULONG,) -> HRESULT,
    fn AddSymbolOptions(Options: ULONG,) -> HRESULT,
    fn RemoveSymbolOptions(Options: ULONG,) -> HRESULT,
    fn SetSymbolOptions(Options: ULONG,) -> HRESULT,
    fn GetNameByOffset(
        Offset: ULONG64,
        NameBuffer: PSTR,
        NameBufferSize: ULONG,
        NameSize: PULONG,
        Displacement: PULONG64,
    ) -> HRESULT,
    fn GetOffsetByName(
        Symbol: PCSTR,
        Offset: PULONG64,
    ) -> HRESULT,
    fn GetNearNameByOffset(
        Offset: ULONG64,
        Delta: LONG,
        NameBuffer: PSTR,
        NameBufferSize: ULONG,
        NameSize: PULONG,
        Displacement: PULONG64,
    ) -> HRESULT,
    fn GetLineByOffset(
        Offset: ULONG64,
        Line: PULONG,
        FileBuffer: PSTR,
        FileBufferSize: ULONG,
        FileSize: PULONG,
        Displacement: PULONG64,
    ) -> HRESULT,
    fn GetOffsetByLine(
        Line: ULONG,
        File: PCSTR,
        Offset: PULONG64,
    ) -> HRESULT,
    fn GetNumberModules(
        Loaded: PULONG,
        Unloaded: PULONG,
    ) -> HRESULT,
    fn GetModuleByIndex(
        Index: ULONG,
        Base: PULONG64,
    ) -> HRESULT,
    fn GetModuleByModuleName(
        Name: PCSTR,
        StartIndex: ULONG,
        Index: PULONG,
        Base: PULONG64,
    ) -> HRESULT,
    fn GetModuleByOffset(
        Offset: ULONG64,
        StartIndex: ULONG,
        Index: PULONG,
        Base: PULONG64,
    ) -> HRESULT,
    fn GetModuleNames(
        Index: ULONG,
        Base: ULONG64,
        ImageNameBuffer: PSTR,
        ImageNameBufferSize: ULONG,
        ImageNameSize: PULONG,
        ModuleNameBuffer: PSTR,
        ModuleNameBufferSize: ULONG,
        ModuleNameSize: PULONG,
        LoadedImageNameBuffer: PSTR,
        LoadedImageNameBufferSize: ULONG,
        LoadedImageNameSize: PULONG,
    ) -> HRESULT,
    fn GetModuleParameters(
        Count: ULONG,
        Bases: PULONG64,
        Start: ULONG,
        Params: PDEBUG_MODULE_PARAMETERS,
    ) -> HRESULT,
    fn GetSymbolModule(
        Symbol: PCSTR,
        Base: PULONG64,
    ) -> HRESULT,
    fn GetTypeName(
        Module: ULONG64,
        TypeId: ULONG,
        NameBuffer: PSTR,
        NameBufferSize: ULONG,
        NameSize: PULONG,
    ) -> HRESULT,
    fn GetTypeId(
        Module: ULONG64,
        Name: PCSTR,
        TypeId: PULONG,
    ) -> HRESULT,
    fn GetTypeSize(
        Module: ULONG64,
        TypeId: ULONG,
        Size: PULONG,
    ) -> HRESULT,
    fn GetFieldOffset(
        Module: ULONG64,
        TypeId: ULONG,
        Field: PCSTR,
        Offset: PULONG,
    ) -> HRESULT,
    fn GetSymbolTypeId(
        Symbol: PCSTR,
        TypeId: PULONG,
        Module: PULONG64,
    ) -> HRESULT,
    fn GetOffsetTypeId(
        Offset: ULONG64,
        TypeId: PULONG,
        Module: PULONG64,
    ) -> HRESULT,
    fn ReadTypedDataVirtual(
        Offset: ULONG64,
        Module: ULONG64,
        TypeId: ULONG,
        Buffer: PVOID,
        BufferSize: ULONG,
        BytesRead: PULONG,
    ) -> HRESULT,
    fn WriteTypedDataVirtual(
        Offset: ULONG64,
        Module: ULONG64,
        TypeId: ULONG,
        Buffer: PVOID,
        BufferSize: ULONG,
        BytesWritten: PULONG,
    ) -> HRESULT,
    fn OutputTypedDataVirtual(
        OutputControl: ULONG,
        Offset: ULONG64,
        Module: ULONG64,
        TypeId: ULONG,
        Flags: ULONG,
    ) -> HRESULT,
    fn ReadTypedDataPhysical(
        Offset: ULONG64,
        Module: ULONG64,
        TypeId: ULONG,
        Buffer: PVOID,
        BufferSize: ULONG,
        BytesRead: PULONG,
    ) -> HRESULT,
    fn WriteTypedDataPhysical(
        Offset: ULONG64,
        Module: ULONG64,
        TypeId: ULONG,
        Buffer: PVOID,
        BufferSize: ULONG,
        BytesWritten: PULONG,
    ) -> HRESULT,
    fn OutputTypedDataPhysical(
        OutputControl: ULONG,
        Offset: ULONG64,
        Module: ULONG64,
        TypeId: ULONG,
        Flags: ULONG,
    ) -> HRESULT,
    fn GetScope(
        InstructionOffset: PULONG64,
        ScopeFrame: PDEBUG_STACK_FRAME,
        ScopeContext: PVOID,
        ScopeContextSize: ULONG,
    ) -> HRESULT,
    fn SetScope(
        InstructionOffset: ULONG64,
        ScopeFrame: PDEBUG_STACK_FRAME,
        ScopeContext: PVOID,
        ScopeContextSize: ULONG,
    ) -> HRESULT,
    fn ResetScope() -> HRESULT,
    fn GetScopeSymbolGroup(
        Flags: ULONG,
        Update: PDEBUG_SYMBOL_GROUP,
        Symbols: *mut PDEBUG_SYMBOL_GROUP,
    ) -> HRESULT,
    fn CreateSymbolGroup(Group: *mut PDEBUG_SYMBOL_GROUP,) -> HRESULT,
    fn StartSymbolMatch(
        Pattern: PCSTR,
        Handle: PULONG64,
    ) -> HRESULT,
    fn GetNextSymbolMatch(
        Handle: ULONG64,
        Buffer: PSTR,
        BufferSize: ULONG,
        MatchSize: PULONG,
        Offset: PULONG64,
    ) -> HRESULT,
    fn EndSymbolMatch(Handle: ULONG64,) -> HRESULT,
    fn Reload(Module: PCSTR,) -> HRESULT,
    fn GetSymbolPath(
        Buffer: PSTR,
        BufferSize: ULONG,
        PathSize: PULONG,
    ) -> HRESULT,
    fn SetSymbolPath(Path: PCSTR,) -> HRESULT,
    fn AppendSymbolPath(Addition: PCSTR,) -> HRESULT,
    fn GetImagePath(
        Buffer: PSTR,
        BufferSize: ULONG,
        PathSize: PULONG,
    ) -> HRESULT,
    fn SetImagePath(Path: PCSTR,) -> HRESULT,
    fn AppendImagePath(Addition: PCSTR,) -> HRESULT,
    fn GetSourcePath(
        Buffer: PSTR,
        BufferSize: ULONG,
        PathSize: PULONG,
    ) -> HRESULT,
    fn GetSourcePathElement(
        Index: ULONG,
        Buffer: PSTR,
        BufferSize: ULONG,
        ElementSize: PULONG,
    ) -> HRESULT,
    fn SetSourcePath(Path: PCSTR,) -> HRESULT,
    fn AppendSourcePath(Addition: PCSTR,) -> HRESULT,
    fn FindSourceFile(
        StartElement: ULONG,
        File: PCSTR,
        Flags: ULONG,
        FoundElement: PULONG,
        Buffer: PSTR,
        BufferSize: ULONG,
        FoundSize: PULONG,
    ) -> HRESULT,
    fn GetSourceFileLineOffsets(
        File: PCSTR,
        Buffer: PULONG64,
        BufferLines: ULONG,
        FileLines: PULONG,
    ) -> HRESULT,
});
pub const DEBUG_MODNAME_IMAGE: ULONG = 0x00000000;
pub const DEBUG_MODNAME_MODULE: ULONG = 0x00000001;
pub const DEBUG_MODNAME_LOADED_IMAGE: ULONG = 0x00000002;
pub const DEBUG_MODNAME_SYMBOL_FILE: ULONG = 0x00000003;
pub const DEBUG_MODNAME_MAPPED_IMAGE: ULONG = 0x00000004;
pub const DEBUG_TYPEOPTS_UNICODE_DISPLAY: ULONG = 0x00000001;
pub const DEBUG_TYPEOPTS_LONGSTATUS_DISPLAY: ULONG = 0x00000002;
pub const DEBUG_TYPEOPTS_FORCERADIX_OUTPUT: ULONG = 0x00000004;
pub const DEBUG_TYPEOPTS_MATCH_MAXSIZE: ULONG = 0x00000008;
RIDL!(#[uuid(0x3a707211, 0xafdd, 0x4495, 0xad, 0x4f, 0x56, 0xfe, 0xcd, 0xf8, 0x16, 0x3f)]
interface IDebugSymbols2(IDebugSymbols2Vtbl): IDebugSymbols(IDebugSymbolsVtbl) {
    fn GetModuleVersionInformation(
        Index: ULONG,
        Base: ULONG64,
        Item: PCSTR,
        Buffer: PVOID,
        BufferSize: ULONG,
        VerInfoSize: PULONG,
    ) -> HRESULT,
    fn GetModuleNameString(
        Which: ULONG,
        Index: ULONG,
        Base: ULONG64,
        Buffer: PSTR,
        BufferSize: ULONG,
        NameSize: PULONG,
    ) -> HRESULT,
    fn GetConstantName(
        Module: ULONG64,
        TypeId: ULONG,
        Value: ULONG64,
        NameBuffer: PSTR,
        NameBufferSize: ULONG,
        NameSize: PULONG,
    ) -> HRESULT,
    fn GetFieldName(
        Module: ULONG64,
        TypeId: ULONG,
        FieldIndex: ULONG,
        NameBuffer: PSTR,
        NameBufferSize: ULONG,
        NameSize: PULONG,
    ) -> HRESULT,
    fn GetTypeOptions(Options: PULONG,) -> HRESULT,
    fn AddTypeOptions(Options: ULONG,) -> HRESULT,
    fn RemoveTypeOptions(Options: ULONG,) -> HRESULT,
    fn SetTypeOptions(Options: ULONG,) -> HRESULT,
});
pub const DEBUG_GETMOD_DEFAULT: ULONG = 0x00000000;
pub const DEBUG_GETMOD_NO_LOADED_MODULES: ULONG = 0x00000001;
pub const DEBUG_GETMOD_NO_UNLOADED_MODULES: ULONG = 0x00000002;
pub const DEBUG_ADDSYNTHMOD_DEFAULT: ULONG = 0x00000000;
pub const DEBUG_ADDSYNTHMOD_ZEROBASE: ULONG = 0x00000001;
pub const DEBUG_ADDSYNTHSYM_DEFAULT: ULONG = 0x00000000;
pub const DEBUG_OUTSYM_DEFAULT: ULONG = 0x00000000;
pub const DEBUG_OUTSYM_FORCE_OFFSET: ULONG = 0x00000001;
pub const DEBUG_OUTSYM_SOURCE_LINE: ULONG = 0x00000002;
pub const DEBUG_OUTSYM_ALLOW_DISPLACEMENT: ULONG = 0x00000004;
pub const DEBUG_GETFNENT_DEFAULT: ULONG = 0x00000000;
pub const DEBUG_GETFNENT_RAW_ENTRY_ONLY: ULONG = 0x00000001;
STRUCT!{struct DEBUG_MODULE_AND_ID {
    ModuleBase: ULONG64,
    Id: ULONG64,
}}
pub type PDEBUG_MODULE_AND_ID = *mut DEBUG_MODULE_AND_ID;
pub const DEBUG_SOURCE_IS_STATEMENT: ULONG = 0x00000001;
pub const DEBUG_GSEL_DEFAULT: ULONG = 0x00000000;
pub const DEBUG_GSEL_NO_SYMBOL_LOADS: ULONG = 0x00000001;
pub const DEBUG_GSEL_ALLOW_LOWER: ULONG = 0x00000002;
pub const DEBUG_GSEL_ALLOW_HIGHER: ULONG = 0x00000004;
pub const DEBUG_GSEL_NEAREST_ONLY: ULONG = 0x00000008;
pub const DEBUG_GSEL_INLINE_CALLSITE: ULONG = 0x00000010;
STRUCT!{struct DEBUG_SYMBOL_SOURCE_ENTRY {
    ModuleBase: ULONG64,
    Offset: ULONG64,
    FileNameId: ULONG64,
    EngineInternal: ULONG64,
    Size: ULONG,
    Flags: ULONG,
    FileNameSize: ULONG,
    StartLine: ULONG,
    EndLine: ULONG,
    StartColumn: ULONG,
    EndColumn: ULONG,
    Reserved: ULONG,
}}
pub type PDEBUG_SYMBOL_SOURCE_ENTRY = *mut DEBUG_SYMBOL_SOURCE_ENTRY;
RIDL!(#[uuid(0xf02fbecc, 0x50ac, 0x4f36, 0x9a, 0xd9, 0xc9, 0x75, 0xe8, 0xf3, 0x2f, 0xf8)]
interface IDebugSymbols3(IDebugSymbols3Vtbl): IDebugSymbols2(IDebugSymbols2Vtbl) {
    fn GetNameByOffsetWide(
        Offset: ULONG64,
        NameBuffer: PWSTR,
        NameBufferSize: ULONG,
        NameSize: PULONG,
        Displacement: PULONG64,
    ) -> HRESULT,
    fn GetOffsetByNameWide(
        Symbol: PCWSTR,
        Offset: PULONG64,
    ) -> HRESULT,
    fn GetNearNameByOffsetWide(
        Offset: ULONG64,
        Delta: LONG,
        NameBuffer: PWSTR,
        NameBufferSize: ULONG,
        NameSize: PULONG,
        Displacement: PULONG64,
    ) -> HRESULT,
    fn GetLineByOffsetWide(
        Offset: ULONG64,
        Line: PULONG,
        FileBuffer: PWSTR,
        FileBufferSize: ULONG,
        FileSize: PULONG,
        Displacement: PULONG64,
    ) -> HRESULT,
    fn GetOffsetByLineWide(
        Line: ULONG,
        File: PCWSTR,
        Offset: PULONG64,
    ) -> HRESULT,
    fn GetModuleByModuleNameWide(
        Name: PCWSTR,
        StartIndex: ULONG,
        Index: PULONG,
        Base: PULONG64,
    ) -> HRESULT,
    fn GetSymbolModuleWide(
        Symbol: PCWSTR,
        Base: PULONG64,
    ) -> HRESULT,
    fn GetTypeNameWide(
        Module: ULONG64,
        TypeId: ULONG,
        NameBuffer: PWSTR,
        NameBufferSize: ULONG,
        NameSize: PULONG,
    ) -> HRESULT,
    fn GetTypeIdWide(
        Module: ULONG64,
        Name: PCWSTR,
        TypeId: PULONG,
    ) -> HRESULT,
    fn GetFieldOffsetWide(
        Module: ULONG64,
        TypeId: ULONG,
        Field: PCWSTR,
        Offset: PULONG,
    ) -> HRESULT,
    fn GetSymbolTypeIdWide(
        Symbol: PCWSTR,
        TypeId: PULONG,
        Module: PULONG64,
    ) -> HRESULT,
    fn GetScopeSymbolGroup2(
        Flags: ULONG,
        Update: PDEBUG_SYMBOL_GROUP2,
        Symbols: *mut PDEBUG_SYMBOL_GROUP2,
    ) -> HRESULT,
    fn CreateSymbolGroup2(Group: *mut PDEBUG_SYMBOL_GROUP2,) -> HRESULT,
    fn StartSymbolMatchWide(
        Pattern: PCWSTR,
        Handle: PULONG64,
    ) -> HRESULT,
    fn GetNextSymbolMatchWide(
        Handle: ULONG64,
        Buffer: PWSTR,
        BufferSize: ULONG,
        MatchSize: PULONG,
        Offset: PULONG64,
    ) -> HRESULT,
    fn ReloadWide(Module: PCWSTR,) -> HRESULT,
    fn GetSymbolPathWide(
        Buffer: PWSTR,
        BufferSize: ULONG,
        PathSize: PULONG,
    ) -> HRESULT,
    fn SetSymbolPathWide(Path: PCWSTR,) -> HRESULT,
    fn AppendSymbolPathWide(Addition: PCWSTR,) -> HRESULT,
    fn GetImagePathWide(
        Buffer: PWSTR,
        BufferSize: ULONG,
        PathSize: PULONG,
    ) -> HRESULT,
    fn SetImagePathWide(Path: PCWSTR,) -> HRESULT,
    fn AppendImagePathWide(Addition: PCWSTR,) -> HRESULT,
    fn GetSourcePathWide(
        Buffer: PWSTR,
        BufferSize: ULONG,
        PathSize: PULONG,
    ) -> HRESULT,
    fn GetSourcePathElementWide(
        Index: ULONG,
        Buffer: PWSTR,
        BufferSize: ULONG,
        ElementSize: PULONG,
    ) -> HRESULT,
    fn SetSourcePathWide(Path: PCWSTR,) -> HRESULT,
    fn AppendSourcePathWide(Addition: PCWSTR,) -> HRESULT,
    fn FindSourceFileWide(
        StartElement: ULONG,
        File: PCWSTR,
        Flags: ULONG,
        FoundElement: PULONG,
        Buffer: PWSTR,
        BufferSize: ULONG,
        FoundSize: PULONG,
    ) -> HRESULT,
    fn GetSourceFileLineOffsetsWide(
        File: PCWSTR,
        Buffer: PULONG64,
        BufferLines: ULONG,
        FileLines: PULONG,
    ) -> HRESULT,
    fn GetModuleVersionInformationWide(
        Index: ULONG,
        Base: ULONG64,
        Item: PCWSTR,
        Buffer: PVOID,
        BufferSize: ULONG,
        VerInfoSize: PULONG,
    ) -> HRESULT,
    fn GetModuleNameStringWide(
        Which: ULONG,
        Index: ULONG,
        Base: ULONG64,
        Buffer: PWSTR,
        BufferSize: ULONG,
        NameSize: PULONG,
    ) -> HRESULT,
    fn GetConstantNameWide(
        Module: ULONG64,
        TypeId: ULONG,
        Value: ULONG64,
        NameBuffer: PWSTR,
        NameBufferSize: ULONG,
        NameSize: PULONG,
    ) -> HRESULT,
    fn GetFieldNameWide(
        Module: ULONG64,
        TypeId: ULONG,
        FieldIndex: ULONG,
        NameBuffer: PWSTR,
        NameBufferSize: ULONG,
        NameSize: PULONG,
    ) -> HRESULT,
    fn IsManagedModule(
        Index: ULONG,
        Base: ULONG64,
    ) -> HRESULT,
    fn GetModuleByModuleName2(
        Name: PCSTR,
        StartIndex: ULONG,
        Flags: ULONG,
        Index: PULONG,
        Base: PULONG64,
    ) -> HRESULT,
    fn GetModuleByModuleName2Wide(
        Name: PCWSTR,
        StartIndex: ULONG,
        Flags: ULONG,
        Index: PULONG,
        Base: PULONG64,
    ) -> HRESULT,
    fn GetModuleByOffset2(
        Offset: ULONG64,
        StartIndex: ULONG,
        Flags: ULONG,
        Index: PULONG,
        Base: PULONG64,
    ) -> HRESULT,
    fn AddSyntheticModule(
        Base: ULONG64,
        Size: ULONG,
        ImagePath: PCSTR,
        ModuleName: PCSTR,
        Flags: ULONG,
    ) -> HRESULT,
    fn AddSyntheticModuleWide(
        Base: ULONG64,
        Size: ULONG,
        ImagePath: PCWSTR,
        ModuleName: PCWSTR,
        Flags: ULONG,
    ) -> HRESULT,
    fn RemoveSyntheticModule(Base: ULONG64,) -> HRESULT,
    fn GetCurrentScopeFrameIndex(Index: PULONG,) -> HRESULT,
    fn SetScopeFrameByIndex(Index: ULONG,) -> HRESULT,
    fn SetScopeFromJitDebugInfo(
        OutputControl: ULONG,
        InfoOffset: ULONG64,
    ) -> HRESULT,
    fn SetScopeFromStoredEvent() -> HRESULT,
    fn OutputSymbolByOffset(
        OutputControl: ULONG,
        Flags: ULONG,
        Offset: ULONG64,
    ) -> HRESULT,
    fn GetFunctionEntryByOffset(
        Offset: ULONG64,
        Flags: ULONG,
        Buffer: PVOID,
        BufferSize: ULONG,
        BufferNeeded: PULONG,
    ) -> HRESULT,
    fn GetFieldTypeAndOffset(
        Module: ULONG64,
        ContainerTypeId: ULONG,
        Field: PCSTR,
        FieldTypeId: PULONG,
        Offset: PULONG,
    ) -> HRESULT,
    fn GetFieldTypeAndOffsetWide(
        Module: ULONG64,
        ContainerTypeId: ULONG,
        Field: PCWSTR,
        FieldTypeId: PULONG,
        Offset: PULONG,
    ) -> HRESULT,
    fn AddSyntheticSymbol(
        Offset: ULONG64,
        Size: ULONG,
        Name: PCSTR,
        Flags: ULONG,
        Id: PDEBUG_MODULE_AND_ID,
    ) -> HRESULT,
    fn AddSyntheticSymbolWide(
        Offset: ULONG64,
        Size: ULONG,
        Name: PCWSTR,
        Flags: ULONG,
        Id: PDEBUG_MODULE_AND_ID,
    ) -> HRESULT,
    fn RemoveSyntheticSymbol(Id: PDEBUG_MODULE_AND_ID,) -> HRESULT,
    fn GetSymbolEntriesByOffset(
        Offset: ULONG64,
        Flags: ULONG,
        Ids: PDEBUG_MODULE_AND_ID,
        Displacements: PULONG64,
        IdsCount: ULONG,
        Entries: PULONG,
    ) -> HRESULT,
    fn GetSymbolEntriesByName(
        Symbol: PCSTR,
        Flags: ULONG,
        Ids: PDEBUG_MODULE_AND_ID,
        IdsCount: ULONG,
        Entries: PULONG,
    ) -> HRESULT,
    fn GetSymbolEntriesByNameWide(
        Symbol: PCWSTR,
        Flags: ULONG,
        Ids: PDEBUG_MODULE_AND_ID,
        IdsCount: ULONG,
        Entries: PULONG,
    ) -> HRESULT,
    fn GetSymbolEntryByToken(
        ModuleBase: ULONG64,
        Token: ULONG,
        Id: PDEBUG_MODULE_AND_ID,
    ) -> HRESULT,
    fn GetSymbolEntryInformation(
        Id: PDEBUG_MODULE_AND_ID,
        Info: PDEBUG_SYMBOL_ENTRY,
    ) -> HRESULT,
    fn GetSymbolEntryString(
        Id: PDEBUG_MODULE_AND_ID,
        Which: ULONG,
        Buffer: PSTR,
        BufferSize: ULONG,
        StringSize: PULONG,
    ) -> HRESULT,
    fn GetSymbolEntryStringWide(
        Id: PDEBUG_MODULE_AND_ID,
        Which: ULONG,
        Buffer: PWSTR,
        BufferSize: ULONG,
        StringSize: PULONG,
    ) -> HRESULT,
    fn GetSymbolEntryOffsetRegions(
        Id: PDEBUG_MODULE_AND_ID,
        Flags: ULONG,
        Regions: PDEBUG_OFFSET_REGION,
        RegionsCount: ULONG,
        RegionsAvail: PULONG,
    ) -> HRESULT,
    fn GetSymbolEntryBySymbolEntry(
        FromId: PDEBUG_MODULE_AND_ID,
        Flags: ULONG,
        ToId: PDEBUG_MODULE_AND_ID,
    ) -> HRESULT,
    fn GetSourceEntriesByOffset(
        Offset: ULONG64,
        Flags: ULONG,
        Entries: PDEBUG_SYMBOL_SOURCE_ENTRY,
        EntriesCount: ULONG,
        EntriesAvail: PULONG,
    ) -> HRESULT,
    fn GetSourceEntriesByLine(
        Line: ULONG,
        File: PCSTR,
        Flags: ULONG,
        Entries: PDEBUG_SYMBOL_SOURCE_ENTRY,
        EntriesCount: ULONG,
        EntriesAvail: PULONG,
    ) -> HRESULT,
    fn GetSourceEntriesByLineWide(
        Line: ULONG,
        File: PCWSTR,
        Flags: ULONG,
        Entries: PDEBUG_SYMBOL_SOURCE_ENTRY,
        EntriesCount: ULONG,
        EntriesAvail: PULONG,
    ) -> HRESULT,
    fn GetSourceEntryString(
        Entry: PDEBUG_SYMBOL_SOURCE_ENTRY,
        Which: ULONG,
        Buffer: PSTR,
        BufferSize: ULONG,
        StringSize: PULONG,
    ) -> HRESULT,
    fn GetSourceEntryStringWide(
        Entry: PDEBUG_SYMBOL_SOURCE_ENTRY,
        Which: ULONG,
        Buffer: PWSTR,
        BufferSize: ULONG,
        StringSize: PULONG,
    ) -> HRESULT,
    fn GetSourceEntryOffsetRegions(
        Entry: PDEBUG_SYMBOL_SOURCE_ENTRY,
        Flags: ULONG,
        Regions: PDEBUG_OFFSET_REGION,
        RegionsCount: ULONG,
        RegionsAvail: PULONG,
    ) -> HRESULT,
    fn GetSourceEntryBySourceEntry(
        FromEntry: PDEBUG_SYMBOL_SOURCE_ENTRY,
        Flags: ULONG,
        ToEntry: PDEBUG_SYMBOL_SOURCE_ENTRY,
    ) -> HRESULT,
});
RIDL!(#[uuid(0xe391bbd8, 0x9d8c, 0x4418, 0x84, 0x0b, 0xc0, 0x06, 0x59, 0x2a, 0x17, 0x52)]
interface IDebugSymbols4(IDebugSymbols4Vtbl): IDebugSymbols3(IDebugSymbols3Vtbl) {
    fn GetScopeEx(
        InstructionOffset: PULONG64,
        ScopeFrame: PDEBUG_STACK_FRAME_EX,
        ScopeContext: PVOID,
        ScopeContextSize: ULONG,
    ) -> HRESULT,
    fn SetScopeEx(
        InstructionOffset: ULONG64,
        ScopeFrame: PDEBUG_STACK_FRAME_EX,
        ScopeContext: PVOID,
        ScopeContextSize: ULONG,
    ) -> HRESULT,
    fn GetNameByInlineContext(
        Offset: ULONG64,
        InlineContext: ULONG,
        NameBuffer: PSTR,
        NameBufferSize: ULONG,
        NameSize: PULONG,
        Displacement: PULONG64,
    ) -> HRESULT,
    fn GetNameByInlineContextWide(
        Offset: ULONG64,
        InlineContext: ULONG,
        NameBuffer: PWSTR,
        NameBufferSize: ULONG,
        NameSize: PULONG,
        Displacement: PULONG64,
    ) -> HRESULT,
    fn GetLineByInlineContext(
        Offset: ULONG64,
        InlineContext: ULONG,
        Line: PULONG,
        FileBuffer: PSTR,
        FileBufferSize: ULONG,
        FileSize: PULONG,
        Displacement: PULONG64,
    ) -> HRESULT,
    fn GetLineByInlineContextWide(
        Offset: ULONG64,
        InlineContext: ULONG,
        Line: PULONG,
        FileBuffer: PWSTR,
        FileBufferSize: ULONG,
        FileSize: PULONG,
        Displacement: PULONG64,
    ) -> HRESULT,
    fn OutputSymbolByInlineContext(
        OutputControl: ULONG,
        Flags: ULONG,
        Offset: ULONG64,
        InlineContext: ULONG,
    ) -> HRESULT,
});
pub const DEBUG_FRAME_DEFAULT: ULONG = 0;
pub const DEBUG_FRAME_IGNORE_INLINE: ULONG = 0x00000001;
RIDL!(#[uuid(0xc65fa83e, 0x1e69, 0x475e, 0x8e, 0x0e, 0xb5, 0xd7, 0x9e, 0x9c, 0xc1, 0x7e)]
interface IDebugSymbols5(IDebugSymbols5Vtbl): IDebugSymbols4(IDebugSymbols4Vtbl) {
    fn GetCurrentScopeFrameIndexEx(
        Flags: ULONG,
        Index: PULONG,
    ) -> HRESULT,
    fn SetScopeFrameByIndexEx(
        Flags: ULONG,
        Index: ULONG,
    ) -> HRESULT,
});
RIDL!(#[uuid(0x6b86fe2c, 0x2c4f, 0x4f0c, 0x9d, 0xa2, 0x17, 0x43, 0x11, 0xac, 0xc3, 0x27)]
interface IDebugSystemObjects(IDebugSystemObjectsVtbl): IUnknown(IUnknownVtbl) {
    fn GetEventThread(Id: PULONG,) -> HRESULT,
    fn GetEventProcess(Id: PULONG,) -> HRESULT,
    fn GetCurrentThreadId(Id: PULONG,) -> HRESULT,
    fn SetCurrentThreadId(Id: ULONG,) -> HRESULT,
    fn GetCurrentProcessId(Id: PULONG,) -> HRESULT,
    fn SetCurrentProcessId(Id: ULONG,) -> HRESULT,
    fn GetNumberThreads(Number: PULONG,) -> HRESULT,
    fn GetTotalNumberThreads(
        Total: PULONG,
        LargestProcess: PULONG,
    ) -> HRESULT,
    fn GetThreadIdsByIndex(
        Start: ULONG,
        Count: ULONG,
        Ids: PULONG,
        SysIds: PULONG,
    ) -> HRESULT,
    fn GetThreadIdByProcessor(
        Processor: ULONG,
        Id: PULONG,
    ) -> HRESULT,
    fn GetCurrentThreadDataOffset(Offset: PULONG64,) -> HRESULT,
    fn GetThreadIdByDataOffset(
        Offset: ULONG64,
        Id: PULONG,
    ) -> HRESULT,
    fn GetCurrentThreadTeb(Offset: PULONG64,) -> HRESULT,
    fn GetThreadIdByTeb(
        Offset: ULONG64,
        Id: PULONG,
    ) -> HRESULT,
    fn GetCurrentThreadSystemId(SysId: PULONG,) -> HRESULT,
    fn GetThreadIdBySystemId(
        SysId: ULONG,
        Id: PULONG,
    ) -> HRESULT,
    fn GetCurrentThreadHandle(Handle: PULONG64,) -> HRESULT,
    fn GetThreadIdByHandle(
        Handle: ULONG64,
        Id: PULONG,
    ) -> HRESULT,
    fn GetNumberProcesses(Number: PULONG,) -> HRESULT,
    fn GetProcessIdsByIndex(
        Start: ULONG,
        Count: ULONG,
        Ids: PULONG,
        SysIds: PULONG,
    ) -> HRESULT,
    fn GetCurrentProcessDataOffset(Offset: PULONG64,) -> HRESULT,
    fn GetProcessIdByDataOffset(
        Offset: ULONG64,
        Id: PULONG,
    ) -> HRESULT,
    fn GetCurrentProcessPeb(Offset: PULONG64,) -> HRESULT,
    fn GetProcessIdByPeb(
        Offset: ULONG64,
        Id: PULONG,
    ) -> HRESULT,
    fn GetCurrentProcessSystemId(SysId: PULONG,) -> HRESULT,
    fn GetProcessIdBySystemId(
        SysId: ULONG,
        Id: PULONG,
    ) -> HRESULT,
    fn GetCurrentProcessHandle(Handle: PULONG64,) -> HRESULT,
    fn GetProcessIdByHandle(
        Handle: ULONG64,
        Id: PULONG,
    ) -> HRESULT,
    fn GetCurrentProcessExecutableName(
        Buffer: PSTR,
        BufferSize: ULONG,
        ExeSize: PULONG,
    ) -> HRESULT,
});
RIDL!(#[uuid(0x0ae9f5ff, 0x1852, 0x4679, 0xb0, 0x55, 0x49, 0x4b, 0xee, 0x64, 0x07, 0xee)]
interface IDebugSystemObjects2(IDebugSystemObjects2Vtbl): IDebugSystemObjects(IDebugSystemObjectsVtbl) {
    fn GetCurrentProcessUpTime(UpTime: PULONG,) -> HRESULT,
    fn GetImplicitThreadDataOffset(Offset: PULONG64,) -> HRESULT,
    fn SetImplicitThreadDataOffset(Offset: ULONG64,) -> HRESULT,
    fn GetImplicitProcessDataOffset(Offset: PULONG64,) -> HRESULT,
    fn SetImplicitProcessDataOffset(Offset: ULONG64,) -> HRESULT,
});
RIDL!(#[uuid(0xe9676e2f, 0xe286, 0x4ea3, 0xb0, 0xf9, 0xdf, 0xe5, 0xd9, 0xfc, 0x33, 0x0e)]
interface IDebugSystemObjects3(IDebugSystemObjects3Vtbl): IDebugSystemObjects2(IDebugSystemObjects2Vtbl) {
    fn GetEventSystem(Id: PULONG,) -> HRESULT,
    fn GetCurrentSystemId(Id: PULONG,) -> HRESULT,
    fn SetCurrentSystemId(Id: ULONG,) -> HRESULT,
    fn GetNumberSystems(Number: PULONG,) -> HRESULT,
    fn GetSystemIdsByIndex(
        Start: ULONG,
        Count: ULONG,
        Ids: PULONG,
    ) -> HRESULT,
    fn GetTotalNumberThreadsAndProcesses(
        TotalThreads: PULONG,
        TotalProcesses: PULONG,
        LargestProcessThreads: PULONG,
        LargestSystemThreads: PULONG,
        LargestSystemProcesses: PULONG,
    ) -> HRESULT,
    fn GetCurrentSystemServer(Server: PULONG64,) -> HRESULT,
    fn GetSystemByServer(
        Server: ULONG64,
        Id: PULONG,
    ) -> HRESULT,
    fn GetCurrentSystemServerName(
        Buffer: PSTR,
        BufferSize: ULONG,
        NameSize: PULONG,
    ) -> HRESULT,
});
RIDL!(#[uuid(0x489468e6, 0x7d0f, 0x4af5, 0x87, 0xab, 0x25, 0x20, 0x74, 0x54, 0xd5, 0x53)]
interface IDebugSystemObjects4(IDebugSystemObjects4Vtbl): IDebugSystemObjects3(IDebugSystemObjects3Vtbl) {
    fn GetCurrentProcessExecutableNameWide(
        Buffer: PWSTR,
        BufferSize: ULONG,
        ExeSize: PULONG,
    ) -> HRESULT,
    fn GetCurrentSystemServerNameWide(
        Buffer: PWSTR,
        BufferSize: ULONG,
        NameSize: PULONG,
    ) -> HRESULT,
});
pub const DEBUG_COMMAND_EXCEPTION_ID: ULONG = 0xdbe00dbe;
pub const DEBUG_CMDEX_INVALID: ULONG = 0x00000000;
pub const DEBUG_CMDEX_ADD_EVENT_STRING: ULONG = 0x00000001;
pub const DEBUG_CMDEX_RESET_EVENT_STRINGS: ULONG = 0x00000002;
/*
#ifndef DEBUG_NO_IMPLEMENTATION
FORCEINLINE void
DebugCommandException(ULONG Command, ULONG ArgSize, PVOID Arg)
{
    ULONG_PTR ExArgs[4];
    ExArgs[0] = DEBUG_COMMAND_EXCEPTION_ID;
    ExArgs[1] = Command;
    ExArgs[2] = ArgSize;
    ExArgs[3] = (ULONG_PTR)Arg;
    RaiseException(DBG_COMMAND_EXCEPTION, 0, 4, ExArgs);
}
#endif
#define DEBUG_EXTENSION_VERSION(Major, Minor) \
    ((((Major) & 0xffff) << 16) | ((Minor) & 0xffff))
#define DEBUG_EXTINIT_HAS_COMMAND_HELP 0x00000001
typedef HRESULT (CALLBACK* PDEBUG_EXTENSION_INITIALIZE)
    (PULONG Version, PULONG Flags);
typedef void (CALLBACK* PDEBUG_EXTENSION_UNINITIALIZE)
    (void);
typedef HRESULT (CALLBACK* PDEBUG_EXTENSION_CANUNLOAD)
    (void);
typedef void (CALLBACK* PDEBUG_EXTENSION_UNLOAD)
    (void);
#define DEBUG_NOTIFY_SESSION_ACTIVE       0x00000000
#define DEBUG_NOTIFY_SESSION_INACTIVE     0x00000001
#define DEBUG_NOTIFY_SESSION_ACCESSIBLE   0x00000002
#define DEBUG_NOTIFY_SESSION_INACCESSIBLE 0x00000003
typedef void (CALLBACK* PDEBUG_EXTENSION_NOTIFY)
    (ULONG Notify, ULONG64 Argument);
#define DEBUG_EXTENSION_CONTINUE_SEARCH \
    HRESULT_FROM_NT(0xC0000271L)
#define DEBUG_EXTENSION_RELOAD_EXTENSION \
    HRESULT_FROM_NT(0xC00000EEL)
typedef HRESULT (CALLBACK* PDEBUG_EXTENSION_CALL)
    (PDEBUG_CLIENT Client, PCSTR Args);
#define DEBUG_KNOWN_STRUCT_GET_NAMES              1
#define DEBUG_KNOWN_STRUCT_GET_SINGLE_LINE_OUTPUT 2
#define DEBUG_KNOWN_STRUCT_SUPPRESS_TYPE_NAME     3
typedef HRESULT (CALLBACK* PDEBUG_EXTENSION_KNOWN_STRUCT)
    (ULONG Flags,
     ULONG64 Offset,
     PSTR TypeName,
     PSTR Buffer,
     PULONG BufferChars);
typedef HRESULT (CALLBACK* PDEBUG_EXTENSION_KNOWN_STRUCT_EX)
    (PDEBUG_CLIENT Client,
     ULONG Flags,
     ULONG64 Offset,
     PCSTR TypeName,
     PSTR Buffer,
     PULONG BufferChars);
typedef PDEBUG_EXTENSION_KNOWN_STRUCT PDEBUG_ENTENSION_KNOWNSTRUCT;
#define DEBUG_EXT_QVALUE_DEFAULT 0x00000000
typedef HRESULT (CALLBACK* PDEBUG_EXTENSION_QUERY_VALUE_NAMES)
    (PDEBUG_CLIENT Client,
     ULONG Flags,
     PWSTR Buffer,
     ULONG BufferChars,
     PULONG BufferNeeded);
#define DEBUG_EXT_PVALUE_DEFAULT 0x00000000
#define DEBUG_EXT_PVTYPE_IS_VALUE   0x00000000
#define DEBUG_EXT_PVTYPE_IS_POINTER 0x00000001
typedef HRESULT (CALLBACK* PDEBUG_EXTENSION_PROVIDE_VALUE)
    (PDEBUG_CLIENT Client,
     ULONG Flags,
     PCWSTR Name,
     PULONG64 Value,
     PULONG64 TypeModBase,
     PULONG TypeId,
     PULONG TypeFlags);
#ifdef __cplusplus
};
typedef HRESULT (CALLBACK* PDEBUG_STACK_PROVIDER_BEGINTHREADSTACKRECONSTRUCTION)
    (
    ULONG StreamType,
    PVOID MiniDumpStreamBuffer,
    ULONG BufferSize);
typedef HRESULT (CALLBACK* PDEBUG_STACK_PROVIDER_RECONSTRUCTSTACK)
    (
    ULONG SystemThreadId,
    PDEBUG_STACK_FRAME_EX NativeFrames,
    ULONG CountNativeFrames,
    PSTACK_SYM_FRAME_INFO *StackSymFrames,
    PULONG StackSymFramesFilled);
typedef HRESULT (CALLBACK* PDEBUG_STACK_PROVIDER_FREESTACKSYMFRAMES)
    (
    PSTACK_SYM_FRAME_INFO StackSymFrames);
typedef HRESULT (CALLBACK* PDEBUG_STACK_PROVIDER_ENDTHREADSTACKRECONSTRUCTION)
    (void);
#if !defined(DEBUG_NO_IMPLEMENTATION) && !defined(_M_CEE_PURE)
class DebugBaseEventCallbacks : public IDebugEventCallbacks
{
public:
    fn QueryInterface(
        REFIID InterfaceId,
        PVOID* Interface,
        )
    {
        *Interface = NULL;
#if _MSC_VER >= 1100
        if (IsEqualIID(InterfaceId, __uuidof(IUnknown)) ||
            IsEqualIID(InterfaceId, __uuidof(IDebugEventCallbacks)))
#else
        if (IsEqualIID(InterfaceId, IID_IUnknown) ||
            IsEqualIID(InterfaceId, IID_IDebugEventCallbacks))
#endif
        {
            *Interface = (IDebugEventCallbacks *)this;
            AddRef();
            return S_OK;
        }
        else
        {
            return E_NOINTERFACE;
        }
    }
    fn Breakpoint(
        PDEBUG_BREAKPOINT Bp
        )
    {
        UNREFERENCED_PARAMETER(Bp);
        return DEBUG_STATUS_NO_CHANGE;
    }
    fn Exception(
        PEXCEPTION_RECORD64 Exception,
        ULONG FirstChance
        )
    {
        UNREFERENCED_PARAMETER(Exception);
        UNREFERENCED_PARAMETER(FirstChance);
        return DEBUG_STATUS_NO_CHANGE;
    }
    fn CreateThread(
        ULONG64 Handle,
        ULONG64 DataOffset,
        ULONG64 StartOffset
        )
    {
        UNREFERENCED_PARAMETER(Handle);
        UNREFERENCED_PARAMETER(DataOffset);
        UNREFERENCED_PARAMETER(StartOffset);
        return DEBUG_STATUS_NO_CHANGE;
    }
    fn ExitThread(
        ULONG ExitCode
        )
    {
        UNREFERENCED_PARAMETER(ExitCode);
        return DEBUG_STATUS_NO_CHANGE;
    }
    fn CreateProcess(
        ULONG64 ImageFileHandle,
        ULONG64 Handle,
        ULONG64 BaseOffset,
        ULONG ModuleSize,
        PCSTR ModuleName,
        PCSTR ImageName,
        ULONG CheckSum,
        ULONG TimeDateStamp,
        ULONG64 InitialThreadHandle,
        ULONG64 ThreadDataOffset,
        ULONG64 StartOffset
        )
    {
        UNREFERENCED_PARAMETER(ImageFileHandle);
        UNREFERENCED_PARAMETER(Handle);
        UNREFERENCED_PARAMETER(BaseOffset);
        UNREFERENCED_PARAMETER(ModuleSize);
        UNREFERENCED_PARAMETER(ModuleName);
        UNREFERENCED_PARAMETER(ImageName);
        UNREFERENCED_PARAMETER(CheckSum);
        UNREFERENCED_PARAMETER(TimeDateStamp);
        UNREFERENCED_PARAMETER(InitialThreadHandle);
        UNREFERENCED_PARAMETER(ThreadDataOffset);
        UNREFERENCED_PARAMETER(StartOffset);
        return DEBUG_STATUS_NO_CHANGE;
    }
    fn ExitProcess(
        ULONG ExitCode
        )
    {
        UNREFERENCED_PARAMETER(ExitCode);
        return DEBUG_STATUS_NO_CHANGE;
    }
    fn LoadModule(
        ULONG64 ImageFileHandle,
        ULONG64 BaseOffset,
        ULONG ModuleSize,
        PCSTR ModuleName,
        PCSTR ImageName,
        ULONG CheckSum,
        ULONG TimeDateStamp
        )
    {
        UNREFERENCED_PARAMETER(ImageFileHandle);
        UNREFERENCED_PARAMETER(BaseOffset);
        UNREFERENCED_PARAMETER(ModuleSize);
        UNREFERENCED_PARAMETER(ModuleName);
        UNREFERENCED_PARAMETER(ImageName);
        UNREFERENCED_PARAMETER(CheckSum);
        UNREFERENCED_PARAMETER(TimeDateStamp);
        return DEBUG_STATUS_NO_CHANGE;
    }
    fn UnloadModule(
        PCSTR ImageBaseName,
        ULONG64 BaseOffset
        )
    {
        UNREFERENCED_PARAMETER(ImageBaseName);
        UNREFERENCED_PARAMETER(BaseOffset);
        return DEBUG_STATUS_NO_CHANGE;
    }
    fn SystemError(
        ULONG Error,
        ULONG Level
        )
    {
        UNREFERENCED_PARAMETER(Error);
        UNREFERENCED_PARAMETER(Level);
        return DEBUG_STATUS_NO_CHANGE;
    }
    fn SessionStatus(
        ULONG Status
        )
    {
        UNREFERENCED_PARAMETER(Status);
        return DEBUG_STATUS_NO_CHANGE;
    }
    fn ChangeDebuggeeState(
        ULONG Flags,
        ULONG64 Argument
        )
    {
        UNREFERENCED_PARAMETER(Flags);
        UNREFERENCED_PARAMETER(Argument);
        return S_OK;
    }
    fn ChangeEngineState(
        ULONG Flags,
        ULONG64 Argument
        )
    {
        UNREFERENCED_PARAMETER(Flags);
        UNREFERENCED_PARAMETER(Argument);
        return S_OK;
    }
    fn ChangeSymbolState(
        ULONG Flags,
        ULONG64 Argument
        )
    {
        UNREFERENCED_PARAMETER(Flags);
        UNREFERENCED_PARAMETER(Argument);
        return S_OK;
    }
};
class DebugBaseEventCallbacksWide : public IDebugEventCallbacksWide
{
public:
    fn QueryInterface(
        REFIID InterfaceId,
        PVOID* Interface
        )
    {
        *Interface = NULL;
#if _MSC_VER >= 1100
        if (IsEqualIID(InterfaceId, __uuidof(IUnknown)) ||
            IsEqualIID(InterfaceId, __uuidof(IDebugEventCallbacksWide)))
#else
        if (IsEqualIID(InterfaceId, IID_IUnknown) ||
            IsEqualIID(InterfaceId, IID_IDebugEventCallbacksWide))
#endif
        {
            *Interface = (IDebugEventCallbacksWide *)this;
            AddRef();
            return S_OK;
        }
        else
        {
            return E_NOINTERFACE;
        }
    }
    fn Breakpoint(
        PDEBUG_BREAKPOINT2 Bp
        )
    {
        UNREFERENCED_PARAMETER(Bp);
        return DEBUG_STATUS_NO_CHANGE;
    }
    fn Exception(
        PEXCEPTION_RECORD64 Exception,
        ULONG FirstChance
        )
    {
        UNREFERENCED_PARAMETER(Exception);
        UNREFERENCED_PARAMETER(FirstChance);
        return DEBUG_STATUS_NO_CHANGE;
    }
    fn CreateThread(
        ULONG64 Handle,
        ULONG64 DataOffset,
        ULONG64 StartOffset
        )
    {
        UNREFERENCED_PARAMETER(Handle);
        UNREFERENCED_PARAMETER(DataOffset);
        UNREFERENCED_PARAMETER(StartOffset);
        return DEBUG_STATUS_NO_CHANGE;
    }
    fn ExitThread(
        ULONG ExitCode
        )
    {
        UNREFERENCED_PARAMETER(ExitCode);
        return DEBUG_STATUS_NO_CHANGE;
    }
    fn CreateProcess(
        ULONG64 ImageFileHandle,
        ULONG64 Handle,
        ULONG64 BaseOffset,
        ULONG ModuleSize,
        PCWSTR ModuleName,
        PCWSTR ImageName,
        ULONG CheckSum,
        ULONG TimeDateStamp,
        ULONG64 InitialThreadHandle,
        ULONG64 ThreadDataOffset,
        ULONG64 StartOffset
        )
    {
        UNREFERENCED_PARAMETER(ImageFileHandle);
        UNREFERENCED_PARAMETER(Handle);
        UNREFERENCED_PARAMETER(BaseOffset);
        UNREFERENCED_PARAMETER(ModuleSize);
        UNREFERENCED_PARAMETER(ModuleName);
        UNREFERENCED_PARAMETER(ImageName);
        UNREFERENCED_PARAMETER(CheckSum);
        UNREFERENCED_PARAMETER(TimeDateStamp);
        UNREFERENCED_PARAMETER(InitialThreadHandle);
        UNREFERENCED_PARAMETER(ThreadDataOffset);
        UNREFERENCED_PARAMETER(StartOffset);
        return DEBUG_STATUS_NO_CHANGE;
    }
    fn ExitProcess(
        ULONG ExitCode
        )
    {
        UNREFERENCED_PARAMETER(ExitCode);
        return DEBUG_STATUS_NO_CHANGE;
    }
    fn LoadModule(
        ULONG64 ImageFileHandle,
        ULONG64 BaseOffset,
        ULONG ModuleSize,
        PCWSTR ModuleName,
        PCWSTR ImageName,
        ULONG CheckSum,
        ULONG TimeDateStamp
        )
    {
        UNREFERENCED_PARAMETER(ImageFileHandle);
        UNREFERENCED_PARAMETER(BaseOffset);
        UNREFERENCED_PARAMETER(ModuleSize);
        UNREFERENCED_PARAMETER(ModuleName);
        UNREFERENCED_PARAMETER(ImageName);
        UNREFERENCED_PARAMETER(CheckSum);
        UNREFERENCED_PARAMETER(TimeDateStamp);
        return DEBUG_STATUS_NO_CHANGE;
    }
    fn UnloadModule(
        PCWSTR ImageBaseName,
        ULONG64 BaseOffset
        )
    {
        UNREFERENCED_PARAMETER(ImageBaseName);
        UNREFERENCED_PARAMETER(BaseOffset);
        return DEBUG_STATUS_NO_CHANGE;
    }
    fn SystemError(
        ULONG Error,
        ULONG Level
        )
    {
        UNREFERENCED_PARAMETER(Error);
        UNREFERENCED_PARAMETER(Level);
        return DEBUG_STATUS_NO_CHANGE;
    }
    fn SessionStatus(
        ULONG Status
        )
    {
        UNREFERENCED_PARAMETER(Status);
        return DEBUG_STATUS_NO_CHANGE;
    }
    fn ChangeDebuggeeState(
        ULONG Flags,
        ULONG64 Argument
        )
    {
        UNREFERENCED_PARAMETER(Flags);
        UNREFERENCED_PARAMETER(Argument);
        return S_OK;
    }
    fn ChangeEngineState(
        ULONG Flags,
        ULONG64 Argument
        )
    {
        UNREFERENCED_PARAMETER(Flags);
        UNREFERENCED_PARAMETER(Argument);
        return S_OK;
    }
    fn ChangeSymbolState(
        ULONG Flags,
        ULONG64 Argument
        )
    {
        UNREFERENCED_PARAMETER(Flags);
        UNREFERENCED_PARAMETER(Argument);
        return S_OK;
    }
};
#endif
#endif
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_WER) */
#endif
*/

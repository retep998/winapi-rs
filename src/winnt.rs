// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! This module defines the 32-Bit Windows types and constants that are defined by NT, but exposed
//! through the Win32 API.
//341
pub type PVOID = *mut ::c_void;
pub type PVOID64 = u64; // This is a 64-bit pointer, even when in 32-bit
//382
pub type VOID = ::c_void;
pub type CHAR = ::c_char;
pub type SHORT = ::c_short;
pub type LONG = ::c_long;
// pub type INT = ::c_int; // Already defined by minwindef.h
pub type WCHAR = ::wchar_t;
pub type PWCHAR = *mut WCHAR;
pub type LPWCH = *mut WCHAR;
pub type PWCH = *mut WCHAR;
pub type LPCWCH = *const WCHAR;
pub type PCWCH = *const WCHAR;
pub type NWPSTR = *mut WCHAR;
pub type LPWSTR = *mut WCHAR;
pub type PWSTR = *mut WCHAR;
pub type PZPWSTR = *mut PWSTR;
pub type PCZPWSTR = *const PWSTR;
pub type LPUWSTR = *mut WCHAR;
pub type PUWSTR = *mut WCHAR;
pub type LPCWSTR = *const WCHAR;
pub type PCWSTR = *const WCHAR;
pub type PZPCWSTR= *mut PCWSTR;
pub type PCZPCWSTR = *const PCWSTR;
pub type LPCUWSTR = *const WCHAR;
pub type PCUWSTR = *const WCHAR;
pub type PZZWSTR= *mut WCHAR;
pub type PCZZWSTR = *const WCHAR;
pub type PUZZWSTR = *mut WCHAR;
pub type PCUZZWSTR = *const WCHAR;
pub type PNZWCH = *mut WCHAR;
pub type PCNZWCH = *const WCHAR;
pub type PUNZWCH = *mut WCHAR;
pub type PCUNZWCH = *const WCHAR;
pub type LPCWCHAR = *const WCHAR;
pub type PCWCHAR = *const WCHAR;
pub type LPCUWCHAR = *const WCHAR;
pub type PCUWCHAR = *const WCHAR;
pub type UCSCHAR = ::c_ulong;
pub type PUCSCHAR = *mut UCSCHAR;
pub type PCUCSCHAR = *const UCSCHAR;
pub type PUCSSTR = *mut UCSCHAR;
pub type PUUCSSTR = *mut UCSCHAR;
pub type PCUCSSTR = *const UCSCHAR;
pub type PCUUCSSTR = *const UCSCHAR;
pub type PUUCSCHAR = *mut UCSCHAR;
pub type PCUUCSCHAR = *const UCSCHAR;
pub type PCHAR = *mut CHAR;
pub type LPCH = *mut CHAR;
pub type PCH = *mut CHAR;
pub type LPCCH = *const CHAR;
pub type PCCH = *const CHAR;
pub type NPSTR = *mut CHAR;
pub type LPSTR = *mut CHAR;
pub type PSTR = *mut CHAR;
pub type PZPSTR = *mut PSTR;
pub type PCZPSTR = *const PSTR;
pub type LPCSTR = *const CHAR;
pub type PCSTR = *const CHAR;
pub type PZPCSTR = *mut PCSTR;
pub type PCZPCSTR = *const PCSTR;
pub type PZZSTR = *mut CHAR;
pub type PCZZSTR = *const CHAR;
pub type PNZCH = *mut CHAR;
pub type PCNZCH = *const CHAR;
// Skipping TCHAR things
pub type PSHORT = *mut SHORT;
pub type PLONG = *mut LONG;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct PROCESSOR_NUMBER {
    pub Group: ::WORD,
    pub Number: ::BYTE,
    pub Reserved: ::BYTE,
}
pub type PPROCESSOR_NUMBER = *mut PROCESSOR_NUMBER;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct GROUP_AFFINITY {
    pub Mask: ::KAFFINITY,
    pub Group: ::WORD,
    pub Reserved: [::WORD; 3],
}
pub type PGROUP_AFFINITY = *mut GROUP_AFFINITY;
pub type HANDLE = *mut ::c_void;
pub type PHANDLE = *mut HANDLE;
pub type FCHAR = ::BYTE;
pub type FSHORT = ::WORD;
pub type FLONG = ::DWORD;
//667
pub type CCHAR = ::c_char;
pub type LCID = ::DWORD;
pub type PLCID = ::PDWORD;
pub type LANGID = ::WORD;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub enum COMPARTMENT_ID {
    UNSPECIFIED_COMPARTMENT_ID = 0,
    DEFAULT_COMPARTMENT_ID = 1,
}
pub type PCOMPARTMENT_ID = *mut COMPARTMENT_ID;
//710
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct FLOAT128 {
    pub LowPart: ::__int64,
    pub HighPart: ::__int64,
}
pub type PFLOAT128 = *mut FLOAT128;
pub type LONGLONG = ::__int64;
pub type ULONGLONG = ::__uint64;
pub type PLONGLONG = *mut LONGLONG;
pub type PULONGLONG = *mut ULONGLONG;
pub type USN = LONGLONG;
pub type LARGE_INTEGER = LONGLONG;
pub type PLARGE_INTEGER = *mut LARGE_INTEGER;
pub type ULARGE_INTEGER = ULONGLONG;
pub type PULARGE_INTEGER= *mut ULARGE_INTEGER;
pub type RTL_REFERENCE_COUNT = ::LONG_PTR;
pub type PRTL_REFERENCE_COUNT = *mut ::LONG_PTR;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct LUID {
    pub LowPart: ::DWORD,
    pub HighPart: LONG,
}
pub type PLUID = *mut LUID;
pub type DWORDLONG = ULONGLONG;
pub type PDWORDLONG = *mut DWORDLONG;
//1042
pub type BOOLEAN = ::BYTE;
pub type PBOOLEAN = *mut BOOLEAN;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct LIST_ENTRY {
    pub Flink: *mut LIST_ENTRY,
    pub Blink: *mut LIST_ENTRY,
}
pub type PLIST_ENTRY = *mut LIST_ENTRY;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SINGLE_LIST_ENTRY {
    pub Next: *mut SINGLE_LIST_ENTRY,
}
pub type PSINGLE_LIST_ENTRY = *mut SINGLE_LIST_ENTRY;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct LIST_ENTRY32 {
    pub Flink: ::DWORD,
    pub Blink: ::DWORD,
}
pub type PLIST_ENTRY32 = *mut LIST_ENTRY32;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct LIST_ENTRY64 {
    pub Flink: ULONGLONG,
    pub Blink: ULONGLONG,
}
pub type PLIST_ENTRY64 = *mut LIST_ENTRY64;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct OBJECTID {
    pub Lineage: ::GUID,
    pub Uniquifier: ::DWORD,
}
pub const MINCHAR: ::CHAR = 0x80u8 as ::CHAR;
pub const MAXCHAR: ::CHAR = 0x7f;
//1300
pub type PEXCEPTION_ROUTINE = Option<unsafe extern "system" fn(
    ExceptionRecord: *mut EXCEPTION_RECORD, EstablisherFrame: PVOID, ContextRecord: *mut CONTEXT,
    DispatcherContext: PVOID,
) -> ::EXCEPTION_DISPOSITION>;
//2273
pub const MAXIMUM_WAIT_OBJECTS: ::DWORD = 64;
pub const MAXIMUM_SUSPEND_COUNT: ::CHAR = MAXCHAR;
//2277
pub type KSPIN_LOCK = ::ULONG_PTR;
pub type PKSPIN_LOCK = *mut KSPIN_LOCK;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct M128A { // FIXME align 16
    pub Low: ULONGLONG,
    pub High: LONGLONG,
}
pub type PM128A = *mut M128A;
#[cfg(target_arch = "x86")] #[repr(C)] #[derive(Copy)]
pub struct XSAVE_FORMAT { // FIXME align 16
    pub ControlWord: ::WORD,
    pub StatusWord: ::WORD,
    pub TagWord: ::BYTE,
    pub Reserved1: ::BYTE,
    pub ErrorOpcode: ::WORD,
    pub ErrorOffset: ::DWORD,
    pub ErrorSelector: ::WORD,
    pub Reserved2: ::WORD,
    pub DataOffset: ::DWORD,
    pub DataSelector: ::WORD,
    pub Reserved3: ::WORD,
    pub MxCsr: ::DWORD,
    pub MxCsr_Mask: ::DWORD,
    pub FloatRegisters: [M128A; 8],
    pub XmmRegisters: [M128A; 8],
    pub Reserved4: [::BYTE; 224],
}
#[cfg(target_arch = "x86_64")] #[repr(C)] #[derive(Copy)]
pub struct XSAVE_FORMAT { // FIXME align 16
    pub ControlWord: ::WORD,
    pub StatusWord: ::WORD,
    pub TagWord: ::BYTE,
    pub Reserved1: ::BYTE,
    pub ErrorOpcode: ::WORD,
    pub ErrorOffset: ::DWORD,
    pub ErrorSelector: ::WORD,
    pub Reserved2: ::WORD,
    pub DataOffset: ::DWORD,
    pub DataSelector: ::WORD,
    pub Reserved3: ::WORD,
    pub MxCsr: ::DWORD,
    pub MxCsr_Mask: ::DWORD,
    pub FloatRegisters: [M128A; 8],
    pub XmmRegisters: [M128A; 16],
    pub Reserved4: [::BYTE; 96],
}
impl Clone for XSAVE_FORMAT { fn clone(&self) -> XSAVE_FORMAT { *self } }
//3563
#[cfg(target_arch = "x86")]
pub const SIZE_OF_80387_REGISTERS: usize = 80;
#[cfg(target_arch = "x86")] #[repr(C)] #[derive(Copy)]
pub struct FLOATING_SAVE_AREA {
    pub ControlWord: ::DWORD,
    pub StatusWord: ::DWORD,
    pub TagWord: ::DWORD,
    pub ErrorOffset: ::DWORD,
    pub ErrorSelector: ::DWORD,
    pub DataOffset: ::DWORD,
    pub DataSelector: ::DWORD,
    pub RegisterArea: [::BYTE; SIZE_OF_80387_REGISTERS],
    pub Spare0: ::DWORD,
}
#[cfg(target_arch = "x86")]
impl Clone for FLOATING_SAVE_AREA { fn clone(&self) -> FLOATING_SAVE_AREA { *self } }
#[cfg(target_arch = "x86")]
pub type PFLOATING_SAVE_AREA = *mut FLOATING_SAVE_AREA;
#[cfg(target_arch = "x86")]
pub const MAXIMUM_SUPPORTED_EXTENSION: usize = 512;
#[cfg(target_arch = "x86")] #[repr(C)] #[derive(Copy)]
pub struct CONTEXT {
    pub ContextFlags: ::DWORD,
    pub Dr0: ::DWORD,
    pub Dr1: ::DWORD,
    pub Dr2: ::DWORD,
    pub Dr3: ::DWORD,
    pub Dr6: ::DWORD,
    pub Dr7: ::DWORD,
    pub FloatSave: FLOATING_SAVE_AREA,
    pub SegGs: ::DWORD,
    pub SegFs: ::DWORD,
    pub SegEs: ::DWORD,
    pub SegDs: ::DWORD,
    pub Edi: ::DWORD,
    pub Esi: ::DWORD,
    pub Ebx: ::DWORD,
    pub Edx: ::DWORD,
    pub Ecx: ::DWORD,
    pub Eax: ::DWORD,
    pub Ebp: ::DWORD,
    pub Eip: ::DWORD,
    pub SegCs: ::DWORD,
    pub EFlags: ::DWORD,
    pub Esp: ::DWORD,
    pub SegSs: ::DWORD,
    pub ExtendedRegisters: [::BYTE; MAXIMUM_SUPPORTED_EXTENSION],
}
#[cfg(target_arch = "x86")]
impl Clone for CONTEXT { fn clone(&self) -> CONTEXT { *self } }
#[cfg(target_arch = "x86_64")]
pub type XMM_SAVE_AREA32 = XSAVE_FORMAT;
pub type PXMM_SAVE_AREA32 = *mut XSAVE_FORMAT;
// FIXME - Align 16
#[cfg(target_arch = "x86_64")] #[repr(C)] #[derive(Clone, Copy)]
pub struct CONTEXT {
    pub P1Home: ::DWORD64,
    pub P2Home: ::DWORD64,
    pub P3Home: ::DWORD64,
    pub P4Home: ::DWORD64,
    pub P5Home: ::DWORD64,
    pub P6Home: ::DWORD64,
    pub ContextFlags: ::DWORD,
    pub MxCsr: ::DWORD,
    pub SegCs: ::WORD,
    pub SegDs: ::WORD,
    pub SegEs: ::WORD,
    pub SegFs: ::WORD,
    pub SegGs: ::WORD,
    pub SegSs: ::WORD,
    pub EFlags: ::DWORD,
    pub Dr0: ::DWORD64,
    pub Dr1: ::DWORD64,
    pub Dr2: ::DWORD64,
    pub Dr3: ::DWORD64,
    pub Dr6: ::DWORD64,
    pub Dr7: ::DWORD64,
    pub Rax: ::DWORD64,
    pub Rcx: ::DWORD64,
    pub Rdx: ::DWORD64,
    pub Rbx: ::DWORD64,
    pub Rsp: ::DWORD64,
    pub Rbp: ::DWORD64,
    pub Rsi: ::DWORD64,
    pub Rdi: ::DWORD64,
    pub R8: ::DWORD64,
    pub R9: ::DWORD64,
    pub R10: ::DWORD64,
    pub R11: ::DWORD64,
    pub R12: ::DWORD64,
    pub R13: ::DWORD64,
    pub R14: ::DWORD64,
    pub R15: ::DWORD64,
    pub Rip: ::DWORD64,
    pub FltSave: XMM_SAVE_AREA32,
    pub VectorRegister: [::M128A; 26],
    pub VectorControl: ::DWORD64,
    pub DebugControl: ::DWORD64,
    pub LastBranchToRip: ::DWORD64,
    pub LastBranchFromRip: ::DWORD64,
    pub LastExceptionToRip: ::DWORD64,
    pub LastExceptionFromRip: ::DWORD64,
}
pub type PCONTEXT = *mut CONTEXT;
#[test]
fn test_CONTEXT_size() {
    use std::mem::size_of;
    if cfg!(target_arch = "x86_64") {
        assert_eq!(size_of::<CONTEXT>(), 1232)
    } else if cfg!(target_arch = "x86") {
        assert_eq!(size_of::<CONTEXT>(), 716)
    }
}
//8983
pub const EXCEPTION_MAXIMUM_PARAMETERS: usize = 15;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct EXCEPTION_RECORD {
    pub ExceptionCode: ::DWORD,
    pub ExceptionFlags: ::DWORD,
    pub ExceptionRecord: *mut EXCEPTION_RECORD,
    pub ExceptionAddress: ::PVOID,
    pub NumberParameters: ::DWORD,
    pub ExceptionInformation: [::ULONG_PTR; EXCEPTION_MAXIMUM_PARAMETERS],
}
pub type PEXCEPTION_RECORD = *mut EXCEPTION_RECORD;
//9023
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct EXCEPTION_POINTERS {
    pub ExceptionRecord: PEXCEPTION_RECORD,
    pub ContextRecord: PCONTEXT,
}
pub type PEXCEPTION_POINTERS = *mut EXCEPTION_POINTERS;
pub type PACCESS_TOKEN = ::PVOID;
pub type PSECURITY_DESCRIPTOR = ::PVOID;
pub type PSID = ::PVOID;
pub type PCLAIMS_BLOB = ::PVOID;
//9091
pub type ACCESS_MASK = ::DWORD;
pub type PACCESS_MASK = *mut ACCESS_MASK;
pub const DELETE: ::DWORD = 0x00010000;
pub const READ_CONTROL: ::DWORD = 0x00020000;
pub const WRITE_DAC: ::DWORD = 0x00040000;
pub const WRITE_OWNER: ::DWORD = 0x00080000;
pub const SYNCHRONIZE: ::DWORD = 0x00100000;
pub const STANDARD_RIGHTS_REQUIRED: ::DWORD = 0x000F0000;
pub const STANDARD_RIGHTS_READ: ::DWORD = READ_CONTROL;
pub const STANDARD_RIGHTS_WRITE: ::DWORD = READ_CONTROL;
pub const STANDARD_RIGHTS_EXECUTE: ::DWORD = READ_CONTROL;
pub const STANDARD_RIGHTS_ALL: ::DWORD = 0x001F0000;
pub const SPECIFIC_RIGHTS_ALL: ::DWORD = 0x0000FFFF;
pub const ACCESS_SYSTEM_SECURITY: ::DWORD = 0x01000000;
pub const MAXIMUM_ALLOWED: ::DWORD = 0x02000000;
pub const GENERIC_READ: ::DWORD = 0x80000000;
pub const GENERIC_WRITE: ::DWORD = 0x40000000;
pub const GENERIC_EXECUTE: ::DWORD = 0x20000000;
pub const GENERIC_ALL: ::DWORD = 0x10000000;
//9170
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct LUID_AND_ATTRIBUTES {
    pub Luid: LUID,
    pub Attributes: ::DWORD,
}
pub type PLUID_AND_ATTRIBUTES = *mut LUID_AND_ATTRIBUTES;
//9802
pub const ACL_REVISION: ::BYTE = 2;
pub const ACL_REVISION_DS: ::BYTE = 4;
pub const ACL_REVISION1: ::BYTE = 1;
pub const MIN_ACL_REVISION: ::BYTE = ACL_REVISION2;
pub const ACL_REVISION2: ::BYTE = 2;
pub const ACL_REVISION3: ::BYTE = 3;
pub const ACL_REVISION4: ::BYTE = 4;
pub const MAX_ACL_REVISION: ::BYTE = ACL_REVISION4;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ACL {
    pub AclRevision: ::BYTE,
    pub Sbz1: ::BYTE,
    pub AclSize: ::WORD,
    pub AceCount: ::WORD,
    pub Sbz2: ::WORD,
}
pub type PACL = *mut ACL;
//10689
pub const TOKEN_ASSIGN_PRIMARY: ::DWORD = 0x0001;
pub const TOKEN_DUPLICATE: ::DWORD = 0x0002;
pub const TOKEN_IMPERSONATE: ::DWORD = 0x0004;
pub const TOKEN_QUERY: ::DWORD = 0x0008;
pub const TOKEN_QUERY_SOURCE: ::DWORD = 0x0010;
pub const TOKEN_ADJUST_PRIVILEGES: ::DWORD = 0x0020;
pub const TOKEN_ADJUST_GROUPS: ::DWORD = 0x0040;
pub const TOKEN_ADJUST_DEFAULT: ::DWORD = 0x0080;
pub const TOKEN_ADJUST_SESSIONID: ::DWORD = 0x0100;
pub const TOKEN_ALL_ACCESS_P: ::DWORD = STANDARD_RIGHTS_REQUIRED | TOKEN_ASSIGN_PRIMARY
    | TOKEN_DUPLICATE | TOKEN_IMPERSONATE | TOKEN_QUERY | TOKEN_QUERY_SOURCE
    | TOKEN_ADJUST_PRIVILEGES | TOKEN_ADJUST_GROUPS | TOKEN_ADJUST_DEFAULT;
pub const TOKEN_ALL_ACCESS: ::DWORD = TOKEN_ALL_ACCESS_P | TOKEN_ADJUST_SESSIONID;
pub const TOKEN_READ: ::DWORD = STANDARD_RIGHTS_READ | TOKEN_QUERY;
pub const TOKEN_WRITE: ::DWORD = STANDARD_RIGHTS_WRITE | TOKEN_ADJUST_PRIVILEGES
    | TOKEN_ADJUST_GROUPS | TOKEN_ADJUST_DEFAULT;
pub const TOKEN_EXECUTE: ::DWORD = STANDARD_RIGHTS_EXECUTE;
//10823
#[repr(C)] #[derive(Debug)] #[allow(missing_copy_implementations)]
pub struct TOKEN_PRIVILEGES {
    pub PrivilegeCount: ::DWORD,
    pub Privileges: [LUID_AND_ATTRIBUTES; 0],
}
pub type PTOKEN_PRIVILEGES = *mut TOKEN_PRIVILEGES;
//10965
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_INVALID: ::WORD = 0x00;
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_INT64: ::WORD = 0x01;
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_UINT64: ::WORD = 0x02;
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_STRING: ::WORD = 0x03;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct CLAIM_SECURITY_ATTRIBUTE_FQBN_VALUE {
    pub Version: ::DWORD64,
    pub Name: ::PWSTR,
}
pub type PCLAIM_SECURITY_ATTRIBUTE_FQBN_VALUE = *mut CLAIM_SECURITY_ATTRIBUTE_FQBN_VALUE;
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_FQBN: ::WORD = 0x04;
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_SID: ::WORD = 0x05;
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_BOOLEAN: ::WORD = 0x06;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct CLAIM_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE {
    pub pValue: ::PVOID,
    pub ValueLength: ::DWORD,
}
pub type PCLAIM_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE =
    *mut CLAIM_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE;
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_OCTET_STRING: ::WORD = 0x10;
pub const CLAIM_SECURITY_ATTRIBUTE_NON_INHERITABLE: ::DWORD = 0x0001;
pub const CLAIM_SECURITY_ATTRIBUTE_VALUE_CASE_SENSITIVE: ::DWORD = 0x0002;
pub const CLAIM_SECURITY_ATTRIBUTE_USE_FOR_DENY_ONLY: ::DWORD = 0x0004;
pub const CLAIM_SECURITY_ATTRIBUTE_DISABLED_BY_DEFAULT: ::DWORD = 0x0008;
pub const CLAIM_SECURITY_ATTRIBUTE_DISABLED: ::DWORD = 0x0010;
pub const CLAIM_SECURITY_ATTRIBUTE_MANDATORY: ::DWORD = 0x0020;
pub const CLAIM_SECURITY_ATTRIBUTE_VALID_FLAGS: ::DWORD = CLAIM_SECURITY_ATTRIBUTE_NON_INHERITABLE
    | CLAIM_SECURITY_ATTRIBUTE_VALUE_CASE_SENSITIVE | CLAIM_SECURITY_ATTRIBUTE_USE_FOR_DENY_ONLY
    | CLAIM_SECURITY_ATTRIBUTE_DISABLED_BY_DEFAULT | CLAIM_SECURITY_ATTRIBUTE_DISABLED
    | CLAIM_SECURITY_ATTRIBUTE_MANDATORY;
pub const CLAIM_SECURITY_ATTRIBUTE_CUSTOM_FLAGS: ::DWORD = 0xFFFF0000;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct CLAIM_SECURITY_ATTRIBUTE_V1 {
    pub Name: ::PWSTR,
    pub ValueType: ::WORD,
    pub Reserved: ::WORD,
    pub Flags: ::DWORD,
    pub ValueCount: ::DWORD,
    // Put data here
}
pub type PCLAIM_SECURITY_ATTRIBUTE_V1 = *mut CLAIM_SECURITY_ATTRIBUTE_V1;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1 {
    pub Name: ::DWORD,
    pub ValueType: ::WORD,
    pub Reserved: ::WORD,
    pub Flags: ::DWORD,
    pub ValueCount: ::DWORD,
    // Put array here
}
pub type PCLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1 = *mut CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1;
pub const CLAIM_SECURITY_ATTRIBUTES_INFORMATION_VERSION_V1: ::WORD = 1;
pub const CLAIM_SECURITY_ATTRIBUTES_INFORMATION_VERSION: ::WORD =
    CLAIM_SECURITY_ATTRIBUTES_INFORMATION_VERSION_V1;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct CLAIM_SECURITY_ATTRIBUTES_INFORMATION {
    pub Version: ::WORD,
    pub Reserved: ::WORD,
    pub AttributeCount: ::DWORD,
    pub pAttributeV1: PCLAIM_SECURITY_ATTRIBUTE_V1,
}
pub type PCLAIM_SECURITY_ATTRIBUTES_INFORMATION = *mut CLAIM_SECURITY_ATTRIBUTES_INFORMATION;
//11294
pub const PROCESS_TERMINATE: ::DWORD = 0x0001;
pub const PROCESS_CREATE_THREAD: ::DWORD = 0x0002;
pub const PROCESS_SET_SESSIONID: ::DWORD = 0x0004;
pub const PROCESS_VM_OPERATION: ::DWORD = 0x0008;
pub const PROCESS_VM_READ: ::DWORD = 0x0010;
pub const PROCESS_VM_WRITE: ::DWORD = 0x0020;
pub const PROCESS_DUP_HANDLE: ::DWORD = 0x0040;
pub const PROCESS_CREATE_PROCESS: ::DWORD = 0x0080;
pub const PROCESS_SET_QUOTA: ::DWORD = 0x0100;
pub const PROCESS_SET_INFORMATION: ::DWORD = 0x0200;
pub const PROCESS_QUERY_INFORMATION: ::DWORD = 0x0400;
pub const PROCESS_SUSPEND_RESUME: ::DWORD = 0x0800;
pub const PROCESS_QUERY_LIMITED_INFORMATION: ::DWORD = 0x1000;
pub const PROCESS_SET_LIMITED_INFORMATION: ::DWORD = 0x2000;
pub const PROCESS_ALL_ACCESS: ::DWORD = STANDARD_RIGHTS_REQUIRED | SYNCHRONIZE | 0xFFFF;
//11490
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IO_COUNTERS {
    pub ReadOperationCount: ::ULONGLONG,
    pub WriteOperationCount: ::ULONGLONG,
    pub OtherOperationCount: ::ULONGLONG,
    pub ReadTransferCount: ::ULONGLONG,
    pub WriteTransferCount: ::ULONGLONG,
    pub OtherTransferCount: ::ULONGLONG,
}
pub type PIO_COUNTERS = *mut IO_COUNTERS;
//11607
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct JOBOBJECT_BASIC_LIMIT_INFORMATION {
    pub PerProcessUserTimeLimit: ::LARGE_INTEGER,
    pub PerJobUserTimeLimit: ::LARGE_INTEGER,
    pub LimitFlags: ::DWORD,
    pub MinimumWorkingSetSize: ::SIZE_T,
    pub MaximumWorkingSetSize: ::SIZE_T,
    pub ActiveProcessLimit: ::DWORD,
    pub Affinity: ::ULONG_PTR,
    pub PriorityClass: ::DWORD,
    pub SchedulingClass: ::DWORD,
}
pub type PJOBOBJECT_BASIC_LIMIT_INFORMATION = *mut JOBOBJECT_BASIC_LIMIT_INFORMATION;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct JOBOBJECT_EXTENDED_LIMIT_INFORMATION {
    pub BasicLimitInformation: JOBOBJECT_BASIC_LIMIT_INFORMATION,
    pub IoInfo: IO_COUNTERS,
    pub ProcessMemoryLimit: ::SIZE_T,
    pub JobMemoryLimit: ::SIZE_T,
    pub PeakProcessMemoryUsed: ::SIZE_T,
    pub PeakJobMemoryUsed: ::SIZE_T,
}
pub type PJOBOBJECT_EXTENDED_LIMIT_INFORMATION = *mut JOBOBJECT_EXTENDED_LIMIT_INFORMATION;
#[repr(C)] #[derive(Debug)] #[allow(missing_copy_implementations)]
pub struct JOBOBJECT_BASIC_PROCESS_ID_LIST {
    pub NumberOfAssignedProcesses: ::DWORD,
    pub NumberOfProcessIdsInList: ::DWORD,
    pub ProcessIdList: [::ULONG_PTR; 0],
}
pub type PJOBOBJECT_BASIC_PROCESS_ID_LIST = *mut JOBOBJECT_BASIC_PROCESS_ID_LIST;
//11712
pub const JOB_OBJECT_TERMINATE_AT_END_OF_JOB: ::DWORD = 0;
pub const JOB_OBJECT_POST_AT_END_OF_JOB: ::DWORD = 1;
pub const JOB_OBJECT_MSG_END_OF_JOB_TIME: ::DWORD = 1;
pub const JOB_OBJECT_MSG_END_OF_PROCESS_TIME: ::DWORD = 2;
pub const JOB_OBJECT_MSG_ACTIVE_PROCESS_LIMIT: ::DWORD = 3;
pub const JOB_OBJECT_MSG_ACTIVE_PROCESS_ZERO: ::DWORD = 4;
pub const JOB_OBJECT_MSG_NEW_PROCESS: ::DWORD = 6;
pub const JOB_OBJECT_MSG_EXIT_PROCESS: ::DWORD = 7;
pub const JOB_OBJECT_MSG_ABNORMAL_EXIT_PROCESS: ::DWORD = 8;
pub const JOB_OBJECT_MSG_PROCESS_MEMORY_LIMIT: ::DWORD = 9;
pub const JOB_OBJECT_MSG_JOB_MEMORY_LIMIT: ::DWORD = 10;
pub const JOB_OBJECT_MSG_NOTIFICATION_LIMIT: ::DWORD = 11;
pub const JOB_OBJECT_MSG_JOB_CYCLE_TIME_LIMIT: ::DWORD = 12;
pub const JOB_OBJECT_MSG_MINIMUM: ::DWORD = 1;
pub const JOB_OBJECT_MSG_MAXIMUM: ::DWORD = 12;
pub const JOB_OBJECT_VALID_COMPLETION_FILTER: ::DWORD = ((1 << (JOB_OBJECT_MSG_MAXIMUM + 1)) - 1)
    - ((1 << JOB_OBJECT_MSG_MINIMUM) - 1);
pub const JOB_OBJECT_LIMIT_WORKINGSET: ::DWORD = 0x00000001;
pub const JOB_OBJECT_LIMIT_PROCESS_TIME: ::DWORD = 0x00000002;
pub const JOB_OBJECT_LIMIT_JOB_TIME: ::DWORD = 0x00000004;
pub const JOB_OBJECT_LIMIT_ACTIVE_PROCESS: ::DWORD = 0x00000008;
pub const JOB_OBJECT_LIMIT_AFFINITY: ::DWORD = 0x00000010;
pub const JOB_OBJECT_LIMIT_PRIORITY_CLASS: ::DWORD = 0x00000020;
pub const JOB_OBJECT_LIMIT_PRESERVE_JOB_TIME: ::DWORD = 0x00000040;
pub const JOB_OBJECT_LIMIT_SCHEDULING_CLASS: ::DWORD = 0x00000080;
pub const JOB_OBJECT_LIMIT_PROCESS_MEMORY: ::DWORD = 0x00000100;
pub const JOB_OBJECT_LIMIT_JOB_MEMORY: ::DWORD = 0x00000200;
pub const JOB_OBJECT_LIMIT_DIE_ON_UNHANDLED_EXCEPTION: ::DWORD = 0x00000400;
pub const JOB_OBJECT_LIMIT_BREAKAWAY_OK: ::DWORD = 0x00000800;
pub const JOB_OBJECT_LIMIT_SILENT_BREAKAWAY_OK: ::DWORD = 0x00001000;
pub const JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE: ::DWORD = 0x00002000;
pub const JOB_OBJECT_LIMIT_SUBSET_AFFINITY: ::DWORD = 0x00004000;
pub const JOB_OBJECT_LIMIT_JOB_READ_BYTES: ::DWORD = 0x00010000;
pub const JOB_OBJECT_LIMIT_JOB_WRITE_BYTES: ::DWORD = 0x00020000;
pub const JOB_OBJECT_LIMIT_RATE_CONTROL: ::DWORD = 0x00040000;
pub const JOB_OBJECT_LIMIT_RESERVED3: ::DWORD = 0x00008000;
pub const JOB_OBJECT_LIMIT_VALID_FLAGS: ::DWORD = 0x0007ffff;
pub const JOB_OBJECT_BASIC_LIMIT_VALID_FLAGS: ::DWORD = 0x000000ff;
pub const JOB_OBJECT_EXTENDED_LIMIT_VALID_FLAGS: ::DWORD = 0x00007fff;
pub const JOB_OBJECT_NOTIFICATION_LIMIT_VALID_FLAGS: ::DWORD = 0x00070204;
pub const JOB_OBJECT_RESERVED_LIMIT_VALID_FLAGS: ::DWORD = 0x0007ffff;
pub const JOB_OBJECT_UILIMIT_NONE: ::DWORD = 0x00000000;
pub const JOB_OBJECT_UILIMIT_HANDLES: ::DWORD = 0x00000001;
pub const JOB_OBJECT_UILIMIT_READCLIPBOARD: ::DWORD = 0x00000002;
pub const JOB_OBJECT_UILIMIT_WRITECLIPBOARD: ::DWORD = 0x00000004;
pub const JOB_OBJECT_UILIMIT_SYSTEMPARAMETERS: ::DWORD = 0x00000008;
pub const JOB_OBJECT_UILIMIT_DISPLAYSETTINGS: ::DWORD = 0x00000010;
pub const JOB_OBJECT_UILIMIT_GLOBALATOMS: ::DWORD = 0x00000020;
pub const JOB_OBJECT_UILIMIT_DESKTOP: ::DWORD = 0x00000040;
pub const JOB_OBJECT_UILIMIT_EXITWINDOWS: ::DWORD = 0x00000080;
pub const JOB_OBJECT_UILIMIT_ALL: ::DWORD = 0x000000FF;
pub const JOB_OBJECT_UI_VALID_FLAGS: ::DWORD = 0x000000FF;
pub const JOB_OBJECT_SECURITY_NO_ADMIN: ::DWORD = 0x00000001;
pub const JOB_OBJECT_SECURITY_RESTRICTED_TOKEN: ::DWORD = 0x00000002;
pub const JOB_OBJECT_SECURITY_ONLY_TOKEN: ::DWORD = 0x00000004;
pub const JOB_OBJECT_SECURITY_FILTER_TOKENS: ::DWORD = 0x00000008;
pub const JOB_OBJECT_SECURITY_VALID_FLAGS: ::DWORD = 0x0000000f;
pub const JOB_OBJECT_CPU_RATE_CONTROL_ENABLE: ::DWORD = 0x1;
pub const JOB_OBJECT_CPU_RATE_CONTROL_WEIGHT_BASED: ::DWORD = 0x2;
pub const JOB_OBJECT_CPU_RATE_CONTROL_HARD_CAP: ::DWORD = 0x4;
pub const JOB_OBJECT_CPU_RATE_CONTROL_NOTIFY: ::DWORD = 0x8;
pub const JOB_OBJECT_CPU_RATE_CONTROL_VALID_FLAGS: ::DWORD = 0xf;
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum JOBOBJECTINFOCLASS {
    JobObjectBasicAccountingInformation = 1,
    JobObjectBasicLimitInformation,
    JobObjectBasicProcessIdList,
    JobObjectBasicUIRestrictions,
    JobObjectSecurityLimitInformation,
    JobObjectEndOfJobTimeInformation,
    JobObjectAssociateCompletionPortInformation,
    JobObjectBasicAndIoAccountingInformation,
    JobObjectExtendedLimitInformation,
    JobObjectJobSetInformation,
    JobObjectGroupInformation,
    JobObjectNotificationLimitInformation,
    JobObjectLimitViolationInformation,
    JobObjectGroupInformationEx,
    JobObjectCpuRateControlInformation,
    JobObjectCompletionFilter,
    JobObjectCompletionCounter,
    JobObjectReserved1Information = 18,
    JobObjectReserved2Information,
    JobObjectReserved3Information,
    JobObjectReserved4Information,
    JobObjectReserved5Information,
    JobObjectReserved6Information,
    JobObjectReserved7Information,
    JobObjectReserved8Information,
    JobObjectReserved9Information,
    MaxJobObjectInfoClass,
}
pub const PAGE_NOACCESS: ::DWORD = 0x01;
pub const PAGE_READONLY: ::DWORD = 0x02;
pub const PAGE_READWRITE: ::DWORD = 0x04;
pub const PAGE_WRITECOPY: ::DWORD = 0x08;
pub const PAGE_EXECUTE: ::DWORD = 0x10;
pub const PAGE_EXECUTE_READ: ::DWORD = 0x20;
pub const PAGE_EXECUTE_READWRITE: ::DWORD = 0x40;
pub const PAGE_EXECUTE_WRITECOPY: ::DWORD = 0x80;
pub const PAGE_GUARD: ::DWORD = 0x100;
pub const PAGE_NOCACHE: ::DWORD = 0x200;
pub const PAGE_WRITECOMBINE: ::DWORD = 0x400;
pub const PAGE_REVERT_TO_FILE_MAP: ::DWORD = 0x80000000;
pub const PAGE_TARGETS_NO_UPDATE: ::DWORD = 0x40000000;
pub const PAGE_TARGETS_INVALID: ::DWORD = 0x40000000;
pub const MEM_COMMIT: ::DWORD = 0x1000;
pub const MEM_RESERVE: ::DWORD = 0x2000;
pub const MEM_DECOMMIT: ::DWORD = 0x4000;
pub const MEM_RELEASE: ::DWORD = 0x8000;
pub const MEM_FREE: ::DWORD = 0x10000;
pub const MEM_PRIVATE: ::DWORD = 0x20000;
pub const MEM_MAPPED: ::DWORD = 0x40000;
pub const MEM_RESET: ::DWORD = 0x80000;
pub const MEM_TOP_DOWN: ::DWORD = 0x100000;
pub const MEM_WRITE_WATCH: ::DWORD = 0x200000;
pub const MEM_PHYSICAL: ::DWORD = 0x400000;
pub const MEM_ROTATE: ::DWORD = 0x800000;
pub const MEM_DIFFERENT_IMAGE_BASE_OK: ::DWORD = 0x800000;
pub const MEM_RESET_UNDO: ::DWORD = 0x1000000;
pub const MEM_LARGE_PAGES: ::DWORD = 0x20000000;
pub const MEM_4MB_PAGES: ::DWORD = 0x80000000;
pub const SEC_FILE: ::DWORD = 0x800000;
pub const SEC_IMAGE: ::DWORD = 0x1000000;
pub const SEC_PROTECTED_IMAGE: ::DWORD = 0x2000000;
pub const SEC_RESERVE: ::DWORD = 0x4000000;
pub const SEC_COMMIT: ::DWORD = 0x8000000;
pub const SEC_NOCACHE: ::DWORD = 0x10000000;
pub const SEC_WRITECOMBINE: ::DWORD = 0x40000000;
pub const SEC_LARGE_PAGES: ::DWORD = 0x80000000;
pub const SEC_IMAGE_NO_EXECUTE: ::DWORD = (SEC_IMAGE | SEC_NOCACHE);
pub const MEM_IMAGE: ::DWORD = SEC_IMAGE;
pub const WRITE_WATCH_FLAG_RESET: ::DWORD = 0x01;
pub const MEM_UNMAP_WITH_TRANSIENT_BOOST: ::DWORD = 0x01;
//12217
pub const FILE_READ_DATA: ::DWORD = 0x0001;
pub const FILE_LIST_DIRECTORY: ::DWORD = 0x0001;
pub const FILE_WRITE_DATA: ::DWORD = 0x0002;
pub const FILE_ADD_FILE: ::DWORD = 0x0002;
pub const FILE_APPEND_DATA: ::DWORD = 0x0004;
pub const FILE_ADD_SUBDIRECTORY: ::DWORD = 0x0004;
pub const FILE_CREATE_PIPE_INSTANCE: ::DWORD = 0x0004;
pub const FILE_READ_EA: ::DWORD = 0x0008;
pub const FILE_WRITE_EA: ::DWORD = 0x0010;
pub const FILE_EXECUTE: ::DWORD = 0x0020;
pub const FILE_TRAVERSE: ::DWORD = 0x0020;
pub const FILE_DELETE_CHILD: ::DWORD = 0x0040;
pub const FILE_READ_ATTRIBUTES: ::DWORD = 0x0080;
pub const FILE_WRITE_ATTRIBUTES: ::DWORD = 0x0100;
pub const FILE_ALL_ACCESS: ::DWORD = STANDARD_RIGHTS_REQUIRED | SYNCHRONIZE | 0x1FF;
pub const FILE_GENERIC_READ: ::DWORD = STANDARD_RIGHTS_READ | FILE_READ_DATA
    | FILE_READ_ATTRIBUTES | FILE_READ_EA | SYNCHRONIZE;
pub const FILE_GENERIC_WRITE: ::DWORD = STANDARD_RIGHTS_WRITE | FILE_WRITE_DATA
    | FILE_WRITE_ATTRIBUTES | FILE_WRITE_EA | FILE_APPEND_DATA | SYNCHRONIZE;
pub const FILE_GENERIC_EXECUTE: ::DWORD = STANDARD_RIGHTS_EXECUTE | FILE_READ_ATTRIBUTES
    | FILE_EXECUTE | SYNCHRONIZE;
pub const FILE_SHARE_READ: ::DWORD = 0x00000001;
pub const FILE_SHARE_WRITE: ::DWORD = 0x00000002;
pub const FILE_SHARE_DELETE: ::DWORD = 0x00000004;
pub const FILE_ATTRIBUTE_READONLY: ::DWORD = 0x00000001;
pub const FILE_ATTRIBUTE_HIDDEN: ::DWORD = 0x00000002;
pub const FILE_ATTRIBUTE_SYSTEM: ::DWORD = 0x00000004;
pub const FILE_ATTRIBUTE_DIRECTORY: ::DWORD = 0x00000010;
pub const FILE_ATTRIBUTE_ARCHIVE: ::DWORD = 0x00000020;
pub const FILE_ATTRIBUTE_DEVICE: ::DWORD = 0x00000040;
pub const FILE_ATTRIBUTE_NORMAL: ::DWORD = 0x00000080;
pub const FILE_ATTRIBUTE_TEMPORARY: ::DWORD = 0x00000100;
pub const FILE_ATTRIBUTE_SPARSE_FILE: ::DWORD = 0x00000200;
pub const FILE_ATTRIBUTE_REPARSE_POINT: ::DWORD = 0x00000400;
pub const FILE_ATTRIBUTE_COMPRESSED: ::DWORD = 0x00000800;
pub const FILE_ATTRIBUTE_OFFLINE: ::DWORD = 0x00001000;
pub const FILE_ATTRIBUTE_NOT_CONTENT_INDEXED: ::DWORD = 0x00002000;
pub const FILE_ATTRIBUTE_ENCRYPTED: ::DWORD = 0x00004000;
pub const FILE_ATTRIBUTE_INTEGRITY_STREAM: ::DWORD = 0x00008000;
pub const FILE_ATTRIBUTE_VIRTUAL: ::DWORD = 0x00010000;
pub const FILE_ATTRIBUTE_NO_SCRUB_DATA: ::DWORD = 0x00020000;
pub const FILE_ATTRIBUTE_EA: ::DWORD = 0x00040000;
pub const FILE_NOTIFY_CHANGE_FILE_NAME: ::DWORD = 0x00000001;
pub const FILE_NOTIFY_CHANGE_DIR_NAME: ::DWORD = 0x00000002;
pub const FILE_NOTIFY_CHANGE_ATTRIBUTES: ::DWORD = 0x00000004;
pub const FILE_NOTIFY_CHANGE_SIZE: ::DWORD = 0x00000008;
pub const FILE_NOTIFY_CHANGE_LAST_WRITE: ::DWORD = 0x00000010;
pub const FILE_NOTIFY_CHANGE_LAST_ACCESS: ::DWORD = 0x00000020;
pub const FILE_NOTIFY_CHANGE_CREATION: ::DWORD = 0x00000040;
pub const FILE_NOTIFY_CHANGE_SECURITY: ::DWORD = 0x00000100;
pub const FILE_ACTION_ADDED: ::DWORD = 0x00000001;
pub const FILE_ACTION_REMOVED: ::DWORD = 0x00000002;
pub const FILE_ACTION_MODIFIED: ::DWORD = 0x00000003;
pub const FILE_ACTION_RENAMED_OLD_NAME: ::DWORD = 0x00000004;
pub const FILE_ACTION_RENAMED_NEW_NAME: ::DWORD = 0x00000005;
pub const MAILSLOT_NO_MESSAGE: ::DWORD = 0xFFFFFFFF;
pub const MAILSLOT_WAIT_FOREVER: ::DWORD = 0xFFFFFFFF;
pub const FILE_CASE_SENSITIVE_SEARCH: ::DWORD = 0x00000001;
pub const FILE_CASE_PRESERVED_NAMES: ::DWORD = 0x00000002;
pub const FILE_UNICODE_ON_DISK: ::DWORD = 0x00000004;
pub const FILE_PERSISTENT_ACLS: ::DWORD = 0x00000008;
pub const FILE_FILE_COMPRESSION: ::DWORD = 0x00000010;
pub const FILE_VOLUME_QUOTAS: ::DWORD = 0x00000020;
pub const FILE_SUPPORTS_SPARSE_FILES: ::DWORD = 0x00000040;
pub const FILE_SUPPORTS_REPARSE_POINTS: ::DWORD = 0x00000080;
pub const FILE_SUPPORTS_REMOTE_STORAGE: ::DWORD = 0x00000100;
pub const FILE_VOLUME_IS_COMPRESSED: ::DWORD = 0x00008000;
pub const FILE_SUPPORTS_OBJECT_IDS: ::DWORD = 0x00010000;
pub const FILE_SUPPORTS_ENCRYPTION: ::DWORD = 0x00020000;
pub const FILE_NAMED_STREAMS: ::DWORD = 0x00040000;
pub const FILE_READ_ONLY_VOLUME: ::DWORD = 0x00080000;
pub const FILE_SEQUENTIAL_WRITE_ONCE: ::DWORD = 0x00100000;
pub const FILE_SUPPORTS_TRANSACTIONS: ::DWORD = 0x00200000;
pub const FILE_SUPPORTS_HARD_LINKS: ::DWORD = 0x00400000;
pub const FILE_SUPPORTS_EXTENDED_ATTRIBUTES: ::DWORD = 0x00800000;
pub const FILE_SUPPORTS_OPEN_BY_FILE_ID: ::DWORD = 0x01000000;
pub const FILE_SUPPORTS_USN_JOURNAL: ::DWORD = 0x02000000;
pub const FILE_SUPPORTS_INTEGRITY_STREAMS: ::DWORD = 0x04000000;
pub const FILE_INVALID_FILE_ID: ::LONGLONG = -1;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct FILE_ID_128 {
    pub Identifier: [::BYTE; 16],
}
pub type PFILE_ID_128 = *mut FILE_ID_128;
#[repr(C)] #[derive(Debug)] #[allow(missing_copy_implementations)]
pub struct FILE_NOTIFY_INFORMATION {
    pub NextEntryOffset: ::DWORD,
    pub Action: ::DWORD,
    pub FileNameLength: ::DWORD,
    pub FileName: [::WCHAR; 0],
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct FILE_SEGMENT_ELEMENT {
    pub Buffer: ::PVOID64,
    pub Alignment: ::ULONGLONG,
}
pub type PFILE_SEGMENT_ELEMENT = *mut FILE_SEGMENT_ELEMENT;
//12475
pub const IO_REPARSE_TAG_MOUNT_POINT: ::DWORD = 0xA0000003;
pub const IO_REPARSE_TAG_HSM: ::DWORD = 0xC0000004;
pub const IO_REPARSE_TAG_HSM2: ::DWORD = 0x80000006;
pub const IO_REPARSE_TAG_SIS: ::DWORD = 0x80000007;
pub const IO_REPARSE_TAG_WIM: ::DWORD = 0x80000008;
pub const IO_REPARSE_TAG_CSV: ::DWORD = 0x80000009;
pub const IO_REPARSE_TAG_DFS: ::DWORD = 0x8000000A;
pub const IO_REPARSE_TAG_SYMLINK: ::DWORD = 0xA000000C;
pub const IO_REPARSE_TAG_DFSR: ::DWORD = 0x80000012;
pub const IO_REPARSE_TAG_DEDUP: ::DWORD = 0x80000013;
pub const IO_REPARSE_TAG_NFS: ::DWORD = 0x80000014;
pub const IO_REPARSE_TAG_FILE_PLACEHOLDER: ::DWORD = 0x80000015;
pub const IO_REPARSE_TAG_WOF: ::DWORD = 0x80000017;
//15000
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IMAGE_FILE_HEADER {
    pub Machine: ::WORD,
    pub NumberOfSections: ::WORD,
    pub TimeDateStamp: ::DWORD,
    pub PointerToSymbolTable: ::DWORD,
    pub NumberOfSymbols: ::DWORD,
    pub SizeOfOptionalHeader: ::WORD,
    pub Characteristics: ::WORD,
}
pub type PIMAGE_FILE_HEADER = *mut IMAGE_FILE_HEADER;
pub const IMAGE_SIZEOF_FILE_HEADER: usize = 20;
pub const IMAGE_FILE_RELOCS_STRIPPED: ::WORD = 0x0001;
pub const IMAGE_FILE_EXECUTABLE_IMAGE: ::WORD = 0x0002;
pub const IMAGE_FILE_LINE_NUMS_STRIPPED: ::WORD = 0x0004;
pub const IMAGE_FILE_LOCAL_SYMS_STRIPPED: ::WORD = 0x0008;
pub const IMAGE_FILE_AGGRESIVE_WS_TRIM: ::WORD = 0x0010;
pub const IMAGE_FILE_LARGE_ADDRESS_AWARE: ::WORD = 0x0020;
pub const IMAGE_FILE_BYTES_REVERSED_LO: ::WORD = 0x0080;
pub const IMAGE_FILE_32BIT_MACHINE: ::WORD = 0x0100;
pub const IMAGE_FILE_DEBUG_STRIPPED: ::WORD = 0x0200;
pub const IMAGE_FILE_REMOVABLE_RUN_FROM_SWAP: ::WORD = 0x0400;
pub const IMAGE_FILE_NET_RUN_FROM_SWAP: ::WORD = 0x0800;
pub const IMAGE_FILE_SYSTEM: ::WORD = 0x1000;
pub const IMAGE_FILE_DLL: ::WORD = 0x2000;
pub const IMAGE_FILE_UP_SYSTEM_ONLY: ::WORD = 0x4000;
pub const IMAGE_FILE_BYTES_REVERSED_HI: ::WORD = 0x8000;
pub const IMAGE_FILE_MACHINE_UNKNOWN: ::WORD = 0;
pub const IMAGE_FILE_MACHINE_I386: ::WORD = 0x014c;
pub const IMAGE_FILE_MACHINE_R3000: ::WORD = 0x0162;
pub const IMAGE_FILE_MACHINE_R4000: ::WORD = 0x0166;
pub const IMAGE_FILE_MACHINE_R10000: ::WORD = 0x0168;
pub const IMAGE_FILE_MACHINE_WCEMIPSV2: ::WORD = 0x0169;
pub const IMAGE_FILE_MACHINE_ALPHA: ::WORD = 0x0184;
pub const IMAGE_FILE_MACHINE_SH3: ::WORD = 0x01a2;
pub const IMAGE_FILE_MACHINE_SH3DSP: ::WORD = 0x01a3;
pub const IMAGE_FILE_MACHINE_SH3E: ::WORD = 0x01a4;
pub const IMAGE_FILE_MACHINE_SH4: ::WORD = 0x01a6;
pub const IMAGE_FILE_MACHINE_SH5: ::WORD = 0x01a8;
pub const IMAGE_FILE_MACHINE_ARM: ::WORD = 0x01c0;
pub const IMAGE_FILE_MACHINE_THUMB: ::WORD = 0x01c2;
pub const IMAGE_FILE_MACHINE_ARMNT: ::WORD = 0x01c4;
pub const IMAGE_FILE_MACHINE_AM33: ::WORD = 0x01d3;
pub const IMAGE_FILE_MACHINE_POWERPC: ::WORD = 0x01F0;
pub const IMAGE_FILE_MACHINE_POWERPCFP: ::WORD = 0x01f1;
pub const IMAGE_FILE_MACHINE_IA64: ::WORD = 0x0200;
pub const IMAGE_FILE_MACHINE_MIPS16: ::WORD = 0x0266;
pub const IMAGE_FILE_MACHINE_ALPHA64: ::WORD = 0x0284;
pub const IMAGE_FILE_MACHINE_MIPSFPU: ::WORD = 0x0366;
pub const IMAGE_FILE_MACHINE_MIPSFPU16: ::WORD = 0x0466;
pub const IMAGE_FILE_MACHINE_AXP64: ::WORD = IMAGE_FILE_MACHINE_ALPHA64;
pub const IMAGE_FILE_MACHINE_TRICORE: ::WORD = 0x0520;
pub const IMAGE_FILE_MACHINE_CEF: ::WORD = 0x0CEF;
pub const IMAGE_FILE_MACHINE_EBC: ::WORD = 0x0EBC;
pub const IMAGE_FILE_MACHINE_AMD64: ::WORD = 0x8664;
pub const IMAGE_FILE_MACHINE_M32R: ::WORD = 0x9041;
pub const IMAGE_FILE_MACHINE_CEE: ::WORD = 0xC0EE;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IMAGE_DATA_DIRECTORY {
    pub VirtualAddress: ::DWORD,
    pub Size: ::DWORD,
}
pub type PIMAGE_DATA_DIRECTORY = *mut IMAGE_DATA_DIRECTORY;
pub const IMAGE_NUMBEROF_DIRECTORY_ENTRIES: usize = 16;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IMAGE_OPTIONAL_HEADER32 {
    pub Magic: ::WORD,
    pub MajorLinkerVersion: ::BYTE,
    pub MinorLinkerVersion: ::BYTE,
    pub SizeOfCode: ::DWORD,
    pub SizeOfInitializedData: ::DWORD,
    pub SizeOfUninitializedData: ::DWORD,
    pub AddressOfEntryPoint: ::DWORD,
    pub BaseOfCode: ::DWORD,
    pub BaseOfData: ::DWORD,
    pub ImageBase: ::DWORD,
    pub SectionAlignment: ::DWORD,
    pub FileAlignment: ::DWORD,
    pub MajorOperatingSystemVersion: ::WORD,
    pub MinorOperatingSystemVersion: ::WORD,
    pub MajorImageVersion: ::WORD,
    pub MinorImageVersion: ::WORD,
    pub MajorSubsystemVersion: ::WORD,
    pub MinorSubsystemVersion: ::WORD,
    pub Win32VersionValue: ::DWORD,
    pub SizeOfImage: ::DWORD,
    pub SizeOfHeaders: ::DWORD,
    pub CheckSum: ::DWORD,
    pub Subsystem: ::WORD,
    pub DllCharacteristics: ::WORD,
    pub SizeOfStackReserve: ::DWORD,
    pub SizeOfStackCommit: ::DWORD,
    pub SizeOfHeapReserve: ::DWORD,
    pub SizeOfHeapCommit: ::DWORD,
    pub LoaderFlags: ::DWORD,
    pub NumberOfRvaAndSizes: ::DWORD,
    pub DataDirectory: [IMAGE_DATA_DIRECTORY; IMAGE_NUMBEROF_DIRECTORY_ENTRIES],
}
pub type PIMAGE_OPTIONAL_HEADER32 = *mut IMAGE_OPTIONAL_HEADER32;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IMAGE_ROM_OPTIONAL_HEADER {
    pub Magic: ::WORD,
    pub MajorLinkerVersion: ::BYTE,
    pub MinorLinkerVersion: ::BYTE,
    pub SizeOfCode: ::DWORD,
    pub SizeOfInitializedData: ::DWORD,
    pub SizeOfUninitializedData: ::DWORD,
    pub AddressOfEntryPoint: ::DWORD,
    pub BaseOfCode: ::DWORD,
    pub BaseOfData: ::DWORD,
    pub BaseOfBss: ::DWORD,
    pub GprMask: ::DWORD,
    pub CprMask: [::DWORD; 4],
    pub GpValue: ::DWORD,
}
pub type PIMAGE_ROM_OPTIONAL_HEADER = *mut IMAGE_ROM_OPTIONAL_HEADER;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IMAGE_OPTIONAL_HEADER64 {
    pub Magic: ::WORD,
    pub MajorLinkerVersion: ::BYTE,
    pub MinorLinkerVersion: ::BYTE,
    pub SizeOfCode: ::DWORD,
    pub SizeOfInitializedData: ::DWORD,
    pub SizeOfUninitializedData: ::DWORD,
    pub AddressOfEntryPoint: ::DWORD,
    pub BaseOfCode: ::DWORD,
    pub ImageBase: ::ULONGLONG,
    pub SectionAlignment: ::DWORD,
    pub FileAlignment: ::DWORD,
    pub MajorOperatingSystemVersion: ::WORD,
    pub MinorOperatingSystemVersion: ::WORD,
    pub MajorImageVersion: ::WORD,
    pub MinorImageVersion: ::WORD,
    pub MajorSubsystemVersion: ::WORD,
    pub MinorSubsystemVersion: ::WORD,
    pub Win32VersionValue: ::DWORD,
    pub SizeOfImage: ::DWORD,
    pub SizeOfHeaders: ::DWORD,
    pub CheckSum: ::DWORD,
    pub Subsystem: ::WORD,
    pub DllCharacteristics: ::WORD,
    pub SizeOfStackReserve: ULONGLONG,
    pub SizeOfStackCommit: ULONGLONG,
    pub SizeOfHeapReserve: ULONGLONG,
    pub SizeOfHeapCommit: ULONGLONG,
    pub LoaderFlags: ::DWORD,
    pub NumberOfRvaAndSizes: ::DWORD,
    pub DataDirectory: [IMAGE_DATA_DIRECTORY; IMAGE_NUMBEROF_DIRECTORY_ENTRIES],
}
pub type PIMAGE_OPTIONAL_HEADER64 = *mut IMAGE_OPTIONAL_HEADER64;
pub const IMAGE_NT_OPTIONAL_HDR32_MAGIC: ::WORD = 0x10b;
pub const IMAGE_NT_OPTIONAL_HDR64_MAGIC: ::WORD = 0x20b;
pub const IMAGE_ROM_OPTIONAL_HDR_MAGIC: ::WORD = 0x107;
#[cfg(target_arch = "x86_64")]
pub type IMAGE_OPTIONAL_HEADER = IMAGE_OPTIONAL_HEADER64;
#[cfg(target_arch = "x86_64")]
pub type PIMAGE_OPTIONAL_HEADER = PIMAGE_OPTIONAL_HEADER64;
#[cfg(target_arch = "x86")]
pub type IMAGE_OPTIONAL_HEADER = IMAGE_OPTIONAL_HEADER32;
#[cfg(target_arch = "x86")]
pub type PIMAGE_OPTIONAL_HEADER = PIMAGE_OPTIONAL_HEADER32;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IMAGE_NT_HEADERS64 {
    pub Signature: ::DWORD,
    pub FileHeader: IMAGE_FILE_HEADER,
    pub OptionalHeader: IMAGE_OPTIONAL_HEADER64,
}
pub type PIMAGE_NT_HEADERS64 = *mut IMAGE_NT_HEADERS64;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IMAGE_NT_HEADERS32 {
    pub Signature: ::DWORD,
    pub FileHeader: IMAGE_FILE_HEADER,
    pub OptionalHeader: IMAGE_OPTIONAL_HEADER32,
}
pub type PIMAGE_NT_HEADERS32 = *mut IMAGE_NT_HEADERS32;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IMAGE_ROM_HEADERS {
    pub FileHeader: IMAGE_FILE_HEADER,
    pub OptionalHeader: IMAGE_ROM_OPTIONAL_HEADER,
}
pub type PIMAGE_ROM_HEADERS = *mut IMAGE_ROM_HEADERS;
#[cfg(target_arch = "x86_64")]
pub type IMAGE_NT_HEADERS = IMAGE_NT_HEADERS64;
#[cfg(target_arch = "x86_64")]
pub type PIMAGE_NT_HEADERS = PIMAGE_NT_HEADERS64;
#[cfg(target_arch = "x86")]
pub type IMAGE_NT_HEADERS = IMAGE_NT_HEADERS32;
#[cfg(target_arch = "x86")]
pub type PIMAGE_NT_HEADERS = PIMAGE_NT_HEADERS32;
pub const IMAGE_SUBSYSTEM_UNKNOWN: ::WORD = 0;
pub const IMAGE_SUBSYSTEM_NATIVE: ::WORD = 1;
pub const IMAGE_SUBSYSTEM_WINDOWS_GUI: ::WORD = 2;
pub const IMAGE_SUBSYSTEM_WINDOWS_CUI: ::WORD = 3;
pub const IMAGE_SUBSYSTEM_OS2_CUI: ::WORD = 5;
pub const IMAGE_SUBSYSTEM_POSIX_CUI: ::WORD = 7;
pub const IMAGE_SUBSYSTEM_NATIVE_WINDOWS: ::WORD = 8;
pub const IMAGE_SUBSYSTEM_WINDOWS_CE_GUI: ::WORD = 9;
pub const IMAGE_SUBSYSTEM_EFI_APPLICATION: ::WORD = 10;
pub const IMAGE_SUBSYSTEM_EFI_BOOT_SERVICE_DRIVER: ::WORD = 11;
pub const IMAGE_SUBSYSTEM_EFI_RUNTIME_DRIVER: ::WORD = 12;
pub const IMAGE_SUBSYSTEM_EFI_ROM: ::WORD = 13;
pub const IMAGE_SUBSYSTEM_XBOX: ::WORD = 14;
pub const IMAGE_SUBSYSTEM_WINDOWS_BOOT_APPLICATION: ::WORD = 16;
pub const IMAGE_DLLCHARACTERISTICS_HIGH_ENTROPY_VA: ::WORD = 0x0020;
pub const IMAGE_DLLCHARACTERISTICS_DYNAMIC_BASE: ::WORD = 0x0040;
pub const IMAGE_DLLCHARACTERISTICS_FORCE_INTEGRITY: ::WORD = 0x0080;
pub const IMAGE_DLLCHARACTERISTICS_NX_COMPAT: ::WORD = 0x0100;
pub const IMAGE_DLLCHARACTERISTICS_NO_ISOLATION: ::WORD = 0x0200;
pub const IMAGE_DLLCHARACTERISTICS_NO_SEH: ::WORD = 0x0400;
pub const IMAGE_DLLCHARACTERISTICS_NO_BIND: ::WORD = 0x0800;
pub const IMAGE_DLLCHARACTERISTICS_APPCONTAINER: ::WORD = 0x1000;
pub const IMAGE_DLLCHARACTERISTICS_WDM_DRIVER: ::WORD = 0x2000;
pub const IMAGE_DLLCHARACTERISTICS_GUARD_CF: ::WORD = 0x4000;
pub const IMAGE_DLLCHARACTERISTICS_TERMINAL_SERVER_AWARE: ::WORD = 0x8000;
pub const IMAGE_DIRECTORY_ENTRY_EXPORT: ::WORD = 0;
pub const IMAGE_DIRECTORY_ENTRY_IMPORT: ::WORD = 1;
pub const IMAGE_DIRECTORY_ENTRY_RESOURCE: ::WORD = 2;
pub const IMAGE_DIRECTORY_ENTRY_EXCEPTION: ::WORD = 3;
pub const IMAGE_DIRECTORY_ENTRY_SECURITY: ::WORD = 4;
pub const IMAGE_DIRECTORY_ENTRY_BASERELOC: ::WORD = 5;
pub const IMAGE_DIRECTORY_ENTRY_DEBUG: ::WORD = 6;
pub const IMAGE_DIRECTORY_ENTRY_ARCHITECTURE: ::WORD = 7;
pub const IMAGE_DIRECTORY_ENTRY_GLOBALPTR: ::WORD = 8;
pub const IMAGE_DIRECTORY_ENTRY_TLS: ::WORD = 9;
pub const IMAGE_DIRECTORY_ENTRY_LOAD_CONFIG: ::WORD = 10;
pub const IMAGE_DIRECTORY_ENTRY_BOUND_IMPORT: ::WORD = 11;
pub const IMAGE_DIRECTORY_ENTRY_IAT: ::WORD = 12;
pub const IMAGE_DIRECTORY_ENTRY_DELAY_IMPORT: ::WORD = 13;
pub const IMAGE_DIRECTORY_ENTRY_COM_DESCRIPTOR: ::WORD = 14;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ANON_OBJECT_HEADER {
    pub Sig1: ::WORD,
    pub Sig2: ::WORD,
    pub Version: ::WORD,
    pub Machine: ::WORD,
    pub TimeDateStamp: ::DWORD,
    pub ClassID: ::CLSID,
    pub SizeOfData: ::DWORD,
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ANON_OBJECT_HEADER_V2 {
    pub Sig1: ::WORD,
    pub Sig2: ::WORD,
    pub Version: ::WORD,
    pub Machine: ::WORD,
    pub TimeDateStamp: ::DWORD,
    pub ClassID: ::CLSID,
    pub SizeOfData: ::DWORD,
    pub Flags: ::DWORD,
    pub MetaDataSize: ::DWORD,
    pub MetaDataOffset: ::DWORD,
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ANON_OBJECT_HEADER_BIGOBJ {
    pub Sig1: ::WORD,
    pub Sig2: ::WORD,
    pub Version: ::WORD,
    pub Machine: ::WORD,
    pub TimeDateStamp: ::DWORD,
    pub ClassID: ::CLSID,
    pub SizeOfData: ::DWORD,
    pub Flags: ::DWORD,
    pub MetaDataSize: ::DWORD,
    pub MetaDataOffset: ::DWORD,
    pub NumberOfSections: ::DWORD,
    pub PointerToSymbolTable: ::DWORD,
    pub NumberOfSymbols: ::DWORD,
}
pub const IMAGE_SIZEOF_SHORT_NAME: usize = 8;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IMAGE_SECTION_HEADER {
    pub Name: [::BYTE; IMAGE_SIZEOF_SHORT_NAME],
    pub PhysicalAddressOrVirtualSize: ::DWORD,
    pub VirtualAddress: ::DWORD,
    pub SizeOfRawData: ::DWORD,
    pub PointerToRawData: ::DWORD,
    pub PointerToRelocations: ::DWORD,
    pub PointerToLinenumbers: ::DWORD,
    pub NumberOfRelocations: ::WORD,
    pub NumberOfLinenumbers: ::WORD,
    pub Characteristics: ::DWORD,
}
pub type PIMAGE_SECTION_HEADER = *mut IMAGE_SECTION_HEADER;
pub const IMAGE_SIZEOF_SECTION_HEADER: usize = 40;
pub const IMAGE_SCN_TYPE_NO_PAD: ::DWORD = 0x00000008;
pub const IMAGE_SCN_CNT_CODE: ::DWORD = 0x00000020;
pub const IMAGE_SCN_CNT_INITIALIZED_DATA: ::DWORD = 0x00000040;
pub const IMAGE_SCN_CNT_UNINITIALIZED_DATA: ::DWORD = 0x00000080;
pub const IMAGE_SCN_LNK_OTHER: ::DWORD = 0x00000100;
pub const IMAGE_SCN_LNK_INFO: ::DWORD = 0x00000200;
pub const IMAGE_SCN_LNK_REMOVE: ::DWORD = 0x00000800;
pub const IMAGE_SCN_LNK_COMDAT: ::DWORD = 0x00001000;
pub const IMAGE_SCN_NO_DEFER_SPEC_EXC: ::DWORD = 0x00004000;
pub const IMAGE_SCN_GPREL: ::DWORD = 0x00008000;
pub const IMAGE_SCN_MEM_FARDATA: ::DWORD = 0x00008000;
pub const IMAGE_SCN_MEM_PURGEABLE: ::DWORD = 0x00020000;
pub const IMAGE_SCN_MEM_16BIT: ::DWORD = 0x00020000;
pub const IMAGE_SCN_MEM_LOCKED: ::DWORD = 0x00040000;
pub const IMAGE_SCN_MEM_PRELOAD: ::DWORD = 0x00080000;
pub const IMAGE_SCN_ALIGN_1BYTES: ::DWORD = 0x00100000;
pub const IMAGE_SCN_ALIGN_2BYTES: ::DWORD = 0x00200000;
pub const IMAGE_SCN_ALIGN_4BYTES: ::DWORD = 0x00300000;
pub const IMAGE_SCN_ALIGN_8BYTES: ::DWORD = 0x00400000;
pub const IMAGE_SCN_ALIGN_16BYTES: ::DWORD = 0x00500000;
pub const IMAGE_SCN_ALIGN_32BYTES: ::DWORD = 0x00600000;
pub const IMAGE_SCN_ALIGN_64BYTES: ::DWORD = 0x00700000;
pub const IMAGE_SCN_ALIGN_128BYTES: ::DWORD = 0x00800000;
pub const IMAGE_SCN_ALIGN_256BYTES: ::DWORD = 0x00900000;
pub const IMAGE_SCN_ALIGN_512BYTES: ::DWORD = 0x00A00000;
pub const IMAGE_SCN_ALIGN_1024BYTES: ::DWORD = 0x00B00000;
pub const IMAGE_SCN_ALIGN_2048BYTES: ::DWORD = 0x00C00000;
pub const IMAGE_SCN_ALIGN_4096BYTES: ::DWORD = 0x00D00000;
pub const IMAGE_SCN_ALIGN_8192BYTES: ::DWORD = 0x00E00000;
pub const IMAGE_SCN_ALIGN_MASK: ::DWORD = 0x00F00000;
pub const IMAGE_SCN_LNK_NRELOC_OVFL: ::DWORD = 0x01000000;
pub const IMAGE_SCN_MEM_DISCARDABLE: ::DWORD = 0x02000000;
pub const IMAGE_SCN_MEM_NOT_CACHED: ::DWORD = 0x04000000;
pub const IMAGE_SCN_MEM_NOT_PAGED: ::DWORD = 0x08000000;
pub const IMAGE_SCN_MEM_SHARED: ::DWORD = 0x10000000;
pub const IMAGE_SCN_MEM_EXECUTE: ::DWORD = 0x20000000;
pub const IMAGE_SCN_MEM_READ: ::DWORD = 0x40000000;
pub const IMAGE_SCN_MEM_WRITE: ::DWORD = 0x80000000;
pub const IMAGE_SCN_SCALE_INDEX: ::DWORD = 0x00000001;
//16590
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IMAGE_DEBUG_DIRECTORY {
    pub Characteristics: ::DWORD,
    pub TimeDateStamp: ::DWORD,
    pub MajorVersion: ::WORD,
    pub MinorVersion: ::WORD,
    pub Type: ::DWORD,
    pub SizeOfData: ::DWORD,
    pub AddressOfRawData: ::DWORD,
    pub PointerToRawData: ::DWORD,
}
pub type PIMAGE_DEBUG_DIRECTORY = *mut IMAGE_DEBUG_DIRECTORY;
pub const IMAGE_DEBUG_TYPE_UNKNOWN: ::DWORD = 0;
pub const IMAGE_DEBUG_TYPE_COFF: ::DWORD = 1;
pub const IMAGE_DEBUG_TYPE_CODEVIEW: ::DWORD = 2;
pub const IMAGE_DEBUG_TYPE_FPO: ::DWORD = 3;
pub const IMAGE_DEBUG_TYPE_MISC: ::DWORD = 4;
pub const IMAGE_DEBUG_TYPE_EXCEPTION: ::DWORD = 5;
pub const IMAGE_DEBUG_TYPE_FIXUP: ::DWORD = 6;
pub const IMAGE_DEBUG_TYPE_OMAP_TO_SRC: ::DWORD = 7;
pub const IMAGE_DEBUG_TYPE_OMAP_FROM_SRC: ::DWORD = 8;
pub const IMAGE_DEBUG_TYPE_BORLAND: ::DWORD = 9;
pub const IMAGE_DEBUG_TYPE_RESERVED10: ::DWORD = 10;
pub const IMAGE_DEBUG_TYPE_CLSID: ::DWORD = 11;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IMAGE_COFF_SYMBOLS_HEADER {
    pub NumberOfSymbols: ::DWORD,
    pub LvaToFirstSymbol: ::DWORD,
    pub NumberOfLinenumbers: ::DWORD,
    pub LvaToFirstLinenumber: ::DWORD,
    pub RvaToFirstByteOfCode: ::DWORD,
    pub RvaToLastByteOfCode: ::DWORD,
    pub RvaToFirstByteOfData: ::DWORD,
    pub RvaToLastByteOfData: ::DWORD,
}
pub type PIMAGE_COFF_SYMBOLS_HEADER = *mut IMAGE_COFF_SYMBOLS_HEADER;
pub const FRAME_FPO: ::WORD = 0;
pub const FRAME_TRAP: ::WORD = 1;
pub const FRAME_TSS: ::WORD = 2;
pub const FRAME_NONFPO: ::WORD = 3;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct FPO_DATA {
    pub ulOffStart: ::DWORD,
    pub cbProcSize: ::DWORD,
    pub cdwLocals: ::DWORD,
    pub cdwParams: ::WORD,
    pub bitfield: ::WORD,
}
pub type PFPO_DATA = *mut FPO_DATA;
pub const SIZEOF_RFPO_DATA: usize = 16;
pub const IMAGE_DEBUG_MISC_EXENAME: ::DWORD = 1;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IMAGE_DEBUG_MISC {
    pub DataType: ::DWORD,
    pub Length: ::DWORD,
    pub Unicode: ::BOOLEAN,
    pub Reserved: [::BYTE; 3],
    pub Data: [::BYTE; 0],
}
pub type PIMAGE_DEBUG_MISC = *mut IMAGE_DEBUG_MISC;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IMAGE_FUNCTION_ENTRY {
    pub StartingAddress: ::DWORD,
    pub EndingAddress: ::DWORD,
    pub EndOfPrologue: ::DWORD,
}
pub type PIMAGE_FUNCTION_ENTRY = *mut IMAGE_FUNCTION_ENTRY;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IMAGE_FUNCTION_ENTRY64 {
    pub StartingAddress: ::ULONGLONG,
    pub EndingAddress: ::ULONGLONG,
    pub EndOfPrologueOrUnwindInfoAddress: ::ULONGLONG,
}
pub type PIMAGE_FUNCTION_ENTRY64 = *mut IMAGE_FUNCTION_ENTRY64;
//18145
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct RTL_CRITICAL_SECTION_DEBUG {
    pub Type: ::WORD,
    pub CreatorBackTraceIndex: ::WORD,
    pub CriticalSection: *mut ::RTL_CRITICAL_SECTION,
    pub ProcessLocksList: ::LIST_ENTRY,
    pub EntryCount: ::DWORD,
    pub ContentionCount: ::DWORD,
    pub Flags: ::DWORD,
    pub CreatorBackTraceIndexHigh: ::WORD,
    pub SpareWORD: ::WORD,
}
pub type PRTL_CRITICAL_SECTION_DEBUG = *mut RTL_CRITICAL_SECTION_DEBUG;
pub type RTL_RESOURCE_DEBUG = RTL_CRITICAL_SECTION_DEBUG;
pub type PRTL_RESOURCE_DEBUG = *mut RTL_CRITICAL_SECTION_DEBUG;
pub const RTL_CRITSECT_TYPE: ::WORD = 0;
pub const RTL_RESOURCE_TYPE: ::WORD = 1;
pub const RTL_CRITICAL_SECTION_FLAG_NO_DEBUG_INFO: ::ULONG_PTR = 0x01000000;
pub const RTL_CRITICAL_SECTION_FLAG_DYNAMIC_SPIN: ::ULONG_PTR = 0x02000000;
pub const RTL_CRITICAL_SECTION_FLAG_STATIC_INIT: ::ULONG_PTR = 0x04000000;
pub const RTL_CRITICAL_SECTION_FLAG_RESOURCE_TYPE: ::ULONG_PTR = 0x08000000;
pub const RTL_CRITICAL_SECTION_FLAG_FORCE_DEBUG_INFO: ::ULONG_PTR = 0x10000000;
pub const RTL_CRITICAL_SECTION_ALL_FLAG_BITS: ::ULONG_PTR = 0xFF000000;
pub const RTL_CRITICAL_SECTION_FLAG_RESERVED: ::ULONG_PTR = RTL_CRITICAL_SECTION_ALL_FLAG_BITS & !(RTL_CRITICAL_SECTION_FLAG_NO_DEBUG_INFO | RTL_CRITICAL_SECTION_FLAG_DYNAMIC_SPIN | RTL_CRITICAL_SECTION_FLAG_STATIC_INIT | RTL_CRITICAL_SECTION_FLAG_RESOURCE_TYPE | RTL_CRITICAL_SECTION_FLAG_FORCE_DEBUG_INFO);
pub const RTL_CRITICAL_SECTION_DEBUG_FLAG_STATIC_INIT: ::DWORD = 0x00000001;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct RTL_CRITICAL_SECTION {
    pub DebugInfo: ::PRTL_CRITICAL_SECTION_DEBUG,
    pub LockCount: ::LONG,
    pub RecursionCount: ::LONG,
    pub OwningThread: ::HANDLE,
    pub LockSemaphore: ::HANDLE,
    pub SpinCount: ::ULONG_PTR,
}
pub type PRTL_CRITICAL_SECTION = *mut RTL_CRITICAL_SECTION;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct RTL_SRWLOCK {
    pub Ptr: ::PVOID,
}
pub type PRTL_SRWLOCK = *mut RTL_SRWLOCK;
pub const RTL_SRWLOCK_INIT: RTL_SRWLOCK = RTL_SRWLOCK { Ptr: 0 as PVOID };
//18204
pub type PAPCFUNC = Option<unsafe extern "system" fn(Parameter: ::ULONG_PTR)>;
pub type PVECTORED_EXCEPTION_HANDLER = Option<unsafe extern "system" fn(
    ExceptionInfo: *mut EXCEPTION_POINTERS,
) -> ::LONG>;
#[repr(i32)] #[derive(Clone, Copy, Debug)] #[allow(unused_qualifications)]
pub enum HEAP_INFORMATION_CLASS {
    HeapCompatibilityInformation = 0,
    HeapEnableTerminationOnCorruption = 1,
    HeapOptimizeResources = 3,
}
pub use self::HEAP_INFORMATION_CLASS::*;
pub const HEAP_OPTIMIZE_RESOURCES_CURRENT_VERSION: ::DWORD = 1;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HEAP_OPTIMIZE_RESOURCES_INFORMATION {
    Version: ::DWORD,
    Flags: ::DWORD,
}
pub type PHEAP_OPTIMIZE_RESOURCES_INFORMATION = *mut HEAP_OPTIMIZE_RESOURCES_INFORMATION;
pub const WT_EXECUTEDEFAULT: ::ULONG = 0x00000000;
pub const WT_EXECUTEINIOTHREAD: ::ULONG = 0x00000001;
pub const WT_EXECUTEINUITHREAD: ::ULONG = 0x00000002;
pub const WT_EXECUTEINWAITTHREAD: ::ULONG = 0x00000004;
pub const WT_EXECUTEONLYONCE: ::ULONG = 0x00000008;
pub const WT_EXECUTEINTIMERTHREAD: ::ULONG = 0x00000020;
pub const WT_EXECUTELONGFUNCTION: ::ULONG = 0x00000010;
pub const WT_EXECUTEINPERSISTENTIOTHREAD: ::ULONG = 0x00000040;
pub const WT_EXECUTEINPERSISTENTTHREAD: ::ULONG = 0x00000080;
pub const WT_TRANSFER_IMPERSONATION: ::ULONG = 0x00000100;
pub type WAITORTIMERCALLBACKFUNC = Option<unsafe extern "system" fn(::PVOID, ::BOOLEAN)>;
pub type WORKERCALLBACKFUNC = Option<unsafe extern "system" fn(::PVOID)>;
pub type APC_CALLBACK_FUNCTION = Option<unsafe extern "system" fn(::DWORD, ::PVOID, ::PVOID)>;
pub type WAITORTIMERCALLBACK = WAITORTIMERCALLBACKFUNC;
pub type PFLS_CALLBACK_FUNCTION = Option<unsafe extern "system" fn(lpFlsData: ::PVOID)>;
pub type PSECURE_MEMORY_CACHE_CALLBACK = Option<unsafe extern "system" fn(
    Addr: ::PVOID, Range: ::SIZE_T,
) -> ::BOOLEAN>;
pub const WT_EXECUTEINLONGTHREAD: ::ULONG = 0x00000010;
pub const WT_EXECUTEDELETEWAIT: ::ULONG = 0x00000008;
//18570
pub const KEY_QUERY_VALUE: ::REGSAM = 0x0001;
pub const KEY_SET_VALUE: ::REGSAM = 0x0002;
pub const KEY_CREATE_SUB_KEY: ::REGSAM = 0x0004;
pub const KEY_ENUMERATE_SUB_KEYS: ::REGSAM = 0x0008;
pub const KEY_NOTIFY: ::REGSAM = 0x0010;
pub const KEY_CREATE_LINK: ::REGSAM = 0x0020;
pub const KEY_WOW64_32KEY: ::REGSAM = 0x0200;
pub const KEY_WOW64_64KEY: ::REGSAM = 0x0100;
pub const KEY_WOW64_RES: ::REGSAM = 0x0300;

pub const KEY_READ: ::REGSAM = (
        STANDARD_RIGHTS_READ |
        KEY_QUERY_VALUE |
        KEY_ENUMERATE_SUB_KEYS |
        KEY_NOTIFY
    ) & (!SYNCHRONIZE);
pub const KEY_WRITE: ::REGSAM = (STANDARD_RIGHTS_WRITE | KEY_SET_VALUE | KEY_CREATE_SUB_KEY) & (!SYNCHRONIZE);
pub const KEY_EXECUTE: ::REGSAM = KEY_READ & (!SYNCHRONIZE);
pub const KEY_ALL_ACCESS: ::REGSAM = (
        STANDARD_RIGHTS_ALL |
        KEY_QUERY_VALUE |
        KEY_SET_VALUE |
        KEY_CREATE_SUB_KEY |
        KEY_ENUMERATE_SUB_KEYS |
        KEY_NOTIFY |
        KEY_CREATE_LINK
    ) & (!SYNCHRONIZE);

pub const REG_CREATED_NEW_KEY: ::DWORD = 0x00000001;
pub const REG_OPENED_EXISTING_KEY: ::DWORD = 0x00000002;

pub const REG_NOTIFY_CHANGE_NAME: ::DWORD = 0x00000001;
pub const REG_NOTIFY_CHANGE_ATTRIBUTES: ::DWORD = 0x00000002;
pub const REG_NOTIFY_CHANGE_LAST_SET: ::DWORD = 0x00000004;
pub const REG_NOTIFY_CHANGE_SECURITY: ::DWORD = 0x00000008;

pub const REG_LEGAL_CHANGE_FILTER: ::DWORD = REG_NOTIFY_CHANGE_NAME |
    REG_NOTIFY_CHANGE_ATTRIBUTES |
    REG_NOTIFY_CHANGE_LAST_SET |
    REG_NOTIFY_CHANGE_SECURITY;

pub const REG_NOTIFY_THREAD_AGNOSTIC: ::DWORD = 0x10000000; //supported only on Windows 8 and later

pub const REG_OPTION_RESERVED: ::DWORD = 0x00000000;
pub const REG_OPTION_NON_VOLATILE: ::DWORD = 0x00000000;
pub const REG_OPTION_VOLATILE: ::DWORD = 0x00000001;
pub const REG_OPTION_CREATE_LINK: ::DWORD = 0x00000002;
pub const REG_OPTION_BACKUP_RESTORE: ::DWORD = 0x00000004;
pub const REG_OPTION_OPEN_LINK: ::DWORD = 0x00000008;

pub const REG_NONE: ::DWORD = 0;
pub const REG_SZ: ::DWORD = 1;
pub const REG_EXPAND_SZ: ::DWORD = 2;
pub const REG_BINARY: ::DWORD = 3;
pub const REG_DWORD: ::DWORD = 4;
pub const REG_DWORD_LITTLE_ENDIAN: ::DWORD = 4;
pub const REG_DWORD_BIG_ENDIAN: ::DWORD = 5;
pub const REG_LINK: ::DWORD = 6;
pub const REG_MULTI_SZ: ::DWORD = 7;
pub const REG_RESOURCE_LIST: ::DWORD = 8;
pub const REG_FULL_RESOURCE_DESCRIPTOR: ::DWORD = 9;
pub const REG_RESOURCE_REQUIREMENTS_LIST: ::DWORD = 10;
pub const REG_QWORD: ::DWORD = 11;
pub const REG_QWORD_LITTLE_ENDIAN: ::DWORD = 11;
//18720
pub const SERVICE_KERNEL_DRIVER: ::DWORD = 0x00000001;
pub const SERVICE_FILE_SYSTEM_DRIVER: ::DWORD = 0x00000002;
pub const SERVICE_ADAPTER: ::DWORD = 0x00000004;
pub const SERVICE_RECOGNIZER_DRIVER: ::DWORD = 0x00000008;
pub const SERVICE_DRIVER: ::DWORD = SERVICE_KERNEL_DRIVER | SERVICE_FILE_SYSTEM_DRIVER
    | SERVICE_RECOGNIZER_DRIVER;
pub const SERVICE_WIN32_OWN_PROCESS: ::DWORD = 0x00000010;
pub const SERVICE_WIN32_SHARE_PROCESS: ::DWORD = 0x00000020;
pub const SERVICE_WIN32: ::DWORD = SERVICE_WIN32_OWN_PROCESS | SERVICE_WIN32_SHARE_PROCESS;
pub const SERVICE_INTERACTIVE_PROCESS: ::DWORD = 0x00000100;
pub const SERVICE_TYPE_ALL: ::DWORD = SERVICE_WIN32 | SERVICE_ADAPTER | SERVICE_DRIVER
    | SERVICE_INTERACTIVE_PROCESS;

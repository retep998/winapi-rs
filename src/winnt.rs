// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! This module defines the 32-Bit Windows types and constants that are defined by NT, but exposed
//! through the Win32 API.
pub type NTSTATUS = ::LONG;
//33
pub const ANYSIZE_ARRAY: usize = 1;
//382
pub type VOID = ::c_void;
pub type CHAR = ::c_char;
pub type SHORT = ::c_short;
pub type LONG = ::c_long;
// pub type INT = ::c_int; // Already defined by minwindef.h
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
// FIXME - Align 16
#[cfg(target_arch = "x86_64")] #[repr(C)] #[derive(Clone, Copy, Debug)]
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
    pub Header: [::M128A; 2],
    pub Legacy: [::M128A; 8],
    pub Xmm0: ::M128A,
    pub Xmm1: ::M128A,
    pub Xmm2: ::M128A,
    pub Xmm3: ::M128A,
    pub Xmm4: ::M128A,
    pub Xmm5: ::M128A,
    pub Xmm6: ::M128A,
    pub Xmm7: ::M128A,
    pub Xmm8: ::M128A,
    pub Xmm9: ::M128A,
    pub Xmm10: ::M128A,
    pub Xmm11: ::M128A,
    pub Xmm12: ::M128A,
    pub Xmm13: ::M128A,
    pub Xmm14: ::M128A,
    pub Xmm15: ::M128A,
    pub VectorRegister: [::M128A; 26],
    pub VectorControl: ::DWORD64,
    pub DebugControl: ::DWORD64,
    pub LastBranchToRip: ::DWORD64,
    pub LastBranchFromRip: ::DWORD64,
    pub LastExceptionToRip: ::DWORD64,
    pub LastExceptionFromRip: ::DWORD64,
}
pub type PCONTEXT = *mut CONTEXT;
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
//9100
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
#[repr(C)] #[derive(Clone, Copy, Debug)]
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
#[repr(C)] #[derive(Clone, Copy, Debug)]
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
//18195
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct RTL_SRWLOCK {
    pub Ptr: ::PVOID,
}
pub type PRTL_SRWLOCK = *mut RTL_SRWLOCK;
//18204
pub type PAPCFUNC = Option<unsafe extern "system" fn(Parameter: ::ULONG_PTR)>;
pub type PVECTORED_EXCEPTION_HANDLER = Option<unsafe extern "system" fn(
    ExceptionInfo: *mut EXCEPTION_POINTERS,
) -> ::LONG>;
//18264
pub type PSECURE_MEMORY_CACHE_CALLBACK = Option<unsafe extern "system" fn(
    Addr: ::PVOID, Range: ::SIZE_T,
) -> ::BOOLEAN>;
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

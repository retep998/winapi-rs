// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! This module defines the 32-Bit Windows types and constants that are defined by NT, but exposed through the Win32 API.
pub const ANYSIZE_ARRAY: usize = 1;
#[repr(C)] #[derive(Copy)] pub struct CONTEXT {
    P1Home: ::DWORD64,
    P2Home: ::DWORD64,
    P3Home: ::DWORD64,
    P4Home: ::DWORD64,
    P5Home: ::DWORD64,
    P6Home: ::DWORD64,
    ContextFlags: ::DWORD,
    MxCsr: ::DWORD,
    SegCs: ::WORD,
    SegDs: ::WORD,
    SegEs: ::WORD,
    SegFs: ::WORD,
    SegGs: ::WORD,
    SegSs: ::WORD,
    EFlags: ::DWORD,
    Dr0: ::DWORD64,
    Dr1: ::DWORD64,
    Dr2: ::DWORD64,
    Dr3: ::DWORD64,
    Dr6: ::DWORD64,
    Dr7: ::DWORD64,
    Rax: ::DWORD64,
    Rcx: ::DWORD64,
    Rdx: ::DWORD64,
    Rbx: ::DWORD64,
    Rsp: ::DWORD64,
    Rbp: ::DWORD64,
    Rsi: ::DWORD64,
    Rdi: ::DWORD64,
    R8: ::DWORD64,
    R9: ::DWORD64,
    R10: ::DWORD64,
    R11: ::DWORD64,
    R12: ::DWORD64,
    R13: ::DWORD64,
    R14: ::DWORD64,
    R15: ::DWORD64,
    Rip: ::DWORD64,
    Header: [::M128A; 2],
    Legacy: [::M128A; 8],
    Xmm0: ::M128A,
    Xmm1: ::M128A,
    Xmm2: ::M128A,
    Xmm3: ::M128A,
    Xmm4: ::M128A,
    Xmm5: ::M128A,
    Xmm6: ::M128A,
    Xmm7: ::M128A,
    Xmm8: ::M128A,
    Xmm9: ::M128A,
    Xmm10: ::M128A,
    Xmm11: ::M128A,
    Xmm12: ::M128A,
    Xmm13: ::M128A,
    Xmm14: ::M128A,
    Xmm15: ::M128A,
    VectorRegister: [::M128A; 26],
    VectorControl: ::DWORD64,
    DebugControl: ::DWORD64,
    LastBranchToRip: ::DWORD64,
    LastBranchFromRip: ::DWORD64,
    LastExceptionToRip: ::DWORD64,
    LastExceptionFromRip: ::DWORD64,
}
pub type PCONTEXT = *mut CONTEXT;
pub const ACL_REVISION: ::BYTE = 2;
pub const ACL_REVISION_DS: ::BYTE = 4;
pub const ACL_REVISION1: ::BYTE = 1;
pub const MIN_ACL_REVISION: ::BYTE = ACL_REVISION2;
pub const ACL_REVISION2: ::BYTE = 2;
pub const ACL_REVISION3: ::BYTE = 3;
pub const ACL_REVISION4: ::BYTE = 4;
pub const MAX_ACL_REVISION: ::BYTE = ACL_REVISION4;
#[repr(C)] #[derive(Copy)] pub struct ACL {
    pub AclRevision: ::BYTE,
    pub Sbz1: ::BYTE,
    pub AclSize: ::WORD,
    pub AceCount: ::WORD,
    pub Sbz2: ::WORD,
}
pub type PACL = *mut ACL;
pub const EXCEPTION_MAXIMUM_PARAMETERS: usize = 15;
#[repr(C)] #[derive(Copy)] pub struct EXCEPTION_RECORD {
    ExceptionCode: ::DWORD,
    ExceptionFlags: ::DWORD,
    ExceptionRecord: *mut EXCEPTION_RECORD,
    ExceptionAddress: ::PVOID,
    NumberParameters: ::DWORD,
    ExceptionInformation: [::ULONG_PTR; EXCEPTION_MAXIMUM_PARAMETERS],
}
pub type PEXCEPTION_RECORD = *mut EXCEPTION_RECORD;
#[repr(C)] #[derive(Copy)] pub struct EXCEPTION_POINTERS {
    pub ExceptionRecord: PEXCEPTION_RECORD,
    pub ContextRecord: PCONTEXT,
}
pub type PEXCEPTION_POINTERS = *mut EXCEPTION_POINTERS;
pub type PACCESS_TOKEN = ::PVOID;
pub type PSECURITY_DESCRIPTOR = ::PVOID;
pub type PSID = ::PVOID;
pub type PCLAIMS_BLOB = ::PVOID;
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_INVALID: ::WORD = 0x00;
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_INT64: ::WORD = 0x01;
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_UINT64: ::WORD = 0x02;
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_STRING: ::WORD = 0x03;
#[repr(C)] #[derive(Copy)] pub struct CLAIM_SECURITY_ATTRIBUTE_FQBN_VALUE {
    pub Version: ::DWORD64,
    pub Name: ::PWSTR,
}
pub type PCLAIM_SECURITY_ATTRIBUTE_FQBN_VALUE = *mut CLAIM_SECURITY_ATTRIBUTE_FQBN_VALUE;
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_FQBN: ::WORD = 0x04;
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_SID: ::WORD = 0x05;
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_BOOLEAN: ::WORD = 0x06;
#[repr(C)] #[derive(Copy)] pub struct CLAIM_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE {
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
#[repr(C)] #[derive(Copy)] pub struct CLAIM_SECURITY_ATTRIBUTE_V1 {
    pub Name: ::PWSTR,
    pub ValueType: ::WORD,
    pub Reserved: ::WORD,
    pub Flags: ::DWORD,
    pub ValueCount: ::DWORD,
    // Put data here
}
pub type PCLAIM_SECURITY_ATTRIBUTE_V1 = *mut CLAIM_SECURITY_ATTRIBUTE_V1;
#[repr(C)] #[derive(Copy)] pub struct CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1 {
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
#[repr(C)] #[derive(Copy)] pub struct CLAIM_SECURITY_ATTRIBUTES_INFORMATION {
    pub Version: ::WORD,
    pub Reserved: ::WORD,
    pub AttributeCount: ::DWORD,
    pub pAttributeV1: PCLAIM_SECURITY_ATTRIBUTE_V1,
}
pub type PCLAIM_SECURITY_ATTRIBUTES_INFORMATION = *mut CLAIM_SECURITY_ATTRIBUTES_INFORMATION;
pub type PVECTORED_EXCEPTION_HANDLER = extern "system" fn(ExceptionInfo: *mut EXCEPTION_POINTERS) -> ::LONG;
pub type PSECURE_MEMORY_CACHE_CALLBACK = extern "system" fn(
    Addr: ::PVOID, Range: ::SIZE_T,
) -> ::BOOLEAN;
#[repr(C)] #[derive(Copy)] pub struct RTL_SRWLOCK {
    pub Ptr: ::PVOID,
}
pub type PRTL_SRWLOCK = *mut RTL_SRWLOCK;

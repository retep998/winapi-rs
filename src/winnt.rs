// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! This module defines the 32-Bit Windows types and constants that are defined by NT, but exposed
//! through the Win32 API.
//33
pub const ANYSIZE_ARRAY: usize = 1;
//382
pub type VOID = ::c_void;
pub type CHAR = ::c_char;
pub type SHORT = ::c_short;
pub type LONG = ::c_long;
// pub type INT = ::c_int; // Already defined by minwindef.h
//3563
// FIXME - Align 16
#[repr(C)] #[derive(Copy)] pub struct CONTEXT {
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
    pub ExceptionCode: ::DWORD,
    pub ExceptionFlags: ::DWORD,
    pub ExceptionRecord: *mut EXCEPTION_RECORD,
    pub ExceptionAddress: ::PVOID,
    pub NumberParameters: ::DWORD,
    pub ExceptionInformation: [::ULONG_PTR; EXCEPTION_MAXIMUM_PARAMETERS],
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
pub type PVECTORED_EXCEPTION_HANDLER = unsafe extern "system" fn(
    ExceptionInfo: *mut EXCEPTION_POINTERS,
) -> ::LONG;
pub type PSECURE_MEMORY_CACHE_CALLBACK = unsafe extern "system" fn(
    Addr: ::PVOID, Range: ::SIZE_T,
) -> ::BOOLEAN;
#[repr(C)] #[derive(Copy)] pub struct RTL_SRWLOCK {
    pub Ptr: ::PVOID,
}
pub type PRTL_SRWLOCK = *mut RTL_SRWLOCK;

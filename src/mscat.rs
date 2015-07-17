// Copyright © 2015, skdltmxn
// Licensed under the MIT License <LICENSE.md>
//! Microsoft Internet Security Catalog API Prototypes and Definitions
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct CRYPTCATSTORE {
    pub cbStruct: ::DWORD,
    pub dwPublicVersion: ::DWORD,
    pub pwszP7File: ::LPWSTR,
    pub hProv: ::HCRYPTPROV,
    pub dwEncodingType: ::DWORD,
    pub fdwStoreFlags: ::DWORD,
    pub hReserved: ::HANDLE,
    pub hAttrs: ::HANDLE,
    pub hCryptMsg: ::HCRYPTMSG,
    pub hSorted: ::HANDLE,
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct CRYPTCATMEMBER{
    pub cbStruct: ::DWORD,
    pub pwszReferenceTag: ::LPWSTR,
    pub pwszFileName: ::LPWSTR,
    pub gSubjectType: ::GUID,
    pub fdwMemberFlags: ::DWORD,
    pub pIndirectData: *mut ::SIP_INDIRECT_DATA,
    pub dwCertVersion: ::DWORD,
    pub dwReserved: ::DWORD,
    pub hReserved: ::HANDLE,
    pub sEncodedIndirectData: ::CRYPT_ATTR_BLOB,
    pub sEncodedMemberInfo: ::CRYPT_ATTR_BLOB,
}

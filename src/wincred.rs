// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! Authentication API Prototypes and Definitions
pub const CRED_MAX_STRING_LENGTH: ::DWORD = 256;
pub const CRED_MAX_USERNAME_LENGTH: ::DWORD = 256+1+256;
pub const CRED_MAX_GENERIC_TARGET_NAME_LENGTH: ::DWORD = 32767;
pub const CRED_MAX_DOMAIN_TARGET_NAME_LENGTH: ::DWORD = 256+1+80;
pub const CRED_MAX_TARGETNAME_NAMESPACE_LENGTH: ::DWORD = 256;
pub const CRED_MAX_TARGETNAME_ATTRIBUTE_LENGTH: ::DWORD = 256;
pub const CRED_MAX_VALUE_SIZE: ::DWORD = 256;
pub const CRED_MAX_ATTRIBUTES: ::DWORD = 64;

pub const CRED_TYPE_GENERIC: ::DWORD = 1;
pub const CRED_TYPE_DOMAIN_PASSWORD: ::DWORD = 2;
pub const CRED_TYPE_DOMAIN_CERTIFICATE: ::DWORD = 3;
pub const CRED_TYPE_DOMAIN_VISIBLE_PASSWORD: ::DWORD = 4;
pub const CRED_TYPE_GENERIC_CERTIFICATE: ::DWORD = 5;
pub const CRED_TYPE_DOMAIN_EXTENDED: ::DWORD = 6;
pub const CRED_TYPE_MAXIMUM: ::DWORD = 7;
pub const CRED_TYPE_MAXIMUM_EX: ::DWORD = CRED_TYPE_MAXIMUM+1000;

pub const CRED_MAX_CREDENTIAL_BLOB_SIZE: ::DWORD = 5*512;

pub const CRED_PERSIST_NONE: ::DWORD = 0;
pub const CRED_PERSIST_SESSION: ::DWORD = 1;
pub const CRED_PERSIST_LOCAL_MACHINE: ::DWORD = 2;
pub const CRED_PERSIST_ENTERPRISE: ::DWORD = 3;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct CREDENTIAL_ATTRIBUTEA {
    pub Keyword: ::LPSTR,
    pub Flags: ::DWORD,
    pub ValueSize: ::DWORD,
    pub Value: ::LPBYTE,
}
pub type PCREDENTIAL_ATTRIBUTEA = *mut CREDENTIAL_ATTRIBUTEA;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct CREDENTIAL_ATTRIBUTEW {
    pub Keyword: ::LPWSTR,
    pub Flags: ::DWORD,
    pub ValueSize: ::DWORD,
    pub Value: ::LPBYTE,
}
pub type PCREDENTIAL_ATTRIBUTEW = *mut CREDENTIAL_ATTRIBUTEW;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct CREDENTIALA {
    pub Flags: ::DWORD,
    pub Type: ::DWORD,
    pub TargetName: ::LPSTR,
    pub Comment: ::LPSTR,
    pub LastWritten: ::FILETIME,
    pub CredentialBlobSize: ::DWORD,
    pub CredentialBlob: ::LPBYTE,
    pub Persist: ::DWORD,
    pub AttributeCount: ::DWORD,
    pub Attributes: PCREDENTIAL_ATTRIBUTEA,
    pub TargetAlias: ::LPSTR,
    pub UserName: ::LPSTR,
}
pub type PCREDENTIALA = *mut CREDENTIALA;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct CREDENTIALW {
    pub Flags: ::DWORD,
    pub Type: ::DWORD,
    pub TargetName: ::LPWSTR,
    pub Comment: ::LPWSTR,
    pub LastWritten: ::FILETIME,
    pub CredentialBlobSize: ::DWORD,
    pub CredentialBlob: ::LPBYTE,
    pub Persist: ::DWORD,
    pub AttributeCount: ::DWORD,
    pub Attributes: PCREDENTIAL_ATTRIBUTEW,
    pub TargetAlias: ::LPWSTR,
    pub UserName: ::LPWSTR,
}
pub type PCREDENTIALW = *mut CREDENTIALW;

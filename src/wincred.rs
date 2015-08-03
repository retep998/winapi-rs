// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! Authentication API Prototypes and Definitions
pub const NERR_BASE: ::DWORD = 2100;
pub const NERR_PasswordExpired: ::DWORD = NERR_BASE+142;

pub const CRED_MAX_STRING_LENGTH: ::DWORD = 256;
pub const CRED_MAX_USERNAME_LENGTH: ::DWORD = 256+1+256;
pub const CRED_MAX_GENERIC_TARGET_NAME_LENGTH: ::DWORD = 32767;
pub const CRED_MAX_DOMAIN_TARGET_NAME_LENGTH: ::DWORD = 256+1+80;
pub const CRED_MAX_TARGETNAME_NAMESPACE_LENGTH: ::DWORD = 256;
pub const CRED_MAX_TARGETNAME_ATTRIBUTE_LENGTH: ::DWORD = 256;
pub const CRED_MAX_VALUE_SIZE: ::DWORD = 256;
pub const CRED_MAX_ATTRIBUTES: ::DWORD = 64;

pub const CRED_LOGON_TYPES_MASK: ::DWORD = 0xF000;

pub const CRED_FLAGS_PASSWORD_FOR_CERT: ::DWORD = 0x0001;
pub const CRED_FLAGS_PROMPT_NOW: ::DWORD = 0x0002;
pub const CRED_FLAGS_USERNAME_TARGET: ::DWORD = 0x0004;
pub const CRED_FLAGS_OWF_CRED_BLOB: ::DWORD = 0x0008;
pub const CRED_FLAGS_REQUIRE_CONFIRMATION: ::DWORD = 0x0010;
pub const CRED_FLAGS_WILDCARD_MATCH: ::DWORD = 0x0020;
pub const CRED_FLAGS_VALID_FLAGS: ::DWORD = 0xF03F;
pub const CRED_FLAGS_VALID_INPUT_FLAGS: ::DWORD = 0xF01F;

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

pub const CRED_TI_SERVER_FORMAT_UNKNOWN: ::ULONG = 0x0001;
pub const CRED_TI_DOMAIN_FORMAT_UNKNOWN: ::ULONG = 0x0002;
pub const CRED_TI_ONLY_PASSWORD_REQUIRED: ::ULONG = 0x0004;
pub const CRED_TI_USERNAME_TARGET: ::ULONG = 0x0008;
pub const CRED_TI_CREATE_EXPLICIT_CRED: ::ULONG = 0x0010;
pub const CRED_TI_WORKGROUP_MEMBER: ::ULONG = 0x0020;
pub const CRED_TI_VALID_FLAGS: ::ULONG = 0xF07F;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct CREDENTIAL_TARGET_INFORMATIONA {
    pub TargetName: ::LPSTR,
    pub NetbiosServerName: ::LPSTR,
    pub DnsServerName: ::LPSTR,
    pub NetbiosDomainName: ::LPSTR,
    pub DnsDomainName: ::LPSTR,
    pub DnsTreeName: ::LPSTR,
    pub PackageName: ::LPSTR,
    pub Flags: ::ULONG,
    pub CredTypeCount: ::DWORD,
    pub CredTypes: ::LPDWORD,
}
pub type PCREDENTIAL_TARGET_INFORMATIONA = *mut CREDENTIAL_TARGET_INFORMATIONA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct CREDENTIAL_TARGET_INFORMATIONW {
    pub TargetName: ::LPWSTR,
    pub NetbiosServerName: ::LPWSTR,
    pub DnsServerName: ::LPWSTR,
    pub NetbiosDomainName: ::LPWSTR,
    pub DnsDomainName: ::LPWSTR,
    pub DnsTreeName: ::LPWSTR,
    pub PackageName: ::LPWSTR,
    pub Flags: ::ULONG,
    pub CredTypeCount: ::DWORD,
    pub CredTypes: ::LPDWORD,
}
pub type PCREDENTIAL_TARGET_INFORMATIONW = *mut CREDENTIAL_TARGET_INFORMATIONW;

pub const CERT_HASH_LENGTH: usize = 20;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct CERT_CREDENTIAL_INFO {
    pub cbSize: ::ULONG,
    pub rgbHashOfCert: [::UCHAR; CERT_HASH_LENGTH],
}
pub type PCERT_CREDENTIAL_INFO = *mut CERT_CREDENTIAL_INFO;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct USERNAME_TARGET_CREDENTIAL_INFO {
    pub UserName: ::LPWSTR,
}
pub type PUSERNAME_TARGET_CREDENTIAL_INFO = *mut USERNAME_TARGET_CREDENTIAL_INFO;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct BINARY_BLOB_CREDENTIAL_INFO {
    pub cbBlob: ::ULONG,
    pub pbBlob: ::LPBYTE,
}
pub type PBINARY_BLOB_CREDENTIAL_INFO = *mut BINARY_BLOB_CREDENTIAL_INFO;

#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum CRED_MARSHAL_TYPE {
    CertCredential = 1,
    UsernameTargetCredential,
    BinaryBlobCredential,
    UsernameForPackedCredentials,
}
pub type PCRED_MARSHAL_TYPE = *mut CRED_MARSHAL_TYPE;

#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum CRED_PROTECTION_TYPE {
    CredUnprotected,
    CredUserProtection,
    CredTrustedProtection,
}
pub type PCRED_PROTECTION_TYPE = *mut CRED_PROTECTION_TYPE;

pub const CRED_PACK_PROTECTED_CREDENTIALS: ::DWORD = 0x1;
pub const CRED_PACK_WOW_BUFFER: ::DWORD = 0x2;
pub const CRED_PACK_GENERIC_CREDENTIALS: ::DWORD = 0x4;
pub const CRED_PACK_ID_PROVIDER_CREDENTIALS: ::DWORD = 0x8;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct CREDUI_INFOA {
    pub cbSize: ::DWORD,
    pub hwndParent: ::HWND,
    pub pszMessageText: ::PCSTR,
    pub pszCaptionText: ::PCSTR,
    pub hbmBanner: ::HBITMAP,
}
pub type PCREDUI_INFOA = *mut CREDUI_INFOA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct CREDUI_INFOW {
    pub cbSize: ::DWORD,
    pub hwndParent: ::HWND,
    pub pszMessageText: ::PCWSTR,
    pub pszCaptionText: ::PCWSTR,
    pub hbmBanner: ::HBITMAP,
}
pub type PCREDUI_INFOW = *mut CREDUI_INFOW;

pub const CREDUI_MAX_MESSAGE_LENGTH: ::DWORD = 1024;
pub const CREDUI_MAX_CAPTION_LENGTH: ::DWORD = 128;
pub const CREDUI_MAX_GENERIC_TARGET_LENGTH: ::DWORD = CRED_MAX_GENERIC_TARGET_NAME_LENGTH;
pub const CREDUI_MAX_DOMAIN_TARGET_LENGTH: ::DWORD = CRED_MAX_DOMAIN_TARGET_NAME_LENGTH;

pub const CREDUI_MAX_USERNAME_LENGTH: ::DWORD = CRED_MAX_USERNAME_LENGTH;
pub const CREDUI_MAX_PASSWORD_LENGTH: ::DWORD = 512 / 2;

pub const CREDUI_FLAGS_INCORRECT_PASSWORD: ::DWORD = 0x00001;
pub const CREDUI_FLAGS_DO_NOT_PERSIST: ::DWORD = 0x00002;
pub const CREDUI_FLAGS_REQUEST_ADMINISTRATOR: ::DWORD = 0x00004;
pub const CREDUI_FLAGS_EXCLUDE_CERTIFICATES: ::DWORD = 0x00008;
pub const CREDUI_FLAGS_REQUIRE_CERTIFICATE: ::DWORD = 0x00010;
pub const CREDUI_FLAGS_SHOW_SAVE_CHECK_BOX: ::DWORD = 0x00040;
pub const CREDUI_FLAGS_ALWAYS_SHOW_UI: ::DWORD = 0x00080;
pub const CREDUI_FLAGS_REQUIRE_SMARTCARD: ::DWORD = 0x00100;
pub const CREDUI_FLAGS_PASSWORD_ONLY_OK: ::DWORD = 0x00200;
pub const CREDUI_FLAGS_VALIDATE_USERNAME: ::DWORD = 0x00400;
pub const CREDUI_FLAGS_COMPLETE_USERNAME: ::DWORD = 0x00800;
pub const CREDUI_FLAGS_PERSIST: ::DWORD = 0x01000;
pub const CREDUI_FLAGS_SERVER_CREDENTIAL: ::DWORD = 0x04000;
pub const CREDUI_FLAGS_EXPECT_CONFIRMATION: ::DWORD = 0x20000;
pub const CREDUI_FLAGS_GENERIC_CREDENTIALS: ::DWORD = 0x40000;
pub const CREDUI_FLAGS_USERNAME_TARGET_CREDENTIALS: ::DWORD = 0x80000;
pub const CREDUI_FLAGS_KEEP_USERNAME: ::DWORD = 0x100000;
pub const CREDUI_FLAGS_PROMPT_VALID: ::DWORD = CREDUI_FLAGS_INCORRECT_PASSWORD
    | CREDUI_FLAGS_DO_NOT_PERSIST | CREDUI_FLAGS_REQUEST_ADMINISTRATOR
    | CREDUI_FLAGS_EXCLUDE_CERTIFICATES | CREDUI_FLAGS_REQUIRE_CERTIFICATE
    | CREDUI_FLAGS_SHOW_SAVE_CHECK_BOX | CREDUI_FLAGS_ALWAYS_SHOW_UI
    | CREDUI_FLAGS_REQUIRE_SMARTCARD | CREDUI_FLAGS_PASSWORD_ONLY_OK
    | CREDUI_FLAGS_VALIDATE_USERNAME | CREDUI_FLAGS_COMPLETE_USERNAME | CREDUI_FLAGS_PERSIST
    | CREDUI_FLAGS_SERVER_CREDENTIAL | CREDUI_FLAGS_EXPECT_CONFIRMATION
    | CREDUI_FLAGS_GENERIC_CREDENTIALS | CREDUI_FLAGS_USERNAME_TARGET_CREDENTIALS
    | CREDUI_FLAGS_KEEP_USERNAME;

pub const CREDUIWIN_GENERIC: ::DWORD = 0x00000001;
pub const CREDUIWIN_CHECKBOX: ::DWORD = 0x00000002;
pub const CREDUIWIN_AUTHPACKAGE_ONLY: ::DWORD = 0x00000010;
pub const CREDUIWIN_IN_CRED_ONLY: ::DWORD = 0x00000020;
pub const CREDUIWIN_ENUMERATE_ADMINS: ::DWORD = 0x00000100;
pub const CREDUIWIN_ENUMERATE_CURRENT_USER: ::DWORD = 0x00000200;
pub const CREDUIWIN_SECURE_PROMPT: ::DWORD = 0x00001000;
pub const CREDUIWIN_PREPROMPTING: ::DWORD = 0x00002000;
pub const CREDUIWIN_PACK_32_WOW: ::DWORD = 0x10000000;
pub const CREDUIWIN_VALID_FLAGS: ::DWORD = CREDUIWIN_GENERIC | CREDUIWIN_CHECKBOX
    | CREDUIWIN_AUTHPACKAGE_ONLY | CREDUIWIN_IN_CRED_ONLY | CREDUIWIN_ENUMERATE_ADMINS
    | CREDUIWIN_ENUMERATE_CURRENT_USER | CREDUIWIN_SECURE_PROMPT | CREDUIWIN_PREPROMPTING
    | CREDUIWIN_PACK_32_WOW;

pub const CRED_PRESERVE_CREDENTIAL_BLOB: ::DWORD = 0x1;
pub const CRED_ENUMERATE_ALL_CREDENTIALS: ::DWORD = 0x1;
pub const CRED_CACHE_TARGET_INFORMATION: ::DWORD = 0x1;
pub const CRED_ALLOW_NAME_RESOLUTION: ::DWORD = 0x1;

// Copyright © 2015, skdltmxn
// Licensed under the MIT License <LICENSE.md>
//! Types and macros for Subauthentication Packages.
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct UNICODE_STRING {
    pub Length: ::USHORT,
    pub MaximumLength: ::USHORT,
    pub Buffer: ::PWSTR,
}
pub type PUNICODE_STRING = *mut UNICODE_STRING;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct STRING {
    pub Length: ::USHORT,
    pub MaximumLength: ::USHORT,
    pub Buffer: ::PCHAR,
}
pub type PSTRING = *mut STRING;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct OLD_LARGE_INTEGER {
    pub LowPart: ::ULONG,
    pub HighPart: ::LONG,
}
pub type POLD_LARGE_INTEGER = *mut OLD_LARGE_INTEGER;
pub type SAM_HANDLE = ::PVOID;
pub type PSAM_HANDLE = *mut ::PVOID;
pub const USER_ACCOUNT_DISABLED: ::ULONG = 0x00000001;
pub const USER_HOME_DIRECTORY_REQUIRED: ::ULONG = 0x00000002;
pub const USER_PASSWORD_NOT_REQUIRED: ::ULONG = 0x00000004;
pub const USER_TEMP_DUPLICATE_ACCOUNT: ::ULONG = 0x00000008;
pub const USER_NORMAL_ACCOUNT: ::ULONG = 0x00000010;
pub const USER_MNS_LOGON_ACCOUNT: ::ULONG = 0x00000020;
pub const USER_INTERDOMAIN_TRUST_ACCOUNT: ::ULONG = 0x00000040;
pub const USER_WORKSTATION_TRUST_ACCOUNT: ::ULONG = 0x00000080;
pub const USER_SERVER_TRUST_ACCOUNT: ::ULONG = 0x00000100;
pub const USER_DONT_EXPIRE_PASSWORD: ::ULONG = 0x00000200;
pub const USER_ACCOUNT_AUTO_LOCKED: ::ULONG = 0x00000400;
pub const USER_ENCRYPTED_TEXT_PASSWORD_ALLOWED: ::ULONG = 0x00000800;
pub const USER_SMARTCARD_REQUIRED: ::ULONG = 0x00001000;
pub const USER_TRUSTED_FOR_DELEGATION: ::ULONG = 0x00002000;
pub const USER_NOT_DELEGATED: ::ULONG = 0x00004000;
pub const USER_USE_DES_KEY_ONLY: ::ULONG = 0x00008000;
pub const USER_DONT_REQUIRE_PREAUTH: ::ULONG = 0x00010000;
pub const USER_PASSWORD_EXPIRED: ::ULONG = 0x00020000;
pub const USER_TRUSTED_TO_AUTHENTICATE_FOR_DELEGATION: ::ULONG = 0x00040000;
pub const USER_NO_AUTH_DATA_REQUIRED: ::ULONG = 0x00080000;
pub const USER_PARTIAL_SECRETS_ACCOUNT: ::ULONG = 0x00100000;
pub const USER_USE_AES_KEYS: ::ULONG = 0x00200000;
pub const NEXT_FREE_ACCOUNT_CONTROL_BIT: ::ULONG = USER_USE_AES_KEYS << 1;
pub const USER_MACHINE_ACCOUNT_MASK: ::ULONG = USER_INTERDOMAIN_TRUST_ACCOUNT
    | USER_WORKSTATION_TRUST_ACCOUNT | USER_SERVER_TRUST_ACCOUNT;
pub const USER_ACCOUNT_TYPE_MASK: ::ULONG = USER_TEMP_DUPLICATE_ACCOUNT | USER_NORMAL_ACCOUNT
    | USER_MACHINE_ACCOUNT_MASK;
pub const USER_COMPUTED_ACCOUNT_CONTROL_BITS: ::ULONG = USER_ACCOUNT_AUTO_LOCKED
| USER_PASSWORD_EXPIRED;
pub const SAM_DAYS_PER_WEEK: ::USHORT = 7;
pub const SAM_HOURS_PER_WEEK: ::USHORT = 24 * SAM_DAYS_PER_WEEK;
pub const SAM_MINUTES_PER_WEEK: ::USHORT = 60 * SAM_HOURS_PER_WEEK;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct LOGON_HOURS {
    pub UnitsPerWeek: ::USHORT,
    pub LogonHours: ::PUCHAR,
}
pub type PLOGON_HOURS = *mut LOGON_HOURS;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SR_SECURITY_DESCRIPTOR {
    pub Length: ::ULONG,
    pub SecurityDescriptor: ::PUCHAR,
}
pub type PSR_SECURITY_DESCRIPTOR = *mut SR_SECURITY_DESCRIPTOR;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct USER_ALL_INFORMATION {
    pub LastLogon: ::LARGE_INTEGER,
    pub LastLogoff: ::LARGE_INTEGER,
    pub PasswordLastSet: ::LARGE_INTEGER,
    pub AccountExpires: ::LARGE_INTEGER,
    pub PasswordCanChange: ::LARGE_INTEGER,
    pub PasswordMustChange: ::LARGE_INTEGER,
    pub UserName: UNICODE_STRING,
    pub FullName: UNICODE_STRING,
    pub HomeDirectory: UNICODE_STRING,
    pub HomeDirectoryDrive: UNICODE_STRING,
    pub ScriptPath: UNICODE_STRING,
    pub ProfilePath: UNICODE_STRING,
    pub AdminComment: UNICODE_STRING,
    pub WorkStations: UNICODE_STRING,
    pub UserComment: UNICODE_STRING,
    pub Parameters: UNICODE_STRING,
    pub LmPassword: UNICODE_STRING,
    pub NtPassword: UNICODE_STRING,
    pub PrivateData: UNICODE_STRING,
    pub SecurityDescriptor: SR_SECURITY_DESCRIPTOR,
    pub UserId: ::ULONG,
    pub PrimaryGroupId: ::ULONG,
    pub UserAccountControl: ::ULONG,
    pub WhichFields: ::ULONG,
    pub LogonHours: LOGON_HOURS,
    pub BadPasswordCount: ::USHORT,
    pub LogonCount: ::USHORT,
    pub CountryCode: ::USHORT,
    pub CodePage: ::USHORT,
    pub LmPasswordPresent: ::BOOLEAN,
    pub NtPasswordPresent: ::BOOLEAN,
    pub PasswordExpired: ::BOOLEAN,
    pub PrivateDataSensitive: ::BOOLEAN,
}
pub type PUSER_ALL_INFORMATION = *mut USER_ALL_INFORMATION;
pub const USER_ALL_PARAMETERS: ::ULONG = 0x00200000;
pub const CLEAR_BLOCK_LENGTH: usize = 8;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct CLEAR_BLOCK {
    pub data: [::CHAR; CLEAR_BLOCK_LENGTH],
}
pub type PCLEAR_BLOCK = *mut CLEAR_BLOCK;
pub const CYPHER_BLOCK_LENGTH: usize = 8;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct CYPHER_BLOCK {
    pub data: [::CHAR; CYPHER_BLOCK_LENGTH],
}
pub type PCYPHER_BLOCK = *mut CYPHER_BLOCK;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct LM_OWF_PASSWORD {
    pub data: [CYPHER_BLOCK; 2],
}
pub type PLM_OWF_PASSWORD = *mut LM_OWF_PASSWORD;
pub type LM_CHALLENGE = CLEAR_BLOCK;
pub type PLM_CHALLENGE = *mut LM_CHALLENGE;
pub type NT_OWF_PASSWORD = LM_OWF_PASSWORD;
pub type PNT_OWF_PASSWORD = *mut NT_OWF_PASSWORD;
pub type NT_CHALLENGE = LM_CHALLENGE;
pub type PNT_CHALLENGE = *mut NT_CHALLENGE;
pub const USER_SESSION_KEY_LENGTH: usize = CYPHER_BLOCK_LENGTH * 2;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct USER_SESSION_KEY {
    pub data: [CYPHER_BLOCK; 2],
}
pub type PUSER_SESSION_KEY = *mut USER_SESSION_KEY;
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum NETLOGON_LOGON_INFO_CLASS {
    NetlogonInteractiveInformation = 1,
    NetlogonNetworkInformation,
    NetlogonServiceInformation,
    NetlogonGenericInformation,
    NetlogonInteractiveTransitiveInformation,
    NetlogonNetworkTransitiveInformation,
    NetlogonServiceTransitiveInformation,
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NETLOGON_LOGON_IDENTITY_INFO {
    pub LogonDomainName: UNICODE_STRING,
    pub ParameterControl: ::ULONG,
    pub LogonId: OLD_LARGE_INTEGER,
    pub UserName: UNICODE_STRING,
    pub Workstation: UNICODE_STRING,
}
pub type PNETLOGON_LOGON_IDENTITY_INFO = *mut NETLOGON_LOGON_IDENTITY_INFO;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NETLOGON_INTERACTIVE_INFO {
    pub Identity: NETLOGON_LOGON_IDENTITY_INFO,
    pub LmOwfPassword: LM_OWF_PASSWORD,
    pub NtOwfPassword: NT_OWF_PASSWORD,
}
pub type PNETLOGON_INTERACTIVE_INFO = *mut NETLOGON_INTERACTIVE_INFO;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NETLOGON_SERVICE_INFO {
    pub Identity: NETLOGON_LOGON_IDENTITY_INFO,
    pub LmOwfPassword: LM_OWF_PASSWORD,
    pub NtOwfPassword: NT_OWF_PASSWORD,
}
pub type PNETLOGON_SERVICE_INFO = *mut NETLOGON_SERVICE_INFO;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NETLOGON_NETWORK_INFO {
    pub Identity: NETLOGON_LOGON_IDENTITY_INFO,
    pub LmChallenge: LM_CHALLENGE,
    pub NtChallengeResponse: STRING,
    pub LmChallengeResponse: STRING,
}
pub type PNETLOGON_NETWORK_INFO = *mut NETLOGON_NETWORK_INFO;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NETLOGON_GENERIC_INFO {
    pub Identity: NETLOGON_LOGON_IDENTITY_INFO,
    pub PackageName: UNICODE_STRING,
    pub DataLength: ::ULONG,
    pub LogonData: ::PUCHAR,
}
pub type PNETLOGON_GENERIC_INFO = *mut NETLOGON_GENERIC_INFO;
pub const MSV1_0_PASSTHRU: ::ULONG = 0x01;
pub const MSV1_0_GUEST_LOGON: ::ULONG = 0x02;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct MSV1_0_VALIDATION_INFO {
    pub LogoffTime: ::LARGE_INTEGER,
    pub KickoffTime: ::LARGE_INTEGER,
    pub LogonServer: UNICODE_STRING,
    pub LogonDomainName: UNICODE_STRING,
    pub SessionKey: USER_SESSION_KEY,
    pub Authoritative: ::BOOLEAN,
    pub UserFlags: ::ULONG,
    pub WhichFields: ::ULONG,
    pub UserId: ::ULONG,
}
pub type PMSV1_0_VALIDATION_INFO = *mut MSV1_0_VALIDATION_INFO;
pub const MSV1_0_VALIDATION_LOGOFF_TIME: ::ULONG = 0x00000001;
pub const MSV1_0_VALIDATION_KICKOFF_TIME: ::ULONG = 0x00000002;
pub const MSV1_0_VALIDATION_LOGON_SERVER: ::ULONG = 0x00000004;
pub const MSV1_0_VALIDATION_LOGON_DOMAIN: ::ULONG = 0x00000008;
pub const MSV1_0_VALIDATION_SESSION_KEY: ::ULONG = 0x00000010;
pub const MSV1_0_VALIDATION_USER_FLAGS: ::ULONG = 0x00000020;
pub const MSV1_0_VALIDATION_USER_ID: ::ULONG = 0x00000040;
pub const MSV1_0_SUBAUTH_ACCOUNT_DISABLED: ::ULONG = 0x00000001;
pub const MSV1_0_SUBAUTH_PASSWORD: ::ULONG = 0x00000002;
pub const MSV1_0_SUBAUTH_WORKSTATIONS: ::ULONG = 0x00000004;
pub const MSV1_0_SUBAUTH_LOGON_HOURS: ::ULONG = 0x00000008;
pub const MSV1_0_SUBAUTH_ACCOUNT_EXPIRY: ::ULONG = 0x00000010;
pub const MSV1_0_SUBAUTH_PASSWORD_EXPIRY: ::ULONG = 0x00000020;
pub const MSV1_0_SUBAUTH_ACCOUNT_TYPE: ::ULONG = 0x00000040;
pub const MSV1_0_SUBAUTH_LOCKOUT: ::ULONG = 0x00000080;

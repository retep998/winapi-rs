// Copyright © 2015, skdltmxn
// Licensed under the MIT License <LICENSE.md>
//! LSA Policy Lookup API
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct LSA_UNICODE_STRING {
    pub Length: ::USHORT,
    pub MaximumLength: ::USHORT,
    pub Buffer: ::PWSTR,
}
pub type PLSA_UNICODE_STRING = *mut LSA_UNICODE_STRING;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct LSA_STRING {
    pub Length: ::USHORT,
    pub MaximumLength: ::USHORT,
    pub Buffer: ::PCHAR,
}
pub type PLSA_STRING = *mut LSA_STRING;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct LSA_OBJECT_ATTRIBUTES {
    pub Length: ::ULONG,
    pub RootDirectory: ::HANDLE,
    pub ObjectName: PLSA_UNICODE_STRING,
    pub Attributes: ::ULONG,
    pub SecurityDescriptor: ::PVOID,
    pub SecurityQualityOfService: ::PVOID,
}
pub type PLSA_OBJECT_ATTRIBUTES = *mut LSA_OBJECT_ATTRIBUTES;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct LSA_TRUST_INFORMATION {
    pub Name: LSA_UNICODE_STRING,
    pub Sid: ::PSID,
}
pub type PLSA_TRUST_INFORMATION = *mut LSA_TRUST_INFORMATION;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct LSA_REFERENCED_DOMAIN_LIST {
    pub Entries: ::ULONG,
    pub Domains: PLSA_TRUST_INFORMATION,
}
pub type PLSA_REFERENCED_DOMAIN_LIST = *mut LSA_REFERENCED_DOMAIN_LIST;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct LSA_TRANSLATED_SID2 {
    pub Use: ::SID_NAME_USE,
    pub Sid: ::PSID,
    pub DomainIndex: ::LONG,
    pub Flags: ::ULONG,
}
pub type PLSA_TRANSLATED_SID2 = *mut LSA_TRANSLATED_SID2;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct LSA_TRANSLATED_NAME {
    pub Use: ::SID_NAME_USE,
    pub Name: LSA_UNICODE_STRING,
    pub DomainIndex: ::LONG,
}
pub type PLSA_TRANSLATED_NAME = *mut LSA_TRANSLATED_NAME;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct POLICY_ACCOUNT_DOMAIN_INFO {
    pub DomainName: LSA_UNICODE_STRING,
    pub DomainSid: ::PSID,
}
pub type PPOLICY_ACCOUNT_DOMAIN_INFO = *mut POLICY_ACCOUNT_DOMAIN_INFO;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct POLICY_DNS_DOMAIN_INFO {
    pub Name: LSA_UNICODE_STRING,
    pub DnsDomainName: LSA_UNICODE_STRING,
    pub DnsForestName: LSA_UNICODE_STRING,
    pub DomainGuid: ::GUID,
    pub Sid: ::PSID,
}
pub type PPOLICY_DNS_DOMAIN_INFO = *mut POLICY_DNS_DOMAIN_INFO;
pub const LOOKUP_VIEW_LOCAL_INFORMATION: ::ACCESS_MASK = 0x00000001;
pub const LOOKUP_TRANSLATE_NAMES: ::ACCESS_MASK = 0x00000800;
ENUM!{enum LSA_LOOKUP_DOMAIN_INFO_CLASS {
    AccountDomainInformation = 5,
    DnsDomainInformation = 12,
}}
pub type PLSA_LOOKUP_DOMAIN_INFO_CLASS = *mut LSA_LOOKUP_DOMAIN_INFO_CLASS;
pub type LSA_LOOKUP_HANDLE = ::PVOID;
pub type PLSA_LOOKUP_HANDLE = *mut ::PVOID;

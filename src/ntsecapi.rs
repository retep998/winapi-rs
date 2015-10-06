// Copyright © 2015, skdltmxn
// Licensed under the MIT License <LICENSE.md>
//! This module defines the Local Security Authority APIs.
DEFINE_GUID!(Audit_System_SecurityStateChange, 0x0cce9210, 0x69ae, 0x11d9,
    0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30);
DEFINE_GUID!(Audit_System_SecuritySubsystemExtension, 0x0cce9211, 0x69ae, 0x11d9,
    0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30);
DEFINE_GUID!(Audit_System_Integrity, 0x0cce9212, 0x69ae, 0x11d9,
    0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30);
DEFINE_GUID!(Audit_System_IPSecDriverEvents, 0x0cce9213, 0x69ae, 0x11d9,
    0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30);
DEFINE_GUID!(Audit_System_Others, 0x0cce9214, 0x69ae, 0x11d9,
    0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30);
DEFINE_GUID!(Audit_Logon_Logon, 0x0cce9215, 0x69ae, 0x11d9,
    0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30);
DEFINE_GUID!(Audit_Logon_Logoff, 0x0cce9216, 0x69ae, 0x11d9,
    0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30);
DEFINE_GUID!(Audit_Logon_AccountLockout, 0x0cce9217, 0x69ae, 0x11d9,
    0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30);
DEFINE_GUID!(Audit_Logon_IPSecMainMode, 0x0cce9218, 0x69ae, 0x11d9,
    0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30);
DEFINE_GUID!(Audit_Logon_IPSecQuickMode, 0x0cce9219, 0x69ae, 0x11d9,
    0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30);
DEFINE_GUID!(Audit_Logon_IPSecUserMode, 0x0cce921a, 0x69ae, 0x11d9,
    0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30);
DEFINE_GUID!(Audit_Logon_SpecialLogon, 0x0cce921b, 0x69ae, 0x11d9,
    0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30);
DEFINE_GUID!(Audit_Logon_Others, 0x0cce921c, 0x69ae, 0x11d9,
    0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30);
DEFINE_GUID!(Audit_ObjectAccess_FileSystem, 0x0cce921d, 0x69ae, 0x11d9,
    0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30);
DEFINE_GUID!(Audit_ObjectAccess_Registry, 0x0cce921e, 0x69ae, 0x11d9,
    0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30);
DEFINE_GUID!(Audit_ObjectAccess_Kernel, 0x0cce921f, 0x69ae, 0x11d9,
    0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30);
DEFINE_GUID!(Audit_ObjectAccess_Sam, 0x0cce9220, 0x69ae, 0x11d9,
    0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30);
DEFINE_GUID!(Audit_ObjectAccess_CertificationServices, 0x0cce9221, 0x69ae, 0x11d9,
    0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30);
DEFINE_GUID!(Audit_ObjectAccess_ApplicationGenerated, 0x0cce9222, 0x69ae, 0x11d9,
    0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30);
DEFINE_GUID!(Audit_ObjectAccess_Handle, 0x0cce9223, 0x69ae, 0x11d9,
    0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30);
DEFINE_GUID!(Audit_ObjectAccess_Share, 0x0cce9224, 0x69ae, 0x11d9,
    0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30);
DEFINE_GUID!(Audit_ObjectAccess_FirewallPacketDrops, 0x0cce9225, 0x69ae, 0x11d9,
    0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30);
DEFINE_GUID!(Audit_ObjectAccess_FirewallConnection, 0x0cce9226, 0x69ae, 0x11d9,
    0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30);
DEFINE_GUID!(Audit_ObjectAccess_Other, 0x0cce9227, 0x69ae, 0x11d9,
    0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30);
DEFINE_GUID!(Audit_PrivilegeUse_Sensitive, 0x0cce9228, 0x69ae, 0x11d9,
    0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30);
DEFINE_GUID!(Audit_PrivilegeUse_NonSensitive, 0x0cce9229, 0x69ae, 0x11d9,
    0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30);
DEFINE_GUID!(Audit_PrivilegeUse_Others, 0x0cce922a, 0x69ae, 0x11d9,
    0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30);
DEFINE_GUID!(Audit_DetailedTracking_ProcessCreation, 0x0cce922b, 0x69ae, 0x11d9,
    0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30);
DEFINE_GUID!(Audit_DetailedTracking_ProcessTermination, 0x0cce922c, 0x69ae, 0x11d9,
    0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30);
DEFINE_GUID!(Audit_DetailedTracking_DpapiActivity, 0x0cce922d, 0x69ae, 0x11d9,
    0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30);
DEFINE_GUID!(Audit_DetailedTracking_RpcCall, 0x0cce922e, 0x69ae, 0x11d9,
    0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30);
DEFINE_GUID!(Audit_PolicyChange_AuditPolicy, 0x0cce922f, 0x69ae, 0x11d9,
    0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30);
DEFINE_GUID!(Audit_PolicyChange_AuthenticationPolicy, 0x0cce9230, 0x69ae, 0x11d9,
    0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30);
DEFINE_GUID!(Audit_PolicyChange_AuthorizationPolicy, 0x0cce9231, 0x69ae, 0x11d9,
    0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30);
DEFINE_GUID!(Audit_PolicyChange_MpsscvRulePolicy, 0x0cce9232, 0x69ae, 0x11d9,
    0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30);
DEFINE_GUID!(Audit_PolicyChange_WfpIPSecPolicy, 0x0cce9233, 0x69ae, 0x11d9,
    0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30);
DEFINE_GUID!(Audit_PolicyChange_Others, 0x0cce9234, 0x69ae, 0x11d9,
    0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30);
DEFINE_GUID!(Audit_AccountManagement_UserAccount, 0x0cce9235, 0x69ae, 0x11d9,
    0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30);
DEFINE_GUID!(Audit_AccountManagement_ComputerAccount, 0x0cce9236, 0x69ae, 0x11d9,
    0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30);
DEFINE_GUID!(Audit_AccountManagement_SecurityGroup, 0x0cce9237, 0x69ae, 0x11d9,
    0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30);
DEFINE_GUID!(Audit_AccountManagement_DistributionGroup, 0x0cce9238, 0x69ae, 0x11d9,
    0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30);
DEFINE_GUID!(Audit_AccountManagement_ApplicationGroup, 0x0cce9239, 0x69ae, 0x11d9,
    0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30);
DEFINE_GUID!(Audit_AccountManagement_Others, 0x0cce923a, 0x69ae, 0x11d9,
    0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30);
DEFINE_GUID!(Audit_DSAccess_DSAccess, 0x0cce923b, 0x69ae, 0x11d9,
    0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30);
DEFINE_GUID!(Audit_DsAccess_AdAuditChanges, 0x0cce923c, 0x69ae, 0x11d9,
    0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30);
DEFINE_GUID!(Audit_Ds_Replication, 0x0cce923d, 0x69ae, 0x11d9,
    0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30);
DEFINE_GUID!(Audit_Ds_DetailedReplication, 0x0cce923e, 0x69ae, 0x11d9,
    0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30);
DEFINE_GUID!(Audit_AccountLogon_CredentialValidation, 0x0cce923f, 0x69ae, 0x11d9,
    0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30);
DEFINE_GUID!(Audit_AccountLogon_Kerberos, 0x0cce9240, 0x69ae, 0x11d9,
    0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30);
DEFINE_GUID!(Audit_AccountLogon_Others, 0x0cce9241, 0x69ae, 0x11d9,
    0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30);
DEFINE_GUID!(Audit_AccountLogon_KerbCredentialValidation, 0x0cce9242, 0x69ae, 0x11d9,
    0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30);
DEFINE_GUID!(Audit_Logon_NPS, 0x0cce9243, 0x69ae, 0x11d9,
    0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30);
DEFINE_GUID!(Audit_ObjectAccess_DetailedFileShare, 0x0cce9244, 0x69ae, 0x11d9,
    0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30);
DEFINE_GUID!(Audit_ObjectAccess_RemovableStorage, 0x0cce9245, 0x69ae, 0x11d9,
    0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30);
DEFINE_GUID!(Audit_ObjectAccess_CbacStaging, 0x0cce9246, 0x69ae, 0x11d9,
    0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30);
DEFINE_GUID!(Audit_Logon_Claims, 0x0cce9247, 0x69ae, 0x11d9,
    0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30);
DEFINE_GUID!(Audit_System, 0x69979848, 0x797a, 0x11d9,
    0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30);
DEFINE_GUID!(Audit_Logon, 0x69979849, 0x797a, 0x11d9,
    0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30);
DEFINE_GUID!(Audit_ObjectAccess, 0x6997984a, 0x797a, 0x11d9,
    0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30);
DEFINE_GUID!(Audit_PrivilegeUse, 0x6997984b, 0x797a, 0x11d9,
    0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30);
DEFINE_GUID!(Audit_DetailedTracking, 0x6997984c, 0x797a, 0x11d9,
    0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30);
DEFINE_GUID!(Audit_PolicyChange, 0x6997984d, 0x797a, 0x11d9,
    0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30);
DEFINE_GUID!(Audit_AccountManagement, 0x6997984e, 0x797a, 0x11d9,
    0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30);
DEFINE_GUID!(Audit_DirectoryServiceAccess, 0x6997984f, 0x797a, 0x11d9,
    0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30);
DEFINE_GUID!(Audit_AccountLogon, 0x69979850, 0x797a, 0x11d9,
    0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30);
ENUM!{enum POLICY_AUDIT_EVENT_TYPE {
    AuditCategorySystem = 0,
    AuditCategoryLogon,
    AuditCategoryObjectAccess,
    AuditCategoryPrivilegeUse,
    AuditCategoryDetailedTracking,
    AuditCategoryPolicyChange,
    AuditCategoryAccountManagement,
    AuditCategoryDirectoryServiceAccess,
    AuditCategoryAccountLogon,
}}
pub type PPOLICY_AUDIT_EVENT_TYPE = *mut POLICY_AUDIT_EVENT_TYPE;
pub const POLICY_AUDIT_EVENT_UNCHANGED: POLICY_AUDIT_EVENT_OPTIONS = 0x00000000;
pub const POLICY_AUDIT_EVENT_SUCCESS: POLICY_AUDIT_EVENT_OPTIONS = 0x00000001;
pub const POLICY_AUDIT_EVENT_FAILURE: POLICY_AUDIT_EVENT_OPTIONS = 0x00000002;
pub const POLICY_AUDIT_EVENT_NONE: POLICY_AUDIT_EVENT_OPTIONS = 0x00000004;
pub const POLICY_AUDIT_EVENT_MASK: POLICY_AUDIT_EVENT_OPTIONS = POLICY_AUDIT_EVENT_SUCCESS
    | POLICY_AUDIT_EVENT_FAILURE | POLICY_AUDIT_EVENT_UNCHANGED | POLICY_AUDIT_EVENT_NONE;
pub const POLICY_VIEW_LOCAL_INFORMATION: ::ACCESS_MASK = 0x00000001;
pub const POLICY_VIEW_AUDIT_INFORMATION: ::ACCESS_MASK = 0x00000002;
pub const POLICY_GET_PRIVATE_INFORMATION: ::ACCESS_MASK = 0x00000004;
pub const POLICY_TRUST_ADMIN: ::ACCESS_MASK = 0x00000008;
pub const POLICY_CREATE_ACCOUNT: ::ACCESS_MASK = 0x00000010;
pub const POLICY_CREATE_SECRET: ::ACCESS_MASK = 0x00000020;
pub const POLICY_CREATE_PRIVILEGE: ::ACCESS_MASK = 0x00000040;
pub const POLICY_SET_DEFAULT_QUOTA_LIMITS: ::ACCESS_MASK = 0x00000080;
pub const POLICY_SET_AUDIT_REQUIREMENTS: ::ACCESS_MASK = 0x00000100;
pub const POLICY_AUDIT_LOG_ADMIN: ::ACCESS_MASK = 0x00000200;
pub const POLICY_SERVER_ADMIN: ::ACCESS_MASK = 0x00000400;
pub const POLICY_LOOKUP_NAMES: ::ACCESS_MASK = 0x00000800;
pub const POLICY_NOTIFICATION: ::ACCESS_MASK = 0x00001000;
pub const POLICY_ALL_ACCESS: ::ACCESS_MASK = ::STANDARD_RIGHTS_REQUIRED
    | POLICY_VIEW_LOCAL_INFORMATION | POLICY_VIEW_AUDIT_INFORMATION
    | POLICY_GET_PRIVATE_INFORMATION | POLICY_TRUST_ADMIN | POLICY_CREATE_ACCOUNT
    | POLICY_CREATE_SECRET | POLICY_CREATE_PRIVILEGE | POLICY_SET_DEFAULT_QUOTA_LIMITS
    | POLICY_SET_AUDIT_REQUIREMENTS | POLICY_AUDIT_LOG_ADMIN | POLICY_SERVER_ADMIN
    | POLICY_LOOKUP_NAMES;
pub const POLICY_READ: ::ACCESS_MASK = ::STANDARD_RIGHTS_READ | POLICY_VIEW_AUDIT_INFORMATION
    | POLICY_GET_PRIVATE_INFORMATION;
pub const POLICY_WRITE: ::ACCESS_MASK = ::STANDARD_RIGHTS_WRITE | POLICY_TRUST_ADMIN
    | POLICY_CREATE_ACCOUNT | POLICY_CREATE_SECRET | POLICY_CREATE_PRIVILEGE
    | POLICY_SET_DEFAULT_QUOTA_LIMITS | POLICY_SET_AUDIT_REQUIREMENTS | POLICY_AUDIT_LOG_ADMIN
    | POLICY_SERVER_ADMIN;
pub const POLICY_EXECUTE: ::ACCESS_MASK = ::STANDARD_RIGHTS_EXECUTE
    | POLICY_VIEW_LOCAL_INFORMATION | POLICY_LOOKUP_NAMES;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct LSA_TRANSLATED_SID {
    pub Use: ::SID_NAME_USE,
    pub RelativeId: ::ULONG,
    pub DomainIndex: ::LONG,
}
pub type PLSA_TRANSLATED_SID = *mut LSA_TRANSLATED_SID;
ENUM!{enum POLICY_LSA_SERVER_ROLE {
    PolicyServerRoleBackup = 2,
    PolicyServerRolePrimary,
}}
pub type PPOLICY_LSA_SERVER_ROLE = *mut POLICY_LSA_SERVER_ROLE;
pub type POLICY_AUDIT_EVENT_OPTIONS = ::ULONG;
pub type PPOLICY_AUDIT_EVENT_OPTIONS = *mut ::ULONG;
ENUM!{enum POLICY_INFORMATION_CLASS {
    PolicyAuditLogInformation = 1,
    PolicyAuditEventsInformation,
    PolicyPrimaryDomainInformation,
    PolicyPdAccountInformation,
    PolicyAccountDomainInformation,
    PolicyLsaServerRoleInformation,
    PolicyReplicaSourceInformation,
    PolicyDefaultQuotaInformation,
    PolicyModificationInformation,
    PolicyAuditFullSetInformation,
    PolicyAuditFullQueryInformation,
    PolicyDnsDomainInformation,
    PolicyDnsDomainInformationInt,
    PolicyLocalAccountDomainInformation,
    PolicyLastEntry,
}}
pub type PPOLICY_INFORMATION_CLASS = *mut POLICY_INFORMATION_CLASS;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct POLICY_AUDIT_LOG_INFO {
    pub AuditLogPercentFull: ::ULONG,
    pub MaximumLogSize: ::ULONG,
    pub AuditRetentionPeriod: ::LARGE_INTEGER,
    pub AuditLogFullShutdownInProgress: ::BOOLEAN,
    pub TimeToShutdown: ::LARGE_INTEGER,
    pub NextAuditRecordId: ::ULONG,
}
pub type PPOLICY_AUDIT_LOG_INFO = *mut POLICY_AUDIT_LOG_INFO;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct POLICY_AUDIT_EVENTS_INFO {
    pub AuditingMode: ::BOOLEAN,
    pub EventAuditingOptions: PPOLICY_AUDIT_EVENT_OPTIONS,
    pub MaximumAuditEventCount: ::ULONG,
}
pub type PPOLICY_AUDIT_EVENTS_INFO = *mut POLICY_AUDIT_EVENTS_INFO;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct POLICY_AUDIT_SUBCATEGORIES_INFO {
    pub MaximumSubCategoryCount: ::ULONG,
    pub EventAuditingOptions: PPOLICY_AUDIT_EVENT_OPTIONS,
}
pub type PPOLICY_AUDIT_SUBCATEGORIES_INFO = *mut POLICY_AUDIT_SUBCATEGORIES_INFO;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct POLICY_AUDIT_CATEGORIES_INFO {
    pub MaximumSubCategoryCount: ::ULONG,
    pub SubCategoriesInfo: PPOLICY_AUDIT_SUBCATEGORIES_INFO,
}
pub type PPOLICY_AUDIT_CATEGORIES_INFO = *mut POLICY_AUDIT_CATEGORIES_INFO;
pub const PER_USER_POLICY_UNCHANGED: ::ULONG = 0x00;
pub const PER_USER_AUDIT_SUCCESS_INCLUDE: ::ULONG = 0x01;
pub const PER_USER_AUDIT_SUCCESS_EXCLUDE: ::ULONG = 0x02;
pub const PER_USER_AUDIT_FAILURE_INCLUDE: ::ULONG = 0x04;
pub const PER_USER_AUDIT_FAILURE_EXCLUDE: ::ULONG = 0x08;
pub const PER_USER_AUDIT_NONE: ::ULONG = 0x10;
pub const VALID_PER_USER_AUDIT_POLICY_FLAG: ::ULONG = PER_USER_AUDIT_SUCCESS_INCLUDE
    | PER_USER_AUDIT_SUCCESS_EXCLUDE | PER_USER_AUDIT_FAILURE_INCLUDE
    | PER_USER_AUDIT_FAILURE_EXCLUDE | PER_USER_AUDIT_NONE;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct POLICY_PRIMARY_DOMAIN_INFO {
    pub Name: ::LSA_UNICODE_STRING,
    pub Sid: ::PSID,
}
pub type PPOLICY_PRIMARY_DOMAIN_INFO = *mut POLICY_PRIMARY_DOMAIN_INFO;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct POLICY_PD_ACCOUNT_INFO {
    pub Name: ::LSA_UNICODE_STRING,
}
pub type PPOLICY_PD_ACCOUNT_INFO = *mut POLICY_PD_ACCOUNT_INFO;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct POLICY_LSA_SERVER_ROLE_INFO {
    pub LsaServerRole: POLICY_LSA_SERVER_ROLE,
}
pub type PPOLICY_LSA_SERVER_ROLE_INFO = *mut POLICY_LSA_SERVER_ROLE_INFO;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct POLICY_REPLICA_SOURCE_INFO {
    pub ReplicaSource: ::LSA_UNICODE_STRING,
    pub ReplicaAccountName: ::LSA_UNICODE_STRING,
}
pub type PPOLICY_REPLICA_SOURCE_INFO = *mut POLICY_REPLICA_SOURCE_INFO;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct POLICY_DEFAULT_QUOTA_INFO {
    pub QuotaLimits: ::QUOTA_LIMITS,
}
pub type PPOLICY_DEFAULT_QUOTA_INFO = *mut POLICY_DEFAULT_QUOTA_INFO;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct POLICY_MODIFICATION_INFO {
    pub ModifiedId: ::LARGE_INTEGER,
    pub DatabaseCreationTime: ::LARGE_INTEGER,
}
pub type PPOLICY_MODIFICATION_INFO = *mut POLICY_MODIFICATION_INFO;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct POLICY_AUDIT_FULL_SET_INFO {
    pub ShutDownOnFull: ::BOOLEAN,
}
pub type PPOLICY_AUDIT_FULL_SET_INFO = *mut POLICY_AUDIT_FULL_SET_INFO;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct POLICY_AUDIT_FULL_QUERY_INFO {
    pub ShutDownOnFull: ::BOOLEAN,
    pub LogIsFull: ::BOOLEAN,
}
pub type PPOLICY_AUDIT_FULL_QUERY_INFO = *mut POLICY_AUDIT_FULL_QUERY_INFO;
ENUM!{enum POLICY_DOMAIN_INFORMATION_CLASS {
    PolicyDomainEfsInformation = 2,
    PolicyDomainKerberosTicketInformation,
}}
pub type PPOLICY_DOMAIN_INFORMATION_CLASS = *mut POLICY_DOMAIN_INFORMATION_CLASS;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct POLICY_DOMAIN_EFS_INFO {
    pub InfoLength: ::ULONG,
    pub EfsBlob: ::PUCHAR,
}
pub type PPOLICY_DOMAIN_EFS_INFO = *mut POLICY_DOMAIN_EFS_INFO;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct POLICY_DOMAIN_KERBEROS_TICKET_INFO {
    pub AuthenticationOptions: ::ULONG,
    pub MaxServiceTicketAge: ::LARGE_INTEGER,
    pub MaxTicketAge: ::LARGE_INTEGER,
    pub MaxRenewAge: ::LARGE_INTEGER,
    pub MaxClockSkew: ::LARGE_INTEGER,
    pub Reserved: ::LARGE_INTEGER,
}
pub type PPOLICY_DOMAIN_KERBEROS_TICKET_INFO = *mut POLICY_DOMAIN_KERBEROS_TICKET_INFO;
ENUM!{enum POLICY_NOTIFICATION_INFORMATION_CLASS {
    PolicyNotifyAuditEventsInformation = 1,
    PolicyNotifyAccountDomainInformation,
    PolicyNotifyServerRoleInformation,
    PolicyNotifyDnsDomainInformation,
    PolicyNotifyDomainEfsInformation,
    PolicyNotifyDomainKerberosTicketInformation,
    PolicyNotifyMachineAccountPasswordInformation,
    PolicyNotifyGlobalSaclInformation,
    PolicyNotifyMax,
}}
pub type PPOLICY_NOTIFICATION_INFORMATION_CLASS = *mut POLICY_NOTIFICATION_INFORMATION_CLASS;
pub type LSA_HANDLE = ::PVOID;
pub type PLSA_HANDLE = *mut ::PVOID;
ENUM!{enum TRUSTED_INFORMATION_CLASS {
    TrustedDomainNameInformation = 1,
    TrustedControllersInformation,
    TrustedPosixOffsetInformation,
    TrustedPasswordInformation,
    TrustedDomainInformationBasic,
    TrustedDomainInformationEx,
    TrustedDomainAuthInformation,
    TrustedDomainFullInformation,
    TrustedDomainAuthInformationInternal,
    TrustedDomainFullInformationInternal,
    TrustedDomainInformationEx2Internal,
    TrustedDomainFullInformation2Internal,
    TrustedDomainSupportedEncryptionTypes,
}}
pub type PTRUSTED_INFORMATION_CLASS = *mut TRUSTED_INFORMATION_CLASS;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct TRUSTED_DOMAIN_NAME_INFO {
    pub Name: ::LSA_UNICODE_STRING,
}
pub type PTRUSTED_DOMAIN_NAME_INFO = *mut TRUSTED_DOMAIN_NAME_INFO;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct TRUSTED_CONTROLLERS_INFO {
    pub Entries: ::ULONG,
    pub Names: ::PLSA_UNICODE_STRING,
}
pub type PTRUSTED_CONTROLLERS_INFO = *mut TRUSTED_CONTROLLERS_INFO;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct TRUSTED_POSIX_OFFSET_INFO {
    pub Offset: ::ULONG,
}
pub type PTRUSTED_POSIX_OFFSET_INFO = *mut TRUSTED_POSIX_OFFSET_INFO;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct TRUSTED_PASSWORD_INFO {
    pub Password: ::LSA_UNICODE_STRING,
    pub OldPassword: ::LSA_UNICODE_STRING,
}
pub type PTRUSTED_PASSWORD_INFO = *mut TRUSTED_PASSWORD_INFO;
pub type TRUSTED_DOMAIN_INFORMATION_BASIC = ::LSA_TRUST_INFORMATION;
pub type PTRUSTED_DOMAIN_INFORMATION_BASIC = ::PLSA_TRUST_INFORMATION;
pub const TRUST_DIRECTION_DISABLED: ::ULONG = 0x00000000;
pub const TRUST_DIRECTION_INBOUND: ::ULONG = 0x00000001;
pub const TRUST_DIRECTION_OUTBOUND: ::ULONG = 0x00000002;
pub const TRUST_DIRECTION_BIDIRECTIONAL: ::ULONG = TRUST_DIRECTION_INBOUND
    | TRUST_DIRECTION_OUTBOUND;
pub const TRUST_TYPE_DOWNLEVEL: ::ULONG = 0x00000001;
pub const TRUST_TYPE_UPLEVEL: ::ULONG = 0x00000002;
pub const TRUST_TYPE_MIT: ::ULONG = 0x00000003;
pub const TRUST_ATTRIBUTE_NON_TRANSITIVE: ::ULONG = 0x00000001;
pub const TRUST_ATTRIBUTE_UPLEVEL_ONLY: ::ULONG = 0x00000002;
pub const TRUST_ATTRIBUTE_QUARANTINED_DOMAIN: ::ULONG = 0x00000004;
pub const TRUST_ATTRIBUTE_FOREST_TRANSITIVE: ::ULONG = 0x00000008;
pub const TRUST_ATTRIBUTE_CROSS_ORGANIZATION: ::ULONG = 0x00000010;
pub const TRUST_ATTRIBUTE_WITHIN_FOREST: ::ULONG = 0x00000020;
pub const TRUST_ATTRIBUTE_TREAT_AS_EXTERNAL: ::ULONG = 0x00000040;
pub const TRUST_ATTRIBUTE_TRUST_USES_RC4_ENCRYPTION: ::ULONG = 0x00000080;
pub const TRUST_ATTRIBUTE_TRUST_USES_AES_KEYS: ::ULONG = 0x00000100;
pub const TRUST_ATTRIBUTE_CROSS_ORGANIZATION_NO_TGT_DELEGATION: ::ULONG = 0x00000200;
pub const TRUST_ATTRIBUTES_VALID: ::ULONG = 0xFF03FFFF;
pub const TRUST_ATTRIBUTES_USER: ::ULONG = 0xFF000000;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct TRUSTED_DOMAIN_INFORMATION_EX {
    pub Name: ::LSA_UNICODE_STRING,
    pub FlatName: ::LSA_UNICODE_STRING,
    pub Sid: ::PSID,
    pub TrustDirection: ::ULONG,
    pub TrustType: ::ULONG,
    pub TrustAttributes: ::ULONG,
}
pub type PTRUSTED_DOMAIN_INFORMATION_EX = *mut TRUSTED_DOMAIN_INFORMATION_EX;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct TRUSTED_DOMAIN_INFORMATION_EX2 {
    pub Name: ::LSA_UNICODE_STRING,
    pub FlatName: ::LSA_UNICODE_STRING,
    pub Sid: ::PSID,
    pub TrustDirection: ::ULONG,
    pub TrustType: ::ULONG,
    pub TrustAttributes: ::ULONG,
    pub ForestTrustLength: ::ULONG,
    pub ForestTrustInfo: ::PUCHAR,
}
pub type PTRUSTED_DOMAIN_INFORMATION_EX2 = *mut TRUSTED_DOMAIN_INFORMATION_EX2;
pub const TRUST_AUTH_TYPE_NONE: ::ULONG = 0;
pub const TRUST_AUTH_TYPE_NT4OWF: ::ULONG = 1;
pub const TRUST_AUTH_TYPE_CLEAR: ::ULONG = 2;
pub const TRUST_AUTH_TYPE_VERSION: ::ULONG = 3;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct LSA_AUTH_INFORMATION {
    pub LastUpdateTime: ::LARGE_INTEGER,
    pub AuthType: ::ULONG,
    pub AuthInfoLength: ::ULONG,
    pub AuthInfo: ::PUCHAR,
}
pub type PLSA_AUTH_INFORMATION = *mut LSA_AUTH_INFORMATION;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct TRUSTED_DOMAIN_AUTH_INFORMATION {
    pub IncomingAuthInfos: ::ULONG,
    pub IncomingAuthenticationInformation: PLSA_AUTH_INFORMATION,
    pub IncomingPreviousAuthenticationInformation: PLSA_AUTH_INFORMATION,
    pub OutgoingAuthInfos: ::ULONG,
    pub OutgoingAuthenticationInformation: PLSA_AUTH_INFORMATION,
    pub OutgoingPreviousAuthenticationInformation: PLSA_AUTH_INFORMATION,
}
pub type PTRUSTED_DOMAIN_AUTH_INFORMATION = *mut TRUSTED_DOMAIN_AUTH_INFORMATION;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct TRUSTED_DOMAIN_FULL_INFORMATION {
    pub Information: TRUSTED_DOMAIN_INFORMATION_EX,
    pub PosixOffset: TRUSTED_POSIX_OFFSET_INFO,
    pub AuthInformation: TRUSTED_DOMAIN_AUTH_INFORMATION,
}
pub type PTRUSTED_DOMAIN_FULL_INFORMATION = *mut TRUSTED_DOMAIN_FULL_INFORMATION;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct TRUSTED_DOMAIN_FULL_INFORMATION2 {
    pub Information: TRUSTED_DOMAIN_INFORMATION_EX2,
    pub PosixOffset: TRUSTED_POSIX_OFFSET_INFO,
    pub AuthInformation: TRUSTED_DOMAIN_AUTH_INFORMATION,
}
pub type PTRUSTED_DOMAIN_FULL_INFORMATION2 = *mut TRUSTED_DOMAIN_FULL_INFORMATION2;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct TRUSTED_DOMAIN_SUPPORTED_ENCRYPTION_TYPES {
    pub SupportedEncryptionTypes: ::ULONG,
}
pub type PTRUSTED_DOMAIN_SUPPORTED_ENCRYPTION_TYPES =
    *mut TRUSTED_DOMAIN_SUPPORTED_ENCRYPTION_TYPES;
ENUM!{enum LSA_FOREST_TRUST_RECORD_TYPE {
    ForestTrustTopLevelName,
    ForestTrustTopLevelNameEx,
    ForestTrustDomainInfo,
    ForestTrustRecordTypeLast, // = ForestTrustDomainInfo,
}}
pub const LSA_FTRECORD_DISABLED_REASONS: ::ULONG = 0x0000FFFF;
pub const LSA_TLN_DISABLED_NEW: ::ULONG = 0x00000001;
pub const LSA_TLN_DISABLED_ADMIN: ::ULONG = 0x00000002;
pub const LSA_TLN_DISABLED_CONFLICT: ::ULONG = 0x00000004;
pub const LSA_SID_DISABLED_ADMIN: ::ULONG = 0x00000001;
pub const LSA_SID_DISABLED_CONFLICT: ::ULONG = 0x00000002;
pub const LSA_NB_DISABLED_ADMIN: ::ULONG = 0x00000004;
pub const LSA_NB_DISABLED_CONFLICT: ::ULONG = 0x00000008;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct LSA_FOREST_TRUST_DOMAIN_INFO {
    pub Sid: ::PSID,
    pub DnsName: ::LSA_UNICODE_STRING,
    pub NetbiosName: ::LSA_UNICODE_STRING,
}
pub type PLSA_FOREST_TRUST_DOMAIN_INFO = *mut LSA_FOREST_TRUST_DOMAIN_INFO;
pub const MAX_FOREST_TRUST_BINARY_DATA_SIZE: ::ULONG = 128 * 1024;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct LSA_FOREST_TRUST_BINARY_DATA {
    pub Length: ::ULONG,
    pub Buffer: ::PUCHAR,
}
pub type PLSA_FOREST_TRUST_BINARY_DATA = *mut LSA_FOREST_TRUST_BINARY_DATA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct LSA_FOREST_TRUST_RECORD_ForestTrustData {
    pub DomainInfo: LSA_FOREST_TRUST_DOMAIN_INFO,
}
UNION!(
    LSA_FOREST_TRUST_RECORD_ForestTrustData, DomainInfo, TopLevelName, TopLevelName_mut,
    ::LSA_UNICODE_STRING
);
UNION!(
    LSA_FOREST_TRUST_RECORD_ForestTrustData, DomainInfo, Data, Data_mut,
    LSA_FOREST_TRUST_BINARY_DATA
);
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct LSA_FOREST_TRUST_RECORD {
    pub Flags: ::ULONG,
    pub ForestTrustType: LSA_FOREST_TRUST_RECORD_TYPE,
    pub Time: ::LARGE_INTEGER,
    pub ForestTrustData: LSA_FOREST_TRUST_RECORD_ForestTrustData,
}
pub type PLSA_FOREST_TRUST_RECORD = *mut LSA_FOREST_TRUST_RECORD;
pub const MAX_RECORDS_IN_FOREST_TRUST_INFO: ::ULONG = 4000;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct LSA_FOREST_TRUST_INFORMATION {
    pub RecordCount: ::ULONG,
    pub Entries: *mut PLSA_FOREST_TRUST_RECORD,
}
pub type PLSA_FOREST_TRUST_INFORMATION = *mut LSA_FOREST_TRUST_INFORMATION;
ENUM!{enum LSA_FOREST_TRUST_COLLISION_RECORD_TYPE {
    CollisionTdo,
    CollisionXref,
    CollisionOther,
}}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct LSA_FOREST_TRUST_COLLISION_RECORD {
    pub Index: ::ULONG,
    pub Type: LSA_FOREST_TRUST_COLLISION_RECORD_TYPE,
    pub Flags: ::ULONG,
    pub Name: ::LSA_UNICODE_STRING,
}
pub type PLSA_FOREST_TRUST_COLLISION_RECORD = *mut LSA_FOREST_TRUST_COLLISION_RECORD;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct LSA_FOREST_TRUST_COLLISION_INFORMATION {
    pub RecordCount: ::ULONG,
    pub Entries: *mut PLSA_FOREST_TRUST_COLLISION_RECORD,
}
pub type PLSA_FOREST_TRUST_COLLISION_INFORMATION = *mut LSA_FOREST_TRUST_COLLISION_INFORMATION;
pub type LSA_ENUMERATION_HANDLE = ::ULONG;
pub type PLSA_ENUMERATION_HANDLE = *mut ::ULONG;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct LSA_ENUMERATION_INFORMATION {
    pub Sid: ::PSID,
}
pub type PLSA_ENUMERATION_INFORMATION = *mut LSA_ENUMERATION_INFORMATION;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct LSA_LAST_INTER_LOGON_INFO {
    pub LastSuccessfulLogon: ::LARGE_INTEGER,
    pub LastFailedLogon: ::LARGE_INTEGER,
    pub FailedAttemptCountSinceLastSuccessfulLogon: ::ULONG,
}
pub type PLSA_LAST_INTER_LOGON_INFO = *mut LSA_LAST_INTER_LOGON_INFO;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SECURITY_LOGON_SESSION_DATA {
    pub Size: ::ULONG,
    pub LogonId: ::LUID,
    pub UserName: ::LSA_UNICODE_STRING,
    pub LogonDomain: ::LSA_UNICODE_STRING,
    pub AuthenticationPackage: ::LSA_UNICODE_STRING,
    pub LogonType: ::ULONG,
    pub Session: ::ULONG,
    pub Sid: ::PSID,
    pub LogonTime: ::LARGE_INTEGER,
    pub LogonServer: ::LSA_UNICODE_STRING,
    pub DnsDomainName: ::LSA_UNICODE_STRING,
    pub Upn: ::LSA_UNICODE_STRING,
    pub UserFlags: ::ULONG,
    pub LastLogonInfo: LSA_LAST_INTER_LOGON_INFO,
    pub LogonScript: ::LSA_UNICODE_STRING,
    pub ProfilePath: ::LSA_UNICODE_STRING,
    pub HomeDirectory: ::LSA_UNICODE_STRING,
    pub HomeDirectoryDrive: ::LSA_UNICODE_STRING,
    pub LogoffTime: ::LARGE_INTEGER,
    pub KickOffTime: ::LARGE_INTEGER,
    pub PasswordLastSet: ::LARGE_INTEGER,
    pub PasswordCanChange: ::LARGE_INTEGER,
    pub PasswordMustChange: ::LARGE_INTEGER,
}
pub type PSECURITY_LOGON_SESSION_DATA = *mut SECURITY_LOGON_SESSION_DATA;
pub const CENTRAL_ACCESS_POLICY_OWNER_RIGHTS_PRESENT_FLAG: ::ULONG = 0x00000001;
pub const CENTRAL_ACCESS_POLICY_STAGED_OWNER_RIGHTS_PRESENT_FLAG: ::ULONG = 0x00000100;
pub const CENTRAL_ACCESS_POLICY_STAGED_FLAG: ::ULONG = 0x00010000;
pub const CENTRAL_ACCESS_POLICY_VALID_FLAG_MASK: ::ULONG =
    CENTRAL_ACCESS_POLICY_OWNER_RIGHTS_PRESENT_FLAG
    | CENTRAL_ACCESS_POLICY_STAGED_OWNER_RIGHTS_PRESENT_FLAG | CENTRAL_ACCESS_POLICY_STAGED_FLAG;
pub const LSASETCAPS_RELOAD_FLAG: ::ULONG = 0x00000001;
pub const LSASETCAPS_VALID_FLAG_MASK: ::ULONG = LSASETCAPS_RELOAD_FLAG;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct CENTRAL_ACCESS_POLICY_ENTRY {
    pub Name: ::LSA_UNICODE_STRING,
    pub Description: ::LSA_UNICODE_STRING,
    pub ChangeId: ::LSA_UNICODE_STRING,
    pub LengthAppliesTo: ::ULONG,
    pub AppliesTo: ::PUCHAR,
    pub LengthSD: ::ULONG,
    pub SD: ::PSECURITY_DESCRIPTOR,
    pub LengthStagedSD: ::ULONG,
    pub StagedSD: ::PSECURITY_DESCRIPTOR,
    pub Flags: ::ULONG,
}
pub type PCENTRAL_ACCESS_POLICY_ENTRY = *mut CENTRAL_ACCESS_POLICY_ENTRY;
pub type PCCENTRAL_ACCESS_POLICY_ENTRY = *const CENTRAL_ACCESS_POLICY_ENTRY;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct CENTRAL_ACCESS_POLICY {
    pub CAPID: ::PSID,
    pub Name: ::LSA_UNICODE_STRING,
    pub Description: ::LSA_UNICODE_STRING,
    pub ChangeId: ::LSA_UNICODE_STRING,
    pub Flags: ::ULONG,
    pub CAPECount: ::ULONG,
    pub CAPEs: *mut PCENTRAL_ACCESS_POLICY_ENTRY,
}
pub type PCENTRAL_ACCESS_POLICY = *mut CENTRAL_ACCESS_POLICY;
pub type PCCENTRAL_ACCESS_POLICY = *const CENTRAL_ACCESS_POLICY;
ENUM!{enum NEGOTIATE_MESSAGES {
    NegEnumPackagePrefixes = 0,
    NegGetCallerName = 1,
    NegTransferCredentials = 2,
    NegCallPackageMax,
}}
pub const NEGOTIATE_MAX_PREFIX: usize = 32;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NEGOTIATE_PACKAGE_PREFIX {
    pub PackageId: ::ULONG_PTR,
    pub PackageDataA: ::PVOID,
    pub PackageDataW: ::PVOID,
    pub PrefixLen: ::ULONG_PTR,
    pub Prefix: [::UCHAR; NEGOTIATE_MAX_PREFIX],
}
pub type PNEGOTIATE_PACKAGE_PREFIX = *mut NEGOTIATE_PACKAGE_PREFIX;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NEGOTIATE_PACKAGE_PREFIXES {
    pub MessageType: ::ULONG,
    pub PrefixCount: ::ULONG,
    pub Offset: ::ULONG,
    pub Pad: ::ULONG,
}
pub type PNEGOTIATE_PACKAGE_PREFIXES = *mut NEGOTIATE_PACKAGE_PREFIXES;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NEGOTIATE_CALLER_NAME_REQUEST {
    pub MessageType: ::ULONG,
    pub LogonId: ::LUID,
}
pub type PNEGOTIATE_CALLER_NAME_REQUEST = *mut NEGOTIATE_CALLER_NAME_REQUEST;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NEGOTIATE_CALLER_NAME_RESPONSE {
    pub MessageType: ::ULONG,
    pub CallerName: ::PWSTR,
}
pub type PNEGOTIATE_CALLER_NAME_RESPONSE = *mut NEGOTIATE_CALLER_NAME_RESPONSE;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DOMAIN_PASSWORD_INFORMATION {
    pub MinPasswordLength: ::USHORT,
    pub PasswordHistoryLength: ::USHORT,
    pub PasswordProperties: ::ULONG,
    pub MaxPasswordAge: ::LARGE_INTEGER,
    pub MinPasswordAge: ::LARGE_INTEGER,
}
pub type PDOMAIN_PASSWORD_INFORMATION = *mut DOMAIN_PASSWORD_INFORMATION;
pub const DOMAIN_PASSWORD_COMPLEX: ::ULONG = 0x00000001;
pub const DOMAIN_PASSWORD_NO_ANON_CHANGE: ::ULONG = 0x00000002;
pub const DOMAIN_PASSWORD_NO_CLEAR_CHANGE: ::ULONG = 0x00000004;
pub const DOMAIN_LOCKOUT_ADMINS: ::ULONG = 0x00000008;
pub const DOMAIN_PASSWORD_STORE_CLEARTEXT: ::ULONG = 0x00000010;
pub const DOMAIN_REFUSE_PASSWORD_CHANGE: ::ULONG = 0x00000020;
pub const DOMAIN_NO_LM_OWF_CHANGE: ::ULONG = 0x00000040;
pub type PSAM_PASSWORD_NOTIFICATION_ROUTINE = Option<unsafe extern "system" fn(
    UserName: ::PUNICODE_STRING, RelativeId: ::ULONG, NewPassword: ::PUNICODE_STRING,
) -> ::NTSTATUS>;
pub type PSAM_INIT_NOTIFICATION_ROUTINE = Option<unsafe extern "system" fn() -> ::BOOLEAN>;
pub type PSAM_PASSWORD_FILTER_ROUTINE = Option<unsafe extern "system" fn(
    AccountName: ::PUNICODE_STRING, FullName: ::PUNICODE_STRING, Password: ::PUNICODE_STRING,
    SetOperation: ::BOOLEAN,
) -> ::BOOLEAN>;
ENUM!{enum MSV1_0_LOGON_SUBMIT_TYPE {
    MsV1_0InteractiveLogon = 2,
    MsV1_0Lm20Logon,
    MsV1_0NetworkLogon,
    MsV1_0SubAuthLogon,
    MsV1_0WorkstationUnlockLogon = 7,
    MsV1_0S4ULogon = 12,
    MsV1_0VirtualLogon = 82,
    MsV1_0NoElevationLogon = 83,
    MsV1_0LuidLogon = 84,
}}
pub type PMSV1_0_LOGON_SUBMIT_TYPE = *mut MSV1_0_LOGON_SUBMIT_TYPE;
ENUM!{enum MSV1_0_PROFILE_BUFFER_TYPE {
    MsV1_0InteractiveProfile = 2,
    MsV1_0Lm20LogonProfile,
    MsV1_0SmartCardProfile,
}}
pub type PMSV1_0_PROFILE_BUFFER_TYPE = *mut MSV1_0_PROFILE_BUFFER_TYPE;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct MSV1_0_INTERACTIVE_LOGON {
    pub MessageType: MSV1_0_LOGON_SUBMIT_TYPE,
    pub LogonDomainName: ::UNICODE_STRING,
    pub UserName: ::UNICODE_STRING,
    pub Password: ::UNICODE_STRING,
}
pub type PMSV1_0_INTERACTIVE_LOGON = *mut MSV1_0_INTERACTIVE_LOGON;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct MSV1_0_INTERACTIVE_PROFILE {
    pub MessageType: MSV1_0_PROFILE_BUFFER_TYPE,
    pub LogonCount: ::USHORT,
    pub BadPasswordCount: ::USHORT,
    pub LogonTime: ::LARGE_INTEGER,
    pub LogoffTime: ::LARGE_INTEGER,
    pub KickOffTime: ::LARGE_INTEGER,
    pub PasswordLastSet: ::LARGE_INTEGER,
    pub PasswordCanChange: ::LARGE_INTEGER,
    pub PasswordMustChange: ::LARGE_INTEGER,
    pub LogonScript: ::UNICODE_STRING,
    pub HomeDirectory: ::UNICODE_STRING,
    pub FullName: ::UNICODE_STRING,
    pub ProfilePath: ::UNICODE_STRING,
    pub HomeDirectoryDrive: ::UNICODE_STRING,
    pub LogonServer: ::UNICODE_STRING,
    pub UserFlags: ::ULONG,
}
pub type PMSV1_0_INTERACTIVE_PROFILE = *mut MSV1_0_INTERACTIVE_PROFILE;
pub const MSV1_0_CHALLENGE_LENGTH: usize = 8;
pub const MSV1_0_USER_SESSION_KEY_LENGTH: usize = 16;
pub const MSV1_0_LANMAN_SESSION_KEY_LENGTH: usize = 8;
pub const MSV1_0_CLEARTEXT_PASSWORD_ALLOWED: ::ULONG = 0x02;
pub const MSV1_0_UPDATE_LOGON_STATISTICS: ::ULONG = 0x04;
pub const MSV1_0_RETURN_USER_PARAMETERS: ::ULONG = 0x08;
pub const MSV1_0_DONT_TRY_GUEST_ACCOUNT: ::ULONG = 0x10;
pub const MSV1_0_ALLOW_SERVER_TRUST_ACCOUNT: ::ULONG = 0x20;
pub const MSV1_0_RETURN_PASSWORD_EXPIRY: ::ULONG = 0x40;
pub const MSV1_0_USE_CLIENT_CHALLENGE: ::ULONG = 0x80;
pub const MSV1_0_TRY_GUEST_ACCOUNT_ONLY: ::ULONG = 0x100;
pub const MSV1_0_RETURN_PROFILE_PATH: ::ULONG = 0x200;
pub const MSV1_0_TRY_SPECIFIED_DOMAIN_ONLY: ::ULONG = 0x400;
pub const MSV1_0_ALLOW_WORKSTATION_TRUST_ACCOUNT: ::ULONG = 0x800;
pub const MSV1_0_DISABLE_PERSONAL_FALLBACK: ::ULONG = 0x00001000;
pub const MSV1_0_ALLOW_FORCE_GUEST: ::ULONG = 0x00002000;
pub const MSV1_0_CLEARTEXT_PASSWORD_SUPPLIED: ::ULONG = 0x00004000;
pub const MSV1_0_USE_DOMAIN_FOR_ROUTING_ONLY: ::ULONG = 0x00008000;
pub const MSV1_0_SUBAUTHENTICATION_DLL_EX: ::ULONG = 0x00100000;
pub const MSV1_0_ALLOW_MSVCHAPV2: ::ULONG = 0x00010000;
pub const MSV1_0_S4U2SELF: ::ULONG = 0x00020000;
pub const MSV1_0_CHECK_LOGONHOURS_FOR_S4U: ::ULONG = 0x00040000;
pub const MSV1_0_INTERNET_DOMAIN: ::ULONG = 0x00080000;
pub const MSV1_0_SUBAUTHENTICATION_DLL: ::ULONG = 0xFF000000;
pub const MSV1_0_SUBAUTHENTICATION_DLL_SHIFT: ::ULONG = 24;
pub const MSV1_0_MNS_LOGON: ::ULONG = 0x01000000;
pub const MSV1_0_SUBAUTHENTICATION_DLL_RAS: ::ULONG = 2;
pub const MSV1_0_SUBAUTHENTICATION_DLL_IIS: ::ULONG = 132;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct MSV1_0_LM20_LOGON {
    pub MessageType: MSV1_0_LOGON_SUBMIT_TYPE,
    pub LogonDomainName: ::UNICODE_STRING,
    pub UserName: ::UNICODE_STRING,
    pub Workstation: ::UNICODE_STRING,
    pub ChallengeToClient: [::UCHAR; MSV1_0_CHALLENGE_LENGTH],
    pub CaseSensitiveChallengeResponse: ::STRING,
    pub CaseInsensitiveChallengeResponse: ::STRING,
    pub ParameterControl: ::ULONG,
}
pub type PMSV1_0_LM20_LOGON = *mut MSV1_0_LM20_LOGON;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct MSV1_0_SUBAUTH_LOGON {
    pub MessageType: MSV1_0_LOGON_SUBMIT_TYPE,
    pub LogonDomainName: ::UNICODE_STRING,
    pub UserName: ::UNICODE_STRING,
    pub Workstation: ::UNICODE_STRING,
    pub ChallengeToClient: [::UCHAR; MSV1_0_CHALLENGE_LENGTH],
    pub AuthenticationInfo1: ::STRING,
    pub AuthenticationInfo2: ::STRING,
    pub ParameterControl: ::ULONG,
    pub SubAuthPackageId: ::ULONG,
}
pub type PMSV1_0_SUBAUTH_LOGON = *mut MSV1_0_SUBAUTH_LOGON;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct MSV1_0_S4U_LOGON {
    pub MessageType: MSV1_0_LOGON_SUBMIT_TYPE,
    pub MSV1_0_LOGON_SUBMIT_TYPE: ::ULONG,
    pub UserPrincipalName: ::UNICODE_STRING,
    pub DomainName: ::UNICODE_STRING,
}
pub type PMSV1_0_S4U_LOGON = *mut MSV1_0_S4U_LOGON;
pub const LOGON_GUEST: ::ULONG = 0x01;
pub const LOGON_NOENCRYPTION: ::ULONG = 0x02;
pub const LOGON_CACHED_ACCOUNT: ::ULONG = 0x04;
pub const LOGON_USED_LM_PASSWORD: ::ULONG = 0x08;
pub const LOGON_EXTRA_SIDS: ::ULONG = 0x20;
pub const LOGON_SUBAUTH_SESSION_KEY: ::ULONG = 0x40;
pub const LOGON_SERVER_TRUST_ACCOUNT: ::ULONG = 0x80;
pub const LOGON_NTLMV2_ENABLED: ::ULONG = 0x100;
pub const LOGON_RESOURCE_GROUPS: ::ULONG = 0x200;
pub const LOGON_PROFILE_PATH_RETURNED: ::ULONG = 0x400;
pub const LOGON_NT_V2: ::ULONG = 0x800;
pub const LOGON_LM_V2: ::ULONG = 0x1000;
pub const LOGON_NTLM_V2: ::ULONG = 0x2000;
pub const LOGON_OPTIMIZED: ::ULONG = 0x4000;
pub const LOGON_WINLOGON: ::ULONG = 0x8000;
pub const LOGON_PKINIT: ::ULONG = 0x10000;
pub const LOGON_NO_OPTIMIZED: ::ULONG = 0x20000;
pub const LOGON_NO_ELEVATION: ::ULONG = 0x40000;
pub const LOGON_MANAGED_SERVICE: ::ULONG = 0x80000;
pub const LOGON_GRACE_LOGON: ::ULONG = 0x01000000;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct MSV1_0_LM20_LOGON_PROFILE {
    pub MessageType: MSV1_0_PROFILE_BUFFER_TYPE,
    pub KickOffTime: ::LARGE_INTEGER,
    pub LogoffTime: ::LARGE_INTEGER,
    pub UserFlags: ::ULONG,
    pub UserSessionKey: [::UCHAR; MSV1_0_USER_SESSION_KEY_LENGTH],
    pub LogonDomainName: ::UNICODE_STRING,
    pub LanmanSessionKey: [::UCHAR; MSV1_0_LANMAN_SESSION_KEY_LENGTH],
    pub LogonServer: ::UNICODE_STRING,
    pub UserParameters: ::UNICODE_STRING,
}
pub type PMSV1_0_LM20_LOGON_PROFILE = *mut MSV1_0_LM20_LOGON_PROFILE;
pub const MSV1_0_OWF_PASSWORD_LENGTH: usize = 16;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct MSV1_0_SUPPLEMENTAL_CREDENTIAL {
    pub Version: ::ULONG,
    pub Flags: ::ULONG,
    pub LmPassword: [::UCHAR; MSV1_0_OWF_PASSWORD_LENGTH],
    pub NtPassword: [::UCHAR; MSV1_0_OWF_PASSWORD_LENGTH],
}
pub type PMSV1_0_SUPPLEMENTAL_CREDENTIAL = *mut MSV1_0_SUPPLEMENTAL_CREDENTIAL;
pub const MSV1_0_NTLM3_RESPONSE_LENGTH: usize = 16;
pub const MSV1_0_NTLM3_OWF_LENGTH: usize = 16;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct MSV1_0_NTLM3_RESPONSE {
    pub Response: [::UCHAR; MSV1_0_NTLM3_RESPONSE_LENGTH],
    pub RespType: ::UCHAR,
    pub HiRespType: ::UCHAR,
    pub Flags: ::USHORT,
    pub MsgWord: ::ULONG,
    pub TimeStamp: ::ULONGLONG,
    pub ChallengeFromClient: [::UCHAR; MSV1_0_CHALLENGE_LENGTH],
    pub AvPairsOff: ::ULONG,
    pub Buffer: [::UCHAR; 1],
}
pub type PMSV1_0_NTLM3_RESPONSE = *mut MSV1_0_NTLM3_RESPONSE;
ENUM!{enum MSV1_0_AVID {
    MsvAvEOL,
    MsvAvNbComputerName,
    MsvAvNbDomainName,
    MsvAvDnsComputerName,
    MsvAvDnsDomainName,
    MsvAvDnsTreeName,
    MsvAvFlags,
    MsvAvTimestamp,
    MsvAvRestrictions,
    MsvAvTargetName,
    MsvAvChannelBindings,
}}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct MSV1_0_AV_PAIR {
    pub AvId: ::USHORT,
    pub AvLen: ::USHORT,
}
pub type PMSV1_0_AV_PAIR = *mut MSV1_0_AV_PAIR;
ENUM!{enum MSV1_0_PROTOCOL_MESSAGE_TYPE {
    MsV1_0Lm20ChallengeRequest = 0,
    MsV1_0Lm20GetChallengeResponse,
    MsV1_0EnumerateUsers,
    MsV1_0GetUserInfo,
    MsV1_0ReLogonUsers,
    MsV1_0ChangePassword,
    MsV1_0ChangeCachedPassword,
    MsV1_0GenericPassthrough,
    MsV1_0CacheLogon,
    MsV1_0SubAuth,
    MsV1_0DeriveCredential,
    MsV1_0CacheLookup,
    MsV1_0SetProcessOption,
    MsV1_0ConfigLocalAliases,
    MsV1_0ClearCachedCredentials,
    MsV1_0LookupToken,
    MsV1_0ValidateAuth,
    MsV1_0CacheLookupEx,
    MsV1_0GetCredentialKey,
    MsV1_0SetThreadOption,
}}
pub type PMSV1_0_PROTOCOL_MESSAGE_TYPE = *mut MSV1_0_PROTOCOL_MESSAGE_TYPE;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct MSV1_0_CHANGEPASSWORD_REQUEST {
    pub MessageType: MSV1_0_PROTOCOL_MESSAGE_TYPE,
    pub DomainName: ::UNICODE_STRING,
    pub AccountName: ::UNICODE_STRING,
    pub OldPassword: ::UNICODE_STRING,
    pub NewPassword: ::UNICODE_STRING,
    pub Impersonating: ::BOOLEAN,
}
pub type PMSV1_0_CHANGEPASSWORD_REQUEST = *mut MSV1_0_CHANGEPASSWORD_REQUEST;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct MSV1_0_CHANGEPASSWORD_RESPONSE {
    pub MessageType: MSV1_0_PROTOCOL_MESSAGE_TYPE,
    pub PasswordInfoValid: ::BOOLEAN,
    pub DomainPasswordInfo: DOMAIN_PASSWORD_INFORMATION,
}
pub type PMSV1_0_CHANGEPASSWORD_RESPONSE = *mut MSV1_0_CHANGEPASSWORD_RESPONSE;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct MSV1_0_PASSTHROUGH_REQUEST {
    pub MessageType: MSV1_0_PROTOCOL_MESSAGE_TYPE,
    pub DomainName: ::UNICODE_STRING,
    pub PackageName: ::UNICODE_STRING,
    pub DataLength: ::ULONG,
    pub LogonData: ::PUCHAR,
    pub Pad: ::ULONG,
}
pub type PMSV1_0_PASSTHROUGH_REQUEST = *mut MSV1_0_PASSTHROUGH_REQUEST;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct MSV1_0_PASSTHROUGH_RESPONSE {
    pub MessageType: MSV1_0_PROTOCOL_MESSAGE_TYPE,
    pub Pad: ::ULONG,
    pub DataLength: ::ULONG,
    pub ValidationData: ::PUCHAR,
}
pub type PMSV1_0_PASSTHROUGH_RESPONSE = *mut MSV1_0_PASSTHROUGH_RESPONSE;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct MSV1_0_SUBAUTH_REQUEST {
    pub MessageType: MSV1_0_PROTOCOL_MESSAGE_TYPE,
    pub SubAuthPackageId: ::ULONG,
    pub SubAuthInfoLength: ::ULONG,
    pub SubAuthSubmitBuffer: ::PUCHAR,
}
pub type PMSV1_0_SUBAUTH_REQUEST = *mut MSV1_0_SUBAUTH_REQUEST;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct MSV1_0_SUBAUTH_RESPONSE {
    pub MessageType: MSV1_0_PROTOCOL_MESSAGE_TYPE,
    pub SubAuthInfoLength: ::ULONG,
    pub SubAuthReturnBuffer: ::PUCHAR,
}
pub type PMSV1_0_SUBAUTH_RESPONSE = *mut MSV1_0_SUBAUTH_RESPONSE;
pub const RTL_ENCRYPT_MEMORY_SIZE: ::ULONG = 8;
pub const RTL_ENCRYPT_OPTION_CROSS_PROCESS: ::ULONG = 0x01;
pub const RTL_ENCRYPT_OPTION_SAME_LOGON: ::ULONG = 0x02;
pub const KERB_ETYPE_NULL: ::LONG = 0;
pub const KERB_ETYPE_DES_CBC_CRC: ::LONG = 1;
pub const KERB_ETYPE_DES_CBC_MD4: ::LONG = 2;
pub const KERB_ETYPE_DES_CBC_MD5: ::LONG = 3;
pub const KERB_ETYPE_AES128_CTS_HMAC_SHA1_96: ::LONG = 17;
pub const KERB_ETYPE_AES256_CTS_HMAC_SHA1_96: ::LONG = 18;
pub const KERB_ETYPE_RC4_MD4: ::LONG = -128;
pub const KERB_ETYPE_RC4_PLAIN2: ::LONG = -129;
pub const KERB_ETYPE_RC4_LM: ::LONG = -130;
pub const KERB_ETYPE_RC4_SHA: ::LONG = -131;
pub const KERB_ETYPE_DES_PLAIN: ::LONG = -132;
pub const KERB_ETYPE_RC4_HMAC_OLD: ::LONG = -133;
pub const KERB_ETYPE_RC4_PLAIN_OLD: ::LONG = -134;
pub const KERB_ETYPE_RC4_HMAC_OLD_EXP: ::LONG = -135;
pub const KERB_ETYPE_RC4_PLAIN_OLD_EXP: ::LONG = -136;
pub const KERB_ETYPE_RC4_PLAIN: ::LONG = -140;
pub const KERB_ETYPE_RC4_PLAIN_EXP: ::LONG = -141;
pub const KERB_ETYPE_AES128_CTS_HMAC_SHA1_96_PLAIN: ::LONG = -148;
pub const KERB_ETYPE_AES256_CTS_HMAC_SHA1_96_PLAIN: ::LONG = -149;
pub const KERB_ETYPE_DSA_SHA1_CMS: ::LONG = 9;
pub const KERB_ETYPE_RSA_MD5_CMS: ::LONG = 10;
pub const KERB_ETYPE_RSA_SHA1_CMS: ::LONG = 11;
pub const KERB_ETYPE_RC2_CBC_ENV: ::LONG = 12;
pub const KERB_ETYPE_RSA_ENV: ::LONG = 13;
pub const KERB_ETYPE_RSA_ES_OEAP_ENV: ::LONG = 14;
pub const KERB_ETYPE_DES_EDE3_CBC_ENV: ::LONG = 15;
pub const KERB_ETYPE_DSA_SIGN: ::LONG = 8;
pub const KERB_ETYPE_RSA_PRIV: ::LONG = 9;
pub const KERB_ETYPE_RSA_PUB: ::LONG = 10;
pub const KERB_ETYPE_RSA_PUB_MD5: ::LONG = 11;
pub const KERB_ETYPE_RSA_PUB_SHA1: ::LONG = 12;
pub const KERB_ETYPE_PKCS7_PUB: ::LONG = 13;
pub const KERB_ETYPE_DES3_CBC_MD5: ::LONG = 5;
pub const KERB_ETYPE_DES3_CBC_SHA1: ::LONG = 7;
pub const KERB_ETYPE_DES3_CBC_SHA1_KD: ::LONG = 16;
pub const KERB_ETYPE_DES_CBC_MD5_NT: ::LONG = 20;
pub const KERB_ETYPE_RC4_HMAC_NT: ::LONG = 23;
pub const KERB_ETYPE_RC4_HMAC_NT_EXP: ::LONG = 24;
pub const KERB_CHECKSUM_NONE: ::LONG = 0;
pub const KERB_CHECKSUM_CRC32: ::LONG = 1;
pub const KERB_CHECKSUM_MD4: ::LONG = 2;
pub const KERB_CHECKSUM_KRB_DES_MAC: ::LONG = 4;
pub const KERB_CHECKSUM_KRB_DES_MAC_K: ::LONG = 5;
pub const KERB_CHECKSUM_MD5: ::LONG = 7;
pub const KERB_CHECKSUM_MD5_DES: ::LONG = 8;
pub const KERB_CHECKSUM_SHA1_NEW: ::LONG = 14;
pub const KERB_CHECKSUM_HMAC_SHA1_96_AES128: ::LONG = 15;
pub const KERB_CHECKSUM_HMAC_SHA1_96_AES256: ::LONG = 16;
pub const KERB_CHECKSUM_LM: ::LONG = -130;
pub const KERB_CHECKSUM_SHA1: ::LONG = -131;
pub const KERB_CHECKSUM_REAL_CRC32: ::LONG = -132;
pub const KERB_CHECKSUM_DES_MAC: ::LONG = -133;
pub const KERB_CHECKSUM_DES_MAC_MD5: ::LONG = -134;
pub const KERB_CHECKSUM_MD25: ::LONG = -135;
pub const KERB_CHECKSUM_RC4_MD5: ::LONG = -136;
pub const KERB_CHECKSUM_MD5_HMAC: ::LONG = -137;
pub const KERB_CHECKSUM_HMAC_MD5: ::LONG = -138;
pub const KERB_CHECKSUM_HMAC_SHA1_96_AES128_Ki: ::LONG = -150;
pub const KERB_CHECKSUM_HMAC_SHA1_96_AES256_Ki: ::LONG = -151;
pub const KERB_TICKET_FLAGS_reserved: ::ULONG = 0x80000000;
pub const KERB_TICKET_FLAGS_forwardable: ::ULONG = 0x40000000;
pub const KERB_TICKET_FLAGS_forwarded: ::ULONG = 0x20000000;
pub const KERB_TICKET_FLAGS_proxiable: ::ULONG = 0x10000000;
pub const KERB_TICKET_FLAGS_proxy: ::ULONG = 0x08000000;
pub const KERB_TICKET_FLAGS_may_postdate: ::ULONG = 0x04000000;
pub const KERB_TICKET_FLAGS_postdated: ::ULONG = 0x02000000;
pub const KERB_TICKET_FLAGS_invalid: ::ULONG = 0x01000000;
pub const KERB_TICKET_FLAGS_renewable: ::ULONG = 0x00800000;
pub const KERB_TICKET_FLAGS_initial: ::ULONG = 0x00400000;
pub const KERB_TICKET_FLAGS_pre_authent: ::ULONG = 0x00200000;
pub const KERB_TICKET_FLAGS_hw_authent: ::ULONG = 0x00100000;
pub const KERB_TICKET_FLAGS_ok_as_delegate: ::ULONG = 0x00040000;
pub const KERB_TICKET_FLAGS_name_canonicalize: ::ULONG = 0x00010000;
pub const KERB_TICKET_FLAGS_cname_in_pa_data: ::ULONG = 0x00040000;
pub const KERB_TICKET_FLAGS_enc_pa_rep: ::ULONG = 0x00010000;
pub const KERB_TICKET_FLAGS_reserved1: ::ULONG = 0x00000001;
pub const KRB_NT_UNKNOWN: ::LONG = 0;
pub const KRB_NT_PRINCIPAL: ::LONG = 1;
pub const KRB_NT_PRINCIPAL_AND_ID: ::LONG = -131;
pub const KRB_NT_SRV_INST: ::LONG = 2;
pub const KRB_NT_SRV_INST_AND_ID: ::LONG = -132;
pub const KRB_NT_SRV_HST: ::LONG = 3;
pub const KRB_NT_SRV_XHST: ::LONG = 4;
pub const KRB_NT_UID: ::LONG = 5;
pub const KRB_NT_ENTERPRISE_PRINCIPAL: ::LONG = 10;
pub const KRB_NT_WELLKNOWN: ::LONG = 11;
pub const KRB_NT_ENT_PRINCIPAL_AND_ID: ::LONG = -130;
pub const KRB_NT_MS_PRINCIPAL: ::LONG = -128;
pub const KRB_NT_MS_PRINCIPAL_AND_ID: ::LONG = -129;
pub const KRB_NT_MS_BRANCH_ID: ::LONG = -133;
pub const KRB_NT_X500_PRINCIPAL: ::LONG = 6;
pub const KERB_WRAP_NO_ENCRYPT: ::ULONG = 0x80000001;
ENUM!{enum KERB_LOGON_SUBMIT_TYPE {
    KerbInteractiveLogon = 2,
    KerbSmartCardLogon = 6,
    KerbWorkstationUnlockLogon = 7,
    KerbSmartCardUnlockLogon = 8,
    KerbProxyLogon = 9,
    KerbTicketLogon = 10,
    KerbTicketUnlockLogon = 11,
    KerbS4ULogon = 12,
    KerbCertificateLogon = 13,
    KerbCertificateS4ULogon = 14,
    KerbCertificateUnlockLogon = 15,
    KerbNoElevationLogon = 83,
    KerbLuidLogon = 84,
}}
pub type PKERB_LOGON_SUBMIT_TYPE = *mut KERB_LOGON_SUBMIT_TYPE;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct KERB_INTERACTIVE_LOGON {
    pub MessageType: KERB_LOGON_SUBMIT_TYPE,
    pub LogonDomainName: ::UNICODE_STRING,
    pub UserName: ::UNICODE_STRING,
    pub Password: ::UNICODE_STRING,
}
pub type PKERB_INTERACTIVE_LOGON = *mut KERB_INTERACTIVE_LOGON;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct KERB_INTERACTIVE_UNLOCK_LOGON {
    pub Logon: KERB_INTERACTIVE_LOGON,
    pub LogonId: ::LUID,
}
pub type PKERB_INTERACTIVE_UNLOCK_LOGON = *mut KERB_INTERACTIVE_UNLOCK_LOGON;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct KERB_SMART_CARD_LOGON {
    pub MessageType: KERB_LOGON_SUBMIT_TYPE,
    pub Pin: ::UNICODE_STRING,
    pub CspDataLength: ::ULONG,
    pub CspData: ::PUCHAR,
}
pub type PKERB_SMART_CARD_LOGON = *mut KERB_SMART_CARD_LOGON;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct KERB_SMART_CARD_UNLOCK_LOGON {
    pub Logon: KERB_SMART_CARD_LOGON,
    pub LogonId: ::LUID,
}
pub type PKERB_SMART_CARD_UNLOCK_LOGON = *mut KERB_SMART_CARD_UNLOCK_LOGON;
pub const KERB_CERTIFICATE_LOGON_FLAG_CHECK_DUPLICATES: ::ULONG = 0x1;
pub const KERB_CERTIFICATE_LOGON_FLAG_USE_CERTIFICATE_INFO: ::ULONG = 0x2;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct KERB_CERTIFICATE_LOGON {
    pub MessageType: KERB_LOGON_SUBMIT_TYPE,
    pub DomainName: ::UNICODE_STRING,
    pub UserName: ::UNICODE_STRING,
    pub Pin: ::UNICODE_STRING,
    pub Flags: ::ULONG,
    pub CspDataLength: ::ULONG,
    pub CspData: ::PUCHAR,
}
pub type PKERB_CERTIFICATE_LOGON = *mut KERB_CERTIFICATE_LOGON;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct KERB_CERTIFICATE_UNLOCK_LOGON {
    pub Logon: KERB_CERTIFICATE_LOGON,
    pub LogonId: ::LUID,
}
pub type PKERB_CERTIFICATE_UNLOCK_LOGON = *mut KERB_CERTIFICATE_UNLOCK_LOGON;
pub const KERB_CERTIFICATE_S4U_LOGON_FLAG_CHECK_DUPLICATES: ::ULONG = 0x1;
pub const KERB_CERTIFICATE_S4U_LOGON_FLAG_CHECK_LOGONHOURS: ::ULONG = 0x2;
pub const KERB_CERTIFICATE_S4U_LOGON_FLAG_FAIL_IF_NT_AUTH_POLICY_REQUIRED: ::ULONG = 0x4;
pub const KERB_CERTIFICATE_S4U_LOGON_FLAG_IDENTIFY: ::ULONG = 0x8;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct KERB_CERTIFICATE_S4U_LOGON {
    pub MessageType: KERB_LOGON_SUBMIT_TYPE,
    pub Flags: ::ULONG,
    pub UserPrincipalName: ::UNICODE_STRING,
    pub DomainName: ::UNICODE_STRING,
    pub CertificateLength: ::ULONG,
    pub Certificate: ::PUCHAR,
}
pub type PKERB_CERTIFICATE_S4U_LOGON = *mut KERB_CERTIFICATE_S4U_LOGON;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct KERB_TICKET_LOGON {
    pub MessageType: KERB_LOGON_SUBMIT_TYPE,
    pub Flags: ::ULONG,
    pub ServiceTicketLength: ::ULONG,
    pub TicketGrantingTicketLength: ::ULONG,
    pub ServiceTicket: ::PUCHAR,
    pub TicketGrantingTicket: ::PUCHAR,
}
pub type PKERB_TICKET_LOGON = *mut KERB_TICKET_LOGON;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct KERB_TICKET_UNLOCK_LOGON {
    pub Logon: KERB_TICKET_LOGON,
    pub LogonId: ::LUID,
}
pub type PKERB_TICKET_UNLOCK_LOGON = *mut KERB_TICKET_UNLOCK_LOGON;
pub const KERB_S4U_LOGON_FLAG_CHECK_LOGONHOURS: ::ULONG = 0x2;
pub const KERB_S4U_LOGON_FLAG_IDENTIFY: ::ULONG = 0x8;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct KERB_S4U_LOGON {
    pub MessageType: KERB_LOGON_SUBMIT_TYPE,
    pub Flags: ::ULONG,
    pub ClientUpn: ::UNICODE_STRING,
    pub ClientRealm: ::UNICODE_STRING,
}
pub type PKERB_S4U_LOGON = *mut KERB_S4U_LOGON;
ENUM!{enum KERB_PROFILE_BUFFER_TYPE {
    KerbInteractiveProfile = 2,
    KerbSmartCardProfile = 4,
    KerbTicketProfile = 6,
}}
pub type PKERB_PROFILE_BUFFER_TYPE = *mut KERB_PROFILE_BUFFER_TYPE;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct KERB_INTERACTIVE_PROFILE {
    pub MessageType: KERB_PROFILE_BUFFER_TYPE,
    pub LogonCount: ::USHORT,
    pub BadPasswordCount: ::USHORT,
    pub LogonTime: ::LARGE_INTEGER,
    pub LogoffTime: ::LARGE_INTEGER,
    pub KickOffTime: ::LARGE_INTEGER,
    pub PasswordLastSet: ::LARGE_INTEGER,
    pub PasswordCanChange: ::LARGE_INTEGER,
    pub PasswordMustChange: ::LARGE_INTEGER,
    pub LogonScript: ::UNICODE_STRING,
    pub HomeDirectory: ::UNICODE_STRING,
    pub FullName: ::UNICODE_STRING,
    pub ProfilePath: ::UNICODE_STRING,
    pub HomeDirectoryDrive: ::UNICODE_STRING,
    pub LogonServer: ::UNICODE_STRING,
    pub UserFlags: ::ULONG,
}
pub type PKERB_INTERACTIVE_PROFILE = *mut KERB_INTERACTIVE_PROFILE;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct KERB_SMART_CARD_PROFILE {
    pub Profile: KERB_INTERACTIVE_PROFILE,
    pub CertificateSize: ::ULONG,
    pub CertificateData: ::PUCHAR,
}
pub type PKERB_SMART_CARD_PROFILE = *mut KERB_SMART_CARD_PROFILE;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct KERB_CRYPTO_KEY {
    pub KeyType: ::LONG,
    pub Length: ::ULONG,
    pub Value: ::PUCHAR,
}
pub type PKERB_CRYPTO_KEY = *mut KERB_CRYPTO_KEY;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct KERB_CRYPTO_KEY32 {
    pub KeyType: ::LONG,
    pub Length: ::ULONG,
    pub Offset: ::ULONG,
}
pub type PKERB_CRYPTO_KEY32 = *mut KERB_CRYPTO_KEY32;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct KERB_TICKET_PROFILE {
    pub Profile: KERB_INTERACTIVE_PROFILE,
    pub SessionKey: KERB_CRYPTO_KEY,
}
pub type PKERB_TICKET_PROFILE = *mut KERB_TICKET_PROFILE;
ENUM!{enum KERB_PROTOCOL_MESSAGE_TYPE {
    KerbDebugRequestMessage = 0,
    KerbQueryTicketCacheMessage,
    KerbChangeMachinePasswordMessage,
    KerbVerifyPacMessage,
    KerbRetrieveTicketMessage,
    KerbUpdateAddressesMessage,
    KerbPurgeTicketCacheMessage,
    KerbChangePasswordMessage,
    KerbRetrieveEncodedTicketMessage,
    KerbDecryptDataMessage,
    KerbAddBindingCacheEntryMessage,
    KerbSetPasswordMessage,
    KerbSetPasswordExMessage,
    KerbVerifyCredentialsMessage,
    KerbQueryTicketCacheExMessage,
    KerbPurgeTicketCacheExMessage,
    KerbRefreshSmartcardCredentialsMessage,
    KerbAddExtraCredentialsMessage,
    KerbQuerySupplementalCredentialsMessage,
    KerbTransferCredentialsMessage,
    KerbQueryTicketCacheEx2Message,
    KerbSubmitTicketMessage,
    KerbAddExtraCredentialsExMessage,
    KerbQueryKdcProxyCacheMessage,
    KerbPurgeKdcProxyCacheMessage,
    KerbQueryTicketCacheEx3Message,
    KerbCleanupMachinePkinitCredsMessage,
    KerbAddBindingCacheEntryExMessage,
    KerbQueryBindingCacheMessage,
    KerbPurgeBindingCacheMessage,
    KerbPinKdcMessage,
    KerbUnpinAllKdcsMessage,
    KerbQueryDomainExtendedPoliciesMessage,
    KerbQueryS4U2ProxyCacheMessage,
}}
pub type PKERB_PROTOCOL_MESSAGE_TYPE = *mut KERB_PROTOCOL_MESSAGE_TYPE;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct KERB_QUERY_TKT_CACHE_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub LogonId: ::LUID,
}
pub type PKERB_QUERY_TKT_CACHE_REQUEST = *mut KERB_QUERY_TKT_CACHE_REQUEST;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct KERB_TICKET_CACHE_INFO {
    pub ServerName: ::UNICODE_STRING,
    pub RealmName: ::UNICODE_STRING,
    pub StartTime: ::LARGE_INTEGER,
    pub EndTime: ::LARGE_INTEGER,
    pub RenewTime: ::LARGE_INTEGER,
    pub EncryptionType: ::LONG,
    pub TicketFlags: ::ULONG,
}
pub type PKERB_TICKET_CACHE_INFO = *mut KERB_TICKET_CACHE_INFO;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct KERB_TICKET_CACHE_INFO_EX {
    pub ClientName: ::UNICODE_STRING,
    pub ClientRealm: ::UNICODE_STRING,
    pub ServerName: ::UNICODE_STRING,
    pub ServerRealm: ::UNICODE_STRING,
    pub StartTime: ::LARGE_INTEGER,
    pub EndTime: ::LARGE_INTEGER,
    pub RenewTime: ::LARGE_INTEGER,
    pub EncryptionType: ::LONG,
    pub TicketFlags: ::ULONG,
}
pub type PKERB_TICKET_CACHE_INFO_EX = *mut KERB_TICKET_CACHE_INFO_EX;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct KERB_TICKET_CACHE_INFO_EX2 {
    pub ClientName: ::UNICODE_STRING,
    pub ClientRealm: ::UNICODE_STRING,
    pub ServerName: ::UNICODE_STRING,
    pub ServerRealm: ::UNICODE_STRING,
    pub StartTime: ::LARGE_INTEGER,
    pub EndTime: ::LARGE_INTEGER,
    pub RenewTime: ::LARGE_INTEGER,
    pub EncryptionType: ::LONG,
    pub TicketFlags: ::ULONG,
    pub SessionKeyType: ::ULONG,
    pub BranchId: ::ULONG,
}
pub type PKERB_TICKET_CACHE_INFO_EX2 = *mut KERB_TICKET_CACHE_INFO_EX2;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct KERB_TICKET_CACHE_INFO_EX3 {
    pub ClientName: ::UNICODE_STRING,
    pub ClientRealm: ::UNICODE_STRING,
    pub ServerName: ::UNICODE_STRING,
    pub ServerRealm: ::UNICODE_STRING,
    pub StartTime: ::LARGE_INTEGER,
    pub EndTime: ::LARGE_INTEGER,
    pub RenewTime: ::LARGE_INTEGER,
    pub EncryptionType: ::LONG,
    pub TicketFlags: ::ULONG,
    pub SessionKeyType: ::ULONG,
    pub BranchId: ::ULONG,
    pub CacheFlags: ::ULONG,
    pub KdcCalled: ::UNICODE_STRING,
}
pub type PKERB_TICKET_CACHE_INFO_EX3 = *mut KERB_TICKET_CACHE_INFO_EX3;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct KERB_QUERY_TKT_CACHE_RESPONSE {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub CountOfTickets: ::ULONG,
    pub Tickets: [KERB_TICKET_CACHE_INFO; ::ANYSIZE_ARRAY],
}
pub type PKERB_QUERY_TKT_CACHE_RESPONSE = *mut KERB_QUERY_TKT_CACHE_RESPONSE;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct KERB_QUERY_TKT_CACHE_EX_RESPONSE {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub CountOfTickets: ::ULONG,
    pub Tickets: [KERB_TICKET_CACHE_INFO_EX; ::ANYSIZE_ARRAY],
}
pub type PKERB_QUERY_TKT_CACHE_EX_RESPONSE = *mut KERB_QUERY_TKT_CACHE_EX_RESPONSE;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct KERB_QUERY_TKT_CACHE_EX2_RESPONSE {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub CountOfTickets: ::ULONG,
    pub Tickets: [KERB_TICKET_CACHE_INFO_EX2; ::ANYSIZE_ARRAY],
}
pub type PKERB_QUERY_TKT_CACHE_EX2_RESPONSE = *mut KERB_QUERY_TKT_CACHE_EX2_RESPONSE;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct KERB_QUERY_TKT_CACHE_EX3_RESPONSE {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub CountOfTickets: ::ULONG,
    pub Tickets: [KERB_TICKET_CACHE_INFO_EX3; ::ANYSIZE_ARRAY],
}
pub type PKERB_QUERY_TKT_CACHE_EX3_RESPONSE = *mut KERB_QUERY_TKT_CACHE_EX3_RESPONSE;
pub const KERB_USE_DEFAULT_TICKET_FLAGS: ::ULONG = 0x0;
pub const KERB_RETRIEVE_TICKET_DEFAULT: ::ULONG = 0x0;
pub const KERB_RETRIEVE_TICKET_DONT_USE_CACHE: ::ULONG = 0x1;
pub const KERB_RETRIEVE_TICKET_USE_CACHE_ONLY: ::ULONG = 0x2;
pub const KERB_RETRIEVE_TICKET_USE_CREDHANDLE: ::ULONG = 0x4;
pub const KERB_RETRIEVE_TICKET_AS_KERB_CRED: ::ULONG = 0x8;
pub const KERB_RETRIEVE_TICKET_WITH_SEC_CRED: ::ULONG = 0x10;
pub const KERB_RETRIEVE_TICKET_CACHE_TICKET: ::ULONG = 0x20;
pub const KERB_RETRIEVE_TICKET_MAX_LIFETIME: ::ULONG = 0x40;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct KERB_AUTH_DATA {
    pub Type: ::ULONG,
    pub Length: ::ULONG,
    pub Data: ::PUCHAR,
}
pub type PKERB_AUTH_DATA = *mut KERB_AUTH_DATA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct KERB_NET_ADDRESS {
    pub Family: ::ULONG,
    pub Length: ::ULONG,
    pub Address: ::PUCHAR,
}
pub type PKERB_NET_ADDRESS = *mut KERB_NET_ADDRESS;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct KERB_NET_ADDRESSES {
    pub Number: ::ULONG,
    pub Addresses: [KERB_NET_ADDRESS; ::ANYSIZE_ARRAY],
}
pub type PKERB_NET_ADDRESSES = *mut KERB_NET_ADDRESSES;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct KERB_EXTERNAL_NAME {
    pub NameType: ::SHORT,
    pub NameCount: ::USHORT,
    pub Names: [::UNICODE_STRING; ::ANYSIZE_ARRAY],
}
pub type PKERB_EXTERNAL_NAME = *mut KERB_EXTERNAL_NAME;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct KERB_EXTERNAL_TICKET {
    pub ServiceName: PKERB_EXTERNAL_NAME,
    pub TargetName: PKERB_EXTERNAL_NAME,
    pub ClientName: PKERB_EXTERNAL_NAME,
    pub DomainName: ::UNICODE_STRING,
    pub TargetDomainName: ::UNICODE_STRING,
    pub AltTargetDomainName: ::UNICODE_STRING,
    pub SessionKey: KERB_CRYPTO_KEY,
    pub TicketFlags: ::ULONG,
    pub Flags: ::ULONG,
    pub KeyExpirationTime: ::LARGE_INTEGER,
    pub StartTime: ::LARGE_INTEGER,
    pub EndTime: ::LARGE_INTEGER,
    pub RenewUntil: ::LARGE_INTEGER,
    pub TimeSkew: ::LARGE_INTEGER,
    pub EncodedTicketSize: ::ULONG,
    pub EncodedTicket: ::PUCHAR,
}
pub type PKERB_EXTERNAL_TICKET = *mut KERB_EXTERNAL_TICKET;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct KERB_RETRIEVE_TKT_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub LogonId: ::LUID,
    pub TargetName: ::UNICODE_STRING,
    pub TicketFlags: ::ULONG,
    pub CacheOptions: ::ULONG,
    pub EncryptionType: ::LONG,
    pub CredentialsHandle: ::SecHandle,
}
pub type PKERB_RETRIEVE_TKT_REQUEST = *mut KERB_RETRIEVE_TKT_REQUEST;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct KERB_RETRIEVE_TKT_RESPONSE {
    pub Ticket: KERB_EXTERNAL_TICKET,
}
pub type PKERB_RETRIEVE_TKT_RESPONSE = *mut KERB_RETRIEVE_TKT_RESPONSE;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct KERB_PURGE_TKT_CACHE_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub LogonId: ::LUID,
    pub ServerName: ::UNICODE_STRING,
    pub RealmName: ::UNICODE_STRING,
}
pub type PKERB_PURGE_TKT_CACHE_REQUEST = *mut KERB_PURGE_TKT_CACHE_REQUEST;
pub const KERB_PURGE_ALL_TICKETS: ::ULONG = 1;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct KERB_PURGE_TKT_CACHE_EX_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub LogonId: ::LUID,
    pub Flags: ::ULONG,
    pub TicketTemplate: KERB_TICKET_CACHE_INFO_EX,
}
pub type PKERB_PURGE_TKT_CACHE_EX_REQUEST = *mut KERB_PURGE_TKT_CACHE_EX_REQUEST;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct KERB_SUBMIT_TKT_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub LogonId: ::LUID,
    pub Flags: ::ULONG,
    pub Key: KERB_CRYPTO_KEY32,
    pub KerbCredSize: ::ULONG,
    pub KerbCredOffset: ::ULONG,
}
pub type PKERB_SUBMIT_TKT_REQUEST = *mut KERB_SUBMIT_TKT_REQUEST;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct KERB_QUERY_KDC_PROXY_CACHE_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub Flags: ::ULONG,
    pub LogonId: ::LUID,
}
pub type PKERB_QUERY_KDC_PROXY_CACHE_REQUEST = *mut KERB_QUERY_KDC_PROXY_CACHE_REQUEST;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct KDC_PROXY_CACHE_ENTRY_DATA {
    pub SinceLastUsed: ::ULONG64,
    pub DomainName: ::UNICODE_STRING,
    pub ProxyServerName: ::UNICODE_STRING,
    pub ProxyServerVdir: ::UNICODE_STRING,
    pub ProxyServerPort: ::USHORT,
    pub LogonId: ::LUID,
    pub CredUserName: ::UNICODE_STRING,
    pub CredDomainName: ::UNICODE_STRING,
    pub GlobalCache: ::BOOLEAN,
}
pub type PKDC_PROXY_CACHE_ENTRY_DATA = *mut KDC_PROXY_CACHE_ENTRY_DATA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct KERB_QUERY_KDC_PROXY_CACHE_RESPONSE {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub CountOfEntries: ::ULONG,
    pub Entries: PKDC_PROXY_CACHE_ENTRY_DATA,
}
pub type PKERB_QUERY_KDC_PROXY_CACHE_RESPONSE = *mut KERB_QUERY_KDC_PROXY_CACHE_RESPONSE;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct KERB_PURGE_KDC_PROXY_CACHE_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub Flags: ::ULONG,
    pub LogonId: ::LUID,
}
pub type PKERB_PURGE_KDC_PROXY_CACHE_REQUEST = *mut KERB_PURGE_KDC_PROXY_CACHE_REQUEST;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct KERB_PURGE_KDC_PROXY_CACHE_RESPONSE {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub CountOfPurged: ::ULONG,
}
pub type PKERB_PURGE_KDC_PROXY_CACHE_RESPONSE = *mut KERB_PURGE_KDC_PROXY_CACHE_RESPONSE;
pub const KERB_S4U2PROXY_CACHE_ENTRY_INFO_FLAG_NEGATIVE: ::ULONG = 0x1;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct KERB_S4U2PROXY_CACHE_ENTRY_INFO {
    pub ServerName: ::UNICODE_STRING,
    pub Flags: ::ULONG,
    pub LastStatus: ::NTSTATUS,
    pub Expiry: ::LARGE_INTEGER,
}
pub type PKERB_S4U2PROXY_CACHE_ENTRY_INFO = *mut KERB_S4U2PROXY_CACHE_ENTRY_INFO;
pub const KERB_S4U2PROXY_CRED_FLAG_NEGATIVE: ::ULONG = 0x1;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct KERB_S4U2PROXY_CRED {
    pub UserName: ::UNICODE_STRING,
    pub DomainName: ::UNICODE_STRING,
    pub Flags: ::ULONG,
    pub LastStatus: ::NTSTATUS,
    pub Expiry: ::LARGE_INTEGER,
    pub CountOfEntries: ::ULONG,
    pub Entries: PKERB_S4U2PROXY_CACHE_ENTRY_INFO,
}
pub type PKERB_S4U2PROXY_CRED = *mut KERB_S4U2PROXY_CRED;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct KERB_QUERY_S4U2PROXY_CACHE_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub Flags: ::ULONG,
    pub LogonId: ::LUID,
}
pub type PKERB_QUERY_S4U2PROXY_CACHE_REQUEST = *mut KERB_QUERY_S4U2PROXY_CACHE_REQUEST;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct KERB_QUERY_S4U2PROXY_CACHE_RESPONSE {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub CountOfCreds: ::ULONG,
    pub Creds: PKERB_S4U2PROXY_CRED,
}
pub type PKERB_QUERY_S4U2PROXY_CACHE_RESPONSE = *mut KERB_QUERY_S4U2PROXY_CACHE_RESPONSE;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct KERB_CHANGEPASSWORD_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub DomainName: ::UNICODE_STRING,
    pub AccountName: ::UNICODE_STRING,
    pub OldPassword: ::UNICODE_STRING,
    pub NewPassword: ::UNICODE_STRING,
    pub Impersonating: ::BOOLEAN,
}
pub type PKERB_CHANGEPASSWORD_REQUEST = *mut KERB_CHANGEPASSWORD_REQUEST;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct KERB_SETPASSWORD_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub LogonId: ::LUID,
    pub CredentialsHandle: ::SecHandle,
    pub Flags: ::ULONG,
    pub DomainName: ::UNICODE_STRING,
    pub AccountName: ::UNICODE_STRING,
    pub Password: ::UNICODE_STRING,
}
pub type PKERB_SETPASSWORD_REQUEST = *mut KERB_SETPASSWORD_REQUEST;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct KERB_SETPASSWORD_EX_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub LogonId: ::LUID,
    pub CredentialsHandle: ::SecHandle,
    pub Flags: ::ULONG,
    pub AccountRealm: ::UNICODE_STRING,
    pub AccountName: ::UNICODE_STRING,
    pub Password: ::UNICODE_STRING,
    pub ClientRealm: ::UNICODE_STRING,
    pub ClientName: ::UNICODE_STRING,
    pub Impersonating: ::BOOLEAN,
    pub KdcAddress: ::UNICODE_STRING,
    pub KdcAddressType: ::ULONG,
}
pub type PKERB_SETPASSWORD_EX_REQUEST = *mut KERB_SETPASSWORD_EX_REQUEST;
pub const DS_UNKNOWN_ADDRESS_TYPE: ::ULONG = 0;
pub const KERB_SETPASS_USE_LOGONID: ::ULONG = 1;
pub const KERB_SETPASS_USE_CREDHANDLE: ::ULONG = 2;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct KERB_DECRYPT_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub LogonId: ::LUID,
    pub Flags: ::ULONG,
    pub CryptoType: ::LONG,
    pub KeyUsage: ::LONG,
    pub Key: KERB_CRYPTO_KEY,
    pub EncryptedDataSize: ::ULONG,
    pub InitialVectorSize: ::ULONG,
    pub InitialVector: ::PUCHAR,
    pub EncryptedData: ::PUCHAR,
}
pub type PKERB_DECRYPT_REQUEST = *mut KERB_DECRYPT_REQUEST;
pub const KERB_DECRYPT_FLAG_DEFAULT_KEY: ::ULONG = 0x00000001;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct KERB_DECRYPT_RESPONSE {
    pub DecryptedData: [::UCHAR; ::ANYSIZE_ARRAY],
}
pub type PKERB_DECRYPT_RESPONSE = *mut KERB_DECRYPT_RESPONSE;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct KERB_ADD_BINDING_CACHE_ENTRY_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub RealmName: ::UNICODE_STRING,
    pub KdcAddress: ::UNICODE_STRING,
    pub AddressType: ::ULONG,
}
pub type PKERB_ADD_BINDING_CACHE_ENTRY_REQUEST = *mut KERB_ADD_BINDING_CACHE_ENTRY_REQUEST;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct KERB_REFRESH_SCCRED_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub CredentialBlob: ::UNICODE_STRING,
    pub LogonId: ::LUID,
    pub Flags: ::ULONG,
}
pub type PKERB_REFRESH_SCCRED_REQUEST = *mut KERB_REFRESH_SCCRED_REQUEST;
pub const KERB_REFRESH_SCCRED_RELEASE: ::ULONG = 0x0;
pub const KERB_REFRESH_SCCRED_GETTGT: ::ULONG = 0x1;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct KERB_ADD_CREDENTIALS_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub UserName: ::UNICODE_STRING,
    pub DomainName: ::UNICODE_STRING,
    pub Password: ::UNICODE_STRING,
    pub LogonId: ::LUID,
    pub Flags: ::ULONG,
}
pub type PKERB_ADD_CREDENTIALS_REQUEST = *mut KERB_ADD_CREDENTIALS_REQUEST;
pub const KERB_REQUEST_ADD_CREDENTIAL: ::ULONG = 1;
pub const KERB_REQUEST_REPLACE_CREDENTIAL: ::ULONG = 2;
pub const KERB_REQUEST_REMOVE_CREDENTIAL: ::ULONG = 4;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct KERB_ADD_CREDENTIALS_REQUEST_EX {
    pub Credentials: KERB_ADD_CREDENTIALS_REQUEST,
    pub PrincipalNameCount: ::ULONG,
    pub PrincipalNames: [::UNICODE_STRING; ::ANYSIZE_ARRAY],
}
pub type PKERB_ADD_CREDENTIALS_REQUEST_EX = *mut KERB_ADD_CREDENTIALS_REQUEST_EX;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct KERB_TRANSFER_CRED_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub OriginLogonId: ::LUID,
    pub DestinationLogonId: ::LUID,
    pub Flags: ::ULONG,
}
pub type PKERB_TRANSFER_CRED_REQUEST = *mut KERB_TRANSFER_CRED_REQUEST;
pub const KERB_TRANSFER_CRED_WITH_TICKETS: ::ULONG = 0x1;
pub const KERB_TRANSFER_CRED_CLEANUP_CREDENTIALS: ::ULONG = 0x2;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct KERB_CLEANUP_MACHINE_PKINIT_CREDS_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub LogonId: ::LUID,
}
pub type PKERB_CLEANUP_MACHINE_PKINIT_CREDS_REQUEST =
    *mut KERB_CLEANUP_MACHINE_PKINIT_CREDS_REQUEST;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct KERB_BINDING_CACHE_ENTRY_DATA {
    pub DiscoveryTime: ::ULONG64,
    pub RealmName: ::UNICODE_STRING,
    pub KdcAddress: ::UNICODE_STRING,
    pub AddressType: ::ULONG,
    pub Flags: ::ULONG,
    pub DcFlags: ::ULONG,
    pub CacheFlags: ::ULONG,
    pub KdcName: ::UNICODE_STRING,
}
pub type PKERB_BINDING_CACHE_ENTRY_DATA = *mut KERB_BINDING_CACHE_ENTRY_DATA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct KERB_QUERY_BINDING_CACHE_RESPONSE {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub CountOfEntries: ::ULONG,
    pub Entries: PKERB_BINDING_CACHE_ENTRY_DATA,
}
pub type PKERB_QUERY_BINDING_CACHE_RESPONSE = *mut KERB_QUERY_BINDING_CACHE_RESPONSE;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct KERB_ADD_BINDING_CACHE_ENTRY_EX_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub RealmName: ::UNICODE_STRING,
    pub KdcAddress: ::UNICODE_STRING,
    pub AddressType: ::ULONG,
    pub DcFlags: ::ULONG,
}
pub type PKERB_ADD_BINDING_CACHE_ENTRY_EX_REQUEST = *mut KERB_ADD_BINDING_CACHE_ENTRY_EX_REQUEST;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct KERB_QUERY_BINDING_CACHE_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
}
pub type PKERB_QUERY_BINDING_CACHE_REQUEST = *mut KERB_QUERY_BINDING_CACHE_REQUEST;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct KERB_PURGE_BINDING_CACHE_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
}
pub type PKERB_PURGE_BINDING_CACHE_REQUEST = *mut KERB_PURGE_BINDING_CACHE_REQUEST;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct KERB_QUERY_DOMAIN_EXTENDED_POLICIES_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub Flags: ::ULONG,
    pub DomainName: ::UNICODE_STRING,
}
pub type PKERB_QUERY_DOMAIN_EXTENDED_POLICIES_REQUEST =
    *mut KERB_QUERY_DOMAIN_EXTENDED_POLICIES_REQUEST;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct KERB_QUERY_DOMAIN_EXTENDED_POLICIES_RESPONSE {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub Flags: ::ULONG,
    pub ExtendedPolicies: ::ULONG,
    pub DsFlags: ::ULONG,
}
pub type PKERB_QUERY_DOMAIN_EXTENDED_POLICIES_RESPONSE =
    *mut KERB_QUERY_DOMAIN_EXTENDED_POLICIES_RESPONSE;
ENUM!{enum KERB_CERTIFICATE_INFO_TYPE {
    CertHashInfo = 1,
}}
pub type PKERB_CERTIFICATE_INFO_TYPE = *mut KERB_CERTIFICATE_INFO_TYPE;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct KERB_CERTIFICATE_HASHINFO {
    pub StoreNameLength: ::USHORT,
    pub HashLength: ::USHORT,
}
pub type PKERB_CERTIFICATE_HASHINFO = *mut KERB_CERTIFICATE_HASHINFO;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct KERB_CERTIFICATE_INFO {
    pub CertInfoSize: ::ULONG,
    pub InfoType: ::ULONG,
}
pub type PKERB_CERTIFICATE_INFO = *mut KERB_CERTIFICATE_INFO;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct POLICY_AUDIT_SID_ARRAY {
    pub UsersCount: ::ULONG,
    pub UserSidArray: *mut ::PSID,
}
pub type PPOLICY_AUDIT_SID_ARRAY = *mut POLICY_AUDIT_SID_ARRAY;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct AUDIT_POLICY_INFORMATION {
    pub AuditSubCategoryGuid: ::GUID,
    pub AuditingInformation: ::ULONG,
    pub AuditCategoryGuid: ::GUID,
}
pub type PAUDIT_POLICY_INFORMATION = *mut AUDIT_POLICY_INFORMATION;
pub type LPAUDIT_POLICY_INFORMATION = PAUDIT_POLICY_INFORMATION;
pub type PCAUDIT_POLICY_INFORMATION = *const AUDIT_POLICY_INFORMATION;
pub const AUDIT_SET_SYSTEM_POLICY: ::ULONG = 0x0001;
pub const AUDIT_QUERY_SYSTEM_POLICY: ::ULONG = 0x0002;
pub const AUDIT_SET_USER_POLICY: ::ULONG = 0x0004;
pub const AUDIT_QUERY_USER_POLICY: ::ULONG = 0x0008;
pub const AUDIT_ENUMERATE_USERS: ::ULONG = 0x0010;
pub const AUDIT_SET_MISC_POLICY: ::ULONG = 0x0020;
pub const AUDIT_QUERY_MISC_POLICY: ::ULONG = 0x0040;
pub const AUDIT_GENERIC_ALL: ::ULONG = ::STANDARD_RIGHTS_REQUIRED | AUDIT_SET_SYSTEM_POLICY
    | AUDIT_QUERY_SYSTEM_POLICY | AUDIT_SET_USER_POLICY | AUDIT_QUERY_USER_POLICY
    | AUDIT_ENUMERATE_USERS | AUDIT_SET_MISC_POLICY | AUDIT_QUERY_MISC_POLICY;
pub const AUDIT_GENERIC_READ: ::ULONG = ::STANDARD_RIGHTS_READ | AUDIT_QUERY_SYSTEM_POLICY
    | AUDIT_QUERY_USER_POLICY | AUDIT_ENUMERATE_USERS | AUDIT_QUERY_MISC_POLICY;
pub const AUDIT_GENERIC_WRITE: ::ULONG = ::STANDARD_RIGHTS_WRITE | AUDIT_SET_USER_POLICY
    | AUDIT_SET_MISC_POLICY | AUDIT_SET_SYSTEM_POLICY;
pub const AUDIT_GENERIC_EXECUTE: ::ULONG = ::STANDARD_RIGHTS_EXECUTE;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct PKU2U_CERT_BLOB {
    pub CertOffset: ::ULONG,
    pub CertLength: ::USHORT,
}
pub type PPKU2U_CERT_BLOB = *mut PKU2U_CERT_BLOB;
pub const PKU2U_CREDUI_CONTEXT_VERSION: ::ULONG64 = 0x4154414454524543;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct PKU2U_CREDUI_CONTEXT {
    pub Version: ::ULONG64,
    pub cbHeaderLength: ::USHORT,
    pub cbStructureLength: ::ULONG,
    pub CertArrayCount: ::USHORT,
    pub CertArrayOffset: ::ULONG,
}
pub type PPKU2U_CREDUI_CONTEXT = *mut PKU2U_CREDUI_CONTEXT;
ENUM!{enum PKU2U_LOGON_SUBMIT_TYPE {
    Pku2uCertificateS4ULogon = 14,
}}
pub type PPKU2U_LOGON_SUBMIT_TYPE = *mut PKU2U_LOGON_SUBMIT_TYPE;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct PKU2U_CERTIFICATE_S4U_LOGON {
    pub MessageType: PKU2U_LOGON_SUBMIT_TYPE,
    pub Flags: ::ULONG,
    pub UserPrincipalName: ::UNICODE_STRING,
    pub DomainName: ::UNICODE_STRING,
    pub CertificateLength: ::ULONG,
    pub Certificate: ::PUCHAR,
}
pub type PPKU2U_CERTIFICATE_S4U_LOGON = *mut PKU2U_CERTIFICATE_S4U_LOGON;

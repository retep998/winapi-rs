// Copyright © 2015, skdltmxn
// Licensed under the MIT License <LICENSE.md>
// This file contains structures, function prototypes, and definitions
// for the NetUser, NetUserModals, NetGroup, NetAccess, and NetLogon API.
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct USER_INFO_0 {
    pub usri0_name: ::LPWSTR,
}
pub type PUSER_INFO_0 = *mut USER_INFO_0;
pub type LPUSER_INFO_0 = *mut USER_INFO_0;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct USER_INFO_1 {
    pub usri1_name: ::LPWSTR,
    pub usri1_password: ::LPWSTR,
    pub usri1_password_age: ::DWORD,
    pub usri1_priv: ::DWORD,
    pub usri1_home_dir: ::LPWSTR,
    pub usri1_comment: ::LPWSTR,
    pub usri1_flags: ::DWORD,
    pub usri1_script_path: ::LPWSTR,
}
pub type PUSER_INFO_1 = *mut USER_INFO_1;
pub type LPUSER_INFO_1 = *mut USER_INFO_1;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct USER_INFO_2 {
    pub usri2_name: ::LPWSTR,
    pub usri2_password: ::LPWSTR,
    pub usri2_password_age: ::DWORD,
    pub usri2_priv: ::DWORD,
    pub usri2_home_dir: ::LPWSTR,
    pub usri2_comment: ::LPWSTR,
    pub usri2_flags: ::DWORD,
    pub usri2_script_path: ::LPWSTR,
    pub usri2_auth_flags: ::DWORD,
    pub usri2_full_name: ::LPWSTR,
    pub usri2_usr_comment: ::LPWSTR,
    pub usri2_parms: ::LPWSTR,
    pub usri2_workstations: ::LPWSTR,
    pub usri2_last_logon: ::DWORD,
    pub usri2_last_logoff: ::DWORD,
    pub usri2_acct_expires: ::DWORD,
    pub usri2_max_storage: ::DWORD,
    pub usri2_units_per_week: ::DWORD,
    pub usri2_logon_hours: ::PBYTE,
    pub usri2_bad_pw_count: ::DWORD,
    pub usri2_num_logons: ::DWORD,
    pub usri2_logon_server: ::LPWSTR,
    pub usri2_country_code: ::DWORD,
    pub usri2_code_page: ::DWORD,
}
pub type PUSER_INFO_2 = *mut USER_INFO_2;
pub type LPUSER_INFO_2 = *mut USER_INFO_2;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct USER_INFO_3 {
    pub usri3_name: ::LPWSTR,
    pub usri3_password: ::LPWSTR,
    pub usri3_password_age: ::DWORD,
    pub usri3_priv: ::DWORD,
    pub usri3_home_dir: ::LPWSTR,
    pub usri3_comment: ::LPWSTR,
    pub usri3_flags: ::DWORD,
    pub usri3_script_path: ::LPWSTR,
    pub usri3_auth_flags: ::DWORD,
    pub usri3_full_name: ::LPWSTR,
    pub usri3_usr_comment: ::LPWSTR,
    pub usri3_parms: ::LPWSTR,
    pub usri3_workstations: ::LPWSTR,
    pub usri3_last_logon: ::DWORD,
    pub usri3_last_logoff: ::DWORD,
    pub usri3_acct_expires: ::DWORD,
    pub usri3_max_storage: ::DWORD,
    pub usri3_units_per_week: ::DWORD,
    pub usri3_logon_hours: ::PBYTE,
    pub usri3_bad_pw_count: ::DWORD,
    pub usri3_num_logons: ::DWORD,
    pub usri3_logon_server: ::LPWSTR,
    pub usri3_country_code: ::DWORD,
    pub usri3_code_page: ::DWORD,
    pub usri3_user_id: ::DWORD,
    pub usri3_primary_group_id: ::DWORD,
    pub usri3_profile: ::LPWSTR,
    pub usri3_home_dir_drive: ::LPWSTR,
    pub usri3_password_expired: ::DWORD,
}
pub type PUSER_INFO_3 = *mut USER_INFO_3;
pub type LPUSER_INFO_3 = *mut USER_INFO_3;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct USER_INFO_4 {
    pub usri4_name: ::LPWSTR,
    pub usri4_password: ::LPWSTR,
    pub usri4_password_age: ::DWORD,
    pub usri4_priv: ::DWORD,
    pub usri4_home_dir: ::LPWSTR,
    pub usri4_comment: ::LPWSTR,
    pub usri4_flags: ::DWORD,
    pub usri4_script_path: ::LPWSTR,
    pub usri4_auth_flags: ::DWORD,
    pub usri4_full_name: ::LPWSTR,
    pub usri4_usr_comment: ::LPWSTR,
    pub usri4_parms: ::LPWSTR,
    pub usri4_workstations: ::LPWSTR,
    pub usri4_last_logon: ::DWORD,
    pub usri4_last_logoff: ::DWORD,
    pub usri4_acct_expires: ::DWORD,
    pub usri4_max_storage: ::DWORD,
    pub usri4_units_per_week: ::DWORD,
    pub usri4_logon_hours: ::PBYTE,
    pub usri4_bad_pw_count: ::DWORD,
    pub usri4_num_logons: ::DWORD,
    pub usri4_logon_server: ::LPWSTR,
    pub usri4_country_code: ::DWORD,
    pub usri4_code_page: ::DWORD,
    pub usri4_user_sid: ::PSID,
    pub usri4_primary_group_id: ::DWORD,
    pub usri4_profile: ::LPWSTR,
    pub usri4_home_dir_drive: ::LPWSTR,
    pub usri4_password_expired: ::DWORD,
}
pub type PUSER_INFO_4 = *mut USER_INFO_4;
pub type LPUSER_INFO_4 = *mut USER_INFO_4;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct USER_INFO_10 {
    pub usri10_name: ::LPWSTR,
    pub usri10_comment: ::LPWSTR,
    pub usri10_usr_comment: ::LPWSTR,
    pub usri10_full_name: ::LPWSTR,
}
pub type PUSER_INFO_10 = *mut USER_INFO_10;
pub type LPUSER_INFO_10 = *mut USER_INFO_10;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct USER_INFO_11 {
    pub usri11_name: ::LPWSTR,
    pub usri11_comment: ::LPWSTR,
    pub usri11_usr_comment: ::LPWSTR,
    pub usri11_full_name: ::LPWSTR,
    pub usri11_priv: ::DWORD,
    pub usri11_auth_flags: ::DWORD,
    pub usri11_password_age: ::DWORD,
    pub usri11_home_dir: ::LPWSTR,
    pub usri11_parms: ::LPWSTR,
    pub usri11_last_logon: ::DWORD,
    pub usri11_last_logoff: ::DWORD,
    pub usri11_bad_pw_count: ::DWORD,
    pub usri11_num_logons: ::DWORD,
    pub usri11_logon_server: ::LPWSTR,
    pub usri11_country_code: ::DWORD,
    pub usri11_workstations: ::LPWSTR,
    pub usri11_max_storage: ::DWORD,
    pub usri11_units_per_week: ::DWORD,
    pub usri11_logon_hours: ::PBYTE,
    pub usri11_code_page: ::DWORD,
}
pub type PUSER_INFO_11 = *mut USER_INFO_11;
pub type LPUSER_INFO_11 = *mut USER_INFO_11;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct USER_INFO_20 {
    pub usri20_name: ::LPWSTR,
    pub usri20_full_name: ::LPWSTR,
    pub usri20_comment: ::LPWSTR,
    pub usri20_flags: ::DWORD,
    pub usri20_user_id: ::DWORD,
}
pub type PUSER_INFO_20 = *mut USER_INFO_20;
pub type LPUSER_INFO_20 = *mut USER_INFO_20;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct USER_INFO_21 {
    pub usri21_password: [::BYTE; ::ENCRYPTED_PWLEN],
}
pub type PUSER_INFO_21 = *mut USER_INFO_21;
pub type LPUSER_INFO_21 = *mut USER_INFO_21;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct USER_INFO_22 {
    pub usri22_name: ::LPWSTR,
    pub usri22_password: [::BYTE; ::ENCRYPTED_PWLEN],
    pub usri22_password_age: ::DWORD,
    pub usri22_priv: ::DWORD,
    pub usri22_home_dir: ::LPWSTR,
    pub usri22_comment: ::LPWSTR,
    pub usri22_flags: ::DWORD,
    pub usri22_script_path: ::LPWSTR,
    pub usri22_auth_flags: ::DWORD,
    pub usri22_full_name: ::LPWSTR,
    pub usri22_usr_comment: ::LPWSTR,
    pub usri22_parms: ::LPWSTR,
    pub usri22_workstations: ::LPWSTR,
    pub usri22_last_logon: ::DWORD,
    pub usri22_last_logoff: ::DWORD,
    pub usri22_acct_expires: ::DWORD,
    pub usri22_max_storage: ::DWORD,
    pub usri22_units_per_week: ::DWORD,
    pub usri22_logon_hours: ::PBYTE,
    pub usri22_bad_pw_count: ::DWORD,
    pub usri22_num_logons: ::DWORD,
    pub usri22_logon_server: ::LPWSTR,
    pub usri22_country_code: ::DWORD,
    pub usri22_code_page: ::DWORD,
}
pub type PUSER_INFO_22 = *mut USER_INFO_22;
pub type LPUSER_INFO_22 = *mut USER_INFO_22;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct USER_INFO_23 {
    pub usri23_name: ::LPWSTR,
    pub usri23_full_name: ::LPWSTR,
    pub usri23_comment: ::LPWSTR,
    pub usri23_flags: ::DWORD,
    pub usri23_user_sid: ::PSID,
}
pub type PUSER_INFO_23 = *mut USER_INFO_23;
pub type LPUSER_INFO_23 = *mut USER_INFO_23;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct USER_INFO_24 {
    pub usri24_internet_identity: ::BOOL,
    pub usri24_flags: ::DWORD,
    pub usri24_internet_provider_name: ::LPWSTR,
    pub usri24_internet_principal_name: ::LPWSTR,
    pub usri24_user_sid: ::PSID,
}
pub type PUSER_INFO_24 = *mut USER_INFO_24;
pub type LPUSER_INFO_24 = *mut USER_INFO_24;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct USER_INFO_1003 {
    pub usri1003_password: ::LPWSTR,
}
pub type PUSER_INFO_1003 = *mut USER_INFO_1003;
pub type LPUSER_INFO_1003 = *mut USER_INFO_1003;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct USER_INFO_1005 {
    pub usri1005_priv: ::DWORD,
}
pub type PUSER_INFO_1005 = *mut USER_INFO_1005;
pub type LPUSER_INFO_1005 = *mut USER_INFO_1005;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct USER_INFO_1006 {
    pub usri1006_home_dir: ::LPWSTR,
}
pub type PUSER_INFO_1006 = *mut USER_INFO_1006;
pub type LPUSER_INFO_1006 = *mut USER_INFO_1006;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct USER_INFO_1007 {
    pub usri1007_comment: ::LPWSTR,
}
pub type PUSER_INFO_1007 = *mut USER_INFO_1007;
pub type LPUSER_INFO_1007 = *mut USER_INFO_1007;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct USER_INFO_1008 {
    pub usri1008_flags: ::DWORD,
}
pub type PUSER_INFO_1008 = *mut USER_INFO_1008;
pub type LPUSER_INFO_1008 = *mut USER_INFO_1008;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct USER_INFO_1009 {
    pub usri1009_script_path: ::LPWSTR,
}
pub type PUSER_INFO_1009 = *mut USER_INFO_1009;
pub type LPUSER_INFO_1009 = *mut USER_INFO_1009;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct USER_INFO_1010 {
    pub usri1010_auth_flags: ::DWORD,
}
pub type PUSER_INFO_1010 = *mut USER_INFO_1010;
pub type LPUSER_INFO_1010 = *mut USER_INFO_1010;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct USER_INFO_1011 {
    pub usri1011_full_name: ::LPWSTR,
}
pub type PUSER_INFO_1011 = *mut USER_INFO_1011;
pub type LPUSER_INFO_1011 = *mut USER_INFO_1011;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct USER_INFO_1012 {
    pub usri1012_usr_comment: ::LPWSTR,
}
pub type PUSER_INFO_1012 = *mut USER_INFO_1012;
pub type LPUSER_INFO_1012 = *mut USER_INFO_1012;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct USER_INFO_1013 {
    pub usri1013_parms: ::LPWSTR,
}
pub type PUSER_INFO_1013 = *mut USER_INFO_1013;
pub type LPUSER_INFO_1013 = *mut USER_INFO_1013;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct USER_INFO_1014 {
    pub usri1014_workstations: ::LPWSTR,
}
pub type PUSER_INFO_1014 = *mut USER_INFO_1014;
pub type LPUSER_INFO_1014 = *mut USER_INFO_1014;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct USER_INFO_1017 {
    pub usri1017_acct_expires: ::DWORD,
}
pub type PUSER_INFO_1017 = *mut USER_INFO_1017;
pub type LPUSER_INFO_1017 = *mut USER_INFO_1017;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct USER_INFO_1018 {
    pub usri1018_max_storage: ::DWORD,
}
pub type PUSER_INFO_1018 = *mut USER_INFO_1018;
pub type LPUSER_INFO_1018 = *mut USER_INFO_1018;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct USER_INFO_1020 {
    pub usri1020_units_per_week: ::DWORD,
    pub usri1020_logon_hours: ::LPBYTE,
}
pub type PUSER_INFO_1020 = *mut USER_INFO_1020;
pub type LPUSER_INFO_1020 = *mut USER_INFO_1020;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct USER_INFO_1023 {
    pub usri1023_logon_server: ::LPWSTR,
}
pub type PUSER_INFO_1023 = *mut USER_INFO_1023;
pub type LPUSER_INFO_1023 = *mut USER_INFO_1023;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct USER_INFO_1024 {
    pub usri1024_country_code: ::DWORD,
}
pub type PUSER_INFO_1024 = *mut USER_INFO_1024;
pub type LPUSER_INFO_1024 = *mut USER_INFO_1024;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct USER_INFO_1025 {
    pub usri1025_code_page: ::DWORD,
}
pub type PUSER_INFO_1025 = *mut USER_INFO_1025;
pub type LPUSER_INFO_1025 = *mut USER_INFO_1025;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct USER_INFO_1051 {
    pub usri1051_primary_group_id: ::DWORD,
}
pub type PUSER_INFO_1051 = *mut USER_INFO_1051;
pub type LPUSER_INFO_1051 = *mut USER_INFO_1051;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct USER_INFO_1052 {
    pub usri1052_profile: ::LPWSTR,
}
pub type PUSER_INFO_1052 = *mut USER_INFO_1052;
pub type LPUSER_INFO_1052 = *mut USER_INFO_1052;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct USER_INFO_1053 {
    pub usri1053_home_dir_drive: ::LPWSTR,
}
pub type PUSER_INFO_1053 = *mut USER_INFO_1053;
pub type LPUSER_INFO_1053 = *mut USER_INFO_1053;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct USER_MODALS_INFO_0 {
    pub usrmod0_min_passwd_len: ::DWORD,
    pub usrmod0_max_passwd_age: ::DWORD,
    pub usrmod0_min_passwd_age: ::DWORD,
    pub usrmod0_force_logoff: ::DWORD,
    pub usrmod0_password_hist_len: ::DWORD,
}
pub type PUSER_MODALS_INFO_0 = *mut USER_MODALS_INFO_0;
pub type LPUSER_MODALS_INFO_0 = *mut USER_MODALS_INFO_0;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct USER_MODALS_INFO_1 {
    pub usrmod1_role: ::DWORD,
    pub usrmod1_primary: ::LPWSTR,
}
pub type PUSER_MODALS_INFO_1 = *mut USER_MODALS_INFO_1;
pub type LPUSER_MODALS_INFO_1 = *mut USER_MODALS_INFO_1;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct USER_MODALS_INFO_2 {
    pub usrmod2_domain_name: ::LPWSTR,
    pub usrmod2_domain_id: ::PSID,
}
pub type PUSER_MODALS_INFO_2 = *mut USER_MODALS_INFO_2;
pub type LPUSER_MODALS_INFO_2 = *mut USER_MODALS_INFO_2;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct USER_MODALS_INFO_3 {
    pub usrmod3_lockout_duration: ::DWORD,
    pub usrmod3_lockout_observation_window: ::DWORD,
    pub usrmod3_lockout_threshold: ::DWORD,
}
pub type PUSER_MODALS_INFO_3 = *mut USER_MODALS_INFO_3;
pub type LPUSER_MODALS_INFO_3 = *mut USER_MODALS_INFO_3;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct USER_MODALS_INFO_1001 {
    pub usrmod1001_min_passwd_len: ::DWORD,
}
pub type PUSER_MODALS_INFO_1001 = *mut USER_MODALS_INFO_1001;
pub type LPUSER_MODALS_INFO_1001 = *mut USER_MODALS_INFO_1001;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct USER_MODALS_INFO_1002 {
    pub usrmod1002_max_passwd_age: ::DWORD,
}
pub type PUSER_MODALS_INFO_1002 = *mut USER_MODALS_INFO_1002;
pub type LPUSER_MODALS_INFO_1002 = *mut USER_MODALS_INFO_1002;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct USER_MODALS_INFO_1003 {
    pub usrmod1003_min_passwd_age: ::DWORD,
}
pub type PUSER_MODALS_INFO_1003 = *mut USER_MODALS_INFO_1003;
pub type LPUSER_MODALS_INFO_1003 = *mut USER_MODALS_INFO_1003;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct USER_MODALS_INFO_1004 {
    pub usrmod1004_force_logoff: ::DWORD,
}
pub type PUSER_MODALS_INFO_1004 = *mut USER_MODALS_INFO_1004;
pub type LPUSER_MODALS_INFO_1004 = *mut USER_MODALS_INFO_1004;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct USER_MODALS_INFO_1005 {
    pub usrmod1005_password_hist_len: ::DWORD,
}
pub type PUSER_MODALS_INFO_1005 = *mut USER_MODALS_INFO_1005;
pub type LPUSER_MODALS_INFO_1005 = *mut USER_MODALS_INFO_1005;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct USER_MODALS_INFO_1006 {
    pub usrmod1006_role: ::DWORD,
}
pub type PUSER_MODALS_INFO_1006 = *mut USER_MODALS_INFO_1006;
pub type LPUSER_MODALS_INFO_1006 = *mut USER_MODALS_INFO_1006;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct USER_MODALS_INFO_1007 {
    pub usrmod1007_primary: ::LPWSTR,
}
pub type PUSER_MODALS_INFO_1007 = *mut USER_MODALS_INFO_1007;
pub type LPUSER_MODALS_INFO_1007 = *mut USER_MODALS_INFO_1007;
pub const UF_SCRIPT: ::DWORD = 0x0001;
pub const UF_ACCOUNTDISABLE: ::DWORD = 0x0002;
pub const UF_HOMEDIR_REQUIRED: ::DWORD = 0x0008;
pub const UF_LOCKOUT: ::DWORD = 0x0010;
pub const UF_PASSWD_NOTREQD: ::DWORD = 0x0020;
pub const UF_PASSWD_CANT_CHANGE: ::DWORD = 0x0040;
pub const UF_ENCRYPTED_TEXT_PASSWORD_ALLOWED: ::DWORD = 0x0080;
pub const UF_TEMP_DUPLICATE_ACCOUNT: ::DWORD = 0x0100;
pub const UF_NORMAL_ACCOUNT: ::DWORD = 0x0200;
pub const UF_INTERDOMAIN_TRUST_ACCOUNT: ::DWORD = 0x0800;
pub const UF_WORKSTATION_TRUST_ACCOUNT: ::DWORD = 0x1000;
pub const UF_SERVER_TRUST_ACCOUNT: ::DWORD = 0x2000;
pub const UF_MACHINE_ACCOUNT_MASK: ::DWORD = UF_INTERDOMAIN_TRUST_ACCOUNT
    | UF_WORKSTATION_TRUST_ACCOUNT | UF_SERVER_TRUST_ACCOUNT;
pub const UF_ACCOUNT_TYPE_MASK: ::DWORD = UF_TEMP_DUPLICATE_ACCOUNT | UF_NORMAL_ACCOUNT
    | UF_INTERDOMAIN_TRUST_ACCOUNT |  UF_WORKSTATION_TRUST_ACCOUNT |  UF_SERVER_TRUST_ACCOUNT;
pub const UF_DONT_EXPIRE_PASSWD: ::DWORD = 0x10000;
pub const UF_MNS_LOGON_ACCOUNT: ::DWORD = 0x20000;
pub const UF_SMARTCARD_REQUIRED: ::DWORD = 0x40000;
pub const UF_TRUSTED_FOR_DELEGATION: ::DWORD = 0x80000;
pub const UF_NOT_DELEGATED: ::DWORD = 0x100000;
pub const UF_USE_DES_KEY_ONLY: ::DWORD = 0x200000;
pub const UF_DONT_REQUIRE_PREAUTH: ::DWORD = 0x400000;
pub const UF_PASSWORD_EXPIRED: ::DWORD = 0x800000;
pub const UF_TRUSTED_TO_AUTHENTICATE_FOR_DELEGATION: ::DWORD = 0x1000000;
pub const UF_NO_AUTH_DATA_REQUIRED: ::DWORD = 0x2000000;
pub const UF_PARTIAL_SECRETS_ACCOUNT: ::DWORD = 0x4000000;
pub const UF_USE_AES_KEYS: ::DWORD = 0x8000000;
pub const UF_SETTABLE_BITS: ::DWORD = UF_SCRIPT |  UF_ACCOUNTDISABLE |  UF_LOCKOUT
    | UF_HOMEDIR_REQUIRED | UF_PASSWD_NOTREQD | UF_PASSWD_CANT_CHANGE | UF_ACCOUNT_TYPE_MASK
    | UF_DONT_EXPIRE_PASSWD | UF_MNS_LOGON_ACCOUNT | UF_ENCRYPTED_TEXT_PASSWORD_ALLOWED
    | UF_SMARTCARD_REQUIRED | UF_TRUSTED_FOR_DELEGATION | UF_NOT_DELEGATED | UF_USE_DES_KEY_ONLY
    | UF_DONT_REQUIRE_PREAUTH | UF_PASSWORD_EXPIRED | UF_TRUSTED_TO_AUTHENTICATE_FOR_DELEGATION
    | UF_NO_AUTH_DATA_REQUIRED | UF_USE_AES_KEYS | UF_PARTIAL_SECRETS_ACCOUNT;
pub const FILTER_TEMP_DUPLICATE_ACCOUNT: ::DWORD = 0x0001;
pub const FILTER_NORMAL_ACCOUNT: ::DWORD = 0x0002;
pub const FILTER_INTERDOMAIN_TRUST_ACCOUNT: ::DWORD = 0x0008;
pub const FILTER_WORKSTATION_TRUST_ACCOUNT: ::DWORD = 0x0010;
pub const FILTER_SERVER_TRUST_ACCOUNT: ::DWORD = 0x0020;
pub const LG_INCLUDE_INDIRECT: ::DWORD = 0x0001;
pub const AF_OP_PRINT: ::DWORD = 0x1;
pub const AF_OP_COMM: ::DWORD = 0x2;
pub const AF_OP_SERVER: ::DWORD = 0x4;
pub const AF_OP_ACCOUNTS: ::DWORD = 0x8;
pub const AF_SETTABLE_BITS: ::DWORD = AF_OP_PRINT | AF_OP_COMM | AF_OP_SERVER | AF_OP_ACCOUNTS;
pub const UAS_ROLE_STANDALONE: ::DWORD = 0;
pub const UAS_ROLE_MEMBER: ::DWORD = 1;
pub const UAS_ROLE_BACKUP: ::DWORD = 2;
pub const UAS_ROLE_PRIMARY: ::DWORD = 3;
pub const USER_NAME_PARMNUM: ::DWORD = 1;
pub const USER_PASSWORD_PARMNUM: ::DWORD = 3;
pub const USER_PASSWORD_AGE_PARMNUM: ::DWORD = 4;
pub const USER_PRIV_PARMNUM: ::DWORD = 5;
pub const USER_HOME_DIR_PARMNUM: ::DWORD = 6;
pub const USER_COMMENT_PARMNUM: ::DWORD = 7;
pub const USER_FLAGS_PARMNUM: ::DWORD = 8;
pub const USER_SCRIPT_PATH_PARMNUM: ::DWORD = 9;
pub const USER_AUTH_FLAGS_PARMNUM: ::DWORD = 10;
pub const USER_FULL_NAME_PARMNUM: ::DWORD = 11;
pub const USER_USR_COMMENT_PARMNUM: ::DWORD = 12;
pub const USER_PARMS_PARMNUM: ::DWORD = 13;
pub const USER_WORKSTATIONS_PARMNUM: ::DWORD = 14;
pub const USER_LAST_LOGON_PARMNUM: ::DWORD = 15;
pub const USER_LAST_LOGOFF_PARMNUM: ::DWORD = 16;
pub const USER_ACCT_EXPIRES_PARMNUM: ::DWORD = 17;
pub const USER_MAX_STORAGE_PARMNUM: ::DWORD = 18;
pub const USER_UNITS_PER_WEEK_PARMNUM: ::DWORD = 19;
pub const USER_LOGON_HOURS_PARMNUM: ::DWORD = 20;
pub const USER_PAD_PW_COUNT_PARMNUM: ::DWORD = 21;
pub const USER_NUM_LOGONS_PARMNUM: ::DWORD = 22;
pub const USER_LOGON_SERVER_PARMNUM: ::DWORD = 23;
pub const USER_COUNTRY_CODE_PARMNUM: ::DWORD = 24;
pub const USER_CODE_PAGE_PARMNUM: ::DWORD = 25;
pub const USER_PRIMARY_GROUP_PARMNUM: ::DWORD = 51;
pub const USER_PROFILE: ::DWORD = 52;
pub const USER_PROFILE_PARMNUM: ::DWORD = 52;
pub const USER_HOME_DIR_DRIVE_PARMNUM: ::DWORD = 53;
pub const USER_NAME_INFOLEVEL: ::DWORD = ::PARMNUM_BASE_INFOLEVEL + USER_NAME_PARMNUM;
pub const USER_PASSWORD_INFOLEVEL: ::DWORD = ::PARMNUM_BASE_INFOLEVEL + USER_PASSWORD_PARMNUM;
pub const USER_PASSWORD_AGE_INFOLEVEL: ::DWORD =
    ::PARMNUM_BASE_INFOLEVEL + USER_PASSWORD_AGE_PARMNUM;
pub const USER_PRIV_INFOLEVEL: ::DWORD = ::PARMNUM_BASE_INFOLEVEL + USER_PRIV_PARMNUM;
pub const USER_HOME_DIR_INFOLEVEL: ::DWORD = ::PARMNUM_BASE_INFOLEVEL + USER_HOME_DIR_PARMNUM;
pub const USER_COMMENT_INFOLEVEL: ::DWORD = ::PARMNUM_BASE_INFOLEVEL + USER_COMMENT_PARMNUM;
pub const USER_FLAGS_INFOLEVEL: ::DWORD = ::PARMNUM_BASE_INFOLEVEL + USER_FLAGS_PARMNUM;
pub const USER_SCRIPT_PATH_INFOLEVEL: ::DWORD =
    ::PARMNUM_BASE_INFOLEVEL + USER_SCRIPT_PATH_PARMNUM;
pub const USER_AUTH_FLAGS_INFOLEVEL: ::DWORD = ::PARMNUM_BASE_INFOLEVEL + USER_AUTH_FLAGS_PARMNUM;
pub const USER_FULL_NAME_INFOLEVEL: ::DWORD = ::PARMNUM_BASE_INFOLEVEL + USER_FULL_NAME_PARMNUM;
pub const USER_USR_COMMENT_INFOLEVEL: ::DWORD =
    ::PARMNUM_BASE_INFOLEVEL + USER_USR_COMMENT_PARMNUM;
pub const USER_PARMS_INFOLEVEL: ::DWORD = ::PARMNUM_BASE_INFOLEVEL + USER_PARMS_PARMNUM;
pub const USER_WORKSTATIONS_INFOLEVEL: ::DWORD =
    ::PARMNUM_BASE_INFOLEVEL + USER_WORKSTATIONS_PARMNUM;
pub const USER_LAST_LOGON_INFOLEVEL: ::DWORD = ::PARMNUM_BASE_INFOLEVEL + USER_LAST_LOGON_PARMNUM;
pub const USER_LAST_LOGOFF_INFOLEVEL: ::DWORD =
    ::PARMNUM_BASE_INFOLEVEL + USER_LAST_LOGOFF_PARMNUM;
pub const USER_ACCT_EXPIRES_INFOLEVEL: ::DWORD =
    ::PARMNUM_BASE_INFOLEVEL + USER_ACCT_EXPIRES_PARMNUM;
pub const USER_MAX_STORAGE_INFOLEVEL: ::DWORD = ::PARMNUM_BASE_INFOLEVEL + USER_MAX_STORAGE_PARMNUM;
pub const USER_UNITS_PER_WEEK_INFOLEVEL: ::DWORD =
    ::PARMNUM_BASE_INFOLEVEL + USER_UNITS_PER_WEEK_PARMNUM;
pub const USER_LOGON_HOURS_INFOLEVEL: ::DWORD =
    ::PARMNUM_BASE_INFOLEVEL + USER_LOGON_HOURS_PARMNUM;
pub const USER_PAD_PW_COUNT_INFOLEVEL: ::DWORD =
    ::PARMNUM_BASE_INFOLEVEL + USER_PAD_PW_COUNT_PARMNUM;
pub const USER_NUM_LOGONS_INFOLEVEL: ::DWORD = ::PARMNUM_BASE_INFOLEVEL + USER_NUM_LOGONS_PARMNUM;
pub const USER_LOGON_SERVER_INFOLEVEL: ::DWORD =
    ::PARMNUM_BASE_INFOLEVEL + USER_LOGON_SERVER_PARMNUM;
pub const USER_COUNTRY_CODE_INFOLEVEL: ::DWORD =
    ::PARMNUM_BASE_INFOLEVEL + USER_COUNTRY_CODE_PARMNUM;
pub const USER_CODE_PAGE_INFOLEVEL: ::DWORD = ::PARMNUM_BASE_INFOLEVEL + USER_CODE_PAGE_PARMNUM;
pub const USER_PRIMARY_GROUP_INFOLEVEL: ::DWORD =
    ::PARMNUM_BASE_INFOLEVEL + USER_PRIMARY_GROUP_PARMNUM;
pub const USER_HOME_DIR_DRIVE_INFOLEVEL: ::DWORD =
    ::PARMNUM_BASE_INFOLEVEL + USER_HOME_DIR_DRIVE_PARMNUM;
pub const TIMEQ_FOREVER: ::DWORD = -1i32 as ::DWORD;
pub const USER_MAXSTORAGE_UNLIMITED: ::DWORD = -1i32 as ::DWORD;
pub const USER_NO_LOGOFF: ::DWORD = -1i32 as ::DWORD;
pub const UNITS_PER_DAY: ::DWORD = 24;
pub const UNITS_PER_WEEK: ::DWORD = UNITS_PER_DAY * 7;
pub const USER_PRIV_MASK: ::DWORD = 0x3;
pub const USER_PRIV_GUEST: ::DWORD = 0;
pub const USER_PRIV_USER: ::DWORD = 1;
pub const USER_PRIV_ADMIN: ::DWORD = 2;
pub const MAX_PASSWD_LEN: ::DWORD = ::PWLEN;
pub const DEF_MIN_PWLEN: ::DWORD = 6;
pub const DEF_PWUNIQUENESS: ::DWORD = 5;
pub const DEF_MAX_PWHIST: ::DWORD = 8;
pub const DEF_MAX_PWAGE: ::DWORD = TIMEQ_FOREVER;
pub const DEF_MIN_PWAGE: ::DWORD = 0;
pub const DEF_FORCE_LOGOFF: ::DWORD = 0xffffffff;
pub const DEF_MAX_BADPW: ::DWORD = 0;
pub const ONE_DAY: ::DWORD = 1 * 24 * 3600;
pub const VALIDATED_LOGON: ::DWORD = 0;
pub const PASSWORD_EXPIRED: ::DWORD = 2;
pub const NON_VALIDATED_LOGON: ::DWORD = 3;
pub const VALID_LOGOFF: ::DWORD = 1;
pub const MODALS_MIN_PASSWD_LEN_PARMNUM: ::DWORD = 1;
pub const MODALS_MAX_PASSWD_AGE_PARMNUM: ::DWORD = 2;
pub const MODALS_MIN_PASSWD_AGE_PARMNUM: ::DWORD = 3;
pub const MODALS_FORCE_LOGOFF_PARMNUM: ::DWORD = 4;
pub const MODALS_PASSWD_HIST_LEN_PARMNUM: ::DWORD = 5;
pub const MODALS_ROLE_PARMNUM: ::DWORD = 6;
pub const MODALS_PRIMARY_PARMNUM: ::DWORD = 7;
pub const MODALS_DOMAIN_NAME_PARMNUM: ::DWORD = 8;
pub const MODALS_DOMAIN_ID_PARMNUM: ::DWORD = 9;
pub const MODALS_LOCKOUT_DURATION_PARMNUM: ::DWORD = 10;
pub const MODALS_LOCKOUT_OBSERVATION_WINDOW_PARMNUM: ::DWORD = 11;
pub const MODALS_LOCKOUT_THRESHOLD_PARMNUM: ::DWORD = 12;
pub const MODALS_MIN_PASSWD_LEN_INFOLEVEL: ::DWORD =
    ::PARMNUM_BASE_INFOLEVEL + MODALS_MIN_PASSWD_LEN_PARMNUM;
pub const MODALS_MAX_PASSWD_AGE_INFOLEVEL: ::DWORD =
    ::PARMNUM_BASE_INFOLEVEL + MODALS_MAX_PASSWD_AGE_PARMNUM;
pub const MODALS_MIN_PASSWD_AGE_INFOLEVEL: ::DWORD =
    ::PARMNUM_BASE_INFOLEVEL + MODALS_MIN_PASSWD_AGE_PARMNUM;
pub const MODALS_FORCE_LOGOFF_INFOLEVEL: ::DWORD =
    ::PARMNUM_BASE_INFOLEVEL + MODALS_FORCE_LOGOFF_PARMNUM;
pub const MODALS_PASSWD_HIST_LEN_INFOLEVEL: ::DWORD =
    ::PARMNUM_BASE_INFOLEVEL + MODALS_PASSWD_HIST_LEN_PARMNUM;
pub const MODALS_ROLE_INFOLEVEL: ::DWORD = ::PARMNUM_BASE_INFOLEVEL + MODALS_ROLE_PARMNUM;
pub const MODALS_PRIMARY_INFOLEVEL: ::DWORD = ::PARMNUM_BASE_INFOLEVEL + MODALS_PRIMARY_PARMNUM;
pub const MODALS_DOMAIN_NAME_INFOLEVEL: ::DWORD =
    ::PARMNUM_BASE_INFOLEVEL + MODALS_DOMAIN_NAME_PARMNUM;
pub const MODALS_DOMAIN_ID_INFOLEVEL: ::DWORD = ::PARMNUM_BASE_INFOLEVEL + MODALS_DOMAIN_ID_PARMNUM;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct GROUP_INFO_0 {
    pub grpi0_name: ::LPWSTR,
}
pub type PGROUP_INFO_0 = *mut GROUP_INFO_0;
pub type LPGROUP_INFO_0 = *mut GROUP_INFO_0;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct GROUP_INFO_1 {
    pub grpi1_name: ::LPWSTR,
    pub grpi1_comment: ::LPWSTR,
}
pub type PGROUP_INFO_1 = *mut GROUP_INFO_1;
pub type LPGROUP_INFO_1 = *mut GROUP_INFO_1;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct GROUP_INFO_2 {
    pub grpi2_name: ::LPWSTR,
    pub grpi2_comment: ::LPWSTR,
    pub grpi2_group_id: ::DWORD,
    pub grpi2_attributes: ::DWORD,
}
pub type PGROUP_INFO_2 = *mut GROUP_INFO_2;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct GROUP_INFO_3 {
    pub grpi3_name: ::LPWSTR,
    pub grpi3_comment: ::LPWSTR,
    pub grpi3_group_sid: ::PSID,
    pub grpi3_attributes: ::DWORD,
}
pub type PGROUP_INFO_3 = *mut GROUP_INFO_3;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct GROUP_INFO_1002 {
    pub grpi1002_comment: ::LPWSTR,
}
pub type PGROUP_INFO_1002 = *mut GROUP_INFO_1002;
pub type LPGROUP_INFO_1002 = *mut GROUP_INFO_1002;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct GROUP_INFO_1005 {
    pub grpi1005_attributes: ::DWORD,
}
pub type PGROUP_INFO_1005 = *mut GROUP_INFO_1005;
pub type LPGROUP_INFO_1005 = *mut GROUP_INFO_1005;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct GROUP_USERS_INFO_0 {
    pub grui0_name: ::LPWSTR,
}
pub type PGROUP_USERS_INFO_0 = *mut GROUP_USERS_INFO_0;
pub type LPGROUP_USERS_INFO_0 = *mut GROUP_USERS_INFO_0;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct GROUP_USERS_INFO_1 {
    pub grui1_name: ::LPWSTR,
    pub grui1_attributes: ::DWORD,
}
pub type PGROUP_USERS_INFO_1 = *mut GROUP_USERS_INFO_1;
pub type LPGROUP_USERS_INFO_1 = *mut GROUP_USERS_INFO_1;
pub const GROUPIDMASK: ::DWORD = 0x8000;
pub const GROUP_SPECIALGRP_USERS: &'static str = "USERS";
pub const GROUP_SPECIALGRP_ADMINS: &'static str = "ADMINS";
pub const GROUP_SPECIALGRP_GUESTS: &'static str = "GUESTS";
pub const GROUP_SPECIALGRP_LOCAL: &'static str = "LOCAL";
pub const GROUP_ALL_PARMNUM: ::DWORD = 0;
pub const GROUP_NAME_PARMNUM: ::DWORD = 1;
pub const GROUP_COMMENT_PARMNUM: ::DWORD = 2;
pub const GROUP_ATTRIBUTES_PARMNUM: ::DWORD = 3;
pub const GROUP_ALL_INFOLEVEL: ::DWORD = ::PARMNUM_BASE_INFOLEVEL + GROUP_ALL_PARMNUM;
pub const GROUP_NAME_INFOLEVEL: ::DWORD = ::PARMNUM_BASE_INFOLEVEL + GROUP_NAME_PARMNUM;
pub const GROUP_COMMENT_INFOLEVEL: ::DWORD = ::PARMNUM_BASE_INFOLEVEL + GROUP_COMMENT_PARMNUM;
pub const GROUP_ATTRIBUTES_INFOLEVEL: ::DWORD = ::PARMNUM_BASE_INFOLEVEL + GROUP_ATTRIBUTES_PARMNUM;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct LOCALGROUP_INFO_0 {
    pub lgrpi0_name: ::LPWSTR,
}
pub type PLOCALGROUP_INFO_0 = *mut LOCALGROUP_INFO_0;
pub type LPLOCALGROUP_INFO_0 = *mut LOCALGROUP_INFO_0;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct LOCALGROUP_INFO_1 {
    pub lgrpi1_name: ::LPWSTR,
    pub lgrpi1_comment: ::LPWSTR,
}
pub type PLOCALGROUP_INFO_1 = *mut LOCALGROUP_INFO_1;
pub type LPLOCALGROUP_INFO_1 = *mut LOCALGROUP_INFO_1;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct LOCALGROUP_INFO_1002 {
    pub lgrpi1002_comment: ::LPWSTR,
}
pub type PLOCALGROUP_INFO_1002 = *mut LOCALGROUP_INFO_1002;
pub type LPLOCALGROUP_INFO_1002 = *mut LOCALGROUP_INFO_1002;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct LOCALGROUP_MEMBERS_INFO_0 {
    pub lgrmi0_sid: ::PSID,
}
pub type PLOCALGROUP_MEMBERS_INFO_0 = *mut LOCALGROUP_MEMBERS_INFO_0;
pub type LPLOCALGROUP_MEMBERS_INFO_0 = *mut LOCALGROUP_MEMBERS_INFO_0;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct LOCALGROUP_MEMBERS_INFO_1 {
    pub lgrmi1_sid: ::PSID,
    pub lgrmi1_sidusage: ::SID_NAME_USE,
    pub lgrmi1_name: ::LPWSTR,
}
pub type PLOCALGROUP_MEMBERS_INFO_1 = *mut LOCALGROUP_MEMBERS_INFO_1;
pub type LPLOCALGROUP_MEMBERS_INFO_1 = *mut LOCALGROUP_MEMBERS_INFO_1;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct LOCALGROUP_MEMBERS_INFO_2 {
    pub lgrmi2_sid: ::PSID,
    pub lgrmi2_sidusage: ::SID_NAME_USE,
    pub lgrmi2_domainandname: ::LPWSTR,
}
pub type PLOCALGROUP_MEMBERS_INFO_2 = *mut LOCALGROUP_MEMBERS_INFO_2;
pub type LPLOCALGROUP_MEMBERS_INFO_2 = *mut LOCALGROUP_MEMBERS_INFO_2;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct LOCALGROUP_MEMBERS_INFO_3 {
    pub lgrmi3_domainandname: ::LPWSTR,
}
pub type PLOCALGROUP_MEMBERS_INFO_3 = *mut LOCALGROUP_MEMBERS_INFO_3;
pub type LPLOCALGROUP_MEMBERS_INFO_3 = *mut LOCALGROUP_MEMBERS_INFO_3;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct LOCALGROUP_USERS_INFO_0 {
    pub lgrui0_name: ::LPWSTR,
}
pub type PLOCALGROUP_USERS_INFO_0 = *mut LOCALGROUP_USERS_INFO_0;
pub type LPLOCALGROUP_USERS_INFO_0 = *mut LOCALGROUP_USERS_INFO_0;
pub const LOCALGROUP_NAME_PARMNUM: ::DWORD = 1;
pub const LOCALGROUP_COMMENT_PARMNUM: ::DWORD = 2;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NET_DISPLAY_USER {
    pub usri1_name: ::LPWSTR,
    pub usri1_comment: ::LPWSTR,
    pub usri1_flags: ::DWORD,
    pub usri1_full_name: ::LPWSTR,
    pub usri1_user_id: ::DWORD,
    pub usri1_next_index: ::DWORD,
}
pub type PNET_DISPLAY_USER = *mut NET_DISPLAY_USER;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NET_DISPLAY_MACHINE {
    pub usri2_name: ::LPWSTR,
    pub usri2_comment: ::LPWSTR,
    pub usri2_flags: ::DWORD,
    pub usri2_user_id: ::DWORD,
    pub usri2_next_index: ::DWORD,
}
pub type PNET_DISPLAY_MACHINE = *mut NET_DISPLAY_MACHINE;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NET_DISPLAY_GROUP {
    pub usri3_name: ::LPWSTR,
    pub usri3_comment: ::LPWSTR,
    pub grpi3_group_id: ::DWORD,
    pub grpi3_attributes: ::DWORD,
    pub grpi3_next_index: ::DWORD,
}
pub type PNET_DISPLAY_GROUP = *mut NET_DISPLAY_GROUP;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ACCESS_INFO_0 {
    pub acc0_resource_name: ::LPWSTR,
}
pub type PACCESS_INFO_0 = *mut ACCESS_INFO_0;
pub type LPACCESS_INFO_0 = *mut ACCESS_INFO_0;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ACCESS_INFO_1 {
    pub acc1_resource_name: ::LPWSTR,
    pub acc1_attr: ::DWORD,
    pub acc1_count: ::DWORD,
}
pub type PACCESS_INFO_1 = *mut ACCESS_INFO_1;
pub type LPACCESS_INFO_1 = *mut ACCESS_INFO_1;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ACCESS_INFO_1002 {
    pub acc1002_attr: ::DWORD,
}
pub type PACCESS_INFO_1002 = *mut ACCESS_INFO_1002;
pub type LPACCESS_INFO_1002 = *mut ACCESS_INFO_1002;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ACCESS_LIST {
    pub acl_ugname: ::LPWSTR,
    pub acl_access: ::DWORD,
}
pub type PACCESS_LIST = *mut ACCESS_LIST;
pub type LPACCESS_LIST = *mut ACCESS_LIST;
pub const ACCESS_NONE: ::DWORD = 0;
pub const ACCESS_ALL: ::DWORD = ACCESS_READ | ACCESS_WRITE | ACCESS_CREATE | ACCESS_EXEC
    | ACCESS_DELETE | ACCESS_ATRIB | ACCESS_PERM;
pub const ACCESS_READ: ::DWORD = 0x01;
pub const ACCESS_WRITE: ::DWORD = 0x02;
pub const ACCESS_CREATE: ::DWORD = 0x04;
pub const ACCESS_EXEC: ::DWORD = 0x08;
pub const ACCESS_DELETE: ::DWORD = 0x10;
pub const ACCESS_ATRIB: ::DWORD = 0x20;
pub const ACCESS_PERM: ::DWORD = 0x40;
pub const ACCESS_GROUP: ::DWORD = 0x8000;
pub const ACCESS_AUDIT: ::DWORD = 0x1;
pub const ACCESS_SUCCESS_OPEN: ::DWORD = 0x10;
pub const ACCESS_SUCCESS_WRITE: ::DWORD = 0x20;
pub const ACCESS_SUCCESS_DELETE: ::DWORD = 0x40;
pub const ACCESS_SUCCESS_ACL: ::DWORD = 0x80;
pub const ACCESS_SUCCESS_MASK: ::DWORD = 0xF0;
pub const ACCESS_FAIL_OPEN: ::DWORD = 0x100;
pub const ACCESS_FAIL_WRITE: ::DWORD = 0x200;
pub const ACCESS_FAIL_DELETE: ::DWORD = 0x400;
pub const ACCESS_FAIL_ACL: ::DWORD = 0x800;
pub const ACCESS_FAIL_MASK: ::DWORD = 0xF00;
pub const ACCESS_FAIL_SHIFT: ::DWORD = 4;
pub const ACCESS_RESOURCE_NAME_PARMNUM: ::DWORD = 1;
pub const ACCESS_ATTR_PARMNUM: ::DWORD = 2;
pub const ACCESS_COUNT_PARMNUM: ::DWORD = 3;
pub const ACCESS_ACCESS_LIST_PARMNUM: ::DWORD = 4;
pub const ACCESS_RESOURCE_NAME_INFOLEVEL: ::DWORD =
    ::PARMNUM_BASE_INFOLEVEL + ACCESS_RESOURCE_NAME_PARMNUM;
pub const ACCESS_ATTR_INFOLEVEL: ::DWORD = ::PARMNUM_BASE_INFOLEVEL + ACCESS_ATTR_PARMNUM;
pub const ACCESS_COUNT_INFOLEVEL: ::DWORD = ::PARMNUM_BASE_INFOLEVEL + ACCESS_COUNT_PARMNUM;
pub const ACCESS_ACCESS_LIST_INFOLEVEL: ::DWORD =
    ::PARMNUM_BASE_INFOLEVEL + ACCESS_ACCESS_LIST_PARMNUM;
ENUM!{enum NET_VALIDATE_PASSWORD_TYPE {
    NetValidateAuthentication = 1,
    NetValidatePasswordChange,
    NetValidatePasswordReset,
}}
pub type PNET_VALIDATE_PASSWORD_TYPE = *mut NET_VALIDATE_PASSWORD_TYPE;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NET_VALIDATE_PASSWORD_HASH {
    pub Length: ::ULONG,
    pub Hash: ::LPBYTE,
}
pub type PNET_VALIDATE_PASSWORD_HASH = *mut NET_VALIDATE_PASSWORD_HASH;
pub const NET_VALIDATE_PASSWORD_LAST_SET: ::ULONG = 0x00000001;
pub const NET_VALIDATE_BAD_PASSWORD_TIME: ::ULONG = 0x00000002;
pub const NET_VALIDATE_LOCKOUT_TIME: ::ULONG = 0x00000004;
pub const NET_VALIDATE_BAD_PASSWORD_COUNT: ::ULONG = 0x00000008;
pub const NET_VALIDATE_PASSWORD_HISTORY_LENGTH: ::ULONG = 0x00000010;
pub const NET_VALIDATE_PASSWORD_HISTORY: ::ULONG = 0x00000020;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NET_VALIDATE_PERSISTED_FIELDS {
    pub PresentFields: ::ULONG,
    pub PasswordLastSet: ::FILETIME,
    pub BadPasswordTime: ::FILETIME,
    pub LockoutTime: ::FILETIME,
    pub BadPasswordCount: ::ULONG,
    pub PasswordHistoryLength: ::ULONG,
    pub PasswordHistory: PNET_VALIDATE_PASSWORD_HASH,
}
pub type PNET_VALIDATE_PERSISTED_FIELDS = *mut NET_VALIDATE_PERSISTED_FIELDS;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NET_VALIDATE_OUTPUT_ARG {
    pub ChangedPersistedFields: NET_VALIDATE_PERSISTED_FIELDS,
    pub ValidationStatus: ::NET_API_STATUS,
}
pub type PNET_VALIDATE_OUTPUT_ARG = *mut NET_VALIDATE_OUTPUT_ARG;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NET_VALIDATE_AUTHENTICATION_INPUT_ARG {
    pub InputPersistedFields: NET_VALIDATE_PERSISTED_FIELDS,
    pub PasswordMatched: ::BOOLEAN,
}
pub type PNET_VALIDATE_AUTHENTICATION_INPUT_ARG = *mut NET_VALIDATE_AUTHENTICATION_INPUT_ARG;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NET_VALIDATE_PASSWORD_CHANGE_INPUT_ARG {
    pub InputPersistedFields: NET_VALIDATE_PERSISTED_FIELDS,
    pub ClearPassword: ::LPWSTR,
    pub UserAccountName: ::LPWSTR,
    pub HashedPassword: NET_VALIDATE_PASSWORD_HASH,
    pub PasswordMatch: ::BOOLEAN,
}
pub type PNET_VALIDATE_PASSWORD_CHANGE_INPUT_ARG = *mut NET_VALIDATE_PASSWORD_CHANGE_INPUT_ARG;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NET_VALIDATE_PASSWORD_RESET_INPUT_ARG {
    pub InputPersistedFields: NET_VALIDATE_PERSISTED_FIELDS,
    pub ClearPassword: ::LPWSTR,
    pub UserAccountName: ::LPWSTR,
    pub HashedPassword: NET_VALIDATE_PASSWORD_HASH,
    pub PasswordMustChangeAtNextLogon: ::BOOLEAN,
    pub ClearLockout: ::BOOLEAN,
}
pub type PNET_VALIDATE_PASSWORD_RESET_INPUT_ARG = *mut NET_VALIDATE_PASSWORD_RESET_INPUT_ARG;
pub const NETLOGON_CONTROL_QUERY: ::DWORD = 1;
pub const NETLOGON_CONTROL_REPLICATE: ::DWORD = 2;
pub const NETLOGON_CONTROL_SYNCHRONIZE: ::DWORD = 3;
pub const NETLOGON_CONTROL_PDC_REPLICATE: ::DWORD = 4;
pub const NETLOGON_CONTROL_REDISCOVER: ::DWORD = 5;
pub const NETLOGON_CONTROL_TC_QUERY: ::DWORD = 6;
pub const NETLOGON_CONTROL_TRANSPORT_NOTIFY: ::DWORD = 7;
pub const NETLOGON_CONTROL_FIND_USER: ::DWORD = 8;
pub const NETLOGON_CONTROL_CHANGE_PASSWORD: ::DWORD = 9;
pub const NETLOGON_CONTROL_TC_VERIFY: ::DWORD = 10;
pub const NETLOGON_CONTROL_FORCE_DNS_REG: ::DWORD = 11;
pub const NETLOGON_CONTROL_QUERY_DNS_REG: ::DWORD = 12;
pub const NETLOGON_CONTROL_UNLOAD_NETLOGON_DLL: ::DWORD = 0xFFFB;
pub const NETLOGON_CONTROL_BACKUP_CHANGE_LOG: ::DWORD = 0xFFFC;
pub const NETLOGON_CONTROL_TRUNCATE_LOG: ::DWORD = 0xFFFD;
pub const NETLOGON_CONTROL_SET_DBFLAG: ::DWORD = 0xFFFE;
pub const NETLOGON_CONTROL_BREAKPOINT: ::DWORD = 0xFFFF;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NETLOGON_INFO_1 {
    pub netlog1_flags: ::DWORD,
    pub netlog1_pdc_connection_status: ::NET_API_STATUS,
}
pub type PNETLOGON_INFO_1 = *mut NETLOGON_INFO_1;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NETLOGON_INFO_2 {
    pub netlog2_flags: ::DWORD,
    pub netlog2_pdc_connection_status: ::NET_API_STATUS,
    pub netlog2_trusted_dc_name: ::LPWSTR,
    pub netlog2_tc_connection_status: ::NET_API_STATUS,
}
pub type PNETLOGON_INFO_2 = *mut NETLOGON_INFO_2;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NETLOGON_INFO_3 {
    pub netlog3_flags: ::DWORD,
    pub netlog3_logon_attempts: ::DWORD,
    pub netlog3_reserved1: ::DWORD,
    pub netlog3_reserved2: ::DWORD,
    pub netlog3_reserved3: ::DWORD,
    pub netlog3_reserved4: ::DWORD,
    pub netlog3_reserved5: ::DWORD,
}
pub type PNETLOGON_INFO_3 = *mut NETLOGON_INFO_3;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NETLOGON_INFO_4 {
    pub netlog4_trusted_dc_name: ::LPWSTR,
    pub netlog4_trusted_domain_name: ::LPWSTR,
}
pub type PNETLOGON_INFO_4 = *mut NETLOGON_INFO_4;
pub const NETLOGON_REPLICATION_NEEDED: ::DWORD = 0x01;
pub const NETLOGON_REPLICATION_IN_PROGRESS: ::DWORD = 0x02;
pub const NETLOGON_FULL_SYNC_REPLICATION: ::DWORD = 0x04;
pub const NETLOGON_REDO_NEEDED: ::DWORD = 0x08;
pub const NETLOGON_HAS_IP: ::DWORD = 0x10;
pub const NETLOGON_HAS_TIMESERV: ::DWORD = 0x20;
pub const NETLOGON_DNS_UPDATE_FAILURE: ::DWORD = 0x40;
pub const NETLOGON_VERIFY_STATUS_RETURNED: ::DWORD = 0x80;
DEFINE_GUID!(ServiceAccountPasswordGUID, 0x262E99C9, 0x6160, 0x4871,
    0xAC, 0xEC, 0x4E, 0x61, 0x73, 0x6B, 0x6F, 0x21);
pub const SERVICE_ACCOUNT_FLAG_LINK_TO_HOST_ONLY: ::DWORD = 0x00000001;
pub const SERVICE_ACCOUNT_FLAG_ADD_AGAINST_RODC: ::DWORD = 0x00000002;
pub const SERVICE_ACCOUNT_FLAG_UNLINK_FROM_HOST_ONLY: ::DWORD = 0x00000001;
pub const SERVICE_ACCOUNT_FLAG_REMOVE_OFFLINE: ::DWORD = 0x00000002;
ENUM!{enum MSA_INFO_LEVEL {
    MsaInfoLevel0 = 0,
    MsaInfoLevelMax,
}}
pub type PMSA_INFO_LEVEL = *mut MSA_INFO_LEVEL;
ENUM!{enum MSA_INFO_STATE {
    MsaInfoNotExist = 1,
    MsaInfoNotService,
    MsaInfoCannotInstall,
    MsaInfoCanInstall,
    MsaInfoInstalled,
}}
pub type PMSA_INFO_STATE = *mut MSA_INFO_STATE;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct MSA_INFO_0 {
    pub State: MSA_INFO_STATE,
}
pub type PMSA_INFO_0 = *mut MSA_INFO_0;
pub type LPMSA_INFO_0 = *mut MSA_INFO_0;

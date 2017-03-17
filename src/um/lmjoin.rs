// Copyright © 2015-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// Definitions and prototypes for the Net setup apis
use shared::minwindef::{DWORD};
use um::winnt::{LPWSTR, LPCWSTR};

ENUM!{enum NETSETUP_NAME_TYPE {
    NetSetupUnknown = 0,
    NetSetupMachine,
    NetSetupWorkgroup,
    NetSetupDomain,
    NetSetupNonExistentDomain,
    NetSetupDnsMachine,
}}
pub type PNETSETUP_NAME_TYPE = *mut NETSETUP_NAME_TYPE;
ENUM!{enum NETSETUP_JOIN_STATUS {
    NetSetupUnknownStatus = 0,
    NetSetupUnjoined,
    NetSetupWorkgroupName,
    NetSetupDomainName,
}}
pub type PNETSETUP_JOIN_STATUS = *mut NETSETUP_JOIN_STATUS;
pub const NETSETUP_JOIN_DOMAIN: DWORD = 0x00000001;
pub const NETSETUP_ACCT_CREATE: DWORD = 0x00000002;
pub const NETSETUP_ACCT_DELETE: DWORD = 0x00000004;
pub const NETSETUP_WIN9X_UPGRADE: DWORD = 0x00000010;
pub const NETSETUP_DOMAIN_JOIN_IF_JOINED: DWORD = 0x00000020;
pub const NETSETUP_JOIN_UNSECURE: DWORD = 0x00000040;
pub const NETSETUP_MACHINE_PWD_PASSED: DWORD = 0x00000080;
pub const NETSETUP_DEFER_SPN_SET: DWORD = 0x00000100;
pub const NETSETUP_JOIN_DC_ACCOUNT: DWORD = 0x00000200;
pub const NETSETUP_JOIN_WITH_NEW_NAME: DWORD = 0x00000400;
pub const NETSETUP_JOIN_READONLY: DWORD = 0x00000800;
pub const NETSETUP_DNS_NAME_CHANGES_ONLY: DWORD = 0x00001000;
pub const NETSETUP_INSTALL_INVOCATION: DWORD = 0x00040000;
pub const NETSETUP_AMBIGUOUS_DC: DWORD = 0x00001000;
pub const NETSETUP_NO_NETLOGON_CACHE: DWORD = 0x00002000;
pub const NETSETUP_DONT_CONTROL_SERVICES: DWORD = 0x00004000;
pub const NETSETUP_SET_MACHINE_NAME: DWORD = 0x00008000;
pub const NETSETUP_FORCE_SPN_SET: DWORD = 0x00010000;
pub const NETSETUP_NO_ACCT_REUSE: DWORD = 0x00020000;
pub const NETSETUP_ALT_SAMACCOUNTNAME: DWORD = 0x00020000;
pub const NETSETUP_IGNORE_UNSUPPORTED_FLAGS: DWORD = 0x10000000;
pub const NETSETUP_VALID_UNJOIN_FLAGS: DWORD = NETSETUP_ACCT_DELETE
    | NETSETUP_IGNORE_UNSUPPORTED_FLAGS | NETSETUP_JOIN_DC_ACCOUNT;
pub const NETSETUP_PROCESS_OFFLINE_FLAGS: DWORD = NETSETUP_JOIN_DOMAIN
    | NETSETUP_DOMAIN_JOIN_IF_JOINED | NETSETUP_JOIN_WITH_NEW_NAME | NETSETUP_DONT_CONTROL_SERVICES
    | NETSETUP_MACHINE_PWD_PASSED;
pub const NETSETUP_PROVISION_DOWNLEVEL_PRIV_SUPPORT: DWORD = 0x00000001;
pub const NETSETUP_PROVISION_REUSE_ACCOUNT: DWORD = 0x00000002;
pub const NETSETUP_PROVISION_USE_DEFAULT_PASSWORD: DWORD = 0x00000004;
pub const NETSETUP_PROVISION_SKIP_ACCOUNT_SEARCH: DWORD = 0x00000008;
pub const NETSETUP_PROVISION_ROOT_CA_CERTS: DWORD = 0x00000010;
pub const NETSETUP_PROVISION_PERSISTENTSITE: DWORD = 0x00000020;
pub const NETSETUP_PROVISION_ONLINE_CALLER: DWORD = 0x40000000;
pub const NETSETUP_PROVISION_CHECK_PWD_ONLY: DWORD = 0x80000000;
pub const NETSETUP_PROVISIONING_PARAMS_WIN8_VERSION: DWORD = 0x00000001;
pub const NETSETUP_PROVISIONING_PARAMS_CURRENT_VERSION: DWORD = 0x00000002;
STRUCT!{struct NETSETUP_PROVISIONING_PARAMS {
    dwVersion: DWORD,
    lpDomain: LPCWSTR,
    lpHostName: LPCWSTR,
    lpMachineAccountOU: LPCWSTR,
    lpDcName: LPCWSTR,
    dwProvisionOptions: DWORD,
    aCertTemplateNames: *mut LPCWSTR,
    cCertTemplateNames: DWORD,
    aMachinePolicyNames: *mut LPCWSTR,
    cMachinePolicyNames: DWORD,
    aMachinePolicyPaths: *mut LPCWSTR,
    cMachinePolicyPaths: DWORD,
    lpNetbiosName: LPWSTR,
    lpSiteName: LPWSTR,
    lpPrimaryDNSDomain: LPWSTR,
}}
pub type PNETSETUP_PROVISIONING_PARAMS = *mut NETSETUP_PROVISIONING_PARAMS;
ENUM!{enum NET_COMPUTER_NAME_TYPE {
    NetPrimaryComputerName,
    NetAlternateComputerNames,
    NetAllComputerNames,
    NetComputerNameTypeMax,
}}
pub type PNET_COMPUTER_NAME_TYPE = *mut NET_COMPUTER_NAME_TYPE;

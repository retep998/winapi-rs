// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Public definitions for the Windows LDAP (winldap) library
use ctypes::{c_char, c_int};
use shared::basetsd::{UINT_PTR, ULONG_PTR};
use shared::minwindef::{BYTE, PINT};
use shared::ntdef::{
    BOOLEAN, HANDLE, INT, LONG, LPCSTR, LPCWSTR, LPSTR, LPWSTR, PCHAR, PSTR, PULONG, PVOID, PWCHAR,
    PWSTR, PZPSTR, PZPWSTR, UCHAR, ULONG, USHORT,
};
use um::schannel::PSecPkgContext_IssuerListInfoEx;
use um::wincrypt::PCCERT_CONTEXT;
pub const LDAP_PORT: LONG = 389;
pub const LDAP_SSL_PORT: LONG = 636;
pub const LDAP_GC_PORT: LONG = 3268;
pub const LDAP_SSL_GC_PORT: LONG = 3269;
pub const LDAP_VERSION1: LONG = 1;
pub const LDAP_VERSION2: LONG = 2;
pub const LDAP_VERSION3: LONG = 3;
pub const LDAP_VERSION: LONG = LDAP_VERSION2;
pub const LDAP_BIND_CMD: LONG = 0x60;
pub const LDAP_UNBIND_CMD: LONG = 0x42;
pub const LDAP_SEARCH_CMD: LONG = 0x63;
pub const LDAP_MODIFY_CMD: LONG = 0x66;
pub const LDAP_ADD_CMD: LONG = 0x68;
pub const LDAP_DELETE_CMD: LONG = 0x4a;
pub const LDAP_MODRDN_CMD: LONG = 0x6c;
pub const LDAP_COMPARE_CMD: LONG = 0x6e;
pub const LDAP_ABANDON_CMD: LONG = 0x50;
pub const LDAP_SESSION_CMD: LONG = 0x71;
pub const LDAP_EXTENDED_CMD: LONG = 0x77;
pub const LDAP_RES_BIND: ULONG = 0x61;
pub const LDAP_RES_SEARCH_ENTRY: ULONG = 0x64;
pub const LDAP_RES_SEARCH_RESULT: ULONG = 0x65;
pub const LDAP_RES_MODIFY: ULONG = 0x67;
pub const LDAP_RES_ADD: ULONG = 0x69;
pub const LDAP_RES_DELETE: ULONG = 0x6b;
pub const LDAP_RES_MODRDN: ULONG = 0x6d;
pub const LDAP_RES_COMPARE: ULONG = 0x6f;
pub const LDAP_RES_SESSION: ULONG = 0x72;
pub const LDAP_RES_REFERRAL: ULONG = 0x73;
pub const LDAP_RES_EXTENDED: ULONG = 0x78;
pub const LDAP_RES_ANY: ULONG = -1i32 as u32;
pub const LDAP_INVALID_CMD: LONG = 0xff;
pub const LDAP_INVALID_RES: LONG = 0xff;
pub const LDAP_AUTH_SIMPLE: LONG = 0x80;
pub const LDAP_AUTH_SASL: LONG = 0x83;
pub const LDAP_AUTH_OTHERKIND: LONG = 0x86;
pub const LDAP_AUTH_SICILY: LONG = LDAP_AUTH_OTHERKIND | 0x0200;
pub const LDAP_AUTH_MSN: LONG = LDAP_AUTH_OTHERKIND | 0x0800;
pub const LDAP_AUTH_NTLM: LONG = LDAP_AUTH_OTHERKIND | 0x1000;
pub const LDAP_AUTH_DPA: LONG = LDAP_AUTH_OTHERKIND | 0x2000;
pub const LDAP_AUTH_NEGOTIATE: LONG = LDAP_AUTH_OTHERKIND | 0x0400;
pub const LDAP_AUTH_SSPI: LONG = LDAP_AUTH_NEGOTIATE;
pub const LDAP_AUTH_DIGEST: LONG = LDAP_AUTH_OTHERKIND | 0x4000;
pub const LDAP_AUTH_EXTERNAL: LONG = LDAP_AUTH_OTHERKIND | 0x0020;
pub const LDAP_FILTER_AND: LONG = 0xa0;
pub const LDAP_FILTER_OR: LONG = 0xa1;
pub const LDAP_FILTER_NOT: LONG = 0xa2;
pub const LDAP_FILTER_EQUALITY: LONG = 0xa3;
pub const LDAP_FILTER_SUBSTRINGS: LONG = 0xa4;
pub const LDAP_FILTER_GE: LONG = 0xa5;
pub const LDAP_FILTER_LE: LONG = 0xa6;
pub const LDAP_FILTER_PRESENT: LONG = 0x87;
pub const LDAP_FILTER_APPROX: LONG = 0xa8;
pub const LDAP_FILTER_EXTENSIBLE: LONG = 0xa9;
pub const LDAP_SUBSTRING_INITIAL: LONG = 0x80;
pub const LDAP_SUBSTRING_ANY: LONG = 0x81;
pub const LDAP_SUBSTRING_FINAL: LONG = 0x82;
pub const LDAP_DEREF_NEVER: LONG = 0;
pub const LDAP_DEREF_SEARCHING: LONG = 1;
pub const LDAP_DEREF_FINDING: LONG = 2;
pub const LDAP_DEREF_ALWAYS: LONG = 3;
pub const LDAP_NO_LIMIT: LONG = 0;
pub const LDAP_OPT_API_INFO : LONG = 0x00;
pub const LDAP_OPT_DESC : LONG = 0x01;
pub const LDAP_OPT_DEREF : LONG = 0x02;
pub const LDAP_OPT_SIZELIMIT : LONG = 0x03;
pub const LDAP_OPT_TIMELIMIT : LONG = 0x04;
pub const LDAP_OPT_THREAD_FN_PTRS : LONG = 0x05;
pub const LDAP_OPT_REBIND_FN : LONG = 0x06;
pub const LDAP_OPT_REBIND_ARG : LONG = 0x07;
pub const LDAP_OPT_REFERRALS : LONG = 0x08;
pub const LDAP_OPT_RESTART : LONG = 0x09;
pub const LDAP_OPT_SSL : LONG = 0x0a;
pub const LDAP_OPT_IO_FN_PTRS : LONG = 0x0b;
pub const LDAP_OPT_CACHE_FN_PTRS : LONG = 0x0d;
pub const LDAP_OPT_CACHE_STRATEGY : LONG = 0x0e;
pub const LDAP_OPT_CACHE_ENABLE : LONG = 0x0f;
pub const LDAP_OPT_REFERRAL_HOP_LIMIT : LONG = 0x10;
pub const LDAP_OPT_PROTOCOL_VERSION : LONG = 0x11;
pub const LDAP_OPT_VERSION : LONG = 0x11;
pub const LDAP_OPT_API_FEATURE_INFO : LONG = 0x15;
pub const LDAP_OPT_HOST_NAME : LONG = 0x30;
pub const LDAP_OPT_ERROR_NUMBER : LONG = 0x31;
pub const LDAP_OPT_ERROR_STRING : LONG = 0x32;
pub const LDAP_OPT_SERVER_ERROR : LONG = 0x33;
pub const LDAP_OPT_SERVER_EXT_ERROR : LONG = 0x34;
pub const LDAP_OPT_HOST_REACHABLE : LONG = 0x3E;
pub const LDAP_OPT_PING_KEEP_ALIVE : LONG = 0x36;
pub const LDAP_OPT_PING_WAIT_TIME : LONG = 0x37;
pub const LDAP_OPT_PING_LIMIT : LONG = 0x38;
pub const LDAP_OPT_DNSDOMAIN_NAME : LONG = 0x3B;
pub const LDAP_OPT_GETDSNAME_FLAGS : LONG = 0x3D;
pub const LDAP_OPT_PROMPT_CREDENTIALS : LONG = 0x3F;
pub const LDAP_OPT_AUTO_RECONNECT : LONG = 0x91;
pub const LDAP_OPT_SSPI_FLAGS : LONG = 0x92;
pub const LDAP_OPT_SSL_INFO : LONG = 0x93;
pub const LDAP_OPT_TLS : LONG = LDAP_OPT_SSL;
pub const LDAP_OPT_TLS_INFO : LONG = LDAP_OPT_SSL_INFO;
pub const LDAP_OPT_SIGN : LONG = 0x95;
pub const LDAP_OPT_ENCRYPT : LONG = 0x96;
pub const LDAP_OPT_SASL_METHOD : LONG = 0x97;
pub const LDAP_OPT_AREC_EXCLUSIVE : LONG = 0x98;
pub const LDAP_OPT_SECURITY_CONTEXT : LONG = 0x99;
pub const LDAP_OPT_ROOTDSE_CACHE : LONG = 0x9a;
pub const LDAP_OPT_TCP_KEEPALIVE : LONG = 0x40;
pub const LDAP_OPT_FAST_CONCURRENT_BIND : LONG = 0x41;
pub const LDAP_OPT_SEND_TIMEOUT : LONG = 0x42;
pub const LDAP_OPT_SCH_FLAGS : LONG = 0x43;
pub const LDAP_OPT_SOCKET_BIND_ADDRESSES : LONG = 0x44;
pub const LDAP_OPT_ON : PVOID = 1 as PVOID;
pub const LDAP_OPT_OFF : PVOID = 0 as PVOID;
pub const LDAP_OPT_DNS: LONG = 0x00000001;
pub const LDAP_OPT_CHASE_REFERRALS: LONG = 0x00000002;
pub const LDAP_OPT_RETURN_REFS: LONG = 0x00000004;
pub const LDAP_MOD_BVALUES: LONG = 0x80;
// pub const mod_values : PWCHAR = mod_vals.modv_strvals;
// pub const mod_bvalues : *mut *mut BERVAL = mod_vals.modv_bvals;
pub const LDAP_SCOPE_BASE: LONG = 0x00;
pub const LDAP_SCOPE_ONELEVEL: LONG = 0x01;
pub const LDAP_SCOPE_SUBTREE: LONG = 0x02;
pub const LDAP_MSG_ONE: LONG = 0;
pub const LDAP_MSG_ALL: LONG = 1;
pub const LDAP_MSG_RECEIVED: LONG = 2;
pub const NULLBER: *mut BerElement = 0i32 as *mut BerElement;
pub const LBER_USE_DER: LONG = 0x01;
pub const LBER_USE_INDEFINITE_LEN: LONG = 0x02;
pub const LBER_TRANSLATE_STRINGS: LONG = 0x04;
pub const LAPI_MAJOR_VER1: LONG = 1;
pub const LAPI_MINOR_VER1: LONG = 1;
pub const LDAP_API_INFO_VERSION: LONG = 1;
pub const LDAP_API_VERSION: LONG = 2004;
pub const LDAP_VERSION_MIN: LONG = 2;
pub const LDAP_VERSION_MAX: LONG = 3;
pub const LDAP_VENDOR_NAME: &'static str = "Microsoft Corporation.";
// pub const LDAP_VENDOR_NAME_W : LPWSTR = "Microsoft Corporation.";
pub const LDAP_VENDOR_VERSION: LONG = 510;
pub const LDAP_FEATURE_INFO_VERSION: LONG = 1;
pub const LDAP_SERVER_SORT_OID: &'static str = "1.2.840.113556.1.4.473";
// pub const LDAP_SERVER_SORT_OID_W : LPWSTR = "1.2.840.113556.1.4.473";
pub const LDAP_SERVER_RESP_SORT_OID: &'static str = "1.2.840.113556.1.4.474";
// pub const LDAP_SERVER_RESP_SORT_OID_W : LPWSTR = "1.2.840.113556.1.4.474";
pub const LDAP_PAGED_RESULT_OID_STRING: &'static str = "1.2.840.113556.1.4.319";
// pub const LDAP_PAGED_RESULT_OID_STRING_W : LPWSTR = "1.2.840.113556.1.4.319";
pub const LDAP_CONTROL_VLVREQUEST: &'static str = "2.16.840.1.113730.3.4.9";
// pub const LDAP_CONTROL_VLVREQUEST_W : LPWSTR = "2.16.840.1.113730.3.4.9";
pub const LDAP_CONTROL_VLVRESPONSE: &'static str = "2.16.840.1.113730.3.4.10";
// pub const LDAP_CONTROL_VLVRESPONSE_W : LPWSTR = "2.16.840.1.113730.3.4.10";
pub const LDAP_API_FEATURE_VIRTUAL_LIST_VIEW: LONG = 1001;
pub const LDAP_VLVINFO_VERSION: LONG = 1;
pub const LDAP_START_TLS_OID: &'static str = "1.3.6.1.4.1.1466.20037";
// pub const LDAP_START_TLS_OID_W : LPWSTR = "1.3.6.1.4.1.1466.20037";
pub const LDAP_TTL_EXTENDED_OP_OID: &'static str = "1.3.6.1.4.1.1466.101.119.1";
// pub const LDAP_TTL_EXTENDED_OP_OID_W : LPWSTR = "1.3.6.1.4.1.1466.101.119.1";
pub const LDAP_OPT_REFERRAL_CALLBACK: LONG = 0x70;
pub const LDAP_OPT_CLIENT_CERTIFICATE: LONG = 0x80;
pub const LDAP_OPT_SERVER_CERTIFICATE: LONG = 0x81;
pub const LDAP_OPT_REF_DEREF_CONN_PER_MSG: LONG = 0x94;
pub type LDAP_RETCODE = LONG;
pub const LDAP_SUCCESS: LDAP_RETCODE = 0x00;
pub const LDAP_OPERATIONS_ERROR: LDAP_RETCODE = 0x01;
pub const LDAP_PROTOCOL_ERROR: LDAP_RETCODE = 0x02;
pub const LDAP_TIMELIMIT_EXCEEDED: LDAP_RETCODE = 0x03;
pub const LDAP_SIZELIMIT_EXCEEDED: LDAP_RETCODE = 0x04;
pub const LDAP_COMPARE_FALSE: LDAP_RETCODE = 0x05;
pub const LDAP_COMPARE_TRUE: LDAP_RETCODE = 0x06;
pub const LDAP_AUTH_METHOD_NOT_SUPPORTED: LDAP_RETCODE = 0x07;
pub const LDAP_STRONG_AUTH_REQUIRED: LDAP_RETCODE = 0x08;
pub const LDAP_REFERRAL_V2: LDAP_RETCODE = 0x09;
pub const LDAP_PARTIAL_RESULTS: LDAP_RETCODE = 0x09;
pub const LDAP_REFERRAL: LDAP_RETCODE = 0x0a;
pub const LDAP_ADMIN_LIMIT_EXCEEDED: LDAP_RETCODE = 0x0b;
pub const LDAP_UNAVAILABLE_CRIT_EXTENSION: LDAP_RETCODE = 0x0c;
pub const LDAP_CONFIDENTIALITY_REQUIRED: LDAP_RETCODE = 0x0d;
pub const LDAP_SASL_BIND_IN_PROGRESS: LDAP_RETCODE = 0x0e;
pub const LDAP_NO_SUCH_ATTRIBUTE: LDAP_RETCODE = 0x10;
pub const LDAP_UNDEFINED_TYPE: LDAP_RETCODE = 0x11;
pub const LDAP_INAPPROPRIATE_MATCHING: LDAP_RETCODE = 0x12;
pub const LDAP_CONSTRAINT_VIOLATION: LDAP_RETCODE = 0x13;
pub const LDAP_ATTRIBUTE_OR_VALUE_EXISTS: LDAP_RETCODE = 0x14;
pub const LDAP_INVALID_SYNTAX: LDAP_RETCODE = 0x15;
pub const LDAP_NO_SUCH_OBJECT: LDAP_RETCODE = 0x20;
pub const LDAP_ALIAS_PROBLEM: LDAP_RETCODE = 0x21;
pub const LDAP_INVALID_DN_SYNTAX: LDAP_RETCODE = 0x22;
pub const LDAP_IS_LEAF: LDAP_RETCODE = 0x23;
pub const LDAP_ALIAS_DEREF_PROBLEM: LDAP_RETCODE = 0x24;
pub const LDAP_INAPPROPRIATE_AUTH: LDAP_RETCODE = 0x30;
pub const LDAP_INVALID_CREDENTIALS: LDAP_RETCODE = 0x31;
pub const LDAP_INSUFFICIENT_RIGHTS: LDAP_RETCODE = 0x32;
pub const LDAP_BUSY: LDAP_RETCODE = 0x33;
pub const LDAP_UNAVAILABLE: LDAP_RETCODE = 0x34;
pub const LDAP_UNWILLING_TO_PERFORM: LDAP_RETCODE = 0x35;
pub const LDAP_LOOP_DETECT: LDAP_RETCODE = 0x36;
pub const LDAP_SORT_CONTROL_MISSING: LDAP_RETCODE = 0x3C;
pub const LDAP_OFFSET_RANGE_ERROR: LDAP_RETCODE = 0x3D;
pub const LDAP_NAMING_VIOLATION: LDAP_RETCODE = 0x40;
pub const LDAP_OBJECT_CLASS_VIOLATION: LDAP_RETCODE = 0x41;
pub const LDAP_NOT_ALLOWED_ON_NONLEAF: LDAP_RETCODE = 0x42;
pub const LDAP_NOT_ALLOWED_ON_RDN: LDAP_RETCODE = 0x43;
pub const LDAP_ALREADY_EXISTS: LDAP_RETCODE = 0x44;
pub const LDAP_NO_OBJECT_CLASS_MODS: LDAP_RETCODE = 0x45;
pub const LDAP_RESULTS_TOO_LARGE: LDAP_RETCODE = 0x46;
pub const LDAP_AFFECTS_MULTIPLE_DSAS: LDAP_RETCODE = 0x47;
pub const LDAP_VIRTUAL_LIST_VIEW_ERROR: LDAP_RETCODE = 0x4c;
pub const LDAP_OTHER: LDAP_RETCODE = 0x50;
pub const LDAP_SERVER_DOWN: LDAP_RETCODE = 0x51;
pub const LDAP_LOCAL_ERROR: LDAP_RETCODE = 0x52;
pub const LDAP_ENCODING_ERROR: LDAP_RETCODE = 0x53;
pub const LDAP_DECODING_ERROR: LDAP_RETCODE = 0x54;
pub const LDAP_TIMEOUT: LDAP_RETCODE = 0x55;
pub const LDAP_AUTH_UNKNOWN: LDAP_RETCODE = 0x56;
pub const LDAP_FILTER_ERROR: LDAP_RETCODE = 0x57;
pub const LDAP_USER_CANCELLED: LDAP_RETCODE = 0x58;
pub const LDAP_PARAM_ERROR: LDAP_RETCODE = 0x59;
pub const LDAP_NO_MEMORY: LDAP_RETCODE = 0x5a;
pub const LDAP_CONNECT_ERROR: LDAP_RETCODE = 0x5b;
pub const LDAP_NOT_SUPPORTED: LDAP_RETCODE = 0x5c;
pub const LDAP_NO_RESULTS_RETURNED: LDAP_RETCODE = 0x5e;
pub const LDAP_CONTROL_NOT_FOUND: LDAP_RETCODE = 0x5d;
pub const LDAP_MORE_RESULTS_TO_RETURN: LDAP_RETCODE = 0x5f;
pub const LDAP_CLIENT_LOOP: LDAP_RETCODE = 0x60;
pub const LDAP_REFERRAL_LIMIT_EXCEEDED: LDAP_RETCODE = 0x61;
macro_rules! LDAP_IS_CLDAP {
    ($x:expr) => {
        (*x).ld_sb.sb_naddr > 0
    };
}
macro_rules! NAME_ERROR {
    ($x:expr) => {
        (x & 0xf0) == 0x20
    };
}
FN! {cdecl QUERYFORCONNECTION(
    PrimaryConnection : PLDAP,
    ReferralFromConnection : PLDAP,
    NewDN : PWCHAR,
    HostName : PCHAR,
    PortNumber : ULONG,
    SecAuthIdentity : PVOID,
    CurrentUserToken : PVOID,
    ConnectionToUse : *mut PLDAP,
) -> ULONG}
FN! {cdecl NOTIFYOFNEWCONNECTION(
    PrimaryConnection : PLDAP,
    ReferralFromConnection : PLDAP,
    NewDN : PWCHAR,
    HostName : PCHAR,
    NewConnection : PLDAP,
    PortNumber : ULONG,
    SecAuthIdentity : PVOID,
    CurrentUser : PVOID,
    ErrorCodeFromBind : ULONG,
) -> BOOLEAN}
FN! {cdecl DEREFERENCECONNECTION(
    PrimaryConnection : PLDAP,
    ConnectionToDereference : PLDAP,
) -> ULONG}
FN! {cdecl QUERYCLIENTCERT(
    Connection : PLDAP,
    trusted_CAs : PSecPkgContext_IssuerListInfoEx,
    ppCertificate : *mut PCCERT_CONTEXT,
) -> BOOLEAN}
FN! {cdecl VERIFYSERVERCERT(
    Connection : PLDAP,
    pServerCert : *mut PCCERT_CONTEXT,
) -> BOOLEAN}
STRUCT! {struct LDAP_REFERRAL_CALLBACK {
    SizeOfCallbacks : ULONG,
    QueryForConnection : *mut QUERYFORCONNECTION,
    NotifyRoutine : *mut NOTIFYOFNEWCONNECTION,
    DereferenceRoutine : *mut DEREFERENCECONNECTION,
}}
pub type PLDAP_REFERRAL_CALLBACK = *mut LDAP_REFERRAL_CALLBACK;
STRUCT! {struct LDAP_s {
    sb_sd : UINT_PTR,
    Reserved1 : [UCHAR; (10*4)+1],
    sb_naddr : ULONG_PTR,
    Reserved2 : [UCHAR; (6*4)],
}}
STRUCT! {struct LDAP {
    ld_sb : LDAP_s,
    ld_host : PCHAR,
    ld_version : ULONG,
    ld_lberoptions : UCHAR,
    ld_deref : ULONG,
    ld_timelimit : ULONG,
    ld_sizelimit : ULONG,
    ld_errno : ULONG,
    ld_matched : PCHAR,
    ld_error : PCHAR,
    ld_msgid : ULONG,
    Reserved3 : [UCHAR; (6*4)+1],
    ld_cldaptries : ULONG,
    ld_cldaptimeout : ULONG,
    ld_refhoplimit : ULONG,
    ld_options : ULONG,
}}
pub type PLDAP = *mut LDAP;
STRUCT! {struct LDAP_TIMEVAL {
    tv_sec : LONG,
    tv_usec : LONG,
}}
pub type PLDAP_TIMEVAL = *mut LDAP_TIMEVAL;
STRUCT! {struct LDAP_BERVAL {
    bv_len : ULONG,
    bv_val : PCHAR,
}}
pub type PLDAP_BERVAL = *mut LDAP_BERVAL;
pub type BERVAL = LDAP_BERVAL;
pub type PBERVAL = *mut LDAP_BERVAL;
pub type BerValue = LDAP_BERVAL;
STRUCT! {struct LDAPMessage {
    lm_msgid : ULONG,
    lm_msgtype : ULONG,
    lm_ber : PVOID,
    lm_chain : *mut LDAPMessage,
    lm_next : *mut LDAPMessage,
    lm_time : ULONG,
    Connection : PLDAP,
    Request : PVOID,
    lm_returncode : ULONG,
    lm_referral : USHORT,
    lm_chased : BOOLEAN,
    lm_eom : BOOLEAN,
    ConnectionReferenced : BOOLEAN,
}}
pub type PLDAPMessage = *mut LDAPMessage;
STRUCT! {struct LDAPControlA {
    ldctl_oid : PCHAR,
    ldctl_value : BERVAL,
    ldctl_iscritical : BOOLEAN,
}}
pub type LDAPControl = LDAPControlA;
pub type PLDAPControl = *mut LDAPControlA;
pub type PLDAPControlA = *mut LDAPControlA;
STRUCT! {struct LDAPControlW {
    ldctl_oid : PWCHAR,
    ldctl_value : BERVAL,
    ldctl_iscritical : BOOLEAN,
}}
pub type PLDAPControlW = *mut LDAPControlW;
UNION! {union LDAPModW_u {
    [u32; 1] [u64; 1],
    modv_strvals modv_strvals_mut : *mut PWCHAR,
    modv_bvals modv_bvals_mut : *mut *mut BERVAL,
}}
STRUCT! {struct LDAPModW {
    mod_op : ULONG,
    mod_type : PWCHAR,
    mod_vals : LDAPModW_u,
}}
pub type PLDAPModW = *mut LDAPModW;
UNION! {union LDAPModA_u {
    [u32; 1] [u64; 1],
    modv_strvals modv_strvals_mut : *mut PCHAR,
    modv_bvals modv_bvals_mut : *mut *mut BERVAL,
}}
STRUCT! {struct LDAPModA {
    mod_op : ULONG,
    mod_type : PCHAR,
    mod_vals : LDAPModA_u,
}}
pub type PLDAPModA = *mut LDAPModA;
pub type LDAPMod = LDAPModA;
pub type PLDAPMod = PLDAPModA;
STRUCT! {struct BerElement {
    opaque : PCHAR,
}}
STRUCT! {struct LDAP_VERSION_INFO {
    lv_size : ULONG,
    lv_major : ULONG,
    lv_minor : ULONG,
}}
pub type PLDAP_VERSION_INFO = *mut LDAP_VERSION_INFO;
STRUCT! {struct LDAPAPIInfoA {
    ldapai_info_version : c_int,
    ldapai_api_version : c_int,
    ldapai_protocol_version : c_int,
    ldapai_extensions : *mut *mut c_char,
    ldapai_vendor_name : *mut c_char,
    ldapai_vendor_version : c_int,
}}
pub type LDAPAPIInfo = LDAPAPIInfoA;
STRUCT! {struct LDAPAPIInfoW {
    ldapai_info_version : c_int,
    ldapai_api_version : c_int,
    ldapai_protocol_version : c_int,
    ldapai_extensions : *mut PWCHAR,
    ldapai_vendor_name : PWCHAR,
    ldapai_vendor_version : c_int,
}}
STRUCT! {struct LDAPAPIFeatureInfoA {
    ldapaif_info_version : c_int,
    ldapaif_name : *mut c_char,
    ldapaif_version : c_int,
}}
pub type LDAPAPIFeatureInfo = LDAPAPIFeatureInfoA;
STRUCT! {struct LDAPAPIFeatureInfoW {
    ldapaif_info_version : c_int,
    ldapaif_name : PWCHAR,
    ldapaif_version : c_int,
}}
STRUCT! {struct LDAPSearch {
    x : BYTE,
}}
pub type PLDAPSearch = *mut LDAPSearch;
STRUCT! {struct LDAPSortKeyW {
    sk_attrtype : PWCHAR,
    sk_matchruleoid : PWCHAR,
    sk_reverseorder : BOOLEAN,
}}
pub type PLDAPSortKeyW = *mut LDAPSortKeyW;
STRUCT! {struct LDAPSortKeyA {
    sk_attrtype : PCHAR,
    sk_matchruleoid : PCHAR,
    sk_reverseorder : BOOLEAN,
}}
pub type PLDAPSortKeyA = *mut LDAPSortKeyA;
pub type LDAPSortKey = LDAPSortKeyA;
pub type PLDAPSortKey = PLDAPSortKeyA;
STRUCT! {struct LDAPVLVInfo {
    ldvlv_version : c_int,
    ldvlv_before_count : ULONG,
    ldvlv_after_count : ULONG,
    ldvlv_offset : ULONG,
    ldvlv_count : ULONG,
    ldvlv_attrvalue : PBERVAL,
    ldvlv_context : PBERVAL,
    ldvlv_extradata : PVOID,
}}
pub type PLDAPVLVInfo = *mut LDAPVLVInfo;
extern "cdecl" {
    // FN!{cdecl DBGPRINT(
    // PCCH Format, ...
    // ) -> ULONG}
    // pub fn foobar(x : y, ...) -> ULONG;
    pub fn ldap_openW(
        HostName: PWSTR,
        PortNumber: ULONG
    ) -> *mut LDAP;
    pub fn ldap_openA(
        HostName: PSTR,
        PortNumber: ULONG
    ) -> *mut LDAP;
    pub fn ldap_initW(
        HostName: PWSTR,
        PortNumber: ULONG
    ) -> *mut LDAP;
    pub fn ldap_initA(
        HostName: PSTR,
        PortNumber: ULONG
    ) -> *mut LDAP;
    pub fn ldap_sslinitW(
        HostName: PWSTR,
        PortNumber: ULONG,
        secure: c_int
    ) -> *mut LDAP;
    pub fn ldap_sslinitA(
        HostName: PSTR,
        PortNumber: ULONG,
        secure: c_int
    ) -> *mut LDAP;
    pub fn ldap_connect(ld: *mut LDAP, timeout: *mut LDAP_TIMEVAL) -> ULONG;
    pub fn ldap_open(HostName: PSTR, PortNumber: ULONG) -> *mut LDAP;
    pub fn ldap_init(HostName: PSTR, PortNumber: ULONG) -> *mut LDAP;
    pub fn ldap_sslinit(HostName: PSTR, PortNumber: ULONG, secure: c_int) -> *mut LDAP;
    pub fn cldap_openW(HostName: PWSTR, PortNumber: ULONG) -> *mut LDAP;
    pub fn cldap_openA(HostName: PSTR, PortNumber: ULONG) -> *mut LDAP;
    pub fn cldap_open(HostName: PSTR, PortNumber: ULONG) -> *mut LDAP;
    pub fn ldap_unbind(ld: *mut LDAP) -> ULONG;
    pub fn ldap_unbind_s(ld: *mut LDAP) -> ULONG;
    pub fn ldap_get_option(ld: *mut LDAP, option: c_int, outvalue: PVOID) -> ULONG;
    pub fn ldap_get_optionW(ld: *mut LDAP, option: c_int, outvalue: PVOID) -> ULONG;
    pub fn ldap_set_option(ld: *mut LDAP, option: c_int, invalue: PVOID) -> ULONG;
    pub fn ldap_set_optionW(ld: *mut LDAP, option: c_int, invalue: PVOID) -> ULONG;
    pub fn ldap_simple_bindW(ld: *mut LDAP, dn: PWSTR, passwd: PWSTR) -> ULONG;
    pub fn ldap_simple_bindA(ld: *mut LDAP, dn: PSTR, passwd: PSTR) -> ULONG;
    pub fn ldap_simple_bind_sW(ld: *mut LDAP, dn: PWSTR, passwd: PWSTR) -> ULONG;
    pub fn ldap_simple_bind_sA(ld: *mut LDAP, dn: PSTR, passwd: PSTR) -> ULONG;
    pub fn ldap_bindW(ld: *mut LDAP, dn: PWSTR, cred: PWCHAR, method: ULONG) -> ULONG;
    pub fn ldap_bindA(ld: *mut LDAP, dn: PSTR, cred: PCHAR, method: ULONG) -> ULONG;
    pub fn ldap_bind_sW(ld: *mut LDAP, dn: PWSTR, cred: PWCHAR, method: ULONG) -> ULONG;
    pub fn ldap_bind_sA(
        ld: *mut LDAP,
        dn: PSTR,
        cred: PCHAR,
        method: ULONG
    ) -> ULONG;
    pub fn ldap_sasl_bindA(
        ExternalHandle: *mut LDAP,
        DistName: PSTR,
        AuthMechanism: PSTR,
        cred: *mut BERVAL,
        ServerCtrls: *mut PLDAPControlA,
        ClientCtrls: *mut PLDAPControlA,
        MessageNumber: *mut c_int,
    ) -> INT;
    pub fn ldap_sasl_bindW(
        ExternalHandle: *mut LDAP,
        DistName: PWSTR,
        AuthMechanism: PWSTR,
        cred: *mut BERVAL,
        ServerCtrls: *mut PLDAPControlW,
        ClientCtrls: *mut PLDAPControlW,
        MessageNumber: *mut c_int,
    ) -> INT;
    pub fn ldap_sasl_bind_sA(
        ExternalHandle: *mut LDAP,
        DistName: PSTR,
        AuthMechanism: PSTR,
        cred: *mut BERVAL,
        ServerCtrls: *mut PLDAPControlA,
        ClientCtrls: *mut PLDAPControlA,
        ServerData: *mut PBERVAL,
    ) -> INT;
    pub fn ldap_sasl_bind_sW(
        ExternalHandle: *mut LDAP,
        DistName: PWSTR,
        AuthMechanism: PWSTR,
        cred: *mut BERVAL,
        ServerCtrls: *mut PLDAPControlW,
        ClientCtrls: *mut PLDAPControlW,
        ServerData: *mut PBERVAL,
    ) -> INT;
    pub fn ldap_simple_bind(ld: *mut LDAP, dn: PSTR, passwd: PSTR) -> ULONG;
    pub fn ldap_simple_bind_s(ld: *mut LDAP, dn: PSTR, passwd: PSTR) -> ULONG;
    pub fn ldap_bind(ld: *mut LDAP, dn: PSTR, cred: PCHAR, method: ULONG) -> ULONG;
    pub fn ldap_bind_s(ld: *mut LDAP, dn: PSTR, cred: PCHAR, method: ULONG) -> ULONG;
    pub fn ldap_searchW(
        ld: *mut LDAP,
        base: PWSTR,
        scope: ULONG,
        filter: PWSTR,
        attrs: PZPWSTR,
        attrsonly: ULONG,
    ) -> ULONG;
    pub fn ldap_searchA(
        ld: *mut LDAP,
        base: PSTR,
        scope: ULONG,
        filter: PSTR,
        attrs: PZPSTR,
        attrsonly: ULONG,
    ) -> ULONG;
    pub fn ldap_search_sW(
        ld: *mut LDAP,
        base: PWSTR,
        scope: ULONG,
        filter: PWSTR,
        attrs: PZPWSTR,
        attrsonly: ULONG,
        res: *mut *mut LDAPMessage,
    ) -> ULONG;
    pub fn ldap_search_sA(
        ld: *mut LDAP,
        base: PSTR,
        scope: ULONG,
        filter: PSTR,
        attrs: PZPSTR,
        attrsonly: ULONG,
        res: *mut *mut LDAPMessage,
    ) -> ULONG;
    pub fn ldap_search_stW(
        ld: *mut LDAP,
        base: PWSTR,
        scope: ULONG,
        filter: PWSTR,
        attrs: PZPWSTR,
        attrsonly: ULONG,
        timeout: *mut LDAP_TIMEVAL,
        res: *mut PLDAPMessage,
    ) -> ULONG;
    pub fn ldap_search_stA(
        ld: *mut LDAP,
        base: PSTR,
        scope: ULONG,
        filter: PSTR,
        attrs: PZPSTR,
        attrsonly: ULONG,
        timeout: *mut LDAP_TIMEVAL,
        res: *mut PLDAPMessage,
    ) -> ULONG;
    pub fn ldap_search_extW(
        ld: *mut LDAP,
        base: PWSTR,
        scope: ULONG,
        filter: PWSTR,
        attrs: PZPWSTR,
        attrsonly: ULONG,
        ServerControls: *mut PLDAPControlW,
        ClientControls: *mut PLDAPControlW,
        TimeLimit: ULONG,
        SizeLimit: ULONG,
        MessageNumber: *mut ULONG,
    ) -> ULONG;
    pub fn ldap_search_extA(
        ld: *mut LDAP,
        base: PSTR,
        scope: ULONG,
        filter: PSTR,
        attrs: PZPSTR,
        attrsonly: ULONG,
        ServerControls: *mut PLDAPControlA,
        ClientControls: *mut PLDAPControlA,
        TimeLimit: ULONG,
        SizeLimit: ULONG,
        MessageNumber: *mut ULONG,
    ) -> ULONG;
    pub fn ldap_search_ext_sW(
        ld: *mut LDAP,
        base: PWSTR,
        scope: ULONG,
        filter: PWSTR,
        attrs: PZPWSTR,
        attrsonly: ULONG,
        ServerControls: *mut PLDAPControlW,
        ClientControls: *mut PLDAPControlW,
        timeout: *mut LDAP_TIMEVAL,
        SizeLimit: ULONG,
        res: *mut PLDAPMessage,
    ) -> ULONG;
    pub fn ldap_search_ext_sA(
        ld: *mut LDAP,
        base: PSTR,
        scope: ULONG,
        filter: PSTR,
        attrs: PZPSTR,
        attrsonly: ULONG,
        ServerControls: *mut PLDAPControlA,
        ClientControls: *mut PLDAPControlA,
        timeout: *mut LDAP_TIMEVAL,
        SizeLimit: ULONG,
        res: *mut PLDAPMessage,
    ) -> ULONG;
    pub fn ldap_search(
        ld: *mut LDAP,
        base: PSTR,
        scope: ULONG,
        filter: PSTR,
        attrs: PZPSTR,
        attrsonly: ULONG,
    ) -> ULONG;
    pub fn ldap_search_s(
        ld: *mut LDAP,
        base: PSTR,
        scope: ULONG,
        filter: PSTR,
        attrs: PZPSTR,
        attrsonly: ULONG,
        res: *mut PLDAPMessage,
    ) -> ULONG;
    pub fn ldap_search_st(
        ld: *mut LDAP,
        base: PSTR,
        scope: ULONG,
        filter: PSTR,
        attrs: PZPSTR,
        attrsonly: ULONG,
        timeout: *mut LDAP_TIMEVAL,
        res: *mut PLDAPMessage,
    ) -> ULONG;
    pub fn ldap_search_ext(
        ld: *mut LDAP,
        base: PSTR,
        scope: ULONG,
        filter: PSTR,
        attrs: PZPSTR,
        attrsonly: ULONG,
        ServerControls: *mut PLDAPControlA,
        ClientControls: *mut PLDAPControlA,
        TimeLimit: ULONG,
        SizeLimit: ULONG,
        MessageNumber: *mut ULONG,
    ) -> ULONG;
    pub fn ldap_search_ext_s(
        ld: *mut LDAP,
        base: PSTR,
        scope: ULONG,
        filter: PSTR,
        attrs: PZPSTR,
        attrsonly: ULONG,
        ServerControls: *mut PLDAPControlA,
        ClientControls: *mut PLDAPControlA,
        timeout: *mut LDAP_TIMEVAL,
        SizeLimit: ULONG,
        res: *mut PLDAPMessage,
    ) -> ULONG;
    pub fn ldap_check_filterW(ld: *mut LDAP, SearchFilter: PWSTR) -> ULONG;
    pub fn ldap_check_filterA(ld: *mut LDAP, SearchFilter: PSTR) -> ULONG;
    pub fn ldap_modifyW(ld: *mut LDAP, dn: PWSTR, mods: *mut *mut LDAPModW) -> ULONG;
    pub fn ldap_modifyA(ld: *mut LDAP, dn: PSTR, mods: *mut *mut LDAPModA) -> ULONG;
    pub fn ldap_modify_sW(ld: *mut LDAP, dn: PWSTR, mods: *mut *mut LDAPModW) -> ULONG;
    pub fn ldap_modify_sA(ld: *mut LDAP, dn: PSTR, mods: *mut *mut LDAPModA) -> ULONG;
    pub fn ldap_modify_extW(
        ld: *mut LDAP,
        dn: PWSTR,
        mods: *mut *mut LDAPModW,
        ServerControls: *mut PLDAPControlW,
        ClientControls: *mut PLDAPControlW,
        MessageNumber: *mut ULONG,
    ) -> ULONG;
    pub fn ldap_modify_extA(
        ld: *mut LDAP,
        dn: PSTR,
        mods: *mut *mut LDAPModA,
        ServerControls: *mut PLDAPControlA,
        ClientControls: *mut PLDAPControlA,
        MessageNumber: *mut ULONG,
    ) -> ULONG;
    pub fn ldap_modify_ext_sW(
        ld: *mut LDAP,
        dn: PWSTR,
        mods: *mut *mut LDAPModW,
        ServerControls: *mut PLDAPControlW,
        ClientControls: *mut PLDAPControlW,
    ) -> ULONG;
    pub fn ldap_modify_ext_sA(
        ld: *mut LDAP,
        dn: PSTR,
        mods: *mut *mut LDAPModA,
        ServerControls: *mut PLDAPControlA,
        ClientControls: *mut PLDAPControlA,
    ) -> ULONG;
    pub fn ldap_modify(ld: *mut LDAP, dn: PSTR, mods: *mut *mut LDAPModA) -> ULONG;
    pub fn ldap_modify_s(ld: *mut LDAP, dn: PSTR, mods: *mut *mut LDAPModA) -> ULONG;
    pub fn ldap_modify_ext(
        ld: *mut LDAP,
        dn: PSTR,
        mods: *mut *mut LDAPModA,
        ServerControls: *mut PLDAPControlA,
        ClientControls: *mut PLDAPControlA,
        MessageNumber: *mut ULONG,
    ) -> ULONG;
    pub fn ldap_modify_ext_s(
        ld: *mut LDAP,
        dn: PSTR,
        mods: *mut *mut LDAPModA,
        ServerControls: *mut PLDAPControlA,
        ClientControls: *mut PLDAPControlA,
    ) -> ULONG;
    pub fn ldap_modrdn2W(
        ExternalHandle: *mut LDAP,
        DistinguishedName: PWSTR,
        NewDistinguishedName: PWSTR,
        DeleteOldRdn: INT,
    ) -> ULONG;
    pub fn ldap_modrdn2A(
        ExternalHandle: *mut LDAP,
        DistinguishedName: PSTR,
        NewDistinguishedName: PSTR,
        DeleteOldRdn: INT,
    ) -> ULONG;
    pub fn ldap_modrdnW(
        ExternalHandle: *mut LDAP,
        DistinguishedName: PWSTR,
        NewDistinguishedName: PWSTR,
    ) -> ULONG;
    pub fn ldap_modrdnA(
        ExternalHandle: *mut LDAP,
        DistinguishedName: PSTR,
        NewDistinguishedName: PSTR,
    ) -> ULONG;
    pub fn ldap_modrdn2_sW(
        ExternalHandle: *mut LDAP,
        DistinguishedName: PWSTR,
        NewDistinguishedName: PWSTR,
        DeleteOldRdn: INT,
    ) -> ULONG;
    pub fn ldap_modrdn2_sA(
        ExternalHandle: *mut LDAP,
        DistinguishedName: PSTR,
        NewDistinguishedName: PSTR,
        DeleteOldRdn: INT,
    ) -> ULONG;
    pub fn ldap_modrdn_sW(
        ExternalHandle: *mut LDAP,
        DistinguishedName: PWSTR,
        NewDistinguishedName: PWSTR,
    ) -> ULONG;
    pub fn ldap_modrdn_sA(
        ExternalHandle: *mut LDAP,
        DistinguishedName: PSTR,
        NewDistinguishedName: PSTR,
    ) -> ULONG;
    pub fn ldap_modrdn2(
        ExternalHandle: *mut LDAP,
        DistinguishedName: PSTR,
        NewDistinguishedName: PSTR,
        DeleteOldRdn: INT,
    ) -> ULONG;
    pub fn ldap_modrdn(
        ExternalHandle: *mut LDAP,
        DistinguishedName: PSTR,
        NewDistinguishedName: PSTR,
    ) -> ULONG;
    pub fn ldap_modrdn2_s(
        ExternalHandle: *mut LDAP,
        DistinguishedName: PSTR,
        NewDistinguishedName: PSTR,
        DeleteOldRdn: INT,
    ) -> ULONG;
    pub fn ldap_modrdn_s(
        ExternalHandle: *mut LDAP,
        DistinguishedName: PSTR,
        NewDistinguishedName: PSTR,
    ) -> ULONG;
    pub fn ldap_rename_extW(
        ld: *mut LDAP,
        dn: PWSTR,
        NewRDN: PWSTR,
        NewParent: PWSTR,
        DeleteOldRdn: INT,
        ServerControls: *mut PLDAPControlW,
        ClientControls: *mut PLDAPControlW,
        MessageNumber: *mut ULONG,
    ) -> ULONG;
    pub fn ldap_rename_extA(
        ld: *mut LDAP,
        dn: PSTR,
        NewRDN: PSTR,
        NewParent: PSTR,
        DeleteOldRdn: INT,
        ServerControls: *mut PLDAPControlA,
        ClientControls: *mut PLDAPControlA,
        MessageNumber: *mut ULONG,
    ) -> ULONG;
    pub fn ldap_rename_ext_sW(
        ld: *mut LDAP,
        dn: PWSTR,
        NewRDN: PWSTR,
        NewParent: PWSTR,
        DeleteOldRdn: INT,
        ServerControls: *mut PLDAPControlW,
        ClientControls: *mut PLDAPControlW,
    ) -> ULONG;
    pub fn ldap_rename_ext_sA(
        ld: *mut LDAP,
        dn: PSTR,
        NewRDN: PSTR,
        NewParent: PSTR,
        DeleteOldRdn: INT,
        ServerControls: *mut PLDAPControlA,
        ClientControls: *mut PLDAPControlA,
    ) -> ULONG;
    pub fn ldap_rename_ext(
        ld: *mut LDAP,
        dn: PSTR,
        NewRDN: PSTR,
        NewParent: PSTR,
        DeleteOldRdn: INT,
        ServerControls: *mut PLDAPControlA,
        ClientControls: *mut PLDAPControlA,
        MessageNumber: *mut ULONG,
    ) -> ULONG;
    pub fn ldap_rename_ext_s(
        ld: *mut LDAP,
        dn: PSTR,
        NewRDN: PSTR,
        NewParent: PSTR,
        DeleteOldRdn: INT,
        ServerControls: *mut PLDAPControlA,
        ClientControls: *mut PLDAPControlA,
    ) -> ULONG;
    pub fn ldap_addW(ld: *mut LDAP, dn: PWSTR, attrs: *mut *mut LDAPModW) -> ULONG;
    pub fn ldap_addA(ld: *mut LDAP, dn: PSTR, attrs: *mut *mut LDAPModA) -> ULONG;
    pub fn ldap_add_sW(ld: *mut LDAP, dn: PWSTR, attrs: *mut *mut LDAPModW) -> ULONG;
    pub fn ldap_add_sA(ld: *mut LDAP, dn: PSTR, attrs: *mut *mut LDAPModA) -> ULONG;
    pub fn ldap_add_extW(
        ld: *mut LDAP,
        dn: PWSTR,
        attrs: *mut *mut LDAPModW,
        ServerControls: *mut PLDAPControlW,
        ClientControls: *mut PLDAPControlW,
        MessageNumber: *mut ULONG,
    ) -> ULONG;
    pub fn ldap_add_extA(
        ld: *mut LDAP,
        dn: PSTR,
        attrs: *mut *mut LDAPModA,
        ServerControls: *mut PLDAPControlA,
        ClientControls: *mut PLDAPControlA,
        MessageNumber: *mut ULONG,
    ) -> ULONG;
    pub fn ldap_add_ext_sW(
        ld: *mut LDAP,
        dn: PWSTR,
        attrs: *mut *mut LDAPModW,
        ServerControls: *mut PLDAPControlW,
        ClientControls: *mut PLDAPControlW,
    ) -> ULONG;
    pub fn ldap_add_ext_sA(
        ld: *mut LDAP,
        dn: PSTR,
        attrs: *mut *mut LDAPModA,
        ServerControls: *mut PLDAPControlA,
        ClientControls: *mut PLDAPControlA,
    ) -> ULONG;
    pub fn ldap_add(ld: *mut LDAP, dn: PSTR, attrs: *mut *mut LDAPMod) -> ULONG;
    pub fn ldap_add_s(ld: *mut LDAP, dn: PSTR, attrs: *mut *mut LDAPMod) -> ULONG;
    pub fn ldap_add_ext(
        ld: *mut LDAP,
        dn: PSTR,
        attrs: *mut *mut LDAPModA,
        ServerControls: *mut PLDAPControlA,
        ClientControls: *mut PLDAPControlA,
        MessageNumber: *mut ULONG,
    ) -> ULONG;
    pub fn ldap_add_ext_s(
        ld: *mut LDAP,
        dn: PSTR,
        attrs: *mut *mut LDAPModA,
        ServerControls: *mut PLDAPControlA,
        ClientControls: *mut PLDAPControlA,
    ) -> ULONG;
    pub fn ldap_compareW(ld: *mut LDAP, dn: PWSTR, attr: PWSTR, value: PWSTR) -> ULONG;
    pub fn ldap_compareA(ld: *mut LDAP, dn: PSTR, attr: PSTR, value: PSTR) -> ULONG;
    pub fn ldap_compare_sW(ld: *mut LDAP, dn: PWSTR, attr: PWSTR, value: PWSTR) -> ULONG;
    pub fn ldap_compare_sA(ld: *mut LDAP, dn: PSTR, attr: PSTR, value: PSTR) -> ULONG;
    pub fn ldap_compare(ld: *mut LDAP, dn: PSTR, attr: PSTR, value: PSTR) -> ULONG;
    pub fn ldap_compare_s(ld: *mut LDAP, dn: PSTR, attr: PSTR, value: PSTR) -> ULONG;
    pub fn ldap_compare_extW(
        ld: *mut LDAP,
        dn: PWSTR,
        Attr: PWSTR,
        Value: PWSTR,
        Data: *mut BERVAL,
        ServerControls: *mut PLDAPControlW,
        ClientControls: *mut PLDAPControlW,
        MessageNumber: *mut ULONG,
    ) -> ULONG;
    pub fn ldap_compare_extA(
        ld: *mut LDAP,
        dn: PSTR,
        Attr: PSTR,
        Value: PSTR,
        Data: *mut BERVAL,
        ServerControls: *mut PLDAPControlA,
        ClientControls: *mut PLDAPControlA,
        MessageNumber: *mut ULONG,
    ) -> ULONG;
    pub fn ldap_compare_ext_sW(
        ld: *mut LDAP,
        dn: PWSTR,
        Attr: PWSTR,
        Value: PWSTR,
        Data: *mut BERVAL,
        ServerControls: *mut PLDAPControlW,
        ClientControls: *mut PLDAPControlW,
    ) -> ULONG;
    pub fn ldap_compare_ext_sA(
        ld: *mut LDAP,
        dn: PSTR,
        Attr: PSTR,
        Value: PSTR,
        Data: *mut BERVAL,
        ServerControls: *mut PLDAPControlA,
        ClientControls: *mut PLDAPControlA,
    ) -> ULONG;
    pub fn ldap_compare_ext(
        ld: *mut LDAP,
        dn: PSTR,
        Attr: PSTR,
        Value: PSTR,
        Data: *mut BERVAL,
        ServerControls: *mut PLDAPControlA,
        ClientControls: *mut PLDAPControlA,
        MessageNumber: *mut ULONG,
    ) -> ULONG;
    pub fn ldap_compare_ext_s(
        ld: *mut LDAP,
        dn: PSTR,
        Attr: PSTR,
        Value: PSTR,
        Data: *mut BERVAL,
        ServerControls: *mut PLDAPControlA,
        ClientControls: *mut PLDAPControlA,
    ) -> ULONG;
    pub fn ldap_deleteW(ld: *mut LDAP, dn: PWSTR) -> ULONG;
    pub fn ldap_deleteA(ld: *mut LDAP, dn: PSTR) -> ULONG;
    pub fn ldap_delete_sW(ld: *mut LDAP, dn: PWSTR) -> ULONG;
    pub fn ldap_delete_sA(ld: *mut LDAP, dn: PSTR) -> ULONG;
    pub fn ldap_delete_extW(
        ld: *mut LDAP,
        dn: PWSTR,
        ServerControls: *mut PLDAPControlW,
        ClientControls: *mut PLDAPControlW,
        MessageNumber: *mut ULONG,
    ) -> ULONG;
    pub fn ldap_delete_extA(
        ld: *mut LDAP,
        dn: PSTR,
        ServerControls: *mut PLDAPControlA,
        ClientControls: *mut PLDAPControlA,
        MessageNumber: *mut ULONG,
    ) -> ULONG;
    pub fn ldap_delete_ext_sW(
        ld: *mut LDAP,
        dn: PWSTR,
        ServerControls: *mut PLDAPControlW,
        ClientControls: *mut PLDAPControlW,
    ) -> ULONG;
    pub fn ldap_delete_ext_sA(
        ld: *mut LDAP,
        dn: PSTR,
        ServerControls: *mut PLDAPControlA,
        ClientControls: *mut PLDAPControlA,
    ) -> ULONG;
    pub fn ldap_delete(ld: *mut LDAP, dn: PSTR) -> ULONG;
    pub fn ldap_delete_s(ld: *mut LDAP, dn: PSTR) -> ULONG;
    pub fn ldap_delete_ext(
        ld: *mut LDAP,
        dn: PSTR,
        ServerControls: *mut PLDAPControlA,
        ClientControls: *mut PLDAPControlA,
        MessageNumber: *mut ULONG,
    ) -> ULONG;
    pub fn ldap_delete_ext_s(
        ld: *mut LDAP,
        dn: PSTR,
        ServerControls: *mut PLDAPControlA,
        ClientControls: *mut PLDAPControlA,
    ) -> ULONG;
    pub fn ldap_abandon(ld: *mut LDAP, msgid: ULONG) -> ULONG;
    pub fn ldap_result(
        ld: *mut LDAP,
        msgid: ULONG,
        all: ULONG,
        timeout: PLDAP_TIMEVAL,
        res: *mut PLDAPMessage,
    ) -> ULONG;
    pub fn ldap_msgfree(res: *mut LDAPMessage) -> ULONG;
    pub fn ldap_result2error(ld: *mut LDAP, res: *mut LDAPMessage, freeit: ULONG) -> ULONG;
    pub fn ldap_parse_resultW(
        Connection: *mut LDAP,
        ResultMessage: *mut LDAPMessage,
        ReturnCode: *mut ULONG,
        MatchedDNs: *mut PWSTR,
        ErrorMessage: *mut PWSTR,
        Referrals: *mut PZPWSTR,
        ServerControls: *mut *mut PLDAPControlW,
        Freeit: BOOLEAN,
    ) -> ULONG;
    pub fn ldap_parse_resultA(
        Connection: *mut LDAP,
        ResultMessage: *mut LDAPMessage,
        ReturnCode: *mut ULONG,
        MatchedDNs: *mut PSTR,
        ErrorMessage: *mut PSTR,
        Referrals: *mut PZPSTR,
        ServerControls: *mut *mut PLDAPControlA,
        Freeit: BOOLEAN,
    ) -> ULONG;
    pub fn ldap_parse_extended_resultA(
        Connection: *mut LDAP,
        ResultMessage: *mut LDAPMessage,
        ResultOID: *mut PSTR,
        ResultData: *mut *mut BerValue,
        Freeit: BOOLEAN,
    ) -> ULONG;
    pub fn ldap_parse_extended_resultW(
        Connection: *mut LDAP,
        ResultMessage: *mut LDAPMessage,
        ResultOID: *mut PWSTR,
        ResultData: *mut *mut BerValue,
        Freeit: BOOLEAN,
    ) -> ULONG;
    pub fn ldap_controls_freeA(Controls: *mut *mut LDAPControlA) -> ULONG;
    pub fn ldap_control_freeA(Controls: *mut LDAPControlA) -> ULONG;
    pub fn ldap_controls_freeW(Control: *mut *mut LDAPControlW) -> ULONG;
    pub fn ldap_control_freeW(Control: *mut LDAPControlW) -> ULONG;
    pub fn ldap_free_controlsW(Controls: *mut *mut LDAPControlW) -> ULONG;
    pub fn ldap_free_controlsA(Controls: *mut *mut LDAPControlA) -> ULONG;
    pub fn ldap_parse_result(
        Connection: *mut LDAP,
        ResultMessage: *mut LDAPMessage,
        ReturnCodeOPTIONAL: *mut ULONG,
        MatchedDNsOPTIONAL: *mut PSTR,
        ErrorMessageOPTIONAL: *mut PSTR,
        ReferralsOPTIONAL: *mut *mut PSTR,
        ServerControlsOPTIONAL: *mut *mut PLDAPControlA,
        Freeit: BOOLEAN,
    ) -> ULONG;
    pub fn ldap_controls_free(Controls: *mut *mut LDAPControlA) -> ULONG;
    pub fn ldap_control_free(Control: *mut LDAPControlA) -> ULONG;
    pub fn ldap_free_controls(Controls: *mut *mut LDAPControlA) -> ULONG;
    pub fn ldap_err2stringW(err: ULONG) -> PWCHAR;
    pub fn ldap_err2stringA(err: ULONG) -> PCHAR;
    pub fn ldap_err2string(err: ULONG) -> PCHAR;
    pub fn ldap_perror(ld: *mut LDAP, msg: PCHAR) -> ();
    pub fn ldap_first_entry(ld: *mut LDAP, res: *mut LDAPMessage) -> *mut LDAPMessage;
    pub fn ldap_next_entry(ld: *mut LDAP, entry: *mut LDAPMessage) -> *mut LDAPMessage;
    pub fn ldap_count_entries(ld: *mut LDAP, res: *mut LDAPMessage) -> ULONG;
    pub fn ldap_first_attributeW(
        ld: *mut LDAP,
        entry: *mut LDAPMessage,
        ptr: *mut *mut BerElement,
    ) -> PWCHAR;
    pub fn ldap_first_attributeA(
        ld: *mut LDAP,
        entry: *mut LDAPMessage,
        ptr: *mut *mut BerElement,
    ) -> PCHAR;
    pub fn ldap_first_attribute(
        ld: *mut LDAP,
        entry: *mut LDAPMessage,
        ptr: *mut *mut BerElement,
    ) -> PCHAR;
    pub fn ldap_next_attributeW(
        ld: *mut LDAP,
        entry: *mut LDAPMessage,
        ptr: *mut BerElement,
    ) -> PWCHAR;
    pub fn ldap_next_attributeA(
        ld: *mut LDAP,
        entry: *mut LDAPMessage,
        ptr: *mut BerElement,
    ) -> PCHAR;
    pub fn ldap_next_attribute(
        ld: *mut LDAP,
        entry: *mut LDAPMessage,
        ptr: *mut BerElement,
    ) -> PCHAR;
    pub fn ldap_get_valuesW(ld: *mut LDAP, entry: *mut LDAPMessage, attr: PWSTR) -> *mut PWCHAR;
    pub fn ldap_get_valuesA(ld: *mut LDAP, entry: *mut LDAPMessage, attr: PSTR) -> *mut PCHAR;
    pub fn ldap_get_values(ld: *mut LDAP, entry: *mut LDAPMessage, attr: PSTR) -> *mut PCHAR;
    pub fn ldap_get_values_lenW(
        ExternalHandle: *mut LDAP,
        Message: *mut LDAPMessage,
        attr: PWSTR,
    ) -> *mut *mut BerValue;
    pub fn ldap_get_values_lenA(
        ExternalHandle: *mut LDAP,
        Message: *mut LDAPMessage,
        attr: PSTR,
    ) -> *mut *mut BerValue;
    pub fn ldap_get_values_len(
        ExternalHandle: *mut LDAP,
        Message: *mut LDAPMessage,
        attr: PSTR,
    ) -> *mut *mut BerValue;
    pub fn ldap_count_valuesW(vals: *mut PWCHAR) -> ULONG;
    pub fn ldap_count_valuesA(vals: *mut PCHAR) -> ULONG;
    pub fn ldap_count_values(vals: *mut PCHAR) -> ULONG;
    pub fn ldap_count_values_len(vals: *mut *mut BerValue) -> ULONG;
    pub fn ldap_value_freeW(vals: *mut PWCHAR) -> ULONG;
    pub fn ldap_value_freeA(vals: *mut PCHAR) -> ULONG;
    pub fn ldap_value_free(vals: *mut PCHAR) -> ULONG;
    pub fn ldap_value_free_len(vals: *mut *mut BerValue) -> ULONG;
    pub fn ldap_get_dnW(ld: *mut LDAP, entry: *mut LDAPMessage) -> PWCHAR;
    pub fn ldap_get_dnA(ld: *mut LDAP, entry: *mut LDAPMessage) -> PCHAR;
    pub fn ldap_get_dn(ld: *mut LDAP, entry: *mut LDAPMessage) -> PCHAR;
    pub fn ldap_explode_dnW(dn: PWSTR, notypes: ULONG) -> *mut PWCHAR;
    pub fn ldap_explode_dnA(dn: PSTR, notypes: ULONG) -> *mut PCHAR;
    pub fn ldap_explode_dn(dn: PSTR, notypes: ULONG) -> *mut PCHAR;
    pub fn ldap_dn2ufnW(dn: PWSTR) -> PWCHAR;
    pub fn ldap_dn2ufnA(dn: PSTR) -> PCHAR;
    pub fn ldap_dn2ufn(dn: PSTR) -> PCHAR;
    pub fn ldap_memfreeW(Block: PWCHAR) -> ();
    pub fn ldap_memfreeA(Block: PCHAR) -> ();
    pub fn ber_bvfree(bv: *mut BERVAL) -> ();
    pub fn ldap_memfree(Block: PCHAR) -> ();
    pub fn ldap_ufn2dnW(ufn: PWSTR, pDn: *mut PWSTR) -> ULONG;
    pub fn ldap_ufn2dnA(ufn: PSTR, pDn: *mut PSTR) -> ULONG;
    pub fn ldap_ufn2dn(ufn: PSTR, pDn: *mut PSTR) -> ULONG;
    pub fn ldap_cleanup(hInstance: HANDLE) -> ULONG;
    pub fn ldap_escape_filter_elementW(
        sourceFilterElement: PCHAR,
        sourceLength: ULONG,
        destFilterElement: PWCHAR,
        destLength: ULONG,
    ) -> ULONG;
    pub fn ldap_escape_filter_elementA(
        sourceFilterElement: PCHAR,
        sourceLength: ULONG,
        destFilterElement: PCHAR,
        destLength: ULONG,
    ) -> ULONG;
    pub fn ldap_escape_filter_element(
        sourceFilterElement: PCHAR,
        sourceLength: ULONG,
        destFilterElement: PCHAR,
        destLength: ULONG,
    ) -> ULONG;
    pub fn ldap_set_dbg_flags(NewFlags: ULONG) -> ULONG;
    // pub fn ldap_set_dbg_routine(DBGPRINT DebugPrintRoutine) -> ();
    pub fn LdapUTF8ToUnicode(
        lpSrcStr: LPCSTR,
        cchSrc: c_int,
        lpDestStr: LPWSTR,
        cchDest: c_int,
    ) -> c_int;
    pub fn LdapUnicodeToUTF8(
        lpSrcStr: LPCWSTR,
        cchSrc: c_int,
        lpDestStr: LPSTR,
        cchDest: c_int,
    ) -> c_int;
    pub fn ldap_create_sort_controlA(
        ExternalHandle: PLDAP,
        SortKeys: *mut PLDAPSortKeyA,
        IsCritical: UCHAR,
        Control: *mut PLDAPControlA,
    ) -> ULONG;
    pub fn ldap_create_sort_controlW(
        ExternalHandle: PLDAP,
        SortKeys: *mut PLDAPSortKeyW,
        IsCritical: UCHAR,
        Control: *mut PLDAPControlW,
    ) -> ULONG;
    pub fn ldap_parse_sort_controlA(
        ExternalHandle: PLDAP,
        Control: *mut PLDAPControlA,
        Result: *mut ULONG,
        Attribute: *mut PCHAR,
    ) -> ULONG;
    pub fn ldap_parse_sort_controlW(
        ExternalHandle: PLDAP,
        Control: *mut PLDAPControlW,
        Result: *mut ULONG,
        Attribute: *mut PWCHAR,
    ) -> ULONG;
    pub fn ldap_create_sort_control(
        ExternalHandle: PLDAP,
        SortKeys: *mut PLDAPSortKeyA,
        IsCritical: UCHAR,
        Control: *mut PLDAPControlA,
    ) -> ULONG;
    pub fn ldap_parse_sort_control(
        ExternalHandle: PLDAP,
        Control: *mut PLDAPControlA,
        Result: *mut ULONG,
        Attribute: *mut PCHAR,
    ) -> ULONG;
    pub fn ldap_encode_sort_controlW(
        ExternalHandle: PLDAP,
        SortKeys: *mut PLDAPSortKeyW,
        Control: PLDAPControlW,
        Criticality: BOOLEAN,
    ) -> ULONG;
    pub fn ldap_encode_sort_controlA(
        ExternalHandle: PLDAP,
        SortKeys: *mut PLDAPSortKeyA,
        Control: PLDAPControlA,
        Criticality: BOOLEAN,
    ) -> ULONG;
    pub fn ldap_encode_sort_control(
        ExternalHandle: PLDAP,
        SortKeys: *mut PLDAPSortKeyA,
        Control: PLDAPControlA,
        Criticality: BOOLEAN,
    ) -> ULONG;
    pub fn ldap_create_page_controlW(
        ExternalHandle: PLDAP,
        PageSize: ULONG,
        Cookie: *mut BERVAL,
        IsCritical: UCHAR,
        Control: *mut PLDAPControlW,
    ) -> ULONG;
    pub fn ldap_create_page_controlA(
        ExternalHandle: PLDAP,
        PageSize: ULONG,
        Cookie: *mut BERVAL,
        IsCritical: UCHAR,
        Control: *mut PLDAPControlA,
    ) -> ULONG;
    pub fn ldap_parse_page_controlW(
        ExternalHandle: PLDAP,
        ServerControls: *mut PLDAPControlW,
        TotalCount: *mut ULONG,
        Cookie: *mut *mut BERVAL,
    ) -> ULONG;
    pub fn ldap_parse_page_controlA(
        ExternalHandle: PLDAP,
        ServerControls: *mut PLDAPControlA,
        TotalCount: *mut ULONG,
        Cookie: *mut *mut BERVAL,
    ) -> ULONG;
    pub fn ldap_create_page_control(
        ExternalHandle: PLDAP,
        PageSize: ULONG,
        Cookie: *mut BERVAL,
        IsCritical: UCHAR,
        Control: *mut PLDAPControlA,
    ) -> ULONG;
    pub fn ldap_parse_page_control(
        ExternalHandle: PLDAP,
        ServerControls: *mut PLDAPControlA,
        TotalCount: *mut ULONG,
        Cookie: *mut *mut BERVAL,
    ) -> ULONG;
    pub fn ldap_search_init_pageW(
        ExternalHandle: PLDAP,
        DistinguishedName: PWSTR,
        ScopeOfSearch: ULONG,
        SearchFilter: PWSTR,
        AttributeList: PZPWSTR,
        AttributesOnly: ULONG,
        ServerControls: *mut PLDAPControlW,
        ClientControls: *mut PLDAPControlW,
        PageTimeLimit: ULONG,
        TotalSizeLimit: ULONG,
        SortKeys: *mut PLDAPSortKeyW,
    ) -> PLDAPSearch;
    pub fn ldap_search_init_pageA(
        ExternalHandle: PLDAP,
        DistinguishedName: PSTR,
        ScopeOfSearch: ULONG,
        SearchFilter: PSTR,
        AttributeList: PZPSTR,
        AttributesOnly: ULONG,
        ServerControls: *mut PLDAPControlA,
        ClientControls: *mut PLDAPControlA,
        PageTimeLimit: ULONG,
        TotalSizeLimit: ULONG,
        SortKeys: *mut PLDAPSortKeyA,
    ) -> PLDAPSearch;
    pub fn ldap_search_init_page(
        ExternalHandle: PLDAP,
        DistinguishedName: PSTR,
        ScopeOfSearch: ULONG,
        SearchFilter: PSTR,
        AttributeList: PZPSTR,
        AttributesOnly: ULONG,
        ServerControls: *mut PLDAPControl,
        ClientControls: *mut PLDAPControl,
        PageTimeLimit: ULONG,
        TotalSizeLimit: ULONG,
        SortKeys: *mut PLDAPSortKey,
    ) -> PLDAPSearch;
    pub fn ldap_get_next_page(
        ExternalHandle: PLDAP,
        SearchHandle: PLDAPSearch,
        PageSize: ULONG,
        MessageNumber: *mut ULONG,
    ) -> ULONG;
    pub fn ldap_get_next_page_s(
        ExternalHandle: PLDAP,
        SearchHandle: PLDAPSearch,
        timeout: *mut LDAP_TIMEVAL,
        PageSize: ULONG,
        TotalCount: *mut ULONG,
        Results: *mut *mut LDAPMessage,
    ) -> ULONG;
    pub fn ldap_get_paged_count(
        ExternalHandle: PLDAP,
        SearchBlock: PLDAPSearch,
        TotalCount: *mut ULONG,
        Results: PLDAPMessage,
    ) -> ULONG;
    pub fn ldap_search_abandon_page(ExternalHandle: PLDAP, SearchBlock: PLDAPSearch) -> ULONG;
    pub fn ldap_create_vlv_controlW(
        ExternalHandle: PLDAP,
        VlvInfo: PLDAPVLVInfo,
        IsCritical: UCHAR,
        Control: *mut PLDAPControlW,
    ) -> INT;
    pub fn ldap_create_vlv_controlA(
        ExternalHandle: PLDAP,
        VlvInfo: PLDAPVLVInfo,
        IsCritical: UCHAR,
        Control: *mut PLDAPControlA,
    ) -> INT;
    pub fn ldap_parse_vlv_controlW(
        ExternalHandle: PLDAP,
        Control: *mut PLDAPControlW,
        TargetPos: PULONG,
        ListCount: PULONG,
        Context: *mut PBERVAL,
        ErrCode: PINT,
    ) -> INT;
    pub fn ldap_parse_vlv_controlA(
        ExternalHandle: PLDAP,
        Control: *mut PLDAPControlA,
        TargetPos: PULONG,
        ListCount: PULONG,
        Context: *mut PBERVAL,
        ErrCode: PINT,
    ) -> INT;
    pub fn ldap_start_tls_sW(
        ExternalHandle: PLDAP,
        ServerReturnValue: PULONG,
        result: *mut *mut LDAPMessage,
        ServerControls: *mut PLDAPControlW,
        ClientControls: *mut PLDAPControlW,
    ) -> ULONG;
    pub fn ldap_start_tls_sA(
        ExternalHandle: PLDAP,
        ServerReturnValue: PULONG,
        result: *mut *mut LDAPMessage,
        ServerControls: *mut PLDAPControlA,
        ClientControls: *mut PLDAPControlA,
    ) -> ULONG;
    pub fn ldap_stop_tls_s(ExternalHandle: PLDAP) -> BOOLEAN;
    pub fn ldap_first_reference(ld: *mut LDAP, res: *mut LDAPMessage) -> *mut LDAPMessage;
    pub fn ldap_next_reference(ld: *mut LDAP, entry: *mut LDAPMessage) -> *mut LDAPMessage;
    pub fn ldap_count_references(ld: *mut LDAP, res: *mut LDAPMessage) -> ULONG;
    pub fn ldap_parse_referenceW(
        Connection: *mut LDAP,
        ResultMessage: *mut LDAPMessage,
        Referrals: *mut *mut PWCHAR,
    ) -> ULONG;
    pub fn ldap_parse_referenceA(
        Connection: *mut LDAP,
        ResultMessage: *mut LDAPMessage,
        Referrals: *mut *mut PCHAR,
    ) -> ULONG;
    pub fn ldap_parse_reference(
        Connection: *mut LDAP,
        ResultMessage: *mut LDAPMessage,
        Referrals: *mut *mut PCHAR,
    ) -> ULONG;
    pub fn ldap_extended_operationW(
        ld: *mut LDAP,
        Oid: PWSTR,
        Data: *mut BERVAL,
        ServerControls: *mut PLDAPControlW,
        ClientControls: *mut PLDAPControlW,
        MessageNumber: *mut ULONG,
    ) -> ULONG;
    pub fn ldap_extended_operationA(
        ld: *mut LDAP,
        Oid: PSTR,
        Data: *mut BERVAL,
        ServerControls: *mut PLDAPControlA,
        ClientControls: *mut PLDAPControlA,
        MessageNumber: *mut ULONG,
    ) -> ULONG;
    pub fn ldap_extended_operation_sA(
        ExternalHandle: *mut LDAP,
        Oid: PSTR,
        Data: *mut BERVAL,
        ServerControls: *mut PLDAPControlA,
        ClientControls: *mut PLDAPControlA,
        ReturnedOid: *mut PCHAR,
        ReturnedData: *mut *mut BERVAL,
    ) -> ULONG;
    pub fn ldap_extended_operation_sW(
        ExternalHandle: *mut LDAP,
        Oid: PWSTR,
        Data: *mut BERVAL,
        ServerControls: *mut PLDAPControlW,
        ClientControls: *mut PLDAPControlW,
        ReturnedOid: *mut PWCHAR,
        ReturnedData: *mut *mut BERVAL,
    ) -> ULONG;
    pub fn ldap_extended_operation(
        ld: *mut LDAP,
        Oid: PSTR,
        Data: *mut BERVAL,
        ServerControls: *mut PLDAPControlA,
        ClientControls: *mut PLDAPControlA,
        MessageNumber: *mut ULONG,
    ) -> ULONG;
    pub fn ldap_close_extended_op(ld: *mut LDAP, MessageNumber: ULONG) -> ULONG;
    pub fn LdapGetLastError() -> ULONG;
    pub fn LdapMapErrorToWin32(LdapError: ULONG) -> ULONG;
    pub fn ldap_conn_from_msg(PrimaryConn: *mut LDAP, res: *mut LDAPMessage) -> *mut LDAP;
    pub fn ldap_startup(version: PLDAP_VERSION_INFO, Instance: *mut HANDLE) -> ULONG;
}
pub const ldap_sasl_bind: unsafe extern "cdecl" fn(
    *mut LDAP,
    PSTR,
    PSTR,
    *mut BERVAL,
    *mut PLDAPControlA,
    *mut PLDAPControlA,
    *mut c_int,
) -> INT = ldap_sasl_bindA;
pub const ldap_sasl_bind_s: unsafe extern "cdecl" fn(
    *mut LDAP,
    PSTR,
    PSTR,
    *mut BERVAL,
    *mut PLDAPControlA,
    *mut PLDAPControlA,
    *mut *mut BERVAL,
) -> INT = ldap_sasl_bind_sA;
pub const ldap_check_filter: unsafe extern "cdecl" fn(*mut LDAP, PSTR) -> ULONG = ldap_check_filterA;
pub const ldap_rename: unsafe extern "cdecl" fn(
    *mut LDAP,
    PSTR,
    PSTR,
    PSTR,
    INT,
    *mut PLDAPControlA,
    *mut PLDAPControlA,
    *mut ULONG,
) -> ULONG = ldap_rename_extA;
pub const ldap_rename_s: unsafe extern "cdecl" fn(
    *mut LDAP,
    PSTR,
    PSTR,
    PSTR,
    INT,
    *mut PLDAPControlA,
    *mut PLDAPControlA,
) -> ULONG = ldap_rename_ext_sA;
pub const ldap_parse_extended_result: unsafe extern "cdecl" fn(
    *mut LDAP,
    *mut LDAPMessage,
    *mut PSTR,
    *mut *mut BerValue,
    BOOLEAN,
) -> ULONG = ldap_parse_extended_resultA;
pub const ldap_create_vlv_control: unsafe extern "cdecl" fn(
    PLDAP,
    PLDAPVLVInfo,
    UCHAR,
    *mut PLDAPControlA,
) -> INT = ldap_create_vlv_controlA;
pub const ldap_parse_vlv_control: unsafe extern "cdecl" fn(
    PLDAP,
    *mut PLDAPControlA,
    PULONG,
    PULONG,
    *mut PBERVAL,
    PINT,
) -> INT = ldap_parse_vlv_controlA;
pub const ldap_start_tls_s: unsafe extern "cdecl" fn(
    PLDAP,
    PULONG,
    *mut *mut LDAPMessage,
    *mut PLDAPControlA,
    *mut PLDAPControlA,
) -> ULONG = ldap_start_tls_sA;
pub const ldap_extended_operation_s: unsafe extern "cdecl" fn(
    *mut LDAP,
    PSTR,
    *mut BERVAL,
    *mut PLDAPControlA,
    *mut PLDAPControlA,
    *mut PCHAR,
    *mut *mut BERVAL,
) -> ULONG = ldap_extended_operation_sA;

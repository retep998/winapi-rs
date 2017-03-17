// Copyright © 2015-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Microsoft SIP Provider Prototypes and Definitions

use shared::guiddef::GUID;
use shared::minwindef::{BOOL, BYTE, DWORD, LPVOID};
use um::mscat::{CRYPTCATMEMBER, CRYPTCATSTORE};
use um::wincrypt::{
    CRYPT_ALGORITHM_IDENTIFIER, CRYPT_ATTRIBUTE_TYPE_VALUE, CRYPT_HASH_BLOB, HCRYPTPROV,
};
use um::winnt::{HANDLE, LPCWSTR, PWSTR, WCHAR};

STRUCT!{struct SIP_SUBJECTINFO {
    cbSize: DWORD,
    pgSubjectType: *mut GUID,
    hFile: HANDLE,
    pwsFileName: LPCWSTR,
    pwsDisplayName: LPCWSTR,
    dwReserved1: DWORD,
    dwIntVersion: DWORD,
    hProv: HCRYPTPROV,
    DigestAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    dwFlags: DWORD,
    dwEncodingType: DWORD,
    dwReserved2: DWORD,
    fdwCAPISettings: DWORD,
    fdwSecuritySettings: DWORD,
    dwIndex: DWORD,
    dwUnionChoice: DWORD,
    psFlat: *mut MS_ADDINFO_FLAT,
    pClientData: LPVOID,
}}
UNION!(SIP_SUBJECTINFO, psFlat, psCatMember, psCatMember_mut, *mut MS_ADDINFO_CATALOGMEMBER);
UNION!(SIP_SUBJECTINFO, psFlat, psBlob, psBlob_mut, *mut MS_ADDINFO_BLOB);
pub type LPSIP_SUBJECTINFO = *mut SIP_SUBJECTINFO;
STRUCT!{struct MS_ADDINFO_FLAT {
    cbStruct: DWORD,
    pIndirectData: *mut SIP_INDIRECT_DATA,
}}
pub type PMS_ADDINFO_FLAT = *mut MS_ADDINFO_FLAT;
STRUCT!{struct MS_ADDINFO_CATALOGMEMBER {
    cbStruct: DWORD,
    pStore: *mut CRYPTCATSTORE,
    pMember: *mut CRYPTCATMEMBER,
}}
pub type PMS_ADDINFO_CATALOGMEMBER = *mut MS_ADDINFO_CATALOGMEMBER;
STRUCT!{struct MS_ADDINFO_BLOB {
    cbStruct: DWORD,
    cbMemObject: DWORD,
    pbMemObject: *mut BYTE,
    cbMemSignedMsg: DWORD,
    pbMemSignedMsg: *mut BYTE,
}}
pub type PMS_ADDINFO_BLOB = *mut MS_ADDINFO_BLOB;
STRUCT!{struct SIP_INDIRECT_DATA {
    Data: CRYPT_ATTRIBUTE_TYPE_VALUE,
    DigestAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    Digest: CRYPT_HASH_BLOB,
}}
pub type PSIP_INDIRECT_DATA = *mut SIP_INDIRECT_DATA;
STRUCT!{struct SIP_ADD_NEWPROVIDER {
    cbStruct: DWORD,
    pgSubject: *mut GUID,
    pwszDLLFileName: *mut WCHAR,
    pwszMagicNumber: *mut WCHAR,
    pwszIsFunctionName: *mut WCHAR,
    pwszGetFuncName: *mut WCHAR,
    pwszPutFuncName: *mut WCHAR,
    pwszCreateFuncName: *mut WCHAR,
    pwszVerifyFuncName: *mut WCHAR,
    pwszRemoveFuncName: *mut WCHAR,
    pwszIsFunctionNameFmt2: *mut WCHAR,
    pwszGetCapFuncName: PWSTR,
}}
pub type PSIP_ADD_NEWPROVIDER = *mut SIP_ADD_NEWPROVIDER;
STRUCT!{struct SIP_CAP_SET_V3 {
    cbSize: DWORD,
    dwVersion: DWORD,
    isMultiSign: BOOL,
    dwFlags: DWORD,
}}
UNION!(SIP_CAP_SET_V3, dwFlags, dwReserved, dwReserved_mut, DWORD);
pub type PSIP_CAP_SET_V3 = *mut SIP_CAP_SET_V3;
pub type SIP_CAP_SET = PSIP_CAP_SET_V3;
pub type pCryptSIPGetSignedDataMsg = Option<unsafe extern "system" fn(
    pSubjectInfo: *mut SIP_SUBJECTINFO, pdwEncodingType: *mut DWORD, dwIndex: DWORD,
    pcbSignedDataMsg: *mut DWORD, pbSignedDataMsg: *mut BYTE,
) -> BOOL>;
pub type pCryptSIPPutSignedDataMsg = Option<unsafe extern "system" fn(
    pSubjectInfo: *mut SIP_SUBJECTINFO, dwEncodingType: DWORD, pdwIndex: *mut DWORD,
    cbSignedDataMsg: DWORD, pbSignedDataMsg: *mut BYTE,
) -> BOOL>;
pub type pCryptSIPCreateIndirectData = Option<unsafe extern "system" fn(
    pSubjectInfo: *mut SIP_SUBJECTINFO, pcbIndirectData: *mut DWORD,
    pIndirectData: *mut SIP_INDIRECT_DATA,
) -> BOOL>;
pub type pCryptSIPVerifyIndirectData = Option<unsafe extern "system" fn(
    pSubjectInfo: *mut SIP_SUBJECTINFO, pIndirectData: *mut SIP_INDIRECT_DATA,
) -> BOOL>;
pub type pCryptSIPRemoveSignedDataMsg = Option<unsafe extern "system" fn(
    pSubjectInfo: *mut SIP_SUBJECTINFO, dwIndex: DWORD,
) -> BOOL>;
STRUCT!{struct SIP_DISPATCH_INFO {
    cbSize: DWORD,
    hSIP: HANDLE,
    pfGet: pCryptSIPGetSignedDataMsg,
    pfPut: pCryptSIPPutSignedDataMsg,
    pfCreate: pCryptSIPCreateIndirectData,
    pfVerify: pCryptSIPVerifyIndirectData,
    pfRemove: pCryptSIPRemoveSignedDataMsg,
}}
pub type LPSIP_DISPATCH_INFO = *mut SIP_DISPATCH_INFO;

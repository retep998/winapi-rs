// Copyright © 2015, skdltmxn
// Licensed under the MIT License <LICENSE.md>
//! Microsoft SIP Provider Prototypes and Definitions
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SIP_SUBJECTINFO {
    pub cbSize: ::DWORD,
    pub pgSubjectType: *mut ::GUID,
    pub hFile: ::HANDLE,
    pub pwsFileName: ::LPCWSTR,
    pub pwsDisplayName: ::LPCWSTR,
    pub dwReserved1: ::DWORD,
    pub dwIntVersion: ::DWORD,
    pub hProv: ::HCRYPTPROV,
    pub DigestAlgorithm: ::CRYPT_ALGORITHM_IDENTIFIER,
    pub dwFlags: ::DWORD,
    pub dwEncodingType: ::DWORD,
    pub dwReserved2: ::DWORD,
    pub fdwCAPISettings: ::DWORD,
    pub fdwSecuritySettings: ::DWORD,
    pub dwIndex: ::DWORD,
    pub dwUnionChoice: ::DWORD,
    pub psFlat: *mut MS_ADDINFO_FLAT,
    pub pClientData: ::LPVOID,
}
UNION!(SIP_SUBJECTINFO, psFlat, psCatMember, psCatMember_mut, *mut MS_ADDINFO_CATALOGMEMBER);
UNION!(SIP_SUBJECTINFO, psFlat, psBlob, psBlob_mut, *mut MS_ADDINFO_BLOB);
pub type LPSIP_SUBJECTINFO = *mut SIP_SUBJECTINFO;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct MS_ADDINFO_FLAT {
    pub cbStruct: ::DWORD,
    pub pIndirectData: *mut SIP_INDIRECT_DATA,
}
pub type PMS_ADDINFO_FLAT = *mut MS_ADDINFO_FLAT;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct MS_ADDINFO_CATALOGMEMBER {
    pub cbStruct: ::DWORD,
    pub pStore: *mut ::CRYPTCATSTORE,
    pub pMember: *mut ::CRYPTCATMEMBER,
}
pub type PMS_ADDINFO_CATALOGMEMBER = *mut MS_ADDINFO_CATALOGMEMBER;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct MS_ADDINFO_BLOB {
    pub cbStruct: ::DWORD,
    pub cbMemObject: ::DWORD,
    pub pbMemObject: *mut ::BYTE,
    pub cbMemSignedMsg: ::DWORD,
    pub pbMemSignedMsg: *mut ::BYTE,
}
pub type PMS_ADDINFO_BLOB = *mut MS_ADDINFO_BLOB;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SIP_INDIRECT_DATA {
    pub Data: ::CRYPT_ATTRIBUTE_TYPE_VALUE,
    pub DigestAlgorithm: ::CRYPT_ALGORITHM_IDENTIFIER,
    pub Digest: ::CRYPT_HASH_BLOB,
}
pub type PSIP_INDIRECT_DATA = *mut SIP_INDIRECT_DATA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SIP_ADD_NEWPROVIDER {
    pub cbStruct: ::DWORD,
    pub pgSubject: *mut ::GUID,
    pub pwszDLLFileName: *mut ::WCHAR,
    pub pwszMagicNumber: *mut ::WCHAR,
    pub pwszIsFunctionName: *mut ::WCHAR,
    pub pwszGetFuncName: *mut ::WCHAR,
    pub pwszPutFuncName: *mut ::WCHAR,
    pub pwszCreateFuncName: *mut ::WCHAR,
    pub pwszVerifyFuncName: *mut ::WCHAR,
    pub pwszRemoveFuncName: *mut ::WCHAR,
    pub pwszIsFunctionNameFmt2: *mut ::WCHAR,
    pub pwszGetCapFuncName: ::PWSTR,
}
pub type PSIP_ADD_NEWPROVIDER = *mut SIP_ADD_NEWPROVIDER;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SIP_CAP_SET_V3 {
    pub cbSize: ::DWORD,
    pub dwVersion: ::DWORD,
    pub isMultiSign: ::BOOL,
    pub dwFlags: ::DWORD,
}
UNION!(SIP_CAP_SET_V3, dwFlags, dwReserved, dwReserved_mut, ::DWORD);
pub type PSIP_CAP_SET_V3 = *mut SIP_CAP_SET_V3;
pub type SIP_CAP_SET = PSIP_CAP_SET_V3;
pub type pCryptSIPGetSignedDataMsg = Option<unsafe extern "system" fn(
    pSubjectInfo: *mut SIP_SUBJECTINFO, pdwEncodingType: *mut ::DWORD, dwIndex: ::DWORD,
    pcbSignedDataMsg: *mut ::DWORD, pbSignedDataMsg: *mut ::BYTE,
) -> ::BOOL>;
pub type pCryptSIPPutSignedDataMsg = Option<unsafe extern "system" fn(
    pSubjectInfo: *mut SIP_SUBJECTINFO, dwEncodingType: ::DWORD, pdwIndex: *mut ::DWORD,
    cbSignedDataMsg: ::DWORD, pbSignedDataMsg: *mut ::BYTE,
) -> ::BOOL>;
pub type pCryptSIPCreateIndirectData = Option<unsafe extern "system" fn(
    pSubjectInfo: *mut SIP_SUBJECTINFO, pcbIndirectData: *mut ::DWORD,
    pIndirectData: *mut SIP_INDIRECT_DATA,
) -> ::BOOL>;
pub type pCryptSIPVerifyIndirectData = Option<unsafe extern "system" fn(
    pSubjectInfo: *mut SIP_SUBJECTINFO, pIndirectData: *mut SIP_INDIRECT_DATA,
) -> ::BOOL>;
pub type pCryptSIPRemoveSignedDataMsg = Option<unsafe extern "system" fn(
    pSubjectInfo: *mut SIP_SUBJECTINFO, dwIndex: ::DWORD,
) -> ::BOOL>;
#[repr(C)] #[derive(Copy)]
pub struct SIP_DISPATCH_INFO {
    pub cbSize: ::DWORD,
    pub hSIP: ::HANDLE,
    pub pfGet: pCryptSIPGetSignedDataMsg,
    pub pfPut: pCryptSIPPutSignedDataMsg,
    pub pfCreate: pCryptSIPCreateIndirectData,
    pub pfVerify: pCryptSIPVerifyIndirectData,
    pub pfRemove: pCryptSIPRemoveSignedDataMsg,
}
impl Clone for SIP_DISPATCH_INFO { fn clone(&self) -> SIP_DISPATCH_INFO { *self } }
pub type LPSIP_DISPATCH_INFO = *mut SIP_DISPATCH_INFO;

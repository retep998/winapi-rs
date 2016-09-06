
ENUM!{enum SE_OBJECT_TYPE {
    SE_UNKNOWN_OBJECT_TYPE = 0,
    SE_FILE_OBJECT,
    SE_SERVICE,
    SE_PRINTER,
    SE_REGISTRY_KEY,
    SE_LMSHARE,
    SE_KERNEL_OBJECT,
    SE_WINDOW_OBJECT,
    SE_DS_OBJECT,
    SE_DS_OBJECT_ALL,
    SE_PROVIDER_DEFINED_OBJECT,
    SE_WMIGUID_OBJECT,
    SE_REGISTRY_WOW64_32KEY,
    SE_REGISTRY_WOW64_64KEY,
}}

ENUM!{enum ACCESS_MODE {
    NOT_USED_ACCESS = 0,
    GRANT_ACCESS,
    SET_ACCESS,
    DENY_ACCESS,
    REVOKE_ACCESS,
    SET_AUDIT_SUCCESS,
    SET_AUDIT_FAILURE,
}}

STRUCT!{struct _PEXPLICIT_ACCESS_A {
    grfAccessPermissions: ::DWORD,
    grfAccessMode: ::ACCESS_MODE,
    grfInheritance: ::DWORD,
    Trustee: TRUSTEE_A  ,
}}

pub type EXPLICIT_ACCESS_A = _PEXPLICIT_ACCESS_A;
pub type PEXPLICIT_ACCESS_A = *mut _PEXPLICIT_ACCESS_A;
pub type EXPLICIT_ACCESSA = _PEXPLICIT_ACCESS_A;
pub type PEXPLICIT_ACCESSA = *mut _PEXPLICIT_ACCESS_A;

STRUCT!{struct _PEXPLICIT_ACCESS_W {
    grfAccessPermissions: ::DWORD,
    grfAccessMode: ::ACCESS_MODE,
    grfInheritance: ::DWORD,
    Trustee: TRUSTEE_W  ,
}}

pub type EXPLICIT_ACCESS_W = _PEXPLICIT_ACCESS_W;
pub type PEXPLICIT_ACCESS_W = *mut _PEXPLICIT_ACCESS_W;
pub type EXPLICIT_ACCESSW = _PEXPLICIT_ACCESS_W;
pub type PEXPLICIT_ACCESSW = *mut _PEXPLICIT_ACCESS_W;

#[cfg(unicode)]
pub type EXPLICIT_ACCESS_=EXPLICIT_ACCESS_W;
#[cfg(unicode)]
pub type PEXPLICIT_ACCESS_=PEXPLICIT_ACCESS_W;
#[cfg(unicode)]
pub type EXPLICIT_ACCESS=EXPLICIT_ACCESSW;
#[cfg(unicode)]
pub type PEXPLICIT_ACCESS=PEXPLICIT_ACCESSW;

#[cfg(not(unicode))]
pub type EXPLICIT_ACCESS_=EXPLICIT_ACCESS_A;
#[cfg(not(unicode))]
pub type PEXPLICIT_ACCESS_=PEXPLICIT_ACCESS_A;
#[cfg(not(unicode))]
pub type EXPLICIT_ACCESS=EXPLICIT_ACCESSA;
#[cfg(not(unicode))]
pub type PEXPLICIT_ACCESS=PEXPLICIT_ACCESSA;

STRUCT!{struct _TRUSTEE_A {
    pMultipleTrustee: *mut _TRUSTEE_A,
    MultipleTrusteeOperation: MULTIPLE_TRUSTEE_OPERATION,
    TrusteeForm: TRUSTEE_FORM,
    TrusteeType: TRUSTEE_TYPE,
    ptstrName: ::LPSTR,
}}

pub type TRUSTEE_A = _TRUSTEE_A;
pub type PTRUSTEE_A = *mut _TRUSTEE_A;
pub type TRUSTEEA = _TRUSTEE_A;
pub type PTRUSTEEA = *mut _TRUSTEE_A;

STRUCT!{struct _TRUSTEE_W {
    pMultipleTrustee: *mut _TRUSTEE_A,
    MultipleTrusteeOperation: MULTIPLE_TRUSTEE_OPERATION,
    TrusteeForm: TRUSTEE_FORM,
    TrusteeType: TRUSTEE_TYPE,
    ptstrName: ::LPWSTR,
}}

pub type TRUSTEE_W = _TRUSTEE_W;
pub type PTRUSTEE_W = *mut _TRUSTEE_W;
pub type TRUSTEEW = _TRUSTEE_W;
pub type PTRUSTEEW = *mut _TRUSTEE_W;

#[cfg(unicode)]
pub type TRUSTEE_=TRUSTEE_W;
#[cfg(unicode)]
pub type PTRUSTEE_=PTRUSTEE_W;
#[cfg(unicode)]
pub type TRUSTEE=TRUSTEEW;
#[cfg(unicode)]
pub type PTRUSTEE=PTRUSTEEW;

#[cfg(not(unicode))]
pub type TRUSTEE_=TRUSTEE_A;
#[cfg(not(unicode))]
pub type PTRUSTEE_=PTRUSTEE_A;
#[cfg(not(unicode))]
pub type TRUSTEE=TRUSTEEA;
#[cfg(not(unicode))]
pub type PTRUSTEE=PTRUSTEEA;

ENUM!{enum TRUSTEE_TYPE {
    TRUSTEE_IS_UNKNOWN,
    TRUSTEE_IS_USER,
    TRUSTEE_IS_GROUP,
    TRUSTEE_IS_DOMAIN,
    TRUSTEE_IS_ALIAS,
    TRUSTEE_IS_WELL_KNOWN_GROUP,
    TRUSTEE_IS_DELETED,
    TRUSTEE_IS_INVALID,
    TRUSTEE_IS_COMPUTER,
}}

ENUM!{enum TRUSTEE_FORM {
    TRUSTEE_IS_SID,
    TRUSTEE_IS_NAME,
    TRUSTEE_BAD_FORM,
    TRUSTEE_IS_OBJECTS_AND_SID,
    TRUSTEE_IS_OBJECTS_AND_NAME,
}}

ENUM!{enum MULTIPLE_TRUSTEE_OPERATION {
    NO_MULTIPLE_TRUSTEE,
    TRUSTEE_IS_IMPERSONATE,
}}

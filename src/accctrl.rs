
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


// typedef struct _EXPLICIT_ACCESS_A
// {
//     DWORD        grfAccessPermissions;
//     ACCESS_MODE  grfAccessMode;
//     DWORD        grfInheritance;
//     TRUSTEE_A    Trustee;
// } EXPLICIT_ACCESS_A, *PEXPLICIT_ACCESS_A, EXPLICIT_ACCESSA, *PEXPLICIT_ACCESSA;

STRUCT!{struct _PEXPLICIT_ACCESS_A {
    grfAccessPermissions: ::DWORD,
    grfAccessMode: ::ACCESS_MODE,
    grfInheritance: ::DWORD,
    // Trustee: TRUSTEE_A  ,
}}

pub type EXPLICIT_ACCESS_A = _PEXPLICIT_ACCESS_A;
pub type PEXPLICIT_ACCESS_A = *mut _PEXPLICIT_ACCESS_A;
pub type EXPLICIT_ACCESSA = _PEXPLICIT_ACCESS_A;
pub type PEXPLICIT_ACCESSA = *mut _PEXPLICIT_ACCESS_A;

// typedef struct _EXPLICIT_ACCESS_W
// {
//     DWORD        grfAccessPermissions;
//     ACCESS_MODE  grfAccessMode;
//     DWORD        grfInheritance;
//     TRUSTEE_W    Trustee;
// } EXPLICIT_ACCESS_W, *PEXPLICIT_ACCESS_W, EXPLICIT_ACCESSW, *PEXPLICIT_ACCESSW;

STRUCT!{struct _PEXPLICIT_ACCESS_W {
    grfAccessPermissions: ::DWORD,
    grfAccessMode: ::ACCESS_MODE,
    grfInheritance: ::DWORD,
    // Trustee: TRUSTEE_W  ,
}}

pub type EXPLICIT_ACCESS_W = _PEXPLICIT_ACCESS_W;
pub type PEXPLICIT_ACCESS_W = *mut _PEXPLICIT_ACCESS_W;
pub type EXPLICIT_ACCESSW = _PEXPLICIT_ACCESS_W;
pub type PEXPLICIT_ACCESSW = *mut _PEXPLICIT_ACCESS_W;

// #ifdef UNICODE
// typedef EXPLICIT_ACCESS_W EXPLICIT_ACCESS_;
// typedef PEXPLICIT_ACCESS_W PEXPLICIT_ACCESS_;
// typedef EXPLICIT_ACCESSW EXPLICIT_ACCESS;
// typedef PEXPLICIT_ACCESSW PEXPLICIT_ACCESS;
// #else
// typedef EXPLICIT_ACCESS_A EXPLICIT_ACCESS_;
// typedef PEXPLICIT_ACCESS_A PEXPLICIT_ACCESS_;
// typedef EXPLICIT_ACCESSA EXPLICIT_ACCESS;
// typedef PEXPLICIT_ACCESSA PEXPLICIT_ACCESS;
// #endif // UNICODE

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

// typedef struct _TRUSTEE_A
// {
//     struct _TRUSTEE_A          *pMultipleTrustee;
//     MULTIPLE_TRUSTEE_OPERATION  MultipleTrusteeOperation;
//     TRUSTEE_FORM                TrusteeForm;
//     TRUSTEE_TYPE                TrusteeType;
// #ifdef __midl
//     [switch_is(TrusteeForm)]
//     union
//     {
//     [case(TRUSTEE_IS_NAME)]
//         LPSTR                   ptstrName;
//     [case(TRUSTEE_IS_SID)]
//         SID                    *pSid;
//     [case(TRUSTEE_IS_OBJECTS_AND_SID)]
//         OBJECTS_AND_SID        *pObjectsAndSid;
//     [case(TRUSTEE_IS_OBJECTS_AND_NAME)]
//         OBJECTS_AND_NAME_A     *pObjectsAndName;
//     };
// #else
//     LPSTR                       ptstrName;
// #endif
// } TRUSTEE_A, *PTRUSTEE_A, TRUSTEEA, *PTRUSTEEA;

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

// typedef struct _TRUSTEE_W
// {
//     struct _TRUSTEE_W          *pMultipleTrustee;
//     MULTIPLE_TRUSTEE_OPERATION  MultipleTrusteeOperation;
//     TRUSTEE_FORM                TrusteeForm;
//     TRUSTEE_TYPE                TrusteeType;
// #ifdef __midl
//     [switch_is(TrusteeForm)]
//     union
//     {
//     [case(TRUSTEE_IS_NAME)]
//         LPWSTR                  ptstrName;
//     [case(TRUSTEE_IS_SID)]
//         SID                    *pSid;
//     [case(TRUSTEE_IS_OBJECTS_AND_SID)]
//         OBJECTS_AND_SID        *pObjectsAndSid;
//     [case(TRUSTEE_IS_OBJECTS_AND_NAME)]
//         OBJECTS_AND_NAME_W     *pObjectsAndName;
//     };
// #else
//     LPWSTR                      ptstrName;
// #endif
// } TRUSTEE_W, *PTRUSTEE_W, TRUSTEEW, *PTRUSTEEW;

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

// #ifdef UNICODE
// typedef TRUSTEE_W TRUSTEE_;
// typedef PTRUSTEE_W PTRUSTEE_;
// typedef TRUSTEEW TRUSTEE;
// typedef PTRUSTEEW PTRUSTEE;
// #else
// typedef TRUSTEE_A TRUSTEE_;
// typedef PTRUSTEE_A PTRUSTEE_;
// typedef TRUSTEEA TRUSTEE;
// typedef PTRUSTEEA PTRUSTEE;
// #endif // UNICODE

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

//
// Definition: EXPLICIT_ACCESS
// This structure is used to pass access control entry information into and out
// of the system using the API defined in this document.
// grfAccessPermissions - This contains the access permissions to assign for the
//                     trustee.  It is in the form of an NT access mask.
// grfAccessMode - This field defines how the permissions are to be applied for
//                 the trustee.
// grfInheritance - For containers, this field defines how the access control
//                  entry is/(is requested) to be inherited on
//                  objects/sub-containers created within the container.
// Trustee - This field contains the definition of the trustee account the
//           explicit access applies to.
//



//
// Definition: TRUSTEE_TYPE
// This enumerated type specifies the type of trustee account for the trustee
// returned by the API described in this document.
// TRUSTEE_IS_UNKNOWN - The trustee is an unknown, but not necessarily invalid
//                      type.  This field is not validated on input to the APIs
//                      that take Trustees.
// TRUSTEE_IS_USER      The trustee account is a user account.
// TRUSTEE_IS_GROUP     The trustee account is a group account.
//

// typedef enum _TRUSTEE_TYPE
// {
//     TRUSTEE_IS_UNKNOWN,
//     TRUSTEE_IS_USER,
//     TRUSTEE_IS_GROUP,
//     TRUSTEE_IS_DOMAIN,
//     TRUSTEE_IS_ALIAS,
//     TRUSTEE_IS_WELL_KNOWN_GROUP,
//     TRUSTEE_IS_DELETED,
//     TRUSTEE_IS_INVALID,
//     TRUSTEE_IS_COMPUTER
// } TRUSTEE_TYPE;


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

//
// Definition: TRUSTEE_FORM
// This enumerated type specifies the form the trustee identifier is in for a
// particular trustee.
// TRUSTEE_IS_SID       The trustee is identified with a SID rather than with a name.
// TRUSTEE_IS_NAME      The trustee is identified with a name.
//

// typedef enum _TRUSTEE_FORM
// {
//     TRUSTEE_IS_SID,
//     TRUSTEE_IS_NAME,
//     TRUSTEE_BAD_FORM,
//     TRUSTEE_IS_OBJECTS_AND_SID,
//     TRUSTEE_IS_OBJECTS_AND_NAME
// } TRUSTEE_FORM;

ENUM!{enum TRUSTEE_FORM {
    TRUSTEE_IS_SID,
    TRUSTEE_IS_NAME,
    TRUSTEE_BAD_FORM,
    TRUSTEE_IS_OBJECTS_AND_SID,
    TRUSTEE_IS_OBJECTS_AND_NAME,
}}


//
// Definition: MULTIPLE_TRUSTEE_OPERATION
// If the trustee is a multiple trustee, this enumerated type specifies the type.
// TRUSTEE_IS_IMPERSONATE       The trustee is an impersonate trustee and the multiple
//                          trustee field in the trustee points to another trustee
//                          that is a trustee for the server that will be doing the
//                          impersonation.
//
//
// typedef enum _MULTIPLE_TRUSTEE_OPERATION
// {
//     NO_MULTIPLE_TRUSTEE,
//     TRUSTEE_IS_IMPERSONATE,
// } MULTIPLE_TRUSTEE_OPERATION;
//


ENUM!{enum MULTIPLE_TRUSTEE_OPERATION {
    NO_MULTIPLE_TRUSTEE,
    TRUSTEE_IS_IMPERSONATE,
}}

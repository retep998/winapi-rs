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

STRUCT!{struct PEXPLICIT_ACCESS_A {
    grfAccessPermissions: DWORD,
    grfAccessMode: ::ACCESS_MODE,
    grfInheritance: DWORD,
    Trustee: TRUSTEE_A  ,
}}

pub type _EXPLICIT_ACCESS_A= PEXPLICIT_ACCESS_A;
pub type *PEXPLICIT_ACCESS_A = PEXPLICIT_ACCESS_A;
pub type EXPLICIT_ACCESSA = PEXPLICIT_ACCESS_A;
pub type *PEXPLICIT_ACCESSA = PEXPLICIT_ACCESS_A;



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

// typedef struct _EXPLICIT_ACCESS_A
// {
//     DWORD        grfAccessPermissions;
//     ACCESS_MODE  grfAccessMode;
//     DWORD        grfInheritance;
//     TRUSTEE_A    Trustee;
// } EXPLICIT_ACCESS_A, *PEXPLICIT_ACCESS_A, EXPLICIT_ACCESSA, *PEXPLICIT_ACCESSA;
// typedef struct _EXPLICIT_ACCESS_W
// {
//     DWORD        grfAccessPermissions;
//     ACCESS_MODE  grfAccessMode;
//     DWORD        grfInheritance;
//     TRUSTEE_W    Trustee;
// } EXPLICIT_ACCESS_W, *PEXPLICIT_ACCESS_W, EXPLICIT_ACCESSW, *PEXPLICIT_ACCESSW;
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

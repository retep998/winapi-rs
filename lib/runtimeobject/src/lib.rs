// Copyright © 2016, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to runtimeobject.
#![cfg(windows)]
extern crate winapi;
use winapi::*;
extern "system" {
    // roapi.h
    pub fn RoInitialize(initType: RO_INIT_TYPE) -> HRESULT;
    pub fn RoUninitialize();
    pub fn RoActivateInstance(activatableClassId: HSTRING, instance: *mut *mut IInspectable) -> HRESULT;
    pub fn RoRegisterActivationFactories(
        activatableClassIds: *const HSTRING,
        activationFactoryCallbacks: *const PFNGETACTIVATIONFACTORY,
        count: UINT32,
        cookie: *mut RO_REGISTRATION_COOKIE,
    ) -> HRESULT;
    pub fn RoRevokeActivationFactories(cookie: RO_REGISTRATION_COOKIE);
    pub fn RoGetActivationFactory(
        activatableClassId: HSTRING, iid: REFIID, factory: *mut *mut VOID
    ) -> HRESULT;
    pub fn RoRegisterForApartmentShutdown(
        callbackObject: *const IApartmentShutdown, apartmentIdentifier: *mut UINT64,
        regCookie: *mut APARTMENT_SHUTDOWN_REGISTRATION_COOKIE,
    ) -> HRESULT;
    pub fn RoUnregisterForApartmentShutdown(
        regCookie: APARTMENT_SHUTDOWN_REGISTRATION_COOKIE,
    ) -> HRESULT;
    pub fn RoGetApartmentIdentifier(apartmentIdentifier: *mut UINT64) -> HRESULT;
    
    // robuffer.h
    pub fn RoGetBufferMarshaler(bufferMarshaler: *mut *mut IMarshal) -> HRESULT;
    
    // roerror.h
    pub fn RoGetErrorReportingFlags(pflags: *mut UINT32) -> HRESULT;
    pub fn RoSetErrorReportingFlags(flags: UINT32) -> HRESULT;
    pub fn RoResolveRestrictedErrorInfoReference(
        reference: PCWSTR, ppRestrictedErrorInfo: *mut *mut IRestrictedErrorInfo 
    ) -> HRESULT;
    pub fn SetRestrictedErrorInfo(pRestrictedErrorInfo: *const IRestrictedErrorInfo) -> HRESULT;
    pub fn GetRestrictedErrorInfo(
        ppRestrictedErrorInfo: *mut *mut IRestrictedErrorInfo
    ) -> HRESULT;
    pub fn RoOriginateErrorW(error: HRESULT, cchMax: UINT, message: PCWSTR) -> BOOL;
    pub fn RoOriginateError(error: HRESULT, message: HSTRING) -> BOOL;
    pub fn RoTransformErrorW(
        oldError: HRESULT, newError: HRESULT, cchMax: UINT, message: PCWSTR
    ) -> BOOL;
    pub fn RoTransformError(oldError: HRESULT, newError: HRESULT, message: HSTRING) -> BOOL;
    pub fn RoCaptureErrorContext(hr: HRESULT) -> HRESULT;
    pub fn RoFailFastWithErrorContext(hrError: HRESULT);
    pub fn RoOriginateLanguageException(
        error: HRESULT, message: HSTRING, languageException: *const IUnknown
    ) -> BOOL;
    pub fn RoClearError();
    pub fn RoReportUnhandledError(pRestrictedErrorInfo: *const IRestrictedErrorInfo) -> HRESULT;
    pub fn RoInspectThreadErrorInfo(
        targetTebAddress: UINT_PTR, machine: USHORT, readMemoryCallback: PINSPECT_MEMORY_CALLBACK,
        context: PVOID, targetErrorInfoAddress: *mut UINT_PTR,
    ) -> HRESULT;
    pub fn RoInspectCapturedStackBackTrace(
        targetErrorInfoAddress: UINT_PTR, machine: USHORT,
        readMemoryCallback: PINSPECT_MEMORY_CALLBACK, context: PVOID, frameCount: *mut UINT32,
        targetBackTraceAddress: *mut UINT_PTR,
    ) -> HRESULT;
    pub fn RoGetMatchingRestrictedErrorInfo(
        hrIn: HRESULT, ppRestrictedErrorInfo: *mut *mut IRestrictedErrorInfo, 
    ) -> HRESULT;
    pub fn RoReportFailedDelegate(
        punkDelegate: *const IUnknown, pRestrictedErrorInfo: *const IRestrictedErrorInfo,
    ) -> HRESULT;
    pub fn IsErrorPropagationEnabled() -> BOOL;

    // pub fn RoFreeParameterizedTypeExtra();
    // pub fn RoGetActivatableClassRegistration();
    // pub fn RoGetMetaDataFile();
    // pub fn RoGetParameterizedTypeInstanceIID();
    // pub fn RoGetServerActivatableClasses();
    // pub fn RoParameterizedTypeExtraGetTypeSignature();
    // pub fn RoParseTypeName();
    // pub fn RoReportFailedDelegate();
    // pub fn RoResolveNamespace();
    
    // winstring.h
    pub fn WindowsCreateString(
        sourceString: PCWSTR, length: UINT32, string: *mut HSTRING
    ) -> HRESULT;
    pub fn WindowsCreateStringReference(
        sourceString: PCWSTR, length: UINT32, hstringHeader: *mut HSTRING_HEADER,
        string: *mut HSTRING
    ) -> HRESULT;
    pub fn WindowsDeleteString(string: HSTRING) -> HRESULT;
    pub fn WindowsDuplicateString(
        string: HSTRING, newString: *mut HSTRING
    ) -> HRESULT;
    pub fn WindowsGetStringLen(string: HSTRING) -> UINT32;
    pub fn WindowsGetStringRawBuffer(string: HSTRING, length: *mut UINT32) -> PCWSTR;
    pub fn WindowsIsStringEmpty(string: HSTRING) -> BOOL;
    pub fn WindowsStringHasEmbeddedNull(string: HSTRING, hasEmbedNull: *mut BOOL) -> HRESULT;
    pub fn WindowsCompareStringOrdinal(
        string1: HSTRING, string2: HSTRING, result: *mut INT32
    ) -> HRESULT;
    pub fn WindowsSubstring(
        string: HSTRING, startIndex: UINT32, newString: *mut HSTRING
    ) -> HSTRING;
    pub fn WindowsSubstringWithSpecifiedLength(
        string: HSTRING, startIndex: UINT32, length: UINT32, newString: *mut HSTRING
    ) -> HRESULT;
    pub fn WindowsConcatString(
        string1: HSTRING, string2: HSTRING, newString: *mut HSTRING
    ) -> HRESULT;
    pub fn WindowsReplaceString(
        string: HSTRING, stringReplaced: HSTRING, stringReplaceWith: HSTRING,
        newString: *mut HSTRING
    ) -> HRESULT;
    pub fn WindowsTrimStringStart(
        string: HSTRING, trimString: HSTRING, newString: *mut HSTRING
    ) -> HRESULT;
    pub fn WindowsTrimStringEnd(
        string: HSTRING, trimString: HSTRING, newString: *mut HSTRING
    ) -> HRESULT;
    pub fn WindowsPreallocateStringBuffer(
        length: UINT32, charBuffer: *mut *mut WCHAR, bufferHandle: *mut HSTRING_BUFFER
    ) -> HRESULT;
    pub fn WindowsPromoteStringBuffer(
        bufferHandle: HSTRING_BUFFER, string: *mut HSTRING
    ) -> HRESULT;
    pub fn WindowsDeleteStringBuffer(
        bufferHandle: HSTRING_BUFFER
    ) -> HRESULT;
    pub fn WindowsInspectString(
        targetHString: UINT_PTR, machine: USHORT, callback: PINSPECT_HSTRING_CALLBACK,
        context: *const VOID, length: *mut UINT32, targetStringAddress: *mut UINT_PTR
    ) -> HRESULT;
    pub fn HSTRING_UserSize(
        pFlags: *const ULONG, StartingSize: ULONG, ppidl: *const HSTRING
    ) -> ULONG;
    pub fn HSTRING_UserMarshal(
        pFlags: *const ULONG, pBuffer: *mut UCHAR, ppidl: *const HSTRING
    ) -> *mut UCHAR;
    pub fn HSTRING_UserUnmarshal(
        pFlags: *const ULONG, pBuffer: *const UCHAR, ppidl: *mut HSTRING
    ) -> *mut UCHAR;
    pub fn HSTRING_UserFree(
        pFlags: *const ULONG, ppidl: *const HSTRING
    );
}
#[cfg(target_arch = "x86_64")]
extern "system" {
    pub fn HSTRING_UserSize64(
        pFlags: *const ULONG, StartingSize: ULONG, ppidl: *const HSTRING
    ) -> ULONG;
    pub fn HSTRING_UserMarshal64(
        pFlags: *const ULONG, pBuffer: *mut UCHAR, ppidl: *const HSTRING
    ) -> *mut UCHAR;
    pub fn HSTRING_UserUnmarshal64(
        pFlags: *const ULONG, pBuffer: *const UCHAR, ppidl: *mut HSTRING
    ) -> *mut UCHAR;
    pub fn HSTRING_UserFree64(
        pFlags: *const ULONG, ppidl: *const HSTRING
    );
}

RIDL!(
interface IRestrictedErrorInfo(IRestrictedErrorInfoVtbl): IUnknown(IUnknownVtbl) {
    fn GetErrorDetails(
        description: *mut ::BSTR, error: *mut ::HRESULT,
        restrictedDescription: *mut ::BSTR, capabilitySid: *mut ::BSTR
    ) -> ::HRESULT,
    fn GetReference(reference: *mut ::BSTR) -> ::HRESULT
}
);

RIDL!(
interface IRestrictedErrorInfo(IRestrictedErrorInfoVtbl): IUnknown(IUnknownVtbl) {
    fn GetErrorDetails(
        &self, description: *mut ::BSTR, error: *mut ::HRESULT,
        restrictedDescription: *mut ::BSTR, capabilitySid: *mut ::BSTR
    ) -> ::HRESULT,
    fn GetReference(&self, reference: *mut ::BSTR) -> ::HRESULT
}
);

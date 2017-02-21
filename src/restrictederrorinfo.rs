RIDL!(
#[uuid(0x82ba7092, 0x4c88, 0x427d, 0xa7, 0xbc, 0x16, 0xdd, 0x93, 0xfe, 0xb6, 0x7e)]
interface IRestrictedErrorInfo(IRestrictedErrorInfoVtbl): IUnknown(IUnknownVtbl) {
    fn GetErrorDetails(
        description: *mut ::BSTR, error: *mut ::HRESULT,
        restrictedDescription: *mut ::BSTR, capabilitySid: *mut ::BSTR
    ) -> ::HRESULT,
    fn GetReference(reference: *mut ::BSTR) -> ::HRESULT
}
);

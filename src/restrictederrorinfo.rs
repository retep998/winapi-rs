RIDL!(
#[uuid(0x82BA7092, 0x4C88, 0x427D, 0xA7, 0xBC, 0x16, 0xDD, 0x93, 0xFE, 0xB6, 0x7E)]
interface IRestrictedErrorInfo(IRestrictedErrorInfoVtbl): IUnknown(IUnknownVtbl) {
    fn GetErrorDetails(
        description: *mut ::BSTR, error: *mut ::HRESULT,
        restrictedDescription: *mut ::BSTR, capabilitySid: *mut ::BSTR
    ) -> ::HRESULT,
    fn GetReference(reference: *mut ::BSTR) -> ::HRESULT
}
);

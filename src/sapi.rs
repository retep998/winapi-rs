// Copyright © 2015, Connor Hilarides
// Licensed under the MIT License <LICENSE.md>
//! Mappings for the contents of sapi.h
ENUM!{enum SPDATAKEYLOCATION {
    SPDKL_DefaultLocation = 0,
    SPDKL_CurrentUser = 1,
    SPDKL_LocalMachine = 2,
    SPDKL_CurrentConfig = 5,
}}
pub const SPDUI_EngineProperties: &'static str = "EngineProperties";
pub const SPDUI_AddRemoveWord: &'static str = "AddRemoveWord";
pub const SPDUI_UserTraining: &'static str = "UserTraining";
pub const SPDUI_MicTraining: &'static str = "MicTraining";
pub const SPDUI_RecoProfileProperties: &'static str = "RecoProfileProperties";
pub const SPDUI_AudioProperties: &'static str = "AudioProperties";
pub const SPDUI_AudioVolume: &'static str = "AudioVolume";
pub const SPDUI_UserEnrollment: &'static str = "UserEnrollment";
pub const SPDUI_ShareData: &'static str = "ShareData";
pub const SPDUI_Tutorial: &'static str = "Tutorial";
ENUM!{enum SPSTREAMFORMAT {
     SPSF_Default = -1i32 as u32,
     SPSF_NoAssignedFormat = 0,
     SPSF_Text = 1,
     SPSF_NonStandardFormat = 2,
     SPSF_ExtendedAudioFormat = 3,
     SPSF_8kHz8BitMono = 4,
     SPSF_8kHz8BitStereo = 5,
     SPSF_8kHz16BitMono = 6,
     SPSF_8kHz16BitStereo = 7,
     SPSF_11kHz8BitMono = 8,
     SPSF_11kHz8BitStereo = 9,
     SPSF_11kHz16BitMono = 10,
     SPSF_11kHz16BitStereo = 11,
     SPSF_12kHz8BitMono = 12,
     SPSF_12kHz8BitStereo = 13,
     SPSF_12kHz16BitMono = 14,
     SPSF_12kHz16BitStereo = 15,
     SPSF_16kHz8BitMono = 16,
     SPSF_16kHz8BitStereo = 17,
     SPSF_16kHz16BitMono = 18,
     SPSF_16kHz16BitStereo = 19,
     SPSF_22kHz8BitMono = 20,
     SPSF_22kHz8BitStereo = 21,
     SPSF_22kHz16BitMono = 22,
     SPSF_22kHz16BitStereo = 23,
     SPSF_24kHz8BitMono = 24,
     SPSF_24kHz8BitStereo = 25,
     SPSF_24kHz16BitMono = 26,
     SPSF_24kHz16BitStereo = 27,
     SPSF_32kHz8BitMono = 28,
     SPSF_32kHz8BitStereo = 29,
     SPSF_32kHz16BitMono = 30,
     SPSF_32kHz16BitStereo = 31,
     SPSF_44kHz8BitMono = 32,
     SPSF_44kHz8BitStereo = 33,
     SPSF_44kHz16BitMono = 34,
     SPSF_44kHz16BitStereo = 35,
     SPSF_48kHz8BitMono = 36,
     SPSF_48kHz8BitStereo = 37,
     SPSF_48kHz16BitMono = 38,
     SPSF_48kHz16BitStereo = 39,
     SPSF_TrueSpeech_8kHz1BitMono = 40,
     SPSF_CCITT_ALaw_8kHzMono = 41,
     SPSF_CCITT_ALaw_8kHzStereo = 42,
     SPSF_CCITT_ALaw_11kHzMono = 43,
     SPSF_CCITT_ALaw_11kHzStereo = 44,
     SPSF_CCITT_ALaw_22kHzMono = 45,
     SPSF_CCITT_ALaw_22kHzStereo = 46,
     SPSF_CCITT_ALaw_44kHzMono = 47,
     SPSF_CCITT_ALaw_44kHzStereo = 48,
     SPSF_CCITT_uLaw_8kHzMono = 49,
     SPSF_CCITT_uLaw_8kHzStereo = 50,
     SPSF_CCITT_uLaw_11kHzMono = 51,
     SPSF_CCITT_uLaw_11kHzStereo = 52,
     SPSF_CCITT_uLaw_22kHzMono = 53,
     SPSF_CCITT_uLaw_22kHzStereo = 54,
     SPSF_CCITT_uLaw_44kHzMono = 55,
     SPSF_CCITT_uLaw_44kHzStereo = 56,
     SPSF_ADPCM_8kHzMono = 57,
     SPSF_ADPCM_8kHzStereo = 58,
     SPSF_ADPCM_11kHzMono = 59,
     SPSF_ADPCM_11kHzStereo = 60,
     SPSF_ADPCM_22kHzMono = 61,
     SPSF_ADPCM_22kHzStereo = 62,
     SPSF_ADPCM_44kHzMono = 63,
     SPSF_ADPCM_44kHzStereo = 64,
     SPSF_GSM610_8kHzMono = 65,
     SPSF_GSM610_11kHzMono = 66,
     SPSF_GSM610_22kHzMono = 67,
     SPSF_GSM610_44kHzMono = 68,
     SPSF_NUM_FORMATS = 69,
}}
pub const SPREG_USER_ROOT: &'static str = "HKEY_CURRENT_USER\\SOFTWARE\\Microsoft\\Speech";
pub const SPREG_LOCAL_MACHINE_ROOT: &'static str = "HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Speech";
pub const SPCAT_AUDIOOUT: &'static str = "HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Speech\\AudioOutput";
pub const SPCAT_AUDIOIN: &'static str = "HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Speech\\AudioInput";
pub const SPCAT_VOICES: &'static str = "HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Speech\\Voices";
pub const SPCAT_RECOGNIZERS: &'static str = "HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Speech\\Recognizers";
pub const SPCAT_APPLEXICONS: &'static str = "HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Speech\\AppLexicons";
pub const SPCAT_PHONECONVERTERS: &'static str = "HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Speech\\PhoneConverters";
pub const SPCAT_TEXTNORMALIZERS: &'static str = "HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Speech\\TextNormalizers";
pub const SPCAT_RECOPROFILES: &'static str = "HKEY_CURRENT_USER\\SOFTWARE\\Microsoft\\Speech\\RecoProfiles";
pub const SPMMSYS_AUDIO_IN_TOKEN_ID: &'static str = "HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Speech\\AudioInput\\TokenEnums\\MMAudioIn\\";
pub const SPMMSYS_AUDIO_OUT_TOKEN_ID: &'static str = "HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Speech\\AudioOutput\\TokenEnums\\MMAudioOut\\";
pub const SPCURRENT_USER_LEXICON_TOKEN_ID: &'static str = "HKEY_CURRENT_USER\\SOFTWARE\\Microsoft\\Speech\\CurrentUserLexicon";
pub const SPCURRENT_USER_SHORTCUT_TOKEN_ID: &'static str = "HKEY_CURRENT_USER\\SOFTWARE\\Microsoft\\Speech\\CurrentUserShortcut";
pub const SPTOKENVALUE_CLSID: &'static str = "CLSID";
pub const SPTOKENKEY_FILES: &'static str = "Files";
pub const SPTOKENKEY_UI: &'static str = "UI";
pub const SPTOKENKEY_ATTRIBUTES: &'static str = "Attributes";
pub const SPTOKENKEY_RETAINEDAUDIO: &'static str = "SecondsPerRetainedAudioEvent";
pub const SPTOKENKEY_AUDIO_LATENCY_WARNING: &'static str = "LatencyWarningThreshold";
pub const SPTOKENKEY_AUDIO_LATENCY_TRUNCATE: &'static str = "LatencyTruncateThreshold";
pub const SPTOKENKEY_AUDIO_LATENCY_UPDATE_INTERVAL: &'static str = "LatencyUpdateInterval";
pub const SPVOICECATEGORY_TTSRATE: &'static str = "DefaultTTSRate";
pub const SPPROP_RESOURCE_USAGE: &'static str = "ResourceUsage";
pub const SPPROP_HIGH_CONFIDENCE_THRESHOLD: &'static str = "HighConfidenceThreshold";
pub const SPPROP_NORMAL_CONFIDENCE_THRESHOLD: &'static str = "NormalConfidenceThreshold";
pub const SPPROP_LOW_CONFIDENCE_THRESHOLD: &'static str = "LowConfidenceThreshold";
pub const SPPROP_RESPONSE_SPEED: &'static str = "ResponseSpeed";
pub const SPPROP_COMPLEX_RESPONSE_SPEED: &'static str = "ComplexResponseSpeed";
pub const SPPROP_ADAPTATION_ON: &'static str = "AdaptationOn";
pub const SPPROP_PERSISTED_BACKGROUND_ADAPTATION: &'static str = "PersistedBackgroundAdaptation";
pub const SPPROP_PERSISTED_LANGUAGE_MODEL_ADAPTATION: &'static str = "PersistedLanguageModelAdaptation";
pub const SPPROP_UX_IS_LISTENING: &'static str = "UXIsListening";
pub const SPTOPIC_SPELLING: &'static str = "Spelling";
pub const SPWILDCARD: &'static str = "...";
pub const SPDICTATION: &'static str = "*";
pub const SPINFDICTATION: &'static str = "*+";
pub const SPREG_SAFE_USER_TOKENS: &'static str = "HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Speech\\UserTokens";
pub const SP_LOW_CONFIDENCE: i32 = -1;
pub const SP_NORMAL_CONFIDENCE: i32 = 0;
pub const SP_HIGH_CONFIDENCE: i32 = 1;
pub const DEFAULT_WEIGHT: i32 = 1;
pub const SP_MAX_WORD_LENGTH: i32 = 128;
pub const SP_MAX_PRON_LENGTH: i32 = 384;
pub const SP_EMULATE_RESULT: i32 = 0x40000000;
RIDL!(
interface ISpNotifyCallback(ISpNotifyCallbackVtbl) {
    fn NotifyCallback(wParam: ::WPARAM, lParam: ::LPARAM) -> ::HRESULT
}
);
pub type SPNOTIFYCALLBACK = unsafe extern "system" fn(wParam: ::WPARAM, lParam: ::LPARAM);
RIDL!(
#[uuid(0x5eff4aef, 0x8487, 0x11d2, 0x96, 0x1c, 0x00, 0xc0, 0x4f, 0x8e, 0xe6, 0x28)]
interface ISpNotifySource(ISpNotifySourceVtbl): IUnknown(IUnknownVtbl) {
    fn SetNotifySink(pNotifySink: *mut ISpNotifySink) -> ::HRESULT,
    fn SetNotifyWindowMessage(
        hWnd: ::HWND, Msg: ::UINT, wParam: ::WPARAM, lParam: ::LPARAM
    ) -> ::HRESULT,
    fn SetNotifyCallbackFunction(
        pfnCallback: SPNOTIFYCALLBACK, wParam: ::WPARAM, lParam: ::LPARAM
    ) -> ::HRESULT,
    fn SetNotifyCallbackInterface(
        pSpCallback: *mut ISpNotifyCallback, wParam: ::WPARAM, lParam: ::LPARAM
    ) -> ::HRESULT,
    fn SetNotifyWin32Event() -> ::HRESULT,
    fn WaitForNotifyEvent(dwMilliseconds: ::DWORD) -> ::HRESULT,
    fn GetNotifyEventHandle() -> ::HANDLE
}
);
RIDL!(
#[uuid(0x259684dc, 0x37c3, 0x11d2, 0x96, 0x03, 0x00, 0xc0, 0x4f, 0x8e, 0xe6, 0x28)]
interface ISpNotifySink(ISpNotifySinkVtbl): IUnknown(IUnknownVtbl) {
    fn Notify() -> ::HRESULT
}
);
RIDL!(
#[uuid(0xaca16614, 0x5d3d, 0x11d2, 0x96, 0x0e, 0x00, 0xc0, 0x4f, 0x8e, 0xe6, 0x28)]
interface ISpNotifyTranslator(ISpNotifyTranslatorVtbl): ISpNotifySink(ISpNotifySinkVtbl) {
    fn InitWindowMessage(
        hWnd: ::HWND, Msg: ::UINT, wParam: ::WPARAM, lParam: ::LPARAM
    ) -> ::HRESULT,
    fn InitCallback(
        pfnCallback: SPNOTIFYCALLBACK, wParam: ::WPARAM, lParam: ::LPARAM
    ) -> ::HRESULT,
    fn InitSpNotifyCallback(
        pSpCallback: *mut ISpNotifyCallback, wParam: ::WPARAM, lParam: ::LPARAM
    ) -> ::HRESULT,
    fn InitWin32Event(hEvent: ::HANDLE, fCloseHandleOnRelease: ::BOOL) -> ::HRESULT,
    fn Wait(dwMilliseconds: ::DWORD) -> ::HRESULT,
    fn GetEventHandle() -> ::HANDLE
}
);
RIDL!(
#[uuid(0x14056581, 0xe16c, 0x11d2, 0xbb, 0x90, 0x00, 0xc0, 0x4f, 0x8e, 0xe6, 0xc0)]
interface ISpDataKey(ISpDataKeyVtbl): IUnknown(IUnknownVtbl) {
    fn SetData(
        pszValueName: ::LPCWSTR, cbData: ::ULONG, pData: *const ::BYTE
    ) -> ::HRESULT,
    fn GetData(
        pszValueName: ::LPCWSTR, pcbData: *mut ::ULONG, pData: *mut ::BYTE
    ) -> ::HRESULT,
    fn SetStringValue(pszValueName: ::LPCWSTR, pszValue: ::LPCWSTR) -> ::HRESULT,
    fn GetStringValue(pszValueName: ::LPCWSTR, ppszValue: *mut ::LPWSTR) -> ::HRESULT,
    fn SetDWORD(pszValueName: ::LPCWSTR, dwValue: ::DWORD) -> ::HRESULT,
    fn GetDWORD(pszValueName: ::LPCWSTR, pdwValue: *mut ::DWORD) -> ::HRESULT,
    fn OpenKey(pszSubKeyName: ::LPCWSTR, ppSubKey: *mut *mut ISpDataKey) -> ::HRESULT,
    fn CreateKey(pszSubKey: ::LPCWSTR, ppSubKey: *mut *mut ISpDataKey) -> ::HRESULT,
    fn DeleteKey(pszSubKey: ::LPCWSTR) -> ::HRESULT,
    fn DeleteValue(pszValueName: ::LPCWSTR) -> ::HRESULT,
    fn EnumKeys(Index: ::ULONG, ppszSubKeyName: *mut ::LPWSTR) -> ::HRESULT,
    fn EnumValues(Index: ::ULONG, ppszValueName: *mut ::LPWSTR) -> ::HRESULT
}
);
RIDL!(
#[uuid(0x92a66e2b, 0xc830, 0x4149, 0x83, 0xdf, 0x6f, 0xc2, 0xba, 0x1e, 0x7a, 0x5b)]
interface ISpRegDataKey(ISpRegDataKeyVtbl): ISpDataKey(ISpDataKeyVtbl) {
    fn SetKey(hkey: ::HKEY, fReadOnly: ::BOOL) -> ::HRESULT
}
);
RIDL!(
#[uuid(0x2d3d3845, 0x39af, 0x4850, 0xbb, 0xf9, 0x40, 0xb4, 0x97, 0x80, 0x01, 0x1d)]
interface ISpObjectTokenCategory(ISpObjectTokenCategoryVtbl): ISpDataKey(ISpDataKeyVtbl) {
    fn SetId(pszCategoryId: ::LPCWSTR, fCreateIfNotExist: ::BOOL) -> ::HRESULT,
    fn GetId(ppszCoMemCategoryId: *mut ::LPWSTR) -> ::HRESULT,
    fn GetDataKey(
        spdkl: SPDATAKEYLOCATION, pppDataKey: *mut *mut ISpDataKey
    ) -> ::HRESULT,
    fn EnumTokens(
        pzsReqAttribs: ::LPCWSTR, pszOptAttribs: ::LPCWSTR,
        ppEnum: *mut *mut IEnumSpObjectTokens
    ) -> ::HRESULT,
    fn SetDefaultTokenId(pszTokenId: ::LPCWSTR) -> ::HRESULT,
    fn GetDefaultTokenId(ppszCoMemTokenId: *mut ::LPWSTR) -> ::HRESULT
}
);
RIDL!(
#[uuid(0x14056589, 0xe16c, 0x11d2, 0xbb, 0x90, 0x00, 0xc0, 0x4f, 0x8e, 0xe6, 0xc0)]
interface ISpObjectToken(ISpObjectTokenVtbl): ISpDataKey(ISpDataKeyVtbl) {
    fn SetId(
        pszCategoryId: ::LPCWSTR, pszTokenId: ::LPCWSTR, fCreateIfNotExist: ::BOOL
    ) -> ::HRESULT,
    fn GetId(ppszCoMemTokenId: *mut ::LPWSTR) -> ::HRESULT,
    fn GetCategory(ppTokenCategory: *mut *mut ISpObjectTokenCategory) -> ::HRESULT,
    fn CreateInstance(
        pUnkOuter: *mut ::IUnknown, dwClsContext: ::DWORD, riid: ::REFIID,
        ppvObject: *mut *mut ::c_void
    ) -> ::HRESULT,
    fn GetStorageFileName(
        clsidCaller: ::REFCLSID, pszValueName: ::LPCWSTR,
        pszFileNameSpecifier: ::LPCWSTR, nFolder: ::ULONG, ppszFilePath: *mut ::LPWSTR
    ) -> ::HRESULT,
    fn RemoveStorageFileName(pszKeyName: ::LPCWSTR, fDeleteFile: ::BOOL) -> ::HRESULT,
    fn Remove(pclsidCaller: *const ::CLSID) -> ::HRESULT,
    fn IsUISupported(
        pszTypeOfUI: ::LPCWSTR, pvExtraData: *mut ::c_void, cbExtraData: ::ULONG,
        punkObject: *mut ::IUnknown, pfSupported: *mut ::BOOL
    ) -> ::HRESULT,
    fn DisplayUI(
        hwndParent: ::HWND, pszTitle: ::LPCWSTR, pszTypeOfUI: ::LPCWSTR,
        pvExtraData: *mut ::c_void, cbExtraData: ::ULONG, punkObject: *mut ::IUnknown
    ) -> ::HRESULT,
    fn MatchesAttributes(pszAttributes: ::LPCWSTR, pfMatches: *mut ::BOOL) -> ::HRESULT
}
);
RIDL!(
#[uuid(0xb8aab0cf, 0x346f, 0x49d8, 0x94, 0x99, 0xc8, 0xb0, 0x3f, 0x16, 0x1d, 0x51)]
interface ISpObjectTokenInit(ISpObjectTokenInitVtbl): ISpObjectToken(ISpObjectTokenVtbl) {
    fn InitFromDataKey(
        pszCategoryId: ::LPCWSTR, pszTokenId: ::LPCWSTR, pDataKey: *mut ISpDataKey
    ) -> ::HRESULT
}
);
RIDL!(
#[uuid(0x06b64f9e, 0x7fda, 0x11d2, 0xb4, 0xf2, 0x00, 0xc0, 0x4f, 0x79, 0x73, 0x96)]
interface IEnumSpObjectTokens(IEnumSpObjectTokensVtbl): IUnknown(IUnknownVtbl) {
    fn Next(
        celt: ::ULONG, pelt: *mut *mut ISpObjectToken, pceltFetched: *mut ::ULONG
    ) -> ::HRESULT,
    fn Skip(celt: ::ULONG) -> ::HRESULT,
    fn Reset() -> ::HRESULT,
    fn Clone(ppEnum: *mut *mut IEnumSpObjectTokens) -> ::HRESULT,
    fn Item(Index: ::ULONG, ppToken: *mut *mut ISpObjectToken) -> ::HRESULT,
    fn GetCount(pCount: *mut ::ULONG) -> ::HRESULT
}
);
RIDL!(
#[uuid(0x5b559f40, 0xe952, 0x11d2, 0xbb, 0x91, 0x00, 0xc0, 0x4f, 0x8e, 0xe6, 0xc0)]
interface ISpObjectWithToken(ISpObjectWithTokenVtbl): IUnknown(IUnknownVtbl) {
    fn SetObjectToken(pToken: *mut ISpObjectToken) -> ::HRESULT,
    fn GetObjectToken(ppToken: *mut *mut ISpObjectToken) -> ::HRESULT
}
);
RIDL!(
#[uuid(0x93384e18, 0x5014, 0x43d5, 0xad, 0xbb, 0xa7, 0x8e, 0x05, 0x59, 0x26, 0xbd)]
interface ISpResourceManager(ISpResourceManagerVtbl): IServiceProvider(IServiceProviderVtbl) {
    fn SetObject(guidServiceId: ::REFGUID, pUnkObject: *mut ::IUnknown) -> ::HRESULT,
    fn GetObject(
        guidServiceId: ::REFGUID, ObjectCLSID: ::REFCLSID, ObjectIID: ::REFIID,
        fReleaseWhenLastExternalRefReleased: ::BOOL, ppObject: *mut *mut ::c_void
    ) -> ::HRESULT
}
);
ENUM!{enum SPEVENTLPARAMTYPE {
    SPET_LPARAM_IS_UNDEFINED = 0,
    SPET_LPARAM_IS_TOKEN,
    SPET_LPARAM_IS_OBJECT,
    SPET_LPARAM_IS_POINTER,
    SPET_LPARAM_IS_STRING,
}}
ENUM!{enum SPEVENTENUM {
    SPEI_UNDEFINED = 0,
    SPEI_START_INPUT_STREAM = 1,
    SPEI_END_INPUT_STREAM = 2,
    SPEI_VOICE_CHANGE = 3,
    SPEI_TTS_BOOKMARK = 4,
    SPEI_WORD_BOUNDARY = 5,
    SPEI_PHONEME = 6,
    SPEI_SENTENCE_BOUNDARY = 7,
    SPEI_VISEME = 8,
    SPEI_TTS_AUDIO_LEVEL = 9,
    SPEI_TTS_PRIVATE = 15,
    SPEI_END_SR_STREAM = 34,
    SPEI_SOUND_START = 35,
    SPEI_SOUND_END = 36,
    SPEI_PHRASE_START = 37,
    SPEI_RECOGNITION = 38,
    SPEI_HYPOTHESIS = 39,
    SPEI_SR_BOOKMARK = 40,
    SPEI_PROPERTY_NUM_CHANGE = 41,
    SPEI_PROPERTY_STRING_CHANGE = 42,
    SPEI_FALSE_RECOGNITION = 43,
    SPEI_INTERFERENCE = 44,
    SPEI_REQUEST_UI = 45,
    SPEI_RECO_STATE_CHANGE = 46,
    SPEI_ADAPTATION = 47,
    SPEI_START_SR_STREAM = 48,
    SPEI_RECO_OTHER_CONTEXT = 49,
    SPEI_SR_AUDIO_LEVEL = 50,
    SPEI_SR_RETAINEDAUDIO = 51,
    SPEI_SR_PRIVATE = 52,
    SPEI_ACTIVE_CATEGORY_CHANGED = 53,
    SPEI_RESERVED5 = 54,
    SPEI_RESERVED6 = 55,
    SPEI_RESERVED1 = 30,
    SPEI_RESERVED2 = 33,
    SPEI_RESERVED3 = 63,
}}
pub const SPEI_MIN_TTS: SPEVENTENUM = SPEI_START_INPUT_STREAM;
pub const SPEI_MAX_TTS: SPEVENTENUM = SPEI_TTS_PRIVATE;
pub const SPEI_MIN_SR: SPEVENTENUM = SPEI_END_SR_STREAM;
pub const SPEI_MAX_SR: SPEVENTENUM = SPEI_RESERVED6;
pub const SPFEI_FLAGCHECK: u64 = (1 << SPEI_RESERVED1 as u64) | (1 << SPEI_RESERVED2 as u64);
pub const SPFEI_ALL_TTS_EVENTS: u64 = 0x000000000000FFFE | SPFEI_FLAGCHECK;
pub const SPFEI_ALL_SR_EVENTS: u64 = 0x003FFFFC00000000 | SPFEI_FLAGCHECK;
pub const SPFEI_ALL_EVENTS: u64 = 0xEFFFFFFFFFFFFFFF;
#[inline]
pub fn SPFEI(SPEI_ord: u64) -> u64 {
    (1 << SPEI_ord) | SPFEI_FLAGCHECK
}
STRUCT!{struct SPEVENT {
    eEventId: ::WORD,
    elParamType: ::WORD,
    ulStreamNum: ::ULONG,
    ullAudioStreamOffset: ::ULONGLONG,
    wParam: ::WPARAM,
    lParam: ::LPARAM,
}}
STRUCT!{struct SPSERIALIZEDEVENT {
    eEventId: ::WORD,
    elParamType: ::WORD,
    ulStreamNum: ::ULONG,
    ullAudioStreamOffset: ::ULONGLONG,
    SerializedwParam: ::ULONG,
    SerializedlParam: ::LONG,
}}
STRUCT!{struct SPSERIALIZEDEVENT64 {
    eEventId: ::WORD,
    elParamType: ::WORD,
    ulStreamNum: ::ULONG,
    ullAudioStreamOffset: ::ULONGLONG,
    SerializedwParam: ::ULONGLONG,
    SerializedlParam: ::LONGLONG,
}}
STRUCT!{struct SPEVENTEX {
    eEventId: ::WORD,
    elParamType: ::WORD,
    ulStreamNum: ::ULONG,
    ullAudioStreamOffset: ::ULONGLONG,
    wParam: ::WPARAM,
    lParam: ::LPARAM,
    ullAudioTimeOffset: ::ULONGLONG,
}}
ENUM!{enum SPINTERFERENCE {
    SPINTERFERENCE_NONE = 0,
    SPINTERFERENCE_NOISE = 1,
    SPINTERFERENCE_NOSIGNAL = 2,
    SPINTERFERENCE_TOOLOUD = 3,
    SPINTERFERENCE_TOOQUIET = 4,
    SPINTERFERENCE_TOOFAST = 5,
    SPINTERFERENCE_TOOSLOW = 6,
    SPINTERFERENCE_LATENCY_WARNING = 7,
    SPINTERFERENCE_LATENCY_TRUNCATE_BEGIN = 8,
    SPINTERFERENCE_LATENCY_TRUNCATE_END = 9,
}}
ENUM!{enum SPENDSRSTREAMFLAGS {
    SPESF_NONE = 0,
    SPESF_STREAM_RELEASED = 1 << 0,
    SPESF_EMULATED = 1 << 1,
}}
ENUM!{enum SPVFEATURE {
    SPVFEATURE_STRESSED = 1 << 0,
    SPVFEATURE_EMPHASIS = 1 << 1,
}}
ENUM!{enum SPVISEMES {
    SP_VISEME_0 = 0,
    SP_VISEME_1,
    SP_VISEME_2,
    SP_VISEME_3,
    SP_VISEME_4,
    SP_VISEME_5,
    SP_VISEME_6,
    SP_VISEME_7,
    SP_VISEME_8,
    SP_VISEME_9,
    SP_VISEME_10,
    SP_VISEME_11,
    SP_VISEME_12,
    SP_VISEME_13,
    SP_VISEME_14,
    SP_VISEME_15,
    SP_VISEME_16,
    SP_VISEME_17,
    SP_VISEME_18,
    SP_VISEME_19,
    SP_VISEME_20,
    SP_VISEME_21,
}}
STRUCT!{struct SPEVENTSOURCEINFO {
    ullEventInterest: ::ULONGLONG,
    ullQueuedInterest: ::ULONGLONG,
    ulCount: ::ULONG,
}}
RIDL!(
#[uuid(0xbe7a9cce, 0x5f9e, 0x11d2, 0x96, 0x0f, 0x00, 0xc0, 0x4f, 0x8e, 0xe6, 0x28)]
interface ISpEventSource(ISpEventSourceVtbl): ISpNotifySource(ISpNotifySourceVtbl) {
    fn SetInterest(
        ullEventInterest: ::ULONGLONG, ullQueuedInterest: ::ULONGLONG
    ) -> ::HRESULT,
    fn GetEvents(
        ulCount: ::ULONG, pEventArray: *mut SPEVENT, pulFetched: *mut ::ULONG
    ) -> ::HRESULT,
    fn GetInfo(pInfo: *mut SPEVENTSOURCEINFO) -> ::HRESULT
}
);
RIDL!(
#[uuid(0x2373a435, 0x6a4b, 0x429e, 0xa6, 0xac, 0xd4, 0x23, 0x1a, 0x61, 0x97, 0x5b)]
interface ISpEventSource2(ISpEventSource2Vtbl): ISpEventSource(ISpEventSourceVtbl) {
    fn GetEventsEx(
        ulCount: ::ULONG, pEventArray: *mut SPEVENTEX, pulFetched: *mut ::ULONG
    ) -> ::HRESULT
}
);
RIDL!(
#[uuid(0xbe7a9cc9, 0x5f9e, 0x11d2, 0x96, 0x0f, 0x00, 0xc0, 0x4f, 0x8e, 0xe6, 0x28)]
interface ISpEventSink(ISpEventSinkVtbl): IUnknown(IUnknownVtbl) {
    fn AddEvents(pEventArray: *const SPEVENT, ulCount: ::ULONG) -> ::HRESULT,
    fn GetEventInterest(pullEventInterest: *mut ::ULONGLONG) -> ::HRESULT
}
);
RIDL!(
#[uuid(0xbed530be, 0x2606, 0x4f4d, 0xa1, 0xc0, 0x54, 0xc5, 0xcd, 0xa5, 0x56, 0x6f)]
interface ISpStreamFormat(ISpStreamFormatVtbl): IStream(IStreamVtbl) {
    fn GetFormat(
        pguidFormatId: *mut ::GUID, ppCoMemWaveFormatEx: *mut *mut ::WAVEFORMATEX
    ) -> ::HRESULT
}
);
ENUM!{enum SPFILEMODE {
    SPFM_OPEN_READONLY = 0,
    SPFM_OPEN_READWRITE = 1,
    SPFM_CREATE = 2,
    SPFM_CREATE_ALWAYS = 3,
    SPFM_NUM_MODES = 4,
}}
RIDL!(
#[uuid(0x12e3cca9, 0x7518, 0x44c5, 0xa5, 0xe7, 0xba, 0x5a, 0x79, 0xcb, 0x92, 0x9e)]
interface ISpStream(ISpStreamVtbl): ISpStreamFormat(ISpStreamFormatVtbl) {
    fn SetBaseStream(
        pStream: *mut ::IStream, rguidFormat: ::REFGUID,
        pWaveFormatEx: *const ::WAVEFORMATEX
    ) -> ::HRESULT,
    fn GetBaseStream(ppStream: *mut *mut ::IStream) -> ::HRESULT,
    fn BindToFile(
        pszFileName: ::LPCWSTR, eMode: SPFILEMODE, pFormatId: *const ::GUID,
        pWaveFormatEx: *const ::WAVEFORMATEX, ullEventInterest: ::ULONGLONG
    ) -> ::HRESULT,
    fn Close() -> ::HRESULT
}
);
RIDL!(
#[uuid(0x678a932c, 0xea71, 0x4446, 0x9b, 0x41, 0x78, 0xfd, 0xa6, 0x28, 0x0a, 0x29)]
interface ISpStreamFormatConverter(ISpStreamFormatConverterVtbl)
    : ISpStreamFormat(ISpStreamFormatVtbl) {
    fn SetBaseStream(
        pStream: *mut ISpStreamFormat, fSetFormatToBaseStreamFormat: ::BOOL,
        fWriteToBaseStream: ::BOOL
    ) -> ::HRESULT,
    fn GetBaseStream(ppStream: *mut *mut ISpStreamFormat) -> ::HRESULT,
    fn SetFormat(
        rguidFormatIdOfConvertedStream: ::REFGUID,
        pWaveFormatExOfConvertedStream: *const ::WAVEFORMATEX
    ) -> ::HRESULT,
    fn ResetSeekPosition() -> ::HRESULT,
    fn ScaleConvertedToBaseOffset(
        ullOffsetConvertedStream: ::ULONGLONG, pullOffsetBaseStream: *mut ::ULONGLONG
    ) -> ::HRESULT,
    fn ScaleBaseToConvertedOffset(
        ullOffsetBaseStream: ::ULONGLONG, pullOffsetConvertedStream: *mut ::ULONGLONG
    ) -> ::HRESULT
}
);
ENUM!{enum SPAUDIOSTATE {
    SPAS_CLOSED = 0,
    SPAS_STOP = 1,
    SPAS_PAUSE = 2,
    SPAS_RUN = 3,
}}
STRUCT!{struct SPAUDIOSTATUS {
    cbFreeBuffSpace: ::LONG,
    cbNonBlockingIO: ::ULONG,
    State: SPAUDIOSTATE,
    CurSeekPos: ::ULONGLONG,
    CurDevicePos: ::ULONGLONG,
    dwAudioLevel: ::DWORD,
    dwReserved2: ::DWORD,
}}
STRUCT!{struct SPAUDIOBUFFERINFO {
    ulMsMinNotification: ::ULONG,
    ulMsBufferSize: ::ULONG,
    ulMsEventBias: ::ULONG,
}}
RIDL!(
#[uuid(0xc05c768f, 0xfae8, 0x4ec2, 0x8e, 0x07, 0x33, 0x83, 0x21, 0xc1, 0x24, 0x52)]
interface ISpAudio(ISpAudioVtbl): ISpStreamFormat(ISpStreamFormatVtbl) {
    fn SetState(NewState: SPAUDIOSTATE, ullReserved: ::ULONGLONG) -> ::HRESULT,
    fn SetFormat(
        rguidFmtId: ::REFGUID, pWaveFormatEx: *const ::WAVEFORMATEX
    ) -> ::HRESULT,
    fn GetStatus(pStatus: *mut SPAUDIOSTATUS) -> ::HRESULT,
    fn SetBufferInfo(pBuffInfo: *const SPAUDIOBUFFERINFO) -> ::HRESULT,
    fn GetBufferInfo(pBuffInfo: *mut SPAUDIOBUFFERINFO) -> ::HRESULT,
    fn GetDefaultFormat(
        pFormatId: *mut ::GUID, ppCoMemWaveFormatEx: *mut *mut ::WAVEFORMATEX
    ) -> ::HRESULT,
    fn EventHandle() -> ::HANDLE,
    fn GetVolumeLevel(pLevel: *mut ::ULONG) -> ::HRESULT,
    fn SetVolumeLevel(Level: ::ULONG) -> ::HRESULT,
    fn GetBufferNotifySize(pcbSize: *mut ::ULONG) -> ::HRESULT,
    fn SetBufferNotifySize(cbSize: ::ULONG) -> ::HRESULT
}
);
RIDL!(
#[uuid(0x15806f6e, 0x1d70, 0x4b48, 0x98, 0xe6, 0x3b, 0x1a, 0x00, 0x75, 0x09, 0xab)]
interface ISpMMSysAudio(ISpMMSysAudioVtbl): ISpAudio(ISpAudioVtbl) {
    fn GetDeviceId(puDeviceId: *mut ::UINT) -> ::HRESULT,
    fn SetDeviceId(uDeviceId: ::UINT) -> ::HRESULT,
    fn GetMMHandle(pHandle: *mut *mut ::c_void) -> ::HRESULT,
    fn GetLineId(puLineId: *mut ::UINT) -> ::HRESULT,
    fn SetLineId(uLineId: ::UINT) -> ::HRESULT
}
);
RIDL!(
#[uuid(0x10f63bce, 0x201a, 0x11d3, 0xac, 0x70, 0x00, 0xc0, 0x4f, 0x8e, 0xe6, 0xc0)]
interface ISpTranscript(ISpTranscriptVtbl): IUnknown(IUnknownVtbl) {
    fn GetTranscript(ppszTranscript: *mut ::LPWSTR) -> ::HRESULT,
    fn AppendTranscript(pszTranscript: ::LPCWSTR) -> ::HRESULT
}
);
ENUM!{enum SPDISPLYATTRIBUTES {
    SPAF_ONE_TRAILING_SPACE = 0x2,
    SPAF_TWO_TRAILING_SPACES = 0x4,
    SPAF_CONSUME_LEADING_SPACES = 0x8,
    SPAF_BUFFER_POSITION = 0x10,
    SPAF_ALL = 0x1f,
    SPAF_USER_SPECIFIED = 0x80,
}}
pub type SPPHONEID = ::WCHAR;
pub type PSPPHONEID = ::LPWSTR;
pub type PCSPPHONEID = ::LPCWSTR;
STRUCT!{struct SPPHRASEELEMENT {
    ulAudioTimeOffset: ::ULONG,
    ulAudioSizeTime: ::ULONG,
    ulAudioStreamOffset: ::ULONG,
    ulAudioSizeBytes: ::ULONG,
    ulRetainedStreamOffset: ::ULONG,
    ulRetainedSizeBytes: ::ULONG,
    pszDisplayText: ::LPCWSTR,
    pszLexicalForm: ::LPCWSTR,
    pszPronunciation: *const SPPHONEID,
    bDisplayAttributes: ::BYTE,
    RequiredConfidence: ::c_char,
    ActualConfidence: ::c_char,
    Reserved: ::BYTE,
    SREngineConfidence: ::c_float,
}}
STRUCT!{struct SPPHRASERULE {
    pszName: ::LPCWSTR,
    ulId: ::ULONG,
    ulFirstElement: ::ULONG,
    ulCountOfElements: ::ULONG,
    pNextSibling: *const SPPHRASERULE,
    pFirstChild: *const SPPHRASERULE,
    SREngineConfidence: ::c_float,
    Confidence: ::c_char,
}}
ENUM!{enum SPPHRASEPROPERTYUNIONTYPE {
    SPPPUT_UNUSED = 0,
    SPPPUT_ARRAY_INDEX,
}}
STRUCT!{struct SPPHRASEPROPERTY {
    pszName: ::LPCWSTR,
    bType: ::BYTE,
    bReserved: ::BYTE,
    usArrayIndex: u16,
    pszValue: ::LPCWSTR,
    vValue: ::VARIANT,
    ulFirstElement: ::ULONG,
    ulCountOfElements: ::ULONG,
    pNextSibling: *const SPPHRASEPROPERTY,
    pFirstChild: *const SPPHRASEPROPERTY,
    SREngineConfidence: ::c_float,
    Confidence: ::c_char,
}}
UNION!(SPPHRASEPROPERTY, bType, ulId, ulId_mut, ::ULONG);
STRUCT!{struct SPPHRASEREPLACEMENT {
    bDisplayAttributes: ::BYTE,
    pszReplacementText: ::LPCWSTR,
    ulFirstElement: ::ULONG,
    ulCountOfElements: ::ULONG,
}}
STRUCT!{struct SPSEMANTICERRORINFO {
    ulLineNumber: ::ULONG,
    pszScriptLine: ::LPWSTR,
    pszSource: ::LPWSTR,
    pszDescription: ::LPWSTR,
    hrResultCode: ::HRESULT,
}}
ENUM!{enum SPSEMANTICFORMAT {
    SPSMF_SAPI_PROPERTIES = 0,
    SPSMF_SRGS_SEMANTICINTERPRETATION_MS = 1,
    SPSMF_SRGS_SAPIPROPERTIES = 2,
    SPSMF_UPS = 4,
    SPSMF_SRGS_SEMANTICINTERPRETATION_W3C = 8,
}}
STRUCT!{struct SPPHRASE_50 {
    cbSize: ::ULONG,
    LangID: ::WORD,
    wHomophoneGroupId: ::WORD,
    ullGrammarID: ::ULONGLONG,
    ftStartTime: ::ULONGLONG,
    ullAudioStreamPosition: ::ULONGLONG,
    ulAudioSizeBytes: ::ULONG,
    ulRetainedSizeBytes: ::ULONG,
    ulAudioSizeTime: ::ULONG,
    Rule: ::SPPHRASERULE,
    pProperties: *const ::SPPHRASEPROPERTY,
    pElements: *const ::SPPHRASEELEMENT,
    cReplacements: ::ULONG,
    pReplacements: *const ::SPPHRASEREPLACEMENT,
    SREngineID: ::GUID,
    ulSREnginePrivateDataSize: ::ULONG,
    pSREnginePrivateData: *const ::BYTE,
}}
STRUCT!{struct SPPHRASE_53 {
    cbSize: ::ULONG,
    LangID: ::WORD,
    wHomophoneGroupId: ::WORD,
    ullGrammarID: ::ULONGLONG,
    ftStartTime: ::ULONGLONG,
    ullAudioStreamPosition: ::ULONGLONG,
    ulAudioSizeBytes: ::ULONG,
    ulRetainedSizeBytes: ::ULONG,
    ulAudioSizeTime: ::ULONG,
    Rule: ::SPPHRASERULE,
    pProperties: *const ::SPPHRASEPROPERTY,
    pElements: *const ::SPPHRASEELEMENT,
    cReplacements: ::ULONG,
    pReplacements: *const ::SPPHRASEREPLACEMENT,
    SREngineID: ::GUID,
    ulSREnginePrivateDataSize: ::ULONG,
    pSREnginePrivateData: *const ::BYTE,
    pSML: ::LPWSTR,
    pSemanticErrorInfo: *mut SPSEMANTICERRORINFO,
}}
STRUCT!{struct SPPHRASE {
    cbSize: ::ULONG,
    LangID: ::WORD,
    wHomophoneGroupId: ::WORD,
    ullGrammarID: ::ULONGLONG,
    ftStartTime: ::ULONGLONG,
    ullAudioStreamPosition: ::ULONGLONG,
    ulAudioSizeBytes: ::ULONG,
    ulRetainedSizeBytes: ::ULONG,
    ulAudioSizeTime: ::ULONG,
    Rule: ::SPPHRASERULE,
    pProperties: *const ::SPPHRASEPROPERTY,
    pElements: *const ::SPPHRASEELEMENT,
    cReplacements: ::ULONG,
    pReplacements: *const ::SPPHRASEREPLACEMENT,
    SREngineID: ::GUID,
    ulSREnginePrivateDataSize: ::ULONG,
    pSREnginePrivateData: *const ::BYTE,
    pSML: ::LPWSTR,
    pSemanticErrorInfo: *mut SPSEMANTICERRORINFO,
    SemanticTagFormat: SPSEMANTICFORMAT,
}}
STRUCT!{struct SPSERIALIZEDPHRASE {
    ulSerializedSize: ::ULONG,
}}
STRUCT!{struct SPRULE {
    pszRuleName: ::LPCWSTR,
    ulRuleId: ::ULONG,
    dwAttributes: ::DWORD,
}}
ENUM!{enum SPVALUETYPE {
    SPDF_PROPERTY = 0x1,
    SPDF_REPLACEMENT = 0x2,
    SPDF_RULE = 0x4,
    SPDF_DISPLAYTEXT = 0x8,
    SPDF_LEXICALFORM = 0x10,
    SPDF_PRONUNCIATION = 0x20,
    SPDF_AUDIO = 0x40,
    SPDF_ALTERNATES = 0x80,
    SPDF_ALL = 0xff,
}}
STRUCT!{struct SPBINARYGRAMMAR {
    ulTotalSerializedSize: ::ULONG,
}}
ENUM!{enum SPPHRASERNG {
    SPPR_ALL_ELEMENTS = -1i32 as u32,
}}
pub const SP_GETWHOLEPHRASE: SPPHRASERNG = SPPR_ALL_ELEMENTS;
pub const SPRR_ALL_ELEMENTS: SPPHRASERNG = SPPR_ALL_ELEMENTS;
DECLARE_HANDLE!(SPSTATEHANDLE, SPSTATEHANDLE__);
ENUM!{enum SPRECOEVENTFLAGS {
    SPREF_AutoPause = 1 << 0,
    SPREF_Emulated = 1 << 1,
    SPREF_SMLTimeout = 1 << 2,
    SPREF_ExtendableParse = 1 << 3,
    SPREF_ReSent = 1 << 4,
    SPREF_Hypothesis = 1 << 5,
    SPREF_FalseRecognition = 1 << 6,
}}
ENUM!{enum SPPARTOFSPEECH {
    SPPS_NotOverriden = -1i32 as u32,
    SPPS_Unknown = 0,
    SPPS_Noun = 0x1000,
    SPPS_Verb = 0x2000,
    SPPS_Modifier = 0x3000,
    SPPS_Function = 0x4000,
    SPPS_Interjection = 0x5000,
    SPPS_Noncontent = 0x6000,
    SPPS_LMA = 0x7000,
    SPPS_SuppressWord = 0xf000,
}}
ENUM!{enum SPLEXICONTYPE {
    eLEXTYPE_USER = 1 << 0,
    eLEXTYPE_APP = 1 << 1,
    eLEXTYPE_VENDORLEXICON = 1 << 2,
    eLEXTYPE_LETTERTOSOUND = 1 << 3,
    eLEXTYPE_MORPHOLOGY = 1 << 4,
    eLEXTYPE_RESERVED4 = 1 << 5,
    eLEXTYPE_USER_SHORTCUT = 1 << 6,
    eLEXTYPE_RESERVED6 = 1 << 7,
    eLEXTYPE_RESERVED7 = 1 << 8,
    eLEXTYPE_RESERVED8 = 1 << 9,
    eLEXTYPE_RESERVED9 = 1 << 10,
    eLEXTYPE_RESERVED10 = 1 << 11,
    eLEXTYPE_PRIVATE1 = 1 << 12,
    eLEXTYPE_PRIVATE2 = 1 << 13,
    eLEXTYPE_PRIVATE3 = 1 << 14,
    eLEXTYPE_PRIVATE4 = 1 << 15,
    eLEXTYPE_PRIVATE5 = 1 << 16,
    eLEXTYPE_PRIVATE6 = 1 << 17,
    eLEXTYPE_PRIVATE7 = 1 << 18,
    eLEXTYPE_PRIVATE8 = 1 << 19,
    eLEXTYPE_PRIVATE9 = 1 << 20,
    eLEXTYPE_PRIVATE10 = 1 << 21,
    eLEXTYPE_PRIVATE11 = 1 << 22,
    eLEXTYPE_PRIVATE12 = 1 << 23,
    eLEXTYPE_PRIVATE13 = 1 << 24,
    eLEXTYPE_PRIVATE14 = 1 << 25,
    eLEXTYPE_PRIVATE15 = 1 << 26,
    eLEXTYPE_PRIVATE16 = 1 << 27,
    eLEXTYPE_PRIVATE17 = 1 << 28,
    eLEXTYPE_PRIVATE18 = 1 << 29,
    eLEXTYPE_PRIVATE19 = 1 << 30,
    eLEXTYPE_PRIVATE20 = 1 << 31,
}}
ENUM!{enum SPWORDTYPE {
    eWORDTYPE_ADDED = 1 << 0,
    eWORDTYPE_DELETED = 1 << 1,
}}
ENUM!{enum SPPRONUNCIATIONFLAGS {
    ePRONFLAG_USED = 1 << 0,
}}
STRUCT!{struct SPWORDPRONUNCIATION {
    pNextWordPronunciation: *mut SPWORDPRONUNCIATION,
    eLexiconType: SPLEXICONTYPE,
    LangID: ::WORD,
    wPronunciationFlags: ::WORD,
    ePartOfSpeech: SPPARTOFSPEECH,
    szPronunciation: [SPPHONEID; 1],
}}
STRUCT!{struct SPWORDPRONUNCIATIONLIST {
    ulSize: ::ULONG,
    pvBuffer: *mut ::BYTE,
    pFirstWordPronunciation: *mut SPWORDPRONUNCIATION,
}}
STRUCT!{struct SPWORD {
    pNextWord: *mut SPWORD,
    LangID: ::WORD,
    wReserved: ::WORD,
    eWordType: SPWORDTYPE,
    pszWord: ::LPWSTR,
    pFirstWordPronunciation: *mut SPWORDPRONUNCIATION,
}}
STRUCT!{struct SPWORDLIST {
    ulSize: ::ULONG,
    pvBuffer: *mut ::BYTE,
    pFirstWord: *mut SPWORD,
}}
RIDL!(
#[uuid(0xda41a7c2, 0x5383, 0x4db2, 0x91, 0x6b, 0x6c, 0x17, 0x19, 0xe3, 0xdb, 0x58)]
interface ISpLexicon(ISpLexiconVtbl): IUnknown(IUnknownVtbl) {
    fn GetPronunciations(
        pszWord: ::LPCWSTR, LangID: ::WORD, dwFlags: ::DWORD,
        pWordPronunciationList: *mut SPWORDPRONUNCIATIONLIST
    ) -> ::HRESULT,
    fn AddPronunciation(
        pszWord: ::LPCWSTR, LangID: ::WORD, ePartOfSpeech: SPPARTOFSPEECH,
        pszPronunciation: PCSPPHONEID
    ) -> ::HRESULT,
    fn RemovePronunciation(
        pszWord: ::LPCWSTR, LangID: ::WORD, ePartOfSpeech: SPPARTOFSPEECH,
        pszPronunciation: PCSPPHONEID
    ) -> ::HRESULT,
    fn GetGeneration(pdwGeneration: *mut ::DWORD) -> ::HRESULT,
    fn GetGenerationChange(
        dwFlags: ::DWORD, pdwGeneration: *mut ::DWORD, pWordList: *mut SPWORDLIST
    ) -> ::HRESULT,
    fn GetWords(
        dwFlags: ::DWORD, pdwGeneration: *mut ::DWORD, pdwCookie: *mut ::DWORD,
        pWordList: *mut SPWORDLIST
    ) -> ::HRESULT
}
);
RIDL!(
#[uuid(0x8565572f, 0xc094, 0x41cc, 0xb5, 0x6e, 0x10, 0xbd, 0x9c, 0x3f, 0xf0, 0x44)]
interface ISpContainerLexicon(ISpContainerLexiconVtbl): ISpLexicon(ISpLexiconVtbl) {
    fn AddLexicon(pAddLexicon: *mut ISpLexicon, dwFlags: ::DWORD) -> ::HRESULT
}
);
ENUM!{enum SPSHORTCUTTYPE {
    SPSHT_NotOverriden = -1i32 as u32,
    SPSHT_Unknown = 0,
    SPSHT_EMAIL = 0x1000,
    SPSHT_OTHER = 0x2000,
    SPPS_RESERVED1 = 0x3000,
    SPPS_RESERVED2 = 0x4000,
    SPPS_RESERVED3 = 0x5000,
    SPPS_RESERVED4 = 0xf000,
}}
STRUCT!{struct SPSHORTCUTPAIR {
    pNextSHORTCUTPAIR: *mut SPSHORTCUTPAIR,
    LangID: ::WORD,
    shType: SPSHORTCUTTYPE,
    pszDisplay: ::LPWSTR,
    pszSpoken: ::LPWSTR,
}}
STRUCT!{struct SPSHORTCUTPAIRLIST {
    ulSize: ::ULONG,
    pvBuffer: *mut ::BYTE,
    pFirstShortcutPair: *mut SPSHORTCUTPAIR,
}}
RIDL!(
#[uuid(0x3df681e2, 0xea56, 0x11d9, 0x8b, 0xde, 0xf6, 0x6b, 0xad, 0x1e, 0x3f, 0x3a)]
interface ISpShortcut(ISpShortcutVtbl): IUnknown(IUnknownVtbl) {
    fn AddShortcut(
        pszDisplay: ::LPCWSTR, LangID: ::WORD, pszSpoken: ::LPCWSTR,
        shType: SPSHORTCUTTYPE
    ) -> ::HRESULT,
    fn RemoveShortcut(
        pszDisplay: ::LPCWSTR, LangID: ::WORD, pszSpoken: ::LPCWSTR,
        shType: SPSHORTCUTTYPE
    ) -> ::HRESULT,
    fn GetShortcuts(
        LangId: ::WORD, pShortcutpairList: *mut SPSHORTCUTPAIRLIST
    ) -> ::HRESULT,
    fn GetGeneration(pdwGeneration: *mut ::DWORD) -> ::HRESULT,
    fn GetWordsFromGenerationChange(
        pdwGeneration: *mut ::DWORD, pWordList: *mut SPWORDLIST
    ) -> ::HRESULT,
    fn GetWords(
        pdwGeneration: *mut ::DWORD, pdwCookie: *mut ::DWORD, pWordList: *mut SPWORDLIST
    ) -> ::HRESULT,
    fn GetShortcutsForGeneration(
        pdwGeneration: *mut ::DWORD, pdwCookie: *mut ::DWORD,
        pShortcutpairList: *mut SPSHORTCUTPAIRLIST
    ) -> ::HRESULT,
    fn GetGenerationChange(
        pdwGeneration: *mut ::DWORD, pShortcutpairList: *mut SPSHORTCUTPAIRLIST
    ) -> ::HRESULT
}
);
RIDL!(
#[uuid(0x8445c581, 0x0cac, 0x4a38, 0xab, 0xfe, 0x9b, 0x2c, 0xe2, 0x82, 0x64, 0x55)]
interface ISpPhoneConverter(ISpPhoneConverterVtbl): ISpObjectWithToken(ISpObjectWithTokenVtbl) {
    fn PhoneToId(pszPhone: ::LPCWSTR, pId: *mut SPPHONEID) -> ::HRESULT,
    fn IdToPhone(pId: PCSPPHONEID, pszPhone: *mut ::WCHAR) -> ::HRESULT
}
);
RIDL!(
#[uuid(0x133adcd4, 0x19b4, 0x4020, 0x9f, 0xdc, 0x84, 0x2e, 0x78, 0x25, 0x3b, 0x17)]
interface ISpPhoneticAlphabetConverter(ISpPhoneticAlphabetConverterVtbl): IUnknown(IUnknownVtbl) {
    fn GetLangId(pLangID: *mut ::WORD) -> ::HRESULT,
    fn SetLangId(LangID: *mut ::WORD) -> ::HRESULT,
    fn SAPI2UPS(
        pszSAPIId: *const SPPHONEID, pszUPSId: *mut SPPHONEID, cMaxLength: ::DWORD
    ) -> ::HRESULT,
    fn UPS2SAPI(
        pszUPSId: *const SPPHONEID, pszSAPIId: *mut SPPHONEID, cMaxLength: ::DWORD
    ) -> ::HRESULT,
    fn GetMaxConvertLength(
        cSrcLength: ::DWORD, bSAPI2UPS: ::BOOL, pcMaxDestLength: *mut ::DWORD
    ) -> ::HRESULT
}
);
RIDL!(
#[uuid(0xb2745efd, 0x42ce, 0x48ca, 0x81, 0xf1, 0xa9, 0x6e, 0x02, 0x53, 0x8a, 0x90)]
interface ISpPhoneticAlphabetSelection(ISpPhoneticAlphabetSelectionVtbl): IUnknown(IUnknownVtbl) {
    fn IsAlphabetUPS(pfIsUPS: *mut ::BOOL) -> ::HRESULT,
    fn SetAlphabetToUPS(fForceUPS: ::BOOL) -> ::HRESULT
}
);
STRUCT!{struct SPVPITCH {
    MiddleAdj: ::c_long,
    RangeAdj: ::c_long,
}}
ENUM!{enum SPVACTIONS {
    SPVA_Speak = 0,
    SPVA_Silence,
    SPVA_Pronounce,
    SPVA_Bookmark,
    SPVA_SpellOut,
    SPVA_Section,
    SPVA_ParseUnknownTag,
}}
STRUCT!{struct SPVCONTEXT {
    pCategory: ::LPCWSTR,
    pBefore: ::LPCWSTR,
    pAfter: ::LPCWSTR,
}}
STRUCT!{struct SPVSTATE {
    eAction: SPVACTIONS,
    LangID: ::WORD,
    wReserved: ::WORD,
    EmphAdj: ::c_long,
    RateAdj: ::c_long,
    Volume: ::ULONG,
    PitchAdj: SPVPITCH,
    SilenceMSecs: ::ULONG,
    pPhoneIds: *mut SPPHONEID,
    ePartOfSpeech: SPPARTOFSPEECH,
    Context: SPVCONTEXT,
}}
ENUM!{enum SPRUNSTATE {
    SPRS_DONE = 1 << 0,
    SPRS_IS_SPEAKING = 1 << 1,
}}
ENUM!{enum SPVLIMITS {
    SPMIN_VOLUME = 0,
    SPMAX_VOLUME = 100,
    SPMIN_RATE = -10i32 as u32,
    SPMAX_RATE = 10,
}}
ENUM!{enum SPVPRIORITY {
    SPVPRI_NORMAL = 0,
    SPVPRI_ALERT = 1 << 0,
    SPVPRI_OVER = 1 << 1,
}}
STRUCT!{struct SPVOICESTATUS {
    ulCurrentStream: ::ULONG,
    ulLastStreamQueued: ::ULONG,
    hrLastResult: ::HRESULT,
    dwRunningState: ::DWORD,
    ulInputWordPos: ::ULONG,
    ulInputWordLen: ::ULONG,
    ulInputSentPos: ::ULONG,
    ulInputSentLen: ::ULONG,
    lBookmarkId: ::LONG,
    PhonemeId: SPPHONEID,
    VisemeId: SPVISEMES,
    dwReserved1: ::DWORD,
    dwReserved2: ::DWORD,
}}
ENUM!{enum SPEAKFLAGS {
    SPF_DEFAULT = 0,
    SPF_ASYNC = 1 << 0,
    SPF_PURGEBEFORESPEAK = 1 << 1,
    SPF_IS_FILENAME = 1 << 2,
    SPF_IS_XML = 1 << 3,
    SPF_IS_NOT_XML = 1 << 4,
    SPF_PERSIST_XML = 1 << 5,
    SPF_NLP_SPEAK_PUNC = 1 << 6,
    SPF_PARSE_SAPI = 1 << 7,
    SPF_PARSE_SSML = 1 << 8,
}}
pub const SPF_PARSE_AUTODETECT: SPEAKFLAGS = SPF_DEFAULT;
pub const SPF_NLP_MASK: SPEAKFLAGS = SPF_NLP_SPEAK_PUNC;
pub const SPF_PARSE_MASK: i32 = SPF_PARSE_SAPI as i32 | SPF_PARSE_SSML as i32;
pub const SPF_VOICE_MASK: i32 =
    SPF_ASYNC as i32 | SPF_PURGEBEFORESPEAK as i32 | SPF_IS_FILENAME as i32 | SPF_IS_XML as i32 |
    SPF_IS_NOT_XML as i32 | SPF_NLP_MASK as i32 | SPF_PERSIST_XML as i32 | SPF_PARSE_MASK;
pub const SPF_UNUSED_FLAGS: i32 = !SPF_VOICE_MASK;
RIDL!(
#[uuid(0x6c44df74, 0x72b9, 0x4992, 0xa1, 0xec, 0xef, 0x99, 0x6e, 0x04, 0x22, 0xd4)]
interface ISpVoice(ISpVoiceVtbl): ISpEventSource(ISpEventSourceVtbl) {
    fn SetOutput(pUnkOutput: *mut ::IUnknown, fAllowFormatChanges: ::BOOL) -> ::HRESULT,
    fn GetOutputObjectToken(ppObjectToken: *mut *mut ISpObjectToken) -> ::HRESULT,
    fn GetOutputStream(ppStream: *mut *mut ISpStreamFormat) -> ::HRESULT,
    fn Pause() -> ::HRESULT,
    fn Resume() -> ::HRESULT,
    fn SetVoice(pToken: *mut ISpObjectToken) -> ::HRESULT,
    fn GetVoice(ppToken: *mut *mut ISpObjectToken) -> ::HRESULT,
    fn Speak(
        pwcs: ::LPCWSTR, dwFlags: ::DWORD, pulStreamNumber: *mut ::ULONG
    ) -> ::HRESULT,
    fn SpeakStream(
        pStream: *mut ::IStream, dwFlags: ::DWORD, pulStreamNumber: *mut ::ULONG
    ) -> ::HRESULT,
    fn GetStatus(
        pStatus: *mut SPVOICESTATUS, ppszLastBookmark: *mut ::LPWSTR
    ) -> ::HRESULT,
    fn Skip(
        pItemType: ::LPCWSTR, lNumItems: ::c_long, pulNumSkipped: *mut ::ULONG
    ) -> ::HRESULT,
    fn SetPriority(ePriority: SPVPRIORITY) -> ::HRESULT,
    fn GetPriority(pePriority: *mut SPVPRIORITY) -> ::HRESULT,
    fn SetAlertBoundary(eBoundary: SPEVENTENUM) -> ::HRESULT,
    fn GetAlertBoundary(peBoundary: *mut SPEVENTENUM) -> ::HRESULT,
    fn SetRate(RateAdjust: ::c_long) -> ::HRESULT,
    fn GetRate(pRateAdjust: *mut ::c_long) -> ::HRESULT,
    fn SetVolume(usVolume: ::USHORT) -> ::HRESULT,
    fn GetVolume(pusVolume: *mut ::USHORT) -> ::HRESULT,
    fn WaitUntilDone(msTimeout: ::ULONG) -> ::HRESULT,
    fn SetSyncSpeakTimeout(msTimeout: ::ULONG) -> ::HRESULT,
    fn GetSyncSpeakTimeout(pmsTimeout: *mut ::ULONG) -> ::HRESULT,
    fn SpeakCompleteEvent() -> ::HANDLE,
    fn IsUISupported(
        pszTypeOfUI: ::LPCWSTR, pvExtraData: *mut ::c_void, cbExtraData: ::ULONG,
        pfSupported: *mut ::BOOL
    ) -> ::HRESULT,
    fn DisplayUI(
        hwndParent: ::HWND, pszTitle: ::LPCWSTR, pszTypeOfUI: ::LPCWSTR,
        pvExtraData: *mut ::c_void, cbExtraData: ::ULONG
    ) -> ::HRESULT
}
);
DEFINE_GUID!(
    UuidOfISpVoice,
    0x6C44DF74, 0x72B9, 0x4992, 0xA1, 0xEC, 0xEF, 0x99, 0x6E, 0x04, 0x22, 0xD4
);
RIDL!(
#[uuid(0x1a5c0354, 0xb621, 0x4b5a, 0x87, 0x91, 0xd3, 0x06, 0xed, 0x37, 0x9e, 0x53)]
interface ISpPhrase(ISpPhraseVtbl): IUnknown(IUnknownVtbl) {
    fn GetPhrase(ppCoMemPhrase: *mut *mut SPPHRASE) -> ::HRESULT,
    fn GetSerializedPhrase(ppCoMemPhrase: *mut *mut SPSERIALIZEDPHRASE) -> ::HRESULT,
    fn GetText(
        ulStart: ::ULONG, ulCount: ::ULONG, fUseTextReplacements: ::BOOL,
        ppszCoMemText: *mut ::LPWSTR, pbDisplayAttributes: *mut ::BYTE
    ) -> ::HRESULT,
    fn Discard(dwValueTypes: ::DWORD) -> ::HRESULT
}
);
RIDL!(
#[uuid(0x8fcebc98, 0x4e49, 0x4067, 0x9c, 0x6c, 0xd8, 0x6a, 0x0e, 0x09, 0x2e, 0x3d)]
interface ISpPhraseAlt(ISpPhraseAltVtbl): ISpPhrase(ISpPhraseVtbl) {
    fn GetAltInfo(
        pParent: *mut *mut ISpPhrase, pulStartElementInParent: *mut ::ULONG,
        pcElementsInParent: *mut ::ULONG, pcElementsInAlt: *mut ::ULONG
    ) -> ::HRESULT,
    fn Commit() -> ::HRESULT
}
);
ENUM!{enum SPXMLRESULTOPTIONS {
    SPXRO_SML = 0,
    SPXRO_Alternates_SML = 1,
}}
RIDL!(
#[uuid(0xf264da52, 0xe457, 0x4696, 0xb8, 0x56, 0xa7, 0x37, 0xb7, 0x17, 0xaf, 0x79)]
interface ISpPhrase2(ISpPhrase2Vtbl): ISpPhrase(ISpPhraseVtbl) {
    fn GetXMLResult(
        ppszCoMemXMLResult: *mut ::LPWSTR, Options: SPXMLRESULTOPTIONS
    ) -> ::HRESULT,
    fn GetXMLErrorInfo(pSemanticErrorInfo: *mut SPSEMANTICERRORINFO) -> ::HRESULT,
    fn GetAudio(
        ulStartElement: ::ULONG, cElements: ::ULONG, ppStream: *mut *mut ISpStreamFormat
    ) -> ::HRESULT
}
);
STRUCT!{struct SPRECORESULTTIMES {
    ftStreamTime: ::FILETIME,
    ullLength: ::ULONGLONG,
    dwTickCount: ::DWORD,
    ullStart: ::ULONGLONG,
}}
STRUCT!{struct SPSERIALIZEDRESULT {
    ulSerializedSize: ::ULONG,
}}
RIDL!(
#[uuid(0x20b053be, 0xe235, 0x43cd, 0x9a, 0x2a, 0x8d, 0x17, 0xa4, 0x8b, 0x78, 0x42)]
interface ISpRecoResult(ISpRecoResultVtbl): ISpPhrase(ISpPhraseVtbl) {
    fn GetResultTimes(pTimes: *mut SPRECORESULTTIMES) -> ::HRESULT,
    fn GetAlternates(
        ulStartElement: ::ULONG, cElements: ::ULONG, ulRequestCount: ::ULONG,
        ppPhrases: *mut *mut ISpPhraseAlt, pcPhrasesReturned: *mut ::ULONG
    ) -> ::HRESULT,
    fn GetAudio(
        ulStartElement: ::ULONG, cElements: ::ULONG, ppStream: *mut *mut ISpStreamFormat
    ) -> ::HRESULT,
    fn SpeakAudio(
        ulStartElement: ::ULONG, cElements: ::ULONG, dwFlags: ::DWORD,
        pulStreamNumber: *mut ::ULONG
    ) -> ::HRESULT,
    fn Serialize(ppCoMemSerializedResult: *mut *mut SPSERIALIZEDRESULT) -> ::HRESULT,
    fn ScaleAudio(
        pAudioFormatId: *const ::GUID, pWaveFormatEx: *const ::WAVEFORMATEX
    ) -> ::HRESULT,
    fn GetRecoContext(ppRecoContext: *mut *mut ISpRecoContext) -> ::HRESULT
}
);
ENUM!{enum SPCOMMITFLAGS {
    SPCF_NONE = 0,
    SPCF_ADD_TO_USER_LEXICON = 1 << 0,
    SPCF_DEFINITE_CORRECTION = 1 << 1,
}}
RIDL!(
#[uuid(0x27cac6c4, 0x88f2, 0x41f2, 0x88, 0x17, 0x0c, 0x95, 0xe5, 0x9f, 0x1e, 0x6e)]
interface ISpRecoResult2(ISpRecoResult2Vtbl): ISpRecoResult(ISpRecoResultVtbl) {
    fn CommitAlternate(
        pPhraseAlt: *mut ISpPhraseAlt, ppNewResult: *mut *mut ISpRecoResult
    ) -> ::HRESULT,
    fn CommitText(
        ulStartElement: ::ULONG, cElements: ::ULONG, pszCorrectedData: ::LPCWSTR,
        eCommitFlags: ::DWORD
    ) -> ::HRESULT,
    fn SetTextFeedback(pszFeedback: ::LPCWSTR, fSuccessful: ::BOOL) -> ::HRESULT
}
);
RIDL!(
#[uuid(0xae39362b, 0x45a8, 0x4074, 0x9b, 0x9e, 0xcc, 0xf4, 0x9a, 0xa2, 0xd0, 0xb6)]
interface ISpXMLRecoResult(ISpXMLRecoResultVtbl): ISpRecoResult(ISpRecoResultVtbl) {
    fn GetXMLResult(
        ppszCoMemXMLResult: *mut ::LPWSTR, Options: SPXMLRESULTOPTIONS
    ) -> ::HRESULT,
    fn GetXMLErrorInfo(pSemanticErrorInfo: *mut SPSEMANTICERRORINFO) -> ::HRESULT
}
);
STRUCT!{struct SPTEXTSELECTIONINFO {
    ulStartActiveOffset: ::ULONG,
    cchActiveChars: ::ULONG,
    ulStartSelection: ::ULONG,
    cchSelection: ::ULONG,
}}
ENUM!{enum SPWORDPRONOUNCEABLE {
    SPWP_UNKNOWN_WORD_UNPRONOUNCEABLE = 0,
    SPWP_UNKNOWN_WORD_PRONOUNCEABLE = 1,
    SPWP_KNOWN_WORD_PRONOUNCEABLE = 2,
}}
ENUM!{enum SPGRAMMARSTATE {
    SPGS_DISABLED = 0,
    SPGS_ENABLED = 1,
    SPGS_EXCLUSIVE = 3,
}}
ENUM!{enum SPCONTEXTSTATE {
    SPCS_DISABLED = 0,
    SPCS_ENABLED = 1,
}}
ENUM!{enum SPRULESTATE {
    SPRS_INACTIVE = 0,
    SPRS_ACTIVE = 1,
    SPRS_ACTIVE_WITH_AUTO_PAUSE = 3,
    SPRS_ACTIVE_USER_DELIMITED = 4,
}}
pub const SP_STREAMPOS_ASAP: ::INT = 0;
pub const SP_STREAMPOS_REALTIME: ::INT = -1;
pub const SPRULETRANS_TEXTBUFFER: SPSTATEHANDLE = -1isize as SPSTATEHANDLE;
pub const SPRULETRANS_WILDCARD: SPSTATEHANDLE = -2isize as SPSTATEHANDLE;
pub const SPRULETRANS_DICTATION: SPSTATEHANDLE = -3isize as SPSTATEHANDLE;
ENUM!{enum SPGRAMMARWORDTYPE {
    SPWT_DISPLAY = 0,
    SPWT_LEXICAL = 1,
    SPWT_PRONUNCIATION = 2,
    SPWT_LEXICAL_NO_SPECIAL_CHARS = 3,
}}
STRUCT!{struct SPPROPERTYINFO {
    pszName: ::LPCWSTR,
    ulId: ::ULONG,
    pszValue: ::LPCWSTR,
    vValue: ::VARIANT,
}}
ENUM!{enum SPCFGRULEATTRIBUTES {
    SPRAF_TopLevel = 1 << 0,
    SPRAF_Active = 1 << 1,
    SPRAF_Export = 1 << 2,
    SPRAF_Import = 1 << 3,
    SPRAF_Interpreter = 1 << 4,
    SPRAF_Dynamic = 1 << 5,
    SPRAF_Root = 1 << 6,
    SPRAF_AutoPause = 1 << 16,
    SPRAF_UserDelimited = 1 << 17,
}}
RIDL!(
#[uuid(0x8137828f, 0x591a, 0x4a42, 0xbe, 0x58, 0x49, 0xea, 0x7e, 0xba, 0xac, 0x68)]
interface ISpGrammarBuilder(ISpGrammarBuilderVtbl): IUnknown(IUnknownVtbl) {
    fn ResetGrammar(NewLanguage: ::WORD) -> ::HRESULT,
    fn GetRule(
        pszRuleName: ::LPCWSTR, dwRuleId: ::DWORD, dwAttributes: ::DWORD,
        fCreateIfNotExist: ::BOOL, phInitialState: *mut SPSTATEHANDLE
    ) -> ::HRESULT,
    fn ClearRule(hState: SPSTATEHANDLE) -> ::HRESULT,
    fn CreateNewState(hState: SPSTATEHANDLE, phState: *mut SPSTATEHANDLE) -> ::HRESULT,
    fn AddWordTransition(
        hFromState: SPSTATEHANDLE, hToState: SPSTATEHANDLE, psz: ::LPCWSTR,
        pszSeparators: ::LPCWSTR, eWordType: SPGRAMMARWORDTYPE, Weight: ::c_float,
        pPropInfo: *const SPPROPERTYINFO
    ) -> ::HRESULT,
    fn AddRuleTransition(
        hFromState: SPSTATEHANDLE, hToState: SPSTATEHANDLE, hRule: SPSTATEHANDLE,
        Weight: ::c_float, pPropInfo: *const SPPROPERTYINFO
    ) -> ::HRESULT,
    fn AddResource(
        hRuleState: SPSTATEHANDLE, pszResourceName: ::LPCWSTR,
        pszResourceValue: ::LPCWSTR
    ) -> ::HRESULT,
    fn Commit(dwReserved: ::DWORD) -> ::HRESULT
}
);
ENUM!{enum SPLOADOPTIONS {
    SPLO_STATIC = 0,
    SPLO_DYNAMIC = 1,
}}
RIDL!(
#[uuid(0x2177db29, 0x7f45, 0x47d0, 0x85, 0x54, 0x06, 0x7e, 0x91, 0xc8, 0x05, 0x02)]
interface ISpRecoGrammar(ISpRecoGrammarVtbl): ISpGrammarBuilder(ISpGrammarBuilderVtbl) {
    fn GetGrammarId(pullGrammarId: *mut ::ULONGLONG) -> ::HRESULT,
    fn GetRecoContext(ppRecoCtxt: *mut *mut ISpRecoContext) -> ::HRESULT,
    fn LoadCmdFromFile(pszFileName: ::LPCWSTR, Options: SPLOADOPTIONS) -> ::HRESULT,
    fn LoadCmdFromObject(
        rcid: ::REFCLSID, pszGrammarName: ::LPCWSTR, Options: SPLOADOPTIONS
    ) -> ::HRESULT,
    fn LoadCmdFromResource(
        hModule: ::HMODULE, pszResourceName: ::LPCWSTR, pszResourceType: ::LPCWSTR,
        wLanguage: ::WORD, Options: SPLOADOPTIONS
    ) -> ::HRESULT,
    fn LoadCmdFromMemory(
        pGrammar: *const SPBINARYGRAMMAR, Options: SPLOADOPTIONS
    ) -> ::HRESULT,
    fn LoadCmdFromProprietaryGrammar(
        rguidParam: ::REFGUID, pszStringParam: ::LPCWSTR, pvDataPrarm: *const ::c_void,
        cbDataSize: ::ULONG, Options: SPLOADOPTIONS
    ) -> ::HRESULT,
    fn SetRuleState(
        pszName: ::LPCWSTR, pReserved: *mut ::c_void, NewState: SPRULESTATE
    ) -> ::HRESULT,
    fn SetRuleIdState(ulRuleId: ::ULONG, NewState: SPRULESTATE) -> ::HRESULT,
    fn LoadDictation(pszTopicName: ::LPCWSTR, Options: SPLOADOPTIONS) -> ::HRESULT,
    fn UnloadDictation() -> ::HRESULT,
    fn SetDictationState(NewState: SPRULESTATE) -> ::HRESULT,
    fn SetWordSequenceData(
        pText: *const ::WCHAR, cchText: ::ULONG, pInfo: *const SPTEXTSELECTIONINFO
    ) -> ::HRESULT,
    fn SetTextSelection(pInfo: *const SPTEXTSELECTIONINFO) -> ::HRESULT,
    fn IsPronounceable(
        pszWord: ::LPCWSTR, pWordPronounceable: *mut SPWORDPRONOUNCEABLE
    ) -> ::HRESULT,
    fn SetGrammarState(eGrammarState: SPGRAMMARSTATE) -> ::HRESULT,
    fn SaveCmd(pStream: *mut ::IStream, ppszCoMemErrorText: *mut ::LPWSTR) -> ::HRESULT,
    fn GetGrammarState(peGrammarState: *mut SPGRAMMARSTATE) -> ::HRESULT
}
);
ENUM!{enum SPMATCHINGMODE {
    AllWords = 0,
    Subsequence = 1,
    OrderedSubset = 3,
    SubsequenceContentRequired = 5,
    OrderedSubsetContentRequired = 7,
}}
ENUM!{enum PHONETICALPHABET {
    PA_Ipa = 0,
    PA_Ups = 1,
    PA_Sapi = 2,
}}
RIDL!(
#[uuid(0x8ab10026, 0x20cc, 0x4b20, 0x8c, 0x22, 0xa4, 0x9c, 0x9b, 0xa7, 0x8f, 0x60)]
interface ISpGrammarBuilder2(ISpGrammarBuilder2Vtbl): IUnknown(IUnknownVtbl) {
    fn AddTextSubset(
        hFromState: SPSTATEHANDLE, hToState: SPSTATEHANDLE, psz: ::LPCWSTR,
        eMatchMode: SPMATCHINGMODE
    ) -> ::HRESULT,
    fn SetPhoneticAlphabet(phoneticALphabet: PHONETICALPHABET) -> ::HRESULT
}
);
RIDL!(
#[uuid(0x4b37bc9e, 0x9ed6, 0x44a3, 0x93, 0xd3, 0x18, 0xf0, 0x22, 0xb7, 0x9e, 0xc3)]
interface ISpRecoGrammar2(ISpRecoGrammar2Vtbl): IUnknown(IUnknownVtbl) {
    fn GetRules(ppCoMemRules: *mut *mut SPRULE, puNumRules: *mut ::UINT) -> ::HRESULT,
    fn LoadCmdFromFile2(
        pszFileName: ::LPCWSTR, Options: SPLOADOPTIONS, pszSharingUri: ::LPCWSTR,
        pszBaseUri: ::LPCWSTR
    ) -> ::HRESULT,
    fn LoadCmdFromMemory2(
        pGrammar: *const SPBINARYGRAMMAR, Options: SPLOADOPTIONS,
        pszSharingUri: ::LPCWSTR, pszBaseUri: ::LPCWSTR
    ) -> ::HRESULT,
    fn SetRulePriority(
        pszRuleName: ::LPCWSTR, ulRuleId: ::ULONG, nRulePriority: ::c_int
    ) -> ::HRESULT,
    fn SetRuleWeight(
        pszRuleName: ::LPCWSTR, ulRuleId: ::ULONG, flWeight: ::c_float
    ) -> ::HRESULT,
    fn SetDictationWeight(flWeight: ::c_float) -> ::HRESULT,
    fn SetGrammarLoader(pLoader: *mut ISpeechResourceLoader) -> ::HRESULT,
    fn SetSMLSecurityManager(
        pSMLSecurityManager: *mut ::IInternetSecurityManager
    ) -> ::HRESULT
}
);
RIDL!(
#[uuid(0xb9ac5783, 0xfcd0, 0x4b21, 0xb1, 0x19, 0xb4, 0xf8, 0xda, 0x8f, 0xd2, 0xc3)]
interface ISpeechResourceLoader(ISpeechResourceLoaderVtbl): IDispatch(IDispatchVtbl) {
    fn LoadResource(
        bstrResourceUri: ::BSTR, fAlwaysReload: ::VARIANT_BOOL,
        pStream: *mut *mut ::IUnknown, pbstrMIMEType: *mut ::BSTR, pfModified: *mut ::VARIANT_BOOL,
        pbstrRedirectUrl: *mut ::BSTR
    ) -> ::HRESULT,
    fn GetLocalCopy(
        bstrResourceUri: ::BSTR, pbstrLocalPath: *mut ::BSTR,
        pbstrMIMEType: *mut ::BSTR, pbstrRedirectUrl: *mut ::BSTR
    ) -> ::HRESULT,
    fn ReleaseLocalCopy(pbstrLocalPath: ::BSTR) -> ::HRESULT
}
);
STRUCT!{struct SPRECOCONTEXTSTATUS {
    eInterference: SPINTERFERENCE,
    szRequestTypeOfUI: [::WCHAR; 255],
    dwReserved1: ::DWORD,
    dwReserved2: ::DWORD,
}}
ENUM!{enum SPBOOKMARKOPTIONS {
    SPBO_NONE = 0,
    SPBO_PAUSE = 1 << 0,
    SPBO_AHEAD = 1 << 1,
    SPBO_TIME_UNITS = 1 << 2,
}}
ENUM!{enum SPAUDIOOPTIONS {
    SPAO_NONE = 0,
    SPAO_RETAIN_AUDIO = 1 << 0,
}}
RIDL!(
#[uuid(0xf740a62f, 0x7c15, 0x489e, 0x82, 0x34, 0x94, 0x0a, 0x33, 0xd9, 0x27, 0x2d)]
interface ISpRecoContext(ISpRecoContextVtbl): ISpEventSource(ISpEventSourceVtbl) {
    fn GetRecognizer(ppRecognizer: *mut *mut ISpRecognizer) -> ::HRESULT,
    fn CreateGrammer(
        ullGrammarId: ::ULONGLONG, ppGrammar: *mut *mut ISpRecoGrammar
    ) -> ::HRESULT,
    fn GetStatus(pState: *mut SPRECOCONTEXTSTATUS) -> ::HRESULT,
    fn GetMaxAlternates(pcAlternates: *mut ::ULONG) -> ::HRESULT,
    fn SetMaxAlternates(cAlternates: ::ULONG) -> ::HRESULT,
    fn SetAudioOptions(
        Options: SPAUDIOOPTIONS, pAudioFormatId: *const ::GUID,
        pWaveFormatEx: *const ::WAVEFORMATEX
    ) -> ::HRESULT,
    fn GetAudioOptions(
        pOptions: *mut SPAUDIOOPTIONS, pAudioFormatId: *mut ::GUID,
        ppCoMemWFEX: *mut *mut ::WAVEFORMATEX
    ) -> ::HRESULT,
    fn DeserializeResult(
        pSerializedResult: *const SPSERIALIZEDRESULT, ppResult: *mut *mut ISpRecoResult
    ) -> ::HRESULT,
    fn Bookmark(
        Options: SPBOOKMARKOPTIONS, ullStreamPosition: ::ULONGLONG,
        lparamEvent: ::LPARAM
    ) -> ::HRESULT,
    fn SetAdaptionData(pAdaptionData: ::LPCWSTR, cch: ::ULONG) -> ::HRESULT,
    fn Pause(dwReserved: ::DWORD) -> ::HRESULT,
    fn Resume(dwReserved: ::DWORD) -> ::HRESULT,
    fn SetVoice(pVoice: *mut ISpVoice, fAllowFormatChanges: ::BOOL) -> ::HRESULT,
    fn GetVoice(ppVoice: *mut *mut ISpVoice) -> ::HRESULT,
    fn SetVoicePurgeEvent(ullEventIntereset: ::ULONGLONG) -> ::HRESULT,
    fn GetVoicePurgeEvent(pullEventIntereset: *mut ::ULONGLONG) -> ::HRESULT,
    fn SetContextState(eContextState: SPCONTEXTSTATE) -> ::HRESULT,
    fn GetContextState(peContextState: *mut SPCONTEXTSTATE) -> ::HRESULT
}
);
ENUM!{enum SPGRAMMAROPTIONS {
    SPGO_SAPI = 0x1,
    SPGO_SRGS = 0x2,
    SPGO_UPS = 0x4,
    SPGO_SRGS_MS_SCRIPT = 0x8,
    SPGO_SRGS_W3C_SCRIPT = 0x100,
    SPGO_SRGS_STG_SCRIPT = 0x200,
    SPGO_SRGS_SCRIPT =
        SPGO_SRGS | SPGO_SRGS_MS_SCRIPT | SPGO_SRGS_W3C_SCRIPT |
             SPGO_SRGS_STG_SCRIPT,
    SPGO_FILE = 0x10,
    SPGO_HTTP = 0x20,
    SPGO_RES = 0x40,
    SPGO_OBJECT = 0x80,
    SPGO_DEFAULT = 0x3fb,
    SPGO_ALL = 0x3ff,
}}
ENUM!{enum SPADAPTATIONSETTINGS {
    SPADS_Default = 0,
    SPADS_CurrentRecognizer = 0x1,
    SPADS_RecoProfile = 0x2,
    SPADS_Immediate = 0x4,
    SPADS_Reset = 0x8,
    SPADS_HighVolumeDataSource = 0x10,
}}
ENUM!{enum SPADAPTATIONRELEVANCE {
    SPAR_Unknown = 0,
    SPAR_Low = 1,
    SPAR_Medium = 2,
    SPAR_High = 3,
}}
RIDL!(
#[uuid(0xbead311c, 0x52ff, 0x437f, 0x94, 0x64, 0x6b, 0x21, 0x05, 0x4c, 0xa7, 0x3d)]
interface ISpRecoContext2(ISpRecoContext2Vtbl): IUnknown(IUnknownVtbl) {
    fn SetGrammarOptions(eGrammarOptions: ::DWORD) -> ::HRESULT,
    fn GetGrammarOptions(peGrammarOptions: *mut ::DWORD) -> ::HRESULT,
    fn SetAdaptationData2(
        pAdaptationData: ::LPCWSTR, cch: ::ULONG, pTopicName: ::LPCWSTR,
        eAdaptationSettings: ::DWORD, eRelevance: SPADAPTATIONRELEVANCE
    ) -> ::HRESULT
}
);
RIDL!(
#[uuid(0x5b4fb971, 0xb115, 0x4de1, 0xad, 0x97, 0xe4, 0x82, 0xe3, 0xbf, 0x6e, 0xe4)]
interface ISpProperties(ISpPropertiesVtbl): IUnknown(IUnknownVtbl) {
    fn SetPropertyNum(pName: ::LPCWSTR, lValue: ::LONG) -> ::HRESULT,
    fn GetPropertyNum(pName: ::LPCWSTR, plValue: *mut ::LONG) -> ::HRESULT,
    fn SetPropertyString(pName: ::LPCWSTR, pValue: ::LPCWSTR) -> ::HRESULT,
    fn GetPropertyString(pName: ::LPCWSTR, ppCoMemValue: *mut ::LPWSTR) -> ::HRESULT
}
);
STRUCT!{struct SPRECOGNIZERSTATUS {
    AudioStatus: SPAUDIOSTATUS,
    ullRecognitionStreamPos: ::ULONGLONG,
    ulStreamNumber: ::ULONG,
    ulNumActive: ::ULONG,
    clsidEngine: ::CLSID,
    cLangIDs: ::ULONG,
    aLangID: [::WORD; 20],
    ullRecognitionStreamTime: ::ULONGLONG,
}}
ENUM!{enum SPWAVEFORMATTYPE {
    SPWF_INPUT = 0,
    SPWF_SRENGINE = 1,
}}
pub type SPSTREAMFORMATTYPE = SPWAVEFORMATTYPE;
ENUM!{enum SPRECOSTATE {
    SPRST_INACTIVE = 0,
    SPRST_ACTIVE = 1,
    SPRST_ACTIVE_ALWAYS = 2,
    SPRST_INACTIVE_WITH_PURGE = 3,
    SPRST_NUM_STATES = 4,
}}
RIDL!(
#[uuid(0xc2b5f241, 0xdaa0, 0x4507, 0x9e, 0x16, 0x5a, 0x1e, 0xaa, 0x2b, 0x7a, 0x5c)]
interface ISpRecognizer(ISpRecognizerVtbl): ISpProperties(ISpPropertiesVtbl) {
    fn SetRecognizer(pRecognizer: *mut ISpObjectToken) -> ::HRESULT,
    fn GetRecognizer(ppRecognizer: *mut *mut ISpObjectToken) -> ::HRESULT,
    fn SetInput(pUnkInput: *mut ::IUnknown, fAllowFormatChanges: ::BOOL) -> ::HRESULT,
    fn GetInputObjectToken(ppToken: *mut *mut ISpObjectToken) -> ::HRESULT,
    fn GetInputStream(ppStream: *mut *mut ISpStreamFormat) -> ::HRESULT,
    fn CreateRecoContext(ppNewCtxt: *mut *mut ISpRecoContext) -> ::HRESULT,
    fn GetRecoProfile(ppToken: *mut *mut ISpObjectToken) -> ::HRESULT,
    fn SetRecoProfile(pToken: *mut ISpObjectToken) -> ::HRESULT,
    fn IsSharedInstance() -> ::HRESULT,
    fn GetRecoState(pState: *mut SPRECOSTATE) -> ::HRESULT,
    fn SetRecoState(NewState: SPRECOSTATE) -> ::HRESULT,
    fn GetStatus(pStatus: *mut SPRECOGNIZERSTATUS) -> ::HRESULT,
    fn GetFormat(
        WaveFormatType: SPSTREAMFORMATTYPE, pFormatId: *mut ::GUID,
        ppCoMemWFEX: *mut ::WAVEFORMATEX
    ) -> ::HRESULT,
    fn IsUISupported(
        pszTypeOfUI: ::LPCWSTR, pvExtraData: *mut ::c_void, cbExtraData: ::ULONG,
        pfSupported: *mut ::BOOL
    ) -> ::HRESULT,
    fn DisplayUI(
        hwndParent: ::HWND, pszTitle: ::LPCWSTR, pszTypeOfUI: ::LPCWSTR,
        pvExtraData: *mut ::c_void, cbExtraData: ::ULONG
    ) -> ::HRESULT,
    fn EmulateRecognition(pPhrase: *mut ISpPhrase) -> ::HRESULT
}
);
RIDL!(
#[uuid(0x21b501a0, 0x0ec7, 0x46c9, 0x92, 0xc3, 0xa2, 0xbc, 0x78, 0x4c, 0x54, 0xb9)]
interface ISpSerializeState(ISpSerializeStateVtbl): IUnknown(IUnknownVtbl) {
    fn GetSerializedState(
        ppbData: *mut *mut ::BYTE, pulSize: *mut ::ULONG, dwReserved: ::DWORD
    ) -> ::HRESULT,
    fn SetSerializedState(
        pbData: *mut ::BYTE, ulSize: ::ULONG, dwReserved: ::DWORD
    ) -> ::HRESULT
}
);
RIDL!(
#[uuid(0x8fc6d974, 0xc81e, 0x4098, 0x93, 0xc5, 0x01, 0x47, 0xf6, 0x1e, 0xd4, 0xd3)]
interface ISpRecognizer2(ISpRecognizer2Vtbl): IUnknown(IUnknownVtbl) {
    fn EmulateRecognitionEx(
        pPhrase: *mut ISpPhrase, dwCompareFlags: ::DWORD
    ) -> ::HRESULT,
    fn SetTrainingState(
        fDoingTraining: ::BOOL, fAdaptFromTrainingData: ::BOOL
    ) -> ::HRESULT,
    fn ResetAcousticModelAdaptation() -> ::HRESULT
}
);
ENUM!{enum SPCATEGORYTYPE {
    SPCT_COMMAND = 0,
    SPCT_DICTATION,
    SPCT_SLEEP,
    SPCT_SUB_COMMAND,
    SPCT_SUB_DICTATION,
}}
RIDL!(
#[uuid(0xda0cd0f9, 0x14a2, 0x4f09, 0x8c, 0x2a, 0x85, 0xcc, 0x48, 0x97, 0x93, 0x45)]
interface ISpRecoCategory(ISpRecoCategoryVtbl): IUnknown(IUnknownVtbl) {
    fn GetType(peCategoryType: *mut SPCATEGORYTYPE) -> ::HRESULT
}
);
RIDL!(
#[uuid(0xdf1b943c, 0x5838, 0x4aa2, 0x87, 0x06, 0xd7, 0xcd, 0x5b, 0x33, 0x34, 0x99)]
interface ISpRecognizer3(ISpRecognizer3Vtbl): IUnknown(IUnknownVtbl) {
    fn GetCategory(
        categoryType: SPCATEGORYTYPE, ppCategory: *mut *mut ISpRecoCategory
    ) -> ::HRESULT,
    fn SetActiveCategory(pCategory: *mut ISpRecoCategory) -> ::HRESULT,
    fn GetActiveCategory(ppCategory: *mut *mut ISpRecoCategory) -> ::HRESULT
}
);
STRUCT!{struct SPNORMALIZATIONLIST {
    ulSize: ::ULONG,
    ppszzNormalizedList: *mut *mut ::WCHAR,
}}
RIDL!(
#[uuid(0xc360ce4b, 0x76d1, 0x4214, 0xad, 0x68, 0x52, 0x65, 0x7d, 0x50, 0x83, 0xda)]
interface ISpEnginePronunciation(ISpEnginePronunciationVtbl): IUnknown(IUnknownVtbl) {
    fn Normalize(
        pszWord: ::LPCWSTR, pszLeftContext: ::LPCWSTR, pszRightContext: ::LPCWSTR,
        LangID: ::WORD, pNormalizationList: *mut SPNORMALIZATIONLIST
    ) -> ::HRESULT,
    fn GetPronunciations(
        pszWord: ::LPCWSTR, pszLeftContext: ::LPCWSTR, pszRightContext: ::LPCWSTR,
        LangID: ::WORD, pEnginePronunciationList: *mut SPWORDPRONUNCIATIONLIST
    ) -> ::HRESULT
}
);
STRUCT!{struct SPDISPLAYTOKEN {
    pszLexical: *const ::WCHAR,
    pszDisplay: *const ::WCHAR,
    bDisplayAttributes: ::BYTE,
}}
STRUCT!{struct SPDISPLAYPHRASE {
    ulNumTokens: ::ULONG,
    pTokens: *mut SPDISPLAYTOKEN,
}}
RIDL!(
#[uuid(0xc8d7c7e2, 0x0dde, 0x44b7, 0xaf, 0xe3, 0xb0, 0xc9, 0x91, 0xfb, 0xeb, 0x5e)]
interface ISpDisplayAlternates(ISpDisplayAlternatesVtbl): IUnknown(IUnknownVtbl) {
    fn GetDisplayAlternates(
        pPhrase: *const SPDISPLAYPHRASE, cRequestCount: ::ULONG,
        ppCoMemPhrases: *mut *mut SPDISPLAYPHRASE, pcPhrasesReturned: *mut ::ULONG
    ) -> ::HRESULT,
    fn SetFullStopTrailSpace(ulTrailSpace: ::ULONG) -> ::HRESULT
}
);
pub type SpeechLanguageId = ::c_long;
ENUM!{enum DISPID_SpeechDataKey {
    DISPID_SDKSetBinaryValue = 1,
    DISPID_SDKGetBinaryValue,
    DISPID_SDKSetStringValue,
    DISPID_SDKGetStringValue,
    DISPID_SDKSetLongValue,
    DISPID_SDKGetlongValue,
    DISPID_SDKOpenKey,
    DISPID_SDKCreateKey,
    DISPID_SDKDeleteKey,
    DISPID_SDKDeleteValue,
    DISPID_SDKEnumKeys,
    DISPID_SDKEnumValues,
}}
ENUM!{enum DISPID_SpeechObjectToken {
    DISPID_SOTId = 1,
    DISPID_SOTDataKey,
    DISPID_SOTCategory,
    DISPID_SOTGetDescription,
    DISPID_SOTSetId,
    DISPID_SOTGetAttribute,
    DISPID_SOTCreateInstance,
    DISPID_SOTRemove,
    DISPID_SOTGetStorageFileName,
    DISPID_SOTRemoveStorageFileName,
    DISPID_SOTIsUISupported,
    DISPID_SOTDisplayUI,
    DISPID_SOTMatchesAttributes,
}}
ENUM!{enum SpeechDataKeyLocation {
    SDKLDefaultLocation = SPDKL_DefaultLocation,
    SDKLCurrentUser = SPDKL_CurrentUser,
    SDKLLocalMachine = SPDKL_LocalMachine,
    SDKLCurrentConfig = SPDKL_CurrentConfig,
}}
ENUM!{enum SpeechTokenContext {
    STCInprocServer = ::CLSCTX_INPROC_SERVER,
    STCInprocHandler = ::CLSCTX_INPROC_HANDLER,
    STCLocalServer = ::CLSCTX_LOCAL_SERVER,
    STCRemoteServer = ::CLSCTX_REMOTE_SERVER,
    STCAll = ::CLSCTX_INPROC_SERVER | ::CLSCTX_INPROC_HANDLER |
             ::CLSCTX_LOCAL_SERVER | ::CLSCTX_REMOTE_SERVER,
}}
ENUM!{enum SpeechTokenShellFolder {
    STSF_AppData = 0x1a,
    STSF_LocalAppData = 0x1c,
    STSF_CommonAppData = 0x23,
    STSF_FlagCreate = 0x8000,
}}
ENUM!{enum DISPID_SpeechObjectTokens {
    DISPID_SOTsCount = 1,
    DISPID_SOTsItem = ::DISPID_VALUE as u32,
    DISPID_SOTs_NewEnum = ::DISPID_NEWENUM as u32,
}}
ENUM!{enum DISPID_SpeechObjectTokenCategory {
    DISPID_SOTCId = 1,
    DISPID_SOTCDefault,
    DISPID_SOTCSetId,
    DISPID_SOTCGetDataKey,
    DISPID_SOTCEnumerateTokens,
}}
ENUM!{enum SpeechAudioFormatType {
    SAFTDefault = -1i32 as u32,
    SAFTNoAssignedFormat = 0,
    SAFTText = 1,
    SAFTNonStandardFormat = 2,
    SAFTExtendedAudioFormat = 3,
    SAFT8kHz8BitMono = 4,
    SAFT8kHz8BitStereo = 5,
    SAFT8kHz16BitMono = 6,
    SAFT8kHz16BitStereo = 7,
    SAFT11kHz8BitMono = 8,
    SAFT11kHz8BitStereo = 9,
    SAFT11kHz16BitMono = 10,
    SAFT11kHz16BitStereo = 11,
    SAFT12kHz8BitMono = 12,
    SAFT12kHz8BitStereo = 13,
    SAFT12kHz16BitMono = 14,
    SAFT12kHz16BitStereo = 15,
    SAFT16kHz8BitMono = 16,
    SAFT16kHz8BitStereo = 17,
    SAFT16kHz16BitMono = 18,
    SAFT16kHz16BitStereo = 19,
    SAFT22kHz8BitMono = 20,
    SAFT22kHz8BitStereo = 21,
    SAFT22kHz16BitMono = 22,
    SAFT22kHz16BitStereo = 23,
    SAFT24kHz8BitMono = 24,
    SAFT24kHz8BitStereo = 25,
    SAFT24kHz16BitMono = 26,
    SAFT24kHz16BitStereo = 27,
    SAFT32kHz8BitMono = 28,
    SAFT32kHz8BitStereo = 29,
    SAFT32kHz16BitMono = 30,
    SAFT32kHz16BitStereo = 31,
    SAFT44kHz8BitMono = 32,
    SAFT44kHz8BitStereo = 33,
    SAFT44kHz16BitMono = 34,
    SAFT44kHz16BitStereo = 35,
    SAFT48kHz8BitMono = 36,
    SAFT48kHz8BitStereo = 37,
    SAFT48kHz16BitMono = 38,
    SAFT48kHz16BitStereo = 39,
    SAFTTrueSpeech_8kHz1BitMono = 40,
    SAFTCCITT_ALaw_8kHzMono = 41,
    SAFTCCITT_ALaw_8kHzStereo = 42,
    SAFTCCITT_ALaw_11kHzMono = 43,
    SAFTCCITT_ALaw_11kHzStereo = 44,
    SAFTCCITT_ALaw_22kHzMono = 45,
    SAFTCCITT_ALaw_22kHzStereo = 46,
    SAFTCCITT_ALaw_44kHzMono = 47,
    SAFTCCITT_ALaw_44kHzStereo = 48,
    SAFTCCITT_uLaw_8kHzMono = 49,
    SAFTCCITT_uLaw_8kHzStereo = 50,
    SAFTCCITT_uLaw_11kHzMono = 51,
    SAFTCCITT_uLaw_11kHzStereo = 52,
    SAFTCCITT_uLaw_22kHzMono = 53,
    SAFTCCITT_uLaw_22kHzStereo = 54,
    SAFTCCITT_uLaw_44kHzMono = 55,
    SAFTCCITT_uLaw_44kHzStereo = 56,
    SAFTADPCM_8kHzMono = 57,
    SAFTADPCM_8kHzStereo = 58,
    SAFTADPCM_11kHzMono = 59,
    SAFTADPCM_11kHzStereo = 60,
    SAFTADPCM_22kHzMono = 61,
    SAFTADPCM_22kHzStereo = 62,
    SAFTADPCM_44kHzMono = 63,
    SAFTADPCM_44kHzStereo = 64,
    SAFTGSM610_8kHzMono = 65,
    SAFTGSM610_11kHzMono = 66,
    SAFTGSM610_22kHzMono = 67,
    SAFTGSM610_44kHzMono = 68,
}}
ENUM!{enum DISPID_SpeechAudioFormat {
    DISPID_SAFType = 1,
    DISPID_SAFGuid,
    DISPID_SAFGetWaveFormatEx,
    DISPID_SAFSetWaveFormatEx,
}}
ENUM!{enum DISPID_SpeechBaseStream {
    DISPID_SBSFormat = 1,
    DISPID_SBSRead,
    DISPID_SBSWrite,
    DISPID_SBSSeek,
}}
ENUM!{enum SpeechStreamSeekPositionType {
    SSSPTRelativeToStart = ::STREAM_SEEK_SET,
    SSSPTRelativeToCurrentPosition = ::STREAM_SEEK_CUR,
    SSSPTRelativeToEnd = ::STREAM_SEEK_END,
}}
ENUM!{enum DISPID_SpeechAudio {
    DISPID_SAStatus = 200,
    DISPID_SABufferInfo,
    DISPID_SADefaultFormat,
    DISPID_SAVolume,
    DISPID_SABufferNotifySize,
    DISPID_SAEventHandle,
    DISPID_SASetState,
}}
ENUM!{enum SpeechAudioState {
    SASClosed = SPAS_CLOSED,
    SASStop = SPAS_STOP,
    SASPause = SPAS_PAUSE,
    SASRun = SPAS_RUN,
}}
ENUM!{enum DISPID_SpeechMMSysAudio {
    DISPID_SMSADeviceId = 300,
    DISPID_SMSALineId,
    DISPID_SMSAMMHandle,
}}
ENUM!{enum DISPID_SpeechFileStream {
    DISPID_SFSOpen = 100,
    DISPID_SFSClose,
}}
ENUM!{enum SpeechStreamFileMode {
    SSFMOpenForRead = SPFM_OPEN_READONLY,
    SSFMOpenReadWrite = SPFM_OPEN_READWRITE,
    SSFMCreate = SPFM_CREATE,
    SSFMCreateForWrite = SPFM_CREATE_ALWAYS,
}}
ENUM!{enum DISPID_SpeechCustomStream {
    DISPID_SCSBaseStream = 100,
}}
ENUM!{enum DISPID_SpeechMemoryStream {
    DISPID_SMSSetData = 100,
    DISPID_SMSGetData,
}}
ENUM!{enum DISPID_SpeechAudioStatus {
    DISPID_SASFreeBufferSpace = 1,
    DISPID_SASNonBlockingIO,
    DISPID_SASState,
    DISPID_SASCurrentSeekPosition,
    DISPID_SASCurrentDevicePosition,
}}
ENUM!{enum DISPID_SpeechAudioBufferInfo {
    DISPID_SABIMinNotification = 1,
    DISPID_SABIBufferSize,
    DISPID_SABIEventBias,
}}
ENUM!{enum DISPID_SpeechWaveFormatEx {
    DISPID_SWFEFormatTag = 1,
    DISPID_SWFEChannels,
    DISPID_SWFESamplesPerSec,
    DISPID_SWFEAvgBytesPerSec,
    DISPID_SWFEBlockAlign,
    DISPID_SWFEBitsPerSample,
    DISPID_SWFEExtraData,
}}
ENUM!{enum DISPID_SpeechVoice {
    DISPID_SVStatus = 1,
    DISPID_SVVoice,
    DISPID_SVAudioOutput,
    DISPID_SVAudioOutputStream,
    DISPID_SVRate,
    DISPID_SVVolume,
    DISPID_SVAllowAudioOuputFormatChangesOnNextSet,
    DISPID_SVEventInterests,
    DISPID_SVPriority,
    DISPID_SVAlertBoundary,
    DISPID_SVSyncronousSpeakTimeout,
    DISPID_SVSpeak,
    DISPID_SVSpeakStream,
    DISPID_SVPause,
    DISPID_SVResume,
    DISPID_SVSkip,
    DISPID_SVGetVoices,
    DISPID_SVGetAudioOutputs,
    DISPID_SVWaitUntilDone,
    DISPID_SVSpeakCompleteEvent,
    DISPID_SVIsUISupported,
    DISPID_SVDisplayUI,
}}
ENUM!{enum SpeechVoicePriority {
    SVPNormal = SPVPRI_NORMAL,
    SVPAlert = SPVPRI_ALERT,
    SVPOver = SPVPRI_OVER,
}}
ENUM!{enum SpeechVoiceSpeakFlags {
    SVSFDefault = SPF_DEFAULT,
    SVSFlagsAsync = SPF_ASYNC,
    SVSFPurgeBeforeSpeak = SPF_PURGEBEFORESPEAK,
    SVSFIsFilename = SPF_IS_FILENAME,
    SVSFIsXML = SPF_IS_XML,
    SVSFIsNotXML = SPF_IS_NOT_XML,
    SVSFPersistXML = SPF_PERSIST_XML,
    SVSFNLPSpeakPunc = SPF_NLP_SPEAK_PUNC,
    SVSFParseSapi = SPF_PARSE_SAPI,
    SVSFParseSsml = SPF_PARSE_SSML,
    SVSFParseMask = SPF_PARSE_MASK as u32,
    SVSFVoiceMask = SPF_VOICE_MASK as u32,
    SVSFUnusedFlags = SPF_UNUSED_FLAGS as u32,
}}
pub const SVSFParseAutodetect: SpeechVoiceSpeakFlags = SVSFDefault;
pub const SVSFNLPMask: SpeechVoiceSpeakFlags = SVSFNLPSpeakPunc;
ENUM!{enum SpeechVoiceEvents {
    SVEStartInputStream = 1 << 1,
    SVEEndInputStream = 1 << 2,
    SVEVoiceChange = 1 << 3,
    SVEBookmark = 1 << 4,
    SVEWordBoundary = 1 << 5,
    SVEPhoneme = 1 << 6,
    SVESentenceBoundary = 1 << 7,
    SVEViseme = 1 << 8,
    SVEAudioLevel = 1 << 9,
    SVEPrivate = 1 << 15,
    SVEAllEvents = 0x83fe,
}}
ENUM!{enum DISPID_SpeechVoiceStatus {
    DISPID_SVSCurrentStreamNumber = 1,
    DISPID_SVSLastStreamNumberQueued,
    DISPID_SVSLastResult,
    DISPID_SVSRunningState,
    DISPID_SVSInputWordPosition,
    DISPID_SVSInputWordLength,
    DISPID_SVSInputSentencePosition,
    DISPID_SVSInputSentenceLength,
    DISPID_SVSLastBookmark,
    DISPID_SVSLastBookmarkId,
    DISPID_SVSPhonemeId,
    DISPID_SVSVisemeId,
}}
ENUM!{enum SpeechRunState {
    SRSEDone = SPRS_DONE,
    SRSEIsSpeaking = SPRS_IS_SPEAKING,
}}
ENUM!{enum SpeechVisemeType {
    SVP_0 = 0,
    SVP_1,
    SVP_2,
    SVP_3,
    SVP_4,
    SVP_5,
    SVP_6,
    SVP_7,
    SVP_8,
    SVP_9,
    SVP_10,
    SVP_11,
    SVP_12,
    SVP_13,
    SVP_14,
    SVP_15,
    SVP_16,
    SVP_17,
    SVP_18,
    SVP_19,
    SVP_20,
    SVP_21,
}}
ENUM!{enum SpeechVisemeFeature {
    SVF_None = 0,
    SVF_Stressed = SPVFEATURE_STRESSED,
    SVF_Emphasis = SPVFEATURE_EMPHASIS,
}}
ENUM!{enum DISPID_SpeechVoiceEvent {
    DISPID_SVEStreamStart = 1,
    DISPID_SVEStreamEnd,
    DISPID_SVEVoiceChange,
    DISPID_SVEBookmark,
    DISPID_SVEWord,
    DISPID_SVEPhoneme,
    DISPID_SVESentenceBoundary,
    DISPID_SVEViseme,
    DISPID_SVEAudioLevel,
    DISPID_SVEEnginePrivate,
}}
ENUM!{enum DISPID_SpeechRecognizer {
    DISPID_SRRecognizer = 1,
    DISPID_SRAllowAudioInputFormatChangesOnNextSet,
    DISPID_SRAudioInput,
    DISPID_SRAudioInputStream,
    DISPID_SRIsShared,
    DISPID_SRState,
    DISPID_SRStatus,
    DISPID_SRProfile,
    DISPID_SREmulateRecognition,
    DISPID_SRCreateRecoContext,
    DISPID_SRGetFormat,
    DISPID_SRSetPropertyNumber,
    DISPID_SRGetPropertyNumber,
    DISPID_SRSetPropertyString,
    DISPID_SRGetPropertyString,
    DISPID_SRIsUISupported,
    DISPID_SRDisplayUI,
    DISPID_SRGetRecognizers,
    DISPID_SVGetAudioInputs,
    DISPID_SVGetProfiles,
}}
ENUM!{enum SpeechRecognizerState {
    SRSInactive = SPRST_INACTIVE,
    SRSActive = SPRST_ACTIVE,
    SRSActiveAlways = SPRST_ACTIVE_ALWAYS,
    SRSInactiveWithPurge = SPRST_INACTIVE_WITH_PURGE,
}}
ENUM!{enum SpeechDisplayAttributes {
    SDA_No_Trailing_Space = 0,
    SDA_One_Trailing_Space = SPAF_ONE_TRAILING_SPACE,
    SDA_Two_Trailing_Spaces = SPAF_TWO_TRAILING_SPACES,
    SDA_Consume_Leading_Spaces = SPAF_CONSUME_LEADING_SPACES,
}}
ENUM!{enum SpeechFormatType {
    SFTInput = SPWF_INPUT,
    SFTSREngine = SPWF_SRENGINE,
}}
ENUM!{enum SpeechEmulationCompareFlags {
    SECFIgnoreCase = 0x1,
    SECFIgnoreKanaType = 0x10000,
    SECFIgnoreWidth = 0x20000,
    SECFNoSpecialChars = 0x20000000,
    SECFEmulateResult = 0x40000000,
    SECFDefault = SECFIgnoreCase | SECFIgnoreKanaType | SECFIgnoreWidth,
}}
ENUM!{enum DISPID_SpeechRecognizerStatus {
    DISPID_SRSAudioStatus = 1,
    DISPID_SRSCurrentStreamPosition,
    DISPID_SRSCurrentStreamNumber,
    DISPID_SRSNumberOfActiveRules,
    DISPID_SRSClsidEngine,
    DISPID_SRSSupportedLanguages,
}}
ENUM!{enum DISPID_SpeechRecoContext {
    DISPID_SRCRecognizer = 1,
    DISPID_SRCAudioInInterferenceStatus,
    DISPID_SRCRequestedUIType,
    DISPID_SRCVoice,
    DISPID_SRAllowVoiceFormatMatchingOnNextSet,
    DISPID_SRCVoicePurgeEvent,
    DISPID_SRCEventInterests,
    DISPID_SRCCmdMaxAlternates,
    DISPID_SRCState,
    DISPID_SRCRetainedAudio,
    DISPID_SRCRetainedAudioFormat,
    DISPID_SRCPause,
    DISPID_SRCResume,
    DISPID_SRCCreateGrammar,
    DISPID_SRCCreateResultFromMemory,
    DISPID_SRCBookmark,
    DISPID_SRCSetAdaptationData,
}}
ENUM!{enum SpeechRetainedAudioOptions {
    SRAONone = SPAO_NONE,
    SRAORetainAudio = SPAO_RETAIN_AUDIO,
}}
ENUM!{enum SpeechBookmarkOptions {
    SBONone = SPBO_NONE,
    SBOPause = SPBO_PAUSE,
}}
ENUM!{enum SpeechInterference {
    SINone = SPINTERFERENCE_NONE,
    SINoise = SPINTERFERENCE_NOISE,
    SINoSignal = SPINTERFERENCE_NOSIGNAL,
    SITooLoud = SPINTERFERENCE_TOOLOUD,
    SITooQuiet = SPINTERFERENCE_TOOQUIET,
    SITooFast = SPINTERFERENCE_TOOFAST,
    SITooSlow = SPINTERFERENCE_TOOSLOW,
}}
ENUM!{enum SpeechRecoEvents {
    SREStreamEnd = 1 << 0,
    SRESoundStart = 1 << 1,
    SRESoundEnd = 1 << 2,
    SREPhraseStart = 1 << 3,
    SRERecognition = 1 << 4,
    SREHypothesis = 1 << 5,
    SREBookmark = 1 << 6,
    SREPropertyNumChange = 1 << 7,
    SREPropertyStringChange = 1 << 8,
    SREFalseRecognition = 1 << 9,
    SREInterference = 1 << 10,
    SRERequestUI = 1 << 11,
    SREStateChange = 1 << 12,
    SREAdaptation = 1 << 13,
    SREStreamStart = 1 << 14,
    SRERecoOtherContext = 1 << 15,
    SREAudioLevel = 1 << 16,
    SREPrivate = 1 << 18,
    SREAllEvents = 0x5ffff,
}}
ENUM!{enum SpeechRecoContextState {
    SRCS_Disabled = SPCS_DISABLED,
    SRCS_Enabled = SPCS_ENABLED,
}}
ENUM!{enum DISPIDSPRG {
    DISPID_SRGId = 1,
    DISPID_SRGRecoContext,
    DISPID_SRGState,
    DISPID_SRGRules,
    DISPID_SRGReset,
    DISPID_SRGCommit,
    DISPID_SRGCmdLoadFromFile,
    DISPID_SRGCmdLoadFromObject,
    DISPID_SRGCmdLoadFromResource,
    DISPID_SRGCmdLoadFromMemory,
    DISPID_SRGCmdLoadFromProprietaryGrammar,
    DISPID_SRGCmdSetRuleState,
    DISPID_SRGCmdSetRuleIdState,
    DISPID_SRGDictationLoad,
    DISPID_SRGDictationUnload,
    DISPID_SRGDictationSetState,
    DISPID_SRGSetWordSequenceData,
    DISPID_SRGSetTextSelection,
    DISPID_SRGIsPronounceable,
}}
ENUM!{enum SpeechLoadOption {
    SLOStatic = SPLO_STATIC,
    SLODynamic = SPLO_DYNAMIC,
}}
ENUM!{enum SpeechWordPronounceable {
    SWPUnknownWordUnpronounceable = SPWP_UNKNOWN_WORD_UNPRONOUNCEABLE,
    SWPUnknownWordPronounceable = SPWP_UNKNOWN_WORD_PRONOUNCEABLE,
    SWPKnownWordPronounceable = SPWP_KNOWN_WORD_PRONOUNCEABLE,
}}
ENUM!{enum SpeechGrammarState {
    SGSEnabled = SPGS_ENABLED,
    SGSDisabled = SPGS_DISABLED,
    SGSExclusive = SPGS_EXCLUSIVE,
}}
ENUM!{enum SpeechRuleState {
    SGDSInactive = SPRS_INACTIVE,
    SGDSActive = SPRS_ACTIVE,
    SGDSActiveWithAutoPause = SPRS_ACTIVE_WITH_AUTO_PAUSE,
    SGDSActiveUserDelimited = SPRS_ACTIVE_USER_DELIMITED,
}}
ENUM!{enum SpeechRuleAttributes {
    SRATopLevel = SPRAF_TopLevel,
    SRADefaultToActive = SPRAF_Active,
    SRAExport = SPRAF_Export,
    SRAImport = SPRAF_Import,
    SRAInterpreter = SPRAF_Interpreter,
    SRADynamic = SPRAF_Dynamic,
    SRARoot = SPRAF_Root,
}}
ENUM!{enum SpeechGrammarWordType {
    SGDisplay = SPWT_DISPLAY,
    SGLexical = SPWT_LEXICAL,
    SGPronounciation = SPWT_PRONUNCIATION,
    SGLexicalNoSpecialChars = SPWT_LEXICAL_NO_SPECIAL_CHARS,
}}
ENUM!{enum DISPID_SpeechRecoContextEvents {
    DISPID_SRCEStartStream = 1,
    DISPID_SRCEEndStream,
    DISPID_SRCEBookmark,
    DISPID_SRCESoundStart,
    DISPID_SRCESoundEnd,
    DISPID_SRCEPhraseStart,
    DISPID_SRCERecognition,
    DISPID_SRCEHypothesis,
    DISPID_SRCEPropertyNumberChange,
    DISPID_SRCEPropertyStringChange,
    DISPID_SRCEFalseRecognition,
    DISPID_SRCEInterference,
    DISPID_SRCERequestUI,
    DISPID_SRCERecognizerStateChange,
    DISPID_SRCEAdaptation,
    DISPID_SRCERecognitionForOtherContext,
    DISPID_SRCEAudioLevel,
    DISPID_SRCEEnginePrivate,
}}
ENUM!{enum SpeechRecognitionType {
    SRTStandard = 0,
    SRTAutopause = SPREF_AutoPause,
    SRTEmulated = SPREF_Emulated,
    SRTSMLTimeout = SPREF_SMLTimeout,
    SRTExtendableParse = SPREF_ExtendableParse,
    SRTReSent = SPREF_ReSent,
}}
ENUM!{enum DISPID_SpeechGrammarRule {
    DISPID_SGRAttributes = 1,
    DISPID_SGRInitialState,
    DISPID_SGRName,
    DISPID_SGRId,
    DISPID_SGRClear,
    DISPID_SGRAddResource,
    DISPID_SGRAddState,
}}
ENUM!{enum DISPID_SpeechGrammarRules {
    DISPID_SGRsCount = 1,
    DISPID_SGRsDynamic,
    DISPID_SGRsAdd,
    DISPID_SGRsCommit,
    DISPID_SGRsCommitAndSave,
    DISPID_SGRsFindRule,
    DISPID_SGRsItem = ::DISPID_VALUE as u32,
    DISPID_SGRs_NewEnum = ::DISPID_NEWENUM as u32,
}}
ENUM!{enum DISPID_SpeechGrammarRuleState {
    DISPID_SGRSRule = 1,
    DISPID_SGRSTransitions,
    DISPID_SGRSAddWordTransition,
    DISPID_SGRSAddRuleTransition,
    DISPID_SGRSAddSpecialTransition,
}}
ENUM!{enum SpeechSpecialTransitionType {
    SSTTWildcard = 1,
    SSTTDictation,
    SSTTTextBuffer,
}}
ENUM!{enum DISPID_SpeechGrammarRuleStateTransitions {
    DISPID_SGRSTsCount = 1,
    DISPID_SGRSTsItem = ::DISPID_VALUE as u32,
    DISPID_SGRSTs_NewEnum = ::DISPID_NEWENUM as u32,
}}
ENUM!{enum DISPID_SpeechGrammarRuleStateTransition {
    DISPID_SGRSTType = 1,
    DISPID_SGRSTText,
    DISPID_SGRSTRule,
    DISPID_SGRSTWeight,
    DISPID_SGRSTPropertyName,
    DISPID_SGRSTPropertyId,
    DISPID_SGRSTPropertyValue,
    DISPID_SGRSTNextState,
}}
ENUM!{enum SpeechGrammarRuleStateTransitionType {
    SGRSTTEpsilon = 0,
    SGRSTTWord,
    SGRSTTRule,
    SGRSTTDictation,
    SGRSTTWildcard,
    SGRSTTTextBuffer,
}}
ENUM!{enum DISPIDSPTSI {
    DISPIDSPTSI_ActiveOffset = 1,
    DISPIDSPTSI_ActiveLength,
    DISPIDSPTSI_SelectionOffset,
    DISPIDSPTSI_SelectionLength,
}}
ENUM!{enum DISPID_SpeechRecoResult {
    DISPID_SRRRecoContext = 1,
    DISPID_SRRTimes,
    DISPID_SRRAudioFormat,
    DISPID_SRRPhraseInfo,
    DISPID_SRRAlternates,
    DISPID_SRRAudio,
    DISPID_SRRSpeakAudio,
    DISPID_SRRSaveToMemory,
    DISPID_SRRDiscardResultInfo,
}}
ENUM!{enum SpeechDiscardType {
    SDTProperty = SPDF_PROPERTY,
    SDTReplacement = SPDF_REPLACEMENT,
    SDTRule = SPDF_RULE,
    SDTDisplayText = SPDF_DISPLAYTEXT,
    SDTLexicalForm = SPDF_LEXICALFORM,
    SDTPronunciation = SPDF_PRONUNCIATION,
    SDTAudio = SPDF_AUDIO,
    SDTAlternates = SPDF_ALTERNATES,
    SDTAll = SPDF_ALL,
}}
ENUM!{enum DISPID_SpeechXMLRecoResult {
    DISPID_SRRGetXMLResult,
    DISPID_SRRGetXMLErrorInfo,
}}
ENUM!{enum DISPID_SpeechRecoResult2 {
    DISPID_SRRSetTextFeedback,
}}
ENUM!{enum DISPID_SpeechPhraseBuilder {
    DISPID_SPPBRestorePhraseFromMemory = 1,
}}
ENUM!{enum DISPID_SpeechRecoResultTimes {
    DISPID_SRRTStreamTime = 1,
    DISPID_SRRTLength,
    DISPID_SRRTTickCount,
    DISPID_SRRTOffsetFromStart,
}}
ENUM!{enum DISPID_SpeechPhraseAlternate {
    DISPID_SPARecoResult = 1,
    DISPID_SPAStartElementInResult,
    DISPID_SPANumberOfElementsInResult,
    DISPID_SPAPhraseInfo,
    DISPID_SPACommit,
}}
ENUM!{enum DISPID_SpeechPhraseAlternates {
    DISPID_SPAsCount = 1,
    DISPID_SPAsItem = ::DISPID_VALUE as u32,
    DISPID_SPAs_NewEnum = ::DISPID_NEWENUM as u32,
}}
ENUM!{enum DISPID_SpeechPhraseInfo {
    DISPID_SPILanguageId = 1,
    DISPID_SPIGrammarId,
    DISPID_SPIStartTime,
    DISPID_SPIAudioStreamPosition,
    DISPID_SPIAudioSizeBytes,
    DISPID_SPIRetainedSizeBytes,
    DISPID_SPIAudioSizeTime,
    DISPID_SPIRule,
    DISPID_SPIProperties,
    DISPID_SPIElements,
    DISPID_SPIReplacements,
    DISPID_SPIEngineId,
    DISPID_SPIEnginePrivateData,
    DISPID_SPISaveToMemory,
    DISPID_SPIGetText,
    DISPID_SPIGetDisplayAttributes,
}}
ENUM!{enum DISPID_SpeechPhraseElement {
    DISPID_SPEAudioTimeOffset = 1,
    DISPID_SPEAudioSizeTime,
    DISPID_SPEAudioStreamOffset,
    DISPID_SPEAudioSizeBytes,
    DISPID_SPERetainedStreamOffset,
    DISPID_SPERetainedSizeBytes,
    DISPID_SPEDisplayText,
    DISPID_SPELexicalForm,
    DISPID_SPEPronunciation,
    DISPID_SPEDisplayAttributes,
    DISPID_SPERequiredConfidence,
    DISPID_SPEActualConfidence,
    DISPID_SPEEngineConfidence,
}}
ENUM!{enum SpeechEngineConfidence {
    SECLowConfidence = -1i32 as u32,
    SECNormalConfidence = 0,
    SECHighConfidence = 1,
}}
ENUM!{enum DISPID_SpeechPhraseElements {
    DISPID_SPEsCount = 1,
    DISPID_SPEsItem = ::DISPID_VALUE as u32,
    DISPID_SPEs_NewEnum = ::DISPID_NEWENUM as u32,
}}
ENUM!{enum DISPID_SpeechPhraseReplacement {
    DISPID_SPRDisplayAttributes = 1,
    DISPID_SPRText,
    DISPID_SPRFirstElement,
    DISPID_SPRNumberOfElements,
}}
ENUM!{enum DISPID_SpeechPhraseReplacements {
    DISPID_SPRsCount = 1,
    DISPID_SPRsItem = ::DISPID_VALUE as u32,
    DISPID_SPRs_NewEnum = ::DISPID_NEWENUM as u32,
}}
ENUM!{enum DISPID_SpeechPhraseProperty {
    DISPID_SPPName = 1,
    DISPID_SPPId,
    DISPID_SPPValue,
    DISPID_SPPFirstElement,
    DISPID_SPPNumberOfElements,
    DISPID_SPPEngineConfidence,
    DISPID_SPPConfidence,
    DISPID_SPPParent,
    DISPID_SPPChildren,
}}
ENUM!{enum DISPID_SpeechPhraseProperties {
    DISPID_SPPsCount = 1,
    DISPID_SPPsItem = ::DISPID_VALUE as u32,
    DISPID_SPPs_NewEnum = ::DISPID_NEWENUM as u32,
}}
ENUM!{enum DISPID_SpeechPhraseRule {
    DISPID_SPRuleName = 1,
    DISPID_SPRuleId,
    DISPID_SPRuleFirstElement,
    DISPID_SPRuleNumberOfElements,
    DISPID_SPRuleParent,
    DISPID_SPRuleChildren,
    DISPID_SPRuleConfidence,
    DISPID_SPRuleEngineConfidence,
}}
ENUM!{enum DISPID_SpeechPhraseRules {
    DISPID_SPRulesCount = 1,
    DISPID_SPRulesItem = ::DISPID_VALUE as u32,
    DISPID_SPRules_NewEnum = ::DISPID_NEWENUM as u32,
}}
ENUM!{enum DISPID_SpeechLexicon {
    DISPID_SLGenerationId = 1,
    DISPID_SLGetWords,
    DISPID_SLAddPronunciation,
    DISPID_SLAddPronunciationByPhoneIds,
    DISPID_SLRemovePronunciation,
    DISPID_SLRemovePronunciationByPhoneIds,
    DISPID_SLGetPronunciations,
    DISPID_SLGetGenerationChange,
}}
ENUM!{enum SpeechLexiconType {
    SLTUser = eLEXTYPE_USER,
    SLTApp = eLEXTYPE_APP,
}}
ENUM!{enum SpeechPartOfSpeech {
    SPSNotOverriden = SPPS_NotOverriden,
    SPSUnknown = SPPS_Unknown,
    SPSNoun = SPPS_Noun,
    SPSVerb = SPPS_Verb,
    SPSModifier = SPPS_Modifier,
    SPSFunction = SPPS_Function,
    SPSInterjection = SPPS_Interjection,
    SPSLMA = SPPS_LMA,
    SPSSuppressWord = SPPS_SuppressWord,
}}
ENUM!{enum DISPID_SpeechLexiconWords {
    DISPID_SLWsCount = 1,
    DISPID_SLWsItem = ::DISPID_VALUE as u32,
    DISPID_SLWs_NewEnum = ::DISPID_NEWENUM as u32,
}}
ENUM!{enum SpeechWordType {
    SWTAdded = eWORDTYPE_ADDED,
    SWTDeleted = eWORDTYPE_DELETED,
}}
ENUM!{enum DISPID_SpeechLexiconWord {
    DISPID_SLWLangId = 1,
    DISPID_SLWType,
    DISPID_SLWWord,
    DISPID_SLWPronunciations,
}}
ENUM!{enum DISPID_SpeechLexiconProns {
    DISPID_SLPsCount = 1,
    DISPID_SLPsItem = ::DISPID_VALUE as u32,
    DISPID_SLPs_NewEnum = ::DISPID_NEWENUM as u32,
}}
ENUM!{enum DISPID_SpeechLexiconPronunciation {
    DISPID_SLPType = 1,
    DISPID_SLPLangId,
    DISPID_SLPPartOfSpeech,
    DISPID_SLPPhoneIds,
    DISPID_SLPSymbolic,
}}
ENUM!{enum DISPID_SpeechPhoneConverter {
    DISPID_SPCLangId = 1,
    DISPID_SPCPhoneToId,
    DISPID_SPCIdToPhone,
}}
RIDL!(
#[uuid(0xce17c09b, 0x4efa, 0x44d5, 0xa4, 0xc9, 0x59, 0xd9, 0x58, 0x5a, 0xb0, 0xcd)]
interface ISpeechDataKey(ISpeechDataKeyVtbl): IDispatch(IDispatchVtbl) {
    fn SetBinaryValue(ValueName: ::BSTR, Value: ::VARIANT) -> ::HRESULT,
    fn GetBinaryValue(ValueName: ::BSTR, Value: *mut ::VARIANT) -> ::HRESULT,
    fn SetStringValue(ValueName: ::BSTR, Value: ::BSTR) -> ::HRESULT,
    fn GetStringValue(ValueName: ::BSTR, Value: *mut ::BSTR) -> ::HRESULT,
    fn SetLongValue(ValueName: ::BSTR, Value: ::c_long) -> ::HRESULT,
    fn GetLongValue(ValueName: ::BSTR, Value: *mut ::c_long) -> ::HRESULT,
    fn OpenKey(SubKeyName: ::BSTR, SubKey: *mut *mut ISpeechDataKey) -> ::HRESULT,
    fn CreateKey(SubKeyName: ::BSTR, SubKey: *mut *mut ISpeechDataKey) -> ::HRESULT,
    fn DeleteKey(SubKeyName: ::BSTR) -> ::HRESULT,
    fn DeleteValue(ValueName: ::BSTR) -> ::HRESULT,
    fn EnumKeys(Index: ::c_long, SubKeyName: *mut ::BSTR) -> ::HRESULT,
    fn EnumValues(Index: ::c_long, ValueName: *mut ::BSTR) -> ::HRESULT
}
);
RIDL!(
#[uuid(0xc74a3adc, 0xb727, 0x4500, 0xa8, 0x4a, 0xb5, 0x26, 0x72, 0x1c, 0x8b, 0x8c)]
interface ISpeechObjectToken(ISpeechObjectTokenVtbl): IDispatch(IDispatchVtbl) {
    fn get_Id(ObjectId: *mut ::BSTR) -> ::HRESULT,
    fn get_DataKey(DataKey: *mut *mut ISpeechDataKey) -> ::HRESULT,
    fn get_Category(Category: *mut *mut ISpeechObjectTokenCategory) -> ::HRESULT,
    fn GetDescription(Locale: ::c_long, Description: *mut ::BSTR) -> ::HRESULT,
    fn SetId(
        Id: ::BSTR, CategoryId: ::BSTR, CreateIfNotExist: ::VARIANT_BOOL
    ) -> ::HRESULT,
    fn GetAttribute(AttributeName: ::BSTR, AttributeValue: *mut ::BSTR) -> ::HRESULT,
    fn CreateInstance(
        pUnkOuter: *mut ::IUnknown, ClsContext: SpeechTokenContext,
        Object: *mut *mut ::IUnknown
    ) -> ::HRESULT,
    fn Remove(ObjectStorageCLSID: ::BSTR) -> ::HRESULT,
    fn GetStorageFileName(
        ObjectStorageCLSID: ::BSTR, KeyName: ::BSTR, FileName: ::BSTR, Folder: ::BSTR,
        FilePath: *mut ::BSTR
    ) -> ::HRESULT,
    fn RemoveStorageFileName(
        ObjectStorageCLSID: ::BSTR, KeyName: ::BSTR, DeleteFile: ::VARIANT_BOOL
    ) -> ::HRESULT,
    fn IsUISupported(
        TypeOfUI: ::BSTR, ExtraData: *const ::VARIANT, Object: *mut ::IUnknown,
        Supported: *mut ::VARIANT_BOOL
    ) -> ::HRESULT,
    fn DisplayUI(
        hWnd: ::c_long, Title: ::BSTR, TypeOfUI: ::BSTR, ExtraData: *const ::VARIANT,
        Object: *mut ::IUnknown
    ) -> ::HRESULT,
    fn MatchesAttributes(Attributes: ::BSTR, Matches: *mut ::VARIANT_BOOL) -> ::HRESULT
}
);
RIDL!(
#[uuid(0x9285b776, 0x2e7b, 0x4bc0, 0xb5, 0x3e, 0x58, 0x0e, 0xb6, 0xfa, 0x96, 0x7f)]
interface ISpeechObjectTokens(ISpeechObjectTokensVtbl): IDispatch(IDispatchVtbl) {
    fn get_Count(Count: *mut ::c_long) -> ::HRESULT,
    fn Item(Index: ::c_long, Token: *mut *mut ISpeechObjectToken) -> ::HRESULT,
    fn get__NewEnum(ppEnumVARIANT: *mut *mut ::IUnknown) -> ::HRESULT
}
);
RIDL!(
#[uuid(0xca7eac50, 0x2d01, 0x4145, 0x86, 0xd4, 0x5a, 0xe7, 0xd7, 0x0f, 0x44, 0x69)]
interface ISpeechObjectTokenCategory(ISpeechObjectTokenCategoryVtbl): IDispatch(IDispatchVtbl) {
    fn get_Id(Id: *mut ::BSTR) -> ::HRESULT,
    fn put_Default(TokenId: ::BSTR) -> ::HRESULT,
    fn get_Default(TokenId: *mut ::BSTR) -> ::HRESULT,
    fn SetId(Id: ::BSTR, CreateIfNotExist: ::VARIANT_BOOL) -> ::HRESULT,
    fn GetDataKey(
        Location: SpeechDataKeyLocation, DataKey: *mut *mut ISpeechDataKey
    ) -> ::HRESULT,
    fn EnumerateTokens(
        RequiredAttributes: ::BSTR, OptionalAttributes: ::BSTR,
        Tokens: *mut *mut ISpeechObjectTokens
    ) -> ::HRESULT
}
);
RIDL!(
#[uuid(0x11b103d8, 0x1142, 0x4edf, 0xa0, 0x93, 0x82, 0xfb, 0x39, 0x15, 0xf8, 0xcc)]
interface ISpeechAudioBufferInfo(ISpeechAudioBufferInfoVtbl): IDispatch(IDispatchVtbl) {
    fn get_MinNotification(MinNotification: *mut ::c_long) -> ::HRESULT,
    fn put_MinNotification(MinNotification: ::c_long) -> ::HRESULT,
    fn get_BufferSize(BufferSize: *mut ::c_long) -> ::HRESULT,
    fn put_BufferSize(BufferSize: ::c_long) -> ::HRESULT,
    fn get_EventBias(EventBias: *mut ::c_long) -> ::HRESULT,
    fn put_EventBias(EventBias: ::c_long) -> ::HRESULT
}
);
RIDL!(
#[uuid(0xc62d9c91, 0x7458, 0x47f6, 0x86, 0x2d, 0x1e, 0xf8, 0x6f, 0xb0, 0xb2, 0x78)]
interface ISpeechAudioStatus(ISpeechAudioStatusVtbl): IDispatch(IDispatchVtbl) {
    fn get_FreeBufferSpace(FreeBufferSpace: *mut ::c_long) -> ::HRESULT,
    fn get_NonBlockingIO(NonBlockingIO: *mut ::c_long) -> ::HRESULT,
    fn get_State(State: *mut SpeechAudioState) -> ::HRESULT,
    fn get_CurrentSeekPosition(CurrentSeekPosition: *mut ::VARIANT) -> ::HRESULT,
    fn get_CurrentDevicePosition(CurrentDevicePosition: *mut ::VARIANT) -> ::HRESULT
}
);
RIDL!(
#[uuid(0xe6e9c590, 0x3e18, 0x40e3, 0x82, 0x99, 0x06, 0x1f, 0x98, 0xbd, 0xe7, 0xc7)]
interface ISpeechAudioFormat(ISpeechAudioFormatVtbl): IDispatch(IDispatchVtbl) {
    fn get_Type(AudioFormat: *mut SpeechAudioFormatType) -> ::HRESULT,
    fn put_Type(AudioFormat: SpeechAudioFormatType) -> ::HRESULT,
    fn get_Guid(Guid: *mut ::BSTR) -> ::HRESULT,
    fn put_Guid(Guid: ::BSTR) -> ::HRESULT,
    fn GetWaveFormatEx(SpeechWaveFormatEx: *mut *mut ISpeechWaveFormatEx) -> ::HRESULT,
    fn SetWaveFormatEx(SpeechWaveFormatEx: *mut ISpeechWaveFormatEx) -> ::HRESULT
}
);
RIDL!(
#[uuid(0x7a1ef0d5, 0x1581, 0x4741, 0x88, 0xe4, 0x20, 0x9a, 0x49, 0xf1, 0x1a, 0x10)]
interface ISpeechWaveFormatEx(ISpeechWaveFormatExVtbl): IDispatch(IDispatchVtbl) {
    fn get_FormatTag(FormatTag: *mut ::c_short) -> ::HRESULT,
    fn put_FormatTag(FormatTag: ::c_short) -> ::HRESULT,
    fn get_Channels(Channels: *mut ::c_short) -> ::HRESULT,
    fn put_Channels(Channels: ::c_short) -> ::HRESULT,
    fn get_SamplesPerSec(SamplesPerSec: *mut ::c_long) -> ::HRESULT,
    fn put_SamplesPerSec(SamplesPerSec: ::c_long) -> ::HRESULT,
    fn get_AvgBytesPerSec(AvgBytesPerSec: *mut ::c_long) -> ::HRESULT,
    fn put_AvgBytesPerSec(AvgBytesPerSec: ::c_long) -> ::HRESULT,
    fn get_BlockAlign(BlockAlign: *mut ::c_short) -> ::HRESULT,
    fn put_BlockAlign(BlockAlign: ::c_short) -> ::HRESULT,
    fn get_BitsPerSample(BitsPerSample: *mut ::c_short) -> ::HRESULT,
    fn put_BitsPerSample(BitsPerSample: ::c_short) -> ::HRESULT,
    fn get_ExtraData(ExtraData: *mut ::VARIANT) -> ::HRESULT,
    fn put_ExtraData(ExtraData: ::VARIANT) -> ::HRESULT
}
);

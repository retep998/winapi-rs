// Copyright © 2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! SAPI 5.1 definitions
use ctypes::{c_char, c_float, c_long, c_void};
use shared::guiddef::{CLSID, GUID, REFCLSID, REFGUID, REFIID};
use shared::minwindef::{
    BOOL, BYTE, DWORD, FILETIME, HKEY, HMODULE, LPARAM, UINT, ULONG, USHORT, WORD, WPARAM
};
use shared::mmreg::WAVEFORMATEX;
use shared::windef::HWND;
use um::oaidl::VARIANT;
use um::objidlbase::{IStream, IStreamVtbl};
use um::servprov::{IServiceProvider, IServiceProviderVtbl};
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::{HANDLE, HRESULT, LONG, LONGLONG, LPCWSTR, LPWSTR, ULONGLONG, WCHAR};
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
extern {
    pub static SPDFID_Text: GUID;
    pub static SPDFID_WaveFormatEx: GUID;
}
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
// TODO: Check types.
pub const SP_LOW_CONFIDENCE: i32 = -1;
pub const SP_NORMAL_CONFIDENCE: i32 = 0;
pub const SP_HIGH_CONFIDENCE: i32 = 1;
pub const DEFAULT_WEIGHT: i32 = 1;
pub const SP_MAX_WORD_LENGTH: i32 = 128;
pub const SP_MAX_PRON_LENGTH: i32 = 384;
pub struct ISpNotifyCallback; // TODO
/*RIDL!(interface ISpNotifyCallback(ISpNotifyCallbackVtbl) {
    fn NotifyCallback(
        wParam: WPARAM,
        lParam: LPARAM
    ) -> HRESULT
});*/
FN!(stdcall SPNOTIFYCALLBACK(wParam: WPARAM, lParam: LPARAM) -> ());
RIDL!(#[uuid(0x5eff4aef, 0x8487, 0x11d2, 0x96, 0x1c, 0x00, 0xc0, 0x4f, 0x8e, 0xe6, 0x28)]
interface ISpNotifySource(ISpNotifySourceVtbl): IUnknown(IUnknownVtbl) {
    fn SetNotifySink(pNotifySink: *mut ISpNotifySink) -> HRESULT,
    fn SetNotifyWindowMessage(
        hWnd: HWND,
        Msg: UINT,
        wParam: WPARAM,
        lParam: LPARAM
    ) -> HRESULT,
    fn SetNotifyCallbackFunction(
        pfnCallback: SPNOTIFYCALLBACK,
        wParam: WPARAM,
        lParam: LPARAM
    ) -> HRESULT,
    fn SetNotifyCallbackInterface(
        pSpCallback: *mut ISpNotifyCallback,
        wParam: WPARAM,
        lParam: LPARAM
    ) -> HRESULT,
    fn SetNotifyWin32Event() -> HRESULT,
    fn WaitForNotifyEvent(dwMilliseconds: DWORD) -> HRESULT,
    fn GetNotifyEventHandle() -> HANDLE
});
RIDL!(#[uuid(0x259684dc, 0x37c3, 0x11d2, 0x96, 0x03, 0x00, 0xc0, 0x4f, 0x8e, 0xe6, 0x28)]
interface ISpNotifySink(ISpNotifySinkVtbl): IUnknown(IUnknownVtbl) {
    fn Notify() -> HRESULT
});
RIDL!(#[uuid(0xaca16614, 0x5d3d, 0x11d2, 0x96, 0x0e, 0x00, 0xc0, 0x4f, 0x8e, 0xe6, 0x28)]
interface ISpNotifyTranslator(ISpNotifyTranslatorVtbl): ISpNotifySink(ISpNotifySinkVtbl) {
    fn InitWindowMessage(
        hWnd: HWND,
        Msg: UINT,
        wParam: WPARAM,
        lParam: LPARAM
    ) -> HRESULT,
    fn InitCallback(
        pfnCallback: SPNOTIFYCALLBACK,
        wParam: WPARAM,
        lParam: LPARAM
    ) -> HRESULT,
    fn InitSpNotifyCallback(
        pSpCallback: *mut ISpNotifyCallback,
        wParam: WPARAM,
        lParam: LPARAM
    ) -> HRESULT,
    fn InitWin32Event(
        hEvent: HANDLE,
        fCloseHandleOnRelease: BOOL
    ) -> HRESULT,
    fn Wait(dwMilliseconds: DWORD) -> HRESULT,
    fn GetEventHandle() -> HANDLE
});
RIDL!(#[uuid(0x14056581, 0xe16c, 0x11d2, 0xbb, 0x90, 0x00, 0xc0, 0x4f, 0x8e, 0xe6, 0xc0)]
interface ISpDataKey(ISpDataKeyVtbl): IUnknown(IUnknownVtbl) {
    fn SetData(
        pszValueName: LPCWSTR,
        cbData: ULONG,
        pData: *const BYTE
    ) -> HRESULT,
    fn GetData(
        pszValueName: LPCWSTR,
        pcbData: *mut ULONG,
        pData: *mut BYTE
    ) -> HRESULT,
    fn SetStringValue(
        pszValueName: LPCWSTR,
        pszValue: LPCWSTR
    ) -> HRESULT,
    fn GetStringValue(
        pszValueName: LPCWSTR,
        ppszValue: *mut LPWSTR
    ) -> HRESULT,
    fn SetDWORD(
        pszValueName: LPCWSTR,
        dwValue: DWORD
    ) -> HRESULT,
    fn GetDWORD(
        pszValueName: LPCWSTR,
        pdwValue: *mut DWORD
    ) -> HRESULT,
    fn OpenKey(
        pszSubKeyName: LPCWSTR,
        ppSubKey: *mut *mut ISpDataKey
    ) -> HRESULT,
    fn CreateKey(
        pszSubKey: LPCWSTR,
        ppSubKey: *mut *mut ISpDataKey
    ) -> HRESULT,
    fn DeleteKey(pszSubKey: LPCWSTR) -> HRESULT,
    fn DeleteValue(pszValueName: LPCWSTR) -> HRESULT,
    fn EnumKeys(
        Index: ULONG,
        ppszSubKeyName: *mut LPWSTR
    ) -> HRESULT,
    fn EnumValues(
        Index: ULONG,
        ppszValueName: *mut LPWSTR
    ) -> HRESULT
});
RIDL!(#[uuid(0x92a66e2b, 0xc830, 0x4149, 0x83, 0xdf, 0x6f, 0xc2, 0xba, 0x1e, 0x7a, 0x5b)]
interface ISpRegDataKey(ISpRegDataKeyVtbl): ISpDataKey(ISpDataKeyVtbl) {
    fn SetKey(
        hkey: HKEY,
        fReadOnly: BOOL
    ) -> HRESULT
});
RIDL!(#[uuid(0x2d3d3845, 0x39af, 0x4850, 0xbb, 0xf9, 0x40, 0xb4, 0x97, 0x80, 0x01, 0x1d)]
interface ISpObjectTokenCategory(ISpObjectTokenCategoryVtbl): ISpDataKey(ISpDataKeyVtbl) {
    fn SetId(
        pszCategoryId: LPCWSTR,
        fCreateIfNotExist: BOOL
    ) -> HRESULT,
    fn GetId(ppszCoMemCategoryId: *mut LPWSTR) -> HRESULT,
    fn GetDataKey(
        spdkl: SPDATAKEYLOCATION,
        pppDataKey: *mut *mut ISpDataKey
    ) -> HRESULT,
    fn EnumTokens(
        pzsReqAttribs: LPCWSTR,
        pszOptAttribs: LPCWSTR,
        ppEnum: *mut *mut IEnumSpObjectTokens
    ) -> HRESULT,
    fn SetDefaultTokenId(pszTokenId: LPCWSTR) -> HRESULT,
    fn GetDefaultTokenId(ppszCoMemTokenId: *mut LPWSTR) -> HRESULT
});
RIDL!(#[uuid(0x14056589, 0xe16c, 0x11d2, 0xbb, 0x90, 0x00, 0xc0, 0x4f, 0x8e, 0xe6, 0xc0)]
interface ISpObjectToken(ISpObjectTokenVtbl): ISpDataKey(ISpDataKeyVtbl) {
    fn SetId(
        pszCategoryId: LPCWSTR,
        pszTokenId: LPCWSTR,
        fCreateIfNotExist: BOOL
    ) -> HRESULT,
    fn GetId(ppszCoMemTokenId: *mut LPWSTR) -> HRESULT,
    fn GetCategory(ppTokenCategory: *mut *mut ISpObjectTokenCategory) -> HRESULT,
    fn CreateInstance(
        pUnkOuter: *mut IUnknown,
        dwClsContext: DWORD,
        riid: REFIID,
        ppvObject: *mut *mut c_void
    ) -> HRESULT,
    fn GetStorageFileName(
        clsidCaller: REFCLSID,
        pszValueName: LPCWSTR,
        pszFileNameSpecifier: LPCWSTR,
        nFolder: ULONG,
        ppszFilePath: *mut LPWSTR
    ) -> HRESULT,
    fn RemoveStorageFileName(
        pszKeyName: LPCWSTR,
        fDeleteFile: BOOL
    ) -> HRESULT,
    fn Remove(pclsidCaller: *const CLSID) -> HRESULT,
    fn IsUISupported(
        pszTypeOfUI: LPCWSTR,
        pvExtraData: *mut c_void,
        cbExtraData: ULONG,
        punkObject: *mut IUnknown,
        pfSupported: *mut BOOL
    ) -> HRESULT,
    fn DisplayUI(
        hwndParent: HWND,
        pszTitle: LPCWSTR,
        pszTypeOfUI: LPCWSTR,
        pvExtraData: *mut c_void,
        cbExtraData: ULONG,
        punkObject: *mut IUnknown
    ) -> HRESULT,
    fn MatchesAttributes(
        pszAttributes: LPCWSTR,
        pfMatches: *mut BOOL
    ) -> HRESULT
});
RIDL!(#[uuid(0xb8aab0cf, 0x346f, 0x49d8, 0x94, 0x99, 0xc8, 0xb0, 0x3f, 0x16, 0x1d, 0x51)]
interface ISpObjectTokenInit(ISpObjectTokenInitVtbl): ISpObjectToken(ISpObjectTokenVtbl) {
    fn InitFromDataKey(
        pszCategoryId: LPCWSTR,
        pszTokenId: LPCWSTR,
        pDataKey: *mut ISpDataKey
    ) -> HRESULT
});
RIDL!(#[uuid(0x06b64f9e, 0x7fda, 0x11d2, 0xb4, 0xf2, 0x00, 0xc0, 0x4f, 0x79, 0x73, 0x96)]
interface IEnumSpObjectTokens(IEnumSpObjectTokensVtbl): IUnknown(IUnknownVtbl) {
    fn Next(
        celt: ULONG,
        pelt: *mut *mut ISpObjectToken,
        pceltFetched: *mut ULONG
    ) -> HRESULT,
    fn Skip(celt: ULONG) -> HRESULT,
    fn Reset() -> HRESULT,
    fn Clone(ppEnum: *mut *mut IEnumSpObjectTokens) -> HRESULT,
    fn Item(
        Index: ULONG,
        ppToken: *mut *mut ISpObjectToken
    ) -> HRESULT,
    fn GetCount(pCount: *mut ULONG) -> HRESULT
});
RIDL!(#[uuid(0x5b559f40, 0xe952, 0x11d2, 0xbb, 0x91, 0x00, 0xc0, 0x4f, 0x8e, 0xe6, 0xc0)]
interface ISpObjectWithToken(ISpObjectWithTokenVtbl): IUnknown(IUnknownVtbl) {
    fn SetObjectToken(pToken: *mut ISpObjectToken) -> HRESULT,
    fn GetObjectToken(ppToken: *mut *mut ISpObjectToken) -> HRESULT
});
RIDL!(#[uuid(0x93384e18, 0x5014, 0x43d5, 0xad, 0xbb, 0xa7, 0x8e, 0x05, 0x59, 0x26, 0xbd)]
interface ISpResourceManager(ISpResourceManagerVtbl): IServiceProvider(IServiceProviderVtbl) {
    fn SetObject(
        guidServiceId: REFGUID,
        pUnkObject: *mut IUnknown
    ) -> HRESULT,
    fn GetObject(
        guidServiceId: REFGUID,
        ObjectCLSID: REFCLSID,
        ObjectIID: REFIID,
        fReleaseWhenLastExternalRefReleased: BOOL,
        ppObject: *mut *mut c_void
    ) -> HRESULT
});
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
    SPEI_MIN_TTS = 1,
    SPEI_MAX_TTS = 15,
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
    SPEI_SR_PRIVATE = 52,
    SPEI_MIN_SR = 34,
    SPEI_MAX_SR = 52,
    SPEI_RESERVED1 = 30,
    SPEI_RESERVED2 = 33,
    SPEI_RESERVED3 = 63,
}}
pub const SPFEI_FLAGCHECK: u64 = (1 << SPEI_RESERVED1 as u64) | (1 << SPEI_RESERVED2 as u64);
pub const SPFEI_ALL_TTS_EVENTS: u64 = 0x000000000000FFFE | SPFEI_FLAGCHECK;
pub const SPFEI_ALL_SR_EVENTS: u64 = 0x003FFFFC00000000 | SPFEI_FLAGCHECK;
pub const SPFEI_ALL_EVENTS: u64 = 0xEFFFFFFFFFFFFFFF;
#[inline]
pub fn SPFEI(SPEI_ord: SPEVENTENUM) -> ULONGLONG {
    (1 << SPEI_ord as ULONGLONG) | SPFEI_FLAGCHECK
}
// TODO: Use BITFIELD! for eEventId and elParamType?
STRUCT!{struct SPEVENT {
    eEventId: WORD,
    elParamType: WORD,
    ulStreamNum: ULONG,
    ullAudioStreamOffset: ULONGLONG,
    wParam: WPARAM,
    lParam: LPARAM,
}}
STRUCT!{struct SPSERIALIZEDEVENT {
    eEventId: WORD,
    elParamType: WORD,
    ulStreamNum: ULONG,
    ullAudioStreamOffset: ULONGLONG,
    SerializedwParam: ULONG,
    SerializedlParam: LONG,
}}
STRUCT!{struct SPSERIALIZEDEVENT64 {
    eEventId: WORD,
    elParamType: WORD,
    ulStreamNum: ULONG,
    ullAudioStreamOffset: ULONGLONG,
    SerializedwParam: ULONGLONG,
    SerializedlParam: LONGLONG,
}}
ENUM!{enum SPINTERFERENCE {
    SPINTERFERENCE_NONE = 0,
    SPINTERFERENCE_NOISE,
    SPINTERFERENCE_NOSIGNAL,
    SPINTERFERENCE_TOOLOUD,
    SPINTERFERENCE_TOOQUIET,
    SPINTERFERENCE_TOOFAST,
    SPINTERFERENCE_TOOSLOW,
    SPINTERFERENCE_LATENCY_WARNING,
    SPINTERFERENCE_LATENCY_TRUNCATE_BEGIN ,
    SPINTERFERENCE_LATENCY_TRUNCATE_END,
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
    ullEventInterest: ULONGLONG,
    ullQueuedInterest: ULONGLONG,
    ulCount: ULONG,
}}
RIDL!(#[uuid(0xbe7a9cce, 0x5f9e, 0x11d2, 0x96, 0x0f, 0x00, 0xc0, 0x4f, 0x8e, 0xe6, 0x28)]
interface ISpEventSource(ISpEventSourceVtbl): ISpNotifySource(ISpNotifySourceVtbl) {
    fn SetInterest(
        ullEventInterest: ULONGLONG,
        ullQueuedInterest: ULONGLONG
    ) -> HRESULT,
    fn GetEvents(
        ulCount: ULONG,
        pEventArray: *mut SPEVENT,
        pulFetched: *mut ULONG
    ) -> HRESULT,
    fn GetInfo(pInfo: *mut SPEVENTSOURCEINFO) -> HRESULT
});
RIDL!(#[uuid(0xbe7a9cc9, 0x5f9e, 0x11d2, 0x96, 0x0f, 0x00, 0xc0, 0x4f, 0x8e, 0xe6, 0x28)]
interface ISpEventSink(ISpEventSinkVtbl): IUnknown(IUnknownVtbl) {
    fn AddEvents(
        pEventArray: *const SPEVENT,
        ulCount: ULONG
    ) -> HRESULT,
    fn GetEventInterest(pullEventInterest: *mut ULONGLONG) -> HRESULT
});
RIDL!(#[uuid(0xbed530be, 0x2606, 0x4f4d, 0xa1, 0xc0, 0x54, 0xc5, 0xcd, 0xa5, 0x56, 0x6f)]
interface ISpStreamFormat(ISpStreamFormatVtbl): IStream(IStreamVtbl) {
    fn GetFormat(
        pguidFormatId: *mut GUID,
        ppCoMemWaveFormatEx: *mut *mut WAVEFORMATEX
    ) -> HRESULT
});
ENUM!{enum SPFILEMODE {
    SPFM_OPEN_READONLY,
    SPFM_OPEN_READWRITE,
    SPFM_CREATE,
    SPFM_CREATE_ALWAYS,
    SPFM_NUM_MODES,
}}
RIDL!(#[uuid(0x12e3cca9, 0x7518, 0x44c5, 0xa5, 0xe7, 0xba, 0x5a, 0x79, 0xcb, 0x92, 0x9e)]
interface ISpStream(ISpStreamVtbl): ISpStreamFormat(ISpStreamFormatVtbl) {
    fn SetBaseStream(
        pStream: *mut IStream,
        rguidFormat: REFGUID,
        pWaveFormatEx: *const WAVEFORMATEX
    ) -> HRESULT,
    fn GetBaseStream(ppStream: *mut *mut IStream) -> HRESULT,
    fn BindToFile(
        pszFileName: LPCWSTR,
        eMode: SPFILEMODE,
        pFormatId: *const GUID,
        pWaveFormatEx: *const WAVEFORMATEX,
        ullEventInterest: ULONGLONG
    ) -> HRESULT,
    fn Close() -> HRESULT
});
RIDL!(#[uuid(0x678a932c, 0xea71, 0x4446, 0x9b, 0x41, 0x78, 0xfd, 0xa6, 0x28, 0x0a, 0x29)]
interface ISpStreamFormatConverter(ISpStreamFormatConverterVtbl)
    : ISpStreamFormat(ISpStreamFormatVtbl) {
    fn SetBaseStream(
        pStream: *mut ISpStreamFormat,
        fSetFormatToBaseStreamFormat: BOOL,
        fWriteToBaseStream: BOOL
    ) -> HRESULT,
    fn GetBaseStream(ppStream: *mut *mut ISpStreamFormat) -> HRESULT,
    fn SetFormat(
        rguidFormatIdOfConvertedStream: REFGUID,
        pWaveFormatExOfConvertedStream: *const WAVEFORMATEX
    ) -> HRESULT,
    fn ResetSeekPosition() -> HRESULT,
    fn ScaleConvertedToBaseOffset(
        ullOffsetConvertedStream: ULONGLONG,
        pullOffsetBaseStream: *mut ULONGLONG
    ) -> HRESULT,
    fn ScaleBaseToConvertedOffset(
        ullOffsetBaseStream: ULONGLONG,
        pullOffsetConvertedStream: *mut ULONGLONG
    ) -> HRESULT
});
ENUM!{enum SPAUDIOSTATE {
    SPAS_CLOSED,
    SPAS_STOP,
    SPAS_PAUSE,
    SPAS_RUN,
}}
STRUCT!{struct SPAUDIOSTATUS {
    cbFreeBuffSpace: c_long,
    cbNonBlockingIO: ULONG,
    State: SPAUDIOSTATE,
    CurSeekPos: ULONGLONG,
    CurDevicePos: ULONGLONG,
    dwAudioLevel: DWORD,
    dwReserved2: DWORD,
}}
STRUCT!{struct SPAUDIOBUFFERINFO {
    ulMsMinNotification: ULONG,
    ulMsBufferSize: ULONG,
    ulMsEventBias: ULONG,
}}
RIDL!(#[uuid(0xc05c768f, 0xfae8, 0x4ec2, 0x8e, 0x07, 0x33, 0x83, 0x21, 0xc1, 0x24, 0x52)]
interface ISpAudio(ISpAudioVtbl): ISpStreamFormat(ISpStreamFormatVtbl) {
    fn SetState(
        NewState: SPAUDIOSTATE,
        ullReserved: ULONGLONG
    ) -> HRESULT,
    fn SetFormat(
        rguidFmtId: REFGUID,
        pWaveFormatEx: *const WAVEFORMATEX
    ) -> HRESULT,
    fn GetStatus(pStatus: *mut SPAUDIOSTATUS) -> HRESULT,
    fn SetBufferInfo(pBuffInfo: *const SPAUDIOBUFFERINFO) -> HRESULT,
    fn GetBufferInfo(pBuffInfo: *mut SPAUDIOBUFFERINFO) -> HRESULT,
    fn GetDefaultFormat(
        pFormatId: *mut GUID,
        ppCoMemWaveFormatEx: *mut *mut WAVEFORMATEX
    ) -> HRESULT,
    fn EventHandle() -> HANDLE,
    fn GetVolumeLevel(pLevel: *mut ULONG) -> HRESULT,
    fn SetVolumeLevel(Level: ULONG) -> HRESULT,
    fn GetBufferNotifySize(pcbSize: *mut ULONG) -> HRESULT,
    fn SetBufferNotifySize(cbSize: ULONG) -> HRESULT
});
RIDL!(#[uuid(0x15806f6e, 0x1d70, 0x4b48, 0x98, 0xe6, 0x3b, 0x1a, 0x00, 0x75, 0x09, 0xab)]
interface ISpMMSysAudio(ISpMMSysAudioVtbl): ISpAudio(ISpAudioVtbl) {
    fn GetDeviceId(puDeviceId: *mut UINT) -> HRESULT,
    fn SetDeviceId(uDeviceId: UINT) -> HRESULT,
    fn GetMMHandle(pHandle: *mut *mut c_void) -> HRESULT,
    fn GetLineId(puLineId: *mut UINT) -> HRESULT,
    fn SetLineId(uLineId: UINT) -> HRESULT
});
RIDL!(#[uuid(0x10f63bce, 0x201a, 0x11d3, 0xac, 0x70, 0x00, 0xc0, 0x4f, 0x8e, 0xe6, 0xc0)]
interface ISpTranscript(ISpTranscriptVtbl): IUnknown(IUnknownVtbl) {
    fn GetTranscript(ppszTranscript: *mut LPWSTR) -> HRESULT,
    fn AppendTranscript(pszTranscript: LPCWSTR) -> HRESULT
});
ENUM!{enum SPDISPLAYATTRIBUTES {
    SPAF_ONE_TRAILING_SPACE = 0x2,
    SPAF_TWO_TRAILING_SPACES = 0x4,
    SPAF_CONSUME_LEADING_SPACES = 0x8,
    SPAF_ALL = 0xf,
}}
pub type SPPHONEID = WCHAR;
pub type PSPPHONEID = LPWSTR;
pub type PCSPPHONEID = LPCWSTR;
STRUCT!{struct SPPHRASEELEMENT {
    ulAudioTimeOffset: ULONG,
    ulAudioSizeTime: ULONG,
    ulAudioStreamOffset: ULONG,
    ulAudioSizeBytes: ULONG,
    ulRetainedStreamOffset: ULONG,
    ulRetainedSizeBytes: ULONG,
    pszDisplayText: LPCWSTR,
    pszLexicalForm: LPCWSTR,
    pszPronunciation: *const SPPHONEID,
    bDisplayAttributes: BYTE,
    RequiredConfidence: c_char,
    ActualConfidence: c_char,
    Reserved: BYTE,
    SREngineConfidence: c_float,
}}
STRUCT!{struct SPPHRASERULE {
    pszName: LPCWSTR,
    ulId: ULONG,
    ulFirstElement: ULONG,
    ulCountOfElements: ULONG,
    pNextSibling: *const SPPHRASERULE,
    pFirstChild: *const SPPHRASERULE,
    SREngineConfidence: c_float,
    Confidence: c_char,
}}
ENUM!{enum SPPHRASEPROPERTYUNIONTYPE {
    SPPPUT_UNUSED = 0,
    SPPPUT_ARRAY_INDEX,
}}
STRUCT!{struct SPPHRASEPROPERTY {
    pszName: LPCWSTR,
    bType: BYTE,
    bReserved: BYTE,
    usArrayIndex: u16,
    pszValue: LPCWSTR,
    vValue: VARIANT,
    ulFirstElement: ULONG,
    ulCountOfElements: ULONG,
    pNextSibling: *const SPPHRASEPROPERTY,
    pFirstChild: *const SPPHRASEPROPERTY,
    SREngineConfidence: c_float,
    Confidence: c_char,
}}
UNION!(SPPHRASEPROPERTY, bType, ulId, ulId_mut, ULONG);
STRUCT!{struct SPPHRASEREPLACEMENT {
    bDisplayAttributes: BYTE,
    pszReplacementText: LPCWSTR,
    ulFirstElement: ULONG,
    ulCountOfElements: ULONG,
}}
STRUCT!{struct SPPHRASE {
    cbSize: ULONG,
    LangID: WORD,
    wHomophoneGroupId: WORD,
    ullGrammarID: ULONGLONG,
    ftStartTime: ULONGLONG,
    ullAudioStreamPosition: ULONGLONG,
    ulAudioSizeBytes: ULONG,
    ulRetainedSizeBytes: ULONG,
    ulAudioSizeTime: ULONG,
    Rule: SPPHRASERULE,
    pProperties: *const SPPHRASEPROPERTY,
    pElements: *const SPPHRASEELEMENT,
    cReplacements: ULONG,
    pReplacements: *const SPPHRASEREPLACEMENT,
    SREngineID: GUID,
    ulSREnginePrivateDataSize: ULONG,
    pSREnginePrivateData: *const BYTE,
}}
STRUCT!{struct SPSERIALIZEDPHRASE {
    ulSerializedSize: ULONG,
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
    ulTotalSerializedSize: ULONG,
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
}}
ENUM!{enum SPPARTOFSPEECH {
    SPPS_NotOverriden = -1i32 as u32,
    SPPS_Unknown = 0,
    SPPS_Noun = 0x1000,
    SPPS_Verb = 0x2000,
    SPPS_Modifier = 0x3000,
    SPPS_Function = 0x4000,
    SPPS_Interjection = 0x5000,
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
STRUCT!{struct SPWORDPRONUNCIATION {
    pNextWordPronunciation: *mut SPWORDPRONUNCIATION,
    eLexiconType: SPLEXICONTYPE,
    LangID: WORD,
    wPronunciationFlags: WORD,
    ePartOfSpeech: SPPARTOFSPEECH,
    szPronunciation: [SPPHONEID; 1],
}}
STRUCT!{struct SPWORDPRONUNCIATIONLIST {
    ulSize: ULONG,
    pvBuffer: *mut BYTE,
    pFirstWordPronunciation: *mut SPWORDPRONUNCIATION,
}}
STRUCT!{struct SPWORD {
    pNextWord: *mut SPWORD,
    LangID: WORD,
    wReserved: WORD,
    eWordType: SPWORDTYPE,
    pszWord: LPWSTR,
    pFirstWordPronunciation: *mut SPWORDPRONUNCIATION,
}}
STRUCT!{struct SPWORDLIST {
    ulSize: ULONG,
    pvBuffer: *mut BYTE,
    pFirstWord: *mut SPWORD,
}}
RIDL!(#[uuid(0xda41a7c2, 0x5383, 0x4db2, 0x91, 0x6b, 0x6c, 0x17, 0x19, 0xe3, 0xdb, 0x58)]
interface ISpLexicon(ISpLexiconVtbl): IUnknown(IUnknownVtbl) {
    fn GetPronunciations(
        pszWord: LPCWSTR,
        LangID: WORD,
        dwFlags: DWORD,
        pWordPronunciationList: *mut SPWORDPRONUNCIATIONLIST
    ) -> HRESULT,
    fn AddPronunciation(
        pszWord: LPCWSTR,
        LangID: WORD,
        ePartOfSpeech: SPPARTOFSPEECH,
        pszPronunciation: PCSPPHONEID
    ) -> HRESULT,
    fn RemovePronunciation(
        pszWord: LPCWSTR,
        LangID: WORD,
        ePartOfSpeech: SPPARTOFSPEECH,
        pszPronunciation: PCSPPHONEID
    ) -> HRESULT,
    fn GetGeneration(pdwGeneration: *mut DWORD) -> HRESULT,
    fn GetGenerationChange(
        dwFlags: DWORD,
        pdwGeneration: *mut DWORD,
        pWordList: *mut SPWORDLIST
    ) -> HRESULT,
    fn GetWords(
        dwFlags: DWORD,
        pdwGeneration: *mut DWORD,
        pdwCookie: *mut DWORD,
        pWordList: *mut SPWORDLIST
    ) -> HRESULT
});
RIDL!(#[uuid(0x8565572f, 0xc094, 0x41cc, 0xb5, 0x6e, 0x10, 0xbd, 0x9c, 0x3f, 0xf0, 0x44)]
interface ISpContainerLexicon(ISpContainerLexiconVtbl): ISpLexicon(ISpLexiconVtbl) {
    fn AddLexicon(
        pAddLexicon: *mut ISpLexicon,
        dwFlags: DWORD
    ) -> HRESULT
});
RIDL!(#[uuid(0x8445c581, 0x0cac, 0x4a38, 0xab, 0xfe, 0x9b, 0x2c, 0xe2, 0x82, 0x64, 0x55)]
interface ISpPhoneConverter(ISpPhoneConverterVtbl): ISpObjectWithToken(ISpObjectWithTokenVtbl) {
    fn PhoneToId(
        pszPhone: LPCWSTR,
        pId: *mut SPPHONEID
    ) -> HRESULT,
    fn IdToPhone(
        pId: PCSPPHONEID,
        pszPhone: *mut WCHAR
    ) -> HRESULT
});
STRUCT!{struct SPVPITCH {
    MiddleAdj: c_long,
    RangeAdj: c_long,
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
    pCategory: LPCWSTR,
    pBefore: LPCWSTR,
    pAfter: LPCWSTR,
}}
STRUCT!{struct SPVSTATE {
    eAction: SPVACTIONS,
    LangID: WORD,
    wReserved: WORD,
    EmphAdj: c_long,
    RateAdj: c_long,
    Volume: ULONG,
    PitchAdj: SPVPITCH,
    SilenceMSecs: ULONG,
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
    ulCurrentStream: ULONG,
    ulLastStreamQueued: ULONG,
    hrLastResult: HRESULT,
    dwRunningState: DWORD,
    ulInputWordPos: ULONG,
    ulInputWordLen: ULONG,
    ulInputSentPos: ULONG,
    ulInputSentLen: ULONG,
    lBookmarkId: LONG,
    PhonemeId: SPPHONEID,
    VisemeId: SPVISEMES,
    dwReserved1: DWORD,
    dwReserved2: DWORD,
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
}}
pub const SPF_NLP_MASK: i32 = SPF_NLP_SPEAK_PUNC as i32;
pub const SPF_VOICE_MASK: i32 = SPF_ASYNC as i32 | SPF_PURGEBEFORESPEAK as i32
    | SPF_IS_FILENAME as i32 | SPF_IS_XML as i32 | SPF_IS_NOT_XML as i32
    | SPF_NLP_MASK as i32 | SPF_PERSIST_XML as i32;
pub const SPF_UNUSED_FLAGS: i32 = !SPF_VOICE_MASK;
RIDL!(#[uuid(0x6c44df74, 0x72b9, 0x4992, 0xa1, 0xec, 0xef, 0x99, 0x6e, 0x04, 0x22, 0xd4)]
interface ISpVoice(ISpVoiceVtbl): ISpEventSource(ISpEventSourceVtbl) {
    fn SetOutput(
        pUnkOutput: *mut IUnknown,
        fAllowFormatChanges: BOOL
    ) -> HRESULT,
    fn GetOutputObjectToken(ppObjectToken: *mut *mut ISpObjectToken) -> HRESULT,
    fn GetOutputStream(ppStream: *mut *mut ISpStreamFormat) -> HRESULT,
    fn Pause() -> HRESULT,
    fn Resume() -> HRESULT,
    fn SetVoice(pToken: *mut ISpObjectToken) -> HRESULT,
    fn GetVoice(ppToken: *mut *mut ISpObjectToken) -> HRESULT,
    fn Speak(
        pwcs: LPCWSTR,
        dwFlags: DWORD,
        pulStreamNumber: *mut ULONG
    ) -> HRESULT,
    fn SpeakStream(
        pStream: *mut IStream,
        dwFlags: DWORD,
        pulStreamNumber: *mut ULONG
    ) -> HRESULT,
    fn GetStatus(
        pStatus: *mut SPVOICESTATUS,
        ppszLastBookmark: *mut LPWSTR
    ) -> HRESULT,
    fn Skip(
        pItemType: LPCWSTR,
        lNumItems: c_long,
        pulNumSkipped: *mut ULONG
    ) -> HRESULT,
    fn SetPriority(ePriority: SPVPRIORITY) -> HRESULT,
    fn GetPriority(pePriority: *mut SPVPRIORITY) -> HRESULT,
    fn SetAlertBoundary(eBoundary: SPEVENTENUM) -> HRESULT,
    fn GetAlertBoundary(peBoundary: *mut SPEVENTENUM) -> HRESULT,
    fn SetRate(RateAdjust: c_long) -> HRESULT,
    fn GetRate(pRateAdjust: *mut c_long) -> HRESULT,
    fn SetVolume(usVolume: USHORT) -> HRESULT,
    fn GetVolume(pusVolume: *mut USHORT) -> HRESULT,
    fn WaitUntilDone(msTimeout: ULONG) -> HRESULT,
    fn SetSyncSpeakTimeout(msTimeout: ULONG) -> HRESULT,
    fn GetSyncSpeakTimeout(pmsTimeout: *mut ULONG) -> HRESULT,
    fn SpeakCompleteEvent() -> HANDLE,
    fn IsUISupported(
        pszTypeOfUI: LPCWSTR,
        pvExtraData: *mut c_void,
        cbExtraData: ULONG,
        pfSupported: *mut BOOL
    ) -> HRESULT,
    fn DisplayUI(
        hwndParent: HWND,
        pszTitle: LPCWSTR,
        pszTypeOfUI: LPCWSTR,
        pvExtraData: *mut c_void,
        cbExtraData: ULONG
    ) -> HRESULT
});
RIDL!(#[uuid(0x1a5c0354, 0xb621, 0x4b5a, 0x87, 0x91, 0xd3, 0x06, 0xed, 0x37, 0x9e, 0x53)]
interface ISpPhrase(ISpPhraseVtbl): IUnknown(IUnknownVtbl) {
    fn GetPhrase(ppCoMemPhrase: *mut *mut SPPHRASE) -> HRESULT,
    fn GetSerializedPhrase(ppCoMemPhrase: *mut *mut SPSERIALIZEDPHRASE) -> HRESULT,
    fn GetText(
        ulStart: ULONG,
        ulCount: ULONG,
        fUseTextReplacements: BOOL,
        ppszCoMemText: *mut LPWSTR,
        pbDisplayAttributes: *mut BYTE
    ) -> HRESULT,
    fn Discard(dwValueTypes: DWORD) -> HRESULT
});
RIDL!(#[uuid(0x8fcebc98, 0x4e49, 0x4067, 0x9c, 0x6c, 0xd8, 0x6a, 0x0e, 0x09, 0x2e, 0x3d)]
interface ISpPhraseAlt(ISpPhraseAltVtbl): ISpPhrase(ISpPhraseVtbl) {
    fn GetAltInfo(
        pParent: *mut *mut ISpPhrase,
        pulStartElementInParent: *mut ULONG,
        pcElementsInParent: *mut ULONG,
        pcElementsInAlt: *mut ULONG
    ) -> HRESULT,
    fn Commit() -> HRESULT
});
STRUCT!{struct SPRECORESULTTIMES {
    ftStreamTime: FILETIME,
    ullLength: ULONGLONG,
    dwTickCount: DWORD,
    ullStart: ULONGLONG,
}}
STRUCT!{struct SPSERIALIZEDRESULT {
    ulSerializedSize: ULONG,
}}
RIDL!(#[uuid(0x20b053be, 0xe235, 0x43cd, 0x9a, 0x2a, 0x8d, 0x17, 0xa4, 0x8b, 0x78, 0x42)]
interface ISpRecoResult(ISpRecoResultVtbl): ISpPhrase(ISpPhraseVtbl) {
    fn GetResultTimes(pTimes: *mut SPRECORESULTTIMES) -> HRESULT,
    fn GetAlternates(
        ulStartElement: ULONG,
        cElements: ULONG,
        ulRequestCount: ULONG,
        ppPhrases: *mut *mut ISpPhraseAlt,
        pcPhrasesReturned: *mut ULONG
    ) -> HRESULT,
    fn GetAudio(
        ulStartElement: ULONG,
        cElements: ULONG,
        ppStream: *mut *mut ISpStreamFormat
    ) -> HRESULT,
    fn SpeakAudio(
        ulStartElement: ULONG,
        cElements: ULONG,
        dwFlags: DWORD,
        pulStreamNumber: *mut ULONG
    ) -> HRESULT,
    fn Serialize(ppCoMemSerializedResult: *mut *mut SPSERIALIZEDRESULT) -> HRESULT,
    fn ScaleAudio(
        pAudioFormatId: *const GUID,
        pWaveFormatEx: *const WAVEFORMATEX
    ) -> HRESULT,
    fn GetRecoContext(ppRecoContext: *mut *mut ISpRecoContext) -> HRESULT
});
STRUCT!{struct SPTEXTSELECTIONINFO {
    ulStartActiveOffset: ULONG,
    cchActiveChars: ULONG,
    ulStartSelection: ULONG,
    cchSelection: ULONG,
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
}}
pub const SP_STREAMPOS_ASAP: ULONGLONG = 0;
pub const SP_STREAMPOS_REALTIME: ULONGLONG = -1i64 as ULONGLONG;
pub const SPRULETRANS_TEXTBUFFER: SPSTATEHANDLE = -1isize as SPSTATEHANDLE;
pub const SPRULETRANS_WILDCARD: SPSTATEHANDLE = -2isize as SPSTATEHANDLE;
pub const SPRULETRANS_DICTATION: SPSTATEHANDLE = -3isize as SPSTATEHANDLE;
ENUM!{enum SPGRAMMARWORDTYPE {
    SPWT_DISPLAY,
    SPWT_LEXICAL,
    SPWT_PRONUNCIATION,
    SPWT_LEXICAL_NO_SPECIAL_CHARS,
}}
STRUCT!{struct SPPROPERTYINFO {
    pszName: LPCWSTR,
    ulId: ULONG,
    pszValue: LPCWSTR,
    vValue: VARIANT,
}}
ENUM!{enum SPCFGRULEATTRIBUTES {
    SPRAF_TopLevel = 1 << 0,
    SPRAF_Active = 1 << 1,
    SPRAF_Export = 1 << 2,
    SPRAF_Import = 1 << 3,
    SPRAF_Interpreter = 1 << 4,
    SPRAF_Dynamic = 1 << 5,
    SPRAF_AutoPause = 1 << 16,
}}
RIDL!(#[uuid(0x8137828f, 0x591a, 0x4a42, 0xbe, 0x58, 0x49, 0xea, 0x7e, 0xba, 0xac, 0x68)]
interface ISpGrammarBuilder(ISpGrammarBuilderVtbl): IUnknown(IUnknownVtbl) {
    fn ResetGrammar(NewLanguage: WORD) -> HRESULT,
    fn GetRule(
        pszRuleName: LPCWSTR,
        dwRuleId: DWORD,
        dwAttributes: DWORD,
        fCreateIfNotExist: BOOL,
        phInitialState: *mut SPSTATEHANDLE
    ) -> HRESULT,
    fn ClearRule(hState: SPSTATEHANDLE) -> HRESULT,
    fn CreateNewState(
        hState: SPSTATEHANDLE,
        phState: *mut SPSTATEHANDLE
    ) -> HRESULT,
    fn AddWordTransition(
        hFromState: SPSTATEHANDLE,
        hToState: SPSTATEHANDLE,
        psz: LPCWSTR,
        pszSeparators: LPCWSTR,
        eWordType: SPGRAMMARWORDTYPE,
        Weight: c_float,
        pPropInfo: *const SPPROPERTYINFO
    ) -> HRESULT,
    fn AddRuleTransition(
        hFromState: SPSTATEHANDLE,
        hToState: SPSTATEHANDLE,
        hRule: SPSTATEHANDLE,
        Weight: c_float,
        pPropInfo: *const SPPROPERTYINFO
    ) -> HRESULT,
    fn AddResource(
        hRuleState: SPSTATEHANDLE,
        pszResourceName: LPCWSTR,
        pszResourceValue: LPCWSTR
    ) -> HRESULT,
    fn Commit(dwReserved: DWORD) -> HRESULT
});
ENUM!{enum SPLOADOPTIONS {
    SPLO_STATIC = 0,
    SPLO_DYNAMIC = 1,
}}
RIDL!(#[uuid(0x2177db29, 0x7f45, 0x47d0, 0x85, 0x54, 0x06, 0x7e, 0x91, 0xc8, 0x05, 0x02)]
interface ISpRecoGrammar(ISpRecoGrammarVtbl): ISpGrammarBuilder(ISpGrammarBuilderVtbl) {
    fn GetGrammarId(pullGrammarId: *mut ULONGLONG) -> HRESULT,
    fn GetRecoContext(ppRecoCtxt: *mut *mut ISpRecoContext) -> HRESULT,
    fn LoadCmdFromFile(
        pszFileName: LPCWSTR,
        Options: SPLOADOPTIONS
    ) -> HRESULT,
    fn LoadCmdFromObject(
        rcid: REFCLSID,
        pszGrammarName: LPCWSTR,
        Options: SPLOADOPTIONS
    ) -> HRESULT,
    fn LoadCmdFromResource(
        hModule: HMODULE,
        pszResourceName: LPCWSTR,
        pszResourceType: LPCWSTR,
        wLanguage: WORD,
        Options: SPLOADOPTIONS
    ) -> HRESULT,
    fn LoadCmdFromMemory(
        pGrammar: *const SPBINARYGRAMMAR,
        Options: SPLOADOPTIONS
    ) -> HRESULT,
    fn LoadCmdFromProprietaryGrammar(
        rguidParam: REFGUID,
        pszStringParam: LPCWSTR,
        pvDataPrarm: *const c_void,
        cbDataSize: ULONG,
        Options: SPLOADOPTIONS
    ) -> HRESULT,
    fn SetRuleState(
        pszName: LPCWSTR,
        pReserved: *mut c_void,
        NewState: SPRULESTATE
    ) -> HRESULT,
    fn SetRuleIdState(
        ulRuleId: ULONG,
        NewState: SPRULESTATE
    ) -> HRESULT,
    fn LoadDictation(
        pszTopicName: LPCWSTR,
        Options: SPLOADOPTIONS
    ) -> HRESULT,
    fn UnloadDictation() -> HRESULT,
    fn SetDictationState(NewState: SPRULESTATE) -> HRESULT,
    fn SetWordSequenceData(
        pText: *const WCHAR,
        cchText: ULONG,
        pInfo: *const SPTEXTSELECTIONINFO
    ) -> HRESULT,
    fn SetTextSelection(pInfo: *const SPTEXTSELECTIONINFO) -> HRESULT,
    fn IsPronounceable(
        pszWord: LPCWSTR,
        pWordPronounceable: *mut SPWORDPRONOUNCEABLE
    ) -> HRESULT,
    fn SetGrammarState(eGrammarState: SPGRAMMARSTATE) -> HRESULT,
    fn SaveCmd(
        pStream: *mut IStream,
        ppszCoMemErrorText: *mut LPWSTR
    ) -> HRESULT,
    fn GetGrammarState(peGrammarState: *mut SPGRAMMARSTATE) -> HRESULT
});
STRUCT!{struct SPRECOCONTEXTSTATUS {
    eInterference: SPINTERFERENCE,
    szRequestTypeOfUI: [WCHAR; 255],
    dwReserved1: DWORD,
    dwReserved2: DWORD,
}}
ENUM!{enum SPBOOKMARKOPTIONS {
    SPBO_NONE = 0,
    SPBO_PAUSE = 1 << 0,
}}
ENUM!{enum SPAUDIOOPTIONS {
    SPAO_NONE = 0,
    SPAO_RETAIN_AUDIO = 1 << 0,
}}
RIDL!(#[uuid(0xf740a62f, 0x7c15, 0x489e, 0x82, 0x34, 0x94, 0x0a, 0x33, 0xd9, 0x27, 0x2d)]
interface ISpRecoContext(ISpRecoContextVtbl): ISpEventSource(ISpEventSourceVtbl) {
    fn GetRecognizer(ppRecognizer: *mut *mut ISpRecognizer) -> HRESULT,
    fn CreateGrammer(
        ullGrammarId: ULONGLONG,
        ppGrammar: *mut *mut ISpRecoGrammar
    ) -> HRESULT,
    fn GetStatus(pState: *mut SPRECOCONTEXTSTATUS) -> HRESULT,
    fn GetMaxAlternates(pcAlternates: *mut ULONG) -> HRESULT,
    fn SetMaxAlternates(cAlternates: ULONG) -> HRESULT,
    fn SetAudioOptions(
        Options: SPAUDIOOPTIONS,
        pAudioFormatId: *const GUID,
        pWaveFormatEx: *const WAVEFORMATEX
    ) -> HRESULT,
    fn GetAudioOptions(
        pOptions: *mut SPAUDIOOPTIONS,
        pAudioFormatId: *mut GUID,
        ppCoMemWFEX: *mut *mut WAVEFORMATEX
    ) -> HRESULT,
    fn DeserializeResult(
        pSerializedResult: *const SPSERIALIZEDRESULT,
        ppResult: *mut *mut ISpRecoResult
    ) -> HRESULT,
    fn Bookmark(
        Options: SPBOOKMARKOPTIONS,
        ullStreamPosition: ULONGLONG,
        lparamEvent: LPARAM
    ) -> HRESULT,
    fn SetAdaptionData(
        pAdaptionData: LPCWSTR,
        cch: ULONG
    ) -> HRESULT,
    fn Pause(dwReserved: DWORD) -> HRESULT,
    fn Resume(dwReserved: DWORD) -> HRESULT,
    fn SetVoice(
        pVoice: *mut ISpVoice,
        fAllowFormatChanges: BOOL
    ) -> HRESULT,
    fn GetVoice(ppVoice: *mut *mut ISpVoice) -> HRESULT,
    fn SetVoicePurgeEvent(ullEventIntereset: ULONGLONG) -> HRESULT,
    fn GetVoicePurgeEvent(pullEventIntereset: *mut ULONGLONG) -> HRESULT,
    fn SetContextState(eContextState: SPCONTEXTSTATE) -> HRESULT,
    fn GetContextState(peContextState: *mut SPCONTEXTSTATE) -> HRESULT
});
RIDL!(#[uuid(0x5b4fb971, 0xb115, 0x4de1, 0xad, 0x97, 0xe4, 0x82, 0xe3, 0xbf, 0x6e, 0xe4)]
interface ISpProperties(ISpPropertiesVtbl): IUnknown(IUnknownVtbl) {
    fn SetPropertyNum(
        pName: LPCWSTR,
        lValue: LONG
    ) -> HRESULT,
    fn GetPropertyNum(
        pName: LPCWSTR,
        plValue: *mut LONG
    ) -> HRESULT,
    fn SetPropertyString(
        pName: LPCWSTR,
        pValue: LPCWSTR
    ) -> HRESULT,
    fn GetPropertyString(
        pName: LPCWSTR,
        ppCoMemValue: *mut LPWSTR
    ) -> HRESULT
});
pub const SP_MAX_LANGIDS: ULONG = 20;
STRUCT!{struct SPRECOGNIZERSTATUS {
    AudioStatus: SPAUDIOSTATUS,
    ullRecognitionStreamPos: ULONGLONG,
    ulStreamNumber: ULONG,
    ulNumActive: ULONG,
    clsidEngine: CLSID,
    cLangIDs: ULONG,
    aLangID: [WORD; SP_MAX_LANGIDS as usize],
    ullRecognitionStreamTime: ULONGLONG,
}}
ENUM!{enum SPWAVEFORMATTYPE {
    SPWF_INPUT,
    SPWF_SRENGINE,
}}
pub type SPSTREAMFORMATTYPE = SPWAVEFORMATTYPE;
ENUM!{enum SPRECOSTATE {
    SPRST_INACTIVE,
    SPRST_ACTIVE,
    SPRST_ACTIVE_ALWAYS,
    SPRST_INACTIVE_WITH_PURGE,
    SPRST_NUM_STATES,
}}
RIDL!(#[uuid(0xc2b5f241, 0xdaa0, 0x4507, 0x9e, 0x16, 0x5a, 0x1e, 0xaa, 0x2b, 0x7a, 0x5c)]
interface ISpRecognizer(ISpRecognizerVtbl): ISpProperties(ISpPropertiesVtbl) {
    fn SetRecognizer(pRecognizer: *mut ISpObjectToken) -> HRESULT,
    fn GetRecognizer(ppRecognizer: *mut *mut ISpObjectToken) -> HRESULT,
    fn SetInput(
        pUnkInput: *mut IUnknown,
        fAllowFormatChanges: BOOL
    ) -> HRESULT,
    fn GetInputObjectToken(ppToken: *mut *mut ISpObjectToken) -> HRESULT,
    fn GetInputStream(ppStream: *mut *mut ISpStreamFormat) -> HRESULT,
    fn CreateRecoContext(ppNewCtxt: *mut *mut ISpRecoContext) -> HRESULT,
    fn GetRecoProfile(ppToken: *mut *mut ISpObjectToken) -> HRESULT,
    fn SetRecoProfile(pToken: *mut ISpObjectToken) -> HRESULT,
    fn IsSharedInstance() -> HRESULT,
    fn GetRecoState(pState: *mut SPRECOSTATE) -> HRESULT,
    fn SetRecoState(NewState: SPRECOSTATE) -> HRESULT,
    fn GetStatus(pStatus: *mut SPRECOGNIZERSTATUS) -> HRESULT,
    fn GetFormat(
        WaveFormatType: SPSTREAMFORMATTYPE,
        pFormatId: *mut GUID,
        ppCoMemWFEX: *mut WAVEFORMATEX
    ) -> HRESULT,
    fn IsUISupported(
        pszTypeOfUI: LPCWSTR,
        pvExtraData: *mut c_void,
        cbExtraData: ULONG,
        pfSupported: *mut BOOL
    ) -> HRESULT,
    fn DisplayUI(
        hwndParent: HWND,
        pszTitle: LPCWSTR,
        pszTypeOfUI: LPCWSTR,
        pvExtraData: *mut c_void,
        cbExtraData: ULONG
    ) -> HRESULT,
    fn EmulateRecognition(pPhrase: *mut ISpPhrase) -> HRESULT
});

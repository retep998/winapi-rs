// Copyright © 2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! SAPI 5.3 definitions
use ctypes::{c_float, c_int, c_long};
use shared::guiddef::{CLSID, GUID};
use shared::minwindef::{BOOL, BYTE, DWORD, LPARAM, UINT, ULONG, WORD, WPARAM};
use shared::wtypes::{BSTR, VARIANT_BOOL};
use um::oaidl::{IDispatch, IDispatchVtbl, VARIANT};
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::urlmon::IInternetSecurityManager;
use um::winnt::{HRESULT, LPCWSTR, LPWSTR, ULONGLONG, WCHAR};
pub use um::sapi51::{
    SPDATAKEYLOCATION,
    SPDKL_DefaultLocation,
    SPDKL_CurrentUser,
    SPDKL_LocalMachine,
    SPDKL_CurrentConfig,
    SPDUI_EngineProperties,
    SPDUI_AddRemoveWord,
    SPDUI_UserTraining,
    SPDUI_MicTraining,
    SPDUI_RecoProfileProperties,
    SPDUI_AudioProperties,
    SPDUI_AudioVolume,
    SPDUI_UserEnrollment,
    SPDUI_ShareData,
    SPDUI_Tutorial,
    SPSTREAMFORMAT,
    SPSF_Default,
    SPSF_NoAssignedFormat,
    SPSF_Text,
    SPSF_NonStandardFormat,
    SPSF_ExtendedAudioFormat,
    SPSF_8kHz8BitMono,
    SPSF_8kHz8BitStereo,
    SPSF_8kHz16BitMono,
    SPSF_8kHz16BitStereo,
    SPSF_11kHz8BitMono,
    SPSF_11kHz8BitStereo,
    SPSF_11kHz16BitMono,
    SPSF_11kHz16BitStereo,
    SPSF_12kHz8BitMono,
    SPSF_12kHz8BitStereo,
    SPSF_12kHz16BitMono,
    SPSF_12kHz16BitStereo,
    SPSF_16kHz8BitMono,
    SPSF_16kHz8BitStereo,
    SPSF_16kHz16BitMono,
    SPSF_16kHz16BitStereo,
    SPSF_22kHz8BitMono,
    SPSF_22kHz8BitStereo,
    SPSF_22kHz16BitMono,
    SPSF_22kHz16BitStereo,
    SPSF_24kHz8BitMono,
    SPSF_24kHz8BitStereo,
    SPSF_24kHz16BitMono,
    SPSF_24kHz16BitStereo,
    SPSF_32kHz8BitMono,
    SPSF_32kHz8BitStereo,
    SPSF_32kHz16BitMono,
    SPSF_32kHz16BitStereo,
    SPSF_44kHz8BitMono,
    SPSF_44kHz8BitStereo,
    SPSF_44kHz16BitMono,
    SPSF_44kHz16BitStereo,
    SPSF_48kHz8BitMono,
    SPSF_48kHz8BitStereo,
    SPSF_48kHz16BitMono,
    SPSF_48kHz16BitStereo,
    SPSF_TrueSpeech_8kHz1BitMono,
    SPSF_CCITT_ALaw_8kHzMono,
    SPSF_CCITT_ALaw_8kHzStereo,
    SPSF_CCITT_ALaw_11kHzMono,
    SPSF_CCITT_ALaw_11kHzStereo,
    SPSF_CCITT_ALaw_22kHzMono,
    SPSF_CCITT_ALaw_22kHzStereo,
    SPSF_CCITT_ALaw_44kHzMono,
    SPSF_CCITT_ALaw_44kHzStereo,
    SPSF_CCITT_uLaw_8kHzMono,
    SPSF_CCITT_uLaw_8kHzStereo,
    SPSF_CCITT_uLaw_11kHzMono,
    SPSF_CCITT_uLaw_11kHzStereo,
    SPSF_CCITT_uLaw_22kHzMono,
    SPSF_CCITT_uLaw_22kHzStereo,
    SPSF_CCITT_uLaw_44kHzMono,
    SPSF_CCITT_uLaw_44kHzStereo,
    SPSF_ADPCM_8kHzMono,
    SPSF_ADPCM_8kHzStereo,
    SPSF_ADPCM_11kHzMono,
    SPSF_ADPCM_11kHzStereo,
    SPSF_ADPCM_22kHzMono,
    SPSF_ADPCM_22kHzStereo,
    SPSF_ADPCM_44kHzMono,
    SPSF_ADPCM_44kHzStereo,
    SPSF_GSM610_8kHzMono,
    SPSF_GSM610_11kHzMono,
    SPSF_GSM610_22kHzMono,
    SPSF_GSM610_44kHzMono,
    SPSF_NUM_FORMATS,
    SPDFID_Text,
    SPDFID_WaveFormatEx,
    SPREG_USER_ROOT,
    SPREG_LOCAL_MACHINE_ROOT,
    SPCAT_AUDIOOUT,
    SPCAT_AUDIOIN,
    SPCAT_VOICES,
    SPCAT_RECOGNIZERS,
    SPCAT_APPLEXICONS,
    SPCAT_PHONECONVERTERS,
    SPCAT_TEXTNORMALIZERS,
    SPCAT_RECOPROFILES,
    SPMMSYS_AUDIO_IN_TOKEN_ID,
    SPMMSYS_AUDIO_OUT_TOKEN_ID,
    SPCURRENT_USER_LEXICON_TOKEN_ID,
    SPTOKENVALUE_CLSID,
    SPTOKENKEY_FILES,
    SPTOKENKEY_UI,
    SPTOKENKEY_ATTRIBUTES
};
pub const SPTOKENKEY_RETAINEDAUDIO: &'static str = "SecondsPerRetainedAudioEvent";
pub const SPTOKENKEY_AUDIO_LATENCY_WARNING: &'static str = "LatencyWarningThreshold";
pub const SPTOKENKEY_AUDIO_LATENCY_TRUNCATE: &'static str = "LatencyTruncateThreshold";
pub const SPTOKENKEY_AUDIO_LATENCY_UPDATE_INTERVAL: &'static str = "LatencyUpdateInterval";
pub use um::sapi51::{
    SPVOICECATEGORY_TTSRATE,
    SPPROP_RESOURCE_USAGE,
    SPPROP_HIGH_CONFIDENCE_THRESHOLD,
    SPPROP_NORMAL_CONFIDENCE_THRESHOLD,
    SPPROP_LOW_CONFIDENCE_THRESHOLD,
    SPPROP_RESPONSE_SPEED,
    SPPROP_COMPLEX_RESPONSE_SPEED,
    SPPROP_ADAPTATION_ON,
    SPPROP_PERSISTED_BACKGROUND_ADAPTATION,
    SPPROP_PERSISTED_LANGUAGE_MODEL_ADAPTATION,
    SPPROP_UX_IS_LISTENING,
    SPTOPIC_SPELLING,
    SPWILDCARD,
    SPDICTATION
};
pub const SPREG_SAFE_USER_TOKENS: &'static str = "HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Speech\\UserTokens";
pub use um::sapi51::{
    SPINFDICTATION,
    SP_LOW_CONFIDENCE,
    SP_NORMAL_CONFIDENCE,
    SP_HIGH_CONFIDENCE,
    DEFAULT_WEIGHT,
    SP_MAX_WORD_LENGTH,
    SP_MAX_PRON_LENGTH
};
pub const SP_EMULATE_RESULT: ULONG = 0x40000000;
pub use um::sapi51::{
    ISpNotifyCallback,
    SPNOTIFYCALLBACK,
    ISpNotifySource, ISpNotifySourceVtbl,
    ISpNotifySink, ISpNotifySinkVtbl,
    ISpNotifyTranslator, ISpNotifyTranslatorVtbl,
    ISpDataKey, ISpDataKeyVtbl,
    ISpRegDataKey, ISpRegDataKeyVtbl,
    ISpObjectTokenCategory, ISpObjectTokenCategoryVtbl,
    ISpObjectToken, ISpObjectTokenVtbl,
    ISpObjectTokenInit, ISpObjectTokenInitVtbl,
    IEnumSpObjectTokens, IEnumSpObjectTokensVtbl,
    ISpObjectWithToken, ISpObjectWithTokenVtbl,
    ISpResourceManager, ISpResourceManagerVtbl,
    SPEVENTLPARAMTYPE,
    SPET_LPARAM_IS_UNDEFINED,
    SPET_LPARAM_IS_TOKEN,
    SPET_LPARAM_IS_OBJECT,
    SPET_LPARAM_IS_POINTER,
    SPET_LPARAM_IS_STRING,
    SPEVENTENUM,
    SPEI_UNDEFINED,
    SPEI_START_INPUT_STREAM,
    SPEI_END_INPUT_STREAM,
    SPEI_VOICE_CHANGE,
    SPEI_TTS_BOOKMARK,
    SPEI_WORD_BOUNDARY,
    SPEI_PHONEME,
    SPEI_SENTENCE_BOUNDARY,
    SPEI_VISEME,
    SPEI_TTS_AUDIO_LEVEL,
    SPEI_TTS_PRIVATE,
    SPEI_MIN_TTS,
    SPEI_MAX_TTS,
    SPEI_END_SR_STREAM,
    SPEI_SOUND_START,
    SPEI_SOUND_END,
    SPEI_PHRASE_START,
    SPEI_RECOGNITION,
    SPEI_HYPOTHESIS,
    SPEI_SR_BOOKMARK,
    SPEI_PROPERTY_NUM_CHANGE,
    SPEI_PROPERTY_STRING_CHANGE,
    SPEI_FALSE_RECOGNITION,
    SPEI_INTERFERENCE,
    SPEI_REQUEST_UI,
    SPEI_RECO_STATE_CHANGE,
    SPEI_ADAPTATION,
    SPEI_START_SR_STREAM,
    SPEI_RECO_OTHER_CONTEXT,
    SPEI_SR_AUDIO_LEVEL
};
pub const SPEI_SR_RETAINEDAUDIO: SPEVENTENUM = 51;
pub use um::sapi51::SPEI_SR_PRIVATE;
pub const SPEI_RESERVED4: SPEVENTENUM = 53;
pub const SPEI_RESERVED5: SPEVENTENUM = 54;
pub const SPEI_RESERVED6: SPEVENTENUM = 55;
pub use um::sapi51::SPEI_MIN_SR;
pub const SPEI_MAX_SR: SPEVENTENUM = 55;
pub use um::sapi51::{
    SPEI_RESERVED1,
    SPEI_RESERVED2,
    SPEI_RESERVED3,
    SPFEI_FLAGCHECK,
    SPFEI_ALL_TTS_EVENTS,
    SPFEI_ALL_SR_EVENTS,
    SPFEI_ALL_EVENTS,
    SPFEI,
    SPEVENT,
    SPSERIALIZEDEVENT,
    SPSERIALIZEDEVENT64
};
STRUCT!{struct SPEVENTEX {
    eEventId: WORD,
    elParamType: WORD,
    ulStreamNum: ULONG,
    ullAudioStreamOffset: ULONGLONG,
    wParam: WPARAM,
    lParam: LPARAM,
    ullAudioTimeOffset: ULONGLONG,
}}
pub use um::sapi51::{
    SPINTERFERENCE,
    SPINTERFERENCE_NONE,
    SPINTERFERENCE_NOISE,
    SPINTERFERENCE_NOSIGNAL,
    SPINTERFERENCE_TOOLOUD,
    SPINTERFERENCE_TOOQUIET,
    SPINTERFERENCE_TOOFAST,
    SPINTERFERENCE_TOOSLOW,
    SPINTERFERENCE_LATENCY_WARNING,
    SPINTERFERENCE_LATENCY_TRUNCATE_BEGIN,
    SPINTERFERENCE_LATENCY_TRUNCATE_END,
    SPENDSRSTREAMFLAGS,
    SPESF_NONE,
    SPESF_STREAM_RELEASED
};
pub const SPESF_EMULATED: SPENDSRSTREAMFLAGS = 1 << 1;
pub use um::sapi51::{
    SPVFEATURE,
    SPVFEATURE_STRESSED,
    SPVFEATURE_EMPHASIS,
    SPVISEMES,
    SP_VISEME_0,
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
    SPEVENTSOURCEINFO,
    ISpEventSource, ISpEventSourceVtbl,
};
RIDL!(#[uuid(0x2373a435, 0x6a4b, 0x429e, 0xa6, 0xac, 0xd4, 0x23, 0x1a, 0x61, 0x97, 0x5b)]
interface ISpEventSource2(ISpEventSource2Vtbl): ISpEventSource(ISpEventSourceVtbl) {
    fn GetEventsEx(
        ulCount: ULONG,
        pEventArray: *mut SPEVENTEX,
        pulFetched: *mut ULONG
    ) -> HRESULT
});
pub use um::sapi51::{
    ISpEventSink, ISpEventSinkVtbl,
    ISpStreamFormat, ISpStreamFormatVtbl,
    SPFILEMODE,
    SPFM_OPEN_READONLY,
    SPFM_OPEN_READWRITE,
    SPFM_CREATE,
    SPFM_CREATE_ALWAYS,
    SPFM_NUM_MODES,
    ISpStream, ISpStreamVtbl,
    ISpStreamFormatConverter, ISpStreamFormatConverterVtbl,
    SPAUDIOSTATE,
    SPAS_CLOSED,
    SPAS_STOP,
    SPAS_PAUSE,
    SPAS_RUN,
    SPAUDIOSTATUS,
    SPAUDIOBUFFERINFO,
    ISpAudio, ISpAudioVtbl,
    ISpMMSysAudio, ISpMMSysAudioVtbl,
    ISpTranscript, ISpTranscriptVtbl,
    SPDISPLAYATTRIBUTES,
    SPAF_ONE_TRAILING_SPACE,
    SPAF_TWO_TRAILING_SPACES,
    SPAF_CONSUME_LEADING_SPACES
};
pub const SPAF_BUFFER_POSITION: SPDISPLAYATTRIBUTES = 0x10;
pub const SPAF_ALL: SPDISPLAYATTRIBUTES = 0x1f;
pub const SPAF_USER_SPECIFIED: SPDISPLAYATTRIBUTES = 0x80;
pub use um::sapi51::{
    SPPHONEID,
    PSPPHONEID,
    PCSPPHONEID,
    SPPHRASEELEMENT,
    SPPHRASERULE,
    SPPHRASEPROPERTYUNIONTYPE,
    SPPPUT_UNUSED,
    SPPPUT_ARRAY_INDEX,
    SPPHRASEPROPERTY,
    SPPHRASEREPLACEMENT
};
STRUCT!{struct SPSEMANTICERRORINFO {
    ulLineNumber: ULONG,
    pszScriptLine: LPWSTR,
    pszSource: LPWSTR,
    pszDescription: LPWSTR,
    hrResultCode: HRESULT,
}}
ENUM!{enum SPSEMANTICFORMAT {
    SPSMF_SAPI_PROPERTIES = 0,
    SPSMF_SRGS_SEMANTICINTERPRETATION_MS = 1,
    SPSMF_SRGS_SAPIPROPERTIES = 2,
    SPSMF_UPS = 4,
    SPSMF_SRGS_SEMANTICINTERPRETATION_W3C = 8,
}}
pub use um::sapi51::SPPHRASE as SPPHRASE_50;
// TODO: pub const SP_SPPHRASESIZE_500: usize = mem::size_of::<SPPHRASE_50>();
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
    pSML: LPWSTR,
    pSemanticErrorInfo: *mut SPSEMANTICERRORINFO,
}}
pub use um::sapi51::SPSERIALIZEDPHRASE;
STRUCT!{struct SPRULE {
    pszRuleName: LPCWSTR,
    ulRuleId: ULONG,
    dwAttributes: DWORD,
}}
pub use um::sapi51::{
    SPVALUETYPE,
    SPDF_PROPERTY,
    SPDF_REPLACEMENT,
    SPDF_RULE,
    SPDF_DISPLAYTEXT,
    SPDF_LEXICALFORM ,
    SPDF_PRONUNCIATION,
    SPDF_AUDIO,
    SPDF_ALTERNATES,
    SPDF_ALL,
    SPBINARYGRAMMAR,
    SPPHRASERNG,
    SPPR_ALL_ELEMENTS,
    SP_GETWHOLEPHRASE,
    SPRR_ALL_ELEMENTS,
    SPSTATEHANDLE,
    SPRECOEVENTFLAGS,
    SPREF_AutoPause,
    SPREF_Emulated
};
pub const SPREF_SMLTimeout: SPRECOEVENTFLAGS = 1 << 2;
pub const SPREF_ExtendableParse: SPRECOEVENTFLAGS = 1 << 3;
pub const SPREF_ReSent: SPRECOEVENTFLAGS = 1 << 4;
pub const SPREF_Hypothesis: SPRECOEVENTFLAGS = 1 << 5;
pub const SPREF_FalseRecognition: SPRECOEVENTFLAGS = 1 << 6;
pub use um::sapi51::{
    SPPARTOFSPEECH,
    SPPS_NotOverriden,
    SPPS_Unknown,
    SPPS_Noun,
    SPPS_Verb,
    SPPS_Modifier,
    SPPS_Function,
    SPPS_Interjection
};
pub const SPPS_Noncontent: SPPARTOFSPEECH = 0x6000;
pub const SPPS_LMA: SPPARTOFSPEECH = 0x7000;
pub const SPPS_SuppressWord: SPPARTOFSPEECH = 0xf000;
pub use um::sapi51::{
    SPLEXICONTYPE,
    eLEXTYPE_USER,
    eLEXTYPE_APP,
    eLEXTYPE_VENDORLEXICON,
    eLEXTYPE_LETTERTOSOUND,
    eLEXTYPE_MORPHOLOGY,
    eLEXTYPE_RESERVED4,
    eLEXTYPE_USER_SHORTCUT,
    eLEXTYPE_RESERVED6,
    eLEXTYPE_RESERVED7,
    eLEXTYPE_RESERVED8,
    eLEXTYPE_RESERVED9,
    eLEXTYPE_RESERVED10,
    eLEXTYPE_PRIVATE1,
    eLEXTYPE_PRIVATE2,
    eLEXTYPE_PRIVATE3,
    eLEXTYPE_PRIVATE4,
    eLEXTYPE_PRIVATE5,
    eLEXTYPE_PRIVATE6,
    eLEXTYPE_PRIVATE7,
    eLEXTYPE_PRIVATE8,
    eLEXTYPE_PRIVATE9,
    eLEXTYPE_PRIVATE10,
    eLEXTYPE_PRIVATE11,
    eLEXTYPE_PRIVATE12,
    eLEXTYPE_PRIVATE13,
    eLEXTYPE_PRIVATE14,
    eLEXTYPE_PRIVATE15,
    eLEXTYPE_PRIVATE16,
    eLEXTYPE_PRIVATE17,
    eLEXTYPE_PRIVATE18,
    eLEXTYPE_PRIVATE19,
    eLEXTYPE_PRIVATE20,
    SPWORDTYPE,
    eWORDTYPE_ADDED,
    eWORDTYPE_DELETED
};
ENUM!{enum SPPRONUNCIATIONFLAGS {
    ePRONFLAG_USED = 1 << 0,
}}
pub use um::sapi51::{
    SPWORDPRONUNCIATION,
    SPWORDPRONUNCIATIONLIST,
    SPWORD,
    SPWORDLIST,
    ISpLexicon, ISpLexiconVtbl,
    ISpContainerLexicon, ISpContainerLexiconVtbl,
};
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
    LangID: WORD,
    shType: SPSHORTCUTTYPE,
    pszDisplay: LPWSTR,
    pszSpoken: LPWSTR,
}}
STRUCT!{struct SPSHORTCUTPAIRLIST {
    ulSize: ULONG,
    pvBuffer: *mut BYTE,
    pFirstShortcutPair: *mut SPSHORTCUTPAIR,
}}
RIDL!(#[uuid(0x3df681e2, 0xea56, 0x11d9, 0x8b, 0xde, 0xf6, 0x6b, 0xad, 0x1e, 0x3f, 0x3a)]
interface ISpShortcut(ISpShortcutVtbl): IUnknown(IUnknownVtbl) {
    fn AddShortcut(
        pszDisplay: LPCWSTR,
        LangID: WORD,
        pszSpoken: LPCWSTR,
        shType: SPSHORTCUTTYPE
    ) -> HRESULT,
    fn RemoveShortcut(
        pszDisplay: LPCWSTR,
        LangID: WORD,
        pszSpoken: LPCWSTR,
        shType: SPSHORTCUTTYPE
    ) -> HRESULT,
    fn GetShortcuts(
        LangId: WORD,
        pShortcutpairList: *mut SPSHORTCUTPAIRLIST
    ) -> HRESULT,
    fn GetGeneration(pdwGeneration: *mut DWORD) -> HRESULT,
    fn GetWordsFromGenerationChange(
        pdwGeneration: *mut DWORD,
        pWordList: *mut SPWORDLIST
    ) -> HRESULT,
    fn GetWords(
        pdwGeneration: *mut DWORD,
        pdwCookie: *mut DWORD,
        pWordList: *mut SPWORDLIST
    ) -> HRESULT,
    fn GetShortcutsForGeneration(
        pdwGeneration: *mut DWORD,
        pdwCookie: *mut DWORD,
        pShortcutpairList: *mut SPSHORTCUTPAIRLIST
    ) -> HRESULT,
    fn GetGenerationChange(
        pdwGeneration: *mut DWORD,
        pShortcutpairList: *mut SPSHORTCUTPAIRLIST
    ) -> HRESULT
});
pub use um::sapi51::{ISpPhoneConverter, ISpPhoneConverterVtbl};
RIDL!(#[uuid(0x133adcd4, 0x19b4, 0x4020, 0x9f, 0xdc, 0x84, 0x2e, 0x78, 0x25, 0x3b, 0x17)]
interface ISpPhoneticAlphabetConverter(ISpPhoneticAlphabetConverterVtbl): IUnknown(IUnknownVtbl) {
    fn GetLangId(pLangID: *mut WORD) -> HRESULT,
    fn SetLangId(LangID: WORD) -> HRESULT,
    fn SAPI2UPS(
        pszSAPIId: *const SPPHONEID,
        pszUPSId: *mut SPPHONEID,
        cMaxLength: DWORD
    ) -> HRESULT,
    fn UPS2SAPI(
        pszUPSId: *const SPPHONEID,
        pszSAPIId: *mut SPPHONEID,
        cMaxLength: DWORD
    ) -> HRESULT,
    fn GetMaxConvertLength(
        cSrcLength: DWORD,
        bSAPI2UPS: BOOL,
        pcMaxDestLength: *mut DWORD
    ) -> HRESULT
});
RIDL!(#[uuid(0xb2745efd, 0x42ce, 0x48ca, 0x81, 0xf1, 0xa9, 0x6e, 0x02, 0x53, 0x8a, 0x90)]
interface ISpPhoneticAlphabetSelection(ISpPhoneticAlphabetSelectionVtbl): IUnknown(IUnknownVtbl) {
    fn IsAlphabetUPS(pfIsUPS: *mut BOOL) -> HRESULT,
    fn SetAlphabetToUPS(fForceUPS: BOOL) -> HRESULT
});
pub use um::sapi51::{
    SPVPITCH,
    SPVACTIONS,
    SPVA_Speak,
    SPVA_Silence,
    SPVA_Pronounce,
    SPVA_Bookmark,
    SPVA_SpellOut,
    SPVA_Section,
    SPVA_ParseUnknownTag,
    SPVCONTEXT,
    SPVSTATE,
    SPRUNSTATE,
    SPRS_DONE,
    SPRS_IS_SPEAKING,
    SPVLIMITS,
    SPMIN_VOLUME,
    SPMAX_VOLUME,
    SPMIN_RATE,
    SPMAX_RATE,
    SPVPRIORITY,
    SPVPRI_NORMAL,
    SPVPRI_ALERT,
    SPVPRI_OVER,
    SPVOICESTATUS,
    SPEAKFLAGS,
    SPF_DEFAULT,
    SPF_ASYNC,
    SPF_PURGEBEFORESPEAK,
    SPF_IS_FILENAME,
    SPF_IS_XML,
    SPF_IS_NOT_XML,
    SPF_PERSIST_XML,
    SPF_NLP_SPEAK_PUNC,
};
pub const SPF_PARSE_SAPI: SPEAKFLAGS = 1 << 7;
pub const SPF_PARSE_SSML: SPEAKFLAGS = 1 << 8;
pub const SPF_PARSE_AUTODETECT: SPEAKFLAGS = 0;
pub use um::sapi51::SPF_NLP_MASK;
pub const SPF_PARSE_MASK: SPEAKFLAGS = SPF_PARSE_SAPI | SPF_PARSE_SSML;
pub const SPF_VOICE_MASK: SPEAKFLAGS = ::um::sapi51::SPF_VOICE_MASK | SPF_PARSE_MASK;
pub const SPF_UNUSED_FLAGS: SPEAKFLAGS = !SPF_VOICE_MASK;
pub use um::sapi51::{
    ISpVoice, ISpVoiceVtbl,
    ISpPhrase, ISpPhraseVtbl,
    ISpPhraseAlt, ISpPhraseAltVtbl,
};
ENUM!{enum SPXMLRESULTOPTIONS {
    SPXRO_SML = 0,
    SPXRO_Alternates_SML = 1,
}}
RIDL!(#[uuid(0xf264da52, 0xe457, 0x4696, 0xb8, 0x56, 0xa7, 0x37, 0xb7, 0x17, 0xaf, 0x79)]
interface ISpPhrase2(ISpPhrase2Vtbl): ISpPhrase(ISpPhraseVtbl) {
    fn GetXMLResult(
        ppszCoMemXMLResult: *mut LPWSTR,
        Options: SPXMLRESULTOPTIONS
    ) -> HRESULT,
    fn GetXMLErrorInfo(pSemanticErrorInfo: *mut SPSEMANTICERRORINFO) -> HRESULT,
    fn GetAudio(
        ulStartElement: ULONG,
        cElements: ULONG,
        ppStream: *mut *mut ISpStreamFormat
    ) -> HRESULT
});
pub use um::sapi51::{
    SPRECORESULTTIMES,
    SPSERIALIZEDRESULT,
    ISpRecoResult, ISpRecoResultVtbl,
};
ENUM!{enum SPCOMMITFLAGS {
    SPCF_NONE = 0,
    SPCF_ADD_TO_USER_LEXICON = 1 << 0,
    SPCF_DEFINITE_CORRECTION = 1 << 1,
}}
RIDL!(#[uuid(0x27cac6c4, 0x88f2, 0x41f2, 0x88, 0x17, 0x0c, 0x95, 0xe5, 0x9f, 0x1e, 0x6e)]
interface ISpRecoResult2(ISpRecoResult2Vtbl): ISpRecoResult(ISpRecoResultVtbl) {
    fn CommitAlternate(
        pPhraseAlt: *mut ISpPhraseAlt,
        ppNewResult: *mut *mut ISpRecoResult
    ) -> HRESULT,
    fn CommitText(
        ulStartElement: ULONG,
        cElements: ULONG,
        pszCorrectedData: LPCWSTR,
        eCommitFlags: DWORD
    ) -> HRESULT,
    fn SetTextFeedback(
        pszFeedback: LPCWSTR,
        fSuccessful: BOOL
    ) -> HRESULT
});
RIDL!(#[uuid(0xae39362b, 0x45a8, 0x4074, 0x9b, 0x9e, 0xcc, 0xf4, 0x9a, 0xa2, 0xd0, 0xb6)]
interface ISpXMLRecoResult(ISpXMLRecoResultVtbl): ISpRecoResult(ISpRecoResultVtbl) {
    fn GetXMLResult(
        ppszCoMemXMLResult: *mut LPWSTR,
        Options: SPXMLRESULTOPTIONS
    ) -> HRESULT,
    fn GetXMLErrorInfo(pSemanticErrorInfo: *mut SPSEMANTICERRORINFO) -> HRESULT
});
pub use um::sapi51::{
    SPTEXTSELECTIONINFO,
    SPWORDPRONOUNCEABLE,
    SPWP_UNKNOWN_WORD_UNPRONOUNCEABLE,
    SPWP_UNKNOWN_WORD_PRONOUNCEABLE,
    SPWP_KNOWN_WORD_PRONOUNCEABLE,
    SPGRAMMARSTATE,
    SPGS_DISABLED,
    SPGS_ENABLED,
    SPGS_EXCLUSIVE,
    SPCONTEXTSTATE,
    SPCS_DISABLED,
    SPCS_ENABLED,
    SPRULESTATE,
    SPRS_INACTIVE,
    SPRS_ACTIVE,
    SPRS_ACTIVE_WITH_AUTO_PAUSE,
};
pub const SPRS_ACTIVE_USER_DELIMITED: SPRULESTATE = 4;
pub use um::sapi51::{
    SP_STREAMPOS_ASAP,
    SP_STREAMPOS_REALTIME,
    SPRULETRANS_TEXTBUFFER,
    SPRULETRANS_WILDCARD,
    SPRULETRANS_DICTATION,
    SPGRAMMARWORDTYPE,
    SPWT_DISPLAY,
    SPWT_LEXICAL,
    SPWT_PRONUNCIATION,
};
pub const SPWT_LEXICAL_NO_SPECIAL_CHARS: SPGRAMMARWORDTYPE = SPWT_PRONUNCIATION + 1;
pub use um::sapi51::{
    SPPROPERTYINFO,
    SPCFGRULEATTRIBUTES,
    SPRAF_TopLevel,
    SPRAF_Active,
    SPRAF_Export,
    SPRAF_Import,
    SPRAF_Interpreter,
    SPRAF_Dynamic,
};
pub const SPRAF_Root: SPCFGRULEATTRIBUTES = 1 << 6;
pub use um::sapi51::SPRAF_AutoPause;
pub const SPRAF_UserDelimited: SPCFGRULEATTRIBUTES = 1 << 17;
pub use um::sapi51::{
    ISpGrammarBuilder, ISpGrammarBuilderVtbl,
    SPLOADOPTIONS,
    SPLO_STATIC,
    SPLO_DYNAMIC,
    ISpRecoGrammar, ISpRecoGrammarVtbl,
};
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
RIDL!(#[uuid(0x8ab10026, 0x20cc, 0x4b20, 0x8c, 0x22, 0xa4, 0x9c, 0x9b, 0xa7, 0x8f, 0x60)]
interface ISpGrammarBuilder2(ISpGrammarBuilder2Vtbl): IUnknown(IUnknownVtbl) {
    fn AddTextSubset(
        hFromState: SPSTATEHANDLE,
        hToState: SPSTATEHANDLE,
        psz: LPCWSTR,
        eMatchMode: SPMATCHINGMODE
    ) -> HRESULT,
    fn SetPhoneticAlphabet(phoneticALphabet: PHONETICALPHABET) -> HRESULT
});
pub const SPRP_NORMAL: i32 = 0; // TODO: Unknown purpose and type
RIDL!(#[uuid(0x4b37bc9e, 0x9ed6, 0x44a3, 0x93, 0xd3, 0x18, 0xf0, 0x22, 0xb7, 0x9e, 0xc3)]
interface ISpRecoGrammar2(ISpRecoGrammar2Vtbl): IUnknown(IUnknownVtbl) {
    fn GetRules(
        ppCoMemRules: *mut *mut SPRULE,
        puNumRules: *mut UINT
    ) -> HRESULT,
    fn LoadCmdFromFile2(
        pszFileName: LPCWSTR,
        Options: SPLOADOPTIONS,
        pszSharingUri: LPCWSTR,
        pszBaseUri: LPCWSTR
    ) -> HRESULT,
    fn LoadCmdFromMemory2(
        pGrammar: *const SPBINARYGRAMMAR,
        Options: SPLOADOPTIONS,
        pszSharingUri: LPCWSTR,
        pszBaseUri: LPCWSTR
    ) -> HRESULT,
    fn SetRulePriority(
        pszRuleName: LPCWSTR,
        ulRuleId: ULONG,
        nRulePriority: c_int
    ) -> HRESULT,
    fn SetRuleWeight(
        pszRuleName: LPCWSTR,
        ulRuleId: ULONG,
        flWeight: c_float
    ) -> HRESULT,
    fn SetDictationWeight(flWeight: c_float) -> HRESULT,
    fn SetGrammarLoader(pLoader: *mut ISpeechResourceLoader) -> HRESULT,
    fn SetSMLSecurityManager(pSMLSecurityManager: *mut IInternetSecurityManager) -> HRESULT
});
RIDL!(#[uuid(0xb9ac5783, 0xfcd0, 0x4b21, 0xb1, 0x19, 0xb4, 0xf8, 0xda, 0x8f, 0xd2, 0xc3)]
interface ISpeechResourceLoader(ISpeechResourceLoaderVtbl): IDispatch(IDispatchVtbl) {
    fn LoadResource(
        bstrResourceUri: BSTR,
        fAlwaysReload: VARIANT_BOOL,
        pStream: *mut *mut IUnknown,
        pbstrMIMEType: *mut BSTR,
        pfModified: *mut VARIANT_BOOL,
        pbstrRedirectUrl: *mut BSTR
    ) -> HRESULT,
    fn GetLocalCopy(
        bstrResourceUri: BSTR,
        pbstrLocalPath: *mut BSTR,
        pbstrMIMEType: *mut BSTR,
        pbstrRedirectUrl: *mut BSTR
    ) -> HRESULT,
    fn ReleaseLocalCopy(pbstrLocalPath: BSTR) -> HRESULT
});
pub use um::sapi51::{
    SPRECOCONTEXTSTATUS,
    SPBOOKMARKOPTIONS,
    SPBO_NONE,
    SPBO_PAUSE,
};
pub const SPBO_AHEAD: SPBOOKMARKOPTIONS = 1 << 1;
pub const SPBO_TIME_UNITS: SPBOOKMARKOPTIONS = 1 << 2;
pub use um::sapi51::{
    SPAUDIOOPTIONS,
    SPAO_NONE,
    SPAO_RETAIN_AUDIO,
    ISpRecoContext, ISpRecoContextVtbl,
};
ENUM!{enum SPGRAMMAROPTIONS {
    SPGO_SAPI = 0x1,
    SPGO_SRGS = 0x2,
    SPGO_UPS = 0x4,
    SPGO_SRGS_MS_SCRIPT = 0x8,
    SPGO_SRGS_W3C_SCRIPT = 0x100,
    SPGO_SRGS_STG_SCRIPT = 0x200,
    SPGO_SRGS_SCRIPT = SPGO_SRGS | SPGO_SRGS_MS_SCRIPT
        | SPGO_SRGS_W3C_SCRIPT | SPGO_SRGS_STG_SCRIPT,
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
RIDL!(#[uuid(0xbead311c, 0x52ff, 0x437f, 0x94, 0x64, 0x6b, 0x21, 0x05, 0x4c, 0xa7, 0x3d)]
interface ISpRecoContext2(ISpRecoContext2Vtbl): IUnknown(IUnknownVtbl) {
    fn SetGrammarOptions(eGrammarOptions: DWORD) -> HRESULT,
    fn GetGrammarOptions(peGrammarOptions: *mut DWORD) -> HRESULT,
    fn SetAdaptationData2(
        pAdaptationData: LPCWSTR,
        cch: ULONG,
        pTopicName: LPCWSTR,
        eAdaptationSettings: DWORD,
        eRelevance: SPADAPTATIONRELEVANCE
    ) -> HRESULT
});
pub use um::sapi51::{
    ISpProperties, ISpPropertiesVtbl,
    SP_MAX_LANGIDS,
    SPRECOGNIZERSTATUS,
    SPWAVEFORMATTYPE,
    SPWF_INPUT,
    SPWF_SRENGINE,
    SPSTREAMFORMATTYPE,
    SPRECOSTATE,
    SPRST_INACTIVE,
    SPRST_ACTIVE,
    SPRST_ACTIVE_ALWAYS,
    SPRST_INACTIVE_WITH_PURGE,
    SPRST_NUM_STATES,
    ISpRecognizer, ISpRecognizerVtbl,
};
RIDL!(#[uuid(0x21b501a0, 0x0ec7, 0x46c9, 0x92, 0xc3, 0xa2, 0xbc, 0x78, 0x4c, 0x54, 0xb9)]
interface ISpSerializeState(ISpSerializeStateVtbl): IUnknown(IUnknownVtbl) {
    fn GetSerializedState(
        ppbData: *mut *mut BYTE,
        pulSize: *mut ULONG,
        dwReserved: DWORD
    ) -> HRESULT,
    fn SetSerializedState(
        pbData: *mut BYTE,
        ulSize: ULONG,
        dwReserved: DWORD
    ) -> HRESULT
});
RIDL!(#[uuid(0x8fc6d974, 0xc81e, 0x4098, 0x93, 0xc5, 0x01, 0x47, 0xf6, 0x1e, 0xd4, 0xd3)]
interface ISpRecognizer2(ISpRecognizer2Vtbl): IUnknown(IUnknownVtbl) {
    fn EmulateRecognitionEx(
        pPhrase: *mut ISpPhrase,
        dwCompareFlags: DWORD
    ) -> HRESULT,
    fn SetTrainingState(
        fDoingTraining: BOOL,
        fAdaptFromTrainingData: BOOL
    ) -> HRESULT,
    fn ResetAcousticModelAdaptation() -> HRESULT
});
STRUCT!{struct SPNORMALIZATIONLIST {
    ulSize: ULONG,
    ppszzNormalizedList: *mut *mut WCHAR,
}}
RIDL!(#[uuid(0xc360ce4b, 0x76d1, 0x4214, 0xad, 0x68, 0x52, 0x65, 0x7d, 0x50, 0x83, 0xda)]
interface ISpEnginePronunciation(ISpEnginePronunciationVtbl): IUnknown(IUnknownVtbl) {
    fn Normalize(
        pszWord: LPCWSTR,
        pszLeftContext: LPCWSTR,
        pszRightContext: LPCWSTR,
        LangID: WORD,
        pNormalizationList: *mut SPNORMALIZATIONLIST
    ) -> HRESULT,
    fn GetPronunciations(
        pszWord: LPCWSTR,
        pszLeftContext: LPCWSTR,
        pszRightContext: LPCWSTR,
        LangID: WORD,
        pEnginePronunciationList: *mut SPWORDPRONUNCIATIONLIST
    ) -> HRESULT
});
STRUCT!{struct SPDISPLAYTOKEN {
    pszLexical: *const WCHAR,
    pszDisplay: *const WCHAR,
    bDisplayAttributes: BYTE,
}}
STRUCT!{struct SPDISPLAYPHRASE {
    ulNumTokens: ULONG,
    pTokens: *mut SPDISPLAYTOKEN,
}}
RIDL!(#[uuid(0xc8d7c7e2, 0x0dde, 0x44b7, 0xaf, 0xe3, 0xb0, 0xc9, 0x91, 0xfb, 0xeb, 0x5e)]
interface ISpDisplayAlternates(ISpDisplayAlternatesVtbl): IUnknown(IUnknownVtbl) {
    fn GetDisplayAlternates(
        pPhrase: *const SPDISPLAYPHRASE,
        cRequestCount: ULONG,
        ppCoMemPhrases: *mut *mut SPDISPLAYPHRASE,
        pcPhrasesReturned: *mut ULONG
    ) -> HRESULT,
    fn SetFullStopTrailSpace(ulTrailSpace: ULONG) -> HRESULT
});
pub use um::sapi51::{
    SpeechLanguageId,
    DISPID_SpeechDataKey,
    DISPID_SDKSetBinaryValue,
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
    DISPID_SpeechObjectToken,
    DISPID_SOTId,
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
    SpeechDataKeyLocation,
    SDKLDefaultLocation,
    SDKLCurrentUser,
    SDKLLocalMachine,
    SDKLCurrentConfig,
    SpeechTokenContext,
    STCInprocServer,
    STCInprocHandler ,
    STCLocalServer,
    STCRemoteServer,
    STCAll,
    SpeechTokenShellFolder,
    STSF_AppData,
    STSF_LocalAppData,
    STSF_CommonAppData,
    STSF_FlagCreate,
    DISPID_SpeechObjectTokens,
    DISPID_SOTsCount,
    DISPID_SOTsItem,
    DISPID_SOTs_NewEnum,
    DISPID_SpeechObjectTokenCategory,
    DISPID_SOTCId,
    DISPID_SOTCDefault,
    DISPID_SOTCSetId,
    DISPID_SOTCGetDataKey,
    DISPID_SOTCEnumerateTokens,
    SpeechAudioFormatType,
    SAFTDefault,
    SAFTNoAssignedFormat,
    SAFTText,
    SAFTNonStandardFormat,
    SAFTExtendedAudioFormat,
    SAFT8kHz8BitMono,
    SAFT8kHz8BitStereo,
    SAFT8kHz16BitMono,
    SAFT8kHz16BitStereo,
    SAFT11kHz8BitMono,
    SAFT11kHz8BitStereo,
    SAFT11kHz16BitMono,
    SAFT11kHz16BitStereo,
    SAFT12kHz8BitMono,
    SAFT12kHz8BitStereo,
    SAFT12kHz16BitMono,
    SAFT12kHz16BitStereo,
    SAFT16kHz8BitMono,
    SAFT16kHz8BitStereo,
    SAFT16kHz16BitMono,
    SAFT16kHz16BitStereo,
    SAFT22kHz8BitMono,
    SAFT22kHz8BitStereo,
    SAFT22kHz16BitMono,
    SAFT22kHz16BitStereo,
    SAFT24kHz8BitMono,
    SAFT24kHz8BitStereo,
    SAFT24kHz16BitMono,
    SAFT24kHz16BitStereo,
    SAFT32kHz8BitMono,
    SAFT32kHz8BitStereo,
    SAFT32kHz16BitMono,
    SAFT32kHz16BitStereo,
    SAFT44kHz8BitMono,
    SAFT44kHz8BitStereo,
    SAFT44kHz16BitMono,
    SAFT44kHz16BitStereo,
    SAFT48kHz8BitMono,
    SAFT48kHz8BitStereo,
    SAFT48kHz16BitMono,
    SAFT48kHz16BitStereo,
    SAFTTrueSpeech_8kHz1BitMono,
    SAFTCCITT_ALaw_8kHzMono,
    SAFTCCITT_ALaw_8kHzStereo,
    SAFTCCITT_ALaw_11kHzMono,
    SAFTCCITT_ALaw_11kHzStereo,
    SAFTCCITT_ALaw_22kHzMono,
    SAFTCCITT_ALaw_22kHzStereo,
    SAFTCCITT_ALaw_44kHzMono,
    SAFTCCITT_ALaw_44kHzStereo,
    SAFTCCITT_uLaw_8kHzMono,
    SAFTCCITT_uLaw_8kHzStereo,
    SAFTCCITT_uLaw_11kHzMono,
    SAFTCCITT_uLaw_11kHzStereo,
    SAFTCCITT_uLaw_22kHzMono,
    SAFTCCITT_uLaw_22kHzStereo,
    SAFTCCITT_uLaw_44kHzMono,
    SAFTCCITT_uLaw_44kHzStereo,
    SAFTADPCM_8kHzMono,
    SAFTADPCM_8kHzStereo,
    SAFTADPCM_11kHzMono,
    SAFTADPCM_11kHzStereo,
    SAFTADPCM_22kHzMono,
    SAFTADPCM_22kHzStereo,
    SAFTADPCM_44kHzMono,
    SAFTADPCM_44kHzStereo,
    SAFTGSM610_8kHzMono,
    SAFTGSM610_11kHzMono,
    SAFTGSM610_22kHzMono,
    SAFTGSM610_44kHzMono,
    DISPID_SpeechAudioFormat,
    DISPID_SAFType,
    DISPID_SAFGuid,
    DISPID_SAFGetWaveFormatEx,
    DISPID_SAFSetWaveFormatEx,
    DISPID_SpeechBaseStream,
    DISPID_SBSFormat,
    DISPID_SBSRead,
    DISPID_SBSWrite,
    DISPID_SBSSeek,
    SpeechStreamSeekPositionType,
    SSSPTRelativeToStart,
    SSSPTRelativeToCurrentPosition,
    SSSPTRelativeToEnd,
    DISPID_SpeechAudio,
    DISPID_SAStatus,
    DISPID_SABufferInfo,
    DISPID_SADefaultFormat,
    DISPID_SAVolume,
    DISPID_SABufferNotifySize,
    DISPID_SAEventHandle,
    DISPID_SASetState,
    SpeechAudioState,
    SASClosed,
    SASStop,
    SASPause,
    SASRun,
    DISPID_SpeechMMSysAudio,
    DISPID_SMSADeviceId,
    DISPID_SMSALineId,
    DISPID_SMSAMMHandle,
    DISPID_SpeechFileStream,
    DISPID_SFSOpen,
    DISPID_SFSClose,
    SpeechStreamFileMode,
    SSFMOpenForRead,
    SSFMOpenReadWrite,
    SSFMCreate,
    SSFMCreateForWrite,
    DISPID_SpeechCustomStream,
    DISPID_SCSBaseStream,
    DISPID_SpeechMemoryStream,
    DISPID_SMSSetData,
    DISPID_SMSGetData,
    DISPID_SpeechAudioStatus,
    DISPID_SASFreeBufferSpace,
    DISPID_SASNonBlockingIO,
    DISPID_SASState,
    DISPID_SASCurrentSeekPosition,
    DISPID_SASCurrentDevicePosition,
    DISPID_SpeechAudioBufferInfo,
    DISPID_SABIMinNotification,
    DISPID_SABIBufferSize,
    DISPID_SABIEventBias,
    DISPID_SpeechWaveFormatEx,
    DISPID_SWFEFormatTag,
    DISPID_SWFEChannels,
    DISPID_SWFESamplesPerSec,
    DISPID_SWFEAvgBytesPerSec,
    DISPID_SWFEBlockAlign,
    DISPID_SWFEBitsPerSample,
    DISPID_SWFEExtraData,
    DISPID_SpeechVoice,
    DISPID_SVStatus,
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
    SpeechVoicePriority,
    SVPNormal,
    SVPAlert,
    SVPOver,
    SpeechVoiceSpeakFlags,
    SVSFDefault,
    SVSFlagsAsync,
    SVSFPurgeBeforeSpeak,
    SVSFIsFilename,
    SVSFIsXML,
    SVSFIsNotXML,
    SVSFPersistXML,
    SVSFNLPSpeakPunc,
};
pub const SVSFParseSapi: SpeechVoiceSpeakFlags = SPF_PARSE_SAPI;
pub const SVSFParseSsml: SpeechVoiceSpeakFlags = SPF_PARSE_SSML;
pub const SVSFParseAutodetect: SpeechVoiceSpeakFlags = SPF_PARSE_AUTODETECT;
pub use um::sapi51::SVSFNLPMask;
pub const SVSFParseMask: SpeechVoiceSpeakFlags = SPF_PARSE_MASK as u32;
pub use um::sapi51::{
    SVSFVoiceMask,
    SVSFUnusedFlags,
    SpeechVoiceEvents,
    SVEStartInputStream,
    SVEEndInputStream,
    SVEVoiceChange,
    SVEBookmark,
    SVEWordBoundary,
    SVEPhoneme,
    SVESentenceBoundary,
    SVEViseme,
    SVEAudioLevel,
    SVEPrivate,
    SVEAllEvents,
    DISPID_SpeechVoiceStatus,
    DISPID_SVSCurrentStreamNumber,
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
    SpeechRunState,
    SRSEDone,
    SRSEIsSpeaking,
    SpeechVisemeType,
    SVP_0,
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
    SpeechVisemeFeature,
    SVF_None,
    SVF_Stressed,
    SVF_Emphasis,
    DISPID_SpeechVoiceEvent,
    DISPID_SVEStreamStart,
    DISPID_SVEStreamEnd,
    DISPID_SVEVoiceChange,
    DISPID_SVEBookmark,
    DISPID_SVEWord,
    DISPID_SVEPhoneme,
    DISPID_SVESentenceBoundary,
    DISPID_SVEViseme,
    DISPID_SVEAudioLevel,
    DISPID_SVEEnginePrivate,
    DISPID_SpeechRecognizer,
    DISPID_SRRecognizer,
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
    SpeechRecognizerState,
    SRSInactive,
    SRSActive,
    SRSActiveAlways,
    SRSInactiveWithPurge,
    SpeechDisplayAttributes,
    SDA_No_Trailing_Space,
    SDA_One_Trailing_Space,
    SDA_Two_Trailing_Spaces,
    SDA_Consume_Leading_Spaces,
    SpeechFormatType,
    SFTInput,
    SFTSREngine,
};
ENUM!{enum SpeechEmulationCompareFlags {
    SECFIgnoreCase = 0x1,
    SECFIgnoreKanaType = 0x10000,
    SECFIgnoreWidth = 0x20000,
    SECFNoSpecialChars = 0x20000000,
    SECFEmulateResult = 0x40000000,
    SECFDefault = SECFIgnoreCase | SECFIgnoreKanaType | SECFIgnoreWidth,
}}
pub use um::sapi51::{
    DISPID_SpeechRecognizerStatus,
    DISPID_SRSAudioStatus,
    DISPID_SRSCurrentStreamPosition,
    DISPID_SRSCurrentStreamNumber,
    DISPID_SRSNumberOfActiveRules,
    DISPID_SRSClsidEngine,
    DISPID_SRSSupportedLanguages,
    DISPID_SpeechRecoContext,
    DISPID_SRCRecognizer,
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
    SpeechRetainedAudioOptions,
    SRAONone,
    SRAORetainAudio,
    SpeechBookmarkOptions,
    SBONone,
    SBOPause,
    SpeechInterference,
    SINone,
    SINoise,
    SINoSignal,
    SITooLoud,
    SITooQuiet,
    SITooFast,
    SITooSlow,
    SpeechRecoEvents,
    SREStreamEnd,
    SRESoundStart,
    SRESoundEnd,
    SREPhraseStart,
    SRERecognition,
    SREHypothesis,
    SREBookmark,
    SREPropertyNumChange,
    SREPropertyStringChange,
    SREFalseRecognition,
    SREInterference,
    SRERequestUI,
    SREStateChange,
    SREAdaptation,
    SREStreamStart,
    SRERecoOtherContext,
    SREAudioLevel,
    SREPrivate,
    SREAllEvents,
    SpeechRecoContextState,
    SRCS_Disabled,
    SRCS_Enabled,
    DISPIDSPRG,
    DISPID_SRGId,
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
    SpeechLoadOption,
    SLOStatic,
    SLODynamic,
    SpeechWordPronounceable,
    SWPUnknownWordUnpronounceable,
    SWPUnknownWordPronounceable,
    SWPKnownWordPronounceable,
    SpeechGrammarState,
    SGSEnabled,
    SGSDisabled,
    SGSExclusive,
    SpeechRuleState,
    SGDSInactive,
    SGDSActive,
    SGDSActiveWithAutoPause,
};
pub const SGDSActiveUserDelimited: SpeechRuleState = SPRS_ACTIVE_USER_DELIMITED;
pub use um::sapi51::{
    SpeechRuleAttributes,
    SRATopLevel,
    SRADefaultToActive,
    SRAExport,
    SRAImport,
    SRAInterpreter,
    SRADynamic,
};
pub const SRARoot: SpeechRuleAttributes = SPRAF_Root;
pub use um::sapi51::{
    SpeechGrammarWordType,
    SGDisplay,
    SGLexical,
    SGPronounciation,
};
pub const SGLexicalNoSpecialChars: SpeechGrammarWordType = SPWT_LEXICAL_NO_SPECIAL_CHARS;
pub use um::sapi51::{
    DISPID_SpeechRecoContextEvents,
    DISPID_SRCEStartStream,
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
    SpeechRecognitionType,
    SRTStandard,
    SRTAutopause,
    SRTEmulated,
};
pub const SRTSMLTimeout: SpeechRecognitionType = SPREF_SMLTimeout;
pub const SRTExtendableParse: SpeechRecognitionType = SPREF_ExtendableParse;
pub const SRTReSent: SpeechRecognitionType = SPREF_ReSent;
pub use um::sapi51::{
    DISPID_SpeechGrammarRule,
    DISPID_SGRAttributes,
    DISPID_SGRInitialState,
    DISPID_SGRName,
    DISPID_SGRId,
    DISPID_SGRClear,
    DISPID_SGRAddResource,
    DISPID_SGRAddState,
    DISPID_SpeechGrammarRules,
    DISPID_SGRsCount,
    DISPID_SGRsDynamic,
    DISPID_SGRsAdd,
    DISPID_SGRsCommit,
    DISPID_SGRsCommitAndSave,
    DISPID_SGRsFindRule,
    DISPID_SGRsItem,
    DISPID_SGRs_NewEnum,
    DISPID_SpeechGrammarRuleState,
    DISPID_SGRSRule,
    DISPID_SGRSTransitions,
    DISPID_SGRSAddWordTransition,
    DISPID_SGRSAddRuleTransition,
    DISPID_SGRSAddSpecialTransition,
    SpeechSpecialTransitionType,
    SSTTWildcard,
    SSTTDictation,
    SSTTTextBuffer,
    DISPID_SpeechGrammarRuleStateTransitions,
    DISPID_SGRSTsCount,
    DISPID_SGRSTsItem,
    DISPID_SGRSTs_NewEnum,
    DISPID_SpeechGrammarRuleStateTransition,
    DISPID_SGRSTType,
    DISPID_SGRSTText,
    DISPID_SGRSTRule,
    DISPID_SGRSTWeight,
    DISPID_SGRSTPropertyName,
    DISPID_SGRSTPropertyId,
    DISPID_SGRSTPropertyValue,
    DISPID_SGRSTNextState,
    SpeechGrammarRuleStateTransitionType,
    SGRSTTEpsilon,
    SGRSTTWord,
    SGRSTTRule,
    SGRSTTDictation,
    SGRSTTWildcard,
    SGRSTTTextBuffer,
    DISPIDSPTSI,
    DISPIDSPTSI_ActiveOffset,
    DISPIDSPTSI_ActiveLength,
    DISPIDSPTSI_SelectionOffset,
    DISPIDSPTSI_SelectionLength,
    DISPID_SpeechRecoResult,
    DISPID_SRRRecoContext,
    DISPID_SRRTimes,
    DISPID_SRRAudioFormat,
    DISPID_SRRPhraseInfo,
    DISPID_SRRAlternates,
    DISPID_SRRAudio,
    DISPID_SRRSpeakAudio,
    DISPID_SRRSaveToMemory,
    DISPID_SRRDiscardResultInfo,
    SpeechDiscardType,
    SDTProperty,
    SDTReplacement,
    SDTRule,
    SDTDisplayText,
    SDTLexicalForm,
    SDTPronunciation,
    SDTAudio,
    SDTAlternates,
    SDTAll,
};
ENUM!{enum DISPID_SpeechXMLRecoResult {
    DISPID_SRRGetXMLResult,
    DISPID_SRRGetXMLErrorInfo,
}}
ENUM!{enum DISPID_SpeechRecoResult2 {
    DISPID_SRRSetTextFeedback,
}}
pub use um::sapi51::{
    DISPID_SpeechPhraseBuilder,
    DISPID_SPPBRestorePhraseFromMemory,
    DISPID_SpeechRecoResultTimes,
    DISPID_SRRTStreamTime,
    DISPID_SRRTLength,
    DISPID_SRRTTickCount,
    DISPID_SRRTOffsetFromStart,
    DISPID_SpeechPhraseAlternate,
    DISPID_SPARecoResult,
    DISPID_SPAStartElementInResult,
    DISPID_SPANumberOfElementsInResult,
    DISPID_SPAPhraseInfo,
    DISPID_SPACommit,
    DISPID_SpeechPhraseAlternates,
    DISPID_SPAsCount,
    DISPID_SPAsItem,
    DISPID_SPAs_NewEnum,
    DISPID_SpeechPhraseInfo,
    DISPID_SPILanguageId,
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
    DISPID_SpeechPhraseElement,
    DISPID_SPEAudioTimeOffset,
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
    SpeechEngineConfidence,
    SECLowConfidence,
    SECNormalConfidence,
    SECHighConfidence,
    DISPID_SpeechPhraseElements,
    DISPID_SPEsCount,
    DISPID_SPEsItem,
    DISPID_SPEs_NewEnum,
    DISPID_SpeechPhraseReplacement,
    DISPID_SPRDisplayAttributes,
    DISPID_SPRText,
    DISPID_SPRFirstElement,
    DISPID_SPRNumberOfElements,
    DISPID_SpeechPhraseReplacements,
    DISPID_SPRsCount,
    DISPID_SPRsItem,
    DISPID_SPRs_NewEnum,
    DISPID_SpeechPhraseProperty,
    DISPID_SPPName,
    DISPID_SPPId,
    DISPID_SPPValue,
    DISPID_SPPFirstElement,
    DISPID_SPPNumberOfElements,
    DISPID_SPPEngineConfidence,
    DISPID_SPPConfidence,
    DISPID_SPPParent,
    DISPID_SPPChildren,
    DISPID_SpeechPhraseProperties,
    DISPID_SPPsCount,
    DISPID_SPPsItem,
    DISPID_SPPs_NewEnum,
    DISPID_SpeechPhraseRule,
    DISPID_SPRuleName,
    DISPID_SPRuleId,
    DISPID_SPRuleFirstElement,
    DISPID_SPRuleNumberOfElements,
    DISPID_SPRuleParent,
    DISPID_SPRuleChildren,
    DISPID_SPRuleConfidence,
    DISPID_SPRuleEngineConfidence,
    DISPID_SpeechPhraseRules,
    DISPID_SPRulesCount,
    DISPID_SPRulesItem,
    DISPID_SPRules_NewEnum,
    DISPID_SpeechLexicon,
    DISPID_SLGenerationId,
    DISPID_SLGetWords,
    DISPID_SLAddPronunciation,
    DISPID_SLAddPronunciationByPhoneIds,
    DISPID_SLRemovePronunciation,
    DISPID_SLRemovePronunciationByPhoneIds,
    DISPID_SLGetPronunciations,
    DISPID_SLGetGenerationChange,
    SpeechLexiconType,
    SLTUser,
    SLTApp,
    SpeechPartOfSpeech,
    SPSNotOverriden,
    SPSUnknown,
    SPSNoun,
    SPSVerb,
    SPSModifier,
    SPSFunction,
    SPSInterjection,
};
pub const SPSLMA: SpeechPartOfSpeech = SPPS_LMA;
pub const SPSSuppressWord: SpeechPartOfSpeech = SPPS_SuppressWord;
pub use um::sapi51::{
    DISPID_SpeechLexiconWords,
    DISPID_SLWsCount,
    DISPID_SLWsItem,
    DISPID_SLWs_NewEnum,
    SpeechWordType,
    SWTAdded,
    SWTDeleted,
    DISPID_SpeechLexiconWord,
    DISPID_SLWLangId,
    DISPID_SLWType,
    DISPID_SLWWord,
    DISPID_SLWPronunciations,
    DISPID_SpeechLexiconProns,
    DISPID_SLPsCount,
    DISPID_SLPsItem,
    DISPID_SLPs_NewEnum,
    DISPID_SpeechLexiconPronunciation,
    DISPID_SLPType,
    DISPID_SLPLangId,
    DISPID_SLPPartOfSpeech,
    DISPID_SLPPhoneIds,
    DISPID_SLPSymbolic,
    DISPID_SpeechPhoneConverter,
    DISPID_SPCLangId,
    DISPID_SPCPhoneToId,
    DISPID_SPCIdToPhone,
    LIBID_SpeechLib,
    ISpeechDataKey, ISpeechDataKeyVtbl,
    ISpeechObjectToken, ISpeechObjectTokenVtbl,
    ISpeechObjectTokens, ISpeechObjectTokensVtbl,
    ISpeechObjectTokenCategory, ISpeechObjectTokenCategoryVtbl,
    ISpeechAudioBufferInfo, ISpeechAudioBufferInfoVtbl,
    ISpeechAudioStatus, ISpeechAudioStatusVtbl,
    ISpeechAudioFormat, ISpeechAudioFormatVtbl,
    ISpeechWaveFormatEx, ISpeechWaveFormatExVtbl,
    ISpeechBaseStream, ISpeechBaseStreamVtbl,
    ISpeechFileStream, ISpeechFileStreamVtbl,
    ISpeechMemoryStream, ISpeechMemoryStreamVtbl,
    ISpeechCustomStream, ISpeechCustomStreamVtbl,
    ISpeechAudio, ISpeechAudioVtbl,
    ISpeechMMSysAudio, ISpeechMMSysAudioVtbl,
    ISpeechVoice, ISpeechVoiceVtbl,
    ISpeechVoiceStatus, ISpeechVoiceStatusVtbl,
    _ISpeechVoiceEvents, _ISpeechVoiceEventsVtbl,
    ISpeechRecognizer, ISpeechRecognizerVtbl,
    ISpeechRecognizerStatus, ISpeechRecognizerStatusVtbl,
    ISpeechRecoContext, ISpeechRecoContextVtbl,
    ISpeechRecoGrammar, ISpeechRecoGrammarVtbl,
    _ISpeechRecoContextEvents, _ISpeechRecoContextEventsVtbl,
    ISpeechGrammarRule, ISpeechGrammarRuleVtbl,
    ISpeechGrammarRules, ISpeechGrammarRulesVtbl,
    ISpeechGrammarRuleState, ISpeechGrammarRuleStateVtbl,
    ISpeechGrammarRuleStateTransition, ISpeechGrammarRuleStateTransitionVtbl,
    ISpeechGrammarRuleStateTransitions, ISpeechGrammarRuleStateTransitionsVtbl,
    ISpeechTextSelectionInformation, ISpeechTextSelectionInformationVtbl,
    ISpeechRecoResult, ISpeechRecoResultVtbl,
};
RIDL!(#[uuid(0x8e0a246d, 0xd3c8, 0x45de, 0x86, 0x57, 0x04, 0x29, 0x0c, 0x45, 0x8c, 0x3c)]
interface ISpeechRecoResult2(ISpeechRecoResult2Vtbl): ISpeechRecoResult(ISpeechRecoResultVtbl) {
    fn SetTextFeedback(
        Feedback: BSTR,
        WasSuccessful: VARIANT_BOOL
    ) -> HRESULT
});
pub use um::sapi51::{
    ISpeechRecoResultTimes, ISpeechRecoResultTimesVtbl,
    ISpeechPhraseAlternate, ISpeechPhraseAlternateVtbl,
    ISpeechPhraseAlternates, ISpeechPhraseAlternatesVtbl,
    ISpeechPhraseInfo, ISpeechPhraseInfoVtbl,
    ISpeechPhraseElement, ISpeechPhraseElementVtbl,
    ISpeechPhraseElements, ISpeechPhraseElementsVtbl,
    ISpeechPhraseReplacement, ISpeechPhraseReplacementVtbl,
    ISpeechPhraseReplacements, ISpeechPhraseReplacementsVtbl,
    ISpeechPhraseProperty, ISpeechPhrasePropertyVtbl,
    ISpeechPhraseProperties, ISpeechPhrasePropertiesVtbl,
    ISpeechPhraseRule, ISpeechPhraseRuleVtbl,
    ISpeechPhraseRules, ISpeechPhraseRulesVtbl,
    ISpeechLexicon, ISpeechLexiconVtbl,
    ISpeechLexiconWords, ISpeechLexiconWordsVtbl,
    ISpeechLexiconWord, ISpeechLexiconWordVtbl,
    ISpeechLexiconPronunciations, ISpeechLexiconPronunciationsVtbl,
    ISpeechLexiconPronunciation, ISpeechLexiconPronunciationVtbl,
    Speech_Default_Weight,
    Speech_Max_Word_Length,
    Speech_Max_Pron_Length,
    Speech_StreamPos_Asap,
    Speech_StreamPos_RealTime,
    SpeechAllElements,
};
RIDL!(#[uuid(0xaaec54af, 0x8f85, 0x4924, 0x94, 0x4d, 0xb7, 0x9d, 0x39, 0xd7, 0x2e, 0x19)]
interface ISpeechXMLRecoResult(ISpeechXMLRecoResultVtbl): ISpeechRecoResult(ISpeechRecoResultVtbl) {
    fn GetXMLResult(
        Options: SPXMLRESULTOPTIONS,
        pResult: *mut BSTR
    ) -> HRESULT,
    fn GetXMLErrorInfo(
        LineNumber: *mut c_long,
        ScriptLine: *mut BSTR,
        Source: *mut BSTR,
        Description: *mut BSTR,
        ResultCode: *mut c_long,
        IsError: *mut VARIANT_BOOL
    ) -> HRESULT
});
RIDL!(#[uuid(0x6d60eb64, 0xaced, 0x40a6, 0xbb, 0xf3, 0x4e, 0x55, 0x7f, 0x71, 0xde, 0xe2)]
interface ISpeechRecoResultDispatch(ISpeechRecoResultDispatchVtbl): IDispatch(IDispatchVtbl) {
    fn get_RecoContext(RecoContext: *mut ISpeechRecoContext) -> HRESULT,
    fn get_Times(Times: *mut ISpeechRecoResultTimes) -> HRESULT,
    fn putref_AudioFormat(Format: *mut ISpeechAudioFormat) -> HRESULT,
    fn get_AudioFormat(Format: *mut *mut ISpeechAudioFormat) -> HRESULT,
    fn get_PhraseInfo(PhraseInfo: *mut *mut ISpeechPhraseInfo) -> HRESULT,
    fn Alternates(
        RequestCount: c_long,
        StartElement: c_long,
        Elements: c_long,
        Alternates: *mut *mut ISpeechPhraseAlternates
    ) -> HRESULT,
    fn Audio(
        StartElement: c_long,
        Elements: c_long,
        Stream: *mut *mut ISpeechMemoryStream
    ) -> HRESULT,
    fn SpeakAudio(
        StartElement: c_long,
        Elements: c_long,
        Flags: SpeechVoiceSpeakFlags,
        StreamNumber: *mut c_long
    ) -> HRESULT,
    fn SaveToMemory(ResultBlock: *mut VARIANT) -> HRESULT,
    fn DiscardResultInfo(ValueTypes: SpeechDiscardType) -> HRESULT,
    fn GetXMLResult(
        Options: SPXMLRESULTOPTIONS,
        pResult: *mut BSTR
    ) -> HRESULT,
    fn GetXMLErrorInfo(
        LineNumber: *mut c_long,
        ScriptLine: *mut BSTR,
        Source: *mut BSTR,
        Description: *mut BSTR,
        ResultCode: *mut HRESULT,
        IsError: *mut VARIANT_BOOL
    ) -> HRESULT,
    fn SetTextFeedback(
        Feedback: BSTR,
        WasSuccessful: VARIANT_BOOL
    ) -> HRESULT
});
pub use um::sapi51::{
    ISpeechPhraseInfoBuilder, ISpeechPhraseInfoBuilderVtbl,
    ISpeechPhoneConverter, ISpeechPhoneConverterVtbl,
    CLSID_SpNotifyTranslator,
    CLSID_SpObjectTokenCategory,
    CLSID_SpObjectToken,
    CLSID_SpResourceManager,
    CLSID_SpStreamFormatConverter,
    CLSID_SpMMAudioEnum,
    CLSID_SpMMAudioIn,
    CLSID_SpMMAudioOut,
    CLSID_SpStream,
    CLSID_SpVoice,
    CLSID_SpSharedRecoContext,
    CLSID_SpInprocRecognizer,
    CLSID_SpSharedRecognizer,
    CLSID_SpLexicon,
    CLSID_SpUnCompressedLexicon,
    CLSID_SpCompressedLexicon,
};
extern {
    pub static CLSID_SpShortcut: CLSID;
}
pub use um::sapi51::CLSID_SpPhoneConverter;
extern {
    pub static CLSID_SpPhoneticAlphabetConverter: CLSID;
}
pub use um::sapi51::{
    CLSID_SpNullPhoneConverter,
    CLSID_SpTextSelectionInformation,
    CLSID_SpPhraseInfoBuilder,
    CLSID_SpAudioFormat,
    CLSID_SpWaveFormatEx,
    CLSID_SpInProcRecoContext,
    CLSID_SpCustomStream,
    CLSID_SpFileStream,
    CLSID_SpMemoryStream,
};

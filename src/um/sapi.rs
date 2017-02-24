// Copyright © 2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! SAPI 5.4 definitions
use shared::guiddef::GUID;
use shared::minwindef::{BYTE, ULONG, WORD};
use shared::rpcdce::RPC_IF_HANDLE;
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::{HRESULT, LPWSTR, ULONGLONG};
pub use um::sapi53::{
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
    SPTOKENKEY_ATTRIBUTES,
    SPTOKENKEY_RETAINEDAUDIO,
    SPTOKENKEY_AUDIO_LATENCY_WARNING,
    SPTOKENKEY_AUDIO_LATENCY_TRUNCATE,
    SPTOKENKEY_AUDIO_LATENCY_UPDATE_INTERVAL,
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
    SPDICTATION,
    SPREG_SAFE_USER_TOKENS,
    SPINFDICTATION,
    SP_LOW_CONFIDENCE,
    SP_NORMAL_CONFIDENCE,
    SP_HIGH_CONFIDENCE,
    DEFAULT_WEIGHT,
    SP_MAX_WORD_LENGTH,
    SP_MAX_PRON_LENGTH,
    SP_EMULATE_RESULT,
    ISpNotifyCallback,
    SPNOTIFYCALLBACK,
};
extern {
    pub static __MIDL_itf_sapi_0000_0000_v0_0_c_ifspec: RPC_IF_HANDLE;
    pub static __MIDL_itf_sapi_0000_0000_v0_0_s_ifspec: RPC_IF_HANDLE;
}
pub use um::sapi53::{
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
    SPEI_SR_AUDIO_LEVEL,
    SPEI_SR_RETAINEDAUDIO,
    SPEI_SR_PRIVATE,
};
pub const ACTIVE_CATEGORY_CHANGED: SPEVENTENUM = 53;
pub use um::sapi53::{
    SPEI_RESERVED5,
    SPEI_RESERVED6,
    SPEI_MIN_SR,
    SPEI_MAX_SR,
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
    SPSERIALIZEDEVENT64,
    SPEVENTEX,
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
    SPESF_STREAM_RELEASED,
    SPESF_EMULATED,
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
};
extern {
    pub static __MIDL_itf_sapi_0000_0011_v0_0_c_ifspec: RPC_IF_HANDLE;
    pub static __MIDL_itf_sapi_0000_0011_v0_0_s_ifspec: RPC_IF_HANDLE;
}
pub use um::sapi53::{
    ISpEventSource, ISpEventSourceVtbl,
    ISpEventSource2, ISpEventSource2Vtbl,
    ISpEventSink, ISpEventSinkVtbl,
    ISpStreamFormat, ISpStreamFormatVtbl,
    SPFILEMODE,
    SPFM_OPEN_READONLY,
    SPFM_OPEN_READWRITE,
    SPFM_CREATE,
    SPFM_CREATE_ALWAYS,
    SPFM_NUM_MODES
};
extern {
    pub static __MIDL_itf_sapi_0000_0015_v0_0_c_ifspec: RPC_IF_HANDLE;
    pub static __MIDL_itf_sapi_0000_0015_v0_0_s_ifspec: RPC_IF_HANDLE;
}
pub use um::sapi53::{
    ISpStream, ISpStreamVtbl,
    ISpStreamFormatConverter, ISpStreamFormatConverterVtbl,
    SPAUDIOSTATE,
    SPAS_CLOSED,
    SPAS_STOP,
    SPAS_PAUSE,
    SPAS_RUN,
    SPAUDIOSTATUS,
    SPAUDIOBUFFERINFO
};
extern {
    pub static __MIDL_itf_sapi_0000_0017_v0_0_c_ifspec: RPC_IF_HANDLE;
    pub static __MIDL_itf_sapi_0000_0017_v0_0_s_ifspec: RPC_IF_HANDLE;
}
pub use um::sapi53::{
    ISpAudio, ISpAudioVtbl,
    ISpMMSysAudio, ISpMMSysAudioVtbl,
    ISpTranscript, ISpTranscriptVtbl,
    SPDISPLAYATTRIBUTES,
    SPAF_ONE_TRAILING_SPACE,
    SPAF_TWO_TRAILING_SPACES,
    SPAF_CONSUME_LEADING_SPACES,
    SPAF_BUFFER_POSITION,
    SPAF_ALL,
    SPAF_USER_SPECIFIED,
    SPPHONEID,
    PSPPHONEID,
    PCSPPHONEID,
    SPPHRASEELEMENT,
    SPPHRASERULE,
    SPPHRASEPROPERTYUNIONTYPE,
    SPPPUT_UNUSED,
    SPPPUT_ARRAY_INDEX,
    SPPHRASEPROPERTY,
    SPPHRASEREPLACEMENT,
    SPSEMANTICERRORINFO,
    SPSEMANTICFORMAT,
    SPPHRASE_50,
//  SPPHRASESIZE_500,
};
pub use um::sapi53::SPPHRASE as SPPHRASE_53;
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
    SemanticTagFormat: SPSEMANTICFORMAT,
}}
pub use um::sapi53::{
    SPSERIALIZEDPHRASE,
    SPRULE,
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
    SPREF_Emulated,
    SPREF_SMLTimeout,
    SPREF_ExtendableParse,
    SPREF_ReSent,
    SPREF_Hypothesis,
    SPREF_FalseRecognition,
    SPPARTOFSPEECH,
    SPPS_NotOverriden,
    SPPS_Unknown,
    SPPS_Noun,
    SPPS_Verb,
    SPPS_Modifier,
    SPPS_Function,
    SPPS_Interjection,
    SPPS_Noncontent,
    SPPS_LMA,
    SPPS_SuppressWord,
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
    eWORDTYPE_DELETED,
    SPPRONUNCIATIONFLAGS,
    ePRONFLAG_USED,
    SPWORDPRONUNCIATION,
    SPWORDPRONUNCIATIONLIST,
    SPWORD,
    SPWORDLIST,
};
extern {
    pub static __MIDL_itf_sapi_0000_0020_v0_0_c_ifspec: RPC_IF_HANDLE;
    pub static __MIDL_itf_sapi_0000_0020_v0_0_s_ifspec: RPC_IF_HANDLE;
}
pub use um::sapi53::{
    ISpLexicon, ISpLexiconVtbl,
    ISpContainerLexicon, ISpContainerLexiconVtbl,
    SPSHORTCUTTYPE,
    SPSHT_NotOverriden,
    SPSHT_Unknown,
    SPSHT_EMAIL,
    SPSHT_OTHER,
    SPPS_RESERVED1,
    SPPS_RESERVED2,
    SPPS_RESERVED3,
    SPPS_RESERVED4,
    SPSHORTCUTPAIR,
    SPSHORTCUTPAIRLIST,
};
extern {
    pub static __MIDL_itf_sapi_0000_0022_v0_0_c_ifspec: RPC_IF_HANDLE;
    pub static __MIDL_itf_sapi_0000_0022_v0_0_s_ifspec: RPC_IF_HANDLE;
}
pub use um::sapi53::{
    ISpShortcut, ISpShortcutVtbl,
    ISpPhoneConverter, ISpPhoneConverterVtbl,
    ISpPhoneticAlphabetConverter, ISpPhoneticAlphabetConverterVtbl,
    ISpPhoneticAlphabetSelection, ISpPhoneticAlphabetSelectionVtbl,
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
    SPF_PARSE_SAPI,
    SPF_PARSE_SSML,
    SPF_PARSE_AUTODETECT,
    SPF_NLP_MASK,
    SPF_PARSE_MASK,
    SPF_VOICE_MASK,
    SPF_UNUSED_FLAGS,
};
extern {
    pub static __MIDL_itf_sapi_0000_0026_v0_0_c_ifspec: RPC_IF_HANDLE;
    pub static __MIDL_itf_sapi_0000_0026_v0_0_s_ifspec: RPC_IF_HANDLE;
}
pub use um::sapi53::{
    ISpVoice, ISpVoiceVtbl,
    ISpPhrase, ISpPhraseVtbl,
    ISpPhraseAlt, ISpPhraseAltVtbl,
    SPXMLRESULTOPTIONS,
    SPXRO_SML,
    SPXRO_Alternates_SML,
};
extern {
    pub static __MIDL_itf_sapi_0000_0029_v0_0_c_ifspe: RPC_IF_HANDLE;
    pub static __MIDL_itf_sapi_0000_0029_v0_0_s_ifspe: RPC_IF_HANDLE;
}
pub use um::sapi53::{
    ISpPhrase2, ISpPhrase2Vtbl,
    SPRECORESULTTIMES,
    SPSERIALIZEDRESULT,
};
extern {
    pub static __MIDL_itf_sapi_0000_0030_v0_0_c_ifspec: RPC_IF_HANDLE;
    pub static __MIDL_itf_sapi_0000_0030_v0_0_s_ifspec: RPC_IF_HANDLE;
}
pub use um::sapi53::{
    ISpRecoResult, ISpRecoResultVtbl,
    SPCOMMITFLAGS,
    SPCF_NONE,
    SPCF_ADD_TO_USER_LEXICON,
    SPCF_DEFINITE_CORRECTION,
};
extern {
    pub static __MIDL_itf_sapi_0000_0031_v0_0_c_ifspec: RPC_IF_HANDLE;
    pub static __MIDL_itf_sapi_0000_0031_v0_0_s_ifspec: RPC_IF_HANDLE;
}
pub use um::sapi53::{
    ISpRecoResult2, ISpRecoResult2Vtbl,
    ISpXMLRecoResult, ISpXMLRecoResultVtbl,
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
    SPWT_LEXICAL_NO_SPECIAL_CHARS,
    SPPROPERTYINFO,
    SPCFGRULEATTRIBUTES,
    SPRAF_TopLevel,
    SPRAF_Active,
    SPRAF_Export,
    SPRAF_Import,
    SPRAF_Interpreter,
    SPRAF_Dynamic,
    SPRAF_Root,
    SPRAF_AutoPause,
    SPRAF_UserDelimited,
};
extern {
    pub static __MIDL_itf_sapi_0000_0033_v0_0_c_ifspec: RPC_IF_HANDLE;
    pub static __MIDL_itf_sapi_0000_0033_v0_0_s_ifspec: RPC_IF_HANDLE;
}
pub use um::sapi53::{
    ISpGrammarBuilder, ISpGrammarBuilderVtbl,
    SPLOADOPTIONS,
    SPLO_STATIC,
    SPLO_DYNAMIC,
};
extern {
    pub static __MIDL_itf_sapi_0000_0034_v0_0_c_ifspec: RPC_IF_HANDLE;
    pub static __MIDL_itf_sapi_0000_0034_v0_0_s_ifspec: RPC_IF_HANDLE;
}
pub use um::sapi53::{
    ISpRecoGrammar, ISpRecoGrammarVtbl,
    SPMATCHINGMODE,
    AllWords,
    Subsequence,
    OrderedSubset,
    SubsequenceContentRequired,
    OrderedSubsetContentRequired,
    PHONETICALPHABET,
    PA_Ipa,
    PA_Ups,
    PA_Sapi,
};
extern {
    pub static __MIDL_itf_sapi_0000_0035_v0_0_c_ifspec: RPC_IF_HANDLE;
    pub static __MIDL_itf_sapi_0000_0035_v0_0_s_ifspec: RPC_IF_HANDLE;
}
pub use um::sapi53::{
    ISpGrammarBuilder2, ISpGrammarBuilder2Vtbl,
//  SPRP_NORMAL,
};
extern {
    pub static __MIDL_itf_sapi_0000_0036_v0_0_c_ifspec: RPC_IF_HANDLE;
    pub static __MIDL_itf_sapi_0000_0036_v0_0_s_ifspec: RPC_IF_HANDLE;
}
pub use um::sapi53::{
    ISpRecoGrammar2, ISpRecoGrammar2Vtbl,
    ISpeechResourceLoader, ISpeechResourceLoaderVtbl,
    SPRECOCONTEXTSTATUS,
    SPBOOKMARKOPTIONS,
    SPBO_NONE,
    SPBO_PAUSE,
    SPBO_AHEAD,
    SPBO_TIME_UNITS,
    SPAUDIOOPTIONS,
    SPAO_NONE,
    SPAO_RETAIN_AUDIO,
};
extern {
    pub static __MIDL_itf_sapi_0000_0038_v0_0_c_ifspec: RPC_IF_HANDLE;
    pub static __MIDL_itf_sapi_0000_0038_v0_0_s_ifspec: RPC_IF_HANDLE;
}
pub use um::sapi53::{
    ISpRecoContext, ISpRecoContextVtbl,
    SPGRAMMAROPTIONS,
    SPGO_SAPI,
    SPGO_SRGS,
    SPGO_UPS,
    SPGO_SRGS_MS_SCRIPT,
    SPGO_SRGS_W3C_SCRIPT,
    SPGO_SRGS_STG_SCRIPT,
    SPGO_SRGS_SCRIPT,
    SPGO_FILE,
    SPGO_HTTP,
    SPGO_RES,
    SPGO_OBJECT,
    SPGO_DEFAULT,
    SPGO_ALL,
    SPADAPTATIONSETTINGS,
    SPADS_Default,
    SPADS_CurrentRecognizer,
    SPADS_RecoProfile,
    SPADS_Immediate,
    SPADS_Reset,
    SPADS_HighVolumeDataSource,
    SPADAPTATIONRELEVANCE,
    SPAR_Unknown,
    SPAR_Low,
    SPAR_Medium,
    SPAR_High,
};
extern {
    pub static __MIDL_itf_sapi_0000_0039_v0_0_c_ifspec: RPC_IF_HANDLE;
    pub static __MIDL_itf_sapi_0000_0039_v0_0_s_ifspec: RPC_IF_HANDLE;
}
pub use um::sapi53::{
    ISpRecoContext2, ISpRecoContext2Vtbl,
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
};
extern {
    pub static __MIDL_itf_sapi_0000_0041_v0_0_c_ifspec: RPC_IF_HANDLE;
    pub static __MIDL_itf_sapi_0000_0041_v0_0_s_ifspec: RPC_IF_HANDLE;
}
pub use um::sapi53::{
    ISpRecognizer, ISpRecognizerVtbl,
    ISpSerializeState, ISpSerializeStateVtbl,
    ISpRecognizer2, ISpRecognizer2Vtbl,
};
ENUM!{enum SPCATEGORYTYPE {
    SPCT_COMMAND,
    SPCT_DICTATION,
    SPCT_SLEEP,
    SPCT_SUB_COMMAND,
    SPCT_SUB_DICTATION,
}}
extern {
    pub static __MIDL_itf_sapi_0000_0044_v0_0_c_ifspec: RPC_IF_HANDLE;
    pub static __MIDL_itf_sapi_0000_0044_v0_0_s_ifspec: RPC_IF_HANDLE;
}
RIDL!(#[uuid(0xda0cd0f9, 0x14a2, 0x4f09, 0x8c, 0x2a, 0x85, 0xcc, 0x48, 0x97, 0x93, 0x45)]
interface ISpRecoCategory(ISpRecoCategoryVtbl): IUnknown(IUnknownVtbl) {
    fn GetType(peCategoryType: *mut SPCATEGORYTYPE) -> HRESULT
});
RIDL!(#[uuid(0xdf1b943c, 0x5838, 0x4aa2, 0x87, 0x06, 0xd7, 0xcd, 0x5b, 0x33, 0x34, 0x99)]
interface ISpRecognizer3(ISpRecognizer3Vtbl): IUnknown(IUnknownVtbl) {
    fn GetCategory(
        categoryType: SPCATEGORYTYPE,
        ppCategory: *mut *mut ISpRecoCategory
    ) -> HRESULT,
    fn SetActiveCategory(pCategory: *mut ISpRecoCategory) -> HRESULT,
    fn GetActiveCategory(ppCategory: *mut *mut ISpRecoCategory) -> HRESULT
});
pub use um::sapi53::SPNORMALIZATIONLIST;
extern {
    pub static __MIDL_itf_sapi_0000_0046_v0_0_c_ifspec: RPC_IF_HANDLE;
    pub static __MIDL_itf_sapi_0000_0046_v0_0_s_ifspec: RPC_IF_HANDLE;
}
pub use um::sapi53::{
    ISpEnginePronunciation, ISpEnginePronunciationVtbl,
    SPDISPLAYTOKEN,
    SPDISPLAYPHRASE,
};
extern {
    pub static __MIDL_itf_sapi_0000_0045_v0_0_c_ifspec: RPC_IF_HANDLE;
    pub static __MIDL_itf_sapi_0000_0045_v0_0_s_ifspec: RPC_IF_HANDLE;
}
pub use um::sapi53::{ISpDisplayAlternates, ISpDisplayAlternatesVtbl};

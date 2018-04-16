// Copyright © 2018 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use ctypes::{c_float, c_void};
use shared::basetsd::{INT32, UINT32, UINT64};
use shared::minwindef::{BOOL, BYTE, DWORD};
use shared::mmreg::WAVEFORMATEX;
use shared::winerror::HRESULT;
use um::audiosessiontypes::AUDIO_STREAM_CATEGORY;
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::LPCWSTR;
pub const XAUDIO2_MAX_BUFFER_BYTES: u32 = 0x80000000;
pub const XAUDIO2_MAX_QUEUED_BUFFERS: u32 = 64;
pub const XAUDIO2_MAX_BUFFERS_SYSTEM: u32 = 2;
pub const XAUDIO2_MAX_AUDIO_CHANNELS: u32 = 64;
pub const XAUDIO2_MIN_SAMPLE_RATE: u32 = 1000;
pub const XAUDIO2_MAX_SAMPLE_RATE: u32 = 200000;
pub const XAUDIO2_MAX_VOLUME_LEVEL: f32 = 16777216.0f32;
pub const XAUDIO2_MIN_FREQ_RATIO: f32 = 1.0f32/1024.0f32;
pub const XAUDIO2_MAX_FREQ_RATIO: f32 = 1024.0f32;
pub const XAUDIO2_DEFAULT_FREQ_RATIO: f32 = 2.0f32;
pub const XAUDIO2_MAX_FILTER_ONEOVERQ: f32 = 1.5f32;
pub const XAUDIO2_MAX_FILTER_FREQUENCY: f32 = 1.0f32;
pub const XAUDIO2_MAX_LOOP_COUNT: u32 = 254;
pub const XAUDIO2_MAX_INSTANCES: u32 = 8;
pub const XAUDIO2_MAX_RATIO_TIMES_RATE_XMA_MONO: u32 = 600000;
pub const XAUDIO2_MAX_RATIO_TIMES_RATE_XMA_MULTICHANNEL: u32 = 300000;
pub const XAUDIO2_COMMIT_NOW: u32 = 0;
pub const XAUDIO2_COMMIT_ALL: u32 = 0;
pub const XAUDIO2_INVALID_OPSET: u32 = -1i32 as u32;
pub const XAUDIO2_NO_LOOP_REGION: u32 = 0;
pub const XAUDIO2_LOOP_INFINITE: u32 = 255;
pub const XAUDIO2_DEFAULT_CHANNELS: u32 = 0;
pub const XAUDIO2_DEFAULT_SAMPLERATE: u32 = 0;
pub const XAUDIO2_DEBUG_ENGINE: u32 = 0x0001;
pub const XAUDIO2_VOICE_NOPITCH: u32 = 0x0002;
pub const XAUDIO2_VOICE_NOSRC: u32 = 0x0004;
pub const XAUDIO2_VOICE_USEFILTER: u32 = 0x0008;
pub const XAUDIO2_PLAY_TAILS: u32 = 0x0020;
pub const XAUDIO2_END_OF_STREAM: u32 = 0x0040;
pub const XAUDIO2_SEND_USEFILTER: u32 = 0x0080;
pub const XAUDIO2_VOICE_NOSAMPLESPLAYED: u32 = 0x0100;
pub const XAUDIO2_STOP_ENGINE_WHEN_IDLE: u32 = 0x2000;
pub const XAUDIO2_1024_QUANTUM: u32 = 0x8000;
pub const XAUDIO2_NO_VIRTUAL_AUDIO_CLIENT: u32 = 0x10000;
pub const XAUDIO2_DEFAULT_FILTER_TYPE: XAUDIO2_FILTER_TYPE = LowPassFilter;
pub const XAUDIO2_DEFAULT_FILTER_FREQUENCY: f32 = XAUDIO2_MAX_FILTER_FREQUENCY;
pub const XAUDIO2_DEFAULT_FILTER_ONEOVERQ: f32 = 1.0f32;
pub const XAUDIO2_QUANTUM_NUMERATOR: u32 = 1;
pub const XAUDIO2_QUANTUM_DENOMINATOR: u32 = 100;
pub const XAUDIO2_QUANTUM_MS: u32 = 1000 * XAUDIO2_QUANTUM_NUMERATOR / XAUDIO2_QUANTUM_DENOMINATOR;
pub const FACILITY_XAUDIO2: HRESULT = 0x896;
pub const XAUDIO2_E_INVALID_CALL: HRESULT = 0x88960001;
pub const XAUDIO2_E_XMA_DECODER_ERROR: HRESULT = 0x88960002;
pub const XAUDIO2_E_XAPO_CREATION_FAILED: HRESULT = 0x88960003;
pub const XAUDIO2_E_DEVICE_INVALIDATED: HRESULT = 0x88960004;
pub type XAUDIO2_PROCESSOR = UINT32;
pub const Processor1: XAUDIO2_PROCESSOR = 0x00000001;
pub const Processor2: XAUDIO2_PROCESSOR = 0x00000002;
pub const Processor3: XAUDIO2_PROCESSOR = 0x00000004;
pub const Processor4: XAUDIO2_PROCESSOR = 0x00000008;
pub const Processor5: XAUDIO2_PROCESSOR = 0x00000010;
pub const Processor6: XAUDIO2_PROCESSOR = 0x00000020;
pub const Processor7: XAUDIO2_PROCESSOR = 0x00000040;
pub const Processor8: XAUDIO2_PROCESSOR = 0x00000080;
pub const Processor9: XAUDIO2_PROCESSOR = 0x00000100;
pub const Processor10: XAUDIO2_PROCESSOR = 0x00000200;
pub const Processor11: XAUDIO2_PROCESSOR = 0x00000400;
pub const Processor12: XAUDIO2_PROCESSOR = 0x00000800;
pub const Processor13: XAUDIO2_PROCESSOR = 0x00001000;
pub const Processor14: XAUDIO2_PROCESSOR = 0x00002000;
pub const Processor15: XAUDIO2_PROCESSOR = 0x00004000;
pub const Processor16: XAUDIO2_PROCESSOR = 0x00008000;
pub const Processor17: XAUDIO2_PROCESSOR = 0x00010000;
pub const Processor18: XAUDIO2_PROCESSOR = 0x00020000;
pub const Processor19: XAUDIO2_PROCESSOR = 0x00040000;
pub const Processor20: XAUDIO2_PROCESSOR = 0x00080000;
pub const Processor21: XAUDIO2_PROCESSOR = 0x00100000;
pub const Processor22: XAUDIO2_PROCESSOR = 0x00200000;
pub const Processor23: XAUDIO2_PROCESSOR = 0x00400000;
pub const Processor24: XAUDIO2_PROCESSOR = 0x00800000;
pub const Processor25: XAUDIO2_PROCESSOR = 0x01000000;
pub const Processor26: XAUDIO2_PROCESSOR = 0x02000000;
pub const Processor27: XAUDIO2_PROCESSOR = 0x04000000;
pub const Processor28: XAUDIO2_PROCESSOR = 0x08000000;
pub const Processor29: XAUDIO2_PROCESSOR = 0x10000000;
pub const Processor30: XAUDIO2_PROCESSOR = 0x20000000;
pub const Processor31: XAUDIO2_PROCESSOR = 0x40000000;
pub const Processor32: XAUDIO2_PROCESSOR = 0x80000000;
pub const XAUDIO2_ANY_PROCESSOR: XAUDIO2_PROCESSOR = 0xffffffff;
pub const XAUDIO2_DEFAULT_PROCESSOR: XAUDIO2_PROCESSOR = Processor1;
STRUCT!{#[repr(packed)] struct XAUDIO2_VOICE_DETAILS {
    CreationFlags: UINT32,
    ActiveFlags: UINT32,
    InputChannels: UINT32,
    InputSampleRate: UINT32,
}}
STRUCT!{#[repr(packed)] struct XAUDIO2_SEND_DESCRIPTOR {
    Flags: UINT32,
    pOutputVoice: *mut IXAudio2Voice,
}}
STRUCT!{#[repr(packed)] struct XAUDIO2_VOICE_SENDS {
    SendCount: UINT32,
    pSends: *mut XAUDIO2_SEND_DESCRIPTOR,
}}
STRUCT!{#[repr(packed)] struct XAUDIO2_EFFECT_DESCRIPTOR {
    pEffect: *mut IUnknown,
    InitialState: BOOL,
    OutputChannels: UINT32,
}}
STRUCT!{#[repr(packed)] struct XAUDIO2_EFFECT_CHAIN {
    EffectCount: UINT32,
    pEffectDescriptors: *mut XAUDIO2_EFFECT_DESCRIPTOR,
}}
ENUM!{enum XAUDIO2_FILTER_TYPE {
    LowPassFilter,
    BandPassFilter,
    HighPassFilter,
    NotchFilter,
    LowPassOnePoleFilter,
    HighPassOnePoleFilter,
}}
STRUCT!{#[repr(packed)] struct XAUDIO2_FILTER_PARAMETERS {
    Type: XAUDIO2_FILTER_TYPE,
    Frequency: c_float,
    OneOverQ: c_float,
}}
STRUCT!{#[repr(packed)] struct XAUDIO2_BUFFER {
    Flags: UINT32,
    AudioBytes: UINT32,
    pAudioData: *const BYTE,
    PlayBegin: UINT32,
    PlayLength: UINT32,
    LoopBegin: UINT32,
    LoopLength: UINT32,
    LoopCount: UINT32,
    pContext: *mut c_void,
}}
STRUCT!{#[repr(packed)] struct XAUDIO2_BUFFER_WMA {
    pDecodedPacketCumulativeBytes: *const UINT32,
    PacketCount: UINT32,
}}
STRUCT!{#[repr(packed)] struct XAUDIO2_VOICE_STATE {
    pCurrentBufferContext: *mut c_void,
    BuffersQueued: UINT32,
    SamplesPlayed: UINT64,
}}
STRUCT!{#[repr(packed)] struct XAUDIO2_PERFORMANCE_DATA {
    AudioCyclesSinceLastQuery: UINT64,
    TotalCyclesSinceLastQuery: UINT64,
    MinimumCyclesPerQuantum: UINT32,
    MaximumCyclesPerQuantum: UINT32,
    MemoryUsageInBytes: UINT32,
    CurrentLatencyInSamples: UINT32,
    GlitchesSinceEngineStarted: UINT32,
    ActiveSourceVoiceCount: UINT32,
    TotalSourceVoiceCount: UINT32,
    ActiveSubmixVoiceCount: UINT32,
    ActiveResamplerCount: UINT32,
    ActiveMatrixMixCount: UINT32,
    ActiveXmaSourceVoices: UINT32,
    ActiveXmaStreams: UINT32,
}}
STRUCT!{#[repr(packed)] struct XAUDIO2_DEBUG_CONFIGURATION {
    TraceMask: UINT32,
    BreakMask: UINT32,
    LogThreadID: BOOL,
    LogFileline: BOOL,
    LogFunctionName: BOOL,
    LogTiming: BOOL,
}}
pub const XAUDIO2_LOG_ERRORS: u32 = 0x0001;
pub const XAUDIO2_LOG_WARNINGS: u32 = 0x0002;
pub const XAUDIO2_LOG_INFO: u32 = 0x0004;
pub const XAUDIO2_LOG_DETAIL: u32 = 0x0008;
pub const XAUDIO2_LOG_API_CALLS: u32 = 0x0010;
pub const XAUDIO2_LOG_FUNC_CALLS: u32 = 0x0020;
pub const XAUDIO2_LOG_TIMING: u32 = 0x0040;
pub const XAUDIO2_LOG_LOCKS: u32 = 0x0080;
pub const XAUDIO2_LOG_MEMORY: u32 = 0x0100;
pub const XAUDIO2_LOG_STREAMING: u32 = 0x1000;
RIDL!{#[uuid(0x2b02e3cf, 0x2e0b, 0x4ec3, 0xbe, 0x45, 0x1b, 0x2a, 0x3f, 0xe7, 0x21, 0x0d)]
interface IXAudio2(IXAudio2Vtbl): IUnknown(IUnknownVtbl) {
    fn RegisterForCallbacks(
        pCallback: *mut IXAudio2EngineCallback,
    ) -> HRESULT,
    fn UnregisterForCallbacks(
        pCallback: *mut IXAudio2EngineCallback,
    ) -> (),
    fn CreateSourceVoice(
        ppSourceVoice: *mut *mut IXAudio2SourceVoice,
        pSourceFormat: *const WAVEFORMATEX,
        Flags: UINT32,
        MaxFrequencyRatio: c_float,
        pCallback: *mut IXAudio2VoiceCallback,
        pSendList: *const XAUDIO2_VOICE_SENDS,
        pEffectChain: *const XAUDIO2_EFFECT_CHAIN,
    ) -> HRESULT,
    fn CreateSubmixVoice(
        ppSubmixVoice: *mut *mut IXAudio2SubmixVoice,
        InputChannels: UINT32,
        InputSampleRate: UINT32,
        Flags: UINT32,
        ProcessingStage: UINT32,
        pSendList: *const XAUDIO2_VOICE_SENDS,
        pEffectChain: *const XAUDIO2_EFFECT_CHAIN,
    ) -> HRESULT,
    fn CreateMasteringVoice(
        ppMasteringVoice: *mut *mut IXAudio2MasteringVoice,
        InputChannels: UINT32,
        InputSampleRate: UINT32,
        Flags: UINT32,
        szDeviceId: LPCWSTR,
        pEffectChain: *const XAUDIO2_EFFECT_CHAIN,
        StreamCategory: AUDIO_STREAM_CATEGORY,
    ) -> HRESULT,
    fn StartEngine() -> HRESULT,
    fn StopEngine() -> (),
    fn CommitChanges(
        OperationSet: UINT32,
    ) -> HRESULT,
    fn GetPerformanceData(
        pPerfData: *mut XAUDIO2_PERFORMANCE_DATA,
    ) -> (),
    fn SetDebugConfiguration(
        pDebugConfiguration: *const XAUDIO2_DEBUG_CONFIGURATION,
        pReserved: *mut c_void,
    ) -> (),
}}
RIDL!{
interface IXAudio2Voice(IXAudio2VoiceVtbl) {
    fn GetVoiceDetails(
        pVoiceDetails: *mut XAUDIO2_VOICE_DETAILS,
    ) -> (),
    fn SetOutputVoices(
        pSendList: *const XAUDIO2_VOICE_SENDS,
    ) -> HRESULT,
    fn SetEffectChain(
        pEffectChain: *const XAUDIO2_EFFECT_CHAIN,
    ) -> HRESULT,
    fn EnableEffect(
        EffectIndex: INT32,
        OperationSet: UINT32,
    ) -> HRESULT,
    fn DisableEffect(
        EffectIndex: UINT32,
        OperationSet: UINT32,
    ) -> HRESULT,
    fn GetEffectState(
        EffectIndex: UINT32,
        pEnabled: *mut BOOL,
    ) -> (),
    fn SetEffectParameters(
        EffectIndex: UINT32,
        pParameters: *const c_void,
        ParametersByteSize: UINT32,
        OperationSet: UINT32,
    ) -> HRESULT,
    fn GetEffectParameters(
        EffectIndex: UINT32,
        pParameters: *mut c_void,
        ParametersByteSize: UINT32,
    ) -> HRESULT,
    fn SetFilterParameters(
        pParameters: *const XAUDIO2_FILTER_PARAMETERS,
        OperationSet: UINT32,
    ) -> HRESULT,
    fn GetFilterParameters(
        pParameters: *mut XAUDIO2_FILTER_PARAMETERS,
    ) -> (),
    fn SetOutputFilterParameters(
        pDestinationVoice: *mut IXAudio2Voice,
        pParameters: *const XAUDIO2_FILTER_PARAMETERS,
        OperationSet: UINT32,
    ) -> HRESULT,
    fn GetOutputFilterParameters(
        pDestinationVoice: *mut IXAudio2Voice,
        pParameters: *mut XAUDIO2_FILTER_PARAMETERS,
    ) -> (),
    fn SetVolume(
        Volume: c_float,
        OperationSet: UINT32,
    ) -> HRESULT,
    fn GetVolume(
        pVolume: *mut c_float,
    ) -> (),
    fn SetChannelVolumes(
        Channels: UINT32,
        pVolumes: *const c_float,
        OperationSet: UINT32,
    ) -> HRESULT,
    fn GetChannelVolumes(
        Channels: UINT32,
        pVolumes: *mut c_float,
    ) -> (),
    fn SetOutputMatrix(
        pDestinationVoice: *mut IXAudio2Voice,
        SourceChannels: UINT32,
        DestinationChannels: UINT32,
        pLevelMatrix: *const c_float,
        OperationSet: UINT32,
    ) -> HRESULT,
    fn GetOutputMatrix(
        pDestinationVoice: *mut IXAudio2Voice,
        SourceChannels: UINT32,
        DestinationChannels: UINT32,
        pLevelMatrix: *mut c_float,
    ) -> (),
    fn DestroyVoice() -> (),
}}
RIDL!{
interface IXAudio2SourceVoice(IXAudio2SourceVoiceVtbl): IXAudio2Voice(IXAudio2VoiceVtbl) {
    fn Start(
        Flags: UINT32,
        OperationSet: UINT32,
    ) -> HRESULT,
    fn Stop(
        Flags: UINT32,
        OperationSet: UINT32,
    ) -> HRESULT,
    fn SubmitSourceBuffer(
        pBuffer: *const XAUDIO2_BUFFER,
        pBufferWMA: *const XAUDIO2_BUFFER_WMA,
    ) -> HRESULT,
    fn FlushSourceBuffers() -> HRESULT,
    fn Discontinuity() -> HRESULT,
    fn ExitLoop(
        OperationSet: UINT32,
    ) -> HRESULT,
    fn GetState(
        pVoiceState: *mut XAUDIO2_VOICE_STATE,
        Flags: UINT32,
    ) -> (),
    fn SetFrequencyRatio(
        Ratio: c_float,
        OperationSet: UINT32,
    ) -> HRESULT,
    fn GetFrequencyRatio(
        pRatio: *mut c_float,
    ) -> (),
    fn SetSourceSampleRate(
        NewSourceSampleRate: UINT32,
    ) -> HRESULT,
}}
RIDL!{
interface IXAudio2SubmixVoice(IXAudio2SubmixVoiceVtbl): IXAudio2Voice(IXAudio2VoiceVtbl) {
}}
RIDL!{
interface IXAudio2MasteringVoice(IXAudio2MasteringVoiceVtbl): IXAudio2Voice(IXAudio2VoiceVtbl) {
    fn GetChannelMask(
        pChannelmask: *mut DWORD,
    ) -> HRESULT,
}}
RIDL!{
interface IXAudio2EngineCallback(IXAudio2EngineCallbackVtbl) {
    fn OnProcessingPassStart() -> (),
    fn OnProcessingPassEnd() -> (),
    fn OnCriticalError(
        Error: HRESULT,
    ) -> (),
}}
RIDL!{
interface IXAudio2VoiceCallback(IXAudio2VoiceCallbackVtbl) {
    fn OnVoiceProcessingPassStart(
        BytesRequired: UINT32,
    ) -> (),
    fn OnVoiceProcessingPassEnd() -> (),
    fn OnStreamEnd() -> (),
    fn OnBufferStart(
        pBufferContext: *mut c_void,
    ) -> (),
    fn OnBufferEnd(
        pBufferContext: *mut c_void,
    ) -> (),
    fn OnLoopEnd(
        pBufferContext: *mut c_void,
    ) -> (),
    fn OnVoiceError(
        pBufferContext: *mut c_void,
        Error: HRESULT,
    ) -> (),
}}
#[inline]
#[cfg(feature = "std")]
pub fn XAudio2DecibelsToAmplitudeRatio(Decibels: f32) -> f32 {
    10.0f32.powf(Decibels / 20.0f32)
}
#[inline]
#[cfg(feature = "std")]
pub fn XAudio2AmplitudeRatioToDecibels(Volume: f32) -> f32 {
    if Volume == 0.0f32 {
        -3.402823466e+38f32
    } else {
        20.0f32 * Volume.log10()
    }
}
#[inline]
#[cfg(feature = "std")]
pub fn XAudio2SemitonesToFrequencyRatio(Semitones: f32) -> f32 {
    2.0f32.powf(Semitones / 12.0f32)
}
#[inline]
#[cfg(feature = "std")]
pub fn XAudio2FrequencyRatioToSemitones(FrequencyRatio: f32) -> f32 {
    39.86313713864835f32 * FrequencyRatio.log10()
}
#[inline]
#[cfg(feature = "std")]
pub fn XAudio2CutoffFrequencyToRadians(CutoffFrequency: f32, SampleRate: u32) -> f32 {
    use _core::f32;
    if ((CutoffFrequency * 6.0f32) as u32) >= SampleRate {
        XAUDIO2_MAX_FILTER_FREQUENCY
    } else {
        2.0f32 * (f32::consts::PI * CutoffFrequency / (SampleRate as f32)).sin()
    }
}
#[inline]
#[cfg(feature = "std")]
pub fn XAudio2RadiansToCutoffFrequency(Radians: f32, SampleRate: f32) -> f32 {
    use _core::f32;
    SampleRate * (Radians / 2.0f32).asin() / f32::consts::PI
}
#[inline]
#[cfg(feature = "std")]
pub fn XAudio2CutoffFrequencyToOnePoleCoefficient(CutoffFrequency: f32, SampleRate: u32) -> f32 {
    if (CutoffFrequency as u32) >= SampleRate {
        XAUDIO2_MAX_FILTER_FREQUENCY
    } else {
        1.0f32 - (1.0f32 - 2.0f32 * CutoffFrequency / (SampleRate as f32)).powf(2.0f32)
    }
}
extern "system" {
    pub fn XAudio2Create(
        ppXAudio2: *mut *mut IXAudio2,
        Flags: UINT32,
        XAudio2Processor: XAUDIO2_PROCESSOR,
    ) -> HRESULT;
}

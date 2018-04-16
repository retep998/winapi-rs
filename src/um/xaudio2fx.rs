// Copyright © 2018 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use ctypes::c_float;
use shared::basetsd::{INT32, UINT32};
use shared::minwindef::{BOOL, BYTE, FALSE};
use shared::winerror::HRESULT;
use um::unknwnbase::IUnknown;
extern "system" {
    pub fn CreateAudioVolumeMeter(
        ppApo: *mut *mut IUnknown,
    ) -> HRESULT;
    pub fn CreateAudioReverb(
        ppApo: *mut *mut IUnknown,
    ) -> HRESULT;
}
#[inline]
pub unsafe fn XAudio2CreateVolumeMeter(ppApo: *mut *mut IUnknown, _flags: UINT32) -> HRESULT {
    CreateAudioVolumeMeter(ppApo)
}
#[inline]
pub unsafe fn XAudio2CreateReverb(ppApo: *mut *mut IUnknown, _flags: UINT32) -> HRESULT {
    CreateAudioReverb(ppApo)
}
STRUCT!{struct XAUDIO2FX_VOLUMEMETER_LEVELS {
    pPeakLevels: *mut c_float,
    pRMSLevels: *mut c_float,
    ChannelCount: UINT32,
}}
pub const XAUDIO2FX_REVERB_MIN_FRAMERATE: u32 = 20000;
pub const XAUDIO2FX_REVERB_MAX_FRAMERATE: u32 = 48000;
STRUCT!{struct XAUDIO2FX_REVERB_PARAMETERS {
    WetDryMix: c_float,
    ReflectionsDelay: UINT32,
    ReverbDelay: BYTE,
    RearDelay: BYTE,
    SideDelay: BYTE,
    PositionLeft: BYTE,
    PositionRight: BYTE,
    PositionMatrixLeft: BYTE,
    PositionMatrixRight: BYTE,
    EarlyDiffusion: BYTE,
    LateDiffusion: BYTE,
    LowEQGain: BYTE,
    LowEQCutoff: BYTE,
    HighEQGain: BYTE,
    HighEQCutoff: BYTE,
    RoomFilterFreq: c_float,
    RoomFilterMain: c_float,
    RoomFilterHF: c_float,
    ReflectionsGain: c_float,
    ReverbGain: c_float,
    DecayTime: c_float,
    Density: c_float,
    RoomSize: c_float,
    DisableLateField: BOOL,
}}
pub const XAUDIO2FX_REVERB_MIN_WET_DRY_MIX: f32 = 0.0f32;
pub const XAUDIO2FX_REVERB_MIN_REFLECTIONS_DELAY: u32 = 0;
pub const XAUDIO2FX_REVERB_MIN_REVERB_DELAY: u8 = 0;
pub const XAUDIO2FX_REVERB_MIN_REAR_DELAY: u8 = 0;
pub const XAUDIO2FX_REVERB_MIN_7POINT1_SIDE_DELAY: u8 = 0;
pub const XAUDIO2FX_REVERB_MIN_7POINT1_REAR_DELAY: u8 = 0;
pub const XAUDIO2FX_REVERB_MIN_POSITION: u8 = 0;
pub const XAUDIO2FX_REVERB_MIN_DIFFUSION: u8 = 0;
pub const XAUDIO2FX_REVERB_MIN_LOW_EQ_GAIN: u8 = 0;
pub const XAUDIO2FX_REVERB_MIN_LOW_EQ_CUTOFF: u8 = 0;
pub const XAUDIO2FX_REVERB_MIN_HIGH_EQ_GAIN: u8 = 0;
pub const XAUDIO2FX_REVERB_MIN_HIGH_EQ_CUTOFF: u8 = 0;
pub const XAUDIO2FX_REVERB_MIN_ROOM_FILTER_FREQ: f32 = 20.0f32;
pub const XAUDIO2FX_REVERB_MIN_ROOM_FILTER_MAIN: f32 = -100.0f32;
pub const XAUDIO2FX_REVERB_MIN_ROOM_FILTER_HF: f32 = -100.0f32;
pub const XAUDIO2FX_REVERB_MIN_REFLECTIONS_GAIN: f32 = -100.0f32;
pub const XAUDIO2FX_REVERB_MIN_REVERB_GAIN: f32 = -100.0f32;
pub const XAUDIO2FX_REVERB_MIN_DECAY_TIME: f32 = 0.1f32;
pub const XAUDIO2FX_REVERB_MIN_DENSITY: f32 = 0.0f32;
pub const XAUDIO2FX_REVERB_MIN_ROOM_SIZE: f32 = 0.0f32;
pub const XAUDIO2FX_REVERB_MAX_WET_DRY_MIX: f32 = 100.0f32;
pub const XAUDIO2FX_REVERB_MAX_REFLECTIONS_DELAY: u32 = 300;
pub const XAUDIO2FX_REVERB_MAX_REVERB_DELAY: u8 = 85;
pub const XAUDIO2FX_REVERB_MAX_REAR_DELAY: u8 = 5;
pub const XAUDIO2FX_REVERB_MAX_7POINT1_SIDE_DELAY: u8 = 5;
pub const XAUDIO2FX_REVERB_MAX_7POINT1_REAR_DELAY: u8 = 20;
pub const XAUDIO2FX_REVERB_MAX_POSITION: u8 = 30;
pub const XAUDIO2FX_REVERB_MAX_DIFFUSION: u8 = 15;
pub const XAUDIO2FX_REVERB_MAX_LOW_EQ_GAIN: u8 = 12;
pub const XAUDIO2FX_REVERB_MAX_LOW_EQ_CUTOFF: u8 = 9;
pub const XAUDIO2FX_REVERB_MAX_HIGH_EQ_GAIN: u8 = 8;
pub const XAUDIO2FX_REVERB_MAX_HIGH_EQ_CUTOFF: u8 = 14;
pub const XAUDIO2FX_REVERB_MAX_ROOM_FILTER_FREQ: f32 = 20000.0f32;
pub const XAUDIO2FX_REVERB_MAX_ROOM_FILTER_MAIN: f32 = 0.0f32;
pub const XAUDIO2FX_REVERB_MAX_ROOM_FILTER_HF: f32 = 0.0f32;
pub const XAUDIO2FX_REVERB_MAX_REFLECTIONS_GAIN: f32 = 20.0f32;
pub const XAUDIO2FX_REVERB_MAX_REVERB_GAIN: f32 = 20.0f32;
pub const XAUDIO2FX_REVERB_MAX_DENSITY: f32 = 100.0f32;
pub const XAUDIO2FX_REVERB_MAX_ROOM_SIZE: f32 = 100.0f32;
pub const XAUDIO2FX_REVERB_DEFAULT_WET_DRY_MIX: f32 = 100.0f32;
pub const XAUDIO2FX_REVERB_DEFAULT_REFLECTIONS_DELAY: u32 = 5;
pub const XAUDIO2FX_REVERB_DEFAULT_REVERB_DELAY: u8 = 5;
pub const XAUDIO2FX_REVERB_DEFAULT_REAR_DELAY: u8 = 5;
pub const XAUDIO2FX_REVERB_DEFAULT_7POINT1_SIDE_DELAY: u8 = 5;
pub const XAUDIO2FX_REVERB_DEFAULT_7POINT1_REAR_DELAY: u8 = 20;
pub const XAUDIO2FX_REVERB_DEFAULT_POSITION: u8 = 6;
pub const XAUDIO2FX_REVERB_DEFAULT_POSITION_MATRIX: u8 = 27;
pub const XAUDIO2FX_REVERB_DEFAULT_EARLY_DIFFUSION: u8 = 8;
pub const XAUDIO2FX_REVERB_DEFAULT_LATE_DIFFUSION: u8 = 8;
pub const XAUDIO2FX_REVERB_DEFAULT_LOW_EQ_GAIN: u8 = 8;
pub const XAUDIO2FX_REVERB_DEFAULT_LOW_EQ_CUTOFF: u8 = 4;
pub const XAUDIO2FX_REVERB_DEFAULT_HIGH_EQ_GAIN: u8 = 8;
pub const XAUDIO2FX_REVERB_DEFAULT_HIGH_EQ_CUTOFF: u8 = 4;
pub const XAUDIO2FX_REVERB_DEFAULT_ROOM_FILTER_FREQ: f32 = 5000.0f32;
pub const XAUDIO2FX_REVERB_DEFAULT_ROOM_FILTER_MAIN: f32 = 0.0f32;
pub const XAUDIO2FX_REVERB_DEFAULT_ROOM_FILTER_HF: f32 = 0.0f32;
pub const XAUDIO2FX_REVERB_DEFAULT_REFLECTIONS_GAIN: f32 = 0.0f32;
pub const XAUDIO2FX_REVERB_DEFAULT_REVERB_GAIN: f32 = 0.0f32;
pub const XAUDIO2FX_REVERB_DEFAULT_DECAY_TIME: f32 = 1.0f32;
pub const XAUDIO2FX_REVERB_DEFAULT_DENSITY: f32 = 100.0f32;
pub const XAUDIO2FX_REVERB_DEFAULT_ROOM_SIZE: f32 = 100.0f32;
pub const XAUDIO2FX_REVERB_DEFAULT_DISABLE_LATE_FIELD: BOOL = FALSE;
STRUCT!{struct XAUDIO2FX_REVERB_I3DL2_PARAMETERS {
    WetDryMix: c_float,
    Room: INT32,
    RoomHF: INT32,
    RoomRolloffFactor: c_float,
    DecayTime: c_float,
    DecayHFRatio: c_float,
    Reflections: INT32,
    ReflectionsDelay: c_float,
    Reverb: INT32,
    ReverbDelay: c_float,
    Diffusion: c_float,
    Density: c_float,
    HFReference: c_float,
}}
#[inline]
#[cfg(feature = "std")]
pub fn ReverbConvertI3DL2ToNative (
    pI3DL2: &XAUDIO2FX_REVERB_I3DL2_PARAMETERS,
    pNative: &mut XAUDIO2FX_REVERB_PARAMETERS,
    sevenDotOneReverb: bool,
) {
    if sevenDotOneReverb {
        pNative.RearDelay = XAUDIO2FX_REVERB_DEFAULT_7POINT1_REAR_DELAY;
    } else {
        pNative.RearDelay = XAUDIO2FX_REVERB_DEFAULT_REAR_DELAY;
    }
    pNative.SideDelay = XAUDIO2FX_REVERB_DEFAULT_7POINT1_SIDE_DELAY;
    pNative.PositionLeft = XAUDIO2FX_REVERB_DEFAULT_POSITION;
    pNative.PositionRight = XAUDIO2FX_REVERB_DEFAULT_POSITION;
    pNative.PositionMatrixLeft = XAUDIO2FX_REVERB_DEFAULT_POSITION_MATRIX;
    pNative.PositionMatrixRight = XAUDIO2FX_REVERB_DEFAULT_POSITION_MATRIX;
    pNative.RoomSize = XAUDIO2FX_REVERB_DEFAULT_ROOM_SIZE;
    pNative.LowEQCutoff = 4;
    pNative.HighEQCutoff = 6;
    pNative.RoomFilterMain = (pI3DL2.Room as f32) / 100.0f32;
    pNative.RoomFilterHF = (pI3DL2.RoomHF as f32) / 100.0f32;
    let mut index: i32 = (-4.0f64 * (pI3DL2.DecayHFRatio as f64).log10()) as i32;
    if index < -8 {
        index = -8;
    }
    if pI3DL2.DecayHFRatio >= 1.0f32 {
        pNative.LowEQGain = if index < 0 { index + 8 } else { 8 } as u8;
        pNative.HighEQGain = 8;
        pNative.DecayTime = pI3DL2.DecayTime * pI3DL2.DecayHFRatio;
    } else {
        pNative.LowEQGain = 8;
        pNative.HighEQGain = if index < 0 { index + 8 } else  { 8 } as u8;
        pNative.DecayTime = pI3DL2.DecayTime;
    }
    let mut reflectionsDelay: f32 = pI3DL2.ReflectionsDelay * 1000.0f32;
    if reflectionsDelay >= XAUDIO2FX_REVERB_MAX_REFLECTIONS_DELAY as f32 {
        reflectionsDelay = (XAUDIO2FX_REVERB_MAX_REFLECTIONS_DELAY - 1) as f32;
    }
    else if reflectionsDelay <= 1.0f32 {
        reflectionsDelay = 1.0f32;
    }
    pNative.ReflectionsDelay = reflectionsDelay as u32;
    let mut reverbDelay: f32 = pI3DL2.ReverbDelay * 1000.0f32;
    if reverbDelay >= XAUDIO2FX_REVERB_MAX_REVERB_DELAY as f32 {
        reverbDelay = (XAUDIO2FX_REVERB_MAX_REVERB_DELAY - 1) as f32;
    }
    pNative.ReverbDelay = reverbDelay as u8;
    pNative.ReflectionsGain = pI3DL2.Reflections as f32 / 100.0f32;
    pNative.ReverbGain = pI3DL2.Reverb as f32 / 100.0f32;
    pNative.EarlyDiffusion = (15.0f32 * pI3DL2.Diffusion / 100.0f32) as u8;
    pNative.LateDiffusion = pNative.EarlyDiffusion;
    pNative.Density = pI3DL2.Density;
    pNative.RoomFilterFreq = pI3DL2.HFReference;
    pNative.WetDryMix = pI3DL2.WetDryMix;
    pNative.DisableLateField = FALSE;
}
pub const XAUDIO2FX_I3DL2_PRESET_DEFAULT: XAUDIO2FX_REVERB_I3DL2_PARAMETERS
    = XAUDIO2FX_REVERB_I3DL2_PARAMETERS {
        WetDryMix: 100f32,
        Room: -10000,
        RoomHF: 0,
        RoomRolloffFactor: 0.0f32,
        DecayTime: 1.00f32,
        DecayHFRatio: 0.50f32,
        Reflections: -10000,
        ReflectionsDelay: 0.020f32,
        Reverb: -10000,
        ReverbDelay: 0.040f32,
        Diffusion: 100.0f32,
        Density: 100.0f32,
        HFReference: 5000.0f32,
    };
pub const XAUDIO2FX_I3DL2_PRESET_GENERIC: XAUDIO2FX_REVERB_I3DL2_PARAMETERS
    = XAUDIO2FX_REVERB_I3DL2_PARAMETERS {
        WetDryMix: 100f32,
        Room: -1000,
        RoomHF: -100,
        RoomRolloffFactor: 0.0f32,
        DecayTime: 1.49f32,
        DecayHFRatio: 0.83f32,
        Reflections: -2602,
        ReflectionsDelay: 0.007f32,
        Reverb: 200,
        ReverbDelay: 0.011f32,
        Diffusion: 100.0f32,
        Density: 100.0f32,
        HFReference: 5000.0f32,
    };
pub const XAUDIO2FX_I3DL2_PRESET_PADDEDCELL: XAUDIO2FX_REVERB_I3DL2_PARAMETERS
    = XAUDIO2FX_REVERB_I3DL2_PARAMETERS {
        WetDryMix: 100f32,
        Room: -1000,
        RoomHF: -6000,
        RoomRolloffFactor: 0.0f32,
        DecayTime: 0.17f32,
        DecayHFRatio: 0.10f32,
        Reflections: -1204,
        ReflectionsDelay: 0.001f32,
        Reverb: 207,
        ReverbDelay: 0.002f32,
        Diffusion: 100.0f32,
        Density: 100.0f32,
        HFReference: 5000.0f32,
    };
pub const XAUDIO2FX_I3DL2_PRESET_ROOM: XAUDIO2FX_REVERB_I3DL2_PARAMETERS
    = XAUDIO2FX_REVERB_I3DL2_PARAMETERS {
        WetDryMix: 100f32,
        Room: -1000,
        RoomHF: -454,
        RoomRolloffFactor: 0.0f32,
        DecayTime: 0.40f32,
        DecayHFRatio: 0.83f32,
        Reflections: -1646,
        ReflectionsDelay: 0.002f32,
        Reverb: 53,
        ReverbDelay: 0.003f32,
        Diffusion: 100.0f32,
        Density: 100.0f32,
        HFReference: 5000.0f32,
    };
pub const XAUDIO2FX_I3DL2_PRESET_BATHROOM: XAUDIO2FX_REVERB_I3DL2_PARAMETERS
    = XAUDIO2FX_REVERB_I3DL2_PARAMETERS {
        WetDryMix: 100f32,
        Room: -1000,
        RoomHF: -1200,
        RoomRolloffFactor: 0.0f32,
        DecayTime: 1.49f32,
        DecayHFRatio: 0.54f32,
        Reflections: -370,
        ReflectionsDelay: 0.007f32,
        Reverb: 1030,
        ReverbDelay: 0.011f32,
        Diffusion: 100.0f32,
        Density: 60.0f32,
        HFReference: 5000.0f32,
    };
pub const XAUDIO2FX_I3DL2_PRESET_LIVINGROOM: XAUDIO2FX_REVERB_I3DL2_PARAMETERS
    = XAUDIO2FX_REVERB_I3DL2_PARAMETERS {
        WetDryMix: 100f32,
        Room: -1000,
        RoomHF: -6000,
        RoomRolloffFactor: 0.0f32,
        DecayTime: 0.50f32,
        DecayHFRatio: 0.10f32,
        Reflections: -1376,
        ReflectionsDelay: 0.003f32,
        Reverb: -1104,
        ReverbDelay: 0.004f32,
        Diffusion: 100.0f32,
        Density: 100.0f32,
        HFReference: 5000.0f32,
    };
pub const XAUDIO2FX_I3DL2_PRESET_STONEROOM: XAUDIO2FX_REVERB_I3DL2_PARAMETERS
    = XAUDIO2FX_REVERB_I3DL2_PARAMETERS {
        WetDryMix: 100f32,
        Room: -1000,
        RoomHF: -300,
        RoomRolloffFactor: 0.0f32,
        DecayTime: 2.31f32,
        DecayHFRatio: 0.64f32,
        Reflections: -711,
        ReflectionsDelay: 0.012f32,
        Reverb: 83,
        ReverbDelay: 0.017f32,
        Diffusion: 100.0f32,
        Density: 100.0f32,
        HFReference: 5000.0f32,
    };
pub const XAUDIO2FX_I3DL2_PRESET_AUDITORIUM: XAUDIO2FX_REVERB_I3DL2_PARAMETERS
    = XAUDIO2FX_REVERB_I3DL2_PARAMETERS {
        WetDryMix: 100f32,
        Room: -1000,
        RoomHF: -476,
        RoomRolloffFactor: 0.0f32,
        DecayTime: 4.32f32,
        DecayHFRatio: 0.59f32,
        Reflections: -789,
        ReflectionsDelay: 0.020f32,
        Reverb: -289,
        ReverbDelay: 0.030f32,
        Diffusion: 100.0f32,
        Density: 100.0f32,
        HFReference: 5000.0f32,
    };
pub const XAUDIO2FX_I3DL2_PRESET_CONCERTHALL: XAUDIO2FX_REVERB_I3DL2_PARAMETERS
    = XAUDIO2FX_REVERB_I3DL2_PARAMETERS {
        WetDryMix: 100f32,
        Room: -1000,
        RoomHF: -500,
        RoomRolloffFactor: 0.0f32,
        DecayTime: 3.92f32,
        DecayHFRatio: 0.70f32,
        Reflections: -1230,
        ReflectionsDelay: 0.020f32,
        Reverb: -2,
        ReverbDelay: 0.029f32,
        Diffusion: 100.0f32,
        Density: 100.0f32,
        HFReference: 5000.0f32,
    };
pub const XAUDIO2FX_I3DL2_PRESET_CAVE: XAUDIO2FX_REVERB_I3DL2_PARAMETERS
    = XAUDIO2FX_REVERB_I3DL2_PARAMETERS {
        WetDryMix: 100f32,
        Room: -1000,
        RoomHF: 0,
        RoomRolloffFactor: 0.0f32,
        DecayTime: 2.91f32,
        DecayHFRatio: 1.30f32,
        Reflections: -602,
        ReflectionsDelay: 0.015f32,
        Reverb: -302,
        ReverbDelay: 0.022f32,
        Diffusion: 100.0f32,
        Density: 100.0f32,
        HFReference: 5000.0f32,
    };
pub const XAUDIO2FX_I3DL2_PRESET_ARENA: XAUDIO2FX_REVERB_I3DL2_PARAMETERS
    = XAUDIO2FX_REVERB_I3DL2_PARAMETERS {
        WetDryMix: 100f32,
        Room: -1000,
        RoomHF: -698,
        RoomRolloffFactor: 0.0f32,
        DecayTime: 7.24f32,
        DecayHFRatio: 0.33f32,
        Reflections: -1166,
        ReflectionsDelay: 0.020f32,
        Reverb: 16,
        ReverbDelay: 0.030f32,
        Diffusion: 100.0f32,
        Density: 100.0f32,
        HFReference: 5000.0f32,
    };
pub const XAUDIO2FX_I3DL2_PRESET_HANGAR: XAUDIO2FX_REVERB_I3DL2_PARAMETERS
    = XAUDIO2FX_REVERB_I3DL2_PARAMETERS {
        WetDryMix: 100f32,
        Room: -1000,
        RoomHF: -1000,
        RoomRolloffFactor: 0.0f32,
        DecayTime: 10.05f32,
        DecayHFRatio: 0.23f32,
        Reflections: -602,
        ReflectionsDelay: 0.020f32,
        Reverb: 198,
        ReverbDelay: 0.030f32,
        Diffusion: 100.0f32,
        Density: 100.0f32,
        HFReference: 5000.0f32,
    };
pub const XAUDIO2FX_I3DL2_PRESET_CARPETEDHALLWAY: XAUDIO2FX_REVERB_I3DL2_PARAMETERS
    = XAUDIO2FX_REVERB_I3DL2_PARAMETERS {
        WetDryMix: 100f32,
        Room: -1000,
        RoomHF: -4000,
        RoomRolloffFactor: 0.0f32,
        DecayTime: 0.30f32,
        DecayHFRatio: 0.10f32,
        Reflections: -1831,
        ReflectionsDelay: 0.002f32,
        Reverb: -1630,
        ReverbDelay: 0.030f32,
        Diffusion: 100.0f32,
        Density: 100.0f32,
        HFReference: 5000.0f32,
    };
pub const XAUDIO2FX_I3DL2_PRESET_HALLWAY: XAUDIO2FX_REVERB_I3DL2_PARAMETERS
    = XAUDIO2FX_REVERB_I3DL2_PARAMETERS {
        WetDryMix: 100f32,
        Room: -1000,
        RoomHF: -300,
        RoomRolloffFactor: 0.0f32,
        DecayTime: 1.49f32,
        DecayHFRatio: 0.59f32,
        Reflections: -1219,
        ReflectionsDelay: 0.007f32,
        Reverb: 441,
        ReverbDelay: 0.011f32,
        Diffusion: 100.0f32,
        Density: 100.0f32,
        HFReference: 5000.0f32,
    };
pub const XAUDIO2FX_I3DL2_PRESET_STONECORRIDOR: XAUDIO2FX_REVERB_I3DL2_PARAMETERS
    = XAUDIO2FX_REVERB_I3DL2_PARAMETERS {
        WetDryMix: 100f32,
        Room: -1000,
        RoomHF: -237,
        RoomRolloffFactor: 0.0f32,
        DecayTime: 2.70f32,
        DecayHFRatio: 0.79f32,
        Reflections: -1214,
        ReflectionsDelay: 0.013f32,
        Reverb: 395,
        ReverbDelay: 0.020f32,
        Diffusion: 100.0f32,
        Density: 100.0f32,
        HFReference: 5000.0f32,
    };
pub const XAUDIO2FX_I3DL2_PRESET_ALLEY: XAUDIO2FX_REVERB_I3DL2_PARAMETERS
    = XAUDIO2FX_REVERB_I3DL2_PARAMETERS {
        WetDryMix: 100f32,
        Room: -1000,
        RoomHF: -270,
        RoomRolloffFactor: 0.0f32,
        DecayTime: 1.49f32,
        DecayHFRatio: 0.86f32,
        Reflections: -1204,
        ReflectionsDelay: 0.007f32,
        Reverb: -4,
        ReverbDelay: 0.011f32,
        Diffusion: 100.0f32,
        Density: 100.0f32,
        HFReference: 5000.0f32,
    };
pub const XAUDIO2FX_I3DL2_PRESET_FOREST: XAUDIO2FX_REVERB_I3DL2_PARAMETERS
    = XAUDIO2FX_REVERB_I3DL2_PARAMETERS {
        WetDryMix: 100f32,
        Room: -1000,
        RoomHF: -3300,
        RoomRolloffFactor: 0.0f32,
        DecayTime: 1.49f32,
        DecayHFRatio: 0.54f32,
        Reflections: -2560,
        ReflectionsDelay: 0.162f32,
        Reverb: -613,
        ReverbDelay: 0.088f32,
        Diffusion: 79.0f32,
        Density: 100.0f32,
        HFReference: 5000.0f32,
    };
pub const XAUDIO2FX_I3DL2_PRESET_CITY: XAUDIO2FX_REVERB_I3DL2_PARAMETERS
    = XAUDIO2FX_REVERB_I3DL2_PARAMETERS {
        WetDryMix: 100f32,
        Room: -1000,
        RoomHF: -800,
        RoomRolloffFactor: 0.0f32,
        DecayTime: 1.49f32,
        DecayHFRatio: 0.67f32,
        Reflections: -2273,
        ReflectionsDelay: 0.007f32,
        Reverb: -2217,
        ReverbDelay: 0.011f32,
        Diffusion: 50.0f32,
        Density: 100.0f32,
        HFReference: 5000.0f32,
    };
pub const XAUDIO2FX_I3DL2_PRESET_MOUNTAINS: XAUDIO2FX_REVERB_I3DL2_PARAMETERS
    = XAUDIO2FX_REVERB_I3DL2_PARAMETERS {
        WetDryMix: 100f32,
        Room: -1000,
        RoomHF: -2500,
        RoomRolloffFactor: 0.0f32,
        DecayTime: 1.49f32,
        DecayHFRatio: 0.21f32,
        Reflections: -2780,
        ReflectionsDelay: 0.300f32,
        Reverb: -2014,
        ReverbDelay: 0.100f32,
        Diffusion: 27.0f32,
        Density: 100.0f32,
        HFReference: 5000.0f32,
    };
pub const XAUDIO2FX_I3DL2_PRESET_QUARRY: XAUDIO2FX_REVERB_I3DL2_PARAMETERS
    = XAUDIO2FX_REVERB_I3DL2_PARAMETERS {
        WetDryMix: 100f32,
        Room: -1000,
        RoomHF: -1000,
        RoomRolloffFactor: 0.0f32,
        DecayTime: 1.49f32,
        DecayHFRatio: 0.83f32,
        Reflections: -10000,
        ReflectionsDelay: 0.061f32,
        Reverb: 500,
        ReverbDelay: 0.025f32,
        Diffusion: 100.0f32,
        Density: 100.0f32,
        HFReference: 5000.0f32,
    };
pub const XAUDIO2FX_I3DL2_PRESET_PLAIN: XAUDIO2FX_REVERB_I3DL2_PARAMETERS
    = XAUDIO2FX_REVERB_I3DL2_PARAMETERS {
        WetDryMix: 100f32,
        Room: -1000,
        RoomHF: -2000,
        RoomRolloffFactor: 0.0f32,
        DecayTime: 1.49f32,
        DecayHFRatio: 0.50f32,
        Reflections: -2466,
        ReflectionsDelay: 0.179f32,
        Reverb: -2514,
        ReverbDelay: 0.100f32,
        Diffusion: 21.0f32,
        Density: 100.0f32,
        HFReference: 5000.0f32,
    };
pub const XAUDIO2FX_I3DL2_PRESET_PARKINGLOT: XAUDIO2FX_REVERB_I3DL2_PARAMETERS
    = XAUDIO2FX_REVERB_I3DL2_PARAMETERS {
        WetDryMix: 100f32,
        Room: -1000,
        RoomHF: 0,
        RoomRolloffFactor: 0.0f32,
        DecayTime: 1.65f32,
        DecayHFRatio: 1.50f32,
        Reflections: -1363,
        ReflectionsDelay: 0.008f32,
        Reverb: -1153,
        ReverbDelay: 0.012f32,
        Diffusion: 100.0f32,
        Density: 100.0f32,
        HFReference: 5000.0f32,
    };
pub const XAUDIO2FX_I3DL2_PRESET_SEWERPIPE: XAUDIO2FX_REVERB_I3DL2_PARAMETERS
    = XAUDIO2FX_REVERB_I3DL2_PARAMETERS {
        WetDryMix: 100f32,
        Room: -1000,
        RoomHF: -1000,
        RoomRolloffFactor: 0.0f32,
        DecayTime: 2.81f32,
        DecayHFRatio: 0.14f32,
        Reflections: 429,
        ReflectionsDelay: 0.014f32,
        Reverb: 648,
        ReverbDelay: 0.021f32,
        Diffusion: 80.0f32,
        Density: 60.0f32,
        HFReference: 5000.0f32,
    };
pub const XAUDIO2FX_I3DL2_PRESET_UNDERWATER: XAUDIO2FX_REVERB_I3DL2_PARAMETERS
    = XAUDIO2FX_REVERB_I3DL2_PARAMETERS {
        WetDryMix: 100f32,
        Room: -1000,
        RoomHF: -4000,
        RoomRolloffFactor: 0.0f32,
        DecayTime: 1.49f32,
        DecayHFRatio: 0.10f32,
        Reflections: -449,
        ReflectionsDelay: 0.007f32,
        Reverb: 1700,
        ReverbDelay: 0.011f32,
        Diffusion: 100.0f32,
        Density: 100.0f32,
        HFReference: 5000.0f32,
    };
pub const XAUDIO2FX_I3DL2_PRESET_SMALLROOM: XAUDIO2FX_REVERB_I3DL2_PARAMETERS
    = XAUDIO2FX_REVERB_I3DL2_PARAMETERS {
        WetDryMix: 100f32,
        Room: -1000,
        RoomHF: -600,
        RoomRolloffFactor: 0.0f32,
        DecayTime: 1.10f32,
        DecayHFRatio: 0.83f32,
        Reflections: -400,
        ReflectionsDelay: 0.005f32,
        Reverb: 500,
        ReverbDelay: 0.010f32,
        Diffusion: 100.0f32,
        Density: 100.0f32,
        HFReference: 5000.0f32,
    };
pub const XAUDIO2FX_I3DL2_PRESET_MEDIUMROOM: XAUDIO2FX_REVERB_I3DL2_PARAMETERS
    = XAUDIO2FX_REVERB_I3DL2_PARAMETERS {
        WetDryMix: 100f32,
        Room: -1000,
        RoomHF: -600,
        RoomRolloffFactor: 0.0f32,
        DecayTime: 1.30f32,
        DecayHFRatio: 0.83f32,
        Reflections: -1000,
        ReflectionsDelay: 0.010f32,
        Reverb: -200,
        ReverbDelay: 0.020f32,
        Diffusion: 100.0f32,
        Density: 100.0f32,
        HFReference: 5000.0f32,
    };
pub const XAUDIO2FX_I3DL2_PRESET_LARGEROOM: XAUDIO2FX_REVERB_I3DL2_PARAMETERS
    = XAUDIO2FX_REVERB_I3DL2_PARAMETERS {
        WetDryMix: 100f32,
        Room: -1000,
        RoomHF: -600,
        RoomRolloffFactor: 0.0f32,
        DecayTime: 1.50f32,
        DecayHFRatio: 0.83f32,
        Reflections: -1600,
        ReflectionsDelay: 0.020f32,
        Reverb: -1000,
        ReverbDelay: 0.040f32,
        Diffusion: 100.0f32,
        Density: 100.0f32,
        HFReference: 5000.0f32,
    };
pub const XAUDIO2FX_I3DL2_PRESET_MEDIUMHALL: XAUDIO2FX_REVERB_I3DL2_PARAMETERS
    = XAUDIO2FX_REVERB_I3DL2_PARAMETERS {
        WetDryMix: 100f32,
        Room: -1000,
        RoomHF: -600,
        RoomRolloffFactor: 0.0f32,
        DecayTime: 1.80f32,
        DecayHFRatio: 0.70f32,
        Reflections: -1300,
        ReflectionsDelay: 0.015f32,
        Reverb: -800,
        ReverbDelay: 0.030f32,
        Diffusion: 100.0f32,
        Density: 100.0f32,
        HFReference: 5000.0f32,
    };
pub const XAUDIO2FX_I3DL2_PRESET_LARGEHALL: XAUDIO2FX_REVERB_I3DL2_PARAMETERS
    = XAUDIO2FX_REVERB_I3DL2_PARAMETERS {
        WetDryMix: 100f32,
        Room: -1000,
        RoomHF: -600,
        RoomRolloffFactor: 0.0f32,
        DecayTime: 1.80f32,
        DecayHFRatio: 0.70f32,
        Reflections: -2000,
        ReflectionsDelay: 0.030f32,
        Reverb: -1400,
        ReverbDelay: 0.060f32,
        Diffusion: 100.0f32,
        Density: 100.0f32,
        HFReference: 5000.0f32,
    };
pub const XAUDIO2FX_I3DL2_PRESET_PLATE: XAUDIO2FX_REVERB_I3DL2_PARAMETERS
    = XAUDIO2FX_REVERB_I3DL2_PARAMETERS {
        WetDryMix: 100f32,
        Room: -1000,
        RoomHF: -200,
        RoomRolloffFactor: 0.0f32,
        DecayTime: 1.30f32,
        DecayHFRatio: 0.90f32,
        Reflections: 0,
        ReflectionsDelay: 0.002f32,
        Reverb: 0,
        ReverbDelay: 0.010f32,
        Diffusion: 100.0f32,
        Density: 75.0f32,
        HFReference: 5000.0f32,
    };

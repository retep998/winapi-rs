// Copyright © 2015, Jordan Miner
// Licensed under the MIT License <LICENSE.md>
#![cfg(windows)]
extern crate winmm;
use winmm::*;
#[inline(never)] fn bb<T>(_: T) {}
#[test]
fn functions() {
    bb(PlaySoundA);
    bb(PlaySoundW);
    bb(sndPlaySoundA);
    bb(sndPlaySoundW);
    bb(timeBeginPeriod);
    bb(timeEndPeriod);
    bb(timeGetDevCaps);
    bb(timeGetTime);
    bb(waveInAddBuffer);
    bb(waveInClose);
    bb(waveInGetDevCapsW);
    bb(waveInGetErrorTextW);
    bb(waveInGetNumDevs);
    bb(waveInGetPosition);
    bb(waveInMessage);
    bb(waveInOpen);
    bb(waveInPrepareHeader);
    bb(waveInReset);
    bb(waveInStart);
    bb(waveInStop);
    bb(waveInUnprepareHeader);
    bb(waveOutBreakLoop);
    bb(waveOutClose);
    bb(waveOutGetDevCapsW);
    bb(waveOutGetErrorTextW);
    bb(waveOutGetNumDevs);
    bb(waveOutGetPitch);
    bb(waveOutGetPlaybackRate);
    bb(waveOutGetPosition);
    bb(waveOutGetVolume);
    bb(waveOutMessage);
    bb(waveOutOpen);
    bb(waveOutPause);
    bb(waveOutPrepareHeader);
    bb(waveOutReset);
    bb(waveOutRestart);
    bb(waveOutSetPitch);
    bb(waveOutSetPlaybackRate);
    bb(waveOutSetVolume);
    bb(waveOutUnprepareHeader);
    bb(waveOutWrite);
}

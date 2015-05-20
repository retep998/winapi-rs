// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to dwmapi.
#![cfg(windows)]
extern crate winapi;
use winapi::*;
extern "system" {
    // pub fn DwmAttachMilContent();
    // pub fn DwmDefWindowProc();
    // pub fn DwmDetachMilContent();
    pub fn DwmEnableBlurBehindWindow(hWnd: HWND, pBlurBehind: *const DWM_BLURBEHIND) -> HRESULT;
    // pub fn DwmEnableComposition();
    // pub fn DwmEnableMMCSS();
    // pub fn DwmExtendFrameIntoClientArea();
    // pub fn DwmFlush();
    // pub fn DwmGetColorizationColor();
    // pub fn DwmGetCompositionTimingInfo();
    // pub fn DwmGetGraphicsStreamClient();
    // pub fn DwmGetGraphicsStreamTransformHint();
    // pub fn DwmGetTransportAttributes();
    // pub fn DwmGetWindowAttribute();
    // pub fn DwmInvalidateIconicBitmaps();
    // pub fn DwmIsCompositionEnabled();
    // pub fn DwmModifyPreviousDxFrameDuration();
    // pub fn DwmQueryThumbnailSourceSize();
    // pub fn DwmRegisterThumbnail();
    // pub fn DwmRenderGesture();
    // pub fn DwmSetDxFrameDuration();
    // pub fn DwmSetIconicLivePreviewBitmap();
    // pub fn DwmSetIconicThumbnail();
    // pub fn DwmSetPresentParameters();
    // pub fn DwmSetWindowAttribute();
    // pub fn DwmShowContact();
    // pub fn DwmTetherContact();
    // pub fn DwmTransitionOwnedWindow();
    // pub fn DwmUnregisterThumbnail();
    // pub fn DwmUpdateThumbnailProperties();
}

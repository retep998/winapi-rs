// Copyright © 2015-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Procedure declarations, constant definitions, and macros for the NLS component.
use shared::basetsd::UINT32;
use shared::minwindef::{BOOL, DWORD, HRGN, LPCVOID, LPVOID, UINT};
use shared::windef::HWND;
use um::winnt::{HRESULT, ULONGLONG};
pub type DWM_FRAME_COUNT = ULONGLONG;
pub type QPC_TIME = ULONGLONG;
STRUCT!{#[repr(packed)] struct DWM_BLURBEHIND {
    dwFlags: DWORD,
    fEnable: BOOL,
    hRgnBlur: HRGN,
    fTransitionOnMaximized: BOOL,
}}
STRUCT!{struct UNSIGNED_RATIO {
    uiNumerator: UINT32,
    uiDenominator: UINT32,
}}
STRUCT!{struct DWM_TIMING_INFO {
    cbSize: UINT32,
    rateRefresh: UNSIGNED_RATIO,
    qpcRefreshPeriod: QPC_TIME,
    rateCompose: UNSIGNED_RATIO,
    qpcVBlank: QPC_TIME,
    cRefresh: DWM_FRAME_COUNT,
    cDXRefresh: UINT,
    qpcCompose: QPC_TIME,
    cFrame: DWM_FRAME_COUNT,
    cDXPresent: UINT,
    cRefreshFrame: DWM_FRAME_COUNT,
    cFrameSubmitted: DWM_FRAME_COUNT,
    cDXPresentSubmitted: UINT,
    cFrameConfirmed: DWM_FRAME_COUNT,
    cDXPresentConfirmed: UINT,
    cRefreshConfirmed: DWM_FRAME_COUNT,
    cDXRefreshConfirmed: UINT,
    cFramesLate: DWM_FRAME_COUNT,
    cFramesOutstanding: UINT,
    cFrameDisplayed: DWM_FRAME_COUNT,
    qpcFrameDisplayed: QPC_TIME,
    cRefreshFrameDisplayed: DWM_FRAME_COUNT,
    cFrameComplete: DWM_FRAME_COUNT,
    qpcFrameComplete: QPC_TIME,
    cFramePending: DWM_FRAME_COUNT,
    qpcFramePending: QPC_TIME,
    cFramesDisplayed: DWM_FRAME_COUNT,
    cFramesComplete: DWM_FRAME_COUNT,
    cFramesPending: DWM_FRAME_COUNT,
    cFramesAvailable: DWM_FRAME_COUNT,
    cFramesDropped: DWM_FRAME_COUNT,
    cFramesMissed: DWM_FRAME_COUNT,
    cRefreshNextDisplayed: DWM_FRAME_COUNT,
    cRefreshNextPresented: DWM_FRAME_COUNT,
    cRefreshesDisplayed: DWM_FRAME_COUNT,
    cRefreshesPresented: DWM_FRAME_COUNT,
    cRefreshStarted: DWM_FRAME_COUNT,
    cPixelsReceived: ULONGLONG,
    cPixelsDrawn: ULONGLONG,
    cBuffersEmpty: DWM_FRAME_COUNT,
}}
pub const DWMWA_NCRENDERING_ENABLED: DWORD = 1;
pub const DWMWA_NCRENDERING_POLICY: DWORD = 2;
pub const DWMWA_TRANSITIONS_FORCEDISABLED: DWORD = 2;
pub const DWMWA_ALLOW_NCPAINT: DWORD = 4;
pub const DWMWA_CAPTION_BUTTON_BOUNDS: DWORD = 5;
pub const DWMWA_NONCLIENT_RTL_LAYOUT: DWORD = 6;
pub const DWMWA_FORCE_ICONIC_REPRESENTATION: DWORD = 7;
pub const DWMWA_FLIP3D_POLICY: DWORD = 8;
pub const DWMWA_EXTENDED_FRAME_BOUNDS: DWORD = 9;
pub const DWMWA_HAS_ICONIC_BITMAP: DWORD = 10;
pub const DWMWA_DISALLOW_PEEK: DWORD = 11;
pub const DWMWA_EXCLUDED_FROM_PEEK: DWORD = 12;
pub const DWMWA_CLOAK: DWORD = 13;
pub const DWMWA_CLOAKED: DWORD = 14;
pub const DWMWA_FREEZE_REPRESENTATION: DWORD = 15;
pub const DWMWA_LAST: DWORD = 16;
extern "system" {
    // pub fn DwmDefWindowProc();
    pub fn DwmEnableBlurBehindWindow(
        hWnd: HWND,
        pBlurBehind: *const DWM_BLURBEHIND,
    ) -> HRESULT;
    // pub fn DwmEnableComposition();
    // pub fn DwmEnableMMCSS();
    // pub fn DwmExtendFrameIntoClientArea();
    // pub fn DwmGetColorizationColor();
    pub fn DwmGetCompositionTimingInfo(
        hWnd: HWND,
        pTimingInfo: *mut DWM_TIMING_INFO,
    ) -> HRESULT;
    pub fn DwmGetWindowAttribute(
        hWnd: HWND,
        dwAttribute: DWORD,
        pvAttribute: LPVOID,
        cbAttribute: DWORD,
    ) -> HRESULT;
    // pub fn DwmIsCompositionEnabled();
    // pub fn DwmModifyPreviousDxFrameDuration();
    // pub fn DwmQueryThumbnailSourceSize();
    // pub fn DwmRegisterThumbnail();
    // pub fn DwmSetDxFrameDuration();
    // pub fn DwmSetPresentParameters();
    pub fn DwmSetWindowAttribute(
        hWnd: HWND,
        dwAttribute: DWORD,
        pvAttribute: LPCVOID,
        cbAttribute: DWORD,
    ) -> HRESULT;
    // pub fn DwmUnregisterThumbnail();
    // pub fn DwmUpdateThumbnailProperties();
    // pub fn DwmSetIconicThumbnail();
    // pub fn DwmSetIconicLivePreviewBitmap();
    // pub fn DwmInvalidateIconicBitmaps();
    // pub fn DwmAttachMilContent();
    // pub fn DwmDetachMilContent();
    pub fn DwmFlush();
    // pub fn DwmGetGraphicsStreamTransformHint();
    // pub fn DwmGetGraphicsStreamClient();
    // pub fn DwmGetTransportAttributes();
    // pub fn DwmTransitionOwnedWindow();
    // pub fn DwmRenderGesture();
    // pub fn DwmTetherContact();
    // pub fn DwmShowContact();
}

// Copyright © 2018 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use _core::ops;
use shared::basetsd::{UINT32, ULONG_PTR};
use shared::minwindef::{BOOL, INT};
use um::gdiplustypes::Status;
use um::winnt::CHAR;
ENUM!{enum DebugEventLevel {
    DebugEventLevelFatal,
    DebugEventLevelWarning,
}}
FN!{stdcall DebugEventProc(
    level: DebugEventLevel,
    message: *mut CHAR,
) -> ()}
FN!{stdcall NotificationHookProc(
    token: *mut ULONG_PTR,
) -> Status}
FN!{stdcall NotificationUnhookProc(
    token: ULONG_PTR,
) -> ()}
STRUCT!{struct GdiplusStartupInput {
    GdiplusVersion: UINT32,
    DebugEventCallback: DebugEventProc,
    SuppressBackgroundThread: BOOL,
    SuppressExternalCodecs: BOOL,
}}
impl GdiplusStartupInput {
    pub fn new(
        debugEventCallback: DebugEventProc,
        suppressBackgroundThread: BOOL,
        suppressExternalCodecs: BOOL,
    ) -> Self {
        GdiplusStartupInput {
            GdiplusVersion: 1,
            DebugEventCallback: debugEventCallback,
            SuppressBackgroundThread: suppressBackgroundThread,
            SuppressExternalCodecs: suppressExternalCodecs,
        }
    }
}
STRUCT!{struct GdiplusStartupInputEx {
    parent: GdiplusStartupInput,
    StartupParameters: INT,
}}
impl GdiplusStartupInputEx {
    pub fn new(
        startupParameters: INT,
        debugEventCallback: DebugEventProc,
        suppressBackgroundThread: BOOL,
        suppressExternalCodecs: BOOL,
    ) -> Self {
        GdiplusStartupInputEx {
            parent: GdiplusStartupInput {
                GdiplusVersion: 2,
                DebugEventCallback: debugEventCallback,
                SuppressBackgroundThread: suppressBackgroundThread,
                SuppressExternalCodecs: suppressExternalCodecs,
            },
            StartupParameters: startupParameters,
        }
    }
}
impl ops::Deref for GdiplusStartupInputEx {
    type Target = GdiplusStartupInput;
    fn deref(&self) -> &Self::Target {
       &self.parent
    }
}
ENUM!{enum GdiplusStartupParams {
    GdiplusStartupDefault = 0,
    GdiplusStartupNoSetRound = 1,
    GdiplusStartupSetPSValue = 2,
    GdiplusStartupTransparencyMask = 0xFF000000,
}}
STRUCT!{struct GdiplusStartupOutput {
    NotificationHook: NotificationHookProc,
    NotificationUnhook: NotificationUnhookProc,
}}
extern "system" {
    pub fn GdiplusStartup(
        token: *mut ULONG_PTR,
        input: *const GdiplusStartupInput,
        output: *mut GdiplusStartupOutput,
    ) -> Status;
    pub fn GdiplusShutdown(
        token: ULONG_PTR,
    );
}

use ctypes::{c_float, c_int};
use shared::basetsd::UINT32;
use shared::guiddef::{GUID, LPCGUID};
use shared::minwindef::{BOOL, DWORD};
use um::audioclient::ISimpleAudioVolume;
use um::audiosessiontypes::AudioSessionState;
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::{HRESULT, LPCWSTR, LPWSTR};
ENUM!{enum AudioSessionDisconnectReason {
  DisconnectReasonDeviceRemoval = 0,
  DisconnectReasonServerShutdown = 0x1,
  DisconnectReasonFormatChanged = 0x2,
  DisconnectReasonSessionLogoff = 0x3,
  DisconnectReasonSessionDisconnected = 0x4,
  DisconnectReasonExclusiveModeOverride = 0x5,
}}
DEFINE_GUID!{IID_IAudioSessionEvents,
  0x24918acc, 0x64b3, 0x37c1, 0x8c, 0xa9, 0x74, 0xa6, 0x6e, 0x99, 0x57, 0xa8}
DEFINE_GUID!{IID_IAudioSessionControl,
  0xf4b1a599, 0x7266, 0x4319, 0xa8, 0xca, 0xe7, 0x0a, 0xcb, 0x11, 0xe8, 0xcd}
DEFINE_GUID!{IID_IAudioSessionControl2,
  0xbfb7ff88, 0x7239, 0x4fc9, 0x8f, 0xa2, 0x07, 0xc9, 0x50, 0xbe, 0x9c, 0x6d}
DEFINE_GUID!{IID_IAudioSessionManager,
  0xbfa971f1, 0x4d5e, 0x40bb, 0x93, 0x5e, 0x96, 0x70, 0x39, 0xbf, 0xbe, 0xe4}
DEFINE_GUID!{IID_IAudioVolumeDuckNotification,
  0xc3b284d4, 0x6d39, 0x4359, 0xb3, 0xcf, 0xb5, 0x6d, 0xdb, 0x3b, 0xb3, 0x9c}
DEFINE_GUID!{IID_IAudioSessionNotification,
  0x641dd20b, 0x4d41, 0x49cc, 0xab, 0xa3, 0x17, 0x4b, 0x94, 0x77, 0xbb, 0x08}
DEFINE_GUID!{IID_IAudioSessionEnumerator,
  0xe2f5bb11, 0x0570, 0x40ca, 0xac, 0xdd, 0x3a, 0xa0, 0x12, 0x77, 0xde, 0xe8}
DEFINE_GUID!{IID_IAudioSessionManager2,
  0x77aa99a0, 0x1bd6, 0x484f, 0x8b, 0xc7, 0x2c, 0x65, 0x4c, 0x9a, 0x9b, 0x6f}
RIDL!{#[uuid(0x24918acc, 0x64b3, 0x37c1, 0x8c, 0xa9, 0x74, 0xa6, 0x6e, 0x99, 0x57, 0xa8)]
interface IAudioSessionEvents(IAudioSessionEventsVtbl): IUnknown(IUnknownVtbl) {
  fn OnDisplayNameChanged(
    NewDisplayName: LPCWSTR,
    EventContext: LPCGUID,
  ) -> HRESULT,
  fn OnIconPathChanged(
    NewIconPath: LPCWSTR,
    EventContext: LPCGUID,
  ) -> HRESULT,
  fn OnSimpleVolumeChanged(
    NewVolume: c_float,
    NewMute: BOOL,
    EventContext: LPCGUID,
  ) -> HRESULT,
  fn OnChannelVolumeChanged(
    ChannelCount: DWORD,
    NewChannelVolumeArray: *mut c_float,
    ChangedChannel: DWORD,
    EventContext: LPCGUID,
  ) -> HRESULT,
  fn OnGroupingParamChanged(
    NewGroupingParam: LPCGUID,
    EventContext: LPCGUID,
  ) -> HRESULT,
  fn OnStateChanged(
    NewState: AudioSessionState,
  ) -> HRESULT,
  fn OnSessionDisconnected(
    DisconnectReason: AudioSessionDisconnectReason,
  ) -> HRESULT,
}}
RIDL!{#[uuid(0xf4b1a599, 0x7266, 0x4319, 0xa8, 0xca, 0xe7, 0x0a, 0xcb, 0x11, 0xe8, 0xcd)]
interface IAudioSessionControl(IAudioSessionControlVtbl): IUnknown(IUnknownVtbl) {
  fn GetState(
    pRetVal: *mut AudioSessionState,
  ) -> HRESULT,
  fn GetDisplayName(
    pRetVal: *mut LPWSTR,
  ) -> HRESULT,
  fn SetDisplayName(
    Value: LPCWSTR,
    EventContext: LPCGUID,
  ) -> HRESULT,
  fn GetIconPath(
    pRetVal: *mut LPWSTR,
  ) -> HRESULT,
  fn SetIconPath(
    Value: LPCWSTR,
    EventContext: LPCGUID,
  ) -> HRESULT,
  fn GetGroupingParam(
    pRetVal: *mut GUID,
  ) -> HRESULT,
  fn SetGroupingParam(
    Override: LPCGUID,
    EventContext: LPCGUID,
  ) -> HRESULT,
  fn RegisterAudioSessionNotification(
    NewNotifications: *mut IAudioSessionEvents,
  ) -> HRESULT,
  fn UnregisterAudioSessionNotification(
    NewNotifications: *mut IAudioSessionEvents,
  ) -> HRESULT,
}}
RIDL!{#[uuid(0xbfb7ff88, 0x7239, 0x4fc9, 0x8f, 0xa2, 0x07, 0xc9, 0x50, 0xbe, 0x9c, 0x6d)]
interface IAudioSessionControl2(IAudioSessionControl2Vtbl):
  IAudioSessionControl(IAudioSessionControlVtbl) {
  fn GetSessionIdentifier(
    pRetVal: *mut LPWSTR,
  ) -> HRESULT,
  fn GetSessionInstanceIdentifier(
    pRetVal: *mut LPWSTR,
  ) -> HRESULT,
  fn GetProcessId(
    pRetVal: *mut DWORD,
  ) -> HRESULT,
  fn IsSystemSoundsSession() -> HRESULT,
  fn SetDuckingPreference(
    optOut: BOOL,
  ) -> HRESULT,
}}
RIDL!{#[uuid(0xbfa971f1, 0x4d5e, 0x40bb, 0x93, 0x5e, 0x96, 0x70, 0x39, 0xbf, 0xbe, 0xe4)]
interface IAudioSessionManager(IAudioSessionManagerVtbl): IUnknown(IUnknownVtbl) {
  fn GetAudioSessionControl(
    AudioSessionGuid: LPCGUID,
    StreamFlags: DWORD,
    SessionControl: *mut *mut IAudioSessionControl,
  ) -> HRESULT,
  fn GetSimpleAudioVolume(
    AudioSessionGuid: LPCGUID,
    StreamFlags: DWORD,
    AudioVolume: *mut *mut ISimpleAudioVolume,
  ) -> HRESULT,
}}
RIDL!{#[uuid(0xc3b284d4, 0x6d39, 0x4359, 0xb3, 0xcf, 0xb5, 0x6d, 0xdb, 0x3b, 0xb3, 0x9c)]
interface IAudioVolumeDuckNotification(IAudioVolumeDuckNotificationVtbl): IUnknown(IUnknownVtbl) {
  fn OnVolumeDuckNotification(
    sessionID: LPCWSTR,
    countCommunicationSessions: UINT32,
  ) -> HRESULT,
  fn OnVolumeUnduckNotification(
    sessionID: LPCWSTR,
  ) -> HRESULT,
}}
RIDL!{#[uuid(0x641dd20b, 0x4d41, 0x49cc, 0xab, 0xa3, 0x17, 0x4b, 0x94, 0x77, 0xbb, 0x08)]
interface IAudioSessionNotification(IAudioSessionNotificationVtbl): IUnknown(IUnknownVtbl) {
  fn OnSessionCreated(
    NewSession: *mut IAudioSessionControl,
  ) -> HRESULT,
}}
RIDL!{#[uuid(0xe2f5bb11, 0x0570, 0x40ca, 0xac, 0xdd, 0x3a, 0xa0, 0x12, 0x77, 0xde, 0xe8)]
interface IAudioSessionEnumerator(IAudioSessionEnumeratorVtbl): IUnknown(IUnknownVtbl) {
  fn GetCount(
    SessionCount: *mut c_int,
  ) -> HRESULT,
  fn GetSession(
    SessionCount: c_int,
    Session: *mut *mut IAudioSessionControl,
  ) -> HRESULT,
}}
RIDL!{#[uuid(0x77aa99a0, 0x1bd6, 0x484f, 0x8b, 0xc7, 0x2c, 0x65, 0x4c, 0x9a, 0x9b, 0x6f)]
interface IAudioSessionManager2(IAudioSessionManager2Vtbl):
  IAudioSessionManager(IAudioSessionManagerVtbl) {
  fn GetSessionEnumerator(
    SessionEnum: *mut *mut IAudioSessionEnumerator,
  ) -> HRESULT,
  fn RegisterSessionNotification(
    SessionNotification: *mut IAudioSessionNotification,
  ) -> HRESULT,
  fn UnregisterSessionNotification(
    SessionNotification: *mut IAudioSessionNotification,
  ) -> HRESULT,
  fn RegisterDuckNotification(
    sessionID: LPCWSTR,
    duckNotification: *mut IAudioVolumeDuckNotification,
  ) -> HRESULT,
  fn UnregisterDuckNotification(
    duckNotification: *mut IAudioVolumeDuckNotification,
  ) -> HRESULT,
}}

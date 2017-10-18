// Copyright © 2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! defines the unified provider side user mode API
use shared::basetsd::{ULONG64, ULONG_PTR};
use shared::guiddef::{LPCGUID, LPGUID};
use shared::minwindef::{UCHAR, ULONG, USHORT};
use um::winnt::{ANYSIZE_ARRAY, BOOLEAN, PCWSTR, PVOID, ULONGLONG, VOID};
pub const EVENT_MIN_LEVEL: UCHAR = 0;
pub const EVENT_MAX_LEVEL: UCHAR = 0xff;
pub const EVENT_ACTIVITY_CTRL_GET_ID: ULONG = 1;
pub const EVENT_ACTIVITY_CTRL_SET_ID: ULONG = 2;
pub const EVENT_ACTIVITY_CTRL_CREATE_ID: ULONG = 3;
pub const EVENT_ACTIVITY_CTRL_GET_SET_ID: ULONG = 4;
pub const EVENT_ACTIVITY_CTRL_CREATE_SET_ID: ULONG = 5;
pub type REGHANDLE = ULONGLONG;
pub type PREGHANDLE = *mut ULONGLONG;
pub const MAX_EVENT_DATA_DESCRIPTORS: usize = 128;
pub const MAX_EVENT_FILTER_DATA_SIZE: usize = 1024;
pub const MAX_EVENT_FILTER_PAYLOAD_SIZE: usize = 4096;
pub const MAX_EVENT_FILTER_EVENT_NAME_SIZE: usize = 4096;
pub const MAX_EVENT_FILTERS_COUNT: usize = 8;
pub const MAX_EVENT_FILTER_PID_COUNT: usize = 8;
pub const MAX_EVENT_FILTER_EVENT_ID_COUNT: usize = 64;
pub const EVENT_FILTER_TYPE_NONE: ULONG = 0x00000000;
pub const EVENT_FILTER_TYPE_SCHEMATIZED: ULONG = 0x80000000;
pub const EVENT_FILTER_TYPE_SYSTEM_FLAGS: ULONG = 0x80000001;
pub const EVENT_FILTER_TYPE_TRACEHANDLE: ULONG = 0x80000002;
pub const EVENT_FILTER_TYPE_PID: ULONG = 0x80000004;
pub const EVENT_FILTER_TYPE_EXECUTABLE_NAME: ULONG = 0x80000008;
pub const EVENT_FILTER_TYPE_PACKAGE_ID: ULONG = 0x80000010;
pub const EVENT_FILTER_TYPE_PACKAGE_APP_ID: ULONG = 0x80000020;
pub const EVENT_FILTER_TYPE_PAYLOAD: ULONG = 0x80000100;
pub const EVENT_FILTER_TYPE_EVENT_ID: ULONG = 0x80000200;
pub const EVENT_FILTER_TYPE_EVENT_NAME: ULONG = 0x80000400;
pub const EVENT_FILTER_TYPE_STACKWALK: ULONG = 0x80001000;
pub const EVENT_DATA_DESCRIPTOR_TYPE_NONE: UCHAR = 0;
pub const EVENT_DATA_DESCRIPTOR_TYPE_EVENT_METADATA: UCHAR = 1;
pub const EVENT_DATA_DESCRIPTOR_TYPE_PROVIDER_METADATA: UCHAR = 2;
pub const EVENT_DATA_DESCRIPTOR_TYPE_TIMESTAMP_OVERRIDE: UCHAR = 3;
pub const EVENT_WRITE_FLAG_NO_FAULTING: ULONG = 0x00000001;
pub const EVENT_WRITE_FLAG_INPRIVATE: ULONG = 0x00000002;
STRUCT!{struct EVENT_DATA_DESCRIPTOR_u_s {
    Type: UCHAR,
    Reserved1: UCHAR,
    Reserved2: USHORT,
}}
UNION2!{union EVENT_DATA_DESCRIPTOR_u {
    [u32; 1],
    Reserved Reserved_mut: ULONG,
    s s_mut: EVENT_DATA_DESCRIPTOR_u_s,
}}
STRUCT!{struct EVENT_DATA_DESCRIPTOR {
    Ptr: ULONGLONG,
    Size: ULONG,
    u: EVENT_DATA_DESCRIPTOR_u,
}}
pub type PEVENT_DATA_DESCRIPTOR = *mut EVENT_DATA_DESCRIPTOR;
STRUCT!{struct EVENT_DESCRIPTOR {
    Id: USHORT,
    Version: UCHAR,
    Channel: UCHAR,
    Level: UCHAR,
    Opcode: UCHAR,
    Task: USHORT,
    Keyword: ULONGLONG,
}}
pub type PEVENT_DESCRIPTOR = *mut EVENT_DESCRIPTOR;
pub type PCEVENT_DESCRIPTOR = *const EVENT_DESCRIPTOR;
STRUCT!{struct EVENT_FILTER_DESCRIPTOR {
    Ptr: ULONGLONG,
    Size: ULONG,
    Type: ULONG,
}}
pub type PEVENT_FILTER_DESCRIPTOR = *mut EVENT_FILTER_DESCRIPTOR;
STRUCT!{struct EVENT_FILTER_HEADER {
    Id: USHORT,
    Version: UCHAR,
    Reserved: [UCHAR; 5],
    InstanceId: ULONGLONG,
    Size: ULONG,
    NextOffset: ULONG,
}}
pub type PEVENT_FILTER_HEADER = *mut EVENT_FILTER_HEADER;
STRUCT!{struct EVENT_FILTER_EVENT_ID {
    FilterIn: BOOLEAN,
    Reserved: UCHAR,
    Count: USHORT,
    Events: [USHORT; ANYSIZE_ARRAY],
}}
pub type PEVENT_FILTER_EVENT_ID = *mut EVENT_FILTER_EVENT_ID;
STRUCT!{struct EVENT_FILTER_EVENT_NAME {
    MatchAnyKeyword: ULONGLONG,
    MatchAllKeyword: ULONGLONG,
    Level: UCHAR,
    FilterIn: BOOLEAN,
    NameCount: USHORT,
    Names: [UCHAR; ANYSIZE_ARRAY],
}}
pub type PEVENT_FILTER_EVENT_NAME = *mut EVENT_FILTER_EVENT_NAME;
ENUM!{enum EVENT_INFO_CLASS {
    EventProviderBinaryTrackInfo,
    EventProviderSetEnableAllKeywords,
    EventProviderSetTraits,
    EventProviderUseDescriptorType,
    MaxEventInfo,
}}
FN!{stdcall PENABLECALLBACK(
    SourceId: LPCGUID,
    IsEnabled: ULONG,
    Level: UCHAR,
    MatchAnyKeyword: ULONGLONG,
    MatchAllKeyword: ULONGLONG,
    FilterData: PEVENT_FILTER_DESCRIPTOR,
    CallbackContext: PVOID,
) -> ()}
extern "system" {
    pub fn EventRegister (
        ProviderId: LPCGUID,
        EnableCallback: PENABLECALLBACK,
        CallbackContext: PVOID,
        RegHandle: PREGHANDLE,
    ) -> ULONG;
    pub fn EventUnregister (
        RegHandle: REGHANDLE,
    ) -> ULONG;
    pub fn EventSetInformation (
        RegHandle: REGHANDLE,
        InformationClass: EVENT_INFO_CLASS,
        EventInformation: PVOID,
        InformationLength: ULONG,
    ) -> ULONG;
    pub fn EventEnabled (
        RegHandle: REGHANDLE,
        EventDescriptor: PCEVENT_DESCRIPTOR,
    ) -> BOOLEAN;
    pub fn EventProviderEnabled (
        RegHandle: REGHANDLE,
        Level: UCHAR,
        Keyword: ULONGLONG,
    ) -> BOOLEAN;
    pub fn EventWrite (
        RegHandle: REGHANDLE,
        EventDescriptor: PCEVENT_DESCRIPTOR,
        UserDataCount: ULONG,
        UserData: PEVENT_DATA_DESCRIPTOR,
    ) -> ULONG;
    pub fn EventWriteTransfer (
        RegHandle: REGHANDLE,
        EventDescriptor: PCEVENT_DESCRIPTOR,
        ActivityId: LPCGUID,
        RelatedActivityId: LPCGUID,
        UserDataCount: ULONG,
        UserData: PEVENT_DATA_DESCRIPTOR,
    ) -> ULONG;
    pub fn EventWriteEx (
        RegHandle: REGHANDLE,
        EventDescriptor: PCEVENT_DESCRIPTOR,
        Filter: ULONG64,
        Flags: ULONG,
        ActivityId: LPCGUID,
        RelatedActivityId: LPCGUID,
        UserDataCount: ULONG,
        UserData: PEVENT_DATA_DESCRIPTOR,
    ) -> ULONG;
    pub fn EventWriteString (
        RegHandle: REGHANDLE,
        Level: UCHAR,
        Keyword: ULONGLONG,
        String_: PCWSTR,
    ) -> ULONG;
    pub fn EventActivityIdControl (
        ControlCode: ULONG,
        ActivityId: LPGUID,
    ) -> ULONG;
}
#[inline]
pub fn EventDataDescCreate(
    EventDataDescriptor: &mut EVENT_DATA_DESCRIPTOR, DataPtr: *const VOID, DataSize: ULONG) {
    EventDataDescriptor.Ptr = DataPtr as ULONG_PTR as ULONGLONG;
    EventDataDescriptor.Size = DataSize;
    unsafe { *EventDataDescriptor.u.Reserved_mut() = 0; }
}
#[inline]
pub fn EventDescCreate(
    EventDescriptor: &mut EVENT_DESCRIPTOR, Id: USHORT, Version: UCHAR, Channel: UCHAR,
    Level: UCHAR, Task: USHORT, Opcode: UCHAR, Keyword: ULONGLONG) {
    EventDescriptor.Id = Id;
    EventDescriptor.Version = Version;
    EventDescriptor.Channel = Channel;
    EventDescriptor.Level = Level;
    EventDescriptor.Task = Task;
    EventDescriptor.Opcode = Opcode;
    EventDescriptor.Keyword = Keyword;
}
#[inline]
pub fn EventDescGetId(EventDescriptor: &EVENT_DESCRIPTOR) -> USHORT {
    EventDescriptor.Id
}
#[inline]
pub fn EventDescGetVersion(EventDescriptor: &EVENT_DESCRIPTOR) -> UCHAR {
    EventDescriptor.Version
}
#[inline]
pub fn EventDescGetTask(EventDescriptor: &EVENT_DESCRIPTOR) -> USHORT {
    EventDescriptor.Task
}
#[inline]
pub fn EventDescGetOpcode(EventDescriptor: &EVENT_DESCRIPTOR) -> UCHAR {
    EventDescriptor.Opcode
}
#[inline]
pub fn EventDescGetChannel(EventDescriptor: &EVENT_DESCRIPTOR) -> UCHAR {
    EventDescriptor.Channel
}
#[inline]
pub fn EventDescGetLevel(EventDescriptor: &EVENT_DESCRIPTOR) -> UCHAR {
    EventDescriptor.Level
}
#[inline]
pub fn EventDescGetKeyword(EventDescriptor: &EVENT_DESCRIPTOR) -> ULONGLONG {
    EventDescriptor.Keyword
}
#[inline]
pub fn EventDescSetId(EventDescriptor: &mut EVENT_DESCRIPTOR, Id: USHORT) {
    EventDescriptor.Id = Id;
}
#[inline]
pub fn EventDescSetVersion(EventDescriptor: &mut EVENT_DESCRIPTOR, Version: UCHAR) {
    EventDescriptor.Version = Version;
}
#[inline]
pub fn EventDescSetTask(EventDescriptor: &mut EVENT_DESCRIPTOR, Task: USHORT) {
    EventDescriptor.Task = Task;
}
#[inline]
pub fn EventDescSetOpcode(EventDescriptor: &mut EVENT_DESCRIPTOR, Opcode: UCHAR) {
    EventDescriptor.Opcode = Opcode;
}
#[inline]
pub fn EventDescSetLevel(EventDescriptor: &mut EVENT_DESCRIPTOR, Level: UCHAR) {
    EventDescriptor.Level = Level;
}
#[inline]
pub fn EventDescSetChannel(EventDescriptor: &mut EVENT_DESCRIPTOR, Channel: UCHAR) {
    EventDescriptor.Channel = Channel;
}
#[inline]
pub fn EventDescSetKeyword(EventDescriptor: &mut EVENT_DESCRIPTOR, Keyword: ULONGLONG) {
    EventDescriptor.Keyword |= Keyword;
}

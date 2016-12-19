// Copyright © 2016 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.

use shared::minwindef::{BOOL, LPDWORD, PULONG};

use shared::guiddef::GUID;

use um::winnt::{HANDLE, PLARGE_INTEGER, LONGLONG, PHANDLE, LPCWSTR, LPCSTR};

ENUM!{enum AVRT_PRIORITY {
    AVRT_PRIORITY_VERYLOW = -2i32 as u32,
    AVRT_PRIORITY_LOW = -1i32 as u32,
    AVRT_PRIORITY_NORMAL = 0u32,
    AVRT_PRIORITY_HIGH = 1u32,
    AVRT_PRIORITY_CRITICAL = 2u32,
}}

pub const THREAD_ORDER_GROUP_INFINITE_TIMEOUT: LONGLONG = -1;

FN!(stdcall AvSetMmThreadCharacteristicsA(TaskName: LPCSTR,
                                         TaskIndex: LPDWORD)
                                         -> HANDLE);

FN!(stdcall AvSetMmThreadCharacteristicsW(TaskName: LPCSTR,
                                         TaskIndex: LPDWORD)
                                         -> HANDLE);

FN!(stdcall AvSetMmMaxThreadCharacteristicsA(FirstTask: LPCSTR,
                                            SecondTask: LPCSTR,
                                            TaskIndex: LPDWORD)
                                            -> HANDLE);

FN!(stdcall AvSetMmMaxThreadCharacteristicsW(FirstTask: LPCWSTR,
                                            SecondTask: LPCWSTR,
                                            TaskIndex: LPDWORD)
                                            -> HANDLE);

FN!(stdcall AvRevertMmThreadCharacteristics(avrt_handle: HANDLE) -> BOOL);

FN!( stdcall AvSetMmThreadPriority(AvrtHandle: HANDLE,
                                 Priority: AVRT_PRIORITY)
                                 -> BOOL);

FN!( stdcall AvRtCreateThreadOrderingGroup(Context: PHANDLE,
                                         Period: PLARGE_INTEGER,
                                         ThreadOrderingGuid: *mut GUID,
                                         Timeout: PLARGE_INTEGER)
                                         -> BOOL);

FN!( stdcall AvRtCreateThreadOrderingGroupExA(Context: PHANDLE,
                                            Period: PLARGE_INTEGER,
                                            ThreadOrderingGuid: *mut GUID,
                                            Timeout: PLARGE_INTEGER,
                                            TaskName: LPCSTR)
                                            -> BOOL);

FN!( stdcall AvRtCreateThreadOrderingGroupExW(Context: PHANDLE,
                                            Period: PLARGE_INTEGER,
                                            ThreadOrderingGuid: *mut GUID,
                                            Timeout: PLARGE_INTEGER,
                                            TaskName: LPCWSTR)
                                            -> BOOL);

FN!( stdcall AvRtJoinThreadOrderingGroup(Context: PHANDLE,
                                       ThreadOrderingGuid: *mut GUID,
                                       Before: BOOL)
                                       -> BOOL);

FN!( stdcall AvRtWaitOnThreadOrderingGroup(Context: HANDLE) -> BOOL);

FN!( stdcall AvRtLeaveThreadOrderingGroup(Context: HANDLE) -> BOOL);

FN!( stdcall AvRtDeleteThreadOrderingGroup(Context: HANDLE) -> BOOL);

FN!( stdcall AvQuerySystemResponsiveness(AvrtHandle: HANDLE,
                                       SystemResponsivenessValue: PULONG)
                                       -> BOOL);

// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use ctypes::{c_void};
use shared::minwindef::{DWORD};
use um::winnt::{HRESULT};
pub const EC_SYSTEMBASE: DWORD = 0x0;
pub const EC_USER: DWORD = 0x8000;
pub const EC_COMPLETE: DWORD = 0x1;
pub const EC_USERABORT: DWORD = 0x2;
pub const EC_ERRORABORT: DWORD = 0x3;
pub const EC_TIME: DWORD = 0x4;
pub const EC_REPAINT: DWORD = 0x5;
pub const EC_STREAM_ERROR_STOPPED: DWORD = 0x6;
pub const EC_STREAM_ERROR_STILLPLAYING: DWORD = 0x7;
pub const EC_ERROR_STILLPLAYING: DWORD = 0x8;
pub const EC_PALETTE_CHANGED: DWORD = 0x9;
pub const EC_VIDEO_SIZE_CHANGED: DWORD = 0x0a;
pub const EC_QUALITY_CHANGE: DWORD = 0x0b;
pub const EC_SHUTTING_DOWN: DWORD = 0x0c;
pub const EC_CLOCK_CHANGED: DWORD = 0x0d;
pub const EC_PAUSED: DWORD = 0x0e;
pub const EC_OPENING_FILE: DWORD = 0x10;
pub const EC_BUFFERING_DATA: DWORD = 0x11;
pub const EC_FULLSCREEN_LOST: DWORD = 0x12;
pub const EC_ACTIVATE: DWORD = 0x13;
pub const EC_NEED_RESTART: DWORD = 0x14;
pub const EC_WINDOW_DESTROYED: DWORD = 0x15;
pub const EC_DISPLAY_CHANGED: DWORD = 0x16;
pub const EC_STARVATION: DWORD = 0x17;
pub const EC_OLE_EVENT: DWORD = 0x18;
pub const EC_NOTIFY_WINDOW_: DWORD = 0x19;
pub const EC_STREAM_CONTROL_STOPPED: DWORD = 0x1a;
pub const EC_STREAM_CONTROL_STARTED: DWORD = 0x1b;
pub const EC_END_OF_SEGMENT: DWORD = 0x1c;
pub const EC_SEGMENT_STARTED: DWORD = 0x1d;
pub const EC_LENGTH_CHANGED: DWORD = 0x1e;
pub const EC_DEVICE_LOST: DWORD = 0x1f;
pub const EC_SAMPLE_NEEDED: DWORD = 0x20;
pub const EC_PROCESSING_LATENCY: DWORD = 0x21;
pub const EC_SAMPLE_LATENCY: DWORD = 0x22;
pub const EC_SCRUB_TIME: DWORD = 0x23;
pub const EC_STEP_COMPLETE: DWORD = 0x24;
pub const EC_TIMECODE_AVAILABLE: DWORD = 0x30;
pub const EC_EXTDEVICE_MODE_CHANGE: DWORD = 0x31;
pub const EC_STATE_CHANGE: DWORD = 0x32;
pub const EC_GRAPH_CHANGED: DWORD = 0x50;
pub const EC_CLOCK_UNSET: DWORD = 0x51;
pub const EC_VMR_RENDERDEVICE_SET: DWORD = 0x53;
pub const VMR_RENDER_DEVICE_OVERLAY: DWORD = 0x01;
pub const VMR_RENDER_DEVICE_VIDMEM: DWORD = 0x02;
pub const VMR_RENDER_DEVICE_SYSTEM: DWORD = 0x04;
pub const EC_VMR_SURFACE_FLIPPED: DWORD = 0x54;
pub const EC_VMR_RECONNECTION_FAILED: DWORD = 0x55;
pub const EC_VMR_PREPROCESS_COMPLETE: DWORD = 0x56;
pub const EC_VMR_CODECAPI_EVENT: DWORD = 0x57;
STRUCT!{struct AM_WM_EVENT_DATA {
    hrStatus: HRESULT,
    pData: *mut c_void,
}}
pub const EC_WMT_EVENT_BASE: DWORD = 0x0251;
pub const EC_WMT_INDEX_EVENT: DWORD = EC_WMT_EVENT_BASE;
pub const EC_WMT_EVENT: DWORD = EC_WMT_EVENT_BASE + 1;
pub const EC_BUILT: DWORD = 0x300;
pub const EC_UNBUILT: DWORD = 0x301;
pub const EC_SKIP_FRAMES: DWORD = 0x25;
pub const EC_PLEASE_REOPEN: DWORD = 0x40;
pub const EC_STATUS: DWORD = 0x41;
pub const EC_MARKER_HIT: DWORD = 0x42;
pub const EC_LOADSTATUS: DWORD = 0x43;
pub const EC_FILE_CLOSED: DWORD = 0x44;
pub const EC_ERRORABORTEX: DWORD = 0x45;
pub const AM_LOADSTATUS_CLOSED: DWORD = 0x0000;
pub const AM_LOADSTATUS_LOADINGDESCR: DWORD = 0x0001;
pub const AM_LOADSTATUS_LOADINGMCAST: DWORD = 0x0002;
pub const AM_LOADSTATUS_LOCATING: DWORD = 0x0003;
pub const AM_LOADSTATUS_CONNECTING: DWORD = 0x0004;
pub const AM_LOADSTATUS_OPENING: DWORD = 0x0005;
pub const AM_LOADSTATUS_OPEN: DWORD = 0x0006;
pub const EC_NEW_PIN: DWORD = 0x20;
pub const EC_RENDER_FINISHED: DWORD = 0x21;
pub const EC_EOS_SOON: DWORD = 0x046;
pub const EC_CONTENTPROPERTYCHANGED: DWORD = 0x47;
pub const AM_CONTENTPROPERTY_TITLE: DWORD = 0x0001;
pub const AM_CONTENTPROPERTY_AUTHOR: DWORD = 0x0002;
pub const AM_CONTENTPROPERTY_COPYRIGHT: DWORD = 0x0004;
pub const AM_CONTENTPROPERTY_DESCRIPTION: DWORD = 0x0008;
pub const EC_BANDWIDTHCHANGE: DWORD = 0x48;
pub const EC_VIDEOFRAMEREADY: DWORD = 0x49;

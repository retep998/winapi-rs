// Copyright © 2018 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.

// #include <winapifamily.h>
use ctypes::*;
use shared::minwindef::*;
use shared::basetsd::*;
use shared::ntdef::*;
use shared::ws2def::*;
use shared::guiddef::GUID;
use um::minwinbase::{
    OVERLAPPED, LPOVERLAPPED, 
};


ENUM!{enum TCP_ESTATS_TYPE {
    TcpConnectionEstatsSynOpts,
    TcpConnectionEstatsData,
    TcpConnectionEstatsSndCong,
    TcpConnectionEstatsPath,
    TcpConnectionEstatsSendBuff,
    TcpConnectionEstatsRec,
    TcpConnectionEstatsObsRec,
    TcpConnectionEstatsBandwidth,
    TcpConnectionEstatsFineRtt,
    TcpConnectionEstatsMaximum,
}}
pub type PTCP_ESTATS_TYPE = *mut TCP_ESTATS_TYPE;

ENUM!{enum TCP_BOOLEAN_OPTIONAL {
    TcpBoolOptDisabled = 0,
    TcpBoolOptEnabled = 1,
    TcpBoolOptUnchanged = -1,
}}
pub type PTCP_BOOLEAN_OPTIONAL = *mut TCP_BOOLEAN_OPTIONAL;

STRUCT!{struct TCP_ESTATS_SYN_OPTS_ROS_v0 {
    ActiveOpen: BOOLEAN,
    MssRcvd: ULONG,
    MssSent: ULONG,
}}
pub type PTCP_ESTATS_SYN_OPTS_ROS_v0 = *mut TCP_ESTATS_SYN_OPTS_ROS_v0;

ENUM!{enum TCP_SOFT_ERROR {
    TcpErrorNone = 0,
    TcpErrorBelowDataWindow,
    TcpErrorAboveDataWindow,
    TcpErrorBelowAckWindow,
    TcpErrorAboveAckWindow,
    TcpErrorBelowTsWindow,
    TcpErrorAboveTsWindow,
    TcpErrorDataChecksumError,
    TcpErrorDataLengthError,
    TcpErrorMaxSoftError,
}}
pub type PTCP_SOFT_ERROR = *mut TCP_SOFT_ERROR;

STRUCT!{struct TCP_ESTATS_DATA_ROD_v0 {
    DataBytesOut: ULONG64,
    DataSegsOut: ULONG64,
    DataBytesIn: ULONG64,
    DataSegsIn: ULONG64,
    SegsOut: ULONG64,
    SegsIn: ULONG64,
    SoftErrors: ULONG,
    SoftErrorReason: ULONG,
    SndUna: ULONG,
    SndNxt: ULONG,
    SndMax: ULONG,
    ThruBytesAcked: ULONG64,
    RcvNxt: ULONG,
    ThruBytesReceived: ULONG64,
}}
pub type PTCP_ESTATS_DATA_ROD_v0 = *mut TCP_ESTATS_DATA_ROD_v0;

STRUCT!{struct TCP_ESTATS_DATA_RW_v0 {
    EnableCollection: BOOLEAN,
}}
pub type PTCP_ESTATS_DATA_RW_v0 = TCP_ESTATS_DATA_RW_v0;

STRUCT!{struct TCP_ESTATS_SND_CONG_ROD_v0 {
    SndLimTransRwin: ULONG,
    SndLimTimeRwin: ULONG,
    SndLimBytesRwin: SIZE_T,
    SndLimTransCwnd: ULONG,
    SndLimTimeCwnd: ULONG,
    SndLimBytesCwnd: SIZE_T,
    SndLimTransSnd: ULONG,
    SndLimTimeSnd: ULONG,
    SndLimBytesSnd: SIZE_T,
    SlowStart: ULONG,
    CongAvoid: ULONG,
    OtherReductions: ULONG,
    CurCwnd: ULONG,
    MaxSsCwnd: ULONG,
    MaxCaCwnd: ULONG,
    CurSsthresh: ULONG,
    MaxSsthresh: ULONG,
    MinSsthresh: ULONG,
}}
pub type PTCP_ESTATS_SND_CONG_ROD_v0 = *mut TCP_ESTATS_SND_CONG_ROD_v0;

STRUCT!{struct TCP_ESTATS_SND_CONG_ROS_v0 {
    LimCwnd: ULONG,
}}
pub type PTCP_ESTATS_SND_CONG_ROS_v0 = *mut TCP_ESTATS_SND_CONG_ROS_v0;

STRUCT!{struct TCP_ESTATS_SND_CONG_RW_v0 {
    EnableCollection: BOOLEAN,
}}
pub type PTCP_ESTATS_SND_CONG_RW_v0 = *mut TCP_ESTATS_SND_CONG_RW_v0;

STRUCT!{struct TCP_ESTATS_PATH_ROD_v0 {
    FastRetran: ULONG,
    Timeouts: ULONG,
    SubsequentTimeouts: ULONG,
    CurTimeoutCount: ULONG,
    AbruptTimeouts: ULONG,
    PktsRetrans: ULONG,
    BytesRetrans: ULONG,
    DupAcksIn: ULONG,
    SacksRcvd: ULONG,
    SackBlocksRcvd: ULONG,
    CongSignals: ULONG,
    PreCongSumCwnd: ULONG,
    PreCongSumRtt: ULONG,
    PostCongSumRtt: ULONG,
    PostCongCountRtt: ULONG,
    EcnSignals: ULONG,
    EceRcvd: ULONG,
    SendStall: ULONG,
    QuenchRcvd: ULONG,
    RetranThresh: ULONG,
    SndDupAckEpisodes: ULONG,
    SumBytesReordered: ULONG,
    NonRecovDa: ULONG,
    NonRecovDaEpisodes: ULONG,
    AckAfterFr: ULONG,
    DsackDups: ULONG,
    SampleRtt: ULONG,
    SmoothedRtt: ULONG,
    RttVar: ULONG,
    MaxRtt: ULONG,
    MinRtt: ULONG,
    SumRtt: ULONG,
    CountRtt: ULONG,
    CurRto: ULONG,
    MaxRto: ULONG,
    MinRto: ULONG,
    CurMss: ULONG,
    MaxMss: ULONG,
    MinMss: ULONG,
    SpuriousRtoDetections: ULONG,
}}
pub type PTCP_ESTATS_PATH_ROD_v0 = *mut TCP_ESTATS_PATH_ROD_v0;

STRUCT!{struct TCP_ESTATS_PATH_RW_v0 {
    EnableCollection: BOOLEAN,
}}
pub type PTCP_ESTATS_PATH_RW_v0 = *mut TCP_ESTATS_PATH_RW_v0;

STRUCT!{struct TCP_ESTATS_SEND_BUFF_ROD_v0 {
    CurRetxQueue: SIZE_T,
    MaxRetxQueue: SIZE_T,
    CurAppWQueue: SIZE_T,
    MaxAppWQueue: SIZE_T,
}}
pub type PTCP_ESTATS_SEND_BUFF_ROD_v0 = *mut TCP_ESTATS_SEND_BUFF_ROD_v0;

STRUCT!{struct TCP_ESTATS_SEND_BUFF_RW_v0 {
    EnableCollection: BOOLEAN,
}}
pub type PTCP_ESTATS_SEND_BUFF_RW_v0 = *mut TCP_ESTATS_SEND_BUFF_RW_v0;

STRUCT!{struct TCP_ESTATS_REC_ROD_v0 {
    CurRwinSent: ULONG,
    MaxRwinSent: ULONG,
    MinRwinSent: ULONG,
    LimRwin: ULONG,
    DupAckEpisodes: ULONG,
    DupAcksOut: ULONG,
    CeRcvd: ULONG,
    EcnSent: ULONG,
    EcnNoncesRcvd: ULONG,
    CurReasmQueue: ULONG,
    MaxReasmQueue: ULONG,
    CurAppRQueue: SIZE_T,
    MaxAppRQueue: SIZE_T,
    WinScaleSent: UCHAR,
}}
pub type PTCP_ESTATS_REC_ROD_v0 = *mut TCP_ESTATS_REC_ROD_v0;

STRUCT!{struct TCP_ESTATS_REC_RW_v0 {
    EnableCollection: BOOLEAN,
}}
pub type PTCP_ESTATS_REC_RW_v0 = *mut TCP_ESTATS_REC_RW_v0;

STRUCT!{struct TCP_ESTATS_OBS_REC_ROD_v0 {
    CurRwinRcvd: ULONG,
    MaxRwinRcvd: ULONG,
    MinRwinRcvd: ULONG,
    WinScaleRcvd: UCHAR,
}}
pub type PTCP_ESTATS_OBS_REC_ROD_v0 = *mut TCP_ESTATS_OBS_REC_ROD_v0;

STRUCT!{struct TCP_ESTATS_OBS_REC_RW_v0 {
    EnableCollection: BOOLEAN,
}}
pub type PTCP_ESTATS_OBS_REC_RW_v0 = *mut TCP_ESTATS_OBS_REC_RW_v0;

STRUCT!{struct TCP_ESTATS_BANDWIDTH_RW_v0 {
    EnableCollectionOutbound: TCP_BOOLEAN_OPTIONAL,
    EnableCollectionInbound: TCP_BOOLEAN_OPTIONAL,
}}
pub type PTCP_ESTATS_BANDWIDTH_RW_v0 = *mut TCP_ESTATS_BANDWIDTH_RW_v0;

STRUCT!{struct TCP_ESTATS_BANDWIDTH_ROD_v0 {
    OutboundBandwidth: ULONG64,
    InboundBandwidth: ULONG64,
    OutboundInstability: ULONG64,
    InboundInstability: ULONG64,
    OutboundBandwidthPeaked: BOOLEAN,
    InboundBandwidthPeaked: BOOLEAN,
}}
pub type PTCP_ESTATS_BANDWIDTH_ROD_v0 = *mut TCP_ESTATS_BANDWIDTH_ROD_v0;

STRUCT!{struct TCP_ESTATS_FINE_RTT_RW_v0 {
    EnableCollection: BOOLEAN,
}}
pub type PTCP_ESTATS_FINE_RTT_RW_v0 = *mut TCP_ESTATS_FINE_RTT_RW_v0;

STRUCT!{struct TCP_ESTATS_FINE_RTT_ROD_v0 {
    RttVar: ULONG,
    MaxRtt: ULONG,
    MinRtt: ULONG,
    SumRtt: ULONG,
}}
pub type PTCP_ESTATS_FINE_RTT_ROD_v0 = *mut TCP_ESTATS_FINE_RTT_ROD_v0;


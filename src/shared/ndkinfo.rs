// Copyright © 2015-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! NetworkDirect adapter information
use shared::basetsd::{SIZE_T, UINT32};
use shared::minwindef::{ULONG, USHORT};
STRUCT!{struct NDK_VERSION {
    Major: USHORT,
    Minor: USHORT,
}}
ENUM!{enum NDK_RDMA_TECHNOLOGY {
    NdkUndefined = 0,
    NdkiWarp,
    NdkInfiniBand,
    NdkRoCE,
    NdkRoCEv2,
    NdkMaxTechnology,
}}
STRUCT!{struct NDK_ADAPTER_INFO {
    Version: NDK_VERSION,
    VendorId: UINT32,
    DeviceId: UINT32,
    MaxRegistrationSize: SIZE_T,
    MaxWindowSize: SIZE_T,
    FRMRPageCount: ULONG,
    MaxInitiatorRequestSge: ULONG,
    MaxReceiveRequestSge: ULONG,
    MaxReadRequestSge: ULONG,
    MaxTransferLength: ULONG,
    MaxInlineDataSize: ULONG,
    MaxInboundReadLimit: ULONG,
    MaxOutboundReadLimit: ULONG,
    MaxReceiveQueueDepth: ULONG,
    MaxInitiatorQueueDepth: ULONG,
    MaxSrqDepth: ULONG,
    MaxCqDepth: ULONG,
    LargeRequestThreshold: ULONG,
    MaxCallerData: ULONG,
    MaxCalleeData: ULONG,
    AdapterFlags: ULONG,
    RdmaTechnology: NDK_RDMA_TECHNOLOGY,
}}
pub const NDK_ADAPTER_FLAG_IN_ORDER_DMA_SUPPORTED: ULONG = 0x00000001;
pub const NDK_ADAPTER_FLAG_RDMA_READ_SINK_NOT_REQUIRED: ULONG = 0x00000002;
pub const NDK_ADAPTER_FLAG_CQ_INTERRUPT_MODERATION_SUPPORTED: ULONG = 0x00000004;
pub const NDK_ADAPTER_FLAG_MULTI_ENGINE_SUPPORTED: ULONG = 0x00000008;
pub const NDK_ADAPTER_FLAG_RDMA_READ_LOCAL_INVALIDATE_SUPPORTED: ULONG = 0x00000010;
pub const NDK_ADAPTER_FLAG_CQ_RESIZE_SUPPORTED: ULONG = 0x00000100;
pub const NDK_ADAPTER_FLAG_LOOPBACK_CONNECTIONS_SUPPORTED: ULONG = 0x00010000;

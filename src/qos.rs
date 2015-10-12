// Copyright © 2015, skdltmxn
// Licensed under the MIT License <LICENSE.md>
//! QoS definitions for NDIS components.
pub type SERVICETYPE = ::ULONG;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct FLOWSPEC {
    pub TokenRate: ::ULONG,
    pub TokenBucketSize: ::ULONG,
    pub PeakBandwidth: ::ULONG,
    pub Latency: ::ULONG,
    pub DelayVariation: ::ULONG,
    pub ServiceType: SERVICETYPE,
    pub MaxSduSize: ::ULONG,
    pub MinimumPolicedSize: ::ULONG,
}
pub type PFLOWSPEC = *mut FLOWSPEC;
pub type LPFLOWSPEC = *mut FLOWSPEC;

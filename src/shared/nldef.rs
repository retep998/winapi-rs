// Copyright © 2018 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::ntdef::BOOLEAN;
ENUM!{enum NL_LINK_LOCAL_ADDRESS_BEHAVIOR {
    LinkLocalAlwaysOff = 0,
    LinkLocalDelayed,
    LinkLocalAlwaysOn,
    LinkLocalUnchanged = -1i32 as u32
}}
STRUCT!{struct NL_INTERFACE_OFFLOAD_ROD {
    Value: BOOLEAN,
}}
BITFIELD!{NL_INTERFACE_OFFLOAD_ROD Value: BOOLEAN [
    NlChecksumSupported set_NlChecksumSupported[0..1],
    NlOptionsSupported set_NlOptionsSupported[1..2],
    TlDatagramChecksumSupported set_TlDatagramChecksumSupported[2..3],
    TlStreamChecksumSupported set_TlStreamChecksumSupported[3..4],
    TlStreamOptionsSupported set_TlStreamOptionsSupported[4..5],
    FastPathCompatible set_FastPathCompatible[5..6],
    TlLargeSendOffloadSupported set_TlLargeSendOffloadSupported[6..7],
    TlGiantSendOffloadSupported set_TlGiantSendOffloadSupported[7..8],
]}
pub type PNL_INTERFACE_OFFLOAD_ROD = *mut NL_INTERFACE_OFFLOAD_ROD;
ENUM!{enum NL_ROUTER_DISCOVERY_BEHAVIOR {
    RouterDiscoveryDisabled = 0,
    RouterDiscoveryEnabled,
    RouterDiscoveryDhcp,
    RouterDiscoveryUnchanged = -1i32 as u32
}}

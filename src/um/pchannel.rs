// Copyright © 2015-2019 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Windows Terminal Server Virtual Channel protocol header
use shared::basetsd::UINT32;
use shared::minwindef::ULONG;
pub const CHANNEL_CHUNK_LENGTH: ULONG = 1600;
pub const CHANNEL_BUFFER_SIZE: ULONG = 65535;
// FIXME: use sizeof::<CHANNEL_PDU_HEADER>() instead of 8;
pub const CHANNEL_PDU_LENGTH: ULONG = CHANNEL_CHUNK_LENGTH + 8;
pub const CHANNEL_FLAG_FIRST: UINT32 = 0x01;
pub const CHANNEL_FLAG_LAST: UINT32 = 0x02;
pub const CHANNEL_FLAG_ONLY: UINT32 = CHANNEL_FLAG_FIRST | CHANNEL_FLAG_LAST;
pub const CHANNEL_FLAG_MIDDLE: UINT32 = 0;
pub const CHANNEL_FLAG_FAIL: UINT32 = 0x100;
pub const CHANNEL_OPTION_INITIALIZED: ULONG = 0x80000000;
pub const CHANNEL_OPTION_ENCRYPT_RDP: ULONG = 0x40000000;
pub const CHANNEL_OPTION_ENCRYPT_SC: ULONG = 0x20000000;
pub const CHANNEL_OPTION_ENCRYPT_CS: ULONG = 0x10000000;
pub const CHANNEL_OPTION_PRI_HIGH: ULONG = 0x08000000;
pub const CHANNEL_OPTION_PRI_MED: ULONG = 0x04000000;
pub const CHANNEL_OPTION_PRI_LOW: ULONG = 0x02000000;
pub const CHANNEL_OPTION_COMPRESS_RDP: ULONG = 0x00800000;
pub const CHANNEL_OPTION_COMPRESS: ULONG = 0x00400000;
pub const CHANNEL_OPTION_SHOW_PROTOCOL: ULONG = 0x00200000;
pub const CHANNEL_OPTION_REMOTE_CONTROL_PERSISTENT: ULONG = 0x00100000;
pub const CHANNEL_MAX_COUNT: usize = 30;
pub const CHANNEL_NAME_LEN: usize = 7;
// #ifdef PAL_PRAGMA_PACK_PUSH
// #pragma PAL_PRAGMA_PACK_PUSH(pchannel_channel_def, 1)
// #else
// #pragma pack(push, pchannel_channel_def, 1)
// #endif
// typedef struct tagCHANNEL_DEF
// {
//     char            name[CHANNEL_NAME_LEN + 1];
//     ULONG           options;
// } PAL_PACKED_STRUCT(CHANNEL_DEF);
// typedef CHANNEL_DEF UNALIGNED FAR *PCHANNEL_DEF;
// typedef PCHANNEL_DEF UNALIGNED FAR *PPCHANNEL_DEF;
// // Restore original packing
// #ifdef PAL_PRAGMA_PACK_POP
// #pragma PAL_PRAGMA_PACK_POP(pchannel_channel_def)
// #else
// #pragma pack(pop, pchannel_channel_def)
// #endif
STRUCT!{struct CHANNEL_PDU_HEADER {
    length: UINT32,
    flags: UINT32,
}}
pub type PCHANNEL_PDU_HEADER = *mut CHANNEL_PDU_HEADER;

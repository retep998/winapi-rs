// Copyright © 2016-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! WFP Structures and Definitions
use ctypes::{c_float, c_double};
use shared::minwindef::ULONG;
use um::winnt::{LPWSTR, PSID, PSID_AND_ATTRIBUTES};
use shared::basetsd::{UINT8, UINT16, UINT32, INT8, INT16, INT32, PUINT8, PUINT64, PINT64};

// TODO: pub?
pub use shared::wtypes::*;

pub use shared::ntdef::LUID;

//use shared::minwindef::{PUCHAR, UCHAR, ULONG, USHORT};
//use um::winnt::{BOOLEAN, HANDLE, LONG, LPCWSTR, LPWSTR, PVOID, PWSTR, ULONGLONG, VOID};
//pub type NTSTATUS = LONG;
//pub type PNTSTATUS = *mut NTSTATUS;

ENUM!{enum FWP_DIRECTION {
    FWP_DIRECTION_OUTBOUND = 0,
    FWP_DIRECTION_INBOUND = ( FWP_DIRECTION_OUTBOUND + 1 ),
    FWP_DIRECTION_MAX = ( FWP_DIRECTION_INBOUND + 1 ),
}}

ENUM!{enum FWP_IP_VERSION {
    FWP_IP_VERSION_V4 = 0,
    FWP_IP_VERSION_V6 = ( FWP_IP_VERSION_V4 + 1 ),
    FWP_IP_VERSION_NONE = ( FWP_IP_VERSION_V6 + 1 ),
    FWP_IP_VERSION_MAX = ( FWP_IP_VERSION_NONE + 1 ),
}}

ENUM!{enum FWP_NE_FAMILY {
    FWP_AF_INET = FWP_IP_VERSION_V4,
    FWP_AF_INET6 = FWP_IP_VERSION_V6,
    FWP_AF_ETHER = FWP_IP_VERSION_NONE,
    FWP_AF_NONE = ( FWP_AF_ETHER + 1 ),
}}

ENUM!{enum FWP_ETHER_ENCAP_METHOD {
    FWP_ETHER_ENCAP_METHOD_ETHER_V2 = 0,
    FWP_ETHER_ENCAP_METHOD_SNAP = 1,
    FWP_ETHER_ENCAP_METHOD_SNAP_W_OUI_ZERO = 3,
}}

ENUM!{enum FWP_DATA_TYPE {
    FWP_EMPTY = 0,
    FWP_UINT8 = ( FWP_EMPTY + 1 ),
    FWP_UINT16 = ( FWP_UINT8 + 1 ),
    FWP_UINT32 = ( FWP_UINT16 + 1 ),
    FWP_UINT64 = ( FWP_UINT32 + 1 ),
    FWP_INT8 = ( FWP_UINT64 + 1 ),
    FWP_INT16 = ( FWP_INT8 + 1 ),
    FWP_INT32 = ( FWP_INT16 + 1 ),
    FWP_INT64 = ( FWP_INT32 + 1 ),
    FWP_FLOAT = ( FWP_INT64 + 1 ),
    FWP_DOUBLE = ( FWP_FLOAT + 1 ),
    FWP_BYTE_ARRAY16_TYPE = ( FWP_DOUBLE + 1 ),
    FWP_BYTE_BLOB_TYPE = ( FWP_BYTE_ARRAY16_TYPE + 1 ),
    FWP_SID = ( FWP_BYTE_BLOB_TYPE + 1 ),
    FWP_SECURITY_DESCRIPTOR_TYPE = ( FWP_SID + 1 ),
    FWP_TOKEN_INFORMATION_TYPE = ( FWP_SECURITY_DESCRIPTOR_TYPE + 1 ),
    FWP_TOKEN_ACCESS_INFORMATION_TYPE = ( FWP_TOKEN_INFORMATION_TYPE + 1 ),
    FWP_UNICODE_STRING_TYPE = ( FWP_TOKEN_ACCESS_INFORMATION_TYPE + 1 ),
    FWP_BYTE_ARRAY6_TYPE = ( FWP_UNICODE_STRING_TYPE + 1 ),
    FWP_BITMAP_INDEX_TYPE = ( FWP_BYTE_ARRAY6_TYPE + 1 ),
    FWP_BITMAP_ARRAY64_TYPE = ( FWP_BITMAP_INDEX_TYPE + 1 ),
    FWP_SINGLE_DATA_TYPE_MAX = 0xff,
    FWP_V4_ADDR_MASK = ( FWP_SINGLE_DATA_TYPE_MAX + 1 ),
    FWP_V6_ADDR_MASK = ( FWP_V4_ADDR_MASK + 1 ),
    FWP_RANGE_TYPE = ( FWP_V6_ADDR_MASK + 1 ),
    FWP_DATA_TYPE_MAX = ( FWP_RANGE_TYPE + 1 ),
}}

STRUCT!{struct FWP_BITMAP_ARRAY64 {
    bitmapArray64: [UINT8; 8],
}}

pub const FWP_BYTEMAP_ARRAY64_SIZE: usize = 8;
pub const FWP_BITMAP_ARRAY64_SIZE: usize = 64;

STRUCT!{struct FWP_BYTE_ARRAY6 {
    byteArray6: [UINT8; 6],
}}

pub const FWP_BYTE_ARRAY6_SIZE: usize = 6;

STRUCT!{struct FWP_BYTE_ARRAY16 {
    byteArray16: [UINT8; 16],
}}

STRUCT!{struct FWP_BYTE_BLOB {
    size: UINT32,
    data: PUINT8,
}}

STRUCT!{struct FWP_TOKEN_INFORMATION {
    sidCount: ULONG,
    sids: PSID_AND_ATTRIBUTES,
    restrictedSidCount: ULONG,
    restrictedSids: PSID_AND_ATTRIBUTES,
}}

UNION2!{union FWP_VALUE0_u {
    [u32; 1] [u64; 1],
    uint8 uint8_mut: UINT8,
    uint16 uint16_mut: UINT16,
    uint32 uint32_mut: UINT32,
    uint64 uint64_mut: PUINT64,
    int8 int8_mut: INT8,
    int16 int16_mut: INT16,
    int32 int32_mut: INT32,
    int64 int64_mut: PINT64,
    float32 float32_mut: c_float,
    double64 double64_mut: *mut c_double,
    byteArray16 byteArray16_mut: *mut FWP_BYTE_ARRAY16,
    byteBlob byteBlob_mut: *mut FWP_BYTE_BLOB,
    sid sid_mut: PSID,
    sd sd_mut: *mut FWP_BYTE_BLOB,
    tokenInformation tokenInformation_mut: *mut FWP_TOKEN_INFORMATION,
    tokenAccessInformation tokenAccessInformation_mut: *mut FWP_BYTE_BLOB,
    unicodeString unicodeString_mut: LPWSTR,
    byteArray6 byteArray6_mut: *mut FWP_BYTE_ARRAY6,
    bitmapArray64 bitmapArray64_mut: *mut FWP_BITMAP_ARRAY64,
}}

STRUCT!{struct FWP_VALUE0 {
    type_: FWP_DATA_TYPE,
	u: FWP_VALUE0_u,
}}


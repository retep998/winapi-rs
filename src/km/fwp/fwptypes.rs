use ::ctypes::*;
pub const FWP_ACTION_FLAG_CALLOUT: u32 = 0x00004000;
pub const FWP_ACTION_FLAG_TERMINATING: u32 = 0x00001000;
pub const FWP_ACTION_CALLOUT_TERMINATING: u32 =
    0x00000003 | FWP_ACTION_FLAG_CALLOUT | FWP_ACTION_FLAG_TERMINATING;

STRUCT! {
    struct FWP_VALUE0 {
        r#type:FWP_DATA_TYPE,
        u:FWP_VALUE0_u,
    }
}

UNION! {union FWP_VALUE0_u {
    [u32; 1] [u64; 1],
    uint8 uint8_mut :u8, //1
    uint16 uint16_mut :u16,//2
    uint32 uint32_mut:u32,//4
    uint64 uint64_mut:* const u64,//8
}}


ENUM! {
enum FWP_DATA_TYPE {
        FWP_EMPTY	= 0,
        FWP_UINT8	= ( FWP_EMPTY + 1 ) ,
        FWP_UINT16	= ( FWP_UINT8 + 1 ) ,
        FWP_UINT32	= ( FWP_UINT16 + 1 ) ,
        FWP_UINT64	= ( FWP_UINT32 + 1 ) ,
        FWP_INT8	= ( FWP_UINT64 + 1 ) ,
        FWP_INT16	= ( FWP_INT8 + 1 ) ,
        FWP_INT32	= ( FWP_INT16 + 1 ) ,
        FWP_INT64	= ( FWP_INT32 + 1 ) ,
        FWP_FLOAT	= ( FWP_INT64 + 1 ) ,   
        FWP_DOUBLE	= ( FWP_FLOAT + 1 ) ,
        FWP_BYTE_ARRAY16_TYPE	= ( FWP_DOUBLE + 1 ) ,
        FWP_BYTE_BLOB_TYPE	= ( FWP_BYTE_ARRAY16_TYPE + 1 ) ,
        FWP_SID	= ( FWP_BYTE_BLOB_TYPE + 1 ) ,
        FWP_SECURITY_DESCRIPTOR_TYPE	= ( FWP_SID + 1 ) ,
        FWP_TOKEN_INFORMATION_TYPE	= ( FWP_SECURITY_DESCRIPTOR_TYPE + 1 ) ,
        FWP_TOKEN_ACCESS_INFORMATION_TYPE	= ( FWP_TOKEN_INFORMATION_TYPE + 1 ) ,
        FWP_UNICODE_STRING_TYPE	= ( FWP_TOKEN_ACCESS_INFORMATION_TYPE + 1 ) ,
        FWP_BYTE_ARRAY6_TYPE	= ( FWP_UNICODE_STRING_TYPE + 1 ) ,
        FWP_BITMAP_INDEX_TYPE	= ( FWP_BYTE_ARRAY6_TYPE + 1 ) ,
        FWP_BITMAP_ARRAY64_TYPE	= ( FWP_BITMAP_INDEX_TYPE + 1 ) ,
        FWP_SINGLE_DATA_TYPE_MAX	= 0xff,
        FWP_V4_ADDR_MASK	= ( FWP_SINGLE_DATA_TYPE_MAX + 1 ) ,
        FWP_V6_ADDR_MASK	= ( FWP_V4_ADDR_MASK + 1 ) ,
        FWP_RANGE_TYPE	= ( FWP_V6_ADDR_MASK + 1 ) ,
        FWP_DATA_TYPE_MAX	= ( FWP_RANGE_TYPE + 1 ) ,
}
}

STRUCT! {
    struct FWP_BYTE_BLOB {
        size:u32,
        data:*const u8,
    }
}

STRUCT! {
    struct FWPS_INBOUND_FRAGMENT_METADATA0 {
   fragmentIdentification: u32 ,
    fragmentOffset:u16 ,
   fragmentLength: c_ulong,
    }
}
ENUM! {
enum FWP_DIRECTION {
        FWP_DIRECTION_OUTBOUND	= 0,
        FWP_DIRECTION_INBOUND	= ( FWP_DIRECTION_OUTBOUND + 1 ) ,
        FWP_DIRECTION_MAX	= ( FWP_DIRECTION_INBOUND + 1 ) ,
    }
}
pub type FWP_ACTION_TYPE = u32;

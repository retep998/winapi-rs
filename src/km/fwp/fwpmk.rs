use super::inner_prelude::*;

pub const RPC_C_AUTHN_WINNT: u32 = 10;

DEFINE_GUID! {
FWPM_LAYER_STREAM_V4,
0x3b89653c,
0xc170,
0x49e4,
0xb1, 0xcd, 0xe0, 0xee, 0xee, 0xe1, 0x9a, 0x3e
}



DEFINE_GUID! {
FWPM_CONDITION_IP_LOCAL_PORT,
0x0c1ba1af,
0x5765,
0x453f,
0xaf, 0x22, 0xa8, 0xf7, 0x91, 0xac, 0x77, 0x5b
}

// 1247d66d-0b60-4a15-8d44-7155d0f53a0c
DEFINE_GUID! {
FWPM_LAYER_ALE_RESOURCE_ASSIGNMENT_V4,
0x1247d66d,
0x0b60,
0x4a15,
0x8d, 0x44, 0x71, 0x55, 0xd0, 0xf5, 0x3a, 0x0c
}

// c38d57d1-05a7-4c33-904f-7fbceee60e82
DEFINE_GUID! {
FWPM_LAYER_ALE_AUTH_CONNECT_V4,
0xc38d57d1,
0x05a7,
0x4c33,
0x90, 0x4f, 0x7f, 0xbc, 0xee, 0xe6, 0x0e, 0x82
}

// 4a72393b-319f-44bc-84c3-ba54dcb3b6b4
DEFINE_GUID! {
FWPM_LAYER_ALE_AUTH_CONNECT_V6,
0x4a72393b,
0x319f,
0x44bc,
0x84, 0xc3, 0xba, 0x54, 0xdc, 0xb3, 0xb6, 0xb4
}
// e1cd9fe7-f4b5-4273-96c0-592e487b8650
DEFINE_GUID! {
FWPM_LAYER_ALE_AUTH_RECV_ACCEPT_V4,
0xe1cd9fe7,
0xf4b5,
0x4273,
0x96, 0xc0, 0x59, 0x2e, 0x48, 0x7b, 0x86, 0x50
}

// a3b42c97-9f04-4672-b87e-cee9c483257f
DEFINE_GUID! {
FWPM_LAYER_ALE_AUTH_RECV_ACCEPT_V6,
0xa3b42c97,
0x9f04,
0x4672,
0xb8, 0x7e, 0xce, 0xe9, 0xc4, 0x83, 0x25, 0x7f
}



STRUCT! {
    struct FWPM_DISPLAY_DATA0 {
        name:*const u16,
        description:*const u16,
    }
}

STRUCT! {
    struct FWPM_CALLOUT0 {
        calloutKey:GUID,
        displayData:FWPM_DISPLAY_DATA0,
        flags:u32,
        providerKey:*const GUID,
        providerData:FWP_BYTE_BLOB,
        applicableLayer:GUID,
        calloutId:u32,
    }
}
STRUCT! {
    struct FWPM_SUBLAYER0 {
        subLayerKey:GUID,
        displayData:FWPM_DISPLAY_DATA0,
        flags:u32,
        providerKey:*const GUID,
        providerData:FWP_BYTE_BLOB,
        weight:u16,
    }
}


STRUCT! {
    struct FWPM_FILTER_CONDITION0 {
        fieldKey:GUID,
        matchType:FWP_MATCH_TYPE,
        conditionValue:FWP_CONDITION_VALUE0,
    }
}

UNION! {union FWPM_ACTION0_u {
    [u8;16],
    filterType filterType_mut:GUID,
    calloutKey calloutKey_mut:GUID,
    bitmapIndex bitmapIndex_mut:u8,
}}

STRUCT! {
    struct FWPM_ACTION0 {
        r#type:FWP_ACTION_TYPE,
        u:FWPM_ACTION0_u,
    }
}
STRUCT! {
    struct FWPM_FILTER_CONDITION0_ {
        fieldKey:GUID,
        matchType:FWP_MATCH_TYPE,
        conditionValue:FWP_CONDITION_VALUE0,
    }
}

STRUCT! {
    struct FWP_CONDITION_VALUE0 {
        r#type:FWP_DATA_TYPE,
        u:   FWP_CONDITION_VALUE0_u,
    }
}

UNION! {union FWP_CONDITION_VALUE0_u {
    [u32;1] [u64;1],
    uint8 uint8_mut:u8,
    uint16 uint16_mut:u16,
    uint64 uint64_mut:*mut u64,
}}

ENUM! {
enum FWP_MATCH_TYPE
{
    FWP_MATCH_EQUAL = 0,
    FWP_MATCH_GREATER	= ( FWP_MATCH_EQUAL + 1 ) ,
    FWP_MATCH_LESS	= ( FWP_MATCH_GREATER + 1 ) ,
    FWP_MATCH_GREATER_OR_EQUAL	= ( FWP_MATCH_LESS + 1 ) ,
    FWP_MATCH_LESS_OR_EQUAL	= ( FWP_MATCH_GREATER_OR_EQUAL + 1 ) ,
    FWP_MATCH_RANGE	= ( FWP_MATCH_LESS_OR_EQUAL + 1 ) ,
    FWP_MATCH_FLAGS_ALL_SET	= ( FWP_MATCH_RANGE + 1 ) ,
    FWP_MATCH_FLAGS_ANY_SET	= ( FWP_MATCH_FLAGS_ALL_SET + 1 ) ,
    FWP_MATCH_FLAGS_NONE_SET	= ( FWP_MATCH_FLAGS_ANY_SET + 1 ) ,
    FWP_MATCH_EQUAL_CASE_INSENSITIVE	= ( FWP_MATCH_FLAGS_NONE_SET + 1 ) ,
    FWP_MATCH_NOT_EQUAL	= ( FWP_MATCH_EQUAL_CASE_INSENSITIVE + 1 ) ,
    FWP_MATCH_PREFIX	= ( FWP_MATCH_NOT_EQUAL + 1 ) ,
    FWP_MATCH_NOT_PREFIX	= ( FWP_MATCH_PREFIX + 1 ) ,
    FWP_MATCH_TYPE_MAX	= ( FWP_MATCH_NOT_PREFIX + 1 ),
}
}

UNION! {union FWPM_FILTER0_u {
    [u64;2],
    rawContext rawContext_mut:u64,
    providerContextKey providerContextKey_mut:GUID ,
}}

STRUCT! {
    struct FWPM_FILTER0 {
        filterKey:GUID,
        displayData:FWPM_DISPLAY_DATA0,
        flags:u32,
        providerKey:*const GUID,
        providerData:FWP_BYTE_BLOB,
        layerKey:GUID,
        subLayerKey:GUID,
        weight:FWP_VALUE0,
        numFilterConditions:u32,
        filterCondition:*const FWPM_FILTER_CONDITION0,
        action:FWPM_ACTION0,
        u:FWPM_FILTER0_u,
        reserved:*const GUID,
        filterId:u64,
        effectiveWeight:FWP_VALUE0,
    }
}

extern "system" {
    pub fn FwpmEngineOpen0(
        serverName: PCVOID, //must be null
        authnService: u32,
        authIdentity: PCVOID,
        session: PCVOID,
        engineHandle: *mut HANDLE,
    ) -> NTSTATUS;
    pub fn FwpmCalloutAdd0(
        engineHandle: HANDLE,
        callout: *const FWPM_CALLOUT0,
        sd: MISS_TYPE_PTR,
        id: *mut u32,
    ) -> NTSTATUS;
    pub fn FwpmSubLayerAdd0(
        engineHandle: HANDLE,
        subLayer: *const FWPM_SUBLAYER0,
        sd: MISS_TYPE_PTR,
    ) -> NTSTATUS;
    pub fn FwpmFilterAdd0(
        engineHandle: HANDLE,
        filter: *const FWPM_FILTER0,
        sd: MISS_TYPE_PTR,
        id: *mut u64,
    ) -> NTSTATUS;
    pub fn FwpmFilterDeleteById0(engineHandle: HANDLE, id: u64) -> NTSTATUS;
    pub fn FwpmSubLayerDeleteByKey0(engineHandle: HANDLE, key: *const GUID) -> NTSTATUS;
    pub fn FwpmCalloutDeleteById0(engineHandle: HANDLE, id: u32) -> NTSTATUS;
    pub fn FwpsCalloutUnregisterById0(calloutId: u32) -> NTSTATUS;
    pub fn FwpmEngineClose0(engineHandle: HANDLE) -> NTSTATUS;
}

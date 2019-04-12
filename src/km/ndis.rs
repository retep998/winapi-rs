use shared::ntdef::{ULONG,PVOID};
use shared::minwindef::*;
use shared::basetsd::SIZE_T;

//FIXME check on 32
STRUCT! {struct NET_BUFFER_LIST {
  u1:NET_BUFFER_LIST_u1,//16 [u8;16]
  _fill_size:[u8;224],
}}

pub type PNET_BUFFER_LIST =*mut NET_BUFFER_LIST;

UNION! {union NET_BUFFER_LIST_u1 {
   [u32;4] [u64;2],//FIXME check on 32
   s s_mut : NET_BUFFER_LIST_u1_s,
   //Link Link_mut :SLIST_HEADER ,           // used in SLIST of free NetBuffers in the block
   //NetBufferListHeader NetBufferListHeader_mut: NET_BUFFER_LIST_HEADER,
}}

STRUCT! {struct NET_BUFFER_LIST_u1_s{
    Next: PNET_BUFFER_LIST ,           // Next NetBufferList in the chain
    FirstNetBuffer:PNET_BUFFER , // First NetBuffer on this NetBufferList
}}

STRUCT!{struct NET_BUFFER{
    u1:NET_BUFFER_u1,
    _fill_size:[u8;128],//FIXME
}}
pub type PNET_BUFFER =*mut NET_BUFFER;


UNION!{union NET_BUFFER_u1 {
    [u32;12] [u64;6],
    s s_mut:NET_BUFFER_u1_s,
    //Link Link_mut: SLIST_HEADER,
    //NetB  ufferHeader NetBufferHeader_mut: NET_BUFFER_HEADER ,
}}

pub type PMDL = PVOID;//FIXME

STRUCT!{struct NET_BUFFER_u1_s{
    Next :PNET_BUFFER,//8
    CurrentMdl :PMDL,//8
    CurrentMdlOffset  :ULONG,//4
    u:NET_BUFFER_u1_s_u,//8
    MdlChain:PMDL,//8
    DataOffset:ULONG,//4
}}

UNION!{union NET_BUFFER_u1_s_u {
    [u32;2] [u64;1],
    DataLength DataLength_mut:ULONG,
    stDataLength stDataLength_mut:SIZE_T,
}}

#[link(name = "Ndis")]
extern "system" {
    pub fn NdisGetDataBuffer(
        deviceObject: PNET_BUFFER,
        BytesNeeded: ULONG,
        Storage: PVOID,
        AlignMultiple:UINT,
        AlignOffset:UINT,
    ) -> PVOID ;
}

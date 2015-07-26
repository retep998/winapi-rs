// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! IPv4 Internet address
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct in_addr_S_un_b {
    pub s_b1: ::UCHAR,
    pub s_b2: ::UCHAR,
    pub s_b3: ::UCHAR,
    pub s_b4: ::UCHAR,
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct in_addr_S_un_w {
    pub s_w1: ::USHORT,
    pub s_w2: ::USHORT,
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct in_addr {
    pub S_un: ::ULONG,
}
UNION!(in_addr, S_un, S_un_b, S_un_b_mut, in_addr_S_un_b);
UNION!(in_addr, S_un, S_un_w, S_un_w_mut, in_addr_S_un_w);
UNION!(in_addr, S_un, S_addr, S_addr_mut, ::ULONG);
pub type IN_ADDR = in_addr;
pub type PIN_ADDR = *mut in_addr;
pub type LPIN_ADDR = *mut in_addr;

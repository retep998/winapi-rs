// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! definitions to be used with the WinSock 2 DLL and WinSock 2 applications.
//!
//! This header file corresponds to version 2.2.x of the WinSock API specification.
pub const WINSOCK_VERSION: ::WORD = 2 | (2 << 8);
pub type u_char = ::c_uchar;
pub type u_short = ::c_ushort;
pub type u_int = ::c_uint;
pub type u_long = ::c_ulong;
pub type u_int64 = ::__uint64;
pub type SOCKET = ::UINT_PTR;
pub const FD_SETSIZE: usize = 64;
#[repr(C)] #[derive(Copy)]
pub struct fd_set {
    pub fd_count: u_int,
    pub fd_array: [SOCKET; FD_SETSIZE],
}
impl Clone for fd_set {
    fn clone(&self) -> fd_set { *self }
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct timeval {
    pub tv_sec: ::c_long,
    pub tv_usec: ::c_long,
}

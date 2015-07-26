// Copyright © 2015, skdltmxn
// Licensed under the MIT License <LICENSE.md>
//! WinSock2 Extension for TCP/IP protocols
pub type LPLOOKUPSERVICE_COMPLETION_ROUTINE = Option<unsafe extern "system" fn(
    dwError: ::DWORD, dwBytes: ::DWORD, lpOverlapped: ::LPWSAOVERLAPPED,
)>;

pub type socklen_t = ::c_int;

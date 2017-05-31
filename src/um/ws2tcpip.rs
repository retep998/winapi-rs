// Copyright © 2015-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! WinSock2 Extension for TCP/IP protocols

use ctypes::c_int;
use shared::minwindef::DWORD;
use um::winsock2::LPWSAOVERLAPPED;

FN!{stdcall LPLOOKUPSERVICE_COMPLETION_ROUTINE(
    dwError: DWORD,
    dwBytes: DWORD,
    lpOverlapped: LPWSAOVERLAPPED,
) -> ()}
pub type socklen_t = c_int;

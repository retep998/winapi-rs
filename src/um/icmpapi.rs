// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::minwindef::{BOOL, DWORD, FARPROC, LPVOID, WORD};
use shared::ntdef::{HANDLE, PVOID};
use shared::ws2ipdef::PSOCKADDR_IN6_LH;
use um::ipexport::IPAddr;
#[cfg(target_pointer_width = "32")]
use um::ipexport::PIP_OPTION_INFORMATION;
#[cfg(target_pointer_width = "64")]
use um::ipexport::PIP_OPTION_INFORMATION32 as PIP_OPTION_INFORMATION;
extern "system" {
    pub fn IcmpCreateFile() -> HANDLE;
    pub fn Icmp6CreateFile() -> HANDLE;
    pub fn IcmpCloseHandle(IcmpHandle: HANDLE) -> BOOL;
    pub fn IcmpSendEcho(
        IcmpHandle: HANDLE,
        DestinationAddress: IPAddr,
        RequestData: LPVOID,
        RequestSize: WORD,
        RequestOptions: PIP_OPTION_INFORMATION,
        ReplyBuffer: LPVOID,
        ReplySize: DWORD,
        Timeout: DWORD
    ) -> DWORD;
    pub fn IcmpSendEcho2(
        IcmpHandle: HANDLE,
        Event: HANDLE,
        ApcRoutine: FARPROC, // or PIO_APC_ROUTINE
        ApcContext: PVOID,
        DestinationAddress: IPAddr,
        RequestData: LPVOID,
        RequestSize: WORD,
        RequestOptions: PIP_OPTION_INFORMATION,
        ReplyBuffer: LPVOID,
        ReplySize: DWORD,
        Timeout: DWORD
    ) -> DWORD;
    pub fn IcmpSendEcho2Ex(
        IcmpHandle: HANDLE,
        Event: HANDLE,
        ApcRoutine: FARPROC, // or PIO_APC_ROUTINE
        ApcContext: PVOID,
        SourceAddress: IPAddr,
        DestinationAddress: IPAddr,
        RequestData: LPVOID,
        RequestSize: WORD,
        RequestOptions: PIP_OPTION_INFORMATION,
        ReplyBuffer: LPVOID,
        ReplySize: DWORD,
        Timeout: DWORD
    ) -> DWORD;
    pub fn Icmp6SendEcho2(
        IcmpHandle: HANDLE,
        Event: HANDLE,
        ApcRoutine: FARPROC, // or PIO_APC_ROUTINE
        ApcContext: PVOID,
        SourceAddress: PSOCKADDR_IN6_LH,
        DestinationAddress: PSOCKADDR_IN6_LH,
        RequestData: LPVOID,
        RequestSize: WORD,
        RequestOptions: PIP_OPTION_INFORMATION,
        ReplyBuffer: LPVOID,
        ReplySize: DWORD,
        Timeout: DWORD
    ) -> DWORD;
    pub fn IcmpParseReplies(
        ReplyBuffer: LPVOID,
        ReplySize: DWORD,
    ) -> DWORD;
    pub fn Icmp6ParseReplies(
        ReplyBuffer: LPVOID,
        ReplySize: DWORD
    ) -> DWORD;
}
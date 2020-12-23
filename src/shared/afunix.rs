use shared::minwindef::DWORD;
use shared::ws2def::{ADDRESS_FAMILY, IOC_VENDOR};
use um::winnt::CHAR;
pub const UNIX_PATH_MAX: usize = 108;
STRUCT!{struct SOCKADDR_UN {
    sun_family: ADDRESS_FAMILY,
    sun_path: [CHAR; UNIX_PATH_MAX],
}}
pub type PSOCKETADDR_UN = *mut SOCKADDR_UN;
pub const SIO_AF_UNIX_GETPEERPID: DWORD = _WSAIOR!(IOC_VENDOR, 256);

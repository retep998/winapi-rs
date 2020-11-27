use shared::minwindef::DWORD;
pub const FILE_VER_GET_LOCALISED: DWORD = 0x01;
pub const FILE_VER_GET_NEUTRAL: DWORD = 0x02;
pub const FILE_VER_GET_PREFETCHED: DWORD = 0x04;
STRUCT!{struct VS_FIXEDFILEINFO {
    dwSignature: DWORD,
    dwStrucVersion: DWORD,
    dwFileVersionMS: DWORD,
    dwFileVersionLS: DWORD,
    dwProductVersionMS: DWORD,
    dwProductVersionLS: DWORD,
    dwFileFlagsMask: DWORD,
    dwFileFlags: DWORD,
    dwFileOS: DWORD,
    dwFileType: DWORD,
    dwFileSubtype: DWORD,
    dwFileDateMS: DWORD,
    dwFileDateLS: DWORD,
}}

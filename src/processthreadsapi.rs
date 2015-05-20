#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct STARTUPINFOA {
    pub cb: ::DWORD,
    pub lpReserved: ::LPSTR,
    pub lpDesktop: ::LPSTR,
    pub lpTitle: ::LPSTR,
    pub dwX: ::DWORD,
    pub dwY: ::DWORD,
    pub dwXSize: ::DWORD,
    pub dwYSize: ::DWORD,
    pub dwXCountChars: ::DWORD,
    pub dwYCountChars: ::DWORD,
    pub dwFillAttribute: ::DWORD,
    pub dwFlags: ::DWORD,
    pub wShowWindow: ::WORD,
    pub cbReserved2: ::WORD,
    pub lpReserved2: ::LPBYTE,
    pub hStdInput: ::HANDLE,
    pub hStdOutput: ::HANDLE,
    pub hStdError: ::HANDLE,
}
pub type LPSTARTUPINFOA = *mut STARTUPINFOA;
impl ::std::default::Default for STARTUPINFOA {
    fn default() -> STARTUPINFOA {
        STARTUPINFOA {
            cb: 0, lpReserved: ::std::ptr::null_mut(), lpDesktop: ::std::ptr::null_mut(),
            lpTitle: ::std::ptr::null_mut(), dwX: 0, dwY: 0, dwXSize: 0, dwYSize: 0,
            dwXCountChars: 0, dwYCountChars: 0, dwFillAttribute: 0, dwFlags: 0, wShowWindow: 0,
            cbReserved2: 0, lpReserved2: ::std::ptr::null_mut(),
            hStdInput: ::std::ptr::null_mut(), hStdOutput: ::std::ptr::null_mut(),
            hStdError: ::std::ptr::null_mut(),
        }
    }
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct STARTUPINFOW {
    pub cb: ::DWORD,
    pub lpReserved: ::LPWSTR,
    pub lpDesktop: ::LPWSTR,
    pub lpTitle: ::LPWSTR,
    pub dwX: ::DWORD,
    pub dwY: ::DWORD,
    pub dwXSize: ::DWORD,
    pub dwYSize: ::DWORD,
    pub dwXCountChars: ::DWORD,
    pub dwYCountChars: ::DWORD,
    pub dwFillAttribute: ::DWORD,
    pub dwFlags: ::DWORD,
    pub wShowWindow: ::WORD,
    pub cbReserved2: ::WORD,
    pub lpReserved2: ::LPBYTE,
    pub hStdInput: ::HANDLE,
    pub hStdOutput: ::HANDLE,
    pub hStdError: ::HANDLE,
}
impl ::std::default::Default for STARTUPINFOW {
    fn default() -> STARTUPINFOW {
        STARTUPINFOW {
            cb: 0, lpReserved: ::std::ptr::null_mut(), lpDesktop: ::std::ptr::null_mut(),
            lpTitle: ::std::ptr::null_mut(), dwX: 0, dwY: 0, dwXSize: 0, dwYSize: 0,
            dwXCountChars: 0, dwYCountChars: 0, dwFillAttribute: 0, dwFlags: 0, wShowWindow: 0,
            cbReserved2: 0, lpReserved2: ::std::ptr::null_mut(),
            hStdInput: ::std::ptr::null_mut(), hStdOutput: ::std::ptr::null_mut(),
            hStdError: ::std::ptr::null_mut(),
        }
    }
}
pub type LPSTARTUPINFOW = *mut STARTUPINFOW;

// Copyright © 2015, Gigih Aji Ibrahim
// Licensed under the MIT License <LICENSE.md>
pub const MAX_MODULE_NAME32: usize = 255;
pub const TH32CS_SNAPHEAPLIST: ::DWORD = 0x00000001;
pub const TH32CS_SNAPPROCESS: ::DWORD = 0x00000002;
pub const TH32CS_SNAPTHREAD: ::DWORD = 0x00000004;
pub const TH32CS_SNAPMODULE: ::DWORD = 0x00000008;
pub const TH32CS_SNAPMODULE32: ::DWORD = 0x00000010;
pub const TH32CS_SNAPALL: ::DWORD =
    (TH32CS_SNAPHEAPLIST | TH32CS_SNAPPROCESS | TH32CS_SNAPTHREAD | TH32CS_SNAPMODULE);
pub const TH32CS_INHERIT: ::DWORD = 0x80000000;
STRUCT!{struct HEAPLIST32 {
    dwSize: ::SIZE_T,
    th32ProcessID: ::DWORD,
    th32HeapID: :: ULONG_PTR,
    dwFlags: ::DWORD,
}}
pub type PHEAPLIST32 = *mut HEAPLIST32;
pub type LPHEAPLIST32 = *mut HEAPLIST32;
pub const HF32_DEFAULT: ::DWORD = 1;
pub const HF32_SHARED: ::DWORD = 2;
STRUCT!{struct HEAPENTRY32 {
    dwSize: ::SIZE_T,
    hHandle: ::HANDLE,
    dwAddress: ::ULONG_PTR,
    dwBlockSize: ::SIZE_T,
    dwFlags: ::DWORD,
    dwLockCount: ::DWORD,
    dwResvd: ::DWORD,
    th32ProcessID: ::DWORD,
    th32HeapID: ::ULONG_PTR,
}}
pub type PHEAPENTRY32 = *mut HEAPENTRY32;
pub type LPHEAPENTRY32 = *mut HEAPENTRY32;
pub const LF32_FIXED: ::DWORD = 0x00000001;
pub const LF32_FREE: ::DWORD = 0x00000002;
pub const LF32_MOVEABLE: ::DWORD = 0x00000004;
#[repr(C)] #[derive(Copy)]
pub struct PROCESSENTRY32W {
    pub dwSize: ::DWORD,
    pub cntUsage: ::DWORD,
    pub th32ProcessID: ::DWORD,
    pub th32DefaultHeapID: ::ULONG_PTR,
    pub th32ModuleID: ::DWORD,
    pub cntThreads: ::DWORD,
    pub th32ParentProcessID: ::DWORD,
    pub pcPriClassBase: ::LONG,
    pub dwFlags: ::DWORD,
    pub szExeFile: [::WCHAR; ::MAX_PATH],
}
impl Clone for PROCESSENTRY32W{ fn clone(&self) -> PROCESSENTRY32W { *self } }
pub type PPROCESSENTRY32W = *mut PROCESSENTRY32W;
pub type LPPROCESSENTRY32W = *mut PROCESSENTRY32W;
#[repr(C)] #[derive(Copy)]
pub struct PROCESSENTRY32 {
    pub dwSize: ::DWORD,
    pub cntUsage: ::DWORD,
    pub th32ProcessID: ::DWORD,
    pub th32DefaultHeapID: ::ULONG_PTR,
    pub th32ModuleID: ::DWORD,
    pub cntThreads: ::DWORD,
    pub th32ParentProcessID: ::DWORD,
    pub pcPriClassBase: ::LONG,
    pub dwFlags: ::DWORD,
    pub szExeFile: [::CHAR; ::MAX_PATH],
}
impl Clone for PROCESSENTRY32{ fn clone(&self) -> PROCESSENTRY32 { *self } }
pub type PPROCESSENTRY32 = *mut PROCESSENTRY32;
pub type LPPROCESSENTRY32 = *mut PROCESSENTRY32;
STRUCT!{struct THREADENTRY32 {
    dwSize: ::DWORD,
    cntUsage: ::DWORD,
    th32ThreadID: ::DWORD,
    th32OwnerProcessID: ::DWORD,
    tpBasePri: ::LONG,
    tpDeltaPri: ::LONG,
    dwFlags: ::DWORD,
}}
pub type PTHREADENTRY32 = *mut THREADENTRY32;
pub type LPTHREADENTRY32 = *mut THREADENTRY32;
#[repr(C)] #[derive(Copy)]
pub struct MODULEENTRY32W {
    pub dwSize: ::DWORD,
    pub th32ModuleID: ::DWORD,
    pub th32ProcessID: ::DWORD,
    pub GlblcntUsage: ::DWORD,
    pub ProccntUsage: ::DWORD,
    pub modBaseAddr: *mut ::BYTE,
    pub modBaseSize: ::DWORD,
    pub hModule: ::HMODULE,
    pub szModule: [::WCHAR; ::MAX_MODULE_NAME32 + 1],
    pub szExePath: [::WCHAR; ::MAX_PATH],
}
impl Clone for MODULEENTRY32W{ fn clone(&self) -> MODULEENTRY32W { *self } }
pub type PMODULEENTRY32W = *mut MODULEENTRY32W;
pub type LPMODULEENTRY32W = *mut MODULEENTRY32W;
#[repr(C)] #[derive(Copy)]
pub struct MODULEENTRY32 {
    pub dwSize: ::DWORD,
    pub th32ModuleID: ::DWORD,
    pub th32ProcessID: ::DWORD,
    pub GlblcntUsage: ::DWORD,
    pub ProccntUsage: ::DWORD,
    pub modBaseAddr: *mut ::BYTE,
    pub modBaseSize: ::DWORD,
    pub hModule: ::HMODULE,
    pub szModule: [::CHAR; ::MAX_MODULE_NAME32 + 1],
    pub szExePath: [::CHAR; ::MAX_PATH],
}
impl Clone for MODULEENTRY32{ fn clone(&self) -> MODULEENTRY32 { *self } }
pub type PMODULEENTRY32 = *mut MODULEENTRY32;
pub type LPMODULEENTRY32 = *mut MODULEENTRY32;

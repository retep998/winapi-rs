// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! This module defines the 32-Bit Windows Base APIs
#[repr(C)] #[derive(Copy)] pub struct SECURITY_ATTRIBUTES {
    pub nLength: ::DWORD,
    pub lpSecurityDescriptor: ::LPVOID,
    pub bInheritHandle: ::BOOL,
}
pub type PSECURITY_ATTRIBUTES = *mut SECURITY_ATTRIBUTES;
pub type LPSECURITY_ATTRIBUTES = *mut SECURITY_ATTRIBUTES;
#[repr(C)] #[derive(Copy)] pub struct OVERLAPPED {
    pub Internal: ::ULONG_PTR,
    pub InternalHigh: ::ULONG_PTR,
    pub Offset: ::DWORD,
    pub OffsetHigh: ::DWORD,
    pub hEvent: ::HANDLE,
}
pub type LPOVERLAPPED = *mut OVERLAPPED;
#[repr(C)] #[derive(Copy)] pub struct OVERLAPPED_ENTRY {
    pub lpCompletionKey: ::ULONG_PTR,
    pub lpOverlapped: LPOVERLAPPED,
    pub Internal: ::ULONG_PTR,
    pub dwNumberOfBytesTransferred: ::DWORD,
}
pub type LPOVERLAPPED_ENTRY = *mut OVERLAPPED_ENTRY;
#[repr(C)] #[derive(Copy)] pub struct SYSTEMTIME {
    pub wYear: ::DWORD,
    pub wMonth: ::DWORD,
    pub wDayOfWeek: ::DWORD,
    pub wDay: ::DWORD,
    pub wHour: ::DWORD,
    pub wMinute: ::DWORD,
    pub wSecond: ::DWORD,
    pub wMilliseconds: ::DWORD,
}
pub type PSYSTEMTIME = *mut SYSTEMTIME;
pub type LPSYSTEMTIME = *mut SYSTEMTIME;
//206
pub type LPOVERLAPPED_COMPLETION_ROUTINE = unsafe extern "system" fn(
    dwErrorCode: ::DWORD, dwNumberOfBytesTransfered: ::DWORD, lpOverlapped: LPOVERLAPPED,
);

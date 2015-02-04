// Licensed under the MIT License <LICENSE.md>
// WinSvc.h
DECLARE_HANDLE!(SC_HANDLE, SC_HANDLE__);
DECLARE_HANDLE!(SERVICE_STATUS_HANDLE, SERVICE_STATUS_HANDLE__);

//
// Function Prototype for the Service Main Function
//
pub type LPSERVICE_MAIN_FUNCTIONW = unsafe extern "system" fn(
    dwNumServicesArgs: ::DWORD,
    lpServiceArgVectors: *mut ::LPWSTR,
);

pub type LPSERVICE_MAIN_FUNCTIONA = unsafe extern "system" fn(
    dwNumServicesArgs: ::DWORD,
    lpServiceArgVectors: *mut ::LPSTR,
);

//
// Prototype for the Service Control Handler Function
//

pub type LPHANDLER_FUNCTION = unsafe extern "system" fn(
    dwControl: SERVICE_CONTROL,
);

pub type LPHANDLER_FUNCTION_EX = unsafe extern "system" fn(
    dwControl: SERVICE_CONTROL,
    dwEventType: ::DWORD,
    lpEventData: ::LPVOID,
    lpContext: ::LPVOID,
) -> ::DWORD;
    
//
// Info levels for QueryServiceStatusEx
//
#[repr(i32)]
#[derive(Copy)]
pub enum SC_STATUS_TYPE {
    SC_STATUS_PROCESS_INFO = 0,
    // Fake element, because invariant enum is not supported.
    SC_STATUS_RESERVED = 1,
}

//
// Service Status Structures
//
#[repr(C)]
#[derive(Copy)]
pub struct SERVICE_STATUS {
    pub dwServiceType: ::DWORD,
    pub dwCurrentState: SERVICE_CURRENT_STATE,
    pub dwControlsAccepted: ::DWORD,
    pub dwWin32ExitCode: ::DWORD,
    pub dwServiceSpecificExitCode: ::DWORD,
    pub dwCheckPoint: ::DWORD,
    pub dwWaitHint: ::DWORD,
}

pub type LPSERVICE_STATUS = *mut SERVICE_STATUS;
pub type LPCSERVICE_STATUS = *const SERVICE_STATUS;

//
// Service Start Table
//
#[repr(C)]
#[derive(Copy)]
pub struct SERVICE_TABLE_ENTRYA {
    pub lpServiceName: ::LPCSTR,
    pub lpServiceProc: LPSERVICE_MAIN_FUNCTIONA,
}

pub type LPSERVICE_TABLE_ENTRYA = *mut SERVICE_TABLE_ENTRYA;
pub type LPCSERVICE_TABLE_ENTRYA = *const SERVICE_TABLE_ENTRYA;

#[repr(C)]
#[derive(Copy)]
pub struct SERVICE_TABLE_ENTRYW {
    pub lpServiceName: ::LPCWSTR,
    pub lpServiceProc: LPSERVICE_MAIN_FUNCTIONW,
}

pub type LPSERVICE_TABLE_ENTRYW = *mut SERVICE_TABLE_ENTRYW;
pub type LPCSERVICE_TABLE_ENTRYW = *const SERVICE_TABLE_ENTRYW;

//
// Controls
//
#[repr(i32)]
#[derive(Copy)]
pub enum SERVICE_CONTROL {
    STOP = 0x00000001,
    PAUSE = 0x00000002,
    CONTINUE = 0x00000003,
    INTERROGATE = 0x00000004,
    SHUTDOWN = 0x00000005,
    PARAMCHANGE = 0x00000006,
    NETBINDADD = 0x00000007,
    NETBINDREMOVE = 0x00000008,
    NETBINDENABLE = 0x00000009,
    NETBINDDISABLE = 0x0000000A,
    DEVICEEVENT = 0x0000000B,
    HARDWAREPROFILECHANGE = 0x0000000C,
    POWEREVENT = 0x0000000D,
    SESSIONCHANGE = 0x0000000E,
    PRESHUTDOWN = 0x0000000F,
    TIMECHANGE = 0x00000010,
    TRIGGEREVENT = 0x00000020,
}

//
// Service State -- for CurrentState
//
#[repr(i32)]
#[derive(Copy)]
pub enum SERVICE_CURRENT_STATE {
    STOPPED = 0x00000001,
    START_PENDING = 0x00000002,
    STOP_PENDING = 0x00000003,
    RUNNING = 0x00000004,
    CONTINUE_PENDING = 0x00000005,
    PAUSE_PENDING = 0x00000006,
    PAUSED = 0x00000007,
}

//
// Controls Accepted  (Bit Mask)
//
pub const SERVICE_ACCEPT_STOP: ::DWORD = 0x00000001;
pub const SERVICE_ACCEPT_PAUSE_CONTINUE: ::DWORD = 0x00000002;
pub const SERVICE_ACCEPT_SHUTDOWN: ::DWORD = 0x00000004;
pub const SERVICE_ACCEPT_PARAMCHANGE: ::DWORD = 0x00000008;
pub const SERVICE_ACCEPT_NETBINDCHANGE: ::DWORD = 0x00000010;
pub const SERVICE_ACCEPT_HARDWAREPROFILECHANGE: ::DWORD = 0x00000020;
pub const SERVICE_ACCEPT_POWEREVENT: ::DWORD = 0x00000040;
pub const SERVICE_ACCEPT_SESSIONCHANGE: ::DWORD = 0x00000080;
pub const SERVICE_ACCEPT_PRESHUTDOWN: ::DWORD = 0x00000100;
pub const SERVICE_ACCEPT_TIMECHANGE: ::DWORD = 0x00000200;
pub const SERVICE_ACCEPT_TRIGGEREVENT: ::DWORD = 0x00000400;

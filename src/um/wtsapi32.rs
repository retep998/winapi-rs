// Copyright © 2015-2019 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Windows Terminal Server public APIs
use shared::minwindef::{BOOL, BYTE, DWORD, LPDWORD, MAX_PATH, PBOOL, PULONG, ULONG, USHORT};
use shared::windef::HWND;
use um::winnt::{
    CHAR, HANDLE, LARGE_INTEGER, LONG, LPSTR, LPWSTR, PCHAR, PHANDLE, PSECURITY_DESCRIPTOR, PSID,
    PSTR, PVOID, PWSTR, SECURITY_INFORMATION, STANDARD_RIGHTS_REQUIRED, WCHAR,
};
pub const WTS_CURRENT_SERVER: HANDLE = 0isize as HANDLE;
pub const WTS_CURRENT_SERVER_HANDLE: HANDLE = 0isize as HANDLE;
pub const WTS_CURRENT_SESSION: DWORD = -1i32 as DWORD;
pub const WTS_ANY_SESSION: DWORD = -2i32 as DWORD;
const USERNAME_LENGTH: usize = 20;
const CLIENTNAME_LENGTH: usize = 20;
const CLIENTADDRESS_LENGTH: usize = 30;
pub const WTS_WSD_LOGOFF: DWORD = 0x00000001;
pub const WTS_WSD_SHUTDOWN: DWORD = 0x00000002;
pub const WTS_WSD_REBOOT: DWORD = 0x00000004;
pub const WTS_WSD_POWEROFF: DWORD = 0x00000008;
pub const WTS_WSD_FASTREBOOT: DWORD = 0x00000010;
const WINSTATIONNAME_LENGTH: usize = 32;
const DOMAIN_LENGTH: usize = 32;
const WTS_DRIVE_LENGTH: usize = 3;
const WTS_LISTENER_NAME_LENGTH: usize = 32;
const WTS_COMMENT_LENGTH: usize = 60;
pub const WTS_LISTENER_CREATE: DWORD = 0x00000001;
pub const WTS_LISTENER_UPDATE: DWORD = 0x00000010;
pub const WTS_SECURITY_QUERY_INFORMATION: DWORD = 0x00000001;
pub const WTS_SECURITY_SET_INFORMATION: DWORD = 0x00000002;
pub const WTS_SECURITY_RESET: DWORD = 0x00000004;
pub const WTS_SECURITY_VIRTUAL_CHANNELS: DWORD = 0x00000008;
pub const WTS_SECURITY_REMOTE_CONTROL: DWORD = 0x00000010;
pub const WTS_SECURITY_LOGON: DWORD = 0x00000020;
pub const WTS_SECURITY_LOGOFF: DWORD = 0x00000040;
pub const WTS_SECURITY_MESSAGE: DWORD = 0x00000080;
pub const WTS_SECURITY_CONNECT: DWORD = 0x00000100;
pub const WTS_SECURITY_DISCONNECT: DWORD = 0x00000200;
pub const WTS_SECURITY_GUEST_ACCESS: DWORD = WTS_SECURITY_LOGON;
pub const WTS_SECURITY_CURRENT_GUEST_ACCESS: DWORD =
    WTS_SECURITY_VIRTUAL_CHANNELS | WTS_SECURITY_LOGOFF;
pub const WTS_SECURITY_USER_ACCESS: DWORD =
    WTS_SECURITY_CURRENT_GUEST_ACCESS | WTS_SECURITY_QUERY_INFORMATION | WTS_SECURITY_CONNECT;
pub const WTS_SECURITY_CURRENT_USER_ACCESS: DWORD = WTS_SECURITY_SET_INFORMATION
    | WTS_SECURITY_RESET
    | WTS_SECURITY_VIRTUAL_CHANNELS
    | WTS_SECURITY_LOGOFF
    | WTS_SECURITY_DISCONNECT;
pub const WTS_SECURITY_ALL_ACCESS: DWORD = STANDARD_RIGHTS_REQUIRED
    | WTS_SECURITY_QUERY_INFORMATION
    | WTS_SECURITY_SET_INFORMATION
    | WTS_SECURITY_RESET
    | WTS_SECURITY_VIRTUAL_CHANNELS
    | WTS_SECURITY_REMOTE_CONTROL
    | WTS_SECURITY_LOGON
    | WTS_SECURITY_MESSAGE
    | WTS_SECURITY_CONNECT
    | WTS_SECURITY_DISCONNECT;
ENUM!{enum WTS_CONNECTSTATE_CLASS {
    WTSActive,
    WTSConnected,
    WTSConnectQuery,
    WTSShadow,
    WTSDisconnected,
    WTSIdle,
    WTSListen,
    WTSReset,
    WTSDown,
    WTSInit,
}}
STRUCT!{struct WTS_SERVER_INFOW {
    pServerName: LPWSTR,
}}
pub type PWTS_SERVER_INFOW = *mut WTS_SERVER_INFOW;
STRUCT!{struct WTS_SERVER_INFOA {
    pServerName: LPSTR,
}}
pub type PWTS_SERVER_INFOA = *mut WTS_SERVER_INFOA;
STRUCT!{struct WTS_SESSION_INFOW {
    SessionId: DWORD,
    pWinStationName: LPWSTR,
    State: WTS_CONNECTSTATE_CLASS,
}}
pub type PWTS_SESSION_INFOW = *mut WTS_SESSION_INFOW;
STRUCT!{struct WTS_SESSION_INFOA {
    SessionId: DWORD,
    pWinStationName: LPSTR,
    State: WTS_CONNECTSTATE_CLASS,
}}
pub type PWTS_SESSION_INFOA = *mut WTS_SESSION_INFOA;
STRUCT!{struct WTS_SESSION_INFO_1W {
    ExecEnvId: DWORD,
    State: WTS_CONNECTSTATE_CLASS,
    SessionId: DWORD,
    pSessionName: LPWSTR,
    pHostName: LPWSTR,
    pUserName: LPWSTR,
    pDomainName: LPWSTR,
    pFarmName: LPWSTR,
}}
pub type PWTS_SESSION_INFO_1W = *mut WTS_SESSION_INFO_1W;
STRUCT!{struct WTS_SESSION_INFO_1A {
    ExecEnvId: DWORD,
    State: WTS_CONNECTSTATE_CLASS,
    SessionId: DWORD,
    pSessionName: LPSTR,
    pHostName: LPSTR,
    pUserName: LPSTR,
    pDomainName: LPSTR,
    pFarmName: LPSTR,
}}
pub type PWTS_SESSION_INFO_1A = *mut WTS_SESSION_INFO_1A;
STRUCT!{struct WTS_PROCESS_INFOW {
    SessionId: DWORD,
    ProcessId: DWORD,
    pProcessName: LPWSTR,
    pUserSid: PSID,
}}
pub type PWTS_PROCESS_INFOW = *mut WTS_PROCESS_INFOW;
STRUCT!{struct WTS_PROCESS_INFOA {
    SessionId: DWORD,
    ProcessId: DWORD,
    pProcessName: LPSTR,
    pUserSid: PSID,
}}
pub type PWTS_PROCESS_INFOA = *mut WTS_PROCESS_INFOA;
pub const WTS_PROTOCOL_TYPE_CONSOLE: DWORD = 0;
pub const WTS_PROTOCOL_TYPE_ICA: DWORD = 1;
pub const WTS_PROTOCOL_TYPE_RDP: DWORD = 2;
ENUM!{enum WTS_INFO_CLASS {
    WTSInitialProgram,
    WTSApplicationName,
    WTSWorkingDirectory,
    WTSOEMId,
    WTSSessionId,
    WTSUserName,
    WTSWinStationName,
    WTSDomainName,
    WTSConnectState,
    WTSClientBuildNumber,
    WTSClientName,
    WTSClientDirectory,
    WTSClientProductId,
    WTSClientHardwareId,
    WTSClientAddress,
    WTSClientDisplay,
    WTSClientProtocolType,
    WTSIdleTime,
    WTSLogonTime,
    WTSIncomingBytes,
    WTSOutgoingBytes,
    WTSIncomingFrames,
    WTSOutgoingFrames,
    WTSClientInfo,
    WTSSessionInfo,
    WTSSessionInfoEx,
    WTSConfigInfo,
    WTSValidationInfo,
    WTSSessionAddressV4,
    WTSIsRemoteSession,
}}
STRUCT!{struct WTSCONFIGINFOW {
    version: ULONG,
    fConnectClientDrivesAtLogon: ULONG,
    fConnectPrinterAtLogon: ULONG,
    fDisablePrinterRedirection: ULONG,
    fDisableDefaultMainClientPrinter: ULONG,
    ShadowSettings: ULONG,
    LogonUserName: [WCHAR; USERNAME_LENGTH + 1],
    LogonDomain: [WCHAR; DOMAIN_LENGTH + 1],
    WorkDirectory: [WCHAR; MAX_PATH + 1],
    InitialProgram: [WCHAR; MAX_PATH + 1],
    ApplicationName: [WCHAR; MAX_PATH + 1],
}}
pub type PWTSCONFIGINFOW = *mut WTSCONFIGINFOW;
STRUCT!{struct WTSCONFIGINFOA {
    version: ULONG,
    fConnectClientDrivesAtLogon: ULONG,
    fConnectPrinterAtLogon: ULONG,
    fDisablePrinterRedirection: ULONG,
    fDisableDefaultMainClientPrinter: ULONG,
    ShadowSettings: ULONG,
    LogonUserName: [CHAR; USERNAME_LENGTH + 1],
    LogonDomain: [CHAR; DOMAIN_LENGTH + 1],
    WorkDirectory: [CHAR; MAX_PATH + 1],
    InitialProgram: [CHAR; MAX_PATH + 1],
    ApplicationName: [CHAR; MAX_PATH + 1],
}}
pub type PWTSCONFIGINFOA = *mut WTSCONFIGINFOA;
STRUCT!{struct WTSINFOW {
    State: WTS_CONNECTSTATE_CLASS,
    SessionId: DWORD,
    IncomingBytes: DWORD,
    OutgoingBytes: DWORD,
    IncomingFrames: DWORD,
    OutgoingFrames: DWORD,
    IncomingCompressedBytes: DWORD,
    OutgoingCompressedBytes: DWORD,
    WinStationName: [WCHAR; WINSTATIONNAME_LENGTH],
    Domain: [WCHAR; DOMAIN_LENGTH],
    UserName: [WCHAR; USERNAME_LENGTH + 1],
    ConnectTime: LARGE_INTEGER,
    DisconnectTime: LARGE_INTEGER,
    LastInputTime: LARGE_INTEGER,
    LogonTime: LARGE_INTEGER,
    CurrentTime: LARGE_INTEGER,
}}
pub type PWTSINFOW = *mut WTSINFOW;
STRUCT!{struct WTSINFOA {
    State: WTS_CONNECTSTATE_CLASS,
    SessionId: DWORD,
    IncomingBytes: DWORD,
    OutgoingBytes: DWORD,
    IncomingFrames: DWORD,
    OutgoingFrames: DWORD,
    IncomingCompressedBytes: DWORD,
    OutgoingCompressedBytes: DWORD,
    WinStationName: [CHAR; WINSTATIONNAME_LENGTH],
    Domain: [CHAR; DOMAIN_LENGTH],
    UserName: [CHAR; USERNAME_LENGTH + 1],
    ConnectTime: LARGE_INTEGER,
    DisconnectTime: LARGE_INTEGER,
    LastInputTime: LARGE_INTEGER,
    LogonTime: LARGE_INTEGER,
    CurrentTime: LARGE_INTEGER,
}}
pub type PWTSINFOA = *mut WTSINFOA;
pub const WTS_SESSIONSTATE_UNKNOWN: DWORD = 0xFFFFFFFF;
pub const WTS_SESSIONSTATE_LOCK: DWORD = 0x00000000;
pub const WTS_SESSIONSTATE_UNLOCK: DWORD = 0x00000001;
STRUCT!{struct WTSINFOEX_LEVEL1_W {
    SessionId: ULONG,
    SessionState: WTS_CONNECTSTATE_CLASS,
    SessionFlags: LONG,
    WinStationName: [WCHAR; WINSTATIONNAME_LENGTH + 1],
    UserName: [WCHAR; USERNAME_LENGTH + 1],
    DomainName: [WCHAR; DOMAIN_LENGTH + 1],
    LogonTime: LARGE_INTEGER,
    ConnectTime: LARGE_INTEGER,
    DisconnectTime: LARGE_INTEGER,
    LastInputTime: LARGE_INTEGER,
    CurrentTime: LARGE_INTEGER,
    IncomingBytes: DWORD,
    OutgoingBytes: DWORD,
    IncomingFrames: DWORD,
    OutgoingFrames: DWORD,
    IncomingCompressedBytes: DWORD,
    OutgoingCompressedBytes: DWORD,
}}
pub type PWTSINFOEX_LEVEL1_W = *mut WTSINFOEX_LEVEL1_W;
STRUCT!{struct WTSINFOEX_LEVEL1_A {
    SessionId: ULONG,
    SessionState: WTS_CONNECTSTATE_CLASS,
    SessionFlags: LONG,
    WinStationName: [CHAR; WINSTATIONNAME_LENGTH + 1],
    UserName: [CHAR; USERNAME_LENGTH + 1],
    DomainName: [CHAR; DOMAIN_LENGTH + 1],
    LogonTime: LARGE_INTEGER,
    ConnectTime: LARGE_INTEGER,
    DisconnectTime: LARGE_INTEGER,
    LastInputTime: LARGE_INTEGER,
    CurrentTime: LARGE_INTEGER,
    IncomingBytes: DWORD,
    OutgoingBytes: DWORD,
    IncomingFrames: DWORD,
    OutgoingFrames: DWORD,
    IncomingCompressedBytes: DWORD,
    OutgoingCompressedBytes: DWORD,
}}
pub type PWTSINFOEX_LEVEL1_A = *mut WTSINFOEX_LEVEL1_A;
UNION!{union WTSINFOEX_LEVEL_W {
    [u64; 28],
    WTSInfoExLevel1 WTSInfoExLevel1_mut: WTSINFOEX_LEVEL1_W,
}}
pub type PWTSINFOEX_LEVEL_W = *mut WTSINFOEX_LEVEL_W;
UNION!{union WTSINFOEX_LEVEL_A {
    [u64; 19],
    WTSInfoExLevel1 WTSInfoExLevel1_mut: WTSINFOEX_LEVEL1_A,
}}
pub type PWTSINFOEX_LEVEL_A = *mut WTSINFOEX_LEVEL_A;
STRUCT!{struct WTSINFOEXW {
    Level: DWORD,
    Data: WTSINFOEX_LEVEL_W,
}}
pub type PWTSINFOEXW = *mut WTSINFOEXW;
STRUCT!{struct WTSINFOEXA {
    Level: DWORD,
    Data: WTSINFOEX_LEVEL_A,
}}
pub type PWTSINFOEXA = *mut WTSINFOEXA;
STRUCT!{struct WTSCLIENTW {
    ClientName: [WCHAR; CLIENTNAME_LENGTH + 1],
    Domain: [WCHAR; DOMAIN_LENGTH + 1],
    UserName: [WCHAR; USERNAME_LENGTH + 1],
    WorkDirectory: [WCHAR; MAX_PATH + 1],
    InitialProgram: [WCHAR; MAX_PATH + 1],
    EncryptionLevel: BYTE,
    ClientAddressFamily: ULONG,
    ClientAddress: [USHORT; CLIENTADDRESS_LENGTH + 1],
    HRes: USHORT,
    VRes: USHORT,
    ColorDepth: USHORT,
    ClientDirectory: [WCHAR; MAX_PATH + 1],
    ClientBuildNumber: ULONG,
    ClientHardwareId: ULONG,
    ClientProductId: USHORT,
    OutBufCountHost: USHORT,
    OutBufCountClient: USHORT,
    OutBufLength: USHORT,
    DeviceId: [WCHAR; MAX_PATH + 1],
}}
pub type PWTSCLIENTW = *mut WTSCLIENTW;
STRUCT!{struct WTSCLIENTA {
    ClientName: [CHAR; CLIENTNAME_LENGTH + 1],
    Domain: [CHAR; DOMAIN_LENGTH + 1],
    UserName: [CHAR; USERNAME_LENGTH + 1],
    WorkDirectory: [CHAR; MAX_PATH + 1],
    InitialProgram: [CHAR; MAX_PATH + 1],
    EncryptionLevel: BYTE,
    ClientAddressFamily: ULONG,
    ClientAddress: [USHORT; CLIENTADDRESS_LENGTH + 1],
    HRes: USHORT,
    VRes: USHORT,
    ColorDepth: USHORT,
    ClientDirectory: [CHAR; MAX_PATH + 1],
    ClientBuildNumber: ULONG,
    ClientHardwareId: ULONG,
    ClientProductId: USHORT,
    OutBufCountHost: USHORT,
    OutBufCountClient: USHORT,
    OutBufLength: USHORT,
    DeviceId: [CHAR; MAX_PATH + 1],
}}
pub type PWTSCLIENTA = *mut WTSCLIENTA;
const PRODUCTINFO_COMPANYNAME_LENGTH: usize = 256;
const PRODUCTINFO_PRODUCTID_LENGTH: usize = 4;
STRUCT!{struct WTS_PRODUCT_INFOA
{
    CompanyName: [CHAR; PRODUCTINFO_COMPANYNAME_LENGTH],
    ProductID: [CHAR; PRODUCTINFO_PRODUCTID_LENGTH],
}}
pub type PRODUCT_INFOA = WTS_PRODUCT_INFOA;
STRUCT!{struct WTS_PRODUCT_INFOW
{
    CompanyName: [WCHAR; PRODUCTINFO_COMPANYNAME_LENGTH],
    ProductID: [WCHAR; PRODUCTINFO_PRODUCTID_LENGTH],
}}
pub type PRODUCT_INFOW = WTS_PRODUCT_INFOW;
const VALIDATIONINFORMATION_LICENSE_LENGTH: usize = 16384;
const VALIDATIONINFORMATION_HARDWAREID_LENGTH: usize = 20;
STRUCT!{struct WTS_VALIDATION_INFORMATIONA {
    ProductInfo: PRODUCT_INFOA,
    License: [BYTE; VALIDATIONINFORMATION_LICENSE_LENGTH],
    LicenseLength: DWORD,
    HardwareID: [BYTE; VALIDATIONINFORMATION_HARDWAREID_LENGTH],
    HardwareIDLength: DWORD,
}}
pub type PWTS_VALIDATION_INFORMATIONA = *mut WTS_VALIDATION_INFORMATIONA;

STRUCT!{struct WTS_VALIDATION_INFORMATIONW {
    ProductInfo: PRODUCT_INFOW,
    License: [BYTE; VALIDATIONINFORMATION_LICENSE_LENGTH],
    LicenseLength: DWORD,
    HardwareID: [BYTE; VALIDATIONINFORMATION_HARDWAREID_LENGTH],
    HardwareIDLength: DWORD,
}}
pub type PWTS_VALIDATION_INFORMATIONW = *mut WTS_VALIDATION_INFORMATIONW;
STRUCT!{struct WTS_CLIENT_ADDRESS {
    AddressFamily: DWORD,
    Address: [BYTE; 20],
}}
pub type PWTS_CLIENT_ADDRESS = *mut WTS_CLIENT_ADDRESS;
STRUCT!{struct WTS_CLIENT_DISPLAY {
    HorizontalResolution: DWORD,
    VerticalResolution: DWORD,
    ColorDepth: DWORD,
}}
pub type PWTS_CLIENT_DISPLAY = *mut WTS_CLIENT_DISPLAY;
ENUM!{enum WTS_CONFIG_CLASS {
    WTSUserConfigInitialProgram,
    WTSUserConfigWorkingDirectory,
    WTSUserConfigfInheritInitialProgram,
    WTSUserConfigfAllowLogonTerminalServer,
    WTSUserConfigTimeoutSettingsConnections,
    WTSUserConfigTimeoutSettingsDisconnections,
    WTSUserConfigTimeoutSettingsIdle,
    WTSUserConfigfDeviceClientDrives,
    WTSUserConfigfDeviceClientPrinters,
    WTSUserConfigfDeviceClientDefaultPrinter,
    WTSUserConfigBrokenTimeoutSettings,
    WTSUserConfigReconnectSettings,
    WTSUserConfigModemCallbackSettings,
    WTSUserConfigModemCallbackPhoneNumber,
    WTSUserConfigShadowingSettings,
    WTSUserConfigTerminalServerProfilePath,
    WTSUserConfigTerminalServerHomeDir,
    WTSUserConfigTerminalServerHomeDirDrive,
    WTSUserConfigfTerminalServerRemoteHomeDir,
    WTSUserConfigUser,
}}
ENUM!{enum WTS_CONFIG_SOURCE {
    WTSUserConfigSourceSAM,
}}
STRUCT!{struct WTSUSERCONFIGA {
    Source: DWORD,
    InheritInitialProgram: DWORD,
    AllowLogonTerminalServer: DWORD,
    TimeoutSettingsConnections: DWORD,
    TimeoutSettingsDisconnections: DWORD,
    TimeoutSettingsIdle: DWORD,
    DeviceClientDrives: DWORD,
    DeviceClientPrinters: DWORD,
    ClientDefaultPrinter: DWORD,
    BrokenTimeoutSettings: DWORD,
    ReconnectSettings: DWORD,
    ShadowingSettings: DWORD,
    TerminalServerRemoteHomeDir: DWORD,
    InitialProgram: [CHAR; MAX_PATH + 1],
    WorkDirectory: [CHAR; MAX_PATH + 1],
    TerminalServerProfilePath: [CHAR; MAX_PATH + 1],
    TerminalServerHomeDir: [CHAR; MAX_PATH + 1],
    TerminalServerHomeDirDrive: [CHAR; WTS_DRIVE_LENGTH + 1],
}}
pub type PWTSUSERCONFIGA = *mut WTSUSERCONFIGA;
STRUCT!{struct WTSUSERCONFIGW {
    Source: DWORD,
    InheritInitialProgram: DWORD,
    AllowLogonTerminalServer: DWORD,
    TimeoutSettingsConnections: DWORD,
    TimeoutSettingsDisconnections: DWORD,
    TimeoutSettingsIdle: DWORD,
    DeviceClientDrives: DWORD,
    DeviceClientPrinters: DWORD,
    ClientDefaultPrinter: DWORD,
    BrokenTimeoutSettings: DWORD,
    ReconnectSettings: DWORD,
    ShadowingSettings: DWORD,
    TerminalServerRemoteHomeDir: DWORD,
    InitialProgram: [WCHAR; MAX_PATH + 1],
    WorkDirectory: [CHAR; MAX_PATH + 1],
    TerminalServerProfilePath: [CHAR; MAX_PATH + 1],
    TerminalServerHomeDir: [CHAR; MAX_PATH + 1],
    TerminalServerHomeDirDrive: [CHAR; WTS_DRIVE_LENGTH + 1],
}}
pub type PWTSUSERCONFIGW = *mut WTSUSERCONFIGW;
pub const WTS_EVENT_NONE: DWORD = 0x00000000;
pub const WTS_EVENT_CREATE: DWORD = 0x00000001;
pub const WTS_EVENT_DELETE: DWORD = 0x00000002;
pub const WTS_EVENT_RENAME: DWORD = 0x00000004;
pub const WTS_EVENT_CONNECT: DWORD = 0x00000008;
pub const WTS_EVENT_DISCONNECT: DWORD = 0x00000010;
pub const WTS_EVENT_LOGON: DWORD = 0x00000020;
pub const WTS_EVENT_LOGOFF: DWORD = 0x00000040;
pub const WTS_EVENT_STATECHANGE: DWORD = 0x00000080;
pub const WTS_EVENT_LICENSE: DWORD = 0x00000100;
pub const WTS_EVENT_ALL: DWORD = 0x7fffffff;
pub const WTS_EVENT_FLUSH: DWORD = 0x80000000;
pub const REMOTECONTROL_KBDSHIFT_HOTKEY: DWORD = 0x1;
pub const REMOTECONTROL_KBDCTRL_HOTKEY: DWORD = 0x2;
pub const REMOTECONTROL_KBDALT_HOTKEY: DWORD = 0x4;
ENUM!{enum WTS_VIRTUAL_CLASS {
    WTSVirtualClientData,
    WTSVirtualFileHandle,
}}
STRUCT!{struct WTS_SESSION_ADDRESS {
    AddressFamily: DWORD,
    Address: [BYTE; 20],
}}
pub type PWTS_SESSION_ADDRESS = *mut WTS_SESSION_ADDRESS;
extern "system" {
    pub fn WTSStopRemoteControlSession(LogonId: ULONG) -> BOOL;
    pub fn WTSStartRemoteControlSessionW(
        pTargetServerName: LPWSTR,
        TargetLogonId: ULONG,
        HotkeyVk: BYTE,
        HotkeyModifiers: USHORT,
    ) -> BOOL;
    pub fn WTSStartRemoteControlSessionA(
        pTargetServerName: LPSTR,
        TargetLogonId: ULONG,
        HotkeyVk: BYTE,
        HotkeyModifiers: USHORT,
    ) -> BOOL;
    pub fn WTSConnectSessionA(
        LogonId: ULONG,
        TargetLogonId: ULONG,
        pPassword: PSTR,
        bWait: BOOL,
    ) -> BOOL;
    pub fn WTSConnectSessionW(
        LogonId: ULONG,
        TargetLogonId: ULONG,
        pPassword: PWSTR,
        bWait: BOOL,
    ) -> BOOL;
    pub fn WTSEnumerateServersW(
        pDomainName: LPWSTR,
        Reserved: DWORD,
        Version: DWORD,
        ppServerInfo: *mut PWTS_SERVER_INFOW,
        pCount: *mut DWORD,
    ) -> BOOL;
    pub fn WTSEnumerateServersA(
        pDomainName: LPSTR,
        Reserved: DWORD,
        Version: DWORD,
        ppServerInfo: *mut PWTS_SERVER_INFOA,
        pCount: *mut DWORD,
    ) -> BOOL;
    pub fn WTSOpenServerW(pServerName: LPWSTR) -> HANDLE;
    pub fn WTSOpenServerA(pServerName: LPSTR) -> HANDLE;
    pub fn WTSOpenServerExW(pServerName: LPWSTR) -> HANDLE;
    pub fn WTSOpenServerExA(pServerName: LPSTR) -> HANDLE;
    pub fn WTSCloseServer(hServer: HANDLE);
    pub fn WTSEnumerateSessionsW(
        hServer: HANDLE,
        Reserved: DWORD,
        Version: DWORD,
        ppSessionInfo: *mut PWTS_SESSION_INFOW,
        pCount: *mut DWORD
        ) -> BOOL;
    pub fn WTSEnumerateSessionsA(
        hServer: HANDLE,
        Reserved: DWORD,
        Version: DWORD,
        ppSessionInfo: *mut PWTS_SESSION_INFOA,
        pCount: *mut DWORD
        ) -> BOOL;
    pub fn WTSEnumerateSessionsExW(
        hServer: HANDLE,
        pLevel: *mut DWORD,
        Filter: DWORD,
        ppSessionInfo: *mut PWTS_SESSION_INFO_1W,
        pCount: *mut DWORD,
    ) -> BOOL;
    pub fn WTSEnumerateSessionsExA(
        hServer: HANDLE,
        pLevel: *mut DWORD,
        Filter: DWORD,
        ppSessionInfo: *mut PWTS_SESSION_INFO_1A,
        pCount: *mut DWORD,
    ) -> BOOL;
    pub fn WTSEnumerateProcessesW(
        hServer: HANDLE,
        Reserved: DWORD,
        Version: DWORD,
        ppProcessInfo: *mut PWTS_PROCESS_INFOW,
        pCount: *mut DWORD,
    ) -> BOOL;
    pub fn WTSEnumerateProcessesA(
        hServer: HANDLE,
        Reserved: DWORD,
        Version: DWORD,
        ppProcessInfo: *mut PWTS_PROCESS_INFOA,
        pCount: *mut DWORD,
    ) -> BOOL;
    pub fn WTSTerminateProcess(
        hServer: HANDLE,
        ProcessId: DWORD,
        ExitCode: DWORD,
    ) -> BOOL;
    pub fn WTSQuerySessionInformationW(
        hServer: HANDLE,
        SessionId: DWORD,
        WTSInfoClass: WTS_INFO_CLASS,
        ppBuffer: *mut LPWSTR,
        pBytesReturned: *mut DWORD,
    ) -> BOOL;
    pub fn WTSQuerySessionInformationA(
        hServer: HANDLE,
        SessionId: DWORD,
        WTSInfoClass: WTS_INFO_CLASS,
        ppBuffer: *mut LPSTR,
        pBytesReturned: *mut DWORD,
    ) -> BOOL;
    pub fn WTSQueryUserConfigW(
        pServerName: LPWSTR,
        pUserName: LPWSTR,
        WTSConfigClass: WTS_CONFIG_CLASS,
        ppBuffer: *mut LPWSTR,
        pBytesReturned: *mut DWORD,
    ) -> BOOL;
    pub fn WTSQueryUserConfigA(
        pServerName: LPSTR,
        pUserName: LPSTR,
        WTSConfigClass: WTS_CONFIG_CLASS,
        ppBuffer: *mut LPSTR,
        pBytesReturned: *mut DWORD,
    ) -> BOOL;
    pub fn WTSQueryUserToken(
        SessionId: ULONG,
        phToken: PHANDLE
        ) -> BOOL;
    pub fn WTSSetUserConfigW(
        pServerName: LPWSTR,
        pUserName: LPWSTR,
        WTSConfigClass: WTS_CONFIG_CLASS,
        pBuffer: LPWSTR,
        DataLength: DWORD,
    ) -> BOOL;
    pub fn WTSSetUserConfigA(
        pServerName: LPSTR,
        pUserName: LPSTR,
        WTSConfigClass: WTS_CONFIG_CLASS,
        pBuffer: LPSTR,
        DataLength: DWORD,
    ) -> BOOL;
    pub fn WTSSendMessageW(
        hServer: HANDLE,
        SessionId: DWORD,
        pTitle: LPWSTR,
        TitleLength: DWORD,
        pMessage: LPWSTR,
        MessageLength: DWORD,
        Style: DWORD,
        Timeout: DWORD,
        pResponse: *mut DWORD,
        bWait: BOOL,
    ) -> BOOL;
    pub fn WTSSendMessageA(
        hServer: HANDLE,
        SessionId: DWORD,
        pTitle: LPSTR,
        TitleLength: DWORD,
        pMessage: LPSTR,
        MessageLength: DWORD,
        Style: DWORD,
        Timeout: DWORD,
        pResponse: *mut DWORD,
        bWait: BOOL,
    ) -> BOOL;
    pub fn WTSDisconnectSession(
        hServer: HANDLE,
        SessionId: DWORD,
        bWait: BOOL,
    ) -> BOOL;
    pub fn WTSLogoffSession(
        hServer: HANDLE,
        SessionId: DWORD,
        bWait: BOOL,
    ) -> BOOL;
    pub fn WTSShutdownSystem(
        hServer: HANDLE,
        ShutdownFlag: DWORD,
    ) -> BOOL;
    pub fn WTSWaitSystemEvent(
        hServer: HANDLE,
        EventMask: DWORD,
        pEventFlags: *mut DWORD,
    ) -> BOOL;
    pub fn WTSVirtualChannelOpen(
        hServer: HANDLE,
        SessionId: DWORD,
        pVirtualName: LPSTR,
    ) -> HANDLE;
}
pub const WTS_CHANNEL_OPTION_DYNAMIC: DWORD = 0x00000001;
pub const WTS_CHANNEL_OPTION_DYNAMIC_PRI_LOW: DWORD = 0x00000000;
pub const WTS_CHANNEL_OPTION_DYNAMIC_PRI_MED: DWORD = 0x00000002;
pub const WTS_CHANNEL_OPTION_DYNAMIC_PRI_HIGH: DWORD = 0x00000004;
pub const WTS_CHANNEL_OPTION_DYNAMIC_PRI_REAL: DWORD = 0x00000006;
pub const WTS_CHANNEL_OPTION_DYNAMIC_NO_COMPRESS: DWORD = 0x00000008;
extern "system" {
    pub fn WTSVirtualChannelOpenEx(
        SessionId: DWORD,
        pVirtualName: LPSTR,
        flags: DWORD,
    ) -> HANDLE;
    pub fn WTSVirtualChannelClose(hChannelHandle: HANDLE) -> BOOL;
    pub fn WTSVirtualChannelRead(
        hChannelHandle: HANDLE,
        TimeOut: ULONG,
        Buffer: PCHAR,
        BufferSize: ULONG,
        pBytesRead: PULONG,
    ) -> BOOL;
    pub fn WTSVirtualChannelWrite(
        hChannelHandle: HANDLE,
        Buffer: PCHAR,
        Length: ULONG,
        pBytesWritten: PULONG,
    ) -> BOOL;
    pub fn WTSVirtualChannelPurgeInput(hChannelHandle: HANDLE) -> BOOL;
    pub fn WTSVirtualChannelPurgeOutput(hChannelHandle: HANDLE) -> BOOL;
    pub fn WTSVirtualChannelQuery(
        hChannelHandle: HANDLE,
        virtualClass: WTS_VIRTUAL_CLASS,
        ppBuffer: *mut PVOID,
        pBytesReturned: *mut DWORD,
    ) -> BOOL;
    pub fn WTSFreeMemory(pMemory: PVOID);
}
pub const NOTIFY_FOR_ALL_SESSIONS: DWORD = 1;
pub const NOTIFY_FOR_THIS_SESSION: DWORD = 0;
extern "system" {
    pub fn WTSRegisterSessionNotification(hWnd: HWND, dwFlags: DWORD) -> BOOL;
    pub fn WTSUnRegisterSessionNotification(hWnd: HWND) -> BOOL;
    pub fn WTSRegisterSessionNotificationEx(
        hServer: HANDLE,
        hWnd: HWND,
        dwFlags: DWORD,
    ) -> BOOL;
    pub fn WTSUnRegisterSessionNotificationEx(hServer: HANDLE, hWnd: HWND) -> BOOL;
}
pub const WTS_PROCESS_INFO_LEVEL_0: DWORD = 0;
pub const WTS_PROCESS_INFO_LEVEL_1: DWORD = 1;
STRUCT!{struct WTS_PROCESS_INFO_EXW {
    SessionId: DWORD,
    ProcessId: DWORD,
    pProcessName: LPWSTR,
    pUserSid: PSID,
    NumberOfThreads: DWORD,
    HandleCount: DWORD,
    PagefileUsage: DWORD,
    PeakPagefileUsage: DWORD,
    WorkingSetSize: DWORD,
    PeakWorkingSetSize: DWORD,
    UserTime: LARGE_INTEGER,
    KernelTime: LARGE_INTEGER,
}}
pub type PWTS_PROCESS_INFO_EXW = *mut WTS_PROCESS_INFO_EXW;
STRUCT!{struct WTS_PROCESS_INFO_EXA {
    SessionId: DWORD,
    ProcessId: DWORD,
    pProcessName: LPSTR,
    pUserSid: PSID,
    NumberOfThreads: DWORD,
    HandleCount: DWORD,
    PagefileUsage: DWORD,
    PeakPagefileUsage: DWORD,
    WorkingSetSize: DWORD,
    PeakWorkingSetSize: DWORD,
    UserTime: LARGE_INTEGER,
    KernelTime: LARGE_INTEGER,
}}
pub type PWTS_PROCESS_INFO_EXA = *mut WTS_PROCESS_INFO_EXA;
ENUM!{enum WTS_TYPE_CLASS {
    WTSTypeProcessInfoLevel0,
    WTSTypeProcessInfoLevel1,
    WTSTypeSessionInfoLevel1,
}}
extern "system" {
    pub fn WTSFreeMemoryExW(
        WTSTypeClass: WTS_TYPE_CLASS,
        pMemory: PVOID,
        NumberOfEntries: ULONG,
    ) -> BOOL;
    pub fn WTSFreeMemoryExA(
        WTSTypeClass: WTS_TYPE_CLASS,
        pMemory: PVOID,
        NumberOfEntries: ULONG,
    ) -> BOOL;
    pub fn WTSEnumerateProcessesExW(
        hServer: HANDLE,
        pLevel: *mut DWORD,
        SessionId: DWORD,
        ppProcessInfo: *mut LPWSTR,
        pCount: *mut DWORD,
    ) -> BOOL;
    pub fn WTSEnumerateProcessesExA(
        hServer: HANDLE,
        pLevel: *mut DWORD,
        SessionId: DWORD,
        ppProcessInfo: *mut LPSTR,
        pCount: *mut DWORD,
    ) -> BOOL;
}
pub type WTSLISTENERNAMEW = [WCHAR; WTS_LISTENER_NAME_LENGTH + 1];
pub type PWTSLISTENERNAMEW = *mut WTSLISTENERNAMEW;
pub type WTSLISTENERNAMEA = [CHAR; WTS_LISTENER_NAME_LENGTH + 1];
pub type PWTSLISTENERNAMEA = *mut WTSLISTENERNAMEA;
extern "system" {
    pub fn WTSEnumerateListenersW(
        hServer: HANDLE,
        pReserved: PVOID,
        Reserved: DWORD,
        pListeners: PWTSLISTENERNAMEW,
        pCount: *mut DWORD,
    ) -> BOOL;
    pub fn WTSEnumerateListenersA(
        hServer: HANDLE,
        pReserved: PVOID,
        Reserved: DWORD,
        pListeners: PWTSLISTENERNAMEA,
        pCount: *mut DWORD,
    ) -> BOOL;
}
STRUCT!{struct WTSLISTENERCONFIGW {
    version: ULONG,
    fEnableListener: ULONG,
    MaxConnectionCount: ULONG,
    fPromptForPassword: ULONG,
    fInheritColorDepth: ULONG,
    ColorDepth: ULONG,
    fInheritBrokenTimeoutSettings: ULONG,
    BrokenTimeoutSettings: ULONG,
    fDisablePrinterRedirection: ULONG,
    fDisableDriveRedirection: ULONG,
    fDisableComPortRedirection: ULONG,
    fDisableLPTPortRedirection: ULONG,
    fDisableClipboardRedirection: ULONG,
    fDisableAudioRedirection: ULONG,
    fDisablePNPRedirection: ULONG,
    fDisableDefaultMainClientPrinter: ULONG,
    LanAdapter: ULONG,
    PortNumber: ULONG,
    fInheritShadowSettings: ULONG,
    ShadowSettings: ULONG,
    TimeoutSettingsConnection: ULONG,
    TimeoutSettingsDisconnection: ULONG,
    TimeoutSettingsIdle: ULONG,
    SecurityLayer: ULONG,
    MinEncryptionLevel: ULONG,
    UserAuthentication: ULONG,
    Comment: [WCHAR; WTS_COMMENT_LENGTH + 1],
    LogonUserName: [WCHAR; USERNAME_LENGTH + 1],
    LogonDomain: [WCHAR; DOMAIN_LENGTH + 1],
    WorkDirectory: [WCHAR; MAX_PATH + 1],
    InitialProgram: [WCHAR; MAX_PATH + 1],
}}
pub type PWTSLISTENERCONFIGW = *mut WTSLISTENERCONFIGW;
STRUCT!{struct WTSLISTENERCONFIGA {
    version: ULONG,
    fEnableListener: ULONG,
    MaxConnectionCount: ULONG,
    fPromptForPassword: ULONG,
    fInheritColorDepth: ULONG,
    ColorDepth: ULONG,
    fInheritBrokenTimeoutSettings: ULONG,
    BrokenTimeoutSettings: ULONG,
    fDisablePrinterRedirection: ULONG,
    fDisableDriveRedirection: ULONG,
    fDisableComPortRedirection: ULONG,
    fDisableLPTPortRedirection: ULONG,
    fDisableClipboardRedirection: ULONG,
    fDisableAudioRedirection: ULONG,
    fDisablePNPRedirection: ULONG,
    fDisableDefaultMainClientPrinter: ULONG,
    LanAdapter: ULONG,
    PortNumber: ULONG,
    fInheritShadowSettings: ULONG,
    ShadowSettings: ULONG,
    TimeoutSettingsConnection: ULONG,
    TimeoutSettingsDisconnection: ULONG,
    TimeoutSettingsIdle: ULONG,
    SecurityLayer: ULONG,
    MinEncryptionLevel: ULONG,
    UserAuthentication: ULONG,
    Comment: [CHAR; WTS_COMMENT_LENGTH + 1],
    LogonUserName: [CHAR; USERNAME_LENGTH + 1],
    LogonDomain: [CHAR; DOMAIN_LENGTH + 1],
    WorkDirectory: [CHAR; MAX_PATH + 1],
    InitialProgram: [CHAR; MAX_PATH + 1],
}}
pub type PWTSLISTENERCONFIGA = *mut WTSLISTENERCONFIGA;
extern "system" {
    pub fn WTSQueryListenerConfigW(
        hServer: HANDLE,
        pReserved: PVOID,
        Reserved: DWORD,
        pListenerName: LPWSTR,
        pBuffer: PWTSLISTENERCONFIGW,
    ) -> BOOL;
    pub fn WTSQueryListenerConfigA(
        hServer: HANDLE,
        pReserved: PVOID,
        Reserved: DWORD,
        pListenerName: LPSTR,
        pBuffer: PWTSLISTENERCONFIGA,
    ) -> BOOL;
    pub fn WTSCreateListenerW(
        hServer: HANDLE,
        pReserved: PVOID,
        Reserved: DWORD,
        pListenerName: LPWSTR,
        pBuffer: PWTSLISTENERCONFIGW,
        flag: DWORD,
    ) -> BOOL;
    pub fn WTSCreateListenerA(
        hServer: HANDLE,
        pReserved: PVOID,
        Reserved: DWORD,
        pListenerName: LPSTR,
        pBuffer: PWTSLISTENERCONFIGA,
        flag: DWORD,
    ) -> BOOL;
    pub fn WTSSetListenerSecurityW(
        hServer: HANDLE,
        pReserved: PVOID,
        Reserved: DWORD,
        pListenerName: LPWSTR,
        SecurityInformation: SECURITY_INFORMATION,
        pSecurityDescriptor: PSECURITY_DESCRIPTOR,
    ) -> BOOL;
    pub fn WTSSetListenerSecurityA(
        hServer: HANDLE,
        pReserved: PVOID,
        Reserved: DWORD,
        pListenerName: LPSTR,
        SecurityInformation: SECURITY_INFORMATION,
        pSecurityDescriptor: PSECURITY_DESCRIPTOR,
    ) -> BOOL;
    pub fn WTSGetListenerSecurityW(
        hServer: HANDLE,
        pReserved: PVOID,
        Reserved: DWORD,
        pListenerName: LPWSTR,
        SecurityInformation: SECURITY_INFORMATION,
        pSecurityDescriptor: PSECURITY_DESCRIPTOR,
        nLength: DWORD,
        lpnLengthNeeded: LPDWORD,
    ) -> BOOL;
    pub fn WTSGetListenerSecurityA(
        hServer: HANDLE,
        pReserved: PVOID,
        Reserved: DWORD,
        pListenerName: LPSTR,
        SecurityInformation: SECURITY_INFORMATION,
        pSecurityDescriptor: PSECURITY_DESCRIPTOR,
        nLength: DWORD,
        lpnLengthNeeded: LPDWORD,
    ) -> BOOL;
    pub fn WTSEnableChildSessions(bEnable: BOOL) -> BOOL;
    pub fn WTSIsChildSessionsEnabled(pbEnabled: PBOOL) -> BOOL;
    pub fn WTSGetChildSessionId(pSessionId: PULONG) -> BOOL;
}

// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::minwindef::{DWORD, BOOL, PBOOL};
use shared::ntdef::{LPCSTR, LPCWSTR};
use shared::windef::HWND;
use um::setupapi::{HDEVINFO, PSP_DEVINFO_DATA};
pub const INSTALLFLAG_FORCE: DWORD = 0x00000001;
pub const INSTALLFLAG_READONLY: DWORD = 0x00000002;
pub const INSTALLFLAG_NONINTERACTIVE: DWORD = 0x00000004;
extern "system" {
    pub fn UpdateDriverForPlugAndPlayDevicesA(
        hwndParent: HWND,
        HardwareId: LPCSTR,
        FullInfPath: LPCSTR,
        InstallFlags: DWORD,
        pRebootRequired: PBOOL,
    ) -> BOOL;
    pub fn UpdateDriverForPlugAndPlayDevicesW(
        hwndParent: HWND,
        HardwareId: LPCWSTR,
        FullInfPath: LPCWSTR,
        InstallFlags: DWORD,
        pRebootRequired: PBOOL,
    ) -> BOOL;
}
pub const DIIRFLAG_INF_ALREADY_COPIED: DWORD = 0x00000001;
pub const DIIRFLAG_FORCE_INF: DWORD = 0x00000002;
pub const DIIRFLAG_HW_USING_THE_INF: DWORD = 0x00000004;
pub const DIIRFLAG_HOTPATCH: DWORD = 0x00000008;
pub const DIIRFLAG_NOBACKUP: DWORD = 0x00000010;
pub const DIIRFLAG_PRE_CONFIGURE_INF: DWORD = 0x00000020;
pub const DIIRFLAG_INSTALL_AS_SET: DWORD = 0x00000040;
extern "system" {
    pub fn DiInstallDriverW(
        hwndParent: HWND,
        InfPath: LPCWSTR,
        Flags: DWORD,
        NeedReboot: PBOOL,
    ) -> BOOL;
    pub fn DiInstallDriverA(
        hwndParent: HWND,
        InfPath: LPCSTR,
        Flags: DWORD,
        NeedReboot: PBOOL,
    ) -> BOOL;
    pub fn DiUninstallDevice(
        hwndParent: HWND,
        DeviceInfoSet: HDEVINFO,
        DeviceInfoData: PSP_DEVINFO_DATA,
        Flags: DWORD,
        NeedReboot: PBOOL,
    ) -> BOOL;
    pub fn DiShowUpdateDevice(
        hwndParent: HWND,
        DeviceInfoSet: HDEVINFO,
        DeviceInfoData: PSP_DEVINFO_DATA,
        Flags: DWORD,
        NeedReboot: PBOOL,
    ) -> BOOL;
}
pub const ROLLBACK_FLAG_NO_UI: DWORD = 0x00000001;
extern "system" {
    pub fn DiRollbackDriver(
        DeviceInfoSet: HDEVINFO,
        DeviceInfoData: PSP_DEVINFO_DATA,
        hwndParent: HWND,
        Flags: DWORD,
        NeedReboot: PBOOL,
    ) -> BOOL;
}
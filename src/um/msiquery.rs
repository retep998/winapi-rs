// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use ctypes::c_int;
use shared::minwindef::{BOOL, LPDWORD, UINT};
use um::msi::{INSTALLMESSAGE, MSIHANDLE};
use um::winnt::{LPCSTR, LPCWSTR, LPSTR, LPWSTR};
extern "system" {
    pub fn MsiCreateRecord(cParams: UINT) -> MSIHANDLE;
    pub fn MsiRecordIsNull(hRecord: MSIHANDLE, iField: UINT) -> BOOL;
    pub fn MsiRecordDataSize(hRecord: MSIHANDLE, iField: UINT) -> UINT;
    pub fn MsiRecordSetInteger(hRecord: MSIHANDLE, iField: UINT, iValue: c_int) -> UINT;
    pub fn MsiRecordSetStringA(hRecord: MSIHANDLE, iField: UINT, szValue: LPCSTR) -> UINT;
    pub fn MsiRecordSetStringW(hRecord: MSIHANDLE, iField: UINT, szValue: LPCWSTR) -> UINT;
    pub fn MsiGetPropertyA(
        hInstall: MSIHANDLE,
        szName: LPCSTR,
        szValueBuf: LPSTR,
        pcchValueBuf: LPDWORD
    ) -> UINT;
    pub fn MsiGetPropertyW(
        hInstall: MSIHANDLE,
        szName: LPCWSTR,
        szValueBuf: LPWSTR,
        pcchValueBuf: LPDWORD
    ) -> UINT;
    pub fn MsiProcessMessage(
        hInstall: MSIHANDLE,
        eMessageType: INSTALLMESSAGE,
        hRecord: MSIHANDLE,
    ) -> c_int;
    pub fn MsiGetTargetPathA(
        hInstall: MSIHANDLE,
        szFolder: LPCSTR,
        szPathBuf: LPSTR,
        pcchPathBuf: LPDWORD
    ) -> UINT;
    pub fn MsiGetTargetPathW(
        hInstall: MSIHANDLE,
        szFolder: LPCWSTR,
        szPathBuf: LPWSTR,
        pcchPathBuf: LPDWORD
    ) -> UINT;
}

// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::minwindef::PULONG;
use um::winnt::{BOOLEAN, LPSTR, LPWSTR};
ENUM!(
    enum EXTENDED_NAME_FORMAT {
        NameUnknown = 0,
        NameFullyQualifiedDN = 1,
        NameSamCompatible = 2,
        NameDisplay = 3,
        NameUniqueId = 6,
        NameCanonical = 7,
        NameUserPrincipal = 8,
        NameCanonicalEx = 9,
        NameServicePrincipal = 10,
        NameDnsDomain = 12,
        NameGivenName = 13,
        NameSurname = 14,
    }
);
extern "system" {
    pub fn GetUserNameExA(
        NameFormat: EXTENDED_NAME_FORMAT,
        lpNameBuffer: LPSTR,
        nSize: PULONG,
    ) -> BOOLEAN;
    pub fn GetUserNameExW(
        NameFormat: EXTENDED_NAME_FORMAT,
        lpNameBuffer: LPWSTR,
        nSize: PULONG,
    ) -> BOOLEAN;
    pub fn GetComputerObjectNameA(
        NameFormat: EXTENDED_NAME_FORMAT,
        lpNameBuffer: LPSTR,
        nSize: PULONG,
    ) -> BOOLEAN;
    pub fn GetComputerObjectNameW(
        NameFormat: EXTENDED_NAME_FORMAT,
        lpNameBuffer: LPWSTR,
        nSize: PULONG,
    ) -> BOOLEAN;
    pub fn TranslateNameA(
        lpAccountName: LPSTR,
        AccountNameFormat: EXTENDED_NAME_FORMAT,
        DesiredNameFormat: EXTENDED_NAME_FORMAT,
        lpTranslatedName: LPSTR,
        nSize: PULONG,
    ) -> BOOLEAN;
    pub fn TranslateNameW(
        lpAccountName: LPSTR,
        AccountNameFormat: EXTENDED_NAME_FORMAT,
        DesiredNameFormat: EXTENDED_NAME_FORMAT,
        lpTranslatedName: LPWSTR,
        nSize: PULONG,
    ) -> BOOLEAN;
}

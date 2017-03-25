// Copyright © 2015-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Procedure declarations, constant definitions, and macros for the NLS component.
pub const CP_ACP: ::DWORD = 0;
pub const CP_OEMCP: ::DWORD = 1;
pub const CP_MACCP: ::DWORD = 2;
pub const CP_THREAD_ACP: ::DWORD = 3;
pub const CP_SYMBOL: ::DWORD = 42;
pub const CP_UTF7: ::DWORD = 65000;
pub const CP_UTF8: ::DWORD = 65001;
pub const MB_PRECOMPOSED: ::DWORD = 0x00000001;
pub const MB_COMPOSITE: ::DWORD = 0x00000002;
pub const MB_USEGLYPHCHARS: ::DWORD = 0x00000004;
pub const MB_ERR_INVALID_CHARS: ::DWORD = 0x00000008;
pub const WC_COMPOSITECHECK: ::DWORD = 0x00000200;
pub const WC_DISCARDNS: ::DWORD = 0x00000010;
pub const WC_SEPCHARS: ::DWORD = 0x00000020;
pub const WC_DEFAULTCHAR: ::DWORD = 0x00000040;
pub const WC_ERR_INVALID_CHARS: ::DWORD = 0x00000080;
pub const WC_NO_BEST_FIT_CHARS: ::DWORD = 0x00000400;
pub const MAX_LEADBYTES: usize = 12;
pub const MAX_DEFAULTCHAR: usize = 2;
pub type LGRPID = ::DWORD;
pub type LCTYPE = ::DWORD;
pub type CALTYPE = ::DWORD;
pub type CALID = ::DWORD;
pub type GEOID = ::LONG;
pub type GEOTYPE = ::DWORD;
pub type GEOCLASS = ::DWORD;
STRUCT!{struct NLSVERSIONINFO {
    dwNLSVersionInfoSize: ::DWORD,
    dwNLSVersion: ::DWORD,
    dwDefinedVersion: ::DWORD,
    dwEffectiveId: ::DWORD,
    guidCustomVersion: ::GUID,
}}
pub type LPNLSVERSIONINFO = *mut NLSVERSIONINFO;
STRUCT!{struct NLSVERSIONINFOEX {
    dwNLSVersionInfoSize: ::DWORD,
    dwNLSVersion: ::DWORD,
    dwDefinedVersion: ::DWORD,
    dwEffectiveId: ::DWORD,
    guidCustomVersion: ::GUID,
}}
pub type LPNLSVERSIONINFOEX = *mut NLSVERSIONINFOEX;
ENUM!{enum NORM_FORM {
    NormalizationOther = 0,
    NormalizationC = 0x1,
    NormalizationD = 0x2,
    NormalizationKC = 0x5,
    NormalizationKD = 0x6,
}}
FN!{stdcall LANGUAGEGROUP_ENUMPROCA(
    ::LGRPID,
    ::LPSTR,
    ::LPSTR,
    ::DWORD,
    ::LONG_PTR,
) -> ::BOOL}
FN!{stdcall LANGGROUPLOCALE_ENUMPROCA(
    ::LGRPID,
    ::LCID,
    ::LPSTR,
    ::LONG_PTR,
) -> ::BOOL}
FN!{stdcall UILANGUAGE_ENUMPROCA(
    ::LPSTR,
    ::LONG_PTR,
) -> ::BOOL}
FN!{stdcall CODEPAGE_ENUMPROCA(
    ::LPSTR,
) -> ::BOOL}
FN!{stdcall DATEFMT_ENUMPROCA(
    ::LPSTR,
) -> ::BOOL}
FN!{stdcall DATEFMT_ENUMPROCEXA(
    ::LPSTR,
    ::CALID,
) -> ::BOOL}
FN!{stdcall TIMEFMT_ENUMPROCA(
    ::LPSTR,
) -> ::BOOL}
FN!{stdcall CALINFO_ENUMPROCA(
    ::LPSTR,
) -> ::BOOL}
FN!{stdcall CALINFO_ENUMPROCEXA(
    ::LPSTR,
    ::CALID,
) -> ::BOOL}
FN!{stdcall LOCALE_ENUMPROCA(
    ::LPSTR,
) -> ::BOOL}
FN!{stdcall LOCALE_ENUMPROCW(
    ::LPWSTR,
) -> ::BOOL}
FN!{stdcall LANGUAGEGROUP_ENUMPROCW(
    ::LGRPID,
    ::LPWSTR,
    ::LPWSTR,
    ::DWORD,
    ::LONG_PTR,
) -> ::BOOL}
FN!{stdcall LANGGROUPLOCALE_ENUMPROCW(
    ::LGRPID,
    ::LCID,
    ::LPWSTR,
    ::LONG_PTR,
) -> ::BOOL}
FN!{stdcall UILANGUAGE_ENUMPROCW(
    ::LPWSTR,
    ::LONG_PTR,
) -> ::BOOL}
FN!{stdcall CODEPAGE_ENUMPROCW(
    ::LPWSTR,
) -> ::BOOL}
FN!{stdcall DATEFMT_ENUMPROCW(
    ::LPWSTR,
) -> ::BOOL}
FN!{stdcall DATEFMT_ENUMPROCEXW(
    ::LPWSTR,
    ::CALID,
) -> ::BOOL}
FN!{stdcall TIMEFMT_ENUMPROCW(
    ::LPWSTR,
) -> ::BOOL}
FN!{stdcall CALINFO_ENUMPROCW(
    ::LPWSTR,
) -> ::BOOL}
FN!{stdcall CALINFO_ENUMPROCEXW(
    ::LPWSTR,
    ::CALID,
) -> ::BOOL}
FN!{stdcall GEO_ENUMPROC(
    GEOID,
) -> ::BOOL}
STRUCT!{struct CPINFO {
    MaxCharSize: ::UINT,
    DefaultChar: [::BYTE; MAX_DEFAULTCHAR],
    LeadByte: [::BYTE; MAX_LEADBYTES],
}}
pub type LPCPINFO = *mut CPINFO;
STRUCT!{struct CPINFOEXA {
    MaxCharSize: ::UINT,
    DefaultChar: [::BYTE; MAX_DEFAULTCHAR],
    LeadByte: [::BYTE; MAX_LEADBYTES],
    UnicodeDefaultChar: ::WCHAR,
    CodePage: ::UINT,
    CodePageName: [::CHAR; ::MAX_PATH],
}}
pub type LPCPINFOEXA = *mut CPINFOEXA;
STRUCT!{struct CPINFOEXW {
    MaxCharSize: ::UINT,
    DefaultChar: [::BYTE; MAX_DEFAULTCHAR],
    LeadByte: [::BYTE; MAX_LEADBYTES],
    UnicodeDefaultChar: ::WCHAR,
    CodePage: ::UINT,
    CodePageName: [::WCHAR; ::MAX_PATH],
}}
pub type LPCPINFOEXW = *mut CPINFOEXW;
STRUCT!{struct NUMBERFMTA {
    NumDigits: ::UINT,
    LeadingZero: ::UINT,
    Grouping: ::UINT,
    lpDecimalSep: ::LPSTR,
    lpThousandSep: ::LPSTR,
    NegativeOrder: ::UINT,
}}
pub type LPNUMBERFMTA = *mut NUMBERFMTA;
STRUCT!{struct NUMBERFMTW {
    NumDigits: ::UINT,
    LeadingZero: ::UINT,
    Grouping: ::UINT,
    lpDecimalSep: ::LPWSTR,
    lpThousandSep: ::LPWSTR,
    NegativeOrder: ::UINT,
}}
pub type LPNUMBERFMTW = *mut NUMBERFMTW;
STRUCT!{struct CURRENCYFMTA {
    NumDigits: ::UINT,
    LeadingZero: ::UINT,
    Grouping: ::UINT,
    lpDecimalSep: ::LPSTR,
    lpThousandSep: ::LPSTR,
    NegativeOrder: ::UINT,
    PositiveOrder: ::UINT,
    lpCurrencySymbol: ::LPSTR,
}}
pub type LPCURRENCYFMTA = *mut CURRENCYFMTA;
STRUCT!{struct CURRENCYFMTW {
    NumDigits: ::UINT,
    LeadingZero: ::UINT,
    Grouping: ::UINT,
    lpDecimalSep: ::LPWSTR,
    lpThousandSep: ::LPWSTR,
    NegativeOrder: ::UINT,
    PositiveOrder: ::UINT,
    lpCurrencySymbol: ::LPWSTR,
}}
pub type LPCURRENCYFMTW = *mut CURRENCYFMTW;
pub type NLS_FUNCTION = ::DWORD;
STRUCT!{struct FILEMUIINFO {
    dwSize: ::DWORD,
    dwVersion: ::DWORD,
    dwFileType: ::DWORD,
    pChecksum: [::BYTE; 16],
    pServiceChecksum: [::BYTE; 16],
    dwLanguageNameOffset: ::DWORD,
    dwTypeIDMainSize: ::DWORD,
    dwTypeIDMainOffset: ::DWORD,
    dwTypeNameMainOffset: ::DWORD,
    dwTypeIDMUISize: ::DWORD,
    dwTypeIDMUIOffset: ::DWORD,
    dwTypeNameMUIOffset: ::DWORD,
    abBuffer: [::BYTE; 8],
}}
pub type PFILEMUIINFO = *mut FILEMUIINFO;
FN!{stdcall CALINFO_ENUMPROCEXEX(
    ::LPWSTR,
    ::CALID,
    ::LPWSTR,
    ::LPARAM,
) -> ::BOOL}
FN!{stdcall DATEFMT_ENUMPROCEXEX(
    ::LPWSTR,
    ::CALID,
    ::LPARAM,
) -> ::BOOL}
FN!{stdcall TIMEFMT_ENUMPROCEX(
    ::LPWSTR,
    ::LPARAM,
) -> ::BOOL}
FN!{stdcall LOCALE_ENUMPROCEX(
    ::LPWSTR,
    ::DWORD,
    ::LPARAM,
) -> ::BOOL}

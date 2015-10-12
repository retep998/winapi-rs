// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! Procedure declarations, constant definitions, and macros for the NLS component.
pub const CP_ACP: ::DWORD = 0;
pub const CP_OEMCP: ::DWORD = 1;
pub const CP_MACCP: ::DWORD = 2;
pub const CP_THREAD_ACP: ::DWORD = 3;
pub const CP_SYMBOL: ::DWORD = 42;
pub const CP_UTF7: ::DWORD = 65000;
pub const CP_UTF8: ::DWORD = 65001;
pub const MAX_LEADBYTES: usize = 12;
pub const MAX_DEFAULTCHAR: usize = 2;
pub type LGRPID = ::DWORD;
pub type LCTYPE = ::DWORD;
pub type CALTYPE = ::DWORD;
pub type CALID = ::DWORD;
pub type GEOID = ::LONG;
pub type GEOTYPE = ::DWORD;
pub type GEOCLASS = ::DWORD;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NLSVERSIONINFO {
    pub dwNLSVersionInfoSize: ::DWORD,
    pub dwNLSVersion: ::DWORD,
    pub dwDefinedVersion: ::DWORD,
    pub dwEffectiveId: ::DWORD,
    pub guidCustomVersion: ::GUID,
}
pub type LPNLSVERSIONINFO = *mut NLSVERSIONINFO;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NLSVERSIONINFOEX {
    pub dwNLSVersionInfoSize: ::DWORD,
    pub dwNLSVersion: ::DWORD,
    pub dwDefinedVersion: ::DWORD,
    pub dwEffectiveId: ::DWORD,
    pub guidCustomVersion: ::GUID,
}
pub type LPNLSVERSIONINFOEX = *mut NLSVERSIONINFOEX;
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum NORM_FORM {
    NormalizationOther = 0,
    NormalizationC = 0x1,
    NormalizationD = 0x2,
    NormalizationKC = 0x5,
    NormalizationKD = 0x6,
}
pub type LANGUAGEGROUP_ENUMPROCA = Option<unsafe extern "system" fn(
    ::LGRPID, ::LPSTR, ::LPSTR, ::DWORD, ::LONG_PTR,
) -> ::BOOL>;
pub type LANGGROUPLOCALE_ENUMPROCA = Option<unsafe extern "system" fn(
    ::LGRPID, ::LCID, ::LPSTR, ::LONG_PTR,
) -> ::BOOL>;
pub type UILANGUAGE_ENUMPROCA = Option<unsafe extern "system" fn(::LPSTR, ::LONG_PTR) -> ::BOOL>;
pub type CODEPAGE_ENUMPROCA = Option<unsafe extern "system" fn(::LPSTR) -> ::BOOL>;
pub type DATEFMT_ENUMPROCA = Option<unsafe extern "system" fn(::LPSTR) -> ::BOOL>;
pub type DATEFMT_ENUMPROCEXA = Option<unsafe extern "system" fn(::LPSTR, ::CALID) -> ::BOOL>;
pub type TIMEFMT_ENUMPROCA = Option<unsafe extern "system" fn(::LPSTR) -> ::BOOL>;
pub type CALINFO_ENUMPROCA = Option<unsafe extern "system" fn(::LPSTR) -> ::BOOL>;
pub type CALINFO_ENUMPROCEXA = Option<unsafe extern "system" fn(::LPSTR, ::CALID) -> ::BOOL>;
pub type LOCALE_ENUMPROCA = Option<unsafe extern "system" fn(::LPSTR) -> ::BOOL>;
pub type LOCALE_ENUMPROCW = Option<unsafe extern "system" fn(::LPWSTR) -> ::BOOL>;
pub type LANGUAGEGROUP_ENUMPROCW = Option<unsafe extern "system" fn(
    ::LGRPID, ::LPWSTR, ::LPWSTR, ::DWORD, ::LONG_PTR,
) -> ::BOOL>;
pub type LANGGROUPLOCALE_ENUMPROCW = Option<unsafe extern "system" fn(
    ::LGRPID, ::LCID, ::LPWSTR, ::LONG_PTR,
) -> ::BOOL>;
pub type UILANGUAGE_ENUMPROCW = Option<unsafe extern "system" fn(::LPWSTR, ::LONG_PTR) -> ::BOOL>;
pub type CODEPAGE_ENUMPROCW = Option<unsafe extern "system" fn(::LPWSTR) -> ::BOOL>;
pub type DATEFMT_ENUMPROCW = Option<unsafe extern "system" fn(::LPWSTR) -> ::BOOL>;
pub type DATEFMT_ENUMPROCEXW = Option<unsafe extern "system" fn(::LPWSTR, ::CALID) -> ::BOOL>;
pub type TIMEFMT_ENUMPROCW = Option<unsafe extern "system" fn(::LPWSTR) -> ::BOOL>;
pub type CALINFO_ENUMPROCW = Option<unsafe extern "system" fn(::LPWSTR) -> ::BOOL>;
pub type CALINFO_ENUMPROCEXW = Option<unsafe extern "system" fn(::LPWSTR, ::CALID) -> ::BOOL>;
pub type GEO_ENUMPROC = Option<unsafe extern "system" fn(GEOID) -> ::BOOL>;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct CPINFO {
    pub MaxCharSize: ::UINT,
    pub DefaultChar: [::BYTE; MAX_DEFAULTCHAR],
    pub LeadByte: [::BYTE; MAX_LEADBYTES],
}
pub type LPCPINFO = *mut CPINFO;
#[repr(C)] #[derive(Copy)]
pub struct CPINFOEXA {
    pub MaxCharSize: ::UINT,
    pub DefaultChar: [::BYTE; MAX_DEFAULTCHAR],
    pub LeadByte: [::BYTE; MAX_LEADBYTES],
    pub UnicodeDefaultChar: ::WCHAR,
    pub CodePage: ::UINT,
    pub CodePageName: [::CHAR; ::MAX_PATH],
}
impl Clone for CPINFOEXA { fn clone(&self) -> CPINFOEXA{ *self } }
pub type LPCPINFOEXA = *mut CPINFOEXA;
#[repr(C)] #[derive(Copy)]
pub struct CPINFOEXW {
    pub MaxCharSize: ::UINT,
    pub DefaultChar: [::BYTE; MAX_DEFAULTCHAR],
    pub LeadByte: [::BYTE; MAX_LEADBYTES],
    pub UnicodeDefaultChar: ::WCHAR,
    pub CodePage: ::UINT,
    pub CodePageName: [::WCHAR; ::MAX_PATH],
}
impl Clone for CPINFOEXW { fn clone(&self) -> CPINFOEXW{ *self } }
pub type LPCPINFOEXW = *mut CPINFOEXW;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NUMBERFMTA {
    pub NumDigits: ::UINT,
    pub LeadingZero: ::UINT,
    pub Grouping: ::UINT,
    pub lpDecimalSep: ::LPSTR,
    pub lpThousandSep: ::LPSTR,
    pub NegativeOrder: ::UINT,
}
pub type LPNUMBERFMTA = *mut NUMBERFMTA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NUMBERFMTW {
    pub NumDigits: ::UINT,
    pub LeadingZero: ::UINT,
    pub Grouping: ::UINT,
    pub lpDecimalSep: ::LPWSTR,
    pub lpThousandSep: ::LPWSTR,
    pub NegativeOrder: ::UINT,
}
pub type LPNUMBERFMTW = *mut NUMBERFMTW;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct CURRENCYFMTA {
    pub NumDigits: ::UINT,
    pub LeadingZero: ::UINT,
    pub Grouping: ::UINT,
    pub lpDecimalSep: ::LPSTR,
    pub lpThousandSep: ::LPSTR,
    pub NegativeOrder: ::UINT,
    pub PositiveOrder: ::UINT,
    pub lpCurrencySymbol: ::LPSTR,
}
pub type LPCURRENCYFMTA = *mut CURRENCYFMTA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct CURRENCYFMTW {
    pub NumDigits: ::UINT,
    pub LeadingZero: ::UINT,
    pub Grouping: ::UINT,
    pub lpDecimalSep: ::LPWSTR,
    pub lpThousandSep: ::LPWSTR,
    pub NegativeOrder: ::UINT,
    pub PositiveOrder: ::UINT,
    pub lpCurrencySymbol: ::LPWSTR,
}
pub type LPCURRENCYFMTW = *mut CURRENCYFMTW;
pub type NLS_FUNCTION = ::DWORD;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct FILEMUIINFO {
    pub dwSize: ::DWORD,
    pub dwVersion: ::DWORD,
    pub dwFileType: ::DWORD,
    pub pChecksum: [::BYTE; 16],
    pub pServiceChecksum: [::BYTE; 16],
    pub dwLanguageNameOffset: ::DWORD,
    pub dwTypeIDMainSize: ::DWORD,
    pub dwTypeIDMainOffset: ::DWORD,
    pub dwTypeNameMainOffset: ::DWORD,
    pub dwTypeIDMUISize: ::DWORD,
    pub dwTypeIDMUIOffset: ::DWORD,
    pub dwTypeNameMUIOffset: ::DWORD,
    pub abBuffer: [::BYTE; 8],
}
pub type PFILEMUIINFO = *mut FILEMUIINFO;
pub type CALINFO_ENUMPROCEXEX = Option<unsafe extern "system" fn(
    ::LPWSTR, ::CALID, ::LPWSTR, ::LPARAM,
) -> ::BOOL>;
pub type DATEFMT_ENUMPROCEXEX = Option<unsafe extern "system" fn(
    ::LPWSTR, ::CALID, ::LPARAM,
) -> ::BOOL>;
pub type TIMEFMT_ENUMPROCEX = Option<unsafe extern "system" fn(
    ::LPWSTR, ::LPARAM,
) -> ::BOOL>;
pub type LOCALE_ENUMPROCEX = Option<unsafe extern "system" fn(
    ::LPWSTR, ::DWORD, ::LPARAM,
) -> ::BOOL>;

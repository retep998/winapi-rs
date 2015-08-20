// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! Types and constants for WinAPI bindings.
#![allow(bad_style, raw_pointer_derive)]
#![warn(missing_copy_implementations, trivial_casts, trivial_numeric_casts)]
#![warn(unused_qualifications, unused)]
#![cfg(windows)]
//-------------------------------------------------------------------------------------------------
// Imports
//-------------------------------------------------------------------------------------------------
pub use std::os::raw::{
    c_void,
    c_char,
    c_schar,
    c_uchar,
    c_short,
    c_ushort,
    c_int,
    c_uint,
    c_long,
    c_ulong,
    c_longlong,
    c_ulonglong,
    c_float,
    c_double,
};
pub use audioclient::*;
pub use basetsd::*;
pub use bcrypt::*;
pub use cfg::*;
pub use cfgmgr32::*;
pub use commctrl::*;
pub use commdlg::*;
pub use corsym::*;
pub use d2d1::*;
pub use d2dbasetypes::*;
pub use d3d9::*;
pub use d3d9caps::*;
pub use d3d9types::*;
pub use d3d11::*;
pub use d3dcommon::*;
pub use dbghelp::*;
pub use dcommon::*;
pub use devpropdef::*;
pub use dpapi::*;
pub use dsound::*;
pub use dwmapi::*;
pub use dwrite::*;
pub use dxgi::*;
pub use dxgiformat::*;
pub use dxgitype::*;
pub use errhandlingapi::*;
pub use excpt::*;
pub use fileapi::*;
pub use gl::*;
pub use guiddef::*;
pub use heapapi::*;
pub use hidclass::*;
pub use hidpi::*;
pub use hidsdi::*;
pub use hidusage::*;
pub use hstring::*;
pub use http::*;
pub use imm::*;
pub use inaddr::*;
pub use inspectable::*;
pub use ksmedia::*;
pub use libloaderapi::*;
pub use lmcons::*;
pub use memoryapi::*;
pub use minwinbase::*;
pub use minwindef::*;
pub use mmdeviceapi::*;
pub use mmreg::*;
pub use mmsystem::*;
pub use mscat::*;
pub use mssip::*;
pub use ncrypt::*;
pub use ntdef::*;
pub use ntstatus::*;
pub use objbase::*;
pub use objidl::*;
pub use objidlbase::*;
pub use olectl::*;
pub use processsnapshot::*;
pub use processthreadsapi::*;
pub use propsys::*;
pub use prsht::*;
pub use psapi::*;
pub use qos::*;
pub use reason::*;
pub use rpcdce::*;
pub use schannel::*;
pub use setupapi::*;
pub use shellapi::*;
pub use shellscalingapi::*;
pub use shlobj::*;
pub use shobjidl::*;
pub use shtypes::*;
pub use spapidef::*;
pub use sspi::*;
pub use subauth::*;
pub use synchapi::*;
pub use sysinfoapi::*;
pub use threadpoolapi::*;
pub use timezoneapi::*;
pub use tlhelp32::*;
pub use unknwnbase::*;
pub use usp10::*;
pub use vadefs::*;
pub use vsbackup::*;
pub use vss::*;
pub use vsserror::*;
pub use vswriter::*;
pub use werapi::*;
pub use winbase::*;
pub use wincon::*;
pub use wincred::*;
pub use wincrypt::*;
pub use windowsx::*;
pub use windef::*;
pub use windowscodecs::*;
pub use winerror::*;
pub use wingdi::*;
pub use winhttp::*;
pub use winioctl::*;
pub use winnetwk::*;
pub use winnls::*;
pub use winnt::*;
pub use winsock2::*;
pub use winspool::*;
pub use winsvc::*;
pub use winuser::*;
pub use ws2def::*;
pub use ws2spi::*;
pub use ws2tcpip::*;
pub use wtypesbase::*;
pub use xinput::*;
//-------------------------------------------------------------------------------------------------
// Modules
//-------------------------------------------------------------------------------------------------
#[macro_use] mod macros;
pub mod audioclient;
pub mod basetsd;
pub mod bcrypt;
pub mod cfg;
pub mod cfgmgr32;
pub mod commctrl;
pub mod commdlg;
pub mod corsym;
pub mod d2d1;
pub mod d2dbasetypes;
pub mod d3d9;
pub mod d3d9caps;
pub mod d3d9types;
pub mod d3d11;
pub mod d3dcommon;
pub mod dbghelp;
pub mod dcommon;
pub mod devpropdef;
pub mod dpapi;
pub mod dsound;
pub mod dwmapi;
pub mod dwrite;
pub mod dxgi;
pub mod dxgiformat;
pub mod dxgitype;
pub mod errhandlingapi;
pub mod excpt;
pub mod fileapi;
pub mod gl;
pub mod guiddef;
pub mod heapapi;
pub mod hidclass;
pub mod hidpi;
pub mod hidsdi;
pub mod hidusage;
pub mod hstring;
pub mod http;
pub mod imm;
pub mod inaddr;
pub mod inspectable;
pub mod ksmedia;
pub mod libloaderapi;
pub mod lmcons;
pub mod memoryapi;
pub mod minwinbase;
pub mod minwindef;
pub mod mmdeviceapi;
pub mod mmreg;
pub mod mmsystem;
pub mod mscat;
pub mod mssip;
pub mod ncrypt;
pub mod ntdef;
pub mod ntstatus;
pub mod objbase;
pub mod objidl;
pub mod objidlbase;
pub mod olectl;
pub mod processsnapshot;
pub mod processthreadsapi;
pub mod propsys;
pub mod prsht;
pub mod psapi;
pub mod qos;
pub mod reason;
pub mod rpcdce;
pub mod schannel;
pub mod setupapi;
pub mod shellapi;
pub mod shellscalingapi;
pub mod shlobj;
pub mod shobjidl;
pub mod shtypes;
pub mod spapidef;
pub mod sspi;
pub mod subauth;
pub mod synchapi;
pub mod sysinfoapi;
pub mod threadpoolapi;
pub mod timezoneapi;
pub mod tlhelp32;
pub mod unknwnbase;
pub mod usp10;
pub mod vadefs;
pub mod vsbackup;
pub mod vss;
pub mod vsserror;
pub mod vswriter;
pub mod werapi;
pub mod winbase;
pub mod wincon;
pub mod wincred;
pub mod wincrypt;
pub mod windef;
pub mod windowscodecs;
pub mod windowsx;
pub mod winerror;
pub mod wingdi;
pub mod winhttp;
pub mod winioctl;
pub mod winnetwk;
pub mod winnls;
pub mod winnt;
pub mod winsock2;
pub mod winspool;
pub mod winsvc;
pub mod winuser;
pub mod ws2def;
pub mod ws2spi;
pub mod ws2tcpip;
pub mod wtypesbase;
pub mod xinput;
//-------------------------------------------------------------------------------------------------
// Primitive types not provided by std
//-------------------------------------------------------------------------------------------------
pub type __int8 = i8;
pub type __uint8 = u8;
pub type __int16 = i16;
pub type __uint16 = u16;
pub type __int32 = i32;
pub type __uint32 = u32;
pub type __int64 = i64;
pub type __uint64 = u64;
pub type wchar_t = c_ushort;
#[cfg(target_arch = "x86")]
pub type size_t = c_uint;
#[cfg(target_arch = "x86_64")]
pub type size_t = __uint64;

//-------------------------------------------------------------------------------------------------
// wtypes.h
//-------------------------------------------------------------------------------------------------
pub type DATE = c_double;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DECIMAL {
    pub wReserved: USHORT,
    pub scale: BYTE,
    pub sign: BYTE,
    pub Hi32: ULONG,
    pub Lo64: ULONGLONG,
}
pub const DECIMAL_NEG: ::BYTE = 0x80;
pub type LPDECIMAL = *mut DECIMAL;
pub type VARTYPE = c_ushort;

pub type BSTR = *mut OLECHAR;
pub type LPBSTR = *mut BSTR;

//-------------------------------------------------------------------------------------------------
// audiosessiontypes.h
//-------------------------------------------------------------------------------------------------
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub enum AUDCLNT_SHAREMODE {
    AUDCLNT_SHAREMODE_SHARED,
    AUDCLNT_SHAREMODE_EXCLUSIVE,
}
pub const AUDCLNT_STREAMFLAGS_CROSSPROCESS: DWORD = 0x00010000;
pub const AUDCLNT_STREAMFLAGS_LOOPBACK: DWORD = 0x00020000;
pub const AUDCLNT_STREAMFLAGS_EVENTCALLBACK: DWORD = 0x00040000;
pub const AUDCLNT_STREAMFLAGS_NOPERSIST: DWORD = 0x00080000;
pub const AUDCLNT_STREAMFLAGS_RATEADJUST: DWORD = 0x00100000;

//-------------------------------------------------------------------------------------------------
// strmif.h
//-------------------------------------------------------------------------------------------------
pub type REFERENCE_TIME = LONGLONG;

//-------------------------------------------------------------------------------------------------
// mmreg.h
//-------------------------------------------------------------------------------------------------
#[repr(C, packed)] #[derive(Clone, Copy, Debug)]
pub struct WAVEFORMATEX {
    pub wFormatTag: WORD,
    pub nChannels: WORD,
    pub nSamplesPerSec: DWORD,
    pub nAvgBytesPerSec: DWORD,
    pub nBlockAlign: WORD,
    pub wBitsPerSample: WORD,
    pub cbSize: WORD,
}

#[repr(C, packed)] #[derive(Clone, Copy, Debug)]
pub struct WAVEFORMATEXTENSIBLE {
    pub Format: WAVEFORMATEX,
    pub Samples: WORD,
    pub dwChannelMask: DWORD,
    pub SubFormat: GUID,
}

//-------------------------------------------------------------------------------------------------
// propidl.h
//-------------------------------------------------------------------------------------------------
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct PROPVARIANT {
    pub vt: VARTYPE,
    pub wReserved1: WORD,
    pub wReserved2: WORD,
    pub wReserved3: WORD,
    pub data: [u8; 16],
}

//-------------------------------------------------------------------------------------------------
// combaseapi.h
// Base Component Object Model defintions.
//-------------------------------------------------------------------------------------------------
pub const CLSCTX_INPROC_SERVER: DWORD = 0x1;
pub const CLSCTX_INPROC_HANDLER: DWORD = 0x2;
pub const CLSCTX_LOCAL_SERVER: DWORD = 0x4;
pub const CLSCTX_REMOTE_SERVER: DWORD = 0x10;
pub const CLSCTX_SERVER: DWORD = CLSCTX_INPROC_SERVER | CLSCTX_LOCAL_SERVER |
                                 CLSCTX_REMOTE_SERVER;
pub const CLSCTX_ALL: DWORD = CLSCTX_INPROC_HANDLER | CLSCTX_SERVER;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ServerInformation {
    pub dwServerPid: DWORD,
    pub dwServerTid: DWORD,
    pub ui64ServerAddress: UINT64,
}
pub type PServerInformation = *mut ServerInformation;
DECLARE_HANDLE!(CO_MTA_USAGE_COOKIE, CO_MTA_USAGE_COOKIE__);

//-------------------------------------------------------------------------------------------------
// playsoundapi.h
// ApiSet Contract for api-ms-win-mm-playsound-l1-1-0
//-------------------------------------------------------------------------------------------------
pub const SND_SYNC: DWORD = 0x0000;
pub const SND_ASYNC: DWORD = 0x0001;
pub const SND_NODEFAULT: DWORD = 0x0002;
pub const SND_MEMORY: DWORD = 0x0004;
pub const SND_LOOP: DWORD = 0x0008;
pub const SND_NOSTOP: DWORD = 0x0010;
pub const SND_NOWAIT: DWORD = 0x00002000;
pub const SND_ALIAS: DWORD = 0x00010000;
pub const SND_ALIAS_ID: DWORD = 0x00110000;
pub const SND_FILENAME: DWORD = 0x00020000;
pub const SND_RESOURCE: DWORD = 0x00040004;
pub const SND_PURGE: DWORD = 0x0040;
pub const SND_APPLICATION: DWORD = 0x0080;
pub const SND_SENTRY: DWORD = 0x00080000;
pub const SND_RING: DWORD = 0x00100000;
pub const SND_SYSTEM: DWORD = 0x00200000;

//-------------------------------------------------------------------------------------------------
// winreg.h
// Registry API procedure declarations, constant definitions and macros
//-------------------------------------------------------------------------------------------------

pub type REGSAM = ACCESS_MASK;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct VALENTA {
    pub ve_valuename: LPSTR,
    pub ve_valuelen: DWORD,
    pub ve_valueptr: DWORD_PTR,
    pub ve_type: DWORD,
}
pub type PVALENTA = *mut VALENTA;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct VALENTW {
    pub ve_valuename: LPWSTR,
    pub ve_valuelen: DWORD,
    pub ve_valueptr: DWORD_PTR,
    pub ve_type: DWORD,
}
pub type PVALENTW = *mut VALENTW;

pub const HKEY_CLASSES_ROOT: HKEY = 0x80000000 as HKEY;
pub const HKEY_CURRENT_USER: HKEY = 0x80000001 as HKEY;
pub const HKEY_LOCAL_MACHINE: HKEY = 0x80000002 as HKEY;
pub const HKEY_USERS: HKEY = 0x80000003 as HKEY;
pub const HKEY_PERFORMANCE_DATA: HKEY = 0x80000004 as HKEY;
pub const HKEY_PERFORMANCE_TEXT: HKEY = 0x80000050 as HKEY;
pub const HKEY_PERFORMANCE_NLSTEXT: HKEY = 0x80000060 as HKEY;
pub const HKEY_CURRENT_CONFIG: HKEY = 0x80000005 as HKEY;
pub const HKEY_DYN_DATA: HKEY = 0x80000006 as HKEY;
pub const HKEY_CURRENT_USER_LOCAL_SETTINGS: HKEY = 0x80000007 as HKEY;

pub const REG_MUI_STRING_TRUNCATE: DWORD = 0x00000001;

pub const RRF_RT_REG_NONE: DWORD = 0x00000001;
pub const RRF_RT_REG_SZ: DWORD = 0x00000002;
pub const RRF_RT_REG_EXPAND_SZ: DWORD = 0x00000004;
pub const RRF_RT_REG_BINARY: DWORD = 0x00000008;
pub const RRF_RT_REG_DWORD: DWORD = 0x00000010;
pub const RRF_RT_REG_MULTI_SZ: DWORD = 0x00000020;
pub const RRF_RT_REG_QWORD: DWORD = 0x00000040;

pub const RRF_RT_DWORD: DWORD = RRF_RT_REG_BINARY|RRF_RT_REG_DWORD;
pub const RRF_RT_QWORD: DWORD = RRF_RT_REG_BINARY|RRF_RT_REG_QWORD;
pub const RRF_RT_ANY: DWORD = 0x0000ffff;

pub const RRF_NOEXPAND: DWORD = 0x10000000;
pub const RRF_ZEROONFAILURE: DWORD = 0x20000000;

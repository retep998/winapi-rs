// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! Types and constants for WinAPI bindings.
#![allow(bad_style)]
#![warn(trivial_casts, trivial_numeric_casts)]
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
pub use audiosessiontypes::*;
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
pub use d3d12::*;
pub use d3d12sdklayers::*;
pub use d3dcommon::*;
pub use dbghelp::*;
pub use dcommon::*;
pub use devpropdef::*;
pub use docobj::*;
pub use dpapi::*;
pub use dsgetdc::*;
pub use dsound::*;
pub use dsrole::*;
pub use dwmapi::*;
pub use dwrite::*;
pub use dxgi::*;
pub use dxgi1_2::*;
pub use dxgi1_3::*;
pub use dxgi1_4::*;
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
pub use lmaccess::*;
pub use lmcons::*;
pub use lmdfs::*;
pub use lmerrlog::*;
pub use lmjoin::*;
pub use lsalookup::*;
pub use memoryapi::*;
pub use minwinbase::*;
pub use minwindef::*;
pub use mmdeviceapi::*;
pub use mmreg::*;
pub use mmsystem::*;
pub use mscat::*;
pub use mssip::*;
pub use nb30::*;
pub use ncrypt::*;
pub use ntdef::*;
pub use ntsecapi::*;
pub use ntstatus::*;
pub use oaidl::*;
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
pub use rpc::*;
pub use rpcdce::*;
pub use sapi::*;
pub use schannel::*;
pub use servprov::*;
pub use setupapi::*;
pub use shellapi::*;
pub use shellscalingapi::*;
pub use shlguid::*;
pub use shlobj::*;
pub use shobjidl::*;
pub use shtypes::*;
pub use spapidef::*;
pub use sqltypes::*;
pub use sspi::*;
pub use subauth::*;
pub use synchapi::*;
pub use sysinfoapi::*;
pub use threadpoolapi::*;
pub use timezoneapi::*;
pub use tlhelp32::*;
pub use unknwnbase::*;
pub use urlhist::*;
pub use urlmon::*;
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
pub use winevt::*;
pub use wingdi::*;
pub use winhttp::*;
pub use winioctl::*;
pub use winnetwk::*;
pub use winnls::*;
pub use winnt::*;
pub use winscard::*;
pub use winsmcrd::*;
pub use winsock2::*;
pub use winspool::*;
pub use winsvc::*;
pub use winuser::*;
pub use ws2def::*;
pub use ws2ipdef::*;
pub use ws2spi::*;
pub use ws2tcpip::*;
pub use wtypes::*;
pub use wtypesbase::*;
pub use xinput::*;
//-------------------------------------------------------------------------------------------------
// Modules
//-------------------------------------------------------------------------------------------------
#[macro_use] mod macros;
pub mod audioclient;
pub mod audiosessiontypes;
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
pub mod d3d12;
pub mod d3d12sdklayers;
pub mod d3dcommon;
pub mod dbghelp;
pub mod dcommon;
pub mod devpropdef;
pub mod docobj;
pub mod dpapi;
pub mod dsgetdc;
pub mod dsound;
pub mod dsrole;
pub mod dwmapi;
pub mod dwrite;
pub mod dxgi;
pub mod dxgi1_2;
pub mod dxgi1_3;
pub mod dxgi1_4;
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
pub mod lmaccess;
pub mod lmcons;
pub mod lmdfs;
pub mod lmerrlog;
pub mod lmjoin;
pub mod lsalookup;
pub mod memoryapi;
pub mod minwinbase;
pub mod minwindef;
pub mod mmdeviceapi;
pub mod mmreg;
pub mod mmsystem;
pub mod mscat;
pub mod mssip;
pub mod nb30;
pub mod ncrypt;
pub mod ntdef;
pub mod ntsecapi;
pub mod ntstatus;
pub mod oaidl;
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
pub mod rpc;
pub mod rpcdce;
pub mod sapi;
pub mod schannel;
pub mod servprov;
pub mod setupapi;
pub mod shellapi;
pub mod shellscalingapi;
pub mod shlguid;
pub mod shlobj;
pub mod shobjidl;
pub mod shtypes;
pub mod spapidef;
pub mod sqltypes;
pub mod sspi;
pub mod subauth;
pub mod synchapi;
pub mod sysinfoapi;
pub mod threadpoolapi;
pub mod timezoneapi;
pub mod tlhelp32;
pub mod unknwnbase;
pub mod urlhist;
pub mod urlmon;
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
pub mod winevt;
pub mod wingdi;
pub mod winhttp;
pub mod winioctl;
pub mod winnetwk;
pub mod winnls;
pub mod winnt;
pub mod winscard;
pub mod winsmcrd;
pub mod winsock2;
pub mod winspool;
pub mod winsvc;
pub mod winuser;
pub mod ws2def;
pub mod ws2ipdef;
pub mod ws2spi;
pub mod ws2tcpip;
pub mod wtypes;
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
// strmif.h
//-------------------------------------------------------------------------------------------------
pub type REFERENCE_TIME = LONGLONG;
//-------------------------------------------------------------------------------------------------
// propidl.h
//-------------------------------------------------------------------------------------------------
STRUCT!{struct PROPVARIANT {
    vt: VARTYPE,
    wReserved1: WORD,
    wReserved2: WORD,
    wReserved3: WORD,
    data: [u8; 16],
}}
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
STRUCT!{struct ServerInformation {
    dwServerPid: DWORD,
    dwServerTid: DWORD,
    ui64ServerAddress: UINT64,
}}
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
STRUCT!{struct VALENTA {
    ve_valuename: LPSTR,
    ve_valuelen: DWORD,
    ve_valueptr: DWORD_PTR,
    ve_type: DWORD,
}}
pub type PVALENTA = *mut VALENTA;
STRUCT!{struct VALENTW {
    ve_valuename: LPWSTR,
    ve_valuelen: DWORD,
    ve_valueptr: DWORD_PTR,
    ve_type: DWORD,
}}
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

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
pub use activation::*;
pub use audioclient::*;
pub use audiosessiontypes::*;
pub use basetsd::*;
pub use bcrypt::*;
pub use cfg::*;
pub use cfgmgr32::*;
pub use combaseapi::*;
pub use commctrl::*;
pub use commdlg::*;
pub use corsym::*;
pub use d2d1::*;
pub use d2dbasetypes::*;
pub use d3d9::*;
pub use d3d9caps::*;
pub use d3d9types::*;
pub use d3d11::*;
pub use d3d10shader::*;
pub use d3d11shader::*;
pub use d3d12::*;
pub use d3d12sdklayers::*;
pub use d3d12shader::*;
pub use d3dcommon::*;
pub use d3dcompiler::*;
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
pub use minschannel::*;
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
pub use pdh::*;
pub use playsoundapi::*;
pub use processsnapshot::*;
pub use processthreadsapi::*;
pub use propidl::*;
pub use propsys::*;
pub use prsht::*;
pub use psapi::*;
pub use qos::*;
pub use reason::*;
pub use restrictederrorinfo::*;
pub use roapi::*;
pub use roerrorapi::*;
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
pub use strmif::*;
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
pub use winreg::*;
pub use winscard::*;
pub use winsmcrd::*;
pub use winsock2::*;
pub use winspool::*;
pub use winstring::*;
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
pub mod activation;
pub mod audioclient;
pub mod audiosessiontypes;
pub mod basetsd;
pub mod bcrypt;
pub mod cfg;
pub mod cfgmgr32;
pub mod combaseapi;
pub mod commctrl;
pub mod commdlg;
pub mod corsym;
pub mod d2d1;
pub mod d2dbasetypes;
pub mod d3d9;
pub mod d3d9caps;
pub mod d3d9types;
pub mod d3d11;
pub mod d3d10shader;
pub mod d3d11shader;
pub mod d3d12;
pub mod d3d12sdklayers;
pub mod d3d12shader;
pub mod d3dcommon;
pub mod d3dcompiler;
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
pub mod minschannel;
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
pub mod pdh;
pub mod playsoundapi;
pub mod processsnapshot;
pub mod processthreadsapi;
pub mod propidl;
pub mod propsys;
pub mod prsht;
pub mod psapi;
pub mod qos;
pub mod reason;
pub mod restrictederrorinfo;
pub mod roapi;
pub mod roerrorapi;
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
pub mod strmif;
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
pub mod winreg;
pub mod winscard;
pub mod winsmcrd;
pub mod winsock2;
pub mod winspool;
pub mod winstring;
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

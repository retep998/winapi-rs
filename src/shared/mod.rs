// Copyright © 2016 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Headers shared between user mode and kernel mode
#[cfg(feature = "basetsd")] pub mod basetsd;
#[cfg(feature = "bcrypt")] pub mod bcrypt;
#[cfg(feature = "bugcodes")] pub mod bugcodes;
#[cfg(feature = "cderr")] pub mod cderr;
#[cfg(feature = "cfg")] pub mod cfg;
#[cfg(feature = "d3d9")] pub mod d3d9;
#[cfg(feature = "d3d9caps")] pub mod d3d9caps;
#[cfg(feature = "d3d9types")] pub mod d3d9types;
#[cfg(feature = "devpkey")] pub mod devpkey;
#[cfg(feature = "devpropdef")] pub mod devpropdef;
#[cfg(feature = "devguid")] pub mod devguid;
#[cfg(feature = "dxgi")] pub mod dxgi;
#[cfg(feature = "dxgi1_2")] pub mod dxgi1_2;
#[cfg(feature = "dxgi1_3")] pub mod dxgi1_3;
#[cfg(feature = "dxgi1_4")] pub mod dxgi1_4;
#[cfg(feature = "dxgi1_5")] pub mod dxgi1_5;
#[cfg(feature = "dxgiformat")] pub mod dxgiformat;
#[cfg(feature = "dxgitype")] pub mod dxgitype;
#[cfg(feature = "guiddef")] pub mod guiddef;
#[cfg(feature = "hidclass")] pub mod hidclass;
#[cfg(feature = "hidpi")] pub mod hidpi;
#[cfg(feature = "hidsdi")] pub mod hidsdi;
#[cfg(feature = "hidusage")] pub mod hidusage;
#[cfg(feature = "inaddr")] pub mod inaddr;
#[cfg(feature = "ksmedia")] pub mod ksmedia;
#[cfg(feature = "lmcons")] pub mod lmcons;
#[cfg(feature = "minwindef")] pub mod minwindef;
#[cfg(feature = "mmreg")] pub mod mmreg;
#[cfg(feature = "ntdef")] pub mod ntdef;
#[cfg(feature = "ntstatus")] pub mod ntstatus;
#[cfg(feature = "rpcndr")] pub mod rpcndr;
#[cfg(feature = "sspi")]  pub mod sspi;
#[cfg(feature = "usb")] pub mod usb;
#[cfg(feature = "windef")] pub mod windef;
#[cfg(feature = "windowsx")] pub mod windowsx;
#[cfg(feature = "winerror")] pub mod winerror;
#[cfg(feature = "wtypes")] pub mod wtypes;
#[cfg(feature = "wtypesbase")] pub mod wtypesbase;

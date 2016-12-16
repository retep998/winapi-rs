// Copyright © 2016 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Headers shared between user mode and kernel mode
#[cfg(feature = "shared-basetsd")] pub mod basetsd;
#[cfg(feature = "shared-bcrypt")] pub mod bcrypt;
#[cfg(feature = "shared-cderr")] pub mod cderr;
#[cfg(feature = "shared-cfg")] pub mod cfg;
#[cfg(feature = "shared-d3d9")] pub mod d3d9;
#[cfg(feature = "shared-d3d9caps")] pub mod d3d9caps;
#[cfg(feature = "shared-d3d9types")] pub mod d3d9types;
#[cfg(feature = "shared-devpropdef")] pub mod devpropdef;
#[cfg(feature = "shared-devguid")] pub mod devguid;
#[cfg(feature = "shared-dxgi")] pub mod dxgi;
#[cfg(feature = "shared-dxgi1_2")] pub mod dxgi1_2;
#[cfg(feature = "shared-dxgi1_3")] pub mod dxgi1_3;
#[cfg(feature = "shared-dxgi1_4")] pub mod dxgi1_4;
#[cfg(feature = "shared-dxgiformat")] pub mod dxgiformat;
#[cfg(feature = "shared-dxgitype")] pub mod dxgitype;
#[cfg(feature = "shared-guiddef")] pub mod guiddef;
#[cfg(feature = "shared-hidclass")] pub mod hidclass;
#[cfg(feature = "shared-hidpi")] pub mod hidpi;
#[cfg(feature = "shared-hidsdi")] pub mod hidsdi;
#[cfg(feature = "shared-hidusage")] pub mod hidusage;
#[cfg(feature = "shared-inaddr")] pub mod inaddr;
#[cfg(feature = "shared-ksmedia")] pub mod ksmedia;
#[cfg(feature = "shared-lmcons")] pub mod lmcons;
#[cfg(feature = "shared-minwindef")] pub mod minwindef;
#[cfg(feature = "shared-mmreg")] pub mod mmreg;
#[cfg(feature = "shared-ntdef")] pub mod ntdef;
#[cfg(feature = "shared-ntstatus")] pub mod ntstatus;
#[cfg(feature = "shared-rpcndr")] pub mod rpcndr;
#[cfg(feature = "shared-sspi")]  pub mod sspi;
#[cfg(feature = "shared-usb")] pub mod usb;
#[cfg(feature = "shared-windef")] pub mod windef;
#[cfg(feature = "shared-windowsx")] pub mod windowsx;
#[cfg(feature = "shared-winerror")] pub mod winerror;
#[cfg(feature = "shared-wtypes")] pub mod wtypes;
#[cfg(feature = "shared-wtypesbase")] pub mod wtypesbase;
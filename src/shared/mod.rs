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
#[cfg(feature = "shared-d3d9caps")] pub mod d3d9caps;
#[cfg(feature = "shared-d3d9types")] pub mod d3d9types;
#[cfg(feature = "shared-guiddef")] pub mod guiddef;
#[cfg(feature = "shared-minwindef")] pub mod minwindef;
#[cfg(feature = "shared-ntstatus")] pub mod ntstatus;
#[cfg(feature = "shared-ntdef")] pub mod ntdef;
#[cfg(feature = "shared-rpcndr")] pub mod rpcndr;
#[cfg(feature = "shared-sspi")]  pub mod sspi;
#[cfg(feature = "shared-windef")] pub mod windef;
#[cfg(feature = "shared-wtypesbase")] pub mod wtypesbase;

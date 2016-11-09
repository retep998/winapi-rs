// Copyright © 2016 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Headers for user mode only
#[cfg(feature = "um-audiosessiontypes")] pub mod audiosessiontypes;
#[cfg(feature = "um-cfgmgr32")] pub mod cfgmgr32;
#[cfg(feature = "um-cguid")] pub mod cguid;
#[cfg(feature = "um-combaseapi")] pub mod combaseapi;
#[cfg(feature = "um-commctrl")] pub mod commctrl;
#[cfg(feature = "um-consoleapi")] pub mod consoleapi;
#[cfg(feature = "um-dbghelp")] pub mod dbghelp;
#[cfg(feature = "um-minwinbase")] pub mod minwinbase;
#[cfg(feature = "um-ncrypt")] pub mod ncrypt;
#[cfg(feature = "um-objidlbase")] pub mod objidlbase;
#[cfg(feature = "um-shellapi")] pub mod shellapi;
#[cfg(feature = "um-unknwnbase")] pub mod unknwnbase;
#[cfg(feature = "um-winbase")] pub mod winbase;
#[cfg(feature = "um-wincon")] pub mod wincon;
#[cfg(feature = "um-winevt")] pub mod winevt;
#[cfg(feature = "um-wingdi")] pub mod wingdi;
#[cfg(feature = "um-winioctl")] pub mod winioctl;
#[cfg(feature = "um-wininet")] pub mod wininet;
#[cfg(feature = "um-winineti")] pub mod winineti;
#[cfg(feature = "um-winnt")] pub mod winnt;
#[cfg(feature = "um-winreg")] pub mod winreg;
#[cfg(feature = "um-winuser")] pub mod winuser;
#[cfg(feature = "um-xinput")] pub mod xinput;


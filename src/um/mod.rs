// Copyright © 2016-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Headers for user mode only
pub mod gl;
#[cfg(feature = "audioclient")] pub mod audioclient;
#[cfg(feature = "audiosessiontypes")] pub mod audiosessiontypes;
#[cfg(feature = "avrt")] pub mod avrt;
#[cfg(feature = "cfgmgr32")] pub mod cfgmgr32;
#[cfg(feature = "cguid")] pub mod cguid;
#[cfg(feature = "combaseapi")] pub mod combaseapi;
#[cfg(feature = "coml2api")] pub mod coml2api;
#[cfg(feature = "commapi")] pub mod commapi;
#[cfg(feature = "commctrl")] pub mod commctrl;
#[cfg(feature = "commdlg")] pub mod commdlg;
#[cfg(feature = "consoleapi")] pub mod consoleapi;
#[cfg(feature = "corsym")] pub mod corsym;
#[cfg(feature = "d2d1")] pub mod d2d1;
#[cfg(feature = "d2d1_1")] pub mod d2d1_1;
#[cfg(feature = "d2d1_2")] pub mod d2d1_2;
#[cfg(feature = "d2d1effectauthor")] pub mod d2d1effectauthor;
#[cfg(feature = "d2d1effects")] pub mod d2d1effects;
#[cfg(feature = "d2d1effects_1")] pub mod d2d1effects_1;
#[cfg(feature = "d2dbasetypes")] pub mod d2dbasetypes;
#[cfg(feature = "d3d10")] pub mod d3d10;
#[cfg(feature = "d3d10shader")] pub mod d3d10shader;
#[cfg(feature = "d3d11")] pub mod d3d11;
#[cfg(feature = "d3d11on12")] pub mod d3d11on12;
#[cfg(feature = "d3d11shader")] pub mod d3d11shader;
#[cfg(feature = "d3d12")] pub mod d3d12;
#[cfg(feature = "d3d12sdklayers")] pub mod d3d12sdklayers;
#[cfg(feature = "d3d12shader")] pub mod d3d12shader;
#[cfg(feature = "d3dcommon")] pub mod d3dcommon;
#[cfg(feature = "d3dcompiler")] pub mod d3dcompiler;
#[cfg(feature = "dbghelp")] pub mod dbghelp;
#[cfg(feature = "dcommon")] pub mod dcommon;
#[cfg(feature = "dcomp")] pub mod dcomp;
#[cfg(feature = "dcompanimation")] pub mod dcompanimation;
#[cfg(feature = "docobj")] pub mod docobj;
#[cfg(feature = "documenttarget")] pub mod documenttarget;
#[cfg(feature = "dpapi")] pub mod dpapi;
#[cfg(feature = "dsgetdc")] pub mod dsgetdc;
#[cfg(feature = "dsound")] pub mod dsound;
#[cfg(feature = "dsrole")] pub mod dsrole;
#[cfg(feature = "dwmapi")] pub mod dwmapi;
#[cfg(feature = "dwrite")] pub mod dwrite;
#[cfg(feature = "dwrite_1")] pub mod dwrite_1;
#[cfg(feature = "dwrite_2")] pub mod dwrite_2;
#[cfg(feature = "dwrite_3")] pub mod dwrite_3;
#[cfg(feature = "errhandlingapi")] pub mod errhandlingapi;
#[cfg(feature = "fileapi")] pub mod fileapi;
#[cfg(feature = "handleapi")] pub mod handleapi;
#[cfg(feature = "heapapi")] pub mod heapapi;
#[cfg(feature = "http")] pub mod http;
#[cfg(feature = "imm")] pub mod imm;
#[cfg(feature = "libloaderapi")] pub mod libloaderapi;
#[cfg(feature = "lmaccess")] pub mod lmaccess;
#[cfg(feature = "lmdfs")] pub mod lmdfs;
#[cfg(feature = "lmerrlog")] pub mod lmerrlog;
#[cfg(feature = "lmjoin")] pub mod lmjoin;
#[cfg(feature = "lsalookup")] pub mod lsalookup;
#[cfg(feature = "memoryapi")] pub mod memoryapi;
#[cfg(feature = "minschannel")] pub mod minschannel;
#[cfg(feature = "minwinbase")] pub mod minwinbase;
#[cfg(feature = "mmdeviceapi")] pub mod mmdeviceapi;
#[cfg(feature = "mmsystem")] pub mod mmsystem;
#[cfg(feature = "mscat")] pub mod mscat;
#[cfg(feature = "mssip")] pub mod mssip;
#[cfg(feature = "nb30")] pub mod nb30;
#[cfg(feature = "ncrypt")] pub mod ncrypt;
#[cfg(feature = "ntsecapi")] pub mod ntsecapi;
#[cfg(feature = "oaidl")] pub mod oaidl;
#[cfg(feature = "objbase")] pub mod objbase;
#[cfg(feature = "objidl")] pub mod objidl;
#[cfg(feature = "objidlbase")] pub mod objidlbase;
#[cfg(feature = "ocidl")] pub mod ocidl;
#[cfg(feature = "oleauto")] pub mod oleauto;
#[cfg(feature = "olectl")] pub mod olectl;
#[cfg(feature = "pdh")] pub mod pdh;
#[cfg(feature = "playsoundapi")] pub mod playsoundapi;
#[cfg(feature = "processsnapshot")] pub mod processsnapshot;
#[cfg(feature = "processthreadsapi")] pub mod processthreadsapi;
#[cfg(feature = "propidl")] pub mod propidl;
#[cfg(feature = "propkeydef")] pub mod propkeydef;
#[cfg(feature = "propsys")] pub mod propsys;
#[cfg(feature = "prsht")] pub mod prsht;
#[cfg(feature = "psapi")] pub mod psapi;
#[cfg(feature = "reason")] pub mod reason;
#[cfg(feature = "restrictederrorinfo")] pub mod restrictederrorinfo;
#[cfg(feature = "sapi")] pub mod sapi;
#[cfg(feature = "sapi51")] pub mod sapi51;
#[cfg(feature = "sapi53")] pub mod sapi53;
#[cfg(feature = "sapiddk")] pub mod sapiddk;
#[cfg(feature = "sapiddk51")] pub mod sapiddk51;
#[cfg(feature = "schannel")] pub mod schannel;
#[cfg(feature = "servprov")] pub mod servprov;
#[cfg(feature = "setupapi")] pub mod setupapi;
#[cfg(feature = "shellapi")] pub mod shellapi;
#[cfg(feature = "shellscalingapi")] pub mod shellscalingapi;
#[cfg(feature = "shlobj")] pub mod shlobj;
#[cfg(feature = "shobjidl")] pub mod shobjidl;
#[cfg(feature = "shobjidl_core")] pub mod shobjidl_core;
#[cfg(feature = "shtypes")] pub mod shtypes;
#[cfg(feature = "spapidef")] pub mod spapidef;
#[cfg(feature = "sql")] pub mod sql;
#[cfg(feature = "sqlext")] pub mod sqlext;
#[cfg(feature = "sqltypes")] pub mod sqltypes;
#[cfg(feature = "sqlucode")] pub mod sqlucode;
#[cfg(feature = "sspi")] pub mod sspi;
#[cfg(feature = "strmif")] pub mod strmif;
#[cfg(feature = "subauth")] pub mod subauth;
#[cfg(feature = "synchapi")] pub mod synchapi;
#[cfg(feature = "sysinfoapi")] pub mod sysinfoapi;
#[cfg(feature = "threadpoolapiset")] pub mod threadpoolapiset;
#[cfg(feature = "timezoneapi")] pub mod timezoneapi;
#[cfg(feature = "tlhelp32")] pub mod tlhelp32;
#[cfg(feature = "unknwnbase")] pub mod unknwnbase;
#[cfg(feature = "urlhist")] pub mod urlhist;
#[cfg(feature = "urlmon")] pub mod urlmon;
#[cfg(feature = "usbspec")] pub mod usbspec;
#[cfg(feature = "usp10")] pub mod usp10;
#[cfg(feature = "vsbackup")] pub mod vsbackup;
#[cfg(feature = "vss")] pub mod vss;
#[cfg(feature = "vsserror")] pub mod vsserror;
#[cfg(feature = "vswriter")] pub mod vswriter;
#[cfg(feature = "werapi")] pub mod werapi;
#[cfg(feature = "winbase")] pub mod winbase;
#[cfg(feature = "wincodec")] pub mod wincodec;
#[cfg(feature = "wincon")] pub mod wincon;
#[cfg(feature = "wincred")] pub mod wincred;
#[cfg(feature = "wincrypt")] pub mod wincrypt;
#[cfg(feature = "winevt")] pub mod winevt;
#[cfg(feature = "wingdi")] pub mod wingdi;
#[cfg(feature = "winhttp")] pub mod winhttp;
#[cfg(feature = "wininet")] pub mod wininet;
#[cfg(feature = "winineti")] pub mod winineti;
#[cfg(feature = "winioctl")] pub mod winioctl;
#[cfg(feature = "winnetwk")] pub mod winnetwk;
#[cfg(feature = "winnls")] pub mod winnls;
#[cfg(feature = "winnt")] pub mod winnt;
#[cfg(feature = "winreg")] pub mod winreg;
#[cfg(feature = "winscard")] pub mod winscard;
#[cfg(feature = "winsmcrd")] pub mod winsmcrd;
#[cfg(feature = "winsock2")] pub mod winsock2;
#[cfg(feature = "winspool")] pub mod winspool;
#[cfg(feature = "winsvc")] pub mod winsvc;
#[cfg(feature = "winusb")] pub mod winusb;
#[cfg(feature = "winusbio")] pub mod winusbio;
#[cfg(feature = "winuser")] pub mod winuser;
#[cfg(feature = "ws2spi")] pub mod ws2spi;
#[cfg(feature = "ws2tcpip")] pub mod ws2tcpip;
#[cfg(feature = "xinput")] pub mod xinput;

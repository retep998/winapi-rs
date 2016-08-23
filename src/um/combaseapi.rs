// Copyright © 2016 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Base Component Object Model defintions.
#![cfg(feature = "um.combaseapi")]
use shared::basetsd::UINT64;
use shared::minwindef::DWORD;
use shared::wtypesbase::{
    CLSCTX, CLSCTX_INPROC_HANDLER, CLSCTX_INPROC_SERVER, CLSCTX_LOCAL_SERVER, CLSCTX_REMOTE_SERVER,
};
#[cfg(feature = "ole32")]
use um::objidlbase::LPMALLOC;
#[cfg(feature = "ole32")]
use um::winnt::HRESULT;
pub const CLSCTX_INPROC: CLSCTX = CLSCTX_INPROC_SERVER | CLSCTX_INPROC_HANDLER;
pub const CLSCTX_ALL: CLSCTX = CLSCTX_INPROC_SERVER | CLSCTX_INPROC_HANDLER | CLSCTX_LOCAL_SERVER
    | CLSCTX_REMOTE_SERVER;
pub const CLSCTX_SERVER: CLSCTX = CLSCTX_INPROC_SERVER | CLSCTX_LOCAL_SERVER
    | CLSCTX_REMOTE_SERVER;
ENUM!{enum REGCLS {
    REGCLS_SINGLEUSE = 0,
    REGCLS_MULTIPLEUSE = 1,
    REGCLS_MULTI_SEPARATE = 2,
    REGCLS_SUSPENDED = 4,
    REGCLS_SURROGATE = 8,
    REGCLS_AGILE = 0x10,
}}
ENUM!{enum COINITBASE {
    COINITBASE_MULTITHREADED = 0x0,
}}
EXTERN!{"ole32" "stdcall" fn CoGetMalloc(
    dwMemContext: DWORD, ppMalloc: *mut LPMALLOC
) -> HRESULT}
STRUCT!{struct ServerInformation {
    dwServerPid: DWORD,
    dwServerTid: DWORD,
    ui64ServerAddress: UINT64,
}}
pub type PServerInformation = *mut ServerInformation;
DECLARE_HANDLE!(CO_MTA_USAGE_COOKIE, CO_MTA_USAGE_COOKIE__);

// Copyright © 2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
ENUM!{enum RO_INIT_TYPE {
    RO_INIT_SINGLETHREADED = 0,
    RO_INIT_MULTITHREADED = 1,
}}

pub enum RO_REGISTRATION_COOKIE__ {}
pub type RO_REGISTRATION_COOKIE = *mut RO_REGISTRATION_COOKIE__;

pub type PFNGETACTIVATIONFACTORY = Option<unsafe extern "system" fn(
    ::HSTRING, *mut *mut ::IActivationFactory,
) -> ::HRESULT>;

DECLARE_HANDLE!(APARTMENT_SHUTDOWN_REGISTRATION_COOKIE, APARTMENT_SHUTDOWN_REGISTRATION_COOKIE__);

// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! IPv4 Internet address
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IN_ADDR(pub ::ULONG);
pub type PIN_ADDR = *mut IN_ADDR;
pub type LPIN_ADDR = *mut IN_ADDR;

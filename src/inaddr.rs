// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! IPv4 Internet address
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct in_addr(pub ::ULONG);
pub type IN_ADDR = in_addr;
pub type PIN_ADDR = *mut in_addr;
pub type LPIN_ADDR = *mut in_addr;

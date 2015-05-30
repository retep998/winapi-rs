// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! Basic Windows Type Definitions
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct POINTL {
    pub x: ::LONG,
    pub y: ::LONG,
}
pub type PPOINTL = *mut POINTL;

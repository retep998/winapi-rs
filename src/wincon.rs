// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! This module contains the public data structures, data types, and procedures exported by the NT
//! console subsystem.
#[repr(C)] #[derive(Copy)] pub struct COORD {
    pub X: ::SHORT,
    pub Y: ::SHORT,
}
pub type PCOORD = *mut COORD;
#[repr(C)] #[derive(Copy)] pub struct SMALL_RECT {
    pub Left: ::SHORT,
    pub Top: ::SHORT,
    pub Right: ::SHORT,
    pub Bottom: ::SHORT,
}
pub type PSMALL_RECT = *mut SMALL_RECT;
#[repr(C)] #[derive(Copy)] pub struct CONSOLE_SCREEN_BUFFER_INFO {
    pub dwSize: COORD,
    pub dwCursorPosition: COORD,
    pub wAttributes: ::WORD,
    pub srWindow: SMALL_RECT,
    pub dwMaximumWindowSize: COORD,
}
pub type PCONSOLE_SCREEN_BUFFER_INFO = *mut CONSOLE_SCREEN_BUFFER_INFO;

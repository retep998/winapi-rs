// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
#[repr(C)] #[derive(Clone, Copy, Debug)] #[allow(unused_qualifications)]
pub enum EXCEPTION_DISPOSITION {
    ExceptionContinueExecution = 0,
    ExceptionContinueSearch = 1,
    ExceptionNestedException = 2,
    ExceptionCollidedUnwind = 3,
}
pub use self::EXCEPTION_DISPOSITION::*;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct _EXCEPTION_RECORD;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct _CONTEXT;
#[cfg(target_arch = "x86_64")] #[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct _DISPATCHER_CONTEXT;

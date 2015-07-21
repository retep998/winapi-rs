// Copyright © 2015, skdltmxn
// Licensed under the MIT License <LICENSE.md>
//! Function prototypes for Windows Error Reporting (WER)
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum WER_REGISTER_FILE_TYPE {
    WerRegFileTypeUserDocument = 1,
    WerRegFileTypeOther = 2,
    WerRegFileTypeMax,
}

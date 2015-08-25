// Copyright © 2015, skdltmxn
// Licensed under the MIT License <LICENSE.md>
#![cfg(windows)]
extern crate comdlg32;
use comdlg32::*;
#[inline(never)] fn bb<T>(_: T) {}
#[test]
fn functions() {
    bb(ChooseColorA);
    bb(ChooseColorW);
    bb(ChooseFontA);
    bb(ChooseFontW);
    bb(CommDlgExtendedError);
    bb(FindTextA);
    bb(FindTextW);
    bb(GetFileTitleA);
    bb(GetFileTitleW);
    bb(GetOpenFileNameA);
    bb(GetOpenFileNameW);
    bb(GetSaveFileNameA);
    bb(GetSaveFileNameW);
    bb(PageSetupDlgA);
    bb(PageSetupDlgW);
    bb(PrintDlgA);
    bb(PrintDlgExA);
    bb(PrintDlgExW);
    bb(PrintDlgW);
    bb(ReplaceTextA);
    bb(ReplaceTextW);
}

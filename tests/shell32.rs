// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
#![cfg(windows)]
extern crate shell32;
use shell32::*;
#[inline(never)] fn bb<T>(_: T) {}
#[test]
fn functions() {
    bb(SHCloneSpecialIDList);
    bb(SHCreateDirectory);
    bb(SHCreateDirectoryExA);
    bb(SHCreateDirectoryExW);
    bb(SHCreateShellItem);
    bb(SHFlushSFCache);
    bb(SHGetFolderLocation);
    bb(SHGetFolderPathA);
    bb(SHGetFolderPathAndSubDirA);
    bb(SHGetFolderPathAndSubDirW);
    bb(SHGetFolderPathW);
    bb(SHGetIconOverlayIndexA);
    bb(SHGetIconOverlayIndexW);
    bb(SHGetKnownFolderIDList);
    bb(SHGetKnownFolderItem);
    bb(SHGetKnownFolderPath);
    bb(SHGetPathFromIDListA);
    bb(SHGetPathFromIDListEx);
    bb(SHGetPathFromIDListW);
    bb(SHGetSpecialFolderLocation);
    bb(SHGetSpecialFolderPathA);
    bb(SHGetSpecialFolderPathW);
    bb(SHOpenFolderAndSelectItems);
    bb(SHSetFolderPathA);
    bb(SHSetFolderPathW);
    bb(SHSetKnownFolderPath);
}

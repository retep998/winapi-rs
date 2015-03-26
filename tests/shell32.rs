// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
#![feature(test)]
extern crate shell32;
extern crate test;
use shell32::*;
use test::black_box as bb;
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

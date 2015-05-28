// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! this ALWAYS GENERATED file contains the definitions for the interfaces
//4498
pub type SFGAOF = ::ULONG;
//9466
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum SIGDN {
    NORMALDISPLAY = 0,
    PARENTRELATIVEPARSING = 0x80018001u32 as i32,
    DESKTOPABSOLUTEPARSING = 0x80028000u32 as i32,
    PARENTRELATIVEEDITING = 0x80031001u32 as i32,
    DESKTOPABSOLUTEEDITING = 0x8004c000u32 as i32,
    FILESYSPATH = 0x80058000u32 as i32,
    URL = 0x80068000u32 as i32,
    PARENTRELATIVEFORADDRESSBAR = 0x8007c001u32 as i32,
    PARENTRELATIVE = 0x80080001u32 as i32,
    PARENTRELATIVEFORUI = 0x80094001u32 as i32,
}
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum SICHINTF {
    DISPLAY = 0,
    ALLFIELDS = 0x80000000u32 as i32,
    CANONICAL = 0x10000000,
    TEST_FILESYSPATH_IF_NOT_EQUAL = 0x20000000,
}
//9498
RIDL!(
interface IShellItem(IShellItemVtbl): IUnknown(IUnknownVtbl) {
    fn BindToHandler(
        &mut self, pbc: *mut ::IBindCtx, bhid: ::REFGUID, riid: ::REFIID, ppv: *mut *mut ::c_void
    ) -> ::HRESULT,
    fn GetParent(&mut self, ppsi: *mut *mut IShellItem) -> ::HRESULT,
    fn GetDisplayName(&mut self, sigdnName: SIGDN, ppszName: *mut ::LPWSTR) -> ::HRESULT,
    fn GetAttributes(&mut self, sfgaoMask: SFGAOF, psfgaoAttribs: *mut SFGAOF) -> ::HRESULT,
    fn Compare(&mut self, psi: *mut IShellItem, hint: SICHINTF, piOrder: *mut ::c_int) -> ::HRESULT
}
);
//20869
RIDL!(
interface IModalWindow(IModalWindowVtbl): IUnknown(IUnknownVtbl) {
    fn Show(&mut self, hwndOwner: ::HWND) -> ::HRESULT
}
);
//22307
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum FDAP {
    BOTTOM = 0,
    TOP = 1,
}
pub type IFileDialogEvents = ::IUnknown; // TODO
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum FILEOPENDIALOGOPTIONS {
    OVERWRITEPROMPT = 0x2,
    STRICTFILETYPES = 0x4,
    NOCHANGEDIR = 0x8,
    PICKFOLDERS = 0x20,
    FORCEFILESYSTEM = 0x40,
    ALLNONSTORAGEITEMS = 0x80,
    NOVALIDATE = 0x100,
    ALLOWMULTISELECT = 0x200,
    PATHMUSTEXIST = 0x800,
    FILEMUSTEXIST = 0x1000,
    CREATEPROMPT = 0x2000,
    SHAREAWARE = 0x4000,
    NOREADONLYRETURN = 0x8000,
    NOTESTFILECREATE = 0x10000,
    HIDEMRUPLACES = 0x20000,
    HIDEPINNEDPLACES = 0x40000,
    NODEREFERENCELINKS = 0x100000,
    DONTADDTORECENT = 0x2000000,
    FORCESHOWHIDDEN = 0x10000000,
    DEFAULTNOMINIMODE = 0x20000000,
    FORCEPREVIEWPANEON = 0x40000000,
    SUPPORTSTREAMABLEITEMS = 0x80000000u32 as i32,
}
RIDL!(
interface IFileDialog(IFileDialogVtbl): IModalWindow(IModalWindowVtbl) {
    fn SetFileTypes(
        &mut self, cFileTypes: ::UINT, rgFilterSpec: *const ::COMDLG_FILTERSPEC
    ) -> ::HRESULT,
    fn SetFileTypeIndex(&mut self, iFileType: ::UINT) -> ::HRESULT,
    fn GetFileTypeIndex(&mut self, piFileType: *mut ::UINT) -> ::HRESULT,
    fn Advise(&mut self, pfde: *mut IFileDialogEvents, pdwCookie: *mut ::DWORD) -> ::HRESULT,
    fn Unadvise(&mut self, dwCookie: ::DWORD) -> ::HRESULT,
    fn SetOptions(&mut self, fos: FILEOPENDIALOGOPTIONS) -> ::HRESULT,
    fn GetOptions(&mut self, pfos: *mut FILEOPENDIALOGOPTIONS) -> ::HRESULT,
    fn SetDefaultFolder(&mut self, psi: *mut IShellItem) -> ::HRESULT,
    fn SetFolder(&mut self, psi: *mut IShellItem) -> ::HRESULT,
    fn GetFolder(&mut self, ppsi: *mut *mut IShellItem) -> ::HRESULT,
    fn GetCurrentSelection(&mut self, ppsi: *mut *mut IShellItem) -> ::HRESULT,
    fn SetFileName(&mut self, pszName: ::LPCWSTR) -> ::HRESULT,
    fn GetFileName(&mut self, pszName: *mut ::LPWSTR) -> ::HRESULT,
    fn SetTitle(&mut self, pszTitle: ::LPCWSTR) -> ::HRESULT,
    fn SetOkButtonLabel(&mut self, pszText: ::LPCWSTR) -> ::HRESULT,
    fn SetFileNameLabel(&mut self, pszLabel: ::LPCWSTR) -> ::HRESULT,
    fn GetResult(&mut self, ppsi: *mut *mut IShellItem) -> ::HRESULT,
    fn AddPlace(&mut self, psi: *mut IShellItem, fdap: FDAP) -> ::HRESULT,
    fn SetDefaultExtension(&mut self, pszDefaultExtension: ::LPCWSTR) -> ::HRESULT,
    fn Close(&mut self, hr: ::HRESULT) -> ::HRESULT,
    fn SetClientGuid(&mut self, guid: ::REFGUID) -> ::HRESULT,
    fn ClearClientData(&mut self) -> ::HRESULT,
    fn SetFilter(&mut self, pFilter: *mut IShellItemFilter) -> ::HRESULT
}
);
//27457
pub type IShellItemFilter = ::IUnknown; // TODO

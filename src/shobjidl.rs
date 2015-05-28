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
//11963
pub type IFileOperationProgressSink = ::IUnknown; // TODO
pub type IShellItemArray = ::IUnknown; // TODO
//20869
RIDL!(
interface IModalWindow(IModalWindowVtbl): IUnknown(IUnknownVtbl) {
    fn Show(&mut self, hwndOwner: ::HWND) -> ::HRESULT
}
);
//22307
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum FDE_OVERWRITE_RESPONSE {
    DEFAULT = 0,
    ACCEPT = 1,
    REFUSE = 2,
}
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum FDE_SHAREVIOLATION_RESPONSE {
    DEFAULT = 0,
    ACCEPT = 1,
    REFUSE = 2,
}
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum FDAP {
    BOTTOM = 0,
    TOP = 1,
}
RIDL!(
interface IFileDialogEvents(IFileDialogEventsVtbl): IUnknown(IUnknownVtbl) {
    fn OnFileOk(&mut self, pfd: *mut IFileDialog) -> ::HRESULT,
    fn OnFolderChanging(&mut self, pfd: *mut IFileDialog, psiFolder: *mut IShellItem) -> ::HRESULT,
    fn OnFolderChange(&mut self, pfd: *mut IFileDialog) -> ::HRESULT,
    fn OnSelectionChange(&mut self, pfd: *mut IFileDialog) -> ::HRESULT,
    fn OnShareViolation(
        &mut self, pfd: *mut IFileDialog, psi: *mut IShellItem,
        pResponse: *mut FDE_SHAREVIOLATION_RESPONSE
    ) -> ::HRESULT,
    fn OnTypeChange(&mut self, pfd: *mut IFileDialog) -> ::HRESULT,
    fn OnOverwrite(
        &mut self, pfd: *mut IFileDialog, psi: *mut IShellItem,
        pResponse: *mut FDE_OVERWRITE_RESPONSE
    ) -> ::HRESULT
}
);
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
RIDL!(
interface IFileSaveDialog(IFileSaveDialogVtbl): IFileDialog(IFileDialogVtbl) {
    fn SetSaveAsItem(&mut self, psi: *mut IShellItem) -> ::HRESULT,
    fn SetProperties(&mut self, pStore: *mut ::IPropertyStore) -> ::HRESULT,
    fn SetCollectedProperties(
        &mut self, pList: *mut ::IPropertyDescriptionList, fAppendDefault: ::BOOL
    ) -> ::HRESULT,
    fn GetProperties(&mut self, ppStore: *mut *mut ::IPropertyStore) -> ::HRESULT,
    fn ApplyProperties(
        &mut self, psi: *mut IShellItem, pStore: *mut ::IPropertyStore, hwnd: ::HWND,
        pSink: *mut IFileOperationProgressSink
    ) -> ::HRESULT
}
);
RIDL!(
interface IFileOpenDialog(IFileOpenDialogVtbl): IFileDialog(IFileDialogVtbl) {
    fn GetResults(&mut self, ppenum: *mut *mut IShellItemArray) -> ::HRESULT,
    fn GetSelectedItems(&mut self, ppsai: *mut *mut IShellItemArray) -> ::HRESULT
}
);
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum CDCONTROLSTATEF {
    INACTIVE = 0x00000000,
    ENABLED = 0x00000001,
    VISIBLE = 0x00000002,
    ENABLEDVISIBLE = 0x00000003,
}
RIDL!(
interface IFileDialogCustomize(IFileDialogCustomizeVtbl): IUnknown(IUnknownVtbl) {
    fn EnableOpenDropDown(&mut self, dwIDCtl: ::DWORD) -> ::HRESULT,
    fn AddMenu(&mut self, dwIDCtl: ::DWORD, pszLabel: ::LPCWSTR) -> ::HRESULT,
    fn AddPushButton(&mut self, dwIDCtl: ::DWORD, pszLabel: ::LPCWSTR) -> ::HRESULT,
    fn AddComboBox(&mut self, dwIDCtl: ::DWORD) -> ::HRESULT,
    fn AddRadioButtonList(&mut self, dwIDCtl: ::DWORD) -> ::HRESULT,
    fn AddCheckButton(
        &mut self, dwIDCtl: ::DWORD, pszLabel: ::LPCWSTR, bChecked: ::BOOL
    ) -> ::HRESULT,
    fn AddEditBox(&mut self, dwIDCtl: ::DWORD, pszText: ::LPCWSTR) -> ::HRESULT,
    fn AddSeparator(&mut self, dwIDCtl: ::DWORD) -> ::HRESULT,
    fn AddText(&mut self, dwIDCtl: ::DWORD, pszText: ::LPCWSTR) -> ::HRESULT,
    fn SetControlLabel(&mut self, dwIDCtl: ::DWORD, pszLabel: ::LPCWSTR) -> ::HRESULT,
    fn GetControlState(&mut self, dwIDCtl: ::DWORD, pdwState: *mut CDCONTROLSTATEF) -> ::HRESULT,
    fn SetControlState(&mut self, dwIDCtl: ::DWORD, dwState: CDCONTROLSTATEF) -> ::HRESULT,
    fn GetEditBoxText(&mut self, dwIDCtl: ::DWORD, ppszText: *mut *mut ::WCHAR) -> ::HRESULT,
    fn SetEditBoxText(&mut self, dwIDCtl: ::DWORD, pszText: ::LPCWSTR) -> ::HRESULT,
    fn GetCheckButtonState(&mut self, dwIDCtl: ::DWORD, pbChecked: *mut ::BOOL) -> ::HRESULT,
    fn SetCheckButtonState(&mut self, dwIDCtl: ::DWORD, bChecked: ::BOOL) -> ::HRESULT,
    fn AddControlItem(
        &mut self, dwIDCtl: ::DWORD, dwIDItem: ::DWORD, pszLabel: ::LPCWSTR
    ) -> ::HRESULT,
    fn RemoveControlItem(&mut self, dwIDCtl: ::DWORD, dwIDItem: ::DWORD) -> ::HRESULT,
    fn RemoveAllControlItems(&mut self, dwIDCtl: ::DWORD) -> ::HRESULT,
    fn GetControlItemState(
        &mut self, dwIDCtl: ::DWORD, dwIDItem: ::DWORD, pdwState: *mut CDCONTROLSTATEF
    ) -> ::HRESULT,
    fn SetControlItemState(
        &mut self, dwIDCtl: ::DWORD, dwIDItem: ::DWORD, dwState: CDCONTROLSTATEF
    ) -> ::HRESULT,
    fn GetSelectedControlItem(&mut self, dwIDCtl: ::DWORD, pdwIDItem: *mut ::DWORD) -> ::HRESULT,
    fn SetSelectedControlItem(&mut self, dwIDCtl: ::DWORD, dwIDItem: ::DWORD) -> ::HRESULT,
    fn StartVisualGroup(&mut self, dwIDCtl: ::DWORD, pszLabel: ::LPCWSTR) -> ::HRESULT,
    fn EndVisualGroup(&mut self) -> ::HRESULT,
    fn MakeProminent(&mut self, dwIDCtl: ::DWORD) -> ::HRESULT,
    fn SetControlItemText(&mut self, dwIDCtl: ::DWORD, dwIDItem: ::DWORD, pszLabel: ::LPCWSTR) -> ::HRESULT
}
);
RIDL!(
interface IFileDialogControlEvents(IFileDialogControlEventsVtbl): IUnknown(IUnknownVtbl) {
    fn OnItemSelected(
        &mut self, pfdc: *mut IFileDialogCustomize, dwIDCtl: ::DWORD, dwIDItem: ::DWORD
    ) -> ::HRESULT,
    fn OnButtonClicked(&mut self, pfdc: *mut IFileDialogCustomize, dwIDCtl: ::DWORD) -> ::HRESULT,
    fn OnCheckButtonToggled(
        &mut self, pfdc: *mut IFileDialogCustomize, dwIDCtl: ::DWORD, bChecked: ::BOOL
    ) -> ::HRESULT,
    fn OnControlActivating(
        &mut self, pfdc: *mut IFileDialogCustomize, dwIDCtl: ::DWORD
    ) -> ::HRESULT
}
);
RIDL!(
interface IFileDialog2(IFileDialog2Vtbl): IFileDialog(IFileDialogVtbl) {
    fn SetCancelButtonLabel(&mut self, pszLabel: ::LPCWSTR) -> ::HRESULT,
    fn SetNavigationRoot(&mut self, psi: IShellItem) -> ::HRESULT
}
);
//27457
pub type IShellItemFilter = ::IUnknown; // TODO

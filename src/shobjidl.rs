// Copyright © 2015-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! this ALWAYS GENERATED file contains the definitions for the interfaces
//4498
pub type SFGAOF = ::ULONG;
//9466
ENUM!{enum SIGDN {
    SIGDN_NORMALDISPLAY = 0,
    SIGDN_PARENTRELATIVEPARSING = 0x80018001,
    SIGDN_DESKTOPABSOLUTEPARSING = 0x80028000,
    SIGDN_PARENTRELATIVEEDITING = 0x80031001,
    SIGDN_DESKTOPABSOLUTEEDITING = 0x8004c000,
    SIGDN_FILESYSPATH = 0x80058000,
    SIGDN_URL = 0x80068000,
    SIGDN_PARENTRELATIVEFORADDRESSBAR = 0x8007c001,
    SIGDN_PARENTRELATIVE = 0x80080001,
    SIGDN_PARENTRELATIVEFORUI = 0x80094001,
}}
ENUM!{enum SICHINTF {
    SICHINT_DISPLAY = 0,
    SICHINT_ALLFIELDS = 0x80000000,
    SICHINT_CANONICAL = 0x10000000,
    SICHINT_TEST_FILESYSPATH_IF_NOT_EQUAL = 0x20000000,
}}
//9498
RIDL!(
#[uuid(0x43826d1e, 0xe718, 0x42ee, 0xbc, 0x55, 0xa1, 0xe2, 0x61, 0xc3, 0x7b, 0xfe)]
interface IShellItem(IShellItemVtbl): IUnknown(IUnknownVtbl) {
    fn BindToHandler(
        pbc: *mut ::IBindCtx,
        bhid: ::REFGUID,
        riid: ::REFIID,
        ppv: *mut *mut ::c_void,
    ) -> ::HRESULT,
    fn GetParent(
        ppsi: *mut *mut IShellItem,
    ) -> ::HRESULT,
    fn GetDisplayName(
        sigdnName: SIGDN,
        ppszName: *mut ::LPWSTR,
    ) -> ::HRESULT,
    fn GetAttributes(
        sfgaoMask: SFGAOF,
        psfgaoAttribs: *mut SFGAOF,
    ) -> ::HRESULT,
    fn Compare(
        psi: *mut IShellItem,
        hint: SICHINTF,
        piOrder: *mut ::c_int,
    ) -> ::HRESULT,
}
);
//11963
pub type IFileOperationProgressSink = ::IUnknown; // TODO
ENUM!{enum GETPROPERTYSTOREFLAGS {
    GPS_DEFAULT = 0,
    GPS_HANDLERPROPERTIESONLY = 0x1,
    GPS_READWRITE = 0x2,
    GPS_TEMPORARY = 0x4,
    GPS_FASTPROPERTIESONLY = 0x8,
    GPS_OPENSLOWITEM = 0x10,
    GPS_DELAYCREATION = 0x20,
    GPS_BESTEFFORT = 0x40,
    GPS_NO_OPLOCK = 0x80,
    GPS_PREFERQUERYPROPERTIES = 0x100,
    GPS_EXTRINSICPROPERTIES = 0x200,
    GPS_EXTRINSICPROPERTIESONLY = 0x400,
    GPS_MASK_VALID = 0x7ff,
}}
ENUM!{enum SIATTRIBFLAGS {
    SIATTRIBFLAGS_AND = 0x1,
    SIATTRIBFLAGS_OR = 0x2,
    SIATTRIBFLAGS_APPCOMPAT = 0x3,
    SIATTRIBFLAGS_MASK = 0x3,
    SIATTRIBFLAGS_ALLITEMS = 0x4000,
}}
STRUCT!{struct PROPERTYKEY {
    fmtid: ::GUID,
    pid: ::DWORD,
}}
pub type REFPROPERTYKEY = *mut PROPERTYKEY;
RIDL!(
#[uuid(0xb63ea76d, 0x1f85, 0x456f, 0xa1, 0x9c, 0x48, 0x15, 0x9e, 0xfa, 0x85, 0x8b)]
interface IShellItemArray(IShellItemArrayVtbl): IUnknown(IUnknownVtbl) {
    fn BindToHandler(
        pbc: *mut ::IBindCtx,
        bhid: ::REFGUID,
        riid: ::REFIID,
        ppvOut: *mut *mut ::c_void,
    ) -> ::HRESULT,
    fn GetPropertyStore(
        flags: GETPROPERTYSTOREFLAGS,
        riid: ::REFIID,
        ppv: *mut *mut ::c_void,
    ) -> ::HRESULT,
    fn GetPropertyDescriptionList(
        keyType: REFPROPERTYKEY,
        riid: ::REFIID,
        ppv: *mut *mut ::c_void,
    ) -> ::HRESULT,
    fn GetAttributes(
        AttribFlags: SIATTRIBFLAGS,
        sfgaoMask: SFGAOF,
        psfgaoAttribs: *mut SFGAOF,
    ) -> ::HRESULT,
    fn GetCount(
        pdwNumItems: *mut ::DWORD,
    ) -> ::HRESULT,
    fn GetItemAt(
        dwIndex: ::DWORD,
        ppsi: *mut *mut IShellItem,
    ) -> ::HRESULT,
    fn EnumItems(
        ppenumShellItems: *mut *mut IEnumShellItems,
    ) -> ::HRESULT,
}
);
//20869
RIDL!(
#[uuid(0xb4db1657, 0x70d7, 0x485e, 0x8e, 0x3e, 0x6f, 0xcb, 0x5a, 0x5c, 0x18, 0x02)]
interface IModalWindow(IModalWindowVtbl): IUnknown(IUnknownVtbl) {
    fn Show(
        hwndOwner: ::HWND,
    ) -> ::HRESULT,
}
);
//22307
ENUM!{enum FDE_OVERWRITE_RESPONSE {
    FDEOR_DEFAULT = 0,
    FDEOR_ACCEPT = 1,
    FDEOR_REFUSE = 2,
}}
ENUM!{enum FDE_SHAREVIOLATION_RESPONSE {
    FDESVR_DEFAULT = 0,
    FDESVR_ACCEPT = 1,
    FDESVR_REFUSE = 2,
}}
ENUM!{enum FDAP {
    FDAP_BOTTOM = 0,
    FDAP_TOP = 1,
}}
RIDL!(
#[uuid(0x973510db, 0x7d7f, 0x452b, 0x89, 0x75, 0x74, 0xa8, 0x58, 0x28, 0xd3, 0x54)]
interface IFileDialogEvents(IFileDialogEventsVtbl): IUnknown(IUnknownVtbl) {
    fn OnFileOk(
        pfd: *mut IFileDialog,
    ) -> ::HRESULT,
    fn OnFolderChanging(
        pfd: *mut IFileDialog,
        psiFolder: *mut IShellItem,
    ) -> ::HRESULT,
    fn OnFolderChange(
        pfd: *mut IFileDialog,
    ) -> ::HRESULT,
    fn OnSelectionChange(
        pfd: *mut IFileDialog,
    ) -> ::HRESULT,
    fn OnShareViolation(
        pfd: *mut IFileDialog,
        psi: *mut IShellItem,
        pResponse: *mut FDE_SHAREVIOLATION_RESPONSE,
    ) -> ::HRESULT,
    fn OnTypeChange(
        pfd: *mut IFileDialog,
    ) -> ::HRESULT,
    fn OnOverwrite(
        pfd: *mut IFileDialog,
        psi: *mut IShellItem,
        pResponse: *mut FDE_OVERWRITE_RESPONSE,
    ) -> ::HRESULT,
}
);
ENUM!{enum FILEOPENDIALOGOPTIONS {
    FOS_OVERWRITEPROMPT = 0x2,
    FOS_STRICTFILETYPES = 0x4,
    FOS_NOCHANGEDIR = 0x8,
    FOS_PICKFOLDERS = 0x20,
    FOS_FORCEFILESYSTEM = 0x40,
    FOS_ALLNONSTORAGEITEMS = 0x80,
    FOS_NOVALIDATE = 0x100,
    FOS_ALLOWMULTISELECT = 0x200,
    FOS_PATHMUSTEXIST = 0x800,
    FOS_FILEMUSTEXIST = 0x1000,
    FOS_CREATEPROMPT = 0x2000,
    FOS_SHAREAWARE = 0x4000,
    FOS_NOREADONLYRETURN = 0x8000,
    FOS_NOTESTFILECREATE = 0x10000,
    FOS_HIDEMRUPLACES = 0x20000,
    FOS_HIDEPINNEDPLACES = 0x40000,
    FOS_NODEREFERENCELINKS = 0x100000,
    FOS_DONTADDTORECENT = 0x2000000,
    FOS_FORCESHOWHIDDEN = 0x10000000,
    FOS_DEFAULTNOMINIMODE = 0x20000000,
    FOS_FORCEPREVIEWPANEON = 0x40000000,
    FOS_SUPPORTSTREAMABLEITEMS = 0x80000000,
}}
RIDL!(
#[uuid(0x42f85136, 0xdb7e, 0x439c, 0x85, 0xf1, 0xe4, 0x07, 0x5d, 0x13, 0x5f, 0xc8)]
interface IFileDialog(IFileDialogVtbl): IModalWindow(IModalWindowVtbl) {
    fn SetFileTypes(
        cFileTypes: ::UINT,
        rgFilterSpec: *const ::COMDLG_FILTERSPEC,
    ) -> ::HRESULT,
    fn SetFileTypeIndex(
        iFileType: ::UINT,
    ) -> ::HRESULT,
    fn GetFileTypeIndex(
        piFileType: *mut ::UINT,
    ) -> ::HRESULT,
    fn Advise(
        pfde: *mut IFileDialogEvents,
        pdwCookie: *mut ::DWORD,
    ) -> ::HRESULT,
    fn Unadvise(
        dwCookie: ::DWORD,
    ) -> ::HRESULT,
    fn SetOptions(
        fos: FILEOPENDIALOGOPTIONS,
    ) -> ::HRESULT,
    fn GetOptions(
        pfos: *mut FILEOPENDIALOGOPTIONS,
    ) -> ::HRESULT,
    fn SetDefaultFolder(
        psi: *mut IShellItem,
    ) -> ::HRESULT,
    fn SetFolder(
        psi: *mut IShellItem,
    ) -> ::HRESULT,
    fn GetFolder(
        ppsi: *mut *mut IShellItem,
    ) -> ::HRESULT,
    fn GetCurrentSelection(
        ppsi: *mut *mut IShellItem,
    ) -> ::HRESULT,
    fn SetFileName(
        pszName: ::LPCWSTR,
    ) -> ::HRESULT,
    fn GetFileName(
        pszName: *mut ::LPWSTR,
    ) -> ::HRESULT,
    fn SetTitle(
        pszTitle: ::LPCWSTR,
    ) -> ::HRESULT,
    fn SetOkButtonLabel(
        pszText: ::LPCWSTR,
    ) -> ::HRESULT,
    fn SetFileNameLabel(
        pszLabel: ::LPCWSTR,
    ) -> ::HRESULT,
    fn GetResult(
        ppsi: *mut *mut IShellItem,
    ) -> ::HRESULT,
    fn AddPlace(
        psi: *mut IShellItem,
        fdap: FDAP,
    ) -> ::HRESULT,
    fn SetDefaultExtension(
        pszDefaultExtension: ::LPCWSTR,
    ) -> ::HRESULT,
    fn Close(
        hr: ::HRESULT,
    ) -> ::HRESULT,
    fn SetClientGuid(
        guid: ::REFGUID,
    ) -> ::HRESULT,
    fn ClearClientData() -> ::HRESULT,
    fn SetFilter(
        pFilter: *mut IShellItemFilter,
    ) -> ::HRESULT,
}
);
RIDL!(
#[uuid(0x84bccd23, 0x5fde, 0x4cdb, 0xae, 0xa4, 0xaf, 0x64, 0xb8, 0x3d, 0x78, 0xab)]
interface IFileSaveDialog(IFileSaveDialogVtbl): IFileDialog(IFileDialogVtbl) {
    fn SetSaveAsItem(
        psi: *mut IShellItem,
    ) -> ::HRESULT,
    fn SetProperties(
        pStore: *mut ::IPropertyStore,
    ) -> ::HRESULT,
    fn SetCollectedProperties(
        pList: *mut ::IPropertyDescriptionList,
        fAppendDefault: ::BOOL,
    ) -> ::HRESULT,
    fn GetProperties(
        ppStore: *mut *mut ::IPropertyStore,
    ) -> ::HRESULT,
    fn ApplyProperties(
        psi: *mut IShellItem,
        pStore: *mut ::IPropertyStore,
        hwnd: ::HWND,
        pSink: *mut IFileOperationProgressSink,
    ) -> ::HRESULT,
}
);
RIDL!(
#[uuid(0xd57c7288, 0xd4ad, 0x4768, 0xbe, 0x02, 0x9d, 0x96, 0x95, 0x32, 0xd9, 0x60)]
interface IFileOpenDialog(IFileOpenDialogVtbl): IFileDialog(IFileDialogVtbl) {
    fn GetResults(
        ppenum: *mut *mut IShellItemArray,
    ) -> ::HRESULT,
    fn GetSelectedItems(
        ppsai: *mut *mut IShellItemArray,
    ) -> ::HRESULT,
}
);
ENUM!{enum CDCONTROLSTATE {
    CDCS_INACTIVE = 0x00000000,
    CDCS_ENABLED = 0x00000001,
    CDCS_VISIBLE = 0x00000002,
    CDCS_ENABLEDVISIBLE = 0x00000003,
}}
RIDL!(
#[uuid(0xe6fdd21a, 0x163f, 0x4975, 0x9c, 0x8c, 0xa6, 0x9f, 0x1b, 0xa3, 0x70, 0x34)]
interface IFileDialogCustomize(IFileDialogCustomizeVtbl): IUnknown(IUnknownVtbl) {
    fn EnableOpenDropDown(
        dwIDCtl: ::DWORD,
    ) -> ::HRESULT,
    fn AddMenu(
        dwIDCtl: ::DWORD,
        pszLabel: ::LPCWSTR,
    ) -> ::HRESULT,
    fn AddPushButton(
        dwIDCtl: ::DWORD,
        pszLabel: ::LPCWSTR,
    ) -> ::HRESULT,
    fn AddComboBox(
        dwIDCtl: ::DWORD,
    ) -> ::HRESULT,
    fn AddRadioButtonList(
        dwIDCtl: ::DWORD,
    ) -> ::HRESULT,
    fn AddCheckButton(
        dwIDCtl: ::DWORD,
        pszLabel: ::LPCWSTR,
        bChecked: ::BOOL,
    ) -> ::HRESULT,
    fn AddEditBox(
        dwIDCtl: ::DWORD,
        pszText: ::LPCWSTR,
    ) -> ::HRESULT,
    fn AddSeparator(
        dwIDCtl: ::DWORD,
    ) -> ::HRESULT,
    fn AddText(
        dwIDCtl: ::DWORD,
        pszText: ::LPCWSTR,
    ) -> ::HRESULT,
    fn SetControlLabel(
        dwIDCtl: ::DWORD,
        pszLabel: ::LPCWSTR,
    ) -> ::HRESULT,
    fn GetControlState(
        dwIDCtl: ::DWORD,
        pdwState: *mut CDCONTROLSTATE,
    ) -> ::HRESULT,
    fn SetControlState(
        dwIDCtl: ::DWORD,
        dwState: CDCONTROLSTATE,
    ) -> ::HRESULT,
    fn GetEditBoxText(
        dwIDCtl: ::DWORD,
        ppszText: *mut *mut ::WCHAR,
    ) -> ::HRESULT,
    fn SetEditBoxText(
        dwIDCtl: ::DWORD,
        pszText: ::LPCWSTR,
    ) -> ::HRESULT,
    fn GetCheckButtonState(
        dwIDCtl: ::DWORD,
        pbChecked: *mut ::BOOL,
    ) -> ::HRESULT,
    fn SetCheckButtonState(
        dwIDCtl: ::DWORD,
        bChecked: ::BOOL,
    ) -> ::HRESULT,
    fn AddControlItem(
        dwIDCtl: ::DWORD,
        dwIDItem: ::DWORD,
        pszLabel: ::LPCWSTR,
    ) -> ::HRESULT,
    fn RemoveControlItem(
        dwIDCtl: ::DWORD,
        dwIDItem: ::DWORD,
    ) -> ::HRESULT,
    fn RemoveAllControlItems(
        dwIDCtl: ::DWORD,
    ) -> ::HRESULT,
    fn GetControlItemState(
        dwIDCtl: ::DWORD,
        dwIDItem: ::DWORD,
        pdwState: *mut CDCONTROLSTATE,
    ) -> ::HRESULT,
    fn SetControlItemState(
        dwIDCtl: ::DWORD,
        dwIDItem: ::DWORD,
        dwState: CDCONTROLSTATE,
    ) -> ::HRESULT,
    fn GetSelectedControlItem(
        dwIDCtl: ::DWORD,
        pdwIDItem: *mut ::DWORD,
    ) -> ::HRESULT,
    fn SetSelectedControlItem(
        dwIDCtl: ::DWORD,
        dwIDItem: ::DWORD,
    ) -> ::HRESULT,
    fn StartVisualGroup(
        dwIDCtl: ::DWORD,
        pszLabel: ::LPCWSTR,
    ) -> ::HRESULT,
    fn EndVisualGroup() -> ::HRESULT,
    fn MakeProminent(
        dwIDCtl: ::DWORD,
    ) -> ::HRESULT,
    fn SetControlItemText(
        dwIDCtl: ::DWORD,
        dwIDItem: ::DWORD,
        pszLabel: ::LPCWSTR,
    ) -> ::HRESULT,
}
);
RIDL!(
#[uuid(0x36116642, 0xd713, 0x4b97, 0x9b, 0x83, 0x74, 0x84, 0xa9, 0xd0, 0x04, 0x33)]
interface IFileDialogControlEvents(IFileDialogControlEventsVtbl): IUnknown(IUnknownVtbl) {
    fn OnItemSelected(
        pfdc: *mut IFileDialogCustomize,
        dwIDCtl: ::DWORD,
        dwIDItem: ::DWORD,
    ) -> ::HRESULT,
    fn OnButtonClicked(
        pfdc: *mut IFileDialogCustomize,
        dwIDCtl: ::DWORD,
    ) -> ::HRESULT,
    fn OnCheckButtonToggled(
        pfdc: *mut IFileDialogCustomize,
        dwIDCtl: ::DWORD,
        bChecked: ::BOOL,
    ) -> ::HRESULT,
    fn OnControlActivating(
        pfdc: *mut IFileDialogCustomize,
        dwIDCtl: ::DWORD,
    ) -> ::HRESULT,
}
);
RIDL!(
#[uuid(0x61744fc7, 0x85b5, 0x4791, 0xa9, 0xb0, 0x27, 0x22, 0x76, 0x30, 0x9b, 0x13)]
interface IFileDialog2(IFileDialog2Vtbl): IFileDialog(IFileDialogVtbl) {
    fn SetCancelButtonLabel(
        pszLabel: ::LPCWSTR,
    ) -> ::HRESULT,
    fn SetNavigationRoot(
        psi: IShellItem,
    ) -> ::HRESULT,
}
);
//27457
pub type IShellItemFilter = ::IUnknown; // TODO

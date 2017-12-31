// Copyright © 2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use ctypes::{c_int, c_long};
use shared::minwindef::{BOOL, DWORD, UINT, ULONG};
use shared::windef::HWND;
use shared::wtypes::{BSTR, DATE, VARIANT_BOOL};
use um::oaidl::{IDispatch, IDispatchVtbl, VARIANT};
use um::objidl::IBindCtx;
use um::objidlbase::{IEnumString, IEnumStringVtbl};
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::{HRESULT, LONG, LPCWSTR, LPWSTR};
// extern RPC_IF_HANDLE __MIDL_itf_shldisp_0000_0000_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shldisp_0000_0000_v0_0_s_ifspec;
ENUM!{enum OfflineFolderStatus {
    OFS_INACTIVE = -1i32 as u32,
    OFS_ONLINE = 0,
    OFS_OFFLINE = OFS_ONLINE + 1,
    OFS_SERVERBACK = OFS_OFFLINE + 1,
    OFS_DIRTYCACHE = OFS_SERVERBACK + 1,
}}
ENUM!{enum ShellFolderViewOptions {
    SFVVO_SHOWALLOBJECTS = 0x1,
    SFVVO_SHOWEXTENSIONS = 0x2,
    SFVVO_SHOWCOMPCOLOR = 0x8,
    SFVVO_SHOWSYSFILES = 0x20,
    SFVVO_WIN95CLASSIC = 0x40,
    SFVVO_DOUBLECLICKINWEBVIEW = 0x80,
    SFVVO_DESKTOPHTML = 0x200,
}}
ENUM!{enum ShellSpecialFolderConstants {
    ssfDESKTOP = 0,
    ssfPROGRAMS = 0x2,
    ssfCONTROLS = 0x3,
    ssfPRINTERS = 0x4,
    ssfPERSONAL = 0x5,
    ssfFAVORITES = 0x6,
    ssfSTARTUP = 0x7,
    ssfRECENT = 0x8,
    ssfSENDTO = 0x9,
    ssfBITBUCKET = 0xa,
    ssfSTARTMENU = 0xb,
    ssfDESKTOPDIRECTORY = 0x10,
    ssfDRIVES = 0x11,
    ssfNETWORK = 0x12,
    ssfNETHOOD = 0x13,
    ssfFONTS = 0x14,
    ssfTEMPLATES = 0x15,
    ssfCOMMONSTARTMENU = 0x16,
    ssfCOMMONPROGRAMS = 0x17,
    ssfCOMMONSTARTUP = 0x18,
    ssfCOMMONDESKTOPDIR = 0x19,
    ssfAPPDATA = 0x1a,
    ssfPRINTHOOD = 0x1b,
    ssfLOCALAPPDATA = 0x1c,
    ssfALTSTARTUP = 0x1d,
    ssfCOMMONALTSTARTUP = 0x1e,
    ssfCOMMONFAVORITES = 0x1f,
    ssfINTERNETCACHE = 0x20,
    ssfCOOKIES = 0x21,
    ssfHISTORY = 0x22,
    ssfCOMMONAPPDATA = 0x23,
    ssfWINDOWS = 0x24,
    ssfSYSTEM = 0x25,
    ssfPROGRAMFILES = 0x26,
    ssfMYPICTURES = 0x27,
    ssfPROFILE = 0x28,
    ssfSYSTEMx86 = 0x29,
    ssfPROGRAMFILESx86 = 0x30,
}}
// EXTERN_C const IID LIBID_Shell32;
DEFINE_GUID!{IID_IFolderViewOC,
    0x9ba05970, 0xf6a8, 0x11cf, 0xa4, 0x42, 0x00, 0xa0, 0xc9, 0x0a, 0x8f, 0x39}
RIDL!{#[uuid(0x9ba05970, 0xf6a8, 0x11cf, 0xa4, 0x42, 0x00, 0xa0, 0xc9, 0x0a, 0x8f, 0x39)]
interface IFolderViewOC(IFolderViewOCVtbl): IDispatch(IDispatchVtbl) {
    fn SetFolderView(
        pdisp: *mut IDispatch,
    ) -> HRESULT,
}}
DEFINE_GUID!{DIID_DShellFolderViewEvents,
    0x62112aa2, 0xebe4, 0x11cf, 0xa5, 0xfb, 0x00, 0x20, 0xaf, 0xe7, 0x29, 0x2d}
RIDL!{#[uuid(0x62112aa2, 0xebe4, 0x11cf, 0xa5, 0xfb, 0x00, 0x20, 0xaf, 0xe7, 0x29, 0x2d)]
interface DShellFolderViewEvents(DShellFolderViewEventsVtbl): IDispatch(IDispatchVtbl) {
}}
DEFINE_GUID!{CLSID_ShellFolderViewOC,
    0x9BA05971, 0xF6A8, 0x11CF, 0xA4, 0x42, 0x00, 0xA0, 0xC9, 0x0A, 0x8F, 0x39}
// class DECLSPEC_UUID("9BA05971-F6A8-11CF-A442-00A0C90A8F39")
// ShellFolderViewOC;
DEFINE_GUID!{IID_DFConstraint,
    0x4a3df050, 0x23bd, 0x11d2, 0x93, 0x9f, 0x00, 0xa0, 0xc9, 0x1e, 0xed, 0xba}
RIDL!{#[uuid(0x4a3df050, 0x23bd, 0x11d2, 0x93, 0x9f, 0x00, 0xa0, 0xc9, 0x1e, 0xed, 0xba)]
interface DFConstraint(DFConstraintVtbl): IDispatch(IDispatchVtbl) {
    fn get_Name(
        pbs: *mut BSTR,
    ) -> HRESULT,
    fn get_Value(
        pv: *mut VARIANT,
    ) -> HRESULT,
}}
pub type LPFOLDERITEM = *mut FolderItem;
DEFINE_GUID!{IID_FolderItem,
    0xfac32c80, 0xcbe4, 0x11ce, 0x83, 0x50, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00}
RIDL!{#[uuid(0xfac32c80, 0xcbe4, 0x11ce, 0x83, 0x50, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00)]
interface FolderItem(FolderItemVtbl): IDispatch(IDispatchVtbl) {
    fn get_Application(
        ppid: *mut *mut IDispatch,
    ) -> HRESULT,
    fn get_Parent(
        ppid: *mut *mut IDispatch,
    ) -> HRESULT,
    fn get_Name(
        pbs: *mut BSTR,
    ) -> HRESULT,
    fn put_Name(
        bs: BSTR,
    ) -> HRESULT,
    fn get_Path(
        pbs: *mut BSTR,
    ) -> HRESULT,
    fn get_GetLink(
        ppid: *mut *mut IDispatch,
    ) -> HRESULT,
    fn get_GetFolder(
        ppid: *mut *mut IDispatch,
    ) -> HRESULT,
    fn get_IsLink(
        pb: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsFolder(
        pb: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsFileSystem(
        pb: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsBrowsable(
        pb: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_ModifyDate(
        pdt: *mut DATE,
    ) -> HRESULT,
    fn put_ModifyDate(
        dt: DATE,
    ) -> HRESULT,
    fn get_Size(
        pul: *mut LONG,
    ) -> HRESULT,
    fn get_Type(
        pbs: *mut BSTR,
    ) -> HRESULT,
    fn Verbs(
        ppfic: *mut *mut FolderItemVerbs,
    ) -> HRESULT,
    fn InvokeVerb(
        vVerb: VARIANT,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_FolderItems,
    0x744129e0, 0xcbe5, 0x11ce, 0x83, 0x50, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00}
RIDL!{#[uuid(0x744129e0, 0xcbe5, 0x11ce, 0x83, 0x50, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00)]
interface FolderItems(FolderItemsVtbl): IDispatch(IDispatchVtbl) {
    fn get_Count(
        plCount: *mut c_long,
    ) -> HRESULT,
    fn get_Application(
        ppid: *mut *mut IDispatch,
    ) -> HRESULT,
    fn get_Parent(
        ppid: *mut *mut IDispatch,
    ) -> HRESULT,
    fn Item(
        index: VARIANT,
        ppid: *mut *mut FolderItem,
    ) -> HRESULT,
    fn _NewEnum(
        ppunk: *mut *mut IUnknown,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_FolderItemVerb,
    0x08ec3e00, 0x50b0, 0x11cf, 0x96, 0x0c, 0x00, 0x80, 0xc7, 0xf4, 0xee, 0x85}
RIDL!{#[uuid(0x08ec3e00, 0x50b0, 0x11cf, 0x96, 0x0c, 0x00, 0x80, 0xc7, 0xf4, 0xee, 0x85)]
interface FolderItemVerb(FolderItemVerbVtbl): IDispatch(IDispatchVtbl) {
    fn get_Application(
        ppid: *mut *mut IDispatch,
    ) -> HRESULT,
    fn get_Parent(
        ppid: *mut *mut IDispatch,
    ) -> HRESULT,
    fn get_Name(
        pbs: *mut BSTR,
    ) -> HRESULT,
    fn DoIt() -> HRESULT,
}}
DEFINE_GUID!{IID_FolderItemVerbs,
    0x1f8352c0, 0x50b0, 0x11cf, 0x96, 0x0c, 0x00, 0x80, 0xc7, 0xf4, 0xee, 0x85}
RIDL!{#[uuid(0x1f8352c0, 0x50b0, 0x11cf, 0x96, 0x0c, 0x00, 0x80, 0xc7, 0xf4, 0xee, 0x85)]
interface FolderItemVerbs(FolderItemVerbsVtbl): IDispatch(IDispatchVtbl) {
    fn get_Count(
        plCount: *mut c_long,
    ) -> HRESULT,
    fn get_Application(
        ppid: *mut *mut IDispatch,
    ) -> HRESULT,
    fn get_Parent(
        ppid: *mut *mut IDispatch,
    ) -> HRESULT,
    fn Item(
        index: VARIANT,
        ppid: *mut *mut FolderItemVerb,
    ) -> HRESULT,
    fn _NewEnum(
        ppunk: *mut *mut IUnknown,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_Folder,
    0xbbcbde60, 0xc3ff, 0x11ce, 0x83, 0x50, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00}
RIDL!{#[uuid(0xbbcbde60, 0xc3ff, 0x11ce, 0x83, 0x50, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00)]
interface Folder(FolderVtbl): IDispatch(IDispatchVtbl) {
    fn get_Title(
        pbs: *mut BSTR,
    ) -> HRESULT,
    fn get_Application(
        ppid: *mut *mut IDispatch,
    ) -> HRESULT,
    fn get_Parent(
        ppid: *mut *mut IDispatch,
    ) -> HRESULT,
    fn get_ParentFolder(
        ppsf: *mut *mut Folder,
    ) -> HRESULT,
    fn Items(
        ppid: *mut *mut FolderItems,
    ) -> HRESULT,
    fn ParseName(
        bName: BSTR,
        ppid: *mut *mut FolderItem,
    ) -> HRESULT,
    fn NewFolder(
        bName: BSTR,
        vOptions: VARIANT,
    ) -> HRESULT,
    fn MoveHere(
        vItem: VARIANT,
        vOptions: VARIANT,
    ) -> HRESULT,
    fn CopyHere(
        vItem: VARIANT,
        vOptions: VARIANT,
    ) -> HRESULT,
    fn GetDetailsOf(
        vItem: VARIANT,
        iColumn: c_int,
        pbs: *mut BSTR,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_Folder2,
    0xf0d2d8ef, 0x3890, 0x11d2, 0xbf, 0x8b, 0x00, 0xc0, 0x4f, 0xb9, 0x36, 0x61}
RIDL!{#[uuid(0xf0d2d8ef, 0x3890, 0x11d2, 0xbf, 0x8b, 0x00, 0xc0, 0x4f, 0xb9, 0x36, 0x61)]
interface Folder2(Folder2Vtbl): Folder(FolderVtbl) {
    fn get_Self(
        ppfi: *mut *mut FolderItem,
    ) -> HRESULT,
    fn get_OfflineStatus(
        pul: *mut LONG,
    ) -> HRESULT,
    fn Synchronize() -> HRESULT,
    fn get_HaveToShowWebViewBarricade(
        pbHaveToShowWebViewBarricade: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn DismissedWebViewBarricade() -> HRESULT,
}}
DEFINE_GUID!{IID_Folder3,
    0xa7ae5f64, 0xc4d7, 0x4d7f, 0x93, 0x07, 0x4d, 0x24, 0xee, 0x54, 0xb8, 0x41}
RIDL!{#[uuid(0xa7ae5f64, 0xc4d7, 0x4d7f, 0x93, 0x07, 0x4d, 0x24, 0xee, 0x54, 0xb8, 0x41)]
interface Folder3(Folder3Vtbl): Folder2(Folder2Vtbl) {
    fn get_ShowWebViewBarricade(
        pbShowWebViewBarricade: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_ShowWebViewBarricade(
        bShowWebViewBarricade: VARIANT_BOOL,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_FolderItem2,
    0xedc817aa, 0x92b8, 0x11d1, 0xb0, 0x75, 0x00, 0xc0, 0x4f, 0xc3, 0x3a, 0xa5}
RIDL!{#[uuid(0xedc817aa, 0x92b8, 0x11d1, 0xb0, 0x75, 0x00, 0xc0, 0x4f, 0xc3, 0x3a, 0xa5)]
interface FolderItem2(FolderItem2Vtbl): FolderItem(FolderItemVtbl) {
    fn InvokeVerbEx(
        vVerb: VARIANT,
        vArgs: VARIANT,
    ) -> HRESULT,
    fn ExtendedProperty(
        bstrPropName: BSTR,
        pvRet: *mut VARIANT,
    ) -> HRESULT,
}}
DEFINE_GUID!{CLSID_ShellFolderItem,
    0x2fe352ea, 0xfd1f, 0x11d2, 0xb1, 0xf4, 0x00, 0xc0, 0x4f, 0x8e, 0xeb, 0x3e}
// class DECLSPEC_UUID("2fe352ea-fd1f-11d2-b1f4-00c04f8eeb3e")
// ShellFolderItem;
DEFINE_GUID!{IID_FolderItems2,
    0xc94f0ad0, 0xf363, 0x11d2, 0xa3, 0x27, 0x00, 0xc0, 0x4f, 0x8e, 0xec, 0x7f}
RIDL!{#[uuid(0xc94f0ad0, 0xf363, 0x11d2, 0xa3, 0x27, 0x00, 0xc0, 0x4f, 0x8e, 0xec, 0x7f)]
interface FolderItems2(FolderItems2Vtbl): FolderItems(FolderItemsVtbl) {
    fn InvokeVerbEx(
        vVerb: VARIANT,
        vArgs: VARIANT,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_FolderItems3,
    0xeaa7c309, 0xbbec, 0x49d5, 0x82, 0x1d, 0x64, 0xd9, 0x66, 0xcb, 0x66, 0x7f}
RIDL!{#[uuid(0xeaa7c309, 0xbbec, 0x49d5, 0x82, 0x1d, 0x64, 0xd9, 0x66, 0xcb, 0x66, 0x7f)]
interface FolderItems3(FolderItems3Vtbl): FolderItems2(FolderItems2Vtbl) {
    fn Filter(
        grfFlags: c_long,
        bstrFileSpec: BSTR,
    ) -> HRESULT,
    fn get_Verbs(
        ppfic: *mut *mut FolderItemVerbs,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IShellLinkDual,
    0x88a05c00, 0xf000, 0x11ce, 0x83, 0x50, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00}
RIDL!{#[uuid(0x88a05c00, 0xf000, 0x11ce, 0x83, 0x50, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00)]
interface IShellLinkDual(IShellLinkDualVtbl): IDispatch(IDispatchVtbl) {
    fn get_Path(
        pbs: *mut BSTR,
    ) -> HRESULT,
    fn put_Path(
        bs: BSTR,
    ) -> HRESULT,
    fn get_Description(
        pbs: *mut BSTR,
    ) -> HRESULT,
    fn put_Description(
        bs: BSTR,
    ) -> HRESULT,
    fn get_WorkingDirectory(
        pbs: *mut BSTR,
    ) -> HRESULT,
    fn put_WorkingDirectory(
        bs: BSTR,
    ) -> HRESULT,
    fn get_Arguments(
        pbs: *mut BSTR,
    ) -> HRESULT,
    fn put_Arguments(
        bs: BSTR,
    ) -> HRESULT,
    fn get_Hotkey(
        piHK: *mut c_int,
    ) -> HRESULT,
    fn put_Hotkey(
        iHK: c_int,
    ) -> HRESULT,
    fn get_ShowCommand(
        piShowCommand: *mut c_int,
    ) -> HRESULT,
    fn put_ShowCommand(
        iShowCommand: c_int,
    ) -> HRESULT,
    fn Resolve(
        fFlags: c_int,
    ) -> HRESULT,
    fn GetIconLocation(
        pbs: *mut BSTR,
        piIcon: *mut c_int,
    ) -> HRESULT,
    fn SetIconLocation(
        bs: BSTR,
        iIcon: c_int,
    ) -> HRESULT,
    fn Save(
        vWhere: VARIANT,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IShellLinkDual2,
    0x317ee249, 0xf12e, 0x11d2, 0xb1, 0xe4, 0x00, 0xc0, 0x4f, 0x8e, 0xeb, 0x3e}
RIDL!{#[uuid(0x317ee249, 0xf12e, 0x11d2, 0xb1, 0xe4, 0x00, 0xc0, 0x4f, 0x8e, 0xeb, 0x3e)]
interface IShellLinkDual2(IShellLinkDual2Vtbl): IShellLinkDual(IShellLinkDualVtbl) {
    fn get_Target(
        ppfi: *mut *mut FolderItem,
    ) -> HRESULT,
}}
DEFINE_GUID!{CLSID_ShellLinkObject,
    0x11219420, 0x1768, 0x11d1, 0x95, 0xBE, 0x00, 0x60, 0x97, 0x97, 0xEA, 0x4F}
// class DECLSPEC_UUID("11219420-1768-11d1-95BE-00609797EA4F")
// ShellLinkObject;
DEFINE_GUID!{IID_IShellFolderViewDual,
    0xe7a1af80, 0x4d96, 0x11cf, 0x96, 0x0c, 0x00, 0x80, 0xc7, 0xf4, 0xee, 0x85}
RIDL!{#[uuid(0xe7a1af80, 0x4d96, 0x11cf, 0x96, 0x0c, 0x00, 0x80, 0xc7, 0xf4, 0xee, 0x85)]
interface IShellFolderViewDual(IShellFolderViewDualVtbl): IDispatch(IDispatchVtbl) {
    fn get_Application(
        ppid: *mut *mut IDispatch,
    ) -> HRESULT,
    fn get_Parent(
        ppid: *mut *mut IDispatch,
    ) -> HRESULT,
    fn get_Folder(
        ppid: *mut *mut Folder,
    ) -> HRESULT,
    fn SelectedItems(
        ppid: *mut *mut FolderItems,
    ) -> HRESULT,
    fn get_FocusedItem(
        ppid: *mut *mut FolderItem,
    ) -> HRESULT,
    fn SelectItem(
        pvfi: *mut VARIANT,
        dwFlags: c_int,
    ) -> HRESULT,
    fn PopupItemMenu(
        pfi: *mut FolderItem,
        vx: VARIANT,
        vy: VARIANT,
        pbs: *mut BSTR,
    ) -> HRESULT,
    fn get_Script(
        ppDisp: *mut *mut IDispatch,
    ) -> HRESULT,
    fn get_ViewOptions(
        plViewOptions: *mut c_long,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IShellFolderViewDual2,
    0x31c147b6, 0x0ade, 0x4a3c, 0xb5, 0x14, 0xdd, 0xf9, 0x32, 0xef, 0x6d, 0x17}
RIDL!{#[uuid(0x31c147b6, 0x0ade, 0x4a3c, 0xb5, 0x14, 0xdd, 0xf9, 0x32, 0xef, 0x6d, 0x17)]
interface IShellFolderViewDual2(IShellFolderViewDual2Vtbl): IShellFolderViewDual(IShellFolderViewDualVtbl) {
    fn get_CurrentViewMode(
        pViewMode: *mut UINT,
    ) -> HRESULT,
    fn put_CurrentViewMode(
        ViewMode: UINT,
    ) -> HRESULT,
    fn SelectItemRelative(
        iRelative: c_int,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IShellFolderViewDual3,
    0x29ec8e6c, 0x46d3, 0x411f, 0xba, 0xaa, 0x61, 0x1a, 0x6c, 0x9c, 0xac, 0x66}
RIDL!{#[uuid(0x29ec8e6c, 0x46d3, 0x411f, 0xba, 0xaa, 0x61, 0x1a, 0x6c, 0x9c, 0xac, 0x66)]
interface IShellFolderViewDual3(IShellFolderViewDual3Vtbl): IShellFolderViewDual2(IShellFolderViewDual2Vtbl) {
    fn get_GroupBy(
        pbstrGroupBy: *mut BSTR,
    ) -> HRESULT,
    fn put_GroupBy(
        bstrGroupBy: BSTR,
    ) -> HRESULT,
    fn get_FolderFlags(
        pdwFlags: *mut DWORD,
    ) -> HRESULT,
    fn put_FolderFlags(
        dwFlags: DWORD,
    ) -> HRESULT,
    fn get_SortColumns(
        pbstrSortColumns: *mut BSTR,
    ) -> HRESULT,
    fn put_SortColumns(
        bstrSortColumns: BSTR,
    ) -> HRESULT,
    fn put_IconSize(
        iIconSize: c_int,
    ) -> HRESULT,
    fn get_IconSize(
        piIconSize: *mut c_int,
    ) -> HRESULT,
    fn FilterView(
        bstrFilterText: BSTR,
    ) -> HRESULT,
}}
DEFINE_GUID!{CLSID_ShellFolderView,
    0x62112AA1, 0xEBE4, 0x11cf, 0xA5, 0xFB, 0x00, 0x20, 0xAF, 0xE7, 0x29, 0x2D}
// class DECLSPEC_UUID("62112AA1-EBE4-11cf-A5FB-0020AFE7292D")
// ShellFolderView;
DEFINE_GUID!{IID_IShellDispatch,
    0xd8f015c0, 0xc278, 0x11ce, 0xa4, 0x9e, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00}
RIDL!{#[uuid(0xd8f015c0, 0xc278, 0x11ce, 0xa4, 0x9e, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00)]
interface IShellDispatch(IShellDispatchVtbl): IDispatch(IDispatchVtbl) {
    fn get_Application(
        ppid: *mut *mut IDispatch,
    ) -> HRESULT,
    fn get_Parent(
        ppid: *mut *mut IDispatch,
    ) -> HRESULT,
    fn NameSpace(
        vDir: VARIANT,
        ppsdf: *mut *mut Folder,
    ) -> HRESULT,
    fn BrowseForFolder(
        Hwnd: c_long,
        Title: BSTR,
        Options: c_long,
        RootFolder: VARIANT,
        ppsdf: *mut *mut Folder,
    ) -> HRESULT,
    fn Windows(
        ppid: *mut *mut IDispatch,
    ) -> HRESULT,
    fn Open(
        vDir: VARIANT,
    ) -> HRESULT,
    fn Explore(
        vDir: VARIANT,
    ) -> HRESULT,
    fn MinimizeAll() -> HRESULT,
    fn UndoMinimizeALL() -> HRESULT,
    fn FileRun() -> HRESULT,
    fn CascadeWindows() -> HRESULT,
    fn TileVertically() -> HRESULT,
    fn TileHorizontally() -> HRESULT,
    fn ShutdownWindows() -> HRESULT,
    fn Suspend() -> HRESULT,
    fn EjectPC() -> HRESULT,
    fn SetTime() -> HRESULT,
    fn TrayProperties() -> HRESULT,
    fn Help() -> HRESULT,
    fn FindFiles() -> HRESULT,
    fn FindComputer() -> HRESULT,
    fn RefreshMenu() -> HRESULT,
    fn ControlPanelItem(
        bstrDir: BSTR,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IShellDispatch2,
    0xa4c6892c, 0x3ba9, 0x11d2, 0x9d, 0xea, 0x00, 0xc0, 0x4f, 0xb1, 0x61, 0x62}
RIDL!{#[uuid(0xa4c6892c, 0x3ba9, 0x11d2, 0x9d, 0xea, 0x00, 0xc0, 0x4f, 0xb1, 0x61, 0x62)]
interface IShellDispatch2(IShellDispatch2Vtbl): IShellDispatch(IShellDispatchVtbl) {
    fn IsRestricted(
        Group: BSTR,
        Restriction: BSTR,
        plRestrictValue: *mut c_long,
    ) -> HRESULT,
    fn ShellExecute(
        File: BSTR,
        vArgs: VARIANT,
        vDir: VARIANT,
        vOperation: VARIANT,
        vShow: VARIANT,
    ) -> HRESULT,
    fn FindPrinter(
        name: BSTR,
        location: BSTR,
        model: BSTR,
    ) -> HRESULT,
    fn GetSystemInformation(
        name: BSTR,
        pv: *mut VARIANT,
    ) -> HRESULT,
    fn ServiceStart(
        ServiceName: BSTR,
        Persistent: VARIANT,
        pSuccess: *mut VARIANT,
    ) -> HRESULT,
    fn ServiceStop(
        ServiceName: BSTR,
        Persistent: VARIANT,
        pSuccess: *mut VARIANT,
    ) -> HRESULT,
    fn IsServiceRunning(
        ServiceName: BSTR,
        pRunning: *mut VARIANT,
    ) -> HRESULT,
    fn CanStartStopService(
        ServiceName: BSTR,
        pCanStartStop: *mut VARIANT,
    ) -> HRESULT,
    fn ShowBrowserBar(
        bstrClsid: BSTR,
        bShow: VARIANT,
        pSuccess: *mut VARIANT,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IShellDispatch3,
    0x177160ca, 0xbb5a, 0x411c, 0x84, 0x1d, 0xbd, 0x38, 0xfa, 0xcd, 0xea, 0xa0}
RIDL!{#[uuid(0x177160ca, 0xbb5a, 0x411c, 0x84, 0x1d, 0xbd, 0x38, 0xfa, 0xcd, 0xea, 0xa0)]
interface IShellDispatch3(IShellDispatch3Vtbl): IShellDispatch2(IShellDispatch2Vtbl) {
    fn AddToRecent(
        varFile: VARIANT,
        bstrCategory: BSTR,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IShellDispatch4,
    0xefd84b2d, 0x4bcf, 0x4298, 0xbe, 0x25, 0xeb, 0x54, 0x2a, 0x59, 0xfb, 0xda}
RIDL!{#[uuid(0xefd84b2d, 0x4bcf, 0x4298, 0xbe, 0x25, 0xeb, 0x54, 0x2a, 0x59, 0xfb, 0xda)]
interface IShellDispatch4(IShellDispatch4Vtbl): IShellDispatch3(IShellDispatch3Vtbl) {
    fn WindowsSecurity() -> HRESULT,
    fn ToggleDesktop() -> HRESULT,
    fn ExplorerPolicy(
        bstrPolicyName: BSTR,
        pValue: *mut VARIANT,
    ) -> HRESULT,
    fn GetSetting(
        lSetting: c_long,
        pResult: *mut VARIANT_BOOL,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IShellDispatch5,
    0x866738b9, 0x6cf2, 0x4de8, 0x87, 0x67, 0xf7, 0x94, 0xeb, 0xe7, 0x4f, 0x4e}
RIDL!{#[uuid(0x866738b9, 0x6cf2, 0x4de8, 0x87, 0x67, 0xf7, 0x94, 0xeb, 0xe7, 0x4f, 0x4e)]
interface IShellDispatch5(IShellDispatch5Vtbl): IShellDispatch4(IShellDispatch4Vtbl) {
    fn WindowSwitcher() -> HRESULT,
}}
DEFINE_GUID!{IID_IShellDispatch6,
    0x286e6f1b, 0x7113, 0x4355, 0x95, 0x62, 0x96, 0xb7, 0xe9, 0xd6, 0x4c, 0x54}
RIDL!{#[uuid(0x286e6f1b, 0x7113, 0x4355, 0x95, 0x62, 0x96, 0xb7, 0xe9, 0xd6, 0x4c, 0x54)]
interface IShellDispatch6(IShellDispatch6Vtbl): IShellDispatch5(IShellDispatch5Vtbl) {
    fn SearchCommand() -> HRESULT,
}}
DEFINE_GUID!{CLSID_Shell,
    0x13709620, 0xC279, 0x11CE, 0xA4, 0x9E, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00}
// class DECLSPEC_UUID("13709620-C279-11CE-A49E-444553540000")
// Shell;
DEFINE_GUID!{CLSID_ShellDispatchInproc,
    0x0A89A860, 0xD7B1, 0x11CE, 0x83, 0x50, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00}
// class DECLSPEC_UUID("0A89A860-D7B1-11CE-8350-444553540000")
// ShellDispatchInproc;
DEFINE_GUID!{IID_IFileSearchBand,
    0x2d91eea1, 0x9932, 0x11d2, 0xbe, 0x86, 0x00, 0xa0, 0xc9, 0xa8, 0x3d, 0xa1}
RIDL!{#[uuid(0x2d91eea1, 0x9932, 0x11d2, 0xbe, 0x86, 0x00, 0xa0, 0xc9, 0xa8, 0x3d, 0xa1)]
interface IFileSearchBand(IFileSearchBandVtbl): IDispatch(IDispatchVtbl) {
    fn SetFocus() -> HRESULT,
    fn SetSearchParameters(
        pbstrSearchID: *mut BSTR,
        bNavToResults: VARIANT_BOOL,
        pvarScope: *mut VARIANT,
        pvarQueryFile: *mut VARIANT,
    ) -> HRESULT,
    fn get_SearchID(
        pbstrSearchID: *mut BSTR,
    ) -> HRESULT,
    fn get_Scope(
        pvarScope: *mut VARIANT,
    ) -> HRESULT,
    fn get_QueryFile(
        pvarFile: *mut VARIANT,
    ) -> HRESULT,
}}
DEFINE_GUID!{CLSID_FileSearchBand,
    0xC4EE31F3, 0x4768, 0x11D2, 0xBE, 0x5C, 0x00, 0xA0, 0xC9, 0xA8, 0x3D, 0xA1}
// class DECLSPEC_UUID("C4EE31F3-4768-11D2-BE5C-00A0C9A83DA1")
// FileSearchBand;
DEFINE_GUID!{IID_IWebWizardHost,
    0x18bcc359, 0x4990, 0x4bfb, 0xb9, 0x51, 0x3c, 0x83, 0x70, 0x2b, 0xe5, 0xf9}
RIDL!{#[uuid(0x18bcc359, 0x4990, 0x4bfb, 0xb9, 0x51, 0x3c, 0x83, 0x70, 0x2b, 0xe5, 0xf9)]
interface IWebWizardHost(IWebWizardHostVtbl): IDispatch(IDispatchVtbl) {
    fn FinalBack() -> HRESULT,
    fn FinalNext() -> HRESULT,
    fn Cancel() -> HRESULT,
    fn put_Caption(
        bstrCaption: BSTR,
    ) -> HRESULT,
    fn get_Caption(
        pbstrCaption: *mut BSTR,
    ) -> HRESULT,
    fn put_Property(
        bstrPropertyName: BSTR,
        pvProperty: *mut VARIANT,
    ) -> HRESULT,
    fn get_Property(
        bstrPropertyName: BSTR,
        pvProperty: *mut VARIANT,
    ) -> HRESULT,
    fn SetWizardButtons(
        vfEnableBack: VARIANT_BOOL,
        vfEnableNext: VARIANT_BOOL,
        vfLastPage: VARIANT_BOOL,
    ) -> HRESULT,
    fn SetHeaderText(
        bstrHeaderTitle: BSTR,
        bstrHeaderSubtitle: BSTR,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IWebWizardHost2,
    0xf9c013dc, 0x3c23, 0x4041, 0x8e, 0x39, 0xcf, 0xb4, 0x02, 0xf7, 0xea, 0x59}
RIDL!{#[uuid(0xf9c013dc, 0x3c23, 0x4041, 0x8e, 0x39, 0xcf, 0xb4, 0x02, 0xf7, 0xea, 0x59)]
interface IWebWizardHost2(IWebWizardHost2Vtbl): IWebWizardHost(IWebWizardHostVtbl) {
    fn SignString(
        value: BSTR,
        signedValue: *mut BSTR,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_INewWDEvents,
    0x0751c551, 0x7568, 0x41c9, 0x8e, 0x5b, 0xe2, 0x2e, 0x38, 0x91, 0x92, 0x36}
RIDL!{#[uuid(0x0751c551, 0x7568, 0x41c9, 0x8e, 0x5b, 0xe2, 0x2e, 0x38, 0x91, 0x92, 0x36)]
interface INewWDEvents(INewWDEventsVtbl): IWebWizardHost(IWebWizardHostVtbl) {
    fn PassportAuthenticate(
        bstrSignInUrl: BSTR,
        pvfAuthenitcated: *mut VARIANT_BOOL,
    ) -> HRESULT,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shldisp_0000_0001_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shldisp_0000_0001_v0_0_s_ifspec;
pub type LPAUTOCOMPLETE = *mut IAutoComplete;
DEFINE_GUID!{IID_IAutoComplete,
    0x00bb2762, 0x6a77, 0x11d0, 0xa5, 0x35, 0x00, 0xc0, 0x4f, 0xd7, 0xd0, 0x62}
RIDL!{#[uuid(0x00bb2762, 0x6a77, 0x11d0, 0xa5, 0x35, 0x00, 0xc0, 0x4f, 0xd7, 0xd0, 0x62)]
interface IAutoComplete(IAutoCompleteVtbl): IUnknown(IUnknownVtbl) {
    fn Init(
        hwndEdit: HWND,
        punkACL: *mut IUnknown,
        pwszRegKeyPath: LPCWSTR,
        pwszQuickComplete: LPCWSTR,
    ) -> HRESULT,
    fn Enable(
        fEnable: BOOL,
    ) -> HRESULT,
}}
pub type LPAUTOCOMPLETE2 = *mut IAutoComplete2;
ENUM!{enum AUTOCOMPLETEOPTIONS {
    ACO_NONE = 0,
    ACO_AUTOSUGGEST = 0x1,
    ACO_AUTOAPPEND = 0x2,
    ACO_SEARCH = 0x4,
    ACO_FILTERPREFIXES = 0x8,
    ACO_USETAB = 0x10,
    ACO_UPDOWNKEYDROPSLIST = 0x20,
    ACO_RTLREADING = 0x40,
    ACO_WORD_FILTER = 0x80,
    ACO_NOPREFIXFILTERING = 0x100,
}}
DEFINE_GUID!{IID_IAutoComplete2,
    0xeac04bc0, 0x3791, 0x11d2, 0xbb, 0x95, 0x00, 0x60, 0x97, 0x7b, 0x46, 0x4c}
RIDL!{#[uuid(0xeac04bc0, 0x3791, 0x11d2, 0xbb, 0x95, 0x00, 0x60, 0x97, 0x7b, 0x46, 0x4c)]
interface IAutoComplete2(IAutoComplete2Vtbl): IAutoComplete(IAutoCompleteVtbl) {
    fn SetOptions(
        dwFlag: DWORD,
    ) -> HRESULT,
    fn GetOptions(
        pdwFlag: *mut DWORD,
    ) -> HRESULT,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shldisp_0000_0003_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shldisp_0000_0003_v0_0_s_ifspec;
pub type PENUMACSTRING = *mut IEnumACString;
pub type LPENUMACSTRING = *mut IEnumACString;
ENUM!{enum ACENUMOPTION {
    ACEO_NONE = 0,
    ACEO_MOSTRECENTFIRST = 0x1,
    ACEO_FIRSTUNUSED = 0x10000,
}}
DEFINE_GUID!{IID_IEnumACString,
    0x8e74c210, 0xcf9d, 0x4eaf, 0xa4, 0x03, 0x73, 0x56, 0x42, 0x8f, 0x0a, 0x5a}
RIDL!{#[uuid(0x8e74c210, 0xcf9d, 0x4eaf, 0xa4, 0x03, 0x73, 0x56, 0x42, 0x8f, 0x0a, 0x5a)]
interface IEnumACString(IEnumACStringVtbl): IEnumString(IEnumStringVtbl) {
    fn NextItem(
        pszUrl: LPWSTR,
        cchMax: ULONG,
        pulSortIndex: *mut ULONG,
    ) -> HRESULT,
    fn SetEnumOptions(
        dwOptions: DWORD,
    ) -> HRESULT,
    fn GetEnumOptions(
        pdwOptions: *mut DWORD,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IDataObjectAsyncCapability,
    0x3d8b0590, 0xf691, 0x11d2, 0x8e, 0xa9, 0x00, 0x60, 0x97, 0xdf, 0x5b, 0xd4}
RIDL!{#[uuid(0x3d8b0590, 0xf691, 0x11d2, 0x8e, 0xa9, 0x00, 0x60, 0x97, 0xdf, 0x5b, 0xd4)]
interface IDataObjectAsyncCapability(IDataObjectAsyncCapabilityVtbl): IUnknown(IUnknownVtbl) {
    fn SetAsyncMode(
        fDoOpAsync: BOOL,
    ) -> HRESULT,
    fn GetAsyncMode(
        pfIsOpAsync: *mut BOOL,
    ) -> HRESULT,
    fn StartOperation(
        pbcReserved: *mut IBindCtx,
    ) -> HRESULT,
    fn InOperation(
        pfInAsyncOp: *mut BOOL,
    ) -> HRESULT,
    fn EndOperation(
        hResult: HRESULT,
        pbcReserved: *mut IBindCtx,
        dwEffects: DWORD,
    ) -> HRESULT,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shldisp_0000_0005_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shldisp_0000_0005_v0_0_s_ifspec;

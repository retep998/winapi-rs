// Copyright © 2015-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.

use ctypes::{c_int, c_void};
use shared::guiddef::{REFGUID, REFIID};
use shared::minwindef::{BOOL, DWORD, ULONG, WORD};
use shared::windef::{HWND};
use um::shtypes::{PIDLIST_ABSOLUTE, PCIDLIST_ABSOLUTE};
use um::objidl::IBindCtx;
use um::minwinbase::{WIN32_FIND_DATAA};
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::{HRESULT, LPCSTR, LPSTR, LPCWSTR, LPWSTR};

DEFINE_GUID!{CLSID_TaskbarList,
    0x56fdf344, 0xfd6d, 0x11d0, 0x95, 0x8a, 0x00, 0x60, 0x97, 0xc9, 0xa0, 0x90}
//4498
pub type SFGAOF = ULONG;
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
RIDL!(#[uuid(0x43826d1e, 0xe718, 0x42ee, 0xbc, 0x55, 0xa1, 0xe2, 0x61, 0xc3, 0x7b, 0xfe)]
interface IShellItem(IShellItemVtbl): IUnknown(IUnknownVtbl) {
    fn BindToHandler(
        pbc: *mut IBindCtx,
        bhid: REFGUID,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
    fn GetParent(
        ppsi: *mut *mut IShellItem,
    ) -> HRESULT,
    fn GetDisplayName(
        sigdnName: SIGDN,
        ppszName: *mut LPWSTR,
    ) -> HRESULT,
    fn GetAttributes(
        sfgaoMask: SFGAOF,
        psfgaoAttribs: *mut SFGAOF,
    ) -> HRESULT,
    fn Compare(
        psi: *mut IShellItem,
        hint: SICHINTF,
        piOrder: *mut c_int,
    ) -> HRESULT,
});
//20869
RIDL!(#[uuid(0xb4db1657, 0x70d7, 0x485e, 0x8e, 0x3e, 0x6f, 0xcb, 0x5a, 0x5c, 0x18, 0x02)]
interface IModalWindow(IModalWindowVtbl): IUnknown(IUnknownVtbl) {
    fn Show(
        hwndOwner: HWND,
    ) -> HRESULT,
});
//22307
//27457
pub type IShellItemFilter = IUnknown; // TODO

RIDL!{#[uuid(0x56fdf342, 0xfd6d, 0x11d0, 0x95, 0x8a, 0x00, 0x60, 0x97, 0xc9, 0xa0, 0x90)]
interface ITaskbarList(ITaskbarListVtbl): IUnknown(IUnknownVtbl) {
    fn HrInit() -> HRESULT,
    fn AddTab(
        hwnd: HWND,
    ) -> HRESULT,
    fn DeleteTab(
        hwnd: HWND,
    ) -> HRESULT,
    fn ActivateTab(
        hwnd: HWND,
    ) -> HRESULT,
    fn SetActiveAlt(
        hwnd: HWND,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x602d4995, 0xb13a, 0x429b, 0xa6, 0x6e, 0x19, 0x35, 0xe4, 0x4f, 0x43, 0x17)]
interface ITaskbarList2(ITaskbarList2Vtbl): ITaskbarList(ITaskbarListVtbl) {
    fn MarkFullscreenWindow(
        hwnd: HWND,
        fFullscreen: BOOL,
    ) -> HRESULT,    
}}

ENUM!{enum SLR_FLAGS {
    SLR_NONE	                    = 0,
    SLR_NO_UI	                    = 0x1,
    SLR_ANY_MATCH	                = 0x2,
    SLR_UPDATE	                    = 0x4,
    SLR_NOUPDATE	                = 0x8,
    SLR_NOSEARCH	                = 0x10,
    SLR_NOTRACK	                    = 0x20,
    SLR_NOLINKINFO	                = 0x40,
    SLR_INVOKE_MSI	                = 0x80,
    SLR_NO_UI_WITH_MSG_PUMP	        = 0x101,
    SLR_OFFER_DELETE_WITHOUT_FILE	= 0x200,
    SLR_KNOWNFOLDER	                = 0x400,
    SLR_MACHINE_IN_LOCAL_TARGET	    = 0x800,
    SLR_UPDATE_MACHINE_AND_SID	    = 0x1000,
    SLR_NO_OBJECT_ID	            = 0x2000,
}}
ENUM!{enum SLGP_FLAGS {
    SLGP_SHORTPATH	                = 0x1,
    SLGP_UNCPRIORITY	            = 0x2,
    SLGP_RAWPATH	                = 0x4,
    SLGP_RELATIVEPRIORITY	        = 0x8,
}}
DEFINE_GUID!{CLSID_ShellLink,
    0x0002140, 0x0000, 0x0000, 0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!(#[uuid(0x000214EE, 0, 0, 0xC0,0,0,0,0,0,0,0x46)]
interface IShellLinkA(IShellLinkAVtbl): IUnknown(IUnknownVtbl) {
    fn GetPath(
        pszFile: LPSTR,
        cch: c_int,
        pfd: *mut WIN32_FIND_DATAA,
        fFlags: DWORD,
    ) -> HRESULT,
    fn GetIDList(
        ppidl: *mut PIDLIST_ABSOLUTE,
    ) -> HRESULT,
    fn SetIDList(
        pidl: PCIDLIST_ABSOLUTE,
    ) -> HRESULT,
    fn GetDescription(
        pszName: LPSTR,
        cch: c_int,
    ) -> HRESULT,
    fn SetDescription(
        pszName: LPCSTR,
    ) -> HRESULT,
    fn GetWorkingDirectory(
        pszDir: LPSTR,
        cch: c_int,
    ) -> HRESULT,
    fn SetWorkingDirectory(
        pszDir: LPCSTR,
    ) -> HRESULT,
    fn GetArguments(
        pszArgs: LPSTR,
        cch: c_int,
    ) -> HRESULT,
    fn SetArguments(
        pszArgs: LPCSTR,
    ) -> HRESULT,
    fn GetHotkey(
        pwHotkey: *mut WORD,
    ) -> HRESULT,
    fn SetHotkey(
        wHotkey: WORD,
    ) -> HRESULT,
    fn GetShowCmd(
        piShowCmd: *mut c_int,
    ) -> HRESULT,
    fn SetShowCmd(
        iShowCmd: c_int,
    ) -> HRESULT,
    fn GetIconLocation(
        pszIconPath: LPSTR,
        cch: c_int,
        piIcon: *mut c_int,
    ) -> HRESULT,
    fn SetIconLocation(
        pszIconPath: LPCSTR,
        iIcon: c_int,
    ) -> HRESULT,
    fn SetRelativePath(
        pszPathRel: LPCSTR,
        dwReserved: DWORD,
    ) -> HRESULT,
    fn Resolve(
        hwnd: HWND,
        fFlags: DWORD,
    ) -> HRESULT,
    fn SetPath(
        pszFile: LPCSTR,
    ) -> HRESULT,
});
RIDL!(#[uuid(0x000214F9, 0, 0, 0xC0,0,0,0,0,0,0,0x46)]
interface IShellLinkW(IShellLinkWVtbl): IUnknown(IUnknownVtbl) {
    fn GetPath(
        pszFile: LPWSTR,
        cch: c_int,
        pfd: *mut WIN32_FIND_DATAA,
        fFlags: DWORD,
    ) -> HRESULT,
    fn GetIDList(
        ppidl: *mut PIDLIST_ABSOLUTE,
    ) -> HRESULT,
    fn SetIDList(
        pidl: PCIDLIST_ABSOLUTE,
    ) -> HRESULT,
    fn GetDescription(
        pszName: LPWSTR,
        cch: c_int,
    ) -> HRESULT,
    fn SetDescription(
        pszName: LPCWSTR,
    ) -> HRESULT,
    fn GetWorkingDirectory(
        pszDir: LPWSTR,
        cch: c_int,
    ) -> HRESULT,
    fn SetWorkingDirectory(
        pszDir: LPCWSTR,
    ) -> HRESULT,
    fn GetArguments(
        pszArgs: LPWSTR,
        cch: c_int,
    ) -> HRESULT,
    fn SetArguments(
        pszArgs: LPCWSTR,
    ) -> HRESULT,
    fn GetHotkey(
        pwHotkey: *mut WORD,
    ) -> HRESULT,
    fn SetHotkey(
        wHotkey: WORD,
    ) -> HRESULT,
    fn GetShowCmd(
        piShowCmd: *mut c_int,
    ) -> HRESULT,
    fn SetShowCmd(
        iShowCmd: c_int,
    ) -> HRESULT,
    fn GetIconLocation(
        pszIconPath: LPWSTR,
        cch: c_int,
        piIcon: *mut c_int,
    ) -> HRESULT,
    fn SetIconLocation(
        pszIconPath: LPCWSTR,
        iIcon: c_int,
    ) -> HRESULT,
    fn SetRelativePath(
        pszPathRel: LPCWSTR,
        dwReserved: DWORD,
    ) -> HRESULT,
    fn Resolve(
        hwnd: HWND,
        fFlags: DWORD,
    ) -> HRESULT,
    fn SetPath(
        pszFile: LPCWSTR,
    ) -> HRESULT,
});


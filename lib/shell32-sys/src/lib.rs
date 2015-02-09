// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to shell32.
#![cfg(all(windows, any(target_arch = "x86", target_arch = "x86_64")))]
extern crate winapi;
use winapi::*;
extern "system" {
    // pub fn AssocCreateForClasses();
    // pub fn AssocGetDetailsOfPropKey();
    // pub fn CheckEscapesW();
    // pub fn CommandLineToArgvW();
    // pub fn DoEnvironmentSubstA();
    // pub fn DoEnvironmentSubstW();
    // pub fn DragAcceptFiles();
    // pub fn DragFinish();
    // pub fn DragQueryFile();
    // pub fn DragQueryFileA();
    // pub fn DragQueryFileAorW();
    // pub fn DragQueryFileW();
    // pub fn DragQueryPoint();
    // pub fn DuplicateIcon();
    // pub fn ExtractAssociatedIconA();
    // pub fn ExtractAssociatedIconExA();
    // pub fn ExtractAssociatedIconExW();
    // pub fn ExtractAssociatedIconW();
    // pub fn ExtractIconA();
    // pub fn ExtractIconEx();
    // pub fn ExtractIconExA();
    // pub fn ExtractIconExW();
    // pub fn ExtractIconW();
    // pub fn FindExecutableA();
    // pub fn FindExecutableW();
    // pub fn GetCurrentProcessExplicitAppUserModelID();
    // pub fn InitNetworkAddressControl();
    // pub fn PathCleanupSpecWorker();
    // pub fn PathIsExeWorker();
    // pub fn SHAddDefaultPropertiesByExt();
    // pub fn SHAddToRecentDocs();
    // pub fn SHAppBarMessage();
    // pub fn SHAssocEnumHandlers();
    // pub fn SHAssocEnumHandlersForProtocolByApplication();
    // pub fn SHBindToFolderIDListParent();
    // pub fn SHBindToFolderIDListParentEx();
    // pub fn SHBindToObject();
    // pub fn SHBindToParent();
    // pub fn SHBrowseForFolder();
    // pub fn SHBrowseForFolderA();
    // pub fn SHBrowseForFolderW();
    // pub fn SHChangeNotify();
    // pub fn SHChangeNotifyRegisterThread();
    pub fn SHCloneSpecialIDList(hwnd: HWND, csidl: c_int, fCreate: BOOL) -> PIDLIST_ABSOLUTE;
    // pub fn SHCoCreateInstanceWorker();
    // pub fn SHCreateAssociationRegistration();
    // pub fn SHCreateDataObject();
    // pub fn SHCreateDefaultContextMenu();
    // pub fn SHCreateDefaultExtractIcon();
    // pub fn SHCreateDefaultPropertiesOp();
    pub fn SHCreateDirectory(hwnd: HWND, pszPath: PCWSTR) -> c_int;
    pub fn SHCreateDirectoryExA(
        hwnd: HWND, pszPath: LPCSTR, psa: *const SECURITY_ATTRIBUTES,
    ) -> c_int;
    pub fn SHCreateDirectoryExW(
        hwnd: HWND, pszPath: LPCWSTR, psa: *const SECURITY_ATTRIBUTES,
    ) -> c_int;
    // pub fn SHCreateDirectoryExWWorker();
    // pub fn SHCreateItemFromIDList();
    // pub fn SHCreateItemFromParsingName();
    // pub fn SHCreateItemFromRelativeName();
    // pub fn SHCreateItemInKnownFolder();
    // pub fn SHCreateItemWithParent();
    // pub fn SHCreateProcessAsUserW();
    // pub fn SHCreateQueryCancelAutoPlayMoniker();
    pub fn SHCreateShellItem(
        pidlParent: PCIDLIST_ABSOLUTE, psfParent: *mut IShellFolder, pidl: PCUITEMID_CHILD,
        ppsi: *mut *mut IShellItem,
    ) -> HRESULT;
    // pub fn SHCreateShellItemArray();
    // pub fn SHCreateShellItemArrayFromDataObject();
    // pub fn SHCreateShellItemArrayFromIDLists();
    // pub fn SHCreateShellItemArrayFromShellItem();
    // pub fn SHEmptyRecycleBinA();
    // pub fn SHEmptyRecycleBinW();
    // pub fn SHEnumerateUnreadMailAccountsW();
    // pub fn SHEvaluateSystemCommandTemplate();
    // pub fn SHFileOperation();
    // pub fn SHFileOperationA();
    // pub fn SHFileOperationW();
    pub fn SHFlushSFCache();
    // pub fn SHFormatDrive();
    // pub fn SHFreeNameMappings();
    // pub fn SHGetDataFromIDListA();
    // pub fn SHGetDataFromIDListW();
    // pub fn SHGetDesktopFolder();
    // pub fn SHGetDesktopFolderWorker();
    // pub fn SHGetDiskFreeSpaceA();
    // pub fn SHGetDiskFreeSpaceExA();
    // pub fn SHGetDiskFreeSpaceExW();
    // pub fn SHGetDriveMedia();
    // pub fn SHGetFileInfo();
    // pub fn SHGetFileInfoA();
    // pub fn SHGetFileInfoW();
    // pub fn SHGetFileInfoWWorker();
    pub fn SHGetFolderLocation(
        hwnd: HWND, csidl: c_int, hToken: HANDLE, dwFlags: DWORD, ppidl: *mut PIDLIST_ABSOLUTE,
    ) -> HRESULT;
    // pub fn SHGetFolderLocationWorker();
    pub fn SHGetFolderPathA(
        hwnd: HWND, csidl: c_int, hToken: HANDLE, dwFlags: DWORD, pszPath: LPSTR,
    ) -> HRESULT;
    // pub fn SHGetFolderPathAWorker();
    pub fn SHGetFolderPathAndSubDirA(
        hwnd: HWND, csidl: c_int, hToken: HANDLE, dwFlags: DWORD, pszSubDir: LPCSTR,
        pszPath: LPSTR,
    ) -> HRESULT;
    pub fn SHGetFolderPathAndSubDirW(
        hwnd: HWND, csidl: c_int, hToken: HANDLE, dwFlags: DWORD, pszSubDir: LPCWSTR,
        pszPath: LPWSTR,
    ) -> HRESULT;
    // pub fn SHGetFolderPathAndSubDirWWorker();
    // pub fn SHGetFolderPathEx();
    pub fn SHGetFolderPathW(
        hwnd: HWND, csidl: c_int, hToken: HANDLE, dwFlags: DWORD, pszPath: LPWSTR,
    ) -> HRESULT;
    // pub fn SHGetFolderPathWWorker();
    // pub fn SHGetIDListFromObject();
    pub fn SHGetIconOverlayIndexA(pszIconPath: LPCSTR, iIconIndex: c_int) -> c_int;
    pub fn SHGetIconOverlayIndexW(pszIconPath: LPCWSTR, iIconIndex: c_int) -> c_int;
    // pub fn SHGetInstanceExplorer();
    // pub fn SHGetInstanceExplorerWorker();
    // pub fn SHGetItemFromDataObject();
    // pub fn SHGetItemFromObject();
    pub fn SHGetKnownFolderIDList(
        rfid: REFKNOWNFOLDERID, dwFlags: DWORD, hToken: HANDLE, ppidl: *mut PIDLIST_ABSOLUTE,
    ) -> HRESULT;
    pub fn SHGetKnownFolderItem(
        rfid: REFKNOWNFOLDERID, flags: KNOWN_FOLDER_FLAG, hToken: HANDLE, riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT;
    pub fn SHGetKnownFolderPath(
        rfid: REFKNOWNFOLDERID, dwFlags: DWORD, hToken: HANDLE, pszPath: *mut PWSTR,
    ) -> HRESULT;
    // pub fn SHGetKnownFolderPathWorker();
    // pub fn SHGetLocalizedName();
    // pub fn SHGetMalloc();
    // pub fn SHGetNameFromIDList();
    // pub fn SHGetPathFromIDList();
    pub fn SHGetPathFromIDListA(pidl: PCIDLIST_ABSOLUTE, pszPath: LPSTR) -> BOOL;
    pub fn SHGetPathFromIDListEx(
        pidl: PCIDLIST_ABSOLUTE, pszPath: PWSTR, cchPath: DWORD, uOpts: GPFIDL_FLAGS,
    ) -> BOOL;
    pub fn SHGetPathFromIDListW(pidl: PCIDLIST_ABSOLUTE, pszPath: LPWSTR) -> BOOL;
    // pub fn SHGetPropertyStoreForWindow();
    // pub fn SHGetPropertyStoreFromIDList();
    // pub fn SHGetPropertyStoreFromParsingName();
    // pub fn SHGetSettings();
    pub fn SHGetSpecialFolderLocation(
        hwnd: HWND, csidl: c_int, ppidl: *mut PIDLIST_ABSOLUTE,
    ) -> HRESULT;
    pub fn SHGetSpecialFolderPathA(
        hwnd: HWND, pszPath: LPSTR, csidl: c_int, fCreate: BOOL,
    ) -> BOOL;
    // pub fn SHGetSpecialFolderPathAWorker();
    pub fn SHGetSpecialFolderPathW(
        hwnd: HWND, pszPath: LPWSTR, csidl: c_int, fCreate: BOOL,
    ) -> BOOL;
    // pub fn SHGetSpecialFolderPathWWorker();
    // pub fn SHGetStockIconInfo();
    // pub fn SHGetTemporaryPropertyForItem();
    // pub fn SHGetUnreadMailCountW();
    // pub fn SHInvokePrinterCommandA();
    // pub fn SHInvokePrinterCommandW();
    // pub fn SHIsFileAvailableOffline();
    // pub fn SHLoadInProc();
    // pub fn SHLoadNonloadedIconOverlayIdentifiers();
    pub fn SHOpenFolderAndSelectItems(
        pidlFolder: PCIDLIST_ABSOLUTE, cidl: UINT, apidl: PCUITEMID_CHILD_ARRAY, dwFlags: DWORD,
    ) -> HRESULT;
    // pub fn SHOpenWithDialog();
    // pub fn SHParseDisplayName();
    // pub fn SHPathPrepareForWriteA();
    // pub fn SHPathPrepareForWriteW();
    // pub fn SHQueryRecycleBinA();
    // pub fn SHQueryRecycleBinW();
    // pub fn SHQueryUserNotificationState();
    // pub fn SHRemoveLocalizedName();
    // pub fn SHResolveLibrary();
    // pub fn SHSetDefaultProperties();
    pub fn SHSetFolderPathA(
        csidl: c_int, hToken: HANDLE, dwFlags: DWORD, pszPath: LPCSTR,
    ) -> HRESULT;
    pub fn SHSetFolderPathW(
        csidl: c_int, hToken: HANDLE, dwFlags: DWORD, pszPath: LPCWSTR,
    ) -> HRESULT;
    pub fn SHSetKnownFolderPath(
        rfid: REFKNOWNFOLDERID, dwFlags: DWORD, hToken: HANDLE, pszPath: PCWSTR,
    ) -> HRESULT;
    // pub fn SHSetKnownFolderPathWorker();
    // pub fn SHSetLocalizedName();
    // pub fn SHSetTemporaryPropertyForItem();
    // pub fn SHSetUnreadMailCountW();
    // pub fn SHShowManageLibraryUI();
    // pub fn SetCurrentProcessExplicitAppUserModelID();
    // pub fn ShellAboutA();
    // pub fn ShellAboutW();
    // pub fn ShellExecuteA();
    // pub fn ShellExecuteEx();
    // pub fn ShellExecuteExA();
    // pub fn ShellExecuteExW();
    // pub fn ShellExecuteW();
    // pub fn ShellHookProc();
    // pub fn Shell_GetCachedImageIndexA();
    // pub fn Shell_GetCachedImageIndexW();
    // pub fn Shell_NotifyIcon();
    // pub fn Shell_NotifyIconA();
    // pub fn Shell_NotifyIconGetRect();
    // pub fn Shell_NotifyIconW();
    // pub fn WOWShellExecute();
}

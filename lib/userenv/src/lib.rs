// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to userenv.
#![cfg(windows)]
extern crate winapi;
use winapi::*;
extern "system" {
    // pub fn AreThereVisibleLogoffScripts();
    // pub fn AreThereVisibleShutdownScripts();
    // pub fn CheckDirectoryOwnership();
    // pub fn CheckXForestLogon();
    // pub fn CopyProfileDirectoryEx2();
    // pub fn CreateAppContainerProfile();
    // pub fn CreateAppContainerProfileInternal();
    // pub fn CreateDirectoryJunctionsForSystem();
    // pub fn CreateDirectoryJunctionsForUserProfile();
    pub fn CreateEnvironmentBlock(
        lpEnvironment: *mut LPVOID, hToken: HANDLE, bInherit: BOOL,
    ) -> BOOL;
    // pub fn CreateGroupEx();
    // pub fn CreateLinkFileEx();
    pub fn CreateProfile(
        pszUserSid: LPCWSTR, pszUserName: LPCWSTR, pszProfilePath: LPWSTR, cchProfilePath: DWORD,
    ) -> HRESULT;
    // pub fn DeleteAppContainerProfile();
    // pub fn DeleteAppContainerProfileInternal();
    // pub fn DeleteGroup();
    // pub fn DeleteLinkFile();
    pub fn DeleteProfileA(
        lpSidString: LPCSTR, lpProfilePath: LPCSTR, lpComputerName: LPCSTR,
    ) -> BOOL;
    // pub fn DeleteProfileDirectory();
    pub fn DeleteProfileW(
        lpSidString: LPCWSTR, lpProfilePath: LPCWSTR, lpComputerName: LPCWSTR,
    ) -> BOOL;
    // pub fn DeriveAppContainerSidFromAppContainerName();
    // pub fn DeriveRestrictedAppContainerSidFromAppContainerSidAndRestrictedName();
    pub fn DestroyEnvironmentBlock(lpEnvironment: LPVOID) -> BOOL;
    pub fn EnterCriticalPolicySection(bMachine: BOOL) -> HANDLE;
    pub fn ExpandEnvironmentStringsForUserA(
        hToken: HANDLE, lpSrc: LPCSTR, lpDest: LPSTR, dwSize: DWORD,
    ) -> BOOL;
    pub fn ExpandEnvironmentStringsForUserW(
        hToken: HANDLE, lpSrc: LPCWSTR, lpDest: LPWSTR, dwSize: DWORD,
    ) -> BOOL;
    // pub fn ForceSyncFgPolicy();
    // pub fn FreeGPOListA();
    // pub fn FreeGPOListW();
    // pub fn GenerateGPNotification();
    pub fn GetAllUsersProfileDirectoryA(lpProfileDir: LPSTR, lpcchSize: LPDWORD) -> BOOL;
    pub fn GetAllUsersProfileDirectoryW(lpProfileDir: LPWSTR, lpcchSize: LPDWORD) -> BOOL;
    // pub fn GetAppContainerFolderPath();
    // pub fn GetAppContainerRegistryLocation();
    // pub fn GetAppliedGPOListA();
    // pub fn GetAppliedGPOListW();
    pub fn GetDefaultUserProfileDirectoryA(lpProfileDir: LPSTR, lpcchSize: LPDWORD) -> BOOL;
    pub fn GetDefaultUserProfileDirectoryW(lpProfileDir: LPWSTR, lpcchSize: LPDWORD) -> BOOL;
    // pub fn GetGPOListA();
    // pub fn GetGPOListW();
    // pub fn GetLongProfilePathName();
    // pub fn GetNextFgPolicyRefreshInfo();
    // pub fn GetPreviousFgPolicyRefreshInfo();
    pub fn GetProfileType(dwFlags: *mut DWORD) -> BOOL;
    pub fn GetProfilesDirectoryA(lpProfileDir: LPSTR, lpcchSize: LPDWORD) -> BOOL;
    pub fn GetProfilesDirectoryW(lpProfileDir: LPWSTR, lpcchSize: LPDWORD) -> BOOL;
    pub fn GetUserProfileDirectoryA(
        hToken: HANDLE, lpProfileDir: LPSTR, lpcchSize: LPDWORD,
    ) -> BOOL;
    // pub fn GetUserProfileDirectoryForUserSidW();
    pub fn GetUserProfileDirectoryW(
        hToken: HANDLE, lpProfileDir: LPWSTR, lpcchSize: LPDWORD,
    ) -> BOOL;
    // pub fn HasPolicyForegroundProcessingCompleted();
    // pub fn IsAppContainerProfilePresentInternal();
    pub fn LeaveCriticalPolicySection(hSection: HANDLE) -> BOOL;
    // pub fn LoadUserProfileA(hToken: HANDLE, lpProfileInfo: LPPROFILEINFOA) -> BOOL;
    // pub fn LoadUserProfileW(hToken: HANDLE, lpProfileInfo: LPPROFILEINFOW) -> BOOL;
    // pub fn LookupAppContainerDisplayName();
    // pub fn PingComputer();
    // pub fn ProcessGroupPolicyCompleted();
    // pub fn ProcessGroupPolicyCompletedEx();
    pub fn RefreshPolicy(bMachine: BOOL) -> BOOL;
    pub fn RefreshPolicyEx(bMachine: BOOL, dwOptions: DWORD) -> BOOL;
    pub fn RegisterGPNotification(hEvent: HANDLE, bMachine: BOOL) -> BOOL;
    // pub fn RemapProfile();
    // pub fn RsopAccessCheckByType();
    // pub fn RsopFileAccessCheck();
    // pub fn RsopResetPolicySettingStatus();
    // pub fn RsopSetPolicySettingStatus();
    pub fn UnloadUserProfile(hToken: HANDLE, hProfile: HANDLE) -> BOOL;
    pub fn UnregisterGPNotification(hEvent: HANDLE) -> BOOL;
    // pub fn UpdateAppContainerProfile();
    // pub fn WaitForMachinePolicyForegroundProcessing();
    // pub fn WaitForUserPolicyForegroundProcessing();
}

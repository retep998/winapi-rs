// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to vssapi.
#![cfg(windows)]
#![allow(non_snake_case)]
extern crate winapi;
use winapi::*;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
extern "system" {
    #[link_name="CreateVssBackupComponentsInternal"]
    pub fn CreateVssBackupComponents(ppBackup: *mut *mut IVssBackupComponents) -> HRESULT;
    #[link_name="CreateVssExamineWriterMetadataInternal"]
    pub fn CreateVssExamineWriterMetadata(
        bstrXML: BSTR, ppMetadata: *mut *mut IVssExamineWriterMetadata
    ) -> HRESULT;
    #[link_name="IsVolumeSnapshottedInternal"]
    pub fn IsVolumeSnapshotted(
        pwszVolumeName: VSS_PWSZ, pbSnapshotsPresent: *mut BOOL, plSnapshotCapability: *mut LONG
    ) -> HRESULT;
    #[link_name="VssFreeSnapshotPropertiesInternal"]
    pub fn VssFreeSnapshotProperties(pProp: *mut VSS_SNAPSHOT_PROP);
    #[link_name="GetProviderMgmtInterfaceInternal"]
    pub fn GetProviderMgmtInterface(
        ProviderId: VSS_ID, InterfaceId: IID, ppItf: *mut *mut IUnknown
    ) -> HRESULT;
    #[link_name="ShouldBlockRevertInternal"]
    pub fn ShouldBlockRevert(wszVolumeName: LPCWSTR, pbBlock: *mut bool) -> HRESULT;
}

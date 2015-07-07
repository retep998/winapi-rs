// Copyright © 2015, Brian Vincent
// Licensed under the MIT License <LICENSE.md>
//! VSS backup interfaces
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct VSS_COMPONENTINFO {
    pub type_: ::VSS_COMPONENT_TYPE, // type is a keyword in rust
    pub bstrLogicalPath: ::BSTR,
    pub bstrComponentName: ::BSTR,
    pub bstrCaption: ::BSTR,
    pub pbIcon: *mut ::BYTE,
    pub cbIcon: ::UINT,
    pub bRestoreMetadata: bool,
    pub bNotifyOnBackupComplete: bool,
    pub bSelectable: bool,
    pub bSelectableForRestore: bool,
    pub dwComponentFlags: ::DWORD,
    pub cFileCount: ::UINT,
    pub cDatabases: ::UINT,
    pub cLogFiles: ::UINT,
    pub cDependencies: ::UINT,
}
pub type PVSSCOMPONENTINFO = *const ::VSS_COMPONENTINFO;

RIDL!(
interface IVssWMComponent(IVssWMComponentVtbl): IUnknown(IUnknownVtbl) {
    fn GetComponentInfo(&mut self, ppInfo: *mut ::PVSSCOMPONENTINFO) -> ::HRESULT,
    fn FreeComponentInfo(&mut self, pInfo: ::PVSSCOMPONENTINFO) -> ::HRESULT,
    fn GetFile(&mut self, iFile: ::UINT, ppFiledesc: *mut *mut ::IVssWMFiledesc) -> ::HRESULT,
    fn GetDatabaseFile(
        &mut self, iDBFile: ::UINT, ppFiledesc: *mut *mut ::IVssWMFiledesc
    ) -> ::HRESULT,
    fn GetDatabaseLogFile(
        &mut self, iDbLogFile: ::UINT, ppFiledesc: *mut *mut ::IVssWMFiledesc
    ) -> ::HRESULT,
    fn GetDependency(
        &mut self, iDependency: ::UINT, ppDependency: *mut *mut ::IVssWMDependency
    ) -> ::HRESULT
}
);

RIDL!(
interface IVssExamineWriterMetadata(IVssExamineWriterMetadataVtbl): IUnknown(IUnknownVtbl) {
    fn GetIdentity(
        &mut self, pidInstance: *mut ::VSS_ID, pidWriter: *mut ::VSS_ID,
        pbstrWriterName: *mut ::BSTR, pUsage: *mut ::VSS_USAGE_TYPE,
        pSource: *mut ::VSS_SOURCE_TYPE
    ) -> ::HRESULT,
    fn GetFileCounts(&mut self, pcIncludeFiles: *mut ::UINT, pcExcludeFiles: *mut ::UINT,
        pcComponents: *mut ::UINT
    ) -> ::HRESULT,
    fn GetIncludeFile(
        &mut self, iFile: ::UINT, ppFiledesc: *mut *mut ::IVssWMFiledesc
    ) -> ::HRESULT,
    fn GetExcludeFile(
        &mut self, iFile: ::UINT, ppFiledesc: *mut *mut ::IVssWMFiledesc
    ) -> ::HRESULT,
    fn GetComponent(
        &mut self, iComponent: ::UINT, ppComponent: *mut *mut ::IVssWMComponent
    ) -> ::HRESULT,
    fn GetRestoreMethod(
        &mut self, pMethod: *mut ::VSS_RESTOREMETHOD_ENUM, pbstrService: *mut ::BSTR,
        pbstrUserProcedure: *mut ::BSTR, pwriterRestore: *mut ::VSS_WRITERRESTORE_ENUM,
        pbRebootRequired: *mut bool, pcMappings: *mut ::UINT
    ) -> ::HRESULT,
    fn GetAlternateLocationMapping(
        &mut self, iMapping: ::UINT, ppFiledesc: *mut *mut ::IVssWMFiledesc
    ) -> ::HRESULT,
    fn GetBackupSchema(&mut self, pdwSchemaMask: *mut ::DWORD) -> ::HRESULT,
    fn GetDocument(&mut self, pDoc: usize) -> ::HRESULT, //TODO IXMLDOMDocument
    fn SaveAsXML(&mut self, pbstrXML: *mut ::BSTR) -> ::HRESULT,
    fn LoadFromXML(&mut self, pbstrXML: *mut ::BSTR) -> ::HRESULT
}
);
// class IVssExamineWriterMetadataEx

// class IVssExamineWriterMetadataEx2

#[repr(C)] #[allow(missing_copy_implementations)]
pub struct IVssWriterComponentsExt {
    pub lpVtbl: *mut IVssWriterComponentsExtVtbl,
}
#[repr(C)]
pub struct IVssWriterComponentsExtVtbl {
    pub parent1: ::IVssWriterComponentsVtbl,
    pub parent2: ::IUnknownVtbl,
}


RIDL!(
interface IVssBackupComponents(IVssBackupComponentsVtbl): IUnknown(IUnknownVtbl) {
    fn GetWriterComponentsCount(&mut self, pcComponents: *mut ::UINT) -> ::HRESULT,
    fn GetWriterComponents(
        &mut self, iWriter: ::UINT, ppWriter: *mut *mut IVssWriterComponentsExt
    ) -> ::HRESULT,
    fn InitializeForBackup(&mut self, bstrXML: ::BSTR) -> ::HRESULT,
    fn SetBackupState(
        &mut self, bSelectComponents: bool, bBackupBootableSystemState: bool,
        backupType: ::VSS_BACKUP_TYPE, bPartialFileSupport: bool
    ) -> ::HRESULT,
    fn InitializeForRestore(&mut self, bstrXML: ::BSTR) -> ::HRESULT,

    fn SetRestoreState(&mut self, restoreType: ::VSS_RESTORE_TYPE) -> ::HRESULT,
    fn GatherWriterMetadata(&mut self, pAsync: *mut *mut ::IVssAsync) -> ::HRESULT,
    fn GetWriterMetadataCount(&mut self, pcWriters: *mut ::UINT) -> ::HRESULT,
    fn GetWriterMetadata(
        &mut self, iWriter: ::UINT, pidInstance: *mut ::VSS_ID,
        ppMetadata: *mut *mut IVssExamineWriterMetadata
    ) -> ::HRESULT,
    fn FreeWriterMetadata(&mut self) -> ::HRESULT,
    fn AddComponent(
        &mut self, instanceId: ::VSS_ID, writerId: ::VSS_ID, ct: ::VSS_COMPONENT_TYPE,
        wszLogicalPath: ::LPCWSTR, wszComponentName: ::LPWSTR
    ) -> ::HRESULT,
    fn PrepareForBackup(&mut self, ppAsync: *mut *mut ::IVssAsync) -> ::HRESULT,
    fn AbortBackup(&mut self) -> ::HRESULT,
    fn GatherWriterStatus(&mut self, ppAsync: *mut *mut ::IVssAsync) -> ::HRESULT,
    fn GetWriterStatusCount(&mut self, pcWriters: *mut ::UINT) -> ::HRESULT,
    fn FreeWriterStatus(&mut self) -> ::HRESULT,
    fn GetWriterStatus(
        &mut self, iWriter: ::UINT, pidInstance: *mut ::VSS_ID, pidWriter: *mut ::VSS_ID,
        pbstrWriter: *mut ::BSTR, pnStatus: *mut ::VSS_WRITER_STATE,
        phResultFailure: *mut ::HRESULT
    ) -> ::HRESULT,
    fn SetBackupSucceeded(
        &mut self, instanceId: ::VSS_ID, writerId: ::VSS_ID, ct: ::VSS_COMPONENT_TYPE,
        wszLogicalPath: ::LPCWSTR, wszComponentName: ::LPWSTR, bSucceded: bool
    ) -> ::HRESULT,
    fn SetBackupOptions(
        &mut self, writerId: ::VSS_ID, ct: ::VSS_COMPONENT_TYPE, wszLogicalPath: ::LPCWSTR,
        wszComponentName: ::LPWSTR, wszBackupOptions: ::LPWSTR
    ) -> ::HRESULT,
    fn SetSelectedForRestore(
        &mut self, writerId: ::VSS_ID, ct: ::VSS_COMPONENT_TYPE, wszLogicalPath: ::LPCWSTR,
        wszComponentName: ::LPWSTR, bSelectedForRestore: bool
    ) -> ::HRESULT,
    fn SetRestoreOptions(
        &mut self, writerId: ::VSS_ID, ct: ::VSS_COMPONENT_TYPE, wszLogicalPath: ::LPCWSTR,
        wszComponentName: ::LPWSTR, wszRestoreOptions: ::LPWSTR
    ) -> ::HRESULT,
    fn SetAdditionalRestores(
        &mut self, writerId: ::VSS_ID, ct: ::VSS_COMPONENT_TYPE, wszLogicalPath: ::LPCWSTR,
        wszComponentName: ::LPWSTR, bAdditionalRestores: bool
    ) -> ::HRESULT,
    fn SetPreviousBackupStamp(
        &mut self, writerId: ::VSS_ID, ct: ::VSS_COMPONENT_TYPE, wszLogicalPath: ::LPCWSTR,
        wszComponentName: ::LPWSTR, wszPreviousBackupStamp: ::LPWSTR
    ) -> ::HRESULT,
    fn SaveAsXML(&mut self, pbstrXML: *mut *mut ::BSTR) -> ::HRESULT,
    fn BackupComplete(&mut self, ppAsync: *mut *mut ::IVssAsync) -> ::HRESULT,
    fn AddAlternativeLocationMapping(
        &mut self, writerId: ::VSS_ID, ct: ::VSS_COMPONENT_TYPE, wszLogicalPath: ::LPCWSTR,
        wszComponentName: ::LPWSTR, wszPath: ::LPCWSTR, wszFilespec: ::LPCWSTR, bRecursive: bool,
        wszDestination: ::LPWSTR
    ) -> ::HRESULT,
    fn AddRestoreSubcomponent(
        &mut self, writerId: ::VSS_ID, ct: ::VSS_COMPONENT_TYPE, wszLogicalPath: ::LPCWSTR,
        wszComponentName: ::LPWSTR, wszSubComponentLogicalPath: ::LPCWSTR,
        wszSubComponentName: ::LPCWSTR, bRepair: bool
    ) -> ::HRESULT,
    fn SetFileRestoreStatus(
        &mut self, writerId: ::VSS_ID, ct: ::VSS_COMPONENT_TYPE, wszLogicalPath: ::LPCWSTR,
        wszComponentName: ::LPWSTR, status: ::VSS_FILE_RESTORE_STATUS
    ) -> ::HRESULT,
    fn AddNewTarget(
        &mut self, writerId: ::VSS_ID, ct: ::VSS_COMPONENT_TYPE, wszLogicalPath: ::LPCWSTR,
        wszComponentName: ::LPWSTR, wszPath: ::LPCWSTR, wszFileName: ::LPCWSTR, bRecursive: bool,
        wszAlternatePath: ::LPWSTR
    ) -> ::HRESULT,
    fn SetRangesFilePath(
        &mut self, writerId: ::VSS_ID, ct: ::VSS_COMPONENT_TYPE, wszLogicalPath: ::LPCWSTR,
        wszComponentName: ::LPWSTR, iPartialFile: ::UINT, wszRangesFile: ::LPCWSTR
    ) -> ::HRESULT,
    fn PreRestore(&mut self, ppAsync: *mut *mut ::IVssAsync) -> ::HRESULT,
    fn PostRestore(&mut self, ppAsync: *mut *mut ::IVssAsync) -> ::HRESULT,
    fn SetContext(&mut self, lContext: ::LONG) -> ::HRESULT,
    fn StartSnapshotSet(&mut self, pSnapshotSetId: *mut ::VSS_ID) -> ::HRESULT,
    fn AddToSnapshotSet(
        &mut self, pwszVolumeName: ::VSS_PWSZ, ProviderId: ::VSS_ID, pidSnapshot: *mut ::VSS_ID
    ) -> ::HRESULT,
    fn DoSnapshotSet(&mut self, ppAsync: *mut *mut ::IVssAsync) -> ::HRESULT,
    fn DeleteSnapshots(
        &mut self, SourceObjectId: ::VSS_ID, eSourceObjectType: ::VSS_OBJECT_TYPE,
        bForceDelete: ::BOOL, plDeletedSnapshots: *mut ::LONG, pNondeletedSnapshotID: *mut ::VSS_ID
    ) -> ::HRESULT,
    fn ImportSnapshots(&mut self, ppAsync: *mut *mut ::IVssAsync) -> ::HRESULT,
    fn BreakSnapshotSet(&mut self, SnapshotSetId: ::VSS_ID) -> ::HRESULT,
    fn GetSnapshotProperties(
        &mut self, SnapshotId: ::VSS_ID,
        pProp: ::VSS_SNAPSHOT_PROP
    ) -> ::HRESULT,
    fn Query(&mut self, QueriedObjectId: ::VSS_ID, eQueriedObjectType: ::VSS_OBJECT_TYPE,
        eReturnedObjectsType: ::VSS_OBJECT_TYPE, ppEnum: *mut *mut ::IVssEnumObject) -> ::HRESULT,
    fn IsVolumeSupported(
        &mut self, ProviderId: ::VSS_ID, pwszVolumeName: ::VSS_PWSZ,
        pbSupportedByThisProvider: *mut ::BOOL
    ) -> ::HRESULT,
    fn DisableWriterClasses(
        &mut self, rgWriterClassId: *const ::VSS_ID, cClassId: ::UINT
    ) -> ::HRESULT,
    fn EnableWriterClasses(
        &mut self, rgWriterClassId: *const ::VSS_ID, cClassId: ::UINT
    ) -> ::HRESULT,
    fn DisableWriterInstances(
        &mut self, rgWriterInstanceId: *const ::VSS_ID, cInstanceId: ::UINT
    ) -> ::HRESULT,
    fn ExposeSnapshot(&mut self, SnapshotId: ::VSS_ID, wszPathFromRoot: ::VSS_PWSZ,
        lAttributes: ::LONG, wszExpose: ::VSS_PWSZ, pwszExposed: ::VSS_PWSZ
    ) -> ::HRESULT,
    fn RevertToSnapshot(&mut self, SnapshotId: ::VSS_ID, bForceDismount: ::BOOL) -> ::HRESULT,
    fn QueryRevertStatus(
        &mut self, pwszVolume: ::VSS_PWSZ, ppAsync: *mut *mut ::IVssAsync
    ) -> ::HRESULT
}
);

// class IVssBackupComponentsEx
// IVssBackupComponentsEx2
// IVssBackupComponentsEx3
// IVssBackupComponentsEx4

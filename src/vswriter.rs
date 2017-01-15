// Copyright © 2015, Brian Vincent
// Licensed under the MIT License <LICENSE.md>
//! VSS Writer header file
ENUM!{enum VSS_USAGE_TYPE {
    VSS_UT_UNDEFINED = 0,
    VSS_UT_BOOTABLESYSTEMSTATE = 1,
    VSS_UT_SYSTEMSERVICE = 2,
    VSS_UT_USERDATA = 3,
    VSS_UT_OTHER = 4,
}}
ENUM!{enum VSS_SOURCE_TYPE {
    VSS_ST_UNDEFINED = 0,
    VSS_ST_TRANSACTEDDB = 1,
    VSS_ST_NONTRANSACTEDDB = 2,
    VSS_ST_OTHER = 3,
}}
ENUM!{enum VSS_RESTOREMETHOD_ENUM {
    VSS_RME_UNDEFINED = 0,
    VSS_RME_RESTORE_IF_NOT_THERE = 1,
    VSS_RME_RESTORE_IF_CAN_REPLACE = 2,
    VSS_RME_STOP_RESTORE_START = 3,
    VSS_RME_RESTORE_TO_ALTERNATE_LOCATION = 4,
    VSS_RME_RESTORE_AT_REBOOT = 5,
    VSS_RME_RESTORE_AT_REBOOT_IF_CANNOT_REPLACE = 6,
    VSS_RME_CUSTOM = 7,
    VSS_RME_RESTORE_STOP_START = 8,
}}
ENUM!{enum VSS_WRITERRESTORE_ENUM {
    VSS_WRE_UNDEFINED = 0,
    VSS_WRE_NEVER = 1,
    VSS_WRE_IF_REPLACE_FAILS = 2,
    VSS_WRE_ALWAYS = 3,
}}
ENUM!{enum VSS_COMPONENT_TYPE {
    VSS_CT_UNDEFINED = 0,
    VSS_CT_DATABASE = 1,
    VSS_CT_FILEGROUP = 2,
}}
ENUM!{enum VSS_ALTERNATE_WRITER_STATE {
    VSS_AWS_UNDEFINED = 0,
    VSS_AWS_NO_ALTERNATE_WRITER = 1,
    VSS_AWS_ALTERNATE_WRITER_EXISTS = 2,
    VSS_AWS_THIS_IS_ALTERNATE_WRITER = 3,
}}
pub type VSS_SUBSCRIBE_MASK = ::DWORD;
pub const VSS_SM_POST_SNAPSHOT_FLAG: ::DWORD = 0x00000001;
pub const VSS_SM_BACKUP_EVENTS_FLAG: ::DWORD = 0x00000002;
pub const VSS_SM_RESTORE_EVENTS_FLAG: ::DWORD = 0x00000004;
pub const VSS_SM_IO_THROTTLING_FLAG: ::DWORD = 0x00000008;
pub const VSS_SM_ALL_FLAGS: ::DWORD = 0xffffffff;
ENUM!{enum VSS_RESTORE_TARGET {
    VSS_RT_UNDEFINED = 0,
    VSS_RT_ORIGINAL = 1,
    VSS_RT_ALTERNATE = 2,
    VSS_RT_DIRECTED = 3,
    VSS_RT_ORIGINAL_LOCATION = 4,
}}
ENUM!{enum VSS_FILE_RESTORE_STATUS {
    VSS_RS_UNDEFINED = 0,
    VSS_RS_NONE = 1,
    VSS_RS_ALL = 2,
    VSS_RS_FAILED = 3,
}}
pub type VSS_COMPONENT_FLAGS = ::DWORD;
pub const VSS_CF_BACKUP_RECOVERY: ::DWORD = 0x00000001;
pub const VSS_CF_APP_ROLLBACK_RECOVERY: ::DWORD = 0x00000002;
pub const VSS_CF_NOT_SYSTEM_STATE: ::DWORD = 0x00000004;
RIDL!(
interface IVssWMFiledesc(IVssWMFiledescVtbl): IUnknown(IUnknownVtbl) {
    fn GetPath(&self, pbstrPath: *mut ::BSTR) -> ::HRESULT,
    fn GetFilespec(&self, pbstrFilespec: *mut ::BSTR) -> ::HRESULT,
    fn GetRecursive(&self, pbRecursive: *mut bool) -> ::HRESULT,
    fn GetAlternateLocation(&self, pbstrAlternateLocation: *mut ::BSTR) -> ::HRESULT,
    fn GetBackupTypeMask(&self, pdwTypeMask: *mut ::DWORD) -> ::HRESULT
}
);
RIDL!(
interface IVssWMDependency(IVssWMDependencyVtbl): IUnknown(IUnknownVtbl) {
    fn GetWriterId(&self, pWriterId: *mut ::VSS_ID) -> ::HRESULT,
    fn GetLogicalPath(&self, pbstrLogicalPath: *mut ::BSTR) -> ::HRESULT,
    fn GetComponentName(&self, pbstrComponentName: *mut ::BSTR) -> ::HRESULT
}
);
RIDL!(
interface IVssComponent(IVssComponentVtbl): IUnknown(IUnknownVtbl) {
    fn GetLogicalPath(&self, pbstrPath: *mut ::BSTR) -> ::HRESULT,
    fn GetComponentType(&self, pct: *mut ::VSS_COMPONENT_TYPE) -> ::HRESULT,
    fn GetComponentName(&self, pbstrName: *mut ::BSTR) -> ::HRESULT,
    fn GetBackupSucceeded(&self, pbSucceeded: *mut bool) -> ::HRESULT,
    fn GetAlternateLocationMappingCount(&self, pcMappings: *mut ::UINT) -> ::HRESULT,
    fn GetAlternateLocationMapping(
        &self, iMapping: ::UINT, ppFiledesc: *mut *mut ::IVssWMFiledesc
    ) -> ::HRESULT,
    fn SetBackupMetadata(&self, wszData: ::LPCWSTR) -> ::HRESULT,
    fn GetBackupMetadata(&self, pbstrData: *mut ::BSTR) -> ::HRESULT,
    fn AddPartialFile(
        &self, wszPath: ::LPCWSTR, wszFilename: ::LPCWSTR, wszRanges: ::LPCWSTR,
        wszMetadata: ::LPCWSTR
    ) -> ::HRESULT,
    fn GetPartialFileCount(&self, pcPartialFiles: *mut ::UINT) -> ::HRESULT,
    fn GetPartialFile(
        &self, iPartialFile: ::UINT, pbstrPath: *mut ::BSTR, pbstrFilename: *mut ::BSTR,
        pbstrRange: *mut ::BSTR, pbstrMetadata: *mut ::BSTR
    ) -> ::HRESULT,
    fn IsSelectedForRestore(&self, pbSelectedForRestore: *mut bool) -> ::HRESULT,
    fn GetAdditionalRestores(&self, pbAdditionalRestores: *mut bool) -> ::HRESULT,
    fn GetNewTargetCount(&self, pcNewTarget: *mut ::UINT) -> ::HRESULT,
    fn GetNewTarget(
        &self, iNewTarget: ::UINT, ppFiledesc: *mut *mut ::IVssWMFiledesc
    ) -> ::HRESULT,
    fn AddDirectedTarget(
        &self, wszSourcePath: ::LPCWSTR, wszSourceFilename: ::LPCWSTR,
        wszSourceRangeList: ::LPCWSTR, wszDestinationPath: ::LPCWSTR,
        wszDestinationFilename: ::LPCWSTR, wszDestinationRangeList: ::LPCWSTR
    ) -> ::HRESULT,
    fn GetDirectedTargetCount(&self, pcDirectedTarget: *mut ::UINT) -> ::HRESULT,
    fn GetDirectedTarget(
        &self, iDirectedTarget: ::UINT, pbstrSourcePath: *mut ::BSTR,
        pbstrSourceFileName: *mut ::BSTR, pbstrSourceRangeList: *mut ::BSTR,
        pbstrDestinationPath: *mut ::BSTR, pbstrDestinationFilename: *mut ::BSTR,
        pbstrDestinationRangeList: *mut ::BSTR
    ) -> ::HRESULT,
    fn SetRestoreMetadata(&self, wszRestoreMetadata: ::LPCWSTR) -> ::HRESULT,
    fn GetRestoreMetadata(&self, pbstrRestoreMetadata: *mut ::BSTR) -> ::HRESULT,
    fn SetRestoreTarget(&self, target: ::VSS_RESTORE_TARGET) -> ::HRESULT,
    fn GetRestoreTarget(&self, pTarget: *mut ::VSS_RESTORE_TARGET) -> ::HRESULT,
    fn SetPreRestoreFailureMsg(&self, wszPreRestoreFailureMsg: ::LPCWSTR) -> ::HRESULT,
    fn GetPreRestoreFailureMsg(&self, pbstrPreRestoreFailureMsg: *mut ::BSTR) -> ::HRESULT,
    fn SetPostRestoreFailureMsg(&self, wszPostRestoreFailureMsg: ::LPCWSTR) -> ::HRESULT,
    fn GetPostRestoreFailureMsg(&self, pbstrPostRestoreFailureMsg: *mut ::BSTR) -> ::HRESULT,
    fn SetBackupStamp(&self, wszBackupStamp: ::LPCWSTR) -> ::HRESULT,
    fn GetBackupStamp(&self, pbstrBackupStamp: *mut ::BSTR) -> ::HRESULT,
    fn GetPreviousBackupStamp(&self, pbstrBackupStamp: *mut ::BSTR) -> ::HRESULT,
    fn GetBackupOptions(&self, pbstrBackupOptions: *mut ::BSTR) -> ::HRESULT,
    fn GetRestoreOptions(&self, pbstrRestoreOptions: *mut ::BSTR) -> ::HRESULT,
    fn GetRestoreSubcomponentCount(&self, pcRestoreSubcomponent: *mut ::UINT) -> ::HRESULT,
    fn GetRestoreSubcomponent(
        &self, iComponent: ::UINT, pbstrLogicalPath: *mut ::BSTR,
        pbstrComponentName: *mut ::BSTR, pbRepair: *mut bool
    ) -> ::HRESULT,
    fn GetFileRestoreStatus(&self, pStatus: *mut VSS_FILE_RESTORE_STATUS) -> ::HRESULT,
    fn AddDifferencedFilesByLastModifyTime(
        &self, wszPath: ::LPCWSTR, wszFilespec: ::LPCWSTR, bRecursive: ::BOOL,
        ftLastModifyTime: ::FILETIME
    ) -> ::HRESULT,
    fn AddDifferencedFilesByLastModifyLSN(
        &self, wszPath: ::LPCWSTR, wszFilespec: ::LPCWSTR, bRecursive: ::BOOL,
        bstrLsnString: ::BSTR
    ) -> ::HRESULT,
    fn GetDifferencedFilesCount(&self, pcDifferencedFiles: *mut ::UINT) -> ::HRESULT,
    fn GetDifferencedFile(
        &self, iDifferencedFile: ::UINT, pbstrPath: *mut ::BSTR, pbstrFilespec: *mut ::BSTR,
        pbRecursive: *mut ::BOOL, pbstrLsnString: *mut ::BSTR, pftLastModifyTime: *mut ::FILETIME
    ) -> ::HRESULT
}
);
RIDL!(
interface IVssWriterComponents(IVssWriterComponentsVtbl) {
    fn GetComponentCount(&self, pcComponents: *mut ::UINT) -> ::HRESULT,
    fn GetWriterInfo(
        &self, pidInstance: *mut ::VSS_ID, pidWriter: *mut ::VSS_ID
    ) -> ::HRESULT,
    fn GetComponent(
        &self, iComponent: ::UINT, ppComponent: *mut *mut ::IVssComponent
    ) -> ::HRESULT
}
);
RIDL!(
interface IVssComponentEx(IVssComponentExVtbl): IVssComponent(IVssComponentVtbl) {
    fn SetPrepareForBackupFailureMsg(&self, wszFailureMsg: ::LPCWSTR) -> ::HRESULT,
    fn SetPostSnapshotFailureMsg(&self, wszFailureMsg: ::LPCWSTR) -> ::HRESULT,
    fn GetPrepareForBackupFailureMsg(&self, pbstrFailureMsg: *mut ::BSTR) -> ::HRESULT,
    fn GetPostSnapshotFailureMsg(&self, pbstrFailureMsg: *mut ::BSTR) -> ::HRESULT,
    fn GetAuthoritativeRestore(&self, pbAuth: *mut bool) -> ::HRESULT,
    fn GetRollForward(
        &self, pRollType: *mut ::VSS_ROLLFORWARD_TYPE, pbstrPoint: *mut ::BSTR
    ) -> ::HRESULT,
    fn GetRestoreName(&self, pbstrName: *mut ::BSTR) -> ::HRESULT
}
);
RIDL!(
interface IVssComponentEx2(IVssComponentEx2Vtbl): IVssComponentEx(IVssComponentExVtbl) {
    fn SetFailure(
        &self, hr: ::HRESULT, hrApplication: ::HRESULT, wszApplicationMessage: ::LPCWSTR,
        dwReserved: ::DWORD
    ) -> ::HRESULT,
    fn GetFailure(
        &self, phr: *mut ::HRESULT, phrApplication: *mut ::HRESULT,
        pbstrApplicationMessage: *mut ::BSTR, pdwReserved: *mut ::DWORD
    ) -> ::HRESULT
}
);
RIDL!(
interface IVssCreateWriterMetadata(IVssCreateWriterMetadataVtbl) {
    fn AddIncludeFiles(
        &self, wszPath: ::LPCWSTR, wszFilespec: ::LPCWSTR, bRecursive: bool,
        wszAlternateLocation: ::LPCWSTR
    ) -> ::HRESULT,
    fn AddExcludeFiles(
        &self, wszPath: ::LPCWSTR, wszFilespec: ::LPCWSTR, bRecursive: bool
    ) -> ::HRESULT,
    fn AddComponent(
        &self, ct: ::VSS_COMPONENT_TYPE, wszLogicalPath: ::LPCWSTR,
        wszComponentName: ::LPCWSTR, wszCaption: ::LPCWSTR, pbIcon: *const ::BYTE, cbIcon: ::UINT,
        bRestoreMetadata: bool, bNotifyOnBackupComplete: bool, bSelectableForRestore: bool,
        dwComponentFlags: ::DWORD
    ) -> ::HRESULT,
    fn AddDatabaseFiles(
        &self, wszLogicalPath: ::LPCWSTR, wszDatabaseName: ::LPCWSTR, wszPath: ::LPCWSTR,
        wszFilespec: ::LPCWSTR, dwBackupTypeMask: ::DWORD
    ) -> ::HRESULT,
    fn AddDatabaseLogFiles(&self, wszLogicalPath: ::LPCWSTR, wszDatabaseName: ::LPCWSTR,
        wszPath: ::LPCWSTR, wszFilespec: ::LPCWSTR, dwBackupTypeMask: ::DWORD
    ) -> ::HRESULT,
    fn AddFilesToFileGroup(&self, wszLogicalPath: ::LPCWSTR, wszGroupName: ::LPCWSTR,
        wszPath: ::LPCWSTR, wszFilespec: ::LPCWSTR, bRecursive: bool,
        wszAlternateLocation: ::LPCWSTR, dwBackupTypeMask: ::DWORD
    ) -> ::HRESULT,
    fn SetRestoreMethod(&self, method: ::VSS_RESTOREMETHOD_ENUM, wszService: ::LPCWSTR,
        wszUserProcedure: ::LPCWSTR, writerRestore: ::VSS_WRITERRESTORE_ENUM,
        bRebootRequired: bool
    ) -> ::HRESULT,
    fn AddAlternateLocationMapping(&self, wszSourcePath: ::LPCWSTR,
        wszSourceFilespec: ::LPCWSTR, bRecursive: bool, wszDestination: ::LPCWSTR
    ) -> ::HRESULT,
    fn AddComponentDependency(&self, wszForLogicalPath: ::LPCWSTR,
        wszForComponentName: ::LPCWSTR, onWriterId: ::VSS_ID, wszOnLogicalPath: ::LPCWSTR,
        wszOnComponentName: ::LPCWSTR
    ) -> ::HRESULT,
    fn SetBackupSchema(&self, dwSchemaMask: ::DWORD) -> ::HRESULT,
    fn GetDocument(&self, pDoc: *mut *mut ::VOID) -> ::HRESULT, //TODO IXMLDOMDocument
    fn SaveAsXML(&self, pbstrXML: *mut ::BSTR) -> ::HRESULT
}
);
//IVssCreateWriterMetadataEx
//IVssWriterImpl
//IVssCreateExpressWriterMetadata
//IVssExpressWriter
//CVssWriter
//CVssWriterEx
//CVssWriterEx2

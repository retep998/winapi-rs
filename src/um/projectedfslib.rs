// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use ctypes::{c_int, c_void};
use shared::basetsd::{INT32, INT64, UINT32, UINT64, UINT8};
use shared::guiddef::GUID;
use shared::minwindef::ULONG;
use um::winnt::{BOOLEAN, HRESULT, LARGE_INTEGER, LPCWSTR, PCWSTR, VOID};
use vc::vcruntime::size_t;
ENUM!{enum PRJ_NOTIFY_TYPES {
    PRJ_NOTIFY_NONE = 0x00000000,
    PRJ_NOTIFY_SUPPRESS_NOTIFICATIONS = 0x00000001,
    PRJ_NOTIFY_FILE_OPENED = 0x00000002,
    PRJ_NOTIFY_NEW_FILE_CREATED = 0x00000004,
    PRJ_NOTIFY_FILE_OVERWRITTEN = 0x00000008,
    PRJ_NOTIFY_PRE_DELETE = 0x00000010,
    PRJ_NOTIFY_PRE_RENAME = 0x00000020,
    PRJ_NOTIFY_PRE_SET_HARDLINK = 0x00000040,
    PRJ_NOTIFY_FILE_RENAMED = 0x00000080,
    PRJ_NOTIFY_HARDLINK_CREATED = 0x00000100,
    PRJ_NOTIFY_FILE_HANDLE_CLOSED_NO_MODIFICATION = 0x00000200,
    PRJ_NOTIFY_FILE_HANDLE_CLOSED_FILE_MODIFIED = 0x00000400,
    PRJ_NOTIFY_FILE_HANDLE_CLOSED_FILE_DELETED = 0x00000800,
    PRJ_NOTIFY_FILE_PRE_CONVERT_TO_FULL = 0x00001000,
    PRJ_NOTIFY_USE_EXISTING_MASK = 0xFFFFFFFF,
}}
ENUM!{enum PRJ_NOTIFICATION {
    PRJ_NOTIFICATION_FILE_OPENED = 0x00000002,
    PRJ_NOTIFICATION_NEW_FILE_CREATED = 0x00000004,
    PRJ_NOTIFICATION_FILE_OVERWRITTEN = 0x00000008,
    PRJ_NOTIFICATION_PRE_DELETE = 0x00000010,
    PRJ_NOTIFICATION_PRE_RENAME = 0x00000020,
    PRJ_NOTIFICATION_PRE_SET_HARDLINK = 0x00000040,
    PRJ_NOTIFICATION_FILE_RENAMED = 0x00000080,
    PRJ_NOTIFICATION_HARDLINK_CREATED = 0x00000100,
    PRJ_NOTIFICATION_FILE_HANDLE_CLOSED_NO_MODIFICATION = 0x00000200,
    PRJ_NOTIFICATION_FILE_HANDLE_CLOSED_FILE_MODIFIED = 0x00000400,
    PRJ_NOTIFICATION_FILE_HANDLE_CLOSED_FILE_DELETED = 0x00000800,
    PRJ_NOTIFICATION_FILE_PRE_CONVERT_TO_FULL = 0x00001000,
}}
DECLARE_HANDLE!(
    PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT,
    PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT__
);
DECLARE_HANDLE!(PRJ_DIR_ENTRY_BUFFER_HANDLE, PRJ_DIR_ENTRY_BUFFER_HANDLE__);
ENUM!{enum PRJ_EXT_INFO_TYPE {
    PRJ_EXT_INFO_TYPE_SYMLINK = 1,
}}
STRUCT!{struct PRJ_EXTENDED_INFO_u_Symlink {
    TargetName: PCWSTR,
}}
UNION!{union PRJ_EXTENDED_INFO_u {
    [u32; 1],
    Symlink Symlink_mut: PRJ_EXTENDED_INFO_u_Symlink,
}}
STRUCT!{struct PRJ_EXTENDED_INFO {
    InfoType: PRJ_EXT_INFO_TYPE,
    NextInfoOffset: ULONG,
    Symlink: PRJ_EXTENDED_INFO_u,
}}
STRUCT!{struct PRJ_NOTIFICATION_MAPPING {
    NotificationBitMask: PRJ_NOTIFY_TYPES,
    NotificationRoot: PCWSTR,
}}
ENUM!{enum PRJ_STARTVIRTUALIZING_FLAGS {
    PRJ_FLAG_NONE = 0x00000000,
    PRJ_FLAG_USE_NEGATIVE_PATH_CACHE = 0x00000001,
}}
STRUCT!{struct PRJ_STARTVIRTUALIZING_OPTIONS {
    Flags: PRJ_STARTVIRTUALIZING_FLAGS,
    PoolThreadCount: UINT32,
    ConcurrentThreadCount: UINT32,
    NotificationMappings: *mut PRJ_NOTIFICATION_MAPPING,
    NotificationMappingsCount: UINT32,
}}
STRUCT!{struct PRJ_VIRTUALIZATION_INSTANCE_INFO {
    InstanceID: GUID,
    WriteAlignment: UINT32,
}}
ENUM!{enum PRJ_PLACEHOLDER_ID {
    PRJ_PLACEHOLDER_ID_LENGTH = 128,
}}
STRUCT!{struct PRJ_PLACEHOLDER_VERSION_INFO {
    ProviderID: [UINT8; PRJ_PLACEHOLDER_ID_LENGTH as usize],
    ContentID: [UINT8; PRJ_PLACEHOLDER_ID_LENGTH as usize],
}}
STRUCT!{struct PRJ_FILE_BASIC_INFO {
    IsDirectory: BOOLEAN,
    FileSize: INT64,
    CreationTime: LARGE_INTEGER,
    LastAccessTime: LARGE_INTEGER,
    LastWriteTime: LARGE_INTEGER,
    ChangeTime: LARGE_INTEGER,
    FileAttributes: UINT32,
}}
STRUCT!{struct PRJ_PLACEHOLDER_INFO_EaInfomation {
    EaBufferSize: UINT32,
    OffsetToFirstEa: UINT32,
}}
STRUCT!{struct PRJ_PLACEHOLDER_INFO_SecurityInfomation {
    SecurityBufferSize: UINT32,
    OffsetToSecurityDescriptor: UINT32,
}}
STRUCT!{struct PRJ_PLACEHOLDER_INFO_StreamsInformation {
    StreamsInfoBufferSize: UINT32,
    OffsetToFirstStreamInfo: UINT32,
}}
STRUCT!{struct PRJ_PLACEHOLDER_INFO {
    FileBasicInfo: PRJ_FILE_BASIC_INFO,
    EaInformation: PRJ_PLACEHOLDER_INFO_EaInfomation,
    SecurityInfomation: PRJ_PLACEHOLDER_INFO_SecurityInfomation,
    StreamsInformation: PRJ_PLACEHOLDER_INFO_StreamsInformation,
    VersionInfo: PRJ_PLACEHOLDER_VERSION_INFO,
    VariableData: [UINT8; 1],
}}
ENUM!{enum PRJ_UPDATE_TYPES {
    PRJ_UPDATE_NONE = 0x00000000,
    PRJ_UPDATE_ALLOW_DIRTY_METADATA = 0x00000001,
    PRJ_UPDATE_ALLOW_DIRTY_DATA = 0x00000002,
    PRJ_UPDATE_ALLOW_TOMBSTONE = 0x00000004,
    PRJ_UPDATE_RESERVED1 = 0x00000008,
    PRJ_UPDATE_RESERVED2 = 0x00000010,
    PRJ_UPDATE_ALLOW_READ_ONLY = 0x00000020,
    PRJ_UPDATE_MAX_VAL = PRJ_UPDATE_ALLOW_READ_ONLY << 1,
}}
ENUM!{enum PRJ_UPDATE_FAILURE_CAUSES {
    PRJ_UPDATE_FAILURE_CAUSE_NONE = 0x00000000,
    PRJ_UPDATE_FAILURE_CAUSE_DIRTY_METADATA = 0x00000001,
    PRJ_UPDATE_FAILURE_CAUSE_DIRTY_DATA = 0x00000002,
    PRJ_UPDATE_FAILURE_CAUSE_TOMBSTONE = 0x00000004,
    PRJ_UPDATE_FAILURE_CAUSE_READ_ONLY = 0x00000008,
}}
ENUM!{enum PRJ_FILE_STATE {
    PRJ_FILE_STATE_PLACEHOLDER = 0x00000001,
    PRJ_FILE_STATE_HYDRATED_PLACEHOLDER = 0x00000002,
    PRJ_FILE_STATE_DIRTY_PLACEHOLDER = 0x00000004,
    PRJ_FILE_STATE_FULL = 0x00000008,
    PRJ_FILE_STATE_TOMBSTONE = 0x00000010,
}}
ENUM!{enum PRJ_CALLBACK_DATA_FLAGS {
    PRJ_CB_DATA_FLAG_ENUM_RESTART_SCAN = 0x00000001,
    PRJ_CB_DATA_FLAG_ENUM_RETURN_SINGLE_ENTRY = 0x00000002,
}}
STRUCT!{struct PRJ_CALLBACK_DATA {
    Size: UINT32,
    Flags: PRJ_CALLBACK_DATA_FLAGS,
    NamespaceVirtualizationContext: PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT,
    CommandId: INT32,
    FileId: GUID,
    DataStreamId: GUID,
    FilePathName: PCWSTR,
    VersionInfo: *mut PRJ_PLACEHOLDER_VERSION_INFO,
    TriggeringProcessId: UINT32,
    TriggeringProcessImageFileName: PCWSTR,
    InstanceContext: *mut c_void,
}}
FN!{stdcall PRJ_START_DIRECTORY_ENUMERATION_CB(
    callbackData: *const PRJ_CALLBACK_DATA,
    enumerationId: *const GUID,
) -> HRESULT}
FN!{stdcall PRJ_GET_DIRECTORY_ENUMERATION_CB(
    callbackData: *const PRJ_CALLBACK_DATA,
    enumerationId: *const GUID,
    searchExpression: PCWSTR,
    dirEntryBufferHandle: PRJ_DIR_ENTRY_BUFFER_HANDLE,
) -> HRESULT}
FN!{stdcall PRJ_END_DIRECTORY_ENUMERATION_CB(
    callbackData: *const PRJ_CALLBACK_DATA,
    enumerationId: *const GUID,
) -> HRESULT}
FN!{stdcall PRJ_GET_PLACEHOLDER_INFO_CB(
    callbackData: *const PRJ_CALLBACK_DATA,
) -> HRESULT}
FN!{stdcall PRJ_GET_FILE_DATA_CB(
    callbackData: *const PRJ_CALLBACK_DATA,
    byteOffset: UINT64,
    length: UINT32,
) -> HRESULT}
FN!{stdcall PRJ_QUERY_FILE_NAME_CB(
    callbackData: *const PRJ_CALLBACK_DATA,
) -> HRESULT}
STRUCT!{struct PRJ_NOTIFICATION_PARAMETERS_PostCreate {
    NotificationMask: PRJ_NOTIFY_TYPES,
}}
STRUCT!{struct PRJ_NOTIFICATION_PARAMETERS_FileRenamed {
    NotificationMask: PRJ_NOTIFY_TYPES,
}}
STRUCT!{struct PRJ_NOTIFICATION_PARAMETERS_FileDeletedOnHandleClose {
    IsFileModified: BOOLEAN,
}}
UNION!{union PRJ_NOTIFICATION_PARAMETERS {
    [u32; 1],
    PostCreate PostCreate_mut: PRJ_NOTIFICATION_PARAMETERS_PostCreate,
    FileRenamed FileRenamed_mut: PRJ_NOTIFICATION_PARAMETERS_FileRenamed,
    FileDeletedOnHandleClose FileDeletedOnHandleClosemut: 
        PRJ_NOTIFICATION_PARAMETERS_FileDeletedOnHandleClose,
}}
FN!{stdcall PRJ_NOTIFICATION_CB(
    callbackData: *const PRJ_CALLBACK_DATA,
    isDirectory: BOOLEAN,
    notification: PRJ_NOTIFICATION,
    destinationFileName: PCWSTR,
    operationParameters: *mut PRJ_NOTIFICATION_PARAMETERS,
) -> HRESULT}
FN!{stdcall PRJ_CANCEL_COMMAND_CB(
    callbackData: *const PRJ_CALLBACK_DATA,
) -> VOID}
STRUCT!{struct PRJ_CALLBACKS {
    StartDirectoryEnumerationCallback: *mut PRJ_START_DIRECTORY_ENUMERATION_CB,
    EndDirectoryEnumerationCallback: *mut PRJ_END_DIRECTORY_ENUMERATION_CB,
    GetDirectoryEnumerationCallback: *mut PRJ_GET_DIRECTORY_ENUMERATION_CB,
    GetPlaceholderInfoCallback: *mut PRJ_GET_PLACEHOLDER_INFO_CB,
    GetFileDataCallback: *mut PRJ_GET_FILE_DATA_CB,
    QueryFileNameCallback: *mut PRJ_QUERY_FILE_NAME_CB,
    NotificationCallback: *mut PRJ_NOTIFICATION_CB,
    CancelCommandCallback: *mut PRJ_CANCEL_COMMAND_CB,
}}
ENUM!{enum PRJ_COMPLETE_COMMAND_TYPE {
    PRJ_COMPLETE_COMMAND_TYPE_NOTIFICATION = 1,
    PRJ_COMPLETE_COMMAND_TYPE_ENUMERATION = 2,
}}
STRUCT!{struct PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_u_Notification {
    NotificationMask: PRJ_NOTIFY_TYPES,
}}
STRUCT!{struct PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_u_Enumeration {
    DirEntryBufferHandle: PRJ_DIR_ENTRY_BUFFER_HANDLE,
}}
UNION!{union PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_u {
    [u32; 1],
    Notification Notification_mut: PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_u_Notification,
    Enumeration Enumeration_mut: PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_u_Enumeration,
}}
STRUCT!{struct PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS {
    CommandType: PRJ_COMPLETE_COMMAND_TYPE,
    Parameters: PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_u,
}}
extern "system" {
    pub fn PrjStartVirtualizing(
        virtualizationRootPath: PCWSTR,
        callbacks: *const PRJ_CALLBACKS,
        instanceContext: *const c_void,
        options: *const PRJ_STARTVIRTUALIZING_OPTIONS,
        namespaceVirtualizationContext: *mut PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT,
    ) -> HRESULT;
    pub fn PrjStopVirtualizing(
        namespaceVirtualizationContext: PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT,
    );
    pub fn PrjClearNegativePathCache(
        namespaceVirtualizationContext: PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT,
        totalEntryNumber: *mut UINT32,
    ) -> HRESULT;
    pub fn PrjGetVirtualizationInstanceInfo(
        namespaceVirtualizationContext: PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT,
        virtualizationInstanceInfo: *mut PRJ_VIRTUALIZATION_INSTANCE_INFO,
    ) -> HRESULT;
    pub fn PrjMarkDirectoryAsPlaceholder(
        rootPathName: PCWSTR,
        targetPathName: PCWSTR,
        versionInfo: *const PRJ_PLACEHOLDER_VERSION_INFO,
        virtualizationInstanceInfo: *const GUID,
    ) -> HRESULT;
    pub fn PrjWritePlaceholderInfo(
        namespaceVirtualizationContext: PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT,
        destinationFileName: PCWSTR,
        placeholderInfo: *const PRJ_PLACEHOLDER_INFO,
        placeholderInfoSize: UINT32,
    ) -> HRESULT;
    pub fn PrjWritePlaceholderInfo2(
        namespaceVirtualizationContext: PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT,
        destinationFileName: PCWSTR,
        placeholderInfo: *const PRJ_PLACEHOLDER_INFO,
        placeholderInfoSize: UINT32,
        ExtendedInfo: *const PRJ_EXTENDED_INFO,
    ) -> HRESULT;
    pub fn PrjUpdateFileIfNeeded(
        namespaceVirtualizationContext: PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT,
        destinationFileName: PCWSTR,
        placeholderInfo: *const PRJ_PLACEHOLDER_INFO,
        placeholderInfoSize: UINT32,
        updateFlags: PRJ_UPDATE_TYPES,
        failureReason: *mut PRJ_UPDATE_FAILURE_CAUSES,
    ) -> HRESULT;
    pub fn PrjDeleteFile(
        namespaceVirtualizationContext: PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT,
        destinationFileName: PCWSTR,
        updateFlags: PRJ_UPDATE_TYPES,
        failureReason: *mut PRJ_UPDATE_FAILURE_CAUSES,
    ) -> HRESULT;
    pub fn PrjWriteFileData(
        namespaceVirtualizationContext: PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT,
        dataStreamId: *const GUID,
        buffer: *mut c_void,
        byteOffset: UINT64,
        length: UINT32,
    ) -> HRESULT;
    pub fn PrjGetOnDiskFileState(destinationFileName: PCWSTR, fileState: *mut PRJ_FILE_STATE);
    pub fn PrjAllocateAlignedBuffer(
        namespaceVirtualizationContext: PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT,
        size: size_t,
    ) -> *mut c_void;
    pub fn PrjFreeAlignedBuffer(buffer: *mut c_void);
    pub fn PrjCompleteCommand(
        namespaceVirtualizationContext: PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT,
        commandId: INT32,
        completionResult: HRESULT,
        extendedParameters: *mut PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS,
    ) -> HRESULT;
    pub fn PrjFillDirEntryBuffer(
        fileName: PCWSTR,
        fileBasicInfo: *mut PRJ_FILE_BASIC_INFO,
        dirEntryBufferHandle: PRJ_DIR_ENTRY_BUFFER_HANDLE,
    ) -> HRESULT;
    pub fn PrjFillDirEntryBuffer2(
        dirEntryBufferHandle: PRJ_DIR_ENTRY_BUFFER_HANDLE,
        fileName: PCWSTR,
        fileBasicInfo: *mut PRJ_FILE_BASIC_INFO,
        extendedInfo: *mut PRJ_EXTENDED_INFO,
    ) -> HRESULT;
    pub fn PrjFileNameMatch(fileNameToCheck: PCWSTR, pattern: PCWSTR) -> BOOLEAN;
    pub fn PrjFileNameCompare(fileName1: PCWSTR, fileName2: PCWSTR) -> c_int;
    pub fn PrjDoesNameContainWildCards(fileName: LPCWSTR) -> BOOLEAN;
}

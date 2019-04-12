use crate::{
    km::MISS_TYPE_PTR,
    km::wdm::{IO_STATUS_BLOCK,PETHREAD, KPROCESSOR_MODE,PDEVICE_OBJECT, PDRIVER_OBJECT, PIO_STATUS_BLOCK, PIRP, PKEVENT,POOL_TYPE},
    shared::{basetsd::*, ntdef::*},
    *,
};

pub type PFLT_FILTER = PVOID;
pub type PFLT_VOLUME = PVOID;
pub type PKTRANSACTION = PVOID;

pub type FLT_OPERATION_REGISTRATION_FLAGS = ULONG;
pub type FLT_POST_OPERATION_FLAGS = ULONG;
pub type FLT_REGISTRATION_FLAGS = ULONG;
pub type PFLT_CONTEXT = PVOID;
pub type PFLT_INSTANCE = PVOID;

pub const FILE_REMOVABLE_MEDIA: u32 = 1;
pub const FILE_READ_ONLY_DEVICE: u32 = 2;
pub const FILE_FLOPPY_DISKETTE: u32 = 4;
pub const FILE_WRITE_ONCE_MEDIA: u32 = 8;
pub const FILE_REMOTE_DEVICE: u32 = 16;

#[link(name = "fltMgr")]
extern "system" {
    pub fn FltUnregisterFilter(Filter: PFLT_FILTER);
    pub fn FltStartFiltering(Filter: PFLT_FILTER) -> NTSTATUS;
    pub fn FltRegisterFilter(
        Driver: PDRIVER_OBJECT,
        Registration: *const FLT_REGISTRATION,
        RetFilter: *mut PFLT_FILTER,
    ) -> NTSTATUS;
    pub fn FltGetVolumeProperties(
        Volume: PFLT_VOLUME,
        VolumeProperties: PFLT_VOLUME_PROPERTIES,
        VolumePropertiesLength: ULONG,
        LengthReturned: PULONG,
    ) -> NTSTATUS;

    pub fn FltGetDiskDeviceObject(
        Volume: PFLT_VOLUME,
        DiskDeviceObject: *mut PDEVICE_OBJECT,
    ) -> NTSTATUS;

    pub fn IoVolumeDeviceToDosName(VolumeDeviceObject: PVOID, DosName: PUNICODE_STRING)
        -> NTSTATUS;
    pub fn IoBuildDeviceIoControlRequest(
        IoControlCode: ULONG,
        DeviceObject: PDEVICE_OBJECT,
        InputBuffer: PVOID,
        InputBufferLength: ULONG,
        OutputBuffer: PVOID,
        OutputBufferLength: ULONG,
        InternalDeviceIoControl: BOOLEAN,
        Event: PKEVENT,
        IoStatusBlock: PIO_STATUS_BLOCK,
    ) -> PIRP;
    pub fn ExFreePoolWithTag(P: PVOID, Tag: ULONG);
}

ENUM! {enum FLT_PREOP_CALLBACK_STATUS {
    FLT_PREOP_SUCCESS_WITH_CALLBACK = 0,
    FLT_PREOP_SUCCESS_NO_CALLBACK = 1,
    FLT_PREOP_PENDING = 2,
    FLT_PREOP_DISALLOW_FASTIO = 3,
    FLT_PREOP_COMPLETE = 4,
    FLT_PREOP_SYNCHRONIZE = 5,
    FLT_PREOP_DISALLOW_FSFILTER_IO = 6,
}}

ENUM! {enum FLT_POSTOP_CALLBACK_STATUS {
    FLT_POSTOP_FINISHED_PROCESSING,
    FLT_POSTOP_MORE_PROCESSING_REQUIRED,
    FLT_POSTOP_DISALLOW_FSFILTER_IO,
}}

ENUM! {enum FLT_FILESYSTEM_TYPE {
    FLT_FSTYPE_UNKNOWN = 0,
    FLT_FSTYPE_RAW = 1,
    FLT_FSTYPE_NTFS = 2,
    FLT_FSTYPE_FAT = 3,
    FLT_FSTYPE_CDFS = 4,
    FLT_FSTYPE_UDFS = 5,
    FLT_FSTYPE_LANMAN = 6,
    FLT_FSTYPE_WEBDAV = 7,
    FLT_FSTYPE_RDPDR = 8,
    FLT_FSTYPE_NFS = 9,
    FLT_FSTYPE_MS_NETWARE = 10,
    FLT_FSTYPE_NETWARE = 11,
    FLT_FSTYPE_BSUDF = 12,
    FLT_FSTYPE_MUP = 13,
    FLT_FSTYPE_RSFX = 14,
    FLT_FSTYPE_ROXIO_UDF1 = 15,
    FLT_FSTYPE_ROXIO_UDF2 = 16,
    FLT_FSTYPE_ROXIO_UDF3 = 17,
    FLT_FSTYPE_TACIT = 18,
    FLT_FSTYPE_FS_REC = 19,
    FLT_FSTYPE_INCD = 20,
    FLT_FSTYPE_INCD_FAT = 21,
    FLT_FSTYPE_EXFAT = 22,
    FLT_FSTYPE_PSFS = 23,
    FLT_FSTYPE_GPFS = 24,
    FLT_FSTYPE_NPFS = 25,
    FLT_FSTYPE_MSFS = 26,
    FLT_FSTYPE_CSVFS = 27,
    FLT_FSTYPE_REFS = 28,
    FLT_FSTYPE_OPENAFS = 29,
}}

STRUCT! {
    struct FLT_OPERATION_REGISTRATION {
        MajorFunction: UCHAR,
        Flags: FLT_OPERATION_REGISTRATION_FLAGS,
        PreOperation: PFLT_PRE_OPERATION_CALLBACK,
        PostOperation: PFLT_POST_OPERATION_CALLBACK,
        Reserved1: PVOID,
    }
}
pub type PFLT_PRE_OPERATION_CALLBACK = ::core::option::Option<
    unsafe extern "system" fn(
        Data: PFLT_CALLBACK_DATA,
        FltObjects: PCFLT_RELATED_OBJECTS,
        CompletionContext: *mut PVOID,
    ) -> FLT_PREOP_CALLBACK_STATUS,
>;

pub type PFLT_POST_OPERATION_CALLBACK = ::core::option::Option<
    unsafe extern "system" fn(
        Data: MISS_TYPE_PTR,
        FltObjects: PCFLT_RELATED_OBJECTS,
        CompletionContext: PVOID,
        Flags: FLT_POST_OPERATION_FLAGS,
    ) -> FLT_POSTOP_CALLBACK_STATUS,
>;

#[repr(C)]
pub struct FLT_REGISTRATION {
    pub Size: USHORT,
    pub Version: USHORT,
    pub Flags: FLT_REGISTRATION_FLAGS,
    pub ContextRegistration: *const FLT_CONTEXT_REGISTRATION,
    pub OperationRegistration: *const FLT_OPERATION_REGISTRATION,
    pub FilterUnloadCallback: PFLT_FILTER_UNLOAD_CALLBACK,
    pub InstanceSetupCallback: PFLT_INSTANCE_SETUP_CALLBACK,
    pub InstanceQueryTeardownCallback: PFLT_INSTANCE_QUERY_TEARDOWN_CALLBACK,
    pub InstanceTeardownStartCallback: PFLT_INSTANCE_TEARDOWN_CALLBACK,
    pub InstanceTeardownCompleteCallback: PFLT_INSTANCE_TEARDOWN_CALLBACK,
    pub GenerateFileNameCallback: MISS_TYPE_PTR,
    pub NormalizeNameComponentCallback: MISS_TYPE_PTR,
    pub NormalizeContextCleanupCallback: PFLT_NORMALIZE_CONTEXT_CLEANUP,
    pub TransactionNotificationCallback: MISS_TYPE_PTR,
    pub NormalizeNameComponentExCallback: MISS_TYPE_PTR,
    pub SectionNotificationCallback: MISS_TYPE_PTR,
}

#[repr(C)]
pub struct FLT_CONTEXT_REGISTRATION {
    pub ContextType: FLT_CONTEXT_TYPE,
    pub Flags: FLT_CONTEXT_REGISTRATION_FLAGS,
    pub ContextCleanupCallback: PFLT_CONTEXT_CLEANUP_CALLBACK,
    pub Size: SIZE_T,
    pub PoolTag: ULONG,
    pub ContextAllocateCallback: PFLT_CONTEXT_ALLOCATE_CALLBACK,
    pub ContextFreeCallback: PFLT_CONTEXT_FREE_CALLBACK,
    pub Reserved1: PVOID,
}

pub type FLT_CONTEXT_TYPE = USHORT;
pub type FLT_CONTEXT_REGISTRATION_FLAGS = USHORT;
pub type PFLT_CONTEXT_CLEANUP_CALLBACK = ::core::option::Option<
    unsafe extern "system" fn(Context: PFLT_CONTEXT, ContextType: FLT_CONTEXT_TYPE),
>;
pub type PFLT_CONTEXT_ALLOCATE_CALLBACK = ::core::option::Option<
    unsafe extern "system" fn(
        PoolType: POOL_TYPE,
        Size: SIZE_T,
        ContextType: FLT_CONTEXT_TYPE,
    ) -> PVOID,
>;
pub type PFLT_CONTEXT_FREE_CALLBACK =
    ::core::option::Option<unsafe extern "system" fn(Pool: PVOID, ContextType: FLT_CONTEXT_TYPE)>;
pub type FLT_FILTER_UNLOAD_FLAGS = ULONG;

pub type PFLT_FILTER_UNLOAD_CALLBACK =
    ::core::option::Option<unsafe extern "system" fn(Flags: FLT_FILTER_UNLOAD_FLAGS) -> NTSTATUS>;

pub type PFLT_INSTANCE_SETUP_CALLBACK = ::core::option::Option<
    unsafe extern "system" fn(
        FltObjects: PCFLT_RELATED_OBJECTS,
        Flags: FLT_INSTANCE_SETUP_FLAGS,
        VolumeDeviceType: ULONG,
        VolumeFilesystemType: FLT_FILESYSTEM_TYPE,
    ) -> NTSTATUS,
>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FLT_RELATED_OBJECTS {
    pub Size: USHORT,
    pub TransactionContext: USHORT,
    pub Filter: PFLT_FILTER,
    pub Volume: PFLT_VOLUME,
    pub Instance: PFLT_INSTANCE,
    pub FileObject: PFILE_OBJECT,
    pub Transaction: PKTRANSACTION,
}

pub type PFILE_OBJECT = MISS_TYPE_PTR;

pub type PCFLT_RELATED_OBJECTS = *const FLT_RELATED_OBJECTS;

pub type PFLT_INSTANCE_QUERY_TEARDOWN_CALLBACK = ::core::option::Option<
    unsafe extern "system" fn(
        FltObjects: PCFLT_RELATED_OBJECTS,
        Flags: FLT_INSTANCE_QUERY_TEARDOWN_FLAGS,
    ) -> NTSTATUS,
>;
pub type FLT_INSTANCE_QUERY_TEARDOWN_FLAGS = ULONG;

pub type FLT_INSTANCE_TEARDOWN_FLAGS = ULONG;
pub type PFLT_INSTANCE_TEARDOWN_CALLBACK = ::core::option::Option<
    unsafe extern "system" fn(
        FltObjects: PCFLT_RELATED_OBJECTS,
        Reason: FLT_INSTANCE_TEARDOWN_FLAGS,
    ),
>;

pub type PFLT_NORMALIZE_CONTEXT_CLEANUP =
    ::core::option::Option<unsafe extern "system" fn(NormalizationContext: *mut PVOID)>;

pub type FLT_INSTANCE_SETUP_FLAGS = ULONG;

pub const FLT_INSTANCE_CONTEXT: USHORT = 2;
pub const FLT_CONTEXT_END: USHORT = 65535;
pub const IRP_MJ_OPERATION_END: u8 = 0x80;

pub const FLT_REGISTRATION_VERSION_0203: USHORT = 0x0203;
pub const FLT_REGISTRATION_VERSION: USHORT = FLT_REGISTRATION_VERSION_0203;

#[repr(C)]
#[derive(Default)]
pub struct _FLT_VOLUME_PROPERTIES {
    pub DeviceType: ULONG,
    pub DeviceCharacteristics: ULONG,
    pub DeviceObjectFlags: ULONG,
    pub AlignmentRequirement: ULONG,
    pub SectorSize: USHORT,
    pub Flags: USHORT,
    pub FileSystemDriverName: UNICODE_STRING,
    pub FileSystemDeviceName: UNICODE_STRING,
    pub RealDeviceName: UNICODE_STRING,
}

pub type FLT_VOLUME_PROPERTIES = _FLT_VOLUME_PROPERTIES;
pub type PFLT_VOLUME_PROPERTIES = *mut _FLT_VOLUME_PROPERTIES;

#[repr(C)]
pub struct _FLT_CALLBACK_DATA {
    pub Flags: FLT_CALLBACK_DATA_FLAGS,
    pub Thread: PETHREAD,
    pub Iopb: PFLT_IO_PARAMETER_BLOCK,
    pub IoStatus: IO_STATUS_BLOCK,
    pub TagData: PVOID,
    pub __bindgen_anon_1: _FLT_CALLBACK_DATA__bindgen_ty_1,
    pub RequestorMode: KPROCESSOR_MODE,
}


UNION!{union _FLT_CALLBACK_DATA__bindgen_ty_1 {
        [u32; 4usize] [u64; 4usize],
        __bindgen_anon_1 __bindgen_anon_1_mut: _FLT_CALLBACK_DATA__bindgen_ty_1__bindgen_ty_1,
        FilterContext FilterContext_mut: [PVOID; 4usize],       
}}

#[repr(C)]
pub struct _FLT_CALLBACK_DATA__bindgen_ty_1__bindgen_ty_1 {
    pub QueueLinks: LIST_ENTRY,
    pub QueueContext: [PVOID; 2usize],
}

pub type FLT_CALLBACK_DATA = _FLT_CALLBACK_DATA;
pub type PFLT_CALLBACK_DATA = *mut _FLT_CALLBACK_DATA;

pub type FLT_IO_PARAMETER_BLOCK = _FLT_IO_PARAMETER_BLOCK;
pub type PFLT_IO_PARAMETER_BLOCK = *mut _FLT_IO_PARAMETER_BLOCK;
pub type FLT_CALLBACK_DATA_FLAGS = ULONG;
pub type FLT_ALLOCATE_CALLBACK_DATA_FLAGS = ULONG;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _FLT_IO_PARAMETER_BLOCK {
    pub IrpFlags: ULONG,
    pub MajorFunction: UCHAR,
    pub MinorFunction: UCHAR,
    pub OperationFlags: UCHAR,
    pub Reserved: UCHAR,
    pub TargetFileObject: PFILE_OBJECT,
    pub TargetInstance: PFLT_INSTANCE,
    pub Parameters: FLT_PARAMETERS,
}

pub type FLT_PARAMETERS = _FLT_PARAMETERS;
pub type PFLT_PARAMETERS = *mut _FLT_PARAMETERS;


UNION! {union _FLT_PARAMETERS {
    [u32; 7usize] [u64; 6usize],
    Create Create_mut: _FLT_PARAMETERS__bindgen_ty_1,
}}


#[cfg(target_arch = "x86")]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _FLT_PARAMETERS__bindgen_ty_1 {
    pub SecurityContext: PIO_SECURITY_CONTEXT,
    pub Options: ULONG,
    pub FileAttributes: USHORT,
    pub ShareAccess: USHORT,
    pub EaLength: ULONG,
    pub EaBuffer: PVOID,
    pub AllocationSize: LARGE_INTEGER,
}

#[cfg(target_arch = "x86_64")]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _FLT_PARAMETERS__bindgen_ty_1 {
    pub SecurityContext: PIO_SECURITY_CONTEXT,
    pub Options: ULONG,
    pub __bindgen_padding_0: [u16; 2usize],
    pub FileAttributes: USHORT,
    pub ShareAccess: USHORT,
    pub __bindgen_padding_1: u32,
    pub EaLength: ULONG,
    pub EaBuffer: PVOID,
    pub AllocationSize: LARGE_INTEGER,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_SECURITY_CONTEXT {
    pub SecurityQos: MISS_TYPE_PTR,
    pub AccessState: MISS_TYPE_PTR,
    pub DesiredAccess: ACCESS_MASK,
    pub FullCreateOptions: ULONG,
}

pub type IO_SECURITY_CONTEXT = _IO_SECURITY_CONTEXT;
pub type PIO_SECURITY_CONTEXT = *mut _IO_SECURITY_CONTEXT;
pub type ACCESS_MASK = ULONG;

pub const IRP_MJ_MAXIMUM_FUNCTION: u32 = 27;

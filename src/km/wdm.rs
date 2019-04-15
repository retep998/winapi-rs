use ::shared::ntdef::*;
use ::shared::basetsd::ULONG_PTR;

use km::ndis::PMDL;
pub use ::shared::minwindef::DWORD;
pub use um::winnt::{WCHAR, OSVERSIONINFOW, PRTL_OSVERSIONINFOW};

#[repr(C)]
#[derive(Default)]
pub struct KSPIN_LOCK {
    pub lock: usize,
}

pub type PDRIVER_INITIALIZE =
Option<extern "system" fn(_self: &mut DRIVER_OBJECT, &UNICODE_STRING) -> NTSTATUS>;
pub type PDRIVER_STARTIO = Option<extern "system" fn(_self: &mut DRIVER_OBJECT, &IRP)>;
pub type PDRIVER_UNLOAD = Option<extern "system" fn(_self: &mut DRIVER_OBJECT)>;
pub type PDRIVER_OBJECT = *mut DRIVER_OBJECT;

#[repr(C)]
pub struct DRIVER_OBJECT {
    pub Type: u16,
    pub Size: u16,
    pub DeviceObject: *mut DEVICE_OBJECT,
    pub Flags: u32,
    pub DriverStart: *const u8,
    pub DriverSize: u32,
    pub DriverSection: *const u8,
    pub DriverExtension: *mut u8,
    pub DriverName: UNICODE_STRING,
    pub HardwareDatabase: *const UNICODE_STRING,
    pub FastIoDispatch: *mut u8,
    pub DriverInit: PDRIVER_INITIALIZE,
    pub DriverStartIo: PDRIVER_STARTIO,
    /// The entry point for the driver's Unload routine, if any.
    pub DriverUnload: PDRIVER_UNLOAD,
    /// A dispatch table consisting of an array of entry points for the driver's `DispatchXxx` routines.
    pub MajorFunction: [PDRIVER_DISPATCH; 28],
}


pub const _KWAIT_REASON_Executive: _KWAIT_REASON = 0;
pub const _KWAIT_REASON_FreePage: _KWAIT_REASON = 1;
pub const _KWAIT_REASON_PageIn: _KWAIT_REASON = 2;
pub const _KWAIT_REASON_PoolAllocation: _KWAIT_REASON = 3;
pub const _KWAIT_REASON_DelayExecution: _KWAIT_REASON = 4;
pub const _KWAIT_REASON_Suspended: _KWAIT_REASON = 5;
pub const _KWAIT_REASON_UserRequest: _KWAIT_REASON = 6;
pub const _KWAIT_REASON_WrExecutive: _KWAIT_REASON = 7;
pub const _KWAIT_REASON_WrFreePage: _KWAIT_REASON = 8;
pub const _KWAIT_REASON_WrPageIn: _KWAIT_REASON = 9;
pub const _KWAIT_REASON_WrPoolAllocation: _KWAIT_REASON = 10;
pub const _KWAIT_REASON_WrDelayExecution: _KWAIT_REASON = 11;
pub const _KWAIT_REASON_WrSuspended: _KWAIT_REASON = 12;
pub const _KWAIT_REASON_WrUserRequest: _KWAIT_REASON = 13;
pub const _KWAIT_REASON_WrSpare0: _KWAIT_REASON = 14;
pub const _KWAIT_REASON_WrQueue: _KWAIT_REASON = 15;
pub const _KWAIT_REASON_WrLpcReceive: _KWAIT_REASON = 16;
pub const _KWAIT_REASON_WrLpcReply: _KWAIT_REASON = 17;
pub const _KWAIT_REASON_WrVirtualMemory: _KWAIT_REASON = 18;
pub const _KWAIT_REASON_WrPageOut: _KWAIT_REASON = 19;
pub const _KWAIT_REASON_WrRendezvous: _KWAIT_REASON = 20;
pub const _KWAIT_REASON_WrKeyedEvent: _KWAIT_REASON = 21;
pub const _KWAIT_REASON_WrTerminated: _KWAIT_REASON = 22;
pub const _KWAIT_REASON_WrProcessInSwap: _KWAIT_REASON = 23;
pub const _KWAIT_REASON_WrCpuRateControl: _KWAIT_REASON = 24;
pub const _KWAIT_REASON_WrCalloutStack: _KWAIT_REASON = 25;
pub const _KWAIT_REASON_WrKernel: _KWAIT_REASON = 26;
pub const _KWAIT_REASON_WrResource: _KWAIT_REASON = 27;
pub const _KWAIT_REASON_WrPushLock: _KWAIT_REASON = 28;
pub const _KWAIT_REASON_WrMutex: _KWAIT_REASON = 29;
pub const _KWAIT_REASON_WrQuantumEnd: _KWAIT_REASON = 30;
pub const _KWAIT_REASON_WrDispatchInt: _KWAIT_REASON = 31;
pub const _KWAIT_REASON_WrPreempted: _KWAIT_REASON = 32;
pub const _KWAIT_REASON_WrYieldExecution: _KWAIT_REASON = 33;
pub const _KWAIT_REASON_WrFastMutex: _KWAIT_REASON = 34;
pub const _KWAIT_REASON_WrGuardedMutex: _KWAIT_REASON = 35;
pub const _KWAIT_REASON_WrRundown: _KWAIT_REASON = 36;
pub const _KWAIT_REASON_WrAlertByThreadId: _KWAIT_REASON = 37;
pub const _KWAIT_REASON_WrDeferredPreempt: _KWAIT_REASON = 38;
pub const _KWAIT_REASON_WrPhysicalFault: _KWAIT_REASON = 39;
pub const _KWAIT_REASON_MaximumWaitReason: _KWAIT_REASON = 40;
pub type _KWAIT_REASON = i32;

pub use self::_KWAIT_REASON as KWAIT_REASON;



#[repr(C)]
pub enum DEVICE_TYPE {
    FILE_DEVICE_BEEP = 0x00000001,
    FILE_DEVICE_CD_ROM = 0x00000002,
    FILE_DEVICE_CD_ROM_FILE_SYSTEM = 0x00000003,
    FILE_DEVICE_CONTROLLER = 0x00000004,
    FILE_DEVICE_DATALINK = 0x00000005,
    FILE_DEVICE_DFS = 0x00000006,
    FILE_DEVICE_DISK = 0x00000007,
    FILE_DEVICE_DISK_FILE_SYSTEM = 0x00000008,
    FILE_DEVICE_FILE_SYSTEM = 0x00000009,
    FILE_DEVICE_INPORT_PORT = 0x0000000a,
    FILE_DEVICE_KEYBOARD = 0x0000000b,
    FILE_DEVICE_MAILSLOT = 0x0000000c,
    FILE_DEVICE_MIDI_IN = 0x0000000d,
    FILE_DEVICE_MIDI_OUT = 0x0000000e,
    FILE_DEVICE_MOUSE = 0x0000000f,
    FILE_DEVICE_MULTI_UNC_PROVIDER = 0x00000010,
    FILE_DEVICE_NAMED_PIPE = 0x00000011,
    FILE_DEVICE_NETWORK = 0x00000012,
    FILE_DEVICE_NETWORK_BROWSER = 0x00000013,
    FILE_DEVICE_NETWORK_FILE_SYSTEM = 0x00000014,
    FILE_DEVICE_NULL = 0x00000015,
    FILE_DEVICE_PARALLEL_PORT = 0x00000016,
    FILE_DEVICE_PHYSICAL_NETCARD = 0x00000017,
    FILE_DEVICE_PRINTER = 0x00000018,
    FILE_DEVICE_SCANNER = 0x00000019,
    FILE_DEVICE_SERIAL_MOUSE_PORT = 0x0000001a,
    FILE_DEVICE_SERIAL_PORT = 0x0000001b,
    FILE_DEVICE_SCREEN = 0x0000001c,
    FILE_DEVICE_SOUND = 0x0000001d,
    FILE_DEVICE_STREAMS = 0x0000001e,
    FILE_DEVICE_TAPE = 0x0000001f,
    FILE_DEVICE_TAPE_FILE_SYSTEM = 0x00000020,
    FILE_DEVICE_TRANSPORT = 0x00000021,
    FILE_DEVICE_UNKNOWN = 0x00000022,
    FILE_DEVICE_VIDEO = 0x00000023,
    FILE_DEVICE_VIRTUAL_DISK = 0x00000024,
    FILE_DEVICE_WAVE_IN = 0x00000025,
    FILE_DEVICE_WAVE_OUT = 0x00000026,
    FILE_DEVICE_8042_PORT = 0x00000027,
    FILE_DEVICE_NETWORK_REDIRECTOR = 0x00000028,
    FILE_DEVICE_BATTERY = 0x00000029,
    FILE_DEVICE_BUS_EXTENDER = 0x0000002a,
    FILE_DEVICE_MODEM = 0x0000002b,
    FILE_DEVICE_VDM = 0x0000002c,
    FILE_DEVICE_MASS_STORAGE = 0x0000002d,
    FILE_DEVICE_SMB = 0x0000002e,
    FILE_DEVICE_KS = 0x0000002f,
    FILE_DEVICE_CHANGER = 0x00000030,
    FILE_DEVICE_SMARTCARD = 0x00000031,
    FILE_DEVICE_ACPI = 0x00000032,
    FILE_DEVICE_DVD = 0x00000033,
    FILE_DEVICE_FULLSCREEN_VIDEO = 0x00000034,
    FILE_DEVICE_DFS_FILE_SYSTEM = 0x00000035,
    FILE_DEVICE_DFS_VOLUME = 0x00000036,
    FILE_DEVICE_SERENUM = 0x00000037,
    FILE_DEVICE_TERMSRV = 0x00000038,
    FILE_DEVICE_KSEC = 0x00000039,
    FILE_DEVICE_FIPS = 0x0000003A,
    FILE_DEVICE_INFINIBAND = 0x0000003B,
    FILE_DEVICE_VMBUS = 0x0000003E,
    FILE_DEVICE_CRYPT_PROVIDER = 0x0000003F,
    FILE_DEVICE_WPD = 0x00000040,
    FILE_DEVICE_BLUETOOTH = 0x00000041,
    FILE_DEVICE_MT_COMPOSITE = 0x00000042,
    FILE_DEVICE_MT_TRANSPORT = 0x00000043,
    FILE_DEVICE_BIOMETRIC = 0x00000044,
    FILE_DEVICE_PMI = 0x00000045,
    FILE_DEVICE_EHSTOR = 0x00000046,
    FILE_DEVICE_DEVAPI = 0x00000047,
    FILE_DEVICE_GPIO = 0x00000048,
    FILE_DEVICE_USBEX = 0x00000049,
    FILE_DEVICE_CONSOLE = 0x00000050,
    FILE_DEVICE_NFP = 0x00000051,
    FILE_DEVICE_SYSENV = 0x00000052,
    FILE_DEVICE_VIRTUAL_BLOCK = 0x00000053,
    FILE_DEVICE_POINT_OF_SERVICE = 0x00000054,
    FILE_DEVICE_STORAGE_REPLICATION = 0x00000055,
    FILE_DEVICE_TRUST_ENV = 0x00000056,
    FILE_DEVICE_UCM = 0x00000057,
    FILE_DEVICE_UCMTCPCI = 0x00000058,
    FILE_DEVICE_PERSISTENT_MEMORY = 0x00000059,
    FILE_DEVICE_NVDIMM = 0x0000005a,
    FILE_DEVICE_HOLOGRAPHIC = 0x0000005b,
    FILE_DEVICE_SDFXHCI = 0x0000005c,
}

/// Device object flags.
#[repr(C)]
pub enum DEVICE_FLAGS {
    NONE = 0,
    DO_VERIFY_VOLUME = 0x00000002,
    DO_BUFFERED_IO = 0x00000004,
    DO_EXCLUSIVE = 0x00000008,
    DO_DIRECT_IO = 0x00000010,
    DO_MAP_IO_BUFFER = 0x00000020,
    DO_DEVICE_HAS_NAME = 0x00000040,
    DO_DEVICE_INITIALIZING = 0x00000080,
    DO_SYSTEM_BOOT_PARTITION = 0x00000100,
    DO_LONG_TERM_REQUESTS = 0x00000200,
    DO_NEVER_LAST_DEVICE = 0x00000400,
    DO_SHUTDOWN_REGISTERED = 0x00000800,
    DO_BUS_ENUMERATED_DEVICE = 0x00001000,
    DO_POWER_PAGABLE = 0x00002000,
    DO_POWER_INRUSH = 0x00004000,
    DO_POWER_NOOP = 0x00008000,
    DO_LOW_PRIORITY_FILESYSTEM = 0x00010000,
    DO_XIP = 0x00020000,
}

/// `IoCompletion` routine result.
#[repr(u32)]
pub enum IO_COMPLETION_ROUTINE_RESULT {
    // STATUS_SUCCESS
    ContinueCompletion = 0,
    // STATUS_MORE_PROCESSING_REQUIRED
    StopCompletion = 0xC0000016,
}

STRUCT!{ struct DISPATCHER_HEADER {
    Type: u8,
    Absolute: u8,
    Size: u8,
    Inserted: u8,
    SignalState: i32,
    WaitListHead: LIST_ENTRY,
}}

/// The `DEVICE_OBJECT` structure is used by the operating system to represent a device object.
#[repr(C)]
pub struct DEVICE_OBJECT {
    pub Type: u16,
    pub Size: u16,
    pub ReferenceCount: i32,
    pub DriverObject: *const DRIVER_OBJECT,
    pub NextDevice: *mut DEVICE_OBJECT,
    pub AttachedDevice: *mut DEVICE_OBJECT,
    pub CurrentIrp: *const IRP,
    pub Timer: *mut u8,
    pub Flags: u32,
    pub Characteristics: u32,
    pub Vpb: *mut u8,
    pub DeviceExtension: *mut u8,
    pub DeviceType: u32,
    pub StackSize: u8,
    pub Queue: *mut WAIT_CONTEXT_BLOCK,
    pub AlignmentRequirement: u32,
    pub DeviceQueue: KDEVICE_QUEUE,
    pub Dpc: KDPC,
    pub ActiveThreadCount: u32,
    pub SecurityDescriptor: *const u8,
    pub DeviceLock: KEVENT,
    pub SectorSize: u16,
    pub Spare1: u16,
    pub DeviceObjectExtension: *mut DEVOBJ_EXTENSION,
    pub Reserved: *const u8,
}

impl DEVICE_OBJECT {
    /// Return a reference to driver-defined data structure.
    pub fn extension<T>(&mut self) -> &mut T {
        unsafe { &mut *(self.DeviceExtension as *mut T) }
    }
}

/// Device object extension structure.
#[repr(C)]
pub struct DEVOBJ_EXTENSION {
    Type: u16,
    Size: u16,
    DeviceObject: *mut DEVICE_OBJECT,
    PowerFlags: u32,
    Dope: *mut u8,
    ExtensionFlags: u32,
    DeviceNode: *mut u8,
    AttachedTo: *mut DEVICE_OBJECT,
    StartIoCount: i32,
    StartIoKey: i32,
    StartIoFlags: u32,
    Vpb: *mut u8,
}

pub type PDEVICE_OBJECT = *mut DEVICE_OBJECT;
pub type PDRIVER_CANCEL =
Option<extern "system" fn(DeviceObject: &mut DEVICE_OBJECT, Irp: &mut IRP)>;

pub type PDRIVER_DISPATCH =
Option<unsafe extern "system" fn(DeviceObject: &mut DEVICE_OBJECT, Irp: &mut IRP) -> NTSTATUS>;

pub type PIO_COMPLETION_ROUTINE = Option<
    extern "system" fn(DeviceObject: &mut DEVICE_OBJECT, Irp: &mut IRP, Context: PVOID)
                       -> IO_COMPLETION_ROUTINE_RESULT,
>;

pub type KIRQL = UCHAR;
pub type EX_SPIN_LOCK = LONG;
pub type PEX_SPIN_LOCK = *mut LONG;

#[link(name = "ntoskrnl")]
extern "system" {
    pub fn ExAcquireSpinLockExclusive(
        SpinLock: PEX_SPIN_LOCK,
    ) -> KIRQL;

    pub fn IoCreateDevice(
        DriverObject: *mut DRIVER_OBJECT,
        DeviceExtensionSize: u32,
        DeviceName: PCUNICODE_STRING,
        DeviceType: DEVICE_TYPE,
        DeviceCharacteristics: u32,
        Exclusive: BOOLEAN,
        DeviceObject: *mut PDEVICE_OBJECT,
    ) -> NTSTATUS;

    pub fn IoDeleteDevice(DeviceObject: *mut DEVICE_OBJECT) -> NTSTATUS;
    pub fn IoCreateSymbolicLink(
        SymbolicLinkName: &UNICODE_STRING,
        DeviceName: &UNICODE_STRING,
    ) -> NTSTATUS;
    pub fn IoDeleteSymbolicLink(SymbolicLinkName: &UNICODE_STRING) -> NTSTATUS;
    pub fn ExReleaseSpinLockExclusive(
        SpinLock: PEX_SPIN_LOCK,
        OldIrql: KIRQL,
    );
    pub fn ExAcquireSpinLockShared(SpinLock: PEX_SPIN_LOCK) -> KIRQL;
    pub fn ExReleaseSpinLockShared(SpinLock: PEX_SPIN_LOCK, OldIrql: KIRQL);
    pub fn RtlGetVersion(lpVersionInformation: PRTL_OSVERSIONINFOW) -> NTSTATUS;
    pub fn IoCompleteRequest(Irp: PIRP, PriorityBoost: KPRIORITY_BOOST);
    pub fn IoAllocateIrp(StackSize: CCHAR, ChargeQuota: bool) -> PIRP;
    pub fn IoFreeIrp(Irp: PIRP);
    pub fn IoReuseIrp(Irp: PIRP, Status: NTSTATUS);
    pub fn IoInitializeIrp(Irp: PIRP, PacketSize: USHORT, StackSize: CCHAR);
    pub fn IoMakeAssociatedIrp(Irp: PIRP, StackSize: CCHAR) -> PIRP;
}

//see wdm.h define of IofCallDriver
#[cfg(target_arch = "x86")]
mod fastcall {
    use super::*;
    extern "fastcall" {
        pub fn IofCallDriver(DeviceObject: PDEVICE_OBJECT, Irp: PIRP) -> NTSTATUS;
    }
}
#[cfg(target_arch = "x86_64")]
mod fastcall {
    use super::*;
    extern "system" {
        pub fn IofCallDriver(DeviceObject: PDEVICE_OBJECT, Irp: PIRP) -> NTSTATUS;
    }
}

pub use self::fastcall::*;
pub type PIRP = *mut IRP;
pub type PIO_STACK_LOCATION = *mut IO_STACK_LOCATION;

#[repr(u8)]
pub enum IRP_MJ {
    /// The operating system sends this request to open a handle to a file object or device object.
    CREATE,
    CREATE_NAMED_PIPE,
    /// Indicates that the last handle of the file object that is associated with the target device object
    /// has been closed and released. All outstanding I/O requests have been completed or canceled.
    /// See also `CLEANUP`.
    CLOSE,
    /// A user-mode application or Win32 component has requested a data transfer from the device.
    /// Or a higher-level driver has created and set up the read IRP.
    READ,
    /// A user-mode application or Win32 component has requested a data transfer to the device.
    /// Or a higher-level driver has created and set up the write IRP.
    WRITE,
    QUERY_INFORMATION,
    SET_INFORMATION,
    QUERY_EA,
    SET_EA,
    /// Indicates that the driver should flush the device's cache or its internal buffer,
    /// or, possibly, should discard the data in its internal buffer.
    FLUSH_BUFFERS,
    QUERY_VOLUME_INFORMATION,
    SET_VOLUME_INFORMATION,
    DIRECTORY_CONTROL,
    FILE_SYSTEM_CONTROL,
    /// An user-mode thread has called the Microsoft Win32 `DeviceIoControl` function, or a higher-level kernel-mode driver has set up the request.
    DEVICE_CONTROL,
    /// Some driver calls either `IoBuildDeviceIoControlRequest` or `IoAllocateIrp` to create a request.
    INTERNAL_DEVICE_CONTROL,
    /// Indicates that a file system driver is sending notice that the system is being shut down.
    SHUTDOWN,
    LOCK_CONTROL,
    /// Indicates that the last handle for a file object that is associated with the target device object has been closed
    /// (but, due to outstanding I/O requests, might not have been released).
    /// See also `CLOSE`.
    CLEANUP,
    CREATE_MAILSLOT,
    QUERY_SECURITY,
    SET_SECURITY,
    POWER,
    SYSTEM_CONTROL,
    DEVICE_CHANGE,
    QUERY_QUOTA,
    SET_QUOTA,
    PNP,
    MAXIMUM_FUNCTION,
}

#[repr(C)]
pub struct _KAPC {
    pub Type: UCHAR,
    pub SpareByte0: UCHAR,
    pub Size: UCHAR,
    pub SpareByte1: UCHAR,
    pub SpareLong0: ULONG,
    pub Thread: PVOID,
    pub ApcListEntry: LIST_ENTRY,
    pub Reserved: [PVOID; 3usize],
    pub NormalContext: PVOID,
    pub SystemArgument1: PVOID,
    pub SystemArgument2: PVOID,
    pub ApcStateIndex: CCHAR,
    pub ApcMode: KPROCESSOR_MODE,
    pub Inserted: BOOLEAN,
}

pub type KAPC = _KAPC;

#[repr(C)]
pub struct IRP {
    pub Type: CSHORT,
    pub Size: USHORT,
    pub MdlAddress: PMDL,
    pub Flags: ULONG,
    pub AssociatedIrp: _IRP__bindgen_ty_1,
    pub ThreadListEntry: LIST_ENTRY,
    pub IoStatus: IO_STATUS_BLOCK,
    pub RequestorMode: KPROCESSOR_MODE,
    pub PendingReturned: BOOLEAN,
    pub StackCount: CHAR,
    pub CurrentLocation: CHAR,
    pub Cancel: BOOLEAN,
    pub CancelIrql: KIRQL,
    pub ApcEnvironment: CCHAR,
    pub AllocationFlags: UCHAR,
    pub UserIosb: PIO_STATUS_BLOCK,
    pub UserEvent: PKEVENT,
    pub Overlay: _IRP__bindgen_ty_2,
    pub CancelRoutine: PDRIVER_CANCEL,
    pub UserBuffer: PVOID,
    pub Tail: _IRP__bindgen_ty_3,
}

UNION!{union _IRP__bindgen_ty_1 {
    [u32;1] [u64;1],
    MasterIrp MasterIrp_mut: *mut IRP,
    IrpCount IrpCount_mut: LONG,
    SystemBuffer SystemBuffer_mut: PVOID,
}}

UNION!{union _IRP__bindgen_ty_2 {
    [u64;1] [u64; 2usize],
    AsynchronousParameters AsynchronousParameters_mut: _IRP__bindgen_ty_2__bindgen_ty_1,
    AllocationSize AllocationSize_mut: LARGE_INTEGER,
}}

#[repr(C)]
pub struct _IRP__bindgen_ty_2__bindgen_ty_1 {
    __bindgen_anon_1 : _IRP__bindgen_ty_2__bindgen_ty_1__bindgen_ty_1,
    UserApcContext : PVOID,
}

UNION!{union _IRP__bindgen_ty_2__bindgen_ty_1__bindgen_ty_1 {
    [u32;1] [u64; 1],
    UserApcRoutine UserApcRoutine_mut: PIO_APC_ROUTINE,
    IssuingProcess IssuingProcess_mut: PVOID,
}}

UNION!{union _IRP__bindgen_ty_3 {
    [u32;12usize] [u64; 11usize],
    Overlay Overlay_mut: _IRP__bindgen_ty_3__bindgen_ty_1,
    Apc Apc_mut: KAPC,
    CompletionKey CompletionKey_mut: PVOID,
}}

#[repr(C)]
pub struct _IRP__bindgen_ty_3__bindgen_ty_1 {
    pub __bindgen_anon_1: _IRP__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1,
    pub Thread: PETHREAD,
    pub AuxiliaryBuffer: PCHAR,
    pub __bindgen_anon_2: _IRP__bindgen_ty_3__bindgen_ty_1__bindgen_ty_2,
    pub OriginalFileObject: PFILE_OBJECT,
}

#[repr(C)]
pub struct _IRP__bindgen_ty_3__bindgen_ty_1__bindgen_ty_2 {
    pub ListEntry: LIST_ENTRY,
    pub __bindgen_anon_1: _IRP__bindgen_ty_3__bindgen_ty_1__bindgen_ty_2__bindgen_ty_1,
}

UNION!{union _IRP__bindgen_ty_3__bindgen_ty_1__bindgen_ty_2__bindgen_ty_1 {
    [u32;1] [u64; 1],
    CurrentStackLocation CurrentStackLocation_mut: PIO_STACK_LOCATION,
    PacketType PacketType_mut: ULONG,
}}

UNION!{union _IRP__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1 {
 [u32; 4usize] [u64; 4usize],
    DeviceQueueEntry DeviceQueueEntry_mut: KDEVICE_QUEUE_ENTRY,
    __bindgen_anon_1 __bindgen_anon_1_mut: _IRP__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1,
}}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IRP__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1 {
    pub DriverContext: [PVOID; 4usize],
}

pub type PEPROCESS = PVOID;
pub type PETHREAD = PVOID;

#[repr(C)]
pub struct _IRP_OVERLAY {
    pub DriverContext: [PVOID; 4],
    pub Thread: PETHREAD,
    pub AuxiliaryBuffer: PVOID,
    pub ListEntry: LIST_ENTRY,
    /// Current stack location.
    pub CurrentStackLocation: PIO_STACK_LOCATION,
    pub OriginalFileObject: PFILE_OBJECT,
}

pub const SL_PENDING_RETURNED: u8 = 0x01;
pub const SL_INVOKE_ON_CANCEL: u8 = 0x20;
pub const SL_INVOKE_ON_SUCCESS: u8 = 0x40;
pub const SL_INVOKE_ON_ERROR: u8 = 0x80;


#[repr(C)]
pub struct IO_STACK_LOCATION {
    pub MajorFunction: UCHAR,
    pub MinorFunction: UCHAR,
    pub Flags: UCHAR,
    pub Control: UCHAR,
    pub Parameters: IO_STACK_LOCATION_s1_Parameters,
    pub DeviceObject: PDEVICE_OBJECT,
    pub FileObject: PFILE_OBJECT,
    pub CompletionRoutine: PIO_COMPLETION_ROUTINE,
    pub Context: PVOID,
}

UNION!{union  IO_STACK_LOCATION_s1_Parameters{
     [u32; 4usize]  [u64; 4usize],
    DeviceIoControl DeviceIoControl_mut: IO_STACK_LOCATION_s1_Parameters_u1_DeviceIoControl,
}}

#[cfg(target_arch = "x86")]
mod _hide {
    use super::*;
    #[repr(C)]
    #[derive(Clone)]
    pub struct IO_STACK_LOCATION_s1_Parameters_u1_DeviceIoControl {
        pub OutputBufferLength: ULONG,
        pub InputBufferLength: ULONG,
        pub IoControlCode: ULONG,
        pub Type3InputBuffer: PVOID,
    }
}

#[cfg(target_arch = "x86_64")]
mod _hide {
    use super::*;
    #[repr(C)]
    #[derive(Clone)]
    pub struct IO_STACK_LOCATION_s1_Parameters_u1_DeviceIoControl {
        pub OutputBufferLength: ULONG,
        pub __bindgen_padding_0: u32,
        pub InputBufferLength: ULONG,
        pub __bindgen_padding_1: u32,
        pub IoControlCode: ULONG,
        pub Type3InputBuffer: PVOID,
    }
}

pub use self::_hide::*;

pub fn IoGetCurrentIrpStackLocation(pirp: PIRP) -> PIO_STACK_LOCATION {
    unsafe {
        return *((&mut *pirp)
        .Tail
        .Overlay()
        .__bindgen_anon_2
        .__bindgen_anon_1
        .CurrentStackLocation());
    }
}

#[repr(C)]
pub struct IO_STACK_LOCATION_Parameters_DeviceControl {
    pub OutputBufferLength: ULONG,
    pub InputBufferLength: ULONG,
    pub IoControlCode: ULONG,
    pub Type3InputBuffer: PVOID,
}

#[repr(C)]
pub struct _IO_STACK_LOCATION_READ {
    pub Length: u32,
    pub Key: u32,
    pub ByteOffset: u64,
}

impl IO_STACK_LOCATION {
    pub fn ParametersRead(&mut self) -> &mut _IO_STACK_LOCATION_READ {
        unsafe { ::core::mem::transmute(&mut self.Parameters) }
    }
}


extern "system" {
    pub fn KeWaitForSingleObject(
        Object: PVOID,
        WaitReason: u32,
        WaitMode: KPROCESSOR_MODE,
        Alertable: bool,
        Timeout: Option<&u32>,
    ) -> NTSTATUS;
}

#[repr(C)]
pub struct WAIT_CONTEXT_BLOCK {
    WaitQueueEntry: *mut KDEVICE_QUEUE_ENTRY,
    DeviceRoutine: extern "system" fn(_obj: PDEVICE_OBJECT, _irp: *mut IRP, *mut u8, *mut u8)
                                      -> IO_ALLOCATION_ACTION,
    DeviceContext: *mut u8,
    NumberOfMapRegisters: u32,
    DeviceObject: *mut u8,
    CurrentIrp: *mut u8,
    BufferChainingDpc: *mut u8,
}

#[repr(C)]
pub enum IO_ALLOCATION_ACTION {
    KeepObject = 0x01,
    DeallocateObject = 0x02,
    DeallocateObjectKeepRegisters = 0x03,
}

#[repr(C)]
pub struct KDEVICE_QUEUE_ENTRY {
    DeviceListEntry: LIST_ENTRY,
    SortKey: u32,
    Inserted: bool,
}

#[repr(C)]
pub struct KDEVICE_QUEUE {
    Type: u16,
    Size: u16,
    DeviceListHead: LIST_ENTRY,
    Lock: KSPIN_LOCK,
    Busy: bool,
}

extern "system" {
    pub fn KeInitializeDpc(
        Dpc: *mut KDPC,
        DeferredRoutine: PDEFERRED_ROUTINE,
        DeferredContext: *mut u8,
    );
    pub fn KeInsertQueueDpc(
        Dpc: *mut KDPC,
        SystemArgument1: *const u8,
        SystemArgument2: *const u8,
    ) -> bool;
    pub fn KeRemoveQueueDpc(Dpc: *mut KDPC) -> bool;
    pub fn KeFlushQueuedDpcs();
    pub fn KeGenericCallDpc(DeferredRoutine: PDEFERRED_ROUTINE, DeferredContext: *mut u8);
}

pub type PDEFERRED_ROUTINE = extern "system" fn(
    Dpc: *const KDPC,
    DeferredContext: *mut u8,
    SystemArgument1: *const u8,
    SystemArgument2: *const u8,
);

/// Deferred Procedure Call object.
#[repr(C)]
pub struct KDPC {
    Type: u8,
    Number: u8,
    Importance: u8,

    DpcListEntry: LIST_ENTRY,
    DeferredRoutine: PDEFERRED_ROUTINE,
    DeferredContext: *mut u8,
    SystemArgument1: *mut u8,
    SystemArgument2: *mut u8,

    DpcData: *mut KDPC_DATA,
}

/// DPC data structure definition.
#[repr(C)]
pub struct KDPC_DATA {
    DpcListHead: LIST_ENTRY,
    DpcLock: KSPIN_LOCK,
    DpcQueueDepth: i32,
    DpcCount: u32,
}

extern "system" {
    pub fn KeInitializeEvent(Event: PKEVENT, Type: EVENT_TYPE, State: bool);
    pub fn KeSetEvent(Event: PKEVENT, Increment: i32, Wait: bool) -> i32;
    pub fn KeReadStateEvent(Event: PKEVENT) -> i32;
    pub fn KeResetEvent(Event: PKEVENT) -> i32;
    pub fn KeClearEvent(Event: PKEVENT);
}


pub type PKEVENT = *mut KEVENT;

ENUM! {
    enum EVENT_TYPE {
        NotificationEvent = 0,
        SynchronizationEvent,
    }
}

STRUCT!{ struct KEVENT {
    Header: DISPATCHER_HEADER,
}}

use self::IO_PRIORITY::*;

/// I/O Request priority.
pub mod IO_PRIORITY {
    /// I/O Request priority type.
    pub type KPRIORITY_BOOST = u8;

    pub const IO_NO_INCREMENT: KPRIORITY_BOOST = 0;
    pub const IO_DISK_INCREMENT: KPRIORITY_BOOST = 1;
    pub const EVENT_INCREMENT: KPRIORITY_BOOST = 1;
}

pub type KPRIORITY = KPRIORITY_BOOST;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct IO_STATUS_BLOCK {
    pub __bindgen_anon_1: _IO_STATUS_BLOCK__bindgen_ty_1,
    pub Information: ULONG_PTR,
}

UNION!{union _IO_STATUS_BLOCK__bindgen_ty_1 {
    [u32;1] [u64;1],
    Status Status_mut: NTSTATUS,
    Pointer Pointer_mut: PVOID,
}}

pub type PIO_STATUS_BLOCK = *mut IO_STATUS_BLOCK;

/// Processor modes.
#[repr(u8)]
pub enum KPROCESSOR_MODE {
    KernelMode,
    UserMode,
}

pub type PIO_APC_ROUTINE = Option<
    extern "system" fn(ApcContext: PVOID, IoStatusBlock: *const IO_STATUS_BLOCK, Reserved: u32),
>;

pub type PFILE_OBJECT = *mut FILE_OBJECT;

/// The `FILE_OBJECT` structure is used by the system to represent a file object.
#[repr(C)]
pub struct FILE_OBJECT {
    Type: u16,
    Size: u16,
    DeviceObject: PDEVICE_OBJECT,
    // ...
}

extern "cdecl" {
    pub fn DbgPrint(Format: *const u8, ...) -> NTSTATUS;
    pub fn DbgPrintEx(ComponentId: u32, Level: u32, Format: *const u8, ...) -> NTSTATUS;
}

extern "system" {
    pub fn DbgBreakPoint();
    pub fn DbgBreakPointWithStatus(Status: NTSTATUS);
}

//access right
pub const FILE_READ_DATA: u32 = 1;
pub const FILE_LIST_DIRECTORY: u32 = 1;
pub const FILE_WRITE_DATA: u32 = 2;
pub const FILE_ADD_FILE: u32 = 2;
pub const FILE_APPEND_DATA: u32 = 4;
pub const FILE_ADD_SUBDIRECTORY: u32 = 4;
pub const FILE_CREATE_PIPE_INSTANCE: u32 = 4;
pub const FILE_READ_EA: u32 = 8;
pub const FILE_WRITE_EA: u32 = 16;
pub const FILE_EXECUTE: u32 = 32;
pub const FILE_TRAVERSE: u32 = 32;
pub const FILE_DELETE_CHILD: u32 = 64;
pub const FILE_READ_ATTRIBUTES: u32 = 128;
pub const FILE_WRITE_ATTRIBUTES: u32 = 256;
pub const FILE_ALL_ACCESS: u32 = 2032127;
pub const FILE_GENERIC_READ: u32 = 1179785;
pub const FILE_GENERIC_WRITE: u32 = 1179926;
pub const FILE_GENERIC_EXECUTE: u32 = 1179808;

pub const DELETE: u32 = 65536;
pub const READ_CONTROL: u32 = 131072;
pub const WRITE_DAC: u32 = 262144;
pub const WRITE_OWNER: u32 = 524288;
pub const SYNCHRONIZE: u32 = 1048576;
pub const STANDARD_RIGHTS_REQUIRED: u32 = 983040;
pub const STANDARD_RIGHTS_READ: u32 = 131072;
pub const STANDARD_RIGHTS_WRITE: u32 = 131072;
pub const STANDARD_RIGHTS_EXECUTE: u32 = 131072;
pub const STANDARD_RIGHTS_ALL: u32 = 2031616;
pub const SPECIFIC_RIGHTS_ALL: u32 = 65535;
pub const ACCESS_SYSTEM_SECURITY: u32 = 16777216;
pub const MAXIMUM_ALLOWED: u32 = 33554432;
pub const GENERIC_READ: u32 = 2147483648;
pub const GENERIC_WRITE: u32 = 1073741824;
pub const GENERIC_EXECUTE: u32 = 536870912;
pub const GENERIC_ALL: u32 = 268435456;

#[repr(C)]
pub enum POOL_TYPE
{
    NonPagedPool,
    NonPagedPoolExecute,
}

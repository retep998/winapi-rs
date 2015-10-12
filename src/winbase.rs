// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! This module defines the 32-Bit Windows Base APIs
//99
pub const FILE_BEGIN: ::DWORD = 0;
pub const FILE_CURRENT: ::DWORD = 1;
pub const FILE_END: ::DWORD = 2;
pub const WAIT_FAILED: ::DWORD = 0xFFFFFFFF;
pub const WAIT_OBJECT_0: ::DWORD = ::STATUS_WAIT_0 as ::DWORD;
pub const WAIT_ABANDONED: ::DWORD = ::STATUS_ABANDONED_WAIT_0 as ::DWORD;
pub const WAIT_ABANDONED_0: ::DWORD = ::STATUS_ABANDONED_WAIT_0 as ::DWORD;
pub const WAIT_IO_COMPLETION: ::DWORD = ::STATUS_USER_APC as ::DWORD;
//123
pub const FILE_FLAG_WRITE_THROUGH: ::DWORD = 0x80000000;
pub const FILE_FLAG_OVERLAPPED: ::DWORD = 0x40000000;
pub const FILE_FLAG_NO_BUFFERING: ::DWORD = 0x20000000;
pub const FILE_FLAG_RANDOM_ACCESS: ::DWORD = 0x10000000;
pub const FILE_FLAG_SEQUENTIAL_SCAN: ::DWORD = 0x08000000;
pub const FILE_FLAG_DELETE_ON_CLOSE: ::DWORD = 0x04000000;
pub const FILE_FLAG_BACKUP_SEMANTICS: ::DWORD = 0x02000000;
pub const FILE_FLAG_POSIX_SEMANTICS: ::DWORD = 0x01000000;
pub const FILE_FLAG_SESSION_AWARE: ::DWORD = 0x00800000;
pub const FILE_FLAG_OPEN_REPARSE_POINT: ::DWORD = 0x00200000;
pub const FILE_FLAG_OPEN_NO_RECALL: ::DWORD = 0x00100000;
pub const FILE_FLAG_FIRST_PIPE_INSTANCE: ::DWORD = 0x00080000;
pub const FILE_FLAG_OPEN_REQUIRING_OPLOCK: ::DWORD = 0x00040000;
pub const PROGRESS_CONTINUE: ::DWORD = 0;
pub const PROGRESS_CANCEL: ::DWORD = 1;
pub const PROGRESS_STOP: ::DWORD = 2;
pub const PROGRESS_QUIET: ::DWORD = 3;
pub const CALLBACK_CHUNK_FINISHED: ::DWORD = 0x00000000;
pub const CALLBACK_STREAM_SWITCH: ::DWORD = 0x00000001;
pub const COPY_FILE_FAIL_IF_EXISTS: ::DWORD = 0x00000001;
pub const COPY_FILE_RESTARTABLE: ::DWORD = 0x00000002;
pub const COPY_FILE_OPEN_SOURCE_FOR_WRITE: ::DWORD = 0x00000004;
pub const COPY_FILE_ALLOW_DECRYPTED_DESTINATION: ::DWORD = 0x00000008;
pub const COPY_FILE_COPY_SYMLINK: ::DWORD = 0x00000800;
pub const COPY_FILE_NO_BUFFERING: ::DWORD = 0x00001000;
pub const COPY_FILE_REQUEST_SECURITY_PRIVILEGES: ::DWORD = 0x00002000;
pub const COPY_FILE_RESUME_FROM_PAUSE: ::DWORD = 0x00004000;
pub const COPY_FILE_NO_OFFLOAD: ::DWORD = 0x00040000;
pub const REPLACEFILE_WRITE_THROUGH: ::DWORD = 0x00000001;
pub const REPLACEFILE_IGNORE_MERGE_ERRORS: ::DWORD = 0x00000002;
pub const REPLACEFILE_IGNORE_ACL_ERRORS: ::DWORD = 0x00000004;
pub const PIPE_ACCESS_INBOUND: ::DWORD = 0x00000001;
pub const PIPE_ACCESS_OUTBOUND: ::DWORD = 0x00000002;
pub const PIPE_ACCESS_DUPLEX: ::DWORD = 0x00000003;
pub const PIPE_CLIENT_END: ::DWORD = 0x00000000;
pub const PIPE_SERVER_END: ::DWORD = 0x00000001;
pub const PIPE_WAIT: ::DWORD = 0x00000000;
pub const PIPE_NOWAIT: ::DWORD = 0x00000001;
pub const PIPE_READMODE_BYTE: ::DWORD = 0x00000000;
pub const PIPE_READMODE_MESSAGE: ::DWORD = 0x00000002;
pub const PIPE_TYPE_BYTE: ::DWORD = 0x00000000;
pub const PIPE_TYPE_MESSAGE: ::DWORD = 0x00000004;
pub const PIPE_ACCEPT_REMOTE_CLIENTS: ::DWORD = 0x00000000;
pub const PIPE_REJECT_REMOTE_CLIENTS: ::DWORD = 0x00000008;
pub const PIPE_UNLIMITED_INSTANCES: ::DWORD = 255;
//268
pub const SECURITY_CONTEXT_TRACKING: ::DWORD = 0x00040000;
pub const SECURITY_EFFECTIVE_ONLY: ::DWORD = 0x00080000;
pub const SECURITY_SQOS_PRESENT: ::DWORD = 0x00100000;
pub const SECURITY_VALID_SQOS_FLAGS: ::DWORD = 0x001F0000;
//664
pub const DRIVE_UNKNOWN: ::DWORD = 0;
pub const DRIVE_NO_ROOT_DIR: ::DWORD = 1;
pub const DRIVE_REMOVABLE: ::DWORD = 2;
pub const DRIVE_FIXED: ::DWORD = 3;
pub const DRIVE_REMOTE: ::DWORD = 4;
pub const DRIVE_CDROM: ::DWORD = 5;
pub const DRIVE_RAMDISK: ::DWORD = 6;
pub const FILE_TYPE_UNKNOWN: ::DWORD = 0x0000;
pub const FILE_TYPE_DISK: ::DWORD = 0x0001;
pub const FILE_TYPE_CHAR: ::DWORD = 0x0002;
pub const FILE_TYPE_PIPE: ::DWORD = 0x0003;
pub const FILE_TYPE_REMOTE: ::DWORD = 0x8000;
pub const STD_INPUT_HANDLE: ::DWORD = 0xFFFFFFF6;
pub const STD_OUTPUT_HANDLE: ::DWORD = 0xFFFFFFF5;
pub const STD_ERROR_HANDLE: ::DWORD = 0xFFFFFFF4;
pub const NOPARITY: ::DWORD = 0;
pub const ODDPARITY: ::DWORD = 1;
pub const EVENPARITY: ::DWORD = 2;
pub const MARKPARITY: ::DWORD = 3;
pub const SPACEPARITY: ::DWORD = 4;
pub const ONESTOPBIT: ::DWORD = 0;
pub const ONE5STOPBITS: ::DWORD = 1;
pub const TWOSTOPBITS: ::DWORD = 2;
pub const IGNORE: ::DWORD = 0;
pub const INFINITE: ::DWORD = 0xFFFFFFFF;
//1728
pub const SEM_FAILCRITICALERRORS: ::UINT = 0x0001;
pub const SEM_NOGPFAULTERRORBOX: ::UINT = 0x0002;
pub const SEM_NOALIGNMENTFAULTEXCEPT: ::UINT = 0x0004;
pub const SEM_NOOPENFILEERRORBOX: ::UINT = 0x8000;
//1733
//2939
// startup flags STARTUPINFOW::dwFlags
pub const STARTF_USESHOWWINDOW: ::DWORD = 0x00000001;
pub const STARTF_USESIZE: ::DWORD = 0x00000002;
pub const STARTF_USEPOSITION: ::DWORD = 0x00000004;
pub const STARTF_USECOUNTCHARS: ::DWORD = 0x00000008;
pub const STARTF_USEFILLATTRIBUTE: ::DWORD = 0x00000010;
pub const STARTF_RUNFULLSCREEN: ::DWORD = 0x00000020;
pub const STARTF_FORCEONFEEDBACK: ::DWORD = 0x00000040;
pub const STARTF_FORCEOFFFEEDBACK: ::DWORD = 0x00000080;
pub const STARTF_USESTDHANDLES: ::DWORD = 0x00000100;
pub const STARTF_USEHOTKEY: ::DWORD = 0x00000200;
pub const STARTF_TITLEISLINKNAME: ::DWORD = 0x00000800;
pub const STARTF_TITLEISAPPID: ::DWORD = 0x00001000;
pub const STARTF_PREVENTPINNING: ::DWORD = 0x00002000;
//5454
pub const MOVEFILE_REPLACE_EXISTING: ::DWORD = 0x00000001;
pub const MOVEFILE_COPY_ALLOWED: ::DWORD = 0x00000002;
pub const MOVEFILE_DELAY_UNTIL_REBOOT: ::DWORD = 0x00000004;
pub const MOVEFILE_WRITE_THROUGH: ::DWORD = 0x00000008;
pub const MOVEFILE_CREATE_HARDLINK: ::DWORD = 0x00000010;
pub const MOVEFILE_FAIL_IF_NOT_TRACKABLE: ::DWORD = 0x00000020;
pub const FORMAT_MESSAGE_IGNORE_INSERTS: ::DWORD = 0x00000200;
pub const FORMAT_MESSAGE_FROM_STRING: ::DWORD = 0x00000400;
pub const FORMAT_MESSAGE_FROM_HMODULE: ::DWORD = 0x00000800;
pub const FORMAT_MESSAGE_FROM_SYSTEM: ::DWORD = 0x00001000;
pub const FORMAT_MESSAGE_ARGUMENT_ARRAY: ::DWORD = 0x00002000;
pub const FORMAT_MESSAGE_MAX_WIDTH_MASK: ::DWORD = 0x000000FF;
pub const FORMAT_MESSAGE_ALLOCATE_BUFFER: ::DWORD = 0x00000100;
pub const DEBUG_PROCESS: ::DWORD = 0x00000001;
pub const DEBUG_ONLY_THIS_PROCESS: ::DWORD = 0x00000002;
pub const CREATE_SUSPENDED: ::DWORD = 0x00000004;
pub const DETACHED_PROCESS: ::DWORD = 0x00000008;
pub const CREATE_NEW_CONSOLE: ::DWORD = 0x00000010;
pub const NORMAL_PRIORITY_CLASS: ::DWORD = 0x00000020;
pub const IDLE_PRIORITY_CLASS: ::DWORD = 0x00000040;
pub const HIGH_PRIORITY_CLASS: ::DWORD = 0x00000080;
pub const REALTIME_PRIORITY_CLASS: ::DWORD = 0x00000100;
pub const CREATE_NEW_PROCESS_GROUP: ::DWORD = 0x00000200;
pub const CREATE_UNICODE_ENVIRONMENT: ::DWORD = 0x00000400;
pub const CREATE_SEPARATE_WOW_VDM: ::DWORD = 0x00000800;
pub const CREATE_SHARED_WOW_VDM: ::DWORD = 0x00001000;
pub const CREATE_FORCEDOS: ::DWORD = 0x00002000;
pub const BELOW_NORMAL_PRIORITY_CLASS: ::DWORD = 0x00004000;
pub const ABOVE_NORMAL_PRIORITY_CLASS: ::DWORD = 0x00008000;
pub const INHERIT_PARENT_AFFINITY: ::DWORD = 0x00010000;
pub const INHERIT_CALLER_PRIORITY: ::DWORD = 0x00020000;
pub const CREATE_PROTECTED_PROCESS: ::DWORD = 0x00040000;
pub const EXTENDED_STARTUPINFO_PRESENT: ::DWORD = 0x00080000;
pub const PROCESS_MODE_BACKGROUND_BEGIN: ::DWORD = 0x00100000;
pub const PROCESS_MODE_BACKGROUND_END: ::DWORD = 0x00200000;
pub const CREATE_BREAKAWAY_FROM_JOB: ::DWORD = 0x01000000;
pub const CREATE_PRESERVE_CODE_AUTHZ_LEVEL: ::DWORD = 0x02000000;
pub const CREATE_DEFAULT_ERROR_MODE: ::DWORD = 0x04000000;
pub const CREATE_NO_WINDOW: ::DWORD = 0x08000000;
pub const PROFILE_USER: ::DWORD = 0x10000000;
pub const PROFILE_KERNEL: ::DWORD = 0x20000000;
pub const PROFILE_SERVER: ::DWORD = 0x40000000;
pub const CREATE_IGNORE_SYSTEM_DEFAULT: ::DWORD = 0x80000000;
pub const THREAD_BASE_PRIORITY_LOWRT: ::DWORD = 15;
pub const THREAD_BASE_PRIORITY_MAX: ::DWORD = 2;
pub const THREAD_BASE_PRIORITY_MIN: ::DWORD = -2i32 as ::DWORD;
pub const THREAD_BASE_PRIORITY_IDLE: ::DWORD = -15i32 as ::DWORD;
pub const THREAD_PRIORITY_LOWEST: ::DWORD = THREAD_BASE_PRIORITY_MIN;
pub const THREAD_PRIORITY_BELOW_NORMAL: ::DWORD = THREAD_PRIORITY_LOWEST + 1;
pub const THREAD_PRIORITY_NORMAL: ::DWORD = 0;
pub const THREAD_PRIORITY_HIGHEST: ::DWORD = THREAD_BASE_PRIORITY_MAX;
pub const THREAD_PRIORITY_ABOVE_NORMAL: ::DWORD = THREAD_PRIORITY_HIGHEST - 1;
pub const THREAD_PRIORITY_ERROR_RETURN: ::DWORD = 0x7fffffff;
pub const THREAD_PRIORITY_TIME_CRITICAL: ::DWORD = THREAD_BASE_PRIORITY_LOWRT;
pub const THREAD_PRIORITY_IDLE: ::DWORD = THREAD_BASE_PRIORITY_IDLE;
pub const THREAD_MODE_BACKGROUND_BEGIN: ::DWORD = 0x00010000;
pub const THREAD_MODE_BACKGROUND_END: ::DWORD = 0x00020000;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct MEMORYSTATUS {
    pub dwLength: ::DWORD,
    pub dwMemoryLoad: ::DWORD,
    pub dwTotalPhys: ::SIZE_T,
    pub dwAvailPhys: ::SIZE_T,
    pub dwTotalPageFile: ::SIZE_T,
    pub dwAvailPageFile: ::SIZE_T,
    pub dwTotalVirtual: ::SIZE_T,
    pub dwAvailVirtual: ::SIZE_T,
}
pub type LPMEMORYSTATUS = *mut MEMORYSTATUS;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct COMMPROP {
    pub wPacketLength: ::WORD,
    pub wPacketVersion: ::WORD,
    pub dwServiceMask: ::DWORD,
    pub dwReserved1: ::DWORD,
    pub dwMaxTxQueue: ::DWORD,
    pub dwMaxRxQueue: ::DWORD,
    pub dwMaxBaud: ::DWORD,
    pub dwProvSubType: ::DWORD,
    pub dwProvCapabilities: ::DWORD,
    pub dwSettableParams: ::DWORD,
    pub dwSettableBaud: ::DWORD,
    pub wSettableData: ::WORD,
    pub wSettableStopParity: ::WORD,
    pub dwCurrentTxQueue: ::DWORD,
    pub dwCurrentRxQueue: ::DWORD,
    pub dwProvSpec1: ::DWORD,
    pub dwProvSpec2: ::DWORD,
    pub wcProvChar: [::WCHAR; 1],
}
pub type LPCOMMPROP = *mut COMMPROP;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct COMSTAT {
    pub BitFields: ::DWORD,
    pub cbInQue: ::DWORD,
    pub cbOutQue : ::DWORD,
}
BITFIELD!(COMSTAT BitFields: ::DWORD [
    fCtsHold set_fCtsHold[0..1],
    fDsrHold set_fDsrHold[1..2],
    fRlsdHold set_fRlsdHold[2..3],
    fXoffHold set_fXoffHold[3..4],
    fXoffSent set_fXoffSent[4..5],
    fEof set_fEof[5..6],
    fTxim set_fTxim[6..7],
    fReserved set_fReserved[7..32],
]);
pub type LPCOMSTAT = *mut COMSTAT;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DCB {
    pub DCBlength: ::DWORD,
    pub BaudRate: ::DWORD,
    pub BitFields: ::DWORD,
    pub wReserved: ::WORD,
    pub XonLim: ::WORD,
    pub XoffLim: ::WORD,
    pub ByteSize: ::BYTE,
    pub Parity: ::BYTE,
    pub StopBits: ::BYTE,
    pub XonChar: ::c_char,
    pub XoffChar: ::c_char,
    pub ErrorChar: ::c_char,
    pub EofChar: ::c_char,
    pub EvtChar: ::c_char,
    pub wReserved1: ::WORD,
}
BITFIELD!(DCB BitFields: ::DWORD [
    fBinary set_fBinary[0..1],
    fParity set_fParity[1..2],
    fOutxCtsFlow set_fOutxCtsFlow[2..3],
    fOutxDsrFlow set_fOutxDsrFlow[3..4],
    fDtrControl set_fDtrControl[4..6],
    fDsrSensitivity set_fDsrSensitivity[6..7],
    fTXContinueOnXoff set_fTXContinueOnXoff[7..8],
    fOutX set_fOutX[8..9],
    fInX set_fInX[9..10],
    fErrorChar set_fErrorChar[10..11],
    fNull set_fNull[11..12],
    fRtsControl set_fRtsControl[12..14],
    fAbortOnError set_fAbortOnError[14..15],
    fDummy2 set_fDummy2[15..32],
]);
pub type LPDCB = *mut DCB;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct COMMTIMEOUTS {
    pub ReadIntervalTimeout: ::DWORD,
    pub ReadTotalTimeoutMultiplier: ::DWORD,
    pub ReadTotalTimeoutConstant: ::DWORD,
    pub WriteTotalTimeoutMultiplier: ::DWORD,
    pub WriteTotalTimeoutConstant: ::DWORD,
}
pub type LPCOMMTIMEOUTS = *mut COMMTIMEOUTS;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct COMMCONFIG {
    pub dwSize: ::DWORD,
    pub wVersion: ::WORD,
    pub wReserved: ::WORD,
    pub dcb: DCB,
    pub dwProviderSubType: ::DWORD,
    pub dwProviderOffset: ::DWORD,
    pub dwProviderSize: ::DWORD,
    pub wcProviderData: [::WCHAR; 1],
}
pub type LPCOMMCONFIG = *mut COMMCONFIG;
pub type PFIBER_CALLOUT_ROUTINE = Option<unsafe extern "system" fn(
    lpParameter: ::LPVOID,
) -> ::LPVOID>;
pub type LPLDT_ENTRY = ::LPVOID;
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum COPYFILE2_MESSAGE_TYPE {
    COPYFILE2_CALLBACK_NONE = 0,
    COPYFILE2_CALLBACK_CHUNK_STARTED,
    COPYFILE2_CALLBACK_CHUNK_FINISHED,
    COPYFILE2_CALLBACK_STREAM_STARTED,
    COPYFILE2_CALLBACK_STREAM_FINISHED,
    COPYFILE2_CALLBACK_POLL_CONTINUE,
    COPYFILE2_CALLBACK_ERROR,
    COPYFILE2_CALLBACK_MAX,
}
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum COPYFILE2_MESSAGE_ACTION {
    COPYFILE2_PROGRESS_CONTINUE = 0,
    COPYFILE2_PROGRESS_CANCEL,
    COPYFILE2_PROGRESS_STOP,
    COPYFILE2_PROGRESS_QUIET,
    COPYFILE2_PROGRESS_PAUSE,
}
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum COPYFILE2_COPY_PHASE {
    COPYFILE2_PHASE_NONE = 0,
    COPYFILE2_PHASE_PREPARE_SOURCE,
    COPYFILE2_PHASE_PREPARE_DEST,
    COPYFILE2_PHASE_READ_SOURCE,
    COPYFILE2_PHASE_WRITE_DESTINATION,
    COPYFILE2_PHASE_SERVER_COPY,
    COPYFILE2_PHASE_NAMEGRAFT_COPY,
    COPYFILE2_PHASE_MAX,
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct COPYFILE2_MESSAGE_ChunkStarted {
    pub dwStreamNumber: ::DWORD,
    pub dwReserved: ::DWORD,
    pub hSourceFile: ::HANDLE,
    pub hDestinationFile: ::HANDLE,
    pub uliChunkNumber: ::ULARGE_INTEGER,
    pub uliChunkSize: ::ULARGE_INTEGER,
    pub uliStreamSize: ::ULARGE_INTEGER,
    pub uliTotalFileSize: ::ULARGE_INTEGER,
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct COPYFILE2_MESSAGE_ChunkFinished {
    pub dwStreamNumber: ::DWORD,
    pub dwFlags: ::DWORD,
    pub hSourceFile: ::HANDLE,
    pub hDestinationFile: ::HANDLE,
    pub uliChunkNumber: ::ULARGE_INTEGER,
    pub uliChunkSize: ::ULARGE_INTEGER,
    pub uliStreamSize: ::ULARGE_INTEGER,
    pub uliStreamBytesTransferred: ::ULARGE_INTEGER,
    pub uliTotalFileSize: ::ULARGE_INTEGER,
    pub uliTotalBytesTransferred: ::ULARGE_INTEGER,
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct COPYFILE2_MESSAGE_StreamStarted {
    pub dwStreamNumber: ::DWORD,
    pub dwReserved: ::DWORD,
    pub hSourceFile: ::HANDLE,
    pub hDestinationFile: ::HANDLE,
    pub uliStreamSize: ::ULARGE_INTEGER,
    pub uliTotalFileSize: ::ULARGE_INTEGER,
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct COPYFILE2_MESSAGE_StreamFinished {
    pub dwStreamNumber: ::DWORD,
    pub dwReserved: ::DWORD,
    pub hSourceFile: ::HANDLE,
    pub hDestinationFile: ::HANDLE,
    pub uliStreamSize: ::ULARGE_INTEGER,
    pub uliStreamBytesTransferred: ::ULARGE_INTEGER,
    pub uliTotalFileSize: ::ULARGE_INTEGER,
    pub uliTotalBytesTransferred: ::ULARGE_INTEGER,
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct COPYFILE2_MESSAGE_PollContinue {
    pub dwReserved: ::DWORD,
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct COPYFILE2_MESSAGE_Error {
    pub CopyPhase: COPYFILE2_COPY_PHASE,
    pub dwStreamNumber: ::DWORD,
    pub hrFailure: ::HRESULT,
    pub dwReserved: ::DWORD,
    pub uliChunkNumber: ::ULARGE_INTEGER,
    pub uliStreamSize: ::ULARGE_INTEGER,
    pub uliStreamBytesTransferred: ::ULARGE_INTEGER,
    pub uliTotalFileSize: ::ULARGE_INTEGER,
    pub uliTotalBytesTransferred: ::ULARGE_INTEGER,
}
#[repr(C)] #[derive(Copy)]
pub struct COPYFILE2_MESSAGE {
    pub Type: COPYFILE2_MESSAGE_TYPE,
    pub dwPadding: ::DWORD,
    #[cfg(target_arch="x86")]
    pub Info: [u8; 64],
    #[cfg(target_arch="x86_64")]
    pub Info: [u8; 72],
}
impl Clone for COPYFILE2_MESSAGE { fn clone(&self) -> COPYFILE2_MESSAGE { *self } }
UNION!(COPYFILE2_MESSAGE, Info, ChunkStarted, ChunkStarted_mut, COPYFILE2_MESSAGE_ChunkStarted);
UNION!(COPYFILE2_MESSAGE, Info, ChunkFinished, ChunkFinished_mut, COPYFILE2_MESSAGE_ChunkFinished);
UNION!(COPYFILE2_MESSAGE, Info, StreamStarted, StreamStarted_mut, COPYFILE2_MESSAGE_StreamStarted);
UNION!(
    COPYFILE2_MESSAGE, Info, StreamFinished, StreamFinished_mut, COPYFILE2_MESSAGE_StreamFinished
);
UNION!(COPYFILE2_MESSAGE, Info, PollContinue, PollContinue_mut, COPYFILE2_MESSAGE_PollContinue);
UNION!(COPYFILE2_MESSAGE, Info, Error, Error_mut, COPYFILE2_MESSAGE_Error);
pub type PCOPYFILE2_PROGRESS_ROUTINE = Option<unsafe extern "system" fn(
    pMessage: *const COPYFILE2_MESSAGE, pvCallbackContext: ::PVOID,
) -> COPYFILE2_MESSAGE_ACTION>;
#[repr(C)] #[derive(Copy)]
pub struct COPYFILE2_EXTENDED_PARAMETERS {
    pub dwSize: ::DWORD,
    pub dwCopyFlags: ::DWORD,
    pub pfCancel: *mut ::BOOL,
    pub pProgressRoutine: PCOPYFILE2_PROGRESS_ROUTINE,
    pub pvCallbackContext: ::PVOID,
}
impl Clone for COPYFILE2_EXTENDED_PARAMETERS {
    fn clone(&self) -> COPYFILE2_EXTENDED_PARAMETERS { *self }
}
pub type LPPROGRESS_ROUTINE = Option<unsafe extern "system" fn(
    TotalFileSize: ::LARGE_INTEGER, TotalBytesTransferred: ::LARGE_INTEGER,
    StreamSize: ::LARGE_INTEGER, StreamBytesTransferred: ::LARGE_INTEGER, dwStreamNumber: ::DWORD,
    dwCallbackReason: ::DWORD, hSourceFile: ::HANDLE, hDestinationFile: ::HANDLE, lpData: ::LPVOID,
) -> ::DWORD>;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ACTCTXA {
    pub cbSize: ::ULONG,
    pub dwFlags: ::DWORD,
    pub lpSource: ::LPCSTR,
    pub wProcessorArchitecture: ::USHORT,
    pub wLangId: ::LANGID,
    pub lpAssemblyDirectory: ::LPCSTR,
    pub lpResourceName: ::LPCSTR,
    pub lpApplicationName: ::LPCSTR,
    pub hModule: ::HMODULE,
}
pub type PACTCTXA = *mut ACTCTXA;
pub type PCACTCTXA = *const ACTCTXA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ACTCTXW {
    pub cbSize: ::ULONG,
    pub dwFlags: ::DWORD,
    pub lpSource: ::LPCWSTR,
    pub wProcessorArchitecture: ::USHORT,
    pub wLangId: ::LANGID,
    pub lpAssemblyDirectory: ::LPCWSTR,
    pub lpResourceName: ::LPCWSTR,
    pub lpApplicationName: ::LPCWSTR,
    pub hModule: ::HMODULE,
}
pub type PACTCTXW = *mut ACTCTXW;
pub type PCACTCTXW = *const ACTCTXW;
pub type PFIBER_START_ROUTINE = Option<unsafe extern "system" fn(lpFiberParameter: ::LPVOID)>;
pub type LPFIBER_START_ROUTINE = PFIBER_START_ROUTINE;
pub type PUMS_CONTEXT = *mut ::c_void;
pub type PUMS_COMPLETION_LIST = *mut ::c_void;
pub type UMS_THREAD_INFO_CLASS = ::RTL_UMS_THREAD_INFO_CLASS;
pub type PUMS_THREAD_INFO_CLASS = *mut UMS_THREAD_INFO_CLASS;
pub type PUMS_SCHEDULER_ENTRY_POINT = ::PRTL_UMS_SCHEDULER_ENTRY_POINT;
#[repr(C)] #[derive(Copy)]
pub struct UMS_SCHEDULER_STARTUP_INFO {
    pub UmsVersion: ::ULONG,
    pub CompletionList: PUMS_COMPLETION_LIST,
    pub SchedulerProc: PUMS_SCHEDULER_ENTRY_POINT,
    pub SchedulerParam: ::PVOID,
}
impl Clone for UMS_SCHEDULER_STARTUP_INFO {
    fn clone(&self) -> UMS_SCHEDULER_STARTUP_INFO { *self }
}
pub type PUMS_SCHEDULER_STARTUP_INFO = *mut UMS_SCHEDULER_STARTUP_INFO;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct UMS_SYSTEM_THREAD_INFORMATION {
    pub UmsVersion: ::ULONG,
    pub BitFields: ::ULONG,
}
BITFIELD!(UMS_SYSTEM_THREAD_INFORMATION BitFields: ::ULONG [
    IsUmsSchedulerThread set_IsUmsSchedulerThread[0..1],
    IsUmsWorkerThread set_IsUmsWorkerThread[1..2],
]);
UNION!(
    UMS_SYSTEM_THREAD_INFORMATION, BitFields, ThreadUmsFlags, ThreadUmsFlags_mut,
    ::ULONG
);
pub type PUMS_SYSTEM_THREAD_INFORMATION = *mut UMS_SYSTEM_THREAD_INFORMATION;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA {
    pub lpInformation: ::PVOID,
    pub lpSectionBase: ::PVOID,
    pub ulSectionLength: ::ULONG,
    pub lpSectionGlobalDataBase: ::PVOID,
    pub ulSectionGlobalDataLength: ::ULONG,
}
pub type PACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA =
    *mut ACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA;
pub type PCACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA =
    *const ACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ACTCTX_SECTION_KEYED_DATA {
    pub cbSize: ::ULONG,
    pub ulDataFormatVersion: ::ULONG,
    pub lpData: ::PVOID,
    pub ulLength: ::ULONG,
    pub lpSectionGlobalData: ::PVOID,
    pub ulSectionGlobalDataLength: ::ULONG,
    pub lpSectionBase: ::PVOID,
    pub ulSectionTotalLength: ::ULONG,
    pub hActCtx: ::HANDLE,
    pub ulAssemblyRosterIndex: ::ULONG,
    pub ulFlags: ::ULONG,
    pub AssemblyMetadata: ACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA,
}
pub type PACTCTX_SECTION_KEYED_DATA = *mut ACTCTX_SECTION_KEYED_DATA;
pub type PCACTCTX_SECTION_KEYED_DATA = *const ACTCTX_SECTION_KEYED_DATA;
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum STREAM_INFO_LEVELS {
    FindStreamInfoStandard,
    FindStreamInfoMaxInfoLevel,
}
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum PROCESS_INFORMATION_CLASS {
    ProcessMemoryPriority,
    ProcessInformationClassMax,
}
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum DEP_SYSTEM_POLICY_TYPE {
    DEPPolicyAlwaysOff = 0,
    DEPPolicyAlwaysOn,
    DEPPolicyOptIn,
    DEPPolicyOptOut,
    DEPTotalPolicyCount,
}
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum PIPE_ATTRIBUTE_TYPE {
    PipeAttribute,
    PipeConnectionAttribute,
    PipeHandleAttribute,
}
pub type APPLICATION_RECOVERY_CALLBACK = Option<unsafe extern "system" fn(
    pvParameter: ::PVOID
) -> ::DWORD>;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SYSTEM_POWER_STATUS {
    pub ACLineStatus: ::BYTE,
    pub BatteryFlag: ::BYTE,
    pub BatteryLifePercent: ::BYTE,
    pub Reserved1: ::BYTE,
    pub BatteryLifeTime: ::DWORD,
    pub BatteryFullLifeTime: ::DWORD,
}
pub type LPSYSTEM_POWER_STATUS = *mut SYSTEM_POWER_STATUS;
pub const OFS_MAXPATHNAME: usize = 128;
#[repr(C)] #[derive(Copy)]
pub struct OFSTRUCT {
    pub cBytes: ::BYTE,
    pub fFixedDisk: ::BYTE,
    pub nErrCode: ::WORD,
    pub Reserved1: ::WORD,
    pub Reserved2: ::WORD,
    pub szPathName: [::CHAR; OFS_MAXPATHNAME],
}
impl Clone for OFSTRUCT { fn clone(&self) -> OFSTRUCT { *self } }
pub type POFSTRUCT = *mut OFSTRUCT;
pub type LPOFSTRUCT = *mut OFSTRUCT;
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum FILE_ID_TYPE {
    FileIdType,
    ObjectIdType,
    ExtendedFileIdType,
    MaximumFileIdType,
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct FILE_ID_DESCRIPTOR {
    pub dwSize: ::DWORD,
    pub Type: FILE_ID_TYPE,
    pub ObjectId: ::GUID,
}
UNION!(FILE_ID_DESCRIPTOR, ObjectId, FileId, FileId_mut, ::LARGE_INTEGER);
UNION!(FILE_ID_DESCRIPTOR, ObjectId, ExtendedFileId, ExtendedFileId_mut, ::FILE_ID_128);
pub type LPFILE_ID_DESCRIPTOR = *mut FILE_ID_DESCRIPTOR;

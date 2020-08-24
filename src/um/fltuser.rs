// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::minwindef::{DWORD, LPCVOID, LPDWORD, LPHANDLE, LPVOID, ULONG, USHORT, WORD};
use shared::ntdef::NTSTATUS;
use um::minwinbase::{LPOVERLAPPED, LPSECURITY_ATTRIBUTES};
use um::winnt::{HANDLE, HRESULT, LPCWSTR, LPWSTR, PHANDLE, ULONGLONG, WCHAR};
pub type HFILTER = HANDLE;
pub type PHFILTER = *mut HFILTER;
pub type HFILTER_INSTANCE = HANDLE;
pub type PHFILTER_INSTANCE = *mut HFILTER_INSTANCE;
pub type HFILTER_VOLUME = HANDLE;
pub const FLTFL_VSI_DETACHED_VOLUME: ULONG = 0x00000001;
pub const FLTFL_AGGREGATE_INFO_IS_MINIFILTER: ULONG = 0x00000001;
pub const FLTFL_AGGREGATE_INFO_IS_LEGACYFILTER: ULONG = 0x00000002;
pub const FLTFL_ASI_IS_MINIFILTER: ULONG = 0x00000001;
pub const FLTFL_ASI_IS_LEGACYFILTER: ULONG = 0x00000002;
pub const FLTFL_IASI_IS_MINIFILTER: ULONG = 0x00000001;
pub const FLTFL_IASI_IS_LEGACYFILTER: ULONG = 0x00000002;
pub const FLTFL_IASIM_DETACHED_VOLUME: ULONG = 0x00000001;
pub const FLTFL_IASIL_DETACHED_VOLUME: ULONG = 0x00000001;
pub const FLT_PORT_FLAG_SYNC_HANDLE: DWORD = 0x00000001;
ENUM!{enum FLT_FILESYSTEM_TYPE {
    FLT_FSTYPE_UNKNOWN,         //an UNKNOWN file system type
    FLT_FSTYPE_RAW,             //Microsoft's RAW file system       (\FileSystem\RAW)
    FLT_FSTYPE_NTFS,            //Microsoft's NTFS file system      (\FileSystem\Ntfs)
    FLT_FSTYPE_FAT,             //Microsoft's FAT file system       (\FileSystem\Fastfat)
    FLT_FSTYPE_CDFS,            //Microsoft's CDFS file system      (\FileSystem\Cdfs)
    FLT_FSTYPE_UDFS,            //Microsoft's UDFS file system      (\FileSystem\Udfs)
    FLT_FSTYPE_LANMAN,          //Microsoft's LanMan Redirector     (\FileSystem\MRxSmb)
    FLT_FSTYPE_WEBDAV,          //Microsoft's WebDav redirector     (\FileSystem\MRxDav)
    FLT_FSTYPE_RDPDR,           //Microsoft's Terminal Server redirector    (\Driver\rdpdr)
    FLT_FSTYPE_NFS,             //Microsoft's NFS file system       (\FileSystem\NfsRdr)
    FLT_FSTYPE_MS_NETWARE,      //Microsoft's NetWare redirector    (\FileSystem\nwrdr)
    FLT_FSTYPE_NETWARE,         //Novell's NetWare redirector
    FLT_FSTYPE_BSUDF,           //The BsUDF CD-ROM driver           (\FileSystem\BsUDF)
    FLT_FSTYPE_MUP,             //Microsoft's Mup redirector        (\FileSystem\Mup)
    FLT_FSTYPE_RSFX,            //Microsoft's WinFS redirector      (\FileSystem\RsFxDrv)
    FLT_FSTYPE_ROXIO_UDF1,      //Roxio's UDF writeable file system (\FileSystem\cdudf_xp)
    FLT_FSTYPE_ROXIO_UDF2,      //Roxio's UDF readable file system  (\FileSystem\UdfReadr_xp)
    FLT_FSTYPE_ROXIO_UDF3,      //Roxio's DVD file system           (\FileSystem\DVDVRRdr_xp)
    FLT_FSTYPE_TACIT,           //Tacit FileSystem                  (\Device\TCFSPSE)
    FLT_FSTYPE_FS_REC,          //Microsoft's File system recognizer (\FileSystem\Fs_rec)
    FLT_FSTYPE_INCD,            //Nero's InCD file system           (\FileSystem\InCDfs)
    FLT_FSTYPE_INCD_FAT,        //Nero's InCD FAT file system       (\FileSystem\InCDFat)
    FLT_FSTYPE_EXFAT,           //Microsoft's EXFat FILE SYSTEM     (\FileSystem\exfat)
    FLT_FSTYPE_PSFS,            //PolyServ's file system            (\FileSystem\psfs)
    FLT_FSTYPE_GPFS,            //IBM General Parallel File System  (\FileSystem\gpfs)
    FLT_FSTYPE_NPFS,            //Microsoft's Named Pipe file system(\FileSystem\npfs)
    FLT_FSTYPE_MSFS,            //Microsoft's Mailslot file system  (\FileSystem\msfs)
    FLT_FSTYPE_CSVFS,           //Microsoft's Cluster Shared Volume file system  (\FileSystem\csvfs)
    FLT_FSTYPE_REFS,            //Microsoft's ReFS file system      (\FileSystem\Refs or \FileSystem\Refsv1)
    FLT_FSTYPE_OPENAFS,         //OpenAFS file system               (\Device\AFSRedirector)
    FLT_FSTYPE_CIMFS,            //Composite Image file system       (\FileSystem\cimfs)
}}
ENUM!{enum FILTER_INFORMATION_CLASS {
    FilterFullInformation = 0,
    FilterAggregateBasicInformation,
    FilterAggregateStandardInformation,
}}
ENUM!{enum FILTER_VOLUME_INFORMATION_CLASS {
    FilterVolumeBasicInformation = 0,
    FilterVolumeStandardInformation,
}}
ENUM!{enum INSTANCE_INFORMATION_CLASS {
    InstanceBasicInformation = 0,
    InstancePartialInformation,
    InstanceFullInformation,
    InstanceAggregateStandardInformation,
}}
STRUCT!{struct FILTER_FULL_INFORMATION {
    NextEntryOffset: ULONG,
    FrameID: ULONG,
    NumberOfInstances: ULONG,
    FilterNameLength: USHORT,
    FilterNameBuffer: [WCHAR; 1],
}}
STRUCT!{struct FILTER_AGGREGATE_BASIC_INFORMATION_u_s_MiniFilter {
    FrameID: ULONG,
    NumberOfInstances: ULONG,
    FilterNameLength: USHORT,
    FilterNameBufferOffset: USHORT,
    FilterAltitudeLength: USHORT,
    FilterAltitudeBufferOffset: USHORT,
}}
STRUCT!{struct FILTER_AGGREGATE_BASIC_INFORMATION_u_s_LegacyFilter {
    FilterNameLength: USHORT,
    FilterNameBufferOffset: USHORT,
}}
UNION!{union FILTER_AGGREGATE_BASIC_INFORMATION_Type_u {
    [u32; 4],
    MiniFilter mut_MiniFilter: FILTER_AGGREGATE_BASIC_INFORMATION_u_s_MiniFilter,
    LegacyFilter mut_LegacyFilter: FILTER_AGGREGATE_BASIC_INFORMATION_u_s_LegacyFilter,
}}
STRUCT!{struct FILTER_AGGREGATE_BASIC_INFORMATION {
    NextEntryOffset: ULONG,
    Flags: ULONG,
    Type: FILTER_AGGREGATE_BASIC_INFORMATION_Type_u,
}}
STRUCT!{struct FILTER_AGGREGATE_STANDARD_INFORMATION_u_s_MiniFilter {
    Flags: ULONG,
    FrameID: ULONG,
    NumberOfInstances: ULONG,
    FilterNameLength: USHORT,
    FilterNameBufferOffset: USHORT,
    FilterAltitudeLength: USHORT,
    FilterAltitudeBufferOffset: USHORT,
}}
STRUCT!{struct FILTER_AGGREGATE_STANDARD_INFORMATION_u_s_LegacyFilter {
    Flags: ULONG,
    FilterNameLength: USHORT,
    FilterNameBufferOffset: USHORT,
    FilterAltitudeLength: USHORT,
    FilterAltitudeBufferOffset: USHORT,
}}
UNION!{union FILTER_AGGREGATE_STANDARD_INFORMATION_Type_u {
    [u32; 5],
    MiniFilter mut_MiniFilter: FILTER_AGGREGATE_STANDARD_INFORMATION_u_s_MiniFilter,
    LegacyFilter mut_LegacyFilter: FILTER_AGGREGATE_STANDARD_INFORMATION_u_s_LegacyFilter,
}}
STRUCT!{struct FILTER_AGGREGATE_STANDARD_INFORMATION {
    NextEntryOffset: ULONG,
    Flags: ULONG,
    Type: FILTER_AGGREGATE_STANDARD_INFORMATION_Type_u,
}}
STRUCT!{struct FILTER_VOLUME_BASIC_INFORMATION {
    FilterVolumeNameLength: USHORT,
    FilterVolumeName: [WCHAR; 1],
 }}
 STRUCT!{struct FILTER_VOLUME_STANDARD_INFORMATION {
    NextEntryOffset: ULONG,
    Flags: ULONG,
    FrameID: ULONG,
    FileSystemType: FLT_FILESYSTEM_TYPE,
    FilterVolumeNameLength: USHORT,
    FilterVolumeName: [WCHAR; 1],
 }}
STRUCT!{struct INSTANCE_BASIC_INFORMATION {
    NextEntryOffset: ULONG,
    InstanceNameLength: USHORT,
    InstanceNameBufferOffset: USHORT,
}}
STRUCT!{struct INSTANCE_PARTIAL_INFORMATION {
    NextEntryOffset: ULONG,
    InstanceNameLength: USHORT,
    InstanceNameBufferOffset: USHORT,
    AltitudeLength: USHORT,
    AltitudeBufferOffset: USHORT,
}}
STRUCT!{struct INSTANCE_FULL_INFORMATION {
    NextEntryOffset: ULONG,
    InstanceNameLength: USHORT,
    InstanceNameBufferOffset: USHORT,
    AltitudeLength: USHORT,
    AltitudeBufferOffset: USHORT,
    VolumeNameLength: USHORT,
    VolumeNameBufferOffset: USHORT,
    FilterNameLength: USHORT,
    FilterNameBufferOffset: USHORT,
}}
STRUCT!{struct INSTANCE_AGGREGATE_STANDARD_INFORMATION_u_s_MiniFilter {
    Flags: ULONG,
    FrameID: ULONG,
    VolumeFileSystemType: FLT_FILESYSTEM_TYPE,
    InstanceNameLength: USHORT,
    InstanceNameBufferOffset: USHORT,
    AltitudeLength: USHORT,
    AltitudeBufferOffset: USHORT,
    VolumeNameLength: USHORT,
    VolumeNameBufferOffset: USHORT,
    FilterNameLength: USHORT,
    FilterNameBufferOffset: USHORT,
    SupportedFeatures: ULONG,
}}
STRUCT!{struct INSTANCE_AGGREGATE_STANDARD_INFORMATION_u_s_LegacyFilter {
    Flags: ULONG,
    AltitudeLength: USHORT,
    AltitudeBufferOffset: USHORT,
    VolumeNameLength: USHORT,
    VolumeNameBufferOffset: USHORT,
    FilterNameLength: USHORT,
    FilterNameBufferOffset: USHORT,
    SupportedFeatures: ULONG,
}}
UNION!{union INSTANCE_AGGREGATE_STANDARD_INFORMATION_Type_u {
    [u32; 8],
    MiniFilter mut_MiniFilter: INSTANCE_AGGREGATE_STANDARD_INFORMATION_u_s_MiniFilter,
    LegacyFilter mut_LegacyFilter: INSTANCE_AGGREGATE_STANDARD_INFORMATION_u_s_LegacyFilter,
}}
STRUCT!{struct INSTANCE_AGGREGATE_STANDARD_INFORMATION {
    NextEntryOffset: ULONG,
    Flags: ULONG,
    Type: INSTANCE_AGGREGATE_STANDARD_INFORMATION_Type_u,
}}
STRUCT!{struct FILTER_MESSAGE_HEADER {
    ReplyLength: ULONG,
    MessageId: ULONGLONG,
}}
STRUCT!{struct FILTER_REPLY_HEADER {
    Status: NTSTATUS,
    MessageId: ULONGLONG,
}}
pub type PFILTER_MESSAGE_HEADER = *mut FILTER_MESSAGE_HEADER;
pub type PFILTER_REPLY_HEADER = *mut FILTER_REPLY_HEADER;
pub type PFILTER_FULL_INFORMATION = *mut FILTER_FULL_INFORMATION;
pub type PFILTER_AGGREGATE_BASIC_INFORMATION = *mut FILTER_AGGREGATE_BASIC_INFORMATION;
pub type PFILTER_AGGREGATE_STANDARD_INFORMATION = *mut FILTER_AGGREGATE_STANDARD_INFORMATION;
pub type PFILTER_VOLUME_BASIC_INFORMATION = *mut FILTER_VOLUME_BASIC_INFORMATION;
pub type PFILTER_VOLUME_STANDARD_INFORMATION = *mut FILTER_VOLUME_STANDARD_INFORMATION;
pub type PINSTANCE_BASIC_INFORMATION = *mut INSTANCE_BASIC_INFORMATION;
pub type PINSTANCE_PARTIAL_INFORMATION = *mut INSTANCE_PARTIAL_INFORMATION;
pub type PINSTANCE_FULL_INFORMATION = *mut INSTANCE_FULL_INFORMATION;
pub type PINSTANCE_AGGREGATE_STANDARD_INFORMATION = *mut INSTANCE_AGGREGATE_STANDARD_INFORMATION;
extern "system" {
    pub fn FilterAttach(
        lpFilterName: LPCWSTR,
        lpVolumeName: LPCWSTR,
        lpInstanceName: LPCWSTR,
        dwCreatedInstanceNameLength: DWORD,
        lpCreatedInstanceName: LPWSTR
    ) -> HRESULT;
    pub fn FilterAttachAtAltitude(
        lpFilterName: LPCWSTR,
        lpVolumeName: LPCWSTR,
        lpAltitude: LPCWSTR,
        lpInstanceName: LPCWSTR,
        dwCreatedInstanceNameLength: DWORD,
        lpCreatedInstanceName: LPWSTR
    ) -> HRESULT;
    pub fn FilterClose(
        hFilter: HFILTER
    ) -> HRESULT;
    pub fn FilterConnectCommunicationPort(
        lpPortName: LPCWSTR,
        dwOptions: DWORD,
        lpContext: LPCVOID,
        wSizeOfContext: WORD,
        lpSecurityAttributes: LPSECURITY_ATTRIBUTES,
        hPort: PHANDLE
    ) -> HRESULT;
    pub fn FilterCreate(
        lpFilterName: LPCWSTR,
        hFilter: PHFILTER
    ) -> HRESULT;
    pub fn FilterDetach(
        lpFilterName: LPCWSTR,
        lpVolumeName: LPCWSTR,
        lpInstanceName: LPCWSTR
    ) -> HRESULT;
    pub fn FilterFindClose(
        hFilterFind: HANDLE
    ) -> HRESULT;
    pub fn FilterFindFirst(
        dwInformationClass: FILTER_INFORMATION_CLASS,
        lpBuffer: LPVOID,
        dwBufferSize: DWORD,
        lpBytesReturned: LPDWORD,
        lpFilterFind: LPHANDLE
    ) -> HRESULT;
    pub fn FilterFindNext(
        hFilterFind: HANDLE,
        dwInformationClass: FILTER_INFORMATION_CLASS,
        lpBuffer: LPVOID,
        dwBufferSize: DWORD,
        lpBytesReturned: LPDWORD
    ) -> HRESULT;
    pub fn FilterGetDosName(
        lpVolumeName: LPCWSTR,
        lpDosName: LPWSTR,
        dwDosNameBufferSize: DWORD
    ) -> HRESULT;
    pub fn FilterGetInformation(
        hFilter: HFILTER,
        dwInformationClass: FILTER_INFORMATION_CLASS,
        lpBuffer: LPVOID,
        dwBufferSize: DWORD,
        lpBytesReturned: LPDWORD
    ) -> HRESULT;
    pub fn FilterGetMessage(
        hPort: HANDLE,
        lpMessageBuffer: PFILTER_MESSAGE_HEADER,
        dwMessageBufferSize: DWORD,
        lpOverlapped: LPOVERLAPPED
    ) -> HRESULT;
    pub fn FilterInstanceClose(
        hInstance: HFILTER_INSTANCE
    ) -> HRESULT;
    pub fn FilterInstanceCreate(
        lpFilterName: LPCWSTR,
        lpVolumeName: LPCWSTR,
        lpInstanceName: LPCWSTR,
        hInstance: PHFILTER_INSTANCE
    ) -> HRESULT;
    pub fn FilterInstanceFindClose(
        hFilterInstanceFind: HANDLE
    ) -> HRESULT;
    pub fn FilterInstanceFindFirst(
        lpFilterName: LPCWSTR,
        dwInformationClass: INSTANCE_INFORMATION_CLASS,
        lpBuffer: LPVOID,
        dwBufferSize: DWORD,
        lpBytesReturned: LPDWORD,
        lpFilterInstanceFind: LPHANDLE
    ) -> HRESULT;
    pub fn FilterInstanceFindNext(
        hFilterInstanceFind: HANDLE,
        dwInformationClass: INSTANCE_INFORMATION_CLASS,
        lpBuffer: LPVOID,
        dwBufferSize: DWORD,
        lpBytesReturned: LPDWORD
    ) -> HRESULT;
    pub fn FilterInstanceGetInformation(
        hInstance: HFILTER_INSTANCE,
        dwInformationClass: INSTANCE_INFORMATION_CLASS,
        lpBuffer: LPVOID,
        dwBufferSize: DWORD,
        lpBytesReturned: LPDWORD
    ) -> HRESULT;
    pub fn FilterLoad(
        lpFilterName: LPCWSTR
    ) -> HRESULT;
    pub fn FilterReplyMessage(
        hPort: HANDLE,
        lpReplyBuffer: PFILTER_REPLY_HEADER,
        dwReplyBufferSize: DWORD
    ) -> HRESULT;
    pub fn FilterSendMessage(
        hPort: HANDLE,
        lpInBuffer: LPVOID,
        dwInBufferSize: DWORD,
        lpOutBuffer: LPVOID,
        dwOutBufferSize: DWORD,
        lpBytesReturned: LPDWORD
    ) -> HRESULT;
    pub fn FilterUnload(
        lpFilterName: LPCWSTR
    ) -> HRESULT;
    // FilterVolumeClose@4 // ?????
    pub fn FilterVolumeFindClose(
        hVolumeFind: HANDLE
    ) -> HRESULT;
    pub fn FilterVolumeFindFirst(
        dwInformationClass: FILTER_VOLUME_INFORMATION_CLASS,
        lpBuffer: LPVOID,
        dwBufferSize: DWORD,
        lpBytesReturned: LPDWORD,
        lpVolumeFind: PHANDLE
    ) -> HRESULT;
    pub fn FilterVolumeFindNext(
        hVolumeFind: HANDLE,
        dwInformationClass: FILTER_VOLUME_INFORMATION_CLASS,
        lpBuffer: LPVOID,
        dwBufferSize: DWORD,
        lpBytesReturned: LPDWORD
    ) -> HRESULT;
    pub fn FilterVolumeInstanceFindClose(
        hVolumeInstanceFind: HANDLE
    ) -> HRESULT;
    pub fn FilterVolumeInstanceFindFirst(
        lpVolumeName: LPCWSTR,
        dwInformationClass: INSTANCE_INFORMATION_CLASS,
        lpBuffer: LPVOID,
        dwBufferSize: DWORD,
        lpBytesReturned: LPDWORD,
        lpVolumeInstanceFind: LPHANDLE
    ) -> HRESULT;
    pub fn FilterVolumeInstanceFindNext(
        hVolumeInstanceFind: HANDLE,
        dwInformationClass: INSTANCE_INFORMATION_CLASS,
        lpBuffer: LPVOID,
        dwBufferSize: DWORD,
        lpBytesReturned: LPDWORD
    ) -> HRESULT;
}

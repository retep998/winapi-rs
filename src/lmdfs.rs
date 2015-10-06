// Copyright © 2015, skdltmxn
// Licensed under the MIT License <LICENSE.md>
// This file contains structures, function prototypes, and definitions for the NetDfs API
pub const DFS_VOLUME_STATES: ::DWORD = 0xF;
pub const DFS_VOLUME_STATE_OK: ::DWORD = 1;
pub const DFS_VOLUME_STATE_INCONSISTENT: ::DWORD = 2;
pub const DFS_VOLUME_STATE_OFFLINE: ::DWORD = 3;
pub const DFS_VOLUME_STATE_ONLINE: ::DWORD = 4;
pub const DFS_VOLUME_STATE_RESYNCHRONIZE: ::DWORD = 0x10;
pub const DFS_VOLUME_STATE_STANDBY: ::DWORD = 0x20;
pub const DFS_VOLUME_STATE_FORCE_SYNC: ::DWORD = 0x40;
pub const DFS_VOLUME_FLAVORS: ::DWORD = 0x0300;
pub const DFS_VOLUME_FLAVOR_UNUSED1: ::DWORD = 0x0000;
pub const DFS_VOLUME_FLAVOR_STANDALONE: ::DWORD = 0x0100;
pub const DFS_VOLUME_FLAVOR_AD_BLOB: ::DWORD = 0x0200;
pub const DFS_STORAGE_FLAVOR_UNUSED2: ::DWORD = 0x0300;
pub const DFS_STORAGE_STATES: ::ULONG = 0xF;
pub const DFS_STORAGE_STATE_OFFLINE: ::ULONG = 1;
pub const DFS_STORAGE_STATE_ONLINE: ::ULONG = 2;
pub const DFS_STORAGE_STATE_ACTIVE: ::ULONG = 4;
ENUM!{enum DFS_TARGET_PRIORITY_CLASS {
    DfsInvalidPriorityClass = -1i32 as u32,
    DfsSiteCostNormalPriorityClass = 0,
    DfsGlobalHighPriorityClass,
    DfsSiteCostHighPriorityClass,
    DfsSiteCostLowPriorityClass,
    DfsGlobalLowPriorityClass,
}}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DFS_TARGET_PRIORITY {
    pub TargetPriorityClass: DFS_TARGET_PRIORITY_CLASS,
    pub TargetPriorityRank: ::USHORT,
    pub Reserved: ::USHORT,
}
pub type PDFS_TARGET_PRIORITY = *mut DFS_TARGET_PRIORITY;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DFS_INFO_1 {
    pub EntryPath: ::LPWSTR,
}
pub type PDFS_INFO_1 = *mut DFS_INFO_1;
pub type LPDFS_INFO_1 = *mut DFS_INFO_1;
#[cfg(target_arch="x86_64")] #[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DFS_INFO_1_32 {
    pub EntryPath: ::ULONG,
}
#[cfg(target_arch="x86_64")]
pub type PDFS_INFO_1_32 = *mut DFS_INFO_1_32;
#[cfg(target_arch="x86_64")]
pub type LPDFS_INFO_1_32 = *mut DFS_INFO_1_32;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DFS_INFO_2 {
    pub EntryPath: ::LPWSTR,
    pub Comment: ::LPWSTR,
    pub State: ::DWORD,
    pub NumberOfStorages: ::DWORD,
}
pub type PDFS_INFO_2 = *mut DFS_INFO_2;
pub type LPDFS_INFO_2 = *mut DFS_INFO_2;
#[cfg(target_arch="x86_64")] #[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DFS_INFO_2_32 {
    pub EntryPath: ::ULONG,
    pub Comment: ::ULONG,
    pub State: ::DWORD,
    pub NumberOfStorages: ::DWORD,
}
#[cfg(target_arch="x86_64")]
pub type PDFS_INFO_2_32 = *mut DFS_INFO_2_32;
#[cfg(target_arch="x86_64")]
pub type LPDFS_INFO_2_32 = *mut DFS_INFO_2_32;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DFS_STORAGE_INFO {
    pub State: ::ULONG,
    pub ServerName: ::LPWSTR,
    pub ShareName: ::LPWSTR,
}
pub type PDFS_STORAGE_INFO = *mut DFS_STORAGE_INFO;
pub type LPDFS_STORAGE_INFO = *mut DFS_STORAGE_INFO;
#[cfg(target_arch="x86_64")] #[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DFS_STORAGE_INFO_0_32 {
    pub State: ::ULONG,
    pub ServerName: ::ULONG,
    pub ShareName: ::ULONG,
}
#[cfg(target_arch="x86_64")]
pub type PDFS_STORAGE_INFO_0_32 = *mut DFS_STORAGE_INFO_0_32;
#[cfg(target_arch="x86_64")]
pub type LPDFS_STORAGE_INFO_0_32 = *mut DFS_STORAGE_INFO_0_32;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DFS_STORAGE_INFO_1 {
    pub State: ::ULONG,
    pub ServerName: ::LPWSTR,
    pub ShareName: ::LPWSTR,
    pub TargetPriority: DFS_TARGET_PRIORITY,
}
pub type PDFS_STORAGE_INFO_1 = *mut DFS_STORAGE_INFO_1;
pub type LPDFS_STORAGE_INFO_1 = *mut DFS_STORAGE_INFO_1;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DFS_INFO_3 {
    pub EntryPath: ::LPWSTR,
    pub Comment: ::LPWSTR,
    pub State: ::DWORD,
    pub NumberOfStorages: ::DWORD,
    pub Storage: LPDFS_STORAGE_INFO,
}
pub type PDFS_INFO_3 = *mut DFS_INFO_3;
pub type LPDFS_INFO_3 = *mut DFS_INFO_3;
#[cfg(target_arch="x86_64")] #[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DFS_INFO_3_32 {
    pub EntryPath: ::ULONG,
    pub Comment: ::ULONG,
    pub State: ::DWORD,
    pub NumberOfStorages: ::DWORD,
    pub Storage: ::ULONG,
}
#[cfg(target_arch="x86_64")]
pub type PDFS_INFO_3_32 = *mut DFS_INFO_3_32;
#[cfg(target_arch="x86_64")]
pub type LPDFS_INFO_3_32 = *mut DFS_INFO_3_32;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DFS_INFO_4 {
    pub EntryPath: ::LPWSTR,
    pub Comment: ::LPWSTR,
    pub State: ::DWORD,
    pub Timeout: ::ULONG,
    pub Guid: ::GUID,
    pub NumberOfStorages: ::DWORD,
    pub Storage: LPDFS_STORAGE_INFO,
}
pub type PDFS_INFO_4 = *mut DFS_INFO_4;
pub type LPDFS_INFO_4 = *mut DFS_INFO_4;
#[cfg(target_arch="x86_64")] #[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DFS_INFO_4_32 {
    pub EntryPath: ::ULONG,
    pub Comment: ::ULONG,
    pub State: ::DWORD,
    pub Timeout: ::ULONG,
    pub Guid: ::GUID,
    pub NumberOfStorages: ::DWORD,
    pub Storage: ::ULONG,
}
#[cfg(target_arch="x86_64")]
pub type PDFS_INFO_4_32 = *mut DFS_INFO_4_32;
#[cfg(target_arch="x86_64")]
pub type LPDFS_INFO_4_32 = *mut DFS_INFO_4_32;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DFS_INFO_5 {
    pub EntryPath: ::LPWSTR,
    pub Comment: ::LPWSTR,
    pub State: ::DWORD,
    pub Timeout: ::ULONG,
    pub Guid: ::GUID,
    pub PropertyFlags: ::ULONG,
    pub MetadataSize: ::ULONG,
    pub NumberOfStorages: ::DWORD,
}
pub type PDFS_INFO_5 = *mut DFS_INFO_5;
pub type LPDFS_INFO_5 = *mut DFS_INFO_5;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DFS_INFO_6 {
    pub EntryPath: ::LPWSTR,
    pub Comment: ::LPWSTR,
    pub State: ::DWORD,
    pub Timeout: ::ULONG,
    pub Guid: ::GUID,
    pub PropertyFlags: ::ULONG,
    pub MetadataSize: ::ULONG,
    pub NumberOfStorages: ::DWORD,
    pub Storage: LPDFS_STORAGE_INFO,
}
pub type PDFS_INFO_6 = *mut DFS_INFO_6;
pub type LPDFS_INFO_6 = *mut DFS_INFO_6;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DFS_INFO_7 {
    pub GenerationGuid: ::GUID,
}
pub type PDFS_INFO_7 = *mut DFS_INFO_7;
pub type LPDFS_INFO_7 = *mut DFS_INFO_7;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DFS_INFO_8 {
    pub EntryPath: ::LPWSTR,
    pub Comment: ::LPWSTR,
    pub State: ::DWORD,
    pub Timeout: ::ULONG,
    pub Guid: ::GUID,
    pub PropertyFlags: ::ULONG,
    pub MetadataSize: ::ULONG,
    pub SdLengthReserved: ::ULONG,
    pub pSecurityDescriptor: ::PSECURITY_DESCRIPTOR,
    pub NumberOfStorages: ::DWORD,
}
pub type PDFS_INFO_8 = *mut DFS_INFO_8;
pub type LPDFS_INFO_8 = *mut DFS_INFO_8;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DFS_INFO_9 {
    pub EntryPath: ::LPWSTR,
    pub Comment: ::LPWSTR,
    pub State: ::DWORD,
    pub Timeout: ::ULONG,
    pub Guid: ::GUID,
    pub PropertyFlags: ::ULONG,
    pub MetadataSize: ::ULONG,
    pub SdLengthReserved: ::ULONG,
    pub pSecurityDescriptor: ::PSECURITY_DESCRIPTOR,
    pub NumberOfStorages: ::DWORD,
    pub Storage: LPDFS_STORAGE_INFO,
}
pub type PDFS_INFO_9 = *mut DFS_INFO_9;
pub type LPDFS_INFO_9 = *mut DFS_INFO_9;
pub const DFS_PROPERTY_FLAG_INSITE_REFERRALS: ::ULONG = 0x00000001;
pub const DFS_PROPERTY_FLAG_ROOT_SCALABILITY: ::ULONG = 0x00000002;
pub const DFS_PROPERTY_FLAG_SITE_COSTING: ::ULONG = 0x00000004;
pub const DFS_PROPERTY_FLAG_TARGET_FAILBACK: ::ULONG = 0x00000008;
pub const DFS_PROPERTY_FLAG_CLUSTER_ENABLED: ::ULONG = 0x00000010;
pub const DFS_PROPERTY_FLAG_ABDE: ::ULONG = 0x00000020;
pub const DFS_VALID_PROPERTY_FLAGS: ::ULONG = DFS_PROPERTY_FLAG_INSITE_REFERRALS
    | DFS_PROPERTY_FLAG_ROOT_SCALABILITY | DFS_PROPERTY_FLAG_SITE_COSTING
    | DFS_PROPERTY_FLAG_TARGET_FAILBACK | DFS_PROPERTY_FLAG_CLUSTER_ENABLED
    | DFS_PROPERTY_FLAG_ABDE;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DFS_INFO_50 {
    pub NamespaceMajorVersion: ::ULONG,
    pub NamespaceMinorVersion: ::ULONG,
    pub NamespaceCapabilities: ::ULONGLONG,
}
pub type PDFS_INFO_50 = *mut DFS_INFO_50;
pub type LPDFS_INFO_50 = *mut DFS_INFO_50;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DFS_INFO_100 {
    pub Comment: ::LPWSTR,
}
pub type PDFS_INFO_100 = *mut DFS_INFO_100;
pub type LPDFS_INFO_100 = *mut DFS_INFO_100;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DFS_INFO_101 {
    pub State: ::DWORD,
}
pub type PDFS_INFO_101 = *mut DFS_INFO_101;
pub type LPDFS_INFO_101 = *mut DFS_INFO_101;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DFS_INFO_102 {
    pub Timeout: ::ULONG,
}
pub type PDFS_INFO_102 = *mut DFS_INFO_102;
pub type LPDFS_INFO_102 = *mut DFS_INFO_102;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DFS_INFO_103 {
    pub PropertyFlagMask: ::ULONG,
    pub PropertyFlags: ::ULONG,
}
pub type PDFS_INFO_103 = *mut DFS_INFO_103;
pub type LPDFS_INFO_103 = *mut DFS_INFO_103;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DFS_INFO_104 {
    pub TargetPriority: DFS_TARGET_PRIORITY,
}
pub type PDFS_INFO_104 = *mut DFS_INFO_104;
pub type LPDFS_INFO_104 = *mut DFS_INFO_104;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DFS_INFO_105 {
    pub Comment: ::LPWSTR,
    pub State: ::DWORD,
    pub Timeout: ::ULONG,
    pub PropertyFlagMask: ::ULONG,
    pub PropertyFlags: ::ULONG,
}
pub type PDFS_INFO_105 = *mut DFS_INFO_105;
pub type LPDFS_INFO_105 = *mut DFS_INFO_105;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DFS_INFO_106 {
    pub State: ::DWORD,
    pub TargetPriority: DFS_TARGET_PRIORITY,
}
pub type PDFS_INFO_106 = *mut DFS_INFO_106;
pub type LPDFS_INFO_106 = *mut DFS_INFO_106;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DFS_INFO_107 {
    pub Comment: ::LPWSTR,
    pub State: ::DWORD,
    pub Timeout: ::ULONG,
    pub PropertyFlagMask: ::ULONG,
    pub PropertyFlags: ::ULONG,
    pub SdLengthReserved: ::ULONG,
    pub pSecurityDescriptor: ::PSECURITY_DESCRIPTOR,
}
pub type PDFS_INFO_107 = *mut DFS_INFO_107;
pub type LPDFS_INFO_107 = *mut DFS_INFO_107;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DFS_INFO_150 {
    pub SdLengthReserved: ::ULONG,
    pub pSecurityDescriptor: ::PSECURITY_DESCRIPTOR,
}
pub type PDFS_INFO_150 = *mut DFS_INFO_150;
pub type LPDFS_INFO_150 = *mut DFS_INFO_150;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DFS_INFO_200 {
    pub FtDfsName: ::LPWSTR,
}
pub type PDFS_INFO_200 = *mut DFS_INFO_200;
pub type LPDFS_INFO_200 = *mut DFS_INFO_200;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DFS_INFO_300 {
    pub Flags: ::DWORD,
    pub DfsName: ::LPWSTR,
}
pub type PDFS_INFO_300 = *mut DFS_INFO_300;
pub type LPDFS_INFO_300 = *mut DFS_INFO_300;
pub const DFS_ADD_VOLUME: ::DWORD = 1;
pub const DFS_RESTORE_VOLUME: ::DWORD = 2;
pub const NET_DFS_SETDC_FLAGS: ::DWORD = 0x00000000;
pub const NET_DFS_SETDC_TIMEOUT: ::DWORD = 0x00000001;
pub const NET_DFS_SETDC_INITPKT: ::DWORD = 0x00000002;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DFS_SITENAME_INFO {
    pub SiteFlags: ::ULONG,
    pub SiteName: ::LPWSTR,
}
pub type PDFS_SITENAME_INFO = *mut DFS_SITENAME_INFO;
pub type LPDFS_SITENAME_INFO = *mut DFS_SITENAME_INFO;
pub const DFS_SITE_PRIMARY: ::ULONG = 0x1;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DFS_SITELIST_INFO {
    pub cSites: ::ULONG,
    pub Site: [DFS_SITENAME_INFO; 1],
}
pub type PDFS_SITELIST_INFO = *mut DFS_SITELIST_INFO;
pub type LPDFS_SITELIST_INFO = *mut DFS_SITELIST_INFO;
ENUM!{enum DFS_NAMESPACE_VERSION_ORIGIN {
    DFS_NAMESPACE_VERSION_ORIGIN_COMBINED = 0,
    DFS_NAMESPACE_VERSION_ORIGIN_SERVER,
    DFS_NAMESPACE_VERSION_ORIGIN_DOMAIN,
}}
pub type PDFS_NAMESPACE_VERSION_ORIGIN = *mut DFS_NAMESPACE_VERSION_ORIGIN;
pub const DFS_NAMESPACE_CAPABILITY_ABDE: ::ULONGLONG = 0x0000000000000001;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DFS_SUPPORTED_NAMESPACE_VERSION_INFO {
    pub DomainDfsMajorVersion: ::ULONG,
    pub DomainDfsMinorVersion: ::ULONG,
    pub DomainDfsCapabilities: ::ULONGLONG,
    pub StandaloneDfsMajorVersion: ::ULONG,
    pub StandaloneDfsMinorVersion: ::ULONG,
    pub StandaloneDfsCapabilities: ::ULONGLONG,
}
pub type PDFS_SUPPORTED_NAMESPACE_VERSION_INFO = *mut DFS_SUPPORTED_NAMESPACE_VERSION_INFO;

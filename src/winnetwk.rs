// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! Standard WINNET Header File for WIN32
pub const WNNC_NET_MSNET: ::DWORD = 0x00010000;
pub const WNNC_NET_SMB: ::DWORD = 0x00020000;
pub const WNNC_NET_NETWARE: ::DWORD = 0x00030000;
pub const WNNC_NET_VINES: ::DWORD = 0x00040000;
pub const WNNC_NET_10NET: ::DWORD = 0x00050000;
pub const WNNC_NET_LOCUS: ::DWORD = 0x00060000;
pub const WNNC_NET_SUN_PC_NFS: ::DWORD = 0x00070000;
pub const WNNC_NET_LANSTEP: ::DWORD = 0x00080000;
pub const WNNC_NET_9TILES: ::DWORD = 0x00090000;
pub const WNNC_NET_LANTASTIC: ::DWORD = 0x000A0000;
pub const WNNC_NET_AS400: ::DWORD = 0x000B0000;
pub const WNNC_NET_FTP_NFS: ::DWORD = 0x000C0000;
pub const WNNC_NET_PATHWORKS: ::DWORD = 0x000D0000;
pub const WNNC_NET_LIFENET: ::DWORD = 0x000E0000;
pub const WNNC_NET_POWERLAN: ::DWORD = 0x000F0000;
pub const WNNC_NET_BWNFS: ::DWORD = 0x00100000;
pub const WNNC_NET_COGENT: ::DWORD = 0x00110000;
pub const WNNC_NET_FARALLON: ::DWORD = 0x00120000;
pub const WNNC_NET_APPLETALK: ::DWORD = 0x00130000;
pub const WNNC_NET_INTERGRAPH: ::DWORD = 0x00140000;
pub const WNNC_NET_SYMFONET: ::DWORD = 0x00150000;
pub const WNNC_NET_CLEARCASE: ::DWORD = 0x00160000;
pub const WNNC_NET_FRONTIER: ::DWORD = 0x00170000;
pub const WNNC_NET_BMC: ::DWORD = 0x00180000;
pub const WNNC_NET_DCE: ::DWORD = 0x00190000;
pub const WNNC_NET_AVID: ::DWORD = 0x001A0000;
pub const WNNC_NET_DOCUSPACE: ::DWORD = 0x001B0000;
pub const WNNC_NET_MANGOSOFT: ::DWORD = 0x001C0000;
pub const WNNC_NET_SERNET: ::DWORD = 0x001D0000;
pub const WNNC_NET_RIVERFRONT1: ::DWORD = 0x001E0000;
pub const WNNC_NET_RIVERFRONT2: ::DWORD = 0x001F0000;
pub const WNNC_NET_DECORB: ::DWORD = 0x00200000;
pub const WNNC_NET_PROTSTOR: ::DWORD = 0x00210000;
pub const WNNC_NET_FJ_REDIR: ::DWORD = 0x00220000;
pub const WNNC_NET_DISTINCT: ::DWORD = 0x00230000;
pub const WNNC_NET_TWINS: ::DWORD = 0x00240000;
pub const WNNC_NET_RDR2SAMPLE: ::DWORD = 0x00250000;
pub const WNNC_NET_CSC: ::DWORD = 0x00260000;
pub const WNNC_NET_3IN1: ::DWORD = 0x00270000;
pub const WNNC_NET_EXTENDNET: ::DWORD = 0x00290000;
pub const WNNC_NET_STAC: ::DWORD = 0x002A0000;
pub const WNNC_NET_FOXBAT: ::DWORD = 0x002B0000;
pub const WNNC_NET_YAHOO: ::DWORD = 0x002C0000;
pub const WNNC_NET_EXIFS: ::DWORD = 0x002D0000;
pub const WNNC_NET_DAV: ::DWORD = 0x002E0000;
pub const WNNC_NET_KNOWARE: ::DWORD = 0x002F0000;
pub const WNNC_NET_OBJECT_DIRE: ::DWORD = 0x00300000;
pub const WNNC_NET_MASFAX: ::DWORD = 0x00310000;
pub const WNNC_NET_HOB_NFS: ::DWORD = 0x00320000;
pub const WNNC_NET_SHIVA: ::DWORD = 0x00330000;
pub const WNNC_NET_IBMAL: ::DWORD = 0x00340000;
pub const WNNC_NET_LOCK: ::DWORD = 0x00350000;
pub const WNNC_NET_TERMSRV: ::DWORD = 0x00360000;
pub const WNNC_NET_SRT: ::DWORD = 0x00370000;
pub const WNNC_NET_QUINCY: ::DWORD = 0x00380000;
pub const WNNC_NET_OPENAFS: ::DWORD = 0x00390000;
pub const WNNC_NET_AVID1: ::DWORD = 0x003A0000;
pub const WNNC_NET_DFS: ::DWORD = 0x003B0000;
pub const WNNC_NET_KWNP: ::DWORD = 0x003C0000;
pub const WNNC_NET_ZENWORKS: ::DWORD = 0x003D0000;
pub const WNNC_NET_DRIVEONWEB: ::DWORD = 0x003E0000;
pub const WNNC_NET_VMWARE: ::DWORD = 0x003F0000;
pub const WNNC_NET_RSFX: ::DWORD = 0x00400000;
pub const WNNC_NET_MFILES: ::DWORD = 0x00410000;
pub const WNNC_NET_MS_NFS: ::DWORD = 0x00420000;
pub const WNNC_NET_GOOGLE: ::DWORD = 0x00430000;
pub const WNNC_NET_NDFS: ::DWORD = 0x00440000;
pub const WNNC_NET_DOCUSHARE: ::DWORD = 0x00450000;
pub const WNNC_CRED_MANAGER: ::DWORD = 0xFFFF0000;
pub const WNNC_NET_LANMAN: ::DWORD = WNNC_NET_SMB;
pub const RESOURCE_CONNECTED: ::DWORD = 0x00000001;
pub const RESOURCE_GLOBALNET: ::DWORD = 0x00000002;
pub const RESOURCE_REMEMBERED: ::DWORD = 0x00000003;
pub const RESOURCE_RECENT: ::DWORD = 0x00000004;
pub const RESOURCE_CONTEXT: ::DWORD = 0x00000005;
pub const RESOURCETYPE_ANY: ::DWORD = 0x00000000;
pub const RESOURCETYPE_DISK: ::DWORD = 0x00000001;
pub const RESOURCETYPE_PRINT: ::DWORD = 0x00000002;
pub const RESOURCETYPE_RESERVED: ::DWORD = 0x00000008;
pub const RESOURCETYPE_UNKNOWN: ::DWORD = 0xFFFFFFFF;
pub const RESOURCEUSAGE_CONNECTABLE: ::DWORD = 0x00000001;
pub const RESOURCEUSAGE_CONTAINER: ::DWORD = 0x00000002;
pub const RESOURCEUSAGE_NOLOCALDEVICE: ::DWORD = 0x00000004;
pub const RESOURCEUSAGE_SIBLING: ::DWORD = 0x00000008;
pub const RESOURCEUSAGE_ATTACHED: ::DWORD = 0x00000010;
pub const RESOURCEUSAGE_ALL: ::DWORD = RESOURCEUSAGE_CONNECTABLE | RESOURCEUSAGE_CONTAINER
    | RESOURCEUSAGE_ATTACHED;
pub const RESOURCEUSAGE_RESERVED: ::DWORD = 0x80000000;
pub const RESOURCEDISPLAYTYPE_GENERIC: ::DWORD = 0x00000000;
pub const RESOURCEDISPLAYTYPE_DOMAIN: ::DWORD = 0x00000001;
pub const RESOURCEDISPLAYTYPE_SERVER: ::DWORD = 0x00000002;
pub const RESOURCEDISPLAYTYPE_SHARE: ::DWORD = 0x00000003;
pub const RESOURCEDISPLAYTYPE_FILE: ::DWORD = 0x00000004;
pub const RESOURCEDISPLAYTYPE_GROUP: ::DWORD = 0x00000005;
pub const RESOURCEDISPLAYTYPE_NETWORK: ::DWORD = 0x00000006;
pub const RESOURCEDISPLAYTYPE_ROOT: ::DWORD = 0x00000007;
pub const RESOURCEDISPLAYTYPE_SHAREADMIN: ::DWORD = 0x00000008;
pub const RESOURCEDISPLAYTYPE_DIRECTORY: ::DWORD = 0x00000009;
pub const RESOURCEDISPLAYTYPE_TREE: ::DWORD = 0x0000000A;
pub const RESOURCEDISPLAYTYPE_NDSCONTAINER: ::DWORD = 0x0000000B;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NETRESOURCEA {
    pub dwScope: ::DWORD,
    pub dwType: ::DWORD,
    pub dwDisplayType: ::DWORD,
    pub dwUsage: ::DWORD,
    pub lpLocalName: ::LPSTR,
    pub lpRemoteName: ::LPSTR,
    pub lpComment: ::LPSTR,
    pub lpProvider: ::LPSTR,
}
pub type LPNETRESOURCEA = *mut NETRESOURCEA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NETRESOURCEW {
    pub dwScope: ::DWORD,
    pub dwType: ::DWORD,
    pub dwDisplayType: ::DWORD,
    pub dwUsage: ::DWORD,
    pub lpLocalName: ::LPWSTR,
    pub lpRemoteName: ::LPWSTR,
    pub lpComment: ::LPWSTR,
    pub lpProvider: ::LPWSTR,
}
pub type LPNETRESOURCEW = *mut NETRESOURCEW;
pub const NETPROPERTY_PERSISTENT: ::DWORD = 1;
pub const CONNECT_UPDATE_PROFILE: ::DWORD = 0x00000001;
pub const CONNECT_UPDATE_RECENT: ::DWORD = 0x00000002;
pub const CONNECT_TEMPORARY: ::DWORD = 0x00000004;
pub const CONNECT_INTERACTIVE: ::DWORD = 0x00000008;
pub const CONNECT_PROMPT: ::DWORD = 0x00000010;
pub const CONNECT_NEED_DRIVE: ::DWORD = 0x00000020;
pub const CONNECT_REFCOUNT: ::DWORD = 0x00000040;
pub const CONNECT_REDIRECT: ::DWORD = 0x00000080;
pub const CONNECT_LOCALDRIVE: ::DWORD = 0x00000100;
pub const CONNECT_CURRENT_MEDIA: ::DWORD = 0x00000200;
pub const CONNECT_DEFERRED: ::DWORD = 0x00000400;
pub const CONNECT_RESERVED: ::DWORD = 0xFF000000;
pub const CONNECT_COMMANDLINE: ::DWORD = 0x00000800;
pub const CONNECT_CMD_SAVECRED: ::DWORD = 0x00001000;
pub const CONNECT_CRED_RESET: ::DWORD = 0x00002000;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct CONNECTDLGSTRUCTA {
    pub cbStructure: ::DWORD,
    pub hwndOwner: ::HWND,
    pub lpConnRes: ::LPNETRESOURCEA,
    pub dwFlags: ::DWORD,
    pub dwDevNum: ::DWORD,
}
pub type LPCONNECTDLGSTRUCTA = *mut CONNECTDLGSTRUCTA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct CONNECTDLGSTRUCTW {
    pub cbStructure: ::DWORD,
    pub hwndOwner: ::HWND,
    pub lpConnRes: ::LPNETRESOURCEW,
    pub dwFlags: ::DWORD,
    pub dwDevNum: ::DWORD,
}
pub type LPCONNECTDLGSTRUCTW = *mut CONNECTDLGSTRUCTW;
pub const CONNDLG_RO_PATH: ::DWORD = 0x00000001;
pub const CONNDLG_CONN_POINT: ::DWORD = 0x00000002;
pub const CONNDLG_USE_MRU: ::DWORD = 0x00000004;
pub const CONNDLG_HIDE_BOX: ::DWORD = 0x00000008;
pub const CONNDLG_PERSIST: ::DWORD = 0x00000010;
pub const CONNDLG_NOT_PERSIST: ::DWORD = 0x00000020;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DISCDLGSTRUCTA {
    pub cbStructure: ::DWORD,
    pub hwndOwner: ::HWND,
    pub lpLocalName: ::LPSTR,
    pub lpRemoteName: ::LPSTR,
    pub dwFlags: ::DWORD,
}
pub type LPDISCDLGSTRUCTA = *mut DISCDLGSTRUCTA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DISCDLGSTRUCTW {
    pub cbStructure: ::DWORD,
    pub hwndOwner: ::HWND,
    pub lpLocalName: ::LPWSTR,
    pub lpRemoteName: ::LPWSTR,
    pub dwFlags: ::DWORD,
}
pub type LPDISCDLGSTRUCTW = *mut DISCDLGSTRUCTW;
pub const DISC_UPDATE_PROFILE: ::DWORD = 0x00000001;
pub const DISC_NO_FORCE: ::DWORD = 0x00000040;
pub const UNIVERSAL_NAME_INFO_LEVEL: ::DWORD = 0x00000001;
pub const REMOTE_NAME_INFO_LEVEL: ::DWORD = 0x00000002;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct UNIVERSAL_NAME_INFOA {
    pub lpUniversalName: ::LPSTR,
}
pub type LPUNIVERSAL_NAME_INFOA = *mut UNIVERSAL_NAME_INFOA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct UNIVERSAL_NAME_INFOW {
    pub lpUniversalName: ::LPWSTR,
}
pub type LPUNIVERSAL_NAME_INFOW = *mut UNIVERSAL_NAME_INFOW;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct REMOTE_NAME_INFOA {
    pub lpUniversalName: ::LPSTR,
    pub lpConnectionName: ::LPSTR,
    pub lpRemainingPath: ::LPSTR,
}
pub type LPREMOTE_NAME_INFOA = *mut REMOTE_NAME_INFOA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct REMOTE_NAME_INFOW {
    pub lpUniversalName: ::LPWSTR,
    pub lpConnectionName: ::LPWSTR,
    pub lpRemainingPath: ::LPWSTR,
}
pub type LPREMOTE_NAME_INFOW = *mut REMOTE_NAME_INFOW;
pub const WNFMT_MULTILINE: ::DWORD = 0x01;
pub const WNFMT_ABBREVIATED: ::DWORD = 0x02;
pub const WNFMT_INENUM: ::DWORD = 0x10;
pub const WNFMT_CONNECTION: ::DWORD = 0x20;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NETINFOSTRUCT {
    pub cbStructure: ::DWORD,
    pub dwProviderVersion: ::DWORD,
    pub dwStatus: ::DWORD,
    pub dwCharacteristics: ::DWORD,
    pub dwHandle: ::ULONG_PTR,
    pub wNetType: ::WORD,
    pub dwPrinters: ::DWORD,
    pub dwDrives: ::DWORD,
}
pub type LPNETINFOSTRUCT = *mut NETINFOSTRUCT;
pub const NETINFO_DLL16: ::DWORD = 0x00000001;
pub const NETINFO_DISKRED: ::DWORD = 0x00000004;
pub const NETINFO_PRINTERRED: ::DWORD = 0x00000008;
pub const WN_SUCCESS: ::DWORD = ::NO_ERROR;
pub const WN_NO_ERROR: ::DWORD = ::NO_ERROR;
pub const WN_NOT_SUPPORTED: ::DWORD = ::ERROR_NOT_SUPPORTED;
pub const WN_CANCEL: ::DWORD = ::ERROR_CANCELLED;
pub const WN_RETRY: ::DWORD = ::ERROR_RETRY;
pub const WN_NET_ERROR: ::DWORD = ::ERROR_UNEXP_NET_ERR;
pub const WN_MORE_DATA: ::DWORD = ::ERROR_MORE_DATA;
pub const WN_BAD_POINTER: ::DWORD = ::ERROR_INVALID_ADDRESS;
pub const WN_BAD_VALUE: ::DWORD = ::ERROR_INVALID_PARAMETER;
pub const WN_BAD_USER: ::DWORD = ::ERROR_BAD_USERNAME;
pub const WN_BAD_PASSWORD: ::DWORD = ::ERROR_INVALID_PASSWORD;
pub const WN_ACCESS_DENIED: ::DWORD = ::ERROR_ACCESS_DENIED;
pub const WN_FUNCTION_BUSY: ::DWORD = ::ERROR_BUSY;
pub const WN_WINDOWS_ERROR: ::DWORD = ::ERROR_UNEXP_NET_ERR;
pub const WN_OUT_OF_MEMORY: ::DWORD = ::ERROR_NOT_ENOUGH_MEMORY;
pub const WN_NO_NETWORK: ::DWORD = ::ERROR_NO_NETWORK;
pub const WN_EXTENDED_ERROR: ::DWORD = ::ERROR_EXTENDED_ERROR;
pub const WN_BAD_LEVEL: ::DWORD = ::ERROR_INVALID_LEVEL;
pub const WN_BAD_HANDLE: ::DWORD = ::ERROR_INVALID_HANDLE;
pub const WN_NOT_INITIALIZING: ::DWORD = ::ERROR_ALREADY_INITIALIZED;
pub const WN_NO_MORE_DEVICES: ::DWORD = ::ERROR_NO_MORE_DEVICES;
pub const WN_NOT_CONNECTED: ::DWORD = ::ERROR_NOT_CONNECTED;
pub const WN_OPEN_FILES: ::DWORD = ::ERROR_OPEN_FILES;
pub const WN_DEVICE_IN_USE: ::DWORD = ::ERROR_DEVICE_IN_USE;
pub const WN_BAD_NETNAME: ::DWORD = ::ERROR_BAD_NET_NAME;
pub const WN_BAD_LOCALNAME: ::DWORD = ::ERROR_BAD_DEVICE;
pub const WN_ALREADY_CONNECTED: ::DWORD = ::ERROR_ALREADY_ASSIGNED;
pub const WN_DEVICE_ERROR: ::DWORD = ::ERROR_GEN_FAILURE;
pub const WN_CONNECTION_CLOSED: ::DWORD = ::ERROR_CONNECTION_UNAVAIL;
pub const WN_NO_NET_OR_BAD_PATH: ::DWORD = ::ERROR_NO_NET_OR_BAD_PATH;
pub const WN_BAD_PROVIDER: ::DWORD = ::ERROR_BAD_PROVIDER;
pub const WN_CANNOT_OPEN_PROFILE: ::DWORD = ::ERROR_CANNOT_OPEN_PROFILE;
pub const WN_BAD_PROFILE: ::DWORD = ::ERROR_BAD_PROFILE;
pub const WN_BAD_DEV_TYPE: ::DWORD = ::ERROR_BAD_DEV_TYPE;
pub const WN_DEVICE_ALREADY_REMEMBERED: ::DWORD = ::ERROR_DEVICE_ALREADY_REMEMBERED;
pub const WN_CONNECTED_OTHER_PASSWORD: ::DWORD = ::ERROR_CONNECTED_OTHER_PASSWORD;
pub const WN_CONNECTED_OTHER_PASSWORD_DEFAULT: ::DWORD = ::ERROR_CONNECTED_OTHER_PASSWORD_DEFAULT;
pub const WN_NO_MORE_ENTRIES: ::DWORD = ::ERROR_NO_MORE_ITEMS;
pub const WN_NOT_CONTAINER: ::DWORD = ::ERROR_NOT_CONTAINER;
pub const WN_NOT_AUTHENTICATED: ::DWORD = ::ERROR_NOT_AUTHENTICATED;
pub const WN_NOT_LOGGED_ON: ::DWORD = ::ERROR_NOT_LOGGED_ON;
pub const WN_NOT_VALIDATED: ::DWORD = ::ERROR_NO_LOGON_SERVERS;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NETCONNECTINFOSTRUCT {
    pub cbStructure: ::DWORD,
    pub dwFlags: ::DWORD,
    pub dwSpeed: ::DWORD,
    pub dwDelay: ::DWORD,
    pub dwOptDataSize: ::DWORD,
}
pub type LPNETCONNECTINFOSTRUCT = *mut NETCONNECTINFOSTRUCT;
pub const WNCON_FORNETCARD: ::DWORD = 0x00000001;
pub const WNCON_NOTROUTED: ::DWORD = 0x00000002;
pub const WNCON_SLOWLINK: ::DWORD = 0x00000004;
pub const WNCON_DYNAMIC: ::DWORD = 0x00000008;

// Copyright © 2015-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! ApiSet Contract for api-ms-win-mm-misc-l1-1
use ctypes::{c_char, c_int};
use shared::basetsd::{DWORD_PTR, INT32};
use shared::minwindef::{BOOL, DWORD, HMODULE, HTASK, LPARAM, LRESULT, UINT};
use um::mmsyscom::{HDRVR, MMRESULT};
use um::winnt::{LONG, LPCSTR, LPCWSTR, LPSTR, LPWSTR, PHANDLE};
STRUCT!{#[repr(packed)] struct DRVCONFIGINFOEX {
    dwDCISize: DWORD,
    lpszDCISectionName: LPCWSTR,
    lpszDCIAliasName: LPCWSTR,
    dnDevNode: DWORD,
}}
pub type PDRVCONFIGINFOEX = *mut DRVCONFIGINFOEX;
pub type NPDRVCONFIGINFOEX = *mut DRVCONFIGINFOEX;
pub type LPDRVCONFIGINFOEX = *mut DRVCONFIGINFOEX;
pub const DRV_LOAD: UINT = 0x0001;
pub const DRV_ENABLE: UINT = 0x0002;
pub const DRV_OPEN: UINT = 0x0003;
pub const DRV_CLOSE: UINT = 0x0004;
pub const DRV_DISABLE: UINT = 0x0005;
pub const DRV_FREE: UINT = 0x0006;
pub const DRV_CONFIGURE: UINT = 0x0007;
pub const DRV_QUERYCONFIGURE: UINT = 0x0008;
pub const DRV_INSTALL: UINT = 0x0009;
pub const DRV_REMOVE: UINT = 0x000A;
pub const DRV_EXITSESSION: UINT = 0x000B;
pub const DRV_POWER: UINT = 0x000F;
pub const DRV_RESERVED: UINT = 0x0800;
pub const DRV_USER: UINT = 0x4000;
STRUCT!{#[repr(packed)] struct DRVCONFIGINFO {
    dwDCISize: DWORD,
    lpszDCISectionName: LPCWSTR,
    lpszDCIAliasName: LPCWSTR,
}}
pub type PDRVCONFIGINFO = *mut DRVCONFIGINFO;
pub type NPDRVCONFIGINFO = *mut DRVCONFIGINFO;
pub type LPDRVCONFIGINFO = *mut DRVCONFIGINFO;
pub const DRVCNF_CANCEL: LRESULT = 0x0000;
pub const DRVCNF_OK: LRESULT = 0x0001;
pub const DRVCNF_RESTART: LRESULT = 0x0002;
FN!{stdcall DRIVERPROC(
    DWORD_PTR,
    HDRVR,
    UINT,
    LPARAM,
    LPARAM,
) -> LRESULT}
extern "system" {
    pub fn CloseDriver(
        hDriver: HDRVR,
        lParam1: LPARAM,
        lParam2: LPARAM,
    ) -> LRESULT;
    pub fn OpenDriver(
        szDriverName: LPCWSTR,
        szSectionName: LPCWSTR,
        lParam2: LPARAM,
    ) -> HDRVR;
    pub fn SendDriverMessage(
        hDriver: HDRVR,
        message: UINT,
        lParam1: LPARAM,
        lParam2: LPARAM,
    ) -> LRESULT;
    pub fn DrvGetModuleHandle(
        hDriver: HDRVR,
    ) -> HMODULE;
    pub fn GetDriverModuleHandle(
        hDriver: HDRVR,
    ) -> HMODULE;
    pub fn DefDriverProc(
        dwDriverIdentifier: DWORD_PTR,
        hdrvr: HDRVR,
        uMsg: UINT,
        lParam1: LPARAM,
        lParam2: LPARAM,
    ) -> LRESULT;
}
pub const DRV_CANCEL: LRESULT = DRVCNF_CANCEL;
pub const DRV_OK: LRESULT = DRVCNF_OK;
pub const DRV_RESTART: LRESULT = DRVCNF_RESTART;
pub const DRV_MCI_FIRST: UINT = DRV_RESERVED;
pub const DRV_MCI_LAST: UINT = DRV_RESERVED + 0xFFF;
extern "system" {
    pub fn DriverCallback(
        dwCallback: DWORD_PTR,
        dwFlags: DWORD,
        hDevice: HDRVR,
        dwMsg: DWORD,
        dwUser: DWORD_PTR,
        dwParam1: DWORD_PTR,
        dwParam2: DWORD_PTR,
    ) -> BOOL;
    pub fn sndOpenSound(
        EventName: LPCWSTR,
        AppName: LPCWSTR,
        Flags: INT32,
        FileHandle: PHANDLE,
    ) -> LONG;
}
FN!{stdcall DRIVERMSGPROC(
    DWORD,
    DWORD,
    DWORD_PTR,
    DWORD_PTR,
    DWORD_PTR,
) -> DWORD}
extern "system" {
    pub fn mmDrvInstall(
        hDriver: HDRVR,
        wszDrvEntry: LPCWSTR,
        drvMessage: DRIVERMSGPROC,
        wFlags: UINT,
    ) -> UINT;
}
pub const MMIOERR_BASE: MMRESULT = 256;
pub const MMIOERR_FILENOTFOUND: MMRESULT = MMIOERR_BASE + 1;
pub const MMIOERR_OUTOFMEMORY: MMRESULT = MMIOERR_BASE + 2;
pub const MMIOERR_CANNOTOPEN: MMRESULT = MMIOERR_BASE + 3;
pub const MMIOERR_CANNOTCLOSE: MMRESULT = MMIOERR_BASE + 4;
pub const MMIOERR_CANNOTREAD: MMRESULT = MMIOERR_BASE + 5;
pub const MMIOERR_CANNOTWRITE: MMRESULT = MMIOERR_BASE + 6;
pub const MMIOERR_CANNOTSEEK: MMRESULT = MMIOERR_BASE + 7;
pub const MMIOERR_CANNOTEXPAND: MMRESULT = MMIOERR_BASE + 8;
pub const MMIOERR_CHUNKNOTFOUND: MMRESULT = MMIOERR_BASE + 9;
pub const MMIOERR_UNBUFFERED: MMRESULT = MMIOERR_BASE + 10;
pub const MMIOERR_PATHNOTFOUND: MMRESULT = MMIOERR_BASE + 11;
pub const MMIOERR_ACCESSDENIED: MMRESULT = MMIOERR_BASE + 12;
pub const MMIOERR_SHARINGVIOLATION: MMRESULT = MMIOERR_BASE + 13;
pub const MMIOERR_NETWORKERROR: MMRESULT = MMIOERR_BASE + 14;
pub const MMIOERR_TOOMANYOPENFILES: MMRESULT = MMIOERR_BASE + 15;
pub const MMIOERR_INVALIDFILE: MMRESULT = MMIOERR_BASE + 16;
pub const CFSEPCHAR: c_char = 0x2b;
pub type FOURCC = DWORD;
pub type HPSTR = *mut c_char;
DECLARE_HANDLE!(HMMIO, HMMIO__);
FN!{stdcall LPMMIOPROC(
    lpmmioinfo: LPSTR,
    uMsg: UINT,
    lParam1: LPARAM,
    lParam2: LPARAM,
) -> LRESULT}
STRUCT!{#[repr(packed)] struct MMIOINFO {
    dwFlags: DWORD,
    fccIOProc: FOURCC,
    pIOProc: LPMMIOPROC,
    wErrorRet: UINT,
    htask: HTASK,
    cchBuffer: LONG,
    pchBuffer: HPSTR,
    pchNext: HPSTR,
    pchEndRead: HPSTR,
    pchEndWrite: HPSTR,
    lBufOffset: LONG,
    lDiskOffset: LONG,
    adwInfo: [DWORD; 3],
    dwReserved1: DWORD,
    dwReserved2: DWORD,
    hmmio: HMMIO,
}}
pub type PMMIOINFO = *mut MMIOINFO;
pub type NPMMIOINFO = *mut MMIOINFO;
pub type LPMMIOINFO = *mut MMIOINFO;
pub type LPCMMIOINFO = *const MMIOINFO;
STRUCT!{#[repr(packed)] struct MMCKINFO {
    ckid: FOURCC,
    cksize: DWORD,
    fccType: FOURCC,
    dwDataOffset: DWORD,
    dwFlags: DWORD,
}}
pub type PMMCKINFO = *mut MMCKINFO;
pub type NPMMCKINFO = *mut MMCKINFO;
pub type LPMMCKINFO = *mut MMCKINFO;
pub type LPC = *const MMCKINFO;
pub const MMIO_RWMODE: DWORD = 0x00000003;
pub const MMIO_SHAREMODE: DWORD = 0x00000070;
pub const MMIO_CREATE: DWORD = 0x00001000;
pub const MMIO_PARSE: DWORD = 0x00000100;
pub const MMIO_DELETE: DWORD = 0x00000200;
pub const MMIO_EXIST: DWORD = 0x00004000;
pub const MMIO_ALLOCBUF: DWORD = 0x00010000;
pub const MMIO_GETTEMP: DWORD = 0x00020000;
pub const MMIO_DIRTY: DWORD = 0x10000000;
pub const MMIO_READ: DWORD = 0x00000000;
pub const MMIO_WRITE: DWORD = 0x00000001;
pub const MMIO_READWRITE: DWORD = 0x00000002;
pub const MMIO_COMPAT: DWORD = 0x00000000;
pub const MMIO_EXCLUSIVE: DWORD = 0x00000010;
pub const MMIO_DENYWRITE: DWORD = 0x00000020;
pub const MMIO_DENYREAD: DWORD = 0x00000030;
pub const MMIO_DENYNONE: DWORD = 0x00000040;
pub const MMIO_FHOPEN: DWORD = 0x0010;
pub const MMIO_EMPTYBUF: DWORD = 0x0010;
pub const MMIO_TOUPPER: DWORD = 0x0010;
pub const MMIO_INSTALLPROC: DWORD = 0x00010000;
pub const MMIO_GLOBALPROC: DWORD = 0x10000000;
pub const MMIO_REMOVEPROC: DWORD = 0x00020000;
pub const MMIO_UNICODEPROC: DWORD = 0x01000000;
pub const MMIO_FINDPROC: DWORD = 0x00040000;
pub const MMIO_FINDCHUNK: DWORD = 0x0010;
pub const MMIO_FINDRIFF: DWORD = 0x0020;
pub const MMIO_FINDLIST: DWORD = 0x0040;
pub const MMIO_CREATERIFF: DWORD = 0x0020;
pub const MMIO_CREATELIST: DWORD = 0x0040;
pub const MMIOM_READ: UINT = MMIO_READ;
pub const MMIOM_WRITE: UINT = MMIO_WRITE;
pub const MMIOM_SEEK: UINT = 2;
pub const MMIOM_OPEN: UINT = 3;
pub const MMIOM_CLOSE: UINT = 4;
pub const MMIOM_WRITEFLUSH: UINT = 5;
pub const MMIOM_RENAME: UINT = 6;
pub const MMIOM_USER: UINT = 0x8000;
pub const FOURCC_RIFF: DWORD = MAKEFOURCC!('R', 'I', 'F', 'F');
pub const FOURCC_LIST: DWORD = MAKEFOURCC!('L', 'I', 'S', 'T');
pub const FOURCC_DOS: DWORD = MAKEFOURCC!('D', 'O', 'S', ' ');
pub const FOURCC_MEM: DWORD = MAKEFOURCC!('M', 'E', 'M', ' ');
pub const SEEK_SET: c_int = 0;
pub const SEEK_CUR: c_int = 1;
pub const SEEK_END: c_int = 2;
pub const MMIO_DEFAULTBUFFER: DWORD = 8192;
extern "system" {
    pub fn mmioStringToFOURCCA(
        sz: LPCSTR,
        uFlags: UINT,
    ) -> FOURCC;
    pub fn mmioStringToFOURCCW(
        sz: LPCWSTR,
        uFlags: UINT,
    ) -> FOURCC;
    pub fn mmioInstallIOProcA(
        fccIOProc: FOURCC,
        pIOProc: LPMMIOPROC,
        dwFlags: DWORD,
    ) -> LPMMIOPROC;
    pub fn mmioInstallIOProcW(
        fccIOProc: FOURCC,
        pIOProc: LPMMIOPROC,
        dwFlags: DWORD,
    ) -> LPMMIOPROC;
    pub fn mmioOpenA(
        pszFileName: LPSTR,
        pmmioinfo: LPMMIOINFO,
        fdwOpen: DWORD,
    ) -> HMMIO;
    pub fn mmioOpenW(
        pszFileName: LPWSTR,
        pmmioinfo: LPMMIOINFO,
        fdwOpen: DWORD,
    ) -> HMMIO;
    pub fn mmioRenameA(
        pszFileName: LPCSTR,
        pszNewFileName: LPCSTR,
        pmmioinfo: LPCMMIOINFO,
        fdwRename: DWORD,
    ) -> MMRESULT;
    pub fn mmioRenameW(
        pszFileName: LPCWSTR,
        pszNewFileName: LPCWSTR,
        pmmioinfo: LPCMMIOINFO,
        fdwRename: DWORD,
    ) -> MMRESULT;
    pub fn mmioClose(
        hmmio: HMMIO,
        fuClose: UINT,
    ) -> MMRESULT;
    pub fn mmioRead(
        hmmio: HMMIO,
        pch: HPSTR,
        cch: LONG,
    ) -> LONG;
    pub fn mmioWrite(
        hmmio: HMMIO,
        pch: *const c_char,
        cch: LONG,
    ) -> LONG;
    pub fn mmioSeek(
        hmmio: HMMIO,
        lOffset: LONG,
        iOrigin: c_int,
    ) -> LONG;
    pub fn mmioGetInfo(
        hmmio: HMMIO,
        pmmioinfo: LPMMIOINFO,
        fuInfo: UINT,
    ) -> MMRESULT;
    pub fn mmioSetInfo(
        hmmio: HMMIO,
        pmmioinfo: LPMMIOINFO,
        fuInfo: UINT,
    ) -> MMRESULT;
    pub fn mmioSetBuffer(
        hmmio: HMMIO,
        pchBuffer: LPSTR,
        cchBuffer: LONG,
        fuBuffer: UINT,
    ) -> MMRESULT;
    pub fn mmioFlush(
        hmmio: HMMIO,
        fuFlush: UINT,
    ) -> MMRESULT;
    pub fn mmioAdvance(
        hmmio: HMMIO,
        pmmioinfo: LPMMIOINFO,
        fuAdvance: UINT,
    ) -> MMRESULT;
    pub fn mmioSendMessage(
        hmmio: HMMIO,
        uMsg: UINT,
        lParam1: LPARAM,
        lParam2: LPARAM,
    ) -> LRESULT;
    pub fn mmioDescend(
        hmmio: HMMIO,
        pmmcki: LPMMCKINFO,
        pmmckiParent: *const MMCKINFO,
        fuDescend: UINT,
    ) -> MMRESULT;
    pub fn mmioAscend(
        hmmio: HMMIO,
        pmmcki: LPMMCKINFO,
        fuAscend: UINT,
    ) -> MMRESULT;
    pub fn mmioCreateChunk(
        hmmio: HMMIO,
        pmmcki: LPMMCKINFO,
        fuCreate: UINT,
    ) -> MMRESULT;
}

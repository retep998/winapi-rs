// Copyright © 2016-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::minwindef::{BYTE, DWORD, ULONG, USHORT};
use shared::wtypes::PROPID;
pub use um::propidlbase::*;
use um::winnt::{BOOLEAN, HRESULT};
pub const PIDDI_THUMBNAIL: ULONG = 0x00000002;
pub const PIDSI_TITLE: ULONG = 0x00000002;
pub const PIDSI_SUBJECT: ULONG = 0x00000003;
pub const PIDSI_AUTHOR: ULONG = 0x00000004;
pub const PIDSI_KEYWORDS: ULONG = 0x00000005;
pub const PIDSI_COMMENTS: ULONG = 0x00000006;
pub const PIDSI_TEMPLATE: ULONG = 0x00000007;
pub const PIDSI_LASTAUTHOR: ULONG = 0x00000008;
pub const PIDSI_REVNUMBER: ULONG = 0x00000009;
pub const PIDSI_EDITTIME: ULONG = 0x0000000a;
pub const PIDSI_LASTPRINTED: ULONG = 0x0000000b;
pub const PIDSI_CREATE_DTM: ULONG = 0x0000000c;
pub const PIDSI_LASTSAVE_DTM: ULONG = 0x0000000d;
pub const PIDSI_PAGECOUNT: ULONG = 0x0000000e;
pub const PIDSI_WORDCOUNT: ULONG = 0x0000000f;
pub const PIDSI_CHARCOUNT: ULONG = 0x00000010;
pub const PIDSI_THUMBNAIL: ULONG = 0x00000011;
pub const PIDSI_APPNAME: ULONG = 0x00000012;
pub const PIDSI_DOC_SECURITY: ULONG = 0x00000013;
pub const PIDDSI_CATEGORY: ULONG = 0x00000002;
pub const PIDDSI_PRESFORMAT: ULONG = 0x00000003;
pub const PIDDSI_BYTECOUNT: ULONG = 0x00000004;
pub const PIDDSI_LINECOUNT: ULONG = 0x00000005;
pub const PIDDSI_PARCOUNT: ULONG = 0x00000006;
pub const PIDDSI_SLIDECOUNT: ULONG = 0x00000007;
pub const PIDDSI_NOTECOUNT: ULONG = 0x00000008;
pub const PIDDSI_HIDDENCOUNT: ULONG = 0x00000009;
pub const PIDDSI_MMCLIPCOUNT: ULONG = 0x0000000A;
pub const PIDDSI_SCALE: ULONG = 0x0000000B;
pub const PIDDSI_HEADINGPAIR: ULONG = 0x0000000C;
pub const PIDDSI_DOCPARTS: ULONG = 0x0000000D;
pub const PIDDSI_MANAGER: ULONG = 0x0000000E;
pub const PIDDSI_COMPANY: ULONG = 0x0000000F;
pub const PIDDSI_LINKSDIRTY: ULONG = 0x00000010;
pub const PIDMSI_EDITOR: ULONG = 0x00000002;
pub const PIDMSI_SUPPLIER: ULONG = 0x00000003;
pub const PIDMSI_SOURCE: ULONG = 0x00000004;
pub const PIDMSI_SEQUENCE_NO: ULONG = 0x00000005;
pub const PIDMSI_PROJECT: ULONG = 0x00000006;
pub const PIDMSI_STATUS: ULONG = 0x00000007;
pub const PIDMSI_OWNER: ULONG = 0x00000008;
pub const PIDMSI_RATING: ULONG = 0x00000009;
pub const PIDMSI_PRODUCTION: ULONG = 0x0000000A;
pub const PIDMSI_COPYRIGHT: ULONG = 0x0000000B;
ENUM!{enum PIDMSI_STATUS_VALUE {
    PIDMSI_STATUS_NORMAL = 0,
    PIDMSI_STATUS_NEW = PIDMSI_STATUS_NORMAL + 1,
    PIDMSI_STATUS_PRELIM = PIDMSI_STATUS_NEW + 1,
    PIDMSI_STATUS_DRAFT = PIDMSI_STATUS_PRELIM + 1,
    PIDMSI_STATUS_INPROGRESS = PIDMSI_STATUS_DRAFT + 1,
    PIDMSI_STATUS_EDIT = PIDMSI_STATUS_INPROGRESS + 1,
    PIDMSI_STATUS_REVIEW = PIDMSI_STATUS_EDIT + 1,
    PIDMSI_STATUS_PROOF = PIDMSI_STATUS_REVIEW + 1,
    PIDMSI_STATUS_FINAL = PIDMSI_STATUS_PROOF + 1,
    PIDMSI_STATUS_OTHER = 0x7fff,
}}
extern "system" {
    pub fn PropVariantCopy(
        pvarDest: *mut PROPVARIANT,
        pvarSrc: *const PROPVARIANT,
    ) -> HRESULT;
    pub fn PropVariantClear(
        pvar: *mut PROPVARIANT,
    ) -> HRESULT;
    pub fn FreePropVariantArray(
        cVariants: ULONG,
        rgvars: *mut PROPVARIANT,
    ) -> HRESULT;
}
#[inline]
pub unsafe fn PropVariantInit(pvar: *mut PROPVARIANT) {
    *pvar = ::_core::mem::zeroed();
}
STRUCT!{struct SERIALIZEDPROPERTYVALUE {
    dwType: DWORD,
    rgb: [BYTE; 1],
}}
extern "system" {
    pub fn StgConvertVariantToProperty(
        pvar: *const PROPVARIANT,
        CodePage: USHORT,
        pprop: *mut SERIALIZEDPROPERTYVALUE,
        pcb: *mut ULONG,
        pid: PROPID,
        fReserved: BOOLEAN,
        pcIndirect: *mut ULONG,
    ) -> *mut SERIALIZEDPROPERTYVALUE;
    // pub fn StgConvertPropertyToVariant(
    //     pprop: *const SERIALIZEDPROPERTYVALUE,
    //     CodePage: USHORT,
    //     pvar: *mut PROPVARIANT,
    //     pma: *mut IMemoryAllocator,
    // ) -> BOOLEAN;
}

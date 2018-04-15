// Copyright © 2018 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::basetsd::{INT16, UINT32};
use shared::minwindef::{DWORD, INT, UINT, WORD};
use shared::windef::{RECTL, SIZEL};
use um::gdiplusenums::{
    MetafileType, MetafileTypeEmf, MetafileTypeEmfPlusDual, MetafileTypeEmfPlusOnly,
    MetafileTypeWmf, MetafileTypeWmfPlaceable
};
use um::gdiplustypes::{REAL, Rect};
use um::wingdi::METAHEADER;
STRUCT!{struct ENHMETAHEADER3 {
    iType: DWORD,
    nSize: DWORD,
    rclBounds: RECTL,
    rclFrame: RECTL,
    dSignature: DWORD,
    nVersion: DWORD,
    nBytes: DWORD,
    nRecords: DWORD,
    nHandles: WORD,
    sReserved: WORD,
    nDescription: DWORD,
    offDescription: DWORD,
    nPalEntries: DWORD,
    szlDevice: SIZEL,
    szlMillimeters: SIZEL,
}}
STRUCT!{struct PWMFRect16 {
    Left: INT16,
    Top: INT16,
    Right: INT16,
    Bottom: INT16,
}}
// #[align(2)]
STRUCT!{struct WmfPlaceableFileHeader {
    Key: UINT32,
    Hmf: INT16,
    BoundingBox: PWMFRect16,
    Inch: INT16,
    Reserved: UINT32,
    Checksum: INT16,
}}
pub const GDIP_EMFPLUSFLAGS_DISPLAY: DWORD = 0x00000001;
UNION!{union MetafileHeader_u {
    [u32; 22],
    WmfHeader WmfHeader_mut: METAHEADER,
    EmfHeader EmfHeader_mut: ENHMETAHEADER3,
}}
STRUCT!{struct MetafileHeader {
    Type: MetafileType,
    Size: UINT,
    Version: UINT,
    EmfPlusFlags: UINT,
    DpiX: REAL,
    DpiY: REAL,
    X: INT,
    Y: INT,
    Width: INT,
    Height: INT,
    u: MetafileHeader_u,
    EmfPlusHeaderSize: INT,
    LogicalDpiX: INT,
    LogicalDpiY: INT,
}}
impl MetafileHeader {
    pub fn GetType(&self) -> MetafileType {
        self.Type
    }
    pub fn GetMetafileSize(&self) -> UINT {
        self.Size
    }
    pub fn GetVersion(&self) -> UINT {
        self.Version
    }
    pub fn GetEmfPlusFlags(&self) -> UINT {
        self.EmfPlusFlags
    }
    pub fn GetDpiX(&self) -> REAL {
        self.DpiX
    }
    pub fn GetDpiY(&self) -> REAL {
        self.DpiY
    }
    pub fn GetBounds(&self) -> Rect {
        Rect {
            X: self.X,
            Y: self.Y,
            Width: self.Width,
            Height: self.Height,
        }
    }
    pub fn IsWmf(&self) -> bool {
        self.Type == MetafileTypeWmf || self.Type == MetafileTypeWmfPlaceable
    }
    pub fn IsWmfPlaceable(&self) -> bool {
        self.Type == MetafileTypeWmfPlaceable
    }
    pub fn IsEmf(&self) -> bool {
        self.Type == MetafileTypeEmf
    }
    pub fn IsEmfOrEmfPlus(&self) -> bool {
        self.Type >= MetafileTypeEmf
    }
    pub fn IsEmfPlus(&self) -> bool {
        self.Type >= MetafileTypeEmfPlusOnly
    }
    pub fn IsEmfPlusDual(&self) -> bool {
        self.Type == MetafileTypeEmfPlusDual
    }
    pub fn IsEmfPlusOnly(&self) -> bool {
        self.Type == MetafileTypeEmfPlusOnly
    }
    pub fn IsDisplay(&self) -> bool {
        self.IsEmfPlus() && (self.EmfPlusFlags & GDIP_EMFPLUSFLAGS_DISPLAY) != 0
    }
    pub fn GetWmfHeader(&self) -> Option<&METAHEADER> {
        if self.IsWmf() {
            Some(unsafe { self.u.WmfHeader() })
        } else {
            None
        }
    }
    pub fn GetEmfHeader(&self) -> Option<&ENHMETAHEADER3> {
        if self.IsEmfOrEmfPlus() {
            Some(unsafe { self.u.EmfHeader() })
        } else {
            None
        }
    }
}

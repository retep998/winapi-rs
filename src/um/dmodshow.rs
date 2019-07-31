// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::guiddef::{REFCLSID};
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::{HRESULT};
DEFINE_GUID!{CLSID_DMOWrapperFilter,
    0x94297043, 0xbd82, 0x4dfd, 0xb0, 0xde, 0x81, 0x77, 0x73, 0x9c, 0x6d, 0x20}
DEFINE_GUID!{CLSID_DMOFilterCategory,
    0xbcd5796c, 0xbd52, 0x4d30, 0xab, 0x76, 0x70, 0xf9, 0x75, 0xb8, 0x91, 0x99}
RIDL!{#[uuid(0x94297043, 0xbd82, 0x4dfd, 0xb0, 0xde, 0x81, 0x77, 0x73, 0x9c, 0x6d, 0x20)]
interface IDMOWrapperFilter(IDMOWrapperFilterVtbl): IUnknown(IUnknownVtbl) {
    fn Init(
        clsidDMO: REFCLSID,
        catDMO: REFCLSID,
    ) -> HRESULT,
}}

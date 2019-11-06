// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use ctypes::c_void;
use shared::guiddef::GUID;
use shared::minwindef::{DWORD, UINT};
use shared::wtypes::PROPERTYKEY;
use um::propidl::{PROPVARIANT, REFPROPVARIANT};
use um::propkeydef::REFPROPERTYKEY;
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::HRESULT;
pub type IPropertyDescriptionList = IUnknown; // TODO
RIDL!{#[uuid(0x886d8eeb, 0x8cf2, 0x4446, 0x8d, 0x02, 0xcd, 0xba, 0x1d, 0xbd, 0xcf, 0x99)]
interface IPropertyStore(IPropertyStoreVtbl): IUnknown(IUnknownVtbl) {
    fn GetCount(
        cProps: *mut DWORD,
    ) -> HRESULT,
    fn GetAt(
        iProp: DWORD,
        pkey: *mut PROPERTYKEY,
    ) -> HRESULT,
    fn GetValue(
        key: REFPROPERTYKEY,
        pv: *mut PROPVARIANT,
    ) -> HRESULT,
    fn SetValue(
        key: REFPROPERTYKEY,
        propvar: REFPROPVARIANT,
    ) -> HRESULT,
    fn Commit() -> HRESULT,
}}
ENUM!{enum GETPROPERTYSTOREFLAGS {
    GPS_DEFAULT = 0,
    GPS_HANDLERPROPERTIESONLY = 0x1,
    GPS_READWRITE = 0x2,
    GPS_TEMPORARY = 0x4,
    GPS_FASTPROPERTIESONLY = 0x8,
    GPS_OPENSLOWITEM = 0x10,
    GPS_DELAYCREATION = 0x20,
    GPS_BESTEFFORT = 0x40,
    GPS_NO_OPLOCK = 0x80,
    GPS_PREFERQUERYPROPERTIES = 0x100,
    GPS_EXTRINSICPROPERTIES = 0x200,
    GPS_EXTRINSICPROPERTIESONLY = 0x400,
    GPS_MASK_VALID = 0x7ff,
}}
RIDL!{#[uuid(0xfc0ca0a7, 0xc316, 0x4fd2, 0x90, 0x31, 0x3e, 0x62, 0x8e, 0x6d, 0x4f, 0x23)]
interface IObjectWithPropertyKey(IObjectWithPropertyKeyVtbl): IUnknown(IUnknownVtbl) {
    fn SetPropertyKey( 
        key: REFPROPERTYKEY,
    ) -> HRESULT,
    fn GetPropertyKey( 
        pkey: *mut PROPERTYKEY,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xf917bc8a, 0x1bba, 0x4478, 0xa2, 0x45, 0x1b, 0xde, 0x03, 0xeb, 0x94, 0x31)]
interface IPropertyChange(IPropertyChangeVtbl): IObjectWithPropertyKey(IObjectWithPropertyKeyVtbl){
    fn ApplyToPropVariant(
        propvarIn: *const PROPVARIANT,
        ppropvarOut: *mut PROPVARIANT,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x380f5cad, 0x1b5e, 0x42f2, 0x80, 0x5d, 0x63, 0x7f, 0xd3, 0x92, 0xd3, 0x1e)]
interface IPropertyChangeArray(IPropertyChangeArrayVtbl): IUnknown(IUnknownVtbl) {
    fn GetCount(
        pcOperations: *mut UINT,
    ) -> HRESULT,
    fn GetAt(
        iIndex: UINT,
        riid: *const GUID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
    fn InsertAt(
        iIndex: UINT,
        ppropChange: *const IPropertyChange,
    ) -> HRESULT,
    fn Append(
        ppropChange: *const IPropertyChange,
    ) -> HRESULT,
    fn AppendOrReplace(
        ppropChange: *const IPropertyChange,
    ) -> HRESULT,
    fn RemoveAt(
        iIndex: UINT,
    ) -> HRESULT,
    fn IsKeyInArray(
        key: *const PROPERTYKEY,
    ) -> HRESULT,
}}

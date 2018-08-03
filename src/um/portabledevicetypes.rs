// Copyright © 2015-2018 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Mappings for the contents of PortableDeviceTypes.h
use shared::guiddef::{CLSID, GUID, REFGUID};
use shared::minwindef::{BOOL, BYTE, DWORD, FLOAT, ULONG};
use shared::wtypes::{PROPERTYKEY, VARTYPE};
use um::propidl::PROPVARIANT;
use um::propkeydef::REFPROPERTYKEY;
use um::propsys::IPropertyStore;
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::{HRESULT, LONG, LONGLONG, LPCWSTR, LPWSTR, ULONGLONG};
//330
RIDL!{#[uuid(0x6848f6f2, 0x3155, 0x4f86, 0xb6, 0xf5, 0x26, 0x3e, 0xee, 0xab, 0x31, 0x43)]
interface IPortableDeviceValues(IPortableDeviceValuesVtbl): IUnknown(IUnknownVtbl) {
    fn GetCount(
        pcelt: *mut DWORD,
    ) -> HRESULT,
    fn GetAt(
        index: DWORD,
        pKey: *mut PROPERTYKEY,
        pValue: *mut PROPVARIANT,
    ) -> HRESULT,
    fn SetValue(
        key: REFPROPERTYKEY,
        pValue: *const PROPVARIANT,
    ) -> HRESULT,
    fn GetValue(
        key: REFPROPERTYKEY,
        pValue: *mut PROPVARIANT,
    ) -> HRESULT,
    fn SetStringValue(
        key: REFPROPERTYKEY,
        Value: LPCWSTR,
    ) -> HRESULT,
    fn GetStringValue(
        key: REFPROPERTYKEY,
        pValue: *mut LPWSTR,
    ) -> HRESULT,
    fn SetUnsignedIntegerValue(
        key: REFPROPERTYKEY,
        Value: ULONG,
    ) -> HRESULT,
    fn GetUnsignedIntegerValue(
        key: REFPROPERTYKEY,
        pValue: *mut ULONG,
    ) -> HRESULT,
    fn SetSignedIntegerValue(
        key: REFPROPERTYKEY,
        Value: LONG,
    ) -> HRESULT,
    fn GetSignedIntegerValue(
        key: REFPROPERTYKEY,
        pValue: *mut LONG,
    ) -> HRESULT,
    fn SetUnsignedLargeIntegerValue(
        key: REFPROPERTYKEY,
        Value: ULONGLONG,
    ) -> HRESULT,
    fn GetUnsignedLargeIntegerValue(
        key: REFPROPERTYKEY,
        pValue: *mut ULONGLONG,
    ) -> HRESULT,
    fn SetSignedLargeIntegerValue(
        key: REFPROPERTYKEY,
        Value: LONGLONG,
    ) -> HRESULT,
    fn GetSignedLargeIntegerValue(
        key: REFPROPERTYKEY,
        pValue: *mut LONGLONG,
    ) -> HRESULT,
    fn SetFloatValue(
        key: REFPROPERTYKEY,
        Value: FLOAT,
    ) -> HRESULT,
    fn GetFloatValue(
        key: REFPROPERTYKEY,
        pValue: *mut FLOAT,
    ) -> HRESULT,
    fn SetErrorValue(
        key: REFPROPERTYKEY,
        Value: HRESULT,
    ) -> HRESULT,
    fn GetErrorValue(
        key: REFPROPERTYKEY,
        pValue: *mut HRESULT,
    ) -> HRESULT,
    fn SetKeyValue(
        key: REFPROPERTYKEY,
        Value: REFPROPERTYKEY,
    ) -> HRESULT,
    fn GetKeyValue(
        key: REFPROPERTYKEY,
        pValue: *mut PROPERTYKEY,
    ) -> HRESULT,
    fn SetBoolValue(
        key: REFPROPERTYKEY,
        Value: BOOL,
    ) -> HRESULT,
    fn GetBoolValue(
        key: REFPROPERTYKEY,
        pValue: *mut BOOL,
    ) -> HRESULT,
    fn SetIUnknownValue(
        key: REFPROPERTYKEY,
        pValue: *mut IUnknown,
    ) -> HRESULT,
    fn GetIUnknownValue(
        key: REFPROPERTYKEY,
        ppValue: *mut *mut IUnknown,
    ) -> HRESULT,
    fn SetGuidValue(
        key: REFPROPERTYKEY,
        Value: REFGUID,
    ) -> HRESULT,
    fn GetGuidValue(
        key: REFPROPERTYKEY,
        pValue: *mut GUID,
    ) -> HRESULT,
    fn SetBufferValue(
        key: REFPROPERTYKEY,
        pValue: *mut BYTE,
        cbValue: DWORD,
    ) -> HRESULT,
    fn GetBufferValue(
        key: REFPROPERTYKEY,
        ppValue: *mut *mut BYTE,
        pcbValue: *mut DWORD,
    ) -> HRESULT,
    fn SetIPortableDeviceValuesValue(
        key: REFPROPERTYKEY,
        pValue: *mut IPortableDeviceValues,
    ) -> HRESULT,
    fn GetIPortableDeviceValuesValue(
        key: REFPROPERTYKEY,
        ppValue: *mut *mut IPortableDeviceValues,
    ) -> HRESULT,
    fn SetIPortableDevicePropVariantCollectionValue(
        key: REFPROPERTYKEY,
        pValue: *mut IPortableDevicePropVariantCollection,
    ) -> HRESULT,
    fn GetIPortableDevicePropVariantCollectionValue(
        key: REFPROPERTYKEY,
        ppValue: *mut *mut IPortableDevicePropVariantCollection,
    ) -> HRESULT,
    fn SetIPortableDeviceKeyCollectionValue(
        key: REFPROPERTYKEY,
        pValue: *mut IPortableDeviceKeyCollection,
    ) -> HRESULT,
    fn GetIPortableDeviceKeyCollectionValue(
        key: REFPROPERTYKEY,
         ppValue: *mut *mut IPortableDeviceKeyCollection,
    ) -> HRESULT,
    fn SetIPortableDeviceValuesCollectionValue(
        key: REFPROPERTYKEY,
        pValue: *mut IPortableDeviceValuesCollection,
    ) -> HRESULT,
    fn GetIPortableDeviceValuesCollectionValue(
        key: REFPROPERTYKEY,
        ppValue: *mut *mut IPortableDeviceValuesCollection,
    ) -> HRESULT,
    fn RemoveValue(
        key: REFPROPERTYKEY,
    ) -> HRESULT,
    fn CopyValuesFromPropertyStore(
        pStore: *mut IPropertyStore,
    ) -> HRESULT,
    fn CopyValuesToPropertyStore(
        pStore: *mut IPropertyStore,
    ) -> HRESULT,
    fn Clear() -> HRESULT,
}}
RIDL!{#[uuid(0xdada2357, 0xe0ad, 0x492e, 0x98, 0xdb, 0xdd, 0x61, 0xc5, 0x3b, 0xa3, 0x53)]
interface IPortableDeviceKeyCollection(IPortableDeviceKeyCollectionVtbl): IUnknown(IUnknownVtbl) {
    fn GetCount(
        pcElems: *mut DWORD,
    ) -> HRESULT,
    fn GetAt(
        dwIndex: DWORD,
        pKey: *mut PROPERTYKEY,
    ) -> HRESULT,
    fn Add(
        Key: REFPROPERTYKEY,
    ) -> HRESULT,
    fn Clear() -> HRESULT,
    fn RemoveAt(
        dwIndex: DWORD,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x89b2e422, 0x4f1b, 0x4316, 0xbc, 0xef, 0xa4, 0x4a, 0xfe, 0xa8, 0x3e, 0xb3)]
interface IPortableDevicePropVariantCollection(IPortableDevicePropVariantCollectionVtbl):
    IUnknown(IUnknownVtbl) {
    fn GetCount(
        pcElems: *mut DWORD,
    ) -> HRESULT,
    fn GetAt(
        dwIndex: DWORD,
        pValue: *mut PROPVARIANT,
    ) -> HRESULT,
    fn Add(
        pValue: *const PROPVARIANT,
    ) -> HRESULT,
    fn GetType(
        pvt: *mut VARTYPE,
    ) -> HRESULT,
    fn ChangeType(
        vt: VARTYPE,
    ) -> HRESULT,
    fn Clear() -> HRESULT,
    fn RemoveAt(
        dwIndex: DWORD,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x6e3f2d79, 0x4e07, 0x48c4, 0x82, 0x08, 0xd8, 0xc2, 0xe5, 0xaf, 0x4a, 0x99)]
interface IPortableDeviceValuesCollection(IPortableDeviceValuesCollectionVtbl):
    IUnknown(IUnknownVtbl) {
    fn GetCount(
        pcElems: *mut DWORD,
    ) -> HRESULT,
    fn GetAt(
        dwIndex: DWORD,
        ppValues: *mut *mut IPortableDeviceValues,
    ) -> HRESULT,
    fn Add(
        pValues: *mut IPortableDeviceValues,
    ) -> HRESULT,
    fn Clear() -> HRESULT,
    fn RemoveAt(
        dwIndex: DWORD,
    ) -> HRESULT,
}}
extern {
// EXTERN_C const IID LIBID_PortableDeviceTypesLib;
    pub static CLSID_WpdSerializer: CLSID;
    pub static CLSID_PortableDeviceValues: CLSID;
    pub static CLSID_PortableDeviceKeyCollection: CLSID;
    pub static CLSID_PortableDevicePropVariantCollection: CLSID;
    pub static CLSID_PortableDeviceValuesCollection: CLSID;
}

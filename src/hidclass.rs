// Copyright © 2015, Alex Daniel Jones
// Licensed under the MIT License <LICENSE.md>
// Taken from hidclass.h
pub type PHIDP_PREPARSED_DATA = ::PVOID;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HID_COLLECTION_INFORMATION {
    pub DescriptorSize: ::ULONG,
    pub Polled: ::BOOLEAN,
    pub Reserved1: [::UCHAR; 1],
    pub VendorID: ::USHORT,
    pub ProductID: ::USHORT,
    pub VersionNumber: ::USHORT,
}
pub type PHID_COLLECTION_INFORMATION = *mut HID_COLLECTION_INFORMATION;

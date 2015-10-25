// Copyright © 2015, Alex Daniel Jones
// Licensed under the MIT License <LICENSE.md>
// Taken from hidclass.h
pub type PHIDP_PREPARSED_DATA = ::PVOID;
STRUCT!{struct HID_COLLECTION_INFORMATION {
    DescriptorSize: ::ULONG,
    Polled: ::BOOLEAN,
    Reserved1: [::UCHAR; 1],
    VendorID: ::USHORT,
    ProductID: ::USHORT,
    VersionNumber: ::USHORT,
}}
pub type PHID_COLLECTION_INFORMATION = *mut HID_COLLECTION_INFORMATION;

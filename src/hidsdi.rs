// Copyright © 2015, Alex Daniel Jones
// Licensed under the MIT License <LICENSE.md>
// Taken from hidsdi.h
STRUCT!{struct HIDD_ATTRIBUTES {
    Size: ::ULONG,
    VendorID: ::USHORT,
    ProductID: ::USHORT,
    VersionNumber: ::USHORT,
}}
pub type PHIDD_ATTRIBUTES = *mut HIDD_ATTRIBUTES;

// Copyright © 2015, Alex Daniel Jones
// Licensed under the MIT License <LICENSE.md>
// Taken from hidsdi.h

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HIDD_ATTRIBUTES {
  pub Size: ::ULONG,
  pub VendorID: ::USHORT,
  pub ProductID: ::USHORT,
  pub VersionNumber: ::USHORT,
}
pub type PHIDD_ATTRIBUTES = *mut HIDD_ATTRIBUTES;

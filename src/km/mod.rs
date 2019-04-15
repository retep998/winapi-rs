// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Headers for kernel mode
#[cfg(feature = "d3dkmthk")] pub mod d3dkmthk;

//make relax and take slow
#[cfg(feature = "km_util")]
pub type MISS_TYPE_PTR = *const ::shared::ntdef::PVOID;

#[cfg(feature = "fwp")]
pub mod fwp;
#[cfg(feature = "ndis")]
pub mod ndis;
#[cfg(feature = "wdm")]
pub mod wdm;
#[cfg(feature = "fltkernel")]
pub mod fltkernel;

pub mod ntifs {
    /// Device object flags.
    #[repr(C)]
    pub enum DEVICE_FLAGS {
        NONE = 0,
        DO_VERIFY_VOLUME = 0x00000002,
        DO_BUFFERED_IO = 0x00000004,
        DO_EXCLUSIVE = 0x00000008,
        DO_DIRECT_IO = 0x00000010,
        DO_MAP_IO_BUFFER = 0x00000020,
        DO_DEVICE_HAS_NAME = 0x00000040,
        DO_DEVICE_INITIALIZING = 0x00000080,
        DO_SYSTEM_BOOT_PARTITION = 0x00000100,
        DO_LONG_TERM_REQUESTS = 0x00000200,
        DO_NEVER_LAST_DEVICE = 0x00000400,
        DO_SHUTDOWN_REGISTERED = 0x00000800,
        DO_BUS_ENUMERATED_DEVICE = 0x00001000,
        DO_POWER_PAGABLE = 0x00002000,
        DO_POWER_INRUSH = 0x00004000,
        DO_POWER_NOOP = 0x00008000,
        DO_LOW_PRIORITY_FILESYSTEM = 0x00010000,
        DO_XIP = 0x00020000,
    }
}

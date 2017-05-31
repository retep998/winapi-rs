// Copyright © 2015-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! ApiSet Contract for api-ms-win-core-threadpool-l1.

use shared::basetsd::ULONG_PTR;
use shared::minwindef::ULONG;
use um::winnt::{PTP_CALLBACK_INSTANCE, PTP_IO, PVOID};

FN!{stdcall PTP_WIN32_IO_CALLBACK(
    Instance: PTP_CALLBACK_INSTANCE,
    Context: PVOID,
    Overlapped: PVOID,
    IoResult: ULONG,
    NumberOfBytesTransferred: ULONG_PTR,
    Io: PTP_IO,
) -> ()}

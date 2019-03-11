// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// #include <time.h>
// #include <vcruntime.h>
use shared::ntdef::{CHAR, LONGLONG, PCHAR, PWCHAR, ULONG, ULONGLONG, WCHAR};
pub type __time32_t = LONG;
pub type __time64_t = LONGLONG;
pub type time_t = __time64_t;

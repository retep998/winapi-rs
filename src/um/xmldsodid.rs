// Copyright © 2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
pub use um::winnt::LONG;
pub const DISPID_XMLDSO: LONG = 0x00010000;
pub const DISPID_XMLDSO_DOCUMENT: LONG = DISPID_XMLDSO + 1;
pub const DISPID_XMLDSO_JAVADSOCOMPATIBLE: LONG = DISPID_XMLDSO_DOCUMENT + 1;

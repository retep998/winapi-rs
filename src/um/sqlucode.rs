// Copyright © 2015-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! This module defines the ODBC Core unicode functions
use um::sqltypes::SQLSMALLINT;
pub const SQL_WCHAR: SQLSMALLINT = -8;
pub const SQL_WVARCHAR: SQLSMALLINT = -9;
pub const SQL_WLONGVARCHAR: SQLSMALLINT = -10;
pub const SQL_C_WCHAR: SQLSMALLINT = SQL_WCHAR;
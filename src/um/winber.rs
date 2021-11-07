// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Public definitions for the winber library
use ctypes::c_int;
use shared::minwindef::{INT, UINT, ULONG};
use shared::ntdef::{LPSTR, PCHAR};
STRUCT! {struct BerElement {
    opaque : PCHAR,
}}
pub type ber_len_t = ULONG;
STRUCT! {struct BerValue {
    bv_len : ber_len_t,
    bv_val : LPSTR,
}}
pub type LDAP_BERVAL = BerValue;
pub type PLDAP_BERVAL = *mut BerValue;
pub type BERVAL = BerValue;
pub type PBERVAL = *mut BerValue;
pub type ber_tag_t = ULONG;
pub type ber_int_t = INT;
pub type ber_uint_t = UINT;
pub type ber_slen_t = INT;
pub const LBER_ERROR: ber_tag_t = -1i32 as u32;
pub const LBER_DEFAULT: ber_tag_t = -1i32 as u32;
pub const LBER_USE_DER: ULONG = 0x01;
extern "C" {
    pub fn ber_init(a: &BerValue) -> *mut BerElement;
    //pub fn ber_printf(BerElement*,const char*,...) -> int;
    pub fn ber_flatten(a: *mut BerElement, b: *mut *mut BerValue) -> c_int;
    // pub fn ber_scanf(BerElement*,const char*,...) -> ber_tag_t;
    pub fn ber_peek_tag(a: *mut BerElement, b: *mut ber_len_t) -> ber_tag_t;
    pub fn ber_skip_tag(a: *mut BerElement, b: *mut ber_len_t) -> ber_tag_t;
    pub fn ber_first_element(a: *mut BerElement, b: *mut ber_len_t, c: *mut LPSTR) -> ber_tag_t;
    pub fn ber_next_element(a: *mut BerElement, b: *mut ber_len_t, c: LPSTR) -> ber_tag_t;
    pub fn ber_bvfree(a: *mut BerValue) -> ();
    pub fn ber_bvecfree(a: *mut *mut BerValue) -> ();
    pub fn ber_free(a: *mut BerElement, b: INT) -> ();
    pub fn ber_bvdup(a: *mut BerValue) -> *mut BerValue;
    pub fn ber_alloc_t(a: INT) -> *mut BerElement;
}

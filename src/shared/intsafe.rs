// Copyright © 2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! This module defines helper functions to prevent integer overflow bugs
use ctypes::{
    __int64, __uint64, c_char, c_int, c_long, c_short, c_uchar, c_uint, c_ulong, c_ushort,
};
pub type CHAR = c_char;
pub type INT8 = c_char;
pub type UCHAR = c_uchar;
pub type UINT8 = c_uchar;
pub type BYTE = c_uchar;
pub type SHORT = c_short;
pub type INT16 = c_short;
pub type USHORT = c_ushort;
pub type UINT16 = c_ushort;
pub type WORD = c_ushort;
pub type INT = c_int;
pub type INT32 = c_int;
pub type UINT = c_uint;
pub type UINT32 = c_uint;
pub type LONG = c_long;
pub type ULONG = c_ulong;
pub type DWORD = c_ulong;
pub type LONGLONG = __int64;
pub type LONG64 = __int64;
pub type INT64 = __int64;
pub type ULONGLONG = __uint64;
pub type DWORDLONG = __uint64;
pub type ULONG64 = __uint64;
pub type DWORD64 = __uint64;
pub type UINT64 = __uint64;
#[cfg(target_arch = "x86_64")]
IFDEF!{
pub type INT_PTR = __int64;
pub type UINT_PTR = __uint64;
pub type LONG_PTR = __int64;
pub type ULONG_PTR = __uint64;
}
#[cfg(target_arch = "x86")]
IFDEF!{
pub type INT_PTR = c_int;
pub type UINT_PTR = c_uint;
pub type LONG_PTR = c_long;
pub type ULONG_PTR = c_ulong;
}
#[cfg(target_arch = "x86_64")]
IFDEF!{
pub type ptrdiff_t = __int64;
pub type size_t = __uint64;
}
#[cfg(target_arch = "x86")]
IFDEF!{
pub type ptrdiff_t = c_int;
pub type size_t = c_uint;
}
pub type DWORD_PTR = ULONG_PTR;
pub type SSIZE_T = LONG_PTR;
pub type SIZE_T = ULONG_PTR;
pub type HRESULT = c_long;
#[inline]
pub fn SUCCEEDED(hr: HRESULT) -> bool {
    hr >= 0
}
#[inline]
pub fn FAILED(hr: HRESULT) -> bool {
    hr < 0
}
pub const S_OK: HRESULT = 0;
pub const INTSAFE_E_ARITHMETIC_OVERFLOW: HRESULT = 0x80070216;
pub const INT8_MIN: INT8 = -127i8 - 1;
pub const SHORT_MIN: SHORT = -32768;
pub const INT16_MIN: INT16 = -32767i16 - 1;
pub const INT_MIN: INT = -2147483647 - 1;
pub const INT32_MIN: INT32 = -2147483647i32 - 1;
pub const LONG_MIN: LONG = -2147483647 - 1;
pub const LONGLONG_MIN: LONGLONG = -9223372036854775807i64 - 1;
pub const LONG64_MIN: LONG64 = -9223372036854775807i64 - 1;
pub const INT64_MIN: INT64 = -9223372036854775807i64 - 1;
// unstable
// pub const INT128_MIN: i128 = -170141183460469231731687303715884105727i128 - 1;
#[cfg(target_arch = "x86_64")]
IFDEF!{
pub const INT_PTR_MIN: INT_PTR = -9223372036854775807i64 - 1;
pub const LONG_PTR_MIN: LONG_PTR = -9223372036854775807i64 - 1;
pub const PTRDIFF_T_MIN: ptrdiff_t = -9223372036854775807i64 - 1;
pub const SSIZE_T_MIN: SSIZE_T = -9223372036854775807i64 - 1;
}
#[cfg(target_arch = "x86")]
IFDEF!{
pub const INT_PTR_MIN: INT_PTR = -2147483647 - 1;
pub const LONG_PTR_MIN: LONG_PTR = -2147483647 - 1;
pub const PTRDIFF_T_MIN: ptrdiff_t = -2147483647 - 1;
pub const SSIZE_T_MIN: SSIZE_T = -2147483647 - 1;
}
pub const INT8_MAX: INT8 = 127;
pub const UINT8_MAX: UINT8 = 0xff;
pub const BYTE_MAX: BYTE = 0xff;
pub const SHORT_MAX: SHORT = 32767;
pub const INT16_MAX: INT16 = 32767;
pub const USHORT_MAX: USHORT = 0xffff;
pub const UINT16_MAX: UINT16 = 0xffff;
pub const WORD_MAX: WORD = 0xffff;
pub const INT_MAX: INT = 2147483647;
pub const INT32_MAX: INT32 = 2147483647;
pub const UINT_MAX: UINT = 0xffffffff;
pub const UINT32_MAX: UINT32 = 0xffffffff;
pub const LONG_MAX: LONG = 2147483647;
pub const ULONG_MAX: ULONG = 0xffffffff;
pub const DWORD_MAX: DWORD = 0xffffffff;
pub const LONGLONG_MAX: LONGLONG = 9223372036854775807;
pub const LONG64_MAX: LONG64 = 9223372036854775807;
pub const INT64_MAX: INT64 = 9223372036854775807i64;
pub const ULONGLONG_MAX: ULONGLONG = 0xffffffffffffffff;
pub const DWORDLONG_MAX: DWORDLONG = 0xffffffffffffffff;
pub const ULONG64_MAX: ULONG64 = 0xffffffffffffffff;
pub const DWORD64_MAX: DWORD64 = 0xffffffffffffffff;
pub const UINT64_MAX: UINT64 = 0xffffffffffffffff;
// unstable
// pub const INT128_MAX: INT128 = 170141183460469231731687303715884105727;
// pub const UINT128_MAX: UINT128 = 0xffffffffffffffffffffffffffffffff;
#[cfg(target_arch = "x86_64")]
IFDEF!{
pub const INT_PTR_MAX: INT_PTR = 9223372036854775807;
pub const UINT_PTR_MAX: UINT_PTR = 0xffffffffffffffff;
pub const LONG_PTR_MAX: LONG_PTR = 9223372036854775807;
pub const ULONG_PTR_MAX: ULONG_PTR = 0xffffffffffffffff;
pub const DWORD_PTR_MAX: DWORD_PTR = 0xffffffffffffffff;
pub const PTRDIFF_T_MAX: ptrdiff_t = 9223372036854775807;
pub const SIZE_T_MAX: SIZE_T = 0xffffffffffffffff;
pub const SSIZE_T_MAX: SSIZE_T = 9223372036854775807;
pub const _SIZE_T_MAX: SIZE_T = 0xffffffffffffffff;
}
#[cfg(target_arch = "x86")]
IFDEF!{
pub const INT_PTR_MAX: INT_PTR = 2147483647;
pub const UINT_PTR_MAX: UINT_PTR = 0xffffffff;
pub const LONG_PTR_MAX: LONG_PTR = 2147483647;
pub const ULONG_PTR_MAX: ULONG_PTR = 0xffffffff;
pub const DWORD_PTR_MAX: DWORD_PTR = 0xffffffff;
pub const PTRDIFF_T_MAX: ptrdiff_t = 2147483647;
pub const SIZE_T_MAX: SIZE_T = 0xffffffff;
pub const SSIZE_T_MAX: SSIZE_T = 2147483647;
pub const _SIZE_T_MAX: SIZE_T = 0xffffffff;
}
pub const INT8_ERROR: INT8 = -1;
pub const UINT8_ERROR: UINT8 = 0xff;
pub const BYTE_ERROR: BYTE = 0xff;
pub const SHORT_ERROR: SHORT = -1;
pub const INT16_ERROR: INT16 = -1;
pub const USHORT_ERROR: USHORT = 0xffff;
pub const UINT16_ERROR: UINT16 = 0xffff;
pub const WORD_ERROR: WORD = 0xffff;
pub const INT_ERROR: INT = -1;
pub const INT32_ERROR: INT32 = -1;
pub const UINT_ERROR: UINT = 0xffffffff;
pub const UINT32_ERROR: UINT32 = 0xffffffff;
pub const LONG_ERROR: LONG = -1;
pub const ULONG_ERROR: ULONG = 0xffffffff;
pub const DWORD_ERROR: DWORD = 0xffffffff;
pub const LONGLONG_ERROR: LONGLONG = -1;
pub const LONG64_ERROR: LONG64 = -1;
pub const INT64_ERROR: INT64 = -1;
pub const ULONGLONG_ERROR: ULONGLONG = 0xffffffffffffffff;
pub const DWORDLONG_ERROR: DWORDLONG = 0xffffffffffffffff;
pub const ULONG64_ERROR: ULONG64 = 0xffffffffffffffff;
pub const UINT64_ERROR: UINT64 = 0xffffffffffffffff;
#[cfg(target_arch = "x86_64")]
IFDEF!{
pub const INT_PTR_ERROR: INT_PTR = -1;
pub const UINT_PTR_ERROR: UINT_PTR = 0xffffffffffffffff;
pub const LONG_PTR_ERROR: LONG_PTR = -1;
pub const ULONG_PTR_ERROR: ULONG_PTR = 0xffffffffffffffff;
pub const DWORD_PTR_ERROR: DWORD_PTR = 0xffffffffffffffff;
pub const PTRDIFF_T_ERROR: ptrdiff_t = -1;
pub const SIZE_T_ERROR: SIZE_T = 0xffffffffffffffff;
pub const SSIZE_T_ERROR: SSIZE_T = -1;
pub const _SIZE_T_ERROR: SIZE_T = 0xffffffffffffffff;
}
#[cfg(target_arch = "x86")]
IFDEF!{
pub const INT_PTR_ERROR: INT_PTR = -1;
pub const UINT_PTR_ERROR: UINT_PTR = 0xffffffff;
pub const LONG_PTR_ERROR: LONG_PTR = -1;
pub const ULONG_PTR_ERROR: ULONG_PTR = 0xffffffff;
pub const DWORD_PTR_ERROR: DWORD_PTR = 0xffffffff;
pub const PTRDIFF_T_ERROR: ptrdiff_t = -1;
pub const SIZE_T_ERROR: SIZE_T = 0xffffffff;
pub const SSIZE_T_ERROR: SSIZE_T = -1;
pub const _SIZE_T_ERROR: SIZE_T = 0xffffffff;
}
#[inline]
pub fn Int8ToUChar(i8Operand: INT8, pch: &mut UCHAR) -> HRESULT {
    if i8Operand >= 0 {
        *pch = i8Operand as UCHAR;
        S_OK
    } else {
        *pch = 0;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn Int8ToUInt8(i8Operand: INT8, pu8Result: &mut UINT8) -> HRESULT {
    if i8Operand >= 0 {
        *pu8Result = i8Operand as UINT8;
        S_OK
    } else {
        *pu8Result = UINT8_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn Int8ToUShort(i8Operand: INT8, pusResult: &mut USHORT) -> HRESULT {
    if i8Operand >= 0 {
        *pusResult = i8Operand as USHORT;
        S_OK
    } else {
        *pusResult = USHORT_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn Int8ToUInt16(i8Operand: INT8, pusResult: &mut UINT16) -> HRESULT {
    Int8ToUShort(i8Operand, pusResult)
}
#[inline]
pub fn Int8ToWord(i8Operand: INT8, pusResult: &mut WORD) -> HRESULT {
    Int8ToUShort(i8Operand, pusResult)
}
#[inline]
pub fn Int8ToUInt(i8Operand: INT8, puResult: &mut UINT) -> HRESULT {
    if i8Operand >= 0 {
        *puResult = i8Operand as UINT;
        S_OK
    } else {
        *puResult = UINT_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn Int8ToUInt32(i8Operand: INT8, puResult: &mut UINT32) -> HRESULT {
    Int8ToUInt(i8Operand, puResult)
}
#[inline]
pub fn Int8ToUIntPtr(i8Operand: INT8, puResult: &mut UINT_PTR) -> HRESULT {
    if i8Operand >= 0 {
        *puResult = i8Operand as UINT_PTR;
        S_OK
    } else {
        *puResult = UINT_PTR_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn Int8ToULong(i8Operand: INT8, pulResult: &mut ULONG) -> HRESULT {
    if i8Operand >= 0 {
        *pulResult = i8Operand as ULONG;
        S_OK
    } else {
        *pulResult = ULONG_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn Int8ToULongPtr(i8Operand: INT8, pulResult: &mut ULONG_PTR) -> HRESULT {
    if i8Operand >= 0 {
        *pulResult = i8Operand as ULONG_PTR;
        S_OK
    } else {
        *pulResult = ULONG_PTR_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn Int8ToDWord(i8Operand: INT8, puResult: &mut DWORD) -> HRESULT {
    Int8ToULong(i8Operand, puResult)
}
#[inline]
pub fn Int8ToDWordPtr(i8Operand: INT8, puResult: &mut DWORD_PTR) -> HRESULT {
    Int8ToULongPtr(i8Operand, puResult)
}
#[inline]
pub fn Int8ToULongLong(i8Operand: INT8, pullResult: &mut ULONGLONG) -> HRESULT {
    if i8Operand >= 0 {
        *pullResult = i8Operand as ULONGLONG;
        S_OK
    } else {
        *pullResult = ULONGLONG_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn Int8ToDWordLong(i8Operand: INT8, pullResult: &mut DWORDLONG) -> HRESULT {
    Int8ToULongLong(i8Operand, pullResult)
}
#[inline]
pub fn Int8ToULong64(i8Operand: INT8, pullResult: &mut ULONG64) -> HRESULT {
    Int8ToULongLong(i8Operand, pullResult)
}
#[inline]
pub fn Int8ToDWord64(i8Operand: INT8, pullResult: &mut DWORD64) -> HRESULT {
    Int8ToULongLong(i8Operand, pullResult)
}
#[inline]
pub fn Int8ToUInt64(i8Operand: INT8, pullResult: &mut UINT64) -> HRESULT {
    Int8ToULongLong(i8Operand, pullResult)
}
#[inline]
pub fn Int8ToSizeT(i8Operand: INT8, puResult: &mut size_t) -> HRESULT {
    Int8ToUIntPtr(i8Operand, puResult)
}
#[inline]
pub fn Int8ToSIZET(i8Operand: INT8, pulResult: &mut SIZE_T) -> HRESULT {
    Int8ToULongPtr(i8Operand, pulResult)
}
#[inline]
pub fn UInt8ToInt8(u8Operand: UINT8, pi8Result: &mut INT8) -> HRESULT {
    if u8Operand <= INT8_MAX as UINT8 {
        *pi8Result = u8Operand as INT8;
        S_OK
    } else {
        *pi8Result = INT8_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn UInt8ToChar(u8Operand: UINT8, pch: &mut CHAR) -> HRESULT {
    UInt8ToInt8(u8Operand, pch)
}
#[inline]
pub fn ByteToInt8(bOperand: BYTE, pi8Result: &mut INT8) -> HRESULT {
    if bOperand <= INT8_MAX as BYTE {
        *pi8Result = bOperand as INT8;
        S_OK
    } else {
        *pi8Result = INT8_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn ByteToChar(bOperand: BYTE, pch: &mut CHAR) -> HRESULT {
    ByteToInt8(bOperand, pch)
}
#[inline]
pub fn ShortToInt8(sOperand: SHORT, pi8Result: &mut INT8) -> HRESULT {
    if (sOperand >= INT8_MIN as SHORT) && (sOperand <= INT8_MAX as SHORT) {
        *pi8Result = sOperand as INT8;
        S_OK
    } else {
        *pi8Result = INT8_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn ShortToUChar(sOperand: SHORT, pch: &mut UCHAR) -> HRESULT {
    if (sOperand >= 0) && (sOperand <= 255) {
        *pch = sOperand as UCHAR;
        S_OK
    } else {
        *pch = 0;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn ShortToChar(sOperand: SHORT, pch: &mut CHAR) -> HRESULT {
    ShortToInt8(sOperand, pch)
}
#[inline]
pub fn ShortToUInt8(sOperand: SHORT, pui8Result: &mut UINT8) -> HRESULT {
    if (sOperand >= 0) && (sOperand <= UINT8_MAX as SHORT) {
        *pui8Result = sOperand as UINT8;
        S_OK
    } else {
        *pui8Result = UINT8_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn ShortToByte(sOperand: SHORT, pch: &mut BYTE) -> HRESULT {
    ShortToUInt8(sOperand, pch)
}
#[inline]
pub fn ShortToUShort(sOperand: SHORT, pusResult: &mut USHORT) -> HRESULT {
    if sOperand >= 0 {
        *pusResult = sOperand as USHORT;
        S_OK
    } else {
        *pusResult = USHORT_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn ShortToUInt16(sOperand: SHORT, pusResult: &mut UINT16) -> HRESULT {
    ShortToUShort(sOperand, pusResult)
}
#[inline]
pub fn ShortToWord(sOperand: SHORT, pusResult: &mut WORD) -> HRESULT {
    ShortToUShort(sOperand, pusResult)
}
#[inline]
pub fn ShortToUInt(sOperand: SHORT, puResult: &mut UINT) -> HRESULT {
    if sOperand >= 0 {
        *puResult = sOperand as UINT;
        S_OK
    } else {
        *puResult = UINT_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn ShortToUInt32(sOperand: SHORT, puResult: &mut UINT32) -> HRESULT {
    ShortToUInt(sOperand, puResult)
}
#[inline]
pub fn ShortToUIntPtr(sOperand: SHORT, puResult: &mut UINT_PTR) -> HRESULT {
    if sOperand >= 0 {
        *puResult = sOperand as UINT_PTR;
        S_OK
    } else {
        *puResult = UINT_PTR_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn ShortToULong(sOperand: SHORT, pulResult: &mut ULONG) -> HRESULT {
    if sOperand >= 0 {
        *pulResult = sOperand as ULONG;
        S_OK
    } else {
        *pulResult = ULONG_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn ShortToULongPtr(sOperand: SHORT, pulResult: &mut ULONG_PTR) -> HRESULT {
    if sOperand >= 0 {
        *pulResult = sOperand as ULONG_PTR;
        S_OK
    } else {
        *pulResult = ULONG_PTR_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn ShortToDWord(sOperand: SHORT, puResult: &mut DWORD) -> HRESULT {
    ShortToULong(sOperand, puResult)
}
#[inline]
pub fn ShortToDWordPtr(sOperand: SHORT, pdwResult: &mut DWORD_PTR) -> HRESULT {
    if sOperand >= 0 {
        *pdwResult = sOperand as DWORD_PTR;
        S_OK
    } else {
        *pdwResult = DWORD_PTR_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn ShortToULongLong(sOperand: SHORT, pullResult: &mut ULONGLONG) -> HRESULT {
    if sOperand >= 0 {
        *pullResult = sOperand as ULONGLONG;
        S_OK
    } else {
        *pullResult = ULONGLONG_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn ShortToDWordLong(sOperand: SHORT, pdwResult: &mut DWORDLONG) -> HRESULT {
    ShortToULongLong(sOperand, pdwResult)
}
#[inline]
pub fn ShortToULong64(sOperand: SHORT, pullResult: &mut ULONG64) -> HRESULT {
    ShortToULongLong(sOperand, pullResult)
}
#[inline]
pub fn ShortToDWord64(sOperand: SHORT, pdwResult: &mut DWORD64) -> HRESULT {
    ShortToULongLong(sOperand, pdwResult)
}
#[inline]
pub fn ShortToUInt64(sOperand: SHORT, puResult: &mut UINT64) -> HRESULT {
    ShortToULongLong(sOperand, puResult)
}
#[inline]
pub fn ShortToSizeT(sOperand: SHORT, puResult: &mut size_t) -> HRESULT {
    ShortToUIntPtr(sOperand, puResult)
}
#[inline]
pub fn ShortToSIZET(sOperand: SHORT, puResult: &mut SIZE_T) -> HRESULT {
    ShortToULongPtr(sOperand, puResult)
}
#[inline]
pub fn Int16ToChar(i16Operand: INT16, pch: &mut CHAR) -> HRESULT {
    ShortToChar(i16Operand, pch)
}
#[inline]
pub fn Int16ToInt8(i16Operand: INT16, pi8Result: &mut INT8) -> HRESULT {
    ShortToInt8(i16Operand, pi8Result)
}
#[inline]
pub fn Int16ToUChar(i16Operand: INT16, pch: &mut UCHAR) -> HRESULT {
    ShortToUChar(i16Operand, pch)
}
#[inline]
pub fn Int16ToUInt8(i16Operand: INT16, pu8Result: &mut UINT8) -> HRESULT {
    ShortToUInt8(i16Operand, pu8Result)
}
#[inline]
pub fn Int16ToByte(i16Operand: INT16, pch: &mut BYTE) -> HRESULT {
    ShortToUInt8(i16Operand, pch)
}
#[inline]
pub fn Int16ToUShort(i16Operand: INT16, pusResult: &mut USHORT) -> HRESULT {
    ShortToUShort(i16Operand, pusResult)
}
#[inline]
pub fn Int16ToUInt16(i16Operand: INT16, pu16Result: &mut UINT16) -> HRESULT {
    ShortToUShort(i16Operand, pu16Result)
}
#[inline]
pub fn Int16ToWord(i16Operand: INT16, pwResult: &mut WORD) -> HRESULT {
    ShortToUShort(i16Operand, pwResult)
}
#[inline]
pub fn Int16ToUInt(i16Operand: INT16, puResult: &mut UINT) -> HRESULT {
    ShortToUInt(i16Operand, puResult)
}
#[inline]
pub fn Int16ToUInt32(i16Operand: INT16, puResult: &mut UINT32) -> HRESULT {
    ShortToUInt(i16Operand, puResult)
}
#[inline]
pub fn Int16ToUIntPtr(i16Operand: INT16, puResult: &mut UINT_PTR) -> HRESULT {
    ShortToUIntPtr(i16Operand, puResult)
}
#[inline]
pub fn Int16ToULong(i16Operand: INT16, pulResult: &mut ULONG) -> HRESULT {
    ShortToULong(i16Operand, pulResult)
}
#[inline]
pub fn Int16ToULongPtr(i16Operand: INT16, pulResult: &mut ULONG_PTR) -> HRESULT {
    ShortToULongPtr(i16Operand, pulResult)
}
#[inline]
pub fn Int16ToDWord(i16Operand: INT16, pdwResult: &mut DWORD) -> HRESULT {
    ShortToULong(i16Operand, pdwResult)
}
#[inline]
pub fn Int16ToDWordPtr(i16Operand: INT16, pdwResult: &mut DWORD_PTR) -> HRESULT {
    ShortToULongPtr(i16Operand, pdwResult)
}
#[inline]
pub fn Int16ToULongLong(i16Operand: INT16, pullResult: &mut ULONGLONG) -> HRESULT {
    ShortToULongLong(i16Operand, pullResult)
}
#[inline]
pub fn Int16ToDWordLong(i16Operand: INT16, pdwResult: &mut DWORDLONG) -> HRESULT {
    ShortToULongLong(i16Operand, pdwResult)
}
#[inline]
pub fn Int16ToULong64(i16Operand: INT16, pullResult: &mut ULONG64) -> HRESULT {
    ShortToULongLong(i16Operand, pullResult)
}
#[inline]
pub fn Int16ToDWord64(i16Operand: INT16, pdwResult: &mut DWORD64) -> HRESULT {
    ShortToULongLong(i16Operand, pdwResult)
}
#[inline]
pub fn Int16ToUInt64(i16Operand: INT16, puResult: &mut UINT64) -> HRESULT {
    ShortToULongLong(i16Operand, puResult)
}
#[inline]
pub fn Int16ToSizeT(i16Operand: INT16, puResult: &mut size_t) -> HRESULT {
    ShortToUIntPtr(i16Operand, puResult)
}
#[inline]
pub fn Int16ToSIZET(i16Operand: INT16, puResult: &mut SIZE_T) -> HRESULT {
    ShortToULongPtr(i16Operand, puResult)
}
#[inline]
pub fn UShortToInt8(usOperand: USHORT, pi8Result: &mut INT8) -> HRESULT {
    if usOperand <= INT8_MAX as USHORT {
        *pi8Result = usOperand as INT8;
        S_OK
    } else {
        *pi8Result = INT8_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn UShortToUChar(usOperand: USHORT, pch: &mut UCHAR) -> HRESULT {
    if usOperand <= 255 {
        *pch = usOperand as UCHAR;
        S_OK
    } else {
        *pch = 0;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn UShortToChar(usOperand: USHORT, pch: &mut CHAR) -> HRESULT {
    UShortToInt8(usOperand, pch)
}
#[inline]
pub fn UShortToUInt8(usOperand: USHORT, pui8Result: &mut UINT8) -> HRESULT {
    if usOperand <= UINT8_MAX as USHORT {
        *pui8Result = usOperand as UINT8;
        S_OK
    } else {
        *pui8Result = UINT8_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn UShortToByte(usOperand: USHORT, pch: &mut BYTE) -> HRESULT {
    UShortToUInt8(usOperand, pch)
}
#[inline]
pub fn UShortToShort(usOperand: USHORT, psResult: &mut SHORT) -> HRESULT {
    if usOperand <= SHORT_MAX as USHORT {
        *psResult = usOperand as SHORT;
        S_OK
    } else {
        *psResult = SHORT_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn UShortToInt16(usOperand: USHORT, pi16Result: &mut INT16) -> HRESULT {
    UShortToShort(usOperand, pi16Result)
}
#[inline]
pub fn UInt16ToChar(u16Operand: UINT16, pch: &mut CHAR) -> HRESULT {
    UShortToChar(u16Operand, pch)
}
#[inline]
pub fn UInt16ToInt8(u16Operand: UINT16, pi8Result: &mut INT8) -> HRESULT {
    UShortToInt8(u16Operand, pi8Result)
}
#[inline]
pub fn UInt16ToUChar(u16Operand: UINT16, pch: &mut UCHAR) -> HRESULT {
    UShortToUChar(u16Operand, pch)
}
#[inline]
pub fn UInt16ToUInt8(u16Operand: UINT16, pu8Result: &mut UINT8) -> HRESULT {
    UShortToUInt8(u16Operand, pu8Result)
}
#[inline]
pub fn UInt16ToByte(u16Operand: UINT16, pu8Result: &mut BYTE) -> HRESULT {
    UShortToUInt8(u16Operand, pu8Result)
}
#[inline]
pub fn UInt16ToShort(u16Operand: UINT16, psResult: &mut SHORT) -> HRESULT {
    UShortToShort(u16Operand, psResult)
}
#[inline]
pub fn UInt16ToInt16(u16Operand: UINT16, pi16Result: &mut INT16) -> HRESULT {
    UShortToShort(u16Operand, pi16Result)
}
#[inline]
pub fn WordToInt8(wOperand: WORD, pi8Result: &mut INT8) -> HRESULT {
    UShortToInt8(wOperand, pi8Result)
}
#[inline]
pub fn WordToChar(wOperand: WORD, pch: &mut CHAR) -> HRESULT {
    UShortToChar(wOperand, pch)
}
#[inline]
pub fn WordToUChar(wOperand: WORD, pch: &mut UCHAR) -> HRESULT {
    UShortToUChar(wOperand, pch)
}
#[inline]
pub fn WordToUInt8(wOperand: WORD, pu8Result: &mut UINT8) -> HRESULT {
    UShortToUInt8(wOperand, pu8Result)
}
#[inline]
pub fn WordToByte(wOperand: WORD, pch: &mut BYTE) -> HRESULT {
    UShortToUInt8(wOperand, pch)
}
#[inline]
pub fn WordToShort(wOperand: WORD, psResult: &mut SHORT) -> HRESULT {
    UShortToShort(wOperand, psResult)
}
#[inline]
pub fn WordToInt16(wOperand: WORD, pi16Result: &mut INT16) -> HRESULT {
    UShortToShort(wOperand, pi16Result)
}
#[inline]
pub fn IntToInt8(iOperand: INT, pi8Result: &mut INT8) -> HRESULT {
    if (iOperand >= INT8_MIN as INT) && (iOperand <= INT8_MAX as INT) {
        *pi8Result = iOperand as INT8;
        S_OK
    } else {
        *pi8Result = INT8_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn IntToUChar(iOperand: INT, pch: &mut UCHAR) -> HRESULT {
    if (iOperand >= 0) && (iOperand <= 255) {
        *pch = iOperand as UCHAR;
        S_OK
    } else {
        *pch = 0;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn IntToChar(iOperand: INT, pch: &mut CHAR) -> HRESULT {
    IntToInt8(iOperand, pch)
}
#[inline]
pub fn IntToByte(iOperand: INT, pu8Result: &mut BYTE) -> HRESULT {
    IntToUInt8(iOperand, pu8Result)
}
#[inline]
pub fn IntToUInt8(iOperand: INT, pu8Result: &mut UINT8) -> HRESULT {
    if (iOperand >= 0) && (iOperand <= UINT8_MAX as INT) {
        *pu8Result = iOperand as UINT8;
        S_OK
    } else {
        *pu8Result = UINT8_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn IntToShort(iOperand: INT, psResult: &mut SHORT) -> HRESULT {
    if (iOperand >= SHORT_MIN as INT) && (iOperand <= SHORT_MAX as INT) {
        *psResult = iOperand as SHORT;
        S_OK
    } else {
        *psResult = SHORT_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn IntToInt16(iOperand: INT, pi16Result: &mut INT16) -> HRESULT {
    IntToShort(iOperand, pi16Result)
}
#[inline]
pub fn IntToUShort(iOperand: INT, pusResult: &mut USHORT) -> HRESULT {
    if (iOperand >= 0) && (iOperand <= USHORT_MAX as INT) {
        *pusResult = iOperand as USHORT;
        S_OK
    } else {
        *pusResult = USHORT_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn IntToUInt16(iOperand: INT, pu16Result: &mut UINT16) -> HRESULT {
    IntToUShort(iOperand, pu16Result)
}
#[inline]
pub fn IntToWord(iOperand: INT, pwResult: &mut WORD) -> HRESULT {
    IntToUShort(iOperand, pwResult)
}
#[inline]
pub fn IntToUInt(iOperand: INT, puResult: &mut UINT) -> HRESULT {
    if iOperand >= 0 {
        *puResult = iOperand as UINT;
        S_OK
    } else {
        *puResult = UINT_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[cfg(target_arch = "x86_64")] #[inline]
pub fn IntToUIntPtr(iOperand: INT, puResult: &mut UINT_PTR) -> HRESULT {
    IntToULongLong(iOperand, puResult)
}
#[cfg(target_arch = "x86")] #[inline]
pub fn IntToUIntPtr(iOperand: INT, puResult: &mut UINT_PTR) -> HRESULT {
    IntToUInt(iOperand, puResult)
}
#[inline]
pub fn IntToULong(iOperand: INT, pulResult: &mut ULONG) -> HRESULT {
    if iOperand >= 0 {
        *pulResult = iOperand as ULONG;
        S_OK
    } else {
        *pulResult = ULONG_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[cfg(target_arch = "x86_64")] #[inline]
pub fn IntToULongPtr(iOperand: INT, pulResult: &mut ULONG_PTR) -> HRESULT {
    IntToULongLong(iOperand, pulResult)
}
#[cfg(target_arch = "x86")] #[inline]
pub fn IntToULongPtr(iOperand: INT, pulResult: &mut ULONG_PTR) -> HRESULT {
    IntToULong(iOperand, pulResult)
}
#[inline]
pub fn IntToDWord(iOperand: INT, pdwResult: &mut DWORD) -> HRESULT {
    IntToULong(iOperand, pdwResult)
}
#[inline]
pub fn IntToDWordPtr(iOperand: INT, pdwResult: &mut DWORD_PTR) -> HRESULT {
    IntToULongPtr(iOperand, pdwResult)
}
#[inline]
pub fn IntToULongLong(iOperand: INT, pullResult: &mut ULONGLONG) -> HRESULT {
    if iOperand >= 0 {
        *pullResult = iOperand as ULONGLONG;
        S_OK
    } else {
        *pullResult = ULONGLONG_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn IntToDWordLong(iOperand: INT, pdwResult: &mut DWORDLONG) -> HRESULT {
    IntToULongLong(iOperand, pdwResult)
}
#[inline]
pub fn IntToULong64(iOperand: INT, pullResult: &mut ULONG64) -> HRESULT {
    IntToULongLong(iOperand, pullResult)
}
#[inline]
pub fn IntToDWord64(iOperand: INT, pdwResult: &mut DWORD64) -> HRESULT {
    IntToULongLong(iOperand, pdwResult)
}
#[inline]
pub fn IntToUInt64(iOperand: INT, puResult: &mut UINT64) -> HRESULT {
    IntToULongLong(iOperand, puResult)
}
#[inline]
pub fn IntToSizeT(iOperand: INT, puResult: &mut size_t) -> HRESULT {
    IntToUIntPtr(iOperand, puResult)
}
#[inline]
pub fn IntToSIZET(iOperand: INT, puResult: &mut SIZE_T) -> HRESULT {
    IntToULongPtr(iOperand, puResult)
}
#[inline]
pub fn Int32ToChar(i32Operand: INT32, pch: &mut CHAR) -> HRESULT {
    IntToChar(i32Operand, pch)
}
#[inline]
pub fn Int32ToInt8(i32Operand: INT32, pi8Result: &mut INT8) -> HRESULT {
    IntToInt8(i32Operand, pi8Result)
}
#[inline]
pub fn Int32ToUChar(i32Operand: INT32, pch: &mut UCHAR) -> HRESULT {
    IntToUChar(i32Operand, pch)
}
#[inline]
pub fn Int32ToByte(i32Operand: INT32, pu8Result: &mut BYTE) -> HRESULT {
    IntToUInt8(i32Operand, pu8Result)
}
#[inline]
pub fn Int32ToUInt8(i32Operand: INT32, pu8Result: &mut UINT8) -> HRESULT {
    IntToUInt8(i32Operand, pu8Result)
}
#[inline]
pub fn Int32ToShort(i32Operand: INT32, psResult: &mut SHORT) -> HRESULT {
    IntToShort(i32Operand, psResult)
}
#[inline]
pub fn Int32ToInt16(i32Operand: INT32, pi16Result: &mut INT16) -> HRESULT {
    IntToShort(i32Operand, pi16Result)
}
#[inline]
pub fn Int32ToUShort(i32Operand: INT32, pusResult: &mut USHORT) -> HRESULT {
    IntToUShort(i32Operand, pusResult)
}
#[inline]
pub fn Int32ToUInt16(i32Operand: INT32, pu16Result: &mut UINT16) -> HRESULT {
    IntToUShort(i32Operand, pu16Result)
}
#[inline]
pub fn Int32ToWord(i32Operand: INT32, pwResult: &mut WORD) -> HRESULT {
    IntToUShort(i32Operand, pwResult)
}
#[inline]
pub fn Int32ToUInt(i32Operand: INT32, puResult: &mut UINT) -> HRESULT {
    IntToUInt(i32Operand, puResult)
}
#[inline]
pub fn Int32ToUInt32(i32Operand: INT32, pu32Result: &mut UINT32) -> HRESULT {
    IntToUInt(i32Operand, pu32Result)
}
#[inline]
pub fn Int32ToUIntPtr(i32Operand: INT32, puResult: &mut UINT_PTR) -> HRESULT {
    IntToUIntPtr(i32Operand, puResult)
}
#[inline]
pub fn Int32ToULong(i32Operand: INT32, pulResult: &mut ULONG) -> HRESULT {
    IntToULong(i32Operand, pulResult)
}
#[inline]
pub fn Int32ToULongPtr(i32Operand: INT32, pulResult: &mut ULONG_PTR) -> HRESULT {
    IntToULongPtr(i32Operand, pulResult)
}
#[inline]
pub fn Int32ToDWord(i32Operand: INT32, pdwResult: &mut DWORD) -> HRESULT {
    IntToULong(i32Operand, pdwResult)
}
#[inline]
pub fn Int32ToDWordPtr(i32Operand: INT32, pdwResult: &mut DWORD_PTR) -> HRESULT {
    IntToULongPtr(i32Operand, pdwResult)
}
#[inline]
pub fn Int32ToULongLong(i32Operand: INT32, pullResult: &mut ULONGLONG) -> HRESULT {
    IntToULongLong(i32Operand, pullResult)
}
#[inline]
pub fn Int32ToDWordLong(i32Operand: INT32, pdwResult: &mut DWORDLONG) -> HRESULT {
    IntToULongLong(i32Operand, pdwResult)
}
#[inline]
pub fn Int32ToULong64(i32Operand: INT32, pullResult: &mut ULONG64) -> HRESULT {
    IntToULongLong(i32Operand, pullResult)
}
#[inline]
pub fn Int32ToDWord64(i32Operand: INT32, pdwResult: &mut DWORD64) -> HRESULT {
    IntToULongLong(i32Operand, pdwResult)
}
#[inline]
pub fn Int32ToUInt64(i32Operand: INT32, puResult: &mut UINT64) -> HRESULT {
    IntToULongLong(i32Operand, puResult)
}
#[inline]
pub fn Int32ToSizeT(i32Operand: INT32, puResult: &mut size_t) -> HRESULT {
    IntToUIntPtr(i32Operand, puResult)
}
#[inline]
pub fn Int32ToSIZET(i32Operand: INT32, puResult: &mut SIZE_T) -> HRESULT {
    IntToULongPtr(i32Operand, puResult)
}
#[inline]
pub fn IntPtrToInt8(iOperand: INT_PTR, pi8Result: &mut INT8) -> HRESULT {
    if (iOperand >= INT8_MIN as INT_PTR) && (iOperand <= INT8_MAX as INT_PTR) {
        *pi8Result = iOperand as INT8;
        S_OK
    } else {
        *pi8Result = INT8_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn IntPtrToUChar(iOperand: INT_PTR, pch: &mut UCHAR) -> HRESULT {
    if (iOperand >= 0) && (iOperand <= 255) {
        *pch = iOperand as UCHAR;
        S_OK
    } else {
        *pch = 0;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn IntPtrToChar(iOperand: INT_PTR, pch: &mut CHAR) -> HRESULT {
    IntPtrToInt8(iOperand, pch)
}
#[inline]
pub fn IntPtrToUInt8(iOperand: INT_PTR, pui8Result: &mut UINT8) -> HRESULT {
    if (iOperand >= 0) && (iOperand <= UINT8_MAX as INT_PTR) {
        *pui8Result = iOperand as UINT8;
        S_OK
    } else {
        *pui8Result = UINT8_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn IntPtrToByte(iOperand: INT_PTR, pu8Result: &mut BYTE) -> HRESULT {
    IntPtrToUInt8(iOperand, pu8Result)
}
#[inline]
pub fn IntPtrToShort(iOperand: INT_PTR, psResult: &mut SHORT) -> HRESULT {
    if (iOperand >= SHORT_MIN as INT_PTR) && (iOperand <= SHORT_MAX as INT_PTR) {
        *psResult = iOperand as SHORT;
        S_OK
    } else {
        *psResult = SHORT_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn IntPtrToInt16(iOperand: INT_PTR, pi16Result: &mut INT16) -> HRESULT {
    IntPtrToShort(iOperand, pi16Result)
}
#[inline]
pub fn IntPtrToUShort(iOperand: INT_PTR, pusResult: &mut USHORT) -> HRESULT {
    if (iOperand >= 0) && (iOperand <= USHORT_MAX as INT_PTR) {
        *pusResult = iOperand as USHORT;
        S_OK
    } else {
        *pusResult = USHORT_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn IntPtrToUInt16(iOperand: INT_PTR, pu16Result: &mut UINT16) -> HRESULT {
    IntPtrToUShort(iOperand, pu16Result)
}
#[inline]
pub fn IntPtrToWord(iOperand: INT_PTR, pwResult: &mut WORD) -> HRESULT {
    IntPtrToUShort(iOperand, pwResult)
}
#[cfg(target_arch = "x86_64")] #[inline]
pub fn IntPtrToInt(iOperand: INT_PTR, piResult: &mut INT) -> HRESULT {
    LongLongToInt(iOperand, piResult)
}
#[cfg(target_arch = "x86")] #[inline]
pub fn IntPtrToInt(iOperand: INT_PTR, piResult: &mut INT) -> HRESULT {
    *piResult = iOperand;
    S_OK
}
#[inline]
pub fn IntPtrToInt32(iOperand: INT_PTR, pi32Result: &mut INT32) -> HRESULT {
    IntPtrToInt(iOperand, pi32Result)
}
#[cfg(target_arch = "x86_64")] #[inline]
pub fn IntPtrToUInt(iOperand: INT_PTR, puResult: &mut UINT) -> HRESULT {
    LongLongToUInt(iOperand, puResult)
}
#[cfg(target_arch = "x86")] #[inline]
pub fn IntPtrToUInt(iOperand: INT_PTR, puResult: &mut UINT) -> HRESULT {
    if iOperand >= 0 {
        *puResult = iOperand as UINT;
        S_OK
    } else {
        *puResult = UINT_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn IntPtrToUInt32(iOperand: INT_PTR, pu32Result: &mut UINT32) -> HRESULT {
    IntPtrToUInt(iOperand, pu32Result)
}
#[cfg(target_arch = "x86_64")] #[inline]
pub fn IntPtrToUIntPtr(iOperand: INT_PTR, puResult: &mut UINT_PTR) -> HRESULT {
    LongLongToULongLong(iOperand, puResult)
}
#[cfg(target_arch = "x86")] #[inline]
pub fn IntPtrToUIntPtr(iOperand: INT_PTR, puResult: &mut UINT_PTR) -> HRESULT {
    if iOperand >= 0 {
        *puResult = iOperand as UINT_PTR;
        S_OK
    } else {
        *puResult = UINT_PTR_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[cfg(target_arch = "x86_64")] #[inline]
pub fn IntPtrToLong(iOperand: INT_PTR, plResult: &mut LONG) -> HRESULT {
    LongLongToLong(iOperand, plResult)
}
#[cfg(target_arch = "x86")] #[inline]
pub fn IntPtrToLong(iOperand: INT_PTR, plResult: &mut LONG) -> HRESULT {
    *plResult = iOperand;
    S_OK
}
#[inline]
pub fn IntPtrToLongPtr(iOperand: INT_PTR, plResult: &mut LONG_PTR) -> HRESULT {
    *plResult = iOperand as LONG_PTR;
    S_OK
}
#[cfg(target_arch = "x86_64")] #[inline]
pub fn IntPtrToULong(iOperand: INT_PTR, pulResult: &mut ULONG) -> HRESULT {
    LongLongToULong(iOperand, pulResult)
}
#[cfg(target_arch = "x86")] #[inline]
pub fn IntPtrToULong(iOperand: INT_PTR, pulResult: &mut ULONG) -> HRESULT {
    if iOperand >= 0 {
        *pulResult = iOperand as ULONG;
        S_OK
    } else {
        *pulResult = ULONG_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[cfg(target_arch = "x86_64")] #[inline]
pub fn IntPtrToULongPtr(iOperand: INT_PTR, pulResult: &mut ULONG_PTR) -> HRESULT {
    LongLongToULongLong(iOperand, pulResult)
}
#[cfg(target_arch = "x86")] #[inline]
pub fn IntPtrToULongPtr(iOperand: INT_PTR, pulResult: &mut ULONG_PTR) -> HRESULT {
    if iOperand >= 0 {
        *pulResult = iOperand as ULONG_PTR;
        S_OK
    } else {
        *pulResult = ULONG_PTR_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn IntPtrToDWord(iOperand: INT_PTR, pdwResult: &mut DWORD) -> HRESULT {
    IntPtrToULong(iOperand, pdwResult)
}
#[inline]
pub fn IntPtrToDWordPtr(iOperand: INT_PTR, pdwResult: &mut DWORD_PTR) -> HRESULT {
    IntPtrToULongPtr(iOperand, pdwResult)
}
#[cfg(target_arch = "x86_64")] #[inline]
pub fn IntPtrToULongLong(iOperand: INT_PTR, pullResult: &mut ULONGLONG) -> HRESULT {
    LongLongToULongLong(iOperand, pullResult)
}
#[cfg(target_arch = "x86")] #[inline]
pub fn IntPtrToULongLong(iOperand: INT_PTR, pullResult: &mut ULONGLONG) -> HRESULT {
    if iOperand >= 0 {
        *pullResult = iOperand as ULONGLONG;
        S_OK
    } else {
        *pullResult = ULONGLONG_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn IntPtrToDWordLong(iOperand: INT_PTR, pdwResult: &mut DWORDLONG) -> HRESULT {
    IntPtrToULongLong(iOperand, pdwResult)
}
#[inline]
pub fn IntPtrToULong64(iOperand: INT_PTR, pullResult: &mut ULONG64) -> HRESULT {
    IntPtrToULongLong(iOperand, pullResult)
}
#[inline]
pub fn IntPtrToDWord64(iOperand: INT_PTR, pdwResult: &mut DWORD64) -> HRESULT {
    IntPtrToULongLong(iOperand, pdwResult)
}
#[inline]
pub fn IntPtrToUInt64(iOperand: INT_PTR, pu64Result: &mut UINT64) -> HRESULT {
    IntPtrToULongLong(iOperand, pu64Result)
}
#[inline]
pub fn IntPtrToSizeT(iOperand: INT_PTR, puResult: &mut size_t) -> HRESULT {
    IntPtrToUIntPtr(iOperand, puResult)
}
#[inline]
pub fn IntPtrToSIZET(iOperand: INT_PTR, puResult: &mut SIZE_T) -> HRESULT {
    IntPtrToULongPtr(iOperand, puResult)
}
#[inline]
pub fn UIntToInt8(uOperand: UINT, pi8Result: &mut INT8) -> HRESULT {
    if uOperand <= INT8_MAX as UINT {
        *pi8Result = uOperand as INT8;
        S_OK
    } else {
        *pi8Result = INT8_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn UIntToUChar(uOperand: UINT, pch: &mut UCHAR) -> HRESULT {
    if uOperand <= 255 {
        *pch = uOperand as UCHAR;
        S_OK
    } else {
        *pch = 0;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn UIntToChar(uOperand: UINT, pch: &mut CHAR) -> HRESULT {
    UIntToInt8(uOperand, pch)
}
#[inline]
pub fn UIntToUInt8(uOperand: UINT, pui8Result: &mut UINT8) -> HRESULT {
    if uOperand <= UINT8_MAX as UINT {
        *pui8Result = uOperand as UINT8;
        S_OK
    } else {
        *pui8Result = UINT8_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn UIntToByte(uOperand: UINT, pui8Result: &mut BYTE) -> HRESULT {
    UIntToUInt8(uOperand, pui8Result)
}
#[inline]
pub fn UIntToShort(uOperand: UINT, psResult: &mut SHORT) -> HRESULT {
    if uOperand <= SHORT_MAX as UINT {
        *psResult = uOperand as SHORT;
        S_OK
    } else {
        *psResult = SHORT_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn UIntToInt16(uOperand: UINT, pi16Result: &mut INT16) -> HRESULT {
    UIntToShort(uOperand, pi16Result)
}
#[inline]
pub fn UIntToUShort(uOperand: UINT, pusResult: &mut USHORT) -> HRESULT {
    if uOperand <= USHORT_MAX as UINT {
        *pusResult = uOperand as USHORT;
        S_OK
    } else {
        *pusResult = USHORT_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn UIntToUInt16(uOperand: UINT, pui16Result: &mut UINT16) -> HRESULT {
    UIntToUShort(uOperand, pui16Result)
}
#[inline]
pub fn UIntToWord(uOperand: UINT, pwResult: &mut WORD) -> HRESULT {
    UIntToUShort(uOperand, pwResult)
}
#[inline]
pub fn UIntToInt(uOperand: UINT, piResult: &mut INT) -> HRESULT {
    if uOperand <= INT_MAX as UINT {
        *piResult = uOperand as INT;
        S_OK
    } else {
        *piResult = INT_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn UIntToInt32(uOperand: UINT, pi32Result: &mut INT32) -> HRESULT {
    UIntToInt(uOperand, pi32Result)
}
#[cfg(target_arch = "x86_64")] #[inline]
pub fn UIntToIntPtr(uOperand: UINT, piResult: &mut INT_PTR) -> HRESULT {
    *piResult = uOperand as INT_PTR;
    S_OK
}
#[cfg(target_arch = "x86")] #[inline]
pub fn UIntToIntPtr(uOperand: UINT, piResult: &mut INT_PTR) -> HRESULT {
    UIntToInt(uOperand, piResult)
}
#[inline]
pub fn UIntToLong(uOperand: UINT, plResult: &mut LONG) -> HRESULT {
    if uOperand <= LONG_MAX as UINT {
        *plResult = uOperand as LONG;
        S_OK
    } else {
        *plResult = LONG_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[cfg(target_arch = "x86_64")] #[inline]
pub fn UIntToLongPtr(uOperand: UINT, plResult: &mut LONG_PTR) -> HRESULT {
    *plResult = uOperand as LONG_PTR;
    S_OK
}
#[cfg(target_arch = "x86")] #[inline]
pub fn UIntToLongPtr(uOperand: UINT, plResult: &mut LONG_PTR) -> HRESULT {
    UIntToLong(uOperand, plResult)
}
#[inline]
pub fn UIntToPtrdiffT(uOperand: UINT, pResult: &mut ptrdiff_t) -> HRESULT {
    UIntToIntPtr(uOperand, pResult)
}
#[inline]
pub fn UIntToSSIZET(uOperand: UINT, pResult: &mut SSIZE_T) -> HRESULT {
    UIntToLongPtr(uOperand, pResult)
}
#[inline]
pub fn UInt32ToChar(ui32Operand: UINT32, pch: &mut CHAR) -> HRESULT {
    UIntToChar(ui32Operand, pch)
}
#[inline]
pub fn UInt32ToInt8(ui32Operand: UINT32, pi8Result: &mut INT8) -> HRESULT {
    UIntToInt8(ui32Operand, pi8Result)
}
#[inline]
pub fn UInt32ToUChar(ui32Operand: UINT32, pch: &mut UCHAR) -> HRESULT {
    UIntToUChar(ui32Operand, pch)
}
#[inline]
pub fn UInt32ToUInt8(ui32Operand: UINT32, pui8Result: &mut UINT8) -> HRESULT {
    UIntToUInt8(ui32Operand, pui8Result)
}
#[inline]
pub fn UInt32ToByte(ui32Operand: UINT32, pui8Result: &mut BYTE) -> HRESULT {
    UInt32ToUInt8(ui32Operand, pui8Result)
}
#[inline]
pub fn UInt32ToShort(ui32Operand: UINT32, psResult: &mut SHORT) -> HRESULT {
    UIntToShort(ui32Operand, psResult)
}
#[inline]
pub fn UInt32ToInt16(ui32Operand: UINT32, pi16Result: &mut INT16) -> HRESULT {
    UIntToShort(ui32Operand, pi16Result)
}
#[inline]
pub fn UInt32ToUShort(ui32Operand: UINT32, pusResult: &mut USHORT) -> HRESULT {
    UIntToUShort(ui32Operand, pusResult)
}
#[inline]
pub fn UInt32ToUInt16(ui32Operand: UINT32, pui16Result: &mut UINT16) -> HRESULT {
    UIntToUShort(ui32Operand, pui16Result)
}
#[inline]
pub fn UInt32ToWord(ui32Operand: UINT32, pwResult: &mut WORD) -> HRESULT {
    UIntToUShort(ui32Operand, pwResult)
}
#[inline]
pub fn UInt32ToInt(ui32Operand: UINT32, piResult: &mut INT) -> HRESULT {
    UIntToInt(ui32Operand, piResult)
}
#[inline]
pub fn UInt32ToIntPtr(ui32Operand: UINT32, piResult: &mut INT_PTR) -> HRESULT {
    UIntToIntPtr(ui32Operand, piResult)
}
#[inline]
pub fn UInt32ToInt32(ui32Operand: UINT32, pi32Result: &mut INT32) -> HRESULT {
    UIntToInt(ui32Operand, pi32Result)
}
#[inline]
pub fn UInt32ToLong(ui32Operand: UINT32, plResult: &mut LONG) -> HRESULT {
    UIntToLong(ui32Operand, plResult)
}
#[inline]
pub fn UInt32ToLongPtr(ui32Operand: UINT32, plResult: &mut LONG_PTR) -> HRESULT {
    UIntToLongPtr(ui32Operand, plResult)
}
#[inline]
pub fn UInt32ToPtrdiffT(ui32Operand: UINT32, pResult: &mut ptrdiff_t) -> HRESULT {
    UIntToPtrdiffT(ui32Operand, pResult)
}
#[inline]
pub fn UInt32ToSSIZET(ui32Operand: UINT32, pResult: &mut SSIZE_T) -> HRESULT {
    UIntToSSIZET(ui32Operand, pResult)
}
#[inline]
pub fn UIntPtrToInt8(uOperand: UINT_PTR, pi8Result: &mut INT8) -> HRESULT {
    if uOperand <= INT8_MAX as UINT_PTR {
        *pi8Result = uOperand as INT8;
        S_OK
    } else {
        *pi8Result = INT8_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn UIntPtrToUChar(uOperand: UINT_PTR, pch: &mut UCHAR) -> HRESULT {
    if uOperand <= 255 {
        *pch = uOperand as UCHAR;
        S_OK
    } else {
        *pch = 0;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn UIntPtrToChar(uOperand: UINT_PTR, pch: &mut CHAR) -> HRESULT {
    UIntPtrToInt8(uOperand, pch)
}
#[inline]
pub fn UIntPtrToUInt8(uOperand: UINT_PTR, pu8Result: &mut UINT8) -> HRESULT {
    if uOperand <= UINT8_MAX as UINT_PTR {
        *pu8Result = uOperand as UINT8;
        S_OK
    } else {
        *pu8Result = UINT8_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn UIntPtrToByte(uOperand: UINT_PTR, pu8Result: &mut BYTE) -> HRESULT {
    UIntPtrToUInt8(uOperand, pu8Result)
}
#[inline]
pub fn UIntPtrToShort(uOperand: UINT_PTR, psResult: &mut SHORT) -> HRESULT {
    if uOperand <= SHORT_MAX as UINT_PTR {
        *psResult = uOperand as SHORT;
        S_OK
    } else {
        *psResult = SHORT_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn UIntPtrToInt16(uOperand: UINT_PTR, pi16Result: &mut INT16) -> HRESULT {
    if uOperand <= INT16_MAX as UINT_PTR {
        *pi16Result = uOperand as INT16;
        S_OK
    } else {
        *pi16Result = INT16_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn UIntPtrToUShort(uOperand: UINT_PTR, pusResult: &mut USHORT) -> HRESULT {
    if uOperand <= USHORT_MAX as UINT_PTR {
        *pusResult = uOperand as USHORT;
        S_OK
    } else {
        *pusResult = USHORT_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn UIntPtrToUInt16(uOperand: UINT_PTR, pu16Result: &mut UINT16) -> HRESULT {
    if uOperand <= UINT16_MAX as UINT_PTR {
        *pu16Result = uOperand as UINT16;
        S_OK
    } else {
        *pu16Result = UINT16_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn UIntPtrToWord(uOperand: UINT_PTR, pwResult: &mut WORD) -> HRESULT {
    UIntPtrToUShort(uOperand, pwResult)
}
#[inline]
pub fn UIntPtrToInt(uOperand: UINT_PTR, piResult: &mut INT) -> HRESULT {
    if uOperand <= INT_MAX as UINT_PTR {
        *piResult = uOperand as INT;
        S_OK
    } else {
        *piResult = INT_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn UIntPtrToInt32(uOperand: UINT_PTR, pi32Result: &mut INT32) -> HRESULT {
    UIntPtrToInt(uOperand, pi32Result)
}
#[inline]
pub fn UIntPtrToIntPtr(uOperand: UINT_PTR, piResult: &mut INT_PTR) -> HRESULT {
    if uOperand <= INT_PTR_MAX as UINT_PTR {
        *piResult = uOperand as INT_PTR;
        S_OK
    } else {
        *piResult = INT_PTR_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[cfg(target_arch = "x86_64")] #[inline]
pub fn UIntPtrToUInt(uOperand: UINT_PTR, puResult: &mut UINT) -> HRESULT {
    ULongLongToUInt(uOperand, puResult)
}
#[cfg(target_arch = "x86")] #[inline]
pub fn UIntPtrToUInt(uOperand: UINT_PTR, puResult: &mut UINT) -> HRESULT {
    *puResult = uOperand;
    S_OK
}
#[inline]
pub fn UIntPtrToUInt32(uOperand: UINT_PTR, pu32Result: &mut UINT32) -> HRESULT {
    UIntPtrToUInt(uOperand, pu32Result)
}
#[inline]
pub fn UIntPtrToLong(uOperand: UINT_PTR, plResult: &mut LONG) -> HRESULT {
    if uOperand <= LONG_MAX as UINT_PTR {
        *plResult = uOperand as LONG;
        S_OK
    } else {
        *plResult = LONG_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn UIntPtrToLongPtr(uOperand: UINT_PTR, plResult: &mut LONG_PTR) -> HRESULT {
    if uOperand <= LONG_PTR_MAX as UINT_PTR {
        *plResult = uOperand as LONG_PTR;
        S_OK
    } else {
        *plResult = LONG_PTR_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[cfg(target_arch = "x86_64")] #[inline]
pub fn UIntPtrToULong(uOperand: UINT_PTR, pulResult: &mut ULONG) -> HRESULT {
    ULongLongToULong(uOperand, pulResult)
}
#[cfg(target_arch = "x86")] #[inline]
pub fn UIntPtrToULong(uOperand: UINT_PTR, pulResult: &mut ULONG) -> HRESULT {
    *pulResult = uOperand;
    S_OK
}
#[inline]
pub fn UIntPtrToDWord(uOperand: UINT_PTR, pdwResult: &mut DWORD) -> HRESULT {
    UIntPtrToULong(uOperand, pdwResult)
}
#[cfg(target_arch = "x86_64")] #[inline]
pub fn UIntPtrToLongLong(uOperand: UINT_PTR, pllResult: &mut LONGLONG) -> HRESULT {
    ULongLongToLongLong(uOperand, pllResult)
}
#[cfg(target_arch = "x86")] #[inline]
pub fn UIntPtrToLongLong(uOperand: UINT_PTR, pllResult: &mut LONGLONG) -> HRESULT {
    *pllResult = uOperand as LONGLONG;
    S_OK
}
#[inline]
pub fn UIntPtrToLong64(uOperand: UINT_PTR, pllResult: &mut LONG64) -> HRESULT {
    UIntPtrToLongLong(uOperand, pllResult)
}
#[inline]
pub fn UIntPtrToInt64(uOperand: UINT_PTR, pi64Result: &mut INT64) -> HRESULT {
    UIntPtrToLongLong(uOperand, pi64Result)
}
#[inline]
pub fn UIntPtrToPtrdiffT(uOperand: UINT_PTR, pResult: &mut ptrdiff_t) -> HRESULT {
    UIntPtrToIntPtr(uOperand, pResult)
}
#[inline]
pub fn UIntPtrToSSIZET(uOperand: UINT_PTR, pResult: &mut SSIZE_T) -> HRESULT {
    UIntPtrToLongPtr(uOperand, pResult)
}
#[inline]
pub fn LongToInt8(lOperand: LONG, pi8Result: &mut INT8) -> HRESULT {
    if (lOperand >= INT8_MIN as LONG) && (lOperand <= INT8_MAX as LONG) {
        *pi8Result = lOperand as INT8;
        S_OK
    } else {
        *pi8Result = INT8_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn LongToUChar(lOperand: LONG, pch: &mut UCHAR) -> HRESULT {
    if (lOperand >= 0) && (lOperand <= 255) {
        *pch = lOperand as UCHAR;
        S_OK
    } else {
        *pch = 0;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn LongToChar(lOperand: LONG, pch: &mut CHAR) -> HRESULT {
    LongToInt8(lOperand, pch)
}
#[inline]
pub fn LongToUInt8(lOperand: LONG, pui8Result: &mut UINT8) -> HRESULT {
    if (lOperand >= 0) && (lOperand <= UINT8_MAX as LONG) {
        *pui8Result = lOperand as UINT8;
        S_OK
    } else {
        *pui8Result = UINT8_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn LongToByte(lOperand: LONG, pui8Result: &mut BYTE) -> HRESULT {
    LongToUInt8(lOperand, pui8Result)
}
#[inline]
pub fn LongToShort(lOperand: LONG, psResult: &mut SHORT) -> HRESULT {
    if (lOperand >= SHORT_MIN as LONG) && (lOperand <= SHORT_MAX as LONG) {
        *psResult = lOperand as SHORT;
        S_OK
    } else {
        *psResult = SHORT_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn LongToInt16(lOperand: LONG, pi16Result: &mut INT16) -> HRESULT {
    LongToShort(lOperand, pi16Result)
}
#[inline]
pub fn LongToUShort(lOperand: LONG, pusResult: &mut USHORT) -> HRESULT {
    if (lOperand >= 0) && (lOperand <= USHORT_MAX as LONG) {
        *pusResult = lOperand as USHORT;
        S_OK
    } else {
        *pusResult = USHORT_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn LongToUInt16(lOperand: LONG, pui16Result: &mut UINT16) -> HRESULT {
    LongToUShort(lOperand, pui16Result)
}
#[inline]
pub fn LongToWord(lOperand: LONG, pwResult: &mut WORD) -> HRESULT {
    LongToUShort(lOperand, pwResult)
}
#[inline]
pub fn LongToInt(lOperand: LONG, piResult: &mut INT) -> HRESULT {
    *piResult = lOperand;
    S_OK
}
#[inline]
pub fn LongToInt32(lOperand: LONG, piResult: &mut INT32) -> HRESULT {
    LongToInt(lOperand, piResult)
}
#[cfg(target_arch = "x86_64")] #[inline]
pub fn LongToIntPtr(lOperand: LONG, piResult: &mut INT_PTR) -> HRESULT {
    *piResult = lOperand as INT_PTR;
    S_OK
}
#[cfg(target_arch = "x86")] #[inline]
pub fn LongToIntPtr(lOperand: LONG, piResult: &mut INT_PTR) -> HRESULT {
    LongToInt(lOperand, piResult)
}
#[inline]
pub fn LongToUInt(lOperand: LONG, puiResult: &mut UINT) -> HRESULT {
    if lOperand >= 0 {
        *puiResult = lOperand as UINT;
        S_OK
    } else {
        *puiResult = UINT_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn LongToUInt32(lOperand: LONG, pui32Result: &mut UINT32) -> HRESULT {
    LongToUInt(lOperand, pui32Result)
}
#[cfg(target_arch = "x86_64")] #[inline]
pub fn LongToUIntPtr(lOperand: LONG, puResult: &mut UINT_PTR) -> HRESULT {
    if lOperand >= 0 {
        *puResult = lOperand as UINT_PTR;
        S_OK
    } else {
        *puResult = UINT_PTR_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[cfg(target_arch = "x86")] #[inline]
pub fn LongToUIntPtr(lOperand: LONG, puResult: &mut UINT_PTR) -> HRESULT {
    LongToUInt(lOperand, puResult)
}
#[inline]
pub fn LongToULong(lOperand: LONG, pulResult: &mut ULONG) -> HRESULT {
    if lOperand >= 0 {
        *pulResult = lOperand as ULONG;
        S_OK
    } else {
        *pulResult = ULONG_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[cfg(target_arch = "x86_64")] #[inline]
pub fn LongToULongPtr(lOperand: LONG, pulResult: &mut ULONG_PTR) -> HRESULT {
    if lOperand >= 0 {
        *pulResult = lOperand as ULONG_PTR;
        S_OK
    } else {
        *pulResult = ULONG_PTR_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[cfg(target_arch = "x86")] #[inline]
pub fn LongToULongPtr(lOperand: LONG, pulResult: &mut ULONG_PTR) -> HRESULT {
    LongToULong(lOperand, pulResult)
}
#[inline]
pub fn LongToDWord(lOperand: LONG, pdwResult: &mut DWORD) -> HRESULT {
    LongToULong(lOperand, pdwResult)
}
#[inline]
pub fn LongToDWordPtr(lOperand: LONG, pdwResult: &mut DWORD_PTR) -> HRESULT {
    LongToULongPtr(lOperand, pdwResult)
}
#[inline]
pub fn LongToULongLong(lOperand: LONG, pullResult: &mut ULONGLONG) -> HRESULT {
    if lOperand >= 0 {
        *pullResult = lOperand as ULONGLONG;
        S_OK
    } else {
        *pullResult = ULONGLONG_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn LongToDWordLong(lOperand: LONG, pdwResult: &mut DWORDLONG) -> HRESULT {
    LongToULongLong(lOperand, pdwResult)
}
#[inline]
pub fn LongToULong64(lOperand: LONG, pullResult: &mut ULONG64) -> HRESULT {
    LongToULongLong(lOperand, pullResult)
}
#[inline]
pub fn LongToDWord64(lOperand: LONG, pdwResult: &mut DWORD64) -> HRESULT {
    LongToULongLong(lOperand, pdwResult)
}
#[inline]
pub fn LongToUInt64(lOperand: LONG, pui64Result: &mut UINT64) -> HRESULT {
    LongToULongLong(lOperand, pui64Result)
}
#[inline]
pub fn LongToPtrdiffT(lOperand: LONG, pResult: &mut ptrdiff_t) -> HRESULT {
    LongToIntPtr(lOperand, pResult)
}
#[inline]
pub fn LongToSizeT(lOperand: LONG, pResult: &mut size_t) -> HRESULT {
    LongToUIntPtr(lOperand, pResult)
}
#[inline]
pub fn LongToSIZET(lOperand: LONG, pResult: &mut SIZE_T) -> HRESULT {
    LongToULongPtr(lOperand, pResult)
}
#[inline]
pub fn LongPtrToInt8(lOperand: LONG_PTR, pi8Result: &mut INT8) -> HRESULT {
    if (lOperand >= INT8_MIN as LONG_PTR) && (lOperand <= INT8_MAX as LONG_PTR) {
        *pi8Result = lOperand as INT8;
        S_OK
    } else {
        *pi8Result = INT8_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn LongPtrToUChar(lOperand: LONG_PTR, pch: &mut UCHAR) -> HRESULT {
    if (lOperand >= 0) && (lOperand <= 255) {
        *pch = lOperand as UCHAR;
        S_OK
    } else {
        *pch = 0;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn LongPtrToChar(lOperand: LONG_PTR, pch: &mut CHAR) -> HRESULT {
    LongPtrToInt8(lOperand, pch)
}
#[inline]
pub fn LongPtrToUInt8(lOperand: LONG_PTR, pui8Result: &mut UINT8) -> HRESULT {
    if (lOperand >= 0) && (lOperand <= UINT8_MAX as LONG_PTR) {
        *pui8Result = lOperand as UINT8;
        S_OK
    } else {
        *pui8Result = UINT8_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn LongPtrToByte(lOperand: LONG_PTR, pui8Result: &mut BYTE) -> HRESULT {
    LongPtrToUInt8(lOperand, pui8Result)
}
#[inline]
pub fn LongPtrToShort(lOperand: LONG_PTR, psResult: &mut SHORT) -> HRESULT {
    if (lOperand >= SHORT_MIN as LONG_PTR) && (lOperand <= SHORT_MAX as LONG_PTR) {
        *psResult = lOperand as SHORT;
        S_OK
    } else {
        *psResult = SHORT_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn LongPtrToInt16(lOperand: LONG_PTR, pi16Result: &mut INT16) -> HRESULT {
    LongPtrToShort(lOperand, pi16Result)
}
#[inline]
pub fn LongPtrToUShort(lOperand: LONG_PTR, pusResult: &mut USHORT) -> HRESULT {
    if (lOperand >= 0) && (lOperand <= USHORT_MAX as LONG_PTR) {
        *pusResult = lOperand as USHORT;
        S_OK
    } else {
        *pusResult = USHORT_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn LongPtrToUInt16(lOperand: LONG_PTR, pui16Result: &mut UINT16) -> HRESULT {
    LongPtrToUShort(lOperand, pui16Result)
}
#[inline]
pub fn LongPtrToWord(lOperand: LONG_PTR, pwResult: &mut WORD) -> HRESULT {
    LongPtrToUShort(lOperand, pwResult)
}
#[cfg(target_arch = "x86_64")] #[inline]
pub fn LongPtrToInt(lOperand: LONG_PTR, piResult: &mut INT) -> HRESULT {
    LongLongToInt(lOperand, piResult)
}
#[cfg(target_arch = "x86")] #[inline]
pub fn LongPtrToInt(lOperand: LONG_PTR, piResult: &mut INT) -> HRESULT {
    *piResult = lOperand;
    S_OK
}
#[inline]
pub fn LongPtrToInt32(lOperand: LONG_PTR, pi32Result: &mut INT32) -> HRESULT {
    LongPtrToInt(lOperand, pi32Result)
}
#[inline]
pub fn LongPtrToIntPtr(lOperand: LONG_PTR, pusResult: &mut INT_PTR) -> HRESULT {
    *pusResult = lOperand;
    S_OK
}
#[cfg(target_arch = "x86_64")] #[inline]
pub fn LongPtrToUInt(lOperand: LONG_PTR, puResult: &mut UINT) -> HRESULT {
    LongLongToUInt(lOperand, puResult)
}
#[cfg(target_arch = "x86")] #[inline]
pub fn LongPtrToUInt(lOperand: LONG_PTR, puResult: &mut UINT) -> HRESULT {
    if lOperand >= 0 {
        *puResult = lOperand as UINT;
        S_OK
    } else {
        *puResult = UINT_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn LongPtrToUInt32(lOperand: LONG_PTR, pu32Result: &mut UINT32) -> HRESULT {
    LongPtrToUInt(lOperand, pu32Result)
}
#[inline]
pub fn LongPtrToUIntPtr(lOperand: LONG_PTR, puResult: &mut UINT_PTR) -> HRESULT {
    if lOperand >= 0 {
        *puResult = lOperand as UINT_PTR;
        S_OK
    } else {
        *puResult = UINT_PTR_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[cfg(target_arch = "x86_64")] #[inline]
pub fn LongPtrToLong(lOperand: LONG_PTR, plResult: &mut LONG) -> HRESULT {
    LongLongToLong(lOperand, plResult)
}
#[cfg(target_arch = "x86")] #[inline]
pub fn LongPtrToLong(lOperand: LONG_PTR, plResult: &mut LONG) -> HRESULT {
    *plResult = lOperand;
    S_OK
}
#[cfg(target_arch = "x86_64")] #[inline]
pub fn LongPtrToULong(lOperand: LONG_PTR, pulResult: &mut ULONG) -> HRESULT {
    LongLongToULong(lOperand, pulResult)
}
#[cfg(target_arch = "x86")] #[inline]
pub fn LongPtrToULong(lOperand: LONG_PTR, pulResult: &mut ULONG) -> HRESULT {
    if lOperand >= 0 {
        *pulResult = lOperand as ULONG;
        S_OK
    } else {
        *pulResult = ULONG_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn LongPtrToULongPtr(lOperand: LONG_PTR, pulResult: &mut ULONG_PTR) -> HRESULT {
    if lOperand >= 0 {
        *pulResult = lOperand as ULONG_PTR;
        S_OK
    } else {
        *pulResult = ULONG_PTR_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn LongPtrToDWord(lOperand: LONG_PTR, pdwResult: &mut DWORD) -> HRESULT {
    LongPtrToULong(lOperand, pdwResult)
}
#[inline]
pub fn LongPtrToDWordPtr(lOperand: LONG_PTR, pdwResult: &mut DWORD_PTR) -> HRESULT {
    LongPtrToULongPtr(lOperand, pdwResult)
}
#[inline]
pub fn LongPtrToULongLong(lOperand: LONG_PTR, pullResult: &mut ULONGLONG) -> HRESULT {
    if lOperand >= 0 {
        *pullResult = lOperand as ULONGLONG;
        S_OK
    } else {
        *pullResult = ULONGLONG_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn LongPtrToDWordLong(lOperand: LONG_PTR, pdwResult: &mut DWORDLONG) -> HRESULT {
    LongPtrToULongLong(lOperand, pdwResult)
}
#[inline]
pub fn LongPtrToULong64(lOperand: LONG_PTR, pullResult: &mut ULONG64) -> HRESULT {
    LongPtrToULongLong(lOperand, pullResult)
}
#[inline]
pub fn LongPtrToDWord64(lOperand: LONG_PTR, pdwResult: &mut DWORD64) -> HRESULT {
    LongPtrToULongLong(lOperand, pdwResult)
}
#[inline]
pub fn LongPtrToUInt64(lOperand: LONG_PTR, pui64Result: &mut UINT64) -> HRESULT {
    LongPtrToULongLong(lOperand, pui64Result)
}
#[inline]
pub fn LongPtrToSizeT(lOperand: LONG_PTR, pResult: &mut size_t) -> HRESULT {
    LongPtrToUIntPtr(lOperand, pResult)
}
#[inline]
pub fn LongPtrToSIZET(lOperand: LONG_PTR, pResult: &mut SIZE_T) -> HRESULT {
    LongPtrToULongPtr(lOperand, pResult)
}
#[inline]
pub fn ULongToInt8(ulOperand: ULONG, pi8Result: &mut INT8) -> HRESULT {
    if ulOperand <= INT8_MAX as ULONG {
        *pi8Result = ulOperand as INT8;
        S_OK
    } else {
        *pi8Result = INT8_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn ULongToUChar(ulOperand: ULONG, pch: &mut UCHAR) -> HRESULT {
    if ulOperand <= 255 {
        *pch = ulOperand as UCHAR;
        S_OK
    } else {
        *pch = 0;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn ULongToChar(ulOperand: ULONG, pch: &mut CHAR) -> HRESULT {
    ULongToInt8(ulOperand, pch)
}
#[inline]
pub fn ULongToUInt8(ulOperand: ULONG, pui8Result: &mut UINT8) -> HRESULT {
    if ulOperand <= UINT8_MAX as ULONG {
        *pui8Result = ulOperand as UINT8;
        S_OK
    } else {
        *pui8Result = UINT8_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn ULongToByte(ulOperand: ULONG, pui8Result: &mut BYTE) -> HRESULT {
    ULongToUInt8(ulOperand, pui8Result)
}
#[inline]
pub fn ULongToShort(ulOperand: ULONG, psResult: &mut SHORT) -> HRESULT {
    if ulOperand <= SHORT_MAX as ULONG {
        *psResult = ulOperand as SHORT;
        S_OK
    } else {
        *psResult = SHORT_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn ULongToInt16(ulOperand: ULONG, pi16Result: &mut INT16) -> HRESULT {
    ULongToShort(ulOperand, pi16Result)
}
#[inline]
pub fn ULongToUShort(ulOperand: ULONG, pusResult: &mut USHORT) -> HRESULT {
    if ulOperand <= USHORT_MAX as ULONG {
        *pusResult = ulOperand as USHORT;
        S_OK
    } else {
        *pusResult = USHORT_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn ULongToUInt16(ulOperand: ULONG, pui16Result: &mut UINT16) -> HRESULT {
    ULongToUShort(ulOperand, pui16Result)
}
#[inline]
pub fn ULongToWord(ulOperand: ULONG, pwResult: &mut WORD) -> HRESULT {
    ULongToUShort(ulOperand, pwResult)
}
#[inline]
pub fn ULongToInt(ulOperand: ULONG, piResult: &mut INT) -> HRESULT {
    if ulOperand <= INT_MAX as ULONG {
        *piResult = ulOperand as INT;
        S_OK
    } else {
        *piResult = INT_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn ULongToInt32(ulOperand: ULONG, pi32Result: &mut INT32) -> HRESULT {
    ULongToInt(ulOperand, pi32Result)
}
#[cfg(target_arch = "x86_64")] #[inline]
pub fn ULongToIntPtr(ulOperand: ULONG, piResult: &mut INT_PTR) -> HRESULT {
    *piResult = ulOperand as INT_PTR;
    S_OK
}
#[cfg(target_arch = "x86")] #[inline]
pub fn ULongToIntPtr(ulOperand: ULONG, piResult: &mut INT_PTR) -> HRESULT {
    ULongToInt(ulOperand, piResult)
}
#[inline]
pub fn ULongToUInt(ulOperand: ULONG, puResult: &mut UINT) -> HRESULT {
    *puResult = ulOperand;
    S_OK
}
#[inline]
pub fn ULongToUInt32(ulOperand: ULONG, pu32Result: &mut UINT32) -> HRESULT {
    ULongToUInt(ulOperand, pu32Result)
}
#[cfg(target_arch = "x86_64")] #[inline]
pub fn ULongToUIntPtr(ulOperand: ULONG, puiResult: &mut UINT_PTR) -> HRESULT {
    *puiResult = ulOperand as UINT_PTR;
    S_OK
}
#[cfg(target_arch = "x86")] #[inline]
pub fn ULongToUIntPtr(ulOperand: ULONG, puiResult: &mut UINT_PTR) -> HRESULT {
    ULongToUInt(ulOperand, puiResult)
}
#[inline]
pub fn ULongToLong(ulOperand: ULONG, plResult: &mut LONG) -> HRESULT {
    if ulOperand <= LONG_MAX as ULONG {
        *plResult = ulOperand as LONG;
        S_OK
    } else {
        *plResult = LONG_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[cfg(target_arch = "x86_64")] #[inline]
pub fn ULongToLongPtr(ulOperand: ULONG, plResult: &mut LONG_PTR) -> HRESULT {
    *plResult = ulOperand as LONG_PTR;
    S_OK
}
#[cfg(target_arch = "x86")] #[inline]
pub fn ULongToLongPtr(ulOperand: ULONG, plResult: &mut LONG_PTR) -> HRESULT {
    ULongToLong(ulOperand, plResult)
}
#[inline]
pub fn ULongToPtrdiffT(ulOperand: ULONG, pResult: &mut ptrdiff_t) -> HRESULT {
    ULongToIntPtr(ulOperand, pResult)
}
#[inline]
pub fn ULongToSSIZET(ulOperand: ULONG, pResult: &mut SSIZE_T) -> HRESULT {
    ULongToLongPtr(ulOperand, pResult)
}
#[inline]
pub fn ULongPtrToInt8(ulOperand: ULONG_PTR, pi8Result: &mut INT8) -> HRESULT {
    if ulOperand <= INT8_MAX as ULONG_PTR {
        *pi8Result = ulOperand as INT8;
        S_OK
    } else {
        *pi8Result = INT8_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn ULongPtrToUChar(ulOperand: ULONG_PTR, pch: &mut UCHAR) -> HRESULT {
    if ulOperand <= 255 {
        *pch = ulOperand as UCHAR;
        S_OK
    } else {
        *pch = 0;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn ULongPtrToChar(ulOperand: ULONG_PTR, pch: &mut CHAR) -> HRESULT {
    ULongPtrToInt8(ulOperand, pch)
}
#[inline]
pub fn ULongPtrToUInt8(ulOperand: ULONG_PTR, pui8Result: &mut UINT8) -> HRESULT {
    if ulOperand <= UINT8_MAX as ULONG_PTR {
        *pui8Result = ulOperand as UINT8;
        S_OK
    } else {
        *pui8Result = UINT8_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn ULongPtrToByte(ulOperand: ULONG_PTR, pui8Result: &mut BYTE) -> HRESULT {
    ULongPtrToUInt8(ulOperand, pui8Result)
}
#[inline]
pub fn ULongPtrToShort(ulOperand: ULONG_PTR, psResult: &mut SHORT) -> HRESULT {
    if ulOperand <= SHORT_MAX as ULONG_PTR {
        *psResult = ulOperand as SHORT;
        S_OK
    } else {
        *psResult = SHORT_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn ULongPtrToInt16(ulOperand: ULONG_PTR, pi16Result: &mut INT16) -> HRESULT {
    ULongPtrToShort(ulOperand, pi16Result)
}
#[inline]
pub fn ULongPtrToUShort(ulOperand: ULONG_PTR, pusResult: &mut USHORT) -> HRESULT {
    if ulOperand <= USHORT_MAX as ULONG_PTR {
        *pusResult = ulOperand as USHORT;
        S_OK
    } else {
        *pusResult = USHORT_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn ULongPtrToUInt16(ulOperand: ULONG_PTR, pui16Result: &mut UINT16) -> HRESULT {
    ULongPtrToUShort(ulOperand, pui16Result)
}
#[inline]
pub fn ULongPtrToWord(ulOperand: ULONG_PTR, pwResult: &mut WORD) -> HRESULT {
    ULongPtrToUShort(ulOperand, pwResult)
}
#[inline]
pub fn ULongPtrToInt(ulOperand: ULONG_PTR, piResult: &mut INT) -> HRESULT {
    if ulOperand <= INT_MAX as ULONG_PTR {
        *piResult = ulOperand as INT;
        S_OK
    } else {
        *piResult = INT_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn ULongPtrToInt32(ulOperand: ULONG_PTR, pi32Result: &mut INT32) -> HRESULT {
    ULongPtrToInt(ulOperand, pi32Result)
}
#[inline]
pub fn ULongPtrToIntPtr(ulOperand: ULONG_PTR, piResult: &mut INT_PTR) -> HRESULT {
    if ulOperand <= INT_PTR_MAX as ULONG_PTR {
        *piResult = ulOperand as INT_PTR;
        S_OK
    } else {
        *piResult = INT_PTR_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[cfg(target_arch = "x86_64")] #[inline]
pub fn ULongPtrToUInt(ulOperand: ULONG_PTR, puResult: &mut UINT) -> HRESULT {
    ULongLongToUInt(ulOperand, puResult)
}
#[cfg(target_arch = "x86")] #[inline]
pub fn ULongPtrToUInt(ulOperand: ULONG_PTR, puResult: &mut UINT) -> HRESULT {
    *puResult = ulOperand;
    S_OK
}
#[inline]
pub fn ULongPtrToUInt32(ulOperand: ULONG_PTR, pu32Result: &mut UINT32) -> HRESULT {
    ULongPtrToUInt(ulOperand, pu32Result)
}
#[inline]
pub fn ULongPtrToUIntPtr(ulOperand: ULONG_PTR, puResult: &mut UINT_PTR) -> HRESULT {
    *puResult = ulOperand;
    S_OK
}
#[inline]
pub fn ULongPtrToLong(ulOperand: ULONG_PTR, plResult: &mut LONG) -> HRESULT {
    if ulOperand <= LONG_MAX as ULONG_PTR {
        *plResult = ulOperand as LONG;
        S_OK
    } else {
        *plResult = LONG_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn ULongPtrToLongPtr(ulOperand: ULONG_PTR, plResult: &mut LONG_PTR) -> HRESULT {
    if ulOperand <= LONG_PTR_MAX as ULONG_PTR {
        *plResult = ulOperand as LONG_PTR;
        S_OK
    } else {
        *plResult = LONG_PTR_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[cfg(target_arch = "x86_64")] #[inline]
pub fn ULongPtrToULong(ulOperand: ULONG_PTR, pulResult: &mut ULONG) -> HRESULT {
    ULongLongToULong(ulOperand, pulResult)
}
#[cfg(target_arch = "x86")] #[inline]
pub fn ULongPtrToULong(ulOperand: ULONG_PTR, pulResult: &mut ULONG) -> HRESULT {
    *pulResult = ulOperand;
    S_OK
}
#[inline]
pub fn ULongPtrToDWord(ulOperand: ULONG_PTR, pdwResult: &mut DWORD) -> HRESULT {
    ULongPtrToULong(ulOperand, pdwResult)
}
#[cfg(target_arch = "x86_64")] #[inline]
pub fn ULongPtrToLongLong(ulOperand: ULONG_PTR, pllResult: &mut LONGLONG) -> HRESULT {
    ULongLongToLongLong(ulOperand, pllResult)
}
#[cfg(target_arch = "x86")] #[inline]
pub fn ULongPtrToLongLong(ulOperand: ULONG_PTR, pllResult: &mut LONGLONG) -> HRESULT {
    *pllResult = ulOperand as LONGLONG;
    S_OK
}
#[inline]
pub fn ULongPtrToLong64(ulOperand: ULONG_PTR, pllResult: &mut LONG64) -> HRESULT {
    ULongPtrToLongLong(ulOperand, pllResult)
}
#[inline]
pub fn ULongPtrToInt64(ulOperand: ULONG_PTR, pi64Result: &mut INT64) -> HRESULT {
    ULongPtrToLongLong(ulOperand, pi64Result)
}
#[inline]
pub fn ULongPtrToPtrdiffT(ulOperand: ULONG_PTR, pResult: &mut ptrdiff_t) -> HRESULT {
    ULongPtrToIntPtr(ulOperand, pResult)
}
#[inline]
pub fn ULongPtrToSSIZET(ulOperand: ULONG_PTR, pResult: &mut SSIZE_T) -> HRESULT {
    ULongPtrToLongPtr(ulOperand, pResult)
}
#[inline]
pub fn DWordToInt8(dwOperand: DWORD, pi8Result: &mut INT8) -> HRESULT {
    ULongToInt8(dwOperand, pi8Result)
}
#[inline]
pub fn DWordToChar(dwOperand: DWORD, pch: &mut CHAR) -> HRESULT {
    ULongToChar(dwOperand, pch)
}
#[inline]
pub fn DWordToUChar(dwOperand: DWORD, pch: &mut UCHAR) -> HRESULT {
    ULongToUChar(dwOperand, pch)
}
#[inline]
pub fn DWordToUInt8(dwOperand: DWORD, pui8Result: &mut UINT8) -> HRESULT {
    ULongToUInt8(dwOperand, pui8Result)
}
#[inline]
pub fn DWordToByte(dwOperand: DWORD, pui8Result: &mut BYTE) -> HRESULT {
    ULongToUInt8(dwOperand, pui8Result)
}
#[inline]
pub fn DWordToShort(dwOperand: DWORD, psResult: &mut SHORT) -> HRESULT {
    ULongToShort(dwOperand, psResult)
}
#[inline]
pub fn DWordToInt16(dwOperand: DWORD, pi16Result: &mut INT16) -> HRESULT {
    ULongToShort(dwOperand, pi16Result)
}
#[inline]
pub fn DWordToUShort(dwOperand: DWORD, pusResult: &mut USHORT) -> HRESULT {
    ULongToUShort(dwOperand, pusResult)
}
#[inline]
pub fn DWordToUInt16(dwOperand: DWORD, pui16Result: &mut UINT16) -> HRESULT {
    ULongToUShort(dwOperand, pui16Result)
}
#[inline]
pub fn DWordToWord(dwOperand: DWORD, pwResult: &mut WORD) -> HRESULT {
    ULongToUShort(dwOperand, pwResult)
}
#[inline]
pub fn DWordToInt(dwOperand: DWORD, piResult: &mut INT) -> HRESULT {
    ULongToInt(dwOperand, piResult)
}
#[inline]
pub fn DWordToInt32(dwOperand: DWORD, pi32Result: &mut INT32) -> HRESULT {
    ULongToInt(dwOperand, pi32Result)
}
#[inline]
pub fn DWordToIntPtr(dwOperand: DWORD, piResult: &mut INT_PTR) -> HRESULT {
    ULongToIntPtr(dwOperand, piResult)
}
#[inline]
pub fn DWordToUInt(dwOperand: DWORD, puResult: &mut UINT) -> HRESULT {
    ULongToUInt(dwOperand, puResult)
}
#[inline]
pub fn DWordToUInt32(dwOperand: DWORD, pu32Result: &mut UINT32) -> HRESULT {
    ULongToUInt(dwOperand, pu32Result)
}
#[inline]
pub fn DWordToUIntPtr(dwOperand: DWORD, puResult: &mut UINT_PTR) -> HRESULT {
    ULongToUIntPtr(dwOperand, puResult)
}
#[inline]
pub fn DWordToLong(dwOperand: DWORD, plResult: &mut LONG) -> HRESULT {
    ULongToLong(dwOperand, plResult)
}
#[inline]
pub fn DWordToLongPtr(dwOperand: DWORD, plResult: &mut LONG_PTR) -> HRESULT {
    ULongToLongPtr(dwOperand, plResult)
}
#[inline]
pub fn DWordToPtrdiffT(dwOperand: DWORD, pResult: &mut ptrdiff_t) -> HRESULT {
    ULongToIntPtr(dwOperand, pResult)
}
#[inline]
pub fn DWordToSSIZET(dwOperand: DWORD, pResult: &mut SSIZE_T) -> HRESULT {
    ULongToLongPtr(dwOperand, pResult)
}
#[inline]
pub fn DWordPtrToInt8(dwOperand: DWORD_PTR, pi8Result: &mut INT8) -> HRESULT {
    ULongPtrToInt8(dwOperand, pi8Result)
}
#[inline]
pub fn DWordPtrToUChar(dwOperand: DWORD_PTR, pch: &mut UCHAR) -> HRESULT {
    ULongPtrToUChar(dwOperand, pch)
}
#[inline]
pub fn DWordPtrToChar(dwOperand: DWORD_PTR, pch: &mut CHAR) -> HRESULT {
    ULongPtrToChar(dwOperand, pch)
}
#[inline]
pub fn DWordPtrToUInt8(dwOperand: DWORD_PTR, pui8Result: &mut UINT8) -> HRESULT {
    ULongPtrToUInt8(dwOperand, pui8Result)
}
#[inline]
pub fn DWordPtrToByte(dwOperand: DWORD_PTR, pui8Result: &mut BYTE) -> HRESULT {
    ULongPtrToUInt8(dwOperand, pui8Result)
}
#[inline]
pub fn DWordPtrToShort(dwOperand: DWORD_PTR, psResult: &mut SHORT) -> HRESULT {
    ULongPtrToShort(dwOperand, psResult)
}
#[inline]
pub fn DWordPtrToInt16(dwOperand: DWORD_PTR, pi16Result: &mut INT16) -> HRESULT {
    ULongPtrToShort(dwOperand, pi16Result)
}
#[inline]
pub fn DWordPtrToUShort(dwOperand: DWORD_PTR, pusResult: &mut USHORT) -> HRESULT {
    ULongPtrToUShort(dwOperand, pusResult)
}
#[inline]
pub fn DWordPtrToUInt16(dwOperand: DWORD_PTR, pui16Result: &mut UINT16) -> HRESULT {
    ULongPtrToUShort(dwOperand, pui16Result)
}
#[inline]
pub fn DWordPtrToWord(dwOperand: DWORD_PTR, pwResult: &mut WORD) -> HRESULT {
    ULongPtrToUShort(dwOperand, pwResult)
}
#[inline]
pub fn DWordPtrToInt(dwOperand: DWORD_PTR, piResult: &mut INT) -> HRESULT {
    ULongPtrToInt(dwOperand, piResult)
}
#[inline]
pub fn DWordPtrToInt32(dwOperand: DWORD_PTR, piResult: &mut INT32) -> HRESULT {
    ULongPtrToInt(dwOperand, piResult)
}
#[inline]
pub fn DWordPtrToIntPtr(dwOperand: DWORD_PTR, piResult: &mut INT_PTR) -> HRESULT {
    ULongPtrToIntPtr(dwOperand, piResult)
}
#[inline]
pub fn DWordPtrToUInt(dwOperand: DWORD_PTR, puResult: &mut UINT) -> HRESULT {
    ULongPtrToUInt(dwOperand, puResult)
}
#[inline]
pub fn DWordPtrToUInt32(dwOperand: DWORD_PTR, puResult: &mut UINT32) -> HRESULT {
    ULongPtrToUInt(dwOperand, puResult)
}
#[inline]
pub fn DWordPtrToUIntPtr(dwOperand: DWORD_PTR, puResult: &mut UINT_PTR) -> HRESULT {
    ULongPtrToUIntPtr(dwOperand, puResult)
}
#[inline]
pub fn DWordPtrToLong(dwOperand: DWORD_PTR, plResult: &mut LONG) -> HRESULT {
    ULongPtrToLong(dwOperand, plResult)
}
#[inline]
pub fn DWordPtrToLongPtr(dwOperand: DWORD_PTR, plResult: &mut LONG_PTR) -> HRESULT {
    ULongPtrToLongPtr(dwOperand, plResult)
}
#[inline]
pub fn DWordPtrToULong(dwOperand: DWORD_PTR, pulResult: &mut ULONG) -> HRESULT {
    ULongPtrToULong(dwOperand, pulResult)
}
#[inline]
pub fn DWordPtrToDWord(dwOperand: DWORD_PTR, pdwResult: &mut DWORD) -> HRESULT {
    ULongPtrToULong(dwOperand, pdwResult)
}
#[inline]
pub fn DWordPtrToLongLong(dwOperand: DWORD_PTR, pllResult: &mut LONGLONG) -> HRESULT {
    ULongPtrToLongLong(dwOperand, pllResult)
}
#[inline]
pub fn DWordPtrToLong64(dwOperand: DWORD_PTR, pllResult: &mut LONG64) -> HRESULT {
    ULongPtrToLongLong(dwOperand, pllResult)
}
#[inline]
pub fn DWordPtrToInt64(dwOperand: DWORD_PTR, pi64Result: &mut INT64) -> HRESULT {
    ULongPtrToLongLong(dwOperand, pi64Result)
}
#[inline]
pub fn DWordPtrToPtrdiffT(dwOperand: DWORD_PTR, pResult: &mut ptrdiff_t) -> HRESULT {
    ULongPtrToIntPtr(dwOperand, pResult)
}
#[inline]
pub fn DWordPtrToSSIZET(dwOperand: DWORD_PTR, pResult: &mut SSIZE_T) -> HRESULT {
    ULongPtrToLongPtr(dwOperand, pResult)
}
#[inline]
pub fn LongLongToInt8(llOperand: LONGLONG, pi8Result: &mut INT8) -> HRESULT {
    if (llOperand >= INT8_MIN as LONGLONG) && (llOperand <= INT8_MAX as LONGLONG) {
        *pi8Result = llOperand as INT8;
        S_OK
    } else {
        *pi8Result = INT8_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn LongLongToUChar(llOperand: LONGLONG, pch: &mut UCHAR) -> HRESULT {
    if (llOperand >= 0) && (llOperand <= 255) {
        *pch = llOperand as UCHAR;
        S_OK
    } else {
        *pch = 0;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn LongLongToChar(llOperand: LONGLONG, pch: &mut CHAR) -> HRESULT {
    LongLongToInt8(llOperand, pch)
}
#[inline]
pub fn LongLongToUInt8(llOperand: LONGLONG, pu8Result: &mut UINT8) -> HRESULT {
    if (llOperand >= 0) && (llOperand <= UINT8_MAX as LONGLONG) {
        *pu8Result = llOperand as UINT8;
        S_OK
    } else {
        *pu8Result = UINT8_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn LongLongToByte(llOperand: LONGLONG, pu8Result: &mut BYTE) -> HRESULT {
    LongLongToUInt8(llOperand, pu8Result)
}
#[inline]
pub fn LongLongToShort(llOperand: LONGLONG, psResult: &mut SHORT) -> HRESULT {
    if (llOperand >= SHORT_MIN as LONGLONG) && (llOperand <= SHORT_MAX as LONGLONG) {
        *psResult = llOperand as SHORT;
        S_OK
    } else {
        *psResult = SHORT_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn LongLongToInt16(llOperand: LONGLONG, pi16Result: &mut INT16) -> HRESULT {
    LongLongToShort(llOperand, pi16Result)
}
#[inline]
pub fn LongLongToUShort(llOperand: LONGLONG, pusResult: &mut USHORT) -> HRESULT {
    if (llOperand >= 0) && (llOperand <= USHORT_MAX as LONGLONG) {
        *pusResult = llOperand as USHORT;
        S_OK
    } else {
        *pusResult = USHORT_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn LongLongToUInt16(llOperand: LONGLONG, pui16Result: &mut UINT16) -> HRESULT {
    LongLongToUShort(llOperand, pui16Result)
}
#[inline]
pub fn LongLongToWord(llOperand: LONGLONG, pwResult: &mut WORD) -> HRESULT {
    LongLongToUShort(llOperand, pwResult)
}
#[inline]
pub fn LongLongToInt(llOperand: LONGLONG, piResult: &mut INT) -> HRESULT {
    if (llOperand >= INT_MIN as LONGLONG) && (llOperand <= INT_MAX as LONGLONG) {
        *piResult = llOperand as INT;
        S_OK
    } else {
        *piResult = INT_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn LongLongToInt32(llOperand: LONGLONG, pi32Result: &mut INT32) -> HRESULT {
    LongLongToInt(llOperand, pi32Result)
}
#[cfg(target_arch = "x86_64")] #[inline]
pub fn LongLongToIntPtr(llOperand: LONGLONG, piResult: &mut INT_PTR) -> HRESULT {
    *piResult = llOperand;
    S_OK
}
#[cfg(target_arch = "x86")] #[inline]
pub fn LongLongToIntPtr(llOperand: LONGLONG, piResult: &mut INT_PTR) -> HRESULT {
    LongLongToInt(llOperand, piResult)
}
#[inline]
pub fn LongLongToUInt(llOperand: LONGLONG, puResult: &mut UINT) -> HRESULT {
    if (llOperand >= 0) && (llOperand <= UINT_MAX as LONGLONG) {
        *puResult = llOperand as UINT;
        S_OK
    } else {
        *puResult = UINT_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn LongLongToUInt32(llOperand: LONGLONG, pu32Result: &mut UINT32) -> HRESULT {
    LongLongToUInt(llOperand, pu32Result)
}
#[cfg(target_arch = "x86_64")] #[inline]
pub fn LongLongToUIntPtr(llOperand: LONGLONG, puResult: &mut UINT_PTR) -> HRESULT {
    LongLongToULongLong(llOperand, puResult)
}
#[cfg(target_arch = "x86")] #[inline]
pub fn LongLongToUIntPtr(llOperand: LONGLONG, puResult: &mut UINT_PTR) -> HRESULT {
    LongLongToUInt(llOperand, puResult)
}
#[inline]
pub fn LongLongToLong(llOperand: LONGLONG, plResult: &mut LONG) -> HRESULT {
    if (llOperand >= LONG_MIN as LONGLONG) && (llOperand <= LONG_MAX as LONGLONG) {
        *plResult = llOperand as LONG;
        S_OK
    } else {
        *plResult = LONG_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[cfg(target_arch = "x86_64")] #[inline]
pub fn LongLongToLongPtr(llOperand: LONGLONG, plResult: &mut LONG_PTR) -> HRESULT {
    *plResult = llOperand;
    S_OK
}
#[cfg(target_arch = "x86")] #[inline]
pub fn LongLongToLongPtr(llOperand: LONGLONG, plResult: &mut LONG_PTR) -> HRESULT {
    LongLongToLong(llOperand, plResult)
}
#[inline]
pub fn LongLongToULong(llOperand: LONGLONG, pulResult: &mut ULONG) -> HRESULT {
    if (llOperand >= 0) && (llOperand <= ULONG_MAX as LONGLONG) {
        *pulResult = llOperand as ULONG;
        S_OK
    } else {
        *pulResult = ULONG_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[cfg(target_arch = "x86_64")] #[inline]
pub fn LongLongToULongPtr(llOperand: LONGLONG, pulResult: &mut ULONG_PTR) -> HRESULT {
    LongLongToULongLong(llOperand, pulResult)
}
#[cfg(target_arch = "x86")] #[inline]
pub fn LongLongToULongPtr(llOperand: LONGLONG, pulResult: &mut ULONG_PTR) -> HRESULT {
    LongLongToULong(llOperand, pulResult)
}
#[inline]
pub fn LongLongToDWord(llOperand: LONGLONG, pdwResult: &mut DWORD) -> HRESULT {
    LongLongToULong(llOperand, pdwResult)
}
#[inline]
pub fn LongLongToDWordPtr(llOperand: LONGLONG, pdwResult: &mut DWORD_PTR) -> HRESULT {
    LongLongToULongPtr(llOperand, pdwResult)
}
#[inline]
pub fn LongLongToULongLong(llOperand: LONGLONG, pullResult: &mut ULONGLONG) -> HRESULT {
    if llOperand >= 0 {
        *pullResult = llOperand as ULONGLONG;
        S_OK
    } else {
        *pullResult = ULONGLONG_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn LongLongToDWordLong(llOperand: LONGLONG, pdwResult: &mut DWORDLONG) -> HRESULT {
    LongLongToULongLong(llOperand, pdwResult)
}
#[inline]
pub fn LongLongToULong64(llOperand: LONGLONG, pullResult: &mut ULONG64) -> HRESULT {
    LongLongToULongLong(llOperand, pullResult)
}
#[inline]
pub fn LongLongToDWord64(llOperand: LONGLONG, pdwResult: &mut DWORD64) -> HRESULT {
    LongLongToULongLong(llOperand, pdwResult)
}
#[inline]
pub fn LongLongToUInt64(llOperand: LONGLONG, pu64Result: &mut UINT64) -> HRESULT {
    LongLongToULongLong(llOperand, pu64Result)
}
#[inline]
pub fn LongLongToPtrdiffT(llOperand: LONGLONG, pResult: &mut ptrdiff_t) -> HRESULT {
    LongLongToIntPtr(llOperand, pResult)
}
#[inline]
pub fn LongLongToSizeT(llOperand: LONGLONG, pResult: &mut size_t) -> HRESULT {
    LongLongToUIntPtr(llOperand, pResult)
}
#[inline]
pub fn LongLongToSSIZET(llOperand: LONGLONG, pResult: &mut SSIZE_T) -> HRESULT {
    LongLongToLongPtr(llOperand, pResult)
}
#[inline]
pub fn LongLongToSIZET(llOperand: LONGLONG, pResult: &mut SIZE_T) -> HRESULT {
    LongLongToULongPtr(llOperand, pResult)
}
#[inline]
pub fn Long64ToChar(llOperand: LONG64, pch: &mut CHAR) -> HRESULT {
    LongLongToChar(llOperand, pch)
}
#[inline]
pub fn Long64ToInt8(llOperand: LONG64, pi8Result: &mut INT8) -> HRESULT {
    LongLongToInt8(llOperand, pi8Result)
}
#[inline]
pub fn Long64ToUChar(llOperand: LONG64, pch: &mut UCHAR) -> HRESULT {
    LongLongToUChar(llOperand, pch)
}
#[inline]
pub fn Long64ToUInt8(llOperand: LONG64, pui8Result: &mut UINT8) -> HRESULT {
    LongLongToUInt8(llOperand, pui8Result)
}
#[inline]
pub fn Long64ToByte(llOperand: LONG64, pui8Result: &mut BYTE) -> HRESULT {
    LongLongToUInt8(llOperand, pui8Result)
}
#[inline]
pub fn Long64ToShort(llOperand: LONG64, psResult: &mut SHORT) -> HRESULT {
    LongLongToShort(llOperand, psResult)
}
#[inline]
pub fn Long64ToInt16(llOperand: LONG64, pi16Result: &mut INT16) -> HRESULT {
    LongLongToShort(llOperand, pi16Result)
}
#[inline]
pub fn Long64ToUShort(llOperand: LONG64, pusResult: &mut USHORT) -> HRESULT {
    LongLongToUShort(llOperand, pusResult)
}
#[inline]
pub fn Long64ToUInt16(llOperand: LONG64, pu16Result: &mut UINT16) -> HRESULT {
    LongLongToUShort(llOperand, pu16Result)
}
#[inline]
pub fn Long64ToWord(llOperand: LONG64, pwResult: &mut WORD) -> HRESULT {
    LongLongToUShort(llOperand, pwResult)
}
#[inline]
pub fn Long64ToInt(llOperand: LONG64, piResult: &mut INT) -> HRESULT {
    LongLongToInt(llOperand, piResult)
}
#[inline]
pub fn Long64ToInt32(llOperand: LONG64, piResult: &mut INT32) -> HRESULT {
    LongLongToInt(llOperand, piResult)
}
#[inline]
pub fn Long64ToIntPtr(llOperand: LONG64, piResult: &mut INT_PTR) -> HRESULT {
    LongLongToIntPtr(llOperand, piResult)
}
#[inline]
pub fn Long64ToUInt(llOperand: LONG64, puResult: &mut UINT) -> HRESULT {
    LongLongToUInt(llOperand, puResult)
}
#[inline]
pub fn Long64ToUInt32(llOperand: LONG64, puResult: &mut UINT32) -> HRESULT {
    LongLongToUInt(llOperand, puResult)
}
#[inline]
pub fn Long64ToUIntPtr(llOperand: LONG64, puResult: &mut UINT_PTR) -> HRESULT {
    LongLongToUIntPtr(llOperand, puResult)
}
#[inline]
pub fn Long64ToLong(llOperand: LONG64, plResult: &mut LONG) -> HRESULT {
    LongLongToLong(llOperand, plResult)
}
#[inline]
pub fn Long64ToLongPtr(llOperand: LONG64, plResult: &mut LONG_PTR) -> HRESULT {
    LongLongToLongPtr(llOperand, plResult)
}
#[inline]
pub fn Long64ToULong(llOperand: LONG64, pulResult: &mut ULONG) -> HRESULT {
    LongLongToULong(llOperand, pulResult)
}
#[inline]
pub fn Long64ToULongPtr(llOperand: LONG64, pulResult: &mut ULONG_PTR) -> HRESULT {
    LongLongToULongPtr(llOperand, pulResult)
}
#[inline]
pub fn Long64ToDWord(llOperand: LONG64, pdwResult: &mut DWORD) -> HRESULT {
    LongLongToULong(llOperand, pdwResult)
}
#[inline]
pub fn Long64ToDWordPtr(llOperand: LONG64, pdwResult: &mut DWORD_PTR) -> HRESULT {
    LongLongToULongPtr(llOperand, pdwResult)
}
#[inline]
pub fn Long64ToULongLong(llOperand: LONG64, pullResult: &mut ULONGLONG) -> HRESULT {
    LongLongToULongLong(llOperand, pullResult)
}
#[inline]
pub fn Long64ToPtrdiffT(llOperand: LONG64, pResult: &mut ptrdiff_t) -> HRESULT {
    LongLongToIntPtr(llOperand, pResult)
}
#[inline]
pub fn Long64ToSizeT(llOperand: LONG64, pResult: &mut size_t) -> HRESULT {
    LongLongToUIntPtr(llOperand, pResult)
}
#[inline]
pub fn Long64ToSSIZET(llOperand: LONG64, pResult: &mut SSIZE_T) -> HRESULT {
    LongLongToLongPtr(llOperand, pResult)
}
#[inline]
pub fn Long64ToSIZET(llOperand: LONG64, pResult: &mut SIZE_T) -> HRESULT {
    LongLongToULongPtr(llOperand, pResult)
}
#[inline]
pub fn Int64ToChar(i64Operand: INT64, pch: &mut CHAR) -> HRESULT {
    LongLongToChar(i64Operand, pch)
}
#[inline]
pub fn Int64ToInt8(i64Operand: INT64, pi8Result: &mut INT8) -> HRESULT {
    LongLongToInt8(i64Operand, pi8Result)
}
#[inline]
pub fn Int64ToUChar(i64Operand: INT64, pch: &mut UCHAR) -> HRESULT {
    LongLongToUChar(i64Operand, pch)
}
#[inline]
pub fn Int64ToUInt8(i64Operand: INT64, pu8Result: &mut UINT8) -> HRESULT {
    LongLongToUInt8(i64Operand, pu8Result)
}
#[inline]
pub fn Int64ToByte(i64Operand: INT64, pu8Result: &mut BYTE) -> HRESULT {
    LongLongToUInt8(i64Operand, pu8Result)
}
#[inline]
pub fn Int64ToShort(i64Operand: INT64, psResult: &mut SHORT) -> HRESULT {
    LongLongToShort(i64Operand, psResult)
}
#[inline]
pub fn Int64ToInt16(i64Operand: INT64, pi16Result: &mut INT16) -> HRESULT {
    LongLongToShort(i64Operand, pi16Result)
}
#[inline]
pub fn Int64ToUShort(i64Operand: INT64, pusResult: &mut USHORT) -> HRESULT {
    LongLongToUShort(i64Operand, pusResult)
}
#[inline]
pub fn Int64ToUInt16(i64Operand: INT64, pui16Result: &mut UINT16) -> HRESULT {
    LongLongToUShort(i64Operand, pui16Result)
}
#[inline]
pub fn Int64ToWord(i64Operand: INT64, pwResult: &mut WORD) -> HRESULT {
    LongLongToUShort(i64Operand, pwResult)
}
#[inline]
pub fn Int64ToInt(i64Operand: INT64, piResult: &mut INT) -> HRESULT {
    LongLongToInt(i64Operand, piResult)
}
#[inline]
pub fn Int64ToInt32(i64Operand: INT64, piResult: &mut INT32) -> HRESULT {
    LongLongToInt(i64Operand, piResult)
}
#[inline]
pub fn Int64ToIntPtr(i64Operand: INT64, piResult: &mut INT_PTR) -> HRESULT {
    LongLongToIntPtr(i64Operand, piResult)
}
#[inline]
pub fn Int64ToUInt(i64Operand: INT64, puResult: &mut UINT) -> HRESULT {
    LongLongToUInt(i64Operand, puResult)
}
#[inline]
pub fn Int64ToUInt32(i64Operand: INT64, puResult: &mut UINT32) -> HRESULT {
    LongLongToUInt(i64Operand, puResult)
}
#[inline]
pub fn Int64ToUIntPtr(i64Operand: INT64, puResult: &mut UINT_PTR) -> HRESULT {
    LongLongToUIntPtr(i64Operand, puResult)
}
#[inline]
pub fn Int64ToLong(i64Operand: INT64, plResult: &mut LONG) -> HRESULT {
    LongLongToLong(i64Operand, plResult)
}
#[inline]
pub fn Int64ToLongPtr(i64Operand: INT64, plResult: &mut LONG_PTR) -> HRESULT {
    LongLongToLongPtr(i64Operand, plResult)
}
#[inline]
pub fn Int64ToULong(i64Operand: INT64, pulResult: &mut ULONG) -> HRESULT {
    LongLongToULong(i64Operand, pulResult)
}
#[inline]
pub fn Int64ToULongPtr(i64Operand: INT64, pulResult: &mut ULONG_PTR) -> HRESULT {
    LongLongToULongPtr(i64Operand, pulResult)
}
#[inline]
pub fn Int64ToDWord(i64Operand: INT64, pdwResult: &mut DWORD) -> HRESULT {
    LongLongToULong(i64Operand, pdwResult)
}
#[inline]
pub fn Int64ToDWordPtr(i64Operand: INT64, pdwResult: &mut DWORD_PTR) -> HRESULT {
    LongLongToULongPtr(i64Operand, pdwResult)
}
#[inline]
pub fn Int64ToULongLong(i64Operand: INT64, pullResult: &mut ULONGLONG) -> HRESULT {
    LongLongToULongLong(i64Operand, pullResult)
}
#[inline]
pub fn Int64ToDWordLong(i64Operand: INT64, pdwResult: &mut DWORDLONG) -> HRESULT {
    LongLongToULongLong(i64Operand, pdwResult)
}
#[inline]
pub fn Int64ToULong64(i64Operand: INT64, pullResult: &mut ULONG64) -> HRESULT {
    LongLongToULongLong(i64Operand, pullResult)
}
#[inline]
pub fn Int64ToDWord64(i64Operand: INT64, pdwResult: &mut DWORD64) -> HRESULT {
    LongLongToULongLong(i64Operand, pdwResult)
}
#[inline]
pub fn Int64ToUInt64(i64Operand: INT64, pi64Result: &mut UINT64) -> HRESULT {
    LongLongToULongLong(i64Operand, pi64Result)
}
#[inline]
pub fn Int64ToPtrdiffT(i64Operand: INT64, pResult: &mut ptrdiff_t) -> HRESULT {
    LongLongToIntPtr(i64Operand, pResult)
}
#[inline]
pub fn Int64ToSizeT(i64Operand: INT64, pResult: &mut size_t) -> HRESULT {
    LongLongToUIntPtr(i64Operand, pResult)
}
#[inline]
pub fn Int64ToSSIZET(i64Operand: INT64, pResult: &mut SSIZE_T) -> HRESULT {
    LongLongToLongPtr(i64Operand, pResult)
}
#[inline]
pub fn Int64ToSIZET(i64Operand: INT64, pResult: &mut SIZE_T) -> HRESULT {
    LongLongToULongPtr(i64Operand, pResult)
}
#[inline]
pub fn ULongLongToInt8(ullOperand: ULONGLONG, pi8Result: &mut INT8) -> HRESULT {
    if ullOperand <= INT8_MAX as ULONGLONG {
        *pi8Result = ullOperand as INT8;
        S_OK
    } else {
        *pi8Result = INT8_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn ULongLongToUChar(ullOperand: ULONGLONG, pch: &mut UCHAR) -> HRESULT {
    if ullOperand <= 255 {
        *pch = ullOperand as UCHAR;
        S_OK
    } else {
        *pch = 0;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn ULongLongToChar(ullOperand: ULONGLONG, pch: &mut CHAR) -> HRESULT {
    ULongLongToInt8(ullOperand, pch)
}
#[inline]
pub fn ULongLongToUInt8(ullOperand: ULONGLONG, pu8Result: &mut UINT8) -> HRESULT {
    if ullOperand <= UINT8_MAX as ULONGLONG {
        *pu8Result = ullOperand as UINT8;
        S_OK
    } else {
        *pu8Result = UINT8_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn ULongLongToByte(ullOperand: ULONGLONG, pu8Result: &mut BYTE) -> HRESULT {
    ULongLongToUInt8(ullOperand, pu8Result)
}
#[inline]
pub fn ULongLongToShort(ullOperand: ULONGLONG, psResult: &mut SHORT) -> HRESULT {
    if ullOperand <= SHORT_MAX as ULONGLONG {
        *psResult = ullOperand as SHORT;
        S_OK
    } else {
        *psResult = SHORT_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn ULongLongToInt16(ullOperand: ULONGLONG, pi16Result: &mut INT16) -> HRESULT {
    ULongLongToShort(ullOperand, pi16Result)
}
#[inline]
pub fn ULongLongToUShort(ullOperand: ULONGLONG, pusResult: &mut USHORT) -> HRESULT {
    if ullOperand <= USHORT_MAX as ULONGLONG {
        *pusResult = ullOperand as USHORT;
        S_OK
    } else {
        *pusResult = USHORT_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn ULongLongToUInt16(ullOperand: ULONGLONG, pu16Result: &mut UINT16) -> HRESULT {
    ULongLongToUShort(ullOperand, pu16Result)
}
#[inline]
pub fn ULongLongToWord(ullOperand: ULONGLONG, pwResult: &mut WORD) -> HRESULT {
    ULongLongToUShort(ullOperand, pwResult)
}
#[inline]
pub fn ULongLongToInt(ullOperand: ULONGLONG, piResult: &mut INT) -> HRESULT {
    if ullOperand <= INT_MAX as ULONGLONG {
        *piResult = ullOperand as INT;
        S_OK
    } else {
        *piResult = INT_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn ULongLongToInt32(ullOperand: ULONGLONG, piResult: &mut INT32) -> HRESULT {
    ULongLongToInt(ullOperand, piResult)
}
#[cfg(target_arch = "x86_64")] #[inline]
pub fn ULongLongToIntPtr(ullOperand: ULONGLONG, piResult: &mut INT_PTR) -> HRESULT {
    ULongLongToLongLong(ullOperand, piResult)
}
#[cfg(target_arch = "x86")] #[inline]
pub fn ULongLongToIntPtr(ullOperand: ULONGLONG, piResult: &mut INT_PTR) -> HRESULT {
    ULongLongToInt(ullOperand, piResult)
}
#[inline]
pub fn ULongLongToUInt(ullOperand: ULONGLONG, puResult: &mut UINT) -> HRESULT {
    if ullOperand <= UINT_MAX as ULONGLONG {
        *puResult = ullOperand as UINT;
        S_OK
    } else {
        *puResult = UINT_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn ULongLongToUInt32(ullOperand: ULONGLONG, puResult: &mut UINT32) -> HRESULT {
    ULongLongToUInt(ullOperand, puResult)
}
#[cfg(target_arch = "x86_64")] #[inline]
pub fn ULongLongToUIntPtr(ullOperand: ULONGLONG, puResult: &mut UINT_PTR) -> HRESULT {
    *puResult = ullOperand;
    S_OK
}
#[cfg(target_arch = "x86")] #[inline]
pub fn ULongLongToUIntPtr(ullOperand: ULONGLONG, puResult: &mut UINT_PTR) -> HRESULT {
    ULongLongToUInt(ullOperand, puResult)
}
#[inline]
pub fn ULongLongToLong(ullOperand: ULONGLONG, plResult: &mut LONG) -> HRESULT {
    if ullOperand <= LONG_MAX as ULONGLONG {
        *plResult = ullOperand as LONG;
        S_OK
    } else {
        *plResult = LONG_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn ULongLongToLongPtr(ullOperand: ULONGLONG, plResult: &mut LONG_PTR) -> HRESULT {
    if ullOperand <= LONG_PTR_MAX as ULONGLONG {
        *plResult = ullOperand as LONG_PTR;
        S_OK
    } else {
        *plResult = LONG_PTR_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn ULongLongToULong(ullOperand: ULONGLONG, pulResult: &mut ULONG) -> HRESULT {
    if ullOperand <= ULONG_MAX as ULONGLONG {
        *pulResult = ullOperand as ULONG;
        S_OK
    } else {
        *pulResult = ULONG_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[cfg(target_arch = "x86_64")] #[inline]
pub fn ULongLongToULongPtr(ullOperand: ULONGLONG, pulResult: &mut ULONG_PTR) -> HRESULT {
    *pulResult = ullOperand;
    S_OK
}
#[cfg(target_arch = "x86")] #[inline]
pub fn ULongLongToULongPtr(ullOperand: ULONGLONG, pulResult: &mut ULONG_PTR) -> HRESULT {
    ULongLongToULong(ullOperand, pulResult)
}
#[inline]
pub fn ULongLongToDWord(ullOperand: ULONGLONG, pdwResult: &mut DWORD) -> HRESULT {
    ULongLongToULong(ullOperand, pdwResult)
}
#[inline]
pub fn ULongLongToDWordPtr(ullOperand: ULONGLONG, pdwResult: &mut DWORD_PTR) -> HRESULT {
    ULongLongToULongPtr(ullOperand, pdwResult)
}
#[inline]
pub fn ULongLongToLongLong(ullOperand: ULONGLONG, pllResult: &mut LONGLONG) -> HRESULT {
    if ullOperand <= LONGLONG_MAX as ULONGLONG {
        *pllResult = ullOperand as LONGLONG;
        S_OK
    } else {
        *pllResult = LONGLONG_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn ULongLongToInt64(ullOperand: ULONGLONG, pi64Result: &mut INT64) -> HRESULT {
    ULongLongToLongLong(ullOperand, pi64Result)
}
#[inline]
pub fn ULongLongToLong64(ullOperand: ULONGLONG, pllResult: &mut LONG64) -> HRESULT {
    ULongLongToLongLong(ullOperand, pllResult)
}
#[inline]
pub fn ULongLongToPtrdiffT(ullOperand: ULONGLONG, pResult: &mut ptrdiff_t) -> HRESULT {
    ULongLongToIntPtr(ullOperand, pResult)
}
#[inline]
pub fn ULongLongToSizeT(ullOperand: ULONGLONG, pResult: &mut size_t) -> HRESULT {
    ULongLongToUIntPtr(ullOperand, pResult)
}
#[inline]
pub fn ULongLongToSSIZET(ullOperand: ULONGLONG, pResult: &mut SSIZE_T) -> HRESULT {
    ULongLongToLongPtr(ullOperand, pResult)
}
#[inline]
pub fn ULongLongToSIZET(ullOperand: ULONGLONG, pResult: &mut SIZE_T) -> HRESULT {
    ULongLongToULongPtr(ullOperand, pResult)
}
#[inline]
pub fn DWordLongToChar(dwOperand: DWORDLONG, pch: &mut CHAR) -> HRESULT {
    ULongLongToChar(dwOperand, pch)
}
#[inline]
pub fn DWordLongToInt8(dwOperand: DWORDLONG, pi8Result: &mut INT8) -> HRESULT {
    ULongLongToInt8(dwOperand, pi8Result)
}
#[inline]
pub fn DWordLongToUChar(dwOperand: DWORDLONG, pch: &mut UCHAR) -> HRESULT {
    ULongLongToUChar(dwOperand, pch)
}
#[inline]
pub fn DWordLongToUInt8(dwOperand: DWORDLONG, pu8Result: &mut UINT8) -> HRESULT {
    ULongLongToUInt8(dwOperand, pu8Result)
}
#[inline]
pub fn DWordLongToByte(dwOperand: DWORDLONG, pu8Result: &mut BYTE) -> HRESULT {
    ULongLongToUInt8(dwOperand, pu8Result)
}
#[inline]
pub fn DWordLongToShort(dwOperand: DWORDLONG, psResult: &mut SHORT) -> HRESULT {
    ULongLongToShort(dwOperand, psResult)
}
#[inline]
pub fn DWordLongToInt16(dwOperand: DWORDLONG, pi16Result: &mut INT16) -> HRESULT {
    ULongLongToShort(dwOperand, pi16Result)
}
#[inline]
pub fn DWordLongToUShort(dwOperand: DWORDLONG, pusResult: &mut USHORT) -> HRESULT {
    ULongLongToUShort(dwOperand, pusResult)
}
#[inline]
pub fn DWordLongToUInt16(dwOperand: DWORDLONG, pu16Result: &mut UINT16) -> HRESULT {
    ULongLongToUShort(dwOperand, pu16Result)
}
#[inline]
pub fn DWordLongToWord(dwOperand: DWORDLONG, pwResult: &mut WORD) -> HRESULT {
    ULongLongToUShort(dwOperand, pwResult)
}
#[inline]
pub fn DWordLongToInt(dwOperand: DWORDLONG, piResult: &mut INT) -> HRESULT {
    ULongLongToInt(dwOperand, piResult)
}
#[inline]
pub fn DWordLongToInt32(dwOperand: DWORDLONG, piResult: &mut INT32) -> HRESULT {
    ULongLongToInt(dwOperand, piResult)
}
#[inline]
pub fn DWordLongToIntPtr(dwOperand: DWORDLONG, piResult: &mut INT_PTR) -> HRESULT {
    ULongLongToIntPtr(dwOperand, piResult)
}
#[inline]
pub fn DWordLongToUInt(dwOperand: DWORDLONG, puResult: &mut UINT) -> HRESULT {
    ULongLongToUInt(dwOperand, puResult)
}
#[inline]
pub fn DWordLongToUInt32(dwOperand: DWORDLONG, puResult: &mut UINT32) -> HRESULT {
    ULongLongToUInt(dwOperand, puResult)
}
#[inline]
pub fn DWordLongToUIntPtr(dwOperand: DWORDLONG, puResult: &mut UINT_PTR) -> HRESULT {
    ULongLongToUIntPtr(dwOperand, puResult)
}
#[inline]
pub fn DWordLongToLong(dwOperand: DWORDLONG, plResult: &mut LONG) -> HRESULT {
    ULongLongToLong(dwOperand, plResult)
}
#[inline]
pub fn DWordLongToLongPtr(dwOperand: DWORDLONG, plResult: &mut LONG_PTR) -> HRESULT {
    ULongLongToLongPtr(dwOperand, plResult)
}
#[inline]
pub fn DWordLongToULong(dwOperand: DWORDLONG, pulResult: &mut ULONG) -> HRESULT {
    ULongLongToULong(dwOperand, pulResult)
}
#[inline]
pub fn DWordLongToULongPtr(dwOperand: DWORDLONG, pulResult: &mut ULONG_PTR) -> HRESULT {
    ULongLongToULongPtr(dwOperand, pulResult)
}
#[inline]
pub fn DWordLongToDWord(dwOperand: DWORDLONG, pdwResult: &mut DWORD) -> HRESULT {
    ULongLongToULong(dwOperand, pdwResult)
}
#[inline]
pub fn DWordLongToDWordPtr(dwOperand: DWORDLONG, pdwResult: &mut DWORD_PTR) -> HRESULT {
    ULongLongToULongPtr(dwOperand, pdwResult)
}
#[inline]
pub fn DWordLongToLongLong(dwOperand: DWORDLONG, pllResult: &mut LONGLONG) -> HRESULT {
    ULongLongToLongLong(dwOperand, pllResult)
}
#[inline]
pub fn DWordLongToLong64(dwOperand: DWORDLONG, pllResult: &mut LONG64) -> HRESULT {
    ULongLongToLongLong(dwOperand, pllResult)
}
#[inline]
pub fn DWordLongToInt64(dwOperand: DWORDLONG, pi64Result: &mut INT64) -> HRESULT {
    ULongLongToLongLong(dwOperand, pi64Result)
}
#[inline]
pub fn DWordLongToPtrdiffT(dwOperand: DWORDLONG, pResult: &mut ptrdiff_t) -> HRESULT {
    ULongLongToIntPtr(dwOperand, pResult)
}
#[inline]
pub fn DWordLongToSizeT(dwOperand: DWORDLONG, pResult: &mut size_t) -> HRESULT {
    ULongLongToUIntPtr(dwOperand, pResult)
}
#[inline]
pub fn DWordLongToSSIZET(dwOperand: DWORDLONG, pResult: &mut SSIZE_T) -> HRESULT {
    ULongLongToLongPtr(dwOperand, pResult)
}
#[inline]
pub fn DWordLongToSIZET(dwOperand: DWORDLONG, pResult: &mut SIZE_T) -> HRESULT {
    ULongLongToULongPtr(dwOperand, pResult)
}
#[inline]
pub fn ULong64ToChar(ullOperand: ULONG64, pch: &mut CHAR) -> HRESULT {
    ULongLongToChar(ullOperand, pch)
}
#[inline]
pub fn ULong64ToInt8(ullOperand: ULONG64, pi8Result: &mut INT8) -> HRESULT {
    ULongLongToInt8(ullOperand, pi8Result)
}
#[inline]
pub fn ULong64ToUChar(ullOperand: ULONG64, pch: &mut UCHAR) -> HRESULT {
    ULongLongToUChar(ullOperand, pch)
}
#[inline]
pub fn ULong64ToUInt8(ullOperand: ULONG64, pu8Result: &mut UINT8) -> HRESULT {
    ULongLongToUInt8(ullOperand, pu8Result)
}
#[inline]
pub fn ULong64ToByte(ullOperand: ULONG64, pu8Result: &mut BYTE) -> HRESULT {
    ULongLongToUInt8(ullOperand, pu8Result)
}
#[inline]
pub fn ULong64ToShort(ullOperand: ULONG64, psResult: &mut SHORT) -> HRESULT {
    ULongLongToShort(ullOperand, psResult)
}
#[inline]
pub fn ULong64ToInt16(ullOperand: ULONG64, pi16Result: &mut INT16) -> HRESULT {
    ULongLongToShort(ullOperand, pi16Result)
}
#[inline]
pub fn ULong64ToUShort(ullOperand: ULONG64, pusResult: &mut USHORT) -> HRESULT {
    ULongLongToUShort(ullOperand, pusResult)
}
#[inline]
pub fn ULong64ToUInt16(ullOperand: ULONG64, pu16Result: &mut UINT16) -> HRESULT {
    ULongLongToUShort(ullOperand, pu16Result)
}
#[inline]
pub fn ULong64ToWord(ullOperand: ULONG64, pwResult: &mut WORD) -> HRESULT {
    ULongLongToUShort(ullOperand, pwResult)
}
#[inline]
pub fn ULong64ToInt(ullOperand: ULONG64, piResult: &mut INT) -> HRESULT {
    ULongLongToInt(ullOperand, piResult)
}
#[inline]
pub fn ULong64ToInt32(ullOperand: ULONG64, pi32Result: &mut INT32) -> HRESULT {
    ULongLongToInt(ullOperand, pi32Result)
}
#[inline]
pub fn ULong64ToIntPtr(ullOperand: ULONG64, piResult: &mut INT_PTR) -> HRESULT {
    ULongLongToIntPtr(ullOperand, piResult)
}
#[inline]
pub fn ULong64ToUInt(ullOperand: ULONG64, puResult: &mut UINT) -> HRESULT {
    ULongLongToUInt(ullOperand, puResult)
}
#[inline]
pub fn ULong64ToUInt32(ullOperand: ULONG64, pu32Result: &mut UINT32) -> HRESULT {
    ULongLongToUInt(ullOperand, pu32Result)
}
#[inline]
pub fn ULong64ToUIntPtr(ullOperand: ULONG64, puResult: &mut UINT_PTR) -> HRESULT {
    ULongLongToUIntPtr(ullOperand, puResult)
}
#[inline]
pub fn ULong64ToLong(ullOperand: ULONG64, plResult: &mut LONG) -> HRESULT {
    ULongLongToLong(ullOperand, plResult)
}
#[inline]
pub fn ULong64ToLongPtr(ullOperand: ULONG64, plResult: &mut LONG_PTR) -> HRESULT {
    ULongLongToLongPtr(ullOperand, plResult)
}
#[inline]
pub fn ULong64ToULong(ullOperand: ULONG64, pulResult: &mut ULONG) -> HRESULT {
    ULongLongToULong(ullOperand, pulResult)
}
#[inline]
pub fn ULong64ToULongPtr(ullOperand: ULONG64, pulResult: &mut ULONG_PTR) -> HRESULT {
    ULongLongToULongPtr(ullOperand, pulResult)
}
#[inline]
pub fn ULong64ToDWord(ullOperand: ULONG64, pdwResult: &mut DWORD) -> HRESULT {
    ULongLongToULong(ullOperand, pdwResult)
}
#[inline]
pub fn ULong64ToDWordPtr(ullOperand: ULONG64, pdwResult: &mut DWORD_PTR) -> HRESULT {
    ULongLongToULongPtr(ullOperand, pdwResult)
}
#[inline]
pub fn ULong64ToLongLong(ullOperand: ULONG64, pllResult: &mut LONGLONG) -> HRESULT {
    ULongLongToLongLong(ullOperand, pllResult)
}
#[inline]
pub fn ULong64ToLong64(ullOperand: ULONG64, pllResult: &mut LONG64) -> HRESULT {
    ULongLongToLongLong(ullOperand, pllResult)
}
#[inline]
pub fn ULong64ToInt64(ullOperand: ULONG64, pi64Result: &mut INT64) -> HRESULT {
    ULongLongToLongLong(ullOperand, pi64Result)
}
#[inline]
pub fn ULong64ToPtrdiffT(ullOperand: ULONG64, pResult: &mut ptrdiff_t) -> HRESULT {
    ULongLongToIntPtr(ullOperand, pResult)
}
#[inline]
pub fn ULong64ToSizeT(ullOperand: ULONG64, pResult: &mut size_t) -> HRESULT {
    ULongLongToUIntPtr(ullOperand, pResult)
}
#[inline]
pub fn ULong64ToSSIZET(ullOperand: ULONG64, pResult: &mut SSIZE_T) -> HRESULT {
    ULongLongToLongPtr(ullOperand, pResult)
}
#[inline]
pub fn ULong64ToSIZET(ullOperand: ULONG64, pResult: &mut SIZE_T) -> HRESULT {
    ULongLongToULongPtr(ullOperand, pResult)
}
#[inline]
pub fn DWord64ToChar(dwOperand: DWORD64, pch: &mut CHAR) -> HRESULT {
    ULongLongToChar(dwOperand, pch)
}
#[inline]
pub fn DWord64ToInt8(dwOperand: DWORD64, pi8Result: &mut INT8) -> HRESULT {
    ULongLongToInt8(dwOperand, pi8Result)
}
#[inline]
pub fn DWord64ToUChar(dwOperand: DWORD64, pch: &mut UCHAR) -> HRESULT {
    ULongLongToUChar(dwOperand, pch)
}
#[inline]
pub fn DWord64ToUInt8(dwOperand: DWORD64, pu8Result: &mut UINT8) -> HRESULT {
    ULongLongToUInt8(dwOperand, pu8Result)
}
#[inline]
pub fn DWord64ToByte(dwOperand: DWORD64, pu8Result: &mut BYTE) -> HRESULT {
    ULongLongToUInt8(dwOperand, pu8Result)
}
#[inline]
pub fn DWord64ToShort(dwOperand: DWORD64, psResult: &mut SHORT) -> HRESULT {
    ULongLongToShort(dwOperand, psResult)
}
#[inline]
pub fn DWord64ToInt16(dwOperand: DWORD64, pi16Result: &mut INT16) -> HRESULT {
    ULongLongToShort(dwOperand, pi16Result)
}
#[inline]
pub fn DWord64ToUShort(dwOperand: DWORD64, pusResult: &mut USHORT) -> HRESULT {
    ULongLongToUShort(dwOperand, pusResult)
}
#[inline]
pub fn DWord64ToUInt16(dwOperand: DWORD64, pu16Result: &mut UINT16) -> HRESULT {
    ULongLongToUShort(dwOperand, pu16Result)
}
#[inline]
pub fn DWord64ToWord(dwOperand: DWORD64, pwResult: &mut WORD) -> HRESULT {
    ULongLongToUShort(dwOperand, pwResult)
}
#[inline]
pub fn DWord64ToInt(dwOperand: DWORD64, piResult: &mut INT) -> HRESULT {
    ULongLongToInt(dwOperand, piResult)
}
#[inline]
pub fn DWord64ToInt32(dwOperand: DWORD64, pi32Result: &mut INT32) -> HRESULT {
    ULongLongToInt(dwOperand, pi32Result)
}
#[inline]
pub fn DWord64ToIntPtr(dwOperand: DWORD64, piResult: &mut INT_PTR) -> HRESULT {
    ULongLongToIntPtr(dwOperand, piResult)
}
#[inline]
pub fn DWord64ToUInt(dwOperand: DWORD64, puResult: &mut UINT) -> HRESULT {
    ULongLongToUInt(dwOperand, puResult)
}
#[inline]
pub fn DWord64ToUInt32(dwOperand: DWORD64, pu32Result: &mut UINT32) -> HRESULT {
    ULongLongToUInt(dwOperand, pu32Result)
}
#[inline]
pub fn DWord64ToUIntPtr(dwOperand: DWORD64, puResult: &mut UINT_PTR) -> HRESULT {
    ULongLongToUIntPtr(dwOperand, puResult)
}
#[inline]
pub fn DWord64ToLong(dwOperand: DWORD64, plResult: &mut LONG) -> HRESULT {
    ULongLongToLong(dwOperand, plResult)
}
#[inline]
pub fn DWord64ToLongPtr(dwOperand: DWORD64, plResult: &mut LONG_PTR) -> HRESULT {
    ULongLongToLongPtr(dwOperand, plResult)
}
#[inline]
pub fn DWord64ToULong(dwOperand: DWORD64, pulResult: &mut ULONG) -> HRESULT {
    ULongLongToULong(dwOperand, pulResult)
}
#[inline]
pub fn DWord64ToULongPtr(dwOperand: DWORD64, pulResult: &mut ULONG_PTR) -> HRESULT {
    ULongLongToULongPtr(dwOperand, pulResult)
}
#[inline]
pub fn DWord64ToDWord(dwOperand: DWORD64, pdwResult: &mut DWORD) -> HRESULT {
    ULongLongToULong(dwOperand, pdwResult)
}
#[inline]
pub fn DWord64ToDWordPtr(dwOperand: DWORD64, pdwResult: &mut DWORD_PTR) -> HRESULT {
    ULongLongToULongPtr(dwOperand, pdwResult)
}
#[inline]
pub fn DWord64ToLongLong(dwOperand: DWORD64, pllResult: &mut LONGLONG) -> HRESULT {
    ULongLongToLongLong(dwOperand, pllResult)
}
#[inline]
pub fn DWord64ToLong64(dwOperand: DWORD64, pllResult: &mut LONG64) -> HRESULT {
    ULongLongToLongLong(dwOperand, pllResult)
}
#[inline]
pub fn DWord64ToInt64(dwOperand: DWORD64, pi64Result: &mut INT64) -> HRESULT {
    ULongLongToLongLong(dwOperand, pi64Result)
}
#[inline]
pub fn DWord64ToPtrdiffT(dwOperand: DWORD64, pResult: &mut ptrdiff_t) -> HRESULT {
    ULongLongToIntPtr(dwOperand, pResult)
}
#[inline]
pub fn DWord64ToSizeT(dwOperand: DWORD64, pResult: &mut size_t) -> HRESULT {
    ULongLongToUIntPtr(dwOperand, pResult)
}
#[inline]
pub fn DWord64ToSSIZET(dwOperand: DWORD64, pResult: &mut SSIZE_T) -> HRESULT {
    ULongLongToLongPtr(dwOperand, pResult)
}
#[inline]
pub fn DWord64ToSIZET(dwOperand: DWORD64, pResult: &mut SIZE_T) -> HRESULT {
    ULongLongToULongPtr(dwOperand, pResult)
}
#[inline]
pub fn UInt64ToChar(u64Operand: UINT64, pch: &mut CHAR) -> HRESULT {
    ULongLongToChar(u64Operand, pch)
}
#[inline]
pub fn UInt64ToInt8(u64Operand: UINT64, pi8Result: &mut INT8) -> HRESULT {
    ULongLongToInt8(u64Operand, pi8Result)
}
#[inline]
pub fn UInt64ToUChar(u64Operand: UINT64, pch: &mut UCHAR) -> HRESULT {
    ULongLongToUChar(u64Operand, pch)
}
#[inline]
pub fn UInt64ToUInt8(u64Operand: UINT64, pu8Result: &mut UINT8) -> HRESULT {
    ULongLongToUInt8(u64Operand, pu8Result)
}
#[inline]
pub fn UInt64ToByte(u64Operand: UINT64, pu8Result: &mut BYTE) -> HRESULT {
    ULongLongToUInt8(u64Operand, pu8Result)
}
#[inline]
pub fn UInt64ToShort(u64Operand: UINT64, psResult: &mut SHORT) -> HRESULT {
    ULongLongToShort(u64Operand, psResult)
}
#[inline]
pub fn UInt64ToInt16(u64Operand: UINT64, pi16Result: &mut INT16) -> HRESULT {
    ULongLongToShort(u64Operand, pi16Result)
}
#[inline]
pub fn UInt64ToUShort(u64Operand: UINT64, pusResult: &mut USHORT) -> HRESULT {
    ULongLongToUShort(u64Operand, pusResult)
}
#[inline]
pub fn UInt64ToUInt16(u64Operand: UINT64, pu16Result: &mut UINT16) -> HRESULT {
    ULongLongToUShort(u64Operand, pu16Result)
}
#[inline]
pub fn UInt64ToWord(u64Operand: UINT64, pwResult: &mut WORD) -> HRESULT {
    ULongLongToUShort(u64Operand, pwResult)
}
#[inline]
pub fn UInt64ToInt(u64Operand: UINT64, piResult: &mut INT) -> HRESULT {
    ULongLongToInt(u64Operand, piResult)
}
#[inline]
pub fn UInt64ToInt32(u64Operand: UINT64, pi32Result: &mut INT32) -> HRESULT {
    ULongLongToInt(u64Operand, pi32Result)
}
#[inline]
pub fn UInt64ToIntPtr(u64Operand: UINT64, piResult: &mut INT_PTR) -> HRESULT {
    ULongLongToIntPtr(u64Operand, piResult)
}
#[inline]
pub fn UInt64ToUInt(u64Operand: UINT64, puResult: &mut UINT) -> HRESULT {
    ULongLongToUInt(u64Operand, puResult)
}
#[inline]
pub fn UInt64ToUInt32(u64Operand: UINT64, pu32Result: &mut UINT32) -> HRESULT {
    ULongLongToUInt(u64Operand, pu32Result)
}
#[inline]
pub fn UInt64ToUIntPtr(u64Operand: UINT64, puResult: &mut UINT_PTR) -> HRESULT {
    ULongLongToUIntPtr(u64Operand, puResult)
}
#[inline]
pub fn UInt64ToLong(u64Operand: UINT64, plResult: &mut LONG) -> HRESULT {
    ULongLongToLong(u64Operand, plResult)
}
#[inline]
pub fn UInt64ToLongPtr(u64Operand: UINT64, plResult: &mut LONG_PTR) -> HRESULT {
    ULongLongToLongPtr(u64Operand, plResult)
}
#[inline]
pub fn UInt64ToULong(u64Operand: UINT64, pulResult: &mut ULONG) -> HRESULT {
    ULongLongToULong(u64Operand, pulResult)
}
#[inline]
pub fn UInt64ToULongPtr(u64Operand: UINT64, pulResult: &mut ULONG_PTR) -> HRESULT {
    ULongLongToULongPtr(u64Operand, pulResult)
}
#[inline]
pub fn UInt64ToDWord(u64Operand: UINT64, pdwResult: &mut DWORD) -> HRESULT {
    ULongLongToULong(u64Operand, pdwResult)
}
#[inline]
pub fn UInt64ToDWordPtr(u64Operand: UINT64, pdwResult: &mut DWORD_PTR) -> HRESULT {
    ULongLongToULongPtr(u64Operand, pdwResult)
}
#[inline]
pub fn UInt64ToLongLong(u64Operand: UINT64, pllResult: &mut LONGLONG) -> HRESULT {
    ULongLongToLongLong(u64Operand, pllResult)
}
#[inline]
pub fn UInt64ToLong64(u64Operand: UINT64, pllResult: &mut LONG64) -> HRESULT {
    ULongLongToLongLong(u64Operand, pllResult)
}
#[inline]
pub fn UInt64ToInt64(u64Operand: UINT64, pi64Result: &mut INT64) -> HRESULT {
    ULongLongToLongLong(u64Operand, pi64Result)
}
#[inline]
pub fn UInt64ToPtrdiffT(u64Operand: UINT64, pResult: &mut ptrdiff_t) -> HRESULT {
    ULongLongToIntPtr(u64Operand, pResult)
}
#[inline]
pub fn UInt64ToSizeT(u64Operand: UINT64, pResult: &mut size_t) -> HRESULT {
    ULongLongToUIntPtr(u64Operand, pResult)
}
#[inline]
pub fn UInt64ToSSIZET(u64Operand: UINT64, pResult: &mut SSIZE_T) -> HRESULT {
    ULongLongToLongPtr(u64Operand, pResult)
}
#[inline]
pub fn UInt64ToSIZET(u64Operand: UINT64, pResult: &mut SIZE_T) -> HRESULT {
    ULongLongToULongPtr(u64Operand, pResult)
}
#[inline]
pub fn PtrdiffTToChar(operand: ptrdiff_t, pch: &mut CHAR) -> HRESULT {
    IntPtrToChar(operand, pch)
}
#[inline]
pub fn PtrdiffTToInt8(operand: ptrdiff_t, pi8Result: &mut INT8) -> HRESULT {
    IntPtrToChar(operand, pi8Result)
}
#[inline]
pub fn PtrdiffTToUChar(operand: ptrdiff_t, pch: &mut UCHAR) -> HRESULT {
    IntPtrToUChar(operand, pch)
}
#[inline]
pub fn PtrdiffTToUInt8(operand: ptrdiff_t, pu8Result: &mut UINT8) -> HRESULT {
    IntPtrToUInt8(operand, pu8Result)
}
#[inline]
pub fn PtrdiffTToByte(operand: ptrdiff_t, pu8Result: &mut BYTE) -> HRESULT {
    IntPtrToUInt8(operand, pu8Result)
}
#[inline]
pub fn PtrdiffTToShort(operand: ptrdiff_t, psResult: &mut SHORT) -> HRESULT {
    IntPtrToShort(operand, psResult)
}
#[inline]
pub fn PtrdiffTToInt16(operand: ptrdiff_t, pi16Result: &mut INT16) -> HRESULT {
    IntPtrToShort(operand, pi16Result)
}
#[inline]
pub fn PtrdiffTToUShort(operand: ptrdiff_t, pusResult: &mut USHORT) -> HRESULT {
    IntPtrToUShort(operand, pusResult)
}
#[inline]
pub fn PtrdiffTToUInt16(operand: ptrdiff_t, pu16Result: &mut UINT16) -> HRESULT {
    IntPtrToUShort(operand, pu16Result)
}
#[inline]
pub fn PtrdiffTToWord(operand: ptrdiff_t, pwResult: &mut WORD) -> HRESULT {
    IntPtrToUShort(operand, pwResult)
}
#[inline]
pub fn PtrdiffTToInt(operand: ptrdiff_t, piResult: &mut INT) -> HRESULT {
    IntPtrToInt(operand, piResult)
}
#[inline]
pub fn PtrdiffTToInt32(operand: ptrdiff_t, pi32Result: &mut INT32) -> HRESULT {
    IntPtrToInt(operand, pi32Result)
}
#[inline]
pub fn PtrdiffTToUInt(operand: ptrdiff_t, puResult: &mut UINT) -> HRESULT {
    IntPtrToUInt(operand, puResult)
}
#[inline]
pub fn PtrdiffTToUInt32(operand: ptrdiff_t, pu32Result: &mut UINT32) -> HRESULT {
    IntPtrToUInt(operand, pu32Result)
}
#[inline]
pub fn PtrdiffTToUIntPtr(operand: ptrdiff_t, puResult: &mut UINT_PTR) -> HRESULT {
    IntPtrToUIntPtr(operand, puResult)
}
#[inline]
pub fn PtrdiffTToLong(operand: ptrdiff_t, plResult: &mut LONG) -> HRESULT {
    IntPtrToLong(operand, plResult)
}
#[inline]
pub fn PtrdiffTToLongPtr(operand: ptrdiff_t, plResult: &mut LONG_PTR) -> HRESULT {
    IntPtrToLongPtr(operand, plResult)
}
#[inline]
pub fn PtrdiffTToULong(operand: ptrdiff_t, pulResult: &mut ULONG) -> HRESULT {
    IntPtrToULong(operand, pulResult)
}
#[inline]
pub fn PtrdiffTToULongPtr(operand: ptrdiff_t, pulResult: &mut ULONG_PTR) -> HRESULT {
    IntPtrToULongPtr(operand, pulResult)
}
#[inline]
pub fn PtrdiffTToDWord(operand: ptrdiff_t, pdwResult: &mut DWORD) -> HRESULT {
    IntPtrToULong(operand, pdwResult)
}
#[inline]
pub fn PtrdiffTToDWordPtr(operand: ptrdiff_t, pdwResult: &mut DWORD_PTR) -> HRESULT {
    IntPtrToULongPtr(operand, pdwResult)
}
#[inline]
pub fn PtrdiffTToULongLong(operand: ptrdiff_t, pullResult: &mut ULONGLONG) -> HRESULT {
    IntPtrToULongLong(operand, pullResult)
}
#[inline]
pub fn PtrdiffTToDWordLong(operand: ptrdiff_t, pdwResult: &mut DWORDLONG) -> HRESULT {
    IntPtrToULongLong(operand, pdwResult)
}
#[inline]
pub fn PtrdiffTToULong64(operand: ptrdiff_t, pullResult: &mut ULONG64) -> HRESULT {
    IntPtrToULongLong(operand, pullResult)
}
#[inline]
pub fn PtrdiffTToDWord64(operand: ptrdiff_t, pdwResult: &mut DWORD64) -> HRESULT {
    IntPtrToULongLong(operand, pdwResult)
}
#[inline]
pub fn PtrdiffTToUInt64(operand: ptrdiff_t, pu64Result: &mut UINT64) -> HRESULT {
    IntPtrToULongLong(operand, pu64Result)
}
#[inline]
pub fn PtrdiffTToSizeT(operand: ptrdiff_t, pResult: &mut size_t) -> HRESULT {
    IntPtrToUIntPtr(operand, pResult)
}
#[inline]
pub fn PtrdiffTToSIZET(operand: ptrdiff_t, pResult: &mut SIZE_T) -> HRESULT {
    IntPtrToULongPtr(operand, pResult)
}
#[inline]
pub fn SizeTToInt8(operand: size_t, pi8Result: &mut INT8) -> HRESULT {
    UIntPtrToInt8(operand, pi8Result)
}
#[inline]
pub fn SizeTToUChar(operand: size_t, pch: &mut UCHAR) -> HRESULT {
    UIntPtrToUChar(operand, pch)
}
#[inline]
pub fn SizeTToChar(operand: size_t, pch: &mut CHAR) -> HRESULT {
    UIntPtrToChar(operand, pch)
}
#[inline]
pub fn SizeTToUInt8(operand: size_t, pu8Result: &mut UINT8) -> HRESULT {
    UIntPtrToUInt8(operand, pu8Result)
}
#[inline]
pub fn SizeTToByte(operand: size_t, pu8Result: &mut BYTE) -> HRESULT {
    UIntPtrToUInt8(operand, pu8Result)
}
#[inline]
pub fn SizeTToShort(operand: size_t, psResult: &mut SHORT) -> HRESULT {
    UIntPtrToShort(operand, psResult)
}
#[inline]
pub fn SizeTToInt16(operand: size_t, pi16Result: &mut INT16) -> HRESULT {
    UIntPtrToShort(operand, pi16Result)
}
#[inline]
pub fn SizeTToUShort(operand: size_t, pusResult: &mut USHORT) -> HRESULT {
    UIntPtrToUShort(operand, pusResult)
}
#[inline]
pub fn SizeTToUInt16(operand: size_t, pu16Result: &mut UINT16) -> HRESULT {
    UIntPtrToUShort(operand, pu16Result)
}
#[inline]
pub fn SizeTToWord(operand: size_t, pwResult: &mut WORD) -> HRESULT {
    UIntPtrToUShort(operand, pwResult)
}
#[inline]
pub fn SizeTToInt(operand: size_t, piResult: &mut INT) -> HRESULT {
    UIntPtrToInt(operand, piResult)
}
#[inline]
pub fn SizeTToInt32(operand: size_t, pi32Result: &mut INT32) -> HRESULT {
    UIntPtrToInt(operand, pi32Result)
}
#[inline]
pub fn SizeTToIntPtr(operand: size_t, piResult: &mut INT_PTR) -> HRESULT {
    UIntPtrToIntPtr(operand, piResult)
}
#[inline]
pub fn SizeTToUInt(operand: size_t, puResult: &mut UINT) -> HRESULT {
    UIntPtrToUInt(operand, puResult)
}
#[inline]
pub fn SizeTToUInt32(operand: size_t, pu32Result: &mut UINT32) -> HRESULT {
    UIntPtrToUInt(operand, pu32Result)
}
#[inline]
pub fn SizeTToLong(operand: size_t, plResult: &mut LONG) -> HRESULT {
    UIntPtrToLong(operand, plResult)
}
#[inline]
pub fn SizeTToLongPtr(operand: size_t, plResult: &mut LONG_PTR) -> HRESULT {
    UIntPtrToLongPtr(operand, plResult)
}
#[inline]
pub fn SizeTToULong(operand: size_t, pulResult: &mut ULONG) -> HRESULT {
    UIntPtrToULong(operand, pulResult)
}
#[inline]
pub fn SizeTToDWord(operand: size_t, pdwResult: &mut DWORD) -> HRESULT {
    UIntPtrToULong(operand, pdwResult)
}
#[inline]
pub fn SizeTToLongLong(operand: size_t, pllResult: &mut LONGLONG) -> HRESULT {
    UIntPtrToLongLong(operand, pllResult)
}
#[inline]
pub fn SizeTToLong64(operand: size_t, pllResult: &mut LONG64) -> HRESULT {
    UIntPtrToLongLong(operand, pllResult)
}
#[inline]
pub fn SizeTToInt64(operand: size_t, pi64Result: &mut INT64) -> HRESULT {
    UIntPtrToLongLong(operand, pi64Result)
}
#[inline]
pub fn SizeTToPtrdiffT(operand: size_t, pResult: &mut ptrdiff_t) -> HRESULT {
    UIntPtrToIntPtr(operand, pResult)
}
#[inline]
pub fn SizeTToSSIZET(operand: size_t, pResult: &mut SSIZE_T) -> HRESULT {
    UIntPtrToLongPtr(operand, pResult)
}
#[inline]
pub fn SSIZETToInt8(operand: SSIZE_T, pi8Result: &mut INT8) -> HRESULT {
    LongPtrToInt8(operand, pi8Result)
}
#[inline]
pub fn SSIZETToUChar(operand: SSIZE_T, pch: &mut UCHAR) -> HRESULT {
    LongPtrToUChar(operand, pch)
}
#[inline]
pub fn SSIZETToChar(operand: SSIZE_T, pch: &mut CHAR) -> HRESULT {
    LongPtrToChar(operand, pch)
}
#[inline]
pub fn SSIZETToUInt8(operand: SSIZE_T, pu8Result: &mut UINT8) -> HRESULT {
    LongPtrToUInt8(operand, pu8Result)
}
#[inline]
pub fn SSIZETToByte(operand: SSIZE_T, pu8Result: &mut BYTE) -> HRESULT {
    LongPtrToUInt8(operand, pu8Result)
}
#[inline]
pub fn SSIZETToShort(operand: SSIZE_T, psResult: &mut SHORT) -> HRESULT {
    LongPtrToShort(operand, psResult)
}
#[inline]
pub fn SSIZETToInt16(operand: SSIZE_T, pi16Result: &mut INT16) -> HRESULT {
    LongPtrToShort(operand, pi16Result)
}
#[inline]
pub fn SSIZETToUShort(operand: SSIZE_T, pusResult: &mut USHORT) -> HRESULT {
    LongPtrToUShort(operand, pusResult)
}
#[inline]
pub fn SSIZETToUInt16(operand: SSIZE_T, pu16Result: &mut UINT16) -> HRESULT {
    LongPtrToUShort(operand, pu16Result)
}
#[inline]
pub fn SSIZETToWord(operand: SSIZE_T, pwResult: &mut WORD) -> HRESULT {
    LongPtrToUShort(operand, pwResult)
}
#[inline]
pub fn SSIZETToInt(operand: SSIZE_T, piResult: &mut INT) -> HRESULT {
    LongPtrToInt(operand, piResult)
}
#[inline]
pub fn SSIZETToInt32(operand: SSIZE_T, pi32Result: &mut INT32) -> HRESULT {
    LongPtrToInt(operand, pi32Result)
}
#[inline]
pub fn SSIZETToIntPtr(operand: SSIZE_T, piResult: &mut INT_PTR) -> HRESULT {
    LongPtrToIntPtr(operand, piResult)
}
#[inline]
pub fn SSIZETToUInt(operand: SSIZE_T, puResult: &mut UINT) -> HRESULT {
    LongPtrToUInt(operand, puResult)
}
#[inline]
pub fn SSIZETToUInt32(operand: SSIZE_T, pu32Result: &mut UINT32) -> HRESULT {
    LongPtrToUInt(operand, pu32Result)
}
#[inline]
pub fn SSIZETToUIntPtr(operand: SSIZE_T, puResult: &mut UINT_PTR) -> HRESULT {
    LongPtrToUIntPtr(operand, puResult)
}
#[inline]
pub fn SSIZETToLong(operand: SSIZE_T, plResult: &mut LONG) -> HRESULT {
    LongPtrToLong(operand, plResult)
}
#[inline]
pub fn SSIZETToULong(operand: SSIZE_T, pulResult: &mut ULONG) -> HRESULT {
    LongPtrToULong(operand, pulResult)
}
#[inline]
pub fn SSIZETToULongPtr(operand: SSIZE_T, pulResult: &mut ULONG_PTR) -> HRESULT {
    LongPtrToULongPtr(operand, pulResult)
}
#[inline]
pub fn SSIZETToDWord(operand: SSIZE_T, pdwResult: &mut DWORD) -> HRESULT {
    LongPtrToULong(operand, pdwResult)
}
#[inline]
pub fn SSIZETToDWordPtr(operand: SSIZE_T, pdwResult: &mut DWORD_PTR) -> HRESULT {
    LongPtrToULongPtr(operand, pdwResult)
}
#[inline]
pub fn SSIZETToULongLong(operand: SSIZE_T, pullResult: &mut ULONGLONG) -> HRESULT {
    LongPtrToULongLong(operand, pullResult)
}
#[inline]
pub fn SSIZETToDWordLong(operand: SSIZE_T, pdwResult: &mut DWORDLONG) -> HRESULT {
    LongPtrToULongLong(operand, pdwResult)
}
#[inline]
pub fn SSIZETToULong64(operand: SSIZE_T, pullResult: &mut ULONG64) -> HRESULT {
    LongPtrToULongLong(operand, pullResult)
}
#[inline]
pub fn SSIZETToDWord64(operand: SSIZE_T, pdwResult: &mut DWORD64) -> HRESULT {
    LongPtrToULongLong(operand, pdwResult)
}
#[inline]
pub fn SSIZETToUInt64(operand: SSIZE_T, pu64Result: &mut UINT64) -> HRESULT {
    LongPtrToULongLong(operand, pu64Result)
}
#[inline]
pub fn SSIZETToSizeT(operand: SSIZE_T, pResult: &mut size_t) -> HRESULT {
    LongPtrToUIntPtr(operand, pResult)
}
#[inline]
pub fn SSIZETToSIZET(operand: SSIZE_T, pResult: &mut SIZE_T) -> HRESULT {
    LongPtrToULongPtr(operand, pResult)
}
#[inline]
pub fn SIZETToInt8(operand: SIZE_T, pi8Result: &mut INT8) -> HRESULT {
    ULongPtrToInt8(operand, pi8Result)
}
#[inline]
pub fn SIZETToUChar(operand: SIZE_T, pch: &mut UCHAR) -> HRESULT {
    ULongPtrToUChar(operand, pch)
}
#[inline]
pub fn SIZETToChar(operand: SIZE_T, pch: &mut CHAR) -> HRESULT {
    ULongPtrToChar(operand, pch)
}
#[inline]
pub fn SIZETToUInt8(operand: SIZE_T, pu8Result: &mut UINT8) -> HRESULT {
    ULongPtrToUInt8(operand, pu8Result)
}
#[inline]
pub fn SIZETToByte(operand: SIZE_T, pu8Result: &mut BYTE) -> HRESULT {
    ULongPtrToUInt8(operand, pu8Result)
}
#[inline]
pub fn SIZETToShort(operand: SIZE_T, psResult: &mut SHORT) -> HRESULT {
    ULongPtrToShort(operand, psResult)
}
#[inline]
pub fn SIZETToInt16(operand: SIZE_T, pi16Result: &mut INT16) -> HRESULT {
    ULongPtrToShort(operand, pi16Result)
}
#[inline]
pub fn SIZETToUShort(operand: SIZE_T, pusResult: &mut USHORT) -> HRESULT {
    ULongPtrToUShort(operand, pusResult)
}
#[inline]
pub fn SIZETToUInt16(operand: SIZE_T, pu16Result: &mut UINT16) -> HRESULT {
    ULongPtrToUShort(operand, pu16Result)
}
#[inline]
pub fn SIZETToWord(operand: SIZE_T, pwResult: &mut WORD) -> HRESULT {
    ULongPtrToUShort(operand, pwResult)
}
#[inline]
pub fn SIZETToInt(operand: SIZE_T, piResult: &mut INT) -> HRESULT {
    ULongPtrToInt(operand, piResult)
}
#[inline]
pub fn SIZETToInt32(operand: SIZE_T, pi32Result: &mut INT32) -> HRESULT {
    ULongPtrToInt(operand, pi32Result)
}
#[inline]
pub fn SIZETToIntPtr(operand: SIZE_T, piResult: &mut INT_PTR) -> HRESULT {
    ULongPtrToIntPtr(operand, piResult)
}
#[inline]
pub fn SIZETToUInt(operand: SIZE_T, puResult: &mut UINT) -> HRESULT {
    ULongPtrToUInt(operand, puResult)
}
#[inline]
pub fn SIZETToUInt32(operand: SIZE_T, pu32Result: &mut UINT32) -> HRESULT {
    ULongPtrToUInt(operand, pu32Result)
}
#[inline]
pub fn SIZETToUIntPtr(operand: SIZE_T, puResult: &mut UINT_PTR) -> HRESULT {
    ULongPtrToUIntPtr(operand, puResult)
}
#[inline]
pub fn SIZETToLong(operand: SIZE_T, plResult: &mut LONG) -> HRESULT {
    ULongPtrToLong(operand, plResult)
}
#[inline]
pub fn SIZETToLongPtr(operand: SIZE_T, plResult: &mut LONG_PTR) -> HRESULT {
    ULongPtrToLongPtr(operand, plResult)
}
#[inline]
pub fn SIZETToULong(operand: SIZE_T, pulResult: &mut ULONG) -> HRESULT {
    ULongPtrToULong(operand, pulResult)
}
#[inline]
pub fn SIZETToDWord(operand: SIZE_T, pdwResult: &mut DWORD) -> HRESULT {
    ULongPtrToULong(operand, pdwResult)
}
#[inline]
pub fn SIZETToLongLong(operand: SIZE_T, pllResult: &mut LONGLONG) -> HRESULT {
    ULongPtrToLongLong(operand, pllResult)
}
#[inline]
pub fn SIZETToLong64(operand: SIZE_T, pllResult: &mut LONG64) -> HRESULT {
    ULongPtrToLongLong(operand, pllResult)
}
#[inline]
pub fn SIZETToInt64(operand: SIZE_T, pi64Result: &mut INT64) -> HRESULT {
    ULongPtrToLongLong(operand, pi64Result)
}
#[inline]
pub fn SIZETToPtrdiffT(operand: SIZE_T, pResult: &mut ptrdiff_t) -> HRESULT {
    ULongPtrToIntPtr(operand, pResult)
}
#[inline]
pub fn SIZETToSSIZET(operand: SIZE_T, pResult: &mut SSIZE_T) -> HRESULT {
    ULongPtrToLongPtr(operand, pResult)
}
#[inline]
pub fn UInt8Add(u8Augend: UINT8, u8Addend: UINT8, pu8Result: &mut UINT8) -> HRESULT {
    if (u8Augend + u8Addend) >= u8Augend {
        *pu8Result = u8Augend + u8Addend;
        S_OK
    } else {
        *pu8Result = UINT8_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn UShortAdd(usAugend: USHORT, usAddend: USHORT, pusResult: &mut USHORT) -> HRESULT {
    if (usAugend + usAddend) >= usAugend {
        *pusResult = usAugend + usAddend;
        S_OK
    } else {
        *pusResult = USHORT_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn UInt16Add(u16Augend: UINT16, u16Addend: UINT16, pu16Result: &mut UINT16) -> HRESULT {
    UShortAdd(u16Augend, u16Addend, pu16Result)
}
#[inline]
pub fn WordAdd(wAugend: WORD, wAddend: WORD, pwResult: &mut WORD) -> HRESULT {
    UShortAdd(wAugend, wAddend, pwResult)
}
#[inline]
pub fn UIntAdd(uAugend: UINT, uAddend: UINT, puResult: &mut UINT) -> HRESULT {
    if (uAugend + uAddend) >= uAugend {
        *puResult = uAugend + uAddend;
        S_OK
    } else {
        *puResult = UINT_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn UInt32Add(u32Augend: UINT32, u32Addend: UINT32, pu32Result: &mut UINT32) -> HRESULT {
    UIntAdd(u32Augend, u32Addend, pu32Result)
}
#[cfg(target_arch = "x86_64")] #[inline]
pub fn UIntPtrAdd(uAugend: UINT_PTR, uAddend: UINT_PTR, puResult: &mut UINT_PTR) -> HRESULT {
    ULongLongAdd(uAugend, uAddend, puResult)
}
#[cfg(target_arch = "x86")] #[inline]
pub fn UIntPtrAdd(uAugend: UINT_PTR, uAddend: UINT_PTR, puResult: &mut UINT_PTR) -> HRESULT {
    if (uAugend + uAddend) >= uAugend {
        *puResult = uAugend + uAddend;
        S_OK
    } else {
        *puResult = UINT_PTR_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn ULongAdd(ulAugend: ULONG, ulAddend: ULONG, pulResult: &mut ULONG) -> HRESULT {
    if (ulAugend + ulAddend) >= ulAugend {
        *pulResult = ulAugend + ulAddend;
        S_OK
    } else {
        *pulResult = ULONG_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[cfg(target_arch = "x86_64")] #[inline]
pub fn ULongPtrAdd(
    ulAugend: ULONG_PTR,
    ulAddend: ULONG_PTR,
    pulResult: &mut ULONG_PTR,
) -> HRESULT {
    ULongLongAdd(ulAugend, ulAddend, pulResult)
}
#[cfg(target_arch = "x86")] #[inline]
pub fn ULongPtrAdd(
    ulAugend: ULONG_PTR,
    ulAddend: ULONG_PTR,
    pulResult: &mut ULONG_PTR,
) -> HRESULT {
    if (ulAugend + ulAddend) >= ulAugend {
        *pulResult = ulAugend + ulAddend;
        S_OK
    } else {
        *pulResult = ULONG_PTR_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn DWordAdd(dwAugend: DWORD, dwAddend: DWORD, pdwResult: &mut DWORD) -> HRESULT {
    ULongAdd(dwAugend, dwAddend, pdwResult)
}
#[cfg(target_arch = "x86_64")] #[inline]
pub fn DWordPtrAdd(
    dwAugend: DWORD_PTR,
    dwAddend: DWORD_PTR,
    pdwResult: &mut DWORD_PTR,
) -> HRESULT {
    ULongLongAdd(dwAugend, dwAddend, pdwResult)
}
#[cfg(target_arch = "x86")] #[inline]
pub fn DWordPtrAdd(
    dwAugend: DWORD_PTR,
    dwAddend: DWORD_PTR,
    pdwResult: &mut DWORD_PTR,
) -> HRESULT {
    if (dwAugend + dwAddend) >= dwAugend {
        *pdwResult = dwAugend + dwAddend;
        S_OK
    } else {
        *pdwResult = DWORD_PTR_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn SizeTAdd(Augend: size_t, Addend: size_t, pResult: &mut size_t) -> HRESULT {
    if (Augend + Addend) >= Augend {
        *pResult = Augend + Addend;
        S_OK
    } else {
        *pResult = SIZE_T_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[cfg(target_arch = "x86_64")] #[inline]
pub fn SIZETAdd(Augend: SIZE_T, Addend: SIZE_T, pResult: &mut SIZE_T) -> HRESULT {
    ULongLongAdd(Augend, Addend, pResult)
}
#[cfg(target_arch = "x86")] #[inline]
pub fn SIZETAdd(Augend: SIZE_T, Addend: SIZE_T, pResult: &mut SIZE_T) -> HRESULT {
    if (Augend + Addend) >= Augend {
        *pResult = Augend + Addend;
        S_OK
    } else {
        *pResult = _SIZE_T_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn ULongLongAdd(
    ullAugend: ULONGLONG,
    ullAddend: ULONGLONG,
    pullResult: &mut ULONGLONG,
) -> HRESULT {
    if (ullAugend + ullAddend) >= ullAugend {
        *pullResult = ullAugend + ullAddend;
        S_OK
    } else {
        *pullResult = ULONGLONG_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn DWordLongAdd(
    dwAugend: DWORDLONG,
    dwAddend: DWORDLONG,
    pdwResult: &mut DWORDLONG,
) -> HRESULT {
    ULongLongAdd(dwAugend, dwAddend, pdwResult)
}
#[inline]
pub fn ULong64Add(ullAugend: ULONG64, ullAddend: ULONG64, pullResult: &mut ULONG64) -> HRESULT {
    ULongLongAdd(ullAugend, ullAddend, pullResult)
}
#[inline]
pub fn DWord64Add(dwAugend: DWORD64, dwAddend: DWORD64, pdwResult: &mut DWORD64) -> HRESULT {
    ULongLongAdd(dwAugend, dwAddend, pdwResult)
}
#[inline]
pub fn UInt64Add(u64Augend: UINT64, u64Addend: UINT64, pu64Result: &mut UINT64) -> HRESULT {
    ULongLongAdd(u64Augend, u64Addend, pu64Result)
}
#[inline]
pub fn UInt8Sub(u8Minuend: UINT8, u8Subtrahend: UINT8, pu8Result: &mut UINT8) -> HRESULT {
    if u8Minuend >= u8Subtrahend {
        *pu8Result = u8Minuend - u8Subtrahend;
        S_OK
    } else {
        *pu8Result = UINT8_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn UShortSub(usMinuend: USHORT, usSubtrahend: USHORT, pusResult: &mut USHORT) -> HRESULT {
    if usMinuend >= usSubtrahend {
        *pusResult = usMinuend - usSubtrahend;
        S_OK
    } else {
        *pusResult = USHORT_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn UInt16Sub(u16Minuend: UINT16, u16Subtrahend: UINT16, pu16Result: &mut UINT16) -> HRESULT {
    UShortSub(u16Minuend, u16Subtrahend, pu16Result)
}
#[inline]
pub fn WordSub(wMinuend: WORD, wSubtrahend: WORD, pwResult: &mut WORD) -> HRESULT {
    UShortSub(wMinuend, wSubtrahend, pwResult)
}
#[inline]
pub fn UIntSub(uMinuend: UINT, uSubtrahend: UINT, puResult: &mut UINT) -> HRESULT {
    if uMinuend >= uSubtrahend {
        *puResult = uMinuend - uSubtrahend;
        S_OK
    } else {
        *puResult = UINT_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn UInt32Sub(u32Minuend: UINT, u32Subtrahend: UINT, pu32Result: &mut UINT) -> HRESULT {
    UIntSub(u32Minuend, u32Subtrahend, pu32Result)
}
#[cfg(target_arch = "x86_64")] #[inline]
pub fn UIntPtrSub(uMinuend: UINT_PTR, uSubtrahend: UINT_PTR, puResult: &mut UINT_PTR) -> HRESULT {
    ULongLongSub(uMinuend, uSubtrahend, puResult)
}
#[cfg(target_arch = "x86")] #[inline]
pub fn UIntPtrSub(uMinuend: UINT_PTR, uSubtrahend: UINT_PTR, puResult: &mut UINT_PTR) -> HRESULT {
    if uMinuend >= uSubtrahend {
        *puResult = uMinuend - uSubtrahend;
        S_OK
    } else {
        *puResult = UINT_PTR_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn ULongSub(ulMinuend: ULONG, ulSubtrahend: ULONG, pulResult: &mut ULONG) -> HRESULT {
    if ulMinuend >= ulSubtrahend {
        *pulResult = ulMinuend - ulSubtrahend;
        S_OK
    } else {
        *pulResult = ULONG_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[cfg(target_arch = "x86_64")] #[inline]
pub fn ULongPtrSub(
    ulMinuend: ULONG_PTR,
    ulSubtrahend: ULONG_PTR,
    pulResult: &mut ULONG_PTR,
) -> HRESULT {
    ULongLongSub(ulMinuend, ulSubtrahend, pulResult)
}
#[cfg(target_arch = "x86")] #[inline]
pub fn ULongPtrSub(
    ulMinuend: ULONG_PTR,
    ulSubtrahend: ULONG_PTR,
    pulResult: &mut ULONG_PTR,
) -> HRESULT {
    if ulMinuend >= ulSubtrahend {
        *pulResult = ulMinuend - ulSubtrahend;
        S_OK
    } else {
        *pulResult = ULONG_PTR_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn DWordSub(dwMinuend: DWORD, dwSubtrahend: DWORD, pdwResult: &mut DWORD) -> HRESULT {
    ULongSub(dwMinuend, dwSubtrahend, pdwResult)
}
#[cfg(target_arch = "x86_64")] #[inline]
pub fn DWordPtrSub(
    dwMinuend: DWORD_PTR,
    dwSubtrahend: DWORD_PTR,
    pdwResult: &mut DWORD_PTR,
) -> HRESULT {
    ULongLongSub(dwMinuend, dwSubtrahend, pdwResult)
}
#[cfg(target_arch = "x86")] #[inline]
pub fn DWordPtrSub(
    dwMinuend: DWORD_PTR,
    dwSubtrahend: DWORD_PTR,
    pdwResult: &mut DWORD_PTR,
) -> HRESULT {
    if dwMinuend >= dwSubtrahend {
        *pdwResult = dwMinuend - dwSubtrahend;
        S_OK
    } else {
        *pdwResult = DWORD_PTR_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn SizeTSub(Minuend: size_t, Subtrahend: size_t, pResult: &mut size_t) -> HRESULT {
    if Minuend >= Subtrahend {
        *pResult = Minuend - Subtrahend;
        S_OK
    } else {
        *pResult = SIZE_T_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[cfg(target_arch = "x86_64")] #[inline]
pub fn SIZETSub(Minuend: SIZE_T, Subtrahend: SIZE_T, pResult: &mut SIZE_T) -> HRESULT {
    ULongLongSub(Minuend, Subtrahend, pResult)
}
#[cfg(target_arch = "x86")] #[inline]
pub fn SIZETSub(Minuend: SIZE_T, Subtrahend: SIZE_T, pResult: &mut SIZE_T) -> HRESULT {
    if Minuend >= Subtrahend {
        *pResult = Minuend - Subtrahend;
        S_OK
    } else {
        *pResult = _SIZE_T_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn ULongLongSub(
    ullMinuend: ULONGLONG,
    ullSubtrahend: ULONGLONG,
    pullResult: &mut ULONGLONG,
) -> HRESULT {
    if ullMinuend >= ullSubtrahend {
        *pullResult = ullMinuend - ullSubtrahend;
        S_OK
    } else {
        *pullResult = ULONGLONG_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn DWordLongSub(
    dwMinuend: DWORDLONG,
    dwSubtrahend: DWORDLONG,
    pdwResult: &mut DWORDLONG,
) -> HRESULT {
    ULongLongSub(dwMinuend, dwSubtrahend, pdwResult)
}
#[inline]
pub fn ULong64Sub(
    ullMinuend: ULONG64,
    ullSubtrahend: ULONG64,
    pullResult: &mut ULONG64,
) -> HRESULT {
    ULongLongSub(ullMinuend, ullSubtrahend, pullResult)
}
#[inline]
pub fn DWord64Sub(dwMinuend: DWORD64, dwSubtrahend: DWORD64, pdwResult: &mut DWORD64) -> HRESULT {
    ULongLongSub(dwMinuend, dwSubtrahend, pdwResult)
}
#[inline]
pub fn UInt64Sub(u64Minuend: UINT64, u64Subtrahend: UINT64, pu64Result: &mut UINT64) -> HRESULT {
    ULongLongSub(u64Minuend, u64Subtrahend, pu64Result)
}
#[inline]
pub fn UInt8Mult(u8Multiplicand: UINT8, u8Multiplier: UINT8, pu8Result: &mut UINT8) -> HRESULT {
    let uResult: UINT = (u8Multiplicand as UINT) * (u8Multiplier as UINT);
    UIntToUInt8(uResult, pu8Result)
}
#[inline]
pub fn UShortMult(
    usMultiplicand: USHORT,
    usMultiplier: USHORT,
    pusResult: &mut USHORT,
) -> HRESULT {
    let ulResult: ULONG = (usMultiplicand as ULONG) * (usMultiplier as ULONG);
    ULongToUShort(ulResult, pusResult)
}
#[inline]
pub fn UInt16Mult(
    u16Multiplicand: UINT16,
    u16Multiplier: UINT16,
    pu16Result: &mut UINT16,
) -> HRESULT {
    UShortMult(u16Multiplicand, u16Multiplier, pu16Result)
}
#[inline]
pub fn WordMult(wMultiplicand: WORD, wMultiplier: WORD, pwResult: &mut WORD) -> HRESULT {
    UShortMult(wMultiplicand, wMultiplier, pwResult)
}
#[inline]
pub fn UIntMult(uMultiplicand: UINT, uMultiplier: UINT, puResult: &mut UINT) -> HRESULT {
    let ull64Result: ULONGLONG = (uMultiplicand as ULONGLONG) * (uMultiplier as ULONGLONG);
    ULongLongToUInt(ull64Result, puResult)
}
#[inline]
pub fn UInt32Mult(
    u32Multiplicand: UINT32,
    u32Multiplier: UINT32,
    pu32Result: &mut UINT32,
) -> HRESULT {
    UIntMult(u32Multiplicand, u32Multiplier, pu32Result)
}
#[cfg(target_arch = "x86_64")] #[inline]
pub fn UIntPtrMult(
    uMultiplicand: UINT_PTR,
    uMultiplier: UINT_PTR,
    puResult: &mut UINT_PTR,
) -> HRESULT {
    ULongLongMult(uMultiplicand, uMultiplier, puResult)
}
#[cfg(target_arch = "x86")] #[inline]
pub fn UIntPtrMult(
    uMultiplicand: UINT_PTR,
    uMultiplier: UINT_PTR,
    puResult: &mut UINT_PTR,
) -> HRESULT {
    let ull64Result: ULONGLONG = (uMultiplicand as ULONGLONG) * (uMultiplier as ULONGLONG);
    ULongLongToUIntPtr(ull64Result, puResult)
}
#[inline]
pub fn ULongMult(ulMultiplicand: ULONG, ulMultiplier: ULONG, pulResult: &mut ULONG) -> HRESULT {
    let ull64Result: ULONGLONG = (ulMultiplicand as ULONGLONG) * (ulMultiplier as ULONGLONG);
    ULongLongToULong(ull64Result, pulResult)
}
#[cfg(target_arch = "x86_64")] #[inline]
pub fn ULongPtrMult(
    ulMultiplicand: ULONG_PTR,
    ulMultiplier: ULONG_PTR,
    pulResult: &mut ULONG_PTR,
) -> HRESULT {
    ULongLongMult(ulMultiplicand, ulMultiplier, pulResult)
}
#[cfg(target_arch = "x86")] #[inline]
pub fn ULongPtrMult(
    ulMultiplicand: ULONG_PTR,
    ulMultiplier: ULONG_PTR,
    pulResult: &mut ULONG_PTR,
) -> HRESULT {
    let ull64Result: ULONGLONG = (ulMultiplicand as ULONGLONG) * (ulMultiplier as ULONGLONG);
    ULongLongToULongPtr(ull64Result, pulResult)
}
#[inline]
pub fn DWordMult(dwMultiplicand: DWORD, dwMultiplier: DWORD, pdwResult: &mut DWORD) -> HRESULT {
    ULongMult(dwMultiplicand, dwMultiplier, pdwResult)
}
#[cfg(target_arch = "x86_64")] #[inline]
pub fn DWordPtrMult(
    dwMultiplicand: DWORD_PTR,
    dwMultiplier: DWORD_PTR,
    pdwResult: &mut DWORD_PTR,
) -> HRESULT {
    ULongLongMult(dwMultiplicand, dwMultiplier, pdwResult)
}
#[cfg(target_arch = "x86")] #[inline]
pub fn DWordPtrMult(
    dwMultiplicand: DWORD_PTR,
    dwMultiplier: DWORD_PTR,
    pdwResult: &mut DWORD_PTR,
) -> HRESULT {
    let ull64Result: ULONGLONG = (dwMultiplicand as ULONGLONG) * (dwMultiplier as ULONGLONG);
    ULongLongToDWordPtr(ull64Result, pdwResult)
}
#[cfg(target_arch = "x86_64")] #[inline]
pub fn SizeTMult(Multiplicand: size_t, Multiplier: size_t, pResult: &mut size_t) -> HRESULT {
    ULongLongMult(Multiplicand, Multiplier, pResult)
}
#[cfg(target_arch = "x86")] #[inline]
pub fn SizeTMult(Multiplicand: size_t, Multiplier: size_t, pResult: &mut size_t) -> HRESULT {
    let ull64Result: ULONGLONG = (Multiplicand as ULONGLONG) * (Multiplier as ULONGLONG);
    ULongLongToSizeT(ull64Result, pResult)
}
#[cfg(target_arch = "x86_64")] #[inline]
pub fn SIZETMult(Multiplicand: SIZE_T, Multiplier: SIZE_T, pResult: &mut SIZE_T) -> HRESULT {
    ULongLongMult(Multiplicand, Multiplier, pResult)
}
#[cfg(target_arch = "x86")] #[inline]
pub fn SIZETMult(Multiplicand: SIZE_T, Multiplier: SIZE_T, pResult: &mut SIZE_T) -> HRESULT {
    let ull64Result: ULONGLONG = (Multiplicand as ULONGLONG) * (Multiplier as ULONGLONG);
    ULongLongToSIZET(ull64Result, pResult)
}
#[inline]
pub fn ULongLongMult(
    ullMultiplicand: ULONGLONG,
    ullMultiplier: ULONGLONG,
    pullResult: &mut ULONGLONG,
) -> HRESULT {
    let dw_a: ULONG = (ullMultiplicand >> 32) as ULONG;
    let dw_c: ULONG = (ullMultiplier >> 32) as ULONG;
    if (dw_a == 0) && (dw_c == 0) {
        let dw_b: ULONG = ullMultiplicand as DWORD;
        let dw_d: ULONG = ullMultiplier as DWORD;
        *pullResult = (dw_b as ULONGLONG) * (dw_d as ULONGLONG);
        return S_OK;
    } else {
        if (dw_a == 0) || (dw_c == 0) {
            let dw_d: ULONG = ullMultiplier as DWORD;
            let ad: ULONGLONG = (dw_a as ULONGLONG) * (dw_d as ULONGLONG);
            if (ad & 0xffffffff00000000) == 0 {
                let dw_b: ULONG = ullMultiplicand as DWORD;
                let bc: ULONGLONG = (dw_b as ULONGLONG) * (dw_c as ULONGLONG);
                if (bc & 0xffffffff00000000) == 0 {
                    let mut ullResult: ULONGLONG = 0;
                    if SUCCEEDED(ULongLongAdd(bc << 32, ad << 32, &mut ullResult)) {
                        let bd: ULONGLONG = (dw_b as ULONGLONG) * (dw_d as ULONGLONG);
                        if SUCCEEDED(ULongLongAdd(ullResult, bd, &mut ullResult)) {
                            *pullResult = ullResult;
                            return S_OK;
                        }
                    }
                }
            }
        }
    }

    *pullResult = ULONGLONG_ERROR;
    INTSAFE_E_ARITHMETIC_OVERFLOW
}
#[inline]
pub fn DWordLongMult(
    dwMultiplicand: DWORDLONG,
    dwMultiplier: DWORDLONG,
    pdwResult: &mut DWORDLONG,
) -> HRESULT {
    ULongLongMult(dwMultiplicand, dwMultiplier, pdwResult)
}
#[inline]
pub fn ULong64Mult(
    ullMultiplicand: ULONG64,
    ullMultiplier: ULONG64,
    pullResult: &mut ULONG64,
) -> HRESULT {
    ULongLongMult(ullMultiplicand, ullMultiplier, pullResult)
}
#[inline]
pub fn DWord64Mult(
    dwMultiplicand: DWORD64,
    dwMultiplier: DWORD64,
    pdwResult: &mut DWORD64,
) -> HRESULT {
    ULongLongMult(dwMultiplicand, dwMultiplier, pdwResult)
}
#[inline]
pub fn UInt64Mult(
    u64Multiplicand: UINT64,
    u64Multiplier: UINT64,
    pu64Result: &mut UINT64,
) -> HRESULT {
    ULongLongMult(u64Multiplicand, u64Multiplier, pu64Result)
}
#[inline]
pub fn Int8Add(i8Augend: INT8, i8Addend: INT8, pi8Result: &mut INT8) -> HRESULT {
    LongToInt8((i8Augend as LONG) + (i8Addend as LONG), pi8Result)
}
#[inline]
pub fn ShortAdd(sAugend: SHORT, sAddend: SHORT, psResult: &mut SHORT) -> HRESULT {
    LongToShort((sAugend as LONG) + (sAddend as LONG), psResult)
}
#[inline]
pub fn Int16Add(i16Augend: INT16, i16Addend: INT16, pi16Result: &mut INT16) -> HRESULT {
    ShortAdd(i16Augend, i16Addend, pi16Result)
}
#[inline]
pub fn IntAdd(iAugend: INT, iAddend: INT, piResult: &mut INT) -> HRESULT {
    LongLongToInt((iAugend as LONGLONG) + (iAddend as LONGLONG), piResult)
}
#[inline]
pub fn Int32Add(i32Augend: INT32, i32Addend: INT32, pi32Result: &mut INT32) -> HRESULT {
    IntAdd(i32Augend, i32Addend, pi32Result)
}
#[cfg(target_arch = "x86_64")] #[inline]
pub fn IntPtrAdd(iAugend: INT_PTR, iAddend: INT_PTR, piResult: &mut INT_PTR) -> HRESULT {
    LongLongAdd(iAugend, iAddend, piResult)
}
#[cfg(target_arch = "x86")] #[inline]
pub fn IntPtrAdd(iAugend: INT_PTR, iAddend: INT_PTR, piResult: &mut INT_PTR) -> HRESULT {
    LongLongToIntPtr((iAugend as LONGLONG) + (iAddend as LONGLONG), piResult)
}
#[inline]
pub fn LongAdd(lAugend: LONG, lAddend: LONG, plResult: &mut LONG) -> HRESULT {
    LongLongToLong((lAugend as LONGLONG) + (lAddend as LONGLONG), plResult)
}
#[inline]
pub fn Long32Add(lAugend: LONG, lAddend: LONG, plResult: &mut LONG) -> HRESULT {
    IntAdd(lAugend, lAddend, plResult)
}
#[cfg(target_arch = "x86_64")] #[inline]
pub fn LongPtrAdd(lAugend: LONG_PTR, lAddend: LONG_PTR, plResult: &mut LONG_PTR) -> HRESULT {
    LongLongAdd(lAugend, lAddend, plResult)
}
#[cfg(target_arch = "x86")] #[inline]
pub fn LongPtrAdd(lAugend: LONG_PTR, lAddend: LONG_PTR, plResult: &mut LONG_PTR) -> HRESULT {
    LongLongToLongPtr((lAugend as LONGLONG) + (lAddend as LONGLONG), plResult)
}
#[inline]
pub fn LongLongAdd(llAugend: LONGLONG, llAddend: LONGLONG, pllResult: &mut LONGLONG) -> HRESULT {
    let llResult: LONGLONG = llAugend + llAddend;

    if ((llAugend < 0) == (llAddend < 0)) && ((llAugend < 0) != (llResult < 0)) {
        *pllResult = LONGLONG_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    } else {
        *pllResult = llResult;
        S_OK
    }
}
#[inline]
pub fn Long64Add(llAugend: LONG64, llAddend: LONG64, pllResult: &mut LONG64) -> HRESULT {
    LongLongAdd(llAugend, llAddend, pllResult)
}
#[inline]
pub fn Int64Add(i64Augend: INT64, i64Addend: INT64, pi64Result: &mut INT64) -> HRESULT {
    LongLongAdd(i64Augend, i64Addend, pi64Result)
}
#[cfg(target_arch = "x86_64")] #[inline]
pub fn PtrdiffTAdd(Augend: ptrdiff_t, Addend: ptrdiff_t, pResult: &mut ptrdiff_t) -> HRESULT {
    LongLongAdd(Augend, Addend, pResult)
}
#[cfg(target_arch = "x86")] #[inline]
pub fn PtrdiffTAdd(Augend: ptrdiff_t, Addend: ptrdiff_t, pResult: &mut ptrdiff_t) -> HRESULT {
    LongLongToLongPtr((Augend as LONGLONG) + (Addend as LONGLONG), pResult)
}
#[cfg(target_arch = "x86_64")] #[inline]
pub fn SSIZETAdd(Augend: SSIZE_T, Addend: SSIZE_T, pResult: &mut SSIZE_T) -> HRESULT {
    LongLongAdd(Augend, Addend, pResult)
}
#[cfg(target_arch = "x86")] #[inline]
pub fn SSIZETAdd(Augend: SSIZE_T, Addend: SSIZE_T, pResult: &mut SSIZE_T) -> HRESULT {
    LongLongToLongPtr((Augend as LONGLONG) + (Addend as LONGLONG), pResult)
}
#[inline]
pub fn Int8Sub(i8Minuend: INT8, i8Subtrahend: INT8, pi8Result: &mut INT8) -> HRESULT {
    LongToInt8((i8Minuend as LONG) - (i8Subtrahend as LONG), pi8Result)
}
#[inline]
pub fn ShortSub(sMinuend: SHORT, sSubtrahend: SHORT, psResult: &mut SHORT) -> HRESULT {
    LongToShort((sMinuend as LONG) - (sSubtrahend as LONG), psResult)
}
#[inline]
pub fn Int16Sub(i16Minuend: INT16, i16Subtrahend: INT16, pi16Result: &mut INT16) -> HRESULT {
    ShortSub(i16Minuend, i16Subtrahend, pi16Result)
}
#[inline]
pub fn IntSub(iMinuend: INT, iSubtrahend: INT, piResult: &mut INT) -> HRESULT {
    LongLongToInt((iMinuend as LONGLONG) - (iSubtrahend as LONGLONG), piResult)
}
#[inline]
pub fn Int32Sub(i32Minuend: INT32, i32Subtrahend: INT32, pi32Result: &mut INT32) -> HRESULT {
    IntSub(i32Minuend, i32Subtrahend, pi32Result)
}
#[cfg(target_arch = "x86_64")] #[inline]
pub fn IntPtrSub(iMinuend: INT_PTR, iSubtrahend: INT_PTR, piResult: &mut INT_PTR) -> HRESULT {
    LongLongSub(iMinuend, iSubtrahend, piResult)
}
#[cfg(target_arch = "x86")] #[inline]
pub fn IntPtrSub(iMinuend: INT_PTR, iSubtrahend: INT_PTR, piResult: &mut INT_PTR) -> HRESULT {
    LongLongToIntPtr((iMinuend as LONGLONG) - (iSubtrahend as LONGLONG), piResult)
}
#[inline]
pub fn LongSub(lMinuend: LONG, lSubtrahend: LONG, plResult: &mut LONG) -> HRESULT {
    LongLongToInt((lMinuend as LONGLONG) - (lSubtrahend as LONGLONG), plResult)
}
#[inline]
pub fn Long32Sub(lMinuend: LONG, lSubtrahend: LONG, plResult: &mut LONG) -> HRESULT {
    IntSub(lMinuend, lSubtrahend, plResult)
}
#[cfg(target_arch = "x86_64")] #[inline]
pub fn LongPtrSub(lMinuend: LONG_PTR, lSubtrahend: LONG_PTR, plResult: &mut LONG_PTR) -> HRESULT {
    LongLongSub(lMinuend, lSubtrahend, plResult)
}
#[cfg(target_arch = "x86")] #[inline]
pub fn LongPtrSub(lMinuend: LONG_PTR, lSubtrahend: LONG_PTR, plResult: &mut LONG_PTR) -> HRESULT {
    LongLongToLongPtr((lMinuend as LONGLONG) - (lSubtrahend as LONGLONG), plResult)
}
#[inline]
pub fn LongLongSub(
    llMinuend: LONGLONG,
    llSubtrahend: LONGLONG,
    pllResult: &mut LONGLONG,
) -> HRESULT {
    let llResult: LONGLONG = llMinuend - llSubtrahend;

    if ((llMinuend < 0) != (llSubtrahend < 0)) && ((llMinuend < 0) != (llResult < 0)) {
        *pllResult = LONGLONG_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    } else {
        *pllResult = llResult;
        S_OK
    }
}
#[inline]
pub fn Long64Sub(llMinuend: LONG64, llSubtrahend: LONG64, pllResult: &mut LONG64) -> HRESULT {
    LongLongSub(llMinuend, llSubtrahend, pllResult)
}
#[inline]
pub fn Int64Sub(i64Minuend: INT64, i64Subtrahend: INT64, pi64Result: &mut INT64) -> HRESULT {
    LongLongSub(i64Minuend, i64Subtrahend, pi64Result)
}
#[cfg(target_arch = "x86_64")] #[inline]
pub fn PtrdiffTSub(Minuend: ptrdiff_t, Subtrahend: ptrdiff_t, pResult: &mut ptrdiff_t) -> HRESULT {
    LongLongSub(Minuend, Subtrahend, pResult)
}
#[cfg(target_arch = "x86")] #[inline]
pub fn PtrdiffTSub(Minuend: ptrdiff_t, Subtrahend: ptrdiff_t, pResult: &mut ptrdiff_t) -> HRESULT {
    LongLongToPtrdiffT((Minuend as LONGLONG) - (Subtrahend as LONGLONG), pResult)
}
#[cfg(target_arch = "x86_64")] #[inline]
pub fn SSIZETSub(Minuend: SSIZE_T, Subtrahend: SSIZE_T, pResult: &mut SSIZE_T) -> HRESULT {
    LongLongSub(Minuend, Subtrahend, pResult)
}
#[cfg(target_arch = "x86")] #[inline]
pub fn SSIZETSub(Minuend: SSIZE_T, Subtrahend: SSIZE_T, pResult: &mut SSIZE_T) -> HRESULT {
    LongLongToSSIZET((Minuend as LONGLONG) - (Subtrahend as LONGLONG), pResult)
}
#[inline]
pub fn Int8Mult(i8Multiplicand: INT8, i8Multiplier: INT8, pi8Result: &mut INT8) -> HRESULT {
    LongToInt8((i8Multiplicand as LONG) * (i8Multiplier as LONG), pi8Result)
}
#[inline]
pub fn ShortMult(sMultiplicand: SHORT, sMultiplier: SHORT, psResult: &mut SHORT) -> HRESULT {
    LongToShort((sMultiplicand as LONG) * (sMultiplier as LONG), psResult)
}
#[inline]
pub fn Int16Mult(sMultiplicand: SHORT, sMultiplier: SHORT, psResult: &mut SHORT) -> HRESULT {
    ShortMult(sMultiplicand, sMultiplier, psResult)
}
#[inline]
pub fn IntMult(iMultiplicand: INT, iMultiplier: INT, piResult: &mut INT) -> HRESULT {
    LongLongToInt((iMultiplicand as LONGLONG) * (iMultiplier as LONGLONG), piResult)
}
#[inline]
pub fn Int32Mult(i32Multiplicand: INT32, i32Multiplier: INT32, pi32Result: &mut INT32) -> HRESULT {
    IntMult(i32Multiplicand, i32Multiplier, pi32Result)
}
#[cfg(target_arch = "x86_64")] #[inline]
pub fn IntPtrMult(
    iMultiplicand: INT_PTR,
    iMultiplier: INT_PTR,
    piResult: &mut INT_PTR,
) -> HRESULT {
    LongLongMult(iMultiplicand, iMultiplier, piResult)
}
#[cfg(target_arch = "x86")] #[inline]
pub fn IntPtrMult(
    iMultiplicand: INT_PTR,
    iMultiplier: INT_PTR,
    piResult: &mut INT_PTR,
) -> HRESULT {
    LongLongToIntPtr((iMultiplicand as LONGLONG) * (iMultiplier as LONGLONG), piResult)
}
#[inline]
pub fn LongMult(lMultiplicand: LONG, lMultiplier: LONG, plResult: &mut LONG) -> HRESULT {
    LongLongToLong((lMultiplicand as LONGLONG) * (lMultiplier as LONGLONG), plResult)
}
#[inline]
pub fn Long32Mult(lMultiplicand: LONG, lMultiplier: LONG, plResult: &mut LONG) -> HRESULT {
    IntMult(lMultiplicand, lMultiplier, plResult)
}
#[cfg(target_arch = "x86_64")] #[inline]
pub fn LongPtrMult(
    lMultiplicand: LONG_PTR,
    lMultiplier: LONG_PTR,
    plResult: &mut LONG_PTR,
) -> HRESULT {
    LongLongMult(lMultiplicand, lMultiplier, plResult)
}
#[cfg(target_arch = "x86")] #[inline]
pub fn LongPtrMult(
    lMultiplicand: LONG_PTR,
    lMultiplier: LONG_PTR,
    plResult: &mut LONG_PTR,
) -> HRESULT {
    LongLongToLongPtr((lMultiplicand as LONGLONG) * (lMultiplier as LONGLONG), plResult)
}
#[inline]
pub fn LongLongMult(
    llMultiplicand: LONGLONG,
    llMultiplier: LONGLONG,
    pllResult: &mut LONGLONG,
) -> HRESULT {
    let LONGLONG_MIN_MAGNITUDE = -(LONGLONG_MIN + 1) as ULONGLONG + 1;
    let ullMultiplicand = if llMultiplicand < 0 {
        -(llMultiplicand + 1) as ULONGLONG + 1
    } else {
        llMultiplicand as ULONGLONG
    };
    let ullMultiplier = if llMultiplier < 0 {
        -(llMultiplier + 1) as ULONGLONG + 1
    } else {
        llMultiplier as ULONGLONG
    };

    let mut ullResult: ULONGLONG = 0;
    if SUCCEEDED(ULongLongMult(ullMultiplicand, ullMultiplier, &mut ullResult)) {
        if (llMultiplicand < 0) != (llMultiplier < 0) {
            if ullResult > LONGLONG_MIN_MAGNITUDE {
                *pllResult = LONGLONG_ERROR;
                INTSAFE_E_ARITHMETIC_OVERFLOW
            } else {
                *pllResult = -(ullResult as LONGLONG);
                S_OK
            }
        } else {
            if ullResult > LONGLONG_MAX as ULONGLONG {
                *pllResult = LONGLONG_ERROR;
                INTSAFE_E_ARITHMETIC_OVERFLOW
            } else {
                *pllResult = ullResult as LONGLONG;
                S_OK
            }
        }
    } else {
        *pllResult = LONGLONG_ERROR;
        INTSAFE_E_ARITHMETIC_OVERFLOW
    }
}
#[inline]
pub fn Long64Mult(
    llMultiplicand: LONG64,
    llMultiplier: LONG64,
    pllResult: &mut LONG64,
) -> HRESULT {
    LongLongMult(llMultiplicand, llMultiplier, pllResult)
}
#[inline]
pub fn Int64Mult(i64Multiplicand: INT64, i64Multiplier: INT64, pi64Result: &mut INT64) -> HRESULT {
    LongLongMult(i64Multiplicand, i64Multiplier, pi64Result)
}
#[cfg(target_arch = "x86_64")] #[inline]
pub fn PtrdiffTMult(
    Multiplicand: ptrdiff_t,
    Multiplier: ptrdiff_t,
    pResult: &mut ptrdiff_t,
) -> HRESULT {
    LongLongMult(Multiplicand, Multiplier, pResult)
}
#[cfg(target_arch = "x86")] #[inline]
pub fn PtrdiffTMult(
    Multiplicand: ptrdiff_t,
    Multiplier: ptrdiff_t,
    pResult: &mut ptrdiff_t,
) -> HRESULT {
    LongLongToPtrdiffT((Multiplicand as LONGLONG) * (Multiplier as LONGLONG), pResult)
}
#[cfg(target_arch = "x86_64")] #[inline]
pub fn SSIZETMult(Multiplicand: SSIZE_T, Multiplier: SSIZE_T, pResult: &mut SSIZE_T) -> HRESULT {
    LongLongMult(Multiplicand, Multiplier, pResult)
}
#[cfg(target_arch = "x86")] #[inline]
pub fn SSIZETMult(Multiplicand: SSIZE_T, Multiplier: SSIZE_T, pResult: &mut SSIZE_T) -> HRESULT {
    LongLongToSSIZET((Multiplicand as LONGLONG) * (Multiplier as LONGLONG), pResult)
}
#[inline]
pub fn LOWORD(_dw: DWORD) -> WORD {
    ((_dw as DWORD_PTR) & 0xffff) as WORD
}
#[inline]
pub fn HIWORD(_dw: DWORD) -> WORD {
    (((_dw as DWORD_PTR) >> 16) & 0xffff) as WORD
}
#[inline]
pub fn LODWORD(_qw: ULONGLONG) -> DWORD {
    _qw as DWORD
}
#[inline]
pub fn HIDWORD(_qw: ULONGLONG) -> DWORD {
    ((_qw >> 32) as DWORD) & 0xffffffff
}

// The MIT License (MIT)
//
// Copyright (c) 2015 Johan Johansson
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

//! Simplify interfacing to C vtable representation of C++ classes that make use of inheritance

#![macro_use]
#![allow(dead_code)]

/// Rust equivalent to windows C DEFINE_GUID macro
#[macro_export]
macro_rules! define_guid {
    ($name:ident, $d1:expr, $d2:expr, $d3:expr,
        $d4:expr, $d5:expr, $d6:expr, $d7:expr, $d8:expr, $d9:expr, $d10:expr, $d11:expr) =>
    {
        pub const $name: GUID = GUID{ Data1: $d1, Data2: $d2, Data3: $d3,
            Data4: [$d4, $d5, $d6, $d7, $d8, $d9, $d10, $d11] };
    }
}
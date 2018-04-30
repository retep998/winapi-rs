// Copyright © 2015-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Mappings for the contents of dcommon.h
use shared::basetsd::UINT32;
use shared::dxgiformat::DXGI_FORMAT;
use shared::minwindef::FLOAT;
use shared::windef::{POINT, RECT};
ENUM!{enum DWRITE_MEASURING_MODE {
    DWRITE_MEASURING_MODE_NATURAL,
    DWRITE_MEASURING_MODE_GDI_CLASSIC,
    DWRITE_MEASURING_MODE_GDI_NATURAL,
}}
ENUM!{enum D2D1_ALPHA_MODE {
    D2D1_ALPHA_MODE_UNKNOWN = 0,
    D2D1_ALPHA_MODE_PREMULTIPLIED = 1,
    D2D1_ALPHA_MODE_STRAIGHT = 2,
    D2D1_ALPHA_MODE_IGNORE = 3,
    D2D1_ALPHA_MODE_FORCE_DWORD = 0xffffffff,
}}
STRUCT!{struct D2D1_PIXEL_FORMAT {
    format: DXGI_FORMAT,
    alphaMode: D2D1_ALPHA_MODE,
}}
STRUCT!{struct D2D_POINT_2U {
    x: UINT32,
    y: UINT32,
}}
STRUCT!{struct D2D_POINT_2F {
    x: FLOAT,
    y: FLOAT,
}}
pub type D2D_POINT_2L = POINT;
STRUCT!{struct D2D_VECTOR_2F {
    x: FLOAT,
    y: FLOAT,
}}
STRUCT!{struct D2D_VECTOR_3F {
    x: FLOAT,
    y: FLOAT,
    z: FLOAT,
}}
STRUCT!{struct D2D_VECTOR_4F {
    x: FLOAT,
    y: FLOAT,
    z: FLOAT,
    w: FLOAT,
}}
STRUCT!{struct D2D_RECT_F {
    left: FLOAT,
    top: FLOAT,
    right: FLOAT,
    bottom: FLOAT,
}}
STRUCT!{struct D2D_RECT_U {
    left: UINT32,
    top: UINT32,
    right: UINT32,
    bottom: UINT32,
}}
pub type D2D_RECT_L = RECT;
STRUCT!{struct D2D_SIZE_F {
    width: FLOAT,
    height: FLOAT,
}}
STRUCT!{struct D2D_SIZE_U {
    width: UINT32,
    height: UINT32,
}}
STRUCT!{struct D2D_MATRIX_3X2_F_u_s1 {
    m11: FLOAT,
    m12: FLOAT,
    m21: FLOAT,
    m22: FLOAT,
    dx: FLOAT,
    dy: FLOAT,
}}
STRUCT!{struct D2D_MATRIX_3X2_F_u_s2 {
    _11: FLOAT,
    _12: FLOAT,
    _21: FLOAT,
    _22: FLOAT,
    _31: FLOAT,
    _32: FLOAT,
}}
UNION!{union D2D_MATRIX_3X2_F_u {
    [u32; 6],
    s1 s1_mut: D2D_MATRIX_3X2_F_u_s1,
    s2 s2_mut: D2D_MATRIX_3X2_F_u_s2,
    m m_mut: [[FLOAT; 3]; 2],
}}
STRUCT!{struct D2D_MATRIX_3X2_F {
    u: D2D_MATRIX_3X2_F_u,
}}
STRUCT!{struct D2D_MATRIX_4X3_F_u_s {
    _11: FLOAT,
    _12: FLOAT,
    _13: FLOAT,
    _21: FLOAT,
    _22: FLOAT,
    _23: FLOAT,
    _31: FLOAT,
    _32: FLOAT,
    _33: FLOAT,
    _41: FLOAT,
    _42: FLOAT,
    _43: FLOAT,
}}
UNION!{union D2D_MATRIX_4X3_F_u {
    [u32; 12],
    s s_mut: D2D_MATRIX_4X3_F_u_s,
    m m_mut: [[FLOAT; 4]; 3],
}}
STRUCT!{struct D2D_MATRIX_4X3_F {
    u: D2D_MATRIX_4X3_F_u,
}}
STRUCT!{struct D2D_MATRIX_4X4_F_u_s {
    _11: FLOAT,
    _12: FLOAT,
    _13: FLOAT,
    _14: FLOAT,
    _21: FLOAT,
    _22: FLOAT,
    _23: FLOAT,
    _24: FLOAT,
    _31: FLOAT,
    _32: FLOAT,
    _33: FLOAT,
    _34: FLOAT,
    _41: FLOAT,
    _42: FLOAT,
    _43: FLOAT,
    _44: FLOAT,
}}
UNION!{union D2D_MATRIX_4X4_F_u {
    [u32; 16],
    s s_mut: D2D_MATRIX_4X4_F_u_s,
    m m_mut: [[FLOAT; 4]; 4],
}}
STRUCT!{struct D2D_MATRIX_4X4_F {
    u: D2D_MATRIX_4X4_F_u,
}}
STRUCT!{struct D2D_MATRIX_5X4_F_u_s {
    _11: FLOAT,
    _12: FLOAT,
    _13: FLOAT,
    _14: FLOAT,
    _21: FLOAT,
    _22: FLOAT,
    _23: FLOAT,
    _24: FLOAT,
    _31: FLOAT,
    _32: FLOAT,
    _33: FLOAT,
    _34: FLOAT,
    _41: FLOAT,
    _42: FLOAT,
    _43: FLOAT,
    _44: FLOAT,
    _51: FLOAT,
    _52: FLOAT,
    _53: FLOAT,
    _54: FLOAT,
}}
UNION!{union D2D_MATRIX_5X4_F_u {
    [u32; 20],
    s s_mut: D2D_MATRIX_5X4_F_u_s,
    m m_mut: [[FLOAT; 5]; 4],
}}
STRUCT!{struct D2D_MATRIX_5X4_F {
    u: D2D_MATRIX_5X4_F_u,
}}
pub type D2D1_POINT_2F = D2D_POINT_2F;
pub type D2D1_POINT_2U = D2D_POINT_2U;
pub type D2D1_POINT_2L = D2D_POINT_2L;
pub type D2D1_RECT_F = D2D_RECT_F;
pub type D2D1_RECT_U = D2D_RECT_U;
pub type D2D1_RECT_L = D2D_RECT_L;
pub type D2D1_SIZE_F = D2D_SIZE_F;
pub type D2D1_SIZE_U = D2D_SIZE_U;
pub type D2D1_MATRIX_3X2_F = D2D_MATRIX_3X2_F;

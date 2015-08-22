// Copyright © 2015, Connor Hilarides
// Licensed under the MIT License <LICENSE.md>
//! Mappings for the contents of d2dbasetypes.h

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D2D_POINT_2U {
    pub x: ::UINT32,
    pub y: ::UINT32,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D2D_POINT_2F {
    pub x: ::FLOAT,
    pub y: ::FLOAT,
}

pub type D2D_POINT_2L = ::POINT;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D2D_VECTOR_2F {
    pub x: ::FLOAT,
    pub y: ::FLOAT,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D2D_VECTOR_3F {
    pub x: ::FLOAT,
    pub y: ::FLOAT,
    pub z: ::FLOAT,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D2D_VECTOR_4F {
    pub x: ::FLOAT,
    pub y: ::FLOAT,
    pub z: ::FLOAT,
    pub w: ::FLOAT,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D2D_RECT_F {
    pub left: ::FLOAT,
    pub top: ::FLOAT,
    pub right: ::FLOAT,
    pub bottom: ::FLOAT,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D2D_RECT_U {
    pub left: ::UINT32,
    pub top: ::UINT32,
    pub right: ::UINT32,
    pub bottom: ::UINT32,
}

pub type D2D_RECT_L = ::RECT;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D2D_SIZE_F {
    pub width: ::FLOAT,
    pub height: ::FLOAT,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D2D_SIZE_U {
    pub width: ::UINT32,
    pub height: ::UINT32,
}

pub type D2D_COLOR_F = ::D3DCOLORVALUE;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D2D_MATRIX_3X2_F {
    pub matrix: [[::FLOAT; 3]; 2],
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D2D_MATRIX_4X3_F {
    pub matrix: [[::FLOAT; 4]; 3],
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D2D_MATRIX_4X4_F {
    pub matrix: [[::FLOAT; 4]; 4],
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct D2D_MATRIX_5X4_F {
    pub matrix: [[::FLOAT; 5]; 4],
}

// Copyright © 2018 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use _core::{cmp, f32, ops};
use ctypes::c_float;
use shared::minwindef::{BOOL, BYTE, INT, UINT};
use um::gdiplusenums::EmfPlusRecordType;
use um::winnt::VOID;
fn min(x: f32, y: f32) -> f32 {
    if y < x { y } else { x }
}
fn max(x: f32, y: f32) -> f32 {
    if x < y { y } else { x }
}
pub type ImageAbort = extern "system" fn(*mut VOID) -> BOOL;
pub type DrawImageAbort = ImageAbort;
pub type GetThumbnailImageAbort = ImageAbort;
pub type EnumerateMetafileProc = extern "system" fn(
    EmfPlusRecordType,
    UINT,
    UINT,
    *const BYTE,
    *mut VOID,
) -> BOOL;
// struct __declspec(novtable) GdiplusAbort
// {
//     virtual HRESULT __stdcall Abort(void) = 0;
// };
pub type REAL = c_float;
pub const REAL_MAX: REAL = f32::MAX;
pub const REAL_MIN: REAL = f32::MIN_POSITIVE;
pub const REAL_TOLERANCE: REAL = (f32::MIN_POSITIVE * 100.0);
pub const REAL_EPSILON: REAL = 1.192092896e-07f32;
ENUM!{enum Status {
    Ok = 0,
    GenericError = 1,
    InvalidParameter = 2,
    OutOfMemory = 3,
    ObjectBusy = 4,
    InsufficientBuffer = 5,
    NotImplemented = 6,
    Win32Error = 7,
    WrongState = 8,
    Aborted = 9,
    FileNotFound = 10,
    ValueOverflow = 11,
    AccessDenied = 12,
    UnknownImageFormat = 13,
    FontFamilyNotFound = 14,
    FontStyleNotFound = 15,
    NotTrueTypeFont = 16,
    UnsupportedGdiplusVersion = 17,
    GdiplusNotInitialized = 18,
    PropertyNotFound = 19,
    PropertyNotSupported = 20,
    ProfileNotFound = 21,
}}
STRUCT!{#[derive(PartialEq)] struct SizeF {
    Width: REAL,
    Height: REAL,
}}
impl SizeF {
    pub fn new(width: REAL, height: REAL) -> Self {
        SizeF {
            Width: width,
            Height: height,
        }
    }
    pub fn Equals(&self, other: &SizeF) -> bool {
        self == other
    }
    pub fn Empty(&self) -> bool {
        self.Width == 0.0 && self.Height == 0.0
    }
}
impl ops::Add for SizeF {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            Width: self.Width + rhs.Width,
            Height: self.Height + rhs.Height,
        }
    }
}
impl<'a> ops::Add<&'a SizeF> for SizeF {
    type Output = Self;
    fn add(self, rhs: &'a SizeF) -> Self::Output {
        self + *rhs
    }
}
impl<'a, 'b> ops::Add<&'a SizeF> for &'b SizeF {
    type Output = SizeF;
    fn add(self, rhs: &'a SizeF) -> Self::Output {
        *self + *rhs
    }
}
impl<'a> ops::Add<SizeF> for &'a SizeF {
    type Output = SizeF;
    fn add(self, rhs: SizeF) -> Self::Output {
        *self + rhs
    }
}
impl ops::Sub for SizeF {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            Width: self.Width - rhs.Width,
            Height: self.Height - rhs.Height,
        }
    }
}
impl<'a> ops::Sub<&'a SizeF> for SizeF {
    type Output = Self;
    fn sub(self, rhs: &'a SizeF) -> Self::Output {
        self - *rhs
    }
}
impl<'a, 'b> ops::Sub<&'a SizeF> for &'b SizeF {
    type Output = SizeF;
    fn sub(self, rhs: &'a SizeF) -> Self::Output {
        *self - *rhs
    }
}
impl<'a> ops::Sub<SizeF> for &'a SizeF {
    type Output = SizeF;
    fn sub(self, rhs: SizeF) -> Self::Output {
        *self - rhs
    }
}
STRUCT!{#[derive(PartialEq, Eq)] struct Size {
    Width: INT,
    Height: INT,
}}
impl Size {
    pub fn new(width: INT, height: INT) -> Self {
        Size {
            Width: width,
            Height: height,
        }
    }
    pub fn Equals(&self, other: &Size) -> bool {
        self == other
    }
    pub fn Empty(&self) -> bool {
        self.Width == 0 && self.Height == 0
    }
}
impl ops::Add for Size {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            Width: self.Width + rhs.Width,
            Height: self.Height + rhs.Height,
        }
    }
}
impl<'a> ops::Add<&'a Size> for Size {
    type Output = Self;
    fn add(self, rhs: &'a Size) -> Self::Output {
        self + *rhs
    }
}
impl<'a, 'b> ops::Add<&'a Size> for &'b Size {
    type Output = Size;
    fn add(self, rhs: &'a Size) -> Self::Output {
        *self + *rhs
    }
}
impl<'a> ops::Add<Size> for &'a Size {
    type Output = Size;
    fn add(self, rhs: Size) -> Self::Output {
        *self + rhs
    }
}
impl ops::Sub for Size {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            Width: self.Width - rhs.Width,
            Height: self.Height - rhs.Height,
        }
    }
}
impl<'a> ops::Sub<&'a Size> for Size {
    type Output = Self;
    fn sub(self, rhs: &'a Size) -> Self::Output {
        self - *rhs
    }
}
impl<'a, 'b> ops::Sub<&'a Size> for &'b Size {
    type Output = Size;
    fn sub(self, rhs: &'a Size) -> Self::Output {
        *self - *rhs
    }
}
impl<'a> ops::Sub<Size> for &'a Size {
    type Output = Size;
    fn sub(self, rhs: Size) -> Self::Output {
        *self - rhs
    }
}
STRUCT!{#[derive(PartialEq)] struct PointF {
    X: REAL,
    Y: REAL,
}}
impl PointF {
    pub fn new(x: REAL, y: REAL) -> Self {
        PointF {
            X: x,
            Y: y,
        }
    }
    pub fn Equals(&self, other: &PointF) -> bool {
        self == other
    }
}
impl ops::Add for PointF {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            X: self.X + rhs.X,
            Y: self.Y + rhs.Y,
        }
    }
}
impl<'a> ops::Add<&'a PointF> for PointF {
    type Output = Self;
    fn add(self, rhs: &'a PointF) -> Self::Output {
        self + *rhs
    }
}
impl<'a, 'b> ops::Add<&'a PointF> for &'b PointF {
    type Output = PointF;
    fn add(self, rhs: &'a PointF) -> Self::Output {
        *self + *rhs
    }
}
impl<'a> ops::Add<PointF> for &'a PointF {
    type Output = PointF;
    fn add(self, rhs: PointF) -> Self::Output {
        *self + rhs
    }
}
impl ops::Sub for PointF {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            X: self.X - rhs.X,
            Y: self.Y - rhs.Y,
        }
    }
}
impl<'a> ops::Sub<&'a PointF> for PointF {
    type Output = Self;
    fn sub(self, rhs: &'a PointF) -> Self::Output {
        self - *rhs
    }
}
impl<'a, 'b> ops::Sub<&'a PointF> for &'b PointF {
    type Output = PointF;
    fn sub(self, rhs: &'a PointF) -> Self::Output {
        *self - *rhs
    }
}
impl<'a> ops::Sub<PointF> for &'a PointF {
    type Output = PointF;
    fn sub(self, rhs: PointF) -> Self::Output {
        *self - rhs
    }
}
STRUCT!{#[derive(PartialEq, Eq)] struct Point {
    X: INT,
    Y: INT,
}}
impl Point {
    pub fn new(x: INT, y: INT) -> Self {
        Point {
            X: x,
            Y: y,
        }
    }
    pub fn Equals(&self, other: &Point) -> bool {
        self == other
    }
}
impl ops::Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            X: self.X + rhs.X,
            Y: self.Y + rhs.Y,
        }
    }
}
impl<'a> ops::Add<&'a Point> for Point {
    type Output = Self;
    fn add(self, rhs: &'a Point) -> Self::Output {
        self + *rhs
    }
}
impl<'a, 'b> ops::Add<&'a Point> for &'b Point {
    type Output = Point;
    fn add(self, rhs: &'a Point) -> Self::Output {
        *self + *rhs
    }
}
impl<'a> ops::Add<Point> for &'a Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Self::Output {
        *self + rhs
    }
}
impl ops::Sub for Point {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            X: self.X - rhs.X,
            Y: self.Y - rhs.Y,
        }
    }
}
impl<'a> ops::Sub<&'a Point> for Point {
    type Output = Self;
    fn sub(self, rhs: &'a Point) -> Self::Output {
        self - *rhs
    }
}
impl<'a, 'b> ops::Sub<&'a Point> for &'b Point {
    type Output = Point;
    fn sub(self, rhs: &'a Point) -> Self::Output {
        *self - *rhs
    }
}
impl<'a> ops::Sub<Point> for &'a Point {
    type Output = Point;
    fn sub(self, rhs: Point) -> Self::Output {
        *self - rhs
    }
}
STRUCT!{#[derive(PartialEq)] struct RectF {
    X: REAL,
    Y: REAL,
    Width: REAL,
    Height: REAL,
}}
impl RectF {
    pub fn new(location: PointF, size: SizeF) -> Self {
        RectF {
            X: location.X,
            Y: location.Y,
            Width: size.Width,
            Height: size.Height,
        }
    }
    pub fn GetLocation(&self) -> PointF {
        PointF {
            X: self.X,
            Y: self.Y,
        }
    }
    pub fn GetSize(&self) -> SizeF {
        SizeF {
            Width: self.Width,
            Height: self.Height,
        }
    }
    pub fn GetLeft(&self) -> REAL {
        self.X
    }
    pub fn GetTop(&self) -> REAL {
        self.Y
    }
    pub fn GetRight(&self) -> REAL {
        self.X + self.Width
    }
    pub fn GetBottom(&self) -> REAL {
        self.Y + self.Height
    }
    pub fn IsEmptyArea(&self) -> bool {
        self.Width <= REAL_EPSILON || self.Height <= REAL_EPSILON
    }
    pub fn Equals(&self, other: &Self) -> bool {
        self == other
    }
    pub fn Contains(&self, p: PointF) -> bool {
        p.X >= self.GetLeft() && p.X < self.GetRight() &&
        p.Y >= self.GetTop() && p.Y < self.GetBottom()
    }
    pub fn Inflate(&mut self, x: REAL, y: REAL) {
        self.X -= x;
        self.Y -= y;
        self.Width += x * 2.0;
        self.Height += y * 2.0;
    }
    pub fn Intersect(&mut self, other: &Self) -> bool {
        let r = min(self.GetRight(), other.GetRight());
        let b = min(self.GetBottom(), other.GetBottom());
        let l = min(self.GetLeft(), other.GetLeft());
        let t = min(self.GetTop(), other.GetTop());

        self.X = l;
        self.Y = t;
        self.Width = r - l;
        self.Height = b - t;
        !self.IsEmptyArea()
    }
    pub fn IntersectsWith(&self, other: &Self) -> bool {
        self.GetLeft() < other.GetRight() &&
        self.GetTop() < other.GetBottom() &&
        self.GetRight() > other.GetLeft() &&
        self.GetBottom() > other.GetTop()
    }
    pub fn Union(&mut self, other: &Self) -> bool {
        let r = max(self.GetRight(), other.GetRight());
        let b = max(self.GetBottom(), other.GetBottom());
        let l = min(self.GetLeft(), other.GetLeft());
        let t = min(self.GetTop(), other.GetTop());

        self.X = l;
        self.Y = t;
        self.Width = r - l;
        self.Height = b - t;
        !self.IsEmptyArea()
    }
    pub fn Offset(&mut self, x: REAL, y: REAL) {
        self.X += x;
        self.Y += y;
    }
}
STRUCT!{#[derive(PartialEq, Eq)] struct Rect {
    X: INT,
    Y: INT,
    Width: INT,
    Height: INT,
}}
impl Rect {
    pub fn new(location: Point, size: Size) -> Self {
        Rect {
            X: location.X,
            Y: location.Y,
            Width: size.Width,
            Height: size.Height,
        }
    }
    pub fn GetLocation(&self) -> Point {
        Point {
            X: self.X,
            Y: self.Y,
        }
    }
    pub fn GetSize(&self) -> Size {
        Size {
            Width: self.Width,
            Height: self.Height,
        }
    }
    pub fn GetLeft(&self) -> INT {
        self.X
    }
    pub fn GetTop(&self) -> INT {
        self.Y
    }
    pub fn GetRight(&self) -> INT {
        self.X + self.Width
    }
    pub fn GetBottom(&self) -> INT {
        self.Y + self.Height
    }
    pub fn IsEmptyArea(&self) -> bool {
        self.Width <= 0 || self.Height <= 0
    }
    pub fn Equals(&self, other: &Self) -> bool {
        self == other
    }
    pub fn Contains(&self, p: Point) -> bool {
        p.X >= self.GetLeft() && p.X < self.GetRight() &&
        p.Y >= self.GetTop() && p.Y < self.GetBottom()
    }
    pub fn Inflate(&mut self, x: INT, y: INT) {
        self.X -= x;
        self.Y -= y;
        self.Width += x * 2;
        self.Height += y * 2;
    }
    pub fn Intersect(&mut self, other: &Self) -> bool {
        let r = cmp::min(self.GetRight(), other.GetRight());
        let b = cmp::min(self.GetBottom(), other.GetBottom());
        let l = cmp::min(self.GetLeft(), other.GetLeft());
        let t = cmp::min(self.GetTop(), other.GetTop());

        self.X = l;
        self.Y = t;
        self.Width = r - l;
        self.Height = b - t;
        !self.IsEmptyArea()
    }
    pub fn IntersectsWith(&self, other: &Self) -> bool {
        self.GetLeft() < other.GetRight() &&
        self.GetTop() < other.GetBottom() &&
        self.GetRight() > other.GetLeft() &&
        self.GetBottom() > other.GetTop()
    }
    pub fn Union(&mut self, other: &Self) -> bool {
        let r = cmp::max(self.GetRight(), other.GetRight());
        let b = cmp::max(self.GetBottom(), other.GetBottom());
        let l = cmp::min(self.GetLeft(), other.GetLeft());
        let t = cmp::min(self.GetTop(), other.GetTop());

        self.X = l;
        self.Y = t;
        self.Width = r - l;
        self.Height = b - t;
        !self.IsEmptyArea()
    }
    pub fn Offset(&mut self, x: INT, y: INT) {
        self.X += x;
        self.Y += y;
    }
}
#[repr(C)]
pub struct PathData {
    pub Count: INT,
    pub Points: *mut PointF,
    pub Types: *mut BYTE,
}
STRUCT!{struct CharacterRange {
    First: INT,
    Length: INT,
}}

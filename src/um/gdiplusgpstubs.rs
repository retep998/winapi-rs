// Copyright © 2018 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use um::gdiplusenums::{
  BrushType, CoordinateSpace, DashCap, DashStyle, FillMode, FlushIntention, HatchStyle, LineCap,
  LineJoin, MatrixOrder, PenAlignment, PenType, Unit, WrapMode
};
use um::gdiplustypes::{PathData, Point, PointF, Rect, RectF, SizeF, Status};
pub enum GpGraphics {}
pub enum GpBrush {}
pub enum GpTexture {}
pub enum GpSolidFill {}
pub enum GpLineGradient {}
pub enum GpPathGradient {}
pub enum GpHatch {}
pub enum GpPen {}
pub enum GpCustomLineCap {}
pub enum GpAdjustableArrowCap {}
pub enum GpImage {}
pub enum GpBitmap {}
pub enum GpMetafile {}
pub enum GpImageAttributes {}
pub enum GpPath {}
pub enum GpRegion {}
pub enum GpPathIterator {}
pub enum GpFontFamily {}
pub enum GpFont {}
pub enum GpStringFormat {}
pub enum GpFontCollection {}
pub enum GpInstalledFontCollection {}
pub enum GpPrivateFontCollection {}
pub enum GpCachedBitmap {}
pub type GpStatus = Status;
pub type GpFillMode = FillMode;
pub type GpWrapMode = WrapMode;
pub type GpUnit = Unit;
pub type GpCoordinateSpace = CoordinateSpace;
pub type GpPointF = PointF;
pub type GpPoint = Point;
pub type GpRectF = RectF;
pub type GpRect = Rect;
pub type GpSizeF = SizeF;
pub type GpHatchStyle = HatchStyle;
pub type GpDashStyle = DashStyle;
pub type GpLineCap = LineCap;
pub type GpDashCap = DashCap;
pub type GpPenAlignment = PenAlignment;
pub type GpLineJoin = LineJoin;
pub type GpPenType = PenType;
pub enum GpMatrix {}
pub type GpBrushType = BrushType;
pub type GpMatrixOrder = MatrixOrder;
pub type GpFlushIntention = FlushIntention;
pub type GpPathData = PathData;

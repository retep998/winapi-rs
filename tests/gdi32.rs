// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
#![cfg(windows)]
extern crate gdi32;
use gdi32::*;
#[inline(never)] fn bb<T>(_: T) {}
#[test]
fn functions() {
    bb(Chord);
    bb(CreateBitmap);
    bb(CreateBitmapIndirect);
    bb(CreateDIBitmap);
    bb(CreateDIBSection);
    bb(Ellipse);
    bb(GetDIBits);
    // Not in MingGW yet
    // bb(Pie);
    bb(Polygon);
    bb(PolyPolygon);
    bb(RoundRect);
    bb(AngleArc);
    bb(Arc);
    bb(ArcTo);
    bb(GetArcDirection);
    bb(LineDDA);
    bb(LineTo);
    bb(MoveToEx);
    bb(PolyBezier);
    bb(PolyBezierTo);
    bb(PolyDraw);
    bb(PolyPolygon);
    bb(PolyPolyline);
    bb(Polyline);
    bb(PolylineTo);
    bb(SetArcDirection);
}

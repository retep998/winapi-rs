// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
#![feature(test)]
#![cfg(windows)]
extern crate gdi32;
extern crate test;
use gdi32::*;
use test::black_box as bb;
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

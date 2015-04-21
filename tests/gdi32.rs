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
    bb(Ellipse);
    bb(Pie);
    bb(Polygon);
    bb(PolyPolygon);
    bb(RoundRect);
}

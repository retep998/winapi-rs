// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
#![cfg(windows)]
extern crate winapi;
use std::mem::{size_of, align_of};
use winapi::*;
#[test] #[cfg(target_arch = "i686")]
fn template() {
}
#[test] #[cfg(target_arch = "x86_64")]
fn template() {
}
#[test] #[cfg(target_arch = "i686")]
fn hidclass() {
    assert_eq!(size_of::<HID_XFER_PACKET>(), 12);
    assert_eq!(align_of::<HID_XFER_PACKET>(), 4);
    assert_eq!(size_of::<HID_COLLECTION_INFORMATION>(), 12);
    assert_eq!(align_of::<HID_COLLECTION_INFORMATION>(), 4);
    assert_eq!(size_of::<HID_DRIVER_CONFIG>(), 8);
    assert_eq!(align_of::<HID_DRIVER_CONFIG>(), 4);
}
#[test] #[cfg(target_arch = "x86_64")]
fn hidclass() {
    assert_eq!(size_of::<HID_XFER_PACKET>(), 16);
    assert_eq!(align_of::<HID_XFER_PACKET>(), 8);
    assert_eq!(size_of::<HID_COLLECTION_INFORMATION>(), 12);
    assert_eq!(align_of::<HID_COLLECTION_INFORMATION>(), 4);
    assert_eq!(size_of::<HID_DRIVER_CONFIG>(), 8);
    assert_eq!(align_of::<HID_DRIVER_CONFIG>(), 4);
}
#[test] #[cfg(target_arch = "i686")]
fn hidpi() {
    assert_eq!(size_of::<USAGE_AND_PAGE>(), 4);
    assert_eq!(align_of::<USAGE_AND_PAGE>(), 2);
    assert_eq!(size_of::<HIDP_BUTTON_CAPS>(), 72);
    assert_eq!(align_of::<HIDP_BUTTON_CAPS>(), 4);
    assert_eq!(size_of::<HIDP_VALUE_CAPS>(), 72);
    assert_eq!(align_of::<HIDP_VALUE_CAPS>(), 4);
    assert_eq!(size_of::<HIDP_LINK_COLLECTION_NODE>(), 20);
    assert_eq!(align_of::<HIDP_LINK_COLLECTION_NODE>(), 4);
    assert_eq!(size_of::<HIDP_CAPS>(), 64);
    assert_eq!(align_of::<HIDP_CAPS>(), 2);
    assert_eq!(size_of::<HIDP_DATA>(), 8);
    assert_eq!(align_of::<HIDP_DATA>(), 4);
    assert_eq!(size_of::<HIDP_UNKNOWN_TOKEN>(), 8);
    assert_eq!(align_of::<HIDP_UNKNOWN_TOKEN>(), 4);
    assert_eq!(size_of::<HIDP_EXTENDED_ATTRIBUTES>(), 12);
    assert_eq!(align_of::<HIDP_EXTENDED_ATTRIBUTES>(), 4);
    assert_eq!(size_of::<HIDP_KEYBOARD_MODIFIER_STATE>(), 4);
    assert_eq!(align_of::<HIDP_KEYBOARD_MODIFIER_STATE>(), 4);
}
#[test] #[cfg(target_arch = "x86_64")]
fn hidpi() {
    assert_eq!(size_of::<USAGE_AND_PAGE>(), 4);
    assert_eq!(align_of::<USAGE_AND_PAGE>(), 2);
    assert_eq!(size_of::<HIDP_BUTTON_CAPS>(), 72);
    assert_eq!(align_of::<HIDP_BUTTON_CAPS>(), 4);
    assert_eq!(size_of::<HIDP_VALUE_CAPS>(), 72);
    assert_eq!(align_of::<HIDP_VALUE_CAPS>(), 4);
    assert_eq!(size_of::<HIDP_LINK_COLLECTION_NODE>(), 24);
    // assert_eq!(align_of::<HIDP_LINK_COLLECTION_NODE>(), 4); // FIXME
    assert_eq!(size_of::<HIDP_CAPS>(), 64);
    assert_eq!(align_of::<HIDP_CAPS>(), 2);
    assert_eq!(size_of::<HIDP_DATA>(), 8);
    assert_eq!(align_of::<HIDP_DATA>(), 4);
    assert_eq!(size_of::<HIDP_UNKNOWN_TOKEN>(), 8);
    assert_eq!(align_of::<HIDP_UNKNOWN_TOKEN>(), 4);
    // assert_eq!(size_of::<HIDP_EXTENDED_ATTRIBUTES>(), 16); // FIXME
    // assert_eq!(align_of::<HIDP_EXTENDED_ATTRIBUTES>(), 4); // FIXME
    assert_eq!(size_of::<HIDP_KEYBOARD_MODIFIER_STATE>(), 4);
    assert_eq!(align_of::<HIDP_KEYBOARD_MODIFIER_STATE>(), 4);
}
#[test] #[cfg(target_arch = "i686")]
fn hidsdi() {
    assert_eq!(size_of::<HIDD_CONFIGURATION>(), 12);
    assert_eq!(align_of::<HIDD_CONFIGURATION>(), 4);
    assert_eq!(size_of::<HIDD_ATTRIBUTES>(), 12);
    assert_eq!(align_of::<HIDD_ATTRIBUTES>(), 4);
}
#[test] #[cfg(target_arch = "x86_64")]
fn hidsdi() {
    assert_eq!(size_of::<HIDD_CONFIGURATION>(), 16);
    // assert_eq!(align_of::<HIDD_CONFIGURATION>(), 4); // FIXME
    assert_eq!(size_of::<HIDD_ATTRIBUTES>(), 12);
    assert_eq!(align_of::<HIDD_ATTRIBUTES>(), 4);
}

// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! USER procedure declarations, constant definitions and macros
pub const PM_NOREMOVE: ::UINT = 0x0000;
pub const PM_REMOVE: ::UINT = 0x0001;
pub const PM_NOYIELD: ::UINT = 0x0002;
pub const PM_QS_INPUT: ::UINT = QS_INPUT << 16;
pub const PM_QS_POSTMESSAGE: ::UINT = (QS_POSTMESSAGE | QS_HOTKEY | QS_TIMER) << 16;
pub const PM_QS_PAINT: ::UINT = QS_PAINT << 16;
pub const PM_QS_SENDMESSAGE: ::UINT = QS_SENDMESSAGE << 16;
pub const QS_KEY: ::DWORD = 0x0001;
pub const QS_MOUSEMOVE: ::DWORD = 0x0002;
pub const QS_MOUSEBUTTON: ::DWORD = 0x0004;
pub const QS_POSTMESSAGE: ::DWORD = 0x0008;
pub const QS_TIMER: ::DWORD = 0x0010;
pub const QS_PAINT: ::DWORD = 0x0020;
pub const QS_SENDMESSAGE: ::DWORD = 0x0040;
pub const QS_HOTKEY: ::DWORD = 0x0080;
pub const QS_ALLPOSTMESSAGE: ::DWORD = 0x0100;
pub const QS_RAWINPUT: ::DWORD = 0x0400;
pub const QS_TOUCH: ::DWORD = 0x0800;
pub const QS_POINTER: ::DWORD = 0x1000;
pub const QS_MOUSE: ::DWORD = QS_MOUSEMOVE | QS_MOUSEBUTTON;
pub const QS_INPUT: ::DWORD = QS_MOUSE | QS_KEY | QS_RAWINPUT | QS_TOUCH | QS_POINTER;
pub const QS_ALLEVENTS: ::DWORD = QS_INPUT | QS_POSTMESSAGE | QS_TIMER | QS_PAINT | QS_HOTKEY;
pub const QS_ALLINPUT: ::DWORD = QS_INPUT | QS_POSTMESSAGE | QS_TIMER
    | QS_PAINT | QS_HOTKEY | QS_SENDMESSAGE;

// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! XInput procedure declarations, constant definitions and macros
pub const XINPUT_DEVTYPE_GAMEPAD: ::BYTE = 0x01;
pub const XINPUT_DEVSUBTYPE_GAMEPAD: ::BYTE = 0x01;
pub const XINPUT_DEVSUBTYPE_WHEEL: ::BYTE = 0x02;
pub const XINPUT_DEVSUBTYPE_ARCADE_STICK: ::BYTE = 0x03;
pub const XINPUT_DEVSUBTYPE_FLIGHT_SICK: ::BYTE = 0x04;
pub const XINPUT_DEVSUBTYPE_DANCE_PAD: ::BYTE = 0x05;
pub const XINPUT_DEVSUBTYPE_GUITAR: ::BYTE = 0x06;
pub const XINPUT_DEVSUBTYPE_DRUM_KIT: ::BYTE = 0x08;
pub const XINPUT_CAPS_VOICE_SUPPORTED: ::WORD = 0x0004;
pub const XINPUT_GAMEPAD_DPAD_UP: ::WORD = 0x0001;
pub const XINPUT_GAMEPAD_DPAD_DOWN: ::WORD = 0x0002;
pub const XINPUT_GAMEPAD_DPAD_LEFT: ::WORD = 0x0004;
pub const XINPUT_GAMEPAD_DPAD_RIGHT: ::WORD = 0x0008;
pub const XINPUT_GAMEPAD_START: ::WORD = 0x0010;
pub const XINPUT_GAMEPAD_BACK: ::WORD = 0x0020;
pub const XINPUT_GAMEPAD_LEFT_THUMB: ::WORD = 0x0040;
pub const XINPUT_GAMEPAD_RIGHT_THUMB: ::WORD = 0x0080;
pub const XINPUT_GAMEPAD_LEFT_SHOULDER: ::WORD = 0x0100;
pub const XINPUT_GAMEPAD_RIGHT_SHOULDER: ::WORD = 0x0200;
pub const XINPUT_GAMEPAD_A: ::WORD = 0x1000;
pub const XINPUT_GAMEPAD_B: ::WORD = 0x2000;
pub const XINPUT_GAMEPAD_X: ::WORD = 0x4000;
pub const XINPUT_GAMEPAD_Y: ::WORD = 0x8000;
pub const XINPUT_GAMEPAD_LEFT_THUMB_DEADZONE: ::SHORT = 7849;
pub const XINPUT_GAMEPAD_RIGHT_THUMB_DEADZONE: ::SHORT = 8689;
pub const XINPUT_GAMEPAD_TRIGGER_THRESHOLD: ::BYTE = 30;
pub const XINPUT_FLAG_GAMEPAD: ::DWORD = 0x00000001;
pub const BATTERY_DEVTYPE_GAMEPAD: ::BYTE = 0x00;
pub const BATTERY_DEVTYPE_HEADSET: ::BYTE = 0x01;
pub const BATTERY_TYPE_DISCONNECTED: ::BYTE = 0x00;
pub const BATTERY_TYPE_WIRED: ::BYTE = 0x01;
pub const BATTERY_TYPE_ALKALINE: ::BYTE = 0x02;
pub const BATTERY_TYPE_NIMH: ::BYTE = 0x03;
pub const BATTERY_TYPE_UNKNOWN: ::BYTE = 0xFF;
pub const BATTERY_LEVEL_EMPTY: ::BYTE = 0x00;
pub const BATTERY_LEVEL_LOW: ::BYTE = 0x01;
pub const BATTERY_LEVEL_MEDIUM: ::BYTE = 0x02;
pub const BATTERY_LEVEL_FULL: ::BYTE = 0x03;
pub const XUSER_MAX_COUNT: ::DWORD = 4;
pub const XUSER_INDEX_ANY: ::DWORD = 0x000000FF;
pub const VK_PAD_A: ::WORD = 0x5800;
pub const VK_PAD_B: ::WORD = 0x5801;
pub const VK_PAD_X: ::WORD = 0x5802;
pub const VK_PAD_Y: ::WORD = 0x5803;
pub const VK_PAD_RSHOULDER: ::WORD = 0x5804;
pub const VK_PAD_LSHOULDER: ::WORD = 0x5805;
pub const VK_PAD_LTRIGGER: ::WORD = 0x5806;
pub const VK_PAD_RTRIGGER: ::WORD = 0x5807;
pub const VK_PAD_DPAD_UP: ::WORD = 0x5810;
pub const VK_PAD_DPAD_DOWN: ::WORD = 0x5811;
pub const VK_PAD_DPAD_LEFT: ::WORD = 0x5812;
pub const VK_PAD_DPAD_RIGHT: ::WORD = 0x5813;
pub const VK_PAD_START: ::WORD = 0x5814;
pub const VK_PAD_BACK: ::WORD = 0x5815;
pub const VK_PAD_LTHUMB_PRESS: ::WORD = 0x5816;
pub const VK_PAD_RTHUMB_PRESS: ::WORD = 0x5817;
pub const VK_PAD_LTHUMB_UP: ::WORD = 0x5820;
pub const VK_PAD_LTHUMB_DOWN: ::WORD = 0x5821;
pub const VK_PAD_LTHUMB_RIGHT: ::WORD = 0x5822;
pub const VK_PAD_LTHUMB_LEFT: ::WORD = 0x5823;
pub const VK_PAD_LTHUMB_UPLEFT: ::WORD = 0x5824;
pub const VK_PAD_LTHUMB_UPRIGHT: ::WORD = 0x5825;
pub const VK_PAD_LTHUMB_DOWNRIGHT: ::WORD = 0x5826;
pub const VK_PAD_LTHUMB_DOWNLEFT: ::WORD = 0x5827;
pub const VK_PAD_RTHUMB_UP: ::WORD = 0x5830;
pub const VK_PAD_RTHUMB_DOWN: ::WORD = 0x5831;
pub const VK_PAD_RTHUMB_RIGHT: ::WORD = 0x5832;
pub const VK_PAD_RTHUMB_LEFT: ::WORD = 0x5833;
pub const VK_PAD_RTHUMB_UPLEFT: ::WORD = 0x5834;
pub const VK_PAD_RTHUMB_UPRIGHT: ::WORD = 0x5835;
pub const VK_PAD_RTHUMB_DOWNRIGHT: ::WORD = 0x5836;
pub const VK_PAD_RTHUMB_DOWNLEFT: ::WORD = 0x5837;
pub const XINPUT_KEYSTROKE_KEYDOWN: ::WORD = 0x0001;
pub const XINPUT_KEYSTROKE_KEYUP: ::WORD = 0x0002;
pub const XINPUT_KEYSTROKE_REPEAT: ::WORD = 0x0004;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct XINPUT_GAMEPAD {
    pub wButtons: ::WORD,
    pub bLeftTrigger: ::BYTE,
    pub bRightTrigger: ::BYTE,
    pub sThumbLX: ::SHORT,
    pub sThumbLY: ::SHORT,
    pub sThumbRX: ::SHORT,
    pub sThumbRY: ::SHORT,
}
pub type PXINPUT_GAMEPAD = *mut XINPUT_GAMEPAD;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct XINPUT_STATE {
    pub dwPacketNumber: ::DWORD,
    pub Gamepad: ::XINPUT_GAMEPAD,
}
pub type PXINPUT_STATE = *mut XINPUT_STATE;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct XINPUT_VIBRATION {
    pub wLeftMotorSpeed: ::WORD,
    pub wRightMotorSpeed: ::WORD,
}
pub type PXINPUT_VIBRATION = *mut XINPUT_VIBRATION;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct XINPUT_CAPABILITIES {
    pub Type: ::BYTE,
    pub SubType: ::BYTE,
    pub Flags: ::WORD,
    pub Gamepad: ::XINPUT_GAMEPAD,
    pub Vibration: ::XINPUT_VIBRATION,
}
pub type PXINPUT_CAPABILITIES = *mut XINPUT_CAPABILITIES;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct XINPUT_BATTERY_INFORMATION {
    pub BatteryType: ::BYTE,
    pub BatteryLevel: ::BYTE,
}
pub type PXINPUT_BATTERY_INFORMATION = *mut XINPUT_BATTERY_INFORMATION;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct XINPUT_KEYSTROKE {
    pub VirtualKey: ::WORD,
    pub Unicode: ::WCHAR,
    pub UserIndex: ::BYTE,
    pub HidCode: ::BYTE,
}
pub type PXINPUT_KEYSTROKE = *mut XINPUT_KEYSTROKE;

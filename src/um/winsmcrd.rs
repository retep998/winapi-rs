// Copyright © 2015-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.

//! Smart Card class/port IOCTL codes.

use shared::minwindef::{BYTE, DWORD, ULONG, WORD};
use um::winioctl::{FILE_ANY_ACCESS, FILE_DEVICE_SMARTCARD, METHOD_BUFFERED};

pub type UWORD = WORD;
DEFINE_GUID!(GUID_DEVINTERFACE_SMARTCARD_READER, 0x50DD5230, 0xBA8A, 0x11D1,
    0xBF, 0x5D, 0x00, 0x00, 0xF8, 0x05, 0xF5, 0x30);
pub const SCARD_ATR_LENGTHL: DWORD = 33;
pub const SCARD_PROTOCOL_UNDEFINED: DWORD = 0x00000000;
pub const SCARD_PROTOCOL_T0: DWORD = 0x00000001;
pub const SCARD_PROTOCOL_T1: DWORD = 0x00000002;
pub const SCARD_PROTOCOL_RAW: DWORD = 0x00010000;
pub const SCARD_PROTOCOL_Tx: DWORD = SCARD_PROTOCOL_T0 | SCARD_PROTOCOL_T1;
pub const SCARD_PROTOCOL_DEFAULT: DWORD = 0x80000000;
pub const SCARD_PROTOCOL_OPTIMAL: DWORD = 0x00000000;
pub const SCARD_POWER_DOWN: DWORD = 0;
pub const SCARD_COLD_RESET: DWORD = 1;
pub const SCARD_WARM_RESET: DWORD = 2;
pub const IOCTL_SMARTCARD_POWER: DWORD = CTL_CODE!(FILE_DEVICE_SMARTCARD, 1, METHOD_BUFFERED,
    FILE_ANY_ACCESS);
pub const IOCTL_SMARTCARD_GET_ATTRIBUTE: DWORD = CTL_CODE!(FILE_DEVICE_SMARTCARD, 2,
    METHOD_BUFFERED, FILE_ANY_ACCESS);
pub const IOCTL_SMARTCARD_SET_ATTRIBUTE: DWORD = CTL_CODE!(FILE_DEVICE_SMARTCARD, 3,
    METHOD_BUFFERED, FILE_ANY_ACCESS);
pub const IOCTL_SMARTCARD_CONFISCATE: DWORD = CTL_CODE!(FILE_DEVICE_SMARTCARD, 4,
    METHOD_BUFFERED, FILE_ANY_ACCESS);
pub const IOCTL_SMARTCARD_TRANSMIT: DWORD = CTL_CODE!(FILE_DEVICE_SMARTCARD, 5,
    METHOD_BUFFERED, FILE_ANY_ACCESS);
pub const IOCTL_SMARTCARD_EJECT: DWORD = CTL_CODE!(FILE_DEVICE_SMARTCARD, 6, METHOD_BUFFERED,
    FILE_ANY_ACCESS);
pub const IOCTL_SMARTCARD_SWALLOW: DWORD = CTL_CODE!(FILE_DEVICE_SMARTCARD, 7,
    METHOD_BUFFERED, FILE_ANY_ACCESS);
pub const IOCTL_SMARTCARD_IS_PRESENT: DWORD = CTL_CODE!(FILE_DEVICE_SMARTCARD, 10,
    METHOD_BUFFERED, FILE_ANY_ACCESS);
pub const IOCTL_SMARTCARD_IS_ABSENT: DWORD = CTL_CODE!(FILE_DEVICE_SMARTCARD, 11,
    METHOD_BUFFERED, FILE_ANY_ACCESS);
pub const IOCTL_SMARTCARD_SET_PROTOCOL: DWORD = CTL_CODE!(FILE_DEVICE_SMARTCARD, 12,
    METHOD_BUFFERED, FILE_ANY_ACCESS);
pub const IOCTL_SMARTCARD_GET_STATE: DWORD = CTL_CODE!(FILE_DEVICE_SMARTCARD, 14,
    METHOD_BUFFERED, FILE_ANY_ACCESS);
pub const IOCTL_SMARTCARD_GET_LAST_ERROR: DWORD = CTL_CODE!(FILE_DEVICE_SMARTCARD, 15,
    METHOD_BUFFERED, FILE_ANY_ACCESS);
pub const IOCTL_SMARTCARD_GET_PERF_CNTR: DWORD = CTL_CODE!(FILE_DEVICE_SMARTCARD, 16,
    METHOD_BUFFERED, FILE_ANY_ACCESS);
pub const MAXIMUM_ATTR_STRING_LENGTH: DWORD = 32;
pub const MAXIMUM_SMARTCARD_READERS: DWORD = 10;
pub const SCARD_CLASS_VENDOR_INFO: ULONG = 1;
pub const SCARD_CLASS_COMMUNICATIONS: ULONG = 2;
pub const SCARD_CLASS_PROTOCOL: ULONG = 3;
pub const SCARD_CLASS_POWER_MGMT: ULONG = 4;
pub const SCARD_CLASS_SECURITY: ULONG = 5;
pub const SCARD_CLASS_MECHANICAL: ULONG = 6;
pub const SCARD_CLASS_VENDOR_DEFINED: ULONG = 7;
pub const SCARD_CLASS_IFD_PROTOCOL: ULONG = 8;
pub const SCARD_CLASS_ICC_STATE: ULONG = 9;
pub const SCARD_CLASS_PERF: ULONG = 0x7ffe;
pub const SCARD_CLASS_SYSTEM: ULONG = 0x7fff;
pub const SCARD_ATTR_VENDOR_NAME: ULONG = SCARD_CLASS_VENDOR_INFO << 16 | 0x0100;
pub const SCARD_ATTR_VENDOR_IFD_TYPE: ULONG = SCARD_CLASS_VENDOR_INFO << 16 | 0x0101;
pub const SCARD_ATTR_VENDOR_IFD_VERSION: ULONG = SCARD_CLASS_VENDOR_INFO << 16 | 0x0102;
pub const SCARD_ATTR_VENDOR_IFD_SERIAL_NO: ULONG = SCARD_CLASS_VENDOR_INFO << 16 | 0x0103;
pub const SCARD_ATTR_CHANNEL_ID: ULONG = SCARD_CLASS_COMMUNICATIONS << 16 | 0x0110;
pub const SCARD_ATTR_PROTOCOL_TYPES: ULONG = SCARD_CLASS_PROTOCOL << 16 | 0x0120;
pub const SCARD_ATTR_DEFAULT_CLK: ULONG = SCARD_CLASS_PROTOCOL << 16 | 0x0121;
pub const SCARD_ATTR_MAX_CLK: ULONG = SCARD_CLASS_PROTOCOL << 16 | 0x0122;
pub const SCARD_ATTR_DEFAULT_DATA_RATE: ULONG = SCARD_CLASS_PROTOCOL << 16 | 0x0123;
pub const SCARD_ATTR_MAX_DATA_RATE: ULONG = SCARD_CLASS_PROTOCOL << 16 | 0x0124;
pub const SCARD_ATTR_MAX_IFSD: ULONG = SCARD_CLASS_PROTOCOL << 16 | 0x0125;
pub const SCARD_ATTR_POWER_MGMT_SUPPORT: ULONG = SCARD_CLASS_POWER_MGMT << 16 | 0x0131;
pub const SCARD_ATTR_USER_TO_CARD_AUTH_DEVICE: ULONG = SCARD_CLASS_SECURITY << 16 | 0x0140;
pub const SCARD_ATTR_USER_AUTH_INPUT_DEVICE: ULONG = SCARD_CLASS_SECURITY << 16 | 0x0142;
pub const SCARD_ATTR_CHARACTERISTICS: ULONG = SCARD_CLASS_MECHANICAL << 16 | 0x0150;
pub const SCARD_ATTR_CURRENT_PROTOCOL_TYPE: ULONG = SCARD_CLASS_IFD_PROTOCOL << 16 | 0x0201;
pub const SCARD_ATTR_CURRENT_CLK: ULONG = SCARD_CLASS_IFD_PROTOCOL << 16 | 0x0202;
pub const SCARD_ATTR_CURRENT_F: ULONG = SCARD_CLASS_IFD_PROTOCOL << 16 | 0x0203;
pub const SCARD_ATTR_CURRENT_D: ULONG = SCARD_CLASS_IFD_PROTOCOL << 16 | 0x0204;
pub const SCARD_ATTR_CURRENT_N: ULONG = SCARD_CLASS_IFD_PROTOCOL << 16 | 0x0205;
pub const SCARD_ATTR_CURRENT_W: ULONG = SCARD_CLASS_IFD_PROTOCOL << 16 | 0x0206;
pub const SCARD_ATTR_CURRENT_IFSC: ULONG = SCARD_CLASS_IFD_PROTOCOL << 16 | 0x0207;
pub const SCARD_ATTR_CURRENT_IFSD: ULONG = SCARD_CLASS_IFD_PROTOCOL << 16 | 0x0208;
pub const SCARD_ATTR_CURRENT_BWT: ULONG = SCARD_CLASS_IFD_PROTOCOL << 16 | 0x0209;
pub const SCARD_ATTR_CURRENT_CWT: ULONG = SCARD_CLASS_IFD_PROTOCOL << 16 | 0x020a;
pub const SCARD_ATTR_CURRENT_EBC_ENCODING: ULONG = SCARD_CLASS_IFD_PROTOCOL << 16 | 0x020b;
pub const SCARD_ATTR_EXTENDED_BWT: ULONG = SCARD_CLASS_IFD_PROTOCOL << 16 | 0x020c;
pub const SCARD_ATTR_ICC_PRESENCE: ULONG = SCARD_CLASS_ICC_STATE << 16 | 0x0300;
pub const SCARD_ATTR_ICC_INTERFACE_STATUS: ULONG = SCARD_CLASS_ICC_STATE << 16 | 0x0301;
pub const SCARD_ATTR_CURRENT_IO_STATE: ULONG = SCARD_CLASS_ICC_STATE << 16 | 0x0302;
pub const SCARD_ATTR_ATR_STRING: ULONG = SCARD_CLASS_ICC_STATE << 16 | 0x0303;
pub const SCARD_ATTR_ICC_TYPE_PER_ATR: ULONG = SCARD_CLASS_ICC_STATE << 16 | 0x0304;
pub const SCARD_ATTR_ESC_RESET: ULONG = SCARD_CLASS_VENDOR_DEFINED << 16 | 0xA000;
pub const SCARD_ATTR_ESC_CANCEL: ULONG = SCARD_CLASS_VENDOR_DEFINED << 16 | 0xA003;
pub const SCARD_ATTR_ESC_AUTHREQUEST: ULONG = SCARD_CLASS_VENDOR_DEFINED << 16 | 0xA005;
pub const SCARD_ATTR_MAXINPUT: ULONG = SCARD_CLASS_VENDOR_DEFINED << 16 | 0xA007;
pub const SCARD_ATTR_DEVICE_UNIT: ULONG = SCARD_CLASS_SYSTEM << 16 | 0x0001;
pub const SCARD_ATTR_DEVICE_IN_USE: ULONG = SCARD_CLASS_SYSTEM << 16 | 0x0002;
pub const SCARD_ATTR_DEVICE_FRIENDLY_NAME_A: ULONG = SCARD_CLASS_SYSTEM << 16 | 0x0003;
pub const SCARD_ATTR_DEVICE_SYSTEM_NAME_A: ULONG = SCARD_CLASS_SYSTEM << 16 | 0x0004;
pub const SCARD_ATTR_DEVICE_FRIENDLY_NAME_W: ULONG = SCARD_CLASS_SYSTEM << 16 | 0x0005;
pub const SCARD_ATTR_DEVICE_SYSTEM_NAME_W: ULONG = SCARD_CLASS_SYSTEM << 16 | 0x0006;
pub const SCARD_ATTR_SUPRESS_T1_IFS_REQUEST: ULONG = SCARD_CLASS_SYSTEM << 16 | 0x0007;
pub const SCARD_PERF_NUM_TRANSMISSIONS: ULONG = SCARD_CLASS_PERF << 16 | 0x0001;
pub const SCARD_PERF_BYTES_TRANSMITTED: ULONG = SCARD_CLASS_PERF << 16 | 0x0002;
pub const SCARD_PERF_TRANSMISSION_TIME: ULONG = SCARD_CLASS_PERF << 16 | 0x0003;
pub const SCARD_T0_HEADER_LENGTH: DWORD = 7;
pub const SCARD_T0_CMD_LENGTH: DWORD = 5;
pub const SCARD_T1_PROLOGUE_LENGTH: DWORD = 3;
pub const SCARD_T1_EPILOGUE_LENGTH: DWORD = 2;
pub const SCARD_T1_MAX_IFS: DWORD = 254;
pub const SCARD_UNKNOWN: ULONG = 0;
pub const SCARD_ABSENT: ULONG = 1;
pub const SCARD_PRESENT: ULONG = 2;
pub const SCARD_SWALLOWED: ULONG = 3;
pub const SCARD_POWERED: ULONG = 4;
pub const SCARD_NEGOTIABLE: ULONG = 5;
pub const SCARD_SPECIFIC: ULONG = 6;
STRUCT!{struct SCARD_IO_REQUEST {
    dwProtocol: DWORD,
    cbPciLength: DWORD,
}}
pub type PSCARD_IO_REQUEST = *mut SCARD_IO_REQUEST;
pub type LPSCARD_IO_REQUEST = *mut SCARD_IO_REQUEST;
pub type LPCSCARD_IO_REQUEST = *const SCARD_IO_REQUEST;
STRUCT!{struct SCARD_T0_COMMAND {
    bCla: BYTE,
    bIns: BYTE,
    bP1: BYTE,
    bP2: BYTE,
    bP3: BYTE,
}}
pub type LPSCARD_T0_COMMAND = *mut SCARD_T0_COMMAND;
STRUCT!{struct SCARD_T0_REQUEST {
    ioRequest: SCARD_IO_REQUEST,
    bSw1: BYTE,
    bSw2: BYTE,
    CmdBytes: SCARD_T0_COMMAND,
}}
UNION!(SCARD_T0_REQUEST, CmdBytes, rgbHeader, rgbHeader_mut, [BYTE; 5]);
pub type PSCARD_T0_REQUEST = *mut SCARD_T0_REQUEST;
pub type LPSCARD_T0_REQUEST = *mut SCARD_T0_REQUEST;
STRUCT!{struct SCARD_T1_REQUEST {
    ioRequest: SCARD_IO_REQUEST,
}}
pub type PSCARD_T1_REQUEST = *mut SCARD_T1_REQUEST;
pub type LPSCARD_T1_REQUEST = *mut SCARD_T1_REQUEST;
pub const SCARD_READER_SWALLOWS: ULONG = 0x00000001;
pub const SCARD_READER_EJECTS: ULONG = 0x00000002;
pub const SCARD_READER_CONFISCATES: ULONG = 0x00000004;
pub const SCARD_READER_TYPE_SERIAL: ULONG = 0x01;
pub const SCARD_READER_TYPE_PARALELL: ULONG = 0x02;
pub const SCARD_READER_TYPE_KEYBOARD: ULONG = 0x04;
pub const SCARD_READER_TYPE_SCSI: ULONG = 0x08;
pub const SCARD_READER_TYPE_IDE: ULONG = 0x10;
pub const SCARD_READER_TYPE_USB: ULONG = 0x20;
pub const SCARD_READER_TYPE_PCMCIA: ULONG = 0x40;
pub const SCARD_READER_TYPE_TPM: ULONG = 0x80;
pub const SCARD_READER_TYPE_NFC: ULONG = 0x100;
pub const SCARD_READER_TYPE_UICC: ULONG = 0x200;
pub const SCARD_READER_TYPE_VENDOR: ULONG = 0xF0;

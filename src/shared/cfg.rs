// Copyright © 2015, skdltmxn
// Licensed under the MIT License <LICENSE.md>
//! common Configuration Manager definitions for both user mode and kernel mode code
use um::cfgmgr32::{CONFIGRET, PRIORITY};

ENUM!{enum PNP_VETO_TYPE {
    PNP_VetoTypeUnknown,
    PNP_VetoLegacyDevice,
    PNP_VetoPendingClose,
    PNP_VetoWindowsApp,
    PNP_VetoWindowsService,
    PNP_VetoOutstandingOpen,
    PNP_VetoDevice,
    PNP_VetoDriver,
    PNP_VetoIllegalDeviceRequest,
    PNP_VetoInsufficientPower,
    PNP_VetoNonDisableable,
    PNP_VetoLegacyDriver,
    PNP_VetoInsufficientRights,
}}
pub type PPNP_VETO_TYPE = *mut PNP_VETO_TYPE;
pub const CM_PROB_NOT_CONFIGURED: CONFIGRET = 0x00000001;
pub const CM_PROB_DEVLOADER_FAILED: CONFIGRET = 0x00000002;
pub const CM_PROB_OUT_OF_MEMORY: CONFIGRET = 0x00000003;
pub const CM_PROB_ENTRY_IS_WRONG_TYPE: CONFIGRET = 0x00000004;
pub const CM_PROB_LACKED_ARBITRATOR: CONFIGRET = 0x00000005;
pub const CM_PROB_BOOT_CONFIG_CONFLICT: CONFIGRET = 0x00000006;
pub const CM_PROB_FAILED_FILTER: CONFIGRET = 0x00000007;
pub const CM_PROB_DEVLOADER_NOT_FOUND: CONFIGRET = 0x00000008;
pub const CM_PROB_INVALID_DATA: CONFIGRET = 0x00000009;
pub const CM_PROB_FAILED_START: CONFIGRET = 0x0000000A;
pub const CM_PROB_LIAR: CONFIGRET = 0x0000000B;
pub const CM_PROB_NORMAL_CONFLICT: CONFIGRET = 0x0000000C;
pub const CM_PROB_NOT_VERIFIED: CONFIGRET = 0x0000000D;
pub const CM_PROB_NEED_RESTART: CONFIGRET = 0x0000000E;
pub const CM_PROB_REENUMERATION: CONFIGRET = 0x0000000F;
pub const CM_PROB_PARTIAL_LOG_CONF: CONFIGRET = 0x00000010;
pub const CM_PROB_UNKNOWN_RESOURCE: CONFIGRET = 0x00000011;
pub const CM_PROB_REINSTALL: CONFIGRET = 0x00000012;
pub const CM_PROB_REGISTRY: CONFIGRET = 0x00000013;
pub const CM_PROB_VXDLDR: CONFIGRET = 0x00000014;
pub const CM_PROB_WILL_BE_REMOVED: CONFIGRET = 0x00000015;
pub const CM_PROB_DISABLED: CONFIGRET = 0x00000016;
pub const CM_PROB_DEVLOADER_NOT_READY: CONFIGRET = 0x00000017;
pub const CM_PROB_DEVICE_NOT_THERE: CONFIGRET = 0x00000018;
pub const CM_PROB_MOVED: CONFIGRET = 0x00000019;
pub const CM_PROB_TOO_EARLY: CONFIGRET = 0x0000001A;
pub const CM_PROB_NO_VALID_LOG_CONF: CONFIGRET = 0x0000001B;
pub const CM_PROB_FAILED_INSTALL: CONFIGRET = 0x0000001C;
pub const CM_PROB_HARDWARE_DISABLED: CONFIGRET = 0x0000001D;
pub const CM_PROB_CANT_SHARE_IRQ: CONFIGRET = 0x0000001E;
pub const CM_PROB_FAILED_ADD: CONFIGRET = 0x0000001F;
pub const CM_PROB_DISABLED_SERVICE: CONFIGRET = 0x00000020;
pub const CM_PROB_TRANSLATION_FAILED: CONFIGRET = 0x00000021;
pub const CM_PROB_NO_SOFTCONFIG: CONFIGRET = 0x00000022;
pub const CM_PROB_BIOS_TABLE: CONFIGRET = 0x00000023;
pub const CM_PROB_IRQ_TRANSLATION_FAILED: CONFIGRET = 0x00000024;
pub const CM_PROB_FAILED_DRIVER_ENTRY: CONFIGRET = 0x00000025;
pub const CM_PROB_DRIVER_FAILED_PRIOR_UNLOAD: CONFIGRET = 0x00000026;
pub const CM_PROB_DRIVER_FAILED_LOAD: CONFIGRET = 0x00000027;
pub const CM_PROB_DRIVER_SERVICE_KEY_INVALID: CONFIGRET = 0x00000028;
pub const CM_PROB_LEGACY_SERVICE_NO_DEVICES: CONFIGRET = 0x00000029;
pub const CM_PROB_DUPLICATE_DEVICE: CONFIGRET = 0x0000002A;
pub const CM_PROB_FAILED_POST_START: CONFIGRET = 0x0000002B;
pub const CM_PROB_HALTED: CONFIGRET = 0x0000002C;
pub const CM_PROB_PHANTOM: CONFIGRET = 0x0000002D;
pub const CM_PROB_SYSTEM_SHUTDOWN: CONFIGRET = 0x0000002E;
pub const CM_PROB_HELD_FOR_EJECT: CONFIGRET = 0x0000002F;
pub const CM_PROB_DRIVER_BLOCKED: CONFIGRET = 0x00000030;
pub const CM_PROB_REGISTRY_TOO_LARGE: CONFIGRET = 0x00000031;
pub const CM_PROB_SETPROPERTIES_FAILED: CONFIGRET = 0x00000032;
pub const CM_PROB_WAITING_ON_DEPENDENCY: CONFIGRET = 0x00000033;
pub const CM_PROB_UNSIGNED_DRIVER: CONFIGRET = 0x00000034;
pub const CM_PROB_USED_BY_DEBUGGER: CONFIGRET = 0x00000035;
pub const NUM_CM_PROB_V1: CONFIGRET = 0x00000025;
pub const NUM_CM_PROB_V2: CONFIGRET = 0x00000032;
pub const NUM_CM_PROB_V3: CONFIGRET = 0x00000033;
pub const NUM_CM_PROB_V4: CONFIGRET = 0x00000034;
pub const NUM_CM_PROB_V5: CONFIGRET = 0x00000035;
pub const NUM_CM_PROB_V6: CONFIGRET = 0x00000036;
pub const DN_ROOT_ENUMERATED: CONFIGRET = 0x00000001;
pub const DN_DRIVER_LOADED: CONFIGRET = 0x00000002;
pub const DN_ENUM_LOADED: CONFIGRET = 0x00000004;
pub const DN_STARTED: CONFIGRET = 0x00000008;
pub const DN_MANUAL: CONFIGRET = 0x00000010;
pub const DN_NEED_TO_ENUM: CONFIGRET = 0x00000020;
pub const DN_NOT_FIRST_TIME: CONFIGRET = 0x00000040;
pub const DN_HARDWARE_ENUM: CONFIGRET = 0x00000080;
pub const DN_LIAR: CONFIGRET = 0x00000100;
pub const DN_HAS_MARK: CONFIGRET = 0x00000200;
pub const DN_HAS_PROBLEM: CONFIGRET = 0x00000400;
pub const DN_FILTERED: CONFIGRET = 0x00000800;
pub const DN_MOVED: CONFIGRET = 0x00001000;
pub const DN_DISABLEABLE: CONFIGRET = 0x00002000;
pub const DN_REMOVABLE: CONFIGRET = 0x00004000;
pub const DN_PRIVATE_PROBLEM: CONFIGRET = 0x00008000;
pub const DN_MF_PARENT: CONFIGRET = 0x00010000;
pub const DN_MF_CHILD: CONFIGRET = 0x00020000;
pub const DN_WILL_BE_REMOVED: CONFIGRET = 0x00040000;
pub const DN_NOT_FIRST_TIMEE: CONFIGRET = 0x00080000;
pub const DN_STOP_FREE_RES: CONFIGRET = 0x00100000;
pub const DN_REBAL_CANDIDATE: CONFIGRET = 0x00200000;
pub const DN_BAD_PARTIAL: CONFIGRET = 0x00400000;
pub const DN_NT_ENUMERATOR: CONFIGRET = 0x00800000;
pub const DN_NT_DRIVER: CONFIGRET = 0x01000000;
pub const DN_NEEDS_LOCKING: CONFIGRET = 0x02000000;
pub const DN_ARM_WAKEUP: CONFIGRET = 0x04000000;
pub const DN_APM_ENUMERATOR: CONFIGRET = 0x08000000;
pub const DN_APM_DRIVER: CONFIGRET = 0x10000000;
pub const DN_SILENT_INSTALL: CONFIGRET = 0x20000000;
pub const DN_NO_SHOW_IN_DM: CONFIGRET = 0x40000000;
pub const DN_BOOT_LOG_PROB: CONFIGRET = 0x80000000;
pub const DN_NEED_RESTART: CONFIGRET = DN_LIAR;
pub const DN_DRIVER_BLOCKED: CONFIGRET = DN_NOT_FIRST_TIME;
pub const DN_LEGACY_DRIVER: CONFIGRET = DN_MOVED;
pub const DN_CHILD_WITH_INVALID_ID: CONFIGRET = DN_HAS_MARK;
pub const DN_DEVICE_DISCONNECTED: CONFIGRET = DN_NEEDS_LOCKING;
pub const DN_CHANGEABLE_FLAGS: CONFIGRET = DN_NOT_FIRST_TIME + DN_HARDWARE_ENUM + DN_HAS_MARK
    + DN_DISABLEABLE + DN_REMOVABLE + DN_MF_CHILD + DN_MF_PARENT + DN_NOT_FIRST_TIMEE
    + DN_STOP_FREE_RES + DN_REBAL_CANDIDATE + DN_NT_ENUMERATOR + DN_NT_DRIVER + DN_SILENT_INSTALL
    + DN_NO_SHOW_IN_DM;
pub const LCPRI_FORCECONFIG: PRIORITY = 0x00000000;
pub const LCPRI_BOOTCONFIG: PRIORITY = 0x00000001;
pub const LCPRI_DESIRED: PRIORITY = 0x00002000;
pub const LCPRI_NORMAL: PRIORITY = 0x00003000;
pub const LCPRI_LASTBESTCONFIG: PRIORITY = 0x00003FFF;
pub const LCPRI_SUBOPTIMAL: PRIORITY = 0x00005000;
pub const LCPRI_LASTSOFTCONFIG: PRIORITY = 0x00007FFF;
pub const LCPRI_RESTART: PRIORITY = 0x00008000;
pub const LCPRI_REBOOT: PRIORITY = 0x00009000;
pub const LCPRI_POWEROFF: PRIORITY = 0x0000A000;
pub const LCPRI_HARDRECONFIG: PRIORITY = 0x0000C000;
pub const LCPRI_HARDWIRED: PRIORITY = 0x0000E000;
pub const LCPRI_IMPOSSIBLE: PRIORITY = 0x0000F000;
pub const LCPRI_DISABLED: PRIORITY = 0x0000FFFF;
pub const MAX_LCPRI: PRIORITY = 0x0000FFFF;

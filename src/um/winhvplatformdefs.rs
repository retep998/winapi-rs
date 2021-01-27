use shared::basetsd::{UINT8, UINT16, UINT32, UINT64};
use shared::minwindef::{
    BOOL, LPVOID
};
ENUM!{enum WHV_CAPABILITY_CODE {
    WHvCapabilityCodeHypervisorPresent = 0x00000000,
    WHvCapabilityCodeFeatures = 0x00000001,
    WHvCapabilityCodeExtendedVmExits = 0x00000002,
    WHvCapabilityCodeExceptionExitBitmap = 0x00000003,
    WHvCapabilityCodeProcessorVendor = 0x00001000,
    WHvCapabilityCodeProcessorFeatures = 0x00001001,
    WHvCapabilityCodeProcessorClFlushSize = 0x00001002,
    WHvCapabilityCodeProcessorXsaveFeatures = 0x00001003,
}}
STRUCT!{struct WHV_CAPABILITY_FEATURES {
    AsUINT64: UINT64,
}}
BITFIELD!{WHV_CAPABILITY_FEATURES AsUINT64: UINT64 [
    PartialUnmap set_PartialUnmap [0..1],
    LocalApicEmulation set_LocalApicEmulation [1..2],
    Xsave set_Xsave [2..3],
    DirtyPageTracking set_DirtyPageTracking [3..4],
    SpeculationControl set_SpeculationControl [4..5],
    Reserved set_Reserved [5..64],
]}
STRUCT!{struct WHV_EXTENDED_VM_EXITS {
    AsUINT64: UINT64,
}}
BITFIELD!{WHV_EXTENDED_VM_EXITS AsUINT64: UINT64 [
    X64CpuidExit set_X64CpuidExit [0..1],
    X64MsrExit set_X64MsrExit [1..2],
    ExceptionExit set_ExceptionExit [2..3],
    Reserved set_Reserved [3..64],
]}
ENUM!{enum WHV_PROCESSOR_VENDOR {
    WHvProcessorVendorAmd = 0x0000,
    WHvProcessorVendorIntel = 0x0001,
    WHvProcessorVendorHygon = 0x0002,
}}
STRUCT!{struct WHV_PROCESSOR_FEATURES {
    AsUINT64: UINT64,
}}
BITFIELD!{WHV_PROCESSOR_FEATURES AsUINT64: UINT64 [
    Sse3Support set_Sse3Support [0..1],
    LahfSahfSupport set_LahfSahfSupport [1..2],
    Ssse3Support set_Ssse3Support [2..3],
    Sse4_1Support set_Sse4_1Support [3..4],
    Sse4_2Support set_Sse4_2Support [4..5],
    Sse4aSupport set_Sse4aSupport [5..6],
    XopSupport set_XopSupport [6..7],
    PopCntSupport set_PopCntSupport [7..8],
    Cmpxchg16bSupport set_Cmpxchg16bSupport [8..9],
    Altmovcr8Support set_Altmovcr8Support [9..10],
    LzcntSupport set_LzcntSupport [10..11],
    MisAlignSseSupport set_MisAlignSseSupport [11..12],
    MmxExtSupport set_MmxExtSupport [12..13],
    Amd3DNowSupport set_Amd3DNowSupport [13..14],
    ExtendedAmd3DNowSupport set_ExtendedAmd3DNowSupport [14..15],
    Page1GbSupport set_Page1GbSupport [15..16],
    AesSupport set_AesSupport [16..17],
    PclmulqdqSupport set_PclmulqdqSupport [17..18],
    PcidSupport set_PcidSupport [18..19],
    Fma4Support set_Fma4Support [19..20],
    F16CSupport set_F16CSupport [20..21],
    RdRandSupport set_RdRandSupport [21..22],
    RdWrFsGsSupport set_RdWrFsGsSupport [22..23],
    SmepSupport set_SmepSupport [23..24],
    EnhancedFastStringSupport set_EnhancedFastStringSupport [24..25],
    Bmi1Support set_Bmi1Support [25..26],
    Bmi2Support set_Bmi2Support [26..27],
    Reserved1 set_Reserved1 [27..29], //
    MovbeSupport set_MovbeSupport [29..30],
    Npiep1Support set_Npiep1Support [30..31],
    DepX87FPUSaveSupport set_DepX87FPUSaveSupport [31..32],
    RdSeedSupport set_RdSeedSupport [32..33],
    AdxSupport set_AdxSupport [33..34],
    IntelPrefetchSupport set_IntelPrefetchSupport [34..35],
    SmapSupport set_SmapSupport [35..36],
    HleSupport set_HleSupport [36..37],
    RtmSupport set_RtmSupport [37..38],
    RdtscpSupport set_RdtscpSupport [38..39],
    ClflushoptSupport set_ClflushoptSupport [39..40],
    ClwbSupport set_ClwbSupport [40..41],
    ShaSupport set_ShaSupport [41..42],
    X87PointersSavedSupport set_X87PointersSavedSupport [42..43],
    InvpcidSupport set_InvpcidSupport [43..44],
    IbrsSupport set_IbrsSupport [44..45],
    StibpSupport set_StibpSupport [45..46],
    IbpbSupport set_IbpbSupport [46..47],
    Reserved2 set_Reserved2 [47..48],
    SsbdSupport set_SsbdSupport [48..49],
    FastShortRepMovSupport set_FastShortRepMovSupport [49..50],
    Reserved3 set_Reserved3 [50..51],
    RdclNo set_RdclNo [51..52],
    IbrsAllSupport set_IbrsAllSupport [52..53],
    Reserved4 set_Reserved4 [53..54],
    SsbNo set_SsbNo [54..55],
    RsbANo set_RsbANo [55..56],
    Reserved5 set_Reserved5 [56..64],
]}
STRUCT!{struct WHV_PROCESSOR_XSAVE_FEATURES {
    AsUINT64: UINT64,
}}
BITFIELD!{WHV_PROCESSOR_FEATURES AsUINT64: UINT64 [
    XsaveSupport set_XsaveSupport [0..1],
    XsaveoptSupport set_XsaveoptSupport [1..2],
    AvxSupport set_AvxSupport [2..3],
    Avx2Support set_Avx2Support [3..4],
    FmaSupport set_FmaSupport [4..5],
    MpxSupport set_MpxSupport [5..6],
    Avx512Support set_Avx512Support [6..7],
    Avx512DQSupport set_Avx512DQSupport [7..8],
    Avx512CDSupport set_Avx512CDSupport [8..9],
    Avx512BWSupport set_Avx512BWSupport [9..10],
    Avx512VLSupport set_Avx512VLSupport [10..11],
    XsaveCompSupport set_XsaveCompSupport [11..12],
    XsaveSupervisorSupport set_XsaveSupervisorSupport [12..13],
    Xcr1Support set_Xcr1Support [13..14],
    Avx512BitalgSupport set_Avx512BitalgSupport [14..15],
    Avx512IfmaSupport set_Avx512IfmaSupport [15..16],
    Avx512VBmiSupport set_Avx512VBmiSupport [16..17],
    Avx512VBmi2Support set_Avx512VBmi2Support [17..18],
    Avx512VnniSupport set_Avx512VnniSupport [18..19],
    GfniSupport set_GfniSupport [19..20],
    VaesSupport set_VaesSupport [20..21],
    Avx512VPopcntdqSupport set_Avx512VPopcntdqSupport [21..22],
    VpclmulqdqSupport set_VpclmulqdqSupport [22..23],
    Reserved set_Reserved [23..64],
]}
pub type PWHV_PROCESSOR_XSAVE_FEATURES = *mut WHV_PROCESSOR_XSAVE_FEATURES;
UNION!{union WHV_CAPABILITY
{
    [u64; 1],
    HypervisorPresent HypervisorPresent_mut: BOOL,
    Features Features_mut: WHV_CAPABILITY_FEATURES,
    ExtendedVmExits ExtendedVmExits_mut: WHV_EXTENDED_VM_EXITS,
    ProcessorVendor ProcessorVendor_mut: WHV_PROCESSOR_VENDOR,
    ProcessorFeatures ProcessorFeatures_mut: WHV_PROCESSOR_FEATURES,
    ProcessorXsaveFeatures ProcessorXsaveFeatures_mut: WHV_PROCESSOR_XSAVE_FEATURES,
    ProcessorClFlushSize ProcessorClFlushSize_mut: UINT8,
    ExceptionExitBitmap ExceptionExitBitmap_mut: UINT64,
}}
pub type WHV_PARTITION_HANDLE = LPVOID;
ENUM!{enum WHV_PARTITION_PROPERTY_CODE
{
    WHvPartitionPropertyCodeExtendedVmExits         = 0x00000001,
    WHvPartitionPropertyCodeExceptionExitBitmap     = 0x00000002,
    WHvPartitionPropertyCodeSeparateSecurityDomain  = 0x00000003,

    WHvPartitionPropertyCodeProcessorFeatures       = 0x00001001,
    WHvPartitionPropertyCodeProcessorClFlushSize    = 0x00001002,
    WHvPartitionPropertyCodeCpuidExitList           = 0x00001003,
    WHvPartitionPropertyCodeCpuidResultList         = 0x00001004,
    WHvPartitionPropertyCodeLocalApicEmulationMode  = 0x00001005,
    WHvPartitionPropertyCodeProcessorXsaveFeatures  = 0x00001006,

    WHvPartitionPropertyCodeProcessorCount          = 0x00001fff,
}}
STRUCT!{struct WHV_X64_CPUID_RESULT
{
    Function: UINT32,
    Reserved: [UINT32; 3],
    Eax: UINT32,
    Ebx: UINT32,
    Ecx: UINT32,
    Edx: UINT32,    
}}
ENUM!{enum WHV_EXCEPTION_TYPE
{
    WHvX64ExceptionTypeDivideErrorFault = 0x0,
    WHvX64ExceptionTypeDebugTrapOrFault = 0x1,
    WHvX64ExceptionTypeBreakpointTrap = 0x3,
    WHvX64ExceptionTypeOverflowTrap = 0x4,
    WHvX64ExceptionTypeBoundRangeFault = 0x5,
    WHvX64ExceptionTypeInvalidOpcodeFault = 0x6,
    WHvX64ExceptionTypeDeviceNotAvailableFault = 0x7,
    WHvX64ExceptionTypeDoubleFaultAbort = 0x8,
    WHvX64ExceptionTypeInvalidTaskStateSegmentFault = 0x0A,
    WHvX64ExceptionTypeSegmentNotPresentFault = 0x0B,
    WHvX64ExceptionTypeStackFault = 0x0C,
    WHvX64ExceptionTypeGeneralProtectionFault = 0x0D,
    WHvX64ExceptionTypePageFault = 0x0E,
    WHvX64ExceptionTypeFloatingPointErrorFault = 0x10,
    WHvX64ExceptionTypeAlignmentCheckFault = 0x11,
    WHvX64ExceptionTypeMachineCheckAbort = 0x12,
    WHvX64ExceptionTypeSimdFloatingPointFault = 0x13,
}}
ENUM!{enum WHV_X64_LOCAL_APIC_EMULATION_MODE
{
    WHvX64LocalApicEmulationModeNone,
    WHvX64LocalApicEmulationModeXApic,
}}
UNION!{union WHV_PARTITION_PROPERTY
{
    [u64; 4],
    ExtendedVmExits ExtendedVmExits_mut: WHV_EXTENDED_VM_EXITS,
    ProcessorFeatures ProcessorFeatures_mut: WHV_PROCESSOR_FEATURES,
    ProcessorXsaveFeatures ProcessorXsaveFeatures_mut: WHV_PROCESSOR_XSAVE_FEATURES,
    ProcessorClFlushSize ProcessorClFlushSize_mut: UINT8,
    ProcessorCount ProcessorCount_mut: UINT32,
    CpuidExitList CpuidExitList_mut: [UINT32; 1],
    CpuidResultList CpuidResultList_mut: [WHV_X64_CPUID_RESULT; 1],
    LocalApicEmulationMode LocalApicEmulationMode_mut: WHV_X64_LOCAL_APIC_EMULATION_MODE,
    SeparateSecurityDomain SeparateSecurityDomain_mut: BOOL,
}}
pub type WHV_GUEST_PHYSICAL_ADDRESS = UINT64;
pub type WHV_GUEST_VIRTUAL_ADDRESS = UINT64;
ENUM!{enum WHV_MAP_GPA_RANGE_FLAGS
{
        WHvMapGpaRangeFlagNone              = 0x00000000,
        WHvMapGpaRangeFlagRead              = 0x00000001,
        WHvMapGpaRangeFlagWrite             = 0x00000002,
        WHvMapGpaRangeFlagExecute           = 0x00000004,
        WHvMapGpaRangeFlagTrackDirtyPages   = 0x00000008,
}}
ENUM!{enum WHV_TRANSLATE_GVA_FLAGS {
    WHvTranslateGvaFlagNone = 0x00000000,
    WHvTranslateGvaFlagValidateRead = 0x00000001,
    WHvTranslateGvaFlagValidateWrite = 0x00000002,
    WHvTranslateGvaFlagValidateExecute = 0x00000004,
    WHvTranslateGvaFlagPrivilegeExempt = 0x00000008,
    WHvTranslateGvaFlagSetPageTableBits = 0x00000010,
}}
ENUM!{enum WHV_TRANSLATE_GVA_RESULT_CODE {
    WHvTranslateGvaResultSuccess = 0,
    WHvTranslateGvaResultPageNotPresent = 1,
    WHvTranslateGvaResultPrivilegeViolation = 2,
    WHvTranslateGvaResultInvalidPageTableFlags = 3,
    WHvTranslateGvaResultGpaUnmapped = 4,
    WHvTranslateGvaResultGpaNoReadAccess = 5,
    WHvTranslateGvaResultGpaNoWriteAccess = 6,
    WHvTranslateGvaResultGpaIllegalOverlayAccess = 7,
    WHvTranslateGvaResultIntercept = 8,
}}
STRUCT!{struct WHV_TRANSLATE_GVA_RESULT {
    ResultCode: WHV_TRANSLATE_GVA_RESULT_CODE,
    Reserved: UINT32,    
}}
ENUM!{enum WHV_REGISTER_NAME {
    WHvX64RegisterRax = 0x00000000,
    WHvX64RegisterRcx = 0x00000001,
    WHvX64RegisterRdx = 0x00000002,
    WHvX64RegisterRbx = 0x00000003,
    WHvX64RegisterRsp = 0x00000004,
    WHvX64RegisterRbp = 0x00000005,
    WHvX64RegisterRsi = 0x00000006,
    WHvX64RegisterRdi = 0x00000007,
    WHvX64RegisterR8 = 0x00000008,
    WHvX64RegisterR9 = 0x00000009,
    WHvX64RegisterR10 = 0x0000000A,
    WHvX64RegisterR11 = 0x0000000B,
    WHvX64RegisterR12 = 0x0000000C,
    WHvX64RegisterR13 = 0x0000000D,
    WHvX64RegisterR14 = 0x0000000E,
    WHvX64RegisterR15 = 0x0000000F,
    WHvX64RegisterRip = 0x00000010,
    WHvX64RegisterRflags = 0x00000011,
    WHvX64RegisterEs = 0x00000012,
    WHvX64RegisterCs = 0x00000013,
    WHvX64RegisterSs = 0x00000014,
    WHvX64RegisterDs = 0x00000015,
    WHvX64RegisterFs = 0x00000016,
    WHvX64RegisterGs = 0x00000017,
    WHvX64RegisterLdtr = 0x00000018,
    WHvX64RegisterTr = 0x00000019,
    WHvX64RegisterIdtr = 0x0000001A,
    WHvX64RegisterGdtr = 0x0000001B,
    WHvX64RegisterCr0 = 0x0000001C,
    WHvX64RegisterCr2 = 0x0000001D,
    WHvX64RegisterCr3 = 0x0000001E,
    WHvX64RegisterCr4 = 0x0000001F,
    WHvX64RegisterCr8 = 0x00000020,
    WHvX64RegisterDr0 = 0x00000021,
    WHvX64RegisterDr1 = 0x00000022,
    WHvX64RegisterDr2 = 0x00000023,
    WHvX64RegisterDr3 = 0x00000024,
    WHvX64RegisterDr6 = 0x00000025,
    WHvX64RegisterDr7 = 0x00000026,
    WHvX64RegisterXCr0 = 0x00000027,
    WHvX64RegisterXmm0 = 0x00001000,
    WHvX64RegisterXmm1 = 0x00001001,
    WHvX64RegisterXmm2 = 0x00001002,
    WHvX64RegisterXmm3 = 0x00001003,
    WHvX64RegisterXmm4 = 0x00001004,
    WHvX64RegisterXmm5 = 0x00001005,
    WHvX64RegisterXmm6 = 0x00001006,
    WHvX64RegisterXmm7 = 0x00001007,
    WHvX64RegisterXmm8 = 0x00001008,
    WHvX64RegisterXmm9 = 0x00001009,
    WHvX64RegisterXmm10 = 0x0000100A,
    WHvX64RegisterXmm11 = 0x0000100B,
    WHvX64RegisterXmm12 = 0x0000100C,
    WHvX64RegisterXmm13 = 0x0000100D,
    WHvX64RegisterXmm14 = 0x0000100E,
    WHvX64RegisterXmm15 = 0x0000100F,
    WHvX64RegisterFpMmx0 = 0x00001010,
    WHvX64RegisterFpMmx1 = 0x00001011,
    WHvX64RegisterFpMmx2 = 0x00001012,
    WHvX64RegisterFpMmx3 = 0x00001013,
    WHvX64RegisterFpMmx4 = 0x00001014,
    WHvX64RegisterFpMmx5 = 0x00001015,
    WHvX64RegisterFpMmx6 = 0x00001016,
    WHvX64RegisterFpMmx7 = 0x00001017,
    WHvX64RegisterFpControlStatus = 0x00001018,
    WHvX64RegisterXmmControlStatus = 0x00001019,
    WHvX64RegisterTsc = 0x00002000,
    WHvX64RegisterEfer = 0x00002001,
    WHvX64RegisterKernelGsBase = 0x00002002,
    WHvX64RegisterApicBase = 0x00002003,
    WHvX64RegisterPat = 0x00002004,
    WHvX64RegisterSysenterCs = 0x00002005,
    WHvX64RegisterSysenterEip = 0x00002006,
    WHvX64RegisterSysenterEsp = 0x00002007,
    WHvX64RegisterStar = 0x00002008,
    WHvX64RegisterLstar = 0x00002009,
    WHvX64RegisterCstar = 0x0000200A,
    WHvX64RegisterSfmask = 0x0000200B,
    WHvX64RegisterMsrMtrrCap = 0x0000200D,
    WHvX64RegisterMsrMtrrDefType = 0x0000200E,
    WHvX64RegisterMsrMtrrPhysBase0 = 0x00002010,
    WHvX64RegisterMsrMtrrPhysBase1 = 0x00002011,
    WHvX64RegisterMsrMtrrPhysBase2 = 0x00002012,
    WHvX64RegisterMsrMtrrPhysBase3 = 0x00002013,
    WHvX64RegisterMsrMtrrPhysBase4 = 0x00002014,
    WHvX64RegisterMsrMtrrPhysBase5 = 0x00002015,
    WHvX64RegisterMsrMtrrPhysBase6 = 0x00002016,
    WHvX64RegisterMsrMtrrPhysBase7 = 0x00002017,
    WHvX64RegisterMsrMtrrPhysBase8 = 0x00002018,
    WHvX64RegisterMsrMtrrPhysBase9 = 0x00002019,
    WHvX64RegisterMsrMtrrPhysBaseA = 0x0000201A,
    WHvX64RegisterMsrMtrrPhysBaseB = 0x0000201B,
    WHvX64RegisterMsrMtrrPhysBaseC = 0x0000201C,
    WHvX64RegisterMsrMtrrPhysBaseD = 0x0000201D,
    WHvX64RegisterMsrMtrrPhysBaseE = 0x0000201E,
    WHvX64RegisterMsrMtrrPhysBaseF = 0x0000201F,
    WHvX64RegisterMsrMtrrPhysMask0 = 0x00002040,
    WHvX64RegisterMsrMtrrPhysMask1 = 0x00002041,
    WHvX64RegisterMsrMtrrPhysMask2 = 0x00002042,
    WHvX64RegisterMsrMtrrPhysMask3 = 0x00002043,
    WHvX64RegisterMsrMtrrPhysMask4 = 0x00002044,
    WHvX64RegisterMsrMtrrPhysMask5 = 0x00002045,
    WHvX64RegisterMsrMtrrPhysMask6 = 0x00002046,
    WHvX64RegisterMsrMtrrPhysMask7 = 0x00002047,
    WHvX64RegisterMsrMtrrPhysMask8 = 0x00002048,
    WHvX64RegisterMsrMtrrPhysMask9 = 0x00002049,
    WHvX64RegisterMsrMtrrPhysMaskA = 0x0000204A,
    WHvX64RegisterMsrMtrrPhysMaskB = 0x0000204B,
    WHvX64RegisterMsrMtrrPhysMaskC = 0x0000204C,
    WHvX64RegisterMsrMtrrPhysMaskD = 0x0000204D,
    WHvX64RegisterMsrMtrrPhysMaskE = 0x0000204E,
    WHvX64RegisterMsrMtrrPhysMaskF = 0x0000204F,
    WHvX64RegisterMsrMtrrFix64k00000 = 0x00002070,
    WHvX64RegisterMsrMtrrFix16k80000 = 0x00002071,
    WHvX64RegisterMsrMtrrFix16kA0000 = 0x00002072,
    WHvX64RegisterMsrMtrrFix4kC0000 = 0x00002073,
    WHvX64RegisterMsrMtrrFix4kC8000 = 0x00002074,
    WHvX64RegisterMsrMtrrFix4kD0000 = 0x00002075,
    WHvX64RegisterMsrMtrrFix4kD8000 = 0x00002076,
    WHvX64RegisterMsrMtrrFix4kE0000 = 0x00002077,
    WHvX64RegisterMsrMtrrFix4kE8000 = 0x00002078,
    WHvX64RegisterMsrMtrrFix4kF0000 = 0x00002079,
    WHvX64RegisterMsrMtrrFix4kF8000 = 0x0000207A,
    WHvX64RegisterTscAux = 0x0000207B,
    WHvX64RegisterSpecCtrl = 0x00002084,
    WHvX64RegisterPredCmd = 0x00002085,
    WHvX64RegisterApicId = 0x00003002,
    WHvX64RegisterApicVersion = 0x00003003,
    WHvRegisterPendingInterruption = 0x80000000,
    WHvRegisterInterruptState = 0x80000001,
    WHvRegisterPendingEvent = 0x80000002,
    WHvX64RegisterDeliverabilityNotifications = 0x80000004,
    WHvRegisterInternalActivityState = 0x80000005,
}}
#[repr(C, align(16))]
#[derive(Copy, Clone, Debug)]
pub struct UINT128(pub [u64; 2]);
impl UINT128{
    #[inline]
    pub fn from_u128(x: u128)->Self{
        UINT128(unsafe{core::mem::transmute(x)})
    }
    #[inline]
    pub fn to_u128(self)->u128{
        unsafe{core::mem::transmute(self.0)}
    }
}

STRUCT!{struct WHV_UINT128_s{
    Low64: UINT64,
    High64: UINT64,
}}
UNION!{union WHV_UINT128{
    [UINT128; 1],
    s s_mut: WHV_UINT128_s,
    Dword Dword_mut: [UINT32; 4],
}}
// Dirty patches to make WHV_UINT128 work.
impl core::ops::Shl<usize> for UINT128{
    type Output = Self;
    fn shl(self, rhs: usize) -> Self::Output {
        Self::from_u128(self.to_u128() << rhs)
    }
}
impl core::ops::Shr<usize> for UINT128{
    type Output = Self;
    fn shr(self, rhs: usize) -> Self::Output {
        Self::from_u128(self.to_u128() >> rhs)
    }
}
impl core::ops::BitAnd<u128> for UINT128{
    type Output = Self;
    fn bitand(self, rhs: u128) -> Self::Output {
        Self::from_u128(self.to_u128() & rhs)
    }
}
impl core::ops::BitOr<u128> for UINT128{
    type Output = Self;
    fn bitor(self, rhs: u128) -> Self::Output {
        Self::from_u128(self.to_u128() | rhs)
    }
}
impl core::ops::BitOr<UINT128> for UINT128{
    type Output = Self;
    fn bitor(self, rhs: UINT128) -> Self::Output {
        Self::from_u128(self.to_u128() | rhs.to_u128())
    }
}
impl core::ops::Shl<usize> for WHV_UINT128{
    type Output = Self;
    fn shl(self, rhs: usize) -> Self::Output {
        Self([self.0[0]<<rhs])
    }
}
impl core::ops::Shr<usize> for WHV_UINT128{
    type Output = Self;
    fn shr(self, rhs: usize) -> Self::Output {
        Self([self.0[0]>>rhs])
    }
}
impl core::ops::BitAnd<u128> for WHV_UINT128{
    type Output = Self;
    fn bitand(self, rhs: u128) -> Self::Output {
        Self([self.0[0] & rhs])
    }
}
impl core::ops::BitAndAssign<u128> for WHV_UINT128{
    fn bitand_assign(&mut self, rhs: u128) {
        self.0[0] = self.0[0] & rhs;
    }
}
impl core::ops::BitOr<u128> for WHV_UINT128{
    type Output = Self;
    fn bitor(self, rhs: u128) -> Self::Output {
        Self([self.0[0] | rhs])
    }
}

impl core::ops::BitOrAssign<WHV_UINT128> for WHV_UINT128{
    fn bitor_assign(&mut self, rhs: WHV_UINT128) {
        self.0[0] = self.0[0] | rhs.0[0];
    }
}
STRUCT!{struct WHV_X64_FP_REGISTER{
    AsUINT128: WHV_UINT128,
}}
BITFIELD!{WHV_X64_FP_REGISTER AsUINT128: WHV_UINT128 [
    Mantissa set_Mantissa [0..64],
    BiasedExponent set_BiasedExponent [64..79],
    Sign set_Sign [79..80],
    Reserved set_Reserved [80..128],    
]}
STRUCT!{struct WHV_X64_FP_CONTROL_STATUS_REGISTER_s_u_s{
    LastFpEip: UINT32,
    LastFpCs: UINT16,
    Reserved2: UINT16,  
}}
UNION!{union WHV_X64_FP_CONTROL_STATUS_REGISTER_s_u{
    [u64; 1],
    LastFpRip LastFpRip_mut: UINT64,
    s s_mut: WHV_X64_FP_CONTROL_STATUS_REGISTER_s_u_s,
}}
STRUCT!{struct WHV_X64_FP_CONTROL_STATUS_REGISTER_s{
    FpControl: UINT16,
    FpStatus: UINT16,
    FpTag: UINT8,
    Reserved: UINT8,
    LastFpOp: UINT16,    
    u: WHV_X64_FP_CONTROL_STATUS_REGISTER_s_u,
}}
UNION!{union WHV_X64_FP_CONTROL_STATUS_REGISTER {
    [UINT128; 1],
    s s_mut: WHV_X64_FP_CONTROL_STATUS_REGISTER_s,
    AsUINT128 AsUINT128_mut: WHV_UINT128,
}}
STRUCT!{struct WHV_X64_XMM_CONTROL_STATUS_REGISTER_s_u_s{
    LastFpDp: UINT32,
    LastFpDs: UINT16,
    Reserved: UINT16,    
}}
UNION!{union WHV_X64_XMM_CONTROL_STATUS_REGISTER_s_u{
    [u64; 1],
    LastFpRdp LastFpRdp_mut: UINT64,
    s s_mut: WHV_X64_XMM_CONTROL_STATUS_REGISTER_s_u_s,
}}
STRUCT!{struct WHV_X64_XMM_CONTROL_STATUS_REGISTER_s{
    u: WHV_X64_XMM_CONTROL_STATUS_REGISTER_s_u,
    XmmStatusControl: UINT32,
    XmmStatusControlMask: UINT32,    
}}
UNION!{union WHV_X64_XMM_CONTROL_STATUS_REGISTER{
    [UINT128; 1],
    s s_mut: WHV_X64_XMM_CONTROL_STATUS_REGISTER_s,
    AsUINT128 AsUINT128_mut: WHV_UINT128,        
}}
STRUCT!{struct WHV_X64_SEGMENT_REGISTER{
    Base: UINT64,
    Limit: UINT32,
    Selector: UINT16,
    Attributes: UINT16,
}}
BITFIELD!{WHV_X64_SEGMENT_REGISTER Attributes: UINT16[
    SegmentType set_SegmentType [0..4],
    NonSystemSegment set_NonSystemSegment [4..5],
    DescriptorPrivilegeLevel set_DescriptorPrivilegeLevel [5..7],
    Present set_Present [7..8],
    Reserved set_Reserved [8..12],
    Available set_Available [12..13],
    Long set_Long [13..14],
    Default set_Default [14..15],
    Granularity set_Granularity [15..16],    
]}
STRUCT!{struct WHV_X64_TABLE_REGISTER {
    Pad: [UINT16; 3],
    Limit: UINT16,
    Base: UINT64,
}}
STRUCT!{struct WHV_X64_INTERRUPT_STATE_REGISTER{
    AsUINT64: UINT64,
}}
BITFIELD!{WHV_X64_INTERRUPT_STATE_REGISTER AsUINT64: UINT64[
    InterruptShadow set_InterruptShadow [0..1],
    NmiMasked set_NmiMasked [1..2],
    Reserved set_Reserved [2..64],
]}
STRUCT!{struct WHV_X64_PENDING_INTERRUPTION_REGISTER{
    AsUINT64: UINT64,
}}
BITFIELD!{WHV_X64_PENDING_INTERRUPTION_REGISTER AsUINT64: UINT64[
    InterruptionPending set_InterruptionPending [0..1],
    InterruptionType set_InterruptionType [1..4],
    DeliverErrorCode set_DeliverErrorCode [4..5],
    InstructionLength set_InstructionLength [5..9],
    NestedEvent set_NestedEvent [9..10],
    Reserved set_Reserved [10..16],
    InterruptionVector set_InterruptionVector [16..32],
    ErrorCode set_ErrorCode [32..64], 
]}
STRUCT!{struct WHV_X64_DELIVERABILITY_NOTIFICATIONS_REGISTER{
    AsUINT64: UINT64,
}}
BITFIELD!{WHV_X64_DELIVERABILITY_NOTIFICATIONS_REGISTER AsUINT64: UINT64[
    NmiNotification set_NmiNotification [0..1],
    InterruptNotification set_InterruptNotification [1..2],
    InterruptPriority set_InterruptPriority [2..6],
    Reserved set_Reserved [6..64],
]}
ENUM!{enum WHV_X64_PENDING_EVENT_TYPE {
    WHvX64PendingEventException = 0,
    WHvX64PendingEventExtInt = 5,
}}
STRUCT!{struct WHV_X64_PENDING_EXCEPTION_EVENT{
    AsUINT128: WHV_UINT128,
}}
BITFIELD!{WHV_X64_PENDING_EXCEPTION_EVENT AsUINT128: WHV_UINT128[
    EventPending set_EventPending [0..1],
    EventType set_EventType [1..4],
    Reserved0 set_Reserved0 [4..8],
    DeliverErrorCode set_DeliverErrorCode [8..9],
    Reserved1 set_Reserved1 [9..16],
    Vector set_Vector [16..32],
    ErrorCode set_ErrorCode [32..64],
    ExceptionParameter set_ExceptionParameter [64..128],    
]}
STRUCT!{struct WHV_X64_PENDING_EXT_INT_EVENT{
    AsUINT128: WHV_UINT128,
}}
BITFIELD!{WHV_X64_PENDING_EXT_INT_EVENT AsUINT128: WHV_UINT128[
    EventPending set_EventPending [0..1],
    EventType set_EventType [1..4],
    Reserved0 set_Reserved0 [4..8],
    Vector set_Vector [8..16],
    Reserved1 set_Reserved1 [16..64],
    Reserved2 set_Reserved2 [64..128],     
]}
UNION!{union WHV_REGISTER_VALUE {
    [UINT128; 1],
    Reg128 Reg128_mut: WHV_UINT128,
    Reg64 Reg64_mut: UINT64,
    Reg32 Reg32_mut: UINT32,
    Reg16 Reg16_mut: UINT16,
    Reg8 Reg8_mut: UINT8,
    Fp Fp_mut: WHV_X64_FP_REGISTER,
    FpControlStatus FpControlStatus_mut: WHV_X64_FP_CONTROL_STATUS_REGISTER,
    XmmControlStatus XmmControlStatus_mut: WHV_X64_XMM_CONTROL_STATUS_REGISTER,
    Segment Segment_mut: WHV_X64_SEGMENT_REGISTER,
    Table Table_mut: WHV_X64_TABLE_REGISTER,
    InterruptState InterruptState_mut: WHV_X64_INTERRUPT_STATE_REGISTER,
    DeliverabilityNotifications DeliverabilityNotifications_mut: WHV_X64_DELIVERABILITY_NOTIFICATIONS_REGISTER,
    ExceptionEvent ExceptionEvent_mut: WHV_X64_PENDING_EXCEPTION_EVENT,
    ExtIntEvent ExtIntEvent_mut: WHV_X64_PENDING_EXT_INT_EVENT,    
}}
ENUM!{enum WHV_RUN_VP_EXIT_REASON {
    WHvRunVpExitReasonNone = 0x00000000,
    WHvRunVpExitReasonMemoryAccess = 0x00000001,
    WHvRunVpExitReasonX64IoPortAccess = 0x00000002,
    WHvRunVpExitReasonUnrecoverableException = 0x00000004,
    WHvRunVpExitReasonInvalidVpRegisterValue = 0x00000005,
    WHvRunVpExitReasonUnsupportedFeature = 0x00000006,
    WHvRunVpExitReasonX64InterruptWindow = 0x00000007,
    WHvRunVpExitReasonX64Halt = 0x00000008,
    WHvRunVpExitReasonX64ApicEoi = 0x00000009,
    WHvRunVpExitReasonX64MsrAccess = 0x00001000,
    WHvRunVpExitReasonX64Cpuid = 0x00001001,
    WHvRunVpExitReasonException = 0x00001002,
    WHvRunVpExitReasonCanceled = 0x00002001,
}}
STRUCT!{struct WHV_X64_VP_EXECUTION_STATE{
    AsUINT16: UINT16,
}}
BITFIELD!{WHV_X64_VP_EXECUTION_STATE AsUINT16: UINT16[
    Cpl set_Cpl [0..2],
    Cr0Pe set_Cr0Pe [2..3],
    Cr0Am set_Cr0Am [3..4],
    EferLma set_EferLma [4..5],
    DebugActive set_DebugActive [5..6],
    InterruptionPending set_InterruptionPending [6..7],
    Reserved0 set_Reserved0 [7..12],
    InterruptShadow set_InterruptShadow [12..13],
    Reserved1 set_Reserved1 [13..16],    
]}
STRUCT!{struct WHV_VP_EXIT_CONTEXT {
    ExecutionState: WHV_X64_VP_EXECUTION_STATE,
    InstructionLengthAndCr8: UINT8, // TODO: What if bitfields appear in a struct?
    Reserved: UINT8,
    Reserved2: UINT32,
    Cs: WHV_X64_SEGMENT_REGISTER,
    Rip: UINT64,
    Rflags: UINT64,    
}}
BITFIELD!{WHV_VP_EXIT_CONTEXT  InstructionLengthAndCr8: UINT8[
    InstructionLength set_InstructionLength [0..3],
    Cr8 set_Cr8 [4..7],
]}
ENUM!{enum WHV_MEMORY_ACCESS_TYPE {
    WHvMemoryAccessRead = 0,
    WHvMemoryAccessWrite = 1,
    WHvMemoryAccessExecute = 2,
}}
STRUCT!{struct WHV_MEMORY_ACCESS_INFO{
    AsUINT32: UINT32,
}}
BITFIELD!{WHV_MEMORY_ACCESS_INFO AsUINT32: UINT32[
    AccessType set_AccessType [0..2],
    GpaUnmapped set_GpaUnmapped [2..3],
    GvaValid set_GvaValid [3..4],
    Reserved set_Reserved [4..32],    
]}
STRUCT!{struct WHV_MEMORY_ACCESS_CONTEXT {
    InstructionByteCount: UINT8,
    Reserved: [UINT8; 3],
    InstructionBytes: [UINT8; 16],
    AccessInfo: WHV_MEMORY_ACCESS_INFO,
    Gpa: WHV_GUEST_PHYSICAL_ADDRESS,
    Gva: WHV_GUEST_VIRTUAL_ADDRESS,    
}}
STRUCT!{struct WHV_X64_IO_PORT_ACCESS_INFO{
    AsUINT32: UINT32,
}}
BITFIELD!{WHV_X64_IO_PORT_ACCESS_INFO AsUINT32: UINT32[
    IsWrite set_IsWrite [0..1],
    AccessSize set_AccessSize [1..4],
    StringOp set_StringOp [4..5],
    RepPrefix set_RepPrefix [5..6],
    Reserved set_Reserved [6..32],     
]}
STRUCT!{struct WHV_X64_IO_PORT_ACCESS_CONTEXT {
    InstructionByteCount: UINT8,
    Reserved: [UINT8; 3],
    InstructionBytes: [UINT8; 16],
    AccessInfo: WHV_X64_IO_PORT_ACCESS_INFO,
    PortNumber: UINT16,
    Reserved2: [UINT16; 3],
    Rax: UINT64,
    Rcx: UINT64,
    Rsi: UINT64,
    Rdi: UINT64,
    Ds: WHV_X64_SEGMENT_REGISTER,
    Es: WHV_X64_SEGMENT_REGISTER,    
}}
STRUCT!{struct WHV_X64_MSR_ACCESS_INFO{
    AsUINT32: UINT32,
}}
BITFIELD!{WHV_X64_MSR_ACCESS_INFO AsUINT32: UINT32[
    IsWrite set_IsWrite [0..1],
    Reserved set_Reserved [1..32],     
]}
STRUCT!{struct WHV_X64_MSR_ACCESS_CONTEXT {
    AccessInfo: WHV_X64_MSR_ACCESS_INFO,
    MsrNumber: UINT32,
    Rax: UINT64,
    Rdx: UINT64,      
}}
STRUCT!{struct WHV_X64_CPUID_ACCESS_CONTEXT {
    Rax: UINT64,
    Rcx: UINT64,
    Rdx: UINT64,
    Rbx: UINT64,
    DefaultResultRax: UINT64,
    DefaultResultRcx: UINT64,
    DefaultResultRdx: UINT64,
    DefaultResultRbx: UINT64,    
}}
STRUCT!{struct WHV_VP_EXCEPTION_INFO{
    AsUINT32: UINT32,
}}
BITFIELD!{WHV_VP_EXCEPTION_INFO AsUINT32: UINT32[
    ErrorCodeValid set_ErrorCodeValid [0..1],
    SoftwareException set_SoftwareException [1..2],
    Reserved set_Reserved [2..32],     
]}
STRUCT!{struct WHV_VP_EXCEPTION_CONTEXT {
    InstructionByteCount: UINT8,
    Reserved: [UINT8; 3],
    InstructionBytes: [UINT8; 16],
    ExceptionInfo: WHV_VP_EXCEPTION_INFO,
    ExceptionType: UINT8,
    Reserved2: [UINT8; 3],
    ErrorCode: UINT32,
    ExceptionParameter: UINT64,    
}}
ENUM!{enum WHV_X64_UNSUPPORTED_FEATURE_CODE {
    WHvUnsupportedFeatureIntercept = 1,
    WHvUnsupportedFeatureTaskSwitchTss = 2,
}}
STRUCT!{struct WHV_X64_UNSUPPORTED_FEATURE_CONTEXT {
    FeatureCode: WHV_X64_UNSUPPORTED_FEATURE_CODE,
    Reserved: UINT32,
    FeatureParameter: UINT64,
}}
ENUM!{enum WHV_RUN_VP_CANCEL_REASON {
    WhvRunVpCancelReasonUser = 0,
}}
STRUCT!{struct WHV_RUN_VP_CANCELED_CONTEXT {
    CancelReason: WHV_RUN_VP_CANCEL_REASON,
}}
ENUM!{enum WHV_X64_PENDING_INTERRUPTION_TYPE {
    WHvX64PendingInterrupt = 0,
    WHvX64PendingNmi = 2,
    WHvX64PendingException = 3,
}}
pub type PWHV_X64_PENDING_INTERRUPTION_TYPE = *mut WHV_X64_PENDING_INTERRUPTION_TYPE;
STRUCT!{struct WHV_X64_INTERRUPTION_DELIVERABLE_CONTEXT {
    DeliverableType: WHV_X64_PENDING_INTERRUPTION_TYPE,
}}
pub type PWHV_X64_INTERRUPTION_DELIVERABLE_CONTEXT = *mut WHV_X64_INTERRUPTION_DELIVERABLE_CONTEXT;
STRUCT!{struct WHV_X64_APIC_EOI_CONTEXT {
    InterruptVector: UINT32,
}}
UNION!{union WHV_RUN_VP_EXIT_CONTEXT_u{
    [u64; 12],
    MemoryAccess MemoryAccess_mut: WHV_MEMORY_ACCESS_CONTEXT,
    IoPortAccess IoPortAccess_mut: WHV_X64_IO_PORT_ACCESS_CONTEXT,
    MsrAccess MsrAccess_mut: WHV_X64_MSR_ACCESS_CONTEXT,
    CpuidAccess CpuidAccess_mut: WHV_X64_CPUID_ACCESS_CONTEXT,
    VpException VpException_mut: WHV_VP_EXCEPTION_CONTEXT,
    InterruptWindow InterruptWindow_mut: WHV_X64_INTERRUPTION_DELIVERABLE_CONTEXT,
    UnsupportedFeature UnsupportedFeature_mut: WHV_X64_UNSUPPORTED_FEATURE_CONTEXT,
    CancelReason CancelReason_mut: WHV_RUN_VP_CANCELED_CONTEXT,
    ApicEoi ApicEoi_mut: WHV_X64_APIC_EOI_CONTEXT,    
}}
STRUCT!{struct WHV_RUN_VP_EXIT_CONTEXT{
    ExitReason: WHV_RUN_VP_EXIT_REASON,
    Reserved: UINT32,
    VpContext: WHV_VP_EXIT_CONTEXT,    
    u: WHV_RUN_VP_EXIT_CONTEXT_u,
}}
ENUM!{enum WHV_INTERRUPT_TYPE {
    WHvX64InterruptTypeFixed = 0,
    WHvX64InterruptTypeLowestPriority = 1,
    WHvX64InterruptTypeNmi = 4,
    WHvX64InterruptTypeInit = 5,
    WHvX64InterruptTypeSipi = 6,
    WHvX64InterruptTypeLocalInt1 = 9,
}}
ENUM!{enum WHV_INTERRUPT_DESTINATION_MODE {
    WHvX64InterruptDestinationModePhysical,
    WHvX64InterruptDestinationModeLogical,
}}
ENUM!{enum WHV_INTERRUPT_TRIGGER_MODE {
    WHvX64InterruptTriggerModeEdge,
    WHvX64InterruptTriggerModeLevel,
}}
STRUCT!{struct WHV_INTERRUPT_CONTROL{
    TypeAndDestinationModeAndTriggerModeAndReserved: UINT64,
    Destination: UINT32,
    Vector: UINT32,
}}
BITFIELD!{WHV_INTERRUPT_CONTROL TypeAndDestinationModeAndTriggerModeAndReserved: UINT64[
    Type set_Type [0..8],
    DestinationMode set_DestinationMode [8..12],
    TriggerMode set_TriggerMode [12..16],
    Reserved set_Reserved [16..64],    
]}
ENUM!{enum WHV_PARTITION_COUNTER_SET {
    WHvPartitionCounterSetMemory,
}}
STRUCT!{struct WHV_PARTITION_MEMORY_COUNTERS {
    Mapped4KPageCount: UINT64,
    Mapped2MPageCount: UINT64,
    Mapped1GPageCount: UINT64,
}}
ENUM!{enum WHV_PROCESSOR_COUNTER_SET {
    WHvProcessorCounterSetRuntime,
    WHvProcessorCounterSetIntercepts,
    WHvProcessorCounterSetEvents,
    WHvProcessorCounterSetApic,
}}
STRUCT!{struct WHV_PROCESSOR_RUNTIME_COUNTERS {
    TotalRuntime100ns: UINT64,
    HypervisorRuntime100ns: UINT64,
}}
STRUCT!{struct WHV_PROCESSOR_INTERCEPT_COUNTER {
    Count: UINT64,
    Time100ns: UINT64,
}}
STRUCT!{struct WHV_PROCESSOR_INTERCEPT_COUNTERS {
    PageInvalidations: WHV_PROCESSOR_INTERCEPT_COUNTER,
    ControlRegisterAccesses: WHV_PROCESSOR_INTERCEPT_COUNTER,
    IoInstructions: WHV_PROCESSOR_INTERCEPT_COUNTER,
    HaltInstructions: WHV_PROCESSOR_INTERCEPT_COUNTER,
    CpuidInstructions: WHV_PROCESSOR_INTERCEPT_COUNTER,
    MsrAccesses: WHV_PROCESSOR_INTERCEPT_COUNTER,
    OtherIntercepts: WHV_PROCESSOR_INTERCEPT_COUNTER,
    PendingInterrupts: WHV_PROCESSOR_INTERCEPT_COUNTER,
    EmulatedInstructions: WHV_PROCESSOR_INTERCEPT_COUNTER,
    DebugRegisterAccesses: WHV_PROCESSOR_INTERCEPT_COUNTER,
    PageFaultIntercepts: WHV_PROCESSOR_INTERCEPT_COUNTER,
}}
STRUCT!{struct WHV_PROCESSOR_EVENT_COUNTERS {
    PageFaultCount: UINT64,
    ExceptionCount: UINT64,
    InterruptCount: UINT64,
}}
STRUCT!{struct WHV_PROCESSOR_APIC_COUNTERS {
    MmioAccessCount: UINT64,
    EoiAccessCount: UINT64,
    TprAccessCount: UINT64,
    SentIpiCount: UINT64,
    SelfIpiCount: UINT64,    
}}
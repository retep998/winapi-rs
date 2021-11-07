use ctypes::{c_void};
use shared::basetsd::{UINT32, UINT16, UINT8};
use shared::winerror::HRESULT;
use um::winhvplatformdefs::{
    WHV_GUEST_PHYSICAL_ADDRESS,
    WHV_REGISTER_NAME,
    WHV_REGISTER_VALUE,
    WHV_GUEST_VIRTUAL_ADDRESS,
    WHV_TRANSLATE_GVA_FLAGS,
    WHV_TRANSLATE_GVA_RESULT_CODE,
    WHV_VP_EXIT_CONTEXT,
    WHV_X64_IO_PORT_ACCESS_CONTEXT,
    WHV_MEMORY_ACCESS_CONTEXT,
};

STRUCT!{struct WHV_EMULATOR_STATUS {
    AsUINT32: UINT32,
}}
BITFIELD!{WHV_EMULATOR_STATUS AsUINT32: UINT32 [
    EmulationSuccessful set_EmulationSuccessful[0..1],
    InternalEmulationFailure set_InternalEmulationFailure[1..2],
    IoPortCallbackFailed set_IoPortCallbackFailed[2..3],
    MemoryCallbackFailed set_MemoryCallbackFailed[3..4],
    TranslateGvaPageCallbackFailed set_TranslateGvaPageCallbackFailed[4..5],
    TranslateGvaPageCallbackGpaIsNotAligned set_TranslateGvaPageCallbackGpaIsNotAligned[5..6],
    GetVirtualProcessorRegistersCallbackFailed set_GetVirtualProcessorRegistersCallbackFailed[6..7],
    SetVirtualProcessorRegistersCallbackFailed set_SetVirtualProcessorRegistersCallbackFailed[7..8],
    InterruptCausedIntercept set_InterruptCausedIntercept[8..9],
    GuestCannotBeFaulted set_GuestCannotBeFaulted[9..10],
    Reserved set_Reserved[10..32],
]}
STRUCT!{struct WHV_EMULATOR_MEMORY_ACCESS_INFO {
    GpaAddress: WHV_GUEST_PHYSICAL_ADDRESS,
    Direction: UINT8,
    AccessSize: UINT8,
    Data: [UINT8; 3],
}}
STRUCT!{struct WHV_EMULATOR_IO_ACCESS_INFO {
    Direction: UINT8,
    Port: UINT16,
    AccessSize: UINT16,
    Data: UINT32,
}}
FN!{stdcall WHV_EMULATOR_IO_PORT_CALLBACK(
    Context: *mut c_void,
    IoAccess: *mut WHV_EMULATOR_IO_ACCESS_INFO,
) -> HRESULT}
FN!{stdcall WHV_EMULATOR_MEMORY_CALLBACK(
    Context: *mut c_void,
    MemoryAccess: *mut WHV_EMULATOR_MEMORY_ACCESS_INFO,
) -> HRESULT}
FN!{stdcall WHV_EMULATOR_GET_VIRTUAL_PROCESSOR_REGISTERS_CALLBACK(
    Context: *mut c_void,
    RegisterNames: *const WHV_REGISTER_NAME,
    RegisterCount: UINT32,
    RegisterValues: *mut WHV_REGISTER_VALUE,
) -> HRESULT}
FN!{stdcall WHV_EMULATOR_SET_VIRTUAL_PROCESSOR_REGISTERS_CALLBACK(
    Context: *mut c_void,
    RegisterNames: *const WHV_REGISTER_NAME,
    RegisterCount: UINT32,
    RegisterValues: *const WHV_REGISTER_VALUE,
) -> HRESULT}
FN!{stdcall WHV_EMULATOR_TRANSLATE_GVA_PAGE_CALLBACK(
    Context: *mut c_void,
    Gva: WHV_GUEST_VIRTUAL_ADDRESS,
    TranslateFlags: WHV_TRANSLATE_GVA_FLAGS,
    TranslationResult: *mut WHV_TRANSLATE_GVA_RESULT_CODE,
    Gpa: *mut WHV_GUEST_PHYSICAL_ADDRESS,
) -> HRESULT}
STRUCT!{struct WHV_EMULATOR_CALLBACKS {
    Size: UINT32,
    Reserved: UINT32,
    WHvEmulatorIoPortCallback: WHV_EMULATOR_IO_PORT_CALLBACK,
    WHvEmulatorMemoryCallback: WHV_EMULATOR_MEMORY_CALLBACK,
    WHvEmulatorGetVirtualProcessorRegisters: WHV_EMULATOR_GET_VIRTUAL_PROCESSOR_REGISTERS_CALLBACK,
    WHvEmulatorSetVirtualProcessorRegisters: WHV_EMULATOR_SET_VIRTUAL_PROCESSOR_REGISTERS_CALLBACK,
    WHvEmulatorTranslateGvaPage: WHV_EMULATOR_TRANSLATE_GVA_PAGE_CALLBACK,
}}
pub type WHV_EMULATOR_HANDLE = *mut c_void;
extern "system" {
    pub fn WHvEmulatorCreateEmulator(
        Callbacks: *const WHV_EMULATOR_CALLBACKS,
        Emulator: *mut WHV_EMULATOR_HANDLE,
    ) -> HRESULT;
    pub fn WHvEmulatorDestroyEmulator(
        Emulator: WHV_EMULATOR_HANDLE,
    ) -> HRESULT;
    pub fn WHvEmulatorTryIoEmulation(
        Emulator: WHV_EMULATOR_HANDLE,
        Context: *mut c_void,
        VpContext: *const WHV_VP_EXIT_CONTEXT,
        IoInstructionContext: *const WHV_X64_IO_PORT_ACCESS_CONTEXT,
        EmulatorReturnStatus: *mut WHV_EMULATOR_STATUS,
    ) -> HRESULT;
    pub fn WHvEmulatorTryMmioEmulation(
        Emulator: WHV_EMULATOR_HANDLE,
        Context: *mut c_void,
        VpContext: *const WHV_VP_EXIT_CONTEXT,
        MmioInstructionContext: *const WHV_MEMORY_ACCESS_CONTEXT,
        EmulatorReturnStatus: *mut WHV_EMULATOR_STATUS,
    ) -> HRESULT;
}

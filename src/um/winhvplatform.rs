use shared::basetsd::{UINT32, UINT64};
use shared::minwindef::{LPCVOID, LPVOID};
use shared::ntdef::{BOOLEAN, HRESULT};
use um::winhvplatformdefs::*;
extern "system"{
    pub fn WHvGetCapability(
        CapabilityCode: WHV_CAPABILITY_CODE,
        CapabilityBuffer: LPVOID,
        CapabilityBufferSizeInBytes: UINT32,
        WrittenSizeInBytes: *mut UINT32,
    ) -> HRESULT;
    pub fn WHvCreatePartition(
        Partition: *mut WHV_PARTITION_HANDLE,
    ) -> HRESULT;
    pub fn WHvSetupPartition(
        Partition: WHV_PARTITION_HANDLE,
    ) -> HRESULT;
    pub fn WHvDeletePartition(
        Partition: WHV_PARTITION_HANDLE,
    ) -> HRESULT;
    pub fn WHvGetPartitionProperty(
        Partition: WHV_PARTITION_HANDLE,
        PropertyCode: WHV_PARTITION_PROPERTY_CODE,
        PropertyBuffer: LPCVOID,
        PropertyBufferSizeInBytes: UINT32,
    ) -> HRESULT;
    pub fn WHvSuspendPartitionTime(
        Partition: WHV_PARTITION_HANDLE,
    ) -> HRESULT;
    pub fn WHvResumePartitionTime(
        Partition: WHV_PARTITION_HANDLE,
    ) -> HRESULT;
    pub fn WHvMapGpaRange(
        Partition: WHV_PARTITION_HANDLE,
        SourceAddress: LPVOID,
        GuestAddress: WHV_GUEST_PHYSICAL_ADDRESS,
        SizeInBytes: UINT64,
        Flags: WHV_MAP_GPA_RANGE_FLAGS,
    ) -> HRESULT;
    pub fn WHvUnmapGpaRange(
        Partition: WHV_PARTITION_HANDLE,
        GuestAddress: WHV_GUEST_PHYSICAL_ADDRESS,
        SizeInBytes: UINT64,
    ) -> HRESULT;
    pub fn WHvTranslateGva(
        Partition: WHV_PARTITION_HANDLE,
        VpIndex: UINT32,
        Gva: WHV_GUEST_VIRTUAL_ADDRESS,
        TranslateFlags: WHV_TRANSLATE_GVA_FLAGS,
        TranslationResult: *mut WHV_TRANSLATE_GVA_RESULT,
        Gpa: WHV_GUEST_PHYSICAL_ADDRESS,
    ) -> HRESULT;
    pub fn WHvCreateVirtualProcessor(
        Partition: WHV_PARTITION_HANDLE,
        VpIndex: UINT32,
        Flags: UINT32,
    ) -> HRESULT;
    pub fn WHvDeleteVirtualProcessor(
        Partition: WHV_PARTITION_HANDLE,
        VpIndex: UINT32,
    ) -> HRESULT;
    pub fn WHvRunVirtualProcessor(
        Partition: WHV_PARTITION_HANDLE,
        VpIndex: UINT32,
        ExitContext: LPVOID,
        ExitContextSizeInBytes: UINT32,
    ) -> HRESULT;
    pub fn WHvCancelRunVirtualProcessor(
        Partition: WHV_PARTITION_HANDLE,
        VpIndex: UINT32,
        Flags: UINT32,
    ) -> HRESULT;
    pub fn WHvGetVirtualProcessorRegisters(
        Partition: WHV_PARTITION_HANDLE,
        VpIndex: UINT32,
        RegisterNames: *const WHV_REGISTER_NAME,
        RegisterCount: UINT32,
        RegisterValues: *mut WHV_REGISTER_VALUE,
    ) -> HRESULT;
    pub fn WHvSetVirtualProcessorRegisters(
        Partition: WHV_PARTITION_HANDLE,
        VpIndex: UINT32,
        RegisterNames: *const WHV_REGISTER_NAME,
        RegisterCount: UINT32,
        RegisterValues: *const WHV_REGISTER_VALUE,
    ) -> HRESULT;
    pub fn WHvGetVirtualProcessorInterruptControllerState(
        Partition: WHV_PARTITION_HANDLE,
        VpIndex: UINT32,
        State: LPVOID,
        StateSize: UINT32,
        WrittenSIze: *mut UINT32,
    ) -> HRESULT;
    pub fn WHvSetVirtualProcessorInterruptControllerState(
        Partition: WHV_PARTITION_HANDLE,
        VpIndex: UINT32,
        State: LPCVOID,
        StateSize: UINT32,
    ) -> HRESULT;
    pub fn WHvRequestInterrupt(
        Partition: WHV_PARTITION_HANDLE,
        Interrupt: *const WHV_INTERRUPT_CONTROL,
        InterruptControlSize: UINT32,
    ) -> HRESULT;
    pub fn WHvGetVirtualProcessorXsaveState(
        Partition: WHV_PARTITION_HANDLE,
        VpIndex: UINT32,
        Buffer: LPVOID,
        BufferSizeInBytes: UINT32,
        BytesWritten: *mut UINT32,
    ) -> HRESULT;
    pub fn WHvSetVirtualProcessorXsaveState(
        Partition: WHV_PARTITION_HANDLE,
        VpIndex: UINT32,
        Buffer: LPVOID,
        BufferSizeInBytes: UINT32,
    ) -> HRESULT;
    pub fn WHvQueryGpaRangeDirtyBirmap(
        Partition: WHV_PARTITION_HANDLE,
        GuestAddress: WHV_GUEST_PHYSICAL_ADDRESS,
        Bitmap: *mut UINT64,
        BitmapSizeInBytes: UINT32,
    ) -> HRESULT;
    pub fn WHvGetPartitionCounters(
        Partition: WHV_PARTITION_HANDLE,
        CounterSet: WHV_PARTITION_COUNTER_SET,
        Buffer: LPVOID,
        BufferSizeInBytes: UINT32,
        BytesWritten: *mut UINT32,
    ) -> HRESULT;
    pub fn WHvGetVirtualPartitionCounters(
        Partition: WHV_PARTITION_HANDLE,
        VpIndex: UINT32,
        CounterSet: WHV_PARTITION_COUNTER_SET,
        Buffer: LPVOID,
        BufferSizeInBytes: UINT32,
        BytesWritten: *mut UINT32,
    ) -> HRESULT;
    pub fn IsWHvGetCapabilityPresent()->BOOLEAN;
    pub fn IsWHvCreatePartitionPresent()->BOOLEAN;
    pub fn IsWHvSetupPartitionPresent()->BOOLEAN;
    pub fn IsWHvDeletePartitionPresent()->BOOLEAN;
    pub fn IsWHvGetPartitionPropertyPresent()->BOOLEAN;
    pub fn IsWHvSetPartitionPropertyPresent()->BOOLEAN;
    pub fn IsWHvSuspendPartitionTimePresent()->BOOLEAN;
    pub fn IsWHvResumePartitionTimePresent()->BOOLEAN;
    pub fn IsWHvMapGpaRangePresent()->BOOLEAN;
    pub fn IsWHvUnmapGpaRangePresent()->BOOLEAN;
    pub fn IsWHvTranslateGvaPresent()->BOOLEAN;
    pub fn IsWHvCreateVirtualProcessorPresent()->BOOLEAN;
    pub fn IsWHvDeleteVirtualProcessorPresent()->BOOLEAN;
    pub fn IsWHvRunVirtualProcessorPresent()->BOOLEAN;
    pub fn IsWHvCancelRunVirtualProcessorPresent()->BOOLEAN;
    pub fn IsWHvGetVirtualProcessorRegistersPresent()->BOOLEAN;
    pub fn IsWHvSetVirtualProcessorRegistersPresent()->BOOLEAN;
    pub fn IsWHvGetVirtualProcessorInterruptControllerStatePresent()->BOOLEAN;
    pub fn IsWHvSetVirtualProcessorInterruptControllerStatePresent()->BOOLEAN;
    pub fn IsWHvRequestInterruptPresent()->BOOLEAN;
    pub fn IsWHvGetVirtualProcessorXsaveStatePresent()->BOOLEAN;
    pub fn IsWHvSetVirtualProcessorXsaveStatePresent()->BOOLEAN;
    pub fn IsWHvQueryGpaRangeDirtyBitmapPresent()->BOOLEAN;
    pub fn IsWHvGetPartitionCountersPresent()->BOOLEAN;
    pub fn IsWHvGetVirtualProcessorCountersPresent()->BOOLEAN;
}
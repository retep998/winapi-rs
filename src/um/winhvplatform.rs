use ctypes::{c_void};
use shared::basetsd::{UINT32, UINT64};
use shared::winerror::HRESULT;
use um::winhvplatformdefs::{
    WHV_CAPABILITY_CODE,
    WHV_PARTITION_HANDLE,
    WHV_PARTITION_PROPERTY_CODE,
    WHV_GUEST_PHYSICAL_ADDRESS,
    WHV_MAP_GPA_RANGE_FLAGS,
    WHV_GUEST_VIRTUAL_ADDRESS,
    WHV_TRANSLATE_GVA_FLAGS,
    WHV_TRANSLATE_GVA_RESULT,
    WHV_REGISTER_NAME,
    WHV_REGISTER_VALUE,
    WHV_INTERRUPT_CONTROL,
    WHV_PARTITION_COUNTER_SET,
    WHV_PROCESSOR_COUNTER_SET,
};

extern "system" {
    pub fn WHvGetCapability(
        CapabilityCode: WHV_CAPABILITY_CODE,
        CapabilityBuffer: *mut c_void,
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
        PropertyBuffer: *mut c_void,
        PropertyBufferSizeInBytes: UINT32,
        WrittenSizeInBytes: *mut UINT32,
    ) -> HRESULT;
    pub fn WHvSetPartitionProperty(
        Partition: WHV_PARTITION_HANDLE,
        PropertyCode: WHV_PARTITION_PROPERTY_CODE,
        PropertyBuffer: *const c_void,
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
        SourceAddress: *mut c_void,
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
        Gpa: *mut WHV_GUEST_PHYSICAL_ADDRESS,
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
        ExitContext: *mut c_void,
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
        State: *mut c_void,
        StateSize: UINT32,
        WrittenSize: *mut UINT32,
    ) -> HRESULT;
    pub fn WHvSetVirtualProcessorInterruptControllerState(
        Partition: WHV_PARTITION_HANDLE,
        VpIndex: UINT32,
        State: *const c_void,
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
        Buffer: *mut c_void,
        BufferSizeInBytes: UINT32,
        BytesWritten: *mut UINT32,
    ) -> HRESULT;
    pub fn WHvSetVirtualProcessorXsaveState(
        Partition: WHV_PARTITION_HANDLE,
        VpIndex: UINT32,
        Buffer: *const c_void,
        BufferSizeInBytes: UINT32,
    ) -> HRESULT;
    pub fn WHvQueryGpaRangeDirtyBitmap(
        Partition: WHV_PARTITION_HANDLE,
        GuestAddress: WHV_GUEST_PHYSICAL_ADDRESS,
        RangeSizeInBytes: UINT64,
        Bitmap: *mut UINT64,
        BitmapSizeInBytes: UINT32,
    ) -> HRESULT;
    pub fn WHvGetPartitionCounters(
        Partition: WHV_PARTITION_HANDLE,
        CounterSet: WHV_PARTITION_COUNTER_SET,
        Buffer: *mut c_void,
        BufferSizeInBytes: UINT32,
        BytesWritten: *mut UINT32,
    ) -> HRESULT;
    pub fn WHvGetVirtualProcessorCounters(
        Partition: WHV_PARTITION_HANDLE,
        VpIndex: UINT32,
        CounterSet: WHV_PROCESSOR_COUNTER_SET,
        Buffer: *mut c_void,
        BufferSizeInBytes: UINT32,
        BytesWritten: *mut UINT32,
    ) -> HRESULT;
}

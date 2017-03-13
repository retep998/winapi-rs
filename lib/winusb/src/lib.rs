// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to winusb.
#![cfg(windows)]
extern crate winapi;
use winapi::*;
extern "system" {
    pub fn WinUsb_Initialize(
        DeviceHandle: HANDLE, InterfaceHandle: PWINUSB_INTERFACE_HANDLE,
    ) -> BOOL;
    pub fn WinUsb_Free(
        InterfaceHandle: WINUSB_INTERFACE_HANDLE,
    ) -> BOOL;
    pub fn WinUsb_GetAssociatedInterface(
        InterfaceHandle: WINUSB_INTERFACE_HANDLE, AssociatedInterfaceIndex: UCHAR,
        AssociatedInterfaceHandle: PWINUSB_INTERFACE_HANDLE,
    ) -> BOOL;
    pub fn WinUsb_GetDescriptor(
        InterfaceHandle: WINUSB_INTERFACE_HANDLE, DescriptorType: UCHAR,
        Index: UCHAR, LanguageID: USHORT,
        Buffer: PUCHAR, BufferLength: ULONG, LengthTransferred: PULONG,
    ) -> BOOL;
    pub fn WinUsb_QueryInterfaceSettings(
        InterfaceHandle: WINUSB_INTERFACE_HANDLE, AlternateInterfaceNumber: UCHAR,
        UsbAltInterfaceDescriptor: PUSB_INTERFACE_DESCRIPTOR,
    ) -> BOOL;
    pub fn WinUsb_QueryDeviceInformation(
        InterfaceHandle: WINUSB_INTERFACE_HANDLE, InformationType: ULONG,
        BufferLength: PULONG, Buffer: PVOID,
    ) -> BOOL;
    pub fn WinUsb_SetCurrentAlternateSetting(
        InterfaceHandle: WINUSB_INTERFACE_HANDLE, SettingNumber: UCHAR,
    ) -> BOOL;
    pub fn WinUsb_GetCurrentAlternateSetting(
        InterfaceHandle: WINUSB_INTERFACE_HANDLE, SettingNumber: PUCHAR,
    ) -> BOOL;
    pub fn WinUsb_QueryPipe(
        InterfaceHandle: WINUSB_INTERFACE_HANDLE, AlternateInterfaceNumber: UCHAR,
        PipeIndex: UCHAR, PipeInformationEx: PWINUSB_PIPE_INFORMATION_EX,
    ) -> BOOL;
    pub fn WinUsb_SetPipePolicy(
        InterfaceHandle: WINUSB_INTERFACE_HANDLE, PipeID: UCHAR,
        PolicyType: ULONG, ValueLength: ULONG, Value: PVOID,
    ) -> BOOL;
    pub fn WinUsb_GetPipePolicy(
        InterfaceHandle: WINUSB_INTERFACE_HANDLE, PipeID: UCHAR,
        PolicyType: ULONG, ValueLength: PULONG, Value: PVOID,
    ) -> BOOL;
    pub fn WinUsb_ReadPipe(
        InterfaceHandle: WINUSB_INTERFACE_HANDLE, PipeID: UCHAR,
        Buffer: PUCHAR, BufferLength: ULONG, 
        LengthTransferred: PULONG, Overlapped: LPOVERLAPPED,
    ) -> BOOL;
    pub fn WinUsb_WritePipe(
        InterfaceHandle: WINUSB_INTERFACE_HANDLE, PipeID: UCHAR,
        Buffer: PUCHAR, BufferLength: ULONG, 
        LengthTransferred: PULONG, Overlapped: LPOVERLAPPED,
    ) -> BOOL;
    pub fn WinUsb_ControlTransfer(
        InterfaceHandle: WINUSB_INTERFACE_HANDLE, SetupPacket: WINUSB_SETUP_PACKET,
        Buffer: PUCHAR, BufferLength: ULONG, 
        LengthTransferred: PULONG, Overlapped: LPOVERLAPPED,
    ) -> BOOL;
    pub fn WinUsb_ResetPipe(
        InterfaceHandle: WINUSB_INTERFACE_HANDLE, PipeID: UCHAR,
    ) -> BOOL;
    pub fn WinUsb_AbortPipe(
        InterfaceHandle: WINUSB_INTERFACE_HANDLE, PipeID: UCHAR,
    ) -> BOOL;
    pub fn WinUsb_FlushPipe(
        InterfaceHandle: WINUSB_INTERFACE_HANDLE, PipeID: UCHAR,
    ) -> BOOL;
    pub fn WinUsb_SetPowerPolicy(
        InterfaceHandle: WINUSB_INTERFACE_HANDLE, PolicyType: ULONG,
        ValueLength: ULONG, Value: PVOID,
    ) -> BOOL;
    pub fn WinUsb_GetPowerPolicy(
        InterfaceHandle: WINUSB_INTERFACE_HANDLE, PolicyType: ULONG,
        ValueLength: PULONG, Value: PVOID,
    ) -> BOOL;
    pub fn WinUsb_GetOverlappedResult(
        InterfaceHandle: WINUSB_INTERFACE_HANDLE, lpOverlapped: LPOVERLAPPED,
        lpNumberOfBytesTransferred: LPDWORD, bWait: BOOL,
    ) -> BOOL;
    pub fn WinUsb_ParseConfigurationDescriptor(
        ConfigurationDescriptor: PUSB_CONFIGURATION_DESCRIPTOR, StartPosition: PVOID,
        InterfaceNumber: LONG, AlternateSetting: LONG,
        InterfaceClass: LONG, InterfaceSubClass: LONG, InterfaceProtocol: LONG,
    ) -> BOOL;
    pub fn WinUsb_ParseDescriptors(
        DescriptorBuffer: PVOID, TotalLength: ULONG,
        StartPosition: PVOID, DescriptorType: LONG,
    ) -> BOOL;
    pub fn WinUsb_GetCurrentFrameNumber(
        InterfaceHandle: WINUSB_INTERFACE_HANDLE, CurrentFrameNumber: PULONG,
        TimeStamp: *mut LARGE_INTEGER, 
    ) -> BOOL;
    pub fn WinUsb_GetAdjustedFrameNumber(
        CurrentFrameNumber: PULONG,
        TimeStamp: LARGE_INTEGER, 
    ) -> BOOL;
    pub fn WinUsb_RegisterIsochBuffer(
        InterfaceHandle: WINUSB_INTERFACE_HANDLE, PipeID: UCHAR,
        Buffer: PUCHAR, BufferLength: ULONG, IsochBufferHandle: PWINUSB_ISOCH_BUFFER_HANDLE,
    ) -> BOOL;
    pub fn WinUsb_UnregisterIsochBuffer(
        IsochBufferHandle: WINUSB_ISOCH_BUFFER_HANDLE,
    ) -> BOOL;
    pub fn WinUsb_WriteIsochPipe(
        BufferHandle: WINUSB_ISOCH_BUFFER_HANDLE, Offset: ULONG,
        Length: ULONG, FrameNumber: PULONG, Overlapped: LPOVERLAPPED,
    ) -> BOOL;
    pub fn WinUsb_ReadIsochPipe(
        BufferHandle: WINUSB_ISOCH_BUFFER_HANDLE, Offset: ULONG,
        Length: ULONG, FrameNumber: PULONG, NumberOfPackets: ULONG,
        IsoPacketDescriptors: PUSBD_ISO_PACKET_DESCRIPTOR, Overlapped: LPOVERLAPPED,
    ) -> BOOL;
    pub fn WinUsb_WriteIsochPipeAsap(
        BufferHandle: WINUSB_ISOCH_BUFFER_HANDLE, Offset: ULONG,
        Length: ULONG, ContinueStream: BOOL, Overlapped: LPOVERLAPPED,
    ) -> BOOL;
    pub fn WinUsb_ReadIsochPipeAsap(
        BufferHandle: WINUSB_ISOCH_BUFFER_HANDLE, Offset: ULONG,
        Length: ULONG, ContinueStream: BOOL, NumberOfPackets: ULONG,
        IsoPacketDescriptors: PUSBD_ISO_PACKET_DESCRIPTOR, Overlapped: LPOVERLAPPED,
    ) -> BOOL;
}

// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::fltuserstructures::{FILTER_INFORMATION_CLASS, FILTER_VOLUME_INFORMATION_CLASS, HFILTER, HFILTER_INSTANCE, INSTANCE_INFORMATION_CLASS, PFILTER_MESSAGE_HEADER, PFILTER_REPLY_HEADER};
use shared::minwindef::{DWORD, LPCVOID, LPDWORD, LPHANDLE, LPVOID, WORD};
use um::minwinbase::{LPOVERLAPPED, LPSECURITY_ATTRIBUTES};
use um::winnt::{HANDLE, HRESULT, LPCWSTR, LPWSTR, PHANDLE};


pub const FLT_PORT_FLAG_SYNC_HANDLE: DWORD = 0x00000001;

extern "system" {
    pub fn FilterAttach(
        lpFilterName: LPCWSTR,
        lpVolumeName: LPCWSTR,
        lpInstanceName: LPCWSTR,
        dwCreatedInstanceNameLength: DWORD,
        lpCreatedInstanceName: LPWSTR
    ) -> HRESULT;
    pub fn FilterAttachAtAltitude(
        lpFilterName: LPCWSTR,
        lpVolumeName: LPCWSTR,
        lpAltitude: LPCWSTR,
        lpInstanceName: LPCWSTR,
        dwCreatedInstanceNameLength: DWORD,
        lpCreatedInstanceName: LPWSTR
    ) -> HRESULT;
    pub fn FilterClose(
        hFilter: HFILTER
    ) -> HRESULT;
    pub fn FilterConnectCommunicationPort(
        lpPortName: LPCWSTR,
        dwOptions: DWORD,
        lpContext: LPCVOID,
        wSizeOfContext: WORD,
        lpSecurityAttributes: LPSECURITY_ATTRIBUTES,
        hPort: PHANDLE
    ) -> HRESULT;
    pub fn FilterCreate(
        lpFilterName: LPCWSTR,
        hFilter: *mut HFILTER
    ) -> HRESULT;
    pub fn FilterDetach(
        lpFilterName: LPCWSTR,
        lpVolumeName: LPCWSTR,
        lpInstanceName: LPCWSTR
    ) -> HRESULT;
    pub fn FilterFindClose(
        hFilterFind: HANDLE
    ) -> HRESULT;
    pub fn FilterFindFirst(
        dwInformationClass: FILTER_INFORMATION_CLASS,
        lpBuffer: LPVOID,
        dwBufferSize: DWORD,
        lpBytesReturned: LPDWORD,
        lpFilterFind: LPHANDLE
    ) -> HRESULT;
    pub fn FilterFindNext(
        hFilterFind: HANDLE,
        dwInformationClass: FILTER_INFORMATION_CLASS,
        lpBuffer: LPVOID,
        dwBufferSize: DWORD,
        lpBytesReturned: LPDWORD
    ) -> HRESULT;
    pub fn FilterGetDosName(
        lpVolumeName: LPCWSTR,
        lpDosName: LPWSTR,
        dwDosNameBufferSize: DWORD
    ) -> HRESULT;
    pub fn FilterGetInformation(
        hFilter: HFILTER,
        dwInformationClass: FILTER_INFORMATION_CLASS,
        lpBuffer: LPVOID,
        dwBufferSize: DWORD,
        lpBytesReturned: LPDWORD
    ) -> HRESULT;
    pub fn FilterGetMessage(
        hPort: HANDLE,
        lpMessageBuffer: PFILTER_MESSAGE_HEADER,
        dwMessageBufferSize: DWORD,
        lpOverlapped: LPOVERLAPPED
    ) -> HRESULT;
    pub fn FilterInstanceClose(
        hInstance: HFILTER_INSTANCE
    ) -> HRESULT;
    pub fn FilterInstanceCreate(
        lpFilterName: LPCWSTR,
        lpVolumeName: LPCWSTR,
        lpInstanceName: LPCWSTR,
        hInstance: *mut HFILTER_INSTANCE
    ) -> HRESULT;
    pub fn FilterInstanceFindClose(
        hFilterInstanceFind: HANDLE
    ) -> HRESULT;
    pub fn FilterInstanceFindFirst(
        lpFilterName: LPCWSTR,
        dwInformationClass: INSTANCE_INFORMATION_CLASS,
        lpBuffer: LPVOID,
        dwBufferSize: DWORD,
        lpBytesReturned: LPDWORD,
        lpFilterInstanceFind: LPHANDLE
    ) -> HRESULT;
    pub fn FilterInstanceFindNext(
        hFilterInstanceFind: HANDLE,
        dwInformationClass: INSTANCE_INFORMATION_CLASS,
        lpBuffer: LPVOID,
        dwBufferSize: DWORD,
        lpBytesReturned: LPDWORD
    ) -> HRESULT;
    pub fn FilterInstanceGetInformation(
        hInstance: HFILTER_INSTANCE,
        dwInformationClass: INSTANCE_INFORMATION_CLASS,
        lpBuffer: LPVOID,
        dwBufferSize: DWORD,
        lpBytesReturned: LPDWORD
    ) -> HRESULT;
    pub fn FilterLoad(
        lpFilterName: LPCWSTR
    ) -> HRESULT;
    pub fn FilterReplyMessage(
        hPort: HANDLE,
        lpReplyBuffer: PFILTER_REPLY_HEADER,
        dwReplyBufferSize: DWORD
    ) -> HRESULT;
    pub fn FilterSendMessage(
        hPort: HANDLE,
        lpInBuffer: LPVOID,
        dwInBufferSize: DWORD,
        lpOutBuffer: LPVOID,
        dwOutBufferSize: DWORD,
        lpBytesReturned: LPDWORD
    ) -> HRESULT;
    pub fn FilterUnload(
        lpFilterName: LPCWSTR
    ) -> HRESULT;
    pub fn FilterVolumeFindClose(
        hVolumeFind: HANDLE
    ) -> HRESULT;
    pub fn FilterVolumeFindFirst(
        dwInformationClass: FILTER_VOLUME_INFORMATION_CLASS,
        lpBuffer: LPVOID,
        dwBufferSize: DWORD,
        lpBytesReturned: LPDWORD,
        lpVolumeFind: PHANDLE
    ) -> HRESULT;
    pub fn FilterVolumeFindNext(
        hVolumeFind: HANDLE,
        dwInformationClass: FILTER_VOLUME_INFORMATION_CLASS,
        lpBuffer: LPVOID,
        dwBufferSize: DWORD,
        lpBytesReturned: LPDWORD
    ) -> HRESULT;
    pub fn FilterVolumeInstanceFindClose(
        hVolumeInstanceFind: HANDLE
    ) -> HRESULT;
    pub fn FilterVolumeInstanceFindFirst(
        lpVolumeName: LPCWSTR,
        dwInformationClass: INSTANCE_INFORMATION_CLASS,
        lpBuffer: LPVOID,
        dwBufferSize: DWORD,
        lpBytesReturned: LPDWORD,
        lpVolumeInstanceFind: LPHANDLE
    ) -> HRESULT;
    pub fn FilterVolumeInstanceFindNext(
        hVolumeInstanceFind: HANDLE,
        dwInformationClass: INSTANCE_INFORMATION_CLASS,
        lpBuffer: LPVOID,
        dwBufferSize: DWORD,
        lpBytesReturned: LPDWORD
    ) -> HRESULT;
}

// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to winspool.
#![cfg(windows)]
extern crate winapi;
use winapi::*;
extern "system" {
    pub fn AbortPrinter(hPrinter: HANDLE) -> BOOL;
    pub fn AddFormA(hPrinter: HANDLE, Level: DWORD, pForm: LPBYTE) -> BOOL;
    pub fn AddFormW(hPrinter: HANDLE, Level: DWORD, pForm: LPBYTE) -> BOOL;
    pub fn AddJobA(
        hPrinter: HANDLE, Level: DWORD, pData: LPBYTE, cbBuf: DWORD, pcbNeeded: LPDWORD,
    ) -> BOOL;
    pub fn AddJobW(
        hPrinter: HANDLE, Level: DWORD, pData: LPBYTE, cbBuf: DWORD, pcbNeeded: LPDWORD,
    ) -> BOOL;
    pub fn AddMonitorA(pName: LPSTR, Level: DWORD, pMonitors: LPBYTE) -> BOOL;
    pub fn AddMonitorW(pName: LPWSTR, Level: DWORD, pMonitors: LPBYTE) -> BOOL;
    pub fn AddPortA(pName: LPSTR, hWnd: HWND, pMonitorName: LPSTR) -> BOOL;
    pub fn AddPortW(pName: LPWSTR, hWnd: HWND, pMonitorName: LPWSTR) -> BOOL;
    pub fn AddPrintProcessorA(
        pName: LPSTR, pEnvironment: LPSTR, pPathName: LPSTR, pPrintProcessorName: LPSTR,
    ) -> BOOL;
    pub fn AddPrintProcessorW(
        pName: LPWSTR, pEnvironment: LPWSTR, pPathName: LPWSTR, pPrintProcessorName: LPWSTR,
    ) -> BOOL;
    pub fn AddPrintProvidorA(pName: LPSTR, Level: DWORD, pProvidorInfo: LPBYTE) -> BOOL;
    pub fn AddPrintProvidorW(pName: LPWSTR, Level: DWORD, pProvidorInfo: LPBYTE) -> BOOL;
    pub fn AddPrinterA(pName: LPSTR, Level: DWORD, pPrinter: LPBYTE) -> HANDLE;
    pub fn AddPrinterW(pName: LPWSTR, Level: DWORD, pPrinter: LPBYTE) -> HANDLE;
    pub fn AddPrinterConnectionA(pName: LPSTR) -> BOOL;
    pub fn AddPrinterConnectionW(pName: LPWSTR) -> BOOL;
    pub fn AddPrinterConnection2A(
        hWnd: HWND, pszName: LPCSTR, dwLevel: DWORD, pConnectionInfo: PVOID,
    ) -> BOOL;
    pub fn AddPrinterConnection2W(
        hWnd: HWND, pszName: LPCWSTR, dwLevel: DWORD, pConnectionInfo: PVOID,
    ) -> BOOL;
    pub fn AddPrinterDriverA(pName: LPSTR, Level: DWORD, pDriverInfo: LPBYTE) -> BOOL;
    pub fn AddPrinterDriverW(pName: LPWSTR, Level: DWORD, pDriverInfo: LPBYTE) -> BOOL;
    pub fn AdvancedDocumentPropertiesA(
        hWnd: HWND, hPrinter: HANDLE, pDeviceName: LPSTR, pDevModeOutput: PDEVMODEA,
        pDevModeInput: PDEVMODEA,
    ) -> LONG;
    pub fn AdvancedDocumentPropertiesW(
        hWnd: HWND, hPrinter: HANDLE, pDeviceName: LPWSTR, pDevModeOutput: PDEVMODEW,
        pDevModeInput: PDEVMODEW,
    ) -> LONG;
    pub fn ClosePrinter(hPrinter: HANDLE) -> BOOL;
    pub fn CloseSpoolFileHandle(hPrinter: HANDLE, hSpoolFile: HANDLE) -> BOOL;
    pub fn CommitSpoolData(hPrinter: HANDLE, hSpoolFile: HANDLE, cbCommit: DWORD) -> HANDLE;
    pub fn ConfigurePortA(pName: LPSTR, hWnd: HWND, pPortName: LPSTR) -> BOOL;
    pub fn ConfigurePortW(pName: LPWSTR, hWnd: HWND, pPortName: LPWSTR) -> BOOL;
    pub fn ConnectToPrinterDlg(hwnd: HWND, Flags: DWORD) -> HANDLE;
    pub fn DeleteFormA(hPrinter: HANDLE, pFormName: LPSTR) -> BOOL;
    pub fn DeleteFormW(hPrinter: HANDLE, pFormName: LPWSTR) -> BOOL;
    pub fn DeleteMonitorA(pName: LPSTR, pEnvironment: LPSTR, pMonitorName: LPSTR) -> BOOL;
    pub fn DeleteMonitorW(pName: LPWSTR, pEnvironment: LPWSTR, pMonitorName: LPWSTR) -> BOOL;
    pub fn DeletePortA(pName: LPSTR, hWnd: HWND, pPortName: LPSTR) -> BOOL;
    pub fn DeletePortW(pName: LPWSTR, hWnd: HWND, pPortName: LPWSTR) -> BOOL;
    pub fn DeletePrintProcessorA(
        pName: LPSTR, pEnvironment: LPSTR, pPrintProcessorName: LPSTR,
    ) -> BOOL;
    pub fn DeletePrintProcessorW(
        pName: LPWSTR, pEnvironment: LPWSTR, pPrintProcessorName: LPWSTR,
    ) -> BOOL;
    pub fn DeletePrintProvidorA(
        pName: LPSTR, pEnvironment: LPSTR, pPrintProvidorName: LPSTR,
    ) -> BOOL;
    pub fn DeletePrintProvidorW(
        pName: LPWSTR, pEnvironment: LPWSTR, pPrintProvidorName: LPWSTR,
    ) -> BOOL;
    pub fn DeletePrinter(hPrinter: HANDLE) -> BOOL;
    pub fn DeletePrinterConnectionA(pName: LPSTR) -> BOOL;
    pub fn DeletePrinterConnectionW(pName: LPWSTR) -> BOOL;
    pub fn DeletePrinterDataA(hPrinter: HANDLE, pValueName: LPSTR) -> DWORD;
    pub fn DeletePrinterDataW(hPrinter: HANDLE, pValueName: LPWSTR) -> DWORD;
    pub fn DeletePrinterDataExA(hPrinter: HANDLE, pKeyName: LPCSTR, pValueName: LPCSTR) -> DWORD;
    pub fn DeletePrinterDataExW(hPrinter: HANDLE, pKeyName: LPCWSTR, pValueName: LPCWSTR) -> DWORD;
    pub fn DeletePrinterDriverA(pName: LPSTR, pEnvironment: LPSTR, pDriverName: LPSTR) -> BOOL;
    pub fn DeletePrinterDriverW(pName: LPWSTR, pEnvironment: LPWSTR, pDriverName: LPWSTR) -> BOOL;
    pub fn DeletePrinterDriverExA(
        pName: LPSTR, pEnvironment: LPSTR, pDriverName: LPSTR, dwDeleteFlag: DWORD,
        dwVersionFlag: DWORD,
    ) -> BOOL;
    pub fn DeletePrinterDriverExW(
        pName: LPWSTR, pEnvironment: LPWSTR, pDriverName: LPWSTR, dwDeleteFlag: DWORD,
        dwVersionFlag: DWORD,
    ) -> BOOL;
    pub fn DeletePrinterKeyA(hPrinter: HANDLE, pKeyName: LPCSTR) -> DWORD;
    pub fn DeletePrinterKeyW(hPrinter: HANDLE, pKeyName: LPCWSTR) -> DWORD;
    //pub fn DevQueryPrintEx(pDQPInfo: PDEVQUERYPRINT_INFO) -> BOOL;
    pub fn DeviceCapabilitiesA(
        pDevice: LPCSTR, pPort: LPCSTR, fwCapability: WORD, pOutput: LPSTR,
        pDevMode: *const DEVMODEA,
    ) -> c_int;
    pub fn DeviceCapabilitiesW(
        pDevice: LPCWSTR, pPort: LPCWSTR, fwCapability: WORD, pOutput: LPWSTR,
        pDevMode: *const DEVMODEW,
    ) -> c_int;
    pub fn DocumentPropertiesA(
        hWnd: HWND, hPrinter: HANDLE, pDeviceName: LPSTR,
        pDevModeOutput: PDEVMODEA, pDevModeInput: PDEVMODEA, fMode: DWORD,
    ) -> LONG;
    pub fn DocumentPropertiesW(
        hWnd: HWND, hPrinter: HANDLE, pDeviceName: LPWSTR,
        pDevModeOutput: PDEVMODEW, pDevModeInput: PDEVMODEW, fMode: DWORD,
    ) -> LONG;
    pub fn EndDocPrinter(hPrinter: HANDLE) -> BOOL;
    pub fn EndPagePrinter(hPrinter: HANDLE) -> BOOL;
    pub fn EnumFormsA(
        hPrinter: HANDLE, Level: DWORD, pForm: LPBYTE, cbBuf: DWORD, pcbNeeded: LPDWORD,
        pcReturned: LPDWORD,
    ) -> BOOL;
    pub fn EnumFormsW(
        hPrinter: HANDLE, Level: DWORD, pForm: LPBYTE, cbBuf: DWORD, pcbNeeded: LPDWORD,
        pcReturned: LPDWORD,
    ) -> BOOL;
    pub fn EnumJobsA(
        hPrinter: HANDLE, FirstJob: DWORD, NoJobs: DWORD, Level: DWORD, pJob: LPBYTE, cbBuf: DWORD,
        pcbNeeded: LPDWORD, pcReturned: LPDWORD,
    ) -> BOOL;
    pub fn EnumJobsW(
        hPrinter: HANDLE, FirstJob: DWORD, NoJobs: DWORD, Level: DWORD, pJob: LPBYTE, cbBuf: DWORD,
        pcbNeeded: LPDWORD, pcReturned: LPDWORD,
    ) -> BOOL;
    pub fn EnumMonitorsA(
        pName: LPSTR, Level: DWORD, pMonitor: LPBYTE, cbBuf: DWORD, pcbNeeded: LPDWORD,
        pcReturned: LPDWORD,
    ) -> BOOL;
    pub fn EnumMonitorsW(
        pName: LPWSTR, Level: DWORD, pMonitor: LPBYTE, cbBuf: DWORD, pcbNeeded: LPDWORD,
        pcReturned: LPDWORD,
    ) -> BOOL;
    pub fn EnumPortsA(
        pName: LPSTR, Level: DWORD, pPort: LPBYTE, cbBuf: DWORD, pcbNeeded: LPDWORD,
        pcReturned: LPDWORD,
    ) -> BOOL;
    pub fn EnumPortsW(
        pName: LPWSTR, Level: DWORD, pPort: LPBYTE, cbBuf: DWORD, pcbNeeded: LPDWORD,
        pcReturned: LPDWORD,
    ) -> BOOL;
    pub fn EnumPrintProcessorDatatypesA(
        pName: LPSTR, pPrintProcessorName: LPSTR, Level: DWORD, pDatatypes: LPBYTE, cbBuf: DWORD,
        pcbNeeded: LPDWORD, pcReturned: LPDWORD,
    ) -> BOOL;
    pub fn EnumPrintProcessorDatatypesW(
        pName: LPWSTR, pPrintProcessorName: LPWSTR, Level: DWORD, pDatatypes: LPBYTE, cbBuf: DWORD,
        pcbNeeded: LPDWORD, pcReturned: LPDWORD,
    ) -> BOOL;
    pub fn EnumPrintProcessorsA(
        pName: LPSTR, pEnvironment: LPSTR, Level: DWORD, pPrintProcessorInfo: LPBYTE, cbBuf: DWORD,
        pcbNeeded: LPDWORD, pcReturned: LPDWORD,
    ) -> BOOL;
    pub fn EnumPrintProcessorsW(
        pName: LPWSTR, pEnvironment: LPWSTR, Level: DWORD, pPrintProcessorInfo: LPBYTE,
        cbBuf: DWORD, pcbNeeded: LPDWORD, pcReturned: LPDWORD,
    ) -> BOOL;
    pub fn EnumPrinterDataA(
        hPrinter: HANDLE, dwIndex: DWORD, pValueName: LPSTR, cbValueName: DWORD,
        pcbValueName: LPDWORD, pType: LPDWORD, pData: LPBYTE, cbData: DWORD, pcbData: LPDWORD,
    ) -> DWORD;
    pub fn EnumPrinterDataW(
        hPrinter: HANDLE, dwIndex: DWORD, pValueName: LPWSTR, cbValueName: DWORD,
        pcbValueName: LPDWORD, pType: LPDWORD, pData: LPBYTE, cbData: DWORD, pcbData: LPDWORD,
    ) -> DWORD;
    pub fn EnumPrinterDataExA(
        hPrinter: HANDLE, pKeyName: LPCSTR, pEnumValues: LPBYTE, cbEnumValues: DWORD,
        pcbEnumValues: LPDWORD, pnEnumValues: LPDWORD,
    ) -> DWORD;
    pub fn EnumPrinterDataExW(
        hPrinter: HANDLE, pKeyName: LPCWSTR, pEnumValues: LPBYTE, cbEnumValues: DWORD,
        pcbEnumValues: LPDWORD, pnEnumValues: LPDWORD,
    ) -> DWORD;
    pub fn EnumPrinterDriversA(
        pName: LPSTR, pEnvironment: LPSTR, Level: DWORD, pDriverInfo: LPBYTE, cbBuf: DWORD,
        pcbNeeded: LPDWORD, pcReturned: LPDWORD,
    ) -> BOOL;
    pub fn EnumPrinterDriversW(
        pName: LPWSTR, pEnvironment: LPWSTR, Level: DWORD, pDriverInfo: LPBYTE, cbBuf: DWORD,
        pcbNeeded: LPDWORD, pcReturned: LPDWORD,
    ) -> BOOL;
    pub fn EnumPrinterKeyA(
        hPrinter: HANDLE, pKeyName: LPCSTR, pSubKey: LPSTR, cbSubkey: DWORD, pcbSubkey: LPDWORD,
    ) -> DWORD;
    pub fn EnumPrinterKeyW(
        hPrinter: HANDLE, pKeyName: LPCWSTR, pSubKey: LPWSTR, cbSubkey: DWORD, pcbSubkey: LPDWORD,
    ) -> DWORD;
    pub fn EnumPrintersA(
        Flags: DWORD, Name: LPSTR, Level: DWORD, pPrinterEnum: LPBYTE, cbBuf: DWORD,
        pcbNeeded: LPDWORD, pcReturned: LPDWORD,
    ) -> BOOL;
    pub fn EnumPrintersW(
        Flags: DWORD, Name: LPWSTR, Level: DWORD, pPrinterEnum: LPBYTE, cbBuf: DWORD,
        pcbNeeded: LPDWORD, pcReturned: LPDWORD,
    ) -> BOOL;
    pub fn ExtDeviceMode(
        hWnd: HWND, hInst: HANDLE, pDevModeOutput: LPDEVMODEA, pDeviceName: LPSTR,
        pPort: LPSTR, pDevModeInput: LPDEVMODEA, pProfile: LPSTR, fMode: DWORD,
    ) -> LONG;
    pub fn FindClosePrinterChangeNotification(hChange: HANDLE) -> BOOL;
    pub fn FindFirstPrinterChangeNotification(
        hPrinter: HANDLE, fdwFilter: DWORD, fdwOptions: DWORD, pPrinterNotifyOptions: LPVOID,
    ) -> HANDLE;
    pub fn FindNextPrinterChangeNotification(
        hChange: HANDLE, pdwChange: PDWORD, pPrinterNotifyOptions: LPVOID,
        ppPrinterNotifyInfo: *mut LPVOID,
    ) -> BOOL;
    pub fn FlushPrinter(
        hPrinter: HANDLE, pBuf: LPVOID, cbBuf: DWORD, pcWritten: LPDWORD, cSleep: DWORD,
    ) -> BOOL;
    pub fn GetDefaultPrinterA(pszBuffer: LPSTR, pcchBuffer: LPDWORD) -> BOOL;
    pub fn GetDefaultPrinterW(pszBuffer: LPWSTR, pcchBuffer: LPDWORD) -> BOOL;
    pub fn GetFormA(
        hPrinter: HANDLE, pFormName: LPSTR, Level: DWORD, pForm: LPBYTE, cbBuf: DWORD,
        pcbNeeded: LPDWORD,
    ) -> BOOL;
    pub fn GetFormW(
        hPrinter: HANDLE, pFormName: LPWSTR, Level: DWORD, pForm: LPBYTE, cbBuf: DWORD,
        pcbNeeded: LPDWORD,
    ) -> BOOL;
    pub fn GetJobA(
        hPrinter: HANDLE, JobId: DWORD, Level: DWORD, pJob: LPBYTE, cbBuf: DWORD,
        pcbNeeded: LPDWORD,
    ) -> BOOL;
    pub fn GetJobW(
        hPrinter: HANDLE, JobId: DWORD, Level: DWORD, pJob: LPBYTE, cbBuf: DWORD,
        pcbNeeded: LPDWORD,
    ) -> BOOL;
    pub fn GetPrintProcessorDirectoryA(
        pName: LPSTR, pEnvironment: LPSTR, Level: DWORD, pPrintProcessorInfo: LPBYTE, cbBuf: DWORD,
        pcbNeeded: LPDWORD,
    ) -> BOOL;
    pub fn GetPrintProcessorDirectoryW(
        pName: LPWSTR, pEnvironment: LPWSTR, Level: DWORD, pPrintProcessorInfo: LPBYTE,
        cbBuf: DWORD, pcbNeeded: LPDWORD,
    ) -> BOOL;
    pub fn GetPrinterA(
        hPrinter: HANDLE, Level: DWORD, pPrinter: LPBYTE, cbBuf: DWORD, pcbNeeded: LPDWORD,
    ) -> BOOL;
    pub fn GetPrinterW(
        hPrinter: HANDLE, Level: DWORD, pPrinter: LPBYTE, cbBuf: DWORD, pcbNeeded: LPDWORD,
    ) -> BOOL;
    pub fn GetPrinterDataA(
        hPrinter: HANDLE, pValueName: LPSTR, pType: LPDWORD, pData: LPBYTE, nSize: DWORD,
        pcbNeeded: LPDWORD,
    ) -> DWORD;
    pub fn GetPrinterDataW(
        hPrinter: HANDLE, pValueName: LPWSTR, pType: LPDWORD, pData: LPBYTE, nSize: DWORD,
        pcbNeeded: LPDWORD,
    ) -> DWORD;
    pub fn GetPrinterDataExA(
        hPrinter: HANDLE, pKeyName: LPCSTR, pValueName: LPCSTR, pType: LPDWORD, pData: LPBYTE,
        nSize: DWORD, pcbNeeded: LPDWORD,
    ) -> DWORD;
    pub fn GetPrinterDataExW(
        hPrinter: HANDLE, pKeyName: LPCWSTR, pValueName: LPCWSTR, pType: LPDWORD, pData: LPBYTE,
        nSize: DWORD, pcbNeeded: LPDWORD,
    ) -> DWORD;
    pub fn GetPrinterDriverA(
        hPrinter: HANDLE, pEnvironment: LPSTR, Level: DWORD, pDriverInfo: LPBYTE, cbBuf: DWORD,
        pcbNeeded: LPDWORD,
    ) -> BOOL;
    pub fn GetPrinterDriverW(
        hPrinter: HANDLE, pEnvironment: LPWSTR, Level: DWORD, pDriverInfo: LPBYTE, cbBuf: DWORD,
        pcbNeeded: LPDWORD,
    ) -> BOOL;
    pub fn GetPrinterDriverDirectoryA(
        pName: LPSTR, pEnvironment: LPSTR, Level: DWORD, pDriverDirectory: LPBYTE, cbBuf: DWORD,
        pcbNeeded: LPDWORD,
    ) -> BOOL;
    pub fn GetPrinterDriverDirectoryW(
        pName: LPWSTR, pEnvironment: LPWSTR, Level: DWORD, pDriverDirectory: LPBYTE, cbBuf: DWORD,
        pcbNeeded: LPDWORD,
    ) -> BOOL;
    pub fn GetSpoolFileHandle(hPrinter: HANDLE) -> HANDLE;
    pub fn IsValidDevmodeA(pDevmode: PDEVMODEA, DevmodeSize: size_t) -> BOOL;
    pub fn IsValidDevmodeW(pDevmode: PDEVMODEW, DevmodeSize: size_t) -> BOOL;
    pub fn OpenPrinterA(
        pPrinterName: LPSTR, phPrinter: LPHANDLE, pDefault: LPPRINTER_DEFAULTSA,
    ) -> BOOL;
    pub fn OpenPrinterW(
        pPrinterName: LPWSTR, phPrinter: LPHANDLE, pDefault: LPPRINTER_DEFAULTSW,
    ) -> BOOL;
    pub fn PrinterMessageBoxA(
        hPrinter: HANDLE, Error: DWORD, hWnd: HWND, pText: LPSTR, pCaption: LPSTR, dwType: DWORD,
    ) -> DWORD;
    pub fn PrinterMessageBoxW(
        hPrinter: HANDLE, Error: DWORD, hWnd: HWND, pText: LPWSTR, pCaption: LPWSTR, dwType: DWORD,
    ) -> DWORD;
    pub fn PrinterProperties(hWnd: HWND, hPrinter: HANDLE) -> BOOL;
    pub fn ReadPrinter(
        hPrinter: HANDLE, pBuf: LPVOID, cbBuf: DWORD, pNoBytesRead: LPDWORD,
    ) -> BOOL;
    pub fn ResetPrinterA(hPrinter: HANDLE, pDefault: LPPRINTER_DEFAULTSA) -> BOOL;
    pub fn ResetPrinterW(hPrinter: HANDLE, pDefault: LPPRINTER_DEFAULTSW) -> BOOL;
    pub fn ScheduleJob(hPrinter: HANDLE, JobId: DWORD) -> BOOL;
    pub fn SetDefaultPrinterA(pszPrinter: LPCSTR) -> BOOL;
    pub fn SetDefaultPrinterW(pszPrinter: LPCWSTR) -> BOOL;
    pub fn SetFormA(hPrinter: HANDLE, pFormName: LPSTR, Level: DWORD, pForm: LPBYTE) -> BOOL;
    pub fn SetFormW(hPrinter: HANDLE, pFormName: LPWSTR, Level: DWORD, pForm: LPBYTE) -> BOOL;
    pub fn SetJobA(
        hPrinter: HANDLE, JobId: DWORD, Level: DWORD, pJob: LPBYTE, Command: DWORD,
    ) -> BOOL;
    pub fn SetJobW(
        hPrinter: HANDLE, JobId: DWORD, Level: DWORD, pJob: LPBYTE, Command: DWORD,
    ) -> BOOL;
    pub fn SetPortA(pName: LPSTR, pPortName: LPSTR, dwLevel: DWORD, pPortInfo: LPBYTE) -> BOOL;
    pub fn SetPortW(pName: LPWSTR, pPortName: LPWSTR, dwLevel: DWORD, pPortInfo: LPBYTE) -> BOOL;
    pub fn SetPrinterA(hPrinter: HANDLE, Level: DWORD, pPrinter: LPBYTE, Command: DWORD) -> BOOL;
    pub fn SetPrinterW(hPrinter: HANDLE, Level: DWORD, pPrinter: LPBYTE, Command: DWORD) -> BOOL;
    pub fn SetPrinterDataA(
        hPrinter: HANDLE, pValueName: LPSTR, Type: DWORD, pData: LPBYTE, cbData: DWORD,
    ) -> DWORD;
    pub fn SetPrinterDataW(
        hPrinter: HANDLE, pValueName: LPWSTR, Type: DWORD, pData: LPBYTE, cbData: DWORD,
    ) -> DWORD;
    pub fn StartDocPrinterA(hPrinter: HANDLE, Level: DWORD, pDocInfo: LPBYTE) -> DWORD;
    pub fn StartDocPrinterW(hPrinter: HANDLE, Level: DWORD, pDocInfo: LPBYTE) -> DWORD;
    pub fn StartPagePrinter(hPrinter: HANDLE) -> BOOL;
    pub fn WaitForPrinterChange(hPrinter: HANDLE, Flags: DWORD) -> DWORD;
    pub fn WritePrinter(hPrinter: HANDLE, pBuf: LPVOID, cbBuf: DWORD, pcWritten: LPDWORD) -> BOOL;
    pub fn XcvDataW(
        hXcv: HANDLE, pszDataName: PCWSTR, pInputData: PBYTE, cbInputData: DWORD,
        pOutputData: PBYTE, cbOutputData: DWORD, pcbOutputNeeded: PDWORD, pdwStatus: PDWORD,
    ) -> BOOL;
}

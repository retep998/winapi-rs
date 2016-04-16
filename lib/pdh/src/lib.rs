// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to pdh.
#![cfg(windows)]
extern crate winapi;
use winapi::*;
extern "system" {
    pub fn PdhGetDllVersion(lpdwVersion: LPDWORD) -> PDH_STATUS;
    pub fn PdhOpenQueryW(
        szDataSource: LPCWSTR, dwUserData: DWORD_PTR, phQuery: *mut PDH_HQUERY,
    ) -> PDH_STATUS;
    pub fn PdhOpenQueryA(
        szDataSource: LPCSTR, dwUserData: DWORD_PTR, phQuery: *mut PDH_HQUERY,
    ) -> PDH_STATUS;
    pub fn PdhAddCounterW(
        hQuery: PDH_HQUERY, szFullCounterPath: LPCWSTR, dwUserData: DWORD_PTR, 
        phCounter: *mut PDH_HCOUNTER,
    ) -> PDH_STATUS;
    pub fn PdhAddCounterA(
        hQuery: PDH_HQUERY, szFullCounterPath: LPCSTR, dwUserData: DWORD_PTR, 
        phCounter: *mut PDH_HCOUNTER,
    ) -> PDH_STATUS;
    // pub fn PdhAddEnglishCounterW();
    // pub fn PdhAddEnglishCounterA();
    // pub fn PdhCollectQueryDataWithTime();
    // pub fn PdhValidatePathExW();
    // pub fn PdhValidatePathExA();
    pub fn PdhRemoveCounter(hCounter: PDH_HCOUNTER) -> PDH_STATUS;
    pub fn PdhCollectQueryData(hQuery: PDH_HQUERY) -> PDH_STATUS;
    pub fn PdhCloseQuery(hQuery: PDH_HQUERY) -> PDH_STATUS;
    pub fn PdhGetFormattedCounterValue(
        hCounter: PDH_HCOUNTER, dwFormat: DWORD, lpdwType: LPDWORD, pValue: PPDH_FMT_COUNTERVALUE,
    ) -> PDH_STATUS;
    // pub fn PdhGetFormattedCounterArrayA();
    // pub fn PdhGetFormattedCounterArrayW();
    // pub fn PdhGetRawCounterValue();
    // pub fn PdhGetRawCounterArrayA();
    // pub fn PdhGetRawCounterArrayW();
    // pub fn PdhCalculateCounterFromRawValue();
    // pub fn PdhComputeCounterStatistics();
    // pub fn PdhGetCounterInfoW();
    // pub fn PdhGetCounterInfoA();
    // pub fn PdhSetCounterScaleFactor();
    // pub fn PdhConnectMachineW();
    // pub fn PdhConnectMachineA();
    // pub fn PdhEnumMachinesW();
    // pub fn PdhEnumMachinesA();
    // pub fn PdhEnumObjectsW();
    // pub fn PdhEnumObjectsA();
    // pub fn PdhEnumObjectItemsW();
    // pub fn PdhEnumObjectItemsA();
    pub fn PdhMakeCounterPathW(
        pCounterPathElements: PPDH_COUNTER_PATH_ELEMENTS_W, szFullPathBuffer: LPWSTR, 
        pcchBufferSize: LPDWORD, dwFlags: DWORD,
    ) -> PDH_STATUS;
    pub fn PdhMakeCounterPathA(
        pCounterPathElements: PPDH_COUNTER_PATH_ELEMENTS_A, szFullPathBuffer: LPSTR, 
        pcchBufferSize: LPDWORD, dwFlags: DWORD,
    ) -> PDH_STATUS;
    // pub fn PdhMakeCounterPathA();
    // pub fn PdhParseCounterPathW();
    // pub fn PdhParseCounterPathA();
    // pub fn PdhParseInstanceNameW();
    // pub fn PdhParseInstanceNameA();
    // pub fn PdhValidatePathW();
    // pub fn PdhValidatePathA();
    // pub fn PdhGetDefaultPerfObjectW();
    // pub fn PdhGetDefaultPerfObjectA();
    // pub fn PdhGetDefaultPerfCounterW();
    // pub fn PdhGetDefaultPerfCounterA();
    // pub fn PdhBrowseCountersW();
    // pub fn PdhBrowseCountersA();
    // pub fn PdhExpandCounterPathW();
    // pub fn PdhExpandCounterPathA();
    // pub fn PdhLookupPerfNameByIndexW();
    // pub fn PdhLookupPerfNameByIndexA();
    // pub fn PdhLookupPerfIndexByNameW();
    // pub fn PdhLookupPerfIndexByNameA();
    // pub fn PdhExpandWildCardPathA();
    // pub fn PdhExpandWildCardPathW();
    // pub fn PdhOpenLogW();
    // pub fn PdhOpenLogA();
    // pub fn PdhUpdateLogW();
    // pub fn PdhUpdateLogA();
    // pub fn PdhUpdateLogFileCatalog();
    // pub fn PdhGetLogFileSize();
    // pub fn PdhCloseLog();
    // pub fn PdhSelectDataSourceW();
    // pub fn PdhSelectDataSourceA();
    // pub fn PdhIsRealTimeQuery();
    // pub fn PdhSetQueryTimeRange();
    // pub fn PdhGetDataSourceTimeRangeW();
    // pub fn PdhGetDataSourceTimeRangeA();
    // pub fn PdhCollectQueryDataEx();
    // pub fn PdhFormatFromRawValue();
    // pub fn PdhGetCounterTimeBase();
    // pub fn PdhReadRawLogRecord();
    // pub fn PdhSetDefaultRealTimeDataSource();
    // pub fn PdhBindInputDataSourceW();
    // pub fn PdhBindInputDataSourceA();
    // pub fn PdhOpenQueryH();
    // pub fn PdhEnumMachinesHW();
    // pub fn PdhEnumMachinesHA();
    // pub fn PdhEnumObjectsHW();
    // pub fn PdhEnumObjectsHA();
    // pub fn PdhEnumObjectItemsHW();
    // pub fn PdhEnumObjectItemsHA();
    // pub fn PdhExpandWildCardPathHW();
    // pub fn PdhExpandWildCardPathHA();
    // pub fn PdhGetDataSourceTimeRangeH();
    // pub fn PdhGetDefaultPerfObjectHW();
    // pub fn PdhGetDefaultPerfObjectHA();
    // pub fn PdhGetDefaultPerfCounterHW();
    // pub fn PdhGetDefaultPerfCounterHA();
    // pub fn PdhBrowseCountersHW();
    // pub fn PdhBrowseCountersHA();
    // pub fn PdhVerifySQLDBW();
    // pub fn PdhVerifySQLDBA();
    // pub fn PdhCreateSQLTablesW();
    // pub fn PdhCreateSQLTablesA();
    // pub fn PdhEnumLogSetNamesW();
    // pub fn PdhEnumLogSetNamesA();
    // pub fn PdhGetLogSetGUID();
    // pub fn PdhSetLogSetRunID();
}

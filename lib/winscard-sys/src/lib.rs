// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to winscard.
#![cfg(windows)]
extern crate winapi;
use winapi::*;
extern "system" {
    pub fn SCardAccessStartedEvent() -> HANDLE;
    pub fn SCardAddReaderToGroupA(
        hContext: SCARDCONTEXT, szReaderName: LPCSTR, szGroupName: LPCSTR,
    ) -> LONG;
    pub fn SCardAddReaderToGroupW(
        hContext: SCARDCONTEXT, szReaderName: LPCWSTR, szGroupName: LPCWSTR,
    ) -> LONG;
    pub fn SCardAudit(hContext: SCARDCONTEXT, dwEvent: DWORD) -> LONG;
    pub fn SCardBeginTransaction(hCard: SCARDHANDLE) -> LONG;
    pub fn SCardCancel(hContext: SCARDCONTEXT) -> LONG;
    pub fn SCardConnectA(
        hContext: SCARDCONTEXT, szReader: LPCSTR, dwShareMode: DWORD, dwPreferredProtocols: DWORD,
        phCard: LPSCARDHANDLE, pdwActiveProtocol: LPDWORD,
    ) -> LONG;
    pub fn SCardConnectW(
        hContext: SCARDCONTEXT, szReader: LPCWSTR, dwShareMode: DWORD, dwPreferredProtocols: DWORD,
        phCard: LPSCARDHANDLE, pdwActiveProtocol: LPDWORD,
    ) -> LONG;
    pub fn SCardControl(
        hCard: SCARDHANDLE, dwControlCode: DWORD, lpInBuffer: LPCVOID, cbInBufferSize: DWORD,
        lpOutBuffer: LPVOID, cbOutBufferSize: DWORD, lpBytesReturned: LPDWORD,
    ) -> LONG;
    pub fn SCardDisconnect(hCard: SCARDHANDLE, dwDisposition: DWORD) -> LONG;
    pub fn SCardEndTransaction(hCard: SCARDHANDLE, dwDisposition: DWORD) -> LONG;
    pub fn SCardEstablishContext(
        dwScope: DWORD, pvReserved1: LPCVOID, pvReserved2: LPCVOID, phContext: LPSCARDCONTEXT,
    ) -> LONG;
    pub fn SCardForgetCardTypeA(hContext: SCARDCONTEXT, szCardName: LPCSTR) -> LONG;
    pub fn SCardForgetCardTypeW(hContext: SCARDCONTEXT, szCardName: LPCWSTR) -> LONG;
    pub fn SCardForgetReaderA(hContext: SCARDCONTEXT, szReaderName: LPCSTR) -> LONG;
    pub fn SCardForgetReaderGroupA(hContext: SCARDCONTEXT, szGroupName: LPCSTR) -> LONG;
    pub fn SCardForgetReaderGroupW(hContext: SCARDCONTEXT, szGroupName: LPCWSTR) -> LONG;
    pub fn SCardForgetReaderW(hContext: SCARDCONTEXT, szReaderName: LPCWSTR) -> LONG;
    pub fn SCardFreeMemory(hContext: SCARDCONTEXT, pvMem: LPCVOID) -> LONG;
    pub fn SCardGetAttrib(
        hCard: SCARDHANDLE, dwAttrId: DWORD, pbAttr: LPBYTE, pcbAttrLen: LPDWORD,
    ) -> LONG;
    pub fn SCardGetCardTypeProviderNameA(
        hContext: SCARDCONTEXT, szCardName: LPCSTR, dwProviderId: DWORD, szProvider: *mut CHAR,
        pcchProvider: LPDWORD,
    ) -> LONG;
    pub fn SCardGetCardTypeProviderNameW(
        hContext: SCARDCONTEXT, szCardName: LPCWSTR, dwProviderId: DWORD, szProvider: *mut WCHAR,
        pcchProvider: LPDWORD,
    ) -> LONG;
    pub fn SCardGetDeviceTypeIdA(
        hContext: SCARDCONTEXT, szReaderName: LPCSTR, pdwDeviceTypeId: LPDWORD,
    ) -> LONG;
    pub fn SCardGetDeviceTypeIdW(
        hContext: SCARDCONTEXT, szReaderName: LPCWSTR, pdwDeviceTypeId: LPDWORD,
    ) -> LONG;
    pub fn SCardGetProviderIdA(
        hContext: SCARDCONTEXT, szCard: LPCSTR, pguidProviderId: LPGUID,
    ) -> LONG;
    pub fn SCardGetProviderIdW(
        hContext: SCARDCONTEXT, szCard: LPCWSTR, pguidProviderId: LPGUID,
    ) -> LONG;
    pub fn SCardGetReaderDeviceInstanceIdA(
        hContext: SCARDCONTEXT, szReaderName: LPCSTR, szDeviceInstanceId: LPSTR,
        pcchDeviceInstanceId: LPDWORD,
    ) -> LONG;
    pub fn SCardGetReaderDeviceInstanceIdW(
        hContext: SCARDCONTEXT, szReaderName: LPCWSTR, szDeviceInstanceId: LPWSTR,
        pcchDeviceInstanceId: LPDWORD,
    ) -> LONG;
    pub fn SCardGetReaderIconA(
        hContext: SCARDCONTEXT, szReaderName: LPCSTR, pbIcon: LPBYTE, pcbIcon: LPDWORD,
    ) -> LONG;
    pub fn SCardGetReaderIconW(
        hContext: SCARDCONTEXT, szReaderName: LPCWSTR, pbIcon: LPBYTE, pcbIcon: LPDWORD,
    ) -> LONG;
    pub fn SCardGetStatusChangeA(
        hContext: SCARDCONTEXT, dwTimeout: DWORD, rgReaderStates: LPSCARD_READERSTATEA,
        cReaders: DWORD,
    ) -> LONG;
    pub fn SCardGetStatusChangeW(
        hContext: SCARDCONTEXT, dwTimeout: DWORD, rgReaderStates: LPSCARD_READERSTATEW,
        cReaders: DWORD,
    ) -> LONG;
    pub fn SCardGetTransmitCount(hCard: SCARDHANDLE, pcTransmitCount: LPDWORD) -> LONG;
    pub fn SCardIntroduceCardTypeA(
        hContext: SCARDCONTEXT, szCardName: LPCSTR, pguidPrimaryProvider: LPCGUID,
        rgguidInterfaces: LPCGUID, dwInterfaceCount: DWORD, pbAtr: LPCBYTE, pbAtrMask: LPCBYTE,
        cbAtrLen: DWORD,
    ) -> LONG;
    pub fn SCardIntroduceCardTypeW(
        hContext: SCARDCONTEXT, szCardName: LPCWSTR, pguidPrimaryProvider: LPCGUID,
        rgguidInterfaces: LPCGUID, dwInterfaceCount: DWORD, pbAtr: LPCBYTE, pbAtrMask: LPCBYTE,
        cbAtrLen: DWORD,
    ) -> LONG;
    pub fn SCardIntroduceReaderA(
        hContext: SCARDCONTEXT, szReaderName: LPCSTR, szDeviceName: LPCSTR,
    ) -> LONG;
    pub fn SCardIntroduceReaderGroupA(hContext: SCARDCONTEXT, szGroupName: LPCSTR) -> LONG;
    pub fn SCardIntroduceReaderGroupW(hContext: SCARDCONTEXT, szGroupName: LPCWSTR) -> LONG;
    pub fn SCardIntroduceReaderW(
        hContext: SCARDCONTEXT, szReaderName: LPCWSTR, szDeviceName: LPCWSTR,
    ) -> LONG;
    pub fn SCardIsValidContext(hContext: SCARDCONTEXT) -> LONG;
    pub fn SCardListCardsA(
        hContext: SCARDCONTEXT, pbAtr: LPCBYTE, rgquidInterfaces: LPCGUID,
        cguidInterfaceCount: DWORD, mszCards: *mut CHAR, pcchCards: LPDWORD,
    ) -> LONG;
    pub fn SCardListCardsW(
        hContext: SCARDCONTEXT, pbAtr: LPCBYTE, rgquidInterfaces: LPCGUID,
        cguidInterfaceCount: DWORD, mszCards: *mut WCHAR, pcchCards: LPDWORD,
    ) -> LONG;
    pub fn SCardListInterfacesA(
        hContext: SCARDCONTEXT, szCard: LPCSTR, pguidInterfaces: LPGUID, pcguidInterfaces: LPDWORD,
    ) -> LONG;
    pub fn SCardListInterfacesW(
        hContext: SCARDCONTEXT, szCard: LPCWSTR, pguidInterfaces: LPGUID, pcguidInterfaces: LPDWORD,
    ) -> LONG;
    pub fn SCardListReaderGroupsA(
        hContext: SCARDCONTEXT, mszGroups: LPSTR, pcchGroups: LPDWORD,
    ) -> LONG;
    pub fn SCardListReaderGroupsW(
        hContext: SCARDCONTEXT, mszGroups: LPWSTR, pcchGroups: LPDWORD,
    ) -> LONG;
    pub fn SCardListReadersA(
        hContext: SCARDCONTEXT, mszGroups: LPCSTR, mszReaders: LPSTR, pcchReaders: LPDWORD,
    ) -> LONG;
    pub fn SCardListReadersW(
        hContext: SCARDCONTEXT, mszGroups: LPCWSTR, mszReaders: LPWSTR, pcchReaders: LPDWORD,
    ) -> LONG;
    pub fn SCardListReadersWithDeviceInstanceIdA(
        hContext: SCARDCONTEXT, szDeviceInstanceId: LPCSTR, mszReaders: LPSTR,
        pcchReaders: LPDWORD,
    ) -> LONG;
    pub fn SCardListReadersWithDeviceInstanceIdW(
        hContext: SCARDCONTEXT, szDeviceInstanceId: LPCWSTR, mszReaders: LPWSTR,
        pcchReaders: LPDWORD,
    ) -> LONG;
    pub fn SCardLocateCardsA(
        hContext: SCARDCONTEXT, mszCards: LPCSTR, rgReaderStates: LPSCARD_READERSTATEA,
        cReaders: DWORD,
    ) -> LONG;
    pub fn SCardLocateCardsByATRA(
        hContext: SCARDCONTEXT, rgAtrMasks: LPSCARD_ATRMASK, cAtrs: DWORD,
        rgReaderStates: LPSCARD_READERSTATEA, cReaders: DWORD,
    ) -> LONG;
    pub fn SCardLocateCardsByATRW(
        hContext: SCARDCONTEXT, rgAtrMasks: LPSCARD_ATRMASK, cAtrs: DWORD,
        rgReaderStates: LPSCARD_READERSTATEW, cReaders: DWORD,
    ) -> LONG;
    pub fn SCardLocateCardsW(
        hContext: SCARDCONTEXT, mszCards: LPCWSTR, rgReaderStates: LPSCARD_READERSTATEW,
        cReaders: DWORD,
    ) -> LONG;
    pub fn SCardReadCacheA(
        hContext: SCARDCONTEXT, CardIdentifier: *mut UUID, FreshnessCounter: DWORD,
        LookupName: LPSTR, Data: PBYTE, DataLen: *mut DWORD,
    ) -> LONG;
    pub fn SCardReadCacheW(
        hContext: SCARDCONTEXT, CardIdentifier: *mut UUID, FreshnessCounter: DWORD,
        LookupName: LPWSTR, Data: PBYTE, DataLen: *mut DWORD,
    ) -> LONG;
    pub fn SCardReconnect(
        hCard: SCARDHANDLE, dwShareMode: DWORD, dwPreferredProtocols: DWORD,
        dwInitialization: DWORD, pdwActiveProtocol: LPDWORD,
    ) -> LONG;
    pub fn SCardReleaseContext(hContext: SCARDCONTEXT) -> LONG;
    pub fn SCardReleaseStartedEvent();
    pub fn SCardRemoveReaderFromGroupA(
        hContext: SCARDCONTEXT, szReaderName: LPCSTR, szGroupName: LPCSTR,
    ) -> LONG;
    pub fn SCardRemoveReaderFromGroupW(
        hContext: SCARDCONTEXT, szReaderName: LPCWSTR, szGroupName: LPCWSTR,
    ) -> LONG;
    pub fn SCardSetAttrib(
        hCard: SCARDHANDLE, dwAttrId: DWORD, pbAttr: LPCBYTE, cbAttrLen: DWORD,
    ) -> LONG;
    pub fn SCardSetCardTypeProviderNameA(
        hContext: SCARDCONTEXT, szCardName: LPCSTR, dwProviderId: DWORD, szProvider: LPCSTR,
    ) -> LONG;
    pub fn SCardSetCardTypeProviderNameW(
        hContext: SCARDCONTEXT, szCardName: LPCWSTR, dwProviderId: DWORD, szProvider: LPCWSTR,
    ) -> LONG;
    pub fn SCardState(
        hCard: SCARDHANDLE, pdwState: LPDWORD, pdwProtocol: LPDWORD, pbAtr: LPBYTE,
        pcbAtrLen: LPDWORD,
    ) -> LONG;
    pub fn SCardStatusA(
        hCard: SCARDHANDLE, mszReaderNames: LPSTR, pcchReaderLen: LPDWORD, pdwState: LPDWORD,
        pdwProtocol: LPDWORD, pbAtr: LPBYTE, pcbAtrLen: LPDWORD,
    ) -> LONG;
    pub fn SCardStatusW(
        hCard: SCARDHANDLE, mszReaderNames: LPWSTR, pcchReaderLen: LPDWORD, pdwState: LPDWORD,
        pdwProtocol: LPDWORD, pbAtr: LPBYTE, pcbAtrLen: LPDWORD,
    ) -> LONG;
    pub fn SCardTransmit(
        hCard: SCARDHANDLE, pioSendPci: LPCSCARD_IO_REQUEST, pbSendBuffer: LPCBYTE,
        cbSendLength: DWORD, pioRecvPci: LPSCARD_IO_REQUEST, pbRecvBuffer: LPBYTE,
        pcbRecvLength: LPDWORD,
    ) -> LONG;
    pub fn SCardWriteCacheA(
        hContext: SCARDCONTEXT, CardIdentifier: *mut UUID, FreshnessCounter: DWORD,
        LookupName: LPSTR, Data: PBYTE, DataLen: DWORD,
    ) -> LONG;
    pub fn SCardWriteCacheW(
        hContext: SCARDCONTEXT, CardIdentifier: *mut UUID, FreshnessCounter: DWORD,
        LookupName: LPWSTR, Data: PBYTE, DataLen: DWORD,
    ) -> LONG;
}

// Copyright © 2015, skdltmxn
// Licensed under the MIT License <LICENSE.md>
#![cfg(windows)]
extern crate winscard;
use winscard::*;
#[inline(never)] fn bb<T>(_: T) {}
#[test] #[cfg(target_env = "msvc")]
fn functions_msvc() {
    bb(SCardAudit);
    bb(SCardGetDeviceTypeIdA);
    bb(SCardGetDeviceTypeIdW);
    bb(SCardGetReaderDeviceInstanceIdA);
    bb(SCardGetReaderDeviceInstanceIdW);
    bb(SCardGetReaderIconA);
    bb(SCardGetReaderIconW);
    bb(SCardListReadersWithDeviceInstanceIdA);
    bb(SCardListReadersWithDeviceInstanceIdW);
}
#[test]
fn functions() {
    bb(SCardAccessStartedEvent);
    bb(SCardAddReaderToGroupA);
    bb(SCardAddReaderToGroupW);
    bb(SCardBeginTransaction);
    bb(SCardCancel);
    bb(SCardConnectA);
    bb(SCardConnectW);
    bb(SCardControl);
    bb(SCardDisconnect);
    bb(SCardEndTransaction);
    bb(SCardEstablishContext);
    bb(SCardForgetCardTypeA);
    bb(SCardForgetCardTypeW);
    bb(SCardForgetReaderA);
    bb(SCardForgetReaderGroupA);
    bb(SCardForgetReaderGroupW);
    bb(SCardForgetReaderW);
    bb(SCardFreeMemory);
    bb(SCardGetAttrib);
    bb(SCardGetCardTypeProviderNameA);
    bb(SCardGetCardTypeProviderNameW);
    bb(SCardGetProviderIdA);
    bb(SCardGetProviderIdW);
    bb(SCardGetStatusChangeA);
    bb(SCardGetStatusChangeW);
    bb(SCardGetTransmitCount);
    bb(SCardIntroduceCardTypeA);
    bb(SCardIntroduceCardTypeW);
    bb(SCardIntroduceReaderA);
    bb(SCardIntroduceReaderGroupA);
    bb(SCardIntroduceReaderGroupW);
    bb(SCardIntroduceReaderW);
    bb(SCardIsValidContext);
    bb(SCardListCardsA);
    bb(SCardListCardsW);
    bb(SCardListInterfacesA);
    bb(SCardListInterfacesW);
    bb(SCardListReaderGroupsA);
    bb(SCardListReaderGroupsW);
    bb(SCardListReadersA);
    bb(SCardListReadersW);
    bb(SCardLocateCardsA);
    bb(SCardLocateCardsByATRA);
    bb(SCardLocateCardsByATRW);
    bb(SCardLocateCardsW);
    bb(SCardReadCacheA);
    bb(SCardReadCacheW);
    bb(SCardReconnect);
    bb(SCardReleaseContext);
    bb(SCardReleaseStartedEvent);
    bb(SCardRemoveReaderFromGroupA);
    bb(SCardRemoveReaderFromGroupW);
    bb(SCardSetAttrib);
    bb(SCardSetCardTypeProviderNameA);
    bb(SCardSetCardTypeProviderNameW);
    bb(SCardState);
    bb(SCardStatusA);
    bb(SCardStatusW);
    bb(SCardTransmit);
    bb(SCardWriteCacheA);
    bb(SCardWriteCacheW);
}

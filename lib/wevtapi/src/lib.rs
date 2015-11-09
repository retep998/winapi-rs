// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to wevtapi.
#![cfg(windows)]
extern crate winapi;
use winapi::*;
extern "system" {
    // pub fn EvtArchiveExportedLog();
    // pub fn EvtCancel();
    // pub fn EvtClearLog();
    // pub fn EvtClose();
    // pub fn EvtCreateBookmark();
    // pub fn EvtCreateRenderContext();
    // pub fn EvtExportLog();
    // pub fn EvtFormatMessage();
    // pub fn EvtGetChannelConfigProperty();
    // pub fn EvtGetEventInfo();
    // pub fn EvtGetEventMetadataProperty();
    // pub fn EvtGetExtendedStatus();
    // pub fn EvtGetLogInfo();
    // pub fn EvtGetObjectArrayProperty();
    // pub fn EvtGetObjectArraySize();
    // pub fn EvtGetPublisherMetadataProperty();
    // pub fn EvtGetQueryInfo();
    // pub fn EvtIntAssertConfig();
    // pub fn EvtIntCreateBinXMLFromCustomXML();
    // pub fn EvtIntCreateLocalLogfile();
    // pub fn EvtIntGetClassicLogDisplayName();
    // pub fn EvtIntRenderResourceEventTemplate();
    // pub fn EvtIntReportAuthzEventAndSourceAsync();
    // pub fn EvtIntReportEventAndSourceAsync();
    // pub fn EvtIntRetractConfig();
    // pub fn EvtIntWriteXmlEventToLocalLogfile();
    // pub fn EvtNext();
    // pub fn EvtNextChannelPath();
    // pub fn EvtNextEventMetadata();
    // pub fn EvtNextPublisherId();
    // pub fn EvtOpenChannelConfig();
    // pub fn EvtOpenChannelEnum();
    // pub fn EvtOpenEventMetadataEnum();
    // pub fn EvtOpenLog();
    // pub fn EvtOpenPublisherEnum();
    // pub fn EvtOpenPublisherMetadata();
    // pub fn EvtOpenSession();
    // pub fn EvtQuery();
    // pub fn EvtRender();
    // pub fn EvtSaveChannelConfig();
    // pub fn EvtSeek();
    // pub fn EvtSetChannelConfigProperty();
    // pub fn EvtSubscribe();
    // pub fn EvtUpdateBookmark();
}

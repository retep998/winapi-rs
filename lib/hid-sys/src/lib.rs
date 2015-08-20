// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to hid.
#![cfg(windows)]
extern crate winapi;
use winapi::*;
extern "system" {
    // pub fn HidD_FlushQueue();
    // pub fn HidD_FreePreparsedData();
    // pub fn HidD_GetAttributes();
    // pub fn HidD_GetConfiguration();
    // pub fn HidD_GetFeature();
    // pub fn HidD_GetHidGuid();
    // pub fn HidD_GetIndexedString();
    // pub fn HidD_GetInputReport();
    // pub fn HidD_GetManufacturerString();
    // pub fn HidD_GetMsGenreDescriptor();
    // pub fn HidD_GetNumInputBuffers();
    // pub fn HidD_GetPhysicalDescriptor();
    // pub fn HidD_GetPreparsedData();
    // pub fn HidD_GetProductString();
    // pub fn HidD_GetSerialNumberString();
    // pub fn HidD_Hello();
    // pub fn HidD_SetConfiguration();
    // pub fn HidD_SetFeature();
    // pub fn HidD_SetNumInputBuffers();
    // pub fn HidD_SetOutputReport();
    // pub fn HidP_GetButtonCaps();
    // pub fn HidP_GetCaps();
    // pub fn HidP_GetData();
    // pub fn HidP_GetExtendedAttributes();
    // pub fn HidP_GetLinkCollectionNodes();
    // pub fn HidP_GetScaledUsageValue();
    // pub fn HidP_GetSpecificButtonCaps();
    // pub fn HidP_GetSpecificValueCaps();
    // pub fn HidP_GetUsageValue();
    // pub fn HidP_GetUsageValueArray();
    // pub fn HidP_GetUsages();
    // pub fn HidP_GetUsagesEx();
    // pub fn HidP_GetValueCaps();
    // pub fn HidP_InitializeReportForID();
    // pub fn HidP_MaxDataListLength();
    // pub fn HidP_MaxUsageListLength();
    // pub fn HidP_SetData();
    // pub fn HidP_SetScaledUsageValue();
    // pub fn HidP_SetUsageValue();
    // pub fn HidP_SetUsageValueArray();
    // pub fn HidP_SetUsages();
    // pub fn HidP_TranslateUsagesToI8042ScanCodes();
    // pub fn HidP_UnsetUsages();
    // pub fn HidP_UsageListDifference();
}

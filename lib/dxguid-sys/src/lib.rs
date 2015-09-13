// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to dxguid.
#![no_std]
#![experimental]
extern crate winapi;
use winapi::*;
extern "system" {
  pub static IID_IDXGIAdapter: IID;
  pub static IID_IDXGIAdapter1: IID;
  pub static IID_IDXGIAdapter2: IID;
  pub static IID_IDXGIAdapter3: IID;
  pub static IID_IDXGIDecodeSwapChain: IID;
  pub static IID_IDXGIDevice: IID;
  pub static IID_IDXGIDevice1: IID;
  pub static IID_IDXGIDevice2: IID;
  pub static IID_IDXGIDevice3: IID;
  pub static IID_IDXGIDeviceSubObject: IID;
  pub static IID_IDXGIDisplayControl: IID;
  pub static IID_IDXGIFactory: IID;
  pub static IID_IDXGIFactory1: IID;
  pub static IID_IDXGIFactory2: IID;
  pub static IID_IDXGIFactory3: IID;
  pub static IID_IDXGIFactory4: IID;
  pub static IID_IDXGIFactoryMedia: IID;
  pub static IID_IDXGIKeyedMutex: IID;
  pub static IID_IDXGIObject: IID;
  pub static IID_IDXGIOutput: IID;
  pub static IID_IDXGIOutput1: IID;
  pub static IID_IDXGIOutput2: IID;
  pub static IID_IDXGIOutput3: IID;
  pub static IID_IDXGIOutput4: IID;
  pub static IID_IDXGIOutputDuplication: IID;
  pub static IID_IDXGIResource: IID;
  pub static IID_IDXGIResource1: IID;
  pub static IID_IDXGISurface: IID;
  pub static IID_IDXGISurface1: IID;
  pub static IID_IDXGISurface2: IID;
  pub static IID_IDXGISwapChain: IID;
  pub static IID_IDXGISwapChain1: IID;
  pub static IID_IDXGISwapChain2: IID;
  pub static IID_IDXGISwapChain3: IID;
  pub static IID_IDXGISwapChainMedia: IID;
}

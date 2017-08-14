// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to dxguid.
#![cfg(windows)]
#![allow(non_upper_case_globals)]
#[macro_use(DEFINE_GUID)] extern crate winapi;
DEFINE_GUID!{GUID_WIFI_IN_STANDBY, 0xf15576e8, 0x98b7, 0x4186, 0xb9, 0x44, 0xea, 0xfa, 0x66, 0x44, 0x02, 0xd9}
DEFINE_GUID!{IID_ID3D10Asynchronous, 0x9b7e4c0d, 0x342c, 0x4106, 0xa1, 0x9f, 0x4f, 0x27, 0x04, 0xf6, 0x89, 0xf0}
DEFINE_GUID!{IID_ID3D10BlendState1, 0xedad8d99, 0x8a35, 0x4d6d, 0x85, 0x66, 0x2e, 0xa2, 0x76, 0xcd, 0xe1, 0x61}
DEFINE_GUID!{IID_ID3D10BlendState, 0xedad8d19, 0x8a35, 0x4d6d, 0x85, 0x66, 0x2e, 0xa2, 0x76, 0xcd, 0xe1, 0x61}
DEFINE_GUID!{IID_ID3D10Buffer, 0x9b7e4c02, 0x342c, 0x4106, 0xa1, 0x9f, 0x4f, 0x27, 0x04, 0xf6, 0x89, 0xf0}
DEFINE_GUID!{IID_ID3D10Counter, 0x9b7e4c11, 0x342c, 0x4106, 0xa1, 0x9f, 0x4f, 0x27, 0x04, 0xf6, 0x89, 0xf0}
DEFINE_GUID!{IID_ID3D10DepthStencilState, 0x2b4b1cc8, 0xa4ad, 0x41f8, 0x83, 0x22, 0xca, 0x86, 0xfc, 0x3e, 0xc6, 0x75}
DEFINE_GUID!{IID_ID3D10DepthStencilView, 0x9b7e4c09, 0x342c, 0x4106, 0xa1, 0x9f, 0x4f, 0x27, 0x04, 0xf6, 0x89, 0xf0}
DEFINE_GUID!{IID_ID3D10Device1, 0x9b7e4c8f, 0x342c, 0x4106, 0xa1, 0x9f, 0x4f, 0x27, 0x04, 0xf6, 0x89, 0xf0}
DEFINE_GUID!{IID_ID3D10Device, 0x9b7e4c0f, 0x342c, 0x4106, 0xa1, 0x9f, 0x4f, 0x27, 0x04, 0xf6, 0x89, 0xf0}
DEFINE_GUID!{IID_ID3D10DeviceChild, 0x9b7e4c00, 0x342c, 0x4106, 0xa1, 0x9f, 0x4f, 0x27, 0x04, 0xf6, 0x89, 0xf0}
DEFINE_GUID!{IID_ID3D10Effect, 0x51b0ca8b, 0xec0b, 0x4519, 0x87, 0x0d, 0x8e, 0xe1, 0xcb, 0x50, 0x17, 0xc7}
DEFINE_GUID!{IID_ID3D10EffectBlendVariable, 0x1fcd2294, 0xdf6d, 0x4eae, 0x86, 0xb3, 0x0e, 0x91, 0x60, 0xcf, 0xb0, 0x7b}
DEFINE_GUID!{IID_ID3D10EffectConstantBuffer, 0x56648f4d, 0xcc8b, 0x4444, 0xa5, 0xad, 0xb5, 0xa3, 0xd7, 0x6e, 0x91, 0xb3}
DEFINE_GUID!{IID_ID3D10EffectDepthStencilVariable, 0xaf482368, 0x330a, 0x46a5, 0x9a, 0x5c, 0x01, 0xc7, 0x1a, 0xf2, 0x4c, 0x8d}
DEFINE_GUID!{IID_ID3D10EffectDepthStencilViewVariable, 0x3e02c918, 0xcc79, 0x4985, 0xb6, 0x22, 0x2d, 0x92, 0xad, 0x70, 0x16, 0x23}
DEFINE_GUID!{IID_ID3D10EffectMatrixVariable, 0x50666c24, 0xb82f, 0x4eed, 0xa1, 0x72, 0x5b, 0x6e, 0x7e, 0x85, 0x22, 0xe0}
DEFINE_GUID!{IID_ID3D10EffectPass, 0x5cfbeb89, 0x1a06, 0x46e0, 0xb2, 0x82, 0xe3, 0xf9, 0xbf, 0xa3, 0x6a, 0x54}
DEFINE_GUID!{IID_ID3D10EffectPool, 0x9537ab04, 0x3250, 0x412e, 0x82, 0x13, 0xfc, 0xd2, 0xf8, 0x67, 0x79, 0x33}
DEFINE_GUID!{IID_ID3D10EffectRasterizerVariable, 0x21af9f0e, 0x4d94, 0x4ea9, 0x97, 0x85, 0x2c, 0xb7, 0x6b, 0x8c, 0x0b, 0x34}
DEFINE_GUID!{IID_ID3D10EffectRenderTargetViewVariable, 0x28ca0cc3, 0xc2c9, 0x40bb, 0xb5, 0x7f, 0x67, 0xb7, 0x37, 0x12, 0x2b, 0x17}
DEFINE_GUID!{IID_ID3D10EffectSamplerVariable, 0x6530d5c7, 0x07e9, 0x4271, 0xa4, 0x18, 0xe7, 0xce, 0x4b, 0xd1, 0xe4, 0x80}
DEFINE_GUID!{IID_ID3D10EffectScalarVariable, 0x00e48f7b, 0xd2c8, 0x49e8, 0xa8, 0x6c, 0x02, 0x2d, 0xee, 0x53, 0x43, 0x1f}
DEFINE_GUID!{IID_ID3D10EffectShaderResourceVariable, 0xc0a7157b, 0xd872, 0x4b1d, 0x80, 0x73, 0xef, 0xc2, 0xac, 0xd4, 0xb1, 0xfc}
DEFINE_GUID!{IID_ID3D10EffectShaderVariable, 0x80849279, 0xc799, 0x4797, 0x8c, 0x33, 0x04, 0x07, 0xa0, 0x7d, 0x9e, 0x06}
DEFINE_GUID!{IID_ID3D10EffectStringVariable, 0x71417501, 0x8df9, 0x4e0a, 0xa7, 0x8a, 0x25, 0x5f, 0x97, 0x56, 0xba, 0xff}
DEFINE_GUID!{IID_ID3D10EffectTechnique, 0xdb122ce8, 0xd1c9, 0x4292, 0xb2, 0x37, 0x24, 0xed, 0x3d, 0xe8, 0xb1, 0x75}
DEFINE_GUID!{IID_ID3D10EffectType, 0x4e9e1ddc, 0xcd9d, 0x4772, 0xa8, 0x37, 0x00, 0x18, 0x0b, 0x9b, 0x88, 0xfd}
DEFINE_GUID!{IID_ID3D10EffectVariable, 0xae897105, 0x00e6, 0x45bf, 0xbb, 0x8e, 0x28, 0x1d, 0xd6, 0xdb, 0x8e, 0x1b}
DEFINE_GUID!{IID_ID3D10EffectVectorVariable, 0x62b98c44, 0x1f82, 0x4c67, 0xbc, 0xd0, 0x72, 0xcf, 0x8f, 0x21, 0x7e, 0x81}
DEFINE_GUID!{IID_ID3D10GeometryShader, 0x6316be88, 0x54cd, 0x4040, 0xab, 0x44, 0x20, 0x46, 0x1b, 0xc8, 0x1f, 0x68}
DEFINE_GUID!{IID_ID3D10InputLayout, 0x9b7e4c0b, 0x342c, 0x4106, 0xa1, 0x9f, 0x4f, 0x27, 0x04, 0xf6, 0x89, 0xf0}
DEFINE_GUID!{IID_ID3D10Multithread, 0x9b7e4e00, 0x342c, 0x4106, 0xa1, 0x9f, 0x4f, 0x27, 0x04, 0xf6, 0x89, 0xf0}
DEFINE_GUID!{IID_ID3D10PixelShader, 0x4968b601, 0x9d00, 0x4cde, 0x83, 0x46, 0x8e, 0x7f, 0x67, 0x58, 0x19, 0xb6}
DEFINE_GUID!{IID_ID3D10Predicate, 0x9b7e4c10, 0x342c, 0x4106, 0xa1, 0x9f, 0x4f, 0x27, 0x04, 0xf6, 0x89, 0xf0}
DEFINE_GUID!{IID_ID3D10Query, 0x9b7e4c0e, 0x342c, 0x4106, 0xa1, 0x9f, 0x4f, 0x27, 0x04, 0xf6, 0x89, 0xf0}
DEFINE_GUID!{IID_ID3D10RasterizerState, 0xa2a07292, 0x89af, 0x4345, 0xbe, 0x2e, 0xc5, 0x3d, 0x9f, 0xbb, 0x6e, 0x9f}
DEFINE_GUID!{IID_ID3D10RenderTargetView, 0x9b7e4c08, 0x342c, 0x4106, 0xa1, 0x9f, 0x4f, 0x27, 0x04, 0xf6, 0x89, 0xf0}
DEFINE_GUID!{IID_ID3D10Resource, 0x9b7e4c01, 0x342c, 0x4106, 0xa1, 0x9f, 0x4f, 0x27, 0x04, 0xf6, 0x89, 0xf0}
DEFINE_GUID!{IID_ID3D10SamplerState, 0x9b7e4c0c, 0x342c, 0x4106, 0xa1, 0x9f, 0x4f, 0x27, 0x04, 0xf6, 0x89, 0xf0}
DEFINE_GUID!{IID_ID3D10ShaderReflection1, 0xc3457783, 0xa846, 0x47ce, 0x95, 0x20, 0xce, 0xa6, 0xf6, 0x6e, 0x74, 0x47}
DEFINE_GUID!{IID_ID3D10ShaderReflection, 0xd40e20b6, 0xf8f7, 0x42ad, 0xab, 0x20, 0x4b, 0xaf, 0x8f, 0x15, 0xdf, 0xaa}
DEFINE_GUID!{IID_ID3D10ShaderReflectionConstantBuffer, 0x66c66a94, 0xdddd, 0x4b62, 0xa6, 0x6a, 0xf0, 0xda, 0x33, 0xc2, 0xb4, 0xd0}
DEFINE_GUID!{IID_ID3D10ShaderReflectionType, 0xc530ad7d, 0x9b16, 0x4395, 0xa9, 0x79, 0xba, 0x2e, 0xcf, 0xf8, 0x3a, 0xdd}
DEFINE_GUID!{IID_ID3D10ShaderReflectionVariable, 0x1bf63c95, 0x2650, 0x405d, 0x99, 0xc1, 0x36, 0x36, 0xbd, 0x1d, 0xa0, 0xa1}
DEFINE_GUID!{IID_ID3D10ShaderResourceView1, 0x9b7e4c87, 0x342c, 0x4106, 0xa1, 0x9f, 0x4f, 0x27, 0x04, 0xf6, 0x89, 0xf0}
DEFINE_GUID!{IID_ID3D10ShaderResourceView, 0x9b7e4c07, 0x342c, 0x4106, 0xa1, 0x9f, 0x4f, 0x27, 0x04, 0xf6, 0x89, 0xf0}
DEFINE_GUID!{IID_ID3D10StateBlock, 0x0803425a, 0x57f5, 0x4dd6, 0x94, 0x65, 0xa8, 0x75, 0x70, 0x83, 0x4a, 0x08}
DEFINE_GUID!{IID_ID3D10Texture1D, 0x9b7e4c03, 0x342c, 0x4106, 0xa1, 0x9f, 0x4f, 0x27, 0x04, 0xf6, 0x89, 0xf0}
DEFINE_GUID!{IID_ID3D10Texture2D, 0x9b7e4c04, 0x342c, 0x4106, 0xa1, 0x9f, 0x4f, 0x27, 0x04, 0xf6, 0x89, 0xf0}
DEFINE_GUID!{IID_ID3D10Texture3D, 0x9b7e4c05, 0x342c, 0x4106, 0xa1, 0x9f, 0x4f, 0x27, 0x04, 0xf6, 0x89, 0xf0}
DEFINE_GUID!{IID_ID3D10VertexShader, 0x9b7e4c0a, 0x342c, 0x4106, 0xa1, 0x9f, 0x4f, 0x27, 0x04, 0xf6, 0x89, 0xf0}
DEFINE_GUID!{IID_ID3D10View, 0xc902b03f, 0x60a7, 0x49ba, 0x99, 0x36, 0x2a, 0x3a, 0xb3, 0x7a, 0x7e, 0x33}
DEFINE_GUID!{IID_ID3D11BlendState1, 0xcc86fabe, 0xda55, 0x401d, 0x85, 0xe7, 0xe3, 0xc9, 0xde, 0x28, 0x77, 0xe9}
DEFINE_GUID!{IID_ID3D11Device1, 0xa04bfb29, 0x08ef, 0x43d6, 0xa4, 0x9c, 0xa9, 0xbd, 0xbd, 0xcb, 0xe6, 0x86}
DEFINE_GUID!{IID_ID3D11Device2, 0x9d06dffa, 0xd1e5, 0x4d07, 0x83, 0xa8, 0x1b, 0xb1, 0x23, 0xf2, 0xf8, 0x41}
DEFINE_GUID!{IID_ID3D11Device3, 0xa05c8c37, 0xd2c6, 0x4732, 0xb3, 0xa0, 0x9c, 0xe0, 0xb0, 0xdc, 0x9a, 0xe6}
DEFINE_GUID!{IID_ID3D11Device4, 0x8992ab71, 0x02e6, 0x4b8d, 0xba, 0x48, 0xb0, 0x56, 0xdc, 0xda, 0x42, 0xc4}
DEFINE_GUID!{IID_ID3D11DeviceContext1, 0xbb2c6faa, 0xb5fb, 0x4082, 0x8e, 0x6b, 0x38, 0x8b, 0x8c, 0xfa, 0x90, 0xe1}
DEFINE_GUID!{IID_ID3D11DeviceContext2, 0x420d5b32, 0xb90c, 0x4da4, 0xbe, 0xf0, 0x35, 0x9f, 0x6a, 0x24, 0xa8, 0x3a}
DEFINE_GUID!{IID_ID3D11DeviceContext3, 0xb4e3c01d, 0xe79e, 0x4637, 0x91, 0xb2, 0x51, 0x0e, 0x9f, 0x4c, 0x9b, 0x8f}
DEFINE_GUID!{IID_ID3D11FunctionLinkingGraph, 0x54133220, 0x1ce8, 0x43d3, 0x82, 0x36, 0x98, 0x55, 0xc5, 0xce, 0xec, 0xff}
DEFINE_GUID!{IID_ID3D11FunctionParameterReflection, 0x42757488, 0x334f, 0x47fe, 0x98, 0x2e, 0x1a, 0x65, 0xd0, 0x8c, 0xc4, 0x62}
DEFINE_GUID!{IID_ID3D11FunctionReflection, 0x207bcecb, 0xd683, 0x4a06, 0xa8, 0xa3, 0x9b, 0x14, 0x9b, 0x9f, 0x73, 0xa4}
DEFINE_GUID!{IID_ID3D11LibraryReflection, 0x54384f1b, 0x5b3e, 0x4bb7, 0xae, 0x01, 0x60, 0xba, 0x30, 0x97, 0xcb, 0xb6}
DEFINE_GUID!{IID_ID3D11Linker, 0x59a6cd0e, 0xe10d, 0x4c1f, 0x88, 0xc0, 0x63, 0xab, 0xa1, 0xda, 0xf3, 0x0e}
DEFINE_GUID!{IID_ID3D11LinkingNode, 0xd80dd70c, 0x8d2f, 0x4751, 0x94, 0xa1, 0x03, 0xc7, 0x9b, 0x35, 0x56, 0xdb}
DEFINE_GUID!{IID_ID3D11Module, 0xcac701ee, 0x80fc, 0x4122, 0x82, 0x42, 0x10, 0xb3, 0x9c, 0x8c, 0xec, 0x34}
DEFINE_GUID!{IID_ID3D11ModuleInstance, 0x469e07f7, 0x045a, 0x48d5, 0xaa, 0x12, 0x68, 0xa4, 0x78, 0xcd, 0xf7, 0x5d}
DEFINE_GUID!{IID_ID3D11On12Device, 0x85611e73, 0x70a9, 0x490e, 0x96, 0x14, 0xa9, 0xe3, 0x02, 0x77, 0x79, 0x04}
DEFINE_GUID!{IID_ID3D11Query1, 0x631b4766, 0x36dc, 0x461d, 0x8d, 0xb6, 0xc4, 0x7e, 0x13, 0xe6, 0x09, 0x16}
DEFINE_GUID!{IID_ID3D11RasterizerState1, 0x1217d7a6, 0x5039, 0x418c, 0xb0, 0x42, 0x9c, 0xbe, 0x25, 0x6a, 0xfd, 0x6e}
DEFINE_GUID!{IID_ID3D11RasterizerState2, 0x6fbd02fb, 0x209f, 0x46c4, 0xb0, 0x59, 0x2e, 0xd1, 0x55, 0x86, 0xa6, 0xac}
DEFINE_GUID!{IID_ID3D11RenderTargetView1, 0xffbe2e23, 0xf011, 0x418a, 0xac, 0x56, 0x5c, 0xee, 0xd7, 0xc5, 0xb9, 0x4b}
DEFINE_GUID!{IID_ID3D11ShaderReflection, 0x8d536ca1, 0x0cca, 0x4956, 0xa8, 0x37, 0x78, 0x69, 0x63, 0x75, 0x55, 0x84}
DEFINE_GUID!{IID_ID3D11ShaderReflectionConstantBuffer, 0xeb62d63d, 0x93dd, 0x4318, 0x8a, 0xe8, 0xc6, 0xf8, 0x3a, 0xd3, 0x71, 0xb8}
DEFINE_GUID!{IID_ID3D11ShaderReflectionType, 0x6e6ffa6a, 0x9bae, 0x4613, 0xa5, 0x1e, 0x91, 0x65, 0x2d, 0x50, 0x8c, 0x21}
DEFINE_GUID!{IID_ID3D11ShaderReflectionVariable, 0x51f23923, 0xf3e5, 0x4bd1, 0x91, 0xcb, 0x60, 0x61, 0x77, 0xd8, 0xdb, 0x4c}
DEFINE_GUID!{IID_ID3D11ShaderResourceView1, 0x91308b87, 0x9040, 0x411d, 0x8c, 0x67, 0xc3, 0x92, 0x53, 0xce, 0x38, 0x02}
DEFINE_GUID!{IID_ID3D11Texture2D1, 0x51218251, 0x1e33, 0x4617, 0x9c, 0xcb, 0x4d, 0x3a, 0x43, 0x67, 0xe7, 0xbb}
DEFINE_GUID!{IID_ID3D11Texture3D1, 0x0c711683, 0x2853, 0x4846, 0x9b, 0xb0, 0xf3, 0xe6, 0x06, 0x39, 0xe4, 0x6a}
DEFINE_GUID!{IID_ID3D11UnorderedAccessView1, 0x7b3b6153, 0xa886, 0x4544, 0xab, 0x37, 0x65, 0x37, 0xc8, 0x50, 0x04, 0x03}
DEFINE_GUID!{IID_ID3D11VideoContext1, 0xa7f026da, 0xa5f8, 0x4487, 0xa5, 0x64, 0x15, 0xe3, 0x43, 0x57, 0x65, 0x1e}
DEFINE_GUID!{IID_ID3D11VideoDevice1, 0x29da1d51, 0x1321, 0x4454, 0x80, 0x4b, 0xf5, 0xfc, 0x9f, 0x86, 0x1f, 0x0f}
DEFINE_GUID!{IID_ID3D11VideoProcessorEnumerator1, 0x465217f2, 0x5568, 0x43cf, 0xb5, 0xb9, 0xf6, 0x1d, 0x54, 0x53, 0x1c, 0xa1}
DEFINE_GUID!{IID_ID3D12CommandAllocator, 0x6102dee4, 0xaf59, 0x4b09, 0xb9, 0x99, 0xb4, 0x4d, 0x73, 0xf0, 0x9b, 0x24}
DEFINE_GUID!{IID_ID3D12CommandList, 0x7116d91c, 0xe7e4, 0x47ce, 0xb8, 0xc6, 0xec, 0x81, 0x68, 0xf4, 0x37, 0xe5}
DEFINE_GUID!{IID_ID3D12CommandQueue, 0x0ec870a6, 0x5d7e, 0x4c22, 0x8c, 0xfc, 0x5b, 0xaa, 0xe0, 0x76, 0x16, 0xed}
DEFINE_GUID!{IID_ID3D12CommandSignature, 0xc36a797c, 0xec80, 0x4f0a, 0x89, 0x85, 0xa7, 0xb2, 0x47, 0x50, 0x82, 0xd1}
DEFINE_GUID!{IID_ID3D12DescriptorHeap, 0x8efb471d, 0x616c, 0x4f49, 0x90, 0xf7, 0x12, 0x7b, 0xb7, 0x63, 0xfa, 0x51}
DEFINE_GUID!{IID_ID3D12Device, 0x189819f1, 0x1db6, 0x4b57, 0xbe, 0x54, 0x18, 0x21, 0x33, 0x9b, 0x85, 0xf7}
DEFINE_GUID!{IID_ID3D12DeviceChild, 0x905db94b, 0xa00c, 0x4140, 0x9d, 0xf5, 0x2b, 0x64, 0xca, 0x9e, 0xa3, 0x57}
DEFINE_GUID!{IID_ID3D12Fence, 0x0a753dcf, 0xc4d8, 0x4b91, 0xad, 0xf6, 0xbe, 0x5a, 0x60, 0xd9, 0x5a, 0x76}
DEFINE_GUID!{IID_ID3D12FunctionParameterReflection, 0xec25f42d, 0x7006, 0x4f2b, 0xb3, 0x3e, 0x02, 0xcc, 0x33, 0x75, 0x73, 0x3f}
DEFINE_GUID!{IID_ID3D12FunctionReflection, 0x1108795c, 0x2772, 0x4ba9, 0xb2, 0xa8, 0xd4, 0x64, 0xdc, 0x7e, 0x27, 0x99}
DEFINE_GUID!{IID_ID3D12GraphicsCommandList, 0x5b160d0f, 0xac1b, 0x4185, 0x8b, 0xa8, 0xb3, 0xae, 0x42, 0xa5, 0xa4, 0x55}
DEFINE_GUID!{IID_ID3D12Heap, 0x6b3b2502, 0x6e51, 0x45b3, 0x90, 0xee, 0x98, 0x84, 0x26, 0x5e, 0x8d, 0xf3}
DEFINE_GUID!{IID_ID3D12LibraryReflection, 0x8e349d19, 0x54db, 0x4a56, 0x9d, 0xc9, 0x11, 0x9d, 0x87, 0xbd, 0xb8, 0x04}
DEFINE_GUID!{IID_ID3D12Object, 0xc4fec28f, 0x7966, 0x4e95, 0x9f, 0x94, 0xf4, 0x31, 0xcb, 0x56, 0xc3, 0xb8}
DEFINE_GUID!{IID_ID3D12Pageable, 0x63ee58fb, 0x1268, 0x4835, 0x86, 0xda, 0xf0, 0x08, 0xce, 0x62, 0xf0, 0xd6}
DEFINE_GUID!{IID_ID3D12PipelineState, 0x765a30f3, 0xf624, 0x4c6f, 0xa8, 0x28, 0xac, 0xe9, 0x48, 0x62, 0x24, 0x45}
DEFINE_GUID!{IID_ID3D12QueryHeap, 0x0d9658ae, 0xed45, 0x469e, 0xa6, 0x1d, 0x97, 0x0e, 0xc5, 0x83, 0xca, 0xb4}
DEFINE_GUID!{IID_ID3D12Resource, 0x696442be, 0xa72e, 0x4059, 0xbc, 0x79, 0x5b, 0x5c, 0x98, 0x04, 0x0f, 0xad}
DEFINE_GUID!{IID_ID3D12RootSignature, 0xc54a6b66, 0x72df, 0x4ee8, 0x8b, 0xe5, 0xa9, 0x46, 0xa1, 0x42, 0x92, 0x14}
DEFINE_GUID!{IID_ID3D12RootSignatureDeserializer, 0x34ab647b, 0x3cc8, 0x46ac, 0x84, 0x1b, 0xc0, 0x96, 0x56, 0x45, 0xc0, 0x46}
DEFINE_GUID!{IID_ID3D12ShaderReflection, 0x5a58797d, 0xa72c, 0x478d, 0x8b, 0xa2, 0xef, 0xc6, 0xb0, 0xef, 0xe8, 0x8e}
DEFINE_GUID!{IID_ID3D12ShaderReflectionConstantBuffer, 0xc59598b4, 0x48b3, 0x4869, 0xb9, 0xb1, 0xb1, 0x61, 0x8b, 0x14, 0xa8, 0xb7}
DEFINE_GUID!{IID_ID3D12ShaderReflectionType, 0xe913c351, 0x783d, 0x48ca, 0xa1, 0xd1, 0x4f, 0x30, 0x62, 0x84, 0xad, 0x56}
DEFINE_GUID!{IID_ID3D12ShaderReflectionVariable, 0x8337a8a6, 0xa216, 0x444a, 0xb2, 0xf4, 0x31, 0x47, 0x33, 0xa7, 0x3a, 0xea}
DEFINE_GUID!{IID_ID3DDeviceContextState, 0x5c1e0d8a, 0x7c23, 0x48f9, 0x8c, 0x59, 0xa9, 0x29, 0x58, 0xce, 0xff, 0x11}
DEFINE_GUID!{IID_ID3DUserDefinedAnnotation, 0xb2daad8b, 0x03d4, 0x4dbf, 0x95, 0xeb, 0x32, 0xab, 0x4b, 0x63, 0xd0, 0xab}
DEFINE_GUID!{IID_ID3DX10BaseMesh, 0x7ed943dd, 0x52e8, 0x40b5, 0xa8, 0xd8, 0x76, 0x68, 0x5c, 0x40, 0x63, 0x30}
DEFINE_GUID!{IID_ID3DX10Font, 0xd79dbb70, 0x5f21, 0x4d36, 0xbb, 0xc2, 0xff, 0x52, 0x5c, 0x21, 0x3c, 0xdc}
DEFINE_GUID!{IID_ID3DX10Mesh, 0x4020e5c2, 0x1403, 0x4929, 0x88, 0x3f, 0xe2, 0xe8, 0x49, 0xfa, 0xc1, 0x95}
DEFINE_GUID!{IID_ID3DX10MeshBuffer, 0x04b0d117, 0x1041, 0x46b1, 0xaa, 0x8a, 0x39, 0x52, 0x84, 0x8b, 0xa2, 0x2e}
DEFINE_GUID!{IID_ID3DX10PMesh, 0x8875769a, 0xd579, 0x4088, 0xaa, 0xeb, 0x53, 0x4d, 0x1a, 0xd8, 0x4e, 0x96}
DEFINE_GUID!{IID_ID3DX10PatchMesh, 0x3ce6cc22, 0xdbf2, 0x44f4, 0x89, 0x4d, 0xf9, 0xc3, 0x4a, 0x33, 0x71, 0x39}
DEFINE_GUID!{IID_ID3DX10SPMesh, 0x667ea4c7, 0xf1cd, 0x4386, 0xb5, 0x23, 0x7c, 0x02, 0x90, 0xb8, 0x3c, 0xc5}
DEFINE_GUID!{IID_ID3DX10SkinInfo, 0x420bd604, 0x1c76, 0x4a34, 0xa4, 0x66, 0xe4, 0x5d, 0x06, 0x58, 0xa3, 0x2c}
DEFINE_GUID!{IID_ID3DX10Sprite, 0xba0b762d, 0x8d28, 0x43ec, 0xb9, 0xdc, 0x2f, 0x84, 0x44, 0x3b, 0x06, 0x14}
DEFINE_GUID!{IID_ID3DX10ThreadPump, 0xc93fecfa, 0x6967, 0x478a, 0xab, 0xbc, 0x40, 0x2d, 0x90, 0x62, 0x1f, 0xcb}
DEFINE_GUID!{IID_ID3DX11FFT, 0xb3f7a938, 0x4c93, 0x4310, 0xa6, 0x75, 0xb3, 0x0d, 0x6d, 0xe5, 0x05, 0x53}
DEFINE_GUID!{IID_ID3DX11Scan, 0x5089b68f, 0xe71d, 0x4d38, 0xbe, 0x8e, 0xf3, 0x63, 0xb9, 0x5a, 0x94, 0x05}
DEFINE_GUID!{IID_ID3DX11SegmentedScan, 0xa915128c, 0xd954, 0x4c79, 0xbf, 0xe1, 0x64, 0xdb, 0x92, 0x31, 0x94, 0xd6}
DEFINE_GUID!{IID_ID3DX11ThreadPump, 0xc93fecfa, 0x6967, 0x478a, 0xab, 0xbc, 0x40, 0x2d, 0x90, 0x62, 0x1f, 0xcb}
DEFINE_GUID!{IID_ID3DXMatrixStack, 0xc7885ba7, 0xf990, 0x4fe7, 0x92, 0x2d, 0x85, 0x15, 0xe4, 0x77, 0xdd, 0x85}
DEFINE_GUID!{IID_IDXGIAdapter1, 0x29038f61, 0x3839, 0x4626, 0x91, 0xfd, 0x08, 0x68, 0x79, 0x01, 0x1a, 0x05}
DEFINE_GUID!{IID_IDXGIAdapter2, 0x0aa1ae0a, 0xfa0e, 0x4b84, 0x86, 0x44, 0xe0, 0x5f, 0xf8, 0xe5, 0xac, 0xb5}
DEFINE_GUID!{IID_IDXGIAdapter3, 0x645967a4, 0x1392, 0x4310, 0xa7, 0x98, 0x80, 0x53, 0xce, 0x3e, 0x93, 0xfd}
DEFINE_GUID!{IID_IDXGIAdapter, 0x2411e7e1, 0x12ac, 0x4ccf, 0xbd, 0x14, 0x97, 0x98, 0xe8, 0x53, 0x4d, 0xc0}
DEFINE_GUID!{IID_IDXGIDecodeSwapChain, 0x2633066b, 0x4514, 0x4c7a, 0x8f, 0xd8, 0x12, 0xea, 0x98, 0x05, 0x9d, 0x18}
DEFINE_GUID!{IID_IDXGIDevice1, 0x77db970f, 0x6276, 0x48ba, 0xba, 0x28, 0x07, 0x01, 0x43, 0xb4, 0x39, 0x2c}
DEFINE_GUID!{IID_IDXGIDevice2, 0x05008617, 0xfbfd, 0x4051, 0xa7, 0x90, 0x14, 0x48, 0x84, 0xb4, 0xf6, 0xa9}
DEFINE_GUID!{IID_IDXGIDevice3, 0x6007896c, 0x3244, 0x4afd, 0xbf, 0x18, 0xa6, 0xd3, 0xbe, 0xda, 0x50, 0x23}
DEFINE_GUID!{IID_IDXGIDevice, 0x54ec77fa, 0x1377, 0x44e6, 0x8c, 0x32, 0x88, 0xfd, 0x5f, 0x44, 0xc8, 0x4c}
DEFINE_GUID!{IID_IDXGIDeviceSubObject, 0x3d3e0379, 0xf9de, 0x4d58, 0xbb, 0x6c, 0x18, 0xd6, 0x29, 0x92, 0xf1, 0xa6}
DEFINE_GUID!{IID_IDXGIDisplayControl, 0xea9dbf1a, 0xc88e, 0x4486, 0x85, 0x4a, 0x98, 0xaa, 0x01, 0x38, 0xf3, 0x0c}
DEFINE_GUID!{IID_IDXGIFactory1, 0x770aae78, 0xf26f, 0x4dba, 0xa8, 0x29, 0x25, 0x3c, 0x83, 0xd1, 0xb3, 0x87}
DEFINE_GUID!{IID_IDXGIFactory2, 0x50c83a1c, 0xe072, 0x4c48, 0x87, 0xb0, 0x36, 0x30, 0xfa, 0x36, 0xa6, 0xd0}
DEFINE_GUID!{IID_IDXGIFactory3, 0x25483823, 0xcd46, 0x4c7d, 0x86, 0xca, 0x47, 0xaa, 0x95, 0xb8, 0x37, 0xbd}
DEFINE_GUID!{IID_IDXGIFactory4, 0x1bc6ea02, 0xef36, 0x464f, 0xbf, 0x0c, 0x21, 0xca, 0x39, 0xe5, 0x16, 0x8a}
DEFINE_GUID!{IID_IDXGIFactory, 0x7b7166ec, 0x21c7, 0x44ae, 0xb2, 0x1a, 0xc9, 0xae, 0x32, 0x1a, 0xe3, 0x69}
DEFINE_GUID!{IID_IDXGIFactoryMedia, 0x41e7d1f2, 0xa591, 0x4f7b, 0xa2, 0xe5, 0xfa, 0x9c, 0x84, 0x3e, 0x1c, 0x12}
DEFINE_GUID!{IID_IDXGIKeyedMutex, 0x9d8e1289, 0xd7b3, 0x465f, 0x81, 0x26, 0x25, 0x0e, 0x34, 0x9a, 0xf8, 0x5d}
DEFINE_GUID!{IID_IDXGIObject, 0xaec22fb8, 0x76f3, 0x4639, 0x9b, 0xe0, 0x28, 0xeb, 0x43, 0xa6, 0x7a, 0x2e}
DEFINE_GUID!{IID_IDXGIOutput1, 0x00cddea8, 0x939b, 0x4b83, 0xa3, 0x40, 0xa6, 0x85, 0x22, 0x66, 0x66, 0xcc}
DEFINE_GUID!{IID_IDXGIOutput2, 0x595e39d1, 0x2724, 0x4663, 0x99, 0xb1, 0xda, 0x96, 0x9d, 0xe2, 0x83, 0x64}
DEFINE_GUID!{IID_IDXGIOutput3, 0x8a6bb301, 0x7e7e, 0x41f4, 0xa8, 0xe0, 0x5b, 0x32, 0xf7, 0xf9, 0x9b, 0x18}
DEFINE_GUID!{IID_IDXGIOutput4, 0xdc7dca35, 0x2196, 0x414d, 0x9f, 0x53, 0x61, 0x78, 0x84, 0x03, 0x2a, 0x60}
DEFINE_GUID!{IID_IDXGIOutput5, 0x80a07424, 0xab52, 0x42eb, 0x83, 0x3c, 0x0c, 0x42, 0xfd, 0x28, 0x2d, 0x98}
DEFINE_GUID!{IID_IDXGIOutput, 0xae02eedb, 0xc735, 0x4690, 0x8d, 0x52, 0x5a, 0x8d, 0xc2, 0x02, 0x13, 0xaa}
DEFINE_GUID!{IID_IDXGIOutputDuplication, 0x191cfac3, 0xa341, 0x470d, 0xb2, 0x6e, 0xa8, 0x64, 0xf4, 0x28, 0x31, 0x9c}
DEFINE_GUID!{IID_IDXGIResource1, 0x30961379, 0x4609, 0x4a41, 0x99, 0x8e, 0x54, 0xfe, 0x56, 0x7e, 0xe0, 0xc1}
DEFINE_GUID!{IID_IDXGIResource, 0x035f3ab4, 0x482e, 0x4e50, 0xb4, 0x1f, 0x8a, 0x7f, 0x8b, 0xd8, 0x96, 0x0b}
DEFINE_GUID!{IID_IDXGISurface1, 0x4ae63092, 0x6327, 0x4c1b, 0x80, 0xae, 0xbf, 0xe1, 0x2e, 0xa3, 0x2b, 0x86}
DEFINE_GUID!{IID_IDXGISurface2, 0xaba496dd, 0xb617, 0x4cb8, 0xa8, 0x66, 0xbc, 0x44, 0xd7, 0xeb, 0x1f, 0xa2}
DEFINE_GUID!{IID_IDXGISurface, 0xcafcb56c, 0x6ac3, 0x4889, 0xbf, 0x47, 0x9e, 0x23, 0xbb, 0xd2, 0x60, 0xec}
DEFINE_GUID!{IID_IDXGISwapChain1, 0x790a45f7, 0x0d42, 0x4876, 0x98, 0x3a, 0x0a, 0x55, 0xcf, 0xe6, 0xf4, 0xaa}
DEFINE_GUID!{IID_IDXGISwapChain2, 0xa8be2ac4, 0x199f, 0x4946, 0xb3, 0x31, 0x79, 0x59, 0x9f, 0xb9, 0x8d, 0xe7}
DEFINE_GUID!{IID_IDXGISwapChain3, 0x94d99bdb, 0xf1f8, 0x4ab0, 0xb2, 0x36, 0x7d, 0xa0, 0x17, 0x0e, 0xda, 0xb1}
DEFINE_GUID!{IID_IDXGISwapChain, 0x310d36a0, 0xd2e7, 0x4c0a, 0xaa, 0x04, 0x6a, 0x9d, 0x23, 0xb8, 0x88, 0x6a}
DEFINE_GUID!{IID_IDXGISwapChainMedia, 0xdd95b90b, 0xf05f, 0x4f6a, 0xbd, 0x65, 0x25, 0xbf, 0xb2, 0x64, 0xbd, 0x84}
DEFINE_GUID!{IID_IDirect3D2, 0x6aae1ec1, 0x662a, 0x11d0, 0x88, 0x9d, 0x00, 0xaa, 0x00, 0xbb, 0xb7, 0x6a}
DEFINE_GUID!{IID_IDirect3D3, 0xbb223240, 0xe72b, 0x11d0, 0xa9, 0xb4, 0x00, 0xaa, 0x00, 0xc0, 0x99, 0x3e}
DEFINE_GUID!{IID_IDirect3D7, 0xf5049e77, 0x4861, 0x11d2, 0xa4, 0x07, 0x00, 0xa0, 0xc9, 0x06, 0x29, 0xa8}
DEFINE_GUID!{IID_IDirect3D8, 0x1dd9e8da, 0x1c77, 0x4d40, 0xb0, 0xcf, 0x98, 0xfe, 0xfd, 0xff, 0x95, 0x12}
DEFINE_GUID!{IID_IDirect3D, 0x3bba0080, 0x2421, 0x11cf, 0xa3, 0x1a, 0x00, 0xaa, 0x00, 0xb9, 0x33, 0x56}
DEFINE_GUID!{IID_IDirect3DBaseTexture8, 0xb4211cfa, 0x51b9, 0x4a9f, 0xab, 0x78, 0xdb, 0x99, 0xb2, 0xbb, 0x67, 0x8e}
DEFINE_GUID!{IID_IDirect3DCubeTexture8, 0x3ee5b968, 0x2aca, 0x4c34, 0x8b, 0xb5, 0x7e, 0x0c, 0x3d, 0x19, 0xb7, 0x50}
DEFINE_GUID!{IID_IDirect3DDevice2, 0x93281501, 0x8cf8, 0x11d0, 0x89, 0xab, 0x00, 0xa0, 0xc9, 0x05, 0x41, 0x29}
DEFINE_GUID!{IID_IDirect3DDevice3, 0xb0ab3b60, 0x33d7, 0x11d1, 0xa9, 0x81, 0x00, 0xc0, 0x4f, 0xd7, 0xb1, 0x74}
DEFINE_GUID!{IID_IDirect3DDevice7, 0xf5049e79, 0x4861, 0x11d2, 0xa4, 0x07, 0x00, 0xa0, 0xc9, 0x06, 0x29, 0xa8}
DEFINE_GUID!{IID_IDirect3DDevice8, 0x7385e5df, 0x8fe8, 0x41d5, 0x86, 0xb6, 0xd7, 0xb4, 0x85, 0x47, 0xb6, 0xcf}
DEFINE_GUID!{IID_IDirect3DDevice, 0x64108800, 0x957d, 0x11d0, 0x89, 0xab, 0x00, 0xa0, 0xc9, 0x05, 0x41, 0x29}
DEFINE_GUID!{IID_IDirect3DExecuteBuffer, 0x4417c145, 0x33ad, 0x11cf, 0x81, 0x6f, 0x00, 0x00, 0xc0, 0x20, 0x15, 0x6e}
DEFINE_GUID!{IID_IDirect3DHALDevice, 0x84e63de0, 0x46aa, 0x11cf, 0x81, 0x6f, 0x00, 0x00, 0xc0, 0x20, 0x15, 0x6e}
DEFINE_GUID!{IID_IDirect3DIndexBuffer8, 0x0e689c9a, 0x053d, 0x44a0, 0x9d, 0x92, 0xdb, 0x0e, 0x3d, 0x75, 0x0f, 0x86}
DEFINE_GUID!{IID_IDirect3DLight, 0x4417c142, 0x33ad, 0x11cf, 0x81, 0x6f, 0x00, 0x00, 0xc0, 0x20, 0x15, 0x6e}
DEFINE_GUID!{IID_IDirect3DMMXDevice, 0x881949a1, 0xd6f3, 0x11d0, 0x89, 0xab, 0x00, 0xa0, 0xc9, 0x05, 0x41, 0x29}
DEFINE_GUID!{IID_IDirect3DMaterial2, 0x93281503, 0x8cf8, 0x11d0, 0x89, 0xab, 0x00, 0xa0, 0xc9, 0x05, 0x41, 0x29}
DEFINE_GUID!{IID_IDirect3DMaterial3, 0xca9c46f4, 0xd3c5, 0x11d1, 0xb7, 0x5a, 0x00, 0x60, 0x08, 0x52, 0xb3, 0x12}
DEFINE_GUID!{IID_IDirect3DMaterial, 0x4417c144, 0x33ad, 0x11cf, 0x81, 0x6f, 0x00, 0x00, 0xc0, 0x20, 0x15, 0x6e}
DEFINE_GUID!{IID_IDirect3DNullDevice, 0x8767df22, 0xbacc, 0x11d1, 0x89, 0x69, 0x00, 0xa0, 0xc9, 0x06, 0x29, 0xa8}
DEFINE_GUID!{IID_IDirect3DRGBDevice, 0xa4665c60, 0x2673, 0x11cf, 0xa3, 0x1a, 0x00, 0xaa, 0x00, 0xb9, 0x33, 0x56}
DEFINE_GUID!{IID_IDirect3DRampDevice, 0xf2086b20, 0x259f, 0x11cf, 0xa3, 0x1a, 0x00, 0xaa, 0x00, 0xb9, 0x33, 0x56}
DEFINE_GUID!{IID_IDirect3DRefDevice, 0x50936643, 0x13e9, 0x11d1, 0x89, 0xaa, 0x00, 0xa0, 0xc9, 0x05, 0x41, 0x29}
DEFINE_GUID!{IID_IDirect3DResource8, 0x1b36bb7b, 0x09b7, 0x410a, 0xb4, 0x45, 0x7d, 0x14, 0x30, 0xd7, 0xb3, 0x3f}
DEFINE_GUID!{IID_IDirect3DSurface8, 0xb96eebca, 0xb326, 0x4ea5, 0x88, 0x2f, 0x2f, 0xf5, 0xba, 0xe0, 0x21, 0xdd}
DEFINE_GUID!{IID_IDirect3DSwapChain8, 0x928c088b, 0x76b9, 0x4c6b, 0xa5, 0x36, 0xa5, 0x90, 0x85, 0x38, 0x76, 0xcd}
DEFINE_GUID!{IID_IDirect3DTexture2, 0x93281502, 0x8cf8, 0x11d0, 0x89, 0xab, 0x00, 0xa0, 0xc9, 0x05, 0x41, 0x29}
DEFINE_GUID!{IID_IDirect3DTexture8, 0xe4cdd575, 0x2866, 0x4f01, 0xb1, 0x2e, 0x7e, 0xec, 0xe1, 0xec, 0x93, 0x58}
DEFINE_GUID!{IID_IDirect3DTexture, 0x2cdcd9e0, 0x25a0, 0x11cf, 0xa3, 0x1a, 0x00, 0xaa, 0x00, 0xb9, 0x33, 0x56}
DEFINE_GUID!{IID_IDirect3DTnLHalDevice, 0xf5049e78, 0x4861, 0x11d2, 0xa4, 0x07, 0x00, 0xa0, 0xc9, 0x06, 0x29, 0xa8}
DEFINE_GUID!{IID_IDirect3DVertexBuffer7, 0xf5049e7d, 0x4861, 0x11d2, 0xa4, 0x07, 0x00, 0xa0, 0xc9, 0x06, 0x29, 0xa8}
DEFINE_GUID!{IID_IDirect3DVertexBuffer8, 0x8aeeeac7, 0x05f9, 0x44d4, 0xb5, 0x91, 0x00, 0x0b, 0x0d, 0xf1, 0xcb, 0x95}
DEFINE_GUID!{IID_IDirect3DVertexBuffer, 0x7a503555, 0x4a83, 0x11d1, 0xa5, 0xdb, 0x00, 0xa0, 0xc9, 0x03, 0x67, 0xf8}
DEFINE_GUID!{IID_IDirect3DViewport2, 0x93281500, 0x8cf8, 0x11d0, 0x89, 0xab, 0x00, 0xa0, 0xc9, 0x05, 0x41, 0x29}
DEFINE_GUID!{IID_IDirect3DViewport3, 0xb0ab3b61, 0x33d7, 0x11d1, 0xa9, 0x81, 0x00, 0xc0, 0x4f, 0xd7, 0xb1, 0x74}
DEFINE_GUID!{IID_IDirect3DViewport, 0x4417c146, 0x33ad, 0x11cf, 0x81, 0x6f, 0x00, 0x00, 0xc0, 0x20, 0x15, 0x6e}
DEFINE_GUID!{IID_IDirect3DVolume8, 0xbd7349f5, 0x14f1, 0x42e4, 0x9c, 0x79, 0x97, 0x23, 0x80, 0xdb, 0x40, 0xc0}
DEFINE_GUID!{IID_IDirect3DVolumeTexture8, 0x4b8aaafa, 0x140f, 0x42ba, 0x91, 0x31, 0x59, 0x7e, 0xaf, 0xaa, 0x2e, 0xad}
DEFINE_GUID!{IID_IDirectDrawFactory2, 0x89b2c488, 0x4af4, 0x11d1, 0x8c, 0x4c, 0x00, 0xc0, 0x4f, 0xd9, 0x30, 0xc5}
DEFINE_GUID!{IID_IDirectDrawOptSurface, 0x51191f1e, 0x4f2b, 0x11d1, 0x8c, 0xc3, 0x00, 0xa0, 0xc9, 0x06, 0x29, 0xa8}
DEFINE_GUID!{IID_IDirectDrawPalette2, 0xc03c477e, 0x6519, 0x11d1, 0x8c, 0x52, 0x00, 0xc0, 0x4f, 0xd9, 0x30, 0xc5}
DEFINE_GUID!{IID_IDirectDrawSurfaceNew, 0x1bab8e96, 0x9cfe, 0x4ce3, 0xbc, 0x72, 0xd7, 0xe9, 0xe9, 0x9a, 0x21, 0x75}

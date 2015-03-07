// Copyright © 2015, Corey Richardson
// Licensed under the MIT License <LICENSE.md>
//! Direct3D capabilities include file

#[repr(C)]
#[derive(Copy, Default)]
pub struct D3DVSHADERCAPS2_0 {
    pub Caps: ::DWORD,
    pub DynamicFlowControlDepth: ::INT,
    pub NumTemps: ::INT,
    pub StaticFlowControlDepth: ::INT,
}

#[repr(C)]
#[derive(Copy, Default)]
pub struct D3DPSHADERCAPS2_0 {
    pub Caps: ::DWORD,
    pub DynamicFlowControlDepth: ::INT,
    pub NumTemps: ::INT,
    pub StaticFlowControlDepth: ::INT,
    pub NumInstructionSlots: ::INT,
}

#[repr(C)]
#[derive(Copy, Default)]
pub struct D3DOVERLAYCAPS {
    pub Caps: ::UINT,
    pub MaxOverlayDisplayWidth: ::UINT,
    pub MaxOverlayDisplayHeight: ::UINT,
}

#[repr(C)]
#[derive(Copy)]
pub struct D3DCONTENTPROTECTIONCAPS {
    pub Caps: ::DWORD,
    pub KeyExchangeType: ::GUID,
    pub BufferAlignmentStart: ::UINT,
    pub BlockAlignmentSize: ::UINT,
    pub ProtectedMemorySize: ::ULONGLONG,
}

#[repr(C)]
#[derive(Copy, Default)]
pub struct D3DCAPS9 {
    pub DeviceType: ::D3DDEVTYPE,
    pub AdapterOrdinal: ::UINT,
    pub Caps: ::DWORD,
    pub Caps2: ::DWORD,
    pub Caps3: ::DWORD,
    pub PresentationIntervals: ::DWORD,
    pub CursorCaps: ::DWORD,
    pub DevCaps: ::DWORD,
    pub PrimitiveMiscCaps: ::DWORD,
    pub RasterCaps: ::DWORD,
    pub ZCmpCaps: ::DWORD,
    pub SrcBlendCaps: ::DWORD,
    pub DestBlendCaps: ::DWORD,
    pub AlphaCmpCaps: ::DWORD,
    pub ShadeCaps: ::DWORD,
    pub TextureCaps: ::DWORD,
    pub TextureFilterCaps: ::DWORD,
    pub CubeTextureFilterCaps: ::DWORD,
    pub VolumeTextureFilterCaps: ::DWORD,
    pub TextureAddressCaps: ::DWORD,
    pub VolumeTextureAddressCaps: ::DWORD,
    pub LineCaps: ::DWORD,
    pub MaxTextureWidth: ::DWORD,
    pub MaxTextureHeight: ::DWORD,
    pub MaxVolumeExtent: ::DWORD,
    pub MaxTextureRepeat: ::DWORD,
    pub MaxTextureAspectRatio: ::DWORD,
    pub MaxAnisotropy: ::DWORD,
    pub MaxVertexW: ::FLOAT,
    pub GuardBandLeft: ::FLOAT,
    pub GuardBandTop: ::FLOAT,
    pub GuardBandRight: ::FLOAT,
    pub GuardBandBottom: ::FLOAT,
    pub ExtentsAdjust: ::FLOAT,
    pub StencilCaps: ::DWORD,
    pub FVFCaps: ::DWORD,
    pub TextureOpCaps: ::DWORD,
    pub MaxTextureBlendStages: ::DWORD,
    pub MaxSimultaneousTextures: ::DWORD,
    pub VertexProcessingCaps: ::DWORD,
    pub MaxActiveLights: ::DWORD,
    pub MaxUserClipPlanes: ::DWORD,
    pub MaxVertexBlendMatrices: ::DWORD,
    pub MaxVertexBlendMatrixIndex: ::DWORD,
    pub MaxPointSize: ::FLOAT,
    pub MaxPrimitiveCount: ::DWORD,
    pub MaxVertexIndex: ::DWORD,
    pub MaxStreams: ::DWORD,
    pub MaxStreamStride: ::DWORD,
    pub VertexShaderVersion: ::DWORD,
    pub MaxVertexShaderConst: ::DWORD,
    pub PixelShaderVersion: ::DWORD,
    pub PixelShader1xMaxValue: ::FLOAT,
    pub DevCaps2: ::DWORD,
    pub MaxNpatchTessellationLevel: ::FLOAT,
    pub Reserved5: ::DWORD,
    pub MasterAdapterOrdinal: ::UINT,
    pub AdapterOrdinalInGroup: ::UINT,
    pub NumberOfAdaptersInGroup: ::UINT,
    pub DeclTypes: ::DWORD,
    pub NumSimultaneousRTs: ::DWORD,
    pub StretchRectFilterCaps: ::DWORD,
    pub VS20Caps: ::D3DVSHADERCAPS2_0,
    pub PS20Caps: ::D3DPSHADERCAPS2_0,
    pub VertexTextureFilterCaps: ::DWORD,
    pub MaxVShaderInstructionsExecuted: ::DWORD,
    pub MaxPShaderInstructionsExecuted: ::DWORD,
    pub MaxVertexShader30InstructionSlots: ::DWORD,
    pub MaxPixelShader30InstructionSlots: ::DWORD,
}


extern "C" {
    pub static D3DCRYPTOTYPE_AES128_CTR: ::GUID;
    pub static D3DCRYPTOTYPE_PROPRIETARY: ::GUID;
    pub static D3DKEYEXCHANGE_RSAES_OAEP: ::GUID;
    pub static D3DKEYEXCHANGE_DXVA: ::GUID;
}

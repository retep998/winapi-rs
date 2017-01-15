// Copyright © 2015, Corey Richardson
// Licensed under the MIT License <LICENSE.md>
//! Direct3D include file
use shared::basetsd::{UINT32};
use shared::d3d9caps::{D3DOVERLAYCAPS, D3DCONTENTPROTECTIONCAPS, D3DCAPS9};
use shared::d3d9types::*;
use shared::guiddef::{GUID, IID};
use shared::minwindef::{UINT, DWORD, FLOAT, BOOL, INT, BYTE};
use shared::windef::{HWND, RECT, HDC, HMONITOR, POINT};
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::wingdi::{RGNDATA, PALETTEENTRY};
use um::winnt::{HRESULT, VOID, HANDLE, LUID};

pub const D3D_SDK_VERSION: DWORD = 32;
pub const D3D9b_SDK_VERSION: DWORD = 31;
RIDL!(
interface IDirect3D9(IDirect3D9Vtbl): IUnknown(IUnknownVtbl) {
    fn RegisterSoftwareDevice(&self, pInitializeFunction: *mut VOID) -> HRESULT,
    fn GetAdapterCount(&self) -> UINT,
    fn GetAdapterIdentifier(
        &self, Adapter: UINT, Flags: DWORD, pIdentifier: *mut D3DADAPTER_IDENTIFIER9
    ) -> HRESULT,
    fn GetAdapterModeCount(&self, Adapter: UINT, Format: D3DFORMAT) -> UINT,
    fn EnumAdapterModes(
        &self, Adapter: UINT, Format: D3DFORMAT, Mode: UINT, pMode: *mut D3DDISPLAYMODE
    ) -> HRESULT,
    fn GetAdapterDisplayMode(
        &self, Adapter: UINT, pMode: *mut D3DDISPLAYMODE
    ) -> HRESULT,
    fn CheckDeviceType(
        &self, Adapter: UINT, DevType: D3DDEVTYPE, AdapterFormat: D3DFORMAT,
        BackBufferFormat: D3DFORMAT, bWindowed: BOOL
    ) -> HRESULT,
    fn CheckDeviceFormat(
        &self, Adapter: UINT, DeviceType: D3DDEVTYPE, AdapterFormat: D3DFORMAT,
        Usage: DWORD, RType: D3DRESOURCETYPE, CheckFormat: D3DFORMAT
    ) -> HRESULT,
    fn CheckDeviceMultiSampleType(
        &self, Adapter: UINT, DeviceType: D3DDEVTYPE, SurfaceFormat: D3DFORMAT,
        Windowed: BOOL, MultiSampleType: D3DMULTISAMPLE_TYPE, pQualityLevels: *mut DWORD
    ) -> HRESULT,
    fn CheckDepthStencilMatch(
        &self, Adapter: UINT, DeviceType: D3DDEVTYPE, AdapterFormat: D3DFORMAT,
        RenderTargetFormat: D3DFORMAT, DepthStencilFormat: D3DFORMAT
    ) -> HRESULT,
    fn CheckDeviceFormatConversion(
        &self, Adapter: UINT, DeviceType: D3DDEVTYPE, SourceFormat: D3DFORMAT,
        TargetFormat: D3DFORMAT
    ) -> HRESULT,
    fn GetDeviceCaps(
        &self, Adapter: UINT, DeviceType: D3DDEVTYPE, pCaps: *mut D3DCAPS9
    ) -> HRESULT,
    fn GetAdapterMonitor(&self, Adapter: UINT) -> HMONITOR,
    fn CreateDevice(
        &self, Adapter: UINT, DeviceType: D3DDEVTYPE, hFocusWindow: HWND,
        BehaviorFlags: DWORD, pPresentationParameters: *mut D3DPRESENT_PARAMETERS,
        ppReturnedDeviceInterface: *mut *mut IDirect3DDevice9
    ) -> HRESULT
}
);
pub type LPDIRECT3D9 = *mut IDirect3D9;
pub type PDIRECT3D9 = *mut IDirect3D9;
RIDL!(
interface IDirect3DDevice9(IDirect3DDevice9Vtbl): IUnknown(IUnknownVtbl) {
    fn TestCooperativeLevel(&self) -> HRESULT,
    fn GetAvailableTextureMem(&self) -> UINT,
    fn EvictManagedResources(&self) -> HRESULT,
    fn GetDirect3D(&self, ppD3D9: *mut *mut IDirect3D9) -> HRESULT,
    fn GetDeviceCaps(&self, pCaps: *mut D3DCAPS9) -> HRESULT,
    fn GetDisplayMode(&self, iSwapChain: UINT, pMode: *mut D3DDISPLAYMODE) -> HRESULT,
    fn GetCreationParameters(
        &self, pParameters: *mut D3DDEVICE_CREATION_PARAMETERS
    ) -> HRESULT,
    fn SetCursorProperties(
        &self, XHotSpot: UINT, YHotSpot: UINT, pCursorBitmap: *mut IDirect3DSurface9
    ) -> HRESULT,
    fn SetCursorPosition(&self, X: INT, Y: INT, Flags: DWORD) -> (),
    fn ShowCursor(&self, bShow: BOOL) -> BOOL,
    fn CreateAdditionalSwapChain(
        &self, pPresentationParameters: *mut D3DPRESENT_PARAMETERS,
        pSwapChain: *mut *mut IDirect3DSwapChain9
    ) -> HRESULT,
    fn GetSwapChain(
        &self, iSwapChain: UINT, pSwapChain: *mut *mut IDirect3DSwapChain9
    ) -> HRESULT,
    fn GetNumberOfSwapChains(&self) -> UINT,
    fn Reset(&self, pPresentationParameters: *mut D3DPRESENT_PARAMETERS) -> HRESULT,
    fn Present(
        &self, pSourceRect: *const RECT, pDestRect: *const RECT,
        hDestWindowOverride: HWND, pDirtyRegion: *const RGNDATA
    ) -> HRESULT,
    fn GetBackBuffer(
        &self, iSwapChain: UINT, iBackBuffer: UINT, Type: D3DBACKBUFFER_TYPE,
        ppBackBuffer: *mut *mut IDirect3DSurface9
    ) -> HRESULT,
    fn GetRasterStatus(
        &self, iSwapChain: UINT, pRasterStatus: *mut D3DRASTER_STATUS
    ) -> HRESULT,
    fn SetDialogBoxMode(&self, bEnableDialogs: BOOL) -> HRESULT,
    fn SetGammaRamp(
        &self, iSwapChain: UINT, Flags: DWORD, pRamp: *const D3DGAMMARAMP
    ) -> (),
    fn GetGammaRamp(&self, iSwapChain: UINT, pRamp: *mut D3DGAMMARAMP) -> (),
    fn CreateTexture(
        &self, Width: UINT, Height: UINT, Levels: UINT, Usage: DWORD,
        Format: D3DFORMAT, Pool: D3DPOOL, ppTexture: *mut *mut IDirect3DTexture9,
        pSharedHandle: *mut HANDLE
    ) -> HRESULT,
    fn CreateVolumeTexture(
        &self, Width: UINT, Height: UINT, Depth: UINT, Levels: UINT, Usage: DWORD,
        Format: D3DFORMAT, Pool: D3DPOOL, ppVolumeTexture: *mut *mut IDirect3DVolumeTexture9,
        pSharedHandle: *mut HANDLE
    ) -> HRESULT,
    fn CreateCubeTexture(
        &self, EdgeLength: UINT, Levels: UINT, Usage: DWORD, Format: D3DFORMAT,
        Pool: D3DPOOL, ppCubeTexture: *mut *mut IDirect3DCubeTexture9,
        pSharedHandle: *mut HANDLE
    ) -> HRESULT,
    fn CreateVertexBuffer(
        &self, Length: UINT, Usage: DWORD, FVF: DWORD, Pool: D3DPOOL,
        ppVertexBuffer: *mut *mut IDirect3DVertexBuffer9, pSharedHandle: *mut HANDLE
    ) -> HRESULT,
    fn CreateIndexBuffer(
        &self, Length: UINT, Usage: DWORD, Format: D3DFORMAT, Pool: D3DPOOL,
        ppIndexBuffer: *mut *mut IDirect3DIndexBuffer9, pSharedHandle: *mut HANDLE
    ) -> HRESULT,
    fn CreateRenderTarget(
        &self, Width: UINT, Height: UINT, Format: D3DFORMAT,
        MultiSample: D3DMULTISAMPLE_TYPE, MultisampleQuality: DWORD, Lockable: BOOL,
        ppSurface: *mut *mut IDirect3DSurface9, pSharedHandle: *mut HANDLE
    ) -> HRESULT,
    fn CreateDepthStencilSurface(
        &self, Width: UINT, Height: UINT, Format: D3DFORMAT,
        MultiSample: D3DMULTISAMPLE_TYPE, MultisampleQuality: DWORD, Discard: BOOL,
        ppSurface: *mut *mut IDirect3DSurface9, pSharedHandle: *mut HANDLE
    ) -> HRESULT,
    fn UpdateSurface(
        &self, pSourceSurface: *mut IDirect3DSurface9, pSourceRect: *const RECT,
        pDestinationSurface: *mut IDirect3DSurface9, pDestPoint: *const POINT
    ) -> HRESULT,
    fn UpdateTexture(
        &self, pSourceTexture: *mut IDirect3DBaseTexture9,
        pDestinationTexture: *mut IDirect3DBaseTexture9
    ) -> HRESULT,
    fn GetRenderTargetData(
        &self, pRenderTarget: *mut IDirect3DSurface9, pDestSurface: *mut IDirect3DSurface9
    ) -> HRESULT,
    fn GetFrontBufferData(
        &self, iSwapChain: UINT, pDestSurface: *mut IDirect3DSurface9
    ) -> HRESULT,
    fn StretchRect(
        &self, pSourceSurface: *mut IDirect3DSurface9, pSourceRect: *const RECT,
        pDestSurface: *mut IDirect3DSurface9, pDestRect: *const RECT,
        Filter: D3DTEXTUREFILTERTYPE
    ) -> HRESULT,
    fn ColorFill(
        &self, pSurface: *mut IDirect3DSurface9, pRect: *const RECT, color: D3DCOLOR
    ) -> HRESULT,
    fn CreateOffscreenPlainSurface(
        &self, Width: UINT, Height: UINT, Format: D3DFORMAT, Pool: D3DPOOL,
        ppSurface: *mut *mut IDirect3DSurface9, pSharedHandle: *mut HANDLE
    ) -> HRESULT,
    fn SetRenderTarget(
        &self, RenderTargetIndex: DWORD, pRenderTarget: *mut IDirect3DSurface9
    ) -> HRESULT,
    fn GetRenderTarget(
        &self, RenderTargetIndex: DWORD, ppRenderTarget: *mut *mut IDirect3DSurface9
    ) -> HRESULT,
    fn SetDepthStencilSurface(&self, pNewZStencil: *mut IDirect3DSurface9) -> HRESULT,
    fn GetDepthStencilSurface(
        &self, ppZStencilSurface: *mut *mut IDirect3DSurface9
    ) -> HRESULT,
    fn BeginScene(&self) -> HRESULT,
    fn EndScene(&self) -> HRESULT,
    fn Clear(
        &self, Count: DWORD, pRects: *const D3DRECT, Flags: DWORD, Color: D3DCOLOR,
        Z: FLOAT, Stencil: DWORD
    ) -> HRESULT,
    fn SetTransform(
        &self, State: D3DTRANSFORMSTATETYPE, pMatrix: *const D3DMATRIX
    ) -> HRESULT,
    fn GetTransform(
        &self, State: D3DTRANSFORMSTATETYPE, pMatrix: *mut D3DMATRIX
    ) -> HRESULT,
    fn MultiplyTransform(
        &self, arg1: D3DTRANSFORMSTATETYPE, arg2: *const D3DMATRIX
    ) -> HRESULT,
    fn SetViewport(&self, pViewport: *const D3DVIEWPORT9) -> HRESULT,
    fn GetViewport(&self, pViewport: *mut D3DVIEWPORT9) -> HRESULT,
    fn SetMaterial(&self, pMaterial: *const D3DMATERIAL9) -> HRESULT,
    fn GetMaterial(&self, pMaterial: *mut D3DMATERIAL9) -> HRESULT,
    fn SetLight(&self, Index: DWORD, arg1: *const D3DLIGHT9) -> HRESULT,
    fn GetLight(&self, Index: DWORD, arg1: *mut D3DLIGHT9) -> HRESULT,
    fn LightEnable(&self, Index: DWORD, Enable: BOOL) -> HRESULT,
    fn GetLightEnable(&self, Index: DWORD, pEnable: *mut BOOL) -> HRESULT,
    fn SetClipPlane(&self, Index: DWORD, pPlane: *const FLOAT) -> HRESULT,
    fn GetClipPlane(&self, Index: DWORD, pPlane: *mut FLOAT) -> HRESULT,
    fn SetRenderState(&self, State: D3DRENDERSTATETYPE, Value: DWORD) -> HRESULT,
    fn GetRenderState(&self, State: D3DRENDERSTATETYPE, pValue: *mut DWORD) -> HRESULT,
    fn CreateStateBlock(
        &self, Type: D3DSTATEBLOCKTYPE, ppSB: *mut *mut IDirect3DStateBlock9
    ) -> HRESULT,
    fn BeginStateBlock(&self) -> HRESULT,
    fn EndStateBlock(&self, ppSB: *mut *mut IDirect3DStateBlock9) -> HRESULT,
    fn SetClipStatus(&self, pClipStatus: *const D3DCLIPSTATUS9) -> HRESULT,
    fn GetClipStatus(&self, pClipStatus: *mut D3DCLIPSTATUS9) -> HRESULT,
    fn GetTexture(
        &self, Stage: DWORD, ppTexture: *mut *mut IDirect3DBaseTexture9
    ) -> HRESULT,
    fn SetTexture(&self, Stage: DWORD, pTexture: *mut IDirect3DBaseTexture9) -> HRESULT,
    fn GetTextureStageState(
        &self, Stage: DWORD, Type: D3DTEXTURESTAGESTATETYPE, pValue: *mut DWORD
    ) -> HRESULT,
    fn SetTextureStageState(
        &self, Stage: DWORD, Type: D3DTEXTURESTAGESTATETYPE, Value: DWORD
    ) -> HRESULT,
    fn GetSamplerState(
        &self, Sampler: DWORD, Type: D3DSAMPLERSTATETYPE, pValue: *mut DWORD
    ) -> HRESULT,
    fn SetSamplerState(
        &self, Sampler: DWORD, Type: D3DSAMPLERSTATETYPE, Value: DWORD
    ) -> HRESULT,
    fn ValidateDevice(&self, pNumPasses: *mut DWORD) -> HRESULT,
    fn SetPaletteEntries(
        &self, PaletteNumber: UINT, pEntries: *const PALETTEENTRY
    ) -> HRESULT,
    fn GetPaletteEntries(
        &self, PaletteNumber: UINT, pEntries: *mut PALETTEENTRY
    ) -> HRESULT,
    fn SetCurrentTexturePalette(&self, PaletteNumber: UINT) -> HRESULT,
    fn GetCurrentTexturePalette(&self, PaletteNumber: *mut UINT) -> HRESULT,
    fn SetScissorRect(&self, pRect: *const RECT) -> HRESULT,
    fn GetScissorRect(&self, pRect: *mut RECT) -> HRESULT,
    fn SetSoftwareVertexProcessing(&self, bSoftware: BOOL) -> HRESULT,
    fn GetSoftwareVertexProcessing(&self) -> BOOL,
    fn SetNPatchMode(&self, nSegments: FLOAT) -> HRESULT,
    fn GetNPatchMode(&self) -> FLOAT,
    fn DrawPrimitive(
        &self, PrimitiveType: D3DPRIMITIVETYPE, StartVertex: UINT, PrimitiveCount: UINT
    ) -> HRESULT,
    fn DrawIndexedPrimitive(
        &self, arg1: D3DPRIMITIVETYPE, BaseVertexIndex: INT, MinVertexIndex: UINT,
        NumVertices: UINT, startIndex: UINT, primCount: UINT
    ) -> HRESULT,
    fn DrawPrimitiveUP(
        &self, PrimitiveType: D3DPRIMITIVETYPE, PrimitiveCount: UINT,
        pVertexStreamZeroData: *const VOID, VertexStreamZeroStride: UINT
    ) -> HRESULT,
    fn DrawIndexedPrimitiveUP(
        &self, PrimitiveType: D3DPRIMITIVETYPE, MinVertexIndex: UINT, NumVertices: UINT,
        PrimitiveCount: UINT, pIndexData: *const VOID, IndexDataFormat: D3DFORMAT,
        pVertexStreamZeroData: *const VOID, VertexStreamZeroStride: UINT
    ) -> HRESULT,
    fn ProcessVertices(
        &self, SrcStartIndex: UINT, DestIndex: UINT, VertexCount: UINT,
        pDestBuffer: *mut IDirect3DVertexBuffer9, pVertexDecl: *mut IDirect3DVertexDeclaration9,
        Flags: DWORD
    ) -> HRESULT,
    fn CreateVertexDeclaration(
        &self, pVertexElements: *const D3DVERTEXELEMENT9,
        ppDecl: *mut *mut IDirect3DVertexDeclaration9
    ) -> HRESULT,
    fn SetVertexDeclaration(&self, pDecl: *mut IDirect3DVertexDeclaration9) -> HRESULT,
    fn GetVertexDeclaration(&self, ppDecl: *mut *mut IDirect3DVertexDeclaration9) -> HRESULT,
    fn SetFVF(&self, FVF: DWORD) -> HRESULT,
    fn GetFVF(&self, pFVF: *mut DWORD) -> HRESULT,
    fn CreateVertexShader(
        &self, pFunction: *const DWORD, ppShader: *mut *mut IDirect3DVertexShader9
    ) -> HRESULT,
    fn SetVertexShader(&self, pShader: *mut IDirect3DVertexShader9) -> HRESULT,
    fn GetVertexShader(&self, ppShader: *mut *mut IDirect3DVertexShader9) -> HRESULT,
    fn SetVertexShaderConstantF(
        &self, StartRegister: UINT, pConstantData: *const FLOAT, Vector4fCount: UINT
    ) -> HRESULT,
    fn GetVertexShaderConstantF(
        &self, StartRegister: UINT, pConstantData: *mut FLOAT, Vector4fCount: UINT
    ) -> HRESULT,
    fn SetVertexShaderConstantI(
        &self, StartRegister: UINT, pConstantData: *const INT, Vector4iCount: UINT
    ) -> HRESULT,
    fn GetVertexShaderConstantI(
        &self, StartRegister: UINT, pConstantData: *mut INT, Vector4iCount: UINT
    ) -> HRESULT,
    fn SetVertexShaderConstantB(
        &self, StartRegister: UINT, pConstantData: *const BOOL, BoolCount: UINT
    ) -> HRESULT,
    fn GetVertexShaderConstantB(
        &self, StartRegister: UINT, pConstantData: *mut BOOL, BoolCount: UINT
    ) -> HRESULT,
    fn SetStreamSource(
        &self, StreamNumber: UINT, pStreamData: *mut IDirect3DVertexBuffer9,
        OffsetInBytes: UINT, Stride: UINT
    ) -> HRESULT,
    fn GetStreamSource(
        &self, StreamNumber: UINT, ppStreamData: *mut *mut IDirect3DVertexBuffer9,
        pOffsetInBytes: *mut UINT, pStride: *mut UINT
    ) -> HRESULT,
    fn SetStreamSourceFreq(&self, StreamNumber: UINT, Setting: UINT) -> HRESULT,
    fn GetStreamSourceFreq(&self, StreamNumber: UINT, pSetting: *mut UINT) -> HRESULT,
    fn SetIndices(&self, pIndexData: *mut IDirect3DIndexBuffer9) -> HRESULT,
    fn GetIndices(&self, ppIndexData: *mut *mut IDirect3DIndexBuffer9) -> HRESULT,
    fn CreatePixelShader(
        &self, pFunction: *const DWORD, ppShader: *mut *mut IDirect3DPixelShader9
    ) -> HRESULT,
    fn SetPixelShader(&self, pShader: *mut IDirect3DPixelShader9) -> HRESULT,
    fn GetPixelShader(&self, ppShader: *mut *mut IDirect3DPixelShader9) -> HRESULT,
    fn SetPixelShaderConstantF(
        &self, StartRegister: UINT, pConstantData: *const FLOAT, Vector4fCount: UINT
    ) -> HRESULT,
    fn GetPixelShaderConstantF(
        &self, StartRegister: UINT, pConstantData: *mut FLOAT, Vector4fCount: UINT
    ) -> HRESULT,
    fn SetPixelShaderConstantI(
        &self, StartRegister: UINT, pConstantData: *const INT, Vector4iCount: UINT
    ) -> HRESULT,
    fn GetPixelShaderConstantI(
        &self, StartRegister: UINT, pConstantData: *mut INT, Vector4iCount: UINT
    ) -> HRESULT,
    fn SetPixelShaderConstantB(
        &self, StartRegister: UINT, pConstantData: *const BOOL, BoolCount: UINT
    ) -> HRESULT,
    fn GetPixelShaderConstantB(
        &self, StartRegister: UINT, pConstantData: *mut BOOL, BoolCount: UINT
    ) -> HRESULT,
    fn DrawRectPatch(
        &self, Handle: UINT, pNumSegs: *const FLOAT,
        pRectPatchInfo: *const D3DRECTPATCH_INFO
    ) -> HRESULT,
    fn DrawTriPatch(
        &self, Handle: UINT, pNumSegs: *const FLOAT,
        pTriPatchInfo: *const D3DTRIPATCH_INFO
    ) -> HRESULT,
    fn DeletePatch(&self, Handle: UINT) -> HRESULT,
    fn CreateQuery(
        &self, Type: D3DQUERYTYPE, ppQuery: *mut *mut IDirect3DQuery9
    ) -> HRESULT
}
);
pub type LPDIRECT3DDEVICE9 = *mut IDirect3DDevice9;
pub type PDIRECT3DDEVICE9 = *mut IDirect3DDevice9;
RIDL!(
interface IDirect3DStateBlock9(IDirect3DStateBlock9Vtbl): IUnknown(IUnknownVtbl) {
    fn GetDevice(&self, ppDevice: *mut *mut IDirect3DDevice9) -> HRESULT,
    fn Capture(&self) -> HRESULT,
    fn Apply(&self) -> HRESULT
}
);
pub type LPDIRECT3DSTATEBLOCK9 = *mut IDirect3DStateBlock9;
pub type PDIRECT3DSTATEBLOCK9 = *mut IDirect3DStateBlock9;
RIDL!(
interface IDirect3DSwapChain9(IDirect3DSwapChain9Vtbl): IUnknown(IUnknownVtbl) {
    fn Present(
        &self, pSourceRect: *const RECT, pDestRect: *const RECT,
        hDestWindowOverride: HWND, pDirtyRegion: *const RGNDATA, dwFlags: DWORD
    ) -> HRESULT,
    fn GetFrontBufferData(&self, pDestSurface: *mut IDirect3DSurface9) -> HRESULT,
    fn GetBackBuffer(
        &self, iBackBuffer: UINT, Type: D3DBACKBUFFER_TYPE,
        ppBackBuffer: *mut *mut IDirect3DSurface9
    ) -> HRESULT,
    fn GetRasterStatus(&self, pRasterStatus: *mut D3DRASTER_STATUS) -> HRESULT,
    fn GetDisplayMode(&self, pMode: *mut D3DDISPLAYMODE) -> HRESULT,
    fn GetDevice(&self, ppDevice: *mut *mut IDirect3DDevice9) -> HRESULT,
    fn GetPresentParameters(
        &self, pPresentationParameters: *mut D3DPRESENT_PARAMETERS
    ) -> HRESULT
}
);
pub type LPDIRECT3DSWAPCHAIN9 = *mut IDirect3DSwapChain9;
pub type PDIRECT3DSWAPCHAIN9 = *mut IDirect3DSwapChain9;
RIDL!(
interface IDirect3DResource9(IDirect3DResource9Vtbl): IUnknown(IUnknownVtbl) {
    fn GetDevice(&self, ppDevice: *mut *mut IDirect3DDevice9) -> HRESULT,
    fn SetPrivateData(
        &self, refguid: *const GUID, pData: *const VOID, SizeOfData: DWORD,
        Flags: DWORD
    ) -> HRESULT,
    fn GetPrivateData(
        &self, refguid: *const GUID, pData: *mut VOID, pSizeOfData: *mut DWORD
    ) -> HRESULT,
    fn FreePrivateData(&self, refguid: *const GUID) -> HRESULT,
    fn SetPriority(&self, PriorityNew: DWORD) -> DWORD,
    fn GetPriority(&self) -> DWORD,
    fn PreLoad(&self) -> (),
    fn GetType(&self) -> D3DRESOURCETYPE
}
);
pub type LPDIRECT3DRESOURCE9 = *mut IDirect3DResource9;
pub type PDIRECT3DRESOURCE9 = *mut IDirect3DResource9;
RIDL!(
interface IDirect3DVertexDeclaration9(IDirect3DVertexDeclaration9Vtbl): IUnknown(IUnknownVtbl) {
    fn GetDevice(&self, ppDevice: *mut *mut IDirect3DDevice9) -> HRESULT,
    fn GetDeclaration(
        &self, pElement: *mut D3DVERTEXELEMENT9, pNumElements: *mut UINT
    ) -> HRESULT
}
);
pub type LPDIRECT3DVERTEXDECLARATION9 = *mut IDirect3DVertexDeclaration9;
pub type PDIRECT3DVERTEXDECLARATION9 = *mut IDirect3DVertexDeclaration9;
RIDL!(
interface IDirect3DVertexShader9(IDirect3DVertexShader9Vtbl): IUnknown(IUnknownVtbl) {
    fn GetDevice(&self, ppDevice: *mut *mut IDirect3DDevice9) -> HRESULT,
    fn GetFunction(&self, arg1: *mut VOID, pSizeOfData: *mut UINT) -> HRESULT
}
);
pub type LPDIRECT3DVERTEXSHADER9 = *mut IDirect3DVertexShader9;
pub type PDIRECT3DVERTEXSHADER9 = *mut IDirect3DVertexShader9;
RIDL!(
interface IDirect3DPixelShader9(IDirect3DPixelShader9Vtbl): IUnknown(IUnknownVtbl) {
    fn GetDevice(&self, ppDevice: *mut *mut IDirect3DDevice9) -> HRESULT,
    fn GetFunction(&self, arg1: *mut VOID, pSizeOfData: *mut UINT) -> HRESULT
}
);
pub type LPDIRECT3DPIXELSHADER9 = *mut IDirect3DPixelShader9;
pub type PDIRECT3DPIXELSHADER9 = *mut IDirect3DPixelShader9;
RIDL!(
interface IDirect3DBaseTexture9(IDirect3DBaseTexture9Vtbl): IDirect3DResource9(IDirect3DResource9Vtbl) {
    fn SetLOD(&self, LODNew: DWORD) -> DWORD,
    fn GetLOD(&self) -> DWORD,
    fn GetLevelCount(&self) -> DWORD,
    fn SetAutoGenFilterType(&self, FilterType: D3DTEXTUREFILTERTYPE) -> HRESULT,
    fn GetAutoGenFilterType(&self) -> D3DTEXTUREFILTERTYPE,
    fn GenerateMipSubLevels(&self) -> ()
}
);
pub type LPDIRECT3DBASETEXTURE9 = *mut IDirect3DBaseTexture9;
pub type PDIRECT3DBASETEXTURE9 = *mut IDirect3DBaseTexture9;
RIDL!(
interface IDirect3DTexture9(IDirect3DTexture9Vtbl): IDirect3DBaseTexture9(IDirect3DBaseTexture9Vtbl) {
    fn GetLevelDesc(&self, Level: UINT, pDesc: *mut D3DSURFACE_DESC) -> HRESULT,
    fn GetSurfaceLevel(
        &self, Level: UINT, ppSurfaceLevel: *mut *mut IDirect3DSurface9
    ) -> HRESULT,
    fn LockRect(
        &self, Level: UINT, pLockedRect: *mut D3DLOCKED_RECT, pRect: *const RECT,
        Flags: DWORD
    ) -> HRESULT,
    fn UnlockRect(&self, Level: UINT) -> HRESULT,
    fn AddDirtyRect(&self, pDirtyRect: *const RECT) -> HRESULT
}
);
pub type LPDIRECT3DTEXTURE9 = *mut IDirect3DTexture9;
pub type PDIRECT3DTEXTURE9 = *mut IDirect3DTexture9;
RIDL!(
interface IDirect3DVolumeTexture9(IDirect3DVolumeTexture9Vtbl): IDirect3DBaseTexture9(IDirect3DBaseTexture9Vtbl) {
    fn GetLevelDesc(&self, Level: UINT, pDesc: *mut D3DVOLUME_DESC) -> HRESULT,
    fn GetVolumeLevel(
        &self, Level: UINT, ppVolumeLevel: *mut *mut IDirect3DVolume9
    ) -> HRESULT,
    fn LockBox(
        &self, Level: UINT, pLockedVolume: *mut D3DLOCKED_BOX, pBox: *const D3DBOX,
        Flags: DWORD
    ) -> HRESULT,
    fn UnlockBox(&self, Level: UINT) -> HRESULT,
    fn AddDirtyBox(&self, pDirtyBox: *const D3DBOX) -> HRESULT
}
);
pub type LPDIRECT3DVOLUMETEXTURE9 = *mut IDirect3DVolumeTexture9;
pub type PDIRECT3DVOLUMETEXTURE9 = *mut IDirect3DVolumeTexture9;
RIDL!(
interface IDirect3DCubeTexture9(IDirect3DCubeTexture9Vtbl): IDirect3DBaseTexture9(IDirect3DBaseTexture9Vtbl) {
    fn GetLevelDesc(&self, Level: UINT, pDesc: *mut D3DSURFACE_DESC) -> HRESULT,
    fn GetCubeMapSurface(
        &self, FaceType: D3DCUBEMAP_FACES, Level: UINT,
        ppCubeMapSurface: *mut *mut IDirect3DSurface9
    ) -> HRESULT,
    fn LockRect(
        &self, FaceType: D3DCUBEMAP_FACES, Level: UINT, pLockedRect: *mut D3DLOCKED_RECT,
        pRect: *const RECT, Flags: DWORD
    ) -> HRESULT,
    fn UnlockRect(&self, FaceType: D3DCUBEMAP_FACES, Level: UINT) -> HRESULT,
    fn AddDirtyRect(
        &self, FaceType: D3DCUBEMAP_FACES, pDirtyRect: *const RECT
    ) -> HRESULT
}
);
pub type LPDIRECT3DCUBETEXTURE9 = *mut IDirect3DCubeTexture9;
pub type PDIRECT3DCUBETEXTURE9 = *mut IDirect3DCubeTexture9;
RIDL!(
interface IDirect3DVertexBuffer9(IDirect3DVertexBuffer9Vtbl): IDirect3DResource9(IDirect3DResource9Vtbl) {
    fn Lock(
        &self, OffsetToLock: UINT, SizeToLock: UINT, ppbData: *mut *mut VOID,
        Flags: DWORD
    ) -> HRESULT,
    fn Unlock(&self) -> HRESULT,
    fn GetDesc(&self, pDesc: *mut D3DVERTEXBUFFER_DESC) -> HRESULT
}
);
pub type LPDIRECT3DVERTEXBUFFER9 = *mut IDirect3DVertexBuffer9;
pub type PDIRECT3DVERTEXBUFFER9 = *mut IDirect3DVertexBuffer9;
RIDL!(
interface IDirect3DIndexBuffer9(IDirect3DIndexBuffer9Vtbl): IDirect3DResource9(IDirect3DResource9Vtbl) {
    fn Lock(
        &self, OffsetToLock: UINT, SizeToLock: UINT, ppbData: *mut *mut VOID,
        Flags: DWORD
    ) -> HRESULT,
    fn Unlock(&self) -> HRESULT,
    fn GetDesc(&self, pDesc: *mut D3DINDEXBUFFER_DESC) -> HRESULT
}
);
pub type LPDIRECT3DINDEXBUFFER9 = *mut IDirect3DIndexBuffer9;
pub type PDIRECT3DINDEXBUFFER9 = *mut IDirect3DIndexBuffer9;
RIDL!(
interface IDirect3DSurface9(IDirect3DSurface9Vtbl): IDirect3DResource9(IDirect3DResource9Vtbl) {
    fn GetContainer(&self, riid: *const IID, ppContainer: *mut *mut VOID) -> HRESULT,
    fn GetDesc(&self, pDesc: *mut D3DSURFACE_DESC) -> HRESULT,
    fn LockRect(
        &self, pLockedRect: *mut D3DLOCKED_RECT, pRect: *const RECT, Flags: DWORD
    ) -> HRESULT,
    fn UnlockRect(&self) -> HRESULT,
    fn GetDC(&self, phdc: *mut HDC) -> HRESULT,
    fn ReleaseDC(&self, hdc: HDC) -> HRESULT
}
);
pub type LPDIRECT3DSURFACE9 = *mut IDirect3DSurface9;
pub type PDIRECT3DSURFACE9 = *mut IDirect3DSurface9;
RIDL!(
interface IDirect3DVolume9(IDirect3DVolume9Vtbl): IUnknown(IUnknownVtbl) {
    fn GetDevice(&self, ppDevice: *mut *mut IDirect3DDevice9) -> HRESULT,
    fn SetPrivateData(
        &self, refguid: *const GUID, pData: *const VOID, SizeOfData: DWORD,
        Flags: DWORD
    ) -> HRESULT,
    fn GetPrivateData(
        &self, refguid: *const GUID, pData: *mut VOID, pSizeOfData: *mut DWORD
    ) -> HRESULT,
    fn FreePrivateData(&self, refguid: *const GUID) -> HRESULT,
    fn GetContainer(&self, riid: *const IID, ppContainer: *mut *mut VOID) -> HRESULT,
    fn GetDesc(&self, pDesc: *mut D3DVOLUME_DESC) -> HRESULT,
    fn LockBox(
        &self, pLockedVolume: *mut D3DLOCKED_BOX, pBox: *const D3DBOX, Flags: DWORD
    ) -> HRESULT,
    fn UnlockBox(&self) -> HRESULT
}
);
pub type LPDIRECT3DVOLUME9 = *mut IDirect3DVolume9;
pub type PDIRECT3DVOLUME9 = *mut IDirect3DVolume9;
RIDL!(
interface IDirect3DQuery9(IDirect3DQuery9Vtbl): IUnknown(IUnknownVtbl) {
    fn GetDevice(&self, ppDevice: *mut *mut IDirect3DDevice9) -> HRESULT,
    fn GetType(&self) -> D3DRESOURCETYPE,
    fn GetDataSize(&self) -> DWORD,
    fn Issue(&self, dwIssueFlags: DWORD) -> HRESULT,
    fn GetData(
        &self, pData: *mut VOID, dwSize: DWORD, dwGetDataFlags: DWORD
    ) -> HRESULT
}
);
pub type LPDIRECT3DQUERY9 = *mut IDirect3DQuery9;
pub type PDIRECT3DQUERY9 = *mut IDirect3DQuery9;
pub const D3DCREATE_FPU_PRESERVE: DWORD = 0x2;
pub const D3DCREATE_MULTITHREADED: DWORD = 0x4;
pub const D3DCREATE_PUREDEVICE: DWORD = 0x10;
pub const D3DCREATE_SOFTWARE_VERTEXPROCESSING: DWORD = 0x20;
pub const D3DCREATE_HARDWARE_VERTEXPROCESSING: DWORD = 0x40;
pub const D3DCREATE_MIXED_VERTEXPROCESSING: DWORD = 0x80;
pub const D3DCREATE_DISABLE_DRIVER_MANAGEMENT: DWORD = 0x100;
pub const D3DCREATE_ADAPTERGROUP_DEVICE: DWORD = 0x200;
pub const D3DCREATE_DISABLE_DRIVER_MANAGEMENT_EX: DWORD = 0x400;
pub const D3DCREATE_NOWINDOWCHANGES: DWORD = 0x800;
pub const D3DCREATE_DISABLE_PSGP_THREADING: DWORD = 0x2000;
pub const D3DCREATE_ENABLE_PRESENTSTATS: DWORD = 0x4000;
pub const D3DCREATE_DISABLE_PRESENTSTATS: DWORD = 0x8000;
pub const D3DCREATE_SCREENSAVER: DWORD = 0x10000000;
pub const D3DADAPTER_DEFAULT: DWORD = 0;
RIDL!(
interface IDirect3D9Ex(IDirect3D9ExVtbl): IDirect3D9(IDirect3D9Vtbl) {
    fn GetAdapterModeCountEx(
        &self, Adapter: UINT, pFilter: *const D3DDISPLAYMODEFILTER
    ) -> UINT,
    fn EnumAdapterModesEx(
        &self, Adapter: UINT, pFilter: *const D3DDISPLAYMODEFILTER, Mode: UINT,
        pMode: *mut D3DDISPLAYMODEEX
    ) -> HRESULT,
    fn GetAdapterDisplayModeEx(
        &self, Adapter: UINT, pMode: *mut D3DDISPLAYMODEEX,
        pRotation: *mut D3DDISPLAYROTATION
    ) -> HRESULT,
    fn CreateDeviceEx(
        &self, Adapter: UINT, DeviceType: D3DDEVTYPE, hFocusWindow: HWND,
        BehaviorFlags: DWORD, pPresentationParameters: *mut D3DPRESENT_PARAMETERS,
        pFullscreenDisplayMode: *mut D3DDISPLAYMODEEX,
        ppReturnedDeviceInterface: *mut *mut IDirect3DDevice9Ex
    ) -> HRESULT,
    fn GetAdapterLUID(&self, Adapter: UINT, pLUID: *mut LUID) -> HRESULT
}
);
pub type LPDIRECT3D9EX = *mut IDirect3D9Ex;
pub type PDIRECT3D9EX = *mut IDirect3D9Ex;
RIDL!(
interface IDirect3DDevice9Ex(IDirect3DDevice9ExVtbl): IDirect3DDevice9(IDirect3DDevice9Vtbl) {
    fn SetConvolutionMonoKernel(
        &self, width: UINT, height: UINT, rows: *mut FLOAT, columns: *mut FLOAT
    ) -> HRESULT,
    fn ComposeRects(
        &self, pSrc: *mut IDirect3DSurface9, pDst: *mut IDirect3DSurface9,
        pSrcRectDescs: *mut IDirect3DVertexBuffer9, NumRects: UINT,
        pDstRectDescs: *mut IDirect3DVertexBuffer9, Operation: D3DCOMPOSERECTSOP, Xoffset: INT,
        Yoffset: INT
    ) -> HRESULT,
    fn PresentEx(
        &self, pSourceRect: *const RECT, pDestRect: *const RECT,
        hDestWindowOverride: HWND, pDirtyRegion: *const RGNDATA, dwFlags: DWORD
    ) -> HRESULT,
    fn GetGPUThreadPriority(&self, pPriority: *mut INT) -> HRESULT,
    fn SetGPUThreadPriority(&self, Priority: INT) -> HRESULT,
    fn WaitForVBlank(&self, iSwapChain: UINT) -> HRESULT,
    fn CheckResourceResidency(
        &self, pResourceArray: *mut *mut IDirect3DResource9, NumResources: UINT32
    ) -> HRESULT,
    fn SetMaximumFrameLatency(&self, MaxLatency: UINT) -> HRESULT,
    fn GetMaximumFrameLatency(&self, pMaxLatency: *mut UINT) -> HRESULT,
    fn CheckDeviceState(&self, hDestinationWindow: HWND) -> HRESULT,
    fn CreateRenderTargetEx(
        &self, Width: UINT, Height: UINT, Format: D3DFORMAT,
        MultiSample: D3DMULTISAMPLE_TYPE, MultisampleQuality: DWORD, Lockable: BOOL,
        ppSurface: *mut *mut IDirect3DSurface9, pSharedHandle: *mut HANDLE, Usage: DWORD
    ) -> HRESULT,
    fn CreateOffscreenPlainSurfaceEx(
        &self, Width: UINT, Height: UINT, Format: D3DFORMAT, Pool: D3DPOOL,
        ppSurface: *mut *mut IDirect3DSurface9, pSharedHandle: *mut HANDLE, Usage: DWORD
    ) -> HRESULT,
    fn CreateDepthStencilSurfaceEx(
        &self, Width: UINT, Height: UINT, Format: D3DFORMAT,
        MultiSample: D3DMULTISAMPLE_TYPE, MultisampleQuality: DWORD, Discard: BOOL,
        ppSurface: *mut *mut IDirect3DSurface9, pSharedHandle: *mut HANDLE, Usage: DWORD
    ) -> HRESULT,
    fn ResetEx(
        &self, pPresentationParameters: *mut D3DPRESENT_PARAMETERS,
        pFullscreenDisplayMode: *mut D3DDISPLAYMODEEX
    ) -> HRESULT,
    fn GetDisplayModeEx(
        &self, iSwapChain: UINT, pMode: *mut D3DDISPLAYMODEEX,
        pRotation: *mut D3DDISPLAYROTATION
    ) -> HRESULT
}
);
pub type LPDIRECT3DDEVICE9EX = *mut IDirect3DDevice9Ex;
pub type PDIRECT3DDEVICE9EX = *mut IDirect3DDevice9Ex;
RIDL!(
interface IDirect3DSwapChain9Ex(IDirect3DSwapChain9ExVtbl): IDirect3DSwapChain9(IDirect3DSwapChain9Vtbl) {
    fn GetLastPresentCount(&self, pLastPresentCount: *mut UINT) -> HRESULT,
    fn GetPresentStats(&self, pPresentationStatistics: *mut D3DPRESENTSTATS) -> HRESULT,
    fn GetDisplayModeEx(
        &self, pMode: *mut D3DDISPLAYMODEEX, pRotation: *mut D3DDISPLAYROTATION
    ) -> HRESULT
}
);
pub type LPDIRECT3DSWAPCHAIN9EX = *mut IDirect3DSwapChain9Ex;
pub type PDIRECT3DSWAPCHAIN9EX = *mut IDirect3DSwapChain9Ex;
RIDL!(
interface IDirect3D9ExOverlayExtension(IDirect3D9ExOverlayExtensionVtbl): IUnknown(IUnknownVtbl) {
    fn CheckDeviceOverlayType(
        &self, Adapter: UINT, DevType: D3DDEVTYPE, OverlayWidth: UINT,
        OverlayHeight: UINT, OverlayFormat: D3DFORMAT, pDisplayMode: *mut D3DDISPLAYMODEEX,
        DisplayRotation: D3DDISPLAYROTATION, pOverlayCaps: *mut D3DOVERLAYCAPS
    ) -> HRESULT
}
);
pub type LPDIRECT3D9EXOVERLAYEXTENSION = *mut IDirect3D9ExOverlayExtension;
pub type PDIRECT3D9EXOVERLAYEXTENSION = *mut IDirect3D9ExOverlayExtension;
RIDL!(
interface IDirect3DDevice9Video(IDirect3DDevice9VideoVtbl): IUnknown(IUnknownVtbl) {
    fn GetContentProtectionCaps(
        &self, pCryptoType: *const GUID, pDecodeProfile: *const GUID,
        pCaps: *mut D3DCONTENTPROTECTIONCAPS
    ) -> HRESULT,
    fn CreateAuthenticatedChannel(
        &self, ChannelType: D3DAUTHENTICATEDCHANNELTYPE,
        ppAuthenticatedChannel: *mut *mut IDirect3DAuthenticatedChannel9,
        pChannelHandle: *mut HANDLE
    ) -> HRESULT,
    fn CreateCryptoSession(
        &self, pCryptoType: *const GUID, pDecodeProfile: *const GUID,
        ppCryptoSession: *mut *mut IDirect3DCryptoSession9, pCryptoHandle: *mut HANDLE
    ) -> HRESULT
}
);
pub type LPDIRECT3DDEVICE9VIDEO = *mut IDirect3DDevice9Video;
pub type PDIRECT3DDEVICE9VIDEO = *mut IDirect3DDevice9Video;
RIDL!(
interface IDirect3DAuthenticatedChannel9(IDirect3DAuthenticatedChannel9Vtbl): IUnknown(IUnknownVtbl) {
    fn GetCertificateSize(&self, pCertificateSize: *mut UINT) -> HRESULT,
    fn GetCertificate(&self, CertifacteSize: UINT, ppCertificate: *mut BYTE) -> HRESULT,
    fn NegotiateKeyExchange(&self, DataSize: UINT, pData: *mut VOID) -> HRESULT,
    fn Query(
        &self, InputSize: UINT, pInput: *const VOID, OutputSize: UINT,
        pOutput: *mut VOID
    ) -> HRESULT,
    fn Configure(
        &self, InputSize: UINT, pInput: *const VOID,
        pOutput: *mut D3DAUTHENTICATEDCHANNEL_CONFIGURE_OUTPUT
    ) -> HRESULT
}
);
pub type LPDIRECT3DAUTHENTICATEDCHANNEL9 = *mut IDirect3DAuthenticatedChannel9;
pub type PDIRECT3DAUTHENTICATEDCHANNEL9 = *mut IDirect3DAuthenticatedChannel9;
RIDL!(
interface IDirect3DCryptoSession9(IDirect3DCryptoSession9Vtbl): IUnknown(IUnknownVtbl) {
    fn GetCertificateSize(&self, pCertificateSize: *mut UINT) -> HRESULT,
    fn GetCertificate(&self, CertifacteSize: UINT, ppCertificate: *mut BYTE) -> HRESULT,
    fn NegotiateKeyExchange(&self, DataSize: UINT, pData: *mut VOID) -> HRESULT,
    fn EncryptionBlt(
        &self, pSrcSurface: *mut IDirect3DSurface9, pDstSurface: *mut IDirect3DSurface9,
        DstSurfaceSize: UINT, pIV: *mut VOID
    ) -> HRESULT,
    fn DecryptionBlt(
        &self, pSrcSurface: *mut IDirect3DSurface9, pDstSurface: *mut IDirect3DSurface9,
        SrcSurfaceSize: UINT, pEncryptedBlockInfo: *mut D3DENCRYPTED_BLOCK_INFO,
        pContentKey: *mut VOID, pIV: *mut VOID
    ) -> HRESULT,
    fn GetSurfacePitch(
        &self, pSrcSurface: *mut IDirect3DSurface9, pSurfacePitch: *mut UINT
    ) -> HRESULT,
    fn StartSessionKeyRefresh(
        &self, pRandomNumber: *mut VOID, RandomNumberSize: UINT
    ) -> HRESULT,
    fn FinishSessionKeyRefresh(&self) -> HRESULT,
    fn GetEncryptionBltKey(&self, pReadbackKey: *mut VOID, KeySize: UINT) -> HRESULT
}
);
pub type LPDIRECT3DCRYPTOSESSION9 = *mut IDirect3DCryptoSession9;
pub type PDIRECT3DCRYPTOSESSION9 = *mut IDirect3DCryptoSession9;

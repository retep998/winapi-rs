// Copyright © 2015, Corey Richardson
// Licensed under the MIT License <LICENSE.md>
//! Direct3D include file
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IDirect3D9 {
    pub lpVtbl: *mut IDirect3D9Vtbl,
}
#[repr(C)]
pub struct IDirect3D9Vtbl {
    pub QueryInterface: Option<unsafe extern "system" fn(
        This: *mut IDirect3D9, riid: *const ::IID, ppvObj: *mut *mut ::VOID,
    ) -> ::HRESULT>,
    pub AddRef: Option<unsafe extern "system" fn(This: *mut IDirect3D9) -> *mut IDirect3D9>,
    pub Release: Option<unsafe extern "system" fn(This: *mut IDirect3D9) -> *mut IDirect3D9>,
    pub RegisterSoftwareDevice: Option<unsafe extern "system" fn(
        This: *mut IDirect3D9, pInitializeFunction: *mut ::VOID,
    ) -> ::HRESULT>,
    pub GetAdapterCount: Option<unsafe extern "system" fn(
        This: *mut IDirect3D9,
    ) -> *mut IDirect3D9>,
    pub GetAdapterIdentifier: Option<unsafe extern "system" fn(
        This: *mut IDirect3D9, Adapter: ::UINT, Flags: ::DWORD,
        pIdentifier: *mut ::D3DADAPTER_IDENTIFIER9,
    ) -> ::HRESULT>,
    pub GetAdapterModeCount: Option<unsafe extern "system" fn(
        This: *mut IDirect3D9, Adapter: ::UINT, Format: ::D3DFORMAT,
    ) -> ::UINT>,
    pub EnumAdapterModes: Option<unsafe extern "system" fn(
        This: *mut IDirect3D9, Adapter: ::UINT, Format: ::D3DFORMAT, Mode: ::UINT,
        pMode: *mut ::D3DDISPLAYMODE,
    ) -> ::HRESULT>,
    pub GetAdapterDisplayMode: Option<unsafe extern "system" fn(
        This: *mut IDirect3D9, Adapter: ::UINT, pMode: *mut ::D3DDISPLAYMODE,
    ) -> ::HRESULT>,
    pub CheckDeviceType: Option<unsafe extern "system" fn(
        This: *mut IDirect3D9, Adapter: ::UINT, DevType: ::D3DDEVTYPE, AdapterFormat: ::D3DFORMAT,
        BackBufferFormat: ::D3DFORMAT, bWindowed: ::BOOL,
    ) -> ::HRESULT>,
    pub CheckDeviceFormat: Option<unsafe extern "system" fn(
        This: *mut IDirect3D9, Adapter: ::UINT, DeviceType: ::D3DDEVTYPE,
        AdapterFormat: ::D3DFORMAT, Usage: ::DWORD, RType: ::D3DRESOURCETYPE,
        CheckFormat: ::D3DFORMAT,
    ) -> ::HRESULT>,
    pub CheckDeviceMultiSampleType: Option<unsafe extern "system" fn(
        This: *mut IDirect3D9, Adapter: ::UINT, DeviceType: ::D3DDEVTYPE,
        SurfaceFormat: ::D3DFORMAT, Windowed: ::BOOL, MultiSampleType: ::D3DMULTISAMPLE_TYPE,
        pQualityLevels: *mut ::DWORD,
    ) -> ::HRESULT>,
    pub CheckDepthStencilMatch: Option<unsafe extern "system" fn(
        This: *mut IDirect3D9, Adapter: ::UINT, DeviceType: ::D3DDEVTYPE,
        AdapterFormat: ::D3DFORMAT, RenderTargetFormat: ::D3DFORMAT,
        DepthStencilFormat: ::D3DFORMAT,
    ) -> ::HRESULT>,
    pub CheckDeviceFormatConversion: Option<unsafe extern "system" fn(
        This: *mut IDirect3D9, Adapter: ::UINT, DeviceType: ::D3DDEVTYPE,
        SourceFormat: ::D3DFORMAT, TargetFormat: ::D3DFORMAT,
    ) -> ::HRESULT>,
    pub GetDeviceCaps: Option<unsafe extern "system" fn(
        This: *mut IDirect3D9, Adapter: ::UINT, DeviceType: ::D3DDEVTYPE, pCaps: *mut ::D3DCAPS9,
    ) -> ::HRESULT>,
    pub GetAdapterMonitor: Option<unsafe extern "system" fn(
        This: *mut IDirect3D9, Adapter: ::UINT,
    ) -> ::HMONITOR>,
    pub CreateDevice: Option<unsafe extern "system" fn(
        This: *mut IDirect3D9, Adapter: ::UINT, DeviceType: ::D3DDEVTYPE, hFocusWindow: ::HWND,
        BehaviorFlags: ::DWORD, pPresentationParameters: *mut ::D3DPRESENT_PARAMETERS,
        ppReturnedDeviceInterface: *mut *mut IDirect3DDevice9,
    ) -> ::HRESULT>,
}
pub type LPDIRECT3D9 = *mut IDirect3D9;
pub type PDIRECT3D9 = *mut IDirect3D9;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IDirect3DDevice9 {
    pub lpVtbl: *mut IDirect3DDevice9Vtbl,
}
#[repr(C)]
pub struct IDirect3DDevice9Vtbl {
    pub QueryInterface: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, riid: *const ::IID, ppvObj: *mut *mut ::VOID,
    ) -> ::HRESULT>,
    pub AddRef: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9,
    ) -> *mut IDirect3DDevice9>,
    pub Release: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9,
    ) -> *mut IDirect3DDevice9>,
    pub TestCooperativeLevel: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9,
    ) -> *mut IDirect3DDevice9>,
    pub GetAvailableTextureMem: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9,
    ) -> *mut IDirect3DDevice9>,
    pub EvictManagedResources: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9,
    ) -> *mut IDirect3DDevice9>,
    pub GetDirect3D: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, ppD3D9: *mut *mut IDirect3D9,
    ) -> ::HRESULT>,
    pub GetDeviceCaps: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, pCaps: *mut ::D3DCAPS9,
    ) -> ::HRESULT>,
    pub GetDisplayMode: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, iSwapChain: ::UINT, pMode: *mut ::D3DDISPLAYMODE,
    ) -> ::HRESULT>,
    pub GetCreationParameters: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, pParameters: *mut ::D3DDEVICE_CREATION_PARAMETERS,
    ) -> ::HRESULT>,
    pub SetCursorProperties: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, XHotSpot: ::UINT, YHotSpot: ::UINT,
        pCursorBitmap: *mut IDirect3DSurface9,
    ) -> ::HRESULT>,
    pub SetCursorPosition: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, X: ::INT, Y: ::INT, Flags: ::DWORD,
    )>,
    pub ShowCursor: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, bShow: ::BOOL,
    ) -> ::BOOL>,
    pub CreateAdditionalSwapChain: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, pPresentationParameters: *mut ::D3DPRESENT_PARAMETERS,
        pSwapChain: *mut *mut IDirect3DSwapChain9,
    ) -> ::HRESULT>,
    pub GetSwapChain: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, iSwapChain: ::UINT, pSwapChain: *mut *mut IDirect3DSwapChain9,
    ) -> ::HRESULT>,
    pub GetNumberOfSwapChains: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9,
    ) -> *mut IDirect3DDevice9>,
    pub Reset: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, pPresentationParameters: *mut ::D3DPRESENT_PARAMETERS,
    ) -> ::HRESULT>,
    pub Present: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, pSourceRect: *const ::RECT, pDestRect: *const ::RECT,
        hDestWindowOverride: ::HWND, pDirtyRegion: *const ::RGNDATA,
    ) -> ::HRESULT>,
    pub GetBackBuffer: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, iSwapChain: ::UINT, iBackBuffer: ::UINT,
        Type: ::D3DBACKBUFFER_TYPE, ppBackBuffer: *mut *mut IDirect3DSurface9,
    ) -> ::HRESULT>,
    pub GetRasterStatus: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, iSwapChain: ::UINT, pRasterStatus: *mut ::D3DRASTER_STATUS,
    ) -> ::HRESULT>,
    pub SetDialogBoxMode: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, bEnableDialogs: ::BOOL,
    ) -> ::HRESULT>,
    pub SetGammaRamp: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, iSwapChain: ::UINT, Flags: ::DWORD,
        pRamp: *const ::D3DGAMMARAMP,
    )>,
    pub GetGammaRamp: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, iSwapChain: ::UINT, pRamp: *mut ::D3DGAMMARAMP,
    )>,
    pub CreateTexture: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, Width: ::UINT, Height: ::UINT, Levels: ::UINT, Usage: ::DWORD,
        Format: ::D3DFORMAT, Pool: ::D3DPOOL, ppTexture: *mut *mut IDirect3DTexture9,
        pSharedHandle: *mut ::HANDLE,
    ) -> ::HRESULT>,
    pub CreateVolumeTexture: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, Width: ::UINT, Height: ::UINT, Depth: ::UINT, Levels: ::UINT,
        Usage: ::DWORD, Format: ::D3DFORMAT, Pool: ::D3DPOOL,
        ppVolumeTexture: *mut *mut IDirect3DVolumeTexture9, pSharedHandle: *mut ::HANDLE,
    ) -> ::HRESULT>,
    pub CreateCubeTexture: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, EdgeLength: ::UINT, Levels: ::UINT, Usage: ::DWORD,
        Format: ::D3DFORMAT, Pool: ::D3DPOOL, ppCubeTexture: *mut *mut IDirect3DCubeTexture9,
        pSharedHandle: *mut ::HANDLE,
    ) -> ::HRESULT>,
    pub CreateVertexBuffer: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, Length: ::UINT, Usage: ::DWORD, FVF: ::DWORD, Pool: ::D3DPOOL,
        ppVertexBuffer: *mut *mut IDirect3DVertexBuffer9, pSharedHandle: *mut ::HANDLE,
    ) -> ::HRESULT>,
    pub CreateIndexBuffer: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, Length: ::UINT, Usage: ::DWORD, Format: ::D3DFORMAT,
        Pool: ::D3DPOOL, ppIndexBuffer: *mut *mut IDirect3DIndexBuffer9,
        pSharedHandle: *mut ::HANDLE,
    ) -> ::HRESULT>,
    pub CreateRenderTarget: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, Width: ::UINT, Height: ::UINT, Format: ::D3DFORMAT,
        MultiSample: ::D3DMULTISAMPLE_TYPE, MultisampleQuality: ::DWORD, Lockable: ::BOOL,
        ppSurface: *mut *mut IDirect3DSurface9, pSharedHandle: *mut ::HANDLE,
    ) -> ::HRESULT>,
    pub CreateDepthStencilSurface: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, Width: ::UINT, Height: ::UINT, Format: ::D3DFORMAT,
        MultiSample: ::D3DMULTISAMPLE_TYPE, MultisampleQuality: ::DWORD, Discard: ::BOOL,
        ppSurface: *mut *mut IDirect3DSurface9, pSharedHandle: *mut ::HANDLE,
    ) -> ::HRESULT>,
    pub UpdateSurface: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, pSourceSurface: *mut IDirect3DSurface9,
        pSourceRect: *const ::RECT, pDestinationSurface: *mut IDirect3DSurface9,
        pDestPoint: *const ::POINT,
    ) -> ::HRESULT>,
    pub UpdateTexture: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, pSourceTexture: *mut IDirect3DBaseTexture9,
        pDestinationTexture: *mut IDirect3DBaseTexture9,
    ) -> ::HRESULT>,
    pub GetRenderTargetData: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, pRenderTarget: *mut IDirect3DSurface9,
        pDestSurface: *mut IDirect3DSurface9,
    ) -> ::HRESULT>,
    pub GetFrontBufferData: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, iSwapChain: ::UINT, pDestSurface: *mut IDirect3DSurface9,
    ) -> ::HRESULT>,
    pub StretchRect: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, pSourceSurface: *mut IDirect3DSurface9,
        pSourceRect: *const ::RECT, pDestSurface: *mut IDirect3DSurface9, pDestRect: *const ::RECT,
        Filter: ::D3DTEXTUREFILTERTYPE,
    ) -> ::HRESULT>,
    pub ColorFill: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, pSurface: *mut IDirect3DSurface9, pRect: *const ::RECT,
        color: ::D3DCOLOR,
    ) -> ::HRESULT>,
    pub CreateOffscreenPlainSurface: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, Width: ::UINT, Height: ::UINT, Format: ::D3DFORMAT,
        Pool: ::D3DPOOL, ppSurface: *mut *mut IDirect3DSurface9, pSharedHandle: *mut ::HANDLE,
    ) -> ::HRESULT>,
    pub SetRenderTarget: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, RenderTargetIndex: ::DWORD,
        pRenderTarget: *mut IDirect3DSurface9,
    ) -> ::HRESULT>,
    pub GetRenderTarget: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, RenderTargetIndex: ::DWORD,
        ppRenderTarget: *mut *mut IDirect3DSurface9,
    ) -> ::HRESULT>,
    pub SetDepthStencilSurface: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, pNewZStencil: *mut IDirect3DSurface9,
    ) -> ::HRESULT>,
    pub GetDepthStencilSurface: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, ppZStencilSurface: *mut *mut IDirect3DSurface9,
    ) -> ::HRESULT>,
    pub BeginScene: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9,
    ) -> *mut IDirect3DDevice9>,
    pub EndScene: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9,
    ) -> *mut IDirect3DDevice9>,
    pub Clear: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, Count: ::DWORD, pRects: *const ::D3DRECT, Flags: ::DWORD,
        Color: ::D3DCOLOR, Z: ::FLOAT, Stencil: ::DWORD,
    ) -> ::HRESULT>,
    pub SetTransform: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, State: ::D3DTRANSFORMSTATETYPE, pMatrix: *const ::D3DMATRIX,
    ) -> ::HRESULT>,
    pub GetTransform: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, State: ::D3DTRANSFORMSTATETYPE, pMatrix: *mut ::D3DMATRIX,
    ) -> ::HRESULT>,
    pub MultiplyTransform: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, arg1: ::D3DTRANSFORMSTATETYPE, arg2: *const ::D3DMATRIX,
    ) -> ::HRESULT>,
    pub SetViewport: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, pViewport: *const ::D3DVIEWPORT9,
    ) -> ::HRESULT>,
    pub GetViewport: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, pViewport: *mut ::D3DVIEWPORT9,
    ) -> ::HRESULT>,
    pub SetMaterial: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, pMaterial: *const ::D3DMATERIAL9,
    ) -> ::HRESULT>,
    pub GetMaterial: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, pMaterial: *mut ::D3DMATERIAL9,
    ) -> ::HRESULT>,
    pub SetLight: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, Index: ::DWORD, arg1: *const ::D3DLIGHT9,
    ) -> ::HRESULT>,
    pub GetLight: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, Index: ::DWORD, arg1: *mut ::D3DLIGHT9,
    ) -> ::HRESULT>,
    pub LightEnable: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, Index: ::DWORD, Enable: ::BOOL,
    ) -> ::HRESULT>,
    pub GetLightEnable: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, Index: ::DWORD, pEnable: *mut ::BOOL,
    ) -> ::HRESULT>,
    pub SetClipPlane: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, Index: ::DWORD, pPlane: *const ::FLOAT,
    ) -> ::HRESULT>,
    pub GetClipPlane: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, Index: ::DWORD, pPlane: *mut ::FLOAT,
    ) -> ::HRESULT>,
    pub SetRenderState: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, State: ::D3DRENDERSTATETYPE, Value: ::DWORD,
    ) -> ::HRESULT>,
    pub GetRenderState: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, State: ::D3DRENDERSTATETYPE, pValue: *mut ::DWORD,
    ) -> ::HRESULT>,
    pub CreateStateBlock: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, Type: ::D3DSTATEBLOCKTYPE,
        ppSB: *mut *mut IDirect3DStateBlock9,
    ) -> ::HRESULT>,
    pub BeginStateBlock: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9,
    ) -> *mut IDirect3DDevice9>,
    pub EndStateBlock: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, ppSB: *mut *mut IDirect3DStateBlock9,
    ) -> ::HRESULT>,
    pub SetClipStatus: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, pClipStatus: *const ::D3DCLIPSTATUS9,
    ) -> ::HRESULT>,
    pub GetClipStatus: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, pClipStatus: *mut ::D3DCLIPSTATUS9,
    ) -> ::HRESULT>,
    pub GetTexture: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, Stage: ::DWORD, ppTexture: *mut *mut IDirect3DBaseTexture9,
    ) -> ::HRESULT>,
    pub SetTexture: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, Stage: ::DWORD, pTexture: *mut IDirect3DBaseTexture9,
    ) -> ::HRESULT>,
    pub GetTextureStageState: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, Stage: ::DWORD, Type: ::D3DTEXTURESTAGESTATETYPE,
        pValue: *mut ::DWORD,
    ) -> ::HRESULT>,
    pub SetTextureStageState: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, Stage: ::DWORD, Type: ::D3DTEXTURESTAGESTATETYPE,
        Value: ::DWORD,
    ) -> ::HRESULT>,
    pub GetSamplerState: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, Sampler: ::DWORD, Type: ::D3DSAMPLERSTATETYPE,
        pValue: *mut ::DWORD,
    ) -> ::HRESULT>,
    pub SetSamplerState: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, Sampler: ::DWORD, Type: ::D3DSAMPLERSTATETYPE, Value: ::DWORD,
    ) -> ::HRESULT>,
    pub ValidateDevice: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, pNumPasses: *mut ::DWORD,
    ) -> ::HRESULT>,
    pub SetPaletteEntries: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, PaletteNumber: ::UINT, pEntries: *const ::PALETTEENTRY,
    ) -> ::HRESULT>,
    pub GetPaletteEntries: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, PaletteNumber: ::UINT, pEntries: *mut ::PALETTEENTRY,
    ) -> ::HRESULT>,
    pub SetCurrentTexturePalette: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, PaletteNumber: ::UINT,
    ) -> ::HRESULT>,
    pub GetCurrentTexturePalette: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, PaletteNumber: *mut ::UINT,
    ) -> ::HRESULT>,
    pub SetScissorRect: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, pRect: *const ::RECT,
    ) -> ::HRESULT>,
    pub GetScissorRect: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, pRect: *mut ::RECT,
    ) -> ::HRESULT>,
    pub SetSoftwareVertexProcessing: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, bSoftware: ::BOOL,
    ) -> ::HRESULT>,
    pub GetSoftwareVertexProcessing: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9,
    ) -> *mut IDirect3DDevice9>,
    pub SetNPatchMode: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, nSegments: ::FLOAT,
    ) -> ::HRESULT>,
    pub GetNPatchMode: Option<unsafe extern "system" fn(This: *mut IDirect3DDevice9) -> ::FLOAT>,
    pub DrawPrimitive: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, PrimitiveType: ::D3DPRIMITIVETYPE, StartVertex: ::UINT,
        PrimitiveCount: ::UINT,
    ) -> ::HRESULT>,
    pub DrawIndexedPrimitive: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, arg1: ::D3DPRIMITIVETYPE, BaseVertexIndex: ::INT,
        MinVertexIndex: ::UINT, NumVertices: ::UINT, startIndex: ::UINT, primCount: ::UINT,
    ) -> ::HRESULT>,
    pub DrawPrimitiveUP: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, PrimitiveType: ::D3DPRIMITIVETYPE, PrimitiveCount: ::UINT,
        pVertexStreamZeroData: *const ::VOID, VertexStreamZeroStride: ::UINT,
    ) -> ::HRESULT>,
    pub DrawIndexedPrimitiveUP: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, PrimitiveType: ::D3DPRIMITIVETYPE, MinVertexIndex: ::UINT,
        NumVertices: ::UINT, PrimitiveCount: ::UINT, pIndexData: *const ::VOID,
        IndexDataFormat: ::D3DFORMAT, pVertexStreamZeroData: *const ::VOID,
        VertexStreamZeroStride: ::UINT,
    ) -> ::HRESULT>,
    pub ProcessVertices: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, SrcStartIndex: ::UINT, DestIndex: ::UINT, VertexCount: ::UINT,
        pDestBuffer: *mut IDirect3DVertexBuffer9, pVertexDecl: *mut IDirect3DVertexDeclaration9,
        Flags: ::DWORD,
    ) -> ::HRESULT>,
    pub CreateVertexDeclaration: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, pVertexElements: *const ::D3DVERTEXELEMENT9,
        ppDecl: *mut *mut IDirect3DVertexDeclaration9,
    ) -> ::HRESULT>,
    pub SetVertexDeclaration: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, pDecl: *mut IDirect3DVertexDeclaration9,
    ) -> ::HRESULT>,
    pub GetVertexDeclaration: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, ppDecl: *mut *mut IDirect3DVertexDeclaration9,
    ) -> ::HRESULT>,
    pub SetFVF: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, FVF: ::DWORD,
    ) -> ::HRESULT>,
    pub GetFVF: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, pFVF: *mut ::DWORD,
    ) -> ::HRESULT>,
    pub CreateVertexShader: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, pFunction: *const ::DWORD,
        ppShader: *mut *mut IDirect3DVertexShader9,
    ) -> ::HRESULT>,
    pub SetVertexShader: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, pShader: *mut IDirect3DVertexShader9,
    ) -> ::HRESULT>,
    pub GetVertexShader: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, ppShader: *mut *mut IDirect3DVertexShader9,
    ) -> ::HRESULT>,
    pub SetVertexShaderConstantF: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, StartRegister: ::UINT, pConstantData: *const ::FLOAT,
        Vector4fCount: ::UINT,
    ) -> ::HRESULT>,
    pub GetVertexShaderConstantF: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, StartRegister: ::UINT, pConstantData: *mut ::FLOAT,
        Vector4fCount: ::UINT,
    ) -> ::HRESULT>,
    pub SetVertexShaderConstantI: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, StartRegister: ::UINT, pConstantData: *const ::INT,
        Vector4iCount: ::UINT,
    ) -> ::HRESULT>,
    pub GetVertexShaderConstantI: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, StartRegister: ::UINT, pConstantData: *mut ::INT,
        Vector4iCount: ::UINT,
    ) -> ::HRESULT>,
    pub SetVertexShaderConstantB: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, StartRegister: ::UINT, pConstantData: *const ::BOOL,
        BoolCount: ::UINT,
    ) -> ::HRESULT>,
    pub GetVertexShaderConstantB: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, StartRegister: ::UINT, pConstantData: *mut ::BOOL,
        BoolCount: ::UINT,
    ) -> ::HRESULT>,
    pub SetStreamSource: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, StreamNumber: ::UINT,
        pStreamData: *mut IDirect3DVertexBuffer9, OffsetInBytes: ::UINT, Stride: ::UINT,
    ) -> ::HRESULT>,
    pub GetStreamSource: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, StreamNumber: ::UINT,
        ppStreamData: *mut *mut IDirect3DVertexBuffer9, pOffsetInBytes: *mut ::UINT,
        pStride: *mut ::UINT,
    ) -> ::HRESULT>,
    pub SetStreamSourceFreq: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, StreamNumber: ::UINT, Setting: ::UINT,
    ) -> ::HRESULT>,
    pub GetStreamSourceFreq: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, StreamNumber: ::UINT, pSetting: *mut ::UINT,
    ) -> ::HRESULT>,
    pub SetIndices: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, pIndexData: *mut IDirect3DIndexBuffer9,
    ) -> ::HRESULT>,
    pub GetIndices: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, ppIndexData: *mut *mut IDirect3DIndexBuffer9,
    ) -> ::HRESULT>,
    pub CreatePixelShader: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, pFunction: *const ::DWORD,
        ppShader: *mut *mut IDirect3DPixelShader9,
    ) -> ::HRESULT>,
    pub SetPixelShader: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, pShader: *mut IDirect3DPixelShader9,
    ) -> ::HRESULT>,
    pub GetPixelShader: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, ppShader: *mut *mut IDirect3DPixelShader9,
    ) -> ::HRESULT>,
    pub SetPixelShaderConstantF: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, StartRegister: ::UINT, pConstantData: *const ::FLOAT,
        Vector4fCount: ::UINT,
    ) -> ::HRESULT>,
    pub GetPixelShaderConstantF: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, StartRegister: ::UINT, pConstantData: *mut ::FLOAT,
        Vector4fCount: ::UINT,
    ) -> ::HRESULT>,
    pub SetPixelShaderConstantI: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, StartRegister: ::UINT, pConstantData: *const ::INT,
        Vector4iCount: ::UINT,
    ) -> ::HRESULT>,
    pub GetPixelShaderConstantI: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, StartRegister: ::UINT, pConstantData: *mut ::INT,
        Vector4iCount: ::UINT,
    ) -> ::HRESULT>,
    pub SetPixelShaderConstantB: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, StartRegister: ::UINT, pConstantData: *const ::BOOL,
        BoolCount: ::UINT,
    ) -> ::HRESULT>,
    pub GetPixelShaderConstantB: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, StartRegister: ::UINT, pConstantData: *mut ::BOOL,
        BoolCount: ::UINT,
    ) -> ::HRESULT>,
    pub DrawRectPatch: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, Handle: ::UINT, pNumSegs: *const ::FLOAT,
        pRectPatchInfo: *const ::D3DRECTPATCH_INFO,
    ) -> ::HRESULT>,
    pub DrawTriPatch: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, Handle: ::UINT, pNumSegs: *const ::FLOAT,
        pTriPatchInfo: *const ::D3DTRIPATCH_INFO,
    ) -> ::HRESULT>,
    pub DeletePatch: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, Handle: ::UINT,
    ) -> ::HRESULT>,
    pub CreateQuery: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9, Type: ::D3DQUERYTYPE, ppQuery: *mut *mut IDirect3DQuery9,
    ) -> ::HRESULT>,
}
pub type LPDIRECT3DDEVICE9 = *mut IDirect3DDevice9;
pub type PDIRECT3DDEVICE9 = *mut IDirect3DDevice9;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IDirect3DStateBlock9 {
    pub lpVtbl: *mut IDirect3DStateBlock9Vtbl,
}
#[repr(C)]
pub struct IDirect3DStateBlock9Vtbl {
    pub QueryInterface: Option<unsafe extern "system" fn(
        This: *mut IDirect3DStateBlock9, riid: *const ::IID, ppvObj: *mut *mut ::VOID,
    ) -> ::HRESULT>,
    pub AddRef: Option<unsafe extern "system" fn(
        This: *mut IDirect3DStateBlock9,
    ) -> *mut IDirect3DStateBlock9>,
    pub Release: Option<unsafe extern "system" fn(
        This: *mut IDirect3DStateBlock9,
    ) -> *mut IDirect3DStateBlock9>,
    pub GetDevice: Option<unsafe extern "system" fn(
        This: *mut IDirect3DStateBlock9, ppDevice: *mut *mut IDirect3DDevice9,
    ) -> ::HRESULT>,
    pub Capture: Option<unsafe extern "system" fn(
        This: *mut IDirect3DStateBlock9,
    ) -> *mut IDirect3DStateBlock9>,
    pub Apply: Option<unsafe extern "system" fn(This: *mut IDirect3DStateBlock9) -> ::HRESULT>,
}
pub type LPDIRECT3DSTATEBLOCK9 = *mut IDirect3DStateBlock9;
pub type PDIRECT3DSTATEBLOCK9 = *mut IDirect3DStateBlock9;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IDirect3DSwapChain9 {
    pub lpVtbl: *mut IDirect3DSwapChain9Vtbl,
}
#[repr(C)]
pub struct IDirect3DSwapChain9Vtbl {
    pub QueryInterface: Option<unsafe extern "system" fn(
        This: *mut IDirect3DSwapChain9, riid: *const ::IID, ppvObj: *mut *mut ::VOID,
    ) -> ::HRESULT>,
    pub AddRef: Option<unsafe extern "system" fn(
        This: *mut IDirect3DSwapChain9,
    ) -> *mut IDirect3DSwapChain9>,
    pub Release: Option<unsafe extern "system" fn(
        This: *mut IDirect3DSwapChain9,
    ) -> *mut IDirect3DSwapChain9>,
    pub Present: Option<unsafe extern "system" fn(
        This: *mut IDirect3DSwapChain9, pSourceRect: *const ::RECT, pDestRect: *const ::RECT,
        hDestWindowOverride: ::HWND, pDirtyRegion: *const ::RGNDATA, dwFlags: ::DWORD,
    ) -> ::HRESULT>,
    pub GetFrontBufferData: Option<unsafe extern "system" fn(
        This: *mut IDirect3DSwapChain9, pDestSurface: *mut IDirect3DSurface9,
    ) -> ::HRESULT>,
    pub GetBackBuffer: Option<unsafe extern "system" fn(
        This: *mut IDirect3DSwapChain9, iBackBuffer: ::UINT, Type: ::D3DBACKBUFFER_TYPE,
        ppBackBuffer: *mut *mut IDirect3DSurface9,
    ) -> ::HRESULT>,
    pub GetRasterStatus: Option<unsafe extern "system" fn(
        This: *mut IDirect3DSwapChain9, pRasterStatus: *mut ::D3DRASTER_STATUS,
    ) -> ::HRESULT>,
    pub GetDisplayMode: Option<unsafe extern "system" fn(
        This: *mut IDirect3DSwapChain9, pMode: *mut ::D3DDISPLAYMODE,
    ) -> ::HRESULT>,
    pub GetDevice: Option<unsafe extern "system" fn(
        This: *mut IDirect3DSwapChain9, ppDevice: *mut *mut IDirect3DDevice9,
    ) -> ::HRESULT>,
    pub GetPresentParameters: Option<unsafe extern "system" fn(
        This: *mut IDirect3DSwapChain9, pPresentationParameters: *mut ::D3DPRESENT_PARAMETERS,
    ) -> ::HRESULT>,
}
pub type LPDIRECT3DSWAPCHAIN9 = *mut IDirect3DSwapChain9;
pub type PDIRECT3DSWAPCHAIN9 = *mut IDirect3DSwapChain9;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IDirect3DResource9 {
    pub lpVtbl: *mut IDirect3DResource9Vtbl,
}
#[repr(C)]
pub struct IDirect3DResource9Vtbl {
    pub QueryInterface: Option<unsafe extern "system" fn(
        This: *mut IDirect3DResource9, riid: *const ::IID, ppvObj: *mut *mut ::VOID,
    ) -> ::HRESULT>,
    pub AddRef: Option<unsafe extern "system" fn(
        This: *mut IDirect3DResource9,
    ) -> *mut IDirect3DResource9>,
    pub Release: Option<unsafe extern "system" fn(
        This: *mut IDirect3DResource9,
    ) -> *mut IDirect3DResource9>,
    pub GetDevice: Option<unsafe extern "system" fn(
        This: *mut IDirect3DResource9, ppDevice: *mut *mut IDirect3DDevice9,
    ) -> ::HRESULT>,
    pub SetPrivateData: Option<unsafe extern "system" fn(
        This: *mut IDirect3DResource9, refguid: *const ::GUID, pData: *const ::VOID,
        SizeOfData: ::DWORD, Flags: ::DWORD,
    ) -> ::HRESULT>,
    pub GetPrivateData: Option<unsafe extern "system" fn(
        This: *mut IDirect3DResource9, refguid: *const ::GUID, pData: *mut ::VOID,
        pSizeOfData: *mut ::DWORD,
    ) -> ::HRESULT>,
    pub FreePrivateData: Option<unsafe extern "system" fn(
        This: *mut IDirect3DResource9, refguid: *const ::GUID,
    ) -> ::HRESULT>,
    pub SetPriority: Option<unsafe extern "system" fn(
        This: *mut IDirect3DResource9, PriorityNew: ::DWORD,
    ) -> ::DWORD>,
    pub GetPriority: Option<unsafe extern "system" fn(
        This: *mut IDirect3DResource9,
    ) -> *mut IDirect3DResource9>,
    pub PreLoad: Option<unsafe extern "system" fn(
        This: *mut IDirect3DResource9,
    )>,
    pub GetType: Option<unsafe extern "system" fn(
        This: *mut IDirect3DResource9,
    ) -> ::D3DRESOURCETYPE>,
}
pub type LPDIRECT3DRESOURCE9 = *mut IDirect3DResource9;
pub type PDIRECT3DRESOURCE9 = *mut IDirect3DResource9;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IDirect3DVertexDeclaration9 {
    pub lpVtbl: *mut IDirect3DVertexDeclaration9Vtbl,
}
#[repr(C)]
pub struct IDirect3DVertexDeclaration9Vtbl {
    pub QueryInterface: Option<unsafe extern "system" fn(
        This: *mut IDirect3DVertexDeclaration9, riid: *const ::IID, ppvObj: *mut *mut ::VOID,
    ) -> ::HRESULT>,
    pub AddRef: Option<unsafe extern "system" fn(
        This: *mut IDirect3DVertexDeclaration9,
    ) -> *mut IDirect3DVertexDeclaration9>,
    pub Release: Option<unsafe extern "system" fn(
        This: *mut IDirect3DVertexDeclaration9,
    ) -> *mut IDirect3DVertexDeclaration9>,
    pub GetDevice: Option<unsafe extern "system" fn(
        This: *mut IDirect3DVertexDeclaration9, ppDevice: *mut *mut IDirect3DDevice9,
    ) -> ::HRESULT>,
    pub GetDeclaration: Option<unsafe extern "system" fn(
        This: *mut IDirect3DVertexDeclaration9, pElement: *mut ::D3DVERTEXELEMENT9,
        pNumElements: *mut ::UINT,
    ) -> ::HRESULT>,
}
pub type LPDIRECT3DVERTEXDECLARATION9 = *mut IDirect3DVertexDeclaration9;
pub type PDIRECT3DVERTEXDECLARATION9 = *mut IDirect3DVertexDeclaration9;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IDirect3DVertexShader9 {
    pub lpVtbl: *mut IDirect3DVertexShader9Vtbl,
}
#[repr(C)]
pub struct IDirect3DVertexShader9Vtbl {
    pub QueryInterface: Option<unsafe extern "system" fn(
        This: *mut IDirect3DVertexShader9, riid: *const ::IID, ppvObj: *mut *mut ::VOID,
    ) -> ::HRESULT>,
    pub AddRef: Option<unsafe extern "system" fn(
        This: *mut IDirect3DVertexShader9,
    ) -> *mut IDirect3DVertexShader9>,
    pub Release: Option<unsafe extern "system" fn(
        This: *mut IDirect3DVertexShader9,
    ) -> *mut IDirect3DVertexShader9>,
    pub GetDevice: Option<unsafe extern "system" fn(
        This: *mut IDirect3DVertexShader9, ppDevice: *mut *mut IDirect3DDevice9,
    ) -> ::HRESULT>,
    pub GetFunction: Option<unsafe extern "system" fn(
        This: *mut IDirect3DVertexShader9, arg1: *mut ::VOID, pSizeOfData: *mut ::UINT,
    ) -> ::HRESULT>,
}
pub type LPDIRECT3DVERTEXSHADER9 = *mut IDirect3DVertexShader9;
pub type PDIRECT3DVERTEXSHADER9 = *mut IDirect3DVertexShader9;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IDirect3DPixelShader9 {
    pub lpVtbl: *mut IDirect3DPixelShader9Vtbl,
}
#[repr(C)]
pub struct IDirect3DPixelShader9Vtbl {
    pub QueryInterface: Option<unsafe extern "system" fn(
        This: *mut IDirect3DPixelShader9, riid: *const ::IID, ppvObj: *mut *mut ::VOID,
    ) -> ::HRESULT>,
    pub AddRef: Option<unsafe extern "system" fn(
        This: *mut IDirect3DPixelShader9,
    ) -> *mut IDirect3DPixelShader9>,
    pub Release: Option<unsafe extern "system" fn(
        This: *mut IDirect3DPixelShader9,
    ) -> *mut IDirect3DPixelShader9>,
    pub GetDevice: Option<unsafe extern "system" fn(
        This: *mut IDirect3DPixelShader9, ppDevice: *mut *mut IDirect3DDevice9,
    ) -> ::HRESULT>,
    pub GetFunction: Option<unsafe extern "system" fn(
        This: *mut IDirect3DPixelShader9, arg1: *mut ::VOID, pSizeOfData: *mut ::UINT,
    ) -> ::HRESULT>,
}
pub type LPDIRECT3DPIXELSHADER9 = *mut IDirect3DPixelShader9;
pub type PDIRECT3DPIXELSHADER9 = *mut IDirect3DPixelShader9;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IDirect3DBaseTexture9 {
    pub lpVtbl: *mut IDirect3DBaseTexture9Vtbl,
}
#[repr(C)]
pub struct IDirect3DBaseTexture9Vtbl {
    pub QueryInterface: Option<unsafe extern "system" fn(
        This: *mut IDirect3DBaseTexture9, riid: *const ::IID, ppvObj: *mut *mut ::VOID,
    ) -> ::HRESULT>,
    pub AddRef: Option<unsafe extern "system" fn(
        This: *mut IDirect3DBaseTexture9,
    ) -> *mut IDirect3DBaseTexture9>,
    pub Release: Option<unsafe extern "system" fn(
        This: *mut IDirect3DBaseTexture9,
    ) -> *mut IDirect3DBaseTexture9>,
    pub GetDevice: Option<unsafe extern "system" fn(
        This: *mut IDirect3DBaseTexture9, ppDevice: *mut *mut IDirect3DDevice9,
    ) -> ::HRESULT>,
    pub SetPrivateData: Option<unsafe extern "system" fn(
        This: *mut IDirect3DBaseTexture9, refguid: *const ::GUID, pData: *const ::VOID,
        SizeOfData: ::DWORD, Flags: ::DWORD,
    ) -> ::HRESULT>,
    pub GetPrivateData: Option<unsafe extern "system" fn(
        This: *mut IDirect3DBaseTexture9, refguid: *const ::GUID, pData: *mut ::VOID,
        pSizeOfData: *mut ::DWORD,
    ) -> ::HRESULT>,
    pub FreePrivateData: Option<unsafe extern "system" fn(
        This: *mut IDirect3DBaseTexture9, refguid: *const ::GUID,
    ) -> ::HRESULT>,
    pub SetPriority: Option<unsafe extern "system" fn(
        This: *mut IDirect3DBaseTexture9, PriorityNew: ::DWORD,
    ) -> ::DWORD>,
    pub GetPriority: Option<unsafe extern "system" fn(
        This: *mut IDirect3DBaseTexture9,
    ) -> *mut IDirect3DBaseTexture9>,
    pub PreLoad: Option<unsafe extern "system" fn(This: *mut IDirect3DBaseTexture9)>,
    pub GetType: Option<unsafe extern "system" fn(
        This: *mut IDirect3DBaseTexture9,
    ) -> *mut IDirect3DBaseTexture9>,
    pub SetLOD: Option<unsafe extern "system" fn(
        This: *mut IDirect3DBaseTexture9, LODNew: ::DWORD,
    ) -> ::DWORD>,
    pub GetLOD: Option<unsafe extern "system" fn(
        This: *mut IDirect3DBaseTexture9,
    ) -> *mut IDirect3DBaseTexture9>,
    pub GetLevelCount: Option<unsafe extern "system" fn(
        This: *mut IDirect3DBaseTexture9,
    ) -> *mut IDirect3DBaseTexture9>,
    pub SetAutoGenFilterType: Option<unsafe extern "system" fn(
        This: *mut IDirect3DBaseTexture9, FilterType: ::D3DTEXTUREFILTERTYPE,
    ) -> ::HRESULT>,
    pub GetAutoGenFilterType: Option<unsafe extern "system" fn(
        This: *mut IDirect3DBaseTexture9,
    ) -> ::D3DTEXTUREFILTERTYPE>,
    pub GenerateMipSubLevels: Option<unsafe extern "system" fn(This: *mut IDirect3DBaseTexture9)>,
}
pub type LPDIRECT3DBASETEXTURE9 = *mut IDirect3DBaseTexture9;
pub type PDIRECT3DBASETEXTURE9 = *mut IDirect3DBaseTexture9;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IDirect3DTexture9 {
    pub lpVtbl: *mut IDirect3DTexture9Vtbl,
}
#[repr(C)]
pub struct IDirect3DTexture9Vtbl {
    pub QueryInterface: Option<unsafe extern "system" fn(
        This: *mut IDirect3DTexture9, riid: *const ::IID, ppvObj: *mut *mut ::VOID,
    ) -> ::HRESULT>,
    pub AddRef: Option<unsafe extern "system" fn(
        This: *mut IDirect3DTexture9,
    ) -> *mut IDirect3DTexture9>,
    pub Release: Option<unsafe extern "system" fn(
        This: *mut IDirect3DTexture9,
    ) -> *mut IDirect3DTexture9>,
    pub GetDevice: Option<unsafe extern "system" fn(
        This: *mut IDirect3DTexture9, ppDevice: *mut *mut IDirect3DDevice9,
    ) -> ::HRESULT>,
    pub SetPrivateData: Option<unsafe extern "system" fn(
        This: *mut IDirect3DTexture9, refguid: *const ::GUID, pData: *const ::VOID,
        SizeOfData: ::DWORD, Flags: ::DWORD,
    ) -> ::HRESULT>,
    pub GetPrivateData: Option<unsafe extern "system" fn(
        This: *mut IDirect3DTexture9, refguid: *const ::GUID, pData: *mut ::VOID,
        pSizeOfData: *mut ::DWORD,
    ) -> ::HRESULT>,
    pub FreePrivateData: Option<unsafe extern "system" fn(
        This: *mut IDirect3DTexture9, refguid: *const ::GUID,
    ) -> ::HRESULT>,
    pub SetPriority: Option<unsafe extern "system" fn(
        This: *mut IDirect3DTexture9, PriorityNew: ::DWORD,
    ) -> ::DWORD>,
    pub GetPriority: Option<unsafe extern "system" fn(
        This: *mut IDirect3DTexture9,
    ) -> *mut IDirect3DTexture9>,
    pub PreLoad: Option<unsafe extern "system" fn(This: *mut IDirect3DTexture9)>,
    pub GetType: Option<unsafe extern "system" fn(
        This: *mut IDirect3DTexture9,
    ) -> *mut IDirect3DTexture9>,
    pub SetLOD: Option<unsafe extern "system" fn(
        This: *mut IDirect3DTexture9, LODNew: ::DWORD,
    ) -> ::DWORD>,
    pub GetLOD: Option<unsafe extern "system" fn(
        This: *mut IDirect3DTexture9,
    ) -> *mut IDirect3DTexture9>,
    pub GetLevelCount: Option<unsafe extern "system" fn(
        This: *mut IDirect3DTexture9,
    ) -> *mut IDirect3DTexture9>,
    pub SetAutoGenFilterType: Option<unsafe extern "system" fn(
        This: *mut IDirect3DTexture9, FilterType: ::D3DTEXTUREFILTERTYPE,
    ) -> ::HRESULT>,
    pub GetAutoGenFilterType: Option<unsafe extern "system" fn(
        This: *mut IDirect3DTexture9,
    ) -> ::D3DTEXTUREFILTERTYPE>,
    pub GenerateMipSubLevels: Option<unsafe extern "system" fn(This: *mut IDirect3DTexture9)>,
    pub GetLevelDesc: Option<unsafe extern "system" fn(
        This: *mut IDirect3DTexture9, Level: ::UINT, pDesc: *mut ::D3DSURFACE_DESC,
    ) -> ::HRESULT>,
    pub GetSurfaceLevel: Option<unsafe extern "system" fn(
        This: *mut IDirect3DTexture9, Level: ::UINT, ppSurfaceLevel: *mut *mut IDirect3DSurface9,
    ) -> ::HRESULT>,
    pub LockRect: Option<unsafe extern "system" fn(
        This: *mut IDirect3DTexture9, Level: ::UINT, pLockedRect: *mut ::D3DLOCKED_RECT,
        pRect: *const ::RECT, Flags: ::DWORD,
    ) -> ::HRESULT>,
    pub UnlockRect: Option<unsafe extern "system" fn(
        This: *mut IDirect3DTexture9, Level: ::UINT,
    ) -> ::HRESULT>,
    pub AddDirtyRect: Option<unsafe extern "system" fn(
        This: *mut IDirect3DTexture9, pDirtyRect: *const ::RECT,
    ) -> ::HRESULT>,
}
pub type LPDIRECT3DTEXTURE9 = *mut IDirect3DTexture9;
pub type PDIRECT3DTEXTURE9 = *mut IDirect3DTexture9;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IDirect3DVolumeTexture9 {
    pub lpVtbl: *mut IDirect3DVolumeTexture9Vtbl,
}
#[repr(C)]
pub struct IDirect3DVolumeTexture9Vtbl {
    pub QueryInterface: Option<unsafe extern "system" fn(
        This: *mut IDirect3DVolumeTexture9, riid: *const ::IID, ppvObj: *mut *mut ::VOID,
    ) -> ::HRESULT>,
    pub AddRef: Option<unsafe extern "system" fn(
        This: *mut IDirect3DVolumeTexture9,
    ) -> *mut IDirect3DVolumeTexture9>,
    pub Release: Option<unsafe extern "system" fn(
        This: *mut IDirect3DVolumeTexture9,
    ) -> *mut IDirect3DVolumeTexture9>,
    pub GetDevice: Option<unsafe extern "system" fn(
        This: *mut IDirect3DVolumeTexture9, ppDevice: *mut *mut IDirect3DDevice9,
    ) -> ::HRESULT>,
    pub SetPrivateData: Option<unsafe extern "system" fn(
        This: *mut IDirect3DVolumeTexture9, refguid: *const ::GUID, pData: *const ::VOID,
        SizeOfData: ::DWORD, Flags: ::DWORD,
    ) -> ::HRESULT>,
    pub GetPrivateData: Option<unsafe extern "system" fn(
        This: *mut IDirect3DVolumeTexture9, refguid: *const ::GUID, pData: *mut ::VOID,
        pSizeOfData: *mut ::DWORD,
    ) -> ::HRESULT>,
    pub FreePrivateData: Option<unsafe extern "system" fn(
        This: *mut IDirect3DVolumeTexture9, refguid: *const ::GUID,
    ) -> ::HRESULT>,
    pub SetPriority: Option<unsafe extern "system" fn(
        This: *mut IDirect3DVolumeTexture9, PriorityNew: ::DWORD,
    ) -> ::DWORD>,
    pub GetPriority: Option<unsafe extern "system" fn(
        This: *mut IDirect3DVolumeTexture9,
    ) -> *mut IDirect3DVolumeTexture9>,
    pub PreLoad: Option<unsafe extern "system" fn(This: *mut IDirect3DVolumeTexture9)>,
    pub GetType: Option<unsafe extern "system" fn(
        This: *mut IDirect3DVolumeTexture9,
    ) -> *mut IDirect3DVolumeTexture9>,
    pub SetLOD: Option<unsafe extern "system" fn(
        This: *mut IDirect3DVolumeTexture9, LODNew: ::DWORD,
    ) -> ::DWORD>,
    pub GetLOD: Option<unsafe extern "system" fn(
        This: *mut IDirect3DVolumeTexture9,
    ) -> *mut IDirect3DVolumeTexture9>,
    pub GetLevelCount: Option<unsafe extern "system" fn(
        This: *mut IDirect3DVolumeTexture9,
    ) -> *mut IDirect3DVolumeTexture9>,
    pub SetAutoGenFilterType: Option<unsafe extern "system" fn(
    This: *mut IDirect3DVolumeTexture9, FilterType: ::D3DTEXTUREFILTERTYPE,
    ) -> ::HRESULT>,
    pub GetAutoGenFilterType: Option<unsafe extern "system" fn(
        This: *mut IDirect3DVolumeTexture9,
    ) -> ::D3DTEXTUREFILTERTYPE>,
    pub GenerateMipSubLevels: Option<unsafe extern "system" fn(
        This: *mut IDirect3DVolumeTexture9,
    )>,
    pub GetLevelDesc: Option<unsafe extern "system" fn(
        This: *mut IDirect3DVolumeTexture9, Level: ::UINT, pDesc: *mut ::D3DVOLUME_DESC,
    ) -> ::HRESULT>,
    pub GetVolumeLevel: Option<unsafe extern "system" fn(
        This: *mut IDirect3DVolumeTexture9, Level: ::UINT,
        ppVolumeLevel: *mut *mut IDirect3DVolume9,
    ) -> ::HRESULT>,
    pub LockBox: Option<unsafe extern "system" fn(
        This: *mut IDirect3DVolumeTexture9, Level: ::UINT, pLockedVolume: *mut ::D3DLOCKED_BOX,
        pBox: *const ::D3DBOX, Flags: ::DWORD,
    ) -> ::HRESULT>,
    pub UnlockBox: Option<unsafe extern "system" fn(
        This: *mut IDirect3DVolumeTexture9, Level: ::UINT,
    ) -> ::HRESULT>,
    pub AddDirtyBox: Option<unsafe extern "system" fn(
        This: *mut IDirect3DVolumeTexture9, pDirtyBox: *const ::D3DBOX,
    ) -> ::HRESULT>,
}
pub type LPDIRECT3DVOLUMETEXTURE9 = *mut IDirect3DVolumeTexture9;
pub type PDIRECT3DVOLUMETEXTURE9 = *mut IDirect3DVolumeTexture9;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IDirect3DCubeTexture9 {
    pub lpVtbl: *mut IDirect3DCubeTexture9Vtbl,
}
#[repr(C)]
pub struct IDirect3DCubeTexture9Vtbl {
    pub QueryInterface: Option<unsafe extern "system" fn(
        This: *mut IDirect3DCubeTexture9, riid: *const ::IID, ppvObj: *mut *mut ::VOID,
    ) -> ::HRESULT>,
    pub AddRef: Option<unsafe extern "system" fn(
        This: *mut IDirect3DCubeTexture9,
    ) -> *mut IDirect3DCubeTexture9>,
    pub Release: Option<unsafe extern "system" fn(
        This: *mut IDirect3DCubeTexture9,
    ) -> *mut IDirect3DCubeTexture9>,
    pub GetDevice: Option<unsafe extern "system" fn(
        This: *mut IDirect3DCubeTexture9, ppDevice: *mut *mut IDirect3DDevice9,
    ) -> ::HRESULT>,
    pub SetPrivateData: Option<unsafe extern "system" fn(
        This: *mut IDirect3DCubeTexture9, refguid: *const ::GUID, pData: *const ::VOID,
        SizeOfData: ::DWORD, Flags: ::DWORD,
    ) -> ::HRESULT>,
    pub GetPrivateData: Option<unsafe extern "system" fn(
        This: *mut IDirect3DCubeTexture9, refguid: *const ::GUID, pData: *mut ::VOID,
        pSizeOfData: *mut ::DWORD,
    ) -> ::HRESULT>,
    pub FreePrivateData: Option<unsafe extern "system" fn(
        This: *mut IDirect3DCubeTexture9, refguid: *const ::GUID,
    ) -> ::HRESULT>,
    pub SetPriority: Option<unsafe extern "system" fn(
        This: *mut IDirect3DCubeTexture9, PriorityNew: ::DWORD,
    ) -> ::DWORD>,
    pub GetPriority: Option<unsafe extern "system" fn(
        This: *mut IDirect3DCubeTexture9,
    ) -> *mut IDirect3DCubeTexture9>,
    pub PreLoad: Option<unsafe extern "system" fn(This: *mut IDirect3DCubeTexture9)>,
    pub GetType: Option<unsafe extern "system" fn(
        This: *mut IDirect3DCubeTexture9,
    ) -> *mut IDirect3DCubeTexture9>,
    pub SetLOD: Option<unsafe extern "system" fn(
        This: *mut IDirect3DCubeTexture9, LODNew: ::DWORD,
    ) -> ::DWORD>,
    pub GetLOD: Option<unsafe extern "system" fn(
        This: *mut IDirect3DCubeTexture9,
    ) -> *mut IDirect3DCubeTexture9>,
    pub GetLevelCount: Option<unsafe extern "system" fn(
        This: *mut IDirect3DCubeTexture9,
    ) -> *mut IDirect3DCubeTexture9>,
    pub SetAutoGenFilterType: Option<unsafe extern "system" fn(
        This: *mut IDirect3DCubeTexture9, FilterType: ::D3DTEXTUREFILTERTYPE,
    ) -> ::HRESULT>,
    pub GetAutoGenFilterType: Option<unsafe extern "system" fn(
        This: *mut IDirect3DCubeTexture9,
    ) -> ::D3DTEXTUREFILTERTYPE>,
    pub GenerateMipSubLevels: Option<unsafe extern "system" fn(
        This: *mut IDirect3DCubeTexture9,
    )>,
    pub GetLevelDesc: Option<unsafe extern "system" fn(
        This: *mut IDirect3DCubeTexture9, Level: ::UINT, pDesc: *mut ::D3DSURFACE_DESC,
    ) -> ::HRESULT>,
    pub GetCubeMapSurface: Option<unsafe extern "system" fn(
        This: *mut IDirect3DCubeTexture9, FaceType: ::D3DCUBEMAP_FACES, Level: ::UINT,
        ppCubeMapSurface: *mut *mut IDirect3DSurface9,
    ) -> ::HRESULT>,
    pub LockRect: Option<unsafe extern "system" fn(
        This: *mut IDirect3DCubeTexture9, FaceType: ::D3DCUBEMAP_FACES, Level: ::UINT,
        pLockedRect: *mut ::D3DLOCKED_RECT, pRect: *const ::RECT, Flags: ::DWORD,
    ) -> ::HRESULT>,
    pub UnlockRect: Option<unsafe extern "system" fn(
        This: *mut IDirect3DCubeTexture9, FaceType: ::D3DCUBEMAP_FACES, Level: ::UINT,
    ) -> ::HRESULT>,
    pub AddDirtyRect: Option<unsafe extern "system" fn(
        This: *mut IDirect3DCubeTexture9, FaceType: ::D3DCUBEMAP_FACES, pDirtyRect: *const ::RECT,
    ) -> ::HRESULT>,
}
pub type LPDIRECT3DCUBETEXTURE9 = *mut IDirect3DCubeTexture9;
pub type PDIRECT3DCUBETEXTURE9 = *mut IDirect3DCubeTexture9;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IDirect3DVertexBuffer9 {
    pub lpVtbl: *mut IDirect3DVertexBuffer9Vtbl,
}
#[repr(C)]
pub struct IDirect3DVertexBuffer9Vtbl {
    pub QueryInterface: Option<unsafe extern "system" fn(
        This: *mut IDirect3DVertexBuffer9, riid: *const ::IID, ppvObj: *mut *mut ::VOID,
    ) -> ::HRESULT>,
    pub AddRef: Option<unsafe extern "system" fn(
        This: *mut IDirect3DVertexBuffer9,
    ) -> *mut IDirect3DVertexBuffer9>,
    pub Release: Option<unsafe extern "system" fn(
        This: *mut IDirect3DVertexBuffer9,
    ) -> *mut IDirect3DVertexBuffer9>,
    pub GetDevice: Option<unsafe extern "system" fn(
        This: *mut IDirect3DVertexBuffer9, ppDevice: *mut *mut IDirect3DDevice9,
    ) -> ::HRESULT>,
    pub SetPrivateData: Option<unsafe extern "system" fn(
        This: *mut IDirect3DVertexBuffer9, refguid: *const ::GUID, pData: *const ::VOID,
        SizeOfData: ::DWORD, Flags: ::DWORD,
    ) -> ::HRESULT>,
    pub GetPrivateData: Option<unsafe extern "system" fn(
        This: *mut IDirect3DVertexBuffer9, refguid: *const ::GUID, pData: *mut ::VOID,
        pSizeOfData: *mut ::DWORD,
    ) -> ::HRESULT>,
    pub FreePrivateData: Option<unsafe extern "system" fn(
        This: *mut IDirect3DVertexBuffer9, refguid: *const ::GUID,
    ) -> ::HRESULT>,
    pub SetPriority: Option<unsafe extern "system" fn(
        This: *mut IDirect3DVertexBuffer9, PriorityNew: ::DWORD,
    ) -> ::DWORD>,
    pub GetPriority: Option<unsafe extern "system" fn(
        This: *mut IDirect3DVertexBuffer9,
    ) -> *mut IDirect3DVertexBuffer9>,
    pub PreLoad: Option<unsafe extern "system" fn(This: *mut IDirect3DVertexBuffer9)>,
    pub GetType: Option<unsafe extern "system" fn(
        This: *mut IDirect3DVertexBuffer9,
    ) -> *mut IDirect3DVertexBuffer9>,
    pub Lock: Option<unsafe extern "system" fn(
        This: *mut IDirect3DVertexBuffer9, OffsetToLock: ::UINT, SizeToLock: ::UINT,
        ppbData: *mut *mut ::VOID, Flags: ::DWORD,
    ) -> ::HRESULT>,
    pub Unlock: Option<unsafe extern "system" fn(
        This: *mut IDirect3DVertexBuffer9,
    ) -> *mut IDirect3DVertexBuffer9>,
    pub GetDesc: Option<unsafe extern "system" fn(
        This: *mut IDirect3DVertexBuffer9, pDesc: *mut ::D3DVERTEXBUFFER_DESC,
    ) -> ::HRESULT>,
}
pub type LPDIRECT3DVERTEXBUFFER9 = *mut IDirect3DVertexBuffer9;
pub type PDIRECT3DVERTEXBUFFER9 = *mut IDirect3DVertexBuffer9;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IDirect3DIndexBuffer9 {
    pub lpVtbl: *mut IDirect3DIndexBuffer9Vtbl,
}
#[repr(C)]
pub struct IDirect3DIndexBuffer9Vtbl {
    pub QueryInterface: Option<unsafe extern "system" fn(
        This: *mut IDirect3DIndexBuffer9, riid: *const ::IID, ppvObj: *mut *mut ::VOID,
    ) -> ::HRESULT>,
    pub AddRef: Option<unsafe extern "system" fn(
        This: *mut IDirect3DIndexBuffer9,
    ) -> *mut IDirect3DIndexBuffer9>,
    pub Release: Option<unsafe extern "system" fn(
        This: *mut IDirect3DIndexBuffer9,
    ) -> *mut IDirect3DIndexBuffer9>,
    pub GetDevice: Option<unsafe extern "system" fn(
        This: *mut IDirect3DIndexBuffer9, ppDevice: *mut *mut IDirect3DDevice9,
    ) -> ::HRESULT>,
    pub SetPrivateData: Option<unsafe extern "system" fn(
        This: *mut IDirect3DIndexBuffer9, refguid: *const ::GUID, pData: *const ::VOID,
        SizeOfData: ::DWORD, Flags: ::DWORD,
    ) -> ::HRESULT>,
    pub GetPrivateData: Option<unsafe extern "system" fn(
        This: *mut IDirect3DIndexBuffer9, refguid: *const ::GUID, pData: *mut ::VOID,
        pSizeOfData: *mut ::DWORD,
    ) -> ::HRESULT>,
    pub FreePrivateData: Option<unsafe extern "system" fn(
        This: *mut IDirect3DIndexBuffer9, refguid: *const ::GUID,
    ) -> ::HRESULT>,
    pub SetPriority: Option<unsafe extern "system" fn(
        This: *mut IDirect3DIndexBuffer9, PriorityNew: ::DWORD,
    ) -> ::DWORD>,
    pub GetPriority: Option<unsafe extern "system" fn(
        This: *mut IDirect3DIndexBuffer9,
    ) -> *mut IDirect3DIndexBuffer9>,
    pub PreLoad: Option<unsafe extern "system" fn(This: *mut IDirect3DIndexBuffer9)>,
    pub GetType: Option<unsafe extern "system" fn(
        This: *mut IDirect3DIndexBuffer9,
    ) -> *mut IDirect3DIndexBuffer9>,
    pub Lock: Option<unsafe extern "system" fn(
        This: *mut IDirect3DIndexBuffer9, OffsetToLock: ::UINT, SizeToLock: ::UINT,
        ppbData: *mut *mut ::VOID, Flags: ::DWORD,
    ) -> ::HRESULT>,
    pub Unlock: Option<unsafe extern "system" fn(
        This: *mut IDirect3DIndexBuffer9,
    ) -> *mut IDirect3DIndexBuffer9>,
    pub GetDesc: Option<unsafe extern "system" fn(
        This: *mut IDirect3DIndexBuffer9, pDesc: *mut ::D3DINDEXBUFFER_DESC,
    ) -> ::HRESULT>,
}
pub type LPDIRECT3DINDEXBUFFER9 = *mut IDirect3DIndexBuffer9;
pub type PDIRECT3DINDEXBUFFER9 = *mut IDirect3DIndexBuffer9;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IDirect3DSurface9 {
    pub lpVtbl: *mut IDirect3DSurface9Vtbl,
}
#[repr(C)]
pub struct IDirect3DSurface9Vtbl {
    pub QueryInterface: Option<unsafe extern "system" fn(
        This: *mut IDirect3DSurface9, riid: *const ::IID, ppvObj: *mut *mut ::VOID,
    ) -> ::HRESULT>,
    pub AddRef: Option<unsafe extern "system" fn(
        This: *mut IDirect3DSurface9,
    ) -> *mut IDirect3DSurface9>,
    pub Release: Option<unsafe extern "system" fn(
        This: *mut IDirect3DSurface9,
    ) -> *mut IDirect3DSurface9>,
    pub GetDevice: Option<unsafe extern "system" fn(
        This: *mut IDirect3DSurface9, ppDevice: *mut *mut IDirect3DDevice9,
    ) -> ::HRESULT>,
    pub SetPrivateData: Option<unsafe extern "system" fn(
        This: *mut IDirect3DSurface9, refguid: *const ::GUID, pData: *const ::VOID,
        SizeOfData: ::DWORD, Flags: ::DWORD,
    ) -> ::HRESULT>,
    pub GetPrivateData: Option<unsafe extern "system" fn(
        This: *mut IDirect3DSurface9, refguid: *const ::GUID, pData: *mut ::VOID,
        pSizeOfData: *mut ::DWORD,
    ) -> ::HRESULT>,
    pub FreePrivateData: Option<unsafe extern "system" fn(
        This: *mut IDirect3DSurface9, refguid: *const ::GUID,
    ) -> ::HRESULT>,
    pub SetPriority: Option<unsafe extern "system" fn(
        This: *mut IDirect3DSurface9, PriorityNew: ::DWORD,
    ) -> ::DWORD>,
    pub GetPriority: Option<unsafe extern "system" fn(
        This: *mut IDirect3DSurface9,
    ) -> *mut IDirect3DSurface9>,
    pub PreLoad: Option<unsafe extern "system" fn(This: *mut IDirect3DSurface9)>,
    pub GetType: Option<unsafe extern "system" fn(
        This: *mut IDirect3DSurface9,
    ) -> *mut IDirect3DSurface9>,
    pub GetContainer: Option<unsafe extern "system" fn(
        This: *mut IDirect3DSurface9, riid: *const ::IID, ppContainer: *mut *mut ::VOID,
    ) -> ::HRESULT>,
    pub GetDesc: Option<unsafe extern "system" fn(
        This: *mut IDirect3DSurface9, pDesc: *mut ::D3DSURFACE_DESC,
    ) -> ::HRESULT>,
    pub LockRect: Option<unsafe extern "system" fn(
        This: *mut IDirect3DSurface9, pLockedRect: *mut ::D3DLOCKED_RECT, pRect: *const ::RECT,
        Flags: ::DWORD,
    ) -> ::HRESULT>,
    pub UnlockRect: Option<unsafe extern "system" fn(
        This: *mut IDirect3DSurface9,
    ) -> *mut IDirect3DSurface9>,
    pub GetDC: Option<unsafe extern "system" fn(
        This: *mut IDirect3DSurface9, phdc: *mut ::HDC,
    ) -> ::HRESULT>,
    pub ReleaseDC: Option<unsafe extern "system" fn(
        This: *mut IDirect3DSurface9, hdc: ::HDC,
    ) -> ::HRESULT>,
}
pub type LPDIRECT3DSURFACE9 = *mut IDirect3DSurface9;
pub type PDIRECT3DSURFACE9 = *mut IDirect3DSurface9;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IDirect3DVolume9 {
    pub lpVtbl: *mut IDirect3DVolume9Vtbl,
}
#[repr(C)]
pub struct IDirect3DVolume9Vtbl {
    pub QueryInterface: Option<unsafe extern "system" fn(
        This: *mut IDirect3DVolume9, riid: *const ::IID, ppvObj: *mut *mut ::VOID,
    ) -> ::HRESULT>,
    pub AddRef: Option<unsafe extern "system" fn(
        This: *mut IDirect3DVolume9,
    ) -> *mut IDirect3DVolume9>,
    pub Release: Option<unsafe extern "system" fn(
        This: *mut IDirect3DVolume9,
    ) -> *mut IDirect3DVolume9>,
    pub GetDevice: Option<unsafe extern "system" fn(
        This: *mut IDirect3DVolume9, ppDevice: *mut *mut IDirect3DDevice9,
    ) -> ::HRESULT>,
    pub SetPrivateData: Option<unsafe extern "system" fn(
        This: *mut IDirect3DVolume9, refguid: *const ::GUID, pData: *const ::VOID,
        SizeOfData: ::DWORD, Flags: ::DWORD,
    ) -> ::HRESULT>,
    pub GetPrivateData: Option<unsafe extern "system" fn(
        This: *mut IDirect3DVolume9, refguid: *const ::GUID, pData: *mut ::VOID,
        pSizeOfData: *mut ::DWORD,
    ) -> ::HRESULT>,
    pub FreePrivateData: Option<unsafe extern "system" fn(
        This: *mut IDirect3DVolume9, refguid: *const ::GUID,
    ) -> ::HRESULT>,
    pub GetContainer: Option<unsafe extern "system" fn(
        This: *mut IDirect3DVolume9, riid: *const ::IID, ppContainer: *mut *mut ::VOID,
    ) -> ::HRESULT>,
    pub GetDesc: Option<unsafe extern "system" fn(
        This: *mut IDirect3DVolume9, pDesc: *mut ::D3DVOLUME_DESC,
    ) -> ::HRESULT>,
    pub LockBox: Option<unsafe extern "system" fn(
        This: *mut IDirect3DVolume9, pLockedVolume: *mut ::D3DLOCKED_BOX, pBox: *const ::D3DBOX,
        Flags: ::DWORD,
    ) -> ::HRESULT>,
    pub UnlockBox: Option<unsafe extern "system" fn(
        This: *mut IDirect3DVolume9,
    ) -> ::HRESULT>,
}
pub type LPDIRECT3DVOLUME9 = *mut IDirect3DVolume9;
pub type PDIRECT3DVOLUME9 = *mut IDirect3DVolume9;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IDirect3DQuery9 {
    pub lpVtbl: *mut IDirect3DQuery9Vtbl,
}
#[repr(C)]
pub struct IDirect3DQuery9Vtbl {
    pub QueryInterface: Option<unsafe extern "system" fn(
        This: *mut IDirect3DQuery9, riid: *const ::IID, ppvObj: *mut *mut ::VOID,
    ) -> ::HRESULT>,
    pub AddRef: Option<unsafe extern "system" fn(
        This: *mut IDirect3DQuery9,
    ) -> *mut IDirect3DQuery9>,
    pub Release: Option<unsafe extern "system" fn(
        This: *mut IDirect3DQuery9,
    ) -> *mut IDirect3DQuery9>,
    pub GetDevice: Option<unsafe extern "system" fn(
        This: *mut IDirect3DQuery9, ppDevice: *mut *mut IDirect3DDevice9,
    ) -> ::HRESULT>,
    pub GetType: Option<unsafe extern "system" fn(
        This: *mut IDirect3DQuery9,
    ) -> *mut IDirect3DQuery9>,
    pub GetDataSize: Option<unsafe extern "system" fn(
        This: *mut IDirect3DQuery9,
    ) -> *mut IDirect3DQuery9>,
    pub Issue: Option<unsafe extern "system" fn(
        This: *mut IDirect3DQuery9, dwIssueFlags: ::DWORD,
    ) -> ::HRESULT>,
    pub GetData: Option<unsafe extern "system" fn(
        This: *mut IDirect3DQuery9, pData: *mut ::VOID, dwSize: ::DWORD, dwGetDataFlags: ::DWORD,
    ) -> ::HRESULT>,
}
pub type LPDIRECT3DQUERY9 = *mut IDirect3DQuery9;
pub type PDIRECT3DQUERY9 = *mut IDirect3DQuery9;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IDirect3D9Ex {
    pub lpVtbl: *mut IDirect3D9ExVtbl,
}
#[repr(C)]
pub struct IDirect3D9ExVtbl {
    pub QueryInterface: Option<unsafe extern "system" fn(
        This: *mut IDirect3D9Ex, riid: *const ::IID, ppvObj: *mut *mut ::VOID,
    ) -> ::HRESULT>,
    pub AddRef: Option<unsafe extern "system" fn(This: *mut IDirect3D9Ex) -> *mut IDirect3D9Ex>,
    pub Release: Option<unsafe extern "system" fn(This: *mut IDirect3D9Ex) -> *mut IDirect3D9Ex>,
    pub GetAdapterCount: Option<unsafe extern "system" fn(
        This: *mut IDirect3D9Ex,
    ) -> *mut IDirect3D9Ex>,
    pub GetAdapterIdentifier: Option<unsafe extern "system" fn(
        This: *mut IDirect3D9Ex, Adapter: ::UINT, Flags: ::DWORD,
        pIdentifier: *mut ::D3DADAPTER_IDENTIFIER9,
    ) -> ::HRESULT>,
    pub GetAdapterModeCount: Option<unsafe extern "system" fn(
        This: *mut IDirect3D9Ex, Adapter: ::UINT, Format: ::D3DFORMAT,
    ) -> ::UINT>,
    pub EnumAdapterModes: Option<unsafe extern "system" fn(
        This: *mut IDirect3D9Ex, Adapter: ::UINT, Format: ::D3DFORMAT, Mode: ::UINT,
        pMode: *mut ::D3DDISPLAYMODE,
    ) -> ::HRESULT>,
    pub GetAdapterDisplayMode: Option<unsafe extern "system" fn(
        This: *mut IDirect3D9Ex, Adapter: ::UINT, pMode: *mut ::D3DDISPLAYMODE,
    ) -> ::HRESULT>,
    pub CheckDeviceType: Option<unsafe extern "system" fn(
        This: *mut IDirect3D9Ex, Adapter: ::UINT, DevType: ::D3DDEVTYPE,
        AdapterFormat: ::D3DFORMAT, BackBufferFormat: ::D3DFORMAT, bWindowed: ::BOOL,
    ) -> ::HRESULT>,
    pub CheckDeviceFormat: Option<unsafe extern "system" fn(
        This: *mut IDirect3D9Ex, Adapter: ::UINT, DeviceType: ::D3DDEVTYPE,
        AdapterFormat: ::D3DFORMAT, Usage: ::DWORD, RType: ::D3DRESOURCETYPE,
        CheckFormat: ::D3DFORMAT,
    ) -> ::HRESULT>,
    pub CheckDeviceMultiSampleType: Option<unsafe extern "system" fn(
        This: *mut IDirect3D9Ex, Adapter: ::UINT, DeviceType: ::D3DDEVTYPE,
        SurfaceFormat: ::D3DFORMAT, Windowed: ::BOOL, MultiSampleType: ::D3DMULTISAMPLE_TYPE,
        pQualityLevels: *mut ::DWORD,
    ) -> ::HRESULT>,
    pub CheckDepthStencilMatch: Option<unsafe extern "system" fn(
        This: *mut IDirect3D9Ex, Adapter: ::UINT, DeviceType: ::D3DDEVTYPE,
        AdapterFormat: ::D3DFORMAT, RenderTargetFormat: ::D3DFORMAT,
        DepthStencilFormat: ::D3DFORMAT,
    ) -> ::HRESULT>,
    pub CheckDeviceFormatConversion: Option<unsafe extern "system" fn(
        This: *mut IDirect3D9Ex, Adapter: ::UINT, DeviceType: ::D3DDEVTYPE,
        SourceFormat: ::D3DFORMAT, TargetFormat: ::D3DFORMAT,
    ) -> ::HRESULT>,
    pub GetDeviceCaps: Option<unsafe extern "system" fn(
        This: *mut IDirect3D9Ex, Adapter: ::UINT, DeviceType: ::D3DDEVTYPE, pCaps: *mut ::D3DCAPS9,
    ) -> ::HRESULT>,
    pub GetAdapterMonitor: Option<unsafe extern "system" fn(
        This: *mut IDirect3D9Ex, Adapter: ::UINT,
    ) -> ::HMONITOR>,
    pub CreateDevice: Option<unsafe extern "system" fn(
        This: *mut IDirect3D9Ex, Adapter: ::UINT, DeviceType: ::D3DDEVTYPE, hFocusWindow: ::HWND,
        BehaviorFlags: ::DWORD, pPresentationParameters: *mut ::D3DPRESENT_PARAMETERS,
        ppReturnedDeviceInterface: *mut *mut IDirect3DDevice9,
    ) -> ::HRESULT>,
    pub GetAdapterModeCountEx: Option<unsafe extern "system" fn(
        This: *mut IDirect3D9Ex, Adapter: ::UINT, pFilter: *const ::D3DDISPLAYMODEFILTER,
    ) -> ::UINT>,
    pub EnumAdapterModesEx: Option<unsafe extern "system" fn(
        This: *mut IDirect3D9Ex, Adapter: ::UINT, pFilter: *const ::D3DDISPLAYMODEFILTER,
        Mode: ::UINT, pMode: *mut ::D3DDISPLAYMODEEX,
    ) -> ::HRESULT>,
    pub GetAdapterDisplayModeEx: Option<unsafe extern "system" fn(
        This: *mut IDirect3D9Ex, Adapter: ::UINT, pMode: *mut ::D3DDISPLAYMODEEX,
        pRotation: *mut ::D3DDISPLAYROTATION,
    ) -> ::HRESULT>,
    pub CreateDeviceEx: Option<unsafe extern "system" fn(
        This: *mut IDirect3D9Ex, Adapter: ::UINT, DeviceType: ::D3DDEVTYPE, hFocusWindow: ::HWND,
        BehaviorFlags: ::DWORD, pPresentationParameters: *mut ::D3DPRESENT_PARAMETERS,
        pFullscreenDisplayMode: *mut ::D3DDISPLAYMODEEX,
        ppReturnedDeviceInterface: *mut *mut IDirect3DDevice9Ex,
    ) -> ::HRESULT>,
    pub GetAdapterLUID: Option<unsafe extern "system" fn(
        This: *mut IDirect3D9Ex, Adapter: ::UINT, pLUID: *mut ::LUID,
    ) -> ::HRESULT>,
}
pub type LPDIRECT3D9EX = *mut IDirect3D9Ex;
pub type PDIRECT3D9EX = *mut IDirect3D9Ex;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IDirect3DDevice9Ex {
    pub lpVtbl: *mut IDirect3DDevice9ExVtbl,
}
#[repr(C)]
pub struct IDirect3DDevice9ExVtbl {
    pub QueryInterface: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, riid: *const ::IID, ppvObj: *mut *mut ::VOID,
    ) -> ::HRESULT>,
    pub AddRef: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex,
    ) -> *mut IDirect3DDevice9Ex>,
    pub Release: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex,
    ) -> *mut IDirect3DDevice9Ex>,
    pub TestCooperativeLevel: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex,
    ) -> *mut IDirect3DDevice9Ex>,
    pub GetAvailableTextureMem: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex,
    ) -> *mut IDirect3DDevice9Ex>,
    pub EvictManagedResources: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex,
    ) -> *mut IDirect3DDevice9Ex>,
    pub GetDirect3D: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, ppD3D9: *mut *mut IDirect3D9,
    ) -> ::HRESULT>,
    pub GetDeviceCaps: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, pCaps: *mut ::D3DCAPS9,
    ) -> ::HRESULT>,
    pub GetDisplayMode: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, iSwapChain: ::UINT, pMode: *mut ::D3DDISPLAYMODE,
    ) -> ::HRESULT>,
    pub GetCreationParameters: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, pParameters: *mut ::D3DDEVICE_CREATION_PARAMETERS,
    ) -> ::HRESULT>,
    pub SetCursorProperties: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, XHotSpot: ::UINT, YHotSpot: ::UINT,
        pCursorBitmap: *mut IDirect3DSurface9,
    ) -> ::HRESULT>,
    pub SetCursorPosition: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, X: ::INT, Y: ::INT, Flags: ::DWORD,
    )>,
    pub ShowCursor: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, bShow: ::BOOL,
    ) -> ::BOOL>,
    pub CreateAdditionalSwapChain: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, pPresentationParameters: *mut ::D3DPRESENT_PARAMETERS,
        pSwapChain: *mut *mut IDirect3DSwapChain9,
    ) -> ::HRESULT>,
    pub GetSwapChain: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, iSwapChain: ::UINT,
        pSwapChain: *mut *mut IDirect3DSwapChain9,
    ) -> ::HRESULT>,
    pub GetNumberOfSwapChains: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex,
    ) -> *mut IDirect3DDevice9Ex>,
    pub Reset: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, pPresentationParameters: *mut ::D3DPRESENT_PARAMETERS,
    ) -> ::HRESULT>,
    pub Present: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, pSourceRect: *const ::RECT, pDestRect: *const ::RECT,
        hDestWindowOverride: ::HWND, pDirtyRegion: *const ::RGNDATA,
    ) -> ::HRESULT>,
    pub GetBackBuffer: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, iSwapChain: ::UINT, iBackBuffer: ::UINT,
        Type: ::D3DBACKBUFFER_TYPE, ppBackBuffer: *mut *mut IDirect3DSurface9,
    ) -> ::HRESULT>,
    pub GetRasterStatus: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, iSwapChain: ::UINT, pRasterStatus: *mut ::D3DRASTER_STATUS,
    ) -> ::HRESULT>,
    pub SetDialogBoxMode: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, bEnableDialogs: ::BOOL,
    ) -> ::HRESULT>,
    pub SetGammaRamp: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, iSwapChain: ::UINT, Flags: ::DWORD,
        pRamp: *const ::D3DGAMMARAMP,
    )>,
    pub GetGammaRamp: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, iSwapChain: ::UINT, pRamp: *mut ::D3DGAMMARAMP,
    )>,
    pub CreateTexture: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, Width: ::UINT, Height: ::UINT, Levels: ::UINT,
        Usage: ::DWORD, Format: ::D3DFORMAT, Pool: ::D3DPOOL,
        ppTexture: *mut *mut IDirect3DTexture9, pSharedHandle: *mut ::HANDLE,
    ) -> ::HRESULT>,
    pub CreateVolumeTexture: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, Width: ::UINT, Height: ::UINT, Depth: ::UINT,
        Levels: ::UINT, Usage: ::DWORD, Format: ::D3DFORMAT, Pool: ::D3DPOOL,
        ppVolumeTexture: *mut *mut IDirect3DVolumeTexture9, pSharedHandle: *mut ::HANDLE,
    ) -> ::HRESULT>,
    pub CreateCubeTexture: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, EdgeLength: ::UINT, Levels: ::UINT, Usage: ::DWORD,
        Format: ::D3DFORMAT, Pool: ::D3DPOOL, ppCubeTexture: *mut *mut IDirect3DCubeTexture9,
        pSharedHandle: *mut ::HANDLE,
    ) -> ::HRESULT>,
    pub CreateVertexBuffer: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, Length: ::UINT, Usage: ::DWORD, FVF: ::DWORD,
        Pool: ::D3DPOOL, ppVertexBuffer: *mut *mut IDirect3DVertexBuffer9,
        pSharedHandle: *mut ::HANDLE,
    ) -> ::HRESULT>,
    pub CreateIndexBuffer: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, Length: ::UINT, Usage: ::DWORD, Format: ::D3DFORMAT,
        Pool: ::D3DPOOL, ppIndexBuffer: *mut *mut IDirect3DIndexBuffer9,
        pSharedHandle: *mut ::HANDLE,
    ) -> ::HRESULT>,
    pub CreateRenderTarget: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, Width: ::UINT, Height: ::UINT, Format: ::D3DFORMAT,
        MultiSample: ::D3DMULTISAMPLE_TYPE, MultisampleQuality: ::DWORD, Lockable: ::BOOL,
        ppSurface: *mut *mut IDirect3DSurface9, pSharedHandle: *mut ::HANDLE,
    ) -> ::HRESULT>,
    pub CreateDepthStencilSurface: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, Width: ::UINT, Height: ::UINT, Format: ::D3DFORMAT,
        MultiSample: ::D3DMULTISAMPLE_TYPE, MultisampleQuality: ::DWORD, Discard: ::BOOL,
        ppSurface: *mut *mut IDirect3DSurface9, pSharedHandle: *mut ::HANDLE,
    ) -> ::HRESULT>,
    pub UpdateSurface: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, pSourceSurface: *mut IDirect3DSurface9,
        pSourceRect: *const ::RECT, pDestinationSurface: *mut IDirect3DSurface9,
        pDestPoint: *const ::POINT,
    ) -> ::HRESULT>,
    pub UpdateTexture: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, pSourceTexture: *mut IDirect3DBaseTexture9,
        pDestinationTexture: *mut IDirect3DBaseTexture9,
    ) -> ::HRESULT>,
    pub GetRenderTargetData: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, pRenderTarget: *mut IDirect3DSurface9,
        pDestSurface: *mut IDirect3DSurface9,
    ) -> ::HRESULT>,
    pub GetFrontBufferData: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, iSwapChain: ::UINT, pDestSurface: *mut IDirect3DSurface9,
    ) -> ::HRESULT>,
    pub StretchRect: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, pSourceSurface: *mut IDirect3DSurface9,
        pSourceRect: *const ::RECT, pDestSurface: *mut IDirect3DSurface9, pDestRect: *const ::RECT,
        Filter: ::D3DTEXTUREFILTERTYPE,
    ) -> ::HRESULT>,
    pub ColorFill: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, pSurface: *mut IDirect3DSurface9, pRect: *const ::RECT,
        color: ::D3DCOLOR,
    ) -> ::HRESULT>,
    pub CreateOffscreenPlainSurface: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, Width: ::UINT, Height: ::UINT, Format: ::D3DFORMAT,
        Pool: ::D3DPOOL, ppSurface: *mut *mut IDirect3DSurface9, pSharedHandle: *mut ::HANDLE,
    ) -> ::HRESULT>,
    pub SetRenderTarget: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, RenderTargetIndex: ::DWORD,
        pRenderTarget: *mut IDirect3DSurface9,
    ) -> ::HRESULT>,
    pub GetRenderTarget: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, RenderTargetIndex: ::DWORD,
        ppRenderTarget: *mut *mut IDirect3DSurface9,
    ) -> ::HRESULT>,
    pub SetDepthStencilSurface: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, pNewZStencil: *mut IDirect3DSurface9,
    ) -> ::HRESULT>,
    pub GetDepthStencilSurface: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, ppZStencilSurface: *mut *mut IDirect3DSurface9,
    ) -> ::HRESULT>,
    pub BeginScene: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex,
    ) -> *mut IDirect3DDevice9Ex>,
    pub EndScene: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex,
    ) -> *mut IDirect3DDevice9Ex>,
    pub Clear: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, Count: ::DWORD, pRects: *const ::D3DRECT, Flags: ::DWORD,
        Color: ::D3DCOLOR, Z: ::FLOAT, Stencil: ::DWORD,
    ) -> ::HRESULT>,
    pub SetTransform: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, State: ::D3DTRANSFORMSTATETYPE, pMatrix: *const ::D3DMATRIX,
    ) -> ::HRESULT>,
    pub GetTransform: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, State: ::D3DTRANSFORMSTATETYPE, pMatrix: *mut ::D3DMATRIX,
    ) -> ::HRESULT>,
    pub MultiplyTransform: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, arg1: ::D3DTRANSFORMSTATETYPE, arg2: *const ::D3DMATRIX,
    ) -> ::HRESULT>,
    pub SetViewport: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, pViewport: *const ::D3DVIEWPORT9,
    ) -> ::HRESULT>,
    pub GetViewport: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, pViewport: *mut ::D3DVIEWPORT9,
    ) -> ::HRESULT>,
    pub SetMaterial: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, pMaterial: *const ::D3DMATERIAL9,
    ) -> ::HRESULT>,
    pub GetMaterial: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, pMaterial: *mut ::D3DMATERIAL9,
    ) -> ::HRESULT>,
    pub SetLight: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, Index: ::DWORD, arg1: *const ::D3DLIGHT9,
    ) -> ::HRESULT>,
    pub GetLight: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, Index: ::DWORD, arg1: *mut ::D3DLIGHT9,
    ) -> ::HRESULT>,
    pub LightEnable: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, Index: ::DWORD, Enable: ::BOOL,
    ) -> ::HRESULT>,
    pub GetLightEnable: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, Index: ::DWORD, pEnable: *mut ::BOOL,
    ) -> ::HRESULT>,
    pub SetClipPlane: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, Index: ::DWORD, pPlane: *const ::FLOAT,
    ) -> ::HRESULT>,
    pub GetClipPlane: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, Index: ::DWORD, pPlane: *mut ::FLOAT,
    ) -> ::HRESULT>,
    pub SetRenderState: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, State: ::D3DRENDERSTATETYPE, Value: ::DWORD,
    ) -> ::HRESULT>,
    pub GetRenderState: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, State: ::D3DRENDERSTATETYPE, pValue: *mut ::DWORD,
    ) -> ::HRESULT>,
    pub CreateStateBlock: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, Type: ::D3DSTATEBLOCKTYPE,
        ppSB: *mut *mut IDirect3DStateBlock9,
    ) -> ::HRESULT>,
    pub BeginStateBlock: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex,
    ) -> *mut IDirect3DDevice9Ex>,
    pub EndStateBlock: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, ppSB: *mut *mut IDirect3DStateBlock9,
    ) -> ::HRESULT>,
    pub SetClipStatus: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, pClipStatus: *const ::D3DCLIPSTATUS9,
    ) -> ::HRESULT>,
    pub GetClipStatus: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, pClipStatus: *mut ::D3DCLIPSTATUS9,
    ) -> ::HRESULT>,
    pub GetTexture: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, Stage: ::DWORD, ppTexture: *mut *mut IDirect3DBaseTexture9,
    ) -> ::HRESULT>,
    pub SetTexture: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, Stage: ::DWORD, pTexture: *mut IDirect3DBaseTexture9,
    ) -> ::HRESULT>,
    pub GetTextureStageState: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, Stage: ::DWORD, Type: ::D3DTEXTURESTAGESTATETYPE,
        pValue: *mut ::DWORD,
    ) -> ::HRESULT>,
    pub SetTextureStageState: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, Stage: ::DWORD, Type: ::D3DTEXTURESTAGESTATETYPE,
        Value: ::DWORD,
    ) -> ::HRESULT>,
    pub GetSamplerState: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, Sampler: ::DWORD, Type: ::D3DSAMPLERSTATETYPE,
        pValue: *mut ::DWORD,
    ) -> ::HRESULT>,
    pub SetSamplerState: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, Sampler: ::DWORD, Type: ::D3DSAMPLERSTATETYPE,
        Value: ::DWORD,
    ) -> ::HRESULT>,
    pub ValidateDevice: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, pNumPasses: *mut ::DWORD,
    ) -> ::HRESULT>,
    pub SetPaletteEntries: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, PaletteNumber: ::UINT, pEntries: *const ::PALETTEENTRY,
    ) -> ::HRESULT>,
    pub GetPaletteEntries: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, PaletteNumber: ::UINT, pEntries: *mut ::PALETTEENTRY,
    ) -> ::HRESULT>,
    pub SetCurrentTexturePalette: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, PaletteNumber: ::UINT,
    ) -> ::HRESULT>,
    pub GetCurrentTexturePalette: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, PaletteNumber: *mut ::UINT,
    ) -> ::HRESULT>,
    pub SetScissorRect: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, pRect: *const ::RECT,
    ) -> ::HRESULT>,
    pub GetScissorRect: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, pRect: *mut ::RECT,
    ) -> ::HRESULT>,
    pub SetSoftwareVertexProcessing: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, bSoftware: ::BOOL,
    ) -> ::HRESULT>,
    pub GetSoftwareVertexProcessing: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex,
    ) -> *mut IDirect3DDevice9Ex>,
    pub SetNPatchMode: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, nSegments: ::FLOAT,
    ) -> ::HRESULT>,
    pub GetNPatchMode: Option<unsafe extern "system" fn(This: *mut IDirect3DDevice9Ex) -> ::FLOAT>,
    pub DrawPrimitive: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, PrimitiveType: ::D3DPRIMITIVETYPE, StartVertex: ::UINT,
        PrimitiveCount: ::UINT,
    ) -> ::HRESULT>,
    pub DrawIndexedPrimitive: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, arg1: ::D3DPRIMITIVETYPE, BaseVertexIndex: ::INT,
        MinVertexIndex: ::UINT, NumVertices: ::UINT, startIndex: ::UINT, primCount: ::UINT,
    ) -> ::HRESULT>,
    pub DrawPrimitiveUP: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, PrimitiveType: ::D3DPRIMITIVETYPE, PrimitiveCount: ::UINT,
        pVertexStreamZeroData: *const ::VOID, VertexStreamZeroStride: ::UINT,
    ) -> ::HRESULT>,
    pub DrawIndexedPrimitiveUP: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, PrimitiveType: ::D3DPRIMITIVETYPE, MinVertexIndex: ::UINT,
        NumVertices: ::UINT, PrimitiveCount: ::UINT, pIndexData: *const ::VOID,
        IndexDataFormat: ::D3DFORMAT, pVertexStreamZeroData: *const ::VOID,
        VertexStreamZeroStride: ::UINT,
    ) -> ::HRESULT>,
    pub ProcessVertices: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, SrcStartIndex: ::UINT, DestIndex: ::UINT,
        VertexCount: ::UINT, pDestBuffer: *mut IDirect3DVertexBuffer9,
        pVertexDecl: *mut IDirect3DVertexDeclaration9, Flags: ::DWORD,
    ) -> ::HRESULT>,
    pub CreateVertexDeclaration: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, pVertexElements: *const ::D3DVERTEXELEMENT9,
        ppDecl: *mut *mut IDirect3DVertexDeclaration9,
    ) -> ::HRESULT>,
    pub SetVertexDeclaration: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, pDecl: *mut IDirect3DVertexDeclaration9,
    ) -> ::HRESULT>,
    pub GetVertexDeclaration: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, ppDecl: *mut *mut IDirect3DVertexDeclaration9,
    ) -> ::HRESULT>,
    pub SetFVF: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, FVF: ::DWORD,
    ) -> ::HRESULT>,
    pub GetFVF: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, pFVF: *mut ::DWORD,
    ) -> ::HRESULT>,
    pub CreateVertexShader: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, pFunction: *const ::DWORD,
        ppShader: *mut *mut IDirect3DVertexShader9,
    ) -> ::HRESULT>,
    pub SetVertexShader: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, pShader: *mut IDirect3DVertexShader9,
    ) -> ::HRESULT>,
    pub GetVertexShader: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, ppShader: *mut *mut IDirect3DVertexShader9,
    ) -> ::HRESULT>,
    pub SetVertexShaderConstantF: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, StartRegister: ::UINT, pConstantData: *const ::FLOAT,
        Vector4fCount: ::UINT,
    ) -> ::HRESULT>,
    pub GetVertexShaderConstantF: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, StartRegister: ::UINT, pConstantData: *mut ::FLOAT,
        Vector4fCount: ::UINT,
    ) -> ::HRESULT>,
    pub SetVertexShaderConstantI: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, StartRegister: ::UINT, pConstantData: *const ::INT,
        Vector4iCount: ::UINT,
    ) -> ::HRESULT>,
    pub GetVertexShaderConstantI: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, StartRegister: ::UINT, pConstantData: *mut ::INT,
        Vector4iCount: ::UINT,
    ) -> ::HRESULT>,
    pub SetVertexShaderConstantB: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, StartRegister: ::UINT, pConstantData: *const ::BOOL,
        BoolCount: ::UINT,
    ) -> ::HRESULT>,
    pub GetVertexShaderConstantB: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, StartRegister: ::UINT, pConstantData: *mut ::BOOL,
        BoolCount: ::UINT,
    ) -> ::HRESULT>,
    pub SetStreamSource: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, StreamNumber: ::UINT,
        pStreamData: *mut IDirect3DVertexBuffer9, OffsetInBytes: ::UINT, Stride: ::UINT,
    ) -> ::HRESULT>,
    pub GetStreamSource: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, StreamNumber: ::UINT,
        ppStreamData: *mut *mut IDirect3DVertexBuffer9, pOffsetInBytes: *mut ::UINT,
        pStride: *mut ::UINT,
    ) -> ::HRESULT>,
    pub SetStreamSourceFreq: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, StreamNumber: ::UINT, Setting: ::UINT,
    ) -> ::HRESULT>,
    pub GetStreamSourceFreq: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, StreamNumber: ::UINT, pSetting: *mut ::UINT,
    ) -> ::HRESULT>,
    pub SetIndices: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, pIndexData: *mut IDirect3DIndexBuffer9,
    ) -> ::HRESULT>,
    pub GetIndices: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, ppIndexData: *mut *mut IDirect3DIndexBuffer9,
    ) -> ::HRESULT>,
    pub CreatePixelShader: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, pFunction: *const ::DWORD,
        ppShader: *mut *mut IDirect3DPixelShader9,
    ) -> ::HRESULT>,
    pub SetPixelShader: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, pShader: *mut IDirect3DPixelShader9,
    ) -> ::HRESULT>,
    pub GetPixelShader: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, ppShader: *mut *mut IDirect3DPixelShader9,
    ) -> ::HRESULT>,
    pub SetPixelShaderConstantF: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, StartRegister: ::UINT, pConstantData: *const ::FLOAT,
        Vector4fCount: ::UINT,
    ) -> ::HRESULT>,
    pub GetPixelShaderConstantF: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, StartRegister: ::UINT, pConstantData: *mut ::FLOAT,
        Vector4fCount: ::UINT,
    ) -> ::HRESULT>,
    pub SetPixelShaderConstantI: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, StartRegister: ::UINT, pConstantData: *const ::INT,
        Vector4iCount: ::UINT,
    ) -> ::HRESULT>,
    pub GetPixelShaderConstantI: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, StartRegister: ::UINT, pConstantData: *mut ::INT,
        Vector4iCount: ::UINT,
    ) -> ::HRESULT>,
    pub SetPixelShaderConstantB: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, StartRegister: ::UINT, pConstantData: *const ::BOOL,
        BoolCount: ::UINT,
    ) -> ::HRESULT>,
    pub GetPixelShaderConstantB: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, StartRegister: ::UINT, pConstantData: *mut ::BOOL,
        BoolCount: ::UINT,
    ) -> ::HRESULT>,
    pub DrawRectPatch: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, Handle: ::UINT, pNumSegs: *const ::FLOAT,
        pRectPatchInfo: *const ::D3DRECTPATCH_INFO,
    ) -> ::HRESULT>,
    pub DrawTriPatch: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, Handle: ::UINT, pNumSegs: *const ::FLOAT,
        pTriPatchInfo: *const ::D3DTRIPATCH_INFO,
    ) -> ::HRESULT>,
    pub DeletePatch: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, Handle: ::UINT,
    ) -> ::HRESULT>,
    pub CreateQuery: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, Type: ::D3DQUERYTYPE, ppQuery: *mut *mut IDirect3DQuery9,
    ) -> ::HRESULT>,
    pub SetConvolutionMonoKernel: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, width: ::UINT, height: ::UINT, rows: *mut ::FLOAT,
        columns: *mut ::FLOAT,
    ) -> ::HRESULT>,
    pub ComposeRects: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, pSrc: *mut IDirect3DSurface9, pDst: *mut IDirect3DSurface9,
        pSrcRectDescs: *mut IDirect3DVertexBuffer9, NumRects: ::UINT,
        pDstRectDescs: *mut IDirect3DVertexBuffer9, Operation: ::D3DCOMPOSERECTSOP, Xoffset: ::INT,
        Yoffset: ::INT,
    ) -> ::HRESULT>,
    pub PresentEx: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, pSourceRect: *const ::RECT, pDestRect: *const ::RECT,
        hDestWindowOverride: ::HWND, pDirtyRegion: *const ::RGNDATA, dwFlags: ::DWORD,
    ) -> ::HRESULT>,
    pub GetGPUThreadPriority: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, pPriority: *mut ::INT,
    ) -> ::HRESULT>,
    pub SetGPUThreadPriority: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, Priority: ::INT,
    ) -> ::HRESULT>,
    pub WaitForVBlank: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, iSwapChain: ::UINT,
    ) -> ::HRESULT>,
    pub CheckResourceResidency: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, pResourceArray: *mut *mut IDirect3DResource9,
        NumResources: ::UINT32,
    ) -> ::HRESULT>,
    pub SetMaximumFrameLatency: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, MaxLatency: ::UINT,
    ) -> ::HRESULT>,
    pub GetMaximumFrameLatency: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, pMaxLatency: *mut ::UINT,
    ) -> ::HRESULT>,
    pub CheckDeviceState: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, hDestinationWindow: ::HWND,
    ) -> ::HRESULT>,
    pub CreateRenderTargetEx: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, Width: ::UINT, Height: ::UINT, Format: ::D3DFORMAT,
        MultiSample: ::D3DMULTISAMPLE_TYPE, MultisampleQuality: ::DWORD, Lockable: ::BOOL,
        ppSurface: *mut *mut IDirect3DSurface9, pSharedHandle: *mut ::HANDLE, Usage: ::DWORD,
    ) -> ::HRESULT>,
    pub CreateOffscreenPlainSurfaceEx: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, Width: ::UINT, Height: ::UINT, Format: ::D3DFORMAT,
        Pool: ::D3DPOOL, ppSurface: *mut *mut IDirect3DSurface9, pSharedHandle: *mut ::HANDLE,
        Usage: ::DWORD,
    ) -> ::HRESULT>,
    pub CreateDepthStencilSurfaceEx: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, Width: ::UINT, Height: ::UINT, Format: ::D3DFORMAT,
        MultiSample: ::D3DMULTISAMPLE_TYPE, MultisampleQuality: ::DWORD, Discard: ::BOOL,
        ppSurface: *mut *mut IDirect3DSurface9, pSharedHandle: *mut ::HANDLE, Usage: ::DWORD,
    ) -> ::HRESULT>,
    pub ResetEx: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, pPresentationParameters: *mut ::D3DPRESENT_PARAMETERS,
        pFullscreenDisplayMode: *mut ::D3DDISPLAYMODEEX,
    ) -> ::HRESULT>,
    pub GetDisplayModeEx: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Ex, iSwapChain: ::UINT, pMode: *mut ::D3DDISPLAYMODEEX,
        pRotation: *mut ::D3DDISPLAYROTATION,
    ) -> ::HRESULT>,
}
pub type LPDIRECT3DDEVICE9EX = *mut IDirect3DDevice9Ex;
pub type PDIRECT3DDEVICE9EX = *mut IDirect3DDevice9Ex;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IDirect3DSwapChain9Ex {
    pub lpVtbl: *mut IDirect3DSwapChain9ExVtbl,
}
#[repr(C)]
pub struct IDirect3DSwapChain9ExVtbl {
    pub QueryInterface: Option<unsafe extern "system" fn(
        This: *mut IDirect3DSwapChain9Ex, riid: *const ::IID, ppvObj: *mut *mut ::VOID,
    ) -> ::HRESULT>,
    pub AddRef: Option<unsafe extern "system" fn(
        This: *mut IDirect3DSwapChain9Ex,
    ) -> *mut IDirect3DSwapChain9Ex>,
    pub Release: Option<unsafe extern "system" fn(
        This: *mut IDirect3DSwapChain9Ex,
    ) -> *mut IDirect3DSwapChain9Ex>,
    pub Present: Option<unsafe extern "system" fn(
        This: *mut IDirect3DSwapChain9Ex, pSourceRect: *const ::RECT, pDestRect: *const ::RECT,
        hDestWindowOverride: ::HWND, pDirtyRegion: *const ::RGNDATA, dwFlags: ::DWORD,
    ) -> ::HRESULT>,
    pub GetFrontBufferData: Option<unsafe extern "system" fn(
        This: *mut IDirect3DSwapChain9Ex, pDestSurface: *mut IDirect3DSurface9,
    ) -> ::HRESULT>,
    pub GetBackBuffer: Option<unsafe extern "system" fn(
        This: *mut IDirect3DSwapChain9Ex, iBackBuffer: ::UINT, Type: ::D3DBACKBUFFER_TYPE,
        ppBackBuffer: *mut *mut IDirect3DSurface9,
    ) -> ::HRESULT>,
    pub GetRasterStatus: Option<unsafe extern "system" fn(
        This: *mut IDirect3DSwapChain9Ex, pRasterStatus: *mut ::D3DRASTER_STATUS,
    ) -> ::HRESULT>,
    pub GetDisplayMode: Option<unsafe extern "system" fn(
        This: *mut IDirect3DSwapChain9Ex, pMode: *mut ::D3DDISPLAYMODE,
    ) -> ::HRESULT>,
    pub GetDevice: Option<unsafe extern "system" fn(
        This: *mut IDirect3DSwapChain9Ex, ppDevice: *mut *mut IDirect3DDevice9,
    ) -> ::HRESULT>,
    pub GetPresentParameters: Option<unsafe extern "system" fn(
        This: *mut IDirect3DSwapChain9Ex, pPresentationParameters: *mut ::D3DPRESENT_PARAMETERS,
    ) -> ::HRESULT>,
    pub GetLastPresentCount: Option<unsafe extern "system" fn(
        This: *mut IDirect3DSwapChain9Ex, pLastPresentCount: *mut ::UINT,
    ) -> ::HRESULT>,
    pub GetPresentStats: Option<unsafe extern "system" fn(
        This: *mut IDirect3DSwapChain9Ex, pPresentationStatistics: *mut ::D3DPRESENTSTATS,
    ) -> ::HRESULT>,
    pub GetDisplayModeEx: Option<unsafe extern "system" fn(
        This: *mut IDirect3DSwapChain9Ex, pMode: *mut ::D3DDISPLAYMODEEX,
        pRotation: *mut ::D3DDISPLAYROTATION,
    ) -> ::HRESULT>,
}
pub type LPDIRECT3DSWAPCHAIN9EX = *mut IDirect3DSwapChain9Ex;
pub type PDIRECT3DSWAPCHAIN9EX = *mut IDirect3DSwapChain9Ex;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IDirect3D9ExOverlayExtension {
    pub lpVtbl: *mut IDirect3D9ExOverlayExtensionVtbl,
}
#[repr(C)]
pub struct IDirect3D9ExOverlayExtensionVtbl {
    pub QueryInterface: Option<unsafe extern "system" fn(
        This: *mut IDirect3D9ExOverlayExtension, riid: *const ::IID, ppvObj: *mut *mut ::VOID,
    ) -> ::HRESULT>,
    pub AddRef: Option<unsafe extern "system" fn(
        This: *mut IDirect3D9ExOverlayExtension,
    ) -> *mut IDirect3D9ExOverlayExtension>,
    pub Release: Option<unsafe extern "system" fn(
        This: *mut IDirect3D9ExOverlayExtension,
    ) -> *mut IDirect3D9ExOverlayExtension>,
    pub CheckDeviceOverlayType: Option<unsafe extern "system" fn(
        This: *mut IDirect3D9ExOverlayExtension, Adapter: ::UINT, DevType: ::D3DDEVTYPE, OverlayWidth: ::UINT, OverlayHeight: ::UINT, OverlayFormat: ::D3DFORMAT, pDisplayMode: *mut ::D3DDISPLAYMODEEX, DisplayRotation: ::D3DDISPLAYROTATION, pOverlayCaps: *mut ::D3DOVERLAYCAPS,
    ) -> ::HRESULT>,
}
pub type LPDIRECT3D9EXOVERLAYEXTENSION = *mut IDirect3D9ExOverlayExtension;
pub type PDIRECT3D9EXOVERLAYEXTENSION = *mut IDirect3D9ExOverlayExtension;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IDirect3DDevice9Video {
    pub lpVtbl: *mut IDirect3DDevice9VideoVtbl,
}
#[repr(C)]
pub struct IDirect3DDevice9VideoVtbl {
    pub QueryInterface: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Video, riid: *const ::IID, ppvObj: *mut *mut ::VOID,
    ) -> ::HRESULT>,
    pub AddRef: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Video,
    ) -> *mut IDirect3DDevice9Video>,
    pub Release: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Video,
    ) -> *mut IDirect3DDevice9Video>,
    pub GetContentProtectionCaps: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Video, pCryptoType: *const ::GUID,
        pDecodeProfile: *const ::GUID, pCaps: *mut ::D3DCONTENTPROTECTIONCAPS,
    ) -> ::HRESULT>,
    pub CreateAuthenticatedChannel: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Video, ChannelType: ::D3DAUTHENTICATEDCHANNELTYPE,
        ppAuthenticatedChannel: *mut *mut IDirect3DAuthenticatedChannel9,
        pChannelHandle: *mut ::HANDLE,
    ) -> ::HRESULT>,
    pub CreateCryptoSession: Option<unsafe extern "system" fn(
        This: *mut IDirect3DDevice9Video, pCryptoType: *const ::GUID,
        pDecodeProfile: *const ::GUID, ppCryptoSession: *mut *mut IDirect3DCryptoSession9,
        pCryptoHandle: *mut ::HANDLE,
    ) -> ::HRESULT>,
}
pub type LPDIRECT3DDEVICE9VIDEO = *mut IDirect3DDevice9Video;
pub type PDIRECT3DDEVICE9VIDEO = *mut IDirect3DDevice9Video;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IDirect3DAuthenticatedChannel9 {
    pub lpVtbl: *mut IDirect3DAuthenticatedChannel9Vtbl,
}
#[repr(C)]
pub struct IDirect3DAuthenticatedChannel9Vtbl {
    pub QueryInterface: Option<unsafe extern "system" fn(
        This: *mut IDirect3DAuthenticatedChannel9, riid: *const ::IID, ppvObj: *mut *mut ::VOID,
    ) -> ::HRESULT>,
    pub AddRef: Option<unsafe extern "system" fn(
        This: *mut IDirect3DAuthenticatedChannel9,
    ) -> *mut IDirect3DAuthenticatedChannel9>,
    pub Release: Option<unsafe extern "system" fn(
        This: *mut IDirect3DAuthenticatedChannel9,
    ) -> *mut IDirect3DAuthenticatedChannel9>,
    pub GetCertificateSize: Option<unsafe extern "system" fn(
        This: *mut IDirect3DAuthenticatedChannel9, pCertificateSize: *mut ::UINT,
    ) -> ::HRESULT>,
    pub GetCertificate: Option<unsafe extern "system" fn(
        This: *mut IDirect3DAuthenticatedChannel9, CertifacteSize: ::UINT,
        ppCertificate: *mut ::BYTE,
    ) -> ::HRESULT>,
    pub NegotiateKeyExchange: Option<unsafe extern "system" fn(
        This: *mut IDirect3DAuthenticatedChannel9, DataSize: ::UINT, pData: *mut ::VOID,
    ) -> ::HRESULT>,
    pub Query: Option<unsafe extern "system" fn(
        This: *mut IDirect3DAuthenticatedChannel9, InputSize: ::UINT, pInput: *const ::VOID,
        OutputSize: ::UINT, pOutput: *mut ::VOID,
    ) -> ::HRESULT>,
    pub Configure: Option<unsafe extern "system" fn(
        This: *mut IDirect3DAuthenticatedChannel9, InputSize: ::UINT, pInput: *const ::VOID,
        pOutput: *mut ::D3DAUTHENTICATEDCHANNEL_CONFIGURE_OUTPUT,
    ) -> ::HRESULT>,
}
pub type LPDIRECT3DAUTHENTICATEDCHANNEL9 = *mut IDirect3DAuthenticatedChannel9;
pub type PDIRECT3DAUTHENTICATEDCHANNEL9 = *mut IDirect3DAuthenticatedChannel9;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IDirect3DCryptoSession9 {
    pub lpVtbl: *mut IDirect3DCryptoSession9Vtbl,
}
#[repr(C)]
pub struct IDirect3DCryptoSession9Vtbl {
    pub QueryInterface: Option<unsafe extern "system" fn(
        This: *mut IDirect3DCryptoSession9, riid: *const ::IID, ppvObj: *mut *mut ::VOID,
    ) -> ::HRESULT>,
    pub AddRef: Option<unsafe extern "system" fn(
        This: *mut IDirect3DCryptoSession9,
    ) -> *mut IDirect3DCryptoSession9>,
    pub Release: Option<unsafe extern "system" fn(
        This: *mut IDirect3DCryptoSession9,
    ) -> *mut IDirect3DCryptoSession9>,
    pub GetCertificateSize: Option<unsafe extern "system" fn(
        This: *mut IDirect3DCryptoSession9, pCertificateSize: *mut ::UINT,
    ) -> ::HRESULT>,
    pub GetCertificate: Option<unsafe extern "system" fn(
        This: *mut IDirect3DCryptoSession9, CertifacteSize: ::UINT, ppCertificate: *mut ::BYTE,
    ) -> ::HRESULT>,
    pub NegotiateKeyExchange: Option<unsafe extern "system" fn(
        This: *mut IDirect3DCryptoSession9, DataSize: ::UINT, pData: *mut ::VOID,
    ) -> ::HRESULT>,
    pub EncryptionBlt: Option<unsafe extern "system" fn(
        This: *mut IDirect3DCryptoSession9, pSrcSurface: *mut IDirect3DSurface9,
        pDstSurface: *mut IDirect3DSurface9, DstSurfaceSize: ::UINT, pIV: *mut ::VOID,
    ) -> ::HRESULT>,
    pub DecryptionBlt: Option<unsafe extern "system" fn(
        This: *mut IDirect3DCryptoSession9, pSrcSurface: *mut IDirect3DSurface9,
        pDstSurface: *mut IDirect3DSurface9, SrcSurfaceSize: ::UINT,
        pEncryptedBlockInfo: *mut ::D3DENCRYPTED_BLOCK_INFO, pContentKey: *mut ::VOID,
        pIV: *mut ::VOID,
    ) -> ::HRESULT>,
    pub GetSurfacePitch: Option<unsafe extern "system" fn(
        This: *mut IDirect3DCryptoSession9, pSrcSurface: *mut IDirect3DSurface9,
        pSurfacePitch: *mut ::UINT,
    ) -> ::HRESULT>,
    pub StartSessionKeyRefresh: Option<unsafe extern "system" fn(
        This: *mut IDirect3DCryptoSession9, pRandomNumber: *mut ::VOID, RandomNumberSize: ::UINT,
    ) -> ::HRESULT>,
    pub FinishSessionKeyRefresh: Option<unsafe extern "system" fn(
        This: *mut IDirect3DCryptoSession9,
    ) -> *mut IDirect3DCryptoSession9>,
    pub GetEncryptionBltKey: Option<unsafe extern "system" fn(
        This: *mut IDirect3DCryptoSession9, pReadbackKey: *mut ::VOID, KeySize: ::UINT,
    ) -> ::HRESULT>,
}
pub type LPDIRECT3DCRYPTOSESSION9 = *mut IDirect3DCryptoSession9;
pub type PDIRECT3DCRYPTOSESSION9 = *mut IDirect3DCryptoSession9;

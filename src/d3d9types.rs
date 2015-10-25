// Copyright © 2015, Corey Richardson
// Licensed under the MIT License <LICENSE.md>
//! Direct3D capabilities include file
pub type D3DCOLOR = ::DWORD;
STRUCT!{struct D3DVECTOR {
    x: ::c_float,
    y: ::c_float,
    z: ::c_float,
}}
STRUCT!{struct D3DCOLORVALUE {
    r: ::c_float,
    g: ::c_float,
    b: ::c_float,
    a: ::c_float,
}}
STRUCT!{struct D3DRECT {
    x1: ::LONG,
    y1: ::LONG,
    x2: ::LONG,
    y2: ::LONG,
}}
STRUCT!{struct D3DMATRIX {
    m: [[::c_float; 4]; 4],
}}
STRUCT!{struct D3DVIEWPORT9 {
    X: ::DWORD,
    Y: ::DWORD,
    Width: ::DWORD,
    Height: ::DWORD,
    MinZ: ::c_float,
    MaxZ: ::c_float,
}}
pub const D3DMAXUSERCLIPPLANES: ::DWORD = 32;
pub const D3DCLIPPLANE0: ::DWORD = (1 << 0);
pub const D3DCLIPPLANE1: ::DWORD = (1 << 1);
pub const D3DCLIPPLANE2: ::DWORD = (1 << 2);
pub const D3DCLIPPLANE3: ::DWORD = (1 << 3);
pub const D3DCLIPPLANE4: ::DWORD = (1 << 4);
pub const D3DCLIPPLANE5: ::DWORD = (1 << 5);
pub const D3DCS_LEFT: ::DWORD = 0x00000001;
pub const D3DCS_RIGHT: ::DWORD = 0x00000002;
pub const D3DCS_TOP: ::DWORD = 0x00000004;
pub const D3DCS_BOTTOM: ::DWORD = 0x00000008;
pub const D3DCS_FRONT: ::DWORD = 0x00000010;
pub const D3DCS_BACK: ::DWORD = 0x00000020;
pub const D3DCS_PLANE0: ::DWORD = 0x00000040;
pub const D3DCS_PLANE1: ::DWORD = 0x00000080;
pub const D3DCS_PLANE2: ::DWORD = 0x00000100;
pub const D3DCS_PLANE3: ::DWORD = 0x00000200;
pub const D3DCS_PLANE4: ::DWORD = 0x00000400;
pub const D3DCS_PLANE5: ::DWORD = 0x00000800;
pub const D3DCS_ALL: ::DWORD = D3DCS_LEFT | D3DCS_RIGHT | D3DCS_TOP | D3DCS_BOTTOM | D3DCS_FRONT
    | D3DCS_BACK | D3DCS_PLANE0 | D3DCS_PLANE1 | D3DCS_PLANE2 | D3DCS_PLANE3 | D3DCS_PLANE4
    | D3DCS_PLANE5;
STRUCT!{struct D3DCLIPSTATUS9 {
    ClipUnion: ::DWORD,
    ClipIntersection: ::DWORD,
}}
STRUCT!{struct D3DMATERIAL9 {
    Diffuse: D3DCOLORVALUE,
    Ambient: D3DCOLORVALUE,
    Specular: D3DCOLORVALUE,
    Emissive: D3DCOLORVALUE,
    Power: ::c_float,
}}
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum D3DLIGHTTYPE {
    POINT = 1,
    SPOT = 2,
    DIRECTIONAL = 3,
}
STRUCT!{struct D3DLIGHT9 {
    Type: D3DLIGHTTYPE,
    Diffuse: D3DCOLORVALUE,
    Specular: D3DCOLORVALUE,
    Ambient: D3DCOLORVALUE,
    Position: D3DVECTOR,
    Direction: D3DVECTOR,
    Range: ::c_float,
    Falloff: ::c_float,
    Attenuation0: ::c_float,
    Attenuation1: ::c_float,
    Attenuation2: ::c_float,
    Theta: ::c_float,
    Phi: ::c_float,
}}
pub const D3DCLEAR_TARGET: ::DWORD = 0x1;
pub const D3DCLEAR_ZBUFFER: ::DWORD = 0x2;
pub const D3DCLEAR_STENCIL: ::DWORD = 0x4;
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum D3DSHADEMODE {
    FLAT = 1,
    GOURAUD = 2,
    PHONG = 3,
}
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum D3DFILLMODE {
    POINT = 1,
    WIREFRAME = 2,
    SOLID = 3,
}
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum D3DBLEND {
    ZERO = 1,
    ONE = 2,
    SRCCOLOR = 3,
    INVSRCCOLOR = 4,
    SRCALPHA = 5,
    INVSRCALPHA = 6,
    DESTALPHA = 7,
    INVDESTALPHA = 8,
    DESTCOLOR = 9,
    INVDESTCOLOR = 10,
    SRCALPHASAT = 11,
    BOTHSRCALPHA = 12,
    BOTHINVSRCALPHA = 13,
    BLENDFACTOR = 14,
    INVBLENDFACTOR = 15,
}
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum D3DBLENDOP {
    ADD = 1,
    SUBTRACT = 2,
    REVSUBTRACT = 3,
    MIN = 4,
    MAX = 5,
}
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum D3DTEXTUREADDRESS {
    WRAP = 1,
    MIRROR = 2,
    CLAMP = 3,
    BORDER = 4,
    MIRRORONCE = 5,
}
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum D3DCULL {
    NONE = 1,
    CW = 2,
    CCW = 3,
}
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum D3DCMPFUNC {
    NEVER = 1,
    LESS = 2,
    EQUAL = 3,
    LESSEQUAL = 4,
    GREATER = 5,
    NOTEQUAL = 6,
    GREATEREQUAL = 7,
    ALWAYS = 8,
}
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum D3DSTENCILOP {
    KEEP = 1,
    ZERO = 2,
    REPLACE = 3,
    INCRSAT = 4,
    DECRSAT = 5,
    INVERT = 6,
    INCR = 7,
    DECR = 8,
}
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum D3DFOGMODE {
    NONE = 0,
    EXP = 1,
    EXP2 = 2,
    LINEAR = 3,
}
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum D3DZBUFFERTYPE {
    FALSE = 0,
    TRUE = 1,
    USEW = 2,
}
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum D3DPRIMITIVETYPE {
    POINTLIST = 1,
    LINELIST = 2,
    LINESTRIP = 3,
    TRIANGLELIST = 4,
    TRIANGLESTRIP = 5,
    TRIANGLEFAN = 6,
}
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum D3DTRANSFORMSTATETYPE {
    VIEW = 2,
    PROJECTION = 3,
    TEXTURE0 = 16,
    TEXTURE1 = 17,
    TEXTURE2 = 18,
    TEXTURE3 = 19,
    TEXTURE4 = 20,
    TEXTURE5 = 21,
    TEXTURE6 = 22,
    TEXTURE7 = 23,
}
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum D3DRENDERSTATETYPE {
    ZENABLE = 7,
    FILLMODE = 8,
    SHADEMODE = 9,
    ZWRITEENABLE = 14,
    ALPHATESTENABLE = 15,
    LASTPIXEL = 16,
    SRCBLEND = 19,
    DESTBLEND = 20,
    CULLMODE = 22,
    ZFUNC = 23,
    ALPHAREF = 24,
    ALPHAFUNC = 25,
    DITHERENABLE = 26,
    ALPHABLENDENABLE = 27,
    FOGENABLE = 28,
    SPECULARENABLE = 29,
    FOGCOLOR = 34,
    FOGTABLEMODE = 35,
    FOGSTART = 36,
    FOGEND = 37,
    FOGDENSITY = 38,
    RANGEFOGENABLE = 48,
    STENCILENABLE = 52,
    STENCILFAIL = 53,
    STENCILZFAIL = 54,
    STENCILPASS = 55,
    STENCILFUNC = 56,
    STENCILREF = 57,
    STENCILMASK = 58,
    STENCILWRITEMASK = 59,
    TEXTUREFACTOR = 60,
    WRAP0 = 128,
    WRAP1 = 129,
    WRAP2 = 130,
    WRAP3 = 131,
    WRAP4 = 132,
    WRAP5 = 133,
    WRAP6 = 134,
    WRAP7 = 135,
    CLIPPING = 136,
    LIGHTING = 137,
    AMBIENT = 139,
    FOGVERTEXMODE = 140,
    COLORVERTEX = 141,
    LOCALVIEWER = 142,
    NORMALIZENORMALS = 143,
    DIFFUSEMATERIALSOURCE = 145,
    SPECULARMATERIALSOURCE = 146,
    AMBIENTMATERIALSOURCE = 147,
    EMISSIVEMATERIALSOURCE = 148,
    VERTEXBLEND = 151,
    CLIPPLANEENABLE = 152,
    POINTSIZE = 154,
    POINTSIZE_MIN = 155,
    POINTSPRITEENABLE = 156,
    POINTSCALEENABLE = 157,
    POINTSCALE_A = 158,
    POINTSCALE_B = 159,
    POINTSCALE_C = 160,
    MULTISAMPLEANTIALIAS = 161,
    MULTISAMPLEMASK = 162,
    PATCHEDGESTYLE = 163,
    DEBUGMONITORTOKEN = 165,
    POINTSIZE_MAX = 166,
    INDEXEDVERTEXBLENDENABLE = 167,
    COLORWRITEENABLE = 168,
    TWEENFACTOR = 170,
    BLENDOP = 171,
    POSITIONDEGREE = 172,
    NORMALDEGREE = 173,
    SCISSORTESTENABLE = 174,
    SLOPESCALEDEPTHBIAS = 175,
    ANTIALIASEDLINEENABLE = 176,
    MINTESSELLATIONLEVEL = 178,
    MAXTESSELLATIONLEVEL = 179,
    ADAPTIVETESS_X = 180,
    ADAPTIVETESS_Y = 181,
    ADAPTIVETESS_Z = 182,
    ADAPTIVETESS_W = 183,
    ENABLEADAPTIVETESSELLATION = 184,
    TWOSIDEDSTENCILMODE = 185,
    CCW_STENCILFAIL = 186,
    CCW_STENCILZFAIL = 187,
    CCW_STENCILPASS = 188,
    CCW_STENCILFUNC = 189,
    COLORWRITEENABLE1 = 190,
    COLORWRITEENABLE2 = 191,
    COLORWRITEENABLE3 = 192,
    BLENDFACTOR = 193,
    SRGBWRITEENABLE = 194,
    DEPTHBIAS = 195,
    WRAP8 = 198,
    WRAP9 = 199,
    WRAP10 = 200,
    WRAP11 = 201,
    WRAP12 = 202,
    WRAP13 = 203,
    WRAP14 = 204,
    WRAP15 = 205,
    SEPARATEALPHABLENDENABLE = 206,
    SRCBLENDALPHA = 207,
    DESTBLENDALPHA = 208,
    BLENDOPALPHA = 209,
}
pub const D3D_MAX_SIMULTANEOUS_RENDERTARGETS: ::DWORD = 4;
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum D3DMATERIALCOLORSOURCE {
    MATERIAL = 0,
    COLOR1 = 1,
    COLOR2 = 2,
}
pub const D3DRENDERSTATE_WRAPBIAS: ::DWORD = 128;
pub const D3DWRAP_U: ::DWORD = 0x00000001;
pub const D3DWRAP_V: ::DWORD = 0x00000002;
pub const D3DWRAP_W: ::DWORD = 0x00000004;
pub const D3DWRAPCOORD_0: ::DWORD = 0x00000001;
pub const D3DWRAPCOORD_1: ::DWORD = 0x00000002;
pub const D3DWRAPCOORD_2: ::DWORD = 0x00000004;
pub const D3DWRAPCOORD_3: ::DWORD = 0x00000008;
pub const D3DCOLORWRITEENABLE_RED: ::DWORD = 1 << 0;
pub const D3DCOLORWRITEENABLE_GREEN: ::DWORD = 1 << 1;
pub const D3DCOLORWRITEENABLE_BLUE: ::DWORD = 1 << 2;
pub const D3DCOLORWRITEENABLE_ALPHA: ::DWORD = 1 << 3;
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum D3DTEXTURESTAGESTATETYPE {
    COLOROP = 1,
    COLORARG1 = 2,
    COLORARG2 = 3,
    ALPHAOP = 4,
    ALPHAARG1 = 5,
    ALPHAARG2 = 6,
    BUMPENVMAT00 = 7,
    BUMPENVMAT01 = 8,
    BUMPENVMAT10 = 9,
    BUMPENVMAT11 = 10,
    TEXCOORDINDEX = 11,
    BUMPENVLSCALE = 22,
    BUMPENVLOFFSET = 23,
    TEXTURETRANSFORMFLAGS = 24,
    COLORARG0 = 26,
    ALPHAARG0 = 27,
    RESULTARG = 28,
    CONSTANT = 32,
}
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum D3DSAMPLERSTATETYPE {
    ADDRESSU = 1,
    ADDRESSV = 2,
    ADDRESSW = 3,
    BORDERCOLOR = 4,
    MAGFILTER = 5,
    MINFILTER = 6,
    MIPFILTER = 7,
    MIPMAPLODBIAS = 8,
    MAXMIPLEVEL = 9,
    MAXANISOTROPY = 10,
    SRGBTEXTURE = 11,
    ELEMENTINDEX = 12,
    DMAPOFFSET = 13,
}
pub const D3DDMAPSAMPLER: ::DWORD = 256;
pub const D3DVERTEXTEXTURESAMPLER0: ::DWORD = D3DDMAPSAMPLER + 1;
pub const D3DVERTEXTEXTURESAMPLER1: ::DWORD = D3DDMAPSAMPLER + 2;
pub const D3DVERTEXTEXTURESAMPLER2: ::DWORD = D3DDMAPSAMPLER + 3;
pub const D3DVERTEXTEXTURESAMPLER3: ::DWORD = D3DDMAPSAMPLER + 4;
pub const D3DTSS_TCI_PASSTHRU: ::DWORD = 0x00000000;
pub const D3DTSS_TCI_CAMERASPACENORMAL: ::DWORD = 0x00010000;
pub const D3DTSS_TCI_CAMERASPACEPOSITION: ::DWORD = 0x00020000;
pub const D3DTSS_TCI_CAMERASPACEREFLECTIONVECTOR: ::DWORD = 0x00030000;
pub const D3DTSS_TCI_SPHEREMAP: ::DWORD = 0x00040000;
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum D3DTEXTUREOP {
    DISABLE = 1,
    SELECTARG1 = 2,
    SELECTARG2 = 3,
    MODULATE = 4,
    MODULATE2X = 5,
    MODULATE4X = 6,
    ADD = 7,
    ADDSIGNED = 8,
    ADDSIGNED2X = 9,
    SUBTRACT = 10,
    ADDSMOOTH = 11,
    BLENDDIFFUSEALPHA = 12,
    BLENDTEXTUREALPHA = 13,
    BLENDFACTORALPHA = 14,
    BLENDTEXTUREALPHAPM = 15,
    BLENDCURRENTALPHA = 16,
    PREMODULATE = 17,
    MODULATEALPHA_ADDCOLOR = 18,
    MODULATECOLOR_ADDALPHA = 19,
    MODULATEINVALPHA_ADDCOLOR = 20,
    MODULATEINVCOLOR_ADDALPHA = 21,
    BUMPENVMAP = 22,
    BUMPENVMAPLUMINANCE = 23,
    DOTPRODUCT3 = 24,
    MULTIPLYADD = 25,
    LERP = 26,
}
pub const D3DTA_SELECTMASK: ::DWORD = 0x0000000f;
pub const D3DTA_DIFFUSE: ::DWORD = 0x00000000;
pub const D3DTA_CURRENT: ::DWORD = 0x00000001;
pub const D3DTA_TEXTURE: ::DWORD = 0x00000002;
pub const D3DTA_TFACTOR: ::DWORD = 0x00000003;
pub const D3DTA_SPECULAR: ::DWORD = 0x00000004;
pub const D3DTA_TEMP: ::DWORD = 0x00000005;
pub const D3DTA_CONSTANT: ::DWORD = 0x00000006;
pub const D3DTA_COMPLEMENT: ::DWORD = 0x00000010;
pub const D3DTA_ALPHAREPLICATE: ::DWORD = 0x00000020;
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum D3DTEXTUREFILTERTYPE {
    NONE = 0,
    POINT = 1,
    LINEAR = 2,
    ANISOTROPIC = 3,
    PYRAMIDALQUAD = 6,
    GAUSSIANQUAD = 7,
    CONVOLUTIONMONO = 8,
}
pub const D3DPV_DONOTCOPYDATA: ::DWORD = 1 << 0;
pub const D3DFVF_RESERVED0: ::DWORD = 0x001;
pub const D3DFVF_POSITION_MASK: ::DWORD = 0x400E;
pub const D3DFVF_XYZ: ::DWORD = 0x002;
pub const D3DFVF_XYZRHW: ::DWORD = 0x004;
pub const D3DFVF_XYZB1: ::DWORD = 0x006;
pub const D3DFVF_XYZB2: ::DWORD = 0x008;
pub const D3DFVF_XYZB3: ::DWORD = 0x00a;
pub const D3DFVF_XYZB4: ::DWORD = 0x00c;
pub const D3DFVF_XYZB5: ::DWORD = 0x00e;
pub const D3DFVF_XYZW: ::DWORD = 0x4002;
pub const D3DFVF_NORMAL: ::DWORD = 0x010;
pub const D3DFVF_PSIZE: ::DWORD = 0x020;
pub const D3DFVF_DIFFUSE: ::DWORD = 0x040;
pub const D3DFVF_SPECULAR: ::DWORD = 0x080;
pub const D3DFVF_TEXCOUNT_MASK: ::DWORD = 0xf00;
pub const D3DFVF_TEXCOUNT_SHIFT: ::DWORD = 8;
pub const D3DFVF_TEX0: ::DWORD = 0x000;
pub const D3DFVF_TEX1: ::DWORD = 0x100;
pub const D3DFVF_TEX2: ::DWORD = 0x200;
pub const D3DFVF_TEX3: ::DWORD = 0x300;
pub const D3DFVF_TEX4: ::DWORD = 0x400;
pub const D3DFVF_TEX5: ::DWORD = 0x500;
pub const D3DFVF_TEX6: ::DWORD = 0x600;
pub const D3DFVF_TEX7: ::DWORD = 0x700;
pub const D3DFVF_TEX8: ::DWORD = 0x800;
pub const D3DFVF_LASTBETA_UBYTE4: ::DWORD = 0x1000;
pub const D3DFVF_LASTBETA_D3DCOLOR: ::DWORD = 0x8000;
pub const D3DFVF_RESERVED2: ::DWORD = 0x6000;
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum D3DDECLUSAGE {
    POSITION = 0,
    BLENDWEIGHT,
    BLENDINDICES,
    NORMAL,
    PSIZE,
    TEXCOORD,
    TANGENT,
    BINORMAL,
    TESSFACTOR,
    POSITIONT,
    COLOR,
    FOG,
    DEPTH,
    SAMPLE,
}
pub const MAXD3DDECLUSAGE: D3DDECLUSAGE = D3DDECLUSAGE::SAMPLE;
pub const MAXD3DDECLUSAGEINDEX: ::DWORD = 15;
pub const MAXD3DDECLLENGTH: ::DWORD = 64;
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum D3DDECLMETHOD {
    DEFAULT = 0,
    PARTIALU,
    PARTIALV,
    CROSSUV,
    UV,
    LOOKUP,
    LOOKUPPRESAMPLED,
}
pub const MAXD3DDECLMETHOD: D3DDECLMETHOD = D3DDECLMETHOD::LOOKUPPRESAMPLED;
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum D3DDECLTYPE {
    FLOAT1 = 0,
    FLOAT2 = 1,
    FLOAT3 = 2,
    FLOAT4 = 3,
    D3DCOLOR = 4,
    UBYTE4 = 5,
    SHORT2 = 6,
    SHORT4 = 7,
    UBYTE4N = 8,
    SHORT2N = 9,
    SHORT4N = 10,
    USHORT2N = 11,
    USHORT4N = 12,
    UDEC3 = 13,
    DEC3N = 14,
    FLOAT16_2 = 15,
    FLOAT16_4 = 16,
    UNUSED = 17,
}
pub const MAXD3DDECLTYPE: D3DDECLTYPE = D3DDECLTYPE::UNUSED;
STRUCT!{struct D3DVERTEXELEMENT9 {
    Stream: ::WORD,
    Offset: ::WORD,
    Type: ::BYTE,
    Method: ::BYTE,
    Usage: ::BYTE,
    UsageIndex: ::BYTE,
}}
pub type LPD3DVERTEXELEMENT9 = *mut D3DVERTEXELEMENT9;
pub const D3DDECL_END: D3DVERTEXELEMENT9 = D3DVERTEXELEMENT9 {
    Stream: 0xFF,
    Offset: 0,
    Type: D3DDECLTYPE::UNUSED as ::BYTE,
    Method: 0,
    Usage: 0,
    UsageIndex: 0,
};
pub const D3DDP_MAXTEXCOORD: ::DWORD = 8;
pub const D3DSTREAMSOURCE_INDEXEDDATA: ::DWORD = 1 << 30;
pub const D3DSTREAMSOURCE_INSTANCEDATA: ::DWORD = 2 << 30;
pub const D3DSI_OPCODE_MASK: ::DWORD = 0x0000FFFF;
pub const D3DSI_INSTLENGTH_MASK: ::DWORD = 0x0F000000;
pub const D3DSI_INSTLENGTH_SHIFT: ::DWORD = 24;
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum D3DSHADER_INSTRUCTION_OPCODE_TYPE {
    NOP = 0,
    MOV,
    ADD,
    SUB,
    MAD,
    MUL,
    RCP,
    RSQ,
    DP3,
    DP4,
    MIN,
    MAX,
    SLT,
    SGE,
    EXP,
    LOG,
    LIT,
    DST,
    LRP,
    FRC,
    M4x4,
    M4x3,
    M3x4,
    M3x3,
    M3x2,
    CALL,
    CALLNZ,
    LOOP,
    RET,
    ENDLOOP,
    LABEL,
    DCL,
    POW,
    CRS,
    SGN,
    ABS,
    NRM,
    SINCOS,
    REP,
    ENDREP,
    IF,
    IFC,
    ELSE,
    ENDIF,
    BREAK,
    BREAKC,
    MOVA,
    DEFB,
    DEFI,
    TEXCOORD = 64,
    TEXKILL,
    TEX,
    TEXBEM,
    TEXBEML,
    TEXREG2AR,
    TEXREG2GB,
    TEXM3x2PAD,
    TEXM3x2TEX,
    TEXM3x3PAD,
    TEXM3x3TEX,
    RESERVED0,
    TEXM3x3SPEC,
    TEXM3x3VSPEC,
    EXPP,
    LOGP,
    CND,
    DEF,
    TEXREG2RGB,
    TEXDP3TEX,
    TEXM3x2DEPTH,
    TEXDP3,
    TEXM3x3,
    TEXDEPTH,
    CMP,
    BEM,
    DP2ADD,
    DSX,
    DSY,
    TEXLDD,
    SETP,
    TEXLDL,
    BREAKP,
    PHASE = 0xFFFD,
    COMMENT = 0xFFFE,
    END = 0xFFFF,
}
pub const D3DSI_COISSUE: ::DWORD = 0x40000000;
pub const D3DSP_OPCODESPECIFICCONTROL_MASK: ::DWORD = 0x00ff0000;
pub const D3DSP_OPCODESPECIFICCONTROL_SHIFT: ::DWORD = 16;
pub const D3DSI_TEXLD_PROJECT: ::DWORD = 0x01 << D3DSP_OPCODESPECIFICCONTROL_SHIFT;
pub const D3DSI_TEXLD_BIAS: ::DWORD = 0x02 << D3DSP_OPCODESPECIFICCONTROL_SHIFT;
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum D3DSHADER_COMPARISON {
    RESERVED0 = 0,
    GT = 1,
    EQ = 2,
    GE = 3,
    LT = 4,
    NE = 5,
    LE = 6,
    RESERVED1 = 7,
}
pub const D3DSHADER_COMPARISON_SHIFT: ::DWORD = D3DSP_OPCODESPECIFICCONTROL_SHIFT;
pub const D3DSHADER_COMPARISON_MASK: ::DWORD = 0x7 << D3DSHADER_COMPARISON_SHIFT;
pub const D3DSHADER_INSTRUCTION_PREDICATED: ::DWORD = 0x1 << 28;
pub const D3DSP_DCL_USAGE_SHIFT: ::DWORD = 0;
pub const D3DSP_DCL_USAGE_MASK: ::DWORD = 0x0000000f;
pub const D3DSP_DCL_USAGEINDEX_SHIFT: ::DWORD = 16;
pub const D3DSP_DCL_USAGEINDEX_MASK: ::DWORD = 0x000f0000;
pub const D3DSP_TEXTURETYPE_SHIFT: ::DWORD = 27;
pub const D3DSP_TEXTURETYPE_MASK: ::DWORD = 0x78000000;
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum D3DSAMPLER_TEXTURE_TYPE {
    UNKNOWN = 0 << D3DSP_TEXTURETYPE_SHIFT,
    _2D = 2 << D3DSP_TEXTURETYPE_SHIFT,
    CUBE = 3 << D3DSP_TEXTURETYPE_SHIFT,
    VOLUME = 4 << D3DSP_TEXTURETYPE_SHIFT,
}
pub const D3DSP_REGNUM_MASK: ::DWORD = 0x000007FF;
pub const D3DSP_WRITEMASK_0: ::DWORD = 0x00010000;
pub const D3DSP_WRITEMASK_1: ::DWORD = 0x00020000;
pub const D3DSP_WRITEMASK_2: ::DWORD = 0x00040000;
pub const D3DSP_WRITEMASK_3: ::DWORD = 0x00080000;
pub const D3DSP_WRITEMASK_ALL: ::DWORD = 0x000F0000;
pub const D3DSP_DSTMOD_SHIFT: ::DWORD = 20;
pub const D3DSP_DSTMOD_MASK: ::DWORD = 0x00F00000;
pub const D3DSPDM_NONE: ::DWORD = 0 << D3DSP_DSTMOD_SHIFT;
pub const D3DSPDM_SATURATE: ::DWORD = 1 << D3DSP_DSTMOD_SHIFT;
pub const D3DSPDM_PARTIALPRECISION: ::DWORD = 2 << D3DSP_DSTMOD_SHIFT;
pub const D3DSPDM_MSAMPCENTROID: ::DWORD = 4 << D3DSP_DSTMOD_SHIFT;
pub const D3DSP_DSTSHIFT_SHIFT: ::DWORD = 24;
pub const D3DSP_DSTSHIFT_MASK: ::DWORD = 0x0F000000;
pub const D3DSP_REGTYPE_SHIFT: ::DWORD = 28;
pub const D3DSP_REGTYPE_SHIFT2: ::DWORD = 8;
pub const D3DSP_REGTYPE_MASK: ::DWORD = 0x70000000;
pub const D3DSP_REGTYPE_MASK2: ::DWORD = 0x00001800;
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum D3DSHADER_PARAM_REGISTER_TYPE {
    TEMP = 0,
    INPUT = 1,
    CONST = 2,
    ADDR = 3,
    // D3DSPR_TEXTURE = 3, // Why Rust?
    RASTOUT = 4,
    ATTROUT = 5,
    TEXCRDOUT = 6,
    // D3DSPR_OUTPUT = 6, // Why are you doing this to me?
    CONSTINT = 7,
    COLOROUT = 8,
    DEPTHOUT = 9,
    SAMPLER = 10,
    CONST2 = 11,
    CONST3 = 12,
    CONST4 = 13,
    CONSTBOOL = 14,
    LOOP = 15,
    TEMPFLOAT16 = 16,
    MISCTYPE = 17,
    LABEL = 18,
    PREDICATE = 19,
}
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum D3DSHADER_MISCTYPE_OFFSETS {
    POSITION = 0,
    FACE = 1,
}
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum D3DVS_RASTOUT_OFFSETS {
    POSITION = 0,
    FOG,
    POINT_SIZE,
}
pub const D3DVS_ADDRESSMODE_SHIFT: ::DWORD = 13;
pub const D3DVS_ADDRESSMODE_MASK: ::DWORD = 1 << D3DVS_ADDRESSMODE_SHIFT;
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum D3DVS_ADDRESSMODE_TYPE {
    ADDRMODE_ABSOLUTE = 0 << D3DVS_ADDRESSMODE_SHIFT,
    ADDRMODE_RELATIVE = 1 << D3DVS_ADDRESSMODE_SHIFT,
}
pub const D3DSHADER_ADDRESSMODE_SHIFT: ::DWORD = 13;
pub const D3DSHADER_ADDRESSMODE_MASK: ::DWORD = 1 << D3DSHADER_ADDRESSMODE_SHIFT;
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum D3DSHADER_ADDRESSMODE_TYPE {
    ABSOLUTE = 0 << D3DSHADER_ADDRESSMODE_SHIFT,
    RELATIVE = 1 << D3DSHADER_ADDRESSMODE_SHIFT,
}
pub const D3DVS_SWIZZLE_SHIFT: ::DWORD = 16;
pub const D3DVS_SWIZZLE_MASK: ::DWORD = 0x00FF0000;
pub const D3DVS_X_X: ::DWORD = 0 << D3DVS_SWIZZLE_SHIFT;
pub const D3DVS_X_Y: ::DWORD = 1 << D3DVS_SWIZZLE_SHIFT;
pub const D3DVS_X_Z: ::DWORD = 2 << D3DVS_SWIZZLE_SHIFT;
pub const D3DVS_X_W: ::DWORD = 3 << D3DVS_SWIZZLE_SHIFT;
pub const D3DVS_Y_X: ::DWORD = 0 << (D3DVS_SWIZZLE_SHIFT + 2);
pub const D3DVS_Y_Y: ::DWORD = 1 << (D3DVS_SWIZZLE_SHIFT + 2);
pub const D3DVS_Y_Z: ::DWORD = 2 << (D3DVS_SWIZZLE_SHIFT + 2);
pub const D3DVS_Y_W: ::DWORD = 3 << (D3DVS_SWIZZLE_SHIFT + 2);
pub const D3DVS_Z_X: ::DWORD = 0 << (D3DVS_SWIZZLE_SHIFT + 4);
pub const D3DVS_Z_Y: ::DWORD = 1 << (D3DVS_SWIZZLE_SHIFT + 4);
pub const D3DVS_Z_Z: ::DWORD = 2 << (D3DVS_SWIZZLE_SHIFT + 4);
pub const D3DVS_Z_W: ::DWORD = 3 << (D3DVS_SWIZZLE_SHIFT + 4);
pub const D3DVS_W_X: ::DWORD = 0 << (D3DVS_SWIZZLE_SHIFT + 6);
pub const D3DVS_W_Y: ::DWORD = 1 << (D3DVS_SWIZZLE_SHIFT + 6);
pub const D3DVS_W_Z: ::DWORD = 2 << (D3DVS_SWIZZLE_SHIFT + 6);
pub const D3DVS_W_W: ::DWORD = 3 << (D3DVS_SWIZZLE_SHIFT + 6);
pub const D3DVS_NOSWIZZLE: ::DWORD = D3DVS_X_X | D3DVS_Y_Y | D3DVS_Z_Z | D3DVS_W_W;
pub const D3DSP_SWIZZLE_SHIFT: ::DWORD = 16;
pub const D3DSP_SWIZZLE_MASK: ::DWORD = 0x00FF0000;
pub const D3DSP_NOSWIZZLE: ::DWORD = (0 << (D3DSP_SWIZZLE_SHIFT + 0))
    | (1 << (D3DSP_SWIZZLE_SHIFT + 2)) | (2 << (D3DSP_SWIZZLE_SHIFT + 4))
    | (3 << (D3DSP_SWIZZLE_SHIFT + 6));
pub const D3DSP_REPLICATERED: ::DWORD = (0 << (D3DSP_SWIZZLE_SHIFT + 0))
    | (0 << (D3DSP_SWIZZLE_SHIFT + 2)) | (0 << (D3DSP_SWIZZLE_SHIFT + 4))
    | (0 << (D3DSP_SWIZZLE_SHIFT + 6));
pub const D3DSP_REPLICATEGREEN: ::DWORD = (1 << (D3DSP_SWIZZLE_SHIFT + 0))
    | (1 << (D3DSP_SWIZZLE_SHIFT + 2)) | (1 << (D3DSP_SWIZZLE_SHIFT + 4))
    | (1 << (D3DSP_SWIZZLE_SHIFT + 6));
pub const D3DSP_REPLICATEBLUE: ::DWORD = (2 << (D3DSP_SWIZZLE_SHIFT + 0))
    | (2 << (D3DSP_SWIZZLE_SHIFT + 2)) | (2 << (D3DSP_SWIZZLE_SHIFT + 4))
    | (2 << (D3DSP_SWIZZLE_SHIFT + 6));
pub const D3DSP_REPLICATEALPHA: ::DWORD = (3 << (D3DSP_SWIZZLE_SHIFT + 0))
    | (3 << (D3DSP_SWIZZLE_SHIFT + 2)) | (3 << (D3DSP_SWIZZLE_SHIFT + 4))
    | (3 << (D3DSP_SWIZZLE_SHIFT + 6));
pub const D3DSP_SRCMOD_SHIFT: ::DWORD = 24;
pub const D3DSP_SRCMOD_MASK: ::DWORD = 0x0F000000;
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum D3DSHADER_PARAM_SRCMOD_TYPE {
    NONE = 0 << D3DSP_SRCMOD_SHIFT,
    NEG = 1 << D3DSP_SRCMOD_SHIFT,
    BIAS = 2 << D3DSP_SRCMOD_SHIFT,
    BIASNEG = 3 << D3DSP_SRCMOD_SHIFT,
    SIGN = 4 << D3DSP_SRCMOD_SHIFT,
    SIGNNEG = 5 << D3DSP_SRCMOD_SHIFT,
    COMP = 6 << D3DSP_SRCMOD_SHIFT,
    X2 = 7 << D3DSP_SRCMOD_SHIFT,
    X2NEG = 8 << D3DSP_SRCMOD_SHIFT,
    DZ = 9 << D3DSP_SRCMOD_SHIFT,
    DW = 10 << D3DSP_SRCMOD_SHIFT,
    ABS = 11 << D3DSP_SRCMOD_SHIFT,
    ABSNEG = 12 << D3DSP_SRCMOD_SHIFT,
    NOT = 13 << D3DSP_SRCMOD_SHIFT,
}
pub const D3DSP_MIN_PRECISION_SHIFT: ::DWORD = 14;
pub const D3DSP_MIN_PRECISION_MASK: ::DWORD = 0x0000C000;
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum D3DSHADER_MIN_PRECISION {
    DEFAULT = 0,
    _16 = 1,
    _2_8 = 2,
}
pub const D3DSI_COMMENTSIZE_SHIFT: ::DWORD = 16;
pub const D3DSI_COMMENTSIZE_MASK: ::DWORD = 0x7FFF0000;
pub const D3DPS_END: ::DWORD = 0x0000FFFF;
pub const D3DVS_END: ::DWORD = 0x0000FFFF;
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum D3DBASISTYPE {
    BEZIER = 0,
    BSPLINE = 1,
    CATMULL_ROM = 2,
}
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum D3DDEGREETYPE {
    LINEAR = 1,
    QUADRATIC = 2,
    CUBIC = 3,
    QUINTIC = 5,
}
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum D3DPATCHEDGESTYLE {
    DISCRETE = 0,
    CONTINUOUS = 1,
}
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum D3DSTATEBLOCKTYPE {
    ALL = 1,
    PIXELSTATE = 2,
    VERTEXSTATE = 3,
}
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum D3DVERTEXBLENDFLAGS {
    DISABLE = 0,
    _1WEIGHTS = 1,
    _2WEIGHTS = 2,
    _3WEIGHTS = 3,
    _TWEENING = 255,
    _0WEIGHTS = 256,
}
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum D3DTEXTURETRANSFORMFLAGS {
    DISABLE = 0,
    COUNT1 = 1,
    COUNT2 = 2,
    COUNT3 = 3,
    COUNT4 = 4,
    PROJECTED = 256,
}
pub const D3DFVF_TEXTUREFORMAT2: ::DWORD = 0;
pub const D3DFVF_TEXTUREFORMAT1: ::DWORD = 3;
pub const D3DFVF_TEXTUREFORMAT3: ::DWORD = 1;
pub const D3DFVF_TEXTUREFORMAT4: ::DWORD = 2;
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum D3DDEVTYPE {
    HAL = 1,
    REF = 2,
    SW = 3,
    NULLREF = 4,
}
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum D3DMULTISAMPLE_TYPE {
    NONE = 0,
    NONMASKABLE = 1,
    _2_SAMPLES = 2,
    _3_SAMPLES = 3,
    _4_SAMPLES = 4,
    _5_SAMPLES = 5,
    _6_SAMPLES = 6,
    _7_SAMPLES = 7,
    _8_SAMPLES = 8,
    _9_SAMPLES = 9,
    _10_SAMPLES = 10,
    _11_SAMPLES = 11,
    _12_SAMPLES = 12,
    _13_SAMPLES = 13,
    _14_SAMPLES = 14,
    _15_SAMPLES = 15,
    _16_SAMPLES = 16,
}
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum D3DFORMAT {
    UNKNOWN = 0,
    R8G8B8 = 20,
    A8R8G8B8 = 21,
    X8R8G8B8 = 22,
    R5G6B5 = 23,
    X1R5G5B5 = 24,
    A1R5G5B5 = 25,
    A4R4G4B4 = 26,
    R3G3B2 = 27,
    A8 = 28,
    A8R3G3B2 = 29,
    X4R4G4B4 = 30,
    A2B10G10R10 = 31,
    A8B8G8R8 = 32,
    X8B8G8R8 = 33,
    G16R16 = 34,
    A2R10G10B10 = 35,
    A16B16G16R16 = 36,
    A8P8 = 40,
    P8 = 41,
    L8 = 50,
    A8L8 = 51,
    A4L4 = 52,
    V8U8 = 60,
    L6V5U5 = 61,
    X8L8V8U8 = 62,
    Q8W8V8U8 = 63,
    V16U16 = 64,
    A2W10V10U10 = 67,
    UYVY = MAKEFOURCC!(b'U', b'Y', b'V', b'Y'),
    R8G8_B8G8 = MAKEFOURCC!(b'R', b'G', b'B', b'G'),
    YUY2 = MAKEFOURCC!(b'Y', b'U', b'Y', b'2'),
    G8R8_G8B8 = MAKEFOURCC!(b'G', b'R', b'G', b'B'),
    DXT1 = MAKEFOURCC!(b'D', b'X', b'T', b'1'),
    DXT2 = MAKEFOURCC!(b'D', b'X', b'T', b'2'),
    DXT3 = MAKEFOURCC!(b'D', b'X', b'T', b'3'),
    DXT4 = MAKEFOURCC!(b'D', b'X', b'T', b'4'),
    DXT5 = MAKEFOURCC!(b'D', b'X', b'T', b'5'),
    D16_LOCKABLE = 70,
    D32 = 71,
    D15S1 = 73,
    D24S8 = 75,
    D24X8 = 77,
    D24X4S4 = 79,
    D16 = 80,
    D32F_LOCKABLE = 82,
    D24FS8 = 83,
    D32_LOCKABLE = 84,
    S8_LOCKABLE = 85,
    L16 = 81,
    VERTEXDATA = 100,
    INDEX16 = 101,
    INDEX32 = 102,
    Q16W16V16U16 = 110,
    MULTI2_ARGB8 = MAKEFOURCC!(b'M', b'E', b'T', b'1'),
    R16F = 111,
    G16R16F = 112,
    A16B16G16R16F = 113,
    R32F = 114,
    G32R32F = 115,
    A32B32G32R32F = 116,
    CxV8U8 = 117,
    A1 = 118,
    A2B10G10R10_XR_BIAS = 119,
    BINARYBUFFER = 199,
}
STRUCT!{struct D3DDISPLAYMODE {
    Width: ::UINT,
    Height: ::UINT,
    RefreshRate: ::UINT,
    Format: D3DFORMAT,
}}
STRUCT!{struct D3DDEVICE_CREATION_PARAMETERS {
    AdapterOrdinal: ::UINT,
    DeviceType: D3DDEVTYPE,
    hFocusWindow: ::HWND,
    BehaviorFlags: ::DWORD,
}}
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum D3DSWAPEFFECT {
    DISCARD = 1,
    FLIP = 2,
    COPY = 3,
    OVERLAY = 4,
    FLIPEX = 5,
}
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum D3DPOOL {
    DEFAULT = 0,
    MANAGED = 1,
    SYSTEMMEM = 2,
    SCRATCH = 3,
}
pub const D3DPRESENT_RATE_DEFAULT: ::DWORD = 0x00000000;
STRUCT!{struct D3DPRESENT_PARAMETERS {
    BackBufferWidth: ::UINT,
    BackBufferHeight: ::UINT,
    BackBufferFormat: D3DFORMAT,
    BackBufferCount: ::UINT,
    MultiSampleType: D3DMULTISAMPLE_TYPE,
    MultiSampleQuality: ::DWORD,
    SwapEffect: D3DSWAPEFFECT,
    hDeviceWindow: ::HWND,
    Windowed: ::BOOL,
    EnableAutoDepthStencil: ::BOOL,
    AutoDepthStencilFormat: D3DFORMAT,
    Flags: ::DWORD,
    FullScreen_RefreshRateInHz: ::UINT,
    PresentationInterval: ::UINT,
}}
pub const D3DPRESENTFLAG_LOCKABLE_BACKBUFFER: ::DWORD = 0x00000001;
pub const D3DPRESENTFLAG_DISCARD_DEPTHSTENCIL: ::DWORD = 0x00000002;
pub const D3DPRESENTFLAG_DEVICECLIP: ::DWORD = 0x00000004;
pub const D3DPRESENTFLAG_VIDEO: ::DWORD = 0x00000010;
pub const D3DPRESENTFLAG_NOAUTOROTATE: ::DWORD = 0x00000020;
pub const D3DPRESENTFLAG_UNPRUNEDMODE: ::DWORD = 0x00000040;
pub const D3DPRESENTFLAG_OVERLAY_LIMITEDRGB: ::DWORD = 0x00000080;
pub const D3DPRESENTFLAG_OVERLAY_YCbCr_BT709: ::DWORD = 0x00000100;
pub const D3DPRESENTFLAG_OVERLAY_YCbCr_xvYCC: ::DWORD = 0x00000200;
pub const D3DPRESENTFLAG_RESTRICTED_CONTENT: ::DWORD = 0x00000400;
pub const D3DPRESENTFLAG_RESTRICT_SHARED_RESOURCE_DRIVER: ::DWORD = 0x00000800;
#[repr(C)] #[derive(Copy)]
pub struct D3DGAMMARAMP {
    pub red: [::WORD; 256],
    pub green: [::WORD; 256],
    pub blue: [::WORD; 256],
}
impl Clone for D3DGAMMARAMP { fn clone(&self) -> D3DGAMMARAMP { *self } }
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum D3DBACKBUFFER_TYPE {
    TYPE_MONO = 0,
    TYPE_LEFT = 1,
    TYPE_RIGHT = 2,
}
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum D3DRESOURCETYPE {
    SURFACE = 1,
    VOLUME = 2,
    TEXTURE = 3,
    VOLUMETEXTURE = 4,
    CUBETEXTURE = 5,
    VERTEXBUFFER = 6,
    INDEXBUFFER = 7,
}
pub const D3DUSAGE_RENDERTARGET: ::DWORD = 0x00000001;
pub const D3DUSAGE_DEPTHSTENCIL: ::DWORD = 0x00000002;
pub const D3DUSAGE_DYNAMIC: ::DWORD = 0x00000200;
pub const D3DUSAGE_NONSECURE: ::DWORD = 0x00800000;
pub const D3DUSAGE_AUTOGENMIPMAP: ::DWORD = 0x00000400;
pub const D3DUSAGE_DMAP: ::DWORD = 0x00004000;
pub const D3DUSAGE_QUERY_LEGACYBUMPMAP: ::DWORD = 0x00008000;
pub const D3DUSAGE_QUERY_SRGBREAD: ::DWORD = 0x00010000;
pub const D3DUSAGE_QUERY_FILTER: ::DWORD = 0x00020000;
pub const D3DUSAGE_QUERY_SRGBWRITE: ::DWORD = 0x00040000;
pub const D3DUSAGE_QUERY_POSTPIXELSHADER_BLENDING: ::DWORD = 0x00080000;
pub const D3DUSAGE_QUERY_VERTEXTEXTURE: ::DWORD = 0x00100000;
pub const D3DUSAGE_QUERY_WRAPANDMIP: ::DWORD = 0x00200000;
pub const D3DUSAGE_WRITEONLY: ::DWORD = 0x00000008;
pub const D3DUSAGE_SOFTWAREPROCESSING: ::DWORD = 0x00000010;
pub const D3DUSAGE_DONOTCLIP: ::DWORD = 0x00000020;
pub const D3DUSAGE_POINTS: ::DWORD = 0x00000040;
pub const D3DUSAGE_RTPATCHES: ::DWORD = 0x00000080;
pub const D3DUSAGE_NPATCHES: ::DWORD = 0x00000100;
pub const D3DUSAGE_TEXTAPI: ::DWORD = 0x10000000;
pub const D3DUSAGE_RESTRICTED_CONTENT: ::DWORD = 0x00000800;
pub const D3DUSAGE_RESTRICT_SHARED_RESOURCE: ::DWORD = 0x00002000;
pub const D3DUSAGE_RESTRICT_SHARED_RESOURCE_DRIVER: ::DWORD = 0x00001000;
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum D3DCUBEMAP_FACES {
    POSITIVE_X = 0,
    NEGATIVE_X = 1,
    POSITIVE_Y = 2,
    NEGATIVE_Y = 3,
    POSITIVE_Z = 4,
    NEGATIVE_Z = 5,
}
pub const D3DLOCK_READONLY: ::DWORD = 0x00000010;
pub const D3DLOCK_DISCARD: ::DWORD = 0x00002000;
pub const D3DLOCK_NOOVERWRITE: ::DWORD = 0x00001000;
pub const D3DLOCK_NOSYSLOCK: ::DWORD = 0x00000800;
pub const D3DLOCK_DONOTWAIT: ::DWORD = 0x00004000;
pub const D3DLOCK_NO_DIRTY_UPDATE: ::DWORD = 0x00008000;
STRUCT!{struct D3DVERTEXBUFFER_DESC {
    Format: D3DFORMAT,
    Type: D3DRESOURCETYPE,
    Usage: ::DWORD,
    Pool: D3DPOOL,
    Size: ::UINT,
    FVF: ::DWORD,
}}
STRUCT!{struct D3DINDEXBUFFER_DESC {
    Format: D3DFORMAT,
    Type: D3DRESOURCETYPE,
    Usage: ::DWORD,
    Pool: D3DPOOL,
    Size: ::UINT,
}}
STRUCT!{struct D3DSURFACE_DESC {
    Format: D3DFORMAT,
    Type: D3DRESOURCETYPE,
    Usage: ::DWORD,
    Pool: D3DPOOL,
    MultiSampleType: D3DMULTISAMPLE_TYPE,
    MultiSampleQuality: ::DWORD,
    Width: ::UINT,
    Height: ::UINT,
}}
STRUCT!{struct D3DVOLUME_DESC {
    Format: D3DFORMAT,
    Type: D3DRESOURCETYPE,
    Usage: ::DWORD,
    Pool: D3DPOOL,
    Width: ::UINT,
    Height: ::UINT,
    Depth: ::UINT,
}}
STRUCT!{struct D3DLOCKED_RECT {
    Pitch: ::INT,
    pBits: *mut ::c_void,
}}
STRUCT!{struct D3DBOX {
    Left: ::UINT,
    Top: ::UINT,
    Right: ::UINT,
    Bottom: ::UINT,
    Front: ::UINT,
    Back: ::UINT,
}}
STRUCT!{struct D3DLOCKED_BOX {
    RowPitch: ::INT,
    SlicePitch: ::INT,
    pBits: *mut ::c_void,
}}
STRUCT!{struct D3DRANGE {
    Offset: ::UINT,
    Size: ::UINT,
}}
STRUCT!{struct D3DRECTPATCH_INFO {
    StartVertexOffsetWidth: ::UINT,
    StartVertexOffsetHeight: ::UINT,
    Width: ::UINT,
    Height: ::UINT,
    Stride: ::UINT,
    Basis: D3DBASISTYPE,
    Degree: D3DDEGREETYPE,
}}
STRUCT!{struct D3DTRIPATCH_INFO {
    StartVertexOffset: ::UINT,
    NumVertices: ::UINT,
    Basis: D3DBASISTYPE,
    Degree: D3DDEGREETYPE,
}}
pub const MAX_DEVICE_IDENTIFIER_STRING: usize = 512;
#[repr(C)] #[derive(Copy)]
pub struct D3DADAPTER_IDENTIFIER9 {
    pub Driver: [::c_char; MAX_DEVICE_IDENTIFIER_STRING],
    pub Description: [::c_char; MAX_DEVICE_IDENTIFIER_STRING],
    pub DeviceName: [::c_char; 32],
    pub DriverVersion: ::LARGE_INTEGER,
    pub VendorId: ::DWORD,
    pub DeviceId: ::DWORD,
    pub SubSysId: ::DWORD,
    pub Revision: ::DWORD,
    pub DeviceIdentifier: ::GUID,
    pub WHQLLevel: ::DWORD,
}
impl Clone for D3DADAPTER_IDENTIFIER9 { fn clone(&self) -> D3DADAPTER_IDENTIFIER9 { *self } }
STRUCT!{struct D3DRASTER_STATUS {
    InVBlank: ::BOOL,
    ScanLine: ::UINT,
}}
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum D3DDEBUGMONITORTOKENS {
    ENABLE = 0,
    DISABLE = 1,
}
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum D3DQUERYTYPE {
    VCACHE = 4,
    RESOURCEMANAGER = 5,
    VERTEXSTATS = 6,
    EVENT = 8,
    OCCLUSION = 9,
    TIMESTAMP = 10,
    TIMESTAMPDISJOINT = 11,
    TIMESTAMPFREQ = 12,
    PIPELINETIMINGS = 13,
    INTERFACETIMINGS = 14,
    VERTEXTIMINGS = 15,
    PIXELTIMINGS = 16,
    BANDWIDTHTIMINGS = 17,
    CACHEUTILIZATION = 18,
    MEMORYPRESSURE = 19,
}
pub const D3DISSUE_END: ::DWORD = 1 << 0;
pub const D3DISSUE_BEGIN: ::DWORD = 1 << 1;
pub const D3DGETDATA_FLUSH: ::DWORD = 1 << 0;
STRUCT!{struct D3DRESOURCESTATS {
    bThrashing: ::BOOL,
    ApproxBytesDownloaded: ::DWORD,
    NumEvicts: ::DWORD,
    NumVidCreates: ::DWORD,
    LastPri: ::DWORD,
    NumUsed: ::DWORD,
    NumUsedInVidMem: ::DWORD,
    WorkingSet: ::DWORD,
    WorkingSetBytes: ::DWORD,
    TotalManaged: ::DWORD,
    TotalBytes: ::DWORD,
}}
pub const D3DRTYPECOUNT: usize = 8;
STRUCT!{struct D3DDEVINFO_RESOURCEMANAGER {
    stats: [D3DRESOURCESTATS; 8 /*D3DRTYPECOUNT, rust bug?*/],
}}
pub type LPD3DDEVINFO_RESOURCEMANAGER = *mut D3DDEVINFO_RESOURCEMANAGER;
STRUCT!{struct D3DDEVINFO_D3DVERTEXSTATS {
    NumRenderedTriangles: ::DWORD,
    NumExtraClippingTriangles: ::DWORD,
}}
pub type LPD3DDEVINFO_D3DVERTEXSTATS = *mut D3DDEVINFO_D3DVERTEXSTATS;
STRUCT!{struct D3DDEVINFO_VCACHE {
    Pattern: ::DWORD,
    OptMethod: ::DWORD,
    CacheSize: ::DWORD,
    MagicNumber: ::DWORD,
}}
pub type LPD3DDEVINFO_VCACHE = *mut D3DDEVINFO_VCACHE;
STRUCT!{struct D3DDEVINFO_D3D9PIPELINETIMINGS {
    VertexProcessingTimePercent: ::FLOAT,
    PixelProcessingTimePercent: ::FLOAT,
    OtherGPUProcessingTimePercent: ::FLOAT,
    GPUIdleTimePercent: ::FLOAT,
}}
STRUCT!{struct D3DDEVINFO_D3D9INTERFACETIMINGS {
    WaitingForGPUToUseApplicationResourceTimePercent: ::FLOAT,
    WaitingForGPUToAcceptMoreCommandsTimePercent: ::FLOAT,
    WaitingForGPUToStayWithinLatencyTimePercent: ::FLOAT,
    WaitingForGPUExclusiveResourceTimePercent: ::FLOAT,
    WaitingForGPUOtherTimePercent: ::FLOAT,
}}
STRUCT!{struct D3DDEVINFO_D3D9STAGETIMINGS {
    MemoryProcessingPercent: ::FLOAT,
    ComputationProcessingPercent: ::FLOAT,
}}
STRUCT!{struct D3DDEVINFO_D3D9BANDWIDTHTIMINGS {
    MaxBandwidthUtilized: ::FLOAT,
    FrontEndUploadMemoryUtilizedPercent: ::FLOAT,
    VertexRateUtilizedPercent: ::FLOAT,
    TriangleSetupRateUtilizedPercent: ::FLOAT,
    FillRateUtilizedPercent: ::FLOAT,
}}
STRUCT!{struct D3DDEVINFO_D3D9CACHEUTILIZATION {
    TextureCacheHitRate: ::FLOAT,
    PostTransformVertexCacheHitRate: ::FLOAT,
}}
STRUCT!{struct D3DMEMORYPRESSURE {
    BytesEvictedFromProcess: ::UINT64,
    SizeOfInefficientAllocation: ::UINT64,
    LevelOfEfficiency: ::DWORD,
}}
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum D3DCOMPOSERECTSOP {
    COPY = 1,
    OR = 2,
    AND = 3,
    NEG = 4,
}
STRUCT!{struct D3DCOMPOSERECTDESC {
    X: ::USHORT,
    Y: ::USHORT,
    Width: ::USHORT,
    Height: ::USHORT,
}}
STRUCT!{struct D3DCOMPOSERECTDESTINATION {
    SrcRectIndex: ::USHORT,
    Reserved: ::USHORT,
    X: ::SHORT,
    Y: ::SHORT,
}}
pub const D3DCOMPOSERECTS_MAXNUMRECTS: ::DWORD = 0xFFFF;
pub const D3DCONVOLUTIONMONO_MAXWIDTH: ::DWORD = 7;
pub const D3DCONVOLUTIONMONO_MAXHEIGHT: ::DWORD = D3DCONVOLUTIONMONO_MAXWIDTH;
pub const D3DFMT_A1_SURFACE_MAXWIDTH: ::DWORD = 8192;
pub const D3DFMT_A1_SURFACE_MAXHEIGHT: ::DWORD = 2048;
STRUCT!{struct D3DPRESENTSTATS {
    PresentCount: ::UINT,
    PresentRefreshCount: ::UINT,
    SyncRefreshCount: ::UINT,
    SyncQPCTime: ::LARGE_INTEGER,
    SyncGPUTime: ::LARGE_INTEGER,
}}
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum D3DSCANLINEORDERING {
    UNKNOWN = 0,
    PROGRESSIVE = 1,
    INTERLACED = 2,
}
STRUCT!{struct D3DDISPLAYMODEEX {
    Size: ::UINT,
    Width: ::UINT,
    Height: ::UINT,
    RefreshRate: ::UINT,
    Format: D3DFORMAT,
    ScanLineOrdering: D3DSCANLINEORDERING,
}}
STRUCT!{struct D3DDISPLAYMODEFILTER {
    Size: ::UINT,
    Format: D3DFORMAT,
    ScanLineOrdering: D3DSCANLINEORDERING,
}}
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum D3DDISPLAYROTATION {
    IDENTITY = 1,
    _90 = 2,
    _180 = 3,
    _270 = 4,
}
pub const D3D9_RESOURCE_PRIORITY_MINIMUM: ::DWORD = 0x28000000;
pub const D3D9_RESOURCE_PRIORITY_LOW: ::DWORD = 0x50000000;
pub const D3D9_RESOURCE_PRIORITY_NORMAL: ::DWORD = 0x78000000;
pub const D3D9_RESOURCE_PRIORITY_HIGH: ::DWORD = 0xa0000000;
pub const D3D9_RESOURCE_PRIORITY_MAXIMUM: ::DWORD = 0xc8000000;
pub const D3D_OMAC_SIZE: usize = 16;
STRUCT!{struct D3D_OMAC {
    Omac: [::BYTE; D3D_OMAC_SIZE],
}}
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum D3DAUTHENTICATEDCHANNELTYPE {
    D3D9 = 1,
    DRIVER_SOFTWARE = 2,
    DRIVER_HARDWARE = 3,
}
STRUCT!{struct D3DAUTHENTICATEDCHANNEL_QUERY_INPUT {
    QueryType: ::GUID,
    hChannel: ::HANDLE,
    SequenceNumber: ::UINT,
}}
STRUCT!{struct D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT {
    omac: D3D_OMAC,
    QueryType: ::GUID,
    hChannel: ::HANDLE,
    SequenceNumber: ::UINT,
    ReturnCode: ::HRESULT,
}}
DEFINE_GUID!(D3DAUTHENTICATEDQUERY_PROTECTION, 0xa84eb584, 0xc495, 0x48aa,
    0xb9, 0x4d, 0x8b, 0xd2, 0xd6, 0xfb, 0xce, 0x5);
STRUCT!{struct D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS {
    Value: ::UINT,
}}
STRUCT!{struct D3DAUTHENTICATEDCHANNEL_QUERYPROTECTION_OUTPUT {
    Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    ProtectionFlags: D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS,
}}
DEFINE_GUID!(D3DAUTHENTICATEDQUERY_CHANNELTYPE, 0xbc1b18a5, 0xb1fb, 0x42ab,
    0xbd, 0x94, 0xb5, 0x82, 0x8b, 0x4b, 0xf7, 0xbe);
STRUCT!{struct D3DAUTHENTICATEDCHANNEL_QUERYCHANNELTYPE_OUTPUT {
    Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    ChannelType: D3DAUTHENTICATEDCHANNELTYPE,
}}
DEFINE_GUID!(D3DAUTHENTICATEDQUERY_DEVICEHANDLE, 0xec1c539d, 0x8cff, 0x4e2a,
    0xbc, 0xc4, 0xf5, 0x69, 0x2f, 0x99, 0xf4, 0x80);
STRUCT!{struct D3DAUTHENTICATEDCHANNEL_QUERYDEVICEHANDLE_OUTPUT {
    Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    DeviceHandle: ::HANDLE,
}}
DEFINE_GUID!(D3DAUTHENTICATEDQUERY_CRYPTOSESSION, 0x2634499e, 0xd018, 0x4d74,
    0xac, 0x17, 0x7f, 0x72, 0x40, 0x59, 0x52, 0x8d);
STRUCT!{struct D3DAUTHENTICATEDCHANNEL_QUERYCRYPTOSESSION_INPUT {
    Input: D3DAUTHENTICATEDCHANNEL_QUERY_INPUT,
    DXVA2DecodeHandle: ::HANDLE,
}}
STRUCT!{struct D3DAUTHENTICATEDCHANNEL_QUERYCRYPTOSESSION_OUTPUT {
    Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    DXVA2DecodeHandle: ::HANDLE,
    CryptoSessionHandle: ::HANDLE,
    DeviceHandle: ::HANDLE,
}}
DEFINE_GUID!(D3DAUTHENTICATEDQUERY_RESTRICTEDSHAREDRESOURCEPROCESSCOUNT, 0xdb207b3, 0x9450, 0x46a6,
    0x82, 0xde, 0x1b, 0x96, 0xd4, 0x4f, 0x9c, 0xf2);
STRUCT!{struct D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESSCOUNT_OUTPUT {
    Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    NumRestrictedSharedResourceProcesses: ::UINT,
}}
DEFINE_GUID!(D3DAUTHENTICATEDQUERY_RESTRICTEDSHAREDRESOURCEPROCESS, 0x649bbadb, 0xf0f4, 0x4639,
    0xa1, 0x5b, 0x24, 0x39, 0x3f, 0xc3, 0xab, 0xac);
STRUCT!{struct D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESS_INPUT {
    Input: D3DAUTHENTICATEDCHANNEL_QUERY_INPUT,
    ProcessIndex: ::UINT,
}}
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum D3DAUTHENTICATEDCHANNEL_PROCESSIDENTIFIERTYPE {
    UNKNOWN = 0,
    DWM = 1,
    HANDLE = 2,
}
STRUCT!{struct D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESS_OUTPUT {
    Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    ProcessIndex: ::UINT,
    ProcessIdentifer: D3DAUTHENTICATEDCHANNEL_PROCESSIDENTIFIERTYPE,
    ProcessHandle: ::HANDLE,
}}
DEFINE_GUID!(D3DAUTHENTICATEDQUERY_UNRESTRICTEDPROTECTEDSHAREDRESOURCECOUNT,
    0x12f0bd6, 0xe662, 0x4474, 0xbe, 0xfd, 0xaa, 0x53, 0xe5, 0x14, 0x3c, 0x6d);
STRUCT!{struct D3DAUTHENTICATEDCHANNEL_QUERYUNRESTRICTEDPROTECTEDSHAREDRESOURCECOUNT_OUTPUT {
    Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    NumUnrestrictedProtectedSharedResources: ::UINT,
}}
DEFINE_GUID!(D3DAUTHENTICATEDQUERY_OUTPUTIDCOUNT, 0x2c042b5e, 0x8c07, 0x46d5,
    0xaa, 0xbe, 0x8f, 0x75, 0xcb, 0xad, 0x4c, 0x31);
STRUCT!{struct D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTIDCOUNT_INPUT {
    Input: D3DAUTHENTICATEDCHANNEL_QUERY_INPUT,
    DeviceHandle: ::HANDLE,
    CryptoSessionHandle: ::HANDLE,
}}
STRUCT!{struct D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTIDCOUNT_OUTPUT {
    Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    DeviceHandle: ::HANDLE,
    CryptoSessionHandle: ::HANDLE,
    NumOutputIDs: ::UINT,
}}
DEFINE_GUID!(D3DAUTHENTICATEDQUERY_OUTPUTID, 0x839ddca3, 0x9b4e, 0x41e4,
    0xb0, 0x53, 0x89, 0x2b, 0xd2, 0xa1, 0x1e, 0xe7);
STRUCT!{struct D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTID_INPUT {
    Input: D3DAUTHENTICATEDCHANNEL_QUERY_INPUT,
    DeviceHandle: ::HANDLE,
    CryptoSessionHandle: ::HANDLE,
    OutputIDIndex: ::UINT,
}}
STRUCT!{struct D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTID_OUTPUT {
    Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    DeviceHandle: ::HANDLE,
    CryptoSessionHandle: ::HANDLE,
    OutputIDIndex: ::UINT,
    OutputID: ::UINT64,
}}
DEFINE_GUID!(D3DAUTHENTICATEDQUERY_ACCESSIBILITYATTRIBUTES, 0x6214d9d2, 0x432c, 0x4abb,
    0x9f, 0xce, 0x21, 0x6e, 0xea, 0x26, 0x9e, 0x3b);
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum D3DBUSTYPE {
    OTHER = 0x00000000,
    PCI = 0x00000001,
    PCIX = 0x00000002,
    PCIEXPRESS = 0x00000003,
    AGP = 0x00000004,
    MODIFIER_INSIDE_OF_CHIPSET = 0x00010000,
    MODIFIER_TRACKS_ON_MOTHER_BOARD_TO_CHIP = 0x00020000,
    MODIFIER_TRACKS_ON_MOTHER_BOARD_TO_SOCKET = 0x00030000,
    MODIFIER_DAUGHTER_BOARD_CONNECTOR = 0x00040000,
    MODIFIER_DAUGHTER_BOARD_CONNECTOR_INSIDE_OF_NUAE = 0x00050000,
    MODIFIER_NON_STANDARD = 0x80000000u32 as i32,
}
STRUCT!{struct D3DAUTHENTICATEDCHANNEL_QUERYINFOBUSTYPE_OUTPUT {
    Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    BusType: D3DBUSTYPE,
    bAccessibleInContiguousBlocks: ::BOOL,
    bAccessibleInNonContiguousBlocks: ::BOOL,
}}
DEFINE_GUID!(D3DAUTHENTICATEDQUERY_ENCRYPTIONWHENACCESSIBLEGUIDCOUNT, 0xb30f7066, 0x203c, 0x4b07,
    0x93, 0xfc, 0xce, 0xaa, 0xfd, 0x61, 0x24, 0x1e);
STRUCT!{struct D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUIDCOUNT_OUTPUT {
    Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    NumEncryptionGuids: ::UINT,
}}
DEFINE_GUID!(D3DAUTHENTICATEDQUERY_ENCRYPTIONWHENACCESSIBLEGUID, 0xf83a5958, 0xe986, 0x4bda,
    0xbe, 0xb0, 0x41, 0x1f, 0x6a, 0x7a, 0x1, 0xb7);
STRUCT!{struct D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUID_INPUT {
    Input: D3DAUTHENTICATEDCHANNEL_QUERY_INPUT,
    EncryptionGuidIndex: ::UINT,
}}
STRUCT!{struct D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUID_OUTPUT {
    Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    EncryptionGuidIndex: ::UINT,
    EncryptionGuid: ::GUID,
}}
DEFINE_GUID!(D3DAUTHENTICATEDQUERY_CURRENTENCRYPTIONWHENACCESSIBLE, 0xec1791c7, 0xdad3, 0x4f15,
    0x9e, 0xc3, 0xfa, 0xa9, 0x3d, 0x60, 0xd4, 0xf0);
STRUCT!{struct D3DAUTHENTICATEDCHANNEL_QUERYUNCOMPRESSEDENCRYPTIONLEVEL_OUTPUT {
    Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    EncryptionGuid: ::GUID,
}}
STRUCT!{struct D3DAUTHENTICATEDCHANNEL_CONFIGURE_INPUT {
    omac: D3D_OMAC,
    ConfigureType: ::GUID,
    hChannel: ::HANDLE,
    SequenceNumber: ::UINT,
}}
STRUCT!{struct D3DAUTHENTICATEDCHANNEL_CONFIGURE_OUTPUT {
    omac: D3D_OMAC,
    ConfigureType: ::GUID,
    hChannel: ::HANDLE,
    SequenceNumber: ::UINT,
    ReturnCode: ::HRESULT,
}}
DEFINE_GUID!(D3DAUTHENTICATEDCONFIGURE_INITIALIZE, 0x6114bdb, 0x3523, 0x470a,
    0x8d, 0xca, 0xfb, 0xc2, 0x84, 0x51, 0x54, 0xf0);
STRUCT!{struct D3DAUTHENTICATEDCHANNEL_CONFIGUREINITIALIZE {
    Parameters: D3DAUTHENTICATEDCHANNEL_CONFIGURE_INPUT,
    StartSequenceQuery: ::UINT,
    StartSequenceConfigure: ::UINT,
}}
DEFINE_GUID!(D3DAUTHENTICATEDCONFIGURE_PROTECTION, 0x50455658, 0x3f47, 0x4362,
    0xbf, 0x99, 0xbf, 0xdf, 0xcd, 0xe9, 0xed, 0x29);
STRUCT!{struct D3DAUTHENTICATEDCHANNEL_CONFIGUREPROTECTION {
    Parameters: D3DAUTHENTICATEDCHANNEL_CONFIGURE_INPUT,
    Protections: D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS,
}}
DEFINE_GUID!(D3DAUTHENTICATEDCONFIGURE_CRYPTOSESSION, 0x6346cc54, 0x2cfc, 0x4ad4,
    0x82, 0x24, 0xd1, 0x58, 0x37, 0xde, 0x77, 0x0);
STRUCT!{struct D3DAUTHENTICATEDCHANNEL_CONFIGURECRYPTOSESSION {
    Parameters: D3DAUTHENTICATEDCHANNEL_CONFIGURE_INPUT,
    DXVA2DecodeHandle: ::HANDLE,
    CryptoSessionHandle: ::HANDLE,
    DeviceHandle: ::HANDLE,
}}
DEFINE_GUID!(D3DAUTHENTICATEDCONFIGURE_SHAREDRESOURCE, 0x772d047, 0x1b40, 0x48e8,
    0x9c, 0xa6, 0xb5, 0xf5, 0x10, 0xde, 0x9f, 0x1);
STRUCT!{struct D3DAUTHENTICATEDCHANNEL_CONFIGURESHAREDRESOURCE {
    Parameters: D3DAUTHENTICATEDCHANNEL_CONFIGURE_INPUT,
    ProcessIdentiferType: D3DAUTHENTICATEDCHANNEL_PROCESSIDENTIFIERTYPE,
    ProcessHandle: ::HANDLE,
    AllowAccess: ::BOOL,
}}
DEFINE_GUID!(D3DAUTHENTICATEDCONFIGURE_ENCRYPTIONWHENACCESSIBLE, 0x41fff286, 0x6ae0, 0x4d43,
    0x9d, 0x55, 0xa4, 0x6e, 0x9e, 0xfd, 0x15, 0x8a);
STRUCT!{struct D3DAUTHENTICATEDCHANNEL_CONFIGUREUNCOMPRESSEDENCRYPTION {
    Parameters: D3DAUTHENTICATEDCHANNEL_CONFIGURE_INPUT,
    EncryptionGuid: ::GUID,
}}
STRUCT!{struct D3DENCRYPTED_BLOCK_INFO {
    NumEncryptedBytesAtBeginning: ::UINT,
    NumBytesInSkipPattern: ::UINT,
    NumBytesInEncryptPattern: ::UINT,
}}
STRUCT!{struct D3DAES_CTR_IV {
    IV: ::UINT64,
    Count: ::UINT64,
}}

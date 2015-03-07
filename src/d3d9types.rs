// Copyright © 2015, Corey Richardson
// Licensed under the MIT License <LICENSE.md>
//! Direct3D capabilities include file

use std::mem::transmute;

pub type D3DCOLOR = ::DWORD;

#[repr(C)]
#[derive(Copy)]
pub struct D3DVECTOR {
    pub x: ::FLOAT,
    pub y: ::FLOAT,
    pub z: ::FLOAT,
}

#[repr(C)]
#[derive(Copy)]
pub struct D3DCOLORVALUE {
    pub r: ::FLOAT,
    pub g: ::FLOAT,
    pub b: ::FLOAT,
    pub a: ::FLOAT,
}

#[repr(C)]
#[derive(Copy)]
pub struct D3DRECT {
    pub x1: ::LONG,
    pub y1: ::LONG,
    pub x2: ::LONG,
    pub y2: ::LONG,
}

#[repr(C)]
#[derive(Copy)]
pub struct D3DMATRIX {
    pub _bindgen_data_1_: [u32; 16usize],
}
impl D3DMATRIX {
    pub unsafe fn _11(&mut self) -> *mut ::FLOAT {
        transmute(&self._bindgen_data_1_)
    }
    pub unsafe fn _12(&mut self) -> *mut ::FLOAT {
        let raw: *mut u8 = transmute(&self._bindgen_data_1_);
        transmute(raw.offset(4isize))
    }
    pub unsafe fn _13(&mut self) -> *mut ::FLOAT {
        let raw: *mut u8 = transmute(&self._bindgen_data_1_);
        transmute(raw.offset(8isize))
    }
    pub unsafe fn _14(&mut self) -> *mut ::FLOAT {
        let raw: *mut u8 = transmute(&self._bindgen_data_1_);
        transmute(raw.offset(12isize))
    }
    pub unsafe fn _21(&mut self) -> *mut ::FLOAT {
        let raw: *mut u8 = transmute(&self._bindgen_data_1_);
        transmute(raw.offset(16isize))
    }
    pub unsafe fn _22(&mut self) -> *mut ::FLOAT {
        let raw: *mut u8 = transmute(&self._bindgen_data_1_);
        transmute(raw.offset(20isize))
    }
    pub unsafe fn _23(&mut self) -> *mut ::FLOAT {
        let raw: *mut u8 = transmute(&self._bindgen_data_1_);
        transmute(raw.offset(24isize))
    }
    pub unsafe fn _24(&mut self) -> *mut ::FLOAT {
        let raw: *mut u8 = transmute(&self._bindgen_data_1_);
        transmute(raw.offset(28isize))
    }
    pub unsafe fn _31(&mut self) -> *mut ::FLOAT {
        let raw: *mut u8 = transmute(&self._bindgen_data_1_);
        transmute(raw.offset(32isize))
    }
    pub unsafe fn _32(&mut self) -> *mut ::FLOAT {
        let raw: *mut u8 = transmute(&self._bindgen_data_1_);
        transmute(raw.offset(36isize))
    }
    pub unsafe fn _33(&mut self) -> *mut ::FLOAT {
        let raw: *mut u8 = transmute(&self._bindgen_data_1_);
        transmute(raw.offset(40isize))
    }
    pub unsafe fn _34(&mut self) -> *mut ::FLOAT {
        let raw: *mut u8 = transmute(&self._bindgen_data_1_);
        transmute(raw.offset(44isize))
    }
    pub unsafe fn _41(&mut self) -> *mut ::FLOAT {
        let raw: *mut u8 = transmute(&self._bindgen_data_1_);
        transmute(raw.offset(48isize))
    }
    pub unsafe fn _42(&mut self) -> *mut ::FLOAT {
        let raw: *mut u8 = transmute(&self._bindgen_data_1_);
        transmute(raw.offset(52isize))
    }
    pub unsafe fn _43(&mut self) -> *mut ::FLOAT {
        let raw: *mut u8 = transmute(&self._bindgen_data_1_);
        transmute(raw.offset(56isize))
    }
    pub unsafe fn _44(&mut self) -> *mut ::FLOAT {
        let raw: *mut u8 = transmute(&self._bindgen_data_1_);
        transmute(raw.offset(60isize))
    }
    pub unsafe fn m(&mut self) -> *mut [[::libc::c_float; 4usize]; 4usize] {
        transmute(&self._bindgen_data_1_)
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct D3DVIEWPORT9 {
    pub X: ::DWORD,
    pub Y: ::DWORD,
    pub Width: ::DWORD,
    pub Height: ::DWORD,
    pub MinZ: ::FLOAT,
    pub MaxZ: ::FLOAT,
}

#[repr(C)]
#[derive(Copy)]
pub struct D3DCLIPSTATUS9 {
    pub ClipUnion: ::DWORD,
    pub ClipIntersection: ::DWORD,
}

#[repr(C)]
#[derive(Copy)]
pub struct D3DMATERIAL9 {
    pub Diffuse: D3DCOLORVALUE,
    pub Ambient: D3DCOLORVALUE,
    pub Specular: D3DCOLORVALUE,
    pub Emissive: D3DCOLORVALUE,
    pub Power: ::FLOAT,
}

pub const D3DLIGHT_POINT: ::UINT = 1;
pub const D3DLIGHT_SPOT: ::UINT = 2;
pub const D3DLIGHT_DIRECTIONAL: ::UINT = 3;
pub const D3DLIGHT_FORCE_DWORD: ::UINT = 2147483647;

pub type D3DLIGHTTYPE = ::UINT;

#[repr(C)]
#[derive(Copy)]
pub struct D3DLIGHT9 {
    pub Type: D3DLIGHTTYPE,
    pub Diffuse: D3DCOLORVALUE,
    pub Specular: D3DCOLORVALUE,
    pub Ambient: D3DCOLORVALUE,
    pub Position: D3DVECTOR,
    pub Direction: D3DVECTOR,
    pub Range: ::FLOAT,
    pub Falloff: ::FLOAT,
    pub Attenuation0: ::FLOAT,
    pub Attenuation1: ::FLOAT,
    pub Attenuation2: ::FLOAT,
    pub Theta: ::FLOAT,
    pub Phi: ::FLOAT,
}

pub type D3DSHADEMODE = ::UINT;
pub const D3DSHADE_FLAT: ::UINT = 1;
pub const D3DSHADE_GOURAUD: ::UINT = 2;
pub const D3DSHADE_PHONG: ::UINT = 3;
pub const D3DSHADE_FORCE_DWORD: ::UINT = 2147483647;

pub type D3DFILLMODE = ::UINT;
pub const D3DFILL_POINT: ::UINT = 1;
pub const D3DFILL_WIREFRAME: ::UINT = 2;
pub const D3DFILL_SOLID: ::UINT = 3;
pub const D3DFILL_FORCE_DWORD: ::UINT = 2147483647;

pub type D3DBLEND = ::UINT;
pub const D3DBLEND_ZERO: ::UINT = 1;
pub const D3DBLEND_ONE: ::UINT = 2;
pub const D3DBLEND_SRCCOLOR: ::UINT = 3;
pub const D3DBLEND_INVSRCCOLOR: ::UINT = 4;
pub const D3DBLEND_SRCALPHA: ::UINT = 5;
pub const D3DBLEND_INVSRCALPHA: ::UINT = 6;
pub const D3DBLEND_DESTALPHA: ::UINT = 7;
pub const D3DBLEND_INVDESTALPHA: ::UINT = 8;
pub const D3DBLEND_DESTCOLOR: ::UINT = 9;
pub const D3DBLEND_INVDESTCOLOR: ::UINT = 10;
pub const D3DBLEND_SRCALPHASAT: ::UINT = 11;
pub const D3DBLEND_BOTHSRCALPHA: ::UINT = 12;
pub const D3DBLEND_BOTHINVSRCALPHA: ::UINT = 13;
pub const D3DBLEND_BLENDFACTOR: ::UINT = 14;
pub const D3DBLEND_INVBLENDFACTOR: ::UINT = 15;
pub const D3DBLEND_SRCCOLOR2: ::UINT = 16;
pub const D3DBLEND_INVSRCCOLOR2: ::UINT = 17;
pub const D3DBLEND_FORCE_DWORD: ::UINT = 2147483647;

pub type D3DBLENDOP = ::UINT;
pub const D3DBLENDOP_ADD: ::UINT = 1;
pub const D3DBLENDOP_SUBTRACT: ::UINT = 2;
pub const D3DBLENDOP_REVSUBTRACT: ::UINT = 3;
pub const D3DBLENDOP_MIN: ::UINT = 4;
pub const D3DBLENDOP_MAX: ::UINT = 5;
pub const D3DBLENDOP_FORCE_DWORD: ::UINT = 2147483647;

pub type D3DTEXTUREADDRESS = ::UINT;
pub const D3DTADDRESS_WRAP: ::UINT = 1;
pub const D3DTADDRESS_MIRROR: ::UINT = 2;
pub const D3DTADDRESS_CLAMP: ::UINT = 3;
pub const D3DTADDRESS_BORDER: ::UINT = 4;
pub const D3DTADDRESS_MIRRORONCE: ::UINT = 5;
pub const D3DTADDRESS_FORCE_DWORD: ::UINT = 2147483647;

pub type D3DCULL = ::UINT;
pub const D3DCULL_NONE: ::UINT = 1;
pub const D3DCULL_CW: ::UINT = 2;
pub const D3DCULL_CCW: ::UINT = 3;
pub const D3DCULL_FORCE_DWORD: ::UINT = 2147483647;

pub type D3DCMPFUNC = ::UINT;
pub const D3DCMP_NEVER: ::UINT = 1;
pub const D3DCMP_LESS: ::UINT = 2;
pub const D3DCMP_EQUAL: ::UINT = 3;
pub const D3DCMP_LESSEQUAL: ::UINT = 4;
pub const D3DCMP_GREATER: ::UINT = 5;
pub const D3DCMP_NOTEQUAL: ::UINT = 6;
pub const D3DCMP_GREATEREQUAL: ::UINT = 7;
pub const D3DCMP_ALWAYS: ::UINT = 8;
pub const D3DCMP_FORCE_DWORD: ::UINT = 2147483647;

pub type D3DSTENCILOP = ::UINT;
pub const D3DSTENCILOP_KEEP: ::UINT = 1;
pub const D3DSTENCILOP_ZERO: ::UINT = 2;
pub const D3DSTENCILOP_REPLACE: ::UINT = 3;
pub const D3DSTENCILOP_INCRSAT: ::UINT = 4;
pub const D3DSTENCILOP_DECRSAT: ::UINT = 5;
pub const D3DSTENCILOP_INVERT: ::UINT = 6;
pub const D3DSTENCILOP_INCR: ::UINT = 7;
pub const D3DSTENCILOP_DECR: ::UINT = 8;
pub const D3DSTENCILOP_FORCE_DWORD: ::UINT = 2147483647;

pub type D3DFOGMODE = ::UINT;
pub const D3DFOG_NONE: ::UINT = 0;
pub const D3DFOG_EXP: ::UINT = 1;
pub const D3DFOG_EXP2: ::UINT = 2;
pub const D3DFOG_LINEAR: ::UINT = 3;
pub const D3DFOG_FORCE_DWORD: ::UINT = 2147483647;

pub type D3DZBUFFERTYPE = ::UINT;
pub const D3DZB_FALSE: ::UINT = 0;
pub const D3DZB_TRUE: ::UINT = 1;
pub const D3DZB_USEW: ::UINT = 2;
pub const D3DZB_FORCE_DWORD: ::UINT = 2147483647;

pub type D3DPRIMITIVETYPE = ::UINT;
pub const D3DPT_POINTLIST: ::UINT = 1;
pub const D3DPT_LINELIST: ::UINT = 2;
pub const D3DPT_LINESTRIP: ::UINT = 3;
pub const D3DPT_TRIANGLELIST: ::UINT = 4;
pub const D3DPT_TRIANGLESTRIP: ::UINT = 5;
pub const D3DPT_TRIANGLEFAN: ::UINT = 6;
pub const D3DPT_FORCE_DWORD: ::UINT = 2147483647;

pub type D3DTRANSFORMSTATETYPE = ::UINT;
pub const D3DTS_VIEW: ::UINT = 2;
pub const D3DTS_PROJECTION: ::UINT = 3;
pub const D3DTS_TEXTURE0: ::UINT = 16;
pub const D3DTS_TEXTURE1: ::UINT = 17;
pub const D3DTS_TEXTURE2: ::UINT = 18;
pub const D3DTS_TEXTURE3: ::UINT = 19;
pub const D3DTS_TEXTURE4: ::UINT = 20;
pub const D3DTS_TEXTURE5: ::UINT = 21;
pub const D3DTS_TEXTURE6: ::UINT = 22;
pub const D3DTS_TEXTURE7: ::UINT = 23;
pub const D3DTS_FORCE_DWORD: ::UINT = 2147483647;

pub type D3DRENDERSTATETYPE = ::UINT;
pub const D3DRS_ZENABLE: ::UINT = 7;
pub const D3DRS_FILLMODE: ::UINT = 8;
pub const D3DRS_SHADEMODE: ::UINT = 9;
pub const D3DRS_ZWRITEENABLE: ::UINT = 14;
pub const D3DRS_ALPHATESTENABLE: ::UINT = 15;
pub const D3DRS_LASTPIXEL: ::UINT = 16;
pub const D3DRS_SRCBLEND: ::UINT = 19;
pub const D3DRS_DESTBLEND: ::UINT = 20;
pub const D3DRS_CULLMODE: ::UINT = 22;
pub const D3DRS_ZFUNC: ::UINT = 23;
pub const D3DRS_ALPHAREF: ::UINT = 24;
pub const D3DRS_ALPHAFUNC: ::UINT = 25;
pub const D3DRS_DITHERENABLE: ::UINT = 26;
pub const D3DRS_ALPHABLENDENABLE: ::UINT = 27;
pub const D3DRS_FOGENABLE: ::UINT = 28;
pub const D3DRS_SPECULARENABLE: ::UINT = 29;
pub const D3DRS_FOGCOLOR: ::UINT = 34;
pub const D3DRS_FOGTABLEMODE: ::UINT = 35;
pub const D3DRS_FOGSTART: ::UINT = 36;
pub const D3DRS_FOGEND: ::UINT = 37;
pub const D3DRS_FOGDENSITY: ::UINT = 38;
pub const D3DRS_RANGEFOGENABLE: ::UINT = 48;
pub const D3DRS_STENCILENABLE: ::UINT = 52;
pub const D3DRS_STENCILFAIL: ::UINT = 53;
pub const D3DRS_STENCILZFAIL: ::UINT = 54;
pub const D3DRS_STENCILPASS: ::UINT = 55;
pub const D3DRS_STENCILFUNC: ::UINT = 56;
pub const D3DRS_STENCILREF: ::UINT = 57;
pub const D3DRS_STENCILMASK: ::UINT = 58;
pub const D3DRS_STENCILWRITEMASK: ::UINT = 59;
pub const D3DRS_TEXTUREFACTOR: ::UINT = 60;
pub const D3DRS_WRAP0: ::UINT = 128;
pub const D3DRS_WRAP1: ::UINT = 129;
pub const D3DRS_WRAP2: ::UINT = 130;
pub const D3DRS_WRAP3: ::UINT = 131;
pub const D3DRS_WRAP4: ::UINT = 132;
pub const D3DRS_WRAP5: ::UINT = 133;
pub const D3DRS_WRAP6: ::UINT = 134;
pub const D3DRS_WRAP7: ::UINT = 135;
pub const D3DRS_CLIPPING: ::UINT = 136;
pub const D3DRS_LIGHTING: ::UINT = 137;
pub const D3DRS_AMBIENT: ::UINT = 139;
pub const D3DRS_FOGVERTEXMODE: ::UINT = 140;
pub const D3DRS_COLORVERTEX: ::UINT = 141;
pub const D3DRS_LOCALVIEWER: ::UINT = 142;
pub const D3DRS_NORMALIZENORMALS: ::UINT = 143;
pub const D3DRS_DIFFUSEMATERIALSOURCE: ::UINT = 145;
pub const D3DRS_SPECULARMATERIALSOURCE: ::UINT = 146;
pub const D3DRS_AMBIENTMATERIALSOURCE: ::UINT = 147;
pub const D3DRS_EMISSIVEMATERIALSOURCE: ::UINT = 148;
pub const D3DRS_VERTEXBLEND: ::UINT = 151;
pub const D3DRS_CLIPPLANEENABLE: ::UINT = 152;
pub const D3DRS_POINTSIZE: ::UINT = 154;
pub const D3DRS_POINTSIZE_MIN: ::UINT = 155;
pub const D3DRS_POINTSPRITEENABLE: ::UINT = 156;
pub const D3DRS_POINTSCALEENABLE: ::UINT = 157;
pub const D3DRS_POINTSCALE_A: ::UINT = 158;
pub const D3DRS_POINTSCALE_B: ::UINT = 159;
pub const D3DRS_POINTSCALE_C: ::UINT = 160;
pub const D3DRS_MULTISAMPLEANTIALIAS: ::UINT = 161;
pub const D3DRS_MULTISAMPLEMASK: ::UINT = 162;
pub const D3DRS_PATCHEDGESTYLE: ::UINT = 163;
pub const D3DRS_DEBUGMONITORTOKEN: ::UINT = 165;
pub const D3DRS_POINTSIZE_MAX: ::UINT = 166;
pub const D3DRS_INDEXEDVERTEXBLENDENABLE: ::UINT = 167;
pub const D3DRS_COLORWRITEENABLE: ::UINT = 168;
pub const D3DRS_TWEENFACTOR: ::UINT = 170;
pub const D3DRS_BLENDOP: ::UINT = 171;
pub const D3DRS_POSITIONDEGREE: ::UINT = 172;
pub const D3DRS_NORMALDEGREE: ::UINT = 173;
pub const D3DRS_SCISSORTESTENABLE: ::UINT = 174;
pub const D3DRS_SLOPESCALEDEPTHBIAS: ::UINT = 175;
pub const D3DRS_ANTIALIASEDLINEENABLE: ::UINT = 176;
pub const D3DRS_MINTESSELLATIONLEVEL: ::UINT = 178;
pub const D3DRS_MAXTESSELLATIONLEVEL: ::UINT = 179;
pub const D3DRS_ADAPTIVETESS_X: ::UINT = 180;
pub const D3DRS_ADAPTIVETESS_Y: ::UINT = 181;
pub const D3DRS_ADAPTIVETESS_Z: ::UINT = 182;
pub const D3DRS_ADAPTIVETESS_W: ::UINT = 183;
pub const D3DRS_ENABLEADAPTIVETESSELLATION: ::UINT = 184;
pub const D3DRS_TWOSIDEDSTENCILMODE: ::UINT = 185;
pub const D3DRS_CCW_STENCILFAIL: ::UINT = 186;
pub const D3DRS_CCW_STENCILZFAIL: ::UINT = 187;
pub const D3DRS_CCW_STENCILPASS: ::UINT = 188;
pub const D3DRS_CCW_STENCILFUNC: ::UINT = 189;
pub const D3DRS_COLORWRITEENABLE1: ::UINT = 190;
pub const D3DRS_COLORWRITEENABLE2: ::UINT = 191;
pub const D3DRS_COLORWRITEENABLE3: ::UINT = 192;
pub const D3DRS_BLENDFACTOR: ::UINT = 193;
pub const D3DRS_SRGBWRITEENABLE: ::UINT = 194;
pub const D3DRS_DEPTHBIAS: ::UINT = 195;
pub const D3DRS_WRAP8: ::UINT = 198;
pub const D3DRS_WRAP9: ::UINT = 199;
pub const D3DRS_WRAP10: ::UINT = 200;
pub const D3DRS_WRAP11: ::UINT = 201;
pub const D3DRS_WRAP12: ::UINT = 202;
pub const D3DRS_WRAP13: ::UINT = 203;
pub const D3DRS_WRAP14: ::UINT = 204;
pub const D3DRS_WRAP15: ::UINT = 205;
pub const D3DRS_SEPARATEALPHABLENDENABLE: ::UINT = 206;
pub const D3DRS_SRCBLENDALPHA: ::UINT = 207;
pub const D3DRS_DESTBLENDALPHA: ::UINT = 208;
pub const D3DRS_BLENDOPALPHA: ::UINT = 209;
pub const D3DRS_FORCE_DWORD: ::UINT = 2147483647;

pub type D3DMATERIALCOLORSOURCE = ::UINT;
pub const D3DMCS_MATERIAL: ::UINT = 0;
pub const D3DMCS_COLOR1: ::UINT = 1;
pub const D3DMCS_COLOR2: ::UINT = 2;
pub const D3DMCS_FORCE_DWORD: ::UINT = 2147483647;

pub type D3DTEXTURESTAGESTATETYPE = ::UINT;
pub const D3DTSS_COLOROP: ::UINT = 1;
pub const D3DTSS_COLORARG1: ::UINT = 2;
pub const D3DTSS_COLORARG2: ::UINT = 3;
pub const D3DTSS_ALPHAOP: ::UINT = 4;
pub const D3DTSS_ALPHAARG1: ::UINT = 5;
pub const D3DTSS_ALPHAARG2: ::UINT = 6;
pub const D3DTSS_BUMPENVMAT00: ::UINT = 7;
pub const D3DTSS_BUMPENVMAT01: ::UINT = 8;
pub const D3DTSS_BUMPENVMAT10: ::UINT = 9;
pub const D3DTSS_BUMPENVMAT11: ::UINT = 10;
pub const D3DTSS_TEXCOORDINDEX: ::UINT = 11;
pub const D3DTSS_BUMPENVLSCALE: ::UINT = 22;
pub const D3DTSS_BUMPENVLOFFSET: ::UINT = 23;
pub const D3DTSS_TEXTURETRANSFORMFLAGS: ::UINT = 24;
pub const D3DTSS_COLORARG0: ::UINT = 26;
pub const D3DTSS_ALPHAARG0: ::UINT = 27;
pub const D3DTSS_RESULTARG: ::UINT = 28;
pub const D3DTSS_CONSTANT: ::UINT = 32;
pub const D3DTSS_FORCE_DWORD: ::UINT = 2147483647;

pub type D3DSAMPLERSTATETYPE = ::UINT;
pub const D3DSAMP_ADDRESSU: ::UINT = 1;
pub const D3DSAMP_ADDRESSV: ::UINT = 2;
pub const D3DSAMP_ADDRESSW: ::UINT = 3;
pub const D3DSAMP_BORDERCOLOR: ::UINT = 4;
pub const D3DSAMP_MAGFILTER: ::UINT = 5;
pub const D3DSAMP_MINFILTER: ::UINT = 6;
pub const D3DSAMP_MIPFILTER: ::UINT = 7;
pub const D3DSAMP_MIPMAPLODBIAS: ::UINT = 8;
pub const D3DSAMP_MAXMIPLEVEL: ::UINT = 9;
pub const D3DSAMP_MAXANISOTROPY: ::UINT = 10;
pub const D3DSAMP_SRGBTEXTURE: ::UINT = 11;
pub const D3DSAMP_ELEMENTINDEX: ::UINT = 12;
pub const D3DSAMP_DMAPOFFSET: ::UINT = 13;
pub const D3DSAMP_FORCE_DWORD: ::UINT = 2147483647;

pub type D3DTEXTUREOP = ::UINT;
pub const D3DTOP_DISABLE: ::UINT = 1;
pub const D3DTOP_SELECTARG1: ::UINT = 2;
pub const D3DTOP_SELECTARG2: ::UINT = 3;
pub const D3DTOP_MODULATE: ::UINT = 4;
pub const D3DTOP_MODULATE2X: ::UINT = 5;
pub const D3DTOP_MODULATE4X: ::UINT = 6;
pub const D3DTOP_ADD: ::UINT = 7;
pub const D3DTOP_ADDSIGNED: ::UINT = 8;
pub const D3DTOP_ADDSIGNED2X: ::UINT = 9;
pub const D3DTOP_SUBTRACT: ::UINT = 10;
pub const D3DTOP_ADDSMOOTH: ::UINT = 11;
pub const D3DTOP_BLENDDIFFUSEALPHA: ::UINT = 12;
pub const D3DTOP_BLENDTEXTUREALPHA: ::UINT = 13;
pub const D3DTOP_BLENDFACTORALPHA: ::UINT = 14;
pub const D3DTOP_BLENDTEXTUREALPHAPM: ::UINT = 15;
pub const D3DTOP_BLENDCURRENTALPHA: ::UINT = 16;
pub const D3DTOP_PREMODULATE: ::UINT = 17;
pub const D3DTOP_MODULATEALPHA_ADDCOLOR: ::UINT = 18;
pub const D3DTOP_MODULATECOLOR_ADDALPHA: ::UINT = 19;
pub const D3DTOP_MODULATEINVALPHA_ADDCOLOR: ::UINT = 20;
pub const D3DTOP_MODULATEINVCOLOR_ADDALPHA: ::UINT = 21;
pub const D3DTOP_BUMPENVMAP: ::UINT = 22;
pub const D3DTOP_BUMPENVMAPLUMINANCE: ::UINT = 23;
pub const D3DTOP_DOTPRODUCT3: ::UINT = 24;
pub const D3DTOP_MULTIPLYADD: ::UINT = 25;
pub const D3DTOP_LERP: ::UINT = 26;
pub const D3DTOP_FORCE_DWORD: ::UINT = 2147483647;

pub type D3DTEXTUREFILTERTYPE = ::UINT;
pub const D3DTEXF_NONE: ::UINT = 0;
pub const D3DTEXF_POINT: ::UINT = 1;
pub const D3DTEXF_LINEAR: ::UINT = 2;
pub const D3DTEXF_ANISOTROPIC: ::UINT = 3;
pub const D3DTEXF_PYRAMIDALQUAD: ::UINT = 6;
pub const D3DTEXF_GAUSSIANQUAD: ::UINT = 7;
pub const D3DTEXF_CONVOLUTIONMONO: ::UINT = 8;
pub const D3DTEXF_FORCE_DWORD: ::UINT = 2147483647;

pub type D3DDECLUSAGE = ::UINT;
pub const D3DDECLUSAGE_POSITION: ::UINT = 0;
pub const D3DDECLUSAGE_BLENDWEIGHT: ::UINT = 1;
pub const D3DDECLUSAGE_BLENDINDICES: ::UINT = 2;
pub const D3DDECLUSAGE_NORMAL: ::UINT = 3;
pub const D3DDECLUSAGE_PSIZE: ::UINT = 4;
pub const D3DDECLUSAGE_TEXCOORD: ::UINT = 5;
pub const D3DDECLUSAGE_TANGENT: ::UINT = 6;
pub const D3DDECLUSAGE_BINORMAL: ::UINT = 7;
pub const D3DDECLUSAGE_TESSFACTOR: ::UINT = 8;
pub const D3DDECLUSAGE_POSITIONT: ::UINT = 9;
pub const D3DDECLUSAGE_COLOR: ::UINT = 10;
pub const D3DDECLUSAGE_FOG: ::UINT = 11;
pub const D3DDECLUSAGE_DEPTH: ::UINT = 12;
pub const D3DDECLUSAGE_SAMPLE: ::UINT = 13;

pub type D3DDECLMETHOD = ::UINT;
pub const D3DDECLMETHOD_DEFAULT: ::UINT = 0;
pub const D3DDECLMETHOD_PARTIALU: ::UINT = 1;
pub const D3DDECLMETHOD_PARTIALV: ::UINT = 2;
pub const D3DDECLMETHOD_CROSSUV: ::UINT = 3;
pub const D3DDECLMETHOD_UV: ::UINT = 4;
pub const D3DDECLMETHOD_LOOKUP: ::UINT = 5;
pub const D3DDECLMETHOD_LOOKUPPRESAMPLED: ::UINT = 6;

pub type D3DDECLTYPE = ::UINT;
pub const D3DDECLTYPE_FLOAT1: ::UINT = 0;
pub const D3DDECLTYPE_FLOAT2: ::UINT = 1;
pub const D3DDECLTYPE_FLOAT3: ::UINT = 2;
pub const D3DDECLTYPE_FLOAT4: ::UINT = 3;
pub const D3DDECLTYPE_D3DCOLOR: ::UINT = 4;
pub const D3DDECLTYPE_UBYTE4: ::UINT = 5;
pub const D3DDECLTYPE_SHORT2: ::UINT = 6;
pub const D3DDECLTYPE_SHORT4: ::UINT = 7;
pub const D3DDECLTYPE_UBYTE4N: ::UINT = 8;
pub const D3DDECLTYPE_SHORT2N: ::UINT = 9;
pub const D3DDECLTYPE_SHORT4N: ::UINT = 10;
pub const D3DDECLTYPE_USHORT2N: ::UINT = 11;
pub const D3DDECLTYPE_USHORT4N: ::UINT = 12;
pub const D3DDECLTYPE_UDEC3: ::UINT = 13;
pub const D3DDECLTYPE_DEC3N: ::UINT = 14;
pub const D3DDECLTYPE_FLOAT16_2: ::UINT = 15;
pub const D3DDECLTYPE_FLOAT16_4: ::UINT = 16;
pub const D3DDECLTYPE_UNUSED: ::UINT = 17;

#[repr(C)]
#[derive(Copy)]
pub struct D3DVERTEXELEMENT9 {
    pub Stream: ::WORD,
    pub Offset: ::WORD,
    pub Type: ::BYTE,
    pub Method: ::BYTE,
    pub Usage: ::BYTE,
    pub UsageIndex: ::BYTE,
}

pub type LPD3DVERTEXELEMENT9 = *mut D3DVERTEXELEMENT9;
pub type D3DSHADER_INSTRUCTION_OPCODE_TYPE = ::UINT;
pub const D3DSIO_NOP: ::UINT = 0;
pub const D3DSIO_MOV: ::UINT = 1;
pub const D3DSIO_ADD: ::UINT = 2;
pub const D3DSIO_SUB: ::UINT = 3;
pub const D3DSIO_MAD: ::UINT = 4;
pub const D3DSIO_MUL: ::UINT = 5;
pub const D3DSIO_RCP: ::UINT = 6;
pub const D3DSIO_RSQ: ::UINT = 7;
pub const D3DSIO_DP3: ::UINT = 8;
pub const D3DSIO_DP4: ::UINT = 9;
pub const D3DSIO_MIN: ::UINT = 10;
pub const D3DSIO_MAX: ::UINT = 11;
pub const D3DSIO_SLT: ::UINT = 12;
pub const D3DSIO_SGE: ::UINT = 13;
pub const D3DSIO_EXP: ::UINT = 14;
pub const D3DSIO_LOG: ::UINT = 15;
pub const D3DSIO_LIT: ::UINT = 16;
pub const D3DSIO_DST: ::UINT = 17;
pub const D3DSIO_LRP: ::UINT = 18;
pub const D3DSIO_FRC: ::UINT = 19;
pub const D3DSIO_M4x4: ::UINT = 20;
pub const D3DSIO_M4x3: ::UINT = 21;
pub const D3DSIO_M3x4: ::UINT = 22;
pub const D3DSIO_M3x3: ::UINT = 23;
pub const D3DSIO_M3x2: ::UINT = 24;
pub const D3DSIO_CALL: ::UINT = 25;
pub const D3DSIO_CALLNZ: ::UINT = 26;
pub const D3DSIO_LOOP: ::UINT = 27;
pub const D3DSIO_RET: ::UINT = 28;
pub const D3DSIO_ENDLOOP: ::UINT = 29;
pub const D3DSIO_LABEL: ::UINT = 30;
pub const D3DSIO_DCL: ::UINT = 31;
pub const D3DSIO_POW: ::UINT = 32;
pub const D3DSIO_CRS: ::UINT = 33;
pub const D3DSIO_SGN: ::UINT = 34;
pub const D3DSIO_ABS: ::UINT = 35;
pub const D3DSIO_NRM: ::UINT = 36;
pub const D3DSIO_SINCOS: ::UINT = 37;
pub const D3DSIO_REP: ::UINT = 38;
pub const D3DSIO_ENDREP: ::UINT = 39;
pub const D3DSIO_IF: ::UINT = 40;
pub const D3DSIO_IFC: ::UINT = 41;
pub const D3DSIO_ELSE: ::UINT = 42;
pub const D3DSIO_ENDIF: ::UINT = 43;
pub const D3DSIO_BREAK: ::UINT = 44;
pub const D3DSIO_BREAKC: ::UINT = 45;
pub const D3DSIO_MOVA: ::UINT = 46;
pub const D3DSIO_DEFB: ::UINT = 47;
pub const D3DSIO_DEFI: ::UINT = 48;
pub const D3DSIO_TEXCOORD: ::UINT = 64;
pub const D3DSIO_TEXKILL: ::UINT = 65;
pub const D3DSIO_TEX: ::UINT = 66;
pub const D3DSIO_TEXBEM: ::UINT = 67;
pub const D3DSIO_TEXBEML: ::UINT = 68;
pub const D3DSIO_TEXREG2AR: ::UINT = 69;
pub const D3DSIO_TEXREG2GB: ::UINT = 70;
pub const D3DSIO_TEXM3x2PAD: ::UINT = 71;
pub const D3DSIO_TEXM3x2TEX: ::UINT = 72;
pub const D3DSIO_TEXM3x3PAD: ::UINT = 73;
pub const D3DSIO_TEXM3x3TEX: ::UINT = 74;
pub const D3DSIO_RESERVED0: ::UINT = 75;
pub const D3DSIO_TEXM3x3SPEC: ::UINT = 76;
pub const D3DSIO_TEXM3x3VSPEC: ::UINT = 77;
pub const D3DSIO_EXPP: ::UINT = 78;
pub const D3DSIO_LOGP: ::UINT = 79;
pub const D3DSIO_CND: ::UINT = 80;
pub const D3DSIO_DEF: ::UINT = 81;
pub const D3DSIO_TEXREG2RGB: ::UINT = 82;
pub const D3DSIO_TEXDP3TEX: ::UINT = 83;
pub const D3DSIO_TEXM3x2DEPTH: ::UINT = 84;
pub const D3DSIO_TEXDP3: ::UINT = 85;
pub const D3DSIO_TEXM3x3: ::UINT = 86;
pub const D3DSIO_TEXDEPTH: ::UINT = 87;
pub const D3DSIO_CMP: ::UINT = 88;
pub const D3DSIO_BEM: ::UINT = 89;
pub const D3DSIO_DP2ADD: ::UINT = 90;
pub const D3DSIO_DSX: ::UINT = 91;
pub const D3DSIO_DSY: ::UINT = 92;
pub const D3DSIO_TEXLDD: ::UINT = 93;
pub const D3DSIO_SETP: ::UINT = 94;
pub const D3DSIO_TEXLDL: ::UINT = 95;
pub const D3DSIO_BREAKP: ::UINT = 96;
pub const D3DSIO_PHASE: ::UINT = 65533;
pub const D3DSIO_COMMENT: ::UINT = 65534;
pub const D3DSIO_END: ::UINT = 65535;
pub const D3DSIO_FORCE_DWORD: ::UINT = 2147483647;

pub type D3DSHADER_COMPARISON = ::UINT;
pub const D3DSPC_RESERVED0: ::UINT = 0;
pub const D3DSPC_GT: ::UINT = 1;
pub const D3DSPC_EQ: ::UINT = 2;
pub const D3DSPC_GE: ::UINT = 3;
pub const D3DSPC_LT: ::UINT = 4;
pub const D3DSPC_NE: ::UINT = 5;
pub const D3DSPC_LE: ::UINT = 6;
pub const D3DSPC_RESERVED1: ::UINT = 7;

pub type D3DSAMPLER_TEXTURE_TYPE = ::UINT;
pub const D3DSTT_UNKNOWN: ::UINT = 0;
pub const D3DSTT_2D: ::UINT = 268435456;
pub const D3DSTT_CUBE: ::UINT = 402653184;
pub const D3DSTT_VOLUME: ::UINT = 536870912;
pub const D3DSTT_FORCE_DWORD: ::UINT = 2147483647;

pub type D3DSHADER_PARAM_REGISTER_TYPE = ::UINT;
pub const D3DSPR_TEMP: ::UINT = 0;
pub const D3DSPR_INPUT: ::UINT = 1;
pub const D3DSPR_CONST: ::UINT = 2;
pub const D3DSPR_ADDR: ::UINT = 3;
pub const D3DSPR_TEXTURE: ::UINT = 3;
pub const D3DSPR_RASTOUT: ::UINT = 4;
pub const D3DSPR_ATTROUT: ::UINT = 5;
pub const D3DSPR_TEXCRDOUT: ::UINT = 6;
pub const D3DSPR_OUTPUT: ::UINT = 6;
pub const D3DSPR_CONSTINT: ::UINT = 7;
pub const D3DSPR_COLOROUT: ::UINT = 8;
pub const D3DSPR_DEPTHOUT: ::UINT = 9;
pub const D3DSPR_SAMPLER: ::UINT = 10;
pub const D3DSPR_CONST2: ::UINT = 11;
pub const D3DSPR_CONST3: ::UINT = 12;
pub const D3DSPR_CONST4: ::UINT = 13;
pub const D3DSPR_CONSTBOOL: ::UINT = 14;
pub const D3DSPR_LOOP: ::UINT = 15;
pub const D3DSPR_TEMPFLOAT16: ::UINT = 16;
pub const D3DSPR_MISCTYPE: ::UINT = 17;
pub const D3DSPR_LABEL: ::UINT = 18;
pub const D3DSPR_PREDICATE: ::UINT = 19;
pub const D3DSPR_FORCE_DWORD: ::UINT = 2147483647;

pub type D3DSHADER_MISCTYPE_OFFSETS = ::UINT;
pub const D3DSMO_POSITION: ::UINT = 0;
pub const D3DSMO_FACE: ::UINT = 1;

pub type D3DVS_RASTOUT_OFFSETS = ::UINT;
pub const D3DSRO_POSITION: ::UINT = 0;
pub const D3DSRO_FOG: ::UINT = 1;
pub const D3DSRO_POINT_SIZE: ::UINT = 2;
pub const D3DSRO_FORCE_DWORD: ::UINT = 2147483647;

pub type D3DVS_ADDRESSMODE_TYPE = ::UINT;
pub const D3DVS_ADDRMODE_ABSOLUTE: ::UINT = 0;
pub const D3DVS_ADDRMODE_RELATIVE: ::UINT = 8192;
pub const D3DVS_ADDRMODE_FORCE_DWORD: ::UINT = 2147483647;

pub type D3DSHADER_ADDRESSMODE_TYPE = ::UINT;
pub const D3DSHADER_ADDRMODE_ABSOLUTE: ::UINT = 0;
pub const D3DSHADER_ADDRMODE_RELATIVE: ::UINT = 8192;
pub const D3DSHADER_ADDRMODE_FORCE_DWORD: ::UINT = 2147483647;

pub type D3DSHADER_PARAM_SRCMOD_TYPE = ::UINT;
pub const D3DSPSM_NONE: ::UINT = 0;
pub const D3DSPSM_NEG: ::UINT = 16777216;
pub const D3DSPSM_BIAS: ::UINT = 33554432;
pub const D3DSPSM_BIASNEG: ::UINT = 50331648;
pub const D3DSPSM_SIGN: ::UINT = 67108864;
pub const D3DSPSM_SIGNNEG: ::UINT = 83886080;
pub const D3DSPSM_COMP: ::UINT = 100663296;
pub const D3DSPSM_X2: ::UINT = 117440512;
pub const D3DSPSM_X2NEG: ::UINT = 134217728;
pub const D3DSPSM_DZ: ::UINT = 150994944;
pub const D3DSPSM_DW: ::UINT = 167772160;
pub const D3DSPSM_ABS: ::UINT = 184549376;
pub const D3DSPSM_ABSNEG: ::UINT = 201326592;
pub const D3DSPSM_NOT: ::UINT = 218103808;
pub const D3DSPSM_FORCE_DWORD: ::UINT = 2147483647;

pub type D3DBASISTYPE = ::UINT;
pub const D3DBASIS_BEZIER: ::UINT = 0;
pub const D3DBASIS_BSPLINE: ::UINT = 1;
pub const D3DBASIS_CATMULL_ROM: ::UINT = 2;
pub const D3DBASIS_FORCE_DWORD: ::UINT = 2147483647;

pub type D3DDEGREETYPE = ::UINT;
pub const D3DDEGREE_LINEAR: ::UINT = 1;
pub const D3DDEGREE_QUADRATIC: ::UINT = 2;
pub const D3DDEGREE_CUBIC: ::UINT = 3;
pub const D3DDEGREE_QUINTIC: ::UINT = 5;
pub const D3DDEGREE_FORCE_DWORD: ::UINT = 2147483647;

pub type D3DPATCHEDGESTYLE = ::UINT;
pub const D3DPATCHEDGE_DISCRETE: ::UINT = 0;
pub const D3DPATCHEDGE_CONTINUOUS: ::UINT = 1;
pub const D3DPATCHEDGE_FORCE_DWORD: ::UINT = 2147483647;

pub type D3DSTATEBLOCKTYPE = ::UINT;
pub const D3DSBT_ALL: ::UINT = 1;
pub const D3DSBT_PIXELSTATE: ::UINT = 2;
pub const D3DSBT_VERTEXSTATE: ::UINT = 3;
pub const D3DSBT_FORCE_DWORD: ::UINT = 2147483647;

pub type D3DVERTEXBLENDFLAGS = ::UINT;
pub const D3DVBF_DISABLE: ::UINT = 0;
pub const D3DVBF_1WEIGHTS: ::UINT = 1;
pub const D3DVBF_2WEIGHTS: ::UINT = 2;
pub const D3DVBF_3WEIGHTS: ::UINT = 3;
pub const D3DVBF_TWEENING: ::UINT = 255;
pub const D3DVBF_0WEIGHTS: ::UINT = 256;
pub const D3DVBF_FORCE_DWORD: ::UINT = 2147483647;

pub type D3DTEXTURETRANSFORMFLAGS = ::UINT;
pub const D3DTTFF_DISABLE: ::UINT = 0;
pub const D3DTTFF_COUNT1: ::UINT = 1;
pub const D3DTTFF_COUNT2: ::UINT = 2;
pub const D3DTTFF_COUNT3: ::UINT = 3;
pub const D3DTTFF_COUNT4: ::UINT = 4;
pub const D3DTTFF_PROJECTED: ::UINT = 256;
pub const D3DTTFF_FORCE_DWORD: ::UINT = 2147483647;

pub type D3DDEVTYPE = ::UINT;
pub const D3DDEVTYPE_HAL: ::UINT = 1;
pub const D3DDEVTYPE_REF: ::UINT = 2;
pub const D3DDEVTYPE_SW: ::UINT = 3;
pub const D3DDEVTYPE_NULLREF: ::UINT = 4;
pub const D3DDEVTYPE_FORCE_DWORD: ::UINT = 2147483647;

pub type D3DMULTISAMPLE_TYPE = ::UINT;
pub const D3DMULTISAMPLE_NONE: ::UINT = 0;
pub const D3DMULTISAMPLE_NONMASKABLE: ::UINT = 1;
pub const D3DMULTISAMPLE_2_SAMPLES: ::UINT = 2;
pub const D3DMULTISAMPLE_3_SAMPLES: ::UINT = 3;
pub const D3DMULTISAMPLE_4_SAMPLES: ::UINT = 4;
pub const D3DMULTISAMPLE_5_SAMPLES: ::UINT = 5;
pub const D3DMULTISAMPLE_6_SAMPLES: ::UINT = 6;
pub const D3DMULTISAMPLE_7_SAMPLES: ::UINT = 7;
pub const D3DMULTISAMPLE_8_SAMPLES: ::UINT = 8;
pub const D3DMULTISAMPLE_9_SAMPLES: ::UINT = 9;
pub const D3DMULTISAMPLE_10_SAMPLES: ::UINT = 10;
pub const D3DMULTISAMPLE_11_SAMPLES: ::UINT = 11;
pub const D3DMULTISAMPLE_12_SAMPLES: ::UINT = 12;
pub const D3DMULTISAMPLE_13_SAMPLES: ::UINT = 13;
pub const D3DMULTISAMPLE_14_SAMPLES: ::UINT = 14;
pub const D3DMULTISAMPLE_15_SAMPLES: ::UINT = 15;
pub const D3DMULTISAMPLE_16_SAMPLES: ::UINT = 16;
pub const D3DMULTISAMPLE_FORCE_DWORD: ::UINT = 2147483647;

pub type D3DFORMAT = ::UINT;
pub const D3DFMT_UNKNOWN: ::UINT = 0;
pub const D3DFMT_R8G8B8: ::UINT = 20;
pub const D3DFMT_A8R8G8B8: ::UINT = 21;
pub const D3DFMT_X8R8G8B8: ::UINT = 22;
pub const D3DFMT_R5G6B5: ::UINT = 23;
pub const D3DFMT_X1R5G5B5: ::UINT = 24;
pub const D3DFMT_A1R5G5B5: ::UINT = 25;
pub const D3DFMT_A4R4G4B4: ::UINT = 26;
pub const D3DFMT_R3G3B2: ::UINT = 27;
pub const D3DFMT_A8: ::UINT = 28;
pub const D3DFMT_A8R3G3B2: ::UINT = 29;
pub const D3DFMT_X4R4G4B4: ::UINT = 30;
pub const D3DFMT_A2B10G10R10: ::UINT = 31;
pub const D3DFMT_A8B8G8R8: ::UINT = 32;
pub const D3DFMT_X8B8G8R8: ::UINT = 33;
pub const D3DFMT_G16R16: ::UINT = 34;
pub const D3DFMT_A2R10G10B10: ::UINT = 35;
pub const D3DFMT_A16B16G16R16: ::UINT = 36;
pub const D3DFMT_A8P8: ::UINT = 40;
pub const D3DFMT_P8: ::UINT = 41;
pub const D3DFMT_L8: ::UINT = 50;
pub const D3DFMT_A8L8: ::UINT = 51;
pub const D3DFMT_A4L4: ::UINT = 52;
pub const D3DFMT_V8U8: ::UINT = 60;
pub const D3DFMT_L6V5U5: ::UINT = 61;
pub const D3DFMT_X8L8V8U8: ::UINT = 62;
pub const D3DFMT_Q8W8V8U8: ::UINT = 63;
pub const D3DFMT_V16U16: ::UINT = 64;
pub const D3DFMT_A2W10V10U10: ::UINT = 67;
pub const D3DFMT_UYVY: ::UINT = 1498831189;
pub const D3DFMT_R8G8_B8G8: ::UINT = 1195525970;
pub const D3DFMT_YUY2: ::UINT = 844715353;
pub const D3DFMT_G8R8_G8B8: ::UINT = 1111970375;
pub const D3DFMT_DXT1: ::UINT = 827611204;
pub const D3DFMT_DXT2: ::UINT = 844388420;
pub const D3DFMT_DXT3: ::UINT = 861165636;
pub const D3DFMT_DXT4: ::UINT = 877942852;
pub const D3DFMT_DXT5: ::UINT = 894720068;
pub const D3DFMT_D16_LOCKABLE: ::UINT = 70;
pub const D3DFMT_D32: ::UINT = 71;
pub const D3DFMT_D15S1: ::UINT = 73;
pub const D3DFMT_D24S8: ::UINT = 75;
pub const D3DFMT_D24X8: ::UINT = 77;
pub const D3DFMT_D24X4S4: ::UINT = 79;
pub const D3DFMT_D16: ::UINT = 80;
pub const D3DFMT_D32F_LOCKABLE: ::UINT = 82;
pub const D3DFMT_D24FS8: ::UINT = 83;
pub const D3DFMT_D32_LOCKABLE: ::UINT = 84;
pub const D3DFMT_S8_LOCKABLE: ::UINT = 85;
pub const D3DFMT_L16: ::UINT = 81;
pub const D3DFMT_VERTEXDATA: ::UINT = 100;
pub const D3DFMT_INDEX16: ::UINT = 101;
pub const D3DFMT_INDEX32: ::UINT = 102;
pub const D3DFMT_Q16W16V16U16: ::UINT = 110;
pub const D3DFMT_MULTI2_ARGB8: ::UINT = 827606349;
pub const D3DFMT_R16F: ::UINT = 111;
pub const D3DFMT_G16R16F: ::UINT = 112;
pub const D3DFMT_A16B16G16R16F: ::UINT = 113;
pub const D3DFMT_R32F: ::UINT = 114;
pub const D3DFMT_G32R32F: ::UINT = 115;
pub const D3DFMT_A32B32G32R32F: ::UINT = 116;
pub const D3DFMT_CxV8U8: ::UINT = 117;
pub const D3DFMT_A1: ::UINT = 118;
pub const D3DFMT_A2B10G10R10_XR_BIAS: ::UINT = 119;
pub const D3DFMT_BINARYBUFFER: ::UINT = 199;
pub const D3DFMT_FORCE_DWORD: ::UINT = 2147483647;

#[repr(C)]
#[derive(Copy)]
pub struct D3DDISPLAYMODE {
    pub Width: ::UINT,
    pub Height: ::UINT,
    pub RefreshRate: ::UINT,
    pub Format: D3DFORMAT,
}

#[repr(C)]
#[derive(Copy)]
pub struct D3DDEVICE_CREATION_PARAMETERS {
    pub AdapterOrdinal: ::UINT,
    pub DeviceType: D3DDEVTYPE,
    pub hFocusWindow: ::HWND,
    pub BehaviorFlags: ::DWORD,
}
pub type D3DSWAPEFFECT = ::UINT;
pub const D3DSWAPEFFECT_DISCARD: ::UINT = 1;
pub const D3DSWAPEFFECT_FLIP: ::UINT = 2;
pub const D3DSWAPEFFECT_COPY: ::UINT = 3;
pub const D3DSWAPEFFECT_OVERLAY: ::UINT = 4;
pub const D3DSWAPEFFECT_FLIPEX: ::UINT = 5;
pub const D3DSWAPEFFECT_FORCE_DWORD: ::UINT = 2147483647;

pub type D3DPOOL = ::UINT;
pub const D3DPOOL_DEFAULT: ::UINT = 0;
pub const D3DPOOL_MANAGED: ::UINT = 1;
pub const D3DPOOL_SYSTEMMEM: ::UINT = 2;
pub const D3DPOOL_SCRATCH: ::UINT = 3;
pub const D3DPOOL_FORCE_DWORD: ::UINT = 2147483647;

#[repr(C)]
#[derive(Copy)]
pub struct D3DPRESENT_PARAMETERS_ {
    pub BackBufferWidth: ::UINT,
    pub BackBufferHeight: ::UINT,
    pub BackBufferFormat: D3DFORMAT,
    pub BackBufferCount: ::UINT,
    pub MultiSampleType: D3DMULTISAMPLE_TYPE,
    pub MultiSampleQuality: ::DWORD,
    pub SwapEffect: D3DSWAPEFFECT,
    pub hDeviceWindow: ::HWND,
    pub Windowed: ::BOOL,
    pub EnableAutoDepthStencil: ::BOOL,
    pub AutoDepthStencilFormat: D3DFORMAT,
    pub Flags: ::DWORD,
    pub FullScreen_RefreshRateInHz: ::UINT,
    pub PresentationInterval: ::UINT,
}
pub type D3DPRESENT_PARAMETERS = D3DPRESENT_PARAMETERS_;
#[repr(C)]
#[derive(Copy)]
pub struct D3DGAMMARAMP {
    pub red: [::WORD; 256usize],
    pub green: [::WORD; 256usize],
    pub blue: [::WORD; 256usize],
}

pub type D3DBACKBUFFER_TYPE = ::UINT;
pub const D3DBACKBUFFER_TYPE_MONO: ::UINT = 0;
pub const D3DBACKBUFFER_TYPE_LEFT: ::UINT = 1;
pub const D3DBACKBUFFER_TYPE_RIGHT: ::UINT = 2;
pub const D3DBACKBUFFER_TYPE_FORCE_DWORD: ::UINT = 2147483647;

pub type D3DRESOURCETYPE = ::UINT;
pub const D3DRTYPE_SURFACE: ::UINT = 1;
pub const D3DRTYPE_VOLUME: ::UINT = 2;
pub const D3DRTYPE_TEXTURE: ::UINT = 3;
pub const D3DRTYPE_VOLUMETEXTURE: ::UINT = 4;
pub const D3DRTYPE_CUBETEXTURE: ::UINT = 5;
pub const D3DRTYPE_VERTEXBUFFER: ::UINT = 6;
pub const D3DRTYPE_INDEXBUFFER: ::UINT = 7;
pub const D3DRTYPE_FORCE_DWORD: ::UINT = 2147483647;

pub type D3DCUBEMAP_FACES = ::UINT;
pub const D3DCUBEMAP_FACE_POSITIVE_X: ::UINT = 0;
pub const D3DCUBEMAP_FACE_NEGATIVE_X: ::UINT = 1;
pub const D3DCUBEMAP_FACE_POSITIVE_Y: ::UINT = 2;
pub const D3DCUBEMAP_FACE_NEGATIVE_Y: ::UINT = 3;
pub const D3DCUBEMAP_FACE_POSITIVE_Z: ::UINT = 4;
pub const D3DCUBEMAP_FACE_NEGATIVE_Z: ::UINT = 5;
pub const D3DCUBEMAP_FACE_FORCE_DWORD: ::UINT = 2147483647;

#[repr(C)]
#[derive(Copy)]
pub struct D3DVERTEXBUFFER_DESC {
    pub Format: D3DFORMAT,
    pub Type: D3DRESOURCETYPE,
    pub Usage: ::DWORD,
    pub Pool: D3DPOOL,
    pub Size: ::UINT,
    pub FVF: ::DWORD,
}

#[repr(C)]
#[derive(Copy)]
pub struct D3DINDEXBUFFER_DESC {
    pub Format: D3DFORMAT,
    pub Type: D3DRESOURCETYPE,
    pub Usage: ::DWORD,
    pub Pool: D3DPOOL,
    pub Size: ::UINT,
}

#[repr(C)]
#[derive(Copy)]
pub struct D3DSURFACE_DESC {
    pub Format: D3DFORMAT,
    pub Type: D3DRESOURCETYPE,
    pub Usage: ::DWORD,
    pub Pool: D3DPOOL,
    pub MultiSampleType: D3DMULTISAMPLE_TYPE,
    pub MultiSampleQuality: ::DWORD,
    pub Width: ::UINT,
    pub Height: ::UINT,
}

#[repr(C)]
#[derive(Copy)]
pub struct D3DVOLUME_DESC {
    pub Format: D3DFORMAT,
    pub Type: D3DRESOURCETYPE,
    pub Usage: ::DWORD,
    pub Pool: D3DPOOL,
    pub Width: ::UINT,
    pub Height: ::UINT,
    pub Depth: ::UINT,
}

#[repr(C)]
#[derive(Copy)]
pub struct D3DLOCKED_RECT {
    pub Pitch: ::INT,
    pub pBits: *mut ::libc::c_void,
}

#[repr(C)]
#[derive(Copy)]
pub struct D3DBOX {
    pub Left: ::UINT,
    pub Top: ::UINT,
    pub Right: ::UINT,
    pub Bottom: ::UINT,
    pub Front: ::UINT,
    pub Back: ::UINT,
}

#[repr(C)]
#[derive(Copy)]
pub struct D3DLOCKED_BOX {
    pub RowPitch: ::INT,
    pub SlicePitch: ::INT,
    pub pBits: *mut ::libc::c_void,
}

#[repr(C)]
#[derive(Copy)]
pub struct D3DRANGE {
    pub Offset: ::UINT,
    pub Size: ::UINT,
}

#[repr(C)]
#[derive(Copy)]
pub struct D3DRECTPATCH_INFO {
    pub StartVertexOffsetWidth: ::UINT,
    pub StartVertexOffsetHeight: ::UINT,
    pub Width: ::UINT,
    pub Height: ::UINT,
    pub Stride: ::UINT,
    pub Basis: D3DBASISTYPE,
    pub Degree: D3DDEGREETYPE,
}

#[repr(C)]
#[derive(Copy)]
pub struct D3DTRIPATCH_INFO {
    pub StartVertexOffset: ::UINT,
    pub NumVertices: ::UINT,
    pub Basis: D3DBASISTYPE,
    pub Degree: D3DDEGREETYPE,
}

#[repr(C)]
#[derive(Copy)]
pub struct D3DADAPTER_IDENTIFIER9 {
    pub Driver: [::libc::c_char; 512usize],
    pub Description: [::libc::c_char; 512usize],
    pub DeviceName: [::libc::c_char; 32usize],
    pub DriverVersion: ::LARGE_INTEGER,
    pub VendorId: ::DWORD,
    pub DeviceId: ::DWORD,
    pub SubSysId: ::DWORD,
    pub Revision: ::DWORD,
    pub DeviceIdentifier: ::GUID,
    pub WHQLLevel: ::DWORD,
}

#[repr(C)]
#[derive(Copy)]
pub struct D3DRASTER_STATUS {
    pub InVBlank: ::BOOL,
    pub ScanLine: ::UINT,
}

pub type D3DDEBUGMONITORTOKENS = ::UINT;
pub const D3DDMT_ENABLE: ::UINT = 0;
pub const D3DDMT_DISABLE: ::UINT = 1;
pub const D3DDMT_FORCE_DWORD: ::UINT = 2147483647;

pub type D3DQUERYTYPE = ::UINT;
pub const D3DQUERYTYPE_VCACHE: ::UINT = 4;
pub const D3DQUERYTYPE_RESOURCEMANAGER: ::UINT = 5;
pub const D3DQUERYTYPE_VERTEXSTATS: ::UINT = 6;
pub const D3DQUERYTYPE_EVENT: ::UINT = 8;
pub const D3DQUERYTYPE_OCCLUSION: ::UINT = 9;
pub const D3DQUERYTYPE_TIMESTAMP: ::UINT = 10;
pub const D3DQUERYTYPE_TIMESTAMPDISJOINT: ::UINT = 11;
pub const D3DQUERYTYPE_TIMESTAMPFREQ: ::UINT = 12;
pub const D3DQUERYTYPE_PIPELINETIMINGS: ::UINT = 13;
pub const D3DQUERYTYPE_INTERFACETIMINGS: ::UINT = 14;
pub const D3DQUERYTYPE_VERTEXTIMINGS: ::UINT = 15;
pub const D3DQUERYTYPE_PIXELTIMINGS: ::UINT = 16;
pub const D3DQUERYTYPE_BANDWIDTHTIMINGS: ::UINT = 17;
pub const D3DQUERYTYPE_CACHEUTILIZATION: ::UINT = 18;
pub const D3DQUERYTYPE_MEMORYPRESSURE: ::UINT = 19;

#[repr(C)]
#[derive(Copy)]
pub struct D3DRESOURCESTATS {
    pub bThrashing: ::BOOL,
    pub ApproxBytesDownloaded: ::DWORD,
    pub NumEvicts: ::DWORD,
    pub NumVidCreates: ::DWORD,
    pub LastPri: ::DWORD,
    pub NumUsed: ::DWORD,
    pub NumUsedInVidMem: ::DWORD,
    pub WorkingSet: ::DWORD,
    pub WorkingSetBytes: ::DWORD,
    pub TotalManaged: ::DWORD,
    pub TotalBytes: ::DWORD,
}

#[repr(C)]
#[derive(Copy)]
pub struct D3DDEVINFO_RESOURCEMANAGER {
    pub stats: [D3DRESOURCESTATS; 8usize],
}

pub type LPD3DDEVINFO_RESOURCEMANAGER = *mut D3DDEVINFO_RESOURCEMANAGER;
#[repr(C)]
#[derive(Copy)]
pub struct D3DDEVINFO_D3DVERTEXSTATS {
    pub NumRenderedTriangles: ::DWORD,
    pub NumExtraClippingTriangles: ::DWORD,
}

pub type LPD3DDEVINFO_D3DVERTEXSTATS = *mut D3DDEVINFO_D3DVERTEXSTATS;
#[repr(C)]
#[derive(Copy)]
pub struct D3DDEVINFO_VCACHE {
    pub Pattern: ::DWORD,
    pub OptMethod: ::DWORD,
    pub CacheSize: ::DWORD,
    pub MagicNumber: ::DWORD,
}

pub type LPD3DDEVINFO_VCACHE = *mut D3DDEVINFO_VCACHE;

#[repr(C)]
#[derive(Copy)]
pub struct D3DDEVINFO_D3D9PIPELINETIMINGS {
    pub VertexProcessingTimePercent: ::FLOAT,
    pub PixelProcessingTimePercent: ::FLOAT,
    pub OtherGPUProcessingTimePercent: ::FLOAT,
    pub GPUIdleTimePercent: ::FLOAT,
}

#[repr(C)]
#[derive(Copy)]
pub struct D3DDEVINFO_D3D9INTERFACETIMINGS {
    pub WaitingForGPUToUseApplicationResourceTimePercent: ::FLOAT,
    pub WaitingForGPUToAcceptMoreCommandsTimePercent: ::FLOAT,
    pub WaitingForGPUToStayWithinLatencyTimePercent: ::FLOAT,
    pub WaitingForGPUExclusiveResourceTimePercent: ::FLOAT,
    pub WaitingForGPUOtherTimePercent: ::FLOAT,
}

#[repr(C)]
#[derive(Copy)]
pub struct D3DDEVINFO_D3D9STAGETIMINGS {
    pub MemoryProcessingPercent: ::FLOAT,
    pub ComputationProcessingPercent: ::FLOAT,
}

#[repr(C)]
#[derive(Copy)]
pub struct D3DDEVINFO_D3D9BANDWIDTHTIMINGS {
    pub MaxBandwidthUtilized: ::FLOAT,
    pub FrontEndUploadMemoryUtilizedPercent: ::FLOAT,
    pub VertexRateUtilizedPercent: ::FLOAT,
    pub TriangleSetupRateUtilizedPercent: ::FLOAT,
    pub FillRateUtilizedPercent: ::FLOAT,
}

#[repr(C)]
#[derive(Copy)]
pub struct D3DDEVINFO_D3D9CACHEUTILIZATION {
    pub TextureCacheHitRate: ::FLOAT,
    pub PostTransformVertexCacheHitRate: ::FLOAT,
}

#[repr(C)]
#[derive(Copy)]
pub struct D3DMEMORYPRESSURE {
    pub BytesEvictedFromProcess: ::UINT64,
    pub SizeOfInefficientAllocation: ::UINT64,
    pub LevelOfEfficiency: ::DWORD,
}

pub type D3DCOMPOSERECTSOP = ::UINT;
pub const D3DCOMPOSERECTS_COPY: ::UINT = 1;
pub const D3DCOMPOSERECTS_OR: ::UINT = 2;
pub const D3DCOMPOSERECTS_AND: ::UINT = 3;
pub const D3DCOMPOSERECTS_NEG: ::UINT = 4;
pub const D3DCOMPOSERECTS_FORCE_DWORD: ::UINT = 2147483647;

#[repr(C)]
#[derive(Copy)]
pub struct D3DCOMPOSERECTDESC {
    pub X: ::USHORT,
    pub Y: ::USHORT,
    pub Width: ::USHORT,
    pub Height: ::USHORT,
}

#[repr(C)]
#[derive(Copy)]
pub struct D3DCOMPOSERECTDESTINATION {
    pub SrcRectIndex: ::USHORT,
    pub Reserved: ::USHORT,
    pub X: ::SHORT,
    pub Y: ::SHORT,
}

#[repr(C)]
#[derive(Copy)]
pub struct D3DPRESENTSTATS {
    pub PresentCount: ::UINT,
    pub PresentRefreshCount: ::UINT,
    pub SyncRefreshCount: ::UINT,
    pub SyncQPCTime: ::LARGE_INTEGER,
    pub SyncGPUTime: ::LARGE_INTEGER,
}

pub type D3DSCANLINEORDERING = ::UINT;
pub const D3DSCANLINEORDERING_UNKNOWN: ::UINT = 0;
pub const D3DSCANLINEORDERING_PROGRESSIVE: ::UINT = 1;
pub const D3DSCANLINEORDERING_INTERLACED: ::UINT = 2;

#[repr(C)]
#[derive(Copy)]
pub struct D3DDISPLAYMODEEX {
    pub Size: ::UINT,
    pub Width: ::UINT,
    pub Height: ::UINT,
    pub RefreshRate: ::UINT,
    pub Format: D3DFORMAT,
    pub ScanLineOrdering: D3DSCANLINEORDERING,
}

#[repr(C)]
#[derive(Copy)]
pub struct D3DDISPLAYMODEFILTER {
    pub Size: ::UINT,
    pub Format: D3DFORMAT,
    pub ScanLineOrdering: D3DSCANLINEORDERING,
}

pub type D3DDISPLAYROTATION = ::UINT;
pub const D3DDISPLAYROTATION_IDENTITY: ::UINT = 1;
pub const D3DDISPLAYROTATION_90: ::UINT = 2;
pub const D3DDISPLAYROTATION_180: ::UINT = 3;
pub const D3DDISPLAYROTATION_270: ::UINT = 4;

#[repr(C)]
#[derive(Copy)]
pub struct D3D_OMAC {
    pub Omac: [::BYTE; 16usize],
}

pub type D3DAUTHENTICATEDCHANNELTYPE = ::UINT;
pub const D3DAUTHENTICATEDCHANNEL_D3D9: ::UINT = 1;
pub const D3DAUTHENTICATEDCHANNEL_DRIVER_SOFTWARE: ::UINT = 2;
pub const D3DAUTHENTICATEDCHANNEL_DRIVER_HARDWARE: ::UINT = 3;

#[repr(C)]
#[derive(Copy)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERY_INPUT {
    pub QueryType: ::GUID,
    pub hChannel: ::HANDLE,
    pub SequenceNumber: ::UINT,
}

#[repr(C)]
#[derive(Copy)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT {
    pub omac: D3D_OMAC,
    pub QueryType: ::GUID,
    pub hChannel: ::HANDLE,
    pub SequenceNumber: ::UINT,
    pub ReturnCode: ::HRESULT,
}

#[repr(C)]
#[derive(Copy)]
pub struct D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS {
    pub _bindgen_data_1_: [u32; 1usize],
}
impl D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS {
    pub unsafe fn Value(&mut self) -> *mut ::UINT {
        transmute(&self._bindgen_data_1_)
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYPROTECTION_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub ProtectionFlags: D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS,
}


#[repr(C)]
#[derive(Copy)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYCHANNELTYPE_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub ChannelType: D3DAUTHENTICATEDCHANNELTYPE,
}

#[repr(C)]
#[derive(Copy)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYDEVICEHANDLE_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub DeviceHandle: ::HANDLE,
}

#[repr(C)]
#[derive(Copy)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYCRYPTOSESSION_INPUT {
    pub Input: D3DAUTHENTICATEDCHANNEL_QUERY_INPUT,
    pub DXVA2DecodeHandle: ::HANDLE,
}

#[repr(C)]
#[derive(Copy)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYCRYPTOSESSION_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub DXVA2DecodeHandle: ::HANDLE,
    pub CryptoSessionHandle: ::HANDLE,
    pub DeviceHandle: ::HANDLE,
}


#[repr(C)]
#[derive(Copy)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESSCOUNT_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub NumRestrictedSharedResourceProcesses: ::UINT,
}


#[repr(C)]
#[derive(Copy)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESS_INPUT {
    pub Input: D3DAUTHENTICATEDCHANNEL_QUERY_INPUT,
    pub ProcessIndex: ::UINT,
}


pub type D3DAUTHENTICATEDCHANNEL_PROCESSIDENTIFIERTYPE = ::UINT;
pub const PROCESSIDTYPE_UNKNOWN: ::UINT = 0;
pub const PROCESSIDTYPE_DWM: ::UINT = 1;
pub const PROCESSIDTYPE_HANDLE: ::UINT = 2;

#[repr(C)]
#[derive(Copy)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESS_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub ProcessIndex: ::UINT,
    pub ProcessIdentifer: D3DAUTHENTICATEDCHANNEL_PROCESSIDENTIFIERTYPE,
    pub ProcessHandle: ::HANDLE,
}


#[repr(C)]
#[derive(Copy)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYUNRESTRICTEDPROTECTEDSHAREDRESOURCECOUNT_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub NumUnrestrictedProtectedSharedResources: ::UINT,
}


#[repr(C)]
#[derive(Copy)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTIDCOUNT_INPUT {
    pub Input: D3DAUTHENTICATEDCHANNEL_QUERY_INPUT,
    pub DeviceHandle: ::HANDLE,
    pub CryptoSessionHandle: ::HANDLE,
}

#[repr(C)]
#[derive(Copy)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTIDCOUNT_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub DeviceHandle: ::HANDLE,
    pub CryptoSessionHandle: ::HANDLE,
    pub NumOutputIDs: ::UINT,
}


#[repr(C)]
#[derive(Copy)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTID_INPUT {
    pub Input: D3DAUTHENTICATEDCHANNEL_QUERY_INPUT,
    pub DeviceHandle: ::HANDLE,
    pub CryptoSessionHandle: ::HANDLE,
    pub OutputIDIndex: ::UINT,
}

#[repr(C)]
#[derive(Copy)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTID_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub DeviceHandle: ::HANDLE,
    pub CryptoSessionHandle: ::HANDLE,
    pub OutputIDIndex: ::UINT,
    pub OutputID: ::UINT64,
}

pub type D3DBUSTYPE = ::UINT;
pub const D3DBUSTYPE_OTHER: ::UINT = 0;
pub const D3DBUSTYPE_PCI: ::UINT = 1;
pub const D3DBUSTYPE_PCIX: ::UINT = 2;
pub const D3DBUSTYPE_PCIEXPRESS: ::UINT = 3;
pub const D3DBUSTYPE_AGP: ::UINT = 4;
pub const D3DBUSIMPL_MODIFIER_INSIDE_OF_CHIPSET: ::UINT = 65536;
pub const D3DBUSIMPL_MODIFIER_TRACKS_ON_MOTHER_BOARD_TO_CHIP: ::UINT = 131072;
pub const D3DBUSIMPL_MODIFIER_TRACKS_ON_MOTHER_BOARD_TO_SOCKET: ::UINT = 196608;
pub const D3DBUSIMPL_MODIFIER_DAUGHTER_BOARD_CONNECTOR: ::UINT = 262144;
pub const D3DBUSIMPL_MODIFIER_DAUGHTER_BOARD_CONNECTOR_INSIDE_OF_NUAE: ::UINT = 327680;
pub const D3DBUSIMPL_MODIFIER_NON_STANDARD: ::UINT = -2147483648;

#[repr(C)]
#[derive(Copy)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYINFOBUSTYPE_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub BusType: D3DBUSTYPE,
    pub bAccessibleInContiguousBlocks: ::BOOL,
    pub bAccessibleInNonContiguousBlocks: ::BOOL,
}

#[repr(C)]
#[derive(Copy)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUIDCOUNT_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub NumEncryptionGuids: ::UINT,
}


#[repr(C)]
#[derive(Copy)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUID_INPUT {
    pub Input: D3DAUTHENTICATEDCHANNEL_QUERY_INPUT,
    pub EncryptionGuidIndex: ::UINT,
}


#[repr(C)]
#[derive(Copy)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUID_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub EncryptionGuidIndex: ::UINT,
    pub EncryptionGuid: ::GUID,
}


#[repr(C)]
#[derive(Copy)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYUNCOMPRESSEDENCRYPTIONLEVEL_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub EncryptionGuid: ::GUID,
}


#[repr(C)]
#[derive(Copy)]
pub struct D3DAUTHENTICATEDCHANNEL_CONFIGURE_INPUT {
    pub omac: D3D_OMAC,
    pub ConfigureType: ::GUID,
    pub hChannel: ::HANDLE,
    pub SequenceNumber: ::UINT,
}

#[repr(C)]
#[derive(Copy)]
pub struct D3DAUTHENTICATEDCHANNEL_CONFIGURE_OUTPUT {
    pub omac: D3D_OMAC,
    pub ConfigureType: ::GUID,
    pub hChannel: ::HANDLE,
    pub SequenceNumber: ::UINT,
    pub ReturnCode: ::HRESULT,
}

#[repr(C)]
#[derive(Copy)]
pub struct D3DAUTHENTICATEDCHANNEL_CONFIGUREINITIALIZE {
    pub Parameters: D3DAUTHENTICATEDCHANNEL_CONFIGURE_INPUT,
    pub StartSequenceQuery: ::UINT,
    pub StartSequenceConfigure: ::UINT,
}

#[repr(C)]
#[derive(Copy)]
pub struct D3DAUTHENTICATEDCHANNEL_CONFIGUREPROTECTION {
    pub Parameters: D3DAUTHENTICATEDCHANNEL_CONFIGURE_INPUT,
    pub Protections: D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS,
}

#[repr(C)]
#[derive(Copy)]
pub struct D3DAUTHENTICATEDCHANNEL_CONFIGURECRYPTOSESSION {
    pub Parameters: D3DAUTHENTICATEDCHANNEL_CONFIGURE_INPUT,
    pub DXVA2DecodeHandle: ::HANDLE,
    pub CryptoSessionHandle: ::HANDLE,
    pub DeviceHandle: ::HANDLE,
}

#[repr(C)]
#[derive(Copy)]
pub struct D3DAUTHENTICATEDCHANNEL_CONFIGURESHAREDRESOURCE {
    pub Parameters: D3DAUTHENTICATEDCHANNEL_CONFIGURE_INPUT,
    pub ProcessIdentiferType: D3DAUTHENTICATEDCHANNEL_PROCESSIDENTIFIERTYPE,
    pub ProcessHandle: ::HANDLE,
    pub AllowAccess: ::BOOL,
}

#[repr(C)]
#[derive(Copy)]
pub struct D3DAUTHENTICATEDCHANNEL_CONFIGUREUNCOMPRESSEDENCRYPTION {
    pub Parameters: D3DAUTHENTICATEDCHANNEL_CONFIGURE_INPUT,
    pub EncryptionGuid: ::GUID,
}


#[repr(C)]
#[derive(Copy)]
pub struct D3DENCRYPTED_BLOCK_INFO {
    pub NumEncryptedBytesAtBeginning: ::UINT,
    pub NumBytesInSkipPattern: ::UINT,
    pub NumBytesInEncryptPattern: ::UINT,
}

#[repr(C)]
#[derive(Copy)]
pub struct D3DAES_CTR_IV {
    pub IV: ::UINT64,
    pub Count: ::UINT64,
}

extern "C" {
    pub static D3DAUTHENTICATEDQUERY_PROTECTION: ::GUID;
    pub static D3DAUTHENTICATEDQUERY_CHANNELTYPE: ::GUID;
    pub static D3DAUTHENTICATEDQUERY_DEVICEHANDLE: ::GUID;
    pub static D3DAUTHENTICATEDQUERY_CRYPTOSESSION: ::GUID;
    pub static D3DAUTHENTICATEDQUERY_RESTRICTEDSHAREDRESOURCEPROCESSCOUNT:
               ::GUID;
    pub static D3DAUTHENTICATEDQUERY_RESTRICTEDSHAREDRESOURCEPROCESS: ::GUID;
    pub static D3DAUTHENTICATEDQUERY_UNRESTRICTEDPROTECTEDSHAREDRESOURCECOUNT:
               ::GUID;
    pub static D3DAUTHENTICATEDQUERY_OUTPUTIDCOUNT: ::GUID;
    pub static D3DAUTHENTICATEDQUERY_OUTPUTID: ::GUID;
    pub static D3DAUTHENTICATEDQUERY_ACCESSIBILITYATTRIBUTES: ::GUID;
    pub static D3DAUTHENTICATEDQUERY_ENCRYPTIONWHENACCESSIBLEGUIDCOUNT: ::GUID;
    pub static D3DAUTHENTICATEDQUERY_ENCRYPTIONWHENACCESSIBLEGUID: ::GUID;
    pub static D3DAUTHENTICATEDQUERY_CURRENTENCRYPTIONWHENACCESSIBLE: ::GUID;
    pub static D3DAUTHENTICATEDCONFIGURE_INITIALIZE: ::GUID;
    pub static D3DAUTHENTICATEDCONFIGURE_PROTECTION: ::GUID;
    pub static D3DAUTHENTICATEDCONFIGURE_CRYPTOSESSION: ::GUID;
    pub static D3DAUTHENTICATEDCONFIGURE_SHAREDRESOURCE: ::GUID;
    pub static D3DAUTHENTICATEDCONFIGURE_ENCRYPTIONWHENACCESSIBLE: ::GUID;
}

// Copyright © 2018 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::basetsd::UINT_PTR;
use shared::guiddef::{CLSID, GUID};
use shared::minwindef::{BYTE, DWORD, INT, UINT, ULONG, WORD};
use shared::winerror::HRESULT;
use shared::wtypes::PROPID;
use um::gdipluspixelformats::PixelFormat;
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::{VOID, WCHAR};
DEFINE_GUID!{ImageFormatUndefined,
    0xb96b3ca9, 0x0728, 0x11d3, 0x9d, 0x7b, 0x00, 0x00, 0xf8, 0x1e, 0xf3, 0x2e}
DEFINE_GUID!{ImageFormatMemoryBMP,
    0xb96b3caa, 0x0728, 0x11d3, 0x9d, 0x7b, 0x00, 0x00, 0xf8, 0x1e, 0xf3, 0x2e}
DEFINE_GUID!{ImageFormatBMP,
    0xb96b3cab, 0x0728, 0x11d3, 0x9d, 0x7b, 0x00, 0x00, 0xf8, 0x1e, 0xf3, 0x2e}
DEFINE_GUID!{ImageFormatEMF,
    0xb96b3cac, 0x0728, 0x11d3, 0x9d, 0x7b, 0x00, 0x00, 0xf8, 0x1e, 0xf3, 0x2e}
DEFINE_GUID!{ImageFormatWMF,
    0xb96b3cad, 0x0728, 0x11d3, 0x9d, 0x7b, 0x00, 0x00, 0xf8, 0x1e, 0xf3, 0x2e}
DEFINE_GUID!{ImageFormatJPEG,
    0xb96b3cae, 0x0728, 0x11d3, 0x9d, 0x7b, 0x00, 0x00, 0xf8, 0x1e, 0xf3, 0x2e}
DEFINE_GUID!{ImageFormatPNG,
    0xb96b3caf, 0x0728, 0x11d3, 0x9d, 0x7b, 0x00, 0x00, 0xf8, 0x1e, 0xf3, 0x2e}
DEFINE_GUID!{ImageFormatGIF,
    0xb96b3cb0, 0x0728, 0x11d3, 0x9d, 0x7b, 0x00, 0x00, 0xf8, 0x1e, 0xf3, 0x2e}
DEFINE_GUID!{ImageFormatTIFF,
    0xb96b3cb1, 0x0728, 0x11d3, 0x9d, 0x7b, 0x00, 0x00, 0xf8, 0x1e, 0xf3, 0x2e}
DEFINE_GUID!{ImageFormatEXIF,
    0xb96b3cb2, 0x0728, 0x11d3, 0x9d, 0x7b, 0x00, 0x00, 0xf8, 0x1e, 0xf3, 0x2e}
DEFINE_GUID!{ImageFormatIcon,
    0xb96b3cb5, 0x0728, 0x11d3, 0x9d, 0x7b, 0x00, 0x00, 0xf8, 0x1e, 0xf3, 0x2e}
DEFINE_GUID!{FrameDimensionTime,
    0x6aedbd6d, 0x3fb5, 0x418a, 0x83, 0xa6, 0x7f, 0x45, 0x22, 0x9d, 0xc8, 0x72}
DEFINE_GUID!{FrameDimensionResolution,
    0x84236f7b, 0x3bd3, 0x428f, 0x8d, 0xab, 0x4e, 0xa1, 0x43, 0x9c, 0xa3, 0x15}
DEFINE_GUID!{FrameDimensionPage,
    0x7462dc86, 0x6180, 0x4c7e, 0x8e, 0x3f, 0xee, 0x73, 0x33, 0xa7, 0xa4, 0x83}
DEFINE_GUID!{FormatIDImageInformation,
    0xe5836cbe, 0x5eef, 0x4f1d, 0xac, 0xde, 0xae, 0x4c, 0x43, 0xb6, 0x08, 0xce}
DEFINE_GUID!{FormatIDJpegAppHeaders,
    0x1c4afdcd, 0x6177, 0x43cf, 0xab, 0xc7, 0x5f, 0x51, 0xaf, 0x39, 0xee, 0x85}
DEFINE_GUID!{EncoderCompression,
    0xe09d739d, 0xccd4, 0x44ee, 0x8e, 0xba, 0x3f, 0xbf, 0x8b, 0xe4, 0xfc, 0x58}
DEFINE_GUID!{EncoderColorDepth,
    0x66087055, 0xad66, 0x4c7c, 0x9a, 0x18, 0x38, 0xa2, 0x31, 0x0b, 0x83, 0x37}
DEFINE_GUID!{EncoderScanMethod,
    0x3a4e2661, 0x3109, 0x4e56, 0x85, 0x36, 0x42, 0xc1, 0x56, 0xe7, 0xdc, 0xfa}
DEFINE_GUID!{EncoderVersion,
    0x24d18c76, 0x814a, 0x41a4, 0xbf, 0x53, 0x1c, 0x21, 0x9c, 0xcc, 0xf7, 0x97}
DEFINE_GUID!{EncoderRenderMethod,
    0x6d42c53a, 0x229a, 0x4825, 0x8b, 0xb7, 0x5c, 0x99, 0xe2, 0xb9, 0xa8, 0xb8}
DEFINE_GUID!{EncoderQuality,
    0x1d5be4b5, 0xfa4a, 0x452d, 0x9c, 0xdd, 0x5d, 0xb3, 0x51, 0x05, 0xe7, 0xeb}
DEFINE_GUID!{EncoderTransformation,
    0x8d0eb2d1, 0xa58e, 0x4ea8, 0xaa, 0x14, 0x10, 0x80, 0x74, 0xb7, 0xb6, 0xf9}
DEFINE_GUID!{EncoderLuminanceTable,
    0xedb33bce, 0x0266, 0x4a77, 0xb9, 0x04, 0x27, 0x21, 0x60, 0x99, 0xe7, 0x17}
DEFINE_GUID!{EncoderChrominanceTable,
    0xf2e455dc, 0x09b3, 0x4316, 0x82, 0x60, 0x67, 0x6a, 0xda, 0x32, 0x48, 0x1c}
DEFINE_GUID!{EncoderSaveFlag,
    0x292266fc, 0xac40, 0x47bf, 0x8c, 0xfc, 0xa8, 0x5b, 0x89, 0xa6, 0x55, 0xde}
DEFINE_GUID!{EncoderColorSpace,
    0xae7a62a0, 0xee2c, 0x49d8, 0x9d, 0x7, 0x1b, 0xa8, 0xa9, 0x27, 0x59, 0x6e}
DEFINE_GUID!{EncoderImageItems,
    0x63875e13, 0x1f1d, 0x45ab, 0x91, 0x95, 0xa2, 0x9b, 0x60, 0x66, 0xa6, 0x50}
DEFINE_GUID!{EncoderSaveAsCMYK,
    0xa219bbc9, 0xa9d, 0x4005, 0xa3, 0xee, 0x3a, 0x42, 0x1b, 0x8b, 0xb0, 0x6c}
DEFINE_GUID!{CodecIImageBytes,
    0x025d1823, 0x6c7d, 0x447b, 0xbb, 0xdb, 0xa3, 0xcb, 0xc3, 0xdf, 0xa2, 0xfc}
RIDL!{#[uuid(0x025D1823, 0x6C7D, 0x447B, 0xBB, 0xDB, 0xA3, 0xCB, 0xC3, 0xDF, 0xA2, 0xFC)]
interface IImageBytes(IImageBytesVtbl): IUnknown(IUnknownVtbl) {
    fn CountBytes(
        pcd: *mut UINT,
    ) -> HRESULT,
    fn LockBytes(
        cb: UINT,
        ulOffset: ULONG,
        ppvBytes: *mut *const VOID,
    ) -> HRESULT,
    fn UnlockBytes(
        pvBytes: *const VOID,
        cb: UINT,
        ulOffset: ULONG,
    ) -> HRESULT,
}}
STRUCT!{struct ImageCodecInfo {
    Clsid: CLSID,
    FormatID: GUID,
    CodecName: *const WCHAR,
    DllName: *const WCHAR,
    FormatDescription: *const WCHAR,
    FilenameExtension: *const WCHAR,
    MimeType: *const WCHAR,
    Flags: DWORD,
    Version: DWORD,
    SigCount: DWORD,
    SigSize: DWORD,
    SigPattern: *const BYTE,
    SigMask: *const BYTE,
}}
ENUM!{enum ImageCodecFlags {
    ImageCodecFlagsEncoder = 0x00000001,
    ImageCodecFlagsDecoder = 0x00000002,
    ImageCodecFlagsSupportBitmap = 0x00000004,
    ImageCodecFlagsSupportVector = 0x00000008,
    ImageCodecFlagsSeekableEncode = 0x00000010,
    ImageCodecFlagsBlockingDecode = 0x00000020,
    ImageCodecFlagsBuiltin = 0x00010000,
    ImageCodecFlagsSystem = 0x00020000,
    ImageCodecFlagsUser = 0x00040000,
}}
ENUM!{enum ImageLockMode {
    ImageLockModeRead = 0x0001,
    ImageLockModeWrite = 0x0002,
    ImageLockModeUserInputBuf = 0x0004,
}}
STRUCT!{struct BitmapData {
    Width: UINT,
    Height: UINT,
    Stride: INT,
    PixelFormat: PixelFormat,
    Scan0: *mut VOID,
    Reserved: UINT_PTR,
}}
ENUM!{enum ImageFlags {
    ImageFlagsNone = 0,
    ImageFlagsScalable = 0x0001,
    ImageFlagsHasAlpha = 0x0002,
    ImageFlagsHasTranslucent = 0x0004,
    ImageFlagsPartiallyScalable = 0x0008,
    ImageFlagsColorSpaceRGB = 0x0010,
    ImageFlagsColorSpaceCMYK = 0x0020,
    ImageFlagsColorSpaceGRAY = 0x0040,
    ImageFlagsColorSpaceYCBCR = 0x0080,
    ImageFlagsColorSpaceYCCK = 0x0100,
    ImageFlagsHasRealDPI = 0x1000,
    ImageFlagsHasRealPixelSize = 0x2000,
    ImageFlagsReadOnly = 0x00010000,
    ImageFlagsCaching = 0x00020000,
}}
ENUM!{enum RotateFlipType {
    RotateNoneFlipNone = 0,
    Rotate90FlipNone = 1,
    Rotate180FlipNone = 2,
    Rotate270FlipNone = 3,
    RotateNoneFlipX = 4,
    Rotate90FlipX = 5,
    Rotate180FlipX = 6,
    Rotate270FlipX = 7,
    RotateNoneFlipY = Rotate180FlipX,
    Rotate90FlipY = Rotate270FlipX,
    Rotate180FlipY = RotateNoneFlipX,
    Rotate270FlipY = Rotate90FlipX,
    RotateNoneFlipXY = Rotate180FlipNone,
    Rotate90FlipXY = Rotate270FlipNone,
    Rotate180FlipXY = RotateNoneFlipNone,
    Rotate270FlipXY = Rotate90FlipNone,
}}
STRUCT!{struct EncoderParameter {
    Guid: GUID,
    NumberOfValues: ULONG,
    Type: ULONG,
    Value: *mut VOID,
}}
STRUCT!{struct EncoderParameters {
    Count: UINT,
    Parameter: [EncoderParameter; 1],
}}
ENUM!{enum ItemDataPosition {
    ItemDataPositionAfterHeader = 0x0,
    ItemDataPositionAfterPalette = 0x1,
    ItemDataPositionAfterBits = 0x2,
}}
STRUCT!{struct ImageItemData {
    Size: UINT,
    Position: UINT,
    Desc: *mut VOID,
    DescSize: UINT,
    Data: *mut VOID,
    DataSize: UINT,
    Cookie: UINT,
}}
STRUCT!{struct PropertyItem {
    id: PROPID,
    length: ULONG,
    type_: WORD,
    value: *mut VOID,
}}
pub const PropertyTagTypeByte: WORD = 1;
pub const PropertyTagTypeASCII: WORD = 2;
pub const PropertyTagTypeShort: WORD = 3;
pub const PropertyTagTypeLong: WORD = 4;
pub const PropertyTagTypeRational: WORD = 5;
pub const PropertyTagTypeUndefined: WORD = 7;
pub const PropertyTagTypeSLONG: WORD = 9;
pub const PropertyTagTypeSRational: WORD = 10;
pub const PropertyTagExifIFD: PROPID = 0x8769;
pub const PropertyTagGpsIFD: PROPID = 0x8825;
pub const PropertyTagNewSubfileType: PROPID = 0x00FE;
pub const PropertyTagSubfileType: PROPID = 0x00FF;
pub const PropertyTagImageWidth: PROPID = 0x0100;
pub const PropertyTagImageHeight: PROPID = 0x0101;
pub const PropertyTagBitsPerSample: PROPID = 0x0102;
pub const PropertyTagCompression: PROPID = 0x0103;
pub const PropertyTagPhotometricInterp: PROPID = 0x0106;
pub const PropertyTagThreshHolding: PROPID = 0x0107;
pub const PropertyTagCellWidth: PROPID = 0x0108;
pub const PropertyTagCellHeight: PROPID = 0x0109;
pub const PropertyTagFillOrder: PROPID = 0x010A;
pub const PropertyTagDocumentName: PROPID = 0x010D;
pub const PropertyTagImageDescription: PROPID = 0x010E;
pub const PropertyTagEquipMake: PROPID = 0x010F;
pub const PropertyTagEquipModel: PROPID = 0x0110;
pub const PropertyTagStripOffsets: PROPID = 0x0111;
pub const PropertyTagOrientation: PROPID = 0x0112;
pub const PropertyTagSamplesPerPixel: PROPID = 0x0115;
pub const PropertyTagRowsPerStrip: PROPID = 0x0116;
pub const PropertyTagStripBytesCount: PROPID = 0x0117;
pub const PropertyTagMinSampleValue: PROPID = 0x0118;
pub const PropertyTagMaxSampleValue: PROPID = 0x0119;
pub const PropertyTagXResolution: PROPID = 0x011A;
pub const PropertyTagYResolution: PROPID = 0x011B;
pub const PropertyTagPlanarConfig: PROPID = 0x011C;
pub const PropertyTagPageName: PROPID = 0x011D;
pub const PropertyTagXPosition: PROPID = 0x011E;
pub const PropertyTagYPosition: PROPID = 0x011F;
pub const PropertyTagFreeOffset: PROPID = 0x0120;
pub const PropertyTagFreeByteCounts: PROPID = 0x0121;
pub const PropertyTagGrayResponseUnit: PROPID = 0x0122;
pub const PropertyTagGrayResponseCurve: PROPID = 0x0123;
pub const PropertyTagT4Option: PROPID = 0x0124;
pub const PropertyTagT6Option: PROPID = 0x0125;
pub const PropertyTagResolutionUnit: PROPID = 0x0128;
pub const PropertyTagPageNumber: PROPID = 0x0129;
pub const PropertyTagTransferFuncition: PROPID = 0x012D;
pub const PropertyTagSoftwareUsed: PROPID = 0x0131;
pub const PropertyTagDateTime: PROPID = 0x0132;
pub const PropertyTagArtist: PROPID = 0x013B;
pub const PropertyTagHostComputer: PROPID = 0x013C;
pub const PropertyTagPredictor: PROPID = 0x013D;
pub const PropertyTagWhitePoint: PROPID = 0x013E;
pub const PropertyTagPrimaryChromaticities: PROPID = 0x013F;
pub const PropertyTagColorMap: PROPID = 0x0140;
pub const PropertyTagHalftoneHints: PROPID = 0x0141;
pub const PropertyTagTileWidth: PROPID = 0x0142;
pub const PropertyTagTileLength: PROPID = 0x0143;
pub const PropertyTagTileOffset: PROPID = 0x0144;
pub const PropertyTagTileByteCounts: PROPID = 0x0145;
pub const PropertyTagInkSet: PROPID = 0x014C;
pub const PropertyTagInkNames: PROPID = 0x014D;
pub const PropertyTagNumberOfInks: PROPID = 0x014E;
pub const PropertyTagDotRange: PROPID = 0x0150;
pub const PropertyTagTargetPrinter: PROPID = 0x0151;
pub const PropertyTagExtraSamples: PROPID = 0x0152;
pub const PropertyTagSampleFormat: PROPID = 0x0153;
pub const PropertyTagSMinSampleValue: PROPID = 0x0154;
pub const PropertyTagSMaxSampleValue: PROPID = 0x0155;
pub const PropertyTagTransferRange: PROPID = 0x0156;
pub const PropertyTagJPEGProc: PROPID = 0x0200;
pub const PropertyTagJPEGInterFormat: PROPID = 0x0201;
pub const PropertyTagJPEGInterLength: PROPID = 0x0202;
pub const PropertyTagJPEGRestartInterval: PROPID = 0x0203;
pub const PropertyTagJPEGLosslessPredictors: PROPID = 0x0205;
pub const PropertyTagJPEGPointTransforms: PROPID = 0x0206;
pub const PropertyTagJPEGQTables: PROPID = 0x0207;
pub const PropertyTagJPEGDCTables: PROPID = 0x0208;
pub const PropertyTagJPEGACTables: PROPID = 0x0209;
pub const PropertyTagYCbCrCoefficients: PROPID = 0x0211;
pub const PropertyTagYCbCrSubsampling: PROPID = 0x0212;
pub const PropertyTagYCbCrPositioning: PROPID = 0x0213;
pub const PropertyTagREFBlackWhite: PROPID = 0x0214;
pub const PropertyTagICCProfile: PROPID = 0x8773;
pub const PropertyTagGamma: PROPID = 0x0301;
pub const PropertyTagICCProfileDescriptor: PROPID = 0x0302;
pub const PropertyTagSRGBRenderingIntent: PROPID = 0x0303;
pub const PropertyTagImageTitle: PROPID = 0x0320;
pub const PropertyTagCopyright: PROPID = 0x8298;
pub const PropertyTagResolutionXUnit: PROPID = 0x5001;
pub const PropertyTagResolutionYUnit: PROPID = 0x5002;
pub const PropertyTagResolutionXLengthUnit: PROPID = 0x5003;
pub const PropertyTagResolutionYLengthUnit: PROPID = 0x5004;
pub const PropertyTagPrintFlags: PROPID = 0x5005;
pub const PropertyTagPrintFlagsVersion: PROPID = 0x5006;
pub const PropertyTagPrintFlagsCrop: PROPID = 0x5007;
pub const PropertyTagPrintFlagsBleedWidth: PROPID = 0x5008;
pub const PropertyTagPrintFlagsBleedWidthScale: PROPID = 0x5009;
pub const PropertyTagHalftoneLPI: PROPID = 0x500A;
pub const PropertyTagHalftoneLPIUnit: PROPID = 0x500B;
pub const PropertyTagHalftoneDegree: PROPID = 0x500C;
pub const PropertyTagHalftoneShape: PROPID = 0x500D;
pub const PropertyTagHalftoneMisc: PROPID = 0x500E;
pub const PropertyTagHalftoneScreen: PROPID = 0x500F;
pub const PropertyTagJPEGQuality: PROPID = 0x5010;
pub const PropertyTagGridSize: PROPID = 0x5011;
pub const PropertyTagThumbnailFormat: PROPID = 0x5012;
pub const PropertyTagThumbnailWidth: PROPID = 0x5013;
pub const PropertyTagThumbnailHeight: PROPID = 0x5014;
pub const PropertyTagThumbnailColorDepth: PROPID = 0x5015;
pub const PropertyTagThumbnailPlanes: PROPID = 0x5016;
pub const PropertyTagThumbnailRawBytes: PROPID = 0x5017;
pub const PropertyTagThumbnailSize: PROPID = 0x5018;
pub const PropertyTagThumbnailCompressedSize: PROPID = 0x5019;
pub const PropertyTagColorTransferFunction: PROPID = 0x501A;
pub const PropertyTagThumbnailData: PROPID = 0x501B;
pub const PropertyTagThumbnailImageWidth: PROPID = 0x5020;
pub const PropertyTagThumbnailImageHeight: PROPID = 0x5021;
pub const PropertyTagThumbnailBitsPerSample: PROPID = 0x5022;
pub const PropertyTagThumbnailCompression: PROPID = 0x5023;
pub const PropertyTagThumbnailPhotometricInterp: PROPID = 0x5024;
pub const PropertyTagThumbnailImageDescription: PROPID = 0x5025;
pub const PropertyTagThumbnailEquipMake: PROPID = 0x5026;
pub const PropertyTagThumbnailEquipModel: PROPID = 0x5027;
pub const PropertyTagThumbnailStripOffsets: PROPID = 0x5028;
pub const PropertyTagThumbnailOrientation: PROPID = 0x5029;
pub const PropertyTagThumbnailSamplesPerPixel: PROPID = 0x502A;
pub const PropertyTagThumbnailRowsPerStrip: PROPID = 0x502B;
pub const PropertyTagThumbnailStripBytesCount: PROPID = 0x502C;
pub const PropertyTagThumbnailResolutionX: PROPID = 0x502D;
pub const PropertyTagThumbnailResolutionY: PROPID = 0x502E;
pub const PropertyTagThumbnailPlanarConfig: PROPID = 0x502F;
pub const PropertyTagThumbnailResolutionUnit: PROPID = 0x5030;
pub const PropertyTagThumbnailTransferFunction: PROPID = 0x5031;
pub const PropertyTagThumbnailSoftwareUsed: PROPID = 0x5032;
pub const PropertyTagThumbnailDateTime: PROPID = 0x5033;
pub const PropertyTagThumbnailArtist: PROPID = 0x5034;
pub const PropertyTagThumbnailWhitePoint: PROPID = 0x5035;
pub const PropertyTagThumbnailPrimaryChromaticities: PROPID = 0x5036;
pub const PropertyTagThumbnailYCbCrCoefficients: PROPID = 0x5037;
pub const PropertyTagThumbnailYCbCrSubsampling: PROPID = 0x5038;
pub const PropertyTagThumbnailYCbCrPositioning: PROPID = 0x5039;
pub const PropertyTagThumbnailRefBlackWhite: PROPID = 0x503A;
pub const PropertyTagThumbnailCopyRight: PROPID = 0x503B;
pub const PropertyTagLuminanceTable: PROPID = 0x5090;
pub const PropertyTagChrominanceTable: PROPID = 0x5091;
pub const PropertyTagFrameDelay: PROPID = 0x5100;
pub const PropertyTagLoopCount: PROPID = 0x5101;
pub const PropertyTagGlobalPalette: PROPID = 0x5102;
pub const PropertyTagIndexBackground: PROPID = 0x5103;
pub const PropertyTagIndexTransparent: PROPID = 0x5104;
pub const PropertyTagPixelUnit: PROPID = 0x5110;
pub const PropertyTagPixelPerUnitX: PROPID = 0x5111;
pub const PropertyTagPixelPerUnitY: PROPID = 0x5112;
pub const PropertyTagPaletteHistogram: PROPID = 0x5113;
pub const PropertyTagExifExposureTime: PROPID = 0x829A;
pub const PropertyTagExifFNumber: PROPID = 0x829D;
pub const PropertyTagExifExposureProg: PROPID = 0x8822;
pub const PropertyTagExifSpectralSense: PROPID = 0x8824;
pub const PropertyTagExifISOSpeed: PROPID = 0x8827;
pub const PropertyTagExifOECF: PROPID = 0x8828;
pub const PropertyTagExifVer: PROPID = 0x9000;
pub const PropertyTagExifDTOrig: PROPID = 0x9003;
pub const PropertyTagExifDTDigitized: PROPID = 0x9004;
pub const PropertyTagExifCompConfig: PROPID = 0x9101;
pub const PropertyTagExifCompBPP: PROPID = 0x9102;
pub const PropertyTagExifShutterSpeed: PROPID = 0x9201;
pub const PropertyTagExifAperture: PROPID = 0x9202;
pub const PropertyTagExifBrightness: PROPID = 0x9203;
pub const PropertyTagExifExposureBias: PROPID = 0x9204;
pub const PropertyTagExifMaxAperture: PROPID = 0x9205;
pub const PropertyTagExifSubjectDist: PROPID = 0x9206;
pub const PropertyTagExifMeteringMode: PROPID = 0x9207;
pub const PropertyTagExifLightSource: PROPID = 0x9208;
pub const PropertyTagExifFlash: PROPID = 0x9209;
pub const PropertyTagExifFocalLength: PROPID = 0x920A;
pub const PropertyTagExifSubjectArea: PROPID = 0x9214;
pub const PropertyTagExifMakerNote: PROPID = 0x927C;
pub const PropertyTagExifUserComment: PROPID = 0x9286;
pub const PropertyTagExifDTSubsec: PROPID = 0x9290;
pub const PropertyTagExifDTOrigSS: PROPID = 0x9291;
pub const PropertyTagExifDTDigSS: PROPID = 0x9292;
pub const PropertyTagExifFPXVer: PROPID = 0xA000;
pub const PropertyTagExifColorSpace: PROPID = 0xA001;
pub const PropertyTagExifPixXDim: PROPID = 0xA002;
pub const PropertyTagExifPixYDim: PROPID = 0xA003;
pub const PropertyTagExifRelatedWav: PROPID = 0xA004;
pub const PropertyTagExifInterop: PROPID = 0xA005;
pub const PropertyTagExifFlashEnergy: PROPID = 0xA20B;
pub const PropertyTagExifSpatialFR: PROPID = 0xA20C;
pub const PropertyTagExifFocalXRes: PROPID = 0xA20E;
pub const PropertyTagExifFocalYRes: PROPID = 0xA20F;
pub const PropertyTagExifFocalResUnit: PROPID = 0xA210;
pub const PropertyTagExifSubjectLoc: PROPID = 0xA214;
pub const PropertyTagExifExposureIndex: PROPID = 0xA215;
pub const PropertyTagExifSensingMethod: PROPID = 0xA217;
pub const PropertyTagExifFileSource: PROPID = 0xA300;
pub const PropertyTagExifSceneType: PROPID = 0xA301;
pub const PropertyTagExifCfaPattern: PROPID = 0xA302;
pub const PropertyTagExifCustomRendered: PROPID = 0xA401;
pub const PropertyTagExifExposureMode: PROPID = 0xA402;
pub const PropertyTagExifWhiteBalance: PROPID = 0xA403;
pub const PropertyTagExifDigitalZoomRatio: PROPID = 0xA404;
pub const PropertyTagExifFocalLengthIn35mmFilm: PROPID = 0xA405;
pub const PropertyTagExifSceneCaptureType: PROPID = 0xA406;
pub const PropertyTagExifGainControl: PROPID = 0xA407;
pub const PropertyTagExifContrast: PROPID = 0xA408;
pub const PropertyTagExifSaturation: PROPID = 0xA409;
pub const PropertyTagExifSharpness: PROPID = 0xA40A;
pub const PropertyTagExifDeviceSettingDesc: PROPID = 0xA40B;
pub const PropertyTagExifSubjectDistanceRange: PROPID = 0xA40C;
pub const PropertyTagExifUniqueImageID: PROPID = 0xA420;
pub const PropertyTagGpsVer: PROPID = 0x0000;
pub const PropertyTagGpsLatitudeRef: PROPID = 0x0001;
pub const PropertyTagGpsLatitude: PROPID = 0x0002;
pub const PropertyTagGpsLongitudeRef: PROPID = 0x0003;
pub const PropertyTagGpsLongitude: PROPID = 0x0004;
pub const PropertyTagGpsAltitudeRef: PROPID = 0x0005;
pub const PropertyTagGpsAltitude: PROPID = 0x0006;
pub const PropertyTagGpsGpsTime: PROPID = 0x0007;
pub const PropertyTagGpsGpsSatellites: PROPID = 0x0008;
pub const PropertyTagGpsGpsStatus: PROPID = 0x0009;
pub const PropertyTagGpsGpsMeasureMode: PROPID = 0x00A;
pub const PropertyTagGpsGpsDop: PROPID = 0x000B;
pub const PropertyTagGpsSpeedRef: PROPID = 0x000C;
pub const PropertyTagGpsSpeed: PROPID = 0x000D;
pub const PropertyTagGpsTrackRef: PROPID = 0x000E;
pub const PropertyTagGpsTrack: PROPID = 0x000F;
pub const PropertyTagGpsImgDirRef: PROPID = 0x0010;
pub const PropertyTagGpsImgDir: PROPID = 0x0011;
pub const PropertyTagGpsMapDatum: PROPID = 0x0012;
pub const PropertyTagGpsDestLatRef: PROPID = 0x0013;
pub const PropertyTagGpsDestLat: PROPID = 0x0014;
pub const PropertyTagGpsDestLongRef: PROPID = 0x0015;
pub const PropertyTagGpsDestLong: PROPID = 0x0016;
pub const PropertyTagGpsDestBearRef: PROPID = 0x0017;
pub const PropertyTagGpsDestBear: PROPID = 0x0018;
pub const PropertyTagGpsDestDistRef: PROPID = 0x0019;
pub const PropertyTagGpsDestDist: PROPID = 0x001A;
pub const PropertyTagGpsProcessingMethod: PROPID = 0x001B;
pub const PropertyTagGpsAreaInformation: PROPID = 0x001C;
pub const PropertyTagGpsDate: PROPID = 0x001D;
pub const PropertyTagGpsDifferential: PROPID = 0x001E;

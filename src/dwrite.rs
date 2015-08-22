// Copyright © 2015, Connor Hilarides
// Licensed under the MIT License <LICENSE.md>
//! Mappings for the contents of dwrite.h

#[repr(i32)] #[derive(Copy, Clone, Debug)] #[allow(unused_qualifications)]
pub enum DWRITE_FONT_FILE_TYPE {
    DWRITE_FONT_FILE_TYPE_UNKNOWN,
    DWRITE_FONT_FILE_TYPE_CFF,
    DWRITE_FONT_FILE_TYPE_TRUETYPE,
    DWRITE_FONT_FILE_TYPE_TRUETYPE_COLLECTION,
    DWRITE_FONT_FILE_TYPE_TYPE1_PFM,
    DWRITE_FONT_FILE_TYPE_TYPE1_PFB,
    DWRITE_FONT_FILE_TYPE_VECTOR,
    DWRITE_FONT_FILE_TYPE_BITMAP
}

pub use self::DWRITE_FONT_FILE_TYPE::*;

#[repr(i32)] #[derive(Copy, Clone, Debug)] #[allow(unused_qualifications)]
pub enum DWRITE_FONT_FACE_TYPE {
    DWRITE_FONT_FACE_TYPE_CFF,
    DWRITE_FONT_FACE_TYPE_TRUETYPE,
    DWRITE_FONT_FACE_TYPE_TRUETYPE_COLLECTION,
    DWRITE_FONT_FACE_TYPE_TYPE1,
    DWRITE_FONT_FACE_TYPE_VECTOR,
    DWRITE_FONT_FACE_TYPE_BITMAP,
    DWRITE_FONT_FACE_TYPE_UNKNOWN,
    DWRITE_FONT_FACE_TYPE_RAW_CFF
}

pub use self::DWRITE_FONT_FACE_TYPE::*;

#[repr(i32)] #[derive(Copy, Clone, Debug)] #[allow(unused_qualifications)]
pub enum DWRITE_FONT_SIMULATIONS {
    DWRITE_FONT_SIMULATIONS_NONE = 0x0000,
    DWRITE_FONT_SIMULATIONS_BOLD = 0x0001,
    DWRITE_FONT_SIMULATIONS_OBLIQUE = 0x0002
}

pub use self::DWRITE_FONT_SIMULATIONS::*;

#[repr(i32)] #[derive(Copy, Clone, Debug)] #[allow(unused_qualifications)]
pub enum DWRITE_FONT_WEIGHT {
    DWRITE_FONT_WEIGHT_THIN = 100,
    DWRITE_FONT_WEIGHT_EXTRA_LIGHT = 200,
    DWRITE_FONT_WEIGHT_LIGHT = 300,
    DWRITE_FONT_WEIGHT_SEMI_LIGHT = 350,
    DWRITE_FONT_WEIGHT_NORMAL = 400,
    DWRITE_FONT_WEIGHT_MEDIUM = 500,
    DWRITE_FONT_WEIGHT_DEMI_BOLD = 600,
    DWRITE_FONT_WEIGHT_BOLD = 700,
    DWRITE_FONT_WEIGHT_ULTRA_BOLD = 800,
    DWRITE_FONT_WEIGHT_BLACK = 900,
    DWRITE_FONT_WEIGHT_EXTRA_BLACK = 950,
}

pub use self::DWRITE_FONT_WEIGHT::*;

pub const DWRITE_FONT_WEIGHT_ULTRA_LIGHT: DWRITE_FONT_WEIGHT = DWRITE_FONT_WEIGHT_EXTRA_LIGHT;
pub const DWRITE_FONT_WEIGHT_REGULAR: DWRITE_FONT_WEIGHT = DWRITE_FONT_WEIGHT_NORMAL;
pub const DWRITE_FONT_WEIGHT_SEMI_BOLD: DWRITE_FONT_WEIGHT = DWRITE_FONT_WEIGHT_DEMI_BOLD;
pub const DWRITE_FONT_WEIGHT_EXTRA_BOLD: DWRITE_FONT_WEIGHT = DWRITE_FONT_WEIGHT_ULTRA_BOLD;
pub const DWRITE_FONT_WEIGHT_HEAVY: DWRITE_FONT_WEIGHT = DWRITE_FONT_WEIGHT_BLACK;
pub const DWRITE_FONT_WEIGHT_ULTRA_BLACK: DWRITE_FONT_WEIGHT = DWRITE_FONT_WEIGHT_EXTRA_BLACK;

#[repr(i32)] #[derive(Copy, Clone, Debug)] #[allow(unused_qualifications)]
pub enum DWRITE_FONT_STRETCH {
    DWRITE_FONT_STRETCH_UNDEFINED = 0,
    DWRITE_FONT_STRETCH_ULTRA_CONDENSED = 1,
    DWRITE_FONT_STRETCH_EXTRA_CONDENSED = 2,
    DWRITE_FONT_STRETCH_CONDENSED = 3,
    DWRITE_FONT_STRETCH_SEMI_CONDENSED = 4,
    DWRITE_FONT_STRETCH_NORMAL = 5,
    DWRITE_FONT_STRETCH_SEMI_EXPANDED = 6,
    DWRITE_FONT_STRETCH_EXPANDED = 7,
    DWRITE_FONT_STRETCH_EXTRA_EXPANDED = 8,
    DWRITE_FONT_STRETCH_ULTRA_EXPANDED = 9
}

pub use self::DWRITE_FONT_STRETCH::*;

pub const DWRITE_FONT_STRETCH_MEDIUM: DWRITE_FONT_STRETCH = DWRITE_FONT_STRETCH_NORMAL;

#[repr(i32)] #[derive(Copy, Clone, Debug)] #[allow(unused_qualifications)]
pub enum DWRITE_FONT_STYLE {
    DWRITE_FONT_STYLE_NORMAL,
    DWRITE_FONT_STYLE_OBLIQUE,
    DWRITE_FONT_STYLE_ITALIC
}

pub use self::DWRITE_FONT_STYLE::*;

#[repr(i32)] #[derive(Copy, Clone, Debug)] #[allow(unused_qualifications)]
pub enum DWRITE_INFORMATIONAL_STRING_ID {
    DWRITE_INFORMATIONAL_STRING_NONE,
    DWRITE_INFORMATIONAL_STRING_COPYRIGHT_NOTICE,
    DWRITE_INFORMATIONAL_STRING_VERSION_STRINGS,
    DWRITE_INFORMATIONAL_STRING_TRADEMARK,
    DWRITE_INFORMATIONAL_STRING_MANUFACTURER,
    DWRITE_INFORMATIONAL_STRING_DESIGNER,
    DWRITE_INFORMATIONAL_STRING_DESIGNER_URL,
    DWRITE_INFORMATIONAL_STRING_DESCRIPTION,
    DWRITE_INFORMATIONAL_STRING_FONT_VENDOR_URL,
    DWRITE_INFORMATIONAL_STRING_LICENSE_DESCRIPTION,
    DWRITE_INFORMATIONAL_STRING_LICENSE_INFO_URL,
    DWRITE_INFORMATIONAL_STRING_WIN32_FAMILY_NAMES,
    DWRITE_INFORMATIONAL_STRING_WIN32_SUBFAMILY_NAMES,
    DWRITE_INFORMATIONAL_STRING_PREFERRED_FAMILY_NAMES,
    DWRITE_INFORMATIONAL_STRING_PREFERRED_SUBFAMILY_NAMES,
    DWRITE_INFORMATIONAL_STRING_SAMPLE_TEXT,
    DWRITE_INFORMATIONAL_STRING_FULL_NAME,
    DWRITE_INFORMATIONAL_STRING_POSTSCRIPT_NAME,
    DWRITE_INFORMATIONAL_STRING_POSTSCRIPT_CID_NAME
}

pub use self::DWRITE_INFORMATIONAL_STRING_ID::*;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DWRITE_FONT_METRICS {
    pub designUnitsPerEm: ::UINT16,
    pub ascent: ::UINT16,
    pub descent: ::UINT16,
    pub lineGap: ::INT16,
    pub capHeight: ::UINT16,
    pub xHeight: ::UINT16,
    pub underlinePosition: ::INT16,
    pub underlineThickness: ::UINT16,
    pub strikethroughPosition: ::INT16,
    pub strikethroughThickness: ::UINT16,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DWRITE_GLYPH_METRICS {
    pub leftSideBearing: ::INT32,
    pub advanceWidth: ::UINT32,
    pub rightSideBearing: ::INT32,
    pub topSideBearing: ::INT32,
    pub advanceHeight: ::UINT32,
    pub bottomSideBearing: ::INT32,
    pub verticalOriginY: ::INT32,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DWRITE_GLYPH_OFFSET {
    pub advanceOffset: ::FLOAT,
    pub ascenderOffset: ::FLOAT,
}

#[repr(i32)] #[derive(Copy, Clone, Debug)] #[allow(unused_qualifications)]
pub enum DWRITE_FACTORY_TYPE {
    DWRITE_FACTORY_TYPE_SHARED,
    DWRITE_FACTORY_TYPE_ISOLATED,
}

pub use self::DWRITE_FACTORY_TYPE::*;

#[inline]
pub fn DWRITE_MAKE_OPENTYPE_TAG(a: u8, b: u8, c: u8, d: u8) -> u32 {
    ((d as u32) << 24) | ((c as u32) << 16) | ((b as u32) << 8) | (a as u32) 
}

RIDL!(
interface IDWriteFontFileLoader(IDWriteFontFileLoaderVtbl): IUnknown(IUnknownVtbl) {
    fn CreateStreamFromKey(
        &mut self, fontFileReferenceKey: *const ::c_void, fontFileReferenceKeySize: ::UINT32,
        fontFileStream: *mut *mut IDWriteFontFileStream
    ) -> ::HRESULT
});

RIDL!(
interface IDWriteLocalFontFileLoader(IDWriteLocalFontFileLoaderVtbl)
    : IDWriteFontFileLoader(IDWriteFontFileLoaderVtbl) {
    fn GetFilePathLengthFromKey(
        &mut self, fontFileReferenceKey: *const ::c_void, fontFileReferenceKeySize: ::UINT32,
        filePathLength: *mut ::UINT32
    ) -> ::HRESULT,
    fn GetFilePathFromKey(
        &mut self, fontFileReferenceKey: *const ::c_void, fontFileReferenceKeySize: ::UINT32,
        filePath: *mut ::WCHAR,
        filePathSize: ::UINT32
    ) -> ::HRESULT,
    fn GetLastWriteTimeFromKey(
        &mut self, fontFileReferenceKey: *const ::c_void, fontFileReferenceKeySize: ::UINT32,
        lastWriteTime: *mut ::FILETIME
    ) -> ::HRESULT
});

RIDL!(
interface IDWriteFontFileStream(IDWriteFontFileStreamVtbl): IUnknown(IUnknownVtbl) {
    fn ReadFileFragment(
        &mut self, fragmentStart: *mut *const ::c_void, fileOffset: ::UINT64,
        fragmentSize: ::UINT64, fragmentContext: *mut *mut ::c_void
    ) -> ::HRESULT,
    fn ReleaseFileFragment(&mut self, fragmentContext: *mut ::c_void) -> (),
    fn GetFileSize(&mut self, fileSize: *mut ::UINT64) -> ::HRESULT,
    fn GetLastWriteTime(&mut self, lastWriteTime: *mut ::UINT64) -> ::HRESULT
});

RIDL!(
interface IDWriteFontFile(IDWriteFontFileVtbl): IUnknown(IUnknownVtbl) {
    fn GetReferenceKey(
        &mut self, fontFileReferenceKey: *mut *const ::c_void,
        fontFileReferenceKeySize: *mut ::UINT32
    ) -> ::HRESULT,
    fn GetLoader(&mut self, fontFileLoader: *mut *mut IDWriteFontFileLoader) -> ::HRESULT,
    fn Analyze(
        &mut self, isSupportedFontType: *mut ::BOOL, fontFileType: *mut DWRITE_FONT_FILE_TYPE,
        fontFaceType: *mut DWRITE_FONT_FACE_TYPE, numberOfFaces: *mut ::UINT32
    ) -> ::HRESULT
});

#[repr(i32)] #[derive(Copy, Clone, Debug)] #[allow(unused_qualifications)]
pub enum DWRITE_PIXEL_GEOMETRY {
    DWRITE_PIXEL_GEOMETRY_FLAT,
    DWRITE_PIXEL_GEOMETRY_RGB,
    DWRITE_PIXEL_GEOMETRY_BGR
}

pub use self::DWRITE_PIXEL_GEOMETRY::*;

#[repr(i32)] #[derive(Copy, Clone, Debug)] #[allow(unused_qualifications)]
pub enum DWRITE_RENDERING_MODE {
    DWRITE_RENDERING_MODE_DEFAULT,
    DWRITE_RENDERING_MODE_ALIASED,
    DWRITE_RENDERING_MODE_GDI_CLASSIC,
    DWRITE_RENDERING_MODE_GDI_NATURAL,
    DWRITE_RENDERING_MODE_NATURAL,
    DWRITE_RENDERING_MODE_NATURAL_SYMMETRIC,
    DWRITE_RENDERING_MODE_OUTLINE,
}

pub use self::DWRITE_RENDERING_MODE::*;

pub const DWRITE_RENDERING_MODE_CLEARTYPE_GDI_CLASSIC: DWRITE_RENDERING_MODE =
    DWRITE_RENDERING_MODE_GDI_CLASSIC;
pub const DWRITE_RENDERING_MODE_CLEARTYPE_GDI_NATURAL: DWRITE_RENDERING_MODE =
    DWRITE_RENDERING_MODE_GDI_NATURAL;
pub const DWRITE_RENDERING_MODE_CLEARTYPE_NATURAL: DWRITE_RENDERING_MODE =
    DWRITE_RENDERING_MODE_NATURAL;
pub const DWRITE_RENDERING_MODE_CLEARTYPE_NATURAL_SYMMETRIC: DWRITE_RENDERING_MODE =
    DWRITE_RENDERING_MODE_NATURAL_SYMMETRIC;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DWRITE_MATRIX {
    pub m11: ::FLOAT,
    pub m12: ::FLOAT,
    pub m21: ::FLOAT,
    pub m22: ::FLOAT,
    pub dx: ::FLOAT,
    pub dy: ::FLOAT,
}

RIDL!(
interface IDWriteRenderingParams(IDWriteRenderingParamsVtbl): IUnknown(IUnknownVtbl) {
    fn GetGamma(&mut self) -> ::FLOAT,
    fn GetEnhancedContrast(&mut self) -> ::FLOAT,
    fn GetClearTypeLevel(&mut self) -> ::FLOAT,
    fn GetPixelGeometry(&mut self) -> DWRITE_PIXEL_GEOMETRY,
    fn GetRenderingMode(&mut self) -> DWRITE_RENDERING_MODE
});

pub type IDWriteGeometrySink = ::ID2D1SimplifiedGeometrySink;

RIDL!(
interface IDWriteFontFace(IDWriteFontFaceVtbl): IUnknown(IUnknownVtbl) {
    fn GetType(&mut self) -> DWRITE_FONT_FACE_TYPE,
    fn GetFiles(
        &mut self, numberOfFiles: *mut ::UINT32, fontFiles: *mut *mut IDWriteFontFile
    ) -> ::HRESULT,
    fn GetIndex(&mut self) -> ::UINT32,
    fn GetSimulations(&mut self) -> DWRITE_FONT_SIMULATIONS,
    fn IsSymbolFont(&mut self) -> ::BOOL,
    fn GetMetrics(&mut self, fontFaceMetrics: *mut DWRITE_FONT_METRICS) -> (),
    fn GetGlyphCount(&mut self) -> ::UINT16,
    fn GetDesignGlyphMetrics(
        &mut self, glyphIndices: *const ::UINT16, glyphCount: ::UINT32,
        glyphMetrics: *mut DWRITE_GLYPH_METRICS, isSideways: ::BOOL
    ) -> ::HRESULT,
    fn GetGlyphIndices(
        &mut self, codePoints: *const ::UINT32, codePointCount: ::UINT32,
        glyphIndices: *mut ::UINT16
    ) -> ::HRESULT,
    fn TryGetFontTable(
        &mut self, openTypeTableTag: ::UINT32, tableData: *mut *const ::c_void,
        tableSize: *mut ::UINT32, tableContext: *mut *mut ::c_void, exists: *mut ::BOOL
    ) -> ::HRESULT,
    fn ReleaseFontTable(
        &mut self, tableContext: *mut ::c_void
    ) -> ::HRESULT,
    fn GetGlyphRunOutline(
        &mut self, emSize: ::FLOAT, glyphIndices: *const ::UINT16, glyphAdvances: *const ::FLOAT,
        glyphOffsets: *const DWRITE_GLYPH_OFFSET, glyphCount: ::UINT32, isSideways: ::BOOL,
        isRightToLeft: ::BOOL, geometrySink: *mut IDWriteGeometrySink
    ) -> ::HRESULT,
    fn GetRecommendedRenderingMode(
        &mut self, emSize: ::FLOAT, pixelsPerDip: ::FLOAT, measuringMode: ::DWRITE_MEASURING_MODE,
        renderingParams: *mut IDWriteRenderingParams, renderingMode: *mut DWRITE_RENDERING_MODE
    ) -> ::HRESULT,
    fn GetGdiCompatibleMetrics(
        &mut self, emSize: ::FLOAT, pixelsPerDip: ::FLOAT, transform: *const DWRITE_MATRIX,
        fontFaceMetrics: *mut DWRITE_FONT_METRICS
    ) -> ::HRESULT,
    fn GetGdiCompatibleGlyphMetrics(
        &mut self, enSize: ::FLOAT, pixelsPerDip: ::FLOAT, transform: *const DWRITE_MATRIX,
        useGdiNatrual: ::BOOL, glyphIndices: *const ::UINT16, glyphCount: ::UINT32,
        glyphMetrics: *mut DWRITE_GLYPH_METRICS, isSideways: ::BOOL
    ) -> ::HRESULT
});

RIDL!(
interface IDWriteFontCollectionLoader(IDWriteFontCollectionLoaderVtbl): IUnknown(IUnknownVtbl) {
    fn CreateEnumeratorFromKey(
        &mut self, factory: *mut IDWriteFactory, collectionKey: *const ::c_void,
        collectionKeySize: ::UINT32, fontFileEnumerator: *mut *mut IDWriteFontFileEnumerator
    ) -> ::HRESULT
});

RIDL!(
interface IDWriteFontFileEnumerator(IDWriteFontFileEnumeratorVtbl): IUnknown(IUnknownVtbl) {
    fn MoveNext(&mut self, hasCurrentFile: *mut ::BOOL) -> ::HRESULT,
    fn GetCurrentFontFile(&mut self, fontFile: *mut *mut IDWriteFontFile) -> ::HRESULT
});

RIDL!(
interface IDWriteLocalizedStrings(IDWriteLocalizedStringsVtbl): IUnknown(IUnknownVtbl) {
    fn GetCount(&mut self) -> ::UINT32,
    fn FindLocaleName(
        &mut self, localeName: *const ::WCHAR, index: *mut ::UINT32, exists: *mut ::BOOL
    ) -> ::HRESULT,
    fn GetLocaleNameLength(&mut self, index: ::UINT32, length: *mut ::UINT32) -> ::HRESULT,
    fn GetLocaleName(
        &mut self, index: ::UINT32, localeName: *mut ::WCHAR, size: ::UINT32
    ) -> ::HRESULT,
    fn GetStringLength(&mut self, index: ::UINT32, length: *mut ::UINT32) -> ::HRESULT,
    fn GetString(
        &mut self, index: ::UINT32, stringBuffer: *mut ::WCHAR, size: ::UINT32
    ) -> ::HRESULT
});

RIDL!(
interface IDWriteFontCollection(IDWriteFontCollectionVtbl): IUnknown(IUnknownVtbl) {
    fn GetFontFamilyCount(&mut self) -> ::UINT32,
    fn GetFontFamily(
        &mut self, index: ::UINT32, fontFamily: *mut *mut IDWriteFontFamily
    ) -> ::HRESULT,
    fn FindFamilyName(
        &mut self, familyName: *const ::WCHAR, index: *mut ::UINT32, exists: *mut ::BOOL
    ) -> ::HRESULT,
    fn GetFontFromFontFace(
        &mut self, fontFace: *mut IDWriteFontFace, font: *mut *mut IDWriteFont
    ) -> ::HRESULT
});

RIDL!(
interface IDWriteFontList(IDWriteFontListVtbl): IUnknown(IUnknownVtbl) {
    fn GetFontCollection(&mut self, fontCollection: *mut *mut IDWriteFontCollection) -> ::HRESULT,
    fn GetFontCount(&mut self) -> ::UINT32,
    fn GetFont(&mut self, index: ::UINT32, font: *mut *mut IDWriteFont) -> ::HRESULT
});

RIDL!(
interface IDWriteFontFamily(IDWriteFontFamilyVtbl): IDWriteFontList(IDWriteFontListVtbl) {
    fn GetFamilyNames(&mut self, names: *mut *mut IDWriteLocalizedStrings) -> ::HRESULT,
    fn GetFirstMatchingFont(
        &mut self, weight: DWRITE_FONT_WEIGHT, stretch: DWRITE_FONT_STRETCH,
        style: DWRITE_FONT_STYLE, matchingFont: *mut *mut IDWriteFont
    ) -> ::HRESULT,
    fn GetMatchingFonts(
        &mut self, weight: DWRITE_FONT_WEIGHT, stretch: DWRITE_FONT_STRETCH,
        style: DWRITE_FONT_STYLE, matchingFonts: *mut *mut IDWriteFontList
    ) -> ::HRESULT
});

RIDL!(
interface IDWriteFont(IDWriteFontVtbl): IUnknown(IUnknownVtbl) {
    fn GetFontFamily(&mut self, fontFamily: *mut *mut IDWriteFontFamily) -> ::HRESULT,
    fn GetWeight(&mut self) -> DWRITE_FONT_WEIGHT,
    fn GetStretch(&mut self) -> DWRITE_FONT_STRETCH,
    fn GetStyle(&mut self) -> DWRITE_FONT_STYLE,
    fn IsSymbolFont(&mut self) -> ::BOOL,
    fn GetFaceNames(&mut self, names: *mut *mut IDWriteLocalizedStrings) -> ::HRESULT,
    fn GetInformationalStrings(
        &mut self, informationalStringId: DWRITE_INFORMATIONAL_STRING_ID,
        informationalStrings: *mut *mut IDWriteLocalizedStrings, exists: *mut ::BOOL
    ) -> ::HRESULT,
    fn GetSimulations(&mut self) -> DWRITE_FONT_SIMULATIONS,
    fn GetMetrics(&mut self, fontMetrics: *mut DWRITE_FONT_METRICS) -> (),
    fn HasCharacter(&mut self, unicodeValue: ::UINT32, exists: *mut ::BOOL) -> ::HRESULT,
    fn CreateFontFace(&mut self, fontFace: *mut *mut IDWriteFontFace) -> ::HRESULT
});

#[repr(i32)] #[derive(Copy, Clone, Debug)] #[allow(unused_qualifications)]
pub enum DWRITE_READING_DIRECTION {
    DWRITE_READING_DIRECTION_LEFT_TO_RIGHT = 0,
    DWRITE_READING_DIRECTION_RIGHT_TO_LEFT = 1,
    DWRITE_READING_DIRECTION_TOP_TO_BOTTOM = 2,
    DWRITE_READING_DIRECTION_BOTTOM_TO_TOP = 3,
}

pub use self::DWRITE_READING_DIRECTION::*;

#[repr(i32)] #[derive(Copy, Clone, Debug)] #[allow(unused_qualifications)]
pub enum DWRITE_FLOW_DIRECTION {
    DWRITE_FLOW_DIRECTION_TOP_TO_BOTTOM = 0,
    DWRITE_FLOW_DIRECTION_BOTTOM_TO_TOP = 1,
    DWRITE_FLOW_DIRECTION_LEFT_TO_RIGHT = 2,
    DWRITE_FLOW_DIRECTION_RIGHT_TO_LEFT = 3,
}

pub use self::DWRITE_FLOW_DIRECTION::*;

#[repr(i32)] #[derive(Copy, Clone, Debug)] #[allow(unused_qualifications)]
pub enum DWRITE_TEXT_ALIGNMENT {
    DWRITE_TEXT_ALIGNMENT_LEADING,
    DWRITE_TEXT_ALIGNMENT_TRAILING,
    DWRITE_TEXT_ALIGNMENT_CENTER,
    DWRITE_TEXT_ALIGNMENT_JUSTIFIED
}

pub use self::DWRITE_TEXT_ALIGNMENT::*;

#[repr(i32)] #[derive(Copy, Clone, Debug)] #[allow(unused_qualifications)]
pub enum DWRITE_PARAGRAPH_ALIGNMENT {
    DWRITE_PARAGRAPH_ALIGNMENT_NEAR,
    DWRITE_PARAGRAPH_ALIGNMENT_FAR,
    DWRITE_PARAGRAPH_ALIGNMENT_CENTER
}

pub use self::DWRITE_PARAGRAPH_ALIGNMENT::*;

#[repr(i32)] #[derive(Copy, Clone, Debug)] #[allow(unused_qualifications)]
pub enum DWRITE_WORD_WRAPPING {
    DWRITE_WORD_WRAPPING_WRAP = 0,
    DWRITE_WORD_WRAPPING_NO_WRAP = 1,
    DWRITE_WORD_WRAPPING_EMERGENCY_BREAK = 2,
    DWRITE_WORD_WRAPPING_WHOLE_WORD = 3,
    DWRITE_WORD_WRAPPING_CHARACTER = 4,
}

pub use self::DWRITE_WORD_WRAPPING::*;

#[repr(i32)] #[derive(Copy, Clone, Debug)] #[allow(unused_qualifications)]
pub enum DWRITE_LINE_SPACING_METHOD {
    DWRITE_LINE_SPACING_METHOD_DEFAULT,
    DWRITE_LINE_SPACING_METHOD_UNIFORM
}

pub use self::DWRITE_LINE_SPACING_METHOD::*;

#[repr(i32)] #[derive(Copy, Clone, Debug)] #[allow(unused_qualifications)]
pub enum DWRITE_TRIMMING_GRANULARITY {
    DWRITE_TRIMMING_GRANULARITY_NONE,
    DWRITE_TRIMMING_GRANULARITY_CHARACTER,
    DWRITE_TRIMMING_GRANULARITY_WORD    
}

pub use self::DWRITE_TRIMMING_GRANULARITY::*;

#[repr(i32)] #[derive(Copy, Clone, Debug)] #[allow(unused_qualifications)]
pub enum DWRITE_FONT_FEATURE_TAG {
    DWRITE_FONT_FEATURE_TAG_ALTERNATIVE_FRACTIONS = 0x63726661, // 'afrc'
    DWRITE_FONT_FEATURE_TAG_PETITE_CAPITALS_FROM_CAPITALS = 0x63703263, // 'c2pc'
    DWRITE_FONT_FEATURE_TAG_SMALL_CAPITALS_FROM_CAPITALS = 0x63733263, // 'c2sc'
    DWRITE_FONT_FEATURE_TAG_CONTEXTUAL_ALTERNATES = 0x746c6163, // 'calt'
    DWRITE_FONT_FEATURE_TAG_CASE_SENSITIVE_FORMS = 0x65736163, // 'case'
    DWRITE_FONT_FEATURE_TAG_GLYPH_COMPOSITION_DECOMPOSITION = 0x706d6363, // 'ccmp'
    DWRITE_FONT_FEATURE_TAG_CONTEXTUAL_LIGATURES = 0x67696c63, // 'clig'
    DWRITE_FONT_FEATURE_TAG_CAPITAL_SPACING = 0x70737063, // 'cpsp'
    DWRITE_FONT_FEATURE_TAG_CONTEXTUAL_SWASH = 0x68777363, // 'cswh'
    DWRITE_FONT_FEATURE_TAG_CURSIVE_POSITIONING = 0x73727563, // 'curs'
    DWRITE_FONT_FEATURE_TAG_DEFAULT = 0x746c6664, // 'dflt'
    DWRITE_FONT_FEATURE_TAG_DISCRETIONARY_LIGATURES = 0x67696c64, // 'dlig'
    DWRITE_FONT_FEATURE_TAG_EXPERT_FORMS = 0x74707865, // 'expt'
    DWRITE_FONT_FEATURE_TAG_FRACTIONS = 0x63617266, // 'frac'
    DWRITE_FONT_FEATURE_TAG_FULL_WIDTH = 0x64697766, // 'fwid'
    DWRITE_FONT_FEATURE_TAG_HALF_FORMS = 0x666c6168, // 'half'
    DWRITE_FONT_FEATURE_TAG_HALANT_FORMS = 0x6e6c6168, // 'haln'
    DWRITE_FONT_FEATURE_TAG_ALTERNATE_HALF_WIDTH = 0x746c6168, // 'halt'
    DWRITE_FONT_FEATURE_TAG_HISTORICAL_FORMS = 0x74736968, // 'hist'
    DWRITE_FONT_FEATURE_TAG_HORIZONTAL_KANA_ALTERNATES = 0x616e6b68, // 'hkna'
    DWRITE_FONT_FEATURE_TAG_HISTORICAL_LIGATURES = 0x67696c68, // 'hlig'
    DWRITE_FONT_FEATURE_TAG_HALF_WIDTH = 0x64697768, // 'hwid'
    DWRITE_FONT_FEATURE_TAG_HOJO_KANJI_FORMS = 0x6f6a6f68, // 'hojo'
    DWRITE_FONT_FEATURE_TAG_JIS04_FORMS = 0x3430706a, // 'jp04'
    DWRITE_FONT_FEATURE_TAG_JIS78_FORMS = 0x3837706a, // 'jp78'
    DWRITE_FONT_FEATURE_TAG_JIS83_FORMS = 0x3338706a, // 'jp83'
    DWRITE_FONT_FEATURE_TAG_JIS90_FORMS = 0x3039706a, // 'jp90'
    DWRITE_FONT_FEATURE_TAG_KERNING = 0x6e72656b, // 'kern'
    DWRITE_FONT_FEATURE_TAG_STANDARD_LIGATURES = 0x6167696c, // 'liga'
    DWRITE_FONT_FEATURE_TAG_LINING_FIGURES = 0x6d756e6c, // 'lnum'
    DWRITE_FONT_FEATURE_TAG_LOCALIZED_FORMS = 0x6c636f6c, // 'locl'
    DWRITE_FONT_FEATURE_TAG_MARK_POSITIONING = 0x6b72616d, // 'mark'
    DWRITE_FONT_FEATURE_TAG_MATHEMATICAL_GREEK = 0x6b72676d, // 'mgrk'
    DWRITE_FONT_FEATURE_TAG_MARK_TO_MARK_POSITIONING = 0x6b6d6b6d, // 'mkmk'
    DWRITE_FONT_FEATURE_TAG_ALTERNATE_ANNOTATION_FORMS = 0x746c616e, // 'nalt'
    DWRITE_FONT_FEATURE_TAG_NLC_KANJI_FORMS = 0x6b636c6e, // 'nlck'
    DWRITE_FONT_FEATURE_TAG_OLD_STYLE_FIGURES = 0x6d756e6f, // 'onum'
    DWRITE_FONT_FEATURE_TAG_ORDINALS = 0x6e64726f, // 'ordn'
    DWRITE_FONT_FEATURE_TAG_PROPORTIONAL_ALTERNATE_WIDTH = 0x746c6170, // 'palt'
    DWRITE_FONT_FEATURE_TAG_PETITE_CAPITALS = 0x70616370, // 'pcap'
    DWRITE_FONT_FEATURE_TAG_PROPORTIONAL_FIGURES = 0x6d756e70, // 'pnum'
    DWRITE_FONT_FEATURE_TAG_PROPORTIONAL_WIDTHS = 0x64697770, // 'pwid'
    DWRITE_FONT_FEATURE_TAG_QUARTER_WIDTHS = 0x64697771, // 'qwid'
    DWRITE_FONT_FEATURE_TAG_REQUIRED_LIGATURES = 0x67696c72, // 'rlig'
    DWRITE_FONT_FEATURE_TAG_RUBY_NOTATION_FORMS = 0x79627572, // 'ruby'
    DWRITE_FONT_FEATURE_TAG_STYLISTIC_ALTERNATES = 0x746c6173, // 'salt'
    DWRITE_FONT_FEATURE_TAG_SCIENTIFIC_INFERIORS = 0x666e6973, // 'sinf'
    DWRITE_FONT_FEATURE_TAG_SMALL_CAPITALS = 0x70636d73, // 'smcp'
    DWRITE_FONT_FEATURE_TAG_SIMPLIFIED_FORMS = 0x6c706d73, // 'smpl'
    DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_1 = 0x31307373, // 'ss01'
    DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_2 = 0x32307373, // 'ss02'
    DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_3 = 0x33307373, // 'ss03'
    DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_4 = 0x34307373, // 'ss04'
    DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_5 = 0x35307373, // 'ss05'
    DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_6 = 0x36307373, // 'ss06'
    DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_7 = 0x37307373, // 'ss07'
    DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_8 = 0x38307373, // 'ss08'
    DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_9 = 0x39307373, // 'ss09'
    DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_10 = 0x30317373, // 'ss10'
    DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_11 = 0x31317373, // 'ss11'
    DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_12 = 0x32317373, // 'ss12'
    DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_13 = 0x33317373, // 'ss13'
    DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_14 = 0x34317373, // 'ss14'
    DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_15 = 0x35317373, // 'ss15'
    DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_16 = 0x36317373, // 'ss16'
    DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_17 = 0x37317373, // 'ss17'
    DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_18 = 0x38317373, // 'ss18'
    DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_19 = 0x39317373, // 'ss19'
    DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_20 = 0x30327373, // 'ss20'
    DWRITE_FONT_FEATURE_TAG_SUBSCRIPT = 0x73627573, // 'subs'
    DWRITE_FONT_FEATURE_TAG_SUPERSCRIPT = 0x73707573, // 'sups'
    DWRITE_FONT_FEATURE_TAG_SWASH = 0x68737773, // 'swsh'
    DWRITE_FONT_FEATURE_TAG_TITLING = 0x6c746974, // 'titl'
    DWRITE_FONT_FEATURE_TAG_TRADITIONAL_NAME_FORMS = 0x6d616e74, // 'tnam'
    DWRITE_FONT_FEATURE_TAG_TABULAR_FIGURES = 0x6d756e74, // 'tnum'
    DWRITE_FONT_FEATURE_TAG_TRADITIONAL_FORMS = 0x64617274, // 'trad'
    DWRITE_FONT_FEATURE_TAG_THIRD_WIDTHS = 0x64697774, // 'twid'
    DWRITE_FONT_FEATURE_TAG_UNICASE = 0x63696e75, // 'unic'
    DWRITE_FONT_FEATURE_TAG_VERTICAL_WRITING = 0x74726576, // 'vert'
    DWRITE_FONT_FEATURE_TAG_VERTICAL_ALTERNATES_AND_ROTATION = 0x32747276, // 'vrt2'
    DWRITE_FONT_FEATURE_TAG_SLASHED_ZERO = 0x6f72657a, // 'zero'
}

pub use self::DWRITE_FONT_FEATURE_TAG::*;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DWRITE_TEXT_RANGE {
    pub startPosition: ::UINT32,
    pub length: ::UINT32,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DWRITE_FONT_FEATURE {
    pub nameTag: DWRITE_FONT_FEATURE_TAG,
    pub parameter: ::UINT32,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DWRITE_TYPOGRAPHIC_FEATURES {
    pub features: *mut DWRITE_FONT_FEATURE,
    pub featureCount: ::UINT32,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DWRITE_TRIMMING {
    pub granularity: DWRITE_TRIMMING_GRANULARITY,
    pub delimiter: ::UINT32,
    pub delimiterCount: ::UINT32,
}

RIDL!(
interface IDWriteTextFormat(IDWriteTextFormatVtbl): IUnknown(IUnknownVtbl) {
    fn SetTextAlignment(&mut self, textAlignment: DWRITE_TEXT_ALIGNMENT) -> ::HRESULT,
    fn SetParagraphAlignment(
        &mut self, paragraphAlignment: DWRITE_PARAGRAPH_ALIGNMENT
    ) -> ::HRESULT,
    fn SetWordWrapping(&mut self, wordWrapping: DWRITE_WORD_WRAPPING) -> ::HRESULT,
    fn SetReadingDirection(&mut self, readingDirection: DWRITE_READING_DIRECTION) -> ::HRESULT,
    fn SetFlowDirection(&mut self, flowDirection: DWRITE_FLOW_DIRECTION) -> ::HRESULT,
    fn SetIncrementalTabStop(&mut self, incrementalTabStop: ::FLOAT) -> ::HRESULT,
    fn SetTrimming(
        &mut self, trimmingOptions: *const DWRITE_TRIMMING, trimmingSign: *mut IDWriteInlineObject
    ) -> ::HRESULT,
    fn SetLineSpacing(
        &mut self, lineSpacingMethod: DWRITE_LINE_SPACING_METHOD, lineSpacing: ::FLOAT,
        baseLine: ::FLOAT
    ) -> ::HRESULT,
    fn GetTextAlignment(&mut self) -> DWRITE_TEXT_ALIGNMENT,
    fn GetParagraphAlignment(&mut self) -> DWRITE_PARAGRAPH_ALIGNMENT,
    fn GetWordWrapping(&mut self) -> DWRITE_WORD_WRAPPING,
    fn GetReadingDirection(&mut self) -> DWRITE_READING_DIRECTION,
    fn GetFlowDirection(&mut self) -> DWRITE_FLOW_DIRECTION,
    fn GetIncrementalTabStop(&mut self) -> ::FLOAT,
    fn GetTrimming(
        &mut self, trimmingOptions: *mut DWRITE_TRIMMING,
        trimmingSign: *mut *mut IDWriteInlineObject
    ) -> ::HRESULT,
    fn GetLineSpacing(
        &mut self, lineSpacingMethod: *mut DWRITE_LINE_SPACING_METHOD, lineSpacing: *mut ::FLOAT,
        baseline: *mut ::FLOAT
    ) -> ::HRESULT,
    fn GetFontCollection(&mut self, fontCollection: *mut *mut IDWriteFontCollection) -> ::HRESULT,
    fn GetFontFamilyNameLength(&mut self) -> ::UINT32,
    fn GetFontFamilyName(&mut self, fontFamilyName: *mut ::WCHAR, nameSize: ::UINT32) -> ::HRESULT,
    fn GetFontWeight(&mut self) -> DWRITE_FONT_WEIGHT,
    fn GetFontStyle(&mut self) -> DWRITE_FONT_STYLE,
    fn GetFontStretch(&mut self) -> DWRITE_FONT_STRETCH,
    fn GetFontSize(&mut self) -> ::FLOAT,
    fn GetLocaleNameLength(&mut self) -> ::UINT32,
    fn GetLocaleName(&mut self, localeName: *mut ::WCHAR, nameSize: ::UINT32) -> ::HRESULT
});

RIDL!(
interface IDWriteTypography(IDWriteTypographyVtbl): IUnknown(IUnknownVtbl) {
    fn AddFontFeature(&mut self, fontFeature: DWRITE_FONT_FEATURE) -> ::HRESULT,
    fn GetFontFeatureCount(&mut self) -> ::UINT32,
    fn GetFontFeature(
        &mut self, fontFeatureIndex: ::UINT32, fontFeature: *mut DWRITE_FONT_FEATURE
    ) -> ::HRESULT
});

#[repr(i32)] #[derive(Copy, Clone, Debug)] #[allow(unused_qualifications)]
pub enum DWRITE_SCRIPT_SHAPES {
    DWRITE_SCRIPT_SHAPES_DEFAULT,
    DWRITE_SCRIPT_SHAPES_NO_VISUAL,
}

pub use self::DWRITE_SCRIPT_SHAPES::*;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DWRITE_SCRIPT_ANALYSIS {
    pub script: ::UINT16,
    pub shapes: DWRITE_SCRIPT_SHAPES,
}

#[repr(i32)] #[derive(Copy, Clone, Debug)] #[allow(unused_qualifications)]
pub enum DWRITE_BREAK_CONDITION {
    DWRITE_BREAK_CONDITION_NEUTRAL,
    DWRITE_BREAK_CONDITION_CAN_BREAK,
    DWRITE_BREAK_CONDITION_MAY_NOT_BREAK,
    DWRITE_BREAK_CONDITION_MUST_BREAK,
}

pub use self::DWRITE_BREAK_CONDITION::*;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DWRITE_LINE_BREAKPOINT {
    bit_fields: ::UINT8,
}

BITFIELD!(DWRITE_LINE_BREAKPOINT bit_fields: ::UINT8 [
    breakConditionBefore set_breakConditionBefore[0..2],
    breakConditionAfter set_breakConditionAfter[2..4],
    isWhitespace set_isWhitespace[4..5],
    isSoftHyphen set_isSoftHyphen[5..6],
    padding set_padding[6..8],
]);

#[repr(i32)] #[derive(Copy, Clone, Debug)] #[allow(unused_qualifications)]
pub enum DWRITE_NUMBER_SUBSTITUTION_METHOD {
    DWRITE_NUMBER_SUBSTITUTION_METHOD_FROM_CULTURE,
    DWRITE_NUMBER_SUBSTITUTION_METHOD_CONTEXTUAL,
    DWRITE_NUMBER_SUBSTITUTION_METHOD_NONE,
    DWRITE_NUMBER_SUBSTITUTION_METHOD_NATIONAL,
    DWRITE_NUMBER_SUBSTITUTION_METHOD_TRADITIONAL,
}

pub use self::DWRITE_NUMBER_SUBSTITUTION_METHOD::*;

RIDL!(
interface IDWriteNumberSubstitution(IDWriteNumberSubstitutionVtbl): IUnknown(IUnknownVtbl) {
});

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DWRITE_SHAPING_TEXT_PROPERTIES {
    bit_fields: ::UINT16,
}

BITFIELD!(DWRITE_SHAPING_TEXT_PROPERTIES bit_fields: ::UINT16 [
    isShapedAlone set_isShapedAlone[0..1],
    reserved set_reserved[1..16],
]);

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DWRITE_SHAPING_GLYPH_PROPERTIES {
    bit_fields: ::UINT16,
}

BITFIELD!(DWRITE_SHAPING_GLYPH_PROPERTIES bit_fields: ::UINT16 [
    justification set_justification[0..4],
    isClusterStart set_isClusterStart[4..5],
    isDiacritic set_isDiacritic[5..6],
    isZeroWidthSpace set_isZeroWidthSpace[6..7],
    reserved set_reserved[7..16],
]);

RIDL!(
interface IDWriteTextAnalysisSource(IDWriteTextAnalysisSourceVtbl): IUnknown(IUnknownVtbl) {
    fn GetTextAtPosition(
        &mut self, textPosition: ::UINT32, textString: *mut *const ::WCHAR,
        textLength: *mut ::UINT32
    ) -> ::HRESULT,
    fn GetTextBeforePosition(
        &mut self, textPosition: ::UINT32, textString: *mut *const ::WCHAR,
        textLength: *mut ::UINT32
    ) -> ::HRESULT,
    fn GetParagraphReadingDirection(&mut self) -> DWRITE_READING_DIRECTION,
    fn GetLocaleName(
        &mut self, textPosition: ::UINT32, textLength: *mut ::UINT32,
        localeName: *mut *const ::WCHAR
    ) -> ::HRESULT,
    fn GetNumberSubstitution(
        &mut self, textPosition: ::UINT32, textLength: *mut ::UINT32,
        numberSubstitution: *mut *mut IDWriteNumberSubstitution
    ) -> ::HRESULT
});

RIDL!(
interface IDWriteTextAnalysisSink(IDWriteTextAnalysisSinkVtbl): IUnknown(IUnknownVtbl) {
    fn SetScriptAnalysis(
        &mut self, textPosition: ::UINT32, textLength: ::UINT32,
        scriptAnalysis: *const DWRITE_SCRIPT_ANALYSIS
    ) -> ::HRESULT,
    fn SetLineBreakpoints(
        &mut self, textPosition: ::UINT32, textLength: ::UINT32,
        lineBreakpoints: *const DWRITE_LINE_BREAKPOINT
    ) -> ::HRESULT,
    fn SetBidiLevel(
        &mut self, textPosition: ::UINT32, textLength: ::UINT32, explicitLevel: ::UINT8,
        resolvedLevel: ::UINT8
    ) -> ::HRESULT,
    fn SetNumberSubstitution(
        &mut self, textPosition: ::UINT32, textLength: ::UINT32,
        numberSubstitution: *mut IDWriteNumberSubstitution
    ) -> ::HRESULT
});

RIDL!(
interface IDWriteTextAnalyzer(IDWriteTextAnalyzerVtbl): IUnknown(IUnknownVtbl) {
    fn AnalyzeScript(
        &mut self, analysisSource: *mut IDWriteTextAnalysisSource, textPosition: ::UINT32,
        textLength: ::UINT32, analysisSink: *mut IDWriteTextAnalysisSink
    ) -> ::HRESULT,
    fn AnalyzeBidi(
        &mut self, analysisSource: *mut IDWriteTextAnalysisSource, textPosition: ::UINT32,
        textLength: ::UINT32, analysisSink: *mut IDWriteTextAnalysisSink
    ) -> ::HRESULT,
    fn AnalyzeNumberSubstitution(
        &mut self, analysisSource: *mut IDWriteTextAnalysisSource, textPosition: ::UINT32,
        textLength: ::UINT32, analysisSink: *mut IDWriteTextAnalysisSink
    ) -> ::HRESULT,
    fn AnalyzeLineBreakpoints(
        &mut self, analysisSource: *mut IDWriteTextAnalysisSource, textPosition: ::UINT32,
        textLength: ::UINT32, analysisSink: *mut IDWriteTextAnalysisSink
    ) -> ::HRESULT,
    fn GetGlyphs(
        &mut self, textString: *const ::WCHAR, textLength: ::UINT32,
        fontFace: *mut IDWriteFontFace, isSideways: ::BOOL, isRightToLeft: ::BOOL,
        scriptAnalysis: *const DWRITE_SCRIPT_ANALYSIS, localeName: *const ::WCHAR,
        numberSubstitution: *mut IDWriteNumberSubstitution,
        features: *mut *const DWRITE_TYPOGRAPHIC_FEATURES, featureRangeLengths: *const ::UINT32,
        featureRanges: ::UINT32, maxGlyphCount: ::UINT32, clusterMap: *mut ::UINT16,
        textProps: *mut DWRITE_SHAPING_TEXT_PROPERTIES, glyphIndices: *mut ::UINT16,
        glyphProps: *mut DWRITE_SHAPING_GLYPH_PROPERTIES, actualGlyphCount: *mut ::UINT32
    ) -> ::HRESULT,
    fn GetGlyphPlacements(
        &mut self, textString: *const ::WCHAR, clusterMap: *const ::UINT16,
        textProps: *mut DWRITE_SHAPING_TEXT_PROPERTIES, textLength: ::UINT32,
        glyphIndices: *const ::UINT16, glyphProps: *const DWRITE_SHAPING_GLYPH_PROPERTIES,
        glyphCount: ::UINT32, fontFace: *mut IDWriteFontFace, fontEmSize: ::FLOAT,
        isSideways: ::BOOL, isRightToLeft: ::BOOL, scriptAnalysis: *const DWRITE_SCRIPT_ANALYSIS,
        localeName: *const ::WCHAR, features: *mut *const DWRITE_TYPOGRAPHIC_FEATURES,
        featureRangeLengths: *const ::UINT32, featureRanges: ::UINT32, glyphAdvances: *mut ::FLOAT,
        glyphOffsets: *mut DWRITE_GLYPH_OFFSET
    ) -> ::HRESULT,
    fn GetGdiCompatibleGlyphPlacements(
        &mut self, textString: *const ::WCHAR, clusterMap: *const ::UINT16,
        textProps: *mut DWRITE_SHAPING_TEXT_PROPERTIES, textLength: ::UINT32,
        glyphIndices: *const ::UINT16, glyphProps: *const DWRITE_SHAPING_GLYPH_PROPERTIES,
        glyphCount: ::UINT32, fontFace: *mut IDWriteFontFace, fontEmSize: ::FLOAT,
        pixelsPerDip: ::FLOAT, transform: *const DWRITE_MATRIX, useGdiNatrual: ::BOOL,
        isSideways: ::BOOL, isRightToLeft: ::BOOL, scriptAnalysis: *const DWRITE_SCRIPT_ANALYSIS,
        localeName: *const ::WCHAR, features: *mut *const DWRITE_TYPOGRAPHIC_FEATURES,
        featureRangeLengths: *const ::UINT32, featureRanges: ::UINT32, glyphAdvances: *mut ::FLOAT,
        glyphOffsets: *mut DWRITE_GLYPH_OFFSET
    ) -> ::HRESULT
});

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DWRITE_GLYPH_RUN {
    pub fontFace: *mut IDWriteFontFace,
    pub fontEmSize: ::FLOAT,
    pub glyphCount: ::UINT32,
    pub glyphIndices: *const ::UINT16,
    pub glyphAdvances: *const ::FLOAT,
    pub glyphOffsets: *const DWRITE_GLYPH_OFFSET,
    pub isSideways: ::BOOL,
    pub bidiLevel: ::UINT32
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DWRITE_GLYPH_RUN_DESCRIPTION {
    pub localeName: *const ::WCHAR,
    pub string: *const ::WCHAR,
    pub stringLength: ::UINT32,
    pub clusterMap: *const ::UINT16,
    pub textPosition: ::UINT32,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DWRITE_UNDERLINE {
    pub width: ::FLOAT,
    pub thickness: ::FLOAT,
    pub offset: ::FLOAT,
    pub runHeight: ::FLOAT,
    pub readingDirection: DWRITE_READING_DIRECTION,
    pub flowDirection: DWRITE_FLOW_DIRECTION,
    pub localeName: *const ::WCHAR,
    pub measuringMode: ::DWRITE_MEASURING_MODE,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DWRITE_STRIKETHROUGH {
    pub width: ::FLOAT,
    pub thickness: ::FLOAT,
    pub offset: ::FLOAT,
    pub readingDirection: DWRITE_READING_DIRECTION,
    pub flowDirection: DWRITE_FLOW_DIRECTION,
    pub localeName: *const ::WCHAR,
    pub measuringMode: ::DWRITE_MEASURING_MODE,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DWRITE_LINE_METRICS {
    pub length: ::UINT32,
    pub trailingWhitespaceLength: ::UINT32,
    pub newlineLength: ::UINT32,
    pub height: ::FLOAT,
    pub baseline: ::FLOAT,
    pub isTrimmed: ::BOOL,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DWRITE_CLUSTER_METRICS {
    pub width: ::FLOAT,
    pub length: ::UINT16,
    pub bit_fields: ::UINT16,
}
BITFIELD!(DWRITE_CLUSTER_METRICS bit_fields: ::UINT16 [
    canWrapLineAfter set_canWrapLineAfter[0..1],
    isWhitespace set_isWhitespace[1..2],
    isNewline set_isNewline[2..3],
    isSoftHyphen set_isSoftHyphen[3..4],
    isRightToLeft set_isRightToLeft[4..5],
    padding set_padding[5..16],
]);

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DWRITE_TEXT_METRICS {
    pub left: ::FLOAT,
    pub top: ::FLOAT,
    pub width: ::FLOAT,
    pub widthIncludingTrailingWhitespace: ::FLOAT,
    pub height: ::FLOAT,
    pub layoutWidth: ::FLOAT,
    pub layoutHeight: ::FLOAT,
    pub maxBidiReorderingDepth: ::UINT32,
    pub lineCount: ::UINT32,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DWRITE_INLINE_OBJECT_METRICS {
    pub width: ::FLOAT,
    pub height: ::FLOAT,
    pub baseline: ::FLOAT,
    pub supportsSideways: ::BOOL,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DWRITE_OVERHANG_METRICS {
    pub left: ::FLOAT,
    pub top: ::FLOAT,
    pub right: ::FLOAT,
    pub bottom: ::FLOAT,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DWRITE_HIT_TEST_METRICS {
    pub textPosition: ::UINT32,
    pub length: ::UINT32,
    pub left: ::FLOAT,
    pub top: ::FLOAT,
    pub width: ::FLOAT,
    pub height: ::FLOAT,
    pub bidiLevel: ::UINT32,
    pub isText: ::BOOL,
    pub isTrimmed: ::BOOL,
}

RIDL!(
interface IDWriteInlineObject(IDWriteInlineObjectVtbl): IUnknown(IUnknownVtbl) {
    fn Draw(
        &mut self, clientDrawingContext: *mut ::c_void, renderer: *mut IDWriteTextRenderer,
        originX: ::FLOAT, originY: ::FLOAT, isSideways: ::BOOL, isRightToLeft: ::BOOL,
        clientDrawingEffect: *mut ::IUnknown
    ) -> ::HRESULT,
    fn GetMetrics(&mut self, metrics: *mut DWRITE_INLINE_OBJECT_METRICS) -> ::HRESULT,
    fn GetOverhangMetrics(&mut self, overhangs: *mut DWRITE_OVERHANG_METRICS) -> ::HRESULT,
    fn GetBreakConditions(
        &mut self, breakConditionBefore: *mut DWRITE_BREAK_CONDITION,
        breakConditionAfter: *mut DWRITE_BREAK_CONDITION
    ) -> ::HRESULT
});

RIDL!(
interface IDWritePixelSnapping(IDWritePixelSnappingVtbl): IUnknown(IUnknownVtbl) {
    fn IsPixelSnappingDisabled(
        &mut self, clientDrawingContext: *mut ::c_void, isDisabled: *mut ::BOOL
    ) -> ::HRESULT,
    fn GetCurrentTransform(
        &mut self, clientDrawingContext: *mut ::c_void, transform: *mut DWRITE_MATRIX
    ) -> ::HRESULT,
    fn GetPixelsPerDip(
        &mut self, clientDrawingContext: *mut ::c_void, pixelsPerDip: *mut ::FLOAT
    ) -> ::HRESULT
});

RIDL!(
interface IDWriteTextRenderer(IDWriteTextRendererVtbl)
    : IDWritePixelSnapping(IDWritePixelSnappingVtbl) {
    fn DrawGlyphRun(
        &mut self, clientDrawingContext: *mut ::c_void, baselineOriginX: ::FLOAT,
        baselineOriginY: ::FLOAT, measuringMode: ::DWRITE_MEASURING_MODE,
        glyphRun: *const DWRITE_GLYPH_RUN,
        glyphRunDescription: *const DWRITE_GLYPH_RUN_DESCRIPTION,
        clientDrawingEffect: *mut ::IUnknown
    ) -> ::HRESULT,
    fn DrawUnderline(
        &mut self, clientDrawingContext: *mut ::c_void, baselineOriginX: ::FLOAT,
        baselineOriginY: ::FLOAT, underline: *const DWRITE_UNDERLINE,
        clientDrawingEffect: *mut ::IUnknown
    ) -> ::HRESULT,
    fn DrawStrikethrough(
        &mut self, clientDrawingContext: *mut ::c_void, baselineOriginX: ::FLOAT,
        baselineOriginY: ::FLOAT, strikethrough: *const DWRITE_STRIKETHROUGH,
        clientDrawingEffect: *mut ::IUnknown
    ) -> ::HRESULT,
    fn DrawInlineObject(
        &mut self, clientDrawingContext: *mut ::c_void, baselineOriginX: ::FLOAT,
        baselineOriginY: ::FLOAT, inlineObject: *mut IDWriteInlineObject,
        isSideways: ::BOOL, isRightToLeft: ::BOOL, clientDrawingEffect: *mut ::IUnknown
    ) -> ::HRESULT
});

RIDL!(
interface IDWriteTextLayout(IDWriteTextLayoutVtbl): IDWriteTextFormat(IDWriteTextFormatVtbl) {
    fn SetMaxWidth(&mut self, maxWidth: ::FLOAT) -> ::HRESULT,
    fn SetMaxHeight(&mut self, maxHeight: ::FLOAT) -> ::HRESULT,
    fn SetFontCollection(
        &mut self, fontCollection: *mut IDWriteFontCollection, textRange: DWRITE_TEXT_RANGE
    ) -> ::HRESULT,
    fn SetFontFamilyName(
        &mut self, fontFamilyName: *const ::WCHAR, textRange: DWRITE_TEXT_RANGE
    ) -> ::HRESULT,
    fn SetFontWeight(
        &mut self, fontWeight: DWRITE_FONT_WEIGHT, textRange: DWRITE_TEXT_RANGE
    ) -> ::HRESULT,
    fn SetFontStyle(
        &mut self, fontStyle: DWRITE_FONT_STYLE, textRange: DWRITE_TEXT_RANGE
    ) -> ::HRESULT,
    fn SetFontStretch(
        &mut self, fontStretch: DWRITE_FONT_STRETCH, textRange: DWRITE_TEXT_RANGE
    ) -> ::HRESULT,
    fn SetFontSize(&mut self, fontSize: ::FLOAT, textRange: DWRITE_TEXT_RANGE) -> ::HRESULT,
    fn SetUnderline(&mut self, hasUnderline: ::BOOL, textRange: DWRITE_TEXT_RANGE) -> ::HRESULT,
    fn SetStrikethrough(
        &mut self, hasStrikethrough: ::BOOL, textRange: DWRITE_TEXT_RANGE
    ) -> ::HRESULT,
    fn SetDrawingEffect(
        &mut self, drawingEffect: *mut ::IUnknown, textRange: DWRITE_TEXT_RANGE
    ) -> ::HRESULT,
    fn SetInlineObject(
        &mut self, inlineObject: *mut IDWriteInlineObject, textRange: DWRITE_TEXT_RANGE
    ) -> ::HRESULT,
    fn SetTypography(
        &mut self, typography: *mut IDWriteTypography, textRange: DWRITE_TEXT_RANGE
    ) -> ::HRESULT,
    fn SetLocaleName(
        &mut self, localeName: *const ::WCHAR, textRange: DWRITE_TEXT_RANGE
    ) -> ::HRESULT,
    fn GetMaxWidth(&mut self) -> ::FLOAT,
    fn GetMaxHeight(&mut self) -> ::FLOAT,
    fn GetFontCollection(
        &mut self, currentPosition: ::UINT32, fontCollection: *mut *mut IDWriteFontCollection,
        textRange: *mut DWRITE_TEXT_RANGE
    ) -> ::HRESULT,
    fn GetFontFamilyNameLength(
        &mut self, currentPosition: *mut ::UINT32, nameLength: *mut ::UINT32,
        textRange: *mut DWRITE_TEXT_RANGE
    ) -> ::HRESULT,
    fn GetFontFamilyName(
        &mut self, currentPosition: ::UINT32, fontFamilyName: *mut ::WCHAR,
        nameSize: ::UINT32, textRange: *mut DWRITE_TEXT_RANGE
    ) -> ::HRESULT,
    fn GetFontWeight(
        &mut self, currentPosition: ::UINT32, fontWeight: *mut DWRITE_FONT_WEIGHT,
        textRange: *mut DWRITE_TEXT_RANGE
    ) -> ::HRESULT,
    fn GetFontStyle(
        &mut self, currentPosition: ::UINT32, fontStyle: *mut DWRITE_FONT_STYLE,
        textRange: *mut DWRITE_TEXT_RANGE
    ) -> ::HRESULT,
    fn GetFontStretch(
        &mut self, currentPosition: ::UINT32, fontStretch: *mut DWRITE_FONT_STRETCH,
        textRange: *mut DWRITE_TEXT_RANGE
    ) -> ::HRESULT,
    fn GetFontSize(
        &mut self, currentPosition: ::UINT32, fontSize: *mut ::FLOAT,
        textRange: *mut DWRITE_TEXT_RANGE
    ) -> ::HRESULT,
    fn GetUnderline(
        &mut self, currentPosition: ::UINT32, hasUnderline: *mut ::BOOL,
        textRange: *mut DWRITE_TEXT_RANGE
    ) -> ::HRESULT,
    fn GetStrikethrough(
        &mut self, currentPosition: ::UINT32, hasStrikethrough: *mut ::BOOL,
        textRange: *mut DWRITE_TEXT_RANGE
    ) -> ::HRESULT,
    fn GetDrawingEffect(
        &mut self, currentPosition: ::UINT32, drawingEffect: *mut *mut ::IUnknown,
        textRange: *mut DWRITE_TEXT_RANGE
    ) -> ::HRESULT,
    fn GetInlineObject(
        &mut self, currentPosition: ::UINT32, inlineObject: *mut *mut IDWriteInlineObject,
        textRange: *mut DWRITE_TEXT_RANGE
    ) -> ::HRESULT,
    fn GetTypography(
        &mut self, currentPosition: ::UINT32, typography: *mut *mut IDWriteTypography,
        textRange: *mut DWRITE_TEXT_RANGE
    ) -> ::HRESULT,
    fn GetLocaleNameLength(
        &mut self, currentPosition: ::UINT32, nameLength: *mut ::UINT32,
        textRange: *mut DWRITE_TEXT_RANGE
    ) -> ::HRESULT,
    fn GetLocaleName(
        &mut self, currentPosition: ::UINT32, localeName: *mut ::WCHAR, nameSize: ::UINT32,
        textRange: *mut DWRITE_TEXT_RANGE
    ) -> ::HRESULT,
    fn Draw(
        &mut self, clientDrawingContext: *mut ::c_void, renderer: *mut IDWriteTextRenderer,
        originX: ::FLOAT, originY: ::FLOAT
    ) -> ::HRESULT,
    fn GetLineMetrics(
        &mut self, lineMetrics: *mut DWRITE_LINE_METRICS, maxLineCount: ::UINT32,
        actualLineCount: *mut ::UINT32
    ) -> ::HRESULT,
    fn GetMetrics(&mut self, textMetrics: *mut DWRITE_TEXT_METRICS) -> ::HRESULT,
    fn GetOverhangMetrics(&mut self, overhangs: *mut DWRITE_OVERHANG_METRICS) -> ::HRESULT,
    fn GetClusterMetrics(
        &mut self, clusterMetrics: *mut DWRITE_CLUSTER_METRICS, maxClusterCount: ::UINT32,
        actualClusterCount: *mut ::UINT32
    ) -> ::HRESULT,
    fn DetermineMinWidth(&mut self, minWidth: *mut ::FLOAT) -> ::HRESULT,
    fn HitTestPoint(
        &mut self, pointX: ::FLOAT, pointY: ::FLOAT, isTrailingHit: *mut ::BOOL,
        isInside: *mut ::BOOL, hitTestMetrics: *mut DWRITE_HIT_TEST_METRICS
    ) -> ::HRESULT,
    fn HitTestTextPosition(
        &mut self, textPosition: ::UINT32, isTrailingHit: ::BOOL, pointX: *mut ::FLOAT,
        pointY: *mut ::FLOAT, hitTestMetrics: *mut DWRITE_HIT_TEST_METRICS
    ) -> ::HRESULT,
    fn HitTestTextRange(
        &mut self, textPosition: ::UINT32, textLength: ::UINT32, originX: ::FLOAT,
        originY: ::FLOAT, hitTestMetrics: *mut DWRITE_HIT_TEST_METRICS,
        maxHitTestMetricsCount: ::UINT32, actualHitTestMetricsCount: *mut ::UINT32
    ) -> ::HRESULT
});

RIDL!(
interface IDWriteBitmapRenderTarget(IDWriteBitmapRenderTargetVtbl): IUnknown(IUnknownVtbl) {
    fn DrawGlyphRun(
        &mut self, baselineOriginX: ::FLOAT, baselineOriginY: ::FLOAT,
        measuringMode: ::DWRITE_MEASURING_MODE, glyphRun: *const ::c_void,
        renderingParams: *mut IDWriteRenderingParams, textColor: ::COLORREF,
        blackBoxRect: *mut ::RECT
    ) -> ::HRESULT,
    fn GetMemoryDC(&mut self) -> ::HDC,
    fn GetPixelsPerDip(&mut self) -> ::FLOAT,
    fn SetPixelsPerDip(&mut self, pixelsPerDip: ::FLOAT) -> ::HRESULT,
    fn GetCurrentTransform(&mut self, transform: *mut DWRITE_MATRIX) -> ::HRESULT,
    fn SetCurrentTransform(&mut self, transform: *const DWRITE_MATRIX) -> ::HRESULT,
    fn GetSize(&mut self, size: *mut ::SIZE) -> ::HRESULT,
    fn Resize(&mut self, width: ::UINT32, height: ::UINT32) -> ::HRESULT
});

RIDL!(
interface IDWriteGdiInterop(IDWriteGdiInteropVtbl): IUnknown(IUnknownVtbl) {
    fn CreateFontFromLOGFONT(
        &mut self, logFont: *const ::LOGFONTW, font: *mut *mut IDWriteFont
    ) -> ::HRESULT,
    fn ConvertFontToLOGFONT(
        &mut self, font: *mut IDWriteFont, logFont: *mut ::LOGFONTW, isSystemFont: *mut ::BOOL
    ) -> ::HRESULT,
    fn ConvertFontFaceToLOGFONT(
        &mut self, font: *mut IDWriteFontFace, logFont: *mut ::LOGFONTW
    ) -> ::HRESULT,
    fn CreateFontFaceFromHdc(
        &mut self, hdc: ::HDC, fontFace: *mut *mut IDWriteFontFace
    ) -> ::HRESULT,
    fn CreateBitmapRenderTarget(
        &mut self, hdc: ::HDC, width: ::UINT32, height: ::UINT32,
        renderTarget: *mut *mut IDWriteBitmapRenderTarget
    ) -> ::HRESULT
});

#[repr(i32)] #[derive(Copy, Clone, Debug)] #[allow(unused_qualifications)]
pub enum DWRITE_TEXTURE_TYPE {
    DWRITE_TEXTURE_ALIASED_1x1 = 0,
    DWRITE_TEXTURE_CLEARTYPE_3x1 = 1,
}

pub use self::DWRITE_TEXTURE_TYPE::*;

RIDL!(
interface IDWriteGlyphRunAnalysis(IDWriteGlyphRunAnalysisVtbl): IUnknown(IUnknownVtbl) {
    fn GetAlphaTextureBounds(
        &mut self, textureType: DWRITE_TEXTURE_TYPE, textureBounds: *mut ::RECT
    ) -> ::HRESULT,
    fn CreateAlphaTexture(
        &mut self, textureType: DWRITE_TEXTURE_TYPE, textureBounds: *const ::RECT,
        alphaValues: *mut ::BYTE, bufferSize: ::UINT32
    ) -> ::HRESULT,
    fn GetAlphaBlendParams(
        &mut self, renderingParams: *mut IDWriteRenderingParams, blendGamma: *mut ::FLOAT,
        blendEnhancedContrast: *mut ::FLOAT, blendClearTypeLevel: *mut ::FLOAT
    ) -> ::HRESULT
});

RIDL!(
interface IDWriteFactory(IDWriteFactoryVtbl): IUnknown(IUnknownVtbl) {
    fn GetSystemFontCollection(
        &mut self, fontCollection: *mut *mut IDWriteFontCollection, checkForUpdates: ::BOOL
    ) -> ::HRESULT,
    fn CreateCustomFontCollection(
        &mut self, collectionLoader: *mut IDWriteFontCollectionLoader,
        collectionKey: *const ::c_void, collectionKeySize: ::UINT32,
        fontCollection: *mut *mut IDWriteFontCollection
    ) -> ::HRESULT,
    fn RegisterFontCollectionLoader(
        &mut self, fontCollectionLoader: *mut IDWriteFontCollectionLoader
    ) -> ::HRESULT,
    fn UnregisterFontCollectionLoader(
        &mut self, fontCollectionLoader: *mut IDWriteFontCollectionLoader
    ) -> ::HRESULT,
    fn CreateFontFileReference(
        &mut self, filePath: *const ::WCHAR, lastWriteTime: *const ::FILETIME,
        fontFile: *mut *mut IDWriteFontFile
    ) -> ::HRESULT,
    fn CreateCustomFontFileReference(
        &mut self, fontFileReferenceKey: *const ::c_void, fontFileReferenceKeySize: ::UINT32,
        fontFileLoader: *mut IDWriteFontFileLoader, fontFile: *mut *mut IDWriteFontFile
    ) -> ::HRESULT,
    fn CreateFontFace(
        &mut self, fontFaceType: DWRITE_FONT_FACE_TYPE, numberOfFiles: ::UINT32,
        fontFiles: *const *mut IDWriteFontFile, faceIndex: ::UINT32,
        fontFaceSimulationFlags: DWRITE_FONT_SIMULATIONS, fontFace: *mut *mut IDWriteFontFace
    ) -> ::HRESULT,
    fn CreateRenderingParams(
        &mut self, renderingParams: *mut *mut IDWriteRenderingParams
    ) -> ::HRESULT,
    fn CreateMonitorRenderingParams(
        &mut self, monitor: ::HMONITOR, renderingParams: *mut *mut IDWriteRenderingParams
    ) -> ::HRESULT,
    fn CreateCustomRenderingParams(
        &mut self, gamma: ::FLOAT, enhancedContrast: ::FLOAT, clearTypeLevel: ::FLOAT,
        pixelGeometry: DWRITE_PIXEL_GEOMETRY, renderingMode: DWRITE_RENDERING_MODE,
        renderingParams: *mut *mut IDWriteRenderingParams
    ) -> ::HRESULT,
    fn RegisterFontFileLoader(
        &mut self, fontFileLoader: *mut IDWriteFontFileLoader
    ) -> ::HRESULT,
    fn UnregisterFontFileLoader(
        &mut self, fontFileLoader: *mut IDWriteFontFileLoader
    ) -> ::HRESULT,
    fn CreateTextFormat(
        &mut self, fontFamilyName: *const ::WCHAR, fontCollection: *mut IDWriteFontCollection,
        fontWeight: DWRITE_FONT_WEIGHT, fontStyle: DWRITE_FONT_STYLE,
        fontStretch: DWRITE_FONT_STRETCH, fontSize: ::FLOAT, localeName: *const ::WCHAR,
        textFormat: *mut *mut IDWriteTextFormat
    ) -> ::HRESULT,
    fn CreateTypography(&mut self, typography: *mut *mut IDWriteTypography) -> ::HRESULT,
    fn GetGdiInterop(&mut self, gdiInterop: *mut *mut IDWriteGdiInterop) -> ::HRESULT,
    fn CreateTextLayout(
        &mut self, string: *const ::WCHAR, stringLength: ::UINT32,
        textFormat: *mut IDWriteTextFormat, maxWidth: ::FLOAT, maxHeight: ::FLOAT,
        textLayout: *mut *mut IDWriteTextLayout
    ) -> ::HRESULT,
    fn CreateGdiCompatibleTextLayout(
        &mut self, string: *const ::WCHAR, stringLength: ::UINT32,
        textFormat: *mut IDWriteTextFormat, layoutWidth: ::FLOAT, layoutHeight: ::FLOAT,
        pixelsPerDip: ::FLOAT, transform: *const DWRITE_MATRIX, useGdiNatrual: ::BOOL,
        textLayout: *mut *mut IDWriteTextLayout
    ) -> ::HRESULT,
    fn CreateEllipsisTrimmingSign(
        &mut self, textFormat: *mut IDWriteTextFormat, trimmingSign: *mut *mut IDWriteInlineObject
    ) -> ::HRESULT,
    fn CreateTextAnalyzer(&mut self, textAnalyzer: *mut *mut IDWriteTextAnalyzer) -> ::HRESULT,
    fn CreateNumberSubstitution(
        &mut self, substitutionMethod: DWRITE_NUMBER_SUBSTITUTION_METHOD,
        localeName: *const ::WCHAR, ignoreUserOverride: ::BOOL,
        numberSubstitution: *mut *mut IDWriteNumberSubstitution
    ) -> ::HRESULT,
    fn CreateGlyphRunAnalysis(
        &mut self, glyphRun: *const DWRITE_GLYPH_RUN, pixelsPerDip: ::FLOAT,
        transform: *const DWRITE_MATRIX, renderingMode: DWRITE_RENDERING_MODE,
        measuringMode: ::DWRITE_MEASURING_MODE, baselineOriginX: ::FLOAT,
        baselineOriginY: ::FLOAT, glyphRunAnalysis: *mut *mut IDWriteGlyphRunAnalysis
    ) -> ::HRESULT
});
DEFINE_GUID!(
    UuidOfIDWriteFactory,0xb859ee5a,0xd838,0x4b5b,0xa2,0xe8,0x1a,0xdc,0x7d,0x93,0xdb,0x48
);

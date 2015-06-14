// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to usp10.
#![cfg(windows)]
extern crate winapi;
use winapi::*;
extern "system" {
    pub fn ScriptApplyDigitSubstitution(
        psds: *const SCRIPT_DIGITSUBSTITUTE, psc: *mut SCRIPT_CONTROL, pss: *mut SCRIPT_STATE,
    ) -> HRESULT;
    pub fn ScriptApplyLogicalWidth(
        piDx: *const c_int, cChars: c_int, cGlyphs: c_int, pwLogClust: *const WORD,
        psva: *const SCRIPT_VISATTR, piAdvance: *const c_int, psa: *const SCRIPT_ANALYSIS,
        pABC: *mut ABC, piJustify: *mut c_int,
    ) -> HRESULT;
    pub fn ScriptBreak(
        pwcChars: *const WCHAR, cChars: c_int, psa: *const SCRIPT_ANALYSIS,
        psla: *mut SCRIPT_LOGATTR,
    ) -> HRESULT;
    pub fn ScriptCPtoX(
        iCP: c_int, fTrailing: BOOL, cChars: c_int, cGlyphs: c_int, pwLogClust: *const WORD,
        psva: *const SCRIPT_VISATTR, piAdvance: *const c_int, psa: *const SCRIPT_ANALYSIS,
        piX: *mut c_int,
    ) -> HRESULT;
    pub fn ScriptCacheGetHeight(
        hdc: HDC, psc: *mut SCRIPT_CACHE, tmHeight: *mut c_long,
    ) -> HRESULT;
    pub fn ScriptFreeCache(psc: *mut SCRIPT_CACHE) -> HRESULT;
    pub fn ScriptGetCMap(
        hdc: HDC, psc: *mut SCRIPT_CACHE, pwcInChars: *const WCHAR, cChars: c_int, dwFlags: DWORD,
        pwOutGlyphs: *mut WORD,
    ) -> HRESULT;
    pub fn ScriptGetFontAlternateGlyphs(
        hdc: HDC, psc: *mut SCRIPT_CACHE, psa: *mut SCRIPT_ANALYSIS, tagScript: OPENTYPE_TAG,
        tagLangSys: OPENTYPE_TAG, tagFeature: OPENTYPE_TAG, wGlyphId: WORD, cMaxAlternates: c_int,
        pAlternateGlyphs: *mut WORD, pcAlternates: *mut c_int,
    ) -> HRESULT;
    pub fn ScriptGetFontFeatureTags(
        hdc: HDC, psc: *mut SCRIPT_CACHE, psa: *mut SCRIPT_ANALYSIS, tagScript: OPENTYPE_TAG,
        tagLangSys: OPENTYPE_TAG, cMaxTags: c_int, pFeatureTags: *mut OPENTYPE_TAG,
        pcTags: *mut c_int,
    ) -> HRESULT;
    pub fn ScriptGetFontLanguageTags(
        hdc: HDC, psc: *mut SCRIPT_CACHE, psa: *mut SCRIPT_ANALYSIS, tagScript: OPENTYPE_TAG,
        cMaxTags: c_int, pLangsysTags: *mut OPENTYPE_TAG, pcTags: *mut c_int,
    ) -> HRESULT;
    pub fn ScriptGetFontProperties(
        hdc: HDC, psc: *mut SCRIPT_CACHE, sfp: *mut SCRIPT_FONTPROPERTIES,
    ) -> HRESULT;
    pub fn ScriptGetFontScriptTags(
        hdc: HDC, psc: *mut SCRIPT_CACHE, psa: *mut SCRIPT_ANALYSIS, cMaxTags: c_int,
        pScriptTags: *mut OPENTYPE_TAG, pcTags: *mut c_int,
    ) -> HRESULT;
    pub fn ScriptGetGlyphABCWidth(
        hdc: HDC, psc: *mut SCRIPT_CACHE, wGlyph: WORD, pABC: *mut ABC,
    ) -> HRESULT;
    pub fn ScriptGetLogicalWidths(
        psa: *const SCRIPT_ANALYSIS, cChars: c_int, cGlyphs: c_int, piGlyphWidth: *const c_int,
        pwLogClust: *const WORD, psva: *const SCRIPT_VISATTR, piDx: *mut c_int,
    ) -> HRESULT;
    pub fn ScriptGetProperties(
        ppSp: *mut *mut *const SCRIPT_PROPERTIES, piNumScripts: *mut c_int,
    ) -> HRESULT;
    pub fn ScriptIsComplex(pwcInChars: *const WCHAR, cInChars: c_int, dwFlags: DWORD) -> HRESULT;
    pub fn ScriptItemize(
        pwcInChars: *const WCHAR, cInChars: c_int, cMaxItems: c_int,
        psControl: *const SCRIPT_CONTROL, psState: *const SCRIPT_STATE, pItems: *mut SCRIPT_ITEM,
        pcItems: *mut c_int,
    ) -> HRESULT;
    pub fn ScriptItemizeOpenType(
        pwcInChars: *const WCHAR, cInChars: c_int, cMaxItems: c_int,
        psControl: *const SCRIPT_CONTROL, psState: *const SCRIPT_STATE, pItems: *mut SCRIPT_ITEM,
        pScriptTags: *mut OPENTYPE_TAG, pcItems: *mut c_int,
    ) -> HRESULT;
    pub fn ScriptJustify(
        psva: *const SCRIPT_VISATTR, piAdvance: *const c_int, cGlyphs: c_int, iDx: c_int,
        iMinKashida: c_int, piJustify: *mut c_int,
    ) -> HRESULT;
    pub fn ScriptLayout(
        cRuns: c_int, pbLevel: *const BYTE, piVisualToLogical: *mut c_int,
        piLogicalToVisual: *mut c_int,
    ) -> HRESULT;
    pub fn ScriptPlace(
        hdc: HDC, psc: *mut SCRIPT_CACHE, pwGlyphs: *const WORD, cGlyphs: c_int,
        psva: *const SCRIPT_VISATTR, psa: *mut SCRIPT_ANALYSIS, piAdvance: *mut c_int,
        pGoffset: *mut GOFFSET, pABC: *mut ABC,
    ) -> HRESULT;
    pub fn ScriptPlaceOpenType(
        hdc: HDC, psc: *mut SCRIPT_CACHE, psa: *mut SCRIPT_ANALYSIS, tagScript: OPENTYPE_TAG,
        tagLangSys: OPENTYPE_TAG, rcRangeChars: *mut c_int,
        rpRangeProperties: *mut *mut TEXTRANGE_PROPERTIES, cRanges: c_int, pwcChars: *const WCHAR,
        pwLogClust: *mut WORD, pCharProps: *mut SCRIPT_CHARPROP, cChars: c_int,
        pwGlyphs: *const WORD, pGlyphProps: *const SCRIPT_GLYPHPROP, cGlyphs: c_int,
        piAdvance: *mut c_int, pGoffset: *mut GOFFSET, pABC: *mut ABC,
    ) -> HRESULT;
    pub fn ScriptPositionSingleGlyph(
        hdc: HDC, psc: *mut SCRIPT_CACHE, psa: *mut SCRIPT_ANALYSIS, tagScript: OPENTYPE_TAG,
        tagLangSys: OPENTYPE_TAG, tagFeature: OPENTYPE_TAG, lParameter: LONG, wGlyphId: WORD,
        iAdvance: c_int, GOffset: GOFFSET, piOutAdvance: *mut c_int, pOutGoffset: *mut GOFFSET,
    ) -> HRESULT;
    pub fn ScriptRecordDigitSubstitution(
        Locale: LCID, psds: *mut SCRIPT_DIGITSUBSTITUTE,
    ) -> HRESULT;
    pub fn ScriptShape(
        hdc: HDC, psc: *mut SCRIPT_CACHE, pwcChars: *const WCHAR, cChars: c_int, cMaxGlyphs: c_int,
        psa: *mut SCRIPT_ANALYSIS, pwOutGlyphs: *mut WORD, pwLogClust: *mut WORD,
        psva: *mut SCRIPT_VISATTR, pcGlyphs: *mut c_int,
    ) -> HRESULT;
    pub fn ScriptShapeOpenType(
        hdc: HDC, psc: *mut SCRIPT_CACHE, psa: *mut SCRIPT_ANALYSIS, tagScript: OPENTYPE_TAG,
        tagLangSys: OPENTYPE_TAG, rcRangeChars: *mut c_int,
        rpRangeProperties: *mut *mut TEXTRANGE_PROPERTIES, cRanges: c_int, pwcChars: *const WCHAR,
        cChars: c_int, cMaxGlyphs: c_int, pwLogClust: *mut WORD, pCharProps: *mut SCRIPT_CHARPROP,
        pwOutGlyphs: *mut WORD, pOutGlyphProps: *mut SCRIPT_GLYPHPROP, pcGlyphs: *mut c_int,
    ) -> HRESULT;
    pub fn ScriptStringAnalyse(
        hdc: HDC, pString: *const c_void, cString: c_int, cGlyphs: c_int, iCharset: c_int,
        dwFlags: DWORD, iReqWidth: c_int, psControl: *mut SCRIPT_CONTROL,
        psState: *mut SCRIPT_STATE, piDx: *const c_int, pTabdef: *mut SCRIPT_TABDEF,
        pbInClass: *const BYTE, pssa: *mut SCRIPT_STRING_ANALYSIS,
    ) -> HRESULT;
    pub fn ScriptStringCPtoX(
        ssa: SCRIPT_STRING_ANALYSIS, icp: c_int, fTrailing: BOOL, pX: *mut c_int,
    ) -> HRESULT;
    pub fn ScriptStringFree(pssa: *mut SCRIPT_STRING_ANALYSIS) -> HRESULT;
    pub fn ScriptStringGetLogicalWidths(ssa: SCRIPT_STRING_ANALYSIS, piDx: *mut c_int) -> HRESULT;
    pub fn ScriptStringGetOrder(ssa: SCRIPT_STRING_ANALYSIS, puOrder: *mut UINT) -> HRESULT;
    pub fn ScriptStringOut(
        ssa: SCRIPT_STRING_ANALYSIS, iX: c_int, iY: c_int, uOptions: UINT, prc: *const RECT,
        iMinSel: c_int, iMaxSel: c_int, fDisabled: BOOL,
    ) -> HRESULT;
    pub fn ScriptStringValidate(ssa: SCRIPT_STRING_ANALYSIS) -> HRESULT;
    pub fn ScriptStringXtoCP(
        ssa: SCRIPT_STRING_ANALYSIS, iX: c_int, piCh: *mut c_int, piTrailing: *mut c_int,
    ) -> HRESULT;
    pub fn ScriptString_pLogAttr(ssa: SCRIPT_STRING_ANALYSIS) -> *const SCRIPT_LOGATTR;
    pub fn ScriptString_pSize(ssa: SCRIPT_STRING_ANALYSIS) -> *const SIZE;
    pub fn ScriptString_pcOutChars(ssa: SCRIPT_STRING_ANALYSIS) -> *const c_int;
    pub fn ScriptSubstituteSingleGlyph(
        hdc: HDC, psc: *mut SCRIPT_CACHE, psa: *mut SCRIPT_ANALYSIS, tagScript: OPENTYPE_TAG,
        tagLangSys: OPENTYPE_TAG, tagFeature: OPENTYPE_TAG, lParameter: LONG, wGlyphId: WORD,
        pwOutGlyphId: *mut WORD,
    ) -> HRESULT;
    pub fn ScriptTextOut(
        hdc: HDC, psc: *mut SCRIPT_CACHE, x: c_int, y: c_int, fuOptions: UINT, lprc: *const RECT,
        psa: *const SCRIPT_ANALYSIS, pwcReserved: *const WCHAR, iReserved: c_int,
        pwGlyphs: *const WORD, cGlyphs: c_int, piAdvance: *const c_int, piJustify: *const c_int,
        pGoffset: *const GOFFSET,
    ) -> HRESULT;
    pub fn ScriptXtoCP(
        iX: c_int, cChars: c_int, cGlyphs: c_int, pwLogClust: *const WORD,
        psva: *const SCRIPT_VISATTR, piAdvance: *const c_int, psa: *const SCRIPT_ANALYSIS,
        piCP: *mut c_int, piTrailing: *mut c_int,
    ) -> HRESULT;
}

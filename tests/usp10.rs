// Copyright © 2015, Jordan Miner
// Licensed under the MIT License <LICENSE.md>
#![cfg(windows)]
extern crate usp10;
use usp10::*;
#[inline(never)] fn bb<T>(_: T) {}
#[test]
fn functions() {
    bb(ScriptApplyDigitSubstitution);
    bb(ScriptApplyLogicalWidth);
    bb(ScriptBreak);
    bb(ScriptCPtoX);
    bb(ScriptCacheGetHeight);
    bb(ScriptFreeCache);
    bb(ScriptGetCMap);
    bb(ScriptGetFontAlternateGlyphs);
    bb(ScriptGetFontFeatureTags);
    bb(ScriptGetFontLanguageTags);
    bb(ScriptGetFontProperties);
    bb(ScriptGetFontScriptTags);
    bb(ScriptGetGlyphABCWidth);
    bb(ScriptGetLogicalWidths);
    bb(ScriptGetProperties);
    bb(ScriptIsComplex);
    bb(ScriptItemize);
    bb(ScriptItemizeOpenType);
    bb(ScriptJustify);
    bb(ScriptLayout);
    bb(ScriptPlace);
    bb(ScriptPlaceOpenType);
    bb(ScriptPositionSingleGlyph);
    bb(ScriptRecordDigitSubstitution);
    bb(ScriptShape);
    bb(ScriptShapeOpenType);
    bb(ScriptStringAnalyse);
    bb(ScriptStringCPtoX);
    bb(ScriptStringFree);
    bb(ScriptStringGetLogicalWidths);
    bb(ScriptStringGetOrder);
    bb(ScriptStringOut);
    bb(ScriptStringValidate);
    bb(ScriptStringXtoCP);
    bb(ScriptString_pLogAttr);
    bb(ScriptString_pSize);
    bb(ScriptString_pcOutChars);
    bb(ScriptSubstituteSingleGlyph);
    bb(ScriptTextOut);
    bb(ScriptXtoCP);
}

// Copyright © 2018 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use ctypes::c_void;
use shared::basetsd::{UINT16, ULONG_PTR};
use shared::guiddef::{CLSID, GUID};
use shared::minwindef::{BOOL, BYTE, HINSTANCE, HMETAFILE, HRGN, INT, LPBYTE, UINT};
use shared::windef::{HBITMAP, HDC, HENHMETAFILE, HICON, HPALETTE, HWND, RECT};
use shared::wtypes::PROPID;
use um::ddraw::IDirectDrawSurface7;
use um::gdipluscolor::ColorChannelFlags;
use um::gdipluscolormatrix::{
    ColorAdjustType, ColorMap, ColorMatrix, ColorMatrixFlags, HistogramFormat
};
use um::gdipluseffects::CGpEffect;
use um::gdiplusenums::{
    CombineMode, CompositingMode, CompositingQuality, CustomLineCapType, EmfPlusRecordType,
    EmfType, GpTestControlEnum, GraphicsContainer, GraphicsState, ImageType, InterpolationMode,
    LinearGradientMode, MetafileFrameUnit, PixelOffsetMode, SmoothingMode, StringAlignment,
    StringDigitSubstitute, StringTrimming, TextRenderingHint, Unit, WarpMode, WrapMode
};
use um::gdiplusgpstubs::{
    GpAdjustableArrowCap, GpBitmap, GpBrush, GpBrushType, GpCachedBitmap, GpCoordinateSpace,
    GpCustomLineCap, GpDashCap, GpDashStyle, GpFillMode, GpFlushIntention, GpFont,
    GpFontCollection, GpFontFamily, GpGraphics, GpHatch, GpHatchStyle, GpImage, GpImageAttributes,
    GpLineCap, GpLineGradient, GpLineJoin, GpMatrix, GpMatrixOrder, GpMetafile, GpPath, GpPathData,
    GpPathGradient, GpPathIterator, GpPen, GpPenAlignment, GpPenType, GpPoint, GpPointF, GpRect,
    GpRectF, GpRegion, GpSolidFill, GpStatus, GpStringFormat, GpTexture, GpUnit, GpWrapMode
};
use um::gdiplusimaging::{
    BitmapData, EncoderParameters, ImageCodecInfo, ImageItemData, PropertyItem, RotateFlipType
};
use um::gdiplusmetaheader::{MetafileHeader, WmfPlaceableFileHeader};
use um::gdipluspixelformats::{ARGB, ColorPalette, DitherType, PaletteType, PixelFormat};
use um::gdiplustypes::{
    CharacterRange, DrawImageAbort, EnumerateMetafileProc, GetThumbnailImageAbort, Point, PointF,
    REAL, Rect, RectF
};
use um::objidlbase::IStream;
use um::wingdi::{BITMAPINFO, LOGFONTA, LOGFONTW};
use um::winnt::{HANDLE, LANGID, LPWSTR, VOID, WCHAR};
extern "system" {
    pub fn GdipCreatePath(
        brushMode: GpFillMode,
        path: *mut *mut GpPath,
    ) -> GpStatus;
    pub fn GdipCreatePath2(
        _: *const GpPointF,
        _: *const BYTE,
        _: INT,
        _: GpFillMode,
        path: *mut *mut GpPath,
    ) -> GpStatus;
    pub fn GdipCreatePath2I(
        _: *const GpPoint,
        _: *const BYTE,
        _: INT,
        _: GpFillMode,
        path: *mut *mut GpPath,
    ) -> GpStatus;
    pub fn GdipClonePath(
        path: *mut GpPath,
        clonePath: *mut *mut GpPath,
    ) -> GpStatus;
    pub fn GdipDeletePath(
        path: *mut GpPath,
    ) -> GpStatus;
    pub fn GdipResetPath(
        path: *mut GpPath,
    ) -> GpStatus;
    pub fn GdipGetPointCount(
        path: *mut GpPath,
        count: *mut INT,
    ) -> GpStatus;
    pub fn GdipGetPathTypes(
        path: *mut GpPath,
        types: *mut BYTE,
        count: INT,
    ) -> GpStatus;
    pub fn GdipGetPathPoints(
        _: *mut GpPath,
        points: *mut GpPointF,
        count: INT,
    ) -> GpStatus;
    pub fn GdipGetPathPointsI(
        _: *mut GpPath,
        points: *mut GpPoint,
        count: INT,
    ) -> GpStatus;
    pub fn GdipGetPathFillMode(
        path: *mut GpPath,
        fillmode: *mut GpFillMode,
    ) -> GpStatus;
    pub fn GdipSetPathFillMode(
        path: *mut GpPath,
        fillmode: GpFillMode,
    ) -> GpStatus;
    pub fn GdipGetPathData(
        path: *mut GpPath,
        pathData: *mut GpPathData,
    ) -> GpStatus;
    pub fn GdipStartPathFigure(
        path: *mut GpPath,
    ) -> GpStatus;
    pub fn GdipClosePathFigure(
        path: *mut GpPath,
    ) -> GpStatus;
    pub fn GdipClosePathFigures(
        path: *mut GpPath,
    ) -> GpStatus;
    pub fn GdipSetPathMarker(
        path: *mut GpPath,
    ) -> GpStatus;
    pub fn GdipClearPathMarkers(
        path: *mut GpPath,
    ) -> GpStatus;
    pub fn GdipReversePath(
        path: *mut GpPath,
    ) -> GpStatus;
    pub fn GdipGetPathLastPoint(
        path: *mut GpPath,
        lastPoint: *mut GpPointF,
    ) -> GpStatus;
    pub fn GdipAddPathLine(
        path: *mut GpPath,
        x1: REAL,
        y1: REAL,
        x2: REAL,
        y2: REAL,
    ) -> GpStatus;
    pub fn GdipAddPathLine2(
        path: *mut GpPath,
        points: *const GpPointF,
        count: INT,
    ) -> GpStatus;
    pub fn GdipAddPathArc(
        path: *mut GpPath,
        x: REAL,
        y: REAL,
        width: REAL,
        height: REAL,
        startAngle: REAL,
        sweepAngle: REAL,
    ) -> GpStatus;
    pub fn GdipAddPathBezier(
        path: *mut GpPath,
        x1: REAL,
        y1: REAL,
        x2: REAL,
        y2: REAL,
        x3: REAL,
        y3: REAL,
        x4: REAL,
        y4: REAL,
    ) -> GpStatus;
    pub fn GdipAddPathBeziers(
        path: *mut GpPath,
        points: *const GpPointF,
        count: INT,
    ) -> GpStatus;
    pub fn GdipAddPathCurve(
        path: *mut GpPath,
        points: *const GpPointF,
        count: INT,
    ) -> GpStatus;
    pub fn GdipAddPathCurve2(
        path: *mut GpPath,
        points: *const GpPointF,
        count: INT,
        tension: REAL,
    ) -> GpStatus;
    pub fn GdipAddPathCurve3(
        path: *mut GpPath,
        points: *const GpPointF,
        count: INT,
        offset: INT,
        numberOfSegments: INT,
        tension: REAL,
    ) -> GpStatus;
    pub fn GdipAddPathClosedCurve(
        path: *mut GpPath,
        points: *const GpPointF,
        count: INT,
    ) -> GpStatus;
    pub fn GdipAddPathClosedCurve2(
        path: *mut GpPath,
        points: *const GpPointF,
        count: INT,
        tension: REAL,
    ) -> GpStatus;
    pub fn GdipAddPathRectangle(
        path: *mut GpPath,
        x: REAL,
        y: REAL,
        width: REAL,
        height: REAL,
    ) -> GpStatus;
    pub fn GdipAddPathRectangles(
        path: *mut GpPath,
        rects: *const GpRectF,
        count: INT,
    ) -> GpStatus;
    pub fn GdipAddPathEllipse(
        path: *mut GpPath,
        x: REAL,
        y: REAL,
        width: REAL,
        height: REAL,
    ) -> GpStatus;
    pub fn GdipAddPathPie(
        path: *mut GpPath,
        x: REAL,
        y: REAL,
        width: REAL,
        height: REAL,
        startAngle: REAL,
        sweepAngle: REAL,
    ) -> GpStatus;
    pub fn GdipAddPathPolygon(
        path: *mut GpPath,
        points: *const GpPointF,
        count: INT,
    ) -> GpStatus;
    pub fn GdipAddPathPath(
        path: *mut GpPath,
        addingPath: *const GpPath,
        connect: BOOL,
    ) -> GpStatus;
    pub fn GdipAddPathString(
        path: *mut GpPath,
        string: *const WCHAR,
        length: INT,
        family: *const GpFontFamily,
        style: INT,
        emSize: REAL,
        layoutRect: *const RectF,
        format: *const GpStringFormat,
    ) -> GpStatus;
    pub fn GdipAddPathStringI(
        path: *mut GpPath,
        string: *const WCHAR,
        length: INT,
        family: *const GpFontFamily,
        style: INT,
        emSize: REAL,
        layoutRect: *const Rect,
        format: *const GpStringFormat,
    ) -> GpStatus;
    pub fn GdipAddPathLineI(
        path: *mut GpPath,
        x1: INT,
        y1: INT,
        x2: INT,
        y2: INT,
    ) -> GpStatus;
    pub fn GdipAddPathLine2I(
        path: *mut GpPath,
        points: *const GpPoint,
        count: INT,
    ) -> GpStatus;
    pub fn GdipAddPathArcI(
        path: *mut GpPath,
        x: INT,
        y: INT,
        width: INT,
        height: INT,
        startAngle: REAL,
        sweepAngle: REAL,
    ) -> GpStatus;
    pub fn GdipAddPathBezierI(
        path: *mut GpPath,
        x1: INT,
        y1: INT,
        x2: INT,
        y2: INT,
        x3: INT,
        y3: INT,
        x4: INT,
        y4: INT,
    ) -> GpStatus;
    pub fn GdipAddPathBeziersI(
        path: *mut GpPath,
        points: *const GpPoint,
        count: INT,
    ) -> GpStatus;
    pub fn GdipAddPathCurveI(
        path: *mut GpPath,
        points: *const GpPoint,
        count: INT,
    ) -> GpStatus;
    pub fn GdipAddPathCurve2I(
        path: *mut GpPath,
        points: *const GpPoint,
        count: INT,
        tension: REAL,
    ) -> GpStatus;
    pub fn GdipAddPathCurve3I(
        path: *mut GpPath,
        points: *const GpPoint,
        count: INT,
        offset: INT,
        numberOfSegments: INT,
        tension: REAL,
    ) -> GpStatus;
    pub fn GdipAddPathClosedCurveI(
        path: *mut GpPath,
        points: *const GpPoint,
        count: INT,
    ) -> GpStatus;
    pub fn GdipAddPathClosedCurve2I(
        path: *mut GpPath,
        points: *const GpPoint,
        count: INT,
        tension: REAL,
    ) -> GpStatus;
    pub fn GdipAddPathRectangleI(
        path: *mut GpPath,
        x: INT,
        y: INT,
        width: INT,
        height: INT,
    ) -> GpStatus;
    pub fn GdipAddPathRectanglesI(
        path: *mut GpPath,
        rects: *const GpRect,
        count: INT,
    ) -> GpStatus;
    pub fn GdipAddPathEllipseI(
        path: *mut GpPath,
        x: INT,
        y: INT,
        width: INT,
        height: INT,
    ) -> GpStatus;
    pub fn GdipAddPathPieI(
        path: *mut GpPath,
        x: INT,
        y: INT,
        width: INT,
        height: INT,
        startAngle: REAL,
        sweepAngle: REAL,
    ) -> GpStatus;
    pub fn GdipAddPathPolygonI(
        path: *mut GpPath,
        points: *const GpPoint,
        count: INT,
    ) -> GpStatus;
    pub fn GdipFlattenPath(
        path: *mut GpPath,
        matrix: *mut GpMatrix,
        flatness: REAL,
    ) -> GpStatus;
    pub fn GdipWindingModeOutline(
        path: *mut GpPath,
        matrix: *mut GpMatrix,
        flatness: REAL,
    ) -> GpStatus;
    pub fn GdipWidenPath(
        nativePath: *mut GpPath,
        pen: *mut GpPen,
        matrix: *mut GpMatrix,
        flatness: REAL,
    ) -> GpStatus;
    pub fn GdipWarpPath(
        path: *mut GpPath,
        matrix: *mut GpMatrix,
        points: *const GpPointF,
        count: INT,
        srcx: REAL,
        srcy: REAL,
        srcwidth: REAL,
        srcheight: REAL,
        warpMode: WarpMode,
        flatness: REAL,
    ) -> GpStatus;
    pub fn GdipTransformPath(
        path: *mut GpPath,
        matrix: *mut GpMatrix,
    ) -> GpStatus;
    pub fn GdipGetPathWorldBounds(
        path: *mut GpPath,
        bounds: *mut GpRectF,
        matrix: *const GpMatrix,
        pen: *const GpPen,
    ) -> GpStatus;
    pub fn GdipGetPathWorldBoundsI(
        path: *mut GpPath,
        bounds: *mut GpRect,
        matrix: *const GpMatrix,
        pen: *const GpPen,
    ) -> GpStatus;
    pub fn GdipIsVisiblePathPoint(
        path: *mut GpPath,
        x: REAL,
        y: REAL,
        graphics: *mut GpGraphics,
        result: *mut BOOL,
    ) -> GpStatus;
    pub fn GdipIsVisiblePathPointI(
        path: *mut GpPath,
        x: INT,
        y: INT,
        graphics: *mut GpGraphics,
        result: *mut BOOL,
    ) -> GpStatus;
    pub fn GdipIsOutlineVisiblePathPoint(
        path: *mut GpPath,
        x: REAL,
        y: REAL,
        pen: *mut GpPen,
        graphics: *mut GpGraphics,
        result: *mut BOOL,
    ) -> GpStatus;
    pub fn GdipIsOutlineVisiblePathPointI(
        path: *mut GpPath,
        x: INT,
        y: INT,
        pen: *mut GpPen,
        graphics: *mut GpGraphics,
        result: *mut BOOL,
    ) -> GpStatus;
    pub fn GdipCreatePathIter(
        iterator: *mut *mut GpPathIterator,
        path: *mut GpPath,
    ) -> GpStatus;
    pub fn GdipDeletePathIter(
        iterator: *mut GpPathIterator,
    ) -> GpStatus;
    pub fn GdipPathIterNextSubpath(
        iterator: *mut GpPathIterator,
        resultCount: *mut INT,
        startIndex: *mut INT,
        endIndex: *mut INT,
        isClosed: *mut BOOL,
    ) -> GpStatus;
    pub fn GdipPathIterNextSubpathPath(
        iterator: *mut GpPathIterator,
        resultCount: *mut INT,
        path: *mut GpPath,
        isClosed: *mut BOOL,
    ) -> GpStatus;
    pub fn GdipPathIterNextPathType(
        iterator: *mut GpPathIterator,
        resultCount: *mut INT,
        pathType: *mut BYTE,
        startIndex: *mut INT,
        endIndex: *mut INT,
    ) -> GpStatus;
    pub fn GdipPathIterNextMarker(
        iterator: *mut GpPathIterator,
        resultCount: *mut INT,
        startIndex: *mut INT,
        endIndex: *mut INT,
    ) -> GpStatus;
    pub fn GdipPathIterNextMarkerPath(
        iterator: *mut GpPathIterator,
        resultCount: *mut INT,
        path: *mut GpPath,
    ) -> GpStatus;
    pub fn GdipPathIterGetCount(
        iterator: *mut GpPathIterator,
        count: *mut INT,
    ) -> GpStatus;
    pub fn GdipPathIterGetSubpathCount(
        iterator: *mut GpPathIterator,
        count: *mut INT,
    ) -> GpStatus;
    pub fn GdipPathIterIsValid(
        iterator: *mut GpPathIterator,
        valid: *mut BOOL,
    ) -> GpStatus;
    pub fn GdipPathIterHasCurve(
        iterator: *mut GpPathIterator,
        hasCurve: *mut BOOL,
    ) -> GpStatus;
    pub fn GdipPathIterRewind(
        iterator: *mut GpPathIterator,
    ) -> GpStatus;
    pub fn GdipPathIterEnumerate(
        iterator: *mut GpPathIterator,
        resultCount: *mut INT,
        points: *mut GpPointF,
        types: *mut BYTE,
        count: INT,
    ) -> GpStatus;
    pub fn GdipPathIterCopyData(
        iterator: *mut GpPathIterator,
        resultCount: *mut INT,
        points: *mut GpPointF,
        types: *mut BYTE,
        startIndex: INT,
        endIndex: INT,
    ) -> GpStatus;
    pub fn GdipCreateMatrix(
        matrix: *mut *mut GpMatrix,
    ) -> GpStatus;
    pub fn GdipCreateMatrix2(
        m11: REAL,
        m12: REAL,
        m21: REAL,
        m22: REAL,
        dx: REAL,
        dy: REAL,
        matrix: *mut *mut GpMatrix,
    ) -> GpStatus;
    pub fn GdipCreateMatrix3(
        rect: *const GpRectF,
        dstplg: *const GpPointF,
        matrix: *mut *mut GpMatrix,
    ) -> GpStatus;
    pub fn GdipCreateMatrix3I(
        rect: *const GpRect,
        dstplg: *const GpPoint,
        matrix: *mut *mut GpMatrix,
    ) -> GpStatus;
    pub fn GdipCloneMatrix(
        matrix: *mut GpMatrix,
        cloneMatrix: *mut *mut GpMatrix,
    ) -> GpStatus;
    pub fn GdipDeleteMatrix(
        matrix: *mut GpMatrix,
    ) -> GpStatus;
    pub fn GdipSetMatrixElements(
        matrix: *mut GpMatrix,
        m11: REAL,
        m12: REAL,
        m21: REAL,
        m22: REAL,
        dx: REAL,
        dy: REAL,
    ) -> GpStatus;
    pub fn GdipMultiplyMatrix(
        matrix: *mut GpMatrix,
        matrix2: *mut GpMatrix,
        order: GpMatrixOrder,
    ) -> GpStatus;
    pub fn GdipTranslateMatrix(
        matrix: *mut GpMatrix,
        offsetX: REAL,
        offsetY: REAL,
        order: GpMatrixOrder,
    ) -> GpStatus;
    pub fn GdipScaleMatrix(
        matrix: *mut GpMatrix,
        scaleX: REAL,
        scaleY: REAL,
        order: GpMatrixOrder,
    ) -> GpStatus;
    pub fn GdipRotateMatrix(
        matrix: *mut GpMatrix,
        angle: REAL,
        order: GpMatrixOrder,
    ) -> GpStatus;
    pub fn GdipShearMatrix(
        matrix: *mut GpMatrix,
        shearX: REAL,
        shearY: REAL,
        order: GpMatrixOrder,
    ) -> GpStatus;
    pub fn GdipInvertMatrix(
        matrix: *mut GpMatrix,
    ) -> GpStatus;
    pub fn GdipTransformMatrixPoints(
        matrix: *mut GpMatrix,
        pts: *mut GpPointF,
        count: INT,
    ) -> GpStatus;
    pub fn GdipTransformMatrixPointsI(
        matrix: *mut GpMatrix,
        pts: *mut GpPoint,
        count: INT,
    ) -> GpStatus;
    pub fn GdipVectorTransformMatrixPoints(
        matrix: *mut GpMatrix,
        pts: *mut GpPointF,
        count: INT,
    ) -> GpStatus;
    pub fn GdipVectorTransformMatrixPointsI(
        matrix: *mut GpMatrix,
        pts: *mut GpPoint,
        count: INT,
    ) -> GpStatus;
    pub fn GdipGetMatrixElements(
        matrix: *const GpMatrix,
        matrixOut: *mut REAL,
    ) -> GpStatus;
    pub fn GdipIsMatrixInvertible(
        matrix: *const GpMatrix,
        result: *mut BOOL,
    ) -> GpStatus;
    pub fn GdipIsMatrixIdentity(
        matrix: *const GpMatrix,
        result: *mut BOOL,
    ) -> GpStatus;
    pub fn GdipIsMatrixEqual(
        matrix: *const GpMatrix,
        matrix2: *const GpMatrix,
        result: *mut BOOL,
    ) -> GpStatus;
    pub fn GdipCreateRegion(
        region: *mut *mut GpRegion,
    ) -> GpStatus;
    pub fn GdipCreateRegionRect(
        rect: *const GpRectF,
        region: *mut *mut GpRegion,
    ) -> GpStatus;
    pub fn GdipCreateRegionRectI(
        rect: *const GpRect,
        region: *mut *mut GpRegion,
    ) -> GpStatus;
    pub fn GdipCreateRegionPath(
        path: *mut GpPath,
        region: *mut *mut GpRegion,
    ) -> GpStatus;
    pub fn GdipCreateRegionRgnData(
        regionData: *const BYTE,
        size: INT,
        region: *mut *mut GpRegion,
    ) -> GpStatus;
    pub fn GdipCreateRegionHrgn(
        hRgn: HRGN,
        region: *mut *mut GpRegion,
    ) -> GpStatus;
    pub fn GdipCloneRegion(
        region: *mut GpRegion,
        cloneRegion: *mut *mut GpRegion,
    ) -> GpStatus;
    pub fn GdipDeleteRegion(
        region: *mut GpRegion,
    ) -> GpStatus;
    pub fn GdipSetInfinite(
        region: *mut GpRegion,
    ) -> GpStatus;
    pub fn GdipSetEmpty(
        region: *mut GpRegion,
    ) -> GpStatus;
    pub fn GdipCombineRegionRect(
        region: *mut GpRegion,
        rect: *const GpRectF,
        combineMode: CombineMode,
    ) -> GpStatus;
    pub fn GdipCombineRegionRectI(
        region: *mut GpRegion,
        rect: *const GpRect,
        combineMode: CombineMode,
    ) -> GpStatus;
    pub fn GdipCombineRegionPath(
        region: *mut GpRegion,
        path: *mut GpPath,
        combineMode: CombineMode,
    ) -> GpStatus;
    pub fn GdipCombineRegionRegion(
        region: *mut GpRegion,
        region2: *mut GpRegion,
        combineMode: CombineMode,
    ) -> GpStatus;
    pub fn GdipTranslateRegion(
        region: *mut GpRegion,
        dx: REAL,
        dy: REAL,
    ) -> GpStatus;
    pub fn GdipTranslateRegionI(
        region: *mut GpRegion,
        dx: INT,
        dy: INT,
    ) -> GpStatus;
    pub fn GdipTransformRegion(
        region: *mut GpRegion,
        matrix: *mut GpMatrix,
    ) -> GpStatus;
    pub fn GdipGetRegionBounds(
        region: *mut GpRegion,
        graphics: *mut GpGraphics,
        rect: *mut GpRectF,
    ) -> GpStatus;
    pub fn GdipGetRegionBoundsI(
        region: *mut GpRegion,
        graphics: *mut GpGraphics,
        rect: *mut GpRect,
    ) -> GpStatus;
    pub fn GdipGetRegionHRgn(
        region: *mut GpRegion,
        graphics: *mut GpGraphics,
        hRgn: *mut HRGN,
    ) -> GpStatus;
    pub fn GdipIsEmptyRegion(
        region: *mut GpRegion,
        graphics: *mut GpGraphics,
        result: *mut BOOL,
    ) -> GpStatus;
    pub fn GdipIsInfiniteRegion(
        region: *mut GpRegion,
        graphics: *mut GpGraphics,
        result: *mut BOOL,
    ) -> GpStatus;
    pub fn GdipIsEqualRegion(
        region: *mut GpRegion,
        region2: *mut GpRegion,
        graphics: *mut GpGraphics,
        result: *mut BOOL,
    ) -> GpStatus;
    pub fn GdipGetRegionDataSize(
        region: *mut GpRegion,
        bufferSize: *mut UINT,
    ) -> GpStatus;
    pub fn GdipGetRegionData(
        region: *mut GpRegion,
        buffer: *mut BYTE,
        bufferSize: UINT,
        sizeFilled: *mut UINT,
    ) -> GpStatus;
    pub fn GdipIsVisibleRegionPoint(
        region: *mut GpRegion,
        x: REAL,
        y: REAL,
        graphics: *mut GpGraphics,
        result: *mut BOOL,
    ) -> GpStatus;
    pub fn GdipIsVisibleRegionPointI(
        region: *mut GpRegion,
        x: INT,
        y: INT,
        graphics: *mut GpGraphics,
        result: *mut BOOL,
    ) -> GpStatus;
    pub fn GdipIsVisibleRegionRect(
        region: *mut GpRegion,
        x: REAL,
        y: REAL,
        width: REAL,
        height: REAL,
        graphics: *mut GpGraphics,
        result: *mut BOOL,
    ) -> GpStatus;
    pub fn GdipIsVisibleRegionRectI(
        region: *mut GpRegion,
        x: INT,
        y: INT,
        width: INT,
        height: INT,
        graphics: *mut GpGraphics,
        result: *mut BOOL,
    ) -> GpStatus;
    pub fn GdipGetRegionScansCount(
        region: *mut GpRegion,
        count: *mut UINT,
        matrix: *mut GpMatrix,
    ) -> GpStatus;
    pub fn GdipGetRegionScans(
        region: *mut GpRegion,
        rects: *mut GpRectF,
        count: *mut INT,
        matrix: *mut GpMatrix,
    ) -> GpStatus;
    pub fn GdipGetRegionScansI(
        region: *mut GpRegion,
        rects: *mut GpRect,
        count: *mut INT,
        matrix: *mut GpMatrix,
    ) -> GpStatus;
    pub fn GdipCloneBrush(
        brush: *mut GpBrush,
        cloneBrush: *mut *mut GpBrush,
    ) -> GpStatus;
    pub fn GdipDeleteBrush(
        brush: *mut GpBrush,
    ) -> GpStatus;
    pub fn GdipGetBrushType(
        brush: *mut GpBrush,
        type_: *mut GpBrushType,
    ) -> GpStatus;
    pub fn GdipCreateHatchBrush(
        hatchstyle: GpHatchStyle,
        forecol: ARGB,
        backcol: ARGB,
        brush: *mut *mut GpHatch,
    ) -> GpStatus;
    pub fn GdipGetHatchStyle(
        brush: *mut GpHatch,
        hatchstyle: *mut GpHatchStyle,
    ) -> GpStatus;
    pub fn GdipGetHatchForegroundColor(
        brush: *mut GpHatch,
        forecol: *mut ARGB,
    ) -> GpStatus;
    pub fn GdipGetHatchBackgroundColor(
        brush: *mut GpHatch,
        backcol: *mut ARGB,
    ) -> GpStatus;
    pub fn GdipCreateTexture(
        image: *mut GpImage,
        wrapmode: GpWrapMode,
        texture: *mut *mut GpTexture,
    ) -> GpStatus;
    pub fn GdipCreateTexture2(
        image: *mut GpImage,
        wrapmode: GpWrapMode,
        x: REAL,
        y: REAL,
        width: REAL,
        height: REAL,
        texture: *mut *mut GpTexture,
    ) -> GpStatus;
    pub fn GdipCreateTextureIA(
        image: *mut GpImage,
        imageAttributes: *const GpImageAttributes,
        x: REAL,
        y: REAL,
        width: REAL,
        height: REAL,
        texture: *mut *mut GpTexture,
    ) -> GpStatus;
    pub fn GdipCreateTexture2I(
        image: *mut GpImage,
        wrapmode: GpWrapMode,
        x: INT,
        y: INT,
        width: INT,
        height: INT,
        texture: *mut *mut GpTexture,
    ) -> GpStatus;
    pub fn GdipCreateTextureIAI(
        image: *mut GpImage,
        imageAttributes: *const GpImageAttributes,
        x: INT,
        y: INT,
        width: INT,
        height: INT,
        texture: *mut *mut GpTexture,
    ) -> GpStatus;
    pub fn GdipGetTextureTransform(
        brush: *mut GpTexture,
        matrix: *mut GpMatrix,
    ) -> GpStatus;
    pub fn GdipSetTextureTransform(
        brush: *mut GpTexture,
        matrix: *const GpMatrix,
    ) -> GpStatus;
    pub fn GdipResetTextureTransform(
        brush: *mut GpTexture,
    ) -> GpStatus;
    pub fn GdipMultiplyTextureTransform(
        brush: *mut GpTexture,
        matrix: *const GpMatrix,
        order: GpMatrixOrder,
    ) -> GpStatus;
    pub fn GdipTranslateTextureTransform(
        brush: *mut GpTexture,
        dx: REAL,
        dy: REAL,
        order: GpMatrixOrder,
    ) -> GpStatus;
    pub fn GdipScaleTextureTransform(
        brush: *mut GpTexture,
        sx: REAL,
        sy: REAL,
        order: GpMatrixOrder,
    ) -> GpStatus;
    pub fn GdipRotateTextureTransform(
        brush: *mut GpTexture,
        angle: REAL,
        order: GpMatrixOrder,
    ) -> GpStatus;
    pub fn GdipSetTextureWrapMode(
        brush: *mut GpTexture,
        wrapmode: GpWrapMode,
    ) -> GpStatus;
    pub fn GdipGetTextureWrapMode(
        brush: *mut GpTexture,
        wrapmode: *mut GpWrapMode,
    ) -> GpStatus;
    pub fn GdipGetTextureImage(
        brush: *mut GpTexture,
        image: *mut *mut GpImage,
    ) -> GpStatus;
    pub fn GdipCreateSolidFill(
        color: ARGB,
        brush: *mut *mut GpSolidFill,
    ) -> GpStatus;
    pub fn GdipSetSolidFillColor(
        brush: *mut GpSolidFill,
        color: ARGB,
    ) -> GpStatus;
    pub fn GdipGetSolidFillColor(
        brush: *mut GpSolidFill,
        color: *mut ARGB,
    ) -> GpStatus;
    pub fn GdipCreateLineBrush(
        point1: *const GpPointF,
        point2: *const GpPointF,
        color1: ARGB,
        color2: ARGB,
        wrapMode: GpWrapMode,
        lineGradient: *mut *mut GpLineGradient,
    ) -> GpStatus;
    pub fn GdipCreateLineBrushI(
        point1: *const GpPoint,
        point2: *const GpPoint,
        color1: ARGB,
        color2: ARGB,
        wrapMode: GpWrapMode,
        lineGradient: *mut *mut GpLineGradient,
    ) -> GpStatus;
    pub fn GdipCreateLineBrushFromRect(
        rect: *const GpRectF,
        color1: ARGB,
        color2: ARGB,
        mode: LinearGradientMode,
        wrapMode: GpWrapMode,
        lineGradient: *mut *mut GpLineGradient,
    ) -> GpStatus;
    pub fn GdipCreateLineBrushFromRectI(
        rect: *const GpRect,
        color1: ARGB,
        color2: ARGB,
        mode: LinearGradientMode,
        wrapMode: GpWrapMode,
        lineGradient: *mut *mut GpLineGradient,
    ) -> GpStatus;
    pub fn GdipCreateLineBrushFromRectWithAngle(
        rect: *const GpRectF,
        color1: ARGB,
        color2: ARGB,
        angle: REAL,
        isAngleScalable: BOOL,
        wrapMode: GpWrapMode,
        lineGradient: *mut *mut GpLineGradient,
    ) -> GpStatus;
    pub fn GdipCreateLineBrushFromRectWithAngleI(
        rect: *const GpRect,
        color1: ARGB,
        color2: ARGB,
        angle: REAL,
        isAngleScalable: BOOL,
        wrapMode: GpWrapMode,
        lineGradient: *mut *mut GpLineGradient,
    ) -> GpStatus;
    pub fn GdipSetLineColors(
        brush: *mut GpLineGradient,
        color1: ARGB,
        color2: ARGB,
    ) -> GpStatus;
    pub fn GdipGetLineColors(
        brush: *mut GpLineGradient,
        colors: *mut ARGB,
    ) -> GpStatus;
    pub fn GdipGetLineRect(
        brush: *mut GpLineGradient,
        rect: *mut GpRectF,
    ) -> GpStatus;
    pub fn GdipGetLineRectI(
        brush: *mut GpLineGradient,
        rect: *mut GpRect,
    ) -> GpStatus;
    pub fn GdipSetLineGammaCorrection(
        brush: *mut GpLineGradient,
        useGammaCorrection: BOOL,
    ) -> GpStatus;
    pub fn GdipGetLineGammaCorrection(
        brush: *mut GpLineGradient,
        useGammaCorrection: *mut BOOL,
    ) -> GpStatus;
    pub fn GdipGetLineBlendCount(
        brush: *mut GpLineGradient,
        count: *mut INT,
    ) -> GpStatus;
    pub fn GdipGetLineBlend(
        brush: *mut GpLineGradient,
        blend: *mut REAL,
        positions: *mut REAL,
        count: INT,
    ) -> GpStatus;
    pub fn GdipSetLineBlend(
        brush: *mut GpLineGradient,
        blend: *const REAL,
        positions: *const REAL,
        count: INT,
    ) -> GpStatus;
    pub fn GdipGetLinePresetBlendCount(
        brush: *mut GpLineGradient,
        count: *mut INT,
    ) -> GpStatus;
    pub fn GdipGetLinePresetBlend(
        brush: *mut GpLineGradient,
        blend: *mut ARGB,
        positions: *mut REAL,
        count: INT,
    ) -> GpStatus;
    pub fn GdipSetLinePresetBlend(
        brush: *mut GpLineGradient,
        blend: *const ARGB,
        positions: *const REAL,
        count: INT,
    ) -> GpStatus;
    pub fn GdipSetLineSigmaBlend(
        brush: *mut GpLineGradient,
        focus: REAL,
        scale: REAL,
    ) -> GpStatus;
    pub fn GdipSetLineLinearBlend(
        brush: *mut GpLineGradient,
        focus: REAL,
        scale: REAL,
    ) -> GpStatus;
    pub fn GdipSetLineWrapMode(
        brush: *mut GpLineGradient,
        wrapmode: GpWrapMode,
    ) -> GpStatus;
    pub fn GdipGetLineWrapMode(
        brush: *mut GpLineGradient,
        wrapmode: *mut GpWrapMode,
    ) -> GpStatus;
    pub fn GdipGetLineTransform(
        brush: *mut GpLineGradient,
        matrix: *mut GpMatrix,
    ) -> GpStatus;
    pub fn GdipSetLineTransform(
        brush: *mut GpLineGradient,
        matrix: *const GpMatrix,
    ) -> GpStatus;
    pub fn GdipResetLineTransform(
        brush: *mut GpLineGradient,
    ) -> GpStatus;
    pub fn GdipMultiplyLineTransform(
        brush: *mut GpLineGradient,
        matrix: *const GpMatrix,
        order: GpMatrixOrder,
    ) -> GpStatus;
    pub fn GdipTranslateLineTransform(
        brush: *mut GpLineGradient,
        dx: REAL,
        dy: REAL,
        order: GpMatrixOrder,
    ) -> GpStatus;
    pub fn GdipScaleLineTransform(
        brush: *mut GpLineGradient,
        sx: REAL,
        sy: REAL,
        order: GpMatrixOrder,
    ) -> GpStatus;
    pub fn GdipRotateLineTransform(
        brush: *mut GpLineGradient,
        angle: REAL,
        order: GpMatrixOrder,
    ) -> GpStatus;
    pub fn GdipCreatePathGradient(
        points: *const GpPointF,
        count: INT,
        wrapMode: GpWrapMode,
        polyGradient: *mut *mut GpPathGradient,
    ) -> GpStatus;
    pub fn GdipCreatePathGradientI(
        points: *const GpPoint,
        count: INT,
        wrapMode: GpWrapMode,
        polyGradient: *mut *mut GpPathGradient,
    ) -> GpStatus;
    pub fn GdipCreatePathGradientFromPath(
        path: *const GpPath,
        polyGradient: *mut *mut GpPathGradient,
    ) -> GpStatus;
    pub fn GdipGetPathGradientCenterColor(
        brush: *mut GpPathGradient,
        colors: *mut ARGB,
    ) -> GpStatus;
    pub fn GdipSetPathGradientCenterColor(
        brush: *mut GpPathGradient,
        colors: ARGB,
    ) -> GpStatus;
    pub fn GdipGetPathGradientSurroundColorsWithCount(
        brush: *mut GpPathGradient,
        color: *mut ARGB,
        count: *mut INT,
    ) -> GpStatus;
    pub fn GdipSetPathGradientSurroundColorsWithCount(
        brush: *mut GpPathGradient,
        color: *const ARGB,
        count: *mut INT,
    ) -> GpStatus;
    pub fn GdipGetPathGradientPath(
        brush: *mut GpPathGradient,
        path: *mut GpPath,
    ) -> GpStatus;
    pub fn GdipSetPathGradientPath(
        brush: *mut GpPathGradient,
        path: *const GpPath,
    ) -> GpStatus;
    pub fn GdipGetPathGradientCenterPoint(
        brush: *mut GpPathGradient,
        points: *mut GpPointF,
    ) -> GpStatus;
    pub fn GdipGetPathGradientCenterPointI(
        brush: *mut GpPathGradient,
        points: *mut GpPoint,
    ) -> GpStatus;
    pub fn GdipSetPathGradientCenterPoint(
        brush: *mut GpPathGradient,
        points: *const GpPointF,
    ) -> GpStatus;
    pub fn GdipSetPathGradientCenterPointI(
        brush: *mut GpPathGradient,
        points: *const GpPoint,
    ) -> GpStatus;
    pub fn GdipGetPathGradientRect(
        brush: *mut GpPathGradient,
        rect: *mut GpRectF,
    ) -> GpStatus;
    pub fn GdipGetPathGradientRectI(
        brush: *mut GpPathGradient,
        rect: *mut GpRect,
    ) -> GpStatus;
    pub fn GdipGetPathGradientPointCount(
        brush: *mut GpPathGradient,
        count: *mut INT,
    ) -> GpStatus;
    pub fn GdipGetPathGradientSurroundColorCount(
        brush: *mut GpPathGradient,
        count: *mut INT,
    ) -> GpStatus;
    pub fn GdipSetPathGradientGammaCorrection(
        brush: *mut GpPathGradient,
        useGammaCorrection: BOOL,
    ) -> GpStatus;
    pub fn GdipGetPathGradientGammaCorrection(
        brush: *mut GpPathGradient,
        useGammaCorrection: *mut BOOL,
    ) -> GpStatus;
    pub fn GdipGetPathGradientBlendCount(
        brush: *mut GpPathGradient,
        count: *mut INT,
    ) -> GpStatus;
    pub fn GdipGetPathGradientBlend(
        brush: *mut GpPathGradient,
        blend: *mut REAL,
        positions: *mut REAL,
        count: INT,
    ) -> GpStatus;
    pub fn GdipSetPathGradientBlend(
        brush: *mut GpPathGradient,
        blend: *const REAL,
        positions: *const REAL,
        count: INT,
    ) -> GpStatus;
    pub fn GdipGetPathGradientPresetBlendCount(
        brush: *mut GpPathGradient,
        count: *mut INT,
    ) -> GpStatus;
    pub fn GdipGetPathGradientPresetBlend(
        brush: *mut GpPathGradient,
        blend: *mut ARGB,
        positions: *mut REAL,
        count: INT,
    ) -> GpStatus;
    pub fn GdipSetPathGradientPresetBlend(
        brush: *mut GpPathGradient,
        blend: *const ARGB,
        positions: *const REAL,
        count: INT,
    ) -> GpStatus;
    pub fn GdipSetPathGradientSigmaBlend(
        brush: *mut GpPathGradient,
        focus: REAL,
        scale: REAL,
    ) -> GpStatus;
    pub fn GdipSetPathGradientLinearBlend(
        brush: *mut GpPathGradient,
        focus: REAL,
        scale: REAL,
    ) -> GpStatus;
    pub fn GdipGetPathGradientWrapMode(
        brush: *mut GpPathGradient,
        wrapmode: *mut GpWrapMode,
    ) -> GpStatus;
    pub fn GdipSetPathGradientWrapMode(
        brush: *mut GpPathGradient,
        wrapmode: GpWrapMode,
    ) -> GpStatus;
    pub fn GdipGetPathGradientTransform(
        brush: *mut GpPathGradient,
        matrix: *mut GpMatrix,
    ) -> GpStatus;
    pub fn GdipSetPathGradientTransform(
        brush: *mut GpPathGradient,
        matrix: *mut GpMatrix,
    ) -> GpStatus;
    pub fn GdipResetPathGradientTransform(
        brush: *mut GpPathGradient,
    ) -> GpStatus;
    pub fn GdipMultiplyPathGradientTransform(
        brush: *mut GpPathGradient,
        matrix: *const GpMatrix,
        order: GpMatrixOrder,
    ) -> GpStatus;
    pub fn GdipTranslatePathGradientTransform(
        brush: *mut GpPathGradient,
        dx: REAL,
        dy: REAL,
        order: GpMatrixOrder,
    ) -> GpStatus;
    pub fn GdipScalePathGradientTransform(
        brush: *mut GpPathGradient,
        sx: REAL,
        sy: REAL,
        order: GpMatrixOrder,
    ) -> GpStatus;
    pub fn GdipRotatePathGradientTransform(
        brush: *mut GpPathGradient,
        angle: REAL,
        order: GpMatrixOrder,
    ) -> GpStatus;
    pub fn GdipGetPathGradientFocusScales(
        brush: *mut GpPathGradient,
        xScale: *mut REAL,
        yScale: *mut REAL,
    ) -> GpStatus;
    pub fn GdipSetPathGradientFocusScales(
        brush: *mut GpPathGradient,
        xScale: REAL,
        yScale: REAL,
    ) -> GpStatus;
    pub fn GdipCreatePen1(
        color: ARGB,
        width: REAL,
        unit: GpUnit,
        pen: *mut *mut GpPen,
    ) -> GpStatus;
    pub fn GdipCreatePen2(
        brush: *mut GpBrush,
        width: REAL,
        unit: GpUnit,
        pen: *mut *mut GpPen,
    ) -> GpStatus;
    pub fn GdipClonePen(
        pen: *mut GpPen,
        clonepen: *mut *mut GpPen,
    ) -> GpStatus;
    pub fn GdipDeletePen(
        pen: *mut GpPen,
    ) -> GpStatus;
    pub fn GdipSetPenWidth(
        pen: *mut GpPen,
        width: REAL,
    ) -> GpStatus;
    pub fn GdipGetPenWidth(
        pen: *mut GpPen,
        width: *mut REAL,
    ) -> GpStatus;
    pub fn GdipSetPenUnit(
        pen: *mut GpPen,
        unit: GpUnit,
    ) -> GpStatus;
    pub fn GdipGetPenUnit(
        pen: *mut GpPen,
        unit: *mut GpUnit,
    ) -> GpStatus;
    pub fn GdipSetPenLineCap197819(
        pen: *mut GpPen,
        startCap: GpLineCap,
        endCap: GpLineCap,
        dashCap: GpDashCap,
    ) -> GpStatus;
    pub fn GdipSetPenStartCap(
        pen: *mut GpPen,
        startCap: GpLineCap,
    ) -> GpStatus;
    pub fn GdipSetPenEndCap(
        pen: *mut GpPen,
        endCap: GpLineCap,
    ) -> GpStatus;
    pub fn GdipSetPenDashCap197819(
        pen: *mut GpPen,
        dashCap: GpDashCap,
    ) -> GpStatus;
    pub fn GdipGetPenStartCap(
        pen: *mut GpPen,
        startCap: *mut GpLineCap,
    ) -> GpStatus;
    pub fn GdipGetPenEndCap(
        pen: *mut GpPen,
        endCap: *mut GpLineCap,
    ) -> GpStatus;
    pub fn GdipGetPenDashCap197819(
        pen: *mut GpPen,
        dashCap: *mut GpDashCap,
    ) -> GpStatus;
    pub fn GdipSetPenLineJoin(
        pen: *mut GpPen,
        lineJoin: GpLineJoin,
    ) -> GpStatus;
    pub fn GdipGetPenLineJoin(
        pen: *mut GpPen,
        lineJoin: *mut GpLineJoin,
    ) -> GpStatus;
    pub fn GdipSetPenCustomStartCap(
        pen: *mut GpPen,
        customCap: *mut GpCustomLineCap,
    ) -> GpStatus;
    pub fn GdipGetPenCustomStartCap(
        pen: *mut GpPen,
        customCap: *mut *mut GpCustomLineCap,
    ) -> GpStatus;
    pub fn GdipSetPenCustomEndCap(
        pen: *mut GpPen,
        customCap: *mut GpCustomLineCap,
    ) -> GpStatus;
    pub fn GdipGetPenCustomEndCap(
        pen: *mut GpPen,
        customCap: *mut *mut GpCustomLineCap,
    ) -> GpStatus;
    pub fn GdipSetPenMiterLimit(
        pen: *mut GpPen,
        miterLimit: REAL,
    ) -> GpStatus;
    pub fn GdipGetPenMiterLimit(
        pen: *mut GpPen,
        miterLimit: *mut REAL,
    ) -> GpStatus;
    pub fn GdipSetPenMode(
        pen: *mut GpPen,
        penMode: GpPenAlignment,
    ) -> GpStatus;
    pub fn GdipGetPenMode(
        pen: *mut GpPen,
        penMode: *mut GpPenAlignment,
    ) -> GpStatus;
    pub fn GdipSetPenTransform(
        pen: *mut GpPen,
        matrix: *mut GpMatrix,
    ) -> GpStatus;
    pub fn GdipGetPenTransform(
        pen: *mut GpPen,
        matrix: *mut GpMatrix,
    ) -> GpStatus;
    pub fn GdipResetPenTransform(
        pen: *mut GpPen,
    ) -> GpStatus;
    pub fn GdipMultiplyPenTransform(
        pen: *mut GpPen,
        matrix: *const GpMatrix,
        order: GpMatrixOrder,
    ) -> GpStatus;
    pub fn GdipTranslatePenTransform(
        pen: *mut GpPen,
        dx: REAL,
        dy: REAL,
        order: GpMatrixOrder,
    ) -> GpStatus;
    pub fn GdipScalePenTransform(
        pen: *mut GpPen,
        sx: REAL,
        sy: REAL,
        order: GpMatrixOrder,
    ) -> GpStatus;
    pub fn GdipRotatePenTransform(
        pen: *mut GpPen,
        angle: REAL,
        order: GpMatrixOrder,
    ) -> GpStatus;
    pub fn GdipSetPenColor(
        pen: *mut GpPen,
        argb: ARGB,
    ) -> GpStatus;
    pub fn GdipGetPenColor(
        pen: *mut GpPen,
        argb: *mut ARGB,
    ) -> GpStatus;
    pub fn GdipSetPenBrushFill(
        pen: *mut GpPen,
        brush: *mut GpBrush,
    ) -> GpStatus;
    pub fn GdipGetPenBrushFill(
        pen: *mut GpPen,
        brush: *mut *mut GpBrush,
    ) -> GpStatus;
    pub fn GdipGetPenFillType(
        pen: *mut GpPen,
        type_: *mut GpPenType,
    ) -> GpStatus;
    pub fn GdipGetPenDashStyle(
        pen: *mut GpPen,
        dashstyle: *mut GpDashStyle,
    ) -> GpStatus;
    pub fn GdipSetPenDashStyle(
        pen: *mut GpPen,
        dashstyle: GpDashStyle,
    ) -> GpStatus;
    pub fn GdipGetPenDashOffset(
        pen: *mut GpPen,
        offset: *mut REAL,
    ) -> GpStatus;
    pub fn GdipSetPenDashOffset(
        pen: *mut GpPen,
        offset: REAL,
    ) -> GpStatus;
    pub fn GdipGetPenDashCount(
        pen: *mut GpPen,
        count: *mut INT,
    ) -> GpStatus;
    pub fn GdipSetPenDashArray(
        pen: *mut GpPen,
        dash: *const REAL,
        count: INT,
    ) -> GpStatus;
    pub fn GdipGetPenDashArray(
        pen: *mut GpPen,
        dash: *mut REAL,
        count: INT,
    ) -> GpStatus;
    pub fn GdipGetPenCompoundCount(
        pen: *mut GpPen,
        count: *mut INT,
    ) -> GpStatus;
    pub fn GdipSetPenCompoundArray(
        pen: *mut GpPen,
        dash: *const REAL,
        count: INT,
    ) -> GpStatus;
    pub fn GdipGetPenCompoundArray(
        pen: *mut GpPen,
        dash: *mut REAL,
        count: INT,
    ) -> GpStatus;
    pub fn GdipCreateCustomLineCap(
        fillPath: *mut GpPath,
        strokePath: *mut GpPath,
        baseCap: GpLineCap,
        baseInset: REAL,
        customCap: *mut *mut GpCustomLineCap,
    ) -> GpStatus;
    pub fn GdipDeleteCustomLineCap(
        customCap: *mut GpCustomLineCap,
    ) -> GpStatus;
    pub fn GdipCloneCustomLineCap(
        customCap: *mut GpCustomLineCap,
        clonedCap: *mut *mut GpCustomLineCap,
    ) -> GpStatus;
    pub fn GdipGetCustomLineCapType(
        customCap: *mut GpCustomLineCap,
        capType: *mut CustomLineCapType,
    ) -> GpStatus;
    pub fn GdipSetCustomLineCapStrokeCaps(
        customCap: *mut GpCustomLineCap,
        startCap: GpLineCap,
        endCap: GpLineCap,
    ) -> GpStatus;
    pub fn GdipGetCustomLineCapStrokeCaps(
        customCap: *mut GpCustomLineCap,
        startCap: *mut GpLineCap,
        endCap: *mut GpLineCap,
    ) -> GpStatus;
    pub fn GdipSetCustomLineCapStrokeJoin(
        customCap: *mut GpCustomLineCap,
        lineJoin: GpLineJoin,
    ) -> GpStatus;
    pub fn GdipGetCustomLineCapStrokeJoin(
        customCap: *mut GpCustomLineCap,
        lineJoin: *mut GpLineJoin,
    ) -> GpStatus;
    pub fn GdipSetCustomLineCapBaseCap(
        customCap: *mut GpCustomLineCap,
        baseCap: GpLineCap,
    ) -> GpStatus;
    pub fn GdipGetCustomLineCapBaseCap(
        customCap: *mut GpCustomLineCap,
        baseCap: *mut GpLineCap,
    ) -> GpStatus;
    pub fn GdipSetCustomLineCapBaseInset(
        customCap: *mut GpCustomLineCap,
        inset: REAL,
    ) -> GpStatus;
    pub fn GdipGetCustomLineCapBaseInset(
        customCap: *mut GpCustomLineCap,
        inset: *mut REAL,
    ) -> GpStatus;
    pub fn GdipSetCustomLineCapWidthScale(
        customCap: *mut GpCustomLineCap,
        widthScale: REAL,
    ) -> GpStatus;
    pub fn GdipGetCustomLineCapWidthScale(
        customCap: *mut GpCustomLineCap,
        widthScale: *mut REAL,
    ) -> GpStatus;
    pub fn GdipCreateAdjustableArrowCap(
        height: REAL,
        width: REAL,
        isFilled: BOOL,
        cap: *mut *mut GpAdjustableArrowCap,
    ) -> GpStatus;
    pub fn GdipSetAdjustableArrowCapHeight(
        cap: *mut GpAdjustableArrowCap,
        height: REAL,
    ) -> GpStatus;
    pub fn GdipGetAdjustableArrowCapHeight(
        cap: *mut GpAdjustableArrowCap,
        height: *mut REAL,
    ) -> GpStatus;
    pub fn GdipSetAdjustableArrowCapWidth(
        cap: *mut GpAdjustableArrowCap,
        width: REAL,
    ) -> GpStatus;
    pub fn GdipGetAdjustableArrowCapWidth(
        cap: *mut GpAdjustableArrowCap,
        width: *mut REAL,
    ) -> GpStatus;
    pub fn GdipSetAdjustableArrowCapMiddleInset(
        cap: *mut GpAdjustableArrowCap,
        middleInset: REAL,
    ) -> GpStatus;
    pub fn GdipGetAdjustableArrowCapMiddleInset(
        cap: *mut GpAdjustableArrowCap,
        middleInset: *mut REAL,
    ) -> GpStatus;
    pub fn GdipSetAdjustableArrowCapFillState(
        cap: *mut GpAdjustableArrowCap,
        fillState: BOOL,
    ) -> GpStatus;
    pub fn GdipGetAdjustableArrowCapFillState(
        cap: *mut GpAdjustableArrowCap,
        fillState: *mut BOOL,
    ) -> GpStatus;
    pub fn GdipLoadImageFromStream(
        stream: *mut IStream,
        image: *mut *mut GpImage,
    ) -> GpStatus;
    pub fn GdipLoadImageFromFile(
        filename: *const WCHAR,
        image: *mut *mut GpImage,
    ) -> GpStatus;
    pub fn GdipLoadImageFromStreamICM(
        stream: *mut IStream,
        image: *mut *mut GpImage,
    ) -> GpStatus;
    pub fn GdipLoadImageFromFileICM(
        filename: *const WCHAR,
        image: *mut *mut GpImage,
    ) -> GpStatus;
    pub fn GdipCloneImage(
        image: *mut GpImage,
        cloneImage: *mut *mut GpImage,
    ) -> GpStatus;
    pub fn GdipDisposeImage(
        image: *mut GpImage,
    ) -> GpStatus;
    pub fn GdipSaveImageToFile(
        image: *mut GpImage,
        filename: *const WCHAR,
        clsidEncoder: *const CLSID,
        encoderParams: *const EncoderParameters,
    ) -> GpStatus;
    pub fn GdipSaveImageToStream(
        image: *mut GpImage,
        stream: *mut IStream,
        clsidEncoder: *const CLSID,
        encoderParams: *const EncoderParameters,
    ) -> GpStatus;
    pub fn GdipSaveAdd(
        image: *mut GpImage,
        encoderParams: *const EncoderParameters,
    ) -> GpStatus;
    pub fn GdipSaveAddImage(
        image: *mut GpImage,
        newImage: *mut GpImage,
        encoderParams: *const EncoderParameters,
    ) -> GpStatus;
    pub fn GdipGetImageGraphicsContext(
        image: *mut GpImage,
        graphics: *mut *mut GpGraphics,
    ) -> GpStatus;
    pub fn GdipGetImageBounds(
        image: *mut GpImage,
        srcRect: *mut GpRectF,
        srcUnit: *mut GpUnit,
    ) -> GpStatus;
    pub fn GdipGetImageDimension(
        image: *mut GpImage,
        width: *mut REAL,
        height: *mut REAL,
    ) -> GpStatus;
    pub fn GdipGetImageType(
        image: *mut GpImage,
        type_: *mut ImageType,
    ) -> GpStatus;
    pub fn GdipGetImageWidth(
        image: *mut GpImage,
        width: *mut UINT,
    ) -> GpStatus;
    pub fn GdipGetImageHeight(
        image: *mut GpImage,
        height: *mut UINT,
    ) -> GpStatus;
    pub fn GdipGetImageHorizontalResolution(
        image: *mut GpImage,
        resolution: *mut REAL,
    ) -> GpStatus;
    pub fn GdipGetImageVerticalResolution(
        image: *mut GpImage,
        resolution: *mut REAL,
    ) -> GpStatus;
    pub fn GdipGetImageFlags(
        image: *mut GpImage,
        flags: *mut UINT,
    ) -> GpStatus;
    pub fn GdipGetImageRawFormat(
        image: *mut GpImage,
        format: *mut GUID,
    ) -> GpStatus;
    pub fn GdipGetImagePixelFormat(
        image: *mut GpImage,
        format: *mut PixelFormat,
    ) -> GpStatus;
    pub fn GdipGetImageThumbnail(
        image: *mut GpImage,
        thumbWidth: UINT,
        thumbHeight: UINT,
        thumbImage: *mut *mut GpImage,
        callback: GetThumbnailImageAbort,
        callbackData: *mut VOID,
    ) -> GpStatus;
    pub fn GdipGetEncoderParameterListSize(
        image: *mut GpImage,
        clsidEncoder: *const CLSID,
        size: *mut UINT,
    ) -> GpStatus;
    pub fn GdipGetEncoderParameterList(
        image: *mut GpImage,
        clsidEncoder: *const CLSID,
        size: UINT,
        buffer: *mut EncoderParameters,
    ) -> GpStatus;
    pub fn GdipImageGetFrameDimensionsCount(
        image: *mut GpImage,
        count: *mut UINT,
    ) -> GpStatus;
    pub fn GdipImageGetFrameDimensionsList(
        image: *mut GpImage,
        dimensionIDs: *mut GUID,
        count: UINT,
    ) -> GpStatus;
    pub fn GdipImageGetFrameCount(
        image: *mut GpImage,
        dimensionID: *const GUID,
        count: *mut UINT,
    ) -> GpStatus;
    pub fn GdipImageSelectActiveFrame(
        image: *mut GpImage,
        dimensionID: *const GUID,
        frameIndex: UINT,
    ) -> GpStatus;
    pub fn GdipImageRotateFlip(
        image: *mut GpImage,
        rfType: RotateFlipType,
    ) -> GpStatus;
    pub fn GdipGetImagePalette(
        image: *mut GpImage,
        palette: *mut ColorPalette,
        size: INT,
    ) -> GpStatus;
    pub fn GdipSetImagePalette(
        image: *mut GpImage,
        palette: *const ColorPalette,
    ) -> GpStatus;
    pub fn GdipGetImagePaletteSize(
        image: *mut GpImage,
        size: *mut INT,
    ) -> GpStatus;
    pub fn GdipGetPropertyCount(
        image: *mut GpImage,
        numOfProperty: *mut UINT,
    ) -> GpStatus;
    pub fn GdipGetPropertyIdList(
        image: *mut GpImage,
        numOfProperty: UINT,
        list: *mut PROPID,
    ) -> GpStatus;
    pub fn GdipGetPropertyItemSize(
        image: *mut GpImage,
        propId: PROPID,
        size: *mut UINT,
    ) -> GpStatus;
    pub fn GdipGetPropertyItem(
        image: *mut GpImage,
        propId: PROPID,
        propSize: UINT,
        buffer: *mut PropertyItem,
    ) -> GpStatus;
    pub fn GdipGetPropertySize(
        image: *mut GpImage,
        totalBufferSize: *mut UINT,
        numProperties: *mut UINT,
    ) -> GpStatus;
    pub fn GdipGetAllPropertyItems(
        image: *mut GpImage,
        totalBufferSize: UINT,
        numProperties: UINT,
        allItems: *mut PropertyItem,
    ) -> GpStatus;
    pub fn GdipRemovePropertyItem(
        image: *mut GpImage,
        propId: PROPID,
    ) -> GpStatus;
    pub fn GdipSetPropertyItem(
        image: *mut GpImage,
        item: *const PropertyItem,
    ) -> GpStatus;
    pub fn GdipFindFirstImageItem(
        image: *mut GpImage,
        item: *mut ImageItemData,
    ) -> GpStatus;
    pub fn GdipFindNextImageItem(
        image: *mut GpImage,
        item: *mut ImageItemData,
    ) -> GpStatus;
    pub fn GdipGetImageItemData(
        image: *mut GpImage,
        item: *mut ImageItemData,
    ) -> GpStatus;
    pub fn GdipImageForceValidation(
        image: *mut GpImage,
    ) -> GpStatus;
    pub fn GdipCreateBitmapFromStream(
        stream: *mut IStream,
        bitmap: *mut *mut GpBitmap,
    ) -> GpStatus;
    pub fn GdipCreateBitmapFromFile(
        filename: *const WCHAR,
        bitmap: *mut *mut GpBitmap,
    ) -> GpStatus;
    pub fn GdipCreateBitmapFromStreamICM(
        stream: *mut IStream,
        bitmap: *mut *mut GpBitmap,
    ) -> GpStatus;
    pub fn GdipCreateBitmapFromFileICM(
        filename: *const WCHAR,
        bitmap: *mut *mut GpBitmap,
    ) -> GpStatus;
    pub fn GdipCreateBitmapFromScan0(
        width: INT,
        height: INT,
        stride: INT,
        format: PixelFormat,
        scan0: *mut BYTE,
        bitmap: *mut *mut GpBitmap,
    ) -> GpStatus;
    pub fn GdipCreateBitmapFromGraphics(
        width: INT,
        height: INT,
        target: *mut GpGraphics,
        bitmap: *mut *mut GpBitmap,
    ) -> GpStatus;
    pub fn GdipCreateBitmapFromDirectDrawSurface(
        surface: *mut IDirectDrawSurface7,
        bitmap: *mut *mut GpBitmap,
    ) -> GpStatus;
    pub fn GdipCreateBitmapFromGdiDib(
        gdiBitmapInfo: *const BITMAPINFO,
        gdiBitmapData: *mut VOID,
        bitmap: *mut *mut GpBitmap,
    ) -> GpStatus;
    pub fn GdipCreateBitmapFromHBITMAP(
        hbm: HBITMAP,
        hpal: HPALETTE,
        bitmap: *mut *mut GpBitmap,
    ) -> GpStatus;
    pub fn GdipCreateHBITMAPFromBitmap(
        bitmap: *mut GpBitmap,
        hbmReturn: *mut HBITMAP,
        background: ARGB,
    ) -> GpStatus;
    pub fn GdipCreateBitmapFromHICON(
        hicon: HICON,
        bitmap: *mut *mut GpBitmap,
    ) -> GpStatus;
    pub fn GdipCreateHICONFromBitmap(
        bitmap: *mut GpBitmap,
        hbmReturn: *mut HICON,
    ) -> GpStatus;
    pub fn GdipCreateBitmapFromResource(
        hInstance: HINSTANCE,
        lpBitmapName: *const WCHAR,
        bitmap: *mut *mut GpBitmap,
    ) -> GpStatus;
    pub fn GdipCloneBitmapArea(
        x: REAL,
        y: REAL,
        width: REAL,
        height: REAL,
        format: PixelFormat,
        srcBitmap: *mut GpBitmap,
        dstBitmap: *mut *mut GpBitmap,
    ) -> GpStatus;
    pub fn GdipCloneBitmapAreaI(
        x: INT,
        y: INT,
        width: INT,
        height: INT,
        format: PixelFormat,
        srcBitmap: *mut GpBitmap,
        dstBitmap: *mut *mut GpBitmap,
    ) -> GpStatus;
    pub fn GdipBitmapLockBits(
        bitmap: *mut GpBitmap,
        rect: *const GpRect,
        flags: UINT,
        format: PixelFormat,
        lockedBitmapData: *mut BitmapData,
    ) -> GpStatus;
    pub fn GdipBitmapUnlockBits(
        bitmap: *mut GpBitmap,
        lockedBitmapData: *mut BitmapData,
    ) -> GpStatus;
    pub fn GdipBitmapGetPixel(
        bitmap: *mut GpBitmap,
        x: INT,
        y: INT,
        color: *mut ARGB,
    ) -> GpStatus;
    pub fn GdipBitmapSetPixel(
        bitmap: *mut GpBitmap,
        x: INT,
        y: INT,
        color: ARGB,
    ) -> GpStatus;
    // pub fn GdipImageSetAbort(
    //     pImage: *mut GpImage,
    //     pIAbort: *mut GdiplusAbort,
    // ) -> GpStatus;
    // pub fn GdipGraphicsSetAbort(
    //     pGraphics: *mut GpGraphics,
    //     pIAbort: *mut GdiplusAbort,
    // ) -> GpStatus;
    pub fn GdipBitmapConvertFormat(
        pInputBitmap: *mut GpBitmap,
        format: PixelFormat,
        dithertype: DitherType,
        palettetype: PaletteType,
        palette: *mut ColorPalette,
        alphaThresholdPercent: REAL,
    ) -> GpStatus;
    pub fn GdipInitializePalette(
        palette: *mut ColorPalette,
        palettetype: PaletteType,
        optimalColors: INT,
        useTransparentColor: BOOL,
        bitmap: *mut GpBitmap,
    ) -> GpStatus;
    pub fn GdipBitmapApplyEffect(
        bitmap: *mut GpBitmap,
        effect: *mut CGpEffect,
        roi: *mut RECT,
        useAuxData: BOOL,
        auxData: *mut *mut VOID,
        auxDataSize: *mut INT,
    ) -> GpStatus;
    pub fn GdipBitmapCreateApplyEffect(
        inputBitmaps: *mut *mut GpBitmap,
        numInputs: INT,
        effect: *mut CGpEffect,
        roi: *mut RECT,
        outputRect: *mut RECT,
        outputBitmap: *mut *mut GpBitmap,
        useAuxData: BOOL,
        auxData: *mut *mut VOID,
        auxDataSize: *mut INT,
    ) -> GpStatus;
    pub fn GdipBitmapGetHistogram(
        bitmap: *mut GpBitmap,
        format: HistogramFormat,
        NumberOfEntries: UINT,
        channel0: *mut UINT,
        channel1: *mut UINT,
        channel2: *mut UINT,
        channel3: *mut UINT,
    ) -> GpStatus;
    pub fn GdipBitmapGetHistogramSize(
        format: HistogramFormat,
        NumberOfEntries: *mut UINT,
    ) -> GpStatus;
    pub fn GdipBitmapSetResolution(
        bitmap: *mut GpBitmap,
        xdpi: REAL,
        ydpi: REAL,
    ) -> GpStatus;
    pub fn GdipCreateImageAttributes(
        imageattr: *mut *mut GpImageAttributes,
    ) -> GpStatus;
    pub fn GdipCloneImageAttributes(
        imageattr: *const GpImageAttributes,
        cloneImageattr: *mut *mut GpImageAttributes,
    ) -> GpStatus;
    pub fn GdipDisposeImageAttributes(
        imageattr: *mut GpImageAttributes,
    ) -> GpStatus;
    pub fn GdipSetImageAttributesToIdentity(
        imageattr: *mut GpImageAttributes,
        type_: ColorAdjustType,
    ) -> GpStatus;
    pub fn GdipResetImageAttributes(
        imageattr: *mut GpImageAttributes,
        type_: ColorAdjustType,
    ) -> GpStatus;
    pub fn GdipSetImageAttributesColorMatrix(
        imageattr: *mut GpImageAttributes,
        type_: ColorAdjustType,
        enableFlag: BOOL,
        colorMatrix: *const ColorMatrix,
        grayMatrix: *const ColorMatrix,
        flags: ColorMatrixFlags,
    ) -> GpStatus;
    pub fn GdipSetImageAttributesThreshold(
        imageattr: *mut GpImageAttributes,
        type_: ColorAdjustType,
        enableFlag: BOOL,
        threshold: REAL,
    ) -> GpStatus;
    pub fn GdipSetImageAttributesGamma(
        imageattr: *mut GpImageAttributes,
        type_: ColorAdjustType,
        enableFlag: BOOL,
        gamma: REAL,
    ) -> GpStatus;
    pub fn GdipSetImageAttributesNoOp(
        imageattr: *mut GpImageAttributes,
        type_: ColorAdjustType,
        enableFlag: BOOL,
    ) -> GpStatus;
    pub fn GdipSetImageAttributesColorKeys(
        imageattr: *mut GpImageAttributes,
        type_: ColorAdjustType,
        enableFlag: BOOL,
        colorLow: ARGB,
        colorHigh: ARGB,
    ) -> GpStatus;
    pub fn GdipSetImageAttributesOutputChannel(
        imageattr: *mut GpImageAttributes,
        type_: ColorAdjustType,
        enableFlag: BOOL,
        channelFlags: ColorChannelFlags,
    ) -> GpStatus;
    pub fn GdipSetImageAttributesOutputChannelColorProfile(
        imageattr: *mut GpImageAttributes,
        type_: ColorAdjustType,
        enableFlag: BOOL,
        colorProfileFilename: *const WCHAR,
    ) -> GpStatus;
    pub fn GdipSetImageAttributesRemapTable(
        imageattr: *mut GpImageAttributes,
        type_: ColorAdjustType,
        enableFlag: BOOL,
        mapSize: UINT,
        map: *const ColorMap,
    ) -> GpStatus;
    pub fn GdipSetImageAttributesWrapMode(
        imageAttr: *mut GpImageAttributes,
        wrap: WrapMode,
        argb: ARGB,
        clamp: BOOL,
    ) -> GpStatus;
    pub fn GdipSetImageAttributesICMMode(
        imageAttr: *mut GpImageAttributes,
        on: BOOL,
    ) -> GpStatus;
    pub fn GdipGetImageAttributesAdjustedPalette(
        imageAttr: *mut GpImageAttributes,
        colorPalette: *mut ColorPalette,
        colorAdjustType: ColorAdjustType,
    ) -> GpStatus;
    pub fn GdipFlush(
        graphics: *mut GpGraphics,
        intention: GpFlushIntention,
    ) -> GpStatus;
    pub fn GdipCreateFromHDC(
        hdc: HDC,
        graphics: *mut *mut GpGraphics,
    ) -> GpStatus;
    pub fn GdipCreateFromHDC2(
        hdc: HDC,
        hDevice: HANDLE,
        graphics: *mut *mut GpGraphics,
    ) -> GpStatus;
    pub fn GdipCreateFromHWND(
        hwnd: HWND,
        graphics: *mut *mut GpGraphics,
    ) -> GpStatus;
    pub fn GdipCreateFromHWNDICM(
        hwnd: HWND,
        graphics: *mut *mut GpGraphics,
    ) -> GpStatus;
    pub fn GdipDeleteGraphics(
        graphics: *mut GpGraphics,
    ) -> GpStatus;
    pub fn GdipGetDC(
        graphics: *mut GpGraphics,
        hdc: *mut HDC,
    ) -> GpStatus;
    pub fn GdipReleaseDC(
        graphics: *mut GpGraphics,
        hdc: HDC,
    ) -> GpStatus;
    pub fn GdipSetCompositingMode(
        graphics: *mut GpGraphics,
        compositingMode: CompositingMode,
    ) -> GpStatus;
    pub fn GdipGetCompositingMode(
        graphics: *mut GpGraphics,
        compositingMode: *mut CompositingMode,
    ) -> GpStatus;
    pub fn GdipSetRenderingOrigin(
        graphics: *mut GpGraphics,
        x: INT,
        y: INT,
    ) -> GpStatus;
    pub fn GdipGetRenderingOrigin(
        graphics: *mut GpGraphics,
        x: *mut INT,
        y: *mut INT,
    ) -> GpStatus;
    pub fn GdipSetCompositingQuality(
        graphics: *mut GpGraphics,
        compositingQuality: CompositingQuality,
    ) -> GpStatus;
    pub fn GdipGetCompositingQuality(
        graphics: *mut GpGraphics,
        compositingQuality: *mut CompositingQuality,
    ) -> GpStatus;
    pub fn GdipSetSmoothingMode(
        graphics: *mut GpGraphics,
        smoothingMode: SmoothingMode,
    ) -> GpStatus;
    pub fn GdipGetSmoothingMode(
        graphics: *mut GpGraphics,
        smoothingMode: *mut SmoothingMode,
    ) -> GpStatus;
    pub fn GdipSetPixelOffsetMode(
        graphics: *mut GpGraphics,
        pixelOffsetMode: PixelOffsetMode,
    ) -> GpStatus;
    pub fn GdipGetPixelOffsetMode(
        graphics: *mut GpGraphics,
        pixelOffsetMode: *mut PixelOffsetMode,
    ) -> GpStatus;
    pub fn GdipSetTextRenderingHint(
        graphics: *mut GpGraphics,
        mode: TextRenderingHint,
    ) -> GpStatus;
    pub fn GdipGetTextRenderingHint(
        graphics: *mut GpGraphics,
        mode: *mut TextRenderingHint,
    ) -> GpStatus;
    pub fn GdipSetTextContrast(
        graphics: *mut GpGraphics,
        contrast: UINT,
    ) -> GpStatus;
    pub fn GdipGetTextContrast(
        graphics: *mut GpGraphics,
        contrast: *mut UINT,
    ) -> GpStatus;
    pub fn GdipSetInterpolationMode(
        graphics: *mut GpGraphics,
        interpolationMode: InterpolationMode,
    ) -> GpStatus;
    pub fn GdipGetInterpolationMode(
        graphics: *mut GpGraphics,
        interpolationMode: *mut InterpolationMode,
    ) -> GpStatus;
    pub fn GdipSetWorldTransform(
        graphics: *mut GpGraphics,
        matrix: *mut GpMatrix,
    ) -> GpStatus;
    pub fn GdipResetWorldTransform(
        graphics: *mut GpGraphics,
    ) -> GpStatus;
    pub fn GdipMultiplyWorldTransform(
        graphics: *mut GpGraphics,
        matrix: *const GpMatrix,
        order: GpMatrixOrder,
    ) -> GpStatus;
    pub fn GdipTranslateWorldTransform(
        graphics: *mut GpGraphics,
        dx: REAL,
        dy: REAL,
        order: GpMatrixOrder,
    ) -> GpStatus;
    pub fn GdipScaleWorldTransform(
        graphics: *mut GpGraphics,
        sx: REAL,
        sy: REAL,
        order: GpMatrixOrder,
    ) -> GpStatus;
    pub fn GdipRotateWorldTransform(
        graphics: *mut GpGraphics,
        angle: REAL,
        order: GpMatrixOrder,
    ) -> GpStatus;
    pub fn GdipGetWorldTransform(
        graphics: *mut GpGraphics,
        matrix: *mut GpMatrix,
    ) -> GpStatus;
    pub fn GdipResetPageTransform(
        graphics: *mut GpGraphics,
    ) -> GpStatus;
    pub fn GdipGetPageUnit(
        graphics: *mut GpGraphics,
        unit: *mut GpUnit,
    ) -> GpStatus;
    pub fn GdipGetPageScale(
        graphics: *mut GpGraphics,
        scale: *mut REAL,
    ) -> GpStatus;
    pub fn GdipSetPageUnit(
        graphics: *mut GpGraphics,
        unit: GpUnit,
    ) -> GpStatus;
    pub fn GdipSetPageScale(
        graphics: *mut GpGraphics,
        scale: REAL,
    ) -> GpStatus;
    pub fn GdipGetDpiX(
        graphics: *mut GpGraphics,
        dpi: *mut REAL,
    ) -> GpStatus;
    pub fn GdipGetDpiY(
        graphics: *mut GpGraphics,
        dpi: *mut REAL,
    ) -> GpStatus;
    pub fn GdipTransformPoints(
        graphics: *mut GpGraphics,
        destSpace: GpCoordinateSpace,
        srcSpace: GpCoordinateSpace,
        points: *mut GpPointF,
        count: INT,
    ) -> GpStatus;
    pub fn GdipTransformPointsI(
        graphics: *mut GpGraphics,
        destSpace: GpCoordinateSpace,
        srcSpace: GpCoordinateSpace,
        points: *mut GpPoint,
        count: INT,
    ) -> GpStatus;
    pub fn GdipGetNearestColor(
        graphics: *mut GpGraphics,
        argb: *mut ARGB,
    ) -> GpStatus;
    pub fn GdipCreateHalftonePalette() -> HPALETTE;
    pub fn GdipDrawLine(
        graphics: *mut GpGraphics,
        pen: *mut GpPen,
        x1: REAL,
        y1: REAL,
        x2: REAL,
        y2: REAL,
    ) -> GpStatus;
    pub fn GdipDrawLineI(
        graphics: *mut GpGraphics,
        pen: *mut GpPen,
        x1: INT,
        y1: INT,
        x2: INT,
        y2: INT,
    ) -> GpStatus;
    pub fn GdipDrawLines(
        graphics: *mut GpGraphics,
        pen: *mut GpPen,
        points: *const GpPointF,
        count: INT,
    ) -> GpStatus;
    pub fn GdipDrawLinesI(
        graphics: *mut GpGraphics,
        pen: *mut GpPen,
        points: *const GpPoint,
        count: INT,
    ) -> GpStatus;
    pub fn GdipDrawArc(
        graphics: *mut GpGraphics,
        pen: *mut GpPen,
        x: REAL,
        y: REAL,
        width: REAL,
        height: REAL,
        startAngle: REAL,
        sweepAngle: REAL,
    ) -> GpStatus;
    pub fn GdipDrawArcI(
        graphics: *mut GpGraphics,
        pen: *mut GpPen,
        x: INT,
        y: INT,
        width: INT,
        height: INT,
        startAngle: REAL,
        sweepAngle: REAL,
    ) -> GpStatus;
    pub fn GdipDrawBezier(
        graphics: *mut GpGraphics,
        pen: *mut GpPen,
        x1: REAL,
        y1: REAL,
        x2: REAL,
        y2: REAL,
        x3: REAL,
        y3: REAL,
        x4: REAL,
        y4: REAL,
    ) -> GpStatus;
    pub fn GdipDrawBezierI(
        graphics: *mut GpGraphics,
        pen: *mut GpPen,
        x1: INT,
        y1: INT,
        x2: INT,
        y2: INT,
        x3: INT,
        y3: INT,
        x4: INT,
        y4: INT,
    ) -> GpStatus;
    pub fn GdipDrawBeziers(
        graphics: *mut GpGraphics,
        pen: *mut GpPen,
        points: *const GpPointF,
        count: INT,
    ) -> GpStatus;
    pub fn GdipDrawBeziersI(
        graphics: *mut GpGraphics,
        pen: *mut GpPen,
        points: *const GpPoint,
        count: INT,
    ) -> GpStatus;
    pub fn GdipDrawRectangle(
        graphics: *mut GpGraphics,
        pen: *mut GpPen,
        x: REAL,
        y: REAL,
        width: REAL,
        height: REAL,
    ) -> GpStatus;
    pub fn GdipDrawRectangleI(
        graphics: *mut GpGraphics,
        pen: *mut GpPen,
        x: INT,
        y: INT,
        width: INT,
        height: INT,
    ) -> GpStatus;
    pub fn GdipDrawRectangles(
        graphics: *mut GpGraphics,
        pen: *mut GpPen,
        rects: *const GpRectF,
        count: INT,
    ) -> GpStatus;
    pub fn GdipDrawRectanglesI(
        graphics: *mut GpGraphics,
        pen: *mut GpPen,
        rects: *const GpRect,
        count: INT,
    ) -> GpStatus;
    pub fn GdipDrawEllipse(
        graphics: *mut GpGraphics,
        pen: *mut GpPen,
        x: REAL,
        y: REAL,
        width: REAL,
        height: REAL,
    ) -> GpStatus;
    pub fn GdipDrawEllipseI(
        graphics: *mut GpGraphics,
        pen: *mut GpPen,
        x: INT,
        y: INT,
        width: INT,
        height: INT,
    ) -> GpStatus;
    pub fn GdipDrawPie(
        graphics: *mut GpGraphics,
        pen: *mut GpPen,
        x: REAL,
        y: REAL,
        width: REAL,
        height: REAL,
        startAngle: REAL,
        sweepAngle: REAL,
    ) -> GpStatus;
    pub fn GdipDrawPieI(
        graphics: *mut GpGraphics,
        pen: *mut GpPen,
        x: INT,
        y: INT,
        width: INT,
        height: INT,
        startAngle: REAL,
        sweepAngle: REAL,
    ) -> GpStatus;
    pub fn GdipDrawPolygon(
        graphics: *mut GpGraphics,
        pen: *mut GpPen,
        points: *const GpPointF,
        count: INT,
    ) -> GpStatus;
    pub fn GdipDrawPolygonI(
        graphics: *mut GpGraphics,
        pen: *mut GpPen,
        points: *const GpPoint,
        count: INT,
    ) -> GpStatus;
    pub fn GdipDrawPath(
        graphics: *mut GpGraphics,
        pen: *mut GpPen,
        path: *mut GpPath,
    ) -> GpStatus;
    pub fn GdipDrawCurve(
        graphics: *mut GpGraphics,
        pen: *mut GpPen,
        points: *const GpPointF,
        count: INT,
    ) -> GpStatus;
    pub fn GdipDrawCurveI(
        graphics: *mut GpGraphics,
        pen: *mut GpPen,
        points: *const GpPoint,
        count: INT,
    ) -> GpStatus;
    pub fn GdipDrawCurve2(
        graphics: *mut GpGraphics,
        pen: *mut GpPen,
        points: *const GpPointF,
        count: INT,
        tension: REAL,
    ) -> GpStatus;
    pub fn GdipDrawCurve2I(
        graphics: *mut GpGraphics,
        pen: *mut GpPen,
        points: *const GpPoint,
        count: INT,
        tension: REAL,
    ) -> GpStatus;
    pub fn GdipDrawCurve3(
        graphics: *mut GpGraphics,
        pen: *mut GpPen,
        points: *const GpPointF,
        count: INT,
        offset: INT,
        numberOfSegments: INT,
        tension: REAL,
    ) -> GpStatus;
    pub fn GdipDrawCurve3I(
        graphics: *mut GpGraphics,
        pen: *mut GpPen,
        points: *const GpPoint,
        count: INT,
        offset: INT,
        numberOfSegments: INT,
        tension: REAL,
    ) -> GpStatus;
    pub fn GdipDrawClosedCurve(
        graphics: *mut GpGraphics,
        pen: *mut GpPen,
        points: *const GpPointF,
        count: INT,
    ) -> GpStatus;
    pub fn GdipDrawClosedCurveI(
        graphics: *mut GpGraphics,
        pen: *mut GpPen,
        points: *const GpPoint,
        count: INT,
    ) -> GpStatus;
    pub fn GdipDrawClosedCurve2(
        graphics: *mut GpGraphics,
        pen: *mut GpPen,
        points: *const GpPointF,
        count: INT,
        tension: REAL,
    ) -> GpStatus;
    pub fn GdipDrawClosedCurve2I(
        graphics: *mut GpGraphics,
        pen: *mut GpPen,
        points: *const GpPoint,
        count: INT,
        tension: REAL,
    ) -> GpStatus;
    pub fn GdipGraphicsClear(
        graphics: *mut GpGraphics,
        color: ARGB,
    ) -> GpStatus;
    pub fn GdipFillRectangle(
        graphics: *mut GpGraphics,
        brush: *mut GpBrush,
        x: REAL,
        y: REAL,
        width: REAL,
        height: REAL,
    ) -> GpStatus;
    pub fn GdipFillRectangleI(
        graphics: *mut GpGraphics,
        brush: *mut GpBrush,
        x: INT,
        y: INT,
        width: INT,
        height: INT,
    ) -> GpStatus;
    pub fn GdipFillRectangles(
        graphics: *mut GpGraphics,
        brush: *mut GpBrush,
        rects: *const GpRectF,
        count: INT,
    ) -> GpStatus;
    pub fn GdipFillRectanglesI(
        graphics: *mut GpGraphics,
        brush: *mut GpBrush,
        rects: *const GpRect,
        count: INT,
    ) -> GpStatus;
    pub fn GdipFillPolygon(
        graphics: *mut GpGraphics,
        brush: *mut GpBrush,
        points: *const GpPointF,
        count: INT,
        fillMode: GpFillMode,
    ) -> GpStatus;
    pub fn GdipFillPolygonI(
        graphics: *mut GpGraphics,
        brush: *mut GpBrush,
        points: *const GpPoint,
        count: INT,
        fillMode: GpFillMode,
    ) -> GpStatus;
    pub fn GdipFillPolygon2(
        graphics: *mut GpGraphics,
        brush: *mut GpBrush,
        points: *const GpPointF,
        count: INT,
    ) -> GpStatus;
    pub fn GdipFillPolygon2I(
        graphics: *mut GpGraphics,
        brush: *mut GpBrush,
        points: *const GpPoint,
        count: INT,
    ) -> GpStatus;
    pub fn GdipFillEllipse(
        graphics: *mut GpGraphics,
        brush: *mut GpBrush,
        x: REAL,
        y: REAL,
        width: REAL,
        height: REAL,
    ) -> GpStatus;
    pub fn GdipFillEllipseI(
        graphics: *mut GpGraphics,
        brush: *mut GpBrush,
        x: INT,
        y: INT,
        width: INT,
        height: INT,
    ) -> GpStatus;
    pub fn GdipFillPie(
        graphics: *mut GpGraphics,
        brush: *mut GpBrush,
        x: REAL,
        y: REAL,
        width: REAL,
        height: REAL,
        startAngle: REAL,
        sweepAngle: REAL,
    ) -> GpStatus;
    pub fn GdipFillPieI(
        graphics: *mut GpGraphics,
        brush: *mut GpBrush,
        x: INT,
        y: INT,
        width: INT,
        height: INT,
        startAngle: REAL,
        sweepAngle: REAL,
    ) -> GpStatus;
    pub fn GdipFillPath(
        graphics: *mut GpGraphics,
        brush: *mut GpBrush,
        path: *mut GpPath,
    ) -> GpStatus;
    pub fn GdipFillClosedCurve(
        graphics: *mut GpGraphics,
        brush: *mut GpBrush,
        points: *const GpPointF,
        count: INT,
    ) -> GpStatus;
    pub fn GdipFillClosedCurveI(
        graphics: *mut GpGraphics,
        brush: *mut GpBrush,
        points: *const GpPoint,
        count: INT,
    ) -> GpStatus;
    pub fn GdipFillClosedCurve2(
        graphics: *mut GpGraphics,
        brush: *mut GpBrush,
        points: *const GpPointF,
        count: INT,
        tension: REAL,
        fillMode: GpFillMode,
    ) -> GpStatus;
    pub fn GdipFillClosedCurve2I(
        graphics: *mut GpGraphics,
        brush: *mut GpBrush,
        points: *const GpPoint,
        count: INT,
        tension: REAL,
        fillMode: GpFillMode,
    ) -> GpStatus;
    pub fn GdipFillRegion(
        graphics: *mut GpGraphics,
        brush: *mut GpBrush,
        region: *mut GpRegion,
    ) -> GpStatus;
    pub fn GdipDrawImageFX(
        graphics: *mut GpGraphics,
        image: *mut GpImage,
        source: *mut GpRectF,
        xForm: *mut GpMatrix,
        effect: *mut CGpEffect,
        imageAttributes: *mut GpImageAttributes,
        srcUnit: GpUnit,
    ) -> GpStatus;
    pub fn GdipDrawImage(
        graphics: *mut GpGraphics,
        image: *mut GpImage,
        x: REAL,
        y: REAL,
    ) -> GpStatus;
    pub fn GdipDrawImageI(
        graphics: *mut GpGraphics,
        image: *mut GpImage,
        x: INT,
        y: INT,
    ) -> GpStatus;
    pub fn GdipDrawImageRect(
        graphics: *mut GpGraphics,
        image: *mut GpImage,
        x: REAL,
        y: REAL,
        width: REAL,
        height: REAL,
    ) -> GpStatus;
    pub fn GdipDrawImageRectI(
        graphics: *mut GpGraphics,
        image: *mut GpImage,
        x: INT,
        y: INT,
        width: INT,
        height: INT,
    ) -> GpStatus;
    pub fn GdipDrawImagePoints(
        graphics: *mut GpGraphics,
        image: *mut GpImage,
        dstpoints: *const GpPointF,
        count: INT,
    ) -> GpStatus;
    pub fn GdipDrawImagePointsI(
        graphics: *mut GpGraphics,
        image: *mut GpImage,
        dstpoints: *const GpPoint,
        count: INT,
    ) -> GpStatus;
    pub fn GdipDrawImagePointRect(
        graphics: *mut GpGraphics,
        image: *mut GpImage,
        x: REAL,
        y: REAL,
        srcx: REAL,
        srcy: REAL,
        srcwidth: REAL,
        srcheight: REAL,
        srcUnit: GpUnit,
    ) -> GpStatus;
    pub fn GdipDrawImagePointRectI(
        graphics: *mut GpGraphics,
        image: *mut GpImage,
        x: INT,
        y: INT,
        srcx: INT,
        srcy: INT,
        srcwidth: INT,
        srcheight: INT,
        srcUnit: GpUnit,
    ) -> GpStatus;
    pub fn GdipDrawImageRectRect(
        graphics: *mut GpGraphics,
        image: *mut GpImage,
        dstx: REAL,
        dsty: REAL,
        dstwidth: REAL,
        dstheight: REAL,
        srcx: REAL,
        srcy: REAL,
        srcwidth: REAL,
        srcheight: REAL,
        srcUnit: GpUnit,
        imageAttributes: *const GpImageAttributes,
        callback: DrawImageAbort,
        callbackData: *mut VOID,
    ) -> GpStatus;
    pub fn GdipDrawImageRectRectI(
        graphics: *mut GpGraphics,
        image: *mut GpImage,
        dstx: INT,
        dsty: INT,
        dstwidth: INT,
        dstheight: INT,
        srcx: INT,
        srcy: INT,
        srcwidth: INT,
        srcheight: INT,
        srcUnit: GpUnit,
        imageAttributes: *const GpImageAttributes,
        callback: DrawImageAbort,
        callbackData: *mut VOID,
    ) -> GpStatus;
    pub fn GdipDrawImagePointsRect(
        graphics: *mut GpGraphics,
        image: *mut GpImage,
        points: *const GpPointF,
        count: INT,
        srcx: REAL,
        srcy: REAL,
        srcwidth: REAL,
        srcheight: REAL,
        srcUnit: GpUnit,
        imageAttributes: *const GpImageAttributes,
        callback: DrawImageAbort,
        callbackData: *mut VOID,
    ) -> GpStatus;
    pub fn GdipDrawImagePointsRectI(
        graphics: *mut GpGraphics,
        image: *mut GpImage,
        points: *const GpPoint,
        count: INT,
        srcx: INT,
        srcy: INT,
        srcwidth: INT,
        srcheight: INT,
        srcUnit: GpUnit,
        imageAttributes: *const GpImageAttributes,
        callback: DrawImageAbort,
        callbackData: *mut VOID,
    ) -> GpStatus;
    pub fn GdipEnumerateMetafileDestPoint(
        graphics: *mut GpGraphics,
        metafile: *const GpMetafile,
        destPoint: *const PointF,
        callback: EnumerateMetafileProc,
        callbackData: *mut VOID,
        imageAttributes: *const GpImageAttributes,
    ) -> GpStatus;
    pub fn GdipEnumerateMetafileDestPointI(
        graphics: *mut GpGraphics,
        metafile: *const GpMetafile,
        destPoint: *const Point,
        callback: EnumerateMetafileProc,
        callbackData: *mut VOID,
        imageAttributes: *const GpImageAttributes,
    ) -> GpStatus;
    pub fn GdipEnumerateMetafileDestRect(
        graphics: *mut GpGraphics,
        metafile: *const GpMetafile,
        destRect: *const RectF,
        callback: EnumerateMetafileProc,
        callbackData: *mut VOID,
        imageAttributes: *const GpImageAttributes,
    ) -> GpStatus;
    pub fn GdipEnumerateMetafileDestRectI(
        graphics: *mut GpGraphics,
        metafile: *const GpMetafile,
        destRect: *const Rect,
        callback: EnumerateMetafileProc,
        callbackData: *mut VOID,
        imageAttributes: *const GpImageAttributes,
    ) -> GpStatus;
    pub fn GdipEnumerateMetafileDestPoints(
        graphics: *mut GpGraphics,
        metafile: *const GpMetafile,
        destPoints: *const PointF,
        count: INT,
        callback: EnumerateMetafileProc,
        callbackData: *mut VOID,
        imageAttributes: *const GpImageAttributes,
    ) -> GpStatus;
    pub fn GdipEnumerateMetafileDestPointsI(
        graphics: *mut GpGraphics,
        metafile: *const GpMetafile,
        destPoints: *const Point,
        count: INT,
        callback: EnumerateMetafileProc,
        callbackData: *mut VOID,
        imageAttributes: *const GpImageAttributes,
    ) -> GpStatus;
    pub fn GdipEnumerateMetafileSrcRectDestPoint(
        graphics: *mut GpGraphics,
        metafile: *const GpMetafile,
        destPoint: *const PointF,
        srcRect: *const RectF,
        srcUnit: Unit,
        callback: EnumerateMetafileProc,
        callbackData: *mut VOID,
        imageAttributes: *const GpImageAttributes,
    ) -> GpStatus;
    pub fn GdipEnumerateMetafileSrcRectDestPointI(
        graphics: *mut GpGraphics,
        metafile: *const GpMetafile,
        destPoint: *const Point,
        srcRect: *const Rect,
        srcUnit: Unit,
        callback: EnumerateMetafileProc,
        callbackData: *mut VOID,
        imageAttributes: *const GpImageAttributes,
    ) -> GpStatus;
    pub fn GdipEnumerateMetafileSrcRectDestRect(
        graphics: *mut GpGraphics,
        metafile: *const GpMetafile,
        destRect: *const RectF,
        srcRect: *const RectF,
        srcUnit: Unit,
        callback: EnumerateMetafileProc,
        callbackData: *mut VOID,
        imageAttributes: *const GpImageAttributes,
    ) -> GpStatus;
    pub fn GdipEnumerateMetafileSrcRectDestRectI(
        graphics: *mut GpGraphics,
        metafile: *const GpMetafile,
        destRect: *const Rect,
        srcRect: *const Rect,
        srcUnit: Unit,
        callback: EnumerateMetafileProc,
        callbackData: *mut VOID,
        imageAttributes: *const GpImageAttributes,
    ) -> GpStatus;
    pub fn GdipEnumerateMetafileSrcRectDestPoints(
        graphics: *mut GpGraphics,
        metafile: *const GpMetafile,
        destPoints: *const PointF,
        count: INT,
        srcRect: *const RectF,
        srcUnit: Unit,
        callback: EnumerateMetafileProc,
        callbackData: *mut VOID,
        imageAttributes: *const GpImageAttributes,
    ) -> GpStatus;
    pub fn GdipEnumerateMetafileSrcRectDestPointsI(
        graphics: *mut GpGraphics,
        metafile: *const GpMetafile,
        destPoints: *const Point,
        count: INT,
        srcRect: *const Rect,
        srcUnit: Unit,
        callback: EnumerateMetafileProc,
        callbackData: *mut VOID,
        imageAttributes: *const GpImageAttributes,
    ) -> GpStatus;
    pub fn GdipPlayMetafileRecord(
        metafile: *const GpMetafile,
        recordType: EmfPlusRecordType,
        flags: UINT,
        dataSize: UINT,
        data: *const BYTE,
    ) -> GpStatus;
    pub fn GdipSetClipGraphics(
        graphics: *mut GpGraphics,
        srcgraphics: *mut GpGraphics,
        combineMode: CombineMode,
    ) -> GpStatus;
    pub fn GdipSetClipRect(
        graphics: *mut GpGraphics,
        x: REAL,
        y: REAL,
        width: REAL,
        height: REAL,
        combineMode: CombineMode,
    ) -> GpStatus;
    pub fn GdipSetClipRectI(
        graphics: *mut GpGraphics,
        x: INT,
        y: INT,
        width: INT,
        height: INT,
        combineMode: CombineMode,
    ) -> GpStatus;
    pub fn GdipSetClipPath(
        graphics: *mut GpGraphics,
        path: *mut GpPath,
        combineMode: CombineMode,
    ) -> GpStatus;
    pub fn GdipSetClipRegion(
        graphics: *mut GpGraphics,
        region: *mut GpRegion,
        combineMode: CombineMode,
    ) -> GpStatus;
    pub fn GdipSetClipHrgn(
        graphics: *mut GpGraphics,
        hRgn: HRGN,
        combineMode: CombineMode,
    ) -> GpStatus;
    pub fn GdipResetClip(
        graphics: *mut GpGraphics,
    ) -> GpStatus;
    pub fn GdipTranslateClip(
        graphics: *mut GpGraphics,
        dx: REAL,
        dy: REAL,
    ) -> GpStatus;
    pub fn GdipTranslateClipI(
        graphics: *mut GpGraphics,
        dx: INT,
        dy: INT,
    ) -> GpStatus;
    pub fn GdipGetClip(
        graphics: *mut GpGraphics,
        region: *mut GpRegion,
    ) -> GpStatus;
    pub fn GdipGetClipBounds(
        graphics: *mut GpGraphics,
        rect: *mut GpRectF,
    ) -> GpStatus;
    pub fn GdipGetClipBoundsI(
        graphics: *mut GpGraphics,
        rect: *mut GpRect,
    ) -> GpStatus;
    pub fn GdipIsClipEmpty(
        graphics: *mut GpGraphics,
        result: *mut BOOL,
    ) -> GpStatus;
    pub fn GdipGetVisibleClipBounds(
        graphics: *mut GpGraphics,
        rect: *mut GpRectF,
    ) -> GpStatus;
    pub fn GdipGetVisibleClipBoundsI(
        graphics: *mut GpGraphics,
        rect: *mut GpRect,
    ) -> GpStatus;
    pub fn GdipIsVisibleClipEmpty(
        graphics: *mut GpGraphics,
        result: *mut BOOL,
    ) -> GpStatus;
    pub fn GdipIsVisiblePoint(
        graphics: *mut GpGraphics,
        x: REAL,
        y: REAL,
        result: *mut BOOL,
    ) -> GpStatus;
    pub fn GdipIsVisiblePointI(
        graphics: *mut GpGraphics,
        x: INT,
        y: INT,
        result: *mut BOOL,
    ) -> GpStatus;
    pub fn GdipIsVisibleRect(
        graphics: *mut GpGraphics,
        x: REAL,
        y: REAL,
        width: REAL,
        height: REAL,
        result: *mut BOOL,
    ) -> GpStatus;
    pub fn GdipIsVisibleRectI(
        graphics: *mut GpGraphics,
        x: INT,
        y: INT,
        width: INT,
        height: INT,
        result: *mut BOOL,
    ) -> GpStatus;
    pub fn GdipSaveGraphics(
        graphics: *mut GpGraphics,
        state: *mut GraphicsState,
    ) -> GpStatus;
    pub fn GdipRestoreGraphics(
        graphics: *mut GpGraphics,
        state: GraphicsState,
    ) -> GpStatus;
    pub fn GdipBeginContainer(
        graphics: *mut GpGraphics,
        dstrect: *const GpRectF,
        srcrect: *const GpRectF,
        unit: GpUnit,
        state: *mut GraphicsContainer,
    ) -> GpStatus;
    pub fn GdipBeginContainerI(
        graphics: *mut GpGraphics,
        dstrect: *const GpRect,
        srcrect: *const GpRect,
        unit: GpUnit,
        state: *mut GraphicsContainer,
    ) -> GpStatus;
    pub fn GdipBeginContainer2(
        graphics: *mut GpGraphics,
        state: *mut GraphicsContainer,
    ) -> GpStatus;
    pub fn GdipEndContainer(
        graphics: *mut GpGraphics,
        state: GraphicsContainer,
    ) -> GpStatus;
    pub fn GdipGetMetafileHeaderFromWmf(
        hWmf: HMETAFILE,
        wmfPlaceableFileHeader: *const WmfPlaceableFileHeader,
        header: *mut MetafileHeader,
    ) -> GpStatus;
    pub fn GdipGetMetafileHeaderFromEmf(
        hEmf: HENHMETAFILE,
        header: *mut MetafileHeader,
    ) -> GpStatus;
    pub fn GdipGetMetafileHeaderFromFile(
        filename: *const WCHAR,
        header: *mut MetafileHeader,
    ) -> GpStatus;
    pub fn GdipGetMetafileHeaderFromStream(
        stream: *mut IStream,
        header: *mut MetafileHeader,
    ) -> GpStatus;
    pub fn GdipGetMetafileHeaderFromMetafile(
        metafile: *mut GpMetafile,
        header: *mut MetafileHeader,
    ) -> GpStatus;
    pub fn GdipGetHemfFromMetafile(
        metafile: *mut GpMetafile,
        hEmf: *mut HENHMETAFILE,
    ) -> GpStatus;
    pub fn GdipCreateStreamOnFile(
        filename: *const WCHAR,
        access: UINT,
        stream: *mut *mut IStream,
    ) -> GpStatus;
    pub fn GdipCreateMetafileFromWmf(
        hWmf: HMETAFILE,
        deleteWmf: BOOL,
        wmfPlaceableFileHeader: *const WmfPlaceableFileHeader,
        metafile: *mut *mut GpMetafile,
    ) -> GpStatus;
    pub fn GdipCreateMetafileFromEmf(
        hEmf: HENHMETAFILE,
        deleteEmf: BOOL,
        metafile: *mut *mut GpMetafile,
    ) -> GpStatus;
    pub fn GdipCreateMetafileFromFile(
        file: *const WCHAR,
        metafile: *mut *mut GpMetafile,
    ) -> GpStatus;
    pub fn GdipCreateMetafileFromWmfFile(
        file: *const WCHAR,
        wmfPlaceableFileHeader: *const WmfPlaceableFileHeader,
        metafile: *mut *mut GpMetafile,
    ) -> GpStatus;
    pub fn GdipCreateMetafileFromStream(
        stream: *mut IStream,
        metafile: *mut *mut GpMetafile,
    ) -> GpStatus;
    pub fn GdipRecordMetafile(
        referenceHdc: HDC,
        type_: EmfType,
        frameRect: *const GpRectF,
        frameUnit: MetafileFrameUnit,
        description: *const WCHAR,
        metafile: *mut *mut GpMetafile,
    ) -> GpStatus;
    pub fn GdipRecordMetafileI(
        referenceHdc: HDC,
        type_: EmfType,
        frameRect: *const GpRect,
        frameUnit: MetafileFrameUnit,
        description: *const WCHAR,
        metafile: *mut *mut GpMetafile,
    ) -> GpStatus;
    pub fn GdipRecordMetafileFileName(
        fileName: *const WCHAR,
        referenceHdc: HDC,
        type_: EmfType,
        frameRect: *const GpRectF,
        frameUnit: MetafileFrameUnit,
        description: *const WCHAR,
        metafile: *mut *mut GpMetafile,
    ) -> GpStatus;
    pub fn GdipRecordMetafileFileNameI(
        fileName: *const WCHAR,
        referenceHdc: HDC,
        type_: EmfType,
        frameRect: *const GpRect,
        frameUnit: MetafileFrameUnit,
        description: *const WCHAR,
        metafile: *mut *mut GpMetafile,
    ) -> GpStatus;
    pub fn GdipRecordMetafileStream(
        stream: *mut IStream,
        referenceHdc: HDC,
        type_: EmfType,
        frameRect: *const GpRectF,
        frameUnit: MetafileFrameUnit,
        description: *const WCHAR,
        metafile: *mut *mut GpMetafile,
    ) -> GpStatus;
    pub fn GdipRecordMetafileStreamI(
        stream: *mut IStream,
        referenceHdc: HDC,
        type_: EmfType,
        frameRect: *const GpRect,
        frameUnit: MetafileFrameUnit,
        description: *const WCHAR,
        metafile: *mut *mut GpMetafile,
    ) -> GpStatus;
    pub fn GdipSetMetafileDownLevelRasterizationLimit(
        metafile: *mut GpMetafile,
        metafileRasterizationLimitDpi: UINT,
    ) -> GpStatus;
    pub fn GdipGetMetafileDownLevelRasterizationLimit(
        metafile: *const GpMetafile,
        metafileRasterizationLimitDpi: *mut UINT,
    ) -> GpStatus;
    pub fn GdipGetImageDecodersSize(
        numDecoders: *mut UINT,
        size: *mut UINT,
    ) -> GpStatus;
    pub fn GdipGetImageDecoders(
        numDecoders: UINT,
        size: UINT,
        decoders: *mut ImageCodecInfo,
    ) -> GpStatus;
    pub fn GdipGetImageEncodersSize(
        numEncoders: *mut UINT,
        size: *mut UINT,
    ) -> GpStatus;
    pub fn GdipGetImageEncoders(
        numEncoders: UINT,
        size: UINT,
        encoders: *mut ImageCodecInfo,
    ) -> GpStatus;
    pub fn GdipComment(
        graphics: *mut GpGraphics,
        sizeData: UINT,
        data: *const BYTE,
    ) -> GpStatus;
    pub fn GdipCreateFontFamilyFromName(
        name: *const WCHAR,
        fontCollection: *mut GpFontCollection,
        fontFamily: *mut *mut GpFontFamily,
    ) -> GpStatus;
    pub fn GdipDeleteFontFamily(
        fontFamily: *mut GpFontFamily,
    ) -> GpStatus;
    pub fn GdipCloneFontFamily(
        fontFamily: *mut GpFontFamily,
        clonedFontFamily: *mut *mut GpFontFamily,
    ) -> GpStatus;
    pub fn GdipGetGenericFontFamilySansSerif(
        nativeFamily: *mut *mut GpFontFamily,
    ) -> GpStatus;
    pub fn GdipGetGenericFontFamilySerif(
        nativeFamily: *mut *mut GpFontFamily,
    ) -> GpStatus;
    pub fn GdipGetGenericFontFamilyMonospace(
        nativeFamily: *mut *mut GpFontFamily,
    ) -> GpStatus;
    pub fn GdipGetFamilyName(
        family: *const GpFontFamily,
        name: LPWSTR,
        language: LANGID,
    ) -> GpStatus;
    pub fn GdipIsStyleAvailable(
        family: *const GpFontFamily,
        style: INT,
        IsStyleAvailable: *mut BOOL,
    ) -> GpStatus;
    pub fn GdipFontCollectionEnumerable(
        fontCollection: *mut GpFontCollection,
        graphics: *mut GpGraphics,
        numFound: *mut INT,
    ) -> GpStatus;
    pub fn GdipFontCollectionEnumerate(
        fontCollection: *mut GpFontCollection,
        numSought: INT,
        gpfamilies: *mut *mut GpFontFamily,
        numFound: *mut INT,
        graphics: *mut GpGraphics,
    ) -> GpStatus;
    pub fn GdipGetEmHeight(
        family: *const GpFontFamily,
        style: INT,
        EmHeight: *mut UINT16,
    ) -> GpStatus;
    pub fn GdipGetCellAscent(
        family: *const GpFontFamily,
        style: INT,
        CellAscent: *mut UINT16,
    ) -> GpStatus;
    pub fn GdipGetCellDescent(
        family: *const GpFontFamily,
        style: INT,
        CellDescent: *mut UINT16,
    ) -> GpStatus;
    pub fn GdipGetLineSpacing(
        family: *const GpFontFamily,
        style: INT,
        LineSpacing: *mut UINT16,
    ) -> GpStatus;
    pub fn GdipCreateFontFromDC(
        hdc: HDC,
        font: *mut *mut GpFont,
    ) -> GpStatus;
    pub fn GdipCreateFontFromLogfontA(
        hdc: HDC,
        logfont: *const LOGFONTA,
        font: *mut *mut GpFont,
    ) -> GpStatus;
    pub fn GdipCreateFontFromLogfontW(
        hdc: HDC,
        logfont: *const LOGFONTW,
        font: *mut *mut GpFont,
    ) -> GpStatus;
    pub fn GdipCreateFont(
        fontFamily: *const GpFontFamily,
        emSize: REAL,
        style: INT,
        unit: Unit,
        font: *mut *mut GpFont,
    ) -> GpStatus;
    pub fn GdipCloneFont(
        font: *mut GpFont,
        cloneFont: *mut *mut GpFont,
    ) -> GpStatus;
    pub fn GdipDeleteFont(
        font: *mut GpFont,
    ) -> GpStatus;
    pub fn GdipGetFamily(
        font: *mut GpFont,
        family: *mut *mut GpFontFamily,
    ) -> GpStatus;
    pub fn GdipGetFontStyle(
        font: *mut GpFont,
        style: *mut INT,
    ) -> GpStatus;
    pub fn GdipGetFontSize(
        font: *mut GpFont,
        size: *mut REAL,
    ) -> GpStatus;
    pub fn GdipGetFontUnit(
        font: *mut GpFont,
        unit: *mut Unit,
    ) -> GpStatus;
    pub fn GdipGetFontHeight(
        font: *const GpFont,
        graphics: *const GpGraphics,
        height: *mut REAL,
    ) -> GpStatus;
    pub fn GdipGetFontHeightGivenDPI(
        font: *const GpFont,
        dpi: REAL,
        height: *mut REAL,
    ) -> GpStatus;
    pub fn GdipGetLogFontA(
        font: *mut GpFont,
        graphics: *mut GpGraphics,
        logfontA: *mut LOGFONTA,
    ) -> GpStatus;
    pub fn GdipGetLogFontW(
        font: *mut GpFont,
        graphics: *mut GpGraphics,
        logfontW: *mut LOGFONTW,
    ) -> GpStatus;
    pub fn GdipNewInstalledFontCollection(
        fontCollection: *mut *mut GpFontCollection,
    ) -> GpStatus;
    pub fn GdipNewPrivateFontCollection(
        fontCollection: *mut *mut GpFontCollection,
    ) -> GpStatus;
    pub fn GdipDeletePrivateFontCollection(
        fontCollection: *mut *mut GpFontCollection,
    ) -> GpStatus;
    pub fn GdipGetFontCollectionFamilyCount(
        fontCollection: *mut GpFontCollection,
        numFound: *mut INT,
    ) -> GpStatus;
    pub fn GdipGetFontCollectionFamilyList(
        fontCollection: *mut GpFontCollection,
        numSought: INT,
        gpfamilies: *mut *mut GpFontFamily,
        numFound: *mut INT,
    ) -> GpStatus;
    pub fn GdipPrivateAddFontFile(
        fontCollection: *mut GpFontCollection,
        filename: *const WCHAR,
    ) -> GpStatus;
    pub fn GdipPrivateAddMemoryFont(
        fontCollection: *mut GpFontCollection,
        memory: *const c_void,
        length: INT,
    ) -> GpStatus;
    pub fn GdipDrawString(
        graphics: *mut GpGraphics,
        string: *const WCHAR,
        length: INT,
        font: *const GpFont,
        layoutRect: *const RectF,
        stringFormat: *const GpStringFormat,
        brush: *const GpBrush,
    ) -> GpStatus;
    pub fn GdipMeasureString(
        graphics: *mut GpGraphics,
        string: *const WCHAR,
        length: INT,
        font: *const GpFont,
        layoutRect: *const RectF,
        stringFormat: *const GpStringFormat,
        boundingBox: *mut RectF,
        codepointsFitted: *mut INT,
        linesFilled: *mut INT,
    ) -> GpStatus;
    pub fn GdipMeasureCharacterRanges(
        graphics: *mut GpGraphics,
        string: *const WCHAR,
        length: INT,
        font: *const GpFont,
        layoutRect: *const RectF,
        stringFormat: *const GpStringFormat,
        regionCount: INT,
        regions: *mut *mut GpRegion,
    ) -> GpStatus;
    pub fn GdipDrawDriverString(
        graphics: *mut GpGraphics,
        text: *const UINT16,
        length: INT,
        font: *const GpFont,
        brush: *const GpBrush,
        positions: *const PointF,
        flags: INT,
        matrix: *const GpMatrix,
    ) -> GpStatus;
    pub fn GdipMeasureDriverString(
        graphics: *mut GpGraphics,
        text: *const UINT16,
        length: INT,
        font: *const GpFont,
        positions: *const PointF,
        flags: INT,
        matrix: *const GpMatrix,
        boundingBox: *mut RectF,
    ) -> GpStatus;
    pub fn GdipCreateStringFormat(
        formatAttributes: INT,
        language: LANGID,
        format: *mut *mut GpStringFormat,
    ) -> GpStatus;
    pub fn GdipStringFormatGetGenericDefault(
        format: *mut *mut GpStringFormat,
    ) -> GpStatus;
    pub fn GdipStringFormatGetGenericTypographic(
        format: *mut *mut GpStringFormat,
    ) -> GpStatus;
    pub fn GdipDeleteStringFormat(
        format: *mut GpStringFormat,
    ) -> GpStatus;
    pub fn GdipCloneStringFormat(
        format: *const GpStringFormat,
        newFormat: *mut *mut GpStringFormat,
    ) -> GpStatus;
    pub fn GdipSetStringFormatFlags(
        format: *mut GpStringFormat,
        flags: INT,
    ) -> GpStatus;
    pub fn GdipGetStringFormatFlags(
        format: *const GpStringFormat,
        flags: *mut INT,
    ) -> GpStatus;
    pub fn GdipSetStringFormatAlign(
        format: *mut GpStringFormat,
        align: StringAlignment,
    ) -> GpStatus;
    pub fn GdipGetStringFormatAlign(
        format: *const GpStringFormat,
        align: *mut StringAlignment,
    ) -> GpStatus;
    pub fn GdipSetStringFormatLineAlign(
        format: *mut GpStringFormat,
        align: StringAlignment,
    ) -> GpStatus;
    pub fn GdipGetStringFormatLineAlign(
        format: *const GpStringFormat,
        align: *mut StringAlignment,
    ) -> GpStatus;
    pub fn GdipSetStringFormatTrimming(
        format: *mut GpStringFormat,
        trimming: StringTrimming,
    ) -> GpStatus;
    pub fn GdipGetStringFormatTrimming(
        format: *const GpStringFormat,
        trimming: *mut StringTrimming,
    ) -> GpStatus;
    pub fn GdipSetStringFormatHotkeyPrefix(
        format: *mut GpStringFormat,
        hotkeyPrefix: INT,
    ) -> GpStatus;
    pub fn GdipGetStringFormatHotkeyPrefix(
        format: *const GpStringFormat,
        hotkeyPrefix: *mut INT,
    ) -> GpStatus;
    pub fn GdipSetStringFormatTabStops(
        format: *mut GpStringFormat,
        firstTabOffset: REAL,
        count: INT,
        tabStops: *const REAL,
    ) -> GpStatus;
    pub fn GdipGetStringFormatTabStops(
        format: *const GpStringFormat,
        count: INT,
        firstTabOffset: *mut REAL,
        tabStops: *mut REAL,
    ) -> GpStatus;
    pub fn GdipGetStringFormatTabStopCount(
        format: *const GpStringFormat,
        count: *mut INT,
    ) -> GpStatus;
    pub fn GdipSetStringFormatDigitSubstitution(
        format: *mut GpStringFormat,
        language: LANGID,
        substitute: StringDigitSubstitute,
    ) -> GpStatus;
    pub fn GdipGetStringFormatDigitSubstitution(
        format: *const GpStringFormat,
        language: *mut LANGID,
        substitute: *mut StringDigitSubstitute,
    ) -> GpStatus;
    pub fn GdipGetStringFormatMeasurableCharacterRangeCount(
        format: *const GpStringFormat,
        count: *mut INT,
    ) -> GpStatus;
    pub fn GdipSetStringFormatMeasurableCharacterRanges(
        format: *mut GpStringFormat,
        rangeCount: INT,
        ranges: *const CharacterRange,
    ) -> GpStatus;
    pub fn GdipCreateCachedBitmap(
        bitmap: *mut GpBitmap,
        graphics: *mut GpGraphics,
        cachedBitmap: *mut *mut GpCachedBitmap,
    ) -> GpStatus;
    pub fn GdipDeleteCachedBitmap(
        cachedBitmap: *mut GpCachedBitmap,
    ) -> GpStatus;
    pub fn GdipDrawCachedBitmap(
        graphics: *mut GpGraphics,
        cachedBitmap: *mut GpCachedBitmap,
        x: INT,
        y: INT,
    ) -> GpStatus;
    pub fn GdipEmfToWmfBits(
        hemf: HENHMETAFILE,
        cbData16: UINT,
        pData16: LPBYTE,
        iMapMode: INT,
        eFlags: INT,
    ) -> UINT;
    pub fn GdipSetImageAttributesCachedBackground(
        imageattr: *mut GpImageAttributes,
        enableFlag: BOOL,
    ) -> GpStatus;
    pub fn GdipTestControl(
        control: GpTestControlEnum,
        param: *mut c_void,
    ) -> GpStatus;
    pub fn GdiplusNotificationHook(
        token: *mut ULONG_PTR,
    ) -> GpStatus;
    pub fn GdiplusNotificationUnhook(
        token: ULONG_PTR,
    );
    pub fn GdipConvertToEmfPlus(
        refGraphics: *const GpGraphics,
        metafile: *mut GpMetafile,
        conversionFailureFlag: *mut INT,
        emfType: EmfType,
        description: *const WCHAR,
        out_metafile: *mut *mut GpMetafile,
    ) -> GpStatus;
    pub fn GdipConvertToEmfPlusToFile(
        refGraphics: *const GpGraphics,
        metafile: *mut GpMetafile,
        conversionFailureFlag: *mut INT,
        filename: *const WCHAR,
        emfType: EmfType,
        description: *const WCHAR,
        out_metafile: *mut *mut GpMetafile,
    ) -> GpStatus;
    pub fn GdipConvertToEmfPlusToStream(
        refGraphics: *const GpGraphics,
        metafile: *mut GpMetafile,
        conversionFailureFlag: *mut INT,
        stream: *mut IStream,
        emfType: EmfType,
        description: *const WCHAR,
        out_metafile: *mut *mut GpMetafile,
    ) -> GpStatus;
}

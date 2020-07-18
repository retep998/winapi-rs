// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Mappings for the contents of UIAutomationCore.h
use ctypes::{c_double, c_int, c_long, c_void};
use shared::guiddef::GUID;
use shared::minwindef::{BOOL, DWORD, UINT};
use shared::windef::HWND;
use shared::winerror::HRESULT;
use shared::wtypes::BSTR;
use um::oaidl::{SAFEARRAY, VARIANT};
use um::oleacc::IAccessible;
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::LPCWSTR;
ENUM!{enum NavigateDirection {
    NavigateDirection_Parent = 0,
    NavigateDirection_NextSibling = 1,
    NavigateDirection_PreviousSibling = 2,
    NavigateDirection_FirstChild = 3,
    NavigateDirection_LastChild = 4,
}}
ENUM!{enum ProviderOptions {
    ProviderOptions_ClientSideProvider = 0x1,
    ProviderOptions_ServerSideProvider = 0x2,
    ProviderOptions_NonClientAreaProvider = 0x4,
    ProviderOptions_OverrideProvider = 0x8,
    ProviderOptions_ProviderOwnsSetFocus = 0x10,
    ProviderOptions_UseComThreading = 0x20,
    ProviderOptions_RefuseNonClientSupport = 0x40,
    ProviderOptions_HasNativeIAccessible = 0x80,
    ProviderOptions_UseClientCoordinates = 0x100,
}}
//DEFINE_ENUM_FLAG_OPERATORS(ProviderOptions);
ENUM!{enum StructureChangeType {
    StructureChangeType_ChildAdded = 0,
    StructureChangeType_ChildRemoved = StructureChangeType_ChildAdded + 1,
    StructureChangeType_ChildrenInvalidated = StructureChangeType_ChildRemoved + 1,
    StructureChangeType_ChildrenBulkAdded = StructureChangeType_ChildrenInvalidated + 1,
    StructureChangeType_ChildrenBulkRemoved = StructureChangeType_ChildrenBulkAdded + 1,
    StructureChangeType_ChildrenReordered = StructureChangeType_ChildrenBulkRemoved + 1,
}}
ENUM!{enum TextEditChangeType {
    TextEditChangeType_None = 0,
    TextEditChangeType_AutoCorrect = 1,
    TextEditChangeType_Composition = 2,
    TextEditChangeType_CompositionFinalized = 3,
    TextEditChangeType_AutoComplete = 4,
}}
ENUM!{enum OrientationType {
    OrientationType_None = 0,
    OrientationType_Horizontal = 1,
    OrientationType_Vertical = 2,
}}
ENUM!{enum DockPosition {
    DockPosition_Top = 0,
    DockPosition_Left = 1,
    DockPosition_Bottom = 2,
    DockPosition_Right = 3,
    DockPosition_Fill = 4,
    DockPosition_None = 5,
}}
ENUM!{enum ExpandCollapseState {
    ExpandCollapseState_Collapsed = 0,
    ExpandCollapseState_Expanded = 1,
    ExpandCollapseState_PartiallyExpanded = 2,
    ExpandCollapseState_LeafNode = 3,
}}
ENUM!{enum ScrollAmount {
    ScrollAmount_LargeDecrement = 0,
    ScrollAmount_SmallDecrement = 1,
    ScrollAmount_NoAmount = 2,
    ScrollAmount_LargeIncrement = 3,
    ScrollAmount_SmallIncrement = 4,
}}
ENUM!{enum RowOrColumnMajor {
    RowOrColumnMajor_RowMajor = 0,
    RowOrColumnMajor_ColumnMajor = 1,
    RowOrColumnMajor_Indeterminate = 2,
}}
ENUM!{enum ToggleState {
    ToggleState_Off = 0,
    ToggleState_On = 1,
    ToggleState_Indeterminate = 2,
}}
ENUM!{enum WindowVisualState {
    WindowVisualState_Normal = 0,
    WindowVisualState_Maximized = 1,
    WindowVisualState_Minimized = 2,
}}
ENUM!{enum SynchronizedInputType {
    SynchronizedInputType_KeyUp = 0x1,
    SynchronizedInputType_KeyDown = 0x2,
    SynchronizedInputType_LeftMouseUp = 0x4,
    SynchronizedInputType_LeftMouseDown = 0x8,
    SynchronizedInputType_RightMouseUp = 0x10,
    SynchronizedInputType_RightMouseDown = 0x20,
}}
//DEFINE_ENUM_FLAG_OPERATORS(SynchronizedInputType);
ENUM!{enum WindowInteractionState {
    WindowInteractionState_Running = 0,
    WindowInteractionState_Closing = 1,
    WindowInteractionState_ReadyForUserInteraction = 2,
    WindowInteractionState_BlockedByModalWindow = 3,
    WindowInteractionState_NotResponding = 4,
}}
ENUM!{enum SayAsInterpretAs {
    SayAsInterpretAs_None = 0,
    SayAsInterpretAs_Spell = 1,
    SayAsInterpretAs_Cardinal = 2,
    SayAsInterpretAs_Ordinal = 3,
    SayAsInterpretAs_Number = 4,
    SayAsInterpretAs_Date = 5,
    SayAsInterpretAs_Time = 6,
    SayAsInterpretAs_Telephone = 7,
    SayAsInterpretAs_Currency = 8,
    SayAsInterpretAs_Net = 9,
    SayAsInterpretAs_Url = 10,
    SayAsInterpretAs_Address = 11,
    SayAsInterpretAs_Alphanumeric = 12,
    SayAsInterpretAs_Name = 13,
    SayAsInterpretAs_Media = 14,
    SayAsInterpretAs_Date_MonthDayYear = 15,
    SayAsInterpretAs_Date_DayMonthYear = 16,
    SayAsInterpretAs_Date_YearMonthDay = 17,
    SayAsInterpretAs_Date_YearMonth = 18,
    SayAsInterpretAs_Date_MonthYear = 19,
    SayAsInterpretAs_Date_DayMonth = 20,
    SayAsInterpretAs_Date_MonthDay = 21,
    SayAsInterpretAs_Date_Year = 22,
    SayAsInterpretAs_Time_HoursMinutesSeconds12 = 23,
    SayAsInterpretAs_Time_HoursMinutes12 = 24,
    SayAsInterpretAs_Time_HoursMinutesSeconds24 = 25,
    SayAsInterpretAs_Time_HoursMinutes24 = 26,
}}
ENUM!{enum TextUnit {
    TextUnit_Character = 0,
    TextUnit_Format = 1,
    TextUnit_Word = 2,
    TextUnit_Line = 3,
    TextUnit_Paragraph = 4,
    TextUnit_Page = 5,
    TextUnit_Document = 6,
}}
ENUM!{enum TextPatternRangeEndpoint {
    TextPatternRangeEndpoint_Start = 0,
    TextPatternRangeEndpoint_End = 1,
}}
ENUM!{enum SupportedTextSelection {
    SupportedTextSelection_None = 0,
    SupportedTextSelection_Single = 1,
    SupportedTextSelection_Multiple = 2,
}}
ENUM!{enum LiveSetting {
    Off = 0,
    Polite = 1,
    Assertive = 2,
}}
ENUM!{enum ActiveEnd {
    ActiveEnd_None = 0,
    ActiveEnd_Start = 1,
    ActiveEnd_End = 2,
}}
ENUM!{enum CaretPosition {
    CaretPosition_Unknown = 0,
    CaretPosition_EndOfLine = 1,
    CaretPosition_BeginningOfLine = 2,
}}
ENUM!{enum CaretBidiMode {
    CaretBidiMode_LTR = 0,
    CaretBidiMode_RTL = 1,
}}
ENUM!{enum ZoomUnit {
    ZoomUnit_NoAmount = 0,
    ZoomUnit_LargeDecrement = 1,
    ZoomUnit_SmallDecrement = 2,
    ZoomUnit_LargeIncrement = 3,
    ZoomUnit_SmallIncrement = 4,
}}
pub type AnimationStyle = i32;
pub const AnimationStyle_None: AnimationStyle = 0;
pub const AnimationStyle_LasVegasLights: AnimationStyle = 1;
pub const AnimationStyle_BlinkingBackground: AnimationStyle = 2;
pub const AnimationStyle_SparkleText: AnimationStyle = 3;
pub const AnimationStyle_MarchingBlackAnts: AnimationStyle = 4;
pub const AnimationStyle_MarchingRedAnts: AnimationStyle = 5;
pub const AnimationStyle_Shimmer: AnimationStyle = 6;
pub const AnimationStyle_Other: AnimationStyle = -1;
pub type BulletStyle = i32;
pub const BulletStyle_None: BulletStyle = 0;
pub const BulletStyle_HollowRoundBullet: BulletStyle = 1;
pub const BulletStyle_FilledRoundBullet: BulletStyle = 2;
pub const BulletStyle_HollowSquareBullet: BulletStyle = 3;
pub const BulletStyle_FilledSquareBullet: BulletStyle = 4;
pub const BulletStyle_DashBullet: BulletStyle = 5;
pub const BulletStyle_Other: BulletStyle = -1;
pub type CapStyle = i32;
pub const CapStyle_None: CapStyle = 0;
pub const CapStyle_SmallCap: CapStyle = 1;
pub const CapStyle_AllCap: CapStyle = 2;
pub const CapStyle_AllPetiteCaps: CapStyle = 3;
pub const CapStyle_PetiteCaps: CapStyle = 4;
pub const CapStyle_Unicase: CapStyle = 5;
pub const CapStyle_Titling: CapStyle = 6;
pub const CapStyle_Other: CapStyle = -1;
ENUM!{enum FillType {
    FillType_None = 0,
    FillType_Color = 1,
    FillType_Gradient = 2,
    FillType_Picture = 3,
    FillType_Pattern = 4,
}}
ENUM!{enum FlowDirections {
    FlowDirections_Default = 0,
    FlowDirections_RightToLeft = 0x1,
    FlowDirections_BottomToTop = 0x2,
    FlowDirections_Vertical = 0x4,
}}
ENUM!{enum HorizontalTextAlignment {
    HorizontalTextAlignment_Left = 0,
    HorizontalTextAlignment_Centered = 1,
    HorizontalTextAlignment_Right = 2,
    HorizontalTextAlignment_Justified = 3,
}}
ENUM!{enum OutlineStyles {
    OutlineStyles_None = 0,
    OutlineStyles_Outline = 1,
    OutlineStyles_Shadow = 2,
    OutlineStyles_Engraved = 4,
    OutlineStyles_Embossed = 8,
}}
pub type TextDecorationLineStyle = i32;
pub const TextDecorationLineStyle_None: TextDecorationLineStyle = 0;
pub const TextDecorationLineStyle_Single: TextDecorationLineStyle = 1;
pub const TextDecorationLineStyle_WordsOnly: TextDecorationLineStyle = 2;
pub const TextDecorationLineStyle_Double: TextDecorationLineStyle = 3;
pub const TextDecorationLineStyle_Dot: TextDecorationLineStyle = 4;
pub const TextDecorationLineStyle_Dash: TextDecorationLineStyle = 5;
pub const TextDecorationLineStyle_DashDot: TextDecorationLineStyle = 6;
pub const TextDecorationLineStyle_DashDotDot: TextDecorationLineStyle = 7;
pub const TextDecorationLineStyle_Wavy: TextDecorationLineStyle = 8;
pub const TextDecorationLineStyle_ThickSingle: TextDecorationLineStyle = 9;
pub const TextDecorationLineStyle_DoubleWavy: TextDecorationLineStyle = 11;
pub const TextDecorationLineStyle_ThickWavy: TextDecorationLineStyle = 12;
pub const TextDecorationLineStyle_LongDash: TextDecorationLineStyle = 13;
pub const TextDecorationLineStyle_ThickDash: TextDecorationLineStyle = 14;
pub const TextDecorationLineStyle_ThickDashDot: TextDecorationLineStyle = 15;
pub const TextDecorationLineStyle_ThickDashDotDot: TextDecorationLineStyle = 16;
pub const TextDecorationLineStyle_ThickDot: TextDecorationLineStyle = 17;
pub const TextDecorationLineStyle_ThickLongDash: TextDecorationLineStyle = 18;
pub const TextDecorationLineStyle_Other: TextDecorationLineStyle = -1;
ENUM!{enum VisualEffects {
    VisualEffects_None = 0,
    VisualEffects_Shadow = 1 << 0,
    VisualEffects_Reflection = 1 << 1,
    VisualEffects_Glow = 1 << 2,
    VisualEffects_SoftEdges = 1 << 3,
    VisualEffects_Bevel = 1 << 4,
}}
ENUM!{enum NotificationProcessing {
    NotificationProcessing_ImportantAll = 0,
    NotificationProcessing_ImportantMostRecent = 1,
    NotificationProcessing_All = 2,
    NotificationProcessing_MostRecent = 3,
    NotificationProcessing_CurrentThenMostRecent = 4,
}}
ENUM!{enum NotificationKind {
    NotificationKind_ItemAdded = 0,
    NotificationKind_ItemRemoved = 1,
    NotificationKind_ActionCompleted = 2,
    NotificationKind_ActionAborted = 3,
    NotificationKind_Other = 4,
}}
pub type PROPERTYID = c_int;
pub type PATTERNID = c_int;
pub type EVENTID = c_int;
pub type TEXTATTRIBUTEID = c_int;
pub type CONTROLTYPEID = c_int;
pub type LANDMARKTYPEID = c_int;
pub type METADATAID = c_int;
pub type HEADINGLEVELID = c_int;
STRUCT!{struct UiaRect {
    left: c_double,
    top: c_double,
    width: c_double,
    height: c_double,
}}
STRUCT!{struct UiaPoint {
    x: c_double,
    y: c_double,
}}
STRUCT!{struct UiaChangeInfo {
    uiaId: c_int,
    payload: VARIANT,
    extraInfo: VARIANT,
}}
ENUM!{enum UIAutomationType {
    UIAutomationType_Int = 0x1,
    UIAutomationType_Bool = 0x2,
    UIAutomationType_String = 0x3,
    UIAutomationType_Double = 0x4,
    UIAutomationType_Point = 0x5,
    UIAutomationType_Rect = 0x6,
    UIAutomationType_Element = 0x7,
    UIAutomationType_Array = 0x10000,
    UIAutomationType_Out = 0x20000,
    UIAutomationType_IntArray = UIAutomationType_Int | UIAutomationType_Array,
    UIAutomationType_BoolArray = UIAutomationType_Bool | UIAutomationType_Array,
    UIAutomationType_StringArray = UIAutomationType_String | UIAutomationType_Array,
    UIAutomationType_DoubleArray = UIAutomationType_Double | UIAutomationType_Array,
    UIAutomationType_PointArray = UIAutomationType_Point | UIAutomationType_Array,
    UIAutomationType_RectArray = UIAutomationType_Rect | UIAutomationType_Array,
    UIAutomationType_ElementArray = UIAutomationType_Element | UIAutomationType_Array,
    UIAutomationType_OutInt = UIAutomationType_Int | UIAutomationType_Out,
    UIAutomationType_OutBool = UIAutomationType_Bool | UIAutomationType_Out,
    UIAutomationType_OutString = UIAutomationType_String | UIAutomationType_Out,
    UIAutomationType_OutDouble = UIAutomationType_Double | UIAutomationType_Out,
    UIAutomationType_OutPoint = UIAutomationType_Point | UIAutomationType_Out,
    UIAutomationType_OutRect = UIAutomationType_Rect | UIAutomationType_Out,
    UIAutomationType_OutElement = UIAutomationType_Element | UIAutomationType_Out,
    UIAutomationType_OutIntArray = (UIAutomationType_Int | UIAutomationType_Array)
        | UIAutomationType_Out,
    UIAutomationType_OutBoolArray = (UIAutomationType_Bool | UIAutomationType_Array)
        | UIAutomationType_Out,
    UIAutomationType_OutStringArray = (UIAutomationType_String | UIAutomationType_Array)
        | UIAutomationType_Out,
    UIAutomationType_OutDoubleArray = (UIAutomationType_Double | UIAutomationType_Array)
        | UIAutomationType_Out,
    UIAutomationType_OutPointArray = (UIAutomationType_Point | UIAutomationType_Array)
        | UIAutomationType_Out,
    UIAutomationType_OutRectArray = (UIAutomationType_Rect | UIAutomationType_Array)
        | UIAutomationType_Out,
    UIAutomationType_OutElementArray = (UIAutomationType_Element | UIAutomationType_Array)
        | UIAutomationType_Out,
}}
//DEFINE_ENUM_FLAG_OPERATORS(UIAutomationType);
STRUCT!{struct UIAutomationParameter {
    type_: UIAutomationType,
    pData: *mut c_void,
}}
STRUCT!{struct UIAutomationPropertyInfo {
    guid: GUID,
    pProgrammaticName: LPCWSTR,
    type_: UIAutomationType,
}}
STRUCT!{struct UIAutomationEventInfo {
    guid: GUID,
    pProgrammaticName: LPCWSTR,
}}
STRUCT!{struct UIAutomationMethodInfo {
    pProgrammaticName: LPCWSTR,
    doSetFocus: BOOL,
    cInParameters: UINT,
    cOutParameters: UINT,
    pParameterTypes: *mut UIAutomationType,
    pParameterNames: *mut LPCWSTR,
}}
STRUCT!{struct UIAutomationPatternInfo {
    guid: GUID,
    pProgrammaticName: LPCWSTR,
    providerInterfaceId: GUID,
    clientInterfaceId: GUID,
    cProperties: UINT,
    pProperties: *mut UIAutomationPropertyInfo,
    cMethods: UINT,
    pMethods: *mut UIAutomationMethodInfo,
    cEvents: UINT,
    pEvents: *mut UIAutomationEventInfo,
    pPatternHandler: *mut IUIAutomationPatternHandler,
}}
pub const UIA_ScrollPatternNoScroll: c_double = -1.0;
RIDL!{#[uuid(0xd6dd68d1, 0x86fd, 0x4332, 0x86, 0x66, 0x9a, 0xbe, 0xde, 0xa2, 0xd2, 0x4c)]
interface IRawElementProviderSimple(IRawElementProviderSimpleVtbl): IUnknown(IUnknownVtbl) {
    fn get_ProviderOptions(
        pRetVal: *mut ProviderOptions,
    ) -> HRESULT,
    fn GetPatternProvider(
        patternId: PATTERNID,
        pRetVal: *mut *mut IUnknown,
    ) -> HRESULT,
    fn GetPropertyValue(
        propertyId: PROPERTYID,
        pRetVal: *mut *mut VARIANT,
    ) -> HRESULT,
    fn get_HostRawElementProvider(
        pRetVal: *mut *mut IRawElementProviderSimple,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xf8b80ada, 0x2c44, 0x48d0, 0x89, 0xbe, 0x5f, 0xf2, 0x3c, 0x9c, 0xd8, 0x75)]
interface IAccessibleEx(IAccessibleExVtbl): IUnknown(IUnknownVtbl) {
    fn GetObjectForChild(
        idChild: c_long,
        pRetVal: *mut *mut IAccessibleEx,
    ) -> HRESULT,
    fn GetIAccessiblePair(
        ppAcc: *mut *mut IAccessible,
        pidChild: *mut c_long,
    ) -> HRESULT,
    fn GetRuntimeId(
        pRetVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    fn ConvertReturnedElement (
        pIn: *mut IRawElementProviderSimple,
        ppRetValOut: *mut *mut IAccessibleEx,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xa0a839a9, 0x8da1, 0x4a82, 0x80, 0x6a, 0x8e, 0x0d, 0x44, 0xe7, 0x9f, 0x56)]
interface IRawElementProviderSimple2(IRawElementProviderSimple2Vtbl):
        IRawElementProviderSimple(IRawElementProviderSimpleVtbl) {
    fn ShowContextMenu() -> HRESULT,
}}
RIDL!{#[uuid(0xfcf5d820, 0xd7ec, 0x4613, 0xbd, 0xf6, 0x42, 0xa8, 0x4c, 0xe7, 0xda, 0xaf)]
interface IRawElementProviderSimple3(IRawElementProviderSimple3Vtbl):
        IRawElementProviderSimple2(IRawElementProviderSimple2Vtbl) {
    fn GetMetadataValue(
        targetId: c_int,
        metadataId: METADATAID,
        returnVal: *mut VARIANT,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x620ce2a5, 0xab8f, 0x40a9, 0x86, 0xcb, 0xde, 0x3c, 0x75, 0x59, 0x9b, 0x58)]
interface IRawElementProviderFragmentRoot(IRawElementProviderFragmentRootVtbl):
        IUnknown(IUnknownVtbl) {
    fn ElementProviderFromPoint(
        x: c_double,
        y: c_double,
        pRetVal: *mut *mut IRawElementProviderFragment,
    ) -> HRESULT,
    fn GetFocus(
        pRetVal: *mut *mut IRawElementProviderFragment,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xf7063da8, 0x8359, 0x439c, 0x92, 0x97, 0xbb, 0xc5, 0x29, 0x9a, 0x7d, 0x87)]
interface IRawElementProviderFragment(IRawElementProviderFragmentVtbl): IUnknown(IUnknownVtbl) {
    fn Navigate(
        direction: NavigateDirection,
        pRetVal: *mut *mut IRawElementProviderFragment,
    ) -> HRESULT,
    fn GetRuntimeId(
        pRetVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    fn get_BoundingRectangle(
        pRetVal: *mut UiaRect,
    ) -> HRESULT,
    fn GetEmbeddedFragmentRoots(
        pRetVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    fn SetFocus() -> HRESULT,
    fn get_FragmentRoot(
        pRetVal: *mut *mut IRawElementProviderFragmentRoot,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xa407b27b, 0x0f6d, 0x4427, 0x92, 0x92, 0x47, 0x3c, 0x7b, 0xf9, 0x32, 0x58)]
interface IRawElementProviderAdviseEvents(IRawElementProviderAdviseEventsVtbl):
        IUnknown(IUnknownVtbl) {
    fn AdviseEventAdded(
        eventId: EVENTID,
        propertyIDs: *mut SAFEARRAY,
    ) -> HRESULT,
    fn AdviseEventRemoved(
        eventId: EVENTID,
        propertyIDs: *mut SAFEARRAY,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x1d5df27c, 0x8947, 0x4425, 0xb8, 0xd9, 0x79, 0x78, 0x7b, 0xb4, 0x60, 0xb8)]
interface IRawElementProviderHwndOverride(IRawElementProviderHwndOverrideVtbl):
        IUnknown(IUnknownVtbl) {
    fn GetOverrideProviderForHwnd(
        hwnd: HWND,
        pRetVal: *mut *mut IRawElementProviderSimple,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x4fd82b78, 0xa43e, 0x46ac, 0x98, 0x03, 0x0a, 0x69, 0x69, 0xc7, 0xc1, 0x83)]
interface IProxyProviderWinEventSink(IProxyProviderWinEventSinkVtbl): IUnknown(IUnknownVtbl) {
    fn AddAutomationPropertyChangedEvent(
        pProvider: *mut IRawElementProviderSimple,
        id: PROPERTYID,
        newValue: VARIANT,
    ) -> HRESULT,
    fn AddAutomationEvent(
        pProvider: *mut IRawElementProviderSimple,
        id: EVENTID,
    ) -> HRESULT,
    fn AddStructureChangedEvent(
        pProvider: *mut IRawElementProviderSimple,
        structureChangeType: StructureChangeType,
        runtimeId: *mut SAFEARRAY,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x89592ad4, 0xf4e0, 0x43d5, 0xa3, 0xb6, 0xba, 0xd7, 0xe1, 0x11, 0xb4, 0x35)]
interface IProxyProviderWinEventHandler(IProxyProviderWinEventHandlerVtbl):
        IUnknown(IUnknownVtbl) {
    fn RespondToWinEvent(
        idWinEvent: DWORD,
        hwnd: HWND,
        idObject: c_long,
        idChild: c_long,
        pSink: *mut IProxyProviderWinEventSink,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x0a2a93cc, 0xbfad, 0x42ac, 0x9b, 0x2e, 0x09, 0x91, 0xfb, 0x0d, 0x3e, 0xa0)]
interface IRawElementProviderWindowlessSite(IRawElementProviderWindowlessSiteVtbl):
        IUnknown(IUnknownVtbl) {
    fn GetAdjacentFragment(
        direction: NavigateDirection,
        ppParent: *mut *mut IRawElementProviderFragment,
    ) -> HRESULT,
    fn GetRuntimeIdPrefix(
        pRetVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x33ac331b, 0x943e, 0x4020, 0xb2, 0x95, 0xdb, 0x37, 0x78, 0x49, 0x74, 0xa3)]
interface IAccessibleHostingElementProviders(IAccessibleHostingElementProvidersVtbl):
        IUnknown(IUnknownVtbl) {
    fn GetEmbeddedFragmentRoots(
        pRetVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    fn GetObjectIdForProvider(
        pProvider: *mut IRawElementProviderSimple,
        pidObject: *mut c_long,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x24be0b07, 0xd37d, 0x487a, 0x98, 0xcf, 0xa1, 0x3e, 0xd4, 0x65, 0xe9, 0xb3)]
interface IRawElementProviderHostingAccessibles(IRawElementProviderHostingAccessiblesVtbl):
        IUnknown(IUnknownVtbl) {
    fn GetEmbeddedAccessibles(
        pRetVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x159bc72c, 0x4ad3, 0x485e, 0x96, 0x37, 0xd7, 0x05, 0x2e, 0xdf, 0x01, 0x46)]
interface IDockProvider(IDockProviderVtbl): IUnknown(IUnknownVtbl) {
    fn SetDockPosition(
        dockPosition: DockPosition,
    ) -> HRESULT,
    fn get_DockPosition(
        pRetVal: *mut DockPosition,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xd847d3a5, 0xcab0, 0x4a98, 0x8c, 0x32, 0xec, 0xb4, 0x5c, 0x59, 0xad, 0x24)]
interface IExpandCollapseProvider(IExpandCollapseProviderVtbl): IUnknown(IUnknownVtbl) {
    fn Expand() -> HRESULT,
    fn Collapse() -> HRESULT,
    fn get_ExpandCollapseState(
        pRetVal: *mut ExpandCollapseState,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xb17d6187, 0x0907, 0x464b, 0xa1, 0x68, 0x0e, 0xf1, 0x7a, 0x15, 0x72, 0xb1)]
interface IGridProvider(IGridProviderVtbl): IUnknown(IUnknownVtbl) {
    fn GetItem(
        row: c_int,
        column: c_int,
        pRetVal: *mut *mut IRawElementProviderSimple,
    ) -> HRESULT,
    fn get_RowCount(
        pRetVal: *mut c_int,
    ) -> HRESULT,
    fn get_ColumnCount(
        pRetVal: *mut c_int,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xd02541f1, 0xfb81, 0x4d64, 0xae, 0x32, 0xf5, 0x20, 0xf8, 0xa6, 0xdb, 0xd1)]
interface IGridItemProvider(IGridItemProviderVtbl): IUnknown(IUnknownVtbl) {
    fn get_Row(
        pRetVal: *mut c_int,
    ) -> HRESULT,
    fn get_Column(
        pRetVal: *mut c_int,
    ) -> HRESULT,
    fn get_RowSpan(
        pRetVal: *mut c_int,
    ) -> HRESULT,
    fn get_ColumnSpan(
        pRetVal: *mut c_int,
    ) -> HRESULT,
    fn get_ContainingGrid(
        pRetVal: *mut *mut IRawElementProviderSimple,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x54fcb24b, 0xe18e, 0x47a2, 0xb4, 0xd3, 0xec, 0xcb, 0xe7, 0x75, 0x99, 0xa2)]
interface IInvokeProvider(IInvokeProviderVtbl): IUnknown(IUnknownVtbl) {
    fn Invoke() -> HRESULT,
}}
RIDL!{#[uuid(0x6278cab1, 0xb556, 0x4a1a, 0xb4, 0xe0, 0x41, 0x8a, 0xcc, 0x52, 0x32, 0x01)]
interface IMultipleViewProvider(IMultipleViewProviderVtbl): IUnknown(IUnknownVtbl) {
    fn GetViewName(
        viewId: c_int,
        pRetVal: *mut BSTR,
    ) -> HRESULT,
    fn SetCurrentView(
        viewId: c_int,
    ) -> HRESULT,
    fn get_CurrentView(
        pRetVal: *mut c_int,
    ) -> HRESULT,
    fn GetSupportedViews(
        pRetVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x36dc7aef, 0x33e6, 0x4691, 0xaf, 0xe1, 0x2b, 0xe7, 0x27, 0x4b, 0x3d, 0x33)]
interface IRangeValueProvider(IRangeValueProviderVtbl): IUnknown(IUnknownVtbl) {
    fn SetValue(
        val: c_double,
    ) -> HRESULT,
    fn get_Value(
        pRetVal: *mut c_double,
    ) -> HRESULT,
    fn get_IsReadOnly(
        pRetVal: *mut BOOL,
    ) -> HRESULT,
    fn get_Maximum(
        pRetVal: *mut c_double,
    ) -> HRESULT,
    fn get_Minimum(
        pRetVal: *mut c_double,
    ) -> HRESULT,
    fn get_LargeChange(
        pRetVal: *mut c_double,
    ) -> HRESULT,
    fn get_SmallChange(
        pRetVal: *mut c_double,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x2360c714, 0x4bf1, 0x4b26, 0xba, 0x65, 0x9b, 0x21, 0x31, 0x61, 0x27, 0xeb)]
interface IScrollItemProvider(IScrollItemProviderVtbl): IUnknown(IUnknownVtbl) {
    fn ScrollIntoView() -> HRESULT,
}}
RIDL!{#[uuid(0xfb8b03af, 0x3bdf, 0x48d4, 0xbd, 0x36, 0x1a, 0x65, 0x79, 0x3b, 0xe1, 0x68)]
interface ISelectionProvider(ISelectionProviderVtbl): IUnknown(IUnknownVtbl) {
    fn GetSelection(
        pRetVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    fn get_CanSelectMultiple(
        pRetVal: *mut BOOL,
    ) -> HRESULT,
    fn get_IsSelectionRequired(
        pRetVal: *mut BOOL,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x14f68475, 0xee1c, 0x44f6, 0xa8, 0x69, 0xd2, 0x39, 0x38, 0x1f, 0x0f, 0xe7)]
interface ISelectionProvider2(ISelectionProvider2Vtbl):
        ISelectionProvider(ISelectionProviderVtbl) {
    fn get_FirstSelectedItem(
        retVal: *mut *mut IRawElementProviderSimple,
    ) -> HRESULT,
    fn get_LastSelectedItem(
        retVal: *mut *mut IRawElementProviderSimple,
    ) -> HRESULT,
    fn get_CurrentSelectedItem(
        retVal: *mut *mut IRawElementProviderSimple,
    ) -> HRESULT,
    fn get_ItemCount(
        retVal: *mut c_int,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xb38b8077, 0x1fc3, 0x42a5, 0x8c, 0xae, 0xd4, 0x0c, 0x22, 0x15, 0x05, 0x5a)]
interface IScrollProvider(IScrollProviderVtbl): IUnknown(IUnknownVtbl) {
    fn Scroll(
        horizontalAmount: ScrollAmount,
        verticalAmount: ScrollAmount,
    ) -> HRESULT,
    fn SetScrollPercent(
        horizontalPercent: c_double,
        verticalPercent: c_double,
    ) -> HRESULT,
    fn get_HorizontalScrollPercent(
        pRetVal: *mut c_double,
    ) -> HRESULT,
    fn get_VerticalScrollPercent(
        pRetVal: *mut c_double,
    ) -> HRESULT,
    fn get_HorizontalViewSize(
        pRetVal: *mut c_double,
    ) -> HRESULT,
    fn get_VerticalViewSize(
        pRetVal: *mut c_double,
    ) -> HRESULT,
    fn get_HorizontallyScrollable(
        pRetVal: *mut BOOL,
    ) -> HRESULT,
    fn get_VerticallyScrollable(
        pRetVal: *mut BOOL,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x2acad808, 0xb2d4, 0x452d, 0xa4, 0x07, 0x91, 0xff, 0x1a, 0xd1, 0x67, 0xb2)]
interface ISelectionItemProvider(ISelectionItemProviderVtbl): IUnknown(IUnknownVtbl) {
    fn Select() -> HRESULT,
    fn AddToSelection() -> HRESULT,
    fn RemoveFromSelection() -> HRESULT,
    fn get_IsSelected(
        pRetVal: *mut BOOL,
    ) -> HRESULT,
    fn get_SelectionContainer(
        pRetVal: *mut *mut IRawElementProviderSimple,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x29db1a06, 0x02ce, 0x4cf7, 0x9b, 0x42, 0x56, 0x5d, 0x4f, 0xab, 0x20, 0xee)]
interface ISynchronizedInputProvider(ISynchronizedInputProviderVtbl): IUnknown(IUnknownVtbl) {
    fn StartListening(
        inputType: SynchronizedInputType,
    ) -> HRESULT,
    fn Cancel() -> HRESULT,
}}
RIDL!{#[uuid(0x9c860395, 0x97b3, 0x490a, 0xb5, 0x2a, 0x85, 0x8c, 0xc2, 0x2a, 0xf1, 0x66)]
interface ITableProvider(ITableProviderVtbl): IUnknown(IUnknownVtbl) {
    fn GetRowHeaders(
        pRetVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    fn GetColumnHeaders(
        pRetVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    fn get_RowOrColumnMajor(
        pRetVal: *mut RowOrColumnMajor,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xb9734fa6, 0x771f, 0x4d78, 0x9c, 0x90, 0x25, 0x17, 0x99, 0x93, 0x49, 0xcd)]
interface ITableItemProvider(ITableItemProviderVtbl): IUnknown(IUnknownVtbl) {
    fn GetRowHeaderItems(
        pRetVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    fn GetColumnHeaderItems(
        pRetVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x56d00bd0, 0xc4f4, 0x433c, 0xa8, 0x36, 0x1a, 0x52, 0xa5, 0x7e, 0x08, 0x92)]
interface IToggleProvider(IToggleProviderVtbl): IUnknown(IUnknownVtbl) {
    fn Toggle() -> HRESULT,
    fn get_ToggleState(
        pRetVal: *mut ToggleState,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x6829ddc4, 0x4f91, 0x4ffa, 0xb8, 0x6f, 0xbd, 0x3e, 0x29, 0x87, 0xcb, 0x4c)]
interface ITransformProvider(ITransformProviderVtbl): IUnknown(IUnknownVtbl) {
    fn Move(
        x: c_double,
        y: c_double,
    ) -> HRESULT,
    fn Resize(
        width: c_double,
        height: c_double,
    ) -> HRESULT,
    fn Rotate(
        degrees: c_double,
    ) -> HRESULT,
    fn get_CanMove(
        pRetVal: *mut BOOL,
    ) -> HRESULT,
    fn get_CanResize(
        pRetVal: *mut BOOL,
    ) -> HRESULT,
    fn get_CanRotate(
        pRetVal: *mut BOOL,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xc7935180, 0x6fb3, 0x4201, 0xb1, 0x74, 0x7d, 0xf7, 0x3a, 0xdb, 0xf6, 0x4a)]
interface IValueProvider(IValueProviderVtbl): IUnknown(IUnknownVtbl) {
    fn SetValue(
        val: LPCWSTR,
    ) -> HRESULT,
    fn get_Value(
        pRetVal: *mut BSTR,
    ) -> HRESULT,
    fn get_IsReadOnly(
        pRetVal: *mut BOOL,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x987df77b, 0xdb06, 0x4d77, 0x8f, 0x8a, 0x86, 0xa9, 0xc3, 0xbb, 0x90, 0xb9)]
interface IWindowProvider(IWindowProviderVtbl): IUnknown(IUnknownVtbl) {
    fn SetVisualState(
        state: WindowVisualState,
    ) -> HRESULT,
    fn Close() -> HRESULT,
    fn WaitForInputIdle(
        milliseconds: c_int,
        pRetVal: *mut BOOL,
    ) -> HRESULT,
    fn get_CanMaximize(
        pRetVal: *mut BOOL,
    ) -> HRESULT,
    fn get_CanMinimize(
        pRetVal: *mut BOOL,
    ) -> HRESULT,
    fn get_IsModal(
        pRetVal: *mut BOOL,
    ) -> HRESULT,
    fn get_WindowVisualState(
        pRetVal: *mut WindowVisualState,
    ) -> HRESULT,
    fn get_WindowInteractionState(
        pRetVal: *mut WindowInteractionState,
    ) -> HRESULT,
    fn get_IsTopmost(
        pRetVal: *mut BOOL,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xe44c3566, 0x915d, 0x4070, 0x99, 0xc6, 0x04, 0x7b, 0xff, 0x5a, 0x08, 0xf5)]
interface ILegacyIAccessibleProvider(ILegacyIAccessibleProviderVtbl): IUnknown(IUnknownVtbl) {
    fn Select(
        flagsSelect: c_long,
    ) -> HRESULT,
    fn DoDefaultAction() -> HRESULT,
    fn SetValue(
        szValue: LPCWSTR,
    ) -> HRESULT,
    fn GetIAccessible(
        ppIAccessible: *mut *mut IAccessible,
    ) -> HRESULT,
    fn get_ChildId(
        pRetVal: *mut c_int,
    ) -> HRESULT,
    fn get_Name(
        pszName: *mut BSTR,
    ) -> HRESULT,
    fn get_Value(
        pszValue: *mut BSTR,
    ) -> HRESULT,
    fn get_Description(
        pszDescription: *mut BSTR,
    ) -> HRESULT,
    fn get_Role(
        pdwRole: *mut DWORD,
    ) -> HRESULT,
    fn get_State(
        pdwState: *mut DWORD,
    ) -> HRESULT,
    fn get_Help(
        pszHelp: *mut BSTR,
    ) -> HRESULT,
    fn get_KeyboardShortcut(
        pszKeyboardShortcut: *mut BSTR,
    ) -> HRESULT,
    fn GetSelection(
        pvarSelectedChildren: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    fn get_DefaultAction(
        pszDefaultAction: *mut BSTR,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xe747770b, 0x39ce, 0x4382, 0xab, 0x30, 0xd8, 0xfb, 0x3f, 0x33, 0x6f, 0x24)]
interface IItemContainerProvider(IItemContainerProviderVtbl): IUnknown(IUnknownVtbl) {
    fn FindItemByProperty(
        pStartAfter: *mut IRawElementProviderSimple,
        propertyId: PROPERTYID,
        value: VARIANT,
        pFound: *mut *mut IRawElementProviderSimple,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xcb98b665, 0x2d35, 0x4fac, 0xad, 0x35, 0xf3, 0xc6, 0x0d, 0x0c, 0x0b, 0x8b)]
interface IVirtualizedItemProvider(IVirtualizedItemProviderVtbl): IUnknown(IUnknownVtbl) {
    fn Realize() -> HRESULT,
}}
RIDL!{#[uuid(0x3ad86ebd, 0xf5ef, 0x483d, 0xbb, 0x18, 0xb1, 0x04, 0x2a, 0x47, 0x5d, 0x64)]
interface IObjectModelProvider(IObjectModelProviderVtbl): IUnknown(IUnknownVtbl) {
    fn GetUnderlyingObjectModel(
        ppUnknown: *mut *mut IUnknown,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xf95c7e80, 0xbd63, 0x4601, 0x97, 0x82, 0x44, 0x5e, 0xbf, 0xf0, 0x11, 0xfc)]
interface IAnnotationProvider(IAnnotationProviderVtbl): IUnknown(IUnknownVtbl) {
    fn get_AnnotationTypeId(
        retVal: *mut c_int,
    ) -> HRESULT,
    fn get_AnnotationTypeName(
        retVal: *mut BSTR,
    ) -> HRESULT,
    fn get_Author(
        retVal: *mut BSTR,
    ) -> HRESULT,
    fn get_DateTime(
        retVal: *mut BSTR,
    ) -> HRESULT,
    fn get_Target(
        retVal: *mut *mut IRawElementProviderSimple,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x19b6b649, 0xf5d7, 0x4a6d, 0xbd, 0xcb, 0x12, 0x92, 0x52, 0xbe, 0x58, 0x8a)]
interface IStylesProvider(IStylesProviderVtbl): IUnknown(IUnknownVtbl) {
    fn get_StyleId(
        retVal: *mut c_int,
    ) -> HRESULT,
    fn get_StyleName(
        retVal: *mut BSTR,
    ) -> HRESULT,
    fn get_FillColor(
        retVal: *mut c_int,
    ) -> HRESULT,
    fn get_FillPatternStyle(
        retVal: *mut BSTR,
    ) -> HRESULT,
    fn get_Shape(
        retVal: BSTR,
    ) -> HRESULT,
    fn get_FillPatternColor(
        retVal: *mut c_int,
    ) -> HRESULT,
    fn get_ExtendedProperties(
        retVal: *mut BSTR,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x6f6b5d35, 0x5525, 0x4f80, 0xb7, 0x58, 0x85, 0x47, 0x38, 0x32, 0xff, 0xc7)]
interface ISpreadsheetProvider(ISpreadsheetProviderVtbl): IUnknown(IUnknownVtbl) {
    fn GetItemByName(
        name: LPCWSTR,
        pRetVal: *mut *mut IRawElementProviderSimple,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xeaed4660, 0x7b3d, 0x4879, 0xa2, 0xe6, 0x36, 0x5c, 0xe6, 0x03, 0xf3, 0xd0)]
interface ISpreadsheetItemProvider(ISpreadsheetItemProviderVtbl): IUnknown(IUnknownVtbl) {
    fn get_Formula(
        pRetVal: *mut BSTR,
    ) -> HRESULT,
    fn GetAnnotationObjects(
        pRetVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    fn GetAnnotationTypes(
        pRetVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x4758742f, 0x7ac2, 0x460c, 0xbc, 0x48, 0x09, 0xfc, 0x09, 0x30, 0x8a, 0x93)]
interface ITransformProvider2(ITransformProvider2Vtbl):
        ITransformProvider(ITransformProviderVtbl) {
    fn Zoom(
        zoom: c_double,
    ) -> HRESULT,
    fn get_CanZoom(
        pRetVal: *mut BOOL,
    ) -> HRESULT,
    fn get_ZoomLevel(
        pRetVal: *mut c_double,
    ) -> HRESULT,
    fn get_ZoomMinimum(
        pRetVal: *mut c_double,
    ) -> HRESULT,
    fn get_ZoomMaximum(
        pRetVal: *mut c_double,
    ) -> HRESULT,
    fn ZoomByUnit(
        zoomUnit: ZoomUnit,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x6aa7bbbb, 0x7ff9, 0x497d, 0x90, 0x4f, 0xd2, 0x0b, 0x89, 0x79, 0x29, 0xd8)]
interface IDragProvider(IDragProviderVtbl): IUnknown(IUnknownVtbl) {
    fn get_IsGrabbed(
        pRetVal: *mut BOOL,
    ) -> HRESULT,
    fn get_DropEffect(
        pRetVal: *mut BSTR,
    ) -> HRESULT,
    fn get_DropEffects(
        pRetVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    fn GetGrabbedItems(
        pRetVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xbae82bfd, 0x358a, 0x481c, 0x85, 0xa0, 0xd8, 0xb4, 0xd9, 0x0a, 0x5d, 0x61)]
interface IDropTargetProvider(IDropTargetProviderVtbl): IUnknown(IUnknownVtbl) {
    fn get_DropTargetEffect(
        pRetVal: *mut BSTR,
    ) -> HRESULT,
    fn get_DropTargetEffects(
        pRetVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x5347ad7b, 0xc355, 0x46f8, 0xaf, 0xf5, 0x90, 0x90, 0x33, 0x58, 0x2f, 0x63)]
interface ITextRangeProvider(ITextRangeProviderVtbl): IUnknown(IUnknownVtbl) {
    fn Clone(
        pRetVal: *mut *mut ITextRangeProvider,
    ) -> HRESULT,
    fn Compare(
        range: *mut ITextRangeProvider,
        pRetVal: *mut BOOL,
    ) -> HRESULT,
    fn CompareEndpoints(
        endpoint: TextPatternRangeEndpoint,
        targetRange: *mut ITextRangeProvider,
        targetEndpoint: TextPatternRangeEndpoint,
        pRetVal: *mut c_int,
    ) -> HRESULT,
    fn ExpandToEnclosingUnit(
        unit: TextUnit,
    ) -> HRESULT,
    fn FindAttribute(
        attributeId: TEXTATTRIBUTEID,
        val: VARIANT,
        backward: BOOL,
        pRetVal: *mut *mut ITextRangeProvider,
    ) -> HRESULT,
    fn FindText(
        text: BSTR,
        backward: BOOL,
        ignoreCase: BOOL,
        pRetVal: *mut *mut ITextRangeProvider,
    ) -> HRESULT,
    fn GetAttributeValue(
        attributeId: TEXTATTRIBUTEID,
        pRetVal: *mut VARIANT,
    ) -> HRESULT,
    fn GetBoundingRectangles(
        pRetVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    fn GetEnclosingElement(
        pRetVal: *mut *mut IRawElementProviderSimple,
    ) -> HRESULT,
    fn GetText(
        maxLength: c_int,
        pRetVal: *mut BSTR,
    ) -> HRESULT,
    fn Move(
        unit: TextUnit,
        count: c_int,
        pRetVal: *mut c_int,
    ) -> HRESULT,
    fn MoveEndpointByUnit(
        endpoint: TextPatternRangeEndpoint,
        unit: TextUnit,
        count: c_int,
        pRetVal: *mut c_int,
    ) -> HRESULT,
    fn MoveEndpointByRange(
        endpoint: TextPatternRangeEndpoint,
        targetRange: *mut ITextRangeProvider,
        targetEndpoint: TextPatternRangeEndpoint,
    ) -> HRESULT,
    fn Select() -> HRESULT,
    fn AddToSelection() -> HRESULT,
    fn RemoveFromSelection() -> HRESULT,
    fn ScrollIntoView(
        alignToTop: BOOL,
    ) -> HRESULT,
    fn GetChildren(
        pRetVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x3589c92c, 0x63f3, 0x4367, 0x99, 0xbb, 0xad, 0xa6, 0x53, 0xb7, 0x7c, 0xf2)]
interface ITextProvider(ITextProviderVtbl): IUnknown(IUnknownVtbl) {
    fn GetSelection(
        pRetVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    fn GetVisibleRanges(
        pRetVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    fn RangeFromChild(
        childElement: *mut IRawElementProviderSimple,
        pRetVal: *mut *mut ITextRangeProvider,
    ) -> HRESULT,
    fn RangeFromPoint(
        point: UiaPoint,
        pRetVal: *mut *mut ITextRangeProvider,
    ) -> HRESULT,
    fn get_DocumentRange(
        pRetVal: *mut *mut ITextRangeProvider,
    ) -> HRESULT,
    fn get_SupportedTextSelection(
        pRetVal: *mut SupportedTextSelection,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x0dc5e6ed, 0x3e16, 0x4bf1, 0x8f, 0x9a, 0xa9, 0x79, 0x87, 0x8b, 0xc1, 0x95)]
interface ITextProvider2(ITextProvider2Vtbl): ITextProvider(ITextProviderVtbl) {
    fn RangeFromAnnotation(
        annotationElement: *mut IRawElementProviderSimple,
        pRetVal: *mut *mut ITextRangeProvider,
    ) -> HRESULT,
    fn GetCaretRange(
        isActive: *mut BOOL,
        pRetVal: *mut *mut ITextRangeProvider,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xea3605b4, 0x3a05, 0x400e, 0xb5, 0xf9, 0x4e, 0x91, 0xb4, 0x0f, 0x61, 0x76)]
interface ITextEditProvider(ITextEditProviderVtbl): ITextProvider(ITextProviderVtbl) {
    fn GetActiveComposition(
        pRetVal: *mut *mut ITextRangeProvider,
    ) -> HRESULT,
    fn GetConversionTarget(
        pRetVal: *mut *mut ITextRangeProvider,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x9bbce42c, 0x1921, 0x4f18, 0x89, 0xca, 0xdb, 0xa1, 0x91, 0x0a, 0x03, 0x86)]
interface ITextRangeProvider2(ITextRangeProvider2Vtbl):
        ITextRangeProvider(ITextRangeProviderVtbl) {
    fn ShowContextMenu() -> HRESULT,
}}
RIDL!{#[uuid(0x4c2de2b9, 0xc88f, 0x4f88, 0xa1, 0x11, 0xf1, 0xd3, 0x36, 0xb7, 0xd1, 0xa9)]
interface ITextChildProvider(ITextChildProviderVtbl): IUnknown(IUnknownVtbl) {
    fn get_TextContainer(
        pRetVal: *mut *mut IRawElementProviderSimple,
    ) -> HRESULT,
    fn get_TextRange(
        pRetVal: *mut *mut ITextRangeProvider,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x2062a28a, 0x8c07, 0x4b94, 0x8e, 0x12, 0x70, 0x37, 0xc6, 0x22, 0xae, 0xb8)]
interface ICustomNavigationProvider(ICustomNavigationProviderVtbl): IUnknown(IUnknownVtbl) {
    fn Navigate(
        direction: NavigateDirection,
        pRetVal: *mut *mut IRawElementProviderSimple,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xc03a7fe4, 0x9431, 0x409f, 0xbe, 0xd8, 0xae, 0x7c, 0x22, 0x99, 0xbc, 0x8d)]
interface IUIAutomationPatternInstance(IUIAutomationPatternInstanceVtbl): IUnknown(IUnknownVtbl) {
    fn GetProperty(
        index: UINT,
        cached: BOOL,
        type_: UIAutomationType,
        pPtr: *mut c_void,
    ) -> HRESULT,
    fn CallMethod(
        index: UINT,
        pParams: *const UIAutomationParameter,
        cParams: UINT,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xd97022f3, 0xa947, 0x465e, 0x8b, 0x2a, 0xac, 0x43, 0x15, 0xfa, 0x54, 0xe8)]
interface IUIAutomationPatternHandler(IUIAutomationPatternHandlerVtbl): IUnknown(IUnknownVtbl) {
    fn CreateClientWrapper(
        pPatternInstance: *mut IUIAutomationPatternInstance,
        pClientWrapper: *mut *mut IUnknown,
    ) -> HRESULT,
    fn Dispatch(
        pTarget: *mut IUnknown,
        index: UINT,
        pParams: *const UIAutomationParameter,
        cParams: UINT,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x8609c4ec, 0x4a1a, 0x4d88, 0xa3, 0x57, 0x5a, 0x66, 0xe0, 0x60, 0xe1, 0xcf)]
interface IUIAutomationRegistrar(IUIAutomationRegistrarVtbl): IUnknown(IUnknownVtbl) {
    fn RegisterProperty(
        property: *const UIAutomationPropertyInfo,
        propertyId: *mut PROPERTYID,
    ) -> HRESULT,
    fn RegisterEvent(
        event: *const UIAutomationEventInfo,
        eventId: *mut EVENTID,
    ) -> HRESULT,
    fn RegisterPattern(
        pattern: *const UIAutomationPatternInfo,
        pPatternId: *mut PATTERNID,
        pPatternAvailablePropertyId: *mut PROPERTYID,
        propertyIdCount: UINT,
        pPropertyIds: *mut PROPERTYID,
        eventIdCount: UINT,
        pEventIds: *mut EVENTID,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x6e29fabf, 0x9977, 0x42d1, 0x8d, 0x0e, 0xca, 0x7e, 0x61, 0xad, 0x87, 0xe6)]
class CUIAutomationRegistrar;}

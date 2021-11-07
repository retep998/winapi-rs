// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Mappings for the contents of UIAutomationClient.h
use ctypes::{c_double, c_int, c_long, c_void};
use shared::guiddef::REFIID;
use shared::minwindef::{BOOL, DWORD, UINT};
use shared::windef::{POINT, RECT};
use shared::winerror::HRESULT;
use shared::wtypes::BSTR;
use um::oaidl::{SAFEARRAY, VARIANT};
use um::oleacc::IAccessible;
use um::uiautomationcore::{
    CONTROLTYPEID, DockPosition, EVENTID, ExpandCollapseState, HEADINGLEVELID,
    IRawElementProviderSimple, LANDMARKTYPEID, LiveSetting, METADATAID, NavigateDirection,
    NotificationKind, NotificationProcessing, OrientationType, PATTERNID, PROPERTYID,
    RowOrColumnMajor, ScrollAmount, StructureChangeType, SupportedTextSelection,
    SynchronizedInputType, TEXTATTRIBUTEID, TextEditChangeType, TextPatternRangeEndpoint,
    TextUnit, ToggleState, UiaChangeInfo, WindowInteractionState, WindowVisualState, ZoomUnit
};
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::{LONG, LPCWSTR};
ENUM!{enum TreeScope {
    TreeScope_None = 0,
    TreeScope_Element = 0x1,
    TreeScope_Children = 0x2,
    TreeScope_Descendants = 0x4,
    TreeScope_Parent = 0x8,
    TreeScope_Ancestors = 0x10,
    TreeScope_Subtree = (TreeScope_Element | TreeScope_Children) | TreeScope_Descendants,
}}
ENUM!{enum PropertyConditionFlags {
    PropertyConditionFlags_None = 0,
    PropertyConditionFlags_IgnoreCase = 0x1,
    PropertyConditionFlags_MatchSubstring = 0x2,
}}
ENUM!{enum AutomationElementMode {
    AutomationElementMode_None = 0,
    AutomationElementMode_Full = AutomationElementMode_None + 1,
}}
ENUM!{enum TreeTraversalOptions {
    TreeTraversalOptions_Default = 0,
    TreeTraversalOptions_PostOrder = 0x1,
    TreeTraversalOptions_LastToFirstOrder = 0x2,
}}
ENUM!{enum ConnectionRecoveryBehaviorOptions {
    ConnectionRecoveryBehaviorOptions_Disabled = 0,
    ConnectionRecoveryBehaviorOptions_Enabled = 0x1,
}}
ENUM!{enum CoalesceEventsOptions {
    CoalesceEventsOptions_Disabled = 0,
    CoalesceEventsOptions_Enabled = 0x1,
}}
STRUCT!{struct ExtendedProperty {
    PropertyName: BSTR,
    PropertyValue: BSTR,
}}
pub type UIA_HWND = *mut c_void;
pub const UIA_InvokePatternId: c_long = 10000;
pub const UIA_SelectionPatternId: c_long = 10001;
pub const UIA_ValuePatternId: c_long = 10002;
pub const UIA_RangeValuePatternId: c_long = 10003;
pub const UIA_ScrollPatternId: c_long = 10004;
pub const UIA_ExpandCollapsePatternId: c_long = 10005;
pub const UIA_GridPatternId: c_long = 10006;
pub const UIA_GridItemPatternId: c_long = 10007;
pub const UIA_MultipleViewPatternId: c_long = 10008;
pub const UIA_WindowPatternId: c_long = 10009;
pub const UIA_SelectionItemPatternId: c_long = 10010;
pub const UIA_DockPatternId: c_long = 10011;
pub const UIA_TablePatternId: c_long = 10012;
pub const UIA_TableItemPatternId: c_long = 10013;
pub const UIA_TextPatternId: c_long = 10014;
pub const UIA_TogglePatternId: c_long = 10015;
pub const UIA_TransformPatternId: c_long = 10016;
pub const UIA_ScrollItemPatternId: c_long = 10017;
pub const UIA_LegacyIAccessiblePatternId: c_long = 10018;
pub const UIA_ItemContainerPatternId: c_long = 10019;
pub const UIA_VirtualizedItemPatternId: c_long = 10020;
pub const UIA_SynchronizedInputPatternId: c_long = 10021;
pub const UIA_ObjectModelPatternId: c_long = 10022;
pub const UIA_AnnotationPatternId: c_long = 10023;
pub const UIA_TextPattern2Id: c_long = 10024;
pub const UIA_StylesPatternId: c_long = 10025;
pub const UIA_SpreadsheetPatternId: c_long = 10026;
pub const UIA_SpreadsheetItemPatternId: c_long = 10027;
pub const UIA_TransformPattern2Id: c_long = 10028;
pub const UIA_TextChildPatternId: c_long = 10029;
pub const UIA_DragPatternId: c_long = 10030;
pub const UIA_DropTargetPatternId: c_long = 10031;
pub const UIA_TextEditPatternId: c_long = 10032;
pub const UIA_CustomNavigationPatternId: c_long = 10033;
pub const UIA_SelectionPattern2Id: c_long = 10034;
pub const UIA_ToolTipOpenedEventId: c_long = 20000;
pub const UIA_ToolTipClosedEventId: c_long = 20001;
pub const UIA_StructureChangedEventId: c_long = 20002;
pub const UIA_MenuOpenedEventId: c_long = 20003;
pub const UIA_AutomationPropertyChangedEventId: c_long = 20004;
pub const UIA_AutomationFocusChangedEventId: c_long = 20005;
pub const UIA_AsyncContentLoadedEventId: c_long = 20006;
pub const UIA_MenuClosedEventId: c_long = 20007;
pub const UIA_LayoutInvalidatedEventId: c_long = 20008;
pub const UIA_Invoke_InvokedEventId: c_long = 20009;
pub const UIA_SelectionItem_ElementAddedToSelectionEventId: c_long = 20010;
pub const UIA_SelectionItem_ElementRemovedFromSelectionEventId: c_long = 20011;
pub const UIA_SelectionItem_ElementSelectedEventId: c_long = 20012;
pub const UIA_Selection_InvalidatedEventId: c_long = 20013;
pub const UIA_Text_TextSelectionChangedEventId: c_long = 20014;
pub const UIA_Text_TextChangedEventId: c_long = 20015;
pub const UIA_Window_WindowOpenedEventId: c_long = 20016;
pub const UIA_Window_WindowClosedEventId: c_long = 20017;
pub const UIA_MenuModeStartEventId: c_long = 20018;
pub const UIA_MenuModeEndEventId: c_long = 20019;
pub const UIA_InputReachedTargetEventId: c_long = 20020;
pub const UIA_InputReachedOtherElementEventId: c_long = 20021;
pub const UIA_InputDiscardedEventId: c_long = 20022;
pub const UIA_SystemAlertEventId: c_long = 20023;
pub const UIA_LiveRegionChangedEventId: c_long = 20024;
pub const UIA_HostedFragmentRootsInvalidatedEventId: c_long = 20025;
pub const UIA_Drag_DragStartEventId: c_long = 20026;
pub const UIA_Drag_DragCancelEventId: c_long = 20027;
pub const UIA_Drag_DragCompleteEventId: c_long = 20028;
pub const UIA_DropTarget_DragEnterEventId: c_long = 20029;
pub const UIA_DropTarget_DragLeaveEventId: c_long = 20030;
pub const UIA_DropTarget_DroppedEventId: c_long = 20031;
pub const UIA_TextEdit_TextChangedEventId: c_long = 20032;
pub const UIA_TextEdit_ConversionTargetChangedEventId: c_long = 20033;
pub const UIA_ChangesEventId: c_long = 20034;
pub const UIA_NotificationEventId: c_long = 20035;
pub const UIA_ActiveTextPositionChangedEventId: c_long = 20036;
pub const UIA_RuntimeIdPropertyId: c_long = 30000;
pub const UIA_BoundingRectanglePropertyId: c_long = 30001;
pub const UIA_ProcessIdPropertyId: c_long = 30002;
pub const UIA_ControlTypePropertyId: c_long = 30003;
pub const UIA_LocalizedControlTypePropertyId: c_long = 30004;
pub const UIA_NamePropertyId: c_long = 30005;
pub const UIA_AcceleratorKeyPropertyId: c_long = 30006;
pub const UIA_AccessKeyPropertyId: c_long = 30007;
pub const UIA_HasKeyboardFocusPropertyId: c_long = 30008;
pub const UIA_IsKeyboardFocusablePropertyId: c_long = 30009;
pub const UIA_IsEnabledPropertyId: c_long = 30010;
pub const UIA_AutomationIdPropertyId: c_long = 30011;
pub const UIA_ClassNamePropertyId: c_long = 30012;
pub const UIA_HelpTextPropertyId: c_long = 30013;
pub const UIA_ClickablePointPropertyId: c_long = 30014;
pub const UIA_CulturePropertyId: c_long = 30015;
pub const UIA_IsControlElementPropertyId: c_long = 30016;
pub const UIA_IsContentElementPropertyId: c_long = 30017;
pub const UIA_LabeledByPropertyId: c_long = 30018;
pub const UIA_IsPasswordPropertyId: c_long = 30019;
pub const UIA_NativeWindowHandlePropertyId: c_long = 30020;
pub const UIA_ItemTypePropertyId: c_long = 30021;
pub const UIA_IsOffscreenPropertyId: c_long = 30022;
pub const UIA_OrientationPropertyId: c_long = 30023;
pub const UIA_FrameworkIdPropertyId: c_long = 30024;
pub const UIA_IsRequiredForFormPropertyId: c_long = 30025;
pub const UIA_ItemStatusPropertyId: c_long = 30026;
pub const UIA_IsDockPatternAvailablePropertyId: c_long = 30027;
pub const UIA_IsExpandCollapsePatternAvailablePropertyId: c_long = 30028;
pub const UIA_IsGridItemPatternAvailablePropertyId: c_long = 30029;
pub const UIA_IsGridPatternAvailablePropertyId: c_long = 30030;
pub const UIA_IsInvokePatternAvailablePropertyId: c_long = 30031;
pub const UIA_IsMultipleViewPatternAvailablePropertyId: c_long = 30032;
pub const UIA_IsRangeValuePatternAvailablePropertyId: c_long = 30033;
pub const UIA_IsScrollPatternAvailablePropertyId: c_long = 30034;
pub const UIA_IsScrollItemPatternAvailablePropertyId: c_long = 30035;
pub const UIA_IsSelectionItemPatternAvailablePropertyId: c_long = 30036;
pub const UIA_IsSelectionPatternAvailablePropertyId: c_long = 30037;
pub const UIA_IsTablePatternAvailablePropertyId: c_long = 30038;
pub const UIA_IsTableItemPatternAvailablePropertyId: c_long = 30039;
pub const UIA_IsTextPatternAvailablePropertyId: c_long = 30040;
pub const UIA_IsTogglePatternAvailablePropertyId: c_long = 30041;
pub const UIA_IsTransformPatternAvailablePropertyId: c_long = 30042;
pub const UIA_IsValuePatternAvailablePropertyId: c_long = 30043;
pub const UIA_IsWindowPatternAvailablePropertyId: c_long = 30044;
pub const UIA_ValueValuePropertyId: c_long = 30045;
pub const UIA_ValueIsReadOnlyPropertyId: c_long = 30046;
pub const UIA_RangeValueValuePropertyId: c_long = 30047;
pub const UIA_RangeValueIsReadOnlyPropertyId: c_long = 30048;
pub const UIA_RangeValueMinimumPropertyId: c_long = 30049;
pub const UIA_RangeValueMaximumPropertyId: c_long = 30050;
pub const UIA_RangeValueLargeChangePropertyId: c_long = 30051;
pub const UIA_RangeValueSmallChangePropertyId: c_long = 30052;
pub const UIA_ScrollHorizontalScrollPercentPropertyId: c_long = 30053;
pub const UIA_ScrollHorizontalViewSizePropertyId: c_long = 30054;
pub const UIA_ScrollVerticalScrollPercentPropertyId: c_long = 30055;
pub const UIA_ScrollVerticalViewSizePropertyId: c_long = 30056;
pub const UIA_ScrollHorizontallyScrollablePropertyId: c_long = 30057;
pub const UIA_ScrollVerticallyScrollablePropertyId: c_long = 30058;
pub const UIA_SelectionSelectionPropertyId: c_long = 30059;
pub const UIA_SelectionCanSelectMultiplePropertyId: c_long = 30060;
pub const UIA_SelectionIsSelectionRequiredPropertyId: c_long = 30061;
pub const UIA_GridRowCountPropertyId: c_long = 30062;
pub const UIA_GridColumnCountPropertyId: c_long = 30063;
pub const UIA_GridItemRowPropertyId: c_long = 30064;
pub const UIA_GridItemColumnPropertyId: c_long = 30065;
pub const UIA_GridItemRowSpanPropertyId: c_long = 30066;
pub const UIA_GridItemColumnSpanPropertyId: c_long = 30067;
pub const UIA_GridItemContainingGridPropertyId: c_long = 30068;
pub const UIA_DockDockPositionPropertyId: c_long = 30069;
pub const UIA_ExpandCollapseExpandCollapseStatePropertyId: c_long = 30070;
pub const UIA_MultipleViewCurrentViewPropertyId: c_long = 30071;
pub const UIA_MultipleViewSupportedViewsPropertyId: c_long = 30072;
pub const UIA_WindowCanMaximizePropertyId: c_long = 30073;
pub const UIA_WindowCanMinimizePropertyId: c_long = 30074;
pub const UIA_WindowWindowVisualStatePropertyId: c_long = 30075;
pub const UIA_WindowWindowInteractionStatePropertyId: c_long = 30076;
pub const UIA_WindowIsModalPropertyId: c_long = 30077;
pub const UIA_WindowIsTopmostPropertyId: c_long = 30078;
pub const UIA_SelectionItemIsSelectedPropertyId: c_long = 30079;
pub const UIA_SelectionItemSelectionContainerPropertyId: c_long = 30080;
pub const UIA_TableRowHeadersPropertyId: c_long = 30081;
pub const UIA_TableColumnHeadersPropertyId: c_long = 30082;
pub const UIA_TableRowOrColumnMajorPropertyId: c_long = 30083;
pub const UIA_TableItemRowHeaderItemsPropertyId: c_long = 30084;
pub const UIA_TableItemColumnHeaderItemsPropertyId: c_long = 30085;
pub const UIA_ToggleToggleStatePropertyId: c_long = 30086;
pub const UIA_TransformCanMovePropertyId: c_long = 30087;
pub const UIA_TransformCanResizePropertyId: c_long = 30088;
pub const UIA_TransformCanRotatePropertyId: c_long = 30089;
pub const UIA_IsLegacyIAccessiblePatternAvailablePropertyId: c_long = 30090;
pub const UIA_LegacyIAccessibleChildIdPropertyId: c_long = 30091;
pub const UIA_LegacyIAccessibleNamePropertyId: c_long = 30092;
pub const UIA_LegacyIAccessibleValuePropertyId: c_long = 30093;
pub const UIA_LegacyIAccessibleDescriptionPropertyId: c_long = 30094;
pub const UIA_LegacyIAccessibleRolePropertyId: c_long = 30095;
pub const UIA_LegacyIAccessibleStatePropertyId: c_long = 30096;
pub const UIA_LegacyIAccessibleHelpPropertyId: c_long = 30097;
pub const UIA_LegacyIAccessibleKeyboardShortcutPropertyId: c_long = 30098;
pub const UIA_LegacyIAccessibleSelectionPropertyId: c_long = 30099;
pub const UIA_LegacyIAccessibleDefaultActionPropertyId: c_long = 30100;
pub const UIA_AriaRolePropertyId: c_long = 30101;
pub const UIA_AriaPropertiesPropertyId: c_long = 30102;
pub const UIA_IsDataValidForFormPropertyId: c_long = 30103;
pub const UIA_ControllerForPropertyId: c_long = 30104;
pub const UIA_DescribedByPropertyId: c_long = 30105;
pub const UIA_FlowsToPropertyId: c_long = 30106;
pub const UIA_ProviderDescriptionPropertyId: c_long = 30107;
pub const UIA_IsItemContainerPatternAvailablePropertyId: c_long = 30108;
pub const UIA_IsVirtualizedItemPatternAvailablePropertyId: c_long = 30109;
pub const UIA_IsSynchronizedInputPatternAvailablePropertyId: c_long = 30110;
pub const UIA_OptimizeForVisualContentPropertyId: c_long = 30111;
pub const UIA_IsObjectModelPatternAvailablePropertyId: c_long = 30112;
pub const UIA_AnnotationAnnotationTypeIdPropertyId: c_long = 30113;
pub const UIA_AnnotationAnnotationTypeNamePropertyId: c_long = 30114;
pub const UIA_AnnotationAuthorPropertyId: c_long = 30115;
pub const UIA_AnnotationDateTimePropertyId: c_long = 30116;
pub const UIA_AnnotationTargetPropertyId: c_long = 30117;
pub const UIA_IsAnnotationPatternAvailablePropertyId: c_long = 30118;
pub const UIA_IsTextPattern2AvailablePropertyId: c_long = 30119;
pub const UIA_StylesStyleIdPropertyId: c_long = 30120;
pub const UIA_StylesStyleNamePropertyId: c_long = 30121;
pub const UIA_StylesFillColorPropertyId: c_long = 30122;
pub const UIA_StylesFillPatternStylePropertyId: c_long = 30123;
pub const UIA_StylesShapePropertyId: c_long = 30124;
pub const UIA_StylesFillPatternColorPropertyId: c_long = 30125;
pub const UIA_StylesExtendedPropertiesPropertyId: c_long = 30126;
pub const UIA_IsStylesPatternAvailablePropertyId: c_long = 30127;
pub const UIA_IsSpreadsheetPatternAvailablePropertyId: c_long = 30128;
pub const UIA_SpreadsheetItemFormulaPropertyId: c_long = 30129;
pub const UIA_SpreadsheetItemAnnotationObjectsPropertyId: c_long = 30130;
pub const UIA_SpreadsheetItemAnnotationTypesPropertyId: c_long = 30131;
pub const UIA_IsSpreadsheetItemPatternAvailablePropertyId: c_long = 30132;
pub const UIA_Transform2CanZoomPropertyId: c_long = 30133;
pub const UIA_IsTransformPattern2AvailablePropertyId: c_long = 30134;
pub const UIA_LiveSettingPropertyId: c_long = 30135;
pub const UIA_IsTextChildPatternAvailablePropertyId: c_long = 30136;
pub const UIA_IsDragPatternAvailablePropertyId: c_long = 30137;
pub const UIA_DragIsGrabbedPropertyId: c_long = 30138;
pub const UIA_DragDropEffectPropertyId: c_long = 30139;
pub const UIA_DragDropEffectsPropertyId: c_long = 30140;
pub const UIA_IsDropTargetPatternAvailablePropertyId: c_long = 30141;
pub const UIA_DropTargetDropTargetEffectPropertyId: c_long = 30142;
pub const UIA_DropTargetDropTargetEffectsPropertyId: c_long = 30143;
pub const UIA_DragGrabbedItemsPropertyId: c_long = 30144;
pub const UIA_Transform2ZoomLevelPropertyId: c_long = 30145;
pub const UIA_Transform2ZoomMinimumPropertyId: c_long = 30146;
pub const UIA_Transform2ZoomMaximumPropertyId: c_long = 30147;
pub const UIA_FlowsFromPropertyId: c_long = 30148;
pub const UIA_IsTextEditPatternAvailablePropertyId: c_long = 30149;
pub const UIA_IsPeripheralPropertyId: c_long = 30150;
pub const UIA_IsCustomNavigationPatternAvailablePropertyId: c_long = 30151;
pub const UIA_PositionInSetPropertyId: c_long = 30152;
pub const UIA_SizeOfSetPropertyId: c_long = 30153;
pub const UIA_LevelPropertyId: c_long = 30154;
pub const UIA_AnnotationTypesPropertyId: c_long = 30155;
pub const UIA_AnnotationObjectsPropertyId: c_long = 30156;
pub const UIA_LandmarkTypePropertyId: c_long = 30157;
pub const UIA_LocalizedLandmarkTypePropertyId: c_long = 30158;
pub const UIA_FullDescriptionPropertyId: c_long = 30159;
pub const UIA_FillColorPropertyId: c_long = 30160;
pub const UIA_OutlineColorPropertyId: c_long = 30161;
pub const UIA_FillTypePropertyId: c_long = 30162;
pub const UIA_VisualEffectsPropertyId: c_long = 30163;
pub const UIA_OutlineThicknessPropertyId: c_long = 30164;
pub const UIA_CenterPointPropertyId: c_long = 30165;
pub const UIA_RotationPropertyId: c_long = 30166;
pub const UIA_SizePropertyId: c_long = 30167;
pub const UIA_IsSelectionPattern2AvailablePropertyId: c_long = 30168;
pub const UIA_Selection2FirstSelectedItemPropertyId: c_long = 30169;
pub const UIA_Selection2LastSelectedItemPropertyId: c_long = 30170;
pub const UIA_Selection2CurrentSelectedItemPropertyId: c_long = 30171;
pub const UIA_Selection2ItemCountPropertyId: c_long = 30172;
pub const UIA_HeadingLevelPropertyId: c_long = 30173;
pub const UIA_IsDialogPropertyId: c_long = 30174;
pub const UIA_AnimationStyleAttributeId: c_long = 40000;
pub const UIA_BackgroundColorAttributeId: c_long = 40001;
pub const UIA_BulletStyleAttributeId: c_long = 40002;
pub const UIA_CapStyleAttributeId: c_long = 40003;
pub const UIA_CultureAttributeId: c_long = 40004;
pub const UIA_FontNameAttributeId: c_long = 40005;
pub const UIA_FontSizeAttributeId: c_long = 40006;
pub const UIA_FontWeightAttributeId: c_long = 40007;
pub const UIA_ForegroundColorAttributeId: c_long = 40008;
pub const UIA_HorizontalTextAlignmentAttributeId: c_long = 40009;
pub const UIA_IndentationFirstLineAttributeId: c_long = 40010;
pub const UIA_IndentationLeadingAttributeId: c_long = 40011;
pub const UIA_IndentationTrailingAttributeId: c_long = 40012;
pub const UIA_IsHiddenAttributeId: c_long = 40013;
pub const UIA_IsItalicAttributeId: c_long = 40014;
pub const UIA_IsReadOnlyAttributeId: c_long = 40015;
pub const UIA_IsSubscriptAttributeId: c_long = 40016;
pub const UIA_IsSuperscriptAttributeId: c_long = 40017;
pub const UIA_MarginBottomAttributeId: c_long = 40018;
pub const UIA_MarginLeadingAttributeId: c_long = 40019;
pub const UIA_MarginTopAttributeId: c_long = 40020;
pub const UIA_MarginTrailingAttributeId: c_long = 40021;
pub const UIA_OutlineStylesAttributeId: c_long = 40022;
pub const UIA_OverlineColorAttributeId: c_long = 40023;
pub const UIA_OverlineStyleAttributeId: c_long = 40024;
pub const UIA_StrikethroughColorAttributeId: c_long = 40025;
pub const UIA_StrikethroughStyleAttributeId: c_long = 40026;
pub const UIA_TabsAttributeId: c_long = 40027;
pub const UIA_TextFlowDirectionsAttributeId: c_long = 40028;
pub const UIA_UnderlineColorAttributeId: c_long = 40029;
pub const UIA_UnderlineStyleAttributeId: c_long = 40030;
pub const UIA_AnnotationTypesAttributeId: c_long = 40031;
pub const UIA_AnnotationObjectsAttributeId: c_long = 40032;
pub const UIA_StyleNameAttributeId: c_long = 40033;
pub const UIA_StyleIdAttributeId: c_long = 40034;
pub const UIA_LinkAttributeId: c_long = 40035;
pub const UIA_IsActiveAttributeId: c_long = 40036;
pub const UIA_SelectionActiveEndAttributeId: c_long = 40037;
pub const UIA_CaretPositionAttributeId: c_long = 40038;
pub const UIA_CaretBidiModeAttributeId: c_long = 40039;
pub const UIA_LineSpacingAttributeId: c_long = 40040;
pub const UIA_BeforeParagraphSpacingAttributeId: c_long = 40041;
pub const UIA_AfterParagraphSpacingAttributeId: c_long = 40042;
pub const UIA_SayAsInterpretAsAttributeId: c_long = 40043;
pub const UIA_ButtonControlTypeId: c_long = 50000;
pub const UIA_CalendarControlTypeId: c_long = 50001;
pub const UIA_CheckBoxControlTypeId: c_long = 50002;
pub const UIA_ComboBoxControlTypeId: c_long = 50003;
pub const UIA_EditControlTypeId: c_long = 50004;
pub const UIA_HyperlinkControlTypeId: c_long = 50005;
pub const UIA_ImageControlTypeId: c_long = 50006;
pub const UIA_ListItemControlTypeId: c_long = 50007;
pub const UIA_ListControlTypeId: c_long = 50008;
pub const UIA_MenuControlTypeId: c_long = 50009;
pub const UIA_MenuBarControlTypeId: c_long = 50010;
pub const UIA_MenuItemControlTypeId: c_long = 50011;
pub const UIA_ProgressBarControlTypeId: c_long = 50012;
pub const UIA_RadioButtonControlTypeId: c_long = 50013;
pub const UIA_ScrollBarControlTypeId: c_long = 50014;
pub const UIA_SliderControlTypeId: c_long = 50015;
pub const UIA_SpinnerControlTypeId: c_long = 50016;
pub const UIA_StatusBarControlTypeId: c_long = 50017;
pub const UIA_TabControlTypeId: c_long = 50018;
pub const UIA_TabItemControlTypeId: c_long = 50019;
pub const UIA_TextControlTypeId: c_long = 50020;
pub const UIA_ToolBarControlTypeId: c_long = 50021;
pub const UIA_ToolTipControlTypeId: c_long = 50022;
pub const UIA_TreeControlTypeId: c_long = 50023;
pub const UIA_TreeItemControlTypeId: c_long = 50024;
pub const UIA_CustomControlTypeId: c_long = 50025;
pub const UIA_GroupControlTypeId: c_long = 50026;
pub const UIA_ThumbControlTypeId: c_long = 50027;
pub const UIA_DataGridControlTypeId: c_long = 50028;
pub const UIA_DataItemControlTypeId: c_long = 50029;
pub const UIA_DocumentControlTypeId: c_long = 50030;
pub const UIA_SplitButtonControlTypeId: c_long = 50031;
pub const UIA_WindowControlTypeId: c_long = 50032;
pub const UIA_PaneControlTypeId: c_long = 50033;
pub const UIA_HeaderControlTypeId: c_long = 50034;
pub const UIA_HeaderItemControlTypeId: c_long = 50035;
pub const UIA_TableControlTypeId: c_long = 50036;
pub const UIA_TitleBarControlTypeId: c_long = 50037;
pub const UIA_SeparatorControlTypeId: c_long = 50038;
pub const UIA_SemanticZoomControlTypeId: c_long = 50039;
pub const UIA_AppBarControlTypeId: c_long = 50040;
pub const AnnotationType_Unknown: c_long = 60000;
pub const AnnotationType_SpellingError: c_long = 60001;
pub const AnnotationType_GrammarError: c_long = 60002;
pub const AnnotationType_Comment: c_long = 60003;
pub const AnnotationType_FormulaError: c_long = 60004;
pub const AnnotationType_TrackChanges: c_long = 60005;
pub const AnnotationType_Header: c_long = 60006;
pub const AnnotationType_Footer: c_long = 60007;
pub const AnnotationType_Highlighted: c_long = 60008;
pub const AnnotationType_Endnote: c_long = 60009;
pub const AnnotationType_Footnote: c_long = 60010;
pub const AnnotationType_InsertionChange: c_long = 60011;
pub const AnnotationType_DeletionChange: c_long = 60012;
pub const AnnotationType_MoveChange: c_long = 60013;
pub const AnnotationType_FormatChange: c_long = 60014;
pub const AnnotationType_UnsyncedChange: c_long = 60015;
pub const AnnotationType_EditingLockedChange: c_long = 60016;
pub const AnnotationType_ExternalChange: c_long = 60017;
pub const AnnotationType_ConflictingChange: c_long = 60018;
pub const AnnotationType_Author: c_long = 60019;
pub const AnnotationType_AdvancedProofingIssue: c_long = 60020;
pub const AnnotationType_DataValidationError: c_long = 60021;
pub const AnnotationType_CircularReferenceError: c_long = 60022;
pub const AnnotationType_Mathematics: c_long = 60023;
pub const AnnotationType_Sensitive: c_long = 60024;
pub const StyleId_Custom: c_long = 70000;
pub const StyleId_Heading1: c_long = 70001;
pub const StyleId_Heading2: c_long = 70002;
pub const StyleId_Heading3: c_long = 70003;
pub const StyleId_Heading4: c_long = 70004;
pub const StyleId_Heading5: c_long = 70005;
pub const StyleId_Heading6: c_long = 70006;
pub const StyleId_Heading7: c_long = 70007;
pub const StyleId_Heading8: c_long = 70008;
pub const StyleId_Heading9: c_long = 70009;
pub const StyleId_Title: c_long = 70010;
pub const StyleId_Subtitle: c_long = 70011;
pub const StyleId_Normal: c_long = 70012;
pub const StyleId_Emphasis: c_long = 70013;
pub const StyleId_Quote: c_long = 70014;
pub const StyleId_BulletedList: c_long = 70015;
pub const StyleId_NumberedList: c_long = 70016;
pub const UIA_CustomLandmarkTypeId: c_long = 80000;
pub const UIA_FormLandmarkTypeId: c_long = 80001;
pub const UIA_MainLandmarkTypeId: c_long = 80002;
pub const UIA_NavigationLandmarkTypeId: c_long = 80003;
pub const UIA_SearchLandmarkTypeId: c_long = 80004;
pub const HeadingLevel_None: c_long = 80050;
pub const HeadingLevel1: c_long = 80051;
pub const HeadingLevel2: c_long = 80052;
pub const HeadingLevel3: c_long = 80053;
pub const HeadingLevel4: c_long = 80054;
pub const HeadingLevel5: c_long = 80055;
pub const HeadingLevel6: c_long = 80056;
pub const HeadingLevel7: c_long = 80057;
pub const HeadingLevel8: c_long = 80058;
pub const HeadingLevel9: c_long = 80059;
pub const UIA_SummaryChangeId: c_long = 90000;
pub const UIA_SayAsInterpretAsMetadataId: c_long = 100000;
RIDL!{#[uuid(0xd22108aa, 0x8ac5, 0x49a5, 0x83, 0x7b, 0x37, 0xbb, 0xb3, 0xd7, 0x59, 0x1e)]
interface IUIAutomationElement(IUIAutomationElementVtbl): IUnknown(IUnknownVtbl) {
    fn SetFocus() -> HRESULT,
    fn GetRuntimeId(
        runtimeId: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    fn FindFirst(
        scope: TreeScope,
        condition: *mut IUIAutomationCondition,
        found: *mut *mut IUIAutomationElement,
    ) -> HRESULT,
    fn FindAll(
        scope: TreeScope,
        condition: *mut IUIAutomationCondition,
        found: *mut *mut IUIAutomationElementArray,
    ) -> HRESULT,
    fn FindFirstBuildCache(
        scope: TreeScope,
        condition: *mut IUIAutomationCondition,
        cacheRequest: *mut IUIAutomationCacheRequest,
        found: *mut *mut IUIAutomationElement,
    ) -> HRESULT,
    fn FindAllBuildCache(
        scope: TreeScope,
        condition: *mut IUIAutomationCondition,
        cacheRequest: *mut IUIAutomationCacheRequest,
        found: *mut *mut IUIAutomationElementArray,
    ) -> HRESULT,
    fn BuildUpdatedCache(
        cacheRequest: *mut IUIAutomationCacheRequest,
        updatedElement: *mut *mut IUIAutomationElement,
    ) -> HRESULT,
    fn GetCurrentPropertyValue(
        propertyId: PROPERTYID,
        retVal: *mut VARIANT,
    ) -> HRESULT,
    fn GetCurrentPropertyValueEx(
        propertyId: PROPERTYID,
        ignoreDefaultValue: BOOL,
        retVal: *mut VARIANT,
    ) -> HRESULT,
    fn GetCachedPropertyValue(
        propertyId: PROPERTYID,
        retVal: *mut VARIANT,
    ) -> HRESULT,
    fn GetCachedPropertyValueEx(
        propertyId: PROPERTYID,
        ignoreDefaultValue: BOOL,
        retVal: *mut VARIANT,
    ) -> HRESULT,
    fn GetCurrentPatternAs(
        patternId: PATTERNID,
        riid: REFIID,
        patternObject: *mut *mut c_void,
    ) -> HRESULT,
    fn GetCachedPatternAs(
        patternId: PATTERNID,
        riid: REFIID,
        patternObject: *mut *mut c_void,
    ) -> HRESULT,
    fn GetCurrentPattern(
        patternId: PATTERNID,
        patternObject: *mut *mut IUnknown,
    ) -> HRESULT,
    fn GetCachedPattern(
        patternId: PATTERNID,
        patternObject: *mut *mut IUnknown,
    ) -> HRESULT,
    fn GetCachedParent(
        parent: *mut *mut IUIAutomationElement,
    ) -> HRESULT,
    fn GetCachedChildren(
        children: *mut *mut IUIAutomationElementArray,
    ) -> HRESULT,
    fn get_CurrentProcessId(
        retVal: *mut c_int,
    ) -> HRESULT,
    fn get_CurrentControlType(
        retVal: *mut CONTROLTYPEID,
    ) -> HRESULT,
    fn get_CurrentLocalizedControlType(
        retVal: *mut BSTR,
    ) -> HRESULT,
    fn get_CurrentName(
        retVal: *mut BSTR,
    ) -> HRESULT,
    fn get_CurrentAcceleratorKey(
        retVal: *mut BSTR,
    ) -> HRESULT,
    fn get_CurrentAccessKey(
        retVal: *mut BSTR,
    ) -> HRESULT,
    fn get_CurrentHasKeyboardFocus(
        retVal: *mut BOOL,
    ) -> HRESULT,
    fn get_CurrentIsKeyboardFocusable(
        retVal: *mut BOOL,
    ) -> HRESULT,
    fn get_CurrentIsEnabled(
        retVal: *mut BOOL,
    ) -> HRESULT,
    fn get_CurrentAutomationId(
        retVal: *mut BSTR,
    ) -> HRESULT,
    fn get_CurrentClassName(
        retVal: *mut BSTR,
    ) -> HRESULT,
    fn get_CurrentHelpText(
        retVal: *mut BSTR,
    ) -> HRESULT,
    fn get_CurrentCulture(
        retVal: *mut c_int,
    ) -> HRESULT,
    fn get_CurrentIsControlElement(
        retVal: *mut BOOL,
    ) -> HRESULT,
    fn get_CurrentIsContentElement(
        retVal: *mut BOOL,
    ) -> HRESULT,
    fn get_CurrentIsPassword(
        retVal: *mut BOOL,
    ) -> HRESULT,
    fn get_CurrentNativeWindowHandle(
        retVal: *mut UIA_HWND,
    ) -> HRESULT,
    fn get_CurrentItemType(
        retVal: *mut BSTR,
    ) -> HRESULT,
    fn get_CurrentIsOffscreen(
        retVal: *mut BOOL,
    ) -> HRESULT,
    fn get_CurrentOrientation(
        retVal: *mut OrientationType,
    ) -> HRESULT,
    fn get_CurrentFrameworkId(
        retVal: *mut BSTR,
    ) -> HRESULT,
    fn get_CurrentIsRequiredForForm(
        retVal: *mut BOOL,
    ) -> HRESULT,
    fn get_CurrentItemStatus(
        retVal: *mut BSTR,
    ) -> HRESULT,
    fn get_CurrentBoundingRectangle(
        retVal: *mut RECT,
    ) -> HRESULT,
    fn get_CurrentLabeledBy(
        retVal: *mut *mut IUIAutomationElement,
    ) -> HRESULT,
    fn get_CurrentAriaRole(
        retVal: *mut BSTR,
    ) -> HRESULT,
    fn get_CurrentAriaProperties(
        retVal: *mut BSTR,
    ) -> HRESULT,
    fn get_CurrentIsDataValidForForm(
        retVal: *mut BOOL,
    ) -> HRESULT,
    fn get_CurrentControllerFor(
        retVal: *mut *mut IUIAutomationElementArray,
    ) -> HRESULT,
    fn get_CurrentDescribedBy(
        retVal: *mut *mut IUIAutomationElementArray,
    ) -> HRESULT,
    fn get_CurrentFlowsTo(
        retVal: *mut *mut IUIAutomationElementArray,
    ) -> HRESULT,
    fn get_CurrentProviderDescription(
        retVal: *mut BSTR,
    ) -> HRESULT,
    fn get_CachedProcessId(
        retVal: *mut c_int,
    ) -> HRESULT,
    fn get_CachedControlType(
        retVal: *mut CONTROLTYPEID,
    ) -> HRESULT,
    fn get_CachedLocalizedControlType(
        retVal: *mut BSTR,
    ) -> HRESULT,
    fn get_CachedName(
        retVal: *mut BSTR,
    ) -> HRESULT,
    fn get_CachedAcceleratorKey(
        retVal: *mut BSTR,
    ) -> HRESULT,
    fn get_CachedAccessKey(
        retVal: *mut BSTR,
    ) -> HRESULT,
    fn get_CachedHasKeyboardFocus(
        retVal: *mut BOOL,
    ) -> HRESULT,
    fn get_CachedIsKeyboardFocusable(
        retVal: *mut BOOL,
    ) -> HRESULT,
    fn get_CachedIsEnabled(
        retVal: *mut BOOL,
    ) -> HRESULT,
    fn get_CachedAutomationId(
        retVal: *mut BSTR,
    ) -> HRESULT,
    fn get_CachedClassName(
        retVal: *mut BSTR,
    ) -> HRESULT,
    fn get_CachedHelpText(
        retVal: *mut BSTR,
    ) -> HRESULT,
    fn get_CachedCulture(
        retVal: *mut c_int,
    ) -> HRESULT,
    fn get_CachedIsControlElement(
        retVal: *mut BOOL,
    ) -> HRESULT,
    fn get_CachedIsContentElement(
        retVal: *mut BOOL,
    ) -> HRESULT,
    fn get_CachedIsPassword(
        retVal: *mut BOOL,
    ) -> HRESULT,
    fn get_CachedNativeWindowHandle(
        retVal: *mut UIA_HWND,
    ) -> HRESULT,
    fn get_CachedItemType(
        retVal: *mut BSTR,
    ) -> HRESULT,
    fn get_CachedIsOffscreen(
        retVal: *mut BOOL,
    ) -> HRESULT,
    fn get_CachedOrientation(
        retVal: *mut OrientationType,
    ) -> HRESULT,
    fn get_CachedFrameworkId(
        retVal: *mut BSTR,
    ) -> HRESULT,
    fn get_CachedIsRequiredForForm(
        retVal: *mut BOOL,
    ) -> HRESULT,
    fn get_CachedItemStatus(
        retVal: *mut BSTR,
    ) -> HRESULT,
    fn get_CachedBoundingRectangle(
        retVal: *mut RECT,
    ) -> HRESULT,
    fn get_CachedLabeledBy(
        retVal: *mut *mut IUIAutomationElement,
    ) -> HRESULT,
    fn get_CachedAriaRole(
        retVal: *mut BSTR,
    ) -> HRESULT,
    fn get_CachedAriaProperties(
        retVal: *mut BSTR,
    ) -> HRESULT,
    fn get_CachedIsDataValidForForm(
        retVal: *mut BOOL,
    ) -> HRESULT,
    fn get_CachedControllerFor(
        retVal: *mut *mut IUIAutomationElementArray,
    ) -> HRESULT,
    fn get_CachedDescribedBy(
        retVal: *mut *mut IUIAutomationElementArray,
    ) -> HRESULT,
    fn get_CachedFlowsTo(
        retVal: *mut *mut IUIAutomationElementArray,
    ) -> HRESULT,
    fn get_CachedProviderDescription(
        retVal: *mut BSTR,
    ) -> HRESULT,
    fn GetClickablePoint(
        clickable: *mut POINT,
        gotClickable: *mut BOOL,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x14314595, 0xb4bc, 0x4055, 0x95, 0xf2, 0x58, 0xf2, 0xe4, 0x2c, 0x98, 0x55)]
interface IUIAutomationElementArray(IUIAutomationElementArrayVtbl): IUnknown(IUnknownVtbl) {
    fn get_Length(
        length: *mut c_int,
    ) -> HRESULT,
    fn GetElement(
        index: c_int,
        element: *mut *mut IUIAutomationElement,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x352ffba8, 0x0973, 0x437c, 0xa6, 0x1f, 0xf6, 0x4c, 0xaf, 0xd8, 0x1d, 0xf9)]
interface IUIAutomationCondition(IUIAutomationConditionVtbl): IUnknown(IUnknownVtbl) {}}
RIDL!{#[uuid(0x1b4e1f2e, 0x75eb, 0x4d0b, 0x89, 0x52, 0x5a, 0x69, 0x98, 0x8e, 0x23, 0x07)]
interface IUIAutomationBoolCondition(IUIAutomationBoolConditionVtbl):
        IUIAutomationCondition(IUIAutomationConditionVtbl) {
    fn get_BooleanValue(
        boolVal: *mut BOOL,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x99ebf2cb, 0x5578, 0x4267, 0x9a, 0xd4, 0xaf, 0xd6, 0xea, 0x77, 0xe9, 0x4b)]
interface IUIAutomationPropertyCondition(IUIAutomationPropertyConditionVtbl):
        IUIAutomationCondition(IUIAutomationConditionVtbl) {
    fn get_PropertyId(
        propertyId: *mut PROPERTYID,
    ) -> HRESULT,
    fn get_PropertyValue(
        propertyValue: *mut VARIANT,
    ) -> HRESULT,
    fn get_PropertyConditionFlags(
        flags: *mut PropertyConditionFlags,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xa7d0af36, 0xb912, 0x45fe, 0x98, 0x55, 0x09, 0x1d, 0xdc, 0x17, 0x4a, 0xec)]
interface IUIAutomationAndCondition(IUIAutomationAndConditionVtbl):
        IUIAutomationCondition(IUIAutomationConditionVtbl) {
    fn get_ChildCount(
        childCount: *mut c_int,
    ) -> HRESULT,
    fn GetChildrenAsNativeArray(
        childArray: *mut *mut *mut IUIAutomationCondition,
        childArrayCount: *mut c_int,
    ) -> HRESULT,
    fn GetChildren(
        childArray: *mut *mut SAFEARRAY,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x8753f032, 0x3db1, 0x47b5, 0xa1, 0xfc, 0x6e, 0x34, 0xa2, 0x66, 0xc7, 0x12)]
interface IUIAutomationOrCondition(IUIAutomationOrConditionVtbl):
        IUIAutomationCondition(IUIAutomationConditionVtbl) {
    fn get_ChildCount(
        childCount: *mut c_int,
    ) -> HRESULT,
    fn GetChildrenAsNativeArray(
        childArray: *mut *mut *mut IUIAutomationCondition,
        childArrayCount: *mut c_int,
    ) -> HRESULT,
    fn GetChildren(
        childArray: *mut *mut SAFEARRAY,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xf528b657, 0x847b, 0x498c, 0x88, 0x96, 0xd5, 0x2b, 0x56, 0x54, 0x07, 0xa1)]
interface IUIAutomationNotCondition(IUIAutomationNotConditionVtbl):
        IUIAutomationCondition(IUIAutomationConditionVtbl) {
    fn GetChild(
        condition: *mut *mut IUIAutomationCondition,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xb32a92b5, 0xbc25, 0x4078, 0x9c, 0x08, 0xd7, 0xee, 0x95, 0xc4, 0x8e, 0x03)]
interface IUIAutomationCacheRequest(IUIAutomationCacheRequestVtbl): IUnknown(IUnknownVtbl) {
    fn AddProperty(
        propertyId: PROPERTYID,
    ) -> HRESULT,
    fn AddPattern(
        patternId: PATTERNID,
    ) -> HRESULT,
    fn Clone(
        clonedRequest: *mut *mut IUIAutomationCacheRequest,
    ) -> HRESULT,
    fn get_TreeScope(
        scope: *mut TreeScope,
    ) -> HRESULT,
    fn put_TreeScope(
        scope: TreeScope,
    ) -> HRESULT,
    fn get_TreeFilter(
        filter: *mut *mut IUIAutomationCondition,
    ) -> HRESULT,
    fn put_TreeFilter(
        filter: *mut IUIAutomationCondition,
    ) -> HRESULT,
    fn get_AutomationElementMode(
        mode: *mut AutomationElementMode,
    ) -> HRESULT,
    fn put_AutomationElementMode(
        mode: AutomationElementMode,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x4042c624, 0x389c, 0x4afc, 0xa6, 0x30, 0x9d, 0xf8, 0x54, 0xa5, 0x41, 0xfc)]
interface IUIAutomationTreeWalker(IUIAutomationTreeWalkerVtbl): IUnknown(IUnknownVtbl) {
    fn GetParentElement(
        element: *mut IUIAutomationElement,
        parent: *mut *mut IUIAutomationElement,
    ) -> HRESULT,
    fn GetFirstChildElement(
        element: *mut IUIAutomationElement,
        first: *mut *mut IUIAutomationElement,
    ) -> HRESULT,
    fn GetLastChildElement(
        element: *mut IUIAutomationElement,
        last: *mut *mut IUIAutomationElement,
    ) -> HRESULT,
    fn GetNextSiblingElement(
        element: *mut IUIAutomationElement,
        next: *mut *mut IUIAutomationElement,
    ) -> HRESULT,
    fn GetPreviousSiblingElement(
        element: *mut IUIAutomationElement,
        previous: *mut *mut IUIAutomationElement,
    ) -> HRESULT,
    fn NormalizeElement(
        element: *mut IUIAutomationElement,
        normalized: *mut *mut IUIAutomationElement,
    ) -> HRESULT,
    fn GetParentElementBuildCache(
        element: *mut IUIAutomationElement,
        cacheRequest: *mut IUIAutomationCacheRequest,
        parent: *mut *mut IUIAutomationElement,
    ) -> HRESULT,
    fn GetFirstChildElementBuildCache(
        element: *mut IUIAutomationElement,
        cacheRequest: *mut IUIAutomationCacheRequest,
        first: *mut *mut IUIAutomationElement,
    ) -> HRESULT,
    fn GetLastChildElementBuildCache(
        element: *mut IUIAutomationElement,
        cacheRequest: *mut IUIAutomationCacheRequest,
        last: *mut *mut IUIAutomationElement,
    ) -> HRESULT,
    fn GetNextSiblingElementBuildCache(
        element: *mut IUIAutomationElement,
        cacheRequest: *mut IUIAutomationCacheRequest,
        next: *mut *mut IUIAutomationElement,
    ) -> HRESULT,
    fn GetPreviousSiblingElementBuildCache(
        element: *mut IUIAutomationElement,
        cacheRequest: *mut IUIAutomationCacheRequest,
        previous: *mut *mut IUIAutomationElement,
    ) -> HRESULT,
    fn NormalizeElementBuildCache(
        element: *mut IUIAutomationElement,
        cacheRequest: *mut IUIAutomationCacheRequest,
        normalized: *mut *mut IUIAutomationElement,
    ) -> HRESULT,
    fn get_Condition(
        condition: *mut *mut IUIAutomationCondition,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x146c3c17, 0xf12e, 0x4e22, 0x8c, 0x27, 0xf8, 0x94, 0xb9, 0xb7, 0x9c, 0x69)]
interface IUIAutomationEventHandler(IUIAutomationEventHandlerVtbl): IUnknown(IUnknownVtbl) {
    fn HandleAutomationEvent(
        sender: *mut IUIAutomationElement,
        eventId: EVENTID,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x40cd37d4, 0xc756, 0x4b0c, 0x8c, 0x6f, 0xbd, 0xdf, 0xee, 0xb1, 0x3b, 0x50)]
interface IUIAutomationPropertyChangedEventHandler(IUIAutomationPropertyChangedEventHandlerVtbl):
        IUnknown(IUnknownVtbl) {
    fn HandlePropertyChangedEvent(
        sender: *mut IUIAutomationElement,
        propertyId: PROPERTYID,
        newValue: VARIANT,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xe81d1b4e, 0x11c5, 0x42f8, 0x97, 0x54, 0xe7, 0x03, 0x6c, 0x79, 0xf0, 0x54)]
interface IUIAutomationStructureChangedEventHandler(IUIAutomationStructureChangedEventHandlerVtbl):
        IUnknown(IUnknownVtbl) {
    fn HandleStructureChangedEvent(
        sender: *mut IUIAutomationElement,
        changeType: StructureChangeType,
        runtimeId: *mut SAFEARRAY,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xc270f6b5, 0x5c69, 0x4290, 0x97, 0x45, 0x7a, 0x7f, 0x97, 0x16, 0x94, 0x68)]
interface IUIAutomationFocusChangedEventHandler(IUIAutomationFocusChangedEventHandlerVtbl):
        IUnknown(IUnknownVtbl) {
    fn HandleFocusChangedEvent(
        sender: *mut IUIAutomationElement,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x92FAA680, 0xE704, 0x4156, 0x93, 0x1A, 0xE3, 0x2D, 0x5B, 0xB3, 0x8F, 0x3F)]
interface IUIAutomationTextEditTextChangedEventHandler
        (IUIAutomationTextEditTextChangedEventHandlerVtbl): IUnknown(IUnknownVtbl) {
    fn HandleTextEditTextChangedEvent(
        sender: *mut IUIAutomationElement,
        textEditChangeType: TextEditChangeType,
        eventStrings: *mut SAFEARRAY,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x58EDCA55, 0x2C3E, 0x4980, 0xB1, 0xB9, 0x56, 0xC1, 0x7F, 0x27, 0xA2, 0xA0)]
interface IUIAutomationChangesEventHandler(IUIAutomationChangesEventHandlerVtbl):
        IUnknown(IUnknownVtbl) {
    fn HandleChangesEvent(
        sender: *mut IUIAutomationElement,
        uiaChanges: *mut UiaChangeInfo,
        changesCount: c_int,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xC7CB2637, 0xE6C2, 0x4D0C, 0x85, 0xDE, 0x49, 0x48, 0xC0, 0x21, 0x75, 0xC7)]
interface IUIAutomationNotificationEventHandler(IUIAutomationNotificationEventHandlerVtbl):
        IUnknown(IUnknownVtbl) {
    fn HandleNotificationEvent(
        sender: *mut IUIAutomationElement,
        notificationKind: NotificationKind,
        notificationProcessing: NotificationProcessing,
        displayString: BSTR,
        activityId: BSTR,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xfb377fbe, 0x8ea6, 0x46d5, 0x9c, 0x73, 0x64, 0x99, 0x64, 0x2d, 0x30, 0x59)]
interface IUIAutomationInvokePattern(IUIAutomationInvokePatternVtbl): IUnknown(IUnknownVtbl) {
    fn Invoke() -> HRESULT,
}}
RIDL!{#[uuid(0xfde5ef97, 0x1464, 0x48f6, 0x90, 0xbf, 0x43, 0xd0, 0x94, 0x8e, 0x86, 0xec)]
interface IUIAutomationDockPattern(IUIAutomationDockPatternVtbl): IUnknown(IUnknownVtbl) {
    fn SetDockPosition(
        dockPos: DockPosition,
    ) -> HRESULT,
    fn get_CurrentDockPosition(
        retVal: *mut DockPosition,
    ) -> HRESULT,
    fn get_CachedDockPosition(
        retVal: *mut DockPosition,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x619be086, 0x1f4e, 0x4ee4, 0xba, 0xfa, 0x21, 0x01, 0x28, 0x73, 0x87, 0x30)]
interface IUIAutomationExpandCollapsePattern(IUIAutomationExpandCollapsePatternVtbl):
        IUnknown(IUnknownVtbl) {
    fn Expand() -> HRESULT,
    fn Collapse() -> HRESULT,
    fn get_CurrentExpandCollapseState(
        retVal: *mut ExpandCollapseState,
    ) -> HRESULT,
    fn get_CachedExpandCollapseState(
        retVal: *mut ExpandCollapseState,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x414c3cdc, 0x856b, 0x4f5b, 0x85, 0x38, 0x31, 0x31, 0xc6, 0x30, 0x25, 0x50)]
interface IUIAutomationGridPattern(IUIAutomationGridPatternVtbl): IUnknown(IUnknownVtbl) {
    fn GetItem(
        row: c_int,
        column: c_int,
        element: *mut *mut IUIAutomationElement,
    ) -> HRESULT,
    fn get_CurrentRowCount(
        retVal: *mut c_int,
    ) -> HRESULT,
    fn get_CurrentColumnCount(
        retVal: *mut c_int,
    ) -> HRESULT,
    fn get_CachedRowCount(
        retVal: *mut c_int,
    ) -> HRESULT,
    fn get_CachedColumnCount(
        retVal: *mut c_int,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x78f8ef57, 0x66c3, 0x4e09, 0xbd, 0x7c, 0xe7, 0x9b, 0x20, 0x04, 0x89, 0x4d)]
interface IUIAutomationGridItemPattern(IUIAutomationGridItemPatternVtbl): IUnknown(IUnknownVtbl) {
    fn get_CurrentContainingGrid(
        retVal: *mut *mut IUIAutomationElement,
    ) -> HRESULT,
    fn get_CurrentRow(
        retVal: *mut c_int,
    ) -> HRESULT,
    fn get_CurrentColumn(
        retVal: *mut c_int,
    ) -> HRESULT,
    fn get_CurrentRowSpan(
        retVal: *mut c_int,
    ) -> HRESULT,
    fn get_CurrentColumnSpan(
        retVal: *mut c_int,
    ) -> HRESULT,
    fn get_CachedContainingGrid(
        retVal: *mut *mut IUIAutomationElement,
    ) -> HRESULT,
    fn get_CachedRow(
        retVal: *mut c_int,
    ) -> HRESULT,
    fn get_CachedColumn(
        retVal: *mut c_int,
    ) -> HRESULT,
    fn get_CachedRowSpan(
        retVal: *mut c_int,
    ) -> HRESULT,
    fn get_CachedColumnSpan(
        retVal: *mut c_int,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x8d253c91, 0x1dc5, 0x4bb5, 0xb1, 0x8f, 0xad, 0xe1, 0x6f, 0xa4, 0x95, 0xe8)]
interface IUIAutomationMultipleViewPattern(IUIAutomationMultipleViewPatternVtbl):
        IUnknown(IUnknownVtbl) {
    fn GetViewName(
        view: c_int,
        name: *mut BSTR,
    ) -> HRESULT,
    fn SetCurrentView(
        view: c_int,
    ) -> HRESULT,
    fn get_CurrentCurrentView(
        retVal: *mut c_int,
    ) -> HRESULT,
    fn GetCurrentSupportedViews(
        retVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    fn get_CachedCurrentView(
        retVal: *mut c_int,
    ) -> HRESULT,
    fn GetCachedSupportedViews(
        retVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x71c284b3, 0xc14d, 0x4d14, 0x98, 0x1e, 0x19, 0x75, 0x1b, 0x0d, 0x75, 0x6d)]
interface IUIAutomationObjectModelPattern(IUIAutomationObjectModelPatternVtbl):
        IUnknown(IUnknownVtbl) {
    fn GetUnderlyingObjectModel(
        retVal: *mut *mut IUnknown,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x59213f4f, 0x7346, 0x49e5, 0xb1, 0x20, 0x80, 0x55, 0x59, 0x87, 0xa1, 0x48)]
interface IUIAutomationRangeValuePattern(IUIAutomationRangeValuePatternVtbl):
        IUnknown(IUnknownVtbl) {
    fn SetValue(
        val: c_double,
    ) -> HRESULT,
    fn get_CurrentValue(
        retVal: *mut c_double,
    ) -> HRESULT,
    fn get_CurrentIsReadOnly(
        retVal: *mut BOOL,
    ) -> HRESULT,
    fn get_CurrentMaximum(
        retVal: *mut c_double,
    ) -> HRESULT,
    fn get_CurrentMinimum(
        retVal: *mut c_double,
    ) -> HRESULT,
    fn get_CurrentLargeChange(
        retVal: *mut c_double,
    ) -> HRESULT,
    fn get_CurrentSmallChange(
        retVal: *mut c_double,
    ) -> HRESULT,
    fn get_CachedValue(
        retVal: *mut c_double,
    ) -> HRESULT,
    fn get_CachedIsReadOnly(
        retVal: *mut BOOL,
    ) -> HRESULT,
    fn get_CachedMaximum(
        retVal: *mut c_double,
    ) -> HRESULT,
    fn get_CachedMinimum(
        retVal: *mut c_double,
    ) -> HRESULT,
    fn get_CachedLargeChange(
        retVal: *mut c_double,
    ) -> HRESULT,
    fn get_CachedSmallChange(
        retVal: *mut c_double,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x88f4d42a, 0xe881, 0x459d, 0xa7, 0x7c, 0x73, 0xbb, 0xbb, 0x7e, 0x02, 0xdc)]
interface IUIAutomationScrollPattern(IUIAutomationScrollPatternVtbl): IUnknown(IUnknownVtbl) {
    fn Scroll(
        horizontalAmount: ScrollAmount,
        verticalAmount: ScrollAmount,
    ) -> HRESULT,
    fn SetScrollPercent(
        horizontalPercent: c_double,
        verticalPercent: c_double,
    ) -> HRESULT,
    fn get_CurrentHorizontalScrollPercent(
        retVal: *mut c_double,
    ) -> HRESULT,
    fn get_CurrentVerticalScrollPercent(
        retVal: *mut c_double,
    ) -> HRESULT,
    fn get_CurrentHorizontalViewSize(
        retVal: *mut c_double,
    ) -> HRESULT,
    fn get_CurrentVerticalViewSize(
        retVal: *mut c_double,
    ) -> HRESULT,
    fn get_CurrentHorizontallyScrollable(
        retVal: *mut BOOL,
    ) -> HRESULT,
    fn get_CurrentVerticallyScrollable(
        retVal: *mut BOOL,
    ) -> HRESULT,
    fn get_CachedHorizontalScrollPercent(
        retVal: *mut c_double,
    ) -> HRESULT,
    fn get_CachedVerticalScrollPercent(
        retVal: *mut c_double,
    ) -> HRESULT,
    fn get_CachedHorizontalViewSize(
        retVal: *mut c_double,
    ) -> HRESULT,
    fn get_CachedVerticalViewSize(
        retVal: *mut c_double,
    ) -> HRESULT,
    fn get_CachedHorizontallyScrollable(
        retVal: *mut BOOL,
    ) -> HRESULT,
    fn get_CachedVerticallyScrollable(
        retVal: *mut BOOL,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xb488300f, 0xd015, 0x4f19, 0x9c, 0x29, 0xbb, 0x59, 0x5e, 0x36, 0x45, 0xef)]
interface IUIAutomationScrollItemPattern(IUIAutomationScrollItemPatternVtbl):
        IUnknown(IUnknownVtbl) {
    fn ScrollIntoView() -> HRESULT,
}}
RIDL!{#[uuid(0x5ed5202e, 0xb2ac, 0x47a6, 0xb6, 0x38, 0x4b, 0x0b, 0xf1, 0x40, 0xd7, 0x8e)]
interface IUIAutomationSelectionPattern(IUIAutomationSelectionPatternVtbl):
        IUnknown(IUnknownVtbl) {
    fn GetCurrentSelection(
        retVal: *mut *mut IUIAutomationElementArray,
    ) -> HRESULT,
    fn get_CurrentCanSelectMultiple(
        retVal: *mut BOOL,
    ) -> HRESULT,
    fn get_CurrentIsSelectionRequired(
        retVal: *mut BOOL,
    ) -> HRESULT,
    fn GetCachedSelection(
        retVal: *mut *mut IUIAutomationElementArray,
    ) -> HRESULT,
    fn get_CachedCanSelectMultiple(
        retVal: *mut BOOL,
    ) -> HRESULT,
    fn get_CachedIsSelectionRequired(
        retVal: *mut BOOL,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x0532bfae, 0xc011, 0x4e32, 0xa3, 0x43, 0x6d, 0x64, 0x2d, 0x79, 0x85, 0x55)]
interface IUIAutomationSelectionPattern2(IUIAutomationSelectionPattern2Vtbl):
        IUIAutomationSelectionPattern(IUIAutomationSelectionPatternVtbl) {
    fn get_CurrentFirstSelectedItem(
        retVal: *mut *mut IUIAutomationElement,
    ) -> HRESULT,
    fn get_CurrentLastSelectedItem(
        retVal: *mut *mut IUIAutomationElement,
    ) -> HRESULT,
    fn get_CurrentCurrentSelectedItem(
        retVal: *mut *mut IUIAutomationElement,
    ) -> HRESULT,
    fn get_CurrentItemCount(
        retVal: *mut c_int,
    ) -> HRESULT,
    fn get_CachedFirstSelectedItem(
        retVal: *mut *mut IUIAutomationElement,
    ) -> HRESULT,
    fn get_CachedLastSelectedItem(
        retVal: *mut *mut IUIAutomationElement,
    ) -> HRESULT,
    fn get_CachedCurrentSelectedItem(
        retVal: *mut *mut IUIAutomationElement,
    ) -> HRESULT,
    fn get_CachedItemCount(
        retVal: *mut c_int,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xa8efa66a, 0x0fda, 0x421a, 0x91, 0x94, 0x38, 0x02, 0x1f, 0x35, 0x78, 0xea)]
interface IUIAutomationSelectionItemPattern(IUIAutomationSelectionItemPatternVtbl):
        IUnknown(IUnknownVtbl) {
    fn Select() -> HRESULT,
    fn AddToSelection() -> HRESULT,
    fn RemoveFromSelection() -> HRESULT,
    fn get_CurrentIsSelected(
        retVal: *mut BOOL,
    ) -> HRESULT,
    fn get_CurrentSelectionContainer(
        retVal: *mut *mut IUIAutomationElement,
    ) -> HRESULT,
    fn get_CachedIsSelected(
        retVal: *mut BOOL,
    ) -> HRESULT,
    fn get_CachedSelectionContainer(
        retVal: *mut *mut IUIAutomationElement,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x2233be0b, 0xafb7, 0x448b, 0x9f, 0xda, 0x3b, 0x37, 0x8a, 0xa5, 0xea, 0xe1)]
interface IUIAutomationSynchronizedInputPattern(IUIAutomationSynchronizedInputPatternVtbl):
        IUnknown(IUnknownVtbl) {
    fn StartListening(
        inputType: SynchronizedInputType,
    ) -> HRESULT,
    fn Cancel() -> HRESULT,
}}
RIDL!{#[uuid(0x620e691c, 0xea96, 0x4710, 0xa8, 0x50, 0x75, 0x4b, 0x24, 0xce, 0x24, 0x17)]
interface IUIAutomationTablePattern(IUIAutomationTablePatternVtbl): IUnknown(IUnknownVtbl) {
    fn GetCurrentRowHeaders(
        retVal: *mut *mut IUIAutomationElementArray,
    ) -> HRESULT,
    fn GetCurrentColumnHeaders(
        retVal: *mut *mut IUIAutomationElementArray,
    ) -> HRESULT,
    fn get_CurrentRowOrColumnMajor(
        retVal: *mut RowOrColumnMajor,
    ) -> HRESULT,
    fn GetCachedRowHeaders(
        retVal: *mut *mut IUIAutomationElementArray,
    ) -> HRESULT,
    fn GetCachedColumnHeaders(
        retVal: *mut *mut IUIAutomationElementArray,
    ) -> HRESULT,
    fn get_CachedRowOrColumnMajor(
        retVal: *mut RowOrColumnMajor,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x0b964eb3, 0xef2e, 0x4464, 0x9c, 0x79, 0x61, 0xd6, 0x17, 0x37, 0xa2, 0x7e)]
interface IUIAutomationTableItemPattern(IUIAutomationTableItemPatternVtbl):
        IUnknown(IUnknownVtbl) {
    fn GetCurrentRowHeaderItems(
        retVal: *mut *mut IUIAutomationElementArray,
    ) -> HRESULT,
    fn GetCurrentColumnHeaderItems(
        retVal: *mut *mut IUIAutomationElementArray,
    ) -> HRESULT,
    fn GetCachedRowHeaderItems(
        retVal: *mut *mut IUIAutomationElementArray,
    ) -> HRESULT,
    fn GetCachedColumnHeaderItems(
        retVal: *mut *mut IUIAutomationElementArray,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x94cf8058, 0x9b8d, 0x4ab9, 0x8b, 0xfd, 0x4c, 0xd0, 0xa3, 0x3c, 0x8c, 0x70)]
interface IUIAutomationTogglePattern(IUIAutomationTogglePatternVtbl): IUnknown(IUnknownVtbl) {
    fn Toggle() -> HRESULT,
    fn get_CurrentToggleState(
        retVal: *mut ToggleState,
    ) -> HRESULT,
    fn get_CachedToggleState(
        retVal: *mut ToggleState,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xa9b55844, 0xa55d, 0x4ef0, 0x92, 0x6d, 0x56, 0x9c, 0x16, 0xff, 0x89, 0xbb)]
interface IUIAutomationTransformPattern(IUIAutomationTransformPatternVtbl):
        IUnknown(IUnknownVtbl) {
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
    fn get_CurrentCanMove(
        retVal: *mut BOOL,
    ) -> HRESULT,
    fn get_CurrentCanResize(
        retVal: *mut BOOL,
    ) -> HRESULT,
    fn get_CurrentCanRotate(
        retVal: *mut BOOL,
    ) -> HRESULT,
    fn get_CachedCanMove(
        retVal: *mut BOOL,
    ) -> HRESULT,
    fn get_CachedCanResize(
        retVal: *mut BOOL,
    ) -> HRESULT,
    fn get_CachedCanRotate(
        retVal: *mut BOOL,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xa94cd8b1, 0x0844, 0x4cd6, 0x9d, 0x2d, 0x64, 0x05, 0x37, 0xab, 0x39, 0xe9)]
interface IUIAutomationValuePattern(IUIAutomationValuePatternVtbl): IUnknown(IUnknownVtbl) {
    fn SetValue(
        val: BSTR,
    ) -> HRESULT,
    fn get_CurrentValue(
        retVal: *mut BSTR,
    ) -> HRESULT,
    fn get_CurrentIsReadOnly(
        retVal: *mut BOOL,
    ) -> HRESULT,
    fn get_CachedValue(
        retVal: *mut BSTR,
    ) -> HRESULT,
    fn get_CachedIsReadOnly(
        retVal: *mut BOOL,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x0faef453, 0x9208, 0x43ef, 0xbb, 0xb2, 0x3b, 0x48, 0x51, 0x77, 0x86, 0x4f)]
interface IUIAutomationWindowPattern(IUIAutomationWindowPatternVtbl): IUnknown(IUnknownVtbl) {
    fn Close() -> HRESULT,
    fn WaitForInputIdle(
        milliseconds: c_int,
        success: *mut BOOL,
    ) -> HRESULT,
    fn SetWindowVisualState(
        state: WindowVisualState,
    ) -> HRESULT,
    fn get_CurrentCanMaximize(
        retVal: *mut BOOL,
    ) -> HRESULT,
    fn get_CurrentCanMinimize(
        retVal: *mut BOOL,
    ) -> HRESULT,
    fn get_CurrentIsModal(
        retVal: *mut BOOL,
    ) -> HRESULT,
    fn get_CurrentIsTopmost(
        retVal: *mut BOOL,
    ) -> HRESULT,
    fn get_CurrentWindowVisualState(
        retVal: *mut WindowVisualState,
    ) -> HRESULT,
    fn get_CurrentWindowInteractionState(
        retVal: *mut WindowInteractionState,
    ) -> HRESULT,
    fn get_CachedCanMaximize(
        retVal: *mut BOOL,
    ) -> HRESULT,
    fn get_CachedCanMinimize(
        retVal: *mut BOOL,
    ) -> HRESULT,
    fn get_CachedIsModal(
        retVal: *mut BOOL,
    ) -> HRESULT,
    fn get_CachedIsTopmost(
        retVal: *mut BOOL,
    ) -> HRESULT,
    fn get_CachedWindowVisualState(
        retVal: *mut WindowVisualState,
    ) -> HRESULT,
    fn get_CachedWindowInteractionState(
        retVal: *mut WindowInteractionState,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xa543cc6a, 0xf4ae, 0x494b, 0x82, 0x39, 0xc8, 0x14, 0x48, 0x11, 0x87, 0xa8)]
interface IUIAutomationTextRange(IUIAutomationTextRangeVtbl): IUnknown(IUnknownVtbl) {
    fn Clone(
        clonedRange: *mut *mut IUIAutomationTextRange,
    ) -> HRESULT,
    fn Compare(
        range: *mut IUIAutomationTextRange,
        areSame: *mut BOOL,
    ) -> HRESULT,
    fn CompareEndpoints(
        srcEndPoint: TextPatternRangeEndpoint,
        range: *mut IUIAutomationTextRange,
        targetEndPoint: TextPatternRangeEndpoint,
        compValue: *mut c_int,
    ) -> HRESULT,
    fn ExpandToEnclosingUnit(
        textUnit: TextUnit,
    ) -> HRESULT,
    fn FindAttribute(
        attr: TEXTATTRIBUTEID,
        val: VARIANT,
        backward: BOOL,
        found: *mut *mut IUIAutomationTextRange,
    ) -> HRESULT,
    fn FindText(
        text: BSTR,
        backward: BOOL,
        ignoreCase: BOOL,
        found: *mut *mut IUIAutomationTextRange,
    ) -> HRESULT,
    fn GetAttributeValue(
        attr: TEXTATTRIBUTEID,
        value: *mut VARIANT,
    ) -> HRESULT,
    fn GetBoundingRectangles(
        boundingRects: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    fn GetEnclosingElement(
        enclosingElement: *mut *mut IUIAutomationElement,
    ) -> HRESULT,
    fn GetText(
        maxLength: c_int,
        text: *mut BSTR,
    ) -> HRESULT,
    fn Move(
        unit: TextUnit,
        count: c_int,
        moved: *mut c_int,
    ) -> HRESULT,
    fn MoveEndpointByUnit(
        endpoint: TextPatternRangeEndpoint,
        unit: TextUnit,
        count: c_int,
        moved: *mut c_int,
    ) -> HRESULT,
    fn MoveEndpointByRange(
        srcEndPoint: TextPatternRangeEndpoint,
        range: *mut IUIAutomationTextRange,
        targetEndPoint: TextPatternRangeEndpoint,
    ) -> HRESULT,
    fn Select() -> HRESULT,
    fn AddToSelection() -> HRESULT,
    fn RemoveFromSelection() -> HRESULT,
    fn ScrollIntoView(
        alignToTop: BOOL,
    ) -> HRESULT,
    fn GetChildren(
        children: *mut *mut IUIAutomationElementArray,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xBB9B40E0, 0x5E04, 0x46BD, 0x9B, 0xE0, 0x4B, 0x60, 0x1B, 0x9A, 0xFA, 0xD4)]
interface IUIAutomationTextRange2(IUIAutomationTextRange2Vtbl):
        IUIAutomationTextRange(IUIAutomationTextRangeVtbl) {
    fn ShowContextMenu() -> HRESULT,
}}
RIDL!{#[uuid(0x6A315D69, 0x5512, 0x4C2E, 0x85, 0xF0, 0x53, 0xFC, 0xE6, 0xDD, 0x4B, 0xC2)]
interface IUIAutomationTextRange3(IUIAutomationTextRange3Vtbl):
        IUIAutomationTextRange2(IUIAutomationTextRange2Vtbl) {
    fn GetEnclosingElementBuildCache(
        cacheRequest: *mut IUIAutomationCacheRequest,
        enclosingElement: *mut *mut IUIAutomationElement,
    ) -> HRESULT,
    fn GetChildrenBuildCache(
        cacheRequest: *mut IUIAutomationCacheRequest,
        children: *mut *mut IUIAutomationElementArray,
    ) -> HRESULT,
    fn GetAttributeValues(
// bookmark
        attributeIds: *const TEXTATTRIBUTEID,
        attributeIdCount: c_int,
        attributeValues: *mut *mut SAFEARRAY,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xce4ae76a, 0xe717, 0x4c98, 0x81, 0xea, 0x47, 0x37, 0x1d, 0x02, 0x8e, 0xb6)]
interface IUIAutomationTextRangeArray(IUIAutomationTextRangeArrayVtbl): IUnknown(IUnknownVtbl) {
    fn get_Length(
        length: c_int,
    ) -> HRESULT,
    fn GetElement(
        index: c_int,
        element: *mut *mut IUIAutomationTextRange,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x32eba289, 0x3583, 0x42c9, 0x9c, 0x59, 0x3b, 0x6d, 0x9a, 0x1e, 0x9b, 0x6a)]
interface IUIAutomationTextPattern(IUIAutomationTextPatternVtbl): IUnknown(IUnknownVtbl) {
    fn RangeFromPoint(
        pt: POINT,
        range: *mut *mut IUIAutomationTextRange,
    ) -> HRESULT,
    fn RangeFromChild(
        child: *mut IUIAutomationElement,
        range: *mut *mut IUIAutomationTextRange,
    ) -> HRESULT,
    fn GetSelection(
        ranges: *mut *mut IUIAutomationTextRangeArray,
    ) -> HRESULT,
    fn GetVisibleRanges(
        ranges: *mut *mut IUIAutomationTextRangeArray,
    ) -> HRESULT,
    fn get_DocumentRange(
        range: *mut *mut IUIAutomationTextRange,
    ) -> HRESULT,
    fn get_SupportedTextSelection(
        supportedTextSelection: *mut SupportedTextSelection,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x506a921a, 0xfcc9, 0x409f, 0xb2, 0x3b, 0x37, 0xeb, 0x74, 0x10, 0x68, 0x72)]
interface IUIAutomationTextPattern2(IUIAutomationTextPattern2Vtbl):
        IUIAutomationTextPattern(IUIAutomationTextPatternVtbl) {
    fn RangeFromAnnotation(
        annotation: *mut IUIAutomationElement,
        range: *mut *mut IUIAutomationTextRange,
    ) -> HRESULT,
    fn GetCaretRange(
        isActive: *mut BOOL,
        range: *mut *mut IUIAutomationTextRange,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x17E21576, 0x996C, 0x4870, 0x99, 0xD9, 0xBF, 0xF3, 0x23, 0x38, 0x0C, 0x06)]
interface IUIAutomationTextEditPattern(IUIAutomationTextEditPatternVtbl):
        IUIAutomationTextPattern(IUIAutomationTextPatternVtbl) {
    fn GetActiveComposition(
        range: *mut *mut IUIAutomationTextRange,
    ) -> HRESULT,
    fn GetConversionTarget(
        range: *mut *mut IUIAutomationTextRange,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x01EA217A, 0x1766, 0x47ED, 0xA6, 0xCC, 0xAC, 0xF4, 0x92, 0x85, 0x4B, 0x1F)]
interface IUIAutomationCustomNavigationPattern(IUIAutomationCustomNavigationPatternVtbl):
        IUnknown(IUnknownVtbl) {
    fn Navigate(
        direction: NavigateDirection,
        pRetVal: *mut *mut IUIAutomationElement,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xF97933B0, 0x8DAE, 0x4496, 0x89, 0x97, 0x5B, 0xA0, 0x15, 0xFE, 0x0D, 0x82)]
interface IUIAutomationActiveTextPositionChangedEventHandler
        (IUIAutomationActiveTextPositionChangedEventHandlerVtbl): IUnknown(IUnknownVtbl) {
    fn HandleActiveTextPositionChangedEvent(
        sender: *mut IUIAutomationElement,
        range: *mut IUIAutomationTextRange,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x828055ad, 0x355b, 0x4435, 0x86, 0xd5, 0x3b, 0x51, 0xc1, 0x4a, 0x9b, 0x1b)]
interface IUIAutomationLegacyIAccessiblePattern(IUIAutomationLegacyIAccessiblePatternVtbl):
        IUnknown(IUnknownVtbl) {
    fn Select(
        flagsSelect: c_long,
    ) -> HRESULT,
    fn DoDefaultAction() -> HRESULT,
    fn SetValue(
        szValue: LPCWSTR,
    ) -> HRESULT,
    fn get_CurrentChildId(
        pRetVal: *mut c_int,
    ) -> HRESULT,
    fn get_CurrentName(
        pszName: *mut BSTR,
    ) -> HRESULT,
    fn get_CurrentValue(
        pszValue: *mut BSTR,
    ) -> HRESULT,
    fn get_CurrentDescription(
        pszDescription: *mut BSTR,
    ) -> HRESULT,
    fn get_CurrentRole(
        pdwRole: *mut DWORD,
    ) -> HRESULT,
    fn get_CurrentState(
        pdwState: *mut DWORD,
    ) -> HRESULT,
    fn get_CurrentHelp(
        pszHelp: *mut BSTR,
    ) -> HRESULT,
    fn get_CurrentKeyboardShortcut(
        pszKeyboardShortcut: *mut BSTR,
    ) -> HRESULT,
    fn GetCurrentSelection(
        pvarSelectedChildren: *mut *mut IUIAutomationElementArray,
    ) -> HRESULT,
    fn get_CurrentDefaultAction(
        pszDefaultAction: *mut BSTR,
    ) -> HRESULT,
    fn get_CachedChildId(
        pRetVal: *mut c_int,
    ) -> HRESULT,
    fn get_CachedName(
        pszName: *mut BSTR,
    ) -> HRESULT,
    fn get_CachedValue(
        pszValue: *mut BSTR,
    ) -> HRESULT,
    fn get_CachedDescription(
        pszDescription: *mut BSTR,
    ) -> HRESULT,
    fn get_CachedRole(
        pdwRole: *mut DWORD,
    ) -> HRESULT,
    fn get_CachedState(
        pdwState: *mut DWORD,
    ) -> HRESULT,
    fn get_CachedHelp(
        pszHelp: *mut BSTR,
    ) -> HRESULT,
    fn get_CachedKeyboardShortcut(
        pszKeyboardShortcut: *mut BSTR,
    ) -> HRESULT,
    fn GetCachedSelection(
        pvarSelectedChildren: *mut *mut IUIAutomationElementArray,
    ) -> HRESULT,
    fn get_CachedDefaultAction(
        pszDefaultAction: *mut BSTR,
    ) -> HRESULT,
    fn GetIAccessible(
        ppAccessible: *mut *mut IAccessible,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xc690fdb2, 0x27a8, 0x423c, 0x81, 0x2d, 0x42, 0x97, 0x73, 0xc9, 0x08, 0x4e)]
interface IUIAutomationItemContainerPattern(IUIAutomationItemContainerPatternVtbl):
        IUnknown(IUnknownVtbl) {
    fn FindItemByProperty(
        pStartAfter: *mut IUIAutomationElement,
        propertyId: PROPERTYID,
        value: VARIANT,
        pFound: *mut *mut IUIAutomationElement,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x6ba3d7a6, 0x04cf, 0x4f11, 0x87, 0x93, 0xa8, 0xd1, 0xcd, 0xe9, 0x96, 0x9f)]
interface IUIAutomationVirtualizedItemPattern(IUIAutomationVirtualizedItemPatternVtbl):
        IUnknown(IUnknownVtbl) {
    fn Realize() -> HRESULT,
}}
RIDL!{#[uuid(0x9a175b21, 0x339e, 0x41b1, 0x8e, 0x8b, 0x62, 0x3f, 0x6b, 0x68, 0x10, 0x98)]
interface IUIAutomationAnnotationPattern(IUIAutomationAnnotationPatternVtbl):
        IUnknown(IUnknownVtbl) {
    fn get_CurrentAnnotationTypeId(
        retVal: *mut c_int,
    ) -> HRESULT,
    fn get_CurrentAnnotationTypeName(
        retVal: *mut BSTR,
    ) -> HRESULT,
    fn get_CurrentAuthor(
        retVal: *mut BSTR,
    ) -> HRESULT,
    fn get_CurrentDateTime(
        retVal: *mut BSTR,
    ) -> HRESULT,
    fn get_CurrentTarget(
        retVal: *mut *mut IUIAutomationElement,
    ) -> HRESULT,
    fn get_CachedAnnotationTypeId(
        retVal: *mut c_int,
    ) -> HRESULT,
    fn get_CachedAnnotationTypeName(
        retVal: *mut BSTR,
    ) -> HRESULT,
    fn get_CachedAuthor(
        retVal: *mut BSTR,
    ) -> HRESULT,
    fn get_CachedDateTime(
        retVal: *mut BSTR,
    ) -> HRESULT,
    fn get_CachedTarget(
        retVal: *mut *mut IUIAutomationElement,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x85b5f0a2, 0xbd79, 0x484a, 0xad, 0x2b, 0x38, 0x8c, 0x98, 0x38, 0xd5, 0xfb)]
interface IUIAutomationStylesPattern(IUIAutomationStylesPatternVtbl): IUnknown(IUnknownVtbl) {
    fn get_CurrentStyleId(
        retVal: *mut c_int,
    ) -> HRESULT,
    fn get_CurrentStyleName(
        retVal: *mut BSTR,
    ) -> HRESULT,
    fn get_CurrentFillColor(
        retVal: *mut c_int,
    ) -> HRESULT,
    fn get_CurrentFillPatternStyle(
        retVal: *mut BSTR,
    ) -> HRESULT,
    fn get_CurrentShape(
        retVal: *mut BSTR,
    ) -> HRESULT,
    fn get_CurrentFillPatternColor(
        retVal: *mut c_int,
    ) -> HRESULT,
    fn get_CurrentExtendedProperties(
        retVal: *mut BSTR,
    ) -> HRESULT,
    fn GetCurrentExtendedPropertiesAsArray(
        propertyArray: *mut *mut ExtendedProperty,
        propertyCount: *mut c_int,
    ) -> HRESULT,
    fn get_CachedStyleId(
        retVal: *mut c_int,
    ) -> HRESULT,
    fn get_CachedStyleName(
        retVal: *mut BSTR,
    ) -> HRESULT,
    fn get_CachedFillColor(
        retVal: *mut c_int,
    ) -> HRESULT,
    fn get_CachedFillPatternStyle(
        retVal: *mut BSTR,
    ) -> HRESULT,
    fn get_CachedShape(
        retVal: *mut BSTR,
    ) -> HRESULT,
    fn get_CachedFillPatternColor(
        retVal: *mut c_int,
    ) -> HRESULT,
    fn get_CachedExtendedProperties(
        retVal: *mut BSTR,
    ) -> HRESULT,
    fn GetCachedExtendedPropertiesAsArray(
        propertyArray: *mut *mut ExtendedProperty,
        propertyCount: *mut c_int,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x7517a7c8, 0xfaae, 0x4de9, 0x9f, 0x08, 0x29, 0xb9, 0x1e, 0x85, 0x95, 0xc1)]
interface IUIAutomationSpreadsheetPattern(IUIAutomationSpreadsheetPatternVtbl):
        IUnknown(IUnknownVtbl) {
    fn GetItemByName(
        name: BSTR,
        element: *mut *mut IUIAutomationElement,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x7d4fb86c, 0x8d34, 0x40e1, 0x8e, 0x83, 0x62, 0xc1, 0x52, 0x04, 0xe3, 0x35)]
interface IUIAutomationSpreadsheetItemPattern(IUIAutomationSpreadsheetItemPatternVtbl):
        IUnknown(IUnknownVtbl) {
    fn get_CurrentFormula(
        retVal: *mut BSTR,
    ) -> HRESULT,
    fn GetCurrentAnnotationObjects(
        retVal: *mut *mut IUIAutomationElementArray,
    ) -> HRESULT,
    fn GetCurrentAnnotationTypes(
        retVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    fn get_CachedFormula(
        retVal: *mut BSTR,
    ) -> HRESULT,
    fn GetCachedAnnotationObjects(
        retVal: *mut *mut IUIAutomationElementArray,
    ) -> HRESULT,
    fn GetCachedAnnotationTypes(
        retVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x6d74d017, 0x6ecb, 0x4381, 0xb3, 0x8b, 0x3c, 0x17, 0xa4, 0x8f, 0xf1, 0xc2)]
interface IUIAutomationTransformPattern2(IUIAutomationTransformPattern2Vtbl):
        IUIAutomationTransformPattern(IUIAutomationTransformPatternVtbl) {
    fn Zoom(
        zoomValue: c_double,
    ) -> HRESULT,
    fn ZoomByUnit(
        zoomUnit: ZoomUnit,
    ) -> HRESULT,
    fn get_CurrentCanZoom(
        retVal: *mut BOOL,
    ) -> HRESULT,
    fn get_CachedCanZoom(
        retVal: *mut BOOL,
    ) -> HRESULT,
    fn get_CurrentZoomLevel(
        retVal: *mut c_double,
    ) -> HRESULT,
    fn get_CachedZoomLevel(
        retVal: *mut c_double,
    ) -> HRESULT,
    fn get_CurrentZoomMinimum(
        retVal: *mut c_double,
    ) -> HRESULT,
    fn get_CachedZoomMinimum(
        retVal: *mut c_double,
    ) -> HRESULT,
    fn get_CurrentZoomMaximum(
        retVal: *mut c_double,
    ) -> HRESULT,
    fn get_CachedZoomMaximum(
        retVal: *mut c_double,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x6552b038, 0xae05, 0x40c8, 0xab, 0xfd, 0xaa, 0x08, 0x35, 0x2a, 0xab, 0x86)]
interface IUIAutomationTextChildPattern(IUIAutomationTextChildPatternVtbl):
        IUnknown(IUnknownVtbl) {
    fn get_TextContainer(
        container: *mut *mut IUIAutomationElement,
    ) -> HRESULT,
    fn get_TextRange(
        range: *mut *mut IUIAutomationTextRange,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x1dc7b570, 0x1f54, 0x4bad, 0xbc, 0xda, 0xd3, 0x6a, 0x72, 0x2f, 0xb7, 0xbd)]
interface IUIAutomationDragPattern(IUIAutomationDragPatternVtbl): IUnknown(IUnknownVtbl) {
    fn get_CurrentIsGrabbed(
        retVal: *mut BOOL,
    ) -> HRESULT,
    fn get_CachedIsGrabbed(
        retVal: *mut BOOL,
    ) -> HRESULT,
    fn get_CurrentDropEffect(
        retVal: *mut BSTR,
    ) -> HRESULT,
    fn get_CachedDropEffect(
        retVal: *mut BSTR,
    ) -> HRESULT,
    fn get_CurrentDropEffects(
        retVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    fn get_CachedDropEffects(
        retVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    fn GetCurrentGrabbedItems(
        retVal: *mut *mut IUIAutomationElementArray,
    ) -> HRESULT,
    fn GetCachedGrabbedItems(
        retVal: *mut *mut IUIAutomationElementArray,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x69a095f7, 0xeee4, 0x430e, 0xa4, 0x6b, 0xfb, 0x73, 0xb1, 0xae, 0x39, 0xa5)]
interface IUIAutomationDropTargetPattern(IUIAutomationDropTargetPatternVtbl):
        IUnknown(IUnknownVtbl) {
    fn get_CurrentDropTargetEffect(
        retVal: *mut BSTR,
    ) -> HRESULT,
    fn get_CachedDropTargetEffect(
        retVal: *mut BSTR,
    ) -> HRESULT,
    fn get_CurrentDropTargetEffects(
        retVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    fn get_CachedDropTargetEffects(
        retVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x6749c683, 0xf70d, 0x4487, 0xa6, 0x98, 0x5f, 0x79, 0xd5, 0x52, 0x90, 0xd6)]
interface IUIAutomationElement2(IUIAutomationElement2Vtbl):
        IUIAutomationElement(IUIAutomationElementVtbl) {
    fn get_CurrentOptimizeForVisualContent(
        retVal: *mut BOOL,
    ) -> HRESULT,
    fn get_CachedOptimizeForVisualContent(
        retVal: *mut BOOL,
    ) -> HRESULT,
    fn get_CurrentLiveSetting(
        retVal: *mut LiveSetting,
    ) -> HRESULT,
    fn get_CachedLiveSetting(
        retVal: *mut LiveSetting,
    ) -> HRESULT,
    fn get_CurrentFlowsFrom(
        retVal: *mut *mut IUIAutomationElementArray,
    ) -> HRESULT,
    fn get_CachedFlowsFrom(
        retVal: *mut *mut IUIAutomationElementArray,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x8471DF34, 0xAEE0, 0x4A01, 0xA7, 0xDE, 0x7D, 0xB9, 0xAF, 0x12, 0xC2, 0x96)]
interface IUIAutomationElement3(IUIAutomationElement3Vtbl):
        IUIAutomationElement2(IUIAutomationElement2Vtbl) {
    fn ShowContextMenu() -> HRESULT,
    fn get_CurrentIsPeripheral(
        retVal: *mut BOOL,
    ) -> HRESULT,
    fn get_CachedIsPeripheral(
        retVal: *mut BOOL,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x3B6E233C, 0x52FB, 0x4063, 0xA4, 0xC9, 0x77, 0xC0, 0x75, 0xC2, 0xA0, 0x6B)]
interface IUIAutomationElement4(IUIAutomationElement4Vtbl):
        IUIAutomationElement3(IUIAutomationElement3Vtbl) {
    fn get_CurrentPositionInSet(
        retVal: *mut c_int,
    ) -> HRESULT,
    fn get_CurrentSizeOfSet(
        retVal: *mut c_int,
    ) -> HRESULT,
    fn get_CurrentLevel(
        retVal: *mut c_int,
    ) -> HRESULT,
    fn get_CurrentAnnotationTypes(
        retVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    fn get_CurrentAnnotationObjects(
        retVal: *mut *mut IUIAutomationElementArray,
    ) -> HRESULT,
    fn get_CachedPositionInSet(
        retVal: *mut c_int,
    ) -> HRESULT,
    fn get_CachedSizeOfSet(
        retVal: *mut c_int,
    ) -> HRESULT,
    fn get_CachedLevel(
        retVal: *mut c_int,
    ) -> HRESULT,
    fn get_CachedAnnotationTypes(
        retVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    fn get_CachedAnnotationObjects(
        retVal: *mut *mut IUIAutomationElementArray,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x98141C1D, 0x0D0E, 0x4175, 0xBB, 0xE2, 0x6B, 0xFF, 0x45, 0x58, 0x42, 0xA7)]
interface IUIAutomationElement5(IUIAutomationElement5Vtbl):
        IUIAutomationElement4(IUIAutomationElement4Vtbl) {
    fn get_CurrentLandmarkType(
        retVal: *mut LANDMARKTYPEID,
    ) -> HRESULT,
    fn get_CurrentLocalizedLandmarkType(
        retVal: *mut BSTR,
    ) -> HRESULT,
    fn get_CachedLandmarkType(
        retVal: *mut LANDMARKTYPEID,
    ) -> HRESULT,
    fn get_CachedLocalizedLandmarkType(
        retVal: *mut BSTR,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x4780d450, 0x8bca, 0x4977, 0xaf, 0xa5, 0xa4, 0xa5, 0x17, 0xf5, 0x55, 0xe3)]
interface IUIAutomationElement6(IUIAutomationElement6Vtbl):
        IUIAutomationElement5(IUIAutomationElement5Vtbl) {
    fn get_CurrentFullDescription(
        retVal: *mut BSTR,
    ) -> HRESULT,
    fn get_CachedFullDescription(
        retVal: *mut BSTR,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x204e8572, 0xcfc3, 0x4c11, 0xb0, 0xc8, 0x7d, 0xa7, 0x42, 0x07, 0x50, 0xb7)]
interface IUIAutomationElement7(IUIAutomationElement7Vtbl):
        IUIAutomationElement6(IUIAutomationElement6Vtbl) {
    fn FindFirstWithOptions(
        scope: TreeScope,
        condition: *mut IUIAutomationCondition,
        traversalOptions: TreeTraversalOptions,
        root: *mut IUIAutomationElement,
        found: *mut *mut IUIAutomationElement,
    ) -> HRESULT,
    fn FindAllWithOptions(
        scope: TreeScope,
        condition: *mut IUIAutomationCondition,
        traversalOptions: TreeTraversalOptions,
        root: *mut IUIAutomationElement,
        found: *mut *mut IUIAutomationElementArray,
    ) -> HRESULT,
    fn FindFirstWithOptionsBuildCache(
        scope: TreeScope,
        condition: *mut IUIAutomationCondition,
        cacheRequest: *mut IUIAutomationCacheRequest,
        traversalOptions: TreeTraversalOptions,
        root: *mut IUIAutomationElement,
        found: *mut *mut IUIAutomationElement,
    ) -> HRESULT,
    fn FindAllWithOptionsBuildCache(
        scope: TreeScope,
        condition: *mut IUIAutomationCondition,
        cacheRequest: *mut IUIAutomationCacheRequest,
        traversalOptions: TreeTraversalOptions,
        root: *mut IUIAutomationElement,
        found: *mut *mut IUIAutomationElementArray,
    ) -> HRESULT,
    fn GetCurrentMetadataValue(
        targetId: c_int,
        metadataId: METADATAID,
        returnVal: *mut VARIANT,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x8C60217D, 0x5411, 0x4CDE, 0xBC, 0xC0, 0x1C, 0xED, 0xA2, 0x23, 0x83, 0x0C)]
interface IUIAutomationElement8(IUIAutomationElement8Vtbl):
        IUIAutomationElement7(IUIAutomationElement7Vtbl) {
    fn get_CurrentHeadingLevel(
        retVal: *mut HEADINGLEVELID,
    ) -> HRESULT,
    fn get_CachedHeadingLevel(
        retVal: *mut HEADINGLEVELID,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x39325fac, 0x039d, 0x440e, 0xa3, 0xa3, 0x5e, 0xb8, 0x1a, 0x5c, 0xec, 0xc3)]
interface IUIAutomationElement9(IUIAutomationElement9Vtbl):
        IUIAutomationElement8(IUIAutomationElement8Vtbl) {
    fn get_CurrentIsDialog(
        retVal: *mut BOOL,
    ) -> HRESULT,
    fn get_CachedIsDialog(
        retVal: *mut BOOL,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x85b94ecd, 0x849d, 0x42b6, 0xb9, 0x4d, 0xd6, 0xdb, 0x23, 0xfd, 0xf5, 0xa4)]
interface IUIAutomationProxyFactory(IUIAutomationProxyFactoryVtbl): IUnknown(IUnknownVtbl) {
    fn CreateProvider(
        hwnd: UIA_HWND,
        idObject: LONG,
        idChild: LONG,
        provider: *mut *mut IRawElementProviderSimple,
    ) -> HRESULT,
    fn get_ProxyFactoryId(
        factoryId: *mut BSTR,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xd50e472e, 0xb64b, 0x490c, 0xbc, 0xa1, 0xd3, 0x06, 0x96, 0xf9, 0xf2, 0x89)]
interface IUIAutomationProxyFactoryEntry(IUIAutomationProxyFactoryEntryVtbl):
        IUnknown(IUnknownVtbl) {
    fn get_ProxyFactory(
        factory: *mut *mut IUIAutomationProxyFactory,
    ) -> HRESULT,
    fn get_ClassName(
        className: *mut BSTR,
    ) -> HRESULT,
    fn get_ImageName(
        imageName: *mut BSTR,
    ) -> HRESULT,
    fn get_AllowSubstringMatch(
        allowSubstringMatch: *mut BOOL,
    ) -> HRESULT,
    fn get_CanCheckBaseClass(
        canCheckBaseClass: *mut BOOL,
    ) -> HRESULT,
    fn get_NeedsAdviseEvents(
        adviseEvents: *mut BOOL,
    ) -> HRESULT,
    fn put_ClassName(
        className: LPCWSTR,
    ) -> HRESULT,
    fn put_ImageName(
        imageName: LPCWSTR,
    ) -> HRESULT,
    fn put_AllowSubstringMatch(
        allowSubstringMatch: BOOL,
    ) -> HRESULT,
    fn put_CanCheckBaseClass(
        canCheckBaseClass: BOOL,
    ) -> HRESULT,
    fn put_NeedsAdviseEvents(
        adviseEvents: BOOL,
    ) -> HRESULT,
    fn SetWinEventsForAutomationEvent(
        eventId: EVENTID,
        propertyId: PROPERTYID,
        winEvents: *mut SAFEARRAY,
    ) -> HRESULT,
    fn GetWinEventsForAutomationEvent(
        eventId: EVENTID,
        propertyId: PROPERTYID,
        winEvents: *mut *mut SAFEARRAY,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x09e31e18, 0x872d, 0x4873, 0x93, 0xd1, 0x1e, 0x54, 0x1e, 0xc1, 0x33, 0xfd)]
interface IUIAutomationProxyFactoryMapping(IUIAutomationProxyFactoryMappingVtbl):
        IUnknown(IUnknownVtbl) {
    fn get_Count(
        count: *mut UINT,
    ) -> HRESULT,
    fn GetTable(
        table: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    fn GetEntry(
        index: UINT,
        entry: *mut *mut IUIAutomationProxyFactoryEntry,
    ) -> HRESULT,
    fn SetTable(
        factoryList: *mut SAFEARRAY,
    ) -> HRESULT,
    fn InsertEntries(
        before: UINT,
        factoryList: *mut SAFEARRAY,
    ) -> HRESULT,
    fn InsertEntry(
        before: UINT,
        factory: *mut IUIAutomationProxyFactoryEntry,
    ) -> HRESULT,
    fn RemoveEntry(
        index: UINT,
    ) -> HRESULT,
    fn ClearTable() -> HRESULT,
    fn RestoreDefaultTable() -> HRESULT,
}}
RIDL!{#[uuid(0xC9EE12F2, 0xC13B, 0x4408, 0x99, 0x7C, 0x63, 0x99, 0x14, 0x37, 0x7F, 0x4E)]
interface IUIAutomationEventHandlerGroup(IUIAutomationEventHandlerGroupVtbl):
        IUnknown(IUnknownVtbl) {
    fn AddActiveTextPositionChangedEventHandler(
        scope: TreeScope,
        cacheRequest: *mut IUIAutomationCacheRequest,
        handler: *mut IUIAutomationActiveTextPositionChangedEventHandler,
    ) -> HRESULT,
    fn AddAutomationEventHandler(
        eventId: EVENTID,
        scope: TreeScope,
        cacheRequest: *mut IUIAutomationCacheRequest,
        handler: *mut IUIAutomationEventHandler,
    ) -> HRESULT,
    fn AddChangesEventHandler(
        scope: TreeScope,
        changeTypes: *mut c_int,
        changesCount: c_int,
        cacheRequest: *mut IUIAutomationCacheRequest,
        handler: *mut IUIAutomationChangesEventHandler,
    ) -> HRESULT,
    fn AddNotificationEventHandler(
        scope: TreeScope,
        cacheRequest: *mut IUIAutomationCacheRequest,
        handler: *mut IUIAutomationNotificationEventHandler,
    ) -> HRESULT,
    fn AddPropertyChangedEventHandler(
        scope: TreeScope,
        cacheRequest: *mut IUIAutomationCacheRequest,
        handler: *mut IUIAutomationPropertyChangedEventHandler,
        propertyArray: *mut PROPERTYID,
        propertyCount: c_int,
    ) -> HRESULT,
    fn AddStructureChangedEventHandler(
        scope: TreeScope,
        cacheRequest: *mut IUIAutomationCacheRequest,
        handler: *mut IUIAutomationStructureChangedEventHandler,
    ) -> HRESULT,
    fn AddTextEditTextChangedEventHandler(
        scope: TreeScope,
        textEditChangeType: TextEditChangeType,
        cacheRequest: *mut IUIAutomationCacheRequest,
        handler: *mut IUIAutomationTextEditTextChangedEventHandler,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x30cbe57d, 0xd9d0, 0x452a, 0xab, 0x13, 0x7a, 0xc5, 0xac, 0x48, 0x25, 0xee)]
interface IUIAutomation(IUIAutomationVtbl): IUnknown(IUnknownVtbl) {
    fn CompareElements(
        el1: *mut IUIAutomationElement,
        el2: *mut IUIAutomationElement,
        areSame: *mut BOOL,
    ) -> HRESULT,
    fn CompareRuntimeIds(
        runtimeId1: *mut SAFEARRAY,
        runtimeId2: *mut SAFEARRAY,
        areSame: *mut BOOL,
    ) -> HRESULT,
    fn GetRootElement(
        root: *mut *mut IUIAutomationElement,
    ) -> HRESULT,
    fn ElementFromHandle(
        hwnd: UIA_HWND,
        element: *mut *mut IUIAutomationElement,
    ) -> HRESULT,
    fn ElementFromPoint(
        pt: POINT,
        element: *mut *mut IUIAutomationElement,
    ) -> HRESULT,
    fn GetFocusedElement(
        element: *mut *mut IUIAutomationElement,
    ) -> HRESULT,
    fn GetRootElementBuildCache(
        cacheRequest: *mut IUIAutomationCacheRequest,
        root: *mut *mut IUIAutomationElement,
    ) -> HRESULT,
    fn ElementFromHandleBuildCache(
        hwnd: UIA_HWND,
        cacheRequest: *mut IUIAutomationCacheRequest,
        element: *mut *mut IUIAutomationElement,
    ) -> HRESULT,
    fn ElementFromPointBuildCache(
        pt: POINT,
        cacheRequest: *mut IUIAutomationCacheRequest,
        element: *mut *mut IUIAutomationElement,
    ) -> HRESULT,
    fn GetFocusedElementBuildCache(
        cacheRequest: *mut IUIAutomationCacheRequest,
        element: *mut *mut IUIAutomationElement,
    ) -> HRESULT,
    fn CreateTreeWalker(
        pCondition: *mut IUIAutomationCondition,
        walker: *mut *mut IUIAutomationTreeWalker,
    ) -> HRESULT,
    fn get_ControlViewWalker(
        walker: *mut *mut IUIAutomationTreeWalker,
    ) -> HRESULT,
    fn get_ContentViewWalker(
        walker: *mut *mut IUIAutomationTreeWalker,
    ) -> HRESULT,
    fn get_RawViewWalker(
        walker: *mut *mut IUIAutomationTreeWalker,
    ) -> HRESULT,
    fn get_RawViewCondition(
        condition: *mut *mut IUIAutomationCondition,
    ) -> HRESULT,
    fn get_ControlViewCondition(
        condition: *mut *mut IUIAutomationCondition,
    ) -> HRESULT,
    fn get_ContentViewCondition(
        condition: *mut *mut IUIAutomationCondition,
    ) -> HRESULT,
    fn CreateCacheRequest(
        cacheRequest: *mut *mut IUIAutomationCacheRequest,
    ) -> HRESULT,
    fn CreateTrueCondition(
        newCondition: *mut *mut IUIAutomationCondition,
    ) -> HRESULT,
    fn CreateFalseCondition(
        newCondition: *mut *mut IUIAutomationCondition,
    ) -> HRESULT,
    fn CreatePropertyCondition(
        propertyId: PROPERTYID,
        value: VARIANT,
        newCondition: *mut *mut IUIAutomationCondition,
    ) -> HRESULT,
    fn CreatePropertyConditionEx(
        propertyId: PROPERTYID,
        value: VARIANT,
        flags: PropertyConditionFlags,
        newCondition: *mut *mut IUIAutomationCondition,
    ) -> HRESULT,
    fn CreateAndCondition(
        condition1: *mut IUIAutomationCondition,
        condition2: *mut IUIAutomationCondition,
        newCondition: *mut *mut IUIAutomationCondition,
    ) -> HRESULT,
    fn CreateAndConditionFromArray(
        conditions: *mut SAFEARRAY,
        newCondition: *mut *mut IUIAutomationCondition,
    ) -> HRESULT,
    fn CreateAndConditionFromNativeArray(
        conditions: *mut *mut IUIAutomationCondition,
        conditionCount: c_int,
        newCondition: *mut *mut IUIAutomationCondition,
    ) -> HRESULT,
    fn CreateOrCondition(
        condition1: *mut IUIAutomationCondition,
        condition2: *mut IUIAutomationCondition,
        newCondition: *mut *mut IUIAutomationCondition,
    ) -> HRESULT,
    fn CreateOrConditionFromArray(
        conditions: *mut SAFEARRAY,
        newCondition: *mut *mut IUIAutomationCondition,
    ) -> HRESULT,
    fn CreateOrConditionFromNativeArray(
        conditions: *mut *mut IUIAutomationCondition,
        conditionCount: c_int,
        newCondition: *mut *mut IUIAutomationCondition,
    ) -> HRESULT,
    fn CreateNotCondition(
        condition: *mut IUIAutomationCondition,
        newCondition: *mut *mut IUIAutomationCondition,
    ) -> HRESULT,
    fn AddAutomationEventHandler(
        eventId: EVENTID,
        element: *mut IUIAutomationElement,
        scope: TreeScope,
        cacheRequest: *mut IUIAutomationCacheRequest,
        handler: *mut IUIAutomationEventHandler,
    ) -> HRESULT,
    fn RemoveAutomationEventHandler(
        eventId: EVENTID,
        element: *mut IUIAutomationElement,
        handler: *mut IUIAutomationEventHandler,
    ) -> HRESULT,
    fn AddPropertyChangedEventHandlerNativeArray(
        element: *mut IUIAutomationElement,
        scope: TreeScope,
        cacheRequest: *mut IUIAutomationCacheRequest,
        handler: *mut IUIAutomationPropertyChangedEventHandler,
        propertyArray: *mut PROPERTYID,
        propertyCount: c_int,
    ) -> HRESULT,
    fn AddPropertyChangedEventHandler(
        element: *mut IUIAutomationElement,
        scope: TreeScope,
        cacheRequest: *mut IUIAutomationCacheRequest,
        handler: *mut IUIAutomationPropertyChangedEventHandler,
        propertyArray: *mut SAFEARRAY,
    ) -> HRESULT,
    fn RemovePropertyChangedEventHandler(
        element: *mut IUIAutomationElement,
        handler: *mut IUIAutomationPropertyChangedEventHandler,
    ) -> HRESULT,
    fn AddStructureChangedEventHandler(
        element: *mut IUIAutomationElement,
        scope: TreeScope,
        cacheRequest: *mut IUIAutomationCacheRequest,
        handler: *mut IUIAutomationStructureChangedEventHandler,
    ) -> HRESULT,
    fn RemoveStructureChangedEventHandler(
        element: *mut IUIAutomationElement,
        handler: *mut IUIAutomationStructureChangedEventHandler,
    ) -> HRESULT,
    fn AddFocusChangedEventHandler(
        cacheRequest: *mut IUIAutomationCacheRequest,
        handler: *mut IUIAutomationFocusChangedEventHandler,
    ) -> HRESULT,
    fn RemoveFocusChangedEventHandler(
        handler: *mut IUIAutomationFocusChangedEventHandler,
    ) -> HRESULT,
    fn RemoveAllEventHandlers() -> HRESULT,
    fn IntNativeArrayToSafeArray(
        array: *mut c_int,
        arrayCount: c_int,
        safeArray: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    fn IntSafeArrayToNativeArray(
        intArray: *mut SAFEARRAY,
        array: *mut *mut c_int,
        arrayCount: *mut c_int,
    ) -> HRESULT,
    fn RectToVariant(
        rc: RECT,
        var: *mut VARIANT,
    ) -> HRESULT,
    fn VariantToRect(
        var: VARIANT,
        rc: *mut RECT,
    ) -> HRESULT,
    fn SafeArrayToRectNativeArray(
        rects: *mut SAFEARRAY,
        rectArray: *mut *mut RECT,
        rectArrayCount: *mut c_int,
    ) -> HRESULT,
    fn CreateProxyFactoryEntry(
        factory: *mut IUIAutomationProxyFactory,
        factoryEntry: *mut *mut IUIAutomationProxyFactoryEntry,
    ) -> HRESULT,
    fn get_ProxyFactoryMapping(
        factoryMapping: *mut *mut IUIAutomationProxyFactoryMapping,
    ) -> HRESULT,
    fn GetPropertyProgrammaticName(
        property: PROPERTYID,
        name: *mut BSTR,
    ) -> HRESULT,
    fn GetPatternProgrammaticName(
        pattern: PATTERNID,
        name: *mut BSTR,
    ) -> HRESULT,
    fn PollForPotentialSupportedPatterns(
        pElement: *mut IUIAutomationElement,
        patternIds: *mut *mut SAFEARRAY,
        patternNames: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    fn PollForPotentialSupportedProperties(
        pElement: *mut IUIAutomationElement,
        propertyIds: *mut *mut SAFEARRAY,
        propertyNames: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    fn CheckNotSupported(
        value: VARIANT,
        isNotSupported: *mut BOOL,
    ) -> HRESULT,
    fn get_ReservedNotSupportedValue(
        notSupportedValue: *mut *mut IUnknown,
    ) -> HRESULT,
    fn get_ReservedMixedAttributeValue(
        mixedAttributeValue: *mut *mut IUnknown,
    ) -> HRESULT,
    fn ElementFromIAccessible(
        accessible: *mut IAccessible,
        childId: c_int,
        element: *mut *mut IUIAutomationElement,
    ) -> HRESULT,
    fn ElementFromIAccessibleBuildCache(
        accessible: *mut IAccessible,
        childId: c_int,
        cacheRequest: *mut IUIAutomationCacheRequest,
        element: *mut *mut IUIAutomationElement,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x34723aff, 0x0c9d, 0x49d0, 0x98, 0x96, 0x7a, 0xb5, 0x2d, 0xf8, 0xcd, 0x8a)]
interface IUIAutomation2(IUIAutomation2Vtbl): IUIAutomation(IUIAutomationVtbl) {
    fn get_AutoSetFocus(
        autoSetFocus: *mut BOOL,
    ) -> HRESULT,
    fn put_AutoSetFocus(
        autoSetFocus: BOOL,
    ) -> HRESULT,
    fn get_ConnectionTimeout(
        timeout: *mut DWORD,
    ) -> HRESULT,
    fn put_ConnectionTimeout(
        timeout: DWORD,
    ) -> HRESULT,
    fn get_TransactionTimeout(
        timeout: *mut DWORD,
    ) -> HRESULT,
    fn put_TransactionTimeout(
        timeout: DWORD,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x73D768DA, 0x9B51, 0x4B89, 0x93, 0x6E, 0xC2, 0x09, 0x29, 0x09, 0x73, 0xE7)]
interface IUIAutomation3(IUIAutomation3Vtbl): IUIAutomation2(IUIAutomation2Vtbl) {
    fn AddTextEditTextChangedEventHandler(
        element: *mut IUIAutomationElement,
        scope: TreeScope,
        textEditChangeType: TextEditChangeType,
        cacheRequest: *mut IUIAutomationCacheRequest,
        handler: *mut IUIAutomationTextEditTextChangedEventHandler,
    ) -> HRESULT,
    fn RemoveTextEditTextChangedEventHandler(
        element: *mut IUIAutomationElement,
        handler: *mut IUIAutomationTextEditTextChangedEventHandler,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x1189C02A, 0x05F8, 0x4319, 0x8E, 0x21, 0xE8, 0x17, 0xE3, 0xDB, 0x28, 0x60)]
interface IUIAutomation4(IUIAutomation4Vtbl): IUIAutomation3(IUIAutomation3Vtbl) {
    fn AddChangesEventHandler(
        element: *mut IUIAutomationElement,
        scope: TreeScope,
        changeTypes: *mut c_int,
        changesCount: c_int,
        pCacheRequest: *mut IUIAutomationCacheRequest,
        handler: *mut IUIAutomationChangesEventHandler,
    ) -> HRESULT,
    fn RemoveChangesEventHandler(
        element: *mut IUIAutomationElement,
        handler: *mut IUIAutomationChangesEventHandler,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x25F700C8, 0xD816, 0x4057, 0xA9, 0xDC, 0x3C, 0xBD, 0xEE, 0x77, 0xE2, 0x56)]
interface IUIAutomation5(IUIAutomation5Vtbl): IUIAutomation4(IUIAutomation4Vtbl) {
    fn AddNotificationEventHandler(
        element: *mut IUIAutomationElement,
        scope: TreeScope,
        cacheRequest: *mut IUIAutomationCacheRequest,
        handler: *mut IUIAutomationNotificationEventHandler,
    ) -> HRESULT,
    fn RemoveNotificationEventHandler(
        element: *mut IUIAutomationElement,
        handler: *mut IUIAutomationNotificationEventHandler,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xAAE072DA, 0x29E3, 0x413D, 0x87, 0xA7, 0x19, 0x2D, 0xBF, 0x81, 0xED, 0x10)]
interface IUIAutomation6(IUIAutomation6Vtbl): IUIAutomation5(IUIAutomation5Vtbl) {
    fn CreateEventHandlerGroup(
        handlerGroup: *mut *mut IUIAutomationEventHandlerGroup,
    ) -> HRESULT,
    fn AddEventHandlerGroup(
        element: *mut IUIAutomationElement,
        handlerGroup: *mut IUIAutomationEventHandlerGroup,
    ) -> HRESULT,
    fn RemoveEventHandlerGroup(
        element: *mut IUIAutomationElement,
        handlerGroup: *mut IUIAutomationEventHandlerGroup,
    ) -> HRESULT,
    fn get_ConnectionRecoveryBehavior(
        connectionRecoveryBehaviorOptions: *mut ConnectionRecoveryBehaviorOptions,
    ) -> HRESULT,
    fn put_ConnectionRecoveryBehavior(
        connectionRecoveryBehaviorOptions: ConnectionRecoveryBehaviorOptions,
    ) -> HRESULT,
    fn get_CoalesceEvents(
        coalesceEventsOptions: *mut CoalesceEventsOptions,
    ) -> HRESULT,
    fn put_CoalesceEvents(
        coalesceEventsOptions: CoalesceEventsOptions,
    ) -> HRESULT,
    fn AddActiveTextPositionChangedEventHandler(
        element: *mut IUIAutomationElement,
        scope: TreeScope,
        cacheRequest: *mut IUIAutomationCacheRequest,
        handler: *mut IUIAutomationActiveTextPositionChangedEventHandler,
    ) -> HRESULT,
    fn RemoveActiveTextPositionChangedEventHandler(
        element: *mut IUIAutomationElement,
        handler: *mut IUIAutomationActiveTextPositionChangedEventHandler,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xff48dba4, 0x60ef, 0x4201, 0xaa, 0x87, 0x54, 0x10, 0x3e, 0xef, 0x59, 0x4e)]
class CUIAutomation;}
RIDL!{#[uuid(0xe22ad333, 0xb25f, 0x460c, 0x83, 0xd0, 0x05, 0x81, 0x10, 0x73, 0x95, 0xc9)]
class CUIAutomation8;}

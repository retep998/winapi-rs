use ctypes::{c_float, c_void};
use shared::basetsd::UINT32;
use shared::guiddef::GUID;
use shared::minwindef::{BOOL, DWORD};
use shared::ntdef::{HANDLE, HRESULT};
use shared::windef::{HWND, RECT};
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winuser::MSG;
DEFINE_GUID!{CLSID_VerticalIndicatorContent,
    0xa10b5f17, 0xafe0, 0x4aa2, 0x91, 0xe9, 0x3e, 0x70, 0x1, 0xd2, 0xe6, 0xb4}
DEFINE_GUID!{CLSID_HorizontalIndicatorContent,
    0xe7d18cf5, 0x3ec7, 0x44d5, 0xa7, 0x6b, 0x37, 0x70, 0xf3, 0xcf, 0x90, 0x3d}
DEFINE_GUID!{CLSID_VirtualViewportContent,
    0x3206a19a, 0x86f0, 0x4cb4, 0xa7, 0xf3, 0x16, 0xe3, 0xb7, 0xe2, 0xd8, 0x52}
DEFINE_GUID!{CLSID_DragDropConfigurationBehavior,
    0x09b01b3e, 0xba6c, 0x454d, 0x82, 0xe8, 0x95, 0xe3, 0x52, 0x32, 0x9f, 0x23}
DEFINE_GUID!{CLSID_AutoScrollBehavior,
    0x26126a51, 0x3c70, 0x4c9a, 0xae, 0xc2, 0x94, 0x88, 0x49, 0xee, 0xb0, 0x93}
DEFINE_GUID!{CLSID_DeferContactService ,
    0xd7b67cf4, 0x84bb, 0x434e, 0x86, 0xae, 0x65, 0x92, 0xbb, 0xc9, 0xab, 0xd9}
// Implements IDirectManipulationViewport2
// Implements IDirectManipulationViewport
RIDL!{#[uuid(0x34e211b6, 0x3650, 0x4f75, 0x83, 0x34, 0xfa, 0x35, 0x95, 0x98, 0xe1, 0xc5)]
class DirectManipulationViewport; }
RIDL!{#[uuid(0x923ccaac, 0x61e1, 0x4385, 0xb7, 0x26, 0x01, 0x7a, 0xf1, 0x89, 0x88, 0x2a)]
interface IDirectManipulationViewport2(IDirectManipulationViewport2Vtbl):
    IDirectManipulationViewport(IDirectManipulationViewportVtbl) {
    fn AddBehavior(
        behavior: *mut IUnknown,
        cookie: *mut DWORD,
    ) -> HRESULT,
    fn RemoveBehavior(
        cookie: DWORD,
    ) -> HRESULT,
    fn RemoveAllBehaviors(
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x28b85a3d, 0x60a0, 0x48bd, 0x9b, 0xa1, 0x5c, 0xe8, 0xd9, 0xea, 0x3a, 0x6d)]
interface IDirectManipulationViewport(IDirectManipulationViewportVtbl): IUnknown(IUnknownVtbl) {
    fn Enable(
    ) -> HRESULT,
    fn Disable(
    ) -> HRESULT,
    fn SetContact(
        pointerId: UINT32,
    ) -> HRESULT,
    fn ReleaseContact(
        pointerId: UINT32,
    ) -> HRESULT,
    fn ReleaseAllContacts(
    ) -> HRESULT,
    fn GetStatus(
        status: *mut DIRECTMANIPULATION_STATUS,
    ) -> HRESULT,
    fn GetTag(
        riid: *const GUID,
        object: *mut *mut c_void,
        id: *mut UINT32,
    ) -> HRESULT,
    fn SetTag(
        object: *mut IUnknown,
        id: UINT32,
    ) -> HRESULT,
    fn GetViewportRect(
        viewport: *mut RECT,
    ) -> HRESULT,
    fn SetViewportRect(
        viewport: *const RECT,
    ) -> HRESULT,
    fn ZoomToRect(
        left: c_float,
        top: c_float,
        right: c_float,
        bottom: c_float,
        animate: BOOL,
    ) -> HRESULT,
    fn SetViewportTransform(
        matrix: *const c_float,
        pointCount: DWORD,
    ) -> HRESULT,
    fn SyncDisplayTransform(
        matrix: *const c_float,
        pointCount: DWORD,
    ) -> HRESULT,
    fn GetPrimaryContent(
        riid: *const GUID,
        object: *mut *mut c_void,
    ) -> HRESULT,
    fn AddContent(
        content: *const IDirectManipulationContent,
    ) -> HRESULT,
    fn RemoveContent(
        content: *const IDirectManipulationContent,
    ) -> HRESULT,
    fn SetViewportOptions(
        options: DIRECTMANIPULATION_VIEWPORT_OPTIONS,
    ) -> HRESULT,
    fn AddConfiguration(
        configuration: DIRECTMANIPULATION_CONFIGURATION,
    ) -> HRESULT,
    fn RemoveConfiguration(
        configuration: DIRECTMANIPULATION_CONFIGURATION,
    ) -> HRESULT,
    fn ActivateConfiguration(
        configuration: DIRECTMANIPULATION_CONFIGURATION,
    ) -> HRESULT,
    fn SetManualGesture(
        configuration: DIRECTMANIPULATION_GESTURE_CONFIGURATION,
    ) -> HRESULT,
    fn SetChaining(
        enabledTypes: DIRECTMANIPULATION_MOTION_TYPES,
    ) -> HRESULT,
    fn AddEventHandler(
        window: HWND,
        eventHandler: *const IDirectManipulationViewportEventHandler,
        cookie: *mut DWORD,
    ) -> HRESULT,
    fn RemoveEventHandler(
        cookie: DWORD,
    ) -> HRESULT,
    fn SetInputMode(
        mode: DIRECTMANIPULATION_INPUT_MODE,
    ) -> HRESULT,
    fn SetUpdateMode(
        mode: DIRECTMANIPULATION_INPUT_MODE,
    ) -> HRESULT,
    fn Stop(
    ) -> HRESULT,
    fn Abandon(
    ) -> HRESULT,
}}
ENUM!{enum DIRECTMANIPULATION_STATUS {
    DIRECTMANIPULATION_BUILDING = 0,
    DIRECTMANIPULATION_ENABLED = 1,
    DIRECTMANIPULATION_DISABLED = 2,
    DIRECTMANIPULATION_RUNNING = 3,
    DIRECTMANIPULATION_INERTIA = 4,
    DIRECTMANIPULATION_READY = 5,
    DIRECTMANIPULATION_SUSPENDED = 6,
}}
RIDL!{#[uuid(0xb89962cb, 0x3d89, 0x442b, 0xbb, 0x58, 0x50, 0x98, 0xfa, 0x0f, 0x9f, 0x16)]
interface IDirectManipulationContent(IDirectManipulationContentVtbl): IUnknown(IUnknownVtbl) {
    fn GetContentRect(
        contentSize: *mut RECT,
    ) -> HRESULT,
    fn SetContentRect(
        contentSize: *const RECT,
    ) -> HRESULT,
    fn GetViewport(
        riid: *const GUID,
        object: *mut *mut c_void,
    ) -> HRESULT,
    fn GetTag(
        riid: *const GUID,
        object: *mut *mut c_void,
        id: *mut UINT32,
    ) -> HRESULT,
    fn SetTag(
        object: *mut IUnknown,
        id: UINT32,
    ) -> HRESULT,
    fn GetOutputTransform(
        matrix: *mut c_float,
        pointCount: DWORD,
    ) -> HRESULT,
    fn GetContentTransform(
        matrix: *mut c_float,
        pointCount: DWORD,
    ) -> HRESULT,
    fn SyncContentTransform(
        matrix: *const c_float,
        pointCount: DWORD,
    ) -> HRESULT,
}}
ENUM!{enum DIRECTMANIPULATION_VIEWPORT_OPTIONS {
    DIRECTMANIPULATION_VIEWPORT_OPTIONS_DEFAULT = 0,
    DIRECTMANIPULATION_VIEWPORT_OPTIONS_AUTODISABLE = 1,
    DIRECTMANIPULATION_VIEWPORT_OPTIONS_MANUALUPDATE = 2,
    DIRECTMANIPULATION_VIEWPORT_OPTIONS_INPUT = 4,
    DIRECTMANIPULATION_VIEWPORT_OPTIONS_EXPLICITHITTEST = 8,
    DIRECTMANIPULATION_VIEWPORT_OPTIONS_DISABLEPIXELSNAPPING = 16,
}}
ENUM!{enum DIRECTMANIPULATION_CONFIGURATION {
    DIRECTMANIPULATION_CONFIGURATION_NONE = 0,
    DIRECTMANIPULATION_CONFIGURATION_INTERACTION = 1,
    DIRECTMANIPULATION_CONFIGURATION_TRANSLATION_X = 2,
    DIRECTMANIPULATION_CONFIGURATION_TRANSLATION_Y = 4,
    DIRECTMANIPULATION_CONFIGURATION_SCALING = 16,
    DIRECTMANIPULATION_CONFIGURATION_TRANSLATION_INERTIA = 32,
    DIRECTMANIPULATION_CONFIGURATION_SCALING_INERTIA = 128,
    DIRECTMANIPULATION_CONFIGURATION_RAILS_X = 256,
    DIRECTMANIPULATION_CONFIGURATION_RAILS_Y = 512,
}}
ENUM!{enum DIRECTMANIPULATION_GESTURE_CONFIGURATION {
    DIRECTMANIPULATION_GESTURE_NONE = 0,
    DIRECTMANIPULATION_GESTURE_DEFAULT = 0,
    DIRECTMANIPULATION_GESTURE_CROSS_SLIDE_VERTICAL = 8,
    DIRECTMANIPULATION_GESTURE_CROSS_SLIDE_HORIZONTAL = 16,
    DIRECTMANIPULATION_GESTURE_PINCH_ZOOM = 32,
}}
ENUM!{enum DIRECTMANIPULATION_MOTION_TYPES {
    DIRECTMANIPULATION_MOTION_NONE = 0,
    DIRECTMANIPULATION_MOTION_TRANSLATEX = 1,
    DIRECTMANIPULATION_MOTION_TRANSLATEY = 2,
    DIRECTMANIPULATION_MOTION_ZOOM = 4,
    DIRECTMANIPULATION_MOTION_CENTERX = 16,
    DIRECTMANIPULATION_MOTION_CENTERY = 32,
    DIRECTMANIPULATION_MOTION_ALL = 55,
}}
RIDL!{#[uuid(0x952121da, 0xd69f, 0x45f9, 0xb0, 0xf9, 0xf2, 0x39, 0x44, 0x32, 0x1a, 0x6d)]
interface IDirectManipulationViewportEventHandler(IDirectManipulationViewportEventHandlerVtbl):
    IUnknown(IUnknownVtbl) {
    fn OnViewportStatusChanged(
        viewport: *const IDirectManipulationViewport,
        current: DIRECTMANIPULATION_STATUS,
        previous: DIRECTMANIPULATION_STATUS,
    ) -> HRESULT,
    fn OnViewportUpdated(
        viewport: *const IDirectManipulationViewport,
    ) -> HRESULT,
    fn OnContentUpdated(
        viewport: *const IDirectManipulationViewport,
        content: *const IDirectManipulationContent,
    ) -> HRESULT,
}}
ENUM!{enum DIRECTMANIPULATION_INPUT_MODE {
    DIRECTMANIPULATION_INPUT_MODE_AUTOMATIC = 0,
    DIRECTMANIPULATION_INPUT_MODE_MANUAL = 1,
}}
// Implements IDirectManipulationUpdateManager
RIDL!{#[uuid(0x9fc1bfd5, 0x1835, 0x441a, 0xb3, 0xb1, 0xb6, 0xcc, 0x74, 0xb7, 0x27, 0xd0)]
class DirectManipulationUpdateManager; }
RIDL!{#[uuid(0xb0ae62fd, 0xbe34, 0x46e7, 0x9c, 0xaa, 0xd3, 0x61, 0xfa, 0xcb, 0xb9, 0xcc)]
interface IDirectManipulationUpdateManager(IDirectManipulationUpdateManagerVtbl):
    IUnknown(IUnknownVtbl) {
    fn RegisterWaitHandleCallback(
        handle: HANDLE,
        eventHandler: *const IDirectManipulationUpdateHandler,
        cookie: *mut DWORD,
    ) -> HRESULT,
    fn UnregisterWaitHandleCallback(
        cookie: DWORD,
    ) -> HRESULT,
    fn Update(
        frameInfo: *const IDirectManipulationFrameInfoProvider,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x790b6337, 0x64f8, 0x4ff5, 0xa2, 0x69, 0xb3, 0x2b, 0xc2, 0xaf, 0x27, 0xa7)]
interface IDirectManipulationUpdateHandler(IDirectManipulationUpdateHandlerVtbl):
    IUnknown(IUnknownVtbl) {
    fn Update(
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xfb759dba, 0x6f4c, 0x4c01, 0x87, 0x4e, 0x19, 0xc8, 0xa0, 0x59, 0x07, 0xf9)]
interface IDirectManipulationFrameInfoProvider(IDirectManipulationFrameInfoProviderVtbl):
    IUnknown(IUnknownVtbl) {
    fn GetNextFrameInfo(
        time: *mut u64,
        processTime: *mut u64,
        compositionTime: *mut u64,
    ) -> HRESULT,
}}
// Implements IDirectManipulationPrimaryContent
RIDL!{#[uuid(0xcaa02661, 0xd59e, 0x41c7, 0x83, 0x93, 0x3b, 0xa3, 0xba, 0xcb, 0x6b, 0x57)]
class DirectManipulationPrimaryContent; }
RIDL!{#[uuid(0xc12851e4, 0x1698, 0x4625, 0xb9, 0xb1, 0x7c, 0xa3, 0xec, 0x18, 0x63, 0x0b)]
interface IDirectManipulationPrimaryContent(IDirectManipulationPrimaryContentVtbl):
    IUnknown(IUnknownVtbl) {
    fn SetSnapInterval(
        motion: DIRECTMANIPULATION_MOTION_TYPES,
        interval: c_float,
        offset: c_float,
    ) -> HRESULT,
    fn SetSnapPoints(
        motion: DIRECTMANIPULATION_MOTION_TYPES,
        points: *const c_float,
        pointCount: DWORD,
    ) -> HRESULT,
    fn SetSnapType(
        motion: DIRECTMANIPULATION_MOTION_TYPES,
        type_: DIRECTMANIPULATION_SNAPPOINT_TYPE,
    ) -> HRESULT,
    fn SetSnapCoordinate(
        motion: DIRECTMANIPULATION_MOTION_TYPES,
        coordinate: DIRECTMANIPULATION_SNAPPOINT_COORDINATE,
        origin: c_float,
    ) -> HRESULT,
    fn SetZoomBoundaries(
        zoomMinimum: c_float,
        zoomMaximum: c_float,
    ) -> HRESULT,
    fn SetHorizontalAlignment(
        alignment: DIRECTMANIPULATION_HORIZONTALALIGNMENT,
    ) -> HRESULT,
    fn SetVerticalAlignment(
        alignment: DIRECTMANIPULATION_VERTICALALIGNMENT,
    ) -> HRESULT,
    fn GetInertiaEndTransform(
        matrix: *mut c_float,
        pointCount: DWORD,
    ) -> HRESULT,
    fn GetCenterPoint(
        centerX: *mut c_float,
        centerY: *mut c_float,
    ) -> HRESULT,
}}
ENUM!{enum DIRECTMANIPULATION_SNAPPOINT_TYPE {
    DIRECTMANIPULATION_SNAPPOINT_MANDATORY = 0,
    DIRECTMANIPULATION_SNAPPOINT_OPTIONAL = 1,
    DIRECTMANIPULATION_SNAPPOINT_MANDATORY_SINGLE = 2,
    DIRECTMANIPULATION_SNAPPOINT_OPTIONAL_SINGLE = 3,
}}
ENUM!{enum DIRECTMANIPULATION_SNAPPOINT_COORDINATE {
    DIRECTMANIPULATION_COORDINATE_BOUNDARY = 0,
    DIRECTMANIPULATION_COORDINATE_ORIGIN = 1,
    DIRECTMANIPULATION_COORDINATE_MIRRORED = 16,
}}
ENUM!{enum DIRECTMANIPULATION_HORIZONTALALIGNMENT {
    DIRECTMANIPULATION_HORIZONTALALIGNMENT_NONE = 0,
    DIRECTMANIPULATION_HORIZONTALALIGNMENT_LEFT = 1,
    DIRECTMANIPULATION_HORIZONTALALIGNMENT_CENTER = 2,
    DIRECTMANIPULATION_HORIZONTALALIGNMENT_RIGHT = 4,
    DIRECTMANIPULATION_HORIZONTALALIGNMENT_UNLOCKCENTER = 8,
}}
ENUM!{enum DIRECTMANIPULATION_VERTICALALIGNMENT {
    DIRECTMANIPULATION_VERTICALALIGNMENT_NONE = 0,
    DIRECTMANIPULATION_VERTICALALIGNMENT_TOP = 1,
    DIRECTMANIPULATION_VERTICALALIGNMENT_CENTER = 2,
    DIRECTMANIPULATION_VERTICALALIGNMENT_BOTTOM = 4,
    DIRECTMANIPULATION_VERTICALALIGNMENT_UNLOCKCENTER = 8,
}}
// Implements IDirectManipulationManager2
// Implements IDirectManipulationManager
RIDL!{#[uuid(0x54e211b6, 0x3650, 0x4f75, 0x83, 0x34, 0xfa, 0x35, 0x95, 0x98, 0xe1, 0xc5)]
class DirectManipulationManager; }
RIDL!{#[uuid(0xfa1005e9, 0x3d16, 0x484c, 0xbf, 0xc9, 0x62, 0xb6, 0x1e, 0x56, 0xec, 0x4e)]
interface IDirectManipulationManager2(IDirectManipulationManager2Vtbl):
    IDirectManipulationManager(IDirectManipulationManagerVtbl) {
    fn CreateBehavior(
        clsid: *const GUID,
        riid: *const GUID,
        object: *mut *mut c_void,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xfbf5d3b4, 0x70c7, 0x4163, 0x93, 0x22, 0x5a, 0x6f, 0x66, 0x0d, 0x6f, 0xbc)]
interface IDirectManipulationManager(IDirectManipulationManagerVtbl): IUnknown(IUnknownVtbl) {
    fn Activate(
        window: HWND,
    ) -> HRESULT,
    fn Deactivate(
        window: HWND,
    ) -> HRESULT,
    fn RegisterHitTestTarget(
        window: HWND,
        hitTestWindow: *const c_void,
        type_: DIRECTMANIPULATION_HITTEST_TYPE,
    ) -> HRESULT,
    fn ProcessInput(
        message: *const MSG,
        handled: *mut BOOL,
    ) -> HRESULT,
    fn GetUpdateManager(
        riid: *const GUID,
        object: *mut *mut c_void,
    ) -> HRESULT,
    fn CreateViewport(
        frameInfo: *const IDirectManipulationFrameInfoProvider,
        window: HWND,
        riid: *const GUID,
        object: *mut *mut c_void,
    ) -> HRESULT,
    fn CreateContent(
        frameInfo: *const IDirectManipulationFrameInfoProvider,
        clsid: *const GUID,
        riid: *const GUID,
        object: *mut *mut c_void,
    ) -> HRESULT,
}}
ENUM!{enum DIRECTMANIPULATION_HITTEST_TYPE {
    DIRECTMANIPULATION_HITTEST_TYPE_ASYNCHRONOUS = 0,
    DIRECTMANIPULATION_HITTEST_TYPE_SYNCHRONOUS = 1,
    DIRECTMANIPULATION_HITTEST_TYPE_AUTO_SYNCHRONOUS = 2,
}}
// Implements IDirectManipulationManager2
// Implements IDirectManipulationManager
RIDL!{#[uuid(0x99793286, 0x77cc, 0x4b57, 0x96, 0xdb, 0x3b, 0x35, 0x4f, 0x6f, 0x9f, 0xb5)]
class DirectManipulationSharedManager; }
// Implements IDirectManipulationCompositor
// Implements IDirectManipulationCompositor2
// Implements IDirectManipulationFrameInfoProvider
RIDL!{#[uuid(0x79dea627, 0xa08a, 0x43ac, 0x8e, 0xf5, 0x69, 0x00, 0xb9, 0x29, 0x91, 0x26)]
class DCompManipulationCompositor; }
RIDL!{#[uuid(0x537a0825, 0x0387, 0x4efa, 0xb6, 0x2f, 0x71, 0xeb, 0x1f, 0x08, 0x5a, 0x7e)]
interface IDirectManipulationCompositor(IDirectManipulationCompositorVtbl):
    IUnknown(IUnknownVtbl) {
    fn AddContent(
        content: *const IDirectManipulationContent,
        device: *mut IUnknown,
        parentVisual: *mut IUnknown,
        childVisual: *mut IUnknown,
    ) -> HRESULT,
    fn RemoveContent(
        content: *const IDirectManipulationContent,
    ) -> HRESULT,
    fn SetUpdateManager(
        updateManager: *const IDirectManipulationUpdateManager,
    ) -> HRESULT,
    fn Flush(
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xd38c7822, 0xf1cb, 0x43cb, 0xb4, 0xb9, 0xac, 0x0c, 0x76, 0x7a, 0x41, 0x2e)]
interface IDirectManipulationCompositor2(IDirectManipulationCompositor2Vtbl):
    IDirectManipulationCompositor(IDirectManipulationCompositorVtbl) {
    fn AddContentWithCrossProcessChaining(
        content: *const IDirectManipulationPrimaryContent,
        device: *mut IUnknown,
        parentVisual: *mut IUnknown,
        childVisual: *mut IUnknown,
    ) -> HRESULT,
}}

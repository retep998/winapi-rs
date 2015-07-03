// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! Types and constants for WinAPI bindings.
#![allow(bad_style, raw_pointer_derive)]
#![warn(missing_copy_implementations, trivial_casts, trivial_numeric_casts)]
#![warn(unused_qualifications, unused)]
#![cfg(windows)]
//-------------------------------------------------------------------------------------------------
// Imports
//-------------------------------------------------------------------------------------------------
pub use std::os::raw::{
    c_void,
    c_char,
    c_schar,
    c_uchar,
    c_short,
    c_ushort,
    c_int,
    c_uint,
    c_long,
    c_ulong,
    c_longlong,
    c_ulonglong,
    c_float,
    c_double,
};
pub use audioclient::*;
pub use basetsd::*;
pub use commctrl::*;
pub use d3d9::*;
pub use d3d9caps::*;
pub use d3d9types::*;
pub use dbghelp::*;
pub use dwmapi::*;
pub use excpt::*;
pub use fileapi::*;
pub use guiddef::*;
pub use heapapi::*;
pub use inaddr::*;
pub use libloaderapi::*;
pub use minwinbase::*;
pub use minwindef::*;
pub use mmdeviceapi::*;
pub use mmreg::*;
pub use mmsystem::*;
pub use objidl::*;
pub use objidlbase::*;
pub use processthreadsapi::*;
pub use propsys::*;
pub use schannel::*;
pub use shellapi::*;
pub use shobjidl::*;
pub use shtypes::*;
pub use sspi::*;
pub use synchapi::*;
pub use timezoneapi::*;
pub use tlhelp32::*;
pub use unknwnbase::*;
pub use usp10::*;
pub use vadefs::*;
pub use winbase::*;
pub use wincon::*;
pub use wincred::*;
pub use wincrypt::*;
pub use windowsx::*;
pub use windef::*;
pub use winerror::*;
pub use wingdi::*;
pub use winioctl::*;
pub use winnetwk::*;
pub use winnls::*;
pub use winnt::*;
pub use winsock2::*;
pub use winsvc::*;
pub use winuser::*;
pub use ws2def::*;
pub use wtypesbase::*;
//-------------------------------------------------------------------------------------------------
// Macros
//-------------------------------------------------------------------------------------------------
macro_rules! DECLARE_HANDLE {
    ($name:ident, $inner:ident) => {
        #[repr(C)] #[allow(missing_copy_implementations)] struct $inner { unused: () }
        pub type $name = *mut $inner;
    };
}
macro_rules! MAKE_HRESULT {
    ($sev:expr, $fac:expr, $code:expr) => {
        ($sev << 31) | ($fac << 16) | $code
    }
}
macro_rules! MAKE_SCODE {
    ($sev:expr, $fac:expr, $code:expr) => {
        ($sev << 31) | ($fac << 16) | $code
    }
}
macro_rules! MAKEFOURCC {
    ($a:expr, $b:expr, $c:expr, $d:expr) => {
        ($a as i32) | (($b as i32) << 8) | (($c as i32) << 16) | (($d as i32) << 24)
    }
}
macro_rules! DEFINE_GUID {
    (
        $name:ident, $l:expr, $w1:expr, $w2:expr, $b1:expr, $b2:expr, $b3:expr, $b4:expr, $b5:expr,
        $b6:expr, $b7:expr, $b8:expr
    ) => {
        pub const $name: ::GUID = ::GUID {
            Data1: $l,
            Data2: $w1,
            Data3: $w2,
            Data4: [$b1, $b2, $b3, $b4, $b5, $b6, $b7, $b8],
        };
    }
}
macro_rules! CTL_CODE {
    ($DeviceType:expr, $Function:expr, $Method:expr, $Access:expr) => {
        ($DeviceType << 16) | ($Access << 14) | ($Function << 2) | $Method
    }
}
macro_rules! RIDL {
    (interface $interface:ident ($vtbl:ident)
        {$(
            fn $method:ident(&mut self $(,$p:ident : $t:ty)*) -> $rtr:ty
        ),+}
    ) => {
        #[repr(C)] #[allow(missing_copy_implementations)]
        pub struct $vtbl {
            $(pub $method: unsafe extern "system" fn(
                This: *mut $interface
                $(,$p: $t)*
            ) -> $rtr),+
        }
        #[repr(C)] #[derive(Debug)] #[allow(missing_copy_implementations)]
        pub struct $interface {
            pub lpVtbl: *const $vtbl
        }
        impl $interface {
            #[inline]
            $(pub unsafe fn $method(&mut self $(,$p: $t)*) -> $rtr {
                ((*self.lpVtbl).$method)(self $(,$p)*)
            })+
        }
    };
    (interface $interface:ident ($vtbl:ident) : $pinterface:ident ($pvtbl:ident)
        {$(
            fn $method:ident(&mut self $(,$p:ident : $t:ty)*) -> $rtr:ty
        ),+}
    ) => {
        #[repr(C)] #[allow(missing_copy_implementations)]
        pub struct $vtbl {
            pub parent: ::$pvtbl
            $(,pub $method: unsafe extern "system" fn(
                This: *mut $interface
                $(,$p: $t)*
            ) -> $rtr)+
        }
        #[repr(C)] #[derive(Debug)] #[allow(missing_copy_implementations)]
        pub struct $interface {
            pub lpVtbl: *const $vtbl
        }
        impl $interface {
            #[inline]
            $(pub unsafe fn $method(&mut self $(,$p: $t)*) -> $rtr {
                ((*self.lpVtbl).$method)(self $(,$p)*)
            })+
        }
        impl ::std::ops::Deref for $interface {
            type Target = ::$pinterface;
            #[inline]
            fn deref(&self) -> &::$pinterface {
                unsafe { ::std::mem::transmute(self) }
            }
        }
        impl ::std::ops::DerefMut for $interface {
            #[inline]
            fn deref_mut(&mut self) -> &mut ::$pinterface {
                unsafe { ::std::mem::transmute(self) }
            }
        }
    };
}
macro_rules! UNION {
    ($base:ident, $field:ident, $variant:ident, $variantmut: ident, $fieldtype:ty) => {
        impl $base {
            #[inline]
            pub unsafe fn $variant(&self) -> &$fieldtype {
                ::std::mem::transmute(&self.$field)
            }
            #[inline]
            pub unsafe fn $variantmut(&mut self) -> &mut $fieldtype {
                ::std::mem::transmute(&mut self.$field)
            }
        }
    }
}
macro_rules! BITFIELD {
    ($base:ident $field:ident: $fieldtype:ty [
        $($thing:ident $set_thing:ident[$r:expr],)+
    ]) => {
        impl $base {$(
            #[inline]
            pub fn $thing(&self) -> $fieldtype {
                let size = ::std::mem::size_of::<$fieldtype>() * 8;
                self.$field << (size - $r.end) >> (size - $r.end + $r.start)
            }
            #[inline]
            pub fn $set_thing(&mut self, val: $fieldtype) {
                let mask = ((1 << ($r.end - $r.start)) - 1) << $r.start;
                self.$field &= !mask;
                self.$field |= (val << $r.start) & mask;
            }
        )+}
    }
}
//-------------------------------------------------------------------------------------------------
// Modules
//-------------------------------------------------------------------------------------------------
pub mod audioclient;
pub mod basetsd;
pub mod commctrl;
pub mod d3d9;
pub mod d3d9caps;
pub mod d3d9types;
pub mod dbghelp;
pub mod dwmapi;
pub mod excpt;
pub mod fileapi;
pub mod guiddef;
pub mod heapapi;
pub mod inaddr;
pub mod libloaderapi;
pub mod minwinbase;
pub mod minwindef;
pub mod mmdeviceapi;
pub mod mmreg;
pub mod mmsystem;
pub mod objidl;
pub mod objidlbase;
pub mod processthreadsapi;
pub mod propsys;
pub mod schannel;
pub mod shellapi;
pub mod shobjidl;
pub mod shtypes;
pub mod sspi;
pub mod synchapi;
pub mod timezoneapi;
pub mod tlhelp32;
pub mod unknwnbase;
pub mod usp10;
pub mod vadefs;
pub mod winbase;
pub mod wincon;
pub mod wincred;
pub mod wincrypt;
pub mod windef;
pub mod windowsx;
pub mod winerror;
pub mod wingdi;
pub mod winioctl;
pub mod winnetwk;
pub mod winnls;
pub mod winnt;
pub mod winsock2;
pub mod winsvc;
pub mod winuser;
pub mod ws2def;
pub mod wtypesbase;
//-------------------------------------------------------------------------------------------------
// Primitive types not provided by std
//-------------------------------------------------------------------------------------------------
pub type __int8 = i8;
pub type __uint8 = u8;
pub type __int16 = i16;
pub type __uint16 = u16;
pub type __int32 = i32;
pub type __uint32 = u32;
pub type __int64 = i64;
pub type __uint64 = u64;
pub type wchar_t = c_ushort;
#[cfg(target_arch = "x86")]
pub type size_t = c_uint;
#[cfg(target_arch = "x86_64")]
pub type size_t = __uint64;

// shlobj.h
pub type GPFIDL_FLAGS = c_int;
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum KNOWN_FOLDER_FLAG {
    KF_FLAG_DEFAULT = 0x00000000,
    KF_FLAG_NO_APPCONTAINER_REDIRECTION = 0x00010000,
    KF_FLAG_CREATE = 0x00008000,
    KF_FLAG_DONT_VERIFY = 0x00004000,
    KF_FLAG_DONT_UNEXPAND = 0x00002000,
    KF_FLAG_NO_ALIAS = 0x00001000,
    KF_FLAG_INIT = 0x00000800,
    KF_FLAG_DEFAULT_PATH = 0x00000400,
    KF_FLAG_NOT_PARENT_RELATIVE = 0x00000200,
    KF_FLAG_SIMPLE_IDLIST = 0x00000100,
    KF_FLAG_ALIAS_ONLY = 0x80000000u32 as i32,
}
pub const IDO_SHGIOI_SHARE: c_int = 0x0FFFFFFF;
pub const IDO_SHGIOI_LINK: c_int = 0x0FFFFFFE;
// Yes, these values are supposed to overflow. Blame Microsoft.
pub const IDO_SHGIOI_SLOWFILE: c_int = 0xFFFFFFFDu32 as c_int;
pub const IDO_SHGIOI_DEFAULT: c_int = 0xFFFFFFFCu32 as c_int;
pub const GPFIDL_DEFAULT: GPFIDL_FLAGS = 0x0000;
pub const GPFIDL_ALTNAME: GPFIDL_FLAGS = 0x0001;
pub const GPFIDL_UNCPRINTER: GPFIDL_FLAGS = 0x0002;
pub const OFASI_EDIT: DWORD = 0x0001;
pub const OFASI_OPENDESKTOP: DWORD = 0x0002;
// shlobjdl.h
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IContextMenu;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IContextMenu2;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IContextMenu3;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IExecuteCommand;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IPersistFolder;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IRunnableTask;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IShellTaskScheduler;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IQueryCodePage;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IPersistFolder2;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IPersistFolder3;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IPersistIDList;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IEnumIDList;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IEnumFullIDList;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IFileSyncMergeHandler;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IObjectWithFolderEnumMode;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IParseAndCreateItem;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IShellFolder;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IEnumExtraSearch;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IShellFolder2;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IFolderViewOptions;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IShellView;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IShellView2;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IShellView3;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IFolderView;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ISearchBoxInfo;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IFolderView2;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IFolderViewSettings;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IPreviewHandlerVisuals;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IVisualProperties;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ICommDlgBrowser;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ICommDlgBrowser2;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ICommDlgBrowser3;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IColumnManager;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IFolderFilterSite;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IFolderFilter;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IInputObjectSite;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IInputObject;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IInputObject2;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IShellIcon;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IShellBrowser;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IProfferService;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IShellItem2;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IShellItemImageFactory;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IUserAccountChangeCallback;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IEnumShellItems;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ITransferAdviseSink;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ITransferSource;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IEnumResources;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IShellItemResources;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ITransferDestination;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IStreamAsync;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IStreamUnbufferedInfo;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IInitializeWithItem;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IObjectWithSelection;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IObjectWithBackReferences;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IPropertyUI;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ICategoryProvider;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ICategorizer;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IDropTargetHelper;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IDragSourceHelper;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IDragSourceHelper2;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IShellLinkA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IShellLinkW;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IShellLinkDataList;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IResolveShellLink;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IActionProgressDialog;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IHWEventHandler;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IHWEventHandler2;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IQueryCancelAutoPlay;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IDynamicHWHandler;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IActionProgress;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IShellExtInit;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IShellPropSheetExt;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IRemoteComputer;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IQueryContinue;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IObjectWithCancelEvent;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IUserNotification;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IUserNotificationCallback;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IUserNotification2;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IItemNameLimits;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ISearchFolderItemFactory;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IExtractImage;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IExtractImage2;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IThumbnailHandlerFactory;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IParentAndItem;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IDockingWindow;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IDeskBand;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IDeskBandInfo;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IDeskBand2;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ITaskbarList;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ITaskbarList2;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ITaskbarList3;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ITaskbarList4;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IStartMenuPinnedList;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ICDBurn;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IWizardSite;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IWizardExtension;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IWebWizardExtension;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IPublishingWizard;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IFolderViewHost;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IExplorerBrowserEvents;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IExplorerBrowser;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IAccessibleObject;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IResultsFolder;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IEnumObjects;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IOperationsProgressDialog;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IIOCancelInformation;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IFileOperation;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IObjectProvider;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct INamespaceWalkCB;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct INamespaceWalkCB2;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct INamespaceWalk;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IAutoCompleteDropDown;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IBandSite;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ICDBurnExt;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IContextMenuSite;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IEnumReadyCallback;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IEnumerableView;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IInsertItem;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IMenuBand;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IFolderBandPriv;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IRegTreeItem;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IImageRecompress;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IDeskBar;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IMenuPopup;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IFileIsInUse;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IApplicationAssociationRegistration;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IApplicationAssociationRegistrationUI;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IDelegateFolder;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IBrowserFrameOptions;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct INewWindowManager;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IAttachmentExecute;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IShellMenuCallback;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IShellMenu;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IShellRunDll;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IKnownFolder;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IKnownFolderManager;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ISharingConfigurationManager;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IPreviousVersionsInfo;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IRelatedItem;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IIdentityName;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IDelegateItem;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ICurrentItem;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ITransferMediumItem;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IUseToBrowseItem;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IDisplayItem;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IViewStateIdentityItem;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IPreviewItem;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IDestinationStreamFactory;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct INewMenuClient;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IInitializeWithBindCtx;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct INameSpaceTreeControl;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct INameSpaceTreeControl2;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct INameSpaceTreeControlEvents;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct INameSpaceTreeControlDropHandler;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct INameSpaceTreeAccessible;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct INameSpaceTreeControlCustomDraw;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct INameSpaceTreeControlFolderCapabilities;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IPreviewHandler;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IPreviewHandlerFrame;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ITrayDeskBand;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IBandHost;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IExplorerPaneVisibility;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IContextMenuCB;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IDefaultExtractIconInit;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IExplorerCommand;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IExplorerCommandState;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IInitializeCommand;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IEnumExplorerCommand;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IExplorerCommandProvider;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IMarkupCallback;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IControlMarkup;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IInitializeNetworkFolder;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IOpenControlPanel;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IComputerInfoChangeNotify;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IFileSystemBindData;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IFileSystemBindData2;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ICustomDestinationList;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IApplicationDestinations;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IApplicationDocumentLists;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IObjectWithAppUserModelID;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IObjectWithProgID;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IUpdateIDList;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IDesktopGadget;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IDesktopWallpaper;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IHomeGroup;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IInitializeWithPropertyStore;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IOpenSearchSource;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IShellLibrary;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IDefaultFolderMenuInitialize;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IApplicationActivationManager;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IAssocHandlerInvoker;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IAssocHandler;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IEnumAssocHandlers;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IDataObjectProvider;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IDataTransferManagerInterop;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IFrameworkInputPaneHandler;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IFrameworkInputPane;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IAccessibilityDockingServiceCallback;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IAccessibilityDockingService;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IAppVisibilityEvents;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IAppVisibility;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IPackageExecutionStateChangeNotification;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IPackageDebugSettings;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ISuspensionDependencyManager;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IExecuteCommandApplicationHostEnvironment;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IExecuteCommandHost;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IApplicationDesignModeSettings;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IApplicationDesignModeSettings2;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ILaunchTargetMonitor;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ILaunchSourceViewSizePreference;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ILaunchTargetViewSizePreference;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ILaunchSourceAppUserModelId;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IInitializeWithWindow;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IHandlerInfo;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IHandlerActivationHost;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IContactManagerInterop;

//-------------------------------------------------------------------------------------------------
// wtypes.h
//-------------------------------------------------------------------------------------------------
pub type DATE = c_double;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DECIMAL {
    pub wReserved: USHORT,
    pub scale: BYTE,
    pub sign: BYTE,
    pub Hi32: ULONG,
    pub Lo64: ULONGLONG,
}
pub const DECIMAL_NEG: ::BYTE = 0x80;
pub type LPDECIMAL = *mut DECIMAL;
pub type VARTYPE = c_ushort;

//-------------------------------------------------------------------------------------------------
// audiosessiontypes.h
//-------------------------------------------------------------------------------------------------
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub enum AUDCLNT_SHAREMODE {
    AUDCLNT_SHAREMODE_SHARED,
    AUDCLNT_SHAREMODE_EXCLUSIVE,
}
pub const AUDCLNT_STREAMFLAGS_CROSSPROCESS: DWORD = 0x00010000;
pub const AUDCLNT_STREAMFLAGS_LOOPBACK: DWORD = 0x00020000;
pub const AUDCLNT_STREAMFLAGS_EVENTCALLBACK: DWORD = 0x00040000;
pub const AUDCLNT_STREAMFLAGS_NOPERSIST: DWORD = 0x00080000;
pub const AUDCLNT_STREAMFLAGS_RATEADJUST: DWORD = 0x00100000;

//-------------------------------------------------------------------------------------------------
// strmif.h
//-------------------------------------------------------------------------------------------------
pub type REFERENCE_TIME = LONGLONG;

//-------------------------------------------------------------------------------------------------
// mmreg.h
//-------------------------------------------------------------------------------------------------
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct WAVEFORMATEX {
    pub wFormatTag: WORD,
    pub nChannels: WORD,
    pub nSamplesPerSec: DWORD,
    pub nAvgBytesPerSec: DWORD,
    pub nBlockAlign: WORD,
    pub wBitsPerSample: WORD,
    pub cbSize: WORD,
}

//-------------------------------------------------------------------------------------------------
// propidl.h
//-------------------------------------------------------------------------------------------------
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct PROPVARIANT {
    pub vt: VARTYPE,
    pub wReserved1: WORD,
    pub wReserved2: WORD,
    pub wReserved3: WORD,
    pub data: [u8; 16],
}

//-------------------------------------------------------------------------------------------------
// combaseapi.h
// Base Component Object Model defintions.
//-------------------------------------------------------------------------------------------------
pub const CLSCTX_INPROC_SERVER: DWORD = 0x1;
pub const CLSCTX_INPROC_HANDLER: DWORD = 0x2;
pub const CLSCTX_LOCAL_SERVER: DWORD = 0x4;
pub const CLSCTX_REMOTE_SERVER: DWORD = 0x10;
pub const CLSCTX_SERVER: DWORD = CLSCTX_INPROC_SERVER | CLSCTX_LOCAL_SERVER |
                                 CLSCTX_REMOTE_SERVER;
pub const CLSCTX_ALL: DWORD = CLSCTX_INPROC_HANDLER | CLSCTX_SERVER;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ServerInformation {
    pub dwServerPid: DWORD,
    pub dwServerTid: DWORD,
    pub ui64ServerAddress: UINT64,
}
pub type PServerInformation = *mut ServerInformation;
DECLARE_HANDLE!(CO_MTA_USAGE_COOKIE, CO_MTA_USAGE_COOKIE__);

//-------------------------------------------------------------------------------------------------
// playsoundapi.h
// ApiSet Contract for api-ms-win-mm-playsound-l1-1-0
//-------------------------------------------------------------------------------------------------
pub const SND_SYNC: DWORD = 0x0000;
pub const SND_ASYNC: DWORD = 0x0001;
pub const SND_NODEFAULT: DWORD = 0x0002;
pub const SND_MEMORY: DWORD = 0x0004;
pub const SND_LOOP: DWORD = 0x0008;
pub const SND_NOSTOP: DWORD = 0x0010;
pub const SND_NOWAIT: DWORD = 0x00002000;
pub const SND_ALIAS: DWORD = 0x00010000;
pub const SND_ALIAS_ID: DWORD = 0x00110000;
pub const SND_FILENAME: DWORD = 0x00020000;
pub const SND_RESOURCE: DWORD = 0x00040004;
pub const SND_PURGE: DWORD = 0x0040;
pub const SND_APPLICATION: DWORD = 0x0080;
pub const SND_SENTRY: DWORD = 0x00080000;
pub const SND_RING: DWORD = 0x00100000;
pub const SND_SYSTEM: DWORD = 0x00200000;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct PROCESS_MEMORY_COUNTERS {
    pub cb: DWORD,
    pub PageFaultCount: DWORD,
    pub PeakWorkingSetSize: SIZE_T,
    pub WorkingSetSize: SIZE_T,
    pub QuotaPeakPagedPoolUsage: SIZE_T,
    pub QuotaPagedPoolUsage: SIZE_T,
    pub QuotaPeakNonPagedPoolUsage: SIZE_T,
    pub QuotaNonPagedPoolUsage: SIZE_T,
    pub PagefileUsage: SIZE_T,
    pub PeakPagefileUsage: SIZE_T,
}
pub type PPROCESS_MEMORY_COUNTERS = *mut PROCESS_MEMORY_COUNTERS;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct PROCESS_MEMORY_COUNTERS_EX {
    pub cb: DWORD,
    pub PageFaultCount: DWORD,
    pub PeakWorkingSetSize: SIZE_T,
    pub WorkingSetSize: SIZE_T,
    pub QuotaPeakPagedPoolUsage: SIZE_T,
    pub QuotaPagedPoolUsage: SIZE_T,
    pub QuotaPeakNonPagedPoolUsage: SIZE_T,
    pub QuotaNonPagedPoolUsage: SIZE_T,
    pub PagefileUsage: SIZE_T,
    pub PeakPagefileUsage: SIZE_T,
    pub PrivateUsage: SIZE_T,
}

//-------------------------------------------------------------------------------------------------
// winreg.h
// Registry API procedure declarations, constant definitions and macros
//-------------------------------------------------------------------------------------------------

pub type REGSAM = ACCESS_MASK;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct VALENTA {
    pub ve_valuename: LPSTR,
    pub ve_valuelen: DWORD,
    pub ve_valueptr: DWORD_PTR,
    pub ve_type: DWORD,
}
pub type PVALENTA = *mut VALENTA;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct VALENTW {
    pub ve_valuename: LPWSTR,
    pub ve_valuelen: DWORD,
    pub ve_valueptr: DWORD_PTR,
    pub ve_type: DWORD,
}
pub type PVALENTW = *mut VALENTW;

pub const HKEY_CLASSES_ROOT: HKEY = 0x80000000 as HKEY;
pub const HKEY_CURRENT_USER: HKEY = 0x80000001 as HKEY;
pub const HKEY_LOCAL_MACHINE: HKEY = 0x80000002 as HKEY;
pub const HKEY_USERS: HKEY = 0x80000003 as HKEY;
pub const HKEY_PERFORMANCE_DATA: HKEY = 0x80000004 as HKEY;
pub const HKEY_PERFORMANCE_TEXT: HKEY = 0x80000050 as HKEY;
pub const HKEY_PERFORMANCE_NLSTEXT: HKEY = 0x80000060 as HKEY;
pub const HKEY_CURRENT_CONFIG: HKEY = 0x80000005 as HKEY;
pub const HKEY_DYN_DATA: HKEY = 0x80000006 as HKEY;
pub const HKEY_CURRENT_USER_LOCAL_SETTINGS: HKEY = 0x80000007 as HKEY;

pub const REG_MUI_STRING_TRUNCATE: DWORD = 0x00000001;

pub const RRF_RT_REG_NONE: DWORD = 0x00000001;
pub const RRF_RT_REG_SZ: DWORD = 0x00000002;
pub const RRF_RT_REG_EXPAND_SZ: DWORD = 0x00000004;
pub const RRF_RT_REG_BINARY: DWORD = 0x00000008;
pub const RRF_RT_REG_DWORD: DWORD = 0x00000010;
pub const RRF_RT_REG_MULTI_SZ: DWORD = 0x00000020;
pub const RRF_RT_REG_QWORD: DWORD = 0x00000040;

pub const RRF_RT_DWORD: DWORD = RRF_RT_REG_BINARY|RRF_RT_REG_DWORD;
pub const RRF_RT_QWORD: DWORD = RRF_RT_REG_BINARY|RRF_RT_REG_QWORD;
pub const RRF_RT_ANY: DWORD = 0x0000ffff;

pub const RRF_NOEXPAND: DWORD = 0x10000000;
pub const RRF_ZEROONFAILURE: DWORD = 0x20000000;

//-------------------------------------------------------------------------------------------------
// winuser.h
// USER procedure declarations, constant definitions and macros
//-------------------------------------------------------------------------------------------------
pub const BN_CLICKED: WORD = 0;
pub const BN_PAINT: WORD = 1;
pub const BN_HILITE: WORD = 2;
pub const BN_UNHILITE: WORD = 3;
pub const BN_DISABLE: WORD = 4;
pub const BN_DOUBLECLICKED: WORD = 5;
pub const BN_PUSHED: WORD = BN_HILITE;
pub const BN_UNPUSHED: WORD = BN_UNHILITE;
pub const BN_DBLCLK: WORD = BN_DOUBLECLICKED;
pub const BN_SETFOCUS: WORD = 6;
pub const BN_KILLFOCUS: WORD = 7;

pub const BS_PUSHBUTTON: DWORD = 0x00000000;
pub const BS_DEFPUSHBUTTON: DWORD = 0x00000001;
pub const BS_CHECKBOX: DWORD = 0x00000002;
pub const BS_AUTOCHECKBOX: DWORD = 0x00000003;
pub const BS_RADIOBUTTON: DWORD = 0x00000004;
pub const BS_3STATE: DWORD = 0x00000005;
pub const BS_AUTO3STATE: DWORD = 0x00000006;
pub const BS_GROUPBOX: DWORD = 0x00000007;
pub const BS_USERBUTTON: DWORD = 0x00000008;
pub const BS_AUTORADIOBUTTON: DWORD = 0x00000009;
pub const BS_PUSHBOX: DWORD = 0x0000000A;
pub const BS_OWNERDRAW: DWORD = 0x0000000B;
pub const BS_TYPEMASK: DWORD = 0x0000000F;
pub const BS_LEFTTEXT: DWORD = 0x00000020;
pub const BS_TEXT: DWORD = 0x00000000;
pub const BS_ICON: DWORD = 0x00000040;
pub const BS_BITMAP: DWORD = 0x00000080;
pub const BS_LEFT: DWORD = 0x00000100;
pub const BS_RIGHT: DWORD = 0x00000200;
pub const BS_CENTER: DWORD = 0x00000300;
pub const BS_TOP: DWORD = 0x00000400;
pub const BS_BOTTOM: DWORD = 0x00000800;
pub const BS_VCENTER: DWORD = 0x00000C00;
pub const BS_PUSHLIKE: DWORD = 0x00001000;
pub const BS_MULTILINE: DWORD = 0x00002000;
pub const BS_NOTIFY: DWORD = 0x00004000;
pub const BS_FLAT: DWORD = 0x00008000;
pub const BS_RIGHTBUTTON: DWORD = BS_LEFTTEXT;

pub const CCHILDREN_SCROLLBAR: usize = 5;

pub const CDS_UPDATEREGISTRY: DWORD = 0x00000001;
pub const CDS_TEST: DWORD = 0x00000002;
pub const CDS_FULLSCREEN: DWORD = 0x00000004;
pub const CDS_GLOBAL: DWORD = 0x00000008;
pub const CDS_SET_PRIMARY: DWORD = 0x00000010;
pub const CDS_VIDEOPARAMETERS: DWORD = 0x00000020;
pub const CDS_ENABLE_UNSAFE_MODES: DWORD = 0x00000100;
pub const CDS_DISABLE_UNSAFE_MODES: DWORD = 0x00000200;
pub const CDS_RESET: DWORD = 0x40000000;
pub const CDS_RESET_EX: DWORD = 0x20000000;
pub const CDS_NORESET: DWORD = 0x10000000;

pub const CS_VREDRAW: DWORD = 0x0001;
pub const CS_HREDRAW: DWORD = 0x0002;
pub const CS_DBLCLKS: DWORD = 0x0008;
pub const CS_OWNDC: DWORD = 0x0020;
pub const CS_CLASSDC: DWORD = 0x0040;
pub const CS_PARENTDC: DWORD = 0x0080;
pub const CS_NOCLOSE: DWORD = 0x0200;
pub const CS_SAVEBITS: DWORD = 0x0800;
pub const CS_BYTEALIGNCLIENT: DWORD = 0x1000;
pub const CS_BYTEALIGNWINDOW: DWORD = 0x2000;
pub const CS_GLOBALCLASS: DWORD = 0x4000;
pub const CS_IME: DWORD = 0x00010000;
pub const CS_DROPSHADOW: DWORD = 0x00020000;

pub const CW_USEDEFAULT: c_int = 0x80000000u32 as c_int;

pub const DISP_CHANGE_SUCCESSFUL: LONG = 0;
pub const DISP_CHANGE_RESTART: LONG = 1;
pub const DISP_CHANGE_FAILED: LONG = -1;
pub const DISP_CHANGE_BADMODE: LONG = -2;
pub const DISP_CHANGE_NOTUPDATED: LONG = -3;
pub const DISP_CHANGE_BADFLAGS: LONG = -4;
pub const DISP_CHANGE_BADPARAM: LONG = -5;
pub const DISP_CHANGE_BADDUALVIEW: LONG = -6;

pub const EDD_GET_DEVICE_INTERFACE_NAME: DWORD = 0x00000001;

pub const ENUM_CURRENT_SETTINGS: DWORD = 0xFFFFFFFF;
pub const ENUM_REGISTRY_SETTINGS: DWORD = 0xFFFFFFFE;

pub const GW_HWNDFIRST: UINT = 0;
pub const GW_HWNDLAST: UINT = 1;
pub const GW_HWNDNEXT: UINT = 2;
pub const GW_HWNDPREV: UINT = 3;
pub const GW_OWNER: UINT = 4;
pub const GW_CHILD: UINT = 5;
pub const GW_ENABLEDPOPUP: UINT = 6;
pub const GW_MAX: UINT = 6;

pub const LSFW_LOCK: UINT = 1;
pub const LSFW_UNLOCK: UINT = 2;

pub const MDITILE_VERTICAL: UINT = 0x0000;
pub const MDITILE_HORIZONTAL: UINT = 0x0001;
pub const MDITILE_SKIPDISABLED: UINT = 0x0002;
pub const MDITILE_ZORDER: UINT = 0x0004;

pub const MB_OK: DWORD = 0x00000000;
pub const MB_OKCANCEL: DWORD = 0x00000001;
pub const MB_ABORTRETRYIGNORE: DWORD = 0x00000002;
pub const MB_YESNOCANCEL: DWORD = 0x00000003;
pub const MB_YESNO: DWORD = 0x00000004;
pub const MB_RETRYCANCEL: DWORD = 0x00000005;
pub const MB_CANCELTRYCONTINUE: DWORD = 0x00000006;
pub const MB_ICONHAND: DWORD = 0x00000010;
pub const MB_ICONQUESTION: DWORD = 0x00000020;
pub const MB_ICONEXCLAMATION: DWORD = 0x00000030;
pub const MB_ICONASTERISK: DWORD = 0x00000040;
pub const MB_USERICON: DWORD = 0x00000080;
pub const MB_ICONWARNING: DWORD = MB_ICONEXCLAMATION;
pub const MB_ICONERROR: DWORD = MB_ICONHAND;
pub const MB_ICONINFORMATION: DWORD = MB_ICONASTERISK;
pub const MB_ICONSTOP: DWORD = MB_ICONHAND;
pub const MB_DEFBUTTON1: DWORD = 0x00000000;
pub const MB_DEFBUTTON2: DWORD = 0x00000100;
pub const MB_DEFBUTTON3: DWORD = 0x00000200;
pub const MB_DEFBUTTON4: DWORD = 0x00000300;
pub const MB_APPLMODAL: DWORD = 0x00000000;
pub const MB_SYSTEMMODAL: DWORD = 0x00001000;
pub const MB_TASKMODAL: DWORD = 0x00002000;
pub const MB_HELP: DWORD = 0x00004000;
pub const MB_NOFOCUS: DWORD = 0x00008000;
pub const MB_SETFOREGROUND: DWORD = 0x00010000;
pub const MB_DEFAULT_DESKTOP_ONLY: DWORD = 0x00020000;
pub const MB_TOPMOST: DWORD = 0x00040000;
pub const MB_RIGHT: DWORD = 0x00080000;
pub const MB_RTLREADING: DWORD = 0x00100000;
pub const MB_SERVICE_NOTIFICATION: DWORD = 0x00200000;
pub const MB_SERVICE_NOTIFICATION_NT3X: DWORD = 0x00040000;
pub const MB_TYPEMASK: DWORD = 0x0000000F;
pub const MB_ICONMASK: DWORD = 0x000000F0;
pub const MB_DEFMASK: DWORD = 0x00000F00;
pub const MB_MODEMASK: DWORD = 0x00003000;
pub const MB_MISCMASK: DWORD = 0x0000C000;

pub const SB_HORZ: c_int = 0;
pub const SB_VERT: c_int = 1;
pub const SB_CTL: c_int = 2;
pub const SB_BOTH: c_int = 3;

pub const SW_HIDE: c_int = 0;
pub const SW_SHOWNORMAL: c_int = 1;
pub const SW_NORMAL: c_int = 1;
pub const SW_SHOWMINIMIZED: c_int = 2;
pub const SW_SHOWMAXIMIZED: c_int = 3;
pub const SW_MAXIMIZE: c_int = 3;
pub const SW_SHOWNOACTIVATE: c_int = 4;
pub const SW_SHOW: c_int = 5;
pub const SW_MINIMIZE: c_int = 6;
pub const SW_SHOWMINNOACTIVE: c_int = 7;
pub const SW_SHOWNA: c_int = 8;
pub const SW_RESTORE: c_int = 9;
pub const SW_SHOWDEFAULT: c_int = 10;
pub const SW_FORCEMINIMIZE: c_int = 11;
pub const SW_MAX: c_int = 11;

pub const SWP_NOSIZE: UINT = 0x0001;
pub const SWP_NOMOVE: UINT = 0x0002;
pub const SWP_NOZORDER: UINT = 0x0004;
pub const SWP_NOREDRAW: UINT = 0x0008;
pub const SWP_NOACTIVATE: UINT = 0x0010;
pub const SWP_FRAMECHANGED: UINT = 0x0020;
pub const SWP_SHOWWINDOW: UINT = 0x0040;
pub const SWP_HIDEWINDOW: UINT = 0x0080;
pub const SWP_NOCOPYBITS: UINT = 0x0100;
pub const SWP_NOOWNERZORDER: UINT = 0x0200;
pub const SWP_NOSENDCHANGING: UINT = 0x0400;
pub const SWP_DRAWFRAME: UINT = SWP_FRAMECHANGED;
pub const SWP_NOREPOSITION: UINT = SWP_NOOWNERZORDER;
pub const SWP_DEFERERASE: UINT = 0x2000;
pub const SWP_ASYNCWINDOWPOS: UINT = 0x4000;

pub const VK_LBUTTON: WPARAM = 0x01;
pub const VK_RBUTTON: WPARAM = 0x02;
pub const VK_CANCEL: WPARAM = 0x03;
pub const VK_MBUTTON: WPARAM = 0x04;
pub const VK_XBUTTON1: WPARAM = 0x05;
pub const VK_XBUTTON2: WPARAM = 0x06;
pub const VK_BACK: WPARAM = 0x08;
pub const VK_TAB: WPARAM = 0x09;
pub const VK_CLEAR: WPARAM = 0x0C;
pub const VK_RETURN: WPARAM = 0x0D;
pub const VK_SHIFT: WPARAM = 0x10;
pub const VK_CONTROL: WPARAM = 0x11;
pub const VK_MENU: WPARAM = 0x12;
pub const VK_PAUSE: WPARAM = 0x13;
pub const VK_CAPITAL: WPARAM = 0x14;
pub const VK_KANA: WPARAM = 0x15;
pub const VK_HANGUEL: WPARAM = 0x15;
pub const VK_HANGUL: WPARAM = 0x15;
pub const VK_JUNJA: WPARAM = 0x17;
pub const VK_FINAL: WPARAM = 0x18;
pub const VK_HANJA: WPARAM = 0x19;
pub const VK_KANJI: WPARAM = 0x19;
pub const VK_ESCAPE: WPARAM = 0x1B;
pub const VK_CONVERT: WPARAM = 0x1C;
pub const VK_NONCONVERT: WPARAM = 0x1D;
pub const VK_ACCEPT: WPARAM = 0x1E;
pub const VK_MODECHANGE: WPARAM = 0x1F;
pub const VK_SPACE: WPARAM = 0x20;
pub const VK_PRIOR: WPARAM = 0x21;
pub const VK_NEXT: WPARAM = 0x22;
pub const VK_END: WPARAM = 0x23;
pub const VK_HOME: WPARAM = 0x24;
pub const VK_LEFT: WPARAM = 0x25;
pub const VK_UP: WPARAM = 0x26;
pub const VK_RIGHT: WPARAM = 0x27;
pub const VK_DOWN: WPARAM = 0x28;
pub const VK_SELECT: WPARAM = 0x29;
pub const VK_PRINT: WPARAM = 0x2A;
pub const VK_EXECUTE: WPARAM = 0x2B;
pub const VK_SNAPSHOT: WPARAM = 0x2C;
pub const VK_INSERT: WPARAM = 0x2D;
pub const VK_DELETE: WPARAM = 0x2E;
pub const VK_HELP: WPARAM = 0x2F;
pub const VK_LWIN: WPARAM = 0x5B;
pub const VK_RWIN: WPARAM = 0x5C;
pub const VK_APPS: WPARAM = 0x5D;
pub const VK_SLEEP: WPARAM = 0x5F;
pub const VK_NUMPAD0: WPARAM = 0x60;
pub const VK_NUMPAD1: WPARAM = 0x61;
pub const VK_NUMPAD2: WPARAM = 0x62;
pub const VK_NUMPAD3: WPARAM = 0x63;
pub const VK_NUMPAD4: WPARAM = 0x64;
pub const VK_NUMPAD5: WPARAM = 0x65;
pub const VK_NUMPAD6: WPARAM = 0x66;
pub const VK_NUMPAD7: WPARAM = 0x67;
pub const VK_NUMPAD8: WPARAM = 0x68;
pub const VK_NUMPAD9: WPARAM = 0x69;
pub const VK_MULTIPLY: WPARAM = 0x6A;
pub const VK_ADD: WPARAM = 0x6B;
pub const VK_SEPARATOR: WPARAM = 0x6C;
pub const VK_SUBTRACT: WPARAM = 0x6D;
pub const VK_DECIMAL: WPARAM = 0x6E;
pub const VK_DIVIDE: WPARAM = 0x6F;
pub const VK_F1: WPARAM = 0x70;
pub const VK_F2: WPARAM = 0x71;
pub const VK_F3: WPARAM = 0x72;
pub const VK_F4: WPARAM = 0x73;
pub const VK_F5: WPARAM = 0x74;
pub const VK_F6: WPARAM = 0x75;
pub const VK_F7: WPARAM = 0x76;
pub const VK_F8: WPARAM = 0x77;
pub const VK_F9: WPARAM = 0x78;
pub const VK_F10: WPARAM = 0x79;
pub const VK_F11: WPARAM = 0x7A;
pub const VK_F12: WPARAM = 0x7B;
pub const VK_F13: WPARAM = 0x7C;
pub const VK_F14: WPARAM = 0x7D;
pub const VK_F15: WPARAM = 0x7E;
pub const VK_F16: WPARAM = 0x7F;
pub const VK_F17: WPARAM = 0x80;
pub const VK_F18: WPARAM = 0x81;
pub const VK_F19: WPARAM = 0x82;
pub const VK_F20: WPARAM = 0x83;
pub const VK_F21: WPARAM = 0x84;
pub const VK_F22: WPARAM = 0x85;
pub const VK_F23: WPARAM = 0x86;
pub const VK_F24: WPARAM = 0x87;
pub const VK_NUMLOCK: WPARAM = 0x90;
pub const VK_SCROLL: WPARAM = 0x91;
pub const VK_OEM_NEC_EQUAL: WPARAM = 0x92;
pub const VK_OEM_FJ_JISHO: WPARAM = 0x92;
pub const VK_OEM_FJ_MASSHOU: WPARAM = 0x93;
pub const VK_OEM_FJ_TOUROKU: WPARAM = 0x94;
pub const VK_OEM_FJ_LOYA: WPARAM = 0x95;
pub const VK_OEM_FJ_ROYA: WPARAM = 0x96;
pub const VK_LSHIFT: WPARAM = 0xA0;
pub const VK_RSHIFT: WPARAM = 0xA1;
pub const VK_LCONTROL: WPARAM = 0xA2;
pub const VK_RCONTROL: WPARAM = 0xA3;
pub const VK_LMENU: WPARAM = 0xA4;
pub const VK_RMENU: WPARAM = 0xA5;
pub const VK_BROWSER_BACK: WPARAM = 0xA6;
pub const VK_BROWSER_FORWARD: WPARAM = 0xA7;
pub const VK_BROWSER_REFRESH: WPARAM = 0xA8;
pub const VK_BROWSER_STOP: WPARAM = 0xA9;
pub const VK_BROWSER_SEARCH: WPARAM = 0xAA;
pub const VK_BROWSER_FAVORITES: WPARAM = 0xAB;
pub const VK_BROWSER_HOME: WPARAM = 0xAC;
pub const VK_VOLUME_MUTE: WPARAM = 0xAD;
pub const VK_VOLUME_DOWN: WPARAM = 0xAE;
pub const VK_VOLUME_UP: WPARAM = 0xAF;
pub const VK_MEDIA_NEXT_TRACK: WPARAM = 0xB0;
pub const VK_MEDIA_PREV_TRACK: WPARAM = 0xB1;
pub const VK_MEDIA_STOP: WPARAM = 0xB2;
pub const VK_MEDIA_PLAY_PAUSE: WPARAM = 0xB3;
pub const VK_LAUNCH_MAIL: WPARAM = 0xB4;
pub const VK_LAUNCH_MEDIA_SELECT: WPARAM = 0xB5;
pub const VK_LAUNCH_APP1: WPARAM = 0xB6;
pub const VK_LAUNCH_APP2: WPARAM = 0xB7;
pub const VK_OEM_1: WPARAM = 0xBA;
pub const VK_OEM_PLUS: WPARAM = 0xBB;
pub const VK_OEM_COMMA: WPARAM = 0xBC;
pub const VK_OEM_MINUS: WPARAM = 0xBD;
pub const VK_OEM_PERIOD: WPARAM = 0xBE;
pub const VK_OEM_2: WPARAM = 0xBF;
pub const VK_OEM_3: WPARAM = 0xC0;
pub const VK_OEM_4: WPARAM = 0xDB;
pub const VK_OEM_5: WPARAM = 0xDC;
pub const VK_OEM_6: WPARAM = 0xDD;
pub const VK_OEM_7: WPARAM = 0xDE;
pub const VK_OEM_8: WPARAM = 0xDF;
pub const VK_OEM_AX: WPARAM = 0xE1;
pub const VK_OEM_102: WPARAM = 0xE2;
pub const VK_ICO_HELP: WPARAM = 0xE3;
pub const VK_ICO_00: WPARAM = 0xE4;
pub const VK_PROCESSKEY: WPARAM = 0xE5;
pub const VK_ICO_CLEAR: WPARAM = 0xE6;
pub const VK_PACKET: WPARAM = 0xE7;
pub const VK_OEM_RESET: WPARAM = 0xE9;
pub const VK_OEM_JUMP: WPARAM = 0xEA;
pub const VK_OEM_PA1: WPARAM = 0xEB;
pub const VK_OEM_PA2: WPARAM = 0xEC;
pub const VK_OEM_PA3: WPARAM = 0xED;
pub const VK_OEM_WSCTRL: WPARAM = 0xEE;
pub const VK_OEM_CUSEL: WPARAM = 0xEF;
pub const VK_OEM_ATTN: WPARAM = 0xF0;
pub const VK_OEM_FINISH: WPARAM = 0xF1;
pub const VK_OEM_COPY: WPARAM = 0xF2;
pub const VK_OEM_AUTO: WPARAM = 0xF3;
pub const VK_OEM_ENLW: WPARAM = 0xF4;
pub const VK_OEM_BACKTAB: WPARAM = 0xF5;
pub const VK_ATTN: WPARAM = 0xF6;
pub const VK_CRSEL: WPARAM = 0xF7;
pub const VK_EXSEL: WPARAM = 0xF8;
pub const VK_EREOF: WPARAM = 0xF9;
pub const VK_PLAY: WPARAM = 0xFA;
pub const VK_ZOOM: WPARAM = 0xFB;
pub const VK_NONAME: WPARAM = 0xFC;
pub const VK_PA1: WPARAM = 0xFD;
pub const VK_OEM_CLEAR: WPARAM = 0xFE;

pub const WM_NULL: UINT = 0x0000;
pub const WM_CREATE: UINT = 0x0001;
pub const WM_DESTROY: UINT = 0x0002;
pub const WM_MOVE: UINT = 0x0003;
pub const WM_SIZE: UINT = 0x0005;
pub const WM_ACTIVATE: UINT = 0x0006;
pub const WM_SETFOCUS: UINT = 0x0007;
pub const WM_KILLFOCUS: UINT = 0x0008;
pub const WM_ENABLE: UINT = 0x000A;
pub const WM_SETREDRAW: UINT = 0x000B;
pub const WM_SETTEXT: UINT = 0x000C;
pub const WM_GETTEXT: UINT = 0x000D;
pub const WM_GETTEXTLENGTH: UINT = 0x000E;
pub const WM_PAINT: UINT = 0x000F;
pub const WM_CLOSE: UINT = 0x0010;
pub const WM_QUERYENDSESSION: UINT = 0x0011;
pub const WM_QUERYOPEN: UINT = 0x0013;
pub const WM_ENDSESSION: UINT = 0x0016;
pub const WM_QUIT: UINT = 0x0012;
pub const WM_ERASEBKGND: UINT = 0x0014;
pub const WM_SYSCOLORCHANGE: UINT = 0x0015;
pub const WM_SHOWWINDOW: UINT = 0x0018;
pub const WM_WININICHANGE: UINT = 0x001A;
pub const WM_SETTINGCHANGE: UINT = WM_WININICHANGE;
pub const WM_DEVMODECHANGE: UINT = 0x001B;
pub const WM_ACTIVATEAPP: UINT = 0x001C;
pub const WM_FONTCHANGE: UINT = 0x001D;
pub const WM_TIMECHANGE: UINT = 0x001E;
pub const WM_CANCELMODE: UINT = 0x001F;
pub const WM_SETCURSOR: UINT = 0x0020;
pub const WM_MOUSEACTIVATE: UINT = 0x0021;
pub const WM_CHILDACTIVATE: UINT = 0x0022;
pub const WM_QUEUESYNC: UINT = 0x0023;
pub const WM_GETMINMAXINFO: UINT = 0x0024;
pub const WM_PAINTICON: UINT = 0x0026;
pub const WM_ICONERASEBKGND: UINT = 0x0027;
pub const WM_NEXTDLGCTL: UINT = 0x0028;
pub const WM_SPOOLERSTATUS: UINT = 0x002A;
pub const WM_DRAWITEM: UINT = 0x002B;
pub const WM_MEASUREITEM: UINT = 0x002C;
pub const WM_DELETEITEM: UINT = 0x002D;
pub const WM_VKEYTOITEM: UINT = 0x002E;
pub const WM_CHARTOITEM: UINT = 0x002F;
pub const WM_SETFONT: UINT = 0x0030;
pub const WM_GETFONT: UINT = 0x0031;
pub const WM_SETHOTKEY: UINT = 0x0032;
pub const WM_GETHOTKEY: UINT = 0x0033;
pub const WM_QUERYDRAGICON: UINT = 0x0037;
pub const WM_COMPAREITEM: UINT = 0x0039;
pub const WM_GETOBJECT: UINT = 0x003D;
pub const WM_COMPACTING: UINT = 0x0041;
pub const WM_COMMNOTIFY: UINT = 0x0044;
pub const WM_WINDOWPOSCHANGING: UINT = 0x0046;
pub const WM_WINDOWPOSCHANGED: UINT = 0x0047;
pub const WM_POWER: UINT = 0x0048;
pub const WM_COPYDATA: UINT = 0x004A;
pub const WM_CANCELJOURNAL: UINT = 0x004B;
pub const WM_NOTIFY: UINT = 0x004E;
pub const WM_INPUTLANGCHANGEREQUEST: UINT = 0x0050;
pub const WM_INPUTLANGCHANGE: UINT = 0x0051;
pub const WM_TCARD: UINT = 0x0052;
pub const WM_HELP: UINT = 0x0053;
pub const WM_USERCHANGED: UINT = 0x0054;
pub const WM_NOTIFYFORMAT: UINT = 0x0055;
pub const WM_CONTEXTMENU: UINT = 0x007B;
pub const WM_STYLECHANGING: UINT = 0x007C;
pub const WM_STYLECHANGED: UINT = 0x007D;
pub const WM_DISPLAYCHANGE: UINT = 0x007E;
pub const WM_GETICON: UINT = 0x007F;
pub const WM_SETICON: UINT = 0x0080;
pub const WM_NCCREATE: UINT = 0x0081;
pub const WM_NCDESTROY: UINT = 0x0082;
pub const WM_NCCALCSIZE: UINT = 0x0083;
pub const WM_NCHITTEST: UINT = 0x0084;
pub const WM_NCPAINT: UINT = 0x0085;
pub const WM_NCACTIVATE: UINT = 0x0086;
pub const WM_GETDLGCODE: UINT = 0x0087;
pub const WM_SYNCPAINT: UINT = 0x0088;
pub const WM_NCMOUSEMOVE: UINT = 0x00A0;
pub const WM_NCLBUTTONDOWN: UINT = 0x00A1;
pub const WM_NCLBUTTONUP: UINT = 0x00A2;
pub const WM_NCLBUTTONDBLCLK: UINT = 0x00A3;
pub const WM_NCRBUTTONDOWN: UINT = 0x00A4;
pub const WM_NCRBUTTONUP: UINT = 0x00A5;
pub const WM_NCRBUTTONDBLCLK: UINT = 0x00A6;
pub const WM_NCMBUTTONDOWN: UINT = 0x00A7;
pub const WM_NCMBUTTONUP: UINT = 0x00A8;
pub const WM_NCMBUTTONDBLCLK: UINT = 0x00A9;
pub const WM_NCXBUTTONDOWN: UINT = 0x00AB;
pub const WM_NCXBUTTONUP: UINT = 0x00AC;
pub const WM_NCXBUTTONDBLCLK: UINT = 0x00AD;
pub const WM_INPUT_DEVICE_CHANGE: UINT = 0x00FE;
pub const WM_INPUT: UINT = 0x00FF;
pub const WM_KEYFIRST: UINT = 0x0100;
pub const WM_KEYDOWN: UINT = 0x0100;
pub const WM_KEYUP: UINT = 0x0101;
pub const WM_CHAR: UINT = 0x0102;
pub const WM_DEADCHAR: UINT = 0x0103;
pub const WM_SYSKEYDOWN: UINT = 0x0104;
pub const WM_SYSKEYUP: UINT = 0x0105;
pub const WM_SYSCHAR: UINT = 0x0106;
pub const WM_SYSDEADCHAR: UINT = 0x0107;
pub const WM_UNICHAR: UINT = 0x0109;
pub const WM_KEYLAST: UINT = 0x0109;
pub const WM_IME_STARTCOMPOSITION: UINT = 0x010D;
pub const WM_IME_ENDCOMPOSITION: UINT = 0x010E;
pub const WM_IME_COMPOSITION: UINT = 0x010F;
pub const WM_IME_KEYLAST: UINT = 0x010F;
pub const WM_INITDIALOG: UINT = 0x0110;
pub const WM_COMMAND: UINT = 0x0111;
pub const WM_SYSCOMMAND: UINT = 0x0112;
pub const WM_TIMER: UINT = 0x0113;
pub const WM_HSCROLL: UINT = 0x0114;
pub const WM_VSCROLL: UINT = 0x0115;
pub const WM_INITMENU: UINT = 0x0116;
pub const WM_INITMENUPOPUP: UINT = 0x0117;
pub const WM_GESTURE: UINT = 0x0119;
pub const WM_GESTURENOTIFY: UINT = 0x011A;
pub const WM_MENUSELECT: UINT = 0x011F;
pub const WM_MENUCHAR: UINT = 0x0120;
pub const WM_ENTERIDLE: UINT = 0x0121;
pub const WM_MENURBUTTONUP: UINT = 0x0122;
pub const WM_MENUDRAG: UINT = 0x0123;
pub const WM_MENUGETOBJECT: UINT = 0x0124;
pub const WM_UNINITMENUPOPUP: UINT = 0x0125;
pub const WM_MENUCOMMAND: UINT = 0x0126;
pub const WM_CHANGEUISTATE: UINT = 0x0127;
pub const WM_UPDATEUISTATE: UINT = 0x0128;
pub const WM_QUERYUISTATE: UINT = 0x0129;
pub const WM_CTLCOLORMSGBOX: UINT = 0x0132;
pub const WM_CTLCOLOREDIT: UINT = 0x0133;
pub const WM_CTLCOLORLISTBOX: UINT = 0x0134;
pub const WM_CTLCOLORBTN: UINT = 0x0135;
pub const WM_CTLCOLORDLG: UINT = 0x0136;
pub const WM_CTLCOLORSCROLLBAR: UINT = 0x0137;
pub const WM_CTLCOLORSTATIC: UINT = 0x0138;
pub const WM_MOUSEFIRST: UINT = 0x0200;
pub const WM_MOUSEMOVE: UINT = 0x0200;
pub const WM_LBUTTONDOWN: UINT = 0x0201;
pub const WM_LBUTTONUP: UINT = 0x0202;
pub const WM_LBUTTONDBLCLK: UINT = 0x0203;
pub const WM_RBUTTONDOWN: UINT = 0x0204;
pub const WM_RBUTTONUP: UINT = 0x0205;
pub const WM_RBUTTONDBLCLK: UINT = 0x0206;
pub const WM_MBUTTONDOWN: UINT = 0x0207;
pub const WM_MBUTTONUP: UINT = 0x0208;
pub const WM_MBUTTONDBLCLK: UINT = 0x0209;
pub const WM_MOUSEWHEEL: UINT = 0x020A;
pub const WM_XBUTTONDOWN: UINT = 0x020B;
pub const WM_XBUTTONUP: UINT = 0x020C;
pub const WM_XBUTTONDBLCLK: UINT = 0x020D;
pub const WM_MOUSEHWHEEL: UINT = 0x020E;
pub const WM_MOUSELAST: UINT = 0x020E;
pub const WM_PARENTNOTIFY: UINT = 0x0210;
pub const WM_ENTERMENULOOP: UINT = 0x0211;
pub const WM_EXITMENULOOP: UINT = 0x0212;
pub const WM_NEXTMENU: UINT = 0x0213;
pub const WM_SIZING: UINT = 0x0214;
pub const WM_CAPTURECHANGED: UINT = 0x0215;
pub const WM_MOVING: UINT = 0x0216;
pub const WM_POWERBROADCAST: UINT = 0x0218;
pub const WM_DEVICECHANGE: UINT = 0x0219;
pub const WM_MDICREATE: UINT = 0x0220;
pub const WM_MDIDESTROY: UINT = 0x0221;
pub const WM_MDIACTIVATE: UINT = 0x0222;
pub const WM_MDIRESTORE: UINT = 0x0223;
pub const WM_MDINEXT: UINT = 0x0224;
pub const WM_MDIMAXIMIZE: UINT = 0x0225;
pub const WM_MDITILE: UINT = 0x0226;
pub const WM_MDICASCADE: UINT = 0x0227;
pub const WM_MDIICONARRANGE: UINT = 0x0228;
pub const WM_MDIGETACTIVE: UINT = 0x0229;
pub const WM_MDISETMENU: UINT = 0x0230;
pub const WM_ENTERSIZEMOVE: UINT = 0x0231;
pub const WM_EXITSIZEMOVE: UINT = 0x0232;
pub const WM_DROPFILES: UINT = 0x0233;
pub const WM_MDIREFRESHMENU: UINT = 0x0234;
pub const WM_POINTERDEVICECHANGE: UINT = 0x238;
pub const WM_POINTERDEVICEINRANGE: UINT = 0x239;
pub const WM_POINTERDEVICEOUTOFRANGE: UINT = 0x23A;
pub const WM_TOUCH: UINT = 0x0240;
pub const WM_NCPOINTERUPDATE: UINT = 0x0241;
pub const WM_NCPOINTERDOWN: UINT = 0x0242;
pub const WM_NCPOINTERUP: UINT = 0x0243;
pub const WM_POINTERUPDATE: UINT = 0x0245;
pub const WM_POINTERDOWN: UINT = 0x0246;
pub const WM_POINTERUP: UINT = 0x0247;
pub const WM_POINTERENTER: UINT = 0x0249;
pub const WM_POINTERLEAVE: UINT = 0x024A;
pub const WM_POINTERACTIVATE: UINT = 0x024B;
pub const WM_POINTERCAPTURECHANGED: UINT = 0x024C;
pub const WM_TOUCHHITTESTING: UINT = 0x024D;
pub const WM_POINTERWHEEL: UINT = 0x024E;
pub const WM_POINTERHWHEEL: UINT = 0x024F;
pub const WM_IME_SETCONTEXT: UINT = 0x0281;
pub const WM_IME_NOTIFY: UINT = 0x0282;
pub const WM_IME_CONTROL: UINT = 0x0283;
pub const WM_IME_COMPOSITIONFULL: UINT = 0x0284;
pub const WM_IME_SELECT: UINT = 0x0285;
pub const WM_IME_CHAR: UINT = 0x0286;
pub const WM_IME_REQUEST: UINT = 0x0288;
pub const WM_IME_KEYDOWN: UINT = 0x0290;
pub const WM_IME_KEYUP: UINT = 0x0291;
pub const WM_MOUSEHOVER: UINT = 0x02A1;
pub const WM_MOUSELEAVE: UINT = 0x02A3;
pub const WM_NCMOUSEHOVER: UINT = 0x02A0;
pub const WM_NCMOUSELEAVE: UINT = 0x02A2;
pub const WM_WTSSESSION_CHANGE: UINT = 0x02B1;
pub const WM_TABLET_FIRST: UINT = 0x02c0;
pub const WM_TABLET_LAST: UINT = 0x02df;
pub const WM_DPICHANGED: UINT = 0x02E0;
pub const WM_CUT: UINT = 0x0300;
pub const WM_COPY: UINT = 0x0301;
pub const WM_PASTE: UINT = 0x0302;
pub const WM_CLEAR: UINT = 0x0303;
pub const WM_UNDO: UINT = 0x0304;
pub const WM_RENDERFORMAT: UINT = 0x0305;
pub const WM_RENDERALLFORMATS: UINT = 0x0306;
pub const WM_DESTROYCLIPBOARD: UINT = 0x0307;
pub const WM_DRAWCLIPBOARD: UINT = 0x0308;
pub const WM_PAINTCLIPBOARD: UINT = 0x0309;
pub const WM_VSCROLLCLIPBOARD: UINT = 0x030A;
pub const WM_SIZECLIPBOARD: UINT = 0x030B;
pub const WM_ASKCBFORMATNAME: UINT = 0x030C;
pub const WM_CHANGECBCHAIN: UINT = 0x030D;
pub const WM_HSCROLLCLIPBOARD: UINT = 0x030E;
pub const WM_QUERYNEWPALETTE: UINT = 0x030F;
pub const WM_PALETTEISCHANGING: UINT = 0x0310;
pub const WM_PALETTECHANGED: UINT = 0x0311;
pub const WM_HOTKEY: UINT = 0x0312;
pub const WM_PRINT: UINT = 0x0317;
pub const WM_PRINTCLIENT: UINT = 0x0318;
pub const WM_APPCOMMAND: UINT = 0x0319;
pub const WM_THEMECHANGED: UINT = 0x031A;
pub const WM_CLIPBOARDUPDATE: UINT = 0x031D;
pub const WM_DWMCOMPOSITIONCHANGED: UINT = 0x031E;
pub const WM_DWMNCRENDERINGCHANGED: UINT = 0x031F;
pub const WM_DWMCOLORIZATIONCOLORCHANGED: UINT = 0x0320;
pub const WM_DWMWINDOWMAXIMIZEDCHANGE: UINT = 0x0321;
pub const WM_DWMSENDICONICTHUMBNAIL: UINT = 0x0323;
pub const WM_DWMSENDICONICLIVEPREVIEWBITMAP: UINT = 0x0326;
pub const WM_GETTITLEBARINFOEX: UINT = 0x033F;
pub const WM_HANDHELDFIRST: UINT = 0x0358;
pub const WM_HANDHELDLAST: UINT = 0x035F;
pub const WM_AFXFIRST: UINT = 0x0360;
pub const WM_AFXLAST: UINT = 0x037F;
pub const WM_PENWINFIRST: UINT = 0x0380;
pub const WM_PENWINLAST: UINT = 0x038F;
pub const WM_APP: UINT = 0x8000;
pub const WM_USER: UINT = 0x0400;

pub const WS_OVERLAPPED: DWORD = 0x00000000;
pub const WS_POPUP: DWORD = 0x80000000;
pub const WS_CHILD: DWORD = 0x40000000;
pub const WS_MINIMIZE: DWORD = 0x20000000;
pub const WS_VISIBLE: DWORD = 0x10000000;
pub const WS_DISABLED: DWORD = 0x08000000;
pub const WS_CLIPSIBLINGS: DWORD = 0x04000000;
pub const WS_CLIPCHILDREN: DWORD = 0x02000000;
pub const WS_MAXIMIZE: DWORD = 0x01000000;
pub const WS_CAPTION: DWORD = 0x00C00000;
pub const WS_BORDER: DWORD = 0x00800000;
pub const WS_DLGFRAME: DWORD = 0x00400000;
pub const WS_VSCROLL: DWORD = 0x00200000;
pub const WS_HSCROLL: DWORD = 0x00100000;
pub const WS_SYSMENU: DWORD = 0x00080000;
pub const WS_THICKFRAME: DWORD = 0x00040000;
pub const WS_GROUP: DWORD = 0x00020000;
pub const WS_TABSTOP: DWORD = 0x00010000;
pub const WS_MINIMIZEBOX: DWORD = 0x00020000;
pub const WS_MAXIMIZEBOX: DWORD = 0x00010000;
pub const WS_TILED: DWORD = WS_OVERLAPPED;
pub const WS_ICONIC: DWORD = WS_MINIMIZE;
pub const WS_SIZEBOX: DWORD = WS_THICKFRAME;
pub const WS_TILEDWINDOW: DWORD = WS_OVERLAPPEDWINDOW;
pub const WS_OVERLAPPEDWINDOW: DWORD = WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_THICKFRAME | WS_MINIMIZEBOX | WS_MAXIMIZEBOX;
pub const WS_POPUPWINDOW: DWORD = WS_POPUP | WS_BORDER | WS_SYSMENU;
pub const WS_CHILDWINDOW: DWORD = WS_CHILD;

pub const WS_EX_DLGMODALFRAME: DWORD = 0x00000001;
pub const WS_EX_NOPARENTNOTIFY: DWORD = 0x00000004;
pub const WS_EX_TOPMOST: DWORD = 0x00000008;
pub const WS_EX_ACCEPTFILES: DWORD = 0x00000010;
pub const WS_EX_TRANSPARENT: DWORD = 0x00000020;
pub const WS_EX_MDICHILD: DWORD = 0x00000040;
pub const WS_EX_TOOLWINDOW: DWORD = 0x00000080;
pub const WS_EX_WINDOWEDGE: DWORD = 0x00000100;
pub const WS_EX_CLIENTEDGE: DWORD = 0x00000200;
pub const WS_EX_CONTEXTHELP: DWORD = 0x00000400;
pub const WS_EX_RIGHT: DWORD = 0x00001000;
pub const WS_EX_LEFT: DWORD = 0x00000000;
pub const WS_EX_RTLREADING: DWORD = 0x00002000;
pub const WS_EX_LTRREADING: DWORD = 0x00000000;
pub const WS_EX_LEFTSCROLLBAR: DWORD = 0x00004000;
pub const WS_EX_RIGHTSCROLLBAR: DWORD = 0x00000000;
pub const WS_EX_CONTROLPARENT: DWORD = 0x00010000;
pub const WS_EX_STATICEDGE: DWORD = 0x00020000;
pub const WS_EX_APPWINDOW: DWORD = 0x00040000;
pub const WS_EX_OVERLAPPEDWINDOW: DWORD = WS_EX_WINDOWEDGE | WS_EX_CLIENTEDGE;
pub const WS_EX_PALETTEWINDOW: DWORD = WS_EX_WINDOWEDGE | WS_EX_TOOLWINDOW | WS_EX_TOPMOST;
pub const WS_EX_LAYERED: DWORD = 0x00080000;
pub const WS_EX_NOINHERITLAYOUT: DWORD = 0x00100000;
pub const WS_EX_NOREDIRECTIONBITMAP: DWORD = 0x00200000;
pub const WS_EX_LAYOUTRTL: DWORD = 0x00400000;
pub const WS_EX_COMPOSITED: DWORD = 0x02000000;
pub const WS_EX_NOACTIVATE: DWORD = 0x08000000;
pub type DESKTOPENUMPROCA = Option<unsafe extern "system" fn(LPSTR, LPARAM) -> BOOL>;
pub type DESKTOPENUMPROCW = Option<unsafe extern "system" fn(LPWSTR, LPARAM) -> BOOL>;
pub type NAMEENUMPROCA = DESKTOPENUMPROCA;
pub type NAMEENUMPROCW = DESKTOPENUMPROCW;
pub type WNDENUMPROC = Option<unsafe extern "system" fn(HWND, LPARAM) -> BOOL>;
pub type WNDPROC = Option<unsafe extern "system" fn(HWND, UINT, WPARAM, LPARAM) -> LRESULT>;
pub type HOOKPROC = Option<unsafe extern "system" fn(
    code: c_int, wParam: WPARAM, lParam: LPARAM,
) -> LRESULT>;
pub type TimerProc = Option<unsafe extern "system" fn(
    hwnd: HWND, uMsg: UINT, idEvent: UINT_PTR, dwTime: DWORD,
)>;

pub type HDEVNOTIFY = PVOID;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct MSG {
    pub hwnd: HWND,
    pub message: UINT,
    pub wParam: WPARAM,
    pub lParam: LPARAM,
    pub time: DWORD,
    pub pt: POINT,
}
pub type PMSG = *mut MSG;
pub type NPMSG = *mut MSG;
pub type LPMSG = *mut MSG;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct PAINTSTRUCT {
    pub hdc: HDC,
    pub fErase: BOOL,
    pub rcPaint: RECT,
    pub fRestore: BOOL,
    pub fIncUpdate: BOOL,
    pub rgbReserved: [BYTE; 32],
}
pub type PPAINTSTRUCT = *mut PAINTSTRUCT;
pub type NPPAINTSTRUCT = *mut PAINTSTRUCT;
pub type LPPAINTSTRUCT = *mut PAINTSTRUCT;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct WINDOWPLACEMENT {
    pub length: UINT,
    pub flags: UINT,
    pub showCmd: UINT,
    pub ptMinPosition: POINT,
    pub ptMaxPosition: POINT,
    pub rcNormalPosition: RECT,
}
pub type PWINDOWPLACEMENT = *mut WINDOWPLACEMENT;
pub type LPWINDOWPLACEMENT = *mut WINDOWPLACEMENT;
#[repr(C)] #[derive(Copy)]
pub struct WNDCLASSEXW {
    pub cbSize: UINT,
    pub style: UINT,
    pub lpfnWndProc: WNDPROC,
    pub cbClsExtra: c_int,
    pub cbWndExtra: c_int,
    pub hInstance: HINSTANCE,
    pub hIcon: HICON,
    pub hCursor: HCURSOR,
    pub hbrBackground: HBRUSH,
    pub lpszMenuName: LPCWSTR,
    pub lpszClassName: LPCWSTR,
    pub hIconSm: HICON,
}
impl Clone for WNDCLASSEXW { fn clone(&self) -> WNDCLASSEXW { *self } }
pub type PWNDCLASSEXW = *mut WNDCLASSEXW;
pub type NPWNDCLASSEXW = *mut WNDCLASSEXW;
pub type LPWNDCLASSEXW = *mut WNDCLASSEXW;
#[repr(C)] #[derive(Copy)]
pub struct WNDCLASSW {
    pub style: UINT,
    pub lpfnWndProc: WNDPROC,
    pub cbClsExtra: c_int,
    pub cbWndExtra: c_int,
    pub hInstance: HINSTANCE,
    pub hIcon: HICON,
    pub hCursor: HCURSOR,
    pub hbrBackground: HBRUSH,
    pub lpszMenuName: LPCWSTR,
    pub lpszClassName: LPCWSTR
}
impl Clone for WNDCLASSW { fn clone(&self) -> WNDCLASSW { *self } }
pub type PWNDCLASSW = *mut WNDCLASSW;
pub type NPWNDCLASSW = *mut WNDCLASSW;
pub type LPWNDCLASSW = *mut WNDCLASSW;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SCROLLBARINFO {
    pub cbSize: DWORD,
    pub rcScrollBar: RECT,
    pub dxyLineButton: c_int,
    pub xyThumbTop: c_int,
    pub xyThumbBottom: c_int,
    pub reserved: c_int,
    pub rgstate: [DWORD; CCHILDREN_SCROLLBAR + 1]
}
pub type PSCROLLBARINFO = *mut SCROLLBARINFO;
pub type LPSCROLLBARINFO = *mut SCROLLBARINFO;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SCROLLINFO {
    pub cbSize: UINT,
    pub fMask: UINT,
    pub nMin: c_int,
    pub nMax: c_int,
    pub nPage: UINT,
    pub nPos: c_int,
    pub nTrackPos: c_int
}
pub type LPSCROLLINFO = *mut SCROLLINFO;
pub type LPCSCROLLINFO = *const SCROLLINFO;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SIZE {
    pub cx: LONG,
    pub cy: LONG
}
pub type PSIZE = *mut SIZE;

//-------------------------------------------------------------------------------------------------
// wingdi.h
// GDI procedure declarations, constant definitions and macros
//-------------------------------------------------------------------------------------------------
pub const DISPLAY_DEVICE_ATTACHED_TO_DESKTOP: DWORD = 0x00000001;
pub const DISPLAY_DEVICE_MULTI_DRIVER: DWORD = 0x00000002;
pub const DISPLAY_DEVICE_PRIMARY_DEVICE: DWORD = 0x00000004;
pub const DISPLAY_DEVICE_MIRRORING_DRIVER: DWORD = 0x00000008;
pub const DISPLAY_DEVICE_VGA_COMPATIBLE: DWORD = 0x00000010;
pub const DISPLAY_DEVICE_REMOVABLE: DWORD = 0x00000020;
pub const DISPLAY_DEVICE_ACC_DRIVER: DWORD = 0x00000040;
pub const DISPLAY_DEVICE_MODESPRUNED: DWORD = 0x08000000;
pub const DISPLAY_DEVICE_REMOTE: DWORD = 0x04000000;
pub const DISPLAY_DEVICE_DISCONNECT: DWORD = 0x02000000;
pub const DISPLAY_DEVICE_TS_COMPATIBLE: DWORD = 0x00200000;
pub const DISPLAY_DEVICE_UNSAFE_MODES_ON: DWORD = 0x00080000;
pub const DISPLAY_DEVICE_ACTIVE: DWORD = 0x00000001;
pub const DISPLAY_DEVICE_ATTACHED: DWORD = 0x00000002;

pub const DM_ORIENTATION: DWORD = 0x00000001;
pub const DM_PAPERSIZE: DWORD = 0x00000002;
pub const DM_PAPERLENGTH: DWORD = 0x00000004;
pub const DM_PAPERWIDTH: DWORD = 0x00000008;
pub const DM_SCALE: DWORD = 0x00000010;
pub const DM_POSITION: DWORD = 0x00000020;
pub const DM_NUP: DWORD = 0x00000040;
pub const DM_DISPLAYORIENTATION: DWORD = 0x00000080;
pub const DM_COPIES: DWORD = 0x00000100;
pub const DM_DEFAULTSOURCE: DWORD = 0x00000200;
pub const DM_PRINTQUALITY: DWORD = 0x00000400;
pub const DM_COLOR: DWORD = 0x00000800;
pub const DM_DUPLEX: DWORD = 0x00001000;
pub const DM_YRESOLUTION: DWORD = 0x00002000;
pub const DM_TTOPTION: DWORD = 0x00004000;
pub const DM_COLLATE: DWORD = 0x00008000;
pub const DM_FORMNAME: DWORD = 0x00010000;
pub const DM_LOGPIXELS: DWORD = 0x00020000;
pub const DM_BITSPERPEL: DWORD = 0x00040000;
pub const DM_PELSWIDTH: DWORD = 0x00080000;
pub const DM_PELSHEIGHT: DWORD = 0x00100000;
pub const DM_DISPLAYFLAGS: DWORD = 0x00200000;
pub const DM_DISPLAYFREQUENCY: DWORD = 0x00400000;
pub const DM_ICMMETHOD: DWORD = 0x00800000;
pub const DM_ICMINTENT: DWORD = 0x01000000;
pub const DM_MEDIATYPE: DWORD = 0x02000000;
pub const DM_DITHERTYPE: DWORD = 0x04000000;
pub const DM_PANNINGWIDTH: DWORD = 0x08000000;
pub const DM_PANNINGHEIGHT: DWORD = 0x10000000;
pub const DM_DISPLAYFIXEDOUTPUT: DWORD = 0x20000000;

pub const PFD_TYPE_RGBA: BYTE = 0;
pub const PFD_TYPE_COLORINDEX: BYTE = 1;
pub const PFD_MAIN_PLANE: BYTE = 0;
pub const PFD_OVERLAY_PLANE: BYTE = 1;
pub const PFD_UNDERLAY_PLANE: BYTE = 0xFF;
pub const PFD_DOUBLEBUFFER: DWORD = 0x00000001;
pub const PFD_STEREO: DWORD = 0x00000002;
pub const PFD_DRAW_TO_WINDOW: DWORD = 0x00000004;
pub const PFD_DRAW_TO_BITMAP: DWORD = 0x00000008;
pub const PFD_SUPPORT_GDI: DWORD = 0x00000010;
pub const PFD_SUPPORT_OPENGL: DWORD = 0x00000020;
pub const PFD_GENERIC_FORMAT: DWORD = 0x00000040;
pub const PFD_NEED_PALETTE: DWORD = 0x00000080;
pub const PFD_NEED_SYSTEM_PALETTE: DWORD = 0x00000100;
pub const PFD_SWAP_EXCHANGE: DWORD = 0x00000200;
pub const PFD_SWAP_COPY: DWORD = 0x00000400;
pub const PFD_SWAP_LAYER_BUFFERS: DWORD = 0x00000800;
pub const PFD_GENERIC_ACCELERATED: DWORD = 0x00001000;
pub const PFD_SUPPORT_DIRECTDRAW: DWORD = 0x00002000;
pub const PFD_DIRECT3D_ACCELERATED: DWORD = 0x00004000;
pub const PFD_SUPPORT_COMPOSITION: DWORD = 0x00008000;
pub const PFD_DEPTH_DONTCARE: DWORD = 0x20000000;
pub const PFD_DOUBLEBUFFER_DONTCARE: DWORD = 0x40000000;
pub const PFD_STEREO_DONTCARE: DWORD = 0x80000000;

pub const CCHFORMNAME: usize = 32;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DEVMODEA {
    pub dmDeviceName: [CHAR; CCHDEVICENAME],
    pub dmSpecVersion: WORD,
    pub dmDriverVersion: WORD,
    pub dmSize: WORD,
    pub dmDriverExtra: WORD,
    pub dmFields: DWORD,
    pub union1: [u8; 16],
    pub dmColor: c_short,
    pub dmDuplex: c_short,
    pub dmYResolution: c_short,
    pub dmTTOption: c_short,
    pub dmCollate: c_short,
    pub dmFormName: [CHAR; CCHFORMNAME],
    pub dmLogPixels: WORD,
    pub dmBitsPerPel: DWORD,
    pub dmPelsWidth: DWORD,
    pub dmPelsHeight: DWORD,
    pub dmDisplayFlags: DWORD,
    pub dmDisplayFrequency: DWORD,
    pub dmICMMethod: DWORD,
    pub dmICMIntent: DWORD,
    pub dmMediaType: DWORD,
    pub dmDitherType: DWORD,
    pub dmReserved1: DWORD,
    pub dmReserved2: DWORD,
    pub dmPanningWidth: DWORD,
    pub dmPanningHeight: DWORD,
}
pub type PDEVMODEA = *mut DEVMODEA;
pub type NPDEVMODEA = *mut DEVMODEA;
pub type LPDEVMODEA = *mut DEVMODEA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DEVMODEW {
    pub dmDeviceName: [WCHAR; CCHDEVICENAME],
    pub dmSpecVersion: WORD,
    pub dmDriverVersion: WORD,
    pub dmSize: WORD,
    pub dmDriverExtra: WORD,
    pub dmFields: DWORD,
    pub union1: [u8; 16],
    pub dmColor: c_short,
    pub dmDuplex: c_short,
    pub dmYResolution: c_short,
    pub dmTTOption: c_short,
    pub dmCollate: c_short,
    pub dmFormName: [WCHAR; CCHFORMNAME],
    pub dmLogPixels: WORD,
    pub dmBitsPerPel: DWORD,
    pub dmPelsWidth: DWORD,
    pub dmPelsHeight: DWORD,
    pub dmDisplayFlags: DWORD,
    pub dmDisplayFrequency: DWORD,
    pub dmICMMethod: DWORD,
    pub dmICMIntent: DWORD,
    pub dmMediaType: DWORD,
    pub dmDitherType: DWORD,
    pub dmReserved1: DWORD,
    pub dmReserved2: DWORD,
    pub dmPanningWidth: DWORD,
    pub dmPanningHeight: DWORD,
}
pub type PDEVMODEW = *mut DEVMODEW;
pub type NPDEVMODEW = *mut DEVMODEW;
pub type LPDEVMODEW = *mut DEVMODEW;
#[repr(C)] #[derive(Copy)]
pub struct DISPLAY_DEVICEW {
    pub cb: DWORD,
    pub DeviceName: [WCHAR; 32],
    pub DeviceString: [WCHAR; 128],
    pub StateFlags: DWORD,
    pub DeviceID: [WCHAR; 128],
    pub DeviceKey: [WCHAR; 128],
}
impl Clone for DISPLAY_DEVICEW { fn clone(&self) -> DISPLAY_DEVICEW { *self } }
pub type PDISPLAY_DEVICEW = *mut DISPLAY_DEVICEW;
pub type LPDISPLAY_DEVICEW = *mut DISPLAY_DEVICEW;
#[repr(C)] #[derive(Copy)]
pub struct DISPLAY_DEVICEA {
    pub cb: DWORD,
    pub DeviceName: [CHAR; 32],
    pub DeviceString: [CHAR; 128],
    pub StateFlags: DWORD,
    pub DeviceID: [CHAR; 128],
    pub DeviceKey: [CHAR; 128],
}
impl Clone for DISPLAY_DEVICEA { fn clone(&self) -> DISPLAY_DEVICEA { *self } }
pub type PDISPLAY_DEVICEA = *mut DISPLAY_DEVICEA;
pub type LPDISPLAY_DEVICEA = *mut DISPLAY_DEVICEA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct PIXELFORMATDESCRIPTOR {
    pub nSize: WORD,
    pub nVersion: WORD,
    pub dwFlags: DWORD,
    pub iPixelType: BYTE,
    pub cColorBits: BYTE,
    pub cRedBits: BYTE,
    pub cRedShift: BYTE,
    pub cGreenBits: BYTE,
    pub cGreenShift: BYTE,
    pub cBlueBits: BYTE,
    pub cBlueShift: BYTE,
    pub cAlphaBits: BYTE,
    pub cAlphaShift: BYTE,
    pub cAccumBits: BYTE,
    pub cAccumRedBits: BYTE,
    pub cAccumGreenBits: BYTE,
    pub cAccumBlueBits: BYTE,
    pub cAccumAlphaBits: BYTE,
    pub cDepthBits: BYTE,
    pub cStencilBits: BYTE,
    pub cAuxBuffers: BYTE,
    pub iLayerType: BYTE,
    pub bReserved: BYTE,
    pub dwLayerMask: DWORD,
    pub dwVisibleMask: DWORD,
    pub dwDamageMask: DWORD,
}
pub type PPIXELFORMATDESCRIPTOR = *mut PIXELFORMATDESCRIPTOR;
pub type LPPIXELFORMATDESCRIPTOR = *mut PIXELFORMATDESCRIPTOR;

//-------------------------------------------------------------------------------------------------
// Constants
//-------------------------------------------------------------------------------------------------

// shlobj.h
// constants
#[cfg(target_arch = "x86")]
pub const INVALID_HANDLE_VALUE: HANDLE = 0xFFFFFFFF as HANDLE;
#[cfg(target_arch = "x86_64")]
pub const INVALID_HANDLE_VALUE: HANDLE = 0xFFFFFFFFFFFFFFFF as HANDLE;

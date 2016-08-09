// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! Macros to make things easier to define
macro_rules! DECLARE_HANDLE {
    ($name:ident, $inner:ident) => {
        #[allow(missing_copy_implementations)] pub enum $inner { }
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
macro_rules! HIDP_ERROR_CODES {
    ($sev:expr, $code:expr) => {
        ($sev << 28) | (::FACILITY_HID_ERROR_CODE << 16) | $code
    }
}
macro_rules! MAKEFOURCC {
    ($a:expr, $b:expr, $c:expr, $d:expr) => {
        ($a as i32) | (($b as i32) << 8) | (($c as i32) << 16) | (($d as i32) << 24)
    }
}
macro_rules! DEFINE_GUID {
    (
        $name:ident, $l:expr, $w1:expr, $w2:expr,
        $b1:expr, $b2:expr, $b3:expr, $b4:expr, $b5:expr, $b6:expr, $b7:expr, $b8:expr
    ) => {
        pub const $name: $crate::GUID = $crate::GUID {
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
macro_rules! HID_CTL_CODE {
    ($id:expr) => {
        CTL_CODE!(::FILE_DEVICE_KEYBOARD, $id, ::METHOD_NEITHER, ::FILE_ANY_ACCESS)
    }
}
macro_rules! HID_BUFFER_CTL_CODE {
    ($id:expr) => {
        CTL_CODE!(::FILE_DEVICE_KEYBOARD, $id, ::METHOD_BUFFERED, ::FILE_ANY_ACCESS)
    }
}
macro_rules! HID_IN_CTL_CODE {
    ($id:expr) => {
        CTL_CODE!(::FILE_DEVICE_KEYBOARD, $id, ::METHOD_IN_DIRECT, ::FILE_ANY_ACCESS)
    }
}
macro_rules! HID_OUT_CTL_CODE {
    ($id:expr) => {
        CTL_CODE!(::FILE_DEVICE_KEYBOARD, $id, ::METHOD_OUT_DIRECT, ::FILE_ANY_ACCESS)
    }
}
macro_rules! AUDCLNT_ERR {
    ($n:expr) => {
        MAKE_HRESULT!(::SEVERITY_ERROR, ::FACILITY_AUDCLNT, $n)
    };
}
macro_rules! AUDCLNT_SUCCESS {
    ($n:expr) => {
        MAKE_SCODE!(::SEVERITY_SUCCESS, ::FACILITY_AUDCLNT, $n)
    };
}
macro_rules! BCRYPT_MAKE_INTERFACE_VERSION {
    ($major:expr, $minor:expr) => {
        ::BCRYPT_INTERFACE_VERSION { MajorVersion: $major, MinorVersion: $minor }
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
        #[repr(C)] #[allow(missing_copy_implementations)]
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
    (interface $interface:ident ($vtbl:ident) : $pinterface:ident ($pvtbl:ident) {
    }) => {
        #[repr(C)] #[allow(missing_copy_implementations)]
        pub struct $vtbl {
            pub parent: $pvtbl
        }
        #[repr(C)] #[allow(missing_copy_implementations)]
        pub struct $interface {
            pub lpVtbl: *const $vtbl
        }
        impl ::std::ops::Deref for $interface {
            type Target = $pinterface;
            #[inline]
            fn deref(&self) -> &$pinterface {
                unsafe { &*(self as *const _ as *const _) }
            }
        }
        impl ::std::ops::DerefMut for $interface {
            #[inline]
            fn deref_mut(&mut self) -> &mut $pinterface {
                unsafe { &mut *(self as *mut _ as *mut _) }
            }
        }
    };
    (interface $interface:ident ($vtbl:ident) : $pinterface:ident ($pvtbl:ident)
        {$(
            fn $method:ident(&mut self $(,$p:ident : $t:ty)*) -> $rtr:ty
        ),+}
    ) => {
        #[repr(C)] #[allow(missing_copy_implementations)]
        pub struct $vtbl {
            pub parent: $crate::$pvtbl
            $(,pub $method: unsafe extern "system" fn(
                This: *mut $interface
                $(,$p: $t)*
            ) -> $rtr)+
        }
        #[repr(C)] #[allow(missing_copy_implementations)]
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
            type Target = $crate::$pinterface;
            #[inline]
            fn deref(&self) -> &$crate::$pinterface {
                unsafe { &*(self as *const _ as *const _) }
            }
        }
        impl ::std::ops::DerefMut for $interface {
            #[inline]
            fn deref_mut(&mut self) -> &mut $crate::$pinterface {
                unsafe { &mut *(self as *mut _ as *mut _) }
            }
        }
    };
}
macro_rules! UNION {
    ($base:ident, $field:ident, $variant:ident, $variantmut:ident, $fieldtype:ty) => {
        impl $base {
            #[inline]
            pub unsafe fn $variant(&self) -> &$fieldtype {
                &*(self as *const _ as *const _)
            }
            #[inline]
            pub unsafe fn $variantmut(&mut self) -> &mut $fieldtype {
                &mut *(self as *mut _ as *mut _)
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
macro_rules! ENUM {
    {enum $name:ident { $($variant:ident = $value:expr,)+ }} => {
        pub type $name = u32;
        $(pub const $variant: u32 = $value;)+
    };
    {enum $name:ident { $variant:ident = $value:expr, $($rest:tt)* }} => {
        pub type $name = u32;
        pub const $variant: u32 = $value;
        ENUM!{@gen $variant, $($rest)*}
    };
    {enum $name:ident { $variant:ident, $($rest:tt)* }} => {
        ENUM!{enum $name { $variant = 0, $($rest)* }}
    };
    {@gen $base:ident,} => {};
    {@gen $base:ident, $variant:ident = $value:expr, $($rest:tt)*} => {
        pub const $variant: u32 = $value;
        ENUM!{@gen $variant, $($rest)*}
    };
    {@gen $base:ident, $variant:ident, $($rest:tt)*} => {
        pub const $variant: u32 = $base + 1u32;
        ENUM!{@gen $variant, $($rest)*}
    };
}
macro_rules! STRUCT {
    {$(#[$attrs:meta])* struct $name:ident { $($field:ident: $ftype:ty,)+ }} => {
        #[repr(C)] $(#[$attrs])*
        pub struct $name {
            $(pub $field: $ftype,)+
        }
        impl Copy for $name {}
        impl Clone for $name { fn clone(&self) -> $name { *self } }
    };
}

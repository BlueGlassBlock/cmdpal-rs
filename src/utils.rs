use crate::bindings::*;
use std::ops::DerefMut;
use windows::Storage::Streams::{IBuffer, IBuffer_Impl};
use windows::Win32::Foundation::E_NOTIMPL;
use windows::Win32::System::WinRT::{IBufferByteAccess, IBufferByteAccess_Impl};
use windows::core::{Array, implement};

#[implement(IBuffer, IBufferByteAccess)]
pub(crate) struct FrozenBuffer {
    data: Vec<u8>,
}

impl From<Vec<u8>> for FrozenBuffer {
    fn from(value: Vec<u8>) -> Self {
        FrozenBuffer { data: value }
    }
}

impl IBuffer_Impl for FrozenBuffer_Impl {
    fn Capacity(&self) -> windows_core::Result<u32> {
        Ok(self.data.len() as u32)
    }
    fn Length(&self) -> windows_core::Result<u32> {
        Ok(self.data.len() as u32)
    }
    fn SetLength(&self, _: u32) -> windows_core::Result<()> {
        Err(E_NOTIMPL.into())
    }
}

impl IBufferByteAccess_Impl for FrozenBuffer_Impl {
    fn Buffer(&self) -> windows_core::Result<*mut u8> {
        Ok(self.data.as_ptr() as *mut u8)
    }
}

#[implement(IGridProperties)]
pub struct GridProperties(pub windows::Foundation::Size);

impl IGridProperties_Impl for GridProperties_Impl {
    fn TileSize(&self) -> windows_core::Result<windows::Foundation::Size> {
        Ok(self.0)
    }
}

pub fn map_array<T: windows::core::Type<T>, S>(slice: &[S], map: fn(&S) -> T::Default) -> Array<T> {
    let mut arr = Array::with_len(slice.len());
    for (i, item) in slice.iter().enumerate() {
        arr.deref_mut()[i] = map(item);
    }
    arr
}

impl From<Option<Color>> for OptionalColor {
    fn from(value: Option<Color>) -> Self {
        match value {
            Some(color) => OptionalColor {
                HasValue: true,
                Color: color,
            },
            None => OptionalColor {
                HasValue: false,
                Color: Color::default(),
            },
        }
    }
}

pub trait OkOrEmpty {
    type Target;
    fn ok_or_empty(self) -> windows_core::Result<Self::Target>;
}

impl<T> OkOrEmpty for Option<T> {
    type Target = T;
    fn ok_or_empty(self) -> windows_core::Result<Self::Target> {
        self.ok_or(windows_core::Error::empty())
    }
}

pub trait ComBuilder: Sized {
    type Target: windows::core::ComObjectInner;
    fn build_unmanaged(self) -> Self::Target;
    fn build(self) -> windows::core::ComObject<Self::Target> {
        self.build_unmanaged().into()
    }
}

#[allow(dead_code, reason = "Compile check only")]
pub(crate) const fn assert_send_sync<T: Send + Sync>() {}


#[doc(hidden)]
#[macro_export]
macro_rules! _define_windows_core_interface_with_bindings_docs {
    ($name:ident, $vtbl:ident, $iid:literal) => {
        #[repr(transparent)]
        #[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::clone::Clone)]
        #[doc = include_str!(concat!(
            "./bindings_docs/",
            stringify!($name),
            ".md"
        ))]
        pub struct $name(::windows_core::IUnknown);
        unsafe impl ::windows_core::Interface for $name {
            type Vtable = $vtbl;
            const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128($iid);
        }
        impl ::core::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.debug_tuple(stringify!($name))
                    .field(&::windows_core::Interface::as_raw(self))
                    .finish()
            }
        }
    };
    ($name:ident, $vtbl:ident) => {
        #[repr(transparent)]
        #[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::clone::Clone)]
        #[doc = include_str!(concat!(
            "./bindings_docs/",
            stringify!($name),
            ".md"
        ))]
        pub struct $name(::core::ptr::NonNull<::core::ffi::c_void>);
        unsafe impl ::windows_core::Interface for $name {
            type Vtable = $vtbl;
            const IID: ::windows_core::GUID = ::windows_core::GUID::zeroed();
            const UNKNOWN: bool = false;
        }
        impl ::core::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.debug_tuple(stringify!($name)).field(&self.0).finish()
            }
        }
    };
}
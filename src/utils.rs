use windows::core::{implement, Array};
use windows::Storage::Streams::{IBuffer, IBuffer_Impl};
use windows::Win32::Foundation::E_NOTIMPL;
use windows::Win32::System::WinRT::{IBufferByteAccess, IBufferByteAccess_Impl};
use std::ops::DerefMut;
use crate::bindings::*;

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

pub fn map_array<T: windows::core::Type<T>, S>(
    slice: &[S],
    map: fn(&S) -> T::Default,
) -> Array<T> {
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

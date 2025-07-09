//! Icon data representation

use std::path::{Path, PathBuf};

use crate::bindings::*;
use crate::utils::{FrozenBuffer, OkOrEmpty};
use windows::Storage::Streams::{
    IBuffer, IRandomAccessStream, InMemoryRandomAccessStream, RandomAccessStreamReference,
};
use windows::core::{AgileReference, HSTRING, implement};

/// Represents icon data.
///
/// Use [`From`] or [`TryFrom`] to create an instance from various types and their references:
/// - [`HSTRING`] / [`String`]: URL or a canonical path to the icon.
/// - [`PathBuf`]: Path to the icon file, which will be canonicalized.
/// - [`Vec<u8>`]: Raw icon data
///
#[doc = include_str!("../bindings_docs/IIconData.md")]
#[implement(IIconData)]
#[derive(Debug, Clone)]
pub struct IconData {
    icon: HSTRING,
    data: Option<AgileReference<IRandomAccessStream>>,
}

impl IIconData_Impl for IconData_Impl {
    fn Icon(&self) -> windows_core::Result<windows_core::HSTRING> {
        Ok(self.icon.clone())
    }

    fn Data(
        &self,
    ) -> windows_core::Result<windows::Storage::Streams::IRandomAccessStreamReference> {
        let stream = self.data.as_ref().ok_or_empty()?;
        RandomAccessStreamReference::CreateFromStream(&stream.resolve()?).map(Into::into)
    }
}

impl From<HSTRING> for IconData {
    fn from(value: HSTRING) -> Self {
        IconData {
            icon: value,
            data: None,
        }
    }
}

impl From<&HSTRING> for IconData {
    fn from(value: &HSTRING) -> Self {
        IconData {
            icon: value.clone(),
            data: None,
        }
    }
}

impl From<&str> for IconData {
    fn from(value: &str) -> Self {
        IconData {
            icon: HSTRING::from(value),
            data: None,
        }
    }
}

impl From<String> for IconData {
    fn from(value: String) -> Self {
        IconData {
            icon: HSTRING::from(value),
            data: None,
        }
    }
}

impl TryFrom<&Path> for IconData {
    type Error = windows::core::Error;
    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        value
            .canonicalize()
            .map(|path| IconData {
                icon: HSTRING::from(path.as_os_str()),
                data: None,
            })
            .map_err(|e| {
                windows::core::Error::new(
                    windows::Win32::Foundation::ERROR_FILE_NOT_FOUND.to_hresult(),
                    e.to_string(),
                )
            })
    }
}

impl TryFrom<PathBuf> for IconData {
    type Error = windows::core::Error;
    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        IconData::try_from(value.as_path())
    }
}

impl TryFrom<Vec<u8>> for IconData {
    type Error = windows::core::Error;
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let buf: IBuffer = FrozenBuffer::from(value).into();
        let stream = InMemoryRandomAccessStream::new()?;
        let op = stream.WriteAsync(&buf)?;
        op.get()?;
        Ok(IconData {
            icon: HSTRING::from(""),
            data: Some(AgileReference::new(&stream.into())?),
        })
    }
}

impl TryFrom<&[u8]> for IconData {
    type Error = windows::core::Error;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let buf: IBuffer = FrozenBuffer::from(value.to_vec()).into();
        let stream = InMemoryRandomAccessStream::new()?;
        let op = stream.WriteAsync(&buf)?;
        op.get()?;
        Ok(IconData {
            icon: HSTRING::from(""),
            data: Some(AgileReference::new(&stream.into())?),
        })
    }
}

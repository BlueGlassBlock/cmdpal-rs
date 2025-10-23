//! Icon data representation

use std::path::{Path, PathBuf};

use crate::bindings::*;
use crate::utils::{FrozenBuffer, OkOrEmpty};
use windows::Storage::Streams::{
    IBuffer, IRandomAccessStream, InMemoryRandomAccessStream, RandomAccessStreamReference,
};
use windows_core::{AgileReference, HSTRING, Interface, implement};

/// Represents icon data.
///
/// Use [`From`] or [`TryFrom`] to create an instance from various types and their references:
/// - [`HSTRING`] / [`String`]: [Segoe Fluent Icon](https://learn.microsoft.com/en-us/windows/apps/design/style/segoe-fluent-icons-font) codepoint, URL or a canonical path to the icon.
/// - [`PathBuf`]: Path to the icon file, which will be canonicalized.
/// - [`Vec<u8>`]: Raw icon data
///
#[doc = include_str!("../bindings_docs/IIconData.md")]
#[implement(IIconData, IExtendedAttributesProvider)]
#[derive(Debug, Clone)]
pub struct IconData {
    icon: HSTRING,
    font_family: HSTRING,
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

impl IExtendedAttributesProvider_Impl for IconData_Impl {
    fn GetProperties(
        &self,
    ) -> windows_core::Result<
        windows_collections::IMap<windows_core::HSTRING, windows_core::IInspectable>,
    > {
        let map = windows::Foundation::Collections::ValueSet::new()?;
        if !self.font_family.is_empty() {
            map.Insert(
                &"FontFamily".into(),
                &windows::Foundation::PropertyValue::CreateString(&self.font_family)?,
            )?;
        }
        Ok(Interface::cast(&map)?)
    }
}

impl IconData {
    /// Creates an icon data from font glyph and its font family.
    ///
    /// Note that Command Palette will default to using the Segoe Fluent Icons,
    /// Segoe MDL2 Assets font for glyphs in the Segoe UI Symbol range, or Segoe
    /// UI for any other glyphs. Use this function if you want a non-Segoe font
    /// icon.
    pub fn from_glyph_and_family<G, F>(glyph: G, font_family: F) -> Self
    where
        G: Into<HSTRING>,
        F: Into<HSTRING>,
    {
        IconData {
            icon: glyph.into(),
            font_family: font_family.into(),
            data: None,
        }
    }
}

impl From<HSTRING> for IconData {
    fn from(value: HSTRING) -> Self {
        IconData {
            icon: value,
            font_family: HSTRING::new(),
            data: None,
        }
    }
}

impl From<&HSTRING> for IconData {
    fn from(value: &HSTRING) -> Self {
        IconData {
            icon: value.clone(),
            font_family: HSTRING::new(),
            data: None,
        }
    }
}

impl From<&str> for IconData {
    fn from(value: &str) -> Self {
        IconData {
            icon: HSTRING::from(value),
            font_family: HSTRING::new(),
            data: None,
        }
    }
}

impl From<String> for IconData {
    fn from(value: String) -> Self {
        IconData {
            icon: HSTRING::from(value),
            font_family: HSTRING::new(),
            data: None,
        }
    }
}

impl TryFrom<&Path> for IconData {
    type Error = windows_core::Error;
    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        value
            .canonicalize()
            .map(|path| IconData {
                icon: HSTRING::from(path.as_os_str()),
                font_family: HSTRING::new(),
                data: None,
            })
            .map_err(|e| {
                windows_core::Error::new(
                    windows::Win32::Foundation::ERROR_FILE_NOT_FOUND.to_hresult(),
                    e.to_string(),
                )
            })
    }
}

impl TryFrom<PathBuf> for IconData {
    type Error = windows_core::Error;
    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        IconData::try_from(value.as_path())
    }
}

impl TryFrom<Vec<u8>> for IconData {
    type Error = windows_core::Error;
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let buf: IBuffer = FrozenBuffer::from(value).into();
        let stream = InMemoryRandomAccessStream::new()?;
        let op = stream.WriteAsync(&buf)?;
        op.join()?;
        Ok(IconData {
            icon: HSTRING::new(),
            font_family: HSTRING::new(),
            data: Some(AgileReference::new(&stream.into())?),
        })
    }
}

impl TryFrom<&[u8]> for IconData {
    type Error = windows_core::Error;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let buf: IBuffer = FrozenBuffer::from(value.to_vec()).into();
        let stream = InMemoryRandomAccessStream::new()?;
        let op = stream.WriteAsync(&buf)?;
        op.join()?;
        Ok(IconData {
            icon: HSTRING::new(),
            font_family: HSTRING::new(),
            data: Some(AgileReference::new(&stream.into())?),
        })
    }
}

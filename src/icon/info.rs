//! Light and dark icon information representation

use windows_core::{ComObject, Result};

use crate::bindings::*;

use super::data::IconData;

/// Representation of light and dark icon bundle.
///
#[doc = include_str!("../bindings_docs/IIconInfo.md")]
#[windows_core::implement(IIconInfo)]
#[derive(Debug, Clone)]
pub struct IconInfo {
    #[doc = include_str!("../bindings_docs/IIconInfo/Light.md")]
    pub light: ComObject<IconData>,
    #[doc = include_str!("../bindings_docs/IIconInfo/Dark.md")]
    pub dark: ComObject<IconData>,
}

impl IconInfo {
    /// Creates an unmanaged `IconInfo` which has the same light and dark icon.
    pub fn new_unmanaged(data: impl Into<ComObject<IconData>>) -> Self {
        let data: ComObject<IconData> = data.into();
        Self {
            light: data.clone(),
            dark: data,
        }
    }

    /// Creates a reference-counted COM object `IconInfo` which has the same light and dark icon.
    pub fn new(data: impl Into<ComObject<IconData>>) -> ComObject<Self> {
        ComObject::new(Self::new_unmanaged(data))
    }

    /// Creates an unmanaged `IconInfo` with different icons.
    pub fn new_unmanaged_bundle(
        light: impl Into<ComObject<IconData>>,
        dark: impl Into<ComObject<IconData>>,
    ) -> Self {
        Self {
            light: light.into(),
            dark: dark.into(),
        }
    }

    /// Creates a reference-counted COM object `IconInfo` with different icons.
    pub fn new_bundle(
        light: impl Into<ComObject<IconData>>,
        dark: impl Into<ComObject<IconData>>,
    ) -> ComObject<Self> {
        ComObject::new(Self::new_unmanaged_bundle(light, dark))
    }
}

impl IIconInfo_Impl for IconInfo_Impl {
    fn Dark(&self) -> Result<IIconData> {
        Ok(self.dark.to_interface())
    }

    fn Light(&self) -> Result<IIconData> {
        Ok(self.light.to_interface())
    }
}

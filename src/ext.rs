//! The [`Extension`] struct.

use windows::Foundation::{IClosable, IClosable_Impl};
use windows_core::{ComObject, IUnknownImpl, Result};

use crate::bindings::*;
use crate::cmd_provider::{CommandProvider, CommandProvider_Impl};

/// Representation of a Command Palette extension.
///
#[doc = include_str!("./bindings_docs/IExtension.md")]
#[windows_core::implement(IExtension, IClosable)]
pub struct Extension {
    /// The command provider for this extension.
    pub cmd_provider: ComObject<CommandProvider>,
}

impl From<CommandProvider> for Extension {
    fn from(value: CommandProvider) -> Self {
        Self {
            cmd_provider: value.into(),
        }
    }
}

impl From<ComObject<CommandProvider>> for Extension {
    fn from(value: ComObject<CommandProvider>) -> Self {
        Self {
            cmd_provider: value,
        }
    }
}

impl From<&CommandProvider_Impl> for Extension {
    fn from(value: &CommandProvider_Impl) -> Self {
        Self {
            cmd_provider: value.to_object(),
        }
    }
}

impl IClosable_Impl for Extension_Impl {
    fn Close(&self) -> Result<()> {
        Ok(())
    }
}

impl IExtension_Impl for Extension_Impl {
    fn GetProvider(&self, provider_type: ProviderType) -> Result<windows_core::IInspectable> {
        match provider_type {
            ProviderType::Commands => Ok(self.cmd_provider.to_interface()),
            _ => Err(windows::Win32::Foundation::E_NOTIMPL.into()),
        }
    }

    fn Dispose(&self) -> Result<()> {
        Ok(())
    }
}

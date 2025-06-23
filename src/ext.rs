use crate::bindings::*;
use windows::Foundation::{IClosable, IClosable_Impl};
use windows::Win32::Foundation::E_NOTIMPL;
use windows_core::{IInspectable, implement};

#[implement(IExtension, IClosable)]
pub struct Extension {
    pub cmd_provider: ICommandProvider,
}

impl From<crate::cmd_provider::CommandProvider> for Extension {
    fn from(value: crate::cmd_provider::CommandProvider) -> Self {
        Self {
            cmd_provider: value.into(),
        }
    }
}

impl From<&crate::cmd_provider::CommandProvider_Impl> for Extension {
    fn from(value: &crate::cmd_provider::CommandProvider_Impl) -> Self {
        Self {
            cmd_provider: windows::core::IUnknownImpl::to_interface(value)
        }
    }
}

impl From<ICommandProvider> for Extension {
    fn from(provider: ICommandProvider) -> Self {
        Self {
            cmd_provider: provider,
        }
    }
}

impl IClosable_Impl for Extension_Impl {
    fn Close(&self) -> windows_core::Result<()> {
        Ok(())
    }
}

impl IExtension_Impl for Extension_Impl {
    fn GetProvider(
        &self,
        provider_type: ProviderType,
    ) -> windows_core::Result<windows_core::IInspectable> {
        let res = match provider_type {
            ProviderType::Commands => Ok(IInspectable::from(self.cmd_provider.clone())),
            _ => Err(E_NOTIMPL.into()),
        };
        res
    }

    fn Dispose(&self) -> windows_core::Result<()> {
        Ok(())
    }
}

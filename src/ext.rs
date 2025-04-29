use crate::bindings::*;
use windows::Foundation::{IClosable, IClosable_Impl};
use windows::Win32::Foundation::E_NOTIMPL;
use windows_core::{implement, IInspectable};

#[implement(IExtension, IClosable)]
pub struct Extension {
    pub cmd_provider: ICommandProvider,
}

impl IClosable_Impl for Extension_Impl {
    fn Close(&self) -> windows_core::Result<()> {
        tracing::info!("Extension closed");
        Ok(())
    }
}

impl IExtension_Impl for Extension_Impl {
    fn GetProvider(
        &self,
        provider_type: ProviderType,
    ) -> windows_core::Result<windows_core::IInspectable> {
        tracing::info!("GetProvider called {:?}", provider_type);
        let res = match provider_type {
            ProviderType::Commands => Ok(IInspectable::from(self.cmd_provider.clone())),
            _ => Err(E_NOTIMPL.into()),
        };
        tracing::info!("GetProvider result: {:?}", res);
        res
    }

    fn Dispose(&self) -> windows_core::Result<()> {
        tracing::info!("Extension disposed");
        Ok(())
    }
}

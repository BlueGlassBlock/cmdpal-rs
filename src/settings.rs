use crate::{bindings::*, page::content::ContentPage};
use windows::core::{ComObject, implement};

// TODO: Add wrappers

#[implement(ICommandSettings)]
pub struct CommandSettings(pub ComObject<ContentPage>);

impl ICommandSettings_Impl for CommandSettings_Impl {
    fn SettingsPage(&self) -> windows_core::Result<IContentPage> {
        Ok(self.0.to_interface())
    }
}

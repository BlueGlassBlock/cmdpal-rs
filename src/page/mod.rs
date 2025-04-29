pub mod content;
pub mod list;
pub mod dyn_list;

use crate::bindings::*;
use crate::cmd::BaseCommand;
use crate::notify::*;
use windows::core::{ComObject, HSTRING, implement, Result, IUnknownImpl as _};

#[implement(IPage)]
pub struct BasePage{
    title: NotifyLock<HSTRING>,
    loading: NotifyLock<bool>,
    accent_color: NotifyLock<Option<Color>>,
    command: ComObject<BaseCommand>,
}

impl BasePage{
    pub fn new(
        title: impl Into<HSTRING>,
        loading: bool,
        accent_color: Option<Color>,
        command: ComObject<BaseCommand>,
    ) -> Self {
        let title = NotifyLock::new(title.into());
        let loading = NotifyLock::new(loading);
        let accent_color = NotifyLock::new(accent_color);

        BasePage {
            title,
            loading,
            accent_color,
            command,
        }
    }
}

impl BasePage_Impl {
    pub fn title(&self) -> Result<NotifyLockReadGuard<'_, HSTRING>> {
        self.title.read()
    }

    pub fn title_mut(&self) -> Result<NotifyLockWriteGuard<'_, HSTRING, impl Fn()>> {
        self.title.write(|| self.command.emit_prop_changed(self.to_interface(), "Title"))
    }

    pub fn loading(&self) -> Result<NotifyLockReadGuard<'_, bool>> {
        self.loading.read()
    }

    pub fn loading_mut(&self) -> Result<NotifyLockWriteGuard<'_, bool, impl Fn()>> {
        self.loading.write(|| self.command.emit_prop_changed(self.to_interface(), "Loading"))
    }

    pub fn accent_color(&self) -> Result<NotifyLockReadGuard<'_, Option<Color>>> {
        self.accent_color.read()
    }

    pub fn accent_color_mut(&self) -> Result<NotifyLockWriteGuard<'_, Option<Color>, impl Fn()>> {
        self.accent_color.write(|| self.command.emit_prop_changed(self.to_interface(), "AccentColor"))
    }
}

impl IPage_Impl for BasePage_Impl{
    fn Title(&self) -> windows_core::Result<windows_core::HSTRING> {
        Ok(self.title.read()?.clone())
    }

    fn IsLoading(&self) -> windows_core::Result<bool> {
        Ok(*self.loading.read()?)
    }

    fn AccentColor(&self) -> windows_core::Result<OptionalColor> {
        Ok((*self.accent_color.read()?).into())
    }
}

impl ICommand_Impl for BasePage_Impl{
    ambassador_impl_ICommand_Impl! {
        body_struct(< >, ComObject<BaseCommand>, command)
    }
}

impl INotifyPropChanged_Impl for BasePage_Impl{
    ambassador_impl_INotifyPropChanged_Impl! {
        body_struct(< >, ComObject<BaseCommand>, command)
    }
}
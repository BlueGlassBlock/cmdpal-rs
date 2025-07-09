pub mod content;
pub mod dyn_list;
pub mod list;

use std::ops::Deref;

use crate::cmd::{BaseCommand, BaseCommand_Impl};
use crate::notify::*;
use crate::utils::assert_send_sync;
use crate::{bindings::*, utils::ComBuilder};
use windows::core::{ComObject, implement};
use windows_core::{HSTRING, IUnknownImpl as _, Result};

#[implement(IPage)]
pub struct BasePage {
    title: NotifyLock<HSTRING>,
    loading: NotifyLock<bool>,
    accent_color: NotifyLock<Option<Color>>,
    pub base: ComObject<BaseCommand>,
}

pub struct BasePageBuilder {
    title: HSTRING,
    loading: bool,
    accent_color: Option<Color>,
    base: ComObject<BaseCommand>,
}

impl BasePageBuilder {
    pub fn new(base: ComObject<BaseCommand>) -> Self {
        BasePageBuilder {
            title: HSTRING::new(),
            loading: true,
            accent_color: None,
            base,
        }
    }

    pub fn title(mut self, title: impl Into<HSTRING>) -> Self {
        self.title = title.into();
        self
    }

    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    pub fn accent_color(mut self, accent_color: Option<Color>) -> Self {
        self.accent_color = accent_color;
        self
    }

    pub fn command(mut self, command: ComObject<BaseCommand>) -> Self {
        self.base = command;
        self
    }
}

impl ComBuilder for BasePageBuilder {
    type Output = BasePage;
    fn build_unmanaged(self) -> BasePage {
        BasePage {
            title: NotifyLock::new(self.title),
            loading: NotifyLock::new(self.loading),
            accent_color: NotifyLock::new(self.accent_color),
            base: self.base,
        }
    }
}

impl Deref for BasePage {
    type Target = BaseCommand_Impl;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl BasePage_Impl {
    pub fn title(&self) -> Result<NotifyLockReadGuard<'_, HSTRING>> {
        self.title.read()
    }

    pub fn title_mut(&self) -> Result<NotifyLockWriteGuard<'_, HSTRING>> {
        self.title
            .write(|| self.base.emit_prop_changed(self.to_interface(), "Title"))
    }

    pub fn loading(&self) -> Result<NotifyLockReadGuard<'_, bool>> {
        self.loading.read()
    }

    pub fn loading_mut(&self) -> Result<NotifyLockWriteGuard<'_, bool>> {
        self.loading.write(|| {
            self.base
                .emit_prop_changed(self.to_interface(), "Loading")
        })
    }

    pub fn accent_color(&self) -> Result<NotifyLockReadGuard<'_, Option<Color>>> {
        self.accent_color.read()
    }

    pub fn accent_color_mut(&self) -> Result<NotifyLockWriteGuard<'_, Option<Color>>> {
        self.accent_color.write(|| {
            self.base
                .emit_prop_changed(self.to_interface(), "AccentColor")
        })
    }
}

impl IPage_Impl for BasePage_Impl {
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

impl ICommand_Impl for BasePage_Impl {
    fn Icon(&self) -> windows_core::Result<IIconInfo> {
        self.base.Icon()
    }
    fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        self.base.Id()
    }
    fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        self.base.Name()
    }
}

impl INotifyPropChanged_Impl for BasePage_Impl {
    fn PropChanged(
        &self,
        handler: windows_core::Ref<
            '_,
            windows::Foundation::TypedEventHandler<
                windows_core::IInspectable,
                IPropChangedEventArgs,
            >,
        >,
    ) -> windows_core::Result<i64> {
        self.base.PropChanged(handler)
    }

    fn RemovePropChanged(&self, token: i64) -> windows_core::Result<()> {
        self.base.RemovePropChanged(token)
    }
}

const _: () = assert_send_sync::<ComObject<BasePage>>();

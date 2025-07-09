//! Pages which can be used to provide more information.
//! 
//! When selecting a command item that contains a page, the page will be displayed.

pub mod content;
pub mod dyn_list;
pub mod list;

use std::ops::Deref;

use crate::cmd::{BaseCommand, BaseCommand_Impl};
use crate::notify::*;
use crate::utils::assert_send_sync;
use crate::{bindings::*, utils::ComBuilder};
use windows_core::{ComObject, implement};
use windows_core::{HSTRING, IUnknownImpl as _, Result};

/// Represents basic properties of a page.
///
/// See [`BasePage_Impl`] for field accessors.
/// 
#[doc = include_str!("../bindings_docs/IPage.md")]
#[implement(IPage)]
pub struct BasePage {
    pub base: ComObject<BaseCommand>,
    title: NotifyLock<HSTRING>,
    loading: NotifyLock<bool>,
    accent_color: NotifyLock<Option<Color>>,
}

/// Builder for [`BasePage`].
pub struct BasePageBuilder {
    base: ComObject<BaseCommand>,
    title: HSTRING,
    loading: bool,
    accent_color: Option<Color>,
}

impl BasePageBuilder {
    /// Creates a new builder.
    pub fn new(base: ComObject<BaseCommand>) -> Self {
        BasePageBuilder {
            title: HSTRING::new(),
            loading: true,
            accent_color: None,
            base,
        }
    }

    /// Sets the title of the page.
    pub fn title(mut self, title: impl Into<HSTRING>) -> Self {
        self.title = title.into();
        self
    }

    /// Sets whether the page is loading.
    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    /// Sets the accent color of the page.
    pub fn accent_color(mut self, accent_color: Option<Color>) -> Self {
        self.accent_color = accent_color;
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
    /// Readonly access to [`IPage::Title`].
    /// 
    #[doc = include_str!("../bindings_docs/IPage/Title.md")]
    pub fn title(&self) -> Result<NotifyLockReadGuard<'_, HSTRING>> {
        self.title.read()
    }

    /// Mutable access to [`IPage::Title`].
    /// 
    #[doc = include_str!("../bindings_docs/IPage/Title.md")]
    ///
    /// Notifies the host about the property change when dropping the guard.
    pub fn title_mut(&self) -> Result<NotifyLockWriteGuard<'_, HSTRING>> {
        self.title
            .write(|| self.base.emit_prop_changed(self.to_interface(), "Title"))
    }

    /// Readonly access to [`IPage::IsLoading`].
    /// 
    #[doc = include_str!("../bindings_docs/IPage/IsLoading.md")]
    pub fn loading(&self) -> Result<NotifyLockReadGuard<'_, bool>> {
        self.loading.read()
    }

    /// Mutable access to [`IPage::IsLoading`].
    /// 
    #[doc = include_str!("../bindings_docs/IPage/IsLoading.md")]
    ///
    /// Notifies the host about the property change when dropping the guard.
    pub fn loading_mut(&self) -> Result<NotifyLockWriteGuard<'_, bool>> {
        self.loading.write(|| {
            self.base
                .emit_prop_changed(self.to_interface(), "Loading")
        })
    }

    /// Readonly access to [`IPage::AccentColor`].
    /// 
    #[doc = include_str!("../bindings_docs/IPage/AccentColor.md")]
    pub fn accent_color(&self) -> Result<NotifyLockReadGuard<'_, Option<Color>>> {
        self.accent_color.read()
    }

    /// Mutable access to [`IPage::AccentColor`].
    /// 
    #[doc = include_str!("../bindings_docs/IPage/AccentColor.md")]
    ///
    /// Notifies the host about the property change when dropping the guard.
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

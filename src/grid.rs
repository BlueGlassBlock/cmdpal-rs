//! GridProperties related types and implementations.

use crate::bindings::*;
use crate::notify::{
    NotifyLock, NotifyLockReadGuard, NotifyLockWriteGuard, PropChangedEventArgs,
    PropChangedEventHandler,
};
use windows_core::{ComObject, IInspectable, IUnknownImpl, Result, implement};

/// Represents small grid layout properties to be used in ListPage
/// 
#[doc = include_str!("./bindings_docs/ISmallGridLayout.md")]
#[implement(ISmallGridLayout, IGridProperties, INotifyPropChanged)]
pub struct SmallGridLayout {
    event: PropChangedEventHandler,
}

impl SmallGridLayout {
    /// Creates a new instance of [`SmallGridLayout`].
    pub fn new() -> ComObject<Self> {
        ComObject::new(Self {
            event: PropChangedEventHandler::new(),
        })
    }

    /// Creates a new unmanaged instance of [`SmallGridLayout`].
    pub fn new_unmanaged() -> Self {
        Self {
            event: PropChangedEventHandler::new(),
        }
    }
}

impl ISmallGridLayout_Impl for SmallGridLayout_Impl {}

impl IGridProperties_Impl for SmallGridLayout_Impl {}

impl INotifyPropChanged_Impl for SmallGridLayout_Impl {
    fn PropChanged(
        &self,
        handler: windows_core::Ref<
            windows::Foundation::TypedEventHandler<
                windows_core::IInspectable,
                IPropChangedEventArgs,
            >,
        >,
    ) -> windows_core::Result<i64> {
        self.event.add(handler.ok()?)
    }

    fn RemovePropChanged(&self, token: i64) -> windows_core::Result<()> {
        self.event.remove(token);
        Ok(())
    }
}

/// Represents medium grid layout properties to be used in ListPage
/// 
/// See [`MediumGridLayout_Impl`] for field accessors.
#[doc = include_str!("./bindings_docs/IMediumGridLayout.md")]
#[implement(IMediumGridLayout, IGridProperties, INotifyPropChanged)]
pub struct MediumGridLayout {
    event: PropChangedEventHandler,
    show_title: NotifyLock<bool>,
}

/// Field accessors for [`MediumGridLayout`].
#[doc(inline)]
pub use self::MediumGridLayout_Impl as Doc_MediumGridLayout_Impl;

impl MediumGridLayout {
    /// Creates a new instance of [`MediumGridLayout`].
    pub fn new(show_title: bool) -> ComObject<Self> {
        ComObject::new(Self {
            event: PropChangedEventHandler::new(),
            show_title: NotifyLock::new(show_title),
        })
    }

    /// Creates a new unmanaged instance of [`MediumGridLayout`].
    pub fn new_unmanaged(show_title: bool) -> Self {
        Self {
            event: PropChangedEventHandler::new(),
            show_title: NotifyLock::new(show_title),
        }
    }
}

impl MediumGridLayout_Impl {
    pub(crate) fn emit_self_prop_changed(&self, prop: &str) {
        let sender: IInspectable = self.to_interface();
        let arg: IPropChangedEventArgs = PropChangedEventArgs(prop.into()).into();
        self.event.call(|handler| handler.Invoke(&sender, &arg));
    }

    /// Readonly access to [`IMediumGridLayout::ShowTitle`].
    #[doc = include_str!("./bindings_docs/IMediumGridLayout/ShowTitle.md")]
    pub fn show_title(&self) -> Result<NotifyLockReadGuard<'_, bool>> {
        self.show_title.read()
    }

    /// Mutable access to [`IMediumGridLayout::ShowTitle`].
    #[doc = include_str!("./bindings_docs/IMediumGridLayout/ShowTitle.md")]
    pub fn show_title_mut(&mut self) -> Result<NotifyLockWriteGuard<'_, bool>> {
        self.show_title.write(|| self.emit_self_prop_changed("ShowTitle"))
    }
}

impl IMediumGridLayout_Impl for MediumGridLayout_Impl {
    fn ShowTitle(&self) -> windows_core::Result<bool> {
        Ok(*self.show_title.read()?)
    }
}

impl IGridProperties_Impl for MediumGridLayout_Impl {}

impl INotifyPropChanged_Impl for MediumGridLayout_Impl {
    fn PropChanged(
        &self,
        handler: windows_core::Ref<
            windows::Foundation::TypedEventHandler<
                windows_core::IInspectable,
                IPropChangedEventArgs,
            >,
        >,
    ) -> windows_core::Result<i64> {
        self.event.add(handler.ok()?)
    }

    fn RemovePropChanged(&self, token: i64) -> windows_core::Result<()> {
        self.event.remove(token);
        Ok(())
    }
}

/// Represents gallery grid layout properties to be used in ListPage
/// 
/// See [`GalleryGridLayout_Impl`] for field accessors.
#[doc = include_str!("./bindings_docs/IGalleryGridLayout.md")]
#[implement(IGalleryGridLayout, IGridProperties, INotifyPropChanged)]
pub struct GalleryGridLayout {
    event: PropChangedEventHandler,
    show_title: NotifyLock<bool>,
    show_subtitle: NotifyLock<bool>,
}

/// Field accessors for [`GalleryGridLayout`].
#[doc(inline)]
pub use self::GalleryGridLayout_Impl as Doc_GalleryGridLayout_Impl;

impl GalleryGridLayout {
    /// Creates a new instance of [`GalleryGridLayout`].
    pub fn new(show_title: bool, show_subtitle: bool) -> ComObject<Self> {
        ComObject::new(Self {
            event: PropChangedEventHandler::new(),
            show_title: NotifyLock::new(show_title),
            show_subtitle: NotifyLock::new(show_subtitle),
        })
    }

    /// Creates a new unmanaged instance of [`GalleryGridLayout`].
    pub fn new_unmanaged(show_title: bool, show_subtitle: bool) -> Self {
        Self {
            event: PropChangedEventHandler::new(),
            show_title: NotifyLock::new(show_title),
            show_subtitle: NotifyLock::new(show_subtitle),
        }
    }
}

impl GalleryGridLayout_Impl {
    pub(crate) fn emit_self_prop_changed(&self, prop: &str) {
        let sender: IInspectable = self.to_interface();
        let arg: IPropChangedEventArgs = PropChangedEventArgs(prop.into()).into();
        self.event.call(|handler| handler.Invoke(&sender, &arg));
    }

    /// Readonly access to [`IGalleryGridLayout::ShowTitle`].
    #[doc = include_str!("./bindings_docs/IGalleryGridLayout/ShowTitle.md")]
    pub fn show_title(&self) -> Result<NotifyLockReadGuard<'_, bool>> {
        self.show_title.read()
    }

    /// Mutable access to [`IGalleryGridLayout::ShowTitle`].
    #[doc = include_str!("./bindings_docs/IGalleryGridLayout/ShowTitle.md")]
    pub fn show_title_mut(&mut self) -> Result<NotifyLockWriteGuard<'_, bool>> {
        self.show_title.write(|| self.emit_self_prop_changed("ShowTitle"))
    }

    /// Readonly access to [`IGalleryGridLayout::ShowSubtitle`].
    #[doc = include_str!("./bindings_docs/IGalleryGridLayout/ShowSubtitle.md")]
    pub fn show_subtitle(&self) -> Result<NotifyLockReadGuard<'_, bool>> {
        self.show_subtitle.read()
    }

    /// Mutable access to [`IGalleryGridLayout::ShowSubtitle`].
    #[doc = include_str!("./bindings_docs/IGalleryGridLayout/ShowSubtitle.md")]
    pub fn show_subtitle_mut(&mut self) -> Result<NotifyLockWriteGuard<'_, bool>> {
        self.show_subtitle
            .write(|| self.emit_self_prop_changed("ShowSubtitle"))
    }
}

impl IGalleryGridLayout_Impl for GalleryGridLayout_Impl {
    fn ShowTitle(&self) -> windows_core::Result<bool> {
        Ok(*self.show_title.read()?)
    }

    fn ShowSubtitle(&self) -> windows_core::Result<bool> {
        Ok(*self.show_subtitle.read()?)
    }
}

impl IGridProperties_Impl for GalleryGridLayout_Impl {}

impl INotifyPropChanged_Impl for GalleryGridLayout_Impl {
    fn PropChanged(
        &self,
        handler: windows_core::Ref<
            windows::Foundation::TypedEventHandler<
                windows_core::IInspectable,
                IPropChangedEventArgs,
            >,
        >,
    ) -> windows_core::Result<i64> {
        self.event.add(handler.ok()?)
    }

    fn RemovePropChanged(&self, token: i64) -> windows_core::Result<()> {
        self.event.remove(token);
        Ok(())
    }
}

/// Represents possible grid layouts.
pub enum GridLayout {
    Small(ComObject<SmallGridLayout>),
    Medium(ComObject<MediumGridLayout>),
    Gallery(ComObject<GalleryGridLayout>),
}

impl From<&GridLayout> for IGridProperties {
    fn from(layout: &GridLayout) -> Self {
        match layout {
            GridLayout::Small(small) => small.to_interface(),
            GridLayout::Medium(medium) => medium.to_interface(),
            GridLayout::Gallery(gallery) => gallery.to_interface(),
        }
    }
}
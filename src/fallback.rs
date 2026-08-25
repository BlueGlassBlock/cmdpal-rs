//! Fallback command items for command items with dynamic attributes.

use std::ops::Deref;

use windows_core::{ComObject, HSTRING, IUnknownImpl, Result, implement};

use crate::cmd_item::{CommandItem, CommandItem_Impl, CommandItemBuilder};
use crate::utils::{ComBuilder, assert_send_sync};
use crate::{bindings::*, notify::*};

/// Fallback handler for command items with query-based content.
///
#[doc = include_str!("./bindings_docs/IFallbackHandler.md")]
#[implement(IFallbackHandler)]
pub struct FallbackHandler {
    querier: Box<dyn Send + Sync + Fn(HSTRING) -> Result<()>>,
}

impl FallbackHandler {
    /// Build a unmanaged fallback handler.
    pub fn new_unmanaged<F>(querier: F) -> Self
    where
        F: Send + Sync + Fn(HSTRING) -> Result<()> + 'static,
    {
        Self {
            querier: Box::new(querier),
        }
    }

    /// Build a reference-counted COM object for the fallback handler.
    pub fn new<F>(querier: F) -> ComObject<Self>
    where
        F: Send + Sync + Fn(HSTRING) -> Result<()> + 'static,
    {
        Self::new_unmanaged(querier).into()
    }
}

impl IFallbackHandler_Impl for FallbackHandler_Impl {
    fn UpdateQuery(&self, query: &HSTRING) -> Result<()> {
        (self.querier)(query.clone())
    }
}

// Builder for [`FallbackCommandItem`].
pub struct FallbackCommandItemBuilder {
    base: ComObject<CommandItem>,
    handler: ComObject<FallbackHandler>,
    title: HSTRING,
}

impl CommandItemBuilder {
    /// Creates a new builder.
    pub fn fallback(self) -> FallbackCommandItemBuilder {
        FallbackCommandItemBuilder {
            base: self.build(),
            handler: FallbackHandler::new(Box::new(|_| Ok(()))),
            title: HSTRING::new(),
        }
    }
}

impl FallbackCommandItemBuilder {
    /// Sets the handler for the fallback command item.
    pub fn handler(mut self, handler: ComObject<FallbackHandler>) -> Self {
        self.handler = handler;
        self
    }

    /// Sets the title of the fallback command item.
    pub fn title(mut self, new_title: impl Into<HSTRING>) -> Self {
        self.title = new_title.into();
        self
    }
}

impl ComBuilder for FallbackCommandItemBuilder {
    type Output = FallbackCommandItem;
    fn build_unmanaged(self) -> FallbackCommandItem {
        FallbackCommandItem {
            base: self.base,
            handler: self.handler,
            title: NotifyLock::new(self.title),
        }
    }
}

/// Fallback command item which can respond to dynamic queries.
///
/// See [`FallbackCommandItem_Impl`] for field accessors.
///
#[doc = include_str!("./bindings_docs/IFallbackCommandItem.md")]
#[implement(IFallbackCommandItem, ICommandItem, INotifyPropChanged)]
pub struct FallbackCommandItem {
    pub base: ComObject<CommandItem>,
    handler: ComObject<FallbackHandler>,
    title: NotifyLock<HSTRING>,
}

impl Deref for FallbackCommandItem {
    type Target = CommandItem_Impl;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl FallbackCommandItem_Impl {
    /// Readonly access to [`IFallbackCommandItem::Title`].
    ///
    #[doc = include_str!("./bindings_docs/IFallbackCommandItem/Title.md")]
    pub fn title(&self) -> Result<NotifyLockReadGuard<'_, HSTRING>> {
        self.title.read()
    }

    /// Mutable access to [`IFallbackCommandItem::Title`].
    ///
    #[doc = include_str!("./bindings_docs/IFallbackCommandItem/Title.md")]
    ///
    /// Notifies the host about the change when dropping the guard.
    pub fn title_mut(&self) -> Result<NotifyLockWriteGuard<'_, HSTRING>> {
        self.title.write(|| {
            self.base
                .emit_prop_changed(&self.to_interface(), "DisplayTitle")
        })
    }
}

impl IFallbackCommandItem_Impl for FallbackCommandItem_Impl {
    fn FallbackHandler(&self) -> Result<IFallbackHandler> {
        Ok(self.handler.to_interface())
    }
    fn DisplayTitle(&self) -> Result<HSTRING> {
        self.title.read().map(|s| s.clone())
    }
}

impl ICommandItem_Impl for FallbackCommandItem_Impl {
    fn Command(&self) -> Result<ICommand> {
        self.base.Command()
    }

    fn Icon(&self) -> Result<IIconInfo> {
        self.base.Icon()
    }

    fn MoreCommands(&self) -> Result<windows_core::Array<IContextItem>> {
        self.base.MoreCommands()
    }

    fn Subtitle(&self) -> Result<HSTRING> {
        self.base.Subtitle()
    }

    fn Title(&self) -> Result<HSTRING> {
        self.base.Title()
    }
}

impl INotifyPropChanged_Impl for FallbackCommandItem_Impl {
    fn PropChanged(&self, handler: RefPropChangedEventHandler<'_>) -> Result<i64> {
        self.base.PropChanged(handler)
    }

    fn RemovePropChanged(&self, token: i64) -> Result<()> {
        self.base.RemovePropChanged(token)
    }
}

const _: () = assert_send_sync::<ComObject<FallbackHandler>>();
const _: () = assert_send_sync::<ComObject<FallbackCommandItem>>();

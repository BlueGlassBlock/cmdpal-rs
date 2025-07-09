//! Pages for displaying various contents.
//!
//! See [`Content`] for supported content types.

use super::{BasePage, BasePage_Impl};
use crate::bindings::*;
use crate::content::Content;
use crate::ctx_item::ContextItem;
use crate::details::Details;
use crate::notify::*;
use crate::utils::{ComBuilder, OkOrEmpty, assert_send_sync, map_array};
use std::ops::Deref;
use windows_core::{ComObject, Event, IInspectable, IUnknownImpl as _, Result, implement};

/// Represents a content page that can display various types of content.
///
/// See [`ContentPage_Impl`] for field accessors.
///
#[doc = include_str!("../bindings_docs/IContentPage.md")]
#[implement(IContentPage, IPage, ICommand, INotifyPropChanged, INotifyItemsChanged)]
pub struct ContentPage {
    pub base: ComObject<BasePage>,
    context_menu: NotifyLock<Vec<ContextItem>>,
    contents: NotifyLock<Vec<Content>>,
    details: NotifyLock<Option<ComObject<Details>>>,
    item_event: ItemsChangedEventHandler,
}

/// Builder for [`ContentPage`].
pub struct ContentPageBuilder {
    base: ComObject<BasePage>,
    context_menu: Vec<ContextItem>,
    contents: Vec<Content>,
    details: Option<ComObject<Details>>,
}

impl ContentPageBuilder {
    /// Creates a new builder.
    pub fn new(base: ComObject<BasePage>) -> Self {
        ContentPageBuilder {
            base,
            context_menu: Vec::new(),
            contents: Vec::new(),
            details: None,
        }
    }

    /// Sets the context menu items.
    /// 
    /// First two of context menu items will become "shortcut commands" and
    /// have extra bindings (<kbd>↲</kbd> and <kbd>Ctrl + ↲</kbd>),
    /// with display text being [`ICommand::Name`] of `cmd_ctx_item.base.command`
    /// instead of [`ICommandItem::Title`] of `cmd_ctx_item.base`.
    /// 
    pub fn context_menu(mut self, context_menu: Vec<ContextItem>) -> Self {
        self.context_menu = context_menu;
        self
    }

    /// Adds a context item to the context menu.
    pub fn add_context_item(mut self, context_item: impl Into<ContextItem>) -> Self {
        self.context_menu.push(context_item.into());
        self
    }

    /// Sets the contents of the page.
    pub fn contents(mut self, contents: Vec<Content>) -> Self {
        self.contents = contents;
        self
    }

    /// Adds a content item to the page.
    pub fn add_content(mut self, content: impl Into<Content>) -> Self {
        self.contents.push(content.into());
        self
    }

    /// Sets the details for the page.
    pub fn details(mut self, details: ComObject<Details>) -> Self {
        self.details = Some(details);
        self
    }
}

impl ComBuilder for ContentPageBuilder {
    type Output = ContentPage;
    fn build_unmanaged(self) -> ContentPage {
        ContentPage {
            base: self.base,
            context_menu: NotifyLock::new(self.context_menu),
            contents: NotifyLock::new(self.contents),
            details: NotifyLock::new(self.details),
            item_event: Event::new(),
        }
    }
}

impl Deref for ContentPage {
    type Target = BasePage_Impl;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl ContentPage_Impl {
    pub(crate) fn emit_items_changed(&self, sender: IInspectable, total_items: i32) {
        let args: IItemsChangedEventArgs = ItemsChangedEventArgs(total_items).into();
        self.item_event
            .call(|handler| handler.Invoke(&sender, &args));
    }

    fn emit_self_items_changed(&self, total_items: i32) {
        self.emit_items_changed(self.to_interface(), total_items);
    }

    /// Readonly access to [`IContentPage::Commands`].
    ///
    #[doc = include_str!("../bindings_docs/IContentPage/Commands.md")]
    pub fn context_menu(&self) -> Result<NotifyLockReadGuard<'_, Vec<ContextItem>>> {
        self.context_menu.read()
    }

    /// Mutable access to [`IContentPage::Commands`].
    ///
    #[doc = include_str!("../bindings_docs/IContentPage/Commands.md")]
    ///
    /// Notifies the host about the change when dropping the guard.
    pub fn context_menu_mut(&self) -> Result<NotifyLockWriteGuard<'_, Vec<ContextItem>, usize>> {
        self.context_menu
            .write_with_peek(|v| v.len(), |len| self.emit_self_items_changed(len as i32))
    }

    /// Readonly access to [`IContentPage::GetContent`].
    ///
    #[doc = include_str!("../bindings_docs/IContentPage/GetContent.md")]
    pub fn contents(&self) -> Result<NotifyLockReadGuard<'_, Vec<Content>>> {
        self.contents.read()
    }

    /// Mutable access to [`IContentPage::GetContent`].
    ///
    #[doc = include_str!("../bindings_docs/IContentPage/GetContent.md")]
    ///
    /// Notifies the host about the change when dropping the guard.
    pub fn contents_mut(&self) -> Result<NotifyLockWriteGuard<'_, Vec<Content>, usize>> {
        self.contents
            .write_with_peek(|v| v.len(), |len| self.emit_self_items_changed(len as i32))
    }

    /// Readonly access to [`IContentPage::Details`].
    ///
    #[doc = include_str!("../bindings_docs/IContentPage/Details.md")]
    pub fn details(&self) -> Result<NotifyLockReadGuard<'_, Option<ComObject<Details>>>> {
        self.details.read()
    }

    /// Mutable access to [`IContentPage::Details`].
    ///
    #[doc = include_str!("../bindings_docs/IContentPage/Details.md")]
    ///
    /// Notifies the host about the change when dropping the guard.
    pub fn details_mut(&self) -> Result<NotifyLockWriteGuard<'_, Option<ComObject<Details>>>> {
        self.details.write(|| {
            self.base
                .base
                .emit_prop_changed(self.to_interface(), "Details")
        })
    }
}

impl IContentPage_Impl for ContentPage_Impl {
    fn Commands(&self) -> windows_core::Result<windows_core::Array<IContextItem>> {
        Ok(map_array(&self.context_menu.read()?, |x| Some(x.into())))
    }

    fn GetContent(&self) -> windows_core::Result<windows_core::Array<IContent>> {
        Ok(map_array(&self.contents.read()?, |x| Some(x.into())))
    }

    fn Details(&self) -> windows_core::Result<IDetails> {
        self.details
            .read()?
            .clone()
            .map(|x| x.to_interface())
            .ok_or_empty()
    }
}

impl INotifyItemsChanged_Impl for ContentPage_Impl {
    fn ItemsChanged(
        &self,
        handler: windows_core::Ref<
            '_,
            windows::Foundation::TypedEventHandler<
                windows_core::IInspectable,
                IItemsChangedEventArgs,
            >,
        >,
    ) -> windows_core::Result<i64> {
        self.item_event.add(handler.ok()?)
    }

    fn RemoveItemsChanged(&self, token: i64) -> windows_core::Result<()> {
        self.item_event.remove(token);
        Ok(())
    }
}

impl IPage_Impl for ContentPage_Impl {
    fn AccentColor(&self) -> windows_core::Result<OptionalColor> {
        self.base.AccentColor()
    }

    fn IsLoading(&self) -> windows_core::Result<bool> {
        self.base.IsLoading()
    }

    fn Title(&self) -> windows_core::Result<windows_core::HSTRING> {
        self.base.Title()
    }
}

impl ICommand_Impl for ContentPage_Impl {
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

impl INotifyPropChanged_Impl for ContentPage_Impl {
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

const _: () = assert_send_sync::<ComObject<ContentPage>>();

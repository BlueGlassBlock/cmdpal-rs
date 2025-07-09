//! List page which can display a scrollable list of items.

use std::ops::Deref;

use crate::{
    bindings::*,
    cmd_item::{CommandItem, CommandItem_Impl},
    details::{Details, Tag},
    filter::Filters,
    notify::*,
    utils::{ComBuilder, GridProperties, OkOrEmpty, assert_send_sync, map_array},
};
use windows_core::{ComObject, IInspectable, IUnknownImpl as _, Result, implement};
use windows_core::HSTRING;

use super::{BasePage, BasePage_Impl};

/// Represents a single item in a list.
///
/// See [`ListItem_Impl`] for field accessors.
/// 
#[doc = include_str!("../bindings_docs/IListItem.md")]
#[implement(IListItem, ICommandItem, INotifyPropChanged)]
pub struct ListItem {
    pub base: ComObject<CommandItem>,
    details: NotifyLock<Option<ComObject<Details>>>,
    tags: NotifyLock<Vec<ComObject<Tag>>>,
    section: NotifyLock<HSTRING>,
    suggestion: NotifyLock<HSTRING>,
}

/// Builder for [`ListItem`].
pub struct ListItemBuilder {
    base: ComObject<CommandItem>,
    details: Option<ComObject<Details>>,
    tags: Vec<ComObject<Tag>>,
    section: Option<HSTRING>,
    suggestion: Option<HSTRING>,
}

impl ListItemBuilder {
    /// Creates a new builder with base.
    pub fn new(base: ComObject<CommandItem>) -> Self {
        ListItemBuilder {
            base,
            details: None,
            tags: Vec::new(),
            section: None,
            suggestion: None,
        }
    }

    /// Sets the details for the list item.
    pub fn details(mut self, details: ComObject<Details>) -> Self {
        self.details = Some(details);
        self
    }

    /// Sets the tags for the list item.
    pub fn tags(mut self, tags: impl IntoIterator<Item = ComObject<Tag>>) -> Self {
        self.tags = tags.into_iter().collect();
        self
    }

    /// Adds a tag to the list item.
    pub fn add_tag(mut self, tag: ComObject<Tag>) -> Self {
        self.tags.push(tag);
        self
    }

    /// Sets the section for the list item.
    pub fn section(mut self, section: impl Into<HSTRING>) -> Self {
        self.section = Some(section.into());
        self
    }

    /// Sets the suggestion text for the list item.
    pub fn suggestion(mut self, suggestion: impl Into<HSTRING>) -> Self {
        self.suggestion = Some(suggestion.into());
        self
    }
}

impl ComBuilder for ListItemBuilder {
    type Output = ListItem;
    fn build_unmanaged(self) -> ListItem {
        ListItem {
            base: self.base,
            details: NotifyLock::new(self.details),
            tags: NotifyLock::new(self.tags),
            section: NotifyLock::new(self.section.unwrap_or_else(|| HSTRING::new())),
            suggestion: NotifyLock::new(self.suggestion.unwrap_or_else(|| HSTRING::new())),
        }
    }
}

impl Deref for ListItem {
    type Target = CommandItem_Impl;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl ListItem_Impl {
    /// Readonly access to [`IListItem::Details`].
    ///
    #[doc = include_str!("../bindings_docs/IListItem/Details.md")]
    pub fn details(&self) -> Result<NotifyLockReadGuard<'_, Option<ComObject<Details>>>> {
        self.details.read()
    }

    /// Mutable access to [`IListItem::Details`].
    ///
    #[doc = include_str!("../bindings_docs/IListItem/Details.md")]
    ///
    /// Notifies the host about the change when dropping the guard.
    pub fn details_mut(&self) -> Result<NotifyLockWriteGuard<'_, Option<ComObject<Details>>>> {
        self.details
            .write(|| self.base.emit_prop_changed(&self.to_interface(), "Details"))
    }

    /// Readonly access to [`IListItem::Tags`].
    ///
    #[doc = include_str!("../bindings_docs/IListItem/Tags.md")]
    pub fn tags(&self) -> Result<NotifyLockReadGuard<'_, Vec<ComObject<Tag>>>> {
        self.tags.read()
    }

    /// Mutable access to [`IListItem::Tags`].
    ///
    #[doc = include_str!("../bindings_docs/IListItem/Tags.md")]
    ///
    /// Notifies the host about the change when dropping the guard.
    pub fn tags_mut(&self) -> Result<NotifyLockWriteGuard<'_, Vec<ComObject<Tag>>>> {
        self.tags
            .write(|| self.base.emit_prop_changed(&self.to_interface(), "Tags"))
    }

    /// Readonly access to [`IListItem::Section`].
    ///
    #[doc = include_str!("../bindings_docs/IListItem/Section.md")]
    pub fn section(&self) -> Result<NotifyLockReadGuard<'_, HSTRING>> {
        self.section.read()
    }

    /// Mutable access to [`IListItem::Section`].
    ///
    #[doc = include_str!("../bindings_docs/IListItem/Section.md")]
    ///
    /// Notifies the host about the change when dropping the guard.
    pub fn section_mut(&self) -> Result<NotifyLockWriteGuard<'_, HSTRING>> {
        self.section
            .write(|| self.base.emit_prop_changed(&self.to_interface(), "Section"))
    }
    /// Readonly access to [`IListItem::TextToSuggest`].
    ///
    #[doc = include_str!("../bindings_docs/IListItem/TextToSuggest.md")]
    pub fn suggestion(&self) -> Result<NotifyLockReadGuard<'_, HSTRING>> {
        self.suggestion.read()
    }

    /// Mutable access to [`IListItem::TextToSuggest`].
    ///
    #[doc = include_str!("../bindings_docs/IListItem/TextToSuggest.md")]
    ///
    /// Notifies the host about the change when dropping the guard.
    pub fn suggestion_mut(&self) -> Result<NotifyLockWriteGuard<'_, HSTRING>> {
        self.suggestion.write(|| {
            self.base
                .emit_prop_changed(&self.to_interface(), "TextToSuggest")
        })
    }
}

impl IListItem_Impl for ListItem_Impl {
    fn Details(&self) -> windows_core::Result<IDetails> {
        self.details
            .read()?
            .as_ref()
            .map(|d| d.to_interface())
            .ok_or_empty()
    }
    fn Tags(&self) -> windows_core::Result<windows_core::Array<ITag>> {
        Ok(map_array(&self.tags.read()?, |t| Some(t.to_interface())))
    }
    fn Section(&self) -> windows_core::Result<windows_core::HSTRING> {
        Ok(self.section.read()?.clone())
    }
    fn TextToSuggest(&self) -> windows_core::Result<windows_core::HSTRING> {
        Ok(self.suggestion.read()?.clone())
    }
}

impl ICommandItem_Impl for ListItem_Impl {
    fn Command(&self) -> windows_core::Result<ICommand> {
        self.base.Command()
    }

    fn Icon(&self) -> windows_core::Result<IIconInfo> {
        self.base.Icon()
    }

    fn MoreCommands(&self) -> windows_core::Result<windows_core::Array<IContextItem>> {
        self.base.MoreCommands()
    }

    fn Subtitle(&self) -> windows_core::Result<windows_core::HSTRING> {
        self.base.Subtitle()
    }

    fn Title(&self) -> windows_core::Result<windows_core::HSTRING> {
        self.base.Title()
    }
}

impl INotifyPropChanged_Impl for ListItem_Impl {
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

/// Represents a page that displays a list of items.
///
/// See [`ListPage_Impl`] for field accessors.
/// 
#[doc = include_str!("../bindings_docs/IListPage.md")]
#[implement(IListPage, IPage, ICommand, INotifyPropChanged, INotifyItemsChanged)]
pub struct ListPage {
    pub base: ComObject<BasePage>,
    empty_content: NotifyLock<Option<ComObject<CommandItem>>>,
    filters: NotifyLock<Option<ComObject<Filters>>>,
    items: NotifyLock<Vec<ComObject<ListItem>>>,
    grid_properties: NotifyLock<Option<ComObject<GridProperties>>>,
    placeholder: NotifyLock<HSTRING>,
    search_text: NotifyLock<HSTRING>,
    has_more: NotifyLock<bool>,
    more_fn: Box<dyn Send + Sync + Fn(&ListPage_Impl) -> Result<()>>,
    show_details: NotifyLock<bool>,
    item_event: ItemsChangedEventHandler,
}

/// Builder for [`ListPage`].
pub struct ListPageBuilder {
    base: ComObject<BasePage>,
    empty_content: Option<ComObject<CommandItem>>,
    filters: Option<ComObject<Filters>>,
    grid_properties: Option<ComObject<GridProperties>>,
    items: Vec<ComObject<ListItem>>,
    placeholder: Option<HSTRING>,
    search_text: Option<HSTRING>,
    more_fn: Option<Box<dyn Send + Sync + Fn(&ListPage_Impl) -> Result<()>>>,
    show_details: Option<bool>,
}

impl ListPageBuilder {
    /// Creates a new builder.
    pub fn new(base: ComObject<BasePage>) -> Self {
        ListPageBuilder {
            base,
            empty_content: None,
            filters: None,
            items: Vec::new(),
            grid_properties: None,
            placeholder: None,
            search_text: None,
            more_fn: None,
            show_details: None,
        }
    }

    /// Sets the empty content for the list page.
    pub fn empty_content(mut self, empty_content: ComObject<CommandItem>) -> Self {
        self.empty_content = Some(empty_content);
        self
    }

    /// Sets the filters for the list page.
    pub fn filters(mut self, filters: ComObject<Filters>) -> Self {
        self.filters = Some(filters);
        self
    }

    /// Sets the items for the list page.
    pub fn items(mut self, items: Vec<ComObject<ListItem>>) -> Self {
        self.items = items;
        self
    }

    /// Adds an item to the list page.
    pub fn add_item(mut self, item: ComObject<ListItem>) -> Self {
        self.items.push(item);
        self
    }

    /// Sets the grid properties for the list page.
    ///
    /// The grid properties define how much space each item should take in the grid layout.
    pub fn grid_properties(mut self, grid_properties: ComObject<GridProperties>) -> Self {
        self.grid_properties = Some(grid_properties);
        self
    }

    /// Sets the placeholder text for the list page.
    pub fn placeholder(mut self, placeholder: impl Into<HSTRING>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    /// Sets the initial search text for the list page.
    pub fn search_text(mut self, search_text: impl Into<HSTRING>) -> Self {
        self.search_text = Some(search_text.into());
        self
    }

    /// Sets the function to call when more items need to be loaded.
    pub fn more_fn<F>(mut self, more_fn: F) -> Self
    where
        F: Send + Sync + Fn(&ListPage_Impl) -> Result<()> + 'static,
    {
        self.more_fn = Some(Box::new(more_fn));
        self
    }

    /// Sets whether to show details for each item in the list.
    pub fn show_details(mut self, show_details: bool) -> Self {
        self.show_details = Some(show_details);
        self
    }
}

impl ComBuilder for ListPageBuilder {
    type Output = ListPage;
    fn build_unmanaged(self) -> ListPage {
        ListPage {
            base: self.base,
            empty_content: NotifyLock::new(self.empty_content),
            filters: NotifyLock::new(self.filters),
            items: NotifyLock::new(self.items),
            grid_properties: NotifyLock::new(self.grid_properties),
            placeholder: NotifyLock::new(self.placeholder.unwrap_or_else(|| HSTRING::new())),
            search_text: NotifyLock::new(self.search_text.unwrap_or_else(|| HSTRING::new())),
            has_more: NotifyLock::new(self.more_fn.is_some()),
            more_fn: self.more_fn.unwrap_or_else(|| {
                Box::new(|page| {
                    page.has_more_mut().map(|mut guard| {
                        *guard = false;
                    })
                })
            }),
            show_details: NotifyLock::new(self.show_details.unwrap_or(false)),
            item_event: ItemsChangedEventHandler::new(),
        }
    }
}

impl Deref for ListPage {
    type Target = BasePage_Impl;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl ListPage_Impl {
    pub(crate) fn emit_self_items_changed(&self, index: i32) {
        let sender: IInspectable = self.to_interface();
        let args: IItemsChangedEventArgs = ItemsChangedEventArgs(index).into();
        self.item_event
            .call(|handler| handler.Invoke(&sender, &args));
    }

    /// Readonly access to [`IListPage::SearchText`].
    ///
    #[doc = include_str!("../bindings_docs/IListPage/SearchText.md")]
    pub fn search_text(&self) -> Result<NotifyLockReadGuard<'_, HSTRING>> {
        self.search_text.read()
    }

    /// Mutable access to [`IListPage::SearchText`].
    ///
    /// Even for `DynamicListPage`, we do not recommend updating the search text directly,
    /// as it causes confusion and may cause recurring updates if improperly handled.
    ///
    #[doc = include_str!("../bindings_docs/IListPage/SearchText.md")]
    ///
    /// Notifies the host about the change when dropping the guard.
    pub fn search_text_mut(&self) -> Result<NotifyLockWriteGuard<'_, HSTRING>> {
        self.search_text.write(|| {
            self.base
                .base
                .emit_prop_changed(self.to_interface(), "SearchText")
        })
    }

    pub(crate) fn search_text_mut_no_notify(&self) -> Result<NotifyLockWriteGuard<'_, HSTRING>> {
        self.search_text.write(|| {})
    }

    /// Readonly access to [`IListPage::EmptyContent`].
    ///
    #[doc = include_str!("../bindings_docs/IListPage/EmptyContent.md")]
    pub fn empty_content(&self) -> Result<NotifyLockReadGuard<'_, Option<ComObject<CommandItem>>>> {
        self.empty_content.read()
    }

    /// Mutable access to [`IListPage::EmptyContent`].
    ///
    #[doc = include_str!("../bindings_docs/IListPage/EmptyContent.md")]
    ///
    /// Notifies the host about the change when dropping the guard.
    pub fn empty_content_mut(
        &self,
    ) -> Result<NotifyLockWriteGuard<'_, Option<ComObject<CommandItem>>>> {
        self.empty_content.write(|| {
            self.base
                .base
                .emit_prop_changed(self.to_interface(), "EmptyContent")
        })
    }

    /// Readonly access to [`IListPage::Filters`].
    ///
    #[doc = include_str!("../bindings_docs/IListPage/Filters.md")]
    pub fn filters(&self) -> Result<NotifyLockReadGuard<'_, Option<ComObject<Filters>>>> {
        self.filters.read()
    }

    /// Mutable access to [`IListPage::Filters`].
    ///
    #[doc = include_str!("../bindings_docs/IListPage/Filters.md")]
    pub fn filters_mut(&self) -> Result<NotifyLockWriteGuard<'_, Option<ComObject<Filters>>>> {
        self.filters.write(|| {
            self.base
                .base
                .emit_prop_changed(self.to_interface(), "Filters")
        })
    }

    /// Readonly access to [`IListPage::GetItems`].
    ///
    #[doc = include_str!("../bindings_docs/IListPage/GetItems.md")]
    pub fn items(&self) -> Result<NotifyLockReadGuard<'_, Vec<ComObject<ListItem>>>> {
        self.items.read()
    }

    /// Mutable access to [`IListPage::GetItems`].
    ///
    #[doc = include_str!("../bindings_docs/IListPage/GetItems.md")]
    ///
    /// Notifies the host about the change when dropping the guard.
    pub fn items_mut(&self) -> Result<NotifyLockWriteGuard<'_, Vec<ComObject<ListItem>>, usize>> {
        self.items
            .write_with_peek(|v| v.len(), |len| self.emit_self_items_changed(len as i32))
    }

    /// Readonly access to [`IListPage::GridProperties`].
    ///
    #[doc = include_str!("../bindings_docs/IListPage/GridProperties.md")]
    pub fn grid_properties(
        &self,
    ) -> Result<NotifyLockReadGuard<'_, Option<ComObject<GridProperties>>>> {
        self.grid_properties.read()
    }

    /// Mutable access to [`IListPage::GridProperties`].
    ///
    #[doc = include_str!("../bindings_docs/IListPage/GridProperties.md")]
    ///
    /// Notifies the host about the change when dropping the guard.
    pub fn grid_properties_mut(
        &self,
    ) -> Result<NotifyLockWriteGuard<'_, Option<ComObject<GridProperties>>>> {
        self.grid_properties.write(|| {
            self.base
                .base
                .emit_prop_changed(self.to_interface(), "GridProperties")
        })
    }

    /// Readonly access to [`IListPage::HasMoreItems`].
    ///
    #[doc = include_str!("../bindings_docs/IListPage/HasMoreItems.md")]
    pub fn has_more(&self) -> Result<NotifyLockReadGuard<'_, bool>> {
        self.has_more.read()
    }

    /// Mutable access to [`IListPage::HasMoreItems`].
    ///
    #[doc = include_str!("../bindings_docs/IListPage/HasMoreItems.md")]
    ///
    /// Notifies the host about the change when dropping the guard.
    pub fn has_more_mut(&self) -> Result<NotifyLockWriteGuard<'_, bool>> {
        self.has_more.write(|| {
            self.base
                .base
                .emit_prop_changed(self.to_interface(), "HasMoreItems")
        })
    }

    /// Readonly access to [`IListPage::PlaceholderText`].
    ///
    #[doc = include_str!("../bindings_docs/IListPage/PlaceholderText.md")]
    pub fn placeholder(&self) -> Result<NotifyLockReadGuard<'_, HSTRING>> {
        self.placeholder.read()
    }

    /// Mutable access to [`IListPage::PlaceholderText`].
    ///
    #[doc = include_str!("../bindings_docs/IListPage/PlaceholderText.md")]
    ///
    /// Notifies the host about the change when dropping the guard.
    pub fn placeholder_mut(&self) -> Result<NotifyLockWriteGuard<'_, HSTRING>> {
        self.placeholder.write(|| {
            self.base
                .base
                .emit_prop_changed(self.to_interface(), "PlaceholderText")
        })
    }

    /// Readonly access to [`IListPage::ShowDetails`].
    ///
    #[doc = include_str!("../bindings_docs/IListPage/ShowDetails.md")]
    pub fn show_details(&self) -> Result<NotifyLockReadGuard<'_, bool>> {
        self.show_details.read()
    }

    /// Mutable access to [`IListPage::ShowDetails`].
    ///
    /// Notifies the host about the change when dropping the guard.
    #[doc = include_str!("../bindings_docs/IListPage/ShowDetails.md")]
    pub fn show_details_mut(&self) -> Result<NotifyLockWriteGuard<'_, bool>> {
        self.show_details.write(|| {
            self.base
                .base
                .emit_prop_changed(self.to_interface(), "ShowDetails")
        })
    }
}

impl IListPage_Impl for ListPage_Impl {
    fn EmptyContent(&self) -> windows_core::Result<ICommandItem> {
        self.empty_content
            .read()?
            .as_ref()
            .map(|c| c.to_interface())
            .ok_or_empty()
    }

    fn Filters(&self) -> windows_core::Result<IFilters> {
        self.filters
            .read()?
            .as_ref()
            .map(|f| f.to_interface())
            .ok_or_empty()
    }

    fn GetItems(&self) -> windows_core::Result<windows_core::Array<IListItem>> {
        Ok(map_array(&self.items.read()?, |x| Some(x.to_interface())))
    }

    fn GridProperties(&self) -> windows_core::Result<IGridProperties> {
        self.grid_properties
            .read()?
            .as_ref()
            .map(|g| g.to_interface())
            .ok_or_empty()
    }

    fn HasMoreItems(&self) -> windows_core::Result<bool> {
        Ok(*self.has_more.read()?)
    }

    fn LoadMore(&self) -> windows_core::Result<()> {
        (self.more_fn)(self)
    }

    fn PlaceholderText(&self) -> windows_core::Result<windows_core::HSTRING> {
        Ok(self.placeholder.read()?.clone())
    }

    fn SearchText(&self) -> windows_core::Result<windows_core::HSTRING> {
        Ok(self.search_text.read()?.clone())
    }

    fn ShowDetails(&self) -> windows_core::Result<bool> {
        Ok(*self.show_details.read()?)
    }
}

impl IPage_Impl for ListPage_Impl {
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

impl ICommand_Impl for ListPage_Impl {
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

impl INotifyPropChanged_Impl for ListPage_Impl {
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

impl INotifyItemsChanged_Impl for ListPage_Impl {
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

const _: () = assert_send_sync::<ComObject<ListPage>>();

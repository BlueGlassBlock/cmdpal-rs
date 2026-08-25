//! List page which can display a scrollable list of items.

use std::ops::Deref;

use windows_core::{Array, ComObject, HSTRING, IInspectable, IUnknownImpl, Result, implement};

use crate::cmd_item::{CommandItem, CommandItem_Impl, CommandItemBuilder};
use crate::details::{Details, Tag};
use crate::filter::{Filters, FiltersBuilder};
use crate::utils::{ComBuilder, OkOrEmpty, assert_send_sync, map_array};
use crate::{bindings::*, grid::GridLayout, notify::*, page::BasePageBuilder};

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

impl CommandItemBuilder {
    /// Creates a new builder with base.
    pub fn list(self) -> ListItemBuilder {
        ListItemBuilder {
            base: self.build(),
            details: None,
            tags: Vec::new(),
            section: None,
            suggestion: None,
        }
    }
}

impl ListItemBuilder {
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
            section: NotifyLock::new(self.section.unwrap_or_default()),
            suggestion: NotifyLock::new(self.suggestion.unwrap_or_default()),
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
    fn Details(&self) -> Result<IDetails> {
        self.details
            .read()?
            .as_ref()
            .map(|d| d.to_interface())
            .ok_or_empty()
    }
    fn Tags(&self) -> Result<Array<ITag>> {
        Ok(map_array(&self.tags.read()?, |t| Some(t.to_interface())))
    }
    fn Section(&self) -> Result<HSTRING> {
        Ok(self.section.read()?.clone())
    }
    fn TextToSuggest(&self) -> Result<HSTRING> {
        Ok(self.suggestion.read()?.clone())
    }
}

impl ICommandItem_Impl for ListItem_Impl {
    fn Command(&self) -> Result<ICommand> {
        self.base.Command()
    }

    fn Icon(&self) -> Result<IIconInfo> {
        self.base.Icon()
    }

    fn MoreCommands(&self) -> Result<Array<IContextItem>> {
        self.base.MoreCommands()
    }

    fn Subtitle(&self) -> Result<HSTRING> {
        self.base.Subtitle()
    }

    fn Title(&self) -> Result<HSTRING> {
        self.base.Title()
    }
}

impl INotifyPropChanged_Impl for ListItem_Impl {
    fn PropChanged(&self, handler: RefPropChangedEventHandler<'_>) -> Result<i64> {
        self.base.PropChanged(handler)
    }

    fn RemovePropChanged(&self, token: i64) -> Result<()> {
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
    grid_layout: NotifyLock<Option<GridLayout>>,
    placeholder: NotifyLock<HSTRING>,
    search_text: NotifyLock<HSTRING>,
    has_more: NotifyLock<bool>,
    more_fn: ListPageMoreFn,
    show_details: NotifyLock<bool>,
    item_event: ItemsChangedEventHandler,
}

type ListPageMoreFn = Box<dyn Send + Sync + Fn(&ListPage_Impl) -> Result<()>>;

/// Builder for [`ListPage`].
pub struct ListPageBuilder {
    base: ComObject<BasePage>,
    empty_content: Option<ComObject<CommandItem>>,
    filters: Option<ComObject<Filters>>,
    grid_layout: Option<GridLayout>,
    items: Vec<ComObject<ListItem>>,
    placeholder: Option<HSTRING>,
    search_text: Option<HSTRING>,
    more_fn: Option<ListPageMoreFn>,
    show_details: Option<bool>,
}

impl BasePageBuilder {
    /// Creates a [`ListPageBuilder`] builder.
    pub fn list(self) -> ListPageBuilder {
        ListPageBuilder {
            base: self.build(),
            empty_content: None,
            filters: None,
            items: Vec::new(),
            grid_layout: None,
            placeholder: None,
            search_text: None,
            more_fn: None,
            show_details: None,
        }
    }
}

impl ListPageBuilder {
    /// Sets the empty content for the list page.
    pub fn empty_content(mut self, empty_content: CommandItemBuilder) -> Self {
        self.empty_content = Some(empty_content.build());
        self
    }

    /// Sets the filters for the list page.
    pub fn filters(mut self, filters: FiltersBuilder) -> Self {
        self.filters = Some(filters.build());
        self
    }

    /// Sets the items for the list page.
    pub fn items(mut self, items: Vec<ListItemBuilder>) -> Self {
        self.items = items.into_iter().map(ListItemBuilder::build).collect();
        self
    }

    /// Adds an item to the list page.
    pub fn add_item(mut self, item: ListItemBuilder) -> Self {
        self.items.push(item.build());
        self
    }

    /// Sets the grid properties for the list page.
    ///
    /// The grid properties define how much space each item should take in the grid layout.
    pub fn grid_layout(mut self, grid_layout: impl Into<GridLayout>) -> Self {
        self.grid_layout = Some(grid_layout.into());
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
            grid_layout: NotifyLock::new(self.grid_layout),
            placeholder: NotifyLock::new(self.placeholder.unwrap_or_default()),
            search_text: NotifyLock::new(self.search_text.unwrap_or_default()),
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
    pub fn grid_layout(&self) -> Result<NotifyLockReadGuard<'_, Option<GridLayout>>> {
        self.grid_layout.read()
    }

    /// Mutable access to [`IListPage::GridProperties`].
    ///
    #[doc = include_str!("../bindings_docs/IListPage/GridProperties.md")]
    ///
    /// Notifies the host about the change when dropping the guard.
    pub fn grid_layout_mut(&self) -> Result<NotifyLockWriteGuard<'_, Option<GridLayout>>> {
        self.grid_layout.write(|| {
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
    fn EmptyContent(&self) -> Result<ICommandItem> {
        self.empty_content
            .read()?
            .as_ref()
            .map(|c| c.to_interface())
            .ok_or_empty()
    }

    fn Filters(&self) -> Result<IFilters> {
        self.filters
            .read()?
            .as_ref()
            .map(|f| f.to_interface())
            .ok_or_empty()
    }

    fn GetItems(&self) -> Result<Array<IListItem>> {
        Ok(map_array(&self.items.read()?, |x| Some(x.to_interface())))
    }

    fn GridProperties(&self) -> Result<IGridProperties> {
        self.grid_layout
            .read()?
            .as_ref()
            .map(Into::into)
            .ok_or_empty()
    }

    fn HasMoreItems(&self) -> Result<bool> {
        Ok(*self.has_more.read()?)
    }

    fn LoadMore(&self) -> Result<()> {
        (self.more_fn)(self)
    }

    fn PlaceholderText(&self) -> Result<HSTRING> {
        Ok(self.placeholder.read()?.clone())
    }

    fn SearchText(&self) -> Result<HSTRING> {
        Ok(self.search_text.read()?.clone())
    }

    fn ShowDetails(&self) -> Result<bool> {
        Ok(*self.show_details.read()?)
    }
}

impl IPage_Impl for ListPage_Impl {
    fn AccentColor(&self) -> Result<OptionalColor> {
        self.base.AccentColor()
    }

    fn IsLoading(&self) -> Result<bool> {
        self.base.IsLoading()
    }

    fn Title(&self) -> Result<HSTRING> {
        self.base.Title()
    }
}

impl ICommand_Impl for ListPage_Impl {
    fn Icon(&self) -> Result<IIconInfo> {
        self.base.Icon()
    }

    fn Id(&self) -> Result<HSTRING> {
        self.base.Id()
    }

    fn Name(&self) -> Result<HSTRING> {
        self.base.Name()
    }
}

impl INotifyPropChanged_Impl for ListPage_Impl {
    fn PropChanged(&self, handler: RefPropChangedEventHandler<'_>) -> Result<i64> {
        self.base.PropChanged(handler)
    }

    fn RemovePropChanged(&self, token: i64) -> Result<()> {
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
    ) -> Result<i64> {
        self.item_event.add(handler.ok()?)
    }

    fn RemoveItemsChanged(&self, token: i64) -> Result<()> {
        self.item_event.remove(token);
        Ok(())
    }
}

const _: () = assert_send_sync::<ComObject<ListPage>>();

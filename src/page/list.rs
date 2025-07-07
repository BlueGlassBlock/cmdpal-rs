use std::ops::Deref;

use crate::{
    bindings::*,
    cmd_item::{CommandItem, CommandItem_Impl},
    details::{Details, Tag},
    filter::Filters,
    notify::*,
    utils::{ComBuilder, GridProperties, OkOrEmpty, assert_send_sync, map_array},
};
use windows::core::{ComObject, IInspectable, IUnknownImpl as _, Result, implement};
use windows_core::HSTRING;

use super::{BasePage, BasePage_Impl};

#[implement(IListItem, ICommandItem, INotifyPropChanged)]
pub struct ListItem {
    pub base: ComObject<CommandItem>,
    details: NotifyLock<Option<ComObject<Details>>>,
    tags: NotifyLock<Vec<ComObject<Tag>>>,
    section: NotifyLock<HSTRING>,
    suggestion: NotifyLock<HSTRING>,
}

pub struct ListItemBuilder {
    base: ComObject<CommandItem>,
    details: Option<ComObject<Details>>,
    tags: Vec<ComObject<Tag>>,
    section: Option<HSTRING>,
    suggestion: Option<HSTRING>,
}

impl ListItemBuilder {
    pub fn new(base: ComObject<CommandItem>) -> Self {
        ListItemBuilder {
            base,
            details: None,
            tags: Vec::new(),
            section: None,
            suggestion: None,
        }
    }

    pub fn details(mut self, details: ComObject<Details>) -> Self {
        self.details = Some(details);
        self
    }

    pub fn tags(mut self, tags: impl IntoIterator<Item = ComObject<Tag>>) -> Self {
        self.tags = tags.into_iter().collect();
        self
    }

    pub fn add_tag(mut self, tag: ComObject<Tag>) -> Self {
        self.tags.push(tag);
        self
    }

    pub fn section(mut self, section: impl Into<HSTRING>) -> Self {
        self.section = Some(section.into());
        self
    }

    pub fn suggestion(mut self, suggestion: impl Into<HSTRING>) -> Self {
        self.suggestion = Some(suggestion.into());
        self
    }
}

impl ComBuilder for ListItemBuilder {
    type Target = ListItem;
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

    pub fn empty_content(mut self, empty_content: ComObject<CommandItem>) -> Self {
        self.empty_content = Some(empty_content);
        self
    }

    pub fn filters(mut self, filters: ComObject<Filters>) -> Self {
        self.filters = Some(filters);
        self
    }

    pub fn items(mut self, items: Vec<ComObject<ListItem>>) -> Self {
        self.items = items;
        self
    }

    pub fn add_item(mut self, item: ComObject<ListItem>) -> Self {
        self.items.push(item);
        self
    }

    pub fn grid_properties(mut self, grid_properties: ComObject<GridProperties>) -> Self {
        self.grid_properties = Some(grid_properties);
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<HSTRING>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn search_text(mut self, search_text: impl Into<HSTRING>) -> Self {
        self.search_text = Some(search_text.into());
        self
    }

    pub fn show_details(mut self, show_details: bool) -> Self {
        self.show_details = Some(show_details);
        self
    }
}

impl ComBuilder for ListPageBuilder {
    type Target = ListPage;
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
                    page.has_more_mut()
                        .map(|mut guard| {
                            *guard = false;
                            Ok(())
                        })
                        .unwrap_or_else(|e| Err(e))
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
    pub fn emit_self_items_changed(&self, index: i32) {
        let sender: IInspectable = self.to_interface();
        let args: IItemsChangedEventArgs = ItemsChangedEventArgs(index).into();
        self.item_event
            .call(|handler| handler.Invoke(&sender, &args));
    }

    pub fn search_text(&self) -> Result<NotifyLockReadGuard<'_, HSTRING>> {
        self.search_text.read()
    }

    pub fn search_text_mut(&self) -> Result<NotifyLockWriteGuard<'_, HSTRING>> {
        self.search_text.write(|| {
            self.base
                .base
                .emit_prop_changed(self.to_interface(), "SearchText")
        })
    }

    pub(crate) fn search_text_mut_no_notify(
        &self,
    ) -> Result<NotifyLockWriteGuard<'_, HSTRING>> {
        self.search_text.write(|| {})
    }

    pub fn empty_content(&self) -> Result<NotifyLockReadGuard<'_, Option<ComObject<CommandItem>>>> {
        self.empty_content.read()
    }

    pub fn empty_content_mut(
        &self,
    ) -> Result<NotifyLockWriteGuard<'_, Option<ComObject<CommandItem>>>> {
        self.empty_content.write(|| {
            self.base
                .base
                .emit_prop_changed(self.to_interface(), "EmptyContent")
        })
    }

    pub fn filters(&self) -> Result<NotifyLockReadGuard<'_, Option<ComObject<Filters>>>> {
        self.filters.read()
    }

    pub fn filters_mut(
        &self,
    ) -> Result<NotifyLockWriteGuard<'_, Option<ComObject<Filters>>>> {
        self.filters.write(|| {
            self.base
                .base
                .emit_prop_changed(self.to_interface(), "Filters")
        })
    }

    pub fn items(&self) -> Result<NotifyLockReadGuard<'_, Vec<ComObject<ListItem>>>> {
        self.items.read()
    }

    pub fn items_mut(
        &self,
    ) -> Result<NotifyLockWriteGuard<'_, Vec<ComObject<ListItem>>>> {
        self.items.write(|| self.emit_self_items_changed(-1))
    }

    pub fn grid_properties(
        &self,
    ) -> Result<NotifyLockReadGuard<'_, Option<ComObject<GridProperties>>>> {
        self.grid_properties.read()
    }

    pub fn grid_properties_mut(
        &self,
    ) -> Result<NotifyLockWriteGuard<'_, Option<ComObject<GridProperties>>>> {
        self.grid_properties.write(|| {
            self.base
                .base
                .emit_prop_changed(self.to_interface(), "GridProperties")
        })
    }

    pub fn has_more(&self) -> Result<NotifyLockReadGuard<'_, bool>> {
        self.has_more.read()
    }

    pub fn has_more_mut(&self) -> Result<NotifyLockWriteGuard<'_, bool>> {
        self.has_more.write(|| {
            self.base
                .base
                .emit_prop_changed(self.to_interface(), "HasMoreItems")
        })
    }

    pub fn placeholder(&self) -> Result<NotifyLockReadGuard<'_, HSTRING>> {
        self.placeholder.read()
    }

    pub fn placeholder_mut(&self) -> Result<NotifyLockWriteGuard<'_, HSTRING>> {
        self.placeholder.write(|| {
            self.base
                .base
                .emit_prop_changed(self.to_interface(), "PlaceholderText")
        })
    }

    pub fn show_details(&self) -> Result<NotifyLockReadGuard<'_, bool>> {
        self.show_details.read()
    }

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

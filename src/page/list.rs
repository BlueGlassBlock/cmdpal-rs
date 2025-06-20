use crate::{
    bindings::*,
    cmd_item::CommandItem,
    details::{Details, Tag},
    filter::Filters,
    notify::*,
    utils::{GridProperties, OkOrEmpty, map_array},
};
use windows::core::{ComObject, IInspectable, IUnknownImpl as _, Result, implement};
use windows_core::HSTRING;

use super::BasePage;

#[implement(IListItem, ICommandItem, INotifyPropChanged)]
pub struct ListItem {
    pub cmd_item: ComObject<CommandItem>,
    details: NotifyLock<Option<ComObject<Details>>>,
    tags: NotifyLock<Vec<ComObject<Tag>>>,
    section: NotifyLock<HSTRING>,
    suggestion: NotifyLock<HSTRING>,
}

pub struct ListItemBuilder {
    cmd_item: ComObject<CommandItem>,
    details: Option<ComObject<Details>>,
    tags: Vec<ComObject<Tag>>,
    section: Option<HSTRING>,
    suggestion: Option<HSTRING>,
}

impl ListItemBuilder {
    pub fn new(cmd_item: ComObject<CommandItem>) -> Self {
        ListItemBuilder {
            cmd_item,
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

    pub fn build_unmanaged(self) -> ListItem {
        ListItem {
            cmd_item: self.cmd_item,
            details: NotifyLock::new(self.details),
            tags: NotifyLock::new(self.tags),
            section: NotifyLock::new(self.section.unwrap_or_else(|| HSTRING::new())),
            suggestion: NotifyLock::new(self.suggestion.unwrap_or_else(|| HSTRING::new())),
        }
    }

    pub fn build(self) -> ComObject<ListItem> {
        self.build_unmanaged().into()
    }
}

impl IListItem_Impl for ListItem_Impl {
    fn Details(&self) -> windows_core::Result<IDetails> {
        self.details
            .read()?
            .as_ref()
            .map(|d| d.to_interface())
            .or_or_empty()
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
    ambassador_impl_ICommandItem_Impl! {
        body_struct(<>, ComObject<CommandItem>, cmd_item)
    }
}

impl INotifyPropChanged_Impl for ListItem_Impl {
    ambassador_impl_INotifyPropChanged_Impl! {
        body_struct(<>, ComObject<CommandItem>, cmd_item)
    }
}

#[implement(IListPage, IPage, ICommand, INotifyPropChanged, INotifyItemsChanged)]
pub struct ListPage {
    pub base: ComObject<BasePage>,
    empty_content: NotifyLock<ComObject<CommandItem>>,
    filters: NotifyLock<ComObject<Filters>>,
    items: NotifyLock<Vec<ComObject<ListItem>>>,
    grid_properties: NotifyLock<ComObject<GridProperties>>,
    placeholder: NotifyLock<HSTRING>,
    search_text: NotifyLock<HSTRING>,
    show_details: NotifyLock<bool>,
    item_event: ItemsChangedEventHandler,
}

pub struct ListPageBuilder {
    base: ComObject<BasePage>,
    empty_content: ComObject<CommandItem>,
    filters: ComObject<Filters>,
    grid_properties: ComObject<GridProperties>,
    items: Vec<ComObject<ListItem>>,
    placeholder: Option<HSTRING>,
    search_text: Option<HSTRING>,
    show_details: Option<bool>,
}

impl ListPageBuilder {
    pub fn new(
        base: ComObject<BasePage>,
        empty_content: ComObject<CommandItem>,
        filters: ComObject<Filters>,
        grid_properties: ComObject<GridProperties>,
    ) -> Self {
        ListPageBuilder {
            base,
            empty_content,
            filters,
            items: Vec::new(),
            grid_properties,
            placeholder: None,
            search_text: None,
            show_details: None,
        }
    }

    pub fn items(mut self, items: Vec<ComObject<ListItem>>) -> Self {
        self.items = items;
        self
    }

    pub fn add_item(mut self, item: ComObject<ListItem>) -> Self {
        self.items.push(item);
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

    pub fn build_unmanaged(self) -> ListPage {
        ListPage {
            base: self.base,
            empty_content: NotifyLock::new(self.empty_content),
            filters: NotifyLock::new(self.filters),
            items: NotifyLock::new(self.items),
            grid_properties: NotifyLock::new(self.grid_properties),
            placeholder: NotifyLock::new(self.placeholder.unwrap_or_else(|| HSTRING::new())),
            search_text: NotifyLock::new(self.search_text.unwrap_or_else(|| HSTRING::new())),
            show_details: NotifyLock::new(self.show_details.unwrap_or(false)),
            item_event: ItemsChangedEventHandler::new(),
        }
    }

    pub fn build(self) -> ComObject<ListPage> {
        self.build_unmanaged().into()
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

    pub fn search_text_mut(&self) -> Result<NotifyLockWriteGuard<'_, HSTRING, impl Fn()>> {
        self.search_text.write(|| {
            self.base
                .command
                .emit_prop_changed(self.to_interface(), "SearchText")
        })
    }

    pub fn empty_content(&self) -> Result<NotifyLockReadGuard<'_, ComObject<CommandItem>>> {
        self.empty_content.read()
    }

    pub fn empty_content_mut(
        &self,
    ) -> Result<NotifyLockWriteGuard<'_, ComObject<CommandItem>, impl Fn()>> {
        self.empty_content.write(|| {
            self.base
                .command
                .emit_prop_changed(self.to_interface(), "EmptyContent")
        })
    }

    pub fn filters(&self) -> Result<NotifyLockReadGuard<'_, ComObject<Filters>>> {
        self.filters.read()
    }

    pub fn filters_mut(&self) -> Result<NotifyLockWriteGuard<'_, ComObject<Filters>, impl Fn()>> {
        self.filters.write(|| {
            self.base
                .command
                .emit_prop_changed(self.to_interface(), "Filters")
        })
    }

    pub fn items(&self) -> Result<NotifyLockReadGuard<'_, Vec<ComObject<ListItem>>>> {
        self.items.read()
    }

    pub fn items_mut(
        &self,
    ) -> Result<NotifyLockWriteGuard<'_, Vec<ComObject<ListItem>>, impl Fn()>> {
        self.items.write(|| self.emit_self_items_changed(-1))
    }

    pub fn grid_properties(&self) -> Result<NotifyLockReadGuard<'_, ComObject<GridProperties>>> {
        self.grid_properties.read()
    }

    pub fn grid_properties_mut(
        &self,
    ) -> Result<NotifyLockWriteGuard<'_, ComObject<GridProperties>, impl Fn()>> {
        self.grid_properties.write(|| {
            self.base
                .command
                .emit_prop_changed(self.to_interface(), "GridProperties")
        })
    }

    pub fn placeholder(&self) -> Result<NotifyLockReadGuard<'_, HSTRING>> {
        self.placeholder.read()
    }

    pub fn placeholder_mut(&self) -> Result<NotifyLockWriteGuard<'_, HSTRING, impl Fn()>> {
        self.placeholder.write(|| {
            self.base
                .command
                .emit_prop_changed(self.to_interface(), "PlaceholderText")
        })
    }

    pub fn show_details(&self) -> Result<NotifyLockReadGuard<'_, bool>> {
        self.show_details.read()
    }

    pub fn show_details_mut(&self) -> Result<NotifyLockWriteGuard<'_, bool, impl Fn()>> {
        self.show_details.write(|| {
            self.base
                .command
                .emit_prop_changed(self.to_interface(), "ShowDetails")
        })
    }
}

impl IListPage_Impl for ListPage_Impl {
    fn EmptyContent(&self) -> windows_core::Result<ICommandItem> {
        Ok(self.empty_content.read()?.to_interface())
    }

    fn Filters(&self) -> windows_core::Result<IFilters> {
        Ok(self.filters.read()?.to_interface())
    }

    fn GetItems(&self) -> windows_core::Result<windows_core::Array<IListItem>> {
        Ok(map_array(&self.items.read()?, |x| Some(x.to_interface())))
    }

    fn GridProperties(&self) -> windows_core::Result<IGridProperties> {
        Ok(self.grid_properties.read()?.to_interface())
    }

    fn HasMoreItems(&self) -> windows_core::Result<bool> {
        Ok(false) // TODO
    }

    fn LoadMore(&self) -> windows_core::Result<()> {
        Ok(()) // TODO
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
    ambassador_impl_IPage_Impl! {
        body_struct(<>, ComObject<BasePage>, base)
    }
}

impl ICommand_Impl for ListPage_Impl {
    ambassador_impl_ICommand_Impl! {
        body_struct(<>, ComObject<BasePage>, base)
    }
}

impl INotifyPropChanged_Impl for ListPage_Impl {
    ambassador_impl_INotifyPropChanged_Impl! {
        body_struct(<>, ComObject<BasePage>, base)
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

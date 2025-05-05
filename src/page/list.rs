use crate::{
    bindings::*,
    cmd_item::CommandItem,
    details::{Details, Tag},
    filter::Filters,
    notify::*,
    utils::{GridProperties, map_array},
};
use windows::core::{ComObject, Error, IInspectable, IUnknownImpl as _, Result, implement};
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

impl IListItem_Impl for ListItem_Impl {
    fn Details(&self) -> windows_core::Result<IDetails> {
        self.details
            .read()?
            .as_ref()
            .map(|d| d.to_interface())
            .ok_or(Error::empty())
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
    base: ComObject<BasePage>,
    empty_content: NotifyLock<ComObject<CommandItem>>,
    filters: NotifyLock<ComObject<Filters>>,
    items: NotifyLock<Vec<ComObject<ListItem>>>,
    grid_properties: NotifyLock<ComObject<GridProperties>>,
    placeholder: NotifyLock<HSTRING>,
    search_text: NotifyLock<HSTRING>,
    show_details: NotifyLock<bool>,
    item_event: ItemsChangedEventHandler,
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

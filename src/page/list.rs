use crate::{
    bindings::*,
    cmd_item::CommandItem,
    details::{Details, Tag},
    filter::Filters,
    notify::*,
    utils::{map_array, GridProperties},
};
use windows::core::{
    ComObject, ComObjectInner, ComObjectInterface, Error, IUnknownImpl as _,
    Result, implement,
};
use windows_core::HSTRING;

use super::BasePage;

#[implement(IListItem, ICommandItem, INotifyPropChanged)]
pub struct ListItem<TC>
where
    TC: ComObjectInner + 'static,
    TC::Outer: ICommand_Impl + ComObjectInterface<ICommand>,
{
    cmd_item: ComObject<CommandItem<TC>>,
    details: NotifyLock<Option<ComObject<Details>>>,
    tags: NotifyLock<Vec<ComObject<Tag>>>,
    section: NotifyLock<HSTRING>,
    suggestion: NotifyLock<HSTRING>,
}

impl<TC> IListItem_Impl for ListItem_Impl<TC>
where
    TC: ComObjectInner + 'static,
    TC::Outer: ICommand_Impl + ComObjectInterface<ICommand>,
{
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

impl<TC> ICommandItem_Impl for ListItem_Impl<TC>
where
    TC: ComObjectInner + 'static,
    TC::Outer: ICommand_Impl + ComObjectInterface<ICommand>,
{
    ambassador_impl_ICommandItem_Impl! {
        body_struct(<>, ComObject<CommandItem<TC>>, cmd_item)
    }
}

impl<TC> INotifyPropChanged_Impl for ListItem_Impl<TC>
where
    TC: ComObjectInner + 'static,
    TC::Outer: ICommand_Impl + ComObjectInterface<ICommand>,
{
    ambassador_impl_INotifyPropChanged_Impl! {
        body_struct(<>, ComObject<CommandItem<TC>>, cmd_item)
    }
}

#[implement(IListPage, IPage, ICommand, INotifyPropChanged, INotifyItemsChanged)]
pub struct ListPage {
    base: ComObject<BasePage>,
    empty_content: NotifyLock<ICommandItem>, // TODO
    filters: NotifyLock<ComObject<Filters>>,
    items: NotifyLock<Vec<IListItem>>, // TODO
    grid_properties: NotifyLock<ComObject<GridProperties>>,
    placeholder: NotifyLock<HSTRING>,
    search_text: NotifyLock<HSTRING>,
    show_details: NotifyLock<bool>,
    item_event: ItemsChangedEventHandler,
}

impl ListPage_Impl {
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
}

impl IListPage_Impl for ListPage_Impl {
    fn EmptyContent(&self) -> windows_core::Result<ICommandItem> {
        Ok(self.empty_content.read()?.clone())
    }

    fn Filters(&self) -> windows_core::Result<IFilters> {
        Ok(self.filters.read()?.to_interface())
    }

    fn GetItems(&self) -> windows_core::Result<windows_core::Array<IListItem>> {
        Ok(map_array(&self.items.read()?, |x| x.clone().into()))
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

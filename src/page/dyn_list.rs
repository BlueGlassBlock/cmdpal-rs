use std::ops::Deref;

use crate::{
    bindings::*,
    page::list::ListPage_Impl,
    utils::{ComBuilder, assert_send_sync},
};
use windows::core::{ComObject, HSTRING, Result, implement};

use super::list::ListPage;

pub type SearchTextUpdateBox =
    Box<dyn Send + Sync + Fn(&DynamicListPage_Impl, HSTRING, HSTRING) -> Result<()>>;

#[implement(
    IDynamicListPage,
    IListPage,
    IPage,
    INotifyItemsChanged,
    INotifyPropChanged,
    ICommand
)]
pub struct DynamicListPage {
    pub base: ComObject<ListPage>,
    update_fn: SearchTextUpdateBox,
}

pub struct DynamicListPageBuilder {
    base: ComObject<ListPage>,
    update_fn: SearchTextUpdateBox,
}

impl DynamicListPageBuilder {
    pub fn new(base: ComObject<ListPage>) -> Self {
        DynamicListPageBuilder {
            base,
            update_fn: Box::new(|_, _, _| Ok(())),
        }
    }

    pub fn update_fn<F>(mut self, update_fn: F) -> Self
    where
        F: Send + Sync + Fn(&DynamicListPage_Impl, HSTRING, HSTRING) -> Result<()> + 'static,
    {
        self.update_fn = Box::new(update_fn);
        self
    }
}

impl ComBuilder for DynamicListPageBuilder {
    type Output = DynamicListPage;
    fn build_unmanaged(self) -> DynamicListPage {
        DynamicListPage {
            base: self.base,
            update_fn: self.update_fn,
        }
    }
}

impl Deref for DynamicListPage {
    type Target = ListPage_Impl;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl IDynamicListPage_Impl for DynamicListPage_Impl {
    fn SetSearchText(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let old = self.base.search_text()?.clone();
        let mut guard = self.base.search_text_mut_no_notify()?;
        *guard = value.clone();
        let new = value.clone();
        (self.update_fn)(self, old, new)?;
        Ok(())
    }
}

impl IListPage_Impl for DynamicListPage_Impl {
    fn EmptyContent(&self) -> windows_core::Result<ICommandItem> {
        self.base.EmptyContent()
    }

    fn Filters(&self) -> windows_core::Result<IFilters> {
        self.base.Filters()
    }

    fn GetItems(&self) -> windows_core::Result<windows_core::Array<IListItem>> {
        self.base.GetItems()
    }

    fn GridProperties(&self) -> windows_core::Result<IGridProperties> {
        self.base.GridProperties()
    }

    fn HasMoreItems(&self) -> windows_core::Result<bool> {
        self.base.HasMoreItems()
    }

    fn LoadMore(&self) -> windows_core::Result<()> {
        self.base.LoadMore()
    }

    fn PlaceholderText(&self) -> windows_core::Result<windows_core::HSTRING> {
        self.base.PlaceholderText()
    }

    fn SearchText(&self) -> windows_core::Result<windows_core::HSTRING> {
        self.base.SearchText()
    }

    fn ShowDetails(&self) -> windows_core::Result<bool> {
        self.base.ShowDetails()
    }
}

impl IPage_Impl for DynamicListPage_Impl {
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

impl INotifyItemsChanged_Impl for DynamicListPage_Impl {
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
        self.base.ItemsChanged(handler)
    }

    fn RemoveItemsChanged(&self, token: i64) -> windows_core::Result<()> {
        self.base.RemoveItemsChanged(token)
    }
}

impl ICommand_Impl for DynamicListPage_Impl {
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

impl INotifyPropChanged_Impl for DynamicListPage_Impl {
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

const _: () = assert_send_sync::<ComObject<DynamicListPage>>();

use crate::bindings::*;
use windows::core::{implement, ComObject, Result, HSTRING};

use super::list::ListPage;

pub type SearchTextUpdateFn = Box<dyn Fn(&DynamicListPage_Impl, HSTRING, HSTRING) -> Result<()>>;

#[implement(IDynamicListPage, IListPage, IPage, INotifyItemsChanged, INotifyPropChanged, ICommand)]
pub struct DynamicListPage {
    pub list_page: ComObject<ListPage>,
    update_fn: SearchTextUpdateFn,
}

impl IDynamicListPage_Impl for DynamicListPage_Impl {
    fn SetSearchText(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let old = self.list_page.search_text()?.clone();
        let new = value.clone();
        (self.update_fn)(self, old, new)?;
        let mut guard = self.list_page.search_text_mut()?;
        *guard = value.clone();
        Ok(())
    }
}

impl IListPage_Impl for DynamicListPage_Impl {
    ambassador_impl_IListPage_Impl! {
        body_struct(< >, ComObject<ListPage>, list_page)
    }
}

impl IPage_Impl for DynamicListPage_Impl {
    ambassador_impl_IPage_Impl! {
        body_struct(< >, ComObject<ListPage>, list_page)
    }
}

impl INotifyItemsChanged_Impl for DynamicListPage_Impl {
    ambassador_impl_INotifyItemsChanged_Impl! {
        body_struct(< >, ComObject<ListPage>, list_page)
    }
}

impl INotifyPropChanged_Impl for DynamicListPage_Impl {
    ambassador_impl_INotifyPropChanged_Impl! {
        body_struct(< >, ComObject<ListPage>, list_page)
    }
}

impl ICommand_Impl for DynamicListPage_Impl {
    ambassador_impl_ICommand_Impl! {
        body_struct(< >, ComObject<ListPage>, list_page)
    }
}


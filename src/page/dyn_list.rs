use std::ops::Deref;

use crate::{
    bindings::*,
    page::list::ListPage_Impl,
    utils::{ComBuilder, assert_send_sync},
};
use windows::core::{ComObject, HSTRING, Result, implement};

use super::list::ListPage;

pub type SearchTextUpdateFn =
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
    update_fn: SearchTextUpdateFn,
}

pub struct DynamicListPageBuilder {
    base: ComObject<ListPage>,
    update_fn: SearchTextUpdateFn,
}

impl DynamicListPageBuilder {
    pub fn new(base: ComObject<ListPage>) -> Self {
        DynamicListPageBuilder {
            base,
            update_fn: Box::new(|_, _, _| Ok(())),
        }
    }

    pub fn update_fn(mut self, update_fn: SearchTextUpdateFn) -> Self {
        self.update_fn = update_fn;
        self
    }
}

impl ComBuilder for DynamicListPageBuilder {
    type Target = DynamicListPage;
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
    ambassador_impl_IListPage_Impl! {
        body_struct(< >, ComObject<ListPage>, base)
    }
}

impl IPage_Impl for DynamicListPage_Impl {
    ambassador_impl_IPage_Impl! {
        body_struct(< >, ComObject<ListPage>, base)
    }
}

impl INotifyItemsChanged_Impl for DynamicListPage_Impl {
    ambassador_impl_INotifyItemsChanged_Impl! {
        body_struct(< >, ComObject<ListPage>, base)
    }
}

impl INotifyPropChanged_Impl for DynamicListPage_Impl {
    ambassador_impl_INotifyPropChanged_Impl! {
        body_struct(< >, ComObject<ListPage>, base)
    }
}

impl ICommand_Impl for DynamicListPage_Impl {
    ambassador_impl_ICommand_Impl! {
        body_struct(< >, ComObject<ListPage>, base)
    }
}

const _: () = assert_send_sync::<ComObject<DynamicListPage>>();

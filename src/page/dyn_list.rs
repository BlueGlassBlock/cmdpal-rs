//! Dynamic list page that can customize items based on search text.

use windows_core::{ComObject, HSTRING, Result};

use crate::bindings::*;
use crate::page::list::{ListPage_Impl, ListPageBuilder};
use crate::utils::{ComBuilder, assert_send_sync};

use super::list::ListPage;

pub type SearchTextUpdateBox =
    Box<dyn Send + Sync + Fn(&DynamicListPage_Impl, HSTRING, HSTRING) -> Result<()>>;

/// Dynamic list page that can customize items based on search text.
///
#[doc = include_str!("../bindings_docs/IDynamicListPage.md")]
#[windows_core::implement(
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

/// Builder for [`DynamicListPage`].
pub struct DynamicListPageBuilder {
    base: ComObject<ListPage>,
    update_fn: SearchTextUpdateBox,
}

impl ListPageBuilder {
    /// Creates a new builder.
    pub fn dynamic(self) -> DynamicListPageBuilder {
        DynamicListPageBuilder {
            base: self.build(),
            update_fn: Box::new(|_, _, _| Ok(())),
        }
    }
}

impl DynamicListPageBuilder {
    /// Sets the update function for search text changes.
    ///
    /// The update function takes (self, old_search_text, new_search_text) as parameters.
    ///
    /// # Note
    ///
    /// The update function should block as little as possible, else the subsequent updates will clutter,
    /// causing incorrect ordering of search text updates.
    ///
    /// Solutions include:
    /// - Fire a unblocking task to handle the update, and cancel the previous task if it is still running.
    /// - Delegate updates to a background thread, and prioritize the latest chronological update.
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

impl std::ops::Deref for DynamicListPage {
    type Target = ListPage_Impl;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl IDynamicListPage_Impl for DynamicListPage_Impl {
    fn SetSearchText(&self, value: &HSTRING) -> Result<()> {
        let old = self.base.search_text()?.clone();
        let mut guard = self.base.search_text_mut_no_notify()?;
        *guard = value.clone();
        let new = value.clone();
        (self.update_fn)(self, old, new)?;
        Ok(())
    }
}

impl IListPage_Impl for DynamicListPage_Impl {
    fn EmptyContent(&self) -> Result<ICommandItem> {
        self.base.EmptyContent()
    }

    fn Filters(&self) -> Result<IFilters> {
        self.base.Filters()
    }

    fn GetItems(&self) -> Result<windows_core::Array<IListItem>> {
        self.base.GetItems()
    }

    fn GridProperties(&self) -> Result<IGridProperties> {
        self.base.GridProperties()
    }

    fn HasMoreItems(&self) -> Result<bool> {
        self.base.HasMoreItems()
    }

    fn LoadMore(&self) -> Result<()> {
        self.base.LoadMore()
    }

    fn PlaceholderText(&self) -> Result<HSTRING> {
        self.base.PlaceholderText()
    }

    fn SearchText(&self) -> Result<HSTRING> {
        self.base.SearchText()
    }

    fn ShowDetails(&self) -> Result<bool> {
        self.base.ShowDetails()
    }
}

impl IPage_Impl for DynamicListPage_Impl {
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
    ) -> Result<i64> {
        self.base.ItemsChanged(handler)
    }

    fn RemoveItemsChanged(&self, token: i64) -> Result<()> {
        self.base.RemoveItemsChanged(token)
    }
}

impl ICommand_Impl for DynamicListPage_Impl {
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

impl INotifyPropChanged_Impl for DynamicListPage_Impl {
    fn PropChanged(&self, handler: crate::notify::RefPropChangedEventHandler<'_>) -> Result<i64> {
        self.base.PropChanged(handler)
    }

    fn RemovePropChanged(&self, token: i64) -> Result<()> {
        self.base.RemovePropChanged(token)
    }
}

const _: () = assert_send_sync::<ComObject<DynamicListPage>>();

//! Types for filtering a list in Command Palette.
//!
//! This module currently doesn't work: <https://github.com/microsoft/PowerToys/issues/38318>

use crate::icon::IconInfo;
use crate::utils::{ComBuilder, assert_send_sync, map_array};
use crate::{bindings::*, utils::OkOrEmpty};
use std::sync::RwLock;
use windows::{
    Win32::Foundation::ERROR_LOCK_VIOLATION,
    core::{ComObject, HSTRING, implement},
};
use windows_core::{Error, Result};

/// Represents a separator in the filter list.
/// 
#[doc = include_str!("./bindings_docs/ISeparatorFilterItem.md")]
#[implement(ISeparatorFilterItem, IFilterItem)]
pub struct FilterSeparator;

impl ISeparatorFilterItem_Impl for FilterSeparator_Impl {}
impl IFilterItem_Impl for FilterSeparator_Impl {}

/// Represents a selectable filter item in the filter list.
///
#[doc = include_str!("./bindings_docs/IFilter.md")]
#[implement(IFilter, IFilterItem)]
pub struct Filter {
    #[doc = include_str!("./bindings_docs/IFilter/Icon.md")]
    pub icon: Option<ComObject<IconInfo>>,
    #[doc = include_str!("./bindings_docs/IFilter/Id.md")]
    pub id: HSTRING,
    #[doc = include_str!("./bindings_docs/IFilter/Name.md")]
    pub name: HSTRING,
}

impl IFilter_Impl for Filter_Impl {
    fn Icon(&self) -> windows_core::Result<IIconInfo> {
        self.icon
            .as_ref()
            .map(|icon| icon.to_interface())
            .ok_or_empty()
    }

    fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        Ok(self.id.clone())
    }

    fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        Ok(self.name.clone())
    }
}
impl IFilterItem_Impl for Filter_Impl {}

/// A filter item that can be used to build [`Filters`] struct.
pub enum FilterItem {
    /// A separator item that can be used to separate between different filter sections.
    Separator(ComObject<FilterSeparator>),
    /// An actual filter that can be used to filter a list.
    Filter(ComObject<Filter>),
}

impl From<&FilterItem> for IFilterItem {
    fn from(item: &FilterItem) -> Self {
        match item {
            FilterItem::Separator(item) => item.to_interface(),
            FilterItem::Filter(item) => item.to_interface(),
        }
    }
}

/// A collection of filters that can be used to filter a list.
#[implement(IFilters)]
pub struct Filters {
    items: Vec<FilterItem>,
    current: RwLock<Option<ComObject<Filter>>>,
    on_update: Box<
        dyn Send + Sync + Fn(Option<ComObject<Filter>>, Option<ComObject<Filter>>) -> Result<()>,
    >,
}

/// Builder for [`Filters`].
pub struct FiltersBuilder {
    items: Vec<FilterItem>,
    on_update: Box<
        dyn Send + Sync + Fn(Option<ComObject<Filter>>, Option<ComObject<Filter>>) -> Result<()>,
    >,
}

impl FiltersBuilder {
    /// Creates a new empty [`FiltersBuilder`].
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            on_update: Box::new(|_, _| Ok(())),
        }
    }

    /// Add a [`FilterItem`].
    pub fn add(mut self, item: FilterItem) -> Self {
        self.items.push(item);
        self
    }

    /// Add a [`Filter`].
    pub fn add_filter(mut self, item: ComObject<Filter>) -> Self {
        self.items.push(FilterItem::Filter(item));
        self
    }

    /// Add a [`FilterSeparator`].
    pub fn add_separator(mut self) -> Self {
        self.items
            .push(FilterItem::Separator(ComObject::new(FilterSeparator)));
        self
    }

    /// Set the callback that will be called when the current filter is updated.
    ///
    /// The callback should accept old and new current filter items,
    /// Update the list of items based on the new filter,
    /// and return a `Result<()>`.
    pub fn on_update<F>(mut self, func: F) -> Self
    where
        F: Send
            + Sync
            + Fn(Option<ComObject<Filter>>, Option<ComObject<Filter>>) -> Result<()>
            + 'static,
    {
        self.on_update = Box::new(func);
        self
    }
}

impl ComBuilder for FiltersBuilder {
    type Output = Filters;
    fn build_unmanaged(self) -> Self::Output {
        Filters {
            items: self.items,
            current: RwLock::new(None),
            on_update: self.on_update,
        }
    }
}

impl IFilters_Impl for Filters_Impl {
    fn CurrentFilterId(&self) -> windows_core::Result<windows_core::HSTRING> {
        self.current
            .read()
            .map_err(|_| Error::from(ERROR_LOCK_VIOLATION))?
            .as_ref()
            .map(|item| item.id.clone())
            .ok_or_empty()
    }

    fn Filters(&self) -> windows_core::Result<windows_core::Array<IFilterItem>> {
        Ok(map_array(&self.items, |filter| {
            Some(IFilterItem::from(filter))
        }))
    }

    fn SetCurrentFilterId(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let mut guard = self
            .current
            .write()
            .map_err(|_| Error::from(ERROR_LOCK_VIOLATION))?;
        let old = guard.clone();
        let mut new = None;
        for filter in self.items.iter() {
            match filter {
                FilterItem::Separator(_) => continue,
                FilterItem::Filter(item) => {
                    if item.id == *value {
                        new = Some(item.clone());
                        break;
                    }
                }
            }
        }
        *guard = new.clone();
        drop(guard);
        (self.on_update)(old, new)
    }
}

const _: () = assert_send_sync::<Filter>();
const _: () = assert_send_sync::<ComObject<Filters>>();

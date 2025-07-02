//! This module currently doesn't work: https://github.com/microsoft/PowerToys/issues/38318

use crate::icon::IconInfo;
use crate::utils::{ComBuilder, assert_send_sync, map_array};
use crate::{bindings::*, utils::OkOrEmpty};
use std::sync::RwLock;
use windows::{
    Win32::Foundation::ERROR_LOCK_VIOLATION,
    core::{ComObject, HSTRING, implement},
};
use windows_core::Error;

#[implement(ISeparatorFilterItem, IFilterItem)]
pub struct SeparatorFilterItem;

impl ISeparatorFilterItem_Impl for SeparatorFilterItem_Impl {}
impl IFilterItem_Impl for SeparatorFilterItem_Impl {}

#[implement(IFilter, IFilterItem)]
pub struct FilterItem {
    pub icon: Option<ComObject<IconInfo>>,
    pub id: HSTRING,
    pub name: HSTRING,
}

impl IFilter_Impl for FilterItem_Impl {
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
impl IFilterItem_Impl for FilterItem_Impl {}

pub enum Filter {
    Separator(ComObject<SeparatorFilterItem>),
    Item(ComObject<FilterItem>),
}

impl From<&Filter> for IFilterItem {
    fn from(item: &Filter) -> Self {
        match item {
            Filter::Separator(item) => item.to_interface(),
            Filter::Item(item) => item.to_interface(),
        }
    }
}

#[implement(IFilters)]
pub struct Filters {
    filters: Vec<Filter>,
    index: RwLock<usize>,
}

pub struct FiltersBuilder {
    filters: Vec<Filter>,
}

impl FiltersBuilder {
    pub fn new() -> Self {
        Self {
            filters: Vec::new(),
        }
    }

    pub fn add(mut self, item: Filter) -> Self {
        self.filters.push(item);
        self
    }

    pub fn add_item(mut self, item: ComObject<FilterItem>) -> Self {
        self.filters.push(Filter::Item(item));
        self
    }

    pub fn add_separator(mut self) -> Self {
        self.filters
            .push(Filter::Separator(ComObject::new(SeparatorFilterItem)));
        self
    }
}

impl ComBuilder for FiltersBuilder {
    type Target = Filters;
    fn build_unmanaged(self) -> Self::Target {
        Filters {
            filters: self.filters,
            index: RwLock::new(0),
        }
    }
}

impl IFilters_Impl for Filters_Impl {
    fn CurrentFilterId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let filter = self
            .filters
            .get(
                *self
                    .index
                    .read()
                    .map_err(|_| Error::from(ERROR_LOCK_VIOLATION))?,
            )
            .ok_or_empty()?;
        match filter {
            Filter::Separator(_) => Err(Error::empty()),
            Filter::Item(item) => item.Id(),
        }
    }

    fn Filters(&self) -> windows_core::Result<windows_core::Array<IFilterItem>> {
        Ok(map_array(&self.filters, |filter| {
            Some(IFilterItem::from(filter))
        }))
    }

    fn SetCurrentFilterId(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        // TODO: inform user with a function that the filter is changed, so that they can change list item accordingly
        // We will resolve and call the filter update function with ComObject<FilterItem> then.
        for (i, filter) in self.filters.iter().enumerate() {
            match filter {
                Filter::Separator(_) => continue,
                Filter::Item(item) => {
                    if item.Id()? == *value {
                        let mut guard = self
                            .index
                            .write()
                            .map_err(|_| Error::from(ERROR_LOCK_VIOLATION))?;
                        *guard = i;
                        return Ok(());
                    }
                }
            }
        }
        Ok(())
    }
}

const _: () = assert_send_sync::<Filter>();
const _: () = assert_send_sync::<ComObject<Filters>>();

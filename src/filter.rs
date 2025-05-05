use crate::bindings::*;
use crate::icon::IconInfo;
use crate::utils::map_array;
use std::sync::RwLock;
use windows::{core::{implement, ComObject, HSTRING}, Win32::Foundation::ERROR_LOCK_VIOLATION};
use windows_core::Error;

#[implement(ISeparatorFilterItem, IFilterItem)]
pub struct SeparatorFilterItem;

impl ISeparatorFilterItem_Impl for SeparatorFilterItem_Impl {}
impl IFilterItem_Impl for SeparatorFilterItem_Impl {}

#[implement(IFilter, IFilterItem)]
pub struct FilterItem {
    icon: Option<ComObject<IconInfo>>,
    id: HSTRING,
    name: HSTRING,
}

impl IFilter_Impl for FilterItem_Impl {
    fn Icon(&self) -> windows_core::Result<IIconInfo> {
        self.icon
            .as_ref()
            .map(|icon| icon.to_interface())
            .ok_or(windows_core::Error::empty())
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

impl IFilters_Impl for Filters_Impl {
    fn CurrentFilterId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let filter = self
            .filters
            .get(*self.index.read().map_err(|_| Error::from(ERROR_LOCK_VIOLATION))?)
            .ok_or(Error::empty())?;
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
        for (i, filter) in self.filters.iter().enumerate() {
            match filter {
                Filter::Separator(_) => continue,
                Filter::Item(item) => {
                    if item.Id()? == *value {
                        let mut guard = self.index.write().map_err(|_| Error::from(ERROR_LOCK_VIOLATION))?;
                        *guard = i;
                        return Ok(());
                    }
                }
            }
        }
        Ok(())
    }
}

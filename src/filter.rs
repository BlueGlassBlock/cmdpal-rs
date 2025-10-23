//! Types for filtering a list in Command Palette.
//!
//! This module currently doesn't work: <https://github.com/microsoft/PowerToys/issues/38318>

use crate::bindings::*;
use crate::icon::IconInfo;
use crate::notify::*;
use crate::utils::{ComBuilder, OkOrEmpty, assert_send_sync, map_array};
use std::sync::RwLock;
use windows::Win32::Foundation::ERROR_LOCK_VIOLATION;
use windows_core::{
    ComObject, Error, Event, HSTRING, IInspectable, IUnknownImpl as _, Result, implement,
};

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
#[implement(IFilter, IFilterItem, INotifyPropChanged)]
pub struct Filter {
    name: NotifyLock<HSTRING>,
    id: NotifyLock<HSTRING>,
    icon: NotifyLock<Option<ComObject<IconInfo>>>,
    event: PropChangedEventHandler,
}

/// Builder for [`Filter`].
pub struct FilterBuilder {
    name: HSTRING,
    id: HSTRING,
    icon: Option<ComObject<IconInfo>>,
}

impl FilterBuilder {
    /// Creates a new builder.
    pub fn new() -> Self {
        Self {
            name: HSTRING::new(),
            id: HSTRING::new(),
            icon: None,
        }
    }

    /// Sets the name of the filter.
    pub fn name(mut self, name: impl Into<HSTRING>) -> Self {
        self.name = name.into();
        self
    }

    /// Sets the unique identifier of the filter.
    pub fn id(mut self, id: impl Into<HSTRING>) -> Self {
        self.id = id.into();
        self
    }

    /// Sets the icon for the filter.
    pub fn icon(mut self, icon: ComObject<IconInfo>) -> Self {
        self.icon = Some(icon);
        self
    }
}

impl ComBuilder for FilterBuilder {
    type Output = Filter;
    fn build_unmanaged(self) -> Filter {
        Filter {
            name: NotifyLock::new(self.name),
            id: NotifyLock::new(self.id),
            icon: NotifyLock::new(self.icon),
            event: Event::new(),
        }
    }
}

impl Default for FilterBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl IFilter_Impl for Filter_Impl {
    fn Icon(&self) -> windows_core::Result<IIconInfo> {
        self.icon
            .read()?
            .as_ref()
            .map(|icon| icon.to_interface())
            .ok_or_empty()
    }

    fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        self.id.read().map(|id| id.clone())
    }

    fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        self.name.read().map(|name| name.clone())
    }
}

impl INotifyPropChanged_Impl for Filter_Impl {
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
        self.event.add(handler.ok()?)
    }

    fn RemovePropChanged(&self, token: i64) -> windows_core::Result<()> {
        self.event.remove(token);
        Ok(())
    }
}

impl Filter_Impl {
    pub(crate) fn emit_prop_changed(&self, sender: IInspectable, prop: &str) {
        let args: IPropChangedEventArgs = PropChangedEventArgs(prop.into()).into();
        self.event
            .call(|handler| handler.Invoke(&sender, &args.clone()));
    }

    fn emit_self_prop_changed(&self, prop: &str) {
        self.emit_prop_changed(self.to_interface(), prop);
    }

    /// Readonly access to [`IFilter::Name`].
    ///
    #[doc = include_str!("./bindings_docs/IFilter/Name.md")]
    pub fn name(&self) -> windows_core::Result<NotifyLockReadGuard<'_, windows_core::HSTRING>> {
        self.name.read()
    }

    /// Mutable access to [`IFilter::Name`].
    ///
    #[doc = include_str!("./bindings_docs/IFilter/Name.md")]
    ///
    /// Notifies the host about the property change when dropping the guard.
    pub fn name_mut(
        &self,
    ) -> windows_core::Result<NotifyLockWriteGuard<'_, windows_core::HSTRING>> {
        self.name.write(|| self.emit_self_prop_changed("Name"))
    }

    /// Readonly access to [`IFilter::Id`].
    ///
    #[doc = include_str!("./bindings_docs/IFilter/Id.md")]
    pub fn id(&self) -> windows_core::Result<NotifyLockReadGuard<'_, windows_core::HSTRING>> {
        self.id.read()
    }

    /// Mutable access to [`IFilter::Id`].
    ///
    #[doc = include_str!("./bindings_docs/IFilter/Id.md")]
    ///
    /// Notifies the host about the property change when dropping the guard.
    pub fn id_mut(&self) -> windows_core::Result<NotifyLockWriteGuard<'_, windows_core::HSTRING>> {
        self.id.write(|| self.emit_self_prop_changed("Id"))
    }

    /// Readonly access to [`IFilter::Icon`].
    ///
    #[doc = include_str!("./bindings_docs/IFilter/Icon.md")]
    pub fn icon(
        &self,
    ) -> windows_core::Result<NotifyLockReadGuard<'_, Option<ComObject<IconInfo>>>> {
        self.icon.read()
    }

    /// Mutable access to [`IFilter::Icon`].
    ///
    #[doc = include_str!("./bindings_docs/IFilter/Icon.md")]
    ///
    /// Notifies the host about the property change when dropping the guard.
    pub fn icon_mut(
        &self,
    ) -> windows_core::Result<NotifyLockWriteGuard<'_, Option<ComObject<IconInfo>>>> {
        self.icon.write(|| self.emit_self_prop_changed("Icon"))
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
            .ok_or_empty()?
            .id()
            .map(|id| id.clone())
    }

    fn GetFilters(&self) -> windows_core::Result<windows_core::Array<IFilterItem>> {
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
                    if let Some(id) = item.id().ok()
                        && *id == *value
                    {
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

use super::cmd_item::CommandItem;
use crate::bindings::*;
use crate::notify::*;
use crate::utils::{ComBuilder, assert_send_sync};
use windows::core::{ComObject, HSTRING, IUnknownImpl as _, Result, implement};

#[implement(IFallbackHandler)]
pub struct FallbackHandler {
    querier: Box<dyn Send + Sync + Fn(HSTRING) -> Result<()>>,
}

impl FallbackHandler {
    pub fn new_unmanaged(querier: Box<dyn Send + Sync + Fn(HSTRING) -> Result<()>>) -> Self {
        Self { querier }
    }

    pub fn new(querier: Box<dyn Send + Sync + Fn(HSTRING) -> Result<()>>) -> ComObject<Self> {
        Self::new_unmanaged(querier).into()
    }
}

impl IFallbackHandler_Impl for FallbackHandler_Impl {
    fn UpdateQuery(&self, query: &windows_core::HSTRING) -> windows_core::Result<()> {
        (self.querier)(query.clone())
    }
}

pub struct FallbackCommandItemBuilder {
    base: ComObject<CommandItem>,
    handler: ComObject<FallbackHandler>,
    title: HSTRING,
}

impl FallbackCommandItemBuilder {
    pub fn new(base: ComObject<CommandItem>, handler: ComObject<FallbackHandler>) -> Self {
        Self {
            base,
            handler,
            title: HSTRING::new(),
        }
    }

    pub fn title(mut self, new_title: impl Into<HSTRING>) -> Self {
        self.title = new_title.into();
        self
    }
}

impl ComBuilder<FallbackCommandItem> for FallbackCommandItemBuilder {
    fn build_unmanaged(self) -> FallbackCommandItem {
        FallbackCommandItem {
            base: self.base,
            handler: self.handler,
            title: NotifyLock::new(self.title),
        }
    }
}

#[implement(IFallbackCommandItem, ICommandItem, INotifyPropChanged)]
pub struct FallbackCommandItem {
    pub base: ComObject<CommandItem>,
    handler: ComObject<FallbackHandler>,
    title: NotifyLock<HSTRING>,
}

impl FallbackCommandItem_Impl {
    pub fn title(&self) -> Result<NotifyLockReadGuard<'_, HSTRING>> {
        self.title.read()
    }

    pub fn title_mut(&self) -> Result<NotifyLockWriteGuard<'_, HSTRING, impl Fn()>> {
        self.title.write(|| {
            self.base
                .emit_prop_changed(&self.to_interface(), "DisplayTitle")
        })
    }
}

impl IFallbackCommandItem_Impl for FallbackCommandItem_Impl {
    fn FallbackHandler(&self) -> windows_core::Result<IFallbackHandler> {
        Ok(self.handler.to_interface())
    }
    fn DisplayTitle(&self) -> windows_core::Result<windows_core::HSTRING> {
        self.title.read().map(|s| s.clone())
    }
}

impl ICommandItem_Impl for FallbackCommandItem_Impl {
    ambassador_impl_ICommandItem_Impl! {
        body_struct(< >, ComObject<CommandItem>, base)
    }
}

impl INotifyPropChanged_Impl for FallbackCommandItem_Impl {
    ambassador_impl_INotifyPropChanged_Impl! {
        body_struct(< >, ComObject<CommandItem>, base)
    }
}

const _: () = assert_send_sync::<ComObject<FallbackHandler>>();
const _: () = assert_send_sync::<ComObject<FallbackCommandItem>>();

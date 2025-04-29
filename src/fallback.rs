
use super::cmd::BaseCommand;
use super::cmd_item::CommandItem;
use crate::bindings::*;
use crate::notify::*;
use windows::core::{ComObject, HSTRING, IUnknownImpl as _, Result, implement};
pub trait FallbackQuerier {
    fn update_query(&self, query: &HSTRING) -> Result<()>;
}

#[implement(IFallbackHandler)]
struct FallbackHandler<Q>
where
    Q: FallbackQuerier + 'static,
{
    pub querier: Q,
}

impl<Q> IFallbackHandler_Impl for FallbackHandler_Impl<Q>
where
    Q: FallbackQuerier + 'static,
{
    fn UpdateQuery(&self, query: &windows_core::HSTRING) -> windows_core::Result<()> {
        self.querier.update_query(query)
    }
}

#[implement(IFallbackCommandItem, ICommandItem, INotifyPropChanged)]
pub struct FallbackCommandItem<Q>
where
    Q: FallbackQuerier + 'static,
{
    command: ComObject<CommandItem<BaseCommand>>,
    handler: ComObject<FallbackHandler<Q>>,
    title: NotifyLock<HSTRING>,
}

impl<Q> FallbackCommandItem_Impl<Q>
where
    Q: FallbackQuerier + 'static,
{
    pub fn title(&self) -> Result<NotifyLockReadGuard<'_, HSTRING>> {
        self.title.read()
    }

    pub fn title_mut(&self) -> Result<NotifyLockWriteGuard<'_, HSTRING, impl Fn()>> {
        self.title.write(|| {
            self.command
                .emit_prop_changed(&self.to_interface(), "DisplayTitle")
        })
    }
}

impl<Q> IFallbackCommandItem_Impl for FallbackCommandItem_Impl<Q>
where
    Q: FallbackQuerier + 'static,
{
    fn FallbackHandler(&self) -> windows_core::Result<IFallbackHandler> {
        Ok(self.handler.to_interface())
    }
    fn DisplayTitle(&self) -> windows_core::Result<windows_core::HSTRING> {
        self.title.read().map(|s| s.clone())
    }
}

impl<Q> ICommandItem_Impl for FallbackCommandItem_Impl<Q>
where
    Q: FallbackQuerier + 'static,
{
    ambassador_impl_ICommandItem_Impl! {
        body_struct(< >, ComObject<CommandItem<BaseCommand>>, command)
    }
}

impl<Q> INotifyPropChanged_Impl for FallbackCommandItem_Impl<Q>
where
    Q: FallbackQuerier + 'static,
{
    ambassador_impl_INotifyPropChanged_Impl! {
        body_struct(< >, ComObject<CommandItem<BaseCommand>>, command)
    }
}

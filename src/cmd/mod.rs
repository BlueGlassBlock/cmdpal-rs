pub mod common;

use crate::bindings::*;
pub use crate::cmd_result::CommandResult;
use crate::icon::IconInfo;
use crate::notify::*;
use crate::utils::OkOrEmpty;
use windows::Foundation::TypedEventHandler;
use windows::core::{ComObject, Event, HSTRING, IInspectable, IUnknownImpl as _, implement};

#[implement(ICommand, INotifyPropChanged)]
pub struct BaseCommand {
    name: NotifyLock<HSTRING>,
    id: NotifyLock<HSTRING>,
    icon: NotifyLock<Option<ComObject<IconInfo>>>,
    event: Event<TypedEventHandler<IInspectable, IPropChangedEventArgs>>,
}

pub struct BaseCommandBuilder {
    name: HSTRING,
    id: HSTRING,
    icon: Option<ComObject<IconInfo>>,
}

impl BaseCommandBuilder {
    pub fn new() -> Self {
        Self {
            name: HSTRING::new(),
            id: HSTRING::new(),
            icon: None,
        }
    }

    pub fn name(mut self, name: impl Into<HSTRING>) -> Self {
        self.name = name.into();
        self
    }

    pub fn id(mut self, id: impl Into<HSTRING>) -> Self {
        self.id = id.into();
        self
    }

    pub fn icon(mut self, icon: ComObject<IconInfo>) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn build_unmanaged(self) -> BaseCommand {
        BaseCommand {
            name: NotifyLock::new(self.name),
            id: NotifyLock::new(self.id),
            icon: NotifyLock::new(self.icon),
            event: windows_core::Event::new(),
        }
    }

    pub fn build(self) -> ComObject<BaseCommand> {
        let obj = self.build_unmanaged();
        ComObject::new(obj)
    }
}

impl ICommand_Impl for BaseCommand_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        self.name.read().map(|name| name.clone())
    }

    fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        self.id.read().map(|id| id.clone())
    }

    fn Icon(&self) -> windows_core::Result<crate::bindings::IIconInfo> {
        self.icon
            .read()?
            .as_ref()
            .map(|icon| icon.to_interface())
            .or_or_empty()
    }
}

impl INotifyPropChanged_Impl for BaseCommand_Impl {
    fn PropChanged(
        &self,
        handler: windows_core::Ref<
            '_,
            windows::Foundation::TypedEventHandler<
                windows_core::IInspectable,
                crate::bindings::IPropChangedEventArgs,
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

impl BaseCommand_Impl {
    pub fn emit_prop_changed(&self, sender: IInspectable, prop: &str) {
        let args: IPropChangedEventArgs = PropChangedEventArgs(prop.into()).into();
        self.event
            .call(|handler| handler.Invoke(&sender, &args.clone()));
    }

    fn emit_self_prop_changed(&self, prop: &str) {
        self.emit_prop_changed(self.to_interface(), prop);
    }

    pub fn name(&self) -> windows_core::Result<NotifyLockReadGuard<'_, windows_core::HSTRING>> {
        self.name.read()
    }

    pub fn name_mut(
        &self,
    ) -> windows_core::Result<NotifyLockWriteGuard<'_, windows_core::HSTRING, impl Fn()>> {
        self.name.write(|| self.emit_self_prop_changed("Name"))
    }

    pub fn id(&self) -> windows_core::Result<NotifyLockReadGuard<'_, windows_core::HSTRING>> {
        self.id.read()
    }

    pub fn id_mut(
        &self,
    ) -> windows_core::Result<NotifyLockWriteGuard<'_, windows_core::HSTRING, impl Fn()>> {
        self.id.write(|| self.emit_self_prop_changed("Id"))
    }

    pub fn icon(
        &self,
    ) -> windows_core::Result<NotifyLockReadGuard<'_, Option<ComObject<IconInfo>>>> {
        self.icon.read()
    }

    pub fn icon_mut(
        &self,
    ) -> windows_core::Result<NotifyLockWriteGuard<'_, Option<ComObject<IconInfo>>, impl Fn()>>
    {
        self.icon.write(|| self.emit_self_prop_changed("Icon"))
    }
}

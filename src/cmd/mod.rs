pub mod common;

use crate::bindings::*;
use crate::notify::*;
use windows::core::implement;
use windows_core::ComObject;
use windows_core::HSTRING;
use windows_core::IInspectable;
use windows_core::IUnknownImpl as _;

pub use crate::cmd_result::CommandResult;
use crate::icon::IconInfo;

#[implement(ICommand, INotifyPropChanged)]
pub struct BaseCommand {
    name: NotifyLock<windows_core::HSTRING>,
    id: NotifyLock<windows_core::HSTRING>,
    icon: NotifyLock<Option<ComObject<IconInfo>>>,
    event: windows_core::Event<
        windows::Foundation::TypedEventHandler<windows_core::IInspectable, IPropChangedEventArgs>,
    >,
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
            .ok_or(windows_core::Error::empty())
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
    pub(crate) fn emit_prop_changed(&self, sender: IInspectable, prop: &str) {
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

impl BaseCommand {
    pub fn new(
        name: impl Into<HSTRING>,
        id: impl Into<HSTRING>,
        icon: Option<ComObject<IconInfo>>,
    ) -> Self {
        Self {
            name: NotifyLock::new(name.into()),
            id: NotifyLock::new(id.into()),
            icon: NotifyLock::new(icon),
            event: windows_core::Event::new(),
        }
    }
}

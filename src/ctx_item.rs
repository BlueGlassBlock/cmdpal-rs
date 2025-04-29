use crate::cmd_item::CommandItem;
use crate::{bindings::*, notify::*};
use windows::core::{
    ComObject, ComObjectInner, ComObjectInterface, Error, IInspectable, IUnknownImpl as _, Result,
    implement,
};

#[implement(ISeparatorContextItem, IContextItem)]
pub struct SeparatorContextItem;

impl ISeparatorContextItem_Impl for SeparatorContextItem_Impl {}
impl IContextItem_Impl for SeparatorContextItem_Impl {}

#[implement(ICommandContextItem, IContextItem, ICommandItem, INotifyPropChanged)]
pub struct CommandContextItem<TC>
where
    TC: ComObjectInner + 'static,
    TC::Outer: ICommand_Impl + ComObjectInterface<ICommand>,
{
    cmd: ComObject<CommandItem<TC>>,
    critical: NotifyLock<bool>,
    shortcut: NotifyLock<Option<KeyChord>>,
}

impl<TC> CommandContextItem_Impl<TC>
where
    TC: ComObjectInner + 'static,
    TC::Outer: ICommand_Impl + ComObjectInterface<ICommand>,
{
    pub(crate) fn emit_self_prop_changed(&self, prop: &str) {
        let sender: IInspectable = self.to_interface();
        self.cmd.emit_prop_changed(&sender, prop);
    }

    pub fn critical(&self) -> Result<NotifyLockReadGuard<'_, bool>> {
        self.critical.read()
    }

    pub fn critical_mut(&self) -> Result<NotifyLockWriteGuard<'_, bool, impl Fn()>> {
        self.critical
            .write(|| self.emit_self_prop_changed("Critical"))
    }

    pub fn shortcut(&self) -> Result<NotifyLockReadGuard<'_, Option<KeyChord>>> {
        self.shortcut.read()
    }
    pub fn shortcut_mut(&self) -> Result<NotifyLockWriteGuard<'_, Option<KeyChord>, impl Fn()>> {
        self.shortcut
            .write(|| self.emit_self_prop_changed("RequestedShortcut"))
    }
}
impl<TC> ICommandContextItem_Impl for CommandContextItem_Impl<TC>
where
    TC: ComObjectInner + 'static,
    TC::Outer: ICommand_Impl + ComObjectInterface<ICommand>,
{
    fn IsCritical(&self) -> windows_core::Result<bool> {
        self.critical.read().map(|x| *x)
    }
    fn RequestedShortcut(&self) -> windows_core::Result<KeyChord> {
        self.shortcut
            .read()?
            .map(|x| x)
            .ok_or(Error::empty())
    }
}

impl<TC> ICommandItem_Impl for CommandContextItem_Impl<TC>
where
    TC: ComObjectInner + 'static,
    TC::Outer: ICommand_Impl + ComObjectInterface<ICommand>,
{
    ambassador_impl_ICommandItem_Impl! {
        body_struct(< >, ComObject<CommandItem<TC>>, cmd)
    }
}

impl<TC> IContextItem_Impl for CommandContextItem_Impl<TC>
where
    TC: ComObjectInner + 'static,
    TC::Outer: ICommand_Impl + ComObjectInterface<ICommand>,
{
}

impl<TC> INotifyPropChanged_Impl for CommandContextItem_Impl<TC>
where
    TC: ComObjectInner + 'static,
    TC::Outer: ICommand_Impl + ComObjectInterface<ICommand>,
{
    ambassador_impl_INotifyPropChanged_Impl! {
        body_struct(< >, ComObject<CommandItem<TC>>, cmd)
    }
}

pub enum ContextItem<TC>
where
    TC: ComObjectInner + 'static,
    TC::Outer: ICommand_Impl + ComObjectInterface<ICommand>,
{
    Separator(ComObject<SeparatorContextItem>),
    Command(ComObject<CommandContextItem<TC>>),
}

impl<TC> From<&ContextItem<TC>> for IContextItem
where
    TC: ComObjectInner + 'static,
    TC::Outer: ICommand_Impl + ComObjectInterface<ICommand>,
{
    fn from(item: &ContextItem<TC>) -> Self {
        match item {
            ContextItem::Separator(item) => item.to_interface(),
            ContextItem::Command(item) => item.to_interface(),
        }
    }
}

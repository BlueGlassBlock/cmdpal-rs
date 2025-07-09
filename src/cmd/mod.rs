//! Commands - the foundation of Command Palette.

pub mod common;

use std::ops::Deref;

use crate::bindings::*;
pub use crate::cmd_result::CommandResult;
use crate::icon::IconInfo;
use crate::notify::*;
use crate::utils::{ComBuilder, OkOrEmpty};
use windows_core::{ComObject, Event, HSTRING, IInspectable, IUnknownImpl as _, implement};

/// Represents basic properties of a command.
/// 
/// See [`BaseCommand_Impl`] for field accessors.
///
#[doc = include_str!("../bindings_docs/ICommand.md")]
#[implement(ICommand, INotifyPropChanged)]
pub struct BaseCommand {
    name: NotifyLock<HSTRING>,
    id: NotifyLock<HSTRING>,
    icon: NotifyLock<Option<ComObject<IconInfo>>>,
    event: PropChangedEventHandler,
}

/// Builder for [`BaseCommand`].
pub struct BaseCommandBuilder {
    name: HSTRING,
    id: HSTRING,
    icon: Option<ComObject<IconInfo>>,
}

impl BaseCommandBuilder {
    /// Creates a new builder.
    pub fn new() -> Self {
        Self {
            name: HSTRING::new(),
            id: HSTRING::new(),
            icon: None,
        }
    }

    /// Sets the name of the command.
    pub fn name(mut self, name: impl Into<HSTRING>) -> Self {
        self.name = name.into();
        self
    }

    /// Sets the unique identifier of the command.
    pub fn id(mut self, id: impl Into<HSTRING>) -> Self {
        self.id = id.into();
        self
    }

    /// Sets the icon for the command.
    pub fn icon(mut self, icon: ComObject<IconInfo>) -> Self {
        self.icon = Some(icon);
        self
    }
}

impl ComBuilder for BaseCommandBuilder {
    type Output = BaseCommand;
    fn build_unmanaged(self) -> BaseCommand {
        BaseCommand {
            name: NotifyLock::new(self.name),
            id: NotifyLock::new(self.id),
            icon: NotifyLock::new(self.icon),
            event: Event::new(),
        }
    }
}

impl Default for BaseCommandBuilder {
    fn default() -> Self {
        Self::new()
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
            .ok_or_empty()
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

    /// Readonly access to [`ICommand::Name`].
    ///
    #[doc = include_str!("../bindings_docs/ICommand/Name.md")]
    pub fn name(&self) -> windows_core::Result<NotifyLockReadGuard<'_, windows_core::HSTRING>> {
        self.name.read()
    }

    /// Mutable access to [`ICommand::Name`].
    ///
    #[doc = include_str!("../bindings_docs/ICommand/Name.md")]
    ///
    /// Notifies the host about the property change when dropping the guard.
    pub fn name_mut(
        &self,
    ) -> windows_core::Result<NotifyLockWriteGuard<'_, windows_core::HSTRING>> {
        self.name.write(|| self.emit_self_prop_changed("Name"))
    }

    /// Readonly access to [`ICommand::Id`].
    ///
    #[doc = include_str!("../bindings_docs/ICommand/Id.md")]
    pub fn id(&self) -> windows_core::Result<NotifyLockReadGuard<'_, windows_core::HSTRING>> {
        self.id.read()
    }

    /// Mutable access to [`ICommand::Id`].
    ///
    #[doc = include_str!("../bindings_docs/ICommand/Id.md")]
    ///
    /// Notifies the host about the property change when dropping the guard.
    pub fn id_mut(&self) -> windows_core::Result<NotifyLockWriteGuard<'_, windows_core::HSTRING>> {
        self.id.write(|| self.emit_self_prop_changed("Id"))
    }

    /// Readonly access to [`ICommand::Icon`].
    ///
    #[doc = include_str!("../bindings_docs/ICommand/Icon.md")]
    pub fn icon(
        &self,
    ) -> windows_core::Result<NotifyLockReadGuard<'_, Option<ComObject<IconInfo>>>> {
        self.icon.read()
    }

    /// Mutable access to [`ICommand::Icon`].
    ///
    #[doc = include_str!("../bindings_docs/ICommand/Icon.md")]
    ///
    /// Notifies the host about the property change when dropping the guard.
    pub fn icon_mut(
        &self,
    ) -> windows_core::Result<NotifyLockWriteGuard<'_, Option<ComObject<IconInfo>>>> {
        self.icon.write(|| self.emit_self_prop_changed("Icon"))
    }
}

type InvokableBox = Box<dyn Send + Sync + Fn(&IInspectable) -> windows_core::Result<CommandResult>>;

/// Represents a command that can be invoked.
///
#[doc = include_str!("../bindings_docs/IInvokableCommand.md")]
#[implement(IInvokableCommand, ICommand, INotifyPropChanged)]
pub struct InvokableCommand {
    pub base: ComObject<BaseCommand>,
    func: InvokableBox,
}

/// Builder for [`InvokableCommand`].
pub struct InvokableCommandBuilder {
    base: ComObject<BaseCommand>,
    func: InvokableBox,
}

impl InvokableCommandBuilder {
    /// Creates a new builder.
    ///
    /// The invocation function is a no-op by default.
    pub fn new(base: ComObject<BaseCommand>) -> Self {
        Self {
            base,
            func: Box::new(|_| Ok(CommandResult::KeepOpen)),
        }
    }

    /// Sets the function to be invoked when the command is executed.
    ///
    /// See [`IInvokableCommand::Invoke`] for more details.
    pub fn func<F>(mut self, func: F) -> Self
    where
        F: Send + Sync + Fn(&IInspectable) -> windows_core::Result<CommandResult> + 'static,
    {
        self.func = Box::new(func);
        self
    }

    /// Sets an anonymous function to be invoked when the command is executed.
    ///
    /// The function should return a `CommandResult`.
    pub fn anon_func<F>(mut self, func: F) -> Self
    where
        F: Send + Sync + Fn() -> windows_core::Result<CommandResult> + 'static,
    {
        self.func = Box::new(move |_| func());
        self
    }
}

impl ComBuilder for InvokableCommandBuilder {
    type Output = InvokableCommand;
    fn build_unmanaged(self) -> InvokableCommand {
        InvokableCommand {
            base: self.base,
            func: self.func,
        }
    }
}

impl Deref for InvokableCommand {
    type Target = BaseCommand_Impl;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl IInvokableCommand_Impl for InvokableCommand_Impl {
    fn Invoke(
        &self,
        sender: windows_core::Ref<'_, windows_core::IInspectable>,
    ) -> windows_core::Result<ICommandResult> {
        let result = (self.func)(sender.ok()?);
        result.map(|r| r.into())
    }
}

impl ICommand_Impl for InvokableCommand_Impl {
    fn Icon(&self) -> windows_core::Result<IIconInfo> {
        self.base.Icon()
    }

    fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        self.base.Id()
    }

    fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        self.base.Name()
    }
}

impl INotifyPropChanged_Impl for InvokableCommand_Impl {
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
        self.base.PropChanged(handler)
    }

    fn RemovePropChanged(&self, token: i64) -> windows_core::Result<()> {
        self.base.RemovePropChanged(token)
    }
}

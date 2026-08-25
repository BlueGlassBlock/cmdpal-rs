//! Commands - the foundation of Command Palette.

use windows_core::{ComObject, Event, HSTRING, IInspectable, IUnknownImpl, Result, implement};

use crate::utils::{ComBuilder, OkOrEmpty};
use crate::{bindings::*, icon::IconInfo, notify::*};

pub use crate::cmd_result::CommandResult;

pub mod common;

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

impl BaseCommand {
    /// Creates a new builder.
    pub fn builder() -> BaseCommandBuilder {
        BaseCommandBuilder {
            name: HSTRING::new(),
            id: HSTRING::new(),
            icon: None,
        }
    }
}

impl BaseCommandBuilder {
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

impl ICommand_Impl for BaseCommand_Impl {
    fn Name(&self) -> Result<HSTRING> {
        self.name.read().map(|name| name.clone())
    }

    fn Id(&self) -> Result<HSTRING> {
        self.id.read().map(|id| id.clone())
    }

    fn Icon(&self) -> Result<crate::bindings::IIconInfo> {
        self.icon
            .read()?
            .as_ref()
            .map(|icon| icon.to_interface())
            .ok_or_empty()
    }
}

impl INotifyPropChanged_Impl for BaseCommand_Impl {
    fn PropChanged(&self, handler: RefPropChangedEventHandler<'_>) -> Result<i64> {
        self.event.add(handler.ok()?)
    }

    fn RemovePropChanged(&self, token: i64) -> Result<()> {
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
    pub fn name(&self) -> Result<NotifyLockReadGuard<'_, HSTRING>> {
        self.name.read()
    }

    /// Mutable access to [`ICommand::Name`].
    ///
    #[doc = include_str!("../bindings_docs/ICommand/Name.md")]
    ///
    /// Notifies the host about the property change when dropping the guard.
    pub fn name_mut(&self) -> Result<NotifyLockWriteGuard<'_, HSTRING>> {
        self.name.write(|| self.emit_self_prop_changed("Name"))
    }

    /// Readonly access to [`ICommand::Id`].
    ///
    #[doc = include_str!("../bindings_docs/ICommand/Id.md")]
    pub fn id(&self) -> Result<NotifyLockReadGuard<'_, HSTRING>> {
        self.id.read()
    }

    /// Mutable access to [`ICommand::Id`].
    ///
    #[doc = include_str!("../bindings_docs/ICommand/Id.md")]
    ///
    /// Notifies the host about the property change when dropping the guard.
    pub fn id_mut(&self) -> Result<NotifyLockWriteGuard<'_, HSTRING>> {
        self.id.write(|| self.emit_self_prop_changed("Id"))
    }

    /// Readonly access to [`ICommand::Icon`].
    ///
    #[doc = include_str!("../bindings_docs/ICommand/Icon.md")]
    pub fn icon(&self) -> Result<NotifyLockReadGuard<'_, Option<ComObject<IconInfo>>>> {
        self.icon.read()
    }

    /// Mutable access to [`ICommand::Icon`].
    ///
    #[doc = include_str!("../bindings_docs/ICommand/Icon.md")]
    ///
    /// Notifies the host about the property change when dropping the guard.
    pub fn icon_mut(&self) -> Result<NotifyLockWriteGuard<'_, Option<ComObject<IconInfo>>>> {
        self.icon.write(|| self.emit_self_prop_changed("Icon"))
    }
}

type InvokableBox = Box<dyn Send + Sync + Fn(&IInspectable) -> Result<CommandResult>>;

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

impl BaseCommandBuilder {
    /// Creates a new [`InvokableCommand`] builder.
    ///
    /// The invocation function is a no-op by default.
    pub fn invokable(self) -> InvokableCommandBuilder {
        InvokableCommandBuilder {
            base: self.build(),
            func: Box::new(|_| Ok(CommandResult::KeepOpen)),
        }
    }
}

impl InvokableCommandBuilder {
    /// Sets the function to be invoked when the command is executed.
    ///
    /// See [`IInvokableCommand::Invoke`] for more details.
    pub fn func<F>(mut self, func: F) -> Self
    where
        F: Send + Sync + Fn(&IInspectable) -> Result<CommandResult> + 'static,
    {
        self.func = Box::new(func);
        self
    }

    /// Sets an anonymous function to be invoked when the command is executed.
    ///
    /// The function should return a `CommandResult`.
    pub fn anon_func<F>(mut self, func: F) -> Self
    where
        F: Send + Sync + Fn() -> Result<CommandResult> + 'static,
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

impl std::ops::Deref for InvokableCommand {
    type Target = BaseCommand_Impl;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl IInvokableCommand_Impl for InvokableCommand_Impl {
    fn Invoke(&self, sender: windows_core::Ref<'_, IInspectable>) -> Result<ICommandResult> {
        let result = (self.func)(sender.ok()?);
        result.map(|r| r.into())
    }
}

impl ICommand_Impl for InvokableCommand_Impl {
    fn Icon(&self) -> Result<IIconInfo> {
        self.base.Icon()
    }

    fn Id(&self) -> Result<HSTRING> {
        self.base.Id()
    }

    fn Name(&self) -> Result<HSTRING> {
        self.base.Name()
    }
}

impl INotifyPropChanged_Impl for InvokableCommand_Impl {
    fn PropChanged(&self, handler: RefPropChangedEventHandler<'_>) -> Result<i64> {
        self.base.PropChanged(handler)
    }

    fn RemovePropChanged(&self, token: i64) -> Result<()> {
        self.base.RemovePropChanged(token)
    }
}

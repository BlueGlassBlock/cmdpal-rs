//! CommandItem represents commands in menus and lists

use crate::ctx_item::ContextItem;
use crate::icon::IconInfo;
use crate::notify::*;
use crate::utils::{ComBuilder, OkOrEmpty, assert_send_sync};
use crate::{bindings::*, utils::map_array};
use windows_core::{
    AgileReference, ComObject, Event, HSTRING, IInspectable, IUnknownImpl as _, implement,
};

/// Represents a command item that can be used in menus and lists.
///
/// See [`CommandItem_Impl`] for field accessors.
///
#[doc = include_str!("./bindings_docs/ICommandItem.md")]
#[implement(ICommandItem, INotifyPropChanged)]
pub struct CommandItem {
    command: NotifyLock<AgileReference<ICommand>>,
    icon: NotifyLock<Option<ComObject<IconInfo>>>,
    title: NotifyLock<HSTRING>,
    subtitle: NotifyLock<HSTRING>,
    more: NotifyLock<Vec<ContextItem>>,
    event: PropChangedEventHandler,
}

/// Field accessors for [`CommandItem`].
#[doc(inline)]
pub use self::CommandItem_Impl as Doc_CommandItem_Impl;

/// Builder for [`CommandItem`].
pub struct CommandItemBuilder {
    icon: Option<ComObject<IconInfo>>,
    title: Option<HSTRING>,
    subtitle: Option<HSTRING>,
    command: AgileReference<ICommand>,
    more: Vec<ContextItem>,
}

impl CommandItemBuilder {
    /// Creates a new builder with the specified command as selection target.
    pub fn new(command: AgileReference<ICommand>) -> Self {
        CommandItemBuilder {
            icon: None,
            title: None,
            subtitle: None,
            command,
            more: Vec::new(),
        }
    }

    /// Creates a new builder with the specified command as selection target.
    pub fn try_new(command: ICommand) -> windows_core::Result<Self> {
        let agile_command = AgileReference::new(&command)?;
        Ok(Self::new(agile_command))
    }

    /// Sets the icon for the command item.
    ///
    /// If unset, the command item will fallback to [`ICommand::Icon`].
    pub fn icon(mut self, icon: ComObject<IconInfo>) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Sets the title for the command item.
    ///
    /// If unset, the command item will fallback to [`ICommand::Name`].
    ///
    /// Note that the text displayed for default action (alongside <kbd>↲</kbd>) is determined by
    /// [`ICommand::Name`], not this title.
    pub fn title(mut self, title: impl Into<HSTRING>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Sets the subtitle for the command item.
    pub fn subtitle(mut self, subtitle: impl Into<HSTRING>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }

    /// Sets context menu items for the command item.
    pub fn more(mut self, more: Vec<ContextItem>) -> Self {
        self.more = more;
        self
    }

    /// Adds a context menu item to the command item.
    pub fn add_context_item(mut self, item: ContextItem) -> Self {
        self.more.push(item);
        self
    }
}

impl ComBuilder for CommandItemBuilder {
    type Output = CommandItem;
    fn build_unmanaged(self) -> CommandItem {
        let title = self.title.unwrap_or_else(|| HSTRING::new());
        let subtitle = self.subtitle.unwrap_or_else(|| HSTRING::new());

        CommandItem {
            command: NotifyLock::new(self.command),
            icon: NotifyLock::new(self.icon),
            title: NotifyLock::new(title),
            subtitle: NotifyLock::new(subtitle),
            more: NotifyLock::new(self.more),
            event: Event::new(),
        }
    }
}

impl CommandItem_Impl {
    pub(crate) fn emit_prop_changed(&self, sender: &IInspectable, prop: &str) {
        let arg: IPropChangedEventArgs = PropChangedEventArgs(prop.into()).into();
        self.event.call(|handler| handler.Invoke(sender, &arg));
    }
    pub(crate) fn emit_self_prop_changed(&self, prop: &str) {
        let sender: IInspectable = self.to_interface();
        let arg: IPropChangedEventArgs = PropChangedEventArgs(prop.into()).into();
        self.event.call(|handler| handler.Invoke(&sender, &arg));
    }

    /// Readonly access to [`ICommandItem::Command`].
    ///
    #[doc = include_str!("./bindings_docs/ICommandItem/Command.md")]
    pub fn command(
        &self,
    ) -> windows_core::Result<NotifyLockReadGuard<'_, AgileReference<ICommand>>> {
        self.command.read()
    }

    /// Mutable access to [`ICommandItem::Command`].
    ///
    #[doc = include_str!("./bindings_docs/ICommandItem/Command.md")]
    ///
    /// Notifies the host about the property change when dropping the guard.
    pub fn command_mut(
        &self,
    ) -> windows_core::Result<NotifyLockWriteGuard<'_, AgileReference<ICommand>>> {
        self.command
            .write(|| self.emit_self_prop_changed("Command"))
    }

    /// Readonly access to [`ICommandItem::Icon`].
    ///
    /// Preferred over [`ICommand::Icon`] of `self.command` when displaying the icon.
    /// 
    #[doc = include_str!("./bindings_docs/ICommandItem/Icon.md")]
    pub fn icon(
        &self,
    ) -> windows_core::Result<NotifyLockReadGuard<'_, Option<ComObject<IconInfo>>>> {
        self.icon.read()
    }

    /// Mutable access to [`ICommandItem::Icon`].
    ///
    /// Preferred over [`ICommand::Icon`] of `self.command` when displaying the icon.
    /// 
    #[doc = include_str!("./bindings_docs/ICommandItem/Icon.md")]
    ///
    /// Notifies the host about the property change when dropping the guard.
    pub fn icon_mut(
        &self,
    ) -> windows_core::Result<NotifyLockWriteGuard<'_, Option<ComObject<IconInfo>>>> {
        self.icon.write(|| self.emit_self_prop_changed("Icon"))
    }

    /// Readonly access to [`ICommandItem::Title`].
    ///
    /// Preferred over [`ICommand::Name`] of `self.command` when displaying the title.
    /// 
    /// Note that the text displayed for default action (alongside <kbd>↲</kbd>) is determined by
    /// [`ICommand::Name`] of `self.command`, not this title.
    /// 
    #[doc = include_str!("./bindings_docs/ICommandItem/Title.md")]
    pub fn title(&self) -> windows_core::Result<NotifyLockReadGuard<'_, HSTRING>> {
        self.title.read()
    }

    /// Mutable access to [`ICommandItem::Title`].
    ///
    /// Preferred over [`ICommand::Name`] of `self.command` when displaying the title.
    /// 
    /// Note that the text displayed for default action (alongside <kbd>↲</kbd>) is determined by
    /// [`ICommand::Name`] of `self.command`, not this title.
    /// 
    #[doc = include_str!("./bindings_docs/ICommandItem/Title.md")]
    ///
    /// Notifies the host about the property change when dropping the guard.
    pub fn title_mut(&self) -> windows_core::Result<NotifyLockWriteGuard<'_, HSTRING>> {
        self.title.write(|| self.emit_self_prop_changed("Title"))
    }

    /// Readonly access to [`ICommandItem::Subtitle`].
    ///
    #[doc = include_str!("./bindings_docs/ICommandItem/Subtitle.md")]
    pub fn subtitle(&self) -> windows_core::Result<NotifyLockReadGuard<'_, HSTRING>> {
        self.subtitle.read()
    }

    /// Mutable access to [`ICommandItem::Subtitle`].
    ///
    #[doc = include_str!("./bindings_docs/ICommandItem/Subtitle.md")]
    ///
    /// Notifies the host about the property change when dropping the guard.
    pub fn subtitle_mut(&self) -> windows_core::Result<NotifyLockWriteGuard<'_, HSTRING>> {
        self.subtitle
            .write(|| self.emit_self_prop_changed("Subtitle"))
    }

    /// Readonly access to [`ICommandItem::MoreCommands`].
    ///
    #[doc = include_str!("./bindings_docs/ICommandItem/MoreCommands.md")]
    pub fn more(&self) -> windows_core::Result<NotifyLockReadGuard<'_, Vec<ContextItem>>> {
        self.more.read()
    }

    /// Mutable access to [`ICommandItem::MoreCommands`].
    ///
    #[doc = include_str!("./bindings_docs/ICommandItem/MoreCommands.md")]
    ///
    /// Notifies the host about the property change when dropping the guard.
    pub fn more_mut(&self) -> windows_core::Result<NotifyLockWriteGuard<'_, Vec<ContextItem>>> {
        self.more
            .write(|| self.emit_self_prop_changed("MoreCommands"))
    }
}

impl ICommandItem_Impl for CommandItem_Impl {
    fn Command(&self) -> windows_core::Result<ICommand> {
        self.command.read()?.resolve()
    }

    fn Icon(&self) -> windows_core::Result<IIconInfo> {
        self.icon
            .read()?
            .as_ref()
            .map(|icon| icon.to_interface())
            .ok_or_empty()
    }

    fn Title(&self) -> windows_core::Result<windows_core::HSTRING> {
        Ok(self.title.read()?.clone())
    }

    fn Subtitle(&self) -> windows_core::Result<windows_core::HSTRING> {
        Ok(self.subtitle.read()?.clone())
    }

    fn MoreCommands(&self) -> windows_core::Result<windows_core::Array<IContextItem>> {
        let more = self.more.read()?;
        Ok(map_array(&more, |x| {
            Some(match x {
                ContextItem::Separator(item) => item.to_interface(),
                ContextItem::Command(item) => item.to_interface(),
            })
        }))
    }
}

impl INotifyPropChanged_Impl for CommandItem_Impl {
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

const _: () = assert_send_sync::<ComObject<CommandItem>>();

//! Context menu items.

use std::ops::Deref;

use crate::cmd_item::{CommandItem, CommandItem_Impl};
use crate::utils::{ComBuilder, OkOrEmpty, assert_send_sync};
use crate::{bindings::*, notify::*};
use windows_core::{ComObject, IInspectable, IUnknownImpl as _, Result, implement};

/// Represents a separator in the context menu.
///
#[doc = include_str!("./bindings_docs/ISeparatorContextItem.md")]
#[implement(ISeparatorContextItem, IContextItem)]
pub struct SeparatorContextItem;

impl SeparatorContextItem {
    pub fn new() -> ComObject<Self> {
        ComObject::new(SeparatorContextItem)
    }
}

impl ISeparatorContextItem_Impl for SeparatorContextItem_Impl {}
impl IContextItem_Impl for SeparatorContextItem_Impl {}

/// Represents a command item in the context menu.
///
/// See [`CommandContextItem_Impl`] for field accessors.
/// 
#[doc = include_str!("./bindings_docs/ICommandContextItem.md")]
#[implement(ICommandContextItem, IContextItem, ICommandItem, INotifyPropChanged)]
pub struct CommandContextItem {
    /// The base command item that this context item wraps.
    pub base: ComObject<CommandItem>,
    critical: NotifyLock<bool>,
    shortcut: NotifyLock<Option<KeyChord>>,
}

pub struct CommandContextItemBuilder {
    base: ComObject<CommandItem>,
    critical: bool,
    shortcut: Option<KeyChord>,
}

impl CommandContextItemBuilder {
    /// Creates a new builder.
    pub fn new(base: ComObject<CommandItem>) -> Self {
        CommandContextItemBuilder {
            base,
            critical: false,
            shortcut: None,
        }
    }

    /// Sets whether the command is critical.
    pub fn critical(mut self, critical: bool) -> Self {
        self.critical = critical;
        self
    }

    /// Sets the keyboard shortcut for the command.
    pub fn shortcut(mut self, shortcut: KeyChord) -> Self {
        self.shortcut = Some(shortcut);
        self
    }
}

impl ComBuilder for CommandContextItemBuilder {
    type Output = CommandContextItem;
    fn build_unmanaged(self) -> CommandContextItem {
        CommandContextItem {
            base: self.base,
            critical: NotifyLock::new(self.critical),
            shortcut: NotifyLock::new(self.shortcut),
        }
    }
}

impl Deref for CommandContextItem {
    type Target = CommandItem_Impl;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl CommandContextItem_Impl {
    pub(crate) fn emit_self_prop_changed(&self, prop: &str) {
        let sender: IInspectable = self.to_interface();
        self.base.emit_prop_changed(&sender, prop);
    }

    /// Readonly access to [`ICommandContextItem::IsCritical`].
    ///
    #[doc = include_str!("./bindings_docs/ICommandContextItem/IsCritical.md")]
    pub fn critical(&self) -> Result<NotifyLockReadGuard<'_, bool>> {
        self.critical.read()
    }

    /// Mutable access to [`ICommandContextItem::IsCritical`].
    ///
    #[doc = include_str!("./bindings_docs/ICommandContextItem/RequestedShortcut.md")]
    ///
    /// Notifies the host about the property change when dropping the guard.
    pub fn critical_mut(&self) -> Result<NotifyLockWriteGuard<'_, bool>> {
        self.critical
            .write(|| self.emit_self_prop_changed("Critical"))
    }

    /// Readonly access to [`ICommandContextItem::RequestedShortcut`].
    ///
    #[doc = include_str!("./bindings_docs/ICommandContextItem/RequestedShortcut.md")]
    pub fn shortcut(&self) -> Result<NotifyLockReadGuard<'_, Option<KeyChord>>> {
        self.shortcut.read()
    }

    /// Mutable access to [`ICommandContextItem::RequestedShortcut`].
    ///
    #[doc = include_str!("./bindings_docs/ICommandContextItem/RequestedShortcut.md")]
    ///
    /// Notifies the host about the property change when dropping the guard.
    pub fn shortcut_mut(&self) -> Result<NotifyLockWriteGuard<'_, Option<KeyChord>>> {
        self.shortcut
            .write(|| self.emit_self_prop_changed("RequestedShortcut"))
    }
}

impl ICommandContextItem_Impl for CommandContextItem_Impl {
    fn IsCritical(&self) -> windows_core::Result<bool> {
        self.critical.read().map(|x| *x)
    }
    fn RequestedShortcut(&self) -> windows_core::Result<KeyChord> {
        self.shortcut.read()?.map(|x| x).ok_or_empty()
    }
}

impl ICommandItem_Impl for CommandContextItem_Impl {
    fn Command(&self) -> windows_core::Result<ICommand> {
        self.base.Command()
    }

    fn Icon(&self) -> windows_core::Result<IIconInfo> {
        self.base.Icon()
    }

    fn MoreCommands(&self) -> windows_core::Result<windows_core::Array<IContextItem>> {
        self.base.MoreCommands()
    }

    fn Subtitle(&self) -> windows_core::Result<windows_core::HSTRING> {
        self.base.Subtitle()
    }

    fn Title(&self) -> windows_core::Result<windows_core::HSTRING> {
        self.base.Title()
    }
}

impl IContextItem_Impl for CommandContextItem_Impl {}

impl INotifyPropChanged_Impl for CommandContextItem_Impl {
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

/// Represents an item in the context menu, which can be either a separator or a command.
///
#[doc = include_str!("./bindings_docs/IContextItem.md")]
pub enum ContextItem {
    /// A separator item in the context menu.
    Separator(ComObject<SeparatorContextItem>),
    /// A command item in the context menu.
    Command(ComObject<CommandContextItem>),
}

impl From<SeparatorContextItem> for ContextItem {
    fn from(item: SeparatorContextItem) -> Self {
        ContextItem::Separator(ComObject::new(item))
    }
}

impl From<CommandContextItem> for ContextItem {
    fn from(item: CommandContextItem) -> Self {
        ContextItem::Command(ComObject::new(item))
    }
}

impl From<ComObject<SeparatorContextItem>> for ContextItem {
    fn from(item: ComObject<SeparatorContextItem>) -> Self {
        ContextItem::Separator(item)
    }
}

impl From<ComObject<CommandContextItem>> for ContextItem {
    fn from(item: ComObject<CommandContextItem>) -> Self {
        ContextItem::Command(item)
    }
}

impl From<&ContextItem> for IContextItem {
    fn from(item: &ContextItem) -> Self {
        match item {
            ContextItem::Separator(item) => item.to_interface(),
            ContextItem::Command(item) => item.to_interface(),
        }
    }
}

const _: () = assert_send_sync::<ContextItem>();

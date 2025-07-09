use std::ops::Deref;

use crate::cmd_item::{CommandItem, CommandItem_Impl};
use crate::utils::{ComBuilder, OkOrEmpty, assert_send_sync};
use crate::{bindings::*, notify::*};
use windows::core::{ComObject, IInspectable, IUnknownImpl as _, Result, implement};

#[implement(ISeparatorContextItem, IContextItem)]
pub struct SeparatorContextItem;

impl SeparatorContextItem {
    pub fn new() -> ComObject<Self> {
        ComObject::new(SeparatorContextItem)
    }
}

impl ISeparatorContextItem_Impl for SeparatorContextItem_Impl {}
impl IContextItem_Impl for SeparatorContextItem_Impl {}

#[implement(ICommandContextItem, IContextItem, ICommandItem, INotifyPropChanged)]
pub struct CommandContextItem {
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
    pub fn new(base: ComObject<CommandItem>) -> Self {
        CommandContextItemBuilder {
            base,
            critical: false,
            shortcut: None,
        }
    }

    pub fn critical(mut self, critical: bool) -> Self {
        self.critical = critical;
        self
    }

    pub fn shortcut(mut self, shortcut: Option<KeyChord>) -> Self {
        self.shortcut = shortcut;
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

    pub fn critical(&self) -> Result<NotifyLockReadGuard<'_, bool>> {
        self.critical.read()
    }

    pub fn critical_mut(&self) -> Result<NotifyLockWriteGuard<'_, bool>> {
        self.critical
            .write(|| self.emit_self_prop_changed("Critical"))
    }

    pub fn shortcut(&self) -> Result<NotifyLockReadGuard<'_, Option<KeyChord>>> {
        self.shortcut.read()
    }
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

pub enum ContextItem {
    Separator(ComObject<SeparatorContextItem>),
    Command(ComObject<CommandContextItem>),
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

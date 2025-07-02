use crate::cmd_item::CommandItem;
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
    pub cmd_item: ComObject<CommandItem>,
    critical: NotifyLock<bool>,
    shortcut: NotifyLock<Option<KeyChord>>,
}

pub struct CommandContextItemBuilder {
    cmd_item: ComObject<CommandItem>,
    critical: bool,
    shortcut: Option<KeyChord>,
}

impl CommandContextItemBuilder {
    pub fn new(cmd_item: ComObject<CommandItem>) -> Self {
        CommandContextItemBuilder {
            cmd_item,
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
    type Target = CommandContextItem;
    fn build_unmanaged(self) -> CommandContextItem {
        CommandContextItem {
            cmd_item: self.cmd_item,
            critical: NotifyLock::new(self.critical),
            shortcut: NotifyLock::new(self.shortcut),
        }
    }
}

impl CommandContextItem_Impl {
    pub(crate) fn emit_self_prop_changed(&self, prop: &str) {
        let sender: IInspectable = self.to_interface();
        self.cmd_item.emit_prop_changed(&sender, prop);
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
impl ICommandContextItem_Impl for CommandContextItem_Impl {
    fn IsCritical(&self) -> windows_core::Result<bool> {
        self.critical.read().map(|x| *x)
    }
    fn RequestedShortcut(&self) -> windows_core::Result<KeyChord> {
        self.shortcut.read()?.map(|x| x).ok_or_empty()
    }
}

impl ICommandItem_Impl for CommandContextItem_Impl {
    ambassador_impl_ICommandItem_Impl! {
        body_struct(< >, ComObject<CommandItem>, cmd_item)
    }
}

impl IContextItem_Impl for CommandContextItem_Impl {}

impl INotifyPropChanged_Impl for CommandContextItem_Impl {
    ambassador_impl_INotifyPropChanged_Impl! {
        body_struct(< >, ComObject<CommandItem>, cmd_item)
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

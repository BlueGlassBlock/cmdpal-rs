use crate::ctx_item::ContextItem;
use crate::icon::IconInfo;
use crate::notify::*;
use crate::utils::{ComBuilder, OkOrEmpty, assert_send_sync};
use crate::{bindings::*, utils::map_array};
use windows::core::{
    AgileReference, ComObject, Event, HSTRING, IInspectable, IUnknownImpl as _, implement,
};

#[implement(ICommandItem, INotifyPropChanged)]
pub struct CommandItem {
    command: NotifyLock<AgileReference<ICommand>>,
    icon: NotifyLock<Option<ComObject<IconInfo>>>,
    title: NotifyLock<HSTRING>,
    subtitle: NotifyLock<HSTRING>,
    more: NotifyLock<Vec<ContextItem>>,
    event: PropChangedEventHandler,
}

pub struct CommandItemBuilder {
    icon: Option<ComObject<IconInfo>>,
    title: Option<HSTRING>,
    subtitle: Option<HSTRING>,
    command: AgileReference<ICommand>,
    more: Vec<ContextItem>,
}

impl CommandItemBuilder {
    pub fn new(command: AgileReference<ICommand>) -> Self {
        CommandItemBuilder {
            icon: None,
            title: None,
            subtitle: None,
            command,
            more: Vec::new(),
        }
    }

    pub fn try_new(command: ICommand) -> windows::core::Result<Self> {
        let agile_command = AgileReference::new(&command)?;
        Ok(Self::new(agile_command))
    }

    pub fn icon(mut self, icon: ComObject<IconInfo>) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn title(mut self, title: impl Into<HSTRING>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn subtitle(mut self, subtitle: impl Into<HSTRING>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }

    pub fn more(mut self, more: Vec<ContextItem>) -> Self {
        self.more = more;
        self
    }

    pub fn add_context_item(mut self, item: ContextItem) -> Self {
        self.more.push(item);
        self
    }
}

impl ComBuilder for CommandItemBuilder {
    type Target = CommandItem;
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

    pub fn command(
        &self,
    ) -> windows_core::Result<NotifyLockReadGuard<'_, AgileReference<ICommand>>> {
        self.command.read()
    }

    pub fn command_mut(
        &self,
    ) -> windows_core::Result<NotifyLockWriteGuard<'_, AgileReference<ICommand>>> {
        self.command
            .write(|| self.emit_self_prop_changed("Command"))
    }

    pub fn icon(
        &self,
    ) -> windows_core::Result<NotifyLockReadGuard<'_, Option<ComObject<IconInfo>>>> {
        self.icon.read()
    }

    pub fn icon_mut(
        &self,
    ) -> windows_core::Result<NotifyLockWriteGuard<'_, Option<ComObject<IconInfo>>>>
    {
        self.icon.write(|| self.emit_self_prop_changed("Icon"))
    }

    pub fn title(&self) -> windows_core::Result<NotifyLockReadGuard<'_, HSTRING>> {
        self.title.read()
    }

    pub fn title_mut(&self) -> windows_core::Result<NotifyLockWriteGuard<'_, HSTRING>> {
        self.title.write(|| self.emit_self_prop_changed("Title"))
    }

    pub fn subtitle(&self) -> windows_core::Result<NotifyLockReadGuard<'_, HSTRING>> {
        self.subtitle.read()
    }

    pub fn subtitle_mut(
        &self,
    ) -> windows_core::Result<NotifyLockWriteGuard<'_, HSTRING>> {
        self.subtitle
            .write(|| self.emit_self_prop_changed("Subtitle"))
    }

    pub fn more(&self) -> windows_core::Result<NotifyLockReadGuard<'_, Vec<ContextItem>>> {
        self.more.read()
    }

    pub fn more_mut(
        &self,
    ) -> windows_core::Result<NotifyLockWriteGuard<'_, Vec<ContextItem>>> {
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

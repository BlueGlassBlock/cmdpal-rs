use crate::icon::IconInfo;
use crate::notify::*;
use crate::{bindings::*, utils::map_array};
use windows::Foundation::TypedEventHandler;
use windows::core::{
    ComObject, Event, HSTRING, IInspectable, IUnknownImpl as _, implement,
};

#[implement(ICommandItem, INotifyPropChanged)]
pub struct CommandItem {
    command: NotifyLock<ICommand>,
    icon: NotifyLock<Option<ComObject<IconInfo>>>,
    title: NotifyLock<HSTRING>,
    subtitle: NotifyLock<HSTRING>,
    more: NotifyLock<Vec<IContextItem>>,
    event: Event<TypedEventHandler<IInspectable, IPropChangedEventArgs>>,
}

impl CommandItem {
    pub fn new(
        icon: Option<ComObject<IconInfo>>,
        title: impl Into<HSTRING>,
        subtitle: impl Into<HSTRING>,
        command: ICommand,
        more: Vec<IContextItem>,
    ) -> Self {
        let title = title.into();
        let subtitle = subtitle.into();
        let more = more.into_iter().collect();

        CommandItem {
            command: NotifyLock::new(command),
            icon: NotifyLock::new(icon),
            title: NotifyLock::new(title),
            subtitle: NotifyLock::new(subtitle),
            more: NotifyLock::new(more),
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

    pub fn command(&self) -> windows_core::Result<NotifyLockReadGuard<'_, ICommand>> {
        self.command.read()
    }

    pub fn command_mut(
        &self,
    ) -> windows_core::Result<NotifyLockWriteGuard<'_, ICommand, impl Fn()>> {
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
    ) -> windows_core::Result<NotifyLockWriteGuard<'_, Option<ComObject<IconInfo>>, impl Fn()>>
    {
        self.icon.write(|| self.emit_self_prop_changed("Icon"))
    }

    pub fn title(&self) -> windows_core::Result<NotifyLockReadGuard<'_, HSTRING>> {
        self.title.read()
    }

    pub fn title_mut(&self) -> windows_core::Result<NotifyLockWriteGuard<'_, HSTRING, impl Fn()>> {
        self.title.write(|| self.emit_self_prop_changed("Title"))
    }

    pub fn subtitle(&self) -> windows_core::Result<NotifyLockReadGuard<'_, HSTRING>> {
        self.subtitle.read()
    }

    pub fn subtitle_mut(
        &self,
    ) -> windows_core::Result<NotifyLockWriteGuard<'_, HSTRING, impl Fn()>> {
        self.subtitle
            .write(|| self.emit_self_prop_changed("Subtitle"))
    }

    pub fn more(&self) -> windows_core::Result<NotifyLockReadGuard<'_, Vec<IContextItem>>> {
        self.more.read()
    }

    pub fn more_mut(
        &self,
    ) -> windows_core::Result<NotifyLockWriteGuard<'_, Vec<IContextItem>, impl Fn()>> {
        self.more
            .write(|| self.emit_self_prop_changed("MoreCommands"))
    }
}

impl ICommandItem_Impl for CommandItem_Impl {
    fn Command(&self) -> windows_core::Result<ICommand> {
        Ok(self.command.read()?.clone())
    }

    fn Icon(&self) -> windows_core::Result<IIconInfo> {
        self.icon
            .read()?
            .as_ref()
            .map(|icon| icon.to_interface())
            .ok_or(windows_core::Error::empty())
    }

    fn Title(&self) -> windows_core::Result<windows_core::HSTRING> {
        Ok(self.title.read()?.clone())
    }

    fn Subtitle(&self) -> windows_core::Result<windows_core::HSTRING> {
        Ok(self.subtitle.read()?.clone())
    }

    fn MoreCommands(&self) -> windows_core::Result<windows_core::Array<IContextItem>> {
        let more = self.more.read()?;
        Ok(map_array(&more, |x| x.clone().into()))
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

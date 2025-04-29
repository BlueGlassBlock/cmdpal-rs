
use crate::bindings::*;
use windows::core::{implement, ComObject};
use windows::Foundation::{IClosable, IClosable_Impl, TypedEventHandler};
use windows_core::{Event, IInspectable};
use crate::icon::IconInfo;
use crate::utils::map_array;

#[implement(ICommandProvider, IClosable, INotifyItemsChanged)]
pub struct CommandProvider {
    id: windows_core::HSTRING,
    display_name: windows_core::HSTRING,
    icon: Option<ComObject<IconInfo>>,
    settings: Option<ICommandSettings>,
    frozen: bool,
    top_level: Vec<ICommandItem>,
    fallbacks: Vec<IFallbackCommandItem>,
    event: Event<TypedEventHandler<IInspectable, IItemsChangedEventArgs>>,
}

impl CommandProvider {
    pub fn new(
        id: impl Into<windows_core::HSTRING>,
        display_name: impl Into<windows_core::HSTRING>,
        icon: Option<ComObject<IconInfo>>,
        settings: Option<ICommandSettings>,
        frozen: bool,
        top_level: Vec<ICommandItem>,
        fallbacks: Vec<IFallbackCommandItem>,
    ) -> Self {
        let id = id.into();
        let display_name = display_name.into();
        let icon = icon;
        let settings = settings;
        let top_level = top_level.into_iter().collect();
        let fallbacks = fallbacks.into_iter().collect();

        CommandProvider {
            id,
            display_name,
            icon,
            settings,
            frozen,
            top_level,
            fallbacks,
            event: Event::new(),
        }
    }
}

impl ICommandProvider_Impl for CommandProvider_Impl {
    fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        Ok(self.id.clone())
    }

    fn DisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        Ok(self.display_name.clone())
    }

    fn Icon(&self) -> windows_core::Result<crate::bindings::IIconInfo> {
        self.icon
            .clone()
            .map(|icon| icon.to_interface())
            .ok_or(windows_core::Error::empty())
    }

    fn Settings(&self) -> windows_core::Result<ICommandSettings> {
        self.settings
            .clone()
            .ok_or(windows_core::Error::empty())
    }

    fn Frozen(&self) -> windows_core::Result<bool> {
        Ok(self.frozen)
    }

    fn TopLevelCommands(&self) -> windows_core::Result<windows_core::Array<ICommandItem>> {
        Ok(map_array(&self.top_level, |x| x.clone().into()))
    }

    fn FallbackCommands(&self) -> windows_core::Result<windows_core::Array<IFallbackCommandItem>> {
        Ok(map_array(&self.fallbacks, |x| x.clone().into()))
    }

    fn GetCommand(&self, _: &windows_core::HSTRING) -> windows_core::Result<ICommand> {
        Err(windows::core::Error::empty())
    }

    fn InitializeWithHost(
            &self,
            host: windows_core::Ref<'_, IExtensionHost>,
        ) -> windows_core::Result<()> {
        crate::host::set_ext_host(host.ok()?);
        Ok(())
    }
}

impl IClosable_Impl for CommandProvider_Impl {
    fn Close(&self) -> windows_core::Result<()> {
        Ok(())
    }
}

impl INotifyItemsChanged_Impl for CommandProvider_Impl {
    fn ItemsChanged(
        &self,
        handler: windows_core::Ref<
            '_,
            TypedEventHandler<IInspectable, IItemsChangedEventArgs>,
        >,
    ) -> windows_core::Result<i64> {
        self.event.add(handler.ok()?)
    }

    fn RemoveItemsChanged(&self, token: i64) -> windows_core::Result<()> {
        self.event.remove(token);
        Ok(())
    }
}
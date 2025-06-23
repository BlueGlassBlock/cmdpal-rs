use crate::bindings::*;
use crate::icon::IconInfo;
use crate::utils::{ComBuilder, OkOrEmpty, map_array};
use windows::Foundation::{IClosable, IClosable_Impl, TypedEventHandler};
use windows::core::{ComObject, implement};
use windows_core::{Event, HSTRING, IInspectable};

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

pub struct CommandProviderBuilder {
    id: windows_core::HSTRING,
    display_name: windows_core::HSTRING,
    icon: Option<ComObject<IconInfo>>,
    settings: Option<ICommandSettings>,
    frozen: bool,
    top_level: Vec<ICommandItem>,
    fallbacks: Vec<IFallbackCommandItem>,
}

impl CommandProviderBuilder {
    pub fn new() -> Self {
        CommandProviderBuilder {
            id: HSTRING::new(),
            display_name: HSTRING::new(),
            icon: None,
            settings: None,
            frozen: false,
            top_level: Vec::new(),
            fallbacks: Vec::new(),
        }
    }

    pub fn id(mut self, id: impl Into<windows_core::HSTRING>) -> Self {
        self.id = id.into();
        self
    }

    pub fn display_name(mut self, display_name: impl Into<windows_core::HSTRING>) -> Self {
        self.display_name = display_name.into();
        self
    }

    pub fn icon(mut self, icon: ComObject<IconInfo>) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn settings(mut self, settings: ICommandSettings) -> Self {
        self.settings = Some(settings);
        self
    }

    pub fn frozen(mut self, frozen: bool) -> Self {
        self.frozen = frozen;
        self
    }

    pub fn top_level(mut self, top_level: Vec<ICommandItem>) -> Self {
        self.top_level = top_level;
        self
    }

    pub fn add_top_level(mut self, item: ICommandItem) -> Self {
        self.top_level.push(item);
        self
    }

    pub fn fallbacks(mut self, fallbacks: Vec<IFallbackCommandItem>) -> Self {
        self.fallbacks = fallbacks;
        self
    }

    pub fn add_fallback(mut self, item: IFallbackCommandItem) -> Self {
        self.fallbacks.push(item);
        self
    }
}

impl ComBuilder<CommandProvider> for CommandProviderBuilder {
    fn build_unmanaged(self) -> CommandProvider {
        CommandProvider {
            id: self.id,
            display_name: self.display_name,
            icon: self.icon,
            settings: self.settings,
            frozen: self.frozen,
            top_level: self.top_level,
            fallbacks: self.fallbacks,
            event: Event::new(),
        }
    }
}

impl Default for CommandProviderBuilder {
    fn default() -> Self {
        Self::new()
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
            .ok_or_empty()
    }

    fn Settings(&self) -> windows_core::Result<ICommandSettings> {
        self.settings.clone().ok_or_empty()
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
        handler: windows_core::Ref<'_, TypedEventHandler<IInspectable, IItemsChangedEventArgs>>,
    ) -> windows_core::Result<i64> {
        self.event.add(handler.ok()?)
    }

    fn RemoveItemsChanged(&self, token: i64) -> windows_core::Result<()> {
        self.event.remove(token);
        Ok(())
    }
}

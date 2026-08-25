//! Command Provider that provides extension information and commands.
//!
use windows::Foundation::{IClosable, IClosable_Impl, TypedEventHandler};
use windows_core::{Array, ComObject, Event, HSTRING, IInspectable, Result};

use crate::utils::{ComBuilder, OkOrEmpty, map_array};
use crate::{
    bindings::*, cmd_item::CommandItemBuilder, icon::IconInfo, notify::ItemsChangedEventHandler,
};

/// Command Provider that provides extension information and commands.
///
#[doc = include_str!("./bindings_docs/ICommandProvider.md")]
#[windows_core::implement(ICommandProvider2, ICommandProvider, IClosable, INotifyItemsChanged)]
pub struct CommandProvider {
    id: HSTRING,
    display_name: HSTRING,
    icon: Option<ComObject<IconInfo>>,
    settings: Option<ICommandSettings>,
    frozen: bool,
    top_level: Vec<ICommandItem>,
    fallbacks: Vec<IFallbackCommandItem>,
    event: ItemsChangedEventHandler,
}

/// Builder for [`CommandProvider`].
pub struct CommandProviderBuilder {
    id: HSTRING,
    display_name: HSTRING,
    icon: Option<ComObject<IconInfo>>,
    settings: Option<ICommandSettings>,
    frozen: bool,
    top_level: Vec<ICommandItem>,
    fallbacks: Vec<IFallbackCommandItem>,
}

impl CommandProvider {
    /// Creates a new [`CommandProvider`] builder.
    pub fn builder() -> CommandProviderBuilder {
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
}

impl CommandProviderBuilder {
    /// Sets the ID of the command provider.
    pub fn id(mut self, id: impl Into<HSTRING>) -> Self {
        self.id = id.into();
        self
    }

    /// Sets the display name of the command provider.
    ///
    /// The name will be displayed at the settings page of the extension.
    pub fn display_name(mut self, display_name: impl Into<HSTRING>) -> Self {
        self.display_name = display_name.into();
        self
    }

    /// Sets the icon of the command provider.
    pub fn icon(mut self, icon: ComObject<IconInfo>) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Sets the settings page of the command provider.
    pub fn settings(mut self, settings: ICommandSettings) -> Self {
        self.settings = Some(settings);
        self
    }

    /// Sets whether the command provider is frozen.
    ///
    /// If frozen, Command Palette will try to cache the commands and call `GetCommand` to accelerate command retrieval process.
    pub fn frozen(mut self, frozen: bool) -> Self {
        self.frozen = frozen;
        self
    }

    /// Sets the top-level commands of the command provider.
    pub fn top_level(mut self, top_level: Vec<ICommandItem>) -> Self {
        self.top_level = top_level;
        self
    }

    /// Adds a top-level command to the command provider.
    pub fn add_top_level(mut self, item: CommandItemBuilder) -> Self {
        self.top_level.push(item.build().into_interface());
        self
    }

    /// Sets the fallback commands of the command provider.
    ///
    /// Fallback commands are dynamic commands that can respond to dynamic queries.
    pub fn fallbacks(mut self, fallbacks: Vec<IFallbackCommandItem>) -> Self {
        self.fallbacks = fallbacks;
        self
    }

    /// Adds a fallback command to the command provider.
    pub fn add_fallback(mut self, item: IFallbackCommandItem) -> Self {
        self.fallbacks.push(item);
        self
    }
}

impl ComBuilder for CommandProviderBuilder {
    type Output = CommandProvider;
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

impl ICommandProvider_Impl for CommandProvider_Impl {
    fn Id(&self) -> Result<HSTRING> {
        Ok(self.id.clone())
    }

    fn DisplayName(&self) -> Result<HSTRING> {
        Ok(self.display_name.clone())
    }

    fn Icon(&self) -> Result<crate::bindings::IIconInfo> {
        self.icon
            .clone()
            .map(|icon| icon.to_interface())
            .ok_or_empty()
    }

    fn Settings(&self) -> Result<ICommandSettings> {
        self.settings.clone().ok_or_empty()
    }

    fn Frozen(&self) -> Result<bool> {
        Ok(self.frozen)
    }

    fn TopLevelCommands(&self) -> Result<Array<ICommandItem>> {
        Ok(map_array(&self.top_level, |x| x.clone().into()))
    }

    fn FallbackCommands(&self) -> Result<Array<IFallbackCommandItem>> {
        Ok(map_array(&self.fallbacks, |x| x.clone().into()))
    }

    fn GetCommand(&self, _: &HSTRING) -> Result<ICommand> {
        Err(windows_core::Error::empty())
    }

    fn InitializeWithHost(&self, host: windows_core::Ref<'_, IExtensionHost>) -> Result<()> {
        crate::host::set_ext_host(host.ok()?);
        Ok(())
    }
}

impl ICommandProvider2_Impl for CommandProvider_Impl {
    fn GetApiExtensionStubs(&self) -> Result<Array<IInspectable>> {
        Ok(map_array(
            &[IInspectable::from(
                crate::ext_api_stubs::ExtendedAttributesProviderStub,
            )],
            |obj| Some(obj.clone()),
        ))
    }
}

impl IClosable_Impl for CommandProvider_Impl {
    fn Close(&self) -> Result<()> {
        Ok(())
    }
}

impl INotifyItemsChanged_Impl for CommandProvider_Impl {
    fn ItemsChanged(
        &self,
        handler: windows_core::Ref<'_, TypedEventHandler<IInspectable, IItemsChangedEventArgs>>,
    ) -> Result<i64> {
        self.event.add(handler.ok()?)
    }

    fn RemoveItemsChanged(&self, token: i64) -> Result<()> {
        self.event.remove(token);
        Ok(())
    }
}

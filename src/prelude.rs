//! Common types and traits used across the cmdpal library.

pub use crate::{
    cmd::{
        BaseCommand, BaseCommandBuilder, CommandResult, InvokableCommand,
        common::{CopyTextCommandBuilder, OpenUrlCommandBuilder, RevealFileCommandBuilder},
    },
    cmd_item::{CommandItem, CommandItemBuilder},
    cmd_provider::{CommandProvider, CommandProviderBuilder},
    content::{FormContent, FormContentBuilder, MarkdownContent, TreeContent, TreeContentBuilder},
    ctx_item::{CommandContextItem, CommandContextItemBuilder, ContextItem, SeparatorContextItem},
    details::{
        Details, DetailsBuilder, DetailsData, DetailsElement, DetailsLink, DetailsLinkBuilder,
        DetailsSeparator, DetailsTags, DetailsTagsBuilder, Tag, TagBuilder,
    },
    ext::Extension,
    ext_registry::ExtRegistry,
    fallback::FallbackCommandItem,
    filter::{Filter, FilterItem, FilterSeparator, Filters},
    grid::{SmallGridLayout, MediumGridLayout, GalleryGridLayout, GridLayout},
    host::{
        LogMessage, MessageState, ProgressState, ProgressStateBuilder, StatusContext,
        StatusMessage, StatusMessageBuilder, hide_status, log_message, show_status,
    },
    icon::{IconData, IconInfo},
    page::{
        BasePage, BasePageBuilder,
        content::{ContentPage, ContentPageBuilder},
        dyn_list::{DynamicListPage, DynamicListPageBuilder},
        list::{ListItem, ListItemBuilder, ListPage, ListPageBuilder},
    },
    settings::{
        Choice, ChoiceSetSetting, CommandSettings, JsonCommandSettings, NumberSetting,
        SettingBasePropModifier, TextSetting, ToggleSetting,
    },
    utils::{ComBuilder},
};

pub use windows_core::{
    ComObject, Error as WinError, GUID, HSTRING, IUnknownImpl as IUnknown_Impl,
    Result as WinResult, h,
};

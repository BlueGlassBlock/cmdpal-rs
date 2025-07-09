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
        Details, DetailsBuilder, DetailsLink, DetailsLinkBuilder, DetailsSeparator, DetailsTags,
        DetailsTagsBuilder, Tag, TagBuilder,
    },
    ext::Extension,
    ext_registry::ExtRegistry,
    fallback::FallbackCommandItem,
    filter::{Filter, FilterItem, FilterSeparator, Filters},
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
    utils::{ComBuilder, GridProperties},
};

pub use windows::core::{
    ComObject, Error as WinError, GUID, HSTRING, IUnknownImpl as IUnknown_Impl,
    Result as WinResult, h,
};

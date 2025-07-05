//! This module provides a prelude for the cmdpal library, re-exporting commonly used types and traits.
//! This allows users to import everything they need with a single `use cmdpal::prelude::*;` statement.

pub use crate::{
    ExtRegistry,
    bindings::StatusContext,
    cmd::{
        BaseCommand, BaseCommandBuilder, CommandResult,
        common::{
            InvokableCommand, copy_text::CopyTextCommandBuilder, no_op::NoOpCommandBuilder,
            open_url::OpenUrlCommandBuilder, reveal_file::RevealFileCommandBuilder,
        },
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
    fallback::FallbackCommandItem,
    filter::{Filter, FilterItem, Filters, SeparatorFilterItem},
    host::{LogMessage, MessageState, ProgressState, hide_status, log_message, show_status},
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

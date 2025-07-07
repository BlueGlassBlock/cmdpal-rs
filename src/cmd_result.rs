//! Execution result representation of [`crate::cmd::InvokableCommand`]

use crate::bindings::{
    CommandResultKind, ICommand, ICommandResult, ICommandResult_Impl, ICommandResultArgs,
    ICommandResultArgs_Impl, IConfirmationArgs, IConfirmationArgs_Impl, IGoToPageArgs,
    IGoToPageArgs_Impl, IToastArgs, IToastArgs_Impl,
};
use windows::{
    Win32::Foundation::ERROR_BAD_ARGUMENTS,
    core::{Error, Result as WinResult, implement},
};
use windows_core::{AgileReference, ComObject};

// windows::core::implement doesn't support `enum` yet, so we manually write out the VTables

/// Wrapper of [`ICommandResult`]
/// 
#[doc = include_str!("./bindings_docs/ICommandResult.md")]
// #[implement(ICommandResult)]
#[derive(Debug, Clone, Default)]
pub enum CommandResult {
    #[doc = include_str!("./bindings_docs/CommandResultKind/Dismiss.md")]
    Dismiss,
    #[doc = include_str!("./bindings_docs/CommandResultKind/GoHome.md")]
    GoHome,
    #[doc = include_str!("./bindings_docs/CommandResultKind/GoBack.md")]
    GoBack,
    #[doc = include_str!("./bindings_docs/CommandResultKind/Hide.md")]
    Hide,
    #[doc = include_str!("./bindings_docs/CommandResultKind/KeepOpen.md")]
    #[default]
    KeepOpen,
    #[doc = include_str!("./bindings_docs/CommandResultKind/GoToPage.md")]
    GoToPage(ComObject<GoToPageArgs>),
    #[doc = include_str!("./bindings_docs/CommandResultKind/ShowToast.md")]
    ShowToast(ComObject<ToastArgs>),
    #[doc = include_str!("./bindings_docs/CommandResultKind/Confirm.md")]
    Confirm(ComObject<ConfirmationArgs>),
}

/// A convenience wrapper for [`CommandResult`]
#[implement(ICommandResult)]
pub(crate) struct CommandResultStruct(CommandResult);

impl ICommandResult_Impl for CommandResultStruct_Impl {
    fn Kind(&self) -> WinResult<CommandResultKind> {
        match &self.0 {
            CommandResult::Dismiss => Ok(CommandResultKind::Dismiss),
            CommandResult::GoHome => Ok(CommandResultKind::GoHome),
            CommandResult::GoBack => Ok(CommandResultKind::GoBack),
            CommandResult::Hide => Ok(CommandResultKind::Hide),
            CommandResult::KeepOpen => Ok(CommandResultKind::KeepOpen),
            CommandResult::GoToPage(_) => Ok(CommandResultKind::GoToPage),
            CommandResult::ShowToast(_) => Ok(CommandResultKind::ShowToast),
            CommandResult::Confirm(_) => Ok(CommandResultKind::Confirm),
        }
    }
    fn Args(&self) -> WinResult<ICommandResultArgs> {
        match &self.0 {
            CommandResult::GoToPage(args) => Ok(args.to_interface()),
            CommandResult::ShowToast(args) => Ok(args.to_interface()),
            CommandResult::Confirm(args) => Ok(args.to_interface()),
            _ => Err(Error::empty()),
        }
    }
}

impl From<CommandResult> for ICommandResult {
    fn from(value: CommandResult) -> Self {
        CommandResultStruct(value).into()
    }
}

/// Wrapper of [`IGoToPageArgs`]
///
#[doc = include_str!("./bindings_docs/IGoToPageArgs.md")]
#[derive(Debug, Clone)]
#[implement(IGoToPageArgs, ICommandResultArgs)]
pub struct GoToPageArgs {
    #[doc = include_str!("./bindings_docs/IGoToPageArgs/NavigationMode.md")]
    pub navigation_mode: NavigationMode,
    #[doc = include_str!("./bindings_docs/IGoToPageArgs/PageId.md")]
    pub page_id: windows::core::HSTRING,
}

// TODO: Builder

impl ICommandResultArgs_Impl for GoToPageArgs_Impl {}

impl IGoToPageArgs_Impl for GoToPageArgs_Impl {
    fn NavigationMode(&self) -> windows_core::Result<crate::bindings::NavigationMode> {
        Ok(self.navigation_mode.into())
    }
    fn PageId(&self) -> windows_core::Result<windows::core::HSTRING> {
        Ok(self.page_id.clone())
    }
}

/// Wrapper of [`NavigationMode`][`crate::bindings::NavigationMode`]
/// 
#[doc = include_str!("./bindings_docs/NavigationMode.md")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NavigationMode {
    #[default]
    #[doc = include_str!("./bindings_docs/NavigationMode/Push.md")]
    Push,
    #[doc = include_str!("./bindings_docs/NavigationMode/GoBack.md")]
    GoBack,
    #[doc = include_str!("./bindings_docs/NavigationMode/GoHome.md")]
    GoHome,
}

impl TryFrom<crate::bindings::NavigationMode> for NavigationMode {
    type Error = Error;
    fn try_from(value: crate::bindings::NavigationMode) -> Result<Self, Self::Error> {
        match value {
            crate::bindings::NavigationMode::Push => Ok(NavigationMode::Push),
            crate::bindings::NavigationMode::GoBack => Ok(NavigationMode::GoBack),
            crate::bindings::NavigationMode::GoHome => Ok(NavigationMode::GoHome),
            _ => Err(ERROR_BAD_ARGUMENTS.into()),
        }
    }
}

impl From<NavigationMode> for crate::bindings::NavigationMode {
    fn from(value: NavigationMode) -> Self {
        match value {
            NavigationMode::Push => crate::bindings::NavigationMode::Push,
            NavigationMode::GoBack => crate::bindings::NavigationMode::GoBack,
            NavigationMode::GoHome => crate::bindings::NavigationMode::GoHome,
        }
    }
}

/// Wrapper of [`IToastArgs`]
/// 
#[doc = include_str!("./bindings_docs/IToastArgs.md")]
#[derive(Debug, Clone)]
#[implement(IToastArgs, ICommandResultArgs)]
pub struct ToastArgs {
    #[doc = include_str!("./bindings_docs/IToastArgs/Message.md")]
    pub message: windows::core::HSTRING,
    #[doc = include_str!("./bindings_docs/IToastArgs/Result.md")]
    pub result: CommandResult,
}

impl From<&windows::core::HSTRING> for ToastArgs {
    fn from(value: &windows::core::HSTRING) -> Self {
        Self {
            message: value.clone(),
            result: CommandResult::Dismiss,
        }
    }
}

impl ICommandResultArgs_Impl for ToastArgs_Impl {}

impl IToastArgs_Impl for ToastArgs_Impl {
    fn Message(&self) -> windows_core::Result<windows_core::HSTRING> {
        Ok(self.message.clone())
    }
    fn Result(&self) -> windows_core::Result<ICommandResult> {
        Ok(CommandResultStruct(self.result.clone()).into())
    }
}

/// Wrapper of [`IConfirmationArgs`]
/// 
#[doc = include_str!("./bindings_docs/IConfirmationArgs.md")]
#[derive(Debug, Clone)]
#[implement(IConfirmationArgs, ICommandResultArgs)]
pub struct ConfirmationArgs {
    #[doc = include_str!("./bindings_docs/IConfirmationArgs/Title.md")]
    pub title: windows::core::HSTRING,
    #[doc = include_str!("./bindings_docs/IConfirmationArgs/Description.md")]
    pub description: windows::core::HSTRING,
    #[doc = include_str!("./bindings_docs/IConfirmationArgs/PrimaryCommand.md")]
    pub primary_command: AgileReference<ICommand>,
    #[doc = include_str!("./bindings_docs/IConfirmationArgs/IsPrimaryCommandCritical.md")]
    pub is_primary_command_critical: bool,
}

impl ICommandResultArgs_Impl for ConfirmationArgs_Impl {}

impl IConfirmationArgs_Impl for ConfirmationArgs_Impl {
    fn Title(&self) -> windows_core::Result<windows_core::HSTRING> {
        Ok(self.title.clone())
    }
    fn Description(&self) -> windows_core::Result<windows_core::HSTRING> {
        Ok(self.description.clone())
    }
    fn PrimaryCommand(&self) -> windows_core::Result<crate::bindings::ICommand> {
        self.primary_command.resolve()
    }
    fn IsPrimaryCommandCritical(&self) -> windows_core::Result<bool> {
        Ok(self.is_primary_command_critical)
    }
}

//! Execution result representation of [`InvokableCommand`][`crate::cmd::InvokableCommand`]

use crate::{
    bindings::{
        CommandResultKind, ICommand, ICommandResult, ICommandResult_Impl, ICommandResultArgs,
        ICommandResultArgs_Impl, IConfirmationArgs, IConfirmationArgs_Impl, IGoToPageArgs,
        IGoToPageArgs_Impl, IToastArgs, IToastArgs_Impl,
    },
    utils::ComBuilder,
};
use windows::Win32::Foundation::ERROR_BAD_ARGUMENTS;
use windows_core::{AgileReference, ComObject, Error, HSTRING, Result, implement};

// implement doesn't support `enum` yet, so we manually write out the VTables

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
    fn Kind(&self) -> Result<CommandResultKind> {
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
    fn Args(&self) -> Result<ICommandResultArgs> {
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
    pub page_id: HSTRING,
}

impl GoToPageArgs {
    /// Creates a new unmanaged `GoToPageArgs` with the specified navigation mode and page ID.
    pub fn new_unmanaged(navigation_mode: NavigationMode, page_id: impl Into<HSTRING>) -> Self {
        Self {
            navigation_mode,
            page_id: page_id.into(),
        }
    }

    /// Creates a new reference-counted COM object for `GoToPageArgs`.
    pub fn new(
        navigation_mode: NavigationMode,
        page_id: impl Into<HSTRING>,
    ) -> Result<ComObject<Self>> {
        Ok(Self::new_unmanaged(navigation_mode, page_id).into())
    }
}

impl ICommandResultArgs_Impl for GoToPageArgs_Impl {}

impl IGoToPageArgs_Impl for GoToPageArgs_Impl {
    fn NavigationMode(&self) -> Result<crate::bindings::NavigationMode> {
        Ok(self.navigation_mode.into())
    }
    fn PageId(&self) -> Result<HSTRING> {
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
    fn try_from(value: crate::bindings::NavigationMode) -> Result<Self> {
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
    pub message: HSTRING,
    #[doc = include_str!("./bindings_docs/IToastArgs/Result.md")]
    pub result: CommandResult,
}

impl ToastArgs {
    /// Creates a new unmanaged `ToastArgs` with the specified message and result.
    pub fn new_unmanaged(message: impl Into<HSTRING>, result: CommandResult) -> Self {
        Self {
            message: message.into(),
            result,
        }
    }

    /// Creates a new reference-counted COM object for `ToastArgs`.
    pub fn new(message: impl Into<HSTRING>, result: CommandResult) -> Result<ComObject<Self>> {
        Ok(Self::new_unmanaged(message, result).into())
    }
}

impl From<&HSTRING> for ToastArgs {
    fn from(value: &HSTRING) -> Self {
        Self {
            message: value.clone(),
            result: CommandResult::Dismiss,
        }
    }
}

impl ICommandResultArgs_Impl for ToastArgs_Impl {}

impl IToastArgs_Impl for ToastArgs_Impl {
    fn Message(&self) -> Result<HSTRING> {
        Ok(self.message.clone())
    }
    fn Result(&self) -> Result<ICommandResult> {
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
    pub title: HSTRING,
    #[doc = include_str!("./bindings_docs/IConfirmationArgs/Description.md")]
    pub description: HSTRING,
    #[doc = include_str!("./bindings_docs/IConfirmationArgs/PrimaryCommand.md")]
    pub primary_command: AgileReference<ICommand>,
    #[doc = include_str!("./bindings_docs/IConfirmationArgs/IsPrimaryCommandCritical.md")]
    pub is_primary_command_critical: bool,
}

/// Builder for [`ConfirmationArgs`]
pub struct ConfirmationArgsBuilder {
    title: HSTRING,
    description: HSTRING,
    primary_command: AgileReference<ICommand>,
    is_primary_command_critical: bool,
}

impl ConfirmationArgsBuilder {
    /// Creates a new builder with the specified primary command.
    pub fn new(primary_command: AgileReference<ICommand>) -> Self {
        Self {
            title: HSTRING::new(),
            description: HSTRING::new(),
            primary_command,
            is_primary_command_critical: false,
        }
    }

    /// Creates a new builder with the specified primary command.
    pub fn try_new(primary_command: ICommand) -> Result<Self> {
        let agile_command = AgileReference::new(&primary_command)?;
        Ok(Self::new(agile_command))
    }

    /// Sets the title of the confirmation.
    pub fn title(mut self, title: impl Into<HSTRING>) -> Self {
        self.title = title.into();
        self
    }

    /// Sets the description of the confirmation.
    pub fn description(mut self, description: impl Into<HSTRING>) -> Self {
        self.description = description.into();
        self
    }

    /// Sets whether the primary command is critical.
    pub fn is_critical(mut self, is_critical: bool) -> Self {
        self.is_primary_command_critical = is_critical;
        self
    }
}

impl ComBuilder for ConfirmationArgsBuilder {
    type Output = ConfirmationArgs;
    fn build_unmanaged(self) -> ConfirmationArgs {
        ConfirmationArgs {
            title: self.title,
            description: self.description,
            primary_command: self.primary_command,
            is_primary_command_critical: self.is_primary_command_critical,
        }
    }
}

impl ICommandResultArgs_Impl for ConfirmationArgs_Impl {}

impl IConfirmationArgs_Impl for ConfirmationArgs_Impl {
    fn Title(&self) -> Result<HSTRING> {
        Ok(self.title.clone())
    }
    fn Description(&self) -> Result<HSTRING> {
        Ok(self.description.clone())
    }
    fn PrimaryCommand(&self) -> Result<crate::bindings::ICommand> {
        self.primary_command.resolve()
    }
    fn IsPrimaryCommandCritical(&self) -> Result<bool> {
        Ok(self.is_primary_command_critical)
    }
}

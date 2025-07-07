//! [`IExtensionHost`] related functions and types.
//!
//! Useful for sending feedbacks to users.

use crate::bindings::*;
use crate::notify::*;
use crate::utils::ComBuilder;
use std::sync::RwLock;
use windows::Win32::Foundation::E_INVALIDARG;
use windows::core::AgileReference;
use windows::core::{ComObject, IInspectable, IUnknownImpl as _, implement};

pub(crate) static EXTENSION_HOST: RwLock<Option<AgileReference<IExtensionHost>>> =
    RwLock::new(None);

/// Struct which represents the progress state of an operation.
///
#[doc = include_str!("./bindings_docs/IProgressState.md")]
#[implement(IProgressState, INotifyPropChanged)]
pub struct ProgressState {
    indeterminate: NotifyLock<bool>,
    percentage: NotifyLock<u32>,
    event: PropChangedEventHandler,
}

/// Builder for [`ProgressState`].
pub struct ProgressStateBuilder {
    indeterminate: bool,
    percentage: u32,
}

impl ProgressStateBuilder {
    /// Creates a new `ProgressStateBuilder`.
    pub fn new() -> Self {
        ProgressStateBuilder {
            indeterminate: true,
            percentage: 0,
        }
    }

    /// Sets the indeterminate state of the progress.
    pub fn indeterminate(mut self, indeterminate: bool) -> Self {
        self.indeterminate = indeterminate;
        self
    }

    /// Sets the progress percentage.
    pub fn percentage(mut self, percentage: u32) -> Self {
        self.percentage = percentage;
        self
    }
}

impl ComBuilder for ProgressStateBuilder {
    type Target = ProgressState;

    fn build_unmanaged(self) -> Self::Target {
        ProgressState {
            indeterminate: NotifyLock::new(self.indeterminate),
            percentage: NotifyLock::new(self.percentage),
            event: PropChangedEventHandler::new(),
        }
    }
}

impl ProgressState_Impl {
    pub(crate) fn emit_self_prop_changed(&self, prop: &str) {
        let sender: IInspectable = self.to_interface();
        let arg: IPropChangedEventArgs = PropChangedEventArgs(prop.into()).into();
        self.event.call(|handler| handler.Invoke(&sender, &arg));
    }

    /// Readonly access to [`IProgressState::IsIndeterminate`]
    ///
    #[doc = include_str!("./bindings_docs/IProgressState/IsIndeterminate.md")]
    pub fn indeterminate(&self) -> windows_core::Result<NotifyLockReadGuard<'_, bool>> {
        self.indeterminate.read()
    }

    /// Mutable access to [`IProgressState::IsIndeterminate`].
    ///
    #[doc = include_str!("./bindings_docs/IProgressState/IsIndeterminate.md")]
    ///
    /// Notifies the host about the change when dropping the guard.
    pub fn indeterminate_mut(
        &self,
    ) -> windows_core::Result<NotifyLockWriteGuard<'_, bool>> {
        self.indeterminate
            .write(|| self.emit_self_prop_changed("IsIndeterminate"))
    }

    /// Readonly access to [`IProgressState::ProgressPercent`]
    ///
    #[doc = include_str!("./bindings_docs/IProgressState/ProgressPercent.md")]
    pub fn percentage(&self) -> windows_core::Result<NotifyLockReadGuard<'_, u32>> {
        self.percentage.read()
    }

    /// Mutable access to [`IProgressState::ProgressPercent`].
    ///
    #[doc = include_str!("./bindings_docs/IProgressState/ProgressPercent.md")]
    ///
    /// Notifies the host about the change when dropping the guard.
    pub fn percentage_mut(&self) -> windows_core::Result<NotifyLockWriteGuard<'_, u32>> {
        self.percentage
            .write(|| self.emit_self_prop_changed("ProgressPercent"))
    }
}

impl INotifyPropChanged_Impl for ProgressState_Impl {
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

impl IProgressState_Impl for ProgressState_Impl {
    fn IsIndeterminate(&self) -> windows_core::Result<bool> {
        self.indeterminate.read().map(|x| *x)
    }

    fn ProgressPercent(&self) -> windows_core::Result<u32> {
        self.percentage.read().map(|x| *x)
    }
}

/// Rust idiomatic representation of [`crate::bindings::MessageState`]
///
#[doc = include_str!("./bindings_docs/MessageState.md")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MessageState {
    #[doc = include_str!("./bindings_docs/MessageState/Info.md")]
    Info,
    #[doc = include_str!("./bindings_docs/MessageState/Success.md")]
    Success,
    #[doc = include_str!("./bindings_docs/MessageState/Warning.md")]
    Warning,
    #[doc = include_str!("./bindings_docs/MessageState/Error.md")]
    Error,
}

impl TryFrom<crate::bindings::MessageState> for MessageState {
    type Error = windows_core::Error;
    fn try_from(value: crate::bindings::MessageState) -> Result<Self, windows_core::Error> {
        match value {
            crate::bindings::MessageState::Error => Ok(MessageState::Error),
            crate::bindings::MessageState::Warning => Ok(MessageState::Warning),
            crate::bindings::MessageState::Info => Ok(MessageState::Info),
            crate::bindings::MessageState::Success => Ok(MessageState::Success),
            _ => Err(E_INVALIDARG.into()),
        }
    }
}

impl From<MessageState> for crate::bindings::MessageState {
    fn from(value: MessageState) -> Self {
        match value {
            MessageState::Error => crate::bindings::MessageState::Error,
            MessageState::Warning => crate::bindings::MessageState::Warning,
            MessageState::Info => crate::bindings::MessageState::Info,
            MessageState::Success => crate::bindings::MessageState::Success,
        }
    }
}

/// Rust idiomatic representation of [`crate::bindings::StatusContext`]
///
#[doc = include_str!("./bindings_docs/StatusContext.md")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StatusContext {
    #[doc = include_str!("./bindings_docs/StatusContext/Page.md")]
    Page,
    #[doc = include_str!("./bindings_docs/StatusContext/Extension.md")]
    Extension,
}

impl TryFrom<crate::bindings::StatusContext> for StatusContext {
    type Error = windows_core::Error;
    fn try_from(value: crate::bindings::StatusContext) -> Result<Self, windows_core::Error> {
        match value {
            crate::bindings::StatusContext::Page => Ok(StatusContext::Page),
            crate::bindings::StatusContext::Extension => Ok(StatusContext::Extension),
            _ => Err(E_INVALIDARG.into()),
        }
    }
}

impl From<StatusContext> for crate::bindings::StatusContext {
    fn from(value: StatusContext) -> Self {
        match value {
            StatusContext::Page => crate::bindings::StatusContext::Page,
            StatusContext::Extension => crate::bindings::StatusContext::Extension,
        }
    }
}

/// Struct which represents a status message.
///
#[doc = include_str!("./bindings_docs/IStatusMessage.md")]
#[implement(IStatusMessage, INotifyPropChanged)]
pub struct StatusMessage {
    state: NotifyLock<MessageState>,
    progress: NotifyLock<ComObject<ProgressState>>,
    message: NotifyLock<windows_core::HSTRING>,
    event: PropChangedEventHandler,
}

/// Builder for [`StatusMessage`].
pub struct StatusMessageBuilder {
    state: MessageState,
    progress: ComObject<ProgressState>,
    message: windows_core::HSTRING,
}

impl StatusMessageBuilder {
    /// Creates a new `StatusMessageBuilder`.
    pub fn new() -> Self {
        StatusMessageBuilder {
            state: MessageState::Info,
            progress: ProgressStateBuilder::new().build(),
            message: windows_core::HSTRING::default(),
        }
    }

    /// Sets the state of the status message.
    pub fn state(mut self, state: MessageState) -> Self {
        self.state = state;
        self
    }

    /// Sets the progress of the status message.
    pub fn progress(mut self, progress: ComObject<ProgressState>) -> Self {
        self.progress = progress;
        self
    }

    /// Sets the message of the status message.
    pub fn message(mut self, message: windows_core::HSTRING) -> Self {
        self.message = message;
        self
    }

    /// Builds the `StatusMessage`.
    pub fn build(self) -> StatusMessage {
        StatusMessage {
            state: NotifyLock::new(self.state),
            progress: NotifyLock::new(self.progress),
            message: NotifyLock::new(self.message),
            event: PropChangedEventHandler::new(),
        }
    }
}

impl StatusMessage_Impl {
    pub(crate) fn emit_self_prop_changed(&self, prop: &str) {
        let sender: IInspectable = self.to_interface();
        let arg: IPropChangedEventArgs = PropChangedEventArgs(prop.into()).into();
        self.event.call(|handler| handler.Invoke(&sender, &arg));
    }

    /// Readonly access to [`IStatusMessage::State`]
    ///
    #[doc = include_str!("./bindings_docs/IStatusMessage/State.md")]
    pub fn state(&self) -> windows_core::Result<NotifyLockReadGuard<'_, MessageState>> {
        self.state.read()
    }

    /// Mutable access to [`IStatusMessage::State`].
    ///
    #[doc = include_str!("./bindings_docs/IStatusMessage/State.md")]
    ///
    /// Notifies the host about the change when dropping the guard.
    pub fn state_mut(
        &self,
    ) -> windows_core::Result<NotifyLockWriteGuard<'_, MessageState>> {
        self.state.write(|| self.emit_self_prop_changed("State"))
    }

    /// Readonly access to [`IStatusMessage::Progress`]
    ///
    #[doc = include_str!("./bindings_docs/IStatusMessage/Progress.md")]
    pub fn progress(
        &self,
    ) -> windows_core::Result<NotifyLockReadGuard<'_, ComObject<ProgressState>>> {
        self.progress.read()
    }

    /// Mutable access to [`IStatusMessage::Progress`].
    ///
    #[doc = include_str!("./bindings_docs/IStatusMessage/Progress.md")]
    ///
    /// Notifies the host about the change when dropping the guard.
    pub fn progress_mut(
        &self,
    ) -> windows_core::Result<NotifyLockWriteGuard<'_, ComObject<ProgressState>>> {
        self.progress
            .write(|| self.emit_self_prop_changed("Progress"))
    }

    /// Readonly access to [`IStatusMessage::Message`]
    ///
    #[doc = include_str!("./bindings_docs/IStatusMessage/Message.md")]
    pub fn message(&self) -> windows_core::Result<NotifyLockReadGuard<'_, windows_core::HSTRING>> {
        self.message.read()
    }

    /// Mutable access to [`IStatusMessage::Message`].
    ///
    #[doc = include_str!("./bindings_docs/IStatusMessage/Message.md")]
    ///
    /// Notifies the host about the change when dropping the guard.
    pub fn message_mut(
        &self,
    ) -> windows_core::Result<NotifyLockWriteGuard<'_, windows_core::HSTRING>> {
        self.message
            .write(|| self.emit_self_prop_changed("Message"))
    }
}

impl IStatusMessage_Impl for StatusMessage_Impl {
    fn State(&self) -> windows_core::Result<crate::bindings::MessageState> {
        self.state.read().map(|x| x.clone().into())
    }

    fn Progress(&self) -> windows_core::Result<IProgressState> {
        self.progress.read().map(|x| x.to_interface())
    }

    fn Message(&self) -> windows_core::Result<windows_core::HSTRING> {
        self.message.read().map(|x| x.clone())
    }
}

impl INotifyPropChanged_Impl for StatusMessage_Impl {
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

/// Struct which represents a log message.
///
#[doc = include_str!("./bindings_docs/ILogMessage.md")]
#[implement(ILogMessage)]
#[derive(Debug, Clone)]
pub struct LogMessage {
    #[doc = include_str!("./bindings_docs/ILogMessage/State.md")]
    pub state: MessageState,
    #[doc = include_str!("./bindings_docs/ILogMessage/Message.md")]
    pub message: windows_core::HSTRING,
}

impl LogMessage {
    /// Creates a new `LogMessage`.
    pub fn new(state: MessageState, message: windows_core::HSTRING) -> Self {
        LogMessage { state, message }
    }

    /// Creates a new `LogMessage` that represents an informational message.
    pub fn info(message: windows_core::HSTRING) -> Self {
        LogMessage::new(MessageState::Info, message)
    }

    /// Creates a new `LogMessage` that represents a success message.
    pub fn success(message: windows_core::HSTRING) -> Self {
        LogMessage::new(MessageState::Success, message)
    }

    /// Creates a new `LogMessage` that represents a warning message.
    pub fn warning(message: windows_core::HSTRING) -> Self {
        LogMessage::new(MessageState::Warning, message)
    }

    /// Creates a new `LogMessage` that represents an error message.
    pub fn error(message: windows_core::HSTRING) -> Self {
        LogMessage::new(MessageState::Error, message)
    }

    /// Logs the message to the host.
    ///
    /// The message will be logged into the log file, yet not shown to the user.
    /// 
    /// This method is a convenient wrapper around the [`log_message`] function,
    /// it consumes `self`, so clone the struct if you want to log it multiple times.
    ///
    pub fn log(self) {
        let ilog: ILogMessage = self.into();
        log_message(ilog);
    }
}

impl ILogMessage_Impl for LogMessage_Impl {
    fn State(&self) -> windows_core::Result<crate::bindings::MessageState> {
        Ok(self.state.clone().into())
    }

    fn Message(&self) -> windows_core::Result<windows_core::HSTRING> {
        Ok(self.message.clone())
    }
}

/// Sets the global extension host reference.
///
/// This function is automatically called
/// when [`crate::cmd_provider::CommandProvider`] is initialized by the Command Palette.
pub fn set_ext_host(host: &IExtensionHost) {
    let reference = AgileReference::new(host).unwrap();
    if let Ok(mut lock) = EXTENSION_HOST.write() {
        *lock = Some(reference);
    }
}

/// Shows a status message to the user.
///
/// The status message will appear as a "pop-up" in the Command Palette.
pub fn show_status(message: ComObject<StatusMessage>, context: StatusContext) {
    if let Ok(lock) = EXTENSION_HOST.read() {
        if let Some(host) = lock.as_ref().and_then(|x| x.resolve().ok()) {
            let _ = host.ShowStatus(message.as_interface(), context.into());
        }
    }
}

/// Hides a status message.
pub fn hide_status(message: ComObject<StatusMessage>) {
    if let Ok(lock) = EXTENSION_HOST.read() {
        if let Some(host) = lock.as_ref().and_then(|x| x.resolve().ok()) {
            let _ = host.HideStatus(message.as_interface());
        }
    }
}

/// Logs a message to the host.
///
/// The message will be logged into the log file, yet not shown to the user.
///
/// Consider [`LogMessage::log`] method for a more idiomatic way to log messages.
pub fn log_message(message: impl std::borrow::Borrow<ILogMessage>) {
    if let Ok(lock) = EXTENSION_HOST.read() {
        if let Some(host) = lock.as_ref().and_then(|x| x.resolve().ok()) {
            let _ = host.LogMessage(message.borrow());
        }
    }
}

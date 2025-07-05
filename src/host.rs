use crate::bindings::*;
use crate::notify::*;
use std::sync::RwLock;
use windows::core::AgileReference;
use windows::core::{ComObject, IInspectable, IUnknownImpl as _, implement};
use windows::Win32::Foundation::E_INVALIDARG;

pub(crate) static EXTENSION_HOST: RwLock<Option<AgileReference<IExtensionHost>>> =
    RwLock::new(None);

#[implement(IProgressState, INotifyPropChanged)]
pub struct ProgressState {
    indeterminate: NotifyLock<bool>,
    percentage: NotifyLock<u32>,
    event: PropChangedEventHandler,
}

impl ProgressState_Impl {
    pub(crate) fn emit_self_prop_changed(&self, prop: &str) {
        let sender: IInspectable = self.to_interface();
        let arg: IPropChangedEventArgs = PropChangedEventArgs(prop.into()).into();
        self.event.call(|handler| handler.Invoke(&sender, &arg));
    }

    pub fn indeterminate(&self) -> windows_core::Result<NotifyLockReadGuard<'_, bool>> {
        self.indeterminate.read()
    }

    pub fn indeterminate_mut(
        &self,
    ) -> windows_core::Result<NotifyLockWriteGuard<'_, bool, impl Fn()>> {
        self.indeterminate
            .write(|| self.emit_self_prop_changed("IsIndeterminate"))
    }

    pub fn percentage(&self) -> windows_core::Result<NotifyLockReadGuard<'_, u32>> {
        self.percentage.read()
    }

    pub fn percentage_mut(&self) -> windows_core::Result<NotifyLockWriteGuard<'_, u32, impl Fn()>> {
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MessageState {
    Info,
    Success,
    Warning,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StatusContext {
    Page,
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

#[implement(IStatusMessage, INotifyPropChanged)]
pub struct StatusMessage {
    state: NotifyLock<MessageState>,
    progress: NotifyLock<ComObject<ProgressState>>,
    message: NotifyLock<windows_core::HSTRING>,
    event: PropChangedEventHandler,
}

impl StatusMessage_Impl {
    pub(crate) fn emit_self_prop_changed(&self, prop: &str) {
        let sender: IInspectable = self.to_interface();
        let arg: IPropChangedEventArgs = PropChangedEventArgs(prop.into()).into();
        self.event.call(|handler| handler.Invoke(&sender, &arg));
    }

    pub fn state(&self) -> windows_core::Result<NotifyLockReadGuard<'_, MessageState>> {
        self.state.read()
    }

    pub fn state_mut(
        &self,
    ) -> windows_core::Result<NotifyLockWriteGuard<'_, MessageState, impl Fn()>> {
        self.state.write(|| self.emit_self_prop_changed("State"))
    }

    pub fn progress(
        &self,
    ) -> windows_core::Result<NotifyLockReadGuard<'_, ComObject<ProgressState>>> {
        self.progress.read()
    }

    pub fn progress_mut(
        &self,
    ) -> windows_core::Result<NotifyLockWriteGuard<'_, ComObject<ProgressState>, impl Fn()>> {
        self.progress
            .write(|| self.emit_self_prop_changed("Progress"))
    }

    pub fn message(&self) -> windows_core::Result<NotifyLockReadGuard<'_, windows_core::HSTRING>> {
        self.message.read()
    }

    pub fn message_mut(
        &self,
    ) -> windows_core::Result<NotifyLockWriteGuard<'_, windows_core::HSTRING, impl Fn()>> {
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

#[implement(ILogMessage)]
pub struct LogMessage(MessageState, windows_core::HSTRING);

impl ILogMessage_Impl for LogMessage_Impl {
    fn State(&self) -> windows_core::Result<crate::bindings::MessageState> {
        Ok(self.0.clone().into())
    }

    fn Message(&self) -> windows_core::Result<windows_core::HSTRING> {
        Ok(self.1.clone())
    }
}

pub fn set_ext_host(host: &IExtensionHost) {
    let reference = AgileReference::new(host).unwrap();
    if let Ok(mut lock) = EXTENSION_HOST.write() {
        *lock = Some(reference);
    }
}

pub fn show_status(message: &IStatusMessage, context: StatusContext) {
    if let Ok(lock) = EXTENSION_HOST.read() {
        if let Some(host) = lock.as_ref().and_then(|x| x.resolve().ok()) {
            let _ = host.ShowStatus(message, context.into());
        }
    }
}

pub fn hide_status(message: &IStatusMessage) {
    if let Ok(lock) = EXTENSION_HOST.read() {
        if let Some(host) = lock.as_ref().and_then(|x| x.resolve().ok()) {
            let _ = host.HideStatus(message);
        }
    }
}

pub fn log_message(message: &ILogMessage) {
    if let Ok(lock) = EXTENSION_HOST.read() {
        if let Some(host) = lock.as_ref().and_then(|x| x.resolve().ok()) {
            let _ = host.LogMessage(message);
        }
    }
}

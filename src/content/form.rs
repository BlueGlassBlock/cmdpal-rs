//! Form content that can be used to accept user input.
use crate::bindings::*;
use crate::cmd::CommandResult;
use crate::notify::*;
use crate::utils::{ComBuilder, assert_send_sync};
use windows_core::{ComObject, Event, HSTRING, IInspectable, IUnknownImpl as _, implement};

pub type SubmitBox = Box<
    dyn Send
        + Sync
        + Fn(&FormContent_Impl, &HSTRING, &HSTRING) -> windows_core::Result<CommandResult>,
>;

/// Form content that can be used to accept user input.
///
/// See [`FormContent_Impl`] for field accessors.
///
#[doc = include_str!("../bindings_docs/IFormContent.md")]
#[implement(IFormContent, IContent, INotifyPropChanged)]
pub struct FormContent {
    template_json: NotifyLock<HSTRING>,
    data_json: NotifyLock<HSTRING>,
    state_json: NotifyLock<HSTRING>,
    submit: SubmitBox,
    event: PropChangedEventHandler,
}

/// Builder for [`FormContent`].
pub struct FormContentBuilder {
    template_json: HSTRING,
    data_json: HSTRING,
    state_json: HSTRING,
    submit: SubmitBox,
}

impl FormContentBuilder {
    /// Creates a new builder.
    pub fn new() -> Self {
        FormContentBuilder {
            template_json: HSTRING::default(),
            data_json: HSTRING::default(),
            state_json: HSTRING::default(),
            submit: Box::new(|_, _, _| Ok(CommandResult::KeepOpen)),
        }
    }

    /// Sets the template JSON for the form.
    pub fn template_json(mut self, template_json: impl Into<HSTRING>) -> Self {
        self.template_json = template_json.into();
        self
    }

    /// Sets the data JSON for the form.
    ///
    /// # Note
    /// Currently not used by Command Palette?
    pub fn data_json(mut self, data_json: impl Into<HSTRING>) -> Self {
        self.data_json = data_json.into();
        self
    }

    /// Sets the state JSON for the form.
    ///
    /// # Note
    /// Currently not used by Command Palette?
    pub fn state_json(mut self, state_json: impl Into<HSTRING>) -> Self {
        self.state_json = state_json.into();
        self
    }

    /// Sets the submit handler for the form.
    ///
    /// # Note
    ///
    /// The submit function should accept (self, inputs, data) as parameters:
    /// - `self` is a `&FormContent_Impl` for convenient field access
    /// - `inputs` is a JSON string of raw form inputs (`Map<String, Value>`),
    /// - `data` is:
    ///     - empty string `""` if the action is not `Action.Submit`,
    ///     - string `"null"` if the action is `Action.Submit` with no `data` key (serialized `null`),
    ///     - a JSON string corresponding to the `data` key if the action is `Action.Submit` with a `data` key.
    ///
    pub fn submit<F>(mut self, submit: F) -> Self
    where
        F: Send
            + Sync
            + Fn(&FormContent_Impl, &HSTRING, &HSTRING) -> windows_core::Result<CommandResult>
            + 'static,
    {
        self.submit = Box::new(submit);
        self
    }
}

impl ComBuilder for FormContentBuilder {
    type Output = FormContent;
    fn build_unmanaged(self) -> FormContent {
        FormContent {
            template_json: NotifyLock::new(self.template_json),
            data_json: NotifyLock::new(self.data_json),
            state_json: NotifyLock::new(self.state_json),
            submit: self.submit,
            event: Event::new(),
        }
    }
}

impl FormContent_Impl {
    pub(crate) fn emit_self_prop_changed(&self, prop: &str) {
        let sender: IInspectable = self.to_interface();
        let arg: IPropChangedEventArgs = PropChangedEventArgs(prop.into()).into();
        self.event.call(|handler| handler.Invoke(&sender, &arg));
    }

    pub fn template_json(&self) -> windows_core::Result<NotifyLockReadGuard<'_, HSTRING>> {
        self.template_json.read()
    }

    pub fn template_json_mut(&self) -> windows_core::Result<NotifyLockWriteGuard<'_, HSTRING>> {
        self.template_json
            .write(|| self.emit_self_prop_changed("TemplateJson"))
    }

    pub fn data_json(&self) -> windows_core::Result<NotifyLockReadGuard<'_, HSTRING>> {
        self.data_json.read()
    }

    pub fn data_json_mut(&self) -> windows_core::Result<NotifyLockWriteGuard<'_, HSTRING>> {
        self.data_json
            .write(|| self.emit_self_prop_changed("DataJson"))
    }

    pub fn state_json(&self) -> windows_core::Result<NotifyLockReadGuard<'_, HSTRING>> {
        self.state_json.read()
    }

    pub fn state_json_mut(&self) -> windows_core::Result<NotifyLockWriteGuard<'_, HSTRING>> {
        self.state_json
            .write(|| self.emit_self_prop_changed("StateJson"))
    }
}

impl IFormContent_Impl for FormContent_Impl {
    fn TemplateJson(&self) -> windows_core::Result<windows_core::HSTRING> {
        Ok(self.template_json.read()?.clone())
    }

    fn DataJson(&self) -> windows_core::Result<windows_core::HSTRING> {
        Ok(self.data_json.read()?.clone())
    }

    fn StateJson(&self) -> windows_core::Result<windows_core::HSTRING> {
        Ok(self.state_json.read()?.clone())
    }

    fn SubmitForm(
        &self,
        inputs: &windows_core::HSTRING,
        data: &windows_core::HSTRING,
    ) -> windows_core::Result<ICommandResult> {
        (self.submit)(self, inputs, data).map(|x| x.into())
    }
}

impl IContent_Impl for FormContent_Impl {}
impl INotifyPropChanged_Impl for FormContent_Impl {
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

const _: () = assert_send_sync::<ComObject<FormContent>>();

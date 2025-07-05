use crate::bindings::*;
use crate::cmd::CommandResult;
use crate::notify::*;
use crate::utils::{ComBuilder, assert_send_sync};
use windows::core::{ComObject, Event, HSTRING, IInspectable, IUnknownImpl as _, implement};

pub type SubmitFn = Box<
    dyn Send
        + Sync
        + Fn(&FormContent_Impl, &HSTRING, &HSTRING) -> windows_core::Result<ComObject<CommandResult>>,
>;

#[implement(IFormContent, IContent, INotifyPropChanged)]
pub struct FormContent {
    template_json: NotifyLock<HSTRING>,
    data_json: NotifyLock<HSTRING>,
    state_json: NotifyLock<HSTRING>,
    submit: SubmitFn,
    event: PropChangedEventHandler,
}

pub struct FormContentBuilder {
    template_json: HSTRING,
    data_json: HSTRING,
    state_json: HSTRING,
    submit: SubmitFn,
}

impl FormContentBuilder {
    pub fn new() -> Self {
        FormContentBuilder {
            template_json: HSTRING::default(),
            data_json: HSTRING::default(),
            state_json: HSTRING::default(),
            submit: Box::new(|_, _, _| Ok(ComObject::new(CommandResult::KeepOpen))),
        }
    }

    pub fn template_json(mut self, template_json: HSTRING) -> Self {
        self.template_json = template_json;
        self
    }

    pub fn data_json(mut self, data_json: HSTRING) -> Self {
        self.data_json = data_json;
        self
    }

    pub fn state_json(mut self, state_json: HSTRING) -> Self {
        self.state_json = state_json;
        self
    }

    pub fn submit(mut self, submit: SubmitFn) -> Self {
        self.submit = submit;
        self
    }
}

impl ComBuilder for FormContentBuilder {
    type Target = FormContent;
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

    pub fn template_json_mut(
        &self,
    ) -> windows_core::Result<NotifyLockWriteGuard<'_, HSTRING, impl Fn()>> {
        self.template_json
            .write(|| self.emit_self_prop_changed("TemplateJson"))
    }

    pub fn data_json(&self) -> windows_core::Result<NotifyLockReadGuard<'_, HSTRING>> {
        self.data_json.read()
    }

    pub fn data_json_mut(
        &self,
    ) -> windows_core::Result<NotifyLockWriteGuard<'_, HSTRING, impl Fn()>> {
        self.data_json
            .write(|| self.emit_self_prop_changed("DataJson"))
    }

    pub fn state_json(&self) -> windows_core::Result<NotifyLockReadGuard<'_, HSTRING>> {
        self.state_json.read()
    }

    pub fn state_json_mut(
        &self,
    ) -> windows_core::Result<NotifyLockWriteGuard<'_, HSTRING, impl Fn()>> {
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
        (self.submit)(self, inputs, data).map(|x| x.to_interface())
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

use crate::bindings::*;
use crate::cmd::CommandResult;
use crate::notify::NotifyLock;
use windows::Foundation::TypedEventHandler;
use windows::core::{ComObject, Event, HSTRING, IInspectable, implement};

pub type SubmitFn = Box<
    dyn Fn(&FormContent_Impl, &HSTRING, &HSTRING) -> windows_core::Result<ComObject<CommandResult>>,
>;

#[implement(IFormContent, IContent, INotifyPropChanged)]
pub struct FormContent {
    template_json: NotifyLock<HSTRING>,
    data_json: NotifyLock<HSTRING>,
    state_json: NotifyLock<HSTRING>,
    submit: SubmitFn,
    event: Event<TypedEventHandler<IInspectable, IPropChangedEventArgs>>,
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

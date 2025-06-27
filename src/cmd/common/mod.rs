pub mod no_op;
pub mod open_url;
pub mod reveal_file;
pub mod copy_text;

use crate::bindings::*;
use windows::core::implement;
use windows_core::ComObject;
use windows_core::IInspectable;

pub use crate::cmd_result::CommandResult;

use super::BaseCommand;

pub type InvokableFn = Box<dyn Send + Sync + Fn(&IInspectable) -> windows_core::Result<CommandResult>>;

#[implement(IInvokableCommand, ICommand, INotifyPropChanged)]
pub struct InvokableCommand {
    pub base: ComObject<BaseCommand>,
    pub func: InvokableFn,
}

impl IInvokableCommand_Impl for InvokableCommand_Impl {
    fn Invoke(
        &self,
        sender: windows_core::Ref<'_, windows_core::IInspectable>,
    ) -> windows_core::Result<ICommandResult> {
        let result = (self.func)(sender.ok()?);
        result.map(|r| r.into())
    }
}

impl ICommand_Impl for InvokableCommand_Impl {
    ambassador_impl_ICommand_Impl! {
        body_struct(< >, ComObject<BaseCommand>, base)
    }
}

impl INotifyPropChanged_Impl for InvokableCommand_Impl {
    ambassador_impl_INotifyPropChanged_Impl! {
        body_struct(< >, ComObject<BaseCommand>, base)
    }
}

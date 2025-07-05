pub mod copy_text;
pub mod no_op;
pub mod open_url;
pub mod reveal_file;

use std::ops::Deref;

use crate::bindings::*;
use crate::cmd::BaseCommand_Impl;
use windows::core::implement;
use windows_core::ComObject;
use windows_core::IInspectable;

pub use crate::cmd_result::CommandResult;
pub use copy_text::CopyTextCommandBuilder;
pub use no_op::NoOpCommandBuilder;
pub use open_url::OpenUrlCommandBuilder;
pub use reveal_file::RevealFileCommandBuilder;

use super::BaseCommand;

pub type InvokableFn =
    Box<dyn Send + Sync + Fn(&IInspectable) -> windows_core::Result<CommandResult>>;

#[implement(IInvokableCommand, ICommand, INotifyPropChanged)]
pub struct InvokableCommand {
    pub base: ComObject<BaseCommand>,
    pub func: InvokableFn,
}

impl Deref for InvokableCommand {
    type Target = BaseCommand_Impl;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
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
    fn Icon(&self) -> windows_core::Result<IIconInfo> {
        self.base.Icon()
    }

    fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        self.base.Id()
    }

    fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        self.base.Name()
    }
}

impl INotifyPropChanged_Impl for InvokableCommand_Impl {
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
        self.base.PropChanged(handler)
    }

    fn RemovePropChanged(&self, token: i64) -> windows_core::Result<()> {
        self.base.RemovePropChanged(token)
    }
}

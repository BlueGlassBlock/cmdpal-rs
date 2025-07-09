use windows::core::ComObject;

use crate::{
    cmd::{BaseCommand, BaseCommandBuilder, CommandResult, InvokableCommand},
    utils::ComBuilder,
};

pub struct NoOpCommandBuilder {
    base: ComObject<BaseCommand>,
}

impl NoOpCommandBuilder {
    pub fn new() -> Self {
        Self {
            base: BaseCommandBuilder::new().build(),
        }
    }

    pub fn base(mut self, base: ComObject<BaseCommand>) -> Self {
        self.base = base;
        self
    }
}

impl ComBuilder for NoOpCommandBuilder {
    type Output = InvokableCommand;
    fn build_unmanaged(self) -> InvokableCommand {
        InvokableCommand {
            base: self.base,
            func: Box::new(|_| Ok(CommandResult::KeepOpen)),
        }
    }
}

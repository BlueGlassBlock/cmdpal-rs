use crate::{
    cmd::{BaseCommand, BaseCommandBuilder, CommandResult, InvokableCommand},
    icon::{IconData, IconInfo},
    utils::ComBuilder,
};
use windows::core::ComObject;

pub struct OpenUrlCommandBuilder {
    base: ComObject<BaseCommand>,
    target_fn: Box<dyn Send + Sync + Fn() -> String>,
    result: CommandResult,
}

fn open_url_base_cmd() -> ComObject<BaseCommand> {
    BaseCommandBuilder::new()
        .name("Open")
        .icon(IconInfo::from(IconData::from("\u{E8A7}")).into())
        .build()
}

impl OpenUrlCommandBuilder {
    pub fn new(target: String) -> Self {
        Self {
            base: open_url_base_cmd(),
            target_fn: Box::new(move || target.clone()),
            result: CommandResult::Dismiss,
        }
    }

    pub fn new_dyn<F>(target_fn: F) -> Self
    where
        F: Send + Sync + Fn() -> String + 'static,
    {
        Self {
            base: open_url_base_cmd(),
            target_fn: Box::new(target_fn),
            result: CommandResult::Dismiss,
        }
    }

    pub fn base(mut self, base: ComObject<BaseCommand>) -> Self {
        self.base = base;
        self
    }
    pub fn result(mut self, result: CommandResult) -> Self {
        self.result = result;
        self
    }
}

impl ComBuilder for OpenUrlCommandBuilder {
    type Output = InvokableCommand;
    fn build_unmanaged(self) -> InvokableCommand {
        InvokableCommand {
            base: self.base,
            func: Box::new(move |_| {
                shell_helper::open_in_shell(&(self.target_fn)())?;
                Ok(self.result.clone())
            }),
        }
    }
}

mod shell_helper {
    use windows::Win32::UI::Shell::{SHELLEXECUTEINFOW, ShellExecuteExW};
    use windows::Win32::UI::{Shell::SEE_MASK_NOCLOSEPROCESS, WindowsAndMessaging::SW_SHOWNORMAL};

    pub(super) fn open_in_shell(target: &str) -> windows::core::Result<()> {
        let mut sei = SHELLEXECUTEINFOW::default();
        sei.cbSize = std::mem::size_of::<SHELLEXECUTEINFOW>() as u32;
        sei.fMask = SEE_MASK_NOCLOSEPROCESS;
        sei.lpFile = windows::core::PCWSTR::from_raw(windows::core::HSTRING::from(target).as_ptr());
        sei.nShow = SW_SHOWNORMAL.0;

        unsafe { ShellExecuteExW(&mut sei) }
    }
}

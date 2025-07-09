//! Builder for creating commands that open URLs in the system's default browser.

use crate::{
    cmd::{BaseCommand, BaseCommandBuilder, CommandResult, InvokableCommand},
    icon::{IconData, IconInfo},
    utils::ComBuilder,
};
use windows::core::ComObject;

/// Builder for a command that opens a URL in the system's default browser.
pub struct OpenUrlCommandBuilder {
    base: ComObject<BaseCommand>,
    target_fn: Box<dyn Send + Sync + Fn() -> String>,
    result: CommandResult,
}

fn open_url_base_cmd() -> ComObject<BaseCommand> {
    BaseCommandBuilder::new()
        .name("Open")
        .icon(IconInfo::new(IconData::from("\u{E8A7}")))
        .build()
}

impl OpenUrlCommandBuilder {
    /// Creates a new `OpenUrlCommandBuilder` with a static target URL.
    pub fn new(target: String) -> Self {
        Self {
            base: open_url_base_cmd(),
            target_fn: Box::new(move || target.clone()),
            result: CommandResult::Dismiss,
        }
    }

    /// Creates a new `OpenUrlCommandBuilder` with a function that returns the target URL.
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

    /// Overrides the base command with a custom one.
    ///
    /// By default, it uses a command with the name "Open" and an OpenInNewWindow icon "\u{E8A7}".
    pub fn base(mut self, base: ComObject<BaseCommand>) -> Self {
        self.base = base;
        self
    }

    /// Overrides the action to be performed when the URL is opened.
    ///
    /// By default, it is set to [`CommandResult::Dismiss`].
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

//! Builder for creating commands that reveals a path in the system's file explorer.

use crate::{
    cmd::{BaseCommand, BaseCommandBuilder, CommandResult, InvokableCommand},
    icon::{IconData, IconInfo},
    utils::ComBuilder,
};
use windows::{Win32::Foundation::ERROR_FILE_INVALID, core::ComObject};

/// Builder for a command that reveals a file in the system's file explorer.
pub struct RevealFileCommandBuilder {
    base: ComObject<BaseCommand>,
    path_fn: Box<dyn Send + Sync + Fn() -> std::path::PathBuf>,
    result: CommandResult,
}

fn reveal_file_base_cmd() -> ComObject<BaseCommand> {
    BaseCommandBuilder::new()
        .name("Show in folder")
        .icon(IconInfo::new(IconData::from("\u{E838}")))
        .build()
}

impl RevealFileCommandBuilder {
    /// Creates a new `RevealFileCommandBuilder` with a static path.
    pub fn new(path: std::path::PathBuf) -> Self {
        Self {
            base: reveal_file_base_cmd(),
            path_fn: Box::new(move || path.clone()),
            result: CommandResult::Dismiss,
        }
    }

    /// Creates a new `RevealFileCommandBuilder` with a function that returns the path.
    pub fn new_dyn<F>(path_fn: F) -> Self
    where
        F: Send + Sync + Fn() -> std::path::PathBuf + 'static,
    {
        Self {
            base: reveal_file_base_cmd(),
            path_fn: Box::new(path_fn),
            result: CommandResult::Dismiss,
        }
    }

    /// Sets the base command for this reveal file command.
    ///
    /// By default, the base command has name "Show in folder" with a folder icon "\u{E8B7}".
    pub fn base(mut self, base: ComObject<BaseCommand>) -> Self {
        self.base = base;
        self
    }
    pub fn result(mut self, result: CommandResult) -> Self {
        self.result = result;
        self
    }
}

impl ComBuilder for RevealFileCommandBuilder {
    type Output = InvokableCommand;
    fn build_unmanaged(self) -> InvokableCommand {
        InvokableCommand {
            base: self.base,
            func: Box::new(move |_| {
                let path = (self.path_fn)()
                    .canonicalize()
                    .map_err(|_| windows::core::Error::from(ERROR_FILE_INVALID))?;
                match path.try_exists() {
                    Ok(true) => {
                        explorer_helper::reveal_file(&path.to_string_lossy().replace("/", r"\"))?;
                        Ok(self.result.clone())
                    }
                    _ => Err(windows::core::Error::from(ERROR_FILE_INVALID)),
                }
            }),
        }
    }
}

mod explorer_helper {
    use windows::Win32::UI::Shell::{SHELLEXECUTEINFOW, ShellExecuteExW};
    use windows::Win32::UI::{Shell::SEE_MASK_NOCLOSEPROCESS, WindowsAndMessaging::SW_SHOWNORMAL};
    use windows::core::{HSTRING, PCWSTR, w};

    pub(super) fn reveal_file(target: &str) -> windows::core::Result<()> {
        let params = format!("/select,\"{}\"", target);
        println!("Revealing file in explorer with params: {}", params);
        let mut sei = SHELLEXECUTEINFOW::default();
        sei.cbSize = std::mem::size_of::<SHELLEXECUTEINFOW>() as u32;
        sei.fMask = SEE_MASK_NOCLOSEPROCESS;
        sei.lpFile = w!("explorer.exe");
        sei.lpParameters = PCWSTR::from_raw(HSTRING::from(params).as_ptr());
        sei.nShow = SW_SHOWNORMAL.0;

        unsafe { ShellExecuteExW(&mut sei) }
    }
}

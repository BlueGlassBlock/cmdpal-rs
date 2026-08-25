//! Builder for creating commands that reveals a path in the system's file explorer.

use windows::Win32::Foundation::ERROR_FILE_INVALID;
use windows::Win32::UI::Shell::{SEE_MASK_NOCLOSEPROCESS, SHELLEXECUTEINFOW, ShellExecuteExW};
use windows_core::ComObject;

use crate::cmd::{BaseCommand, CommandResult, InvokableCommand};
use crate::icon::{IconData, IconInfo};
use crate::utils::ComBuilder;

/// Builder for a command that reveals a file in the system's file explorer.
pub struct RevealFileCommand {
    base: ComObject<BaseCommand>,
    path_fn: Box<dyn Send + Sync + Fn() -> std::path::PathBuf>,
    result: CommandResult,
}

fn reveal_file_base_cmd() -> ComObject<BaseCommand> {
    BaseCommand::builder()
        .name("Show in folder")
        .icon(IconInfo::new(IconData::from("\u{E838}")))
        .build()
}

impl RevealFileCommand {
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

impl ComBuilder for RevealFileCommand {
    type Output = InvokableCommand;
    fn build_unmanaged(self) -> InvokableCommand {
        InvokableCommand {
            base: self.base,
            func: Box::new(move |_| {
                let path = (self.path_fn)()
                    .canonicalize()
                    .map_err(|_| ERROR_FILE_INVALID)?;
                match path.try_exists() {
                    Ok(true) => {
                        reveal_file(&path.to_string_lossy().replace("/", r"\"))?;
                        Ok(self.result.clone())
                    }
                    _ => Err(ERROR_FILE_INVALID.into()),
                }
            }),
        }
    }
}

fn reveal_file(target: &str) -> windows_core::Result<()> {
    let mut sei = SHELLEXECUTEINFOW {
        cbSize: std::mem::size_of::<SHELLEXECUTEINFOW>() as u32,
        fMask: SEE_MASK_NOCLOSEPROCESS,
        lpFile: windows_core::w!("explorer.exe"),
        lpParameters: windows_core::PCWSTR::from_raw(
            windows_core::HSTRING::from(format!("/select,\"{target}\"")).as_ptr(),
        ),
        nShow: windows::Win32::UI::WindowsAndMessaging::SW_SHOWNORMAL.0,
        ..Default::default()
    };

    unsafe { ShellExecuteExW(&mut sei) }
}

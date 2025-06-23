use super::InvokableCommand;
use crate::{
    cmd::{BaseCommand, BaseCommandBuilder, CommandResult},
    icon::{IconData, IconInfo},
    utils::ComBuilder,
};
use windows::{Win32::Foundation::ERROR_FILE_INVALID, core::ComObject};

pub struct RevealFileCommandBuilder {
    base: ComObject<BaseCommand>,
    path_fn: Box<dyn Fn() -> std::path::PathBuf>,
    result: CommandResult,
}

fn reveal_file_base_cmd() -> ComObject<BaseCommand> {
    BaseCommandBuilder::new()
        .name("Show in folder")
        .icon(IconInfo::from(IconData::from("\u{E838}")).into())
        .build()
}

impl RevealFileCommandBuilder {
    pub fn new(path: std::path::PathBuf) -> Self {
        Self {
            base: reveal_file_base_cmd(),
            path_fn: Box::new(move || path.clone()),
            result: CommandResult::Dismiss,
        }
    }

    pub fn new_dyn(path_fn: Box<dyn Fn() -> std::path::PathBuf>) -> Self {
        Self {
            base: reveal_file_base_cmd(),
            path_fn,
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

impl ComBuilder<InvokableCommand> for RevealFileCommandBuilder {
    fn build_unmanaged(self) -> InvokableCommand {
        InvokableCommand {
            base: self.base,
            func: Box::new(move |_| {
                let f = &self.path_fn;
                let path = f()
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

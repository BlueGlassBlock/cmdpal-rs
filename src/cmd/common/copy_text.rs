use crate::{
    cmd::{BaseCommand, BaseCommandBuilder, CommandResult, InvokableCommand},
    cmd_result::ToastArgs,
    icon::{IconData, IconInfo},
    utils::ComBuilder,
};
use windows::core::{ComObject, HSTRING, h};

pub struct CopyTextCommandBuilder {
    base: ComObject<BaseCommand>,
    text_fn: Box<dyn Send + Sync + Fn() -> HSTRING>,
    result: CommandResult,
}

fn copy_text_base_cmd() -> ComObject<BaseCommand> {
    BaseCommandBuilder::new()
        .name("Copy")
        .icon(IconInfo::from(IconData::from("\u{E8C8}")).into())
        .build()
}

impl CopyTextCommandBuilder {
    pub fn new(text: HSTRING) -> Self {
        Self {
            base: copy_text_base_cmd(),
            text_fn: Box::new(move || text.clone()),
            result: CommandResult::ShowToast(ToastArgs::from(h!("Copied to clipboard")).into()),
        }
    }

    pub fn new_dyn(text_fn: Box<dyn Send + Sync + Fn() -> HSTRING>) -> Self {
        Self {
            base: copy_text_base_cmd(),
            text_fn,
            result: CommandResult::ShowToast(ToastArgs::from(h!("Copied to clipboard")).into()),
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

impl ComBuilder for CopyTextCommandBuilder {
    type Target = InvokableCommand;
    fn build_unmanaged(self) -> InvokableCommand {
        InvokableCommand {
            base: self.base,
            func: Box::new(move |_| {
                clipboard_helper::set_clipboard_text((self.text_fn)())?;
                Ok(self.result.clone())
            }),
        }
    }
}

mod clipboard_helper {
    use windows::Win32::Foundation::{E_FAIL, E_POINTER, ERROR_LOCKED, GlobalFree, HANDLE};
    use windows::Win32::System::Com::{COINIT_APARTMENTTHREADED, CoInitializeEx};
    use windows::Win32::System::DataExchange::{
        CloseClipboard, EmptyClipboard, OpenClipboard, SetClipboardData,
    };
    use windows::Win32::System::Memory::{GHND, GlobalAlloc, GlobalLock, GlobalUnlock};
    use windows::Win32::System::Ole::CF_UNICODETEXT;
    use windows::core::{HSTRING, Result};

    pub(super) fn set_clipboard_text(text: HSTRING) -> Result<()> {
        // start a new thread with STA
        std::thread::spawn(move || {
            const RETRY_COUNT: usize = 5;
            let mut retries = 0;
            let mut result = E_POINTER.ok();
            while retries < RETRY_COUNT {
                result = set_clipboard_text_sta(&text);
                if result.is_ok() {
                    return result;
                }
                retries += 1;
            }
            return result;
        })
        .join()
        .map_err(|_| windows::core::Error::from(E_FAIL))??;
        Ok(())
    }

    fn set_clipboard_text_sta(text: &HSTRING) -> Result<()> {
        unsafe {
            CoInitializeEx(None, COINIT_APARTMENTTHREADED).ok()?;
            let mem = GlobalAlloc(GHND, size_of::<u16>() * (text.len() + 1))?;
            let ptr = GlobalLock(mem) as *mut u16;
            if ptr.is_null() {
                return E_POINTER.ok();
            }
            ptr.copy_from((*text).as_ptr(), text.len());
            ptr.offset(text.len() as isize).write(0);

            let result = (|| -> Result<()> {
                match GlobalUnlock(mem) {
                    Ok(_) => ERROR_LOCKED.ok()?,
                    Err(e) if e.code().0 != 0 => Err(e)?,
                    Err(_) => {}
                };
                OpenClipboard(None)?;
                EmptyClipboard()?;
                SetClipboardData(CF_UNICODETEXT.0.into(), Some(HANDLE(mem.0)))?;
                CloseClipboard()?;
                Ok(())
            })();
            if result.is_err() {
                GlobalFree(Some(mem))?;
                return result;
            }
            Ok(())
        }
    }
}

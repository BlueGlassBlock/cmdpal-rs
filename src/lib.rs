pub mod bindings;
pub mod cmd;
pub mod cmd_item;
pub mod cmd_provider;
pub mod cmd_result;
pub mod content;
pub mod ctx_item;
pub mod details;
pub mod ext;
pub mod ext_factory;
pub mod fallback;
pub mod filter;
pub mod host;
pub mod icon;
pub mod notify;
pub mod page;
pub mod prelude;
pub mod settings;
pub(crate) mod utils;

pub struct ExtRegistry {
    factories: Vec<(
        windows::core::GUID,
        windows::core::ComObject<ext::Extension>,
    )>,
}

impl ExtRegistry {
    pub fn new() -> Self {
        ExtRegistry {
            factories: Vec::new(),
        }
    }

    pub fn register(
        mut self,
        guid: windows::core::GUID,
        extension: impl Into<windows::core::ComObject<ext::Extension>>,
    ) -> Self {
        self.factories.push((guid, extension.into()));
        self
    }

    pub fn serve(self) -> windows::core::Result<()> {
        use windows::Win32::System::Com::{
            CLSCTX_INPROC_SERVER, CLSCTX_LOCAL_SERVER, CLSID_GlobalOptions, COINIT_MULTITHREADED,
            COMGLB_FAST_RUNDOWN, COMGLB_RO_SETTINGS, CoCreateInstance, CoInitializeEx,
            CoRegisterClassObject, CoResumeClassObjects, IClassFactory, IGlobalOptions,
            REGCLS_MULTIPLEUSE, REGCLS_SUSPENDED,
        };
        use windows::Win32::UI::WindowsAndMessaging::{
            DispatchMessageW, GetMessageW, MSG, TranslateMessage,
        };

        // SAFETY: Following code are safe to run on a properly initialized Windows environment
        unsafe {
            CoInitializeEx(None, COINIT_MULTITHREADED).ok()?;
            let global_options: IGlobalOptions =
                CoCreateInstance(&CLSID_GlobalOptions, None, CLSCTX_INPROC_SERVER)?;
            global_options.Set(COMGLB_RO_SETTINGS, COMGLB_FAST_RUNDOWN.0 as usize)?;

            for (guid, extension) in self.factories.into_iter() {
                let factory: IClassFactory = ext_factory::ExtensionClassFactory(extension).into();
                CoRegisterClassObject(
                    &guid,
                    &factory,
                    CLSCTX_LOCAL_SERVER,
                    REGCLS_MULTIPLEUSE | REGCLS_SUSPENDED,
                )?;
            }
            CoResumeClassObjects()?;

            let mut msg: MSG = MSG::default();
            while GetMessageW(&mut msg, None, 0, 0).as_bool() {
                if TranslateMessage(&msg).as_bool() {
                    DispatchMessageW(&msg);
                }
            }
        }
        Ok(())
    }
}

#[cfg(feature = "unstable-doc")]
pub mod _cookbook;

//! Convenient extension registry for Command Palette extensions.

use crate::ext::Extension;
use crate::ext_factory::ExtensionClassFactory;
use windows::Win32::System::Com::{
    CLSCTX_INPROC_SERVER, CLSCTX_LOCAL_SERVER, CLSID_GlobalOptions, COINIT_MULTITHREADED,
    COMGLB_FAST_RUNDOWN, COMGLB_RO_SETTINGS, CoCreateInstance, CoInitializeEx,
    CoRegisterClassObject, CoResumeClassObjects, IClassFactory, IGlobalOptions, REGCLS_MULTIPLEUSE,
    REGCLS_SUSPENDED,
};
use windows::Win32::UI::WindowsAndMessaging::{
    DispatchMessageW, GetMessageW, MSG, TranslateMessage,
};
use windows::core::{ComObject, GUID, Result};

/// A registry for extensions that can be registered and served.
/// Convenient for building a extension executable that can host multiple extensions.
///
pub struct ExtRegistry {
    pub(crate) factories: Vec<(GUID, ComObject<ExtensionClassFactory>)>,
}

impl ExtRegistry {
    /// Create a new extension registry.
    pub fn new() -> Self {
        ExtRegistry {
            factories: Vec::new(),
        }
    }

    /// Register an extension with the given GUID.
    ///
    /// The GUID should match GUID specified in `AppxManifest.xml` file
    pub fn register(mut self, guid: GUID, extension: impl Into<ComObject<Extension>>) -> Self {
        self.factories
            .push((guid, ExtensionClassFactory(extension.into()).into()));
        self
    }

    /// Register an extension factory with the given GUID.
    ///
    /// The GUID should match GUID specified in `AppxManifest.xml` file.
    ///
    /// Useful when you want to register a custom factory directly.
    pub fn register_factory(
        mut self,
        guid: GUID,
        factory: impl Into<ComObject<ExtensionClassFactory>>,
    ) -> Self {
        self.factories.push((guid, factory.into()));
        self
    }

    /// Start serving the registered extensions.
    ///
    /// This will initialize COM, register the class objects,
    /// and start a message loop to process messages.
    /// This method will run indefinitely until the message loop is exited.
    ///
    pub fn serve(self) -> Result<()> {
        // SAFETY: Following code are safe to run on a properly initialized Windows environment
        unsafe {
            CoInitializeEx(None, COINIT_MULTITHREADED).ok()?;
            let global_options: IGlobalOptions =
                CoCreateInstance(&CLSID_GlobalOptions, None, CLSCTX_INPROC_SERVER)?;
            global_options.Set(COMGLB_RO_SETTINGS, COMGLB_FAST_RUNDOWN.0 as usize)?;

            for (guid, factory) in self.factories.into_iter() {
                let ifactory: IClassFactory = factory.to_interface();
                CoRegisterClassObject(
                    &guid,
                    &ifactory,
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

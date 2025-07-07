//! Convenient implementation of `IClassFactory` for extensions.



use crate::bindings::IExtension;
use crate::ext::Extension;
use windows::Win32::Foundation::E_POINTER;
use windows::Win32::System::Com::{IClassFactory, IClassFactory_Impl};
use windows::core::{ComObject, implement};
use windows_core::Interface;

/// A class factory for Command Palette extensions.
/// 
/// Automatically used by [`crate::ext_registry::ExtRegistry::register`] to register extensions.
#[implement(IClassFactory)]
pub struct ExtensionClassFactory(pub ComObject<Extension>);

impl IClassFactory_Impl for ExtensionClassFactory_Impl {
    /// SAFETY: This function does not validate the validity of `iid` and `interface`.
    /// Therefore, it is inherently unsound. The caller must ensure that `iid` and `interface`
    /// are valid pointers. On the Rust side, `CoCreateInstance` is already an unsafe function,
    /// and we rely on the caller to carefully validate their pointers. For FFI calls, we must
    /// similarly trust that the caller has ensured pointer validity.
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn CreateInstance(
        &self,
        _: windows_core::Ref<'_, windows_core::IUnknown>,
        iid: *const windows_core::GUID,
        interface: *mut *mut core::ffi::c_void,
    ) -> windows_core::Result<()> {
        // Validate the interface pointer for minimal safety.
        if iid.is_null() || interface.is_null() {
            return Err(E_POINTER.into());
        }
        let extension = self.0.cast::<IExtension>()?;
        unsafe {
            // SAFETY: described in documentation.
            extension.query(iid, interface).ok()
        }
    }

    fn LockServer(&self, _: windows_core::BOOL) -> windows_core::Result<()> {
        Ok(())
    }
}

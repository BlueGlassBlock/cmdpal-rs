//! API Extension stubs for future interfaces in Command Palette.
//!
//! Should be registered through [`ICommandProvider2::GetApiExtensionStubs`].
//!
//! The [`crate::cmd_provider::CommandProvider`] provides them all by default.
//!
use crate::bindings::*;
use windows_core::{Error, implement};

/// Stub implementation of [`IExtendedAttributesProvider`].
///
#[implement(IExtendedAttributesProvider)]
pub struct ExtendedAttributesProviderStub;

impl IExtendedAttributesProvider_Impl for ExtendedAttributesProviderStub_Impl {
    fn GetProperties(
        &self,
    ) -> windows_core::Result<
        windows_collections::IMap<windows_core::HSTRING, windows_core::IInspectable>,
    > {
        Err(Error::empty())
    }
}

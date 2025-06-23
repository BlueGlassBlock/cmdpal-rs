use crate::bindings::{CommandResultKind, ICommand, ICommandResult, ICommandResultArgs, ICommandResultArgs_Impl, ICommandResult_Impl, IConfirmationArgs, IConfirmationArgs_Impl, IGoToPageArgs, IGoToPageArgs_Impl, IToastArgs, IToastArgs_Impl};
use windows::{core::{implement, Error, Result as WinResult}, Win32::Foundation::ERROR_BAD_ARGUMENTS};
use windows_core::ComObject;



// windows::core::implement doesn't support `enum` yet, so we manually write out the VTables

// #[implement(ICommandResult)]
#[derive(Debug, Clone)]
pub enum CommandResult {
    Dismiss,
    GoHome,
    GoBack,
    Hide,
    KeepOpen,
    GoToPage(ComObject<GoToPageArgs>),
    ShowToast(ComObject<ToastArgs>),
    Confirm(ComObject<ConfirmationArgs>),
}

impl ICommandResult_Impl for CommandResult_Impl {
    fn Kind(&self) -> WinResult<CommandResultKind> {
        match self.this {
            CommandResult::Dismiss => Ok(CommandResultKind::Dismiss),
            CommandResult::GoHome => Ok(CommandResultKind::GoHome),
            CommandResult::GoBack => Ok(CommandResultKind::GoBack),
            CommandResult::Hide => Ok(CommandResultKind::Hide),
            CommandResult::KeepOpen => Ok(CommandResultKind::KeepOpen),
            CommandResult::GoToPage(_) => Ok(CommandResultKind::GoToPage),
            CommandResult::ShowToast(_) => Ok(CommandResultKind::ShowToast),
            CommandResult::Confirm(_) => Ok(CommandResultKind::Confirm),
        }
    }
    fn Args(&self) -> WinResult<ICommandResultArgs> {
        match &self.this {
            CommandResult::Dismiss => Err(Error::empty()),
            CommandResult::GoHome => Err(Error::empty()),
            CommandResult::GoBack => Err(Error::empty()),
            CommandResult::Hide => Err(Error::empty()),
            CommandResult::KeepOpen => Err(Error::empty()),
            CommandResult::GoToPage(args) => Ok(args.to_interface()),
            CommandResult::ShowToast(args) => Ok(args.to_interface()),
            CommandResult::Confirm(args) => Ok(args.to_interface()),
        }
    }
}

#[derive(Debug, Clone)]
#[implement(IGoToPageArgs, ICommandResultArgs)]
pub struct GoToPageArgs {
    pub navigation_mode: NavigationMode,
    pub page_id: windows::core::HSTRING,
}

impl ICommandResultArgs_Impl for GoToPageArgs_Impl {}

impl IGoToPageArgs_Impl for GoToPageArgs_Impl {
    fn NavigationMode(&self) -> windows_core::Result<crate::bindings::NavigationMode> {
        Ok(self.navigation_mode.into())
    }
    fn PageId(&self) -> windows_core::Result<windows::core::HSTRING> {
        Ok(self.page_id.clone())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NavigationMode {
    #[default]
    Push,
    GoBack,
    GoHome,
}

impl TryFrom<crate::bindings::NavigationMode> for NavigationMode {
    type Error = Error;
    fn try_from(value: crate::bindings::NavigationMode) -> Result<Self, Self::Error> {
        match value {
            crate::bindings::NavigationMode::Push => Ok(NavigationMode::Push),
            crate::bindings::NavigationMode::GoBack => Ok(NavigationMode::GoBack),
            crate::bindings::NavigationMode::GoHome => Ok(NavigationMode::GoHome),
            _ => Err(ERROR_BAD_ARGUMENTS.into()),
        }
    }
}

impl From<NavigationMode> for crate::bindings::NavigationMode {
    fn from(value: NavigationMode) -> Self {
        match value {
            NavigationMode::Push => crate::bindings::NavigationMode::Push,
            NavigationMode::GoBack => crate::bindings::NavigationMode::GoBack,
            NavigationMode::GoHome => crate::bindings::NavigationMode::GoHome,
        }
    }
}

#[derive(Debug, Clone)]
#[implement(IToastArgs, ICommandResultArgs)]
pub struct ToastArgs {
    pub message: windows::core::HSTRING,
    pub result: ComObject<CommandResult>,
}

impl From<&windows::core::HSTRING> for ToastArgs {
    fn from(value: &windows::core::HSTRING) -> Self {
        Self {
            message: value.clone(),
            result: ComObject::new(CommandResult::Dismiss),
        }
    }
}

impl ICommandResultArgs_Impl for ToastArgs_Impl {}

impl IToastArgs_Impl for ToastArgs_Impl {
    fn Message(&self) -> windows_core::Result<windows_core::HSTRING> {
        Ok(self.message.clone())
    }
    fn Result(&self) -> windows_core::Result<ICommandResult> {
        Ok(self.result.to_interface())   
    }
}

#[derive(Debug, Clone)]
#[implement(IConfirmationArgs, ICommandResultArgs)]
pub struct ConfirmationArgs {
    pub title: windows::core::HSTRING,
    pub description: windows::core::HSTRING,
    pub primary_command: ICommand, // TODO: ComObject ...
    pub is_primary_command_critical: bool,
}

impl ICommandResultArgs_Impl for ConfirmationArgs_Impl {}

impl IConfirmationArgs_Impl for ConfirmationArgs_Impl {
    fn Title(&self) -> windows_core::Result<windows_core::HSTRING> {
        Ok(self.title.clone())
    }
    fn Description(&self) -> windows_core::Result<windows_core::HSTRING> {
        Ok(self.description.clone())
    }
    fn PrimaryCommand(&self) -> windows_core::Result<crate::bindings::ICommand> {
        Ok(self.primary_command.clone())
    }
    fn IsPrimaryCommandCritical(&self) -> windows_core::Result<bool> {
        Ok(self.is_primary_command_critical)
    }
}

// region: implement ICommandResult for CommandResult

impl CommandResult {
    #[inline(always)]
    const fn into_outer(self) -> CommandResult_Impl {
        CommandResult_Impl {
            identity: &CommandResult_Impl::VTABLE_IDENTITY,
            interface1_icommandresult: &CommandResult_Impl::VTABLE_INTERFACE1_ICOMMANDRESULT,
            count: ::windows_core::imp::WeakRefCount::new(),
            this: self,
        }
    }
    /// This converts a partially-constructed COM object (in the sense that it contains
    /// application state but does not yet have vtable and reference count constructed)
    /// into a `StaticComObject`. This allows the COM object to be stored in static
    /// (global) variables.
    pub const fn into_static(self) -> ::windows_core::StaticComObject<Self> {
        ::windows_core::StaticComObject::from_outer(self.into_outer())
    }
}
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct CommandResult_Impl {
    identity: &'static ::windows_core::IInspectable_Vtbl,
    interface1_icommandresult: &'static <ICommandResult as ::windows_core::Interface>::Vtable,
    this: CommandResult,
    count: ::windows_core::imp::WeakRefCount,
}
impl ::core::ops::Deref for CommandResult_Impl {
    type Target = CommandResult;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.this
    }
}
impl CommandResult_Impl {
    const VTABLE_IDENTITY: ::windows_core::IInspectable_Vtbl =
        ::windows_core::IInspectable_Vtbl::new::<CommandResult_Impl, ICommandResult, 0>();
    const VTABLE_INTERFACE1_ICOMMANDRESULT: <ICommandResult as ::windows_core::Interface>::Vtable =
        <ICommandResult as ::windows_core::Interface>::Vtable::new::<CommandResult_Impl, -1isize>();
}
#[allow(unsafe_op_in_unsafe_fn)]
impl ::windows_core::IUnknownImpl for CommandResult_Impl {
    type Impl = CommandResult;
    #[inline(always)]
    fn get_impl(&self) -> &Self::Impl {
        &self.this
    }
    #[inline(always)]
    fn get_impl_mut(&mut self) -> &mut Self::Impl {
        &mut self.this
    }
    #[inline(always)]
    fn into_inner(self) -> Self::Impl {
        self.this
    }
    #[inline(always)]
    fn AddRef(&self) -> u32 {
        self.count.add_ref()
    }
    #[inline(always)]
    unsafe fn Release(self_: *mut Self) -> u32 {
        let remaining = (*self_).count.release();
        if remaining == 0 {
            _ = ::windows_core::imp::Box::from_raw(self_);
        }
        remaining
    }
    #[inline(always)]
    fn is_reference_count_one(&self) -> bool {
        self.count.is_one()
    }
    unsafe fn GetTrustLevel(&self, value: *mut i32) -> ::windows_core::HRESULT {
        if value.is_null() {
            return ::windows_core::imp::E_POINTER;
        }
        *value = 0;
        ::windows_core::HRESULT(0)
    }
    fn to_object(&self) -> ::windows_core::ComObject<Self::Impl> {
        self.count.add_ref();
        unsafe {
            ::windows_core::ComObject::from_raw(::core::ptr::NonNull::new_unchecked(
                self as *const Self as *mut Self,
            ))
        }
    }
    unsafe fn QueryInterface(
        &self,
        iid: *const ::windows_core::GUID,
        interface: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT {
        unsafe {
            if iid.is_null() || interface.is_null() {
                return ::windows_core::imp::E_POINTER;
            }
            let iid = *iid;
            let interface_ptr: *const ::core::ffi::c_void = 'found: {
                if iid == <::windows_core::IUnknown as ::windows_core::Interface>::IID
                    || iid == <::windows_core::IInspectable as ::windows_core::Interface>::IID
                    || iid == <::windows_core::imp::IAgileObject as ::windows_core::Interface>::IID
                {
                    break 'found &self.identity as *const _ as *const ::core::ffi::c_void;
                }
                if <ICommandResult as ::windows_core::Interface>::Vtable::matches(&iid) {
                    break 'found &self.interface1_icommandresult as *const _
                        as *const ::core::ffi::c_void;
                }
                #[cfg(windows)]
                if iid == <::windows_core::imp::IMarshal as ::windows_core::Interface>::IID {
                    return ::windows_core::imp::marshaler(self.to_interface(), interface);
                }
                if iid == ::windows_core::DYNAMIC_CAST_IID {
                    (interface as *mut *const dyn core::any::Any)
                        .write(self as &dyn ::core::any::Any as *const dyn ::core::any::Any);
                    return ::windows_core::HRESULT(0);
                }
                let tear_off_ptr = self.count.query(&iid, &self.identity as *const _ as *mut _);
                if !tear_off_ptr.is_null() {
                    *interface = tear_off_ptr;
                    return ::windows_core::HRESULT(0);
                }
                *interface = ::core::ptr::null_mut();
                return ::windows_core::imp::E_NOINTERFACE;
            };
            assert!(!interface_ptr.is_null());
            *interface = interface_ptr as *mut ::core::ffi::c_void;
            self.count.add_ref();
            ::windows_core::HRESULT(0)
        }
    }
}
impl ::windows_core::ComObjectInner for CommandResult {
    type Outer = CommandResult_Impl;
    fn into_object(self) -> ::windows_core::ComObject<Self> {
        let boxed = ::windows_core::imp::Box::<CommandResult_Impl>::new(self.into_outer());
        unsafe {
            let ptr = ::windows_core::imp::Box::into_raw(boxed);
            ::windows_core::ComObject::from_raw(::core::ptr::NonNull::new_unchecked(ptr))
        }
    }
}
impl ::core::convert::From<CommandResult> for ::windows_core::IUnknown {
    #[inline(always)]
    fn from(this: CommandResult) -> Self {
        let com_object = ::windows_core::ComObject::new(this);
        com_object.into_interface()
    }
}
impl ::core::convert::From<CommandResult> for ::windows_core::IInspectable {
    #[inline(always)]
    fn from(this: CommandResult) -> Self {
        let com_object = ::windows_core::ComObject::new(this);
        com_object.into_interface()
    }
}
impl ::core::convert::From<CommandResult> for ICommandResult {
    #[inline(always)]
    fn from(this: CommandResult) -> Self {
        let com_object = ::windows_core::ComObject::new(this);
        com_object.into_interface()
    }
}
impl ::windows_core::ComObjectInterface<::windows_core::IUnknown> for CommandResult_Impl {
    #[inline(always)]
    fn as_interface_ref(&self) -> ::windows_core::InterfaceRef<'_, ::windows_core::IUnknown> {
        unsafe {
            let interface_ptr = &self.identity;
            ::core::mem::transmute(interface_ptr)
        }
    }
}
impl ::windows_core::ComObjectInterface<::windows_core::IInspectable> for CommandResult_Impl {
    #[inline(always)]
    fn as_interface_ref(&self) -> ::windows_core::InterfaceRef<'_, ::windows_core::IInspectable> {
        unsafe {
            let interface_ptr = &self.identity;
            ::core::mem::transmute(interface_ptr)
        }
    }
}
#[allow(clippy::needless_lifetimes)]
impl ::windows_core::ComObjectInterface<ICommandResult> for CommandResult_Impl {
    #[inline(always)]
    fn as_interface_ref(&self) -> ::windows_core::InterfaceRef<'_, ICommandResult> {
        unsafe { ::core::mem::transmute(&self.interface1_icommandresult) }
    }
}
impl ::windows_core::AsImpl<CommandResult> for ICommandResult {
    #[inline(always)]
    unsafe fn as_impl_ptr(&self) -> ::core::ptr::NonNull<CommandResult> {
        unsafe {
            let this = ::windows_core::Interface::as_raw(self);
            let this =
                (this as *mut *mut ::core::ffi::c_void).sub(1) as *mut CommandResult_Impl;
            ::core::ptr::NonNull::new_unchecked(
                &raw const ((*this).this) as *mut CommandResult,
            )
        }
    }
}

// endregion: implement ICommandResult for CommandResult
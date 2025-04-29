use crate::bindings::*;
use std::mem::ManuallyDrop;
use std::ops::{Deref, DerefMut};
use std::sync::{RwLock, RwLockWriteGuard};
use windows::Foundation::TypedEventHandler;
use windows::Win32::Foundation::ERROR_LOCK_VIOLATION;
use windows::core::{Result, implement, Event, IInspectable};

pub struct NotifyLock<T> {
    lock: RwLock<T>,
}

pub use std::sync::RwLockReadGuard as NotifyLockReadGuard;

pub struct NotifyLockWriteGuard<'a, T, N>
where
    N: Fn() + 'a,
{
    guard: ManuallyDrop<RwLockWriteGuard<'a, T>>,
    notify: N,
}

impl<'a, T, N> Deref for NotifyLockWriteGuard<'a, T, N>
where
    N: Fn() + 'a,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.guard
    }
}

impl<'a, T, N> DerefMut for NotifyLockWriteGuard<'a, T, N>
where
    N: Fn() + 'a,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.guard
    }
}

impl<'a, T, N> Drop for NotifyLockWriteGuard<'a, T, N>
where
    N: Fn() + 'a,
{
    fn drop(&mut self) {
        unsafe {
            // SAFETY: We are already dropping ourself.
            ManuallyDrop::drop(&mut self.guard);
        }
        (self.notify)();
    }
}

impl<T> NotifyLock<T> {
    pub fn new(value: T) -> Self {
        NotifyLock {
            lock: RwLock::new(value),
        }
    }

    pub fn read(&self) -> Result<NotifyLockReadGuard<'_, T>> {
        self.lock.read().map_err(|_| ERROR_LOCK_VIOLATION.into())
    }

    pub fn write<'a, N>(&'a self, notify: N) -> Result<NotifyLockWriteGuard<'a, T, N>>
    where
        N: Fn() + 'a,
    {
        self.lock
            .write()
            .map(|guard| NotifyLockWriteGuard { guard: ManuallyDrop::new(guard) , notify })
            .map_err(|_| ERROR_LOCK_VIOLATION.into())
    }
}

pub type PropChangedEventHandler = Event<TypedEventHandler<IInspectable, IPropChangedEventArgs>>;

#[implement(IPropChangedEventArgs)]
pub struct PropChangedEventArgs(pub windows::core::HSTRING);

impl IPropChangedEventArgs_Impl for PropChangedEventArgs_Impl {
    fn PropertyName(&self) -> windows_core::Result<windows_core::HSTRING> {
        Ok(self.0.clone())
    }
}

impl From<windows::core::HSTRING> for PropChangedEventArgs {
    fn from(value: windows::core::HSTRING) -> Self {
        PropChangedEventArgs(value)
    }
}

pub type ItemsChangedEventHandler = Event<TypedEventHandler<IInspectable, IItemsChangedEventArgs>>;

#[implement(IItemsChangedEventArgs)]
pub struct ItemsChangedEventArgs(pub i32);

impl IItemsChangedEventArgs_Impl for ItemsChangedEventArgs_Impl{
    fn TotalItems(&self) -> windows_core::Result<i32> {
        Ok(self.0)        
    }
}

impl From<i32> for ItemsChangedEventArgs {
    fn from(value: i32) -> Self {
        ItemsChangedEventArgs(value)
    }
}
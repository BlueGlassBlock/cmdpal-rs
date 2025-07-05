//! `NotifyLock` struct and event handling utilities
use crate::bindings::*;
use std::mem::ManuallyDrop;
use std::ops::{Deref, DerefMut};
use std::sync::{RwLock, RwLockWriteGuard};
use windows::Foundation::TypedEventHandler;
use windows::Win32::Foundation::ERROR_LOCK_VIOLATION;
use windows::core::{Event, IInspectable, Result, implement};

/// `NotifyLock` struct is a wrapper around [`RwLock`] that allows for notification callbacks.
/// When exposing the interface, `NotifyLock` references shouldn't be returned directly.
/// Instead, return [`NotifyLockReadGuard`] or [`NotifyLockWriteGuard`]
/// to ensure that intended notification function is called.
///
/// Useful for implementing COM interfaces like `INotifyPropChanged`.
pub struct NotifyLock<T> {
    lock: RwLock<T>,
}

/// Alias of [`std::sync::RwLockReadGuard`]
pub use std::sync::RwLockReadGuard as NotifyLockReadGuard;

/// A thin wrapper around [`RwLockWriteGuard`] that allows for notification callbacks
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
    /// Creates a new `NotifyLock` with the given value.
    pub fn new(value: T) -> Self {
        NotifyLock {
            lock: RwLock::new(value),
        }
    }

    /// Get a read guard for the lock.
    pub fn read(&self) -> Result<NotifyLockReadGuard<'_, T>> {
        self.lock.read().map_err(|_| ERROR_LOCK_VIOLATION.into())
    }

    /// Get a mutable write guard for the lock, with a notification callback.
    /// The callback will be called when the guard is dropped.
    pub fn write<'a, N>(&'a self, notify: N) -> Result<NotifyLockWriteGuard<'a, T, N>>
    where
        N: Fn() + 'a,
    {
        self.lock
            .write()
            .map(|guard| NotifyLockWriteGuard {
                guard: ManuallyDrop::new(guard),
                notify,
            })
            .map_err(|_| ERROR_LOCK_VIOLATION.into())
    }
}

pub type PropChangedEventHandler = Event<TypedEventHandler<IInspectable, IPropChangedEventArgs>>;

/// `PropChangedEventArgs` is used to notify about property changes in COM interfaces.
/// It implements the `IPropChangedEventArgs` interface and contains the name of the property that changed.
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

/// `ItemsChangedEventArgs` is used to notify about changes in a collection of items.
/// It implements the `IItemsChangedEventArgs` interface and contains the total number of items (-1 if unknown).
#[implement(IItemsChangedEventArgs)]
pub struct ItemsChangedEventArgs(pub i32);

impl IItemsChangedEventArgs_Impl for ItemsChangedEventArgs_Impl {
    fn TotalItems(&self) -> windows_core::Result<i32> {
        Ok(self.0)
    }
}

impl From<i32> for ItemsChangedEventArgs {
    fn from(value: i32) -> Self {
        ItemsChangedEventArgs(value)
    }
}

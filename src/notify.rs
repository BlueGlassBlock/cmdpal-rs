//! [`NotifyLock`] struct and event handling utilities
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
pub struct NotifyLockWriteGuard<'a, T, P = ()> {
    guard: ManuallyDrop<RwLockWriteGuard<'a, T>>,
    peeker: Box<dyn Fn(&T) -> P + 'a>,
    notifier: Box<dyn Fn(P) + 'a>,
}

impl<'a, T, P> Deref for NotifyLockWriteGuard<'a, T, P> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.guard
    }
}

impl<'a, T, P> DerefMut for NotifyLockWriteGuard<'a, T, P> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.guard
    }
}

impl<'a, T, P> Drop for NotifyLockWriteGuard<'a, T, P> {
    fn drop(&mut self) {
        let peek_data = (self.peeker)(&self.guard);
        unsafe {
            // SAFETY: We are already dropping ourself.
            ManuallyDrop::drop(&mut self.guard);
        }
        (self.notifier)(peek_data);
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
    pub fn write<'a, F>(&'a self, notifier: F) -> Result<NotifyLockWriteGuard<'a, T>>
    where
        F: Fn() + 'a,
    {
        self.lock
            .write()
            .map(|guard| NotifyLockWriteGuard {
                guard: ManuallyDrop::new(guard),
                peeker: Box::new(|_| ()),
                notifier: Box::new(move |_| notifier()),
            })
            .map_err(|_| ERROR_LOCK_VIOLATION.into())
    }

    pub fn write_with_peek<'a, PF, NF, P>(
        &'a self,
        peeker: PF,
        notifier: NF,
    ) -> Result<NotifyLockWriteGuard<'a, T, P>>
    where
        PF: Fn(&T) -> P + 'a,
        NF: Fn(P) + 'a,
    {
        self.lock
            .write()
            .map(|guard| NotifyLockWriteGuard {
                guard: ManuallyDrop::new(guard),
                peeker: Box::new(peeker),
                notifier: Box::new(notifier),
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

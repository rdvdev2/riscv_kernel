use core::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
    sync::atomic::AtomicBool,
};

pub struct SpinLock<T> {
    value: UnsafeCell<T>,
    key: AtomicBool,
}

// Safety: This type implements a mutex on the value
unsafe impl<T: Send> Sync for SpinLock<T> {}

impl<T> SpinLock<T> {
    pub const fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
            key: AtomicBool::new(true),
        }
    }

    pub fn acquire<'a>(&'a self) -> SpinLockHandle<'a, T> {
        while !self.key.swap(false, core::sync::atomic::Ordering::SeqCst) {}
        SpinLockHandle { parent: self }
    }

    // Safety: Caller must ensure no other instance of SpinLockHandle is in use.
    // Typical use case is resource acquisition on a panic handler.
    pub unsafe fn acquire_unchecked<'a>(&'a self) -> SpinLockHandle<'a, T> {
        SpinLockHandle { parent: self }
    }
}

pub struct SpinLockHandle<'a, T> {
    parent: &'a SpinLock<T>,
}

impl<'a, T> Deref for SpinLockHandle<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // Safety: Only one SpinLockHandle can exist at once, no mutable
        // aliasing occurs thank to the shared reference of self
        unsafe { &*self.parent.value.get() }
    }
}

impl<'a, T> DerefMut for SpinLockHandle<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // Safety: Only one SpinLockHandle can exist at once, no mutable
        // aliasing occurs thank to the exclusive reference of self
        unsafe { &mut *self.parent.value.get() }
    }
}

impl<'a, T> Drop for SpinLockHandle<'a, T> {
    fn drop(&mut self) {
        self.parent
            .key
            .store(true, core::sync::atomic::Ordering::SeqCst);
    }
}

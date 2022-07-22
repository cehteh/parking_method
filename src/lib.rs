#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]
//! # The Locking methods
//!
//! There are plenty flavors on how a lock can be obtained. The normal blocking way, trying to
//! obtain a lock, possibly with timeouts, allow a thread to lock a single RwLock multiple
//! times. These are (zero-cost) abstracted here.

/// reexport for convenience
pub use std::time::{Duration, Instant};
pub use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};

/// Marker for blocking locks,
/// waits until the lock becomes available.
pub struct Blocking;

/// Marker for trying to obtain a lock in a fallible way.
pub struct TryLock;

/// Marker for recursive locking. Allows to obtain a read-lock multiple times by a single
/// thread.
///
/// # Panics
/// There are no try_write_recursive forms in parking_lot. Trying to call this will panic.
pub struct Recursive<T>(pub T);

mod rwlock_method;
pub use rwlock_method::*;

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn smoke() {
        let rwlock = RwLock::new(String::from("test"));
        assert_eq!(*RwLockMethod::read(&Blocking, &rwlock).unwrap(), "test");
    }

    #[test]
    fn trylocks() {
        let rwlock = RwLock::new(String::from("test"));

        assert_eq!(*RwLockMethod::read(&TryLock, &rwlock).unwrap(), "test");
        assert_eq!(
            *RwLockMethod::read(&Duration::from_millis(100), &rwlock).unwrap(),
            "test"
        );
        assert_eq!(
            *RwLockMethod::read(&(Instant::now() + Duration::from_millis(100)), &rwlock).unwrap(),
            "test"
        );
    }

    #[test]
    fn recursivelocks() {
        let rwlock = RwLock::new(String::from("test"));

        let guard = RwLockMethod::read(&Blocking, &rwlock).unwrap();

        assert_eq!(
            *RwLockMethod::read(&Recursive(Blocking), &rwlock).unwrap(),
            "test"
        );
        assert_eq!(
            *RwLockMethod::read(&Recursive(TryLock), &rwlock).unwrap(),
            "test"
        );
        assert_eq!(
            *RwLockMethod::read(&Recursive(Duration::from_millis(100)), &rwlock).unwrap(),
            "test"
        );
        assert_eq!(
            *RwLockMethod::read(
                &Recursive(Instant::now() + Duration::from_millis(100)),
                &rwlock
            )
            .unwrap(),
            "test"
        );

        drop(guard);
    }

    #[test]
    #[should_panic]
    fn recursive_try_write_panics() {
        let rwlock = RwLock::new(String::from("test"));

        let guard = RwLockMethod::read(&Blocking, &rwlock).unwrap();

        let _ = RwLockMethod::write(&Recursive(TryLock), &rwlock).unwrap();

        drop(guard);
    }
}

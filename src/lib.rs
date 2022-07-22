#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]
//! # The Locking methods
//!
//! There are plenty flavors on how a lock can be obtained. The normal blocking way, trying to
//! obtain a lock, possibly with timeouts, allow a thread to lock a single RwLock multiple
//! times. These are (zero-cost) abstracted here.

pub use parking_lot::{
    Mutex, MutexGuard, ReentrantMutex, ReentrantMutexGuard, RwLock, RwLockReadGuard,
    RwLockWriteGuard,
};
/// reexport for convenience
pub use std::time::{Duration, Instant};

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

mod mutex_method;
pub use mutex_method::*;

mod reentrant_mutex_method;
pub use reentrant_mutex_method::*;

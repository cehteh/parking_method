use crate::*;

/// Trait for implementing read/write flavors on RwLocks.
pub trait RwLockMethod<'a, V> {
    /// Obtain a read lock. Blocking locks are infallible and always return a 'Some()' variant.
    fn read(&self, rwlock: &'a RwLock<V>) -> Option<RwLockReadGuard<'a, V>>;

    /// Obtain a write lock. Blocking locks are infallible and always return a 'Some()' variant.
    fn write(&self, rwlock: &'a RwLock<V>) -> Option<RwLockWriteGuard<'a, V>>;
}

macro_rules! impl_locking_method {
    ($policy:ty, $read:expr, $write:expr) => {
        impl<'a, V> RwLockMethod<'a, V> for $policy {
            #[inline(always)]
            #[allow(unused_variables)]
            fn read(&self, rwlock: &'a RwLock<V>) -> Option<RwLockReadGuard<'a, V>> {
                #[allow(unused_macros)]
                macro_rules! method {
                    () => {
                        self
                    };
                }
                #[allow(unused_macros)]
                macro_rules! lock {
                    () => {
                        rwlock
                    };
                }
                $read
            }

            #[inline(always)]
            #[allow(unused_variables)]
            fn write(&self, rwlock: &'a RwLock<V>) -> Option<RwLockWriteGuard<'a, V>> {
                #[allow(unused_macros)]
                macro_rules! method {
                    () => {
                        self
                    };
                }
                #[allow(unused_macros)]
                macro_rules! lock {
                    () => {
                        rwlock
                    };
                }
                $write
            }
        }
    };
}

impl_locking_method!(Blocking, Some(lock!().read()), Some(lock!().write()));

impl_locking_method!(TryLock, lock!().try_read(), lock!().try_write());

impl_locking_method!(
    Duration,
    lock!().try_read_for(*method!()),
    lock!().try_write_for(*method!())
);

impl_locking_method!(
    Instant,
    lock!().try_read_until(*method!()),
    lock!().try_write_until(*method!())
);

impl_locking_method!(
    Recursive<Blocking>,
    Some(lock!().read_recursive()),
    Some(lock!().write())
);

impl_locking_method!(
    Recursive<TryLock>,
    lock!().try_read_recursive(),
    unimplemented!("Not implemented in parking_lot")
);

impl_locking_method!(
    Recursive<Duration>,
    lock!().try_read_recursive_for(method!().0),
    unimplemented!("Not implemented in parking_lot")
);

impl_locking_method!(
    Recursive<Instant>,
    lock!().try_read_recursive_until(method!().0),
    unimplemented!("Not implemented in parking_lot")
);

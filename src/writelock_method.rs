use crate::*;

/// Trait for implementing write flavors on RwLocks.
pub trait WriteLockMethod<'a, V> {
    /// Obtain a write lock. Blocking locks are infallible and always return a 'Some()' variant.
    fn write(&self, rwlock: &'a RwLock<V>) -> Option<RwLockWriteGuard<'a, V>>;
}

macro_rules! impl_locking_method {
    ($policy:ty, $write:expr) => {
        impl<'a, V> WriteLockMethod<'a, V> for $policy {
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

impl_locking_method!(Blocking, Some(lock!().write()));

impl_locking_method!(TryLock, lock!().try_write());

impl_locking_method!(Duration, lock!().try_write_for(*method!()));

impl_locking_method!(Instant, lock!().try_write_until(*method!()));

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn smoke() {
        let rwlock = RwLock::new(String::from("test"));
        assert_eq!(*WriteLockMethod::write(&Blocking, &rwlock).unwrap(), "test");
    }

    #[test]
    fn trylocks() {
        let rwlock = RwLock::new(String::from("test"));

        assert_eq!(*WriteLockMethod::write(&TryLock, &rwlock).unwrap(), "test");
        assert_eq!(
            *WriteLockMethod::write(&Duration::from_millis(100), &rwlock).unwrap(),
            "test"
        );
        assert_eq!(
            *WriteLockMethod::write(&(Instant::now() + Duration::from_millis(100)), &rwlock)
                .unwrap(),
            "test"
        );
    }
}

use crate::*;

/// Trait for implementing read flavors on RwLocks.
pub trait ReadLockMethod {
    /// Obtain a read lock. Blocking locks are infallible and always return a 'Some()' variant.
    fn read<'a, V>(&self, rwlock: &'a RwLock<V>) -> Option<RwLockReadGuard<'a, V>>;
}

macro_rules! impl_locking_method {
    ($policy:ty, $read:expr $(, $as_write:expr)?) => {
        impl ReadLockMethod for $policy {
            #[inline(always)]
            #[allow(unused_variables)]
            fn read<'a, V>(&self, rwlock: &'a RwLock<V>) -> Option<RwLockReadGuard<'a, V>> {
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
        }
    };
}

impl_locking_method!(Blocking, Some(lock!().read()));

impl_locking_method!(TryLock, lock!().try_read());

impl_locking_method!(Duration, lock!().try_read_for(*method!()));

impl_locking_method!(Instant, lock!().try_read_until(*method!()));

impl_locking_method!(Recursive<Blocking>, Some(lock!().read_recursive()));

impl_locking_method!(Recursive<TryLock>, lock!().try_read_recursive());

impl_locking_method!(
    Recursive<Duration>,
    lock!().try_read_recursive_for(method!().0)
);

impl_locking_method!(
    Recursive<Instant>,
    lock!().try_read_recursive_until(method!().0)
);

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn smoke() {
        let rwlock = RwLock::new(String::from("test"));
        assert_eq!(*ReadLockMethod::read(&Blocking, &rwlock).unwrap(), "test");
    }

    #[test]
    fn trylocks() {
        let rwlock = RwLock::new(String::from("test"));

        assert_eq!(*ReadLockMethod::read(&TryLock, &rwlock).unwrap(), "test");
        assert_eq!(
            *ReadLockMethod::read(&Duration::from_millis(100), &rwlock).unwrap(),
            "test"
        );
        assert_eq!(
            *ReadLockMethod::read(&(Instant::now() + Duration::from_millis(100)), &rwlock).unwrap(),
            "test"
        );
    }

    #[test]
    fn recursivelocks() {
        let rwlock = RwLock::new(String::from("test"));

        let guard = ReadLockMethod::read(&Blocking, &rwlock).unwrap();

        assert_eq!(
            *ReadLockMethod::read(&Recursive(Blocking), &rwlock).unwrap(),
            "test"
        );
        assert_eq!(
            *ReadLockMethod::read(&Recursive(TryLock), &rwlock).unwrap(),
            "test"
        );
        assert_eq!(
            *ReadLockMethod::read(&Recursive(Duration::from_millis(100)), &rwlock).unwrap(),
            "test"
        );
        assert_eq!(
            *ReadLockMethod::read(
                &Recursive(Instant::now() + Duration::from_millis(100)),
                &rwlock
            )
            .unwrap(),
            "test"
        );

        drop(guard);
    }
}

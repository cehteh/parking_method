use crate::*;

/// Trait for implementing read/write flavors on Mutex.
/// Note that there are no Recursive locks in Mutex, use ReentrantMutex for that.
pub trait MutexMethod<'a, V> {
    /// Obtain a lock on a mutex. Blocking locks are infallible and always return a 'Some()' variant.
    fn lock(&self, mutex: &'a Mutex<V>) -> Option<MutexGuard<'a, V>>;
}

macro_rules! impl_locking_method {
    ($policy:ty, $lock:expr) => {
        impl<'a, V> MutexMethod<'a, V> for $policy {
            #[inline(always)]
            #[allow(unused_variables)]
            fn lock(&self, mutex: &'a Mutex<V>) -> Option<MutexGuard<'a, V>> {
                #[allow(unused_macros)]
                macro_rules! method {
                    () => {
                        self
                    };
                }
                #[allow(unused_macros)]
                macro_rules! lock {
                    () => {
                        mutex
                    };
                }
                $lock
            }
        }
    };
}

impl_locking_method!(Blocking, Some(lock!().lock()));

impl_locking_method!(TryLock, lock!().try_lock());

impl_locking_method!(Duration, lock!().try_lock_for(*method!()));

impl_locking_method!(Instant, lock!().try_lock_until(*method!()));

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn smoke() {
        let mutex = Mutex::new(String::from("test"));
        assert_eq!(*MutexMethod::lock(&Blocking, &mutex).unwrap(), "test");
    }

    #[test]
    fn trylocks() {
        let mutex = Mutex::new(String::from("test"));

        assert_eq!(*MutexMethod::lock(&TryLock, &mutex).unwrap(), "test");
        assert_eq!(
            *MutexMethod::lock(&Duration::from_millis(100), &mutex).unwrap(),
            "test"
        );
        assert_eq!(
            *MutexMethod::lock(&(Instant::now() + Duration::from_millis(100)), &mutex).unwrap(),
            "test"
        );
    }
}

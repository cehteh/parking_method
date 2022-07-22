use crate::*;

/// Trait for implementing read/write flavors on ReentantMutex.
pub trait ReentrantMutexMethod<'a, V> {
    /// Obtain a lock on a reentrant mutex. Blocking locks are infallible and always return a
    /// 'Some()' variant.
    fn lock(&self, mutex: &'a ReentrantMutex<V>) -> Option<ReentrantMutexGuard<'a, V>>;
}

macro_rules! impl_locking_method {
    ($policy:ty, $lock:expr) => {
        impl<'a, V> ReentrantMutexMethod<'a, V> for $policy {
            #[inline(always)]
            #[allow(unused_variables)]
            fn lock(&self, mutex: &'a ReentrantMutex<V>) -> Option<ReentrantMutexGuard<'a, V>> {
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

impl_locking_method!(Recursive<Blocking>, Some(lock!().lock()));

impl_locking_method!(Recursive<TryLock>, lock!().try_lock());

impl_locking_method!(Recursive<Duration>, lock!().try_lock_for(method!().0));

impl_locking_method!(Recursive<Instant>, lock!().try_lock_until(method!().0));

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn smoke() {
        let mutex = ReentrantMutex::new(String::from("test"));
        assert_eq!(
            *ReentrantMutexMethod::lock(&Blocking, &mutex).unwrap(),
            "test"
        );
    }

    #[test]
    fn trylocks() {
        let mutex = ReentrantMutex::new(String::from("test"));

        assert_eq!(
            *ReentrantMutexMethod::lock(&TryLock, &mutex).unwrap(),
            "test"
        );
        assert_eq!(
            *ReentrantMutexMethod::lock(&Duration::from_millis(100), &mutex).unwrap(),
            "test"
        );
        assert_eq!(
            *ReentrantMutexMethod::lock(&(Instant::now() + Duration::from_millis(100)), &mutex)
                .unwrap(),
            "test"
        );
    }

    #[test]
    fn recursivelocks() {
        let recmutex = ReentrantMutex::new(String::from("test"));

        let guard = ReentrantMutexMethod::lock(&Blocking, &recmutex).unwrap();

        assert_eq!(
            *ReentrantMutexMethod::lock(&Recursive(Blocking), &recmutex).unwrap(),
            "test"
        );
        assert_eq!(
            *ReentrantMutexMethod::lock(&Recursive(TryLock), &recmutex).unwrap(),
            "test"
        );
        assert_eq!(
            *ReentrantMutexMethod::lock(&Recursive(Duration::from_millis(100)), &recmutex).unwrap(),
            "test"
        );
        assert_eq!(
            *ReentrantMutexMethod::lock(
                &Recursive(Instant::now() + Duration::from_millis(100)),
                &recmutex
            )
            .unwrap(),
            "test"
        );

        drop(guard);
    }
}

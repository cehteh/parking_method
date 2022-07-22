# Zero cost abstraction for Locking Methods

'parking_lot' has a lot variants for locking, there is the normal blocking lock, a try_lock,
two forms of timed try locks and all of these are available as recursive variant as well.

This leads to a some explosion on the api, dispatching to these variants in generic code
becomes pretty bloated. This crate abstracts all the different locking variants into a single
trait with a uniform API that takes policy objects for the dispatch on underlying locking
methods

## Example
```
// reexports parts of parking_lot as well
use parking_method::*;

let rwlock = RwLock::new(String::from("test"));

// Note the 2 following syntax forms are equivalent
assert_eq!(*Blocking.read(&rwlock).unwrap(), "test");
assert_eq!(*ReadLockMethod::read(&Blocking, &rwlock).unwrap(), "test");

assert_eq!(*TryLock.read(&rwlock).unwrap(), "test");

let timeout = Duration::from_millis(100);

assert_eq!(
    *ReadLockMethod::read(&timeout, &rwlock).unwrap(),
    "test"
);

assert_eq!(
    *ReadLockMethod::read(
        &Recursive(Instant::now() + Duration::from_millis(100)),
        &rwlock).unwrap(),
    "test"
);
```


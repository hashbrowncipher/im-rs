// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub(crate) use self::lock::Lock;

#[cfg(all(threadsafe, feature = "std"))]
mod lock {
    use alloc::sync::Arc;
    use std::sync::{Mutex, MutexGuard};

    /// Thread safe lock: just wraps a `Mutex`.
    pub(crate) struct Lock<A> {
        lock: Arc<Mutex<A>>,
    }

    impl<A> Lock<A> {
        pub(crate) fn new(value: A) -> Self {
            Lock {
                lock: Arc::new(Mutex::new(value)),
            }
        }

        #[inline]
        pub(crate) fn lock(&mut self) -> Option<MutexGuard<'_, A>> {
            self.lock.lock().ok()
        }
    }

    impl<A> Clone for Lock<A> {
        fn clone(&self) -> Self {
            Lock {
                lock: self.lock.clone(),
            }
        }
    }
}

// Without std (no Mutex available) or without threadsafe, use RefCell.
// This is sound because FocusMut::split_at (the only operation that clones
// a Lock to create shared mutable access) is gated behind the std feature.
// All other Lock usage is single-holder.
#[cfg(any(not(threadsafe), not(feature = "std")))]
mod lock {
    use core::cell::{RefCell, RefMut};
    use alloc::rc::Rc;

    /// Single threaded lock: a `RefCell` so we should safely panic if somehow
    /// trying to access the stored data twice from the same thread.
    pub(crate) struct Lock<A> {
        lock: Rc<RefCell<A>>,
    }

    impl<A> Lock<A> {
        pub(crate) fn new(value: A) -> Self {
            Lock {
                lock: Rc::new(RefCell::new(value)),
            }
        }

        #[inline]
        pub(crate) fn lock(&mut self) -> Option<RefMut<'_, A>> {
            self.lock.try_borrow_mut().ok()
        }
    }

    impl<A> Clone for Lock<A> {
        fn clone(&self) -> Self {
            Lock {
                lock: self.lock.clone(),
            }
        }
    }
}

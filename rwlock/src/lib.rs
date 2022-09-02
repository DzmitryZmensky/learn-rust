use std::sync::{Condvar, Mutex};

pub struct RWLock {
    cvar: Condvar,
    mtx: Mutex<RWLockState>,
}

struct RWLockState {
    pending_readers: u32,
    pending_writers: u32,
    active_readers: u32,
    active_writer: bool,
}

pub struct RWLockAutoReleaser <'a> {
    rwlock: &'a RWLock,
    is_read_lock: bool,
}

impl RWLockAutoReleaser<'_> {
    pub fn new(rwlock: &RWLock, is_read_lock: bool) -> RWLockAutoReleaser {
        RWLockAutoReleaser { 
            rwlock, 
            is_read_lock,
         }
    }    
}

impl Drop for RWLockAutoReleaser<'_> {
    fn drop(&mut self) {
        if self.is_read_lock { 
            self.rwlock.release_read_lock(); 
        } else { 
            self.rwlock.release_write_lock(); 
        }
    }
}

impl RWLockState {
    pub fn new() -> Self {
        Self {
            pending_readers: 0,
            pending_writers: 0,
            active_readers: 0,
            active_writer: false,
        }
    }
}

impl RWLock {
    pub fn new() -> Self {
        Self { 
            cvar: Condvar::new(), 
            mtx: Mutex::<RWLockState>::new(RWLockState::new())
        }
    }
    pub fn acquire_write_lock(&self) -> RWLockAutoReleaser {
        let mut guard = self.mtx.lock().unwrap();
        guard.pending_writers += 1;
        while guard.active_writer || guard.active_readers > 0 {
            guard = self.cvar.wait(guard).unwrap();
        }
        guard.pending_writers -= 1;
        guard.active_writer = true;
        RWLockAutoReleaser::new(self, false)
    }

    fn release_write_lock(&self) {
        let mut guard = self.mtx.lock().unwrap();
        guard.active_writer = false;
        self.cvar.notify_all();
    }

    pub fn acquire_read_lock(&self) -> RWLockAutoReleaser {
        let mut guard = self.mtx.lock().unwrap();
        guard.pending_readers += 1;
        while guard.active_writer || guard.pending_writers > 0  {
            guard = self.cvar.wait(guard).unwrap();
        }
        guard.pending_readers -= 1;
        guard.active_readers += 1;

        RWLockAutoReleaser::new(self, true)
    }

    fn release_read_lock(&self) {
        let mut guard = self.mtx.lock().unwrap();
        guard.active_readers -=  1;
        self.cvar.notify_all();
    }
}

#[cfg(test)]
mod tests {
    use std::{thread, time, sync::Arc};
    use crate::RWLock;

    #[test]
    fn acquire_write_lock_during_read_lock() {
        let rwlock = Arc::new(RWLock::new());
        let rwlock_clone = rwlock.clone();
        let join_handle;
        {
            let _r_guard = rwlock.acquire_read_lock();
            join_handle = thread::spawn(move || {
                let _w_guard = rwlock_clone.acquire_write_lock();
            });
            thread::sleep(time::Duration::from_millis(200));
        }
        join_handle.join().unwrap();
    }

    #[test]
    fn acquire_another_read_lock_during_read_lock() {
        let rwlock = Arc::new(RWLock::new());
        let rwlock_clone = rwlock.clone();
        let _r_guard1 = rwlock.acquire_read_lock();
        let join_handle = thread::spawn(move || {
            let _r_guard2 = rwlock_clone.acquire_read_lock();
        });
        join_handle.join().unwrap();
    }

    #[test]
    fn acquire_release_write_lock() {
        let rwlock = RWLock::new();
        let _w_guard = rwlock.acquire_write_lock();
    }

    #[test]
    fn acquire_release_read_lock() {
        let rwlock = RWLock::new();
        let _r_guard = rwlock.acquire_read_lock();
    }
}

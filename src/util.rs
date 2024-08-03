use std::{
    future::Future,
    sync::atomic::{self, AtomicUsize},
    thread::{self},
    time::Duration,
};

use atomic::Ordering::SeqCst;
use bytes::Bytes;
use flate2::bufread::GzDecoder;
use tar::Archive;
use tokio::task::JoinHandle;

pub fn extract_tarball(bytes: Bytes, dest: String) {
    let bytes = &bytes.to_vec()[..];
    let gz = GzDecoder::new(bytes);
    let mut archive = Archive::new(gz);

    archive.unpack()
}

pub static ACTIVE_TASKS: AtomicUsize = AtomicUsize::new(0);

pub struct TaskAllocator;

impl TaskAllocator {
    pub fn add_task<T>(future: T) -> JoinHandle<T::Output>
    where
        T: Future + Send + 'static,
        T::Output: Send + 'static,
    {
        tokio::spawn(async move {
            Self::increment_tasks();
            let future_result = future.await;
            Self::decrement_tasks();

            future_result
        })
    }

    pub fn add_blocking<F, R>(f: F) -> JoinHandle<R>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        tokio::task::spawn_blocking(move || {
            Self::increment_tasks();
            let task_result = f();
            Self::decrement_tasks();

            task_result
        })
    }

    pub fn block_until_done() {
        while Self::task_count() != 0 {
            thread::sleep(Duration::from_millis(1));
        }
    }

    fn increment_tasks() {
        ACTIVE_TASKS.fetch_add(1, SeqCst);
    }

    fn decrement_tasks() {
        ACTIVE_TASKS.fetch_sub(1, SeqCst);
    }

    fn task_count() -> usize {
        ACTIVE_TASKS.load(SeqCst)
    }
}

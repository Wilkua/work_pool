use std::thread::{self, JoinHandle};

mod work_queue;
use work_queue::WorkQueue;

#[derive(Clone)]
enum Work<T> {
    Job(T),
    Quit,
}

pub struct WorkPool<T> {
    queue: WorkQueue<Work<T>>,
    threads: Vec<JoinHandle<()>>,
}

impl<T: Clone + Send + std::fmt::Debug> WorkPool<T> {
    /// Create a new WorkPool
    pub fn new(num_threads: usize) -> Result<WorkPool<T>, ()> {
        let queue = WorkQueue::new(64);

        let num_threads = if num_threads == 0 {
            usize::from(std::thread::available_parallelism().unwrap())
        } else {
            num_threads
        };

        let threads = Vec::with_capacity(num_threads);
        Ok(WorkPool {
            queue,
            threads,
        })
    }

    /// Send a job to the pool
    pub fn dispatch(&mut self, work: T) {
        self.queue.dispatch(Work::Job(work));
    }

    /// Send a list of jobs to the pool
    pub fn dispatch_many(&mut self, work: Vec<T>) {
        let work = work.iter()
            .map(|w| { Work::Job(w.to_owned()) })
            .collect();
        self.queue.dispatch_many(work);
    }

    /// Setup the job executor function and start threads
    pub fn set_executor_and_start<F>(&mut self, _executor: F)
    where
        F: FnOnce(T) + Send + 'static,
        T: Send + 'static
    {
        for _ in 0..self.threads.capacity() {
            let mut queue = self.queue.clone();
            self.threads.push(thread::spawn(move || {
                // Implement work stealing queue
                // steal work -> Pass to executor
                // Executor should accept parameter of type T
                loop {
                    match queue.find_work() {
                        Work::Job(w) => println!("thrad {:?} got {:?}", std::thread::current().id(), w),
                        Work::Quit => break,
                    }
                }
            }));
        }
    }
}

impl<T> Drop for WorkPool<T> {
    /// When dropping this struct, threads will be detached
    fn drop(&mut self) {
        for t in self.threads.iter_mut() {
            self.queue.dispatch(Work::Quit);
            drop(t)
        }
    }
}

/* #[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_pool() {
        let pool: WorkPool<u8> = WorkPool::new(8).unwrap();
        assert_eq!(pool.threads.capacity(), 8);
    }
} */

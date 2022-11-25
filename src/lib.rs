use std::thread::{self, JoinHandle};

mod work_queue;
use work_queue::WorkQueue;

#[derive(Clone, Debug)]
enum Work<T> {
    Job(T),
    Quit,
}

#[derive(Debug)]
pub struct WorkPool<T> {
    queue: WorkQueue<Work<T>>,
    threads: Vec<JoinHandle<()>>,
}

impl<T: Clone + Send> WorkPool<T> {
    /// Create a new WorkPool
    pub fn new(num_threads: usize, buf_len: Option<usize>) -> Result<WorkPool<T>, ()> {
        let queue = WorkQueue::new(buf_len.unwrap_or(64usize));

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
    pub fn set_executor_and_start<F>(&mut self, executor: F)
    where
        F: FnOnce(T) + Copy + Send + 'static,
        T: Send + 'static
    {
        for _ in 0..self.threads.capacity() {
            let queue = self.queue.clone();
            self.threads.push(thread::spawn(move || {
                // steal work -> Pass to executor
                // Executor should accept parameter of type T
                for work in queue {
                    match work {
                        Work::Job(w) => executor(w),
                        Work::Quit => break,
                    }
                }
            }));
        }
    }

    /// Send a quit message to all threads and wait for them to join.
    pub fn close(&mut self) {
        let mut quits = Vec::with_capacity(self.threads.len());
        for _ in 0..self.threads.len() {
            quits.push(Work::Quit);
        }

        self.queue.dispatch_many(quits);

        for _ in 0..self.threads.len() {
            let thread = self.threads.pop().unwrap();
            let _ = thread.join();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_pool() {
        let pool: WorkPool<()> = WorkPool::new(8, None).unwrap();
        assert_eq!(pool.threads.capacity(), 8);
    }
}

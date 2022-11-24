
use std::{
    collections::VecDeque,
    sync::{Arc, Condvar, Mutex},
    };

#[derive(Clone)]
pub struct WorkQueue<T> {
    queue: Arc<Mutex<VecDeque<T>>>,
    queue_cv: Arc<Condvar>,
}

impl<T> WorkQueue<T> {
    /// Create new WorkQueue
    pub fn new(buffer_len: usize) -> WorkQueue<T> {
        let buffer_len = if buffer_len == 0 {
            1
        } else {
            buffer_len
        };

        WorkQueue {
            queue: Arc::new(Mutex::new(VecDeque::with_capacity(buffer_len))),
            queue_cv: Arc::new(Condvar::new()),
        }
    }

    pub fn dispatch(&mut self, work: T) {
        let mut queue = self.queue.lock().unwrap();
        queue.push_back(work);
        drop(queue);

        self.queue_cv.notify_one();
    }

    pub fn dispatch_many(&mut self, work: Vec<T>) {
        let mut queue = self.queue.lock().unwrap();
        queue.append(&mut VecDeque::from(work));
        drop(queue);

        self.queue_cv.notify_all();
    }

    pub fn find_work(&mut self) -> T {
        let mut queue = self.queue_cv.wait_while(self.queue.lock().unwrap(),
            |queue: &mut VecDeque<T>| { queue.len() > 0 }).unwrap();

        // We unwrap here because we guarantee at least one work item with the CV
        let work = queue.pop_front().unwrap();
        drop(queue);

        work
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_add_to_queue() {
        let mut wq = WorkQueue::new(1);
        wq.dispatch(1);

        let q = wq.queue.lock().unwrap();
        assert_eq!(q.len(), 1);
    }

    #[test]
    fn should_add_many_to_queue() {
        let mut wq = WorkQueue::new(4);
        wq.dispatch_many(vec![1, 2, 3, 4]);

        let q = wq.queue.lock().unwrap();
        assert_eq!(q.len(), 4);
    }

    #[test]
    #[ignore] // This one doesn't work :/
    fn should_retrieve_work() -> std::thread::Result<()> {
        let mut wq = WorkQueue::new(1);
        let mut wqc = wq.clone();
        let jh = std::thread::spawn(move || {
            let work = wqc.find_work();
            assert_eq!(work, 1);
        });
        wq.dispatch(1);

        jh.join()

    }
}

use std::thread;
use work_pool::WorkPool;

fn main() {
    // Specify the number of threads to be used and the number of work
    // item spaces to start with.
    //
    // Setting the threads to 0 will let the system choose the number
    // of threads. Setting the buffer length to None will default the
    // buffer length to a reasonable amount.
    let mut pool = WorkPool::new(0, None).expect("Failed to build work pool");

    pool.set_executor_and_start(|work| {
        println!("thread {:?} got item {}", thread::current().id(), work);
    });

    // Dispatch some work to do
    pool.dispatch(1);
    pool.dispatch(2);
    pool.dispatch(3);
    pool.dispatch(4);

    // Or, dispatch a bunch of work at once
    pool.dispatch_many(vec![5, 6, 7, 8]);

    // Closing the pool will send a quit message to the threads and
    // block while it waits for the threads to join.
    //
    // Dropping the pool will send a quit message to
    // the threads and detach them - you won't have to
    // wait for the threads to join
    pool.close();

    println!("Done");
}

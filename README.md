# WorkPool

A super simple implementation of a work queue wrapped by a thread pool.

The goal of this package is to have a quick way of creating a thread
pool with a work queue backing for quick projects.

# Example

```rust
use std::sync::Arc;
use work_pool::WorkPool;

fn main() {
    // Specify number of threads and work queue capacity
    // Work queue capacity is workload based, but a good
    // estimate might be 2 or 4 times the number of threads
    let mut pool = WorkPool::new(8, 64);

    // Set the executor function and star tthe work listener
    pool.set_executor_and_start(|work| {
        // Work is the data sent by `dispatch`
    });

    loop {
        // do something, like get a TcpStream
        let stream = accept_next_connection();
        match stream {
            Some(Ok(s)) => pool.dispatch(Arc::new(s)),
            Some(Err(e)) => panic!(e),
            None => break,
        }
    }

    // Dropping the pool will send a Quit message to all active threads
    and detach them. No joins happen here.
    drop(pool);
}
```


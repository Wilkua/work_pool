# WorkPool

A super simple implementation of a work queue wrapped by a thread pool.

The goal of this package is to have a quick way of creating a thread
pool with a work queue backing for quick projects.

# Example

```rust
fn main() {
    let mut pool = WorkPool::new();

    pool.set_executor(|work| {
        // Work is the data sent by `dispatch`
    });

    loop {
        // do something, like get a TcpStream
        let stream = accept_next_connection();
        match stream {
            Some(Ok(s)) => pool.dispatch(s),
            Some(Err(e)) => panic!(e),
            None => break,
        }
    }

    // Dropping the pool will send a Quit message to all active threads
    and detach them. No joins happen here.
    drop(pool);
}
```


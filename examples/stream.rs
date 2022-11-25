use work_pool::WorkPool;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::Arc;

fn handle(stream: Arc<TcpStream>) {
    println!("Got connection");

    drop(stream);
}

fn main() {
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let listener = TcpListener::bind(addr).expect("Failed to bind to port 8080");

    // Setting the threads to 0 will let the system choose the number
    // of threads. Setting the buffer length to None will default the
    // buffer length to a reasonable amount.
    let mut pool = WorkPool::new(0, None).expect("Failed to build work pool");
    pool.set_executor_and_start(|stream| { handle(stream); });

    println!("Bound to port 8080 - Send a request!");
    println!("Press Ctrl+C to quit ...");

    for stream in listener.incoming() {
        match stream {
            Ok(s) => pool.dispatch(Arc::new(s)),
            Err(e) => eprintln!("failed to get incomign stream {:?}", e),
        }
    }
}

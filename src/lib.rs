use std::thread;


pub struct ThreadPuddle {
    threads: Vec<thread::JoinHandle<()>>,
}

impl ThreadPuddle {
    pub fn new(n: usize) -> ThreadPuddle {
        assert!(n > 0);

        ThreadPuddle {
            threads: (0..n).map(|_| {thread::spawn(||{})}).collect(),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}

use std::thread;


pub struct ThreadPuddle {
    threads: Vec<thread::JoinHandle<()>>,
}

impl ThreadPuddle {
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}

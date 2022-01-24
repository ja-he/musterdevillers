use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPuddle {
    threads: Vec<Worker>,
}

impl ThreadPuddle {
    pub fn new(n: usize) -> ThreadPuddle {
        assert!(n > 0);

        ThreadPuddle {
            threads: (0..n).map(|_| Worker::new(Arc::clone(&receiver))).collect(),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}

struct Worker {
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            job();
        });

        Worker { thread }
    }
}

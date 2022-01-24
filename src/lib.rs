use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPuddle {
    sender: mpsc::Sender<Job>,
    threads: Vec<Worker>,
}

impl ThreadPuddle {
    pub fn new(n: usize) -> ThreadPuddle {
        assert!(n > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        ThreadPuddle {
            threads: (0..n).map(|_| Worker::new(Arc::clone(&receiver))).collect(),
            sender,
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

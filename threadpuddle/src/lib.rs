use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

/// A job a worker executes, e.g. reading file data from disk and writing it to a TCP stream.
type Job = Box<dyn FnOnce() + Send + 'static>;

/// An instruction for a worker, either to execute a job or to terminate working.
enum Instruction {
    Execute(Job),
    Terminate,
}

/// A collection of workers, each handling a thread in which jobs are executed.
/// Workers receive new jobs via a channel to which this type maintains the sending end.
pub struct ThreadPuddle {
    sender: mpsc::Sender<Instruction>,
    workers: Vec<Worker>,
}

impl ThreadPuddle {
    /// Create and return a new ThreadPuddle
    pub fn new(n: usize) -> ThreadPuddle {
        assert!(n > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        ThreadPuddle {
            workers: (0..n).map(|_| Worker::new(Arc::clone(&receiver))).collect(),
            sender,
        }
    }

    /// Send the given work out as a job to the workers.
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender
            .send(Instruction::Execute(Box::new(f)))
            .expect("could not send job");
    }
}

impl Drop for ThreadPuddle {
    /// Notify all workers to terminate and clean up their thread handles.
    fn drop(&mut self) {
        for _ in &mut self.workers {
            self.sender
                .send(Instruction::Terminate)
                .expect("could not send termination instruction");
        }

        for worker in &mut self.workers {
            worker.thread.take().unwrap().join().unwrap();
        }
    }
}

/// A worker which maintains a thread which waits for jobs to come through and executes them.
struct Worker {
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    /// Create and return a new Worker who listens to the passed receiving end of a channel for new
    /// jobs.
    fn new(receiver: Arc<Mutex<mpsc::Receiver<Instruction>>>) -> Worker {
        Worker {
            thread: Some(thread::spawn(move || loop {
                let instruction = receiver.lock().unwrap().recv().unwrap();
                match instruction {
                    Instruction::Execute(job) => job(),
                    Instruction::Terminate => break,
                }
            })),
        }
    }
}

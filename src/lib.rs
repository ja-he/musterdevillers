use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Instruction {
    Execute(Job),
    Terminate,
}

pub struct ThreadPuddle {
    sender: mpsc::Sender<Instruction>,
    workers: Vec<Worker>,
}

impl ThreadPuddle {
    pub fn new(n: usize) -> ThreadPuddle {
        assert!(n > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        ThreadPuddle {
            workers: (0..n).map(|_| Worker::new(Arc::clone(&receiver))).collect(),
            sender,
        }
    }

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

struct Worker {
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
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

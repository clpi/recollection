//! Module for handling concurrent communications
//! and job execution, related tasks.
//! Base on code from the Rust programming language book v2
//!
use crate::{RecolError, RecolResult};
use std::{sync::{self, Arc, Mutex, mpsc}, thread};

pub fn sender() -> () {
}

enum Op {
    NewAction(ActionHandler),
    Terminate,
}

trait OpLike {

}

pub struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>, 
}
impl Worker {

    fn new(id: usize, recv: Arc<Mutex<mpsc::Receiver<Op>>>) -> Self {
        let th = thread::spawn(move || {
            loop {
                let action = recv.lock().unwrap().recv().unwrap();
                match action {
                    Op::NewAction(handler) => {
                        println!("Worker got new job, handling {}", id );
                        handler.call_box();
                    },
                    Op::Terminate => {
                        println!("Worker {} stopping ", id);
                        break;
                    }
                }
            }
        });
        Worker { id, thread: Some(th) }
    }
}

trait ActionWrap {
    fn call_box(self: Box<Self>);
}
impl<F: FnOnce()> ActionWrap for F {
    fn call_box(self: Box<Self>) {
        (*self)()
    }
}

pub type ActionHandler = Box<dyn ActionWrap + Send + 'static>;

pub struct ActionQueue {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Op>
}

impl ActionQueue {

    pub fn new(size: usize) -> Self {
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        Self {
            workers,
            sender,
        }
    }
    pub fn exec<F: FnOnce() + Send + 'static>(&self, f: F) {
        let job = Box::new(f);
        self.sender.send(Op::NewAction(job)).unwrap();
    }
}

impl Drop for ActionQueue {
    fn drop(&mut self) {
        println!("Sending terminate sig to all workers");
        self.workers.iter()
            .inspect(|_| self.sender.send(Op::Terminate).unwrap());
        for worker in &mut self.workers {
                if let Some(thread) = worker.thread.take() {
                    thread.join();
                }
        }

        for worker in &mut self.workers {
            self.sender.send(Op::Terminate).unwrap();
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

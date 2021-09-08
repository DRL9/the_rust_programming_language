use std::{
    sync::{
        mpsc::{self, Sender},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
};

pub struct ThreadPool {
    sender: Sender<Message>,
    workers: Vec<Worker>,
}

impl ThreadPool {
    pub fn new(limit: usize) -> ThreadPool {
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(limit);
        for id in 0..limit {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool { sender, workers }
    }
    pub fn execute<F>(&self, func: F)
    where
        F: FnOnce() -> () + Send + 'static,
    {
        self.sender.send(Message::NewJob(Box::new(func))).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }
        println!("Shutting down all workers.");
        for worker in &mut self.workers {
            println!("shutdown worker{}", worker.id);
            if let Some(t) = worker.thread.take() {
                t.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>,
}

enum Message {
    NewJob(Job),
    Terminate,
}

type Job = Box<dyn FnOnce() -> () + 'static + Send>;

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            if let Message::NewJob(job) = job {
                println!("worker[{}] executing", id);
                job();
                println!("worker[{}] executing end", id);
            } else {
                println!("worker[{}] start terminate", id);
                // 终止循环
                break;
            }
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}

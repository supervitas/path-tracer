extern crate num_cpus;

use std::thread;
use core::time;
use std::sync::mpsc;
use std::sync::mpsc::Sender;

pub struct ThreadPool {
    threads: usize,
    workers: Vec<(Sender<()>, thread::JoinHandle<()>)>,
}

impl ThreadPool {
    pub fn new() -> Self {
        let threads = num_cpus::get();
        let ten_millis = time::Duration::from_millis(10);

        let mut workers = Vec::with_capacity(threads);

        for i in 0..threads {
            let (tx, rx) = mpsc::channel();

            let worker = thread::spawn(move || loop {
                match rx.try_recv() {
                    Ok(_) => { break; } // destroy thread on message;
                    _ => {
                        thread::sleep(ten_millis);

                    }
                }
            });

            workers.push((tx, worker));
        }

        ThreadPool {
            threads,
            workers,
        }

    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for (sender, _) in &self.workers {
            sender.send(()); // destroy threads
        }
    }
}

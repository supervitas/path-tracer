extern crate num_cpus;

use std::thread;
use std::sync::{Arc, Condvar, Mutex};
use std::sync::mpsc::Sender;
use std::sync::atomic::{AtomicBool, Ordering, AtomicUsize};
use std::thread::yield_now;
use std::collections::VecDeque;


pub struct ThreadPool {
    threads: usize,
    task_count: Arc<AtomicUsize>,
    stop: Arc<AtomicBool>,
    workers: Vec<thread::JoinHandle<()>>,
    cond_mutex_pair: Arc<(Mutex<VecDeque<Box<dyn FnOnce() + Send + 'static>>>, Condvar)>
}

impl ThreadPool {
    pub fn new() -> Self {
        let threads = num_cpus::get();
        let workers = Vec::with_capacity(threads);
        let stop = Arc::new(AtomicBool::from(false));
        let cond_mutex_pair = Arc::new((Mutex::new(VecDeque::new()), Condvar::new()));
        let task_count = Arc::new(AtomicUsize::new(0));

        let mut thread_pool = ThreadPool {
            threads,
            workers,
            stop,
            cond_mutex_pair,
            task_count
        };

        thread_pool.create_workers();

        thread_pool
    }

    pub fn get_workers_num(&self) -> usize {
        return self.threads;
    }

    pub fn create_workers(&mut self) {
        for _i in 0..self.threads {
            let stop_ref = Arc::clone(&self.stop);
            let pair = self.cond_mutex_pair.clone();
            let task_count = self.task_count.clone();

            let worker = thread::spawn(move || loop {
                let task;
                {
                    let (lock, cvar) = &*pair;
                    let mut started = lock.lock().unwrap();

                    while (*started).len() == 0 {
                        started = cvar.wait(started).unwrap();
                    }

                    task = started.pop_front().unwrap();

                    if stop_ref.load(Ordering::Relaxed)  {
                        println!("STOP");
                        return;
                    }
                }
                task();

                task_count.fetch_sub(1, Ordering::Release);
            });

            self.workers.push( worker);
        }
    }

    pub fn add_task(&mut self, task: Box<dyn FnOnce() + Send + 'static>) {
        let (lock, cvar) = &*self.cond_mutex_pair;
        {
            (*lock.lock().unwrap()).push_back(task);
        }
        self.task_count.fetch_add(1, Ordering::Release);
        cvar.notify_one();
    }

    pub fn wait_all(&self) {
        while self.task_count.load(Ordering::Relaxed) != 0 {
            yield_now();
        }
    }

    pub fn destroy(&self) {
        self.stop.store(true, Ordering::Release);
        let (_, cvar) = &*self.cond_mutex_pair;
        cvar.notify_all();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        self.destroy();
    }
}

extern crate num_cpus;

use std::thread;
use std::sync::{Arc, Condvar, Mutex};
use std::sync::mpsc::Sender;
use std::sync::atomic::{AtomicBool, Ordering, AtomicUsize};
use std::thread::yield_now;
use std::collections::VecDeque;

struct ThreadPoolData {
    task_count: AtomicUsize,
    stop: AtomicBool,
    cond_mutex_pair: (Mutex<VecDeque<Box<dyn FnOnce() + Send + 'static>>>, Condvar)
}

pub struct ThreadPool {
    threads: usize,
    workers: Vec<thread::JoinHandle<()>>,
    thread_pool_data: Arc<ThreadPoolData>
}

impl ThreadPool {
    pub fn new() -> Self {
        let threads = num_cpus::get();
        let workers = Vec::with_capacity(threads);

        let thread_pool_data = Arc::new(
            ThreadPoolData {
                stop: AtomicBool::from(false),
                cond_mutex_pair: (Mutex::new(VecDeque::new()), Condvar::new()),
                task_count: AtomicUsize::new(0)
            }
        );

        let mut thread_pool = ThreadPool {
            threads,
            workers,
            thread_pool_data
        };

        thread_pool.create_workers();

        thread_pool
    }

    pub fn get_workers_num(&self) -> usize {
        return self.threads;
    }

    fn create_workers(&mut self) {
        for _i in 0..self.threads {
            let thread_pool_data = Arc::clone(&self.thread_pool_data);

            let worker = thread::spawn(move || loop {
                let task;
                {
                    let (lock, cvar) = &thread_pool_data.cond_mutex_pair;
                    let mut started = lock.lock().unwrap();

                    while (*started).len() == 0 {
                        started = cvar.wait(started).unwrap();
                    }

                    task = started.pop_front().unwrap();

                    if thread_pool_data.stop.load(Ordering::Relaxed)  {
                        return;
                    }
                }
                task();

                thread_pool_data.task_count.fetch_sub(1, Ordering::Release);
            });

            self.workers.push( worker);
        }
    }

    pub fn add_task(&mut self, task: Box<dyn FnOnce() + Send + 'static>) {
        let (lock, cvar) = &self.thread_pool_data.cond_mutex_pair;
        {
            (*lock.lock().unwrap()).push_back(task);
        }
        self.thread_pool_data.task_count.fetch_add(1, Ordering::Release);
        cvar.notify_one();
    }

    pub fn wait_all(&self) {
        while self.thread_pool_data.task_count.load(Ordering::Relaxed) != 0 {
            yield_now();
        }
    }

    pub fn destroy(&self) {
        self.thread_pool_data.stop.store(true, Ordering::Release);
        let (_, cvar) = &self.thread_pool_data.cond_mutex_pair;
        cvar.notify_all();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        self.destroy();
    }
}

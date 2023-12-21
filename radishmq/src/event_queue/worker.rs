use crate::event_queue::event::TopicEvent;
use crate::utils::queue::Queue;
use crate::{db, STATE};
use futures::executor::block_on;
use sqlx::{Pool, Postgres};
use std::env;
use std::sync::{Arc, Mutex};

pub type EventQueue = Queue<TopicEvent>;

pub struct ThreadData {
    pub db_connection_pool: Pool<Postgres>,
}

#[allow(dead_code)]
struct SyncFlagTx {
    inner: Arc<Mutex<bool>>,
}

#[allow(dead_code)]
impl SyncFlagTx {
    fn set(&mut self, state: bool) -> Result<(), ()> {
        if let Ok(mut v) = self.inner.lock() {
            *v = state;
            Ok(())
        } else {
            Err(())
        }
    }
}

#[derive(Clone)]
struct SyncFlagRx {
    inner: Arc<Mutex<bool>>,
}

impl SyncFlagRx {
    fn get(&self) -> Result<bool, ()> {
        if let Ok(v) = self.inner.lock() {
            // Deref the MutexGuard to get at the bool inside
            Ok(*v)
        } else {
            Err(())
        }
    }
}

fn new_syncflag(initial_state: bool) -> (SyncFlagTx, SyncFlagRx) {
    let state = Arc::new(Mutex::new(initial_state));
    let tx = SyncFlagTx {
        inner: state.clone(),
    };
    let rx = SyncFlagRx {
        inner: state.clone(),
    };

    (tx, rx)
}

pub async fn create_event_queue_workers() {
    use std::sync::mpsc::channel;
    let (results_tx, _) = channel();

    let (_, more_jobs_rx) = new_syncflag(true);

    use std::thread;
    let mut threads = Vec::new();

    let max_workers = env::var("MAX_WORKER")
        .expect("MAX_WORKER must be set")
        .parse::<u16>()
        .expect("HTTP_PORT must be a number");

    println!("Spawning {} workers.", max_workers);

    for thread_num in 0..max_workers {
        let thread_db_connection_pool = match db::pool::initialize_connection_pool().await {
            Some(pool) => pool,
            None => panic!("DB connection pool could not be initialized"),
        };

        let mut thread_queue = {
            match STATE.event_queue.lock() {
                Ok(queue) => queue.clone(),
                Err(_) => panic!("create_event_queue_workers: tried to lock a poised mutex"),
            }
        };

        // Similarly, create a new transmitter for the thread to use
        let thread_results_tx = results_tx.clone();

        // ... and a SyncFlagRx for the thread.
        let thread_more_jobs_rx = more_jobs_rx.clone();

        // thread::spawn takes a closure (an anonymous function that "closes"
        // over its environment). The move keyword means it takes ownership of
        // those variables, meaning they can't be used again in the main thread.
        let handle = thread::spawn(move || {
            let mut work_done = 0;

            // Loop while there's expected to be work, looking for work.
            while thread_more_jobs_rx.get().unwrap() {
                // If work is available, do that work.
                if let Some(work) = thread_queue.dequeue() {
                    block_on(work.handle(ThreadData {
                        db_connection_pool: thread_db_connection_pool.clone(),
                    }));

                    // Record that some work was done.
                    work_done += 1;

                    // Send the work and the result of that work.
                    //
                    // Sending could fail. If so, there's no use in
                    // doing any more work, so abort.
                    match thread_results_tx.send(work) {
                        Ok(_) => (),
                        Err(_) => {
                            break;
                        }
                    }
                }

                // Signal to the operating system that now is a good time
                // to give another thread a chance to run.
                //
                // This isn't strictly necessary - the OS can preemptively
                // switch between threads, without asking - but it helps make
                // sure that other threads do get a chance to get some work.
                thread::yield_now();
            }

            // Report the amount of work done.
            println!("Thread {} did {} jobs.", thread_num, work_done);
        });

        // Add the handle for the newly spawned thread to the list of handles
        threads.push(handle);
    }
}

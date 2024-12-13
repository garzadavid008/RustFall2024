use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::config::WebsiteStatus;
use crate::monitor::check_website;

pub fn create_thread_pool(
    num_threads: usize,
    job_receiver: Arc<Mutex<mpsc::Receiver<String>>>,
    result_sender: mpsc::Sender<WebsiteStatus>,
    timeout: Duration,
    retries: usize,
) {
    for _ in 0..num_threads {
        let job_receiver = Arc::clone(&job_receiver);
        let result_sender = result_sender.clone();

        thread::spawn(move || {
            while let Ok(url) = {
                let lock = job_receiver.lock().unwrap();
                lock.recv()
            } {
                let mut attempts = 0;
                let result = loop {
                    let status = check_website(&url, timeout);

                    if status.status.is_ok() || attempts >= retries {
                        break status;
                    }
                    attempts += 1;
                };

                if result_sender.send(result).is_err() {
                    break; 
                }
            }
        });
    }
}

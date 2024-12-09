use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::config::WebsiteStatus;
use crate::monitor::check_website;

/// Spawns a thread pool to handle website monitoring jobs.
///
/// # Parameters
/// - `num_threads`: Number of worker threads to spawn.
/// - `job_receiver`: Shared receiver for fetching jobs.
/// - `result_sender`: Sender to return `WebsiteStatus` to the main thread.
/// - `timeout`: Timeout duration for HTTP requests.
/// - `retries`: Maximum number of retries for each website.
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

                    // Stop retrying on success or after exhausting retries
                    if status.status.is_ok() || attempts >= retries {
                        break status;
                    }
                    attempts += 1;
                };

                // Send the result back to the main thread
                if result_sender.send(result).is_err() {
                    break; // Exit if the main thread is no longer listening
                }
            }
        });
    }
}

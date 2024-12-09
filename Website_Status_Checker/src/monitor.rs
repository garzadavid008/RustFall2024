use std::time::{Duration, Instant, SystemTime};
use std::sync::{mpsc, Arc, Mutex};
use crate::config::{Config, WebsiteStatus};
use crate::worker::create_thread_pool;
use ureq;

/// Check the status of a single website.
///
/// # Arguments
/// - `url`: The website URL to check.
/// - `timeout`: The duration to wait before timing out the request.
///
/// # Returns
/// A `WebsiteStatus` struct with the results of the check.
pub fn check_website(url: &str, timeout: Duration) -> WebsiteStatus {
    let start_time = Instant::now();

    // Perform the HTTP request
    let response = ureq::get(url)
        .timeout(timeout)
        .call();

    // Determine the status
    let status = if let Ok(response) = response {
        Ok(response.status())
    } else {
        Err(response.unwrap_err().to_string())
    };

    // Construct and return the result
    WebsiteStatus {
        url: url.to_string(),
        status,
        response_time: start_time.elapsed(),
        timestamp: SystemTime::now(), // Use SystemTime to get the current timestamp
    }
}

/// Monitor a list of websites concurrently.
///
/// # Arguments
/// - `urls`: A list of website URLs to monitor.
/// - `config`: Configuration options for the monitoring process.
///
/// # Returns
/// A vector of `WebsiteStatus` objects with the results of the monitoring.
pub fn monitor_websites(urls: Vec<String>, config: Config) -> Vec<WebsiteStatus> {
    let (job_sender, job_receiver) = mpsc::channel();
    let (result_sender, result_receiver) = mpsc::channel();

    let job_receiver = Arc::new(Mutex::new(job_receiver));

    // Spawn the thread pool
    create_thread_pool(
        config.num_threads,
        job_receiver.clone(),
        result_sender,
        config.timeout,
        config.retries,
    );

    // Send jobs to the worker threads by borrowing the URLs
    for url in &urls {  // Borrow the vector, not moving it
        job_sender.send(url.clone()).unwrap();  // Clone the URL to send
    }

    // Close the job sender to signal the end of jobs
    drop(job_sender);

    // Collect and return results
    let mut results = Vec::new();
    for _ in 0..urls.len() {
        if let Ok(result) = result_receiver.recv() {
            results.push(result);
        }
    }

    results
}

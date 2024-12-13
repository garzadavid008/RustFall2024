use std::time::{Duration, Instant, SystemTime};
use std::sync::{mpsc, Arc, Mutex};
use crate::config::{Config, WebsiteStatus};
use crate::worker::create_thread_pool;
use ureq;

pub fn check_website(url: &str, timeout: Duration) -> WebsiteStatus {
    let start_time = Instant::now();

    let response = ureq::get(url)
        .timeout(timeout)
        .call();

    let status = if let Ok(response) = response {
        Ok(response.status())
    } else {
        Err(response.unwrap_err().to_string())
    };

    WebsiteStatus {
        url: url.to_string(),
        status,
        response_time: start_time.elapsed(),
        timestamp: SystemTime::now(),
    }
}

pub fn monitor_websites(urls: Vec<String>, config: Config) -> Vec<WebsiteStatus> {
    let (job_sender, job_receiver) = mpsc::channel();
    let (result_sender, result_receiver) = mpsc::channel();

    let job_receiver = Arc::new(Mutex::new(job_receiver));

    create_thread_pool(
        config.num_threads,
        job_receiver.clone(),
        result_sender,
        config.timeout,
        config.retries,
    );

    for url in &urls {  
        job_sender.send(url.clone()).unwrap();  
    }

    drop(job_sender);

    let mut results = Vec::new();
    for _ in 0..urls.len() {
        if let Ok(result) = result_receiver.recv() {
            results.push(result);
        }
    }

    results
}

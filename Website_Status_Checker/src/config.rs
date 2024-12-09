use std::time::{Duration, SystemTime};

/// Represents the monitoring result for a single website.
#[derive(Debug)]
pub struct WebsiteStatus {
    pub url: String,                  // URL of the website
    pub status: Result<u16, String>, // HTTP status code or error message
    pub response_time: Duration,     // Time taken to get a response
    pub timestamp: SystemTime,       // Timestamp of the check (using SystemTime instead of DateTime)
}

/// Configuration options for the website monitoring system.
#[derive(Debug)]
pub struct Config {
    pub timeout: Duration,  // Timeout duration for HTTP requests
    pub retries: usize,     // Maximum number of retries per website
    pub num_threads: usize, // Number of worker threads
}

/// Provides default values for the `Config` struct.
impl Default for Config {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(5),
            retries: 3,
            num_threads: 4,
        }
    }
}

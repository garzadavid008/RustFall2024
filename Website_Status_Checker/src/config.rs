use std::time::{Duration, SystemTime};

#[derive(Debug)]
pub struct WebsiteStatus {
    pub url: String,                  
    pub status: Result<u16, String>, 
    pub response_time: Duration,     
    pub timestamp: SystemTime,      
}

#[derive(Debug)]
pub struct Config {
    pub timeout: Duration,  
    pub retries: usize,     
    pub num_threads: usize, 
}

impl Default for Config {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(5),
            retries: 3,
            num_threads: 4,
        }
    }
}

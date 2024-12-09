mod config;
mod worker;
mod monitor;

use config::Config;
use monitor::monitor_websites;
use std::time::Duration;

fn main() {
    // Step 1: Read Configuration
    let config = Config {
        timeout: Duration::from_secs(5), // Timeout for each request
        retries: 3,                      // Number of retries for failed requests
        num_threads: 4,                  // Number of worker threads
    };

    // Step 2: Define URLs to Monitor
    let urls = vec![
        "https://www.google.com".to_string(),
        "https://www.rust-lang.org".to_string(),
        "https://nonexistent.website".to_string(),
    ];

    // Step 3: Perform Monitoring
    println!("Starting website monitoring...");
    let results = monitor_websites(urls, config);

    // Step 4: Display Results
    println!("\nMonitoring Results:");
    for result in results {
        match result.status {
            Ok(status) => println!(
                "URL: {}, Status: {}, Response Time: {:?}, Checked At: {:?}",
                result.url, status, result.response_time, result.timestamp
            ),
            Err(err) => println!(
                "URL: {}, Error: {}, Checked At: {:?}",
                result.url, err, result.timestamp
            ),
        }
    }
}

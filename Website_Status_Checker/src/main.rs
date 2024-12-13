mod config;
mod worker;
mod monitor;

use config::Config;
use monitor::monitor_websites;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Duration;

fn load_into_vec() -> Vec<String> {
    let mut result = Vec::new();
    let file = File::open("links.txt").unwrap();
    let content = BufReader::new(file);

    for line in content.lines() {
        if let Ok(url) = line {
            result.push(url);
        }
    }
    result
}

fn main() {
    let config = Config {
        timeout: Duration::from_secs(5), 
        retries: 3,                    
        num_threads: 4,                  
    };

    let urls = load_into_vec();

    println!("Starting website monitoring...");
    let results = monitor_websites(urls, config);

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

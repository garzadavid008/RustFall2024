#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{TcpListener, TcpStream};
    use std::io::{Write, Read};
    use std::thread;
    use std::time::Duration;

    fn mock_http_server(response: &'static str) -> String {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap().to_string();

        thread::spawn(move || {
            for stream in listener.incoming() {
                let mut stream = stream.unwrap();
                let mut buffer = [0; 512];
                stream.read(&mut buffer).unwrap();
                stream.write(response.as_bytes()).unwrap();
            }
        });

        addr
    }

    #[test]
    fn test_check_website_success() {
        let addr = mock_http_server("HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\n");
        let result = check_website(&format!("http://{}", addr), Duration::from_secs(5));
        assert_eq!(result.status.unwrap(), 200);
    }

    #[test]
    fn test_check_website_invalid_url() {
        let url = "invalid-url";
        let timeout = Duration::from_secs(5);
        let result = check_website(url, timeout);

        assert!(result.status.is_err());
    }

    #[test]
    fn test_check_website_success_static_url() {
        let url = "https://example.com";
        let timeout = Duration::from_secs(5);
        let result = check_website(url, timeout);

        assert!(result.status.is_ok());
    }
}

use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};
pub mod store;
fn main() {
    let s = store::Store::new();
    let stream = TcpListener::bind("127.0.0.1:8080").expect("Could not bind to address");
    for stream in stream.incoming() {
        match stream {
            Ok(stream) => {
                let now = std::time::Instant::now();
                std::thread::spawn(|| handle_connection(stream, &s));
                println!(
                    "Time elapsed in expensive_function() is: {:?}",
                    now.elapsed()
                );
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream, s: &store::Store) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write_all(response.as_bytes()).unwrap();
}

use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use lazy_static::lazy_static; // 1.4.0
use std::sync::Mutex;

pub mod store;
pub mod sets;

lazy_static! {
    static ref DATA: Mutex<store::Store> = Mutex::new(store::Store::new());
}

fn exec(cmd: &String) -> String {
    let mut data = DATA.lock().unwrap();
    let res = data.exec(cmd);
    match res {
        Ok(s) => {
            match s {
                Some(s) => s,
                None => "OK".to_string(),
            }
        }
        Err(e) => e.to_string(),
    }
}


fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 50]; // using 50 byte buffer
    while match stream.read(&mut data) {
        Ok(size) => {
            if size == 0 {
                println!("The client has disconnected");
                stream.shutdown(Shutdown::Both).unwrap();
                return;
            }
            println!("Received {} bytes", size);
            let cmd = String::from_utf8_lossy(&data[0..size]);
            let res = exec(&cmd.to_string());
            println!("Sending response: {}", res);
            stream.write_all(res.as_bytes()).unwrap();
            // stream.write(&data[0..size]).unwrap();
            true
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 3333");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move|| {
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
    // close the socket server
    drop(listener);
}

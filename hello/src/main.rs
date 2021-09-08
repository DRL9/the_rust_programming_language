use std::{
    fs,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread::{self, sleep},
    time::Duration,
};

use hello::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(3);
    for stream in listener.incoming().take(7) {
        let stream = stream.unwrap();
        println!("Connection established!");
        &pool.execute(move || handle_connection(&stream));
    }
    println!("shutdown");
    thread::sleep(Duration::from_secs(1));
}
fn handle_connection(mut stream: &TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    // println!("Request: \n{}", String::from_utf8_lossy(&buffer[..]));·
    let get = b"GET / HTTP/1.1\r\n";
    if buffer.starts_with(get) {
        send_file(stream, 200, "OK", "index.html");
    } else {
        sleep(Duration::from_secs(2));
        send_file(stream, 404, "Not Found", "404.html");
    }
}

fn send_file(mut stream: &TcpStream, status: u32, phase: &str, file_path: &str) {
    let html = fs::read_to_string(file_path).unwrap();
    let response = format!(
        "HTTP/1.1 {} {}\r\nContent-Length: {}\r\n\r\n{}",
        status,
        phase,
        html.len(),
        html
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

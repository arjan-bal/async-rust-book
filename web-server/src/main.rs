use std::{
    fs,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    const PORT: usize = 7878;
    let listener = TcpListener::bind(format!("127.0.0.1:{}", PORT)).unwrap();
    println!("Server is running on port {}", PORT);

    for stream_result in listener.incoming() {
        let stream = stream_result.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let (status, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{} {}", status, contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

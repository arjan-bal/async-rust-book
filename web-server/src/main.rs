use std::time::Duration;

use tokio::{
    fs,
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

#[tokio::main]
async fn main() {
    const PORT: usize = 7878;
    let listener = TcpListener::bind(format!("127.0.0.1:{}", PORT))
        .await
        .unwrap();
    println!("Server is running on port {}", PORT);

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        tokio::spawn(handle_connection(stream));
    }
}

async fn handle_connection(mut stream: TcpStream) {
    println!("Handling a request");
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await.unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        tokio::time::sleep(Duration::new(5, 0)).await;
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).await.unwrap();
    let response = format!("{} {}", status, contents);
    stream.write(response.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
    println!("Completed handling request");
}

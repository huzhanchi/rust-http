use tokio::{
    fs,
    net::{TcpListener, TcpStream},
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    time::sleep,
};

use std::time::Duration;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();
    println!("Http server is listening on 127.0.0.1:7878");

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            handle_connection(stream).await;
        });
    }
}


async fn handle_connection(mut stream: TcpStream) {
    let mut  buff_reader = BufReader::new(&mut stream);
    let mut request_line = String::new();
    buff_reader.read_line(&mut request_line).await.unwrap();

    let (status_line, filename) = match request_line.trim() {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            sleep(Duration::from_secs(10)).await;
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };
    let contents = fs::read_to_string(filename).await.unwrap();
    let length = contents.len();

    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
    );

    stream.write_all(response.as_bytes()).await.unwrap();
    println!("{request_line} has responsed");
}


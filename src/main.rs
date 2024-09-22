use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}


fn handle_connection(mut stream: TcpStream) {
    let buff_reader = BufReader::new(&mut stream);

    let request_line = buff_reader.lines().next().unwrap().unwrap();
    let status_line: String;
    let length: usize;
    let contents: String;

    if request_line == "GET / HTTP/1.1" {
        status_line = "HTTP/1.1 200 OK".to_string();
        contents = fs::read_to_string("hello.html").unwrap();
        length = contents.len();
    } else {
        status_line = "HTTP/1.1 404 NOT FOUND".to_string();
        contents = "not found".to_string();
        length = contents.len();
    }

    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
    );
    stream.write_all(response.as_bytes()).unwrap();
}


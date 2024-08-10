use std::net::TcpListener;
use std::net::TcpStream;

use std::io::prelude::*;
use std::fs;

fn main() {

    let listener: TcpListener = TcpListener::bind("127.0.0.1:7878").unwrap();


    for stream in listener.incoming() {
   
        let stream: TcpStream = stream.unwrap();
        handle_connection(stream);

    }

}

fn handle_connection(mut stream: TcpStream) {

    let mut buffer = [0;1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
            ("HTTP/1.1 200 OK", "./src/html-pages/index.html")
    } else {
            ("HTTP/1.1 404 NOT FOUND", "./src/html-pages/404.html")
    };

    let contents: String =
        fs::read_to_string(filename).unwrap();

    let reponse: String = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(reponse.as_bytes()).unwrap();
    stream.flush().unwrap();

}

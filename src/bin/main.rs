use custom_web_server::ThreadPool;
use std::io::prelude::*;
use std::fs;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let response;

    if buffer.starts_with(get) {
        response = construct_response("200 OK", "templates/hello.html");
    } else {
        response = construct_response("404 NOT FOUND", "templates/404.html");
    }

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn construct_response(status: &str, template_path: &str) -> String {
    let protocol = "HTTP/1.1";
    let contents = fs::read_to_string(&template_path).unwrap();
    let response = format!("{} {}\r\n\r\n{}", protocol, &status, contents);
    response
}

use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};

fn main() {
    //listing on localhost port 7878 for TCP connection
    //unwrap stops the program if errors happen
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream)
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    // let http_request: Vec<_> = buf_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let parts: Vec<&str> = request_line.split_whitespace().collect();
    let method = parts[0];
    let path = parts[1];
    //if they are looking for the main page, give it to them
    if method == "GET" && path == "/"
    {
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("../../static/index.html").unwrap();
        let length = contents.len();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
        stream.write_all(response.as_bytes()).unwrap();

    }
    else if method == "GET" && path.starts_with("/search")
    {
        
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("../../static/index.html").unwrap();
        let length = contents.len();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
        stream.write_all(response.as_bytes()).unwrap();
    }



}
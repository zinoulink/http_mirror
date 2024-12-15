use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};
use threadpool::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    let pool = ThreadPool::new(25);

    println!("http_mirror is running ...");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let http_request = request_line.replace("GET ", "").replace(" HTTP/1.1", "");

    let http_response = reqwest::blocking::get(format!("https://en.wikipedia.com{http_request}")).unwrap();


    let content: &[u8] = &http_response.bytes().unwrap();

    stream.write_all(content).unwrap();
    stream.flush().unwrap();
}

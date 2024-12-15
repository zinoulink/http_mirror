use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    str, env,
};
use threadpool::ThreadPool;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("No arguments provided.");
        return;
    }

    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    let pool = ThreadPool::new(25);

    println!("http_mirror is running ...");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let main_host = args[1].clone();
        pool.execute(|| {
            handle_connection(stream, main_host);
        });
    }
}

fn handle_connection(mut stream: TcpStream, main_host: String) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let http_request = request_line.replace("GET ", "").replace(" HTTP/1.1", "");

    let http_response = reqwest::blocking::get(format!("https://{main_host}{http_request}")).unwrap();

    let mut string_response = format!("HTTP/1.1 {} \r\n", http_response.status());
    for header in http_response.headers() {
        let key = header.0.as_str();
        if key == "set-cookie:" {
            let value = header.1.as_bytes();
            string_response = format!(
                "{}{}: {}\r\n",
                string_response,
                key,
                str::from_utf8(value).unwrap()
            );
        }
    }
    string_response = format!("{}\r\n", string_response);

    println!("{string_response}");
    stream.write_all(string_response.as_bytes()).unwrap();

    let content: &[u8] = &http_response.bytes().unwrap();

    stream.write_all(content).unwrap();
    stream.flush().unwrap();
}

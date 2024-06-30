use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

use anyhow::Result;

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => handle_stream(&mut stream)?,
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
    Ok(())
}

fn handle_stream(stream: &mut TcpStream) -> Result<()> {
    println!("Accepted new connection from {}.", stream.peer_addr()?);

    let mut buf = [0; 1024];
    stream.read(&mut buf).unwrap();

    let mut headers = [httparse::EMPTY_HEADER; 4];
    let mut h_req = httparse::Request::new(&mut headers);
    let res = h_req.parse(&buf).unwrap();
    dbg!(headers);

    let req = String::from_utf8(buf.to_vec()).unwrap();
    let req_vec = req.split(" ").collect::<Vec<&str>>();

    let req_path = req_vec[1]
        .split("/")
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();

    let res = match req_path.is_empty() {
        true => "HTTP/1.1 200 OK\r\n\r\n".to_string(),
        false => {
            match req_path[0] {
                "echo" => {
                    format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",req_path[1].len(), req_path[1])
                }
                "user-agent" => {
                    let header = headers.iter().find(|header| header.name == "User-Agent");
                    let val = String::from_utf8(header.unwrap().value.to_vec())?;

                    format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",header.unwrap().value.len(), val )
                }
                _ => "HTTP/1.1 404 Not Found\r\n\r\n".to_string(),
            }
        }
    };

    stream.write(res.as_bytes()).unwrap();
    Ok(())
}

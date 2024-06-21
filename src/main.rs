use std::{
    io::{Read, Write},
    net::TcpListener,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");
                let mut buf = [0; 100];
                stream.read(&mut buf).unwrap();
                let req = String::from_utf8(buf.to_vec()).unwrap();
                let req_vec = req.split(" ").collect::<Vec<&str>>();
                let req_path = req_vec[1]
                    .split("/")
                    .filter(|s| !s.is_empty())
                    .collect::<Vec<_>>();

                let res = match req_path.is_empty() {
                    true => "HTTP/1.1 200 OK\r\n\r\n".to_string(),
                    false => match req_path[0] {
                        "echo" => {
                            format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",req_path[1].len(), req_path[1])
                        }
                        _ => "HTTP/1.1 404 Not Found\r\n\r\n".to_string(),
                    },
                };

                stream.write(res.as_bytes()).unwrap();
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

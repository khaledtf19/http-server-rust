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
                println!("{:?}", req.split(" ").collect::<Vec<_>>());
                let req_vec = req.split(" ").collect::<Vec<&str>>();
                let res = match req_vec[1] {
                    "/" => "HTTP/1.1 200 OK\r\n\r\n",
                    _ => "HTTP/1.1 404 Not Found\r\n\r\n",
                };

                stream.write(res.as_bytes()).unwrap();
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

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
                let req_path = req_vec[1];

                let res = match req_path {
                    "/" => "HTTP/1.1 200 OK\r\n\r\n".to_string(),
                    path => {
                        if path.starts_with("/echo/") {
                            let path_vec = path
                                .split("/")
                                .filter(|st| !st.is_empty())
                                .collect::<Vec<&str>>();
                            format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",path_vec[1].len(), path_vec[1])
                        } else {
                            "HTTP/1.1 404 Not Found\r\n\r\n".to_string()
                        }
                    }
                };

                stream.write(res.as_bytes()).unwrap();
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

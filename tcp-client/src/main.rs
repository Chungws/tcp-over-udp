use std::{io::prelude::*, net::TcpStream, str};

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:7878").expect("couldn't connect with server");
    let message = "hello, window";

    stream
        .write(message.as_bytes())
        .expect("couldn't send message to server");

    let mut buf = [0; 2048];
    let number_of_bytes = stream.read(&mut buf).expect("couldn't get echo");

    let echo = str::from_utf8(&buf[..number_of_bytes]).unwrap();
    println!("Echo : {}", echo);
}

use std::net::UdpSocket;
use std::str;

fn main() {
    let dst_addr = "127.0.0.1:34254";
    let socket = UdpSocket::bind("127.0.0.1:2048").expect("couldn't bind this address");

    let message = "hello, window";

    socket.send_to(message.as_bytes(), dst_addr.to_string()).expect("Error on send");

    let mut buf = [0; 2048];
    let (number_of_bytes, _) = socket.recv_from(&mut buf).expect("Didn't receive data");

    let echo = str::from_utf8(&buf[..number_of_bytes]).unwrap();
    println!("Echo {}", echo);
}

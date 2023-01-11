use std::net::UdpSocket;

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:34254").expect("couldn't bind to address");

    let mut buf = [0; 2048];

    loop {
        let (number_of_bytes, src_addr) = socket.recv_from(&mut buf).expect("Didn't receive data");

        let filled_buf = &mut buf[..number_of_bytes];
        
        socket.send_to(filled_buf, &src_addr).expect("Error on Send");
    }
}

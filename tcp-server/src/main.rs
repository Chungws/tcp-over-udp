use std::{
    io::prelude::*,
    net::{TcpListener, TcpStream},
};

fn main() {
    let listner = TcpListener::bind("127.0.0.1:7878").expect("couldn't bind this address");

    for stream in listner.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream);
            }
            Err(_) => {
                println!("Connection Failed!");
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut read = [0; 2048];
    loop {
        match stream.read(&mut read) {
            Ok(n) => {
                if n == 0 {
                    break;
                }
                stream.write(&read[..n]).expect("couldn't write");
            }
            Err(_) => {
                println!("Error occured!");
                break;
            }
        }
    }
    println!("Disconnected");
}

extern crate rand;

use rand::Rng;
use std::env;

fn main() {
    println!("sjs0468 - Stell Shuman-Thomas");
    let args: Vec<String> = env::args().collect();

    let addr_ip = args[1].clone();
    let addr_port: u16 = args[2].trim().parse().expect("not a valid port number");

    let addr = (addr_ip, addr_port);
    let socket = std::net::UdpSocket::bind(addr).expect("couldn't bind to address");

    let mut rng = rand::rng();
    let pong = "PONG".as_bytes();

    println!("[server]: Ready to receive data..");
    loop {
        let mut buf = [0; 10];
        let (amt, src) = socket.recv_from(&mut buf).expect("didn't receive data");
        let vec = Vec::from(&mut buf[..amt]);
        let message = String::from_utf8(vec).expect("not a valid UTF-8 string");

        println!("[client]: {message}");

        if rng.random_ratio(7, 10) {
            let result = socket.send_to(pong, src);
            match result {
                Ok(_) => println!("[server]: PONG"),
                Err(_) => println!("Couldn't send message"),
            }
        } else {
            println!("[server]: Dropping packet");
        }
    }
}

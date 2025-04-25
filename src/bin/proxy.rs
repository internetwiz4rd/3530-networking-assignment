use std::{env, io, net, result, time::Duration};

fn main() {
    println!("sjs0468 - Stell Shuman-Thomas");

    let args: Vec<String> = env::args().collect();

    let proxy_port: u16 = args[1].trim().parse().expect("not a valid port number");
    
    let server_addrs: Vec<(String, u16)> = args[2..].iter()
        .map(|port| (String::from("127.0.0.1"), port.trim().parse().expect("not a valid port number"))).collect();

    let proxy_addr = ("127.0.0.1", proxy_port);
    let socket = net::UdpSocket::bind(proxy_addr).expect("couldn't bind to address");

    let _ = socket.set_read_timeout(Some(Duration::new(3, 0)));

    loop {
        for server_addr in &server_addrs {
            let mut buf = [0; 10];
            let result = socket.recv_from(&mut buf);

            print!("{}", server_addr.1);
            
            match result {
                Ok((amt, source_addr)) => {

                },
                Err(_) => todo!()
            }

        }
    }
}

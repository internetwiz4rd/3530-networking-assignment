use std::{env, time::Duration, thread};

fn main() -> std::io:: Result<()> {
    let args: Vec<String> = env::args().collect();

    let dest_addr_ip = args[1].clone();
    let dest_addr_port: u16 = args[2].trim().parse().expect("not a valid port number");

    let src_addr = ("127.0.0.1", 12345);
    let dest_addr = (dest_addr_ip, dest_addr_port);
    let socket = std::net::UdpSocket::bind(src_addr).expect("couldn't bind to address");

    socket.set_read_timeout(Some(Duration::new(5, 0)))?;


    let ping = "PING".as_bytes();

    for n in 1..=10 {
        print!("{n}. ");
        let result = socket.send_to(ping, &dest_addr);
        match result {
            Ok(_) => print!("sent PING... "),
            Err(_) => print!("couldn't send data")
        }

        let dur = Duration::from_millis(50);
        thread::sleep(dur);

        let mut buf = [0; 10];
        let result = socket.recv_from(&mut buf);
        match result {
            Ok((amt, _)) => {
                let vec = Vec::from(&mut buf[..amt]);
                let message = String::from_utf8(vec).expect("not a valid UTF-8 string");
                println!("received {message}");
            },
            Err(_) => println!("timed out"),
        }

    }

    Ok(())
}

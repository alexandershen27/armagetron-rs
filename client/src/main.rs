
use std::io;
use std::io::prelude::*;
use std::net::TcpStream;

use std::thread::sleep;
use std::time::Duration;

pub fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let uid = buf.as_bytes()[0];

    let mut stream = TcpStream::connect("127.0.0.1:8000")
        .expect("Failed connect");

    // Send UID byte
    stream.write(&[uid]).unwrap();

    // Connected confirmation
    let mut buf = [0; 9];
    stream.read_exact(&mut buf).expect("Failed receive.");
    let msg = String::from_utf8_lossy(&buf);
    println!("{}", msg);

    sleep(Duration::from_secs(120));
}
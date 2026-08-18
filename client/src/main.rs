use render::TerminalRenderer;
use sim::game::Game;

use std::io;
use std::io::{BufReader, prelude::*};
use std::net::TcpStream;
pub fn main() {
    println!("Enter UID (one byte char): ");

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Invalid input.");
    let uid = buf.as_bytes()[0];

    let mut stream =
        TcpStream::connect("127.0.0.1:8000").expect("Failed connect, is the server running?");

    // Send UID byte
    stream.write_all(&[uid]).unwrap();

    // Render game
    let renderer = TerminalRenderer::new();
    let mut reader = BufReader::new(&stream);
    loop {
        let mut line = String::new();
        let res = reader.read_line(&mut line).expect("Read failed");
        if res == 0 {
            println!("Server disconnected.");
            return;
        }

        let snap: Game = serde_json::from_str(&line).expect("Failed deserialize");
        renderer.draw_frame(&snap);
    }
}

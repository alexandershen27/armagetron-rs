use render::TerminalRenderer;
use sim::game::{Command, Direction, Game, Input};

use crossterm::{
    event::{Event, KeyCode, read as read_term},
    terminal::enable_raw_mode,
};

use std::io::{BufReader, prelude::*, stdin};
use std::net::TcpStream;
use std::thread;

pub fn main() {
    println!("Enter UID (one byte char): ");

    let mut buf = String::new();
    stdin().read_line(&mut buf).expect("Invalid input.");
    let uid = buf.as_bytes()[0];

    let mut stream =
        TcpStream::connect("127.0.0.1:8000").expect("Failed connect, is the server running?");

    // Send UID byte
    stream.write_all(&[uid]).unwrap();

    // Write Inputs to server
    let mut writer = stream
        .try_clone()
        .expect("Failed to initialize write stream");

    thread::spawn(move || {
        enable_raw_mode().expect("Failed to enable terminal raw mode");
        loop {
            let command = loop {
                break match read_term().expect("Failed read event") {
                    Event::Key(key) => match key.code {
                        KeyCode::Left => Command::Direction(Direction::Left),
                        KeyCode::Right => Command::Direction(Direction::Right),
                        _ => {
                            println!("Invalid key press");
                            continue;
                        }
                    },
                    _ => {
                        println!("Invald event");
                        continue;
                    }
                };
            };

            println!("Caught event!");
            let input = Input {
                uid: uid as u32,
                command,
            };

            if let Err(e) = serde_json::to_writer(&writer, &input) {
                println!("{e} while sending input.");
            }

            if let Err(e) = writer.write_all(b"\n") {
                println!("{e} while sending input.");
            }
        }
    });

    // Read data from server
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

use sim::game::{Game, GameConfig, GridConfig, Input, PlayerUid};

use std::collections::HashMap;
use std::io::BufReader;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

const ADDRESS: &str = "127.0.0.1:8000";
const TICKRATE: u32 = 10;

pub fn main() {
    // Get connections
    println!("Launching server...");
    let mut connections: HashMap<PlayerUid, TcpStream> = HashMap::new();
    let (new_connection_tx, new_connection_rx) = mpsc::channel();
    thread::spawn(move || {
        let listener = TcpListener::bind(ADDRESS).expect("Failed to bind to address");

        for stream in listener.incoming() {
            let stream = match stream {
                Ok(s) => s,
                Err(_) => continue,
            };

            new_connection_tx
                .send(stream)
                .expect("Failed to send stream down channel");
        }
    });

    // Start game loop
    println!("Building game...");
    let config = GameConfig {
        grid_config: GridConfig {
            max_x: 25.0,
            max_y: 25.0,
            turn_cooldown: 2,
        },
    };

    let mut my_game = Game::new(config);

    loop {
        // State 1: Lobby
        println!("Waiting for players...");
        thread::sleep(Duration::from_secs(5));
        while let Ok(mut stream) = new_connection_rx.try_recv() {
            let mut uid = [0; 1];
            match stream.read_exact(&mut uid) {
                Ok(_) => {
                    let uid = uid[0] as PlayerUid;
                    my_game.join(uid);
                    connections.insert(uid, stream);
                    println!("Player '{}' joined.", uid);
                }
                Err(_) => continue,
            }
        }

        let n_players = my_game.active_player_count();
        if n_players < 1 {
            continue;
        }

        // State 2: Simulate game
        println!("Starting game with {} players...", n_players);
        my_game.spawn_active_players();

        let tickrate = TICKRATE;
        let ns_per_frame = Duration::from_secs(1)
            .checked_div(tickrate)
            .expect("Nonzero tickrate");

        let (reader_tx, reader_rx) = mpsc::channel();
        for (uid, stream) in connections.iter() {
            let mut reader = BufReader::new(
                stream
                    .try_clone()
                    .expect("Failed to initialize reader stream"),
            );
            let uid = *uid;
            let reader_tx_clone = reader_tx.clone();
            thread::spawn(move || {
                loop {
                    let mut line = String::new();
                    let res = reader.read_line(&mut line).expect("Read failed");
                    if res == 0 {
                        println!("Client {uid} disconnected.");
                        return;
                    }

                    let input: Input =
                        serde_json::from_str(&line).expect("Failed deserialize input.");
                    reader_tx_clone
                        .send(input)
                        .expect("Failed to send input down channel");
                }
            });
        }

        while reader_rx.try_recv().is_ok() {
            // drain inputs before next round
            continue;
        }

        while my_game.is_active() {
            let now = Instant::now();

            let mut inputs: Vec<Input> = vec![];
            while let Ok(input) = reader_rx.try_recv() {
                inputs.push(input)
            }
            my_game.tick(&inputs);

            let mut dropped: Vec<PlayerUid> = vec![];

            for (uid, stream) in connections.iter_mut() {
                if let Err(e) = serde_json::to_writer(&mut *stream, &my_game) {
                    println!("{e} for {uid}");
                    dropped.push(*uid);
                    continue;
                }
                if let Err(e) = stream.write_all(b"\n") {
                    println!("{e} for {uid}");
                    dropped.push(*uid);
                    continue;
                }
            }

            for uid in dropped {
                connections.remove(&uid);
                my_game.leave(uid);
            }

            thread::sleep(ns_per_frame.saturating_sub(now.elapsed()));
        }
        println!("Game ended.");
    }
}

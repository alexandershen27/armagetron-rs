use sim::game::{Game, Input, PlayerUid};

use std::collections::HashMap;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

const ADDRESS: &str = "127.0.0.1:8000";
const TICKRATE: u32 = 10;

pub fn main() {
    println!("Launching server...");

    let mut connections: HashMap<PlayerUid, TcpStream> = HashMap::new();
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let listener = TcpListener::bind(ADDRESS).expect("Failed to bind to address");

        for stream in listener.incoming() {
            let stream = match stream {
                Ok(s) => s,
                Err(_) => continue,
            };

            tx.send(stream).expect("Failed to send stream down channel");
        }
    });

    println!("Building game...");
    let mut my_game = Game::new(30.0);

    loop {
        println!("Waiting for players...");
        thread::sleep(Duration::from_secs(5));
        while let Ok(mut stream) = rx.try_recv() {
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

        // Simulate game
        println!("Starting game with {} players...", n_players);
        my_game.spawn_active_players();
        let inputs: Vec<Input> = vec![];

        let tickrate = TICKRATE;
        let ns_per_frame = Duration::from_secs(1)
            .checked_div(tickrate)
            .expect("Nonzero tickrate");

        while my_game.is_active() {
            let now = Instant::now();
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

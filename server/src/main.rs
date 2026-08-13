use sim::game::{Game, PlayerInput};
use sim::player::{Player, PlayerUid};

use std::io::prelude::*;
use std::net::{TcpListener};
use std::sync::mpsc;
use std::thread;
use std::time::{Instant, Duration};

pub fn main() {

    println!("Launching server...");

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let listener = TcpListener::bind("127.0.0.1:8000")
            .expect("Failed to bind server");

        for stream in listener.incoming() {
            let mut stream = match stream {
                Ok(s) => s,
                Err(_) => continue,
            };

            let mut uid = [0; 1];
            match stream.read_exact(&mut uid) {
                Ok(a) => a,
                Err(_) => continue,
            }

            let uid = uid[0] as PlayerUid;

            stream.write(b"Connected").unwrap();
            tx.send(uid).expect("Failed to send UID down channel");
        }
    });
    
    println!("Building game...");
    let mut my_game = Game::new(30.0);

    loop {
        
        println!("Waiting for players...");
        thread::sleep(Duration::from_secs(10));
        while let Ok(uid) = rx.try_recv() {
            let new_player = Player::new(uid);
            println!("Player '{}' joined.", uid);
            my_game.join(new_player);
        }

        let n_players = my_game.active_player_count();
        if n_players < 1 {
            continue
        }

        // Simulate game
        println!("Starting game with {} players...", n_players);
        my_game.spawn_active_players();
        let inputs: Vec<PlayerInput> = vec![];

        let tickrate = 10;
        let ns_per_frame = Duration::from_secs(1).checked_div(tickrate).unwrap();

        while my_game.is_active() {
            let now = Instant::now();
            my_game.tick(&inputs);
            thread::sleep(ns_per_frame - now.elapsed());
        }
        println!("Game ended.");
    }
    

}
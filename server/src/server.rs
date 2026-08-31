use sim::game::{Game, GameConfig, Input, PlayerUid, Tick};

use serde::Serialize;

use std::collections::HashMap;
use std::io::{BufReader, prelude::*};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

pub struct ServerConfig {
    pub address: SocketAddr,
    pub tickrate: Tick,
    pub game_config: GameConfig,
}

pub struct Server {
    address: SocketAddr,
    ns_per_frame: Duration,
    writers: HashMap<PlayerUid, TcpStream>, // Writes to clients
    game: Game,
    join_tx: mpsc::Sender<TcpStream>,   // New connection upstream
    join_rx: mpsc::Receiver<TcpStream>, // New connection downstream
    input_tx: mpsc::Sender<Input>,      // Input upstream, clone per thread
    input_rx: mpsc::Receiver<Input>,    // Input downstream, collects all inputs
}

impl Server {
    pub fn new(config: ServerConfig) -> Self {
        let ns_per_frame = Duration::from_secs(1)
            .checked_div(config.tickrate as u32)
            .expect("Nonzero tickrate");
        let (join_tx, join_rx) = mpsc::channel();
        let (input_tx, input_rx) = mpsc::channel();

        Server {
            address: config.address,
            ns_per_frame,
            writers: HashMap::new(),
            game: Game::new(config.game_config),
            join_tx,
            join_rx,
            input_tx,
            input_rx,
        }
    }

    /// Bind to socket, spawn thread to accept connections, manage stages
    pub fn run(&mut self) {
        let listener = TcpListener::bind(self.address).expect("Failed to bind to address");
        let join_tx_clone = self.join_tx.clone();
        thread::spawn(move || {
            for stream in listener.incoming().flatten() {
                join_tx_clone
                    .send(stream)
                    .expect("Failed to send new join downstream");
            }
        });

        println!("Server started at {}", self.address);

        loop {
            self.stage_lobby();
            self.stage_play();
        }
    }

    /// Handle incoming connections and inputs
    fn handle_events(&mut self) -> Vec<Input> {
        // Gather new joins
        while let Ok(stream) = self.join_rx.try_recv() {
            self.add_client(stream);
        }

        // Gather input
        let mut inputs: Vec<Input> = vec![];
        while let Ok(input) = self.input_rx.try_recv() {
            inputs.push(input)
        }
        inputs
    }

    pub fn stage_lobby(&mut self) {
        println!("Waiting for players");
        loop {
            self.handle_events();
            if self.game.active_player_count() >= 1 {
                break;
            }
            thread::sleep(Duration::from_secs(1));
        }
    }

    pub fn stage_play(&mut self) {
        println!(
            "Starting game with {} players...",
            self.game.active_player_count(),
        );

        self.game.start();

        while self.game.is_active() {
            let now = Instant::now();
            let inputs = self.handle_events();
            self.game.tick(&inputs);
            let dropped = Self::broadcast(&mut self.writers, &self.game);
            for client in dropped {
                self.remove_client(client);
            }
            thread::sleep(self.ns_per_frame.saturating_sub(now.elapsed()));
        }
    }

    /// Spawn thread for reader stream, add new connection
    fn add_client(&mut self, mut stream: TcpStream) {
        let writer = stream.try_clone().expect("Failed to clone stream");
        let input_tx_clone = self.input_tx.clone();

        // Receive UID
        let mut uid = [0; 1];
        stream.read_exact(&mut uid).expect("Failed read UID");
        let uid = uid[0] as u32;

        // Add player and streams to server
        self.game.join(uid);
        self.writers.insert(uid, writer);

        thread::spawn(move || {
            // Spawn thread to handle inputs
            let mut bufreader = BufReader::new(stream);
            loop {
                let mut line = String::new();
                let res = bufreader.read_line(&mut line).expect("Read failed");
                if res == 0 {
                    return;
                }

                let input: Input = serde_json::from_str(&line).expect("Failed deserialize input.");
                input_tx_clone
                    .send(input)
                    .expect("Failed to send input downstream");
            }
        });
    }

    fn remove_client(&mut self, uid: PlayerUid) {
        self.writers.remove(&uid);
        self.game.leave(uid);
    }

    fn broadcast<T: Serialize>(
        writers: &mut HashMap<PlayerUid, TcpStream>,
        message: &T,
    ) -> Vec<PlayerUid> {
        let mut dropped: Vec<PlayerUid> = vec![];

        for (uid, stream) in writers.iter_mut() {
            if let Err(e) = net::send(&mut *stream, &message) {
                println!("{e} for {uid}");
                dropped.push(*uid);
                continue;
            }
        }
        dropped
    }
}

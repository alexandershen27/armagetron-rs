

use std::io;
use std::time::{Duration, Instant};
use std::thread::sleep;

use sim::Scalar;
use sim::game::{Game, PlayerInput};
use sim::player::Player;

use render::TerminalRenderer;

pub fn main() {

    // Get tickrate (future "server" struct)
    println!("Input tickrate: ");
    let mut tickrate = String::new();
    io::stdin().read_line(&mut tickrate).expect("Fail");
    let tickrate: u32 = tickrate.trim().parse().expect("Fail");
    let ns_per_frame = Duration::from_secs(1).checked_div(tickrate).unwrap();

    // Build game/grid
    println!("Input grid bound: ");
    let mut grid_size = String::new();
    io::stdin().read_line(&mut grid_size).expect("Fail");
    let grid_size: Scalar = grid_size.trim().parse().expect("Fail");
    let mut my_game = Game::new(grid_size);

    // Join players
    println!("Input number of players (<=26): ");
    let mut n_players = String::new();
    io::stdin().read_line(&mut n_players).expect("Fail");
    let n_players: usize = n_players.trim().parse().expect("Fail");
    
    for i in 0..n_players {
        let id = (i + 65) as u32;

        let new_player = Player {
            uid: id,
            cycle_id: None,
        };

        my_game.join(new_player);
        my_game.spawn_cycle_random_for(id);
    }

    // Start simulation
    #[allow(unused_mut)]
    let mut inputs: Vec<PlayerInput> = Vec::new();
    let renderer = TerminalRenderer::new();

    while my_game.is_active() {
        let now = Instant::now();
        renderer.draw_frame(&my_game);

        // [fetching inputs would happen here]
        my_game.tick(&inputs);
        sleep(ns_per_frame - now.elapsed());
    }
}
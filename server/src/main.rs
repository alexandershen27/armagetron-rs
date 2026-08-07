

use std::io;
use std::time::{Duration, SystemTime};
use std::thread::sleep;

use sim::{Cardinal, Coordinate};
use sim::arena::Arena;
use sim::cycle::{Cycle};

pub fn main() {

    // Get tickrate
    println!("Input tickrate: ");
    let mut tickrate = String::new();
    io::stdin().read_line(&mut tickrate).expect("Fail");
    let tickrate: u32 = tickrate.trim().parse().expect("Fail");
    let ns_per_frame = Duration::from_secs(1).checked_div(tickrate).unwrap();

    // Build arena
    println!("Input arena bound: ");
    let mut arena_size = String::new();
    io::stdin().read_line(&mut arena_size).expect("Fail");
    let arena_size: i32 = arena_size.trim().parse().expect("Fail");
    let mut my_arena = Arena::new(arena_size, arena_size);

    // Spawn cycles
    println!("Input number of cycles (<=26): ");
    let mut n_cycles = String::new();
    io::stdin().read_line(&mut n_cycles).expect("Fail");
    let n_cycles: i32 = n_cycles.trim().parse().expect("Fail");
    
    for i in 0..n_cycles {
        let mut pass = false;

        while !pass {
            let id = ((i + 65) as u8) as char;
            println!("Position/facing for cycle {} (x y d)", id);
            let mut pos = String::new();
            io::stdin().read_line(&mut pos).expect("Fail");
            let pos_split: Vec<&str> = pos.split(' ').collect();
            let x: i32 = pos_split[0].trim().parse().expect("Fail");
            let y: i32 = pos_split[1].trim().parse().expect("Fail");
            let d: char = pos_split[2].trim().parse().expect("Fail");

            if x.abs() >= arena_size || y.abs() >= arena_size {
                println!("Invalid");
                continue
            } 
            
            let f = match d {
                'n' => Cardinal::North,
                's' => Cardinal::South,
                'e' => Cardinal::East,
                'w' => Cardinal::West,
                _ => Cardinal::North,
            };
            my_arena.add_cycle(Cycle::new(Coordinate{x, y}, f));
            pass = true;
        }
    }

    // Start simulation
    while my_arena.living_count() > 0 {
        let now = SystemTime::now();
        my_arena.tick();
        sleep(ns_per_frame - now.elapsed().unwrap());
    }
}
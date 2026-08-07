
use std::collections::VecDeque;
use crate::{Cardinal, Coordinate};
use crate::wall::Wall;


#[derive(PartialEq)]
pub enum Direction { Left, Right }

pub struct Cycle {
    alive: bool,
    position: Coordinate,
    facing: Cardinal,
    speed: i32,
    walls: Vec<Wall>,
    queued_turns: VecDeque<Direction>
}

// Accessors and settors
impl Cycle {
    pub fn new(position: Coordinate, facing: Cardinal) -> Self {
        Cycle {
            alive: true,
            position,
            facing,
            speed: 1,
            walls: vec![Wall::new(position)],
            queued_turns: VecDeque::new(),
        }
    }

    pub fn is_alive(&self) -> bool { self.alive }
    pub fn position(&self) -> Coordinate { self.position }
    pub fn walls(&self) -> &[Wall] { &self.walls }

    pub fn kill(&mut self) { self.alive = false; }
    pub fn queue_turn(&mut self, dir: Direction) { self.queued_turns.push_back(dir); }

    // Pre-collision actions
    pub fn advance(&mut self) {
        if !self.is_alive() { return }

        self.try_turn();
        self.position.step(self.facing, self.speed);
    }

    // Post-collision actions
    pub fn update(&mut self) {
        if !self.is_alive() { return }

        self.update_wall();
    }

}

impl Cycle {

    fn try_turn(&mut self) {
        if let Some(dir) = self.queued_turns.pop_front() {

            self.facing = match (&self.facing, dir) {
                (Cardinal::North, Direction::Left)  => Cardinal::West,
                (Cardinal::North, Direction::Right) => Cardinal::East,
                (Cardinal::East,  Direction::Left)  => Cardinal::North,
                (Cardinal::East,  Direction::Right) => Cardinal::South,
                (Cardinal::South, Direction::Left)  => Cardinal::East,
                (Cardinal::South, Direction::Right) => Cardinal::West,
                (Cardinal::West,  Direction::Left)  => Cardinal::South,
                (Cardinal::West,  Direction::Right) => Cardinal::North,
            };

            self.walls.push(Wall::new(self.position));
        }
    }

    fn update_wall(&mut self) {
        if let Some(current_wall) = self.walls.last_mut() {
            current_wall.update_end_position(self.position);
        }
    }

}
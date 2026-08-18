use super::wall::Wall;
use crate::game::{Command, Direction};
use crate::{Cardinal, Coordinate, Scalar};
use std::collections::VecDeque;

pub struct Cycle {
    alive: bool,
    position: Coordinate,
    facing: Cardinal,
    speed: Scalar,
    walls: Vec<Wall>,
    queued_turns: VecDeque<Direction>,
}

// Accessors and settors
impl Cycle {
    pub fn new(position: Coordinate, facing: Cardinal, speed: Scalar) -> Self {
        Cycle {
            alive: true,
            position,
            facing,
            speed,
            walls: vec![Wall::new(position)],
            queued_turns: VecDeque::new(),
        }
    }

    pub fn is_alive(&self) -> bool {
        self.alive
    }
    pub fn position(&self) -> Coordinate {
        self.position
    }
    pub fn facing(&self) -> Cardinal {
        self.facing
    }
    pub fn speed(&self) -> Scalar {
        self.speed
    }
    pub fn walls(&self) -> &[Wall] {
        &self.walls
    }

    pub fn kill(&mut self) {
        self.alive = false;
    }
    pub fn set_position(&mut self, pos: Coordinate) {
        self.position = pos;
        self.update_wall();
    }

    // Pre-collision action
    pub fn act(&mut self, command: Command) {
        match command {
            Command::Direction(dir) => {
                self.queued_turns.push_back(dir);
                self.try_turn();
            }
        }
    }

    // Post-collision update
    pub fn update(&mut self) {
        self.position.step(self.facing, self.speed);
        self.update_wall();
    }
}

impl Cycle {
    fn try_turn(&mut self) {
        if let Some(dir) = self.queued_turns.pop_front() {
            self.facing = match (&self.facing, dir) {
                (Cardinal::North, Direction::Left) => Cardinal::West,
                (Cardinal::North, Direction::Right) => Cardinal::East,
                (Cardinal::East, Direction::Left) => Cardinal::North,
                (Cardinal::East, Direction::Right) => Cardinal::South,
                (Cardinal::South, Direction::Left) => Cardinal::East,
                (Cardinal::South, Direction::Right) => Cardinal::West,
                (Cardinal::West, Direction::Left) => Cardinal::South,
                (Cardinal::West, Direction::Right) => Cardinal::North,
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

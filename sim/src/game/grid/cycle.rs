use std::collections::VecDeque;

use super::GridConfig;
use super::wall::Wall;
use crate::game::{Cardinal, Command, Coordinate, Direction, Scalar, Tick};

pub struct Cycle {
    config: GridConfig,
    alive: bool,
    position: Coordinate,
    facing: Cardinal,
    speed: Scalar,
    walls: Vec<Wall>,
    turn_cooldown: Tick,
    queued_turns: VecDeque<Direction>,
}

// Accessors and settors
impl Cycle {
    pub fn new(config: GridConfig, position: Coordinate, facing: Cardinal, speed: Scalar) -> Self {
        Cycle {
            config,
            alive: true,
            position,
            facing,
            speed,
            walls: vec![Wall::new(position)],
            turn_cooldown: 0,
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
            Command::Direction(dir) => self.queued_turns.push_back(dir),
        }
    }

    // Post-collision update
    pub fn update(&mut self) {
        self.try_turn();
        self.position.step(self.facing, self.speed);
        self.update_wall();
    }
}

impl Cycle {
    fn try_turn(&mut self) {
        if self.turn_cooldown > 0 {
            self.turn_cooldown -= 1;
            return;
        }

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
            self.turn_cooldown = self.config.turn_cooldown;
        }
    }

    fn update_wall(&mut self) {
        if let Some(current_wall) = self.walls.last_mut() {
            current_wall.update_end_position(self.position);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn turn_delay() {
        let mut c = Cycle::new(
            GridConfig {
                max_x: 5.0,
                max_y: 5.0,
                turn_cooldown: 5,
            },
            Coordinate { x: 0.0, y: 0.0 },
            Cardinal::North,
            1.0,
        );

        c.act(Command::Direction(Direction::Left));
        c.act(Command::Direction(Direction::Left));

        c.update();
        c.update();
        assert_eq!(c.facing, Cardinal::West);

        for _ in 0..c.config.turn_cooldown - 1 {
            c.update();
            assert_eq!(c.facing, Cardinal::West);
        }

        c.update();
        assert_eq!(c.facing, Cardinal::South);
    }
}

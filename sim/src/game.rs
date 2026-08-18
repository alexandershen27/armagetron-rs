mod grid;
mod player;

use self::grid::Grid;
pub use self::player::PlayerUid;
use self::player::{Player, PlayerStatus};
use crate::game::grid::GridConfig;

use rand::{random_bool, random_range};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub enum Cardinal {
    North,
    East,
    South,
    West,
}

pub type Scalar = f32;
pub type Tick = i32;

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Coordinate {
    pub x: Scalar,
    pub y: Scalar,
}

impl Coordinate {
    fn step(&mut self, dir: Cardinal, speed: Scalar) {
        match dir {
            Cardinal::North => self.y += speed,
            Cardinal::South => self.y -= speed,
            Cardinal::East => self.x += speed,
            Cardinal::West => self.x -= speed,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Line {
    pos1: Coordinate,
    pos2: Coordinate,
}

impl Line {
    pub fn new(pos1: Coordinate, pos2: Coordinate) -> Self {
        if pos1.x != pos2.x && pos1.y != pos2.y {
            panic!("Line must be axis-aligned")
        }

        Line { pos1, pos2 }
    }

    pub fn len(&self) -> Scalar {
        if self.is_horizontal() {
            return (self.pos2.x - self.pos1.x).abs();
        }
        if self.is_vertical() {
            return (self.pos2.y - self.pos1.y).abs();
        }
        unreachable!("Should be axis-aligned")
    }

    pub fn hit(&self, other: Line) -> bool {
        if self.is_horizontal() && other.is_vertical() {
            let self_min_x = self.pos1.x.min(self.pos2.x);
            let self_max_x = self.pos1.x.max(self.pos2.x);

            let other_min_y = other.pos1.y.min(other.pos2.y);
            let other_max_y = other.pos1.y.max(other.pos2.y);

            return other.pos1.x >= self_min_x
                && other.pos1.x <= self_max_x
                && self.pos1.y >= other_min_y
                && self.pos1.y <= other_max_y;
        }

        if self.is_vertical() && other.is_horizontal() {
            let self_min_y = self.pos1.y.min(self.pos2.y);
            let self_max_y = self.pos1.y.max(self.pos2.y);

            let other_min_x = other.pos1.x.min(other.pos2.x);
            let other_max_x = other.pos1.x.max(other.pos2.x);

            return self.pos1.x >= other_min_x
                && self.pos1.x <= other_max_x
                && other.pos1.y >= self_min_y
                && other.pos1.y <= self_max_y;
        }

        false
    }
}

impl Line {
    fn is_horizontal(&self) -> bool {
        self.pos1.y == self.pos2.y
    }

    fn is_vertical(&self) -> bool {
        self.pos1.x == self.pos2.x
    }
}

pub struct Input {
    pub uid: PlayerUid,
    pub command: Command,
}

#[derive(PartialEq, Clone, Copy)]
pub enum Command {
    Direction(Direction),
}

#[derive(PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Serialize, Deserialize)]
pub struct GameSnapshot {
    game: Game,
}

#[derive(Serialize, Deserialize)]
pub struct Game {
    grid: Grid,
    players: HashMap<PlayerUid, Player>,
}

// Initializer, public getters and setters
impl Game {
    pub fn new(grid_size: Scalar) -> Self {
        let config = GridConfig {
            max_x: grid_size,
            max_y: grid_size,
            turn_cooldown: 5,
        };

        Game {
            grid: Grid::new(config),
            players: HashMap::new(),
        }
    }

    fn active_players(&self) -> Vec<PlayerUid> {
        self.players
            .values()
            .filter(|p| p.status == PlayerStatus::Active)
            .map(|p| p.uid)
            .collect()
    }

    pub fn active_player_count(&self) -> usize {
        self.active_players().len()
    }

    pub fn is_active(&self) -> bool {
        self.grid.living_count() >= 1
    }

    pub fn join(&mut self, uid: PlayerUid) {
        self.players.insert(uid, Player::new(uid));
    }

    pub fn leave(&mut self, uid: PlayerUid) {
        self.players
            .get_mut(&uid)
            .expect("Player should be in game")
            .status = PlayerStatus::Inactive
    }
}

// Cycle spawning
impl Game {
    pub fn spawn_active_players(&mut self) {
        let active_players = self.active_players();
        for uid in active_players {
            self.spawn_cycle_random_for(uid);
        }
    }

    #[allow(unused)]
    fn spawn_cycle_for(&mut self, player_uid: PlayerUid, position: Coordinate, facing: Cardinal) {
        self.grid.spawn_cycle(player_uid, position, facing);
    }

    fn spawn_cycle_random_for(&mut self, player_uid: PlayerUid) {
        let grid = self.grid();
        let radius = grid.max_x().min(grid.max_y()) * 0.7;
        let x = random_range(-radius..radius);
        let y = (radius.powi(2) - x.powi(2)).sqrt() * if random_bool(0.5) { 1.0 } else { -1.0 };

        let position = Coordinate { x, y };
        let facing = if x.abs() > y.abs() {
            if x > 0.0 {
                Cardinal::West
            } else {
                Cardinal::East
            }
        } else {
            if y > 0.0 {
                Cardinal::South
            } else {
                Cardinal::North
            }
        };

        self.grid.spawn_cycle(player_uid, position, facing);
    }

    pub fn tick(&mut self, inputs: &[Input]) {
        self.grid.tick(inputs)
    }

    pub fn grid(&self) -> &Grid {
        &self.grid
    }
}

mod grid;
mod player;

use self::grid::Grid;
use self::player::Player;
use crate::{Cardinal, Coordinate, Scalar};

pub use self::player::PlayerUid;

use rand::{random_bool, random_range};

pub struct Input {
    pub uid: PlayerUid,
    pub command: Command,
}

#[derive(PartialEq, Clone, Copy)]
pub enum Command {
    Direction(Direction),
}

#[derive(PartialEq, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
}

pub struct Game {
    grid: Grid,
    players: Vec<Player>,
}

// Initializer, public getters and setters
impl Game {
    pub fn new(grid_size: Scalar) -> Self {
        Game {
            grid: Grid::new(grid_size, grid_size),
            players: vec![],
        }
    }

    fn active_uids(&self) -> Vec<PlayerUid> {
        self.players
            .iter()
            .filter(|p| p.is_active())
            .map(|p| p.uid())
            .collect()
    }

    pub fn active_player_count(&self) -> usize {
        self.active_uids().len()
    }

    pub fn is_active(&self) -> bool {
        self.grid.living_count() >= 1
    }

    pub fn join(&mut self, uid: PlayerUid) {
        self.players.push(Player::new(uid));
    }
}

// Cycle spawning
impl Game {
    pub fn spawn_active_players(&mut self) {
        let active_uids = self.active_uids();
        for uid in active_uids {
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

use crate::{Scalar, Coordinate, Cardinal};
use crate::grid::{Grid, SimInput};
use crate::cycle::Command;
use crate::player::{Player, PlayerUid};

use rand::{random_bool, random_range};

pub struct PlayerInput {
    pub player_uid: PlayerUid,
    pub command: Command
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
            .map(|p| p.get_uid())
            .collect()
    }

    pub fn active_player_count(&self) -> usize {
        self.active_uids().len()
    }

    pub fn is_active(&self) -> bool {
        self.grid.living_count() >= 1
    }

    pub fn join(&mut self, player: Player) {
        self.players.push(player);
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

    fn spawn_cycle_for(&mut self, player_uid: PlayerUid, position: Coordinate, facing: Cardinal) {
        let cycle_id = self.grid.spawn_cycle(position, facing);
        self.get_player_mut(player_uid).set_cycle_id(cycle_id);
    }

    fn spawn_cycle_random_for(&mut self, player_uid: PlayerUid) {
        
        let grid = self.get_grid();
        let radius = grid.max_x().min(grid.max_y()) * 0.7;
        let x = random_range(-radius..radius);
        let y = (radius.powi(2) - x.powi(2)).sqrt() * if random_bool(0.5) { 1.0 } else { -1.0 };

        let position = Coordinate { x, y };
        let facing = if x.abs() > y.abs() {
            if x > 0.0 { Cardinal::West } else { Cardinal::East }
        } else {
            if y > 0.0 { Cardinal::South } else { Cardinal::North }
        };

        let cycle_id = self.grid.spawn_cycle(position, facing);
        self.get_player_mut(player_uid).set_cycle_id(cycle_id);

    }

    pub fn tick(&mut self, inputs: &[PlayerInput]) {
        let mut sim_input = vec![];
        for player_input in inputs {
            sim_input.push(SimInput {
                cycle_id: self.get_player(player_input.player_uid).get_cycle_id()
                    .expect("Player should have cycle_id"),
                command: player_input.command,
            })
        }
        self.grid.tick(&sim_input)
    }

    pub fn get_grid(&self) -> &Grid {
        &self.grid
    }
}

impl Game {

    fn get_player(&self, player_uid: PlayerUid) -> &Player {
        self.players.iter().find(|x| x.get_uid() == player_uid)
            .expect("Player should be in game")
    }

    fn get_player_mut(&mut self, player_uid: PlayerUid) -> &mut Player {
        self.players.iter_mut().find(|x| x.get_uid() == player_uid)
            .expect("Player should be in game")
    }

}
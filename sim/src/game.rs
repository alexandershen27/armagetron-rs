use crate::{Coordinate, Cardinal};
use crate::arena::{Arena, SimInput};
use crate::cycle::{CycleId, Command};
use crate::player::{Player, PlayerUid};

pub struct PlayerInput {
    pub player_uid: PlayerUid,
    pub command: Command
}

pub struct Game {
    arena: Arena,
    players: Vec<Player>,
}

impl Game {
    pub fn new(arena_size: i32) -> Self {
        Game {
            arena: Arena::new(arena_size, arena_size),
            players: vec![],
        }
    }

    pub fn is_active(&self) -> bool {
        self.arena.living_count() >= 1
    }

    pub fn join(&mut self, player: Player) {
        self.players.push(player);
    }

    pub fn spawn_cycle_for(&mut self, player_uid: PlayerUid, position: Coordinate, facing: Cardinal) {
        let cycle_id = self.arena.spawn_cycle(position, facing);
        self.get_player_mut(player_uid).cycle_id = Some(cycle_id);
    }

    pub fn tick(&mut self, inputs: &[PlayerInput]) {
        let mut sim_input = vec![];
        for player_input in inputs {
            sim_input.push(SimInput {
                cycle_id: self.get_cycle_id(player_input.player_uid).expect("Player should have cycle_id"),
                command: player_input.command,
            })
        }
        self.arena.tick(&sim_input)
    }
}

impl Game {

    fn get_player(&self, player_uid: PlayerUid) -> &Player {
        self.players.iter().find(|x| x.uid == player_uid).expect("Player should be in game")
    }

    fn get_player_mut(&mut self, player_uid: PlayerUid) -> &mut Player {
        self.players.iter_mut().find(|x| x.uid == player_uid).expect("Player should be in game")
    }

    fn get_cycle_id(&self, player_uid: PlayerUid) -> Option<CycleId> {
        self.get_player(player_uid).cycle_id
    }

}
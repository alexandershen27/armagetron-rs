mod cycle;
mod sensor;
mod wall;

use std::collections::HashMap;

use self::cycle::Cycle;
use self::sensor::Sensor;
use super::{Cardinal, Coordinate, Input, PlayerUid, Scalar, Tick};

pub struct Grid {
    config: GridConfig,
    cycles: HashMap<PlayerUid, Cycle>,
}

// Grid initialization, and config
#[derive(Clone, Copy)]
pub struct GridConfig {
    pub max_x: Scalar,
    pub max_y: Scalar,
    pub turn_cooldown: Tick,
}

impl Grid {
    pub fn new(config: GridConfig) -> Self {
        Grid {
            config,
            cycles: HashMap::new(),
        }
    }

    pub fn max_x(&self) -> Scalar {
        self.config.max_x
    }
    pub fn max_y(&self) -> Scalar {
        self.config.max_y
    }

    pub fn cycles(&self) -> Vec<(&PlayerUid, &Cycle)> {
        self.cycles.iter().collect()
    }

    pub fn living_count(&self) -> usize {
        self.cycles.values().filter(|c| c.is_alive()).count()
    }

    pub fn spawn_cycle(&mut self, player: PlayerUid, position: Coordinate, facing: Cardinal) {
        let cycle = Cycle::new(self.config, position, facing, 1.0);
        self.cycles.insert(player, cycle);
    }

    pub fn tick(&mut self, inputs: &[Input]) {
        // Phase 1: Does inputs
        for input in inputs {
            self.cycles
                .get_mut(&input.uid)
                .expect("Player should have cycle")
                .act(input.command);
        }

        // Phase 2: Check collisions in next tick
        let mut dead_cycles = Vec::new();
        for (uid, cycle) in self.cycles.iter() {
            if !cycle.is_alive() {
                continue;
            }
            let sensor = Sensor::new(self, *uid, cycle).ray_hit();
            if sensor.ticks_to_collision <= 1 {
                dead_cycles.push((*uid, sensor));
            }
        }

        // Phase 3: Kill and advance cycles
        for (uid, sensor) in dead_cycles {
            self.cycles
                .get_mut(&uid)
                .expect("Player should have cycle")
                .set_position(sensor.position);
            self.cycles
                .get_mut(&uid)
                .expect("Player should have cycle")
                .kill();
        }

        for cycle in self.cycles.values_mut() {
            if cycle.is_alive() {
                cycle.update();
            }
        }
    }
}

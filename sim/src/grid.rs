
use crate::{Scalar, Coordinate, Cardinal};
use crate::cycle::{Cycle, CycleId, Command};
use crate::sensor::{Sensor, SensorHit};

pub struct SimInput {
    pub cycle_id: CycleId,
    pub command: Command
}

pub struct Grid {
    max_x: Scalar,
    max_y: Scalar,
    cycles: Vec<Cycle>,
    next_id: CycleId,
}

impl Grid {
    pub fn new(max_x: Scalar, max_y: Scalar) -> Self {
        Grid {
            max_x,
            max_y,
            cycles: vec![],
            next_id: 0,
        }
    }

    pub fn max_x(&self) -> Scalar { self.max_x }
    pub fn max_y(&self) -> Scalar { self.max_y }
    pub fn cycles(&self) -> &[Cycle] { &self.cycles }

    pub fn allocate_id(&mut self) -> CycleId { 
        self.next_id += 1;
        self.next_id - 1
    }

    pub fn living_count(&self) -> usize { 
        self.cycles.iter().filter(|c| c.is_alive()).count() 
    }

    pub fn spawn_cycle(&mut self, position: Coordinate, facing: Cardinal) -> CycleId {
        let new_id = self.allocate_id();
        let cycle = Cycle::new(
            new_id,
            position,
            facing,
            1.0,
        );
        self.cycles.push(cycle);
        new_id
    }

    pub fn tick(&mut self, inputs: &[SimInput]) {

        // Phase 1: Does inputs
        for input in inputs {
            self.cycles[input.cycle_id].act(input.command);
        }

        // Phase 2: Check collisions in next tick
        let mut dead_cycles: Vec<(usize, SensorHit)> = Vec::new();
        for (i, cycle) in self.cycles.iter().enumerate() {
            if !cycle.is_alive() { continue; }
            let sensor = Sensor::new(&self, cycle).ray_hit();
            if sensor.ticks_to_collision <= 1 {
                dead_cycles.push((i, sensor));
            }
        }

        // Phase 3: Kill and advance cycles
        for (index, sensor) in dead_cycles {
            self.cycles[index].set_position(sensor.position);
            self.cycles[index].kill();
        }

        for cycle in &mut self.cycles {
            if cycle.is_alive() {
                cycle.update();
            }
        }
        
    }
}

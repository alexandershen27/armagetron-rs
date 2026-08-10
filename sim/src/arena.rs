
use crate::{Coordinate, Cardinal};
use crate::cycle::{Cycle, CycleId, Command};

pub struct SimInput {
    pub cycle_id: CycleId,
    pub command: Command
}

pub struct Arena {
    max_x: i32,
    max_y: i32,
    cycles: Vec<Cycle>,
    next_id: CycleId,
}

impl Arena {
    pub fn new(max_x: i32, max_y: i32) -> Self {
        Arena {
            max_x,
            max_y,
            cycles: vec![],
            next_id: 0,
        }
    }

    pub fn max_x(&self) -> i32 { self.max_x }
    pub fn max_y(&self) -> i32 { self.max_y }
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
            1,
        );
        self.cycles.push(cycle);
        new_id
    }

    pub fn tick(&mut self, inputs: &[SimInput]) {

        // Phase 1: Queue inputs
        for input in inputs {
            self.cycles[input.cycle_id].act(input.command);
        }


        // Phase 2: Advance cycles
        for cycle in &mut self.cycles {
            if cycle.is_alive() {
                cycle.advance();
            }
        }

        // Phase 3: Check collisions
        let mut dead_indices = Vec::new();

        for (i, cycle) in self.cycles.iter().enumerate() {
            if !cycle.is_alive() { continue; }

            // Outside arena
            if self.out_of_bounds(cycle.position()) {
                dead_indices.push(i);
                continue;
            }

            // Wall collisions
            'crash: for other in &self.cycles {
                for wall in other.walls() {
                    if wall.has_point(cycle.position()) {
                        dead_indices.push(i);
                        break 'crash;
                    }
                }
            }
        }

        // Phase 4: Update state
        for index in dead_indices {
            self.cycles[index].kill();
        }

        for cycle in &mut self.cycles {
            if cycle.is_alive() {
                cycle.update();
            }
        }
    }
}

impl Arena {
    fn out_of_bounds(&self, point: Coordinate) -> bool {
        if point.x.abs() >= self.max_x || point.y.abs() >= self.max_y {
            return true;
        }
        false
    }
}
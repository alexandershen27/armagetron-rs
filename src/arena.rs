
use crate::{Coordinate};
use crate::cycle::{Cycle, Direction};

pub struct Arena {
    max_x: i32,
    max_y: i32,
    cycles: Vec<Cycle>,
}

impl Arena {
    pub fn new(max_x: i32, max_y: i32) -> Self {
        Arena {
            max_x,
            max_y,
            cycles: vec![],
        }
    }

    pub fn max_x(&self) -> i32 { self.max_x }
    pub fn max_y(&self) -> i32 { self.max_y }
    pub fn cycles(&self) -> &[Cycle] { &self.cycles }

    pub fn living_count(&self) -> usize { self.cycles.iter().filter(|c| c.is_alive()).count() }

    pub fn add_cycle(&mut self, cycle: Cycle) {
        self.cycles.push(cycle)
    }

    pub fn queue_turn(&mut self, id: usize, dir: Direction) {
        self.cycles[id].queue_turn(dir);
    }

    pub fn tick(&mut self) {

        // Phase 1: All cycles advance
        for cycle in &mut self.cycles {
            if cycle.is_alive() {
                cycle.advance();
            }
        }

        // Phase 2: Check collisions
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

        // Phase 3: Update state
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
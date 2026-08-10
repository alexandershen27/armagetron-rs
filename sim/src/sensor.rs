
use crate::{Scalar, Coordinate, Cardinal, Line};
use crate::grid::Grid;
use crate::cycle::{Cycle, CycleId};

pub struct Sensor<'a> {
    grid: &'a Grid,
    cycle_id: CycleId,
    position: Coordinate,
    facing: Cardinal,
    speed: Scalar,
}

pub struct SensorHit {
    pub ticks_to_collision: u32,
    pub target: CycleId,
    pub position: Coordinate,
}

impl<'a> Sensor<'a> {
    pub fn new(grid: &'a Grid, cycle: &Cycle) -> Self {
        Sensor {
            grid,
            cycle_id: cycle.get_id(),
            position: cycle.position(),
            facing: cycle.facing(),
            speed: cycle.speed(),
        }
    }

    fn ray_max(&self) -> Line {
        let mut x = self.position.x;
        let mut y = self.position.y;

        match self.facing {
            Cardinal::North => y = self.grid.max_y(),
            Cardinal::South => y = -self.grid.max_y(),
            Cardinal::East => x = self.grid.max_x(),
            Cardinal::West => x = -self.grid.max_x(),
        };

        let end = Coordinate { x, y }; 
        Line { pos1: self.position, pos2: end }
    }

    pub fn ray_hit(&self) -> SensorHit {
        let mut ray = self.ray_max();
        let mut target = self.cycle_id;

        for cycle in self.grid.cycles() {
            for wall in cycle.walls() {
                if cycle.get_id() == self.cycle_id && ray.pos1 == wall.end_position() { continue }
                if ray.hit(wall.as_line()) {
                    match self.facing {
                        Cardinal::North => ray.pos2.y = wall.start_position().y,
                        Cardinal::South => ray.pos2.y = wall.start_position().y,
                        Cardinal::East => ray.pos2.x = wall.start_position().x,
                        Cardinal::West => ray.pos2.x = wall.start_position().x,
                    }
                    target = cycle.get_id()
                }
            }
        }

        SensorHit {
            ticks_to_collision: (ray.len() / self.speed).ceil() as u32,
            target,
            position: ray.pos2
        }
    }

    
}
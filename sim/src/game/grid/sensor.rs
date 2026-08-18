use super::cycle::Cycle;
use super::{Grid, PlayerUid};
use crate::game::{Cardinal, Coordinate, Line, Scalar, Tick};

pub struct Sensor<'a> {
    grid: &'a Grid,
    player_uid: PlayerUid,
    position: Coordinate,
    facing: Cardinal,
    speed: Scalar,
}

pub struct SensorHit {
    pub ticks_to_collision: Tick,
    #[allow(unused)]
    pub target: PlayerUid,
    pub position: Coordinate,
}

impl<'a> Sensor<'a> {
    pub fn new(grid: &'a Grid, uid: PlayerUid, cycle: &Cycle) -> Self {
        Sensor {
            grid,
            player_uid: uid,
            position: cycle.position(),
            facing: cycle.facing(),
            speed: cycle.speed(),
        }
    }

    pub fn ray_hit(&self) -> SensorHit {
        let mut ray = self.ray_max();
        let mut target = self.player_uid;

        for (uid, cycle) in self.grid.cycles() {
            for wall in cycle.walls() {
                if *uid == self.player_uid && ray.pos1 == wall.end_position() {
                    continue;
                }
                if ray.hit(wall.as_line()) {
                    match self.facing {
                        Cardinal::North => ray.pos2.y = wall.start_position().y,
                        Cardinal::South => ray.pos2.y = wall.start_position().y,
                        Cardinal::East => ray.pos2.x = wall.start_position().x,
                        Cardinal::West => ray.pos2.x = wall.start_position().x,
                    }
                    target = *uid
                }
            }
        }

        SensorHit {
            ticks_to_collision: (ray.len() / self.speed).ceil() as Tick,
            target,
            position: ray.pos2,
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
        Line {
            pos1: self.position,
            pos2: end,
        }
    }
}

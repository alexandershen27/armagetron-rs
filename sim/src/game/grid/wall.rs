use crate::game::{Coordinate, Line};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Wall {
    start_position: Coordinate,
    end_position: Coordinate,
}

impl Wall {
    pub fn new(position: Coordinate) -> Self {
        Wall {
            start_position: position,
            end_position: position,
        }
    }
    pub fn as_line(&self) -> Line {
        Line::new(self.start_position, self.end_position)
    }
    pub fn start_position(&self) -> Coordinate {
        self.start_position
    }
    pub fn end_position(&self) -> Coordinate {
        self.end_position
    }
    pub fn update_end_position(&mut self, new_position: Coordinate) {
        if self.start_position().x != new_position.x && self.start_position().y != new_position.y {
            panic!("Wall update not axially aligned")
        }
        self.end_position = new_position;
    }
}

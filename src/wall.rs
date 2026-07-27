
use std::cmp;
use crate::Coordinate;

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
    pub fn start_position(&self) -> Coordinate { self.start_position }
    pub fn end_position(&self) -> Coordinate { self.end_position }

    pub(crate) fn update_end_position(&mut self, new_position: Coordinate) {
        self.end_position = new_position;
    }

    pub fn has_point(&self, point: Coordinate) -> bool {
        // Vertical Wall (parallel to y-axis)
        if self.start_position.x == self.end_position.x 
            && point.x == self.start_position.x {
                let min_y = cmp::min(self.start_position.y, self.end_position.y);
                let max_y = cmp::max(self.start_position.y, self.end_position.y);

                if point.y >= min_y && point.y <= max_y {
                    return true
                }
            }
        
        // Horizontal Wall (parallel to x-axis)
        if self.start_position.y == self.end_position.y
            && point.y == self.start_position.y {   
                let min_x = cmp::min(self.start_position.x, self.end_position.x);
                let max_x = cmp::max(self.start_position.x, self.end_position.x);

                if point.x >= min_x && point.x <= max_x {
                    return true
                }
            }

        false
    }
}
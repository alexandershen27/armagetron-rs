#[derive(Clone, Copy, PartialEq)]
pub enum Cardinal {
    North,
    East,
    South,
    West,
}

pub type Scalar = f32;

#[derive(Clone, Copy, PartialEq)]
pub struct Coordinate {
    pub x: Scalar,
    pub y: Scalar,
}

impl Coordinate {
    fn step(&mut self, dir: Cardinal, speed: Scalar) {
        match dir {
            Cardinal::North => self.y += speed,
            Cardinal::South => self.y -= speed,
            Cardinal::East => self.x += speed,
            Cardinal::West => self.x -= speed,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Line {
    pos1: Coordinate,
    pos2: Coordinate,
}

impl Line {
    pub fn new(pos1: Coordinate, pos2: Coordinate) -> Self {
        if pos1.x != pos2.x && pos1.y != pos2.y {
            panic!("Line must be axis-aligned")
        }

        Line { pos1, pos2 }
    }

    pub fn len(&self) -> Scalar {
        if self.is_horizontal() {
            return (self.pos2.x - self.pos1.x).abs();
        }
        if self.is_vertical() {
            return (self.pos2.y - self.pos1.y).abs();
        }
        unreachable!("Should be axis-aligned")
    }

    pub fn hit(&self, other: Line) -> bool {
        if self.is_horizontal() && other.is_vertical() {
            let self_min_x = self.pos1.x.min(self.pos2.x);
            let self_max_x = self.pos1.x.max(self.pos2.x);

            let other_min_y = other.pos1.y.min(other.pos2.y);
            let other_max_y = other.pos1.y.max(other.pos2.y);

            return other.pos1.x >= self_min_x
                && other.pos1.x <= self_max_x
                && self.pos1.y >= other_min_y
                && self.pos1.y <= other_max_y;
        }

        if self.is_vertical() && other.is_horizontal() {
            let self_min_y = self.pos1.y.min(self.pos2.y);
            let self_max_y = self.pos1.y.max(self.pos2.y);

            let other_min_x = other.pos1.x.min(other.pos2.x);
            let other_max_x = other.pos1.x.max(other.pos2.x);

            return self.pos1.x >= other_min_x
                && self.pos1.x <= other_max_x
                && other.pos1.y >= self_min_y
                && other.pos1.y <= self_max_y;
        }

        false
    }
}

impl Line {
    fn is_horizontal(&self) -> bool {
        self.pos1.y == self.pos2.y
    }

    fn is_vertical(&self) -> bool {
        self.pos1.x == self.pos2.x
    }
}

// game modules
pub mod game;


#[derive(Clone, Copy)]
pub enum Cardinal {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Copy, PartialEq)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

impl Coordinate {
    fn step(&mut self, dir: Cardinal, speed: i32) {
        match dir {
            Cardinal::North => self.y += speed,
            Cardinal::South => self.y -= speed,
            Cardinal::East => self.x += speed,
            Cardinal::West => self.x -= speed,
        }
    }
}

// sim modules
pub mod arena;
pub mod wall;
pub mod cycle;

// game modules
pub mod game;
pub mod player;

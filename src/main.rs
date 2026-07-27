
#[derive(Clone, Copy)]
enum Cardinal {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Copy, PartialEq)]
struct Coordinate {
    x: i32,
    y: i32,
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

mod arena;
mod wall;
mod cycle;
mod game;
mod render;

fn main() {
    game::game();
}


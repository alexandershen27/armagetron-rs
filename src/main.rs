

const ARENA_WIDTH: i32 = 10;
const ARENA_HEIGHT: i32 = 10;

#[derive(Debug)]
enum Cardinal {
    North,
    East,
    South,
    West,
}

#[derive(Debug)]
struct Wall {
    start_x: i32,
    start_y: i32,
    end_x: i32,
    end_y: i32,
}

impl Wall {
    fn new(x: i32, y: i32) -> Self {
        Wall {
            start_x: x,
            start_y: y,
            end_x: x,
            end_y: y,
        }
    }
}

#[derive(Debug, PartialEq)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
enum CycleState {
    Alive,
    Dead
}

#[derive(Debug)]
struct Cycle {
    state: CycleState,
    x: i32,
    y: i32,
    facing: Cardinal,
    speed: i32,
    walls: Vec<Wall>,
}

impl Cycle {
    fn spawn(x: i32, y: i32, facing: Cardinal) -> Self {
        Cycle {
            state: CycleState::Alive,
            x,
            y, 
            facing,
            speed: 1,
            walls: vec![Wall::new(x, y)]
        }
    }

    fn tick(&mut self) {
        if self.state == CycleState::Dead {
            return
        }

        self.do_move()
    }

    fn do_move(&mut self) {
        match self.facing {
            Cardinal::North => self.y += self.speed,
            Cardinal::East => self.x += self.speed,
            Cardinal::South => self.y -= self.speed,
            Cardinal::West => self.x -= self.speed,
        }

        if let Some(current_wall) = self.walls.last_mut() {
            current_wall.end_x = self.x;
            current_wall.end_y = self.y;
        }

        if self.x.abs() > ARENA_WIDTH || self.y.abs() > ARENA_HEIGHT {
            self.state = CycleState::Dead
        }
    }

    fn do_turn(&mut self, dir: Direction) {
        self.facing = match (&self.facing, dir) {
            (Cardinal::North, Direction::Left)  => Cardinal::West,
            (Cardinal::North, Direction::Right) => Cardinal::East,
            (Cardinal::East,  Direction::Left)  => Cardinal::North,
            (Cardinal::East,  Direction::Right) => Cardinal::South,
            (Cardinal::South, Direction::Left)  => Cardinal::East,
            (Cardinal::South, Direction::Right) => Cardinal::West,
            (Cardinal::West,  Direction::Left)  => Cardinal::South,
            (Cardinal::West,  Direction::Right) => Cardinal::North,
        };

        self.walls.push(Wall::new(self.x, self.y));
    }
}

fn main() {
    let mut cycle1 = Cycle::spawn(0, 0, Cardinal::North);

    dbg!(&cycle1);
    cycle1.tick();
    cycle1.tick();
    cycle1.tick();
    cycle1.do_turn(Direction::Left);
    cycle1.tick();
    cycle1.tick();
    dbg!(&cycle1);

    return
}

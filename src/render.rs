
use std::cmp;
use crate::arena::Arena;

pub fn vis(arena: &Arena) {
    let xoff = arena.max_x();
    let yoff = arena.max_y() + 1;
    let h = arena.max_y() * 2 + 1;
    let w = arena.max_x() * 2 + 1;

    let mut arena_map = vec![vec!['.'; w as usize]; h as usize];
    
    // Arena borders
    for x in 0..w {
        arena_map[0][x as usize] = '#';
        arena_map[(h-1) as usize][x as usize] = '#';
    }
    for y in 1..h {
        arena_map[y as usize][0] = '#';
        arena_map[y as usize][(w-1) as usize] = '#';
    }

    for cycle in arena.cycles() {
        for wall in cycle.walls() {
            if wall.start_position().y == wall.end_position().y {
                let wy = wall.start_position().y;
                let y = h - wy - yoff;
                let x1 = cmp::min(wall.start_position().x, wall.end_position().x) + xoff;
                let x2 = cmp::max(wall.start_position().x, wall.end_position().x) + xoff;
                for x in x1..x2+1 {
                    arena_map[y as usize][x as usize] = '—';
                }
            }
            if wall.start_position().x == wall.end_position().x {
                let wx = wall.start_position().x;
                let x = wx + xoff;
                let y1 = h - cmp::max(wall.start_position().y, wall.end_position().y) - yoff;
                let y2 = h - cmp::min(wall.start_position().y, wall.end_position().y) - yoff;
                for y in y1..y2+1 {
                    arena_map[y as usize][x as usize] = '|';
                }
            }
        }
    }
    

    for (id, cycle )in arena.cycles().iter().enumerate() {
        let x = cycle.position().x + xoff;
        let y = h - cycle.position().y - yoff;
        if cycle.is_alive() {
            arena_map[y as usize][x as usize] = (id+65) as u8 as char
        } else {
            arena_map[y as usize][x as usize] = 'x'  
        }
    }

    for row in arena_map {
        println!("{}", String::from_iter(row));
    }
}
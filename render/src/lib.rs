
use std::cmp;
use sim::game::Game;

type GridMap = Vec<Vec<String>>;
pub struct TerminalRenderer { }

impl TerminalRenderer {
    pub fn new() -> Self {
        TerminalRenderer { }
    }

    pub fn draw_frame(&self, game: &Game) {
        let state = TerminalRenderer::build_grid_map(game);
        for row in state {
            println!("{}", row.join(""));
        }
    }

    fn build_grid_map(game: &Game) -> GridMap {

        let grid = game.get_grid();

        // Offsets range from [-max, max] to [0, 2*max]
        let xoff = grid.max_x().round() as i32;
        let yoff = grid.max_y().round() as i32 + 1;
        let h = grid.max_y().round() as i32 * 2 + 1;
        let w = grid.max_x().round() as i32 * 2 + 1;

        let mut grid_map = vec![vec!["..".to_string(); w as usize]; h as usize];
        
        // Draw grid borders
        for x in 0..w {
            grid_map[0][x as usize] = "##".to_string();
            grid_map[(h-1) as usize][x as usize] = "##".to_string();
        }
        for y in 1..h {
            grid_map[y as usize][0] = "##".to_string();
            grid_map[y as usize][(w-1) as usize] = "##".to_string();
        }

        // Draw walls
        for cycle in grid.cycles() {
            for wall in cycle.walls() {
                let start_x = wall.start_position().x.round() as i32;
                let start_y = wall.start_position().y.round() as i32;
                let end_x = wall.end_position().x.round() as i32;
                let end_y = wall.end_position().y.round() as i32;

                if start_y == end_y {
                    let wy = start_y;
                    let y = h - wy - yoff;
                    let x1 = cmp::min(start_x, end_x) + xoff;
                    let x2 = cmp::max(start_x, end_x) + xoff;
                    for x in x1..x2+1 {
                        grid_map[y as usize][x as usize] = "——".to_string();
                    }
                }
                if start_x == end_x {
                    let wx = start_x;
                    let x = wx + xoff;
                    let y1 = h - cmp::max(start_y, end_y) - yoff;
                    let y2 = h - cmp::min(start_y, end_y) - yoff;
                    for y in y1..y2+1 {
                        grid_map[y as usize][x as usize] = "||".to_string();
                    }
                }
            }
        }

        // Draw Cycles
        for (id, cycle ) in grid.cycles().iter().enumerate() {
            let cycle_x = cycle.position().x.round() as i32;
            let cycle_y = cycle.position().y.round() as i32;

            let x = cycle_x + xoff;
            let y = h - cycle_y - yoff;
            if cycle.is_alive() {
                let icon = (id+65) as u8 as char;
                grid_map[y as usize][x as usize] = format!("{}{}", icon, icon);
            } else {
                grid_map[y as usize][x as usize] = "XX".to_string()  
            }
        }

        grid_map
    }
}
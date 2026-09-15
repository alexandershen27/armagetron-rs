use crate::app;
use sim::game::{Game, GameConfig, GridConfig};
const CONFIG: GameConfig = GameConfig {
    grid_config: GridConfig {
        max_x: 25.0,
        max_y: 25.0,
        turn_cooldown: 2,
    },
};

pub fn run() {
    let mut sim = Game::new(CONFIG);
    sim.join(0);
    sim.start();
    sim.tick(&[]);
    app::run(sim);
}

mod server;
use crate::server::{Server, ServerConfig};

use sim::game::{GameConfig, GridConfig};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

const CONFIG: ServerConfig = ServerConfig {
    address: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8000),
    tickrate: 20,
    game_config: GameConfig {
        grid_config: GridConfig {
            max_x: 25.0,
            max_y: 25.0,
            turn_cooldown: 2,
        },
    },
};

pub fn main() {
    let mut server = Server::new(CONFIG);
    server.run();
}

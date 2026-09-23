mod app;
mod state;

pub fn main() {
    env_logger::init();
    let _ = app::run();
}

mod app;
mod engine;
mod utils;
mod ia;

fn main() {
    env_logger::init();
    app::launch_browser();
}

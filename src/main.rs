pub mod config;
pub mod app;

use config::Config;

use clap::Parser;

fn main() {
    let config = Config::parse();
    app::run(config);
}

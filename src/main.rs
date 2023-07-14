mod app;
mod config;
mod custom_components;
mod image;
mod stats;
mod util;

fn main() {
    // Terminate if configuration fails to load
    if let Err(error_msg) = config::Config::load() {
        println!("{}", error_msg);
        std::process::exit(-1);
    }

    app::launch_app();
}

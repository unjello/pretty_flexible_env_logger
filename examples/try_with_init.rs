use log::{info,warn,error,debug};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let default = "RUST_LOG".to_string();
    let level = args.get(1).unwrap_or(&default);

    if let Err(e) = pretty_flexible_env_logger::try_init_with(level) {
        eprintln!("Some custom msg {}", e);
        panic!("error!") // or whatever
    }

    info!("info");
    warn!("warn");
    error!("error");
    debug!("debug");
}

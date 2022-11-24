// #[cfg(windows)]
// mod for_windows;
use clap::Parser;

/// Simple program to greet a person
pub use self::traits::{for_mem::MemoryStore, Backend};

mod traits;
mod user_id;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The port which the service needs to listen on
    #[arg(short, long, default_value_t = 7777)]
    port: u16,
    postgres: String,
}

impl Args {
    pub fn run(&self) {}
}

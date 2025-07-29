use clap::Parser;
use simple_logger::SimpleLogger;

mod amplicon;
mod args;
mod search;
mod utils;

use amplicon::amplicon;
use args::Args;

fn main() {
    SimpleLogger::new().init().unwrap();

    let args = Args::parse();

    amplicon(&args);
}

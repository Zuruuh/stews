use args::RawArgs;
use clap::Parser;

mod args;

fn main() {
    let raw_args = RawArgs::parse();
    env_logger::Builder::new()
        .filter_level(raw_args.verbose.log_level_filter())
        .init();

    log::debug!("test");

    println!("Hello, world!");
}

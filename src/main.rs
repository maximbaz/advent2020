use std::env;
use std::process;

mod config;
use config::*;

fn exit_with_error(msg: &str) -> ! {
    println!("{}", msg);
    process::exit(1);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| exit_with_error(err));

    match config.part {
        Part::One => match config.day {
            _ => exit_with_error("This day is not solved yet!"),
        },
        Part::Two => match config.day {
            _ => exit_with_error("This day is not solved yet!"),
        },
    };
}

#![feature(core, exit_status)]

use std::env;
mod ranger;
mod streamer;

fn help() -> i32 {
    println!("Usage: cmded-getline <-|filename> <start line>[..<$|end line>]");
    1
}

macro_rules! validate {
    ($e:expr) => (match $e { Ok(e) => e, Err(_) => return help() })
}

fn get_lines(path: &str, range: &str) -> i32 {
    let stream = validate!(streamer::from(path));
    let (start, end) = validate!(ranger::from(range));
    if 1 > start || start > end {
        return help();
    }

    println!("{} {} {}", path, start, end);
    0
}

fn main() {
    let args: Vec<_> = env::args().collect();
    match args.len() {
        3 => env::set_exit_status(get_lines(&args[1], &args[2])),
        _ => env::set_exit_status(help())
    }
}


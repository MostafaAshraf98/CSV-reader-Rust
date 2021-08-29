use crate::args::Args; // importing the struct Args from args.rs
use crate::reader::run; // importing the method run from reader.rs
use std::env;
use std::time::Instant;

mod args;
mod reader;

fn main() {
    let start = Instant::now();
    let mut args: Args = Args::new();
    args.filename = env::args().nth(1).expect("Missing file path");
    args.group_by = env::args().nth(2);
    args.query = env::args().nth(3);
    args.select = env::args().nth(4);
    run(&args);
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
}

use std::{env, fs::File};

use cache::Cache;
use parser::LineIterator;
use types::Sizes;

mod args;
mod cache;
mod parser;
mod types;

fn main() {
    let args: Vec<String> = env::args().collect();
    let in_file = File::open(&args[1]).unwrap();
    // let out_file = File::open(...)
    let c = args[2].parse().unwrap();
    let e = args[3].parse().unwrap();
    let s = args[4].parse().unwrap();

    let mut cache = Cache::new(c, e, s);

    let mut line_iter = LineIterator::new(in_file, Sizes::new(c, s, e));

    while let Some(line) = line_iter.next() {
        println!("{:?}", line);
        cache.insert(line);
    }
    cache.print_summary();
}

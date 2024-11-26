#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use std::{
    env,
    fs::{File, OpenOptions},
    io,
};

use cache::Cache;
use parser::LineIterator;
use types::Sizes;

mod args;
mod cache;
mod parser;
mod types;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let in_file = File::open(&args[1])?;
    let out_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("out_file.txt")?;

    // trazas/blowfish.xex 2048 4 8 | file c e s
    let c = args[2].parse().unwrap();
    let e = args[3].parse().unwrap();
    let s = args[4].parse().unwrap();

    let mut cache = Cache::new(Sizes::new(c, s, e), out_file);

    let line_iter = LineIterator::new(in_file, Sizes::new(c, s, e));
    for line in line_iter {
        cache.insert(&line)?;
    }

    println!("{:#?}", cache);
    cache.print_summary();
    Ok(())
}

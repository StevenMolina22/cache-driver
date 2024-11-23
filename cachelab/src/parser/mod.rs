use crate::types::Operation;
use std::fs::File;
use std::io::{self, BufRead};

pub struct Transaction {
    i_op: usize,
    op: Operation,
    tag: usize,
    set: usize,
}

pub fn get_transaction(file: &File) -> Option<Transaction> {
    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
    }
    todo!()
}

pub fn parse_line(line: &str) -> Transaction {
    let cols: Vec<&str> = line.split_whitespace().collect();

    Transaction {
        i_op: cols[0].parse().unwrap(),
        op: Operation::from_char(cols[1].chars().next().unwrap()),
        set: cols[2].parse().unwrap(),
        tag: cols[3].parse().unwrap(),
    }
}

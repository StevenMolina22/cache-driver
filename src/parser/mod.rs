#![allow(dead_code)]
use crate::types::{Operation, Sizes};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
pub struct Transaction {
    pub i_op: usize,
    pub op: Operation,
    pub tag: usize,
    pub set: usize,
}

pub struct LineIterator {
    reader: BufReader<File>,
    idx: usize,
    sizes: Sizes,
}

impl LineIterator {
    pub fn new(file: File, sizes: Sizes) -> Self {
        LineIterator {
            reader: BufReader::new(file),
            idx: 0,
            sizes,
        }
    }
}

impl Iterator for LineIterator {
    type Item = Transaction;

    fn next(&mut self) -> Option<Self::Item> {
        let mut line = String::new();
        match self.reader.read_line(&mut line) {
            Ok(0) => None,
            Ok(_) => {
                let val = parse_line(&line, self.sizes, self.idx);
                self.idx += 1;
                return Some(val);
            }
            Err(_) => None,
        }
    }
}

/// Line ex:
/// ```sh
///     "0xb7fc7489: W 0xbff20468 4 0xb7fc748e"
/// ```
/// With indexes:
///     - [0] => Instruction Address
///     - [1] => Operation Type
///     - [2] => Memory Address
///     - [3] => Bytes read / written
///     - [4] => Data read / written
///
pub fn parse_line(line: &str, sizes: Sizes, idx: usize) -> Transaction {
    let cols: Vec<&str> = line.split_whitespace().collect();

    // arguments that are parsed: [0] and [2]
    let mem_addr = usize::from_str_radix(&cols[2][2..], 16).unwrap();

    // lengths used for getting the tag and the set from mem address
    let block_len = (sizes.size / sizes.sets / sizes.asociativity).ilog2();
    let set_len = (sizes.sets).ilog2();

    Transaction {
        i_op: idx,
        op: Operation::from_char(cols[1].chars().next().unwrap()),
        set: (mem_addr >> block_len) & ((1 << set_len) - 1),
        tag: (mem_addr) >> (block_len + set_len),
    }
}

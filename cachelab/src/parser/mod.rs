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
    reader: io::BufReader<File>,
    sizes: Sizes,
}

impl LineIterator {
    pub fn new(file: File, sizes: Sizes) -> Self {
        LineIterator {
            reader: BufReader::new(file),
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
            Ok(_) => Some(parse_line(&line, self.sizes)),
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
pub fn parse_line(line: &str, sizes: Sizes) -> Transaction {
    let cols: Vec<&str> = line.split_whitespace().collect();

    // arguments that are parsed: [0] and [2]
    let inst_addr = &cols[0][2..cols[0].len() - 1]; // from: "0x80540ed:"
    let mem_addr = usize::from_str_radix(&cols[2][2..], 16).unwrap();

    // lengths used for getting the tag and the set from mem address
    let block_len = (sizes.size / sizes.sets / sizes.asociativity).ilog2();
    let set_len = (sizes.sets).ilog2();

    Transaction {
        i_op: usize::from_str_radix(inst_addr, 16).unwrap(),
        op: Operation::from_char(cols[1].chars().next().unwrap()),
        set: (mem_addr >> block_len) & ((1 << set_len) - 1),
        tag: (mem_addr) >> (block_len + set_len),
    }
}

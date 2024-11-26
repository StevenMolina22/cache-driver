#![allow(dead_code)]
#[derive(Default, Clone, Debug)]
pub enum Operation {
    #[default]
    Read,
    Write,
}

#[derive(Default, Clone, Debug)]
pub enum Case {
    #[default]
    Hit,
    CleanMiss,
    DirtyMiss,
}

/// Metrics sizes for the cache
#[derive(Debug, Clone, Copy)]
pub struct Sizes {
    pub size: usize,
    pub sets: usize,
    pub asociativity: usize,
}

impl Sizes {
    pub fn new(size: usize, sets: usize, asociativity: usize) -> Self {
        Sizes {
            size,
            sets,
            asociativity,
        }
    }
}

impl Operation {
    pub fn from_char(c: char) -> Self {
        match c {
            'r' | 'R' => Operation::Read,
            'w' | 'W' => Operation::Write,
            _ => panic!("not valid item"),
        }
    }
}

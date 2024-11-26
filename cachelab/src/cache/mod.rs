#![allow(dead_code)]
use std::{fs::File, usize};
pub mod funcs;
pub mod io;

use crate::types::{Case, Sizes};

#[derive(Debug)]
pub struct Cache {
    size: usize,
    asociativity: usize,
    sets: Vec<Set>,
    metrics: Metrics,
    out_file: File,
}

#[derive(Default, Debug, Clone)]
pub struct Set {
    lines: Vec<Option<Line>>,
}

#[derive(Default, Clone, Debug)]
pub struct Line {
    i_op: usize,
    case: Case,
    i_set: usize,
    tag: usize,
    line_tag: usize,
    is_valid: bool,
    is_dirty: bool,
    last_used_by: usize,
}

#[derive(Default, Clone, Debug)]
struct Metrics {
    loads: usize,
    stores: usize,
    rmisses: usize,
    dirty_rmisses: usize,
    dirty_wmisses: usize,
    bytes_read: usize,
    bytes_written: usize,
    rcycles_time: usize,
    wcycles_time: usize,
}

impl Cache {
    pub fn new(sizes: Sizes, out_file: File) -> Self {
        Cache {
            size: sizes.size,
            asociativity: sizes.asociativity,
            sets: (0..sizes.sets)
                .map(|_| Set {
                    lines: vec![None; sizes.asociativity],
                })
                .collect(),
            out_file,
            metrics: Metrics::default(),
        }
    }
}

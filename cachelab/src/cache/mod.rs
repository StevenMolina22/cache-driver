use crate::{
    parser::Transaction,
    types::{Case, Operation},
};

pub struct Cache {
    size: usize,
    asociativity: usize,
    sets: Vec<Set>,
    metrics: Metrics,
}

#[derive(Default, Clone)]
pub struct Set {
    lines: Vec<Line>,
}

#[derive(Default, Clone)]
pub struct Line {
    i_op: usize,
    case: Case,
    tag: usize,
    line_tag: usize,
    is_valid: bool,
    is_dirty: bool,
    last_used_by: usize,
}

#[derive(Default, Clone)]
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
    pub fn new(size: usize, asociativity: usize, sets: usize) -> Self {
        Cache {
            size,
            asociativity,
            sets: vec![Set::default(); sets],
            metrics: Metrics::default(),
        }
    }
    pub fn insert(&mut self, transaction: Transaction) {}
    pub fn print_summary(&self) {}
    pub fn print_verbose(&self) {}
}

impl Line {
    pub fn init_from(&mut self, transaction: &Transaction) {
        self.case = Case::CleanMiss;
        self.is_valid = true;
        self.is_dirty = match transaction.op {
            Operation::Read => false,
            Operation::Write => true,
        };
        self.tag = transaction.tag;
        self.line_tag = 0; // TODO!: Should be -1 :(
        self.last_used_by = self.i_op;
    }
}

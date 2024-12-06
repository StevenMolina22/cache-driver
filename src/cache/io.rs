use super::Line;
use crate::{
    parser::Transaction,
    types::{Case, Operation},
};

impl Line {
    pub fn from(tx: &Transaction) -> Self {
        Line {
            i_op: tx.i_op,
            case: Case::CleanMiss,
            i_set: tx.set,
            tag: tx.tag,
            i_line: 0,
            line_tag: 0,
            is_valid: true,
            is_dirty: match tx.op {
                Operation::Read => false,
                Operation::Write => true,
            },
            last_used_by: tx.i_op,
        }
    }
    pub fn display(&self, is_valid: bool, is_dirty: bool, last_used_by: usize) -> String {
        format!(
            "{} {} {:x} {:x} {} {:x} {} {} {}\n",
            self.i_op,
            match self.case {
                Case::CleanMiss => "2a",
                Case::DirtyMiss => "2b",
                Case::Hit => "1",
            },
            self.i_set,
            self.tag,
            self.i_line, // TODO!: should cache line
            self.line_tag,
            is_valid as i32,
            is_dirty as i32,
            last_used_by,
        )
    }
}

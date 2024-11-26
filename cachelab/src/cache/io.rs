use super::Line;
use crate::{
    parser::Transaction,
    types::{Case, Operation},
};

impl Line {
    pub fn from(transaction: &Transaction) -> Self {
        println!("{}", transaction.i_op);
        Line {
            i_op: transaction.i_op,
            case: Case::CleanMiss,
            i_set: transaction.set,
            is_valid: true,
            is_dirty: match transaction.op {
                Operation::Read => false,
                Operation::Write => true,
            },
            tag: transaction.tag,
            line_tag: 0,
            last_used_by: transaction.i_op,
        }
    }
    pub fn display(&self) -> String {
        format!(
            "{} {} {:x} {:x} {} {} {} {} {}\n",
            self.i_op,
            match self.case {
                Case::CleanMiss => "2a",
                Case::DirtyMiss => "2b",
                Case::Hit => "1",
            },
            self.i_set,
            self.tag,
            0, // TODO!: should cache line
            self.line_tag,
            self.is_valid as i32,
            self.is_dirty as i32,
            self.last_used_by,
        )
    }
}

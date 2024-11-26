#![allow(dead_code)]
use std::io::{self, Write};

use crate::{
    parser::Transaction,
    types::{Case, Operation},
};

use super::{Cache, Line};

impl Cache {
    fn handle_hit(&mut self, transaction: &Transaction) -> bool {
        for set in self.sets.iter_mut() {
            for line in set.lines.iter_mut() {
                if let Some(line_i) = line {
                    if line_i.tag != transaction.tag {
                        continue;
                    }
                    let debug_str = line_i.display();
                    self.out_file.write_all(debug_str.as_bytes()).unwrap();
                    line_i.case = Case::Hit;
                    line_i.line_tag = transaction.tag;
                    if let Operation::Write = transaction.op {
                        line_i.is_dirty = true;
                    }
                    line_i.last_used_by = transaction.i_op;
                    return true;
                }
            }
        }
        false
    }

    fn find_lru(&self, i_set: usize) -> usize {
        let mut i_lru = None;
        let mut min_last_used_by = usize::MAX;

        for (idx, line) in self.sets[i_set].lines.iter().enumerate() {
            match line {
                Some(line_i) => {
                    if line_i.last_used_by < min_last_used_by {
                        min_last_used_by = line_i.last_used_by;
                        i_lru = Some(idx);
                    }
                }
                None => {
                    return idx;
                }
            }
        }
        i_lru.unwrap_or(0)
    }

    fn replace_line(&self, transaction: &Transaction, to_replace: &mut Line) {
        if to_replace.is_valid && to_replace.is_dirty {
            to_replace.case = Case::DirtyMiss;
        } else {
            to_replace.case = Case::CleanMiss;
        }
        to_replace.last_used_by = transaction.i_op;
        to_replace.is_valid = true;
        to_replace.is_dirty = match transaction.op {
            Operation::Read => false,
            Operation::Write => true,
        };
        to_replace.line_tag = to_replace.tag;
    }

    pub fn insert(&mut self, transaction: &Transaction) -> io::Result<()> {
        if self.handle_hit(transaction) {
            return Ok(());
        }

        let i_set = transaction.set;
        let i_lru = self.find_lru(i_set);

        let line = std::mem::replace(&mut self.sets[i_set].lines[i_lru], None); // Temporarily take ownership
        match line {
            Some(mut line_i) => {
                let debug_str = line_i.display();
                self.out_file.write_all(debug_str.as_bytes())?;

                self.replace_line(transaction, &mut line_i);
                self.sets[i_set].lines[i_lru] = Some(line_i); // Put the modified line back
            }
            None => {
                self.sets[i_set].lines[i_lru] = Some(Line::from(transaction));
            }
        }
        Ok(())
    }

    pub fn print_summary(&self) {}
    pub fn print_verbose(&self) {}
}

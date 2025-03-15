#![allow(dead_code)]
use std::io::{self, Write};

use crate::{
    parser::Transaction,
    types::{Case, Operation},
};

use super::{Cache, Line};

impl Cache {
    fn handle_hit(&mut self, tx: &Transaction) -> bool {
        if let Some(set) = self.sets.get_mut(tx.set) {
            for (i, maybe_line) in set.lines.iter_mut().enumerate() {
                if let Some(line) = maybe_line {
                    if line.tag != tx.tag {
                        continue;
                    }
                    line.i_op = tx.i_op;
                    line.i_line = i;
                    line.case = Case::Hit;
                    line.line_tag = line.tag;
                    let debug_str = line.display(line.is_valid, line.is_dirty, line.last_used_by);
                    self.out_file.write_all(debug_str.as_bytes()).unwrap();
                    if let Operation::Write = tx.op {
                        line.is_dirty = true;
                    }
                    line.last_used_by = tx.i_op;
                    return true;
                }
            }
        }
        false
    }

    fn find_lru(&self, i_set: usize) -> usize {
        let mut i_lru = None;
        let mut min_last_used_by = usize::MAX;

        for (i, maybe_line) in self.sets[i_set].lines.iter().enumerate() {
            match maybe_line {
                Some(line) => {
                    if line.last_used_by < min_last_used_by {
                        min_last_used_by = line.last_used_by;
                        i_lru = Some(i);
                    }
                }
                None => return i,
            }
        }
        i_lru.unwrap_or(0)
    }

    fn replace_line(&self, tx: &Transaction, line: &mut Line) {
        if line.is_dirty {
            line.case = Case::DirtyMiss;
        } else {
            line.case = Case::CleanMiss;
        }
        line.line_tag = line.tag;
        line.tag = tx.tag;
        line.is_valid = true;
        line.is_dirty = match tx.op {
            Operation::Read => false,
            Operation::Write => true,
        };
        line.i_set = tx.set;
        line.last_used_by = tx.i_op;
    }

    pub fn insert(&mut self, tx: &Transaction) -> io::Result<()> {
        if self.handle_hit(tx) {
            println!("There was a hit at: {}", tx.i_op);
            return Ok(());
        }

        let i_set = tx.set;
        let i_lru = self.find_lru(i_set);

        // Temporarily take ownership
        let line = std::mem::replace(&mut self.sets[i_set].lines[i_lru], None);
        // save stats before making the line replacement

        let is_valid = line.map_or(false, |line| line.is_valid);
        let is_dirty = line.map_or(false, |line| line.is_dirty);
        let last_used_by = line.map_or(0, |line| line.last_used_by);

        match line {
            Some(mut line) => {
                self.replace_line(tx, &mut line);
                self.sets[i_set].lines[i_lru] = Some(line);
            }
            None => {
                self.sets[i_set].lines[i_lru] = Some(Line::from(tx));
            }
        }
        let mut line = self.sets[i_set].lines[i_lru].unwrap();
        line.i_line = i_lru;
        let debug_str = line.display(is_valid, is_dirty, last_used_by);
        self.out_file.write_all(debug_str.as_bytes())?;
        Ok(())
    }

    pub fn print_summary(&self) {}
    pub fn print_verbose(&self) {}
}

#[derive(Default, Clone)]
pub enum Operation {
    #[default]
    Read,
    Write,
}

#[derive(Default, Clone)]
pub enum Case {
    #[default]
    Hit,
    CleanMiss,
    DirtyMiss,
}

impl Operation {
    pub fn from_char(c: char) -> Self {
        match c {
            'r' => Operation::Read,
            'w' => Operation::Write,
            _ => Operation::Read,
        }
    }
}
